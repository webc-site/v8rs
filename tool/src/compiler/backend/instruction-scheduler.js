#!/usr/bin/env bun
// tool/src/compiler/backend/instruction-scheduler.js
// v8/src/compiler/backend/instruction-scheduler.{h,cc}
//   → v8-compiler-backend/src/instruction_scheduler.rs.skel
// 指令调度器: ArchOpcodeFlags 位标志/资源分配/双向调度 (forward/backward)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/instruction-scheduler.h");
const cc = readV8("src/compiler/backend/instruction-scheduler.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 位标志 (真解析值)
for (const [n, v] of [["kHasSideEffect", 1], ["kIsLoadOperation", 2], ["kMayNeedDeoptOrTrapCheck", 4], ["kIsBarrier", 8]])
  must(h.includes(`${n} = ${v}`), `h 位标志 ${n}=${v}`);
must(h.includes("enum ArchOpcodeFlags {"), "h ArchOpcodeFlags");

// h: 资源表 (真解析宏展开 FOREACH_ARCH_RESOURCE 6 资源)
const res = [...h.matchAll(/#define FOREACH_ARCH_RESOURCE\(V\) \\\n((?:\s*V\(\w+\)[ \t]*\\?\n?)+)/g)][0];
if (!res) throw new Error("未找到 FOREACH_ARCH_RESOURCE");
const resources = [...res[1].matchAll(/V\((\w+)\)/g)].map((m) => m[1]);
if (resources.length !== 6) throw new Error(`资源 ${resources.length} != 6`);
for (const r of ["Fetch", "IntSingle", "IntMulti", "FP", "Load", "Store"]) must(resources.includes(r), `资源 ${r}`);
must(h.includes("kNumResources"), "h kNumResources 哨兵");

// h: 枚举 2 (方向 + 平局裁决)
must(h.includes("enum class SchedulingDirection : uint8_t {\n  kForward,\n  kBackward,\n};"), "h SchedulingDirection");
must(h.includes("enum class UseTieBreaker : uint8_t { kDisable, kEnable };"), "h UseTieBreaker");

// h: ResourceAllocation + InstructionScheduler 主类 + 嵌套 ScheduleGraphNode
must(h.includes("class ResourceAllocation {"), "h ResourceAllocation");
must(h.includes("class InstructionScheduler final : public ZoneObject {"), "h 主类");
must(h.includes("class ScheduleGraphNode : public ZoneObject {"), "h ScheduleGraphNode");
must(h.includes("V8_EXPORT_PRIVATE InstructionScheduler(Zone* zone,"), "h ctor");
must(h.includes("V8_EXPORT_PRIVATE void StartBlock(RpoNumber rpo);"), "h StartBlock");
must(h.includes("V8_EXPORT_PRIVATE void EndBlock(RpoNumber rpo, Instruction* terminator);"), "h EndBlock");
must(h.includes("V8_EXPORT_PRIVATE void AddInstruction(Instruction* instr);"), "h AddInstruction");
must(h.includes("static bool SchedulerSupported();"), "h SchedulerSupported");
// h: SchedulingQueue 模板嵌套 + 各向调度
must(h.includes("SchedulingQueue"), "h SchedulingQueue 模板");
must(h.includes("int ScheduleForward()") || h.includes("int ScheduleForward();"), "h ScheduleForward");
must(h.includes("int ScheduleBackward()") || h.includes("int ScheduleBackward();"), "h ScheduleBackward");
must(h.includes("GetInstructionFlags(const Instruction* instr) const"), "h GetInstructionFlags");
must(h.includes("ComputeTotalLatencies()"), "h ComputeTotalLatencies");

// cc: 成员实现面 (模板 + 节点 + 主类)
must(count(cc, /^[\w:<>* &]*InstructionScheduler::/gm) >= 14, "cc 实现面 ≥14");
must(cc.includes("int InstructionScheduler::ScheduleForward() {"), "cc ScheduleForward 实现");
must(cc.includes("int InstructionScheduler::ScheduleBackward() {"), "cc ScheduleBackward 实现");
must(cc.includes("void InstructionScheduler::ComputeTotalLatencies() {"), "cc ComputeTotalLatencies");
must(cc.includes("void InstructionScheduler::Reset() {"), "cc Reset");
must(cc.includes("int InstructionScheduler::GetInstructionFlags(const Instruction* instr) const {"), "cc GetInstructionFlags 实现");
must(cc.includes("PopBestCandidate(int cycle)"), "cc PopBestCandidate (贪心出队)");
// cc: 公共 opcode 分发面
must(count(cc, /case kArch/g) === 42, `cc case kArch 42, 实得 ${count(cc, /case kArch/g)}`);
must(cc.includes("UNREACHABLE();"), "cc 默认臂 UNREACHABLE");
// cc: 资源冲突判定
must(cc.includes("ResourceAllocation::PrintState()"), "cc 资源打印");
must(count(cc, /FOREACH_ARCH_RESOURCE/g) === 1, "cc 资源宏消费 (PrintState DUMP_UNITS)");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/instruction-scheduler.js",
  "src/compiler/backend/instruction-scheduler.h + instruction-scheduler.cc",
  "指令调度器: 资源模型/双向列表调度 (forward+backward 两轮取优)")}
//! 台账: ArchOpcodeFlags 位标志 4 (sideEffect=1/load=2/deoptTrap=4/barrier=8);
//!   ArchInstResource 6 资源 (Fetch/IntSingle/IntMulti/FP/Load/Store) ×
//!   ResourceAllocation (资源占用行×kNumResources 位向量);
//!   ScheduleGraphNode (前驱/后继计数 + latency/total 正反向);
//!   SchedulingQueue<direction> (AddReady/Advance/PopBestCandidate 贪心);
//!   主流程 StartBlock/EndBlock/AddInstruction → Schedule (双向:
//!   ScheduleForward/ScheduleBackward + ComputeTotalLatencies);
//!   GetInstructionFlags 公共 case kArch ×42 (架构侧另有 GetTargetInstructionFlags)。

/// v8: ArchOpcodeFlags — 指令性质位标志
pub const HAS_SIDE_EFFECT: u8 = 1;
pub const IS_LOAD_OPERATION: u8 = 2;
pub const MAY_NEED_DEOPT_OR_TRAP_CHECK: u8 = 4;
pub const IS_BARRIER: u8 = 8;

/// v8: ArchInstResource — 调度资源 (宏展开实测 6)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArchInstResource {
  /// kFetch
  Fetch = 0,
  /// kIntSingle
  IntSingle = 1,
  /// kIntMulti
  IntMulti = 2,
  /// kFP
  FP = 3,
  /// kLoad
  Load = 4,
  /// kStore
  Store = 5,
}
/// v8: ArchInstResource::kNumResources
pub const NUM_RESOURCES: usize = 6;

