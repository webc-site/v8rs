//! Wave 9 W9MFb: 新挂载 Graph 宿主基线数据面回归。
//!
//! 覆盖 maglev-graph.{h,cc} 已就位的 arena/常量 intern/块位/sweep 面,
//! maglev-graph-labeller.h 的节点/块标签注册, maglev-interpreter-frame-state.cc 的 MergeDead 计数核。
//! 原版 builder/processor unittest (`test/unittests/maglev/maglev-graph-{builder,processor}-unittest.cc`)
//! 全族 `TestWithNativeContextAndZone` + `isolate()->AsLocalIsolate()` → Isolate 深水, 登记 readme/zh.md 台账,
//! 不在此强转。

use aok::{OK, Void};
use log::info;
use v8_maglev_frontend::{
  BasicBlockRec, Graph, K_MAX_UINT32, MaglevGraphLabeller, MergePointCounters, NodeRef, Provenance,
};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// v8: maglev-graph.h:59 Graph::New (zone 直给接缝, 与 hamt tests 同源)
fn new_graph<'z>(zone: &'z Zone<'z>) -> Graph<'z> {
  Graph::new(zone)
}

/// v8: maglev-graph.cc:70-73 Add (自增分配 id + 尾插)
#[test]
fn add_allocates_block_id_in_order() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "GraphAdd");
  let mut g = new_graph(&zone);

  // 空图初值
  assert_eq!(g.num_blocks(), 0);
  assert_eq!(g.max_block_id(), 0);

  let b0 = g.add(BasicBlockRec::new());
  let b1 = g.add(BasicBlockRec::new());
  let b2 = g.add(BasicBlockRec::new());

  // v8: max_block_id_ 从 0 起, Add 分配后自增 (cc:70-73)
  assert_eq!(b0.0, 0);
  assert_eq!(b1.0, 1);
  assert_eq!(b2.0, 2);
  assert_eq!(g.max_block_id(), 3);
  assert_eq!(g.num_blocks(), 3);

  // 位置取块
  assert_eq!(g.block(0).id, 0);
  assert_eq!(g.block(1).id, 1);
  info!("graph add ids 0/1/2");
  OK
}

/// v8: maglev-graph.h:125 AddBlocksAt (index 后插列, 全量重编号)
#[test]
fn add_blocks_at_reindexes_ids() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "GraphAddBlocksAt");
  let mut g = new_graph(&zone);

  let a = g.add(BasicBlockRec::new());
  let c = g.add(BasicBlockRec::new());
  assert_eq!(a.0, 0);
  assert_eq!(c.0, 1);

  // 在 index=0 之后插入两新块 (原版语义: 分配新 id 后从 index+1 位置 insert)
  g.add_blocks_at(vec![BasicBlockRec::new(), BasicBlockRec::new()], 0);

  // 现应 4 块, id 分别 0,2,3,1 (尾块保留 id=1)
  assert_eq!(g.num_blocks(), 4);
  assert_eq!(g.block(0).id, 0);
  assert_eq!(g.block(3).id, 1);
  // 中间两块是新分配的 (id>=2)
  assert!(g.block(1).id >= 2);
  assert!(g.block(2).id >= 2);
  assert!(g.block(1).id != g.block(2).id);
  OK
}

/// v8: maglev-graph.cc:49-67 IterateGraphAndSweepDeadBlocks (闭包判 dead, 就地压缩)
#[test]
fn iterate_graph_sweep_compacts_dead_blocks() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "GraphSweep");
  let mut g = new_graph(&zone);

  for _ in 0..5 {
    g.add(BasicBlockRec::new());
  }
  // 标记 0/2/4 号为 dead (index 位)
  let dead_ids = [0u32, 2, 4];
  for i in 0..g.num_blocks() {
    if dead_ids.contains(&(g.block(i).id)) {
      g.block_mut(i).mark_dead();
    }
  }

  // 模板 → 闭包参 (Rust 承接: 判 dead 走标志位)
  g.iterate_graph_and_sweep_dead_blocks(|bb| bb.is_dead());

  // 剩两块 (原 id 1 与 3), 且非 dead
  assert_eq!(g.num_blocks(), 2);
  assert_eq!(g.block(0).id, 1);
  assert_eq!(g.block(1).id, 3);
  assert!(!g.block(0).is_dead());
  assert!(!g.block(1).is_dead());
  OK
}

/// v8: maglev-graph.h:387 stack slots 四元组 set/get (K_MAX_UINT32 初值 + 单写覆盖)
#[test]
fn stack_slots_set_get() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "GraphSlots");
  let mut g = new_graph(&zone);

  // 初值为 K_MAX_UINT32 (未定哨)
  assert_eq!(g.tagged_stack_slots(), K_MAX_UINT32);
  assert_eq!(g.untagged_stack_slots(), K_MAX_UINT32);

  g.set_tagged_stack_slots(16);
  g.set_untagged_stack_slots(3);
  assert_eq!(g.tagged_stack_slots(), 16);
  assert_eq!(g.untagged_stack_slots(), 3);
  OK
}

/// v8: maglev-graph.h:309-338 Get*Constant 族 (intern 去重)
#[test]
fn constant_interning_dedups_by_value() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "GraphConstIntern");
  let mut g = new_graph(&zone);

  let a = g.get_int32_constant(42);
  let b = g.get_int32_constant(42);
  let c = g.get_int32_constant(-1);
  assert_eq!(a, b, "同值 int32 常量应 intern 复用");
  assert_ne!(a, c, "不同值应建新节点");

  let s1 = g.get_smi_constant(100);
  let s2 = g.get_smi_constant(100);
  assert_eq!(s1, s2);

  // 常量节点 opcode 落表 (与 ir Opcode::Int32Constant 一致)
  let rec = g.node(a);
  assert_eq!(
    format!("{:?}", rec.opcode),
    "Int32Constant",
    "常量节点 opcode 名对表"
  );
  OK
}

