// 由 tool/src/objects/fixed-array.h.js 从 v8/src/objects/fixed-array.h + fixed-array-inl.h + fixed-array.tq 生成; 修订就地改, 复跑覆盖
// TaggedArrayBase → trait + blanket impl; 9 tagged 族; primitive 族移交 primitive_array.rs; isolate/factory 依赖推迟 (脚本头登记)

//! FixedArray 族: 定长 tagged 元素数组 (FixedArray / WeakFixedArray /
//! trusted/protected 变体 / ArrayList / WeakArrayList)。
//!
//! 建模 (glossary: CRTP → trait + 泛型; 继承 → 包装 + trait):
//! - [TaggedArray]: 布局差异 (length/capacity 槽位, 数据区起点) → 关联常量;
//! - [TaggedArrayExt]: 公共读写语义 (get/set 4 序 + swap/cas + 容量/尺寸)。
//!
//! 移交登记 (脚本头): PrimitiveArrayBase 族 (ByteArray/TrustedByteArray/
//! FixedDoubleArray/FixedIntegerArray) 归 primitive_array.rs。
//!
//! 布局双轨制 (同 fixed_array_base 模块头): snapshot 偏移常量按压缩默认
//! (K_TAGGED_SIZE=4); 运行时槽位按结构体逻辑布局 (Tagged 槽 usize 宽),
//! offset_of / size_of 承载, Tagged 槽压缩收窄后两者合一。
//!
//! 跳过登记 (脚本头): New/Allocate 族, MoveElements/CopyElements (写屏障簿记),
//! set 内 DCHECK(!IsCowArray()), clear_optional_padding, BodyDescriptor/verifier,
//! roots 表 (kMapRootIndex)。

use std::{
  mem, ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_common::{K_MAX_REGULAR_HEAP_OBJECT_SIZE, K_TAGGED_SIZE};
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type_checker as itc,
  map::Map,
  objects::{Cast, WriteBarrierMode, instance_type_of},
  smi::Smi,
  tagged::{MaybeObject, Object, Tagged},
  tagged_field::TaggedSlot,
};

use crate::{
  error::{Error, Result},
  fixed_array_base::{
    FixedArrayBase, K_MAX_FIXED_ARRAY_CAPACITY, TrustedFixedArrayBase, u32_slot, write_barrier_stub,
  },
};

// ── 受保护元素标记类型 (fixed-array.h UnionOf 展开) ────────────────────────
/// UnionOf<TrustedObject, Smi>: ProtectedFixedArray 元素 (受保护指针或 Smi)。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TrustedObjectOrSmi;

/// UnionOf<MaybeWeak<TrustedObject>, Smi>: ProtectedWeakFixedArray 元素。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MaybeWeakTrustedObjectOrSmi;

// ── TaggedArrayBase → trait (fixed-array.h:57-207) ────────────────────────
/// tagged 元素数组布局契约 (TaggedArrayBase&lt;Derived, ElementT, Super&gt;)。
///
/// 各具体族声明槽位/数据区偏移; 公共语义由 TaggedArrayExt blanket 承载。
pub trait TaggedArray: Sized {
  /// 元素标记类型 (ElementT)。
  type Element;

  /// length_ 槽运行时偏移 (Super 承载, 或 capacity+length 族自声明)。
  const LENGTH_OFFSET: usize;

  /// capacity_field() 槽运行时偏移 (HasCapacity → capacity_, 否则同 length)。
  const CAPACITY_OFFSET: usize;

  /// 数据区 (objects 尾) 运行时起始偏移 (= OFFSET_OF_DATA_START(Derived))。
  const OBJECTS_OFFSET: usize;

  /// 单元素字节数 (kElementSize = kTaggedSize; 运行时槽 usize 宽)。
  const ELEMENT_SIZE: usize = mem::size_of::<TaggedSlot>();
}

/// TaggedArrayExt::iter 的元素迭代器 ([0, capacity) 逐槽 get)。
pub struct TaggedArrayIter<'a, A: TaggedArrayExt> {
  arr: &'a A,
  i: u32,
  n: u32,
}

impl<A: TaggedArrayExt> Iterator for TaggedArrayIter<'_, A> {
  type Item = Tagged<A::Element>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    if self.i < self.n {
      let v = unsafe { self.arr.get(self.i) };
      self.i += 1;
      Some(v)
    } else {
      None
    }
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let rem = (self.n - self.i) as usize;
    (rem, Some(rem))
  }
}

impl<A: TaggedArrayExt> ExactSizeIterator for TaggedArrayIter<'_, A> {}

/// Tagged&lt;T: TaggedArray&gt; 的公共读写语义 (fixed-array-inl.h:37-226)。
///
/// 元素槽读写均为原子 (C++ 普通读写 → Relaxed, 同 core TaggedMember 先例);
/// 写屏障为桩 (L3 heap 接入, 见 fixed_array_base::write_barrier_stub)。
pub trait TaggedArrayExt: Copy {
  /// 元素标记类型透传。
  type Element;

  /// capacity() (fixed-array-inl.h:104)。
  ///
  /// # Safety
  /// 指针须指向有效数组对象内存。
  #[inline]
  unsafe fn capacity(&self) -> u32 {
    // SAFETY: 调用方契约; 槽位恒在对象内
    unsafe { u32_slot::load(self.capacity_addr()) }
  }

