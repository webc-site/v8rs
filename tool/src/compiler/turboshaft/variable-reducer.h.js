#!/usr/bin/env bun
// tool/src/compiler/turboshaft/variable-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/variable-reducer.h
// → v8-turboshaft/src/variable_reducer.rs.skel
//
// 提取面: VariableTable (ChangeTracking 复用); kMaxAllowedMergeStateSize;
// Bind 合并协议; REDUCE(Goto); loop phi 面。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/variable-reducer.h");

// 1. 哨兵
for (const [re, name] of [
  [/class VariableReducer : public RequiredOptimizationReducer<AfterNext>/, "基栈 (强挂 RequiredOpt)"],
  [/struct VariableTable\s*\n\s*: ChangeTrackingSnapshotTable<VariableTable, OpIndex, VariableData>/, "VariableTable"],
  [/constexpr uint32_t kMaxAllowedMergeStateSize = 100'000;/, "合并上限"],
  [/struct GetActiveLoopVariablesIndex \{/, "侵入集索引"],
  [/ZoneIntrusiveSet<Variable, GetActiveLoopVariablesIndex>\s*\n\s*active_loop_variables;/, "活跃集"],
  [/V<None> REDUCE\(Goto\)\(Block\* destination, bool is_backedge\)/, "REDUCE(Goto)"],
  [/void Bind\(Block\* new_block\)/, "Bind"],
  [/PendingLoopPhi\(table_\.Get\(var\)/, "loop phi"],
  [/too_many_variables_bailouts_count_/, "bailout 计数"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. VariableData / OnValueChange 协议
const onChange = h.match(/void OnValueChange\(Variable var, OpIndex old_value, OpIndex new_value\) \{([\s\S]*?)\n  \}/);
if (!onChange) throw new Error("未找到 OnValueChange");
const loopInvariantSkip = /var\.data\(\)\.loop_invariant/.test(onChange[1]);

// 3. 公开 API 面
const api = ["SetVariable", "Set", "GetVariable", "GetPredecessorValue", "NewVariable",
  "NewLoopInvariantVariable", "CanCreateNVariables", "Bind", "RestoreTemporaryVariableSnapshotAfter",
  "CloseTemporaryVariableSnapshot", "SealAndSaveVariableSnapshot"];

let rust = banner(
  "src/compiler/turboshaft/variable-reducer.h.js",
  "src/compiler/turboshaft/variable-reducer.h",
  "变量 reducer: 块间 SSA 变量与 phi 物化",
);

rust += `
/// 原版 VariableReducer — SnapshotTable<OpIndex, VariableData> 驱动,
/// 栈内恒插 RequiredOptimizationReducer (常量 phi 须消)。
/// v8: src/compiler/turboshaft/variable-reducer.h VariableReducer
pub struct VariableReducer {
  pub table: VariableTable,
  /// block_to_snapshot_mapping_: 块 -> Snapshot
  pub snapshots: Vec<Option<Snapshot>>,
  /// loop_pending_phis_: 循环头的 PendingLoopPhi 批
  pub loop_pending_phis: HashMap<BlockIndex, Vec<(Variable, OpIndex)>>,
}

/// 原版 VariableTable — ChangeTrackingSnapshotTable 子类:
/// OnValueChange 维护 active_loop_variables 侵入集
/// (${loopInvariantSkip ? "loop_invariant 变量跳过" : "无跳过"});
/// valid->invalid 摘除, invalid->valid 加入。
/// v8: variable-reducer.h VariableReducer::VariableTable
pub struct VariableTable {
  pub active_loop_variables: IntrusiveSet<Variable>,
}

/// 原版 kMaxAllowedMergeStateSize = 100_000 —
/// max_merge_pred_count * (variable_count + n) 超限即拒绝建变量
/// (防 OOM), too_many_variables_bailouts_count_ 记数。
/// v8: variable-reducer.h VariableReducer::CanCreateNVariables
pub const MAX_ALLOWED_MERGE_STATE_SIZE: u32 = 100_000;

impl VariableReducer {
  /// v8: Bind — 前驱快照逆序后 StartNewSnapshot(merge_variables);
  /// 任一前驱 Invalid 则该变量不合并; LoadRootRegister 值恒定;
  /// 循环头为活跃变量发 PendingLoopPhi
  pub fn bind(&mut self, new_block: &Block) { todo!() }

  /// v8: REDUCE(Goto) — 发边时记录目标块前驱快照
  pub fn reduce_goto(&mut self, destination: &Block, is_backedge: bool) -> V<None> { todo!() }

  /// 其余 API 面 (${api.length} 个):
${api.filter((a) => !["Bind", "REDUCE"].includes(a)).map((a) => `  /// v8: ${a}`).join("\n")}
  pub fn api(&mut self) { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "variable_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`API=${api.length} loopInvariant跳过=${loopInvariantSkip}`);
