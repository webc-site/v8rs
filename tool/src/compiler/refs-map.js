#!/usr/bin/env bun
// tool/src/compiler/refs-map.js
// v8/src/compiler/refs-map.h + refs-map.cc
// → v8-compiler-passes/src/refs_map.rs.skel
// Address→ObjectData 哈希表: 解析 RefsMap 方法面与 Hash 混合常量。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/refs-map.h");
const cc = readV8("src/compiler/refs-map.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`refs-map: ${msg}`);
};

for (const [s, name] of [
  ["class AddressMatcher : public base::KeyEqualityMatcher<Address> {", "AddressMatcher"],
  ["class RefsMap", "RefsMap"],
  ["static uint32_t Hash(Address addr);", "Hash 声明"],
  ["RefsMap::RefsMap(uint32_t capacity, AddressMatcher match, Zone* zone)", "cc ctor"],
  ["RefsMap::Entry* RefsMap::Lookup(const Address& key) const", "cc Lookup"],
  ["RefsMap::Entry* RefsMap::InsertNew(const Address& key)", "cc InsertNew"],
  ["ObjectData* RefsMap::Remove(const Address& key)", "cc Remove"],
  ["uint32_t RefsMap::Hash(Address addr)", "cc Hash"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Hash 实体现解析 (cc) ——
const hashBody = cc.slice(cc.indexOf("uint32_t RefsMap::Hash(Address addr)"));
const hashImpl = hashBody.match(/return static_cast<uint32_t>\(([^;]+)\);/);
need(hashImpl, "Hash 实现未找到");
need(hashImpl[1].includes("base::hash64"), `Hash 非预期实现 ${hashImpl[1]}`);
const linearProbingNote = /linear probing/.test(hashBody);

const body = `${banner("src/compiler/refs-map.js", "src/compiler/refs-map.h + refs-map.cc", "规范 Address → ObjectData 映射 (JSHeapBroker refs 与 PerIsolateCompilerCache snapshot 共用)")}
//! refs-map — 自研哈希表 (廉价拷贝: broker refs map ↔ compiler cache snapshot)
//!
//! 底层 base::TemplateHashMapImpl; Rust 侧以 RapidHashMap + 显式 Entry
//! 迭代重建。Hash = base::hash64 截断 u32 (handle 位置指针对齐, 线性探测
//! 退化 → 用全 64 位散列; ${linearProbingNote ? "cc 注释已核对" : "注意核对 cc 注释"})。

/// 原版 AddressMatcher — 地址判等 (裸 ==)
#[derive(Default)]
pub struct AddressMatcher;

/// 原版 RefsMap — Address → ObjectData* 映射
pub struct RefsMap<'z> {
  /// 底层表 (capacity 起始可指定; Rust 侧 RapidHashMap)
  pub map: RapidHashMap<u64, *mut ObjectData>,
  /// zone (entry 分配)
  pub zone: &'z Zone<'z>,
}

impl RefsMap<'_> {
  /// 原版 Hash — Address → u32 = (u32)base::hash64(addr)
  /// v8: src/compiler/refs-map.cc RefsMap::Hash (${hashImpl[1].trim()})
  pub fn hash(addr: u64) -> u32 {
    v8_base::hashing::hash64(addr) as u32
  }

  /// 原版 IsEmpty — occupancy == 0
  /// v8: src/compiler/refs-map.h RefsMap::IsEmpty
  pub fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  /// 原版 Lookup — 查 entry (不存在返回 None)
  /// v8: src/compiler/refs-map.cc RefsMap::Lookup
  pub fn lookup(&self, key: u64) -> Option<&mut ObjectData> {
    todo!("v8: src/compiler/refs-map.cc RefsMap::Lookup")
  }

  /// 原版 InsertNew — 插入空 entry 并返回
  /// v8: src/compiler/refs-map.cc RefsMap::InsertNew
  pub fn insert_new(&mut self, key: u64) -> &mut ObjectData {
    todo!("v8: src/compiler/refs-map.cc RefsMap::InsertNew")
  }

  /// 原版 Remove — 移除并返回旧值
  /// v8: src/compiler/refs-map.cc RefsMap::Remove
  pub fn remove(&mut self, key: u64) -> Option<*mut ObjectData> {
    todo!("v8: src/compiler/refs-map.cc RefsMap::Remove")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "refs_map.rs.skel", body);
console.log(`written ${p}\nhash=${hashImpl[1].trim()} linear_probing_note=${linearProbingNote}`);
