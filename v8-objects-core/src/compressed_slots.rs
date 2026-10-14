// 由 tool/src/objects/compressed-slots.h.js 从 v8/src/objects/compressed-slots.h + compressed-slots-inl.h 生成; 修订就地改, 复跑覆盖
// 钉死配置: V8_COMPRESS_POINTERS 开 → 本模块为槽位运行时主承载 (TData = Tagged_t = u32)

//! 压缩槽位族 (原版 compressed-slots.{h,-inl.h})
//!
//! 钉死指针压缩配置 (`v8_enable_pointer_compression=true`, todo.md 五 构建配置钉死):
//! `TData = Tagged_t = u32` (v8-common globals.rs:43), 槽存裸压缩值, 读经
//! `V8HeapCompressionScheme::DecompressTagged` (base|零扩展), 写经
//! `CompressObject` (截断低 32 位, 含 cage 落位 DCHECK)。
//! 写屏障 (`WriteBarrier::ForValue`) 与 GC 通知归 v8-heap-gc, 本层 store
//! 只做裸落槽 (与原版 `CompressedObjectSlot::store` 一致 — 原版 store 本身
//! 亦不含写屏障, 屏障在调用方 HeapObject::UpdateField)。
//!
//! 槽地址不变: 构造后即 `Address` (槽的裸地址), 步长 `kTaggedSize` (=4)。

use std::{
  marker::PhantomData,
  sync::atomic::{AtomicU32, Ordering},
};

use v8_common::{K_TAGGED_SIZE, PtrComprCageBase, V8HeapCompressionScheme};

use crate::{
  heap_object::HeapObject,
  map::Map,
  tagged::{HeapObjectReference, MaybeObject, Object, Tagged},
  tagged_constants::{HEAP_OBJECT_TAG, HEAP_OBJECT_TAG_MASK},
};

/// 槽位数据大小 (原版 kSlotDataSize = sizeof(Tagged_t))
pub const K_SLOT_DATA_SIZE: usize = K_TAGGED_SIZE;

/// 空槽地址哨兵 (原版 kNullAddress)
pub const K_NULL_ADDRESS: usize = 0;

/// 压缩槽基类 (原版 slots.h `SlotBase<TData, kSlotDataSize>` 公共算术面)
///
/// Rust 侧不做 CRTP, 各具体槽类内含 `address`; 步长 `kTaggedSize` 的
/// 指针算术以显式方法承接原版 16 个运算符 (1:1 语义登记于 slots.rs)。
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

  /// 原版 location(): 槽地址 (压缩配置下指向 Tagged_t=u32 字)
  #[inline]
  pub const fn location(&self) -> usize {
    self.address
  }

  /// 原版 operator++ / --: ptr ± kSlotDataSize
  #[inline]
  pub const fn step(self, n: isize) -> Self {
    Self::new(
      self
        .address
        .wrapping_add_signed(n * K_SLOT_DATA_SIZE as isize),
    )
  }

  /// 原版 operator-(a, b): 元素距离 = (ptr 差) / kSlotDataSize
  #[inline]
  pub const fn distance(self, other: Self) -> isize {
    (self.address as isize - other.address as isize) / K_SLOT_DATA_SIZE as isize
  }
}

/// 裸读写原子适配 (原版 AsAtomicTagged::* 在 Tagged_t=u32 槽上)
///
/// # Safety
/// `address` 必须指向进程内有效且 4 字节对齐的 `Tagged_t` 字。
#[inline]
unsafe fn as_atomic(address: usize) -> &'static AtomicU32 {
  // SAFETY: 调用方契约 — address 指向有效对齐的 u32 原子槽 (V8 堆内 Tagged_t
  // 字均满足); 取 'static 视图仅借用底层原子对象, 生命周期由堆保证。
  unsafe { &*(address as *const AtomicU32) }
}

/// 压缩对象槽 (原版 CompressedObjectSlot: Tagged<Object>, kCanBeWeak=false)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CompressedObjectSlot {
  base: SlotBase,
}

