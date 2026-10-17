#!/usr/bin/env bun
// tool/src/objects/objects.cc.js — 覆盖 v8/src/objects/objects.cc 的语义子集
//   → v8-objects-core/src/objects.rs 的 @cc-semantics 区段 (区段标记替换, 幂等;
//     不触碰 objects.h.js 生成的其余内容)
//
// 转写区段登记 (objects.cc 实测行号):
//   144-160  ComparisonResultToBool (Operation + ComparisonResult → bool)
//   813-826  Object::BooleanValue (Smi/Boolean/NullUndefined/Undetectable/
//            HeapNumber 分支; String/BigInt 分支推迟)
//   840-876  StrictNumberCompare / StrictNumberEquals / Reverse (匿名命名空间)
//   1026-1038 Object::StrictEquals (按 obj 分派; 单边 String/BigInt → false; 双侧同型 → 内容级相等推迟)
//   1538-1552 Object::ToInt32
//   1799-1834 Object::SameValue / SameValueZero
//   objects-inl.h:631-643 Object::SameNumberValue
//   objects-inl.h:1060-1107 Object::GetSimpleHash (Smi/HeapNumber/JSReceiver 尾分支;
//            Name/Oddball/BigInt/SFI/ScopeInfo/Script/TemplateInfo 分支推迟)
//   object-conversions-inl.h:18-85 NumberValue/ToUint32/ToArrayLength/ToArrayIndex/
//            ToIntegerIndex
//
// 跳过区段登记 (推迟, 归属见 objects.h.js 头注释):
//   objects.cc 其余全部: isolate/handle 依赖的转换 (ToObject/ToPrimitive/...),
//   查找语义 (GetProperty/SetProperty/LookupIterator), 打印 (归 L4 v8-diagnostics),
//   HeapObject::SizeFromMap/Rehash (归 v8-objects-maps/heap)
import { banner, readV8 } from "../../lib.js";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../..");
const target = join(root, "v8-objects-core/src/objects.rs");
// 源文件存在性对数 (转写区段锚点)
for (const anchor of [
  ["src/objects/objects.cc", "bool ComparisonResultToBool(Operation op, ComparisonResult result) {"],
  ["src/objects/objects.cc", "bool Object::StrictEquals(Tagged<Object> obj, Tagged<Object> that) {"],
  ["src/objects/objects-inl.h", "bool Object::SameNumberValue(double value1, double value2) {"],
  ["src/objects/object-conversions-inl.h", "bool Object::ToIntegerIndex(Tagged<Object> obj, size_t* index) {"],
]) {
  if (!readV8(anchor[0]).includes(anchor[1])) throw new Error(`锚点失配: ${anchor[0]} 无 "${anchor[1].slice(0, 40)}..."`);
}

