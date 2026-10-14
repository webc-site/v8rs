#!/usr/bin/env bun
// tool/src/objects/js-function.h.js — 校验并提取 JSFunction 族布局与常量
// 来源:
//   v8/src/objects/js-function.h + js-function-inl.h
//   → 对应 v8-objects-js-func/src/js_function.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/js-function.h");
const inl = readV8("src/objects/js-function-inl.h");

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
mustContain(h, "V8_OBJECT class JSFunctionOrBoundFunctionOrWrappedFunction : public JSObject {", "js-function.h");
mustContain(h, "V8_OBJECT class JSBoundFunction", "js-function.h");
mustContain(h, "V8_OBJECT class JSWrappedFunction", "js-function.h");
mustContain(h, "V8_OBJECT class JSFunction : public JSFunctionOrBoundFunctionOrWrappedFunction {", "js-function.h");
mustContain(h, "JSDispatchHandleMember dispatch_handle_;", "js-function.h");
mustContain(h, "TaggedMember<SharedFunctionInfo> shared_function_info_;", "js-function.h");
mustContain(h, "TaggedMember<Context> context_;", "js-function.h");
mustContain(h, "TaggedMember<FeedbackCell> feedback_cell_;", "js-function.h");
mustContain(h, "TaggedMember<UnionOf<JSReceiver, Map, Tuple2, TheHole>>", "js-function.h");
mustContain(h, "V8_OBJECT class JSFunctionWithoutPrototype : public JSFunction {", "js-function.h");
mustContain(h, "V8_OBJECT class JSFunctionWithPrototype : public JSFunction {", "js-function.h");

mustContain(inl, "Tagged<FeedbackCell> JSFunction::raw_feedback_cell() const", "js-function-inl.h");
mustContain(inl, "void JSFunction::set_raw_feedback_cell(", "js-function-inl.h");
mustContain(inl, "Tagged<Context> JSFunction::context(AcquireLoadTag) const", "js-function-inl.h");
mustContain(inl, "void JSFunction::set_context(Tagged<Context> value", "js-function-inl.h");
mustContain(inl, "Tagged<Code> JSFunction::code(IsolateForSandbox isolate) const", "js-function-inl.h");
mustContain(inl, "void JSFunction::UpdateCode(", "js-function-inl.h");

// ── 2. 布局常量计算 ──────────────────────────────────────────────────────
// JSObject 占 3 words (map, properties_or_hash, elements)
// 64-bit 架构下 (word = 8 bytes):
// JSObject 大小 = 24 字节
// JSFunction:
//   dispatch_handle_ (u32, 4 bytes) + padding_ (u32, 4 bytes) = 8 bytes (offset 24)
//   shared_function_info_ (Tagged, 8 bytes) = offset 32
//   context_ (Tagged, 8 bytes) = offset 40
//   feedback_cell_ (Tagged, 8 bytes) = offset 48
//   JSFunction::kHeaderSize = 56 bytes (7 words)
// JSFunctionWithPrototype:
//   prototype_or_initial_map_ (Tagged, 8 bytes) = offset 56
//   kHeaderSize = 64 bytes (8 words)

export const K_JS_OBJECT_HEADER_SIZE_64 = 24;
export const K_DISPATCH_HANDLE_OFFSET_64 = 24;
export const K_SHARED_FUNCTION_INFO_OFFSET_64 = 32;
export const K_CONTEXT_OFFSET_64 = 40;
export const K_FEEDBACK_CELL_OFFSET_64 = 48;
export const K_JS_FUNCTION_HEADER_SIZE_64 = 56;
export const K_PROTOTYPE_OR_INITIAL_MAP_OFFSET_64 = 56;
export const K_JS_FUNCTION_WITH_PROTOTYPE_HEADER_SIZE_64 = 64;

// 32-bit / 指针压缩架构下 (word = 4 bytes):
// JSObject 大小 = 12 字节
// JSFunction:
//   dispatch_handle_ (u32, 4 bytes) = offset 12
//   shared_function_info_ (Tagged, 4 bytes) = offset 16
//   context_ (Tagged, 4 bytes) = offset 20
//   feedback_cell_ (Tagged, 4 bytes) = offset 24
//   kHeaderSize = 28 bytes
export const K_JS_OBJECT_HEADER_SIZE_32 = 12;
export const K_DISPATCH_HANDLE_OFFSET_32 = 12;
export const K_SHARED_FUNCTION_INFO_OFFSET_32 = 16;
export const K_CONTEXT_OFFSET_32 = 20;
export const K_FEEDBACK_CELL_OFFSET_32 = 24;
export const K_JS_FUNCTION_HEADER_SIZE_32 = 28;

must(K_DISPATCH_HANDLE_OFFSET_64 === K_JS_OBJECT_HEADER_SIZE_64, "dispatch_handle offset 校验");
must(K_SHARED_FUNCTION_INFO_OFFSET_64 === K_DISPATCH_HANDLE_OFFSET_64 + 8, "shared_function_info offset 校验");
must(K_CONTEXT_OFFSET_64 === K_SHARED_FUNCTION_INFO_OFFSET_64 + 8, "context offset 校验");
must(K_FEEDBACK_CELL_OFFSET_64 === K_CONTEXT_OFFSET_64 + 8, "feedback_cell offset 校验");
must(K_JS_FUNCTION_HEADER_SIZE_64 === K_FEEDBACK_CELL_OFFSET_64 + 8, "header size 校验");

console.log(`[js-function.h.js] 提取与锚点校验通过 (共 ${checks} 项校验):`);
console.log({
  K_JS_OBJECT_HEADER_SIZE_64,
  K_DISPATCH_HANDLE_OFFSET_64,
  K_SHARED_FUNCTION_INFO_OFFSET_64,
  K_CONTEXT_OFFSET_64,
  K_FEEDBACK_CELL_OFFSET_64,
  K_JS_FUNCTION_HEADER_SIZE_64,
  K_PROTOTYPE_OR_INITIAL_MAP_OFFSET_64,
  K_JS_FUNCTION_WITH_PROTOTYPE_HEADER_SIZE_64,
});
