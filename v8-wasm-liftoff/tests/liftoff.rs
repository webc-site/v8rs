//! Liftoff 基线编译器核心数据结构与常数集成测试
//! 对应原版 v8/src/wasm/baseline/liftoff-*.h 规范

use aok::{OK, Void};
use v8_wasm_core::value_type::ValueKind;
use v8_wasm_liftoff::{
  Endianness, INLINE_LOCAL_KINDS, IgnoreLocals, JumpDirection, LiftoffBailoutReason, LoadKind,
  REG_CLASS_FOR, RegClass, RegPairHalf, STACK_SLOT_SIZE, SUPPORTS_STORE_CONST, SmiCheckMode,
  SpillLocation, VarStateLocation, arm64, register_arm64, register_x64, x64,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_liftoff_bailout_reasons() -> Void {
  assert_eq!(LiftoffBailoutReason::NoReason.to_str(), "no bailout");
  assert_eq!(LiftoffBailoutReason::DecodeError.to_str(), "decode error");
  assert_eq!(LiftoffBailoutReason::Gc.to_str(), "gc unsupported");
  assert_eq!(LiftoffBailoutReason::OtherReason.to_str(), "other reason");
  assert_eq!(
    LiftoffBailoutReason::NumBailoutReasons.to_str(),
    "num bailout reasons"
  );
  assert_eq!(LiftoffBailoutReason::NoReason as i8, 0);
  assert_eq!(LiftoffBailoutReason::OtherReason as i8, 20);
  OK
}

#[test]
fn test_reg_class_mapping() -> Void {
  for (kind, reg_class) in REG_CLASS_FOR {
    match kind {
      ValueKind::I32 | ValueKind::I64 | ValueKind::I8 | ValueKind::I16 => {
        assert_eq!(reg_class, RegClass::GpReg);
      }
      ValueKind::Ref | ValueKind::RefNull => {
        assert_eq!(reg_class, RegClass::GpReg);
      }
      ValueKind::F32 | ValueKind::F64 | ValueKind::S128 | ValueKind::F16 => {
        assert_eq!(reg_class, RegClass::FpReg);
      }
      ValueKind::Void => {
        assert_eq!(reg_class, RegClass::NoReg);
      }
      _ => {}
    }
  }
  OK
}

#[test]
fn test_assembler_cache_registers() -> Void {
  // x64 寄存器分布
  assert_eq!(x64::GP_CACHE_REGS.len(), 10);
  assert_eq!(x64::FP_CACHE_REGS.len(), 8);
  assert_eq!(x64::FRAME_SETUP_FUNCTION_REG, "r12");

  // arm64 寄存器分布
  assert_eq!(arm64::GP_CACHE_REGS.len(), 24);
  assert_eq!(arm64::FP_CACHE_REGS.len(), 27);
  assert_eq!(arm64::FRAME_SETUP_FUNCTION_REG, "x8");

  // 寄存器位宽与限制
  assert_eq!(register_x64::BITS_PER_GP_REG_CODE, 4);
  assert_eq!(register_x64::AFTER_MAX_LIFTOFF_REG_CODE, 24);
  assert_eq!(register_arm64::BITS_PER_GP_REG_CODE, 5);
  assert_eq!(register_arm64::AFTER_MAX_LIFTOFF_REG_CODE, 56);
  OK
}

#[test]
fn test_liftoff_state_enums() -> Void {
  assert_eq!(VarStateLocation::Stack as u8, 0);
  assert_eq!(VarStateLocation::Register as u8, 1);
  assert_eq!(VarStateLocation::IntConst as u8, 2);

  assert_eq!(LoadKind::Nop as u8, 0);
  assert_eq!(LoadKind::Constant as u8, 1);
  assert_eq!(LoadKind::HighHalfStack as u8, 4);

  assert_eq!(SpillLocation::TopOfStack as u8, 0);
  assert_eq!(SpillLocation::StackSlots as u8, 1);

  assert_eq!(IgnoreLocals::IncludeLocals as u8, 0);
  assert_eq!(IgnoreLocals::IgnoreLocals as u8, 1);

  assert_eq!(JumpDirection::ForwardJump as u8, 0);
  assert_eq!(JumpDirection::BackwardJump as u8, 1);

  assert_eq!(Endianness::Native as u8, 0);
  assert_eq!(Endianness::Little as u8, 1);

  assert_eq!(SmiCheckMode::JumpOnSmi as u8, 0);
  assert_eq!(SmiCheckMode::JumpOnNotSmi as u8, 1);

  assert_eq!(RegPairHalf::LowWord as u8, 0);
  assert_eq!(RegPairHalf::HighWord as u8, 1);

  assert_eq!(STACK_SLOT_SIZE, 8);
  assert_eq!(INLINE_LOCAL_KINDS, 16);
  const { assert!(SUPPORTS_STORE_CONST) };
  OK
}

#[test]
fn test_supports_liftoff_compilation() -> Void {
  use v8_wasm_core::wasm_tier::ExecutionTier;
  use v8_wasm_liftoff::supports_liftoff_compilation;

  assert!(supports_liftoff_compilation(ExecutionTier::Liftoff));
  assert!(!supports_liftoff_compilation(ExecutionTier::Turbofan));
  assert!(!supports_liftoff_compilation(ExecutionTier::None));
  OK
}
