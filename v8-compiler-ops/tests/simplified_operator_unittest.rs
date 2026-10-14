//! SimplifiedOperatorBuilder 单元测试

use aok::{OK, Void};
use v8_compiler_graph::{CheckForMinusZeroMode, FeedbackSource, IrOpcode, Properties};
use v8_compiler_ops::{
  CheckBoundsFlags, CheckMapsFlags, DeoptimizeReason, SimplifiedOperatorBuilder,
};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_simplified_pure_operators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let simplified = SimplifiedOperatorBuilder::new(&zone);

  // NumberAdd
  let num_add = simplified.number_add();
  assert_eq!(num_add.opcode(), IrOpcode::NumberAdd as u16);
  assert!(num_add.properties().contains(Properties::PURE));
  assert!(num_add.properties().contains(Properties::COMMUTATIVE));
  assert_eq!(num_add.value_input_count(), 2);
  assert_eq!(num_add.value_output_count(), 1);

  // NumberSubtract
  let num_sub = simplified.number_subtract();
  assert_eq!(num_sub.opcode(), IrOpcode::NumberSubtract as u16);
  assert!(num_sub.properties().contains(Properties::PURE));
  assert!(!num_sub.properties().contains(Properties::COMMUTATIVE));
  assert_eq!(num_sub.value_input_count(), 2);

  // NumberMultiply & NumberDivide
  let num_mul = simplified.number_multiply();
  assert_eq!(num_mul.opcode(), IrOpcode::NumberMultiply as u16);
  assert!(num_mul.properties().contains(Properties::COMMUTATIVE));

  let num_div = simplified.number_divide();
  assert_eq!(num_div.opcode(), IrOpcode::NumberDivide as u16);
  assert!(!num_div.properties().contains(Properties::COMMUTATIVE));

  // NumberEqual
  let num_eq = simplified.number_equal();
  assert_eq!(num_eq.opcode(), IrOpcode::NumberEqual as u16);
  assert!(num_eq.properties().contains(Properties::COMMUTATIVE));

  OK
}

#[test]
fn test_simplified_speculative_operators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let simplified = SimplifiedOperatorBuilder::new(&zone);

  // CheckBounds (cached vs dynamic)
  let check_bounds_cached =
    simplified.check_bounds(FeedbackSource::invalid(), CheckBoundsFlags::NONE)?;
  assert_eq!(check_bounds_cached.opcode(), IrOpcode::CheckBounds as u16);
  assert!(
    check_bounds_cached
      .properties()
      .contains(Properties::FOLDABLE)
  );
  assert!(
    check_bounds_cached
      .properties()
      .contains(Properties::NO_THROW)
  );
  assert_eq!(check_bounds_cached.value_input_count(), 2);
  assert_eq!(check_bounds_cached.effect_input_count(), 1);
  assert_eq!(check_bounds_cached.control_input_count(), 1);
  assert_eq!(check_bounds_cached.value_output_count(), 1);

  let check_bounds_abort = simplified.check_bounds(
    FeedbackSource::invalid(),
    CheckBoundsFlags::ABORT_ON_OUT_OF_BOUNDS,
  )?;
  assert_eq!(check_bounds_abort.opcode(), IrOpcode::CheckBounds as u16);

  // CheckMaps
  let check_maps_nowrite =
    simplified.check_maps(CheckMapsFlags::NONE, FeedbackSource::invalid())?;
  assert_eq!(check_maps_nowrite.opcode(), IrOpcode::CheckMaps as u16);
  assert!(
    check_maps_nowrite
      .properties()
      .contains(Properties::NO_WRITE)
  );
  assert!(
    check_maps_nowrite
      .properties()
      .contains(Properties::NO_THROW)
  );
  assert_eq!(check_maps_nowrite.value_input_count(), 1);
  assert_eq!(check_maps_nowrite.value_output_count(), 0);

  let check_maps_migrate = simplified.check_maps(
    CheckMapsFlags::TRY_MIGRATE_INSTANCE,
    FeedbackSource::invalid(),
  )?;
  assert!(
    !check_maps_migrate
      .properties()
      .contains(Properties::NO_WRITE)
  );

  // CheckNumber / CheckSmi / CheckString
  let check_num = simplified.check_number(FeedbackSource::invalid())?;
  assert_eq!(check_num.opcode(), IrOpcode::CheckNumber as u16);
  assert_eq!(check_num.value_input_count(), 1);
  assert_eq!(check_num.value_output_count(), 1);

  let check_smi = simplified.check_smi(FeedbackSource::invalid())?;
  assert_eq!(check_smi.opcode(), IrOpcode::CheckSmi as u16);

  let check_str = simplified.check_string(FeedbackSource::invalid())?;
  assert_eq!(check_str.opcode(), IrOpcode::CheckString as u16);

  // CheckIf
  let check_if = simplified.check_if(DeoptimizeReason::OutOfBounds, FeedbackSource::invalid())?;
  assert_eq!(check_if.opcode(), IrOpcode::CheckIf as u16);
  assert_eq!(check_if.value_output_count(), 0);

  // CheckedInt32Add & CheckedInt32Sub
  let c_i32_add = simplified.checked_int32_add();
  assert_eq!(c_i32_add.opcode(), IrOpcode::CheckedInt32Add as u16);
  assert_eq!(c_i32_add.value_input_count(), 2);

  let c_i32_sub = simplified.checked_int32_sub();
  assert_eq!(c_i32_sub.opcode(), IrOpcode::CheckedInt32Sub as u16);

  // CheckedInt32Mul with CheckForMinusZeroMode
  let c_i32_mul = simplified.checked_int32_mul(CheckForMinusZeroMode::Check)?;
  assert_eq!(c_i32_mul.opcode(), IrOpcode::CheckedInt32Mul as u16);

  OK
}
