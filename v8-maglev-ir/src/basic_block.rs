// 由 tool/src/maglev/maglev-basic-block.js 从 v8/src/maglev/maglev-basic-block.h 生成; 修订就地改, 复跑覆盖
// BasicBlock 类型枚举 3 变体 + 旗位 3; BasicBlockPredecessorInfo=本快照不存在(已登记)

//! Maglev 基本块的零依赖数据面: 块类型枚举 + 位旗。
//! 原版 `class BasicBlock` 本体依赖 Graph/FrameState/MaglevAssembler, 属后续波次;
//! 见 readme/zh.md skip 登记。

/// 原版匿名 `enum : uint8_t { kMerge, kEdgeSplit, kOther } type_`。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BasicBlockType {
  Merge = 0,
  EdgeSplit = 1,
  Other = 2,
}

/// 原版 `type_` 存储宽度 (uint8_t)。
pub const BASIC_BLOCK_TYPE_BITS: u32 = 8;

/// 原版 `bool xxx_ : 1` 旗位序号 (声明序即位序)。
pub const BLOCK_FLAG_COUNT: usize = 3;
/// 原版 `bool deferred_ : 1` 的旗位序号。
pub const BLOCK_FLAG_DEFERRED: usize = 0;
/// 原版 `bool is_start_block_of_switch_case_ : 1` 的旗位序号。
pub const BLOCK_FLAG_IS_START_BLOCK_OF_SWITCH_CASE: usize = 1;
/// 原版 `bool is_dead_ : 1` 的旗位序号。
pub const BLOCK_FLAG_IS_DEAD: usize = 2;

/// 原版三个 1 位旗的位名 (顺序即声明序)。
pub const BLOCK_FLAG_NAMES: [&str; BLOCK_FLAG_COUNT] =
  ["deferred_", "is_start_block_of_switch_case_", "is_dead_"];

// 对数: 枚举 3 (变体名逐个核) + 旗位 3 (原文整行锚 + 声明序锚) +
//   本头 enum 总数 1 反证锚 = 全过; BasicBlockPredecessorInfo 缺失已登台账。
