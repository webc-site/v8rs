// 由 tool/src/objects/turbofan-types-inl.h.js 从 v8/src/objects/turbofan-types-inl.h + turbofan-types.tq 生成; 修订就地改, 复跑覆盖
// 位集 32+4 段; 声明链在 turbofan_types.rs

//! TurboFan 类型内联面 (原版 turbofan-types-inl.h + turbofan-types.tq)
//!
//! 位集表示: 64 位拆两个 uint32 (Torque 64 位位集支持受限):
//! 低 32 位承载标量种类, 高 4 位承载 machine/hole 等新种类.

// ===== 位集段表 (tq bitfield struct 机械展开) =====
/// TurbofanTypeLowBits 段表 (32 段铺满 uint32)
pub const TURBOFAN_TYPE_LOW_BITS: [(&str, u32, u32); 32] = [
  // (段名, shift, width)
  ("_unused_padding_field_1", 0, 1),
  ("other_unsigned31", 1, 1),
  ("other_unsigned32", 2, 1),
  ("other_signed32", 3, 1),
  ("other_number", 4, 1),
  ("other_string", 5, 1),
  ("negative31", 6, 1),
  ("null", 7, 1),
  ("undefined", 8, 1),
  ("boolean", 9, 1),
  ("unsigned30", 10, 1),
  ("minus_zero", 11, 1),
  ("naN", 12, 1),
  ("symbol", 13, 1),
  ("internalized_string", 14, 1),
  ("other_callable", 15, 1),
  ("other_object", 16, 1),
  ("other_undetectable", 17, 1),
  ("callable_proxy", 18, 1),
  ("other_proxy", 19, 1),
  ("callable_function", 20, 1),
  ("class_constructor", 21, 1),
  ("bound_function", 22, 1),
  ("other_internal", 23, 1),
  ("external_pointer", 24, 1),
  ("array", 25, 1),
  ("unsigned_big_int_63", 26, 1),
  ("other_unsigned_big_int_64", 27, 1),
  ("negative_big_int_63", 28, 1),
  ("other_big_int", 29, 1),
  ("wasm_object", 30, 1),
  ("sandboxed_pointer", 31, 1),
];

/// TurbofanTypeHighBits 段表 (4 段)
pub const TURBOFAN_TYPE_HIGH_BITS: [(&str, u32, u32); 4] = [
  ("machine", 0, 1),
  ("hole", 1, 1),
  ("string_wrapper", 2, 1),
  ("typed_array", 3, 1),
];

/// intptr 边界常量 (tq constexpr, 64 位目标)
pub const K_MAX_INT_PTR: u64 = 0x7FFF_FFFF_FFFF_FFFF;
/// intptr 下界 (64 位目标)
pub const K_MIN_INT_PTR: u64 = 0x8000_0000_0000_0000;

// ===== 布局类 (tq, TurbofanType 抽象基 + 5 具体类, 头 8 = HeapObject) =====
/// 布局类字段表 (声明序; 头 = HeapObject 8 字节, 非压缩)
pub const TURBOFAN_TYPE_CLASSES: [(&str, &str, &str); 5] = [
  (
    "TurbofanBitsetType",
    "TurbofanType",
    "bitset_low:TurbofanTypeLowBits, bitset_high:TurbofanTypeHighBits",
  ),
  (
    "TurbofanUnionType",
    "TurbofanType",
    "type1:TurbofanType, type2:TurbofanType",
  ),
  (
    "TurbofanRangeType",
    "TurbofanType",
    "min:float64, max:float64",
  ),
  (
    "TurbofanHeapConstantType",
    "TurbofanType",
    "constant:HeapObject",
  ),
  (
    "TurbofanOtherNumberConstantType",
    "TurbofanType",
    "constant:float64",
  ),
];

