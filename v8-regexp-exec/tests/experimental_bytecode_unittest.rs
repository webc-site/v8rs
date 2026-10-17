//! 实验性字节码 IR 规约测试 (v8-regexp-exec / experimental::bytecode)。
//!
//! 规约来源 (实测枚举): v8/test/unittests/regexp/ 与 v8/test/cctest/regexp/ 下无
//! experimental-bytecode 专有用例 (实测 grep 0 命中; experimental 引擎经
//! mjsunit regexp-experimental.js 间接消费)。按任务规范, Dump 金标准向量依
//! v8/src/regexp/experimental/experimental-bytecode.cc 的 std::ostream 语义
//! (含 std::hex 粘性、setfill/setw、endl) 逐字符手工推演构造, 与原版 C++ 输出 1:1。

use aok::{OK, Void};
use v8_regexp_exec::experimental::bytecode::{
  ALL_OPCODES, Instruction, LookaroundPayload, Opcode, Payload, Uc16Range, dump,
};
use v8_regexp_syntax::{AssertionType, LookaroundType};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// Opcode 枚举序数金标准 (v8 h:101-121, 19 值逐一对账; ALL_OPCODES 序即序数)
#[test]
fn opcode_discriminants_golden() -> Void {
  let golden = [
    ("ACCEPT", 0),
    ("ASSERTION", 1),
    ("CLEAR_REGISTER", 2),
    ("CONSUME_RANGE", 3),
    ("RANGE_COUNT", 4),
    ("FORK", 5),
    ("JMP", 6),
    ("SET_REGISTER_TO_CP", 7),
    ("SET_QUANTIFIER_TO_CLOCK", 8),
    ("FILTER_QUANTIFIER", 9),
    ("FILTER_GROUP", 10),
    ("FILTER_LOOKAROUND", 11),
    ("FILTER_CHILD", 12),
    ("BEGIN_LOOP", 13),
    ("END_LOOP", 14),
    ("START_LOOKAROUND", 15),
    ("END_LOOKAROUND", 16),
    ("WRITE_LOOKAROUND_TABLE", 17),
    ("READ_LOOKAROUND_TABLE", 18),
  ];
  assert_eq!(golden.len(), ALL_OPCODES.len());
  for (pos, (name, want)) in golden.iter().enumerate() {
    let op = ALL_OPCODES[pos];
    assert_eq!(op as i32, *want, "Opcode {name} 序数");
    assert_eq!(
      op, ALL_OPCODES[*want as usize],
      "Opcode {name} 序数↔枚举双射"
    );
  }
  // C++ 名别名常量与 camelCase 变体同源
  assert_eq!(Opcode::READ_LOOKAROUND_TABLE, Opcode::ReadLookaroundTable);
  OK
}

