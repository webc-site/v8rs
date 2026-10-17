use v8_codegen_core::{AssemblerBase, AssemblerOptions, Distance, Label};

use crate::{
  error::{Error, Result},
  instructions::{
    ADD_EXT, ADD_IMM, ADD_SHIFTED, ADDS_EXT, ADDS_IMM, ADDS_SHIFTED, AND, B, B_COND, BL, BLR, BR,
    EOR, Instruction, LDR_B_REG, LDR_B_UNSIGNED, LDR_D_REG, LDR_D_UNSIGNED, LDR_H_REG,
    LDR_H_UNSIGNED, LDR_Q_REG, LDR_Q_UNSIGNED, LDR_S_REG, LDR_S_UNSIGNED, LDR_W_REG,
    LDR_W_UNSIGNED, LDR_X_REG, LDR_X_UNSIGNED, LDUR_B, LDUR_D, LDUR_H, LDUR_Q, LDUR_S, LDUR_W,
    LDUR_X, LOGICAL_SHIFTED_FIXED, NOP, ORR, RET, STR_B_REG, STR_B_UNSIGNED, STR_D_REG,
    STR_D_UNSIGNED, STR_H_REG, STR_H_UNSIGNED, STR_Q_REG, STR_Q_UNSIGNED, STR_S_REG,
    STR_S_UNSIGNED, STR_W_REG, STR_W_UNSIGNED, STR_X_REG, STR_X_UNSIGNED, STUR_B, STUR_D, STUR_H,
    STUR_Q, STUR_S, STUR_W, STUR_X, SUB_EXT, SUB_IMM, SUB_SHIFTED, SUBS_EXT, SUBS_IMM,
    SUBS_SHIFTED, encode_condition, encode_extend_mode, encode_imm_add_sub, encode_imm_cond_branch,
    encode_imm_dp_shift, encode_imm_extend_shift, encode_imm_ls_unscaled, encode_imm_ls_unsigned,
    encode_imm_uncond_branch, encode_rd, encode_rm, encode_rn, encode_rt, encode_sf,
    encode_shift_dp, encode_v_rt,
  },
  operand::{Extend, MemOperand, Operand, Shift},
  registers::{Condition, LR, Register, VRegister, WZR, XZR},
};

#[inline]
const fn zero_reg_for(r: Register) -> Register {
  if r.is_64_bits() { XZR } else { WZR }
}

/// ARM64 架构汇编器 (对应原版 v8::internal::Assembler)
#[derive(Debug)]
pub struct Assembler {
  base: AssemblerBase,
}

impl Assembler {
  #[inline]
  pub fn new(options: AssemblerOptions) -> Self {
    Self {
      base: AssemblerBase::new(options),
    }
  }

  #[inline]
  pub fn pc_offset(&self) -> usize {
    self.base.pc_offset()
  }

  #[inline]
  pub fn buffer_slice(&self) -> &[u8] {
    self.base.buffer_slice()
  }

  #[inline]
  pub fn take_buffer(self) -> Vec<u8> {
    self.base.take_buffer()
  }

  #[inline]
  pub fn emit_u32(&mut self, val: u32) -> Result<()> {
    self.base.emit_u32(val)?;
    Ok(())
  }

  #[inline]
  pub fn emit_instr(&mut self, instr: Instruction) -> Result<()> {
    self.emit_u32(instr.bits())
  }

  // ── 基础数据传输与算术指令 ──

  /// 寄存器移动 (mov rd, rm)
  /// - 涉及 SP 时编码为 add rd, rm, #0
  /// - 否则编码为 orr rd, xzr/wzr, rm
  pub fn mov(&mut self, rd: Register, rm: Register) -> Result<()> {
    if !rd.is_valid() || !rm.is_valid() {
      return Err(Error::InvalidRegister("mov requires valid registers"));
    }
    if rd.size_in_bits() != rm.size_in_bits() {
      return Err(Error::InvalidRegister(
        "mov registers must be of the same size",
      ));
    }
    if rd.is_sp() || rm.is_sp() {
      self.add(rd, rm, 0)
    } else {
      self.orr(rd, zero_reg_for(rd), rm)
    }
  }

