use v8_codegen_arm64::Condition;

use crate::{
  constants::{K_END_OF_SIM_ADDRESS, STACK_ALIGNMENT},
  error::{Error, Result},
  memory::SimulatorMemory,
  registers::SimRegisters,
};

/// ARM64 模拟器 (用于 x64/宿主平台执行 ARM64 生成代码)
#[derive(Debug)]
pub struct Simulator {
  /// 寄存器集
  pub registers: SimRegisters,
  /// 模拟内存与栈
  pub memory: SimulatorMemory,
  /// 已执行指令计数
  pub instruction_count: u64,
}

impl Default for Simulator {
  fn default() -> Self {
    Self::new()
  }
}

impl Simulator {
  /// 创建并初始化模拟器
  pub fn new() -> Self {
    let mut sim = Self {
      registers: SimRegisters::default(),
      memory: SimulatorMemory::new(),
      instruction_count: 0,
    };
    sim.reset();
    sim
  }

  /// 重置模拟器状态与栈指针
  pub fn reset(&mut self) {
    self.registers.reset();
    self.registers.set_sp(self.memory.initial_sp());
    self.instruction_count = 0;
  }

  /// 将生成的指令字节码加载到模拟器内存
  pub fn load_code(&mut self, entry: u64, code: &[u8]) {
    self.memory.load_code(entry, code);
  }

  /// 获取内存基地址并在使用 SP 时校验 16 字节对齐 (对应 ARM64 SP Alignment Check)
  #[inline]
  fn mem_base(&self, rn: usize) -> Result<u64> {
    if rn == 31 {
      let sp = self.registers.sp();
      self.memory.check_alignment(sp, STACK_ALIGNMENT)?;
      Ok(sp)
    } else {
      Ok(self.registers.x(rn))
    }
  }

