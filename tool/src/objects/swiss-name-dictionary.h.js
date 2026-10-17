#!/usr/bin/env bun
// tool/src/objects/swiss-name-dictionary.h.js — 校验并提取 SwissNameDictionary 与 SwissTable 常量与布局
// 来源:
//   v8/src/objects/swiss-name-dictionary.h + swiss-name-dictionary-inl.h + swiss-hash-table-helpers.h
//   → 对应 v8-objects-collections/src/swiss_name_dictionary.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/swiss-name-dictionary.h");
const inl = readV8("src/objects/swiss-name-dictionary-inl.h");
const helpers = readV8("src/objects/swiss-hash-table-helpers.h");

function mustContain(content, substr, filename) {
  if (!content.includes(substr)) {
    throw new Error(`锚点失配: ${filename} 缺少 "${substr.slice(0, 48)}..."`);
  }
}

// ── 1. 控制字节与 H1/H2 哈希拆分检验 ───────────────────────────────────────
mustContain(helpers, "kEmpty = -128,", "swiss-hash-table-helpers.h");
mustContain(helpers, "kDeleted = -2,", "swiss-hash-table-helpers.h");
mustContain(helpers, "kSentinel = -1,", "swiss-hash-table-helpers.h");
mustContain(helpers, "static constexpr int kH2Bits = 7;", "swiss-hash-table-helpers.h");
mustContain(helpers, "static constexpr int kNotFullMask = (1 << kH2Bits);", "swiss-hash-table-helpers.h");
mustContain(helpers, "inline static uint32_t H1(uint32_t hash)", "swiss-hash-table-helpers.h");
mustContain(helpers, "inline static swiss_table::ctrl_t H2(uint32_t hash)", "swiss-hash-table-helpers.h");

// ── 2. SwissNameDictionary 布局与元表检验 ────────────────────────────────────
mustContain(h, "class V8_EXPORT_PRIVATE SwissNameDictionary : public HeapObject", "swiss-name-dictionary.h");
mustContain(h, "static constexpr uint32_t kDataTableEntryCount = 2;", "swiss-name-dictionary.h");
mustContain(h, "static constexpr uint32_t kDataTableKeyEntryIndex = 0;", "swiss-name-dictionary.h");
mustContain(h, "static constexpr uint32_t kDataTableValueEntryIndex =", "swiss-name-dictionary.h");
mustContain(h, "static constexpr uint32_t kMetaTableElementCountFieldIndex = 0;", "swiss-name-dictionary.h");
mustContain(h, "static constexpr uint32_t kMetaTableDeletedElementCountFieldIndex = 1;", "swiss-name-dictionary.h");
mustContain(h, "static constexpr uint32_t kMetaTableEnumerationDataStartIndex = 2;", "swiss-name-dictionary.h");
mustContain(h, "static constexpr uint32_t kMax1ByteMetaTableCapacity = (1 << 8);", "swiss-name-dictionary.h");
mustContain(h, "static constexpr uint32_t kMax2ByteMetaTableCapacity = (1 << 16);", "swiss-name-dictionary.h");

// ── 3. 容量与装载因子检验 ───────────────────────────────────────────────────
mustContain(inl, "constexpr uint32_t SwissNameDictionary::MaxUsableCapacity(uint32_t capacity)", "swiss-name-dictionary-inl.h");
mustContain(inl, "uint32_t SwissNameDictionary::CapacityFor(uint32_t at_least_space_for)", "swiss-name-dictionary-inl.h");

// ── 4. 导出常量与计算方法 ──────────────────────────────────────────────────
export const CTRL_BYTE = {
  kEmpty: -128,   // 0x80 (i8 as u8: 128)
  kDeleted: -2,   // 0xFE (i8 as u8: 254)
  kSentinel: -1,  // 0xFF (i8 as u8: 255)
  kEmptyU8: 0x80,
  kDeletedU8: 0xfe,
  kSentinelU8: 0xff,
};

export const SWISS_TABLE_HASH = {
  kH2Bits: 7,
  kH2Mask: (1 << 7) - 1, // 0x7F
  H1: (hash) => (hash >>> 7),
  H2: (hash) => (hash & 0x7F),
};

export const SWISS_NAME_DICTIONARY = {
  kInitialCapacity: 4,
  kGroupWidthSse2: 16,
  kGroupWidthPortable: 8,
  kDataTableEntryCount: 2,
  kDataTableKeyEntryIndex: 0,
  kDataTableValueEntryIndex: 1,
  kMetaTableElementCountFieldIndex: 0,
  kMetaTableDeletedElementCountFieldIndex: 1,
  kMetaTableEnumerationDataStartIndex: 2,
  kMax1ByteMetaTableCapacity: 256,
  kMax2ByteMetaTableCapacity: 65536,
  maxUsableCapacity: (capacity, groupWidth = 16) => {
    if (groupWidth === 8 && capacity === 4) return 3;
    return capacity - Math.floor(capacity / 8);
  },
};

console.log("[swiss-name-dictionary.h.js] 提取与锚点校验通过:");
console.log({
  CTRL_BYTE,
  SWISS_TABLE_HASH,
  SWISS_NAME_DICTIONARY,
});
