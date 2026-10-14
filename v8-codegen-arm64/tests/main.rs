use std::{f64::consts::PI, ptr::null_mut};

use aok::{OK, Void};
use log::info;
use v8_codegen_arm64::{
  Assembler, B0, CPURegList, Condition, D0, D8, D15, D31, Error, Extend, H0, LR, MemOperand,
  Operand, Q0, Q31, Register, S0, S31, SP, Shift, VRegister, W0, W1, W2, W31, WSP, WZR, X0, X1, X2,
  X3, X4, X5, X30, X31, XZR,
};
use v8_codegen_core::{AssemblerOptions, Label};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_registers_and_conditions() -> Void {
  info!("Testing register properties and conditions");

  assert_eq!(X0.code(), 0);
  assert!(X0.is_64_bits());
  assert_eq!(X0.size_in_bytes(), 8);

  assert_eq!(X30.code(), 30);
  assert_eq!(LR.code(), 30);

  assert_eq!(X31.code(), 31);
  assert!(XZR.is_zero());
  assert!(!XZR.is_sp());

  assert!(SP.is_sp());
  assert!(!SP.is_zero());
  assert!(SP.is_64_bits());

  assert!(WSP.is_sp());
  assert!(WSP.is_32_bits());
  assert_eq!(W0.code(), 0);
  assert!(W0.is_32_bits());
  assert_eq!(W31.code(), 31);
  assert!(WZR.is_zero());

  assert_eq!(D0.code(), 0);
  assert!(D0.is_d());
  assert_eq!(D31.code(), 31);

  assert_eq!(S0.code(), 0);
  assert!(S0.is_s());
  assert_eq!(S31.code(), 31);

  assert_eq!(Q0.code(), 0);
  assert!(Q0.is_q());
  assert_eq!(Q31.code(), 31);

  // Condition negation tests
  assert_eq!(Condition::Eq.negate(), Condition::Ne);
  assert_eq!(Condition::Ne.negate(), Condition::Eq);
  assert_eq!(Condition::Cs.negate(), Condition::Cc);
  assert_eq!(Condition::Mi.negate(), Condition::Pl);
  assert_eq!(Condition::Vs.negate(), Condition::Vc);
  assert_eq!(Condition::Hi.negate(), Condition::Ls);
  assert_eq!(Condition::Ge.negate(), Condition::Lt);
  assert_eq!(Condition::Gt.negate(), Condition::Le);
  assert_eq!(Condition::Al.negate(), Condition::Nv);

  OK
}

#[test]
fn test_cpureglist() -> Void {
  info!("Testing CPURegList operations");

  let mut list = CPURegList::from_slice(&[X0, X1, X2]);
  assert_eq!(list.count(), 3);
  assert!(list.has(X0));
  assert!(list.has(X1));
  assert!(list.has(X2));
  assert!(!list.has(X3));

  list.clear(X1);
  assert_eq!(list.count(), 2);
  assert!(!list.has(X1));

  list.set(X5);
  assert_eq!(list.count(), 3);
  assert!(list.has(X5));

  let lowest = list.pop_lowest_index();
  assert_eq!(lowest.map(|r| r.code()), Some(0));

  let highest = list.pop_highest_index();
  assert_eq!(highest.map(|r| r.code()), Some(5));

  let remaining = list.pop_lowest_index();
  assert_eq!(remaining.map(|r| r.code()), Some(2));
  assert!(list.is_empty());

  let callee_saved = CPURegList::get_callee_saved();
  assert_eq!(callee_saved.count(), 10);
  assert!(callee_saved.has(v8_codegen_arm64::X19));
  assert!(callee_saved.has(v8_codegen_arm64::X28));
  assert!(!callee_saved.has(v8_codegen_arm64::X29));

  let caller_saved = CPURegList::get_caller_saved();
  // 原版 GetCallerSaved 仅 x0-x17 (assembler-arm64.cc:251), lr 不在列表
  assert_eq!(caller_saved.count(), 18);
  assert!(caller_saved.has(X0));
  assert!(caller_saved.has(v8_codegen_arm64::X17));
  assert!(!caller_saved.has(LR));

  let callee_saved_d = CPURegList::get_callee_saved_d();
  assert_eq!(callee_saved_d.count(), 8);
  assert!(callee_saved_d.has_v(D8));
  assert!(callee_saved_d.has_v(D15));
  assert!(!callee_saved_d.has_v(D0));

  let callee_saved_v = CPURegList::get_callee_saved_v();
  assert_eq!(callee_saved_v.count(), 0);
  assert!(callee_saved_v.is_empty());
  assert_eq!(callee_saved_v.size_in_bits(), 128);

  let caller_saved_v = CPURegList::get_caller_saved_v();
  assert_eq!(caller_saved_v.count(), 32);
  assert_eq!(caller_saved_v.size_in_bits(), 128);

  let arg_regs = CPURegList::get_argument_registers();
  assert_eq!(arg_regs.count(), 8);
  assert!(arg_regs.has(X0));
  assert!(arg_regs.has(v8_codegen_arm64::X7));
  assert!(!arg_regs.has(v8_codegen_arm64::X8));

  let arg_regs_d = CPURegList::get_argument_registers_d();
  assert_eq!(arg_regs_d.count(), 8);
  assert!(arg_regs_d.has_v(D0));
  assert!(arg_regs_d.has_v(v8_codegen_arm64::D7));
  assert!(!arg_regs_d.has_v(D8));

  // includes_alias_of
  assert!(arg_regs.includes_alias_of(X0));
  assert!(arg_regs.includes_alias_of(v8_codegen_arm64::W0)); // w0 别名 x0 共享 code
  assert!(!arg_regs.includes_alias_of(v8_codegen_arm64::X8));
  assert!(!arg_regs.includes_alias_of(D0)); // 跨类型不包含

  // align() 栈对齐: 确保寄存器数量为偶数
  let mut align_list = CPURegList::from_slice(&[X0, X1, X2]); // 3 个寄存器 (奇数，不含 padreg)
  assert_eq!(align_list.count(), 3);
  align_list.align();
  assert_eq!(align_list.count(), 4); // 补充 padreg (xzr) 达到偶数
  assert!(align_list.has(v8_codegen_arm64::XZR));
  align_list.align();
  assert_eq!(align_list.count(), 4); // 已是偶数，幂等保持 4

  let mut odd_with_pad = CPURegList::from_slice(&[X0, X1, v8_codegen_arm64::XZR]); // 3 个寄存器 (奇数，含 padreg)
  assert_eq!(odd_with_pad.count(), 3);
  odd_with_pad.align();
  assert_eq!(odd_with_pad.count(), 2); // 移除 padreg 达到偶数
  assert!(!odd_with_pad.has(v8_codegen_arm64::XZR));

  let mut vlist = CPURegList::from_vslice(&[D0, D8]);
  assert_eq!(vlist.count(), 2);
  assert!(vlist.has_v(D0));
  assert!(vlist.has_v(D8));
  let lowest_v = vlist.pop_lowest_cpu_reg();
  assert_eq!(lowest_v.map(|r| r.code()), Some(0));

  OK
}

#[test]
fn test_standard_instructions_byte_exact() -> Void {
  info!("Testing standard instruction binary encodings");

  let mut assm = Assembler::new(AssemblerOptions::default());

  // mov x0, x1 -> E0 03 01 AA
  assm.mov(X0, X1)?;
  assert_eq!(assm.buffer_slice(), &[0xE0, 0x03, 0x01, 0xAA]);

  // ret -> C0 03 5F D6
  let offset = assm.pc_offset();
  assm.ret(None)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0xC0, 0x03, 0x5F, 0xD6]);

  // nop -> 1F 20 03 D5
  let offset = assm.pc_offset();
  assm.nop()?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x1F, 0x20, 0x03, 0xD5]);

  // br x0 -> 00 00 1F D6
  let offset = assm.pc_offset();
  assm.br(X0)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x00, 0x00, 0x1F, 0xD6]);

  // blr x1 -> 20 00 3F D6
  let offset = assm.pc_offset();
  assm.blr(X1)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x00, 0x3F, 0xD6]);

  // add x0, x1, x2 -> 20 00 02 8B
  let offset = assm.pc_offset();
  assm.add(X0, X1, X2)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x00, 0x02, 0x8B]);

  // sub x3, x4, x5 -> 83 00 05 CB
  let offset = assm.pc_offset();
  assm.sub(X3, X4, X5)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x83, 0x00, 0x05, 0xCB]);

  // r7 D1 回归: extended 形式纳入 set_flags——subs x1, x2, x3 UXTX #2
  // = SUBS_EXT 0x6B200000 | SF | Rd=1 | Rn=2<<5 | Rm=3<<16 | Extend=3<<13 | imm=2<<10
  // = EB 23 68 41 (DataProcExtendedRegister, assembler-arm64.cc:4361-4368)
  let offset = assm.pc_offset();
  assm.subs(X1, X2, Operand::extended(X3, Extend::UXTX, 2))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x41, 0x68, 0x23, 0xEB]);

  // r7 D2 回归: SP 参与 add/sub 时 shifted→extended 转换 (assembler-arm64.cc:4056-4064;
  // ToExtendedRegister LSL#0 → UXTX, assembler-arm64-inl.h:337-341)
  // add sp, sp, x0 == add xsp, xsp, x0 UXTX #0
  // = 0x8B200000 | RdSP=31 | RnSP=31<<5 | UXTX 3<<13 = 8B 20 63 FF (LE: FF 63 20 8B)
  let offset = assm.pc_offset();
  assm.add(SP, SP, X0)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0xFF, 0x63, 0x20, 0x8B]);

  // wsp 64-bit 尺寸不匹配须拒 (rm 宽检查先于转换); UXTW 支: add wsp, wsp, w0
  let offset = assm.pc_offset();
  assm.add(WSP, WSP, W0)?;
  // = 0x0B200000 | 31 | 31<<5 | UXTW 2<<13 = 0x0B2043FF → LE FF 43 20 0B
  assert_eq!(&assm.buffer_slice()[offset..], &[0xFF, 0x43, 0x20, 0x0B]);

  // add x0, x1, #16 -> 20 40 00 91
  let offset = assm.pc_offset();
  assm.add(X0, X1, 16)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x40, 0x00, 0x91]);

  // and x0, x1, x2 -> 20 00 02 8A
  let offset = assm.pc_offset();
  assm.and_(X0, X1, X2)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x00, 0x02, 0x8A]);

  // orr x0, x1, x2 -> 20 00 02 AA
  let offset = assm.pc_offset();
  assm.orr(X0, X1, X2)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x00, 0x02, 0xAA]);

  // eor x0, x1, x2 -> 20 00 02 CA
  let offset = assm.pc_offset();
  assm.eor(X0, X1, X2)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x00, 0x02, 0xCA]);

  // cmp x0, x1 (subs xzr, x0, x1) -> 1F 00 01 EB
  let offset = assm.pc_offset();
  assm.cmp(X0, X1)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x1F, 0x00, 0x01, 0xEB]);

  // ldr x0, [x1, #16] -> 20 08 40 F9
  let offset = assm.pc_offset();
  assm.ldr(X0, MemOperand::offset(X1, 16))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x08, 0x40, 0xF9]);

  // str x0, [x1, #16] -> 20 08 00 F9
  let offset = assm.pc_offset();
  assm.str(X0, MemOperand::offset(X1, 16))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x08, 0x00, 0xF9]);

  OK
}

