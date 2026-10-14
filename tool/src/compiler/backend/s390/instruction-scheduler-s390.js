#!/usr/bin/env bun
// tool/src/compiler/backend/s390/instruction-scheduler-s390.js
// v8/src/compiler/backend/s390/instruction-scheduler-s390.cc
//   → v8-compiler-backend/src/instruction_scheduler_s390.rs.skel
// s390 指令调度架构面: GetTargetInstructionFlags 390 case
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/s390/instruction-scheduler-s390.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("bool InstructionScheduler::SchedulerSupported() { return true; }"), "cc SchedulerSupported=true");
must(cc.includes("int InstructionScheduler::GetTargetInstructionFlags("), "cc GetTargetInstructionFlags");
must(count(cc, /case kS390/g) === 390, `cc case kS390 390, 实得 ${count(cc, /case kS390/g)}`);
must(cc.includes("UNREACHABLE();"), "cc 默认臂");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/s390/instruction-scheduler-s390.js",
  "src/compiler/backend/s390/instruction-scheduler-s390.cc",
  "s390 调度架构面: GetTargetInstructionFlags (390 case)")}
//! 台账: SchedulerSupported()=true; GetTargetInstructionFlags case kS390 ×390。

/// v8: InstructionScheduler::GetTargetInstructionFlags (s390)
// pub fn get_target_instruction_flags(instr: &Instruction) -> i32
pub const SCHEDULER_SUPPORTED: bool = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(SCHEDULER_SUPPORTED);
  assert!(TARGET_CASE_COUNT == 390);
};
pub const TARGET_CASE_COUNT: usize = 390;
`;

writeSrc("v8-compiler-backend", "instruction_scheduler_s390.rs.skel", skel);
console.log(`s390/instruction-scheduler-s390.js: 对数 ${checks} 条全过; case kS390 390 实测; ` +
  `产物 instruction_scheduler_s390.rs.skel`);
