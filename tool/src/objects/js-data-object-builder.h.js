#!/usr/bin/env bun
// tool/src/objects/js-data-object-builder.h.js — JSDataObjectBuilder 概念/模式/签名骨架提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-data-object-builder.{h,-inl.h,cc}
//   → v8-objects-js-core/src/js_data_object_builder.rs.skel
// 注意: 本类是离堆 C++ builder (非 HeapObject), 产出为 trait 概念 + 字段骨架。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-data-object-builder.h");
const inl = readV8("src/objects/js-data-object-builder-inl.h");
const cc = readV8("src/objects/js-data-object-builder.cc");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. HeapNumberMode 枚举 ────────────────────────────────────────────────
has(h, "enum HeapNumberMode {\n    kNormalHeapNumbers,\n    kHeapNumbersGuaranteedUniquelyOwned\n  };", "h HeapNumberMode");

// ── 2. 两个 concept (raw keys / eager keys) ──────────────────────────────
has(h, "concept JSDataPropertyIteratorWithRawKeys =", "h concept raw");
has(h, "concept JSDataPropertyIteratorWithEagerKeys =", "h concept eager");
// raw: kSupportsRawKeys == true; eager: == false (concept 谓词取反)
has(h, "std::remove_reference_t<PropertyIterator>::kSupportsRawKeys &&", "h raw 谓词");
has(h, "!std::remove_reference_t<PropertyIterator>::kSupportsRawKeys &&", "h eager 谓词");
// concept 要求的方法全集: raw 6 + eager 5
const rawSeg = h.slice(h.indexOf("concept JSDataPropertyIteratorWithRawKeys"), h.indexOf("concept JSDataPropertyIteratorWithEagerKeys"));
for (const r of ["it.Advance();", "it.Done();", "it.kMayHaveDuplicateKeys;", "it.GetKeyChars();", "it.GetKey(key_hint);", "it.RevisitValues();"])
  must(rawSeg.includes(r), `raw concept 缺 ${r}`);
const eagerSeg = h.slice(h.indexOf("concept JSDataPropertyIteratorWithEagerKeys"));
for (const r of ["it.Advance();", "it.Done();", "it.kMayHaveDuplicateKeys;", "it.GetKey();", "it.RevisitValues();"])
  must(eagerSeg.includes(r), `eager concept 缺 ${r}`);

// ── 3. 成员字段全集 (11; 圈最后一个 private: 区, 严格 2 空格缩进行) ──────
const classBody = h.slice(h.indexOf("class JSDataObjectBuilder {"));
const priv = classBody.slice(classBody.lastIndexOf(" private:"));
const fields = [...priv.matchAll(/^ {2}\S.*;$/gm)]
  .map((m) => m[0].replace(/\s*=[^;]*;/, ";").trim())
  .filter((l) => !l.startsWith("//") && !l.includes("("))
  .map((l) => l.match(/(\w+);$/)[1]);
must(fields.length === 11, `成员字段应 11, 实得 ${fields.length}:\n${fields.join(",")}`);
must(fields.join(",") ===
  "isolate_,elements_kind_,expected_property_count_,heap_number_mode_,may_have_duplicate_keys_,map_,current_property_index_,extra_heap_numbers_needed_,object_,expected_final_map_,property_count_in_expected_final_map_",
  `字段序失配: ${fields.join(",")}`);
has(h, "bool may_have_duplicate_keys_ = true;", "默认值 true");
has(h, "int current_property_index_ = 0;", "默认值 0");

// ── 4. 方法签名骨架 (h+inl 联合对数) ─────────────────────────────────────
const METHODS = ["JSDataObjectBuilder(", "BuildFromIterator(", "CreateAndInitialiseObject(",
  "AddSlowProperty(", "TryAddFastPropertyForValue(", "TryFastTransitionToPropertyKey(",
  "TryAddFastPropertyTransitionForValue(", "TryGeneralizeFieldToValue(",
  "TryInitializeMapFromExpectedFinalMap()", "InitializeMapFromZero()",
  "IsOnExpectedFinalMapFastPath() const", "RewindExpectedFinalMapFastPathToBeforeCurrent()",
  "RewindExpectedFinalMapFastPathToIncludeCurrent()", "RegisterFieldNeedsFreshHeapNumber()",
  "RecalculateExtraHeapNumbersNeeded()", "AdvanceToNextProperty()"];
for (const m of METHODS) must(h.includes(m) || inl.includes(m), `缺方法 ${m}`);
must((h.match(/BuildFromIterator\(/g) || []).length === 2, "BuildFromIterator 应 2 个重载");
must((inl.match(/TryAddFastPropertyForValue\(/g) || []).length >= 3, "TryAddFastPropertyForValue 重载应 ≥3 (2 模板 + 1 实例)");
has(inl, "static_assert(!USE_ALLOCATION_ALIGNMENT_HEAP_NUMBER_BOOL);", "inl 对齐断言");
has(cc, "JSDataObjectBuilder::AddSlowProperty", "cc AddSlowProperty");

const body = `${banner("src/objects/js-data-object-builder.h.js", "src/objects/js-data-object-builder.{h,-inl.h,cc}", "JSDataObjectBuilder 概念/HeapNumberMode/字段骨架")}
//! 差集骨架: JSDataObjectBuilder (解析器/描述符 → JSObject 的快路径构建器, 离堆类)。
//! C++ concept → Rust trait; 值迭代器经 RevisitValues 二次访问。

/// v8: src/objects/js-data-object-builder.h JSDataObjectBuilder::HeapNumberMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapNumberMode { NormalHeapNumbers = 0, HeapNumbersGuaranteedUniquelyOwned = 1 }

/// v8: src/objects/js-data-object-builder.h concept JSDataPropertyIteratorWithRawKeys
///   (kSupportsRawKeys = true): Advance/Done/kMayHaveDuplicateKeys/GetKeyChars/
///   GetKey(expected_key_hint)/GetValue(will_revisit_value)/RevisitValues
pub trait PropertyIteratorWithRawKeys { /* 语义面留 Developer */ }

/// v8: src/objects/js-data-object-builder.h concept JSDataPropertyIteratorWithEagerKeys
///   (kSupportsRawKeys = false): Advance/Done/kMayHaveDuplicateKeys/GetKey/
///   GetValue(will_revisit_value)/RevisitValues
pub trait PropertyIteratorWithEagerKeys { /* 语义面留 Developer */ }

/// v8: src/objects/js-data-object-builder.h JSDataObjectBuilder 成员 (${fields.length} 个):
///   isolate_, elements_kind_, expected_property_count_, heap_number_mode_,
///   may_have_duplicate_keys_ = true, map_, current_property_index_ = 0,
///   extra_heap_numbers_needed_ = 0, object_, expected_final_map_,
///   property_count_in_expected_final_map_ = 0
pub struct JSDataObjectBuilder { /* 见上; Rust 侧以 builder 模式重建 */ }

/// v8: src/objects/js-data-object-builder.h 方法签名骨架 (${METHODS.length} 组):
${METHODS.map((m) => `///   ${m.replace(/[()]$/, "")}`).join("\n")}
/// v8: src/objects/js-data-object-builder-inl.h static_assert(!USE_ALLOCATION_ALIGNMENT_HEAP_NUMBER_BOOL)
`;

const p = writeSrc("v8-objects-js-core", "js_data_object_builder.rs.skel", body);
console.log(
  `js-data-object-builder.h.js: ${p} — 枚举 2 值、concept 2 (raw 6 + eager 5 要求)、` +
    `字段 ${fields.length}/11、方法 ${METHODS.length} 组、断言 ${checks} 项通过`,
);
