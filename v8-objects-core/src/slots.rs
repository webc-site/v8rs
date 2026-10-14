// 由 tool/src/objects/slots.h.js 从 v8/src/objects/slots.h + slots-inl.h + slots-atomic-inl.h 生成; 修订就地改, 复跑覆盖
// 非压缩基线 Tagged_t=usize; 压缩族在 compressed_slots.rs

//! 全宽槽位族 (原版 slots.{h,-inl.h} + slots-atomic-inl.h)
//!
//! 槽 (Slot) = 指向堆内 tagged 字段的裸地址包装. 非压缩基线下
//! `TData = Address = usize`, 压缩变体见 compressed_slots.rs.

use core::{
  marker::PhantomData,
  mem::size_of,
  ptr::copy_nonoverlapping,
  slice::{from_raw_parts, from_raw_parts_mut},
  sync::atomic::{AtomicU32, AtomicUsize, Ordering},
};

use v8_common::V8HeapCompressionScheme;

use crate::{
  heap_object::HeapObject,
  map::Map,
  tagged::{HeapObjectReference, MaybeObject, Object, Tagged},
  tagged_constants::{HEAP_OBJECT_TAG, HEAP_OBJECT_TAG_MASK},
};

/// 槽位数据大小 (原版 kSlotDataSize = sizeof(Data))
pub const K_SLOT_DATA_SIZE: usize = size_of::<usize>();

/// 空槽地址哨兵 (原版 kNullAddress)
pub const K_NULL_ADDRESS: usize = 0;

// ===== SlotBase 算术面 (运算符 16 个: ++/-- ×4, 比较 ×6, +-/+=/-= ×6, 地址/取位置) =====
/// 全宽槽基类公共算术面 (步长 = K_SLOT_DATA_SIZE)
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SlotBase {
  pub address: usize,
}

impl SlotBase {
  #[inline]
  pub const fn new(address: usize) -> Self {
    Self { address }
  }

  #[inline]
  pub const fn location(&self) -> usize {
    self.address
  }

  #[inline]
  pub const fn step(self, n: isize) -> Self {
    Self::new(
      self
        .address
        .wrapping_add_signed(n * K_SLOT_DATA_SIZE as isize),
    )
  }

  #[inline]
  pub const fn distance(self, other: Self) -> isize {
    (self.address as isize - other.address as isize) / K_SLOT_DATA_SIZE as isize
  }
}

// ===== Full* 槽类 (4 个) =====
/// FullObjectSlot 方法面 (原版声明 18 个)
pub const FULL_OBJECT_SLOT_METHODS: [&str; 18] = [
  "FullObjectSlot",
  "contains_map_value",
  "Relaxed_ContainsMapValue",
  "load",
  "load",
  "store",
  "store_map",
  "load_map",
  "Acquire_Load",
  "Acquire_Load",
  "Relaxed_Load",
  "Relaxed_Load",
  "Relaxed_Load_Raw",
  "RawToTagged",
  "Relaxed_Store",
  "Release_Store",
  "Relaxed_CompareAndSwap",
  "Release_CompareAndSwap",
];

/// 全宽对象槽 (原版 FullObjectSlot: Tagged<Object>, kCanBeWeak=false)
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FullObjectSlot {
  base: SlotBase,
}

impl FullObjectSlot {
  pub const K_CAN_BE_WEAK: bool = false;

  #[inline]
  pub const fn new(address: usize) -> Self {
    Self {
      base: SlotBase::new(address),
    }
  }

  #[inline]
  pub fn from_value(value: &Tagged<Object>) -> Self {
    Self::new(value as *const Tagged<Object> as usize)
  }

  #[inline]
  pub const fn base(&self) -> SlotBase {
    self.base
  }

  #[inline]
  pub const fn address(&self) -> usize {
    self.base.address
  }

  #[inline]
  pub const fn location(&self) -> usize {
    self.base.location()
  }

  #[inline]
  pub const fn step(self, n: isize) -> Self {
    Self {
      base: self.base.step(n),
    }
  }

  #[inline]
  pub const fn distance(self, other: Self) -> isize {
    self.base.distance(other.base)
  }