  /// 加法指令 (add rd, rn, operand)
  pub fn add(&mut self, rd: Register, rn: Register, operand: impl Into<Operand>) -> Result<()> {
    self.add_sub_internal(rd, rn, operand.into(), false, false)
  }

  /// 设定标志位加法 (adds rd, rn, operand)
  pub fn adds(&mut self, rd: Register, rn: Register, operand: impl Into<Operand>) -> Result<()> {
    self.add_sub_internal(rd, rn, operand.into(), false, true)
  }

  /// 减法指令 (sub rd, rn, operand)
  pub fn sub(&mut self, rd: Register, rn: Register, operand: impl Into<Operand>) -> Result<()> {
    self.add_sub_internal(rd, rn, operand.into(), true, false)
  }

  /// 设定标志位减法 (subs rd, rn, operand)
  pub fn subs(&mut self, rd: Register, rn: Register, operand: impl Into<Operand>) -> Result<()> {
    self.add_sub_internal(rd, rn, operand.into(), true, true)
  }

  /// 比较指令 (cmp rn, operand) -> 别名为 subs xzr/wzr, rn, operand
  pub fn cmp(&mut self, rn: Register, operand: impl Into<Operand>) -> Result<()> {
    self.subs(zero_reg_for(rn), rn, operand)
  }

  fn add_sub_internal(
    &mut self,
    rd: Register,
    rn: Register,
    op: Operand,
    is_sub: bool,
    set_flags: bool,
  ) -> Result<()> {
    if !rd.is_valid() || !rn.is_valid() {
      return Err(Error::InvalidRegister("add/sub requires valid registers"));
    }
    if rd.is_64_bits() != rn.is_64_bits() {
      return Err(Error::InvalidRegister(
        "mismatched register size in add/sub",
      ));
    }
    // 承接原版三处 Rd() 系 DCHECK (assembler-arm64.h:2818-2821 DCHECK_NE code
    // kSPRegInternalCode): imm 支 cc:4046、shifted→SP 转换支 cc:4062、
    // extended 支 cc:4366 均 S?Rd:RdSP——set_flags 形式 Rd 域不得为 SP
    if set_flags && rd.is_sp() {
      return Err(Error::InvalidRegister(
        "cannot set flags with SP destination",
      ));
    }
    let sf = encode_sf(rd);
    match op {
      // 纯寄存器 == shifted 形式 LSL #0 (原版 Operand 缺省移位)
      op @ (Operand::Reg(_) | Operand::Shifted(..)) => {
        self.add_sub_shifted_or_extended(rd, rn, op, is_sub, set_flags)
      }
      op @ Operand::Extended(..) => self.emit_add_sub_extended(rd, rn, op, is_sub, set_flags),
      Operand::Imm(val) => {
        let (imm_val, shift) = if (0..4096).contains(&val) {
          (val as u32, false)
        } else if (val & 0xFFF) == 0 && (val >> 12 >= 0) && (val >> 12 < 4096) {
          ((val >> 12) as u32, true)
        } else {
          return Err(Error::ImmediateOutOfRange(val));
        };
        let opcode = match (is_sub, set_flags) {
          (false, false) => ADD_IMM,
          (false, true) => ADDS_IMM,
          (true, false) => SUB_IMM,
          (true, true) => SUBS_IMM,
        };
        self.emit_u32(
          sf | opcode | encode_rd(rd) | encode_rn(rn) | encode_imm_add_sub(imm_val, shift),
        )
      }
    }
  }

