#!/usr/bin/env bun
// tool/src/regexp/x64/regexp-macro-assembler-x64.h.js
// 机械提取 v8/src/regexp/x64/regexp-macro-assembler-x64.h 的 x64 正则栈帧布局
// 分支展开: V8_TARGET_OS_WIN 开/关两态 (帧参数区 + callee-save 备份)

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/regexp/x64/regexp-macro-assembler-x64.h");
const K = 8; // kSystemPointerSize (x64)

function must(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// 公共帧骨架
must(/static constexpr int kFramePointerOffset = 0;/, "kFramePointerOffset");
const frameAlign = 2 * K; // kFramePointerOffset + 2*kSystemPointerSize
must(/static_assert\(kFrameTypeOffset ==\s*\n\s*CommonFrameConstants::kContextOrFrameTypeOffset\);/, "frame type static_assert");

// WIN 分支 (参数在调用者栈上)
const winSeg = hSrc.match(/#ifdef V8_TARGET_OS_WIN([\s\S]*?)#else([\s\S]*?)#endif\n\n  \/\/ We push callee-save/);
if (!winSeg) throw new Error("未找到 V8_TARGET_OS_WIN 参数区分支");
const winParam = winSeg[1];
const sysvParam = winSeg[2];

function val(src, name) {
  const m = src.match(new RegExp(`static constexpr int ${name} =\s*([^;]+);`));
  if (!m) throw new Error(`未找到 ${name}`);
  return m[1].trim();
}
function fold(expr, env) {
  let e = expr.replace(/\s/g, "");
  // 长名优先替换, 避免前缀串扰
  const keys = Object.keys(env).sort((a, b) => b.length - a.length);
  for (const k of keys) e = e.split(k).join(String(env[k]));
  if (!/^-?\d+$/.test(e)) {
    // 仅支持 +/- 线性组合
    if (!/^-?\d+([+-]\d+)*$/.test(e)) throw new Error(`表达式无法折叠: ${expr} → ${e}`);
    return e.match(/-?\d+/g).reduce((acc, t) => acc + Number(t), 0);
  }
  return parseInt(e, 10);
}

// —— SysV 分支取值 (非 WIN) ——
const sysv = {};
{
  const env = { kFramePointerOffset: 0, kSystemPointerSize: K, kFrameAlign: frameAlign, kFrameTypeOffset: -K };
  sysv.kReturnAddressOffset = K;
  sysv.kFrameTypeOffset = -K;
  // 逐项折叠: 后项引用前项符号, 折叠即入 env
  for (const name of ["kInputStringOffset", "kStartIndexOffset", "kInputStartOffset", "kInputEndOffset", "kRegisterOutputOffset", "kNumOutputRegistersOffset"]) {
    env[name] = fold(val(sysvParam, name), env);
    sysv[name] = env[name];
  }
  sysv.kDirectCallOffset = frameAlign; // kDirectCallOffset = kFrameAlign
  sysv.kIsolateOffset = sysv.kDirectCallOffset + K;
}
// callee-save 两分支
const csSeg = hSrc.match(/#ifdef V8_TARGET_OS_WIN\n  static constexpr int kBackupRsiOffset[\s\S]*?#else([\s\S]*?)#endif/);
if (!csSeg) throw new Error("未找到 callee-save 分支");
const csSysv = csSeg[1];
const csWin = csSeg[0].slice(csSeg[0].indexOf("#ifdef V8_TARGET_OS_WIN"), csSeg[0].indexOf("#else"));

// —— WIN 分支取值 ——
const win = {};
{
  const env = { kFramePointerOffset: 0, kSystemPointerSize: K, kFrameAlign: frameAlign, kFrameTypeOffset: -K };
  const wenv = { ...env };
  for (const [name, expr] of [
    ["kInputStringOffset", "kFrameAlign"],
    ["kStartIndexOffset", "kInputStringOffset + kSystemPointerSize"],
    ["kInputStartOffset", "kStartIndexOffset + kSystemPointerSize"],
    ["kInputEndOffset", "kInputStartOffset + kSystemPointerSize"],
    ["kRegisterOutputOffset", "kInputEndOffset + kSystemPointerSize"],
    ["kNumOutputRegistersOffset", "kRegisterOutputOffset + kSystemPointerSize"],
  ]) {
    win[name] = fold(expr, wenv);
    wenv[name] = win[name];
    // 与源一致性校验 (WIN 分支源码即上述公式)
    if (val(winParam, name).replace(/\s/g, "") !== expr.replace(/\s/g, "")) {
      throw new Error(`WIN ${name} 公式与源不符: ${val(winParam, name)}`);
    }
  }
  win.kDirectCallOffset = win.kNumOutputRegistersOffset + K;
  win.kIsolateOffset = win.kDirectCallOffset + K;
  // 校验与源一致
  if (val(winParam, "kInputStringOffset") !== "kFrameAlign") throw new Error("WIN kInputStringOffset 公式变更");
  // callee-save WIN: rsi/rdi/rbx/r12 递减
  const env2 = { ...env };
  env2.kNumOutputRegistersOffset = win.kNumOutputRegistersOffset;
  win.kBackupRsiOffset = fold(val(csWin, "kBackupRsiOffset"), env2);
  win.kBackupRdiOffset = win.kBackupRsiOffset - K;
  win.kBackupRbxOffset = win.kBackupRdiOffset - K;
  win.kBackupR12Offset = win.kBackupRbxOffset - K;
  win.kNumCalleeSaveRegisters = 4;
  win.kLastCalleeSaveRegister = win.kBackupR12Offset;
}
// SysV callee-save
const sysvCsEnv = { kSystemPointerSize: K, kNumOutputRegistersOffset: sysv.kNumOutputRegistersOffset };
const sysvBackupRbx = fold(val(csSysv, "kBackupRbxOffset"), sysvCsEnv);
sysv.kNumCalleeSaveRegisters = 2;
sysv.kLastCalleeSaveRegister = sysvBackupRbx - K;

// 提取本地变量常量
const localNames = [
  "kSuccessfulCapturesOffset",
  "kStringStartMinusOneOffset",
  "kBacktrackCountOffset",
  "kRegExpStackBasePointerOffset",
  "kRegisterZeroOffset",
];

const sysvLocEnv = { kSystemPointerSize: K, kLastCalleeSaveRegister: sysv.kLastCalleeSaveRegister };
for (const name of localNames) {
  sysvLocEnv[name] = fold(val(hSrc, name), sysvLocEnv);
  sysv[name] = sysvLocEnv[name];
}

const winLocEnv = { kSystemPointerSize: K, kLastCalleeSaveRegister: win.kLastCalleeSaveRegister };
for (const name of localNames) {
  winLocEnv[name] = fold(val(hSrc, name), winLocEnv);
  win[name] = winLocEnv[name];
}

// 代码缓冲区初始尺寸
const regExpCodeSize = fold(val(hSrc, "kRegExpCodeSize"), {});
if (regExpCodeSize !== 1024) throw new Error("kRegExpCodeSize 不符");

// 专用寄存器 (h 中的 4 个访问器 + cc 中的通用约定)
for (const [name, reg] of [
  ["current_character", "rdx"],
  ["backtrack_stackpointer", "rbx"],
  ["code_object_pointer", "r8"],
  ["regexp_stack", "r12"],
]) {
  must(new RegExp(`static constexpr Register ${name}\\(\\) \\{ return ${reg}; \\}`), `寄存器访问器 ${name}`);
}

const regs = {
  current_character: "rdx",
  current_position: "rdi",
  input_end: "rsi",
  frame_pointer: "rbp",
  stack_pointer: "rsp",
  backtrack_stackpointer: "rbx",
  code_object_pointer: "r8",
  regexp_stack: "r12",
};

// 校验 (人工对数)
if (sysv.kInputStringOffset !== -2 * K) throw new Error("SysV kInputStringOffset 不符");
if (sysv.kNumOutputRegistersOffset !== -7 * K) throw new Error("SysV kNumOutputRegistersOffset 不符");
if (sysvBackupRbx !== sysv.kNumOutputRegistersOffset - K) throw new Error("SysV kBackupRbxOffset 不符");
if (sysv.kRegisterZeroOffset !== -14 * K) throw new Error("SysV kRegisterZeroOffset 不符");
if (win.kInputStringOffset !== K * 2) throw new Error("WIN kInputStringOffset 不符");
if (win.kRegisterZeroOffset !== -10 * K) throw new Error("WIN kRegisterZeroOffset 不符");

console.log(`x64 frame: sysv inputString=${sysv.kInputStringOffset} win inputString=${win.kInputStringOffset}`);

function emit(name, v, cs, note) {
  return `/// ${name}${note}
pub mod ${name} {
  pub const FRAME_POINTER_OFFSET: i32 = 0;
  /// 原版 kReturnAddressOffset
  pub const RETURN_ADDRESS_OFFSET: i32 = ${K};
  /// 原版 kFrameTypeOffset (帧类型标记, 位于 rbp 下方)
  pub const FRAME_TYPE_OFFSET: i32 = ${-K};
  /// 原版 kFrameAlign
  pub const FRAME_ALIGN: i32 = ${frameAlign};
  /// 原版 kInputStringOffset
  pub const INPUT_STRING_OFFSET: i32 = ${v.kInputStringOffset};
  /// 原版 kStartIndexOffset
  pub const START_INDEX_OFFSET: i32 = ${v.kStartIndexOffset};
  /// 原版 kInputStartOffset
  pub const INPUT_START_OFFSET: i32 = ${v.kInputStartOffset};
  /// 原版 kInputEndOffset
  pub const INPUT_END_OFFSET: i32 = ${v.kInputEndOffset};
  /// 原版 kRegisterOutputOffset
  pub const REGISTER_OUTPUT_OFFSET: i32 = ${v.kRegisterOutputOffset};
  /// 原版 kNumOutputRegistersOffset
  pub const NUM_OUTPUT_REGISTERS_OFFSET: i32 = ${v.kNumOutputRegistersOffset};
  /// 原版 kDirectCallOffset
  pub const DIRECT_CALL_OFFSET: i32 = ${v.kDirectCallOffset};
  /// 原版 kIsolateOffset
  pub const ISOLATE_OFFSET: i32 = ${v.kIsolateOffset};
${cs}  /// 原版 kNumCalleeSaveRegisters
  pub const NUM_CALLEE_SAVE_REGISTERS: i32 = ${v.kNumCalleeSaveRegisters};
  /// 原版 kLastCalleeSaveRegister
  pub const LAST_CALLEE_SAVE_REGISTER: i32 = ${v.kLastCalleeSaveRegister};
  /// 原版 kSuccessfulCapturesOffset
  pub const SUCCESSFUL_CAPTURES_OFFSET: i32 = ${v.kSuccessfulCapturesOffset};
  /// 原版 kStringStartMinusOneOffset
  pub const STRING_START_MINUS_ONE_OFFSET: i32 = ${v.kStringStartMinusOneOffset};
  /// 原版 kBacktrackCountOffset
  pub const BACKTRACK_COUNT_OFFSET: i32 = ${v.kBacktrackCountOffset};
  /// 原版 kRegExpStackBasePointerOffset
  pub const REGEXP_STACK_BASE_POINTER_OFFSET: i32 = ${v.kRegExpStackBasePointerOffset};
  /// 原版 kRegisterZeroOffset
  pub const REGISTER_ZERO_OFFSET: i32 = ${v.kRegisterZeroOffset};

  /// 正则内部寄存器 (register_index) 在栈上的 rbp 偏移
  /// 对应原版 register_location(index) 寻址：kRegisterZeroOffset - index * 8
  #[inline]
  pub const fn register_location_offset(register_index: i32) -> i32 {
    REGISTER_ZERO_OFFSET - register_index * 8
  }
}

`;
}

const sysvCsBlock = `  /// 原版 kBackupRbxOffset
  pub const BACKUP_RBX_OFFSET: i32 = ${sysvBackupRbx};
  /// 原版 kBackupR12Offset
  pub const BACKUP_R12_OFFSET: i32 = ${sysvBackupRbx - K};
`;
const winCsBlock = `  /// 原版 kBackupRsiOffset
  pub const BACKUP_RSI_OFFSET: i32 = ${win.kBackupRsiOffset};
  /// 原版 kBackupRdiOffset
  pub const BACKUP_RDI_OFFSET: i32 = ${win.kBackupRdiOffset};
  /// 原版 kBackupRbxOffset
  pub const BACKUP_RBX_OFFSET: i32 = ${win.kBackupRbxOffset};
  /// 原版 kBackupR12Offset
  pub const BACKUP_R12_OFFSET: i32 = ${win.kBackupR12Offset};
`;

const rust =
  banner(
    "src/regexp/x64/regexp-macro-assembler-x64.h.js",
    "src/regexp/x64/regexp-macro-assembler-x64.h",
    "x64 正则匹配栈帧布局 (rbp 相对偏移; SysV/Win 两态展开)",
  ) +
  "\n" +
  emit("sysv", sysv, sysvCsBlock, " (V8_TARGET_OS_WIN 关; AMD64 ABI, 参数走寄存器)") +
  emit("win", win, winCsBlock, " (V8_TARGET_OS_WIN 开; 参数在调用者栈上, 备份 rsi/rdi/rbx/r12)") +
  `/// 首次匹配缓冲 (原版 kRegExpCodeSize)
pub const REGEXP_CODE_SIZE: i32 = ${regExpCodeSize};

/// 专用寄存器分配 (原版静态访问器与核心约定; 名称 → x64 寄存器)
pub const REGISTERS: &[(&str, &str)] = &[
${Object.entries(regs).map(([k, r]) => `  ("${k}", "${r}"),`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(sysv::FRAME_TYPE_OFFSET == -(sysv::RETURN_ADDRESS_OFFSET));
  assert!(sysv::INPUT_STRING_OFFSET == sysv::FRAME_TYPE_OFFSET - 8);
  assert!(sysv::NUM_OUTPUT_REGISTERS_OFFSET == sysv::INPUT_STRING_OFFSET - 5 * 8);
  assert!(sysv::NUM_CALLEE_SAVE_REGISTERS == 2);
  assert!(sysv::SUCCESSFUL_CAPTURES_OFFSET == sysv::LAST_CALLEE_SAVE_REGISTER - 8);
  assert!(sysv::STRING_START_MINUS_ONE_OFFSET == sysv::SUCCESSFUL_CAPTURES_OFFSET - 8);
  assert!(sysv::BACKTRACK_COUNT_OFFSET == sysv::STRING_START_MINUS_ONE_OFFSET - 8);
  assert!(sysv::REGEXP_STACK_BASE_POINTER_OFFSET == sysv::BACKTRACK_COUNT_OFFSET - 8);
  assert!(sysv::REGISTER_ZERO_OFFSET == sysv::REGEXP_STACK_BASE_POINTER_OFFSET - 8);

  assert!(win::INPUT_STRING_OFFSET == win::FRAME_ALIGN);
  assert!(win::NUM_CALLEE_SAVE_REGISTERS == 4);
  assert!(win::SUCCESSFUL_CAPTURES_OFFSET == win::LAST_CALLEE_SAVE_REGISTER - 8);
  assert!(win::STRING_START_MINUS_ONE_OFFSET == win::SUCCESSFUL_CAPTURES_OFFSET - 8);
  assert!(win::BACKTRACK_COUNT_OFFSET == win::STRING_START_MINUS_ONE_OFFSET - 8);
  assert!(win::REGEXP_STACK_BASE_POINTER_OFFSET == win::BACKTRACK_COUNT_OFFSET - 8);
  assert!(win::REGISTER_ZERO_OFFSET == win::REGEXP_STACK_BASE_POINTER_OFFSET - 8);

  assert!(REGEXP_CODE_SIZE == 1024);
  assert!(!REGISTERS.is_empty());
};
`;

const p = writeSrc("v8-regexp-x64", "frame_layout.rs", rust);
console.log(`written ${p}`);
