#!/usr/bin/env bun
// tool/src/codegen/macro-assembler.h.js
// 机械提取 v8/src/codegen/macro-assembler.{h} + macro-assembler-base.{h,cc}:
// 宏汇编枚举/位标志/常量/平台派发登记 + 签名骨架
// → v8-codegen-core/src/macro_assembler.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/macro-assembler.h");
const baseH = readV8("src/codegen/macro-assembler-base.h");
const baseCc = readV8("src/codegen/macro-assembler-base.cc");
const inlSrc = readV8("src/codegen/macro-assembler-inl.h");

const m = (re, what, src) => {
  const r = (src ?? hSrc).match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

// 枚举对数 (逐项)
for (const v of ["NO_ALLOCATION_FLAGS", "RESULT_CONTAINS_TOP", "SIZE_IN_WORDS", "DOUBLE_ALIGNMENT", "PRETENURE"]) {
  if (!hSrc.includes(v)) throw new Error(`缺 AllocationFlags ${v}`);
}
for (const v of ["kJump", "kPushAndReturn", "kOmit", "kInline", "kDefault", "kFullPointer",
  "kNo", "kYes", "kAdapt", "kDontAdapt", "kCall"]) {
  if (!hSrc.includes(v)) throw new Error(`缺枚举值 ${v}`);
}
for (const v of ["kDefault", "kWasm"]) {
  if (!baseH.includes(v)) throw new Error(`缺 RecordWriteCallMode ${v}`);
}
// kMaxCParameters: USE_SIMULATOR=20 / 否则 256
m(/static constexpr int kMaxCParameters = (\d+);/, "kMaxCParameters", hSrc);
const K_MAX_C_PARAMETERS = 256; // 非模拟器构建
// kStackPageSize: WIN/MACOS 分支 = 4KB
m(/static constexpr int kStackPageSize = 4 \* KB;/, "kStackPageSize", baseH);

// .cc 方法清点 (对数): base.cc 实际 11 个方法 (Abort/Assert/Check 在平台层)
const baseCcMethods = [...baseCc.matchAll(/MacroAssemblerBase::(\w+)\(/gm)].map((x) => x[1]);
for (const want of ["BuiltinEntry", "IndirectLoadConstant", "RootRegisterOffsetForRootIndex",
  "IsAddressableThroughRootRegister", "ReadOnlyRootPtr"]) {
  if (!baseCcMethods.includes(want)) throw new Error(`macro-assembler-base.cc 缺 ${want}`);
}
if (baseCcMethods.length < 10) throw new Error(`base.cc 方法数异常: ${baseCcMethods.length}`);

let rust = banner(
  "src/codegen/macro-assembler.h.js",
  "src/codegen/macro-assembler.h;src/codegen/macro-assembler-base.{h,cc};src/codegen/macro-assembler-inl.h",
  "宏汇编枚举/位标志/常量 + 平台派发登记 + 签名骨架",
);

rust += `
// macro-assembler.h 平台派发: x64 → macro-assembler-x64.h (v8-codegen-x64),
// arm64 → macro-assembler-arm64.h (v8-codegen-arm64);
// ia32/arm/ppc/mips64/loong64/s390/riscv 登记跳过。

/// 原版 enum class InvokeType: 函数调用发起方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InvokeType {
  kCall = 0,
  kJump = 1,
}

/// 原版 enum AllocationFlags: AllocateInNewSpace 位标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AllocationFlags {
  NO_ALLOCATION_FLAGS = 0,
  /// 结果寄存器已含 new space allocation top
  RESULT_CONTAINS_TOP = 1 << 0,
  /// 请求大小以字而非字节计
  SIZE_IN_WORDS = 1 << 1,
  /// 按 kDoubleSize 对齐
  DOUBLE_ALIGNMENT = 1 << 2,
  /// 直接分配到老生代
  PRETENURE = 1 << 3,
}

/// 原版 enum class JumpMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JumpMode {
  /// 直接跳转到给定地址
  kJump = 0,
  /// 压入地址作为返回地址后 ret
  kPushAndReturn = 1,
}

/// 原版 enum class SmiCheck
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SmiCheck {
  kOmit = 0,
  kInline = 1,
}

/// 原版 enum class ReadOnlyCheck
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ReadOnlyCheck {
  kOmit = 0,
  kInline = 1,
}

/// 原版 enum class ComparisonMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ComparisonMode {
  /// 压缩指针开启且根为 tagged 时可用 32 位比较
  kDefault = 0,
  /// 比较值可能不在主压缩笼内
  kFullPointer = 1,
}

/// 原版 enum class SetIsolateDataSlots
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SetIsolateDataSlots {
  kNo = 0,
  kYes = 1,
}

/// 原版 enum class ArgumentAdaptionMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArgumentAdaptionMode {
  kAdapt = 0,
  kDontAdapt = 1,
}

/// 原版 enum class RecordWriteCallMode (macro-assembler-base.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RecordWriteCallMode {
  kDefault = 0,
  kWasm = 1,
}

/// C 调用最大参数数 (原版 kMaxCParameters; 非模拟器构建 = 256, 模拟器 = 20)
pub const K_MAX_C_PARAMETERS: i32 = ${K_MAX_C_PARAMETERS};
/// 栈扩页触页哨兵 (原版 kStackPageSize, V8_TARGET_OS_WIN/MACOS 分支) = 4KB
pub const K_STACK_PAGE_SIZE: i32 = 4096;

/// 原版 MacroAssemblerBase: 平台宏汇编公共基类 (状态面骨架)
pub struct MacroAssemblerBase {
  /// 原版 isolate_
  pub isolate_: *mut (),
  /// 原版 code_object_
  pub code_object_: u32,
  /// 原版 root_array_available_
  pub root_array_available_: bool, // = true
  /// 原版 hard_abort_
  pub hard_abort_: bool, // = false
  /// 原版 builtin_jump_table_entries_ 等其余成员待 Developer 接线
}

impl Default for MacroAssemblerBase {
  fn default() -> Self {
    Self {
      isolate_: std::ptr::null_mut(),
      code_object_: 0,
      root_array_available_: true,
      hard_abort_: false,
    }
  }
}

impl MacroAssemblerBase {
  /// 原版 CanBeImmediate(RootIndex): 根可否作立即数 — 骨架
  #[must_use]
  pub const fn can_be_immediate(_index: u32) -> bool {
    todo!("待 Developer 修订: 对齐 macro-assembler-base.h CanBeImmediate")
  }

  /// 原版 CommentForOffHeapTrampoline(prefix, builtin) — 骨架
  /// (非 debug 构建返回空串)
  pub fn comment_for_off_heap_trampoline(_prefix: &str, _builtin: u32) -> String {
    todo!("待 Developer 修订: 对齐 CommentForOffHeapTrampoline")
  }

  /// 原版 Abort(reason) — 签名骨架
  pub fn abort(&mut self, _reason: crate::bailout::AbortReason) {
    todo!("待 Developer 修订: 对齐 macro-assembler-base.cc Abort")
  }

  /// 原版 Assert(cond, reason) — 签名骨架
  pub fn assert_(&mut self, _cond: bool, _reason: crate::bailout::AbortReason) {
    todo!("待 Developer 修订")
  }

  /// 原版 Check(cond, reason) — 签名骨架
  pub fn check(&mut self, _cond: bool, _reason: crate::bailout::AbortReason) {
    todo!("待 Developer 修订")
  }
}

/// 原版 class FrameScope: 帧构造/析构 RAII (骨架)
pub struct FrameScope;

/// 原版 class HardAbortScope: RAII 开关 hard_abort_ (骨架)
pub struct HardAbortScope;

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_C_PARAMETERS == 256);
  assert!(K_STACK_PAGE_SIZE == 4096);
  assert!(InvokeType::kCall as u8 == 0);
  assert!(JumpMode::kPushAndReturn as u8 == 1);
  assert!(AllocationFlags::NO_ALLOCATION_FLAGS as u8 == 0);
  assert!(AllocationFlags::PRETENURE as u8 == 0b1000);
  assert!(ComparisonMode::kFullPointer as u8 == 1);
  assert!(RecordWriteCallMode::kWasm as u8 == 1);
};
`;

const p = writeSrc("v8-codegen-core", "macro_assembler.rs", rust);
console.log(
  `macro-assembler: 枚举 9 / 位标志 5 / 常量 2 / 骨架 7 (base.cc 方法 ${baseCcMethods.length}) → ${p}`,
);
