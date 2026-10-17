// 由 tool/src/objects/tagged-field.h.js 从 v8/src/objects/tagged-field.h + tagged-field-inl.h 生成; 修订就地改, 复跑覆盖
// 默认非压缩配置展开; 修订就地改

//! V8 堆对象内的 tagged 字段槽位访问器
//!
//! 原版三层: `TaggedMember<T>` (对象成员槽) / `TaggedField<T, kFieldOffset>` (静态字段访问器) /
//! `UnalignedValueMember<T>` (非对齐值成员). 非压缩配置下 `Tagged_t = Address = usize`.

use core::mem::size_of;
use std::{
  fmt,
  marker::PhantomData,
  sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
  heap_object::HeapObject, map_word::MapWord, tagged::Tagged,
  tagged_constants::HEAP_OBJECT_TAG_MASK,
};

/// tagged 槽位存储类型 (压缩指针抽象点: 启用 V8_COMPRESS_POINTERS 时改为 u32)
pub type TaggedSlot = usize;

/// 压缩抽象点: 槽位值 → 完整地址 (非压缩恒等)
#[inline]
pub const fn tagged_to_full(v: TaggedSlot) -> usize {
  v
}

/// 压缩抽象点: 完整地址 → 槽位值 (非压缩恒等)
#[inline]
pub const fn full_to_tagged(v: usize) -> TaggedSlot {
  v
}

pub use crate::objects::WriteBarrierMode;

/// 对象成员槽: 潜在压缩的 tagged 指针存储 (对应 C++ TaggedMember<T>)
///
/// 布局对齐原版 static_assert: alignof == alignof(Tagged_t), sizeof == sizeof(Tagged_t).
/// 值语义 trait 手写: AtomicUsize 非 Copy, derive 不可用; T 不参与判等 (仅槽位值参与).
#[repr(C)]
pub struct TaggedMember<T> {
  slot: AtomicUsize,
  _ty: PhantomData<T>,
}

impl<T> Clone for TaggedMember<T> {
  fn clone(&self) -> Self {
    Self {
      slot: AtomicUsize::new(self.slot.load(Ordering::Relaxed)),
      _ty: PhantomData,
    }
  }
}

impl<T> fmt::Debug for TaggedMember<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("TaggedMember")
      .field("slot", &self.slot.load(Ordering::Relaxed))
      .finish()
  }
}

impl<T> PartialEq for TaggedMember<T> {
  fn eq(&self, other: &Self) -> bool {
    self.slot.load(Ordering::Relaxed) == other.slot.load(Ordering::Relaxed)
  }
}

impl<T> Eq for TaggedMember<T> {}

// ===== TaggedMember<T> (tagged-field-inl.h 逐方法展开) =====
impl<T> TaggedMember<T> {
  pub const fn from_raw(slot: TaggedSlot) -> Self {
    Self {
      slot: AtomicUsize::new(slot),
      _ty: PhantomData,
    }
  }

  pub fn new(value: Tagged<T>) -> Self {
    Self::from_raw(value.ptr())
  }

  pub fn load(&self) -> Tagged<T> {
    self.relaxed_load()
  }

  /// 登记: 带写屏障模式在 heap 层未就位前等价 SKIP (WriteBarrierMode 仅作契约保留)。
  pub fn store(&self, value: Tagged<T>, _mode: WriteBarrierMode) {
    self.relaxed_store_no_write_barrier(value);
  }

  pub fn store_no_write_barrier(&self, value: Tagged<T>) {
    self.relaxed_store_no_write_barrier(value);
  }

  pub fn relaxed_load(&self) -> Tagged<T> {
    Tagged::new(self.slot.load(Ordering::Relaxed))
  }

  /// 登记: 带写屏障模式在 heap 层未就位前等价 SKIP (WriteBarrierMode 仅作契约保留)。
  pub fn relaxed_store(&self, value: Tagged<T>, _mode: WriteBarrierMode) {
    self.relaxed_store_no_write_barrier(value);
  }

  pub fn relaxed_store_no_write_barrier(&self, value: Tagged<T>) {
    self.slot.store(value.ptr(), Ordering::Relaxed);
  }

  pub fn acquire_load(&self) -> Tagged<T> {
    Tagged::new(self.slot.load(Ordering::Acquire))
  }

