use v8_codegen_arm64::Condition;

use crate::constants::{
  C_FLAG_MASK, FP_DEFAULT_NAN_BITS, K_END_OF_SIM_ADDRESS, N_FLAG_MASK, V_FLAG_MASK, Z_FLAG_MASK,
};

/// ARM64 NZCV 状态条件标志位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Nzcv {
  /// 负数标志 (Negative, bit 31)
  pub n: bool,
  /// 零标志 (Zero, bit 30)
  pub z: bool,
  /// 进位标志 (Carry, bit 29)
  pub c: bool,
  /// 溢出标志 (oVerflow, bit 28)
  pub v: bool,
}

impl Nzcv {
  /// 编码为 32 位整数 (bit 31..28)
  pub const fn as_u32(&self) -> u32 {
    let mut val = 0u32;
    if self.n {
      val |= N_FLAG_MASK;
    }
    if self.z {
      val |= Z_FLAG_MASK;
    }
    if self.c {
      val |= C_FLAG_MASK;
    }
    if self.v {
      val |= V_FLAG_MASK;
    }
    val
  }

  /// 从 32 位整数解码
  pub fn set_from_u32(&mut self, val: u32) {
    self.n = (val & N_FLAG_MASK) != 0;
    self.z = (val & Z_FLAG_MASK) != 0;
    self.c = (val & C_FLAG_MASK) != 0;
    self.v = (val & V_FLAG_MASK) != 0;
  }

  /// 依据当前标志位测试条件是否成立 (对应原版 ConditionPassed)
  pub const fn condition_passed(&self, cond: Condition) -> bool {
    match cond {
      Condition::Eq => self.z,
      Condition::Ne => !self.z,
      Condition::Cs => self.c,
      Condition::Cc => !self.c,
      Condition::Mi => self.n,
      Condition::Pl => !self.n,
      Condition::Vs => self.v,
      Condition::Vc => !self.v,
      Condition::Hi => self.c && !self.z,
      Condition::Ls => !self.c || self.z,
      Condition::Ge => self.n == self.v,
      Condition::Lt => self.n != self.v,
      Condition::Gt => !self.z && (self.n == self.v),
      Condition::Le => self.z || (self.n != self.v),
      Condition::Al | Condition::Nv => true,
    }
  }

  /// 依据 32 位运算结果更新 NZCV
  pub fn update_nzcv_32(&mut self, result: u32, carry: bool, overflow: bool) {
    self.n = (result as i32) < 0;
    self.z = result == 0;
    self.c = carry;
    self.v = overflow;
  }

  /// 依据 64 位运算结果更新 NZCV
  pub fn update_nzcv_64(&mut self, result: u64, carry: bool, overflow: bool) {
    self.n = (result as i64) < 0;
    self.z = result == 0;
    self.c = carry;
    self.v = overflow;
  }
}

/// ARM64 模拟器寄存器状态集
#[derive(Debug, Clone)]
pub struct SimRegisters {
  /// 31 个 64 位通用寄存器 (x0..x30)
  x: [u64; 31],
  /// 栈指针 (SP)
  sp: u64,
  /// 程序计数器 (PC)
  pc: u64,
  /// 32 个 128 位向量寄存器 (v0..v31)
  v: [[u8; 16]; 32],
  /// 条件标志位
  pub nzcv: Nzcv,
}

impl Default for SimRegisters {
  fn default() -> Self {
    let mut regs = Self {
      x: [0; 31],
      sp: 0,
      pc: 0,
      v: [[0; 16]; 32],
      nzcv: Nzcv::default(),
    };
    regs.reset();
    regs
  }
}

impl SimRegisters {
  /// 重置全部寄存器状态为原版初值
  pub fn reset(&mut self) {
    self.x.fill(0);
    self.sp = 0;
    self.pc = 0;
    self.nzcv = Nzcv::default();
    // 向量寄存器填充双精度 NaN
    for reg in &mut self.v {
      reg.fill(0);
      reg[0..8].copy_from_slice(&FP_DEFAULT_NAN_BITS.to_le_bytes());
      reg[8..16].copy_from_slice(&FP_DEFAULT_NAN_BITS.to_le_bytes());
    }
    // 返回地址置结束哨兵 0
    self.set_lr(K_END_OF_SIM_ADDRESS);
  }

  /// 读取 64 位寄存器 (0..30 返回对应 xN; 31 为 XZR 恒等于 0)
  #[inline]
  pub fn x(&self, reg: usize) -> u64 {
    if reg < 31 { self.x[reg] } else { 0 }
  }

