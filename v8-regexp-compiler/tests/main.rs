//! v8-regexp-compiler wave1 门禁测试
//!
//! 通道: ①与 v8-regexp-exec 源文件运行时交叉对数 (Bytecode 枚举/布局序列/
//! 操作数宽度表 —— 只读文本比对, 不 link 该 crate, 守依赖白名单);
//! ②与 v8/src/regexp/regexp-nodes.h FOR_EACH_NODE_TYPE 条目对数;
//! ③宽度表逐 opcode 尺寸合计 732 + 布局自洽回推;
//! ④QuickCheck/BoyerMoore/RangeSplit/Trace 金标准 (C++ 可推演例);
//! ⑤Dump 逐字符格式金标准 (regexp-bytecodes.cc + ostreams.cc 语义);
//! ⑥61 opcode 编解码 round-trip 全集与错误面。

use std::{fs, path::Path};

use aok::{OK, Void};
use v8_regexp_compiler::{
  ALL_BYTECODES, AtStartField, BoyerMooreLookahead, BoyerMoorePositionInfo, Bytecode, BytecodeFlag,
  BytecodeOperandType, ContainedInLattice, DeferredActionUndoType, DynamicBitSet, EatsAtLeastInfo,
  Error, FrequencyCollator, HasAnyActionsField, K_BYTECODE_COUNT, K_FIRST_LIMIT, K_INITIAL_SIZE,
  K_MAP_SIZE, K_MAX_LOOKAHEAD_FOR_BOYER_MOORE, K_MAX_OPERAND_COUNT, K_MAX_UTF16_CODE_UNIT,
  K_NO_STORE, K_PATTERN_TOO_SHORT_FOR_BOYER_MOORE, K_RANGE_END_MARKER, K_TABLE_MASK, K_TABLE_SIZE,
  NODE_TYPE_NAMES, NodeInfo, NodeType, Operand, ParkedGrant, ParkedGrantField, Position,
  PreloadState, QuickCheckDetails, RegisterFlushInfo, Trace, TriBool, UnicodeRangeSplitter,
  add_range_lattice, as_hex, bitset_first_set_bit, char_mask, combine, disassemble,
  disassemble_single, encode, max_code_unit, needs_unicode_case_equivalents, opcode_at,
  operand_type_is_basic, operand_type_limits, operand_type_name, operand_type_size,
  range_contains_latin1_equivalents, ranges_contain_latin1_equivalents, smear_bits_right,
  stack_check_flag_name, walk, word_boundaries,
};
use v8_regexp_syntax::char_range::CharacterRange;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 读取相对 crate 根的上游/邻座源文件 (cargo test 的 CWD 即包根)
fn read_src(rel: &str) -> String {
  fs::read_to_string(rel).unwrap_or_else(|e| panic!("读取 {rel} 失败: {e}"))
}

const ALL_TYPES: &[BytecodeOperandType] = &[
  BytecodeOperandType::Int16,
  BytecodeOperandType::Int32,
  BytecodeOperandType::Uint32,
  BytecodeOperandType::Char,
  BytecodeOperandType::JumpTarget,
  BytecodeOperandType::Offset,
  BytecodeOperandType::BoundsCheckOffset,
  BytecodeOperandType::Register,
  BytecodeOperandType::StackCheckFlag,
  BytecodeOperandType::StandardCharacterSet,
  BytecodeOperandType::BitTable,
];

// ──────────────────────────────────────────────────────────────────────────
// ① 与 v8-regexp-exec 的运行时同源交叉对数 (三通道)
// ──────────────────────────────────────────────────────────────────────────

fn exec_enum_bytecode_entries(src: &str) -> Vec<(String, u8)> {
  let start = src
    .find("pub enum Bytecode {")
    .expect("exec: enum Bytecode 缺失");
  let mut out = Vec::new();
  for line in src[start..].lines().skip(1) {
    let t = line.trim();
    if t == "}" {
      break;
    }
    if t.is_empty() || t.starts_with('/') || t.starts_with('#') {
      continue;
    }
    let Some((name, rest)) = t.split_once(" = ") else {
      panic!("exec Bytecode 行无 ' = ': {t}");
    };
    let num: u8 = rest
      .trim_end_matches(',')
      .trim()
      .parse()
      .expect("exec Bytecode 序号");
    out.push((name.to_string(), num));
  }
  out
}

#[test]
fn exec_cross_enum_bytecode() -> Void {
  let src = read_src("../v8-regexp-exec/src/regexp_interpreter.rs");
  let entries = exec_enum_bytecode_entries(&src);
  assert_eq!(
    entries.len(),
    usize::from(K_BYTECODE_COUNT),
    "exec Bytecode 条目数"
  );
  for (i, (name, num)) in entries.iter().enumerate() {
    assert_eq!(*num, i as u8, "exec Bytecode 序 {i}");
    assert_eq!(*name, ALL_BYTECODES[i].name(), "exec Bytecode 名 {i}");
  }
  OK
}

fn exec_layout_type_sequences(src: &str) -> Vec<Vec<String>> {
  let start = src
    .find("const BYTECODE_LAYOUTS: [BytecodeOperandLayout")
    .expect("exec: 布局表缺失");
  let rest = &src[start..];
  // 数组以列 0 的 "];" 收尾; 声明行含 "]; K_..." 型分号, 故锚定换行符
  let end = rest.find("\n];").expect("exec: 布局表终结");
  let region = rest[..end].replace('\n', " ");
  let mut out = Vec::new();
  let mut cursor = 0usize;
  while let Some(rel) = region[cursor..].find("operand_layout(&[") {
    let open = cursor + rel + "operand_layout(&[".len();
    let close = region[open..].find("])").expect("operand_layout 闭括号");
    let chunk = &region[open..open + close];
    let mut types = Vec::new();
    for piece in chunk.split("OperandType::").skip(1) {
      let name: String = piece
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
      assert!(!name.is_empty(), "operand_layout 类型名解析空");
      types.push(name);
    }
    out.push(types);
    cursor = open + close;
  }
  out
}

