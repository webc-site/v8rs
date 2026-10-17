//! tests/memory_chunk_unittest.rs — v8-heap-core/src/memory_chunk.rs 金标准边界用例 (HC-1R2a)。
//!
//! 原版测试勘定实测: v8/test/unittests/heap 无 memory-chunk 专属 unittest (grep 实测仅
//! heap-unittest.cc / spaces-unittest.cc / page-promotion-unittest.cc / mark-compact-unittest.cc
//! 与 code-pages-unittest.cc 消费 MemoryChunk* 面, 全部依赖 Isolate/Heap 运行态, 不可直迁)。
//! 本档按票面纪律: 无专属 unittest 的语义面按原版 DCHECK/注释写金标准边界用例;
//! 尺寸/掩码/Flag 值以 tool/src/heap/memory-chunk-*.js clang -E 展开侧数值为准。
//!
//! 用例 ↔ 原版映射 (逐函数见各 test 头注):
//!   flag/mask 值 ← memory-chunk.h:56-143 (枚举与掩码 constexpr)
//!   结构布局 ← h:301-303/314-316 + cc:25-26 static_assert
//!   对齐/归位 ← h:145-181 (BaseAddress/IsAligned/AddressToOffset/FromAddress/FromHeapObject;
//!                语义锚 heap-unittest.cc:922-929 FromAddress 往返 — 原用例 Isolate 态, 此处纯地址承接)
//!   chunk hash ← h:390-404 (base::hash 特化)
//!   常量 9 项 ← memory-chunk-constants.h:18-36 (skel 实测锚 16384/4096/32768)
//!   表索引 ← memory-chunk.cc:82-110 三分支 (cc:88/96/104/108 DCHECK_LT 界值)
//!   布局 7 函数 ← memory-chunk-layout.h:18-66 (layout:38/64 static_assert 界)
//!   旗标语义 ← h:196-271 谓词矩阵 + inl:73-95 实体

use core::{mem::size_of, ptr::null_mut};

use aok::{OK, Void};
use log::info;
use v8_base::bits::bit_width64;
use v8_common::{
  Address, AllocationSpace, K_CODE_ALIGNMENT, K_MAX_REGULAR_HEAP_OBJECT_SIZE,
  K_PTR_COMPR_CAGE_RESERVATION_SIZE, K_REGULAR_PAGE_SIZE, K_TAGGED_SIZE,
};
use v8_heap_core::memory_chunk::*;
use v8_objects_core::tagged::{Object, Tagged};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// Flag 14 逐值金标准 (h:56-109; 值域=源码与 clang -E 展开侧对数双口径, sticky ×2 照录面值)
#[test]
fn chunk_flag_values_gold() -> Void {
  // h:57-97 目标配置 12 项
  assert_eq!(0, NO_FLAGS);
  assert_eq!(1, IN_WRITABLE_SHARED_SPACE);
  assert_eq!(2, POINTERS_TO_HERE_ARE_INTERESTING);
  assert_eq!(4, POINTERS_FROM_HERE_ARE_INTERESTING);
  assert_eq!(8, FROM_PAGE);
  assert_eq!(16, TO_PAGE);
  assert_eq!(32, INCREMENTAL_MARKING);
  assert_eq!(64, BLACK_ALLOCATED);
  assert_eq!(128, LARGE_PAGE);
  assert_eq!(256, EVACUATION_CANDIDATE);
  assert_eq!(512, NEW_SPACE_BELOW_AGE_MARK);
  assert_eq!(1024, READ_ONLY_HEAP);
  // h:103/107 非目标配置面值照录 (推迟台账 8 侧翼)
  assert_eq!(2048, STICKY_MARK_BIT_CONTAINS_ONLY_OLD);
  assert_eq!(4096, STICKY_MARK_BIT_IS_MAJOR_GC_IN_PROGRESS);
  info!("chunk_flag_values_gold ok");
  OK
}