  /// capacity(kRelaxedLoad)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn capacity_relaxed(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.capacity_addr(), Ordering::Relaxed) }
  }

  /// capacity(kAcquireLoad)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn capacity_acquire(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.capacity_addr(), Ordering::Acquire) }
  }

  /// set_capacity(value)。
  ///
  /// # Safety
  /// 同 capacity; 无并发别名写。
  #[inline]
  unsafe fn set_capacity(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store(self.capacity_addr(), value) }
  }

  /// set_capacity(value, kReleaseStore)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn set_capacity_release(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store_order(self.capacity_addr(), value, Ordering::Release) }
  }

  /// length() (继承自 (Trusted)FixedArrayBase / 自声明)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn length(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load(self.length_addr()) }
  }

  /// ulength()。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn ulength(&self) -> u32 {
    // SAFETY: 同 length 契约
    unsafe { self.length() }
  }

  /// length(kRelaxedLoad)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn length_relaxed(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.length_addr(), Ordering::Relaxed) }
  }

  /// length(kAcquireLoad)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn length_acquire(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.length_addr(), Ordering::Acquire) }
  }

  /// set_length(value)。
  ///
  /// # Safety
  /// 同 capacity; 无并发别名写。
  #[inline]
  unsafe fn set_length(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store(self.length_addr(), value) }
  }

  /// set_length(value, kReleaseStore)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn set_length_release(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store_order(self.length_addr(), value, Ordering::Release) }
  }

  /// get(index) (无 tag 重载 = Relaxed 读; 原版 TODO jgruber 同源)。
  ///
  /// # Safety
  /// 指针有效且 index &lt; capacity() (debug 断言拦截)。
  #[inline]
  unsafe fn get(&self, index: u32) -> Tagged<Self::Element> {
    // SAFETY: 调用方契约; 越界 debug 拦截
    unsafe { self.get_relaxed(index) }
  }

  /// get(index, kRelaxedLoad)。
  ///
  /// # Safety
  /// 同 get。
  #[inline]
  unsafe fn get_relaxed(&self, index: u32) -> Tagged<Self::Element> {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约; 槽位原子读
    let v = unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.load(Ordering::Relaxed);
    Tagged::new(v)
  }

  /// get(index, kAcquireLoad)。
  ///
  /// # Safety
  /// 同 get。
  #[inline]
  unsafe fn get_acquire(&self, index: u32) -> Tagged<Self::Element> {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    let v = unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.load(Ordering::Acquire);
    Tagged::new(v)
  }

  /// get(index, kSeqCstAccess)。
  ///
  /// # Safety
  /// 同 get。
  #[inline]
  unsafe fn get_seqcst(&self, index: u32) -> Tagged<Self::Element> {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    let v = unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.load(Ordering::SeqCst);
    Tagged::new(v)
  }

  /// set(index, value, mode) (无 tag 重载 = Relaxed 写;
  /// 原版 DCHECK(!IsCowArray()) 依赖 roots → 调用方契约, 脚本头登记)。
  ///
  /// # Safety
  /// 指针可写有效且 index &lt; capacity(); 非 cow 数组。
  #[inline]
  unsafe fn set(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe { self.set_relaxed(index, value, mode) };
  }

  /// set(index, value, kRelaxedStore, mode)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_relaxed(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约; 槽位原子写
    unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }
      .store(value.ptr(), Ordering::Relaxed);
    write_barrier_stub(mode);
  }

  /// set(index, value, kReleaseStore, mode)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_release(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }
      .store(value.ptr(), Ordering::Release);
    write_barrier_stub(mode);
  }

  /// set(index, value, kSeqCstAccess, mode)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_seqcst(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.store(value.ptr(), Ordering::SeqCst);
    write_barrier_stub(mode);
  }

  /// set(index, Smi) (SKIP_WRITE_BARRIER; kSupportsSmiElements 展开)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_smi(&self, index: u32, value: Tagged<Smi>) {
    // SAFETY: 调用方契约; Smi 免屏障
    unsafe { self.set_relaxed(index, value.cast(), WriteBarrierMode::SkipWriteBarrier) };
  }

  /// set(index, Smi, kRelaxedStore)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_smi_relaxed(&self, index: u32, value: Tagged<Smi>) {
    // SAFETY: 调用方契约
    unsafe { self.set_relaxed(index, value.cast(), WriteBarrierMode::SkipWriteBarrier) };
  }

  /// set(index, Smi, kReleaseStore)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_smi_release(&self, index: u32, value: Tagged<Smi>) {
    // SAFETY: 调用方契约
    unsafe { self.set_release(index, value.cast(), WriteBarrierMode::SkipWriteBarrier) };
  }

  /// set(index, Smi, kSeqCstAccess)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_smi_seqcst(&self, index: u32, value: Tagged<Smi>) {
    // SAFETY: 调用方契约
    unsafe { self.set_seqcst(index, value.cast(), WriteBarrierMode::SkipWriteBarrier) };
  }

  /// swap(index, value, kSeqCstAccess, mode) → 旧值。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn swap(
    &self,
    index: u32,
    value: Tagged<Self::Element>,
    mode: WriteBarrierMode,
  ) -> Tagged<Self::Element> {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    let old = unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }
      .swap(value.ptr(), Ordering::SeqCst);
    write_barrier_stub(mode);
    Tagged::new(old)
  }

  /// compare_and_swap(index, expected, value, kSeqCstAccess, mode) → 槽位旧值。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn compare_and_swap(
    &self,
    index: u32,
    expected: Tagged<Self::Element>,
    value: Tagged<Self::Element>,
    mode: WriteBarrierMode,
  ) -> Tagged<Self::Element> {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    let r = unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.compare_exchange(
      expected.ptr(),
      value.ptr(),
      Ordering::SeqCst,
      Ordering::SeqCst,
    );
    let old = match r {
      Ok(v) | Err(v) => v,
    };
    if old == expected.ptr() {
      write_barrier_stub(mode);
    }
    Tagged::new(old)
  }

  /// IsInBounds(index) (index &lt; capacity)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn is_in_bounds(&self, index: u32) -> bool {
    // SAFETY: 调用方契约 (capacity 槽在读)
    unsafe { index < self.capacity() }
  }

  /// SizeFor(capacity) (= OBJECTS_OFFSET + capacity * kElementSize)。
  #[must_use]
  fn size_for(capacity: u32) -> usize;

  /// OffsetOfElementAt(index) (= SizeFor(index))。
  #[must_use]
  fn offset_of_element_at(index: u32) -> usize;

  /// MaxRegularCapacity() (常规页内最大容量)。
  #[must_use]
  fn max_regular_capacity() -> usize;

  /// kMaxCapacity (= kMaxFixedArrayCapacity)。
  const MAX_CAPACITY: u32;

  /// AllocatedSize() (acquire 容量 → SizeFor; 右裁后并发访客对齐)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn allocated_size(&self) -> usize {
    // SAFETY: 调用方契约
    Self::size_for(unsafe { self.capacity_acquire() })
  }

  /// NewCapacityForIndex (fixed-array-inl.h:351-361, 1.5x+16 增长至覆盖 index)。
  #[inline]
  #[must_use]
  fn new_capacity_for_index(index: u32, old_capacity: u32) -> u32 {
    debug_assert!(index >= old_capacity);
    let mut capacity = old_capacity;
    // 原 do-while: 至少增长一轮
    loop {
      capacity = capacity + (capacity >> 1) + 16;
      if capacity > index {
        return capacity;
      }
    }
  }

  /// IsCowArray (map 比较; roots 由调用方注入, 脚本头登记)。
  ///
  /// # Safety
  /// 指针须指向有效对象 (读 map 槽)。
  #[inline]
  unsafe fn is_cow_array(&self, fixed_cow_array_map: Tagged<Map>) -> bool {
    // SAFETY: 调用方契约; 只读 map 首槽
    unsafe { self.as_heap_object().map() == fixed_cow_array_map }
  }

  /// RightTrim 不变式校验 + heap 层登记 (fixed-array-inl.h:198-206)。
  ///
  /// 不变式: 0 &lt; new_capacity &lt;= capacity; 实际裁剪 (filler 发布 +
  /// release 长度) 归 Heap::RightTrimArray (L3 heap)。
  fn right_trim(&self, new_capacity: u32) -> Result<()> {
    // SAFETY: 调用方契约 (对象有效)
    let old = unsafe { self.capacity() };
    if new_capacity == 0 {
      return Err(Error::Invariant("RightTrim: new_capacity > 0 (可能规范化)"));
    }
    if new_capacity > old {
      return Err(Error::Invariant("RightTrim: new_capacity <= capacity"));
    }
    if new_capacity == old {
      return Ok(());
    }
    Err(Error::Unsupported(
      "RightTrim: Heap::RightTrimArray 归 L3 heap",
    ))
  }

  /// RawFieldOfFirstElement → 数据区首槽。
  ///
  /// # Safety
  /// 指针须指向有效数组对象。
  #[inline]
  unsafe fn raw_field_of_first_element(&self) -> *mut TaggedSlot {
    // SAFETY: 调用方契约
    unsafe { self.raw_field_of_element_at(0) }
  }

  /// RawFieldOfElementAt(index) → 元素槽。
  ///
  /// # Safety
  /// 同 raw_field_of_first_element; index &lt; capacity。
  unsafe fn raw_field_of_element_at(&self, index: u32) -> *mut TaggedSlot;

  /// MemsetTagged 语义: [from, to) 填充 value (写屏障一次结算)。
  ///
  /// # Safety
  /// 同 set; to &lt;= capacity。
  #[inline]
  unsafe fn fill(&self, from: u32, to: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      for i in from..to {
        debug_assert!(self.is_in_bounds(i));
        AtomicUsize::from_ptr(self.element_slot(i)).store(value.ptr(), Ordering::Relaxed);
      }
    }
    write_barrier_stub(mode);
  }

  /// 切片复制 (允许内存重叠)
  ///
  /// # Safety
  /// dst 与 src 必须有效，范围不越界。
  #[inline]
  unsafe fn copy_elements(
    dst: &Self,
    dst_index: u32,
    src: &Self,
    src_index: u32,
    len: u32,
    mode: WriteBarrierMode,
  ) {
    if len == 0 {
      return;
    }
    unsafe {
      let dst_ptr = dst.element_slot(dst_index);
      let src_ptr = src.element_slot(src_index) as *const usize;
      ptr::copy(src_ptr, dst_ptr, len as usize);
    }
    write_barrier_stub(mode);
  }

  /// MoveElements (fixed-array-inl.h): 与 CopyElements 同为 memmove 语义,
  /// 同数组重叠自搬保源序; len=0 早退。
  ///
  /// # Safety
  /// 同 [`Self::copy_elements`]。
  #[inline]
  unsafe fn move_elements(
    dst: &Self,
    dst_index: u32,
    src: &Self,
    src_index: u32,
    len: u32,
    mode: WriteBarrierMode,
  ) {
    // SAFETY: 调用方契约 (同 copy_elements)
    unsafe { Self::copy_elements(dst, dst_index, src, src_index, len, mode) }
  }

  /// 元素迭代器 ([0, capacity) 逐槽 get)。
  #[inline]
  fn iter(&self) -> TaggedArrayIter<'_, Self> {
    // SAFETY: 只读迭代, 槽位处于有效容量内
    let n = unsafe { self.capacity() };
    TaggedArrayIter { arr: self, i: 0, n }
  }

  /// 带边界检查的安全读取
  #[inline]
  fn get_checked(&self, index: usize) -> Result<Tagged<Self::Element>> {
    let cap = unsafe { self.capacity() } as usize;
    if index >= cap {
      return Err(Error::OutOfBounds { index, length: cap });
    }
    unsafe { Ok(self.get(index as u32)) }
  }

  /// 带边界检查的安全写入
  #[inline]
  fn set_checked(&self, index: usize, value: Tagged<Self::Element>) -> Result<()> {
    let cap = unsafe { self.capacity() } as usize;
    if index >= cap {
      return Err(Error::OutOfBounds { index, length: cap });
    }
    unsafe { self.set(index as u32, value, WriteBarrierMode::UpdateWriteBarrier) };
    Ok(())
  }

  // ── 内部: 槽位寻址 (实现侧提供) ────────────────────────────────────────

  /// length 槽地址。
  ///
  /// # Safety
  /// 指针须指向有效对象。
  unsafe fn length_addr(&self) -> usize;

  /// capacity 槽地址。
  ///
  /// # Safety
  /// 同 length_addr。
  unsafe fn capacity_addr(&self) -> usize;

  /// 元素槽指针。
  ///
  /// # Safety
  /// 同 length_addr; index &lt; capacity。
  unsafe fn element_slot(&self, index: u32) -> *mut TaggedSlot;

  /// 转 HeapObject tagged 视图。
  fn as_heap_object(&self) -> Tagged<HeapObject>;
}