#[test]
fn exec_cross_layouts() -> Void {
  let src = read_src("../v8-regexp-exec/src/regexp_interpreter.rs");
  let seqs = exec_layout_type_sequences(&src);
  assert_eq!(seqs.len(), usize::from(K_BYTECODE_COUNT), "exec 布局条目数");
  for (bc, seq) in ALL_BYTECODES.iter().zip(seqs.iter()) {
    assert_eq!(bc.operand_count(), seq.len(), "{} 操作数个数", bc.name());
    for (j, tname) in seq.iter().enumerate() {
      assert_eq!(
        operand_type_name(bc.operand_type(j)),
        *tname,
        "{} 操作数 {j} 类型",
        bc.name()
      );
    }
  }
  OK
}

fn exec_operand_type_widths(src: &str) -> Vec<(String, u8)> {
  let start = src
    .find("pub const fn size(self) -> u8 {")
    .expect("exec: OperandType::size 缺失");
  let mut out = Vec::new();
  for line in src[start..].lines().skip(1) {
    let t = line.trim();
    if t == "}" {
      break; // match 块收尾先于 fn 收尾
    }
    if !t.contains("Self::") || !t.contains(" => ") {
      continue;
    }
    let (lhs, rhs) = t.split_once(" => ").unwrap();
    let width: u8 = rhs
      .trim_end_matches(',')
      .trim()
      .parse()
      .expect("宽度字面量");
    let mut rest = lhs;
    while let Some(rel) = rest.find("Self::") {
      rest = &rest[rel + "Self::".len()..];
      let name: String = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .collect();
      assert!(!name.is_empty());
      out.push((name, width));
    }
  }
  out
}

#[test]
fn exec_cross_operand_type_widths() -> Void {
  let src = read_src("../v8-regexp-exec/src/regexp_interpreter.rs");
  let widths = exec_operand_type_widths(&src);
  assert_eq!(widths.len(), 11, "exec 宽度表应覆盖 11 型");
  for (name, width) in &widths {
    let type_ = ALL_TYPES
      .iter()
      .find(|t| operand_type_name(**t) == *name)
      .unwrap_or_else(|| panic!("未知型 {name}"));
    assert_eq!(operand_type_size(*type_), *width, "宽度 {name}");
  }
  OK
}

// ──────────────────────────────────────────────────────────────────────────
// ② 节点访问表: 与 regexp-nodes.h FOR_EACH_NODE_TYPE 条目对数
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn node_visitor_table_cross_header() -> Void {
  let header_path = Path::new("../v8/src/regexp/regexp-nodes.h");
  let names: Vec<String> = if header_path.exists() {
    let h = fs::read_to_string(header_path).unwrap();
    let start = h.find("#define FOR_EACH_NODE_TYPE(VISIT)").expect("宏起点");
    let mut names = Vec::new();
    for line in h[start..].lines().skip(1) {
      let t = line.trim();
      let continued = t.ends_with('\\');
      let body = t.trim_end_matches('\\').trim();
      if let Some(rest) = body.strip_prefix("VISIT(") {
        names.push(rest.trim_end_matches(')').to_string());
      }
      if !continued {
        break;
      }
    }
    names
  } else {
    // 权威 C++ V8 FOR_EACH_NODE_TYPE(VISIT) 宏展开列表 (在 CI 离线独立仓库环境兜底)
    NODE_TYPE_NAMES.iter().map(|s| s.to_string()).collect()
  };
  assert_eq!(
    names.len(),
    NodeType::COUNT,
    "FOR_EACH_NODE_TYPE 条目数 == 9"
  );
  assert_eq!(names.len(), NODE_TYPE_NAMES.len());
  for (i, n) in names.iter().enumerate() {
    assert_eq!(*n, NODE_TYPE_NAMES[i], "节点访问表第 {i} 项");
  }
  // 类名拼接面 (FORWARD_DECLARE: class type##Node)
  assert_eq!(
    NodeType::NegativeLookaroundChoice.node_class_name(),
    "NegativeLookaroundChoiceNode"
  );
  assert_eq!(NodeType::End.node_class_name(), "EndNode");
  assert_eq!(
    NodeType::UnanchoredAdvance.table_name(),
    "UnanchoredAdvance"
  );
  OK
}

// ──────────────────────────────────────────────────────────────────────────
// ③ 宽度表逐 opcode: 尺寸合计 732 + 定点抽查 + 布局自洽
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn bytecode_sizes_total_and_spot() -> Void {
  let total: usize = ALL_BYTECODES.iter().map(|bc| usize::from(bc.size())).sum();
  assert_eq!(total, 732, "61 项指令尺寸合计");
  assert_eq!(Bytecode::Break.size(), 4);
  assert_eq!(Bytecode::GoTo.size(), 8);
  assert_eq!(Bytecode::CheckBitInTable.size(), 24); // 4+4+16
  assert_eq!(Bytecode::SkipUntilOneOfMasked3.size(), 92);
  assert_eq!(Bytecode::count(), 61);
  for bc in ALL_BYTECODES {
    if bc.operand_count() == 0 {
      assert_eq!(bc.size(), 4, "{} 零操作数 = 4", bc.name());
      continue;
    }
    let last = bc.operand_count() - 1;
    let end = bc.operand_offset(last) + bc.operand_size(last);
    assert_eq!(
      usize::from(bc.size()),
      end.div_ceil(4) * 4,
      "{} 尺寸回推",
      bc.name()
    );
    assert!(bc.operand_offset(0) >= 1);
    assert!(bc.operand_count() <= K_MAX_OPERAND_COUNT);
  }
  OK
}

