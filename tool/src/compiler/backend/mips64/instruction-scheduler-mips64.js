#!/usr/bin/env bun
// tool/src/compiler/backend/mips64/instruction-scheduler-mips64.js
// v8/src/compiler/backend/mips64/instruction-scheduler-mips64.cc
//   → v8-compiler-backend/src/instruction_scheduler_mips64.rs.skel
// mips64 指令调度架构面: GetTargetInstructionFlags 543 case
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/mips64/instruction-scheduler-mips64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("bool InstructionScheduler::SchedulerSupported() { return true; }"), "cc SchedulerSupported=true");
must(cc.includes("int InstructionScheduler::GetTargetInstructionFlags("), "cc GetTargetInstructionFlags");
must(count(cc, /case kMips64/g) === 543, `cc case kMips64 543, 实得 ${count(cc, /case kMips64/g)}`);
must(cc.includes("UNREACHABLE();"), "cc 默认臂");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/mips64/instruction-scheduler-mips64.js",
  "src/compiler/backend/mips64/instruction-scheduler-mips64.cc",
  "mips64 调度架构面: GetTargetInstructionFlags (543 case)")}
//! 台账: SchedulerSupported()=true; GetTargetInstructionFlags case kMips64 ×543
//!   → 公共位标志 4 值组合。

/// v8: InstructionScheduler::GetTargetInstructionFlags (mips64)
// pub fn get_target_instruction_flags(instr: &Instruction) -> i32
pub const SCHEDULER_SUPPORTED: bool = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(SCHEDULER_SUPPORTED);
  assert!(TARGET_CASE_COUNT == 543);
};
pub const TARGET_CASE_COUNT: usize = 543;
`;

writeSrc("v8-compiler-backend", "instruction_scheduler_mips64.rs.skel", skel);
console.log(`mips64/instruction-scheduler-mips64.js: 对数 ${checks} 条全过; case kMips64 543 实测; ` +
  `产物 instruction_scheduler_mips64.rs.skel`);