  /// shifted 支统一发射; rd/rn 任一为 SP 时按原版转 extended 形式
  /// (assembler-arm64.cc:4056-4067: "convert the operand from shifted to
  /// extended register mode"; 转换约束对 Operand::ToExtendedRegister——
  /// 仅 LSL 且 amount≤4, extend 按 rm 宽取 UXTX/UXTW, assembler-arm64-inl.h:337-341)
  fn add_sub_shifted_or_extended(
    &mut self,
    rd: Register,
    rn: Register,
    operand: Operand,
    is_sub: bool,
    set_flags: bool,
  ) -> Result<()> {
    let (rm, shift, amount) = match operand {
      Operand::Shifted(rm, shift, amount) => (rm, shift, amount),
      Operand::Reg(rm) => (rm, Shift::LSL, 0),
      _ => {
        return Err(Error::InvalidRegister(
          "internal: only Reg/Shifted operands route to shifted path",
        ));
      }
    };
    if !rm.is_valid() {
      return Err(Error::InvalidRegister("operand register must be valid"));
    }
    if rm.is_64_bits() != rd.is_64_bits() {
      return Err(Error::InvalidRegister(
        "operand register must match destination size",
      ));
    }
    // 原版 shifted 支拒 ROR (assembler-arm64.cc:4052 DCHECK_NE(operand.shift(), ROR))
    if matches!(shift, Shift::ROR) {
      return Err(Error::InvalidRegister(
        "ROR shift is not allowed for add/sub",
      ));
    }
    let max_shift = if rd.is_64_bits() { 63 } else { 31 };
    if amount > max_shift {
      return Err(Error::ImmediateOutOfRange(amount as i64));
    }
    if rd.is_sp() || rn.is_sp() {
      if !matches!(shift, Shift::LSL) || amount > 4 {
        return Err(Error::InvalidRegister(
          "only LSL #0-4 converts to extended form for SP add/sub",
        ));
      }
      let extend = if rm.is_64_bits() {
        Extend::UXTX
      } else {
        Extend::UXTW
      };
      return self.emit_add_sub_extended(
        rd,
        rn,
        Operand::Extended(rm, extend, amount),
        is_sub,
        set_flags,
      );
    }
    let sf = encode_sf(rd);
    let opcode = match (is_sub, set_flags) {
      (false, false) => ADD_SHIFTED,
      (false, true) => ADDS_SHIFTED,
      (true, false) => SUB_SHIFTED,
      (true, true) => SUBS_SHIFTED,
    };
    self.emit_u32(
      sf | opcode
        | encode_rd(rd)
        | encode_rn(rn)
        | encode_rm(rm)
        | encode_shift_dp(shift)
        | encode_imm_dp_shift(amount),
    )
  }

  /// extended 形式统一发射 (原版 DataProcExtendedRegister,
  /// assembler-arm64.cc:4361-4368: SF|op|Flags(bit29)|Rm|ExtendMode|
  /// ImmExtendShift|S?Rd:RdSP|RnSP——set_flags 走 ADDS_EXT/SUBS_EXT, r7 D1)
  fn emit_add_sub_extended(
    &mut self,
    rd: Register,
    rn: Register,
    operand: Operand,
    is_sub: bool,
    set_flags: bool,
  ) -> Result<()> {
    let (rm, extend, amount) = match operand {
      Operand::Extended(rm, extend, amount) => (rm, extend, amount),
      _ => {
        return Err(Error::InvalidRegister(
          "internal: only Extended operands route to extended path",
        ));
      }
    };
    if !rm.is_valid() {
      return Err(Error::InvalidRegister("operand register must be valid"));
    }
    if amount > 4 {
      return Err(Error::ImmediateOutOfRange(amount as i64));
    }
    let sf = encode_sf(rd);
    let opcode = match (is_sub, set_flags) {
      (false, false) => ADD_EXT,
      (false, true) => ADDS_EXT,
      (true, false) => SUB_EXT,
      (true, true) => SUBS_EXT,
    };
    self.emit_u32(
      sf | opcode
        | encode_rd(rd)
        | encode_rn(rn)
        | encode_rm(rm)
        | encode_extend_mode(extend)
        | encode_imm_extend_shift(amount),
    )
  }

  // ── 逻辑运算指令 ──

  /// 按位与 (and rd, rn, operand)
  pub fn and_(&mut self, rd: Register, rn: Register, operand: impl Into<Operand>) -> Result<()> {
    self.logical_internal(rd, rn, operand.into(), AND)
  }

  /// 按位或 (orr rd, rn, operand)
  pub fn orr(&mut self, rd: Register, rn: Register, operand: impl Into<Operand>) -> Result<()> {
    self.logical_internal(rd, rn, operand.into(), ORR)
  }

  /// 按位异或 (eor rd, rn, operand)
  pub fn eor(&mut self, rd: Register, rn: Register, operand: impl Into<Operand>) -> Result<()> {
    self.logical_internal(rd, rn, operand.into(), EOR)
  }