#[test]
fn test_label_backward_jump() -> Void {
  info!("Testing backward label jump (B, B.cond, BL)");

  let mut assm = Assembler::new(AssemblerOptions::default());
  let mut target = Label::new();

  // 绑定在 offset 0
  assm.bind(&mut target)?;
  // 插入 2 个 nop (8 字节)
  assm.nop()?; // offset 0
  assm.nop()?; // offset 4

  // 在 offset 8 执行无条件跳转 B 至 target (offset 0)
  // disp_bytes = 0 - 8 = -8 字节 -> disp_words = -2
  // B | (-2 & 0x03FFFFFF) = 0x14000000 | 0x03FFFFFE = 0x17FFFFFE -> [0xFE, 0xFF, 0xFF, 0x17]
  let b_pos = assm.pc_offset();
  assm.b(&mut target)?;
  assert_eq!(
    &assm.buffer_slice()[b_pos..b_pos + 4],
    &[0xFE, 0xFF, 0xFF, 0x17]
  );

  // 在 offset 12 执行条件跳转 B.cond(ne) 至 target (offset 0)
  // disp_bytes = 0 - 12 = -12 字节 -> disp_words = -3
  // B_COND | ((-3 & 0x7FFFF) << 5) | ne(1) = 0x54FF_FFA1 -> [0xA1, 0xFF, 0xFF, 0x54]
  let b_cond_pos = assm.pc_offset();
  assm.b_cond(Condition::Ne, &mut target)?;
  assert_eq!(
    &assm.buffer_slice()[b_cond_pos..b_cond_pos + 4],
    &[0xA1, 0xFF, 0xFF, 0x54]
  );

  // 在 offset 16 执行带链接跳转 BL 至 target (offset 0)
  // disp_bytes = 0 - 16 = -16 字节 -> disp_words = -4
  // BL | (-4 & 0x03FFFFFF) = 0x94000000 | 0x03FFFFFC = 0x97FFFFFC -> [0xFC, 0xFF, 0xFF, 0x97]
  let bl_pos = assm.pc_offset();
  assm.bl(&mut target)?;
  assert_eq!(
    &assm.buffer_slice()[bl_pos..bl_pos + 4],
    &[0xFC, 0xFF, 0xFF, 0x97]
  );

  OK
}

#[test]
fn test_label_forward_jump_and_backpatch() -> Void {
  info!("Testing forward label jump and backpatching (B, BL, B.cond chain)");

  let mut assm = Assembler::new(AssemblerOptions::default());
  let mut target = Label::new();

  // offset 0: 向前跳转 B 至尚未绑定的 target
  let branch1_pos = assm.pc_offset();
  assm.b(&mut target)?;

  // offset 4: 向前跳转 BL 至尚未绑定的 target
  let branch2_pos = assm.pc_offset();
  assm.bl(&mut target)?;

  // offset 8: 向前条件跳转 B.cond(eq) 至尚未绑定的 target
  let branch3_pos = assm.pc_offset();
  assm.b_cond(Condition::Eq, &mut target)?;

  // 插入 2 个 nop
  assm.nop()?; // offset 12
  assm.nop()?; // offset 16

  // 绑定 target 于 offset 20
  assm.bind(&mut target)?;

  // branch1 在 offset 0，跳转至 offset 20 -> disp_bytes = +20 -> disp_words = +5
  // B | 5 = 0x14000005 -> [0x05, 0x00, 0x00, 0x14]
  assert_eq!(
    &assm.buffer_slice()[branch1_pos..branch1_pos + 4],
    &[0x05, 0x00, 0x00, 0x14]
  );

  // branch2 在 offset 4，跳转至 offset 20 -> disp_bytes = +16 -> disp_words = +4
  // BL | 4 = 0x94000004 -> [0x04, 0x00, 0x00, 0x94]
  assert_eq!(
    &assm.buffer_slice()[branch2_pos..branch2_pos + 4],
    &[0x04, 0x00, 0x00, 0x94]
  );

  // branch3 在 offset 8，跳转至 offset 20 -> disp_bytes = +12 -> disp_words = +3
  // B_COND | (3 << 5) | eq(0) = 0x54000000 | 0x60 = 0x54000060 -> [0x60, 0x00, 0x00, 0x54]
  assert_eq!(
    &assm.buffer_slice()[branch3_pos..branch3_pos + 4],
    &[0x60, 0x00, 0x00, 0x54]
  );

  OK
}

#[test]
fn test_sp_moves_and_extended_ops() -> Void {
  info!("Testing SP moves and extended/shifted arithmetic");

  let mut assm = Assembler::new(AssemblerOptions::default());

  // mov sp, x0 -> add sp, x0, #0 -> 1F 00 00 91
  let offset = assm.pc_offset();
  assm.mov(SP, X0)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x1F, 0x00, 0x00, 0x91]);

  // mov x0, sp -> add x0, sp, #0 -> E0 03 00 91
  let offset = assm.pc_offset();
  assm.mov(X0, SP)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0xE0, 0x03, 0x00, 0x91]);

  // mov wsp, w0 -> add wsp, w0, #0 -> 1F 00 00 11
  let offset = assm.pc_offset();
  assm.mov(WSP, W0)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x1F, 0x00, 0x00, 0x11]);

  // mov w0, wsp -> add w0, wsp, #0 -> E0 03 00 11
  let offset = assm.pc_offset();
  assm.mov(W0, WSP)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0xE0, 0x03, 0x00, 0x11]);

  // add x0, x1, 4096 (12-bit shift) -> 20 04 40 91
  let offset = assm.pc_offset();
  assm.add(X0, X1, 4096)?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x04, 0x40, 0x91]);

  // ldr x0, [x1, #-8] -> unscaled LDUR_x -> 20 80 5F F8
  // LDUR_X = 0xF8400000 | Rt(0) | Rn(1: 0x20) | ImmLS(-8 & 0x1FF = 0x1F8: 0x1F8000)
  // 0xF8400000 | 0x20 | 0x1F8000 = 0xF85F8020 -> [0x20, 0x80, 0x5F, 0xF8]
  let offset = assm.pc_offset();
  assm.ldr(X0, MemOperand::offset(X1, -8))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x80, 0x5F, 0xF8]);

  // str x0, [x1, #-8] -> unscaled STUR_x -> 20 80 1F F8
  let offset = assm.pc_offset();
  assm.str(X0, MemOperand::offset(X1, -8))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x80, 0x1F, 0xF8]);

  // ldr_v d0, [x1, #16] -> LDR_D_UNSIGNED -> 20 08 40 FD
  let offset = assm.pc_offset();
  assm.ldr_v(D0, MemOperand::offset(X1, 16))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x08, 0x40, 0xFD]);

  // str_v s0, [x1, #16] -> STR_S_UNSIGNED (scale 4, imm = 4) -> 20 10 00 BD
  let offset = assm.pc_offset();
  assm.str_v(S0, MemOperand::offset(X1, 16))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x10, 0x00, 0xBD]);

  // ldr_v b0, [x1, #16] -> LDR_B_UNSIGNED (scale 1, imm = 16) -> 20 40 40 3D
  let offset = assm.pc_offset();
  assm.ldr_v(B0, MemOperand::offset(X1, 16))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x40, 0x40, 0x3D]);

  // str_v b0, [x1, #16] -> STR_B_UNSIGNED (scale 1, imm = 16) -> 20 40 00 3D
  let offset = assm.pc_offset();
  assm.str_v(B0, MemOperand::offset(X1, 16))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x40, 0x00, 0x3D]);

  // ldr_v h0, [x1, #16] -> LDR_H_UNSIGNED (scale 2, imm = 8) -> 20 20 40 7D
  let offset = assm.pc_offset();
  assm.ldr_v(H0, MemOperand::offset(X1, 16))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x20, 0x40, 0x7D]);

  // str_v h0, [x1, #16] -> STR_H_UNSIGNED (scale 2, imm = 8) -> 20 20 00 7D
  let offset = assm.pc_offset();
  assm.str_v(H0, MemOperand::offset(X1, 16))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x20, 0x00, 0x7D]);

  // ldr_v q0, [x1, #16] -> LDR_Q_UNSIGNED (scale 16, imm = 1) -> 20 04 C0 3D
  let offset = assm.pc_offset();
  assm.ldr_v(Q0, MemOperand::offset(X1, 16))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x04, 0xC0, 0x3D]);

  // str_v q0, [x1, #16] -> STR_Q_UNSIGNED (scale 16, imm = 1) -> 20 04 80 3D
  let offset = assm.pc_offset();
  assm.str_v(Q0, MemOperand::offset(X1, 16))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x04, 0x80, 0x3D]);

  // ldr_v d0, [x1, #-8] -> unscaled LDUR_D -> 20 80 5F FC
  let offset = assm.pc_offset();
  assm.ldr_v(D0, MemOperand::offset(X1, -8))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x80, 0x5F, 0xFC]);

  // str_v d0, [x1, #-8] -> unscaled STUR_D -> 20 80 1F FC
  let offset = assm.pc_offset();
  assm.str_v(D0, MemOperand::offset(X1, -8))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x80, 0x1F, 0xFC]);

  // ldr_v d0, [x1, x2, lsl #3] -> register-offset LDR_D_REG -> 20 78 62 FC
  let offset = assm.pc_offset();
  assm.ldr_v(D0, MemOperand::reg_offset(X1, X2, Shift::LSL, 3))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x78, 0x62, 0xFC]);

  // str_v d0, [x1, x2, lsl #3] -> register-offset STR_D_REG -> 20 78 22 FC
  let offset = assm.pc_offset();
  assm.str_v(D0, MemOperand::reg_offset(X1, X2, Shift::LSL, 3))?;
  assert_eq!(&assm.buffer_slice()[offset..], &[0x20, 0x78, 0x22, 0xFC]);

  OK
}

