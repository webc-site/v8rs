#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-revec-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-revec-reducer.{h,cc}
// → v8-turboshaft/src/wasm_revec_reducer.rs.skel
//
// 提取面: NodeGroup 常量; PackNode::NodeType 枚举 (5); ShufflePackNode
// SpecificInfo::Kind; SLPTree 常量; analyzer/reducer 结构面。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-revec-reducer.h");
const cc = readV8("src/compiler/turboshaft/wasm-revec-reducer.cc");

// 1. NodeGroup 常量
const ngM = h.match(/static constexpr int kSize = kSimd256Size \/ kSimd128Size;/);
if (!ngM) throw new Error("未找到 NodeGroup::kSize");

// 2. NodeType 枚举
const ntM = h.match(/enum NodeType : uint8_t \{([^}]+)\}/);
if (!ntM) throw new Error("未找到 NodeType");
const nodeTypes = [...ntM[1].replace(/\/\/[^\n]*/g, "").matchAll(/k(\w+)/g)].map((m) => m[1]);
if (nodeTypes.length !== 5) throw new Error(`NodeType ${nodeTypes.length} != 5`);

// 3. ShufflePackNode::SpecificInfo::Kind
const skM = h.match(/class SpecificInfo \{[\s\S]*?enum class Kind \{([^}]+)\}/);
if (!skM) throw new Error("未找到 SpecificInfo::Kind");
const shuffleKinds = [...skM[1].replace(/\/\/[^\n]*/g, "").matchAll(/k(\w+)/g)].map((m) => m[1]);
if (shuffleKinds.length < 3) throw new Error(`SpecificInfo::Kind ${shuffleKinds.length} < 3`);

// 4. SLPTree 常量与结构
const maxDepth = h.match(/static constexpr size_t kMaxRecursionDepth = (\d+);/);
if (!maxDepth) throw new Error("未找到 kMaxRecursionDepth");

// 5. 结构哨兵
for (const [re, name] of [
  [/class NodeGroup \{/, "NodeGroup"],
  [/class PackNode : public NON_EXPORTED_BASE\(ZoneObject\) \{/, "PackNode"],
  [/class ForcePackNode : public PackNode \{/, "ForcePackNode"],
  [/enum ForcePackType : uint8_t \{/, "ForcePackType"],
  [/class ShufflePackNode : public PackNode \{/, "ShufflePackNode"],
  [/class BundlePackNode : public PackNode \{/, "BundlePackNode"],
  [/class SLPTree : public NON_EXPORTED_BASE\(ZoneObject\) \{/, "SLPTree"],
  [/struct LaneExtendInfo \{/, "LaneExtendInfo"],
  [/class WasmRevecAnalyzer \{/, "analyzer"],
  [/class WasmRevecReducer : public UniformReducerAdapter<WasmRevecReducer, Next>/, "reducer (Adapter 基)"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(WasmRevec\)/, "boilerplate"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 6. cc 方法面
const slpMethods = [...new Set([...cc.matchAll(/^void SLPTree::(\w+)\(|^bool SLPTree::(\w+)\(/gm)].map((m) => m[1] || m[2]))];
const analyzerMethods = [...new Set([...cc.matchAll(/^void WasmRevecAnalyzer::(\w+)\(|^bool WasmRevecAnalyzer::(\w+)\(/gm)].map((m) => m[1] || m[2]))];

let rust = banner(
  "src/compiler/turboshaft/wasm-revec-reducer.h.js",
  "src/compiler/turboshaft/wasm-revec-reducer.{h,cc}",
  "wasm 重向量化: SLP 式 128->256 位打包",
);

rust += `
/// 原版 NodeGroup — 128 位节点对 (${2}) 为重向量化单元
/// v8: wasm-revec-reducer.h NodeGroup
pub struct NodeGroup {
  /// kSize = kSimd256Size / kSimd128Size
  pub nodes: [OpIndex; 2],
}

/// 原版 PackNode::NodeType — 打包节点五类
/// v8: wasm-revec-reducer.h PackNode::NodeType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NodeType {
  /// kDefault: 自然成对
  ${nodeTypes[0]} = 0,
  /// kForcePackNode: 不满足打包规则但可 Pack128To256 强合 (x64 vinsertf128)
  ${nodeTypes[1]} = 1,
  /// kShufflePackNode: 特定 shuffle 信息
  ${nodeTypes[2]} = 2,
  /// kBundlePackNode: i8x16/i16x8 -> f32x4 转换束
  ${nodeTypes[3]} = 3,
  /// kIntersectPackNode: 已被现有 PackNode 打包的节点
  ${nodeTypes[4]} = 4,
}

/// 原版 ShufflePackNode::SpecificInfo::Kind (${shuffleKinds.length} 态):
${shuffleKinds.map((k) => `/// - k${k}`).join("\n")}
/// v8: wasm-revec-reducer.h ShufflePackNode::SpecificInfo::Kind
pub enum ShufflePackKind;

/// 原版 SLPTree — superword level parallelism 树, 递归深度上限
/// kMaxRecursionDepth = ${maxDepth[1]}。
/// v8: wasm-revec-reducer.h SLPTree
pub const K_MAX_RECURSION_DEPTH: usize = ${maxDepth[1]};

/// 原版结构面:
/// - ForcePackNode (ForcePackType 枚举) / ShufflePackNode / BundlePackNode
///   三类 PackNode 子类; is_force_packing() 含 kForcePackNode 与
///   kIntersectPackNode
/// - SLPTree 方法面 (cc, ${slpMethods.length} 个): ${slpMethods.join(", ")}
/// - WasmRevecAnalyzer 方法面 (cc, ${analyzerMethods.length} 个): ${analyzerMethods.join(", ")}
/// - WasmRevecReducer: UniformReducerAdapter 基, 按树重写
/// v8: src/compiler/turboshaft/wasm-revec-reducer.{h,cc}
pub struct WasmRevecAnalyzer;
pub struct WasmRevecReducer;
`;

const p = writeSrc("v8-turboshaft", "wasm_revec_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`NodeType=${nodeTypes.length} ShuffleKind=${shuffleKinds.length} SLP方法=${slpMethods.length} analyzer方法=${analyzerMethods.length}`);
