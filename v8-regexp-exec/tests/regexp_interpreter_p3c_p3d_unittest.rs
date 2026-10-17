//! P-3c 反向引用/锚点 (9) + P-3d skip-until peephole (7) handler 落地测试
//! (REG-1b4, 承 REG-1b3 已落地的 exec_backref_ops / exec_skip_ops 两组)。
//!
//! 规约映射 (实测 v8/test/unittests/regexp/ 目录无 interpreter 级纯函数单测,
//! 全部 Isolate 硬耦合 ExecuteForTesting 路径; 按票规约 3 手工复刻 opcode 级断言):
//! - 逐 handler 两臂 (Jump 目标臂 / ADVANCE 臂) + 副作用面 (current / current_char)
//! - 原版 `||` 短路序逐臂验证: 界检先于比较核 (cc:884/912/926), 后向态 `current-len < 0`
//!   先于比较核 (cc:898/941/956)
//! - 未捕获回跳 (registers[i] = kNoMatchValue = -1) → cc:883 `from >= 0 && len > 0` 守卫臂
//! - P-3c-icu 运行态推迟通道: 四个 NoCase handler 两字节态 → Step::NotImplemented
//!   { cc_line: 46, batch: "P-3c-icu" } (禁伪装结果, 原版 uc16 重载 cc:46-61 → ICU)
//! - P-3d 循环面: on_match / on_no_match / on_failure 三出口 + SkipUntilOneOfMasked3
//!   的 bc0..bc7 复合体五臂 (bc1_on_failure / bc5 / bc6 / bc7-fallthrough / bc3 continue 回边)
//! - 断言强度: pc (ADVANCE ≡ bytecode_size, Jump ≡ 操作数值)、current、current_char 三路。
//!
//! 纪律: 文件级 #[ctor::ctor(unsafe)] 日志, 每 #[test] fn 返回 aok::Void。

use aok::{OK, Void};
use log::info;
use v8_regexp_exec::regexp_interpreter::{
  Bytecode, K_BYTECODE_COUNT, RawMatch, RawMatchParams, Step, TraceHooks, bytecode_size,
  operand_count, operand_offset, operand_size,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[derive(Default)]
struct Asm(Vec<u8>);

impl Asm {
  fn insn(&mut self, bc: Bytecode, ops: &[Vec<u8>]) -> usize {
    assert_eq!(ops.len(), operand_count(bc), "{bc:?} 操作数个数");
    let at = self.0.len();
    let size = usize::from(bytecode_size(bc));
    self.0.resize(at + size, 0);
    self.0[at] = bc.to_byte();
    for (i, bytes) in ops.iter().enumerate() {
      let width = usize::from(operand_size(bc, i));
      assert_eq!(bytes.len(), width, "{bc:?} op{i} 宽度");
      let off = at + operand_offset(bc, i);
      self.0[off..off + width].copy_from_slice(bytes);
    }
    at
  }
  fn code(&self) -> &[u8] {
    &self.0
  }
}

fn w2(v: u16) -> Vec<u8> {
  v.to_ne_bytes().to_vec()
}
fn w2i(v: i16) -> Vec<u8> {
  v.to_ne_bytes().to_vec()
}
fn w4(v: u32) -> Vec<u8> {
  v.to_ne_bytes().to_vec()
}

/// kBitTable 操作数定宽 16 字节 (regexp-bytecodes.h:43 OperandType::kBitTable)
fn table(chars: &[u8]) -> Vec<u8> {
  let mut t = vec![0u8; 16];
  for &c in chars {
    let u = u32::from(c);
    // CheckBitInTable 窗口式 (cc:321-326): 字节 = (c & 127) >> 3, 位 = c & 7
    t[usize::try_from((u & 127) >> 3).unwrap_or(usize::MAX)] |= 1u8 << (u & 7);
  }
  t
}

/// 反向引用寄存器对: registers[start_reg] = from, registers[start_reg+1] = from+len
const START_REG: usize = 2;
/// ADVANCE 臂落点之外的独立跳转目标 (单指令面 step_limit=1, 不实际派发)
const TGT_JUMP: u32 = 64;
const TGT_JUMP2: u32 = 68;
const TGT_JUMP3: u32 = 72;

const SUBJ8: &[u8] = b"ababc";
/// 大小写折叠面专用: s[0..2]="ab", s[2..4]="AB" (case-sensitive 不等, 折等)
const SUBJ8CI: &[u8] = b"abABc";
const SUBJ16: &[u16] = &[0x61, 0x62, 0x61, 0x62, 0x63];

fn params(current: i32, current_char: u32) -> RawMatchParams {
  RawMatchParams {
    total_register_count: 6,
    current,
    current_char,
    step_limit: Some(1),
    ..Default::default()
  }
}

fn m8<'a>(
  code: &'a [u8],
  out: &'a mut [i32],
  p: RawMatchParams,
  subj: &'static [u8],
) -> RawMatch<'a, &'static [u8], TraceHooks> {
  RawMatch::new(code, subj, out, p, TraceHooks::default())
}

fn m16<'a>(
  code: &'a [u8],
  out: &'a mut [i32],
  p: RawMatchParams,
) -> RawMatch<'a, &'static [u16], TraceHooks> {
  RawMatch::new(code, SUBJ16, out, p, TraceHooks::default())
}

/// ADVANCE 臂期望 pc = 指令定宽 (cc:368 Bytecodes::Size)
fn nxt(bc: Bytecode) -> u32 {
  u32::from(bytecode_size(bc))
}

/// 回跳对操作数 (start_reg, on_not_equal) 共 6 项 handler 同形
fn ref_ops(start_reg: u16) -> Vec<Vec<u8>> {
  vec![w2(start_reg), w4(TGT_JUMP)]
}