#[test]
fn test_branch_link_and_errors() -> Void {
  info!("Testing BL branch-with-link and label error conditions");

  let mut assm = Assembler::new(AssemblerOptions::default());
  let mut target = Label::new();

  // bl to unborn target
  let bl_pos = assm.pc_offset();
  assm.bl(&mut target)?;
  assm.nop()?;
  assm.bind(&mut target)?;

  // target is at offset 8, bl was at 0 -> disp_words = 2 -> BL | 2 = 0x94000002 -> [0x02, 0x00, 0x00, 0x94]
  assert_eq!(
    &assm.buffer_slice()[bl_pos..bl_pos + 4],
    &[0x02, 0x00, 0x00, 0x94]
  );

  // Re-binding an already bound label must fail with LabelAlreadyBound
  let res = assm.bind(&mut target);
  assert!(matches!(res, Err(Error::LabelAlreadyBound(8))));

  // Offset out of range tests (including extreme i32 values to test overflow safety)
  assert!(matches!(
    assm.b_offset(40_000_000),
    Err(Error::BranchOffsetOutOfRange(_))
  ));
  assert!(matches!(
    assm.b_offset(-40_000_000),
    Err(Error::BranchOffsetOutOfRange(_))
  ));
  assert!(matches!(
    assm.b_offset(i32::MAX),
    Err(Error::BranchOffsetOutOfRange(_))
  ));
  assert!(matches!(
    assm.b_offset(i32::MIN),
    Err(Error::BranchOffsetOutOfRange(_))
  ));
  assert!(matches!(
    assm.bl_offset(40_000_000),
    Err(Error::BranchOffsetOutOfRange(_))
  ));
  assert!(matches!(
    assm.bl_offset(i32::MAX),
    Err(Error::BranchOffsetOutOfRange(_))
  ));
  assert!(matches!(
    assm.bl_offset(i32::MIN),
    Err(Error::BranchOffsetOutOfRange(_))
  ));
  assert!(matches!(
    assm.b_cond_offset(Condition::Eq, 300_000),
    Err(Error::BranchOffsetOutOfRange(_))
  ));
  assert!(matches!(
    assm.b_cond_offset(Condition::Eq, -300_000),
    Err(Error::BranchOffsetOutOfRange(_))
  ));
  assert!(matches!(
    assm.b_cond_offset(Condition::Eq, i32::MAX),
    Err(Error::BranchOffsetOutOfRange(_))
  ));
  assert!(matches!(
    assm.b_cond_offset(Condition::Eq, i32::MIN),
    Err(Error::BranchOffsetOutOfRange(_))
  ));

  // Immediate out of range error checks
  assert!(matches!(
    assm.add(X0, X1, Operand::shifted(X2, Shift::LSL, 64)),
    Err(Error::ImmediateOutOfRange(64))
  ));
  assert!(matches!(
    assm.add(W0, W1, Operand::shifted(W2, Shift::LSL, 32)),
    Err(Error::ImmediateOutOfRange(32))
  ));
  assert!(matches!(
    assm.add(X0, X1, Operand::extended(X2, Extend::UXTX, 5)),
    Err(Error::ImmediateOutOfRange(5))
  ));
  // r7 D3 回归: shifted 支拒 ROR (cc:4052 DCHECK_NE(operand.shift(), ROR))
  assert!(matches!(
    assm.add(X0, X1, Operand::shifted(X2, Shift::ROR, 3)),
    Err(Error::InvalidRegister(_))
  ));
  // r7 D2 回归衔接: SP 转换仅容 LSL #0-4 (inl.h:339)——LSR 与 LSL#8 均拒
  assert!(matches!(
    assm.add(SP, X1, Operand::shifted(X2, Shift::LSR, 1)),
    Err(Error::InvalidRegister(_))
  ));
  assert!(matches!(
    assm.add(SP, X1, Operand::shifted(X2, Shift::LSL, 8)),
    Err(Error::InvalidRegister(_))
  ));
  // Rd() 系 DCHECK 承接 (assembler-arm64.h:2818-2821): set_flags + SP rd 拒
  assert!(matches!(
    assm.subs(SP, X1, X2),
    Err(Error::InvalidRegister(_))
  ));
  assert!(matches!(
    assm.add(X0, X1, Operand::imm(5000)),
    Err(Error::ImmediateOutOfRange(5000))
  ));
  assert!(matches!(
    assm.ldr(X0, MemOperand::offset(X1, 100_000)),
    Err(Error::ImmediateOutOfRange(100_000))
  ));
  assert!(matches!(
    assm.ldr_v(D0, MemOperand::offset(X1, 100_000)),
    Err(Error::ImmediateOutOfRange(100_000))
  ));

  // Register mismatch error checks
  assert!(matches!(assm.mov(X0, W0), Err(Error::InvalidRegister(_))));
  assert!(matches!(
    assm.add(X0, W0, X1),
    Err(Error::InvalidRegister(_))
  ));
  assert!(matches!(
    assm.sub(X0, X1, W0),
    Err(Error::InvalidRegister(_))
  ));
  assert!(matches!(
    assm.and_(X0, W0, X1),
    Err(Error::InvalidRegister(_))
  ));
  assert!(matches!(assm.br(W0), Err(Error::InvalidRegister(_))));
  assert!(matches!(assm.blr(W0), Err(Error::InvalidRegister(_))));
  assert!(matches!(assm.ret(Some(W0)), Err(Error::InvalidRegister(_))));
  assert!(matches!(
    assm.ldr(X0, MemOperand::offset(W0, 0)),
    Err(Error::InvalidRegister(_))
  ));
  assert!(matches!(
    assm.ldr_v(D0, MemOperand::offset(W0, 0)),
    Err(Error::InvalidRegister(_))
  ));

  OK
}

// ── wave1 数据面金标准: 寄存器名表 ↔ 编号互逆 (DEFINE_REGISTER_NAMES / GetSpecialRegisterName) ──
#[test]
fn test_wave1_register_name_roundtrip() -> Void {
  info!("Testing wave1 register name table roundtrip");
  use v8_codegen_arm64 as arm;

  // 全表金标准: GENERAL_REGISTERS 串化 "x0".."x31" (x31 即零寄存器, 旧上游名 zeroreg)
  let golden: [&str; 32] = [
    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14",
    "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27",
    "x28", "x29", "x30", "x31",
  ];
  for (idx, (&actual, &want)) in arm::GENERAL_REG_NAMES.iter().zip(golden.iter()).enumerate() {
    assert_eq!(actual, want, "名表 index {idx}");
  }

  // 编号→名称→编号 互逆性
  for (i, name) in arm::GENERAL_REG_NAMES.iter().enumerate() {
    let reg = arm::Register::x(i as i8);
    assert_eq!(arm::register_name(reg), *name);
    assert_eq!(arm::reg_code_of(name), Some(i as i8));
    assert_eq!(arm::register_name(arm::Register::from_code(i as i8)), *name);
  }

  // w 与 x 共享 code, register_name 统一返回 X 名
  assert_eq!(arm::register_name(arm::W5), "x5");

  // 边界守护: no_reg(-1) 与 sp 内部编码(63) 越界返回 "invalid" (原版为 UB)
  assert_eq!(arm::register_name(arm::NO_REG), "invalid");
  assert_eq!(arm::register_name(arm::SP), "invalid");
  assert_eq!(arm::reg_code_of("sp"), None);
  assert_eq!(arm::reg_code_of("ip0"), None);

  // GetSpecialRegisterName (register-arm64.h:279) 逐值 1:1
  assert_eq!(arm::get_special_register_name(63), "sp");
  assert_eq!(arm::get_special_register_name(31), "UNKNOWN");
  assert_eq!(arm::get_special_register_name(0), "UNKNOWN");
  assert_eq!(arm::get_special_register_name(-1), "UNKNOWN");
  OK
}

