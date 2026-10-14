#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use v8_wasm_core::{wasm_features::WasmEnabledFeatures, wasm_tier::ExecutionTier};
mod error;
mod random_module_generation_impl;

pub use error::{Error, Result};
pub use random_module_generation_impl::{
  CallKind, IfType, IncludeAllGenerics, IncludeNumericTypes, IncludePackedTypes, IncludeS128,
  K_MAX_ARRAY_SIZE, K_MAX_ARRAYS, K_MAX_CATCH_CASES, K_MAX_EXCEPTIONS, K_MAX_GLOBALS, K_MAX_LOCALS,
  K_MAX_MEMORIES, K_MAX_PARAMETERS, K_MAX_PASSIVE_DATA_SEGMENTS, K_MAX_RECURSION_DEPTH,
  K_MAX_RETURNS, K_MAX_STRUCT_FIELDS, K_MAX_STRUCTS, K_MAX_TABLES, generator_alternative_counts,
};

/// 桥接: 从 Wasm 编译层级与特性判定是否启用 Turbofan 编译 (打通 Turbofan 优化编译入口)
#[inline]
#[must_use]
pub fn supports_turbofan_compilation(tier: ExecutionTier, _features: &WasmEnabledFeatures) -> bool {
  matches!(tier, ExecutionTier::Turbofan)
}