/// 单条反向引用指令码流 + 寄存器播种 (from/len 对), 返回 (step, pc, current, current_char)
fn run_backref(
  bc: Bytecode,
  from: i32,
  len: i32,
  current: i32,
  subj: &'static [u8],
) -> (Step, u32, i32, u32) {
  let mut asm = Asm::default();
  asm.insn(bc, &ref_ops(START_REG as u16));
  let code = asm.code().to_vec();
  let mut out = [0i32; 4];
  let mut m = m8(&code, &mut out, params(current, 0), subj);
  // registers[start_reg]/[+1] (原版 cc:881-882 读取面)
  m.registers.set(START_REG, from);
  m.registers.set(START_REG + 1, from + len);
  let step = m.run();
  (step, m.pc, m.current, m.current_char)
}

// ── P-3c 反向引用对 6 项 ────────────────────────────────────────────────────

/// T1 CheckNotBackRef cc:880 — 命中 ADVANCE_CURRENT_POSITION(len) / 失配与越界 Jump / 未捕获守卫
#[test]
fn p3c_check_not_back_ref_cc880() -> Void {
  // from=0, len=2 ("ab"); current=2 → s[2..4]="ab" 相等 → cc:889 ADVANCE_CURRENT_POSITION(2)
  let (step, pc, current, _cc) = run_backref(Bytecode::CheckNotBackRef, 0, 2, 2, SUBJ8);
  assert_eq!(step, Step::StepLimitExceeded { steps: 1 });
  assert_eq!(pc, nxt(Bytecode::CheckNotBackRef), "ADVANCE() (cc:890)");
  assert_eq!(current, 4, "ADVANCE_CURRENT_POSITION(len=2) (cc:889)");

  // 失配: current=3 → s[3..5]="bc" ≠ "ab" → cc:886 SET_PC_FROM_OFFSET(on_not_equal)
  let (step, pc, current, _) = run_backref(Bytecode::CheckNotBackRef, 0, 2, 3, SUBJ8);
  assert_eq!(step, Step::StepLimitExceeded { steps: 1 });
  assert_eq!(pc, TGT_JUMP, "Jump(on_not_equal) (cc:886)");
  assert_eq!(current, 3, "失配不改位置");

  // 越界: current=4, len=2 → 4+2>5 → 短路于比较核之前 (cc:884 `||` 左臂)
  let (_, pc, current, _) = run_backref(Bytecode::CheckNotBackRef, 0, 2, 4, SUBJ8);
  assert_eq!(pc, TGT_JUMP, "current+len>length 短路 (cc:884)");
  assert_eq!(current, 4);

  // 未捕获回跳: from=-1 (kNoMatchValue) → cc:883 守卫整块跳过 → 仅 ADVANCE
  let (_, pc, current, _) = run_backref(Bytecode::CheckNotBackRef, -1, 0, 0, SUBJ8);
  assert_eq!(pc, nxt(Bytecode::CheckNotBackRef), "from>=0 守卫 (cc:883)");
  assert_eq!(current, 0, "守卫臂不改位置");

  // len<=0 (空捕获) 同守卫臂
  let (_, pc, _) = {
    let mut asm = Asm::default();
    asm.insn(Bytecode::CheckNotBackRef, &ref_ops(START_REG as u16));
    let code = asm.code().to_vec();
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, params(0, 0), SUBJ8);
    m.registers.set(START_REG, 0);
    m.registers.set(START_REG + 1, 0); // len = 0
    (m.run(), m.pc, m.current)
  };
  assert_eq!(pc, nxt(Bytecode::CheckNotBackRef), "len>0 守卫 (cc:883)");
  info!("P3c-T1 ok");
  OK
}

/// T2 CheckNotBackRefBackward cc:894 — 比较窗口在 current-len, 命中回拨 SET_CURRENT_POSITION
#[test]
fn p3c_check_not_back_ref_backward_cc894() -> Void {
  // from=0 ("ab"), len=2, current=4 → 窗口 current-len=2 = "ab" 相等 → cc:902 回拨至 2
  let (step, pc, current, _) = run_backref(Bytecode::CheckNotBackRefBackward, 0, 2, 4, SUBJ8);
  assert_eq!(step, Step::StepLimitExceeded { steps: 1 });
  assert_eq!(
    pc,
    nxt(Bytecode::CheckNotBackRefBackward),
    "ADVANCE() (cc:904)"
  );
  assert_eq!(current, 2, "SET_CURRENT_POSITION(current-len) (cc:902)");

  // 失配: current=3 → 窗口 1 = "ba" ≠ "ab" → Jump
  let (_, pc, current, _) = run_backref(Bytecode::CheckNotBackRefBackward, 0, 2, 3, SUBJ8);
  assert_eq!(pc, TGT_JUMP, "Jump(on_not_equal) (cc:900)");
  assert_eq!(current, 3);

  // 负窗口: current=1, len=2 → current-len=-1 < 0 短路 (cc:898 左臂)
  let (_, pc, current, _) = run_backref(Bytecode::CheckNotBackRefBackward, 0, 2, 1, SUBJ8);
  assert_eq!(pc, TGT_JUMP, "current-len<0 短路 (cc:898)");
  assert_eq!(current, 1, "短路不回拨");
  info!("P3c-T2 ok");
  OK
}

