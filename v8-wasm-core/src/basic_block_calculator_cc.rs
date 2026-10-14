// 由 tool/src/wasm/basic-block-calculator.cc.js 从 v8/src/wasm/basic-block-calculator.cc 生成; 修订就地改, 复跑覆盖
// 基本块划分规则 (骨架在 basic_block_calculator.rs.skel)

//! 函数体基本块划分。

/// 块终结操作码 (执行到此即结束当前基本块; 26 个)
/// v8: src/wasm/basic-block-calculator.cc BasicBlockCalculator::ComputeBasicBlocks
#[rustfmt::skip]
pub static BLOCK_TERMINATORS: &[&str] = &[
    "kExprEnd", "kExprIf", "kExprElse", "kExprUnreachable",
    "kExprBr", "kExprBrTable", "kExprReturn", "kExprDelegate",
    "kExprRethrow", "kExprThrow", "kExprThrowRef", "kExprCatch",
    "kExprCatchAll", "kExprBrIf", "kExprBrOnNull", "kExprBrOnNonNull",
    "kExprBrOnCast", "kExprBrOnCastFail", "kExprBrOnCastDescEq", "kExprBrOnCastDescEqFail",
    "kExprReturnCall", "kExprReturnCallIndirect", "kExprReturnCallRef", "kExprCallFunction",
    "kExprCallIndirect", "kExprCallRef",
];

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
  use crate::table_eq::{contains, str_eq};
  assert!(BLOCK_TERMINATORS.len() == 26);
  assert!(str_eq(BLOCK_TERMINATORS[0], "kExprEnd"));
  assert!(str_eq(BLOCK_TERMINATORS[25], "kExprCallRef"));
  assert!(contains(BLOCK_TERMINATORS, "kExprBrTable"));
  assert!(contains(BLOCK_TERMINATORS, "kExprThrowRef"));
  assert!(BASIC_BLOCK_RULES.len() == 5);
};
