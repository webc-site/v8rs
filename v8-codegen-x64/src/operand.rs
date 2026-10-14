use std::mem::size_of;

use v8_codegen_core::RelocMode;

use crate::registers::{RBP, RSP, Register};

/// SIB 比例因子 (ScaleFactor 1, 2, 4, 8)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ScaleFactor {
  Times1 = 0,
  Times2 = 1,
  Times4 = 2,
  Times8 = 3,
}

impl ScaleFactor {
  #[inline]
  pub const fn value(self) -> u8 {
    self as u8
  }

  #[inline]
  pub const fn scale(self) -> u8 {
    match self {
      Self::Times1 => 1,
      Self::Times2 => 2,
      Self::Times4 => 4,
      Self::Times8 => 8,
    }
  }
}

// 注: 原版 C++ 侧比例因子别名 (times_1/2/4/8) 为小写 constexpr,
// Rust 侧由 ScaleFactor 枚举变体 (Times1/2/4/8) 直接承接, 小写别名已清障移除。

/// 立即数 (32 位有符号数, 附带 RelocMode)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Immediate {
  pub value: i32,
  pub rmode: RelocMode,
}

impl Immediate {
  #[inline]
  pub const fn new(value: i32) -> Self {
    Self {
      value,
      rmode: RelocMode::NoInfo,
    }
  }

  #[inline]
  pub const fn with_rmode(value: i32, rmode: RelocMode) -> Self {
    Self { value, rmode }
  }

  /// 组合判定 (值域 + rmode==NoInfo): 仅对应原版 immediate_arithmetic_op 32/64 支
  /// `is_int8(src.value_) && RelocInfo::IsNoInfo(src.rmode_)` (assembler-x64.cc:928,946)。
  /// pushq/imul 等值域单查支勿用本方法, 走 [`Self::is_int8_value`]。
  #[inline]
  pub const fn is_int8(self) -> bool {
    self.is_int8_value() && matches!(self.rmode, RelocMode::NoInfo)
  }

  /// 纯值域判定, 镜像原版自由函数 `is_int8(value_)` (基类只查值不查 rmode):
  /// pushq (assembler-x64.cc:2362)、emit_imul rri/rmi (:1548,:1562)。
  /// 原版怪癖照抄: int8 值 + rmode≠NoInfo 时折 imm8 并弃 reloc——imm32 支的
  /// `emit(imm)` 才落 reloc, 折叠支无 reloc 通道。
  #[inline]
  pub const fn is_int8_value(self) -> bool {
    self.value >= -128 && self.value <= 127
  }
}

impl From<i32> for Immediate {
  #[inline]
  fn from(value: i32) -> Self {
    Self::new(value)
  }
}

/// x64 内存寻址操作数
///
/// 内部包含 ModR/M、SIB、Displacement 字节序列 (1..6 字节) 与 REX 扩展标志
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Operand {
  pub rex: u8,
  pub buf: [u8; 6],
  pub len: u8,
}

const _: () = assert!(size_of::<Operand>() == 8);

impl Operand {
  /// [base + disp] 寻址
  #[inline]
  pub const fn base_disp(base: Register, disp: i32) -> Self {
    let mut op = Self {
      rex: 0,
      buf: [0; 6],
      len: 1,
    };

    // rsp / r12 作为基址寄存器时必须使用 SIB 字节
    if base.low_bits() == 0x4 {
      op.set_sib(ScaleFactor::Times1, RSP, base);
    }

    // rbp / r13 作为基址寄存器且偏移为 0 时, 必须编码为 mod=01 伴随 disp8=0
    if disp == 0 && base.low_bits() != 0x5 {
      op.set_modrm(0, base);
    } else if disp >= -128 && disp <= 127 {
      op.set_modrm(1, base);
      op.set_disp8(disp as i8);
    } else {
      op.set_modrm(2, base);
      op.set_disp32(disp);
    }

    op
  }

  /// [base] 快捷构造
  #[inline]
  pub const fn base(base: Register) -> Self {
    Self::base_disp(base, 0)
  }

  /// [base + index * scale + disp] 寻址
  #[inline]
  pub const fn base_index_scale_disp(
    base: Register,
    index: Register,
    scale: ScaleFactor,
    disp: i32,
  ) -> Self {
    debug_assert!(
      index.code() != RSP.code(),
      "RSP cannot be used as SIB index"
    );
    let mut op = Self {
      rex: 0,
      buf: [0; 6],
      len: 1,
    };
    op.set_sib(scale, index, base);
    if disp == 0 && base.low_bits() != 0x5 {
      op.set_modrm(0, RSP);
    } else if disp >= -128 && disp <= 127 {
      op.set_modrm(1, RSP);
      op.set_disp8(disp as i8);
    } else {
      op.set_modrm(2, RSP);
      op.set_disp32(disp);
    }
    op
  }

  /// [index * scale + disp] (无基址寄存器, mod=00, SIB base=rbp/5, disp32)
  #[inline]
  pub const fn index_scale_disp(index: Register, scale: ScaleFactor, disp: i32) -> Self {
    // 原版承接: 此构造编码比 {base,disp} 版长, 仅当 scale>1 才该用 (assembler-x64.h:284-290)
    debug_assert!(
      !matches!(scale, ScaleFactor::Times1),
      "index_scale_disp requires scale > times_1"
    );
    debug_assert!(
      index.code() != RSP.code(),
      "RSP cannot be used as SIB index"
    );
    let mut op = Self {
      rex: 0,
      buf: [0; 6],
      len: 1,
    };
    op.set_modrm(0, RSP);
    op.set_sib(scale, index, RBP);
    op.set_disp32(disp);
    op
  }

  /// RIP 相对寻址 (mod=00, rm=101, disp32)
  #[inline]
  pub const fn rip_disp32(disp: i32) -> Self {
    let mut op = Self {
      rex: 0,
      buf: [0; 6],
      len: 1,
    };
    op.buf[0] = 0x05;
    op.set_disp32(disp);
    op
  }

  #[inline]
  const fn set_modrm(&mut self, mod_bits: u8, rm_reg: Register) {
    self.buf[0] = (mod_bits << 6) | rm_reg.low_bits();
    self.rex |= rm_reg.high_bit();
  }

  #[inline]
  const fn set_sib(&mut self, scale: ScaleFactor, index: Register, base: Register) {
    self.buf[1] = (scale.value() << 6) | (index.low_bits() << 3) | base.low_bits();
    self.rex |= (index.high_bit() << 1) | base.high_bit();
    self.len = 2;
  }

  #[inline]
  const fn set_disp8(&mut self, disp: i8) {
    self.buf[self.len as usize] = disp as u8;
    self.len += 1;
  }

  #[inline]
  const fn set_disp32(&mut self, disp: i32) {
    let bytes = disp.to_le_bytes();
    let idx = self.len as usize;
    self.buf[idx] = bytes[0];
    self.buf[idx + 1] = bytes[1];
    self.buf[idx + 2] = bytes[2];
    self.buf[idx + 3] = bytes[3];
    self.len += 4;
  }
}
