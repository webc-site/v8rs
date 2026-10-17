#!/usr/bin/env bun
// tool/src/compiler/type-cache.js
// v8/src/compiler/type-cache.h + type-cache.cc
// → v8-compiler-passes/src/type_cache.rs.skel
// 类型缓存: 真解析 k* 常量表 (CreateRange / 工厂调用形态分类计数)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/type-cache.h");
const cc = readV8("src/compiler/type-cache.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`type-cache: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE TypeCache final {", "TypeCache 定义"],
  ["Type const kInt8 = CreateRange<int8_t>();", "kInt8 首项"],
  ["Type const kSingletonZero = CreateRange(0.0, 0.0);", "kSingletonZero"],
  ["static TypeCache const* Get();", "Get 单例"],
]) need(h.includes(s), `哨兵缺失 ${name}`);
need(cc.includes("DEFINE_LAZY_LEAKY_OBJECT_GETTER(const TypeCache, TypeCache::Get)"), "cc Get 缺失");

// —— k* 常量全表 (真解析: 名 + RHS 表达式) ——
const pub = h.slice(h.indexOf("class V8_EXPORT_PRIVATE TypeCache final {")).replace(/= *\n\s+/g, "= ").replace(/\(([^()]*) *\n\s*/g, "($1 ");
const entries = [];
for (const m of pub.matchAll(/^\s{2}Type const (\w+) = ([^;]+);/gm)) {
  entries.push({ name: m[1], rhs: m[2].replace(/\s+/g, " ").trim() });
}
need(entries.length >= 60, `常量数 ${entries.length} (全表 69 项 Type const k*)`);

// RHS 形态分类 (对数)
const byForm = {
  createRangeTyped: entries.filter((e) => /^CreateRange<\w+>\(\)$/.test(e.rhs)).length,
  createRangeDouble: entries.filter((e) => /^CreateRange\((-?[\d.e+]+), (-?[\d.e+]+)\)$/.test(e.rhs)).length,
  createRangeConst: entries.filter((e) => /^CreateRange\(([-\w.:()]+), ([-\w.:()]+)\)$/.test(e.rhs) && !/^CreateRange\(-?[\d.e+]+, -?[\d.e+]+\)$/.test(e.rhs)).length,
  typeFactory: entries.filter((e) => /^Type::\w+\(\)$/.test(e.rhs)).length,
  alias: entries.filter((e) => /^k\w+$/.test(e.rhs)).length,
  union: entries.filter((e) => e.rhs.includes("Type::Union")).length,
  fieldRef: entries.filter((e) => /::k\w+/.test(e.rhs) && !e.rhs.includes("CreateRange") && !e.rhs.includes("Type::")).length,
};
// 样例对数
need(entries.some((e) => e.name === "kInt8" && e.rhs === "CreateRange<int8_t>();".replace(";", "")), "kInt8 形态");
need(entries.some((e) => e.name === "kUint8Clamped" && e.rhs === "kUint8"), "kUint8Clamped 别名");
need(entries.some((e) => e.name === "kSafeInteger" && e.rhs.includes("kMaxSafeInteger")), "kSafeInteger 锚");

const snakeUpper = (n) => n.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

const body = `${banner("src/compiler/type-cache.js", "src/compiler/type-cache.h + type-cache.cc", "全图共享的单例/区间类型缓存")}
//! type-cache — TypeCache 常量表 (${entries.length} 项)
//!
//! RHS 形态对数: CreateRange<T> ${byForm.createRangeTyped} /
//! CreateRange(数字) ${byForm.createRangeDouble} / CreateRange(常量域) ${byForm.createRangeConst} /
//! Type::工厂 ${byForm.typeFactory} / 别名 ${byForm.alias} / Union ${byForm.union} /
//! 域引用 ${byForm.fieldRef}。

/// 原版 TypeCache — 单例缓存 (LazyLock, zone 无关)
pub struct TypeCache {
  // —— 全表 (${entries.length} 项; RHS 见 v8 锚, Developer 依 turbofan_types 常量填值) ——
${entries.map((e) => `  /// ${e.name} = ${e.rhs}\n  /// v8: src/compiler/turbofan-types.h ${e.rhs.includes("Type::") ? "Type::" + e.rhs.match(/Type::(\w+)/)?.[1] : "CreateRange/别名"}\n  pub ${snakeUpper(e.name)}: Type,`).join("\n")}
}

impl TypeCache {
  /// 原版 Get — 进程级单例
  /// v8: src/compiler/type-cache.cc TypeCache::Get
  pub fn get() -> &'static Self {
    todo!("v8: src/compiler/type-cache.cc TypeCache::Get (LazyLock 单例)")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "type_cache.rs.skel", body);
console.log(
  [
    `written ${p}`,
    `entries=${entries.length}`,
    `形态: range_typed=${byForm.createRangeTyped} range_num=${byForm.createRangeDouble} range_const=${byForm.createRangeConst} factory=${byForm.typeFactory} alias=${byForm.alias} union=${byForm.union} field=${byForm.fieldRef}`,
  ].join("\n"),
);
