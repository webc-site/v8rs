use aok::{OK, Void};
use log::info;
use v8_codegen_core::{AssemblerOptions, Distance, Label, RelocMode};
use v8_codegen_x64::{
  Assembler, Condition, Immediate, K_ROOT_REGISTER_BIAS, Operand, R8, R9, R12, R13, R15, RAX, RBP,
  RBX, RCX, RDI, RDX, RSI, RSP, RegList, ScaleFactor, XMM0, XMM7, XMM15,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_registers_and_reglist() -> Void {
  info!("测试 Register 与 RegList 编号与位图");

  // 基础通用寄存器编码
  assert_eq!(RAX.code(), 0);
  assert_eq!(RCX.code(), 1);
  assert_eq!(RDX.code(), 2);
  assert_eq!(RBX.code(), 3);
  assert_eq!(RSP.code(), 4);
  assert_eq!(RBP.code(), 5);
  assert_eq!(RSI.code(), 6);
  assert_eq!(RDI.code(), 7);
  assert_eq!(R8.code(), 8);
  assert_eq!(R15.code(), 15);

  assert!(RAX.is_valid());
  assert!(R15.is_valid());

  // 字节寄存器判定 (rax..rbx 为 true)
  assert!(RAX.is_byte_register());
  assert!(RBX.is_byte_register());
  assert!(!RSP.is_byte_register());
  assert!(!R8.is_byte_register());

  // 高位与低位提取
  assert_eq!(RAX.high_bit(), 0);
  assert_eq!(RAX.low_bits(), 0);
  assert_eq!(R8.high_bit(), 1);
  assert_eq!(R8.low_bits(), 0);
  assert_eq!(R15.high_bit(), 1);
  assert_eq!(R15.low_bits(), 7);

  // XMM 寄存器
  assert_eq!(XMM0.code(), 0);
  assert_eq!(XMM7.code(), 7);
  assert_eq!(XMM15.code(), 15);
  assert_eq!(XMM15.high_bit(), 1);
  assert_eq!(XMM15.low_bits(), 7);

  // RegList 集合操作
  let mut list = RegList::empty();
  assert!(list.is_empty());
  assert_eq!(list.len(), 0);

  list.set(RAX);
  list.set(R8);
  assert!(list.has(RAX));
  assert!(list.has(R8));
  assert!(!list.has(RBX));
  assert_eq!(list.len(), 2);
  assert_eq!(list.bits(), (1 << 0) | (1 << 8));

  list.clear(RAX);
  assert!(!list.has(RAX));
  assert_eq!(list.len(), 1);

  let other = RegList::from_slice(&[RAX, RBX]);
  let unioned = list.union(other);
  assert!(unioned.has(RAX));
  assert!(unioned.has(RBX));
  assert!(unioned.has(R8));

  let intersected = unioned.intersect(other);
  assert_eq!(intersected.len(), 2);
  assert!(intersected.has(RAX));
  assert!(intersected.has(RBX));
  assert!(!intersected.has(R8));

  OK
}

#[test]
fn test_operands_encoding() -> Void {
  info!("测试 Operand 内存操作数寻址字节编码");

  // [rax] (无位移)
  let op_rax = Operand::base(RAX);
  assert_eq!(op_rax.rex, 0);
  assert_eq!(op_rax.buf[0], 0x00);
  assert_eq!(op_rax.len, 1);

  // [rbx + 0x12] (8 位正位移)
  let op_disp8 = Operand::base_disp(RBX, 0x12);
  assert_eq!(op_disp8.rex, 0);
  assert_eq!(op_disp8.buf[0], 0x43); // mod=01, rm=3
  assert_eq!(op_disp8.buf[1], 0x12);
  assert_eq!(op_disp8.len, 2);

  // [rbp] (位移为 0 必须编码为 mod=01, disp8=0)
  let op_rbp = Operand::base(RBP);
  assert_eq!(op_rbp.rex, 0);
  assert_eq!(op_rbp.buf[0], 0x45); // mod=01, rm=5
  assert_eq!(op_rbp.buf[1], 0x00);
  assert_eq!(op_rbp.len, 2);

  // [rsp] (基址为 rsp 必须携带 SIB 字节)
  let op_rsp = Operand::base(RSP);
  assert_eq!(op_rsp.buf[0], 0x04); // mod=00, rm=4 (SIB)
  assert_eq!(op_rsp.buf[1], 0x24); // scale=0, index=rsp/4, base=rsp/4
  assert_eq!(op_rsp.len, 2);

  // [r12] (r12 low_bits 为 4, 必须携带 SIB 且 REX.B 置位)
  let op_r12 = Operand::base(R12);
  assert_eq!(op_r12.rex, 1); // REX.B = 1
  assert_eq!(op_r12.buf[0], 0x04);
  assert_eq!(op_r12.buf[1], 0x24);
  assert_eq!(op_r12.len, 2);

  // [rax + rbx * 4 + 0x1000] (带比例变址与 32 位位移)
  let op_sib = Operand::base_index_scale_disp(RAX, RBX, ScaleFactor::Times4, 0x1000);
  assert_eq!(op_sib.buf[0], 0x84); // mod=10, rm=4
  assert_eq!(op_sib.buf[1], (2 << 6) | (3 << 3)); // scale=times_4(2), index=rbx(3), base=rax(0)
  assert_eq!(&op_sib.buf[2..6], &0x1000i32.to_le_bytes());
  assert_eq!(op_sib.len, 6);

  // 立即数判别
  let imm_small = Immediate::new(42);
  assert!(imm_small.is_int8());
  let imm_big = Immediate::new(0x123456);
  assert!(!imm_big.is_int8());

  // RIP 相对寻址
  let op_rip = Operand::rip_disp32(0x12345678);
  assert_eq!(op_rip.buf[0], 0x05);
  assert_eq!(&op_rip.buf[1..5], &0x12345678i32.to_le_bytes());
  assert_eq!(op_rip.len, 5);
  assert_eq!(op_rip.rex, 0);

  // 无基址变址寻址 [rbx * 8 + 0x2000]
  let op_no_base = Operand::index_scale_disp(RBX, ScaleFactor::Times8, 0x2000);
  assert_eq!(op_no_base.buf[0], 0x04);
  assert_eq!(op_no_base.buf[1], (3 << 6) | (3 << 3) | 5);
  assert_eq!(&op_no_base.buf[2..6], &0x2000i32.to_le_bytes());
  assert_eq!(op_no_base.len, 6);
  assert_eq!(op_no_base.rex, 0);

  OK
}

#[test]
fn test_mov_instructions_encoding() -> Void {
  info!("测试 movq 与 movl 指令二进制编码比对");

  let mut masm = Assembler::new(AssemblerOptions::default());

  // movq rax, rbx -> 48 8b c3 (原版 V8 1:1 对齐 0x8B dst, src)
  masm.movq(RAX, RBX)?;
  assert_eq!(masm.buffer_slice(), &[0x48, 0x8b, 0xc3]);

  // movq rax, [rbx + 0x10] -> 48 8b 43 10
  let mark = masm.pc_offset();
  masm.movq(RAX, Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x8b, 0x43, 0x10]);

  // movq [rbx + 0x10], rax -> 48 89 43 10
  let mark = masm.pc_offset();
  masm.movq(Operand::base_disp(RBX, 0x10), RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x89, 0x43, 0x10]);

  // movq rax, imm32 -> 48 c7 c0 2a 00 00 00
  let mark = masm.pc_offset();
  masm.movq(RAX, Immediate::new(42))?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0x48, 0xc7, 0xc0, 0x2a, 0x00, 0x00, 0x00]
  );

  // movq rax, imm64 -> 48 b8 ...
  let mark = masm.pc_offset();
  masm.movq(RAX, 0x1122334455667788i64)?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0x48, 0xb8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]
  );

  // movq [rbx + 0x10], imm32 -> 48 c7 43 10 2a 00 00 00
  let mark = masm.pc_offset();
  masm.movq(Operand::base_disp(RBX, 0x10), Immediate::new(42))?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0x48, 0xc7, 0x43, 0x10, 0x2a, 0x00, 0x00, 0x00]
  );

  // movl rax, rbx -> 8b c3 (原版 V8 1:1 对齐 0x8B dst, src)
  let mark = masm.pc_offset();
  masm.movl(RAX, RBX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x8b, 0xc3]);

  // movl [rbx + 0x10], rax -> 89 43 10
  let mark = masm.pc_offset();
  masm.movl(Operand::base_disp(RBX, 0x10), RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x89, 0x43, 0x10]);

  // movl rax, [rbx + 0x10] -> 8b 43 10
  let mark = masm.pc_offset();
  masm.movl(RAX, Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x8b, 0x43, 0x10]);

  // movl rax, imm32 -> b8 64 00 00 00
  let mark = masm.pc_offset();
  masm.movl(RAX, Immediate::new(100))?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0xb8, 0x64, 0x00, 0x00, 0x00]
  );

  // movl [rbx + 0x10], imm32 -> c7 43 10 64 00 00 00
  let mark = masm.pc_offset();
  masm.movl(Operand::base_disp(RBX, 0x10), Immediate::new(100))?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0xc7, 0x43, 0x10, 0x64, 0x00, 0x00, 0x00]
  );

  OK
}

