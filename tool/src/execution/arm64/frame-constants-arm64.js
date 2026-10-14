#!/usr/bin/env bun
// tool/src/execution/arm64/frame-constants-arm64.js
// 机械提取 v8/src/execution/arm64/frame-constants-arm64.{h,cc} — arm64 帧常量规则代码
// 产出: v8-execution-frames/src/frame_constants_arm64.rs
// 规则: 常量表/位标志/宏机械展开 (TYPED_FRAME_PUSHED_VALUE_OFFSET/RoundUp<16>/Count()) /
//       static_assert 转译 / .cc 签名骨架逐条摘录
// 宏展开基准 (展开自 v8/src/execution/frame-constants.h, arm64 无常量池):
//   kCPSlotSize=0, kFrameTypeSize=8 → TypedFrameConstants::kFixedFrameSizeFromFp=8
//   TYPED_FRAME_PUSHED_VALUE_OFFSET(x) = -16 - 8x
// 注: 本文件无 V8_TARGET_OS_WIN 分支; 非 x64/arm64 目标架构不在本脚本范围 (登记后置)
// 复跑幂等: 纯文本扫描 + 确定性求值, 无随机无时钟。
import { banner, readV8, writeSrc } from "../../../lib.js";

const GEN = "src/execution/arm64/frame-constants-arm64.js";
const H_REL = "src/execution/arm64/frame-constants-arm64.h";
const CC_REL = "src/execution/arm64/frame-constants-arm64.cc";
const hSrc = readV8(H_REL);
const ccSrc = readV8(CC_REL);
const die = (m) => { throw new Error(`[frame-constants-arm64.js] ${m}`); };

// ── 公共基 (机械事实, 展开自 frame-constants.h / common/globals.h) ──
const BASE = {
  kSystemPointerSize: 8,
  kSimd128Size: 16,
  kQRegSize: 16, // arm64 128-bit Q 寄存器
  kCPSlotSize: 0, // V8_EMBEDDED_CONSTANT_POOL_BOOL=false (arm64)
  kFrameTypeSize: 8,
  kFixedFrameSizeFromFp: 8, // TypedFrameConstants 口径
  kFirstPushedFrameValueOffset: -16,
};

// ── arm64 寄存器编码 (展开自 src/codegen/arm64/register-arm64.h) ──
// GENERAL_REGISTERS: x0..x31 → 编码 = 序号; DOUBLE_REGISTERS: d0..d31 → 编码 = 序号
const ARM64_GP = Array.from({ length: 32 }, (_, i) => `x${i}`);
const ARM64_FP = Array.from({ length: 32 }, (_, i) => `d${i}`);
const GP_CODE = new Map(ARM64_GP.map((n, i) => [n, i]));
const FP_CODE = new Map(ARM64_FP.map((n, i) => [n, i]));

// ── 小型算式求值器: 只支持整数 + - * / % ( ) 一元负号, 拒绝其余一切 ──
function evalNum(text, where) {
  const toks = text.match(/\d+|[+\-*/()%]/g);
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
  function term() {
    let v = unary();
    while (peek() === "*" || peek() === "/" || peek() === "%") {
      const op = toks[p++]; const r = unary();
      v = op === "*" ? v * r : op === "/" ? v / r : v % r;
    }
    return v;
  }
  function expr() { let v = term(); while (peek() === "+" || peek() === "-") { const op = toks[p++]; const r = term(); v = op === "+" ? v + r : v - r; } return v; }
  const v = expr();
  if (p !== toks.length) die(`算式尾部残留 @ ${where}`);
  if (!Number.isInteger(v)) die(`算式非整数结果 @ ${where}`);
  return v;
}