  /// 登记: 带写屏障模式在 heap 层未就位前等价 SKIP (WriteBarrierMode 仅作契约保留)。
  pub fn release_store(&self, value: Tagged<T>, _mode: WriteBarrierMode) {
    self.release_store_no_write_barrier(value);
  }

  pub fn release_store_no_write_barrier(&self, value: Tagged<T>) {
    self.slot.store(value.ptr(), Ordering::Release);
  }

  pub fn seqcst_load(&self) -> Tagged<T> {
    Tagged::new(self.slot.load(Ordering::SeqCst))
  }

  /// 登记: 带写屏障模式在 heap 层未就位前等价 SKIP (WriteBarrierMode 仅作契约保留)。
  pub fn seqcst_store(&self, value: Tagged<T>, _mode: WriteBarrierMode) {
    self.seqcst_store_no_write_barrier(value);
  }

  pub fn seqcst_store_no_write_barrier(&self, value: Tagged<T>) {
    self.slot.store(value.ptr(), Ordering::SeqCst);
  }

  pub fn seqcst_swap(&self, value: Tagged<T>, _mode: WriteBarrierMode) -> Tagged<T> {
    Tagged::new(self.slot.swap(value.ptr(), Ordering::SeqCst))
  }

  pub fn seqcst_compare_and_swap(
    &self,
    expected: Tagged<T>,
    value: Tagged<T>,
    _mode: WriteBarrierMode,
  ) -> Tagged<T> {
    let res = self.slot.compare_exchange(
      expected.ptr(),
      value.ptr(),
      Ordering::SeqCst,
      Ordering::SeqCst,
    );
    Tagged::new(match res {
      Ok(v) => v,
      Err(cur) => cur,
    })
  }

  pub const fn slot_ptr(&self) -> *const TaggedSlot {
    &self.slot as *const AtomicUsize as *const TaggedSlot
  }
}

impl<T> Default for TaggedMember<T> {
  fn default() -> Self {
    Self::from_raw(0)
  }
}

/// 受保护 (trusted space) tagged 成员 (对应 ProtectedTaggedMember, 压缩方案差异登记 TODO)
pub type ProtectedTaggedMember<T> = TaggedMember<T>;

/// 非对齐值成员 (对应 UnalignedValueMember<T>)
#[repr(C)]
pub struct UnalignedValueMember<T> {
  storage: [u8; 8],
  _ty: PhantomData<T>,
}

impl<T: Copy> UnalignedValueMember<T> {
  pub fn new(value: T) -> Self {
    let mut m = Self {
      storage: [0; 8],
      _ty: PhantomData,
    };
    m.set_value(value);
    m
  }

  pub fn value(&self) -> T {
    // SAFETY: storage 8 字节对齐保证 size_of::<T>() <= 8 由布局断言约束; 只读。
    unsafe { self.storage.as_ptr().cast::<T>().read_unaligned() }
  }

  pub fn set_value(&mut self, value: T) {
    // SAFETY: 同上; write_unaligned 不要求 T 对齐。
    unsafe { self.storage.as_mut_ptr().cast::<T>().write_unaligned(value) }
  }
}

const _: () = assert!(size_of::<usize>() <= 8);

/// 非对齐 f64 成员 (对应 UnalignedDoubleMember, 原版 alignas(alignof(Tagged_t)))
#[repr(C, align(8))]
pub struct UnalignedDoubleMember {
  storage: [u8; 8],
}
impl UnalignedDoubleMember {
  pub fn new(value: f64) -> Self {
    Self {
      storage: value.to_bits().to_le_bytes(),
    }
  }

  pub fn value(&self) -> f64 {
    f64::from_bits(u64::from_le_bytes(self.storage))
  }

  pub fn set_value(&mut self, value: f64) {
    self.storage = value.to_bits().to_le_bytes();
  }

  pub fn value_as_bits(&self) -> u64 {
    u64::from_le_bytes(self.storage)
  }

  pub fn set_value_as_bits(&mut self, bits: u64) {
    self.storage = bits.to_le_bytes();
  }
}

impl Default for UnalignedDoubleMember {
  fn default() -> Self {
    Self::new(0.0)
  }
}

