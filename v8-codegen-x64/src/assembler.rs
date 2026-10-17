use v8_codegen_core::{
  AssemblerBase, AssemblerOptions, Distance, Error as V8CodegenCoreError, Label,
};

use crate::{
  error::{Error, Result},
  operand::{Immediate, Operand},
  registers::{RAX, Register},
};

/// x64 条件码枚举 (对应原版 v8::internal::Condition)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Condition {
  Overflow = 0,
  NoOverflow = 1,
  Below = 2,
  AboveEqual = 3,
  Equal = 4,
  NotEqual = 5,
  BelowEqual = 6,
  Above = 7,
  Negative = 8,
  Positive = 9,
  ParityEven = 10,
  ParityOdd = 11,
  Less = 12,
  GreaterEqual = 13,
  LessEqual = 14,
  Greater = 15,
}

// 注: 原版 C++ 侧条件别名 (carry/zero/sign 及 not_* 前缀) 为小写 constexpr,
// Rust 侧由 Condition 枚举变体 (Below/Equal/Negative/...) 直接承接, 小写别名已清障移除。

impl Condition {
  #[inline]
  pub const fn negate(self) -> Self {
    match (self as u8) ^ 1 {
      0 => Self::Overflow,
      1 => Self::NoOverflow,
      2 => Self::Below,
      3 => Self::AboveEqual,
      4 => Self::Equal,
      5 => Self::NotEqual,
      6 => Self::BelowEqual,
      7 => Self::Above,
      8 => Self::Negative,
      9 => Self::Positive,
      10 => Self::ParityEven,
      11 => Self::ParityOdd,
      12 => Self::Less,
      13 => Self::GreaterEqual,
      14 => Self::LessEqual,
      _ => Self::Greater,
    }
  }
}