/// 掩码常量金标准 (h:113-143; 12 项, h:140-142 sticky 掩码不落地)
#[test]
fn chunk_masks_gold() -> Void {
  // h:113 ~NO_FLAGS → 全位
  assert_eq!(usize::MAX, K_ALL_FLAGS_MASK);
  assert_eq!(2, K_POINTERS_TO_HERE_ARE_INTERESTING_MASK);
  assert_eq!(4, K_POINTERS_FROM_HERE_ARE_INTERESTING_MASK);
  assert_eq!(256, K_EVACUATION_CANDIDATE_MASK);
  // h:120-121 FROM|TO = 8|16
  assert_eq!(24, K_IS_IN_YOUNG_GENERATION_MASK);
  assert_eq!(128, K_IS_LARGE_PAGE_MASK);
  assert_eq!(1, K_IN_SHARED_HEAP);
  assert_eq!(32, K_INCREMENTAL_MARKING);
  // h:125-127 EVAC | FROM | TO
  assert_eq!(256 | 8 | 16, K_SKIP_EVACUATION_SLOTS_RECORDING_MASK);
  // h:128-130 BLACK | SHARED
  assert_eq!(64 | 1, K_IS_BLACK_ALLOCATED_OR_WRITEABLE_SHARED_MASK);
  // h:133/134-136 非连续 RO 面 (目标配置保留)
  assert_eq!(1024, K_IS_IN_READ_ONLY_HEAP_MASK);
  assert_eq!(1024 | 1, K_IS_READ_ONLY_OR_SHARED_HEAP_MASK);
  info!("chunk_masks_gold ok");
  OK
}

/// 结构布局金标准 (h:357+360 双字段沙箱形态; FlagsOffset h:301 / MetadataIndexOffset h:314;
/// cc:25-26 清弱引用断言的 sizeof 侧)
#[test]
fn chunk_layout_gold() -> Void {
  // skel 台账: repr(C) { usize@0, u32@8 } → 16
  assert_eq!(16, size_of::<MemoryChunk>());
  assert_eq!(0, MemoryChunk::flags_offset());
  assert_eq!(8, MemoryChunk::metadata_index_offset());
  // cc:25-26 (kClearedWeakHeapObjectLower32 ∈ (0, sizeof(MemoryChunk))) — 展开侧实测值 3
  assert_eq!(3, v8_common::K_CLEARED_WEAK_HEAP_OBJECT_LOWER32);
  assert!((v8_common::K_CLEARED_WEAK_HEAP_OBJECT_LOWER32 as usize) < size_of::<MemoryChunk>());
  info!("chunk_layout_gold ok");
  OK
}

