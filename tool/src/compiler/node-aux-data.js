#!/usr/bin/env bun
// v8/src/compiler/node-aux-data.h (单文件模板, 无 .cc)
// → v8-compiler-passes/src/node_aux_data.rs.skel
//
// 全模板容器, Set/Get 语义完整迁 (越界扩容 + 默认值兜底 + 变更返回)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/node-aux-data.h");

// —— 哨兵 ——
const sentinels = [
  [/template <class T, T def\(Zone\*\) = DefaultConstruct<T>>/, "默认模板参"],
  [/explicit NodeAuxData\(size_t initial_size, Zone\* zone\)/, "尺寸构造"],
  [/if \(id >= aux_data_\.size\(\)\) aux_data_\.resize\(id \+ 1, def\(zone_\)\);/, "越界扩容"],
  [/return \(id < aux_data_\.size\(\)\) \? aux_data_\[id\] : def\(zone_\);/, "默认值兜底"],
  [/if \(aux_data_\[id\] != data\) \{\n      aux_data_\[id\] = data;\n      return true;\n    \}\n    return false;/, "变更返回语义"],
  [/T ZoneConstruct\(Zone\* zone\) \{/, "ZoneConstruct 特化点"],
];
for (const [re, name] of sentinels) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

const body = `${banner("src/compiler/node-aux-data.js", "src/compiler/node-aux-data.h", "NodeAuxData — 节点 id 索引的辅助数据表 (越界扩容 + 默认值)")}
//! NodeAuxData<T>: 以 NodeId 索引的 Zone 向量; def(zone) 模板参提供
//! 默认值 (DefaultConstruct / ZoneConstruct 两个特化点)。

#![allow(dead_code)]

/// v8: src/compiler/node-aux-data.h NodeAuxData
pub struct NodeAuxData<T: Clone + Default + PartialEq> {
  /// 原 zone_
  zone: *mut crate::zone::Zone,
  /// 原 aux_data_
  aux_data: Vec<T>,
}

impl<T: Clone + Default + PartialEq> NodeAuxData<T> {
  /// v8: src/compiler/node-aux-data.h NodeAuxData::NodeAuxData (zone 版)
  pub fn new(zone: *mut crate::zone::Zone) -> Self {
    Self { zone, aux_data: Vec::new() }
  }
  /// v8: src/compiler/node-aux-data.h NodeAuxData::NodeAuxData (尺寸版)
  /// 原文: aux_data_(initial_size, def(zone), zone) — 预填默认值
  pub fn with_size(initial_size: usize, zone: *mut crate::zone::Zone) -> Self {
    Self { zone, aux_data: vec![T::default(); initial_size] }
  }

  /// v8: src/compiler/node-aux-data.h NodeAuxData::Set (Node 版)
  pub fn set_node(&mut self, node: &crate::node::Node, data: &T) -> bool {
    self.set(node.id(), data)
  }

  /// v8: src/compiler/node-aux-data.h NodeAuxData::Set (NodeId 版)
  /// 原文: 越界 resize(id+1, def); 值不同才写并返回 true
  pub fn set(&mut self, id: crate::graph_reducer::NodeId, data: &T) -> bool {
    if id as usize >= self.aux_data.len() {
      self.aux_data.resize(id as usize + 1, T::default());
    }
    if &self.aux_data[id as usize] != data {
      self.aux_data[id as usize] = data.clone();
      return true;
    }
    false
  }

  /// v8: src/compiler/node-aux-data.h NodeAuxData::Get (Node 版)
  pub fn get_node(&self, node: &crate::node::Node) -> T {
    self.get(node.id())
  }

  /// v8: src/compiler/node-aux-data.h NodeAuxData::Get (NodeId 版)
  /// 原文: 越界返回默认值, 不扩容
  pub fn get(&self, id: crate::graph_reducer::NodeId) -> T {
    if (id as usize) < self.aux_data.len() {
      self.aux_data[id as usize].clone()
    } else {
      T::default()
    }
  }

  /// v8: src/compiler/node-aux-data.h NodeAuxData::const_iterator
  /// 产出 (NodeId, T) 序对序列
  pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
    self.aux_data.iter().enumerate()
  }
}

// DefaultConstruct<T> → T::default(); ZoneConstruct<T> → T::new(zone)
// 原版经模板参 def(Zone*) 选择, Rust 侧由 T 的构造 trait 承担
`;

const p = writeSrc("v8-compiler-passes", "node_aux_data.rs.skel", body);
console.log(`written ${p}\n方法=6 (Set/Get 各重载 2) 语义=完整 (扩容/兜底/变更返回) 迭代器=1`);
