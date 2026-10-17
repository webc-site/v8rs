#!/usr/bin/env bun
// tool/src/wasm/basic-block-calculator.cc.js
// 机械提取 v8/src/wasm/basic-block-calculator.cc 的基本块划分规则:
// 块终止操作码清单 (26 个 case)、loop 特殊规则、区间端点约定。
// 骨架在 v8-wasm-core/src/basic_block_calculator.rs.skel (basic-block-calculator.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/basic-block-calculator.cc");

// ---------- 1. 块终止操作码 (switch case 清单) ----------
const block = src.match(/void BasicBlockCalculator::ComputeBasicBlocks\(\) \{([\s\S]*?)\n\}/);
if (!block) throw new Error("ComputeBasicBlocks 缺失");
const body = block[1];
// kExprLoop 单独一组
const endCases = [...body.matchAll(/case (kExpr\w+):/g)].map((m) => m[1]);
const loopIdx = endCases.indexOf("kExprLoop");
if (loopIdx < 0) throw new Error("kExprLoop 分支缺失");
// Loop 分支之后的 default 组是块终结符
const terminators = endCases.slice(loopIdx + 1);
if (terminators.length !== 26) throw new Error(`块终结操作码 ${terminators.length} != 26`);
const expectHead = ["kExprEnd", "kExprIf", "kExprElse", "kExprUnreachable", "kExprBr"];
for (let i = 0; i < expectHead.length; i++) {
  if (terminators[i] !== expectHead[i]) throw new Error(`块终结序异常: ${terminators.slice(0, 5).join(",")}`);
}
if (terminators.indexOf("kExprCallRef") !== terminators.length - 1) {
  throw new Error("块终结尾项非 kExprCallRef");
}

// ---------- 2. loop 规则: 前段非空则先结束当前块再开新块 ----------
if (!body.includes("case kExprLoop:")) throw new Error("loop 分支缺失");
if (!body.includes("if (iterator.position() > current_basic_block_start_pos_) {")) {
  throw new Error("loop 前段判定缺失");
}

// ---------- 3. 区间端点: 结束块 = position + 指令长 - 1 (EndCurrentBlock); 收尾块 = position - 1 ----------
if (!src.includes("decoder->position() + final_instruction_length - 1);")) {
  throw new Error("结束块端点公式缺失");
}
if (!body.includes("basic_block_ranges_.emplace_back(current_basic_block_start_pos_,\n                                     iterator.position() - 1);")) {
  throw new Error("收尾块端点公式缺失");
}

// ---------- 4. 规则注释五条 (规则面语义锚) ----------
for (const anchor of [
  "// 1. The first basic block starts at function offset 0.",
  "// 4. The first instruction after the end of a block starts a new block.",
  "// 5. The last basic block ends at the offset of the final function 'end'",
]) {
  if (!body.includes(anchor)) throw new Error(`规则注释缺失: ${anchor}`);
}

// ---------- 5. 局部声明前置 ----------
if (!body.includes("BodyLocalDecls locals;")) throw new Error("局部声明前置缺失");
if (!body.includes("DCHECK_LT(0, locals.encoded_size);")) throw new Error("编码尺寸断言缺失");

console.log(`basic-block-calculator: 块终结=${terminators.length} loop分支=1`);

// ---------- 生成 Rust ----------
let termList = "";
for (let i = 0; i < terminators.length; i += 4) {
  termList += `    ${terminators.slice(i, i + 4).map((t) => `"${t}"`).join(", ")},\n`;
}

const rust = `${banner(
  "src/wasm/basic-block-calculator.cc.js",
  "src/wasm/basic-block-calculator.cc",
  "基本块划分规则 (骨架在 basic_block_calculator.rs.skel)",
)}
//! 函数体基本块划分。

/// 块终结操作码 (执行到此即结束当前基本块; 26 个)
/// v8: src/wasm/basic-block-calculator.cc BasicBlockCalculator::ComputeBasicBlocks
#[rustfmt::skip]
pub static BLOCK_TERMINATORS: &[&str] = &[
${termList}];

/// loop 规则: loop 结束当前块并开新块; 若 loop 前为空段则仅推进起点
/// v8: src/wasm/basic-block-calculator.cc case kExprLoop
pub const LOOP_RULE: &str = "loop 终结当前块并开新块 (空段除外)";

/// 区间端点约定 (含端点):
/// 结束块终点 = 分支指令 position + 该指令长 - 1;
/// 收尾块终点 = 最终 'end' 的 position - 1
/// v8: src/wasm/basic-block-calculator.cc EndCurrentBlock / ComputeBasicBlocks
pub const BLOCK_END_FORMULA: &str = "position + OpcodeLength(pc) - 1";
pub const FINAL_BLOCK_END_FORMULA: &str = "position - 1";

/// 划分规则五条 (源注释语义)
/// v8: src/wasm/basic-block-calculator.cc ComputeBasicBlocks
#[rustfmt::skip]
pub static BASIC_BLOCK_RULES: &[&str] = &[
    "首个基本块从函数偏移 0 开始",
    "else/end 及一切分支/调用/尾调用/unreachable/return/if/throw 终结当前块",
    "loop 终结当前块并开新块",
    "块终结后首条指令开新块 (其间不可达指令不属于任何块)",
    "最后一块在函数末尾 'end' 偏移处结束",
];

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(BLOCK_TERMINATORS.len(), 26);
    assert_eq!(BLOCK_TERMINATORS[0], "kExprEnd");
    assert_eq!(BLOCK_TERMINATORS[25], "kExprCallRef");
    assert!(BLOCK_TERMINATORS.contains(&"kExprBrTable"));
    assert!(BLOCK_TERMINATORS.contains(&"kExprThrowRef"));
    assert_eq!(BASIC_BLOCK_RULES.len(), 5);
};
`;

const p = writeSrc("v8-wasm-core", "basic_block_calculator_cc.rs", rust);
console.log(`written ${p}`);
