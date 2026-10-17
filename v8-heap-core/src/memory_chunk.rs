//! 内存块 (MemoryChunk) 与页布局/常量。
//! 对应原版 v8/src/heap/memory-chunk.{h,cc,-inl.h} + memory-chunk-layout.h + memory-chunk-constants.h (HC-1R 落地)。
//!
//! 目标配置 (与 tool/src/heap/*.js clang -E 展开侧同源):
//!   V8_COMPRESS_POINTERS + V8_ENABLE_SANDBOX + V8_EXTERNAL_CODE_SPACE, DEBUG/TSAN/STICKY 关。
//! 布局金标准: repr(C) { flags usize@0, metadata_index u32@8 } → size_of = 16
//!   (原版 FlagsOffset h:301 / MetadataIndexOffset h:314 即 offset 导出面)。

use core::{
  mem::{offset_of, size_of},
  ptr::from_ref,
};
use std::sync::atomic::{Ordering, fence};

use v8_base::{bits::bit_width64, macros::round_up_address};
pub use v8_common::K_REGULAR_PAGE_SIZE;
use v8_common::{
  Address, AllocationSpace, K_CLEARED_WEAK_HEAP_OBJECT_LOWER32, K_CODE_ALIGNMENT, K_DOUBLE_SIZE,
  K_MAX_REGULAR_HEAP_OBJECT_SIZE, K_PAGE_SIZE_BITS, K_PTR_COMPR_CAGE_BASE_ALIGNMENT,
  K_PTR_COMPR_CAGE_RESERVATION_SIZE, K_TAGGED_SIZE, TaggedT, TrustedSpaceCompressionScheme,
  V8HeapCompressionScheme, align_to_allocation_alignment, is_any_code_space,
};
use v8_objects_core::tagged::Tagged;

/// 原版 heap/base-page.h BasePage 前向声明 (memory-chunk.h:34) 的 opaque 承接 (他票)。
/// C++ 侧只透传指针 → Rust 以空 enum 承接 `*const/*mut BasePage` 形参与回指槽。
pub enum BasePage {}

// ══════════════════════════════════════════════════════════════════════════
// MemoryChunkConstants (原版 memory-chunk-constants.h:15-38, #ifdef V8_ENABLE_SANDBOX 全量)
// ══════════════════════════════════════════════════════════════════════════

/// 主 cage 页数 (constants.h:18-19) = 2^32 / 2^18 = 16384
pub const K_PAGES_IN_MAIN_CAGE: usize = K_PTR_COMPR_CAGE_RESERVATION_SIZE / K_REGULAR_PAGE_SIZE;
/// 码 cage 页数 (constants.h:20-21) — K_MAXIMAL_CODE_RANGE_SIZE 随架构 (x64 512MB / arm64 256MB)
pub const K_PAGES_IN_CODE_CAGE: usize = v8_common::K_MAXIMAL_CODE_RANGE_SIZE / K_REGULAR_PAGE_SIZE;
/// trusted cage 页数 (constants.h:22-23) = 2^30 / 2^18 = 4096
pub const K_PAGES_IN_TRUSTED_CAGE: usize =
  v8_common::K_MAXIMAL_TRUSTED_RANGE_SIZE / K_REGULAR_PAGE_SIZE;
/// 主 cage 元数据表偏移 (constants.h:25)
pub const K_MAIN_CAGE_METADATA_OFFSET: usize = 0;
/// trusted space 元数据表偏移 (constants.h:26-27)
pub const K_TRUSTED_SPACE_METADATA_OFFSET: usize =
  K_MAIN_CAGE_METADATA_OFFSET + K_PAGES_IN_MAIN_CAGE;
/// code range 元数据表偏移 (constants.h:28-29)
pub const K_CODE_RANGE_METADATA_OFFSET: usize =
  K_TRUSTED_SPACE_METADATA_OFFSET + K_PAGES_IN_TRUSTED_CAGE;
/// 表容量位宽 (constants.h:31-32) — base::bits::BitWidth(主+码+信)
pub const K_METADATA_POINTER_TABLE_SIZE_LOG2: usize =
  bit_width64((K_PAGES_IN_MAIN_CAGE + K_PAGES_IN_CODE_CAGE + K_PAGES_IN_TRUSTED_CAGE) as u64)
    as usize;
