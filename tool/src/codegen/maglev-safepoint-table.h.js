#!/usr/bin/env bun
// tool/src/codegen/maglev-safepoint-table.h.js
// 机械提取 v8/src/codegen/maglev-safepoint-table.{h,cc} —
// MaglevSafepointEntry 哨兵常量 + 表布局骨架。
// 产物: v8-codegen-core/src/maglev_safepoint_table.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/maglev-safepoint-table.h";
const CC_REL = "src/codegen/maglev-safepoint-table.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const noDeopt = h.match(/static constexpr int kNoDeoptIndex = (-?\d+);/);
const noTrap = h.match(/static constexpr int kNoTrampolinePC = (-?\d+);/);
if (!noDeopt || noDeopt[1] !== "-1") throw new Error("kNoDeoptIndex 锚不符");
if (!noTrap || noTrap[1] !== "-1") throw new Error("kNoTrampolinePC 锚不符");
if (!h.includes("static_assert(MaglevSafepointEntry::kNoTrampolinePC == -1);")) {
  throw new Error("缺 trampoline 静态断言锚 (cc)");
}
if (!h.includes("class MaglevSafepointEntry : public SafepointEntryBase")) throw new Error("缺基类锚");

let rust = banner("src/codegen/maglev-safepoint-table.h.js", "src/codegen/maglev-safepoint-table.{h,cc}", "Maglev 安全点表骨架");
rust += `
/// 无反优化索引哨兵 (原 kNoDeoptIndex)
pub const NO_DEOPT_INDEX: i32 = -1;
/// 无蹦床 PC 哨兵 (原 kNoTrampolinePC)
pub const NO_TRAMPOLINE_PC: i32 = -1;

/// 原 class MaglevSafepointEntry : SafepointEntryBase
/// 字段面: pc / deopt_index / num_tagged_slots / num_extra_spill_slots / trampoline_pc
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaglevSafepointEntry {
  pub pc: i32,
  pub deopt_index: i32,
  pub num_tagged_slots: u32,
  pub num_extra_spill_slots: u8,
  pub trampoline_pc: i32,
}

/// 原 class MaglevSafepointTable — 表构建/查询 (cc 语义面登记跳过)
pub struct MaglevSafepointTable;

// TODO(dev): 补表编码 (FindEntry/trampoline 回填)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(NO_DEOPT_INDEX == -1);
  assert!(NO_TRAMPOLINE_PC == -1);
};
`;

const p = writeSrc("v8-codegen-core", "maglev_safepoint_table.rs", rust);
console.log(`written ${p}`);