// ── wave1 数据面金标准: ALIAS_REGISTER 别名链 + 调用约定 23 条编号 ──
#[test]
fn test_wave1_aliases_and_calling_convention() -> Void {
  info!("Testing wave1 alias chain and calling convention");
  use v8_codegen_arm64 as arm;

  // 别名链 golden 编号 (register-arm64.h:563-598)
  assert_eq!(arm::K_STACK_POINTER_REGISTER.code(), 63);
  assert!(arm::K_STACK_POINTER_REGISTER.is_sp());
  assert_eq!(arm::IP0.code(), 16);
  assert_eq!(arm::IP1.code(), 17);
  assert_eq!(arm::WIP0.code(), 16);
  assert_eq!(arm::WIP1.code(), 17);
  assert!(!arm::WIP0.is_64_bits());
  assert_eq!(arm::K_ROOT_REGISTER, arm::RR);
  assert_eq!(arm::RR, arm::K_ROOT_REGISTER);
  assert_eq!(arm::K_ROOT_REGISTER.code(), 26);
  assert_eq!(
    arm::K_PTR_COMPR_CAGE_BASE_REGISTER_WITH_COMPRESSION,
    arm::X28
  );
  assert_eq!(
    arm::K_PTR_COMPR_CAGE_BASE_REGISTER_WITHOUT_COMPRESSION.code(),
    -1
  );
  assert_eq!(arm::CP.code(), 27);
  assert_eq!(arm::FP.code(), 29);
  assert_eq!(arm::LR.code(), 30);
  assert!(arm::XZR.is_zero());
  assert_eq!(arm::PADREG, arm::XZR);
  assert_eq!(arm::PADREG, arm::PADREG);
  assert_eq!(arm::V8_, arm::V8);
  assert_eq!(arm::FP_ZERO, arm::D15);
  assert_eq!(arm::FP_FIXED1, arm::D27);
  assert_eq!(arm::FP_FIXED2, arm::D28);
  assert_eq!(arm::FP_SCRATCH, arm::D30);
  assert_eq!(arm::FP_SCRATCH1, arm::D30);
  assert_eq!(arm::FP_SCRATCH2, arm::D31);
  assert_eq!(arm::FP_ZERO, arm::D15);

  // C 调用约定参数寄存器
  assert_eq!(arm::K_C_ARG_REGS.len(), 8);
  for (i, reg) in arm::K_C_ARG_REGS.iter().enumerate() {
    assert_eq!(reg.code(), i as i8);
  }
  assert_eq!(arm::K_REGISTER_PASSED_ARGUMENTS, 8);
  assert_eq!(arm::K_FP_REGISTER_PASSED_ARGUMENTS, 8);

  // 调用约定 23 条逐编号 (register-arm64.h:663-688)
  assert_eq!(arm::K_RETURN_REGISTER0, arm::X0);
  assert_eq!(arm::K_RETURN_REGISTER1, arm::X1);
  assert_eq!(arm::K_RETURN_REGISTER2, arm::X2);
  assert_eq!(arm::K_JS_FUNCTION_REGISTER, arm::X1);
  assert_eq!(arm::K_CONTEXT_REGISTER, arm::CP);
  assert_eq!(arm::K_ALLOCATE_SIZE_REGISTER, arm::X1);
  assert_eq!(arm::K_INTERPRETER_ACCUMULATOR_REGISTER, arm::X0);
  assert_eq!(arm::K_INTERPRETER_BYTECODE_OFFSET_REGISTER.code(), 19);
  assert_eq!(arm::K_INTERPRETER_BYTECODE_ARRAY_REGISTER.code(), 20);
  assert_eq!(arm::K_INTERPRETER_DISPATCH_TABLE_REGISTER.code(), 21);
  assert_eq!(arm::K_JAVA_SCRIPT_CALL_ARG_COUNT_REGISTER, arm::X0);
  assert_eq!(arm::K_JAVA_SCRIPT_CALL_CODE_START_REGISTER, arm::X2);
  assert_eq!(
    arm::K_JAVA_SCRIPT_CALL_TARGET_REGISTER,
    arm::K_JS_FUNCTION_REGISTER
  );
  assert_eq!(arm::K_JAVA_SCRIPT_CALL_NEW_TARGET_REGISTER, arm::X3);
  assert_eq!(arm::K_JAVA_SCRIPT_CALL_EXTRA_ARG1_REGISTER, arm::X2);
  assert_eq!(arm::K_JAVA_SCRIPT_CALL_DISPATCH_HANDLE_REGISTER, arm::X4);
  assert_eq!(arm::K_RUNTIME_CALL_FUNCTION_REGISTER, arm::X1);
  assert_eq!(arm::K_RUNTIME_CALL_ARG_COUNT_REGISTER, arm::X0);
  assert_eq!(arm::K_RUNTIME_CALL_ARGV_REGISTER.code(), 11);
  assert_eq!(arm::K_WASM_IMPLICIT_ARG_REGISTER.code(), 7);
  assert_eq!(arm::K_WASM_COMPILE_LAZY_FUNC_INDEX_REGISTER.code(), 8);
  assert_eq!(arm::K_WASM_TRAP_HANDLER_FAULT_ADDRESS_REGISTER.code(), 16);
  assert_eq!(arm::K_SIMULATOR_HLT_ARGUMENT.code(), 16);
  assert_eq!(arm::K_FP_RETURN_REGISTER0, arm::D0);
  OK
}

