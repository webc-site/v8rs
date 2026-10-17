#!/usr/bin/env bun
// tool/src/wasm/interpreter/wasm-interpreter.h.js
// 机械提取 v8/src/wasm/interpreter/wasm-interpreter.h 的
// 解释器(Drumbrake)常量、状态枚举与 kRegModes[256] 寄存器优化表

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/interpreter/wasm-interpreter.h");

function must(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}
const MB = 1024 * 1024;

// 1. 栈常量
const initialStackSize = 1 * MB;
const stackSizeIncrement = 1 * MB;
const maxStackSize = 32 * MB;
must(/static constexpr uint32_t kInitialStackSize = 1 \* MB;/, "kInitialStackSize");
must(/static constexpr uint32_t kStackSizeIncrement = 1 \* MB;/, "kStackSizeIncrement");
must(/static constexpr uint32_t kMaxStackSize = 32 \* MB;/, "kMaxStackSize");
must(/static constexpr uint32_t kInterpreterStackGuardSize = kStackSizeIncrement;/, "kInterpreterStackGuardSize");
const interpreterStackReservationSize = maxStackSize + stackSizeIncrement; // 原版公式

// 2. 状态枚举
const stateMatch = must(/enum State \{ STOPPED, RUNNING, FINISHED, TRAPPED, EH_UNWINDING \};/, "State");
const stateNames = ["STOPPED", "RUNNING", "FINISHED", "TRAPPED", "EH_UNWINDING"];
const ehrMatch = must(/enum ExceptionHandlingResult \{ HANDLED, UNWOUND \};/, "ExceptionHandlingResult");
const ecrMatch = must(/enum ExternalCallResult \{[\s\S]*?EXTERNAL_RETURNED,[\s\S]*?EXTERNAL_EXCEPTION\n\};/, "ExternalCallResult");

// 3. EH 索引哨兵
must(/static const int kCatchAllTagIndex = -1;/, "kCatchAllTagIndex");
must(/static const int kDelegateToCallerIndex = 0;/, "kDelegateToCallerIndex");
must(/static const int kMaxPercentValue = 100000;/, "kMaxPercentValue");

// 4. OperatorMode
const omMatch = must(/enum OperatorMode \{ kR2R = 0, kR2S, kS2R, kS2S \};/, "OperatorMode");
must(/static const size_t kOperatorModeCount = 4;/, "kOperatorModeCount");

// 5. RegMode
const rmBody = must(/enum class RegMode \{([\s\S]*?)\n\};/, "RegMode")[1];
const regModes = [...rmBody.matchAll(/^\s*k(\w+),/gm)].map((m) => m[1]);
if (regModes.join(",") !== "NoReg,I32Reg,I64Reg,F32Reg,F64Reg,AnyReg") {
  throw new Error(`RegMode 内容不符: ${regModes}`);
}

// 6. kRegModes[256] 表
const tableMatch = must(/static const RegModeTransform kRegModes\[256\] = \{([\s\S]*?)\n\};/, "kRegModes");
const regModeValues = {};
for (const [i, name] of regModes.entries()) regModeValues[name] = i;
const entries = [...tableMatch[1].matchAll(/\{\s*RegMode::k(\w+),\s*RegMode::k(\w+)\s*\}/g)].map(
  (m) => [regModeValues[m[1]], regModeValues[m[2]]],
);
if (entries.length !== 256) {
  throw new Error(`kRegModes 条数不符: ${entries.length}, 期望 256`);
}
// 槽位/偏移/指令表
must(/static const size_t kSlotSize = sizeof\(int32_t\);/, "kSlotSize");
must(/static const ptrdiff_t kCodeOffsetSize = sizeof\(int32_t\);/, "kCodeOffsetSize");
const kInstructionTableSize = 4096;
must(/static constexpr uint32_t kInstructionTableSize = 4096;/, "kInstructionTableSize");
must(/static constexpr uint32_t kInstructionTableMask = kInstructionTableSize - 1;/, "kInstructionTableMask");
must(/constexpr uint32_t kBranchOnCastDataTargetTypeBitSize = 30;/, "kBranchOnCastDataTargetTypeBitSize");
must(/static const CodeOffset kInvalidCodeOffset = \(CodeOffset\)-1;/, "kInvalidCodeOffset");
must(/static const int64_t kWriteCountCheckInterval = 1000;/, "kWriteCountCheckInterval");
must(/static const int kFileChunksCount = 10;/, "kFileChunksCount");
must(/static const int64_t kMaxFileSize = 100 \* MB;/, "kMaxFileSize");

console.log(
  `wasm-interpreter.h: State=5 RegMode=${regModes.length} kRegModes=256 stackMax=${maxStackSize >> 20}MB`,
);

// 7. 生成 Rust
let rust = banner(
  "src/wasm/interpreter/wasm-interpreter.h.js",
  "src/wasm/interpreter/wasm-interpreter.h",
  "Wasm 解释器 (Drumbrake) 常量与寄存器优化表",
);

