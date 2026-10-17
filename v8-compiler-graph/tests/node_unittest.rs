//! node-unittest.cc 1:1 移植 + Node API 语义补测
//! (AppendInput/ReplaceInput/RemoveInput/TrimInputCount/InsertInput(s)/
//!  EnsureInputCount/Use 链一致性/内联→外联迁移/zone 复用)

use std::ptr;

use aok::{OK, Void};
use log::info;
use v8_compiler_graph::{IrOpcode, Node, Operator, Operator1, OperatorCounts, Properties};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// node-unittest.cc kOp0/1/2 (opcode=Start/Loop/Branch, value_in=0/1/2, value_out=1)
fn op0() -> Operator {
  Operator::new(
    IrOpcode::Start as u16,
    Properties::NONE,
    "Op0",
    OperatorCounts::new(0, 0, 0, 1, 0, 0),
  )
  .unwrap()
}
fn op1() -> Operator {
  Operator::new(
    IrOpcode::Loop as u16,
    Properties::NONE,
    "Op1",
    OperatorCounts::new(1, 0, 0, 1, 0, 0),
  )
  .unwrap()
}
fn op2() -> Operator {
  Operator::new(
    IrOpcode::Branch as u16,
    Properties::NONE,
    "Op2",
    OperatorCounts::new(2, 0, 0, 1, 0, 0),
  )
  .unwrap()
}

/// 节点 id 集 (UnorderedElementsAre 等价)
fn use_ids(n: &Node) -> Vec<u32> {
  let mut ids: Vec<u32> = n.uses().map(|u| u.id()).collect();
  ids.sort_unstable();
  ids
}

/// 输入 id 序列 (ElementsAre 等价; null 输入记 u32::MAX)
fn input_ids(n: &Node) -> Vec<u32> {
  n.inputs().map(|i| i.map_or(u32::MAX, |x| x.id())).collect()
}

/// NodeTest.New — id/uses/inputs/op/opcode 初值
#[test]
fn node_test_new() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let node = Node::new(&zone, 1, &op0(), &[], false)?;
  assert_eq!(1, node.id());
  assert_eq!(0, node.use_count());
  assert!(node.uses().next().is_none());
  assert_eq!(0, node.input_count());
  assert!(node.inputs().next().is_none());
  assert_eq!("Op0", node.op().mnemonic());
  assert_eq!(IrOpcode::Start, node.opcode());
  info!("new ok");
  OK
}

/// NodeTest.NewWithInputs — 输入建立 + 双向 Use 链
#[test]
fn node_test_new_with_inputs() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  assert_eq!(0, n0.use_count());
  assert_eq!(0, n0.input_count());
  let n1 = Node::new(&zone, 1, &op1(), &[n0], false)?;
  assert_eq!(1, n0.use_count());
  assert_eq!(vec![1], use_ids(n0));
  assert_eq!(0, n1.use_count());
  assert_eq!(1, n1.input_count());
  assert!(ptr::eq(n0, n1.input_at(0).unwrap()));
  let n2 = Node::new(&zone, 2, &op2(), &[n0, n1], false)?;
  assert_eq!(2, n0.use_count());
  assert_eq!(vec![1, 2], use_ids(n0));
  assert_eq!(vec![2], use_ids(n1));
  assert_eq!(2, n2.input_count());
  assert!(ptr::eq(n0, n2.input_at(0).unwrap()));
  assert!(ptr::eq(n1, n2.input_at(1).unwrap()));
  OK
}

/// NodeTest.InputIteratorEmpty / One / Two
#[test]
fn node_test_input_iterators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let e = Node::new(&zone, 0, &op0(), &[], false)?;
  assert_eq!(0, e.inputs().count());
  let n1 = Node::new(&zone, 1, &op1(), &[e], false)?;
  assert_eq!(vec![0], input_ids(n1));
  let n2 = Node::new(&zone, 2, &op2(), &[e, n1], false)?;
  assert_eq!(vec![0, 1], input_ids(n2));
  // ExactSizeIterator 语义 (原版 Inputs::count())
  assert_eq!(2, n2.inputs().len());
  assert_eq!(2, n2.input_edges().len());
  OK
}

