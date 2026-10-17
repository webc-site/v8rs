// 由 tool/src/wasm/fuzzing/random-module-generation.cc.js 从 v8/src/wasm/fuzzing/random-module-generation.cc 生成; 修订就地改, 复跑覆盖
// 随机 wasm 模块生成器规则面: 上限常量与备选公式

//! 生成器语义面 (随机流消费次序) 由 Developer 就地修订。

/// 生成上限常量 (原版 constexpr kMax* 族)
/// v8: src/wasm/fuzzing/random-module-generation.cc
pub const K_MAX_ARRAYS: i32 = 3;
pub const K_MAX_STRUCTS: i32 = 4;
pub const K_MAX_STRUCT_FIELDS: i32 = 4;
pub const K_MAX_GLOBALS: i32 = 64;
pub const K_MAX_LOCALS: u32 = 32;
pub const K_MAX_PARAMETERS: i32 = 15;
pub const K_MAX_RETURNS: i32 = 15;
pub const K_MAX_EXCEPTIONS: i32 = 4;
pub const K_MAX_TABLES: i32 = 4;
pub const K_MAX_MEMORIES: i32 = 4;
pub const K_MAX_ARRAY_SIZE: i32 = 20;
pub const K_MAX_PASSIVE_DATA_SEGMENTS: i32 = 2;
pub const K_MAX_RECURSION_DEPTH: u32 = 64;
pub const K_MAX_CATCH_CASES: i32 = 6;

/// 数值类型包含开关 (原版 enum IncludeNumericTypes : 隐式 bool)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IncludeNumericTypes {
  /// kExcludeNumericTypes
  ExcludeNumericTypes = 0,
  /// kIncludeNumericTypes
  IncludeNumericTypes = 1,
}

/// 打包类型包含开关 (原版 enum IncludePackedTypes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IncludePackedTypes {
  /// kExcludePackedTypes
  ExcludePackedTypes = 0,
  /// kIncludePackedTypes
  IncludePackedTypes = 1,
}

/// 泛型类别包含开关 (原版 enum IncludeAllGenerics)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IncludeAllGenerics {
  /// kExcludeSomeGenerics
  ExcludeSomeGenerics = 0,
  /// kIncludeAllGenerics
  IncludeAllGenerics = 1,
}

/// s128 包含开关 (原版 enum IncludeS128)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IncludeS128 {
  /// kExcludeS128
  ExcludeS128 = 0,
  /// kIncludeS128
  IncludeS128 = 1,
}

/// if 形态 (原版 enum IfType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IfType {
  /// kIf
  If = 0,
  /// kIfElse
  IfElse = 1,
}

/// 调用形态 (原版 enum CallKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CallKind {
  /// kCallDirect
  CallDirect = 0,
  /// kCallIndirect
  CallIndirect = 1,
  /// kCallRef
  CallRef = 2,
}

/// 生成器备选数组尺寸公式 (原版 GeneratorAlternativesPerOption 模板参数关系)
/// v8: src/wasm/fuzzing/random-module-generation.cc
pub const fn generator_alternative_counts(
  num_mvp: usize,
  additional_simd: usize,
  additional_wasmgc: usize,
) -> (usize, usize, usize) {
  let num_simd = num_mvp + additional_simd;
  let num_wasmgc = num_mvp + additional_wasmgc;
  let num_all = num_simd + additional_wasmgc;
  (num_simd, num_wasmgc, num_all)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_GLOBALS == 64 && K_MAX_PARAMETERS == 15);
  assert!(K_MAX_RECURSION_DEPTH == 64 && K_MAX_CATCH_CASES == 6);
  // mvp=4, simd=2, gc=3 → simd 选项 6, gc 选项 7, all 9
  assert!(IfType::IfElse as u8 == 1);
  assert!(CallKind::CallRef as u8 == 2);
};
