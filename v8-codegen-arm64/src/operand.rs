use crate::registers::Register;

/// 移位类型 (对应原版 v8::internal::Shift)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum Shift {
  #[default]
  LSL = 0,
  LSR = 1,
  ASR = 2,
  ROR = 3,
  // MSL 与 constants.rs:475 MSL=4 对齐 (constants-arm64.h:389);
  // 原版 NO_SHIFT=-1 哨兵在 repr(u8) 域不可表, 以 Option<Shift>/整数域承接 (round5 D1 补)
  MSL = 4,
}

/// 扩展类型 (对应原版 v8::internal::Extend)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum Extend {
  #[default]
  UXTB = 0,
  UXTH = 1,
  UXTW = 2,
  UXTX = 3,
  SXTB = 4,
  SXTH = 5,
  SXTW = 6,
  SXTX = 7,
}

/// 内存寻址模式 (对应原版 v8::internal::AddrMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AddrMode {
  #[default]
  Offset,
  PreIndex,
  PostIndex,
}

/// 汇编指令操作数 (寄存器、带移位寄存器、带扩展寄存器或立即数)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
  Reg(Register),
  Shifted(Register, Shift, u32),
  Extended(Register, Extend, u32),
  Imm(i64),
}

impl Operand {
  #[inline]
  pub const fn reg(reg: Register) -> Self {
    Self::Reg(reg)
  }

  #[inline]
  pub const fn shifted(reg: Register, shift: Shift, shift_amount: u32) -> Self {
    Self::Shifted(reg, shift, shift_amount)
  }

  #[inline]
  pub const fn extended(reg: Register, extend: Extend, shift_amount: u32) -> Self {
    Self::Extended(reg, extend, shift_amount)
  }

  #[inline]
  pub const fn imm(val: i64) -> Self {
    Self::Imm(val)
  }

  #[inline]
  pub const fn is_immediate(&self) -> bool {
    matches!(self, Self::Imm(_))
  }

  #[inline]
  pub const fn is_reg(&self) -> bool {
    matches!(self, Self::Reg(_))
  }

  #[inline]
  pub const fn is_shifted(&self) -> bool {
    matches!(self, Self::Shifted(_, _, _))
  }

  #[inline]
  pub const fn is_extended(&self) -> bool {
    matches!(self, Self::Extended(_, _, _))
  }
}

impl From<Register> for Operand {
  #[inline]
  fn from(reg: Register) -> Self {
    Self::Reg(reg)
  }
}

impl From<i32> for Operand {
  #[inline]
  fn from(val: i32) -> Self {
    Self::Imm(val as i64)
  }
}

impl From<i64> for Operand {
  #[inline]
  fn from(val: i64) -> Self {
    Self::Imm(val)
  }
}

/// 内存操作数 (对应原版 v8::internal::MemOperand)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemOperand {
  base: Register,
  offset: i64,
  regoffset: Option<Register>,
  shift: Shift,
  extend: Extend,
  shift_amount: u32,
  addrmode: AddrMode,
}

impl MemOperand {
  #[inline]
  pub const fn base(base: Register) -> Self {
    Self {
      base,
      offset: 0,
      regoffset: None,
      shift: Shift::LSL,
      extend: Extend::UXTX,
      shift_amount: 0,
      addrmode: AddrMode::Offset,
    }
  }

  #[inline]
  pub const fn offset(base: Register, offset: i64) -> Self {
    Self {
      base,
      offset,
      regoffset: None,
      shift: Shift::LSL,
      extend: Extend::UXTX,
      shift_amount: 0,
      addrmode: AddrMode::Offset,
    }
  }

  #[inline]
  pub const fn pre_index(base: Register, offset: i64) -> Self {
    Self {
      base,
      offset,
      regoffset: None,
      shift: Shift::LSL,
      extend: Extend::UXTX,
      shift_amount: 0,
      addrmode: AddrMode::PreIndex,
    }
  }

  #[inline]
  pub const fn post_index(base: Register, offset: i64) -> Self {
    Self {
      base,
      offset,
      regoffset: None,
      shift: Shift::LSL,
      extend: Extend::UXTX,
      shift_amount: 0,
      addrmode: AddrMode::PostIndex,
    }
  }

  #[inline]
  pub const fn reg_offset(
    base: Register,
    regoffset: Register,
    shift: Shift,
    shift_amount: u32,
  ) -> Self {
    Self {
      base,
      offset: 0,
      regoffset: Some(regoffset),
      shift,
      extend: Extend::UXTX,
      shift_amount,
      addrmode: AddrMode::Offset,
    }
  }

  #[inline]
  pub const fn reg_extended(
    base: Register,
    regoffset: Register,
    extend: Extend,
    shift_amount: u32,
  ) -> Self {
    Self {
      base,
      offset: 0,
      regoffset: Some(regoffset),
      shift: Shift::LSL,
      extend,
      shift_amount,
      addrmode: AddrMode::Offset,
    }
  }

  #[inline]
  pub const fn get_base(&self) -> Register {
    self.base
  }

  #[inline]
  pub const fn get_offset(&self) -> i64 {
    self.offset
  }

  #[inline]
  pub const fn get_regoffset(&self) -> Option<Register> {
    self.regoffset
  }

  #[inline]
  pub const fn get_shift(&self) -> Shift {
    self.shift
  }

  #[inline]
  pub const fn get_extend(&self) -> Extend {
    self.extend
  }

  #[inline]
  pub const fn get_shift_amount(&self) -> u32 {
    self.shift_amount
  }

  #[inline]
  pub const fn get_addrmode(&self) -> AddrMode {
    self.addrmode
  }

  #[inline]
  pub const fn is_immediate_offset(&self) -> bool {
    self.regoffset.is_none() && matches!(self.addrmode, AddrMode::Offset)
  }

  #[inline]
  pub const fn is_register_offset(&self) -> bool {
    self.regoffset.is_some() && matches!(self.addrmode, AddrMode::Offset)
  }

  #[inline]
  pub const fn is_pre_index(&self) -> bool {
    matches!(self.addrmode, AddrMode::PreIndex)
  }

  #[inline]
  pub const fn is_post_index(&self) -> bool {
    matches!(self.addrmode, AddrMode::PostIndex)
  }
}