  /// 读取 32 位寄存器 (0..30 返回低 32 位; 31 为 WZR 恒等于 0)
  #[inline]
  pub fn w(&self, reg: usize) -> u32 {
    self.x(reg) as u32
  }

  /// 写入 64 位寄存器 (0..30 写入; 31 为 XZR 忽略写入)
  #[inline]
  pub fn set_x(&mut self, reg: usize, val: u64) {
    if reg < 31 {
      self.x[reg] = val;
    }
  }

  /// 写入 32 位寄存器 (0..30 写入并将高 32 位清零; 31 为 WZR 忽略写入)
  #[inline]
  pub fn set_w(&mut self, reg: usize, val: u32) {
    if reg < 31 {
      self.x[reg] = val as u64;
    }
  }

  /// 依据 is_64 标志读取 64 位或 32 位寄存器数值 (对应原版 reg<T>)
  #[inline]
  pub fn reg(&self, is_64: bool, reg: usize) -> u64 {
    if is_64 {
      self.x(reg)
    } else {
      self.w(reg) as u64
    }
  }

  /// 依据 is_64 标志写入 64 位或 32 位寄存器数值 (对应原版 set_reg)
  #[inline]
  pub fn set_reg(&mut self, is_64: bool, reg: usize, val: u64) {
    if is_64 {
      self.set_x(reg, val);
    } else {
      self.set_w(reg, val as u32);
    }
  }

  /// 读取栈指针 SP
  #[inline]
  pub const fn sp(&self) -> u64 {
    self.sp
  }

  /// 设置栈指针 SP
  #[inline]
  pub fn set_sp(&mut self, val: u64) {
    self.sp = val;
  }

  /// 读取程序计数器 PC
  #[inline]
  pub const fn pc(&self) -> u64 {
    self.pc
  }

  /// 设置程序计数器 PC
  #[inline]
  pub fn set_pc(&mut self, val: u64) {
    self.pc = val;
  }

  /// 读取链接寄存器 LR (x30)
  #[inline]
  pub fn lr(&self) -> u64 {
    self.x(30)
  }

  /// 设置链接寄存器 LR (x30)
  #[inline]
  pub fn set_lr(&mut self, val: u64) {
    self.set_x(30, val);
  }

  /// 读取帧指针 FP (x29)
  #[inline]
  pub fn fp(&self) -> u64 {
    self.x(29)
  }

  /// 设置帧指针 FP (x29)
  #[inline]
  pub fn set_fp(&mut self, val: u64) {
    self.set_x(29, val);
  }

  /// 读取 64 位双精度浮点寄存器 (d0..d31)
  #[inline]
  pub fn d(&self, reg: usize) -> f64 {
    let reg = reg & 31;
    let bytes: [u8; 8] = self.v[reg][0..8].try_into().unwrap_or([0; 8]);
    f64::from_le_bytes(bytes)
  }

  /// 设置 64 位双精度浮点寄存器 (d0..d31, 高 64 位清零)
  #[inline]
  pub fn set_d(&mut self, reg: usize, val: f64) {
    let reg = reg & 31;
    self.v[reg][0..8].copy_from_slice(&val.to_le_bytes());
    self.v[reg][8..16].fill(0);
  }

  /// 读取 32 位单精度浮点寄存器 (s0..s31)
  #[inline]
  pub fn s(&self, reg: usize) -> f32 {
    let reg = reg & 31;
    let bytes: [u8; 4] = self.v[reg][0..4].try_into().unwrap_or([0; 4]);
    f32::from_le_bytes(bytes)
  }

  /// 设置 32 位单精度浮点寄存器 (s0..s31, 高 96 位清零)
  #[inline]
  pub fn set_s(&mut self, reg: usize, val: f32) {
    let reg = reg & 31;
    self.v[reg][0..4].copy_from_slice(&val.to_le_bytes());
    self.v[reg][4..16].fill(0);
  }

  /// 读取 128 位向量寄存器 (q0..q31)
  #[inline]
  pub fn q(&self, reg: usize) -> u128 {
    let reg = reg & 31;
    u128::from_le_bytes(self.v[reg])
  }

  /// 设置 128 位向量寄存器 (q0..q31)
  #[inline]
  pub fn set_q(&mut self, reg: usize, val: u128) {
    let reg = reg & 31;
    self.v[reg] = val.to_le_bytes();
  }
}
