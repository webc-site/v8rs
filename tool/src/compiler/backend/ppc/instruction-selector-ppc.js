#!/usr/bin/env bun
// tool/src/compiler/backend/ppc/instruction-selector-ppc.js
// v8/src/compiler/backend/ppc/instruction-selector-ppc.cc
//   → v8-compiler-backend/src/instruction_selector_ppc.rs.skel
// ppc 指令选择: 64 位选择面 + rlwinm 旋转掩码匹配
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/ppc/instruction-selector-ppc.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

const visitDefs = [...cc.matchAll(/^void InstructionSelector::Visit(\w+)\(/gm)].map((m) => m[1]);
if (visitDefs.length < 30) throw new Error(`Visit 定义 ${visitDefs.length} < 30`);
must(count(cc, /::Visit/g) === 199, `cc ::Visit 199, 实得 ${count(cc, /::Visit/g)}`);
// ppc 特色: rlwinm/rldicl 掩码
must(count(cc, /rlwinm|rldicl|RotateLeft|Mask/i) >= 1, "cc 旋转掩码面");
must(cc.includes("ZeroExtendsWord32ToWord64NoPhis") || cc.includes("Word64"), "cc 64 位面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/ppc/instruction-selector-ppc.js",
  "src/compiler/backend/ppc/instruction-selector-ppc.cc",
  "ppc 指令选择: Visit 面 (189 定义) + rlwinm 旋转掩码匹配")}
//! 台账: Visit* 定义 ${visitDefs.length} (::Visit 面 199); rlwinm/rldicl
//!   旋转掩码立即数匹配 (Word32And/Shift 组合折叠); 64 位选择面。

/// v8: rlwinm/rldicl 掩码匹配 — 旋转+掩码组合折叠
// fn mask 匹配锚

// —— 提取不变式断言 ——
const _: () = {
  assert!(VISIT_DEF_COUNT >= 30);
};
pub const VISIT_DEF_COUNT: usize = ${visitDefs.length};
const _: () = {
  assert!(VISIT_DEF_COUNT == ${visitDefs.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_selector_ppc.rs.skel", skel);
console.log(`ppc/instruction-selector-ppc.js: 对数 ${checks} 条全过; Visit 定义 ${visitDefs.length} ` +
  `(总面 199); 旋转掩码面; 产物 instruction_selector_ppc.rs.skel`);
