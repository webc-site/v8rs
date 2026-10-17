#!/usr/bin/env bun
// tool/src/compiler/backend/arm/instruction-scheduler-arm.js
// v8/src/compiler/backend/arm/instruction-scheduler-arm.cc
//   → v8-compiler-backend/src/instruction_scheduler_arm.rs.skel
// arm 指令调度架构面: GetTargetInstructionFlags 355 case
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/arm/instruction-scheduler-arm.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("bool InstructionScheduler::SchedulerSupported() { return true; }"), "cc SchedulerSupported=true");
must(cc.includes("int InstructionScheduler::GetTargetInstructionFlags("), "cc GetTargetInstructionFlags");
must(count(cc, /case kArm/g) === 355, `cc case kArm 355, 实得 ${count(cc, /case kArm/g)}`);
must(cc.includes("UNREACHABLE();"), "cc 默认臂");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/arm/instruction-scheduler-arm.js",
  "src/compiler/backend/arm/instruction-scheduler-arm.cc",
  "arm 调度架构面: GetTargetInstructionFlags (355 case)")}
//! 台账: SchedulerSupported()=true; GetTargetInstructionFlags case kArm ×355
//!   → 公共位标志 4 值组合 (见 instruction_scheduler 台账)。

/// v8: InstructionScheduler::GetTargetInstructionFlags (arm)
// pub fn get_target_instruction_flags(instr: &Instruction) -> i32
/// v8: SchedulerSupported (arm) — true
pub const SCHEDULER_SUPPORTED: bool = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(SCHEDULER_SUPPORTED);
  assert!(TARGET_CASE_COUNT == 355);
};
pub const TARGET_CASE_COUNT: usize = 355;
`;

writeSrc("v8-compiler-backend", "instruction_scheduler_arm.rs.skel", skel);
console.log(`arm/instruction-scheduler-arm.js: 对数 ${checks} 条全过; case kArm 355 实测; ` +
  `产物 instruction_scheduler_arm.rs.skel`);