/// T3 CheckNotBackRefNoCaseUnicode cc:908 — 一字节态走 Latin1 核 (unicode 标志无差别, cc:65)
#[test]
fn p3c_check_not_back_ref_no_case_unicode_cc908() -> Void {
  // from=0 ("ab"), current=2 ("AB"): case-sensitive 不等, 折小写等 → 命中
  let (step, pc, current, _) =
    run_backref(Bytecode::CheckNotBackRefNoCaseUnicode, 0, 2, 2, SUBJ8CI);
  assert_eq!(step, Step::StepLimitExceeded { steps: 1 });
  assert_eq!(
    pc,
    nxt(Bytecode::CheckNotBackRefNoCaseUnicode),
    "ADVANCE() (cc:918)"
  );
  assert_eq!(current, 4, "ADVANCE_CURRENT_POSITION(len) (cc:916)");

  // 真失配: current=3 → "BC" 折后 ≠ "ab" → Jump
  let (_, pc, ..) = run_backref(Bytecode::CheckNotBackRefNoCaseUnicode, 0, 2, 3, SUBJ8CI);
  assert_eq!(pc, TGT_JUMP, "Jump(on_not_equal) (cc:914)");

  // 越界先短路, 不打比较核 (cc:912 左臂)
  let (_, pc, current, _) = run_backref(Bytecode::CheckNotBackRefNoCaseUnicode, 0, 2, 4, SUBJ8CI);
  assert_eq!(pc, TGT_JUMP, "current+len>length 短路 (cc:912)");
  assert_eq!(current, 4);
  info!("P3c-T3 ok");
  OK
}

/// T4 CheckNotBackRefNoCase cc:922 — 非 unicode 前向; 一字节态与 T3 同核
#[test]
fn p3c_check_not_back_ref_no_case_cc922() -> Void {
  let (step, pc, current, _) = run_backref(Bytecode::CheckNotBackRefNoCase, 0, 2, 2, SUBJ8CI);
  assert_eq!(step, Step::StepLimitExceeded { steps: 1 });
  assert_eq!(
    pc,
    nxt(Bytecode::CheckNotBackRefNoCase),
    "ADVANCE() (cc:932)"
  );
  assert_eq!(current, 4, "ADVANCE_CURRENT_POSITION(len) (cc:930)");

  let (_, pc, ..) = run_backref(Bytecode::CheckNotBackRefNoCase, 0, 2, 3, SUBJ8CI);
  assert_eq!(pc, TGT_JUMP, "Jump(on_not_equal) (cc:928)");
  info!("P3c-T4 ok");
  OK
}

/// T5 CheckNotBackRefNoCaseUnicodeBackward cc:937 / T6 CheckNotBackRefNoCaseBackward cc:952
#[test]
fn p3c_no_case_backward_cc937_952() -> Void {
  for (bc, cc) in [
    (
      Bytecode::CheckNotBackRefNoCaseUnicodeBackward,
      "937: cc:947 ADVANCE / cc:945 回拨",
    ),
    (
      Bytecode::CheckNotBackRefNoCaseBackward,
      "952: cc:962 ADVANCE / cc:960 回拨",
    ),
  ] {
    // 命中: from=0 ("ab"), current=4 → 窗口 2 = "AB" 折等 → SET_CURRENT_POSITION(2)
    let (step, pc, current, _) = run_backref(bc, 0, 2, 4, SUBJ8CI);
    assert_eq!(step, Step::StepLimitExceeded { steps: 1 }, "{bc:?} ({cc})");
    assert_eq!(pc, nxt(bc), "{bc:?} ADVANCE() ({cc})");
    assert_eq!(current, 2, "{bc:?} 回拨 current-len ({cc})");

    // 失配: current=3 → 窗口 1 = "bA" ≠ "ab" → Jump
    let (_, pc, ..) = run_backref(bc, 0, 2, 3, SUBJ8CI);
    assert_eq!(pc, TGT_JUMP, "{bc:?} Jump(on_not_equal)");

    // 负窗口短路 (cc:941/956 左臂), 不打比较核
    let (_, pc, current, _) = run_backref(bc, 0, 2, 1, SUBJ8CI);
    assert_eq!(pc, TGT_JUMP, "{bc:?} current-len<0 短路");
    assert_eq!(current, 1, "{bc:?} 短路不回拨");
  }
  info!("P3c-T5/T6 ok");
  OK
}

// ── P-3c 锚点/末尾定位 3 项 ─────────────────────────────────────────────────

/// T7 CheckAtStart cc:967 / CheckNotAtStart cc:975 — 同式互补两出口
#[test]
fn p3c_check_at_start_pair_cc967_975() -> Void {
  for bc in [Bytecode::CheckAtStart, Bytecode::CheckNotAtStart] {
    let mut asm = Asm::default();
    asm.insn(bc, &[w2i(0), w4(TGT_JUMP)]); // cp_offset=0, on_*=TGT_JUMP
    let code = asm.code().to_vec();

    // current+0 == 0: CheckAtStart → Jump(on_at_start); CheckNotAtStart → ADVANCE
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, params(0, 0), SUBJ8);
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    let at_start_pc = m.pc;
    assert_eq!(
      at_start_pc,
      if bc == Bytecode::CheckAtStart {
        TGT_JUMP
      } else {
        nxt(bc)
      },
      "{bc:?} current=0 出口 (cc:968/976)"
    );

    // current=1 → 式不成立: CheckAtStart → ADVANCE; CheckNotAtStart → Jump
    let mut out2 = [0i32; 4];
    let mut m2 = m8(&code, &mut out2, params(1, 0), SUBJ8);
    assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(
      m2.pc,
      if bc == Bytecode::CheckAtStart {
        nxt(bc)
      } else {
        TGT_JUMP
      },
      "{bc:?} current=1 出口 (cc:970/978)"
    );

    // cp_offset=-1 使 current=1 重归 0 → 回到 current=0 的出口 (原版 cp_offset 面)
    let mut asm3 = Asm::default();
    asm3.insn(bc, &[w2i(-1), w4(TGT_JUMP)]);
    let code3 = asm3.code().to_vec();
    let mut out3 = [0i32; 4];
    let mut m3 = m8(&code3, &mut out3, params(1, 0), SUBJ8);
    assert_eq!(m3.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m3.pc, at_start_pc, "{bc:?} cp_offset=-1 使式为 0");
  }
  info!("P3c-T7 ok");
  OK
}

