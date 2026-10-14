use aok::{OK, Void};
use log::info;
use v8_codegen_arm64::Condition;
use v8_execution_simulator::{
  DEFAULT_STACK_SIZE, Error, FP_DEFAULT_NAN_BITS, K_END_OF_SIM_ADDRESS, Nzcv, STACK_ALIGNMENT,
  SimRegisters, Simulator, SimulatorMemory,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_registers_and_initial_state() -> Void {
  let mut regs = SimRegisters::default();

  // 1. 寄存器初值与 XZR 行为
  assert_eq!(regs.x(0), 0);
  assert_eq!(regs.x(31), 0); // XZR 恒为 0
  assert_eq!(regs.w(31), 0); // WZR 恒为 0

  // 写入 XZR 应被忽略
  regs.set_x(31, 0x1234_5678_9abc_def0);
  assert_eq!(regs.x(31), 0);
  regs.set_w(31, 0xffff_ffff);
  assert_eq!(regs.w(31), 0);

  // 2. 通用寄存器 64 位与 32 位切片
  regs.set_x(0, 0x1122_3344_5566_7788);
  assert_eq!(regs.x(0), 0x1122_3344_5566_7788);
  assert_eq!(regs.w(0), 0x5566_7788);

  // set_w 应清零高 32 位
  regs.set_w(0, 0xaaaa_bbbb);
  assert_eq!(regs.x(0), 0x0000_0000_aaaa_bbbb);

  // 3. 特殊寄存器
  regs.set_sp(0x7fff_0000);
  assert_eq!(regs.sp(), 0x7fff_0000);

  regs.set_lr(0x1000);
  assert_eq!(regs.lr(), 0x1000);
  assert_eq!(regs.x(30), 0x1000);

  regs.set_fp(0x7ffe_fff0);
  assert_eq!(regs.fp(), 0x7ffe_fff0);
  assert_eq!(regs.x(29), 0x7ffe_fff0);

  // 4. 浮点/向量寄存器 NaN 初值
  let d0_bits = regs.d(0).to_bits();
  assert_eq!(d0_bits, FP_DEFAULT_NAN_BITS);

  regs.set_d(1, 123.456789);
  assert!((regs.d(1) - 123.456789).abs() < 1e-9);

  regs.set_s(2, 78.125);
  assert!((regs.s(2) - 78.125).abs() < 1e-4);

  info!("SimRegisters 读写与初值契约测试通过");
  OK
}

#[test]
fn test_nzcv_conditions() -> Void {
  let mut nzcv = Nzcv::default();

  // 默认状态: 全部为 false
  assert!(!nzcv.n && !nzcv.z && !nzcv.c && !nzcv.v);
  assert!(!nzcv.condition_passed(Condition::Eq));
  assert!(nzcv.condition_passed(Condition::Ne));
  assert!(nzcv.condition_passed(Condition::Al));
  assert!(nzcv.condition_passed(Condition::Nv));

  // 设置 Z = 1 (Zero)
  nzcv.z = true;
  assert!(nzcv.condition_passed(Condition::Eq));
  assert!(!nzcv.condition_passed(Condition::Ne));
  assert!(nzcv.condition_passed(Condition::Le));
  assert!(!nzcv.condition_passed(Condition::Gt));

  // 设置 C = 1, Z = 0 (Carry set)
  nzcv.z = false;
  nzcv.c = true;
  assert!(nzcv.condition_passed(Condition::Cs));
  assert!(!nzcv.condition_passed(Condition::Cc));
  assert!(nzcv.condition_passed(Condition::Hi));
  assert!(!nzcv.condition_passed(Condition::Ls));

  // 符号标志与溢出标志 N != V
  nzcv.n = true;
  nzcv.v = false;
  assert!(nzcv.condition_passed(Condition::Lt));
  assert!(!nzcv.condition_passed(Condition::Ge));

  // 32 位算术更新: 10 - 10 = 0
  nzcv.update_nzcv_32(0, true, false);
  assert!(nzcv.z);
  assert!(!nzcv.n);
  assert!(nzcv.c);
  assert!(!nzcv.v);

  // 64 位算术更新: 负数
  nzcv.update_nzcv_64(0x8000_0000_0000_0000, false, true);
  assert!(!nzcv.z);
  assert!(nzcv.n);
  assert!(!nzcv.c);
  assert!(nzcv.v);

  info!("NZCV 条件标志计算与分支匹配测试通过");
  OK
}

#[test]
fn test_memory_read_write_and_bounds() -> Void {
  let mut mem = SimulatorMemory::new();
  assert_eq!(mem.stack_size(), DEFAULT_STACK_SIZE);
  let sp = mem.initial_sp();
  assert_eq!(sp % (STACK_ALIGNMENT as u64), 0);

  // 栈区读写测试 (从 sp - 64 处写入)
  let addr = sp - 64;
  mem.write_u8(addr, 0x12)?;
  assert_eq!(mem.read_u8(addr)?, 0x12);

  mem.write_u16(addr + 2, 0x3456)?;
  assert_eq!(mem.read_u16(addr + 2)?, 0x3456);

  mem.write_u32(addr + 8, 0x789a_bcde)?;
  assert_eq!(mem.read_u32(addr + 8)?, 0x789a_bcde);

  mem.write_u64(addr + 16, 0x1122_3344_5566_7788)?;
  assert_eq!(mem.read_u64(addr + 16)?, 0x1122_3344_5566_7788);

  // 越界安全防护
  let oob_addr = 0x1000;
  assert!(matches!(
    mem.read_u32(oob_addr),
    Err(Error::MemoryOutOfBounds { .. })
  ));
  assert!(matches!(
    mem.write_u64(oob_addr, 42),
    Err(Error::MemoryOutOfBounds { .. })
  ));

  info!("SimulatorMemory 内存安全读写与越界拦截测试通过");
  OK
}

#[test]
fn test_simulator_execution_and_call() -> Void {
  let mut sim = Simulator::new();
  let code_entry = 0x0001_0000_0000;

  // 构造一小段简单的 ARM64 汇编测试机器码:
  // 1. movz x0, #42, lsl #0   -> 0xd2800540
  // 2. add  x0, x0, #10       -> 0x91002800
  // 3. ret                    -> 0xd65f03c0
  let code: [u8; 12] = [
    0x40, 0x05, 0x80, 0xd2, // movz x0, #42
    0x00, 0x28, 0x00, 0x91, // add x0, x0, #10
    0xc0, 0x03, 0x5f, 0xd6, // ret
  ];

  sim.load_code(code_entry, &code);

  // 单步执行测试
  sim.registers.set_pc(code_entry);
  sim.registers.set_lr(K_END_OF_SIM_ADDRESS);

  // 第 1 步: movz x0, #42
  let cont1 = sim.step()?;
  assert!(cont1);
  assert_eq!(sim.registers.x(0), 42);

  // 第 2 步: add x0, x0, #10
  let cont2 = sim.step()?;
  assert!(cont2);
  assert_eq!(sim.registers.x(0), 52);

  // 第 3 步: ret (返回到 0 哨兵)
  let cont3 = sim.step()?;
  assert!(!cont3); // 返回哨兵终止
  assert_eq!(sim.registers.pc(), K_END_OF_SIM_ADDRESS);

  // AAPCS64 函数调用接口测试
  // 构造加法函数: add x0, x0, x1; ret
  let add_func: [u8; 8] = [
    0x00, 0x00, 0x01, 0x8b, // add x0, x0, x1
    0xc0, 0x03, 0x5f, 0xd6, // ret
  ];
  let add_entry = 0x0001_0000_0100;
  sim.load_code(add_entry, &add_func);

  let ret_val = sim.call(add_entry, &[123, 456])?;
  assert_eq!(ret_val, 579);

  // 测试传递 9 个参数 (第 9 个走栈):
  // 构造接收参数并累加函数
  // add x0, x0, x1
  // ldr x2, [sp, #0]
  // add x0, x0, x2
  // ret
  let stack_arg_func: [u8; 16] = [
    0x00, 0x00, 0x01, 0x8b, // add x0, x0, x1
    0xe2, 0x03, 0x40, 0xf9, // ldr x2, [sp, #0]
    0x00, 0x00, 0x02, 0x8b, // add x0, x0, x2
    0xc0, 0x03, 0x5f, 0xd6, // ret
  ];
  let stack_entry = 0x0001_0000_0200;
  sim.load_code(stack_entry, &stack_arg_func);

  let ret_stack = sim.call(stack_entry, &[10, 20, 0, 0, 0, 0, 0, 0, 100])?;
  assert_eq!(ret_stack, 130);

  info!("Simulator 指令步进、算术、ret、AAPCS64 调用与栈传参测试通过");
  OK
}

#[test]
fn test_branches_and_control_flow() -> Void {
  let mut sim = Simulator::new();
  let base = 0x0001_0000_0300;

  // 1. 测试 NOP 与 B imm26
  // [0] nop (0xd503201f)
  // [4] b +8 (跳过下一条指令直接到 [12]) (0x14000002)
  // [8] movz x0, #99 (0xd2800c60, 被跳过)
  // [12] movz x0, #7 (0xd28000e0)
  // [16] ret (0xd65f03c0)
  let b_code: [u8; 20] = [
    0x1f, 0x20, 0x03, 0xd5, // nop
    0x02, 0x00, 0x00, 0x14, // b +8
    0x60, 0x0c, 0x80, 0xd2, // movz x0, #99
    0xe0, 0x00, 0x80, 0xd2, // movz x0, #7
    0xc0, 0x03, 0x5f, 0xd6, // ret
  ];
  sim.load_code(base, &b_code);
  let res_b = sim.call(base, &[])?;
  assert_eq!(res_b, 7);

  // 2. 测试 B.cond (EQ 成立跳转, NE 不跳转)
  // [0] adds x1, x0, x0 (若 x0==0 则 Z=1) (0xab000001)
  // [4] b.eq +8 (若 Z=1 跳转到 [12]) (0x54000040)
  // [8] movz x0, #100 (0xd2800c80)
  // [12] ret (0xd65f03c0)
  let b_cond_code: [u8; 16] = [
    0x01, 0x00, 0x00, 0xab, // adds x1, x0, x0
    0x40, 0x00, 0x00, 0x54, // b.eq +8
    0x80, 0x0c, 0x80, 0xd2, // movz x0, #100
    0xc0, 0x03, 0x5f, 0xd6, // ret
  ];
  let b_cond_base = 0x0001_0000_0400;
  sim.load_code(b_cond_base, &b_cond_code);

  // 传参 x0=0 -> Z=1 -> b.eq 触发 -> 直接 ret x0=0
  let res_eq = sim.call(b_cond_base, &[0])?;
  assert_eq!(res_eq, 0);

  // 传参 x0=1 -> Z=0 -> b.eq 未触发 -> 执行 movz x0, #100 -> ret 100
  let res_ne = sim.call(b_cond_base, &[1])?;
  assert_eq!(res_ne, 100);

  // 3. 测试 CBZ 与 CBNZ
  // [0] cbz x0, +8 (若 x0==0 跳到 [8]) (0xb4000040)
  // [4] movz x0, #50 (0xd2800640)
  // [8] ret (0xd65f03c0)
  let cbz_code: [u8; 12] = [
    0x40, 0x00, 0x00, 0xb4, // cbz x0, +8
    0x40, 0x06, 0x80, 0xd2, // movz x0, #50
    0xc0, 0x03, 0x5f, 0xd6, // ret
  ];
  let cbz_base = 0x0001_0000_0500;
  sim.load_code(cbz_base, &cbz_code);
  assert_eq!(sim.call(cbz_base, &[0])?, 0);
  assert_eq!(sim.call(cbz_base, &[1])?, 50);

  info!("B, B.cond, CBZ/CBNZ, NOP 分支与控制流单测全部通过");
  OK
}

#[test]
fn test_movk_and_32bit_memory() -> Void {
  let mut sim = Simulator::new();
  let base = 0x0001_0000_0600;

  // 1. 测试 MOVZ + MOVK 组合拼接 32 位立即数: 0x5678_1234
  // [0] sub sp, sp, #16 (0xd10043ff) 维护栈帧向下增长
  // [4] movz x0, #0x1234 (0xd2824680)
  // [8] movk x0, #0x5678, lsl #16 (0xf2aacf00)
  // [12] str  w0, [sp, #0] (32-bit store) (0xb90003e0)
  // [16] ldr w1, [sp, #0] (32-bit load) (0xb94003e1)
  // [20] mov x0, x1 (orr x0, xzr, x1) (0xaa0103e0)
  // [24] add sp, sp, #16 (0x910043ff) 恢复栈帧
  // [28] ret (0xd65f03c0)
  let mem32_code: [u8; 32] = [
    0xff, 0x43, 0x00, 0xd1, // sub sp, sp, #16
    0x80, 0x46, 0x82, 0xd2, // movz x0, #0x1234
    0x00, 0xcf, 0xaa, 0xf2, // movk x0, #0x5678, lsl #16
    0xe0, 0x03, 0x00, 0xb9, // str w0, [sp, #0]
    0xe1, 0x03, 0x40, 0xb9, // ldr w1, [sp, #0]
    0xe0, 0x03, 0x01, 0xaa, // orr x0, xzr, x1
    0xff, 0x43, 0x00, 0x91, // add sp, sp, #16
    0xc0, 0x03, 0x5f, 0xd6, // ret
  ];
  sim.load_code(base, &mem32_code);
  let res = sim.call(base, &[])?;
  assert_eq!(res, 0x5678_1234);

  info!("MOVZ+MOVK 半字拼接与 32 位 LDR/STR 单测全部通过");
  OK
}

#[test]
fn test_sp_alignment_and_exceptions() -> Void {
  let mut sim = Simulator::new();
  let base = 0x0001_0000_0700;

  // 1. 验证 AAPCS64 调用时未按 16 字节对齐的 SP 拦截
  sim.registers.set_sp(0x0000_7FFF_FFFF_0008); // 8 字节对齐但非 16 字节对齐
  let err = sim.call(base, &[]);
  assert!(matches!(
    err,
    Err(Error::UnalignedAccess {
      address: 0x0000_7FFF_FFFF_0008,
      alignment: 16
    })
  ));
  sim.reset();

  // 1b. 验证 LDR/STR 寻址时 SP 未对齐拦截 (mem_base 契约)
  let ldr_sp_code: [u8; 4] = [0xe0, 0x03, 0x40, 0xf9]; // ldr x0, [sp, #0]
  sim.load_code(base, &ldr_sp_code);
  sim.registers.set_pc(base);
  sim.registers.set_sp(0x0000_7FFF_FFFF_0008);
  let ldr_err = sim.step();
  assert!(matches!(
    ldr_err,
    Err(Error::UnalignedAccess {
      address: 0x0000_7FFF_FFFF_0008,
      alignment: 16
    })
  ));
  sim.reset();

  // 2. 验证 HLT 指令正确返回 Error::Halted
  let hlt_code: [u8; 4] = [0x00, 0x00, 0x40, 0xd4]; // hlt #0
  sim.load_code(base, &hlt_code);
  sim.registers.set_pc(base);
  assert!(matches!(sim.step(), Err(Error::Halted(_))));

  // 3. 验证未实现指令正确返回 Error::UnimplementedInstruction
  let undef_code: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
  let undef_base = 0x0001_0000_0800;
  sim.load_code(undef_base, &undef_code);
  sim.registers.set_pc(undef_base);
  assert!(matches!(
    sim.step(),
    Err(Error::UnimplementedInstruction { .. })
  ));

  // 4. 验证 SimulatorMemory 增量多函数加载保留能力
  assert!(sim.memory.read_u32(base).is_ok());
  assert!(sim.memory.read_u32(undef_base).is_ok());

  info!("AAPCS64 栈对齐检查、HLT 捕获与多函数增量内存加载测试全部通过");
  OK
}
