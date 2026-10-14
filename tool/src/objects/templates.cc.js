#!/usr/bin/env bun
// tool/src/objects/templates.cc.js — 覆盖 v8/src/objects/templates.cc
// (差集: h/-inl.h/tq 归既有 templates.h.js 席)
// → v8-objects-js-func/src/templates_cc.rs.skel
// 机械提取: TemplateInfo/FunctionTemplateInfo/ObjectTemplateInfo/DictionaryTemplateInfo
//   cc 函数清单 + 实例化缓存/序列号语义锚。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/templates.cc");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── 1. 函数清单 (类限定) ───────────────────────────────────────────────
const classDefs = [...new Set([...cc.matchAll(
  /^(?:[A-Za-z_][\w:<>,*&]*[ \t]+)?(FunctionTemplateInfo|ObjectTemplateInfo|DictionaryTemplateInfo|TemplateInfo)::(\w+)\(/gm,
)].map((m) => `${m[1]}::${m[2]}`))];
must(classDefs.length >= 15, `cc 类方法应 ≥15, 实得 ${classDefs.length}`);
for (const need of ["FunctionTemplateInfo::HasInstanceType", "FunctionTemplateInfo::GetOrCreateSharedFunctionInfo",
  "FunctionTemplateInfo::IsTemplateFor", "FunctionTemplateInfo::IsLeafTemplateForApiObject",
  "FunctionTemplateInfo::SealAndPrepareForPromotionToReadOnly",
  "FunctionTemplateInfo::AllocateFunctionTemplateRareData",
  "FunctionTemplateInfo::TryGetCachedPropertyName", "FunctionTemplateInfo::GetCFunctionsCount",
  "FunctionTemplateInfo::GetCFunction", "ObjectTemplateInfo::SealAndPrepareForPromotionToReadOnly",
  "DictionaryTemplateInfo::Create", "DictionaryTemplateInfo::NewInstance",
  "TemplateInfo::ProbeInstantiationsCache", "TemplateInfo::CacheTemplateInstantiation",
  "TemplateInfo::UncacheTemplateInstantiation", "FunctionTemplateInfo::BreakAtEntry",
  "TemplateInfo::EnsureHasSerialNumber"])
  must(classDefs.includes(need), `cc 缺 ${need}`);

// ── 2. 语义锚 ─────────────────────────────────────────────────────────
// HasInstanceType: instance_type != kNoJSApiObjectType
must(cc.includes("bool FunctionTemplateInfo::HasInstanceType() {") &&
  cc.includes("return instance_type() != kNoJSApiObjectType;"), "HasInstanceType 语义锚缺失");
// IsTemplateFor: map 构造函数 → 沿 parent 模板链上行
must(cc.includes("bool FunctionTemplateInfo::IsTemplateFor(Tagged<Map> map) const {"),
  "IsTemplateFor 定义锚缺失");
must(cc.includes("Tagged<Object> cons_obj = map->GetConstructor();"), "GetConstructor 锚缺失");
must(cc.includes("fun->shared()->api_func_data();"), "api_func_data 锚缺失");
must(cc.includes("type = Cast<FunctionTemplateInfo>(type)->GetParentTemplate();"),
  "parent 链上行锚缺失");
// TryGetCachedPropertyName: 缓存属性名走 symbol 化
must(cc.includes("std::optional<Tagged<Name>> FunctionTemplateInfo::TryGetCachedPropertyName("),
  "TryGetCachedPropertyName 定义锚缺失");
// CFunction 表
must(cc.includes("uint32_t FunctionTemplateInfo::GetCFunctionsCount() const {"),
  "GetCFunctionsCount 定义锚缺失");
must(cc.includes("CFunctionWithSignature FunctionTemplateInfo::GetCFunction("),
  "GetCFunction 定义锚缺失");
// 实例化缓存三件套
must(cc.includes("MaybeHandle<Object> TemplateInfo::ProbeInstantiationsCache("),
  "ProbeInstantiationsCache 定义锚缺失");
must(cc.includes("void TemplateInfo::CacheTemplateInstantiation("),
  "CacheTemplateInstantiation 定义锚缺失");
must(cc.includes("void TemplateInfo::UncacheTemplateInstantiation("),
  "UncacheTemplateInstantiation 定义锚缺失");
// 序列号
must(cc.includes("uint32_t TemplateInfo::EnsureHasSerialNumber(Isolate* isolate) {"),
  "EnsureHasSerialNumber 定义锚缺失");
// SealAndPrepareForPromotionToReadOnly 双类
must(cc.includes("void FunctionTemplateInfo::SealAndPrepareForPromotionToReadOnly("),
  "FunctionTemplateInfo Seal 定义锚缺失");
must(cc.includes("void ObjectTemplateInfo::SealAndPrepareForPromotionToReadOnly("),
  "ObjectTemplateInfo Seal 定义锚缺失");
// DictionaryTemplateInfo::Create / NewInstance
must(cc.includes("DirectHandle<DictionaryTemplateInfo> DictionaryTemplateInfo::Create("),
  "DictionaryTemplateInfo::Create 定义锚缺失");
must(cc.includes("DirectHandle<JSObject> DictionaryTemplateInfo::NewInstance("),
  "DictionaryTemplateInfo::NewInstance 定义锚缺失");

// ── 3. 自由函数 / 匿名命名空间辅助 ────────────────────────────────────
must(cc.includes("namespace {") && cc.includes("}  // namespace"), "匿名 namespace 锚缺失");

// ── 4. 生成 ────────────────────────────────────────────────────────────
const body = `${banner("src/objects/templates.cc.js", "src/objects/templates.cc", "差集 (h/-inl.h/tq 归 templates.h.js 席): 函数清单 + 缓存/序列号语义骨架")}
//! 差集骨架: TemplateInfo 族 cc 语义面。
//! 依赖 (Isolate/NativeContext/ Factory) 未落地, 语义留 Developer 就地修订。

// —— cc 函数清单 (${classDefs.length} 类方法) ——
/// v8: src/objects/templates.cc
pub const TEMPLATES_CC_METHODS: &[&str] = &[
${classDefs.map((d) => `  "${d}",`).join("\n")}
];

// —— 语义规则面 ——
/// v8: src/objects/templates.cc
pub const TEMPLATES_CC_SEMANTICS: &[&str] = &[
  "HasInstanceType: instance_type() != kNoJSApiObjectType",
  "GetOrCreateSharedFunctionInfo: 模板 → SFI (懒建, 缓存于模板 rare data)",
  "IsTemplateFor: map.GetConstructor → JSFunction(shared.api_func_data) 或 FunctionTemplateInfo; 沿 GetParentTemplate 继承链上行查找本模板 (IsLeafTemplateForApiObject 为免链快路径)",
  "SealAndPrepareForPromotionToReadOnly: FunctionTemplateInfo/ObjectTemplateInfo 双实现 — 密封模板并预备晋升只读堆",
  "AllocateFunctionTemplateRareData: 懒分配 rare data 区",
  "TryGetCachedPropertyName: 缓存属性名 optional<Name> (合法时 symbol 化加速)",
  "GetCFunctionsCount/GetCFunction: fast API C 函数表计数与取项",
  "ProbeInstantiationsCache / CacheTemplateInstantiation / UncacheTemplateInstantiation: 模板实例化三态缓存 (probe 命中复用; cache 登记; uncache 撤销)",
  "BreakAtEntry: 调试断点入口标志",
  "EnsureHasSerialNumber: 懒取模板序列号 (0 表示未分配)",
  "DictionaryTemplateInfo::Create/NewInstance: 字典模板创建与实例化",
];
`;

const p = writeSrc("v8-objects-js-func", "templates_cc.rs.skel", body);
console.log(
  `templates.cc.js: ${p} — 类方法 ${classDefs.length}、语义规则 11 条、断言 ${checks} 项通过`,
);