/// 表容量 (constants.h:33-34)
pub const K_METADATA_POINTER_TABLE_SIZE: usize = 1 << K_METADATA_POINTER_TABLE_SIZE_LOG2;
/// 表容量掩码 (constants.h:35-36)
pub const K_METADATA_POINTER_TABLE_SIZE_MASK: usize = K_METADATA_POINTER_TABLE_SIZE - 1;

// ══════════════════════════════════════════════════════════════════════════
// MemoryChunk::Flag 14 逐值 (原版 memory-chunk.h:56-109) + MainThreadFlags (h:111)
// 目标配置展开侧 11 移位值 + NO_FLAGS; STICKY_* ×2 (h:103/h:107) 为非目标配置,
// 枚举面值照录保台账完整 (h 脚本①逐值锚), 消费面 ContainsOnlyOldObjects/IsOnlyOldOrMajorMarkingOn
// 登记推迟 (见文末推迟台账)。
// ══════════════════════════════════════════════════════════════════════════

/// 原版 `using MainThreadFlags = base::Flags<Flag, uintptr_t>` (h:111);
/// Rust 以 usize 位型承接 (uintptr_t), DEFINE_OPERATORS_FOR_FLAGS (h:378) 的 |/& 即 Rust 内建位运算。
pub type MainThreadFlags = usize;

/// 无标志 (h:57)
pub const NO_FLAGS: MainThreadFlags = 0;
/// 块属可写共享空间 (h:60)
pub const IN_WRITABLE_SHARED_SPACE: MainThreadFlags = 1 << 0;
/// 指向此块的指针需写屏障关注 (h:64)
pub const POINTERS_TO_HERE_ARE_INTERESTING: MainThreadFlags = 1 << 1;
/// 从此块发出的指针需写屏障关注 (h:65)
pub const POINTERS_FROM_HERE_ARE_INTERESTING: MainThreadFlags = 1 << 2;
/// from-space 页或未 scavenge 的年轻大页 (h:69)
pub const FROM_PAGE: MainThreadFlags = 1 << 3;
/// to-space 页或已 scavenge 的年轻大页 (h:71)
pub const TO_PAGE: MainThreadFlags = 1 << 4;
/// 增量标记进行中 (h:74)
pub const INCREMENTAL_MARKING: MainThreadFlags = 1 << 5;
/// 主增量标记期间分配、仅含旧对象 (h:78)
pub const BLACK_ALLOCATED: MainThreadFlags = 1 << 6;
/// 大块非均匀页 (h:81)
pub const LARGE_PAGE: MainThreadFlags = 1 << 7;
/// 摘出候选 (对象正在搬移) (h:85)
pub const EVACUATION_CANDIDATE: MainThreadFlags = 1 << 8;
/// 年轻代 new space 且已活过至少一轮 GC (h:89)
pub const NEW_SPACE_BELOW_AGE_MARK: MainThreadFlags = 1 << 9;
/// 只读堆块、不参与 GC (h:97; 非连续 RO 配置下存在, 目标配置保留)
pub const READ_ONLY_HEAP: MainThreadFlags = 1 << 10;
/// sticky markbits 专属: 空间不含年轻对象 (h:103; 目标配置枚举剥除, 面值照录)
pub const STICKY_MARK_BIT_CONTAINS_ONLY_OLD: MainThreadFlags = 1 << 11;
/// sticky markbits 专属: 主 GC 进行中 (h:107; 目标配置枚举剥除, 面值照录)
pub const STICKY_MARK_BIT_IS_MAJOR_GC_IN_PROGRESS: MainThreadFlags = 1 << 12;

// ── 掩码常量 (原版 h:113-143; 目标配置 12 项, kIsOnlyOldOrMajorGCInProgressMask h:140-142 推迟) ──

/// 全标志掩码 ~MainThreadFlags(NO_FLAGS) (h:113)
pub const K_ALL_FLAGS_MASK: MainThreadFlags = !NO_FLAGS;
/// (h:114-115)
pub const K_POINTERS_TO_HERE_ARE_INTERESTING_MASK: MainThreadFlags =
  POINTERS_TO_HERE_ARE_INTERESTING;
/// (h:116-117)
pub const K_POINTERS_FROM_HERE_ARE_INTERESTING_MASK: MainThreadFlags =
  POINTERS_FROM_HERE_ARE_INTERESTING;
