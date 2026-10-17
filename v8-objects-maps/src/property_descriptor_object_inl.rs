// 由 tool/src/objects/property-descriptor-object-inl.h.js 从 v8/src/objects/property-descriptor-object-inl.h 生成; 修订就地改, 复跑覆盖
// PropertyDescriptorObject inline 存取器 + 构造器 the_hole 初值约定
//! PropertyDescriptorObject inline 面: 4 槽 (flags Smi / value / get / set)。
//! 初值约定: flags = Smi 0; value/get/set = the_hole (与 descriptor-object 的位标志面互补)。
//!
//! 布局 (property-descriptor-object.h: Struct 继承 HeapObject 1 槽 + 4 字段 = 5 槽);
//! 写屏障模式随 store 透传 (本层不驱动 GC 屏障, 与 crate 既有 TaggedMember.store 一致)。

use std::{
  mem::{offset_of, size_of},
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_core::{
  heap_object::HeapObject,
  objects::WriteBarrierMode,
  smi::Smi,
  tagged::{Object, Tagged},
};

/// flags: int 经 Smi 装箱存取 (Is*/Has* 位标志, 定义面见 property-descriptor-object.h)。
pub const LOAD_ACCESSORS: usize = 4;
/// value/get/set/flags 写槽数。
pub const SET_ACCESSORS: usize = 4;
/// 含 UnionOf<FunctionTemplateInfo,...> 的槽数 (get/set 两槽)。
pub const UNION_FT_SLOTS: usize = 2;

// ── PropertyDescriptorObjectFlags 位段常量 (对齐 property-descriptor-object.h:19-55) ──
pub const IS_ENUMERABLE_BIT_SHIFT: u32 = 0;
pub const HAS_ENUMERABLE_BIT_SHIFT: u32 = 1;
pub const IS_CONFIGURABLE_BIT_SHIFT: u32 = 2;
pub const HAS_CONFIGURABLE_BIT_SHIFT: u32 = 3;
pub const IS_WRITABLE_BIT_SHIFT: u32 = 4;
pub const HAS_WRITABLE_BIT_SHIFT: u32 = 5;
pub const HAS_VALUE_BIT_SHIFT: u32 = 6;
pub const HAS_GET_BIT_SHIFT: u32 = 7;
pub const HAS_SET_BIT_SHIFT: u32 = 8;
pub const K_FLAG_COUNT: usize = 9;

pub const K_REGULAR_ACCESSOR_PROPERTY_BITS: i32 = (1 << HAS_ENUMERABLE_BIT_SHIFT)
  | (1 << HAS_CONFIGURABLE_BIT_SHIFT)
  | (1 << HAS_GET_BIT_SHIFT)
  | (1 << HAS_SET_BIT_SHIFT);

pub const K_REGULAR_DATA_PROPERTY_BITS: i32 = (1 << HAS_ENUMERABLE_BIT_SHIFT)
  | (1 << HAS_CONFIGURABLE_BIT_SHIFT)
  | (1 << HAS_WRITABLE_BIT_SHIFT)
  | (1 << HAS_VALUE_BIT_SHIFT);

pub const K_HAS_MASK: i32 = (1 << HAS_ENUMERABLE_BIT_SHIFT)
  | (1 << HAS_CONFIGURABLE_BIT_SHIFT)
  | (1 << HAS_WRITABLE_BIT_SHIFT)
  | (1 << HAS_VALUE_BIT_SHIFT)
  | (1 << HAS_GET_BIT_SHIFT)
  | (1 << HAS_SET_BIT_SHIFT);

/// PropertyDescriptorObject 头布局 (Struct 继承 HeapObject 1 槽 + 4 数据槽 = 5 槽)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PropertyDescriptorObject {
  /// HeapObject 头 (map 槽; static_assert(sizeof(Struct) == sizeof(HeapObject)))。
  pub header: HeapObject,
  /// TaggedMember<Smi> flags_ (SmiTagged<PropertyDescriptorObjectFlags>)。
  pub flags: Tagged<Smi>,
  /// TaggedMember<UnionOf<JSAny, TheHole>> value_。
  pub value: Tagged<Object>,
  /// TaggedMember<UnionOf<FunctionTemplateInfo, JSAny, TheHole>> get_。
  pub get: Tagged<Object>,
  /// TaggedMember<UnionOf<FunctionTemplateInfo, JSAny, TheHole>> set_。
  pub set: Tagged<Object>,
}