#[test]
fn test_arithmetic_instructions_encoding() -> Void {
  info!("测试 64 位核心算术逻辑指令二进制编码比对");

  let mut masm = Assembler::new(AssemblerOptions::default());

  // addq rax, rbx -> 48 03 c3
  let mark = masm.pc_offset();
  masm.addq(RAX, RBX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x03, 0xc3]);

  // addq rax, imm8 -> 48 83 c0 01
  let mark = masm.pc_offset();
  masm.addq(RAX, Immediate::new(1))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x83, 0xc0, 0x01]);

  // subq rax, rbx -> 48 2b c3
  let mark = masm.pc_offset();
  masm.subq(RAX, RBX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x2b, 0xc3]);

  // subq rax, imm8 -> 48 83 e8 05
  let mark = masm.pc_offset();
  masm.subq(RAX, Immediate::new(5))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x83, 0xe8, 0x05]);

  // imulq rax, rbx -> 48 0f af c3
  let mark = masm.pc_offset();
  masm.imulq(RAX, RBX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x0f, 0xaf, 0xc3]);

  // andq rax, rbx -> 48 23 c3
  let mark = masm.pc_offset();
  masm.andq(RAX, RBX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x23, 0xc3]);

  // orq rax, rbx -> 48 0b c3
  let mark = masm.pc_offset();
  masm.orq(RAX, RBX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x0b, 0xc3]);

  // xorq rax, rax -> 48 33 c0
  let mark = masm.pc_offset();
  masm.xorq(RAX, RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x33, 0xc0]);

  // cmpq rax, rbx -> 48 3b c3
  let mark = masm.pc_offset();
  masm.cmpq(RAX, RBX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x3b, 0xc3]);

  // testq rax, rbx -> 48 85 c3
  let mark = masm.pc_offset();
  masm.testq(RAX, RBX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x85, 0xc3]);

  // testq rax, imm32 (rax 专用紧凑编码 0xA9)
  let mark = masm.pc_offset();
  masm.testq(RAX, Immediate::new(0x1234))?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0x48, 0xa9, 0x34, 0x12, 0x00, 0x00]
  );

  // testq rbx, imm32 (通用编码 0xF7 /0)
  let mark = masm.pc_offset();
  masm.testq(RBX, Immediate::new(0x1234))?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0x48, 0xf7, 0xc3, 0x34, 0x12, 0x00, 0x00]
  );

  // 内存操作数算术/逻辑测试
  // addq [rbx + 0x10], rax -> 48 01 43 10
  let mark = masm.pc_offset();
  masm.addq(Operand::base_disp(RBX, 0x10), RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x01, 0x43, 0x10]);

  // addq rax, [rbx + 0x10] -> 48 03 43 10
  let mark = masm.pc_offset();
  masm.addq(RAX, Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x03, 0x43, 0x10]);

  // addq [rbx + 0x10], imm8 -> 48 83 43 10 01
  let mark = masm.pc_offset();
  masm.addq(Operand::base_disp(RBX, 0x10), Immediate::new(1))?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0x48, 0x83, 0x43, 0x10, 0x01]
  );

  // subq [rbx + 0x10], rax -> 48 29 43 10
  let mark = masm.pc_offset();
  masm.subq(Operand::base_disp(RBX, 0x10), RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x29, 0x43, 0x10]);

  // subq rax, [rbx + 0x10] -> 48 2b 43 10
  let mark = masm.pc_offset();
  masm.subq(RAX, Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x2b, 0x43, 0x10]);

  // subq [rbx + 0x10], imm8 -> 48 83 6b 10 05
  let mark = masm.pc_offset();
  masm.subq(Operand::base_disp(RBX, 0x10), Immediate::new(5))?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0x48, 0x83, 0x6b, 0x10, 0x05]
  );

  // andq [rbx + 0x10], rax -> 48 21 43 10
  let mark = masm.pc_offset();
  masm.andq(Operand::base_disp(RBX, 0x10), RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x21, 0x43, 0x10]);

  // andq rax, [rbx + 0x10] -> 48 23 43 10
  let mark = masm.pc_offset();
  masm.andq(RAX, Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x23, 0x43, 0x10]);

  // orq [rbx + 0x10], rax -> 48 09 43 10
  let mark = masm.pc_offset();
  masm.orq(Operand::base_disp(RBX, 0x10), RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x09, 0x43, 0x10]);

  // orq rax, [rbx + 0x10] -> 48 0b 43 10
  let mark = masm.pc_offset();
  masm.orq(RAX, Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x0b, 0x43, 0x10]);

  // xorq [rbx + 0x10], rax -> 48 31 43 10
  let mark = masm.pc_offset();
  masm.xorq(Operand::base_disp(RBX, 0x10), RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x31, 0x43, 0x10]);

  // xorq rax, [rbx + 0x10] -> 48 33 43 10
  let mark = masm.pc_offset();
  masm.xorq(RAX, Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x33, 0x43, 0x10]);

  // cmpq [rbx + 0x10], rax -> 48 39 43 10
  let mark = masm.pc_offset();
  masm.cmpq(Operand::base_disp(RBX, 0x10), RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x39, 0x43, 0x10]);

  // cmpq rax, [rbx + 0x10] -> 48 3b 43 10
  let mark = masm.pc_offset();
  masm.cmpq(RAX, Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x3b, 0x43, 0x10]);

  // cmpq [rbx + 0x10], imm8 -> 48 83 7b 10 01
  let mark = masm.pc_offset();
  masm.cmpq(Operand::base_disp(RBX, 0x10), Immediate::new(1))?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0x48, 0x83, 0x7b, 0x10, 0x01]
  );

  // testq [rbx + 0x10], rax -> 48 85 43 10
  let mark = masm.pc_offset();
  masm.testq(Operand::base_disp(RBX, 0x10), RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x48, 0x85, 0x43, 0x10]);

  // testq [rbx + 0x10], imm32 -> 48 f7 43 10 34 12 00 00
  let mark = masm.pc_offset();
  masm.testq(Operand::base_disp(RBX, 0x10), Immediate::new(0x1234))?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0x48, 0xf7, 0x43, 0x10, 0x34, 0x12, 0x00, 0x00]
  );

  OK
}