impl<T: TaggedArray> TaggedArrayExt for Tagged<T> {
  type Element = T::Element;

  #[inline]
  fn size_for(capacity: u32) -> usize {
    T::OBJECTS_OFFSET + capacity as usize * T::ELEMENT_SIZE
  }

  #[inline]
  fn offset_of_element_at(index: u32) -> usize {
    Self::size_for(index)
  }

  #[inline]
  fn max_regular_capacity() -> usize {
    (K_MAX_REGULAR_HEAP_OBJECT_SIZE - T::OBJECTS_OFFSET) / T::ELEMENT_SIZE
  }

  const MAX_CAPACITY: u32 = K_MAX_FIXED_ARRAY_CAPACITY;

  #[inline]
  unsafe fn length_addr(&self) -> usize {
    self.address() + T::LENGTH_OFFSET
  }

  #[inline]
  unsafe fn capacity_addr(&self) -> usize {
    self.address() + T::CAPACITY_OFFSET
  }

  #[inline]
  unsafe fn element_slot(&self, index: u32) -> *mut TaggedSlot {
    (self.address() + T::OBJECTS_OFFSET + index as usize * T::ELEMENT_SIZE) as *mut TaggedSlot
  }

  #[inline]
  unsafe fn raw_field_of_element_at(&self, index: u32) -> *mut TaggedSlot {
    // SAFETY: 调用方契约; 元素槽即裸槽位
    unsafe { self.element_slot(index) }
  }