  fn logical_internal(
    &mut self,
    rd: Register,
    rn: Register,
    op: Operand,
    logical_op: u32,
  ) -> Result<()> {
    if !rd.is_valid() || !rn.is_valid() {
      return Err(Error::InvalidRegister(
        "logical op requires valid registers",
      ));
    }
    if rd.is_64_bits() != rn.is_64_bits() {
      return Err(Error::InvalidRegister(
        "mismatched register size in logical op",
      ));
    }
    let sf = encode_sf(rd);
    match op {
      Operand::Reg(rm) => {
        if !rm.is_valid() {
          return Err(Error::InvalidRegister("operand register must be valid"));
        }
        if rm.is_64_bits() != rd.is_64_bits() {
          return Err(Error::InvalidRegister(
            "operand register must match destination size",
          ));
        }
        self.emit_u32(
          sf | LOGICAL_SHIFTED_FIXED | logical_op | encode_rd(rd) | encode_rn(rn) | encode_rm(rm),
        )
      }
      Operand::Shifted(rm, shift, amount) => {
        if !rm.is_valid() {
          return Err(Error::InvalidRegister("operand register must be valid"));
        }
        if rm.is_64_bits() != rd.is_64_bits() {
          return Err(Error::InvalidRegister(
            "operand register must match destination size",
          ));
        }
        let max_shift = if rd.is_64_bits() { 63 } else { 31 };
        if amount > max_shift {
          return Err(Error::ImmediateOutOfRange(amount as i64));
        }
        self.emit_u32(
          sf | LOGICAL_SHIFTED_FIXED
            | logical_op
            | encode_rd(rd)
            | encode_rn(rn)
            | encode_rm(rm)
            | encode_shift_dp(shift)
            | encode_imm_dp_shift(amount),
        )
      }
      _ => Err(Error::InvalidRegister("unsupported logical operand type")),
    }
  }

  // ── 内存读写指令 (ldr / str) ──

  /// 通用寄存器加载 (ldr rt, [rn, ...])
  pub fn ldr(&mut self, rt: Register, addr: MemOperand) -> Result<()> {
    if !rt.is_valid() {
      return Err(Error::InvalidRegister("rt must be a valid register"));
    }
    let base = addr.get_base();
    if !base.is_valid() || !base.is_64_bits() {
      return Err(Error::InvalidRegister(
        "base register must be valid 64-bit register",
      ));
    }
    if addr.is_immediate_offset() {
      let offset = addr.get_offset();
      let scale = rt.size_in_bytes() as i64;
      if offset >= 0 && (offset % scale == 0) && (offset / scale < 4096) {
        let op = if rt.is_64_bits() {
          LDR_X_UNSIGNED
        } else {
          LDR_W_UNSIGNED
        };
        self.emit_u32(
          op | encode_rt(rt) | encode_rn(base) | encode_imm_ls_unsigned((offset / scale) as u32),
        )
      } else if (-256..=255).contains(&offset) {
        let op = if rt.is_64_bits() { LDUR_X } else { LDUR_W };
        self.emit_u32(op | encode_rt(rt) | encode_rn(base) | encode_imm_ls_unscaled(offset as i32))
      } else {
        Err(Error::ImmediateOutOfRange(offset))
      }
    } else if addr.is_register_offset() {
      let regoffset = addr
        .get_regoffset()
        .ok_or(Error::InvalidRegister("missing register offset"))?;
      if !regoffset.is_valid() {
        return Err(Error::InvalidRegister("regoffset must be a valid register"));
      }
      let op = if rt.is_64_bits() {
        LDR_X_REG
      } else {
        LDR_W_REG
      };
      let shift_bit = if addr.get_shift_amount() > 0 {
        1 << 12
      } else {
        0
      };
      self.emit_u32(
        op | encode_rt(rt)
          | encode_rn(base)
          | encode_rm(regoffset)
          | encode_extend_mode(addr.get_extend())
          | shift_bit,
      )
    } else {
      Err(Error::InvalidRegister("unsupported address mode for ldr"))
    }
  }

