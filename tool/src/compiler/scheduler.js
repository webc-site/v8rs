#!/usr/bin/env bun
// tool/src/compiler/scheduler.js
// v8/src/compiler/scheduler.h + scheduler.cc
// → v8-compiler-passes/src/scheduler.rs.skel
// 调度器: 解析 Flag 位标志 / Placement 状态机 / 六阶段流水 / SchedulerData。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/scheduler.h");
const cc = readV8("src/compiler/scheduler.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`scheduler: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE Scheduler {", "Scheduler 定义"],
  ["enum Flag { kNoFlags = 0u, kSplitNodes = 1u << 1, kTempSchedule = 1u << 2 };", "Flag 位标志"],
  ["enum Placement { kUnknown, kSchedulable, kFixed, kCoupled, kScheduled };", "Placement 状态机"],
  ["struct SchedulerData {", "SchedulerData"],
  ["Schedule* Scheduler::ComputeSchedule(Zone* zone, TFGraph* graph, Flags flags,", "cc ComputeSchedule"],
  ["BasicBlockVector* Scheduler::ComputeSpecialRPO(Zone* zone, Schedule* schedule)", "cc ComputeSpecialRPO"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Flag 位值真解析 ——
const flagMatch = h.match(/kNoFlags = 0u, kSplitNodes = (1u << \d+), kTempSchedule = (1u << \d+)/);
need(flagMatch, "Flag 位值解析失败");
const splitBit = 1 << Number(flagMatch[1].replace("1u << ", ""));
const tempBit = 1 << Number(flagMatch[2].replace("1u << ", ""));
need(splitBit === 2 && tempBit === 4, `位值 ${splitBit}/${tempBit}`);

// —— Placement 成员 (顺序 = 判别值) ——
const placements = [...h.matchAll(/enum Placement \{([\s\S]*?)\};/g)][0][1].match(/k(\w+)/g).map((s) => s.slice(1));
need(placements.length === 5 && placements[0] === "Unknown" && placements[4] === "Scheduled", `Placement ${placements}`);

// —— 六阶段注释面 (真解析 cc 内 Phase 注释) ——
const phases = [...cc.matchAll(/Phase (\d): ([^\n*]+)/g)].map((m) => `${m[1]}: ${m[2].trim()}`);
need(phases.length >= 6, `阶段注释 ${phases.length}`);

// —— Scheduler 方法 (h 私有面) ——
const methods = [...h.matchAll(/^  (?!\s)(?:inline )?(?:static )?(?:[\w:<>&*\s]+?) (BuildCFG|ComputeSpecialRPONumbering|GenerateDominatorTree|PrepareUses|ScheduleEarly|ScheduleLate|SealFinalSchedule|GetPlacement|InitializePlacement|UpdatePlacement|IsLive|FuseFloatingControl|MovePlannedNodes|PropagateImmediateDominators|GetCommonDominator)\(/gm)].map((m) => m[1]);
for (const needM of ["BuildCFG", "ComputeSpecialRPONumbering", "GenerateDominatorTree", "PrepareUses", "ScheduleEarly", "ScheduleLate", "SealFinalSchedule", "GetPlacement", "InitializePlacement", "UpdatePlacement", "FuseFloatingControl", "MovePlannedNodes"]) {
  need(methods.includes(needM), `方法 ${needM} 缺席`);
}

const body = `${banner("src/compiler/scheduler.js", "src/compiler/scheduler.h + scheduler.cc", "图调度器: CFG 构建→特殊 RPO→支配树→早/晚调度→封版")}
//! scheduler — 从图计算调度
//!
//! Placement 状态机: Unknown→(Coupled|Schedulable|Fixed)→(Fixed|Scheduled);
//! 六阶段: ${phases.map((pp) => pp.replace(/^Phase \d: /, "")).join("; ")}。

/// 原版 Scheduler::Flag 位标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SchedulerFlags(u8);

impl SchedulerFlags {
  /// kNoFlags
  pub const NONE: Self = Self(0);
  /// kSplitNodes = ${flagMatch[1]}
  pub const SPLIT_NODES: Self = Self(${splitBit});
  /// kTempSchedule = ${flagMatch[2]}
  pub const TEMP_SCHEDULE: Self = Self(${tempBit});
}

/// 原版 Scheduler::Placement — 节点放置状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
  /// kUnknown (默认, 从 End 不可达)
  #[default]
  Unknown,
  /// kSchedulable — 可自由放置
  Schedulable,
  /// kFixed — 固定于某块
  Fixed,
  /// kCoupled — 与耦合控制边绑定
  Coupled,
  /// kScheduled — 已放置
  Scheduled,
}

/// 原版 Scheduler::SchedulerData — 每节点跟踪数据
pub struct SchedulerData {
  /// minimum_block_ — 最小合法 RPO 块
  pub minimum_block: *mut BasicBlock<'static>,
  /// unscheduled_count_ — 未调度使用数
  pub unscheduled_count: i32,
  /// placement_
  pub placement: Placement,
}

/// 原版 Scheduler — AllStatic 门面 + 私有实例面
pub struct Scheduler<'z> {
  /// zone_
  pub zone: &'z Zone<'z>,
  /// graph_
  pub graph: *mut TFGraph,
  /// schedule_
  pub schedule: *mut Schedule<'z>,
  /// flags_
  pub flags: SchedulerFlags,
  /// scheduled_nodes_ — 每块倒序列表
  pub scheduled_nodes: Vec<Vec<NodeVector>>,
  /// schedule_root_nodes_ — 固定根工作列表
  pub schedule_root_nodes: NodeVector,
  /// schedule_queue_ — 可调度工作队列
  pub schedule_queue: ZoneQueue<Node>,
  /// node_data_
  pub node_data: Vec<SchedulerData>,
  /// control_flow_builder_ (CFGBuilder)
  pub control_flow_builder: *mut CFGBuilder<'z>,
  /// special_rpo_ (SpecialRPONumberer)
  pub special_rpo: *mut SpecialRPONumberer<'z>,
  /// equivalence_ (ControlEquivalence)
  pub equivalence: *mut ControlEquivalence<'z>,
  /// common_dominator_cache_ — (int,int)→block 二维缓存
  pub common_dominator_cache: ZoneMap<i32, ZoneMap<i32, *mut BasicBlock<'z>>>,
}

impl Scheduler<'_, '_> {
  /// 原版 ComputeSchedule — 全流程入口 (静态)
  /// v8: src/compiler/scheduler.cc Scheduler::ComputeSchedule
  pub fn compute_schedule(temp_zone: &Zone<'_>, graph: &TFGraph, flags: SchedulerFlags, tick_counter: &mut TickCounter, profile_data: Option<&ProfileDataFromFile>) -> &Schedule<'_> {
    todo!("v8: src/compiler/scheduler.cc Scheduler::ComputeSchedule (六阶段: ${phases.length} 相位注释核对)")
  }

  /// 原版 ComputeSpecialRPO — 特殊 RPO (环尽量早出)
  /// v8: src/compiler/scheduler.cc Scheduler::ComputeSpecialRPO
  pub fn compute_special_rpo(zone: &Zone<'_>, schedule: &mut Schedule<'_>) -> &mut [BasicBlock<'_>] {
    todo!("v8: src/compiler/scheduler.cc Scheduler::ComputeSpecialRPO")
  }

  /// 原版 GenerateDominatorTree — 支配树 (按 RPO 两遍)
  /// v8: src/compiler/scheduler.cc Scheduler::GenerateDominatorTree
  pub fn generate_dominator_tree(schedule: &mut Schedule<'_>) {
    todo!("v8: src/compiler/scheduler.cc Scheduler::GenerateDominatorTree")
  }

  /// 原版 GetPlacement / InitializePlacement / UpdatePlacement — 状态机迁移
  /// v8: src/compiler/scheduler.cc Scheduler::InitializePlacement
  pub fn initialize_placement(&mut self, node: &Node) -> Placement {
    todo!("v8: src/compiler/scheduler.cc Scheduler::InitializePlacement")
  }

  /// 原版 BuildCFG — 阶段 1
  /// v8: src/compiler/scheduler.cc Scheduler::BuildCFG
  pub fn build_cfg(&mut self) {
    todo!("v8: src/compiler/scheduler.cc Scheduler::BuildCFG")
  }

  /// 原版 ScheduleEarly — 阶段 4
  /// v8: src/compiler/scheduler.cc Scheduler::ScheduleEarly
  pub fn schedule_early(&mut self) {
    todo!("v8: src/compiler/scheduler.cc Scheduler::ScheduleEarly")
  }

  /// 原版 ScheduleLate — 阶段 5
  /// v8: src/compiler/scheduler.cc Scheduler::ScheduleLate
  pub fn schedule_late(&mut self) {
    todo!("v8: src/compiler/scheduler.cc Scheduler::ScheduleLate")
  }

  /// 原版 SealFinalSchedule — 阶段 6
  /// v8: src/compiler/scheduler.cc Scheduler::SealFinalSchedule
  pub fn seal_final_schedule(&mut self) {
    todo!("v8: src/compiler/scheduler.cc Scheduler::SealFinalSchedule")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "scheduler.rs.skel", body);
console.log(`written ${p}\nflags=split:${splitBit}/temp:${tempBit} placements=${placements.length} phases=${phases.length} methods=${methods.length}`);