impl CompressedObjectSlot {
  pub const K_CAN_BE_WEAK: bool = false;

  #[inline]
  pub const fn new(address: usize) -> Self {
    Self {
      base: SlotBase::new(address),
    }
  }

  /// 从堆内强引用 tagged 字构造 (原版 CompressedObjectSlot(Tagged<Object>*):
  /// 取 `&object->ptr_` 的裸地址)
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

  /// 原版 operator* / load (compressed-slots-inl.h:39-55): 读裸值后解压。
  ///
  /// # Safety
  /// 槽地址须有效对齐且指向已初始化的 Tagged_t。
  #[inline]
  pub unsafe fn load(&self) -> Tagged<Object> {
    // SAFETY: 契约由调用方承接; 裸读单 u32。
    let raw = unsafe { *(self.location() as *const u32) };
    Tagged::new(V8HeapCompressionScheme::decompress_tagged(raw))
  }

  /// 原版 load(cage_base): 经给定 cage 解压 (非全局主 cage 场景)。
  ///
  /// # Safety
  /// 同 [`Self::load`], 且 `cage_base` 须为槽所属 cage。
  #[inline]
  pub unsafe fn load_with_cage(&self, cage_base: PtrComprCageBase) -> Tagged<Object> {
    // SAFETY: 同 load。
    let raw = unsafe { *(self.location() as *const u32) };
    Tagged::new(cage_base.address() | raw as usize)
  }

  /// 原版 store (compressed-slots-inl.h:57-59): CompressObject 后裸落槽 (无写屏障)。
  ///
  /// # Safety
  /// 槽地址须有效对齐可写; 写屏障/可见性由调用方 (heap 层) 负责。
  #[inline]
  pub unsafe fn store(&self, value: Tagged<Object>) {
    let raw = V8HeapCompressionScheme::compress_object(value.ptr());
    // SAFETY: 契约由调用方承接。
    unsafe { *(self.location() as *mut u32) = raw };
  }

  /// 原版 store_map (compressed-slots-inl.h:61-65): 压缩配置不支持 map packing, 直转发 store。
  ///
  /// # Safety
  /// 同 [`Self::store`]。
  #[inline]
  pub unsafe fn store_map(&self, map: Tagged<Map>) {
    // SAFETY: 转发同语义 store。
    unsafe { self.store(map.cast()) };
  }

  /// 原版 load_map (compressed-slots-inl.h:67-71): UncheckedCast<Map>(Relaxed_Load())。
  ///
  /// # Safety
  /// 同 [`Self::relaxed_load`]。
  #[inline]
  pub unsafe fn load_map(&self) -> Tagged<Map> {
    // SAFETY: 转发同语义 relaxed_load。
    unsafe { self.relaxed_load().cast() }
  }

  /// 原版 Relaxed_Load (compressed-slots-inl.h:79-82): 原子松读后解压。
  ///
  /// # Safety
  /// 槽地址须有效对齐的原子 Tagged_t。
  #[inline]
  pub unsafe fn relaxed_load(&self) -> Tagged<Object> {
    // SAFETY: 契约由调用方承接。
    let raw = unsafe { as_atomic(self.location()).load(Ordering::Relaxed) };
    Tagged::new(V8HeapCompressionScheme::decompress_tagged(raw))
  }

  /// 原版 Relaxed_Load(cage_base)。
  ///
  /// # Safety
  /// 同 [`Self::relaxed_load`]。
  #[inline]
  pub unsafe fn relaxed_load_with_cage(&self, cage_base: PtrComprCageBase) -> Tagged<Object> {
    // SAFETY: 契约由调用方承接。
    let raw = unsafe { as_atomic(self.location()).load(Ordering::Relaxed) };
    Tagged::new(cage_base.address() | raw as usize)
  }

  /// 原版 Acquire_Load (compressed-slots-inl.h:74-77)。
  ///
  /// # Safety
  /// 槽地址须有效对齐的原子 Tagged_t。
  #[inline]
  pub unsafe fn acquire_load(&self) -> Tagged<Object> {
    // SAFETY: 契约由调用方承接。
    let raw = unsafe { as_atomic(self.location()).load(Ordering::Acquire) };
    Tagged::new(V8HeapCompressionScheme::decompress_tagged(raw))
  }

