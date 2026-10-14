#!/usr/bin/env bun
// tool/src/compiler/wasm-load-elimination.js
// v8/src/compiler/wasm-load-elimination.h + wasm-load-elimination.cc
// → v8-compiler-passes/src/wasm_load_elimination.rs.skel
// wasm 载入消除: 解析 FieldOrElementValue / HalfState / AbstractState
// 三层状态与 Reduce* 面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-load-elimination.h");
const cc = readV8("src/compiler/wasm-load-elimination.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-load-elimination: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE WasmLoadElimination final", "类定义"],
  ["struct FieldOrElementValue {", "FieldOrElementValue"],
  ["class HalfState final : public ZoneObject {", "HalfState"],
  ["struct AbstractState : public ZoneObject {", "AbstractState"],
  ["Reduction WasmLoadElimination::Reduce(Node* node) {", "cc Reduce"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Reduce* 面 (真解析) ——
const reducers = [...h.matchAll(/^\s{2}Reduction Reduce(\w+)\(Node\* node\);/gm)].map((m) => m[1]);
need(reducers.length >= 5, `Reduce* ${reducers.length}`);

// —— HalfState 方法面 ——
const half = h.slice(h.indexOf("class HalfState final"));
const halfMethods = [...half.slice(0, half.indexOf("\n  };") > 0 ? half.indexOf("\n  };") : 2000).matchAll(/^\s{4}(?:HalfState const\*|FieldOrElementValue|void|bool) (\w+)\(/gm)].map((m) => m[1]);
for (const m of ["Equals", "IsEmpty", "IntersectWith", "KillField", "AddField", "LookupField"]) {
  need(halfMethods.includes(m), `HalfState::${m} 缺席 (实得 ${halfMethods.join(",")})`);
}

// —— AbstractState 双状态 (可变/不可变字段) ——
const abs = h.slice(h.indexOf("struct AbstractState"));
const absEnd = abs.indexOf("\n  };");
const absFields = [...abs.slice(0, absEnd > 0 ? absEnd : 800).matchAll(/^\s{4}HalfState (\w+);/gm)].map((m) => m[1]);
need(absFields.length === 2 && absFields[0] === "mutable_state" && absFields[1] === "immutable_state", `AbstractState 字段 ${absFields}`);

// —— cc 定义对数 ——
const ccDefs = ["Reduce", ...reducers.map((r) => r)].filter((r) => cc.includes(`WasmLoadElimination::${r}(Node*`) || cc.includes(`WasmLoadElimination::Reduce${r}(Node*`));

const body = `${banner("src/compiler/wasm-load-elimination.js", "src/compiler/wasm-load-elimination.h + wasm-load-elimination.cc", "wasm 结构字段/数组元素载入消除")}
//! wasm-load-elimination — 效应路径上的字段值缓存
//!
//! 三层: FieldOrElementValue (值) → HalfState (${halfMethods.join("/")} 不可变
//! 半区, OuterMap 双层) → AbstractState (可变 ${absFields[0]} / 不可变 ${absFields[1]});
//! Reduce* 面 ${reducers.length} 个。

/// 原版 FieldOrElementValue — 单槽缓存值
pub struct FieldOrElementValue {
  /// value (null = 空)
  pub value: *mut Node,
}

impl FieldOrElementValue {
  /// 原版 IsEmpty
  /// v8: src/compiler/wasm-load-elimination.h FieldOrElementValue::IsEmpty
  pub fn is_empty(&self) -> bool {
    self.value.is_null()
  }
}

/// 原版 HalfState — 字段缓存半区 (ZoneObject, 不可变共享)
pub struct HalfState<'z> {
  /// zone_
  pub zone: &'z Zone<'z>,
  // OuterMap<OuterKey> → (inner_key → FieldOrElementValue)
}

impl HalfState<'_> {
  /// 原版 Equals
  /// v8: src/compiler/wasm-load-elimination.cc HalfState::Equals
  pub fn equals(&self, that: &Self) -> bool {
    todo!("v8: src/compiler/wasm-load-elimination.cc HalfState::Equals")
  }

  /// 原版 IsEmpty
  /// v8: src/compiler/wasm-load-elimination.cc HalfState::IsEmpty
  pub fn is_empty(&self) -> bool {
    todo!("v8: src/compiler/wasm-load-elimination.cc HalfState::IsEmpty")
  }

  /// 原版 IntersectWith — 分支汇聚取交
  /// v8: src/compiler/wasm-load-elimination.cc HalfState::IntersectWith
  pub fn intersect_with(&mut self, that: &Self) {
    todo!("v8: src/compiler/wasm-load-elimination.cc HalfState::IntersectWith")
  }

  /// 原版 KillField — 写即杀 (copy-on-write)
  /// v8: src/compiler/wasm-load-elimination.cc HalfState::KillField
  pub fn kill_field<'z>(&self, field_index: i32, object: &Node) -> &HalfState<'z> {
    todo!("v8: src/compiler/wasm-load-elimination.cc HalfState::KillField")
  }

  /// 原版 AddField — 记录字段值 (copy-on-write)
  /// v8: src/compiler/wasm-load-elimination.cc HalfState::AddField
  pub fn add_field<'z>(&self, field_index: i32, object: &Node, value: &Node) -> &HalfState<'z> {
    todo!("v8: src/compiler/wasm-load-elimination.cc HalfState::AddField")
  }

  /// 原版 LookupField — 字段值查询
  /// v8: src/compiler/wasm-load-elimination.cc HalfState::LookupField
  pub fn lookup_field(&self, field_index: i32, object: &Node) -> FieldOrElementValue {
    todo!("v8: src/compiler/wasm-load-elimination.cc HalfState::LookupField")
  }
}

/// 原版 AbstractState — 双 HalfState (可变 ${absFields[0]} / 不可变 ${absFields[1]})
pub struct AbstractState<'z> {
  /// ${absFields[0]}
  pub ${absFields[0]}: *const HalfState<'z>,
  /// ${absFields[1]}
  pub ${absFields[1]}: *const HalfState<'z>,
}

/// 原版 WasmLoadElimination — AdvancedReducer
pub struct WasmLoadElimination<'z, 'g> {
  /// jsgraph_
  pub jsgraph: &'g JSGraph<'z>,
  /// zone_
  pub zone: &'z Zone<'z>,
}

impl WasmLoadElimination<'_, '_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "WasmLoadElimination";

${reducers.map((r) => `  /// 原版 Reduce${r}\n  /// v8: src/compiler/wasm-load-elimination.cc WasmLoadElimination::Reduce${r}\n  fn ${"reduce_" + r.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, node: &Node) -> Reduction {\n    todo!("v8: src/compiler/wasm-load-elimination.cc WasmLoadElimination::Reduce${r}")\n  }`).join("\n\n")}
}
`;

const p = writeSrc("v8-compiler-passes", "wasm_load_elimination.rs.skel", body);
console.log(`written ${p}\nreduce_face=${reducers.length} half_methods=${halfMethods.length} abstract_dual=${absFields.length} cc_defs=${ccDefs.length}`);
