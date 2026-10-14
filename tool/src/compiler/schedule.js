#!/usr/bin/env bun
// tool/src/compiler/schedule.js
// v8/src/compiler/schedule.h + schedule.cc
// → v8-compiler-passes/src/schedule.rs.skel
// 调度结构: 解析 BasicBlock::Control 枚举 / setter-getter 面 / Schedule 方法与 cc 对数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/schedule.h");
const cc = readV8("src/compiler/schedule.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`schedule: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE BasicBlock final", "BasicBlock"],
  ["enum Control {", "Control 枚举"],
  ["class V8_EXPORT_PRIVATE Schedule final", "Schedule"],
  ["friend class Scheduler;", "Scheduler 友元"],
  ["BasicBlock::BasicBlock(Zone* zone, Id id)", "cc ctor"],
  ["void Schedule::AddNode(BasicBlock* block, Node* node)", "cc AddNode"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Control 枚举成员 (真解析, 顺序敏感) ——
const ctrlBody = h.slice(h.indexOf("enum Control {"), h.indexOf("};", h.indexOf("enum Control {")));
const controls = [...ctrlBody.matchAll(/\bk(\w+)[, ]/g)].map((m) => m[1]);
need(controls.length === 9 && controls[0] === "None" && controls[8] === "Throw", `Control 成员 ${controls}`);
need(controls.indexOf("Goto") === 1 && controls.indexOf("Return") === 7, "Control 顺序异常");

// —— BasicBlock 公有方法 (2 空格缩进签名, 外层 public 区) ——
const bbBody = h.slice(h.indexOf("class V8_EXPORT_PRIVATE BasicBlock final"), h.indexOf("class V8_EXPORT_PRIVATE Schedule final"));
const bbPub = bbBody.slice(0, bbBody.lastIndexOf("\n private:"));
const bbMethods = [...bbPub.matchAll(/^  (?!\s)(?:[\w:<>&* ]+?) (\w+)\(([^()]*)\)(?: const)?\s*[;{]/gm)].map((m) => m[1]);
for (const needM of ["AddPredecessor", "RemovePredecessor", "AddSuccessor", "AddNode", "TrimNodes", "ResetRPOInfo", "set_control", "control", "IsLoopHeader", "LoopContains", "GetCommonDominator"]) {
  need(bbMethods.includes(needM), `BasicBlock 缺 ${needM} (实得 ${bbMethods.length})`);
}

// —— Schedule 公有方法 ——
const schedBody = h.slice(h.indexOf("class V8_EXPORT_PRIVATE Schedule final"));
const schedPub = schedBody.slice(0, schedBody.lastIndexOf("\n private:"));
const schedMethods = [...schedPub.matchAll(/^  (?!\s)(?:[\w:<>&* ]+?) (\w+)\(([^()]*)\)(?: const)?\s*[;{]/gm)].map((m) => m[1]);
for (const needM of ["NewBasicBlock", "PlanNode", "AddNode", "AddGoto", "AddCall", "AddBranch", "AddSwitch", "AddTailCall", "AddReturn", "AddThrow", "AddDeoptimize", "block", "GetBlockById", "RpoBlockCount", "BasicBlockCount"]) {
  need(schedMethods.includes(needM), `Schedule 缺 ${needM}`);
}

// —— cc 定义对数 ——
const ccDefs = [
  "BasicBlock::BasicBlock(Zone* zone, Id id)",
  "void BasicBlock::AddPredecessor(BasicBlock* predecessor)",
  "void BasicBlock::AddSuccessor(BasicBlock* successor)",
  "void BasicBlock::AddNode(Node* node)",
  "void BasicBlock::set_control(Control control)",
  "bool BasicBlock::LoopContains(BasicBlock* block) const",
  "BasicBlock* BasicBlock::GetCommonDominator(BasicBlock* b1, BasicBlock* b2)",
  "std::ostream& operator<<(std::ostream& os, const BasicBlock::Control& c)",
  "Schedule::Schedule(Zone* zone, size_t node_count_hint)",
  "BasicBlock* Schedule::NewBasicBlock()",
  "void Schedule::PlanNode(BasicBlock* block, Node* node)",
  "void Schedule::AddNode(BasicBlock* block, Node* node)",
  "void Schedule::AddGoto(BasicBlock* block, BasicBlock* succ)",
  "void Schedule::AddCall(BasicBlock* block, Node* call, BasicBlock* success_block,",
  "void Schedule::AddBranch(BasicBlock* block, Node* branch, BasicBlock* tblock,",
  "void Schedule::AddSwitch(BasicBlock* block, Node* sw, BasicBlock** succ_blocks,",
  "void Schedule::AddTailCall(BasicBlock* block, Node* input)",
  "void Schedule::AddReturn(BasicBlock* block, Node* input)",
  "void Schedule::AddThrow(BasicBlock* block, Node* input)",
  "void Schedule::AddDeoptimize(BasicBlock* block, Node* input)",
];
for (const d of ccDefs) need(cc.includes(d), `cc 缺 ${d}`);

const body = `${banner("src/compiler/schedule.js", "src/compiler/schedule.h + schedule.cc", "基本块调度结构与图-调度映射")}
//! schedule — BasicBlock (节点序列 + 控制尾) + Schedule (RPO 序 + node→block 映射)
//!
//! Control 枚举 ${controls.length} 值与原版顺序一致 (判别值参与 JSON 导出);
//! loop_header/loop_end 双向链支撑 LoopContains; dominator_depth 缓存支配深。

/// 原版 BasicBlock::Control — 块尾控制种类 (顺序 = 判别值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Control {
${controls.map((c) => `  /// k${c}\n  ${c === "None" ? "#[default]\n  " : ""}${c},`).join("\n")}
}

/// 原版 BasicBlock::Id — 块序号 (newtype)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasicBlockId(pub usize);

/// 原版 BasicBlock — 基本块
pub struct BasicBlock<'z> {
  /// nodes_
  pub nodes: Vec<*mut Node>,
  /// deferred_ — 含延迟代码
  pub deferred: bool,
  /// dominator_depth_
  pub dominator_depth: i32,
  /// dominator_
  pub dominator: Option<*mut BasicBlock<'z>>,
  /// rpo_next_
  pub rpo_next: Option<*mut BasicBlock<'z>>,
  /// loop_header_
  pub loop_header: Option<*mut BasicBlock<'z>>,
  /// loop_end_
  pub loop_end: Option<*mut BasicBlock<'z>>,
  /// loop_depth_
  pub loop_depth: i32,
  /// loop_number_
  pub loop_number: i32,
  /// rpo_number_
  pub rpo_number: i32,
  /// pgo_execution_count_
  pub pgo_execution_count: u64,
  /// control_
  pub control: Control,
  /// control_input_
  pub control_input: *mut Node,
  /// successors_
  pub successors: Vec<*mut BasicBlock<'z>>,
  /// predecessors_
  pub predecessors: Vec<*mut BasicBlock<'z>>,
  /// id_
  pub id: BasicBlockId,
}

impl BasicBlock<'_> {
  /// 原版 AddPredecessor / RemovePredecessor
  /// v8: src/compiler/schedule.cc BasicBlock::AddPredecessor
  pub fn add_predecessor(&mut self, predecessor: *mut BasicBlock<'_>) {
    todo!("v8: src/compiler/schedule.cc BasicBlock::AddPredecessor")
  }

  /// 原版 AddSuccessor
  /// v8: src/compiler/schedule.cc BasicBlock::AddSuccessor
  pub fn add_successor(&mut self, successor: *mut BasicBlock<'_>) {
    todo!("v8: src/compiler/schedule.cc BasicBlock::AddSuccessor")
  }

  /// 原版 IsLoopHeader — loop_end_ != nullptr
  /// v8: src/compiler/schedule.h BasicBlock::IsLoopHeader
  pub fn is_loop_header(&self) -> bool {
    self.loop_end.is_some()
  }

  /// 原版 LoopContains — loop_number 区间包含判定
  /// v8: src/compiler/schedule.cc BasicBlock::LoopContains
  pub fn loop_contains(&self, block: &BasicBlock<'_>) -> bool {
    todo!("v8: src/compiler/schedule.cc BasicBlock::LoopContains")
  }

  /// 原版 GetCommonDominator
  /// v8: src/compiler/schedule.cc BasicBlock::GetCommonDominator
  pub fn get_common_dominator(b1: *mut BasicBlock<'_>, b2: *mut BasicBlock<'_>) -> *mut BasicBlock<'_> {
    todo!("v8: src/compiler/schedule.cc BasicBlock::GetCommonDominator")
  }

  /// 原版 ResetRPOInfo
  /// v8: src/compiler/schedule.cc BasicBlock::ResetRPOInfo
  pub fn reset_rpo_info(&mut self) {
    todo!("v8: src/compiler/schedule.cc BasicBlock::ResetRPOInfo")
  }
}

