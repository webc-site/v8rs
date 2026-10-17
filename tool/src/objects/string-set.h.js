#!/usr/bin/env bun
// tool/src/objects/string-set.h.js — 覆盖 string-set 同名族:
//   v8/src/objects/string-set.h + string-set-inl.h
//   → v8-objects-strings/src/string_set.rs
//
// 映射登记 (C++ → Rust):
//   StringSetShape (BaseShape<Tagged<String>> 特化) → 常量组 + 泛型判定函数
//   IsMatch (key->Equals(Cast<String>(value))) → StringSetEntry::equals
//   Hash / HashForObject (EnsureHash) → StringSetEntry::ensure_hash
//   (String 对象 Equals/EnsureHash 归 OS-1 string.rs → 以 entry trait 承接
//    shape 契约, HashTable 集成时直接约束)
//
// 跳过区段登记 (推迟):
//   EXTERN_DECLARE_HASH_TABLE / StringSet::New/Add/Has (hash-table.h 基建 +
//     isolate/factory): 归 v8-objects-collections HashTable 层 + L4;
//     kNeedsHoleCheck 查洞语义随基建落地
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/objects/string-set.h");
const inl = readV8("src/objects/string-set-inl.h");

const anchors = [
  [h, "static const int kPrefixSize = 0;", "h"],
  [h, "static const int kEntrySize = 1;", "h"],
  [h, "static const bool kMatchNeedsHoleCheck = true;", "h"],
  [h, "static const bool kDoHashSpreading = false;", "h"],
  [h, "static const uint32_t kHashBits = 0;", "h"],
  [inl, "return key->EnsureHash();", "inl"],
  [inl, "return key->Equals(Cast<String>(value));", "inl"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

const grab = (s, re, f) => {
  const m = s.match(re);
  if (!m) throw new Error(`提取失配: ${f}`);
  return m[1];
};
const kPrefixSize = grab(h, /kPrefixSize = (\d+);/, "kPrefixSize");
const kEntrySize = grab(h, /kEntrySize = (\d+);/, "kEntrySize");
const kHashBits = grab(h, /kHashBits = (\d+);/, "kHashBits");

const body = `${banner("src/objects/string-set.h.js", "src/objects/string-set.{h,-inl.h}", "Shape 常量与判定契约 1:1; HashTable 基建/New/Add/Has 推迟 (登记见脚本头)")}
//! StringSet 形状 (Shape): 堆上字符串集合哈希表的键形状契约。
//!
//! 原版 StringSet 是 HashTable<StringSet, StringSetShape> 的特化;
//! HashTable 基建归 v8-objects-collections, 本模块落地形状常量与
//! 判定契约 (登记见脚本头)。

/// 字符串集合条目契约 (String 的 EnsureHash/Equals 段;
/// String 对象绑定归 OS-1, 集成时为 String 实现)。
pub trait StringSetEntry {
  /// 确保哈希已计算并返回 (原版 EnsureHash)。
  fn ensure_hash(&mut self) -> u32;

  /// 内容等值 (原版 String::Equals)。
  fn equals(&self, other: &Self) -> bool;
}

/// 字符串集合形状 (string-set.h:16 StringSetShape)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringSetShape;

impl StringSetShape {
  /// 前缀槽数 (kPrefixSize)。
  pub const PREFIX_SIZE: usize = ${kPrefixSize};
  /// 每条目槽数 (kEntrySize)。
  pub const ENTRY_SIZE: usize = ${kEntrySize};
  /// 匹配需查洞 (kMatchNeedsHoleCheck; 删除槽语义随 HashTable 基建)。
  pub const MATCH_NEEDS_HOLE_CHECK: bool = true;
  /// 不做哈希扩散 (kDoHashSpreading)。
  pub const DO_HASH_SPREADING: bool = false;
  /// 哈希位数 (kHashBits = 0 → 全宽)。
  pub const HASH_BITS: u32 = ${kHashBits};

  /// 键与槽值匹配 (string-set-inl.h:19 IsMatch: key->Equals(value))。
  #[inline]
  #[must_use]
  pub fn is_match<E: StringSetEntry>(key: &E, value: &E) -> bool {
    key.equals(value)
  }

  /// 键哈希 (string-set-inl.h:24 Hash: EnsureHash)。
  #[inline]
  pub fn hash<E: StringSetEntry>(key: &mut E) -> u32 {
    key.ensure_hash()
  }

  /// 槽值哈希 (string-set-inl.h:28 HashForObject: EnsureHash)。
  #[inline]
  pub fn hash_for_object<E: StringSetEntry>(object: &mut E) -> u32 {
    object.ensure_hash()
  }
}
`;

writeSrc("v8-objects-strings", "string_set.rs", body);
console.log(`string_set.rs: Shape 常量 ${kPrefixSize}/${kEntrySize}/${kHashBits} + 判定契约; HashTable 段推迟登记`);