  #[inline]
  fn as_heap_object(&self) -> Tagged<HeapObject> {
    self.cast()
  }
}

// ══ 具体族 (9 个 TaggedArrayBase 族, 机械展开) ═════════════════════════════

/// FixedArray (fixed-array.h: TaggedArrayBase<FixedArray, Object, FixedArrayBase>)。
///
/// 头继承 FixedArrayBase (map + length); 元素尾 (objects) 经基址 + 数据区偏移
/// 裸访问; snapshot 数据区锚点 = fixed_array_base::K_HEADER_SIZE。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FixedArray {
  /// FixedArrayBase 头。
  pub base: FixedArrayBase,
}

impl FixedArray {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      base: FixedArrayBase::new(map, length),
    }
  }

  /// 计算全对象所需字节大小 (= TaggedArrayExt::size_for, 元素步长同轨)
  #[inline]
  #[must_use]
  pub const fn size_for(capacity: u32) -> usize {
    Self::OBJECTS_OFFSET + (capacity as usize) * Self::ELEMENT_SIZE
  }

  /// 元素偏移量 (= OffsetOfElementAt)
  #[inline]
  #[must_use]
  pub const fn offset_of_element_at(index: u32) -> usize {
    Self::size_for(index)
  }
}

impl TaggedArray for FixedArray {
  type Element = Object;

  const LENGTH_OFFSET: usize = mem::offset_of!(FixedArray, base.length);
  const CAPACITY_OFFSET: usize = Self::LENGTH_OFFSET; // capacity_field() == length_
  const OBJECTS_OFFSET: usize = mem::size_of::<FixedArray>();
}