/// (h:118-119)
pub const K_EVACUATION_CANDIDATE_MASK: MainThreadFlags = EVACUATION_CANDIDATE;
/// FROM_PAGE | TO_PAGE (h:120-121)
pub const K_IS_IN_YOUNG_GENERATION_MASK: MainThreadFlags = FROM_PAGE | TO_PAGE;
/// (h:122)
pub const K_IS_LARGE_PAGE_MASK: MainThreadFlags = LARGE_PAGE;
/// (h:123)
pub const K_IN_SHARED_HEAP: MainThreadFlags = IN_WRITABLE_SHARED_SPACE;
/// (h:124)
pub const K_INCREMENTAL_MARKING: MainThreadFlags = INCREMENTAL_MARKING;
/// 摘出槽位记录跳过掩码 (h:125-127)
pub const K_SKIP_EVACUATION_SLOTS_RECORDING_MASK: MainThreadFlags =
  K_EVACUATION_CANDIDATE_MASK | K_IS_IN_YOUNG_GENERATION_MASK;
/// 黑分配或可写共享掩码 (h:128-130)
pub const K_IS_BLACK_ALLOCATED_OR_WRITEABLE_SHARED_MASK: MainThreadFlags =
  BLACK_ALLOCATED | IN_WRITABLE_SHARED_SPACE;
/// 只读堆掩码 (h:133; 非连续 RO 配置面, 目标配置落地)
pub const K_IS_IN_READ_ONLY_HEAP_MASK: MainThreadFlags = READ_ONLY_HEAP;
/// 只读或共享堆掩码 (h:134-136)
pub const K_IS_READ_ONLY_OR_SHARED_HEAP_MASK: MainThreadFlags =
  READ_ONLY_HEAP | IN_WRITABLE_SHARED_SPACE;

// ══════════════════════════════════════════════════════════════════════════
// MemoryChunk (原版 memory-chunk.h:47-376)
// ══════════════════════════════════════════════════════════════════════════

/// 任意大小的一块堆内存的头部 (原版 h:42-376)。
/// repr(C): flags(usize) + metadata_index(u32) → size_of=16, offset 0/8
/// (原版沙箱形态 h:357 + h:360; 非沙箱 metadata_ 直回指形态 h:362 见推迟台账)。
#[repr(C)]
#[derive(Debug)]
pub struct MemoryChunk {
  /// 主线程标志位 (仅无并发时可改; 沙箱语义下不可信) (原版 h:351-357)
  untrusted_main_thread_flags_: MainThreadFlags,
  /// 全局 metadata pointer table 索引 (原版 h:359-360, #ifdef V8_ENABLE_SANDBOX)
  metadata_index_: u32,
}

// 原版目标配置守护断言承接: sandbox 必配 external code space (h:310-312 #error) 与
// 只读空间非连续形态 (h:91/h:132 #if !CONTIGUOUS..._BOOL 保留分支)
const _: () = assert!(v8_common::V8_ENABLE_SANDBOX && v8_common::V8_EXTERNAL_CODE_SPACE);
const _: () = assert!(!v8_common::CONTIGUOUS_COMPRESSED_READ_ONLY_SPACE);
// 原版 memory-chunk.cc:25-26 — 清弱引用低 32 位与对象地址不重叠/小于 chunk 头
const _: () = assert!(K_CLEARED_WEAK_HEAP_OBJECT_LOWER32 as usize > 0);
const _: () = assert!((K_CLEARED_WEAK_HEAP_OBJECT_LOWER32 as usize) < size_of::<MemoryChunk>());
// 原版 memory-chunk.cc:86 — static_assert(kPtrComprCageReservationSize == kPtrComprCageBaseAlignment)
const _: () = assert!(K_PTR_COMPR_CAGE_RESERVATION_SIZE == K_PTR_COMPR_CAGE_BASE_ALIGNMENT);

impl MemoryChunk {
  // ── 静态 constexpr 面 ──

  /// 页对齐 (块基址掩码) = 1 << kPageSizeBits (原版 h:305-306);
  /// 原版 private 由 friend 消费 → 本 crate 导出供上层/测试 (同 linear_allocation_area 先例)
  pub const fn alignment() -> usize {
    1 << K_PAGE_SIZE_BITS
  }

  /// kAlignment - 1 (原版 h:307)
  pub const fn alignment_mask() -> usize {
    Self::alignment() - 1
  }

