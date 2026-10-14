#!/usr/bin/env bun
// tool/src/execution/x64/frame-constants-x64.js
// 机械提取 v8/src/execution/x64/frame-constants-x64.{h,cc} — x64 帧常量规则代码
// 产出: v8-execution-frames/src/frame_constants_x64.rs
// 规则: 常量表/位标志/宏机械展开 (#ifdef V8_TARGET_OS_WIN 双分支展开, 主目标非 WIN) / .cc 签名骨架逐条摘录
// 宏展开基准 (展开自 v8/src/execution/frame-constants.h, x64 无常量池):
//   kCPSlotSize=0, kFrameTypeSize=8 → TypedFrameConstants::kFixedFrameSizeFromFp=8
//   TYPED_FRAME_PUSHED_VALUE_OFFSET(x) = -16 - 8x
// 复跑幂等: 纯文本扫描 + 确定性求值, 无随机无时钟。
import { banner, readV8, writeSrc } from "../../../lib.js";

const GEN = "src/execution/x64/frame-constants-x64.js";
const H_REL = "src/execution/x64/frame-constants-x64.h";
const CC_REL = "src/execution/x64/frame-constants-x64.cc";
const hSrc = readV8(H_REL);
const ccSrc = readV8(CC_REL);
const die = (m) => { throw new Error(`[frame-constants-x64.js] ${m}`); };

// ── 公共基 (机械事实, 展开自 frame-constants.h / common/globals.h) ──
const BASE = {
  kSystemPointerSize: 8,
  kSimd128Size: 16,
  kCPSlotSize: 0, // V8_EMBEDDED_CONSTANT_POOL_BOOL=false (x64)
  kFrameTypeSize: 8,
  kFixedFrameSizeFromFp: 8, // TypedFrameConstants 口径
  kFirstPushedFrameValueOffset: -16,
};

// ── x64 寄存器编码 (展开自 src/codegen/x64/register-x64.h GENERAL_REGISTERS 顺序) ──
const X64_GP = ["rax","rcx","rdx","rbx","rsp","rbp","rsi","rdi","r8","r9","r10","r11","r12","r13","r14","r15"];
const X64_FP = Array.from({ length: 16 }, (_, i) => `xmm${i}`);
const GP_CODE = new Map(X64_GP.map((n, i) => [n, i]));
const FP_CODE = new Map(X64_FP.map((n, i) => [n, i]));

// ── 小型算式求值器: 只支持整数 + - * / ( ) 一元负号, 拒绝其余一切 ──
function evalNum(text, where) {
  const toks = text.match(/\d+|[+\-*/()]/g);
  if (!toks || toks.join("") !== text.replace(/\s+/g, "")) {
    die(`算式残留非纯算术 token: ${JSON.stringify(text)} @ ${where}`);
  }
  let p = 0;
  const peek = () => toks[p];
  const eat = (t) => { if (toks[p] === t) { p++; return true; } return false; };
  function primary() {
    if (eat("(")) { const v = expr(); if (!eat(")")) die(`括号不闭合 @ ${where}`); return v; }
    if (/^\d+$/.test(peek() ?? "")) return parseInt(toks[p++], 10);
    die(`无法求值 token '${peek()}' @ ${where}`);
  }
  function unary() { if (eat("-")) return -unary(); return primary(); }
  function term() { let v = unary(); while (peek() === "*" || peek() === "/") { const op = toks[p++]; const r = unary(); v = op === "*" ? v * r : v / r; } return v; }
  function expr() { let v = term(); while (peek() === "+" || peek() === "-") { const op = toks[p++]; const r = term(); v = op === "+" ? v + r : v - r; } return v; }
  const v = expr();
  if (p !== toks.length) die(`算式尾部残留 @ ${where}`);
  if (!Number.isInteger(v)) die(`算式非整数结果 @ ${where}`);
  return v;
}

