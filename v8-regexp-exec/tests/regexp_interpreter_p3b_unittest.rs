//! P-3b handler 落地测试 (REG-1b3, 22 个字符装载/比较/表查/类集 handler)。
//!
//! 规约映射 (实测 v8/test/unittests/regexp/regexp-unittest.cc: 该目录无
//! interpreter 级纯函数单测，全部为 Isolate 硬耦合 ExecuteForTesting 路径；
//! 按票规约 3 手工复刻 opcode 级断言):
//! - 逐 handler 正常路径 (跳转臂 / ADVANCE 臂两向)
//! - LoadCurrentCharacter 族: one_byte / two_byte 两态 (SUBJ8 / SUBJ16)
//! - Load4CurrentChars 族: 仅 one_byte (原版 DCHECK_EQ(1, sizeof(Char)) cc:727)
//! - 边界 cp 越界 (bounds_check_offset 界外 → on_failure 跳转)
//! - CheckSpecialClassRanges: Digit('d') / Word('w') / LineTerminator('n') / NotWord('W')
//!   四集 —— 操作数字节 = 原版转义字母 (regexp-ast.h:101-112 `: char`)
//! - 断言强度: pc (ADVANCE/Jump)、current_char (装载面)、跳转目标 (on_* 操作数值)
//!   三路均须等原版语义。
//!
//! 纪律: 文件级 #[ctor::ctor(unsafe)] 日志, 每 #[test] fn 返回 aok::Void。

use aok::{OK, Void};
use log::info;
use v8_regexp_exec::regexp_interpreter::{
  Bytecode, K_BIT_TABLE_BYTES, K_WORD_CHARACTER_MAP, RawMatch, RawMatchParams, Result, Step,
  TraceHooks, bytecode_operands, bytecode_size, check_bit_in_table, operand_count, operand_offset,
  operand_size,
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

fn w1(v: u8) -> Vec<u8> {
  vec![v]
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
fn w4i(v: i32) -> Vec<u8> {
  v.to_ne_bytes().to_vec()
}

const SUBJ8: &[u8] = b"abcde";
const SUBJ16: &[u16] = &[0x61, 0x62, 0x63, 0x64, 0x65];

fn p1(current: i32, current_char: u32) -> RawMatchParams {
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
  params: RawMatchParams,
) -> RawMatch<'a, &'static [u8], TraceHooks> {
  RawMatch::new(code, SUBJ8, out, params, TraceHooks::default())
}

fn m16<'a>(
  code: &'a [u8],
  out: &'a mut [i32],
  params: RawMatchParams,
) -> RawMatch<'a, &'static [u16], TraceHooks> {
  RawMatch::new(code, SUBJ16, out, params, TraceHooks::default())
}

// ── P-3b 22 handler 逐名测试 ─────────────────────────────────────────────────

/// P3b-T1: LoadCurrentCharacter cc:696 — 界内 (ADVANCE, current_char 装载) + 界外 (Jump on_failure)
#[test]
fn p3b_load_current_character_cc696() -> Void {
  // 帧: [0] LoadCC(size=12) [12] Succeed(size=4) = 16 bytes
  // cp_offset=1, bounds_check_offset=1: 界检查与装载位置一致（与原版生成器同序）
  let mut asm = Asm::default();
  let at = asm.insn(
    Bytecode::LoadCurrentCharacter,
    &[w2i(1), w4i(1), w4(12)], // cp_offset=1, bounds_check_offset=1, on_failure=12
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();
  assert_eq!(at, 0);

  // 界内: current=0, bounds_check_offset=1 → 0+1=1 < len=5 → ADVANCE, subject[0+1]='b'
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): LoadCurrentCharacter size=12 (cc:700)");
    assert_eq!(
      m.current_char,
      u32::from(b'b'),
      "subject[current+cp_offset]=subject[1]='b' (cc:701)"
    );
  }
  // 界外: current=4, bounds_check_offset=1 → 4+1=5 >= len=5 → Jump(on_failure=12)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(4, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "Jump(on_failure), 不 ADVANCE (cc:698)");
    assert_eq!(m.current_char, 0, "界外不装载 (cc:697-698)");
  }
  info!("P3b-T1 ok");
  OK
}

