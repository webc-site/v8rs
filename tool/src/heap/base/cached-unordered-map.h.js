#!/usr/bin/env bun
// tool/src/heap/base/cached-unordered-map.h.js — cached_unordered_map.rs 初版生成
// 组→文件映射 (本脚本管辖):
//   v8/src/heap/base/cached-unordered-map.h → v8-heap-base/src/cached_unordered_map.rs
// 规则:
//   - 公开方法清单锚点断言 (operator[]/erase/find/begin/end/contains/clear/empty/Take)
//   - LRU 缓存指针 (last_key_/last_mapped_) 语义骨架: 插入/删除即失效
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/heap/base/cached-unordered-map.h");

// ---- 锚点断言 ----
for (const m of ["operator\\[\\]", "erase", "find", "begin", "end", "contains", "clear", "empty", "Take"]) {
  if (!new RegExp(m).test(h)) throw new Error(`cached-unordered-map.h: ${m} 缺失`);
}
// LRU 缓存字段证据
if (!/Key last_key_ = nullptr/.test(h) || !/Mapped\* last_mapped_ = nullptr/.test(h))
  throw new Error("last_key_/last_mapped_ 缓存字段缺失");

const body = `${banner("src/heap/base/cached-unordered-map.h.js", "src/heap/base/cached-unordered-map.h", "LRU 单键缓存哈希表 (absl flat_hash_map → gxhash)")}
//! CachedUnorderedMap: 以 LRU 方式加速 operator[] 的哈希表。
//! 原版缓存 last_key_/last_mapped_ 裸指针 (对数核对通过);
//! Rust 侧以 Option<(K, *mut V)> 等价承载, 任何插入/删除/清空立即失效缓存
//! (gxhash 扩容会迁移条目, 保留悬垂指针违背健全性 — 语义收严)。

use gxhash::HashMap;
use std::{hash::Hash, mem};

/// 原 CachedUnorderedMap (cached-unordered-map.h:16)。
pub struct CachedUnorderedMap<K: Eq + Hash, V> {
  map: HashMap<K, V>,
  /// SAFETY 约束: 指针仅在上一次 map 变更为纯查找时有效;
  /// 插入/erase/clear/take 后必须置 None。
  last: Option<(K, *mut V)>,
}

impl<K: Eq + Hash, V> CachedUnorderedMap<K, V> {
  pub fn new() -> Self { todo!("就地修订") }
  /// 原 operator[] (h:24-43): 命中缓存免查找; 缺省值插入。
  pub fn get_or_insert(&mut self, key: &K) -> &mut V { todo!("就地修订") }
  /// 原 erase (h:45-51): 命中缓存即失效。
  pub fn erase(&mut self, key: &K) -> bool { todo!("就地修订") }
  /// 原 find (h:55): 无迭代器缓存, 每次真实查找。
  pub fn find(&mut self, key: &K) -> Option<&V> { todo!("就地修订") }
  /// 原 begin/end → iter。
  pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> { todo!("就地修订") }
  /// 原 contains (h:62)。
  pub fn contains(&self, key: &K) -> bool { todo!("就地修订") }
  /// 原 clear (h:64-68)。
  pub fn clear(&mut self) { todo!("就地修订") }
  /// 原 empty (h:70)。
  pub fn is_empty(&self) -> bool { todo!("就地修订") }
  /// 原 Take (h:72-79): 取走整个 map, 自身清空。
  pub fn take(&mut self) -> HashMap<K, V> { todo!("就地修订") }
}
`;

const p = writeSrc("v8-heap-base", "cached_unordered_map.rs", body);
console.log(`cached-unordered-map.h.js: ${p} (方法 9 项 + 缓存字段核对通过)`);
