//! 12.6 延期台账恢复 (轮10 W10OC): objects.cc 语义子集 (comparison_result_to_bool/
//! strict_number_*/same_*/object_strict_equals/object_boolean_value/get_simple_hash/
//! 数值转换族) 由 tool/src/objects/objects.cc.js 注入 @cc-semantics 区段落地后解冻。

// 迁移规约:
//   casting-unittest.cc TEST(CastingTest, Smi/Number): Is/TryCast/Cast/UncheckedCast
//   objects.cc:144-160 ComparisonResultToBool / 840-876 StrictNumber* / 1026-1038
//              StrictEquals / 1538-1552 ToInt32 / 1799-1834 SameValue 族
//   objects-inl.h:631-643 SameNumberValue / 1060-1107 GetSimpleHash 可承载子集
//   object-conversions-inl.h:18-85 NumberValue/ToUint32/ToArrayLength/
//              ToArrayIndex/ToIntegerIndex
// isolate/heap-layout 依赖的 ReadOnly/Weak/Union 用例推迟 (登记于脚本头)。
use aok::{OK, Void};
use v8_common::Operation;
use v8_objects_core::{
  heap_number::HeapNumber,
  heap_object::HeapObject,
  instance_type::InstanceType,
  map::Map,
  objects::{self as objs, ComparisonResult as Cmp, cast, is, try_cast, unchecked_cast},
  oddball::{Oddball, OddballKind},
  smi::Smi,
  symbol::Symbol,
  tagged::{Object, Tagged},
};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

type TestResult<T> = Result<T, v8_objects_core::Error>;

/// 在 Zone 中构造带指定实例类型的 HeapNumber。
fn zone_heap_number(
  zone: &v8_zone::Zone,
  ty: InstanceType,
  value: f64,
) -> TestResult<Tagged<Object>> {
  let meta = Tagged::<Map>::new(0x1);
  let map = Tagged::allocate_in_zone(zone, Map::new(meta, ty, 16))?;
  let num = Tagged::allocate_in_zone(zone, HeapNumber::new(map, value))?;
  Ok(num.cast())
}

/// 在 Zone 中构造带位域标志的 Oddball。
fn zone_oddball(
  zone: &v8_zone::Zone,
  kind: OddballKind,
  undetectable: bool,
) -> TestResult<Tagged<Object>> {
  let meta = Tagged::<Map>::new(0x1);
  let mut map = Map::new(meta, InstanceType::Oddball, 24);
  map.set_is_undetectable(undetectable);
  let map_tagged = Tagged::allocate_in_zone(zone, map)?;
  let empty = Tagged::<Object>::new(0);
  let odd = Oddball::new(map_tagged, kind, f64::NAN, empty, empty, empty);
  Ok(Tagged::allocate_in_zone(zone, odd)?.cast())
}

/// Zone 分配的裸对象 (任意实例类型, 无字段)。
fn zone_bare(
  zone: &v8_zone::Zone,
  ty: InstanceType,
  undetectable: bool,
) -> TestResult<Tagged<Object>> {
  let meta = Tagged::<Map>::new(0x1);
  let mut map = Map::new(meta, ty, 8);
  map.set_is_undetectable(undetectable);
  let map_tagged = Tagged::allocate_in_zone(zone, map)?;
  Ok(Tagged::allocate_in_zone(zone, HeapObject::new(map_tagged))?.cast())
}

/// 规约 (casting-unittest.cc:148-153 CastingTest.Smi):
/// Smi(0/1/-1): Smi✓ Number✓ HeapObject✗ Object✓; TryCast/Cast 语义一致。
#[test]
fn casting_smi() -> Void {
  for n in [0, 1, -1] {
    let v: Tagged<Object> = Smi::from_int(n).cast();
    assert!(is::<Smi>(v), "n={n}");
    assert!(is::<objs::Number>(v), "n={n}");
    assert!(!is::<HeapObject>(v));
    assert!(is::<Object>(v));
    assert!(!is::<HeapNumber>(v));
    assert_eq!(try_cast::<Smi>(v).map(|t| t.ptr()), Some(v.ptr()));
    assert!(cast::<objs::Number>(v).is_ok());
    assert_eq!(unchecked_cast::<HeapObject>(v).ptr(), v.ptr());
  }
  OK
}