/// x64 汇编器 (对应原版 v8::internal::Assembler)
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

  // --- 基础字节/字/双字发射接口 ---

  #[inline]
  pub fn emit_u8(&mut self, val: u8) -> Result<()> {
    self.base.emit_u8(val)?;
    Ok(())
  }

  #[inline]
  pub fn emit_u16(&mut self, val: u16) -> Result<()> {
    self.base.emit_u16(val)?;
    Ok(())
  }

  #[inline]
  pub fn emit_u32(&mut self, val: u32) -> Result<()> {
    self.base.emit_u32(val)?;
    Ok(())
  }

  #[inline]
  pub fn emit_u64(&mut self, val: u64) -> Result<()> {
    self.base.emit_u64(val)?;
    Ok(())
  }

  #[inline]
  pub fn emit_bytes(&mut self, bytes: &[u8]) -> Result<()> {
    self.base.emit_bytes(bytes)?;
    Ok(())
  }

  // --- REX 前缀与 ModR/M 编码辅助 ---

  #[inline]
  pub fn emit_rex_64(&mut self) -> Result<()> {
    self.emit_u8(0x48)
  }

  #[inline]
  pub fn emit_rex_64_rr(&mut self, reg: Register, rm_reg: Register) -> Result<()> {
    self.emit_u8(0x48 | (reg.high_bit() << 2) | rm_reg.high_bit())
  }

  #[inline]
  pub fn emit_rex_64_r(&mut self, reg: Register) -> Result<()> {
    self.emit_u8(0x48 | reg.high_bit())
  }

  #[inline]
  pub fn emit_rex_64_ro(&mut self, reg: Register, op: Operand) -> Result<()> {
    self.emit_u8(0x48 | (reg.high_bit() << 2) | op.rex)
  }

  #[inline]
  pub fn emit_rex_64_o(&mut self, op: Operand) -> Result<()> {
    self.emit_u8(0x48 | op.rex)
  }

  #[inline]
  pub fn emit_optional_rex_32_rr(&mut self, reg: Register, rm_reg: Register) -> Result<()> {
    let rex_bits = (reg.high_bit() << 2) | rm_reg.high_bit();
    if rex_bits != 0 {
      self.emit_u8(0x40 | rex_bits)?;
    }
    Ok(())
  }

  #[inline]
  pub fn emit_optional_rex_32_r(&mut self, reg: Register) -> Result<()> {
    if reg.high_bit() != 0 {
      self.emit_u8(0x41)?;
    }
    Ok(())
  }

  #[inline]
  pub fn emit_optional_rex_32_ro(&mut self, reg: Register, op: Operand) -> Result<()> {
    let rex_bits = (reg.high_bit() << 2) | op.rex;
    if rex_bits != 0 {
      self.emit_u8(0x40 | rex_bits)?;
    }
    Ok(())
  }

  #[inline]
  pub fn emit_optional_rex_32_o(&mut self, op: Operand) -> Result<()> {
    if op.rex != 0 {
      self.emit_u8(0x40 | op.rex)?;
    }
    Ok(())
  }

  #[inline]
  pub fn emit_modrm(&mut self, reg: Register, rm_reg: Register) -> Result<()> {
    self.emit_u8(0xC0 | (reg.low_bits() << 3) | rm_reg.low_bits())
  }

  #[inline]
  pub fn emit_modrm_code(&mut self, code: u8, rm_reg: Register) -> Result<()> {
    self.emit_u8(0xC0 | ((code & 7) << 3) | rm_reg.low_bits())
  }

  #[inline]
  pub fn emit_operand(&mut self, code: u8, op: Operand) -> Result<()> {
    let first = op.buf[0] | ((code & 7) << 3);
    self.emit_u8(first)?;
    if op.len > 1 {
      self.emit_bytes(&op.buf[1..op.len as usize])?;
    }
    Ok(())
  }

  #[inline]
  fn arithmetic_op_rr(
    &mut self,
    opcode: u8,
    reg: Register,
    rm_reg: Register,
    is_64: bool,
  ) -> Result<()> {
    if is_64 {
      self.emit_rex_64_rr(reg, rm_reg)?;
    } else {
      self.emit_optional_rex_32_rr(reg, rm_reg)?;
    }
    self.emit_u8(opcode)?;
    self.emit_modrm(reg, rm_reg)
  }

  #[inline]
  fn arithmetic_op_ro(
    &mut self,
    opcode: u8,
    reg: Register,
    op: Operand,
    is_64: bool,
  ) -> Result<()> {
    if is_64 {
      self.emit_rex_64_ro(reg, op)?;
    } else {
      self.emit_optional_rex_32_ro(reg, op)?;
    }
    self.emit_u8(opcode)?;
    self.emit_operand(reg.low_bits(), op)
  }

  #[inline]
  fn immediate_arithmetic_op_r(
    &mut self,
    subcode: u8,
    dst: Register,
    src: Immediate,
    is_64: bool,
  ) -> Result<()> {
    if is_64 {
      self.emit_rex_64_r(dst)?;
    } else {
      self.emit_optional_rex_32_r(dst)?;
    }
    if src.is_int8() {
      self.emit_u8(0x83)?;
      self.emit_modrm_code(subcode, dst)?;
      self.emit_u8(src.value as u8)?;
    } else if dst == RAX {
      self.emit_u8(0x05 | (subcode << 3))?;
      self.emit_u32(src.value as u32)?;
    } else {
      self.emit_u8(0x81)?;
      self.emit_modrm_code(subcode, dst)?;
      self.emit_u32(src.value as u32)?;
    }
    Ok(())
  }

  #[inline]
  fn immediate_arithmetic_op_o(
    &mut self,
    subcode: u8,
    dst: Operand,
    src: Immediate,
    is_64: bool,
  ) -> Result<()> {
    if is_64 {
      self.emit_rex_64_o(dst)?;
    } else {
      self.emit_optional_rex_32_o(dst)?;
    }
    if src.is_int8() {
      self.emit_u8(0x83)?;
      self.emit_operand(subcode, dst)?;
      self.emit_u8(src.value as u8)?;
    } else {
      self.emit_u8(0x81)?;
      self.emit_operand(subcode, dst)?;
      self.emit_u32(src.value as u32)?;
    }
    Ok(())
  }

  // --- movq (64 位传输指令) ---

  #[inline]
  pub fn movq_rr(&mut self, dst: Register, src: Register) -> Result<()> {
    self.emit_rex_64_rr(dst, src)?;
    self.emit_u8(0x8B)?;
    self.emit_modrm(dst, src)
  }

  #[inline]
  pub fn movq_mr(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.emit_rex_64_ro(src, dst)?;
    self.emit_u8(0x89)?;
    self.emit_operand(src.low_bits(), dst)
  }

  #[inline]
  pub fn movq_rm(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.emit_rex_64_ro(dst, src)?;
    self.emit_u8(0x8B)?;
    self.emit_operand(dst.low_bits(), src)
  }

  #[inline]
  pub fn movq_ri(&mut self, dst: Register, imm: Immediate) -> Result<()> {
    self.emit_rex_64_r(dst)?;
    self.emit_u8(0xC7)?;
    self.emit_modrm_code(0, dst)?;
    self.emit_u32(imm.value as u32)
  }

  #[inline]
  pub fn movq_mi(&mut self, dst: Operand, imm: Immediate) -> Result<()> {
    self.emit_rex_64_o(dst)?;
    self.emit_u8(0xC7)?;
    self.emit_operand(0, dst)?;
    self.emit_u32(imm.value as u32)
  }

  #[inline]
  pub fn movq_imm64(&mut self, dst: Register, imm: i64) -> Result<()> {
    self.emit_rex_64_r(dst)?;
    self.emit_u8(0xB8 | dst.low_bits())?;
    self.emit_u64(imm as u64)
  }

  // --- movl (32 位传输指令) ---

  #[inline]
  pub fn movl_rr(&mut self, dst: Register, src: Register) -> Result<()> {
    self.emit_optional_rex_32_rr(dst, src)?;
    self.emit_u8(0x8B)?;
    self.emit_modrm(dst, src)
  }

  #[inline]
  pub fn movl_mr(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.emit_optional_rex_32_ro(src, dst)?;
    self.emit_u8(0x89)?;
    self.emit_operand(src.low_bits(), dst)
  }

  #[inline]
  pub fn movl_rm(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.emit_optional_rex_32_ro(dst, src)?;
    self.emit_u8(0x8B)?;
    self.emit_operand(dst.low_bits(), src)
  }

  #[inline]
  pub fn movl_ri(&mut self, dst: Register, imm: Immediate) -> Result<()> {
    self.emit_optional_rex_32_r(dst)?;
    self.emit_u8(0xB8 | dst.low_bits())?;
    self.emit_u32(imm.value as u32)
  }

  #[inline]
  pub fn movl_mi(&mut self, dst: Operand, imm: Immediate) -> Result<()> {
    self.emit_optional_rex_32_o(dst)?;
    self.emit_u8(0xC7)?;
    self.emit_operand(0, dst)?;
    self.emit_u32(imm.value as u32)
  }

  // --- addq ---

  #[inline]
  pub fn addq_rr(&mut self, dst: Register, src: Register) -> Result<()> {
    self.arithmetic_op_rr(0x03, dst, src, true)
  }

  #[inline]
  pub fn addq_rm(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.arithmetic_op_ro(0x03, dst, src, true)
  }

  #[inline]
  pub fn addq_mr(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.arithmetic_op_ro(0x01, src, dst, true)
  }

  #[inline]
  pub fn addq_ri(&mut self, dst: Register, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_r(0x0, dst, imm, true)
  }

  #[inline]
  pub fn addq_mi(&mut self, dst: Operand, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_o(0x0, dst, imm, true)
  }

  // --- subq ---

  #[inline]
  pub fn subq_rr(&mut self, dst: Register, src: Register) -> Result<()> {
    self.arithmetic_op_rr(0x2B, dst, src, true)
  }

  #[inline]
  pub fn subq_rm(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.arithmetic_op_ro(0x2B, dst, src, true)
  }

  #[inline]
  pub fn subq_mr(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.arithmetic_op_ro(0x29, src, dst, true)
  }

  #[inline]
  pub fn subq_ri(&mut self, dst: Register, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_r(0x5, dst, imm, true)
  }

  #[inline]
  pub fn subq_mi(&mut self, dst: Operand, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_o(0x5, dst, imm, true)
  }

  // --- imulq ---

  #[inline]
  pub fn imulq_rr(&mut self, dst: Register, src: Register) -> Result<()> {
    self.emit_rex_64_rr(dst, src)?;
    self.emit_u8(0x0F)?;
    self.emit_u8(0xAF)?;
    self.emit_modrm(dst, src)
  }

  #[inline]
  pub fn imulq_rm(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.emit_rex_64_ro(dst, src)?;
    self.emit_u8(0x0F)?;
    self.emit_u8(0xAF)?;
    self.emit_operand(dst.low_bits(), src)
  }

  #[inline]
  pub fn imulq_rri(&mut self, dst: Register, src: Register, imm: Immediate) -> Result<()> {
    self.emit_rex_64_rr(dst, src)?;
    if imm.is_int8_value() {
      self.emit_u8(0x6B)?;
      self.emit_modrm(dst, src)?;
      self.emit_u8(imm.value as u8)
    } else {
      self.emit_u8(0x69)?;
      self.emit_modrm(dst, src)?;
      self.emit_u32(imm.value as u32)
    }
  }

  #[inline]
  pub fn imulq_rmi(&mut self, dst: Register, src: Operand, imm: Immediate) -> Result<()> {
    self.emit_rex_64_ro(dst, src)?;
    if imm.is_int8_value() {
      self.emit_u8(0x6B)?;
      self.emit_operand(dst.low_bits(), src)?;
      self.emit_u8(imm.value as u8)
    } else {
      self.emit_u8(0x69)?;
      self.emit_operand(dst.low_bits(), src)?;
      self.emit_u32(imm.value as u32)
    }
  }

  // --- andq ---

  #[inline]
  pub fn andq_rr(&mut self, dst: Register, src: Register) -> Result<()> {
    self.arithmetic_op_rr(0x23, dst, src, true)
  }

  #[inline]
  pub fn andq_rm(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.arithmetic_op_ro(0x23, dst, src, true)
  }

  #[inline]
  pub fn andq_mr(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.arithmetic_op_ro(0x21, src, dst, true)
  }

  #[inline]
  pub fn andq_ri(&mut self, dst: Register, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_r(0x4, dst, imm, true)
  }

  #[inline]
  pub fn andq_mi(&mut self, dst: Operand, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_o(0x4, dst, imm, true)
  }

  // --- orq ---

  #[inline]
  pub fn orq_rr(&mut self, dst: Register, src: Register) -> Result<()> {
    self.arithmetic_op_rr(0x0B, dst, src, true)
  }

  #[inline]
  pub fn orq_rm(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.arithmetic_op_ro(0x0B, dst, src, true)
  }

  #[inline]
  pub fn orq_mr(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.arithmetic_op_ro(0x09, src, dst, true)
  }

  #[inline]
  pub fn orq_ri(&mut self, dst: Register, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_r(0x1, dst, imm, true)
  }

  #[inline]
  pub fn orq_mi(&mut self, dst: Operand, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_o(0x1, dst, imm, true)
  }

  // --- xorq ---

  #[inline]
  pub fn xorq_rr(&mut self, dst: Register, src: Register) -> Result<()> {
    self.arithmetic_op_rr(0x33, dst, src, true)
  }

  #[inline]
  pub fn xorq_rm(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.arithmetic_op_ro(0x33, dst, src, true)
  }

  #[inline]
  pub fn xorq_mr(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.arithmetic_op_ro(0x31, src, dst, true)
  }

  #[inline]
  pub fn xorq_ri(&mut self, dst: Register, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_r(0x6, dst, imm, true)
  }

  #[inline]
  pub fn xorq_mi(&mut self, dst: Operand, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_o(0x6, dst, imm, true)
  }

  // --- cmpq ---

  #[inline]
  pub fn cmpq_rr(&mut self, dst: Register, src: Register) -> Result<()> {
    self.arithmetic_op_rr(0x3B, dst, src, true)
  }

  #[inline]
  pub fn cmpq_rm(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.arithmetic_op_ro(0x3B, dst, src, true)
  }

  #[inline]
  pub fn cmpq_mr(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.arithmetic_op_ro(0x39, src, dst, true)
  }

  #[inline]
  pub fn cmpq_ri(&mut self, dst: Register, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_r(0x7, dst, imm, true)
  }

  #[inline]
  pub fn cmpq_mi(&mut self, dst: Operand, imm: Immediate) -> Result<()> {
    self.immediate_arithmetic_op_o(0x7, dst, imm, true)
  }

  // --- testq ---

  #[inline]
  pub fn testq_rr(&mut self, dst: Register, src: Register) -> Result<()> {
    self.emit_rex_64_rr(dst, src)?;
    self.emit_u8(0x85)?;
    self.emit_modrm(dst, src)
  }

  #[inline]
  pub fn testq_mr(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.emit_rex_64_ro(src, dst)?;
    self.emit_u8(0x85)?;
    self.emit_operand(src.low_bits(), dst)
  }

  #[inline]
  pub fn testq_ri(&mut self, dst: Register, imm: Immediate) -> Result<()> {
    self.emit_rex_64_r(dst)?;
    if dst == RAX {
      self.emit_u8(0xA9)?;
      self.emit_u32(imm.value as u32)
    } else {
      self.emit_u8(0xF7)?;
      self.emit_modrm_code(0, dst)?;
      self.emit_u32(imm.value as u32)
    }
  }

  #[inline]
  pub fn testq_mi(&mut self, dst: Operand, imm: Immediate) -> Result<()> {
    self.emit_rex_64_o(dst)?;
    self.emit_u8(0xF7)?;
    self.emit_operand(0, dst)?;
    self.emit_u32(imm.value as u32)
  }

  // --- 栈操作 ---

  #[inline]
  pub fn pushq_r(&mut self, src: Register) -> Result<()> {
    self.emit_optional_rex_32_r(src)?;
    self.emit_u8(0x50 | src.low_bits())
  }

  #[inline]
  pub fn pushq_m(&mut self, src: Operand) -> Result<()> {
    self.emit_optional_rex_32_o(src)?;
    self.emit_u8(0xFF)?;
    self.emit_operand(6, src)
  }

  #[inline]
  pub fn pushq_i(&mut self, imm: Immediate) -> Result<()> {
    if imm.is_int8_value() {
      self.emit_u8(0x6A)?;
      self.emit_u8(imm.value as u8)
    } else {
      self.emit_u8(0x68)?;
      self.emit_u32(imm.value as u32)
    }
  }

  #[inline]
  pub fn popq_r(&mut self, dst: Register) -> Result<()> {
    self.emit_optional_rex_32_r(dst)?;
    self.emit_u8(0x58 | dst.low_bits())
  }

  #[inline]
  pub fn popq_m(&mut self, dst: Operand) -> Result<()> {
    self.emit_optional_rex_32_o(dst)?;
    self.emit_u8(0x8F)?;
    self.emit_operand(0, dst)
  }

  // --- 控制流与跳转 ---

  #[inline]
  pub fn ret(&mut self) -> Result<()> {
    self.emit_u8(0xC3)
  }

  #[inline]
  pub fn ret_imm(&mut self, imm16: u16) -> Result<()> {
    if imm16 == 0 {
      self.emit_u8(0xC3)
    } else {
      self.emit_u8(0xC2)?;
      self.emit_u16(imm16)
    }
  }

  #[inline]
  pub fn nop(&mut self) -> Result<()> {
    self.emit_u8(0x90)
  }

  #[inline]
  pub fn call_r(&mut self, target: Register) -> Result<()> {
    self.emit_optional_rex_32_r(target)?;
    self.emit_u8(0xFF)?;
    self.emit_modrm_code(2, target)
  }

  #[inline]
  fn emit_far_link(&mut self, label: &mut Label) -> Result<()> {
    if label.is_linked() {
      let pos = label.pos()?;
      self.emit_u32(pos as u32)?;
      label.link_to((self.pc_offset() - 4) as i32, Distance::Far);
    } else {
      let current = self.pc_offset() as i32;
      self.emit_u32(current as u32)?;
      label.link_to(current, Distance::Far);
    }
    Ok(())
  }

  #[inline]
  fn emit_near_link(&mut self, label: &mut Label) -> Result<u8> {
    let disp = if label.is_near_linked() {
      let delta = label.near_link_pos()? - (self.pc_offset() as i32);
      if !(-128..=127).contains(&delta) {
        return Err(Error::NearJumpOutOfRange(delta));
      }
      delta as u8
    } else {
      0
    };
    label.link_to(self.pc_offset() as i32, Distance::Near);
    Ok(disp)
  }

  #[inline]
  pub fn call_m(&mut self, target: Operand) -> Result<()> {
    self.emit_optional_rex_32_o(target)?;
    self.emit_u8(0xFF)?;
    self.emit_operand(2, target)
  }

  #[inline]
  pub fn call_label(&mut self, label: &mut Label) -> Result<()> {
    self.emit_u8(0xE8)?;
    if label.is_bound() {
      let offset = label.pos()? - (self.pc_offset() as i32) - 4;
      self.emit_u32(offset as u32)
    } else {
      self.emit_far_link(label)
    }
  }

  #[inline]
  pub fn jmp_r(&mut self, target: Register) -> Result<()> {
    self.emit_optional_rex_32_r(target)?;
    self.emit_u8(0xFF)?;
    self.emit_modrm_code(4, target)
  }

  #[inline]
  pub fn jmp_m(&mut self, target: Operand) -> Result<()> {
    self.emit_optional_rex_32_o(target)?;
    self.emit_u8(0xFF)?;
    self.emit_operand(4, target)
  }

  #[inline]
  pub fn jmp_rel(&mut self, offset: i32) -> Result<()> {
    let short_disp = offset - 2;
    if (-128..=127).contains(&short_disp) {
      self.emit_u8(0xEB)?;
      self.emit_u8(short_disp as u8)
    } else {
      let near_disp = offset - 5;
      self.emit_u8(0xE9)?;
      self.emit_u32(near_disp as u32)
    }
  }

  #[inline]
  pub fn jmp_label(&mut self, label: &mut Label, distance: Distance) -> Result<()> {
    if label.is_bound() {
      let offset = label.pos()? - (self.pc_offset() as i32);
      self.jmp_rel(offset)
    } else if distance == Distance::Near {
      self.emit_u8(0xEB)?;
      let disp = self.emit_near_link(label)?;
      self.emit_u8(disp)
    } else {
      self.emit_u8(0xE9)?;
      self.emit_far_link(label)
    }
  }

  #[inline]
  pub fn j(&mut self, cc: Condition, label: &mut Label, distance: Distance) -> Result<()> {
    if label.is_bound() {
      let offs = label.pos()? - (self.pc_offset() as i32);
      if (-128..=127).contains(&(offs - 2)) {
        self.emit_u8(0x70 | (cc as u8))?;
        self.emit_u8((offs - 2) as u8)
      } else {
        self.emit_u8(0x0F)?;
        self.emit_u8(0x80 | (cc as u8))?;
        self.emit_u32((offs - 6) as u32)
      }
    } else if distance == Distance::Near {
      self.emit_u8(0x70 | (cc as u8))?;
      let disp = self.emit_near_link(label)?;
      self.emit_u8(disp)
    } else {
      self.emit_u8(0x0F)?;
      self.emit_u8(0x80 | (cc as u8))?;
      self.emit_far_link(label)
    }
  }

  // --- 标号绑定与跳转位移回填 ---

  #[inline]
  pub fn bind(&mut self, label: &mut Label) -> Result<()> {
    let pos = self.pc_offset() as i32;
    if label.is_bound() {
      return Err(V8CodegenCoreError::LabelAlreadyBound(label.pos()?).into());
    }
    // 回填远跳转链表 (32-bit relative displacement)
    if label.is_linked() {
      let mut current = label.pos()?;
      let mut next = self.base.read_i32_at(current as usize);
      while next != current {
        let imm32 = pos - (current + 4);
        self.base.write_i32_at(current as usize, imm32);
        current = next;
        next = self.base.read_i32_at(next as usize);
      }
      let imm32 = pos - (current + 4);
      self.base.write_i32_at(current as usize, imm32);
    }
    // 回填近跳转链表 (8-bit relative displacement)
    while label.is_near_linked() {
      let fixup_pos = label.near_link_pos()? as usize;
      let offset_to_next = self.base.byte_at(fixup_pos) as i8;
      let disp = pos - (fixup_pos as i32 + 1);
      if !(-128..=127).contains(&disp) {
        return Err(Error::NearJumpOutOfRange(disp));
      }
      self.base.set_byte_at(fixup_pos, disp as u8);
      if offset_to_next < 0 {
        label.link_to((fixup_pos as i32) + (offset_to_next as i32), Distance::Near);
      } else {
        label.unuse_near();
      }
    }
    label.bind_to(pos)?;
    Ok(())
  }

  // --- 多态便携 API ---

  #[inline]
  pub fn movq<D, S>(&mut self, dst: D, src: S) -> Result<()>
  where
    Self: MovqOp<D, S>,
  {
    self.emit_movq(dst, src)
  }

  #[inline]
  pub fn movl<D, S>(&mut self, dst: D, src: S) -> Result<()>
  where
    Self: MovlOp<D, S>,
  {
    self.emit_movl(dst, src)
  }

  #[inline]
  pub fn addq<D, S>(&mut self, dst: D, src: S) -> Result<()>
  where
    Self: AddqOp<D, S>,
  {
    self.emit_addq(dst, src)
  }

  #[inline]
  pub fn subq<D, S>(&mut self, dst: D, src: S) -> Result<()>
  where
    Self: SubqOp<D, S>,
  {
    self.emit_subq(dst, src)
  }

  #[inline]
  pub fn imulq<D, S>(&mut self, dst: D, src: S) -> Result<()>
  where
    Self: ImulqOp<D, S>,
  {
    self.emit_imulq(dst, src)
  }

  #[inline]
  pub fn andq<D, S>(&mut self, dst: D, src: S) -> Result<()>
  where
    Self: AndqOp<D, S>,
  {
    self.emit_andq(dst, src)
  }

  #[inline]
  pub fn orq<D, S>(&mut self, dst: D, src: S) -> Result<()>
  where
    Self: OrqOp<D, S>,
  {
    self.emit_orq(dst, src)
  }

  #[inline]
  pub fn xorq<D, S>(&mut self, dst: D, src: S) -> Result<()>
  where
    Self: XorqOp<D, S>,
  {
    self.emit_xorq(dst, src)
  }

  #[inline]
  pub fn cmpq<D, S>(&mut self, dst: D, src: S) -> Result<()>
  where
    Self: CmpqOp<D, S>,
  {
    self.emit_cmpq(dst, src)
  }

  #[inline]
  pub fn testq<D, S>(&mut self, dst: D, src: S) -> Result<()>
  where
    Self: TestqOp<D, S>,
  {
    self.emit_testq(dst, src)
  }

  #[inline]
  pub fn pushq<S>(&mut self, src: S) -> Result<()>
  where
    Self: PushqOp<S>,
  {
    self.emit_pushq(src)
  }

  #[inline]
  pub fn popq<D>(&mut self, dst: D) -> Result<()>
  where
    Self: PopqOp<D>,
  {
    self.emit_popq(dst)
  }

  #[inline]
  pub fn jmp<T>(&mut self, target: T) -> Result<()>
  where
    Self: JmpOp<T>,
  {
    self.emit_jmp(target)
  }

  #[inline]
  pub fn call<T>(&mut self, target: T) -> Result<()>
  where
    Self: CallOp<T>,
  {
    self.emit_call(target)
  }
}

