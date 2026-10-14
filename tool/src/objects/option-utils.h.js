#!/usr/bin/env bun
// tool/src/objects/option-utils.h.js — 覆盖 v8/src/objects/option-utils.{h,cc}
// → v8-objects-js-async/src/option_utils.rs
// 机械提取: ECMA402 GetOption 工具族签名 (h 声明 7 + 模板 2 + cc 实现 7) 骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/option-utils.h");
const cc = readV8("src/objects/option-utils.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[option-utils.h.js] ${m}`); };

// ── 1. .h 声明面 (7 个具名函数) ──────────────────────────────────────────
const hDecls = [
  ["GetOptionsObject", "MaybeDirectHandle<JSReceiver>"],
  ["CoerceOptionsToObject", "MaybeDirectHandle<JSReceiver>"],
  ["GetStringOption", "Maybe<bool>"],
  ["GetStringOrBooleanOption", "static Maybe<T>"], // 模板 (h 内联定义)
  ["GetBoolOption", "Maybe<bool>"],
  ["GetNumberOption", "Maybe<int>"],
  ["GetNumberOptionAsDouble", "Maybe<double>"],
  ["DefaultNumberOption", "Maybe<int>"],
];
for (const [name, ret] of hDecls) {
  must(new RegExp(`${name}\\(`).test(h) && h.includes(name), `h 缺 ${name}`);
  must(cc.includes(name) || name === "GetStringOrBooleanOption", `cc 缺 ${name} 对应面`);
}
// GetStringOption 双重载: 裸字符串版 (h 39) + 枚举匹配模板版 (h 61)
const gsCount = (h.match(/Maybe<bool> GetStringOption\(|static Maybe<T> GetStringOption\(/g) || []).length;
must(gsCount === 2, `GetStringOption 重载数 ${gsCount} != 2`);
must(h.includes("template <typename T>\nV8_WARN_UNUSED_RESULT static Maybe<T> GetStringOption("), "h 缺 GetStringOption<T> 模板");
must(h.includes("template <typename T>\nV8_WARN_UNUSED_RESULT static Maybe<T> GetStringOrBooleanOption("), "h 缺 GetStringOrBooleanOption<T> 模板");

// ── 2. .h 语义锚 (ECMA402 条款) ──────────────────────────────────────────
must(h.includes("https://tc39.es/ecma262/#sec-getoptionsobject"), "h 缺 GetOptionsObject 条款注");
must(h.includes("https://tc39.es/ecma402/#sec-coerceoptionstoobject"), "h 缺 CoerceOptionsToObject 条款注");
must(h.includes("DCHECK_EQ(str_values.size(), enum_values.size());"), "h 缺 str/enum 等长 DCHECK");
must(h.includes("MessageTemplate::kValueOutOfRange"), "h 缺 kValueOutOfRange 抛错锚");

// ── 3. .cc 实现面 (7 个定义, 源码序) ─────────────────────────────────────
const ccDefs = [...cc.matchAll(/^(?:V8_WARN_UNUSED_RESULT\s+)?(?:MaybeDirectHandle<JSReceiver>|Maybe<bool>|Maybe<int>|Maybe<double>)\s+(\w+)\(/gm)].map((m) => m[1]);
must(ccDefs.length === 7, `cc 实现数 ${ccDefs.length} != 7`);
must(ccDefs.join(",") === "GetOptionsObject,CoerceOptionsToObject,GetStringOption,GetBoolOption,DefaultNumberOption,GetNumberOption,GetNumberOptionAsDouble",
  `cc 实现序漂移: ${ccDefs.join(",")}`);
// GetStringOption 枚举匹配版在 .cc 的部分? 模板在 h; cc 只有裸版
must(cc.includes("Maybe<bool> GetStringOption(Isolate* isolate, DirectHandle<JSReceiver> options,"), "cc 缺 GetStringOption 裸版实现");

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

let rust = banner("src/objects/option-utils.h.js", "src/objects/option-utils.{h,cc}",
  "ECMA402 GetOption 工具族: options 对象读取/校验/回退骨架");
rust += `//! option-utils: Intl/Temporal 共用的 options 对象工具族 (ECMA402 9.2.10)。
//!
//! 读取 → ToString/ToBoolean → 枚举匹配 (str_values ↔ enum_values 等长)
//! → 未命中抛 RangeError(kValueOutOfRange) → 回退默认值。

// ── 函数签名骨架 (cc 实现序 + h 模板 2; 语义就地修订) ──
`;
for (const m of ccDefs) {
  rust += `/// 原版 ${m} (option-utils.cc; h 声明)。
pub fn ${snake(m)}() {
  todo!("就地修订: option-utils.cc ${m}")
}

`;
}
rust += `/// 原版 GetStringOption<T> 模板 (option-utils.h: 字符串 → 枚举匹配, RangeError 回退)。
pub fn get_string_option_enum() {
  todo!("就地修订: option-utils.h GetStringOption<T> 模板 (h 内联)")
}

/// 原版 GetStringOrBooleanOption<T> 模板 (option-utils.h: undefined 回退 / true→true_value / false→false_value)。
pub fn get_string_or_boolean_option() {
  todo!("就地修订: option-utils.h GetStringOrBooleanOption<T> 模板 (h 内联)")
}

// ── 提取不变式断言 ──
const _: () = {
  // 工具族 9 面: cc 实现 7 + h 模板 2
  assert!(K_OPTION_UTILS_FN_COUNT == 9);
};

/// 工具族函数面总数 (cc 7 + h 模板 2)。
pub const K_OPTION_UTILS_FN_COUNT: usize = ${ccDefs.length + 2};
`;

const p = writeSrc("v8-objects-js-async", "option_utils.rs", rust);
console.log(`option-utils.h.js: ${p} — h 声明 ${hDecls.length} 面 (GetStringOption 双重载) + cc 实现 ${ccDefs.length} + 模板 2, 断言 ${checks} 项全过`);