/// 原版 Schedule — 调度容器 (node→block + RPO)
pub struct Schedule<'z> {
  /// all_blocks_ (block Id 索引)
  pub all_blocks: Vec<Box<BasicBlock<'z>>>,
  /// rpo_order_
  pub rpo_order: Vec<*mut BasicBlock<'z>>,
  /// nodeid_to_block_
  pub nodeid_to_block: Vec<*mut BasicBlock<'z>>,
  /// graph_ (装饰器面)
  pub graph: *mut TFGraph,
  /// zone_
  pub zone: &'z Zone<'z>,
}

impl Schedule<'_> {
  /// 原版 NewBasicBlock
  /// v8: src/compiler/schedule.cc Schedule::NewBasicBlock
  pub fn new_basic_block(&mut self) -> *mut BasicBlock<'_> {
    todo!("v8: src/compiler/schedule.cc Schedule::NewBasicBlock")
  }

  /// 原版 PlanNode — 计划加入 (尚未真加)
  /// v8: src/compiler/schedule.cc Schedule::PlanNode
  pub fn plan_node(&mut self, block: *mut BasicBlock<'_>, node: *mut Node) {
    todo!("v8: src/compiler/schedule.cc Schedule::PlanNode")
  }

  /// 原版 AddNode — 真加入块
  /// v8: src/compiler/schedule.cc Schedule::AddNode
  pub fn add_node(&mut self, block: *mut BasicBlock<'_>, node: *mut Node) {
    todo!("v8: src/compiler/schedule.cc Schedule::AddNode")
  }

  /// 原版 AddGoto / AddCall / AddBranch / AddSwitch / AddTailCall / AddReturn / AddThrow / AddDeoptimize — 8 个控制尾构造 (${schedMethods.length} 公有方法面, 均返回 void)
  /// v8: src/compiler/schedule.cc Schedule::AddBranch
  pub fn add_branch(&mut self, block: *mut BasicBlock<'_>, branch: *mut Node, tblock: *mut BasicBlock<'_>, fblock: *mut BasicBlock<'_>) {
    todo!("v8: src/compiler/schedule.cc Schedule::AddBranch (同族 AddGoto/AddCall/AddSwitch/AddTailCall/AddReturn/AddThrow/AddDeoptimize)")
  }

  /// 原版 block — node → 所在块
  /// v8: src/compiler/schedule.h Schedule::block
  pub fn block(&self, node: &Node) -> *mut BasicBlock<'_> {
    todo!("v8: src/compiler/schedule.h Schedule::block")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "schedule.rs.skel", body);
console.log(`written ${p}\ncontrol=${controls.length} bb_methods=${bbMethods.length} sched_methods=${schedMethods.length} cc_defs=${ccDefs.length}`);
