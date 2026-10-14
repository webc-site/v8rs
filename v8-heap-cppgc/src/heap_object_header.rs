// 由 tool/src/heap/cppgc-internal/heap-object-header.js 从 v8/src/heap/cppgc-internal/heap-object-header.h
// + src/heap/cppgc-internal/heap-object-header.cc 生成; 修订就地改, 复跑覆盖
// 对象头: 双 u16 半字位域 (高: GCInfoIndex|unused|fully_constructed / 低: mark_bit|size) + 页内 u32 链位
// 推迟台账 (详账见 readme/zh.md): SetNextUnfinalized/GetNextUnfinalized (h:328-353, 依赖 CagedHeap
// 偏移换算) 随 caged_heap 批次; 无名臂 GetName() (cc:41-43, 依赖 BasePage::FromPayload + HeapBase
// .name_of_unnamed_object) 随 heap_page/heap_base 批次; ASAN_UNPOISON 臂 (cc:28-34) 属 sanitizer 裁剪面。

use core::{ffi::c_void, ptr::from_ref};
use std::sync::atomic::{AtomicU16, Ordering};

use crate::{
  gc_info::{GcInfoIndex, K_MAX_GC_INFO_INDEX},
  gc_info_table::GlobalGcInfoTable,
  globals::{AccessMode, Address, K_ALLOCATION_GRANULARITY, K_FREE_LIST_GC_INFO_INDEX},
  name_trait::{HeapObjectName, HeapObjectNameForUnnamedObject},
  visitor::Visitor,
};

/// v8: src/heap/cppgc-internal/heap-object-header.h:60 HeapObjectHeader::kSizeLog2
pub const K_SIZE_LOG2: usize = 17;
/// v8: src/heap/cppgc-internal/heap-object-header.h:61 HeapObjectHeader::kMaxSize
pub const K_MAX_SIZE: usize = (1usize << K_SIZE_LOG2) - 1;
/// v8: src/heap/cppgc-internal/heap-object-header.h:62 HeapObjectHeader::kLargeObjectSizeInHeader
pub const K_LARGE_OBJECT_SIZE_IN_HEADER: u16 = 0;

/// v8: heap-object-header.h:129 GCInfoIndexField = BitField16<GCInfoIndex, 0, 14>
const K_GC_INFO_INDEX_MASK: u16 = 0x3FFF;
/// v8: heap-object-header.h:131 FullyConstructedField = UnusedField1::Next<bool, 1> → 高半字 bit15
/// (h:21-22 与 include/cppgc/internal/api-constants.h:31 kFullyConstructedBitMask = 1 << 15 static_assert)
const K_FULLY_CONSTRUCTED_MASK: u16 = 1 << 15;
/// v8: heap-object-header.h:133 MarkBitField = BitField16<bool, 0, 1> → 低半字 bit0
const K_MARK_BIT_MASK: u16 = 0x1;
/// v8: heap-object-header.h:134-135 SizeField = MarkBitField::Next<size_t, 15> → 低半字 bit1..15
const K_SIZE_MASK: u16 = 0xFFFE;

/// v8: heap-object-header.h:126 EncodedHalf (private enum class EncodedHalf { kLow, kHigh })
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EncodedHalf {
  Low,
  High,
}

/// v8: src/heap/cppgc-internal/heap-object-header.h:58-167 HeapObjectHeader
/// 布局 (64 位 + CPPGC_CAGED_HEAP): [next_unfinalized_: u32][encoded_high_: u16][encoded_low_: u16]
/// - encoded_high_: GCInfoIndex(14) | unused(1) | fully_constructed(1, 构造中编码为 false)
/// - encoded_low_:  mark_bit(1) | size(15, 编码值 = 尺寸 / kAllocationGranularity)
///
/// 两半字用原子量存储以承接原版 atomic_ref 双臂 (h:363-394): kNonAtomic 走 Relaxed 载 /
/// get_mut 纯存 (与原版非原子路径同码), kAtomic 走 acquire / release / acq_rel。
#[repr(C)]
pub struct HeapObjectHeader {
  /// v8: heap-object-header.h:160 next_unfinalized_ (CPPGC_CAGED_HEAP) — cage 内偏移, sweeper 未终结链
  next_unfinalized: u32,
  /// v8: heap-object-header.h:165 encoded_high_
  encoded_high: AtomicU16,
  /// v8: heap-object-header.h:166 encoded_low_
  encoded_low: AtomicU16,
}