  /// 原版 Relaxed_Load_Raw (compressed-slots-inl.h:90-92): 松读裸 Tagged_t 不解压。
  ///
  /// # Safety
  /// 槽地址须有效对齐的原子 Tagged_t。
  #[inline]
  pub unsafe fn relaxed_load_raw(&self) -> u32 {
    // SAFETY: 契约由调用方承接。
    unsafe { as_atomic(self.location()).load(Ordering::Relaxed) }
  }

  /// 原版 RawToTagged(cage_base, raw) (compressed-slots-inl.h:94-97): 裸值经 cage 解压为 Object。
  #[inline]
  pub fn raw_to_tagged(cage_base: PtrComprCageBase, raw: u32) -> Tagged<Object> {
    Tagged::new(cage_base.address() | raw as usize)
  }

  /// 原版 Relaxed_Store (compressed-slots-inl.h:99-102): CompressObject 后原子松写。
  ///
  /// # Safety
  /// 槽地址须有效对齐可写的原子 Tagged_t。
  #[inline]
  pub unsafe fn relaxed_store(&self, value: Tagged<Object>) {
    let raw = V8HeapCompressionScheme::compress_object(value.ptr());
    // SAFETY: 契约由调用方承接。
    unsafe { as_atomic(self.location()).store(raw, Ordering::Relaxed) };
  }

  /// 原版 Release_Store (compressed-slots-inl.h:104-107)。
  ///
  /// # Safety
  /// 同 [`Self::relaxed_store`]。
  #[inline]
  pub unsafe fn release_store(&self, value: Tagged<Object>) {
    let raw = V8HeapCompressionScheme::compress_object(value.ptr());
    // SAFETY: 契约由调用方承接。
    unsafe { as_atomic(self.location()).store(raw, Ordering::Release) };
  }

  /// 原版 Release_CompareAndSwap (compressed-slots-inl.h:109-116):
  /// 双侧 CompressObject, 释放序 CAS, 返回实际值解压。
  ///
  /// # Safety
  /// 槽地址须有效对齐可写的原子 Tagged_t。
  #[inline]
  pub unsafe fn release_compare_and_swap(
    &self,
    old: Tagged<Object>,
    target: Tagged<Object>,
  ) -> Tagged<Object> {
    let old_raw = V8HeapCompressionScheme::compress_object(old.ptr());
    let target_raw = V8HeapCompressionScheme::compress_object(target.ptr());
    // SAFETY: 契约由调用方承接。
    let result = unsafe {
      as_atomic(self.location()).compare_exchange(
        old_raw,
        target_raw,
        Ordering::Release,
        Ordering::Relaxed,
      )
    };
    Tagged::new(V8HeapCompressionScheme::decompress_tagged(
      result.unwrap_or_else(|actual| actual),
    ))
  }

  /// 原版 contains_map_value (compressed-slots-inl.h:26-32): 裸读低 32 位比 map 值。
  ///
  /// # Safety
  /// 槽地址须有效对齐。
  #[inline]
  pub unsafe fn contains_map_value(&self, raw_value: usize) -> bool {
    // 原版 DCHECK(!V8_MAP_PACKING_BOOL): 压缩配置无 map packing。
    // SAFETY: 契约由调用方承接。
    let value = unsafe { *(self.location() as *const u32) };
    value == raw_value as u32
  }

  /// 原版 Relaxed_ContainsMapValue (compressed-slots-inl.h:34-40): 原子松读低 32 位比 map 值。
  ///
  /// # Safety
  /// 槽地址须有效对齐的原子 Tagged_t。
  #[inline]
  pub unsafe fn relaxed_contains_map_value(&self, raw_value: usize) -> bool {
    // SAFETY: 契约由调用方承接。
    let value = unsafe { as_atomic(self.location()).load(Ordering::Relaxed) };
    value == raw_value as u32
  }
}

