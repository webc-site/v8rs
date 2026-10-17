//! opcodes 区间谓词数据面: 各分组 FIRST/LAST 边界取首末值验证。
//! 边界名与推导值独立抄自 v8/src/maglev/maglev-ir.h 各 LIST 宏:
//!   CONSTANT_VALUE_NODE_LIST :193-203, CONVERSION_NODE_LIST :230-272 (末 V(UnsafeSmiUntag) :272),
//!   VALUE_NODE_LIST :274-423 (V(Identity) :275 起, 尾接 TURBOLEV_VALUE_NODE_LIST 末 V(ProcessWasmArgument) :220),
//!   GAP_MOVE_NODE_LIST :425-427, NODE_LIST :508-510 (NON_VALUE 首 V(AssertInt32) :430),
//!   BRANCH_CONTROL_NODE_LIST :512-529 (16 项, IF_UD 项在目标配置剔除),
//!   CONDITIONAL :531-533, UNCONDITIONAL :535-538, TERMINAL :540-544, CONTROL :546-550,
//!   kOpcodeCount/kFirstOpcode/kLastOpcode :556-566。

use aok::{OK, Void};
use log::info;
use v8_maglev_ir::*;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 首末值入谓词为真, 越界一格为真为假; 并全表遍历谓词 ⟺ 闭区间 (原版 std::min/max 展开)。
fn check_range(pred: fn(Opcode) -> bool, first: Opcode, last: Opcode, group: &str) {
  assert!(pred(first), "{}: 首值 {:?} 应在区间内", group, first);
  assert!(pred(last), "{}: 末值 {:?} 应在区间内", group, last);
  if first.to_u16() > FIRST_OPCODE {
    let below = Opcode::from_u16(first.to_u16() - 1).unwrap();
    assert!(!pred(below), "{}: 首值前驱 {:?} 应在区间外", group, below);
  }
  if last.to_u16() < LAST_OPCODE {
    let above = Opcode::from_u16(last.to_u16() + 1).unwrap();
    assert!(!pred(above), "{}: 末值后继 {:?} 应在区间外", group, above);
  }
  for op in OPCODES {
    let in_range = op.to_u16() >= first.to_u16() && op.to_u16() <= last.to_u16();
    assert_eq!(pred(op), in_range, "{}: {:?} 区间归属不符", group, op);
  }
}

// CONTROL_NODE_LIST = TERMINAL + CONDITIONAL + UNCONDITIONAL (:546-550): 首 Abort, 末 JumpLoop
#[test]
fn control_node_bounds() -> Void {
  assert_eq!(FIRST_CONTROL_NODE_OPCODE, Opcode::Abort);
  assert_eq!(LAST_CONTROL_NODE_OPCODE, Opcode::JumpLoop);
  check_range(is_control_node, Opcode::Abort, Opcode::JumpLoop, "control");
  OK
}

// TERMINAL_CONTROL_NODE_LIST: V(Abort) V(Return) V(Deopt) V(Throw) (:540-544)
#[test]
fn terminal_control_node_bounds() -> Void {
  assert_eq!(FIRST_TERMINAL_CONTROL_NODE_OPCODE, Opcode::Abort);
  assert_eq!(LAST_TERMINAL_CONTROL_NODE_OPCODE, Opcode::Throw);
  check_range(
    is_terminal_control_node,
    Opcode::Abort,
    Opcode::Throw,
    "terminal",
  );
  OK
}

// CONDITIONAL_CONTROL_NODE_LIST = V(Switch) + BRANCH (:531-533)
#[test]
fn conditional_control_node_bounds() -> Void {
  assert_eq!(FIRST_CONDITIONAL_CONTROL_NODE_OPCODE, Opcode::Switch);
  assert_eq!(
    LAST_CONDITIONAL_CONTROL_NODE_OPCODE,
    Opcode::BranchIfTypedArrayBounds
  );
  check_range(
    is_conditional_control_node,
    Opcode::Switch,
    Opcode::BranchIfTypedArrayBounds,
    "conditional",
  );
  OK
}

// BRANCH_CONTROL_NODE_LIST 首 V(BranchIfSmi) 末 V(BranchIfTypedArrayBounds) (:512-529)
#[test]
fn branch_control_node_bounds() -> Void {
  assert_eq!(FIRST_BRANCH_CONTROL_NODE_OPCODE, Opcode::BranchIfSmi);
  assert_eq!(
    LAST_BRANCH_CONTROL_NODE_OPCODE,
    Opcode::BranchIfTypedArrayBounds
  );
  check_range(
    is_branch_control_node,
    Opcode::BranchIfSmi,
    Opcode::BranchIfTypedArrayBounds,
    "branch",
  );
  OK
}

// UNCONDITIONAL_CONTROL_NODE_LIST: V(Jump) V(CheckpointedJump) V(JumpLoop) (:535-538)
#[test]
fn unconditional_control_node_bounds() -> Void {
  assert_eq!(FIRST_UNCONDITIONAL_CONTROL_NODE_OPCODE, Opcode::Jump);
  assert_eq!(LAST_UNCONDITIONAL_CONTROL_NODE_OPCODE, Opcode::JumpLoop);
  check_range(
    is_unconditional_control_node,
    Opcode::Jump,
    Opcode::JumpLoop,
    "unconditional",
  );
  OK
}

