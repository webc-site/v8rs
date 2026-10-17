#!/usr/bin/env bun
// tool/src/wasm/constant-expression-interface.cc.js
// 机械提取 v8/src/wasm/constant-expression-interface.cc 的求值操作表
// (UnOp 2 / BinOp 6)、i31ref Smi 编码、end 提前停机语义。
// 骨架在 v8-wasm-core/src/constant_expression_interface.rs.skel (constant-expression-interface.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/constant-expression-interface.cc");

// ---------- 1. UnOp 表 (2 项) ----------
const unOps = src.match(/void ConstantExpressionInterface::UnOp[\s\S]*?switch \(opcode\) \{([\s\S]*?)\n  \}/)[1];
const unCases = [...unOps.matchAll(/case (kExpr\w+):/g)].map((m) => m[1]);
if (unCases.join(",") !== "kExprExternConvertAny,kExprAnyConvertExtern") {
  throw new Error(`UnOp 序异常: ${unCases.join(",")}`);
}

// ---------- 2. BinOp 表 (6 项回绕算术) ----------
const binBlock = src.match(/void ConstantExpressionInterface::BinOp[\s\S]*?switch \(opcode\) \{([\s\S]*?)\n  \}/)[1];
const binCases = [...binBlock.matchAll(/case (kExpr\w+):/g)].map((m) => m[1]);
if (binCases.join(",") !== "kExprI32Add,kExprI32Sub,kExprI32Mul,kExprI64Add,kExprI64Sub,kExprI64Mul") {
  throw new Error(`BinOp 序异常: ${binCases.join(",")}`);
}
// 全部回绕
const wrapOps = [...binBlock.matchAll(/base::(Add|Sub|Mul)WithWraparound/g)].map((m) => m[1]);
if (wrapOps.length !== 6) throw new Error(`回绕算术 ${wrapOps.length} != 6`);

// ---------- 3. i31ref Smi 编码 (31/32 位 Smi 双轨) ----------
if (!src.includes("static_assert((SmiValuesAre31Bits() ^ SmiValuesAre32Bits()) == 1);")) {
  throw new Error("Smi 双轨断言缺失");
}
if (!src.includes("shifted = raw << (kSmiTagSize + kSmiShiftSize);")) {
  throw new Error("31 位 Smi 移位缺失");
}
if (!src.includes("static_cast<intptr_t>(raw << (kSmiTagSize + kSmiShiftSize + 1)) >> 1;")) {
  throw new Error("32 位 Smi 符号扩展缺失");
}

// ---------- 4. end 提前停机: set_end(pc+1) ----------
if (!src.includes("decoder->set_end(decoder->pc() + 1);")) throw new Error("end 停机缺失");

// ---------- 5. RefFunc 无 isolate 时仅登记 declared ----------
if (!src.includes("outer_module_->functions[function_index].declared = true;")) {
  throw new Error("declared 登记缺失");
}

// ---------- 6. null 差异化 (use_wasm_null) ----------
if (!src.includes("type.use_wasm_null() ? Cast<Object>(isolate_->factory()->wasm_null())")) {
  throw new Error("null 差异化缺失");
}

// ---------- 7. struct.new 结尾探测 ----------
if (!src.includes("ends_with_struct_new_ = decoder->lookahead(offset_to_next_instr, kExprEnd);")) {
  throw new Error("struct.new 结尾探测缺失");
}

console.log(`constant-expression-interface: UnOp=${unCases.length} BinOp=${binCases.length}`);

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/constant-expression-interface.cc.js",
  "src/wasm/constant-expression-interface.cc",
  "常量表达式求值操作表 (骨架在 constant_expression_interface.rs.skel)",
)}
//! 常量表达式接口: 解码回调求值。

/// 单目操作表 (2 项): extern.convert_any / any.convert_extern (JS<->wasm 对象包装)
/// v8: src/wasm/constant-expression-interface.cc UnOp
#[rustfmt::skip]
pub static UN_OPS: &[&str] = &[
    "kExprExternConvertAny",
    "kExprAnyConvertExtern",
];

/// 双目操作表 (6 项): 全部回绕算术 (Add/Sub/Mul x I32/I64)
/// v8: src/wasm/constant-expression-interface.cc BinOp
#[rustfmt::skip]
pub static BIN_OPS: &[&str] = &[
    "kExprI32Add",
    "kExprI32Sub",
    "kExprI32Mul",
    "kExprI64Add",
    "kExprI64Sub",
    "kExprI64Mul",
];

/// i31ref 的 Smi 编码: 接受越界输入, 手工构造 Smi
/// 31 位 Smi: raw << (kSmiTagSize + kSmiShiftSize);
/// 32 位 Smi: (raw << (kSmiTagSize + kSmiShiftSize + 1)) >> 1 (算术右移符号扩展次高位)
/// v8: src/wasm/constant-expression-interface.cc RefI31
pub const I31_SMI_SHIFT_31BIT: &str = "raw << (kSmiTagSize + kSmiShiftSize)";
pub const I31_SMI_SHIFT_32BIT: &str = "(raw << (kSmiTagSize + kSmiShiftSize + 1)) as intptr >> 1";

/// end 提前停机: 常量表达式长度未知, 遇 end 将解码终点设为 pc+1
/// v8: src/wasm/constant-expression-interface.cc DoReturn
pub const END_EARLY_STOP: &str = "set_end(pc + 1)";

/// RefFunc 在无 isolate (仅校验) 时只登记函数为 declared
/// v8: src/wasm/constant-expression-interface.cc RefFunc
pub const REFFUNC_DECLARE_ONLY: &str = "functions[i].declared = true";

/// ref.null 按类型选 wasm_null / 普通 null
/// v8: src/wasm/constant-expression-interface.cc RefNull
pub const NULL_VARIANT_RULE: &str = "use_wasm_null ? wasm_null : null_value";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(UN_OPS.len(), 2);
    assert_eq!(BIN_OPS.len(), 6);
};
`;

const p = writeSrc("v8-wasm-core", "constant_expression_interface_cc.rs", rust);
console.log(`written ${p}`);
