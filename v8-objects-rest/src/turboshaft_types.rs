// 由 tool/src/objects/turboshaft-types.h.js 从 v8/src/objects/turboshaft-types.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// 非压缩布局; 柔性尾三态

//! Turboshaft 类型族 (原版 turboshaft-types.{h,-inl.h,tq})
//!
//! Word32/Word64/Float64 三支, 每支 Range (定尺寸) + Set (柔性尾) 两型;
//! Word64Set 尾部拆高低双段, Float64Set 尾部非对齐 double 且 SizeFor 不补对齐.

use std::mem::{offset_of, size_of};

use v8_objects_core::{heap_object::HeapObject, tagged::Tagged};

/// 特殊浮点位域 (TurboshaftFloatSpecialValues: nan@0, minus_zero@1, unused@2..31)
pub const FLOAT_SPECIAL_VALUES: [(&str, u32, u32); 3] =
  [("nan", 0, 1), ("minus_zero", 1, 1), ("unused", 2, 30)];

// ===== 类层级 (h 声明序 10 类, 其中抽象 4) =====
/// 类清单 (名, 基类)
pub const TURBOSHAFT_TYPE_CLASSES: [(&str, &str); 10] = [
  ("TurboshaftType", "HeapObject"),
  ("TurboshaftWord32Type", "TurboshaftType"),
  ("TurboshaftWord32RangeType", "TurboshaftWord32Type"),
  ("TurboshaftWord32SetType", "TurboshaftWord32Type"),
  ("TurboshaftWord64Type", "TurboshaftType"),
  ("TurboshaftWord64RangeType", "TurboshaftWord64Type"),
  ("TurboshaftWord64SetType", "TurboshaftWord64Type"),
  ("TurboshaftFloat64Type", "TurboshaftType"),
  ("TurboshaftFloat64RangeType", "TurboshaftFloat64Type"),
  ("TurboshaftFloat64SetType", "TurboshaftFloat64Type"),
];

// ===== 布局常量 (非压缩 kTaggedSize=8, 头 = HeapObject map 槽) =====
/// Word32Range: 头 8 + from 4 + to 4
pub const K_WORD32_RANGE_SIZE: usize = 16;
/// Word32Set 数据起点 (头 8 + set_size 4); 尺寸 = ALIGN8(12 + 4·n)
pub const K_WORD32_SET_DATA_START: usize = 12;
/// Word64Range: 头 8 + 4×u32
pub const K_WORD64_RANGE_SIZE: usize = 24;
/// Word64Set 数据起点; 尺寸 = ALIGN8(12 + 8·n) — 低位段在 elements[set_size + i]
pub const K_WORD64_SET_DATA_START: usize = 12;
/// Float64Type: 头 8 + special_values 4
pub const K_FLOAT64_TYPE_SIZE: usize = 12;
/// Float64Range: 12 + 显式 padding 4 + min 8 + max 8
pub const K_FLOAT64_RANGE_SIZE: usize = 32;
/// Float64Set 数据起点 (头 8 + special_values 4 + set_size 4); 尺寸 = 16 + 8·n
pub const K_FLOAT64_SET_DATA_START: usize = 16;

/// Word32Set 尺寸 (原版 SizeFor: OBJECT_POINTER_ALIGN(12 + set_size*4))
pub const fn word32_set_size_for(set_size: usize) -> usize {
  (K_WORD32_SET_DATA_START + set_size * 4 + 7) & !7
}

/// Word64Set 尺寸 (原版 SizeFor: OBJECT_POINTER_ALIGN(12 + 2*set_size*4))
pub const fn word64_set_size_for(set_size: usize) -> usize {
  (K_WORD64_SET_DATA_START + 2 * set_size * 4 + 7) & !7
}

/// Float64Set 尺寸 (原版 SizeFor: 16 + set_size*8, 无对齐)
pub const fn float64_set_size_for(set_size: usize) -> usize {
  K_FLOAT64_SET_DATA_START + set_size * 8
}

// ===== 抽象基类标记 (abstract, 无附加字段面) =====
/// 根抽象类 (原版 TurboshaftType, 仅 map 头)
pub struct TurboshaftType;
/// Word32 支抽象类 (原版无字段)
pub struct TurboshaftWord32Type;
/// Word64 支抽象类 (原版无字段)
pub struct TurboshaftWord64Type;

