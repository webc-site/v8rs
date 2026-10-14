#!/usr/bin/env bun
// tool/src/objects/js-duration-format.h.js — JSDurationFormat 枚举/双 flags 位布局提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-duration-format.{h,-inl.h,cc,tq}
//   → v8-objects-js-core/src/js_duration_format.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-duration-format.h");
const inl = readV8("src/objects/js-duration-format-inl.h");
const cc = readV8("src/objects/js-duration-format.cc");
const tq = readV8("src/objects/js-duration-format.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. 枚举 (Display 2 / Style 4 / Separator 4 / FieldStyle 7+3max) ──────
has(h, "enum class Display {\n    kAuto,\n    kAlways,\n\n    kMax = kAlways\n  };", "h Display");
has(h, "enum class Style {\n    kLong,\n    kShort,\n    kNarrow,\n    kDigital,\n\n    kMax = kDigital\n  };", "h Style");
has(h, "enum class Separator {\n    kColon,", "h Separator");
has(h, "kFullwidthColon,          // U+FF1A", "h Separator fullwidth");
has(h, "kArabicDecimalSeparator,  // U+066B", "h Separator arabic");
has(h, "enum class FieldStyle {\n    kLong,\n    kShort,\n    kNarrow,\n    kNumeric,\n    k2Digit,\n    kFractional,\n    kUndefined,\n\n    kStyle3Max = kNarrow,\n    kStyle4Max = kFractional,\n    kStyle5Max = k2Digit,\n  };", "h FieldStyle");

// ── 2. style_flags / display_flags 位布局 (h BitField 链 + tq bitfield 对数) ──
const STYLE_H = [["style", "StyleBits", 0, 2], ["years_style", "YearsStyleBits", 2, 2],
  ["months_style", "MonthsStyleBits", 4, 2], ["weeks_style", "WeeksStyleBits", 6, 2],
  ["days_style", "DaysStyleBits", 8, 2], ["hours_style", "HoursStyleBits", 10, 3],
  ["minutes_style", "MinutesStyleBits", 13, 3], ["seconds_style", "SecondsStyleBits", 16, 3],
  ["milliseconds_style", "MillisecondsStyleBits", 19, 3], ["microseconds_style", "MicrosecondsStyleBits", 22, 3],
  ["nanoseconds_style", "NanosecondsStyleBits", 25, 3], ["separator", "SeparatorBits", 28, 2]];
for (const [n, alias, shift, bits] of STYLE_H)
  has(h, `using ${alias} =`, `h ${alias}`);
// BitField 链逐项验证 (Next 链 shift 累加)
const styleTq = tq.slice(tq.indexOf("bitfield struct JSDurationFormatStyleFlags"));
const styleTqF = [...styleTq.slice(0, styleTq.indexOf("}")).matchAll(/(\w+): \w+.*?: (\d+) bit/g)].map((m) => [m[1], +m[2]]);
must(styleTqF.length === 12, `tq style_flags 应 12 项, 实得 ${styleTqF.length}`);
let acc = 0;
const styleLay = styleTqF.map(([n, b]) => { const s = acc; acc += b; return [n, s, b]; });
must(acc === 30, `style_flags 总位宽应 30, 实得 ${acc}`);
for (let i = 0; i < 12; i++)
  must(styleLay[i][0] === STYLE_H[i][0] && styleLay[i][2] === STYLE_H[i][3],
    `style_flags[${i}] ${styleLay[i]} 与 h 表 ${STYLE_H[i]} 不一致`);

const DISP_TQ_EXPECT = [
  ["years_display", 1], ["months_display", 1], ["weeks_display", 1], ["days_display", 1],
  ["hours_display", 1], ["minutes_display", 1], ["seconds_display", 1],
  ["milliseconds_display", 1], ["microseconds_display", 1], ["nanoseconds_display", 1],
  ["fractional_digits", 4],
];
const dispTq = tq.slice(tq.indexOf("bitfield struct JSDurationFormatDisplayFlags"));
const dispTqF = [...dispTq.slice(0, dispTq.indexOf("}")).matchAll(/(\w+): \w+.*?: (\d+) bit/g)].map((m) => [m[1], +m[2]]);
must(JSON.stringify(dispTqF) === JSON.stringify(DISP_TQ_EXPECT), `tq display_flags 失配: ${JSON.stringify(dispTqF)}`);
acc = 0;
const dispLay = dispTqF.map(([n, b]) => { const s = acc; acc += b; return [n, s, b]; });
must(acc === 14, `display_flags 总位宽应 14, 实得 ${acc}`);
// static_assert 全集: display 10 + style 1 + field style 10 = 21
must((h.match(/static_assert\(/g) || []).length === 21, "static_assert 应 21 项");
has(h, "static const uint32_t kUndefinedFractionalDigits = 15;", "h kUndefinedFractionalDigits");

// ── 3. 字段 (4) 与方法 ────────────────────────────────────────────────────
const seg = tq.slice(tq.indexOf("extern class JSDurationFormat"));
const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (\w+):\s*([^;]+);/gm)]
  .map((m) => [m[1], m[2].replace(/\s+/g, " ").trim()]);