  /// GetAlignmentForAllocation (原版 h:161)
  #[inline]
  pub const fn get_alignment_for_allocation() -> isize {
    Self::alignment() as isize
  }

  /// GetAlignmentMaskForAssembler (原版 h:166-168, 汇编器侧消费)
  #[inline]
  pub const fn get_alignment_mask_for_assembler() -> isize {
    Self::alignment_mask() as isize
  }

  /// BaseAddress: 地址向下取整到块边界 (原版 h:145-155; LINT.IfChange 关联汇编器侧)
  #[inline]
  pub const fn base_address(a: Address) -> Address {
    a & !Self::alignment_mask()
  }

  /// IsAligned (原版 h:157-159)
  #[inline]
  pub const fn is_aligned(address: Address) -> bool {
    (address & Self::alignment_mask()) == 0
  }

  /// AddressToOffset: 低 32 位截断后取页内偏移 (原版 h:170-172, static_cast<uint32_t> 截断照录)
  #[inline]
  pub const fn address_to_offset(address: Address) -> u32 {
    (address as TaggedT) & (Self::alignment_mask() as TaggedT)
  }

  /// FromAddress: 任意地址归位到块头指针 (原版 h:174-176)。
  /// 返回裸指针, 有效性由调用方 (页表) 保证 —— 与原版 reinterpret_cast 语义一致。
  #[inline]
  pub const fn from_address(addr: Address) -> *mut MemoryChunk {
    Self::base_address(addr) as *mut MemoryChunk
  }

  /// FromHeapObject: 由堆对象 tagged 指针反查块头 (原版 h:178-181, object.ptr() 位型照录)
  #[inline]
  pub const fn from_heap_object<T>(object: Tagged<T>) -> *mut MemoryChunk {
    Self::from_address(object.ptr())
  }

  /// FlagsOffset (原版 h:301-303, offsetof(untrusted_main_thread_flags_))
  pub const fn flags_offset() -> isize {
    offset_of!(MemoryChunk, untrusted_main_thread_flags_) as isize
  }

  /// MetadataIndexOffset (原版 h:314-316, offsetof(metadata_index_); 沙箱形态)
  pub const fn metadata_index_offset() -> isize {
    offset_of!(MemoryChunk, metadata_index_) as isize
  }

  // ── 实例面 ──

  /// ctor 数据面承接 (原版 cc:46-63): flags + 已算好的 metadata_index。
  /// 原版沙箱分支的 metadata_pointer_table 写入 (cc:53-61, IsolateGroup 耦合) 登记推迟,
  /// 表写入侧由上层经 MetadataTableEnv 消费后调用本构造。
  pub fn with_metadata_index(flags: MainThreadFlags, metadata_index: u32) -> Self {
    Self {
      untrusted_main_thread_flags_: flags,
      metadata_index_: metadata_index,
    }
  }

  /// 沙箱关闭形态的直回指构造 (原版 cc:48-51 非沙箱分支 metadata_(metadata))。
  /// 目标配置 (sandbox 开) 不走此形态; 提供非沙箱布局下的 1:1 承接位 (字段复用 index 槽宽)。
  pub fn with_metadata_direct(flags: MainThreadFlags, metadata: *mut BasePage) -> Self {
    // 非沙箱形态 metadata_ 为指针字段 (h:362); 目标配置 const 门: 此构造等价保留槽位
    let _ = metadata;
    Self {
      untrusted_main_thread_flags_: flags,
      metadata_index_: u32::MAX,
    }
  }

  /// address(): 块自身地址 (原版 h:185, reinterpret_cast<Address>(this))
  #[inline]
  pub fn address(&self) -> Address {
    from_ref(self) as Address
  }

  /// GetFlags (原版 h:343-345)
  #[inline]
  pub fn get_flags(&self) -> MainThreadFlags {
    self.untrusted_main_thread_flags_
  }

  /// IsFlagSet (原版 h:347-349)
  #[inline]
  pub fn is_flag_set(&self, flag: MainThreadFlags) -> bool {
    self.untrusted_main_thread_flags_ & flag != 0
  }

  /// IsMarking (原版 h:196)
  #[inline]
  pub fn is_marking(&self) -> bool {
    self.is_flag_set(INCREMENTAL_MARKING)
  }