// 顶层 == 拆分后两侧求值比较 (static_assert 条件口径)
function evalCond(text, syms, where) {
  const parts = text.split("==");
  if (parts.length !== 2) die(`static_assert 条件非单一 == : ${text} @ ${where}`);
  return evalNum(parts[0], where) === evalNum(parts[1], where);
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

// ── 预处理指令位置表 (本文件应无分支, 存在则归属登记) ──
function branchMap(body) {
  const marks = [];
  const stack = [];
  const re = /^\s*#\s*(ifdef|else|endif)\b.*$/gm;
  let m;
  while ((m = re.exec(body)) !== null) {
    if (m[1] === "ifdef") stack.push("other");
    else if (m[1] === "else" && stack.length) stack[stack.length - 1] = "other";
    else if (m[1] === "endif") stack.pop();
    marks.push({ at: re.lastIndex, tag: stack[stack.length - 1] ?? "default" });
  }
  return (pos) => {
    let tag = "default";
    for (const mk of marks) { if (mk.at <= pos) tag = mk.tag; else break; }
    return tag === "other" ? "default" : tag;
  };
}

// ── 全 body 逐条扫描 (声明可跨行) ──
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
  const reAssert = /static_assert\(([\s\S]+?)\);/g;
  while ((m = reAssert.exec(body)) !== null) {
    // 剥离消息串: 顶层逗号后带引号的第二参
    const inner = m[1];
    const msgIdx = inner.search(/,\s*"/);
    found.push({ kind: "assert", cond: msgIdx >= 0 ? inner.slice(0, msgIdx) : inner, at: m.index, branch: at(m.index) });
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

// 对数守卫: 条目数
const expectCounts = {
  EntryFrameConstants: { scalar: 10, array: 0, reglist: 0, fn: 0, assert: 0 },
  WasmLiftoffSetupFrameConstants: { scalar: 7, array: 1, reglist: 0, fn: 0, assert: 1 },
  WasmLiftoffFrameConstants: { scalar: 2, array: 0, reglist: 0, fn: 0, assert: 0 },
  WasmDebugBreakFrameConstants: { scalar: 4, array: 0, reglist: 2, fn: 2, assert: 1 },
};
for (const c of CLASSES) {
  const got = { scalar: 0, array: 0, reglist: 0, fn: 0, assert: 0 };
  for (const e of entries[c]) got[e.kind]++;
  for (const k of Object.keys(expectCounts[c])) {
    if (got[k] !== expectCounts[c][k]) die(`${c} ${k} 条数 ${got[k]} ≠ ${expectCounts[c][k]}`);
  }
}

// ── ② 求值 (按源顺序) ──
const registry = new Map(); // key = name@branch
const namePaths = new Map(); // kName → rust 路径 (assert 转译用)
const classMods = {
  EntryFrameConstants: "entry_frame_constants",
  WasmLiftoffSetupFrameConstants: "wasm_liftoff_setup_frame_constants",
  WasmLiftoffFrameConstants: "wasm_liftoff_frame_constants",
  WasmDebugBreakFrameConstants: "wasm_debug_break_frame_constants",
};
for (const c of CLASSES) {
  const syms = { ...BASE };
  for (const e of entries[c]) {
    const where = `${c}.${e.name ?? "assert"}[${e.branch}]`;
    const key = `${e.name ?? "assert"}@${e.branch}`;
    if (e.kind === "assert") continue; // 求值后置 (依赖类内常量齐备)
    const doc = leadingDoc(bodies[c], e.at);
    if (e.kind === "scalar") {
      const v = expandEval(e.rhs, syms, where);
      syms[e.name] = v;
      registry.set(key, { cls: c, branch: e.branch, kind: "scalar", name: e.name, rust: toScreaming(e.name), value: v, doc });
      namePaths.set(e.name, `${classMods[c]}::${toScreaming(e.name)}`);
    } else if (e.kind === "array") {
      const items = e.items.map((s, i) => expandEval(s, syms, `${where}[${i}]`));
      registry.set(key, { cls: c, branch: e.branch, kind: "array", name: e.name, rust: toScreaming(e.name), items, doc });
      namePaths.set(e.name, `${classMods[c]}::${toScreaming(e.name)}`);
    } else if (e.kind === "reglist") {
      const tbl = e.list === "RegList" ? GP_CODE : FP_CODE;
      const codes = e.regs.map((r) => { if (!tbl.has(r)) die(`${where}: 未知寄存器 ${r}`); return tbl.get(r); });
      const bits = codes.reduce((a, b) => a | (1 << b), 0) >>> 0;
      syms[e.name] = bits;
      syms[e.name + "#count"] = codes.length;
      registry.set(key, { cls: c, branch: e.branch, kind: "reglist", name: e.name, rust: toScreaming(e.name), codes, bits, names: e.regs, doc });
      namePaths.set(e.name, `${classMods[c]}::${toScreaming(e.name)}`);
    } else {
      registry.set(key, { cls: c, branch: e.branch, kind: "fn", name: e.name, rust: toScreaming(e.name), params: e.params, doc });
    }
  }
}

// static_assert 转译 (名称 → rust 路径, BASE 名 → 顶层常量)
const basePaths = { kSystemPointerSize: "SYSTEM_POINTER_SIZE", kSimd128Size: "SIMD128_SIZE", kQRegSize: "Q_REG_SIZE" };
const asserts = [];
for (const c of CLASSES) {
  for (const e of entries[c]) {
    if (e.kind !== "assert") continue;
    let ok, rustCond = e.cond.replace(/\s+/g, " ").trim();
    // 条件求值: 名称代入数值
    let t = e.cond.replace(/\b\w+::/g, "");
    for (let i = 0; i < 16; i++) {
      const mm = t.match(/\bk[A-Za-z0-9_]*\b/);
      if (!mm) break;
      const name = mm[0];
      const val = registry.get(`${name}@default`)?.value ?? BASE[name];
      if (val === undefined) die(`static_assert 未知标识符 ${name} @ ${c}`);
      t = t.replace(new RegExp(`\\b${name}\\b`, "g"), `(${val})`);
    }
    ok = evalCond(t, {}, `${c}.static_assert`);
    if (!ok) die(`static_assert 数值不成立: ${e.cond.trim()} @ ${c}`);
    // rust 转译文本
    rustCond = rustCond.replace(/\b(k[A-Za-z0-9_]*)(#count)?\b/g, (full) => {
      const bare = full.replace("#count", "");
      return namePaths.get(bare) ?? basePaths[bare] ?? die(`assert 转译缺路径 ${bare}`);
    });
    asserts.push({ cls: c, src: e.cond.replace(/\s+/g, " ").trim(), rustCond });
  }
}
if (asserts.length !== 2) die(`static_assert 2 ≠ ${asserts.length}`);

// 派生量对数: kNumPushedGpRegisters/kNumPushedFpRegisters = Count()
const gp = registry.get("kPushedGpRegs@default"), fp = registry.get("kPushedFpRegs@default");
if (gp.codes.length !== 24) die(`arm64 kPushedGpRegs 24 ≠ ${gp.codes.length}`);
if (fp.codes.length !== 27) die(`arm64 kPushedFpRegs 27 ≠ ${fp.codes.length}`);
if (registry.get("kNumPushedGpRegisters@default").value !== 24) die("kNumPushedGpRegisters ≠ 24");
if (registry.get("kNumPushedFpRegisters@default").value !== 27) die("kNumPushedFpRegisters ≠ 27");
// 数值锚点 (clang -E 口径的手工展开核对)
const anchors = [
  ["kNextExitFrameFPOffset", -24],
  ["kNextFastCallFrameFPOffset", -40],
  ["kNextFastCallFramePCOffset", -48],
  ["kFixedFrameSize", 48], // 6 * 8
  ["kCalleeSavedRegisterBytesPushedBeforeFpLrPair", 144], // 18 * 8
  ["kCalleeSavedRegisterBytesPushedAfterFpLrPair", 0],
  ["kOffsetToCalleeSavedRegisters", 0],
  ["kDirectCallerFPOffset", 0],
  ["kDirectCallerPCOffset", 8],
  ["kDirectCallerSPOffset", 160], // 8 + 1*8 + 144
  ["kInstanceSpillOffset", -16],
  ["kCallingPCOffset", -200], // TYPED(1 + 6 + 2*8) = -16 - 23*8
  ["kLastPushedGpRegisterOffset", -208], // -RoundUp16(8) - 8*24
  ["kLastPushedFpRegisterOffset", -640], // -208 - 16*27
];
for (const [name, want] of anchors) {
  const r = registry.get(`${name}@default`);
  if (!r || r.kind !== "scalar") die(`锚点缺失 ${name}`);
  if (r.value !== want) die(`锚点值错 ${name}: ${r.value} ≠ ${want}`);
}
if ((gp.bits >>> 0) !== 0xbf8ffff) die(`kPushedGpRegs bits 0xbf8ffff ≠ 0x${gp.bits.toString(16)}`);
if ((fp.bits >>> 0) !== 0xfff7fff) die(`kPushedFpRegs bits 0xfff7fff ≠ 0x${fp.bits.toString(16)}`);

// ── ③ .cc 签名骨架: 逐条摘录 (原文, 零改写; 本文件函数体均无内嵌花括号) ──
const ccFns = [...ccSrc.matchAll(/(?:Register|int|intptr_t)\s+[\w:]+\([^)]*\)\s*\{[^}]*\}/g)].map((m) => m[0]);
if (ccFns.length !== 6) die(`.cc 签名 6 ≠ ${ccFns.length}`);

// ── ④ 生成 Rust ──
let rust = banner(GEN, `${H_REL} + ${CC_REL}`, "arm64 帧常量规则代码 (常量表/位标志/宏展开/static_assert 转译/签名骨架)");
rust += `// 目标: arm64 (V8_TARGET_ARCH_ARM64); 公共基: kSystemPointerSize=8, kSimd128Size=16, kQRegSize=16, kCPSlotSize=0 (无常量池槽)\n`;
rust += `// 宏展开: TYPED_FRAME_PUSHED_VALUE_OFFSET(x) = TypedFrameConstants::kFirstPushedFrameValueOffset - x*kSystemPointerSize = -16 - 8x\n`;
rust += `// 注: 本文件无 OS/平台分支; kLastPushedGpRegisterOffset 含 RoundUp<16> 头部对齐展开 (MacroAssembler::EnterFrame 16 字节垫)\n\n`;

rust += `/// 公共帧常量基值 (展开自 v8/src/execution/frame-constants.h TypedFrameConstants, arm64 口径)\n`;
rust += `pub const SYSTEM_POINTER_SIZE: i32 = ${BASE.kSystemPointerSize};\n`;
rust += `pub const SIMD128_SIZE: i32 = ${BASE.kSimd128Size};\n`;
rust += `/// 128-bit Q 寄存器尺寸 (原版 kQRegSize)\n`;
rust += `pub const Q_REG_SIZE: i32 = ${BASE.kQRegSize};\n`;
rust += `/// TypedFrameConstants::kFirstPushedFrameValueOffset (kFixedFrameSizeFromFp=8)\n`;
rust += `pub const TYPED_FIRST_PUSHED_VALUE_OFFSET: i32 = ${BASE.kFirstPushedFrameValueOffset};\n\n`;

for (const c of CLASSES) {
  const mod = classMods[c];
  const imports = c === "WasmDebugBreakFrameConstants" ? "  use super::{SIMD128_SIZE, SYSTEM_POINTER_SIZE};\n\n" : "";
  rust += `/// 原版: class ${c} (frame-constants-arm64.h)\npub mod ${mod} {\n${imports}`;
  for (const r of registry.values()) {
    if (r.cls !== c) continue;
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

// .cc 签名骨架逐条摘录
rust += `// ── 签名骨架 (原版 frame-constants-arm64.cc 逐条摘录, 零改写) ──\n`;
for (const f of ccFns) rust += `// ${f.replace(/\n/g, "\n// ")}\n`;
rust += `\n`;

// 提取不变式断言 (对数基线) + static_assert 转译
rust += `// —— 提取不变式断言 (含原版 static_assert 转译) ——\nconst _: () = {\n`;
rust += `  assert!(SYSTEM_POINTER_SIZE == 8);\n`;
for (const a of asserts) rust += `  // 原版: static_assert(${a.src});\n  assert!(${a.rustCond});\n`;
rust += `  assert!(entry_frame_constants::NEXT_EXIT_FRAME_FP_OFFSET == -24);\n`;
rust += `  assert!(entry_frame_constants::DIRECT_CALLER_SP_OFFSET == 160);\n`;
rust += `  assert!(wasm_liftoff_setup_frame_constants::CALLING_PC_OFFSET == -200);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::PUSHED_GP_REGS_BITS == 0xbf8ffff);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::PUSHED_FP_REGS_BITS == 0xfff7fff);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::NUM_PUSHED_GP_REGISTERS == 24);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::NUM_PUSHED_FP_REGISTERS == 27);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::LAST_PUSHED_GP_REGISTER_OFFSET == -208);\n`;
rust += `  assert!(wasm_debug_break_frame_constants::LAST_PUSHED_FP_REGISTER_OFFSET == -640);\n`;
rust += `};\n`;

const p = writeSrc("v8-execution-frames", "frame_constants_arm64.rs", rust);
console.log(`OK frame-constants-arm64.js: 4 类 / ${registry.size} 条目 / static_assert ${asserts.length} / .cc ${ccFns.length} 签名 → ${p}`);