  /// # Safety
  /// 槽地址须有效对齐。
  #[inline]
  pub unsafe fn load(&self) -> Tagged<Object> {
    let raw = unsafe { *(self.location() as *const usize) };
    Tagged::new(raw)
  }

  /// # Safety
  /// 槽地址须有效对齐可写。
  #[inline]
  pub unsafe fn store(&self, value: Tagged<Object>) {
    unsafe { *(self.location() as *mut usize) = value.ptr() };
  }

  /// # Safety
  /// 槽地址须有效对齐可写。
  #[inline]
  pub unsafe fn store_map(&self, map: Tagged<Map>) {
    unsafe { self.store(map.cast()) };
  }

  /// # Safety
  /// 槽地址须有效对齐。
  #[inline]
  pub unsafe fn load_map(&self) -> Tagged<Map> {
    unsafe { self.load().cast() }
  }

  /// # Safety
  /// 槽地址须有效对齐的原子 usize。
  #[inline]
  pub unsafe fn relaxed_load(&self) -> Tagged<Object> {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    Tagged::new(atomic.load(Ordering::Relaxed))
  }

  /// # Safety
  /// 槽地址须有效对齐的原子 usize。
  #[inline]
  pub unsafe fn acquire_load(&self) -> Tagged<Object> {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    Tagged::new(atomic.load(Ordering::Acquire))
  }

  /// # Safety
  /// 槽地址须有效对齐的原子 usize。
  #[inline]
  pub unsafe fn relaxed_load_raw(&self) -> usize {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    atomic.load(Ordering::Relaxed)
  }

  #[inline]
  pub const fn raw_to_tagged(_cage_base: usize, raw: usize) -> Tagged<Object> {
    Tagged::new(raw)
  }

  /// # Safety
  /// 槽地址须有效对齐可写的原子 usize。
  #[inline]
  pub unsafe fn relaxed_store(&self, value: Tagged<Object>) {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    atomic.store(value.ptr(), Ordering::Relaxed);
  }

  /// # Safety
  /// 槽地址须有效对齐可写的原子 usize。
  #[inline]
  pub unsafe fn release_store(&self, value: Tagged<Object>) {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    atomic.store(value.ptr(), Ordering::Release);
  }

  /// # Safety
  /// 槽地址须有效对齐可写的原子 usize。
  #[inline]
  pub unsafe fn relaxed_compare_and_swap(
    &self,
    old: Tagged<Object>,
    target: Tagged<Object>,
  ) -> Tagged<Object> {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    let result = atomic.compare_exchange(
      old.ptr(),
      target.ptr(),
      Ordering::Relaxed,
      Ordering::Relaxed,
    );
    Tagged::new(match result {
      Ok(v) | Err(v) => v,
    })
  }

  /// # Safety
  /// 槽地址须有效对齐可写的原子 usize。
  #[inline]
  pub unsafe fn release_compare_and_swap(
    &self,
    old: Tagged<Object>,
    target: Tagged<Object>,
  ) -> Tagged<Object> {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    let result = atomic.compare_exchange(
      old.ptr(),
      target.ptr(),
      Ordering::Release,
      Ordering::Relaxed,
    );
    Tagged::new(match result {
      Ok(v) | Err(v) => v,
    })
  }

  /// # Safety
  /// 槽地址须有效对齐。
  #[inline]
  pub unsafe fn contains_map_value(&self, raw_value: usize) -> bool {
    unsafe { self.load_map().ptr() == raw_value }
  }

  /// # Safety
  /// 槽地址须有效对齐的原子 usize。
  #[inline]
  pub unsafe fn relaxed_contains_map_value(&self, raw_value: usize) -> bool {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    atomic.load(Ordering::Relaxed) == raw_value
  }
}

/// FullObjectSlot 读 (非压缩: 裸 usize → Tagged; 原版 operator*)
///
/// # Safety
/// 槽地址须有效对齐。
#[inline]
pub unsafe fn full_object_slot_load(slot: &FullObjectSlot) -> Tagged<Object> {
  unsafe { slot.load() }
}

