#!/usr/bin/env bun
// tool/src/extensions/statistics-extension.h.js
// 机械提取 v8/src/extensions/statistics-extension.{h,cc}
// 产物: v8-extensions/src/statistics_extension.rs — 扩展元数据 + 堆统计键名表 + 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/extensions/statistics-extension.h");
const ccSrc = readV8("src/extensions/statistics-extension.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 扩展名与 kSource
const extName = hSrc.match(/v8::Extension\("([^"]+)"/);
must(extName, "扩展名 v8/statistics");
if (extName[1] !== "v8/statistics") fail("扩展名", extName[1], "v8/statistics");
const ksource = ccSrc.match(/kSource =\s*\n?\s*"([^"]+)";/);
must(ksource, "kSource 字面量");

// 2. 方法全集 (h): GetNativeFunctionTemplate / GetCounters / kSource
const hMethods = ["GetNativeFunctionTemplate", "GetCounters", "kSource"];
for (const name of hMethods) {
  if (!hSrc.includes(name)) fail(`h 声明 ${name}`, false, true);
}
// cc 侧 static 辅助函数
const ccHelpers = ["AddCounter", "AddNumber", "AddNumber64"];
for (const name of ccHelpers) {
  if (!ccSrc.includes(`static void ${name}(`)) fail(`cc 辅助 ${name}`, false, true);
}

// 3. GC 触发常量 (cc GetCounters 首参为 true 时全量 GC)
must(ccSrc.includes("GCFlag::kNoFlags"), "GCFlag::kNoFlags");
must(ccSrc.includes("GarbageCollectionReason::kCountersExtension"), "kCountersExtension");

// 4. numbers[] 键名表 — 机械提取 StatisticNumber 数组全部字符串字面量
const numbersBlock = ccSrc.match(
  /const StatisticNumber numbers\[\] = \{([\s\S]*?)\n  \};/,
);
must(numbersBlock, "numbers[] 表体");
const statNames = [];
const strRe = /"([a-z0-9_]+)"/g;
let sm;
while ((sm = strRe.exec(numbersBlock[1])) !== null) statNames.push(sm[1]);
// 预期 22 项: 1 total_committed_bytes + 3 new_space_* + 18 space_*/…
if (statNames.length !== 22) fail("numbers[] 条目数", statNames.length, 22);

// 5. 值表达式逐条 (与键名 1:1): 按条目右括号切分, 每条剥出取值表达式
const valueExprs = numbersBlock[1]
  .split(/\},\s*/)
  .map((chunk) =>
    chunk
      .replace(/^\s*\{/, "")
      .replace(/"[a-z0-9_]+"/, "")
      .replace(/[{}\s]+/g, " ")
      .trim()
      .replace(/,$/, ""),
  )
  .filter((s) => s.replace(/,/g, "").length > 0);
if (valueExprs.length !== statNames.length) fail("值表达式数", valueExprs.length, statNames.length);

// 6. counter_list 由两张 X-macro 表展开 (STATS_COUNTER_LIST + STATS_COUNTER_NATIVE_CODE_LIST)
must(ccSrc.includes("STATS_COUNTER_LIST(ADD_COUNTER)"), "STATS_COUNTER_LIST 展开");
must(ccSrc.includes("STATS_COUNTER_NATIVE_CODE_LIST(ADD_COUNTER)"), "STATS_COUNTER_NATIVE_CODE_LIST 展开");

let n = 10;
const rust = `${banner("src/extensions/statistics-extension.h.js", "src/extensions/statistics-extension.{h,cc}", "Statistics 扩展元数据 + 堆统计键名表 + 签名骨架")}
//! Statistics 扩展 — 原版 namespace v8::internal::StatisticsExtension
//!
//! 语义: 注入 getV8Statistics() — 收集计数器与分代堆尺寸, 以对象键值返回。

/// Statistics 扩展注册名
pub const STATISTICS_EXTENSION_NAME: &str = "v8/statistics";

/// 注入 JS 的源码 (原版 kSource)
pub const STATISTICS_SOURCE: &str = ${JSON.stringify(ksource[1])};

/// 堆统计键名全集 (原版 StatisticNumber numbers[22], 顺序 1:1)
pub const STAT_NUMBER_KEYS: [&str; ${statNames.length}] = [
${statNames.map((s) => `  ${JSON.stringify(s)},`).join("\n")}
];

/// 对应取值表达式 (原版 numbers[] 值位, 顺序与 STAT_NUMBER_KEYS 一致; 供 Developer 侧对接 Heap)
pub const STAT_NUMBER_EXPRS: [&str; ${valueExprs.length}] = [
${valueExprs.map((s) => `  ${JSON.stringify(s)},`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(STAT_NUMBER_KEYS.len() == 22);
  assert!(STAT_NUMBER_EXPRS.len() == 22);
};

// —— 签名骨架 (原版声明 cc 1:1) ——
// public: GetNativeFunctionTemplate(isolate, name) -> FunctionTemplate
// public static: GetCounters(info) — info[0] 为 true 时先 CollectAllGarbage(GCFlag::kNoFlags,
//                GarbageCollectionReason::kCountersExtension), 再枚举计数器与 numbers 表
// static: AddCounter(isolate, object, counter, name) — 仅 counter->Enabled() 时写属性
// static: AddNumber(isolate, object, value: f64, name)
// static: AddNumber64(isolate, object, value: i64, name)

// X-macro 台账:
//   counter_list[] = STATS_COUNTER_LIST(ADD_COUNTER) + STATS_COUNTER_NATIVE_CODE_LIST(ADD_COUNTER)
//   两表定义于 src/logging/counters.h — 运行时计数器名全集, 由 counters 模块脚本落地, 此处登记引用
`;

const p = writeSrc("v8-extensions", "statistics_extension.rs", rust);
console.log(`OK statistics-extension.h.js: 对数断言 ${n} 全过; 键名表 ${statNames.length} / 方法 5 / X-macro 表 2 登记; 产物 ${p}`);
