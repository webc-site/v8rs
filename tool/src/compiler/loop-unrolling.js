#!/usr/bin/env bun
// v8/src/compiler/loop-unrolling.cc + loop-unrolling.h
// → v8-compiler-passes/src/loop_unrolling.rs.skel
//
// 规则面: kMaximumUnnestedSize=50 / kMaximumUnrollingCount=5 常量 +
// unrolling_count_heuristic / maximum_unrollable_size 两启发公式
// (逐字核对 min 语义) + UnrollLoop 签名。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/loop-unrolling.h");
const cc = readV8("src/compiler/loop-unrolling.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /static constexpr uint32_t kMaximumUnnestedSize = 50;/, "kMaximumUnnestedSize"],
  [h, /static constexpr uint32_t kMaximumUnrollingCount = 5;/, "kMaximumUnrollingCount"],
  [h, /V8_INLINE uint32_t unrolling_count_heuristic\(uint32_t size, uint32_t depth\) \{/, "heuristic"],
  [h, /return std::min\(\(depth \+ 1\) \* kMaximumUnnestedSize \/ size,/, "min 语义"],
  [h, /V8_INLINE uint32_t maximum_unrollable_size\(uint32_t depth\) \{/, "max size 公式"],
  [h, /void UnrollLoop\(Node\* loop_node, ZoneUnorderedSet<Node\*>\* loop, uint32_t depth,/, "UnrollLoop 声明"],
  [cc, /void UnrollLoop\(Node\* loop_node, ZoneUnorderedSet<Node\*>\* loop, uint32_t depth,/, "UnrollLoop 实现"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
if (!/return \(depth \+ 1\) \* kMaximumUnnestedSize;/.test(h)) throw new Error("max size 公式体缺失");

const body = `${banner("src/compiler/loop-unrolling.js", "src/compiler/loop-unrolling.h + src/compiler/loop-unrolling.cc", "loop unrolling — 循环展开 (小/深环优先启发)")}
//! UnrollLoop: 对无嵌套小环按启发式份数复制循环体。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/loop-unrolling.h kMaximumUnnestedSize
pub const K_MAXIMUM_UNNESTED_SIZE: u32 = 50;
/// v8: src/compiler/loop-unrolling.h kMaximumUnrollingCount
pub const K_MAXIMUM_UNROLLING_COUNT: u32 = 5;

/// v8: src/compiler/loop-unrolling.h unrolling_count_heuristic
/// 原文: std::min((depth + 1) * kMaximumUnnestedSize / size,
///                kMaximumUnrollingCount)
/// — 小而深的环展开更多次
pub const fn unrolling_count_heuristic(size: u32, depth: u32) -> u32 {
  ((depth + 1) * K_MAXIMUM_UNNESTED_SIZE / size).min(K_MAXIMUM_UNROLLING_COUNT)
}

/// v8: src/compiler/loop-unrolling.h maximum_unrollable_size
/// 原文: (depth + 1) * kMaximumUnnestedSize
pub const fn maximum_unrollable_size(depth: u32) -> u32 {
  (depth + 1) * K_MAXIMUM_UNNESTED_SIZE
}

/// v8: src/compiler/loop-unrolling.cc UnrollLoop
/// 语义面: NodeCopier 按 heuristic 份数复制 + 回边/计数器重构
pub fn unroll_loop(
  loop_node: crate::node::Node,
  loop: &mut std::collections::HashSet<crate::node::Node>,
  depth: u32,
  graph: &mut crate::graph::TFGraph,
  common: &mut crate::common_operator::CommonOperatorBuilder,
  tmp_zone: &mut crate::zone::Zone,
  source_positions: &mut crate::compiler_source_position_table::SourcePositionTable,
  node_origins: &mut crate::node_origin_table::NodeOriginTable,
) {
  let _ = (loop_node, loop, depth, graph, common, tmp_zone, source_positions, node_origins);
  todo!("启发份数 + 副本 + 控制流重接")
}
`;

const p = writeSrc("v8-compiler-passes", "loop_unrolling.rs.skel", body);
console.log(`written ${p}\n常量=2 (50/5) 公式=2 (完整语义) 入口=1`);