/// TrustedFixedArray (fixed-array.h: TaggedArrayBase<TrustedFixedArray, Object, TrustedFixedArrayBase>)。
///
/// 头继承 TrustedFixedArrayBase (map + length); 元素尾 (objects) 经基址 + 数据区偏移
/// 裸访问; snapshot 数据区锚点 = fixed_array_base::K_HEADER_SIZE。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TrustedFixedArray {
  /// TrustedFixedArrayBase 头。
  pub trusted_base: TrustedFixedArrayBase,
}

impl TrustedFixedArray {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      trusted_base: TrustedFixedArrayBase::new(map, length),
    }
  }
}

impl TaggedArray for TrustedFixedArray {
  type Element = Object;

  const LENGTH_OFFSET: usize = mem::offset_of!(TrustedFixedArray, trusted_base.length);
  const CAPACITY_OFFSET: usize = Self::LENGTH_OFFSET; // capacity_field() == length_
  const OBJECTS_OFFSET: usize = mem::size_of::<TrustedFixedArray>();
}

/// ProtectedFixedArray (fixed-array.h: TaggedArrayBase<ProtectedFixedArray, UnionOf<TrustedObject, Smi>, TrustedFixedArrayBase>)。
///
/// 头继承 TrustedFixedArrayBase (map + length); 元素尾 (objects) 经基址 + 数据区偏移
/// 裸访问; snapshot 数据区锚点 = fixed_array_base::K_HEADER_SIZE。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ProtectedFixedArray {
  /// TrustedFixedArrayBase 头。
  pub trusted_base: TrustedFixedArrayBase,
}

impl ProtectedFixedArray {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      trusted_base: TrustedFixedArrayBase::new(map, length),
    }
  }
}

impl TaggedArray for ProtectedFixedArray {
  type Element = TrustedObjectOrSmi;

  const LENGTH_OFFSET: usize = mem::offset_of!(ProtectedFixedArray, trusted_base.length);
  const CAPACITY_OFFSET: usize = Self::LENGTH_OFFSET; // capacity_field() == length_
  const OBJECTS_OFFSET: usize = mem::size_of::<ProtectedFixedArray>();
}

/// WeakFixedArray (fixed-array.h: TaggedArrayBase<WeakFixedArray, MaybeObject, FixedArrayBase>)。
///
/// 头继承 FixedArrayBase (map + length); 元素尾 (objects) 经基址 + 数据区偏移
/// 裸访问; snapshot 数据区锚点 = fixed_array_base::K_HEADER_SIZE。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WeakFixedArray {
  /// FixedArrayBase 头。
  pub base: FixedArrayBase,
}

impl WeakFixedArray {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      base: FixedArrayBase::new(map, length),
    }
  }
}

impl TaggedArray for WeakFixedArray {
  type Element = MaybeObject;

  const LENGTH_OFFSET: usize = mem::offset_of!(WeakFixedArray, base.length);
  const CAPACITY_OFFSET: usize = Self::LENGTH_OFFSET; // capacity_field() == length_
  const OBJECTS_OFFSET: usize = mem::size_of::<WeakFixedArray>();
}

/// WeakHomomorphicFixedArray (fixed-array.h: TaggedArrayBase<WeakHomomorphicFixedArray, MaybeObject, FixedArrayBase>)。
///
/// 头继承 FixedArrayBase (map + length); 元素尾 (objects) 经基址 + 数据区偏移
/// 裸访问; snapshot 数据区锚点 = fixed_array_base::K_HEADER_SIZE。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WeakHomomorphicFixedArray {
  /// FixedArrayBase 头。
  pub base: FixedArrayBase,
}

impl WeakHomomorphicFixedArray {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      base: FixedArrayBase::new(map, length),
    }
  }
}

impl TaggedArray for WeakHomomorphicFixedArray {
  type Element = MaybeObject;

  const LENGTH_OFFSET: usize = mem::offset_of!(WeakHomomorphicFixedArray, base.length);
  const CAPACITY_OFFSET: usize = Self::LENGTH_OFFSET; // capacity_field() == length_
  const OBJECTS_OFFSET: usize = mem::size_of::<WeakHomomorphicFixedArray>();
}

/// TrustedWeakFixedArray (fixed-array.h: TaggedArrayBase<TrustedWeakFixedArray, MaybeObject, TrustedFixedArrayBase>)。
///
/// 头继承 TrustedFixedArrayBase (map + length); 元素尾 (objects) 经基址 + 数据区偏移
/// 裸访问; snapshot 数据区锚点 = fixed_array_base::K_HEADER_SIZE。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TrustedWeakFixedArray {
  /// TrustedFixedArrayBase 头。
  pub trusted_base: TrustedFixedArrayBase,
}

impl TrustedWeakFixedArray {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      trusted_base: TrustedFixedArrayBase::new(map, length),
    }
  }
}

impl TaggedArray for TrustedWeakFixedArray {
  type Element = MaybeObject;

  const LENGTH_OFFSET: usize = mem::offset_of!(TrustedWeakFixedArray, trusted_base.length);
  const CAPACITY_OFFSET: usize = Self::LENGTH_OFFSET; // capacity_field() == length_
  const OBJECTS_OFFSET: usize = mem::size_of::<TrustedWeakFixedArray>();
}

/// ProtectedWeakFixedArray (fixed-array.h: TaggedArrayBase<ProtectedWeakFixedArray, UnionOf<MaybeWeak<TrustedObject>, Smi>, TrustedFixedArrayBase>)。
///
/// 头继承 TrustedFixedArrayBase (map + length); 元素尾 (objects) 经基址 + 数据区偏移
/// 裸访问; snapshot 数据区锚点 = fixed_array_base::K_HEADER_SIZE。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ProtectedWeakFixedArray {
  /// TrustedFixedArrayBase 头。
  pub trusted_base: TrustedFixedArrayBase,
}