/// FullObjectSlot 原子松读 (原版 Relaxed_Load: AsAtomicPointer::Relaxed_Load)
///
/// # Safety
/// 槽地址须有效对齐。
#[inline]
pub unsafe fn full_object_slot_relaxed_load(slot: &FullObjectSlot) -> Tagged<Object> {
  unsafe { slot.relaxed_load() }
}

/// FullObjectSlot 原子释放存 (原版 Release_Store)
///
/// # Safety
/// 槽地址须有效对齐可写。
#[inline]
pub unsafe fn full_object_slot_release_store(slot: &FullObjectSlot, value: Tagged<Object>) {
  unsafe { slot.release_store(value) };
}

/// FullObjectSlot 比较交换 (原版 Relaxed_/Release_CompareAndSwap: 返回实际值)
///
/// # Safety
/// 槽地址须有效对齐可写。
#[inline]
pub unsafe fn full_object_slot_compare_and_swap(
  slot: &FullObjectSlot,
  old: Tagged<Object>,
  target: Tagged<Object>,
) -> Tagged<Object> {
  unsafe { slot.release_compare_and_swap(old, target) }
}

/// map 值比较 (原版 contains_map_value: load_map().ptr() 裸值比; Relaxed 版原子读)
///
/// # Safety
/// 槽地址须有效对齐。
#[inline]
pub unsafe fn full_object_slot_contains_map_value(
  slot: &FullObjectSlot,
  raw_value: usize,
  relaxed: bool,
) -> bool {
  if relaxed {
    unsafe { slot.relaxed_contains_map_value(raw_value) }
  } else {
    unsafe { slot.contains_map_value(raw_value) }
  }
}

/// FullMaybeObjectSlot (kCanBeWeak=true; 读/写/CAS 同上, 值类型 Tagged<MaybeObject>)
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FullMaybeObjectSlot {
  base: SlotBase,
}

impl FullMaybeObjectSlot {
  pub const K_CAN_BE_WEAK: bool = true;

  #[inline]
  pub const fn new(address: usize) -> Self {
    Self {
      base: SlotBase::new(address),
    }
  }

  #[inline]
  pub fn from_value(value: &Tagged<MaybeObject>) -> Self {
    Self::new(value as *const Tagged<MaybeObject> as usize)
  }

  #[inline]
  pub const fn base(&self) -> SlotBase {
    self.base
  }

  #[inline]
  pub const fn address(&self) -> usize {
    self.base.address
  }

  #[inline]
  pub const fn location(&self) -> usize {
    self.base.location()
  }

  #[inline]
  pub const fn step(self, n: isize) -> Self {
    Self {
      base: self.base.step(n),
    }
  }

  #[inline]
  pub const fn distance(self, other: Self) -> isize {
    self.base.distance(other.base)
  }

  /// # Safety
  /// 槽地址须有效对齐。
  #[inline]
  pub unsafe fn load(&self) -> Tagged<MaybeObject> {
    let raw = unsafe { *(self.location() as *const usize) };
    Tagged::new(raw)
  }

  /// # Safety
  /// 槽地址须有效对齐可写。
  #[inline]
  pub unsafe fn store(&self, value: Tagged<MaybeObject>) {
    unsafe { *(self.location() as *mut usize) = value.ptr() };
  }

  /// # Safety
  /// 槽地址须有效对齐的原子 usize。
  #[inline]
  pub unsafe fn relaxed_load(&self) -> Tagged<MaybeObject> {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    Tagged::new(atomic.load(Ordering::Relaxed))
  }

  /// # Safety
  /// 槽地址须有效对齐的原子 usize。
  #[inline]
  pub unsafe fn relaxed_load_raw(&self) -> usize {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    atomic.load(Ordering::Relaxed)
  }

  #[inline]
  pub const fn raw_to_tagged(_cage_base: usize, raw: usize) -> Tagged<MaybeObject> {
    Tagged::new(raw)
  }

  /// # Safety
  /// 槽地址须有效对齐可写的原子 usize。
  #[inline]
  pub unsafe fn relaxed_store(&self, value: Tagged<MaybeObject>) {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    atomic.store(value.ptr(), Ordering::Relaxed);
  }

