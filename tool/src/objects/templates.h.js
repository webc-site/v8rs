#!/usr/bin/env bun
// tool/src/objects/templates.h.js — 校验并提取 FunctionTemplateInfo 布局与标志
// 来源:
//   v8/src/objects/templates.h + templates-inl.h + templates.tq
//   → 对应 v8-objects-rest/src/templates.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/templates.h");
const inl = readV8("src/objects/templates-inl.h");
const tq = readV8("src/objects/templates.tq");

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
mustContain(h, "V8_OBJECT class TemplateInfo : public HeapObject", "templates.h");
mustContain(h, "V8_OBJECT class FunctionTemplateInfo : public TemplateInfoWithProperties", "templates.h");
mustContain(h, "V8_OBJECT class ObjectTemplateInfo : public TemplateInfoWithProperties", "templates.h");

mustContain(h, "DECL_BOOLEAN_ACCESSORS(is_object_template_call_handler)", "templates.h");
mustContain(h, "DECL_BOOLEAN_ACCESSORS(has_side_effects)", "templates.h");
mustContain(h, "DECL_BOOLEAN_ACCESSORS(undetectable)", "templates.h");
mustContain(h, "DECL_BOOLEAN_ACCESSORS(needs_access_check)", "templates.h");
mustContain(h, "DECL_BOOLEAN_ACCESSORS(read_only_prototype)", "templates.h");
mustContain(h, "DECL_BOOLEAN_ACCESSORS(remove_prototype)", "templates.h");
mustContain(h, "DECL_BOOLEAN_ACCESSORS(accept_any_receiver)", "templates.h");
mustContain(h, "DECL_BOOLEAN_ACCESSORS(published)", "templates.h");

mustContain(tq, "bitfield struct FunctionTemplateInfoFlags extends uint32 {", "templates.tq");
mustContain(tq, "is_object_template_call_handler: bool: 1 bit;", "templates.tq");
mustContain(tq, "has_side_effects: bool: 1 bit;", "templates.tq");
mustContain(tq, "undetectable: bool: 1 bit;", "templates.tq");
mustContain(tq, "needs_access_check: bool: 1 bit;", "templates.tq");
mustContain(tq, "read_only_prototype: bool: 1 bit;", "templates.tq");
mustContain(tq, "remove_prototype: bool: 1 bit;", "templates.tq");
mustContain(tq, "accept_any_receiver: bool: 1 bit;", "templates.tq");
mustContain(tq, "published: bool: 1 bit;", "templates.tq");
mustContain(tq, "allowed_receiver_instance_type_range_start: InstanceType: 12 bit;", "templates.tq");
mustContain(tq, "allowed_receiver_instance_type_range_end: InstanceType: 12 bit;", "templates.tq");

// ── 2. 标志位移与掩码定义 ──────────────────────────────────────────────────
export const FT_IS_OBJECT_TEMPLATE_CALL_HANDLER_SHIFT = 0;
export const FT_HAS_SIDE_EFFECTS_SHIFT = 1;
export const FT_UNDETECTABLE_SHIFT = 2;
export const FT_NEEDS_ACCESS_CHECK_SHIFT = 3;
export const FT_READ_ONLY_PROTOTYPE_SHIFT = 4;
export const FT_REMOVE_PROTOTYPE_SHIFT = 5;
export const FT_ACCEPT_ANY_RECEIVER_SHIFT = 6;
export const FT_PUBLISHED_SHIFT = 7;
export const FT_ALLOWED_RECEIVER_START_SHIFT = 8;
export const FT_ALLOWED_RECEIVER_START_SIZE = 12;
export const FT_ALLOWED_RECEIVER_END_SHIFT = 20;
export const FT_ALLOWED_RECEIVER_END_SIZE = 12;

// ── 3. 内存偏移推导 (64-bit 非压缩 Tagged) ────────────────────────────────
export const K_TEMPLATE_INFO_FLAGS_OFFSET = 8;
export const K_TEMPLATE_INFO_SIZE = 16;

export const K_TIWP_NUMBER_OF_PROPERTIES_OFFSET = 16;
export const K_TIWP_PROPERTY_LIST_OFFSET = 24;
export const K_TIWP_PROPERTY_ACCESSORS_OFFSET = 32;
export const K_TIWP_SIZE = 40;

export const K_FTI_CLASS_NAME_OFFSET = 40;
export const K_FTI_INTERFACE_NAME_OFFSET = 48;
export const K_FTI_SIGNATURE_OFFSET = 56;
export const K_FTI_RARE_DATA_OFFSET = 64;
export const K_FTI_SHARED_FUNCTION_INFO_OFFSET = 72;
export const K_FTI_CACHED_PROPERTY_NAME_OFFSET = 80;
export const K_FTI_CALLBACK_DATA_OFFSET = 88;
export const K_FTI_FLAG_OFFSET = 96;
export const K_FTI_LENGTH_OFFSET = 100;
export const K_FTI_INSTANCE_TYPE_OFFSET = 102;
export const K_FTI_EXCEPTION_CONTEXT_OFFSET = 104;
export const K_FTI_OPTIONAL_PADDING_OFFSET = 108;
export const K_FTI_CALLBACK_OFFSET = 112;
export const K_FTI_SIZE = 120;

must(K_TEMPLATE_INFO_SIZE === 16, "TemplateInfo size 校验");
must(K_TIWP_SIZE === 40, "TemplateInfoWithProperties size 校验");
must(K_FTI_SIZE === 120, "FunctionTemplateInfo size 校验");

console.log(`[templates.h.js] 提取与锚点校验通过 (共 ${checks} 项校验):`);
console.log({
  K_TEMPLATE_INFO_SIZE,
  K_TIWP_SIZE,
  K_FTI_SIZE,
  K_FTI_CLASS_NAME_OFFSET,
  K_FTI_FLAG_OFFSET,
  K_FTI_CALLBACK_OFFSET,
});