/// 各类头大小/尺寸 (非压缩: kTaggedSize=8, double 经 UnalignedDoubleMember 8 字节)
pub const K_HEADER_SIZE: usize = 8;
/// TurbofanBitsetType 尺寸 = 头 + 4 + 4 = 16
pub const K_BITSET_TYPE_SIZE: usize = 16;
/// TurbofanUnionType 尺寸 = 头 + 8 + 8 = 24
pub const K_UNION_TYPE_SIZE: usize = 24;
/// TurbofanRangeType 尺寸 = 头 + 8 + 8 = 24 (min/max 非对齐双精度)
pub const K_RANGE_TYPE_SIZE: usize = 24;
/// TurbofanHeapConstantType 尺寸 = 头 + 8 = 16
pub const K_HEAP_CONSTANT_TYPE_SIZE: usize = 16;
/// TurbofanOtherNumberConstantType 尺寸 = 头 + 8 = 16
pub const K_OTHER_NUMBER_CONSTANT_TYPE_SIZE: usize = 16;

// ===== 访问器 (16 个, 全部平凡读写; 原版 turbofan-types-inl.h 逐法挂载) =====
use std::mem::{offset_of, size_of};

use v8_objects_core::{
  heap_object::HeapObject,
  objects::WriteBarrierMode,
  tagged::{Object, Tagged},
};

/// TurbofanType 抽象基占位 (原版 class TurbofanType: abstract 联合前端)
///
/// 单元标记: 供 `Tagged<TurbofanType>` 句柄形态 (联合支路元素类型)。
pub struct TurbofanType;

/// 位集型 (原版 TurbofanBitsetType: 头 8 + bitset_low_ 4 + bitset_high_ 4)
#[repr(C)]
pub struct TurbofanBitsetType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] bitset_low_ (TurbofanTypeLowBits)
  pub bitset_low: u32,
  /// [12] bitset_high_ (TurbofanTypeHighBits)
  pub bitset_high: u32,
}

const _: () = assert!(offset_of!(TurbofanBitsetType, bitset_low) == 8);
const _: () = assert!(offset_of!(TurbofanBitsetType, bitset_high) == 12);
const _: () = assert!(size_of::<TurbofanBitsetType>() == K_BITSET_TYPE_SIZE);

/// 位集访问面 (原版 bitset_low/bitset_high 平凡读写)
pub trait TurbofanBitsetTypeExt: Copy {
  /// v8: src/objects/turbofan-types-inl.h TurbofanBitsetType::bitset_low
  fn bitset_low(&self) -> u32;
  /// v8: src/objects/turbofan-types-inl.h TurbofanBitsetType::set_bitset_low
  fn set_bitset_low(&self, value: u32);
  /// v8: src/objects/turbofan-types-inl.h TurbofanBitsetType::bitset_high
  fn bitset_high(&self) -> u32;
  /// v8: src/objects/turbofan-types-inl.h TurbofanBitsetType::set_bitset_high
  fn set_bitset_high(&self, value: u32);
}

impl TurbofanBitsetTypeExt for Tagged<TurbofanBitsetType> {
  #[inline]
  fn bitset_low(&self) -> u32 {
    // SAFETY: 契约 — Tagged<TurbofanBitsetType> 指向有效对象
    unsafe { (*self.as_ptr()).bitset_low }
  }

  #[inline]
  fn set_bitset_low(&self, value: u32) {
    let addr = self.address() + offset_of!(TurbofanBitsetType, bitset_low);
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }

  #[inline]
  fn bitset_high(&self) -> u32 {
    // SAFETY: 同上
    unsafe { (*self.as_ptr()).bitset_high }
  }

  #[inline]
  fn set_bitset_high(&self, value: u32) {
    let addr = self.address() + offset_of!(TurbofanBitsetType, bitset_high);
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }
}

/// 联合型 (原版 TurbofanUnionType: 头 8 + type1_ 8 + type2_ 8)
#[repr(C)]
pub struct TurbofanUnionType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] type1_ (TaggedMember<TurbofanType>)
  pub type1: Tagged<Object>,
  /// [16] type2_ (TaggedMember<TurbofanType>)
  pub type2: Tagged<Object>,
}

const _: () = assert!(offset_of!(TurbofanUnionType, type1) == 8);
const _: () = assert!(offset_of!(TurbofanUnionType, type2) == 16);
const _: () = assert!(size_of::<TurbofanUnionType>() == K_UNION_TYPE_SIZE);

