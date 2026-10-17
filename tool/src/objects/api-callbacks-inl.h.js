#!/usr/bin/env bun
// tool/src/objects/api-callbacks-inl.h.js — API 回调内联访问器生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/api-callbacks-inl.h → v8-objects-core/src/api_callbacks_inl.rs
//   (api-callbacks.{h,tq} 的位域/结构面归 tool/src/objects/api-callbacks.h.js → v8-objects-rest/api_callbacks.rs,
//    本脚本只登记 -inl 侧: 回调访问器宏展开 / 位段访问器 / 序列化辅助)
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/api-callbacks-inl.h");
const h = readV8("src/objects/api-callbacks.h");

// —— 锚点断言: Internals 偏移契约 ——
if (!/Internals::kCallbackInfoDataOffset ==\s*$/.test("") ) { /* 占位, 实际断言在下方 */ }
const dataOffsetAsserts = [...inl.matchAll(/static_assert\(Internals::kCallbackInfoDataOffset ==\s*offsetof\((\w+), data_\)\)/g)].map((m) => m[1]);
if (dataOffsetAsserts.join() !== "AccessorInfo,InterceptorInfo")
  throw new Error(`kCallbackInfoDataOffset 断言对象应为 AccessorInfo,InterceptorInfo, 实得 ${dataOffsetAsserts}`);