/// 对齐面金标准 (h:145-172: BaseAddress/IsAligned/GetAlignmentForAllocation(161)/
/// GetAlignmentMaskForAssembler(166)/AddressToOffset(170); h:305-307 kAlignment)
#[test]
fn chunk_alignment_gold() -> Void {
  // h:305-306 kAlignment = 1<<kPageSizeBits = 页尺寸; h:307 mask = kAlignment-1
  assert_eq!(1 << 18, MemoryChunk::alignment());
  assert_eq!(K_REGULAR_PAGE_SIZE, MemoryChunk::alignment());
  assert_eq!(K_REGULAR_PAGE_SIZE - 1, MemoryChunk::alignment_mask());
  assert_eq!(262144isize, MemoryChunk::get_alignment_for_allocation());
  assert_eq!(262143isize, MemoryChunk::get_alignment_mask_for_assembler());

  // h:157-159 IsAligned 边界: 0 页边界对齐; ±1 不对齐 (h:242-244 对象 tag 低位语义同源)
  assert!(MemoryChunk::is_aligned(0));
  assert!(MemoryChunk::is_aligned(K_REGULAR_PAGE_SIZE));
  assert!(!MemoryChunk::is_aligned(K_REGULAR_PAGE_SIZE - 1));
  assert!(!MemoryChunk::is_aligned(K_REGULAR_PAGE_SIZE + 1));

  // h:145-155 BaseAddress: 向下归位; 已对齐地址不动 (原版 LINT 面 codegen 同式)
  assert_eq!(0, MemoryChunk::base_address(0));
  assert_eq!(0x40000, MemoryChunk::base_address(0x40000));
  assert_eq!(0x40000, MemoryChunk::base_address(0x40000 + 3)); // heap object 强引用 tag=01
  assert_eq!(0x40000, MemoryChunk::base_address(0x5FFFF)); // 页窗上沿
  assert_eq!(
    0x70_0000 - K_REGULAR_PAGE_SIZE,
    MemoryChunk::base_address(0x70_0000 - 1)
  );

  // h:170-172 AddressToOffset: 先截低 32 位再掩 (压缩堆位型, 4GB 回绕边界金标准)
  assert_eq!(0u32, MemoryChunk::address_to_offset(0));
  assert_eq!(8u32, MemoryChunk::address_to_offset(0x1_0000_0008)); // 低 32 位截断后 8
  assert_eq!(262143u32, MemoryChunk::address_to_offset(0xFFFF_FFFF));
  assert_eq!(
    0u32,
    MemoryChunk::address_to_offset(K_PTR_COMPR_CAGE_RESERVATION_SIZE)
  );
  info!("chunk_alignment_gold ok");
  OK
}

/// FromAddress/FromHeapObject 往返金标准 (h:174-181)。
/// 语义锚: v8/test/unittests/heap/heap-unittest.cc:922-929
/// `ContainsSlow(FromAddress(arr->address())->address())` — 原用例需 Isolate 态 (不可迁),
/// 此处承接其纯不变量: 归位幂等 + 同页窗内任意地址 (含 tag 位) 归到同一块。
#[test]
fn chunk_from_address_roundtrip() -> Void {
  for raw in [
    0usize,
    3,
    0x4_0000,
    0x4_0001,
    0x5_FFFF,
    0x1_0000_0008,
    0x002A_BCDE_0123,
  ] {
    let base = MemoryChunk::base_address(raw);
    // 归位幂等 (base 必对齐)
    assert!(MemoryChunk::is_aligned(base));
    assert_eq!(base, MemoryChunk::base_address(base));
    // FromAddress(h:174-176) 位型: reinterpret_cast → 指针值即 base 地址
    assert_eq!(base, MemoryChunk::from_address(raw) as Address);
    // 同页窗内任意地址归一 (FromHeapObject h:178-181 经 object.ptr() 含 tag 同源)
    for delta in [0usize, 1, 2, 3, 8, K_REGULAR_PAGE_SIZE - 1] {
      let addr = base + delta;
      assert_eq!(base, MemoryChunk::from_address(addr) as Address);
      let tagged: Tagged<Object> = Tagged::new(addr);
      assert_eq!(base, MemoryChunk::from_heap_object(tagged) as Address);
    }
  }
  info!("chunk_from_address_roundtrip ok");
  OK
}

/// chunk hash 金标准 (h:390-404): Tagged_t 截低 32 位 (弃 cage base, h:388-389 注释) 后
/// >> kPageSizeBits (弃对齐尾零)。
#[test]
fn chunk_hash_gold() -> Void {
  // h:393-395 展开式: (addr as u32) >> 18
  assert_eq!(0usize, chunk_hash(0));
  assert_eq!(7usize, chunk_hash(0x3_001C_0000)); // cage 高位 3 弃, 低 32 页序 7
  assert_eq!(0usize, chunk_hash(0x1_0000_0000)); // 纯 cage 基: 低 32 截断为 0 → hash 0 (h:388-389 注释语义)
  assert_eq!(1usize, chunk_hash(K_REGULAR_PAGE_SIZE));
  assert_eq!(1usize, chunk_hash(K_REGULAR_PAGE_SIZE + 3)); // 页内 tag 尾零被弃
  assert_eq!(
    1023usize,
    chunk_hash(K_MAIN_CAGE_METADATA_OFFSET as Address + 1023 * K_REGULAR_PAGE_SIZE)
  );
  info!("chunk_hash_gold ok");
  OK
}

