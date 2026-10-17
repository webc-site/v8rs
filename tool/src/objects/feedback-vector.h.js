#!/usr/bin/env bun
// tool/src/objects/feedback-vector.h.js — 校验并提取 FeedbackCell 与 FeedbackVector 布局
// 来源:
//   v8/src/objects/feedback-vector.h + feedback-vector-inl.h + feedback-vector.tq
//   v8/src/objects/feedback-cell.h + feedback-cell-inl.h + feedback-cell.tq
//   → 对应 v8-objects-rest/src/feedback_cell.rs 与 src/feedback_vector.rs
import { readV8 } from "../../lib.js";

const fvH = readV8("src/objects/feedback-vector.h");
const fvInl = readV8("src/objects/feedback-vector-inl.h");
const fvTq = readV8("src/objects/feedback-vector.tq");

const fcH = readV8("src/objects/feedback-cell.h");
const fcInl = readV8("src/objects/feedback-cell-inl.h");
const fcTq = readV8("src/objects/feedback-cell.tq");

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

// ── 1. FeedbackCell 锚点检验 ───────────────────────────────────────────────
mustContain(fcH, "V8_OBJECT class FeedbackCell : public Struct", "feedback-cell.h");
mustContain(fcH, "TaggedMember<Value> value_;", "feedback-cell.h");
mustContain(fcH, "JSDispatchHandleMember dispatch_handle_;", "feedback-cell.h");
mustContain(fcH, "int32_t interrupt_budget_ = 0;", "feedback-cell.h");
mustContain(fcH, "static_assert(sizeof(FeedbackCell) ==", "feedback-cell.h");

mustContain(fcInl, "int32_t FeedbackCell::interrupt_budget() const", "feedback-cell-inl.h");
mustContain(fcInl, "void FeedbackCell::set_interrupt_budget(int32_t value)", "feedback-cell-inl.h");

// ── 2. FeedbackVector 锚点检验 ─────────────────────────────────────────────
mustContain(fvH, "V8_OBJECT class FeedbackVector : public HeapObject", "feedback-vector.h");
mustContain(fvH, "const int32_t length_;", "feedback-vector.h");
mustContain(fvH, "std::atomic<int32_t> invocation_count_ = 0;", "feedback-vector.h");
mustContain(fvH, "std::atomic<uint8_t> invocation_count_before_stable_ = 0;", "feedback-vector.h");
mustContain(fvH, "uint8_t osr_state_ V8_TQ_TYPE(OsrState) = 0;", "feedback-vector.h");
mustContain(fvH, "uint16_t flags_ V8_TQ_TYPE(FeedbackVectorFlags) = 0;", "feedback-vector.h");
mustContain(fvH, "const TaggedMember<SharedFunctionInfo> shared_function_info_;", "feedback-vector.h");
mustContain(fvH, "const TaggedMember<ClosureFeedbackCellArray> closure_feedback_cell_array_;", "feedback-vector.h");
mustContain(fvH, "const TaggedMember<FeedbackCell> parent_feedback_cell_;", "feedback-vector.h");
mustContain(fvH, "FLEXIBLE_ARRAY_MEMBER(TaggedMember<MaybeObject>, raw_feedback_slots", "feedback-vector.h");

// ── 3. FeedbackCell 内存偏移 (64-bit 非压缩 Tagged) ───────────────────────
export const K_FEEDBACK_CELL_VALUE_OFFSET = 8;
export const K_FEEDBACK_CELL_DISPATCH_HANDLE_OFFSET = 16;
export const K_FEEDBACK_CELL_INTERRUPT_BUDGET_OFFSET = 20;
export const K_FEEDBACK_CELL_SIZE = 24;

must(K_FEEDBACK_CELL_VALUE_OFFSET === 8, "fc value_ offset 校验");
must(K_FEEDBACK_CELL_DISPATCH_HANDLE_OFFSET === 16, "fc dispatch_handle_ offset 校验");
must(K_FEEDBACK_CELL_INTERRUPT_BUDGET_OFFSET === 20, "fc interrupt_budget_ offset 校验");
must(K_FEEDBACK_CELL_SIZE === 24, "fc size 校验");

// ── 4. FeedbackVector 内存偏移 (64-bit 非压缩 Tagged) ─────────────────────
export const K_FEEDBACK_VECTOR_LENGTH_OFFSET = 8;
export const K_FEEDBACK_VECTOR_INVOCATION_COUNT_OFFSET = 12;
export const K_FEEDBACK_VECTOR_OPTIONAL_PADDING_OFFSET = 16;
export const K_FEEDBACK_VECTOR_INVOCATION_COUNT_BEFORE_STABLE_OFFSET = 20;
export const K_FEEDBACK_VECTOR_OSR_STATE_OFFSET = 21;
export const K_FEEDBACK_VECTOR_FLAGS_OFFSET = 22;
export const K_FEEDBACK_VECTOR_SHARED_FUNCTION_INFO_OFFSET = 24;
export const K_FEEDBACK_VECTOR_CLOSURE_FEEDBACK_CELL_ARRAY_OFFSET = 32;
export const K_FEEDBACK_VECTOR_PARENT_FEEDBACK_CELL_OFFSET = 40;
export const K_FEEDBACK_VECTOR_HEADER_SIZE = 48;
export const K_FEEDBACK_VECTOR_RAW_FEEDBACK_SLOTS_OFFSET = 48;

must(K_FEEDBACK_VECTOR_HEADER_SIZE === 48, "fv header_size 校验");
must(K_FEEDBACK_VECTOR_RAW_FEEDBACK_SLOTS_OFFSET === 48, "fv raw slots offset 校验");

console.log(`[feedback-vector.h.js] 提取与锚点校验通过 (共 ${checks} 项校验):`);
console.log({
  K_FEEDBACK_CELL_SIZE,
  K_FEEDBACK_CELL_VALUE_OFFSET,
  K_FEEDBACK_CELL_INTERRUPT_BUDGET_OFFSET,
  K_FEEDBACK_VECTOR_HEADER_SIZE,
  K_FEEDBACK_VECTOR_RAW_FEEDBACK_SLOTS_OFFSET,
});