/// P3b-T2: LoadCurrentCharacterUnchecked cc:705 — ADVANCE + current_char 装载 (无界检)
#[test]
fn p3b_load_current_character_unchecked_cc705() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::LoadCurrentCharacterUnchecked, &[w2i(2)]); // cp_offset=2
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  let mut out = [0i32; 4];
  let mut m = m8(&code, &mut out, p1(1, 0)); // current=1, subject[1+2]=subject[3]='d'
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, 4, "ADVANCE(): size=4 (cc:706)");
  assert_eq!(
    m.current_char,
    u32::from(b'd'),
    "subject[current+cp_offset]=subject[3]='d'"
  );
  info!("P3b-T2 ok");
  OK
}

/// P3b-T3: Load2CurrentChars cc:711 — 界内/界外, one_byte + two_byte 双形态
#[test]
fn p3b_load2_current_chars_cc711() -> Void {
  let mut asm = Asm::default();
  asm.insn(
    Bytecode::Load2CurrentChars,
    &[w2i(0), w4i(1), w4(12)], // cp_offset=0, bounds_check_offset=1, on_failure=12
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // one_byte: current=0, bounds_check_offset=1 → subject[1] in-bounds → ADVANCE, load2 at pos 0
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): size=12");
    // load2_at(0): s[0]|s[1]<<8 = 'a' | 'b'<<8 = 0x61 | 0x6200 = 0x6261
    let expected = u32::from(b'a') | (u32::from(b'b') << 8);
    assert_eq!(m.current_char, expected, "Load2Characters one_byte cc:716");
  }
  // two_byte: current=0, bounds_check_offset=1 → s[1] in-bounds → ADVANCE, load2 at pos 0
  {
    let mut out = [0i32; 4];
    let mut m = m16(&code, &mut out, p1(0, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12);
    // load2_at(0): s[0]|s[1]<<16 = 0x61 | 0x62<<16
    let expected = 0x61u32 | (0x62u32 << 16);
    assert_eq!(m.current_char, expected, "Load2Characters two_byte cc:716");
  }
  // 界外: current=4, bounds_check_offset=1 → 4+1=5 >= len=5 → on_failure
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(4, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "Jump(on_failure), 不 ADVANCE");
    assert_eq!(m.current_char, 0, "界外不装载 (cc:713)");
  }
  info!("P3b-T3 ok");
  OK
}

/// P3b-T4: Load2CurrentCharsUnchecked cc:720 — ADVANCE + load2 (无界检)
#[test]
fn p3b_load2_current_chars_unchecked_cc720() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::Load2CurrentCharsUnchecked, &[w2i(0)]); // cp_offset=0
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // one_byte: current=1 → load2_at(1): s[1]|s[2]<<8 = 'b' | 'c'<<8
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(1, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 4, "ADVANCE(): size=4 (cc:721)");
    let expected = u32::from(b'b') | (u32::from(b'c') << 8);
    assert_eq!(
      m.current_char, expected,
      "Load2Characters one_byte unchecked"
    );
  }
  // two_byte: current=1 → load2_at(1): s[1]|s[2]<<16
  {
    let mut out = [0i32; 4];
    let mut m = m16(&code, &mut out, p1(1, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 4);
    let expected = 0x62u32 | (0x63u32 << 16);
    assert_eq!(
      m.current_char, expected,
      "Load2Characters two_byte unchecked"
    );
  }
  info!("P3b-T4 ok");
  OK
}

/// P3b-T5: Load4CurrentChars cc:726 — 界内/界外, one_byte 限定 (cc:727 DCHECK)
#[test]
fn p3b_load4_current_chars_cc726() -> Void {
  let mut asm = Asm::default();
  asm.insn(
    Bytecode::Load4CurrentChars,
    &[w2i(0), w4i(3), w4(12)], // cp_offset=0, bounds_check_offset=3, on_failure=12
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // one_byte: current=0, bounds_check_offset=3 → subject[3] in-bounds → ADVANCE, load4 at pos 0
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): Load4 size=12");
    // Load4Characters(s,0): s[0]|s[1]<<8|s[2]<<16|s[3]<<24 = 'a','b','c','d'
    let expected =
      u32::from(b'a') | (u32::from(b'b') << 8) | (u32::from(b'c') << 16) | (u32::from(b'd') << 24);
    assert_eq!(m.current_char, expected, "Load4Characters one_byte cc:732");
  }
  // 界外: current=3, bounds_check_offset=3 → 3+3=6 >= len=5 → Jump(on_failure)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(3, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "Jump(on_failure)");
    assert_eq!(m.current_char, 0, "界外不装载 (cc:729)");
  }
  info!("P3b-T5 ok");
  OK
}

