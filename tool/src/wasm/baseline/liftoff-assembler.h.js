#!/usr/bin/env bun
// tool/src/wasm/baseline/liftoff-assembler.h.js
// 机械提取 v8/src/wasm/baseline/liftoff-assembler.h 的帧布局常量与控制流枚举

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/baseline/liftoff-assembler.h");

function must(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// 1. 栈槽宽
const slotMatch = must(/static constexpr int kStackSlotSize = (\d+);/, "kStackSlotSize");
const stackSlotSize = parseInt(slotMatch[1], 10);
if (stackSlotSize !== 8) throw new Error(`kStackSlotSize 不符: ${stackSlotSize}`);

// 2. SpillLocation
const spillMatch = must(/enum class SpillLocation \{ ([^}]*) \};/, "SpillLocation");
const spills = spillMatch[1].split(",").map((s) => s.trim());
if (spills.join(",") !== "kTopOfStack,kStackSlots") throw new Error(`SpillLocation 不符: ${spills}`);

// 3. 哨兵
const noCachedMemMatch = must(/static constexpr int kNoCachedMemIndex = (-?\d+);/, "kNoCachedMemIndex");
const noCachedMemIndex = parseInt(noCachedMemMatch[1], 10);

// 4. 内联局部槽数
const inlineMatch = must(/static constexpr uint32_t kInlineLocalKinds = (\d+);/, "kInlineLocalKinds");
const inlineLocalKinds = parseInt(inlineMatch[1], 10);
if (inlineLocalKinds !== 16) throw new Error(`kInlineLocalKinds 不符: ${inlineLocalKinds}`);

// 5. 控制流枚举
const ilMatch = must(/enum IgnoreLocals : bool \{ kIncludeLocals = false, kIgnoreLocals = true \};/, "IgnoreLocals");
const jdMatch = must(/enum JumpDirection \{ kForwardJump, kBackwardJump \};/, "JumpDirection");
const endMatch = must(/enum Endianness \{ kNative, kLittle \};/, "Endianness");
const smiMatch = must(/enum SmiCheckMode \{ kJumpOnSmi, kJumpOnNotSmi \};/, "SmiCheckMode");

// 6. kSupportsStoreConst 双分支 (V8_TARGET_ARCH 分支宏, 两种取值)
const sscTrue = hSrc.match(/kSupportsStoreConst = true;/);
const sscFalse = hSrc.match(/kSupportsStoreConst = false;/);
if (!sscTrue || !sscFalse) throw new Error("未找到 kSupportsStoreConst 双分支");

console.log(`liftoff-assembler.h: enums ok, kStackSlotSize=${stackSlotSize}`);

let rust = banner(
  "src/wasm/baseline/liftoff-assembler.h.js",
  "src/wasm/baseline/liftoff-assembler.h",
  "Liftoff 汇编器帧布局常量与控制流枚举",
);

rust += `
/// 栈槽宽 (原版 kStackSlotSize)
pub const STACK_SLOT_SIZE: i32 = ${stackSlotSize};

/// 内联局部类别槽位数 (原版 kInlineLocalKinds; 超出转堆数组)
pub const INLINE_LOCAL_KINDS: u32 = ${inlineLocalKinds};

/// 无缓存内存起始下标记 (原版 kNoCachedMemIndex)
pub const NO_CACHED_MEM_INDEX: i32 = ${noCachedMemIndex};

/// 寄存器溢出目标 (对应原版 CacheState::SpillLocation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SpillLocation {
  /// kTopOfStack — push 方式溢出到栈顶
  TopOfStack = 0,
  /// kStackSlots — 写回各自栈槽
  StackSlots = 1,
}

/// 分支时局部处理 (对应原版 IgnoreLocals)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(bool)]
pub enum IgnoreLocals {
  /// kIncludeLocals
  IncludeLocals = false,
  /// kIgnoreLocals
  IgnoreLocals = true,
}

/// 栈合并方向 (对应原版 JumpDirection)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JumpDirection {
  /// kForwardJump
  ForwardJump = 0,
  /// kBackwardJump
  BackwardJump = 1,
}

/// 字节序选择 (对应原版 Endianness)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Endianness {
  /// kNative
  Native = 0,
  /// kLittle
  Little = 1,
}

/// Smi 检查分支 (对应原版 SmiCheckMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SmiCheckMode {
  /// kJumpOnSmi
  JumpOnSmi = 0,
  /// kJumpOnNotSmi
  JumpOnNotSmi = 1,
}

/// 是否支持立即数存储 (原版 kSupportsStoreConst; 按架构两分支)
pub const SUPPORTS_STORE_CONST: bool = true;
/// 不支持立即数存储的架构登记 (源内另一分支取值)
pub const SUPPORTS_STORE_CONST_ARCH_BRANCH: bool = false;

// —— 提取不变式断言 ——
const _: () = {
  assert!(STACK_SLOT_SIZE == 8);
  assert!(INLINE_LOCAL_KINDS == 16);
  assert!(NO_CACHED_MEM_INDEX == -1);
  assert!(SUPPORTS_STORE_CONST);
};
`;

const p = writeSrc("v8-wasm-liftoff", "liftoff_assembler_consts.rs", rust);
console.log(`written ${p}`);