/// 规约 (casting-unittest.cc:160-167 CastingTest.Number):
/// HeapNumber: Number✓ (任意来源); Smi✗。
#[test]
fn casting_number() -> Void {
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "cast-num");
  let num = zone_heap_number(&zone, InstanceType::HeapNumber, 1.5)?;
  assert!(is::<objs::Number>(num));
  assert!(is::<HeapNumber>(num));
  assert!(!is::<Smi>(num));
  assert!(is::<HeapObject>(num));
  assert_eq!(cast::<HeapNumber>(num)?.ptr(), num.ptr());
  // 非 Number 堆对象
  let other = zone_bare(&zone, InstanceType::JSObject, false)?;
  assert!(!is::<objs::Number>(other));
  assert!(cast::<objs::Number>(other).is_err());
  OK
}

/// 规约 (casting.h CastTraits 各特化): Map/Symbol/Oddball 精确类型判定。
#[test]
fn casting_typed_markers() -> Void {
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "cast-ty");
  let map_obj = zone_bare(&zone, InstanceType::Map, false)?;
  assert!(is::<Map>(map_obj));
  assert!(!is::<Map>(zone_bare(
    &zone,
    InstanceType::HeapNumber,
    false
  )?));
  let sym = zone_bare(&zone, InstanceType::Symbol, false)?;
  assert!(is::<Symbol>(sym));
  assert!(is::<Oddball>(zone_oddball(
    &zone,
    OddballKind::True,
    false
  )?));
  assert!(!is::<Oddball>(sym));
  // instance_type_of 助手
  assert_eq!(objs::instance_type_of(map_obj), Some(InstanceType::Map));
  assert_eq!(objs::instance_type_of(Smi::from_int(3).cast()), None);
  OK
}

/// 规约 (objects.cc:144-160): 四个关系运算 × 四种结果的全矩阵。
#[test]
fn comparison_result_to_bool_matrix() -> Void {
  // (op, LessThan→, Equal→, GreaterThan→, Undefined→)
  let table = [
    (Operation::LessThan, true, false, false, false),
    (Operation::LessThanOrEqual, true, true, false, false),
    (Operation::GreaterThan, false, false, true, false),
    (Operation::GreaterThanOrEqual, false, true, true, false),
  ];
  for (op, lt, eq, gt, und) in table {
    assert_eq!(
      objs::comparison_result_to_bool(op, Cmp::LessThan)?,
      lt,
      "{op:?}"
    );
    assert_eq!(
      objs::comparison_result_to_bool(op, Cmp::Equal)?,
      eq,
      "{op:?}"
    );
    assert_eq!(
      objs::comparison_result_to_bool(op, Cmp::GreaterThan)?,
      gt,
      "{op:?}"
    );
    assert_eq!(
      objs::comparison_result_to_bool(op, Cmp::Undefined)?,
      und,
      "{op:?}"
    );
  }
  // 非关系运算 → Err (原版 UNREACHABLE)
  for op in [
    Operation::Add,
    Operation::Equal,
    Operation::StrictEqual,
    Operation::BitwiseAnd,
  ] {
    assert!(
      objs::comparison_result_to_bool(op, Cmp::Equal).is_err(),
      "{op:?}"
    );
  }
  OK
}

/// 规约 (objects.cc:840-876): StrictNumberCompare / StrictNumberEquals / Reverse。
#[test]
fn strict_number_semantics() -> Void {
  use Cmp::*;
  assert_eq!(objs::strict_number_compare(1.0, 2.0), LessThan);
  assert_eq!(objs::strict_number_compare(2.0, 1.0), GreaterThan);
  assert_eq!(objs::strict_number_compare(1.0, 1.0), Equal);
  assert_eq!(objs::strict_number_compare(f64::NAN, 1.0), Undefined);
  assert_eq!(objs::strict_number_compare(1.0, f64::NAN), Undefined);
  assert_eq!(objs::strict_number_compare(-0.0, 0.0), Equal); // 严格相等意义下 -0 == +0
  assert!(objs::strict_number_equals(1.0, 1.0));
  assert!(objs::strict_number_equals(-0.0, 0.0));
  assert!(!objs::strict_number_equals(f64::NAN, f64::NAN));
  assert!(!objs::strict_number_equals(1.0, 2.0));
  assert_eq!(objs::reverse(LessThan), GreaterThan);
  assert_eq!(objs::reverse(GreaterThan), LessThan);
  assert_eq!(objs::reverse(Equal), Equal);
  assert_eq!(objs::reverse(Undefined), Undefined);
  OK
}