// 多态 Trait 绑定实现

pub trait MovqOp<D, S> {
  fn emit_movq(&mut self, dst: D, src: S) -> Result<()>;
}

impl MovqOp<Register, Register> for Assembler {
  #[inline]
  fn emit_movq(&mut self, dst: Register, src: Register) -> Result<()> {
    self.movq_rr(dst, src)
  }
}

impl MovqOp<Operand, Register> for Assembler {
  #[inline]
  fn emit_movq(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.movq_mr(dst, src)
  }
}

impl MovqOp<Register, Operand> for Assembler {
  #[inline]
  fn emit_movq(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.movq_rm(dst, src)
  }
}

impl MovqOp<Register, Immediate> for Assembler {
  #[inline]
  fn emit_movq(&mut self, dst: Register, src: Immediate) -> Result<()> {
    self.movq_ri(dst, src)
  }
}

impl MovqOp<Operand, Immediate> for Assembler {
  #[inline]
  fn emit_movq(&mut self, dst: Operand, src: Immediate) -> Result<()> {
    self.movq_mi(dst, src)
  }
}

impl MovqOp<Register, i64> for Assembler {
  #[inline]
  fn emit_movq(&mut self, dst: Register, src: i64) -> Result<()> {
    self.movq_imm64(dst, src)
  }
}

