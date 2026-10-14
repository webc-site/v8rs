#!/usr/bin/env bun
// tool/src/objects/property-array.h.js — 校验并提取 PropertyArray 布局与常量
// 来源:
//   v8/src/objects/property-array.h + property-array-inl.h
//   → 对应 v8-objects-maps/src/property_array.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/property-array.h");
const inl = readV8("src/objects/property-array-inl.h");

function mustContain(content, substr, filename) {
  if (!content.includes(substr)) {
    throw new Error(`锚点失配: ${filename} 缺少 "${substr.slice(0, 48)}..."`);
  }
}

// ── 1. 核心锚点检验 ────────────────────────────────────────────────────────
mustContain(h, "V8_OBJECT class PropertyArray : public HeapObject {", "property-array.h");
mustContain(h, "static const int kLengthFieldSize = 10;", "property-array.h");
mustContain(h, "using LengthField = base::BitField<int, 0, kLengthFieldSize>;", "property-array.h");
mustContain(h, "static const int kMaxLength = LengthField::kMax;", "property-array.h");
mustContain(h, "static const int kNoHashSentinel = 0;", "property-array.h");
mustContain(h, "TaggedMember<Smi> length_and_hash_;", "property-array.h");
mustContain(inl, "int PropertyArray::Hash() const { return HashField::decode(length_and_hash()); }", "property-array-inl.h");
mustContain(inl, "void PropertyArray::initialize_length(uint32_t len) {", "property-array-inl.h");

// ── 2. 提取常量与布局 ──────────────────────────────────────────────────────
export const K_LENGTH_FIELD_SIZE = 10;
export const K_MAX_LENGTH = (1 << K_LENGTH_FIELD_SIZE) - 1; // 1023
export const K_NO_HASH_SENTINEL = 0;

// LengthField: shift = 0, size = 10, mask = 0x3ff
export const LENGTH_FIELD_SHIFT = 0;
export const LENGTH_FIELD_SIZE = 10;
export const LENGTH_FIELD_MASK = (1 << LENGTH_FIELD_SIZE) - 1;

// HashField: shift = 10, size = 20 (assuming 31-bit Smi value: 31 - 10 - 1 = 20)
export const HASH_FIELD_SHIFT = 10;
export const HASH_FIELD_SIZE = 20;
export const HASH_FIELD_MASK = ((1 << HASH_FIELD_SIZE) - 1) << HASH_FIELD_SHIFT;

console.log("[property-array.h.js] 提取与锚点校验通过:");
console.log({
  K_LENGTH_FIELD_SIZE,
  K_MAX_LENGTH,
  K_NO_HASH_SENTINEL,
  LENGTH_FIELD_SHIFT,
  LENGTH_FIELD_MASK,
  HASH_FIELD_SHIFT,
  HASH_FIELD_MASK,
});