// ===== 具体类布局 (inl 访问器逐字段挂载) =====

/// Word32Range (原版头 8 + from_/to_ 双 u32)
#[repr(C)]
pub struct TurboshaftWord32RangeType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] from_
  pub from: u32,
  /// [12] to_
  pub to: u32,
}

const _: () = assert!(offset_of!(TurboshaftWord32RangeType, from) == 8);
const _: () = assert!(offset_of!(TurboshaftWord32RangeType, to) == 12);
const _: () = assert!(size_of::<TurboshaftWord32RangeType>() == K_WORD32_RANGE_SIZE);

/// Word32 区间界访问面 (原版 from/set_from/to/set_to)
pub trait TurboshaftWord32RangeTypeExt: Copy {
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord32RangeType::from
  fn from(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord32RangeType::set_from
  fn set_from(&self, value: u32);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord32RangeType::to
  fn to(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord32RangeType::set_to
  fn set_to(&self, value: u32);
}

impl TurboshaftWord32RangeTypeExt for Tagged<TurboshaftWord32RangeType> {
  #[inline]
  fn from(&self) -> u32 {
    // SAFETY: 契约 — Tagged<TurboshaftWord32RangeType> 指向有效对象
    unsafe { (*self.as_ptr()).from }
  }

  #[inline]
  fn set_from(&self, value: u32) {
    let addr = self.address() + offset_of!(TurboshaftWord32RangeType, from);
    // SAFETY: 同 from 契约 (原版平凡写)
    unsafe { *(addr as *mut u32) = value }
  }

  #[inline]
  fn to(&self) -> u32 {
    // SAFETY: 契约同上
    unsafe { (*self.as_ptr()).to }
  }

  #[inline]
  fn set_to(&self, value: u32) {
    let addr = self.address() + offset_of!(TurboshaftWord32RangeType, to);
    // SAFETY: 同 to 契约
    unsafe { *(addr as *mut u32) = value }
  }
}

/// Word32Set (原版头 8 + set_size_ 4; elements 柔性尾 u32 不入结构体)
#[repr(C)]
pub struct TurboshaftWord32SetType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] set_size_ (V8_TQ_CONST)
  pub set_size: u32,
}

const _: () = assert!(offset_of!(TurboshaftWord32SetType, set_size) == 8);
// Rust repr(C) 尾 u32 对齐补至 16; 堆侧数据起点恒 12 (K_WORD32_SET_DATA_START),
// 全对象尺寸经 word32_set_size_for 计算

/// Word32 集访问面 (原版 set_size/set_set_size/elements/set_elements)
pub trait TurboshaftWord32SetTypeExt: Copy {
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord32SetType::set_size
  fn set_size(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord32SetType::set_set_size
  fn set_set_size_field(&self, value: u32);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord32SetType::elements
  /// (DCHECK_GE(i,0)+DCHECK_LT(i,set_size) 后柔性尾索引)
  fn element(&self, i: usize) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord32SetType::set_elements
  fn set_element(&self, i: usize, value: u32);
}

impl TurboshaftWord32SetTypeExt for Tagged<TurboshaftWord32SetType> {
  #[inline]
  fn set_size(&self) -> u32 {
    // SAFETY: 契约同上
    unsafe { (*self.as_ptr()).set_size }
  }

  #[inline]
  fn set_set_size_field(&self, value: u32) {
    let addr = self.address() + offset_of!(TurboshaftWord32SetType, set_size);
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }

  #[inline]
  fn element(&self, i: usize) -> u32 {
    let n = self.set_size() as usize;
    debug_assert!(i < n, "原版 DCHECK_LT(i, static_cast<int>(set_size()))");
    // 原版: elements()[i] — 数据起点 + i*4
    let addr = self.address() + K_WORD32_SET_DATA_START + i * 4;
    // SAFETY: i < set_size (上方域检)
    unsafe { *(addr as *const u32) }
  }

  #[inline]
  fn set_element(&self, i: usize, value: u32) {
    let n = self.set_size() as usize;
    debug_assert!(i < n, "原版 DCHECK_LT(i, static_cast<int>(set_size()))");
    let addr = self.address() + K_WORD32_SET_DATA_START + i * 4;
    // SAFETY: 同 element 写路径
    unsafe { *(addr as *mut u32) = value }
  }
}

/// Word64Range (原版头 8 + from_high/from_low/to_high/to_low 四 u32)
#[repr(C)]
pub struct TurboshaftWord64RangeType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] from_high_
  pub from_high: u32,
  /// [12] from_low_
  pub from_low: u32,
  /// [16] to_high_
  pub to_high: u32,
  /// [20] to_low_
  pub to_low: u32,
}