pub trait MovlOp<D, S> {
  fn emit_movl(&mut self, dst: D, src: S) -> Result<()>;
}

impl MovlOp<Register, Register> for Assembler {
  #[inline]
  fn emit_movl(&mut self, dst: Register, src: Register) -> Result<()> {
    self.movl_rr(dst, src)
  }
}

impl MovlOp<Operand, Register> for Assembler {
  #[inline]
  fn emit_movl(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.movl_mr(dst, src)
  }
}

impl MovlOp<Register, Operand> for Assembler {
  #[inline]
  fn emit_movl(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.movl_rm(dst, src)
  }
}

impl MovlOp<Register, Immediate> for Assembler {
  #[inline]
  fn emit_movl(&mut self, dst: Register, src: Immediate) -> Result<()> {
    self.movl_ri(dst, src)
  }
}

impl MovlOp<Operand, Immediate> for Assembler {
  #[inline]
  fn emit_movl(&mut self, dst: Operand, src: Immediate) -> Result<()> {
    self.movl_mi(dst, src)
  }
}

pub trait AddqOp<D, S> {
  fn emit_addq(&mut self, dst: D, src: S) -> Result<()>;
}

impl AddqOp<Register, Register> for Assembler {
  #[inline]
  fn emit_addq(&mut self, dst: Register, src: Register) -> Result<()> {
    self.addq_rr(dst, src)
  }
}

