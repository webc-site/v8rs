#!/usr/bin/env bun
// tool/src/compiler/property-access-builder.js
// v8/src/compiler/property-access-builder.h + property-access-builder.cc
// → v8-compiler-passes/src/property_access_builder.rs.skel
// 属性访问构建: 解析 TryBuild*/Build*/Fold* 方法面与 3 个自由函数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/property-access-builder.h");
const cc = readV8("src/compiler/property-access-builder.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`property-access-builder: ${msg}`);
};

for (const [s, name] of [
  ["class PropertyAccessBuilder {", "类定义"],
  ["bool TryBuildStringCheck(JSHeapBroker* broker, ZoneVector<MapRef> const& maps,", "TryBuildStringCheck"],
  ["static MachineRepresentation ConvertRepresentation(", "ConvertRepresentation"],
  ["bool HasOnlyStringMaps(JSHeapBroker* broker, ZoneVector<MapRef> const& maps);", "HasOnlyStringMaps"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— 方法面 (真解析 h) ——
const buildMethods = [...h.matchAll(/^\s{2}(?:bool|void|Node\*|std::optional<Node\*>|static MachineRepresentation) (\w+)\(/gm)].map((m) => m[1]);
for (const needM of ["TryBuildStringCheck", "TryBuildNumberCheck", "BuildCheckMaps", "BuildCheckValue", "BuildCheckSmi", "BuildCheckNumber", "BuildCheckNumberFitsInt32", "BuildLoadDataField", "FoldLoadDictPrototypeConstant", "BuildLoadDictionaryField", "ConvertRepresentation"]) {
  need(buildMethods.includes(needM), `方法 ${needM} 缺席 (实得 ${buildMethods.join(",")})`);
}

// —— 自由函数 ——
const freeFns = [...h.matchAll(/^bool (HasOnly\w+)\(JSHeapBroker\* broker,/gm)].map((m) => m[1]);
need(freeFns.length === 3, `自由函数 ${freeFns.length}`);

// —— cc ConvertRepresentation 的映射表 (真解析) ——
const crIdx = cc.indexOf("MachineRepresentation PropertyAccessBuilder::ConvertRepresentation(");
need(crIdx > 0, "cc ConvertRepresentation 未找到");
const crBody = cc.slice(crIdx, cc.indexOf("\n}", crIdx));
const repCases = [...crBody.matchAll(/case Representation::k(\w+):/g)].map((m) => m[1]);
need(repCases.length >= 4, `表示 case ${repCases.length}`);

const body = `${banner("src/compiler/property-access-builder.js", "src/compiler/property-access-builder.h + property-access-builder.cc", "命名属性访问的检查/加载节点构建")}
//! property-access-builder — 依 PropertyAccessInfo 建图
//!
//! map 族判定: ${freeFns.join(" / ")};
//! ConvertRepresentation 映射 (${repCases.length} case): ${repCases.join(", ")}。

/// 原版 PropertyAccessBuilder — 构建器
pub struct PropertyAccessBuilder<'z, 'g> {
  /// jsgraph_
  pub jsgraph: &'g JSGraph<'z>,
  /// broker_
  pub broker: &'g mut JSHeapBroker<'z>,
}

impl PropertyAccessBuilder<'_, '_> {
  /// 原版 TryBuildStringCheck — 全 string map 时建 String 检查
  /// v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::TryBuildStringCheck
  pub fn try_build_string_check(&mut self, broker: &JSHeapBroker, maps: &[MapRef], receiver: &mut *mut Node, effect: &mut *mut Node, control: *mut Node) -> bool {
    todo!("v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::TryBuildStringCheck")
  }

  /// 原版 TryBuildNumberCheck — 全 number map 时建 Number 检查
  /// v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::TryBuildNumberCheck
  pub fn try_build_number_check(&mut self, broker: &JSHeapBroker, maps: &[MapRef], receiver: &mut *mut Node, effect: &mut *mut Node, control: *mut Node) -> bool {
    todo!("v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::TryBuildNumberCheck")
  }

  /// 原版 BuildCheckMaps — 依 map 集建 CheckMaps (含迁移分支)
  /// v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::BuildCheckMaps
  pub fn build_check_maps(&mut self, object: &Node, effect: &mut *mut Node, control: *mut Node, maps: &[MapRef], has_deprecated_map_without_migration_target: bool) {
    todo!("v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::BuildCheckMaps")
  }

  /// 原版 BuildCheckValue — 常量receiver检查
  /// v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::BuildCheckValue
  pub fn build_check_value(&mut self, receiver: &Node, effect: &mut *mut Node, control: *mut Node, value: ObjectRef) -> *mut Node {
    todo!("v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::BuildCheckValue")
  }

  /// 原版 BuildLoadDataField — data-field/constant-field 加载
  /// v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::BuildLoadDataField
  pub fn build_load_data_field(&mut self, name: &NameRef, access_info: &PropertyAccessInfo, lookup_start_object: &Node, effect: &mut *mut Node, control: &mut *mut Node) -> *mut Node {
    todo!("v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::BuildLoadDataField")
  }

  /// 原版 FoldLoadDictPrototypeConstant — 原型字典常量折叠
  /// v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::FoldLoadDictPrototypeConstant
  pub fn fold_load_dict_prototype_constant(&mut self, access_info: &PropertyAccessInfo) -> Option<*mut Node> {
    todo!("v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::FoldLoadDictPrototypeConstant")
  }

  /// 原版 BuildLoadDictionaryField — 字典字段加载
  /// v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::BuildLoadDictionaryField
  pub fn build_load_dictionary_field(&mut self, name: &NameRef, access_info: &PropertyAccessInfo, lookup_start_object: &Node, receiver: &Node, effect: &mut *mut Node, control: &mut *mut Node, source: &FeedbackSource, context: &Node, frame_state: &Node) -> *mut Node {
    todo!("v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::BuildLoadDictionaryField")
  }

  /// 原版 ConvertRepresentation — Representation → MachineRepresentation
  /// v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::ConvertRepresentation (${repCases.join("/")}）
  pub fn convert_representation(representation: Representation) -> MachineRepresentation {
    todo!("v8: src/compiler/property-access-builder.cc PropertyAccessBuilder::ConvertRepresentation")
  }
}

${freeFns.map((f) => `/// 原版 ${f}\n/// v8: src/compiler/property-access-builder.cc ${f}\npub fn ${f.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(broker: &JSHeapBroker, maps: &[MapRef]) -> bool {\n  todo!("v8: src/compiler/property-access-builder.cc ${f}")\n}`).join("\n\n")}
`;

const p = writeSrc("v8-compiler-passes", "property_access_builder.rs.skel", body);
console.log(`written ${p}\nmethods=${buildMethods.length} free_fns=${freeFns.length} rep_cases=${repCases.length} (${repCases.join(",")})`);