const _: () = assert!(offset_of!(TurboshaftWord64RangeType, from_high) == 8);
const _: () = assert!(offset_of!(TurboshaftWord64RangeType, to_low) == 20);
const _: () = assert!(size_of::<TurboshaftWord64RangeType>() == K_WORD64_RANGE_SIZE);

/// Word64 区间界访问面 (原版四段高低平凡读写)
pub trait TurboshaftWord64RangeTypeExt: Copy {
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64RangeType::from_high
  fn get_from_high(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64RangeType::set_from_high
  fn set_from_high(&self, value: u32);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64RangeType::from_low
  fn get_from_low(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64RangeType::set_from_low
  fn set_from_low(&self, value: u32);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64RangeType::to_high
  fn to_high(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64RangeType::set_to_high
  fn set_to_high(&self, value: u32);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64RangeType::to_low
  fn to_low(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64RangeType::set_to_low
  fn set_to_low(&self, value: u32);
}

impl TurboshaftWord64RangeTypeExt for Tagged<TurboshaftWord64RangeType> {
  #[inline]
  fn get_from_high(&self) -> u32 {
    // SAFETY: 契约同上
    unsafe { (*self.as_ptr()).from_high }
  }

  #[inline]
  fn set_from_high(&self, value: u32) {
    let addr = self.address() + offset_of!(TurboshaftWord64RangeType, from_high);
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }

  #[inline]
  fn get_from_low(&self) -> u32 {
    // SAFETY: 同上
    unsafe { (*self.as_ptr()).from_low }
  }

  #[inline]
  fn set_from_low(&self, value: u32) {
    let addr = self.address() + offset_of!(TurboshaftWord64RangeType, from_low);
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }

  #[inline]
  fn to_high(&self) -> u32 {
    // SAFETY: 同上
    unsafe { (*self.as_ptr()).to_high }
  }

  #[inline]
  fn set_to_high(&self, value: u32) {
    let addr = self.address() + offset_of!(TurboshaftWord64RangeType, to_high);
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }

  #[inline]
  fn to_low(&self) -> u32 {
    // SAFETY: 同上
    unsafe { (*self.as_ptr()).to_low }
  }

  #[inline]
  fn set_to_low(&self, value: u32) {
    let addr = self.address() + offset_of!(TurboshaftWord64RangeType, to_low);
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }
}

/// Word64Set (原版头 8 + set_size_ 4; 柔性尾 = 高段 [n] + 低段 [n])
#[repr(C)]
pub struct TurboshaftWord64SetType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] set_size_ (V8_TQ_CONST)
  pub set_size: u32,
}

const _: () = assert!(offset_of!(TurboshaftWord64SetType, set_size) == 8);
// 同 Word32Set: 尾 u32 对齐补至 16; 数据起点恒 12 (K_WORD64_SET_DATA_START)

/// Word64 集访问面 (原版 elements_high/low: 低段寻址 elements[set_size + i])
pub trait TurboshaftWord64SetTypeExt: Copy {
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64SetType::set_size
  fn set_size(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64SetType::set_set_size
  fn set_set_size_field(&self, value: u32);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64SetType::elements_high
  fn element_high(&self, i: usize) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64SetType::set_elements_high
  fn set_element_high(&self, i: usize, value: u32);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64SetType::elements_low
  fn element_low(&self, i: usize) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftWord64SetType::set_elements_low
  fn set_element_low(&self, i: usize, value: u32);
}

impl TurboshaftWord64SetTypeExt for Tagged<TurboshaftWord64SetType> {
  #[inline]
  fn set_size(&self) -> u32 {
    // SAFETY: 契约同上
    unsafe { (*self.as_ptr()).set_size }
  }

  #[inline]
  fn set_set_size_field(&self, value: u32) {
    let addr = self.address() + offset_of!(TurboshaftWord64SetType, set_size);
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }

  #[inline]
  fn element_high(&self, i: usize) -> u32 {
    let n = self.set_size() as usize;
    debug_assert!(i < n, "原版 DCHECK_LT(i, static_cast<int>(set_size()))");
    let addr = self.address() + K_WORD64_SET_DATA_START + i * 4;
    // SAFETY: 上方域检
    unsafe { *(addr as *const u32) }
  }

  #[inline]
  fn set_element_high(&self, i: usize, value: u32) {
    let n = self.set_size() as usize;
    debug_assert!(i < n, "原版 DCHECK_LT(i, static_cast<int>(set_size()))");
    let addr = self.address() + K_WORD64_SET_DATA_START + i * 4;
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }

  #[inline]
  fn element_low(&self, i: usize) -> u32 {
    let n = self.set_size() as usize;
    debug_assert!(i < n, "原版 DCHECK_LT(i, static_cast<int>(set_size()))");
    // 原版: elements()[set_size() + i] — 低段在高段之后
    let addr = self.address() + K_WORD64_SET_DATA_START + (n + i) * 4;
    // SAFETY: 上方域检 (低段占 [n, 2n) 槽)
    unsafe { *(addr as *const u32) }
  }

  #[inline]
  fn set_element_low(&self, i: usize, value: u32) {
    let n = self.set_size() as usize;
    debug_assert!(i < n, "原版 DCHECK_LT(i, static_cast<int>(set_size()))");
    let addr = self.address() + K_WORD64_SET_DATA_START + (n + i) * 4;
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }
}

/// Float64 抽象基 (原版头 8 + special_values_ 4; 具体两支共享该字段)
#[repr(C)]
pub struct TurboshaftFloat64Type {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] special_values_ (TurboshaftFloatSpecialValues 位域)
  pub special_values: u32,
}

const _: () = assert!(offset_of!(TurboshaftFloat64Type, special_values) == 8);
// 尾 u32 对齐补至 16; 堆侧 kFloat64TypeSize=12 语义由常量表登记 (K_FLOAT64_TYPE_SIZE)

/// 特殊浮点标志访问面 (原版 special_values/set_special_values, 两支共用)
pub trait TurboshaftFloat64TypeExt: Copy {
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64Type::special_values
  fn special_values(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64Type::set_special_values
  fn set_special_values(&self, value: u32);
}

/// 基字段区读 special_values (两支布局前 12 字节与抽象基全等)
#[inline]
fn float64_special_values_load(obj: usize) -> u32 {
  // SAFETY: 契约 — obj 为有效 Float64 支对象基址, [8] 恒为 special_values_
  unsafe { *((obj + offset_of!(TurboshaftFloat64Type, special_values)) as *const u32) }
}

/// 基字段区写 special_values
#[inline]
fn float64_special_values_store(obj: usize, value: u32) {
  // SAFETY: 同上
  unsafe { *((obj + offset_of!(TurboshaftFloat64Type, special_values)) as *mut u32) = value }
}

impl TurboshaftFloat64TypeExt for Tagged<TurboshaftFloat64Type> {
  #[inline]
  fn special_values(&self) -> u32 {
    float64_special_values_load(self.address())
  }

  #[inline]
  fn set_special_values(&self, value: u32) {
    float64_special_values_store(self.address(), value)
  }
}

/// Float64Range (原版 special 8..12 + 显式 _padding_ 12..16 + min/max 非对齐双段)
#[repr(C)]
pub struct TurboshaftFloat64RangeType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] special_values_
  pub special_values: u32,
  /// [12] _padding_ (对齐 min_/max_ 至 8 字节界)
  pub padding: u32,
  /// [16] min_ (UnalignedDoubleMember, 位级读写)
  pub min: [u8; 8],
  /// [24] max_ (UnalignedDoubleMember, 位级读写)
  pub max: [u8; 8],
}

const _: () = assert!(offset_of!(TurboshaftFloat64RangeType, min) == 16);
const _: () = assert!(size_of::<TurboshaftFloat64RangeType>() == K_FLOAT64_RANGE_SIZE);

/// Float64 区间访问面 (原版 min/max: UnalignedDoubleMember value/set_value)
pub trait TurboshaftFloat64RangeTypeExt: Copy {
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64Type::special_values
  /// (继承基类)
  fn special_values(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64Type::set_special_values
  fn set_special_values(&self, value: u32);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64RangeType::min
  fn min(&self) -> f64;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64RangeType::set_min
  fn set_min(&self, value: f64);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64RangeType::max
  fn max(&self) -> f64;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64RangeType::set_max
  fn set_max(&self, value: f64);
}

impl TurboshaftFloat64RangeTypeExt for Tagged<TurboshaftFloat64RangeType> {
  #[inline]
  fn special_values(&self) -> u32 {
    float64_special_values_load(self.address())
  }