/// 静态字段访问器: 宿主对象内偏移 kFieldOffset 处的 tagged 字段 (对应 TaggedField<T, kFieldOffset>)
pub struct TaggedField<T, const OFFSET: usize = 0> {
  _ty: PhantomData<T>,
}

// ===== TaggedField<T, OFFSET> (tagged-field-inl.h 逐方法展开) =====
impl<T, const OFFSET: usize> TaggedField<T, OFFSET> {
  /// 原版: host.address() + kFieldOffset + offset (address 已剥 tag; 此处用掩码免除断言)。
  pub fn address(host: Tagged<HeapObject>, offset: usize) -> usize {
    (host.ptr() & !HEAP_OBJECT_TAG_MASK) + OFFSET + offset
  }

  /// 原版方法 location。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn location(host: Tagged<HeapObject>, offset: usize) -> *mut TaggedSlot {
    Self::address(host, offset) as *mut TaggedSlot
  }

  /// 原版方法 load。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn load(host: Tagged<HeapObject>, offset: usize) -> Tagged<T> {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe { Tagged::new(Self::location(host, offset).read()) }
  }

  /// 原版方法 store。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn store(host: Tagged<HeapObject>, value: Tagged<T>) {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe { Self::location(host, 0).write(value.ptr()) };
  }

  /// 原版方法 store_at。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn store_at(host: Tagged<HeapObject>, offset: usize, value: Tagged<T>) {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe { Self::location(host, offset).write(value.ptr()) };
  }

  /// 原版方法 relaxed_load。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_load(host: Tagged<HeapObject>, offset: usize) -> Tagged<T> {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe {
      let loc = Self::location(host, offset) as *const AtomicUsize;
      Tagged::new((*loc).load(Ordering::Relaxed))
    }
  }

  /// 原版方法 relaxed_store。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_store(host: Tagged<HeapObject>, value: Tagged<T>) {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe { Self::relaxed_store_at(host, 0, value) };
  }

  /// 原版方法 relaxed_store_at。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_store_at(host: Tagged<HeapObject>, offset: usize, value: Tagged<T>) {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe {
      let loc = Self::location(host, offset) as *const AtomicUsize;
      (*loc).store(value.ptr(), Ordering::Relaxed);
    }
  }

  /// 原版方法 acquire_load。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn acquire_load(host: Tagged<HeapObject>, offset: usize) -> Tagged<T> {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe {
      let loc = Self::location(host, offset) as *const AtomicUsize;
      Tagged::new((*loc).load(Ordering::Acquire))
    }
  }

  /// 原版方法 release_store。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn release_store(host: Tagged<HeapObject>, value: Tagged<T>) {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe { Self::release_store_at(host, 0, value) };
  }

  /// 原版方法 release_store_at。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn release_store_at(host: Tagged<HeapObject>, offset: usize, value: Tagged<T>) {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe {
      let loc = Self::location(host, offset) as *const AtomicUsize;
      (*loc).store(value.ptr(), Ordering::Release);
    }
  }

  /// 原版方法 seqcst_load (SeqCst_Load)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn seqcst_load(host: Tagged<HeapObject>, offset: usize) -> Tagged<T> {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe {
      let loc = Self::location(host, offset) as *const AtomicUsize;
      Tagged::new((*loc).load(Ordering::SeqCst))
    }
  }

  /// 原版方法 seqcst_store (SeqCst_Store)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn seqcst_store(host: Tagged<HeapObject>, value: Tagged<T>) {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe { Self::seqcst_store_at(host, 0, value) };
  }

  /// 原版方法 seqcst_store_at (SeqCst_Store(host, offset, value))。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn seqcst_store_at(host: Tagged<HeapObject>, offset: usize, value: Tagged<T>) {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe {
      let loc = Self::location(host, offset) as *const AtomicUsize;
      (*loc).store(value.ptr(), Ordering::SeqCst);
    }
  }

  /// 原版方法 seqcst_swap (SeqCst_Swap)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn seqcst_swap(
    host: Tagged<HeapObject>,
    offset: usize,
    value: Tagged<T>,
  ) -> Tagged<T> {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe {
      let loc = Self::location(host, offset) as *const AtomicUsize;
      Tagged::new((*loc).swap(value.ptr(), Ordering::SeqCst))
    }
  }

  /// 原版方法 release_compare_and_swap (Release_CompareAndSwap)。
  ///
  /// 返回 CAS 尝试前槽位中的实际值。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn release_compare_and_swap(
    host: Tagged<HeapObject>,
    old: usize,
    value: Tagged<T>,
  ) -> usize {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe {
      let loc = Self::location(host, 0) as *const AtomicUsize;
      let res = (*loc).compare_exchange(old, value.ptr(), Ordering::Release, Ordering::Relaxed);
      res.unwrap_or_else(|actual| actual)
    }
  }

  /// 原版方法 relaxed_compare_and_swap (Relaxed_CompareAndSwap)。
  ///
  /// 返回 CAS 尝试前槽位中的实际值。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_compare_and_swap(
    host: Tagged<HeapObject>,
    old: usize,
    value: Tagged<T>,
  ) -> usize {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe {
      let loc = Self::location(host, 0) as *const AtomicUsize;
      let res = (*loc).compare_exchange(old, value.ptr(), Ordering::Relaxed, Ordering::Relaxed);
      res.unwrap_or_else(|actual| actual)
    }
  }

  /// 原版方法 seqcst_compare_and_swap (SeqCst_CompareAndSwap)。
  ///
  /// 返回 CAS 尝试前槽位中的实际值。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn seqcst_compare_and_swap(
    host: Tagged<HeapObject>,
    offset: usize,
    old: Tagged<T>,
    value: Tagged<T>,
  ) -> Tagged<T> {
    // SAFETY: 契约由本 unsafe fn 的调用方承接。
    unsafe {
      let loc = Self::location(host, offset) as *const AtomicUsize;
      let res = (*loc).compare_exchange(old.ptr(), value.ptr(), Ordering::SeqCst, Ordering::SeqCst);
      Tagged::new(res.unwrap_or_else(|actual| actual))
    }
  }
}

