#!/usr/bin/env bun
// tool/src/wasm/basic-block-calculator.h.js
// 机械提取 v8/src/wasm/basic-block-calculator.{h,cc} 的基本块终结/分裂 opcode 表

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/basic-block-calculator.h");
const ccSrc = readV8("src/wasm/basic-block-calculator.cc");

// 1. h: 类签名锚
if (!hSrc.includes("class BasicBlockCalculator")) throw new Error("未找到 BasicBlockCalculator");
if (!hSrc.includes("int current_basic_block_ = -1;")) {
  throw new Error("未找到 current_basic_block_ 初始值 -1");
}
if (!hSrc.includes("uint32_t current_basic_block_start_pos_ = 0;")) {
  throw new Error("未找到 current_basic_block_start_pos_");
}

// 2. cc: 真解析 switch 中的块终结 opcode (EndCurrentBlock 分支)
if (!ccSrc.includes("case kExprLoop:")) throw new Error("未找到 loop case");
const switchBlock = ccSrc.match(
  /case kExprLoop:([\s\S]*?)break;\s*\n\s*(case kExprEnd:[\s\S]*?)EndCurrentBlock\(&iterator\);/,
);
if (!switchBlock) throw new Error("未找到 ComputeBasicBlocks 的 switch");
const loopCase = switchBlock[1];
const endersCase = switchBlock[2];
if (!loopCase.includes("basic_block_ranges_.emplace_back(current_basic_block_start_pos_,\n                                           iterator.position() - 1);")) {
  throw new Error("loop 分支未封闭当前区间");
}

const enderOpcodes = [...endersCase.matchAll(/case kExpr(\w+)/g)].map((m) => m[1]);
if (endersCase.includes("kExprLoop")) throw new Error("终结分支混入 loop");
if (enderOpcodes.length !== 26) {
  throw new Error(`块终结 opcode 数 ${enderOpcodes.length} != 26`);
}
if (!ccSrc.includes("at_block_end = true;")) throw new Error("未找到 at_block_end");
if (!ccSrc.includes("if (!at_block_end) {")) throw new Error("未找到收尾分支条件");

// 3. cc: 规则注释锚 (5 条基本块规则)
for (const rule of [
  "1. The first basic block starts at function offset 0.",
  "2. The 'else' instructions",
  "3. The 'loop' instructions end the current block and start a new block.",
  "4. The first instruction after the end of a block starts a new block.",
  "5. The last basic block ends at the offset of the final function 'end'",
]) {
  if (!ccSrc.includes(rule)) throw new Error(`规则注释缺失: ${rule}`);
}

// 4. cc: 收尾区间公式 (position-1)
if (!ccSrc.includes("iterator.position() - 1")) throw new Error("未找到收尾公式 position()-1");
if (!ccSrc.includes("OpcodeLength(decoder->pc(), decoder->end())")) {
  throw new Error("未找到 OpcodeLength 终结指令长度");
}

console.log(`basic-block-calculator: loop特殊=1 终结=${enderOpcodes.length}`);

// 5. 生成 Rust
const snake = (s) => s.toLowerCase();
let rust = banner(
  "src/wasm/basic-block-calculator.h.js",
  "src/wasm/basic-block-calculator.{h,cc}",
  "BasicBlockCalculator 块终结/分裂 opcode 判定表",
);

rust += `
//! 基本块切分规则 (原版 ComputeBasicBlocks 的 switch 展开)。
/// v8: src/wasm/basic-block-calculator.cc ComputeBasicBlocks

/// 未开始时的 current_basic_block_ 哨兵
pub const NO_CURRENT_BLOCK: i32 = -1;

/// 是否为"终结当前块且下一指令开新块"的 opcode
/// (原版 switch default 之外的 case 集合, 共 ${enderOpcodes.length} 个)
pub fn opcode_ends_basic_block(op: Opcode) -> bool {
    matches!(
        op,
        Opcode::${enderOpcodes.map(snake).join("\n            | Opcode::")}
    )
}

/// loop 特殊: 先封当前区间 (position-1) 再原地开新块
/// v8: src/wasm/basic-block-calculator.cc case kExprLoop
pub fn opcode_splits_basic_block(op: Opcode) -> bool {
    matches!(op, Opcode::loop)
}

/// 块区间右端点公式: 结束指令最后字节 = position + OpcodeLength - 1
/// v8: src/wasm/basic-block-calculator.cc EndCurrentBlock
pub fn block_end(first_byte_of_next: u32, final_instr_len: u32) -> u32 {
    first_byte_of_next + final_instr_len - 1
}

// —— 提取不变式断言 ——
const _: () = {
    assert!(NO_CURRENT_BLOCK == -1);
};
`;

const p = writeSrc("v8-wasm-core", "basic_block_calculator.rs.skel", rust);
console.log(`written ${p}`);