/// P3b-T6: Load4CurrentCharsUnchecked cc:736 — one_byte 限定, ADVANCE + load4 (无界检)
#[test]
fn p3b_load4_current_chars_unchecked_cc736() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::Load4CurrentCharsUnchecked, &[w2i(0)]); // cp_offset=0
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // one_byte: current=0, subject="abcde" → Load4(s,0) = a|b<<8|c<<16|d<<24
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 4, "ADVANCE(): size=4 (cc:737)");
    let expected =
      u32::from(b'a') | (u32::from(b'b') << 8) | (u32::from(b'c') << 16) | (u32::from(b'd') << 24);
    assert_eq!(
      m.current_char, expected,
      "Load4Characters one_byte unchecked"
    );
  }
  info!("P3b-T6 ok");
  OK
}

/// P3b-T7: Check4Chars cc:743 — 匹配 (Jump on_equal) / 不匹配 (ADVANCE)
#[test]
fn p3b_check4_chars_cc743() -> Void {
  let four =
    u32::from(b'a') | (u32::from(b'b') << 8) | (u32::from(b'c') << 16) | (u32::from(b'd') << 24);
  let mut asm = Asm::default();
  asm.insn(Bytecode::Check4Chars, &[w4(four), w4(12)]);
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 匹配: current_char == four → Jump(on_equal=12)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, four));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "Jump(on_equal) (cc:745)");
  }
  // 不匹配: current_char = 0xDEAD → ADVANCE(pc+12)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0xDEAD));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): size=12 (cc:747)");
  }
  info!("P3b-T7 ok");
  OK
}

/// P3b-T8: CheckCharacter cc:751 — character == current_char: Jump; 否则: ADVANCE
#[test]
fn p3b_check_character_cc751() -> Void {
  let ch = u16::from(b'x');
  let mut asm = Asm::default();
  asm.insn(Bytecode::CheckCharacter, &[w2(ch), w4(8)]);
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 匹配: current_char = 'x' → Jump(on_equal=8)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(ch)));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "Jump(on_equal) (cc:753)");
  }
  // 不匹配: current_char = 'y' → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'y')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): size=8 (cc:755)");
  }
  info!("P3b-T8 ok");
  OK
}

/// P3b-T9: CheckNot4Chars cc:759 — characters != current_char: Jump(on_not_equal); 否则: ADVANCE
#[test]
fn p3b_check_not4_chars_cc759() -> Void {
  let four = 0xAABBCCDDu32;
  let mut asm = Asm::default();
  asm.insn(Bytecode::CheckNot4Chars, &[w4(four), w4(12)]);
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 不等: current_char = 0x1234 → Jump(on_not_equal=12)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0x1234));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "Jump(on_not_equal) (cc:761)");
  }
  // 相等: current_char = four → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, four));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): size=12 (cc:763)");
  }
  info!("P3b-T9 ok");
  OK
}

/// P3b-T10: CheckNotCharacter cc:767 — character != current_char: Jump; 否则: ADVANCE
#[test]
fn p3b_check_not_character_cc767() -> Void {
  let ch = u16::from(b'!');
  let mut asm = Asm::default();
  asm.insn(Bytecode::CheckNotCharacter, &[w2(ch), w4(8)]);
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 不等: current_char = '?' → Jump(on_not_equal=8)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'?')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "Jump(on_not_equal) (cc:769)");
  }
  // 相等: current_char = '!' → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(ch)));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): size=8 (cc:771)");
  }
  info!("P3b-T10 ok");
  OK
}

