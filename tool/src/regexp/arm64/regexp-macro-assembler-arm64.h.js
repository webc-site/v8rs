#!/usr/bin/env bun
// tool/src/regexp/arm64/regexp-macro-assembler-arm64.h.js
// 机械提取 v8/src/regexp/arm64/regexp-macro-assembler-arm64.h 的
// arm64 正则栈帧布局与缓存寄存器常量, 以及专用寄存器分配

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/regexp/arm64/regexp-macro-assembler-arm64.h");
const K = 8; // kSystemPointerSize / kXRegSize
const W = 4; // kWRegSize

function val(name, src = hSrc) {
  const m = src.match(new RegExp(`static constexpr int ${name} =\s*([^;]+);`));
  if (!m) throw new Error(`未找到 ${name}`);
  return m[1].trim().replace(/\s+/g, " ");
}
function fold(expr, env) {
  let e = expr.replace(/\s/g, "");
  const keys = Object.keys(env).sort((a, b) => b.length - a.length);
  for (const k of keys) e = e.split(k).join(String(env[k]));
  if (!/^-?\d+$/.test(e)) {
    if (!/^-?\d+([+-]\d+)*$/.test(e)) throw new Error(`无法折叠: ${expr} → ${e}`);
    return e.match(/-?\d+/g).reduce((a, t) => a + Number(t), 0);
  }
  return parseInt(e, 10);
}

// 逐项链式折叠 (后项引用前项)
const names = [
  "kFramePointerOffset", "kReturnAddressOffset", "kCalleeSavedRegistersOffset",
  "kFrameTypeOffset", "kIsolateOffset", "kDirectCallOffset",
  "kNumOutputRegistersOffset", "kInputStringOffset", "kSuccessfulCapturesOffset",
  "kBacktrackCountOffset", "kRegExpStackBasePointerOffset", "kStackLocalPadding",
  "kFirstRegisterOnStackOffset", "kFirstCaptureOnStackOffset",
];
const env = {
  kFramePointerOffset: 0, kSystemPointerSize: K, kFrameTypeOffset: -K,
  kPaddingAfterFrameType: K, kWRegSize: W, kXRegSize: K,
};
const v = {};
v.kFramePointerOffset = 0;
for (const n of names.slice(1)) {
  env[n] = fold(val(n), env);
  v[n] = env[n];
}
// 显式常量
const numCalleeSaved = parseInt(val("kNumCalleeSavedRegisters"), 10);
const paddingAfterFrameType = K;
const numberOfStackLocals = parseInt(val("kNumberOfStackLocals"), 10);
const initialBufferSize = parseInt(val("kInitialBufferSize"), 10);
const numRegistersToUnroll = parseInt(val("kNumRegistersToUnroll"), 10);
const numCachedRegisters = parseInt(val("kNumCachedRegisters"), 10);
if (numCalleeSaved !== 10) throw new Error("kNumCalleeSavedRegisters 不符");
if (numberOfStackLocals !== 4) throw new Error("kNumberOfStackLocals 不符");
if (initialBufferSize !== 1024) throw new Error("kInitialBufferSize 不符");
if (numRegistersToUnroll !== 16) throw new Error("kNumRegistersToUnroll 不符");
if (numCachedRegisters !== 16) throw new Error("kNumCachedRegisters 不符");

// 源侧静态锚
if (!hSrc.includes("static_assert(kFrameTypeOffset ==\n                CommonFrameConstants::kContextOrFrameTypeOffset);")) {
  throw new Error("帧类型 static_assert 缺失");
}

// 专用寄存器分配 (原版 current_input_offset() 等; 机械事实)
const regs = {};
for (const [m, name] of [
  [/static constexpr Register current_input_offset\(\) \{ return (\w+); \}/, "current_input_offset"],
  [/static constexpr Register current_character\(\) \{ return (\w+); \}/, "current_character"],
  [/static constexpr Register input_end\(\) \{ return (\w+); \}/, "input_end"],
  [/static constexpr Register input_start\(\) \{ return (\w+); \}/, "input_start"],
  [/static constexpr Register start_offset\(\) \{ return (\w+); \}/, "start_offset"],
  [/static constexpr Register output_array\(\) \{ return (\w+); \}/, "output_array"],
  [/static constexpr Register frame_pointer\(\) \{ return (\w+); \}/, "frame_pointer"],
  [/static constexpr Register backtrack_stackpointer\(\) \{ return (\w+); \}/, "backtrack_stackpointer"],
  [/static constexpr Register code_pointer\(\) \{ return (\w+); \}/, "code_pointer"],
  [/static constexpr Register regexp_stack\(\) \{ return (\w+); \}/, "regexp_stack"],
]) {
  const r = hSrc.match(m);
  if (!r) throw new Error(`未找到寄存器分配 ${name}`);
  regs[name] = r[1];
}

