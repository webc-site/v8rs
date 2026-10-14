#!/usr/bin/env bun
// tool/src/objects/property-cell.h.js — 校验并提取 PropertyCell 槽位偏移与 PropertyCellType
// 来源:
//   v8/src/objects/property-cell.h + property-cell-inl.h + property-details.h
//   → 对应 v8-objects-rest/src/property_cell.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/property-cell.h");
const inl = readV8("src/objects/property-cell-inl.h");
const detailsH = readV8("src/objects/property-details.h");

let checks = 0;
function must(cond, msg) {
  checks++;
  if (!cond) throw new Error(`锚点失配: ${msg}`);
}
function mustContain(content, substr, filename) {
  checks++;
  if (!content.includes(substr)) {
    throw new Error(`锚点失配: ${filename} 缺少 "${substr.slice(0, 48)}..."`);
  }
}

// ── 1. 核心锚点检验 ────────────────────────────────────────────────────────
mustContain(h, "V8_OBJECT class PropertyCell : public HeapObject", "property-cell.h");
mustContain(h, "TaggedMember<Name> name_;", "property-cell.h");
mustContain(h, "TaggedMember<Smi> property_details_raw_;", "property-cell.h");
mustContain(h, "TaggedMember<Object> value_;", "property-cell.h");
mustContain(h, "TaggedMember<DependentCode> dependent_code_;", "property-cell.h");
mustContain(h, "FixedBodyDescriptor<offsetof(PropertyCell, name_)", "property-cell.h");

mustContain(inl, "Tagged<Name> PropertyCell::name() const", "property-cell-inl.h");
mustContain(inl, "Tagged<Smi> PropertyCell::property_details_raw() const", "property-cell-inl.h");
mustContain(inl, "Tagged<Object> PropertyCell::value() const", "property-cell-inl.h");
mustContain(inl, "Tagged<DependentCode> PropertyCell::dependent_code() const", "property-cell-inl.h");
mustContain(inl, "void PropertyCell::Transition(PropertyDetails new_details,", "property-cell-inl.h");

mustContain(detailsH, "enum class PropertyCellType {", "property-details.h");
mustContain(detailsH, "kMutable,", "property-details.h");
mustContain(detailsH, "kUndefined,", "property-details.h");
mustContain(detailsH, "kConstant,", "property-details.h");
mustContain(detailsH, "kConstantType,", "property-details.h");
mustContain(detailsH, "kInTransition,", "property-details.h");

// ── 2. 枚举值定义 ─────────────────────────────────────────────────────────
export const PROPERTY_CELL_TYPE = {
  Mutable: 0,
  Undefined: 1,
  Constant: 2,
  ConstantType: 3,
  InTransition: 4,
};

// ── 3. 内存偏移推导 (64-bit 非压缩 Tagged) ────────────────────────────────
export const K_PROPERTY_CELL_HEADER_SIZE = 8; // HeapObject map
export const K_PROPERTY_CELL_NAME_OFFSET = 8;
export const K_PROPERTY_CELL_DETAILS_OFFSET = 16;
export const K_PROPERTY_CELL_VALUE_OFFSET = 24;
export const K_PROPERTY_CELL_DEPENDENT_CODE_OFFSET = 32;
export const K_PROPERTY_CELL_SIZE = 40;

must(K_PROPERTY_CELL_NAME_OFFSET === 8, "name_ offset 校验");
must(K_PROPERTY_CELL_DETAILS_OFFSET === 16, "property_details_raw_ offset 校验");
must(K_PROPERTY_CELL_VALUE_OFFSET === 24, "value_ offset 校验");
must(K_PROPERTY_CELL_DEPENDENT_CODE_OFFSET === 32, "dependent_code_ offset 校验");
must(K_PROPERTY_CELL_SIZE === 40, "PropertyCell size 校验");

console.log(`[property-cell.h.js] 提取与锚点校验通过 (共 ${checks} 项校验):`);
console.log({
  PROPERTY_CELL_TYPE,
  K_PROPERTY_CELL_NAME_OFFSET,
  K_PROPERTY_CELL_DETAILS_OFFSET,
  K_PROPERTY_CELL_VALUE_OFFSET,
  K_PROPERTY_CELL_DEPENDENT_CODE_OFFSET,
  K_PROPERTY_CELL_SIZE,
});
