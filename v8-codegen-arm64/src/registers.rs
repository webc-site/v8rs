// 由 tool/src/codegen/arm64/register-arm64.h.js 从 v8/src/codegen/arm64/register-arm64.h 生成; 修订就地改, 复跑覆盖
// ARM64 架构寄存器与条件码定义
// 注: 原版 C++ 侧寄存器/条件码常量为小写 (x0/d0/eq/...), Rust 侧统一以大写常量与
// Condition 枚举变体承接; 小写别名已按 non_upper_case_globals 清障移除。

use core::mem::size_of;

use crate::constants::{
  K_B_REG_SIZE_IN_BITS, K_D_REG_SIZE_IN_BITS, K_H_REG_SIZE_IN_BITS, K_NUMBER_OF_REGISTERS,
  K_NUMBER_OF_V_REGISTERS, K_Q_REG_SIZE_IN_BITS, K_S_REG_SIZE_IN_BITS, K_SP_REG_INTERNAL_CODE,
  K_W_REG_SIZE_IN_BITS, K_X_REG_SIZE_IN_BITS, K_ZERO_REG_CODE,
};

/// 对应原版 RegisterBase::kCode_no_reg = -1
pub const K_CODE_NO_REG: i8 = -1;

/// 寄存器类别 (对应原版 CPURegister::RegisterType, register-arm64.h:112 声明序 kRegister,kVRegister,kZRegister,kNoRegister = 0,1,2,3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum CPURegisterType {
  Register = 0,
  VRegister = 1,
  ZRegister = 2,
  #[default]
  NoRegister = 3,
}

/// 通用/SIMD/FP 统一寄存器基础类型 (对应原版 v8::internal::CPURegister)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CPURegister {
  code: i8,
  size_in_bits: u8,
  reg_type: CPURegisterType,
}

impl CPURegister {
  pub const fn no_reg() -> Self {
    Self {
      code: K_CODE_NO_REG,
      size_in_bits: 0,
      reg_type: CPURegisterType::NoRegister,
    }
  }

  pub const fn new(code: i8, size_in_bits: u8, reg_type: CPURegisterType) -> Self {
    Self {
      code,
      size_in_bits,
      reg_type,
    }
  }

  #[inline]
  pub const fn is_valid(self) -> bool {
    self.code != K_CODE_NO_REG && self.reg_type as i8 != CPURegisterType::NoRegister as i8
  }

  #[inline]
  pub const fn is_none(self) -> bool {
    !self.is_valid()
  }

  #[inline]
  pub const fn code(self) -> i8 {
    self.code
  }

  #[inline]
  pub const fn size_in_bits(self) -> usize {
    self.size_in_bits as usize
  }

  #[inline]
  pub const fn size_in_bytes(self) -> usize {
    (self.size_in_bits as usize) / 8
  }

  #[inline]
  pub const fn reg_type(self) -> CPURegisterType {
    self.reg_type
  }

  #[inline]
  pub const fn is_register(self) -> bool {
    matches!(self.reg_type, CPURegisterType::Register)
  }

  #[inline]
  pub const fn is_v_register(self) -> bool {
    matches!(self.reg_type, CPURegisterType::VRegister)
  }

  #[inline]
  pub const fn is_64_bits(self) -> bool {
    self.size_in_bits == 64
  }

  #[inline]
  pub const fn is_32_bits(self) -> bool {
    self.size_in_bits == 32
  }

  #[inline]
  pub const fn is_sp(self) -> bool {
    self.is_register() && self.code == K_SP_REG_INTERNAL_CODE as i8
  }

  #[inline]
  pub const fn is_zero(self) -> bool {
    self.is_register() && self.code == K_ZERO_REG_CODE as i8
  }
}

impl Default for CPURegister {
  fn default() -> Self {
    Self::no_reg()
  }
}

/// 通用寄存器 (区分 64 位 X 与 32 位 W，对应原版 v8::internal::Register)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register {
  code: i8,
  size_in_bits: u8,
}

impl Register {
  pub const fn no_reg() -> Self {
    Self {
      code: K_CODE_NO_REG,
      size_in_bits: 0,
    }
  }