/// v8: heap-object-header.h:169-171 static_assert(kAllocationGranularity == sizeof(HeapObjectHeader))
const _: () = assert!(size_of::<HeapObjectHeader>() == K_ALLOCATION_GRANULARITY);
/// v8: heap-object-header.cc:18 static_assert(kAllocationGranularity % sizeof(HeapObjectHeader) == 0)
const _: () = assert!(K_ALLOCATION_GRANULARITY.is_multiple_of(size_of::<HeapObjectHeader>()));
/// v8: heap-object-header.cc:20-25 CheckApiConstants 双 static_assert — 位罩与半字距 payload 偏移
const _: () = assert!(K_FULLY_CONSTRUCTED_MASK == 1u16 << 15);
const _: () = assert!(2 * size_of::<u16>() == size_of::<HeapObjectHeader>() - size_of::<u32>());

impl HeapObjectHeader {
  /// v8: heap-object-header.h:65 HeapObjectHeader::FromObject(const void*)
  /// 头紧邻 payload 之前; 原版 reinterpret_cast 回退 sizeof(header)。
  ///
  /// # Safety
  /// `object` 必须指向一个已分配对象 (payload 起点), 且其前置 8 字节为有效 HeapObjectHeader 内存。
  pub unsafe fn from_object<'a>(object: *const c_void) -> &'a Self {
    // SAFETY: 调用契约保证 object 前置 sizeof(Self) 字节为活对象头 (原版 reinterpret_cast 同形)。
    unsafe { &*((object as *const u8).sub(size_of::<Self>())).cast::<Self>() }
  }

  /// v8: heap-object-header.h:64 HeapObjectHeader::FromObject(void*)
  ///
  /// # Safety
  /// 同 `from_object`, 且返回的可变引用不得与既有借用别名冲突 (原版 mutable overload)。
  pub unsafe fn from_object_mut<'a>(object: *mut c_void) -> &'a mut Self {
    // SAFETY: 调用契约同 from_object; 可变借用由调用方独占性保证。
    unsafe { &mut *((object as *mut u8).sub(size_of::<Self>())).cast::<Self>() }
  }

  /// v8: heap-object-header.h:185-220 HeapObjectHeader(size, gc_info_index)
  /// 64 位小端整字 = index<<32 | EncodeSize(size)<<48, 构造位隐 0 (即构造中)。
  pub fn new(size: usize, gc_info_index: GcInfoIndex) -> Self {
    debug_assert!(gc_info_index < K_MAX_GC_INFO_INDEX);
    debug_assert_eq!(0, size & (size_of::<Self>() - 1));
    debug_assert!(K_MAX_SIZE >= size);
    let header = Self {
      next_unfinalized: 0,
      encoded_high: AtomicU16::new(gc_info_index & K_GC_INFO_INDEX_MASK),
      encoded_low: AtomicU16::new(Self::encode_size(size)),
    };
    debug_assert!(header.is_in_construction(AccessMode::NonAtomic));
    header
  }

  /// v8: heap-object-header.h:142-145 EncodeSize — SizeField::encode(size / kAllocationGranularity)
  /// (原版注释: 实际优化成 >> 1)
  const fn encode_size(size: usize) -> u16 {
    (((size / K_ALLOCATION_GRANULARITY) as u16) << 1) & K_SIZE_MASK
  }

  /// v8: heap-object-header.h:137-140 DecodeSize — SizeField::decode(encoded) * kAllocationGranularity
  /// (原版注释: 实际优化成 << 1 后再 *8)
  const fn decode_size(encoded: u16) -> usize {
    (encoded & K_SIZE_MASK) as usize / 2 * K_ALLOCATION_GRANULARITY
  }

  /// v8: heap-object-header.h:69-70 ObjectStart — 对象起于头之后
  pub fn object_start(&self) -> Address {
    (from_ref(self) as usize + size_of::<Self>()) as Address
  }

  /// v8: heap-object-header.h:71-72,227-232 ObjectEnd<mode>
  pub fn object_end(&self, mode: AccessMode) -> Address {
    debug_assert!(!self.is_large_object(mode));
    (from_ref(self) as usize + self.allocated_size(mode)) as Address
  }

  /// v8: heap-object-header.h:234-239 GetGCInfoIndex<mode> — 高半字 acquire 载
  pub fn get_gc_info_index(&self, mode: AccessMode) -> GcInfoIndex {
    self.load_encoded(mode, EncodedHalf::High, Ordering::Acquire) & K_GC_INFO_INDEX_MASK
  }

