use crate::{
  constants::{
    IMM_ADD_SUB_OFFSET, IMM_COND_BRANCH_OFFSET, IMM_DP_SHIFT_OFFSET, IMM_EXTEND_SHIFT_OFFSET,
    IMM_LS_OFFSET, IMM_LS_UNSIGNED_OFFSET, IMM_UNCOND_BRANCH_OFFSET, RD_OFFSET, RM_OFFSET,
    RN_OFFSET, RT_OFFSET, SHIFT_ADD_SUB_OFFSET, SHIFT_DP_OFFSET, SIXTY_FOUR_BITS, THIRTY_TWO_BITS,
  },
  operand::{Extend, Shift},
  registers::{Condition, Register, VRegister},
};

/// 32 位定长 ARM64 指令包装
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Instruction(pub u32);

impl Instruction {
  #[inline]
  pub const fn new(instr: u32) -> Self {
    Self(instr)
  }

  #[inline]
  pub const fn bits(self) -> u32 {
    self.0
  }

  #[inline]
  pub const fn to_le_bytes(self) -> [u8; 4] {
    self.0.to_le_bytes()
  }

  #[inline]
  pub const fn from_le_bytes(bytes: [u8; 4]) -> Self {
    Self(u32::from_le_bytes(bytes))
  }

  #[inline]
  pub const fn mask(self, mask: u32) -> u32 {
    self.0 & mask
  }

  /// 提取无条件跳转 26 位有符号字偏移
  #[inline]
  pub const fn imm_uncond_branch(self) -> i32 {
    let raw = self.0 & 0x03FF_FFFF;
    ((raw as i32) << 6) >> 6
  }

  /// 提取条件跳转 19 位有符号字偏移
  #[inline]
  pub const fn imm_cond_branch(self) -> i32 {
    let raw = (self.0 >> IMM_COND_BRANCH_OFFSET) & 0x0007_FFFF;
    ((raw as i32) << 13) >> 13
  }

  /// 是否为 B / BL 无条件跳转指令
  #[inline]
  pub const fn is_uncond_branch(self) -> bool {
    (self.0 & 0x7C00_0000) == B
  }

  /// 是否为 B.cond 条件跳转指令
  #[inline]
  pub const fn is_cond_branch(self) -> bool {
    (self.0 & 0xFF00_0010) == B_COND
  }

  #[inline]
  pub const fn bit(self, pos: u32) -> u32 {
    (self.0 >> pos) & 1
  }

  #[inline]
  pub const fn bit_range(self, msb: u32, lsb: u32) -> u32 {
    (self.0 >> lsb) & (u32::MAX >> (31 - (msb - lsb)))
  }

  #[inline]
  pub const fn signed_bit_range(self, msb: u32, lsb: u32) -> i32 {
    let width = msb - lsb + 1;
    let raw = (self.0 >> lsb) & (u32::MAX >> (32 - width));
    ((raw as i32) << (32 - width)) >> (32 - width)
  }

  #[inline]
  pub const fn rd(self) -> u32 {
    self.bit_range(4, 0)
  }

  #[inline]
  pub const fn rn(self) -> u32 {
    self.bit_range(9, 5)
  }

  #[inline]
  pub const fn rm(self) -> u32 {
    self.bit_range(20, 16)
  }

  #[inline]
  pub const fn ra(self) -> u32 {
    self.bit_range(14, 10)
  }

  #[inline]
  pub const fn rt(self) -> u32 {
    self.bit_range(4, 0)
  }

  #[inline]
  pub const fn rt2(self) -> u32 {
    self.bit_range(14, 10)
  }

  #[inline]
  pub const fn rs(self) -> u32 {
    self.bit_range(20, 16)
  }

  #[inline]
  pub const fn sixty_four_bits(self) -> bool {
    self.bit(31) == 1
  }

  #[inline]
  pub const fn flags_update(self) -> bool {
    self.bit(29) == 1
  }

  #[inline]
  pub const fn imm_pc_rel(self) -> i32 {
    let hi = self.bit_range(23, 5);
    let lo = self.bit_range(30, 29);
    let raw = (hi << 2) | lo;
    ((raw as i32) << 11) >> 11
  }

  #[inline]
  pub const fn shift_dp(self) -> u32 {
    self.bit_range(23, 22)
  }

  #[inline]
  pub const fn imm_dp_shift(self) -> u32 {
    self.bit_range(15, 10)
  }

  #[inline]
  pub const fn imm_add_sub(self) -> u32 {
    self.bit_range(21, 10)
  }

  #[inline]
  pub const fn shift_add_sub(self) -> bool {
    self.bit(22) == 1
  }

