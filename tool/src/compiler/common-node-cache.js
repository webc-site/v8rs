#!/usr/bin/env bun
// v8/src/compiler/common-node-cache.cc + common-node-cache.h
// → v8-compiler-passes/src/common_node_cache.rs.skel
//
// 规则面: 11 个缓存字段 → 11 个 FindXxx 方法 1:1; 浮点常量按位模式
// 规范化 (bit_cast) 的键规则逐条核对。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/common-node-cache.h");
const cc = readV8("src/compiler/common-node-cache.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class CommonNodeCache final \{/, "CommonNodeCache"],
  [h, /FindInt32Constant\(int32_t value\)/, "FindInt32"],
  [h, /FindFloat64Constant\(Float64 value\)/, "FindFloat64"],
  [h, /FindRelocatableInt64Constant\(int64_t value, RelocInfoMode rmode\)/, "FindReloc64"],
  [h, /RelocInt64NodeCache relocatable_int64_constants_;/, "字段尾"],
  [cc, /void CommonNodeCache::GetCachedNodes\(ZoneVector<Node\*>\* nodes\) \{/, "GetCachedNodes"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— 方法与字段清单 ——
const finds = [...h.matchAll(/^  Node\*\* Find(\w+)\(/gm)].map((m) => m[1]);
if (finds.join() !== "Int32Constant,Int64Constant,TaggedIndexConstant,Float32Constant,Float64Constant,ExternalConstant,PointerConstant,NumberConstant,HeapConstant,RelocatableInt32Constant,RelocatableInt64Constant") {
  throw new Error(`Find 漂移: ${finds}`);
}
const fields = [...h.matchAll(/^  (\w+NodeCache) (\w+);/gm)].map((m) => `${m[1]} ${m[2]}`);
if (fields.length !== 11) throw new Error(`缓存字段 ${fields.length} != 11`);
// 位规范化键: float32 → bit_cast<int32_t>, float64 → get_bits(), number → bit_cast<int64_t>
if (!/float32_constants_\.Find\(base::bit_cast<int32_t>\(value\)\)/.test(h)) throw new Error("f32 位键缺失");
if (!/float64_constants_\.Find\(value\.get_bits\(\)\)/.test(h)) throw new Error("f64 位键缺失");
if (!/number_constants_\.Find\(base::bit_cast<int64_t>\(value\)\)/.test(h)) throw new Error("number 位键缺失");

const body = `${banner("src/compiler/common-node-cache.js", "src/compiler/common-node-cache.h + src/compiler/common-node-cache.cc", "CommonNodeCache — 常量节点缓存束 (11 类键)")}
//! 11 类常量缓存: int32/int64/taggedIndex/float32/float64/external/
//! pointer/number/heap/reloc32/reloc64。
//!
//! 规则面: Find 键规则 — 浮点与 number 按位模式规范化
//! (f32: bit_cast<int32_t>; f64: Float64.get_bits; number: bit_cast<int64_t>);
//! reloc 系以 (值, RelocInfoMode) 二元组为键。

#![allow(dead_code)]

/// v8: src/compiler/common-node-cache.h CommonNodeCache
pub struct CommonNodeCache {
${fields.map(([, name]) => `  /// 原 ${name} — ${name.replace("_constants_", "")}缓存`).join("\n")}
${fields.map((f) => `  pub ${f.replace(/_constants_$/, "").replace(/_$/, "")}: crate::node_cache::${f.split(" ")[0]},`).join("\n")}
}

impl CommonNodeCache {
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::CommonNodeCache
  pub fn new(zone: &mut crate::zone::Zone) -> Self {
    let _ = zone;
    todo!("11 缓存以 zone 构造")
  }

  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindInt32Constant
  pub fn find_int32_constant(&mut self, value: i32) -> &mut Option<crate::node::Node> {
    todo!("int32_constants_.Find(value)")
  }
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindInt64Constant
  pub fn find_int64_constant(&mut self, value: i64) -> &mut Option<crate::node::Node> {
    todo!("int64_constants_.Find(value)")
  }
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindTaggedIndexConstant
  pub fn find_tagged_index_constant(&mut self, value: i32) -> &mut Option<crate::node::Node> {
    todo!("tagged_index_constants_.Find(value)")
  }
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindFloat32Constant
  /// 原文: 按位表示规范化 → Find(base::bit_cast<int32_t>(value))
  pub fn find_float32_constant(&mut self, value: f32) -> &mut Option<crate::node::Node> {
    todo!("float32_constants_.Find(value.to_bits() as i32)")
  }
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindFloat64Constant
  /// 原文: Find(value.get_bits())
  pub fn find_float64_constant(&mut self, value: crate::float64::Float64) -> &mut Option<crate::node::Node> {
    todo!("float64_constants_.Find(value.get_bits())")
  }
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindExternalConstant (cc 实现)
  pub fn find_external_constant(&mut self, value: crate::external_reference::ExternalReference) -> &mut Option<crate::node::Node> {
    todo!("external_constants_.Find(value.address())")
  }
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindPointerConstant
  pub fn find_pointer_constant(&mut self, value: isize) -> &mut Option<crate::node::Node> {
    todo!("pointer_constants_.Find(value)")
  }
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindNumberConstant
  /// 原文: Find(base::bit_cast<int64_t>(value))
  pub fn find_number_constant(&mut self, value: f64) -> &mut Option<crate::node::Node> {
    todo!("number_constants_.Find(value.to_bits() as i64)")
  }
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindHeapConstant (cc 实现)
  pub fn find_heap_constant(&mut self, value: crate::heap_refs::HeapObjectHandle) -> &mut Option<crate::node::Node> {
    todo!("heap_constants_.Find(value.address())")
  }
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindRelocatableInt32Constant
  /// 原文: Find(std::make_pair(value, rmode))
  pub fn find_relocatable_int32_constant(&mut self, value: i32, rmode: crate::reloc_info::Mode) -> &mut Option<crate::node::Node> {
    todo!("relocatable_int32_constants_.Find((value, rmode))")
  }
  /// v8: src/compiler/common-node-cache.h CommonNodeCache::FindRelocatableInt64Constant
  pub fn find_relocatable_int64_constant(&mut self, value: i64, rmode: crate::reloc_info::Mode) -> &mut Option<crate::node::Node> {
    todo!("relocatable_int64_constants_.Find((value, rmode))")
  }

  /// v8: src/compiler/common-node-cache.cc CommonNodeCache::GetCachedNodes
  /// 原文: 11 个缓存逐一 GetCachedNodes(nodes)
  pub fn get_cached_nodes(&self, nodes: &mut Vec<crate::node::Node>) {
    todo!("11 缓存汇总")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "common_node_cache.rs.skel", body);
console.log(`written ${p}\nFind=${finds.length} 缓存字段=${fields.length} 位规范化键=3`);
