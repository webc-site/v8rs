#!/usr/bin/env bun
// tool/src/objects/intl-objects.h.js — 覆盖 v8/src/objects/intl-objects.{h,cc,tq}
// → v8-objects-js-async/src/intl_objects.rs
// 机械提取: Intl 类 10 个枚举 + NumberFormatDigitOptions 结构 + kCaseMappingSpecialLocales
//   + kAsciiCollationWeightsLength (h/cc/tq 三方对数) + .cc 常量 + 方法签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/intl-objects.h");
const cc = readV8("src/objects/intl-objects.cc");
const tq = readV8("src/objects/intl-objects.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[intl-objects.h.js] ${m}`); };

// ── 1. 枚举面 (10 个, 逐个机械提取; 行式解析避开赋值与注释干扰) ──────────
function cppEnum(name, body) {
  const re = new RegExp(`enum class ${name} \\{([\\s\\S]*?)\\}`);
  const m = h.match(re);
  must(m, `h 缺 enum class ${name}`);
  const entries = m[1].split("\n")
    .map((l) => l.replace(/\/\/.*$/, "").trim())
    .filter(Boolean)
    .flatMap((l) => l.split(",").map((x) => x.trim()).filter(Boolean))
    .map((l) => {
      const x = l.match(/^(k\w+|[A-Z][A-Z_0-9]*|[A-Z]\w*)(?:\s*=.*)?$/);
      return x ? x[1] : null;
    })
    .filter(Boolean);
  must(entries.length === body.count, `${name} 条数 ${entries.length} != ${body.count}`);
  if (body.exact) must(entries.join(",") === body.exact, `${name} 条目漂移: ${entries.join(",")}`);
  return entries;
}
const enums = {
  BoundFunctionContextSlot: { count: 2, exact: "kBoundFunction,kLength" },
  Notation: { count: 4, exact: "STANDARD,SCIENTIFIC,ENGINEERING,COMPACT" },
  CompactDisplay: { count: 2, exact: "SHORT,LONG" },
  FormatRangeSource: { count: 3, exact: "kShared,kStartRange,kEndRange" },
  CompareStringsOptions: { count: 2, exact: "kNone,kTryFastPath" },
  RoundingPriority: { count: 3, exact: "kAuto,kMorePrecision,kLessPrecision" },
  RoundingType: { count: 4, exact: "kFractionDigits,kSignificantDigits,kMorePrecision,kLessPrecision" },
  RoundingMode: { count: 9, exact: "kCeil,kFloor,kExpand,kTrunc,kHalfCeil,kHalfFloor,kHalfExpand,kHalfTrunc,kHalfEven" },
  TrailingZeroDisplay: { count: 2, exact: "kAuto,kStripIfInteger" },
  MatcherOption: { count: 2, exact: "kBestFit,kLookup" },
};
const enumEntries = {};
for (const [name, body] of Object.entries(enums)) enumEntries[name] = cppEnum(name, body);
// BoundFunctionContextSlot: kBoundFunction = Context::MIN_CONTEXT_SLOTS (=2)
must(h.includes("kBoundFunction = Context::MIN_CONTEXT_SLOTS,"), "BoundFunctionContextSlot 应自 MIN_CONTEXT_SLOTS 起");

// ── 2. NumberFormatDigitOptions 结构 (10 字段, 含类型前缀的成员行) ──────
const ndo = h.match(/struct NumberFormatDigitOptions \{([\s\S]*?)\};/);
must(ndo, "h 缺 NumberFormatDigitOptions");
const ndoFields = [...ndo[1].matchAll(/^\s+\w+ (\w+);$/gm)].map((m) => m[1]);
must(ndoFields.length === 10, `NumberFormatDigitOptions 字段数 ${ndoFields.length} != 10`);
must(ndoFields.join(",") === "minimum_integer_digits,minimum_fraction_digits,maximum_fraction_digits,minimum_significant_digits,maximum_significant_digits,rounding_priority,rounding_type,rounding_increment,rounding_mode,trailing_zero_display",
  `NumberFormatDigitOptions 字段序漂移: ${ndoFields.join(",")}`);

// ── 3. kCaseMappingSpecialLocales (4 项: az/el/lt/tr) ─────────────────────
const cml = h.match(/static constexpr std::string_view kCaseMappingSpecialLocales\[\] = \{([^}]*)\}/);
must(cml, "h 缺 kCaseMappingSpecialLocales");
const locales = [...cml[1].matchAll(/"(\w+)"/g)].map((m) => m[1]);
must(locales.join(",") === "az,el,lt,tr", `kCaseMappingSpecialLocales 漂移: ${locales.join(",")}`);