#[test]
fn test_stack_and_control_instructions() -> Void {
  info!("测试 pushq, popq, call, ret, nop 指令编码");

  let mut masm = Assembler::new(AssemblerOptions::default());

  // pushq rax -> 50
  let mark = masm.pc_offset();
  masm.pushq(RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x50]);

  // pushq r12 (需要 REX.B) -> 41 54
  let mark = masm.pc_offset();
  masm.pushq(R12)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x41, 0x54]);

  // pushq imm8 -> 6a 0a
  let mark = masm.pc_offset();
  masm.pushq(Immediate::new(10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x6a, 0x0a]);

  // r12 D1 回归: pushq/imul 原版仅查值域不查 rmode (assembler-x64.cc:2362,1548,1562)——
  // int8 值 + rmode≠NoInfo 仍折 imm8 并弃 reloc (原版怪癖照抄, 与 cc:928/946 的
  // immediate_arithmetic_op 双查支相区分)
  let mark = masm.pc_offset();
  masm.pushq(Immediate::with_rmode(10, RelocMode::CodeTarget))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x6a, 0x0a]);
  assert!(Immediate::with_rmode(10, RelocMode::CodeTarget).is_int8_value());
  assert!(!Immediate::with_rmode(10, RelocMode::CodeTarget).is_int8());

  // popq rax -> 58
  let mark = masm.pc_offset();
  masm.popq(RAX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x58]);

  // popq r13 -> 41 5d
  let mark = masm.pc_offset();
  masm.popq(R13)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x41, 0x5d]);

  // ret -> c3
  let mark = masm.pc_offset();
  masm.ret()?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0xc3]);

  // ret_imm 8 -> c2 08 00
  let mark = masm.pc_offset();
  masm.ret_imm(8)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0xc2, 0x08, 0x00]);

  // nop -> 90
  let mark = masm.pc_offset();
  masm.nop()?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x90]);

  // call rbx -> ff d3
  let mark = masm.pc_offset();
  masm.call(RBX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0xff, 0xd3]);

  // jmp rdx -> ff e2
  let mark = masm.pc_offset();
  masm.jmp(RDX)?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0xff, 0xe2]);

  // pushq [rbx + 0x10] -> ff 73 10
  let mark = masm.pc_offset();
  masm.pushq(Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0xff, 0x73, 0x10]);

  // popq [rbx + 0x10] -> 8f 43 10
  let mark = masm.pc_offset();
  masm.popq(Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0x8f, 0x43, 0x10]);

  // call [rbx + 0x10] -> ff 53 10
  let mark = masm.pc_offset();
  masm.call(Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0xff, 0x53, 0x10]);

  // jmp [rbx + 0x10] -> ff 63 10
  let mark = masm.pc_offset();
  masm.jmp(Operand::base_disp(RBX, 0x10))?;
  assert_eq!(&masm.buffer_slice()[mark..], &[0xff, 0x63, 0x10]);

  // call &mut Label (前向跳转链表修正)
  let mut call_lbl = Label::new();
  let mark = masm.pc_offset();
  masm.call(&mut call_lbl)?;
  masm.nop()?;
  masm.bind(&mut call_lbl)?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0xe8, 0x01, 0x00, 0x00, 0x00, 0x90]
  );

  OK
}