const semantics = `${banner("src/objects/objects.cc.js", "src/objects/objects.cc 语义子集 + object-conversions-inl.h", "转写/跳过区段登记见脚本头注释")}
// 外部依赖在此引入; crate 内符号沿用文件头部导入 (区段不重复 use)
use v8_common::Operation;
use v8_numbers::conversions::double_to_uint32_if_equal_to_self;
use v8_utils::utils::{smi_hash32, smi_hash64};

// ── 比较语义 (objects.cc:144-160, 840-876) ────────────────────────────────

/// 原版 ComparisonResultToBool (objects.cc:144)。
/// 原版 default 分支 UNREACHABLE → 禁 panic, 落 Err。
#[inline]
pub fn comparison_result_to_bool(op: Operation, result: ComparisonResult) -> Result<bool> {
  match op {
    Operation::LessThan => Ok(result == ComparisonResult::LessThan),
    Operation::LessThanOrEqual => Ok(matches!(result, ComparisonResult::LessThan | ComparisonResult::Equal)),
    Operation::GreaterThan => Ok(result == ComparisonResult::GreaterThan),
    Operation::GreaterThanOrEqual => Ok(matches!(result, ComparisonResult::GreaterThan | ComparisonResult::Equal)),
    _ => Err(Error::Unsupported("ComparisonResultToBool: 非关系运算")),
  }
}

/// 原版 StrictNumberCompare (objects.cc:840): NaN 参与 → Undefined。
#[inline]
#[must_use]
pub fn strict_number_compare(x: f64, y: f64) -> ComparisonResult {
  if x.is_nan() || y.is_nan() {
    ComparisonResult::Undefined
  } else if x < y {
    ComparisonResult::LessThan
  } else if x > y {
    ComparisonResult::GreaterThan
  } else {
    ComparisonResult::Equal
  }
}

/// 原版 StrictNumberEquals (objects.cc:854): NaN 不等, -0 == +0。
#[inline]
#[must_use]
pub fn strict_number_equals(x: f64, y: f64) -> bool {
  if x.is_nan() || y.is_nan() {
    return false;
  }
  x == y
}

/// 原版 Reverse (objects.cc:868): 翻转大小关系。
#[inline]
#[must_use]
pub const fn reverse(result: ComparisonResult) -> ComparisonResult {
  match result {
    ComparisonResult::LessThan => ComparisonResult::GreaterThan,
    ComparisonResult::GreaterThan => ComparisonResult::LessThan,
    _ => result,
  }
}

// ── Object 数值语义 (object-conversions-inl.h:18-85, objects.cc:1538-1552) ──

/// 原版 Object::NumberValue (object-conversions-inl.h:19): Smi 或 HeapNumber → f64。
/// 非 Number → 原版 DCHECK 崩溃路径, 此处落 Err (禁 panic)。
#[inline]
pub fn number_value(obj: Tagged<Object>) -> Result<f64> {
  if obj.is_smi() {
    return Ok(f64::from(Smi::value(obj.cast::<Smi>())));
  }
  let ty = instance_type_of(obj).ok_or(Error::Unsupported("NumberValue: 非堆对象"))?;
  if !itc::is_heap_number(ty) {
    return Err(Error::Unsupported("NumberValue: 非 Number"));
  }
  // SAFETY: 实例类型已验证为 HeapNumber, 读 value 字段只读。
  unsafe { Ok((*obj.cast::<HeapNumber>().as_ptr()).value) }
}

/// 原版 Object::ToInt32 (objects.cc:1538)。
#[inline]
#[must_use]
pub fn object_to_int32(obj: Tagged<Object>) -> Option<i32> {
  if obj.is_smi() {
    return Some(Smi::value(obj.cast::<Smi>()));
  }
  if instance_type_of(obj).is_some_and(itc::is_heap_number) {
    // SAFETY: 已验证 HeapNumber。
    let num = unsafe { (*obj.cast::<HeapNumber>().as_ptr()).value };
    // 先查范围再转换, 避免浮点→整数 UB; 回转校验无损。
    if num >= i32::MIN as f64 && num <= i32::MAX as f64 && f64::from(num as i32) == num {
      return Some(num as i32);
    }
  }
  None
}

/// 原版 Object::ToUint32 (object-conversions-inl.h:39)。
#[inline]
#[must_use]
pub fn object_to_uint32(obj: Tagged<Object>) -> Option<u32> {
  if obj.is_smi() {
    let num = Smi::value(obj.cast::<Smi>());
    return if num < 0 { None } else { Some(num as u32) };
  }
  if instance_type_of(obj).is_some_and(itc::is_heap_number) {
    // SAFETY: 已验证 HeapNumber。
    let num = unsafe { (*obj.cast::<HeapNumber>().as_ptr()).value };
    return double_to_uint32_if_equal_to_self(num);
  }
  None
}

/// 原版 Object::ToArrayLength (object-conversions-inl.h:53): 允许 kMaxUInt32。
#[inline]
#[must_use]
pub fn object_to_array_length(obj: Tagged<Object>) -> Option<u32> {
  object_to_uint32(obj)
}

/// 原版 Object::ToArrayIndex (object-conversions-inl.h:59): 不允许 kMaxUInt32。
#[inline]
#[must_use]
pub fn object_to_array_index(obj: Tagged<Object>) -> Option<u32> {
  object_to_uint32(obj).filter(|&v| v != u32::MAX)
}

/// 原版 Object::ToIntegerIndex (object-conversions-inl.h:64): 0..usize::MAX-1。
#[inline]
#[must_use]
pub fn object_to_integer_index(obj: Tagged<Object>) -> Option<usize> {
  if obj.is_smi() {
    let num = Smi::value(obj.cast::<Smi>());
    return if num < 0 { None } else { Some(num as usize) };
  }
  if instance_type_of(obj).is_some_and(itc::is_heap_number) {
    // SAFETY: 已验证 HeapNumber。
    let num = unsafe { (*obj.cast::<HeapNumber>().as_ptr()).value };
    // NaN 与负数同拒 (原版 !(num >= 0))。
    if num.is_nan() || num < 0.0 {
      return None;
    }
    // usize::MAX 保留为非法哨兵; 上界同时受 kMaxSafeInteger 约束。
    const MAX: f64 = 9_007_199_254_740_991.0; // 2^53 - 1
    let max = MAX.min((usize::MAX - 1) as f64);
    if num > max {
      return None;
    }
    let result = num as usize;
    if num != result as f64 {
      return None; // 丢失小数精度
    }
    return Some(result);
  }
  None
}

// ── Object 相等语义 (objects.cc:1026-1038, 1799-1834; objects-inl.h:631-643) ──

/// 原版 Object::SameNumberValue (objects-inl.h:631):
/// 位级比较覆盖 -0 != +0; 不同位模式的 NaN 互等。
#[inline]
#[must_use]
pub fn same_number_value(number1: f64, number2: f64) -> bool {
  if number1.to_bits() == number2.to_bits() {
    return true;
  }
  number1.is_nan() && number2.is_nan()
}

/// Object 是否 Number (Smi 或 HeapNumber)。
#[inline]
#[must_use]
fn is_number(obj: Tagged<Object>) -> bool {
  obj.is_smi() || instance_type_of(obj).is_some_and(itc::is_heap_number)
}

/// 两侧同为 String 或同为 BigInt → 内容级相等推迟 (归 v8-objects-strings / v8-bigint
/// 集成), 落 Err; 单边 String/BigInt 原版落 return false, 不推迟。
#[inline]
fn defer_if_both_string_or_bigint(
  obj: Tagged<Object>,
  other: Tagged<Object>,
  who: &'static str,
) -> Result<()> {
  let both_string =
    instance_type_of(obj).is_some_and(itc::is_string)
      && instance_type_of(other).is_some_and(itc::is_string);
  let both_bigint =
    instance_type_of(obj).is_some_and(itc::is_big_int)
      && instance_type_of(other).is_some_and(itc::is_big_int);
  if both_string || both_bigint {
    return Err(Error::Unsupported(who));
  }
  Ok(())
}

/// 原版 Object::StrictEquals (objects.cc:1026-1038): 按 obj 类型分派。
/// Number 双侧判等; 单边 String/BigInt → false; 同为 String/BigInt → 内容级相等推迟
/// 落 Err; 其余落指针相等 (obj == that)。
#[inline]
pub fn object_strict_equals(obj: Tagged<Object>, that: Tagged<Object>) -> Result<bool> {
  if is_number(obj) {
    if !is_number(that) {
      return Ok(false);
    }
    return Ok(strict_number_equals(number_value(obj)?, number_value(that)?));
  }
  if instance_type_of(obj).is_some_and(itc::is_string) {
    if !instance_type_of(that).is_some_and(itc::is_string) {
      return Ok(false);
    }
    return Err(Error::Unsupported("StrictEquals: String 内容级相等"));
  }
  if instance_type_of(obj).is_some_and(itc::is_big_int) {
    if !instance_type_of(that).is_some_and(itc::is_big_int) {
      return Ok(false);
    }
    return Err(Error::Unsupported("StrictEquals: BigInt 内容级相等"));
  }
  Ok(obj == that)
}

/// 原版 Object::SameValue (objects.cc:1799-1815; Object.is 语义)。
/// String/BigInt 分支同上推迟 (仅两侧同型)。
#[inline]
pub fn object_same_value(obj: Tagged<Object>, other: Tagged<Object>) -> Result<bool> {
  if other == obj {
    return Ok(true);
  }
  if is_number(obj) && is_number(other) {
    return Ok(same_number_value(number_value(obj)?, number_value(other)?));
  }
  defer_if_both_string_or_bigint(obj, other, "SameValue: String/BigInt 内容级相等")?;
  Ok(false)
}

/// 原版 Object::SameValueZero (objects.cc:1817-1834; +0 == -0, NaN 互等)。
#[inline]
pub fn object_same_value_zero(obj: Tagged<Object>, other: Tagged<Object>) -> Result<bool> {
  if other == obj {
    return Ok(true);
  }
  if is_number(obj) && is_number(other) {
    let a = number_value(obj)?;
    let b = number_value(other)?;
    return Ok(a == b || (a.is_nan() && b.is_nan()));
  }
  defer_if_both_string_or_bigint(obj, other, "SameValueZero: String/BigInt 内容级相等")?;
  Ok(false)
}

// ── Object 布尔语义 (objects.cc:813-826) ──────────────────────────────────

/// 原版 Object::BooleanValue (objects.cc:813)。
/// String (length) / BigInt (ToBoolean) 分支推迟, 落 Err;
/// undetectable 经 Map 位域判定 (非 isolate 依赖)。
#[inline]
pub fn object_boolean_value(obj: Tagged<Object>) -> Result<bool> {
  if obj.is_smi() {
    return Ok(Smi::value(obj.cast::<Smi>()) != 0);
  }
  let Some(ho) = obj.to_heap_object() else {
    return Err(Error::Unsupported("BooleanValue: 非堆对象"));
  };
  // SAFETY: 堆对象 tag 已验证; Map 一次读取, instance_type/kind/位域均为只读字段。
  let (ty, undetectable) = unsafe {
    let map: Tagged<Map> = (*ho.as_ptr()).map;
    let m = &*map.as_ptr();
    (m.instance_type, m.is_undetectable())
  };
  if itc::is_oddball(ty) {
    // SAFETY: 已验证 Oddball; 读 kind 字段只读。
    let kind = unsafe { (*obj.cast::<crate::oddball::Oddball>().as_ptr()).kind };
    return match kind {
      crate::oddball::OddballKind::True => Ok(true),
      // 原版: IsBoolean(true)/IsNullOrUndefined(false) 逐行对齐;
      // OddballKind 恰 {True,False,Null,Undefined} 四变体, 全覆盖 (TheHole 非 Oddball)
      crate::oddball::OddballKind::False
      | crate::oddball::OddballKind::Null
      | crate::oddball::OddballKind::Undefined => Ok(false),
    };
  }
  if itc::is_string(ty) {
    return Err(Error::Unsupported("BooleanValue: String length"));
  }
  if itc::is_big_int(ty) {
    return Err(Error::Unsupported("BooleanValue: BigInt ToBoolean"));
  }
  if itc::is_heap_number(ty) {
    // SAFETY: 已验证 HeapNumber。
    let num = unsafe { (*obj.cast::<HeapNumber>().as_ptr()).value };
    // DoubleToBoolean: NaN 与 0 (含 -0) 为假。
    return Ok(!num.is_nan() && num != 0.0);
  }
  // Undetectable 对象为假 (原版 821 行)。
  Ok(!undetectable)
}

// ── Object 哈希语义 (objects-inl.h:1060-1107) ─────────────────────────────

/// 原版 Object::GetSimpleHash (objects-inl.h:1060) 的可承载子集:
/// Smi / HeapNumber 分支 1:1; JSReceiver 尾分支原样返回对象自身;
/// Name/Oddball/BigInt/SharedFunctionInfo/ScopeInfo/Script/TemplateInfo
/// 的 EnsureHash/Hash 分支推迟 (归各对象族 crate), 落 Err。
#[inline]
pub fn get_simple_hash(obj: Tagged<Object>) -> Result<Tagged<Object>> {
  if obj.is_smi() {
    let h = smi_hash32(Smi::value(obj.cast::<Smi>()) as u32);
    return Ok(Tagged::new(Smi::from_int(h as i32).ptr()));
  }
  let Some(ty) = instance_type_of(obj) else {
    return Err(Error::Unsupported("GetSimpleHash: 非堆对象"));
  };
  if itc::is_heap_number(ty) {
    // SAFETY: 已验证 HeapNumber。
    let num = unsafe { (*obj.cast::<HeapNumber>().as_ptr()).value };
    if num.is_nan() {
      return Ok(Tagged::new(Smi::from_int(Smi::MAX_VALUE).ptr()));
    }
    // Signed32 范围 (含 -0, 集合语义下 -0 == 0) 复用 Smi 哈希。
    let h = if num >= i32::MIN as f64 && num <= i32::MAX as f64 && f64::from(num as i32) == num {
      smi_hash32(num as i32 as u32)
    } else {
      smi_hash64(num.to_bits())
    };
    return Ok(Tagged::new(Smi::from_int(h as i32).ptr()));
  }
  let deferred = matches!(
    ty,
    InstanceType::Symbol
      | InstanceType::BigIntBase
      | InstanceType::Oddball
      | InstanceType::SharedFunctionInfo
      | InstanceType::ScopeInfo
      | InstanceType::Script
  ) || itc::is_template_info(ty)
    || itc::is_name(ty) && !itc::is_symbol(ty);
  if deferred {
    return Err(Error::Unsupported("GetSimpleHash: Name/Oddball/BigInt 等分支"));
  }
  // 其余 (JSReceiver): 原版返回对象自身 (调用方走 GetIdentityHash)。
  Ok(obj)
}
`;

// 区段标记替换 (幂等)
const cur = readFileSync(target, "utf8");
const BEGIN = "// @cc-semantics-begin";
const END = "// @cc-semantics-end";
const bi = cur.indexOf(BEGIN);
const ei = cur.indexOf(END);
if (bi < 0 || ei < 0) throw new Error("objects.rs 缺少 @cc-semantics 区段标记 (先跑 objects.h.js)");
const next = cur.slice(0, bi + BEGIN.length) + "\n" + semantics + "\n" + cur.slice(ei);
writeFileSync(target, next);
console.log("objects.rs @cc-semantics 区段已注入: 比较/数值/相等/布尔/哈希语义");
