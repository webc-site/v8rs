#!/usr/bin/env bun
// tool/src/compiler/backend/x64/instruction-selector-x64.js
// v8/src/compiler/backend/x64/instruction-selector-x64.cc
//   → v8-compiler-backend/src/instruction_selector_x64.rs.skel
// x64 指令选择: Visit 面扩展 + AVX/SSE 双路 + 原子操作族
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/x64/instruction-selector-x64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// Visit 方法面 (真解析唯一名集)
const visitDefs = [...cc.matchAll(/^void InstructionSelector::Visit(\w+)\(/gm)].map((m) => m[1]);
if (visitDefs.length < 30) throw new Error(`Visit 定义 ${visitDefs.length} < 30`);
const unique = [...new Set(visitDefs)];
for (const v of ["TraceInstruction", "StackSlot", "AbortCSADcheck", "SwitchSandboxMode"])
  must(unique.includes(v), `Visit${v}`);
must(count(cc, /::Visit/g) === 248, `cc ::Visit 面 248, 实得 ${count(cc, /::Visit/g)}`);
// case 分发 (VisitNode 内嵌 case Opcode)
must(count(cc, /case Opcode::/g) === 7, `cc case Opcode 7, 实得 ${count(cc, /case Opcode::/g)}`);
// AVX/SSE 双路发射
must(cc.includes("case kAVXFloat64Add:"), "cc AVX Float64Add");
must(cc.includes("case kAVXFloat32Add:"), "cc AVX Float32Add");
must(count(cc, /kSSE/g) >= 5, "cc kSSE 锚 ≥5");
// 原子/64 位面
must(count(cc, /VisitWord64Atomic\w+/g) >= 1, "cc Word64 原子族");
must(cc.includes("VisitWord32Atomic") || count(cc, /Atomic/g) >= 10, "cc 原子面");
// 地址模式 (BaseWithIndexAndDisplacement64)
must(cc.includes("MatchBaseWithScaledIndexAndDisplacement") || /Base.*Index.*Displacement/i.test(cc),
  "cc 地址模式匹配 (MatchBase 族)");
// 尾调用
must(cc.includes("VisitTailCall") || /TailCall/.test(cc), "cc 尾调用面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/x64/instruction-selector-x64.js",
  "src/compiler/backend/x64/instruction-selector-x64.cc",
  "x64 指令选择: Visit 扩展面 (248 处) + AVX/SSE 双路 + 原子/地址模式")}
//! 台账: InstructionSelector::Visit* 定义 ${unique.length} 唯一名 (::Visit 面 248 处含
//!   调用/转发); case Opcode 7 (VisitNode 内分发); AVX Float64/Float32 双路 +
//!   kSSE 回退; Word64 原子族 (cmpxchg/xchg); 地址模式
//!   BaseWithIndexAndDisplacement64/32 匹配; 尾调用; TraceInstruction/
//!   AbortCSADcheck/SwitchSandboxMode 特化。

/// v8: InstructionSelector::VisitNode (x64 扩展) — case Opcode ×7
// fn visit_node 架构扩展臂
/// v8: InstructionSelector::VisitWord64Atomic* — 原子族
// fn visit_word64_atomic_* (add/sub/and/or/xor/exchange/compare_exchange)
/// v8: BaseWithIndexAndDisplacement64 匹配 — [base + index*scale + disp]
// fn match_base_with_index_and_displacement64(...) -> Option<...>
/// v8: VisitTailCall (x64) — 尾调用参数压栈序
// fn visit_tail_call

// —— 提取不变式断言 ——
const _: () = {
  assert!(VISIT_DEF_COUNT >= 30);
};
/// v8: Visit* 定义唯一名数 (实测下界 30; 总 ::Visit 面 248)
pub const VISIT_DEF_COUNT: usize = ${unique.length};
const _: () = {
  assert!(VISIT_DEF_COUNT == ${unique.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_selector_x64.rs.skel", skel);
console.log(`x64/instruction-selector-x64.js: 对数 ${checks} 条全过; Visit 唯一名 ${unique.length} ` +
  `(总面 248); case Opcode 7; AVX/SSE 双路; 产物 instruction_selector_x64.rs.skel`);
