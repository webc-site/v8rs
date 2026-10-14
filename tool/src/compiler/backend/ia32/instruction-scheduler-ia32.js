#!/usr/bin/env bun
// tool/src/compiler/backend/ia32/instruction-scheduler-ia32.js
// v8/src/compiler/backend/ia32/instruction-scheduler-ia32.cc
//   → v8-compiler-backend/src/instruction_scheduler_ia32.rs.skel
// ia32 指令调度架构面: GetTargetInstructionFlags
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/ia32/instruction-scheduler-ia32.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("bool InstructionScheduler::SchedulerSupported() { return true; }"), "cc SchedulerSupported=true");
must(cc.includes("int InstructionScheduler::GetTargetInstructionFlags("), "cc GetTargetInstructionFlags");
must(count(cc, /case kIA32/g) === 326, `cc case kIA32 326, 实得 ${count(cc, /case kIA32/g)}`);
must(cc.includes("UNREACHABLE();"), "cc 默认臂");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/ia32/instruction-scheduler-ia32.js",
  "src/compiler/backend/ia32/instruction-scheduler-ia32.cc",
  "ia32 调度架构面: GetTargetInstructionFlags")}
//! 台账: SchedulerSupported()=true; GetTargetInstructionFlags case kIA32 ×326 →
//!   公共位标志 4 值组合。

/// v8: InstructionScheduler::GetTargetInstructionFlags (ia32)
// pub fn get_target_instruction_flags(instr: &Instruction) -> i32
pub const SCHEDULER_SUPPORTED: bool = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(SCHEDULER_SUPPORTED);
  assert!(TARGET_CASE_COUNT == 326);
};
pub const TARGET_CASE_COUNT: usize = 326;
`;

writeSrc("v8-compiler-backend", "instruction_scheduler_ia32.rs.skel", skel);
console.log(`ia32/instruction-scheduler-ia32.js: 对数 ${checks} 条全过; case kIA32 326 实测; ` +
  `产物 instruction_scheduler_ia32.rs.skel`);