  pub const fn from_code(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_X_REG_SIZE_IN_BITS as u8,
    }
  }

  pub const fn from_code_and_size(code: i8, size_in_bits: u8) -> Self {
    Self { code, size_in_bits }
  }

  pub const fn x(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_X_REG_SIZE_IN_BITS as u8,
    }
  }

  pub const fn w(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_W_REG_SIZE_IN_BITS as u8,
    }
  }

  pub const fn sp() -> Self {
    Self {
      code: K_SP_REG_INTERNAL_CODE as i8,
      size_in_bits: K_X_REG_SIZE_IN_BITS as u8,
    }
  }

  pub const fn wsp() -> Self {
    Self {
      code: K_SP_REG_INTERNAL_CODE as i8,
      size_in_bits: K_W_REG_SIZE_IN_BITS as u8,
    }
  }

  #[inline]
  pub const fn is_valid(self) -> bool {
    self.code != K_CODE_NO_REG
  }

  #[inline]
  pub const fn code(self) -> i8 {
    self.code
  }

  #[inline]
  pub const fn size_in_bits(self) -> usize {
    self.size_in_bits as usize
  }

  #[inline]
  pub const fn size_in_bytes(self) -> usize {
    (self.size_in_bits as usize) / 8
  }

  #[inline]
  pub const fn is_64_bits(self) -> bool {
    self.size_in_bits == K_X_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_32_bits(self) -> bool {
    self.size_in_bits == K_W_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_sp(self) -> bool {
    self.code == K_SP_REG_INTERNAL_CODE as i8
  }

  #[inline]
  pub const fn is_zero(self) -> bool {
    self.code == K_ZERO_REG_CODE as i8
  }

  #[inline]
  pub const fn to_x(self) -> Self {
    Self {
      code: self.code,
      size_in_bits: K_X_REG_SIZE_IN_BITS as u8,
    }
  }

  #[inline]
  pub const fn to_w(self) -> Self {
    Self {
      code: self.code,
      size_in_bits: K_W_REG_SIZE_IN_BITS as u8,
    }
  }

  #[inline]
  pub const fn as_cpu_register(self) -> CPURegister {
    CPURegister::new(self.code, self.size_in_bits, CPURegisterType::Register)
  }
}

impl Default for Register {
  fn default() -> Self {
    Self::no_reg()
  }
}

/// NEON/FP 向量寄存器 (对应原版 v8::internal::VRegister)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VRegister {
  code: i8,
  size_in_bits: u8,
  lane_count: u8,
}

impl VRegister {
  pub const fn no_reg() -> Self {
    Self {
      code: K_CODE_NO_REG,
      size_in_bits: 0,
      lane_count: 0,
    }
  }

