#!/usr/bin/env bun
// tool/src/wasm/wasm-init-expr.h.js
// 机械提取 v8/src/wasm/wasm-init-expr.h 的 WasmInitExpr::Operator 枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-init-expr.h");

const opMatch = hSrc.match(/enum Operator \{([\s\S]*?)\n  \};/);
if (!opMatch) throw new Error("未找到 WasmInitExpr::Operator");
const ops = opMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
// 机械展开为显式序号, 无别名
const EXPECTED = [
  "kGlobalGet", "kI32Const", "kI64Const", "kF32Const", "kF64Const", "kS128Const",
  "kI32Add", "kI32Sub", "kI32Mul", "kI64Add", "kI64Sub", "kI64Mul",
  "kRefNullConst", "kRefFuncConst", "kStructNew", "kStructNewDefault",
  "kStructNewDesc", "kStructNewDefaultDesc", "kArrayNew", "kArrayNewDefault",
  "kArrayNewFixed", "kRefI31", "kStringConst", "kAnyConvertExtern",
  "kExternConvertAny",
];
if (ops.join(",") !== EXPECTED.join(",")) {
  throw new Error(`Operator 内容不符:\n  提取: ${ops}\n  期望: ${EXPECTED}`);
}

console.log(`wasm-init-expr: Operator=${ops.length}`);

let rust = banner(
  "src/wasm/wasm-init-expr.h.js",
  "src/wasm/wasm-init-expr.h",
  "WasmInitExpr 操作符枚举 (不依赖 wire bytes 的常量表达式表示)",
);

rust += `
/// 初始化表达式操作符 (对应原版 WasmInitExpr::Operator, 共 ${ops.length} 种)
/// 与 ConstantExpression 不同, 该表示不依赖模块字节码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InitExprOperator {
`;
for (const [i, op] of ops.entries()) {
  rust += `  /// ${op}\n  ${op.replace(/^k/, "")} = ${i},\n`;
}
rust += `}

impl InitExprOperator {
  /// 操作符总数
  pub const COUNT: usize = ${ops.length};

  /// 从数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    match val {
`;
for (const [i, op] of ops.entries()) {
  rust += `      ${i} => Some(Self::${op.replace(/^k/, "")}),\n`;
}
rust += `      _ => None,
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(InitExprOperator::COUNT == ${ops.length});
  assert!(InitExprOperator::GlobalGet as u8 == 0);
  assert!(InitExprOperator::ExternConvertAny as u8 == ${ops.length - 1});
};
`;

const p = writeSrc("v8-wasm-core", "wasm_init_expr.rs", rust);
console.log(`written ${p}`);
