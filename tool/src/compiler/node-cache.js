#!/usr/bin/env bun
// v8/src/compiler/node-cache.h (单文件模板, 无 .cc)
// → v8-compiler-passes/src/node_cache.rs.skel
//
// 规则面: NodeCache<Key,Hash,Pred> (Find 槽位语义) + 默认缓存别名 5 项
// (Int32/Int64/IntPtr/RelocInt32/RelocInt64), 逐条提取。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/node-cache.h");

// —— 哨兵 ——
const sentinels = [
  [/template <typename Key, typename Hash = base::hash<Key>,/, "NodeCache 模板"],
  [/Node\*\* Find\(Key key\) \{ return &\(map_\[key\]\); \}/, "Find 槽位语义"],
  [/if \(entry\.second\) nodes->push_back\(entry\.second\);/, "GetCachedNodes 过滤空"],
  [/using Int32NodeCache = NodeCache<int32_t>;/, "Int32 别名"],
  [/#if V8_HOST_ARCH_32_BIT\nusing IntPtrNodeCache = Int32NodeCache;/, "IntPtr 别名 (32 位分支)"],
  [/using RelocInt32Key = std::pair<int32_t, RelocInfoMode>;/, "Reloc 键"],
];
for (const [re, name] of sentinels) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}
// 别名全集
const aliases = [...h.matchAll(/^using (\w+NodeCache) = NodeCache<([\w:<>]+)>;/gm)].map((m) => [m[1], m[2]]);
if (aliases.length !== 4) throw new Error(`别名 ${aliases.length} != 4: ${aliases.map((a) => a[0])}`);
if (aliases.map((a) => a[0]).join() !== "Int32NodeCache,Int64NodeCache,RelocInt32NodeCache,RelocInt64NodeCache") throw new Error(`别名序漂移`);
const relocAliases = [...h.matchAll(/^using (Reloc\w+NodeCache) = NodeCache<(Reloc\w+Key)>;/gm)].map((m) => [m[1], m[2]]);
if (relocAliases.length !== 2) throw new Error(`Reloc 别名 ${relocAliases.length} != 2`);

const body = `${banner("src/compiler/node-cache.js", "src/compiler/node-cache.h", "NodeCache — 键 → 节点槽缓存 (canonicalization 支撑)")}
//! NodeCache: unordered_map<Key, Node*> 包装; Find 返回槽位指针
//! (调用方: 槽空则自建节点回填 — 惰性 canonicalization)。

#![allow(dead_code, non_camel_case_types)]

/// v8: src/compiler/node-cache.h NodeCache
pub struct NodeCache<Key: std::hash::Hash + Eq> {
  /// 原 map_ — ZoneUnorderedMap<Key, Node*, Hash, Pred>
  map: std::collections::HashMap<Key, Option<crate::node::Node>>,
}

impl<Key: std::hash::Hash + Eq + Clone> NodeCache<Key> {
  /// v8: src/compiler/node-cache.h NodeCache::NodeCache
  pub fn new(zone: *mut crate::zone::Zone) -> Self {
    let _ = zone;
    Self { map: Default::default() }
  }

  /// v8: src/compiler/node-cache.h NodeCache::Find
  /// 原文: return &(map_[key]) — 返回键槽位 (operator[] 空槽插入);
  /// Rust 侧以 entry 可变引用等价
  pub fn find(&mut self, key: Key) -> &mut Option<crate::node::Node> {
    self.map.entry(key).or_insert(None)
  }

  /// v8: src/compiler/node-cache.h NodeCache::GetCachedNodes
  /// 原文: 空槽 (entry.second == nullptr) 跳过
  pub fn get_cached_nodes(&self, nodes: &mut Vec<crate::node::Node>) {
    for entry in self.map.values() {
      if let Some(node) = entry {
        nodes.push(*node);
      }
    }
  }
}

// —— 默认缓存类型 (v8: src/compiler/node-cache.h) ——
${aliases.map(([n, k]) => `/// v8: using ${n} = NodeCache<${k}>;\npub type ${n} = NodeCache<${k.replace(/std::/, "").replace(/int32_t/, "i32").replace(/int64_t/, "i64").replace(/intptr_t/, "isize")}>;`).join("\n")}

/// v8: src/compiler/node-cache.h RelocInfoMode — 仅取数值 (char)
pub type RelocInfoMode = i8;
/// v8: using RelocInt32Key = std::pair<int32_t, RelocInfoMode>;
pub type RelocInt32Key = (i32, RelocInfoMode);
/// v8: using RelocInt64Key = std::pair<int64_t, RelocInfoMode>;
pub type RelocInt64Key = (i64, RelocInfoMode);
${relocAliases.map(([n, k]) => `/// v8: using ${n} = NodeCache<${k}>;\npub type ${n} = NodeCache<${k.replace("RelocInt32Key", "RelocInt32Key").replace("RelocInt64Key", "RelocInt64Key")}>;`).join("\n")}
`;

const p = writeSrc("v8-compiler-passes", "node_cache.rs.skel", body);
console.log(`written ${p}\n方法=3 别名=4+2 (含 Reloc 键 2) Find槽位语义=完整`);