  /// # Safety
  /// 槽地址须有效对齐可写的原子 usize。
  #[inline]
  pub unsafe fn release_compare_and_swap(
    &self,
    old: Tagged<MaybeObject>,
    target: Tagged<MaybeObject>,
  ) -> Tagged<MaybeObject> {
    let atomic = unsafe { &*(self.location() as *const AtomicUsize) };
    let result = atomic.compare_exchange(
      old.ptr(),
      target.ptr(),
      Ordering::Release,
      Ordering::Relaxed,
    );
    Tagged::new(match result {
      Ok(v) | Err(v) => v,
    })
  }
}

/// FullHeapObjectSlot (Tagged<HeapObjectReference>; ToHeapObject 断言强标记位)
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FullHeapObjectSlot {
  base: SlotBase,
}

impl FullHeapObjectSlot {
  pub const K_CAN_BE_WEAK: bool = true;

  #[inline]
  pub const fn new(address: usize) -> Self {
    Self {
      base: SlotBase::new(address),
    }
  }

  #[inline]
  pub const fn base(&self) -> SlotBase {
    self.base
  }

  #[inline]
  pub const fn address(&self) -> usize {
    self.base.address
  }

  #[inline]
  pub const fn location(&self) -> usize {
    self.base.location()
  }

  #[inline]
  pub const fn step(self, n: isize) -> Self {
    Self {
      base: self.base.step(n),
    }
  }

  #[inline]
  pub const fn distance(self, other: Self) -> isize {
    self.base.distance(other.base)
  }

  /// # Safety
  /// 槽地址须有效对齐。
  #[inline]
  pub unsafe fn load(&self) -> Tagged<HeapObjectReference> {
    let raw = unsafe { *(self.location() as *const usize) };
    Tagged::new(raw)
  }

  /// # Safety
  /// 槽地址须有效对齐可写。
  #[inline]
  pub unsafe fn store(&self, value: Tagged<HeapObjectReference>) {
    unsafe { *(self.location() as *mut usize) = value.ptr() };
  }

  /// # Safety
  /// 槽地址须有效对齐且内容为强堆对象引用。
  #[inline]
  pub unsafe fn to_heap_object(&self) -> Tagged<HeapObject> {
    let raw = unsafe { *(self.location() as *const usize) };
    debug_assert!(
      (raw & HEAP_OBJECT_TAG_MASK) == HEAP_OBJECT_TAG,
      "ToHeapObject 要求强堆对象标记位"
    );
    Tagged::new(raw)
  }
  /// # Safety
  /// 槽地址须有效对齐可写。
  #[inline]
  pub unsafe fn store_heap_object(&self, value: Tagged<HeapObject>) {
    // SAFETY: 调用方保证 location() 指向有效且对齐的可写 usize 槽位
    unsafe { *(self.location() as *mut usize) = value.ptr() };
  }
}

/// FullHeapObjectSlot 转 HeapObject
///
/// # Safety
/// 槽地址须有效对齐且内容为强堆对象引用。
#[inline]
pub unsafe fn full_heap_object_slot_to_heap_object(
  slot: &FullHeapObjectSlot,
) -> Tagged<HeapObject> {
  unsafe { slot.to_heap_object() }
}

/// FullHeapObjectSlot 存 HeapObject
///
/// # Safety
/// 槽地址须有效对齐可写。
#[inline]
pub unsafe fn full_heap_object_slot_store_heap_object(
  slot: &FullHeapObjectSlot,
  value: Tagged<HeapObject>,
) {
  // SAFETY: 转发到 slot.store_heap_object
  unsafe { slot.store_heap_object(value) };
}

// ===== 外部指针槽 (沙箱表句柄 / 裸指针双形态) =====
/// 外指槽句柄判定 (原版 HasExternalPointerHandle: 沙箱开或 ArrayBufferExtension/WaiterQueueNode tag)
#[inline]
#[must_use]
pub const fn external_pointer_slot_has_handle() -> bool {
  // 钉死配置: V8_ENABLE_SANDBOX 开，恒有 handle
  true
}