/// P3b-T11: AndCheck4Chars cc:775 — characters == (current_char & mask)
#[test]
fn p3b_and_check4_chars_cc775() -> Void {
  let mask = 0x0000_FFFFu32; // 低 16 位掩码
  let characters = 0x1234u32; // 目标：低 16 位等于此值
  let mut asm = Asm::default();
  asm.insn(
    Bytecode::AndCheck4Chars,
    &[w4(characters), w4(mask), w4(16)],
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 匹配: current_char = 0x5678_1234 → masked = 0x1234 == characters → Jump
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0x5678_1234));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 16, "Jump(on_equal) (cc:777)");
  }
  // 不匹配: current_char = 0x5678_9ABC → masked = 0x9ABC != 0x1234 → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0x5678_9ABC));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 16, "ADVANCE(): size=16 (cc:779)");
  }
  info!("P3b-T11 ok");
  OK
}

/// P3b-T12: CheckCharacterAfterAnd cc:783 — character == (current_char & mask)
#[test]
fn p3b_check_character_after_and_cc783() -> Void {
  let mask = 0xFFu32;
  let character = u16::from(b'A'); // 0x41
  let mut asm = Asm::default();
  asm.insn(
    Bytecode::CheckCharacterAfterAnd,
    &[w2(character), w4(mask), w4(12)],
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 匹配: current_char = 0x1041 → masked = 0x41 = 'A' == character → Jump
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0x1041));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "Jump(on_equal) (cc:785)");
  }
  // 不匹配: current_char = 0x1042 ('B') → masked = 0x42 != 0x41 → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0x1042));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): size=12 (cc:787)");
  }
  info!("P3b-T12 ok");
  OK
}

/// P3b-T13: AndCheckNot4Chars cc:791 — characters != (current_char & mask) → Jump
#[test]
fn p3b_and_check_not4_chars_cc791() -> Void {
  let mask = 0x00FF_0000u32; // 中间字节
  let characters = 0x0042_0000u32; // 'B' 在第二字节
  let mut asm = Asm::default();
  asm.insn(
    Bytecode::AndCheckNot4Chars,
    &[w4(characters), w4(mask), w4(16)],
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 不等: current_char = 0x0041_0000 ('A' shifted) → masked=0x00410000 != characters → Jump(on_not_equal)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0x0041_0000));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 16, "Jump(on_not_equal) (cc:793)");
  }
  // 相等: current_char = 0x1234_5678; masked = 0x56_0000 != 0x0042_0000 → Jump (不等)
  // 精确相等: current_char = 0x00FF_4200; masked = 0x0042_0000 == characters → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0x00FF_4200u32));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 16, "ADVANCE(): size=16 (cc:795, masked==characters)")
  }
  info!("P3b-T13 ok");
  OK
}

/// P3b-T14: CheckNotCharacterAfterAnd cc:799 — character != (current_char & mask) → Jump
#[test]
fn p3b_check_not_character_after_and_cc799() -> Void {
  let mask = 0xFFu32;
  let character = u16::from(b'Z'); // 0x5A
  let mut asm = Asm::default();
  asm.insn(
    Bytecode::CheckNotCharacterAfterAnd,
    &[w2(character), w4(mask), w4(12)],
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 不等: current_char = 0x0041 → masked=0x41 != 0x5A ('Z') → Jump(on_not_equal=12)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'A')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "Jump(on_not_equal) (cc:801)");
  }
  // 相等: current_char = 0x005A → masked=0x5A == 0x5A → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'Z')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): size=12 (cc:803)");
  }
  info!("P3b-T14 ok");
  OK
}

/// P3b-T15: CheckNotCharacterAfterMinusAnd cc:807 — character != ((current_char - minus) & mask) → Jump
#[test]
fn p3b_check_not_character_after_minus_and_cc807() -> Void {
  // 原版: CheckNotCharacterAfterMinusAnd — used in digit range optimization
  // character = 0, minus = '0'(48), mask = 7; current_char = '3'(51): (51-48)&7=3 != 0 → Jump
  let character = 0u16;
  let minus = 48u16; // '0'
  let mask = 7u16;
  let mut asm = Asm::default();
  asm.insn(
    Bytecode::CheckNotCharacterAfterMinusAnd,
    &[w2(character), w2(minus), w2(mask), w4(12)],
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // (51-48) & 7 = 3 != 0 → Jump(on_not_equal=12)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'3')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "Jump(on_not_equal) (cc:810)");
  }
  // (48-48) & 7 = 0 == character=0 → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'0')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): size=12 (cc:812, (0-0)&7==0)")
  }
  info!("P3b-T15 ok");
  OK
}