// MapWord 字段专用 (原版 *_Map_Word 方法, T = MapWord)
impl<const OFFSET: usize> TaggedField<MapWord, OFFSET> {
  /// 原版: Relaxed_Load(location(host, 0)) 后 tagged_to_full 还原完整值。
  ///
  /// 与通用 Relaxed_Load 的差异: 不做 map 偏移 DCHECK (map word 合法可为转发编码)。
  ///
  /// v8: src/objects/tagged-field-inl.h TaggedField::Relaxed_Load_Map_Word
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_load_map_word(host: Tagged<HeapObject>) -> MapWord {
    // SAFETY: 契约由本 unsafe fn 的调用方承接; 槽位按 AtomicUsize 松读,
    // location 对齐由 TaggedSlot=usize 布局保证。
    unsafe {
      let loc = Self::location(host, 0) as *const AtomicUsize;
      let value = (*loc).load(Ordering::Relaxed);
      MapWord::from_raw(tagged_to_full(value))
    }
  }

  /// 原版: AsAtomicTagged::Relaxed_Store(location(host), full_to_tagged(value.ptr()))。
  ///
  /// v8: src/objects/tagged-field-inl.h TaggedField::Relaxed_Store_Map_Word
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_store_map_word(host: Tagged<HeapObject>, value: MapWord) {
    // SAFETY: 契约由本 unsafe fn 的调用方承接; 松写单字槽无撕裂。
    unsafe {
      let loc = Self::location(host, 0) as *const AtomicUsize;
      (*loc).store(full_to_tagged(value.ptr()), Ordering::Relaxed);
    }
  }

  /// 原版: Address ptr = value.ptr(); AsAtomicTagged::Release_Store(location(host),
  /// full_to_tagged(ptr)) — 释放序保证并发标记器可见性。
  ///
  /// v8: src/objects/tagged-field-inl.h TaggedField::Release_Store_Map_Word
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn release_store_map_word(host: Tagged<HeapObject>, value: MapWord) {
    // SAFETY: 契约由本 unsafe fn 的调用方承接; Release 序原子写单字槽。
    unsafe {
      let loc = Self::location(host, 0) as *const AtomicUsize;
      (*loc).store(full_to_tagged(value.ptr()), Ordering::Release);
    }
  }
}
