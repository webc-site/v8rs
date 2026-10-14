#!/usr/bin/env bun
// tool/src/objects/lookup.cc.js — 覆盖 lookup 同名族 .cc 段:
//   v8/src/objects/lookup.cc + lookup-inl.h
//   → v8-objects-collections/src/lookup.rs
// 配置/状态枚举 (Configuration/State/InterceptorState/Result) 归
//   v8-objects-rest/lookup.rs (lookup.h.js);
// 本脚本登记 .cc 方法面 (LookupIterator / ConcurrentLookupIterator) 与
// -inl.h 访问器清单 + 关键状态迁移规则 + 骨架 todo!("就地修订")。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/lookup.cc");
const inl = readV8("src/objects/lookup-inl.h");

// ── .cc 方法面 + 数量 must 断言 ───────────────────────────────────────────
const lookupIt = [...new Set([...cc.matchAll(/LookupIterator::(\w+)\(/g)].map((m) => m[1]))].sort();
const conIt = [...new Set([...cc.matchAll(/ConcurrentLookupIterator::(\w+)\(/g)].map((m) => m[1]))].sort();
if (lookupIt.length !== 46) throw new Error(`LookupIterator 方法应 46, 实得 ${lookupIt.length}: ${lookupIt.join(",")}`);
if (conIt.length !== 4) throw new Error(`ConcurrentLookupIterator 方法应 4, 实得 ${conIt.length}: ${conIt.join(",")}`);
for (const n of ["Start", "Next", "NextInternal", "RestartInternal", "PrepareForDataProperty", "ReconfigureDataProperty", "ApplyTransitionToDataProperty", "Delete", "FetchValue", "WriteDataValue", "LookupInRegularHolder", "LookupInSpecialHolder", "NotFound"]) {
  if (!lookupIt.includes(n)) throw new Error(`LookupIterator 缺 ${n}`);
}
for (const n of ["TryGetOwnCowElement", "TryGetOwnConstantElement", "TryGetOwnChar", "TryGetPropertyCell"]) {
  if (!conIt.includes(n)) throw new Error(`ConcurrentLookupIterator 缺 ${n}`);
}

// ── -inl.h 访问器/内联定义清单 + 数量断言 ────────────────────────────────
const inlDefs = [...new Set([...inl.matchAll(/(\w+)::(\w+)\(/g)].map((m) => `${m[1]}::${m[2]}`))]
  .filter((s) => s.startsWith("LookupIterator") || s.startsWith("ConcurrentLookupIterator"));
if (inlDefs.length < 15) throw new Error(`lookup-inl.h 定义应 ≥15, 实得 ${inlDefs.length}`);

// ── 状态迁移/规则锚 (must 断言) ───────────────────────────────────────────
const anchors = [
  [cc, "state_ = IsElement() ? LookupInSpecialHolder<true>(map, holder)", "lookup.cc (元素/属性分派)"],
  [cc, "switch (state_) {", "lookup.cc (状态机)"],
  [cc, "if (!new_map->is_dictionary_map()) {  // fast -> fast", "lookup.cc (字典/快属性分派)"],
  [cc, "if (IsJSProxy(*holder)) return;", "lookup.cc (代理分派)"],
  [cc, "if (IsElement(*holder)) {", "lookup.cc (元素分支)"],
  [inl, "DisallowGarbageCollection no_gc", "lookup-inl.h (GC 禁区)"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f}`);
}

const body = `${banner("src/objects/lookup.cc.js", "src/objects/lookup.{cc,-inl.h}", "属性查找迭代器方法面 (配置/状态枚举归 v8-objects-rest/lookup.rs)")}
//! LookupIterator / ConcurrentLookupIterator 的 .cc + -inl.h 方法面。
//!
//! - 配置位/状态机枚举已由 lookup.h.js 提取至 v8-objects-rest/src/lookup.rs
//! - 查找分派: 元素 (IsElement) / 拦截器 / 特殊 receiver / 常规 holder 四路
//! - 字典 map 走 NameDictionary 慢路径; 快 map 走描述符 + 字段索引

// ── LookupIterator 方法面 (${lookupIt.length} 项) ────────────────────────
/// LookupIterator 全部 .cc 定义 (对数基准)。
pub const LOOKUP_ITERATOR_METHODS: [&str; ${lookupIt.length}] = [
${lookupIt.map((s) => `  "${s}",`).join("\n")}
];

// ── ConcurrentLookupIterator 方法面 (${conIt.length} 项) ─────────────────
/// ConcurrentLookupIterator 全部 .cc 定义 (无锁并发读路径)。
pub const CONCURRENT_LOOKUP_ITERATOR_METHODS: [&str; ${conIt.length}] = [
${conIt.map((s) => `  "${s}",`).join("\n")}
];

// ── 分派规则登记 ───────────────────────────────────────────────────────
/// 四路分派: 元素/拦截器/特殊 receiver/常规 holder。
pub const DISPATCH_RULES: [&str; 4] = [
  "IsElement(holder) → 元素路径",
  "拦截器 (SkipInterceptor 判定) → API 拦截",
  "IsSpecialReceiverMap → LookupInSpecialHolder",
  "常规 → LookupInRegularHolder (字典/描述符二分)",
];

// ── 主流程签名骨架 (就地修订; 参数为 Address/索引占位) ─────────────────
/// 启动查找 (Start: 首次 LookupInHolder + NotFound 预判)。
pub fn start(_it: u64) { todo!("就地修订") }
/// 推进到下一原型层 (Next: 代理跳层 + NextInternal)。
pub fn next(_it: u64) { todo!("就地修订") }
/// 常规 holder 查找 (LookupInRegularHolder: 字典/描述符二分)。
pub fn lookup_in_regular_holder(_map: u64, _holder: u64) -> u8 { todo!("就地修订") }
/// 特殊 receiver 查找 (LookupInSpecialHolder: 代理/拦截器/字符串包装)。
pub fn lookup_in_special_holder(_map: u64, _holder: u64) -> u8 { todo!("就地修订") }
/// 数据属性赋值准备 (PrepareForDataProperty: 常量性/表示法保持判定)。
pub fn prepare_for_data_property(_it: u64, _value: u64) { todo!("就地修订") }
/// 重配置数据属性 (ReconfigureDataProperty: 属性属性/表示法变更)。
pub fn reconfigure_data_property(_it: u64, _value: u64, _attributes: u8) { todo!("就地修订") }
/// 删除属性 (Delete: 字典直删 / 快属性转字典)。
pub fn delete(_it: u64) { todo!("就地修订") }
/// 取当前值 (FetchValue: 字段/常量/访问器三路)。
pub fn fetch_value(_it: u64) -> u64 { todo!("就地修订") }
/// 写当前值 (WriteDataValue: inobject/outofobject 字段写 + 写屏障)。
pub fn write_data_value(_it: u64, _value: u64, _store_barrier: u8) { todo!("就地修订") }

// lookup-inl.h 定义清单 (对数基准, ${inlDefs.length} 项):
${inlDefs.map((s) => `// ${s}`).join("\n")}

// —— 提取不变式断言 ——
const _: () = {
  assert!(LOOKUP_ITERATOR_METHODS.len() == ${lookupIt.length});
  assert!(CONCURRENT_LOOKUP_ITERATOR_METHODS.len() == ${conIt.length});
  assert!(LOOKUP_ITERATOR_METHODS.contains(&"ApplyTransitionToDataProperty"));
  assert!(DISPATCH_RULES.len() == 4);
};
`;

writeSrc("v8-objects-collections", "lookup.rs", body);
console.log(
  `lookup.rs: LookupIterator ${lookupIt.length} + Concurrent ${conIt.length} + 分派规则 4 + 骨架 9 + inl ${inlDefs.length}, 对数通过`,
);