must(JSON.stringify(tqF.map((f) => f[0])) === JSON.stringify(
  ["style_flags", "display_flags", "icu_locale", "icu_number_formatter"]), `tq 字段失配: ${JSON.stringify(tqF)}`);
for (const m of ["New(", "ResolvedOptions(", "Format(", "FormatToParts(", "TemporalToLocaleString(",
  "GetAvailableLocales();"]) has(h, m, `JSDurationFormat::${m}`);
// 宏展开 setter/getter: display 10 + style/separator 2 + field style 10 = 22
must((h.match(/^ {2}DECLARE_INLINE_DISPLAY_SETTER_GETTER\(/gm) || []).length === 10, "display 宏应 10");
must((h.match(/^ {2}DECLARE_INLINE_FIELD_STYLE_SETTER_GETTER\(/gm) || []).length === 10, "field style 宏应 10");
has(inl, "inline void JSDurationFormat::set_fractional_digits(int32_t digits) {", "inl set_fractional_digits");
has(inl, "kUndefinedFractionalDigits", "inl undefined 哨兵");

// ── 4. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const K_HEADER = 3 * T + 4 * T;
must(K_HEADER === 56, `布局推导失配 ${K_HEADER}`);

const body = `${banner("src/objects/js-duration-format.h.js", "src/objects/js-duration-format.{h,-inl.h,cc,tq}", "JSDurationFormat 枚举/双 flags 位布局骨架")}
//! 差集骨架: JSDurationFormat (DurationFormat 10 字段 display+style 压缩进两个 Smi flags)。
//! 配置: 64 位非压缩 (kTaggedSize=8)。ICU 数字格式化语义面留 Developer。

/// v8: src/objects/js-duration-format.h JSDurationFormat::Display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display { Auto = 0, Always = 1 }

/// v8: src/objects/js-duration-format.h JSDurationFormat::Style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style { Long = 0, Short = 1, Narrow = 2, Digital = 3 }

/// v8: src/objects/js-duration-format.h JSDurationFormat::Separator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Separator { Colon = 0, FullStop = 1, FullwidthColon = 2, ArabicDecimalSeparator = 3 }

/// v8: src/objects/js-duration-format.h JSDurationFormat::FieldStyle
///   (kStyle3Max = kNarrow=2, kStyle4Max = kFractional=5, kStyle5Max = k2Digit=4)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldStyle { Long = 0, Short = 1, Narrow = 2, Numeric = 3, TwoDigit = 4, Fractional = 5, Undefined = 6 }

/// v8: src/objects/js-duration-format.h kUndefinedFractionalDigits (4 位全 1 哨兵)
pub const K_UNDEFINED_FRACTIONAL_DIGITS: u32 = 15;

// ── style_flags_ 位布局 (uint31 Smi, 总 30 位; h BitField 链 = tq StyleFlags) ──
${styleLay.map(([n, s, b]) => `pub const K_${n.toUpperCase()}_SHIFT: u32 = ${s}; // ${b} bits`).join("\n")}

// ── display_flags_ 位布局 (uint31 Smi, 总 14 位; = tq DisplayFlags) ──
${dispLay.map(([n, s, b]) => `pub const K_${n.toUpperCase()}_SHIFT: u32 = ${s}; // ${b} bits`).join("\n")}

/// v8: src/objects/js-duration-format.h JSDurationFormat: style_flags_ offset=24 (Smi),
///   display_flags_ 32 (Smi), icu_locale_ 40 (Foreign), icu_number_formatter_ 48 (Foreign)
pub const K_JS_DURATION_FORMAT_HEADER_SIZE: usize = ${K_HEADER};

/// v8: src/objects/js-duration-format-inl.h set_fractional_digits/fractional_digits +
///   宏展开 22 组 display/style setter-getter
pub struct JSDurationFormat { /* style_flags_, display_flags_, icu_locale_, icu_number_formatter_ */ }
`;

const p = writeSrc("v8-objects-js-core", "js_duration_format.rs.skel", body);
console.log(
  `js-duration-format.h.js: ${p} — 枚举 2+4+4+7 值、style 位域 ${styleTqF.length}/12 (30 位)、` +
    `display 位域 ${dispTqF.length}/11 (14 位)、字段 ${tqF.length}/4、布局 ${K_HEADER}、断言 ${checks} 项通过`,
);
