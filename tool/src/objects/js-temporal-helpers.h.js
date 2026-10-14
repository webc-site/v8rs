#!/usr/bin/env bun
// tool/src/objects/js-temporal-helpers.h.js — temporal 辅助记录骨架 (DurationRecord 字段 + 宏 + Table 8 对数)
// 组→文件映射: js-temporal-helpers.h + js-temporal-helpers.cc
// 产物: v8-objects-js-core/src/js_temporal_helpers.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-temporal-helpers.h");
const cc = readV8("src/objects/js-temporal-helpers.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. TimeDurationRecord / DurationRecord 字段解析 ────────────────────
const tdBody = h.slice(h.indexOf("struct TimeDurationRecord"), h.indexOf("// https://tc39.es/proposal-temporal/#sec-temporal-duration-records"));
const tdFields = [...tdBody.matchAll(/^  double (\w+);$/gm)].map((m) => m[1]);
eq(tdFields.length, 7, "TimeDurationRecord 字段数");
must(tdFields.join(",") === "days,hours,minutes,seconds,milliseconds,microseconds,nanoseconds", `Time 字段序 ${tdFields}`);
const durBody = h.slice(h.indexOf("struct DurationRecord"), h.indexOf("// https://tc39.es/proposal-temporal/#sec-todurationrecord-deleted"));
const durFields = [...durBody.matchAll(/^  (double (\w+);|TimeDurationRecord (\w+);)$/gm)].map((m) => m[2] || m[3]);
eq(durFields.length, 4, "DurationRecord 字段数");
must(durFields.join(",") === "years,months,weeks,time_duration", `Duration 字段序 ${durFields}`);

// ── 2. 静态方法声明 ─────────────────────────────────────────────────────
must(h.includes("static Maybe<TimeDurationRecord> Create(Isolate* isolate, double days,"), "TimeDurationRecord::Create 声明");
must(h.includes("static Maybe<DurationRecord> Create(Isolate* isolate, double years,"), "DurationRecord::Create 声明");
must(h.includes("static int32_t Sign(const DurationRecord& dur);"), "DurationRecord::Sign 声明");
must(h.includes("Maybe<DurationRecord> ToDurationRecord("), "ToDurationRecord 声明");
must(h.includes("bool IsValidDuration(Isolate* isolate, const DurationRecord& dur);"), "IsValidDuration 声明");
must(h.includes("namespace temporal {"), "temporal 命名空间");

// ── 3. 宏族 (错误构造) ──────────────────────────────────────────────────
for (const m of ["NEW_TEMPORAL_TYPE_ERROR(str)", "NEW_TEMPORAL_RANGE_ERROR(str)",
  "NEW_TEMPORAL_RANGE_ERROR_WITH_ARG(str, arg)", "NEW_TEMPORAL_TYPE_ERROR_WITH_ARG(str, arg)"])
  must(h.includes(`#define ${m}`), `宏 ${m}`);
must(h.includes("#define TEMPORAL_DEBUG_INFO AT") || h.includes("#define TEMPORAL_DEBUG_INFO \"\""), "TEMPORAL_DEBUG_INFO 双分支");

// ── 4. cc Table 8 对数 (10 行 duration 字段表, 按名排序遍历) ────────────
const table8 = [...cc.matchAll(/\{factory->(\w+_string)\(\), &record->(?:time_duration\.)?(\w+)\}/g)].map((m) => `${m[1]}→${m[2]}`);
eq(table8.length, 10, "cc Table 8 行数");
must(table8[0] === "days_string→days", `Table8 首行 ${table8[0]}`);
must(table8[9] === "years_string→years", `Table8 末行 ${table8[9]}`);
// IsValidDuration 边界常量
must(cc.includes("static const double kPower32Of2 = static_cast<double>(int64_t(1) << 32);"), "kPower32Of2 = 2^32");
must(cc.includes("static const int64_t kPower53Of2 = int64_t(1) << 53;"), "kPower53Of2 = 2^53");
must(cc.includes("bool temporal::IsValidDuration(Isolate* isolate, const DurationRecord& dur) {") ||
  cc.includes("bool IsValidDuration(Isolate* isolate, const DurationRecord& dur) {"), "cc IsValidDuration 定义");
must(cc.includes("ToIntegerWithoutRounding"), "cc ToIntegerWithoutRounding (非积分数抛 RangeError)");
must(cc.includes("bool IsIntegralNumber(Isolate* isolate, DirectHandle<Object> argument) {"), "cc IsIntegralNumber 定义");

// ── 5. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-temporal-helpers.h.js", "src/objects/js-temporal-helpers.h + js-temporal-helpers.cc", "temporal DurationRecord 记录与校验差集骨架")}
//! 非堆对象记录 (Rust struct); Table 8 遍历按属性名字典序 days..years。

/// v8: src/objects/js-temporal-helpers.h temporal::TimeDurationRecord
#[derive(Clone, Copy, Default)]
pub struct TimeDurationRecord {
${tdFields.map((f) => `    pub ${f}: f64,`).join("\n")}
}

/// v8: src/objects/js-temporal-helpers.h temporal::DurationRecord
#[derive(Clone, Copy, Default)]
pub struct DurationRecord {
    pub years: f64,
    pub months: f64,
    pub weeks: f64,
    pub time_duration: TimeDurationRecord,
}

// cc 校验语义 (js-temporal-helpers.cc):
//   IsValidDuration: 全字段有限 + 符号一致 (sign>0 ⇒ 无负分量; sign<0 ⇒ 无正分量) +
//     |years| < 2^32 / |days| < 2^32 / 其余 < 2^53 (kPower32Of2 / kPower53Of2)
//   DurationRecord::Sign: 10 字段全零 → 0, 否则按首非零分量符号
//   ToIntegerWithoutRounding: NaN/±0 → 0; 非积分 → RangeError
//   IterateDurationRecordFieldsTable: 10 行 Table 8 (days..years 字典序) 逐属性 Get
// 宏映射: NEW_TEMPORAL_TYPE_ERROR/_RANGE_ERROR(_WITH_ARG) → MessageTemplate::kTemporal(_WithArg)
`;

const p = writeSrc("v8-objects-js-core", "js_temporal_helpers.rs.skel", body);
console.log(
  `js-temporal-helpers.h.js: ${p} — Time 字段 ${tdFields.length}/7、Duration 字段 ${durFields.length}/4、宏 4/4、Table8 ${table8.length}/10、cc 锚 5/5、断言 ${checks} 项全部通过`,
);