  /// 通用寄存器存储 (str rt, [rn, ...])
  pub fn str(&mut self, rt: Register, addr: MemOperand) -> Result<()> {
    if !rt.is_valid() {
      return Err(Error::InvalidRegister("rt must be a valid register"));
    }
    let base = addr.get_base();
    if !base.is_valid() || !base.is_64_bits() {
      return Err(Error::InvalidRegister(
        "base register must be valid 64-bit register",
      ));
    }
    if addr.is_immediate_offset() {
      let offset = addr.get_offset();
      let scale = rt.size_in_bytes() as i64;
      if offset >= 0 && (offset % scale == 0) && (offset / scale < 4096) {
        let op = if rt.is_64_bits() {
          STR_X_UNSIGNED
        } else {
          STR_W_UNSIGNED
        };
        self.emit_u32(
          op | encode_rt(rt) | encode_rn(base) | encode_imm_ls_unsigned((offset / scale) as u32),
        )
      } else if (-256..=255).contains(&offset) {
        let op = if rt.is_64_bits() { STUR_X } else { STUR_W };
        self.emit_u32(op | encode_rt(rt) | encode_rn(base) | encode_imm_ls_unscaled(offset as i32))
      } else {
        Err(Error::ImmediateOutOfRange(offset))
      }
    } else if addr.is_register_offset() {
      let regoffset = addr
        .get_regoffset()
        .ok_or(Error::InvalidRegister("missing register offset"))?;
      if !regoffset.is_valid() {
        return Err(Error::InvalidRegister("regoffset must be a valid register"));
      }
      let op = if rt.is_64_bits() {
        STR_X_REG
      } else {
        STR_W_REG
      };
      let shift_bit = if addr.get_shift_amount() > 0 {
        1 << 12
      } else {
        0
      };
      self.emit_u32(
        op | encode_rt(rt)
          | encode_rn(base)
          | encode_rm(regoffset)
          | encode_extend_mode(addr.get_extend())
          | shift_bit,
      )
    } else {
      Err(Error::InvalidRegister("unsupported address mode for str"))
    }
  }

  /// 向量/浮点寄存器加载 (ldr vt, [rn, ...])
  pub fn ldr_v(&mut self, vt: VRegister, addr: MemOperand) -> Result<()> {
    if !vt.is_valid() {
      return Err(Error::InvalidRegister("vt must be a valid vregister"));
    }
    let base = addr.get_base();
    if !base.is_valid() || !base.is_64_bits() {
      return Err(Error::InvalidRegister(
        "base register must be valid 64-bit register",
      ));
    }
    if addr.is_immediate_offset() {
      let offset = addr.get_offset();
      let (unsigned_op, unscaled_op, scale) = if vt.is_b() {
        (LDR_B_UNSIGNED, LDUR_B, 1)
      } else if vt.is_h() {
        (LDR_H_UNSIGNED, LDUR_H, 2)
      } else if vt.is_s() {
        (LDR_S_UNSIGNED, LDUR_S, 4)
      } else if vt.is_d() {
        (LDR_D_UNSIGNED, LDUR_D, 8)
      } else if vt.is_q() {
        (LDR_Q_UNSIGNED, LDUR_Q, 16)
      } else {
        return Err(Error::InvalidRegister("unsupported vregister size"));
      };
      if offset >= 0 && (offset % scale == 0) && (offset / scale < 4096) {
        self.emit_u32(
          unsigned_op
            | encode_v_rt(vt)
            | encode_rn(base)
            | encode_imm_ls_unsigned((offset / scale) as u32),
        )
      } else if (-256..=255).contains(&offset) {
        self.emit_u32(
          unscaled_op | encode_v_rt(vt) | encode_rn(base) | encode_imm_ls_unscaled(offset as i32),
        )
      } else {
        Err(Error::ImmediateOutOfRange(offset))
      }
    } else if addr.is_register_offset() {
      let regoffset = addr
        .get_regoffset()
        .ok_or(Error::InvalidRegister("missing register offset"))?;
      if !regoffset.is_valid() {
        return Err(Error::InvalidRegister("regoffset must be a valid register"));
      }
      let op = if vt.is_b() {
        LDR_B_REG
      } else if vt.is_h() {
        LDR_H_REG
      } else if vt.is_s() {
        LDR_S_REG
      } else if vt.is_d() {
        LDR_D_REG
      } else if vt.is_q() {
        LDR_Q_REG
      } else {
        return Err(Error::InvalidRegister("unsupported vregister size"));
      };
      let shift_bit = if addr.get_shift_amount() > 0 {
        1 << 12
      } else {
        0
      };
      self.emit_u32(
        op | encode_v_rt(vt)
          | encode_rn(base)
          | encode_rm(regoffset)
          | encode_extend_mode(addr.get_extend())
          | shift_bit,
      )
    } else {
      Err(Error::InvalidRegister("unsupported address mode for ldr_v"))
    }
  }