// ── wave1 数据面金标准: constants-arm64.h 常量关系/字段全表/条件码/系统寄存器 ──
#[test]
fn test_wave1_constants_golden() -> Void {
  info!("Testing wave1 constants golden values");
  use v8_codegen_arm64::constants as c;

  // 指令尺寸与字面量池
  assert_eq!(c::K_INSTR_SIZE, 4);
  assert_eq!(1u8 << c::K_INSTR_SIZE_LOG2, c::K_INSTR_SIZE);
  assert_eq!(1u8 << c::K_LOAD_LITERAL_SCALE_LOG2, c::K_LOAD_LITERAL_SCALE);
  assert_eq!(c::K_LOAD_LITERAL_SCALE, c::K_INSTR_SIZE);
  assert_eq!(c::K_MAX_LOAD_LITERAL_RANGE, 1 << 20);
  assert_eq!(c::K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB, 128);
  assert_eq!(
    c::K_MAX_LOAD_LITERAL_RANGE,
    (1 << c::K_INSTR_SIZE_LOG2) * (1 << 18)
  );

  // 寄存器数量与尺寸关系
  assert_eq!(c::K_NUMBER_OF_REGISTERS, 32);
  assert_eq!(c::K_NUMBER_OF_V_REGISTERS, 32);
  assert_eq!(c::K_NUMBER_OF_Z_REGISTERS, 32);
  assert_eq!(c::K_NUMBER_OF_CALLEE_SAVED_REGISTERS, 10);
  assert_eq!(c::K_NUMBER_OF_CALLEE_SAVED_D_REGISTERS, 8);
  assert_eq!(c::K_NUMBER_OF_CALLEE_SAVED_V_REGISTERS, 0);
  assert_eq!(c::K_W_REG_SIZE * 8, c::K_W_REG_SIZE_IN_BITS);
  assert_eq!(c::K_X_REG_SIZE * 8, c::K_X_REG_SIZE_IN_BITS);
  assert_eq!(c::K_S_REG_SIZE * 8, c::K_S_REG_SIZE_IN_BITS);
  assert_eq!(c::K_D_REG_SIZE * 8, c::K_D_REG_SIZE_IN_BITS);
  assert_eq!(c::K_Q_REG_SIZE * 8, c::K_Q_REG_SIZE_IN_BITS);
  assert_eq!(1i32 << c::K_X_REG_SIZE_LOG2, c::K_X_REG_SIZE);
  assert_eq!(1i32 << c::K_Q_REG_SIZE_LOG2, c::K_Q_REG_SIZE);
  assert_eq!(c::K_B_REG_SIZE_IN_BITS, 8);
  assert_eq!(c::K_H_REG_SIZE_IN_BITS, 16);

  // 寄存器编号 golden
  assert_eq!(c::K_IP0_CODE, 16);
  assert_eq!(c::K_IP1_CODE, 17);
  assert_eq!(c::K_FRAME_POINTER_REG_CODE, 29);
  assert_eq!(c::K_LINK_REG_CODE, 30);
  assert_eq!(c::K_ZERO_REG_CODE, 31);
  assert_eq!(c::K_SP_REG_INTERNAL_CODE, 63);
  assert_eq!(c::K_REG_CODE_MASK, c::K_ZERO_REG_CODE as u32);
  assert_eq!(c::K_ROOT_REGISTER_BIAS, 256);

  // 符号位/NaN 位与掩码跨表达式关系
  assert_eq!(c::K_X_SIGN_MASK as u64, 1u64 << c::K_X_SIGN_BIT);
  assert_eq!(c::K_D_SIGN_MASK as u64, 1u64 << c::K_D_SIGN_BIT);
  assert_eq!(c::K_W_SIGN_MASK as u64, 1u64 << c::K_W_SIGN_BIT);
  assert_eq!(c::K_S_SIGN_MASK as u64, 1u64 << c::K_S_SIGN_BIT);
  assert_eq!(c::K_D_SIGN_BIT, c::K_X_SIGN_BIT);
  assert_eq!(c::K_W_SIGN_BIT, c::K_S_SIGN_BIT);
  assert_eq!(c::K_D_QUIET_NAN_MASK as u64, 1u64 << c::K_D_QUIET_NAN_BIT);
  assert_eq!(c::K_S_QUIET_NAN_MASK as u64, 1u64 << c::K_S_QUIET_NAN_BIT);
  assert_eq!(c::K_H_QUIET_NAN_MASK as u64, 1u64 << c::K_H_QUIET_NAN_BIT);
  assert_eq!(c::K_X_REG_MASK, c::K_X_MAX_U_INT as i64);
  assert_eq!(c::K_W_REG_MASK, c::K_W_MAX_U_INT as i64);
  assert_eq!(c::K_S_REG_MASK, c::K_W_REG_MASK);
  assert_eq!(c::K_D_REG_MASK, c::K_X_REG_MASK);
  assert_eq!(c::K_X_MAX_INT, i64::MAX);
  assert_eq!(c::K_X_MIN_INT, i64::MIN);
  assert_eq!(c::K_W_MAX_INT, i32::MAX);
  assert_eq!(c::K_W_MIN_INT, i32::MIN);
  assert_eq!(c::K_BYTE_MASK, 0xFF);
  assert_eq!(c::K_HALF_WORD_MASK, 0xFFFF);
  assert_eq!(c::K_WORD_MASK, 0xFFFF_FFFF);
  assert_eq!(c::K_ADDRESS_TAG_MASK, 0xFFu64 << c::K_ADDRESS_TAG_OFFSET);

  // 编码域字段全表恒等式 mask == ((1 << width) - 1) << offset (88 条)
  macro_rules! field {
    ($o:expr, $w:expr, $m:expr) => {{
      let (o, w) = ($o as u32, $w as u32);
      assert_eq!(
        $m,
        ((1u32 << w) - 1) << o,
        "字段恒等式漂移: {}",
        stringify!($m)
      );
    }};
  }
  field!(c::RD_OFFSET, c::RD_WIDTH, c::RD_MASK);
  field!(c::RN_OFFSET, c::RN_WIDTH, c::RN_MASK);
  field!(c::RM_OFFSET, c::RM_WIDTH, c::RM_MASK);
  field!(c::RA_OFFSET, c::RA_WIDTH, c::RA_MASK);
  field!(c::RT_OFFSET, c::RT_WIDTH, c::RT_MASK);
  field!(c::RT2_OFFSET, c::RT2_WIDTH, c::RT2_MASK);
  field!(c::RS_OFFSET, c::RS_WIDTH, c::RS_MASK);
  field!(
    c::PREFETCH_MODE_OFFSET,
    c::PREFETCH_MODE_WIDTH,
    c::PREFETCH_MODE_MASK
  );
  field!(
    c::SIXTY_FOUR_BITS_OFFSET,
    c::SIXTY_FOUR_BITS_WIDTH,
    c::SIXTY_FOUR_BITS_MASK
  );
  field!(
    c::FLAGS_UPDATE_OFFSET,
    c::FLAGS_UPDATE_WIDTH,
    c::FLAGS_UPDATE_MASK
  );
  field!(
    c::IMM_PC_REL_HI_OFFSET,
    c::IMM_PC_REL_HI_WIDTH,
    c::IMM_PC_REL_HI_MASK
  );
  field!(
    c::IMM_PC_REL_LO_OFFSET,
    c::IMM_PC_REL_LO_WIDTH,
    c::IMM_PC_REL_LO_MASK
  );
  field!(c::SHIFT_DP_OFFSET, c::SHIFT_DP_WIDTH, c::SHIFT_DP_MASK);
  field!(
    c::IMM_DP_SHIFT_OFFSET,
    c::IMM_DP_SHIFT_WIDTH,
    c::IMM_DP_SHIFT_MASK
  );
  field!(
    c::IMM_ADD_SUB_OFFSET,
    c::IMM_ADD_SUB_WIDTH,
    c::IMM_ADD_SUB_MASK
  );
  field!(
    c::SHIFT_ADD_SUB_OFFSET,
    c::SHIFT_ADD_SUB_WIDTH,
    c::SHIFT_ADD_SUB_MASK
  );
  field!(
    c::IMM_EXTEND_SHIFT_OFFSET,
    c::IMM_EXTEND_SHIFT_WIDTH,
    c::IMM_EXTEND_SHIFT_MASK
  );
  field!(
    c::EXTEND_MODE_OFFSET,
    c::EXTEND_MODE_WIDTH,
    c::EXTEND_MODE_MASK
  );
  field!(
    c::IMM_MOVE_WIDE_OFFSET,
    c::IMM_MOVE_WIDE_WIDTH,
    c::IMM_MOVE_WIDE_MASK
  );
  field!(
    c::SHIFT_MOVE_WIDE_OFFSET,
    c::SHIFT_MOVE_WIDE_WIDTH,
    c::SHIFT_MOVE_WIDE_MASK
  );
  field!(c::BIT_N_OFFSET, c::BIT_N_WIDTH, c::BIT_N_MASK);
  field!(
    c::IMM_ROTATE_OFFSET,
    c::IMM_ROTATE_WIDTH,
    c::IMM_ROTATE_MASK
  );
  field!(
    c::IMM_SET_BITS_OFFSET,
    c::IMM_SET_BITS_WIDTH,
    c::IMM_SET_BITS_MASK
  );
  field!(c::IMM_R_OFFSET, c::IMM_R_WIDTH, c::IMM_R_MASK);
  field!(c::IMM_S_OFFSET, c::IMM_S_WIDTH, c::IMM_S_MASK);
  field!(
    c::IMM_TEST_BRANCH_OFFSET,
    c::IMM_TEST_BRANCH_WIDTH,
    c::IMM_TEST_BRANCH_MASK
  );
  field!(
    c::IMM_TEST_BRANCH_BIT40_OFFSET,
    c::IMM_TEST_BRANCH_BIT40_WIDTH,
    c::IMM_TEST_BRANCH_BIT40_MASK
  );
  field!(
    c::IMM_TEST_BRANCH_BIT5_OFFSET,
    c::IMM_TEST_BRANCH_BIT5_WIDTH,
    c::IMM_TEST_BRANCH_BIT5_MASK
  );
  field!(c::CONDITION_OFFSET, c::CONDITION_WIDTH, c::CONDITION_MASK);
  field!(
    c::CONDITION_BRANCH_OFFSET,
    c::CONDITION_BRANCH_WIDTH,
    c::CONDITION_BRANCH_MASK
  );
  field!(c::NZCV_OFFSET, c::NZCV_WIDTH, c::NZCV_MASK);
  field!(
    c::IMM_COND_CMP_OFFSET,
    c::IMM_COND_CMP_WIDTH,
    c::IMM_COND_CMP_MASK
  );
  field!(
    c::IMM_COND_BRANCH_OFFSET,
    c::IMM_COND_BRANCH_WIDTH,
    c::IMM_COND_BRANCH_MASK
  );
  field!(c::FP_TYPE_OFFSET, c::FP_TYPE_WIDTH, c::FP_TYPE_MASK);
  field!(c::IMM_FP_OFFSET, c::IMM_FP_WIDTH, c::IMM_FP_MASK);
  field!(c::FP_SCALE_OFFSET, c::FP_SCALE_WIDTH, c::FP_SCALE_MASK);
  field!(c::IMM_LS_OFFSET, c::IMM_LS_WIDTH, c::IMM_LS_MASK);
  field!(
    c::IMM_LS_UNSIGNED_OFFSET,
    c::IMM_LS_UNSIGNED_WIDTH,
    c::IMM_LS_UNSIGNED_MASK
  );
  field!(
    c::IMM_LS_PAIR_OFFSET,
    c::IMM_LS_PAIR_WIDTH,
    c::IMM_LS_PAIR_MASK
  );
  field!(
    c::IMM_SHIFT_LS_OFFSET,
    c::IMM_SHIFT_LS_WIDTH,
    c::IMM_SHIFT_LS_MASK
  );
  field!(c::LS_OPC_OFFSET, c::LS_OPC_WIDTH, c::LS_OPC_MASK);
  field!(c::LS_VECTOR_OFFSET, c::LS_VECTOR_WIDTH, c::LS_VECTOR_MASK);
  field!(c::LS_SIZE_OFFSET, c::LS_SIZE_WIDTH, c::LS_SIZE_MASK);
  field!(c::NEON_Q_OFFSET, c::NEON_Q_WIDTH, c::NEON_Q_MASK);
  field!(c::NEON_SIZE_OFFSET, c::NEON_SIZE_WIDTH, c::NEON_SIZE_MASK);
  field!(
    c::NEON_LS_SIZE_OFFSET,
    c::NEON_LS_SIZE_WIDTH,
    c::NEON_LS_SIZE_MASK
  );
  field!(c::NEON_S_OFFSET, c::NEON_S_WIDTH, c::NEON_S_MASK);
  field!(c::NEON_L_OFFSET, c::NEON_L_WIDTH, c::NEON_L_MASK);
  field!(c::NEON_M_OFFSET, c::NEON_M_WIDTH, c::NEON_M_MASK);
  field!(c::NEON_H_OFFSET, c::NEON_H_WIDTH, c::NEON_H_MASK);
  field!(
    c::IMM_NEON_EXT_OFFSET,
    c::IMM_NEON_EXT_WIDTH,
    c::IMM_NEON_EXT_MASK
  );
  field!(
    c::IMM_NEON_5_OFFSET,
    c::IMM_NEON_5_WIDTH,
    c::IMM_NEON_5_MASK
  );
  field!(
    c::IMM_NEON_4_OFFSET,
    c::IMM_NEON_4_WIDTH,
    c::IMM_NEON_4_MASK
  );
  field!(
    c::IMM_UNCOND_BRANCH_OFFSET,
    c::IMM_UNCOND_BRANCH_WIDTH,
    c::IMM_UNCOND_BRANCH_MASK
  );
  field!(
    c::IMM_CMP_BRANCH_OFFSET,
    c::IMM_CMP_BRANCH_WIDTH,
    c::IMM_CMP_BRANCH_MASK
  );
  field!(
    c::IMM_L_LITERAL_OFFSET,
    c::IMM_L_LITERAL_WIDTH,
    c::IMM_L_LITERAL_MASK
  );
  field!(
    c::IMM_EXCEPTION_OFFSET,
    c::IMM_EXCEPTION_WIDTH,
    c::IMM_EXCEPTION_MASK
  );
  field!(c::IMM_HINT_OFFSET, c::IMM_HINT_WIDTH, c::IMM_HINT_MASK);
  field!(
    c::IMM_BARRIER_DOMAIN_OFFSET,
    c::IMM_BARRIER_DOMAIN_WIDTH,
    c::IMM_BARRIER_DOMAIN_MASK
  );
  field!(
    c::IMM_BARRIER_TYPE_OFFSET,
    c::IMM_BARRIER_TYPE_WIDTH,
    c::IMM_BARRIER_TYPE_MASK
  );
  field!(
    c::IMM_SYSTEM_REGISTER_OFFSET,
    c::IMM_SYSTEM_REGISTER_WIDTH,
    c::IMM_SYSTEM_REGISTER_MASK
  );
  field!(c::SYS_O0_OFFSET, c::SYS_O0_WIDTH, c::SYS_O0_MASK);
  field!(c::SYS_OP1_OFFSET, c::SYS_OP1_WIDTH, c::SYS_OP1_MASK);
  field!(c::SYS_OP2_OFFSET, c::SYS_OP2_WIDTH, c::SYS_OP2_MASK);
  field!(c::CRN_OFFSET, c::CRN_WIDTH, c::CRN_MASK);
  field!(c::CRM_OFFSET, c::CRM_WIDTH, c::CRM_MASK);
  field!(
    c::LOAD_STORE_X_LOAD_OFFSET,
    c::LOAD_STORE_X_LOAD_WIDTH,
    c::LOAD_STORE_X_LOAD_MASK
  );
  field!(
    c::LOAD_STORE_X_NOT_EXCLUSIVE_OFFSET,
    c::LOAD_STORE_X_NOT_EXCLUSIVE_WIDTH,
    c::LOAD_STORE_X_NOT_EXCLUSIVE_MASK
  );
  field!(
    c::LOAD_STORE_X_ACQUIRE_RELEASE_OFFSET,
    c::LOAD_STORE_X_ACQUIRE_RELEASE_WIDTH,
    c::LOAD_STORE_X_ACQUIRE_RELEASE_MASK
  );
  field!(
    c::LOAD_STORE_X_SIZE_LOG2_OFFSET,
    c::LOAD_STORE_X_SIZE_LOG2_WIDTH,
    c::LOAD_STORE_X_SIZE_LOG2_MASK
  );
  field!(
    c::LOAD_STORE_X_PAIR_OFFSET,
    c::LOAD_STORE_X_PAIR_WIDTH,
    c::LOAD_STORE_X_PAIR_MASK
  );
  field!(c::NEON_LOAD_OFFSET, c::NEON_LOAD_WIDTH, c::NEON_LOAD_MASK);
  field!(
    c::IMM_NEON_ABC_OFFSET,
    c::IMM_NEON_ABC_WIDTH,
    c::IMM_NEON_ABC_MASK
  );
  field!(
    c::IMM_NEON_DEFGH_OFFSET,
    c::IMM_NEON_DEFGH_WIDTH,
    c::IMM_NEON_DEFGH_MASK
  );
  field!(
    c::NEON_MOD_IMM_OP_OFFSET,
    c::NEON_MOD_IMM_OP_WIDTH,
    c::NEON_MOD_IMM_OP_MASK
  );
  field!(
    c::NEON_CMODE_OFFSET,
    c::NEON_CMODE_WIDTH,
    c::NEON_CMODE_MASK
  );
  field!(
    c::IMM_NEON_IMMH_IMMB_OFFSET,
    c::IMM_NEON_IMMH_IMMB_WIDTH,
    c::IMM_NEON_IMMH_IMMB_MASK
  );
  field!(
    c::IMM_NEON_IMMH_OFFSET,
    c::IMM_NEON_IMMH_WIDTH,
    c::IMM_NEON_IMMH_MASK
  );
  field!(
    c::IMM_NEON_IMMB_OFFSET,
    c::IMM_NEON_IMMB_WIDTH,
    c::IMM_NEON_IMMB_MASK
  );
  field!(c::FLAGS_OFFSET, c::FLAGS_WIDTH, c::FLAGS_MASK);
  field!(c::N_OFFSET, c::N_WIDTH, c::N_MASK);
  field!(c::Z_OFFSET, c::Z_WIDTH, c::Z_MASK);
  field!(c::C_OFFSET, c::C_WIDTH, c::C_MASK);
  field!(c::V_OFFSET, c::V_WIDTH, c::V_MASK);
  field!(c::AHP_OFFSET, c::AHP_WIDTH, c::AHP_MASK);
  field!(c::DN_OFFSET, c::DN_WIDTH, c::DN_MASK);
  field!(c::FZ_OFFSET, c::FZ_WIDTH, c::FZ_MASK);
  field!(c::R_MODE_OFFSET, c::R_MODE_WIDTH, c::R_MODE_MASK);

  // 复合域: ImmPCRelMask = Lo | Hi (constants-arm64.h:307)
  assert_eq!(
    c::IMM_PC_REL_MASK,
    c::IMM_PC_REL_LO_MASK | c::IMM_PC_REL_HI_MASK
  );

  // Generic fields 组合关系
  assert_eq!(c::K_UNALLOCATED_INSTRUCTION, u32::MAX);
  assert_eq!(c::THIRTY_TWO_BITS, 0);
  assert_eq!(c::FP32, c::THIRTY_TWO_BITS);
  assert_eq!(c::FP64, c::NEON_SZ);
  assert_eq!(c::NEON_16B, c::NEON_8B | c::NEON_Q);
  assert_eq!(c::NEON_8H, c::NEON_4H | c::NEON_Q);
  assert_eq!(c::NEON_4S, c::NEON_2S | c::NEON_Q);
  assert_eq!(c::NEON_2D, c::NEON_1D | c::NEON_Q);
  assert_eq!(c::NEON_FP_8H, c::NEON_FP_4H | c::NEON_Q);
  assert_eq!(c::NEON_FP_4S, c::NEON_FP_2S | c::NEON_Q);
  assert_eq!(c::NEON_FP_2D, c::NEON_FP_2S | c::NEON_Q | c::NEON_SZ);
  assert_eq!(c::LS_NEON_16B, c::LS_NEON_8B | c::NEON_Q);
  assert_eq!(c::LS_NEON_8H, c::LS_NEON_4H | c::NEON_Q);
  assert_eq!(c::LS_NEON_4S, c::LS_NEON_2S | c::NEON_Q);
  assert_eq!(c::LS_NEON_2D, c::LS_NEON_1D | c::NEON_Q);
  assert_eq!(c::NEON_FORMAT_FIELD_MASK, c::NEON_Q | c::NEON_SIZE_MASK);
  assert_eq!(c::NEON_FP_FORMAT_FIELD_MASK, c::NEON_Q | c::NEON_SZ);
  assert_eq!(
    c::NEON_LS_FORMAT_FIELD_MASK,
    c::NEON_Q | c::NEON_LS_SIZE_MASK
  );
  assert_eq!(c::NEON_SCALAR_FORMAT_FIELD_MASK, c::NEON_SIZE_MASK);
  assert_eq!(c::SVE_SIZE_FIELD_MASK, c::NEON_SIZE_MASK);
  assert_eq!(c::NEON_B, 0);
  assert_eq!(c::NEON_H, c::NEON_SZ);
  assert_eq!(c::NEON_S, c::NEON_SZ << 1);
  assert_eq!(c::NEON_D, c::NEON_SIZE_MASK);
  assert_eq!(c::SVE_B, c::NEON_B);
  assert_eq!(c::SVE_H, c::NEON_H);
  assert_eq!(c::SVE_S, c::NEON_S);
  assert_eq!(c::SVE_D, c::NEON_D);

  // Condition 16 码逐值 golden (constants-arm64.h:310-327)
  assert_eq!(
    [
      c::EQ,
      c::NE,
      c::CS,
      c::CC,
      c::MI,
      c::PL,
      c::VS,
      c::VC,
      c::HI,
      c::LS,
      c::GE,
      c::LT,
      c::GT,
      c::LE,
      c::AL,
      c::NV
    ],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
  );
  assert_eq!(c::HS, c::CS);
  assert_eq!(c::LO, c::CC);
  assert_eq!(c::K_EQUAL, c::EQ);
  assert_eq!(c::K_NOT_EQUAL, c::NE);
  assert_eq!(c::K_ZERO, c::EQ);
  assert_eq!(c::K_NOT_ZERO, c::NE);
  assert_eq!(c::K_UNSIGNED_GREATER_THAN_EQUAL, c::CS);
  assert_eq!(c::K_UNSIGNED_LESS_THAN, c::CC);
  assert_eq!(c::K_OVERFLOW, c::VS);
  assert_eq!(c::K_NO_OVERFLOW, c::VC);
  assert_eq!(c::K_UNSIGNED_GREATER_THAN, c::HI);
  assert_eq!(c::K_UNSIGNED_LESS_THAN_EQUAL, c::LS);
  assert_eq!(c::K_GREATER_THAN_EQUAL, c::GE);
  assert_eq!(c::K_LESS_THAN, c::LT);
  assert_eq!(c::K_GREATER_THAN, c::GT);
  assert_eq!(c::K_LESS_THAN_EQUAL, c::LE);

  // NegateCondition: 奇偶配对 (EQ..LE 共 7 对), al/nv 除外
  for k in 0..7i32 {
    assert_eq!(c::negate_condition(2 * k), 2 * k + 1);
    assert_eq!(c::negate_condition(2 * k + 1), 2 * k);
  }

  // StatusFlags 由 N/Z/C/V 域推导
  assert_eq!(c::NO_FLAG, 0);
  assert_eq!(c::N_FLAG, c::N_MASK);
  assert_eq!(c::Z_FLAG, c::Z_MASK);
  assert_eq!(c::C_FLAG, c::C_MASK);
  assert_eq!(c::V_FLAG, c::V_MASK);
  assert_eq!(c::NZ_FLAG, c::N_FLAG | c::Z_FLAG);
  assert_eq!(c::NZCV_FLAG, c::N_FLAG | c::Z_FLAG | c::C_FLAG | c::V_FLAG);
  assert_eq!(c::FP_EQUAL_FLAG, c::ZC_FLAG);
  assert_eq!(c::FP_LESS_THAN_FLAG, c::N_FLAG);
  assert_eq!(c::FP_GREATER_THAN_FLAG, c::C_FLAG);
  assert_eq!(c::FP_UNORDERED_FLAG, c::CV_FLAG);

  // SystemRegister 编码逐值 golden (NZCV=0x5A10, FPCR=0x5A20)
  assert_eq!(c::NZCV, 0x5A10);
  assert_eq!(c::FPCR, 0x5A20);

  // BranchTargetIdentifier 判别值 0..5
  assert_eq!(c::BranchTargetIdentifier::None as u8, 0);
  assert_eq!(c::BranchTargetIdentifier::Bti as u8, 1);
  assert_eq!(c::BranchTargetIdentifier::BtiCall as u8, 2);
  assert_eq!(c::BranchTargetIdentifier::BtiJump as u8, 3);
  assert_eq!(c::BranchTargetIdentifier::BtiJumpCall as u8, 4);
  assert_eq!(c::BranchTargetIdentifier::Pacibsp as u8, 5);

  // FlagsUpdate / Shift / Extend / Barrier / SystemHint 数值锚
  assert_eq!((c::SET_FLAGS, c::LEAVE_FLAGS, c::NO_SHIFT), (1, 0, -1));
  assert_eq!((c::LSL, c::LSR, c::ASR, c::ROR, c::MSL), (0, 1, 2, 3, 4));
  assert_eq!((c::NO_EXTEND, c::UXTB, c::SXTX), (-1, 0, 7));
  assert_eq!(
    (
      c::BARRIER_OTHER,
      c::BARRIER_READS,
      c::BARRIER_WRITES,
      c::BARRIER_ALL
    ),
    (0, 1, 2, 3)
  );
  assert_eq!(
    (
      c::OUTER_SHAREABLE,
      c::NON_SHAREABLE,
      c::INNER_SHAREABLE,
      c::FULL_SYSTEM
    ),
    (0, 1, 2, 3)
  );
  assert_eq!(
    (c::NOP, c::YIELD, c::WFE, c::WFI, c::SEV, c::SEVL),
    (0, 1, 2, 3, 4, 5)
  );
  assert_eq!(
    (c::CSDB, c::BTI, c::BTI_C, c::BTI_J, c::BTI_JC),
    (20, 32, 34, 36, 38)
  );
  OK
}