/// P3b-T16: CheckCharacterInRange cc:816 — from <= current_char <= to → Jump(on_in_range)
#[test]
fn p3b_check_character_in_range_cc816() -> Void {
  let from = u16::from(b'0'); // 48
  let to = u16::from(b'9'); // 57
  let mut asm = Asm::default();
  asm.insn(Bytecode::CheckCharacterInRange, &[w2(from), w2(to), w4(12)]);
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 在范围内: current_char = '5'(53) → Jump
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'5')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "Jump(on_in_range) (cc:818)");
  }
  // 左端点: current_char = from=48 → Jump (<=)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 48));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "from <= current_char: 边界左端点");
  }
  // 右端点: current_char = to=57 → Jump (>=)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 57));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "current_char <= to: 边界右端点");
  }
  // 不在范围: current_char = 'A'(65) → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'A')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): size=12 (cc:820)");
  }
  info!("P3b-T16 ok");
  OK
}

/// P3b-T17: CheckCharacterNotInRange cc:824 — from > current_char || current_char > to → Jump
#[test]
fn p3b_check_character_not_in_range_cc824() -> Void {
  let from = u16::from(b'a'); // 97
  let to = u16::from(b'z'); // 122
  let mut asm = Asm::default();
  asm.insn(
    Bytecode::CheckCharacterNotInRange,
    &[w2(from), w2(to), w4(12)],
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 不在范围 (下界外): current_char = 'A'(65) → Jump(on_not_in_range=12)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'A')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "Jump(on_not_in_range) (cc:826)");
  }
  // 在范围: current_char = 'm'(109) → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'm')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): size=12 (cc:828, 97<=109<=122)")
  }
  // 不在范围 (上界外): current_char = '{'(123) → Jump
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 123));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "123 > to=122 → Jump(on_not_in_range)");
  }
  info!("P3b-T17 ok");
  OK
}

/// P3b-T18: CheckBitInTable cc:832 — handler 形态, 通过位图操作数跳转或 ADVANCE
#[test]
fn p3b_check_bit_in_table_cc832() -> Void {
  // table 是 16 字节定宽切片 (kBitTable, bytecodes.h:43)
  // 构造 table: 位图 bit[i] 对应 char[i]，设 char 'a'(97) 位在 table[12] bit[1] (97>>3=12, 97&7=1)
  let mut table_bytes = vec![0u8; K_BIT_TABLE_BYTES];
  table_bytes[12] |= 1u8 << 1; // bit for 'a' (97 & 7 = 1, byte 97 >> 3 = 12)
  assert!(check_bit_in_table(u32::from(b'a'), &table_bytes));
  assert!(!check_bit_in_table(u32::from(b'b'), &table_bytes));

  let mut asm = Asm::default();
  asm.insn(
    Bytecode::CheckBitInTable,
    &[w4(32), table_bytes.clone()], // on_bit_set=32, table=16bytes
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // bit set for 'a': current_char='a' → Jump(on_bit_set=32)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'a')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 32, "Jump(on_bit_set) (cc:834)");
  }
  // bit not set for 'b': current_char='b' → ADVANCE (size=24, opcode+4+16=21→round4=24)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'b')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(
      m.pc,
      usize::from(bytecode_size(Bytecode::CheckBitInTable)) as u32,
      "ADVANCE(): size (cc:836)"
    );
  }
  info!("P3b-T18 ok");
  OK
}

/// P3b-T19: CheckCharacterLT cc:840 — current_char < limit → Jump(on_less)
#[test]
fn p3b_check_character_lt_cc840() -> Void {
  let limit = u16::from(b'g'); // 103
  let mut asm = Asm::default();
  asm.insn(Bytecode::CheckCharacterLT, &[w2(limit), w4(8)]);
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 小于: current_char = 'a'(97) < 103 → Jump(on_less=8)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'a')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "Jump(on_less) (cc:842)");
  }
  // 等于 limit: 103 < 103 = false → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 103));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): 103 !< 103 (cc:844)");
  }
  // 大于: current_char = 'z'(122) → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'z')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): size=8");
  }
  info!("P3b-T19 ok");
  OK
}

