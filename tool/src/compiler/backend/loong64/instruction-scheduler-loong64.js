#!/usr/bin/env bun
// tool/src/compiler/backend/loong64/instruction-scheduler-loong64.js
// v8/src/compiler/backend/loong64/instruction-scheduler-loong64.cc
//   → v8-compiler-backend/src/instruction_scheduler_loong64.rs.skel
// loong64 调度架构面: 不支持调度 (全 UNREACHABLE 桩)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/loong64/instruction-scheduler-loong64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("bool InstructionScheduler::SchedulerSupported() { return false; }"),
  "cc SchedulerSupported=false (LOONG_dev TODO)");
must(count(cc, /UNREACHABLE\(\);/g) === 4, `cc UNREACHABLE 桩 4, 实得 ${count(cc, /UNREACHABLE\(\);/g)}`);
for (const m of ["ResourceAllocation InstructionScheduler::GetResourceTable()",
  "ArchInstResource InstructionScheduler::GetInstructionResource(",
  "int InstructionScheduler::GetTargetInstructionFlags(",
  "int InstructionScheduler::GetInstructionLatency(const Instruction* instr)"])
  must(cc.includes(m), `cc 桩 ${m.slice(0, 55)}`);
must(cc.includes("TODO(LOONG_dev)"), "cc 支持注释锚");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/loong64/instruction-scheduler-loong64.js",
  "src/compiler/backend/loong64/instruction-scheduler-loong64.cc",
  "loong64 调度架构面: 不支持调度 — 4 桩 UNREACHABLE")}
//! 台账: SchedulerSupported()=false; GetResourceTable/GetInstructionResource/
//!   GetTargetInstructionFlags/GetInstructionLatency 四桩全 UNREACHABLE。

/// v8: SchedulerSupported (loong64) — false
pub const SCHEDULER_SUPPORTED: bool = false;

// —— 提取不变式断言 ——
const _: () = {
  assert!(!SCHEDULER_SUPPORTED);
};
`;

writeSrc("v8-compiler-backend", "instruction_scheduler_loong64.rs.skel", skel);
console.log(`loong64/instruction-scheduler-loong64.js: 对数 ${checks} 条全过; 不支持调度; ` +
  `4 桩 UNREACHABLE; 产物 instruction_scheduler_loong64.rs.skel`);