/// 规约 (objects.cc:1026-1038 StrictEquals + 1799-1834 SameValue 族 +
///  objects-inl.h:631-643 SameNumberValue)。
#[test]
fn equality_semantics() -> Void {
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "eq");
  let smi_1: Tagged<Object> = Smi::from_int(1).cast();
  let hn_1 = zone_heap_number(&zone, InstanceType::HeapNumber, 1.0)?;
  let hn_1b = zone_heap_number(&zone, InstanceType::HeapNumber, 1.0)?;
  let hn_nan = zone_heap_number(&zone, InstanceType::HeapNumber, f64::NAN)?;
  let hn_nan_b = zone_heap_number(&zone, InstanceType::HeapNumber, f64::NAN)?;
  let hn_neg0 = zone_heap_number(&zone, InstanceType::HeapNumber, -0.0)?;
  let hn_pos0 = zone_heap_number(&zone, InstanceType::HeapNumber, 0.0)?;

  // SameNumberValue: 位级, NaN 互等, -0 != +0
  assert!(objs::same_number_value(f64::NAN, f64::NAN));
  assert!(objs::same_number_value(-0.0, -0.0));
  assert!(!objs::same_number_value(-0.0, 0.0));
  assert!(objs::same_number_value(1.0, 1.0));

  // StrictEquals: Number 跨表示相等; 指针兜底
  assert!(objs::object_strict_equals(smi_1, hn_1)?);
  assert!(objs::object_strict_equals(hn_1, smi_1)?);
  assert!(!objs::object_strict_equals(
    hn_1,
    zone_heap_number(&zone, InstanceType::HeapNumber, 2.0)?
  )?);
  assert!(!objs::object_strict_equals(
    smi_1,
    zone_bare(&zone, InstanceType::JSObject, false)?
  )?);
  let obj_a = zone_bare(&zone, InstanceType::JSObject, false)?;
  assert!(objs::object_strict_equals(obj_a, obj_a)?);
  // SameValue: NaN 互等 (跨对象), -0 != +0
  assert!(objs::object_same_value(hn_nan, hn_nan_b)?);
  assert!(!objs::object_same_value(hn_neg0, hn_pos0)?);
  assert!(objs::object_same_value(smi_1, hn_1)?);
  // SameValueZero: +0 == -0, NaN 互等
  assert!(objs::object_same_value_zero(hn_neg0, hn_pos0)?);
  assert!(objs::object_same_value_zero(hn_nan, hn_nan_b)?);
  assert!(objs::object_same_value_zero(hn_1, hn_1b)?);
  // String/BigInt 分支推迟; 指针相等短路先行 (原版 other == obj 首行)
  let str_obj = zone_bare(&zone, InstanceType::SeqOneByteString, false)?;
  let str_obj_b = zone_bare(&zone, InstanceType::SeqOneByteString, false)?;
  assert!(objs::object_same_value(str_obj, str_obj)?);
  assert!(objs::object_strict_equals(str_obj, str_obj_b).is_err());
  assert!(objs::object_same_value(str_obj, str_obj_b).is_err());
  let big = zone_bare(&zone, InstanceType::BigIntBase, false)?;
  let big_b = zone_bare(&zone, InstanceType::BigIntBase, false)?;
  assert!(objs::object_same_value_zero(big, big)?);
  assert!(objs::object_same_value_zero(big, big_b).is_err());
  // 单边 String/BigInt: 原版直接 false (objects.cc:1029/1032 非 String → return false;
  // 1807-1816 尾分支 return false), 不落内容级推迟
  let plain = zone_bare(&zone, InstanceType::JSObject, false)?;
  assert!(!objs::object_strict_equals(str_obj, plain)?);
  assert!(!objs::object_strict_equals(plain, str_obj)?);
  assert!(!objs::object_strict_equals(smi_1, big)?);
  assert!(!objs::object_same_value(str_obj, plain)?);
  assert!(!objs::object_same_value(big, smi_1)?);
  assert!(!objs::object_same_value_zero(big, plain)?);
  assert!(!objs::object_same_value_zero(plain, str_obj)?);
  OK
}

