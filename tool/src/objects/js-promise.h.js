#!/usr/bin/env bun
// tool/src/objects/js-promise.h.js — 校验并提取 JSPromise 布局与常量
// 来源:
//   v8/src/objects/js-promise.h + js-promise-inl.h
//   → 对应 v8-objects-js-async/src/js_promise.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/js-promise.h");
const inl = readV8("src/objects/js-promise-inl.h");

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
mustContain(h, "V8_OBJECT class JSPromise : public JSObjectWithEmbedderSlots {", "js-promise.h");
mustContain(h, "static constexpr uint32_t kInvalidAsyncTaskId = 0;", "js-promise.h");
mustContain(h, "TaggedMember<UnionOf<PromiseReaction, JSAny>> reactions_or_result_;", "js-promise.h");
mustContain(h, "TaggedMember<Smi> flags_ V8_TQ_TYPE(SmiTagged<JSPromiseFlags>);", "js-promise.h");
mustContain(h, "using StatusBits = base::BitField<Promise::PromiseState, 0, 2, uint32_t>;", "js-promise.h");
mustContain(h, "using IsNativeResolverInvokedBit = StatusBits::Next<bool, 1>;", "js-promise.h");
mustContain(h, "using HasHandlerBit = IsNativeResolverInvokedBit::Next<bool, 1>;", "js-promise.h");
mustContain(h, "using IsSilentBit = HasHandlerBit::Next<bool, 1>;", "js-promise.h");
mustContain(h, "using AsyncTaskIdBits = IsSilentBit::Next<uint32_t, 26>;", "js-promise.h");
mustContain(h, "static_assert(v8::Promise::kPending == 0);", "js-promise.h");
mustContain(h, "static_assert(v8::Promise::kFulfilled == 1);", "js-promise.h");
mustContain(h, "static_assert(v8::Promise::kRejected == 2);", "js-promise.h");

mustContain(inl, "Tagged<UnionOf<PromiseReaction, JSAny>> JSPromise::reactions_or_result() const {", "js-promise-inl.h");
mustContain(inl, "void JSPromise::set_reactions_or_result(", "js-promise-inl.h");
mustContain(inl, "int JSPromise::flags() const { return flags_.load().value(); }", "js-promise-inl.h");
mustContain(inl, "void JSPromise::set_flags(int value) {", "js-promise-inl.h");
mustContain(inl, "BOOL_ACCESSORS(JSPromise, flags, is_native_resolver_invoked,", "js-promise-inl.h");
mustContain(inl, "BOOL_ACCESSORS(JSPromise, flags, has_handler, HasHandlerBit::kShift)", "js-promise-inl.h");
mustContain(inl, "BOOL_ACCESSORS(JSPromise, flags, is_silent, IsSilentBit::kShift)", "js-promise-inl.h");
mustContain(inl, "uint32_t JSPromise::GetNextAsyncTaskId(uint32_t async_task_id) {", "js-promise-inl.h");
mustContain(inl, "Tagged<Object> JSPromise::result() const {", "js-promise-inl.h");
mustContain(inl, "Tagged<Object> JSPromise::reactions() const {", "js-promise-inl.h");

// ── 2. PromiseState 常量 ───────────────────────────────────────────────────
export const K_PROMISE_PENDING = 0;
export const K_PROMISE_FULFILLED = 1;
export const K_PROMISE_REJECTED = 2;

// ── 3. Flags 位域布局 ──────────────────────────────────────────────────────
// uint31 空间内:
// [0..1]  : Status (2 bits)
// [2]     : IsNativeResolverInvoked (1 bit)
// [3]     : HasHandler (1 bit)
// [4]     : IsSilent (1 bit)
// [5..30] : AsyncTaskId (26 bits)
export const K_STATUS_SHIFT = 0;
export const K_STATUS_BITS = 2;
export const K_STATUS_MASK = (1 << K_STATUS_BITS) - 1; // 0x3