  #[inline]
  fn set_special_values(&self, value: u32) {
    float64_special_values_store(self.address(), value)
  }

  #[inline]
  fn min(&self) -> f64 {
    let addr = self.address() + offset_of!(TurboshaftFloat64RangeType, min);
    // 原版: min_.value() (memcpy 非对齐读)
    // SAFETY: 契约 — addr 处于对象 [16,24) 数据区
    unsafe { (addr as *const f64).read_unaligned() }
  }

  #[inline]
  fn set_min(&self, value: f64) {
    let addr = self.address() + offset_of!(TurboshaftFloat64RangeType, min);
    // SAFETY: 同上 (写路径)
    unsafe { (addr as *mut f64).write_unaligned(value) }
  }

  #[inline]
  fn max(&self) -> f64 {
    let addr = self.address() + offset_of!(TurboshaftFloat64RangeType, max);
    // SAFETY: 契约 — addr 处于对象 [24,32) 数据区
    unsafe { (addr as *const f64).read_unaligned() }
  }

  #[inline]
  fn set_max(&self, value: f64) {
    let addr = self.address() + offset_of!(TurboshaftFloat64RangeType, max);
    // SAFETY: 同上
    unsafe { (addr as *mut f64).write_unaligned(value) }
  }
}

/// Float64Set (原版 special 8 + set_size 12; 柔性尾 = 非对齐 double [n])
#[repr(C)]
pub struct TurboshaftFloat64SetType {
  /// HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] special_values_
  pub special_values: u32,
  /// [12] set_size_ (V8_TQ_CONST)
  pub set_size: u32,
}

const _: () = assert!(offset_of!(TurboshaftFloat64SetType, set_size) == 12);
const _: () = assert!(size_of::<TurboshaftFloat64SetType>() == K_FLOAT64_SET_DATA_START);

/// Float64 集访问面 (原版 elements: 非对齐 double 柔性尾)
pub trait TurboshaftFloat64SetTypeExt: Copy {
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64Type::special_values
  /// (继承基类)
  fn special_values(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64Type::set_special_values
  fn set_special_values(&self, value: u32);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64SetType::set_size
  fn set_size(&self) -> u32;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64SetType::set_set_size
  fn set_set_size_field(&self, value: u32);
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64SetType::elements
  fn element(&self, i: usize) -> f64;
  /// v8: src/objects/turboshaft-types-inl.h TurboshaftFloat64SetType::set_elements
  fn set_element(&self, i: usize, value: f64);
}

impl TurboshaftFloat64SetTypeExt for Tagged<TurboshaftFloat64SetType> {
  #[inline]
  fn special_values(&self) -> u32 {
    float64_special_values_load(self.address())
  }

