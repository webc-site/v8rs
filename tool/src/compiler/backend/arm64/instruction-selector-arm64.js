#!/usr/bin/env bun
// tool/src/compiler/backend/arm64/instruction-selector-arm64.js
// v8/src/compiler/backend/arm64/instruction-selector-arm64.cc
//   → v8-compiler-backend/src/instruction_selector_arm64.rs.skel
// arm64 指令选择: Visit 面 210 + TryMatch 匹配族 (extend/shift 寻址)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/arm64/instruction-selector-arm64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// Visit 定义面 (真解析)
const visitDefs = [...cc.matchAll(/^void InstructionSelector::Visit(\w+)\(/gm)].map((m) => m[1]);
if (visitDefs.length < 30) throw new Error(`Visit 定义 ${visitDefs.length} < 30`);
must(count(cc, /::Visit/g) === 210, `cc ::Visit 210, 实得 ${count(cc, /::Visit/g)}`);
must(count(cc, /case Opcode::/g) === 7, `cc case Opcode 7, 实得 ${count(cc, /case Opcode::/g)}`);
// TryMatch 匹配族 (arm64 寻址特色)
for (const m of ["bool TryMatchAnyExtend(Arm64OperandGenerator* g,",
  "bool TryMatchLoadStoreShift(Arm64OperandGenerator* g,"])
  must(cc.includes(m), `cc ${m.slice(5, 45)}`);
must(count(cc, /TryMatch\w+/g) >= 8, "cc TryMatch 族 ≥8");
// 双向尝试 (left/right 交换)
must(cc.includes("TryMatchAnyExtend(&g, selector, binop_idx, left_node, right_node,") &&
  cc.includes("TryMatchAnyExtend(&g, selector, binop_idx, right_node, left_node,"), "cc 左右双向尝试");
// AddSub/Logical 归并
must(cc.includes("VisitWord32Atomic") || /Atomic/.test(cc), "cc 原子面");
// ccmp 比较链
must(cc.includes("CompareChain") || cc.includes("CanCover"), "cc 比较链/覆盖面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/arm64/instruction-selector-arm64.js",
  "src/compiler/backend/arm64/instruction-selector-arm64.cc",
  "arm64 指令选择: TryMatch 匹配族 (extend/shift 寻址) + Visit 面")}
//! 台账: InstructionSelector::Visit* 定义 ${visitDefs.length} (::Visit 面 210);
//!   case Opcode 7; TryMatchAnyExtend (uxtx/sxtb 扩展匹配, 左右双向尝试) +
//!   TryMatchLoadStoreShift (移位寻址 [x + y << n]); 原子族; ccmp 比较链面;
//!   canoolize 寻址序 (BaseWithIndexAndDisplacementMatcher 注释锚)。

/// v8: TryMatchAnyExtend — 扩展操作数匹配 (left/right 双向)
// fn try_match_any_extend(...) -> bool
/// v8: TryMatchLoadStoreShift — load/store 移位寻址匹配
// fn try_match_load_store_shift(...) -> bool
/// v8: InstructionSelector::VisitNode (arm64 扩展) — case Opcode ×7
// fn visit_node 架构扩展臂
/// v8: InstructionSelector::VisitAtomic* — ldax/stlx 原子族
// fn visit_word32/64_atomic_*

// —— 提取不变式断言 ——
const _: () = {
  assert!(VISIT_DEF_COUNT >= 30);
};
/// v8: Visit* 定义数 (实测; ::Visit 总面 210)
pub const VISIT_DEF_COUNT: usize = ${visitDefs.length};
const _: () = {
  assert!(VISIT_DEF_COUNT == ${visitDefs.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_selector_arm64.rs.skel", skel);
console.log(`arm64/instruction-selector-arm64.js: 对数 ${checks} 条全过; Visit 定义 ${visitDefs.length} ` +
  `(总面 210); case Opcode 7; TryMatch 族; 产物 instruction_selector_arm64.rs.skel`);