  /// InWritableSharedSpace (原版 h:198-200)
  #[inline]
  pub fn in_writable_shared_space(&self) -> bool {
    self.is_flag_set(IN_WRITABLE_SHARED_SPACE)
  }

  /// IsBlackAllocated (原版 h:202)
  #[inline]
  pub fn is_black_allocated(&self) -> bool {
    self.is_flag_set(BLACK_ALLOCATED)
  }

  /// IsBlackAllocatedOrWritableShared (原版 h:204-206)
  #[inline]
  pub fn is_black_allocated_or_writable_shared(&self) -> bool {
    self.get_flags() & K_IS_BLACK_ALLOCATED_OR_WRITEABLE_SHARED_MASK != 0
  }

  /// InYoungGeneration (原版 h:217-221; UNREACHABLE_WITH_STICKY_MARK_BITS h:218 目标配置为空宏 h:12-16)
  #[inline]
  pub fn in_young_generation(&self) -> bool {
    // 原版局部 constexpr kYoungGenerationMask = FROM_PAGE | TO_PAGE (h:219)
    self.get_flags() & (FROM_PAGE | TO_PAGE) != 0
  }

  /// IsYoungOrSharedChunk (原版 h:224-228)
  #[inline]
  pub fn is_young_or_shared_chunk(&self) -> bool {
    self.get_flags() & (FROM_PAGE | TO_PAGE | IN_WRITABLE_SHARED_SPACE) != 0
  }

  /// InitializationMemoryFence (原版 cc:134-136; TSAN 注释区 cc:137-160 剥除)
  /// base::SeqCst_MemoryFence → Rust 全序栅栏
  #[inline]
  pub fn initialization_memory_fence(&mut self) {
    fence(Ordering::SeqCst);
  }

  /// ShouldSkipEvacuationSlotRecording (原版 h:238-240)
  #[inline]
  pub fn should_skip_evacuation_slot_recording(&self) -> bool {
    (self.get_flags() & K_SKIP_EVACUATION_SLOTS_RECORDING_MASK) != 0
  }

  /// IsFromPage (原版 h:242-245; sticky 空宏面同 h:218 注)
  #[inline]
  pub fn is_from_page(&self) -> bool {
    self.is_flag_set(FROM_PAGE)
  }

  /// IsToPage (原版 h:246-249)
  #[inline]
  pub fn is_to_page(&self) -> bool {
    self.is_flag_set(TO_PAGE)
  }

  /// IsLargePage (原版 h:250)
  #[inline]
  pub fn is_large_page(&self) -> bool {
    self.is_flag_set(LARGE_PAGE)
  }

  /// InNewSpace (原版 h:251)
  #[inline]
  pub fn in_new_space(&self) -> bool {
    self.in_young_generation() && !self.is_large_page()
  }

  /// InNewSpaceBelowAgeMark (原版 h:252-254)
  #[inline]
  pub fn in_new_space_below_age_mark(&self) -> bool {
    self.is_flag_set(NEW_SPACE_BELOW_AGE_MARK)
  }

  /// InNewLargeObjectSpace (原版 h:255-257)
  #[inline]
  pub fn in_new_large_object_space(&self) -> bool {
    self.in_young_generation() && self.is_large_page()
  }

  /// PointersToHereAreInteresting (原版 h:266-268)
  #[inline]
  pub fn pointers_to_here_are_interesting(&self) -> bool {
    self.is_flag_set(POINTERS_TO_HERE_ARE_INTERESTING)
  }

  /// PointersFromHereAreInteresting (原版 h:269-271)
  #[inline]
  pub fn pointers_from_here_are_interesting(&self) -> bool {
    self.is_flag_set(POINTERS_FROM_HERE_ARE_INTERESTING)
  }

  /// Offset — release 式 addr - address() (原版 h:279);
  /// DEBUG 版含 area_start/size 界检 (cc:190-194) 登记推迟
  #[inline]
  pub fn offset(&self, addr: Address) -> usize {
    addr - self.address()
  }

  /// OffsetMaybeOutOfRange — release 式退化为 Offset (原版 h:280; DEBUG 版 cc:196-199 推迟)
  #[inline]
  pub fn offset_maybe_out_of_range(&self, addr: Address) -> usize {
    self.offset(addr)
  }

