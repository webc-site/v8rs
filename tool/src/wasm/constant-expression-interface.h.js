#!/usr/bin/env bun
// tool/src/wasm/constant-expression-interface.h.js
// 机械提取 v8/src/wasm/constant-expression-interface.{h,cc} 的常量表达式指令白名单

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/constant-expression-interface.h");
const ccSrc = readV8("src/wasm/constant-expression-interface.cc");

// 1. h: 双模式类契约
for (const a of [
  "using ValidationTag = Decoder::FullValidationTag;",
  "static constexpr DecodingMode decoding_mode = kConstantExpression;",
  "static constexpr bool kUsesPoppedArgs = true;",
]) {
  if (!hSrc.includes(a)) throw new Error(`h 契约缺失: ${a}`);
}

// 2. h: 三段宏生成 (空/不可达/实现)
for (const m of [
  "INTERFACE_META_FUNCTIONS(EMPTY_INTERFACE_FUNCTION)",
  "INTERFACE_NON_CONSTANT_FUNCTIONS(UNREACHABLE_INTERFACE_FUNCTION)",
  "INTERFACE_CONSTANT_FUNCTIONS(DECLARE_INTERFACE_FUNCTION)",
]) {
  if (!hSrc.includes(m)) throw new Error(`宏展开缺失: ${m}`);
}

// 3. h: 状态字段
for (const f of [
  "bool end_found_ = false;",
  "bool ends_with_struct_new_ = false;",
  "WasmValue computed_value_;",
  "MessageTemplate error_ = MessageTemplate::kNone;",
]) {
  if (!hSrc.includes(f)) throw new Error(`状态字段缺失: ${f}`);
}

// 4. cc: 真解析实现的接口函数全集
const implFns = [...ccSrc.matchAll(
  /void ConstantExpressionInterface::(\w+)\(/g,
)].map((m) => m[1]);
if (implFns.length !== 21) throw new Error(`实现函数 ${implFns.length} != 21`);

// 5. cc: BinOp 允许的 6 个算术
const binMatch = ccSrc.match(/void ConstantExpressionInterface::BinOp[\s\S]*?switch \(opcode\) \{([\s\S]*?)\n  \}/);
if (!binMatch) throw new Error("未找到 BinOp switch");
const binOps = [...binMatch[1].matchAll(/case kExpr(\w+):/g)].map((m) => m[1]);
if (binOps.join(",") !== "I32Add,I32Sub,I32Mul,I64Add,I64Sub,I64Mul") {
  throw new Error(`BinOp 白名单异常: ${binOps.join(",")}`);
}

// 6. cc: UnOp 允许的 2 个转换
const unMatch = ccSrc.match(/void ConstantExpressionInterface::UnOp[\s\S]*?switch \(opcode\) \{([\s\S]*?)\n  \}/);
if (!unMatch) throw new Error("未找到 UnOp switch");
const unOps = [...unMatch[1].matchAll(/case kExpr(\w+):/g)].map((m) => m[1]);
if (unOps.join(",") !== "ExternConvertAny,AnyConvertExtern") {
  throw new Error(`UnOp 白名单异常: ${unOps.join(",")}`);
}

// 7. cc: 错误路径模板
if ((ccSrc.match(/error_ = MessageTemplate::kWasmTrap/g) || []).length < 5) {
  throw new Error("kWasmTrap 错误路径过少");
}

// 8. cc: struct.new 结尾标记 (lookahead 下一指令是否 end)
const endsWithCount = (ccSrc.match(/ends_with_struct_new_ = decoder->lookahead\(/g) || []).length;
if (endsWithCount !== 2) throw new Error(`ends_with_struct_new_ 置位 ${endsWithCount} 次 != 2`);

// 9. cc: 计算值守卫
if (!ccSrc.includes("if (!generate_value()) return;")) throw new Error("未找到 generate_value 守卫");

console.log(
  `constant-expression-interface: 实现函数=${implFns.length} BinOp=${binOps.length} UnOp=${unOps.length}`,
);

// 10. 生成 Rust
let rust = banner(
  "src/wasm/constant-expression-interface.h.js",
  "src/wasm/constant-expression-interface.{h,cc}",
  "常量表达式解码接口的指令白名单与双模式契约",
);

rust += `
//! WasmFullDecoder 接口: 仅校验 (isolate==null) 或产码 (isolate!=null) 两模式合一。
/// v8: src/wasm/constant-expression-interface.h ConstantExpressionInterface

/// 解码模式与校验档
/// v8: src/wasm/constant-expression-interface.h ValidationTag / decoding_mode
pub const DECODING_MODE_CONSTANT_EXPRESSION: bool = true;
pub const USES_POPPED_ARGS: bool = true;
pub const FULL_VALIDATION: bool = true;

/// cc 侧实际实现的接口函数 (INTERFACE_CONSTANT_FUNCTIONS 展开的子集)
/// v8: src/wasm/constant-expression-interface.cc ConstantExpressionInterface::*
pub const IMPLEMENTED_INTERFACE_FUNCTIONS: &[&str] = &[
${implFns.map((f) => `    "${f}",`).join("\n")}
];

/// BinOp 白名单: i32/i64 的 add/sub/mul 共 6 个
/// v8: src/wasm/constant-expression-interface.cc BinOp
pub fn constant_binop_allowed(op: Opcode) -> bool {
    matches!(
        op,
        Opcode::i32_add
            | Opcode::i32_sub
            | Opcode::i32_mul
            | Opcode::i64_add
            | Opcode::i64_sub
            | Opcode::i64_mul
    )
}

/// UnOp 白名单: any↔extern 互转共 2 个
/// v8: src/wasm/constant-expression-interface.cc UnOp
pub fn constant_unop_allowed(op: Opcode) -> bool {
    matches!(op, Opcode::extern_convert_any | Opcode::any_convert_extern)
}

/// 双模式判定: 产码要求 isolate 存在且无错
/// v8: src/wasm/constant-expression-interface.h generate_value()
pub fn generate_value(isolate_present: bool, has_error: bool) -> bool {
    isolate_present && !has_error
}

/// struct.new 结尾标记 (供外部识别结构体常量)
/// v8: src/wasm/constant-expression-interface.h ends_with_struct_new_
pub const TRACKS_ENDS_WITH_STRUCT_NEW: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(IMPLEMENTED_INTERFACE_FUNCTIONS.len(), 21);
    assert!(USES_POPPED_ARGS);
    assert!(!generate_value(false, false));
    assert!(!generate_value(true, true));
    assert!(generate_value(true, false));
};
`;

const p = writeSrc("v8-wasm-core", "constant_expression_interface.rs.skel", rust);
console.log(`written ${p}`);