impl AddqOp<Register, Operand> for Assembler {
  #[inline]
  fn emit_addq(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.addq_rm(dst, src)
  }
}

impl AddqOp<Operand, Register> for Assembler {
  #[inline]
  fn emit_addq(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.addq_mr(dst, src)
  }
}

impl AddqOp<Register, Immediate> for Assembler {
  #[inline]
  fn emit_addq(&mut self, dst: Register, src: Immediate) -> Result<()> {
    self.addq_ri(dst, src)
  }
}

impl AddqOp<Operand, Immediate> for Assembler {
  #[inline]
  fn emit_addq(&mut self, dst: Operand, src: Immediate) -> Result<()> {
    self.addq_mi(dst, src)
  }
}

pub trait SubqOp<D, S> {
  fn emit_subq(&mut self, dst: D, src: S) -> Result<()>;
}

impl SubqOp<Register, Register> for Assembler {
  #[inline]
  fn emit_subq(&mut self, dst: Register, src: Register) -> Result<()> {
    self.subq_rr(dst, src)
  }
}

impl SubqOp<Register, Operand> for Assembler {
  #[inline]
  fn emit_subq(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.subq_rm(dst, src)
  }
}

impl SubqOp<Operand, Register> for Assembler {
  #[inline]
  fn emit_subq(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.subq_mr(dst, src)
  }
}

