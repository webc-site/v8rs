#!/usr/bin/env bun
// tool/src/deoptimizer/arm64/deoptimizer-arm64.cc.js
// 机械提取 v8/src/deoptimizer/arm64/deoptimizer-arm64.cc —
// arm64 反优化平台常量 (影子栈分支恒 0 / PatchToJump 缓冲)。
// 产物: v8-deoptimizer/src/arm64/deoptimizer_arm64.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const CC_REL = "src/deoptimizer/arm64/deoptimizer-arm64.cc";
const cc = readV8(CC_REL);

const ssm = cc.match(/const int Deoptimizer::kAdaptShadowStackOffsetToSubtract = (\d+);/);
if (!ssm || ssm[1] !== "0") throw new Error("kAdaptShadowStackOffsetToSubtract 锚不符 (arm64 无影子栈)");
const sizeM = cc.match(/constexpr int kSize = (\d+);/);
if (!sizeM) throw new Error("缺 PatchToJump 缓冲常量锚");
if (!cc.includes("void Deoptimizer::ZapCode(") && !cc.includes("TODO(422364570)")) {
  throw new Error("缺 ZapCode 登记锚");
}
if (!cc.includes("masm.b(")) throw new Error("缺 b 指令发射锚");

let rust = banner("src/deoptimizer/arm64/deoptimizer-arm64.cc.js", CC_REL, "arm64 反优化平台常量");
rust += `
// arm64 目标展开; 语义面 (ZapCode/PatchToJump/寄存器读写) 登记跳过。

/// 影子栈自适应偏移 (arm64 无 CET 影子栈, 恒 0)
pub const ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT: i32 = 0;

/// PatchToJump 覆写缓冲 (原 kSize; 4 字节 b 指令 + 余量)
pub const PATCH_TO_JUMP_BUFFER_SIZE: i32 = ${sizeM[1]};

// 原版登记: ZapCode 平台 TODO(422364570) 未支持, 为空实现。
// PatchToJump: offset = (new_pc - pc) / kInstrSize, 发射 b 指令。

// —— 提取不变式断言 ——
const _: () = {
  assert!(ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT == 0);
};
`;

const p = writeSrc("v8-deoptimizer", "arm64/deoptimizer_arm64.rs", rust);
console.log(`written ${p}`);
