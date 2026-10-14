#!/usr/bin/env bun
// tool/src/compiler/backend/ppc/instruction-scheduler-ppc.js
// v8/src/compiler/backend/ppc/instruction-scheduler-ppc.cc
//   → v8-compiler-backend/src/instruction_scheduler_ppc.rs.skel
// ppc 指令调度架构面: GetTargetInstructionFlags 383 case
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/ppc/instruction-scheduler-ppc.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("bool InstructionScheduler::SchedulerSupported() { return true; }"), "cc SchedulerSupported=true");
must(cc.includes("int InstructionScheduler::GetTargetInstructionFlags("), "cc GetTargetInstructionFlags");
must(count(cc, /case kPPC/g) === 383, `cc case kPPC 383, 实得 ${count(cc, /case kPPC/g)}`);
must(cc.includes("UNREACHABLE();"), "cc 默认臂");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/ppc/instruction-scheduler-ppc.js",
  "src/compiler/backend/ppc/instruction-scheduler-ppc.cc",
  "ppc 调度架构面: GetTargetInstructionFlags (383 case)")}
//! 台账: SchedulerSupported()=true; GetTargetInstructionFlags case kPPC ×383
//!   → 公共位标志 4 值组合。

/// v8: InstructionScheduler::GetTargetInstructionFlags (ppc)
// pub fn get_target_instruction_flags(instr: &Instruction) -> i32
pub const SCHEDULER_SUPPORTED: bool = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(SCHEDULER_SUPPORTED);
  assert!(TARGET_CASE_COUNT == 383);
};
pub const TARGET_CASE_COUNT: usize = 383;
`;

writeSrc("v8-compiler-backend", "instruction_scheduler_ppc.rs.skel", skel);
console.log(`ppc/instruction-scheduler-ppc.js: 对数 ${checks} 条全过; case kPPC 383 实测; ` +
  `产物 instruction_scheduler_ppc.rs.skel`);
