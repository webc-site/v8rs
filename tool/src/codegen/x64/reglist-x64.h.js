#!/usr/bin/env bun
// tool/src/codegen/x64/reglist-x64.h.js
// 机械提取 v8/src/codegen/x64/reglist-x64.h: 寄存器列表别名 + caller-saved 位集
// → v8-codegen-x64/src/reglist.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/codegen/x64/reglist-x64.h");

const m = (re, what) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

// 类型别名对数
for (const a of ["using RegList = RegListBase<Register>;", "using DoubleRegList = RegListBase<DoubleRegister>;",
  "using Simd128RegList = RegListBase<Simd128Register>;"]) {
  if (!src.includes(a)) throw new Error(`缺别名 ${a}`);
}
const numJsCallerSaved = m(/constexpr int kNumJSCallerSaved = (\d+);/, "kNumJSCallerSaved");
if (numJsCallerSaved[1] !== "5") throw new Error("kNumJSCallerSaved 异常");

// kJSCallerSaved: rax,rcx,rdx,rbx,rdi (5) — 先剥行尾注释再解析
const jsCallerBody = m(/constexpr RegList kJSCallerSaved = \{([\s\S]*?)\};/, "kJSCallerSaved")[1]
  .replace(/\/\/.*$/gm, "");
const jsCallerRegs = jsCallerBody
  .split(",")
  .map((s) => s.trim())
  .filter((s) => /^[a-z0-9]+$/i.test(s));
if (jsCallerRegs.length !== 5) throw new Error(`kJSCallerSaved ${jsCallerRegs.length} != 5: ${jsCallerRegs}`);

// kCallerSaved 双分支: WIN / 非 WIN
const winBranch = m(/#ifdef V8_TARGET_OS_WIN\s*\{([^}]*)\};\s*#else\s*\{([^}]*)\};/, "kCallerSaved 分支");
const winRegs = winBranch[1].split(",").map((s) => s.trim()).filter(Boolean);
const nixRegs = winBranch[2].split(",").map((s) => s.trim()).filter(Boolean);
if (winRegs.length !== 7) throw new Error(`WIN caller-saved ${winRegs.length} != 7`);
if (nixRegs.length !== 9) throw new Error(`非 WIN caller-saved ${nixRegs.length} != 9`);

// 寄存器名 → 位码 (x64: rax=0..rdi=7, r8=8..r15=15)
function codeOf(reg) {
  const map = { rax: 0, rcx: 1, rdx: 2, rbx: 3, rsp: 4, rbp: 5, rsi: 6, rdi: 7 };
  if (map[reg] !== undefined) return map[reg];
  const m = reg.match(/^r(\d+)$/);
  if (m) return parseInt(m[1], 10);
  throw new Error(`未知寄存器 ${reg}`);
}
function bits(regs) {
  let b = 0n;
  for (const r of regs) b |= 1n << BigInt(codeOf(r));
  return b;
}
const jsBits = bits(jsCallerRegs);
const winBits = bits(winRegs);
const nixBits = bits(nixRegs);

let rust = banner(
  "src/codegen/x64/reglist-x64.h.js",
  "src/codegen/x64/reglist-x64.h",
  "x64 寄存器列表 + caller-saved 位集",
);

rust += `
// 寄存器位码: rax=0 rcx=1 rdx=2 rbx=3 rsp=4 rbp=5 rsi=6 rdi=7, r8..r15 = 8..15

/// 原版 kJSCallerSaved: rax, rcx, rdx, rbx(JS 侧 caller-saved), rdi(callee 函数)
pub const K_JS_CALLER_SAVED: u64 = 0b${jsBits.toString(2)};
/// 原版 kNumJSCallerSaved
pub const K_NUM_JS_CALLER_SAVED: usize = ${numJsCallerSaved[1]};

/// 原版 kCallerSaved (V8_TARGET_OS_WIN 分支): rax, rcx, rdx, r8..r11
pub const K_CALLER_SAVED_WIN: u64 = 0b${winBits.toString(2)};

/// 原版 kCallerSaved (非 WIN: Linux/macOS, 本目标生效分支): rax, rcx, rdx, rdi, rsi, r8..r11
pub const K_CALLER_SAVED: u64 = 0b${nixBits.toString(2)};

/// 原版 using RegList = RegListBase<Register> (基类见 v8-codegen-core::register_base)
pub type RegList = u64;
/// 原版 using DoubleRegList = RegListBase<DoubleRegister>
pub type DoubleRegList = u64;
/// 原版 using Simd128RegList = RegListBase<Simd128Register>
pub type Simd128RegList = u64;

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_JS_CALLER_SAVED.count_ones() as usize == K_NUM_JS_CALLER_SAVED);
  assert!(K_NUM_JS_CALLER_SAVED == 5);
  assert!(K_CALLER_SAVED_WIN.count_ones() == 7);
  assert!(K_CALLER_SAVED.count_ones() == 9);
  // 抽查位: rax=bit0, rdi=bit7, r11=bit11
  assert!(K_JS_CALLER_SAVED & 1 != 0);
  assert!(K_JS_CALLER_SAVED & (1 << 7) != 0);
  assert!(K_CALLER_SAVED & (1 << 11) != 0);
  assert!(K_CALLER_SAVED & (1 << 6) != 0); // rsi 在非 WIN 分支
  assert!(K_CALLER_SAVED_WIN & (1 << 6) == 0); // WIN 分支无 rsi
};
`;

const p = writeSrc("v8-codegen-x64", "reglist.rs", rust);
console.log(
  `reglist-x64: 别名 3 / kJSCallerSaved 5 寄存器 / kCallerSaved 7(WIN)+9(非WIN) → ${p}`,
);
