#!/usr/bin/env bun
// tool/src/base/small-map.h.js — 覆盖 v8/src/base/small-map.h
// 解析 SmallMap<NormalMap, kArraySize, EqualKey, MapInit> 双态 (数组→全 map) 布局、
// kUsingFullMapSentinel 哨兵与 map 面方法 (find/insert/operator[]/erase)
// → v8-base/src/small_map.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/small-map.h");

for (const anchor of [
  "V8_BASE_SMALL_MAP_H_",
  "struct has_key_equal {",
  "struct select_equal_key {",
  "kUsingFullMapSentinel",
  "SmallMap() : size_(0), functor_(MapInit()) {}",
  "const_iterator find(const key_type& key) const V8_LIFETIME_BOUND {",
  "std::pair<iterator, bool> insert(const value_type& x) V8_LIFETIME_BOUND {",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 哨兵值
const sentinel = h.match(/kUsingFullMapSentinel =\s*std::numeric_limits<size_t>::max/);
// 真解析: 内部辅助结构
const internals = [...new Set([...h.matchAll(/^struct (\w+) \{/gm)].map((m) => m[1]))];
// 真解析: map 面方法
const mapOps = [...new Set([...h.matchAll(/^  (?:const_)?iterator|std::pair<iterator, bool> (\w+)\(|data_type& (\w+)\[|void (\w+)\(|size_t (\w+)\(|bool (\w+)\(/gm)].map((m) => m[1] || m[2] || m[3] || m[4] || m[5]).filter(Boolean))];
// 真解析: iterator/const_iterator 运算
const iterClass = (h.match(/class (?:const_)?iterator|struct (?:const_)?iterator/g) || []).length;

const body = `${banner("src/base/small-map.h.js", "src/base/small-map.h", "小 map (数组直查, 超 kArraySize 转 NormalMap)")}
//! v8 侧: 内部结构 ${internals.length}; 迭代器类 ${iterClass}; map 面方法 ${mapOps.length}
//! 哨兵: size_ == ${sentinel ? "usize::MAX" : "?"} (kUsingFullMapSentinel) 表示已切全 map
//! Rust 策略: enum { Array(Vec<(K, V)>) — 上限 kArraySize, Map(BTreeMap/HashMap) } 状态机

/// v8: src/base/small-map.h internal::has_key_equal — SFINAE 探测 key_equal
/// v8: internal::select_equal_key — 无 key_equal 时由 key_comp 双比合成等值 (!comp(a,b) && !comp(b,a))
/// Rust: Eq trait 显式, 无需 SFINAE 探测

/// v8: class SmallMap<NormalMap, kArraySize, EqualKey, MapInit>
/// v8 布局: size_ + array_[kArraySize] + map_ (union 语义, size_==MAX 时 array_ 未初始化)
/// 迭代器跨增删失效 (v8 IMPORTANT 注)
pub enum SmallMap<K, V, const K_ARRAY_SIZE: usize> {
    /// v8: size_ < kUsingFullMapSentinel — 线性数组暴力查 (双比或 EqualKey)
    Array { entries: [(K, V); K_ARRAY_SIZE], len: usize },
    /// v8: size_ == kUsingFullMapSentinel — 超限后 MapInit placement-new 出 NormalMap
    Map(std::collections::HashMap<K, V>),
}

impl<K: Eq + std::hash::Hash, V, const N: usize> SmallMap<K, V, N> {
    /// v8: find(key) — 数组态线性扫 (compare), map 态委托
    pub fn find(&self, key: &K) -> Option<&V> { todo!() }
    /// v8: operator[](key) — 不存在则插入 value_type(key, data_type()); 满则 ConvertToMap
    pub fn get_or_insert(&mut self, key: K) -> &mut V where V: Default { todo!() }
    /// v8: insert(x) → (iterator, bool); 重复返回 false
    pub fn insert(&mut self, k: K, v: V) -> bool { todo!() }
    /// v8: insert(f, l) — 区间插入; emplace(args...)
    /// v8: erase(key) / erase(it); size() / empty(); begin()/end() 迭代器
    pub fn erase(&mut self, key: &K) -> usize { todo!() }
    pub fn size(&self) -> usize { todo!() }
    pub fn empty(&self) -> bool { todo!() }
    /// v8: ConvertToMap — 数组搬进 NormalMap 后置哨兵
    fn convert_to_map(&mut self) { todo!() }
}

// ---- v8 map 面方法对账 (解析 ${mapOps.length} 项) ----
// ${mapOps.join(", ")}
`;

writeSrc("v8-base", "small_map.rs.skel", body);
console.log(`small-map.h.js: internals=${internals.length} iterators=${iterClass} mapOps=${mapOps.length}`);
