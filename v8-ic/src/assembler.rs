use strum::{Display, EnumIter, EnumString};

/// IC CSA/TSA 汇编生成器种类 (来源: accessor-assembler.h, binary-op-assembler.h, unary-op-assembler.h, keyed-store-generic.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum AssemblerKind {
  AccessorAssembler,
  BinaryOpAssembler,
  UnaryOpAssembler,
  KeyedStoreGeneric,
}

impl AssemblerKind {
  #[inline]
  pub const fn is_accessor(&self) -> bool {
    matches!(self, Self::AccessorAssembler)
  }

  #[inline]
  pub const fn is_binary_op(&self) -> bool {
    matches!(self, Self::BinaryOpAssembler)
  }

  #[inline]
  pub const fn is_unary_op(&self) -> bool {
    matches!(self, Self::UnaryOpAssembler)
  }

  #[inline]
  pub const fn is_keyed_store(&self) -> bool {
    matches!(self, Self::KeyedStoreGeneric)
  }
}

/// IC 二元操作算子种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum BinaryOpKind {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  ShiftLeft,
  ShiftRight,
  ShiftRightLogical,
}

impl BinaryOpKind {
  #[inline]
  pub const fn is_arithmetic(&self) -> bool {
    matches!(
      self,
      Self::Add | Self::Subtract | Self::Multiply | Self::Divide | Self::Modulus
    )
  }

  #[inline]
  pub const fn is_bitwise(&self) -> bool {
    matches!(
      self,
      Self::BitwiseAnd
        | Self::BitwiseOr
        | Self::BitwiseXor
        | Self::ShiftLeft
        | Self::ShiftRight
        | Self::ShiftRightLogical
    )
  }
}

/// IC 一元操作算子种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum UnaryOpKind {
  Negate,
  BitwiseNot,
}

/// 汇编生成配置参数
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AssemblerConfig {
  pub collect_feedback: bool,
  pub is_baseline: bool,
}