// 宏/限定名/标识符/Count() 替换至纯算术后求值; 未知 k* 标识符 → throw (对数守卫)
function expandEval(rhs, syms, where) {
  let t = rhs.replace(/\/\/[^\n]*/g, " ").replace(/\s+/g, " ").trim();
  t = t.replace(/TYPED_FRAME_PUSHED_VALUE_OFFSET\(\s*([^()]+?)\s*\)/g,
    (_, n) => `(${BASE.kFirstPushedFrameValueOffset} - (${n}) * ${BASE.kSystemPointerSize})`);
  t = t.replace(/\b\w+::/g, ""); // TypedFrameConstants:: 等限定前缀
  // RegList::Count() → 预登记的成员数
  t = t.replace(/\b(k\w+)\.Count\(\)/g, (_, name) => {
    if (!(name + "#count" in syms)) die(`未知 Count() 接收者 ${name} @ ${where}`);
    return `(${syms[name + "#count"]})`;
  });
  for (let i = 0; i < 16; i++) {
    t = t.replace(/RoundUp<\s*16\s*>\(([^()]+)\)/g, (_, e) => `(((${e})+15)-(((${e})+15)%16))`);
    const unknown = t.match(/\bk[A-Za-z0-9_]*\b/);
    if (unknown) {
      const name = unknown[0];
      if (!(name in syms) && !(name in BASE)) die(`未知常量标识符 ${name} @ ${where}`);
      t = t.replace(new RegExp(`\\b${name}\\b`, "g"), `(${name in syms ? syms[name] : BASE[name]})`);
    } else break;
  }
  return evalNum(t, where);
}

// kCamelName → SCREAMING_SNAKE
function toScreaming(k) {
  return k.replace(/^k/, "")
    .replace(/([^A-Z])([A-Z])/g, "$1_$2")
    .replace(/([A-Z])([A-Z][a-z])/g, "$1_$2")
    .toUpperCase();
}

// ── 取类体: 类名定位 + 花括号配平 (容忍成员初始化列表内嵌 {}) ──
function classBody(src, className) {
  const m = src.match(new RegExp(`class\\s+${className}\\s*:\\s*public\\s+\\w+\\s*\\{`));
  if (!m) die(`未找到 class ${className}`);
  let depth = 0, i = m.index + m[0].length - 1;
  for (; i < src.length; i++) {
    if (src[i] === "{") depth++;
    else if (src[i] === "}") { depth--; if (depth === 0) break; }
  }
  return src.slice(m.index + m[0].length, i);
}

// ── 预处理指令位置表: 每个字符位置的分支归属 (V8_TARGET_OS_WIN → win) ──
function branchMap(body) {
  const marks = []; // {at, tag}
  const stack = [];
  const re = /^\s*#\s*(ifdef|else|endif)\b.*$/gm;
  let m;
  while ((m = re.exec(body)) !== null) {
    if (m[1] === "ifdef") stack.push(/V8_TARGET_OS_WIN/.test(m[0]) ? "win" : "other");
    else if (m[1] === "else" && stack.length) stack[stack.length - 1] = stack[stack.length - 1] === "win" ? "default" : "other";
    else if (m[1] === "endif") stack.pop();
    marks.push({ at: re.lastIndex, tag: stack[stack.length - 1] ?? "default" });
  }
  return (pos) => {
    let tag = "default";
    for (const mk of marks) { if (mk.at <= pos) tag = mk.tag; else break; }
    return tag === "other" ? "default" : tag;
  };
}