  /// 向量/浮点寄存器存储 (str vt, [rn, ...])
  pub fn str_v(&mut self, vt: VRegister, addr: MemOperand) -> Result<()> {
    if !vt.is_valid() {
      return Err(Error::InvalidRegister("vt must be a valid vregister"));
    }
    let base = addr.get_base();
    if !base.is_valid() || !base.is_64_bits() {
      return Err(Error::InvalidRegister(
        "base register must be valid 64-bit register",
      ));
    }
    if addr.is_immediate_offset() {
      let offset = addr.get_offset();
      let (unsigned_op, unscaled_op, scale) = if vt.is_b() {
        (STR_B_UNSIGNED, STUR_B, 1)
      } else if vt.is_h() {
        (STR_H_UNSIGNED, STUR_H, 2)
      } else if vt.is_s() {
        (STR_S_UNSIGNED, STUR_S, 4)
      } else if vt.is_d() {
        (STR_D_UNSIGNED, STUR_D, 8)
      } else if vt.is_q() {
        (STR_Q_UNSIGNED, STUR_Q, 16)
      } else {
        return Err(Error::InvalidRegister("unsupported vregister size"));
      };
      if offset >= 0 && (offset % scale == 0) && (offset / scale < 4096) {
        self.emit_u32(
          unsigned_op
            | encode_v_rt(vt)
            | encode_rn(base)
            | encode_imm_ls_unsigned((offset / scale) as u32),
        )
      } else if (-256..=255).contains(&offset) {
        self.emit_u32(
          unscaled_op | encode_v_rt(vt) | encode_rn(base) | encode_imm_ls_unscaled(offset as i32),
        )
      } else {
        Err(Error::ImmediateOutOfRange(offset))
      }
    } else if addr.is_register_offset() {
      let regoffset = addr
        .get_regoffset()
        .ok_or(Error::InvalidRegister("missing register offset"))?;
      if !regoffset.is_valid() {
        return Err(Error::InvalidRegister("regoffset must be a valid register"));
      }
      let op = if vt.is_b() {
        STR_B_REG
      } else if vt.is_h() {
        STR_H_REG
      } else if vt.is_s() {
        STR_S_REG
      } else if vt.is_d() {
        STR_D_REG
      } else if vt.is_q() {
        STR_Q_REG
      } else {
        return Err(Error::InvalidRegister("unsupported vregister size"));
      };
      let shift_bit = if addr.get_shift_amount() > 0 {
        1 << 12
      } else {
        0
      };
      self.emit_u32(
        op | encode_v_rt(vt)
          | encode_rn(base)
          | encode_rm(regoffset)
          | encode_extend_mode(addr.get_extend())
          | shift_bit,
      )
    } else {
      Err(Error::InvalidRegister("unsupported address mode for str_v"))
    }
  }

  // ── 跳转与控制流指令 ──

  /// 无条件跳转立即数位移 (字偏移 imm26)
  #[inline]
  pub fn b_offset(&mut self, imm26_words: i32) -> Result<()> {
    if !(-33554432..=33554431).contains(&imm26_words) {
      return Err(Error::BranchOffsetOutOfRange(imm26_words.saturating_mul(4)));
    }
    self.emit_u32(B | encode_imm_uncond_branch(imm26_words))
  }

  /// 条件跳转立即数位移 (字偏移 imm19)
  #[inline]
  pub fn b_cond_offset(&mut self, cond: Condition, imm19_words: i32) -> Result<()> {
    if !(-262144..=262143).contains(&imm19_words) {
      return Err(Error::BranchOffsetOutOfRange(imm19_words.saturating_mul(4)));
    }
    self.emit_u32(B_COND | encode_imm_cond_branch(imm19_words) | encode_condition(cond))
  }