  /// 单步执行当前 PC 指向的指令
  ///
  /// 返回 Ok(true) 表示可继续执行；Ok(false) 表示遇到返回哨兵 0 结束模拟
  pub fn step(&mut self) -> Result<bool> {
    let pc = self.registers.pc();
    if pc == K_END_OF_SIM_ADDRESS {
      return Ok(false);
    }

    let instr = self.memory.read_u32(pc)?;
    self.instruction_count += 1;

    // 默认下一条指令 PC = pc + 4
    let mut next_pc = pc.wrapping_add(4);

    // ── 指令解码与执行 ──
    // 1. NOP
    if instr == 0xd503_201f {
      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 2. RET [xn] (1101 0110 0101 1111 0000 00nn nnn0 0000)
    if (instr & 0xffff_fc1f) == 0xd65f_0000 {
      let rn = ((instr >> 5) & 0x1f) as usize;
      let target = self.registers.x(rn);
      self.registers.set_pc(target);
      return Ok(target != K_END_OF_SIM_ADDRESS);
    }

    // 3. BR xn (1101 0110 0001 1111 0000 00nn nnn0 0000)
    if (instr & 0xffff_fc1f) == 0xd61f_0000 {
      let rn = ((instr >> 5) & 0x1f) as usize;
      let target = self.registers.x(rn);
      self.registers.set_pc(target);
      return Ok(target != K_END_OF_SIM_ADDRESS);
    }

    // 4. BLR xn (1101 0110 0011 1111 0000 00nn nnn0 0000)
    if (instr & 0xffff_fc1f) == 0xd63f_0000 {
      let rn = ((instr >> 5) & 0x1f) as usize;
      let target = self.registers.x(rn);
      self.registers.set_lr(next_pc);
      self.registers.set_pc(target);
      return Ok(target != K_END_OF_SIM_ADDRESS);
    }

    // 5. B imm26 (0001 01ii iiii iiii iiii iiii iiii iiii)
    if (instr & 0xfc00_0000) == 0x1400_0000 {
      let imm26 = (instr & 0x03ff_ffff) as i32;
      let sign_ext = ((imm26 << 6) >> 6) as i64;
      let offset = (sign_ext * 4) as u64;
      next_pc = pc.wrapping_add(offset);
      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 6. BL imm26 (1001 01ii iiii iiii iiii iiii iiii iiii)
    if (instr & 0xfc00_0000) == 0x9400_0000 {
      let imm26 = (instr & 0x03ff_ffff) as i32;
      let sign_ext = ((imm26 << 6) >> 6) as i64;
      let offset = (sign_ext * 4) as u64;
      self.registers.set_lr(next_pc);
      next_pc = pc.wrapping_add(offset);
      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 7. B.cond (0101 0100 iiii iiii iiii iiii iii0 cccc)
    if (instr & 0xff00_0010) == 0x5400_0000 {
      let cond_code = (instr & 0x0f) as u8;
      let imm19 = ((instr >> 5) & 0x0007_ffff) as i32;
      let sign_ext = ((imm19 << 13) >> 13) as i64;
      let offset = (sign_ext * 4) as u64;

      if let Some(cond) = decode_condition(cond_code) {
        if self.registers.nzcv.condition_passed(cond) {
          next_pc = pc.wrapping_add(offset);
        }
        self.registers.set_pc(next_pc);
        return Ok(true);
      }
    }

    // 8. CBZ / CBNZ (sf 0110 10o iiii iiii iiii iiii iii rrrrr)
    if (instr & 0x7e00_0000) == 0x3400_0000 {
      let is_64 = (instr & 0x8000_0000) != 0;
      let is_cbnz = (instr & 0x0100_0000) != 0;
      let imm19 = ((instr >> 5) & 0x0007_ffff) as i32;
      let sign_ext = ((imm19 << 13) >> 13) as i64;
      let offset = (sign_ext * 4) as u64;
      let rt = (instr & 0x1f) as usize;

      let val = self.registers.reg(is_64, rt);
      let take_branch = if is_cbnz { val != 0 } else { val == 0 };
      if take_branch {
        next_pc = pc.wrapping_add(offset);
      }
      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 9. MOVZ (sf 1010 0101 hw iiiiiiiiiiiiiiii rrrrr)
    if (instr & 0x7f80_0000) == 0x5280_0000 {
      let is_64 = (instr & 0x8000_0000) != 0;
      let hw = (instr >> 21) & 0x3;
      let imm16 = ((instr >> 5) & 0xffff) as u64;
      let rd = (instr & 0x1f) as usize;
      let shift = hw * 16;
      let val = imm16 << shift;

      self.registers.set_reg(is_64, rd, val);
      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 10. MOVK (sf 1110 0101 hw iiiiiiiiiiiiiiii rrrrr)
    if (instr & 0x7f80_0000) == 0x7280_0000 {
      let is_64 = (instr & 0x8000_0000) != 0;
      let hw = (instr >> 21) & 0x3;
      let imm16 = ((instr >> 5) & 0xffff) as u64;
      let rd = (instr & 0x1f) as usize;
      let shift = hw * 16;
      let mask = !(0xffffu64 << shift);
      let cur = self.registers.reg(is_64, rd);
      let val = (cur & mask) | (imm16 << shift);

      self.registers.set_reg(is_64, rd, val);
      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 11. ADD / SUB (立即数) (sf op S 100010 sh iiiiiiiiiiii nnnnn rrrrr)
    if (instr & 0x1f00_0000) == 0x1100_0000 {
      let is_64 = (instr & 0x8000_0000) != 0;
      let is_sub = (instr & 0x4000_0000) != 0;
      let set_flags = (instr & 0x2000_0000) != 0;
      let shift12 = ((instr >> 22) & 0x1) != 0;
      let imm12 = ((instr >> 10) & 0x0fff) as u64;
      let imm = if shift12 { imm12 << 12 } else { imm12 };
      let rn = ((instr >> 5) & 0x1f) as usize;
      let rd = (instr & 0x1f) as usize;

      // 若 rd 为 31 且没有 set_flags，则代表 SP
      let rn_val = if rn == 31 {
        self.registers.sp()
      } else {
        self.registers.x(rn)
      };

      if is_64 {
        let (res, carry, overflow) = if is_sub {
          let (res, ovf) = (rn_val as i64).overflowing_sub(imm as i64);
          let carry = rn_val >= imm;
          (res as u64, carry, ovf)
        } else {
          let (res, carry) = rn_val.overflowing_add(imm);
          let (_, ovf) = (rn_val as i64).overflowing_add(imm as i64);
          (res, carry, ovf)
        };

        if set_flags {
          self.registers.nzcv.update_nzcv_64(res, carry, overflow);
        }
        if rd == 31 {
          if !set_flags {
            self.registers.set_sp(res);
          }
        } else {
          self.registers.set_x(rd, res);
        }
      } else {
        let rn32 = rn_val as u32;
        let imm32 = imm as u32;
        let (res, carry, overflow) = if is_sub {
          let (res, ovf) = (rn32 as i32).overflowing_sub(imm32 as i32);
          let carry = rn32 >= imm32;
          (res as u32, carry, ovf)
        } else {
          let (res, carry) = rn32.overflowing_add(imm32);
          let (_, ovf) = (rn32 as i32).overflowing_add(imm32 as i32);
          (res, carry, ovf)
        };

        if set_flags {
          self.registers.nzcv.update_nzcv_32(res, carry, overflow);
        }
        if rd == 31 {
          if !set_flags {
            self.registers.set_sp(res as u64);
          }
        } else {
          self.registers.set_w(rd, res);
        }
      }

      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 12. 逻辑与算术寄存器指令 (ADD/SUB/AND/ORR/EOR 移位寄存器)
    if (instr & 0x1f00_0000) == 0x0b00_0000 || (instr & 0x1f00_0000) == 0x0a00_0000 {
      let is_64 = (instr & 0x8000_0000) != 0;
      let op = (instr >> 29) & 0x3;
      let is_logic = (instr & 0x1f00_0000) == 0x0a00_0000;
      let rm = ((instr >> 16) & 0x1f) as usize;
      let rn = ((instr >> 5) & 0x1f) as usize;
      let rd = (instr & 0x1f) as usize;

      let rn_val = self.registers.reg(is_64, rn);
      let rm_val = self.registers.reg(is_64, rm);

      if is_logic {
        let res = match op {
          0 => rn_val & rm_val, // AND
          1 => rn_val | rm_val, // ORR (含 MOV rd, rm)
          2 => rn_val ^ rm_val, // EOR
          _ => rn_val & !rm_val,
        };
        self.registers.set_reg(is_64, rd, res);
      } else {
        let is_sub = (op & 2) != 0;
        let set_flags = (op & 1) != 0;

        if is_64 {
          let (res, carry, overflow) = if is_sub {
            let (res, ovf) = (rn_val as i64).overflowing_sub(rm_val as i64);
            (res as u64, rn_val >= rm_val, ovf)
          } else {
            let (res, carry) = rn_val.overflowing_add(rm_val);
            let (_, ovf) = (rn_val as i64).overflowing_add(rm_val as i64);
            (res, carry, ovf)
          };
          if set_flags {
            self.registers.nzcv.update_nzcv_64(res, carry, overflow);
          }
          if !set_flags || rd != 31 {
            self.registers.set_x(rd, res);
          }
        } else {
          let rn32 = rn_val as u32;
          let rm32 = rm_val as u32;
          let (res, carry, overflow) = if is_sub {
            let (res, ovf) = (rn32 as i32).overflowing_sub(rm32 as i32);
            (res as u32, rn32 >= rm32, ovf)
          } else {
            let (res, carry) = rn32.overflowing_add(rm32);
            let (_, ovf) = (rn32 as i32).overflowing_add(rm32 as i32);
            (res, carry, ovf)
          };
          if set_flags {
            self.registers.nzcv.update_nzcv_32(res, carry, overflow);
          }
          if !set_flags || rd != 31 {
            self.registers.set_w(rd, res);
          }
        }
      }

      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 13. 加载与存储 (LDR / STR)
    // 64-bit LDR (1111 1001 01ii iiiiiiiiiiii nnnnn ttttt)
    if (instr & 0xffc0_0000) == 0xf940_0000 {
      let imm12 = ((instr >> 10) & 0x0fff) as u64;
      let offset = imm12 * 8;
      let rn = ((instr >> 5) & 0x1f) as usize;
      let rt = (instr & 0x1f) as usize;
      let addr = self.mem_base(rn)?.wrapping_add(offset);
      let val = self.memory.read_u64(addr)?;
      self.registers.set_x(rt, val);
      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 64-bit STR (1111 1001 00ii iiiiiiiiiiii nnnnn ttttt)
    if (instr & 0xffc0_0000) == 0xf900_0000 {
      let imm12 = ((instr >> 10) & 0x0fff) as u64;
      let offset = imm12 * 8;
      let rn = ((instr >> 5) & 0x1f) as usize;
      let rt = (instr & 0x1f) as usize;
      let addr = self.mem_base(rn)?.wrapping_add(offset);
      let val = self.registers.x(rt);
      self.memory.write_u64(addr, val)?;
      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 32-bit LDR (1011 1001 01ii iiiiiiiiiiii nnnnn ttttt)
    if (instr & 0xffc0_0000) == 0xb940_0000 {
      let imm12 = ((instr >> 10) & 0x0fff) as u64;
      let offset = imm12 * 4;
      let rn = ((instr >> 5) & 0x1f) as usize;
      let rt = (instr & 0x1f) as usize;
      let addr = self.mem_base(rn)?.wrapping_add(offset);
      let val = self.memory.read_u32(addr)?;
      self.registers.set_w(rt, val);
      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 32-bit STR (1011 1001 00ii iiiiiiiiiiii nnnnn ttttt)
    if (instr & 0xffc0_0000) == 0xb900_0000 {
      let imm12 = ((instr >> 10) & 0x0fff) as u64;
      let offset = imm12 * 4;
      let rn = ((instr >> 5) & 0x1f) as usize;
      let rt = (instr & 0x1f) as usize;
      let addr = self.mem_base(rn)?.wrapping_add(offset);
      let val = self.registers.w(rt);
      self.memory.write_u32(addr, val)?;
      self.registers.set_pc(next_pc);
      return Ok(true);
    }

    // 14. HLT 指令 (1101 0100 010i iiiiiiiiiiiiiiii 000 00)
    if (instr & 0xffe0_001f) == 0xd440_0000 {
      let imm16 = (instr >> 5) & 0xffff;
      return Err(Error::Halted(format!("HLT #{imm16} at PC 0x{pc:x}")));
    }

    // 未知或未实现指令
    Err(Error::UnimplementedInstruction {
      instruction: instr,
      pc,
    })
  }

  /// 运行模拟器直到执行完毕或超出循环限制
  pub fn run(&mut self, max_cycles: Option<u64>) -> Result<u64> {
    let limit = max_cycles.unwrap_or(10_000_000);
    let mut cycles = 0;

    while self.registers.pc() != K_END_OF_SIM_ADDRESS {
      if cycles >= limit {
        return Err(Error::CycleLimitExceeded { limit });
      }
      cycles += 1;
      if !self.step()? {
        break;
      }
    }

    Ok(cycles)
  }

  /// 依据 AAPCS64 调用约定调用指定入口函数
  ///
  /// - 前 8 个参数装入 x0..x7
  /// - 超过 8 个的参数压栈
  /// - lr 设为 0 (K_END_OF_SIM_ADDRESS)
  /// - 执行完成后返回 x0
  pub fn call(&mut self, entry: u64, args: &[u64]) -> Result<u64> {
    // 校验进入函数时 SP 的 16 字节对齐 (AAPCS64 契约)
    let original_sp = self.registers.sp();
    self.memory.check_alignment(original_sp, STACK_ALIGNMENT)?;

    // 1. 设置寄存器参数 (x0..x7)
    for (i, &arg) in args.iter().take(8).enumerate() {
      self.registers.set_x(i, arg);
    }

    // 2. 栈参数处理
    if args.len() > 8 {
      let stack_args = &args[8..];
      let bytes = stack_args.len() * 8;
      // 16 字节对齐
      let aligned_bytes = (bytes + STACK_ALIGNMENT - 1) & !(STACK_ALIGNMENT - 1);
      let entry_sp = original_sp.wrapping_sub(aligned_bytes as u64);
      self.registers.set_sp(entry_sp);

      for (i, &arg) in stack_args.iter().enumerate() {
        let addr = entry_sp.wrapping_add((i * 8) as u64);
        self.memory.write_u64(addr, arg)?;
      }
    }

    // 3. 设置入口与返回哨兵
    self.registers.set_pc(entry);
    self.registers.set_lr(K_END_OF_SIM_ADDRESS);

    // 4. 执行模拟循环
    self.run(None)?;

    // 5. 恢复栈指针
    self.registers.set_sp(original_sp);

    // 6. 返回 x0
    Ok(self.registers.x(0))
  }
}

/// 将 4 位条件码转为 Condition 枚举
const fn decode_condition(code: u8) -> Option<Condition> {
  match code & 0xf {
    0x0 => Some(Condition::Eq),
    0x1 => Some(Condition::Ne),
    0x2 => Some(Condition::Cs),
    0x3 => Some(Condition::Cc),
    0x4 => Some(Condition::Mi),
    0x5 => Some(Condition::Pl),
    0x6 => Some(Condition::Vs),
    0x7 => Some(Condition::Vc),
    0x8 => Some(Condition::Hi),
    0x9 => Some(Condition::Ls),
    0xa => Some(Condition::Ge),
    0xb => Some(Condition::Lt),
    0xc => Some(Condition::Gt),
    0xd => Some(Condition::Le),
    0xe => Some(Condition::Al),
    0xf => Some(Condition::Nv),
    _ => None,
  }
}