  /// v8: heap-object-header.h:241-249 AllocatedSize<mode> — 低半字 relaxed 载 (构造后尺寸不可变)
  pub fn allocated_size(&self, mode: AccessMode) -> usize {
    Self::decode_size(self.load_encoded(mode, EncodedHalf::Low, Ordering::Relaxed))
  }

  /// v8: heap-object-header.h:251-262 SetAllocatedSize — 保位写低半字 (不清 mark bit)
  pub fn set_allocated_size(&mut self, size: usize) {
    // v8: :252-257 CPPGC_YOUNG_GENERATION 关时 DCHECK(!IsMarked()) — 目标配置开分代, 不校验。
    let low = self.encoded_low.get_mut();
    *low &= !K_SIZE_MASK;
    *low |= Self::encode_size(size);
  }

  /// v8: heap-object-header.h:264-269 ObjectSize<mode> = AllocatedSize - sizeof(header)
  pub fn object_size(&self, mode: AccessMode) -> usize {
    let allocated = self.allocated_size(mode);
    debug_assert!(allocated > size_of::<Self>());
    allocated - size_of::<Self>()
  }

  /// v8: heap-object-header.h:271-274 IsLargeObject<mode> — 头内尺寸编码为 0
  pub fn is_large_object(&self, mode: AccessMode) -> bool {
    self.allocated_size(mode) == K_LARGE_OBJECT_SIZE_IN_HEADER as usize
  }

  /// v8: heap-object-header.h:276-281 IsInConstruction<mode> — acquire, fully_constructed 位为 0
  pub fn is_in_construction(&self, mode: AccessMode) -> bool {
    self.load_encoded(mode, EncodedHalf::High, Ordering::Acquire) & K_FULLY_CONSTRUCTED_MASK == 0
  }

  /// v8: heap-object-header.h:89 声明 + cc:51-54 → include/cppgc/allocation.h:55-68
  /// MarkObjectAsFullyConstructed — 单写者契约下 load(relaxed)+or+store(release), 非 CAS
  /// (原版注释: 该 16 位位域仅单线程改动, 拆分比 RMW 更省)。尺寸位与构造位同半字, 互不影响。
  pub fn mark_as_fully_constructed(&mut self) {
    let value = self.encoded_high.load(Ordering::Relaxed) | K_FULLY_CONSTRUCTED_MASK;
    self.encoded_high.store(value, Ordering::Release);
  }

  /// v8: heap-object-header.h:283-288 IsMarked<mode> — 低半字 bit0, relaxed
  pub fn is_marked(&self, mode: AccessMode) -> bool {
    self.load_encoded(mode, EncodedHalf::Low, Ordering::Relaxed) & K_MARK_BIT_MASK != 0
  }

  /// v8: heap-object-header.h:290-295 Unmark<mode> — StoreEncoded(kLow, relaxed, false, kMarkMask)
  pub fn unmark(&mut self, mode: AccessMode) {
    debug_assert!(self.is_marked(mode));
    self.store_encoded(
      mode,
      EncodedHalf::Low,
      Ordering::Relaxed,
      0,
      K_MARK_BIT_MASK,
    );
  }

  /// v8: heap-object-header.h:96,297-306 TryMarkAtomic — relaxed CAS 置 mark 位, 已标返回 false
  pub fn try_mark_atomic(&self) -> bool {
    let old_value = self.encoded_low.load(Ordering::Relaxed);
    let new_value = old_value | K_MARK_BIT_MASK;
    if new_value == old_value {
      return false;
    }
    // 原版 atomic_ref<uint16_t>::compare_exchange_strong (relaxed/relaxed); Rust CAS 仅需 &self,
    // 「同一半字内并发改动互斥」的内存纪律与原版一致 (h:377-381 caveat 注释)。
    self
      .encoded_low
      .compare_exchange(old_value, new_value, Ordering::Relaxed, Ordering::Relaxed)
      .is_ok()
  }

  /// v8: heap-object-header.h:98,308-311 MarkNonAtomic
  pub fn mark_non_atomic(&mut self) {
    debug_assert!(!self.is_marked(AccessMode::NonAtomic));
    *self.encoded_low.get_mut() |= K_MARK_BIT_MASK;
  }

  /// v8: heap-object-header.h:313-316 IsYoung<mode> — 粘性位语义: 未标即年轻代
  pub fn is_young(&self, mode: AccessMode) -> bool {
    !self.is_marked(mode)
  }

  /// v8: heap-object-header.h:318-321 IsFree<mode> — index == kFreeListGCInfoIndex
  pub fn is_free(&self, mode: AccessMode) -> bool {
    self.get_gc_info_index(mode) == K_FREE_LIST_GC_INFO_INDEX
  }