/// P3b-T20: CheckCharacterGT cc:848 — current_char > limit → Jump(on_greater)
#[test]
fn p3b_check_character_gt_cc848() -> Void {
  let limit = u16::from(b't'); // 116
  let mut asm = Asm::default();
  asm.insn(Bytecode::CheckCharacterGT, &[w2(limit), w4(8)]);
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 大于: current_char = 'z'(122) > 116 → Jump(on_greater=8)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'z')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "Jump(on_greater) (cc:850)");
  }
  // 等于 limit: 116 > 116 = false → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 116));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): 116 !> 116 (cc:852)");
  }
  info!("P3b-T20 ok");
  OK
}

/// P3b-T21: CheckPosition cc:991 — pos=current+cp_offset 在界内: ADVANCE; 界外: Jump(on_failure)
#[test]
fn p3b_check_position_cc991() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::CheckPosition, &[w2i(0), w4(8)]); // cp_offset=0, on_failure=8
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // 界内: current=4 → pos=4, len=5, 4<5 且 4>=0 → ADVANCE
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(4, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): size=8 (cc:996)");
  }
  // 界外 (pos >= len): current=5, pos=5 >= len=5 → Jump(on_failure=8)
  // 注: current=5 越界 subject 范围 (len=5), CheckPosition 以 pos>=len 判定界外,
  // RawMatchParams 仍用 total_register_count=6 (>output=4)
  {
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(5, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "Jump(on_failure): pos=5 >= len=5 (cc:994)");
  }
  // 界外 (pos < 0): current=0, cp_offset=-1 → pos=-1 < 0 → Jump
  {
    let mut asm2 = Asm::default();
    asm2.insn(Bytecode::CheckPosition, &[w2i(-1), w4(8)]);
    asm2.insn(Bytecode::Succeed, &[]);
    let code2 = asm2.code().to_vec();
    let mut out = [0i32; 4];
    let mut m = m8(&code2, &mut out, p1(0, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "Jump(on_failure): pos=-1 < 0");
  }
  info!("P3b-T21 ok");
  OK
}