// VALUE_NODE_LIST 首 V(Identity) (:275), 末接 TURBOLEV_VALUE_NODE_LIST V(ProcessWasmArgument) (:220/:423)
#[test]
fn value_node_bounds() -> Void {
  assert_eq!(FIRST_VALUE_NODE_OPCODE, Opcode::Identity);
  assert_eq!(LAST_VALUE_NODE_OPCODE, Opcode::ProcessWasmArgument);
  check_range(
    is_value_node,
    Opcode::Identity,
    Opcode::ProcessWasmArgument,
    "value",
  );
  OK
}

// CONSTANT_VALUE_NODE_LIST: V(HeapConstant) … V(TrustedConstant) (:193-203)
#[test]
fn constant_node_bounds() -> Void {
  assert_eq!(FIRST_CONSTANT_NODE_OPCODE, Opcode::HeapConstant);
  assert_eq!(LAST_CONSTANT_NODE_OPCODE, Opcode::TrustedConstant);
  check_range(
    is_constant_node,
    Opcode::HeapConstant,
    Opcode::TrustedConstant,
    "constant",
  );
  OK
}

// CONVERSION_NODE_LIST 首 V(ChangeInt32ToFloat64) 末 V(UnsafeSmiUntag) (:230/:272)
#[test]
fn conversion_node_bounds() -> Void {
  assert_eq!(FIRST_CONVERSION_NODE_OPCODE, Opcode::ChangeInt32ToFloat64);
  assert_eq!(LAST_CONVERSION_NODE_OPCODE, Opcode::UnsafeSmiUntag);
  check_range(
    is_conversion_node,
    Opcode::ChangeInt32ToFloat64,
    Opcode::UnsafeSmiUntag,
    "conversion",
  );
  // 原版列表序: 转换段恰接常量段之后 (230 紧跟 193-203 块)
  assert_eq!(
    FIRST_CONVERSION_NODE_OPCODE.to_u16(),
    LAST_CONSTANT_NODE_OPCODE.to_u16() + 1
  );
  OK
}

// GAP_MOVE_NODE_LIST: V(ConstantGapMove) V(GapMove) (:425-427)
#[test]
fn gap_move_node_bounds() -> Void {
  assert_eq!(FIRST_GAP_MOVE_NODE_OPCODE, Opcode::ConstantGapMove);
  assert_eq!(LAST_GAP_MOVE_NODE_OPCODE, Opcode::GapMove);
  check_range(
    is_gap_move_node,
    Opcode::ConstantGapMove,
    Opcode::GapMove,
    "gap_move",
  );
  OK
}

// NODE_LIST = NON_VALUE (首 V(AssertInt32) :430) + VALUE (末 V(ProcessWasmArgument)) (:508-510)
#[test]
fn node_bounds() -> Void {
  assert_eq!(FIRST_NODE_OPCODE, Opcode::AssertInt32);
  assert_eq!(LAST_NODE_OPCODE, Opcode::ProcessWasmArgument);
  check_range(
    is_node,
    Opcode::AssertInt32,
    Opcode::ProcessWasmArgument,
    "node",
  );
  OK
}

// 控制区判别值金标准: kOpcodeCount=378 (:560; :562 系 kLastOpcode), 控制区 24 项 = TERMINAL 4 + CONDITIONAL 17(1+16) + UNCOND 3,
// 占据尾部连续段 354..377 ⇒ NODE/VALUE 末值 = 353; 逐项由 C++ 列表条目数独立清点而得。
#[test]
fn control_region_raw_values() -> Void {
  assert_eq!(OPCODE_COUNT, 378);
  assert_eq!(LAST_OPCODE, 377);
  assert_eq!(FIRST_OPCODE, 0);
  assert_eq!(FIRST_CONTROL_NODE_OPCODE.to_u16(), 354);
  assert_eq!(LAST_TERMINAL_CONTROL_NODE_OPCODE.to_u16(), 357);
  assert_eq!(FIRST_CONDITIONAL_CONTROL_NODE_OPCODE.to_u16(), 358);
  assert_eq!(FIRST_BRANCH_CONTROL_NODE_OPCODE.to_u16(), 359);
  assert_eq!(
    LAST_BRANCH_CONTROL_NODE_OPCODE.to_u16(),
    LAST_CONDITIONAL_CONTROL_NODE_OPCODE.to_u16(),
    "条件控制区 = 分支控制区并 Switch 一项, 末值同为 BranchIfTypedArrayBounds"
  );
  assert_eq!(
    LAST_CONDITIONAL_CONTROL_NODE_OPCODE.to_u16(),
    FIRST_UNCONDITIONAL_CONTROL_NODE_OPCODE.to_u16() - 1,
    "无条件控制区紧跟条件控制区"
  );
  assert_eq!(LAST_CONTROL_NODE_OPCODE.to_u16(), 377);
  assert_eq!(LAST_NODE_OPCODE.to_u16(), 353);
  assert_eq!(LAST_VALUE_NODE_OPCODE.to_u16(), 353);
  // 控制区首末恰与 opcode 表尾 24 项对齐
  assert_eq!(
    FIRST_CONTROL_NODE_OPCODE.to_u16(),
    LAST_NODE_OPCODE.to_u16() + 1,
    "NODE_LIST 与 CONTROL_NODE_LIST 在 NODE_BASE_LIST 中紧邻"
  );
  info!("OK opcodes.rs 十组区间谓词首末边界 + 控制区判别值金标准全过");
  OK
}
