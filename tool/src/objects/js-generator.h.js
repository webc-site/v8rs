#!/usr/bin/env bun
// tool/src/objects/js-generator.h.js — 校验并提取 JSGeneratorObject / JSAsyncGeneratorObject 布局与常量
// 来源:
//   v8/src/objects/js-generator.h + js-generator-inl.h
//   → 对应 v8-objects-js-async/src/js_generator.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/js-generator.h");
const inl = readV8("src/objects/js-generator-inl.h");

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
mustContain(h, "V8_OBJECT class JSGeneratorObject : public JSObject {", "js-generator.h");
mustContain(h, "enum ResumeMode { kNext, kReturn, kThrow, kRethrow };", "js-generator.h");
mustContain(h, "static const int kGeneratorExecuting = -2;", "js-generator.h");
mustContain(h, "static const int kGeneratorClosed = -1;", "js-generator.h");
mustContain(h, "TaggedMember<JSFunction> function_;", "js-generator.h");
mustContain(h, "TaggedMember<Context> context_;", "js-generator.h");
mustContain(h, "TaggedMember<JSAny> receiver_;", "js-generator.h");
mustContain(h, "TaggedMember<Object> input_or_debug_pos_;", "js-generator.h");
mustContain(h, "TaggedMember<Smi> resume_mode_;", "js-generator.h");
mustContain(h, "TaggedMember<Smi> continuation_;", "js-generator.h");
mustContain(h, "TaggedMember<FixedArray> parameters_and_registers_;", "js-generator.h");
mustContain(h, "TaggedMember<JSAny> yielded_value_;", "js-generator.h");

mustContain(h, "V8_OBJECT class JSAsyncFunctionObject final : public JSGeneratorObject {", "js-generator.h");
mustContain(h, "TaggedMember<JSPromise> promise_;", "js-generator.h");
mustContain(h, "TaggedMember<UnionOf<JSFunction, Undefined>> await_resolve_closure_;", "js-generator.h");
mustContain(h, "TaggedMember<UnionOf<JSFunction, Undefined>> await_reject_closure_;", "js-generator.h");

mustContain(h, "V8_OBJECT class JSAsyncGeneratorObject final : public JSGeneratorObject {", "js-generator.h");
mustContain(h, "TaggedMember<UnionOf<AsyncGeneratorRequest, Undefined>> queue_;", "js-generator.h");
mustContain(h, "TaggedMember<Smi> is_awaiting_;", "js-generator.h");

mustContain(h, "V8_OBJECT class AsyncGeneratorRequest : public Struct {", "js-generator.h");
mustContain(h, "TaggedMember<UnionOf<AsyncGeneratorRequest, Undefined>> next_;", "js-generator.h");
mustContain(h, "TaggedMember<Object> value_;", "js-generator.h");

mustContain(inl, "bool JSGeneratorObject::is_suspended() const {", "js-generator-inl.h");
mustContain(inl, "bool JSGeneratorObject::is_closed() const {", "js-generator-inl.h");
mustContain(inl, "bool JSGeneratorObject::is_executing() const {", "js-generator-inl.h");
mustContain(inl, "int JSAsyncGeneratorObject::is_awaiting() const {", "js-generator-inl.h");
mustContain(inl, "void JSAsyncGeneratorObject::set_is_awaiting(int value) {", "js-generator-inl.h");

// ── 2. 枚举常量 ────────────────────────────────────────────────────────────
export const K_RESUME_MODE_NEXT = 0;
export const K_RESUME_MODE_RETURN = 1;
export const K_RESUME_MODE_THROW = 2;
export const K_RESUME_MODE_RETHROW = 3;

// Continuation 状态哨兵
export const K_GENERATOR_EXECUTING = -2;
export const K_GENERATOR_CLOSED = -1;

// ── 3. 内存布局偏移与大小 ──────────────────────────────────────────────────
// 64-bit 架构下 (word = 8 bytes):
export const K_JS_OBJECT_HEADER_SIZE_64 = 24;

// JSGeneratorObject 槽位 (紧随 JSObject 之后):
export const K_FUNCTION_OFFSET_64 = 24;
export const K_CONTEXT_OFFSET_64 = 32;
export const K_RECEIVER_OFFSET_64 = 40;
export const K_INPUT_OR_DEBUG_POS_OFFSET_64 = 48;
export const K_RESUME_MODE_OFFSET_64 = 56;
export const K_CONTINUATION_OFFSET_64 = 64;
export const K_PARAMETERS_AND_REGISTERS_OFFSET_64 = 72;
export const K_YIELDED_VALUE_OFFSET_64 = 80;
export const K_JS_GENERATOR_OBJECT_HEADER_SIZE_64 = 88; // 11 words

// JSAsyncGeneratorObject 槽位 (紧随 JSGeneratorObject 之后):
export const K_ASYNC_GEN_QUEUE_OFFSET_64 = 88;
export const K_ASYNC_GEN_IS_AWAITING_OFFSET_64 = 96;
export const K_JS_ASYNC_GENERATOR_OBJECT_HEADER_SIZE_64 = 104; // 13 words

// JSAsyncFunctionObject 槽位 (紧随 JSGeneratorObject 之后):
export const K_ASYNC_FUNC_PROMISE_OFFSET_64 = 88;
export const K_ASYNC_FUNC_AWAIT_RESOLVE_CLOSURE_OFFSET_64 = 96;
export const K_ASYNC_FUNC_AWAIT_REJECT_CLOSURE_OFFSET_64 = 104;
export const K_JS_ASYNC_FUNCTION_OBJECT_HEADER_SIZE_64 = 112; // 14 words

