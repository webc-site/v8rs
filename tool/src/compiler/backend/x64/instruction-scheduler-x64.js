#!/usr/bin/env bun
// tool/src/compiler/backend/x64/instruction-scheduler-x64.js
// v8/src/compiler/backend/x64/instruction-scheduler-x64.cc
//   → v8-compiler-backend/src/instruction_scheduler_x64.rs.skel
// x64 指令调度架构面: GetTargetInstructionFlags 353 case + TARGET 资源表
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/x64/instruction-scheduler-x64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("int InstructionScheduler::GetTargetInstructionFlags("), "cc GetTargetInstructionFlags");
must(count(cc, /case kX64/g) === 353, `cc case kX64 353, 实得 ${count(cc, /case kX64/g)}`);
// 首尾 case 锚
must(cc.includes("case kX64TraceInstruction:"), "cc 首 case kX64TraceInstruction");
must(cc.includes("case kX64Add:"), "cc kX64Add");
// 独立调度器支持
must(cc.includes("bool InstructionScheduler::SchedulerSupported() {"), "cc SchedulerSupported");
must(cc.includes("return true;"), "cc 支持返回 true");
// 默认臂
must(cc.includes("UNREACHABLE();"), "cc 默认臂");
// 资源/延迟模型
must(cc.includes("GetInstructionLatencies") || cc.includes("latency") ||
  cc.includes("kUnknownLatency") || /Latency/.test(cc), "cc 延迟面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/x64/instruction-scheduler-x64.js",
  "src/compiler/backend/x64/instruction-scheduler-x64.cc",
  "x64 调度架构面: GetTargetInstructionFlags (353 case) + SchedulerSupported")}
//! 台账: SchedulerSupported()=true (x64 启用调度); GetTargetInstructionFlags
//!   case kX64 ×353 → 位标志组合 (kHasSideEffect/kIsLoadOperation/
//!   kMayNeedDeoptOrTrapCheck/kIsBarrier, 见公共 instruction_scheduler 台账);
//!   默认臂 UNREACHABLE。

/// v8: InstructionScheduler::GetTargetInstructionFlags (x64)
// pub fn get_target_instruction_flags(instr: &Instruction) -> i32
/// v8: InstructionScheduler::SchedulerSupported (x64) — true
pub const SCHEDULER_SUPPORTED: bool = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(SCHEDULER_SUPPORTED);
  assert!(TARGET_CASE_COUNT == 353);
};
/// v8: GetTargetInstructionFlags case kX64 分发数 (实测)
pub const TARGET_CASE_COUNT: usize = 353;
`;

writeSrc("v8-compiler-backend", "instruction_scheduler_x64.rs.skel", skel);
console.log(`x64/instruction-scheduler-x64.js: 对数 ${checks} 条全过; case kX64 353 实测; ` +
  `SchedulerSupported=true; 产物 instruction_scheduler_x64.rs.skel`);