// ── 全 body 逐条扫描 (声明可跨行), 按匹配位置归属分支 ──
function scanEntries(body) {
  const at = branchMap(body);
  const found = [];
  let m;
  const reScalar = /static\s+constexpr\s+int\s+(k\w+)\s*=\s*([^;]+);/g;
  while ((m = reScalar.exec(body)) !== null) {
    found.push({ kind: "scalar", name: m[1], rhs: m[2], at: m.index, branch: at(m.index) });
  }
  const reArray = /static\s+constexpr\s+int\s+(k\w+)\s*\[\s*\]\s*=\s*\{([^}]*)\}\s*;/g;
  while ((m = reArray.exec(body)) !== null) {
    found.push({ kind: "array", name: m[1], items: m[2].split(",").map((s) => s.trim()).filter(Boolean), at: m.index, branch: at(m.index) });
  }
  const reList = /static\s+constexpr\s+(RegList|DoubleRegList)\s+(k\w+)\s*=\s*\{([^}]*)\}\s*;/g;
  while ((m = reList.exec(body)) !== null) {
    found.push({ kind: "reglist", list: m[1], name: m[2], regs: m[3].split(",").map((s) => s.trim()).filter(Boolean), at: m.index, branch: at(m.index) });
  }
  const reFn = /static\s+int\s+(\w+)\s*\(([^)]*)\)\s*\{/g;
  while ((m = reFn.exec(body)) !== null) {
    found.push({ kind: "fn", name: m[1], params: m[2].trim(), at: m.index, branch: at(m.index) });
  }
  found.sort((a, b) => a.at - b.at);
  return found;
}

// 取匹配位置前紧邻的 // 注释 (≤3 行) 作 doc
function leadingDoc(body, idx) {
  const prefix = body.slice(0, idx).split("\n");
  prefix.pop(); // 匹配所在首行残余
  const docs = [];
  for (let i = prefix.length - 1; i >= 0 && docs.length < 3; i--) {
    const t = prefix[i].trim();
    if (t.startsWith("//")) docs.unshift(t);
    else if (t === "") continue;
    else break;
  }
  return docs;
}

// ── ① 解析四类 ──
const CLASSES = ["EntryFrameConstants", "WasmLiftoffSetupFrameConstants", "WasmLiftoffFrameConstants", "WasmDebugBreakFrameConstants"];
const bodies = Object.fromEntries(CLASSES.map((c) => [c, classBody(hSrc, c)]));
const entries = Object.fromEntries(CLASSES.map((c) => [c, scanEntries(bodies[c])]));

// 对数守卫: 条目数 (分支合计)
const expectCounts = {
  EntryFrameConstants: { scalar: 9, array: 0, reglist: 0, fn: 0 }, // 默认 4 + WIN 5
  WasmLiftoffSetupFrameConstants: { scalar: 7, array: 1, reglist: 0, fn: 0 },
  WasmLiftoffFrameConstants: { scalar: 2, array: 0, reglist: 0, fn: 0 },
  WasmDebugBreakFrameConstants: { scalar: 4, array: 0, reglist: 2, fn: 2 },
};
for (const c of CLASSES) {
  const got = { scalar: 0, array: 0, reglist: 0, fn: 0 };
  for (const e of entries[c]) got[e.kind]++;
  for (const k of Object.keys(expectCounts[c])) {
    if (got[k] !== expectCounts[c][k]) die(`${c} ${k} 条数 ${got[k]} ≠ ${expectCounts[c][k]}`);
  }
}

// ── ② 求值 (按源顺序, 分支符号表分离; registry 键 = 名称@分支) ──
const registry = new Map();
const classMods = {
  EntryFrameConstants: "entry_frame_constants",
  WasmLiftoffSetupFrameConstants: "wasm_liftoff_setup_frame_constants",
  WasmLiftoffFrameConstants: "wasm_liftoff_frame_constants",
  WasmDebugBreakFrameConstants: "wasm_debug_break_frame_constants",
};

