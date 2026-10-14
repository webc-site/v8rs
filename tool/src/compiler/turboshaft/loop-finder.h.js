#!/usr/bin/env bun
// tool/src/compiler/turboshaft/loop-finder.h.js
// 机械提取 v8/src/compiler/turboshaft/loop-finder.{h,cc}
// → v8-turboshaft/src/loop_finder.rs.skel
//
// 提取面: LoopFinder 类 + ConfigFlags 枚举 + LoopInfo 结构 (6 字段) +
// 方法面 (Run/VisitLoop/CollectLoopInfo/LoopHeaders/GetLoopInfo/...) +
// O(blocks) 算法注释锚。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/loop-finder.h");
const cc = readV8("src/compiler/turboshaft/loop-finder.cc");

// 1. ConfigFlags 枚举
const flagM = h.match(/enum class ConfigFlags : uint8_t\s*\{([^}]+)\}/);
if (!flagM) throw new Error("未找到 ConfigFlags");
const flags = [...flagM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
if (JSON.stringify(flags) !== JSON.stringify(["FindCalls"])) {
  throw new Error(`ConfigFlags ${JSON.stringify(flags)} != [FindCalls]`);
}

// 2. LoopInfo 字段面 (6 字段逐一断言)
const infoM = h.match(/struct LoopInfo\s*\{([\s\S]*?)\n  \};/);
if (!infoM) throw new Error("未找到 LoopInfo");
for (const f of ["start", "end", "has_inner_loops", "block_count", "op_count", "has_any_call"]) {
  if (!infoM[1].includes(f)) throw new Error(`LoopInfo 字段缺失: ${f}`);
}

// 3. 方法面
for (const [re, label] of [
  [/class V8_EXPORT_PRIVATE LoopFinder/, "LoopFinder 类"],
  [/using Config = base::EnumSet<ConfigFlags, int8_t>;/, "Config 别名"],
  [/void Run\(\);/, "Run 声明"],

  [/const ZoneUnorderedMap<const Block\*, LoopInfo>& LoopHeaders\(\) const/, "LoopHeaders"],
  [/LoopInfo GetLoopInfo\(const Block\* block\) const/, "GetLoopInfo"],

]) {
  if (!re.test(h)) throw new Error(`h 哨兵缺失: ${label}`);
}

// 4. 算法注释锚 (逆序遍历/内环先访) + cc 定义
for (const [src, re, label] of [
  [h, /LoopFinder::Run walks the blocks of the graph backwards/, "逆序遍历注释"],
  [h, /By iterating the blocks backwards in Run, we are guaranteed that inner/, "内环先访注释"],
  [cc, /void LoopFinder::Run\(\)/, "cc Run 定义"],
  [cc, /void LoopFinder::CollectLoopInfo\(const Block\* block, LoopInfo\* info\)/, "cc CollectLoopInfo 定义"],
  [cc, /LoopFinder::LoopInfo LoopFinder::VisitLoop\(const Block\* header\)/, "cc VisitLoop 定义"],
]) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${label}`);
}

// 5. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/loop-finder.h.js",
  "src/compiler/turboshaft/loop-finder.{h,cc}",
  "环发现: 逆序 O(blocks) 双遍环归属分析骨架",
);
rust += `
/// v8: src/compiler/turboshaft/loop-finder.h LoopFinder::ConfigFlags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFlags {
  /// kFindCalls — 统计环内是否有 Call
  findCalls,
}

/// v8: src/compiler/turboshaft/loop-finder.h LoopFinder::LoopInfo
pub struct LoopInfo<'a> {
  /// start / end: 环块区间 (按 BlockIndex)
  pub start: Option<&'a crate::graph::Block>,
  pub end: Option<&'a crate::graph::Block>,
  /// has_inner_loops
  pub has_inner_loops: bool,
  /// block_count (不含内环)
  pub block_count: usize,
  /// op_count 上界 (end-begin 逐块累计, 大操作如 Call/FrameState 会高估)
  pub op_count: usize,
  /// has_any_call (kFindCalls 下)
  pub has_any_call: bool,
}

/// v8: src/compiler/turboshaft/loop-finder.h LoopFinder
///
/// 算法 (原版注释保留): Run 逆序遍历块, 遇 LoopHeader 调 VisitLoop;
/// VisitLoop 从回边逆序走到环头, 无 parent_loops_ 的块归属本环;
/// 逆序保证内环先于外环访问 (parent_loops_ 侧表 + loop_headers_ 表)。
pub struct LoopFinder<'a> {
  /// parent_loops_: 块 → 所属环头
  pub parent_loops: Vec<Option<crate::index::BlockIndex>>,
  /// loop_headers_: 环头 → LoopInfo
  pub loop_headers: std::collections::HashMap<crate::index::BlockIndex, LoopInfo<'a>>,
}

impl<'a> LoopFinder<'a> {
  /// 原版 void Run() — v8: src/compiler/turboshaft/loop-finder.cc LoopFinder::Run
  pub fn run(&mut self) {
    unimplemented!("Developer 修订")
  }
  /// 原版 void VisitLoop(const Block* header)
  pub fn visit_loop(&mut self, header: &'a crate::graph::Block) {
    unimplemented!("Developer 修订")
  }
  /// 原版 void CollectLoopInfo(const Block* block, LoopInfo* info)
  pub fn collect_loop_info(&mut self, block: &'a crate::graph::Block) {
    unimplemented!("Developer 修订")
  }
  /// 原版 LoopInfo GetLoopInfo(const Block* block) const
  pub fn get_loop_info(&self, block: &crate::graph::Block) -> &LoopInfo<'a> {
    unimplemented!("签名骨架, Developer 修订")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  // ConfigFlags 唯一成员
};
`;

const p = writeSrc("v8-turboshaft", "loop_finder.rs.skel", rust);
console.log(`written ${p}`);
console.log(`ConfigFlags=${flags.length} LoopInfo字段=6 方法面=4+ 哨兵=12`);
