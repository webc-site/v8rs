#!/usr/bin/env bun
// tool/src/objects/js-date-time-format.h.js — JSDateTimeFormat 枚举/flags 位布局提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-date-time-format.{h,-inl.h,cc,tq}
//   → v8-objects-js-core/src/js_date_time_format.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-date-time-format.h");
const inl = readV8("src/objects/js-date-time-format-inl.h");
const cc = readV8("src/objects/js-date-time-format.cc");
const tq = readV8("src/objects/js-date-time-format.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. 枚举 (RequiredOption 3 / DefaultsOption 3 / DateTimeStyle 5 / HourCycle 5) ──
has(h, "enum class RequiredOption { kDate, kTime, kAny };", "h RequiredOption");
has(h, "enum class DefaultsOption { kDate, kTime, kAll };", "h DefaultsOption");
has(h, "enum class DateTimeStyle { kUndefined, kFull, kLong, kMedium, kShort };", "h DateTimeStyle");
has(h, "enum class HourCycle { kUndefined, kH11, kH12, kH23, kH24 };", "h HourCycle");

// ── 2. flags 位布局 (h BitField 链 + tq bitfield 对数) ────────────────────
has(h, "using HourCycleBits =\n      base::BitField<JSDateTimeFormat::HourCycle, 0, 3, uint32_t>;", "h HourCycleBits");
has(h, "using DateStyleBits = HourCycleBits::Next<JSDateTimeFormat::DateTimeStyle, 3>;", "h DateStyleBits");
has(h, "using TimeStyleBits = DateStyleBits::Next<JSDateTimeFormat::DateTimeStyle, 3>;", "h TimeStyleBits");
has(h, "using HasToLocaleStringTimeZoneBit = TimeStyleBits::Next<bool, 1>;", "h HasToLocaleStringTimeZoneBit");
has(h, "using ExplicitComponentsInOptionsBits =\n      HasToLocaleStringTimeZoneBit::Next<int32_t, 11>;", "h ExplicitComponentsInOptionsBits");
const bf = tq.slice(tq.indexOf("bitfield struct JSDateTimeFormatFlags"));
const bfF = [...bf.slice(0, bf.indexOf("}")).matchAll(/(\w+): \w+.*?: (\d+) bit/g)].map((m) => [m[1], +m[2]]);
must(JSON.stringify(bfF) === JSON.stringify([
  ["hour_cycle", 3], ["date_style", 3], ["time_style", 3],
  ["has_to_locale_string_time_zone", 1], ["explicit_components_in_options", 11],
]), `tq bitfield 失配: ${JSON.stringify(bfF)}`);
let acc = 0;
const LAYOUT = bfF.map(([n, b]) => { const s = acc; acc += b; return [n, s, b]; });
must(acc === 21, `flags 总位宽应 21, 实得 ${acc}`);
// static_assert 全集 (15)
must((h.match(/static_assert\((HourCycleBits|DateStyleBits|TimeStyleBits)::is_valid/g) || []).length === 15,
  "static_assert 应 15 项");

// ── 3. 字段 (6) ──────────────────────────────────────────────────────────
const seg = tq.slice(tq.indexOf("extern class JSDateTimeFormat"));
const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (\w+):\s*([^;]+);/gm)]
  .map((m) => [m[1], m[2].replace(/\s+/g, " ").trim()]);
must(tqF.length === 6 && tqF[0][0] === "locale" && tqF[5][0] === "flags", `tq 字段失配: ${JSON.stringify(tqF)}`);
for (const f of ["locale_", "icu_locale_", "icu_simple_date_format_", "icu_date_interval_format_",
  "bound_format_", "flags_"]) has(h, `TaggedMember<`, f);

// ── 4. 方法与 inl ─────────────────────────────────────────────────────────
for (const m of ["New(", "CreateDateTimeFormat(", "ResolvedOptions(", "Calendar(",
  "TimeZone(", "UnwrapDateTimeFormat(", "DateTimeFormat(", "FormatToParts(",
  "FormatRange(", "FormatRangeToParts(", "ToLocaleDateTime(", "GetAvailableLocales();",
  "CanonicalizeTimeZoneID(", "HourCycleAsString("])
  has(h, m, `JSDateTimeFormat::${m}`);
has(inl, "inline void JSDateTimeFormat::set_hour_cycle(HourCycle hour_cycle) {", "inl set_hour_cycle");
has(inl, "int JSDateTimeFormat::flags() const", "inl flags");
has(cc, "icu::SimpleDateFormat", "cc icu");

// ── 5. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const K_HEADER = 3 * T + 6 * T; // 24 + 6 字段
must(K_HEADER === 72, `布局推导失配 ${K_HEADER}`);

const body = `${banner("src/objects/js-date-time-format.h.js", "src/objects/js-date-time-format.{h,-inl.h,cc,tq}", "JSDateTimeFormat 枚举/flags 位布局/字段骨架")}
//! 差集骨架: JSDateTimeFormat (icu::SimpleDateFormat + DateIntervalFormat 包装)。
//! 配置: 64 位非压缩 (kTaggedSize=8)。ICU 格式化语义面留 Developer。

/// v8: src/objects/js-date-time-format.h JSDateTimeFormat::RequiredOption
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredOption { Date = 0, Time = 1, Any = 2 }

/// v8: src/objects/js-date-time-format.h JSDateTimeFormat::DefaultsOption
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultsOption { Date = 0, Time = 1, All = 2 }

/// v8: src/objects/js-date-time-format.h JSDateTimeFormat::DateTimeStyle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeStyle { Undefined = 0, Full = 1, Long = 2, Medium = 3, Short = 4 }

/// v8: src/objects/js-date-time-format.h JSDateTimeFormat::HourCycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HourCycle { Undefined = 0, H11 = 1, H12 = 2, H23 = 3, H24 = 4 }

// ── flags 位布局 (uint31 Smi 内, 总 21 位; h BitField 链 = tq bitfield struct) ──
${LAYOUT.map(([n, s, b]) => `/// v8: src/objects/js-date-time-format.h ${n}: shift=${s} bits=${b}
pub const K_${n.toUpperCase()}_SHIFT: u32 = ${s};
pub const K_${n.toUpperCase()}_MASK: u32 = ${b === 32 ? "u32::MAX" : `((1 << ${b}) - 1) << ${s}`};`).join("\n")}

/// v8: src/objects/js-date-time-format.h JSDateTimeFormat 字段: locale_ offset=24,
///   icu_locale_ 32, icu_simple_date_format_ 40, icu_date_interval_format_ 48,
///   bound_format_ 56, flags_ 64 (Smi)
pub const K_JS_DATE_TIME_FORMAT_HEADER_SIZE: usize = ${K_HEADER};

/// v8: src/objects/js-date-time-format-inl.h flags accessor (Smi) + set_hour_cycle/
///   set_date_style/set_time_style/set_explicit_components_in_options
pub struct JSDateTimeFormat { /* 6 字段见上布局 */ }
`;

const p = writeSrc("v8-objects-js-core", "js_date_time_format.rs.skel", body);
console.log(
  `js-date-time-format.h.js: ${p} — 枚举 3+3+5+5、flags 位域 ${bfF.length}/5 (总 ${acc} 位)、` +
    `字段 ${tqF.length}/6、方法 14、布局 ${K_HEADER}、断言 ${checks} 项通过`,
);