for (const c of CLASSES) {
  const symsDefault = {};
  const symsWin = {};
  for (const e of entries[c]) {
    // WIN 分支回退类公共段 (#ifdef 之前定义的常量两分支共用); 本条目所属分支表最后覆盖
    const syms = e.branch === "win" ? symsWin : symsDefault;
    const lookup = e.branch === "win"
      ? { ...BASE, ...symsDefault, ...symsWin }
      : { ...BASE, ...symsWin, ...symsDefault };
    const where = `${c}.${e.name}[${e.branch}]`;
    const key = `${e.name}@${e.branch}`;
    const doc = leadingDoc(bodies[c], e.at);
    if (e.kind === "scalar") {
      const v = expandEval(e.rhs, lookup, where);
      syms[e.name] = v;
      registry.set(key, { cls: c, branch: e.branch, kind: "scalar", name: e.name, rust: toScreaming(e.name), value: v, doc });
    } else if (e.kind === "array") {
      const items = e.items.map((s, i) => expandEval(s, lookup, `${where}[${i}]`));
      registry.set(key, { cls: c, branch: e.branch, kind: "array", name: e.name, rust: toScreaming(e.name), items, doc });
    } else if (e.kind === "reglist") {
      const tbl = e.list === "RegList" ? GP_CODE : FP_CODE;
      const codes = e.regs.map((r) => { if (!tbl.has(r)) die(`${where}: 未知寄存器 ${r}`); return tbl.get(r); });
      const bits = codes.reduce((a, b) => a | (1 << b), 0) >>> 0;
      syms[e.name] = bits; // 位标志后续引用
      syms[e.name + "#count"] = codes.length; // Count() 后续引用
      registry.set(key, { cls: c, branch: e.branch, kind: "reglist", name: e.name, rust: toScreaming(e.name), codes, bits, names: e.regs, doc });
    } else {
      registry.set(key, { cls: c, branch: e.branch, kind: "fn", name: e.name, rust: toScreaming(e.name), params: e.params, doc });
    }
  }
}

// 派生量对数: kNumPushedGpRegisters/kNumPushedFpRegisters = Count()
const gp = registry.get("kPushedGpRegs@default"), fp = registry.get("kPushedFpRegs@default");
if (gp.codes.length !== 10) die(`x64 kPushedGpRegs 10 ≠ ${gp.codes.length}`);
if (fp.codes.length !== 8) die(`x64 kPushedFpRegs 8 ≠ ${fp.codes.length}`);
if (registry.get("kNumPushedGpRegisters@default").value !== 10) die("kNumPushedGpRegisters ≠ 10");
if (registry.get("kNumPushedFpRegisters@default").value !== 8) die("kNumPushedFpRegisters ≠ 8");
// 数值锚点 (clang -E 口径的手工展开核对)
const anchors = [
  ["kXMMRegisterSize", "default", 16],
  ["kNextExitFrameFPOffset", "default", -64],   // -3*8 + -5*8
  ["kNextFastCallFrameFPOffset", "default", -72],
  ["kNextFastCallFramePCOffset", "default", -80],
  ["kCalleeSaveXMMRegisters", "win", 10],
  ["kXMMRegistersBlockSize", "win", 160],
  ["kNextExitFrameFPOffset", "win", -240],      // -24 -56 -160
  ["kArgcOffset", "win", 48],
  ["kArgvOffset", "win", 56],
  ["kCallingPCOffset", "default", -16],
  ["kInstanceSpillOffset", "default", -24],
  ["kLastPushedGpRegisterOffset", "default", -88],  // -8 - 10*8
  ["kLastPushedFpRegisterOffset", "default", -216], // -88 - 8*16
];
for (const [name, br, want] of anchors) {
  const r = registry.get(`${name}@${br}`);
  if (!r || r.kind !== "scalar") die(`锚点缺失 ${name}@${br}`);
  if (r.value !== want) die(`锚点值错 ${name}@${br}: ${r.value} ≠ ${want}`);
}
if ((gp.bits >>> 0) !== 0x93cf) die(`kPushedGpRegs bits 0x93cf ≠ 0x${gp.bits.toString(16)}`);

// ── ③ .cc 签名骨架: 逐条摘录 (原文, 零改写; 本文件函数体均无内嵌花括号) ──
const ccFns = [...ccSrc.matchAll(/(?:Register|int|intptr_t)\s+[\w:]+\([^)]*\)\s*\{[^}]*\}/g)].map((m) => m[0]);
if (ccFns.length !== 6) die(`.cc 签名 6 ≠ ${ccFns.length}`);