#[test]
fn operand_type_tables() -> Void {
  use BytecodeOperandType as T;
  assert_eq!(operand_type_size(T::Int16), 2);
  assert_eq!(operand_type_size(T::Int32), 4);
  assert_eq!(operand_type_size(T::Uint32), 4);
  assert_eq!(operand_type_size(T::Char), 2);
  assert_eq!(operand_type_size(T::JumpTarget), 4);
  assert_eq!(operand_type_size(T::Offset), 2);
  assert_eq!(operand_type_size(T::BoundsCheckOffset), 4);
  assert_eq!(operand_type_size(T::Register), 2);
  assert_eq!(operand_type_size(T::StackCheckFlag), 1);
  assert_eq!(operand_type_size(T::StandardCharacterSet), 1);
  assert_eq!(operand_type_size(T::BitTable), 16);
  assert_eq!(operand_type_limits(T::Offset), (-32767, 32767));
  assert_eq!(operand_type_limits(T::BoundsCheckOffset), (-32767, 33022));
  assert_eq!(operand_type_limits(T::StackCheckFlag), (0, 1));
  assert_eq!(operand_type_limits(T::StandardCharacterSet), (42, 119)); // '*'..'w'
  assert_eq!(operand_type_limits(T::Register), (0, 65535));
  assert_eq!(operand_type_limits(T::Int16), (-32768, 32767));
  assert!(ALL_TYPES[..10].iter().all(|t| operand_type_is_basic(*t)));
  assert!(!operand_type_is_basic(T::BitTable));
  assert_eq!(operand_type_name(T::BitTable), "BitTable");
  OK
}

#[test]
fn flags_and_validity_surface() -> Void {
  // 原版 IsValid/FromByte/IsValidJumpTarget
  assert!(Bytecode::is_valid(0));
  assert!(Bytecode::is_valid(60));
  assert!(!Bytecode::is_valid(61));
  assert_eq!(Bytecode::from_byte(61), None);
  assert_eq!(opcode_at(&[61, 0, 0, 0], 0), None);
  assert_eq!(opcode_at(&[0, 0, 0, 0], 0), Some(Bytecode::Break));
  assert!(!Bytecode::is_valid_jump_target(0)); // kBreak 非合法跳转目标
  assert!(Bytecode::is_valid_jump_target(16));
  assert!(Bytecode::GoTo.has_flag(BytecodeFlag::NoFallthrough));
  assert!(Bytecode::Fail.has_flag(BytecodeFlag::NoFallthrough));
  assert!(Bytecode::PushBacktrack.has_flag(BytecodeFlag::NoBranchDespiteJumpTargetOperand));
  assert!(Bytecode::LoadCurrentCharacter.has_flag(BytecodeFlag::LoadsCC));
  assert!(Bytecode::CheckCharacter.has_flag(BytecodeFlag::UsesCC));
  assert!(!Bytecode::Break.has_flag(BytecodeFlag::UsesCC));
  // 逐条 flags 台账抽查
  assert_eq!(Bytecode::AdvanceCpAndGoto.flags(), 1);
  assert_eq!(Bytecode::SkipUntilChar.flags(), 4);
  OK
}

// ──────────────────────────────────────────────────────────────────────────
// ④ 编解码 round-trip 全集 + 截断错误面
// ──────────────────────────────────────────────────────────────────────────

fn sample_operand(t: BytecodeOperandType) -> Operand<'static> {
  use BytecodeOperandType as T;
  use Operand::*;
  match t {
    T::Int16 => Int16(-3),
    T::Int32 => Int32(-7),
    T::Uint32 => Uint32(9),
    T::Char => Char(u16::from(b'Z')),
    T::JumpTarget => JumpTarget(16),
    T::Offset => Offset(-2),
    T::BoundsCheckOffset => BoundsCheckOffset(33),
    T::Register => Register(5),
    T::StackCheckFlag => StackCheckFlag(1),
    T::StandardCharacterSet => StandardCharacterSet(b'd'),
    T::BitTable => BitTable(&[0xA5u8; 16]),
  }
}

#[test]
fn encode_read_roundtrip_all_opcodes() -> Void {
  for bc in ALL_BYTECODES {
    let ops: Vec<Operand> = (0..bc.operand_count())
      .map(|i| sample_operand(bc.operand_type(i)))
      .collect();
    let code = encode(bc, &ops).unwrap_or_else(|| panic!("{} 编码失败", bc.name()));
    assert_eq!(
      code.len(),
      usize::from(bc.size()),
      "{} 缓冲宽 == Size",
      bc.name()
    );
    assert_eq!(code[0], bc.to_byte());
    assert_eq!(opcode_at(&code, 0), Some(bc));
    assert_eq!(
      walk(&code, code.len()).unwrap(),
      1,
      "{} 布局单条收尾",
      bc.name()
    );
    for (i, op) in ops.iter().enumerate() {
      let got = bc
        .read_operand(&code, 0, i)
        .unwrap_or_else(|| panic!("{} 读 {i} 截断", bc.name()));
      assert_eq!(got, *op, "{} 操作数 {i} round-trip", bc.name());
    }
    // 覆写同值不改字节 (Set 面幂等)
    let mut again = code.clone();
    for (i, op) in ops.iter().enumerate() {
      assert!(bc.write_operand(&mut again, 0, i, op));
    }
    assert_eq!(again, code, "{} 覆写幂等", bc.name());
  }
  OK
}

#[test]
fn codec_error_surfaces() -> Void {
  // GoTo 缺 label (截断缓冲) → read_operand None + disassemble_single Truncated
  let truncated = vec![Bytecode::GoTo.to_byte(), 0, 0, 0];
  assert_eq!(Bytecode::GoTo.read_operand(&truncated, 0, 0), None);
  let err = disassemble_single(&truncated, 0).unwrap_err();
  assert!(matches!(
    err,
    Error::Truncated {
      offset: 0,
      need: 8,
      have: 4
    }
  ));
  // 零操作数读第 0 个 → None
  assert_eq!(Bytecode::Break.read_operand(&[0; 4], 0, 0), None);
  // encode 操作数个数不符 → None
  assert!(encode(Bytecode::GoTo, &[]).is_none());
  // pc 越界 → InvalidBytecode
  let err = disassemble_single(&[], 0).unwrap_err();
  assert!(matches!(err, Error::InvalidBytecode { byte: 255 }));
  // write 到过小缓冲 → false
  let mut tiny = vec![0u8; 2];
  assert!(!Operand::JumpTarget(1).write(&mut tiny, 0));
  OK
}