/// MemoryChunkConstants 9 项金标准 (constants.h:18-36; skel 实测锚 16384/4096 + 展开式重算)
#[test]
fn chunk_constants_gold() -> Void {
  // constants.h:18-19 实测锚 (memory-chunk-constants.h.js 对数: 2^32/2^18 = 16384)
  assert_eq!(16384, K_PAGES_IN_MAIN_CAGE);
  assert_eq!(
    K_PTR_COMPR_CAGE_RESERVATION_SIZE / K_REGULAR_PAGE_SIZE,
    K_PAGES_IN_MAIN_CAGE
  );
  // constants.h:20-21 cfg 双形态 (x64 512MB→2048 / arm64 256MB→1024), 与 v8-common 同源
  assert_eq!(
    v8_common::K_MAXIMAL_CODE_RANGE_SIZE / K_REGULAR_PAGE_SIZE,
    K_PAGES_IN_CODE_CAGE
  );
  // constants.h:22-23 实测锚 2^30/2^18 = 4096
  assert_eq!(4096, K_PAGES_IN_TRUSTED_CAGE);
  // constants.h:25-29 三段表偏移
  assert_eq!(0, K_MAIN_CAGE_METADATA_OFFSET);
  assert_eq!(16384, K_TRUSTED_SPACE_METADATA_OFFSET);
  assert_eq!(16384 + 4096, K_CODE_RANGE_METADATA_OFFSET);
  // constants.h:31-36 BitWidth(主+码+信) → 1<<log2 → -1 (两架构 x64 22528 / arm64 21504 均落 15 位)
  let total = K_PAGES_IN_MAIN_CAGE + K_PAGES_IN_CODE_CAGE + K_PAGES_IN_TRUSTED_CAGE;
  assert_eq!(
    bit_width64(total as u64) as usize,
    K_METADATA_POINTER_TABLE_SIZE_LOG2
  );
  assert_eq!(15, K_METADATA_POINTER_TABLE_SIZE_LOG2);
  assert_eq!(32768, K_METADATA_POINTER_TABLE_SIZE);
  assert_eq!(32767, K_METADATA_POINTER_TABLE_SIZE_MASK);
  info!("chunk_constants_gold ok");
  OK
}

/// main_cage_metadata_index (cc:84-90) 页序算术金标准 (cc:88 DCHECK_LT 界语义)
#[test]
fn main_cage_index_gold() -> Void {
  assert_eq!(0u32, main_cage_metadata_index(0));
  assert_eq!(5u32, main_cage_metadata_index(5 * K_REGULAR_PAGE_SIZE));
  // cc:87 CompressAny 截低 32: 高 cage 位不影响页序
  assert_eq!(
    9u32,
    main_cage_metadata_index((1usize << 32) + 9 * K_REGULAR_PAGE_SIZE)
  );
  // cc:88 界: 主 cage 最大页序 16383 (2^32-2^18 低 32 截断后)
  assert_eq!(
    16383u32,
    main_cage_metadata_index((1usize << 32) - K_REGULAR_PAGE_SIZE)
  );
  info!("main_cage_index_gold ok");
  OK
}

/// 测试期 MetadataTableEnv 实现 (三分支运行态注入; 他票 IsolateGroup 面的最小替身)
struct TestEnv {
  /// cc:85 V8HeapCompressionScheme::base() 替身
  main_base: Address,
  /// cc:91-94 trusted region contains 替身 (命中地址上界)
  trusted_addr: Option<Address>,
  /// cc:101-103 code_range->base() 替身
  code_base: Address,
}

impl MetadataTableEnv for TestEnv {
  fn main_cage_base(&self) -> Address {
    self.main_base
  }
  fn trusted_cage_contains(&self, chunk_address: Address) -> bool {
    self.trusted_addr == Some(chunk_address)
  }
  fn code_range_base(&self) -> Address {
    self.code_base
  }
}