#[test]
fn test_wave2_utils() -> Void {
  use v8_codegen_arm64::utils::{
    FpClass, count_leading_sign_bits, count_leading_zeros, count_set_bits, count_trailing_zeros,
    double_exp, double_mantissa, double_pack, double_sign, float_exp, float_mantissa, float_pack,
    float_sign, float16classify, fused_multiply_add, fused_multiply_add_f32,
    highest_set_bit_position, is_quiet_nan_f32, is_quiet_nan_f64, is_signalling_nan_f16,
    is_signalling_nan_f32, is_signalling_nan_f64, largest_power_of_2_divisor,
    lowest_set_bit_position, mask_to_bit, reverse_bytes, to_quiet_nan_f32, to_quiet_nan_f64,
  };

  // 1. reverse_bytes REV16 / REV32 / REV64
  let val: u64 = 0x0123_4567_89ab_cdef;
  assert_eq!(reverse_bytes(val, 1), 0x2301_6745_ab89_efcd);
  assert_eq!(reverse_bytes(val, 2), 0x6745_2301_efcd_ab89);
  assert_eq!(reverse_bytes(val, 3), 0xefcd_ab89_6745_2301);

  // 2. Float decomposition & pack roundtrip
  let test_floats = [
    0.0f32,
    -0.0f32,
    1.0f32,
    -1.0f32,
    42.5f32,
    1e-35f32,
    f32::INFINITY,
    f32::NEG_INFINITY,
  ];
  for &f in &test_floats {
    let sign = float_sign(f);
    let exp = float_exp(f);
    let mant = float_mantissa(f);
    let packed = float_pack(sign, exp, mant);
    assert_eq!(f.to_bits(), packed.to_bits());
  }

  // 3. Double decomposition & pack roundtrip
  let test_doubles = [
    0.0f64,
    -0.0f64,
    1.0f64,
    -1.0f64,
    PI,
    1e-300f64,
    f64::INFINITY,
    f64::NEG_INFINITY,
  ];
  for &d in &test_doubles {
    let sign = double_sign(d);
    let exp = double_exp(d);
    let mant = double_mantissa(d);
    let packed = double_pack(sign as u64, exp as u64, mant);
    assert_eq!(d.to_bits(), packed.to_bits());
  }

  // 4. float16classify
  assert_eq!(float16classify(0x0000), FpClass::FpZero as i32);
  assert_eq!(float16classify(0x8000), FpClass::FpZero as i32);
  assert_eq!(float16classify(0x0001), FpClass::FpSubnormal as i32);
  assert_eq!(float16classify(0x03ff), FpClass::FpSubnormal as i32);
  assert_eq!(float16classify(0x0400), FpClass::FpNormal as i32);
  assert_eq!(float16classify(0x3c00), FpClass::FpNormal as i32);
  assert_eq!(float16classify(0x7bff), FpClass::FpNormal as i32);
  assert_eq!(float16classify(0x7c00), FpClass::FpInfinite as i32);
  assert_eq!(float16classify(0xfc00), FpClass::FpInfinite as i32);
  assert_eq!(float16classify(0x7c01), FpClass::FpNan as i32);
  assert_eq!(float16classify(0x7e00), FpClass::FpNan as i32);

  // 5. f16 signalling NaN check
  assert!(is_signalling_nan_f16(0x7c01));
  assert!(!is_signalling_nan_f16(0x7e00)); // quiet bit 0x0200 set

  // 6. f32 / f64 quiet & signalling NaN
  let s_nan_mask: u32 = 0x0040_0000;
  let d_nan_mask: u64 = 0x0008_0000_0000_0000;
  let quiet_f32 = f32::from_bits(0x7fc0_0000);
  let signal_f32 = f32::from_bits(0x7f80_0001);
  assert!(is_quiet_nan_f32(quiet_f32, s_nan_mask));
  assert!(!is_signalling_nan_f32(quiet_f32, s_nan_mask));
  assert!(is_signalling_nan_f32(signal_f32, s_nan_mask));
  assert_eq!(
    to_quiet_nan_f32(signal_f32, s_nan_mask).to_bits(),
    signal_f32.to_bits() | s_nan_mask
  );

  let quiet_f64 = f64::from_bits(0x7ff8_0000_0000_0000);
  let signal_f64 = f64::from_bits(0x7ff0_0000_0000_0001);
  assert!(is_quiet_nan_f64(quiet_f64, d_nan_mask));
  assert!(!is_signalling_nan_f64(quiet_f64, d_nan_mask));
  assert!(is_signalling_nan_f64(signal_f64, d_nan_mask));
  assert_eq!(
    to_quiet_nan_f64(signal_f64, d_nan_mask).to_bits(),
    signal_f64.to_bits() | d_nan_mask
  );

  // 7. Bit count & manipulation utilities
  assert_eq!(count_leading_zeros(0, 32), 32);
  assert_eq!(count_leading_zeros(1, 32), 31);
  assert_eq!(count_leading_zeros(0x8000_0000, 32), 0);
  assert_eq!(count_trailing_zeros(0, 64), 64);
  assert_eq!(count_trailing_zeros(0b1000, 64), 3);

  assert_eq!(count_leading_sign_bits(0, 32), 31);
  assert_eq!(count_leading_sign_bits(-1, 32), 31);
  assert_eq!(count_leading_sign_bits(1, 32), 30);
  assert_eq!(count_leading_sign_bits(-2, 32), 30);

  assert_eq!(count_set_bits(0x5555_5555, 32), 16);
  assert_eq!(count_set_bits(0xFFFF_FFFF_FFFF_FFFF, 64), 64);

  assert_eq!(lowest_set_bit_position(1), 1);
  assert_eq!(lowest_set_bit_position(8), 4);
  assert_eq!(highest_set_bit_position(1), 0);
  assert_eq!(highest_set_bit_position(8), 3);
  assert_eq!(highest_set_bit_position(0x8000_0000_0000_0000), 63);

  assert_eq!(mask_to_bit(1), 0);
  assert_eq!(mask_to_bit(1 << 15), 15);
  assert_eq!(largest_power_of_2_divisor(48), 16);
  assert_eq!(largest_power_of_2_divisor(12), 4);
  assert_eq!(largest_power_of_2_divisor(7), 1);

  // 8. FMA
  assert_eq!(fused_multiply_add(2.0, 3.0, 4.0), 10.0);
  assert_eq!(fused_multiply_add_f32(2.0f32, 3.0f32, 4.0f32), 10.0f32);

  OK
}