/// 压缩可弱引用槽 (原版 CompressedMaybeObjectSlot: Tagged<MaybeObject>, kCanBeWeak=true)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CompressedMaybeObjectSlot {
  base: SlotBase,
}

impl CompressedMaybeObjectSlot {
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
  pub fn from_object_value(value: &Tagged<Object>) -> Self {
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

  /// 原版 operator* / load (compressed-slots-inl.h:121-135)。
  ///
  /// # Safety
  /// 槽地址须有效对齐的 Tagged_t。
  #[inline]
  pub unsafe fn load(&self) -> Tagged<MaybeObject> {
    // SAFETY: 契约由调用方承接。
    let raw = unsafe { *(self.location() as *const u32) };
    Tagged::new(V8HeapCompressionScheme::decompress_tagged(raw))
  }

  /// 原版 load(cage_base)。
  ///
  /// # Safety
  /// 同 [`Self::load`]。
  #[inline]
  pub unsafe fn load_with_cage(&self, cage_base: PtrComprCageBase) -> Tagged<MaybeObject> {
    // SAFETY: 契约由调用方承接。
    let raw = unsafe { *(self.location() as *const u32) };
    Tagged::new(cage_base.address() | raw as usize)
  }

  /// 原版 store (compressed-slots-inl.h:137-139): CompressAny 截断落槽 (强弱皆可)。
  ///
  /// # Safety
  /// 槽地址须有效对齐可写。
  #[inline]
  pub unsafe fn store(&self, value: Tagged<MaybeObject>) {
    // CompressAny: 压缩弱引用不做 cage DCHECK (清除值可在任意 cage 外)。
    let raw = V8HeapCompressionScheme::compress_any(value.ptr());
    // SAFETY: 契约由调用方承接。
    unsafe { *(self.location() as *mut u32) = raw };
  }

  /// 原版 Relaxed_Load (compressed-slots-inl.h:141-145)。
  ///
  /// # Safety
  /// 槽地址须有效对齐的原子 Tagged_t。
  #[inline]
  pub unsafe fn relaxed_load(&self) -> Tagged<MaybeObject> {
    // SAFETY: 契约由调用方承接。
    let raw = unsafe { as_atomic(self.location()).load(Ordering::Relaxed) };
    Tagged::new(V8HeapCompressionScheme::decompress_tagged(raw))
  }

  /// 原版 Relaxed_Load(cage_base)。
  ///
  /// # Safety
  /// 同 [`Self::relaxed_load`]。
  #[inline]
  pub unsafe fn relaxed_load_with_cage(&self, cage_base: PtrComprCageBase) -> Tagged<MaybeObject> {
    // SAFETY: 契约由调用方承接。
    let raw = unsafe { as_atomic(self.location()).load(Ordering::Relaxed) };
    Tagged::new(cage_base.address() | raw as usize)
  }

  /// 原版 Relaxed_Load_Raw (compressed-slots-inl.h:147-149)。
  ///
  /// # Safety
  /// 槽地址须有效对齐的原子 Tagged_t。
  #[inline]
  pub unsafe fn relaxed_load_raw(&self) -> u32 {
    // SAFETY: 契约由调用方承接。
    unsafe { as_atomic(self.location()).load(Ordering::Relaxed) }
  }

  /// 原版 RawToTagged(cage_base, raw) (compressed-slots-inl.h:151-155)。
  #[inline]
  pub fn raw_to_tagged(cage_base: PtrComprCageBase, raw: u32) -> Tagged<MaybeObject> {
    Tagged::new(cage_base.address() | raw as usize)
  }

  /// 原版 Relaxed_Store (compressed-slots-inl.h:157-160)。
  ///
  /// # Safety
  /// 槽地址须有效对齐可写的原子 Tagged_t。
  #[inline]
  pub unsafe fn relaxed_store(&self, value: Tagged<MaybeObject>) {
    let raw = V8HeapCompressionScheme::compress_any(value.ptr());
    // SAFETY: 契约由调用方承接。
    unsafe { as_atomic(self.location()).store(raw, Ordering::Relaxed) };
  }

  /// 原版 Release_CompareAndSwap (compressed-slots-inl.h:162-171): 返回实际裸值。
  ///
  /// # Safety
  /// 槽地址须有效对齐可写的原子 Tagged_t。
  #[inline]
  pub unsafe fn release_compare_and_swap(&self, old: u32, target: u32) -> u32 {
    // SAFETY: 契约由调用方承接。
    unsafe {
      as_atomic(self.location())
        .compare_exchange(old, target, Ordering::Release, Ordering::Relaxed)
        .unwrap_or_else(|actual| actual)
    }
  }

  /// 原版 Release_CompareAndSwap(Tagged<MaybeObject>, Tagged<MaybeObject>) (compressed-slots-inl.h:168-173)
  ///
  /// # Safety
  /// 槽地址须有效对齐可写的原子 Tagged_t。
  #[inline]
  pub unsafe fn release_compare_and_swap_tagged(
    &self,
    old: Tagged<MaybeObject>,
    target: Tagged<MaybeObject>,
  ) -> Tagged<MaybeObject> {
    let old_raw = V8HeapCompressionScheme::compress_any(old.ptr());
    let target_raw = V8HeapCompressionScheme::compress_any(target.ptr());
    let res = unsafe { self.release_compare_and_swap(old_raw, target_raw) };
    Tagged::new(V8HeapCompressionScheme::decompress_tagged(res))
  }
}

/// 压缩堆对象引用槽 (原版 CompressedHeapObjectSlot: Tagged<HeapObjectReference>)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CompressedHeapObjectSlot {
  base: SlotBase,
}

