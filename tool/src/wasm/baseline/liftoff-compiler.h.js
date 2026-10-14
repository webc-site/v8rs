#!/usr/bin/env bun
// tool/src/wasm/baseline/liftoff-compiler.h.js
// 机械提取 v8/src/wasm/baseline/liftoff-compiler.{h,cc} 的 tier-up 预算与编译器契约常量

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/baseline/liftoff-compiler.h");
const ccSrc = readV8("src/wasm/baseline/liftoff-compiler.cc");

// 1. h: 编译入口签名
if (!hSrc.includes("WasmCompilationResult ExecuteLiftoffCompilation(")) {
  throw new Error("未找到 ExecuteLiftoffCompilation");
}

// 2. cc: 类契约
for (const a of [
  "class LiftoffCompiler {",
  "using ValidationTag = Decoder::NoValidationTag;",
  "static constexpr bool kUsesPoppedArgs = false;",
  "using FullDecoder = WasmFullDecoder<ValidationTag, LiftoffCompiler>;",
]) {
  if (!ccSrc.includes(a)) throw new Error(`类契约缺失: ${a}`);
}

// 3. cc: tier-up 成本常量
const tierMatch = ccSrc.match(
  /static constexpr int kTierUpCostForCheck = (\d+);\s*static constexpr int kTierUpCostForFunctionEntry = (\d+);/,
);
if (!tierMatch) throw new Error("未找到 tier-up 成本常量");
const tierCheckCost = parseInt(tierMatch[1], 10);
const tierEntryCost = parseInt(tierMatch[2], 10);

// 4. cc: 预算钳位 (单次最多用掉总预算 1/4)
if (!ccSrc.includes("const int max_budget_use = std::max(1, v8_flags.wasm_tiering_budget / 4);")) {
  throw new Error("未找到预算钳位公式");
}
if (!ccSrc.includes("budget_used += kTierUpCostForCheck;")) {
  throw new Error("未找到 budget_used 附加检查成本");
}

// 5. cc: 快路径常量记录数
const recMatch = ccSrc.match(/static constexpr size_t kNumRecordedConstants = (\d+);/);
if (!recMatch) throw new Error("未找到 kNumRecordedConstants");
const numRecorded = parseInt(recMatch[1], 10);
if (!ccSrc.includes("std::array<uint32_t, kNumRecordedConstants> recorded_constants_;")) {
  throw new Error("未找到 recorded_constants_ 数组");
}

// 6. cc: 签名内联存储
const inlineMatch = ccSrc.match(/static constexpr size_t kInlineStorage = (\d+);/);
if (!inlineMatch) throw new Error("未找到 kInlineStorage");
const inlineStorage = parseInt(inlineMatch[1], 10);
if (!ccSrc.includes("class MostlySmallValueKindSig")) throw new Error("未找到 MostlySmallValueKindSig");

// 7. cc: 调用描述符第一输入索引
const firstInput = ccSrc.match(/static constexpr uint32_t kFirstInputIdx = (\d+);/);
if (!firstInput) throw new Error("未找到 kFirstInputIdx");

// 8. cc: 判定用小枚举
for (const e of [
  "enum AssumeSpilling",
  "enum ForceCheck : bool { kDoForceCheck = true, kDontForceCheck = false };",
  "enum AlignmentCheck : bool",
  "enum TypeConversionTrapping : bool { kCanTrap = true, kNoTrap = false };",
]) {
  if (!ccSrc.includes(e)) throw new Error(`枚举缺失: ${e}`);
}

// 9. cc: OOL trap 机制
if (!ccSrc.includes("class OolTrapLabel")) throw new Error("未找到 OolTrapLabel");
if (!ccSrc.includes("OolTrapLabel AddOutOfLineTrap(FullDecoder* decoder, Builtin builtin)")) {
  throw new Error("未找到 AddOutOfLineTrap");
}

// 10. cc: tierup 的寄存器保留规则
if (!ccSrc.includes("// The cached instance data and memory start will be reloaded separately.")) {
  throw new Error("未找到 tierup 寄存器保留注释");
}

// 11. cc: debug 模式不 tierup
if (!ccSrc.includes("if (for_debugging_ != kNotForDebugging) return;")) {
  throw new Error("未找到 debug 禁 tierup");
}

