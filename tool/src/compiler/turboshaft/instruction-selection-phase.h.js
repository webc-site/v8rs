#!/usr/bin/env bun
// tool/src/compiler/turboshaft/instruction-selection-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/instruction-selection-phase.{h,cc}
// → v8-turboshaft/src/instruction_selection_phase.rs.skel
//
// 提取面: 三个 phase (ProfileApplication / SpecialRPOScheduling /
// InstructionSelection) + TurboshaftSpecialRPONumberer 的 RPO 编号常量
// (kBlockOnStack/-2, kBlockVisited1/-3, kBlockVisited2/-4, kBlockUnvisited/-1)
// + SpecialRPO 核心结构 (LoopInfo/BlockData) + PropagateDeferred 入口。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/instruction-selection-phase.h");
const cc = readV8("src/compiler/turboshaft/instruction-selection-phase.cc");

// 1. 三 phase 结构
const phases = [];
for (const m of h.matchAll(/struct (\w+Phase)\s*\{([\s\S]*?)\n\};/g)) {
  const decl = m[2].match(/DECL_TURBOSHAFT_PHASE_CONSTANTS(?:_WITH_LEGACY_NAME)?\((\w+)\)/);
  if (!decl) throw new Error(`${m[1]} 缺 DECL 宏`);
  phases.push({ struct: m[1], decl: decl[1] });
}
const expectPhases = [
  { struct: "ProfileApplicationPhase", decl: "ProfileApplication" },
  { struct: "SpecialRPOSchedulingPhase", decl: "SpecialRPOScheduling" },
  { struct: "InstructionSelectionPhase", decl: "InstructionSelection" },
];
if (JSON.stringify(phases) !== JSON.stringify(expectPhases)) {
  throw new Error(`phase 集 ${JSON.stringify(phases)} != 预期 3 个`);
}

// 2. SpecialRPONumberer 常量 (对数=4, 值逐一核对)
function numConst(re, label, expect) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  if (Number(m[1]) !== expect) throw new Error(`${label}=${m[1]} 预期 ${expect}`);
  return Number(m[1]);
}
const cOnStack = numConst(/static const int kBlockOnStack = (-?\d+);/, "kBlockOnStack", -2);
const cVisited1 = numConst(/static const int kBlockVisited1 = (-?\d+);/, "kBlockVisited1", -3);
const cVisited2 = numConst(/static const int kBlockVisited2 = (-?\d+);/, "kBlockVisited2", -4);
const cUnvisited = numConst(/static const int kBlockUnvisited = (-?\d+);/, "kBlockUnvisited", -1);
const noLoop = h.match(/static constexpr size_t kNoLoopNumber = std::numeric_limits<size_t>::max\(\);/);
if (!noLoop) throw new Error("kNoLoopNumber 缺失");