#[test]
fn test_label_forward_and_backward_patching() -> Void {
  info!("测试 Label 标号近跳转/远跳转链表与相对位移自动回填");

  let mut masm = Assembler::new(AssemblerOptions::default());

  // 1. 近距离前向跳转 (Distance::Near)
  let mut forward_label = Label::new();
  masm.jmp_label(&mut forward_label, Distance::Near)?; // pos 0..2: EB 00
  masm.nop()?; // pos 2: 90
  masm.bind(&mut forward_label)?; // bound at pos 3
  // 目标位置 3 - (跳转后 pc 2) = +1
  assert_eq!(masm.buffer_slice(), &[0xeb, 0x01, 0x90]);

  // 2. 远距离前向跳转 (Distance::Far)
  let mut far_label = Label::new();
  let far_start = masm.pc_offset();
  masm.jmp(&mut far_label)?; // E9 00 00 00 00 (5 字节)
  masm.nop()?; // 90
  masm.bind(&mut far_label)?;
  // 偏移 = 1
  assert_eq!(
    &masm.buffer_slice()[far_start..],
    &[0xe9, 0x01, 0x00, 0x00, 0x00, 0x90]
  );

  // 3. 链式多个前向跳转链接到同一标号
  let mut multi_label = Label::new();
  let multi_start = masm.pc_offset();
  masm.jmp(&mut multi_label)?; // jmp 1
  masm.nop()?;
  masm.jmp(&mut multi_label)?; // jmp 2
  masm.nop()?;
  masm.bind(&mut multi_label)?;
  let end_pos = masm.pc_offset() as i32;

  // jmp 1 位移: end_pos - (multi_start + 5)
  let jmp1_disp = i32::from_le_bytes(
    masm.buffer_slice()[multi_start + 1..multi_start + 5]
      .try_into()
      .unwrap_or_default(),
  );
  assert_eq!(jmp1_disp, end_pos - (multi_start as i32 + 5));

  // jmp 2 位移: end_pos - (multi_start + 5 + 1 + 5)
  let jmp2_start = multi_start + 5 + 1;
  let jmp2_disp = i32::from_le_bytes(
    masm.buffer_slice()[jmp2_start + 1..jmp2_start + 5]
      .try_into()
      .unwrap_or_default(),
  );
  assert_eq!(jmp2_disp, end_pos - (jmp2_start as i32 + 5));

  // 4. 后向跳转 (Backward jump)
  let mut back_label = Label::new();
  let back_start = masm.pc_offset();
  masm.bind(&mut back_label)?; // bound at back_start
  masm.nop()?; // 90 at back_start
  masm.jmp_label(&mut back_label, Distance::Near)?; // EB FD at back_start+1..back_start+3
  // 目标 back_start - (back_start + 3) = -3 = 0xFD
  assert_eq!(
    &masm.buffer_slice()[back_start..back_start + 3],
    &[0x90, 0xeb, 0xfd]
  );

  OK
}

