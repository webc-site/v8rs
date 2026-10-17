//! JS 算子前端差集 — ForInMode / JSWasmCallParameters / CallFrequency
//!
//! 算子工厂主面 (JSOperatorBuilder, CallParameters, ConstructParameters) 冻结于
//! v8-compiler-ops; 本模块承接 v8/src/compiler/js-operator.{h,cc} 中前端直接
//! 消费的参数/枚举类型。

use std::fmt;

/// for-in 遍历模式
/// v8: src/compiler/js-operator.h ForInMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ForInMode {
  /// 枚举缓存 keys+indices
  /// v8: src/compiler/js-operator.h ForInMode::kUseEnumCacheKeysAndIndices
  UseEnumCacheKeysAndIndices = 0,
  /// 枚举缓存 keys
  /// v8: src/compiler/js-operator.h ForInMode::kUseEnumCacheKeys
  UseEnumCacheKeys = 1,
  /// 通用
  /// v8: src/compiler/js-operator.h ForInMode::kGeneric
  Generic = 2,
}

/// 调用频率 — float 封装, NaN 表未知
/// v8: src/compiler/js-operator.h CallFrequency
#[derive(Clone, Copy)]
pub struct CallFrequency {
  /// value_ (默认 NaN = 未知)
  pub value: f32,
}

impl Default for CallFrequency {
  /// v8: src/compiler/js-operator.h CallFrequency::CallFrequency()
  fn default() -> Self {
    Self { value: f32::NAN }
  }
}

impl CallFrequency {
  /// v8: src/compiler/js-operator.h CallFrequency::CallFrequency(float value)
  pub const fn new(value: f32) -> Self {
    debug_assert!(!value.is_nan(), "显式构造禁 NaN");
    Self { value }
  }

  /// v8: src/compiler/js-operator.h CallFrequency::IsKnown
  pub const fn is_known(&self) -> bool {
    !self.value.is_nan()
  }

  /// v8: src/compiler/js-operator.h CallFrequency::IsUnknown
  pub const fn is_unknown(&self) -> bool {
    self.value.is_nan()
  }

  /// v8: src/compiler/js-operator.h CallFrequency::value
  pub const fn value(&self) -> f32 {
    debug_assert!(self.is_known(), "value() 要求已知频率");
    self.value
  }

  /// kNoFeedbackCallFrequency = -1
  /// v8: src/compiler/js-operator.h CallFrequency::kNoFeedbackCallFrequency
  pub const NO_FEEDBACK: f32 = -1.0;

  /// hash — bit_cast<uint32>(value_)
  /// v8: src/compiler/js-operator.h hash_value(CallFrequency)
  pub const fn hash_value(&self) -> usize {
    self.value.to_bits() as usize
  }
}

impl PartialEq for CallFrequency {
  /// 原版按位比较 (NaN==NaN)
  /// v8: src/compiler/js-operator.h CallFrequency::operator==
  fn eq(&self, other: &Self) -> bool {
    self.value.to_bits() == other.value.to_bits()
  }
}
impl Eq for CallFrequency {}

/// v8: src/compiler/js-operator.h CallFrequency operator<<
impl fmt::Debug for CallFrequency {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "CallFrequency({})", self.value)
  }
}

/// wasm 调用算子参数 — native module 指针 + slot + signature
/// v8: src/compiler/js-operator.h JSWasmCallParameters
#[derive(Debug, Clone, Copy)]
pub struct JSWasmCallParameters {
  /// native_module_ (原版 wasm::NativeModule*, 执行面对象)
  pub native_module: *const (),
  /// slot_ (uint64)
  pub slot: u64,
  /// signature_ (wasm::Signature sig_index)
  pub signature: i32,
}

// —— 对数基准 ——
// ForInMode 3 项 / cc 工厂定义面 62 项 (归 v8-compiler-ops JSOperatorBuilder)