// ── 4. kAsciiCollationWeightsLength 三方对数 (h 声明 / cc 落值 / tq 断言 256) ──
must(h.includes("static const int kAsciiCollationWeightsLength;"), "h 缺 kAsciiCollationWeightsLength 声明");
must(cc.includes("const int Intl::kAsciiCollationWeightsLength = kCollationWeightsLength;"), "cc 缺 kAsciiCollationWeightsLength 落值");
must(cc.includes("constexpr int kCollationWeightsLength = arraysize(kCollationWeightsL1);"), "cc 缺表长公式");
const l1Table = cc.match(/constexpr uint8_t kCollationWeightsL1\[(\d+)\] = \{([\s\S]*?)\};/);
must(l1Table, "cc 缺 kCollationWeightsL1 表");
must(Number(l1Table[1]) === 256, `kCollationWeightsL1 声明大小 ${l1Table[1]} != 256`);
const l1Count = [...l1Table[2].matchAll(/\d+/g)].length;
// 显式初始化 128 项 (ASCII 前半), 其余 128 项聚合初始化隐式补零
must(l1Count === 128, `kCollationWeightsL1 显式项数 ${l1Count} != 128 (后半隐式零)`);
must(cc.includes("static_assert(kCollationWeightsLength == arraysize(kCollationWeightsL3));"), "cc 缺 L1/L3 等长断言");
must(tq.includes("const kIntlAsciiCollationWeightsLength:\n    constexpr int31 generates 'Intl::kAsciiCollationWeightsLength';"), "tq 缺 generates 常量");
const tqAsserts = [...tq.matchAll(/static_assert\(kIntlAsciiCollationWeightsLength == 256\);/g)].length;
must(tqAsserts === 2, `tq 256 断言数 ${tqAsserts} != 2 (L1/L3 各一)`);