/// T8 SetCurrentPositionFromEnd cc:983 — 先 ADVANCE 再界判, 命中回拨至 len-by 并取前字符
#[test]
fn p3c_set_current_position_from_end_cc983() -> Void {
  let bc = Bytecode::SetCurrentPositionFromEnd;
  // by=1: 5-0 > 1 → SET_CURRENT_POSITION(5-1=4); current_char = subject[3] = 'b'
  let mut asm = Asm::default();
  asm.insn(bc, &[w2i(1)]);
  let code = asm.code().to_vec();
  let mut out = [0i32; 4];
  let mut m = m8(&code, &mut out, params(0, 0), SUBJ8);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, nxt(bc), "ADVANCE() 先于界判 (cc:984)");
  assert_eq!(m.current, 4, "SET_CURRENT_POSITION(length-by) (cc:986)");
  assert_eq!(
    m.current_char,
    u32::from(b'b'),
    "current_char = subject[current-1] (cc:988)"
  );

  // by=5: 5-0 > 5 = false → 位置与字符均不变, 但 pc 仍 ADVANCE
  let mut asm2 = Asm::default();
  asm2.insn(bc, &[w2i(5)]);
  let code2 = asm2.code().to_vec();
  let mut out2 = [0i32; 4];
  let mut m2 = m8(&code2, &mut out2, params(0, 0), SUBJ8);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m2.pc, nxt(bc), "无命中亦 ADVANCE (cc:984)");
  assert_eq!(m2.current, 0, "len-current>by 不成立则不回拨 (cc:985)");
  assert_eq!(m2.current_char, 0, "无命中不装载 current_char");
  info!("P3c-T8 ok");
  OK
}

// ── P-3c-icu 运行态推迟通道 (唯一残留, 禁伪装) ─────────────────────────────

/// T9 四个 NoCase handler 的两字节态 → NotImplemented { cc_line: 46, batch: "P-3c-icu" };
/// 界检/负窗短路臂不得进入该通道 (原版 `||` 序, 证明通道只在比较核需求时触发)
#[test]
fn p3c_no_case_two_byte_icu_channel() -> Void {
  for (bc, from, len, current) in [
    (Bytecode::CheckNotBackRefNoCaseUnicode, 0, 2, 2),
    (Bytecode::CheckNotBackRefNoCase, 0, 2, 2),
    (Bytecode::CheckNotBackRefNoCaseUnicodeBackward, 0, 2, 4),
    (Bytecode::CheckNotBackRefNoCaseBackward, 0, 2, 4),
  ] {
    let mut asm = Asm::default();
    asm.insn(bc, &ref_ops(START_REG as u16));
    let code = asm.code().to_vec();
    let mut out = [0i32; 4];
    let mut m = m16(&code, &mut out, params(current, 0));
    m.registers.set(START_REG, from);
    m.registers.set(START_REG + 1, from + len);
    assert_eq!(
      m.run(),
      Step::NotImplemented {
        bytecode: bc,
        cc_line: 46,
        batch: "P-3c-icu",
      },
      "{bc:?} uc16 态 ICU 显式登记 (regexp-interpreter.cc:46-61)"
    );
  }

  // 短路臂: 前向越界 (current+len>len) 与后向负窗 (current-len<0) 不进 ICU 通道
  let fwd = Bytecode::CheckNotBackRefNoCase;
  let mut asm = Asm::default();
  asm.insn(fwd, &ref_ops(START_REG as u16));
  let code = asm.code().to_vec();
  let mut out = [0i32; 4];
  let mut m = m16(&code, &mut out, params(4, 0));
  m.registers.set(START_REG, 0);
  m.registers.set(START_REG + 1, 2);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 }, "越界短路");
  assert_eq!(m.pc, TGT_JUMP, "cc:926 左臂 Jump, 不打 ICU");

  let bwd = Bytecode::CheckNotBackRefNoCaseBackward;
  let mut asm2 = Asm::default();
  asm2.insn(bwd, &ref_ops(START_REG as u16));
  let code2 = asm2.code().to_vec();
  let mut out2 = [0i32; 4];
  let mut m2 = m16(&code2, &mut out2, params(1, 0));
  m2.registers.set(START_REG, 0);
  m2.registers.set(START_REG + 1, 2);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 }, "负窗短路");
  assert_eq!(m2.pc, TGT_JUMP, "cc:956 左臂 Jump, 不打 ICU");

  // 未捕获守卫臂 (from=-1) 同样不进通道
  let mut out3 = [0i32; 4];
  let mut m3 = m16(&code, &mut out3, params(0, 0));
  m3.registers.set(START_REG, -1);
  m3.registers.set(START_REG + 1, -1);
  assert_eq!(m3.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m3.pc, nxt(fwd), "from>=0 守卫 (cc:924)");
  info!("P3c-T9 ok");
  OK
}

// ── P-3d skip-until peephole 7 项 ──────────────────────────────────────────