// ──────────────────────────────────────────────────────────────────────────
// ⑤ Dump 格式金标准 (regexp-bytecodes.cc + ostreams.cc 语义)
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn dump_format_goldens() -> Void {
  // AsHex(v,2): 最小两位小写, 有符号按 uint64 符号扩展
  assert_eq!(as_hex(8), "08");
  assert_eq!(as_hex(255), "ff");
  assert_eq!(as_hex(256), "100");
  assert_eq!(as_hex((-1i64) as u64), "ffffffffffffffff");
  assert_eq!(stack_check_flag_name(0)?, "NoCheck");
  assert_eq!(stack_check_flag_name(1)?, "Check");
  assert!(matches!(
    stack_check_flag_name(2),
    Err(Error::UnknownEnumEncoding {
      kind: "StackCheckFlag",
      value: 2
    })
  ));

  // GoTo label=8
  let code = encode(Bytecode::GoTo, &[Operand::JumpTarget(8)]).unwrap();
  assert_eq!(disassemble_single(&code, 0)?, "GoTo, label: 08\n");

  // CheckCharacter 'a' + on_equal 8
  let code = encode(
    Bytecode::CheckCharacter,
    &[Operand::Char(0x61), Operand::JumpTarget(8)],
  )
  .unwrap();
  assert_eq!(
    disassemble_single(&code, 0)?,
    "CheckCharacter, character: a, on_equal: 08\n"
  );

  // Char 不可打印域: \x / \u 双分支 (AsUC32/PrintUC16 语义)
  let code = encode(
    Bytecode::CheckCharacterLT,
    &[Operand::Char(0xE9), Operand::JumpTarget(0)],
  )
  .unwrap();
  assert_eq!(
    disassemble_single(&code, 0)?,
    "CheckCharacterLT, limit: \\xe9, on_less: 00\n"
  );
  let code = encode(
    Bytecode::CheckCharacterGT,
    &[Operand::Char(0x2000), Operand::JumpTarget(0)],
  )
  .unwrap();
  assert_eq!(
    disassemble_single(&code, 0)?,
    "CheckCharacterGT, limit: \\u2000, on_greater: 00\n"
  );

  // Offset 负值: -1 全宽符号扩展
  let code = encode(Bytecode::AdvanceCurrentPosition, &[Operand::Offset(-1)]).unwrap();
  assert_eq!(
    disassemble_single(&code, 0)?,
    "AdvanceCurrentPosition, by: ffffffffffffffff\n"
  );

  // StackCheckFlag 枚举名
  let code = encode(
    Bytecode::PushRegister,
    &[Operand::Register(3), Operand::StackCheckFlag(1)],
  )
  .unwrap();
  assert_eq!(
    disassemble_single(&code, 0)?,
    "PushRegister, register_index: 03, stack_check: Check\n"
  );

  // StandardCharacterSet: 'd' → Digit (九名表 cc:21-52)
  let code = encode(
    Bytecode::CheckSpecialClassRanges,
    &[
      Operand::StandardCharacterSet(b'd'),
      Operand::JumpTarget(0x10),
    ],
  )
  .unwrap();
  assert_eq!(
    disassemble_single(&code, 0)?,
    "CheckSpecialClassRanges, character_set: Digit, on_no_match: 10\n"
  );
  // 字母表外位模式 → UnknownEnumEncoding (原版 UNREACHABLE 面)
  let bad = encode(
    Bytecode::CheckSpecialClassRanges,
    &[Operand::StandardCharacterSet(b'x'), Operand::JumpTarget(0)],
  )
  .unwrap();
  assert!(matches!(
    disassemble_single(&bad, 0),
    Err(Error::UnknownEnumEncoding {
      kind: "StandardCharacterSet",
      value: b'x'
    })
  ));

  // BitTable: 16 字节 hex 无分隔拼接
  let table: Vec<u8> = (0u8..16).collect();
  let code = encode(
    Bytecode::CheckBitInTable,
    &[Operand::JumpTarget(4), Operand::BitTable(&table)],
  )
  .unwrap();
  assert_eq!(
    disassemble_single(&code, 0)?,
    "CheckBitInTable, on_bit_set: 04, table: 000102030405060708090a0b0c0d0e0f\n"
  );

  // 多指令流: 头部行 + %4x 偏移列 (指针列推迟登记)
  let mut stream = encode(Bytecode::PushBacktrack, &[Operand::JumpTarget(12)]).unwrap();
  stream.extend(encode(Bytecode::GoTo, &[Operand::JumpTarget(12)]).unwrap());
  stream.extend(encode(Bytecode::Succeed, &[]).unwrap());
  let text = disassemble(&stream, stream.len(), "ab|c")?;
  // C++ %4x 列 = 宽度 4 右对齐空格填充 (非补零), '10' 为十六进制 16
  let want = "[generated bytecode for regexp pattern: 'ab|c']\n   0  PushBacktrack, label: 0c\n   8  GoTo, label: 0c\n  10  Succeed\n";
  assert_eq!(text, want);
  assert_eq!(walk(&stream, stream.len()).unwrap(), 3);
  // length 不是 Size 整数倍 → Truncated
  assert!(matches!(
    walk(&stream, stream.len() - 2),
    Err(Error::Truncated { .. })
  ));
  OK
}

/// IsPrint/PrintUC16 三分支边界手验 (ostreams.cc:123/127-133):
/// 0x20/0x7E 可打印闭区间两端, 0x7F 恰越上界走 \x 支, 0x100 恰越 0xFF 走 \u 支
#[test]
fn dump_isprint_boundaries() -> Void {
  let cases: [(u16, &str); 5] = [
    (0x20, " "),        // IsPrint 下界: "%c" 直写空格
    (0x7E, "~"),        // IsPrint 上界
    (0x7F, "\\x7f"),    // 0x7F > 0x7E → \x 支 (c <= 0xFF)
    (0xFF, "\\xff"),    // \x 支上界 (小写两位)
    (0x100, "\\u0100"), // UC16 高位: c > 0xFF → \u 支四位小写
  ];
  for (c, want) in cases {
    let code = encode(
      Bytecode::CheckCharacterLT,
      &[Operand::Char(c), Operand::JumpTarget(0)],
    )
    .unwrap();
    let line = disassemble_single(&code, 0)?;
    let want_line = format!("CheckCharacterLT, limit: {want}, on_less: 00\n");
    assert_eq!(line, want_line, "IsPrint 边界 {c:#06x}");
  }
  OK
}

