//! WebAssembly Turbofan / Random Module Generation 集成测试
//! 对应原版 v8/src/wasm/fuzzing/random-module-generation.cc 规范

use aok::{OK, Void};
use v8_wasm_turbofan::{
  CallKind, IfType, IncludeAllGenerics, IncludeNumericTypes, IncludePackedTypes, IncludeS128,
  K_MAX_ARRAY_SIZE, K_MAX_ARRAYS, K_MAX_CATCH_CASES, K_MAX_EXCEPTIONS, K_MAX_GLOBALS, K_MAX_LOCALS,
  K_MAX_MEMORIES, K_MAX_PARAMETERS, K_MAX_PASSIVE_DATA_SEGMENTS, K_MAX_RECURSION_DEPTH,
  K_MAX_RETURNS, K_MAX_STRUCT_FIELDS, K_MAX_STRUCTS, K_MAX_TABLES, generator_alternative_counts,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_generator_constants() -> Void {
  assert_eq!(K_MAX_ARRAYS, 3);
  assert_eq!(K_MAX_STRUCTS, 4);
  assert_eq!(K_MAX_STRUCT_FIELDS, 4);
  assert_eq!(K_MAX_GLOBALS, 64);
  assert_eq!(K_MAX_LOCALS, 32);
  assert_eq!(K_MAX_PARAMETERS, 15);
  assert_eq!(K_MAX_RETURNS, 15);
  assert_eq!(K_MAX_EXCEPTIONS, 4);
  assert_eq!(K_MAX_TABLES, 4);
  assert_eq!(K_MAX_MEMORIES, 4);
  assert_eq!(K_MAX_ARRAY_SIZE, 20);
  assert_eq!(K_MAX_PASSIVE_DATA_SEGMENTS, 2);
  assert_eq!(K_MAX_RECURSION_DEPTH, 64);
  assert_eq!(K_MAX_CATCH_CASES, 6);
  OK
}

#[test]
fn test_configuration_enums() -> Void {
  // IncludeNumericTypes
  assert_eq!(IncludeNumericTypes::ExcludeNumericTypes as u8, 0);
  assert_eq!(IncludeNumericTypes::IncludeNumericTypes as u8, 1);
  assert_ne!(
    IncludeNumericTypes::ExcludeNumericTypes,
    IncludeNumericTypes::IncludeNumericTypes
  );

  // IncludePackedTypes
  assert_eq!(IncludePackedTypes::ExcludePackedTypes as u8, 0);
  assert_eq!(IncludePackedTypes::IncludePackedTypes as u8, 1);
  assert_ne!(
    IncludePackedTypes::ExcludePackedTypes,
    IncludePackedTypes::IncludePackedTypes
  );

  // IncludeAllGenerics
  assert_eq!(IncludeAllGenerics::ExcludeSomeGenerics as u8, 0);
  assert_eq!(IncludeAllGenerics::IncludeAllGenerics as u8, 1);
  assert_ne!(
    IncludeAllGenerics::ExcludeSomeGenerics,
    IncludeAllGenerics::IncludeAllGenerics
  );

  // IncludeS128
  assert_eq!(IncludeS128::ExcludeS128 as u8, 0);
  assert_eq!(IncludeS128::IncludeS128 as u8, 1);
  assert_ne!(IncludeS128::ExcludeS128, IncludeS128::IncludeS128);

  // IfType
  assert_eq!(IfType::If as u8, 0);
  assert_eq!(IfType::IfElse as u8, 1);
  assert_ne!(IfType::If, IfType::IfElse);

  // CallKind
  assert_eq!(CallKind::CallDirect as u8, 0);
  assert_eq!(CallKind::CallIndirect as u8, 1);
  assert_eq!(CallKind::CallRef as u8, 2);
  assert_ne!(CallKind::CallDirect, CallKind::CallIndirect);
  assert_ne!(CallKind::CallIndirect, CallKind::CallRef);
  assert_ne!(CallKind::CallDirect, CallKind::CallRef);

  OK
}

#[test]
fn test_generator_alternative_counts() -> Void {
  // 零输入边界
  let (simd, wasmgc, all) = generator_alternative_counts(0, 0, 0);
  assert_eq!(simd, 0);
  assert_eq!(wasmgc, 0);
  assert_eq!(all, 0);

  // 典型输入
  // num_mvp = 10, additional_simd = 5, additional_wasmgc = 3
  // num_simd = 10 + 5 = 15
  // num_wasmgc = 10 + 3 = 13
  // num_all = 15 + 3 = 18
  let (simd, wasmgc, all) = generator_alternative_counts(10, 5, 3);
  assert_eq!(simd, 15);
  assert_eq!(wasmgc, 13);
  assert_eq!(all, 18);

  // 纯 MVP
  let (simd, wasmgc, all) = generator_alternative_counts(7, 0, 0);
  assert_eq!(simd, 7);
  assert_eq!(wasmgc, 7);
  assert_eq!(all, 7);

  // const evaluation check
  const COUNTS: (usize, usize, usize) = generator_alternative_counts(4, 2, 3);
  assert_eq!(COUNTS.0, 6);
  assert_eq!(COUNTS.1, 7);
  assert_eq!(COUNTS.2, 9);

  OK
}

#[test]
fn test_supports_turbofan_compilation() -> Void {
  use v8_wasm_core::{
    wasm_features::{WasmEnabledFeature, WasmEnabledFeatures},
    wasm_tier::ExecutionTier,
  };
  use v8_wasm_turbofan::supports_turbofan_compilation;

  let empty_features = WasmEnabledFeatures::none();
  let mut shared_features = WasmEnabledFeatures::none();
  shared_features.add(WasmEnabledFeature::Shared);

  // Turbofan 编译入口应支持 Turbofan 层级，无论是否开启实验性 Shared 特性
  assert!(supports_turbofan_compilation(
    ExecutionTier::Turbofan,
    &empty_features
  ));
  assert!(supports_turbofan_compilation(
    ExecutionTier::Turbofan,
    &shared_features
  ));

  // 其余层级不走 Turbofan 编译
  assert!(!supports_turbofan_compilation(
    ExecutionTier::Liftoff,
    &empty_features
  ));
  assert!(!supports_turbofan_compilation(
    ExecutionTier::None,
    &empty_features
  ));
  OK
}