impl SubqOp<Register, Immediate> for Assembler {
  #[inline]
  fn emit_subq(&mut self, dst: Register, src: Immediate) -> Result<()> {
    self.subq_ri(dst, src)
  }
}

impl SubqOp<Operand, Immediate> for Assembler {
  #[inline]
  fn emit_subq(&mut self, dst: Operand, src: Immediate) -> Result<()> {
    self.subq_mi(dst, src)
  }
}

pub trait ImulqOp<D, S> {
  fn emit_imulq(&mut self, dst: D, src: S) -> Result<()>;
}

impl ImulqOp<Register, Register> for Assembler {
  #[inline]
  fn emit_imulq(&mut self, dst: Register, src: Register) -> Result<()> {
    self.imulq_rr(dst, src)
  }
}

impl ImulqOp<Register, Operand> for Assembler {
  #[inline]
  fn emit_imulq(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.imulq_rm(dst, src)
  }
}

pub trait AndqOp<D, S> {
  fn emit_andq(&mut self, dst: D, src: S) -> Result<()>;
}

impl AndqOp<Register, Register> for Assembler {
  #[inline]
  fn emit_andq(&mut self, dst: Register, src: Register) -> Result<()> {
    self.andq_rr(dst, src)
  }
}

impl AndqOp<Register, Operand> for Assembler {
  #[inline]
  fn emit_andq(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.andq_rm(dst, src)
  }
}

