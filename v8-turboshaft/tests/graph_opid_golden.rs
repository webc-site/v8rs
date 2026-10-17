//! graph 操作 id 计数与归属判定金标准 — div_ceil/floor 语义鉴别
//! v8: src/compiler/turboshaft/graph.h Graph::op_id_count/op_id_capacity
//!     + BelongsToThisGraph (DEBUG 世代位奇偶, release 语义等价面)

use aok::{OK, Void};
use log::info;
use v8_turboshaft::{Graph, OpIndex, Origin};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// op_id_count 走 div_ceil (W9TSb 由 manual_div_ceil 改 div_ceil)
/// 原版 (size + kSlotsPerId - 1) / kSlotsPerId, 与 Rust div_ceil(size, 2) 等价
#[test]
fn graph_op_id_count_uses_div_ceil() -> Void {
  info!("> Graph::op_id_count div_ceil 金标准");
  let mut g = Graph::new(Origin::PureTurboshaft);
  // 空缓冲: 0 → 0
  g.operations.slot_count = 0;
  assert_eq!(g.op_id_count(), 0);
  // 奇数槽 → ceil
  g.operations.slot_count = 1;
  assert_eq!(g.op_id_count(), 1); // div_ceil(1,2)=1
  g.operations.slot_count = 3;
  assert_eq!(g.op_id_count(), 2); // div_ceil(3,2)=2
  g.operations.slot_count = 5;
  assert_eq!(g.op_id_count(), 3); // div_ceil(5,2)=3
  // 偶数槽 → 精确商
  g.operations.slot_count = 2;
  assert_eq!(g.op_id_count(), 1);
  g.operations.slot_count = 4;
  assert_eq!(g.op_id_count(), 2);
  g.operations.slot_count = 6;
  assert_eq!(g.op_id_count(), 3);
  OK
}

/// op_id_capacity 走 floor (原版 capacity / kSlotsPerId, 不 ceil)
/// 与 op_id_count 鉴别: slot=1 → count=1 但 capacity=0
#[test]
fn graph_op_id_capacity_uses_floor() -> Void {
  info!("> Graph::op_id_capacity floor 金标准");
  let mut g = Graph::new(Origin::PureTurboshaft);
  g.operations.slot_capacity = 0;
  assert_eq!(g.op_id_capacity(), 0);
  // floor 与 div_ceil 分岔点: 奇数容量 → floor 少 1
  g.operations.slot_capacity = 1;
  assert_eq!(g.op_id_capacity(), 0); // floor(1,2)=0
  g.operations.slot_capacity = 3;
  assert_eq!(g.op_id_capacity(), 1); // floor(3,2)=1
  g.operations.slot_capacity = 5;
  assert_eq!(g.op_id_capacity(), 2); // floor(5,2)=2
  // 偶数容量: 与 ceil 相同
  g.operations.slot_capacity = 2;
  assert_eq!(g.op_id_capacity(), 1);
  g.operations.slot_capacity = 4;
  assert_eq!(g.op_id_capacity(), 2);
  // 交叉鉴别: slot_count=slot_capacity=1 → count=1 而 capacity=0 (语义分工)
  g.operations.slot_count = 1;
  g.operations.slot_capacity = 1;
  assert_eq!(g.op_id_count(), 1);
  assert_eq!(g.op_id_capacity(), 0);
  assert_ne!(g.op_id_count(), g.op_id_capacity());
  OK
}

/// belongs_to_this_graph release 语义等价面: valid ∧ ¬external ∧ id < op_id_count
/// 原版 DEBUG 用 idx.generation_mod2() == generation_mod2(); Rust OpIndex 无世代位
/// (见台账推迟), 采用三重合取判定 release 面等价
#[test]
fn graph_belongs_to_this_graph_release_surface() -> Void {
  info!("> Graph::belongs_to_this_graph release 面三重合取");
  let mut g = Graph::new(Origin::PureTurboshaft);
  // 分配 4 槽 → op_id_count = div_ceil(4,2) = 2 (合法 id: 0, 1)
  g.operations.slot_count = 4;
  // OpIndex::from_offset: 每 slot 8 字节, id = offset / 8 / 2
  // offset 0 → id 0; offset 16 → id 1; offset 32 → id 2 (越界)
  let in0 = OpIndex::from_offset(0); // id 0
  let in16 = OpIndex::from_offset(16); // id 1
  let out32 = OpIndex::from_offset(32); // id 2, 越 op_id_count
  let out48 = OpIndex::from_offset(48); // id 3, 越 op_id_count
  let inv = OpIndex::INVALID; // valid=false
  let ext = OpIndex::encode_external_id(0); // raw=1, is_external_id=true
  let ext2 = OpIndex::encode_external_id(7); // raw=57

  assert!(g.belongs_to_this_graph(in0), "id 0 在范围");
  assert!(g.belongs_to_this_graph(in16), "id 1 在范围");
  assert!(
    !g.belongs_to_this_graph(out32),
    "id 2 越 op_id_count=2, 不属于本图"
  );
  assert!(
    !g.belongs_to_this_graph(out48),
    "id 3 越 op_id_count=2, 不属于本图"
  );
  assert!(!g.belongs_to_this_graph(inv), "INVALID valid=false 拒绝");
  assert!(
    !g.belongs_to_this_graph(ext),
    "external id 即使 id=0<op_id_count 也拒绝 (跨图源)"
  );
  assert!(
    !g.belongs_to_this_graph(ext2),
    "外部 id 7 拒绝 (external+越界)"
  );
  OK
}