  #[inline]
  fn set_special_values(&self, value: u32) {
    float64_special_values_store(self.address(), value)
  }

  #[inline]
  fn set_size(&self) -> u32 {
    // SAFETY: 契约同上
    unsafe { (*self.as_ptr()).set_size }
  }

  #[inline]
  fn set_set_size_field(&self, value: u32) {
    let addr = self.address() + offset_of!(TurboshaftFloat64SetType, set_size);
    // SAFETY: 同上
    unsafe { *(addr as *mut u32) = value }
  }

  #[inline]
  fn element(&self, i: usize) -> f64 {
    let n = self.set_size() as usize;
    debug_assert!(i < n, "原版 DCHECK_LT(i, static_cast<int>(set_size()))");
    // 原版: elements()[i].value() — 非对齐双段柔性尾
    let addr = self.address() + K_FLOAT64_SET_DATA_START + i * 8;
    // SAFETY: 上方域检
    unsafe { (addr as *const f64).read_unaligned() }
  }

  #[inline]
  fn set_element(&self, i: usize, value: f64) {
    let n = self.set_size() as usize;
    debug_assert!(i < n, "原版 DCHECK_LT(i, static_cast<int>(set_size()))");
    let addr = self.address() + K_FLOAT64_SET_DATA_START + i * 8;
    // SAFETY: 同上
    unsafe { (addr as *mut f64).write_unaligned(value) }
  }
}