/// 规约 (objects.cc:813-826 BooleanValue 可承载分支)。
#[test]
fn boolean_value_semantics() -> Void {
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "bool");
  // Smi
  assert!(!objs::object_boolean_value(Smi::from_int(0).cast())?);
  assert!(objs::object_boolean_value(Smi::from_int(-1).cast())?);
  // HeapNumber (DoubleToBoolean)
  assert!(!objs::object_boolean_value(zone_heap_number(
    &zone,
    InstanceType::HeapNumber,
    0.0
  )?)?);
  assert!(!objs::object_boolean_value(zone_heap_number(
    &zone,
    InstanceType::HeapNumber,
    -0.0
  )?)?);
  assert!(!objs::object_boolean_value(zone_heap_number(
    &zone,
    InstanceType::HeapNumber,
    f64::NAN
  )?)?);
  assert!(objs::object_boolean_value(zone_heap_number(
    &zone,
    InstanceType::HeapNumber,
    0.5
  )?)?);
  // Oddball: true/false/null/undefined
  assert!(objs::object_boolean_value(zone_oddball(
    &zone,
    OddballKind::True,
    false
  )?)?);
  assert!(!objs::object_boolean_value(zone_oddball(
    &zone,
    OddballKind::False,
    false
  )?)?);
  assert!(!objs::object_boolean_value(zone_oddball(
    &zone,
    OddballKind::Null,
    false
  )?)?);
  assert!(!objs::object_boolean_value(zone_oddball(
    &zone,
    OddballKind::Undefined,
    false
  )?)?);
  // Undetectable → false
  assert!(!objs::object_boolean_value(zone_bare(
    &zone,
    InstanceType::JSObject,
    true
  )?)?);
  // 其余堆对象 → true
  assert!(objs::object_boolean_value(zone_bare(
    &zone,
    InstanceType::JSObject,
    false
  )?)?);
  // String/BigInt 分支推迟
  assert!(
    objs::object_boolean_value(zone_bare(&zone, InstanceType::SeqOneByteString, false)?).is_err()
  );
  assert!(objs::object_boolean_value(zone_bare(&zone, InstanceType::BigIntBase, false)?).is_err());
  OK
}

/// 规约 (objects.cc:1538-1552 ToInt32 + object-conversions-inl.h:39-85)。
#[test]
fn numeric_conversions() -> Void {
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "conv");
  let hn = |v: f64| zone_heap_number(&zone, InstanceType::HeapNumber, v);
  // NumberValue
  assert_eq!(objs::number_value(Smi::from_int(-7).cast())?, -7.0);
  assert_eq!(objs::number_value(hn(1.5)?)?, 1.5);
  assert!(objs::number_value(zone_bare(&zone, InstanceType::JSObject, false)?).is_err());
  // ToInt32
  assert_eq!(objs::object_to_int32(Smi::from_int(42).cast()), Some(42));
  assert_eq!(objs::object_to_int32(hn(42.0)?), Some(42));
  assert_eq!(objs::object_to_int32(hn(-0.0)?), Some(0));
  assert_eq!(objs::object_to_int32(hn(1.5)?), None);
  assert_eq!(objs::object_to_int32(hn(2147483648.0)?), None); // i32::MAX + 1
  assert_eq!(objs::object_to_int32(hn(2147483647.0)?), Some(i32::MAX));
  assert_eq!(objs::object_to_int32(hn(f64::NAN)?), None);
  assert_eq!(
    objs::object_to_int32(zone_bare(&zone, InstanceType::JSObject, false)?),
    None
  );
  // ToUint32 / ToArrayLength / ToArrayIndex
  assert_eq!(objs::object_to_uint32(Smi::from_int(3).cast()), Some(3));
  assert_eq!(objs::object_to_uint32(Smi::from_int(-3).cast()), None);
  assert_eq!(objs::object_to_uint32(hn(3.0)?), Some(3));
  assert_eq!(objs::object_to_uint32(hn(4294967295.0)?), Some(u32::MAX));
  assert_eq!(objs::object_to_uint32(hn(4294967296.0)?), None);
  assert_eq!(objs::object_to_uint32(hn(-0.0)?), Some(0)); // 2^52 归一化
  assert_eq!(
    objs::object_to_array_length(hn(4294967295.0)?),
    Some(u32::MAX)
  );
  assert_eq!(objs::object_to_array_index(hn(4294967295.0)?), None);
  assert_eq!(objs::object_to_array_index(hn(10.0)?), Some(10));
  // ToIntegerIndex
  assert_eq!(
    objs::object_to_integer_index(Smi::from_int(5).cast()),
    Some(5)
  );
  assert_eq!(
    objs::object_to_integer_index(Smi::from_int(-1).cast()),
    None
  );
  assert_eq!(objs::object_to_integer_index(hn(9007199254740992.0)?), None); // 2^53 越界
  assert_eq!(
    objs::object_to_integer_index(hn(9007199254740991.0)?),
    Some(9_007_199_254_740_991)
  );
  assert_eq!(objs::object_to_integer_index(hn(3.25)?), None);
  assert_eq!(objs::object_to_integer_index(hn(-0.0)?), Some(0));
  OK
}

