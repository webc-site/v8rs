#!/usr/bin/env bun
// tool/src/objects/js-objects.h.js — 校验并提取 JSReceiver 与 JSObject 布局与常量
// 来源:
//   v8/src/objects/js-objects.h + js-objects-inl.h
//   → 对应 v8-objects-js-core/src/js_receiver.rs 与 js_object.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/js-objects.h");
const inl = readV8("src/objects/js-objects-inl.h");

function mustContain(content, substr, filename) {
  if (!content.includes(substr)) {
    throw new Error(`锚点失配: ${filename} 缺少 "${substr.slice(0, 48)}..."`);
  }
}

// ── 1. 核心锚点检验 ────────────────────────────────────────────────────────
mustContain(h, "V8_OBJECT class JSReceiver : public HeapObject {", "js-objects.h");
mustContain(h, "TaggedMember<PropertiesOrHash> properties_or_hash_;", "js-objects.h");
mustContain(h, "V8_OBJECT class JSObject : public JSReceiver {", "js-objects.h");
mustContain(h, "TaggedMember<FixedArrayBase> elements_;", "js-objects.h");
mustContain(h, "static const int kHeaderSize;", "js-objects.h");
mustContain(h, "inline constexpr int JSObject::kHeaderSize = sizeof(JSObject);", "js-objects.h");
mustContain(h, "inline constexpr int JSObject::kEndOfStrongFieldsOffset =", "js-objects.h");
mustContain(h, "static constexpr uint32_t kMaxElementCount = kMaxUInt32;", "js-objects.h");
mustContain(h, "static constexpr uint32_t kMaxElementIndex = kMaxElementCount - 1;", "js-objects.h");
mustContain(h, "static const uint32_t kMaxGap = 1024;", "js-objects.h");
mustContain(h, "static const int kMaxUncheckedFastElementsLength = 5000;", "js-objects.h");
mustContain(h, "static const int kMaxUncheckedOldFastElementsLength = 500;", "js-objects.h");
mustContain(h, "static const int kInitialGlobalObjectUnusedPropertiesCount = 4;", "js-objects.h");
mustContain(h, "static const int kMaxInstanceSize = 255 * kTaggedSize;", "js-objects.h");
mustContain(h, "static const int kFieldsAdded = 3;", "js-objects.h");

mustContain(inl, "Tagged<FixedArrayBase> JSObject::elements() const { return elements_.load(); }", "js-objects-inl.h");
mustContain(inl, "Tagged<JSReceiver::PropertiesOrHash> JSReceiver::raw_properties_or_hash()", "js-objects-inl.h");
mustContain(inl, "Tagged<JSAny> JSObject::RawFastPropertyAt(FieldIndex index) const", "js-objects-inl.h");
mustContain(inl, "void JSObject::FastPropertyAtPut(FieldIndex index, Tagged<Object> value,", "js-objects-inl.h");

// ── 2. 提取常量与布局 ──────────────────────────────────────────────────────
export const K_MAX_ELEMENT_COUNT = 0xffff_ffff; // kMaxUInt32
export const K_MAX_ELEMENT_INDEX = K_MAX_ELEMENT_COUNT - 1;
export const K_MAX_GAP = 1024;
export const K_MAX_UNCHECKED_FAST_ELEMENTS_LENGTH = 5000;
export const K_MAX_UNCHECKED_OLD_FAST_ELEMENTS_LENGTH = 500;
export const K_INITIAL_GLOBAL_OBJECT_UNUSED_PROPERTIES_COUNT = 4;
export const K_FIELDS_ADDED = 3;

// 指针压缩默认配置 (K_TAGGED_SIZE = 4):
// HeapObject: map (4)
// JSReceiver: map (4) + properties_or_hash (4) = 8
// JSObject: map (4) + properties_or_hash (4) + elements (4) = 12
export const K_JS_RECEIVER_PROPERTIES_OR_HASH_OFFSET = 4;
export const K_JS_RECEIVER_HEADER_SIZE = 8;

export const K_JS_OBJECT_ELEMENTS_OFFSET = 8;
export const K_JS_OBJECT_HEADER_SIZE = 12;
export const K_JS_OBJECT_END_OF_STRONG_FIELDS_OFFSET = 12;

export const K_MAX_INSTANCE_SIZE = 255 * 4; // 1020
export const K_MAX_IN_OBJECT_PROPERTIES = (K_MAX_INSTANCE_SIZE - K_JS_OBJECT_HEADER_SIZE) >> 2; // (1020 - 12) / 4 = 252

console.log("[js-objects.h.js] 提取与锚点校验通过:");
console.log({
  K_MAX_ELEMENT_COUNT,
  K_MAX_ELEMENT_INDEX,
  K_MAX_GAP,
  K_MAX_UNCHECKED_FAST_ELEMENTS_LENGTH,
  K_MAX_UNCHECKED_OLD_FAST_ELEMENTS_LENGTH,
  K_INITIAL_GLOBAL_OBJECT_UNUSED_PROPERTIES_COUNT,
  K_FIELDS_ADDED,
  K_JS_RECEIVER_PROPERTIES_OR_HASH_OFFSET,
  K_JS_RECEIVER_HEADER_SIZE,
  K_JS_OBJECT_ELEMENTS_OFFSET,
  K_JS_OBJECT_HEADER_SIZE,
  K_JS_OBJECT_END_OF_STRONG_FIELDS_OFFSET,
  K_MAX_INSTANCE_SIZE,
  K_MAX_IN_OBJECT_PROPERTIES,
});