/// metadata_table_index 三分支金标准 (cc:82-110; cc:88/96/104 页序界 + cc:108 表界 debug_assert
/// 在 debug 构建下实测激活 — 越界会在本用例 panic, 故各分支取界内代表值)
#[test]
fn metadata_table_index_three_branches() -> Void {
  // 分支 1 主 cage (cc:84-90): 地址 4GB cage 基与 env.main_base 重合
  let env = TestEnv {
    main_base: 0,
    trusted_addr: None,
    code_base: 0,
  };
  assert_eq!(7u32, metadata_table_index(&env, 0x1C_0000)); // 7 * 2^18
  assert_eq!(0u32, metadata_table_index(&env, 0));

  // 分支 2 trusted cage (cc:91-99): 主 cage 判定落空 (地址 cage 基 != main_base) 后命中 trusted
  let addr_trusted: Address = 0x1_1000_0000; // cage 基 0x1_0000_0000 ≠ 0; trusted 界内 (256MB, 页序 1024 < 4096, cc:96 界)
  let env2 = TestEnv {
    main_base: 0,
    trusted_addr: Some(addr_trusted),
    code_base: 0,
  };
  // cc:95-99: 低 32 = 0x1000_0000 → 页序 1024 → 16384 + 1024
  assert_eq!(16384 + 1024, metadata_table_index(&env2, addr_trusted));

  // 分支 3 code range (cc:100-107): 尾分支, base 差值截 uint32
  let addr_code: Address = 0x2_0008_0000;
  let env3 = TestEnv {
    main_base: 0,
    trusted_addr: None,
    code_base: 0x2_0000_0000,
  };
  // cc:103-106: offset 0x8_0000 → 页序 2 → 20480 + 2
  assert_eq!(20480 + 2, metadata_table_index(&env3, addr_code));
  info!("metadata_table_index_three_branches ok");
  OK
}

/// 布局 7 函数数值链金标准 (layout:18-66; skel 台账: 数据页 16/262128, 码页随 kCodeAlignment cfg)
#[test]
fn layout_fns_gold() -> Void {
  let h = K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED; // [P9] 注入值 16
  // layout:30-33 数据页: RoundUp(16, ALIGN_TO_ALLOCATION_ALIGNMENT(8)) = 16
  assert_eq!(16, MemoryChunkLayout::object_start_offset_in_data_page());
  // layout:35-40: 2^18 - 16 = 262128; layout:38 内断言侧 131072 <= 262128
  assert_eq!(262128, MemoryChunkLayout::allocatable_memory_in_data_page());
  assert!(K_MAX_REGULAR_HEAP_OBJECT_SIZE <= MemoryChunkLayout::allocatable_memory_in_data_page());

  // layout:18-24 码页 (cfg 双形态: kCodeAlignment x64 64 / arm64 32):
  let expected_start = ((32 + K_CODE_ALIGNMENT - 1) & !(K_CODE_ALIGNMENT - 1)) - h;
  assert_eq!(
    expected_start,
    MemoryChunkLayout::object_start_offset_in_code_page(h)
  );
  // layout:26-28
  assert_eq!(
    K_REGULAR_PAGE_SIZE - expected_start,
    MemoryChunkLayout::allocatable_memory_in_code_page(h)
  );
  // layout:61-66: RoundDown(码页可分配/2, kTaggedSize); layout:64 内断言侧
  let half = MemoryChunkLayout::allocatable_memory_in_code_page(h) / 2;
  assert_eq!(
    (half & !(K_TAGGED_SIZE - 1)) as i32,
    MemoryChunkLayout::max_regular_code_object_size(h)
  );
  assert!(
    (MemoryChunkLayout::max_regular_code_object_size(h) as usize) <= K_MAX_REGULAR_HEAP_OBJECT_SIZE
  );

  // layout:42-49 ObjectStartOffsetInMemoryChunk: 码空间 (含 CODE_LO, IsAnyCodeSpace) → 码页式
  assert_eq!(
    expected_start,
    MemoryChunkLayout::object_start_offset_in_memory_chunk(AllocationSpace::CodeSpace, h)
  );
  assert_eq!(
    expected_start,
    MemoryChunkLayout::object_start_offset_in_memory_chunk(AllocationSpace::CodeLoSpace, h)
  );
  // 其余空间与数据页同式 (RO 同常规, layout:47-48 注释)
  for space in [
    AllocationSpace::RoSpace,
    AllocationSpace::NewSpace,
    AllocationSpace::OldSpace,
    AllocationSpace::TrustedSpace,
    AllocationSpace::LoSpace,
  ] {
    assert_eq!(
      16,
      MemoryChunkLayout::object_start_offset_in_memory_chunk(space, h)
    );
  }

  // layout:51-59 AllocatableMemoryInMemoryChunk: 仅 CODE_SPACE 走码页 (layout:54);
  // CODE_LO_SPACE 落数据页式 — 与原版 release 语义一致 (DCHECK_NE layout:53 剥除不复活)
  assert_eq!(
    K_REGULAR_PAGE_SIZE - expected_start,
    MemoryChunkLayout::allocatable_memory_in_memory_chunk(AllocationSpace::CodeSpace, h)
  );
  assert_eq!(
    262128,
    MemoryChunkLayout::allocatable_memory_in_memory_chunk(AllocationSpace::CodeLoSpace, h)
  );
  assert_eq!(
    262128,
    MemoryChunkLayout::allocatable_memory_in_memory_chunk(AllocationSpace::OldSpace, h)
  );
  info!("layout_fns_gold ok");
  OK
}