/// T10 SkipUntilChar cc:1010 — 逐字符扫到命中; 界内耗尽则 on_no_match
#[test]
fn p3d_skip_until_char_cc1010() -> Void {
  let bc = Bytecode::SkipUntilChar;
  let build = |character: u16, bounds: i32| {
    let mut asm = Asm::default();
    asm.insn(
      bc,
      &[
        w2i(0),
        w2i(1),
        w2(character),
        w4(bounds as u32),
        w4(TGT_JUMP),
        w4(TGT_JUMP2),
      ],
    );
    asm.code().to_vec()
  };

  // 命中 'c'(99): 0'a'→1'b'→2'a'→3'b' 皆跳过, 4'c' 命中且不再 ADVANCE
  let code = build(u16::from(b'c'), 0);
  let mut out = [0i32; 4];
  let mut m = m8(&code, &mut out, params(0, 0), SUBJ8);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, TGT_JUMP, "SET_PC_FROM_OFFSET(on_match) (cc:1015)");
  assert_eq!(m.current, 4, "命中位不自增 (cc:1014)");
  assert_eq!(m.current_char, u32::from(b'c'), "cc:1013 装载");

  // 界外耗尽: 'z' 永不相等 → current 走到 len 后 on_no_match
  let code2 = build(u16::from(b'z'), 0);
  let mut out2 = [0i32; 4];
  let mut m2 = m8(&code2, &mut out2, params(0, 0), SUBJ8);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(
    m2.pc, TGT_JUMP2,
    "SET_PC_FROM_OFFSET(on_no_match) (cc:1019)"
  );
  assert_eq!(m2.current, 5, "循环内 ADVANCE_CURRENT_POSITION(1) 至界外");
  assert_eq!(m2.current_char, u32::from(b'c'), "末次装载 subject[4]");

  // bounds_check_offset=3: 界位恒为 current+3, 于 current=2 出界 (原版 IndexIsInBounds cc:1012)
  // —— 命中位 4 永不可达, 与 bounds=0 一臂形成对照 (界式而非字符决定出口)
  let code3 = build(u16::from(b'c'), 3);
  let mut out3 = [0i32; 4];
  let mut m3 = m8(&code3, &mut out3, params(0, 0), SUBJ8);
  assert_eq!(m3.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(
    m3.pc, TGT_JUMP2,
    "0+3/1+3 界内, 2+3>=5 出界 → on_no_match (cc:1012)"
  );
  assert_eq!(m3.current, 2, "两窗后出界");
  assert_eq!(m3.current_char, u32::from(b'b'), "末次装载 subject[1]");

  // bounds=4 起点 1: 首判即 1+4>=5 出界 → 循环体零次执行, 不装载
  let code4 = build(u16::from(b'c'), 4);
  let mut out4 = [0i32; 4];
  let mut m4 = m8(&code4, &mut out4, params(1, 0), SUBJ8);
  assert_eq!(m4.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m4.pc, TGT_JUMP2, "首判即出界 (cc:1012)");
  assert_eq!(m4.current, 1, "出界臂不装载不自增");
  assert_eq!(m4.current_char, 0, "循环体未执行, current_char 保持入参");
  info!("P3d-T10 ok");
  OK
}

/// T11 SkipUntilCharAnd cc:1023 — 命中条件 (current_char & mask) == character
#[test]
fn p3d_skip_until_char_and_cc1023() -> Void {
  let bc = Bytecode::SkipUntilCharAnd;
  let build = |character: u16, mask: u32, start: i32| {
    let mut asm = Asm::default();
    asm.insn(
      bc,
      &[
        w2i(0),
        w2i(1),
        w2(character),
        w4(mask),
        w4(0),
        w4(TGT_JUMP),
        w4(TGT_JUMP2),
      ],
    );
    (asm.code().to_vec(), start)
  };

  // mask=0x7E: 'a'(0x61)&0x7E = 0x60 → character=0x60 命中; 'b'(0x62)&0x7E=0x62 不命中
  let (code, start) = build(0x60, 0x7E, 0);
  let mut out = [0i32; 4];
  let mut m = m8(&code, &mut out, params(start, 0), SUBJ8);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, TGT_JUMP, "首位 (c&mask)==character 命中 (cc:1027)");
  assert_eq!(m.current, 0);
  assert_eq!(m.current_char, u32::from(b'a'));

  // 从 current=1 起: 'b' 不命中 → 前进 → 2 'a' 命中
  let (code2, start2) = build(0x60, 0x7E, 1);
  let mut out2 = [0i32; 4];
  let mut m2 = m8(&code2, &mut out2, params(start2, 0), SUBJ8);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m2.pc, TGT_JUMP);
  assert_eq!(
    m2.current, 2,
    "掩码不命中则 ADVANCE_CURRENT_POSITION(1) (cc:1030)"
  );

  // 掩码使全串不命中 → on_no_match, current 至界外
  let (code3, start3) = build(0x70, 0x7E, 0); // 需 'p'/'q' 类, SUBJ8 无
  let mut out3 = [0i32; 4];
  let mut m3 = m8(&code3, &mut out3, params(start3, 0), SUBJ8);
  assert_eq!(m3.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m3.pc, TGT_JUMP2, "cc:1032 on_no_match");
  assert_eq!(m3.current, 5);
  info!("P3d-T11 ok");
  OK
}