// ── ④ 生成 Rust ──
let rust = banner(GEN, `${H_REL} + ${CC_REL}`, "x64 帧常量规则代码 (常量表/位标志/宏展开/签名骨架)");
rust += `// 目标: x64 (V8_TARGET_ARCH_X64); 公共基: kSystemPointerSize=8, kSimd128Size=16, kCPSlotSize=0 (无常量池槽)\n`;
rust += `// 宏展开: TYPED_FRAME_PUSHED_VALUE_OFFSET(x) = TypedFrameConstants::kFirstPushedFrameValueOffset - x*kSystemPointerSize = -16 - 8x\n`;
rust += `// 分支: EntryFrameConstants 按 V8_TARGET_OS_WIN 双分支展开; 默认=非 WIN (主目标), WIN 变体入 windows 子模块\n`;
rust += `// 对齐注记: gn arg v8_x64_16byte_stack_alignment 默认 false → V8_X64_16BYTE_STACK_ALIGNMENT_BOOL=false\n\n`;

rust += `/// 公共帧常量基值 (展开自 v8/src/execution/frame-constants.h TypedFrameConstants, x64 口径)\n`;
rust += `pub const SYSTEM_POINTER_SIZE: i32 = ${BASE.kSystemPointerSize};\n`;
rust += `pub const SIMD128_SIZE: i32 = ${BASE.kSimd128Size};\n`;
rust += `/// TypedFrameConstants::kFirstPushedFrameValueOffset (kFixedFrameSizeFromFp=8)\n`;
rust += `pub const TYPED_FIRST_PUSHED_VALUE_OFFSET: i32 = ${BASE.kFirstPushedFrameValueOffset};\n\n`;

for (const c of CLASSES) {
  const mod = classMods[c];
  const imports = c === "WasmDebugBreakFrameConstants" ? "  use super::{SIMD128_SIZE, SYSTEM_POINTER_SIZE};\n\n" : "";
  rust += `/// 原版: class ${c} (frame-constants-x64.h)\npub mod ${mod} {\n${imports}`;
  for (const r of registry.values()) {
    if (r.cls !== c || r.branch === "win") continue; // WIN 变体单独收尾
    for (const d of r.doc) rust += `  ${d.replace(/^\/\//, "///")}\n`;
    if (r.kind === "scalar") rust += `  pub const ${r.rust}: i32 = ${r.value};\n`;
    else if (r.kind === "array") rust += `  pub const ${r.rust}: [i32; ${r.items.length}] = [${r.items.join(", ")}];\n`;
    else if (r.kind === "reglist") {
      rust += `  /// 位标志: bit n = 寄存器编码 n (展开自 ${r.name})\n`;
      rust += `  pub const ${r.rust}_BITS: u32 = 0x${r.bits.toString(16)};\n`;
    } else if (r.name === "GetPushedGpRegisterOffset") {
      rust += `  #[inline]\n  pub const fn get_pushed_gp_register_offset(reg_code: i32) -> i32 {\n    let lower_regs = PUSHED_GP_REGS_BITS & ((1u32 << reg_code) - 1);\n    LAST_PUSHED_GP_REGISTER_OFFSET + (lower_regs.count_ones() as i32) * SYSTEM_POINTER_SIZE\n  }\n`;
    } else if (r.name === "GetPushedFpRegisterOffset") {
      rust += `  #[inline]\n  pub const fn get_pushed_fp_register_offset(reg_code: i32) -> i32 {\n    let lower_regs = PUSHED_FP_REGS_BITS & ((1u32 << reg_code) - 1);\n    LAST_PUSHED_FP_REGISTER_OFFSET + (lower_regs.count_ones() as i32) * SIMD128_SIZE\n  }\n`;
    } else {
      rust += `  // static int ${r.name}(${r.params})\n`;
    }
  }
  rust += `}\n\n`;
}