/// 联合支路访问面 (原版 type1/type2: tagged 槽 load/store, set 带写屏障形参)
pub trait TurbofanUnionTypeExt: Copy {
  /// v8: src/objects/turbofan-types-inl.h TurbofanUnionType::type1
  fn type1(&self) -> Tagged<TurbofanType>;
  /// v8: src/objects/turbofan-types-inl.h TurbofanUnionType::set_type1
  fn set_type1(&self, value: Tagged<TurbofanType>, mode: WriteBarrierMode);
  /// v8: src/objects/turbofan-types-inl.h TurbofanUnionType::type2
  fn type2(&self) -> Tagged<TurbofanType>;
  /// v8: src/objects/turbofan-types-inl.h TurbofanUnionType::set_type2
  fn set_type2(&self, value: Tagged<TurbofanType>, mode: WriteBarrierMode);
}

/// tagged 槽地址 (基址 + 字段偏移)
#[inline]
fn union_slot_addr(obj: usize, field_offset: usize) -> usize {
  obj + field_offset
}

impl TurbofanUnionTypeExt for Tagged<TurbofanUnionType> {
  #[inline]
  fn type1(&self) -> Tagged<TurbofanType> {
    // 原版: type1_.load()
    let addr = union_slot_addr(self.address(), offset_of!(TurbofanUnionType, type1));
    // SAFETY: 契约同上; 槽位恒在对象内
    Tagged::<Object>::new(unsafe { *(addr as *const usize) }).cast()
  }

  #[inline]
  fn set_type1(&self, value: Tagged<TurbofanType>, mode: WriteBarrierMode) {
    let _ = mode; // 基线: 写屏障桩 (remembered-set 归 heap 席)
    let addr = union_slot_addr(self.address(), offset_of!(TurbofanUnionType, type1));
    // SAFETY: 同上 (写路径)
    unsafe { *(addr as *mut usize) = value.ptr() }
  }

  #[inline]
  fn type2(&self) -> Tagged<TurbofanType> {
    let addr = union_slot_addr(self.address(), offset_of!(TurbofanUnionType, type2));
    // SAFETY: 同 type1 契约
    Tagged::<Object>::new(unsafe { *(addr as *const usize) }).cast()
  }

  #[inline]
  fn set_type2(&self, value: Tagged<TurbofanType>, mode: WriteBarrierMode) {
    let _ = mode;
    let addr = union_slot_addr(self.address(), offset_of!(TurbofanUnionType, type2));
    // SAFETY: 同上
    unsafe { *(addr as *mut usize) = value.ptr() }
  }
}

/// 区间型 (原版 TurbofanRangeType: 头 8 + min_ 8 + max_ 8, 非对齐双精度)
#[repr(C)]
pub struct TurbofanRangeType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] min_ (UnalignedDoubleMember, 位级读写)
  pub min: [u8; 8],
  /// [16] max_ (UnalignedDoubleMember, 位级读写)
  pub max: [u8; 8],
}

const _: () = assert!(offset_of!(TurbofanRangeType, min) == 8);
const _: () = assert!(size_of::<TurbofanRangeType>() == K_RANGE_TYPE_SIZE);

/// 区间访问面 (原版 min/max: UnalignedDoubleMember value/set_value)
pub trait TurbofanRangeTypeExt: Copy {
  /// v8: src/objects/turbofan-types-inl.h TurbofanRangeType::min
  fn min(&self) -> f64;
  /// v8: src/objects/turbofan-types-inl.h TurbofanRangeType::set_min
  fn set_min(&self, value: f64);
  /// v8: src/objects/turbofan-types-inl.h TurbofanRangeType::max
  fn max(&self) -> f64;
  /// v8: src/objects/turbofan-types-inl.h TurbofanRangeType::set_max
  fn set_max(&self, value: f64);
}

impl TurbofanRangeTypeExt for Tagged<TurbofanRangeType> {
  #[inline]
  fn min(&self) -> f64 {
    let addr = self.address() + offset_of!(TurbofanRangeType, min);
    // SAFETY: 契约 — addr 处于对象数据区
    unsafe { (addr as *const f64).read_unaligned() }
  }

