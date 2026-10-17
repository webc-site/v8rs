#!/usr/bin/env bun
// tool/src/compiler/backend/riscv/instruction-scheduler-riscv.js
// v8/src/compiler/backend/riscv/instruction-scheduler-riscv.cc
//   → v8-compiler-backend/src/instruction_scheduler_riscv.rs.skel
// riscv 指令调度架构面: GetTargetInstructionFlags 795 case (RVV 大表)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/riscv/instruction-scheduler-riscv.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("bool InstructionScheduler::SchedulerSupported() { return true; }"), "cc SchedulerSupported=true");
must(cc.includes("int InstructionScheduler::GetTargetInstructionFlags("), "cc GetTargetInstructionFlags");
must(count(cc, /case kRiscv/g) === 795, `cc case kRiscv 795, 实得 ${count(cc, /case kRiscv/g)}`);
must(cc.includes("UNREACHABLE();"), "cc 默认臂");
// RVV 段在场
must(count(cc, /case kRiscvRvv/g) === 4, `cc RVV case 4, 实得 ${count(cc, /case kRiscvRvv/g)}`);

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/riscv/instruction-scheduler-riscv.js",
  "src/compiler/backend/riscv/instruction-scheduler-riscv.cc",
  "riscv 调度架构面: GetTargetInstructionFlags (795 case, 含 RVV 段)")}
//! 台账: SchedulerSupported()=true; GetTargetInstructionFlags case kRiscv ×795
//!   (RVV 向量段大表); 公共位标志 4 值组合。

/// v8: InstructionScheduler::GetTargetInstructionFlags (riscv)
// pub fn get_target_instruction_flags(instr: &Instruction) -> i32
pub const SCHEDULER_SUPPORTED: bool = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(SCHEDULER_SUPPORTED);
  assert!(TARGET_CASE_COUNT == 795);
};
pub const TARGET_CASE_COUNT: usize = 795;
`;

writeSrc("v8-compiler-backend", "instruction_scheduler_riscv.rs.skel", skel);
console.log(`riscv/instruction-scheduler-riscv.js: 对数 ${checks} 条全过; case kRiscv 795 实测 ` +
  `(RVV ${count(cc, /case kRiscvRvv/g)}); 产物 instruction_scheduler_riscv.rs.skel`);
