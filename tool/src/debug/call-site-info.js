#!/usr/bin/env bun
// tool/src/debug/call-site-info.js — 覆盖 v8/src/objects/call-site-info.{-inl.h,cc,tq}
// 组→文件映射 (本脚本管辖): 席位表 3 文件 (.h 归既有 tool/src/objects/call-site-info.h.js)
// 产物: v8-debug/src/call_site_info.rs
// 机械提取: tq CallSiteInfoFlags 位域链 (7+1 DRUMBRAKE) + extern class 5 字段布局
//   + -inl.h BOOL_GETTER 访问器面 + .cc 方法签名骨架 (todo! 就地修订)

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/call-site-info.tq");
const inl = readV8("src/objects/call-site-info-inl.h");
const cc = readV8("src/objects/call-site-info.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[call-site-info.js] ${m}`); };

// ── 1. tq bitfield struct CallSiteInfoFlags (uint31) ──────────────────────
must(tq.includes("@cppScope('CallSiteInfo')"), "tq 缺 @cppScope");
const bf = tq.slice(tq.indexOf("bitfield struct CallSiteInfoFlags extends uint31 {"));
const bfFields = [...bf.matchAll(/^  (?:@if\(V8_ENABLE_DRUMBRAKE\) )?(\w+): (\w+): (\d+) bit;/gm)]
  .map((m) => [m[1], m[2], +m[3], m[0].includes("@if")]);
must(bfFields.length === 8, `tq flags 字段数 ${bfFields.length} != 8 (7 + DRUMBRAKE 1)`);
// 非 DRUMBRAKE 7 位布局: 与既有 .h 位域链 1:1
const plain = bfFields.filter((f) => !f[3]);
must(plain.length === 7, `非 DRUMBRAKE 位段数 ${plain.length} != 7`);
const expectPlain = [
  ["is_wasm", "bool", 1], ["is_strict", "bool", 1], ["is_constructor", "bool", 1],
  ["is_async", "bool", 1], ["is_builtin", "bool", 1],
  ["is_source_position_computed", "bool", 1], ["is_deferred_baseline_frame", "bool", 1],
];
let shift = 0;
const layout = [];
for (let i = 0; i < plain.length; i++) {
  must(plain[i][0] === expectPlain[i][0] && plain[i][1] === expectPlain[i][1] && plain[i][2] === expectPlain[i][2],
    `tq flags 第 ${i} 位应为 ${expectPlain[i]}, 实得 ${plain[i]}`);
  layout.push({ name: plain[i][0], shift, bits: plain[i][2] });
  shift += plain[i][2];
}
must(shift === 7, `非 DRUMBRAKE 位域总宽 ${shift} != 7`);
const drum = bfFields.find((f) => f[3]);
must(drum && drum[0] === "is_wasm_interpreted_frame" && drum[2] === 1, "DRUMBRAKE 变体应为 is_wasm_interpreted_frame 1 bit");

// ── 2. tq extern class CallSiteInfo 布局 (5 字段) ─────────────────────────
const cls = tq.slice(tq.indexOf("extern class CallSiteInfo extends Struct"));
const fields = [...cls.matchAll(/^  (\w+): ([^;]+);$/gm)].map((m) => [m[1], m[2].trim()]);
must(fields.length === 5, `tq CallSiteInfo 字段数 ${fields.length} != 5`);
const expectFields = [
  ["code_object", "TrustedPointer<Code|BytecodeArray>"],
  ["receiver_or_instance", "JSAny"],
  ["function", "JSFunction|Smi"],
  ["code_offset_or_source_position", "Smi"],
  ["flags", "SmiTagged<CallSiteInfoFlags>"],
];
for (let i = 0; i < 5; i++) {
  must(fields[i][0] === expectFields[i][0] && fields[i][1] === expectFields[i][1],
    `tq 字段 ${i} 应为 ${expectFields[i]}, 实得 ${fields[i]}`);
}

// ── 3. -inl.h 访问器面: 6 BOOL_GETTER + 构造器 + 直load访问器 ─────────────
const boolGetters = [...inl.matchAll(/BOOL_GETTER\(CallSiteInfo, flags, (\w+),/g)].map((m) => m[1]);
must(boolGetters.join(",") === "IsWasm,IsWasmInterpretedFrame,IsBuiltin,IsStrict,IsConstructor,IsAsync",
  `BOOL_GETTER 序列漂移: ${boolGetters.join(",")}`);
must(inl.includes("CallSiteInfo::CallSiteInfo(\n    const AllocationWitness& witness"), "-inl.h 缺构造器");
for (const s of ["code_object(IsolateForSandbox isolate) const", "receiver_or_instance() const",
  "function() const", "code_offset_or_source_position() const", "flags() const"]) {
  must(inl.includes(s), `-inl.h 缺访问器 ${s}`);
}

// ── 4. .cc 方法签名骨架 (源码序提取) ──────────────────────────────────────
const sigRe = /^(?:DirectHandle|Handle|MaybeHandle|MaybeDirectHandle|Maybe|Tagged|bool|void|int|uint32_t|std::optional)[^;]*\bCallSiteInfo::(\w+)\(/gm;
const ccMethods = [...cc.matchAll(sigRe)].map((m) => m[1]);
// 重载合并 (如 GetScript() / GetScript(isolate, info))
const unique = [...new Set(ccMethods)];
must(unique.length >= 25, `.cc CallSiteInfo 唯一方法数 ${unique.length} < 25`);
must(unique.includes("ConstructFromRawData") && unique.includes("ExpandDeferredFrames"), ".cc 缺关键方法 ConstructFromRawData/ExpandDeferredFrames");
must(unique.includes("GetSourcePosition") && unique.includes("ComputeLocation"), ".cc 缺关键方法 GetSourcePosition/ComputeLocation");
const overloadCount = ccMethods.length - unique.length;

const pascal = (s) => s.replace(/(^|_)(\w)/g, (_, a, b) => b.toUpperCase());
const K = (s) => "K_" + s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

let rust = banner("src/debug/call-site-info.js", "src/objects/call-site-info.{-inl.h,cc,tq}",
  "席位表差集: tq 位域/布局 + inl 访问器 + cc 签名骨架 (.h 位域数值归既有 call-site-info.h.js)");
rust += `//! CallSiteInfo: Error.stack 栈帧对象 (debug 域)。
//!
//! tq 差集: CallSiteInfoFlags 位域 + Struct 布局 5 槽; .cc 方法面为签名骨架。

// ── CallSiteInfoFlags (tq uint31; 非 DRUMBRAKE 7 位 + DRUMBRAKE 位 7) ──
`;
for (const f of layout) {
  const mask = ((1 << f.bits) - 1) << f.shift;
  rust += `pub const ${K(f.name)}_SHIFT: u32 = ${f.shift};\npub const ${K(f.name)}_MASK: u32 = 0x${mask.toString(16)};\n`;
}
rust += `/// DRUMBRAKE 构建追加位 (默认关闭, bit 7)。
pub const K_IS_WASM_INTERPRETED_FRAME_SHIFT_DRUMBRAKE: u32 = 7;
pub const K_IS_WASM_INTERPRETED_FRAME_MASK_DRUMBRAKE: u32 = 0x80;

// ── 布局槽位 (tq extern class CallSiteInfo extends Struct, 源码序) ──
pub const CALL_SITE_INFO_FIELDS: [(&str, &str); ${fields.length}] = [
`;
for (const f of fields) rust += `  ("${f[0]}", "${f[1]}"),\n`;
rust += `];

// ── .cc 方法签名骨架 (${unique.length} 个唯一方法, 源码序, 重载 ${overloadCount} 处; 语义就地修订) ──
`;
for (const m of unique) {
  rust += `/// 原版 CallSiteInfo::${m} (call-site-info.cc)。
pub fn ${m.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}() {\n  todo!("就地修订: call-site-info.cc ${m}")\n}\n\n`;
}
rust += `/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(K_IS_WASM_SHIFT == 0);
  assert!(K_IS_DEFERRED_BASELINE_FRAME_SHIFT == 6);
  assert!(K_IS_DEFERRED_BASELINE_FRAME_MASK == 0x40);
  assert!(CALL_SITE_INFO_FIELDS.len() == ${fields.length});
  assert!(str_eq(CALL_SITE_INFO_FIELDS[4].0, "flags"));
};
`;

const p = writeSrc("v8-debug", "call_site_info.rs", rust);
console.log(`call-site-info.js: ${p} — tq 位域 7+1、布局 ${fields.length} 字段、BOOL_GETTER ${boolGetters.length}、.cc 唯一方法 ${unique.length} (重载 ${overloadCount}), 断言 ${checks} 项全过`);
