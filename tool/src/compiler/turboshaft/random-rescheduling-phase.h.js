#!/usr/bin/env bun
// tool/src/compiler/turboshaft/random-rescheduling-phase.h.js
// 机械提取 v8/src/compiler/turboshaft/random-rescheduling-phase.{h,cc}
// → v8-turboshaft/src/random_rescheduling_phase.rs.skel
//
// 提取面: ReschedulingReducer::OpState 枚举; RandomRescheduler 组装器
// 方法面 (Visit*/ComputeReadyOperations); phase 入口。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/random-rescheduling-phase.h");
const cc = readV8("src/compiler/turboshaft/random-rescheduling-phase.cc");

// 1. OpState 枚举
const osM = cc.match(/enum class OpState \{([^}]+)\}/);
if (!osM) throw new Error("未找到 OpState");
const opStates = [...osM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
if (opStates.length !== 4) throw new Error(`OpState ${opStates.length} != 4`);

// 2. 哨兵
for (const [re, name] of [
  [/class ReschedulingReducer : public Next/, "ReschedulingReducer"],
  [/class RandomRescheduler : public Assembler<ReschedulingReducer, GraphVisitor>/, "RandomRescheduler"],
  [/struct RandomReschedulingPhase/, "phase struct"],
  [/void RandomReschedulingPhase::Run/, "phase Run 实现"],
  [/base::RandomNumberGenerator rng_;/, "RNG"],
  [/GetOrCreateCompanion\(\)/, "companion graph"],
]) {
  if (!re.test(h) && !re.test(cc)) throw new Error(`哨兵缺失: ${name}`);
}

// 3. RandomRescheduler 方法面
const visitMethods = [...cc.matchAll(/^\s{2}void (Visit\w+|Compute\w+|Run)\(/gm)].map((m) => m[1]);
for (const must of ["VisitAllBlocks", "VisitBlock", "VisitBlockBody", "ComputeReadyOperations", "Run"]) {
  if (!visitMethods.includes(must)) throw new Error(`RandomRescheduler 缺 ${must}`);
}

let rust = banner(
  "src/compiler/turboshaft/random-rescheduling-phase.h.js",
  "src/compiler/turboshaft/random-rescheduling-phase.{h,cc}",
  "随机重排: 调试用操作调度重排",
);

rust += `
/// 原版 ReschedulingReducer::OpState — 操作调度状态机
/// v8: src/compiler/turboshaft/random-rescheduling-phase.cc ReschedulingReducer::OpState
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpState {
  /// kPending: 未发出且依赖未满足
  Pending = 0,
  /// kReady: 未发出但依赖已满足
  Ready = 1,
  /// kFixed: 位置固定 (块终结符 / Phi)
  Fixed = 2,
  /// kScheduled: 已发出
  Scheduled = 3,
}

/// 原版 ReschedulingReducer — op_states_ 侧表驱动
/// (GrowingOpIndexSidetable<OpState>, 初值 kPending)
/// v8: src/compiler/turboshaft/random-rescheduling-phase.cc ReschedulingReducer
pub struct ReschedulingReducer {
  pub op_states: Vec<OpState>,
}

/// 原版 RandomRescheduler — Assembler<ReschedulingReducer, GraphVisitor>
/// 方法面:
${visitMethods.map((m) => `/// - ${m}`).join("\n")}
/// - Run: 输入图 -> companion 图, 随机就绪序发出
/// - ComputeReadyOperations: 扫块内依赖, kPending -> kReady
/// v8: src/compiler/turboshaft/random-rescheduling-phase.cc RandomRescheduler
pub struct RandomRescheduler {
  pub ready_ops: Vec<OpIndex>,
  pub rng: RandomNumberGenerator,
}

/// 原版 RandomReschedulingPhase — 由 v8_flags.turboshaft_random_schedule 控制
/// v8: src/compiler/turboshaft/random-rescheduling-phase.{h,cc} RandomReschedulingPhase
pub struct RandomReschedulingPhase;
`;

const p = writeSrc("v8-turboshaft", "random_rescheduling_phase.rs.skel", rust);
console.log(`written ${p}`);
console.log(`OpState=${opStates.length} 方法=${visitMethods.length}`);