/// NodeTest.UseIteratorEmpty / One / Two
#[test]
fn node_test_use_iterators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let e = Node::new(&zone, 0, &op0(), &[], false)?;
  assert_eq!(0, e.uses().count());
  let n1 = Node::new(&zone, 1, &op1(), &[e], false)?;
  assert_eq!(vec![1], use_ids(e));
  let _n2 = Node::new(&zone, 2, &op2(), &[e, n1], false)?;
  assert_eq!(vec![1, 2], use_ids(e));
  // use_edges 与 uses 等长; 边 index/from/to 配对
  assert_eq!(e.use_edges().count(), e.uses().count());
  OK
}

/// NodeTest.OwnedBy — 单/双属主
#[test]
fn node_test_owned_by() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  assert!(!n0.owned_by(n0));
  let n1 = Node::new(&zone, 1, &op1(), &[n0], false)?;
  assert!(!n0.owned_by(n0));
  assert!(!n1.owned_by(n1));
  assert!(n0.owned_by(n1));
  let n2 = Node::new(&zone, 2, &op2(), &[n0, n1], false)?;
  assert!(!n0.owned_by(n0));
  assert!(!n1.owned_by(n1));
  assert!(!n2.owned_by(n2));
  assert!(!n0.owned_by(n1));
  assert!(!n0.owned_by(n2));
  assert!(n1.owned_by(n2));
  assert!(n0.owned_by2(n1, n2));
  n2.replace_input(0, Some(n2));
  assert!(n0.owned_by(n1));
  assert!(n1.owned_by(n2));
  n2.replace_input(1, Some(n0));
  assert!(!n0.owned_by(n1));
  assert!(!n1.owned_by(n2));
  OK
}

/// NodeTest.ReplaceUsesNone — 无使用时替换不引炸
#[test]
fn node_test_replace_uses_none() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op1(), &[n0], false)?;
  let n2 = Node::new(&zone, 2, &op2(), &[n0, n1], false)?;
  let node = Node::new(&zone, 42, &op0(), &[], false)?;
  assert_eq!(0, node.use_count());
  node.replace_uses(n0);
  assert_eq!(0, node.use_count());
  node.replace_uses(n1);
  assert_eq!(0, node.use_count());
  node.replace_uses(n2);
  assert_eq!(0, node.use_count());
  OK
}

/// ReplaceUses 有使用 — 嫁接后链一致 (node.cc:304-321)
#[test]
fn node_test_replace_uses_with_uses() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op1(), &[n0], false)?;
  let n2 = Node::new(&zone, 2, &op2(), &[n0, n1], false)?;
  // n0 被 n1, n2 使用 → 嫁接到 n2
  n0.replace_uses(n2);
  assert_eq!(0, n0.use_count());
  assert_eq!(2, n2.use_count());
  assert_eq!(vec![1, 2], use_ids(n2));
  // n1 的输入 0 与 n2 的输入 0 均已指向 n2
  assert!(ptr::eq(n2, n1.input_at(0).unwrap()));
  assert!(ptr::eq(n2, n2.input_at(0).unwrap()));
  OK
}