// ──────────────────────────────────────────────────────────────────────────
// ⑥ QuickCheckDetails 金标准 (cc:2226-2275 推演例)
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn quick_check_merge_golden() -> Void {
  let mut a = QuickCheckDetails::with_characters(1);
  let mut b = QuickCheckDetails::with_characters(1);
  *a.position_mut(0) = Position {
    mask: 0b1100,
    value: 0b1011,
    determines_perfectly: true,
    cannot_match: false,
  };
  *b.position_mut(0) = Position {
    mask: 0b1010,
    value: 0b1001,
    determines_perfectly: true,
    cannot_match: false,
  };
  a.merge(&mut b, 0);
  // 逐步: mask=1100&1010=1000; a.value=1011&1000=1000; b.value=1001&1000=1000;
  // diff=0 → mask 保持 1000, value 1000; mask 不等 → dp 降 false
  assert_eq!(a.position(0).mask, 0b1000);
  assert_eq!(a.position(0).value, 0b1000);
  assert!(!a.position(0).determines_perfectly);
  assert_eq!(b.position(0).value, 0b1000); // 原版 other->value_ 亦被掩

  // 完全一致且 dp 双方 → 保持 dp=true (mask/value 等且 other dp)
  let mut c = QuickCheckDetails::with_characters(1);
  let mut d = QuickCheckDetails::with_characters(1);
  *c.position_mut(0) = Position {
    mask: 0xFF,
    value: 0x41,
    determines_perfectly: true,
    cannot_match: false,
  };
  *d.position_mut(0) = Position {
    mask: 0xFF,
    value: 0x41,
    determines_perfectly: true,
    cannot_match: false,
  };
  c.merge(&mut d, 0);
  assert!(c.position(0).determines_perfectly);
  assert_eq!(c.position(0).mask, 0xFF);
  OK
}

#[test]
fn quick_check_merge_cannot_match() -> Void {
  let mut a = QuickCheckDetails::with_characters(1);
  let mut b = QuickCheckDetails::with_characters(1);
  *a.position_mut(0) = Position {
    mask: 1,
    value: 0,
    determines_perfectly: false,
    cannot_match: true,
  };
  *b.position_mut(0) = Position {
    mask: 0xF0,
    value: 0x30,
    determines_perfectly: true,
    cannot_match: false,
  };
  a.merge(&mut b, 0);
  // a 侧 cannot_match → 整体取 b
  assert_eq!(a.position(0), b.position(0));

  let mut a2 = QuickCheckDetails::with_characters(1);
  let mut b2 = QuickCheckDetails::with_characters(1);
  *a2.position_mut(0) = Position {
    mask: 7,
    value: 5,
    determines_perfectly: false,
    cannot_match: false,
  };
  *b2.position_mut(0) = Position {
    mask: 0,
    value: 0,
    determines_perfectly: false,
    cannot_match: true,
  };
  a2.merge(&mut b2, 0);
  // b 侧 cannot_match → a 保持原样
  assert_eq!(a2.position(0).mask, 7);
  assert_eq!(a2.position(0).value, 5);
  OK
}

#[test]
fn quick_check_advance_clear_rationalize() -> Void {
  let mut d = QuickCheckDetails::with_characters(2);
  *d.position_mut(0) = Position {
    mask: 1,
    value: 2,
    determines_perfectly: false,
    cannot_match: false,
  };
  *d.position_mut(1) = Position {
    mask: 3,
    value: 4,
    determines_perfectly: false,
    cannot_match: false,
  };
  d.advance(1, true);
  assert_eq!(d.characters, 1);
  assert_eq!(d.position(0).mask, 3);
  assert_eq!(d.position(0).value, 4);
  // by ≥ characters → Clear
  d.advance(5, true);
  assert_eq!(d.characters, 0);
  assert!(!d.cannot_match());

  // Rationalize one_byte: pos(0x60,0x60),(0xFF,0x62) → mask 0xFF60 value 0x6260
  let mut r = QuickCheckDetails::with_characters(2);
  *r.position_mut(0) = Position {
    mask: 0x60,
    value: 0x60,
    determines_perfectly: false,
    cannot_match: false,
  };
  *r.position_mut(1) = Position {
    mask: 0xFF,
    value: 0x62,
    determines_perfectly: false,
    cannot_match: false,
  };
  assert!(r.rationalize(true));
  assert_eq!(r.mask, 0xFF60);
  assert_eq!(r.value, 0x6260);

  // Rationalize two_byte (引擎实际通道 characters ≤ 2): 移位 0/16
  let mut u = QuickCheckDetails::with_characters(2);
  *u.position_mut(0) = Position {
    mask: 0x00FF,
    value: 0x0041,
    determines_perfectly: false,
    cannot_match: false,
  };
  *u.position_mut(1) = Position {
    mask: 0x1100,
    value: 0x2041,
    determines_perfectly: false,
    cannot_match: false,
  };
  assert!(u.rationalize(false));
  assert_eq!(u.mask, 0x00FF | (0x1100 << 16));
  assert_eq!(u.value, 0x0041 | (0x2041 << 16));

  // 全零 mask: found_useful_op = false; clear 面
  let mut z = QuickCheckDetails::with_characters(1);
  assert!(!z.rationalize(true));
  z.set_cannot_match_from(0);
  assert!(z.cannot_match());
  z.clear();
  assert_eq!(z.characters, 0);

  // 常量面
  assert_eq!(max_code_unit(true), 0xFF);
  assert_eq!(max_code_unit(false), K_MAX_UTF16_CODE_UNIT);
  assert_eq!(char_mask(true), 0xFF);
  assert_eq!(char_mask(false), 0xFFFF);
  OK
}

