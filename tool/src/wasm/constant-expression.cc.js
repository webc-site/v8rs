#!/usr/bin/env bun
// tool/src/wasm/constant-expression.cc.js
// 机械提取 v8/src/wasm/constant-expression.cc 的差集:
// EvaluateConstantExpression 的 Kind 五路分派清单 + ConstantExpression::wire_bytes_ref。
// 头侧 Kind 枚举在 v8-wasm-core/src/constant_expression.rs (constant-expression.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/constant-expression.cc");

// ---------- 锚断言 ----------
const anchors = [
  ["include 自头", src.includes('#include "src/wasm/constant-expression.h"')],
  ["wire_bytes_ref", src.includes("WireBytesRef ConstantExpression::wire_bytes_ref() const {")],
  ["Evaluate 定义", src.includes("ValueOrError EvaluateConstantExpression(")],
  ["解码器", src.includes("WasmFullDecoder<Decoder::FullValidationTag, ConstantExpressionInterface,")],
  ["zone 复位", src.includes("zone->Reset();")],
];
for (const [n, ok] of anchors) {
  if (!ok) throw new Error(`锚点缺失: ${n}`);
}

// ---------- 分派: EvaluateConstantExpression 的 case 清单 ----------
const evalBody = src.slice(
  src.indexOf("ValueOrError EvaluateConstantExpression("),
);
const cases = [...evalBody.matchAll(/case ConstantExpression::Kind::k(\w+):/g)].map((m) => m[1]);
if (cases.length !== 5) throw new Error(`Kind 分派 ${cases.length} 路, 期望 5`);
if (cases.join(",") !== "Empty,I32Const,RefNull,RefFunc,WireBytesRef") {
  throw new Error(`Kind 分派序漂移: ${cases}`);
}
// 各 case 关键语义锚
if (!evalBody.includes("return WasmValue(expr.i32_value());")) throw new Error("kI32Const 语义锚缺失");
if (!evalBody.includes("expected.use_wasm_null()")) throw new Error("kRefNull 的 wasm null 分派缺失");
if (!evalBody.includes("WasmTrustedInstanceData::GetOrCreateFuncRef")) throw new Error("kRefFunc 取 FuncRef 缺失");
if (!evalBody.includes("kConstantExpression")) throw new Error("kWireBytesRef 解码用途缺失");

// ---------- wire_bytes_ref 位域 ----------
if (!src.includes("WireBytesRef(OffsetField::decode(bit_field_),")) throw new Error("OffsetField 解码缺失");
if (!src.includes("LengthField::decode(bit_field_))")) throw new Error("LengthField 解码缺失");

console.log(`constant-expression.cc: Kind分派=${cases.length} 方法=2`);

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/constant-expression.cc.js",
  "src/wasm/constant-expression.cc",
  "EvaluateConstantExpression 的 Kind 分派台账 + wire_bytes_ref 位域解码",
)}
//! 常量表达式求值分派与 wire bytes 引用解码。

/// EvaluateConstantExpression 分派 (${cases.length} 路)
/// v8: src/wasm/constant-expression.cc EvaluateConstantExpression
#[rustfmt::skip]
pub static EVALUATE_DISPATCH: &[(&str, &str)] = &[
  ("Empty", "UNREACHABLE (不可求值)"),
  ("I32Const", "WasmValue(expr.i32_value())"),
  ("RefNull", "expected.use_wasm_null() ? wasm_null : null_value; 类型取 RefNull(expr.type()) 的 canonical 类型"),
  ("RefFunc", "GetOrCreateFuncRef(isolate, instance, expr.index()); 类型取 expected 的 canonical 类型"),
  ("WireBytesRef", "构造 FunctionBody (FixedSizeSignature::Returns(expected)), WasmFullDecoder 以 kConstantExpression 用途解码; 完成后 zone->Reset()"),
];

/// kWireBytesRef 求值后复位 Zone (解码器析构先于 Zone 复位)
/// v8: src/wasm/constant-expression.cc EvaluateConstantExpression
pub const WIRE_BYTES_REF_RESETS_ZONE: bool = true;

/// wire_bytes_ref: bit_field_ 经 OffsetField/LengthField 解码为 (offset, length)
/// v8: src/wasm/constant-expression.cc ConstantExpression::wire_bytes_ref
pub fn wire_bytes_ref_decode<D>(bit_field: u32, offset_field: D, length_field: D) -> (u32, u32)
where
    D: Fn(u32) -> u32,
{
    let _ = (bit_field, &offset_field, &length_field);
    unimplemented!("骨架: 参照原版 OffsetField::decode/LengthField::decode 修订")
}

/// 方法台账 (2 条, .cc 侧)
/// v8: src/wasm/constant-expression.cc
#[rustfmt::skip]
pub static METHODS: &[(&str, u32)] = &[
  ("ConstantExpression::wire_bytes_ref", 23),
  ("EvaluateConstantExpression", 29),
];

// —— 提取不变式断言 ——
const _: () = {
    assert!(EVALUATE_DISPATCH.len() == 5);
    assert!(METHODS.len() == 2);
    assert!(WIRE_BYTES_REF_RESETS_ZONE);
};
`;

const p = writeSrc("v8-wasm-core", "constant_expression_cc.rs.skel", rust);
console.log(`written ${p}`);
