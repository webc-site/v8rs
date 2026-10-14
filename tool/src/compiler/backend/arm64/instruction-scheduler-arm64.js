#!/usr/bin/env bun
// tool/src/compiler/backend/arm64/instruction-scheduler-arm64.js
// v8/src/compiler/backend/arm64/instruction-scheduler-arm64.cc
//   → v8-compiler-backend/src/instruction_scheduler_arm64.rs.skel
// arm64 指令调度架构面: GetTargetInstructionFlags 1034 case
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/arm64/instruction-scheduler-arm64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("int InstructionScheduler::GetTargetInstructionFlags("), "cc GetTargetInstructionFlags");
must(count(cc, /case kArm64/g) === 1034, `cc case kArm64 1034, 实得 ${count(cc, /case kArm64/g)}`);
must(cc.includes("bool InstructionScheduler::SchedulerSupported() {"), "cc SchedulerSupported");
must(cc.includes("return true;"), "cc 支持返回 true");
must(cc.includes("UNREACHABLE();"), "cc 默认臂");
// 首锚
must(cc.includes("case kArm64Float32Cmp:"), "cc 首段 case kArm64Float32Cmp");
// 资源延迟
must(cc.includes("kHasSideEffect") || cc.includes("kIsLoadOperation"), "cc 位标志组合");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/arm64/instruction-scheduler-arm64.js",
  "src/compiler/backend/arm64/instruction-scheduler-arm64.cc",
  "arm64 调度架构面: GetTargetInstructionFlags (1034 case) + SchedulerSupported")}
//! 台账: SchedulerSupported()=true; GetTargetInstructionFlags case kArm64 ×1034
//!   → 位标志组合 (公共 4 值, 见 instruction_scheduler 台账); 默认 UNREACHABLE。

/// v8: InstructionScheduler::GetTargetInstructionFlags (arm64)
// pub fn get_target_instruction_flags(instr: &Instruction) -> i32
/// v8: InstructionScheduler::SchedulerSupported (arm64) — true
pub const SCHEDULER_SUPPORTED: bool = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(SCHEDULER_SUPPORTED);
  assert!(TARGET_CASE_COUNT == 1034);
};
/// v8: GetTargetInstructionFlags case kArm64 分发数 (实测)
pub const TARGET_CASE_COUNT: usize = 1034;
`;

writeSrc("v8-compiler-backend", "instruction_scheduler_arm64.rs.skel", skel);
console.log(`arm64/instruction-scheduler-arm64.js: 对数 ${checks} 条全过; case kArm64 1034 实测; ` +
  `SchedulerSupported=true; 产物 instruction_scheduler_arm64.rs.skel`);