  #[inline]
  fn set_min(&self, value: f64) {
    let addr = self.address() + offset_of!(TurbofanRangeType, min);
    // SAFETY: 同上
    unsafe { (addr as *mut f64).write_unaligned(value) }
  }

  #[inline]
  fn max(&self) -> f64 {
    let addr = self.address() + offset_of!(TurbofanRangeType, max);
    // SAFETY: 同上
    unsafe { (addr as *const f64).read_unaligned() }
  }

  #[inline]
  fn set_max(&self, value: f64) {
    let addr = self.address() + offset_of!(TurbofanRangeType, max);
    // SAFETY: 同上
    unsafe { (addr as *mut f64).write_unaligned(value) }
  }
}

/// 堆常量型 (原版 TurbofanHeapConstantType: 头 8 + constant_ tagged)
#[repr(C)]
pub struct TurbofanHeapConstantType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] constant_ (TaggedMember<HeapObject>)
  pub constant: Tagged<Object>,
}

const _: () = assert!(offset_of!(TurbofanHeapConstantType, constant) == 8);
const _: () = assert!(size_of::<TurbofanHeapConstantType>() == K_HEAP_CONSTANT_TYPE_SIZE);

/// 堆常量访问面 (原版 constant: Cast<HeapObject>(constant_.load()))
pub trait TurbofanHeapConstantTypeExt: Copy {
  /// v8: src/objects/turbofan-types-inl.h TurbofanHeapConstantType::constant
  fn constant(&self) -> Tagged<HeapObject>;
  /// v8: src/objects/turbofan-types-inl.h TurbofanHeapConstantType::set_constant
  fn set_constant(&self, value: Tagged<HeapObject>, mode: WriteBarrierMode);
}

impl TurbofanHeapConstantTypeExt for Tagged<TurbofanHeapConstantType> {
  #[inline]
  fn constant(&self) -> Tagged<HeapObject> {
    // 原版: Cast<HeapObject>(constant_.load()) — 基线裸转换 (信任链)
    // SAFETY: 契约同上
    unsafe { (*self.as_ptr()).constant }.cast()
  }

  #[inline]
  fn set_constant(&self, value: Tagged<HeapObject>, mode: WriteBarrierMode) {
    let _ = mode; // 基线: 写屏障桩
    let addr = self.address() + offset_of!(TurbofanHeapConstantType, constant);
    // SAFETY: 同上
    unsafe { *(addr as *mut usize) = value.ptr() }
  }
}

/// 其他数值常量型 (原版 TurbofanOtherNumberConstantType: 头 8 + constant_ 非对齐双精度)
#[repr(C)]
pub struct TurbofanOtherNumberConstantType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] constant_ (UnalignedDoubleMember, 位级读写)
  pub constant: [u8; 8],
}

const _: () = assert!(offset_of!(TurbofanOtherNumberConstantType, constant) == 8);
const _: () =
  assert!(size_of::<TurbofanOtherNumberConstantType>() == K_OTHER_NUMBER_CONSTANT_TYPE_SIZE);

/// 其他数值常量访问面 (原版 constant: UnalignedDoubleMember value/set_value)
pub trait TurbofanOtherNumberConstantTypeExt: Copy {
  /// v8: src/objects/turbofan-types-inl.h TurbofanOtherNumberConstantType::constant
  fn constant(&self) -> f64;
  /// v8: src/objects/turbofan-types-inl.h TurbofanOtherNumberConstantType::set_constant
  fn set_constant(&self, value: f64);
}

impl TurbofanOtherNumberConstantTypeExt for Tagged<TurbofanOtherNumberConstantType> {
  #[inline]
  fn constant(&self) -> f64 {
    let addr = self.address() + offset_of!(TurbofanOtherNumberConstantType, constant);
    // SAFETY: 契约 — addr 处于对象数据区
    unsafe { (addr as *const f64).read_unaligned() }
  }

  #[inline]
  fn set_constant(&self, value: f64) {
    let addr = self.address() + offset_of!(TurbofanOtherNumberConstantType, constant);
    // SAFETY: 同上
    unsafe { (addr as *mut f64).write_unaligned(value) }
  }
}
