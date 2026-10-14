// 由 tool/src/compiler/machine-operator.h.js 从 v8/src/compiler/machine-operator.h + machine-operator.cc 生成; 修订就地改, 复跑覆盖

//! MachineOperatorBuilder 与 Machine 算子族
//!
//! 对应原版 machine-operator.h / machine-operator.cc。
//! 包含 Word32And, Word64And, Int32Add, Float64Add, Load, Store 等，
//! 支持 LoadRepresentation 与 StoreRepresentation。

use core::result::Result;
use std::sync::LazyLock;

use v8_compiler_graph::{
  IrOpcode, MachineRepresentation, MachineType, Operator, Operator1, OperatorCounts, Properties,
};
use v8_zone::Zone;

use crate::Error;

/// 写屏障种类 (WriteBarrierKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum WriteBarrierKind {
  #[default]
  NoWriteBarrier = 0,
  AssertNoWriteBarrier = 1,
  SkippedWriteBarrier = 2,
  MapWriteBarrier = 3,
  PointerWriteBarrier = 4,
  IndirectPointerWriteBarrier = 5,
  EphemeronKeyWriteBarrier = 6,
  FullWriteBarrier = 7,
}

/// LoadRepresentation = MachineType (1:1 原版 using LoadRepresentation = MachineType)
pub type LoadRepresentation = MachineType;

/// StoreRepresentation (机器表示 + 写屏障类型)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreRepresentation {
  representation: MachineRepresentation,
  write_barrier_kind: WriteBarrierKind,
}

impl StoreRepresentation {
  /// 创建 StoreRepresentation
  pub const fn new(
    representation: MachineRepresentation,
    write_barrier_kind: WriteBarrierKind,
  ) -> Self {
    Self {
      representation,
      write_barrier_kind,
    }
  }

  /// 机器表示
  #[inline]
  pub const fn representation(&self) -> MachineRepresentation {
    self.representation
  }

  /// 写屏障种类
  #[inline]
  pub const fn write_barrier_kind(&self) -> WriteBarrierKind {
    self.write_barrier_kind
  }
}

/// 全局共享 Machine 纯算子缓存
pub struct MachineOperatorGlobalCache {
  pub word32_and: Operator,
  pub word32_or: Operator,
  pub word32_xor: Operator,
  pub word32_shl: Operator,
  pub word32_shr: Operator,
  pub word32_ror: Operator,
  pub word32_equal: Operator,
  pub int32_add: Operator,
  pub int32_sub: Operator,
  pub int32_mul: Operator,
  pub int32_mul_high: Operator,
  pub int32_div: Operator,
  pub int32_mod: Operator,
  pub int32_less_than: Operator,
  pub int32_less_than_or_equal: Operator,
  pub uint32_div: Operator,
  pub uint32_less_than: Operator,
  pub uint32_less_than_or_equal: Operator,
  pub uint32_mod: Operator,
  pub uint32_mul_high: Operator,
  pub word64_and: Operator,
  pub word64_or: Operator,
  pub word64_xor: Operator,
  pub word64_shl: Operator,
  pub word64_shr: Operator,
  pub word64_ror: Operator,
  pub word64_ror_lowerable: Operator,
  pub word64_equal: Operator,
  pub int64_add: Operator,
  pub int64_sub: Operator,
  pub int64_mul: Operator,
  pub int64_mul_high: Operator,
  pub int64_div: Operator,
  pub int64_mod: Operator,
  pub int64_less_than: Operator,
  pub int64_less_than_or_equal: Operator,
  pub uint64_mul_high: Operator,
  pub uint64_div: Operator,
  pub uint64_mod: Operator,
  pub uint64_less_than: Operator,
  pub uint64_less_than_or_equal: Operator,
  pub float32_add: Operator,
  pub float32_sub: Operator,
  pub float32_mul: Operator,
  pub float32_div: Operator,
  pub float32_equal: Operator,
  pub float32_less_than: Operator,
  pub float32_less_than_or_equal: Operator,
  pub float64_add: Operator,
  pub float64_sub: Operator,
  pub float64_mul: Operator,
  pub float64_div: Operator,
  pub float64_mod: Operator,
  pub float64_equal: Operator,
  pub float64_less_than: Operator,
  pub float64_less_than_or_equal: Operator,
}