// WIN 分支变体: 5 条分支内条目 + 类公共 3 条按 WIN 符号表重算 = clang -E WIN 口径 8 条
const winEntries = [...registry.values()].filter((r) => r.branch === "win");
if (winEntries.length !== 5) die(`WIN 分支 5 条 ≠ ${winEntries.length}`);
const winLookup = { ...BASE, ...{} };
{
  const cls = "EntryFrameConstants";
  const symsDefault = {}, symsWin = {};
  for (const e of entries[cls]) {
    if (e.kind !== "scalar") continue;
    (e.branch === "win" ? symsWin : symsDefault)[e.name] =
      expandEval(e.rhs, { ...BASE, ...symsDefault, ...symsWin }, `${cls}.${e.name}[re-${e.branch}]`);
  }
  winLookup.symsWin = symsWin; winLookup.symsDefault = symsDefault;
}
const winCommon = [];
const winNames = new Set(winEntries.map((r) => r.name));
for (const r of registry.values()) {
  if (r.cls !== "EntryFrameConstants" || r.branch !== "default" || r.kind !== "scalar") continue;
  if (winNames.has(r.name)) continue; // WIN 分支已有专属条目, 不重算
  const v = expandEval(entries["EntryFrameConstants"].find((e) => e.name === r.name).rhs,
    { ...BASE, ...winLookup.symsDefault, ...winLookup.symsWin }, `EntryFrameConstants.${r.name}[win-re]`);
  winCommon.push({ ...r, value: v });
}
if (winCommon.length !== 3) die(`WIN 类公共重算 3 条 ≠ ${winCommon.length}`);
rust += `/// EntryFrameConstants 的 V8_TARGET_OS_WIN 完整展开 (分支内 5 条 + 类公共重算 3 条)\npub mod entry_frame_constants_windows {\n`;
for (const r of winCommon) {
  rust += `  /// 原版 ${r.name} (类公共条目, 按 WIN 符号表重算)\n`;
  rust += `  pub const ${r.rust}: i32 = ${r.value};\n`;
}
for (const r of winEntries) {
  rust += `  /// 原版 ${r.name} (WIN 分支内条目)\n`;
  rust += `  pub const ${r.rust}: i32 = ${r.value};\n`;
}
rust += `}\n\n`;

// .cc 签名骨架逐条摘录
rust += `// ── 签名骨架 (原版 frame-constants-x64.cc 逐条摘录, 零改写) ──\n`;
for (const f of ccFns) rust += `// ${f.replace(/\n/g, "\n// ")}\n`;
rust += `\n`;

// 提取不变式断言 (对数基线)
rust += `// —— 提取不变式断言 ——\nconst _: () = {\n`;
rust += `  assert!(SYSTEM_POINTER_SIZE == 8);\n`;
rust += `  assert!(entry_frame_constants::NEXT_EXIT_FRAME_FP_OFFSET == -64);\n`;
rust += `  assert!(entry_frame_constants_windows::NEXT_EXIT_FRAME_FP_OFFSET == -240);\n`;
rust += `  assert!(entry_frame_constants_windows::XMM_REGISTERS_BLOCK_SIZE == 160);\n`;
rust += `  assert!(entry_frame_constants_windows::NEXT_FAST_CALL_FRAME_FP_OFFSET == -248);\n`;
rust += `  assert!(entry_frame_constants_windows::NEXT_FAST_CALL_FRAME_PC_OFFSET == -256);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::PUSHED_GP_REGS_BITS == 0x93cf);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::NUM_PUSHED_GP_REGISTERS == 10);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::NUM_PUSHED_FP_REGISTERS == 8);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::LAST_PUSHED_GP_REGISTER_OFFSET == -88);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::LAST_PUSHED_FP_REGISTER_OFFSET == -216);\n`;
rust += `};\n`;

const p = writeSrc("v8-execution-frames", "frame_constants_x64.rs", rust);
console.log(`OK frame-constants-x64.js: 4 类 / ${registry.size} 条目 / .cc ${ccFns.length} 签名 → ${p}`);
