#!/usr/bin/env bun
// tool/src/objects/transitions.h.js — 校验并提取 TransitionArray 与 TransitionsAccessor 布局与常量
// 来源:
//   v8/src/objects/transitions.h + transitions-inl.h
//   → 对应 v8-objects-maps/src/transitions.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/transitions.h");
const inl = readV8("src/objects/transitions-inl.h");

function mustContain(content, substr, filename) {
  if (!content.includes(substr)) {
    throw new Error(`锚点失配: ${filename} 缺少 "${substr.slice(0, 48)}..."`);
  }
}

// ── 1. 核心锚点检验 ────────────────────────────────────────────────────────
mustContain(h, "enum class Kind : uint32_t {", "transitions.h");
mustContain(h, "kCloneObject,", "transitions.h");
mustContain(h, "kObjectAssign,", "transitions.h");
mustContain(h, "kObjectAssignValidityCell,", "transitions.h");
mustContain(h, "static const int kMaxNumberOfTransitions = 1024 + 512;", "transitions.h");
mustContain(h, "static constexpr int kMaxElementsForLinearSearch = 32;", "transitions.h");
mustContain(h, "static const int kPrototypeTransitionsIndex = 0;", "transitions.h");
mustContain(h, "static const int kSideStepTransitionsIndex = 1;", "transitions.h");
mustContain(h, "static const int kTransitionLengthIndex = 2;", "transitions.h");
mustContain(h, "static const uint32_t kFirstIndex = 3;", "transitions.h");
mustContain(h, "static const int kEntryKeyIndex = 0;", "transitions.h");
mustContain(h, "static const int kEntryTargetIndex = 1;", "transitions.h");
mustContain(h, "static const int kEntrySize = 2;", "transitions.h");
mustContain(h, "enum Encoding {", "transitions.h");
mustContain(h, "kFullTransitionArray,", "transitions.h");
mustContain(inl, "int TransitionArray::number_of_transitions() const", "transitions-inl.h");

// ── 2. 提取常量与布局 ──────────────────────────────────────────────────────
export const K_MAX_NUMBER_OF_TRANSITIONS = 1024 + 512;
export const K_MAX_ELEMENTS_FOR_LINEAR_SEARCH = 32;

export const K_PROTOTYPE_TRANSITIONS_INDEX = 0;
export const K_SIDE_STEP_TRANSITIONS_INDEX = 1;
export const K_TRANSITION_LENGTH_INDEX = 2;
export const K_FIRST_INDEX = 3;

export const K_ENTRY_KEY_INDEX = 0;
export const K_ENTRY_TARGET_INDEX = 1;
export const K_ENTRY_SIZE = 2;

export const SIDE_STEP_TRANSITION_KIND = {
  kCloneObject: 0,
  kObjectAssign: 1,
  kObjectAssignValidityCell: 2,
  kSize: 3,
};

export const ENCODING = {
  kPrototypeInfo: 0,
  kUninitialized: 1,
  kMigrationTarget: 2,
  kWeakRef: 3,
  kFullTransitionArray: 4,
  kPrototypeSharedClosureInfo: 5,
};

export function toKeyIndex(transitionNumber) {
  return K_FIRST_INDEX + transitionNumber * K_ENTRY_SIZE + K_ENTRY_KEY_INDEX;
}

export function toTargetIndex(transitionNumber) {
  return K_FIRST_INDEX + transitionNumber * K_ENTRY_SIZE + K_ENTRY_TARGET_INDEX;
}

console.log("[transitions.h.js] 提取与锚点校验通过:");
console.log({
  K_MAX_NUMBER_OF_TRANSITIONS,
  K_MAX_ELEMENTS_FOR_LINEAR_SEARCH,
  K_FIRST_INDEX,
  K_ENTRY_SIZE,
  SIDE_STEP_TRANSITION_KIND,
  ENCODING,
});