#[inline]
fn unwrap_op<T>(res: Result<T, v8_compiler_graph::Error>) -> T {
  match res {
    Ok(v) => v,
    Err(_) => unreachable!(),
  }
}

impl MachineOperatorGlobalCache {
  fn new() -> Self {
    Self {
      word32_and: unwrap_op(Operator::new(
        IrOpcode::Word32And as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Word32And",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word32_or: unwrap_op(Operator::new(
        IrOpcode::Word32Or as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Word32Or",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word32_xor: unwrap_op(Operator::new(
        IrOpcode::Word32Xor as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Word32Xor",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word32_shl: unwrap_op(Operator::new(
        IrOpcode::Word32Shl as u16,
        Properties::PURE,
        "Word32Shl",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word32_shr: unwrap_op(Operator::new(
        IrOpcode::Word32Shr as u16,
        Properties::PURE,
        "Word32Shr",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word32_ror: unwrap_op(Operator::new(
        IrOpcode::Word32Ror as u16,
        Properties::PURE,
        "Word32Ror",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word32_equal: unwrap_op(Operator::new(
        IrOpcode::Word32Equal as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "Word32Equal",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int32_add: unwrap_op(Operator::new(
        IrOpcode::Int32Add as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Int32Add",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int32_sub: unwrap_op(Operator::new(
        IrOpcode::Int32Sub as u16,
        Properties::PURE,
        "Int32Sub",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int32_mul: unwrap_op(Operator::new(
        IrOpcode::Int32Mul as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Int32Mul",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int32_mul_high: unwrap_op(Operator::new(
        IrOpcode::Int32MulHigh as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Int32MulHigh",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int32_div: unwrap_op(Operator::new(
        IrOpcode::Int32Div as u16,
        Properties::PURE,
        "Int32Div",
        OperatorCounts::new(2, 0, 1, 1, 0, 0),
      )),
      int32_mod: unwrap_op(Operator::new(
        IrOpcode::Int32Mod as u16,
        Properties::PURE,
        "Int32Mod",
        OperatorCounts::new(2, 0, 1, 1, 0, 0),
      )),
      int32_less_than: unwrap_op(Operator::new(
        IrOpcode::Int32LessThan as u16,
        Properties::PURE,
        "Int32LessThan",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int32_less_than_or_equal: unwrap_op(Operator::new(
        IrOpcode::Int32LessThanOrEqual as u16,
        Properties::PURE,
        "Int32LessThanOrEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      uint32_div: unwrap_op(Operator::new(
        IrOpcode::Uint32Div as u16,
        Properties::PURE,
        "Uint32Div",
        OperatorCounts::new(2, 0, 1, 1, 0, 0),
      )),
      uint32_less_than: unwrap_op(Operator::new(
        IrOpcode::Uint32LessThan as u16,
        Properties::PURE,
        "Uint32LessThan",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      uint32_less_than_or_equal: unwrap_op(Operator::new(
        IrOpcode::Uint32LessThanOrEqual as u16,
        Properties::PURE,
        "Uint32LessThanOrEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      uint32_mod: unwrap_op(Operator::new(
        IrOpcode::Uint32Mod as u16,
        Properties::PURE,
        "Uint32Mod",
        OperatorCounts::new(2, 0, 1, 1, 0, 0),
      )),
      uint32_mul_high: unwrap_op(Operator::new(
        IrOpcode::Uint32MulHigh as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Uint32MulHigh",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word64_and: unwrap_op(Operator::new(
        IrOpcode::Word64And as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Word64And",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word64_or: unwrap_op(Operator::new(
        IrOpcode::Word64Or as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Word64Or",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word64_xor: unwrap_op(Operator::new(
        IrOpcode::Word64Xor as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Word64Xor",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word64_shl: unwrap_op(Operator::new(
        IrOpcode::Word64Shl as u16,
        Properties::PURE,
        "Word64Shl",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word64_shr: unwrap_op(Operator::new(
        IrOpcode::Word64Shr as u16,
        Properties::PURE,
        "Word64Shr",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word64_ror: unwrap_op(Operator::new(
        IrOpcode::Word64Ror as u16,
        Properties::PURE,
        "Word64Ror",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      word64_ror_lowerable: unwrap_op(Operator::new(
        IrOpcode::Word64RorLowerable as u16,
        Properties::PURE,
        "Word64RorLowerable",
        OperatorCounts::new(2, 0, 1, 1, 0, 0),
      )),
      word64_equal: unwrap_op(Operator::new(
        IrOpcode::Word64Equal as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "Word64Equal",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int64_add: unwrap_op(Operator::new(
        IrOpcode::Int64Add as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Int64Add",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int64_sub: unwrap_op(Operator::new(
        IrOpcode::Int64Sub as u16,
        Properties::PURE,
        "Int64Sub",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int64_mul: unwrap_op(Operator::new(
        IrOpcode::Int64Mul as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Int64Mul",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int64_mul_high: unwrap_op(Operator::new(
        IrOpcode::Int64MulHigh as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Int64MulHigh",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int64_div: unwrap_op(Operator::new(
        IrOpcode::Int64Div as u16,
        Properties::PURE,
        "Int64Div",
        OperatorCounts::new(2, 0, 1, 1, 0, 0),
      )),
      int64_mod: unwrap_op(Operator::new(
        IrOpcode::Int64Mod as u16,
        Properties::PURE,
        "Int64Mod",
        OperatorCounts::new(2, 0, 1, 1, 0, 0),
      )),
      int64_less_than: unwrap_op(Operator::new(
        IrOpcode::Int64LessThan as u16,
        Properties::PURE,
        "Int64LessThan",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      int64_less_than_or_equal: unwrap_op(Operator::new(
        IrOpcode::Int64LessThanOrEqual as u16,
        Properties::PURE,
        "Int64LessThanOrEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      uint64_mul_high: unwrap_op(Operator::new(
        IrOpcode::Uint64MulHigh as u16,
        Properties::PURE | Properties::ASSOCIATIVE | Properties::COMMUTATIVE,
        "Uint64MulHigh",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      uint64_div: unwrap_op(Operator::new(
        IrOpcode::Uint64Div as u16,
        Properties::PURE,
        "Uint64Div",
        OperatorCounts::new(2, 0, 1, 1, 0, 0),
      )),
      uint64_mod: unwrap_op(Operator::new(
        IrOpcode::Uint64Mod as u16,
        Properties::PURE,
        "Uint64Mod",
        OperatorCounts::new(2, 0, 1, 1, 0, 0),
      )),
      uint64_less_than: unwrap_op(Operator::new(
        IrOpcode::Uint64LessThan as u16,
        Properties::PURE,
        "Uint64LessThan",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      uint64_less_than_or_equal: unwrap_op(Operator::new(
        IrOpcode::Uint64LessThanOrEqual as u16,
        Properties::PURE,
        "Uint64LessThanOrEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float32_add: unwrap_op(Operator::new(
        IrOpcode::Float32Add as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "Float32Add",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float32_sub: unwrap_op(Operator::new(
        IrOpcode::Float32Sub as u16,
        Properties::PURE,
        "Float32Sub",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float32_mul: unwrap_op(Operator::new(
        IrOpcode::Float32Mul as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "Float32Mul",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float32_div: unwrap_op(Operator::new(
        IrOpcode::Float32Div as u16,
        Properties::PURE,
        "Float32Div",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float32_equal: unwrap_op(Operator::new(
        IrOpcode::Float32Equal as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "Float32Equal",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float32_less_than: unwrap_op(Operator::new(
        IrOpcode::Float32LessThan as u16,
        Properties::PURE,
        "Float32LessThan",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float32_less_than_or_equal: unwrap_op(Operator::new(
        IrOpcode::Float32LessThanOrEqual as u16,
        Properties::PURE,
        "Float32LessThanOrEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float64_add: unwrap_op(Operator::new(
        IrOpcode::Float64Add as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "Float64Add",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float64_sub: unwrap_op(Operator::new(
        IrOpcode::Float64Sub as u16,
        Properties::PURE,
        "Float64Sub",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float64_mul: unwrap_op(Operator::new(
        IrOpcode::Float64Mul as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "Float64Mul",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float64_div: unwrap_op(Operator::new(
        IrOpcode::Float64Div as u16,
        Properties::PURE,
        "Float64Div",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float64_mod: unwrap_op(Operator::new(
        IrOpcode::Float64Mod as u16,
        Properties::PURE,
        "Float64Mod",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float64_equal: unwrap_op(Operator::new(
        IrOpcode::Float64Equal as u16,
        Properties::PURE | Properties::COMMUTATIVE,
        "Float64Equal",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float64_less_than: unwrap_op(Operator::new(
        IrOpcode::Float64LessThan as u16,
        Properties::PURE,
        "Float64LessThan",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
      float64_less_than_or_equal: unwrap_op(Operator::new(
        IrOpcode::Float64LessThanOrEqual as u16,
        Properties::PURE,
        "Float64LessThanOrEqual",
        OperatorCounts::new(2, 0, 0, 1, 0, 0),
      )),
    }
  }
}

static MACHINE_CACHE: LazyLock<MachineOperatorGlobalCache> =
  LazyLock::new(MachineOperatorGlobalCache::new);

/// MachineOperatorBuilder
pub struct MachineOperatorBuilder<'z, 'a> {
  zone: &'z Zone<'a>,
}

impl<'z, 'a> MachineOperatorBuilder<'z, 'a> {
  /// 创建构建器
  pub fn new(zone: &'z Zone<'a>) -> Self {
    Self { zone }
  }

  /// 底层 Zone 引用
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  // —— 纯计算算子 ——

  /// Word32And
  #[inline]
  pub fn word32_and(&self) -> &'static Operator {
    &MACHINE_CACHE.word32_and
  }

  /// Word32Or
  #[inline]
  pub fn word32_or(&self) -> &'static Operator {
    &MACHINE_CACHE.word32_or
  }

  /// Word32Xor
  #[inline]
  pub fn word32_xor(&self) -> &'static Operator {
    &MACHINE_CACHE.word32_xor
  }

  /// Word32Shl
  #[inline]
  pub fn word32_shl(&self) -> &'static Operator {
    &MACHINE_CACHE.word32_shl
  }

  /// Word32Shr
  #[inline]
  pub fn word32_shr(&self) -> &'static Operator {
    &MACHINE_CACHE.word32_shr
  }

  /// Word32Ror
  #[inline]
  pub fn word32_ror(&self) -> &'static Operator {
    &MACHINE_CACHE.word32_ror
  }

  /// Word32Equal
  #[inline]
  pub fn word32_equal(&self) -> &'static Operator {
    &MACHINE_CACHE.word32_equal
  }

  /// Int32Add
  #[inline]
  pub fn int32_add(&self) -> &'static Operator {
    &MACHINE_CACHE.int32_add
  }

  /// Int32Sub
  #[inline]
  pub fn int32_sub(&self) -> &'static Operator {
    &MACHINE_CACHE.int32_sub
  }

  /// Int32Mul
  #[inline]
  pub fn int32_mul(&self) -> &'static Operator {
    &MACHINE_CACHE.int32_mul
  }

  /// Int32MulHigh
  #[inline]
  pub fn int32_mul_high(&self) -> &'static Operator {
    &MACHINE_CACHE.int32_mul_high
  }

  /// Int32Div
  #[inline]
  pub fn int32_div(&self) -> &'static Operator {
    &MACHINE_CACHE.int32_div
  }

  /// Int32Mod
  #[inline]
  pub fn int32_mod(&self) -> &'static Operator {
    &MACHINE_CACHE.int32_mod
  }

  /// Int32LessThan
  #[inline]
  pub fn int32_less_than(&self) -> &'static Operator {
    &MACHINE_CACHE.int32_less_than
  }

  /// Int32LessThanOrEqual
  #[inline]
  pub fn int32_less_than_or_equal(&self) -> &'static Operator {
    &MACHINE_CACHE.int32_less_than_or_equal
  }

  /// Uint32Div
  #[inline]
  pub fn uint32_div(&self) -> &'static Operator {
    &MACHINE_CACHE.uint32_div
  }

  /// Uint32LessThan
  #[inline]
  pub fn uint32_less_than(&self) -> &'static Operator {
    &MACHINE_CACHE.uint32_less_than
  }

  /// Uint32LessThanOrEqual
  #[inline]
  pub fn uint32_less_than_or_equal(&self) -> &'static Operator {
    &MACHINE_CACHE.uint32_less_than_or_equal
  }

  /// Uint32Mod
  #[inline]
  pub fn uint32_mod(&self) -> &'static Operator {
    &MACHINE_CACHE.uint32_mod
  }

  /// Uint32MulHigh
  #[inline]
  pub fn uint32_mul_high(&self) -> &'static Operator {
    &MACHINE_CACHE.uint32_mul_high
  }

  /// Word64And
  #[inline]
  pub fn word64_and(&self) -> &'static Operator {
    &MACHINE_CACHE.word64_and
  }

  /// Word64Or
  #[inline]
  pub fn word64_or(&self) -> &'static Operator {
    &MACHINE_CACHE.word64_or
  }

  /// Word64Xor
  #[inline]
  pub fn word64_xor(&self) -> &'static Operator {
    &MACHINE_CACHE.word64_xor
  }

  /// Word64Shl
  #[inline]
  pub fn word64_shl(&self) -> &'static Operator {
    &MACHINE_CACHE.word64_shl
  }

  /// Word64Shr
  #[inline]
  pub fn word64_shr(&self) -> &'static Operator {
    &MACHINE_CACHE.word64_shr
  }

  /// Word64Ror
  #[inline]
  pub fn word64_ror(&self) -> &'static Operator {
    &MACHINE_CACHE.word64_ror
  }

  /// Word64RorLowerable
  #[inline]
  pub fn word64_ror_lowerable(&self) -> &'static Operator {
    &MACHINE_CACHE.word64_ror_lowerable
  }

  /// Word64Equal
  #[inline]
  pub fn word64_equal(&self) -> &'static Operator {
    &MACHINE_CACHE.word64_equal
  }

  /// Int64Add
  #[inline]
  pub fn int64_add(&self) -> &'static Operator {
    &MACHINE_CACHE.int64_add
  }

  /// Int64Sub
  #[inline]
  pub fn int64_sub(&self) -> &'static Operator {
    &MACHINE_CACHE.int64_sub
  }

  /// Int64Mul
  #[inline]
  pub fn int64_mul(&self) -> &'static Operator {
    &MACHINE_CACHE.int64_mul
  }

  /// Int64MulHigh
  #[inline]
  pub fn int64_mul_high(&self) -> &'static Operator {
    &MACHINE_CACHE.int64_mul_high
  }

  /// Int64Div
  #[inline]
  pub fn int64_div(&self) -> &'static Operator {
    &MACHINE_CACHE.int64_div
  }

  /// Int64Mod
  #[inline]
  pub fn int64_mod(&self) -> &'static Operator {
    &MACHINE_CACHE.int64_mod
  }

  /// Int64LessThan
  #[inline]
  pub fn int64_less_than(&self) -> &'static Operator {
    &MACHINE_CACHE.int64_less_than
  }

  /// Int64LessThanOrEqual
  #[inline]
  pub fn int64_less_than_or_equal(&self) -> &'static Operator {
    &MACHINE_CACHE.int64_less_than_or_equal
  }

  /// Uint64MulHigh
  #[inline]
  pub fn uint64_mul_high(&self) -> &'static Operator {
    &MACHINE_CACHE.uint64_mul_high
  }

  /// Uint64Div
  #[inline]
  pub fn uint64_div(&self) -> &'static Operator {
    &MACHINE_CACHE.uint64_div
  }

  /// Uint64Mod
  #[inline]
  pub fn uint64_mod(&self) -> &'static Operator {
    &MACHINE_CACHE.uint64_mod
  }

  /// Uint64LessThan
  #[inline]
  pub fn uint64_less_than(&self) -> &'static Operator {
    &MACHINE_CACHE.uint64_less_than
  }

  /// Uint64LessThanOrEqual
  #[inline]
  pub fn uint64_less_than_or_equal(&self) -> &'static Operator {
    &MACHINE_CACHE.uint64_less_than_or_equal
  }

  /// Float32Add
  #[inline]
  pub fn float32_add(&self) -> &'static Operator {
    &MACHINE_CACHE.float32_add
  }

  /// Float32Sub
  #[inline]
  pub fn float32_sub(&self) -> &'static Operator {
    &MACHINE_CACHE.float32_sub
  }

  /// Float32Mul
  #[inline]
  pub fn float32_mul(&self) -> &'static Operator {
    &MACHINE_CACHE.float32_mul
  }

  /// Float32Div
  #[inline]
  pub fn float32_div(&self) -> &'static Operator {
    &MACHINE_CACHE.float32_div
  }

  /// Float32Equal
  #[inline]
  pub fn float32_equal(&self) -> &'static Operator {
    &MACHINE_CACHE.float32_equal
  }

  /// Float32LessThan
  #[inline]
  pub fn float32_less_than(&self) -> &'static Operator {
    &MACHINE_CACHE.float32_less_than
  }

  /// Float32LessThanOrEqual
  #[inline]
  pub fn float32_less_than_or_equal(&self) -> &'static Operator {
    &MACHINE_CACHE.float32_less_than_or_equal
  }

  /// Float64Add
  #[inline]
  pub fn float64_add(&self) -> &'static Operator {
    &MACHINE_CACHE.float64_add
  }

  /// Float64Sub
  #[inline]
  pub fn float64_sub(&self) -> &'static Operator {
    &MACHINE_CACHE.float64_sub
  }

  /// Float64Mul
  #[inline]
  pub fn float64_mul(&self) -> &'static Operator {
    &MACHINE_CACHE.float64_mul
  }

  /// Float64Div
  #[inline]
  pub fn float64_div(&self) -> &'static Operator {
    &MACHINE_CACHE.float64_div
  }

  /// Float64Mod
  #[inline]
  pub fn float64_mod(&self) -> &'static Operator {
    &MACHINE_CACHE.float64_mod
  }

  /// Float64Equal
  #[inline]
  pub fn float64_equal(&self) -> &'static Operator {
    &MACHINE_CACHE.float64_equal
  }

  /// Float64LessThan
  #[inline]
  pub fn float64_less_than(&self) -> &'static Operator {
    &MACHINE_CACHE.float64_less_than
  }

  /// Float64LessThanOrEqual
  #[inline]
  pub fn float64_less_than_or_equal(&self) -> &'static Operator {
    &MACHINE_CACHE.float64_less_than_or_equal
  }

  /// Load 算子
  pub fn load(&self, rep: LoadRepresentation) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::Load as u16,
      Properties::ELIMINATABLE,
      "Load",
      OperatorCounts::new(2, 1, 1, 1, 1, 0),
      rep,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// Store 算子
  pub fn store(&self, rep: StoreRepresentation) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::Store as u16,
      Properties::NO_DEOPT | Properties::NO_READ | Properties::NO_THROW,
      "Store",
      OperatorCounts::new(3, 1, 1, 0, 1, 0),
      rep,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// UnalignedLoad 算子
  pub fn unaligned_load(&self, rep: LoadRepresentation) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::UnalignedLoad as u16,
      Properties::ELIMINATABLE,
      "UnalignedLoad",
      OperatorCounts::new(2, 1, 1, 1, 1, 0),
      rep,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// UnalignedStore 算子
  pub fn unaligned_store(&self, rep: MachineRepresentation) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::UnalignedStore as u16,
      Properties::NO_DEOPT | Properties::NO_READ | Properties::NO_THROW,
      "UnalignedStore",
      OperatorCounts::new(3, 1, 1, 0, 1, 0),
      rep,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }
}
