#!/usr/bin/env bun
// tool/src/snapshot/sort-builtins.js
// 机械提取 v8/src/snapshot/sort-builtins.{h,cc} 的聚类排序常量与类型别名

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/sort-builtins.h");
const ccSrc = readV8("src/snapshot/sort-builtins.cc");

// 1. BuiltinsSorter 常量 (h, 类内 const)
const minEdgeM = hSrc.match(/const int32_t kMinEdgeProbabilityThreshold = (\d+);/);
const maxClusterM = hSrc.match(/const uint32_t kMaxClusterSize = (\d+) \* MB;/);
const maxDensityM = hSrc.match(/const uint32_t kMaxDensityDecreaseThreshold = (\d+);/);
if (!minEdgeM || !maxClusterM || !maxDensityM) throw new Error("未找到 BuiltinsSorter 常量组");
const minEdge = parseInt(minEdgeM[1], 10);
const maxClusterMb = parseInt(maxClusterM[1], 10); // 单位 MB
const maxDensity = parseInt(maxDensityM[1], 10);
if (minEdge !== 10 || maxClusterMb !== 1 || maxDensity !== 8) {
  throw new Error(`BuiltinsSorter 常量异常: ${minEdge}/${maxClusterMb}/${maxDensity}`);
}

// 2. marker 字符串
const markerBlockM = hSrc.match(/const std::string kBuiltinCallBlockDensityMarker = "(\w+)";/);
const markerBuiltinM = hSrc.match(/const std::string kBuiltinDensityMarker = "(\w+)";/);
if (!markerBlockM || !markerBuiltinM) throw new Error("未找到密度 marker");
if (markerBlockM[1] !== "block_count" || markerBuiltinM[1] !== "builtin_count") {
  throw new Error("marker 字符串异常");
}

// 3. 类型别名族 (h)
const aliases = [
  { cpp: "CallProbabilities", rust: "CallProbabilities" },
  { cpp: "CallGraph", rust: "CallGraph" },
  { cpp: "BuiltinDensityMap", rust: "BuiltinDensityMap" },
  { cpp: "BuiltinSize", rust: "BuiltinSize" },
  { cpp: "BuiltinClusterMap", rust: "BuiltinClusterMap" },
];
for (const a of aliases) {
  if (!hSrc.includes(`using ${a.cpp} =`)) throw new Error(`未找到别名 ${a.cpp}`);
}

// 4. BuiltinsSorter 方法面
const clsM = hSrc.match(/class BuiltinsSorter \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 BuiltinsSorter 类体");
const methods = [...clsM[1].matchAll(/^\s{2}(?:std::vector<Builtin>|void|Builtin)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 8) {
  throw new Error(`BuiltinsSorter 方法数不符: 期望 8, 实得 ${methods.length}: ${methods.join(",")}`);
}
console.log(`常量 3 + marker 2 + 别名 ${aliases.length} + 方法 ${methods.length}`);

let rust = banner("src/snapshot/sort-builtins.js", "src/snapshot/sort-builtins.{h,cc}", "内建函数内嵌快照排序器");
rust += `
/// BuiltinsSorter 聚类常量 (原版 h, 类内 const, 机械展开):
/// 相邻聚类合并的最小边概率阈值 (千分比)
pub const MIN_EDGE_PROBABILITY_THRESHOLD: i32 = ${minEdge};
/// 单聚类大小上限 (原版 1 * MB)
pub const MAX_CLUSTER_SIZE: u32 = ${maxClusterMb} * 1024 * 1024;
/// 密度允许下降阈值
pub const MAX_DENSITY_DECREASE_THRESHOLD: u32 = ${maxDensity};

/// 性能日志 marker (原版 std::string 常量):
pub const BUILTIN_CALL_BLOCK_DENSITY_MARKER: &str = "${markerBlockM[1]}";
pub const BUILTIN_DENSITY_MARKER: &str = "${markerBuiltinM[1]}";

/// 类型别名族 (原版 using, 语义由 Developer 落型):
`;
for (const a of aliases) rust += `/// - \`${a.cpp}\`\n`;
rust += `
/// BuiltinsSorter 方法面 (原版 h, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// 排序契约: 读 profiling 日志 (块计数/内建计数 marker) → 建调用图 →
/// 按最佳前驱聚类合并 (上限 MAX_CLUSTER_SIZE) → 聚类内按密度排序输出
/// 内嵌快照序 (embedded snapshot order, 对齐 embedded-data.rs 布局)。
`;

const p = writeSrc("v8-snapshot", "sort_builtins.rs", rust);
console.log(`written ${p}`);