impl CompressedHeapObjectSlot {
  pub const K_CAN_BE_WEAK: bool = true;

  #[inline]
  pub const fn new(address: usize) -> Self {
    Self {
      base: SlotBase::new(address),
    }
  }

  #[inline]
  pub fn from_value(value: &Tagged<HeapObjectReference>) -> Self {
    Self::new(value as *const Tagged<HeapObjectReference> as usize)
  }

  #[inline]
  pub fn from_heap_object(value: &Tagged<HeapObject>) -> Self {
    Self::new(value as *const Tagged<HeapObject> as usize)
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

  /// 原版 operator* (compressed-slots-inl.h:177-181)。
  ///
  /// # Safety
  /// 槽地址须有效对齐的 Tagged_t。
  #[inline]
  pub unsafe fn load(&self) -> Tagged<HeapObjectReference> {
    // SAFETY: 契约由调用方承接。
    let raw = unsafe { *(self.location() as *const u32) };
    Tagged::new(V8HeapCompressionScheme::decompress_tagged(raw))
  }

  /// 原版 load(cage_base)。
  ///
  /// # Safety
  /// 同 [`Self::load`]。
  #[inline]
  pub unsafe fn load_with_cage(&self, cage_base: PtrComprCageBase) -> Tagged<HeapObjectReference> {
    // SAFETY: 契约由调用方承接。
    let raw = unsafe { *(self.location() as *const u32) };
    Tagged::new(cage_base.address() | raw as usize)
  }

  /// 原版 store (compressed-slots-inl.h:185-187)。
  ///
  /// # Safety
  /// 槽地址须有效对齐可写。
  #[inline]
  pub unsafe fn store(&self, value: Tagged<HeapObjectReference>) {
    let raw = V8HeapCompressionScheme::compress_any(value.ptr());
    // SAFETY: 契约由调用方承接。
    unsafe { *(self.location() as *mut u32) = raw };
  }

  /// 原版 ToHeapObject (compressed-slots-inl.h:189-193):
  /// 断言强堆对象标记位后剥位为 Tagged<HeapObject>。
  ///
  /// # Safety
  /// 槽地址须有效对齐且内容为强堆对象引用。
  #[inline]
  pub unsafe fn to_heap_object(&self) -> Tagged<HeapObject> {
    // SAFETY: 契约由调用方承接。
    let raw = unsafe { *(self.location() as *const u32) };
    let ptr = V8HeapCompressionScheme::decompress_tagged(raw);
    debug_assert!(
      (ptr & HEAP_OBJECT_TAG_MASK) == HEAP_OBJECT_TAG,
      "ToHeapObject 要求强堆对象标记位"
    );
    Tagged::new(ptr)
  }

  /// 原版 StoreHeapObject (compressed-slots-inl.h:195-197): 强堆对象直存。
  ///
  /// # Safety
  /// 槽地址须有效对齐可写。
  #[inline]
  pub unsafe fn store_heap_object(&self, value: Tagged<HeapObject>) {
    // SAFETY: 转发同语义 store (强引用 cast 到引用标记)。
    unsafe { self.store(value.cast()) };
  }
}

/// 非堆压缩对象槽基面 (原版 OffHeapCompressedObjectSlotBase<CompressionScheme,
/// TObject, Subclass> 的方法族, 压缩方案按 cage 泛型单态化, 禁 dyn)。
///
/// 与堆内槽的差异 (原版注释): 不假设槽在堆上, 不提供隐式 Isolate 计算的
/// operator*; 其余读写语义一致。
macro_rules! offheap_slot_methods {
  ($ty:ty, $tagged:ty) => {
    impl<C: v8_common::CompressionCage> $ty {
      #[inline]
      pub const fn new(address: usize) -> Self {
        Self {
          base: SlotBase::new(address),
          _cage: std::marker::PhantomData,
        }
      }

      #[inline]
      pub const fn location(&self) -> usize {
        self.base.location()
      }

      /// 原版 load() (compressed-slots-inl.h:212-216)。
      ///
      /// # Safety
      /// 槽地址须有效对齐的 Tagged_t。
      #[inline]
      pub unsafe fn load(&self) -> $tagged {
        // SAFETY: 契约由调用方承接。
        let raw = unsafe { *(self.location() as *const u32) };
        <$tagged>::new(v8_common::V8HeapCompressionSchemeImpl::<C>::decompress_tagged(raw))
      }

      /// 原版 load(cage_base) (compressed-slots-inl.h:218-224)。
      ///
      /// # Safety
      /// 同 [`Self::load`]。
      #[inline]
      pub unsafe fn load_with_cage(&self, cage_base: PtrComprCageBase) -> $tagged {
        // SAFETY: 契约由调用方承接。
        let raw = unsafe { *(self.location() as *const u32) };
        <$tagged>::new(cage_base.address() | raw as usize)
      }

      /// 原版 store() (compressed-slots-inl.h:226-229)。
      ///
      /// # Safety
      /// 槽地址须有效对齐可写。
      #[inline]
      pub unsafe fn store(&self, value: $tagged) {
        let raw = v8_common::V8HeapCompressionSchemeImpl::<C>::compress_object(value.ptr());
        // SAFETY: 契约由调用方承接。
        unsafe { *(self.location() as *mut u32) = raw };
      }

      /// 原版 Relaxed_Load() (compressed-slots-inl.h:231-235)。
      ///
      /// # Safety
      /// 槽地址须有效对齐的原子 Tagged_t。
      #[inline]
      pub unsafe fn relaxed_load(&self) -> $tagged {
        // SAFETY: 契约由调用方承接。
        let raw = unsafe { as_atomic(self.location()).load(Ordering::Relaxed) };
        <$tagged>::new(v8_common::V8HeapCompressionSchemeImpl::<C>::decompress_tagged(raw))
      }

      /// 原版 Relaxed_Load(cage_base)。
      ///
      /// # Safety
      /// 同 [`Self::relaxed_load`]。
      #[inline]
      pub unsafe fn relaxed_load_with_cage(&self, cage_base: PtrComprCageBase) -> $tagged {
        // SAFETY: 契约由调用方承接。
        let raw = unsafe { as_atomic(self.location()).load(Ordering::Relaxed) };
        <$tagged>::new(cage_base.address() | raw as usize)
      }

      /// 原版 Relaxed_Load_Raw。
      ///
      /// # Safety
      /// 槽地址须有效对齐的原子 Tagged_t。
      #[inline]
      pub unsafe fn relaxed_load_raw(&self) -> u32 {
        // SAFETY: 契约由调用方承接。
        unsafe { as_atomic(self.location()).load(Ordering::Relaxed) }
      }

      /// 原版 RawToTagged(cage_base, raw)。
      #[inline]
      pub fn raw_to_tagged(cage_base: PtrComprCageBase, raw: u32) -> $tagged {
        <$tagged>::new(cage_base.address() | raw as usize)
      }

      /// 原版 Acquire_Load()。
      ///
      /// # Safety
      /// 槽地址须有效对齐的原子 Tagged_t。
      #[inline]
      pub unsafe fn acquire_load(&self) -> $tagged {
        // SAFETY: 契约由调用方承接。
        let raw = unsafe { as_atomic(self.location()).load(Ordering::Acquire) };
        <$tagged>::new(v8_common::V8HeapCompressionSchemeImpl::<C>::decompress_tagged(raw))
      }

      /// 原版 Acquire_Load(cage_base)。
      ///
      /// # Safety
      /// 同 [`Self::acquire_load`]。
      #[inline]
      pub unsafe fn acquire_load_with_cage(&self, cage_base: PtrComprCageBase) -> $tagged {
        // SAFETY: 契约由调用方承接。
        let raw = unsafe { as_atomic(self.location()).load(Ordering::Acquire) };
        <$tagged>::new(cage_base.address() | raw as usize)
      }

      /// 原版 Relaxed_Store。
      ///
      /// # Safety
      /// 槽地址须有效对齐可写的原子 Tagged_t。
      #[inline]
      pub unsafe fn relaxed_store(&self, value: $tagged) {
        let raw = v8_common::V8HeapCompressionSchemeImpl::<C>::compress_object(value.ptr());
        // SAFETY: 契约由调用方承接。
        unsafe { as_atomic(self.location()).store(raw, Ordering::Relaxed) };
      }

      /// 原版 Release_Store。
      ///
      /// # Safety
      /// 同 [`Self::relaxed_store`]。
      #[inline]
      pub unsafe fn release_store(&self, value: $tagged) {
        let raw = v8_common::V8HeapCompressionSchemeImpl::<C>::compress_object(value.ptr());
        // SAFETY: 契约由调用方承接。
        unsafe { as_atomic(self.location()).store(raw, Ordering::Release) };
      }

      /// 原版 Release_CompareAndSwap (非堆槽返回实际裸值, 调用方自析)。
      ///
      /// # Safety
      /// 槽地址须有效对齐可写的原子 Tagged_t。
      #[inline]
      pub unsafe fn release_compare_and_swap_raw(&self, old: u32, target: u32) -> u32 {
        // SAFETY: 契约由调用方承接。
        unsafe {
          as_atomic(self.location())
            .compare_exchange(old, target, Ordering::Release, Ordering::Relaxed)
            .unwrap_or_else(|actual| actual)
        }
      }
    }
  };
}

/// 非堆压缩对象槽 (原版 OffHeapCompressedObjectSlot<CompressionScheme>:
/// Tagged<Object>, kCanBeWeak=false)
pub struct OffHeapCompressedObjectSlot<C: v8_common::CompressionCage> {
  base: SlotBase,
  _cage: PhantomData<C>,
}

/// 非堆压缩可弱引用槽 (原版 OffHeapCompressedMaybeObjectSlot<CompressionScheme>:
/// Tagged<MaybeObject>, kCanBeWeak=true; 方法面与对象槽同形, 值类型可弱)
pub struct OffHeapCompressedMaybeObjectSlot<C: v8_common::CompressionCage> {
  base: SlotBase,
  _cage: PhantomData<C>,
}

offheap_slot_methods!(OffHeapCompressedObjectSlot<C>, Tagged<Object>);
offheap_slot_methods!(OffHeapCompressedMaybeObjectSlot<C>, Tagged<MaybeObject>);
