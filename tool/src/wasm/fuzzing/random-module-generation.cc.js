#!/usr/bin/env bun
// tool/src/wasm/fuzzing/random-module-generation.cc.js
// 机械提取 v8/src/wasm/fuzzing/random-module-generation.cc 的
// 随机模块生成上限常量、包含类别枚举、生成器备选公式与调用形态

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/wasm/fuzzing/random-module-generation.cc");

function must(re, what) {
  const m = ccSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// —— 1. 上限常量表 (名字 → 期望值) ——
const kMaxConsts = [
  ["kMaxArrays", 3, "int"],
  ["kMaxStructs", 4, "int"],
  ["kMaxStructFields", 4, "int"],
  ["kMaxGlobals", 64, "int"],
  ["kMaxLocals", 32, "uint32_t"],
  ["kMaxParameters", 15, "int"],
  ["kMaxReturns", 15, "int"],
  ["kMaxExceptions", 4, "int"],
  ["kMaxTables", 4, "int"],
  ["kMaxMemories", 4, "int"],
  ["kMaxArraySize", 20, "int"],
  ["kMaxPassiveDataSegments", 2, "int"],
  ["kMaxRecursionDepth", 64, "uint32_t"],
  ["kMaxCatchCases", 6, "int"],
];
for (const [name, val, ty] of kMaxConsts) {
  must(new RegExp(`constexpr ${ty} ${name} = ${val};`), name);
}

// —— 2. 包含类别枚举 (bool 底层三组 + 显式值组) ——
must(/enum IncludeNumericTypes \{\s*kIncludeNumericTypes = true,\s*kExcludeNumericTypes = false\s*\};/, "IncludeNumericTypes");
must(/enum IncludePackedTypes \{\s*kIncludePackedTypes = true,\s*kExcludePackedTypes = false\s*\};/, "IncludePackedTypes");
must(/enum IncludeAllGenerics \{\s*kIncludeAllGenerics = true,\s*kExcludeSomeGenerics = false\s*\};/, "IncludeAllGenerics");
must(/enum IncludeS128 \{ kIncludeS128 = true, kExcludeS128 = false \};/, "IncludeS128");

// —— 3. 控制流/调用形态枚举 ——
must(/enum IfType \{ kIf, kIfElse \};/, "IfType");
must(/enum CallKind \{ kCallDirect, kCallIndirect, kCallRef \};/, "CallKind");

// —— 4. 生成器备选数公式 (GeneratorAlternativesPerOption 模板参数) ——
must(/static constexpr size_t kNumSimd = kNumMVP \+ kAdditionalSimd;/, "kNumSimd 公式");
must(/static constexpr size_t kNumWasmGC = kNumMVP \+ kAdditionalWasmGC;/, "kNumWasmGC 公式");
must(/static constexpr size_t kNumAll = kNumSimd \+ kAdditionalWasmGC;/, "kNumAll 公式");
must(/all_\(ConcatArrays\(mvp, ConcatArrays\(simd, wasmgc\)\)\)/, "all 连接序");

// —— 5. 关键表锚 ——
must(/constexpr ValueType kReturns\[\] = \{kWasmI64, kWasmI64\};/, "kReturns 表");
must(/constexpr ValueType kWanted\[\] = \{kWasmI64\};/, "kWanted 表");
must(/static constexpr WasmOpcode kConvertOpcodes\[\]/, "kConvertOpcodes 表");
must(/constexpr static uint8_t max_alignment\(WasmOpcode memop\)/, "max_alignment");

// —— 6. 数据源规则锚 ——
must(/int num_params = int\{data->get<uint8_t>\(\)\} % \(kMaxParameters \+ 1\);/, "参数数采样");
must(/uint8_t num_catch = data->get<uint8_t>\(\) % kMaxCatchCases;/, "catch 数采样");

console.log(`random-module-generation.cc: kMax 常量=${kMaxConsts.length} 枚举=6`);

// —— 7. 生成 Rust ——
let rust = banner(
  "src/wasm/fuzzing/random-module-generation.cc.js",
  "src/wasm/fuzzing/random-module-generation.cc",
  "随机 wasm 模块生成器规则面: 上限常量与备选公式",
);

rust += `
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IncludeNumericTypes {
  /// kIncludeNumericTypes
  K_INCLUDE_NUMERIC_TYPES = 1,
  /// kExcludeNumericTypes
  K_EXCLUDE_NUMERIC_TYPES = 0,
}

/// 打包类型包含开关 (原版 enum IncludePackedTypes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IncludePackedTypes {
  /// kIncludePackedTypes
  K_INCLUDE_PACKED_TYPES = 1,
  /// kExcludePackedTypes
  K_EXCLUDE_PACKED_TYPES = 0,
}

/// 泛型类别包含开关 (原版 enum IncludeAllGenerics)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IncludeAllGenerics {
  /// kIncludeAllGenerics
  K_INCLUDE_ALL_GENERICS = 1,
  /// kExcludeSomeGenerics
  K_EXCLUDE_SOME_GENERICS = 0,
}

/// s128 包含开关 (原版 enum IncludeS128)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IncludeS128 {
  /// kIncludeS128
  K_INCLUDE_S128 = 1,
  /// kExcludeS128
  K_EXCLUDE_S128 = 0,
}

/// if 形态 (原版 enum IfType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IfType {
  /// kIf
  K_IF = 0,
  /// kIfElse
  K_IF_ELSE = 1,
}

/// 调用形态 (原版 enum CallKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CallKind {
  /// kCallDirect
  K_CALL_DIRECT = 0,
  /// kCallIndirect
  K_CALL_INDIRECT = 1,
  /// kCallRef
  K_CALL_REF = 2,
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
  assert!(generator_alternative_counts(4, 2, 3) == (6, 7, 9));
  assert!(IfType::K_IF_ELSE as u8 == 1);
  assert!(CallKind::K_CALL_REF as u8 == 2);
};
`;

const p = writeSrc("v8-wasm-turbofan", "random_module_generation_impl.rs", rust);
console.log(`written ${p}`);