/// NodeTest.AppendInput — 可扩展节点尾插 (含内联→外联迁移)
#[test]
fn node_test_append_input() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op1(), &[n0], false)?;
  let node = Node::new(&zone, 12345, &op0(), &[], true)?;
  assert_eq!(0, node.input_count());
  node.append_input(&zone, n0)?;
  assert_eq!(vec![0], input_ids(node));
  node.append_input(&zone, n1)?;
  assert_eq!(vec![0, 1], input_ids(node));
  node.append_input(&zone, n0)?;
  assert_eq!(vec![0, 1, 0], input_ids(node));
  // extensible 内联容量 = min(0+3, 14) = 3 → 第 4 次触发内联→外联迁移
  node.append_input(&zone, n0)?;
  assert_eq!(vec![0, 1, 0, 0], input_ids(node));
  node.append_input(&zone, n1)?;
  assert_eq!(vec![0, 1, 0, 0, 1], input_ids(node));
  // 迁移后 Use 链保持: n0 有 4 个使用, n1 有 2 个
  assert_eq!(4, n0.use_count());
  assert_eq!(2, n1.use_count());
  OK
}

/// NodeTest.TrimThenAppend — 裁剪后再生长 (原版 10 节点全序)
#[test]
fn node_test_trim_then_append() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let ids: Vec<_> = (0..10)
    .map(|i| Node::new(&zone, i, &op0(), &[], false))
    .collect::<Result<_, _>>()?;
  let [n0, n1, n2, n3, n4, n5, n6, n7, n8, n9] = ids.as_slice() else {
    unreachable!()
  };
  let node = Node::new(&zone, 12345, &op0(), &[], true)?;
  assert_eq!(0, node.input_count());

  node.append_input(&zone, n0)?;
  assert_eq!(vec![0], input_ids(node));
  node.trim_input_count(0);
  assert_eq!(0, node.input_count());
  // 裁剪后 n0 无使用
  assert_eq!(0, n0.use_count());

  node.append_input(&zone, n1)?;
  assert_eq!(vec![1], input_ids(node));
  node.append_input(&zone, n2)?;
  assert_eq!(vec![1, 2], input_ids(node));
  node.trim_input_count(1);
  assert_eq!(vec![1], input_ids(node));

  node.append_input(&zone, n3)?;
  assert_eq!(vec![1, 3], input_ids(node));
  node.append_input(&zone, n4)?;
  assert_eq!(vec![1, 3, 4], input_ids(node));
  node.append_input(&zone, n5)?;
  assert_eq!(vec![1, 3, 4, 5], input_ids(node));
  // 容量 3 用尽 → 迁移外联后继续
  node.append_input(&zone, n6)?;
  assert_eq!(vec![1, 3, 4, 5, 6], input_ids(node));
  node.append_input(&zone, n7)?;
  assert_eq!(vec![1, 3, 4, 5, 6, 7], input_ids(node));
  node.trim_input_count(4);
  assert_eq!(vec![1, 3, 4, 5], input_ids(node));
  node.append_input(&zone, n8)?;
  assert_eq!(vec![1, 3, 4, 5, 8], input_ids(node));
  node.append_input(&zone, n9)?;
  assert_eq!(vec![1, 3, 4, 5, 8, 9], input_ids(node));
  OK
}

/// NodeTest.BigNodes — 13..=512 步进 9 的大输入 (外联布局)
#[test]
fn node_test_big_nodes() -> Void {
  const MAX: usize = 512;
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op1(), &[n0], false)?;
  let mut inputs = Vec::with_capacity(MAX);
  for i in 0..MAX {
    inputs.push(if i & 1 == 1 { n1 } else { n0 });
  }
  let mut size = 13;
  while size <= MAX {
    let slice = &inputs[..size];
    let node = Node::new(&zone, 12345, &op0(), slice, false)?;
    assert_eq!(size, node.input_count());
    for (i, expect) in slice.iter().enumerate() {
      assert!(
        ptr::eq(*expect, node.input_at(i).unwrap()),
        "size={size} i={i}"
      );
    }
    // uses 包含 node
    let used0 = n0.uses().any(|u| ptr::eq(u, node));
    let used1 = n1.uses().any(|u| ptr::eq(u, node));
    assert!(used0 && used1, "size={size}");
    assert_eq!(
      slice.iter().filter(|n| ptr::eq(**n, n0)).count(),
      n0.uses().filter(|u| ptr::eq(*u, node)).count()
    );
    size += 9;
  }
  OK
}