// —— LAZY_* 回调访问器宏展开: 16 项 (named 7 + indexed 9) ——
// 形态: LAZY_...CHECKED2(Class, name, Address, offsetof(Class, field_), Tag, cond_load, cond_store)
const lazy = [...inl.matchAll(
  /LAZY_\w+\(\s*(\w+),\s*(\w+),\s*Address,\s*offsetof\(\1, (\w+)_\),\s*(\w+)/g,
)].map((m) => ({ cls: m[1], name: m[2], field: m[3], tag: m[4] }));
if (lazy.length !== 16) throw new Error(`LAZY 回调访问器应 16, 实得 ${lazy.length}`);
const namedCount = lazy.filter((e) => e.name.startsWith("named_")).length;
const indexedCount = lazy.filter((e) => e.name.startsWith("indexed_")).length;
if (namedCount !== 7 || indexedCount !== 9)
  throw new Error(`named/indexed 应 7/9, 实得 ${namedCount}/${indexedCount}`);
// tag 锚点抽查
const tagOf = (n) => lazy.find((e) => e.name === n)?.tag;
if (tagOf("named_getter") !== "kApiNamedPropertyGetterCallbackTag") throw new Error("named_getter tag 漂移");
if (tagOf("indexed_definer") !== "kApiIndexedPropertyDefinerCallbackTag") throw new Error("indexed_definer tag 漂移");
if (tagOf("indexed_index_of") !== "kApiIndexedPropertyIndexOfCallbackTag") throw new Error("indexed_index_of tag 漂移");
if (tagOf("indexed_iterable_to_list") !== "kApiIndexedPropertyIterableToListCallbackTag") throw new Error("indexed_iterable_to_list tag 漂移");

// —— 非 LAZY 的 AccessorInfo getter/setter (REDIRECTED/EXTERNAL 各 1 对) ——
const plainCbs = [...inl.matchAll(/(?:REDIRECTED_CALLBACK_ACCESSORS_MAYBE_READ_ONLY_HOST|EXTERNAL_POINTER_ACCESSORS_MAYBE_READ_ONLY_HOST)\(\s*AccessorInfo,\s*(\w+)/g)].map((m) => m[1]);
if (plainCbs.join() !== "getter,setter") throw new Error(`AccessorInfo plain 回调应 getter,setter, 实得 ${plainCbs}`);

// —— INTERCEPTOR_INFO_HAS_GETTER X-macro ×7 + 手写 ×2 ——
const hasGetters = [...inl.matchAll(/^INTERCEPTOR_INFO_HAS_GETTER\((\w+)\)/gm)].map((m) => m[1]);
if (hasGetters.join() !== "getter,setter,query,descriptor,deleter,definer,enumerator")
  throw new Error(`INTERCEPTOR_INFO_HAS_GETTER 应 7 项序不变, 实得 ${hasGetters}`);

// —— InterceptorInfo 标志位访问器: BOOL_ACCESSORS ×5 + BOOL_GETTER ×1 ——
const boolAcc = [...inl.matchAll(/^BOOL_ACCESSORS\(InterceptorInfo, flags, (\w+)/gm)].map((m) => m[1]);
const boolGet = [...inl.matchAll(/^BOOL_GETTER\(InterceptorInfo, flags, (\w+)/gm)].map((m) => m[1]);
if (boolAcc.join() !== "can_intercept_symbols,non_masking,has_no_side_effect,has_new_callbacks_signature,has_dont_delete_property")
  throw new Error(`InterceptorInfo BOOL_ACCESSORS 序漂移: ${boolAcc}`);
if (boolGet.join() !== "is_named") throw new Error(`BOOL_GETTER 应 is_named, 实得 ${boolGet}`);

// —— AccessorInfo 位段访问器 BIT_FIELD_ACCESSORS ×4 + setter_side_effect_type ——
const bitAcc = [...inl.matchAll(/^BIT_FIELD_ACCESSORS\(AccessorInfo, flags, (\w+)/gm)].map((m) => m[1]);
if (bitAcc.join() !== "replace_on_access,is_sloppy,getter_side_effect_type,initial_property_attributes")
  throw new Error(`AccessorInfo BIT_FIELD_ACCESSORS 序漂移: ${bitAcc}`);
if (!/SideEffectType AccessorInfo::setter_side_effect_type/.test(inl)) throw new Error("setter_side_effect_type 缺失");

// —— 位段在头部定义 (跨文件核对: shift/width 由 api-callbacks.h BitField 链得出) ——
for (const need of ["ReplaceOnAccessBit", "IsSloppyBit", "GetterSideEffectTypeBits", "SetterSideEffectTypeBits", "InitialAttributesBits", "NamedBit", "CanInterceptSymbolsBit", "NonMaskingBit", "HasNoSideEffectBit", "HasNewCallbacksSignatureBit", "HasDontDeletePropertyBit"])
  if (!new RegExp(`using ${need}\\b|${need}\\s*=|struct ${need}`).test(h) && !h.includes(need))
    throw new Error(`api-callbacks.h 缺位段 ${need}`);

const body = `${banner("src/objects/api-callbacks-inl.h.js", "src/objects/api-callbacks-inl.h (+ api-callbacks.h 位段核对)", "回调访问器宏展开登记; 位域本体在 v8-objects-rest/api_callbacks.rs")}
//! API 回调内联访问器 (原版 api-callbacks-inl.h)
//!
//! 原版布局契约: \`Internals::kCallbackInfoDataOffset\` 与 AccessorInfo::data_ /
//! InterceptorInfo::data_ 的 offsetof 全等 (Api 免类型判断直读 data 槽).

/// 偏移契约: data_ 槽在两个类上偏移一致 (原版 static_assert ×2)
pub const CALLBACK_INFO_DATA_OFFSET_OWNERS: [&str; 2] = ["AccessorInfo", "InterceptorInfo"];

/// InterceptorInfo 惰性回调访问器清单 (LAZY_*_CHECKED2 展开, ${lazy.length} 项:
/// named ${namedCount} + indexed ${indexedCount}; 原版按 named/indexed 复用同一物理槽)
pub const INTERCEPTOR_CALLBACKS: [(&str, &str, &str, bool); ${lazy.length}] = [
${lazy.map((e) => `  ("${e.name}", "${e.field}_", "${e.tag}", ${e.name.startsWith("named_")}),`).join("\n")}
];

/// InterceptorInfo::has_* 派生谓词 (INTERCEPTOR_INFO_HAS_GETTER X-macro 展开, ${hasGetters.length} 项)
pub const INTERCEPTOR_HAS_PREDICATES: [&str; ${hasGetters.length}] = [${hasGetters.map((n) => `"has_${n}"`).join(", ")}];

/// InterceptorInfo 标志位 (BOOL_ACCESSORS/BOOL_GETTER 展开, bool 位段 ${boolAcc.length + boolGet.length} 个)
pub const INTERCEPTOR_FLAG_BITS: [&str; ${boolAcc.length + boolGet.length}] = [${[...boolAcc, ...boolGet].map((n) => `"${n}"`).join(", ")}];

/// AccessorInfo 位段访问器 (BIT_FIELD_ACCESSORS 展开 + 手写 setter_side_effect_type)
pub const ACCESSOR_INFO_BIT_FIELDS: [&str; ${bitAcc.length + 1}] = [${[...bitAcc, "setter_side_effect_type"].map((n) => `"${n}"`).join(", ")}];

/// AccessorInfo 普通回调 (REDIRECTED/EXTERNAL 展开各 1 对)
pub const ACCESSOR_INFO_CALLBACKS: [&str; 2] = ["getter", "setter"];

/// AccessorInfo::has_getter — 惰性 getter 槽非空 (原版 getter(isolate) != kNullAddress)
pub fn accessor_info_has_getter(/* isolate, this */) -> bool {
  todo!("就地修订: getter(isolate) != kNullAddress")
}

/// AccessorInfo::has_setter — 惰性 setter 槽非空
pub fn accessor_info_has_setter(/* isolate, this */) -> bool {
  todo!("就地修订: setter(isolate) != kNullAddress")
}

/// AccessorInfo::setter_side_effect_type — 禁止 kHasNoSideEffect (原版 CHECK_NE)
pub fn accessor_info_setter_side_effect_type(/* value: SideEffectType */) {
  todo!("就地修订: CHECK_NE(value, kHasNoSideEffect) 后 SetterSideEffectTypeBits::update")
}

/// 序列化前摘除回调重定向 (仅模拟器构建有效, 原版 CHECK(USE_SIMULATOR_BOOL))
pub fn remove_callback_redirection_for_serialization(/* class, isolate */) {
  todo!("就地修订: remove_getter_redirection / 仅 named_getter 分支")
}

/// 反序列化后恢复回调重定向
pub fn restore_callback_redirection_after_deserialization(/* class, isolate */) {
  todo!("就地修订: init_getter_redirection / 仅 named_getter 分支")
}

/// 清零可选填充 (保证快照确定性)
pub fn clear_padding(/* this, kOptionalPaddingOffset */) {
  todo!("就地修订: FIELD_SIZE(kOptionalPaddingOffset) 为 0 时直接返回")
}

/// 取 Api 回调的持有者: JSGlobalObject → 对应 JSGlobalProxy (原版 GetHolderForApi)
pub fn get_holder_for_api(/* holder: Tagged<JSObject> */) /* -> Tagged<JSObject> */ {
  todo!("就地修订: TryCast<JSGlobalObject> 则换 global_proxy_for_api()")
}
`;

const p = writeSrc("v8-objects-core", "api_callbacks_inl.rs", body);
console.log(`api-callbacks-inl.h.js: 已生成 ${p} (LAZY ${lazy.length}, BOOL ${boolAcc.length + boolGet.length}, BIT ${bitAcc.length + 1}, has_* ${hasGetters.length})`);
