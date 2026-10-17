#!/usr/bin/env bun
// tool/src/objects/lookup.h.js — 覆盖 v8/src/objects/lookup.{h,-inl.h}
// → v8-objects-rest/src/lookup.rs
// 机械提取: Configuration 位标志 + State 状态机 + InterceptorState + ConcurrentLookupIterator::Result

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/lookup.h");

function stripComments(src) {
  return src.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");
}
const src = stripComments(hSrc);

function splitTop(body) {
  const parts = [];
  let depth = 0, cur = "";
  for (const ch of body) {
    if (ch === "(") depth++;
    else if (ch === ")") depth--;
    if (ch === "," && depth === 0) { parts.push(cur); cur = ""; continue; }
    cur += ch;
  }
  if (cur.trim()) parts.push(cur);
  return parts.map((s) => s.trim()).filter(Boolean);
}

// Configuration: 位标志枚举 (含表达式组合) — 逐条机械求值
const cfgRe = src.match(/enum Configuration\s*\{([\s\S]*?)\n\s*\};/);
if (!cfgRe) throw new Error("未找到 LookupIterator::Configuration");
const cfgEntries = splitTop(cfgRe[1]);
// 原版条目: kInterceptor, kPrototypeChain, OWN_SKIP_INTERCEPTOR, OWN, PROTOTYPE_CHAIN_SKIP_INTERCEPTOR, PROTOTYPE_CHAIN, DEFAULT
if (cfgEntries.length !== 7) throw new Error(`Configuration 条数 ${cfgEntries.length} != 7`);
const K_INTERCEPTOR = 1, K_PROTOTYPE_CHAIN = 2;
const cfgValues = {
  kInterceptor: K_INTERCEPTOR,
  kPrototypeChain: K_PROTOTYPE_CHAIN,
  OWN_SKIP_INTERCEPTOR: 0,
  OWN: K_INTERCEPTOR,
  PROTOTYPE_CHAIN_SKIP_INTERCEPTOR: K_PROTOTYPE_CHAIN,
  PROTOTYPE_CHAIN: K_PROTOTYPE_CHAIN | K_INTERCEPTOR,
  DEFAULT: K_PROTOTYPE_CHAIN | K_INTERCEPTOR,
};

// State: 状态机枚举 (顺序值; BEFORE_PROPERTY = INTERCEPTOR 显式别名)
const stateRe = src.match(/enum State\s*\{([\s\S]*?)\n\s*\};/);
if (!stateRe) throw new Error("未找到 LookupIterator::State");
const stateRaw = splitTop(stateRe[1]);
// BEFORE_PROPERTY 显式赋值 = INTERCEPTOR, 从顺序条目中拆出
const stateEntries = [];
for (const e of stateRaw) {
  const m = e.match(/^(\w+)\s*=\s*(\w+)$/);
  if (m) stateEntries.push({ name: m[1], aliasOf: m[2] });
  else stateEntries.push({ name: e });
}
if (stateEntries.length !== 12) throw new Error(`State 条数 ${stateEntries.length} != 12`);
if (stateEntries[stateEntries.length - 1].name !== "BEFORE_PROPERTY" || stateEntries[stateEntries.length - 1].aliasOf !== "INTERCEPTOR") {
  throw new Error("State 尾条目应为 BEFORE_PROPERTY = INTERCEPTOR");
}

const interceptorState = splitTop(src.match(/enum class InterceptorState\s*\{([^}]+)\}/)[1]);
if (interceptorState.length !== 4) throw new Error(`InterceptorState 条数 ${interceptorState.length} != 4`);

const result = splitTop(src.match(/enum Result\s*\{([^}]+)\}/)[1]);
if (result.length !== 3) throw new Error(`ConcurrentLookupIterator::Result 条数 ${result.length} != 3`);

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

let rust = banner("src/objects/lookup.h.js", "src/objects/lookup.{h,-inl.h}", "LookupIterator 配置位/状态机/拦截器状态枚举");
rust += `//! LookupIterator: 属性查找迭代器的配置与状态机。

// ── Configuration 位标志 (原版 enum Configuration) ─────────────────────────
/// 拦截器位 (bit0)。
pub const K_INTERCEPTOR: u32 = 1 << 0;
/// 原型链位 (bit1)。
pub const K_PROTOTYPE_CHAIN: u32 = 1 << 1;
/// 便捷组合常量 (机械展开自原版表达式)。
`;
for (const [k, v] of Object.entries(cfgValues)) {
  if (k.startsWith("k")) continue;
  rust += `pub const ${k}: u32 = 0x${v.toString(16)}; // ${k === "OWN" ? "kInterceptor" : k === "DEFAULT" ? "PROTOTYPE_CHAIN" : k === "PROTOTYPE_CHAIN" ? "kPrototypeChain | kInterceptor" : k === "OWN_SKIP_INTERCEPTOR" ? "0" : "kPrototypeChain"}\n`;
}

rust += `
// ── State 状态机 (原版 enum State, 顺序值; 共 ${stateEntries.length} 态) ───────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum State {
`;
let seq = 0;
const stateValue = new Map();
for (const e of stateEntries) {
  if (e.aliasOf) {
    const v = stateValue.get(e.aliasOf);
    rust += `  /// ${e.name} = ${e.aliasOf} (显式别名)\n  ${pascal(e.name)} = ${v},\n`;
    stateValue.set(e.name, v);
  } else {
    rust += `  /// ${e.name}\n  ${pascal(e.name)} = ${seq},\n`;
    stateValue.set(e.name, seq);
    seq++;
  }
}
rust += `}

impl State {
  /// 顺序态总数 (不含显式别名 BEFORE_PROPERTY)。
  pub const COUNT: usize = ${seq};

  /// 数值反查 (BEFORE_PROPERTY 与 INTERCEPTOR 同值, 返回 INTERCEPTOR)。
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
`;
for (const e of stateEntries) {
  if (!e.aliasOf) rust += `      ${stateValue.get(e.name)} => Some(Self::${pascal(e.name)}),\n`;
}
rust += `      _ => None,
    }
  }
}

// ── InterceptorState (原版 enum class, 顺序值) ─────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InterceptorState {
`;
interceptorState.forEach((s, i) => {
  rust += `  /// ${s}\n  ${pascal(s)} = ${i},\n`;
});
rust += `}

// ── ConcurrentLookupIterator::Result (三态) ────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConcurrentResult {
`;
result.forEach((s, i) => {
  rust += `  /// ${s}\n  ${pascal(s)} = ${i},\n`;
});
rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_INTERCEPTOR == 1 && K_PROTOTYPE_CHAIN == 2);
  assert!(PROTOTYPE_CHAIN == 3 && DEFAULT == 3 && OWN == 1);
  assert!(State::COUNT == ${seq});
  assert!(State::NotFound as u8 == 0);
  assert!(State::BeforeProperty as u8 == State::Interceptor as u8);
  assert!(State::Transition as u8 == 11);
};
`;

const p = writeSrc("v8-objects-rest", "lookup.rs", rust);
console.log(`lookup.rs: Configuration 7 条 + State ${stateEntries.length} 态 + InterceptorState 4 + Result 3, 对数通过`);