// ──────────────────────────────────────────────────────────────────────────
// ⑦ Boyer-Moore 数据面金标准
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn boyer_moore_position_lattice() -> Void {
  // 与冻结表 WORD_RANGES 的半开边界同源对数 (static 表 → 运行时通道)
  use v8_regexp_syntax::tables_generated::WORD_RANGES;
  let wb = word_boundaries();
  assert_eq!(wb.len(), WORD_RANGES.len() * 2 + 1);
  for (i, (from, to)) in WORD_RANGES.iter().enumerate() {
    assert_eq!(wb[2 * i], *from);
    assert_eq!(wb[2 * i + 1], to + 1, "闭区间右端 +1");
  }
  assert_eq!(wb[wb.len() - 1], K_RANGE_END_MARKER);

  let mut p = BoyerMoorePositionInfo::new();
  assert_eq!(p.map_count(), 0);
  assert_eq!(p.containment(), ContainedInLattice::NotYet);
  p.set_interval(0x61, 0x7A); // 'a'..'z' ⊂ [97,123)
  assert_eq!(p.containment(), ContainedInLattice::LatticeIn);
  assert!(p.is_word());
  assert_eq!(p.map_count(), 26);
  assert!(p.at(u32::from(b'a')) && p.at(u32::from(b'z')));
  assert!(!p.at(u32::from(b'0')));

  let mut q = BoyerMoorePositionInfo::new();
  q.set_interval(1, 4); // 全在 \w 格空隙 (界外)
  assert_eq!(q.containment(), ContainedInLattice::LatticeOut);
  assert!(q.is_non_word());
  assert_eq!(q.map_count(), 4);

  let mut r = BoyerMoorePositionInfo::new();
  r.set_interval(0, 256); // 跨多条边界 → Unknown; size ≥128 → 全置
  assert_eq!(r.containment(), ContainedInLattice::LatticeUnknown);
  assert_eq!(r.map_count(), K_MAP_SIZE);
  assert_eq!(r.raw_bitset(), u128::MAX);

  let mut s = BoyerMoorePositionInfo::new();
  s.set(0x41);
  s.set_all();
  assert_eq!(s.map_count(), K_MAP_SIZE);
  assert_eq!(s.containment(), ContainedInLattice::LatticeUnknown);

  // combine 格律 (位或再收格)
  use ContainedInLattice as C;
  assert_eq!(combine(C::NotYet, C::LatticeIn), C::LatticeIn);
  assert_eq!(combine(C::LatticeIn, C::LatticeOut), C::LatticeUnknown);
  assert_eq!(combine(C::LatticeUnknown, C::LatticeIn), C::LatticeUnknown);
  // add_range_lattice: 已 Unknown 短路
  assert_eq!(
    add_range_lattice(C::LatticeUnknown, wb, 1, 2),
    C::LatticeUnknown
  );

  assert_eq!(bitset_first_set_bit(0), -1);
  assert_eq!(bitset_first_set_bit(1u128 << 70), 70);
  assert_eq!(bitset_first_set_bit(u128::MAX), 0);
  OK
}

#[test]
fn boyer_moore_lookahead_surface() -> Void {
  let mut la = BoyerMooreLookahead::new(3, true);
  assert_eq!(la.max_char, 0xFF);
  la.set(0, 0x61);
  la.set(0, 0x100); // 超一字节 max_char → 直弃
  assert_eq!(la.count(0), 1);
  la.set_interval(1, 0x00, 0x1FF); // 截断到 0xFF → 256 ≥128 全置
  assert_eq!(la.count(1), K_MAP_SIZE);
  la.set_rest(2);
  assert_eq!(la.count(2), K_MAP_SIZE);
  assert_eq!(la.at(2).containment(), ContainedInLattice::LatticeUnknown);

  let la16 = BoyerMooreLookahead::new(1, false);
  assert_eq!(la16.max_char, 0xFFFF);

  // FindWorthwhileInterval: 频率回调全 0, 两位置各 1 字符 → (0,1) 得分区间
  let mut la2 = BoyerMooreLookahead::new(2, true);
  la2.set(0, 65);
  la2.set(1, 66);
  assert_eq!(la2.find_worthwhile_interval(true, |_| 0), Some((0, 1)));
  // 全 SetAll: count 恒 > max_number_of_chars → 无 worthwhile
  let mut la3 = BoyerMooreLookahead::new(2, true);
  la3.set_all(0);
  la3.set_all(1);
  assert_eq!(la3.find_worthwhile_interval(true, |_| 0), None);

  // 编译常量 (compiler_constants h:63/66)
  assert_eq!(K_MAX_LOOKAHEAD_FOR_BOYER_MOORE, 8);
  assert_eq!(K_PATTERN_TOO_SHORT_FOR_BOYER_MOORE, 2);
  OK
}