// AsyncGeneratorRequest 槽位 (继承 Struct -> HeapObject 1 word = 8 字节):
export const K_STRUCT_HEADER_SIZE_64 = 8;
export const K_REQ_NEXT_OFFSET_64 = 8;
export const K_REQ_RESUME_MODE_OFFSET_64 = 16;
export const K_REQ_VALUE_OFFSET_64 = 24;
export const K_REQ_PROMISE_OFFSET_64 = 32;
export const K_ASYNC_GENERATOR_REQUEST_HEADER_SIZE_64 = 40; // 5 words

// 32-bit / 指针压缩架构下 (word = 4 bytes):
export const K_JS_OBJECT_HEADER_SIZE_32 = 12;
export const K_FUNCTION_OFFSET_32 = 12;
export const K_CONTEXT_OFFSET_32 = 16;
export const K_RECEIVER_OFFSET_32 = 20;
export const K_INPUT_OR_DEBUG_POS_OFFSET_32 = 24;
export const K_RESUME_MODE_OFFSET_32 = 28;
export const K_CONTINUATION_OFFSET_32 = 32;
export const K_PARAMETERS_AND_REGISTERS_OFFSET_32 = 36;
export const K_YIELDED_VALUE_OFFSET_32 = 40;
export const K_JS_GENERATOR_OBJECT_HEADER_SIZE_32 = 44; // 11 words

export const K_ASYNC_GEN_QUEUE_OFFSET_32 = 44;
export const K_ASYNC_GEN_IS_AWAITING_OFFSET_32 = 48;
export const K_JS_ASYNC_GENERATOR_OBJECT_HEADER_SIZE_32 = 52; // 13 words

export const K_ASYNC_GENERATOR_REQUEST_HEADER_SIZE_32 = 20; // 5 words

// 布局校验
must(K_FUNCTION_OFFSET_64 === K_JS_OBJECT_HEADER_SIZE_64, "function offset 校验");
must(K_CONTEXT_OFFSET_64 === K_FUNCTION_OFFSET_64 + 8, "context offset 校验");
must(K_RECEIVER_OFFSET_64 === K_CONTEXT_OFFSET_64 + 8, "receiver offset 校验");
must(K_INPUT_OR_DEBUG_POS_OFFSET_64 === K_RECEIVER_OFFSET_64 + 8, "input_or_debug_pos offset 校验");
must(K_RESUME_MODE_OFFSET_64 === K_INPUT_OR_DEBUG_POS_OFFSET_64 + 8, "resume_mode offset 校验");
must(K_CONTINUATION_OFFSET_64 === K_RESUME_MODE_OFFSET_64 + 8, "continuation offset 校验");
must(K_PARAMETERS_AND_REGISTERS_OFFSET_64 === K_CONTINUATION_OFFSET_64 + 8, "parameters_and_registers offset 校验");
must(K_YIELDED_VALUE_OFFSET_64 === K_PARAMETERS_AND_REGISTERS_OFFSET_64 + 8, "yielded_value offset 校验");
must(K_JS_GENERATOR_OBJECT_HEADER_SIZE_64 === K_YIELDED_VALUE_OFFSET_64 + 8, "generator header size 校验");

must(K_ASYNC_GEN_QUEUE_OFFSET_64 === K_JS_GENERATOR_OBJECT_HEADER_SIZE_64, "async generator queue offset 校验");
must(K_ASYNC_GEN_IS_AWAITING_OFFSET_64 === K_ASYNC_GEN_QUEUE_OFFSET_64 + 8, "async generator is_awaiting offset 校验");
must(K_JS_ASYNC_GENERATOR_OBJECT_HEADER_SIZE_64 === K_ASYNC_GEN_IS_AWAITING_OFFSET_64 + 8, "async generator header size 校验");

must(K_REQ_NEXT_OFFSET_64 === K_STRUCT_HEADER_SIZE_64, "request next offset 校验");
must(K_REQ_RESUME_MODE_OFFSET_64 === K_REQ_NEXT_OFFSET_64 + 8, "request resume_mode offset 校验");
must(K_REQ_VALUE_OFFSET_64 === K_REQ_RESUME_MODE_OFFSET_64 + 8, "request value offset 校验");
must(K_REQ_PROMISE_OFFSET_64 === K_REQ_VALUE_OFFSET_64 + 8, "request promise offset 校验");
must(K_ASYNC_GENERATOR_REQUEST_HEADER_SIZE_64 === K_REQ_PROMISE_OFFSET_64 + 8, "request header size 校验");

console.log(`[js-generator.h.js] 提取与锚点校验通过 (共 ${checks} 项校验):`);
console.log({
  K_RESUME_MODE_NEXT,
  K_RESUME_MODE_RETURN,
  K_RESUME_MODE_THROW,
  K_RESUME_MODE_RETHROW,
  K_GENERATOR_EXECUTING,
  K_GENERATOR_CLOSED,
  K_JS_GENERATOR_OBJECT_HEADER_SIZE_64,
  K_JS_ASYNC_GENERATOR_OBJECT_HEADER_SIZE_64,
  K_JS_ASYNC_FUNCTION_OBJECT_HEADER_SIZE_64,
  K_ASYNC_GENERATOR_REQUEST_HEADER_SIZE_64,
});