  /// 带链接跳转立即数位移 (字偏移 imm26)
  #[inline]
  pub fn bl_offset(&mut self, imm26_words: i32) -> Result<()> {
    if !(-33554432..=33554431).contains(&imm26_words) {
      return Err(Error::BranchOffsetOutOfRange(imm26_words.saturating_mul(4)));
    }
    self.emit_u32(BL | encode_imm_uncond_branch(imm26_words))
  }

  /// 无条件跳转至 Label
  pub fn b(&mut self, label: &mut Label) -> Result<()> {
    let curr_pos = self.pc_offset() as i32;
    if label.is_bound() {
      let target_pos = label.pos()?;
      let disp_bytes = target_pos - curr_pos;
      if disp_bytes % 4 != 0 {
        return Err(Error::MisalignedBranchOffset(disp_bytes));
      }
      let disp_words = disp_bytes / 4;
      if !(-33554432..=33554431).contains(&disp_words) {
        return Err(Error::BranchOffsetOutOfRange(disp_bytes));
      }
      self.emit_u32(B | encode_imm_uncond_branch(disp_words))
    } else {
      let link_offset = if label.is_linked() {
        let prev_pos = label.pos()?;
        let disp_bytes = prev_pos - curr_pos;
        if disp_bytes % 4 != 0 {
          return Err(Error::MisalignedBranchOffset(disp_bytes));
        }
        let disp_words = disp_bytes / 4;
        if !(-33554432..=33554431).contains(&disp_words) {
          return Err(Error::BranchOffsetOutOfRange(disp_bytes));
        }
        disp_words
      } else {
        0
      };
      self.emit_u32(B | encode_imm_uncond_branch(link_offset))?;
      label.link_to(curr_pos, Distance::Far);
      Ok(())
    }
  }

  /// 条件跳转至 Label
  pub fn b_cond(&mut self, cond: Condition, label: &mut Label) -> Result<()> {
    let curr_pos = self.pc_offset() as i32;
    if label.is_bound() {
      let target_pos = label.pos()?;
      let disp_bytes = target_pos - curr_pos;
      if disp_bytes % 4 != 0 {
        return Err(Error::MisalignedBranchOffset(disp_bytes));
      }
      let disp_words = disp_bytes / 4;
      if !(-262144..=262143).contains(&disp_words) {
        return Err(Error::BranchOffsetOutOfRange(disp_bytes));
      }
      self.emit_u32(B_COND | encode_imm_cond_branch(disp_words) | encode_condition(cond))
    } else {
      let link_offset = if label.is_linked() {
        let prev_pos = label.pos()?;
        let disp_bytes = prev_pos - curr_pos;
        if disp_bytes % 4 != 0 {
          return Err(Error::MisalignedBranchOffset(disp_bytes));
        }
        let disp_words = disp_bytes / 4;
        if !(-262144..=262143).contains(&disp_words) {
          return Err(Error::BranchOffsetOutOfRange(disp_bytes));
        }
        disp_words
      } else {
        0
      };
      self.emit_u32(B_COND | encode_imm_cond_branch(link_offset) | encode_condition(cond))?;
      label.link_to(curr_pos, Distance::Far);
      Ok(())
    }
  }

  /// 带链接跳转至 Label
  pub fn bl(&mut self, label: &mut Label) -> Result<()> {
    let curr_pos = self.pc_offset() as i32;
    if label.is_bound() {
      let target_pos = label.pos()?;
      let disp_bytes = target_pos - curr_pos;
      if disp_bytes % 4 != 0 {
        return Err(Error::MisalignedBranchOffset(disp_bytes));
      }
      let disp_words = disp_bytes / 4;
      if !(-33554432..=33554431).contains(&disp_words) {
        return Err(Error::BranchOffsetOutOfRange(disp_bytes));
      }
      self.emit_u32(BL | encode_imm_uncond_branch(disp_words))
    } else {
      let link_offset = if label.is_linked() {
        let prev_pos = label.pos()?;
        let disp_bytes = prev_pos - curr_pos;
        if disp_bytes % 4 != 0 {
          return Err(Error::MisalignedBranchOffset(disp_bytes));
        }
        let disp_words = disp_bytes / 4;
        if !(-33554432..=33554431).contains(&disp_words) {
          return Err(Error::BranchOffsetOutOfRange(disp_bytes));
        }
        disp_words
      } else {
        0
      };
      self.emit_u32(BL | encode_imm_uncond_branch(link_offset))?;
      label.link_to(curr_pos, Distance::Far);
      Ok(())
    }
  }