#[test]
fn test_wave2_reglist() -> Void {
  use v8_codegen_arm64::reglist::CPURegList as Arm64CPURegList;

  // 1. AAPCS64 convention register lists
  let callee = Arm64CPURegList::get_callee_saved();
  assert_eq!(callee.count(), 10);
  for r in 19..=28 {
    assert!(callee.includes_alias_of(Register::x(r)));
  }
  assert!(!callee.includes_alias_of(Register::x(0)));
  assert!(!callee.includes_alias_of(Register::x(29)));

  let callee_d = Arm64CPURegList::get_callee_saved_d();
  assert_eq!(callee_d.count(), 8);
  for r in 8..=15 {
    assert!(callee_d.includes_alias_of(VRegister::d(r)));
  }

  let callee_v = Arm64CPURegList::get_callee_saved_v();
  assert_eq!(callee_v.count(), 0); // AAPCS64 specifies d8..d15, V list is 0
  assert_eq!(callee_v.register_size_in_bits(), 128);

  let caller = Arm64CPURegList::get_caller_saved();
  assert_eq!(caller.count(), 18); // x0..=x17
  for r in 0..=17 {
    assert!(caller.includes_alias_of(Register::x(r)));
  }

  let caller_d = Arm64CPURegList::get_caller_saved_d();
  assert_eq!(caller_d.count(), 24); // d0..d7, d16..d31

  let caller_v = Arm64CPURegList::get_caller_saved_v();
  assert_eq!(caller_v.count(), 32);

  // 2. Combine and Remove
  let mut list = Arm64CPURegList::from_gp(64, 0);
  assert!(list.is_empty());
  list.combine(&Arm64CPURegList::from_gp(64, (1 << 0) | (1 << 5)));
  assert_eq!(list.count(), 2);
  assert!(list.includes_alias_of(Register::x(0)));
  assert!(list.includes_alias_of(Register::x(5)));

  list.remove(&Arm64CPURegList::from_gp(64, 1 << 0));
  assert_eq!(list.count(), 1);
  assert!(!list.includes_alias_of(Register::x(0)));

  // 3. Align (odd count toggles x31)
  let mut list_1 = Arm64CPURegList::from_gp(64, 1 << 0); // count = 1 (odd)
  list_1.align();
  assert_eq!(list_1.count(), 2);
  assert_eq!(list_1.bits() & (1 << 31), 1 << 31);

  let mut list_2 = Arm64CPURegList::from_gp(64, (1 << 0) | (1 << 1)); // count = 2 (even)
  list_2.align();
  assert_eq!(list_2.count(), 2); // untouched

  // 4. Pop lowest & highest
  let mut pop_list = Arm64CPURegList::from_gp(64, (1 << 2) | (1 << 8) | (1 << 20));
  assert_eq!(pop_list.pop_lowest_index(), 2);
  assert_eq!(pop_list.pop_highest_index(), 20);
  assert_eq!(pop_list.pop_lowest_index(), 8);
  assert_eq!(pop_list.pop_lowest_index(), u32::MAX);
  assert!(pop_list.is_empty());

  OK
}

#[test]
fn test_wave2_constant_pool() -> Void {
  use v8_codegen_arm64::constant_pool::{
    Alignment, ConstantPool, ConstantPoolKey, Emission, Jump, RelocInfoStatus,
  };

  // 1. ConstantPoolKey properties and deduplication rules
  let k32 = ConstantPoolKey::from_value32(0xdead_beef, 0);
  assert!(k32.is_value32());
  assert_eq!(k32.value32(), 0xdead_beef);
  assert!(k32.allows_deduplication());

  let k64 = ConstantPoolKey::from_value64(0xcafe_babe_feed_face, 0);
  assert!(!k64.is_value32());
  assert_eq!(k64.value64(), 0xcafe_babe_feed_face);
  assert!(k64.allows_deduplication());

  // 64-bit entries sort before 32-bit entries for alignment
  assert!(k64 < k32);

  // 2. ConstantPool basic operations
  let mut pool = ConstantPool::new(null_mut());
  assert!(pool.is_empty());
  assert_eq!(pool.entry32_count(), 0);
  assert_eq!(pool.entry64_count(), 0);

  // Record 32-bit entry
  let st1 = pool.record_entry32(0x1122_3344, 0, 100);
  assert_eq!(st1, RelocInfoStatus::MustRecord);
  assert_eq!(pool.entry32_count(), 1);

  // Record identical entry with NO_INFO -> deduplicated!
  let st2 = pool.record_entry32(0x1122_3344, 0, 104);
  assert_eq!(st2, RelocInfoStatus::MustOmitForDuplicate);
  assert_eq!(pool.entry32_count(), 1);

  // Record 64-bit entry
  let st3 = pool.record_entry64(0x8877_6655_4433_2211, 0, 108);
  assert_eq!(st3, RelocInfoStatus::MustRecord);
  assert_eq!(pool.entry64_count(), 1);
  assert!(!pool.is_empty());

  // 3. Prologue and size computations
  assert_eq!(ConstantPool::prologue_size(Jump::Required), 12);
  assert_eq!(ConstantPool::prologue_size(Jump::Omitted), 8);
  let sz = pool.compute_size(Jump::Required, Alignment::Omitted);
  // 12 (prologue) + 4 (32-bit) + 0 (pad) + 8 (64-bit) = 24
  assert_eq!(sz, 24);

  // 4. Imm range check
  assert!(pool.is_in_imm_range_if_emitted_at(120));

  // 5. Blocking
  assert!(!pool.is_blocked());
  pool.start_block();
  assert!(pool.is_blocked());
  assert!(!pool.check(Emission::IfNeeded, Jump::Required, 0, 200));
  pool.end_block();
  assert!(!pool.is_blocked());

  // 6. Emit and clear
  let emitted = pool.emit_and_clear(Jump::Required);
  assert_eq!(emitted.len(), 2);
  assert!(pool.is_empty());
  assert_eq!(pool.entry32_count(), 0);
  assert_eq!(pool.entry64_count(), 0);

  // 7. Non-deduplicatable entries (e.g. CodeTarget with value 0)
  use v8_codegen_core::RelocMode;
  let non_dedup_key = ConstantPoolKey::from_value32(0, RelocMode::CodeTarget as u32);
  assert!(!non_dedup_key.allows_deduplication());
  let mut non_dedup_pool = ConstantPool::new(null_mut());
  let s1 = non_dedup_pool.record_entry32(0, RelocMode::CodeTarget as u32, 10);
  assert_eq!(s1, RelocInfoStatus::MustRecord);
  let s2 = non_dedup_pool.record_entry32(0, RelocMode::CodeTarget as u32, 20);
  assert_eq!(s2, RelocInfoStatus::MustRecord);
  assert_eq!(non_dedup_pool.entry32_count(), 2);
  let non_dedup_emitted = non_dedup_pool.emit_and_clear(Jump::Required);
  assert_eq!(non_dedup_emitted.len(), 2);

  OK
}