/// 21 个构造工厂的 opcode/payload 配对 (v8 h:150-288) + union 成员语义
#[test]
fn factory_payload_pairing() -> Void {
  // CONSUME_RANGE 族 (h:150-163)
  assert_eq!(
    Instruction::consume_range(0xFFFF, 0x0000),
    Instruction {
      opcode: Opcode::ConsumeRange,
      payload: Payload::ConsumeRange(Uc16Range {
        min: 0xFFFF,
        max: 0x0000
      })
    }
  );
  assert_eq!(
    Instruction::consume_any_char().payload,
    Payload::ConsumeRange(Uc16Range {
      min: 0,
      max: 0xFFFF
    })
  );
  assert_eq!(
    Instruction::fail().payload,
    Payload::ConsumeRange(Uc16Range {
      min: 0xFFFF,
      max: 0
    })
  );
  // pc 通道 (h:172-184, 241-246): FORK/JMP/FILTER_CHILD 共用 union.pc
  assert_eq!(Instruction::fork(7).payload, Payload::Pc(7));
  assert_eq!(Instruction::jmp(7).payload, Payload::Pc(7));
  assert_eq!(Instruction::filter_child(7).payload, Payload::Pc(7));
  // 寄存器 (h:192-211)
  assert_eq!(
    Instruction::set_register_to_cp(3).payload,
    Payload::RegisterIndex(3)
  );
  assert_eq!(
    Instruction::clear_register(3).payload,
    Payload::RegisterIndex(3)
  );
  // 断言 (h:199-204)
  assert_eq!(Instruction::assertion(AssertionType::Boundary).payload, {
    Payload::Assertion(AssertionType::Boundary)
  });
  // 量词/分组/环视 id (h:213-239, 275-280)
  assert_eq!(
    Instruction::set_quantifier_to_clock(2).payload,
    Payload::QuantifierId(2)
  );
  assert_eq!(
    Instruction::filter_quantifier(2).payload,
    Payload::QuantifierId(2)
  );
  assert_eq!(Instruction::filter_group(4).payload, Payload::GroupId(4));
  assert_eq!(
    Instruction::filter_lookaround(6).payload,
    Payload::LookaroundId(6)
  );
  assert_eq!(
    Instruction::write_look_table(6).payload,
    Payload::LookaroundId(6)
  );
  // 无载荷 (h:186-190, 248-258, 269-273)
  for inst in [
    Instruction::accept(),
    Instruction::begin_loop(),
    Instruction::end_loop(),
    Instruction::end_lookaround(),
  ] {
    assert_eq!(inst.payload, Payload::None);
  }
  // START/READ_LOOKAROUND 位域载荷同编码 (h:260-288)
  assert_eq!(
    Instruction::start_lookaround(3, false, LookaroundType::Lookahead).payload,
    Instruction::read_look_table(3, false, LookaroundType::Lookahead).payload
  );
  OK
}

/// IsFilter 语义 (v8 h:290-296): 仅 FILTER_GROUP/FILTER_QUANTIFIER/FILTER_CHILD;
/// 陷阱对账 — FILTER_LOOKAROUND 不属于 IsFilter (原版显式排除)
#[test]
fn is_filter_semantics() -> Void {
  assert!(Instruction::filter_group(0).is_filter());
  assert!(Instruction::filter_quantifier(0).is_filter());
  assert!(Instruction::filter_child(0).is_filter());
  assert!(!Instruction::filter_lookaround(0).is_filter());
  assert!(!Instruction::accept().is_filter());
  assert!(!Instruction::fork(0).is_filter());
  OK
}

/// LookaroundPayload 位域往返 (v8 h:128-148): index 占 bit2..31 (30 位)、
/// is_positive 占 bit0、type 占 bit1; 相邻位域互不污染
#[test]
fn lookaround_payload_bitfield_roundtrip() -> Void {
  let max_index: i32 = (1 << 30) - 1;
  for (index, positive, ty) in [
    (0, false, LookaroundType::Lookahead),
    (max_index, true, LookaroundType::Lookbehind),
    { (5, true, LookaroundType::Lookbehind) },
    (12, false, LookaroundType::Lookahead),
  ] {
    let p = LookaroundPayload::new(index, positive, ty);
    assert_eq!(p.index(), index as u32, "index 位域 (shift 2, size 30)");
    assert_eq!(
      p.is_positive(),
      positive,
      "IsPositive 位域 (shift 0, size 1)"
    );
    assert_eq!(p.type_(), ty, "Type 位域 (shift 1, size 1)");
  }
  // 编码等价类: 同三元组必然同 raw (位域确定性), 异三元组必然可区分
  let a = Instruction::start_lookaround(9, true, LookaroundType::Lookahead).payload;
  let b = Instruction::read_look_table(9, true, LookaroundType::Lookahead).payload;
  let c = Instruction::read_look_table(9, false, LookaroundType::Lookahead).payload;
  let d = Instruction::read_look_table(9, true, LookaroundType::Lookbehind).payload;
  let e = Instruction::read_look_table(8, true, LookaroundType::Lookahead).payload;
  assert_eq!(a, b);
  for other in [c, d, e] {
    assert_ne!(a, other, "位域任一分量不同则 payload 不同");
  }
  // LookaroundPayload Display (cc:149-155): `<index> (ahead|behind, positive|negative)`
  assert_eq!(
    LookaroundPayload::new(5, true, LookaroundType::Lookbehind).to_string(),
    "5 (behind, positive)"
  );
  OK
}