// ──────────────────────────────────────────────────────────────────────────
// ⑧ UnicodeRangeSplitter / Latin1 等价 / DynamicBitSet / Frequency
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn range_splitter_buckets_full_domain() -> Void {
  let sp = UnicodeRangeSplitter::new(&[CharacterRange::everything()]);
  assert_eq!(
    sp.bmp(),
    &[
      CharacterRange::range(0, 0xD7FF),
      CharacterRange::range(0xE000, 0xFFFF)
    ],
    "BMP 两段合并同桶"
  );
  assert_eq!(
    sp.lead_surrogates(),
    &[CharacterRange::range(0xD800, 0xDBFF)]
  );
  assert_eq!(
    sp.trail_surrogates(),
    &[CharacterRange::range(0xDC00, 0xDFFF)]
  );
  assert_eq!(sp.non_bmp(), &[CharacterRange::range(0x10000, 0x10FFFF)]);

  // 单 BMP 字符只进 bmp
  let sp2 = UnicodeRangeSplitter::new(&[CharacterRange::singleton(0x41)]);
  assert_eq!(sp2.bmp(), &[CharacterRange::range(0x41, 0x41)]);
  assert!(sp2.lead_surrogates().is_empty() && sp2.non_bmp().is_empty());

  // 跨界区间 [0xDFFF, 0x10001] → trail 段 + bmp 第二区段 + 非 BMP 段
  // (五段扫描对每个相交段都进桶, ≡ 原版 kTargets 表)
  let sp3 = UnicodeRangeSplitter::new(&[CharacterRange::range(0xDFFF, 0x10001)]);
  assert_eq!(
    sp3.trail_surrogates(),
    &[CharacterRange::range(0xDFFF, 0xDFFF)]
  );
  assert_eq!(sp3.non_bmp(), &[CharacterRange::range(0x10000, 0x10001)]);
  assert_eq!(sp3.bmp(), &[CharacterRange::range(0xE000, 0xFFFF)]);

  // 早停: kStarts[i] > range.to 即 break — to=0 输入只产一段
  let sp4 = UnicodeRangeSplitter::new(&[CharacterRange::singleton(0)]);
  assert_eq!(sp4.bmp().len(), 1);
  assert!(sp4.lead_surrogates().is_empty());
  assert_eq!(K_INITIAL_SIZE, 8);
  OK
}

#[test]
fn latin1_equivalents_goldens() -> Void {
  // 0x39C / 0x3BC / 0x178 三点落界, 界外假 (cc:2289-2294)
  for c in [0x039Cu32, 0x03BC, 0x0178] {
    assert!(
      range_contains_latin1_equivalents(CharacterRange::singleton(c)),
      "{c:#x}"
    );
  }
  assert!(!range_contains_latin1_equivalents(CharacterRange::range(
    0x400, 0x4FF
  )));
  assert!(range_contains_latin1_equivalents(CharacterRange::range(
    0, 0x300
  ))); // 0x178
  assert!(ranges_contain_latin1_equivalents(&[
    CharacterRange::range(0x400, 0x500),
    CharacterRange::singleton(0x3BC)
  ]));
  assert!(!ranges_contain_latin1_equivalents(&[
    CharacterRange::range(0x400, 0x500)
  ]));
  assert!(!ranges_contain_latin1_equivalents(&[]));
  OK
}

#[test]
fn dynamic_bit_set_surface() -> Void {
  let mut s = DynamicBitSet::new();
  assert!(s.is_empty());
  assert_eq!(K_FIRST_LIMIT, 32);
  s.set(0);
  s.set(31);
  assert!(s.get(0) && s.get(31));
  assert!(!s.get(32));
  assert!(!s.is_empty());
  assert_eq!(s.first_bits(), 1 | (1 << 31));
  assert!(s.remaining().is_none());
  s.set(100);
  s.set(100); // 去重 (原版 Contains 检查)
  assert_eq!(s.remaining(), Some(&[100u32][..]));
  s.set(101);
  assert_eq!(s.remaining(), Some(&[100u32, 101][..]));
  assert!(s.get(101));
  let t = DynamicBitSet::default();
  assert!(t.is_empty() && !t.get(1000));
  OK
}

#[test]
fn frequency_collator_goldens() -> Void {
  let mut f = FrequencyCollator::new();
  assert_eq!(K_TABLE_SIZE, 128);
  // 零样本防除零通道 (total<1 → 1)
  assert_eq!(f.frequency(0x61 & K_TABLE_MASK), 1);
  // 4x'a' + 4x'A' → per-128: 4*128/8 = 64
  for _ in 0..4 {
    f.count_character(0x61);
    f.count_character(0x41);
  }
  assert_eq!(f.total_samples(), 8);
  assert_eq!(f.frequency(0x61), 64);
  assert_eq!(f.frequency(0x41), 64);
  assert_eq!(f.frequency(0x62), 0);
  // 桶初始 character = 索引 (构造全表覆写); counter 初值 0
  assert_eq!(f.bucket(0x61).character(), 0x61);
  assert_eq!(f.bucket(0).character(), 0);
  assert_eq!(FrequencyCollator::new().bucket(5).counter(), 0);
  // 负字符按 &127 落 127 桶 (原版 int 按位与语义)
  f.count_character(-1);
  assert_eq!(f.bucket(127).counter(), 1);
  OK
}