export const K_IS_NATIVE_RESOLVER_INVOKED_SHIFT = K_STATUS_SHIFT + K_STATUS_BITS; // 2
export const K_IS_NATIVE_RESOLVER_INVOKED_BITS = 1;
export const K_IS_NATIVE_RESOLVER_INVOKED_MASK = 1 << K_IS_NATIVE_RESOLVER_INVOKED_SHIFT; // 0x4

export const K_HAS_HANDLER_SHIFT = K_IS_NATIVE_RESOLVER_INVOKED_SHIFT + K_IS_NATIVE_RESOLVER_INVOKED_BITS; // 3
export const K_HAS_HANDLER_BITS = 1;
export const K_HAS_HANDLER_MASK = 1 << K_HAS_HANDLER_SHIFT; // 0x8

export const K_IS_SILENT_SHIFT = K_HAS_HANDLER_SHIFT + K_HAS_HANDLER_BITS; // 4
export const K_IS_SILENT_BITS = 1;
export const K_IS_SILENT_MASK = 1 << K_IS_SILENT_SHIFT; // 0x10

export const K_ASYNC_TASK_ID_SHIFT = K_IS_SILENT_SHIFT + K_IS_SILENT_BITS; // 5
export const K_ASYNC_TASK_ID_BITS = 26;
export const K_ASYNC_TASK_ID_MASK = ((1 << K_ASYNC_TASK_ID_BITS) - 1) << K_ASYNC_TASK_ID_SHIFT;
export const K_MAX_ASYNC_TASK_ID = (1 << K_ASYNC_TASK_ID_BITS) - 1; // 0x3FFFFFF = 67,108,863
export const K_INVALID_ASYNC_TASK_ID = 0;

must(K_ASYNC_TASK_ID_SHIFT + K_ASYNC_TASK_ID_BITS === 31, "flags 位域总位宽必须等于 31 位");

// ── 4. 内存槽位与大小 ──────────────────────────────────────────────────────
// JSObject 占 3 个 tagged 槽位: map, properties_or_hash, elements
// 64-bit 架构下 (word = 8 bytes):
export const K_JS_OBJECT_HEADER_SIZE_64 = 24;
export const K_REACTIONS_OR_RESULT_OFFSET_64 = 24;
export const K_FLAGS_OFFSET_64 = 32;
export const K_JS_PROMISE_HEADER_SIZE_64 = 40; // 5 words

// 32-bit / 指针压缩架构下 (word = 4 bytes):
export const K_JS_OBJECT_HEADER_SIZE_32 = 12;
export const K_REACTIONS_OR_RESULT_OFFSET_32 = 12;
export const K_FLAGS_OFFSET_32 = 16;
export const K_JS_PROMISE_HEADER_SIZE_32 = 20; // 5 words

must(K_REACTIONS_OR_RESULT_OFFSET_64 === K_JS_OBJECT_HEADER_SIZE_64, "reactions_or_result 偏移校验");
must(K_FLAGS_OFFSET_64 === K_REACTIONS_OR_RESULT_OFFSET_64 + 8, "flags 偏移校验");
must(K_JS_PROMISE_HEADER_SIZE_64 === K_FLAGS_OFFSET_64 + 8, "header size 校验");

console.log(`[js-promise.h.js] 提取与锚点校验通过 (共 ${checks} 项校验):`);
console.log({
  K_PROMISE_PENDING,
  K_PROMISE_FULFILLED,
  K_PROMISE_REJECTED,
  K_STATUS_SHIFT,
  K_STATUS_BITS,
  K_STATUS_MASK,
  K_IS_NATIVE_RESOLVER_INVOKED_SHIFT,
  K_HAS_HANDLER_SHIFT,
  K_IS_SILENT_SHIFT,
  K_ASYNC_TASK_ID_SHIFT,
  K_ASYNC_TASK_ID_BITS,
  K_MAX_ASYNC_TASK_ID,
  K_REACTIONS_OR_RESULT_OFFSET_64,
  K_FLAGS_OFFSET_64,
  K_JS_PROMISE_HEADER_SIZE_64,
});