/// T12 SkipUntilBitInTable cc:1036 — 命中条件为位图表查 (CheckBitInTable 纯函数)
#[test]
fn p3d_skip_until_bit_in_table_cc1036() -> Void {
  let bc = Bytecode::SkipUntilBitInTable;
  let build = |chars: &[u8]| {
    let mut asm = Asm::default();
    asm.insn(
      bc,
      &[
        w2i(0),
        w2i(1),
        table(chars),
        w4(0),
        w4(TGT_JUMP),
        w4(TGT_JUMP2),
      ],
    );
    asm.code().to_vec()
  };

  // 表含 'b': 0'a' 未中 → 1'b' 中
  let code = build(b"b");
  let mut out = [0i32; 4];
  let mut m = m8(&code, &mut out, params(0, 0), SUBJ8);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, TGT_JUMP, "表命中 → on_match (cc:1041)");
  assert_eq!(m.current, 1);
  assert_eq!(m.current_char, u32::from(b'b'));

  // 空表 → 全扫出界 → on_no_match
  let code2 = build(b"");
  let mut out2 = [0i32; 4];
  let mut m2 = m8(&code2, &mut out2, params(0, 0), SUBJ8);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m2.pc, TGT_JUMP2, "cc:1045 on_no_match");
  assert_eq!(m2.current, 5);

  // 表含 'c' (末位): 扫至 4 命中 — 位窗口 (c&127)>>3 / c&7 与纯函数同式
  let code3 = build(b"c");
  let mut out3 = [0i32; 4];
  let mut m3 = m8(&code3, &mut out3, params(0, 0), SUBJ8);
  assert_eq!(m3.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m3.pc, TGT_JUMP);
  assert_eq!(m3.current, 4);
  info!("P3d-T13 ok"); // 注: 序号与 fn 名 T12 对应, 日志沿本档序
  OK
}

/// T13 SkipUntilGtOrNotBitInTable cc:1049 — 两 if 分立 (GT 先, 表查后), 原版注释禁合并
#[test]
fn p3d_skip_until_gt_or_not_bit_in_table_cc1049() -> Void {
  let bc = Bytecode::SkipUntilGtOrNotBitInTable;
  let build = |character: u16, chars: &[u8]| {
    let mut asm = Asm::default();
    asm.insn(
      bc,
      &[
        w2i(0),
        w2i(1),
        w2(character),
        table(chars),
        w4(0),
        w4(TGT_JUMP),
        w4(TGT_JUMP2),
      ],
    );
    asm.code().to_vec()
  };

  // GT 臂: character='`'(0x60) < 'a'(0x61) → 首位即 on_match (cc:1052-1054)
  let code = build(u16::from(b'`'), b"");
  let mut out = [0i32; 4];
  let mut m = m8(&code, &mut out, params(0, 0), SUBJ8);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(
    m.pc, TGT_JUMP,
    "current_char > character → on_match (cc:1054)"
  );
  assert_eq!(m.current, 0);
  assert_eq!(m.current_char, u32::from(b'a'));

  // 表查臂: GT 不成立 (character=200), 表仅含 'a' → 0 命中表则前进, 1 'b' 不在表 → on_match
  let code2 = build(200, b"a");
  let mut out2 = [0i32; 4];
  let mut m2 = m8(&code2, &mut out2, params(0, 0), SUBJ8);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(
    m2.pc, TGT_JUMP,
    "!CheckBitInTable → on_match (cc:1057-1058)"
  );
  assert_eq!(m2.current, 1);
  assert_eq!(m2.current_char, u32::from(b'b'));

  // 两臂皆不成立 (GT 界外 + 全集入表) → on_no_match
  let code3 = build(200, b"abc");
  let mut out3 = [0i32; 4];
  let mut m3 = m8(&code3, &mut out3, params(0, 0), SUBJ8);
  assert_eq!(m3.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m3.pc, TGT_JUMP2, "cc:1063 on_no_match");
  assert_eq!(m3.current, 5);
  info!("P3d-T14 ok");
  OK
}

/// T14 SkipUntilCharOrChar cc:1066 — char1/char2 双 if 分立 (原版注释明示勿合并)
#[test]
fn p3d_skip_until_char_or_char_cc1066() -> Void {
  let bc = Bytecode::SkipUntilCharOrChar;
  let build = |c1: u16, c2: u16| {
    let mut asm = Asm::default();
    asm.insn(
      bc,
      &[
        w2i(0),
        w2i(1),
        w2(c1),
        w2(c2),
        w4(0),
        w4(TGT_JUMP),
        w4(TGT_JUMP2),
      ],
    );
    asm.code().to_vec()
  };

  // char1 命中末位 'c'
  let code = build(u16::from(b'c'), 200);
  let mut out = [0i32; 4];
  let mut m = m8(&code, &mut out, params(0, 0), SUBJ8);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, TGT_JUMP, "char1 相等 → on_match (cc:1072-1074)");
  assert_eq!(m.current, 4);

  // char2 命中首位 'a' (char1 永不中) — 独立 if 而非复合条件
  let code2 = build(200, u16::from(b'a'));
  let mut out2 = [0i32; 4];
  let mut m2 = m8(&code2, &mut out2, params(0, 0), SUBJ8);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m2.pc, TGT_JUMP, "char2 相等 → on_match (cc:1077-1079)");
  assert_eq!(m2.current, 0);

  // 双不中 → on_no_match
  let code3 = build(200, 201);
  let mut out3 = [0i32; 4];
  let mut m3 = m8(&code3, &mut out3, params(0, 0), SUBJ8);
  assert_eq!(m3.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m3.pc, TGT_JUMP2, "cc:1083 on_no_match");
  assert_eq!(m3.current, 5);
  info!("P3d-T15 ok");
  OK
}