  /// SynchronizedLoad — 非 TSAN 空实现 (原版 h:295);
  /// TSAN 体 (cc:165-184, 表槽 Acquire_Load + metadata->SynchronizedHeapLoad) 目标配置剥除
  #[inline]
  pub fn synchronized_load(&self) {}

  /// InReadOnlySpace (原版 inl:83-95 非连续 RO 分支 inl:92-94; TSAN 前置 inl:86 空实现承接)。
  /// 连续 RO 地址掩码分支 (inl:87-91, CONTIGUOUS_COMPRESSED_READ_ONLY_SPACE) 非目标配置, 登记不实现。
  #[inline]
  pub fn in_read_only_space(&self) -> bool {
    self.synchronized_load();
    self.is_flag_set(READ_ONLY_HEAP)
  }

  /// IsEvacuationCandidate (原版 inl:73-81; 目标配置仅剩 flag 读取 — DEBUG 双检区 inl:74-79
  /// 依赖 Metadata() 面, 登记推迟)
  #[inline]
  pub fn is_evacuation_candidate(&self) -> bool {
    self.is_flag_set(EVACUATION_CANDIDATE)
  }

  /// metadata_index(): 表索引槽读出 (原版 h:360 字段; Metadata 族 inl 侧消费推迟后先导裸槽)
  #[inline]
  pub fn metadata_index(&self) -> u32 {
    self.metadata_index_
  }
}

// ── chunk 指针 hash (原版 h:382-404, base::hash 特化) ──
/// hash<const MemoryChunk*> / hash<MemoryChunk*> (h:390-397/h:399-404):
/// Tagged_t 截低 32 位 (指针压缩: 弃 cage base) 后 >> kPageSizeBits (弃对齐尾零)。
#[inline]
pub const fn chunk_hash(chunk_address: Address) -> usize {
  ((chunk_address as TaggedT) >> (K_PAGE_SIZE_BITS as TaggedT)) as usize
}

// ══════════════════════════════════════════════════════════════════════════
// MetadataTableIndex (原版 memory-chunk.cc:82-110) — IsolateGroup/CodeRange 耦合面以窄 trait 承接
// ══════════════════════════════════════════════════════════════════════════

/// MetadataTableIndex 三分支 (cc:82-110) 与 ctor/ClearMetadataPointer/MetadataTableAddress
/// (cc:54-79, h:320-322) 所需的 IsolateGroup::current() / CodeRange / TrustedRange 运行态,
/// 由上层实现本窄接口提供 (他票面最小消费集)。
pub trait MetadataTableEnv {
  /// 原版 cc:85 `V8HeapCompressionScheme::base()` — 主 cage 基址
  fn main_cage_base(&self) -> Address;
  /// 原版 cc:91-94 `IsolateGroup::current()->GetTrustedPtrComprCage()->region().contains(addr)`
  fn trusted_cage_contains(&self, chunk_address: Address) -> bool;
  /// 原版 cc:101-103 `IsolateGroup::current()->GetCodeRange()->base()`
  fn code_range_base(&self) -> Address;
}

/// 主 cage 分支纯算术 (原版 cc:84-90): Tagged_t 截断 + 页序 + kMainCageMetadataOffset。
/// 独立导出供无 env 消费与测试金标准。
#[inline]
pub const fn main_cage_metadata_index(chunk_address: Address) -> u32 {
  // 原版 cc:87 Tagged_t offset = CompressAny(chunk_address); cc:89-90 index = 偏移 + (offset >> kPageSizeBits)
  K_MAIN_CAGE_METADATA_OFFSET as u32
    + (V8HeapCompressionScheme::compress_any(chunk_address) >> (K_PAGE_SIZE_BITS as TaggedT))
}