  #[inline]
  pub const fn imm_extend_shift(self) -> u32 {
    self.bit_range(12, 10)
  }

  #[inline]
  pub const fn extend_mode(self) -> u32 {
    self.bit_range(15, 13)
  }

  #[inline]
  pub const fn imm_move_wide(self) -> u32 {
    self.bit_range(20, 5)
  }

  #[inline]
  pub const fn shift_move_wide(self) -> u32 {
    self.bit_range(22, 21)
  }

  #[inline]
  pub const fn bit_n(self) -> u32 {
    self.bit(22)
  }

  #[inline]
  pub const fn imm_rotate(self) -> u32 {
    self.bit_range(21, 16)
  }

  #[inline]
  pub const fn imm_set_bits(self) -> u32 {
    self.bit_range(15, 10)
  }

  #[inline]
  pub const fn imm_r(self) -> u32 {
    self.bit_range(21, 16)
  }

  #[inline]
  pub const fn imm_s(self) -> u32 {
    self.bit_range(15, 10)
  }

  #[inline]
  pub const fn condition(self) -> u32 {
    self.bit_range(15, 12)
  }

  #[inline]
  pub const fn condition_branch(self) -> u32 {
    self.bit_range(3, 0)
  }

  #[inline]
  pub const fn nzcv(self) -> u32 {
    self.bit_range(3, 0)
  }

  #[inline]
  pub const fn imm_fp(self) -> u32 {
    self.bit_range(20, 13)
  }

  #[inline]
  pub const fn imm_fp32(self) -> f32 {
    imm8_to_fp32(self.imm_fp())
  }

  #[inline]
  pub const fn imm_fp64(self) -> f64 {
    imm8_to_fp64(self.imm_fp())
  }

  #[inline]
  pub const fn imm_ls(self) -> i32 {
    self.signed_bit_range(20, 12)
  }

  #[inline]
  pub const fn imm_ls_unsigned(self) -> u32 {
    self.bit_range(21, 10)
  }

  #[inline]
  pub const fn imm_ls_pair(self) -> i32 {
    self.signed_bit_range(21, 15)
  }

  pub fn imm_logical(self) -> u64 {
    let reg_size = if self.sixty_four_bits() { 64 } else { 32 };
    let n = self.bit_n();
    let imm_s = self.imm_s();
    let imm_r = self.imm_r();

    if n == 1 {
      if imm_s == 0x3F {
        return 0;
      }
      let bits = (1u64 << (imm_s + 1)) - 1;
      rotate_right(bits, imm_r, 64)
    } else {
      if (imm_s >> 1) == 0x1F {
        return 0;
      }
      let mut width = 0x20u32;
      while width >= 0x2 {
        if (imm_s & width) == 0 {
          let mask = width - 1;
          if (imm_s & mask) == mask {
            return 0;
          }
          let bits = (1u64 << ((imm_s & mask) + 1)) - 1;
          return repeat_bits_across_reg(reg_size, rotate_right(bits, imm_r & mask, width), width);
        }
        width >>= 1;
      }
      0
    }
  }
}

#[inline]
pub const fn rotate_right(value: u64, rotate: u32, width: u32) -> u64 {
  let rot = rotate & 63;
  if rot == 0 {
    value
  } else {
    let mask = (1u64 << rot) - 1;
    ((value & mask) << (width - rot)) | (value >> rot)
  }
}

pub const fn repeat_bits_across_reg(reg_size: u32, value: u64, width: u32) -> u64 {
  let mut result = value & ((1u64 << width) - 1);
  let mut i = width;
  while i < reg_size {
    result |= result << i;
    i *= 2;
  }
  result
}

#[inline]
pub const fn imm8_to_fp32(imm8: u32) -> f32 {
  let bit7 = (imm8 >> 7) & 1;
  let bit6 = (imm8 >> 6) & 1;
  let bit5_to_0 = imm8 & 0x3f;
  let result = (bit7 << 31) | ((32 - bit6) << 25) | (bit5_to_0 << 19);
  f32::from_bits(result)
}

#[inline]
pub const fn imm8_to_fp64(imm8: u32) -> f64 {
  let bit7 = (imm8 as u64 >> 7) & 1;
  let bit6 = (imm8 as u64 >> 6) & 1;
  let bit5_to_0 = imm8 as u64 & 0x3f;
  let result = (bit7 << 63) | ((256 - bit6) << 54) | (bit5_to_0 << 48);
  f64::from_bits(result)
}

#[inline]
pub const fn is_imm_fp32(bits: u32) -> bool {
  if (bits & 0x7FFFF) != 0 {
    return false;
  }
  let b_pattern = (bits >> 16) & 0x3E00;
  if b_pattern != 0 && b_pattern != 0x3E00 {
    return false;
  }
  if ((bits ^ (bits << 1)) & 0x4000_0000) == 0 {
    return false;
  }
  true
}

