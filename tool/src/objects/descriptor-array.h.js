#!/usr/bin/env bun
// tool/src/objects/descriptor-array.h.js — 校验并提取 DescriptorArray 槽位布局与常量
// 来源:
//   v8/src/objects/descriptor-array.h + descriptor-array-inl.h
//   → 对应 v8-objects-maps/src/descriptor_array.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/descriptor-array.h");
const inl = readV8("src/objects/descriptor-array-inl.h");

function mustContain(content, substr, filename) {
  if (!content.includes(substr)) {
    throw new Error(`锚点失配: ${filename} 缺少 "${substr.slice(0, 48)}..."`);
  }
}

// ── 1. 核心锚点检验 ────────────────────────────────────────────────────────
mustContain(h, "static constexpr int kMaxElementsForLinearSearch = 32;", "descriptor-array.h");
mustContain(h, "static const int kNotFound = -1;", "descriptor-array.h");
mustContain(h, "static const int kEntryKeyIndex = 0;", "descriptor-array.h");
mustContain(h, "static const int kEntryDetailsIndex = 1;", "descriptor-array.h");
mustContain(h, "static const int kEntryValueIndex = 2;", "descriptor-array.h");
mustContain(h, "static const int kEntrySize = 3;", "descriptor-array.h");
mustContain(h, "enum class FastIterableState : uint8_t", "descriptor-array.h");
mustContain(h, "kJsonFast = 0b00,", "descriptor-array.h");
mustContain(h, "kJsonSlow = 0b01,", "descriptor-array.h");
mustContain(h, "kUnknown = 0b11", "descriptor-array.h");
mustContain(h, "V8_OBJECT class EnumCache : public Struct", "descriptor-array.h");
mustContain(inl, "int16_t DescriptorArray::number_of_all_descriptors() const", "descriptor-array-inl.h");
mustContain(inl, "int16_t DescriptorArray::number_of_descriptors() const", "descriptor-array-inl.h");
mustContain(inl, "uint32_t DescriptorArray::flags(RelaxedLoadTag) const", "descriptor-array-inl.h");

// ── 2. 提取常量与布局 ──────────────────────────────────────────────────────
export const K_MAX_ELEMENTS_FOR_LINEAR_SEARCH = 32;
export const K_NOT_FOUND = -1;
export const K_ENTRY_KEY_INDEX = 0;
export const K_ENTRY_DETAILS_INDEX = 1;
export const K_ENTRY_VALUE_INDEX = 2;
export const K_ENTRY_SIZE = 3;

export const FAST_ITERABLE_STATE = {
  kJsonFast: 0b00,
  kJsonSlow: 0b01,
  kUnknown: 0b11,
};

export function toKeyIndex(descriptorNumber) {
  return descriptorNumber * K_ENTRY_SIZE + K_ENTRY_KEY_INDEX;
}

export function toDetailsIndex(descriptorNumber) {
  return descriptorNumber * K_ENTRY_SIZE + K_ENTRY_DETAILS_INDEX;
}

export function toValueIndex(descriptorNumber) {
  return descriptorNumber * K_ENTRY_SIZE + K_ENTRY_VALUE_INDEX;
}

console.log("[descriptor-array.h.js] 提取与锚点校验通过:");
console.log({
  K_MAX_ELEMENTS_FOR_LINEAR_SEARCH,
  K_ENTRY_SIZE,
  K_ENTRY_KEY_INDEX,
  K_ENTRY_DETAILS_INDEX,
  K_ENTRY_VALUE_INDEX,
  FAST_ITERABLE_STATE,
});
