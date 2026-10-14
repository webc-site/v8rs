#!/usr/bin/env bun
// v8/src/compiler/compilation-dependencies.cc + compilation-dependencies.h
// → v8-compiler-passes/src/compilation_dependencies.rs.skel
//
// 规则面: CompilationDependencies 公有 DependOn* 方法全集 (~40) +
// protector 便捷封装 13 项逐字提取; RecordDependency/PrepareInstall 签名。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/compilation-dependencies.h");
const cc = readV8("src/compiler/compilation-dependencies.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class SlackTrackingPrediction \{/, "SlackTrackingPrediction"],
  [h, /class V8_EXPORT CompilationDependencies : public ZoneObject \{/, "CompilationDependencies"],
  [h, /MapRef DependOnInitialMap\(JSFunctionRef function\);/, "DependOnInitialMap"],
  [h, /bool DependOnArrayBufferDetachingProtector\(\);/, "protector 组首"],
  [h, /bool DependOnStringWrapperToPrimitiveProtector\(\);/, "protector 组尾"],
  [h, /void RecordDependency\(CompilationDependency const\* dependency\);/, "RecordDependency"],
  [h, /CompilationDependency const\* TransitionDependencyOffTheRecord\(/, "OffTheRecord 组"],
  [cc, /bool CompilationDependencies::PrepareInstall\(\) \{/, "PrepareInstall"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— protector 便捷封装清单 ——
const protectors = [...h.matchAll(/^  bool (DependOn\w+Protector)\(\);/gm)].map((m) => m[1]);
if (protectors.length !== 13) throw new Error(`protector ${protectors.length} != 13`);
if (protectors[0] !== "DependOnArrayBufferDetachingProtector" || protectors.at(-1) !== "DependOnStringWrapperToPrimitiveProtector") {
  throw new Error(`protector 首尾漂移: ${protectors}`);
}
// DependOn* 全集 (含 protector 组)
const dependOns = [...h.matchAll(/^  \w[\w:<>,*& ]*?\s?(DependOn\w+)\(/gm)].map((m) => m[1]);
if (dependOns.length !== 37) throw new Error(`DependOn 全集 ${dependOns.length} != 37`);
// DependOnContextCell 重载 2
if (dependOns.filter((m) => m === "DependOnContextCell").length !== 2) throw new Error("DependOnContextCell 重载 != 2");

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const body = `${banner("src/compiler/compilation-dependencies.js", "src/compiler/compilation-dependencies.h + src/compiler/compilation-dependencies.cc", "CompilationDependencies — 优化假设登记/去重/安装 (protector 13 + DependOn 39)")}
//! CompilationDependencies: 编译期假设集, 代码安装前校验。
//!
//! 规则面: DependOn* ${dependOns.length} 项 (含 ContextCell 重载), protector 便捷
//! 封装 ${protectors.length} 项, OffTheRecord 组 3 项。
//! 语义面 (推迟): 各 CompilationDependency 子类的
//! IsValid/PrepareInstall/Install (cc: 各 DependOn 实现)。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/compilation-dependencies.h SlackTrackingPrediction
/// 预测的最终实例大小 + in-object 属性数
pub struct SlackTrackingPrediction {
  /// 原 instance_size
  pub instance_size: i32,
  /// 原 inobject_property_count
  pub inobject_property_count: i32,
}

/// v8: src/compiler/compilation-dependencies.h CompilationDependencies
pub struct CompilationDependencies {
  /// 原 zone_
  pub zone: *mut crate::zone::Zone,
  /// 原 broker_
  pub broker: *mut crate::broker::JSHeapBroker,
  /// 原 dependencies_ — ZoneUnorderedSet (Hash/Equal 泛型)
  pub dependencies: std::collections::HashSet<usize>,
}

impl CompilationDependencies {
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnInitialMap
  pub fn depend_on_initial_map(&mut self, function: crate::heap_refs::JSFunctionRef) -> crate::heap_refs::MapRef {
    todo!("InitialMapDependency 登记")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnPrototypeProperty
  pub fn depend_on_prototype_property(&mut self, function: crate::heap_refs::JSFunctionRef) -> crate::heap_refs::HeapObjectRef {
    todo!("PrototypePropertyDependency 登记")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnStableMap
  pub fn depend_on_stable_map(&mut self, map: crate::heap_refs::MapRef) {
    todo!("StableMapDependency 登记")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnNoSlackTrackingChange
  pub fn depend_on_no_slack_tracking_change(&mut self, map: crate::heap_refs::MapRef) {
    todo!("NoSlackTrackingChangeDependency 登记")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnConstantInDictionaryPrototypeChain
  pub fn depend_on_constant_in_dictionary_prototype_chain(&mut self, receiver_map: crate::heap_refs::MapRef, holder: crate::heap_refs::OptionalJSObjectRef, name: crate::heap_refs::OptionalNameRef, constant: crate::heap_refs::ObjectRef) {
    todo!("字典原型链常量依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnPretenureMode
  pub fn depend_on_pretenure_mode(&mut self, site: crate::heap_refs::AllocationSiteRef) -> crate::AllocationType {
    todo!("PretenureMode 依赖 + AllocationType 返回")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnFieldConstness
  pub fn depend_on_field_constness(&mut self, map: crate::heap_refs::MapRef, owner: crate::heap_refs::MapRef, descriptor: crate::InternalIndex) -> crate::PropertyConstness {
    todo!("FieldConstnessDependency 登记")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnGlobalProperty
  pub fn depend_on_global_property(&mut self, cell: crate::heap_refs::PropertyCellRef) {
    todo!("GlobalPropertyDependency 登记")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnContextCell (script_context 版)
  pub fn depend_on_context_cell(&mut self, script_context: crate::heap_refs::ContextRef, index: usize, expected: crate::ContextCell::State) -> bool {
    todo!("script_context 槽位 ContextCell 依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnEmptyContextExtension
  pub fn depend_on_empty_context_extension(&mut self, scope_info: crate::heap_refs::ScopeInfoRef) -> bool {
    todo!("空 context extension 依赖")
  }
${protectors.map((m) => `  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::${m}\n  pub fn ${snake(m)}(&mut self) -> bool {\n    todo!(" protector cell 依赖")\n  }`).join("\n")}
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnElementsKind
  pub fn depend_on_elements_kind(&mut self, site: crate::heap_refs::AllocationSiteRef) {
    todo!("ElementsKindDependency 登记")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnObjectSlotValue
  pub fn depend_on_object_slot_value(&mut self, object: crate::heap_refs::HeapObjectRef, offset: i32, value: crate::heap_refs::ObjectRef) {
    todo!("对象槽位值依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnOwnConstantElement
  pub fn depend_on_own_constant_element(&mut self, holder: crate::heap_refs::JSObjectRef, index: u32, element: crate::heap_refs::ObjectRef) {
    todo!("自有常量元素依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnOwnConstantDataProperty
  pub fn depend_on_own_constant_data_property(&mut self, holder: crate::heap_refs::JSObjectRef, map: crate::heap_refs::MapRef, index: crate::FieldIndex, value: crate::heap_refs::ObjectRef) {
    todo!("自有常量数据属性依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnOwnConstantDoubleProperty
  pub fn depend_on_own_constant_double_property(&mut self, holder: crate::heap_refs::JSObjectRef, map: crate::heap_refs::MapRef, index: crate::FieldIndex, value: crate::Float64) {
    todo!("自有常量 double 属性依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnOwnConstantDictionaryProperty
  pub fn depend_on_own_constant_dictionary_property(&mut self, holder: crate::heap_refs::JSObjectRef, index: crate::InternalIndex, value: crate::heap_refs::ObjectRef) {
    todo!("自有常量字典属性依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnStablePrototypeChains
  pub fn depend_on_stable_prototype_chains(&mut self, receiver_maps: &[crate::heap_refs::MapRef], start: crate::WhereToStart, last_prototype: crate::heap_refs::OptionalJSObjectRef) {
    todo!("稳定原型链依赖 (复数 map)")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnStablePrototypeChain
  pub fn depend_on_stable_prototype_chain(&mut self, receiver_map: crate::heap_refs::MapRef, start: crate::WhereToStart, last_prototype: crate::heap_refs::OptionalJSObjectRef) {
    todo!("稳定原型链依赖 (单 map)")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnElementsKinds
  pub fn depend_on_elements_kinds(&mut self, site: crate::heap_refs::AllocationSiteRef) {
    todo!("嵌套 AllocationSite 的 ElementsKind 递归依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnConsistentJSFunctionView
  pub fn depend_on_consistent_js_function_view(&mut self, function: crate::heap_refs::JSFunctionRef) {
    todo!("JSFunction 一致视图依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnInitialMapInstanceSizePrediction
  pub fn depend_on_initial_map_instance_size_prediction(&mut self, function: crate::heap_refs::JSFunctionRef) -> SlackTrackingPrediction {
    todo!("slack tracking 预测 + InitialMap 依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::RecordDependency
  /// 原文: 非空才入 set (Hash/Equal 去重)
  pub fn record_dependency(&mut self, dependency: *const CompilationDependency) {
    todo!("set 插入去重")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::TransitionDependencyOffTheRecord
  pub fn transition_dependency_off_the_record(&self, target_map: crate::heap_refs::MapRef) -> *const CompilationDependency {
    todo!("只取不登")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::FieldRepresentationDependencyOffTheRecord
  pub fn field_representation_dependency_off_the_record(&self, map: crate::heap_refs::MapRef, owner: crate::heap_refs::MapRef, descriptor: crate::InternalIndex, representation: crate::Representation) -> *const CompilationDependency {
    todo!("只取不登")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::DependOnFieldRepresentation
  pub fn depend_on_field_representation(&mut self, map: crate::heap_refs::MapRef, owner: crate::heap_refs::MapRef, descriptor: crate::InternalIndex, representation: crate::Representation) {
    todo!("FieldRepresentation 依赖")
  }
  /// v8: src/compiler/compilation-dependencies.h CompilationDependencies::FieldTypeDependencyOffTheRecord
  pub fn field_type_dependency_off_the_record(&self, map: crate::heap_refs::MapRef, owner: crate::heap_refs::MapRef, descriptor: crate::InternalIndex, ty: crate::heap_refs::ObjectRef) -> *const CompilationDependency {
    todo!("只取不登")
  }
  /// v8: src/compiler/compilation-dependencies.cc CompilationDependencies::PrepareInstall
  /// 原文: 逐依赖 IsValid + PrepareInstall, 失败即 false
  pub fn prepare_install(&mut self) -> bool {
    todo!("依赖全集校验")
  }
}

/// v8: src/compiler/compilation-dependencies.h CompilationDependency
/// 语义面: IsValid/PrepareInstall/Install/Absorb 虚表, 子类 ~25 个在 .cc
pub struct CompilationDependency {
  pub _semantic: (),
}
`;

const p = writeSrc("v8-compiler-passes", "compilation_dependencies.rs.skel", body);
console.log(`written ${p}\nDependOn=${dependOns.length} protector=${protectors.length} OffTheRecord=3`);