impl AndqOp<Operand, Register> for Assembler {
  #[inline]
  fn emit_andq(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.andq_mr(dst, src)
  }
}

impl AndqOp<Register, Immediate> for Assembler {
  #[inline]
  fn emit_andq(&mut self, dst: Register, src: Immediate) -> Result<()> {
    self.andq_ri(dst, src)
  }
}

impl AndqOp<Operand, Immediate> for Assembler {
  #[inline]
  fn emit_andq(&mut self, dst: Operand, src: Immediate) -> Result<()> {
    self.andq_mi(dst, src)
  }
}

pub trait OrqOp<D, S> {
  fn emit_orq(&mut self, dst: D, src: S) -> Result<()>;
}

impl OrqOp<Register, Register> for Assembler {
  #[inline]
  fn emit_orq(&mut self, dst: Register, src: Register) -> Result<()> {
    self.orq_rr(dst, src)
  }
}

impl OrqOp<Register, Operand> for Assembler {
  #[inline]
  fn emit_orq(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.orq_rm(dst, src)
  }
}

impl OrqOp<Operand, Register> for Assembler {
  #[inline]
  fn emit_orq(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.orq_mr(dst, src)
  }
}

impl OrqOp<Register, Immediate> for Assembler {
  #[inline]
  fn emit_orq(&mut self, dst: Register, src: Immediate) -> Result<()> {
    self.orq_ri(dst, src)
  }
}

impl OrqOp<Operand, Immediate> for Assembler {
  #[inline]
  fn emit_orq(&mut self, dst: Operand, src: Immediate) -> Result<()> {
    self.orq_mi(dst, src)
  }
}

pub trait XorqOp<D, S> {
  fn emit_xorq(&mut self, dst: D, src: S) -> Result<()>;
}

impl XorqOp<Register, Register> for Assembler {
  #[inline]
  fn emit_xorq(&mut self, dst: Register, src: Register) -> Result<()> {
    self.xorq_rr(dst, src)
  }
}

impl XorqOp<Register, Operand> for Assembler {
  #[inline]
  fn emit_xorq(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.xorq_rm(dst, src)
  }
}

impl XorqOp<Operand, Register> for Assembler {
  #[inline]
  fn emit_xorq(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.xorq_mr(dst, src)
  }
}

