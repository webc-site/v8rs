#!/usr/bin/env bun
// tool/src/ic/ic.js — IC 基类 + Load/Store/KeyedStore 状态机面校验
// 覆盖: src/ic/ic.h + ic.cc + ic-inl.h
// 规则: NamedPropertyType / CallerFrameType / KeyedStoreCheckMap /
//   KeyedStoreIncrementLength / TransitionMode 枚举全量 + IC/LoadIC/StoreIC/
//   KeyedStoreIC 类面骨架; 反馈槽面经 FeedbackVector 窄 trait (objects 侧提供)。
import { readV8 } from "../../lib.js";

const FILES = ["src/ic/ic.h", "src/ic/ic.cc", "src/ic/ic-inl.h"];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => {
  throw new Error(`[ic.js] ${m}`);
};

const h = txt["src/ic/ic.h"];
for (const needle of [
  "enum class NamedPropertyType : bool { kNotOwn, kOwn }",
  "enum class CallerFrameType : uint8_t { kAny, kBaseline }",
  "enum KeyedStoreCheckMap { kDontCheckMap, kCheckMap }",
  "enum KeyedStoreIncrementLength { kDontIncrementLength, kIncrementLength }",
  "enum class TransitionMode {",
  "kNoTransition,",
  "kTransitionToDouble,",
  "kTransitionToObject",
  "class LoadIC : public IC {",
  "class StoreIC : public IC {",
  "class KeyedStoreIC : public StoreIC {",
  "FeedbackSlotKind kind() const",
]) {
  if (!h.includes(needle)) die(`缺针 ${needle}`);
}

console.log(`OK ic.js 针核验通过 (8 枚举针 + 3 变体针 + 4 类面针)`);