/// API 补测: ReplaceInput 迁移 Use (node.cc:76-87)
#[test]
fn node_test_replace_input() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op0(), &[], false)?;
  let n2 = Node::new(&zone, 2, &op2(), &[n0, n1], false)?;
  assert_eq!(1, n0.use_count());
  assert_eq!(1, n1.use_count());
  n2.replace_input(0, Some(n1));
  assert_eq!(0, n0.use_count());
  assert_eq!(2, n1.use_count());
  assert_eq!(vec![1, 1], input_ids(n2));
  // 同值替换不动链
  n2.replace_input(0, Some(n1));
  assert_eq!(2, n1.use_count());
  // null 替换 (原版 UpdateTo(nullptr) 路径)
  n2.replace_input(0, None);
  assert_eq!(1, n1.use_count());
  assert!(n2.input_at(0).is_none());
  OK
}

/// API 补测: RemoveInput 滚动前移并返回被移除输入 (node.cc:230-240)
#[test]
fn node_test_remove_input() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op0(), &[], false)?;
  let n2 = Node::new(&zone, 2, &op0(), &[], false)?;
  let node = Node::new(&zone, 3, &op2(), &[n0, n1, n2], false)?;
  let removed = node.remove_input(1);
  assert!(removed.is_some_and(|r| ptr::eq(r, n1)));
  assert_eq!(vec![0, 2], input_ids(node));
  assert_eq!(0, n1.use_count());
  assert_eq!(1, n0.use_count());
  assert_eq!(1, n2.use_count());
  OK
}

/// API 补测: InsertInput 中位插入 (node.cc:201-211)
#[test]
fn node_test_insert_input() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op0(), &[], false)?;
  let n2 = Node::new(&zone, 2, &op0(), &[], false)?;
  let node = Node::new(&zone, 3, &op2(), &[n0, n1], false)?;
  node.insert_input(&zone, 1, n2)?;
  assert_eq!(vec![0, 2, 1], input_ids(node));
  // Use 链同步
  assert_eq!(1, n0.use_count());
  assert_eq!(1, n1.use_count());
  assert_eq!(1, n2.use_count());
  OK
}

/// API 补测: InsertInputs 插 count 个槽 (node.cc:213-228; 空档可为 null)
#[test]
fn node_test_insert_inputs() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op0(), &[], false)?;
  let node = Node::new(&zone, 3, &op2(), &[n0, n1], false)?;
  node.insert_inputs(&zone, 1, 2)?;
  assert_eq!(4, node.input_count());
  // 原 [n0, n1] → [n0, null, null, n1]
  assert!(node.input_at(0).is_some_and(|n| ptr::eq(n, n0)));
  assert!(node.input_at(1).is_none());
  assert!(node.input_at(2).is_none());
  assert!(node.input_at(3).is_some_and(|n| ptr::eq(n, n1)));
  OK
}

/// API 补测: EnsureInputCount 裁/补两向 (node.cc:272-284)
#[test]
fn node_test_ensure_input_count() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op0(), &[], false)?;
  let node = Node::new(&zone, 2, &op2(), &[n0, n1], false)?;
  node.ensure_input_count(&zone, 5)?;
  assert_eq!(vec![0, 1, 1, 1, 1], input_ids(node));
  assert_eq!(4, n1.use_count()); // 原 1 + 补 3
  node.ensure_input_count(&zone, 2)?;
  assert_eq!(vec![0, 1], input_ids(node));
  assert_eq!(1, n1.use_count()); // 裁回原 1
  OK
}

/// API 补测: Kill / IsDead / NullAllInputs (node.cc:145-149, 461-464)
#[test]
fn node_test_kill_and_is_dead() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let node = Node::new(&zone, 1, &op1(), &[n0], false)?;
  assert!(!node.is_dead());
  node.kill();
  assert!(node.is_dead());
  // NullAllInputs 只清槽不裁 count (原版语义: is_dead 由首槽 null 判定)
  assert_eq!(1, node.input_count());
  assert!(node.input_at(0).is_none());
  assert_eq!(0, n0.use_count());
  OK
}