#[inline]
pub const fn is_imm_fp64(bits: u64) -> bool {
  if (bits & 0xFFFF_FFFF_FFFF) != 0 {
    return false;
  }
  let b_pattern = (bits >> 48) & 0x3FC0;
  if b_pattern != 0 && b_pattern != 0x3FC0 {
    return false;
  }
  if ((bits ^ (bits << 1)) & 0x4000_0000_0000_0000) == 0 {
    return false;
  }
  true
}

#[inline]
pub const fn fp_to_imm8(imm: f64) -> u32 {
  let bits = imm.to_bits();
  debug_assert!(is_imm_fp64(bits));
  let bit7 = ((bits >> 63) & 1) << 7;
  let bit6 = ((bits >> 61) & 1) << 6;
  let bit5_to_0 = (bits >> 48) & 0x3F;
  (bit7 | bit6 | bit5_to_0) as u32
}

// ── 基础指令操作码 (常量面) ──

// 跳转类
pub const B: u32 = 0x1400_0000;
pub const BL: u32 = 0x9400_0000;
pub const B_COND: u32 = 0x5400_0000;
pub const BR: u32 = 0xD61F_0000;
pub const BLR: u32 = 0xD63F_0000;
pub const RET: u32 = 0xD65F_0000;

// 系统指令 (HINT / NOP)
pub const HINT: u32 = 0xD503_201F;
pub const NOP: u32 = 0xD503_201F;

// 算术类操作码
pub const ADD_SHIFTED: u32 = 0x0B00_0000;
pub const ADDS_SHIFTED: u32 = 0x2B00_0000;
pub const SUB_SHIFTED: u32 = 0x4B00_0000;
pub const SUBS_SHIFTED: u32 = 0x6B00_0000;

pub const ADD_IMM: u32 = 0x1100_0000;
pub const ADDS_IMM: u32 = 0x3100_0000;
pub const SUB_IMM: u32 = 0x5100_0000;
pub const SUBS_IMM: u32 = 0x7100_0000;

pub const ADD_EXT: u32 = 0x0B20_0000;
pub const SUB_EXT: u32 = 0x4B20_0000;
// C++ 侧无独立 ADDS_EXT/SUBS_EXT 常量——原版由 AddSubExtendedFixed|op 再叠
// Flags(S) (bit29) 组合 (assembler-arm64.cc:4361-4368 DataProcExtendedRegister);
// Rust 沿用本文件显式常量风格预折叠 set_flags 位 (r7 D1)
pub const ADDS_EXT: u32 = 0x2B20_0000;
pub const SUBS_EXT: u32 = 0x6B20_0000;

// 逻辑类操作码
pub const LOGICAL_SHIFTED_FIXED: u32 = 0x0A00_0000;
pub const LOGICAL_IMM_FIXED: u32 = 0x1200_0000;

pub const AND: u32 = 0x0000_0000;
pub const BIC: u32 = 0x0020_0000;
pub const ORR: u32 = 0x2000_0000;
pub const ORN: u32 = 0x2020_0000;
pub const EOR: u32 = 0x4000_0000;
pub const EON: u32 = 0x4020_0000;
pub const ANDS: u32 = 0x6000_0000;

// 访存类 (无符号缩放与无缩放立即数 / 寄存器偏移)
pub const LDR_W_UNSIGNED: u32 = 0xB940_0000;
pub const LDR_X_UNSIGNED: u32 = 0xF940_0000;
pub const STR_W_UNSIGNED: u32 = 0xB900_0000;
pub const STR_X_UNSIGNED: u32 = 0xF900_0000;

pub const LDUR_W: u32 = 0xB840_0000;
pub const LDUR_X: u32 = 0xF840_0000;
pub const STUR_W: u32 = 0xB800_0000;
pub const STUR_X: u32 = 0xF800_0000;

pub const LDR_W_REG: u32 = 0xB860_0800;
pub const LDR_X_REG: u32 = 0xF860_0800;
pub const STR_W_REG: u32 = 0xB820_0800;
pub const STR_X_REG: u32 = 0xF820_0800;