#[test]
fn test_conditional_jumps_and_predicates() -> Void {
  info!("测试条件跳转 j(Condition) 指令与条件码翻转");

  let mut masm = Assembler::new(AssemblerOptions::default());

  // 条件取反测试
  assert_eq!(Condition::Equal.negate(), Condition::NotEqual);
  assert_eq!(Condition::Below.negate(), Condition::AboveEqual);
  assert_eq!(Condition::Less.negate(), Condition::GreaterEqual);

  // 条件近跳转
  let mut l1 = Label::new();
  masm.j(Condition::Equal, &mut l1, Distance::Near)?; // 74 00
  masm.nop()?;
  masm.bind(&mut l1)?;
  assert_eq!(masm.buffer_slice(), &[0x74, 0x01, 0x90]);

  // 条件远跳转
  let mut l2 = Label::new();
  let mark = masm.pc_offset();
  masm.j(Condition::NotEqual, &mut l2, Distance::Far)?; // 0f 85 00 00 00 00
  masm.nop()?;
  masm.bind(&mut l2)?;
  assert_eq!(
    &masm.buffer_slice()[mark..],
    &[0x0f, 0x85, 0x01, 0x00, 0x00, 0x00, 0x90]
  );

  OK
}

#[test]
fn test_root_register_bias_and_r13_boundary() -> Void {
  info!("测试 K_ROOT_REGISTER_BIAS 槽位紧凑编码与 R13/R12 深层寻址边界");

  // 1. R13 零位移强制 mod=01, disp8=0 边界 (防止与 RIP 相对寻址混淆, 且 REX.B=1)
  let op_r13_zero = Operand::base(R13);
  assert_eq!(op_r13_zero.rex, 1); // REX.B = 1 (r13 code=13, high_bit=1)
  assert_eq!(op_r13_zero.buf[0], 0x45); // mod=01, rm=5 (r13 low_bits=5)
  assert_eq!(op_r13_zero.buf[1], 0x00); // disp8 = 0
  assert_eq!(op_r13_zero.len, 2);

  // 2. K_ROOT_REGISTER_BIAS 紧凑负偏移寻址边界
  // IsolateData 槽 0 (offset 0): 实际寻址偏移为 0 - 128 = -128, 紧凑 disp8
  let op_slot0 = Operand::base_disp(R13, 0 - K_ROOT_REGISTER_BIAS);
  assert_eq!(op_slot0.rex, 1);
  assert_eq!(op_slot0.buf[0], 0x45); // mod=01
  assert_eq!(op_slot0.buf[1], (-128i8) as u8); // 0x80
  assert_eq!(op_slot0.len, 2);

  // IsolateData 槽 31 (offset 31 * 8 = 248): 实际寻址偏移为 248 - 128 = 120, 紧凑 disp8
  let op_slot31 = Operand::base_disp(R13, 31 * 8 - K_ROOT_REGISTER_BIAS);
  assert_eq!(op_slot31.rex, 1);
  assert_eq!(op_slot31.buf[0], 0x45); // mod=01
  assert_eq!(op_slot31.buf[1], 120);
  assert_eq!(op_slot31.len, 2);

  // 若不带偏置访问 256 字节偏移 (超出 int8 范围 [-128, 127]): 必须升级为 32 位位移 (len=5)
  let op_slot32_unbiased = Operand::base_disp(R13, 256);
  assert_eq!(op_slot32_unbiased.buf[0], 0x85); // mod=10, rm=5
  assert_eq!(&op_slot32_unbiased.buf[1..5], &256i32.to_le_bytes());
  assert_eq!(op_slot32_unbiased.len, 5);

  // 3. R12(base) + R9(index) 组合 SIB 与 REX 联合置位
  let op_r12_r9 = Operand::base_index_scale_disp(R12, R9, ScaleFactor::Times8, 8);
  // REX.X (r9 high_bit=1) << 1 | REX.B (r12 high_bit=1) = (1 << 1) | 1 = 3
  assert_eq!(op_r12_r9.rex, 3);
  assert_eq!(op_r12_r9.buf[0], 0x44); // mod=01, rm=4 (SIB)
  // SIB: scale=times_8(3 << 6), index=r9 low_bits(1 << 3), base=r12 low_bits(4)
  assert_eq!(op_r12_r9.buf[1], (3 << 6) | (1 << 3) | 4);
  assert_eq!(op_r12_r9.buf[2], 8);
  assert_eq!(op_r12_r9.len, 3);

  OK
}