/// T15 SkipUntilOneOfMasked cc:1086 — 4 字符窗口预筛 + 两路分查; 仅一字节态 (cc:1094)
#[test]
fn p3d_skip_until_one_of_masked_cc1086() -> Void {
  let bc = Bytecode::SkipUntilOneOfMasked;
  // subject "ababc" 窗口: 0..3 = a,b,a,b → 0x62616261; 1..4 = b,a,b,c → 0x63626162
  const W0: u32 = 0x6261_6261;
  let build =
    |both_chars: u32, both_mask: u32, chars1: u32, mask1: u32, chars2: u32, mask2: u32| {
      let mut asm = Asm::default();
      asm.insn(
        bc,
        &[
          w2i(0),
          w2i(1),
          w4(both_chars),
          w4(both_mask),
          w4(3), // max_offset (窗口界: current+3 < len)
          w4(chars1),
          w4(mask1),
          w4(chars2),
          w4(mask2),
          w4(TGT_JUMP),
          w4(TGT_JUMP2),
          w4(TGT_JUMP3),
        ],
      );
      asm.code().to_vec()
    };

  // on_match1: 预筛 (低字节 'a') 过, 第一路 (高两字节 0x6261) 命中
  let code = build(0x61, 0xFF, W0 & 0xFFFF_0000, 0xFFFF_0000, 0, 0);
  let mut out = [0i32; 4];
  let mut m = m8(&code, &mut out, params(0, 0), SUBJ8);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, TGT_JUMP, "on_match1 (cc:1098)");
  assert_eq!(m.current_char, W0, "Load4Characters(subject, 0) (cc:1097)");
  assert_eq!(m.current, 0);

  // on_match2: 第一路不中, 第二路 (低字节 'a') 中
  let code2 = build(0x61, 0xFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0x61, 0xFF);
  let mut out2 = [0i32; 4];
  let mut m2 = m8(&code2, &mut out2, params(0, 0), SUBJ8);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m2.pc, TGT_JUMP2, "on_match2 (cc:1103)");

  // 预筛不通过 → 两路皆不打, 逐窗前进至界外 on_failure
  let code3 = build(0xDEAD, 0xFFFF, 0, 0, 0, 0xFF);
  let mut out3 = [0i32; 4];
  let mut m3 = m8(&code3, &mut out3, params(0, 0), SUBJ8);
  assert_eq!(m3.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m3.pc, TGT_JUMP3, "on_failure (cc:1108)");
  assert_eq!(m3.current, 2, "current+max_offset(3) < 5 的两窗后出界");
  info!("P3d-T16 ok");
  OK
}

/// T16 SkipUntilOneOfMasked3 cc:1111 — bc0..bc7 复合体五臂逐出
#[test]
fn p3d_skip_until_one_of_masked3_cc1111() -> Void {
  let bc = Bytecode::SkipUntilOneOfMasked3;
  const W0: u32 = 0x6261_6261; // subject[0..4] = a,b,a,b
  let build = |bc0_table: &[u8],
               bc1_bounds: i32,
               bc1_on_failure: u32,
               bc2_characters: u32,
               bc2_mask: u32,
               bc3_by: i16,
               bc4_bounds: i32,
               bc5_characters: u32,
               bc5_mask: u32,
               bc6_characters: u32,
               bc6_mask: u32,
               bc7_characters: u32,
               bc7_mask: u32,
               fallthrough: u32| {
    let mut asm = Asm::default();
    asm.insn(
      bc,
      &[
        w2i(0), // bc0_cp_offset
        w2i(1), // bc0_advance_by
        table(bc0_table),
        w4(bc1_bounds as u32), // bc1_bounds_check_offset
        w4(bc1_on_failure),
        w2i(0), // bc1_cp_offset
        w4(bc2_characters),
        w4(bc2_mask),
        w2i(bc3_by), // bc3_by
        w4(bc4_bounds as u32),
        w2i(0), // bc4_cp_offset
        w4(bc5_characters),
        w4(bc5_mask),
        w4(TGT_JUMP), // bc5_on_equal
        w4(bc6_characters),
        w4(bc6_mask),
        w4(TGT_JUMP2), // bc6_on_equal
        w4(bc7_characters),
        w4(bc7_mask),
        w4(fallthrough),
      ],
    );
    asm.code().to_vec()
  };
  let run = |code: &[u8], current: i32| {
    let mut out = [0i32; 4];
    let mut m = m8(code, &mut out, params(current, 0), SUBJ8);
    let step = m.run();
    (step, m.pc, m.current, m.current_char)
  };

  // 臂 a: bc0 空表扫到界外 → bc1 界检失败 → bc1_on_failure (cc:1121-1134)
  let ca = build(b"", 0, TGT_JUMP3, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, TGT_JUMP);
  let (step, pc, current, _) = run(&ca, 0);
  assert_eq!(step, Step::StepLimitExceeded { steps: 1 });
  assert_eq!(pc, TGT_JUMP3, "bc1_on_failure (cc:1132)");
  assert_eq!(current, 5, "bc0 扫描 ADVANCE_CURRENT_POSITION(1) 至界外");

  // 臂 b: bc0 表含 'a' 首位命中 → bc2 中 → bc5 中 → bc5_on_equal (cc:1124/1141/1158-1159)
  let cb = build(
    b"a",
    3,
    TGT_JUMP3,
    W0 & 0xFF,
    0xFF,
    1,
    3,
    W0 & 0xFFFF_0000,
    0xFFFF_0000,
    0,
    0,
    0,
    0,
    TGT_JUMP,
  );
  let (_, pc, current, cc) = run(&cb, 0);
  assert_eq!(pc, TGT_JUMP, "bc5_on_equal (cc:1159)");
  assert_eq!(current, 0, "bc0 首位命中即 break");
  assert_eq!(cc, W0, "bc4 再 Load4 (cc:1154-1155)");

  // 臂 c: bc5 不中, bc6 中 → bc6_on_equal (cc:1162-1163)
  let cc_code = build(
    b"a",
    3,
    TGT_JUMP3,
    W0 & 0xFF,
    0xFF,
    1,
    3,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    W0 & 0xFF,
    0xFF,
    0,
    0,
    TGT_JUMP,
  );
  let (_, pc, ..) = run(&cc_code, 0);
  assert_eq!(pc, TGT_JUMP2, "bc6_on_equal (cc:1145-1146)");

  // 臂 d: bc5/bc6 不中, bc7 中 → fallthrough_jump_target (cc:1166-1168)
  let cd = build(
    b"a",
    3,
    TGT_JUMP3,
    W0 & 0xFF,
    0xFF,
    1,
    3,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    W0 & 0xFF,
    0xFF,
    TGT_JUMP3,
  );
  let (_, pc, ..) = run(&cd, 0);
  assert_eq!(
    pc, TGT_JUMP3,
    "bc7 (AndCheckNot4Chars) → fallthrough (cc:1167)"
  );

  // 臂 e: bc2 不中 → bc3 ADVANCE_CURRENT_POSITION(1) → continue 回边 →
  //       bc0 从 1 扫到 2 命中 'a' → bc1 界检 (2+3>=5) 失败 → bc1_on_failure
  let ce = build(
    b"a",
    3,
    TGT_JUMP3,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    1,
    3,
    0,
    0,
    0,
    0,
    0,
    0,
    TGT_JUMP,
  );
  let (step, pc, current, _) = run(&ce, 0);
  assert_eq!(step, Step::StepLimitExceeded { steps: 1 });
  assert_eq!(
    pc, TGT_JUMP3,
    "bc3 continue 回边后由 bc1 出界收口 (cc:1144-1148)"
  );
  assert_eq!(current, 2, "bc3_by=1 + bc0 扫描至下一个 'a'");
  info!("P3d-T17 ok");
  OK
}

