// 由 tool/src/codegen/maglev-safepoint-table.h.js 从 v8/src/codegen/maglev-safepoint-table.{h,cc} 生成; 修订就地改, 复跑覆盖
// Maglev 安全点表骨架

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
