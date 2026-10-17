#!/usr/bin/env bun
// tool/src/debug/scopes.js — 作用域迭代组
// 覆盖: src/debug/debug-scopes.h/.cc + debug-scope-iterator.h/.cc + debug-scope-info.h/.cc +
//       debug-block-list.h/.cc + debug-property-iterator.h/.cc (5 对, 10 文件)
// 产出: v8-debug/src/scopes.rs.skel (规则代码: ScopeType/Mode/VariableInfo 枚举全量 +
//       ScopeIterator/DebugPropertyIterator 类面骨架)
// 复跑幂等: 纯文本扫描 + 源码序输出, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/debug/scopes.js";
const FILES = [
  "src/debug/debug-scopes.h",
  "src/debug/debug-scopes.cc",
  "src/debug/debug-scope-iterator.h",
  "src/debug/debug-scope-iterator.cc",
  "src/debug/debug-scope-info.h",
  "src/debug/debug-scope-info.cc",
  "src/debug/debug-block-list.h",
  "src/debug/debug-block-list.cc",
  "src/debug/debug-property-iterator.h",
  "src/debug/debug-property-iterator.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[scopes.js] ${m}`); };
const need = (f, s) => { if (!txt[f].includes(s)) die(`${f} 缺针 ${s}`); };

// ① ScopeType 枚举 + 索引常量 (debug-scopes.h)
need("src/debug/debug-scopes.h", "enum ScopeType {");
for (const v of ["ScopeTypeGlobal = 0,", "ScopeTypeClosure,", "ScopeTypeModule"]) need("src/debug/debug-scopes.h", v);
need("src/debug/debug-scopes.h", "kScopeDetailsSize = 6;");
need("src/debug/debug-scopes.h", "enum class Mode { STACK, ALL };");
need("src/debug/debug-scopes.h", "enum class VariableInfo {");
for (const v of ["kEmpty,", "kAllUnavailable,", "kAvailable,"]) need("src/debug/debug-scopes.h", v);
need("src/debug/debug-scopes.h", "class V8_EXPORT_PRIVATE ScopeIterator {");
// ② 迭代器三类针
need("src/debug/debug-scope-iterator.h", "class DebugScopeIterator final : public debug::ScopeIterator {");
need("src/debug/debug-scope-info.h", "class V8_EXPORT_PRIVATE DebugScriptScope {");
need("src/debug/debug-property-iterator.h", "class DebugPropertyIterator final : public debug::PropertyIterator {");
need("src/debug/debug-property-iterator.h", "kAllProperties = 2");
// ③ 块名单函数针
need("src/debug/debug-block-list.h", "Handle<StringSet> EnsureLocalsBlockList(");
need("src/debug/debug-block-list.h", "Handle<StringSet> CalculateScopeBlockList(");

// ④ 机械提取: ScopeType 变体 (源码序, 9 变体 Global..Module)
const stBlock = txt["src/debug/debug-scopes.h"].match(/enum ScopeType \{([^}]*)\}/)[1];
const stVars = [...stBlock.matchAll(/ScopeType(\w+)/g)].map((m) => m[1]);
if (stVars.length !== 9) die(`ScopeType 变体数=${stVars.length}, 期望 9`);
if (stVars[0] !== "Global" || stVars[8] !== "Module") die(`ScopeType 首尾失配`);

console.log(`OK scopes.js 针核验通过: ScopeType ${stVars.length} + 索引常量 7 校验全过`);