/// 单指令 Dump 金标准 (cc:27-118 operator<<(ostream&, Instruction), 全新流=十进制),
/// 覆盖全部 19 opcode + ASSERTION 6 个可达子型 (END_OF_BUFFER 为原版 UNREACHABLE 不测)
#[test]
fn instruction_display_golden() -> Void {
  let cases: [(Instruction, &str); 24] = [
    (Instruction::accept(), "ACCEPT"),
    (
      Instruction::assertion(AssertionType::StartOfInput),
      "ASSERTION START_OF_INPUT",
    ),
    (
      Instruction::assertion(AssertionType::EndOfInput),
      "ASSERTION END_OF_INPUT",
    ),
    (
      Instruction::assertion(AssertionType::StartOfLine),
      "ASSERTION START_OF_LINE",
    ),
    (
      Instruction::assertion(AssertionType::EndOfLine),
      "ASSERTION END_OF_LINE",
    ),
    (
      Instruction::assertion(AssertionType::Boundary),
      "ASSERTION BOUNDARY",
    ),
    (
      Instruction::assertion(AssertionType::NonBoundary),
      "ASSERTION NON_BOUNDARY",
    ),
    (Instruction::clear_register(0), "CLEAR_REGISTER 0"),
    (
      Instruction::consume_range(0x61, 0x62),
      "CONSUME_RANGE [a, b]",
    ),
    // 不可打印: "0x" + 小写十六进制无前导零 (cc:16-23)
    (
      Instruction::consume_any_char(),
      "CONSUME_RANGE [0x0, 0xffff]",
    ),
    (Instruction::fail(), "CONSUME_RANGE [0xffff, 0x0]"),
    // 0x20 空格与 0x7E 波浪号是 isprint 边界 (C locale 可打印 = 0x20..=0x7E)
    (
      Instruction::consume_range(0x20, 0x7E),
      "CONSUME_RANGE [ , ~]",
    ),
    (
      Instruction::consume_range(0x1F, 0x7F),
      "CONSUME_RANGE [0x1f, 0x7f]",
    ),
    (Instruction::range_count(17), "RANGE_COUNT 17"),
    (Instruction::fork(5), "FORK 5"),
    (Instruction::jmp(6), "JMP 6"),
    (Instruction::set_register_to_cp(7), "SET_REGISTER_TO_CP 7"),
    (
      Instruction::set_quantifier_to_clock(8),
      "SET_QUANTIFIER_TO_CLOCK 8",
    ),
    (Instruction::filter_quantifier(9), "FILTER_QUANTIFIER 9"),
    (Instruction::filter_group(10), "FILTER_GROUP 10"),
    (Instruction::filter_lookaround(11), "FILTER_LOOKAROUND 11"),
    (Instruction::filter_child(12), "FILTER_CHILD 12"),
    (Instruction::begin_loop(), "BEGIN_LOOP"),
    (Instruction::end_loop(), "END_LOOP"),
  ];
  for (inst, want) in cases {
    assert_eq!(inst.to_string(), want);
  }
  // START/END/READ/WRITE_LOOKAROUND 族 (cc:104-115)
  assert_eq!(
    Instruction::start_lookaround(5, false, LookaroundType::Lookbehind).to_string(),
    "START_LOOKAROUND 5 (behind, negative)"
  );
  assert_eq!(
    Instruction::read_look_table(3, true, LookaroundType::Lookahead).to_string(),
    "READ_LOOKAROUND_TABLE 3 (ahead, positive)"
  );
  assert_eq!(Instruction::end_lookaround().to_string(), "END_LOOKAROUND");
  assert_eq!(
    Instruction::write_look_table(2).to_string(),
    "WRITE_LOOKAROUND_TABLE 2"
  );
  OK
}