// 浮点与 SIMD 访存 (无符号缩放、无缩放立即数与寄存器偏移)
pub const LDR_B_UNSIGNED: u32 = 0x3D40_0000;
pub const LDR_H_UNSIGNED: u32 = 0x7D40_0000;
pub const LDR_S_UNSIGNED: u32 = 0xBD40_0000;
pub const LDR_D_UNSIGNED: u32 = 0xFD40_0000;
pub const LDR_Q_UNSIGNED: u32 = 0x3DC0_0000;
pub const STR_B_UNSIGNED: u32 = 0x3D00_0000;
pub const STR_H_UNSIGNED: u32 = 0x7D00_0000;
pub const STR_S_UNSIGNED: u32 = 0xBD00_0000;
pub const STR_D_UNSIGNED: u32 = 0xFD00_0000;
pub const STR_Q_UNSIGNED: u32 = 0x3D80_0000;

pub const LDUR_B: u32 = 0x3C40_0000;
pub const LDUR_H: u32 = 0x7C40_0000;
pub const LDUR_S: u32 = 0xBC40_0000;
pub const LDUR_D: u32 = 0xFC40_0000;
pub const LDUR_Q: u32 = 0x3CC0_0000;
pub const STUR_B: u32 = 0x3C00_0000;
pub const STUR_H: u32 = 0x7C00_0000;
pub const STUR_S: u32 = 0xBC00_0000;
pub const STUR_D: u32 = 0xFC00_0000;
pub const STUR_Q: u32 = 0x3C80_0000;

pub const LDR_B_REG: u32 = 0x3C60_0800;
pub const LDR_H_REG: u32 = 0x7C60_0800;
pub const LDR_S_REG: u32 = 0xBC60_0800;
pub const LDR_D_REG: u32 = 0xFC60_0800;
pub const LDR_Q_REG: u32 = 0x3CE0_0800;
pub const STR_B_REG: u32 = 0x3C20_0800;
pub const STR_H_REG: u32 = 0x7C20_0800;
pub const STR_S_REG: u32 = 0xBC20_0800;
pub const STR_D_REG: u32 = 0xFC20_0800;
pub const STR_Q_REG: u32 = 0x3CA0_0800;

// ── 字段编码函数 ──

#[inline]
pub const fn encode_rd(reg: Register) -> u32 {
  ((reg.code() as u32) & 0x1F) << RD_OFFSET
}

#[inline]
pub const fn encode_rn(reg: Register) -> u32 {
  ((reg.code() as u32) & 0x1F) << RN_OFFSET
}

#[inline]
pub const fn encode_rm(reg: Register) -> u32 {
  ((reg.code() as u32) & 0x1F) << RM_OFFSET
}

#[inline]
pub const fn encode_rt(reg: Register) -> u32 {
  ((reg.code() as u32) & 0x1F) << RT_OFFSET
}

#[inline]
pub const fn encode_v_rt(reg: VRegister) -> u32 {
  ((reg.code() as u32) & 0x1F) << RT_OFFSET
}

#[inline]
pub const fn encode_sf(reg: Register) -> u32 {
  if reg.is_64_bits() {
    SIXTY_FOUR_BITS
  } else {
    THIRTY_TWO_BITS
  }
}

#[inline]
pub const fn encode_imm_uncond_branch(imm26: i32) -> u32 {
  ((imm26 as u32) & 0x03FF_FFFF) << IMM_UNCOND_BRANCH_OFFSET
}

#[inline]
pub const fn encode_imm_cond_branch(imm19: i32) -> u32 {
  ((imm19 as u32) & 0x0007_FFFF) << IMM_COND_BRANCH_OFFSET
}

#[inline]
pub const fn encode_condition(cond: Condition) -> u32 {
  cond.code() & 0xF
}

#[inline]
pub const fn encode_shift_dp(shift: Shift) -> u32 {
  ((shift as u32) & 0x3) << SHIFT_DP_OFFSET
}

#[inline]
pub const fn encode_imm_dp_shift(amount: u32) -> u32 {
  (amount & 0x3F) << IMM_DP_SHIFT_OFFSET
}

#[inline]
pub const fn encode_extend_mode(extend: Extend) -> u32 {
  ((extend as u32) & 0x7) << 13
}

#[inline]
pub const fn encode_imm_extend_shift(amount: u32) -> u32 {
  (amount & 0x7) << IMM_EXTEND_SHIFT_OFFSET
}

#[inline]
pub const fn encode_imm_add_sub(imm12: u32, shift12: bool) -> u32 {
  ((imm12 & 0xFFF) << IMM_ADD_SUB_OFFSET)
    | if shift12 {
      1 << SHIFT_ADD_SUB_OFFSET
    } else {
      0
    }
}

#[inline]
pub const fn encode_imm_ls_unsigned(imm12: u32) -> u32 {
  (imm12 & 0xFFF) << IMM_LS_UNSIGNED_OFFSET
}

#[inline]
pub const fn encode_imm_ls_unscaled(offset: i32) -> u32 {
  ((offset as u32) & 0x1FF) << IMM_LS_OFFSET
}