/// API 补测: BranchUseCount (node.cc:294-302)
#[test]
fn node_test_branch_use_count() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let branch = Operator::new(
    IrOpcode::Branch as u16,
    Properties::NONE,
    "Branch",
    OperatorCounts::new(1, 0, 1, 1, 0, 2),
  )?;
  let b1 = Node::new(&zone, 1, &branch, &[n0, n0], false)?;
  let u = Node::new(&zone, 2, &op1(), &[n0], false)?;
  assert_eq!(2, n0.branch_use_count());
  assert_eq!(0, u.branch_use_count());
  assert_eq!(b1.opcode(), IrOpcode::Branch);
  OK
}

/// API 补测: Clone 同输入新 id (node.cc:134-142)
#[test]
fn node_test_clone() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op1(), &[n0], false)?;
  let clone = Node::clone_node(&zone, 77, n1)?;
  assert_eq!(77, clone.id());
  assert_eq!(vec![0], input_ids(clone));
  assert_eq!(2, n0.use_count()); // n1 + clone
  OK
}

/// API 补测: zone 复用语义 — 同 zone 连续建节点, 先建节点仍可用 (bump 分配不回收)
#[test]
fn node_test_zone_reuse() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let nodes: Vec<_> = (0..100)
    .map(|i| Node::new(&zone, i, &op0(), &[], false))
    .collect::<Result<_, _>>()?;
  for (i, n) in nodes.iter().enumerate() {
    assert_eq!(i as u32, n.id());
  }
  // 旧节点追加输入仍作用新节点 Use 链
  nodes[0].append_input(&zone, nodes[50])?;
  assert_eq!(1, nodes[50].use_count());
  assert_eq!(1, nodes[0].input_count());
  OK
}

/// API 补测: Display 格式 (原版 operator<<: "{id}: {mnemonic}({input-ids})")
#[test]
fn node_test_display() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n0 = Node::new(&zone, 0, &op0(), &[], false)?;
  let n1 = Node::new(&zone, 1, &op1(), &[n0], false)?;
  assert_eq!("0: Op0", format!("{n0}"));
  assert_eq!("1: Op1(0)", format!("{n1}"));
  OK
}

/// API 补测: Operator1<T> 参数面 (operator.h:179-235)
#[test]
fn node_test_operator1_parameter() -> Void {
  let a = Operator1::new(
    IrOpcode::NumberConstant as u16,
    Properties::NONE,
    "Num",
    OperatorCounts::new(0, 0, 0, 1, 0, 0),
    42_u32,
  )?;
  let b = Operator1::new(
    IrOpcode::NumberConstant as u16,
    Properties::NONE,
    "Num",
    OperatorCounts::new(0, 0, 0, 1, 0, 0),
    42_u32,
  )?;
  let c = Operator1::new(
    IrOpcode::NumberConstant as u16,
    Properties::NONE,
    "Num",
    OperatorCounts::new(0, 0, 0, 1, 0, 0),
    43_u32,
  )?;
  assert_eq!(42, *a.parameter());
  assert!(a.equals(&b));
  assert!(!a.equals(&c)); // 参数不同
  assert_eq!(a.hash_code(), b.hash_code());
  // 节点持有 Operator1.base
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n = Node::new(&zone, 0, &a.base, &[], false)?;
  assert_eq!(IrOpcode::NumberConstant, n.opcode());
  OK
}

/// API 补测: Mark 读写 (单调递增域; NodeMarker 层接管)
#[test]
fn node_test_mark() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let n = Node::new(&zone, 0, &op0(), &[], false)?;
  assert_eq!(0, n.mark());
  n.set_mark(7);
  assert_eq!(7, n.mark());
  OK
}