// 校验 (人工对数): 输入串在 rbp-28 (帧类型-8, padding-16, isolate-24)
if (v.kInputStringOffset !== -6 * K) throw new Error(`kInputStringOffset 不符: ${v.kInputStringOffset}`);
if (v.kFirstRegisterOnStackOffset !== v.kStackLocalPadding - W) throw new Error(`kFirstRegisterOnStackOffset 不符: ${v.kFirstRegisterOnStackOffset}`);

console.log(`arm64 frame: inputString=${v.kInputStringOffset} firstReg=${v.kFirstRegisterOnStackOffset} cached=${numCachedRegisters}`);

const field = (label, name) => `  /// 原版 ${name}\n  pub const ${label}: i32 = ${v[name]};\n`;

const rust =
  banner(
    "src/regexp/arm64/regexp-macro-assembler-arm64.h.js",
    "src/regexp/arm64/regexp-macro-assembler-arm64.h",
    "arm64 正则匹配栈帧布局与专用寄存器分配",
  ) +
  `
/// arm64 正则栈帧布局 (rbp 相对偏移; kSystemPointerSize=8, kWRegSize=4)
pub const K_SYSTEM_POINTER_SIZE: i32 = ${K};
pub const K_W_REG_SIZE: i32 = ${W};
${field("FRAME_POINTER_OFFSET", "kFramePointerOffset")}${field("RETURN_ADDRESS_OFFSET", "kReturnAddressOffset")}
/// 原版 kNumCalleeSavedRegisters (x19-x28)
pub const NUM_CALLEE_SAVED_REGISTERS: i32 = ${numCalleeSaved};
${field("CALLEE_SAVED_REGISTERS_OFFSET", "kCalleeSavedRegistersOffset")}${field("FRAME_TYPE_OFFSET", "kFrameTypeOffset")}
/// 原版 kPaddingAfterFrameType
pub const PADDING_AFTER_FRAME_TYPE: i32 = ${paddingAfterFrameType};
${field("ISOLATE_OFFSET", "kIsolateOffset")}${field("DIRECT_CALL_OFFSET", "kDirectCallOffset")}${field("NUM_OUTPUT_REGISTERS_OFFSET", "kNumOutputRegistersOffset")}${field("INPUT_STRING_OFFSET", "kInputStringOffset")}${field("SUCCESSFUL_CAPTURES_OFFSET", "kSuccessfulCapturesOffset")}${field("BACKTRACK_COUNT_OFFSET", "kBacktrackCountOffset")}${field("REGEXP_STACK_BASE_POINTER_OFFSET", "kRegExpStackBasePointerOffset")}${field("STACK_LOCAL_PADDING", "kStackLocalPadding")}
/// 原版 kNumberOfStackLocals
pub const NUMBER_OF_STACK_LOCALS: i32 = ${numberOfStackLocals};
${field("FIRST_REGISTER_ON_STACK_OFFSET", "kFirstRegisterOnStackOffset")}${field("FIRST_CAPTURE_ON_STACK_OFFSET", "kFirstCaptureOnStackOffset")}
/// 首次匹配缓冲 (原版 kInitialBufferSize)
pub const INITIAL_BUFFER_SIZE: i32 = ${initialBufferSize};

/// 展开初始化循环的寄存器数上限 (原版 kNumRegistersToUnroll)
pub const NUM_REGISTERS_TO_UNROLL: i32 = ${numRegistersToUnroll};

/// x0-x7 缓存捕获寄存器数 (每硬件寄存器存一对 position; 原版 kNumCachedRegisters)
pub const NUM_CACHED_REGISTERS: i32 = ${numCachedRegisters};

/// 专用寄存器分配 (原版静态访问器; 名称 → arm64 寄存器)
pub const REGISTERS: &[(&str, &str)] = &[
${Object.entries(regs).map(([k, r]) => `  ("${k}", "${r}"),`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(FRAME_TYPE_OFFSET == -K_SYSTEM_POINTER_SIZE);
  assert!(INPUT_STRING_OFFSET == -6 * K_SYSTEM_POINTER_SIZE);
  assert!(FIRST_REGISTER_ON_STACK_OFFSET == STACK_LOCAL_PADDING - K_W_REG_SIZE);
  assert!(NUM_CACHED_REGISTERS == 16);
  assert!(!REGISTERS.is_empty());
};
`;

const p = writeSrc("v8-regexp-arm64", "frame_layout.rs", rust);
console.log(`written ${p}`);
