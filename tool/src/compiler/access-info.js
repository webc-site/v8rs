#!/usr/bin/env bun
// v8/src/compiler/access-info.h + access-info.cc
// → v8-compiler-passes/src/access_info.rs.skel
//
// 规则面: PropertyAccessInfo::Kind 枚举 (13 变体) 逐项提取;
// AccessInfoFactory::KeyType 枚举 (2 变体); ElementAccessInfo /
// PropertyAccessInfo / AccessInfoFactory 三类的公有方法签名 1:1 展开
// (grep 计数对账), 语义面 (map 查找/依赖记录) 留 todo! + /// v8: 锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/access-info.h");
const cc = readV8("src/compiler/access-info.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class ElementAccessInfo final \{/, "ElementAccessInfo"],
  [h, /class PropertyAccessInfo final \{/, "PropertyAccessInfo"],
  [h, /enum Kind \{/, "PropertyAccessInfo::Kind"],
  [h, /kDictionaryProtoDataConstant,/, "Kind 变体"],
  [h, /class AccessInfoFactory final \{/, "AccessInfoFactory"],
  [h, /enum class KeyType \{ kString, kIndex \};/, "KeyType"],
  [h, /std::optional<ElementAccessInfo> ComputeElementAccessInfo\(/, "工厂方法面"],
  [cc, /PropertyAccessInfo AccessInfoFactory::ComputePropertyAccessInfo\(/, "cc 主实现"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— PropertyAccessInfo::Kind 变体序 ——
const kindBlock = h.match(/enum Kind \{([\s\S]*?)\n  \};/)[1];
const kindVariants = [...kindBlock.matchAll(/^\s*k(\w+),?\s*$/gm)].map((m) => m[1]);
if (kindVariants.length !== 12) throw new Error(`Kind ${kindVariants.length} != 12`);
if (kindVariants[0] !== "Invalid" || kindVariants.at(-1) !== "TypedArrayLength") {
  throw new Error(`Kind 首尾漂移: ${kindVariants}`);
}

// —— 方法签名计数 (三类的公有函数面, 逐字对账) ——
function count(re) {
  return [...h.matchAll(re)].length;
}
const paiCtors = count(/static PropertyAccessInfo \w+\(\n?[^;]*?\) ?;/g); // 静态工厂构造
const paiMethods = [...h.matchAll(/^  (?:bool|OptionalObjectRef|OptionalMapRef|OptionalJSObjectRef|FieldIndex|Type|MapRef|PropertyAccessInfo|ElementsKind|int|CellRef|Zone\*|OptionalHeapObjectRef) (\w+)\(/gm)].map((m) => m[1]);
const eaiMethods = ["ElementAccessInfo", "elements_kind", "lookup_start_object_maps", "transition_sources", "AddTransitionSource", "is_proxy_on_prototype", "string_keys", "accessor", "target"];
for (const n of eaiMethods) {
  if (!new RegExp(`\\b${n}\\b`).test(h)) throw new Error(`ElementAccessInfo 方法缺失: ${n}`);
}
const factoryMethods = [
  "ComputeElementAccessInfo", "ComputeElementAccessInfos", "ComputePropertyAccessInfo",
  "ComputeDictionaryProtoAccessInfo", "FinalizePropertyAccessInfos", "FinalizePropertyAccessInfosAsOne",
  "ConsolidateElementLoad", "LookupSpecialFieldAccessor", "LookupSpecialFieldAccessorInHolder",
  "LookupTransition", "ComputeDataFieldAccessInfo", "ComputeAccessorDescriptorAccessInfo",
  "MergePropertyAccessInfos", "TryLoadPropertyDetails",
  "ObjectMayHaveElements", "ObjectMayHaveOwnProperties", "ObjectIsSuitableProxyTarget",
];
const ccImpls = [...cc.matchAll(/AccessInfoFactory::(\w+)\(/g)].map((m) => m[1]);
const missing = factoryMethods.filter((m) => !ccImpls.includes(m));
if (missing.length) throw new Error(`cc 缺实现: ${missing}`);
// ComputeDictionaryDataFieldAccessInfo 声明在 .h, cc 以自由函数 TryCompute… 落地
if (!/^PropertyAccessInfo TryComputeDictionaryDataFieldAccessInfo\(/m.test(cc)) {
  throw new Error("cc 缺自由函数 TryComputeDictionaryDataFieldAccessInfo");
}
// PropertyAccessInfo 静态工厂 (h 声明 vs cc 定义)
const paiStatics = [...h.matchAll(/^  static PropertyAccessInfo (\w+)\(/gm)].map((m) => m[1]);
if (paiStatics.join() !== "NotFound,DataField,FastDataConstant,FastAccessorConstant,ModuleExport,StringLength,StringWrapperLength,TypedArrayLength,Invalid,DictionaryDataField,DictionaryProtoDataConstant,DictionaryProtoAccessorConstant") {
  throw new Error(`静态工厂漂移: ${paiStatics}`);
}

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
const kindRs = kindVariants.map((v) => `  /// k${v}\n  ${v[0].toUpperCase() + v.slice(1)},`).join("\n");

const body = `${banner("src/compiler/access-info.js", "src/compiler/access-info.h + src/compiler/access-info.cc", "ElementAccessInfo / PropertyAccessInfo(Kind 12) / AccessInfoFactory 签名骨架")}
//! 访问信息 — 属性/元素访问的编译期分析结果。
//!
//! 规则面: PropertyAccessInfo::Kind 13 变体, KeyType 2 变体, 三类方法签名。
//! 语义面 (推迟): map 合并 / 字段索引推导 / 依赖登记 在 .cc, 依赖
//! JSHeapBroker 对象层。

#![allow(dead_code, unused_imports)]

/// v8: src/compiler/access-info.h PropertyAccessInfo::Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyAccessKind {
${kindRs}
}

/// v8: src/compiler/access-info.h AccessInfoFactory::KeyType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
  /// kString
  String,
  /// kIndex
  Index,
}

/// v8: src/compiler/access-info.h ElementAccessInfo
/// 元素访问信息: elements_kind + 查找起点 map 集 (+ Proxy-on-Prototype 特例)
pub struct ElementAccessInfo {
  /// 原 elements_kind_
  pub elements_kind: crate::objects::ElementsKind,
  /// 原 string_keys_ — Proxy trap 期望 string 键
  pub string_keys: bool,
  /// 原 lookup_start_object_maps_
  pub lookup_start_object_maps: Vec<crate::heap_refs::MapRef>,
  /// 原 transition_sources_
  pub transition_sources: Vec<crate::heap_refs::MapRef>,
  /// 原 accessor_ / target_ — Proxy-on-Prototype 模式
  pub accessor: Option<crate::heap_refs::ObjectRef>,
  pub target: Option<crate::heap_refs::ObjectRef>,
}

impl ElementAccessInfo {
  /// v8: src/compiler/access-info.h ElementAccessInfo::AddTransitionSource
  pub fn add_transition_source(&mut self, map: crate::heap_refs::MapRef) {
    debug_assert_eq!(self.lookup_start_object_maps.len(), 1);
    self.transition_sources.push(map);
  }
  /// v8: src/compiler/access-info.h ElementAccessInfo::is_proxy_on_prototype
  pub fn is_proxy_on_prototype(&self) -> bool {
    self.accessor.is_some()
  }
  /// v8: src/compiler/access-info.h ElementAccessInfo::string_keys
  pub fn string_keys(&self) -> bool {
    self.string_keys
  }
}

/// v8: src/compiler/access-info.h PropertyAccessInfo
/// 属性访问信息: 13 种 Kind (数据字段/字典字段/访问器/模块导出/字符串长度…)
pub struct PropertyAccessInfo {
  /// 原 kind_
  pub kind: PropertyAccessKind,
  /// 原多个静态工厂构造字段 (field_index/field_type/transition_map/…) 推迟
  pub _semantic: (),
}

impl PropertyAccessInfo {
${paiStatics.map((s) => `  /// v8: src/compiler/access-info.h PropertyAccessInfo::${s}\n  pub fn ${snake(s)}(/* 参数签名见 .h: ${s} */) -> Self {\n    todo!("PropertyAccessInfo::${s}")\n  }`).join("\n")}
}

/// v8: src/compiler/access-info.h AccessInfoFactory
pub struct AccessInfoFactory {
  /// 原 broker_
  pub broker: *mut crate::broker::JSHeapBroker,
  /// 原 type_cache_
  pub type_cache: *const crate::type_cache::TypeCache,
  /// 原 zone_
  pub zone: *mut crate::zone::Zone,
}

impl AccessInfoFactory {
${factoryMethods.map((m) => `  /// v8: src/compiler/access-info.h AccessInfoFactory::${m}\n  /// v8: src/compiler/access-info.cc AccessInfoFactory::${m}\n  pub fn ${snake(m)}(&self /* 参数签名见 .h: ${m} */) {\n    todo!("AccessInfoFactory::${m}")\n  }`).join("\n")}
}
`;

const p = writeSrc("v8-compiler-passes", "access_info.rs.skel", body);
console.log(
  [
    `written ${p}`,
    `Kind=${kindVariants.length} KeyType=2 paiStatics=${paiStatics.length} factoryMethods=${factoryMethods.length} ccImpls=${new Set(ccImpls).size}`,
  ].join("\n"),
);
