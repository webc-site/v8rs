// 由 tool/src/objects/option-utils.h.js 从 v8/src/objects/option-utils.{h,cc} 生成; 修订就地改, 复跑覆盖
// ECMA402 GetOption 工具族: options 对象读取/校验/回退骨架
//! option-utils: Intl/Temporal 共用的 options 对象工具族 (ECMA402 9.2.10)。
//!
//! 读取 → ToString/ToBoolean → 枚举匹配 (str_values ↔ enum_values 等长)
//! → 未命中抛 RangeError(kValueOutOfRange) → 回退默认值。
//!
//! Isolate 依赖面 (属性 Get / ToString / ToBoolean / ToNumber / 抛异常落桩)
//! 按推迟台账登记 (readme/zh.md); 本体积已就位的纯语义核 1:1 转写:
//! 数值校验与枚举匹配按原版函数体的"已取值"后置形态承接。

use crate::error::{Error, Result};

/// 原版 "true"/"false" 工厂字符串 (option-utils.cc/h 判定用)。
const K_TRUE_STR: &str = "true";
const K_FALSE_STR: &str = "false";

// ── 函数签名骨架 (cc 实现序 + h 模板 2; 深水推迟台账) ──
/// v8: src/objects/option-utils.cc GetOptionsObject。
pub fn get_options_object() {
  todo!(
    "GetOptionsObject: 依赖 Isolate + factory.new_empty_object + Object 类型判定 + TypeError(kInvalidArgument) 抛出; 解锁 v8-execution-isolate + v8-objects-js-core"
  )
}

/// v8: src/objects/option-utils.cc CoerceOptionsToObject。
pub fn coerce_options_to_object() {
  todo!(
    "CoerceOptionsToObject: 依赖 Isolate + Object::ToObject + TypeError 抛出; 解锁 v8-execution-isolate + v8-objects-js-core"
  )
}

/// v8: src/objects/option-utils.cc GetStringOption。
pub fn get_string_option() {
  todo!(
    "GetStringOption: 依赖 Isolate + JSReceiver::Get + Object::ToString + 原生 %String% realm; 解锁 v8-execution-isolate + v8-objects-js-core"
  )
}

/// v8: src/objects/option-utils.cc GetBoolOption。
pub fn get_bool_option() {
  todo!(
    "GetBoolOption: 依赖 Isolate + JSReceiver::Get + Object::BooleanValue; 解锁 v8-execution-isolate + v8-objects-js-core"
  )
}

/// 原版 DefaultNumberOption (option-utils.cc; h 声明)。
///
/// v8: src/objects/option-utils.cc DefaultNumberOption
///
/// `value` = 已 ? ToNumber 的取值 (None = undefined, 对应原版 IsUndefined 早返);
/// NaN 或出 [min, max] 区间 → RangeError(kPropertyValueOutOfRange);
/// 否则 floor 收敛为整 (原版注释: 区间校验保证 double→int 安全)。
pub fn default_number_option(
  value: Option<f64>,
  min: i32,
  max: i32,
  fallback: i32,
  property: &'static str,
) -> Result<i32> {
  // 2. Else, return fallback.
  let Some(number) = value else {
    return Ok(fallback);
  };
  // b. If value is NaN or less than minimum or greater than maximum, throw RangeError.
  if number.is_nan() || number < min as f64 || number > max as f64 {
    return Err(Error::PropertyValueOutOfRange(property));
  }
  // c. Return floor(value).
  Ok(number.floor() as i32)
}

/// 原版 GetNumberOption (option-utils.cc; h 声明)。
///
/// v8: src/objects/option-utils.cc GetNumberOption
/// 函数体 = Get(options, property) + DefaultNumberOption; Get 面按推迟台账
/// 由调用方承接, 此处 1:1 转发校验核。
pub fn get_number_option(
  value: Option<f64>,
  min: i32,
  max: i32,
  fallback: i32,
  property: &'static str,
) -> Result<i32> {
  default_number_option(value, min, max, fallback, property)
}