/// v8: maglev-graph-labeller.h:15-104 MaglevGraphLabeller 节点标签注册/去重
#[test]
fn labeller_register_node_dedups_and_assigns_labels() -> Void {
  let mut lab = MaglevGraphLabeller::new();
  assert_eq!(lab.max_node_id(), 0, "next=1, max=next-1=0");

  let n = NodeRef(7);
  lab.register_node_default(n);
  assert_eq!(lab.node_id(n), Some(1));
  assert_eq!(lab.max_node_id(), 1);

  // 重复注册不改动既有 label (emplace .second)
  lab.register_node_default(n);
  assert_eq!(lab.node_id(n), Some(1));
  assert_eq!(lab.max_node_id(), 1);

  // 新节点分配下一 label
  let m = NodeRef(8);
  lab.register_node_default(m);
  assert_eq!(lab.node_id(m), Some(2));
  assert_eq!(lab.max_node_id(), 2);
  OK
}

/// v8: maglev-graph-labeller.h:49-54 line_number get/set (未注册 → None)
#[test]
fn labeller_line_number_round_trip() -> Void {
  let mut lab = MaglevGraphLabeller::new();
  let n = NodeRef(1);
  lab.register_node_default(n);
  assert_eq!(lab.get_node_line_number(n), Some(-1));
  lab.set_node_line_number(n, 42);
  assert_eq!(lab.get_node_line_number(n), Some(42));
  // 未注册节点 → None
  assert_eq!(lab.get_node_line_number(NodeRef(99)), None);
  OK
}

/// v8: maglev-graph-labeller.cc:14-27 GetBlockLabel (block 首次分配 id, 重复取回原标)
#[test]
fn labeller_block_label_allocates_on_demand() -> Void {
  let mut lab = MaglevGraphLabeller::new();
  let b = v8_maglev_frontend::BlockRef(3);
  let l0 = lab.get_block_label(b);
  let l1 = lab.get_block_label(b);
  assert_eq!(l0, l1, "同块两次调用应返同标");
  // 下一新块应得下一 id
  let c = v8_maglev_frontend::BlockRef(4);
  let l2 = lab.get_block_label(c);
  assert_ne!(l0, l2);
  OK
}

/// v8: maglev-graph-labeller.h:28-35 RegisterNode 携 provenance (unit/bytecode_offset/position)
#[test]
fn labeller_provenance_stored() -> Void {
  let mut lab = MaglevGraphLabeller::new();
  let n = NodeRef(5);
  let p = Provenance::unknown();
  lab.register_node_with_provenance(n, &p);
  let got = lab.get_node_provenance(n);
  assert!(got.is_some(), "已注册必返 Some");
  if let Some(got) = got {
    assert_eq!(got.unit, None);
    assert_eq!(got.position, -1);
  }
  OK
}

/// v8: maglev-interpreter-frame-state.cc:MergeDead 纯计数核 (predecessor 收支)
#[test]
fn merge_point_counters_merge_dead() -> Void {
  let mut c = MergePointCounters {
    predecessor_count: 3,
    predecessors_so_far: 0,
  };
  c.merge_dead(1);
  assert_eq!(c.predecessor_count, 2);
  assert_eq!(c.predecessors_so_far, 1);
  c.merge_dead(2);
  assert_eq!(c.predecessor_count, 0);
  assert_eq!(c.predecessors_so_far, 3);
  OK
}

/// v8: maglev-graph.cc:ClearRecordedFrames (eager/lazy 清空)
#[test]
fn clear_recorded_frames_empties_both() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "GraphClearDeopt");
  let mut g = new_graph(&zone);
  // 空清空也应安全 (原版 cc: 直接 .clear())
  g.clear_recorded_frames();
  assert!(g.eager_deopt_top_frames().is_empty());
  assert!(g.lazy_deopt_top_frames().is_empty());
  OK
}

/// v8: maglev-graph.h:257-258 set_has_recursive_calls / has_recursive_calls
#[test]
fn has_recursive_calls_flag_toggle() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "GraphRecursive");
  let mut g = new_graph(&zone);
  assert!(!g.has_recursive_calls());
  g.set_has_recursive_calls(true);
  assert!(g.has_recursive_calls());
  g.set_has_recursive_calls(false);
  assert!(!g.has_recursive_calls());
  OK
}

/// v8: maglev-graph.h:304-307 context_may_alias
#[test]
fn context_may_alias_checks_scope_hierarchy() -> Void {
  use v8_maglev_frontend::{BlockRef, NO_INPUT};
  use v8_maglev_ir::Opcode;

  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "GraphContextAlias");
  let mut g = new_graph(&zone);

  // NO_INPUT 不能别名
  assert!(!g.context_may_alias(NO_INPUT, Some(1)));

  // 创建两个上下文节点: c0 (id 0), c1 (id 1, input 包含 c0)
  let c0 = g.new_node(Opcode::DeadValue, BlockRef(0), vec![], None);
  let c1 = g.new_node(Opcode::DeadValue, BlockRef(0), vec![c0], None);

  // 自身匹配
  assert!(g.context_may_alias(c0, Some(c0.0)));
  assert!(g.context_may_alias(c1, Some(c1.0)));

  // c1 外层链包含 c0
  assert!(g.context_may_alias(c1, Some(c0.0)));

  // 完全无关的 scope
  assert!(!g.context_may_alias(c0, Some(999)));
  assert!(!g.context_may_alias(c1, Some(999)));

  // 未指定 scope 时保守别名
  assert!(g.context_may_alias(c1, None));

  OK
}