/// v8: SchedulingDirection — kForward/kBackward
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SchedulingDirection { Forward = 0, Backward = 1 }

/// v8: InstructionScheduler::AddInstruction — 依赖边构建
// pub fn add_instruction(&mut self, instr: &Instruction)
/// v8: InstructionScheduler::Schedule — 主驱动 (双向取优)
// pub fn schedule(&mut self)
/// v8: InstructionScheduler::ScheduleForward / ScheduleBackward
// fn schedule_forward(&mut self) -> i32 / schedule_backward
/// v8: InstructionScheduler::GetInstructionFlags — 公共面 (42 case; 架构面镜像)
// fn get_instruction_flags(&self, instr: &Instruction) -> i32
/// v8: InstructionScheduler::ComputeTotalLatencies — 全节点总延迟
// fn compute_total_latencies(&mut self)

// —— 提取不变式断言 ——
const _: () = {
  assert!(HAS_SIDE_EFFECT == 1);
  assert!(IS_BARRIER == 8);
  assert!(NUM_RESOURCES == 6);
};
`;

writeSrc("v8-compiler-backend", "instruction_scheduler.rs.skel", skel);
console.log(`instruction-scheduler.js: 对数 ${checks} 条全过; 位标志 4; 资源 6 实测; ` +
  `方向枚举 2; case kArch ×42; 产物 instruction_scheduler.rs.skel`);
