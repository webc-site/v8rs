#!/usr/bin/env bun
// tool/src/objects/ordered-hash-table.h.js — 校验并提取 OrderedHashTable / SmallOrderedHashTable 布局与常量
// 来源:
//   v8/src/objects/ordered-hash-table.h + ordered-hash-table-inl.h
//   → 对应 v8-objects-collections/src/ordered_hash_table.rs 与 small_ordered_hash_table.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/ordered-hash-table.h");
const inl = readV8("src/objects/ordered-hash-table-inl.h");

function mustContain(content, substr, filename) {
  if (!content.includes(substr)) {
    throw new Error(`锚点失配: ${filename} 缺少 "${substr.slice(0, 48)}..."`);
  }
}

// ── 1. OrderedHashTable 核心锚点检验 ──────────────────────────────────────────
mustContain(h, "template <class Derived, int entrysize>", "ordered-hash-table.h");
mustContain(h, "static const int kLoadFactor = 2;", "ordered-hash-table.h");
mustContain(h, "static const int kNotFound = -1;", "ordered-hash-table.h");
mustContain(h, "static const int kInitialCapacity = 4;", "ordered-hash-table.h");
mustContain(h, "static const int kClearedTableSentinel = -1;", "ordered-hash-table.h");
mustContain(h, "static constexpr int PrefixIndex() { return 0; }", "ordered-hash-table.h");
mustContain(h, "static constexpr int NumberOfElementsIndex() { return Derived::kPrefixSize; }", "ordered-hash-table.h");
mustContain(h, "static constexpr int NextTableIndex() { return NumberOfElementsIndex(); }", "ordered-hash-table.h");
mustContain(h, "static constexpr int NumberOfDeletedElementsIndex()", "ordered-hash-table.h");
mustContain(h, "static constexpr int NumberOfBucketsIndex()", "ordered-hash-table.h");
mustContain(h, "static constexpr int HashTableStartIndex()", "ordered-hash-table.h");

// OrderedHashSet & OrderedHashMap
mustContain(h, "class V8_EXPORT_PRIVATE OrderedHashSet", "ordered-hash-table.h");
mustContain(h, "class V8_EXPORT_PRIVATE OrderedHashMap", "ordered-hash-table.h");
mustContain(h, "static const int kValueOffset = 1;", "ordered-hash-table.h");

// SmallOrderedHashTable 核心锚点检验
mustContain(h, "class SmallOrderedHashTable : public HeapObject", "ordered-hash-table.h");
mustContain(h, "static const int kMinCapacity = 4;", "ordered-hash-table.h");
mustContain(h, "static const uint8_t kNotFound = 0xFF;", "ordered-hash-table.h");
mustContain(h, "static const int kMaxCapacity = 254;", "ordered-hash-table.h");
mustContain(h, "static const int kGrowthHack = 256;", "ordered-hash-table.h");

// SmallOrderedHashSet & SmallOrderedHashMap
mustContain(h, "class SmallOrderedHashSet", "ordered-hash-table.h");
mustContain(h, "class SmallOrderedHashMap", "ordered-hash-table.h");

// ── 2. 导出常量与计算方法 ──────────────────────────────────────────────────
export const ORDERED_HASH_TABLE = {
  kLoadFactor: 2,
  kNotFound: -1,
  kInitialCapacity: 4,
  kClearedTableSentinel: -1,
  prefixIndex: 0,
  numberOfElementsIndex: (prefixSize) => prefixSize,
  numberOfDeletedElementsIndex: (prefixSize) => prefixSize + 1,
  numberOfBucketsIndex: (prefixSize) => prefixSize + 2,
  hashTableStartIndex: (prefixSize) => prefixSize + 3,
  calcCapacity: (numberOfBuckets) => numberOfBuckets * 2,
  hashToBucket: (hash, numberOfBuckets) => hash & (numberOfBuckets - 1),
};

export const ORDERED_HASH_SET = {
  kEntrySizeWithoutChain: 1,
  kEntrySize: 2,
  kChainOffset: 1,
  kPrefixSize: 0,
};

export const ORDERED_HASH_MAP = {
  kEntrySizeWithoutChain: 2,
  kEntrySize: 3,
  kValueOffset: 1,
  kChainOffset: 2,
  kPrefixSize: 0,
};

export const SMALL_ORDERED_HASH_TABLE = {
  kMinCapacity: 4,
  kMaxCapacity: 254,
  kNotFound: 0xFF,
  kLoadFactor: 2,
  kGrowthHack: 256,
};

console.log("[ordered-hash-table.h.js] 提取与锚点校验通过:");
console.log({
  ORDERED_HASH_TABLE,
  ORDERED_HASH_SET,
  ORDERED_HASH_MAP,
  SMALL_ORDERED_HASH_TABLE,
});