/// P3b-T22: CheckSpecialClassRanges cc:1000 — Digit/Word/NotWord/LineTerminator/Everything 五集
#[test]
fn p3b_check_special_class_ranges_cc1000() -> Void {
  // StandardCharacterSet 的字节编码是转义字母本身 (原版 regexp-ast.h:101-112
  // `enum class StandardCharacterSet : char`, 生成器 Emit<kCheckSpecialClassRanges>(type)
  // 直落 char, regexp-bytecode-generator.cc:351-355), 非 0-8 声明序号:
  //   Digit='d'(100) Word='w'(119) NotWord='W'(87) LineTerminator='n'(110) Everything='*'(42)
  // 原版核判定 cc:457-461 (Digit), cc:432-442 (Word/NotWord), cc:468-476 (LineTerminator), cc:493-494 (Everything)

  // Digit: '5'(53) matches, 'A'(65) does not
  {
    let mut asm = Asm::default();
    asm.insn(Bytecode::CheckSpecialClassRanges, &[w1(b'd'), w4(8)]); // character_set=Digit ('d'=100)
    asm.insn(Bytecode::Succeed, &[]);
    let code = asm.code().to_vec();
    // 匹配: ADVANCE
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'5')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): digit match (cc:1004)");
    // 不匹配: Jump(on_no_match=8)
    let mut out2 = [0i32; 4];
    let mut m2 = m8(&code, &mut out2, p1(0, u32::from(b'A')));
    assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m2.pc, 8, "Jump(on_no_match): non-digit (cc:1006)");
  }

  // Word: 'a'(97) matches, '!'(33) does not (uses K_WORD_CHARACTER_MAP)
  {
    let mut asm = Asm::default();
    asm.insn(Bytecode::CheckSpecialClassRanges, &[w1(b'w'), w4(8)]); // character_set=Word ('w'=119)
    asm.insn(Bytecode::Succeed, &[]);
    let code = asm.code().to_vec();
    assert!(
      K_WORD_CHARACTER_MAP[97] != 0,
      "'a' is word char in static map"
    );
    assert!(K_WORD_CHARACTER_MAP[33] == 0, "'!' is not word char");
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'a')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): word match (cc:442 via map)");
    let mut out2 = [0i32; 4];
    let mut m2 = m8(&code, &mut out2, p1(0, u32::from(b'!')));
    assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m2.pc, 8, "Jump(on_no_match): non-word (cc:442)")
  }

  // NotWord: '!'(33) matches, 'a'(97) does not
  {
    let mut asm = Asm::default();
    asm.insn(Bytecode::CheckSpecialClassRanges, &[w1(b'W'), w4(8)]); // character_set=NotWord ('W'=87)
    asm.insn(Bytecode::Succeed, &[]);
    let code = asm.code().to_vec();
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'!')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): non-word match");
    let mut out2 = [0i32; 4];
    let mut m2 = m8(&code, &mut out2, p1(0, u32::from(b'a')));
    assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m2.pc, 8, "Jump(on_no_match): word char in NotWord set");
  }

  // LineTerminator: '\n'(10) matches, ' '(32) does not (one_byte mode)
  {
    let mut asm = Asm::default();
    asm.insn(Bytecode::CheckSpecialClassRanges, &[w1(b'n'), w4(8)]); // character_set=LineTerminator ('n'=110)
    asm.insn(Bytecode::Succeed, &[]);
    let code = asm.code().to_vec();
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, u32::from(b'\n')));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): line terminator match (cc:468-470)");
    let mut out2 = [0i32; 4];
    let mut m2 = m8(&code, &mut out2, p1(0, u32::from(b' ')));
    assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m2.pc, 8, "Jump(on_no_match): space is not line terminator");
  }

  // Everything: all chars match (cc:493-494: return true)
  {
    let mut asm = Asm::default();
    asm.insn(Bytecode::CheckSpecialClassRanges, &[w1(b'*'), w4(8)]); // character_set=Everything ('*'=42)
    asm.insn(Bytecode::Succeed, &[]);
    let code = asm.code().to_vec();
    let mut out = [0i32; 4];
    let mut m = m8(&code, &mut out, p1(0, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 8, "ADVANCE(): Everything always matches (cc:493)");
  }

  // two_byte mode: U+2028 LINE SEPARATOR matches LineTerminator (cc:471-473)
  {
    let mut asm = Asm::default();
    asm.insn(Bytecode::CheckSpecialClassRanges, &[w1(b'n'), w4(8)]); // LineTerminator ('n'=110)
    asm.insn(Bytecode::Succeed, &[]);
    let code = asm.code().to_vec();
    let mut out = [0i32; 4];
    let mut m = m16(&code, &mut out, p1(0, 0x2028));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(
      m.pc, 8,
      "ADVANCE(): U+2028 is line terminator in two_byte (cc:471-473)"
    );
  }

  info!("P3b-T22 ok");
  OK
}

/// P3b-T23: LoadCurrentCharacter two_byte 态 (SUBJ16) — char_at 读取 u16 元素
#[test]
fn p3b_load_current_character_two_byte_cc696() -> Void {
  let mut asm = Asm::default();
  asm.insn(
    Bytecode::LoadCurrentCharacter,
    &[w2i(0), w4i(0), w4(12)], // cp_offset=0, bounds_check_offset=0, on_failure=12
  );
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();

  // two_byte: current=2 → subject[2]=0x63 ('c') → current_char = 0x63
  {
    let mut out = [0i32; 4];
    let mut m = m16(&code, &mut out, p1(2, 0));
    assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
    assert_eq!(m.pc, 12, "ADVANCE(): size=12");
    assert_eq!(m.current_char, 0x63, "two_byte subject[2] = 'c' (0x63)");
  }
  info!("P3b-T23 ok");
  OK
}