#[test]
fn test_wave2_instructions() -> Void {
  use v8_codegen_arm64::instructions::{
    Instruction, fp_to_imm8, imm8_to_fp32, imm8_to_fp64, is_imm_fp32, is_imm_fp64,
  };

  // 1. Bit extraction and register decoding
  // Encoding for: ADD X0, X1, #42 -> 0x9100a820
  let instr = Instruction::new(0x9100_a820);
  assert!(instr.sixty_four_bits());
  assert_eq!(instr.rd(), 0);
  assert_eq!(instr.rn(), 1);
  assert_eq!(instr.imm_add_sub(), 42);
  assert_eq!(instr.bit(31), 1);
  assert_eq!(instr.bit_range(9, 5), 1);
  assert_eq!(instr.bit_range(4, 0), 0);

  // 2. Signed bit range
  // Negative 5-bit number in bits 4..0: 0x1F (-1)
  let instr_neg = Instruction::new(0x1F);
  assert_eq!(instr_neg.signed_bit_range(4, 0), -1);

  // 3. imm_pc_rel (ADR 21 位相对有符号偏移)
  assert_eq!(Instruction::new(0x1000_0000).imm_pc_rel(), 0);
  assert_eq!(Instruction::new(0x3000_0000).imm_pc_rel(), 1);
  assert_eq!(Instruction::new(0x1000_0020).imm_pc_rel(), 4);
  assert_eq!(Instruction::new(0x70FF_FFE0).imm_pc_rel(), -1);
  assert_eq!(Instruction::new(0x10FF_FFE0).imm_pc_rel(), -4);
  assert_eq!(Instruction::new(0x707F_FFE0).imm_pc_rel(), 1048575);
  assert_eq!(Instruction::new(0x1080_0000).imm_pc_rel(), -1048576);

  // 4. imm_logical (逻辑立即数解码)
  // 64-bit 单比特 (N=1, imms=0, immr=0) -> 0x0000_0000_0000_0001
  assert_eq!(Instruction::new(0xB240_03E0).imm_logical(), 1u64);
  // 64-bit 跨 32 位重复单比特 (N=0, imms=0, immr=0) -> 0x0000_0001_0000_0001
  assert_eq!(
    Instruction::new(0xB200_03E0).imm_logical(),
    0x0000_0001_0000_0001u64
  );
  // 64-bit 63 个连续 1 (N=1, imms=62, immr=0)
  assert_eq!(
    Instruction::new(0xB240_F800).imm_logical(),
    0x7FFF_FFFF_FFFF_FFFFu64
  );
  // 64-bit imms=63 全 1 (非法编码/无法表示 0) 返回 0
  assert_eq!(Instruction::new(0xB240_FC00).imm_logical(), 0u64);
  // 32-bit 宽度 32 掩码 0xFF (SF=0, N=0, imms=7, immr=0)
  assert_eq!(Instruction::new(0x3200_1C00).imm_logical(), 0x00FFu64);
  // 64-bit 宽度 32 掩码 0xFF 跨寄存器复制 (SF=1, N=0, imms=7, immr=0)
  assert_eq!(
    Instruction::new(0xB200_1C00).imm_logical(),
    0x0000_00FF_0000_00FFu64
  );
  // 64-bit 宽度 16 掩码 0x0F 四重复制 (SF=1, N=0, imms=0x23, immr=0)
  assert_eq!(
    Instruction::new(0xB200_8C00).imm_logical(),
    0x000F_000F_000F_000Fu64
  );
  // 64-bit 宽度 32 掩码 0xFF 右旋 4 位并在 64 位内重复 (SF=1, N=0, imms=7, immr=4)
  assert_eq!(
    Instruction::new(0xB204_1C00).imm_logical(),
    0xF000_000F_F000_000Fu64
  );

  // 5. FP immediate roundtrips
  for imm in [0x00u32, 0x10, 0x55, 0x7F, 0x80, 0xFF] {
    let f32_val = imm8_to_fp32(imm);
    assert!(is_imm_fp32(f32_val.to_bits()));

    let f64_val = imm8_to_fp64(imm);
    assert!(is_imm_fp64(f64_val.to_bits()));
    assert_eq!(fp_to_imm8(f64_val), imm);
  }

  // Arbitrary number not encodable as 8-bit FP imm
  assert!(!is_imm_fp32(12345.678f32.to_bits()));
  assert!(!is_imm_fp64(12345.678f64.to_bits()));

  OK
}

#[test]
fn test_wave2_decoder() -> Void {
  use v8_codegen_arm64::{
    decoder::{Decoder, DecoderVisitor, DispatchingDecoderVisitor},
    instructions::Instruction,
  };

  struct TrackingVisitor {
    add_sub_count: usize,
    branch_count: usize,
    last_rd: u32,
  }

  impl DecoderVisitor for TrackingVisitor {
    fn visit_add_sub_immediate(&mut self, instr: &Instruction) {
      self.add_sub_count += 1;
      self.last_rd = instr.rd();
    }

    fn visit_unconditional_branch(&mut self, _instr: &Instruction) {
      self.branch_count += 1;
    }
  }

  // ADD X5, X1, #100 -> 0x91019025
  let add_instr = Instruction::new(0x9101_9025);
  // B #16 (offset = 4 instructions) -> 0x14000004
  let b_instr = Instruction::new(0x1400_0004);

  // 1. Direct Decoder dispatch
  let mut tracker = TrackingVisitor {
    add_sub_count: 0,
    branch_count: 0,
    last_rd: 99,
  };
  {
    let mut decoder = Decoder::new(&mut tracker);
    decoder.decode(&add_instr);
    decoder.decode(&b_instr);
  }
  assert_eq!(tracker.add_sub_count, 1);
  assert_eq!(tracker.last_rd, 5);
  assert_eq!(tracker.branch_count, 1);

  // 2. DispatchingDecoderVisitor dispatch & remove
  let mut dispatch = DispatchingDecoderVisitor::new();
  dispatch.append_visitor(&mut tracker as *mut dyn DecoderVisitor);
  assert_eq!(dispatch.visitors_count(), 1);

  dispatch.decode(&add_instr);
  assert_eq!(tracker.add_sub_count, 2);

  dispatch.remove_visitor(&mut tracker as *mut dyn DecoderVisitor);
  assert_eq!(dispatch.visitors_count(), 0);

  dispatch.decode(&add_instr);
  assert_eq!(tracker.add_sub_count, 2); // no change

  // 3. Visitor ordering: Append, Prepend, InsertBefore/After (aligned with V8 decoder-arm64.h:119-128)
  struct OrderVisitor {
    id: usize,
    log: *mut Vec<usize>,
  }
  impl DecoderVisitor for OrderVisitor {
    fn visit_add_sub_immediate(&mut self, _instr: &Instruction) {
      unsafe { (*self.log).push(self.id) };
    }
  }

  let mut log = Vec::new();
  let mut v1 = OrderVisitor {
    id: 1,
    log: &mut log,
  };
  let mut v2 = OrderVisitor {
    id: 2,
    log: &mut log,
  };
  let mut v3 = OrderVisitor {
    id: 3,
    log: &mut log,
  };
  let mut v4 = OrderVisitor {
    id: 4,
    log: &mut log,
  };

  let mut d = DispatchingDecoderVisitor::new();
  d.append_visitor(&mut v1 as *mut dyn DecoderVisitor);
  d.append_visitor(&mut v2 as *mut dyn DecoderVisitor);
  d.prepend_visitor(&mut v2 as *mut dyn DecoderVisitor);
  d.insert_visitor_before(
    &mut v3 as *mut dyn DecoderVisitor,
    &mut v2 as *mut dyn DecoderVisitor,
  );
  d.append_visitor(&mut v4 as *mut dyn DecoderVisitor);
  d.append_visitor(&mut v4 as *mut dyn DecoderVisitor);
  d.decode(&add_instr);
  assert_eq!(log, vec![3, 2, 1, 4]);

  OK
}

#[test]
fn test_wave2_cpu() -> Void {
  use v8_codegen_arm64::cpu::{
    CacheLineSizes, DCACHE_LINE_SIZE_SHIFT, ICACHE_LINE_SIZE_SHIFT, flush_i_cache,
  };

  let cls = CacheLineSizes::new();
  // With cache_type_register = 0: 4 << 0 = 4 bytes
  assert_eq!(cls.icache_line_size(), 4);
  assert_eq!(cls.dcache_line_size(), 4);

  // Verify extract_cache_line_size constants
  assert_eq!(ICACHE_LINE_SIZE_SHIFT, 0);
  assert_eq!(DCACHE_LINE_SIZE_SHIFT, 16);

  // Safe invocation of flush_i_cache on memory buffer
  let mut code_buf = [0xd503201fu32; 16]; // NOP instructions
  flush_i_cache(code_buf.as_mut_ptr() as *mut u8, code_buf.len() * 4);

  // Safe invocation on null pointer or zero length
  flush_i_cache(null_mut(), 0);
  flush_i_cache(null_mut(), 1024);

  OK
}