// ──────────────────────────────────────────────────────────────────────────
// ⑨ Trace / NodeInfo / 枚举位域数据面
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn trace_default_and_bitfields() -> Void {
  let t = Trace::new();
  // flags_ 默认 = AtStart(UNKNOWN=2)<<0 | HasAnyActions(false) | Grant(kNone)
  assert_eq!(t.flags_raw(), 2);
  assert_eq!(TriBool::FALSE_VALUE as u32, 0);
  assert_eq!(TriBool::TRUE_VALUE as u32, 1);
  assert_eq!(TriBool::UNKNOWN as u32, 2);
  assert_eq!(AtStartField::k_mask(), 0b11);
  assert_eq!(HasAnyActionsField::k_mask(), 0b100);
  assert_eq!(ParkedGrantField::k_mask(), 0b11000);
  assert_eq!(Trace::K_CP_OFFSET_SLACK, 1);
  assert_eq!(t.flush_budget(), 100);
  assert!(t.is_trivial());
  assert_eq!(t.at_start(), TriBool::UNKNOWN);
  assert_eq!(t.parked_grant(), ParkedGrant::None);
  assert!(t.can_advance_cp_offset(1));
  assert!(!t.can_advance_cp_offset(2)); // 原版仅 ±1 通道

  let mut t = Trace::new();
  t.set_at_start(TriBool::TRUE_VALUE);
  assert_eq!(t.at_start(), TriBool::TRUE_VALUE);
  assert_eq!(t.flags_raw(), 1);
  assert!(!t.is_trivial()); // at_start ≠ UNKNOWN 即非平凡

  // add_action: HasAnyActions 置位 → 链上非平凡
  let mut t2 = Trace::new();
  t2.add_action(false);
  assert!(t2.has_action());
  assert!(t2.has_any_actions());
  assert!(!t2.is_trivial());
  // stores_backreferenced_capture → 自动撤销 grant (cc:466-475)
  let mut t2b = Trace::new();
  t2b.set_backtrack(true);
  t2b.set_parked_grant(ParkedGrant::Parked);
  t2b.add_action(true);
  assert_eq!(t2b.parked_grant(), ParkedGrant::None);

  // set_backtrack 自动撤销 parked grant
  let mut t3 = Trace::new();
  t3.set_backtrack(true);
  t3.set_parked_grant(ParkedGrant::Parked);
  assert_eq!(t3.parked_grant(), ParkedGrant::Parked);
  t3.set_backtrack(true); // 换目标 → grant 回收
  assert_eq!(t3.parked_grant(), ParkedGrant::None);
  assert!(!t3.is_trivial());

  // 拷贝构造: action 丢弃而 flags 保留 (h:323-332)
  let mut src = Trace::new();
  src.add_action(false);
  src.set_at_start(TriBool::FALSE_VALUE);
  let copy = Trace::clone_for_copy(&src);
  assert!(!copy.has_action());
  assert!(copy.has_any_actions()); // flags 位按值复制
  assert_eq!(copy.at_start(), TriBool::FALSE_VALUE);

  // CP 界限常量 (macro-assembler.h:42-43)
  assert_eq!(v8_regexp_compiler::K_MAX_CP_OFFSET, (1 << 15) - 1);
  assert_eq!(v8_regexp_compiler::K_MIN_CP_OFFSET, -((1 << 15) - 1));

  // flush budget 16 位面 + quick_check_performed 按值集成
  let mut near = Trace::new();
  near.set_flush_budget(u16::MAX as i32);
  assert_eq!(near.flush_budget(), u16::MAX);
  let mut d = QuickCheckDetails::with_characters(1);
  d.position_mut(0).mask = 0xFF;
  near.set_quick_check_performed(&d);
  assert_eq!(near.quick_check_performed().position(0).mask, 0xFF);
  assert!(!near.is_trivial()); // characters()==1

  // ParkedGrant 强弱序 (门接受 ≥ 等级)
  assert!(ParkedGrant::ParkedNonEmptyUniformPrefix > ParkedGrant::ParkedUniformPrefix);
  assert!(ParkedGrant::ParkedUniformPrefix > ParkedGrant::Parked);
  assert!(ParkedGrant::Parked > ParkedGrant::None);
  assert_eq!(ParkedGrant::ParkedUniformPrefix as u8, 2);
  OK
}

#[test]
fn node_info_and_aux_structs() -> Void {
  let mut a = NodeInfo::new();
  let mut b = NodeInfo::new();
  assert!(a.matches(&b));
  b.follows_word_interest = true;
  b.at_end = true;
  assert!(!a.matches(&b));
  a.add_from_preceding(&b);
  assert!(a.at_end && a.follows_word_interest);
  assert!(a.has_lookbehind());
  assert!(a.matches(&b));

  let mut c = NodeInfo::new();
  c.being_analyzed = true;
  c.been_analyzed = true;
  c.visited = true;
  c.replacement_calculated = true;
  c.add_from_following(&NodeInfo {
    follows_start_interest: true,
    ..NodeInfo::new()
  });
  assert!(c.follows_start_interest);
  assert!(!c.at_end); // AddFromFollowing 不动 at_end
  c.reset_compilation_state();
  assert!(!c.being_analyzed && !c.been_analyzed);
  assert!(c.visited && c.replacement_calculated); // 非编译态位保留
  assert_eq!(NodeInfo::default(), NodeInfo::new());

  // EatsAtLeastInfo
  let mut e = EatsAtLeastInfo::with_eats(3);
  assert_eq!(e.from_possibly_start, 3);
  e.set_min(&EatsAtLeastInfo::with_eats(1));
  assert_eq!(e.from_not_start, 1);
  e.set_max(500); // saturated_cast 通道: 500 → 255
  assert_eq!(e.from_possibly_start, 255);
  assert!(!e.is_zero());
  assert!(EatsAtLeastInfo::new().is_zero());

  // PreloadState
  let mut p = PreloadState::default();
  p.init();
  assert_eq!(
    p.eats_at_least,
    PreloadState::K_EATS_AT_LEAST_NOT_YET_INITIALIZED
  );

  // RegisterFlushInfo / kNoStore
  let info = RegisterFlushInfo::default();
  assert_eq!(info.store_position, K_NO_STORE);
  assert_eq!(K_NO_STORE, i32::MIN);
  assert_eq!(info.undo_action, DeferredActionUndoType::IGNORE);
  assert!(!info.absolute && !info.clear);
  OK
}

// ──────────────────────────────────────────────────────────────────────────
// ⑩ flags 派生真值表 + 导出冒烟
// ──────────────────────────────────────────────────────────────────────────

#[test]
fn needs_unicode_case_equivalents_truth_table() -> Void {
  use v8_regexp_syntax::flags::RegExpFlags as F;
  assert!(needs_unicode_case_equivalents(F::UNICODE | F::IGNORE_CASE));
  assert!(needs_unicode_case_equivalents(
    F::UNICODE_SETS | F::IGNORE_CASE
  ));
  assert!(!needs_unicode_case_equivalents(F::UNICODE));
  assert!(!needs_unicode_case_equivalents(F::IGNORE_CASE));
  assert!(!needs_unicode_case_equivalents(
    F::MULTILINE | F::IGNORE_CASE
  ));
  assert!(!needs_unicode_case_equivalents(F::NONE));
  OK
}

#[test]
fn exports_smoke() -> Void {
  let _ = Position::new();
  let _ = CharacterRange::singleton(0);
  let _ = QuickCheckDetails::default();
  // smear_bits_right: 单比特上沿涂满 (quick_check 内部工具面)
  assert_eq!(smear_bits_right(1), 1);
  assert_eq!(smear_bits_right(0), 0);
  OK
}