/// 旗标谓词矩阵金标准 (h:196-271 谓词族 + inl:73-81/83-95 实体面)
#[test]
fn flag_predicate_matrix() -> Void {
  // NO_FLAGS (h:57): 全谓词 false; IsFlagSet(0) 与原版 `flags & NO_FLAGS` 同为 false (h:347-349)
  let none = MemoryChunk::with_metadata_index(NO_FLAGS, 0);
  assert!(!none.is_marking());
  assert!(!none.in_young_generation());
  assert!(!none.is_flag_set(NO_FLAGS));
  assert!(!none.should_skip_evacuation_slot_recording());
  assert!(!none.in_read_only_space());

  // 单旗逐项翻转对应谓词 (h:196-271 各定义式)
  type ChunkPred = fn(&MemoryChunk) -> bool;
  let cases: &[(MainThreadFlags, ChunkPred)] = &[
    (INCREMENTAL_MARKING, |c| c.is_marking()),
    (IN_WRITABLE_SHARED_SPACE, |c| c.in_writable_shared_space()),
    (BLACK_ALLOCATED, |c| c.is_black_allocated()),
    (BLACK_ALLOCATED, |c| {
      c.is_black_allocated_or_writable_shared()
    }),
    (FROM_PAGE, |c| c.is_from_page()),
    (FROM_PAGE, |c| c.in_young_generation()),
    (TO_PAGE, |c| c.is_to_page()),
    (TO_PAGE, |c| c.in_young_generation()),
    (LARGE_PAGE, |c| c.is_large_page()),
    (EVACUATION_CANDIDATE, |c| c.is_evacuation_candidate()),
    (EVACUATION_CANDIDATE, |c| {
      c.should_skip_evacuation_slot_recording()
    }),
    (NEW_SPACE_BELOW_AGE_MARK, |c| {
      c.in_new_space_below_age_mark()
    }),
    (POINTERS_TO_HERE_ARE_INTERESTING, |c| {
      c.pointers_to_here_are_interesting()
    }),
    (POINTERS_FROM_HERE_ARE_INTERESTING, |c| {
      c.pointers_from_here_are_interesting()
    }),
    (READ_ONLY_HEAP, |c| c.in_read_only_space()),
    (READ_ONLY_HEAP, |c| {
      !c.is_black_allocated_or_writable_shared()
    }),
  ];
  for (flag, pred) in cases {
    let c = MemoryChunk::with_metadata_index(*flag, 7);
    assert!(pred(&c), "flag={flag:#b}");
  }

  // InNewSpace/InNewLargeObjectSpace 组合式 (h:251/255-257): 年轻代 × 大页矩阵
  assert!(MemoryChunk::with_metadata_index(TO_PAGE, 0).in_new_space());
  assert!(!MemoryChunk::with_metadata_index(TO_PAGE | LARGE_PAGE, 0).in_new_space());
  assert!(MemoryChunk::with_metadata_index(FROM_PAGE | LARGE_PAGE, 0).in_new_large_object_space());
  // IsYoungOrSharedChunk (h:224-228): FROM|TO|SHARED 三旗任一
  assert!(MemoryChunk::with_metadata_index(IN_WRITABLE_SHARED_SPACE, 0).is_young_or_shared_chunk());
  assert!(!MemoryChunk::with_metadata_index(BLACK_ALLOCATED, 0).is_young_or_shared_chunk());

  // 谓词全 false 的旗 (LARGE_PAGE 单旗) 不误伤
  let lp = MemoryChunk::with_metadata_index(LARGE_PAGE, 0);
  assert!(!lp.in_young_generation());
  assert!(!lp.is_marking());
  info!("flag_predicate_matrix ok");
  OK
}