/// 原版 GetNumberOptionAsDouble (option-utils.cc; h 声明)。
///
/// v8: src/objects/option-utils.cc GetNumberOptionAsDouble
/// (ECMA262 GetOption type="number"): undefined→default; 已取值为 NaN →
/// RangeError(kPropertyValueOutOfRange); 否则原值返回。
pub fn get_number_option_as_double(
  value: Option<f64>,
  default_value: f64,
  property: &'static str,
) -> Result<f64> {
  // 2. If value is undefined, then b. Return default.
  let Some(number) = value else {
    return Ok(default_value);
  };
  // 4.b If value is NaN, throw a RangeError exception.
  if number.is_nan() {
    return Err(Error::PropertyValueOutOfRange(property));
  }
  // 7. Return value.
  Ok(number)
}

/// 原版 GetStringOption<T> 模板 (option-utils.h: 字符串 → 枚举匹配, RangeError 回退)。
///
/// v8: src/objects/option-utils.h GetStringOption<T>
/// `found` = Some(已 ? ToString 的属性值) / None (属性 undefined)。
/// 前置: str_values 与 enum_values 等长 (原版 DCHECK_EQ);
/// 命中 → 对应枚举; 未取到值且有 default → default; 其余 →
/// RangeError(kValueOutOfRange) (消息模板携 value/method/property, 上层落桩)。
pub fn get_string_option_enum<T: Copy>(
  found: Option<&str>,
  str_values: &[&str],
  enum_values: &[T],
  default_value: Option<T>,
  property: &'static str,
) -> Result<T> {
  debug_assert_eq!(str_values.len(), enum_values.len());
  if let Some(found_string) = found {
    // 2. d. if values is not undefined, then
    for (i, candidate) in str_values.iter().enumerate() {
      // found_string->IsEqualTo(str_values[i], isolate)
      if *candidate == found_string {
        return Ok(enum_values[i]);
      }
    }
  } else if let Some(default) = default_value {
    // 2. If value is undefined, then b. Return default.
    return Ok(default);
  }
  // 2. d. i. If values does not contain an element equal to value, throw RangeError.
  Err(Error::ValueOutOfRange(property))
}

/// 属性值谓词分解形态 (对应 GetStringOrBooleanOption 函数体内
/// IsUndefined / IsTrue / BooleanValue / ToString 四级判定)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoercedOptionValue<'a> {
  /// undefined
  Undefined,
  /// `true` (原版 IsTrue 短路)
  True,
  /// ToBoolean 为 false (undefined 之外)
  Falsy,
  /// 其余 truthy 值的 ? ToString 结果
  Str(&'a str),
}

/// 原版 GetStringOrBooleanOption<T> 模板 (option-utils.h: undefined 回退 / true→true_value / false→false_value)。
///
/// v8: src/objects/option-utils.h GetStringOrBooleanOption<T>
pub fn get_string_or_boolean_option<T: Copy>(
  value: CoercedOptionValue<'_>,
  str_values: &[&str],
  enum_values: &[T],
  true_value: T,
  false_value: T,
  fallback_value: T,
  property: &'static str,
) -> Result<T> {
  debug_assert_eq!(str_values.len(), enum_values.len());
  let s = match value {
    // 2. If value is undefined, then return fallback.
    CoercedOptionValue::Undefined => return Ok(fallback_value),
    // 3. If value is true, then return trueValue.
    CoercedOptionValue::True => return Ok(true_value),
    // 4.-5. ToBoolean(value) 为 false → false_value。
    CoercedOptionValue::Falsy => return Ok(false_value),
    // 6. Let value be ? ToString(value).
    CoercedOptionValue::Str(s) => s,
  };
  // 7. If value is "true" or "false", return fallback.
  if s == K_TRUE_STR || s == K_FALSE_STR {
    return Ok(fallback_value);
  }
  // 8.-9. 枚举匹配, 未命中抛 RangeError(kValueOutOfRange)。
  for (i, candidate) in str_values.iter().enumerate() {
    if *candidate == s {
      return Ok(enum_values[i]);
    }
  }
  Err(Error::ValueOutOfRange(property))
}

// ── 提取不变式断言 ──
const _: () = {
  // 工具族 9 面: cc 实现 7 + h 模板 2
  assert!(K_OPTION_UTILS_FN_COUNT == 9);
};

/// 工具族函数面总数 (cc 7 + h 模板 2)。
pub const K_OPTION_UTILS_FN_COUNT: usize = 9;
