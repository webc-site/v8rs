#!/usr/bin/env bun
// tool/src/base/hashmap-entry.h.js — 族: src/base/hashmap-entry.h
// 覆盖: NoHashMapValue + TemplateHashMapEntry 双特化 (hash:exists 位打包)
// → v8-base/src/hashmap_entry.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/hashmap-entry.h");

// —— 锚点断言 ——
if (!src.includes("struct NoHashMapValue {};")) throw new Error("缺 NoHashMapValue");
if (!src.includes("struct TemplateHashMapEntry {")) throw new Error("缺主模板");
if (!src.includes("struct TemplateHashMapEntry<Key, NoHashMapValue> {")) throw new Error("缺无值特化");
if (!src.includes("using HashField = BitField<uint32_t, 0, 31>;")) throw new Error("缺 HashField 位域");
if (!src.includes("using ExistsField = HashField::Next<bool, 1>;")) throw new Error("缺 ExistsField 位域");
if (!src.includes("static constexpr uint32_t kHashValueMask = HashField::kMax;")) throw new Error("缺掩码");

// —— 真解析: 位布局计数 (两个特化各一份) ——
const bitfieldCount = (src.match(/HashField::encode\(hash\)/g) || []).length;
if (bitfieldCount !== 2) throw new Error(`构造器位打包 ${bitfieldCount} != 2`);
const hashAndExists = (src.match(/uint32_t hash_and_exists_;/g) || []).length;
if (hashAndExists !== 2) throw new Error(`hash_and_exists_ 成员 ${hashAndExists} != 2`);
console.log("hashmap-entry: 特化 2 (有值/无值), 位布局 hash[0:31) + exists[31], 方法 3×2");

let rust = banner(
  "src/base/hashmap-entry.h.js",
  "src/base/hashmap-entry.h",
  "哈希表条目 (hash 与 exists 打包进单 u32)",
);

rust += `
//! v8: src/base/hashmap-entry.h

// v8: struct NoHashMapValue {} — 集合 (无值) 哨兵类型, 不占条目存储

/// v8: src/base/hashmap-entry.h TemplateHashMapEntry<Key, Value>
/// 位布局: hash_and_exists_ = hash[0..31) | exists<<31 (BitField)
///   HashField = BitField<u32, 0, 31>; ExistsField = HashField::Next<bool, 1>
///   kHashValueMask = HashField::kMax = 0x7FFF_FFFF
pub struct TemplateHashMapEntry<K, V> {
    pub key: K,
    pub value: V,
    hash_and_exists: u32,
}

impl<K, V> TemplateHashMapEntry<K, V> {
    /// v8: 构造 — encode(hash) | encode(true)
    pub fn new(key: K, value: V, hash: u32) -> Self;
    /// v8: hash() — HashField::decode (低 31 位)
    pub fn hash(&self) -> u32;
    /// v8: exists() — bit31
    pub fn exists(&self) -> bool;
    /// v8: clear() — 仅清 exists 位 (hash 保留)
    pub fn clear(&mut self);
}

/// v8: src/base/hashmap-entry.h TemplateHashMapEntry<Key, NoHashMapValue>
/// 无值特化: key 与空 value 共 union 不占空间; 方法面同上
pub struct TemplateHashMapSetEntry<K> { key: K, hash_and_exists: u32 }

pub const HASH_VALUE_MASK: u32 = 0x7FFF_FFFF; // v8: kHashValueMask
`;

writeSrc("v8-base", "hashmap_entry.rs.skel", rust);
console.log("written v8-base/src/hashmap_entry.rs.skel");