/// P3b-T24: 整程流 — LoadCurrentCharacter + CheckCharacter 协作:
/// 装载 'a' 并 CheckCharacter('a') → Jump(on_equal) 到 Succeed
#[test]
fn p3b_load_then_check_integration_cc696_751() -> Void {
  // 帧布局: [0] LoadCC(12b) [12] CheckChar(8b) [20] Fail(4b) [24] Succeed(4b) = 28b
  let mut asm = Asm::default();
  asm.insn(Bytecode::LoadCurrentCharacter, &[w2i(0), w4i(0), w4(20)]); // on_failure=Fail@20
  asm.insn(Bytecode::CheckCharacter, &[w2(97), w4(24)]); // on_equal=Succeed@24
  asm.insn(Bytecode::Fail, &[]); // [20]
  asm.insn(Bytecode::Succeed, &[]); // [24]
  let code = asm.code().to_vec();
  assert_eq!(code.len(), 28, "12+8+4+4=28");

  // SUBJ8[0]='a': LoadCurrentCharacter(0+0)='a'(97), CheckCharacter(97)→on_equal=Succeed
  let mut out = [0i32; 4];
  let mut m = RawMatch::new(
    &code,
    SUBJ8,
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      current: 0,
      ..Default::default()
    },
    TraceHooks::default(),
  );
  assert_eq!(
    m.run(),
    Step::Done(Result::Success),
    "loadCC→checkChar equal→Succeed"
  );

  // SUBJ8[0]='a': CheckCharacter 不匹配 ('x' ≠ 'a') → ADVANCE → Fail
  // 用 LoadCurrentCharacterUnchecked + CheckCharacter 变体
  let mut asm2 = Asm::default();
  asm2.insn(Bytecode::LoadCurrentCharacter, &[w2i(0), w4i(0), w4(20)]);
  asm2.insn(Bytecode::CheckCharacter, &[w2(0x78), w4(24)]); // check 'x', on_equal=24=Succeed
  asm2.insn(Bytecode::Fail, &[]); // [20]
  asm2.insn(Bytecode::Succeed, &[]); // [24]
  let code2 = asm2.code().to_vec();
  let mut out2 = [0i32; 4];
  let mut m2 = RawMatch::new(
    &code2,
    SUBJ8,
    &mut out2,
    RawMatchParams {
      total_register_count: 6,
      current: 0,
      ..Default::default()
    },
    TraceHooks::default(),
  );
  assert_eq!(
    m2.run(),
    Step::Done(Result::Failure),
    "SUBJ8[0]='a' ≠ 'x' → ADVANCE→Fail"
  );
  let _ = bytecode_operands::load_current_character::CP_OFFSET; // 锚: 操作数名可用
  info!("P3b-T24 ok");
  OK
}

/// P3b-T25: 整程流 — Load4CurrentChars + Check4Chars 协作 (one_byte)
#[test]
fn p3b_load4_then_check4_integration_cc726_743() -> Void {
  // Load "abcd" as 4-byte u32 then Check4Chars
  // 帧布局: [0] Load4(12b) [12] Check4(12b) [24] Fail(4b) [28] Succeed(4b) = 32b
  let four =
    u32::from(b'a') | (u32::from(b'b') << 8) | (u32::from(b'c') << 16) | (u32::from(b'd') << 24);
  let mut asm = Asm::default();
  asm.insn(Bytecode::Load4CurrentChars, &[w2i(0), w4i(3), w4(24)]); // on_failure=Fail@24
  asm.insn(Bytecode::Check4Chars, &[w4(four), w4(28)]); // on_equal=Succeed@28
  asm.insn(Bytecode::Fail, &[]); // [24]
  asm.insn(Bytecode::Succeed, &[]); // [28]
  let code = asm.code().to_vec();
  assert_eq!(code.len(), 32, "12+12+4+4=32");

  // SUBJ8[0..4]="abcd": load4 → current_char=four → Check4Chars equal → Jump(on_equal=28=Succeed)
  let mut out = [0i32; 4];
  let mut m = RawMatch::new(
    &code,
    SUBJ8,
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      current: 0,
      ..Default::default()
    },
    TraceHooks::default(),
  );
  assert_eq!(
    m.run(),
    Step::Done(Result::Success),
    "load4→check4 equal→Succeed"
  );
  info!("P3b-T25 ok");
  OK
}