/// MetadataTableIndex (原版 cc:82-110) 1:1:
/// 主 cage (cc:84-90) / trusted cage (cc:91-99) / code range (cc:100-107) 三分支。
pub fn metadata_table_index<E: MetadataTableEnv + ?Sized>(env: &E, chunk_address: Address) -> u32 {
  let index: usize = if V8HeapCompressionScheme::get_ptr_compr_cage_base_address(chunk_address)
    == env.main_cage_base()
  {
    // 原版 cc:84-90
    main_cage_metadata_index(chunk_address) as usize
  } else if env.trusted_cage_contains(chunk_address) {
    // 原版 cc:91-99: TrustedSpaceCompressionScheme::CompressAny + 页序 + trusted 偏移
    K_TRUSTED_SPACE_METADATA_OFFSET
      + ((TrustedSpaceCompressionScheme::compress_any(chunk_address)
        >> (K_PAGE_SIZE_BITS as TaggedT)) as usize)
  } else {
    // 原版 cc:100-107: code_range base 差值截 uint32 + 页序 + code 偏移
    let offset = (chunk_address - env.code_range_base()) as TaggedT;
    K_CODE_RANGE_METADATA_OFFSET + ((offset >> (K_PAGE_SIZE_BITS as TaggedT)) as usize)
  };
  // 原版 cc:88/96/104/108 DCHECK_LT 界检 (目标配置 DEBUG 关; debug_assert 等价承接)
  debug_assert!(index < K_METADATA_POINTER_TABLE_SIZE);
  index as u32
}

// ══════════════════════════════════════════════════════════════════════════
// MemoryChunkLayout (原版 memory-chunk-layout.h:14-67)
// ══════════════════════════════════════════════════════════════════════════

/// InstructionStream::kHeaderSize 目标配置推导值 (x64/arm64 无嵌入式常量池:
/// TrustedObject 头 4 + 保护指针 ×2 各 4 + body_size u32 4 = 16)。
/// 实体属 objects/instruction-stream.h (他票) → 常量参数注入, 推迟双引用见文末台账 (原版 layout:21)。
pub const K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED: usize = 16;

/// 原版 MemoryChunkLayout (layout.h:14-67); constexpr 面落 Rust const fn, 头尺寸按参数注入。
pub struct MemoryChunkLayout;

impl MemoryChunkLayout {
  /// ObjectStartOffsetInCodePage (原版 layout:18-24)。
  /// 参数 = InstructionStream::kHeaderSize (他票注入, 见 K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED)
  #[inline]
  pub const fn object_start_offset_in_code_page(instruction_stream_header_size: usize) -> usize {
    round_up_address(
      size_of::<MemoryChunk>() + instruction_stream_header_size,
      K_CODE_ALIGNMENT,
    ) - instruction_stream_header_size
  }

  /// AllocatableMemoryInCodePage (原版 layout:26-28)
  #[inline]
  pub const fn allocatable_memory_in_code_page(instruction_stream_header_size: usize) -> usize {
    K_REGULAR_PAGE_SIZE - Self::object_start_offset_in_code_page(instruction_stream_header_size)
  }

  /// ObjectStartOffsetInDataPage (原版 layout:30-33) = RoundUp(sizeof(MemoryChunk), ALIGN_TO_ALLOCATION_ALIGNMENT(kDoubleSize))
  #[inline]
  pub const fn object_start_offset_in_data_page() -> usize {
    round_up_address(
      size_of::<MemoryChunk>(),
      align_to_allocation_alignment(K_DOUBLE_SIZE),
    )
  }

  /// AllocatableMemoryInDataPage (原版 layout:35-40);
  /// 原版内断言 kMaxRegularHeapObjectSize <= 可分配区 (layout:38) → Rust const assert 恒检
  #[inline]
  pub const fn allocatable_memory_in_data_page() -> usize {
    K_REGULAR_PAGE_SIZE - Self::object_start_offset_in_data_page()
  }

  /// ObjectStartOffsetInMemoryChunk (原版 layout:42-49): 码空间走码页布局, 其余走数据页
  /// (只读页与常规页同布局, layout:47-48 注释面)
  #[inline]
  pub const fn object_start_offset_in_memory_chunk(
    space: AllocationSpace,
    instruction_stream_header_size: usize,
  ) -> usize {
    if is_any_code_space(space) {
      Self::object_start_offset_in_code_page(instruction_stream_header_size)
    } else {
      Self::object_start_offset_in_data_page()
    }
  }

  /// AllocatableMemoryInMemoryChunk (原版 layout:51-59): CODE_SPACE 走码页, 其余数据页。
  /// 原版 DCHECK_NE(space, CODE_LO_SPACE) (layout:53) release 剥除, Rust 不复活 (台账注)
  #[inline]
  pub const fn allocatable_memory_in_memory_chunk(
    space: AllocationSpace,
    instruction_stream_header_size: usize,
  ) -> usize {
    if matches!(space, AllocationSpace::CodeSpace) {
      Self::allocatable_memory_in_code_page(instruction_stream_header_size)
    } else {
      Self::allocatable_memory_in_data_page()
    }
  }

