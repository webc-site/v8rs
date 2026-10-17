//! basic_block 数据面: 3 旗位声明序 + 块类型枚举。
//! 期望值独立抄自 v8/src/maglev/maglev-basic-block.h:
//!   :394 `enum : uint8_t { kMerge, kEdgeSplit, kOther } type_;`
//!   :395 `bool deferred_ : 1` / :396 `bool is_start_block_of_switch_case_ : 1`
//!   / :397 `bool is_dead_ : 1` (声明序即位序 0/1/2)。

use aok::{OK, Void};
use log::info;
use v8_maglev_ir::*;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// 3 旗位: 序号按声明序 0/1/2, 名表与原版成员名逐字对齐, 各占独立 1 位
#[test]
fn block_flags() -> Void {
  assert_eq!(BLOCK_FLAG_COUNT, 3);
  let flags: [usize; 3] = [
    BLOCK_FLAG_DEFERRED,
    BLOCK_FLAG_IS_START_BLOCK_OF_SWITCH_CASE,
    BLOCK_FLAG_IS_DEAD,
  ];
  let mut mask = 0u32;
  for (i, flag) in flags.iter().enumerate() {
    assert_eq!(*flag, i, "旗位 {} 声明序", BLOCK_FLAG_NAMES[i]);
    assert!(flag < &BLOCK_FLAG_COUNT);
    mask |= 1u32 << flag;
  }
  // 两两不交校验: 三旗各自占独立位, 累加掩码恰 3 个置位 (复审 R5M4 L1)
  assert_eq!(mask.count_ones(), BLOCK_FLAG_COUNT as u32);
  // :395-397 原版成员名逐字
  assert_eq!(BLOCK_FLAG_NAMES[0], "deferred_");
  assert_eq!(BLOCK_FLAG_NAMES[1], "is_start_block_of_switch_case_");
  assert_eq!(BLOCK_FLAG_NAMES[2], "is_dead_");
  assert_eq!(BLOCK_FLAG_NAMES.len(), BLOCK_FLAG_COUNT);
  info!("OK basic_block.rs 3 旗位全过");
  OK
}

// 块类型枚举: 匿名 enum 声明序 kMerge=0, kEdgeSplit=1, kOther=2; 存储 uint8_t
#[test]
fn block_types() -> Void {
  assert_eq!(BasicBlockType::Merge as u8, 0);
  assert_eq!(BasicBlockType::EdgeSplit as u8, 1);
  assert_eq!(BasicBlockType::Other as u8, 2);
  // :394 匿名 enum 底类型 uint8_t → 位宽 8
  assert_eq!(BASIC_BLOCK_TYPE_BITS, 8);
  // 三变体判别值两两不同
  assert_ne!(BasicBlockType::Merge, BasicBlockType::EdgeSplit);
  assert_ne!(BasicBlockType::EdgeSplit, BasicBlockType::Other);
  OK
}
