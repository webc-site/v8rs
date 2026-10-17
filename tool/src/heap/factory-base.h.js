#!/usr/bin/env bun
// tool/src/heap/factory-base.h.js
// 机械提取 v8/src/heap/factory-base.{h,cc,-inl.h}:
// NumberCacheMode 枚举 + NewCodeOptions 字段清单 (20) + FactoryBase New* 方法
// 名称清单 (正则提取 + 计数对数) + ToBoolean 规则。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/factory-base.h");
const cc = readV8("src/heap/factory-base.cc");
const inl = readV8("src/heap/factory-base-inl.h");

// NumberCacheMode
const ncm = h.match(/enum class NumberCacheMode \{ ([^}]*) \};/);
if (!ncm || ncm[1].replace(/,\s*$/, "").split(",").map((s) => s.trim()).join() !== "kIgnore,kSetOnly,kBoth") {
  throw new Error("NumberCacheMode 漂移");
}
// NewCodeOptions 字段清单
const nco = h.match(/struct NewCodeOptions \{([\s\S]*?)\n\};/);
if (!nco) throw new Error("NewCodeOptions 缺失");
const fields = [...nco[1].matchAll(/^\s+(?:[\w:<>, ]+?)\s+(\w+);/gm)].map((m) => m[1]);
if (fields.length !== 20) throw new Error(`NewCodeOptions 字段 ${fields.length}, 期望 20`);
for (const must of ["kind", "builtin", "instruction_start", "deoptimization_data"]) {
  if (!fields.includes(must)) throw new Error(`NewCodeOptions 字段缺 ${must}`);
}
// New* 方法名清单 (Handle/DirectHandle 返回)
const news = [...h.matchAll(/(?:Direct)?Handle<[\w:<>, ]+>\s+(New\w+)\(/g)].map((m) => m[1]);
const uniqueNews = [...new Set(news)];
if (uniqueNews.length < 60) throw new Error(`FactoryBase New* ${uniqueNews.length}, 期望 >= 60`);
for (const must of ["NewFixedArray", "NewCode", "NewNumber", "NewHeapNumber", "NewHeapNumberFromBits"]) {
  if (!uniqueNews.includes(must)) throw new Error(`New* 清单缺 ${must}`);
}
// 关键锚
for (const a of [
  "template <typename Impl>",
  "class FactoryBase {",
  "template <typename T, typename... Args>",
  "inline DirectHandle<T> New(AllocationType allocation, Args&&... args);",
  "inline Handle<Boolean> ToBoolean(bool value);",
  "Handle<Code> NewCode(const NewCodeOptions& options);",
  "DirectHandle<CodeWrapper> NewCodeWrapper();",
  "using FixedInt32Array = FixedIntegerArrayBase<int32_t, ByteArray>;",
  "using FixedUInt32Array = FixedIntegerArrayBase<uint32_t, ByteArray>;",
]) {
  if (!h.includes(a)) throw new Error(`factory-base.h 锚缺失: ${a}`);
}
// cc / inl
for (const a of [
  "Handle<Code> FactoryBase<Impl>::NewCode(",
]) {
  if (!cc.includes(a)) throw new Error(`factory-base.cc 锚缺失: ${a}`);
}
for (const a of [
  "FactoryBase<Impl>::NewNumber(",
  "FactoryBase<Impl>::ToBoolean(",
]) {
  if (!inl.includes(a)) throw new Error(`factory-base-inl.h 锚缺失: ${a}`);
}
// ToBoolean 规则 (inl): value ? impl()->true_value() : impl()->false_value()
if (!inl.includes("value ? Cast<Boolean>(impl()->true_value())")) throw new Error("ToBoolean 实现体漂移");
console.log(`FactoryBase: 枚举 1 + NewCodeOptions 字段 ${fields.length} + New* ${uniqueNews.length}`);

let rust = banner(
  "src/heap/factory-base.h.js",
  "src/heap/factory-base.{h,cc,-inl.h}",
  "工厂基类: NewCodeOptions 字段清单与 New* 方法台账",
);

rust += `
/// 数字缓存模式 (对应 enum class NumberCacheMode)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NumberCacheMode {
  /// kIgnore: 不读不写缓存
  IGNORE = 0,
  /// kSetOnly: 只写
  SET_ONLY = 1,
  /// kBoth: 读写
  BOTH = 2,
}

/// NewCode 字段集 (对应 struct NewCodeOptions, 20 字段逐 1:1)。
/// v8: src/heap/factory-base.h NewCodeOptions
#[derive(Debug, Clone, Copy, Default)]
pub struct NewCodeOptionsSkeleton {
${fields.map((f) => `  pub ${f}: (),`).join("\n")}
}

/// NewCodeOptions 字段名清单 (供对象族 12.1 对账)。
pub const NEW_CODE_OPTIONS_FIELDS: &[&str] = &[
${fields.map((f) => `  "${f}",`).join("\n")}
];

/// FactoryBase New* 方法名清单 (正则提取, 含模板缺省重载去重)。
pub const FACTORY_BASE_NEW_METHODS: &[&str] = &[
${uniqueNews.map((n) => `  "${n}",`).join("\n")}
];

/// 对应 ToBoolean: 条件取 true/false 根值。
/// v8: src/heap/factory-base-inl.h FactoryBase::ToBoolean
#[must_use]
pub fn to_boolean(value: bool, true_value: u64, false_value: u64) -> u64 {
  if value {
    true_value
  } else {
    false_value
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(NumberCacheMode::IGNORE as u8, 0);
  assert_eq!(NumberCacheMode::BOTH as u8, 2);
  assert_eq!(NEW_CODE_OPTIONS_FIELDS.len(), 20);
  assert!(NEW_CODE_OPTIONS_FIELDS.contains(&"instruction_start"));
  assert!(FACTORY_BASE_NEW_METHODS.contains(&"NewFixedArray"));
  assert_eq!(to_boolean(true, 1, 0), 1);
  assert_eq!(to_boolean(false, 1, 0), 0);
};
`;

const p = writeSrc("v8-heap-core", "factory_base.rs.skel", rust);
console.log(`written ${p}`);
