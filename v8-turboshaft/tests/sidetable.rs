//! sidetable 收编核验 — 增长侧表 NextSize 公式 / Reset 保内存 / SwapData / 定长越界面
//! v8: src/compiler/turboshaft/sidetable.h 语义面 (原版无对应 unittest, 机械面自证)

use aok::{OK, Void};
use log::info;
use v8_turboshaft::{
  FixedOpIndexSidetable, FixedSidetable, GrowingBlockSidetable, GrowingOpIndexSidetable, OpIndex,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 NextSize 增长后, 已写入的低索引必须仍可命中 (增长只 append, 不搬迁)
/// v8: sidetable.h GrowingSidetable::operator[] 增长语义
#[test]
fn sidetable_growing_old_indices_valid_after_growth() -> Void {
  info!("> GrowingSidetable 增长保旧索引");
  let mut table = GrowingBlockSidetable::<u32>::new();
  // index 0 边界: NextSize(0) = 0 + 0/2 + 32 = 32
  *table.get_or_grow(0) = 111;
  assert_eq!(table.get(0), Some(&111), "index 0 落位");
  let len_after_0 = table.table.len();
  // 稀疏低位 (只读探查不触发增长)
  assert_eq!(table.get(5), Some(&0), "增长区未写槽为默认值");
  assert_eq!(table.get(len_after_0 as u32), None, "只读 get 越界不得增长");
  assert_eq!(table.table.len(), len_after_0, "get 为只读面, 长度不变");
  // 高索引触发再增长: NextSize(100) = 100 + 50 + 32 = 182
  *table.get_or_grow(100) = 222;
  assert!(table.table.len() >= 182, "应至少增长到 NextSize(100)=182");
  // 旧低索引在增长后仍有效 (Vec::resize 只追加, 不前移)
  assert_eq!(table.get(0), Some(&111), "增长后 index 0 必须保留");
  assert_eq!(table.get(5), Some(&0), "增长后稀疏默认槽仍为 0");
  assert_eq!(table.get(100), Some(&222));
  OK
}

/// 原版 Reset 后仍可继续按需增长; Reset 只 fill 默认, 不缩容 (内存留给后续 phase)
/// v8: sidetable.h GrowingSidetable::Reset
#[test]
fn sidetable_reset_then_further_growth() -> Void {
  info!("> GrowingSidetable Reset 后继续增长");
  let mut table = GrowingBlockSidetable::<u64>::new();
  *table.get_or_grow(4) = 40;
  *table.get_or_grow(9) = 90;
  let len_before_reset = table.table.len();
  table.reset();
  assert_eq!(table.table.len(), len_before_reset, "Reset 保容量");
  assert_eq!(table.get(4), Some(&0), "Reset 清值");
  assert!(!table.is_empty(), "Reset 后 empty() 仍 false");
  // 再增长到更高索引
  *table.get_or_grow(200) = 7;
  assert!(table.table.len() > len_before_reset, "Reset 后仍可增长");
  assert_eq!(table.get(200), Some(&7));
  assert_eq!(table.get(4), Some(&0), "增长不得复活 Reset 前旧值");
  OK
}

/// 原版 operator[] 越界增长: NextSize(i) = i + i/2 + 32, 且吞并过分配
#[test]
fn sidetable_growing_next_size_growth_formula() -> Void {
  info!("> GrowingSidetable NextSize 增长");
  let mut table = GrowingBlockSidetable::<u32>::new();
  assert!(table.is_empty(), "原版 empty(): 构造后从未含值");
  // id=10 越界 → NextSize(10) = 10 + 5 + 32 = 47, 再吞并 capacity
  *table.get_or_grow(10) = 123;
  assert!(table.table.len() >= 47, "应至少增长到 NextSize(10)=47");
  assert_eq!(table.get(10), Some(&123));
  assert!(!table.is_empty(), "增长后 empty() 应为 false");
  // 已占位不再增长
  let len_before = table.table.len();
  *table.get_or_grow(3) = 7;
  assert_eq!(table.table.len(), len_before);
  assert_eq!(table.get(3), Some(&7));
  OK
}

/// 原版 Reset — fill(T{}) 不缩容, empty() 仍 false (内存留给后续 phase)
#[test]
fn sidetable_reset_keeps_memory() -> Void {
  info!("> GrowingSidetable Reset 保内存");
  let mut table = GrowingBlockSidetable::<u32>::with_size(8, 5);
  assert_eq!(table.get(7), Some(&5));
  table.reset();
  assert_eq!(table.get(7), Some(&0));
  assert_eq!(table.table.len(), 8, "Reset 不得缩容");
  assert!(!table.is_empty(), "Reset 后 empty() 仍 false (原版语义)");
  OK
}

/// 原版 SwapData — 交换底层 table_
#[test]
fn sidetable_swap_data() -> Void {
  info!("> GrowingOpIndexSidetable SwapData");
  let mut a = GrowingOpIndexSidetable::<u32>::new(None);
  let mut b = GrowingOpIndexSidetable::<u32>::new(None);
  *a.get_or_grow(OpIndex::from_offset(16)) = 1; // id = 16/16 = 1
  *b.get_or_grow(OpIndex::from_offset(32)) = 2; // id = 2
  a.swap_data(&mut b);
  assert_eq!(b.base.get(1), Some(&1));
  assert_eq!(a.base.get(2), Some(&2));
  assert_eq!(a.base.get(1), Some(&0), "对侧表增长后空位为默认值");
  OK
}

/// 原版 FixedSidetable — 定长, 越界 DCHECK 面以 None 表达
#[test]
fn sidetable_fixed_bounds() -> Void {
  info!("> FixedSidetable 定长越界面");
  let mut table = FixedSidetable::<u32>::with_init(4, 9);
  assert_eq!(table.get(3), Some(&9));
  assert_eq!(table.get(4), None, "越界 (DCHECK_LT) 面不得命中");
  if let Some(slot) = table.get_mut(0) {
    *slot = 1;
  }
  assert_eq!(table.get(0), Some(&1));
  OK
}

/// 原版 FixedOpIndexSidetable — OpIndex 键 + SwapData
#[test]
fn sidetable_fixed_op_index() -> Void {
  info!("> FixedOpIndexSidetable");
  let mut a = FixedOpIndexSidetable::<u32>::new(3, None);
  let mut b = FixedOpIndexSidetable::<u32>::with_init(3, 8, None);
  if let Some(slot) = a.get_mut(OpIndex::from_offset(16)) {
    *slot = 5;
  }
  assert_eq!(a.get(OpIndex::from_offset(16)), Some(&5));
  assert_eq!(
    a.get(OpIndex::from_offset(64)),
    None,
    "id=4 越定长 3 → None"
  );
  a.swap_data(&mut b);
  assert_eq!(b.get(OpIndex::from_offset(16)), Some(&5));
  assert_eq!(a.get(OpIndex::from_offset(0)), Some(&8));
  OK
}
