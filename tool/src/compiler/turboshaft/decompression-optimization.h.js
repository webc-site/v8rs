#!/usr/bin/env bun
// tool/src/compiler/turboshaft/decompression-optimization.h.js
// 机械提取 v8/src/compiler/turboshaft/decompression-optimization.{h,cc}
// → v8-turboshaft/src/decompression_optimization.rs.skel
//
// 提取面: RunDecompressionOptimization 入口 + DecompressionAnalyzer 字段面
// + ProcessOperation 一级 opcode 分派表 (10 case) + Run 内就地改写表
// (4 case: Constant/Phi/Load/TaggedBitcast)。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/decompression-optimization.h");
const cc = readV8("src/compiler/turboshaft/decompression-optimization.cc");

// 1. h 侧: 唯一公开入口
const entryM = h.match(/void (RunDecompressionOptimization)\(Graph& graph, Zone\* phase_zone\);/);
if (!entryM) throw new Error("h 未找到 RunDecompressionOptimization 声明");

// 2. cc 侧: DecompressionAnalyzer 字段面 + 方法面
for (const [re, label] of [
  [/struct DecompressionAnalyzer\s*\{/, "DecompressionAnalyzer"],
  [/const Graph& graph;/, "字段 graph"],
  [/Zone\* phase_zone;/, "字段 phase_zone"],
  [/FixedOpIndexSidetable<uint8_t> needs_decompression;/, "字段 needs_decompression (u8 表, 避免 vector<bool> 位压缩)"],
  [/ZoneVector<OpIndex> candidates;/, "字段 candidates"],
  [/candidates\.reserve\(graph\.op_id_count\(\) \/ 8\);/, "candidates 预留 op_id_count/8"],
  [/void Run\(\)/, "Analyzer::Run (逆块序)"],
  [/bool NeedsDecompression\(OpIndex op\)/, "NeedsDecompression(OpIndex)"],
  [/bool MarkAsNeedsDecompression\(OpIndex op\)/, "MarkAsNeedsDecompression"],
  [/void ProcessOperation\(const Operation& op\);/, "ProcessOperation 声明"],
  [/void MarkAddressingBase\(OpIndex base_idx\);/, "MarkAddressingBase 声明"],
]) {
  if (!re.test(cc)) throw new Error(`哨兵缺失: ${label}`);
}

// 3. ProcessOperation 一级 opcode 分派 (switch 到 default 前)
const procM = cc.match(/void DecompressionAnalyzer::ProcessOperation\(const Operation& op\) \{\s*switch \(op\.opcode\) \{([\s\S]*?)\n    default:/);
if (!procM) throw new Error("未找到 ProcessOperation switch");
const analysisCases = [...procM[1].matchAll(/case Opcode::k(\w+):/g)].map((m) => m[1]);
const expectAnalysis = ["Store", "FrameState", "Phi", "Comparison", "WordBinop", "Shift", "Change", "TaggedBitcast", "Constant", "Load"];
if (JSON.stringify(analysisCases) !== JSON.stringify(expectAnalysis)) {
  throw new Error(`分析分派表 ${JSON.stringify(analysisCases)} != 预期 10 项`);
}

// 4. RunDecompressionOptimization 就地改写表 (default 前)
const runM = cc.match(/void RunDecompressionOptimization\(Graph& graph, Zone\* phase_zone\) \{([\s\S]*?)\n\}/);
if (!runM) throw new Error("未找到 RunDecompressionOptimization 定义");
const rewriteCases = [...runM[1].matchAll(/case Opcode::k(\w+): \{/g)].map((m) => m[1]);
const expectRewrite = ["Constant", "Phi", "Load", "TaggedBitcast"];
if (JSON.stringify(rewriteCases) !== JSON.stringify(expectRewrite)) {
  throw new Error(`改写分派表 ${JSON.stringify(rewriteCases)} != 预期 4 项`);
}
// 非 CopyingPhase 声明锚
if (!/Instead of using `CopyingPhase`, we directly mutate the operations/.test(cc)) {
  throw new Error("非 CopyingPhase 设计注释锚缺失");
}

// 5. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/decompression-optimization.h.js",
  "src/compiler/turboshaft/decompression-optimization.{h,cc}",
  "指针解压优化: 逆向使用分析 + 就地压缩改写骨架",
);
rust += `
/// v8: src/compiler/turboshaft/decompression-optimization.cc DecompressionAnalyzer
///
/// 逆向使用分析: 自后向前遍历, 标记需要解压的值; 环 phi 需定点
/// (重访环回块), candidates 记录可能变化的操作避免全图重扫。
/// 字段面: graph / phase_zone / needs_decompression: u8 侧表 / candidates: Vec<OpIndex>
pub struct DecompressionAnalyzer {
  pub needs_decompression: Vec<u8>,
  pub candidates: Vec<crate::index::OpIndex>,
}

impl DecompressionAnalyzer {
  /// 原版 Run(): 逆块序; Loop 块走 ProcessBlock<true>(phi 定点), 其余 <false>
  pub fn run(&mut self) {
    unimplemented!("Developer 修订: 逆块序 ProcessBlock + ProcessOperation")
  }

  /// ProcessOperation 一级分派表 (保序, 原版 ${analysisCases.length} case):
${analysisCases.map((c) => `  ///   k${c}`).join("\n")}
  /// default: 全部 inputs 标记 NeedsDecompression
  pub fn process_operation(&mut self, op: &crate::operations::Operation) {
    unimplemented!("Developer 修订: 按分派表处理")
  }

  /// MarkAddressingBase: Load 基址若可走复杂寻址模式保持压缩
  /// (DECOMPRESS_POINTER_BY_ADDRESSING_MODE 下, phi 输入全为可压缩 Load 且唯一使用者)
  pub fn mark_addressing_base(&mut self, base_idx: crate::index::OpIndex) {
    unimplemented!("签名骨架, Developer 修订")
  }
}

/// v8: src/compiler/turboshaft/decompression-optimization.h RunDecompressionOptimization
///
/// 非 CopyingPhase: 分析后对 candidates 就地改写操作选项。改写表 (${rewriteCases.length} case):
${rewriteCases.map((c) => `/// - k${c}`).join("\n")}
/// - kConstant: kHeapObject → kCompressedHeapObject
/// - kPhi: rep Tagged → Compressed
/// - kLoad: loaded_rep 可压缩时 result_rep → Compressed
/// - kTaggedBitcast: from Tagged → Compressed / to WordPtr|kSmi → Word32
pub fn run_decompression_optimization(graph: &mut crate::graph::Graph, phase_zone: *mut crate::phase::Zone) {
  unimplemented!("Developer 修订: analyzer.run() + candidates 就地改写")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ANALYSIS_CASES.len() == ${analysisCases.length});
  assert!(REWRITE_CASES.len() == ${rewriteCases.length});
};
const ANALYSIS_CASES: &[&str] = &[${analysisCases.map((c) => `"${c}"`).join(", ")}];
const REWRITE_CASES: &[&str] = &[${rewriteCases.map((c) => `"${c}"`).join(", ")}];
`;

const p = writeSrc("v8-turboshaft", "decompression_optimization.rs.skel", rust);
console.log(`written ${p}`);
console.log(`入口=1 分析分派=${analysisCases.length} 改写分派=${rewriteCases.length} (${analysisCases.join("/")})`);