  /// 间接跳转 (br xn)
  pub fn br(&mut self, rn: Register) -> Result<()> {
    if !rn.is_valid() || !rn.is_64_bits() {
      return Err(Error::InvalidRegister("br requires valid 64-bit register"));
    }
    self.emit_u32(BR | encode_rn(rn))
  }

  /// 间接函数调用 (blr xn)
  pub fn blr(&mut self, rn: Register) -> Result<()> {
    if !rn.is_valid() || !rn.is_64_bits() {
      return Err(Error::InvalidRegister("blr requires valid 64-bit register"));
    }
    self.emit_u32(BLR | encode_rn(rn))
  }

  /// 函数返回 (ret xn，默认 x30 / lr)
  pub fn ret(&mut self, rn: Option<Register>) -> Result<()> {
    let r = rn.unwrap_or(LR);
    if !r.is_valid() || !r.is_64_bits() {
      return Err(Error::InvalidRegister("ret requires valid 64-bit register"));
    }
    self.emit_u32(RET | encode_rn(r))
  }

  /// 空操作 (nop) -> 0x1F2003D5 小端
  pub fn nop(&mut self) -> Result<()> {
    self.emit_u32(NOP)
  }

  // ── 标号绑定与 4 字节对齐字偏移回填 ──

  /// 绑定标号位置并遍历回填前向跳转字位移
  pub fn bind(&mut self, label: &mut Label) -> Result<()> {
    let bind_pos = self.pc_offset() as i32;
    if bind_pos % 4 != 0 {
      return Err(Error::MisalignedLabel(bind_pos));
    }
    if label.is_bound() {
      return Err(Error::LabelAlreadyBound(label.pos()?));
    }

    if label.is_linked() {
      let mut link_pos = label.pos()?;
      loop {
        if link_pos < 0 || link_pos % 4 != 0 || (link_pos as usize) + 4 > self.pc_offset() {
          return Err(Error::MisalignedBranchOffset(link_pos));
        }
        let instr = self.base.read_i32_at(link_pos as usize) as u32;
        let instruction = Instruction::new(instr);
        let (offset_to_prev_words, patched_instr) = if instruction.is_uncond_branch() {
          let offset_words = instruction.imm_uncond_branch();
          let disp_bytes = bind_pos - link_pos;
          if disp_bytes % 4 != 0 {
            return Err(Error::MisalignedBranchOffset(disp_bytes));
          }
          let disp_words = disp_bytes / 4;
          if !(-33554432..=33554431).contains(&disp_words) {
            return Err(Error::BranchOffsetOutOfRange(disp_bytes));
          }
          let patched = (instr & !0x03FF_FFFF) | encode_imm_uncond_branch(disp_words);
          (offset_words, patched)
        } else if instruction.is_cond_branch() {
          let offset_words = instruction.imm_cond_branch();
          let disp_bytes = bind_pos - link_pos;
          if disp_bytes % 4 != 0 {
            return Err(Error::MisalignedBranchOffset(disp_bytes));
          }
          let disp_words = disp_bytes / 4;
          if !(-262144..=262143).contains(&disp_words) {
            return Err(Error::BranchOffsetOutOfRange(disp_bytes));
          }
          let patched = (instr & !0x00FF_FFE0) | encode_imm_cond_branch(disp_words);
          (offset_words, patched)
        } else {
          return Err(Error::MisalignedBranchOffset(link_pos));
        };

        if offset_to_prev_words > 0 {
          return Err(Error::MisalignedBranchOffset(link_pos));
        }

        self
          .base
          .write_i32_at(link_pos as usize, patched_instr as i32);
        if offset_to_prev_words == 0 {
          break;
        }
        link_pos = link_pos.saturating_add(offset_to_prev_words.saturating_mul(4));
      }
    }

    label.bind_to(bind_pos)?;
    Ok(())
  }
}