impl PropertyDescriptorObject {
  /// 定长头尺寸 (含 4 数据槽)。
  pub const K_FIELD_COUNT: usize = 4;

  pub const FLAGS_OFFSET: usize = offset_of!(Self, flags);
  pub const VALUE_OFFSET: usize = offset_of!(Self, value);
  pub const GET_OFFSET: usize = offset_of!(Self, get);
  pub const SET_OFFSET: usize = offset_of!(Self, set);

  /// flags 位标志读 (原版 int flags() const { return flags_.load().value(); })。
  #[inline]
  #[must_use]
  pub fn flags(&self) -> i32 {
    Smi::value(self.flags)
  }

  /// flags 位标志写 (原版 void set_flags(int value) { flags_.store(this, Smi::FromInt(value)); })。
  #[inline]
  pub fn set_flags(&mut self, value: i32) {
    self.flags = Smi::from_int(value);
  }

  /// value: UnionOf<JSAny, TheHole> — the_hole 表示未设置 (原版 value_.load())。
  #[inline]
  #[must_use]
  pub fn value(&self) -> Tagged<Object> {
    self.value
  }

  /// set_value(value, mode) — 普通 store, 写屏障模式透传。
  #[inline]
  pub fn set_value(&mut self, value: Tagged<Object>, _mode: WriteBarrierMode) {
    self.value = value;
  }

  /// get: UnionOf<FunctionTemplateInfo, JSAny, TheHole> (原版 get_.load())。
  #[inline]
  #[must_use]
  pub fn get(&self) -> Tagged<Object> {
    self.get
  }

  /// set_get(value, mode)。
  #[inline]
  pub fn set_get(&mut self, value: Tagged<Object>, _mode: WriteBarrierMode) {
    self.get = value;
  }

  /// set: UnionOf<FunctionTemplateInfo, JSAny, TheHole> (原版 set_.load())。
  #[inline]
  #[must_use]
  pub fn set(&self) -> Tagged<Object> {
    self.set
  }

  /// set_set(value, mode)。
  #[inline]
  pub fn set_set(&mut self, value: Tagged<Object>, _mode: WriteBarrierMode) {
    self.set = value;
  }

