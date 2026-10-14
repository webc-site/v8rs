//! CommonOperatorBuilder 单元测试

use aok::{OK, Void};
use v8_compiler_graph::{FeedbackSource, IrOpcode, MachineRepresentation, Properties};
use v8_compiler_ops::{BranchHint, BranchSemantics, CommonOperatorBuilder, DeoptimizeReason};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_common_static_operators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let common = CommonOperatorBuilder::new(&zone);

  // Dead
  let dead = common.dead();
  assert_eq!(dead.opcode(), IrOpcode::Dead as u16);
  assert!(dead.properties().contains(Properties::FOLDABLE));
  assert_eq!(dead.value_input_count(), 0);
  assert_eq!(dead.effect_input_count(), 0);
  assert_eq!(dead.control_input_count(), 0);
  assert_eq!(dead.value_output_count(), 1);
  assert_eq!(dead.effect_output_count(), 1);
  assert_eq!(dead.control_output_count(), 1);

  // IfTrue / IfFalse
  let if_true = common.if_true();
  assert_eq!(if_true.opcode(), IrOpcode::IfTrue as u16);
  assert!(if_true.properties().contains(Properties::KONTROL));
  assert_eq!(if_true.control_input_count(), 1);
  assert_eq!(if_true.control_output_count(), 1);

  let if_false = common.if_false();
  assert_eq!(if_false.opcode(), IrOpcode::IfFalse as u16);
  assert!(if_false.properties().contains(Properties::KONTROL));
  assert_eq!(if_false.control_input_count(), 1);
  assert_eq!(if_false.control_output_count(), 1);

  // IfSuccess / IfException
  let if_succ = common.if_success();
  assert_eq!(if_succ.opcode(), IrOpcode::IfSuccess as u16);
  let if_ex = common.if_exception();
  assert_eq!(if_ex.opcode(), IrOpcode::IfException as u16);
  assert_eq!(if_ex.effect_output_count(), 1);

  OK
}

#[test]
fn test_common_dynamic_operators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let common = CommonOperatorBuilder::new(&zone);

  // Start
  let start = common.start(3)?;
  assert_eq!(start.opcode(), IrOpcode::Start as u16);
  assert!(start.properties().contains(Properties::FOLDABLE));
  assert!(start.properties().contains(Properties::NO_THROW));
  assert_eq!(start.value_output_count(), 3);
  assert_eq!(start.effect_output_count(), 1);
  assert_eq!(start.control_output_count(), 1);

  // End
  let end = common.end(2)?;
  assert_eq!(end.opcode(), IrOpcode::End as u16);
  assert_eq!(end.control_input_count(), 2);
  assert_eq!(end.control_output_count(), 0);

  // Parameter
  let param = common.parameter(0, Some("this"))?;
  assert_eq!(param.opcode(), IrOpcode::Parameter as u16);
  assert!(param.properties().contains(Properties::PURE));
  assert_eq!(param.value_input_count(), 1);
  assert_eq!(param.value_output_count(), 1);

  // Branch
  let br = common.branch(BranchHint::True, BranchSemantics::JS);
  assert_eq!(br.opcode(), IrOpcode::Branch as u16);
  assert!(br.properties().contains(Properties::KONTROL));
  assert_eq!(br.value_input_count(), 1);
  assert_eq!(br.control_input_count(), 1);
  assert_eq!(br.control_output_count(), 2);

  // Switch
  let sw = common.switch_op(4)?;
  assert_eq!(sw.opcode(), IrOpcode::Switch as u16);
  assert_eq!(sw.control_output_count(), 4);

  // Return
  let ret1 = common.return_op(1)?;
  assert_eq!(ret1.opcode(), IrOpcode::Return as u16);
  assert_eq!(ret1.value_input_count(), 2); // 1 + 1 (hidden value input)
  assert_eq!(ret1.effect_input_count(), 1);
  assert_eq!(ret1.control_input_count(), 1);
  assert_eq!(ret1.control_output_count(), 1);

  // Deoptimize
  let deopt = common.deoptimize(DeoptimizeReason::DivisionByZero, FeedbackSource::invalid())?;
  assert_eq!(deopt.opcode(), IrOpcode::Deoptimize as u16);
  assert_eq!(deopt.value_input_count(), 1);
  assert_eq!(deopt.control_output_count(), 1);

  // EffectPhi
  let ephi = common.effect_phi(2)?;
  assert_eq!(ephi.opcode(), IrOpcode::EffectPhi as u16);
  assert_eq!(ephi.effect_input_count(), 2);
  assert_eq!(ephi.control_input_count(), 1);
  assert_eq!(ephi.effect_output_count(), 1);

  // Loop & Merge
  let loop_op = common.loop_op(2, FeedbackSource::invalid())?;
  assert_eq!(loop_op.opcode(), IrOpcode::Loop as u16);
  assert_eq!(loop_op.control_input_count(), 2);
  assert_eq!(loop_op.control_output_count(), 1);

  let merge = common.merge(3)?;
  assert_eq!(merge.opcode(), IrOpcode::Merge as u16);
  assert_eq!(merge.control_input_count(), 3);
  assert_eq!(merge.control_output_count(), 1);

  // Phi
  let phi = common.phi(MachineRepresentation::Word32, 2)?;
  assert_eq!(phi.opcode(), IrOpcode::Phi as u16);
  assert!(phi.properties().contains(Properties::PURE));
  assert_eq!(phi.value_input_count(), 2);
  assert_eq!(phi.control_input_count(), 1);
  assert_eq!(phi.value_output_count(), 1);

  OK
}