impl ProtectedWeakFixedArray {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      trusted_base: TrustedFixedArrayBase::new(map, length),
    }
  }
}

impl TaggedArray for ProtectedWeakFixedArray {
  type Element = MaybeWeakTrustedObjectOrSmi;

  const LENGTH_OFFSET: usize = mem::offset_of!(ProtectedWeakFixedArray, trusted_base.length);
  const CAPACITY_OFFSET: usize = Self::LENGTH_OFFSET; // capacity_field() == length_
  const OBJECTS_OFFSET: usize = mem::size_of::<ProtectedWeakFixedArray>();
}

/// WeakArrayList (fixed-array.h: TaggedArrayBase<WeakArrayList, MaybeObject, HeapObject>)。
///
/// 自声明 capacity_ + length_ 双 u32 头 (无基类 length 继承);
/// 元素尾 (objects) 经基址 + 数据区偏移裸访问。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WeakArrayList {
  /// HeapObject 头 (map 槽)。
  pub header: HeapObject,
  /// 分配容量 (V8_TQ_CONST uint32_t capacity_)。
  pub capacity: u32,
  /// 已用元素数 (uint32_t length_)。
  pub length: u32,
}

impl WeakArrayList {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, capacity: u32, length: u32) -> Self {
    Self {
      header: HeapObject::new(map),
      capacity,
      length,
    }
  }

  /// 计算全对象所需字节大小 (= TaggedArrayExt::size_for, 元素步长同轨)
  #[inline]
  #[must_use]
  pub const fn size_for(capacity: u32) -> usize {
    Self::OBJECTS_OFFSET + (capacity as usize) * Self::ELEMENT_SIZE
  }

  /// 依据容量计算对象大小 (= SizeForCapacity)
  #[inline]
  #[must_use]
  pub const fn size_for_capacity(capacity: u32) -> usize {
    Self::size_for(capacity)
  }

  /// 增长容量计算 (CapacityForLength: length + max(length/2, 2))
  #[inline]
  #[must_use]
  pub const fn capacity_for_length(length: u32) -> u32 {
    let half = length / 2;
    length + if half > 2 { half } else { 2 }
  }
}

impl TaggedArray for WeakArrayList {
  type Element = MaybeObject;

  const LENGTH_OFFSET: usize = mem::offset_of!(WeakArrayList, length);
  const CAPACITY_OFFSET: usize = mem::offset_of!(WeakArrayList, capacity);
  const OBJECTS_OFFSET: usize = mem::size_of::<WeakArrayList>();
}

/// ArrayList (fixed-array.h: TaggedArrayBase<ArrayList, Object, HeapObject>)。
///
/// 自声明 capacity_ + length_ 双 u32 头 (无基类 length 继承);
/// 元素尾 (objects) 经基址 + 数据区偏移裸访问。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ArrayList {
  /// HeapObject 头 (map 槽)。
  pub header: HeapObject,
  /// 分配容量 (V8_TQ_CONST uint32_t capacity_)。
  pub capacity: u32,
  /// 已用元素数 (uint32_t length_)。
  pub length: u32,
}

impl ArrayList {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, capacity: u32, length: u32) -> Self {
    Self {
      header: HeapObject::new(map),
      capacity,
      length,
    }
  }

  /// 计算全对象所需字节大小 (= TaggedArrayExt::size_for, 元素步长同轨)
  #[inline]
  #[must_use]
  pub const fn size_for(capacity: u32) -> usize {
    Self::OBJECTS_OFFSET + (capacity as usize) * Self::ELEMENT_SIZE
  }

  /// 增长容量计算 (CapacityForLength: length + max(length/2, 2))
  #[inline]
  #[must_use]
  pub const fn capacity_for_length(length: u32) -> u32 {
    let half = length / 2;
    length + if half > 2 { half } else { 2 }
  }
}

impl TaggedArray for ArrayList {
  type Element = Object;

  const LENGTH_OFFSET: usize = mem::offset_of!(ArrayList, length);
  const CAPACITY_OFFSET: usize = mem::offset_of!(ArrayList, capacity);
  const OBJECTS_OFFSET: usize = mem::size_of::<ArrayList>();
}

// ── capacity+length 族 snapshot 布局 (fixed-array.h:646-648, 725-727) ──────
/// WeakArrayList::kCapacityOffset (= sizeof(HeapObject))。
pub const K_WEAK_ARRAY_LIST_CAPACITY_OFFSET: usize = K_TAGGED_SIZE;
/// WeakArrayList::kLengthOffset。
pub const K_WEAK_ARRAY_LIST_LENGTH_OFFSET: usize = K_WEAK_ARRAY_LIST_CAPACITY_OFFSET + 4;
/// WeakArrayList::kHeaderSize。
pub const K_WEAK_ARRAY_LIST_HEADER_SIZE: usize = K_WEAK_ARRAY_LIST_LENGTH_OFFSET + 4;
/// ArrayList::kCapacityOffset。
pub const K_ARRAY_LIST_CAPACITY_OFFSET: usize = K_TAGGED_SIZE;
/// ArrayList::kLengthOffset。
pub const K_ARRAY_LIST_LENGTH_OFFSET: usize = K_ARRAY_LIST_CAPACITY_OFFSET + 4;
/// ArrayList::kHeaderSize。
pub const K_ARRAY_LIST_HEADER_SIZE: usize = K_ARRAY_LIST_LENGTH_OFFSET + 4;