/// 规约 (objects-inl.h:1060-1107 GetSimpleHash 可承载子集)。
#[test]
fn simple_hash_semantics() -> Void {
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "hash");
  // Smi: 确定性 + 同值同哈希
  let h1 = objs::get_simple_hash(Smi::from_int(1234).cast())?;
  let h1b = objs::get_simple_hash(Smi::from_int(1234).cast())?;
  assert_eq!(h1, h1b);
  assert_ne!(h1, objs::get_simple_hash(Smi::from_int(1235).cast())?);
  // HeapNumber: NaN → Smi::MAX_VALUE; Signed32 与同值 Smi 哈希一致; 其余走 64 位
  let hn = |v: f64| zone_heap_number(&zone, InstanceType::HeapNumber, v);
  let hnan = objs::get_simple_hash(hn(f64::NAN)?)?;
  assert_eq!(Smi::value(hnan.cast::<Smi>()), Smi::MAX_VALUE);
  let hnum = objs::get_simple_hash(hn(1234.0)?)?;
  assert_eq!(hnum, h1); // 与 Smi 1234 哈希一致 (Signed32 复用)
  let hfrac = objs::get_simple_hash(hn(1.5e300)?)?;
  assert!(hfrac.is_smi());
  // -0 与 +0 在集合语义 (SameValueZero) 下同哈希
  assert_eq!(
    objs::get_simple_hash(hn(-0.0)?)?,
    objs::get_simple_hash(hn(0.0)?)?
  );
  assert_eq!(
    objs::get_simple_hash(hn(-0.0)?)?,
    objs::get_simple_hash(Smi::zero().cast())?
  );
  // JSReceiver 尾分支: 原样返回
  let obj = zone_bare(&zone, InstanceType::JSObject, false)?;
  assert_eq!(objs::get_simple_hash(obj)?, obj);
  // 推迟分支: Symbol / Name / Oddball / BigInt / SFI / Script
  for ty in [
    InstanceType::Symbol,
    InstanceType::Oddball,
    InstanceType::BigIntBase,
    InstanceType::SharedFunctionInfo,
    InstanceType::Script,
    InstanceType::SeqOneByteString,
  ] {
    assert!(
      objs::get_simple_hash(zone_bare(&zone, ty, false)?).is_err(),
      "{ty:?}"
    );
  }
  OK
}

/// 规约 (objects.h:750-764 BooleanBit)。
#[test]
fn boolean_bit() -> Void {
  assert!(objs::BooleanBit::get(0b101, 0));
  assert!(!objs::BooleanBit::get(0b101, 1));
  assert_eq!(objs::BooleanBit::set(0b101, 1, true), 0b111);
  assert_eq!(objs::BooleanBit::set(0b101, 0, false), 0b100);
  OK
}

/// 规约 (objects.h:98-104): stub key 位宽 = kSmiValueSize - major - 1。
/// 本 crate 64 位非压缩模型 kSmiValueSize = 32 → 次键 23 位。
#[test]
fn stub_key_bits() -> Void {
  assert_eq!(objs::K_STUB_MAJOR_KEY_BITS, 8);
  assert_eq!(objs::K_STUB_MINOR_KEY_BITS, 23);
  assert_eq!(objs::K_VARIABLE_SIZE_SENTINEL, 0);
  OK
}