rust += `
/// 栈布局常量 (原版 WasmInterpreterThread 侧)
pub mod stack_limits {
  pub const INITIAL_STACK_SIZE: u32 = ${initialStackSize};
  pub const STACK_SIZE_INCREMENT: u32 = ${stackSizeIncrement};
  pub const MAX_STACK_SIZE: u32 = ${maxStackSize};
  /// 尾部守卫区宽 = 增量 (原版 kInterpreterStackGuardSize)
  pub const INTERPRETER_STACK_GUARD_SIZE: u32 = ${stackSizeIncrement};
  /// 预留总宽 = 上限 + 守卫 (原版 kInterpreterStackReservationSize)
  pub const INTERPRETER_STACK_RESERVATION_SIZE: u32 = ${interpreterStackReservationSize};
}

/// 解释器线程状态 (对应原版 WasmInterpreterThread::State)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InterpreterState {
  /// STOPPED
  Stopped = 0,
  /// RUNNING
  Running = 1,
  /// FINISHED
  Finished = 2,
  /// TRAPPED
  Trapped = 3,
  /// EH_UNWINDING
  EhUnwinding = 4,
}

/// 异常处理结果 (对应原版 ExceptionHandlingResult)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExceptionHandlingResult {
  /// HANDLED
  Handled = 0,
  /// UNWOUND
  Unwound = 1,
}

/// 外部调用结果 (对应原版 ExternalCallResult)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExternalCallResult {
  /// EXTERNAL_RETURNED — 正常返回
  ExternalReturned = 0,
  /// EXTERNAL_EXCEPTION — 抛出异常
  ExternalException = 1,
}

/// catch-all tag 索引哨兵 (原版 WasmEHData::kCatchAllTagIndex)
pub const CATCH_ALL_TAG_INDEX: i32 = -1;

/// 委托给调用者的块索引哨兵 (原版 kDelegateToCallerIndex; 0 恒为主块 id)
pub const DELEGATE_TO_CALLER_INDEX: i32 = 0;

/// 覆盖率采样百分比上限 (原版 kMaxPercentValue)
pub const MAX_PERCENT_VALUE: i32 = 100000;

/// 操作数模式 (对应原版 OperatorMode: 寄存器/栈顶来源组合)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OperatorMode {
  /// kR2R
  R2R = 0,
  /// kR2S
  R2S = 1,
  /// kS2R
  S2R = 2,
  /// kS2S
  S2S = 3,
}
/// 模式总数 (原版 kOperatorModeCount)
pub const OPERATOR_MODE_COUNT: usize = 4;

/// 寄存器优化模式 (对应原版 enum class RegMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RegMode {
`;
for (const [i, name] of regModes.entries()) {
  rust += `  /// k${name}\n  ${name} = ${i},\n`;
}
rust += `}

/// 寄存器模式变换对 (原版 RegModeTransform: from/to)
pub type RegModeTransform = (RegMode, RegMode);

/// 逐 opcode 寄存器优化表 (原版 kRegModes[256], 索引即 wasm opcode)
#[rustfmt::skip]
pub static REG_MODES: [RegModeTransform; 256] = [
`;
for (let i = 0; i < 256; i += 4) {
  const row = entries
    .slice(i, i + 4)
    .map(([f, t]) => `(RegMode::${regModes[f]}, RegMode::${regModes[t]})`);
  rust += `  ${row.join(", ")},\n`;
}
rust += `];

/// 栈槽宽 (原版 kSlotSize = sizeof(int32_t))
pub const SLOT_SIZE: usize = 4;

/// 代码偏移宽 (原版 kCodeOffsetSize = sizeof(int32_t))
pub const CODE_OFFSET_SIZE: usize = 4;

/// 指令表大小 (原版 kInstructionTableSize)
pub const INSTRUCTION_TABLE_SIZE: u32 = ${kInstructionTableSize};

/// 指令表掩码 (原版 kInstructionTableMask)
pub const INSTRUCTION_TABLE_MASK: u32 = ${kInstructionTableSize - 1};

/// cast data 目标类型位宽 (原版 kBranchOnCastDataTargetTypeBitSize)
pub const BRANCH_ON_CAST_DATA_TARGET_TYPE_BIT_SIZE: u32 = 30;

/// 无效代码偏移哨兵 (原版 kInvalidCodeOffset = (CodeOffset)-1)
pub const INVALID_CODE_OFFSET: u32 = u32::MAX;

/// tracing 落盘写次数检查间隔 (原版 kWriteCountCheckInterval)
pub const WRITE_COUNT_CHECK_INTERVAL: i64 = 1000;

/// tracing 文件块数 (原版 kFileChunksCount)
pub const FILE_CHUNKS_COUNT: i32 = 10;

/// tracing 单文件上限 (原版 kMaxFileSize)
pub const MAX_FILE_SIZE: i64 = 100 * 1024 * 1024;

// —— 提取不变式断言 ——
const _: () = {
  assert!(stack_limits::MAX_STACK_SIZE == 32 * 1024 * 1024);
  assert!(stack_limits::INTERPRETER_STACK_RESERVATION_SIZE > stack_limits::MAX_STACK_SIZE);
  assert!(REG_MODES.len() == 256);
  assert!(INSTRUCTION_TABLE_MASK == ${kInstructionTableSize - 1});
  assert!(OPERATOR_MODE_COUNT == 4);
};
`;

const p = writeSrc("v8-wasm-interp", "wasm_interpreter_consts.rs", rust);
console.log(`written ${p}`);