const _: () = assert!(K_WEAK_ARRAY_LIST_HEADER_SIZE == K_ARRAY_LIST_HEADER_SIZE);

/// FixedArrayExact (fixed-array.h:411): FixedArray 精确类型别名。
///
/// 仅 IsFixedArrayExact 谓词 (= FIXED_ARRAY_TYPE 精确匹配, 非区间) 需要;
/// Rust 侧与 FixedArray 共享类型与 Cast 区间谓词, 精确谓词独立成函数
/// (类型层合并登记)。
pub type FixedArrayExact = FixedArray;

/// IsFixedArrayExact 谓词 (== FIXED_ARRAY_TYPE 精确匹配)。
#[inline]
#[must_use]
pub fn is_fixed_array_exact(value: Tagged<Object>) -> bool {
  instance_type_of(value).is_some_and(itc::is_fixed_array_exact)
}

// ── Cast (casting.h DEF_CAST_TRAITS 对应; 覆盖 CAST_TRAITS_TYPES 核对项:
//    FixedArrayBase / FixedArrayExact / WeakHomomorphicFixedArray) ──────────
impl Cast for FixedArrayBase {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_fixed_array_base)
  }
}

impl Cast for TrustedFixedArrayBase {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_trusted_fixed_array_base)
  }
}

impl Cast for FixedArray {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_fixed_array)
  }
}

impl Cast for TrustedFixedArray {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_trusted_fixed_array)
  }
}

impl Cast for ProtectedFixedArray {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_protected_fixed_array)
  }
}

impl Cast for WeakFixedArray {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_weak_fixed_array)
  }
}

impl Cast for WeakHomomorphicFixedArray {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_weak_homomorphic_fixed_array)
  }
}

impl Cast for TrustedWeakFixedArray {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_trusted_weak_fixed_array)
  }
}

impl Cast for ProtectedWeakFixedArray {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_protected_weak_fixed_array)
  }
}

impl Cast for WeakArrayList {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_weak_array_list)
  }
}

impl Cast for ArrayList {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_array_list)
  }
}

// ══ 族专属语义 (fixed-array-inl.h) ═══════════

/// Tagged&lt;FixedArray&gt; 专属语义 (fixed-array-inl.h:379-421)。
///
/// hole 槽值由调用方注入 (ReadOnlyRoots 归 L3 heap, 脚本头登记)。
pub trait FixedArrayExt: Copy + TaggedArrayExt<Element = Object> {
  /// is_the_hole(index) (与 hole 槽值位比较)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get。
  #[inline]
  unsafe fn is_the_hole(&self, hole: Tagged<Object>, index: u32) -> bool {
    // SAFETY: 调用方契约
    unsafe { self.get(index) == hole }
  }

  /// set_the_hole(index) (SKIP 屏障写 hole 槽值)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::set。
  #[inline]
  unsafe fn set_the_hole(&self, hole: Tagged<Object>, index: u32) {
    // SAFETY: 调用方契约; 只读根免屏障
    unsafe { self.set_relaxed(index, hole, WriteBarrierMode::SkipWriteBarrier) };
  }

  /// FillWithHoles(from, to)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::fill。
  #[inline]
  unsafe fn fill_with_holes(&self, hole: Tagged<Object>, from: u32, to: u32) {
    // SAFETY: 调用方契约
    unsafe { self.fill(from, to, hole, WriteBarrierMode::SkipWriteBarrier) };
  }
}

impl FixedArrayExt for Tagged<FixedArray> {}

/// Tagged&lt;WeakArrayList&gt; 专属语义 (fixed-array.h:558-679 + inl:499-529)。
///
/// Get/Set 与 TaggedArrayExt 的 get/set 同签名同语义 (Relaxed), 不重复
/// 承载 (调用歧义规避); Set 的数组须足够大, 需扩容走 heap 层 AddToEnd (登记)。
pub trait WeakArrayListExt: Copy + TaggedArrayExt<Element = MaybeObject> {
  /// Set(index, Smi) (SKIP 屏障; inl:508)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::set。
  #[inline]
  unsafe fn set_smi(&self, index: u32, value: Tagged<Smi>) {
    // SAFETY: 调用方契约
    unsafe {
      TaggedArrayExt::set_relaxed(
        self,
        index,
        value.cast(),
        WriteBarrierMode::SkipWriteBarrier,
      )
    };
  }

  /// SizeForCapacity(capacity) (= SizeFor)。
  #[inline]
  #[must_use]
  fn size_for_capacity(capacity: u32) -> usize {
    <Tagged<WeakArrayList> as TaggedArrayExt>::size_for(capacity)
  }

  /// CapacityForLength(length) (length + max(length/2, 2); inl 同源 h:600)。
  #[inline]
  #[must_use]
  fn capacity_for_length(length: u32) -> u32 {
    length + (if length / 2 > 2 { length / 2 } else { 2 })
  }

  /// data_start() (MaybeObjectSlot → 槽位指针)。
  ///
  /// # Safety
  /// 指针须指向有效数组对象。
  #[inline]
  unsafe fn data_start(&self) -> *mut TaggedSlot {
    // SAFETY: 调用方契约
    unsafe { self.raw_field_of_first_element() }
  }