impl XorqOp<Register, Immediate> for Assembler {
  #[inline]
  fn emit_xorq(&mut self, dst: Register, src: Immediate) -> Result<()> {
    self.xorq_ri(dst, src)
  }
}

impl XorqOp<Operand, Immediate> for Assembler {
  #[inline]
  fn emit_xorq(&mut self, dst: Operand, src: Immediate) -> Result<()> {
    self.xorq_mi(dst, src)
  }
}

pub trait CmpqOp<D, S> {
  fn emit_cmpq(&mut self, dst: D, src: S) -> Result<()>;
}

impl CmpqOp<Register, Register> for Assembler {
  #[inline]
  fn emit_cmpq(&mut self, dst: Register, src: Register) -> Result<()> {
    self.cmpq_rr(dst, src)
  }
}

impl CmpqOp<Register, Operand> for Assembler {
  #[inline]
  fn emit_cmpq(&mut self, dst: Register, src: Operand) -> Result<()> {
    self.cmpq_rm(dst, src)
  }
}

impl CmpqOp<Operand, Register> for Assembler {
  #[inline]
  fn emit_cmpq(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.cmpq_mr(dst, src)
  }
}

impl CmpqOp<Register, Immediate> for Assembler {
  #[inline]
  fn emit_cmpq(&mut self, dst: Register, src: Immediate) -> Result<()> {
    self.cmpq_ri(dst, src)
  }
}

impl CmpqOp<Operand, Immediate> for Assembler {
  #[inline]
  fn emit_cmpq(&mut self, dst: Operand, src: Immediate) -> Result<()> {
    self.cmpq_mi(dst, src)
  }
}

pub trait TestqOp<D, S> {
  fn emit_testq(&mut self, dst: D, src: S) -> Result<()>;
}

impl TestqOp<Register, Register> for Assembler {
  #[inline]
  fn emit_testq(&mut self, dst: Register, src: Register) -> Result<()> {
    self.testq_rr(dst, src)
  }
}

impl TestqOp<Operand, Register> for Assembler {
  #[inline]
  fn emit_testq(&mut self, dst: Operand, src: Register) -> Result<()> {
    self.testq_mr(dst, src)
  }
}

impl TestqOp<Register, Immediate> for Assembler {
  #[inline]
  fn emit_testq(&mut self, dst: Register, src: Immediate) -> Result<()> {
    self.testq_ri(dst, src)
  }
}

impl TestqOp<Operand, Immediate> for Assembler {
  #[inline]
  fn emit_testq(&mut self, dst: Operand, src: Immediate) -> Result<()> {
    self.testq_mi(dst, src)
  }
}

pub trait PushqOp<S> {
  fn emit_pushq(&mut self, src: S) -> Result<()>;
}

impl PushqOp<Register> for Assembler {
  #[inline]
  fn emit_pushq(&mut self, src: Register) -> Result<()> {
    self.pushq_r(src)
  }
}

impl PushqOp<Operand> for Assembler {
  #[inline]
  fn emit_pushq(&mut self, src: Operand) -> Result<()> {
    self.pushq_m(src)
  }
}

impl PushqOp<Immediate> for Assembler {
  #[inline]
  fn emit_pushq(&mut self, src: Immediate) -> Result<()> {
    self.pushq_i(src)
  }
}

pub trait PopqOp<D> {
  fn emit_popq(&mut self, dst: D) -> Result<()>;
}

impl PopqOp<Register> for Assembler {
  #[inline]
  fn emit_popq(&mut self, dst: Register) -> Result<()> {
    self.popq_r(dst)
  }
}

impl PopqOp<Operand> for Assembler {
  #[inline]
  fn emit_popq(&mut self, dst: Operand) -> Result<()> {
    self.popq_m(dst)
  }
}

pub trait JmpOp<T> {
  fn emit_jmp(&mut self, target: T) -> Result<()>;
}

impl JmpOp<Register> for Assembler {
  #[inline]
  fn emit_jmp(&mut self, target: Register) -> Result<()> {
    self.jmp_r(target)
  }
}

impl JmpOp<Operand> for Assembler {
  #[inline]
  fn emit_jmp(&mut self, target: Operand) -> Result<()> {
    self.jmp_m(target)
  }
}

impl JmpOp<&mut Label> for Assembler {
  #[inline]
  fn emit_jmp(&mut self, target: &mut Label) -> Result<()> {
    self.jmp_label(target, Distance::Far)
  }
}

pub trait CallOp<T> {
  fn emit_call(&mut self, target: T) -> Result<()>;
}

impl CallOp<Register> for Assembler {
  #[inline]
  fn emit_call(&mut self, target: Register) -> Result<()> {
    self.call_r(target)
  }
}

impl CallOp<Operand> for Assembler {
  #[inline]
  fn emit_call(&mut self, target: Operand) -> Result<()> {
    self.call_m(target)
  }
}

impl CallOp<&mut Label> for Assembler {
  #[inline]
  fn emit_call(&mut self, target: &mut Label) -> Result<()> {
    self.call_label(target)
  }
}