// ── 批段台账自校验 (禁静默吞账: 名单/行锚/计数与原版全集对数) ─────────────

/// T18 61 handler 批段对数: P-3a 23 + P-3b 22 + P-3c 9 + P-3d 7 = 全集;
/// P-3c/P-3d 16 项逐项行锚与本档测试函数一一对应, 原 "未落地分支" 台账已退役
#[test]
fn p3c_p3d_landed_ledger() -> Void {
  const P3A: usize = 23;
  const P3B: usize = 22;
  const LANDED: [(Bytecode, u32, &str); 16] = [
    (Bytecode::CheckNotBackRef, 880, "exec_backref_ops"),
    (Bytecode::CheckNotBackRefBackward, 894, "exec_backref_ops"),
    (
      Bytecode::CheckNotBackRefNoCaseUnicode,
      908,
      "exec_backref_ops",
    ),
    (Bytecode::CheckNotBackRefNoCase, 922, "exec_backref_ops"),
    (
      Bytecode::CheckNotBackRefNoCaseUnicodeBackward,
      937,
      "exec_backref_ops",
    ),
    (
      Bytecode::CheckNotBackRefNoCaseBackward,
      952,
      "exec_backref_ops",
    ),
    (Bytecode::CheckAtStart, 967, "exec_backref_ops"),
    (Bytecode::CheckNotAtStart, 975, "exec_backref_ops"),
    (Bytecode::SetCurrentPositionFromEnd, 983, "exec_backref_ops"),
    (Bytecode::SkipUntilChar, 1010, "exec_skip_ops"),
    (Bytecode::SkipUntilCharAnd, 1023, "exec_skip_ops"),
    (Bytecode::SkipUntilBitInTable, 1036, "exec_skip_ops"),
    (Bytecode::SkipUntilGtOrNotBitInTable, 1049, "exec_skip_ops"),
    (Bytecode::SkipUntilCharOrChar, 1066, "exec_skip_ops"),
    (Bytecode::SkipUntilOneOfMasked, 1086, "exec_skip_ops"),
    (Bytecode::SkipUntilOneOfMasked3, 1111, "exec_skip_ops"),
  ];
  let p3c = LANDED
    .iter()
    .filter(|(_, _, group)| *group == "exec_backref_ops")
    .count();
  let p3d = LANDED
    .iter()
    .filter(|(_, _, group)| *group == "exec_skip_ops")
    .count();
  assert_eq!((p3c, p3d), (9, 7), "批段切分 P-3c=9 P-3d=7");
  assert_eq!(
    P3A + P3B + p3c + p3d,
    usize::from(K_BYTECODE_COUNT),
    "P-3a 23 + P-3b 22 + P-3c 9 + P-3d 7 = 61 全集, 无缺项"
  );
  // cc 行锚严格递增 = 与原版 BYTECODE 宏源序同构 (禁重号/漏号)
  for w in LANDED.windows(2) {
    assert!(
      w[0].1 < w[1].1,
      "行锚须随 cc 源序递增: {:?} ({}) → {:?} ({})",
      w[0].0,
      w[0].1,
      w[1].0,
      w[1].1
    );
  }
  // 逐项操作数布局已注册 (宽度非零 + 个数 ≥1), 与骨架票 T11/T12 同源对数
  for (bc, cc, _) in LANDED.iter() {
    assert!(operand_count(*bc) >= 1, "#{cc} {bc:?} 操作数未注册");
    for i in 0..operand_count(*bc) {
      assert!(operand_size(*bc, i) >= 1, "#{cc} {bc:?} op{i} 宽度");
    }
    assert_eq!(
      usize::from(bytecode_size(*bc)) % 4,
      0,
      "#{cc} {bc:?} 非 kBytecodeAlignment=4 倍数"
    );
  }
  info!("P3c/P3d-T18 台账 ok: 16 项落地 + 61 对数全过");
  OK
}
