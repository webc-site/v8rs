#!/usr/bin/env bun
// tool/src/compiler/backend/riscv/register-constraints-riscv.js
// v8/src/compiler/backend/riscv/register-constraints-riscv.h
//   → v8-compiler-backend/src/register_constraints_riscv.rs.skel
// riscv 寄存器约束: RiscvRegisterConstraint 枚举 (RVV 重叠规则) + 位域 3 位
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/riscv/register-constraints-riscv.h");

// ── 对数断言 (真解析枚举) ────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

const m = h.match(/enum class RiscvRegisterConstraint \{([\s\S]*?)\};/);
if (!m) throw new Error("未找到 RiscvRegisterConstraint");
const variants = [...m[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)( = \d+)?/g)]
  .map((x) => ({ name: x[1], val: x[2] ? Number(x[2].slice(3)) : null }));
if (variants.length !== 5) throw new Error(`枚举 ${variants.length} != 5`);
must(variants[0].name === "None" && variants[0].val === 0, "kNone=0 首位");
for (const v of ["NoDestinationSourceOverlap", "RegisterGroup", "NoInput2Overlap", "RegisterGroupNoOverlap"])
  must(variants.some((x) => x.name === v), `成员 ${v}`);
must(h.includes("using RiscvRegisterConstraintField =\n    AccessModeField::Next<RiscvRegisterConstraint, 3>;"),
  "h 位域 Next 3 位");
// static_assert 4 (真断言)
must(count(h, /static_assert\(/g) === 4, `h static_assert 4, 实得 ${count(h, /static_assert\(/g)}`);
for (const sa of ["MiscField::kSize == 9", "LaneSizeField::kSize == 2",
  "VectorLengthField::kSize == 2", "AccessModeField::kSize == 1"])
  must(h.includes(sa), `h static_assert ${sa}`);

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/riscv/register-constraints-riscv.js",
  "src/compiler/backend/riscv/register-constraints-riscv.h",
  "riscv 寄存器约束: RVV 目标/源重叠规则 (5 值, 3 位位域)")}
//! 台账: RiscvRegisterConstraint 5 值 — kNone=0 / kNoDestinationSourceOverlap
//!   (向量操作目标与源禁重叠) / kRegisterGroup (偶奇寄存器组) /
//!   kNoInput2Overlap / kRegisterGroupNoOverlap (组与目标禁重叠;
//!   窄化/宽化可再拆分 TODO); RiscvRegisterConstraintField =
//!   AccessModeField::Next<_, 3> (MiscField 9 位内); static_assert ×4
//!   (Misc 9/LaneSize 2/VectorLength 2/AccessMode 1)。

/// v8: RiscvRegisterConstraint — RVV 重叠约束 (实测 5 值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RiscvRegisterConstraint {
  /// kNone = 0
  None = 0,
  /// kNoDestinationSourceOverlap
  NoDestinationSourceOverlap = 1,
  /// kRegisterGroup
  RegisterGroup = 2,
  /// kNoInput2Overlap
  NoInput2Overlap = 3,
  /// kRegisterGroupNoOverlap
  RegisterGroupNoOverlap = 4,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(RiscvRegisterConstraint::None as u8 == 0);
  assert!(RiscvRegisterConstraint::RegisterGroupNoOverlap as u8 == 4);
};
pub const CONSTRAINT_FIELD_WIDTH: u32 = 3;
const _: () = {
  assert!(CONSTRAINT_FIELD_WIDTH == 3);
};
`;

writeSrc("v8-compiler-backend", "register_constraints_riscv.rs.skel", skel);
console.log(`riscv/register-constraints-riscv.js: 对数 ${checks} 条全过; 枚举 5 值实测 + ` +
  `位域 3 位 + static_assert ×4; 产物 register_constraints_riscv.rs.skel`);