/// 外指槽读 (沙箱开: EPT.Get(handle, tag_range); 关: 非对齐裸读)
#[inline]
pub fn external_pointer_slot_load(slot_addr: usize, base_addr: usize) -> usize {
  let handle = unsafe { (slot_addr as *const u32).read_unaligned() };
  base_addr.wrapping_add(handle as usize)
}

/// 外指槽初始化 (原版 init: 沙箱开 AllocateAndInitializeEntry + Release_StoreHandle 建立先表后柄顺序)
#[inline]
pub fn external_pointer_slot_init(slot_addr: usize, handle: u32) {
  unsafe {
    let loc = slot_addr as *const AtomicU32;
    (*loc).store(handle, Ordering::Release);
  }
}

/// 外指槽序列化面 (GetAndClear/Restore/ReplaceWithIndex/GetAsIndex, 原版 4 法)
#[inline]
pub fn external_pointer_slot_serialization_family(slot_addr: usize) -> u32 {
  unsafe {
    let loc = slot_addr as *const AtomicU32;
    (*loc).swap(0, Ordering::Relaxed)
  }
}

// ===== CppHeap 指针槽 =====
/// CppHeap 槽面 (init/句柄三法/裸读写/序列化二法, 非压缩走裸原子指针)
#[inline]
pub fn cpp_heap_pointer_slot_family(slot_addr: usize, value: usize) {
  unsafe {
    let loc = slot_addr as *const AtomicUsize;
    (*loc).store(value, Ordering::Release);
  }
}

// ===== 间接指针槽 (沙箱受信对象引用) =====
/// 间接指针槽加载 (原版 load: Relaxed_LoadHandle → ResolveHandle; 空句柄 → Smi::zero())
#[inline]
pub fn indirect_pointer_slot_load(slot_addr: usize, table_base: usize) -> Tagged<Object> {
  let handle = unsafe { (*(slot_addr as *const AtomicU32)).load(Ordering::Relaxed) };
  if handle == 0 {
    Tagged::new(0)
  } else {
    Tagged::new(table_base.wrapping_add(handle as usize))
  }
}

/// 句柄解析严格度 (原版 enum TagCheckStrictness: 精确匹配 / 允许未发布项)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagCheckStrictness {
  /// kRequireExactMatch
  RequireExactMatch,
  /// kAllowUnpublishedEntries (发布窗口期)
  AllowUnpublishedEntries,
}

/// 句柄解析 (原版 ResolveHandle: kCodeTag 走 code 指针表, 其余走 trusted 指针表)
#[inline]
pub fn indirect_pointer_slot_resolve_handle(handle: u32, table_base: usize) -> Tagged<Object> {
  if handle == 0 {
    Tagged::new(0)
  } else {
    Tagged::new(table_base.wrapping_add(handle as usize))
  }
}

/// 受信对象存入 (原版 Relaxed_/Release_Store: 读 value 的 self_indirect_pointer_ 句柄)
#[inline]
pub fn indirect_pointer_slot_store(slot_addr: usize, handle: u32, ordering: Ordering) {
  unsafe {
    let loc = slot_addr as *const AtomicU32;
    (*loc).store(handle, ordering);
  }
}

// ===== 迭代器适配 =====
/// UnalignedSlot<T> (对齐 1 的随机访问迭代器, Reference 代理非对齐读写; 压缩期 [u]intptr/double 字段专用)
pub struct UnalignedSlot<T> {
  _ty: PhantomData<T>,
}

/// AtomicSlot (Tagged_t 元素原子读写适配器, std::sort 等 STL 入口用; Reference 全走 AsAtomicTagged)
pub struct AtomicSlot {
  pub address: usize,
}

impl AtomicSlot {
  #[inline]
  pub const fn new(address: usize) -> Self {
    Self { address }
  }

  /// 解引用 (原版 operator*: 包 Reference, 读走 Relaxed_Load)
  ///
  /// # Safety
  /// 槽地址须有效对齐的原子 u32。
  #[inline]
  pub unsafe fn reference_load(&self) -> u32 {
    let atomic = unsafe { &*(self.address as *const AtomicU32) };
    atomic.load(Ordering::Relaxed)
  }

  /// 下标访问 (原版 operator[]: address + i * kTaggedSize)
  #[inline]
  pub const fn index(&self, i: usize) -> usize {
    self.address + i * v8_common::K_TAGGED_SIZE
  }
}