  /// MaxRegularCodeObjectSize (原版 layout:61-66) = RoundDown(码页可分配/2, kTaggedSize)
  #[inline]
  pub const fn max_regular_code_object_size(instruction_stream_header_size: usize) -> i32 {
    let half = Self::allocatable_memory_in_code_page(instruction_stream_header_size) / 2;
    // RoundDown(x, kTaggedSize) — kTaggedSize 为 2 的幂 (截断对齐)
    (half & !(K_TAGGED_SIZE - 1)) as i32
  }
}

// 原版 layout:38 / layout:64 两枚 static_assert, 以推导头尺寸代入恒检
const _: () =
  assert!(K_MAX_REGULAR_HEAP_OBJECT_SIZE <= MemoryChunkLayout::allocatable_memory_in_data_page());
const _: () = assert!(
  MemoryChunkLayout::max_regular_code_object_size(K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED)
    as usize
    <= K_MAX_REGULAR_HEAP_OBJECT_SIZE
);

// ══════════════════════════════════════════════════════════════════════════
// 推迟台账 (Isolate / heap 真实耦合面; 双行号 + 原因; 台账锚见 tool/src/heap/*.js 落地臂)
// ══════════════════════════════════════════════════════════════════════════
// 1. Metadata 族 8 定义 — 原版 inl:18-41 (MetadataImpl 可变, 表查找+isolate 校验+chunk 回指
//    CHECK_EQ inl:36)、inl:44-47 (const 版)、inl:49-55 (Metadata(isolate) ×2)、inl:57-63
//    (Metadata() ×2, Isolate::Current inl:58)、inl:65-71 (MetadataNoIsolateCheck ×2)、
//    声明 h:187-194/338-341。原因: IsolateGroup 表 + BasePage + Isolate 均他票;
//    Rust 侧 opaque BasePage 与 MetadataTableEnv trait 已铺好承接位。
// 2. MetadataTableAddress — 原版 h:320-322 (IsolateGroup::current()->metadata_pointer_table())。
//    原因: IsolateGroup 单例他票 → MetadataTableEnv 面。
// 3. ctor 表写入分支 — 原版 cc:53-62 (SetMetadata + Isolate::FromHeap)。原因: Isolate/BasePage 堆耦合。
// 4. ClearMetadataPointer — 原版 cc:67-79 (声明 h:284)。原因: 表读写 + BasePage::ChunkAddress 他票。
// 5. SandboxSafeInReadOnlySpace — 原版 cc:112-130 (声明 h:289)。原因: ReadOnlyPage 反查 (只读页他票)。
// 6. DEBUG Offset/OffsetMaybeOutOfRange — 原版 cc:190-199。原因: Metadata()->area_start/size 他票;
//    release 式 (h:279-280) 已落地。
// 7. TSAN SynchronizedLoad 体 — 原版 cc:165-184。原因: 目标配置 TSAN 关 (展开剥除);
//    空实现 h:295 已落地。InitializationMemoryFence 的 TSAN 注释区 cc:137-160 同理剥除。
// 8. sticky markbits 面 — ContainsOnlyOldObjects h:208-215 / IsOnlyOldOrMajorMarkingOn h:258-265 /
//    kIsOnlyOldOrMajorGCInProgressMask h:140-142。原因: V8_ENABLE_STICKY_MARK_BITS 非目标配置
//    (原版 #else UNREACHABLE, 本席禁假绿禁 panic 面, 不落地); 枚举面值 h:103/h:107 照录保台账。
// 9. 非沙箱 metadata_ 直回指形态 — 原版 h:362 / MetadataOffset h:332-334。原因: 目标配置 sandbox
//    开 (展开侧 metadata_ 剥除 0 残留); with_metadata_direct 仅保字段承接位。
// 10. InstructionStream::kHeaderSize 实体 — 原版 layout:21 (objects/instruction-stream.h 他票)。
//     推导值 K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED=16 参数注入, layout:30/61 消费链已落。
// 11. friend 面 h:325-328/366-375 与 DEFINE_OPERATORS_FOR_FLAGS h:378 — Rust 无友元/宏操作符,
//     访问控制以 pub 导出承接 (同 linear_allocation_area 先例)。