  pub const fn from_code(code: i8) -> Self {
    // 默认 D 寄存器 (64 位，1 车道)
    Self {
      code,
      size_in_bits: K_D_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn b(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_B_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn h(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_H_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn s(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_S_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn d(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_D_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn q(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_Q_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  pub const fn v(code: i8) -> Self {
    Self {
      code,
      size_in_bits: K_Q_REG_SIZE_IN_BITS as u8,
      lane_count: 1,
    }
  }

  #[inline]
  pub const fn is_valid(self) -> bool {
    self.code != K_CODE_NO_REG
  }

  #[inline]
  pub const fn code(self) -> i8 {
    self.code
  }

  #[inline]
  pub const fn size_in_bits(self) -> usize {
    self.size_in_bits as usize
  }

  #[inline]
  pub const fn size_in_bytes(self) -> usize {
    (self.size_in_bits as usize) / 8
  }

  #[inline]
  pub const fn lane_count(self) -> usize {
    self.lane_count as usize
  }

  #[inline]
  pub const fn is_b(self) -> bool {
    self.size_in_bits == K_B_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_h(self) -> bool {
    self.size_in_bits == K_H_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_s(self) -> bool {
    self.size_in_bits == K_S_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_d(self) -> bool {
    self.size_in_bits == K_D_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn is_q(self) -> bool {
    self.size_in_bits == K_Q_REG_SIZE_IN_BITS as u8
  }

  #[inline]
  pub const fn as_cpu_register(self) -> CPURegister {
    CPURegister::new(self.code, self.size_in_bits, CPURegisterType::VRegister)
  }
}

impl Default for VRegister {
  fn default() -> Self {
    Self::no_reg()
  }
}

pub type FloatRegister = VRegister;
pub type DoubleRegister = VRegister;
pub type Simd128Register = VRegister;

/// 条件码枚举 (对应原版 v8::internal::Condition)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Condition {
  Eq = 0,
  Ne = 1,
  Cs = 2,
  Cc = 3,
  Mi = 4,
  Pl = 5,
  Vs = 6,
  Vc = 7,
  Hi = 8,
  Ls = 9,
  Ge = 10,
  Lt = 11,
  Gt = 12,
  Le = 13,
  Al = 14,
  Nv = 15,
}

impl Condition {
  #[inline]
  pub const fn code(self) -> u32 {
    self as u32
  }

  #[inline]
  pub const fn negate(self) -> Self {
    match (self as u8) ^ 1 {
      0 => Self::Eq,
      1 => Self::Ne,
      2 => Self::Cs,
      3 => Self::Cc,
      4 => Self::Mi,
      5 => Self::Pl,
      6 => Self::Vs,
      7 => Self::Vc,
      8 => Self::Hi,
      9 => Self::Ls,
      10 => Self::Ge,
      11 => Self::Lt,
      12 => Self::Gt,
      13 => Self::Le,
      14 => Self::Al,
      _ => Self::Nv,
    }
  }
}

impl From<Register> for CPURegister {
  #[inline]
  fn from(reg: Register) -> Self {
    reg.as_cpu_register()
  }
}

impl From<VRegister> for CPURegister {
  #[inline]
  fn from(reg: VRegister) -> Self {
    reg.as_cpu_register()
  }
}

/// 寄存器列表集合 (对应原版 v8::internal::CPURegList)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CPURegList {
  bits: u64,
  size_in_bits: u8,
  reg_type: CPURegisterType,
}

impl CPURegList {
  pub const fn empty(size_in_bits: u8, reg_type: CPURegisterType) -> Self {
    Self {
      bits: 0,
      size_in_bits,
      reg_type,
    }
  }

  pub const fn from_bits(bits: u64, size_in_bits: u8, reg_type: CPURegisterType) -> Self {
    Self {
      bits,
      size_in_bits,
      reg_type,
    }
  }

  pub fn from_slice(regs: &[Register]) -> Self {
    let mut list = Self::empty(64, CPURegisterType::Register);
    if let Some(first) = regs.first() {
      list.size_in_bits = first.size_in_bits;
    }
    for &r in regs {
      list.set(r);
    }
    list
  }

  pub fn from_vslice(regs: &[VRegister]) -> Self {
    let mut list = Self::empty(64, CPURegisterType::VRegister);
    if let Some(first) = regs.first() {
      list.size_in_bits = first.size_in_bits;
    }
    for &r in regs {
      list.set_v(r);
    }
    list
  }

  #[inline]
  pub const fn bits(self) -> u64 {
    self.bits
  }

  #[inline]
  pub const fn size_in_bits(self) -> usize {
    self.size_in_bits as usize
  }

  #[inline]
  pub const fn reg_type(self) -> CPURegisterType {
    self.reg_type
  }

  #[inline]
  pub const fn is_empty(self) -> bool {
    self.bits == 0
  }

  #[inline]
  pub const fn count(self) -> usize {
    self.bits.count_ones() as usize
  }

  #[inline]
  pub const fn total_size_in_bytes(self) -> usize {
    (self.size_in_bits as usize / 8) * self.count()
  }

  #[inline]
  pub fn set(&mut self, reg: Register) {
    if reg.is_valid() && reg.code() >= 0 && reg.code() < 32 {
      self.bits |= 1u64 << (reg.code() as u32);
    }
  }

  #[inline]
  pub fn clear(&mut self, reg: Register) {
    if reg.is_valid() && reg.code() >= 0 && reg.code() < 32 {
      self.bits &= !(1u64 << (reg.code() as u32));
    }
  }

  #[inline]
  pub const fn has(self, reg: Register) -> bool {
    if !reg.is_valid() || reg.code() < 0 || reg.code() >= 32 {
      false
    } else {
      (self.bits & (1u64 << (reg.code() as u32))) != 0
    }
  }

  #[inline]
  pub fn set_v(&mut self, reg: VRegister) {
    if reg.is_valid() && reg.code() >= 0 && reg.code() < 32 {
      self.bits |= 1u64 << (reg.code() as u32);
    }
  }

  #[inline]
  pub fn clear_v(&mut self, reg: VRegister) {
    if reg.is_valid() && reg.code() >= 0 && reg.code() < 32 {
      self.bits &= !(1u64 << (reg.code() as u32));
    }
  }

  #[inline]
  pub const fn has_v(self, reg: VRegister) -> bool {
    if !reg.is_valid() || reg.code() < 0 || reg.code() >= 32 {
      false
    } else {
      (self.bits & (1u64 << (reg.code() as u32))) != 0
    }
  }

  #[inline]
  pub fn combine(&mut self, other: Self) {
    self.bits |= other.bits;
  }

  #[inline]
  pub fn remove(&mut self, other: Self) {
    self.bits &= !other.bits;
  }

  pub fn pop_lowest_index(&mut self) -> Option<Register> {
    if self.bits == 0 {
      return None;
    }
    let idx = self.bits.trailing_zeros() as i8;
    self.bits &= self.bits - 1;
    Some(Register::from_code_and_size(idx, self.size_in_bits))
  }

  pub fn pop_highest_index(&mut self) -> Option<Register> {
    if self.bits == 0 {
      return None;
    }
    let idx = 63 - self.bits.leading_zeros() as i8;
    self.bits &= !(1u64 << idx);
    Some(Register::from_code_and_size(idx, self.size_in_bits))
  }

  pub fn pop_lowest_cpu_reg(&mut self) -> Option<CPURegister> {
    if self.bits == 0 {
      return None;
    }
    let idx = self.bits.trailing_zeros() as i8;
    self.bits &= self.bits - 1;
    Some(CPURegister::new(idx, self.size_in_bits, self.reg_type))
  }

  pub fn pop_highest_cpu_reg(&mut self) -> Option<CPURegister> {
    if self.bits == 0 {
      return None;
    }
    let idx = 63 - self.bits.leading_zeros() as i8;
    self.bits &= !(1u64 << idx);
    Some(CPURegister::new(idx, self.size_in_bits, self.reg_type))
  }

  /// AAPCS64 被调用者保存寄存器 (x19..x28)
  pub const fn get_callee_saved() -> Self {
    let mut bits = 0u64;
    let mut i = 19;
    while i <= 28 {
      bits |= 1u64 << i;
      i += 1;
    }
    Self::from_bits(bits, 64, CPURegisterType::Register)
  }

  /// AAPCS64 被调用者保存浮点寄存器 (d8..d15)
  pub const fn get_callee_saved_d() -> Self {
    let mut bits = 0u64;
    let mut i = 8;
    while i <= 15 {
      bits |= 1u64 << i;
      i += 1;
    }
    Self::from_bits(bits, 64, CPURegisterType::VRegister)
  }

  /// 原版 GetCallerSaved: 通用寄存器 x0..x17 (assembler-arm64.cc:248-252)。
  /// x18 为平台寄存器保留 (cc:249-250 注), lr/x30 不在实现列表——头注释
  /// reglist-arm64.h:108 "includes lr" 系过时注释, 以 .cc 实现为准
  pub const fn get_caller_saved() -> Self {
    let bits = (1u64 << 18) - 1; // 0..17
    Self::from_bits(bits, 64, CPURegisterType::Register)
  }

  /// AAPCS64 调用者保存浮点寄存器 (d0..d7, d16..d31)
  pub const fn get_caller_saved_d() -> Self {
    let low = (1u64 << 8) - 1; // 0..7
    let high = ((1u64 << 32) - 1) & !((1u64 << 16) - 1); // 16..31
    Self::from_bits(low | high, 64, CPURegisterType::VRegister)
  }

  /// 原版 GetCalleeSavedV: **空集** (assembler-arm64.cc:244-246——AAPCS64 仅要求
  /// 保存 v8-v15 的低 64 位即 d8-d15, 整个 Q 寄存器无须被调保存), size=kQRegSizeInBits=128
  pub const fn get_callee_saved_v() -> Self {
    Self::from_bits(0, 128, CPURegisterType::VRegister)
  }

  /// 原版 GetCallerSavedV: v0..v31 全体 (assembler-arm64.cc:262-268,
  /// kQRegSizeInBits=128, CPURegister::kVRegister, 0, 31)
  pub const fn get_caller_saved_v() -> Self {
    let bits = (1u64 << 32) - 1; // 0..31
    Self::from_bits(bits, 128, CPURegisterType::VRegister)
  }

  /// V8 原版无对应函数 (x0..x7 系 AAPCS64 参数寄存器惯例), 归 macro-assembler 票收编
  pub const fn get_argument_registers() -> Self {
    let bits = (1u64 << 8) - 1; // 0..7
    Self::from_bits(bits, 64, CPURegisterType::Register)
  }

  /// V8 原版无对应函数 (d0..d7 系 AAPCS64 浮点参数寄存器惯例), 归 macro-assembler 票收编
  pub const fn get_argument_registers_d() -> Self {
    let bits = (1u64 << 8) - 1; // 0..7
    Self::from_bits(bits, 64, CPURegisterType::VRegister)
  }

  /// 原版 IncludesAliasOf 变参模板收窄为单参 (reglist-arm64.h:119-133):
  /// 各参先隐式转 CPURegister (Rust 侧以 impl Into 承接 std::is_convertible_v 约束),
  /// 有效且 type()==type_ 才置位, 返回 (list_ & list) != 0。跨类型恒 false
  pub fn includes_alias_of(&self, reg: impl Into<CPURegister>) -> bool {
    let reg: CPURegister = reg.into();
    if !reg.is_valid() || reg.reg_type() != self.reg_type {
      return false;
    }
    let code = reg.code();
    code >= 0 && (code as u64) < 64 && ((self.bits >> (code as u64)) & 1) == 1
  }

  /// 原版 CPURegList::Align (assembler-arm64.cc:221-232): 栈对齐维护——奇数个时
  /// 翻转 padreg: 已含则 Remove, 否则 Combine; padreg=x31 (register-arm64.h:586,
  /// Rust 侧 xzr 与 x31 同 code)。偶数不动, 收尾承接 DCHECK_EQ(Count()%2, 0) (:230)
  pub fn align(&mut self) {
    const PAD_BIT: u64 = 1u64 << 31;
    if !self.count().is_multiple_of(2) {
      if (self.bits & PAD_BIT) != 0 {
        self.bits &= !PAD_BIT; // Remove(padreg)
      } else {
        self.bits |= PAD_BIT; // Combine(padreg)
      }
    }
    debug_assert!(self.count().is_multiple_of(2));
  }
}

// ── 32 个通用 64 位 X 寄存器常量 ──
pub const X0: Register = Register::x(0);
pub const X1: Register = Register::x(1);
pub const X2: Register = Register::x(2);
pub const X3: Register = Register::x(3);
pub const X4: Register = Register::x(4);
pub const X5: Register = Register::x(5);
pub const X6: Register = Register::x(6);
pub const X7: Register = Register::x(7);
pub const X8: Register = Register::x(8);
pub const X9: Register = Register::x(9);
pub const X10: Register = Register::x(10);
pub const X11: Register = Register::x(11);
pub const X12: Register = Register::x(12);
pub const X13: Register = Register::x(13);
pub const X14: Register = Register::x(14);
pub const X15: Register = Register::x(15);
pub const X16: Register = Register::x(16);
pub const X17: Register = Register::x(17);
pub const X18: Register = Register::x(18);
pub const X19: Register = Register::x(19);
pub const X20: Register = Register::x(20);
pub const X21: Register = Register::x(21);
pub const X22: Register = Register::x(22);
pub const X23: Register = Register::x(23);
pub const X24: Register = Register::x(24);
pub const X25: Register = Register::x(25);
pub const X26: Register = Register::x(26);
pub const X27: Register = Register::x(27);
pub const X28: Register = Register::x(28);
pub const X29: Register = Register::x(29);
pub const X30: Register = Register::x(30);
pub const X31: Register = Register::x(31);
pub const SP: Register = Register::sp();
pub const XZR: Register = Register::x(31);
pub const NO_REG: Register = Register::no_reg();

// ── 32 个通用 32 位 W 寄存器常量 ──
pub const W0: Register = Register::w(0);
pub const W1: Register = Register::w(1);
pub const W2: Register = Register::w(2);
pub const W3: Register = Register::w(3);
pub const W4: Register = Register::w(4);
pub const W5: Register = Register::w(5);
pub const W6: Register = Register::w(6);
pub const W7: Register = Register::w(7);
pub const W8: Register = Register::w(8);
pub const W9: Register = Register::w(9);
pub const W10: Register = Register::w(10);
pub const W11: Register = Register::w(11);
pub const W12: Register = Register::w(12);
pub const W13: Register = Register::w(13);
pub const W14: Register = Register::w(14);
pub const W15: Register = Register::w(15);
pub const W16: Register = Register::w(16);
pub const W17: Register = Register::w(17);
pub const W18: Register = Register::w(18);
pub const W19: Register = Register::w(19);
pub const W20: Register = Register::w(20);
pub const W21: Register = Register::w(21);
pub const W22: Register = Register::w(22);
pub const W23: Register = Register::w(23);
pub const W24: Register = Register::w(24);
pub const W25: Register = Register::w(25);
pub const W26: Register = Register::w(26);
pub const W27: Register = Register::w(27);
pub const W28: Register = Register::w(28);
pub const W29: Register = Register::w(29);
pub const W30: Register = Register::w(30);
pub const W31: Register = Register::w(31);
pub const WSP: Register = Register::wsp();
pub const WZR: Register = Register::w(31);

// ── 32 个 VRegister (D 寄存器) 常量 ──
pub const D0: VRegister = VRegister::d(0);
pub const V0: VRegister = VRegister::v(0);
pub const S0: VRegister = VRegister::s(0);
pub const Q0: VRegister = VRegister::q(0);
pub const B0: VRegister = VRegister::b(0);
pub const H0: VRegister = VRegister::h(0);
pub const D1: VRegister = VRegister::d(1);
pub const V1: VRegister = VRegister::v(1);
pub const S1: VRegister = VRegister::s(1);
pub const Q1: VRegister = VRegister::q(1);
pub const B1: VRegister = VRegister::b(1);
pub const H1: VRegister = VRegister::h(1);
pub const D2: VRegister = VRegister::d(2);
pub const V2: VRegister = VRegister::v(2);
pub const S2: VRegister = VRegister::s(2);
pub const Q2: VRegister = VRegister::q(2);
pub const B2: VRegister = VRegister::b(2);
pub const H2: VRegister = VRegister::h(2);
pub const D3: VRegister = VRegister::d(3);
pub const V3: VRegister = VRegister::v(3);
pub const S3: VRegister = VRegister::s(3);
pub const Q3: VRegister = VRegister::q(3);
pub const B3: VRegister = VRegister::b(3);
pub const H3: VRegister = VRegister::h(3);
pub const D4: VRegister = VRegister::d(4);
pub const V4: VRegister = VRegister::v(4);
pub const S4: VRegister = VRegister::s(4);
pub const Q4: VRegister = VRegister::q(4);
pub const B4: VRegister = VRegister::b(4);
pub const H4: VRegister = VRegister::h(4);
pub const D5: VRegister = VRegister::d(5);
pub const V5: VRegister = VRegister::v(5);
pub const S5: VRegister = VRegister::s(5);
pub const Q5: VRegister = VRegister::q(5);
pub const B5: VRegister = VRegister::b(5);
pub const H5: VRegister = VRegister::h(5);
pub const D6: VRegister = VRegister::d(6);
pub const V6: VRegister = VRegister::v(6);
pub const S6: VRegister = VRegister::s(6);
pub const Q6: VRegister = VRegister::q(6);
pub const B6: VRegister = VRegister::b(6);
pub const H6: VRegister = VRegister::h(6);
pub const D7: VRegister = VRegister::d(7);
pub const V7: VRegister = VRegister::v(7);
pub const S7: VRegister = VRegister::s(7);
pub const Q7: VRegister = VRegister::q(7);
pub const B7: VRegister = VRegister::b(7);
pub const H7: VRegister = VRegister::h(7);
pub const D8: VRegister = VRegister::d(8);
pub const V8: VRegister = VRegister::v(8);
pub const S8: VRegister = VRegister::s(8);
pub const Q8: VRegister = VRegister::q(8);
pub const B8: VRegister = VRegister::b(8);
pub const H8: VRegister = VRegister::h(8);
pub const D9: VRegister = VRegister::d(9);
pub const V9: VRegister = VRegister::v(9);
pub const S9: VRegister = VRegister::s(9);
pub const Q9: VRegister = VRegister::q(9);
pub const B9: VRegister = VRegister::b(9);
pub const H9: VRegister = VRegister::h(9);
pub const D10: VRegister = VRegister::d(10);
pub const V10: VRegister = VRegister::v(10);
pub const S10: VRegister = VRegister::s(10);
pub const Q10: VRegister = VRegister::q(10);
pub const B10: VRegister = VRegister::b(10);
pub const H10: VRegister = VRegister::h(10);
pub const D11: VRegister = VRegister::d(11);
pub const V11: VRegister = VRegister::v(11);
pub const S11: VRegister = VRegister::s(11);
pub const Q11: VRegister = VRegister::q(11);
pub const B11: VRegister = VRegister::b(11);
pub const H11: VRegister = VRegister::h(11);
pub const D12: VRegister = VRegister::d(12);
pub const V12: VRegister = VRegister::v(12);
pub const S12: VRegister = VRegister::s(12);
pub const Q12: VRegister = VRegister::q(12);
pub const B12: VRegister = VRegister::b(12);
pub const H12: VRegister = VRegister::h(12);
pub const D13: VRegister = VRegister::d(13);
pub const V13: VRegister = VRegister::v(13);
pub const S13: VRegister = VRegister::s(13);
pub const Q13: VRegister = VRegister::q(13);
pub const B13: VRegister = VRegister::b(13);
pub const H13: VRegister = VRegister::h(13);
pub const D14: VRegister = VRegister::d(14);
pub const V14: VRegister = VRegister::v(14);
pub const S14: VRegister = VRegister::s(14);
pub const Q14: VRegister = VRegister::q(14);
pub const B14: VRegister = VRegister::b(14);
pub const H14: VRegister = VRegister::h(14);
pub const D15: VRegister = VRegister::d(15);
pub const V15: VRegister = VRegister::v(15);
pub const S15: VRegister = VRegister::s(15);
pub const Q15: VRegister = VRegister::q(15);
pub const B15: VRegister = VRegister::b(15);
pub const H15: VRegister = VRegister::h(15);
pub const D16: VRegister = VRegister::d(16);
pub const V16: VRegister = VRegister::v(16);
pub const S16: VRegister = VRegister::s(16);
pub const Q16: VRegister = VRegister::q(16);
pub const B16: VRegister = VRegister::b(16);
pub const H16: VRegister = VRegister::h(16);
pub const D17: VRegister = VRegister::d(17);
pub const V17: VRegister = VRegister::v(17);
pub const S17: VRegister = VRegister::s(17);
pub const Q17: VRegister = VRegister::q(17);
pub const B17: VRegister = VRegister::b(17);
pub const H17: VRegister = VRegister::h(17);
pub const D18: VRegister = VRegister::d(18);
pub const V18: VRegister = VRegister::v(18);
pub const S18: VRegister = VRegister::s(18);
pub const Q18: VRegister = VRegister::q(18);
pub const B18: VRegister = VRegister::b(18);
pub const H18: VRegister = VRegister::h(18);
pub const D19: VRegister = VRegister::d(19);
pub const V19: VRegister = VRegister::v(19);
pub const S19: VRegister = VRegister::s(19);
pub const Q19: VRegister = VRegister::q(19);
pub const B19: VRegister = VRegister::b(19);
pub const H19: VRegister = VRegister::h(19);
pub const D20: VRegister = VRegister::d(20);
pub const V20: VRegister = VRegister::v(20);
pub const S20: VRegister = VRegister::s(20);
pub const Q20: VRegister = VRegister::q(20);
pub const B20: VRegister = VRegister::b(20);
pub const H20: VRegister = VRegister::h(20);
pub const D21: VRegister = VRegister::d(21);
pub const V21: VRegister = VRegister::v(21);
pub const S21: VRegister = VRegister::s(21);
pub const Q21: VRegister = VRegister::q(21);
pub const B21: VRegister = VRegister::b(21);
pub const H21: VRegister = VRegister::h(21);
pub const D22: VRegister = VRegister::d(22);
pub const V22: VRegister = VRegister::v(22);
pub const S22: VRegister = VRegister::s(22);
pub const Q22: VRegister = VRegister::q(22);
pub const B22: VRegister = VRegister::b(22);
pub const H22: VRegister = VRegister::h(22);
pub const D23: VRegister = VRegister::d(23);
pub const V23: VRegister = VRegister::v(23);
pub const S23: VRegister = VRegister::s(23);
pub const Q23: VRegister = VRegister::q(23);
pub const B23: VRegister = VRegister::b(23);
pub const H23: VRegister = VRegister::h(23);
pub const D24: VRegister = VRegister::d(24);
pub const V24: VRegister = VRegister::v(24);
pub const S24: VRegister = VRegister::s(24);
pub const Q24: VRegister = VRegister::q(24);
pub const B24: VRegister = VRegister::b(24);
pub const H24: VRegister = VRegister::h(24);
pub const D25: VRegister = VRegister::d(25);
pub const V25: VRegister = VRegister::v(25);
pub const S25: VRegister = VRegister::s(25);
pub const Q25: VRegister = VRegister::q(25);
pub const B25: VRegister = VRegister::b(25);
pub const H25: VRegister = VRegister::h(25);
pub const D26: VRegister = VRegister::d(26);
pub const V26: VRegister = VRegister::v(26);
pub const S26: VRegister = VRegister::s(26);
pub const Q26: VRegister = VRegister::q(26);
pub const B26: VRegister = VRegister::b(26);
pub const H26: VRegister = VRegister::h(26);
pub const D27: VRegister = VRegister::d(27);
pub const V27: VRegister = VRegister::v(27);
pub const S27: VRegister = VRegister::s(27);
pub const Q27: VRegister = VRegister::q(27);
pub const B27: VRegister = VRegister::b(27);
pub const H27: VRegister = VRegister::h(27);
pub const D28: VRegister = VRegister::d(28);
pub const V28: VRegister = VRegister::v(28);
pub const S28: VRegister = VRegister::s(28);
pub const Q28: VRegister = VRegister::q(28);
pub const B28: VRegister = VRegister::b(28);
pub const H28: VRegister = VRegister::h(28);
pub const D29: VRegister = VRegister::d(29);
pub const V29: VRegister = VRegister::v(29);
pub const S29: VRegister = VRegister::s(29);
pub const Q29: VRegister = VRegister::q(29);
pub const B29: VRegister = VRegister::b(29);
pub const H29: VRegister = VRegister::h(29);
pub const D30: VRegister = VRegister::d(30);
pub const V30: VRegister = VRegister::v(30);
pub const S30: VRegister = VRegister::s(30);
pub const Q30: VRegister = VRegister::q(30);
pub const B30: VRegister = VRegister::b(30);
pub const H30: VRegister = VRegister::h(30);
pub const D31: VRegister = VRegister::d(31);
pub const V31: VRegister = VRegister::v(31);
pub const S31: VRegister = VRegister::s(31);
pub const Q31: VRegister = VRegister::q(31);
pub const B31: VRegister = VRegister::b(31);
pub const H31: VRegister = VRegister::h(31);
pub const NO_DREG: VRegister = VRegister::no_reg();

// ── 常见架构别名 (原版小写 ip0/fp/lr/... 由下列大写形态承接) ──
pub const IP0: Register = X16;
pub const IP1: Register = X17;
pub const WIP0: Register = W16;
pub const WIP1: Register = W17;
pub const FP: Register = X29;
pub const LR: Register = X30;
pub const CP: Register = X27;
pub const RR: Register = X26;

// C 调用约定参数寄存器 (kCArgRegs = {x0..x7})
pub const K_C_ARG_REGS: [Register; 8] = [X0, X1, X2, X3, X4, X5, X6, X7];
pub const K_REGISTER_PASSED_ARGUMENTS: usize = 8;
pub const K_FP_REGISTER_PASSED_ARGUMENTS: usize = 8;
// ===== 名字表 (DEFINE_REGISTER_NAMES(Register, GENERAL_REGISTERS) 串化展开) =====
pub const GENERAL_REG_NAMES: [&str; 32] = [
  "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14",
  "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28",
  "x29", "x30", "x31",
];

/// RegisterName(Register) 1:1 (register-base.h:83-87): 无效或越界编码返回 "invalid"
/// 注: sp 内部编码 63 越界, 原版 Names[63] 为未定义行为, 此处按 v8-codegen-x64 同款边界守护
#[inline]
pub const fn register_name(reg: Register) -> &'static str {
  let code = reg.code();
  if code >= 0 && (code as usize) < GENERAL_REG_NAMES.len() {
    GENERAL_REG_NAMES[code as usize]
  } else {
    "invalid"
  }
}

/// 名称反查编号, 与 register_name 互逆 (GENERAL_REG_NAMES 内名字唯一)
pub fn reg_code_of(name: &str) -> Option<i8> {
  GENERAL_REG_NAMES
    .iter()
    .position(|&n| n == name)
    .map(|i| i as i8)
}

/// GetSpecialRegisterName (register-arm64.h:279-281) 1:1: 仅认 sp 内部编码, 其余 "UNKNOWN"
#[inline]
pub const fn get_special_register_name(code: i32) -> &'static str {
  if code == K_SP_REG_INTERNAL_CODE {
    "sp"
  } else {
    "UNKNOWN"
  }
}

pub const K_STACK_POINTER_REGISTER: Register = SP;
pub const V8_: VRegister = V8;
pub const K_ROOT_REGISTER: Register = X26;
/// kPtrComprCageBaseRegister (V8_COMPRESS_POINTERS WITH_COMPRESSION)
pub const K_PTR_COMPR_CAGE_BASE_REGISTER_WITH_COMPRESSION: Register = X28;
/// kPtrComprCageBaseRegister (V8_COMPRESS_POINTERS WITHOUT_COMPRESSION)
pub const K_PTR_COMPR_CAGE_BASE_REGISTER_WITHOUT_COMPRESSION: Register = NO_REG;
pub const PADREG: Register = X31;
pub const FP_ZERO: VRegister = D15;
pub const FP_FIXED1: VRegister = D27;
pub const FP_FIXED2: VRegister = D28;
pub const FP_SCRATCH: VRegister = D30;
pub const FP_SCRATCH1: VRegister = D30;
pub const FP_SCRATCH2: VRegister = D31;

// ===== 调用约定别名 (constexpr Register 23 条 1:1) =====
pub const K_RETURN_REGISTER0: Register = X0;
pub const K_RETURN_REGISTER1: Register = X1;
pub const K_RETURN_REGISTER2: Register = X2;
pub const K_JS_FUNCTION_REGISTER: Register = X1;
pub const K_CONTEXT_REGISTER: Register = CP;
pub const K_ALLOCATE_SIZE_REGISTER: Register = X1;
pub const K_INTERPRETER_ACCUMULATOR_REGISTER: Register = X0;
pub const K_INTERPRETER_BYTECODE_OFFSET_REGISTER: Register = X19;
pub const K_INTERPRETER_BYTECODE_ARRAY_REGISTER: Register = X20;
pub const K_INTERPRETER_DISPATCH_TABLE_REGISTER: Register = X21;
pub const K_JAVA_SCRIPT_CALL_ARG_COUNT_REGISTER: Register = X0;
pub const K_JAVA_SCRIPT_CALL_CODE_START_REGISTER: Register = X2;
pub const K_JAVA_SCRIPT_CALL_TARGET_REGISTER: Register = K_JS_FUNCTION_REGISTER;
pub const K_JAVA_SCRIPT_CALL_NEW_TARGET_REGISTER: Register = X3;
pub const K_JAVA_SCRIPT_CALL_EXTRA_ARG1_REGISTER: Register = X2;
pub const K_JAVA_SCRIPT_CALL_DISPATCH_HANDLE_REGISTER: Register = X4;
pub const K_RUNTIME_CALL_FUNCTION_REGISTER: Register = X1;
pub const K_RUNTIME_CALL_ARG_COUNT_REGISTER: Register = X0;
pub const K_RUNTIME_CALL_ARGV_REGISTER: Register = X11;
pub const K_WASM_IMPLICIT_ARG_REGISTER: Register = X7;
pub const K_WASM_COMPILE_LAZY_FUNC_INDEX_REGISTER: Register = X8;
pub const K_WASM_TRAP_HANDLER_FAULT_ADDRESS_REGISTER: Register = X16;
pub const K_SIMULATOR_HLT_ARGUMENT: Register = X16;
pub const K_FP_RETURN_REGISTER0: VRegister = D0; // constexpr DoubleRegister kFPReturnRegister0 = d0;

// ── 静态断言 ──
const _: () = assert!(size_of::<CPURegister>() <= 4);
const _: () = assert!(size_of::<Register>() <= 4);
const _: () = assert!(size_of::<VRegister>() <= 4);
const _: () = assert!(K_NUMBER_OF_REGISTERS == 32);
const _: () = assert!(K_NUMBER_OF_V_REGISTERS == 32);
const _: () = assert!(K_C_ARG_REGS.len() == 8);
const _: () = assert!(GENERAL_REG_NAMES.len() == K_NUMBER_OF_REGISTERS as usize);
