#!/usr/bin/env bun
// tool/src/objects/js-array.h.js — 校验并提取 JSArray 布局与常量
// 来源:
//   v8/src/objects/js-array.h + js-array-inl.h
//   → 对应 v8-objects-js-core/src/js_array.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/js-array.h");
const inl = readV8("src/objects/js-array-inl.h");

function mustContain(content, substr, filename) {
  if (!content.includes(substr)) {
    throw new Error(`锚点失配: ${filename} 缺少 "${substr.slice(0, 48)}..."`);
  }
}

// ── 1. 核心锚点检验 ────────────────────────────────────────────────────────
mustContain(h, "V8_OBJECT class JSArray : public JSObject {", "js-array.h");
mustContain(h, "TaggedMember<Number> length_;", "js-array.h");
mustContain(h, "static const int kPreallocatedArrayElements = 4;", "js-array.h");
mustContain(h, "static const int kLengthDescriptorIndex = 0;", "js-array.h");
mustContain(h, "static const int kMaxCopyElements = 100;", "js-array.h");
mustContain(h, "static constexpr int kMaxInlineSortLength = 16;", "js-array.h");
mustContain(h, "static constexpr uint32_t kMaxArrayLength = JSObject::kMaxElementCount;", "js-array.h");
mustContain(h, "static constexpr uint32_t kMaxArrayIndex = JSObject::kMaxElementIndex;", "js-array.h");
mustContain(h, "static constexpr uint32_t kMaxFastArrayLength =", "js-array.h");
mustContain(h, "static const uint32_t kMinJoinStackSize = 2;", "js-array.h");
mustContain(h, "inline constexpr int JSArray::kHeaderSize = sizeof(JSArray);", "js-array.h");

mustContain(inl, "Tagged<Number> JSArray::length() const { return length_.load(); }", "js-array-inl.h");
mustContain(inl, "void JSArray::set_length(Tagged<Number> value, WriteBarrierMode mode) {", "js-array-inl.h");
mustContain(inl, "bool JSArray::SetLengthWouldNormalize(Heap* heap, uint32_t new_length) {", "js-array-inl.h");

// ── 2. 提取常量与布局 ──────────────────────────────────────────────────────
export const K_PREALLOCATED_ARRAY_ELEMENTS = 4;
export const K_LENGTH_DESCRIPTOR_INDEX = 0;
export const K_MAX_COPY_ELEMENTS = 100;
export const K_MAX_INLINE_SORT_LENGTH = 16;
export const K_MAX_ARRAY_LENGTH = 0xffff_ffff; // JSObject::kMaxElementCount
export const K_MAX_ARRAY_INDEX = K_MAX_ARRAY_LENGTH - 1;
// V8_LOWER_LIMITS_MODE_BOOL 默认 false: 32 * 1024 * 1024
export const K_MAX_FAST_ARRAY_LENGTH = 32 * 1024 * 1024;
export const K_MIN_JOIN_STACK_SIZE = 2;

// 布局:
// JSObject: map (4) + properties_or_hash (4) + elements (4) = 12
// JSArray: JSObject (12) + length_ (4) = 16
export const K_JS_ARRAY_LENGTH_OFFSET = 12;
export const K_JS_ARRAY_HEADER_SIZE = 16;

console.log("[js-array.h.js] 提取与锚点校验通过:");
console.log({
  K_PREALLOCATED_ARRAY_ELEMENTS,
  K_LENGTH_DESCRIPTOR_INDEX,
  K_MAX_COPY_ELEMENTS,
  K_MAX_INLINE_SORT_LENGTH,
  K_MAX_ARRAY_LENGTH,
  K_MAX_ARRAY_INDEX,
  K_MAX_FAST_ARRAY_LENGTH,
  K_MIN_JOIN_STACK_SIZE,
  K_JS_ARRAY_LENGTH_OFFSET,
  K_JS_ARRAY_HEADER_SIZE,
});