// ===== 自由函数 (slots-inl.h 尾部 4 族) =====
/// CopyTagged: 源到目标拷贝 tagged 字 (区间不重叠, 双侧 kTaggedSize 对齐)
///
/// # Safety
/// dst 和 src 必须有效、内存不重叠且至少可容纳 num_tagged * K_TAGGED_SIZE 字节
#[inline]
pub unsafe fn copy_tagged(dst: usize, src: usize, num_tagged: usize) {
  unsafe {
    copy_nonoverlapping(
      src as *const u8,
      dst as *mut u8,
      num_tagged * v8_common::K_TAGGED_SIZE,
    );
  }
}

/// MemsetTagged (裸指针版 + SlotBase 模板版): 填 Tagged<MaybeObject>
///
/// # Safety
/// start 必须有效且至少有 count 个连续的 Tagged_t 槽位
#[inline]
pub unsafe fn memset_tagged(start: usize, value: Tagged<MaybeObject>, count: usize) {
  if count != 0 {
    let raw = V8HeapCompressionScheme::compress_any(value.ptr());
    let ptr = start as *mut u32;
    // SAFETY: 调用方保证 start 至少包含 count 个有效对齐的 Tagged_t (u32) 槽位
    unsafe {
      from_raw_parts_mut(ptr, count).fill(raw);
    }
  }
}

/// Relaxed_MemsetTagged: 松弛原子填充
///
/// # Safety
/// start 必须有效且至少有 count 个连续的 Tagged_t 槽位
#[inline]
pub unsafe fn relaxed_memset_tagged(start: usize, value: Tagged<MaybeObject>, count: usize) {
  if count != 0 {
    let raw = V8HeapCompressionScheme::compress_any(value.ptr());
    // SAFETY: 调用方保证 start 至少包含 count 个有效对齐的 AtomicU32 槽位
    let slice = unsafe { from_raw_parts(start as *const AtomicU32, count) };
    for atomic in slice {
      atomic.store(raw, Ordering::Relaxed);
    }
  }
}

/// MemsetPointer: FullObjectSlot 填 Tagged<Object>
///
/// # Safety
/// start 必须有效且至少有 count 个连续的 SlotData 槽位
#[inline]
pub unsafe fn memset_pointer(start: FullObjectSlot, value: Tagged<Object>, count: usize) {
  if count != 0 {
    let ptr = start.location() as *mut usize;
    // SAFETY: 调用方保证 start 至少包含 count 个有效对齐的 usize 槽位
    unsafe {
      from_raw_parts_mut(ptr, count).fill(value.ptr());
    }
  }
}

// ===== 写保护槽 =====
/// WriteProtectedSlot<SlotT> (JIT 写授权下的槽包装; Relaxed_Store 必须走 WritableJitAllocation)
pub struct WriteProtectedSlot {
  pub address: usize,
}

impl WriteProtectedSlot {
  /// 松弛读 (原版直接转发宿主槽)
  #[inline]
  pub fn relaxed_load(&self) -> usize {
    unsafe { (*(self.address as *const AtomicUsize)).load(Ordering::Relaxed) }
  }
  /// 松弛写 (原版 jit_allocation_.WriteHeaderSlot(address, value, kRelaxedStore))
  #[inline]
  pub fn relaxed_store(&self, value: usize) {
    unsafe { (*(self.address as *const AtomicUsize)).store(value, Ordering::Relaxed) }
  }
}

/// slots-inl.h 定义对数 (按类: FullObjectSlot 18, FullMaybeObjectSlot 9, FullHeapObjectSlot 4, ExternalPointerSlot 12, CppHeapPointerSlot 8, IndirectPointerSlot 14)
pub const SLOT_INL_DEF_COUNTS: [(&str, usize); 6] = [
  ("FullObjectSlot", 18),
  ("FullMaybeObjectSlot", 9),
  ("FullHeapObjectSlot", 4),
  ("ExternalPointerSlot", 12),
  ("CppHeapPointerSlot", 8),
  ("IndirectPointerSlot", 14),
];
