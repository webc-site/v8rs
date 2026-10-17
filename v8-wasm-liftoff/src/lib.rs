#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod liftoff_assembler_consts;
mod liftoff_assembler_defs;
mod liftoff_bailout_reasons;
mod liftoff_register;
mod liftoff_varstate;
mod parallel_move;

pub use error::{Error, Result};
pub use liftoff_assembler_consts::{
  Endianness, INLINE_LOCAL_KINDS, IgnoreLocals, JumpDirection, NO_CACHED_MEM_INDEX,
  STACK_SLOT_SIZE, SUPPORTS_STORE_CONST, SUPPORTS_STORE_CONST_ARCH_BRANCH, SmiCheckMode,
  SpillLocation,
};
pub use liftoff_assembler_defs::{arm64, x64};
pub use liftoff_bailout_reasons::LiftoffBailoutReason;
pub use liftoff_register::{
  FP_REG_PAIR, GP_REG_PAIR, HAS_INDEPENDENT_SIMD128_REGS, NEED_I64_REG_PAIR, NEED_S128_REG_PAIR,
  REG_CLASS_FOR, RegClass, RegPairHalf, SIMD128_REG, arm64 as register_arm64, x64 as register_x64,
};
pub use liftoff_varstate::VarStateLocation;
pub use parallel_move::{LoadKind, RegisterMove};
use v8_wasm_core::wasm_tier::ExecutionTier;

/// 桥接: 从 Wasm 编译层级判定是否启用 Liftoff 基线编译 (打通 Liftoff 基线编译入口)
#[inline]
#[must_use]
pub fn supports_liftoff_compilation(tier: ExecutionTier) -> bool {
  matches!(tier, ExecutionTier::Liftoff)
}