  #[inline]
  #[must_use]
  pub fn is_enumerable(&self) -> bool {
    (self.flags() & (1 << IS_ENUMERABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  #[must_use]
  pub fn has_enumerable(&self) -> bool {
    (self.flags() & (1 << HAS_ENUMERABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  #[must_use]
  pub fn is_configurable(&self) -> bool {
    (self.flags() & (1 << IS_CONFIGURABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  #[must_use]
  pub fn has_configurable(&self) -> bool {
    (self.flags() & (1 << HAS_CONFIGURABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  #[must_use]
  pub fn is_writable(&self) -> bool {
    (self.flags() & (1 << IS_WRITABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  #[must_use]
  pub fn has_writable(&self) -> bool {
    (self.flags() & (1 << HAS_WRITABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  #[must_use]
  pub fn has_value(&self) -> bool {
    (self.flags() & (1 << HAS_VALUE_BIT_SHIFT)) != 0
  }

  #[inline]
  #[must_use]
  pub fn has_get(&self) -> bool {
    (self.flags() & (1 << HAS_GET_BIT_SHIFT)) != 0
  }

  #[inline]
  #[must_use]
  pub fn has_set(&self) -> bool {
    (self.flags() & (1 << HAS_SET_BIT_SHIFT)) != 0
  }

  #[inline]
  #[must_use]
  pub fn is_regular_accessor_property(&self) -> bool {
    (self.flags() & K_REGULAR_ACCESSOR_PROPERTY_BITS) == K_REGULAR_ACCESSOR_PROPERTY_BITS
  }

  #[inline]
  #[must_use]
  pub fn is_regular_data_property(&self) -> bool {
    (self.flags() & K_REGULAR_DATA_PROPERTY_BITS) == K_REGULAR_DATA_PROPERTY_BITS
  }

  /// v8: property-descriptor-object.tq:22 — IsDataDescriptor
  #[inline]
  #[must_use]
  pub fn is_data_descriptor(&self) -> bool {
    self.has_value() || self.has_writable()
  }

  /// v8: property-descriptor-object.tq:26 — IsAccessorDescriptor
  #[inline]
  #[must_use]
  pub fn is_accessor_descriptor(&self) -> bool {
    self.has_get() || self.has_set()
  }

  /// v8: property-descriptor-object.tq:30 — IsGenericDescriptor
  #[inline]
  #[must_use]
  pub fn is_generic_descriptor(&self) -> bool {
    !self.is_data_descriptor() && !self.is_accessor_descriptor()
  }

  /// 原版构造器: flags=Smi::zero(), value/get/set=the_hole_value()。
  ///
  /// v8: property-descriptor-object-inl.h:23
  pub fn new(_witness: &usize) -> Self {
    Self {
      header: HeapObject { map: Tagged::NULL },
      flags: Smi::from_int(0),
      value: Tagged::NULL,
      get: Tagged::NULL,
      set: Tagged::NULL,
    }
  }

  /// 内部头尺寸断言辅助 (snapshot 布局锚点)。
  #[inline]
  #[must_use]
  pub const fn slot_count() -> usize {
    // Struct 继承 (HeapObject 1 槽) + 4 数据槽 = 5 槽。
    1 + Self::K_FIELD_COUNT
  }
}

/// 对 `Tagged<PropertyDescriptorObject>` 的堆上字段存取。
pub trait PropertyDescriptorObjectExt {
  fn flags(&self) -> i32;
  fn set_flags(&self, value: i32);
  fn value(&self) -> Tagged<Object>;
  fn set_value(&self, value: Tagged<Object>, mode: WriteBarrierMode);
  fn get(&self) -> Tagged<Object>;
  fn set_get(&self, value: Tagged<Object>, mode: WriteBarrierMode);
  fn get_set(&self) -> Tagged<Object>;
  fn set_set(&self, value: Tagged<Object>, mode: WriteBarrierMode);

  /// v8: property-descriptor-object.h:70 — set() const
  #[inline]
  fn set(&self) -> Tagged<Object> {
    self.get_set()
  }

  #[inline]
  fn is_enumerable(&self) -> bool {
    (self.flags() & (1 << IS_ENUMERABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  fn has_enumerable(&self) -> bool {
    (self.flags() & (1 << HAS_ENUMERABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  fn is_configurable(&self) -> bool {
    (self.flags() & (1 << IS_CONFIGURABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  fn has_configurable(&self) -> bool {
    (self.flags() & (1 << HAS_CONFIGURABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  fn is_writable(&self) -> bool {
    (self.flags() & (1 << IS_WRITABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  fn has_writable(&self) -> bool {
    (self.flags() & (1 << HAS_WRITABLE_BIT_SHIFT)) != 0
  }

  #[inline]
  fn has_value(&self) -> bool {
    (self.flags() & (1 << HAS_VALUE_BIT_SHIFT)) != 0
  }

  #[inline]
  fn has_get(&self) -> bool {
    (self.flags() & (1 << HAS_GET_BIT_SHIFT)) != 0
  }

  #[inline]
  fn has_set(&self) -> bool {
    (self.flags() & (1 << HAS_SET_BIT_SHIFT)) != 0
  }

  #[inline]
  fn is_regular_accessor_property(&self) -> bool {
    (self.flags() & K_REGULAR_ACCESSOR_PROPERTY_BITS) == K_REGULAR_ACCESSOR_PROPERTY_BITS
  }

  #[inline]
  fn is_regular_data_property(&self) -> bool {
    (self.flags() & K_REGULAR_DATA_PROPERTY_BITS) == K_REGULAR_DATA_PROPERTY_BITS
  }

  #[inline]
  fn is_data_descriptor(&self) -> bool {
    self.has_value() || self.has_writable()
  }

  #[inline]
  fn is_accessor_descriptor(&self) -> bool {
    self.has_get() || self.has_set()
  }

  #[inline]
  fn is_generic_descriptor(&self) -> bool {
    !self.is_data_descriptor() && !self.is_accessor_descriptor()
  }
}

impl PropertyDescriptorObjectExt for Tagged<PropertyDescriptorObject> {
  #[inline]
  fn flags(&self) -> i32 {
    let addr = self.address() + PropertyDescriptorObject::FLAGS_OFFSET;
    // SAFETY: addr 落在有效对象的 flags 槽
    let raw = unsafe { AtomicUsize::from_ptr(addr as *mut usize).load(Ordering::Relaxed) };
    Smi::value(Tagged::new(raw))
  }

  #[inline]
  fn set_flags(&self, value: i32) {
    let addr = self.address() + PropertyDescriptorObject::FLAGS_OFFSET;
    let smi = Smi::from_int(value);
    // SAFETY: addr 落在有效对象的 flags 槽
    unsafe { AtomicUsize::from_ptr(addr as *mut usize).store(smi.ptr(), Ordering::Relaxed) };
  }

  #[inline]
  fn value(&self) -> Tagged<Object> {
    let addr = self.address() + PropertyDescriptorObject::VALUE_OFFSET;
    let raw = unsafe { AtomicUsize::from_ptr(addr as *mut usize).load(Ordering::Relaxed) };
    Tagged::new(raw)
  }

  #[inline]
  fn set_value(&self, value: Tagged<Object>, _mode: WriteBarrierMode) {
    let addr = self.address() + PropertyDescriptorObject::VALUE_OFFSET;
    unsafe { AtomicUsize::from_ptr(addr as *mut usize).store(value.ptr(), Ordering::Relaxed) };
  }

  #[inline]
  fn get(&self) -> Tagged<Object> {
    let addr = self.address() + PropertyDescriptorObject::GET_OFFSET;
    let raw = unsafe { AtomicUsize::from_ptr(addr as *mut usize).load(Ordering::Relaxed) };
    Tagged::new(raw)
  }

  #[inline]
  fn set_get(&self, value: Tagged<Object>, _mode: WriteBarrierMode) {
    let addr = self.address() + PropertyDescriptorObject::GET_OFFSET;
    unsafe { AtomicUsize::from_ptr(addr as *mut usize).store(value.ptr(), Ordering::Relaxed) };
  }

  #[inline]
  fn get_set(&self) -> Tagged<Object> {
    let addr = self.address() + PropertyDescriptorObject::SET_OFFSET;
    let raw = unsafe { AtomicUsize::from_ptr(addr as *mut usize).load(Ordering::Relaxed) };
    Tagged::new(raw)
  }

  #[inline]
  fn set_set(&self, value: Tagged<Object>, _mode: WriteBarrierMode) {
    let addr = self.address() + PropertyDescriptorObject::SET_OFFSET;
    unsafe { AtomicUsize::from_ptr(addr as *mut usize).store(value.ptr(), Ordering::Relaxed) };
  }
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(LOAD_ACCESSORS == 4);
  assert!(SET_ACCESSORS == 4);
  assert!(UNION_FT_SLOTS == 2);
  assert!(K_FLAG_COUNT == 9);
  assert!(PropertyDescriptorObject::slot_count() == 5);
  // Struct 继承 HeapObject 1 槽 + 4 数据槽 = 5 槽。
  assert!(size_of::<PropertyDescriptorObject>() == 5 * size_of::<usize>());
  assert!(PropertyDescriptorObject::FLAGS_OFFSET == size_of::<HeapObject>());
  assert!(
    PropertyDescriptorObject::VALUE_OFFSET
      == PropertyDescriptorObject::FLAGS_OFFSET + size_of::<usize>()
  );
  assert!(
    PropertyDescriptorObject::GET_OFFSET
      == PropertyDescriptorObject::VALUE_OFFSET + size_of::<usize>()
  );
  assert!(
    PropertyDescriptorObject::SET_OFFSET
      == PropertyDescriptorObject::GET_OFFSET + size_of::<usize>()
  );

  // 金标准字面量槽序与位段掩码锚定
  assert!(PropertyDescriptorObject::FLAGS_OFFSET == size_of::<usize>());
  assert!(PropertyDescriptorObject::VALUE_OFFSET == 2 * size_of::<usize>());
  assert!(PropertyDescriptorObject::GET_OFFSET == 3 * size_of::<usize>());
  assert!(PropertyDescriptorObject::SET_OFFSET == 4 * size_of::<usize>());
  assert!(K_REGULAR_ACCESSOR_PROPERTY_BITS == 0x18a);
  assert!(K_REGULAR_DATA_PROPERTY_BITS == 0x6a);
  assert!(K_HAS_MASK == 0x1ea);
};
