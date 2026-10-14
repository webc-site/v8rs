#!/usr/bin/env bun
// v8/src/compiler/graph-zone-traits.h (单文件, 无 .cc)
// → v8-compiler-passes/src/graph_zone_traits.rs.skel
//
// 全文件仅类型别名 (ZoneTypeTraits 实体在 zone 层), 逐条提取。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/graph-zone-traits.h");

// —— 哨兵 ——
const sentinels = [
  [/using GraphZoneTraits = ZoneTypeTraits;/, "GraphZoneTraits 别名"],
  [/using ZoneNodePtr = GraphZoneTraits::Ptr<Node>;/, "ZoneNodePtr 别名"],
];
for (const [re, name] of sentinels) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

const body = `${banner("src/compiler/graph-zone-traits.js", "src/compiler/graph-zone-traits.h", "graph 层 zone 指针特质别名 (ZoneTypeTraits 复用)")}
//! zone 分配指针的类型别名层 — Rust 侧 zone 指针策略落地于 zone 层,
//! 此处保留镜像别名。

#![allow(dead_code)]

/// v8: src/compiler/graph-zone-traits.h GraphZoneTraits
/// 原文: using GraphZoneTraits = ZoneTypeTraits;
/// (ZoneTypeTraits 提供 zone 内分配指针的 Ptr<T> 模板)
pub use crate::zone::ZoneTypeTraits as GraphZoneTraits;

/// v8: src/compiler/graph-zone-traits.h ZoneNodePtr
/// 原文: using ZoneNodePtr = GraphZoneTraits::Ptr<Node>;
/// Rust 侧: zone 分配 Node 的裸指针包装 (非全局 GC 指针)
pub type ZoneNodePtr = <GraphZoneTraits as crate::zone::ZoneTraitsKind>::Ptr<crate::node::Node>;
`;

const p = writeSrc("v8-compiler-passes", "graph_zone_traits.rs.skel", body);
console.log(`written ${p}\n别名=2 (GraphZoneTraits / ZoneNodePtr), ZoneTypeTraits 实体在 zone 层`);