/// 向量 Dump 金标准 (cc:136-147 operator<<(ostream&, Vector<Instruction>)):
/// 行号 setfill('0') + setw(DigitsRequiredBelow(len)) + ": " + inst + endl
#[test]
fn dump_vector_golden_decimal() -> Void {
  // 3 条 → 1 位行号, 全程十进制 (无 hex 触发)
  let insts = [
    Instruction::accept(),
    Instruction::fork(1),
    Instruction::consume_range(0x61, 0x62),
  ];
  assert_eq!(
    dump(&insts),
    "0: ACCEPT\n1: FORK 1\n2: CONSUME_RANGE [a, b]\n"
  );
  // 空向量 → 空串 (循环 0 次; DigitsRequiredBelow(0)=1 无输出)
  assert_eq!(dump(&[]), "");
  // 11 条 → 2 位行号 (cc:124-132: i=10 起每 10 倍进一位; 10 条仍 1 位、11 条 2 位)
  let ten = vec![Instruction::accept(); 10];
  assert_eq!(dump(&ten).lines().next(), Some("0: ACCEPT"));
  assert_eq!(dump(&ten).lines().last(), Some("9: ACCEPT"));
  let eleven = vec![Instruction::accept(); 11];
  assert_eq!(dump(&eleven).lines().next(), Some("00: ACCEPT"));
  assert_eq!(dump(&eleven).lines().last(), Some("10: ACCEPT"));
  // 100 条 → 2 位行号, 99 收尾; 行数 = len
  let hundred = vec![Instruction::accept(); 100];
  let hundred_dump = dump(&hundred);
  let dumped: Vec<&str> = hundred_dump.lines().collect();
  assert_eq!(dumped.len(), 100);
  assert_eq!(dumped[99], "99: ACCEPT");
  OK
}

/// 向量 Dump 的 std::hex 粘性对账 (原版真实行为, 逐字符 1:1 的关键差异点):
/// PrintAsciiOrHex 一旦走 "0x" 分支, 同流后续所有数值 (含行号、pc、index) 转小写 hex;
/// setfill('0') 与 setw 仍生效 → 十进制 10 的行号渲染为 "0a"
#[test]
fn dump_vector_sticky_hex_golden() -> Void {
  // 3 条: fail() 触发 hex 后, RANGE_COUNT 17 → "11", FORK 15 → "f"
  let insts = [
    Instruction::fail(),
    Instruction::range_count(17),
    Instruction::fork(15),
  ];
  assert_eq!(
    dump(&insts),
    "0: CONSUME_RANGE [0xffff, 0x0]\n1: RANGE_COUNT 11\n2: FORK f\n"
  );
  // 11 条: 行号 10 在 hex 下为 "a", setw(2)+setfill('0') → "0a"
  let mut insts = vec![Instruction::fail()];
  insts.extend(vec![Instruction::accept(); 10]);
  let dumped = dump(&insts);
  assert_eq!(
    dumped.lines().next(),
    Some("00: CONSUME_RANGE [0xffff, 0x0]")
  );
  assert_eq!(dumped.lines().nth(1), Some("01: ACCEPT"));
  assert_eq!(dumped.lines().last(), Some("0a: ACCEPT"));
  // hex 下 lookaround index 12 → "c"
  let insts = [
    Instruction::consume_range(0xFFFF, 0x0000),
    Instruction::read_look_table(12, true, LookaroundType::Lookahead),
  ];
  assert_eq!(
    dump(&insts),
    "0: CONSUME_RANGE [0xffff, 0x0]\n1: READ_LOOKAROUND_TABLE c (ahead, positive)\n"
  );
  OK
}
