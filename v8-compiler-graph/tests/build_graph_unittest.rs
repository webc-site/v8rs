//! build_graph (bytecode-graph-builder.cc 建图纯面) 单测:
//! InputBuffer 扩容公式 / 容量复用 / CreateGraph Start-End 骨架 / new_phi 输入边与 use 链

use aok::{OK, Void};
use log::info;
use v8_compiler_graph::{
  Error, Graph, InputBuffer, IrOpcode, K_INPUT_BUFFER_SIZE_INCREMENT, Operator, OperatorCounts,
  Properties, create_end, create_start, new_phi,
};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const MNEM: &str = "BgOp";

fn op(opcode: IrOpcode, counts: OperatorCounts) -> Operator {
  Operator::new(opcode as u16, Properties::NONE, MNEM, counts).unwrap()
}

/// EnsureInputBufferSize — 扩容公式 `需求 + 64 + 旧容量`; 够用时零重分配
#[test]
fn input_buffer_ensure_growth() -> Void {
  let alloc = AccountingAllocator::new();
  let zone = Zone::new(&alloc, "bg");
  let mut buf = InputBuffer::new();
  assert_eq!(0, buf.size());
  buf.ensure(&zone, 10)?;
  let cap1 = 10 + K_INPUT_BUFFER_SIZE_INCREMENT;
  assert_eq!(cap1, buf.size());
  buf.ensure(&zone, 80)?;
  let cap2 = 80 + K_INPUT_BUFFER_SIZE_INCREMENT + cap1;
  assert_eq!(cap2, buf.size());
  // 需求 ≤ 容量: 不重分配, 容量保持 (旧缓冲弃用于 zone, 对齐 bump 语义)
  buf.ensure(&zone, 50)?;
  assert_eq!(cap2, buf.size());
  info!("input_buffer_ensure_growth ok");
  OK
}

/// CreateGraph 骨架 — SetStart(NewNode(Start)) / exit_controls 非空 → NewNode(End) + SetEnd
#[test]
fn create_start_end_skeleton() -> Void {
  let alloc = AccountingAllocator::new();
  let zone = Zone::new(&alloc, "bg");
  let mut graph = Graph::new(&zone);
  let start_op = op(IrOpcode::Start, OperatorCounts::new(0, 0, 0, 1, 0, 0));
  let const_op = op(IrOpcode::Parameter, OperatorCounts::new(0, 0, 0, 1, 0, 0));
  let end_op = op(IrOpcode::End, OperatorCounts::new(2, 0, 0, 0, 0, 0));

  let start = create_start(&mut graph, &start_op)?;
  assert_eq!(0, start.id());
  assert_eq!(0, start.input_count());
  assert!(graph.start().is_some());

  let a = graph.new_node(&const_op, &[], false)?;
  let b = graph.new_node(&const_op, &[], false)?;
  // exit_controls 空 → Err (原版 DCHECK_NE(0u, ...) 映射), 且不发号
  assert!(
    matches!(
      create_end(&mut graph, &end_op, &[]),
      Err(Error::CountOutOfRange { .. })
    ),
    "空 exit_controls 必须 Err"
  );
  assert_eq!(3, graph.node_count());

  let end = create_end(&mut graph, &end_op, &[a, b])?;
  assert_eq!(3, end.id());
  assert!(graph.end().is_some());
  assert_eq!(2, end.input_count());
  assert_eq!(a.id(), end.input_at(0).unwrap().id());
  assert_eq!(b.id(), end.input_at(1).unwrap().id());
  // use 链完整: a/b 各被 end 使用一次
  assert_eq!(1, a.use_count());
  assert_eq!(1, b.use_count());
  assert!(a.uses().any(|u| u.id() == end.id()));
  assert!(b.uses().any(|u| u.id() == end.id()));
  info!("create_start_end_skeleton ok");
  OK
}

/// NewPhi/NewEffectPhi 路 — count 份输入 + 尾置 control, use 链与 incomplete 建节点
#[test]
fn new_phi_inputs_and_uses() -> Void {
  let alloc = AccountingAllocator::new();
  let zone = Zone::new(&alloc, "bg");
  let graph = Graph::new(&zone);
  let const_op = op(IrOpcode::Parameter, OperatorCounts::new(0, 0, 0, 1, 0, 0));
  let phi_op = op(IrOpcode::Phi, OperatorCounts::new(0, 0, 1, 1, 0, 0));
  let value = graph.new_node(&const_op, &[], false)?;
  let control = graph.new_node(&const_op, &[], false)?;

  let mut buf = InputBuffer::new();
  let phi = new_phi(&graph, &mut buf, &phi_op, 3, value, control)?;
  assert_eq!(2, phi.id());
  assert_eq!(3, graph.node_count());
  // 输入边: [value ×3, control]
  assert_eq!(4, phi.input_count());
  for i in 0..3 {
    assert_eq!(value.id(), phi.input_at(i).unwrap().id());
  }
  assert_eq!(control.id(), phi.input_at(3).unwrap().id());
  // use 链: value×3 / control×1
  assert_eq!(3, value.use_count());
  assert_eq!(1, control.use_count());
  assert!(value.uses().any(|u| u.id() == phi.id()));
  assert!(control.uses().any(|u| u.id() == phi.id()));
  // 缓冲按公式扩容: 4 + 64 + 0
  assert_eq!(4 + K_INPUT_BUFFER_SIZE_INCREMENT, buf.size());

  // 第二个 phi (count=10 → 11 槽 ≤ 68): 容量复用不重分配, 建图正确
  let phi2 = new_phi(&graph, &mut buf, &phi_op, 10, value, control)?;
  assert_eq!(3, phi2.id());
  assert_eq!(11, phi2.input_count());
  assert_eq!(4 + K_INPUT_BUFFER_SIZE_INCREMENT, buf.size());
  assert_eq!(13, value.use_count());
  assert_eq!(2, control.use_count());
  info!("new_phi_inputs_and_uses ok");
  OK
}