  /// 弱堆对象迭代器 (Iterator::Next 语义: 跳过已清除弱引用)。
  ///
  /// # Safety
  /// 迭代期间对象须有效存活 (tagged 裸引用, GC 禁止窗口由调用方保证)。
  unsafe fn iter_weak_heap_objects(self) -> WeakArrayListIter;
}

impl WeakArrayListExt for Tagged<WeakArrayList> {
  #[inline]
  unsafe fn iter_weak_heap_objects(self) -> WeakArrayListIter {
    WeakArrayListIter {
      array: self,
      index: 0,
    }
  }
}

/// WeakArrayList::Iterator (fixed-array.h:667-679)。
///
/// 逐个产出未清除的弱引用堆对象; 交错 Smi 元素原版 DCHECK 拦截 →
/// 此处跳过弱谓词不满足项 (登记: 断言语义弱化为跳过)。
pub struct WeakArrayListIter {
  array: Tagged<WeakArrayList>,
  index: u32,
}

impl Iterator for WeakArrayListIter {
  type Item = Tagged<HeapObject>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.array.is_null() {
      return None;
    }
    // SAFETY: 构造方契约 (iter_weak_heap_objects): 迭代期间对象有效
    let len = unsafe { TaggedArrayExt::length(&self.array) };
    while self.index < len {
      // SAFETY: 同上
      let item = unsafe { TaggedArrayExt::get(&self.array, self.index) };
      self.index += 1;
      debug_assert!(item.is_weak_or_cleared() || item.is_smi());
      if let Some(obj) = item.get_heap_object_if_weak() {
        return Some(obj);
      }
    }
    self.array = Tagged::NULL;
    None
  }
}

// == fixed-array.cc 语义区段 (由 tool/src/objects/fixed-array.cc.js 维护) ==
// @cc-semantics-begin
// 由 tool/src/objects/fixed-array.cc.js 从 v8/src/objects/fixed-array.cc (纯数组语义子集) 生成; 修订就地改, 复跑覆盖
// WeakArrayList Compact/IsFull/CountLive*/RemoveOne/Contains; isolate/factory 依赖推迟 (脚本头登记)

// ── WeakArrayList 纯数组方法 (fixed-array.cc:271-352) ─────────────────────
/// WeakArrayList 的 fixed-array.cc 语义 (trait 承载, 与 -inl.h 语义的
/// WeakArrayListExt 分立, 保留来源可追溯)。
pub trait WeakArrayListCc: Copy + TaggedArrayExt<Element = MaybeObject> {
  /// Compact (cc:271): 清除项剔除, 存活项原地前移; length 收敛为存活数。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get/set; 迭代期间无并发修改。
  #[inline]
  unsafe fn compact(&self) {
    // SAFETY: 调用方契约
    unsafe {
      let len = self.length();
      let mut new_length = 0u32;
      for i in 0..len {
        let value = self.get(i);
        if !value.is_cleared() {
          if new_length != i {
            // Set(new_length, value): 存活项前移
            self.set_relaxed(new_length, value, WriteBarrierMode::UpdateWriteBarrier);
          }
          new_length += 1;
        }
      }
      self.set_length(new_length);
    }
  }

  /// IsFull (cc:290)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::capacity。
  #[inline]
  unsafe fn is_full(&self) -> bool {
    // SAFETY: 调用方契约
    unsafe { self.length() == self.capacity() }
  }

  /// CountLiveWeakReferences (cc:306): 活弱引用计数。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get。
  #[inline]
  unsafe fn count_live_weak_references(&self) -> u32 {
    // SAFETY: 调用方契约; 单遍历计数 (闭包内 unsafe: 槽位读)
    unsafe {
      (0..self.length())
        .filter(|&i| self.get(i).is_weak_live())
        .count() as u32
    }
  }

  /// CountLiveElements (cc:317): 未清除元素计数。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get。
  #[inline]
  unsafe fn count_live_elements(&self) -> u32 {
    // SAFETY: 调用方契约; 单遍历计数
    unsafe {
      (0..self.length())
        .filter(|&i| !self.get(i).is_cleared())
        .count() as u32
    }
  }

  /// RemoveOne (cc:328): 倒序找到首个等值项 → 尾元素补位 + 尾槽置清除
  /// 哨兵 + length-1; 未找到 → false (元素序会变, 调用方须知)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get/set。
  #[inline]
  unsafe fn remove_one(&self, value: Tagged<MaybeObject>) -> bool {
    // SAFETY: 调用方契约
    unsafe {
      let len = self.length();
      if len == 0 {
        return false;
      }
      let last_index = len - 1;
      let mode = WriteBarrierMode::UpdateWriteBarrier;
      for i in (0..len).rev() {
        if self.get(i) != value {
          continue;
        }
        let last = self.get(last_index);
        self.set_relaxed(i, last, mode);
        self.set_relaxed(last_index, Tagged::cleared(), mode);
        self.set_length(last_index);
        return true;
      }
      false
    }
  }

  /// Contains (cc:346): 线性查找等值项。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get。
  #[inline]
  unsafe fn contains(&self, value: Tagged<MaybeObject>) -> bool {
    // SAFETY: 调用方契约; 提前终止扫描
    unsafe { (0..self.length()).any(|i| self.get(i) == value) }
  }
}

impl WeakArrayListCc for Tagged<WeakArrayList> {}
// @cc-semantics-end
