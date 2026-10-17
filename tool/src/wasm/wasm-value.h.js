#!/usr/bin/env bun
// tool/src/wasm/wasm-value.h.js
// 机械提取 v8/src/wasm/wasm-value.h 的 FOREACH_PRIMITIVE_WASMVAL_TYPE 与 WasmValue 骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-value.h");

// 1. FOREACH_PRIMITIVE_WASMVAL_TYPE: V(name, localtype, ctype)
const macroMatch = hSrc.match(
  /#define\s+FOREACH_PRIMITIVE_WASMVAL_TYPE\(V\)\s*\\\s*\n([\s\S]*?)\n\n/,
);
if (!macroMatch) throw new Error("未找到 FOREACH_PRIMITIVE_WASMVAL_TYPE");

const entries = [...macroMatch[1].matchAll(/V\(\s*(\w+)\s*,\s*(kWasm\w+)\s*,\s*([\w:]+)\s*\)/g)].map(
  (m) => ({ name: m[1], localtype: m[2], ctype: m[3] }),
);
if (entries.length !== 12) throw new Error(`提取 ${entries.length} 条, 期望 12`);
{
  const names = entries.map((e) => e.name);
  if (names[0] !== "i8" || names[names.length - 1] !== "s128") {
    throw new Error("条目锚点漂移: 首条应为 i8, 末条应为 s128");
  }
  if (!names.includes("f32_boxed") || !names.includes("f64_boxed")) {
    throw new Error("boxed 条目缺失");
  }
}
console.log(`FOREACH_PRIMITIVE_WASMVAL_TYPE ${entries.length} 条`);

// localtype 记号 → 既有 value_type.rs 的 ValueKind 变体
const kindVariant = (t) => {
  const m = t.match(/^kWasm(\w+)$/);
  if (!m) throw new Error(`未知类型记号 ${t}`);
  return m[1]; // I8 / I16 / I32 / I64 / F16 / F32 / F64 / S128
};

let rust = banner(
  "src/wasm/wasm-value.h.js",
  "src/wasm/wasm-value.h",
  "WasmValue: 带类型信息的 wasm 值 (FOREACH_PRIMITIVE_WASMVAL_TYPE 机械展开)",
);

rust += `
use crate::value_type::ValueKind;

/// 原始类型条目 (对应 FOREACH_PRIMITIVE_WASMVAL_TYPE, ${entries.length} 条)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimitiveWasmvalType {
  /// 方法名片段, 如 "i32" / "f32_boxed" (to_<name> 系列)
  pub name: &'static str,
  /// 对应 wasm 类型记号 (kWasmI32 等)
  pub localtype: ValueKind,
  /// C++ 载体类型 (int32_t / float / Simd128 等, 仅存档)
  pub ctype: &'static str,
}

/// FOREACH_PRIMITIVE_WASMVAL_TYPE 展开表 (源顺序)
pub static PRIMITIVE_WASMVAL_TYPES: &[PrimitiveWasmvalType] = &[
`;

for (const e of entries) {
  rust += `  PrimitiveWasmvalType { name: "${e.name}", localtype: ValueKind::${kindVariant(e.localtype)}, ctype: "${e.ctype}" },\n`;
}

rust += `];

/// 条目总数
pub const PRIMITIVE_WASMVAL_TYPE_COUNT: usize = ${entries.length};

/// WasmValue: 16 字节位模式 + 类型标签 (对应 class WasmValue, 字段布局直译)
#[derive(Clone, Copy)]
pub struct WasmValue {
  /// 类型 (CanonicalValueType; 骨架期以 ValueKind 代替, 语义修订时替换)
  pub type_: ValueKind,
  /// 位模式 (uint8_t bit_pattern_[16])
  pub bit_pattern_: [u8; 16],
}

impl WasmValue {
  /// 默认: kWasmVoid + 全零位模式
  #[must_use]
  pub const fn new() -> Self {
    Self { type_: ValueKind::Void, bit_pattern_: [0; 16] }
  }
`;

for (const e of entries) {
  const t = e.ctype;
  const rustTy =
    t === "int8_t" ? "i8" :
    t === "int16_t" ? "i16" :
    t === "int32_t" ? "i32" :
    t === "uint32_t" ? "u32" :
    t === "int64_t" ? "i64" :
    t === "uint64_t" ? "u64" :
    t === "uint16_t" ? "u16" :
    t === "float" ? "f32" :
    t === "double" ? "f64" :
    t === "Float32" || t === "Float64" || t === "Simd128" ? "[u8; 16]" : "!";
  rust += `
  /// 对应 ${e.ctype} 构造/取值 (原版 DEFINE_TYPE_SPECIFIC_METHODS 展开)
  pub fn from_${e.name}(v: ${rustTy}) -> Self { unimplemented!("骨架") }
  /// 对应 to_${e.name}()
  pub fn to_${e.name}(&self) -> ${rustTy} { self.to_${e.name}_unchecked() }
  /// 对应 to_${e.name}_unchecked()
  pub fn to_${e.name}_unchecked(&self) -> ${rustTy} { unimplemented!("骨架") }
`;
}

rust += `}

impl Default for WasmValue {
  fn default() -> Self {
    Self::new()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(PRIMITIVE_WASMVAL_TYPES.len() == 12);
  assert!(PRIMITIVE_WASMVAL_TYPES[0].name == "i8");
  assert!(PRIMITIVE_WASMVAL_TYPES[11].name == "s128");
  assert!(size_of::<WasmValue>() >= 16);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_value.rs", rust);
console.log(`written ${p}`);
