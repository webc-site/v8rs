#!/usr/bin/env bun
// tool/src/compiler/persistent-map.h.js
// v8/src/compiler/persistent-map.h
// → v8-compiler-passes/src/persistent_map.rs.skel
// 持久哈希树: 解析 kHashBits / Bit 方向枚举 / 公有方法面 / Zip-遍历语义。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/persistent-map.h");

const need = (cond, msg) => {
  if (!cond) throw new Error(`persistent-map: ${msg}`);
};

for (const [s, name] of [
  ["class PersistentMap {", "PersistentMap"],
  ["static constexpr size_t kHashBits = 32;", "kHashBits"],
  ["enum Bit : int { kLeft = 0, kRight = 1 };", "Bit 方向"],
  ["struct FocusedTree;", "FocusedTree 前向"],
  ["size_t last_depth() const {", "last_depth"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— 方法面 (inline 体直接 includes 核对; 声明与定义混合) ——
const methods = ["last_depth", "Get", "Set", "Zip", "begin", "end", "Modify"].filter((mm) => new RegExp(`\\b${mm}\\(`).test(h));
for (const needM of ["Get", "Set", "Zip", "begin", "end", "last_depth"]) {
  need(methods.includes(needM), `方法 ${needM} 缺席 (实得 ${methods.join(",")})`);
}

// FocusedTree 字段面 (全限定定义: PersistentMap<...>::FocusedTree)
// key_value / length / key_hash / more / path_array[1] (柔性尾数组)
const ftDef = h.slice(h.indexOf("struct PersistentMap<Key, Value, Hasher>::FocusedTree {"));
const ftFieldNames = ["key_value", "length", "key_hash", "more", "path_array"];
const ftFields = ftFieldNames.filter((f) => new RegExp(`\\\\b${f}[;\\[]`).test(ftDef) || new RegExp(`\\b${f}[;\\[]`).test(ftDef));
need(ftFields.length === 5, `FocusedTree 字段 ${ftFields.length}`);
// 柔性数组注释核对
need(ftDef.includes("has to be the last member"), "柔性尾数组注释缺失");

// KeyValue 结构
need(h.includes("struct KeyValue : std::pair<Key, Value> {"), "KeyValue 缺席");

const body = `${banner("src/compiler/persistent-map.h.js", "src/compiler/persistent-map.h", "持久化哈希树映射 (结构共享, O(log n) 更新)")}
//! persistent-map — hash tree (哈希位作地址的二叉树)
//!
//! 概念无限: 缺省键映默认值; 删除 = 写回默认值; 迭代恰产非默认键。
//! kHashBits = 32, 方向位 kLeft=0/kRight=1; FocusedTree 即路径前缀节点。
//! 复杂度: 拷贝 O(1); 读/写 O(log n); Zip O(n)。

/// 原版 kHashBits — 哈希树深度位数
pub const K_HASH_BITS: u32 = 32;

/// 原版 Bit — 树方向 (哈希位取值)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
  /// kLeft = 0
  Left = 0,
  /// kRight = 1
  Right = 1,
}

/// 原版 PersistentMap::KeyValue — 键值对 (std::pair 继承面)
pub struct KeyValue<K, V> {
  /// first
  pub key: K,
  /// second
  pub value: V,
}

/// 原版 PersistentMap::FocusedTree — 聚焦路径节点
/// 字段面: ${ftFields.join(" / ")} (path_array 为柔性尾数组, 越 path_array 便是内联 path 区)
pub struct FocusedTree<K, V> {
  /// key_value
  pub key_value: KeyValue<K, V>,
  /// length — 聚焦路径深度 (内联指针数)
  pub length: i8,
  /// key_hash
  pub key_hash: u32,
  /// more — 哈希冲突的旁路表
  pub more: Option<&'static ZoneMap<K, V>>,
  /// path_array[1] — 柔性尾数组 (越界即内联路径)
  pub path_array: [*const FocusedTree<K, V>; 1],
}

/// 原版 PersistentMap — 持久映射
pub struct PersistentMap<K, V, H = DefaultHasher> {
  /// default_value — 概念无限映射的缺省
  pub default_value: V,
  /// tree_ — 根 (None = 空)
  pub tree: Option<Box<FocusedTree<K, V>>>,
  /// hasher 标记
  pub hasher: core::marker::PhantomData<H>,
}

impl<K: Eq + Clone, V: Clone + PartialEq> PersistentMap<K, V, DefaultHasher> {
  /// 原版 Get — 只读访问 (O(log n))
  /// v8: src/compiler/persistent-map.h PersistentMap::Get
  pub fn get(&self, key: &K) -> &V {
    todo!("v8: src/compiler/persistent-map.h PersistentMap::Get")
  }

  /// 原版 Set — 结构共享更新 (O(log n) 时间/空间)
  /// v8: src/compiler/persistent-map.h PersistentMap::Set
  pub fn set(&self, key: K, value: V) -> Self {
    todo!("v8: src/compiler/persistent-map.h PersistentMap::Set")
  }

  /// 原版 Zip — 两树同位遍历产出差异流
  /// v8: src/compiler/persistent-map.h PersistentMap::Zip
  pub fn zip<'a>(&'a self, other: &'a Self) -> ZipIterator<'a, K, V> {
    todo!("v8: src/compiler/persistent-map.h PersistentMap::Zip")
  }

  /// 原版 last_depth — 最近添加元素的深度 (树规模廉价估计)
  /// v8: src/compiler/persistent-map.h PersistentMap::last_depth
  pub fn last_depth(&self) -> usize {
    self.tree.as_ref().map_or(0, |t| t.length)
  }

  /// 原版 begin/end — 非默认键迭代
  /// v8: src/compiler/persistent-map.h PersistentMap::begin
  pub fn iter(&self) -> PersistentMapIter<'_, K, V> {
    todo!("v8: src/compiler/persistent-map.h PersistentMap::begin/end")
  }
}

/// 原版 PersistentMap::iterator / ZipIterator — 栈式遍历
pub struct PersistentMapIter<'m, K, V> {
  /// 栈
  pub stack: Vec<&'m FocusedTree<K, V>>,
}

/// Zip 迭代器 — (key, old, new) 差异流
pub struct ZipIterator<'m, K, V> {
  /// 左树
  pub lhs: Option<&'m FocusedTree<K, V>>,
  /// 右树
  pub rhs: Option<&'m FocusedTree<K, V>>,
}
`;

const p = writeSrc("v8-compiler-passes", "persistent_map.rs.skel", body);
console.log(`written ${p}\nmethods=${methods.length} focused_tree_fields=${ftFields.length} kHashBits=32`);