// 3. 结构与方法面哨兵
for (const [re, label] of [
  [/class V8_EXPORT_PRIVATE TurboshaftSpecialRPONumberer/, "TurboshaftSpecialRPONumberer"],
  [/struct SpecialRPOStackFrame/, "SpecialRPOStackFrame"],
  [/struct LoopInfo/, "LoopInfo"],
  [/struct BlockData/, "BlockData"],
  [/ZoneVector<uint32_t> ComputeSpecialRPO\(\);/, "ComputeSpecialRPO"],
  [/void ComputeLoopInfo\(size_t num_loops, ZoneVector<Backedge>& backedges\);/, "ComputeLoopInfo"],
  [/ZoneVector<uint32_t> ComputeBlockPermutation\(const Block\* entry\);/, "ComputeBlockPermutation"],
  [/V8_EXPORT_PRIVATE void PropagateDeferred\(Graph& graph\);/, "PropagateDeferred"],
  [/static constexpr bool kOutputIsTraceableGraph = false;/, "InstructionSelection kOutputIsTraceableGraph=false"],
  [/std::optional<BailoutReason> InstructionSelectionPhase::Run\(/, "cc InstructionSelectionPhase::Run"],
  [/InstructionSelector::ForTurboshaft\(/, "cc InstructionSelector::ForTurboshaft"],
  [/graph\.ReorderBlocks\(base::VectorOf\(schedule\)\);/, "cc ReorderBlocks"],
]) {
  if (!(re.test(h) || re.test(cc))) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/instruction-selection-phase.h.js",
  "src/compiler/turboshaft/instruction-selection-phase.{h,cc}",
  "指令选择 phase 组: 特殊 RPO 编号常量与三 phase 骨架",
);
rust += `
/// v8: src/compiler/turboshaft/instruction-selection-phase.h TurboshaftSpecialRPONumberer
///
/// 特殊 RPO: 支配序 + 环体连续 (loop bodies contiguous)。
/// BasicBlock::rpo_number 编号哨兵常量 (原版 static const int):
pub const BLOCK_ON_STACK: i32 = ${cOnStack};
pub const BLOCK_VISITED1: i32 = ${cVisited1};
pub const BLOCK_VISITED2: i32 = ${cVisited2};
pub const BLOCK_UNVISITED: i32 = ${cUnvisited};
/// BlockData::kNoLoopNumber = size_t::MAX
pub const NO_LOOP_NUMBER: usize = usize::MAX;

pub struct TurboshaftSpecialRpoNumberer {
  /// block_data_: FixedBlockSidetable<BlockData> (rpo_number / loop_number / rpo_next)
  pub block_data: Vec<BlockData>,
  /// loops_: ZoneVector<LoopInfo>
  pub loops: Vec<LoopInfo>,
}

/// v8: src/compiler/turboshaft/instruction-selection-phase.h SpecialRPOStackFrame
pub struct SpecialRpoStackFrame {
  pub block: Option<crate::graph::BlockRef>,
  pub index: usize,
}

/// v8: src/compiler/turboshaft/instruction-selection-phase.h LoopInfo
/// 字段面: header / outgoing / members: SparseBitVector / prev / end / start
pub struct LoopInfo {
  pub header: crate::graph::BlockRef,
  pub members: (),
}

/// v8: src/compiler/turboshaft/instruction-selection-phase.h BlockData
pub struct BlockData {
  pub rpo_number: i32,
  pub loop_number: usize,
}

impl TurboshaftSpecialRpoNumberer {
  /// 原版 ZoneVector<uint32_t> ComputeSpecialRPO()
  /// v8: src/compiler/turboshaft/instruction-selection-phase.cc
  pub fn compute_special_rpo(&mut self) -> Vec<u32> {
    unimplemented!("Developer 修订: ComputeLoopInfo → ComputeBlockPermutation")
  }
}

/// v8: src/compiler/turboshaft/instruction-selection-phase.h PropagateDeferred
pub fn propagate_deferred(graph: &mut crate::graph::Graph) {
  unimplemented!("签名骨架, Developer 修订")
}

/// v8: src/compiler/turboshaft/instruction-selection-phase.h ProfileApplicationPhase
pub struct ProfileApplicationPhase;

impl ProfileApplicationPhase {
  /// 原版 kPhaseName = "V8.TF" "Turboshaft" "ProfileApplication"
  pub const PHASE_NAME: &'static str = "V8.TFTurboshaftProfileApplication";
  /// 原版 void Run(PipelineData*, Zone*, const ProfileDataFromFile*)
  /// PGO: BranchOp hint 就地更新 (BUILTIN_BLOCK_POSITION 分支另计数)
  pub fn run(&self) {
    unimplemented!("Developer 修订")
  }
}

/// v8: src/compiler/turboshaft/instruction-selection-phase.h SpecialRPOSchedulingPhase
pub struct SpecialRposSchedulingPhase;

impl SpecialRposSchedulingPhase {
  /// 原版 kPhaseName = "V8.TF" "Turboshaft" "SpecialRPOScheduling"
  pub const PHASE_NAME: &'static str = "V8.TFTurboshaftSpecialRPOScheduling";
  /// 原版 void Run(PipelineData*, Zone*): ComputeSpecialRPO → ReorderBlocks → PropagateDeferred
  pub fn run(&self) {
    unimplemented!("Developer 修订")
  }
}

/// v8: src/compiler/turboshaft/instruction-selection-phase.h InstructionSelectionPhase
pub struct InstructionSelectionPhase;

impl InstructionSelectionPhase {
  /// 原版 kPhaseName = "V8.TF" "Turboshaft" "InstructionSelection"
  pub const PHASE_NAME: &'static str = "V8.TFTurboshaftInstructionSelection";
  /// 原版 std::optional<BailoutReason> Run(PipelineData*, Zone*, const CallDescriptor*, Linkage*, CodeTracer*)
  /// 流程: InitializeInstructionComponent → InstructionSelector::ForTurboshaft → SelectInstructions
  pub fn run(&self) -> Option<crate::phase::BailoutReason> {
    unimplemented!("Developer 修订")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BLOCK_ON_STACK == -2);
  assert!(BLOCK_VISITED1 == -3);
  assert!(BLOCK_VISITED2 == -4);
  assert!(BLOCK_UNVISITED == -1);
};
`;

const p = writeSrc("v8-turboshaft", "instruction_selection_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`phase=${phases.length} (${phases.map((x) => x.decl).join("/")}) RPO常量=4 结构=3 (StackFrame/LoopInfo/BlockData)`);