  /// v8: heap-object-header.h:329-335 SetNextUnfinalized — CPPGC_CAGED_HEAP 下存 cage 内 u32 偏移。
  /// 原版入参是 `HeapObjectHeader*`, 经 `CagedHeap::OffsetFromAddress<uint32_t>` 换算;
  /// 指针↔偏移换算随 caged_heap 批次挂载, 本件先落偏移存取对 (见件首台账)。
  pub fn set_next_unfinalized(&mut self, cage_offset: u32) {
    self.next_unfinalized = cage_offset;
  }

  /// v8: heap-object-header.h:337-353 GetNextUnfinalized — 偏移 0 = 链尾 (原版返回 nullptr)
  pub const fn next_unfinalized(&self) -> Option<u32> {
    if self.next_unfinalized == 0 {
      None
    } else {
      Some(self.next_unfinalized)
    }
  }

  /// v8: heap-object-header.h:323-326 IsFinalizable — GCInfo.finalize 非空
  pub fn is_finalizable(&self) -> bool {
    GlobalGcInfoTable::gc_info_from_index(self.get_gc_info_index(AccessMode::NonAtomic))
      .finalize
      .is_some()
  }

  /// v8: src/heap/cppgc-internal/heap-object-header.cc:27-39 Finalize — 调 GCInfo.finalize(payload)
  /// (ASAN_UNPOISON 臂属 sanitizer 裁剪面, 见件首台账)
  ///
  /// # Safety
  /// payload 上的 finalize 回调须以本对象 payload 指针为参可调用 (原版 GCInfo 注册契约)。
  pub fn finalize(&self) {
    let gc_info =
      GlobalGcInfoTable::gc_info_from_index(self.get_gc_info_index(AccessMode::NonAtomic));
    if let Some(finalize) = gc_info.finalize {
      // SAFETY: 回调由注册方 (EnsureGCInfoIndexTrait) 保证与对象类型匹配。
      unsafe { finalize(self.object_start().cast::<c_void>()) };
    }
  }

  /// v8: src/heap/cppgc-internal/heap-object-header.cc:45-49 GetName(HeapObjectNameForUnnamedObject)
  pub fn get_name(&self, mode: HeapObjectNameForUnnamedObject) -> HeapObjectName {
    let gc_info =
      GlobalGcInfoTable::gc_info_from_index(self.get_gc_info_index(AccessMode::NonAtomic));
    (gc_info.name)(self.object_start().cast::<c_void>(), mode)
  }

  /// v8: heap-object-header.h:356-361 TraceImpl<mode> — GCInfo.trace(visitor, ObjectStart())
  pub fn trace_impl(&self, mode: AccessMode, visitor: &mut Visitor) {
    let gc_info = GlobalGcInfoTable::gc_info_from_index(self.get_gc_info_index(mode));
    // SAFETY: 回调按原版调用约定接裸 payload 指针, 注册契约保证对象类型与回调匹配。
    unsafe { (gc_info.trace)(visitor, self.object_start().cast_const().cast::<c_void>()) };
  }

  /// v8: heap-object-header.h:363-372 LoadEncoded<mode, part, memory_order>
  fn load_encoded(&self, mode: AccessMode, half: EncodedHalf, memory_order: Ordering) -> u16 {
    let atomic = match half {
      EncodedHalf::Low => &self.encoded_low,
      EncodedHalf::High => &self.encoded_high,
    };
    // kNonAtomic 原版为纯读; Relaxed 原子读在 x64/arm64 生成同一纯载入指令, 语义等价。
    match mode {
      AccessMode::NonAtomic => atomic.load(Ordering::Relaxed),
      AccessMode::Atomic => atomic.load(memory_order),
    }
  }

  /// v8: heap-object-header.h:374-394 StoreEncoded<mode, part, memory_order>
  fn store_encoded(
    &mut self,
    mode: AccessMode,
    half: EncodedHalf,
    memory_order: Ordering,
    bits: u16,
    mask: u16,
  ) {
    debug_assert_eq!(0, bits & !mask);
    let atomic = match half {
      EncodedHalf::Low => &mut self.encoded_low,
      EncodedHalf::High => &mut self.encoded_high,
    };
    if mode == AccessMode::NonAtomic {
      let value = atomic.get_mut();
      *value = (*value & !mask) | bits;
      return;
    }
    // 原版不做 CAS: 同一半字内不会有两处并发改动 (h:387-389 注释)。
    let mut value = atomic.load(Ordering::Relaxed);
    value = (value & !mask) | bits;
    atomic.store(value, memory_order);
  }
}
