#!/usr/bin/env bun
// tool/src/objects/js-number-format.h.js — JSNumberFormat 族骨架 (3 槽 + 7 枚举 + 方法对数)
// 组→文件映射: js-number-format.h + js-number-format-inl.h + js-number-format.cc + js-number-format.tq
// 产物: v8-objects-js-core/src/js_number_format.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-number-format.h");
const inl = readV8("src/objects/js-number-format-inl.h");
const cc = readV8("src/objects/js-number-format.cc");
const tq = readV8("src/objects/js-number-format.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. 类签名 + 字段槽 (JSObject 头 3 槽后) ────────────────────────────
must(h.includes("V8_OBJECT class JSNumberFormat : public JSObject {"), "类签名");
const fields = [...h.slice(h.indexOf("V8_OBJECT class JSNumberFormat"), h.indexOf("} V8_OBJECT_END;"))
  .matchAll(/TaggedMember<(.+?)> (\w+);/g)].map((m) => [m[2], m[1]]);
eq(fields.length, 3, "TaggedMember 字段数");
must(fields[0][0] === "locale_" && fields[0][1] === "String", `字段0 ${fields[0]}`);
must(fields[1][0] === "icu_number_formatter_" && fields[1][1] === "Foreign", `字段1 ${fields[1]}`);
must(fields[2][0] === "bound_format_" && fields[2][1] === "UnionOf<JSFunction, Undefined>", `字段2 ${fields[2]}`);
const K_LOCALE = 24, K_ICU = 32, K_BOUND = 40, K_SIZE = 48;
must(h.includes("inline constexpr int JSNumberFormat::kHeaderSize = sizeof(JSNumberFormat);"), "kHeaderSize 外置定义");

// ── 2. ShowTrailingZeros 枚举 (h) ───────────────────────────────────────
must(h.includes("enum class ShowTrailingZeros { kShow, kHide };"), "ShowTrailingZeros 枚举");

// ── 3. cc 枚举全集 (选项字符串语义) ────────────────────────────────────
const enums = [...cc.matchAll(/enum class (\w+) \{([^}]*)\}/g)].map((m) => [m[1], m[2].split(",").map((s) => s.trim()).filter(Boolean)]);
eq(enums.length, 6, "cc enum class 总数");
const expectEnums = {
  Style: 4, CurrencyDisplay: 4, CurrencySign: 2, UnitDisplay: 3, SignDisplay: 5, UseGrouping: 4,
};
for (const [name, n] of Object.entries(expectEnums)) {
  const got = enums.find((e) => e[0] === name);
  must(!!got, `cc 枚举 ${name} 缺失`);
  eq(got[1].length, n, `cc 枚举 ${name} 值数`);
}
must(enums.find((e) => e[0] === "Style")[1].join(",") === "DECIMAL,PERCENT,CURRENCY,UNIT", "Style 值序");
must(enums.find((e) => e[0] === "SignDisplay")[1].join(",") === "AUTO,ALWAYS,NEVER,EXCEPT_ZERO,NEGATIVE", "SignDisplay 值序");
must(enums.find((e) => e[0] === "UseGrouping")[1].join(",") === "OFF,MIN2,AUTO,ALWAYS", "UseGrouping 值序");

// ── 4. 静态方法对数 ─────────────────────────────────────────────────────
for (const sig of ["MaybeDirectHandle<JSNumberFormat> JSNumberFormat::New(", "JSNumberFormat::UnwrapNumberFormat(",
  "JSNumberFormat::NumberFormatFunction(", "DirectHandle<JSObject> JSNumberFormat::ResolvedOptions(",
  "JSNumberFormat::FormatToParts(", "JSNumberFormat::FormatNumericRangeToParts(",
  "JSNumberFormat::FormatNumericRange(", "JSNumberFormat::FormatNumeric("])
  must(cc.includes(sig), `cc 方法 ${sig.slice(0, 44)}`);
eq((cc.match(/JSNumberFormat::/g) || []).length, 22, "cc JSNumberFormat:: 限定名计数");
must(cc.includes("MinimumIntegerDigitsFromSkeleton") && cc.includes("FractionDigitsFromSkeleton") &&
  cc.includes("SignificantDigitsFromSkeleton"), "skeleton 三兄弟 (JSPluralRules 共享)");

// ── 5. inl 访问器 + tq 交叉 ─────────────────────────────────────────────
must(inl.includes("Tagged<String> JSNumberFormat::locale() const { return locale_.load(); }"), "inl locale");
must(inl.includes("JSNumberFormat::bound_format() const {"), "inl bound_format");
must(tq.includes("extern class JSNumberFormat extends JSObject {"), "tq 类");
must(tq.includes("bound_format: JSFunction|Undefined;"), "tq bound_format");

// ── 6. IntlMathematicalValue (h 栈上辅助类) ────────────────────────────
must(h.includes("class V8_NODISCARD IntlMathematicalValue {"), "IntlMathematicalValue 类");
must(h.includes("double approx_;"), "approx_ 字段");
must(h.includes("Handle<Object> value_;  // Number, BigInt or String"), "value_ 字段");

// ── 7. 骨架产物 ──────────────────────────────────────────────────────────
const enumDefs = enums.map(([name, vals]) =>
  `pub enum ${name} { ${vals.join(", ")} } // ${vals.length} 值`).join("\n");
const body = `${banner("src/objects/js-number-format.h.js", "src/objects/js-number-format.h + -inl.h + .cc + .tq", "JSNumberFormat 槽位/选项枚举差集骨架")}
//! JSNumberFormat: Intl.NumberFormat 对象; icu_number_formatter_ 以 Foreign 槽
//! 承载 Managed<icu::number::LocalizedNumberFormatter>。

/// v8: src/objects/js-number-format.h JSNumberFormat::locale_
pub const K_LOCALE_OFFSET: usize = ${K_LOCALE};
/// v8: src/objects/js-number-format.h JSNumberFormat::icu_number_formatter_
pub const K_ICU_NUMBER_FORMATTER_OFFSET: usize = ${K_ICU};
/// v8: src/objects/js-number-format.h JSNumberFormat::bound_format_
pub const K_BOUND_FORMAT_OFFSET: usize = ${K_BOUND};
/// v8: src/objects/js-number-format.h JSNumberFormat::kHeaderSize
pub const K_HEADER_SIZE: usize = ${K_SIZE};

// cc 选项枚举 (字符串语义, js-number-format.cc:48-96):
${enumDefs}
// h: enum class ShowTrailingZeros { kShow, kHide } (枚举值 k 前缀映射为 Rust K_*)

// 方法映射 (cc, 全依赖 ICU/Isolate → 语义推迟): New / UnwrapNumberFormat /
//   NumberFormatFunction / ResolvedOptions / FormatToParts / FormatNumericRange(ToParts) /
//   FormatNumeric / GetAvailableLocales / skeleton 解析三兄弟 + Rounding 四查询
// IntlMathematicalValue: 栈上值 (approx_: f64 + value_: Handle) — Number/BigInt/String 三态
`;

const p = writeSrc("v8-objects-js-core", "js_number_format.rs.skel", body);
console.log(
  `js-number-format.h.js: ${p} — 槽 3/3 (${K_LOCALE}/${K_ICU}/${K_BOUND}, size ${K_SIZE})、cc 枚举 ${enums.length}/6 共 ${Object.values(expectEnums).reduce((a, b) => a + b)} 值、cc 方法锚 8/8、限定名 22、断言 ${checks} 项全部通过`,
);
