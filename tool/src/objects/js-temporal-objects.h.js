#!/usr/bin/env bun
// tool/src/objects/js-temporal-objects.h.js — JSTemporal 8 类骨架 (Rust 包装访问器 + 方法对数)
// 组→文件映射: js-temporal-objects.h + js-temporal-objects-inl.h + js-temporal-objects.cc + js-temporal-objects.tq
// 产物: v8-objects-js-core/src/js_temporal_objects.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-temporal-objects.h");
const inl = readV8("src/objects/js-temporal-objects-inl.h");
const cc = readV8("src/objects/js-temporal-objects.cc");
const tq = readV8("src/objects/js-temporal-objects.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. V8_OBJECT 类全集 (8) ─────────────────────────────────────────────
const classes = [...h.matchAll(/V8_OBJECT class (JSTemporal\w+) : public JSObject \{/g)].map((m) => m[1]);
eq(classes.length, 8, "JSTemporal 类总数");
must(classes.join(",") === "JSTemporalDuration,JSTemporalInstant,JSTemporalPlainDate,JSTemporalPlainDateTime,JSTemporalPlainMonthDay,JSTemporalPlainTime,JSTemporalPlainYearMonth,JSTemporalZonedDateTime", `类序 ${classes}`);

// ── 2. 每类 Rust 包装字段 + calendar 标志 ──────────────────────────────
const wrapper = [...h.matchAll(/DECL_ACCESSORS_FOR_RUST_WRAPPER\((\w+), temporal_rs::(\w+)\)/g)].map((m) => [m[1], m[2]]);
eq(wrapper.length, 8, "DECL_ACCESSORS_FOR_RUST_WRAPPER 计数");
const expectWrap = [
  ["duration", "Duration"], ["instant", "Instant"], ["date", "PlainDate"], ["date_time", "PlainDateTime"],
  ["month_day", "PlainMonthDay"], ["time", "PlainTime"], ["year_month", "PlainYearMonth"], ["zoned_date_time", "ZonedDateTime"],
];
for (let i = 0; i < 8; i++) must(wrapper[i][0] === expectWrap[i][0] && wrapper[i][1] === expectWrap[i][1], `包装 ${i} 应 ${expectWrap[i]}, 实得 ${wrapper[i]}`);
const calFlags = [...h.matchAll(/static constexpr bool kTypeContainsCalendar = (true|false);/g)].map((m) => m[1]);
eq(calFlags.length, 8, "kTypeContainsCalendar 计数");
must(calFlags.join(",") === "false,false,true,true,true,false,true,true", `calendar 标志序 ${calFlags}`);
eq((h.match(/^  DECL_CTOR_HELPER\(\)$/gm) || []).length, 8, "DECL_CTOR_HELPER 调用计数 (2 空格缩进, 排除 #define 本体)");

// ── 3. ManagedTypeId 登记表 (8) ─────────────────────────────────────────
const ids = [...h.matchAll(/ASSIGN_MANAGED_TYPE_ID_FOR_MANAGED\(temporal_rs::(\w+),\s*\n\s*ManagedTypeId::k(\w+)\)/g)].map((m) => [m[1], m[2]]);
eq(ids.length, 8, "ASSIGN_MANAGED_TYPE_ID 计数");
must(ids[0][0] === "Instant" && ids[0][1] === "TemporalInstant", `首项 ${ids[0]}`);
must(ids[7][0] === "ZonedDateTime" && ids[7][1] === "TemporalZonedDateTime", `末项 ${ids[7]}`);

// ── 4. 宏定义本体 (h:32-42) ────────────────────────────────────────────
must(h.includes("#define DECL_ACCESSORS_FOR_RUST_WRAPPER(field, RustType_)"), "DECL_ACCESSORS_FOR_RUST_WRAPPER 宏定义");
must(h.includes("inline CppGCManaged<RustType_>::Ptr wrapped_rust() const;"), "wrapped_rust() 声明 (宏体内)");
must(h.includes("TaggedMember<CppGCManaged<RustType_>> field##_;"), "宏体存储槽");

// ── 5. cc 方法对数 (各类静态方法定义) ──────────────────────────────────
const ccDefs = {};
for (const c of classes) ccDefs[c] = (cc.match(new RegExp(`^(?:V8_WARN_UNUSED_RESULT )?(?:MaybeDirectHandle<[\\w<>]+>|MaybeDirectHandle<[^>]+>|Maybe<[^>]+>|MaybeDirectHandle<\\w+>) ${c}::(\\w+)\\(`, "gm")) || []).length;
const totalDefs = Object.values(ccDefs).reduce((a, b) => a + b, 0);
must(totalDefs > 100, `cc JSTemporal 静态方法定义总数 ${totalDefs} (应 > 100)`);
eq((inl.match(/DEFINE_TEMPORAL_ACCESSORS\(JSTemporal/g) || []).length, 8, "inl DEFINE_TEMPORAL_ACCESSORS 计数");
must(cc.includes("JSTemporalDuration::Constructor(") || cc.includes("JSTemporalDuration::Constructor(Isolate*"), "cc Duration::Constructor");
must(cc.includes("JSTemporalZonedDateTime::ToString("), "cc ZonedDateTime::ToString");

// ── 6. inl + tq 交叉 ────────────────────────────────────────────────────
must(inl.includes("JSTemporalDuration::duration()") || inl.includes("wrapped_rust() const {"), "inl wrapped_rust 实现");
must(inl.includes("GetConstructorTarget"), "inl GetConstructorTarget");
const tqClasses = [...tq.matchAll(/extern class (JSTemporal\w+) extends JSObject \{/g)].map((m) => m[1]);
eq(tqClasses.length, 8, "tq 类计数");
for (const c of tqClasses) must(classes.includes(c), `tq 类 ${c} 未在 h 中`);
eq((tq.match(/CppGCManagedBase;/g) || []).length, 8, "tq CppGCManagedBase 槽数");

// ── 7. 命名空间级自由函数 ───────────────────────────────────────────────
must(h.includes("MaybeDirectHandle<String> JSTemporalNowTimeZoneId("), "JSTemporalNowTimeZoneId 声明");
must(h.includes("CreateTemporalInstantWithValidityCheck(Isolate* isolate,"), "CreateTemporalInstantWithValidityCheck 声明");
must(h.includes("BuiltinTimeZoneGetPlainDateTimeFor("), "BuiltinTimeZoneGetPlainDateTimeFor 声明");
must(h.includes("ToTemporalDurationAsRecord("), "ToTemporalDurationAsRecord 声明");

// ── 8. 骨架产物 ──────────────────────────────────────────────────────────
const wrapDefs = expectWrap.map(([f, t], i) =>
  `/// v8: src/objects/js-temporal-objects.h ${classes[i]} (槽: ${f}_ @ 24, CppGCManaged<temporal_rs::${t}>)\npub const K_${classes[i].toUpperCase()}_HEADER_SIZE: usize = 32; // 3+1 槽; calendar=${calFlags[i]}`).join("\n");
const body = `${banner("src/objects/js-temporal-objects.h.js", "src/objects/js-temporal-objects.h + -inl.h + .cc + .tq", "JSTemporal 8 类包装槽/类型登记差集骨架")}
//! 每类 = JSObject + 1 个 CppGCManaged<temporal_rs::T> tagged 槽;
//! 类型登记 ASSIGN_MANAGED_TYPE_ID_FOR_MANAGED → ManagedTypeId::kTemporal*。

${wrapDefs}

// ManagedTypeId 登记表 (h:54-69):
${ids.map(([t, id]) => `// temporal_rs::${t} → ManagedTypeId::k${id}`).join("\n")}

// 方法映射 (cc ${totalDefs} 个静态定义, 语义推迟到 temporal_rs FFI 层):
//   Constructor/From/Compare/Equals/Add/Subtract/Round/Until/Since/With(WithCalendar|WithPlainTime|WithTimeZone)/
//   ToJSON/ToLocaleString/ToString/ToPlain*/ToZonedDateTime*/NowISO/GetEpochMillisecondsForUtc/...
//   JSTemporalNowTimeZoneId + temporal::CreateTemporalInstantWithValidityCheck/BuiltinTimeZoneGetPlainDateTimeFor/
//   ToTemporalDurationAsRecord (DurationFormat 共享)
// 宏三角: DECL_ACCESSORS_FOR_RUST_WRAPPER (h) ↔ DEFINE_TEMPORAL_ACCESSORS (inl x8) ↔ initialize_with_wrapped_rust_value (cc)
`;

const p = writeSrc("v8-objects-js-core", "js_temporal_objects.rs.skel", body);
console.log(
  `js-temporal-objects.h.js: ${p} — 类 ${classes.length}/8、包装 ${wrapper.length}/8、calendar 标志 ${calFlags.length}/8、ManagedTypeId ${ids.length}/8、cc 方法 ${totalDefs} (>100)、tq 类 ${tqClasses.length}/8、断言 ${checks} 项全部通过`,
);