// 12. 接口函数实现规模 (FullDecoder 回调签名)
const methods = [...new Set([
  ...ccSrc.matchAll(/\n  (?:V8_INLINE\s+)?(?:void|bool|int|uint32_t|std::optional<[\w:<>*]+>|LiftoffRegister|\w+Ptr) (\w+)\(FullDecoder\* decoder/g)
].map((m) => m[1]))];
if (methods.length < 100) throw new Error(`方法面 ${methods.length} < 100`);

console.log(
  `liftoff-compiler: tierCheck=${tierCheckCost} tierEntry=${tierEntryCost} recorded=${numRecorded} inline=${inlineStorage} 方法面=${methods.length}`,
);

// 13. 生成 Rust
let rust = banner(
  "src/wasm/baseline/liftoff-compiler.h.js",
  "src/wasm/baseline/liftoff-compiler.{h,cc}",
  "Liftoff 编译器 tier-up 预算与解码接口契约",
);

rust += `
//! Liftoff 基线编译器: 已验证代码的 NoValidation 重解码 + 直接产码。
/// v8: src/wasm/baseline/liftoff-compiler.cc LiftoffCompiler

/// 解码契约
/// v8: src/wasm/baseline/liftoff-compiler.cc LiftoffCompiler 类型别名
pub const USES_NO_VALIDATION_TAG: bool = true;
pub const USES_POPPED_ARGS: bool = false;

/// tier-up 预算成本 (以执行机器码字节数估计耗时)
/// v8: src/wasm/baseline/liftoff-compiler.cc kTierUpCostForCheck
pub const TIER_UP_COST_FOR_CHECK: i32 = ${tierCheckCost};
/// v8: 函数序言 frame setup stub 的成本
pub const TIER_UP_COST_FOR_FUNCTION_ENTRY: i32 = ${tierEntryCost};

/// 单次 tier-up 检查最多消耗总预算的 1/4 (至少 1)
/// v8: src/wasm/baseline/liftoff-compiler.cc TierupCheck
pub fn clamp_budget_use(budget_used: i32, wasm_tiering_budget: i32) -> i32 {
    let with_check = budget_used + TIER_UP_COST_FOR_CHECK;
    let max_budget_use = (wasm_tiering_budget / 4).max(1);
    with_check.min(max_budget_use)
}

/// load/store 快路径记录的常量槽数
/// v8: src/wasm/baseline/liftoff-compiler.cc kNumRecordedConstants
pub const NUM_RECORDED_CONSTANTS: usize = ${numRecorded};

/// 签名 kinds 内联存储 (超出走 zone 分配)
/// v8: src/wasm/baseline/liftoff-compiler.cc MostlySmallValueKindSig::kInlineStorage
pub const SIG_INLINE_STORAGE: usize = ${inlineStorage};

/// 调用描述符第一输入索引 (target+instance 之后)
/// v8: src/wasm/baseline/liftoff-compiler.cc kFirstInputIdx
pub const FIRST_INPUT_IDX: u32 = ${firstInput[1]};

/// OOL trap: 带破坏性语义的指令先跳 OOL 标签
/// v8: src/wasm/baseline/liftoff-compiler.cc OolTrapLabel / AddOutOfLineTrap
pub const OOL_TRAP_MECHANISM: bool = true;

/// debug 编译不做 tier-up 检查
/// v8: src/wasm/baseline/liftoff-compiler.cc TierupCheck 首行
pub const DEBUGGING_SKIPS_TIERUP: bool = true;

/// 接口方法实现数 (WasmFullDecoder 接口面)
/// v8: src/wasm/baseline/liftoff-compiler.cc LiftoffCompiler::*
pub const IMPLEMENTED_METHODS_COUNT: usize = ${methods.length};

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(TIER_UP_COST_FOR_CHECK, 20);
    assert_eq!(TIER_UP_COST_FOR_FUNCTION_ENTRY, 40);
    assert_eq!(clamp_budget_use(0, 1000), 20);
    assert_eq!(clamp_budget_use(5000, 1000), 250);
    assert_eq!(NUM_RECORDED_CONSTANTS, 9);
    assert_eq!(SIG_INLINE_STORAGE, 8);
    assert!(IMPLEMENTED_METHODS_COUNT >= 100);
};
`;

const p = writeSrc("v8-wasm-liftoff", "liftoff_compiler.rs.skel", rust);
console.log(`written ${p}`);