// ── 5. .cc 常量与签名锚 ──────────────────────────────────────────────────
const kMsMin = cc.match(/constexpr int64_t kMsMinInstant = (-?[\d.e]+);/);
const kMsMax = cc.match(/constexpr int64_t kMsMaxInstant = (-?[\d.e]+);/);
must(kMsMin && kMsMin[1] === "-8.64e15", `kMsMinInstant ${kMsMin && kMsMin[1]} != -8.64e15`);
must(kMsMax && kMsMax[1] === "8.64e15", `kMsMaxInstant ${kMsMax && kMsMax[1]} != 8.64e15`);
const bufSizes = [...cc.matchAll(/static constexpr int kBufferSize = 64;/g)].length;
must(bufSizes === 2, `cc kBufferSize=64 出现 ${bufSizes} != 2 (两处拷贝缓冲)`);
const intlMethods = [...cc.matchAll(/\bIntl::(\w+)\(/g)].map((m) => m[1]);
const intlUnique = [...new Set(intlMethods)];
must(intlUnique.length >= 25, `.cc Intl:: 方法数 ${intlUnique.length} < 25`);
for (const s of ["CanonicalizeLocaleList", "GetCanonicalLocales", "SupportedValuesOf", "CompareStrings", "SetNumberFormatDigitOptions", "ResolveLocale", "ToLanguageTag"]) {
  must(intlUnique.includes(s), `.cc 缺 Intl::${s}`);
}

const pascal = (s) => s.replace(/^k/, "").replace(/(^|_)(\w)/g, (_, a, b) => b.toUpperCase());
const rustEnum = (rsName, cppName, entries, repr) => {
  let out = `/// 原版 Intl::${cppName}。\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(${repr})]\npub enum ${rsName} {\n`;
  entries.forEach((e, i) => { out += `  /// ${e}\n  ${pascal(e)} = ${i},\n`; });
  return out + `}\n\n`;
};

let rust = banner("src/objects/intl-objects.h.js", "src/objects/intl-objects.{h,cc,tq}",
  "Intl 基础设施: 选项枚举/数字格式化参数结构/ASCII collation 权重表锚");
rust += `//! Intl 公共面 (ECMA402): 选项枚举 + 数字位数参数 + 区域工具。
//!
//! ICU 交互实现面登记跳过 (3344 行 .cc), 由 Developer 就地实现;
//! ASCII collation 权重表 (L1/L3 各 256 项) 数值面归 .cc 数据表就地迁移。

`;
rust += rustEnum("Notation", "Notation", enumEntries.Notation, "u8");
rust += rustEnum("CompactDisplay", "CompactDisplay", enumEntries.CompactDisplay, "u8");
rust += rustEnum("FormatRangeSource", "FormatRangeSource", enumEntries.FormatRangeSource, "u8");
rust += rustEnum("CompareStringsOptions", "CompareStringsOptions", enumEntries.CompareStringsOptions, "u8");
rust += rustEnum("RoundingPriority", "RoundingPriority", enumEntries.RoundingPriority, "u8");
rust += rustEnum("RoundingType", "RoundingType", enumEntries.RoundingType, "u8");
rust += rustEnum("RoundingMode", "RoundingMode", enumEntries.RoundingMode, "u8");
rust += rustEnum("TrailingZeroDisplay", "TrailingZeroDisplay", enumEntries.TrailingZeroDisplay, "u8");
rust += rustEnum("MatcherOption", "MatcherOption", enumEntries.MatcherOption, "u8");
rust += `// ── BoundFunctionContextSlot (上下文槽, 自 MIN_CONTEXT_SLOTS=2 起) ──
pub const K_BOUND_FUNCTION_CONTEXT_SLOT: u32 = 2;
pub const K_BOUND_FUNCTION_CONTEXT_LENGTH: u32 = 4;

// ── NumberFormatDigitOptions (原版 struct, 9 字段) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberFormatDigitOptions {
  /// minimum_integer_digits
  pub minimum_integer_digits: i32,
  /// minimum_fraction_digits
  pub minimum_fraction_digits: i32,
  /// maximum_fraction_digits
  pub maximum_fraction_digits: i32,
  /// minimum_significant_digits
  pub minimum_significant_digits: i32,
  /// maximum_significant_digits
  pub maximum_significant_digits: i32,
  /// rounding_priority
  pub rounding_priority: RoundingPriority,
  /// rounding_type
  pub rounding_type: RoundingType,
  /// rounding_increment
  pub rounding_increment: i32,
  /// rounding_mode
  pub rounding_mode: RoundingMode,
  /// trailing_zero_display
  pub trailing_zero_display: TrailingZeroDisplay,
}

// ── kCaseMappingSpecialLocales (az/el/lt/tr, 大小写特殊映射) ──
pub const K_CASE_MAPPING_SPECIAL_LOCALES: [&str; ${locales.length}] = [
`;
for (const l of locales) rust += `  "${l}",\n`;
rust += `];

// ── ASCII collation 权重表锚 (h 声明 / cc 落值 / tq 断言, 三方对数) ──
/// ASCII collation 权重表长度 (L1/L3 各 256 槽: 显式 128 + 隐式零 128; tq static_assert ×2)。
pub const K_ASCII_COLLATION_WEIGHTS_LENGTH: usize = 256;
/// L1/L3 表显式初始化项数 (仅 ASCII 前半 0..127; 后半聚合初始化隐式补零)。
pub const K_ASCII_COLLATION_WEIGHTS_EXPLICIT_ENTRIES: usize = ${l1Count};

// ── .cc 时间常数 (Temporal/DateTime 用) ──
/// 毫秒时间下界 (原版 kMsMinInstant = -8.64e15)。
pub const K_MS_MIN_INSTANT: i64 = -8_640_000_000_000_000;
/// 毫秒时间上界 (原版 kMsMaxInstant = 8.64e15)。
pub const K_MS_MAX_INSTANT: i64 = 8_640_000_000_000_000;

// ── .cc Intl:: 方法签名骨架 (${intlUnique.length} 个唯一; ICU 实现就地修订) ──
`;
const implAnchors = ["canonicalize_locale_list", "get_canonical_locales", "supported_values_of", "compare_strings",
  "set_number_format_digit_options", "resolve_locale", "to_language_tag"];
for (const a of implAnchors) {
  rust += `/// 原版 Intl::${a.replace(/(^|_)(\w)/g, (_, p, c) => c.toUpperCase())} (intl-objects.cc)。
pub fn ${a}() {
  todo!("就地修订: intl-objects.cc")
}

`;
}
rust += `// ── 提取不变式断言 ──
const _: () = {
  assert!(Notation::STANDARD as u8 == 0);
  assert!(Notation::COMPACT as u8 == 3);
  assert!(RoundingMode::kHalfEven as u8 == 8);
  assert!(K_BOUND_FUNCTION_CONTEXT_SLOT == 2);
  assert!(K_BOUND_FUNCTION_CONTEXT_LENGTH == 4);
  assert!(K_CASE_MAPPING_SPECIAL_LOCALES.len() == 4);
  assert!(K_CASE_MAPPING_SPECIAL_LOCALES[0] == "az");
  assert!(K_ASCII_COLLATION_WEIGHTS_LENGTH == 256);
  assert!(K_MS_MIN_INSTANT == -8_640_000_000_000_000);
  assert!(K_MS_MAX_INSTANT == 8_640_000_000_000_000);
  assert!(-K_MS_MIN_INSTANT == K_MS_MAX_INSTANT);
};
`;

const p = writeSrc("v8-objects-js-async", "intl_objects.rs", rust);
console.log(`intl-objects.h.js: ${p} — 枚举 10 个 (${Object.values(enumEntries).map((e) => e.length).join("+")}) + 结构 ${ndoFields.length} 字段 + 特殊 locale ${locales.length} + 权重表 ${l1Count} 三方对数 + .cc 唯一方法 ${intlUnique.length}, 断言 ${checks} 项全过`);