/// ctor 数据面 + address/offset release 式金标准 (cc:46-63 数据侧 [P3]; h:185/279-280;
/// 推迟 6 的 DEBUG 界检不激活断言)
#[test]
fn ctor_and_offset_release() -> Void {
  let mut chunk = MemoryChunk::with_metadata_index(FROM_PAGE | TO_PAGE, 42);
  assert_eq!(FROM_PAGE | TO_PAGE, chunk.get_flags());
  assert_eq!(42, chunk.metadata_index());

  // h:185 address() = this 位型; h:279 Offset = addr - address() (release 式)
  let a = chunk.address();
  assert_eq!(0, chunk.offset(a));
  assert_eq!(100, chunk.offset(a + 100));
  // h:275-276 注释: 尾地址允许越出分配区 → release 式无界检, 同值退化
  assert_eq!(1000, chunk.offset_maybe_out_of_range(a + 1000));

  // cc:48-51 非沙箱形态承接位 (推迟 9): 旗标照存, index 槽置哨兵
  let direct = MemoryChunk::with_metadata_direct(BLACK_ALLOCATED, null_mut());
  assert_eq!(BLACK_ALLOCATED, direct.get_flags());
  assert_eq!(u32::MAX, direct.metadata_index());

  // cc:134-136 fence 与 h:295 空 SynchronizedLoad: 调用不 panic 即语义 (TSAN 区剥除)
  chunk.initialization_memory_fence();
  chunk.synchronized_load();
  info!("ctor_and_offset_release ok");
  OK
}

/// 不变量收束: alignment == kRegularPageSize == 页尺寸常量三方同源
/// (h:305-306 ↔ v8-internal.h:120 kPageSizeBits ↔ v8-common internal.rs:55-56)
#[test]
fn page_size_cfg_duality() -> Void {
  assert_eq!(K_REGULAR_PAGE_SIZE, 1 << 18);
  assert_eq!(MemoryChunk::alignment(), K_REGULAR_PAGE_SIZE);
  assert_eq!(18, K_PAGE_SIZE_BITS_ANCHOR);
  info!("page_size_cfg_duality ok");
  OK
}

/// K_PAGE_SIZE_BITS 锚 (v8-common internal.rs:55; h:306 消费面)
const K_PAGE_SIZE_BITS_ANCHOR: usize = v8_common::K_PAGE_SIZE_BITS;
