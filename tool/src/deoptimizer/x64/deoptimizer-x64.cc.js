#!/usr/bin/env bun
// tool/src/deoptimizer/x64/deoptimizer-x64.cc.js
// 机械提取 v8/src/deoptimizer/x64/deoptimizer-x64.cc —
// 反优化出口尺寸常量 (含 CET 双分支) + PatchToJump 布局常量。
// 语义面: ZapCode/PatchToJump/寄存器读写实现登记跳过。
// 产物: v8-deoptimizer/src/x64/deoptimizer_x64.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const CC_REL = "src/deoptimizer/x64/deoptimizer-x64.cc";
const cc = readV8(CC_REL);

const eagerM = cc.match(/const int Deoptimizer::kEagerDeoptExitSize = (\d+);/);
if (!eagerM || eagerM[1] !== "4") throw new Error("kEagerDeoptExitSize 锚不符");
const lazyM = cc.match(/#ifdef V8_ENABLE_CET_IBT[\s\S]*?const int Deoptimizer::kLazyDeoptExitSize = (\d+);\n#else\nconst int Deoptimizer::kLazyDeoptExitSize = (\d+);/);
if (!lazyM) throw new Error("kLazyDeoptExitSize 双分支锚不符");
const ssm = cc.match(/#if V8_ENABLE_CET_SHADOW_STACK\nconst int Deoptimizer::kAdaptShadowStackOffsetToSubtract = (\d+);\n#else\nconst int Deoptimizer::kAdaptShadowStackOffsetToSubtract = (\d+);/);
if (!ssm) throw new Error("kAdaptShadowStackOffsetToSubtract 双分支锚不符");
const sizeM = cc.match(/constexpr int kSize = (\d+);/);
const extraM = cc.match(/constexpr int kExtra = (\d+);/);
if (!sizeM || !extraM) throw new Error("缺 PatchToJump 布局常量锚");
if (!cc.includes("ASSERT_OFFSET(Builtin::kDeoptimizationEntry_Eager);")) throw new Error("缺 ASSERT_OFFSET 锚");

let rust = banner("src/deoptimizer/x64/deoptimizer-x64.cc.js", CC_REL, "x64 反优化平台常量");
rust += `
// x64 目标展开; 语义面 (ZapCode/PatchToJump/寄存器读写) 登记跳过。

/// 急切反优化出口尺寸 (原 kEagerDeoptExitSize)
pub const EAGER_DEOPT_EXIT_SIZE: i32 = ${eagerM[1]};

/// 惰性反优化出口尺寸双分支:
/// - CET IBT 开启 (额外 endbr64): ${lazyM[1]}
/// - 关闭: ${lazyM[2]}
pub const LAZY_DEOPT_EXIT_SIZE_CET_IBT: i32 = ${lazyM[1]};
pub const LAZY_DEOPT_EXIT_SIZE: i32 = ${lazyM[2]};

/// 影子栈自适应偏移双分支 (CET shadow stack):
/// - 开启: ${ssm[1]}
/// - 关闭: ${ssm[2]}
pub const ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT_CET: i32 = ${ssm[1]};
pub const ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT: i32 = ${ssm[2]};

/// PatchToJump 覆写缓冲 (原 kSize)
pub const PATCH_TO_JUMP_BUFFER_SIZE: i32 = ${sizeM[1]};
/// ZapCode 缓冲余量 (原 kExtra)
pub const ZAP_CODE_EXTRA: i32 = ${extraM[1]};

// 原版 ASSERT_OFFSET 断言锚: tier0 入口表 + DeoptimizationEntry_Eager/Lazy
// × kSystemPointerSize <= 0x7F (IsolateData 布局保证)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(EAGER_DEOPT_EXIT_SIZE == 4);
  assert!(LAZY_DEOPT_EXIT_SIZE == 4);
};
`;

const p = writeSrc("v8-deoptimizer", "x64/deoptimizer_x64.rs", rust);
console.log(`written ${p}`);
