//! IrregexpInterpreter 骨架锚点测试 (REG-1b1)。
//!
//! 规约映射 (实测 v8/test/unittests/regexp/ 目录: regexp-unittest.cc 的
//! "Tests of interpreter" 区 :657 起全部 Isolate 硬耦合 — ContextInitializer/
//! CreateJSRegExp/ExecuteForTesting native 路径, MatchInternal 直接调用仅
//! :1438/:1459 且需完整 Isolate; 无纯函数用例可迁):
//! 按票规约 3 以骨架锚点兜底, 断言面:
//! - T1 返回值协议逐值 (h:21-27 + regexp.h:150-155; CallOrigin regexp.h:112-113)
//! - T2 opcode 计数/编号表 (regexp-bytecodes.h:336-373; 与 regexp-interpreter
//!   .cc.js 对数 N=61 同源) + from_ptr/from_byte/is_valid_jump_target 边界
//! - T3 BacktrackStack sp 语义 (cc:129-165): sp=首个空槽下标 / push 预算 bool /
//!   set_sp 截断
//! - T4 InterpreterRegisters 布局 (cc:167-213): kNoMatchValue 初值 / 索引读写 /
//!   CopyToOutputRegisters 定长拷贝 / 寄存器↔字节偏移换算表 / 捕获对索引接驳
//!   regexp_utils::capture_start/end_index (REG-1a)
//! - T5 IndexIsInBounds 换算表 (cc:329-332)
//! - T6 CheckBitInTable 位图窗口 (cc:321-326, macro-assembler.h:48-50)
//! - T7 Load2/Load4Characters + BitsPerChar (cc:105-122)
//! - T8 CheckSpecialClassRanges 逐集判定表 (cc:418-497, 一/两字节模式)
//! - T9 BackRefMatchesNoCase Latin1 折小写 (cc:63-81, 含 ÷/× 非字母锚)
//! - T10 ADVANCE/SET_PC/位置变更寻址纯函数 (cc:367-381)
//!
//! REG-1b2 面 (操作数解码基座 + RawMatch 派发主体 + P-3a 23 handler):
//! - T11 61 项指令宽度 golden (= 原版 detail::kBytecodeSizes, inl.h:312-316)
//! - T12 P-3a + 排布难点项的 操作数个数/偏移/类型 golden (inl.h:107-136)
//! - T13 操作数读取器 Get<op> 往返 + bytecode_operands 下标常量全等 (inl.h:244-273)
//! - T14 OperandType 宽度/对齐/is_basic 表 + 限幅常量 (bytecodes.h:30-51)
//! - T15..T37 P-3a 23 handler 逐名手工汇编向量 (cc:577-878, 见各 fn 名)
//! - T38 栈溢出预算 (PushBacktrack cc:587-590 于 BacktrackStack::K_MAX_SIZE 越限)
//! - T39 Backtrack 的 return_code 协议五值 + 域外值 (cc:645-648)
//! - T40 中断注入传播 (HandleInterrupts cc:650-652) / 非法码流 (帧截断/跳转越界)
//! - T41 P-3b/c/d 38 项未落地分支逐名逐原版行号逐批段台账
//! - T42 AdvanceCpAndGoto 一/两字节两态等价 (票面差异 ③)

use aok::{OK, Void};
use log::info;
use v8_regexp_exec::{
  regexp_interpreter::{
    BacktrackStack, Bytecode, CallOrigin, InterpreterRegisters, K_BIT_TABLE_BYTES,
    K_BYTECODE_COUNT, K_BYTECODE_MASK, K_MAX_BOUNDS_CHECK_OFFSET, K_MAX_CP_OFFSET,
    K_MAX_OPERAND_COUNT, K_MAX_REGISTER, K_MAX_REGISTER_COUNT, K_MIN_CP_OFFSET,
    K_NO_BACKTRACK_LIMIT, K_OPERAND_TYPE_COUNT, K_PADDED_BYTECODE_COUNT, OperandType, RawMatch,
    RawMatchParams, Result, Step, Subject, TraceHooks, advance_current_position, advance_pc,
    back_ref_matches_no_case_latin1, bits_per_char_one_byte, bits_per_char_two_byte,
    bytecode_operands, bytecode_size, check_bit_in_table, check_special_class_ranges,
    index_is_in_bounds, load_2_characters_one_byte, load_2_characters_two_byte,
    load_4_characters_one_byte, operand_bit_table, operand_count, operand_i16, operand_i32,
    operand_offset, operand_size, operand_type, operand_u8, operand_u16, operand_u32,
    pc_from_offset, set_current_position,
  },
  regexp_stack::K_MAXIMUM_STACK_SIZE,
  regexp_utils::{capture_end_index, capture_start_index},
};
use v8_regexp_syntax::StandardCharacterSet as Scs;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// T1 返回值协议逐值 (h:21-27; regexp.h:112-113/150-155)
#[test]
fn result_protocol_values() -> Void {
  // Result 五值 = RegExp::kInternalRegExp* (regexp.h:150-154)
  assert_eq!(Result::Failure as i32, 0); // kInternalRegExpFailure
  assert_eq!(Result::Success as i32, 1); // kInternalRegExpSuccess
  assert_eq!(Result::Exception as i32, -1); // kInternalRegExpException
  assert_eq!(Result::Retry as i32, -2); // kInternalRegExpRetry
  assert_eq!(Result::FallbackToExperimental as i32, -3); // kInternalRegExpFallback
  // 协议下界 (regexp.h:155 kInternalRegExpSmallestResult)
  assert_eq!(Result::SMALLEST, Result::FallbackToExperimental as i32);
  // CallOrigin (regexp.h:112-113)
  assert_eq!(CallOrigin::FromRuntime as i32, 0);
  assert_eq!(CallOrigin::FromJs as i32, 1);
  info!("T1 ok");
  OK
}

/// T2 opcode 计数与编号 (regexp-bytecodes.h:90-373; 61 = 脚本对数 N)
#[test]
fn bytecode_numbering_anchors() -> Void {
  // 四表计数: INVALID 1 (h:90) + BASIC 35 (h:93-186) + SPECIAL 18 (h:188-243)
  // + PEEPHOLE 7 (h:245-328) = 61 — 与 cc 端 BYTECODE handler 对数一致
  assert_eq!(K_BYTECODE_COUNT, 61);
  // kBreak 必须为 0 (h:87 注释: 跳 0 视为非法)
  assert_eq!(Bytecode::Break as u8, 0);
  // 组界编号锚
  assert_eq!(Bytecode::PushCurrentPosition as u8, 1); // BASIC 首项
  assert_eq!(Bytecode::SetCurrentPositionFromEnd as u8, 35); // BASIC 末项
  assert_eq!(Bytecode::Backtrack as u8, 36); // SPECIAL 首项
  assert_eq!(Bytecode::CheckNotBackRefNoCaseUnicodeBackward as u8, 53); // SPECIAL 末
  assert_eq!(Bytecode::SkipUntilBitInTable as u8, 54); // PEEPHOLE 首
  assert_eq!(Bytecode::SkipUntilOneOfMasked3 as u8, 60); // kLast (h:341-343)
  // 掩码/填充 (cc:517-520): bit_ceil(61)=64, mask=63
  assert_eq!(K_PADDED_BYTECODE_COUNT, 64);
  assert_eq!(K_BYTECODE_MASK, 63);
  // IsValid/FromByte (h:359-371): 0..=60 有效, 61..=63 (掩码域内) 无效
  for byte in 0..=K_BYTECODE_COUNT - 1 {
    assert!(Bytecode::from_byte(byte).is_some(), "byte {byte} 应有效");
  }
  for byte in K_BYTECODE_COUNT..=u8::try_from(K_BYTECODE_MASK).unwrap_or(u8::MAX) {
    assert!(Bytecode::from_byte(byte).is_none(), "byte {byte} 应无效");
    assert!(!Bytecode::is_valid(byte));
  }
  assert!(Bytecode::is_valid(0) && Bytecode::is_valid(60));
  // IsValidJumpTarget (h:372-374): kBreak(0) 非合法跳转目标
  assert!(!Bytecode::is_valid_jump_target(0));
  assert!(Bytecode::is_valid_jump_target(1));
  assert!(Bytecode::is_valid_jump_target(60));
  assert!(!Bytecode::is_valid_jump_target(61));
  // ToByte 往返 (h:356-358)
  for bc in [
    Bytecode::Break,
    Bytecode::GoTo,
    Bytecode::Backtrack,
    Bytecode::SkipUntilOneOfMasked3,
  ] {
    assert_eq!(Bytecode::from_byte(bc.to_byte()), Some(bc));
  }
  // FromPtr (h:365-370): 读 pc 处 opcode 字节
  let buf = [Bytecode::GoTo.to_byte(), 0, 0, 0];
  let got = unsafe { Bytecode::from_ptr(buf.as_ptr()) };
  assert_eq!(got, Some(Bytecode::GoTo));
  let illegal = [200u8];
  assert_eq!(unsafe { Bytecode::from_ptr(illegal.as_ptr()) }, None);
  info!("T2 ok");
  OK
}

/// T3 回溯栈 (cc:129-165)
#[test]
fn backtrack_stack_semantics() -> Void {
  // kMaxSize (cc:164) = kMaximumStackSize/sizeof(int) = 64MB/4
  assert_eq!(BacktrackStack::K_MAX_SIZE, K_MAXIMUM_STACK_SIZE / 4);
  assert_eq!(BacktrackStack::K_STATIC_CAPACITY, 64); // cc:159

  let mut s = BacktrackStack::new();
  // sp = 首个空槽下标 (cc:149-150)
  assert_eq!(s.sp(), 0);
  // push (cc:135-138): 预算内 true
  assert!(s.push(11));
  assert!(s.push(22));
  assert_eq!(s.sp(), 2);
  // peek 不动栈 (cc:139-142)
  assert_eq!(s.peek(), 22);
  assert_eq!(s.sp(), 2);
  // pop = peek+出栈 (cc:143-147)
  assert_eq!(s.pop(), 22);
  assert_eq!(s.sp(), 1);
  assert_eq!(s.pop(), 11);
  assert_eq!(s.sp(), 0);
  // 混合压弹: pc 偏移与整数混存 (cc:126-128 通用栈语义)
  assert!(s.push(0)); // pc 偏移
  assert!(s.push(-1)); // kNoMatchValue 形态整数
  assert_eq!(s.peek(), -1);
  // set_sp (cc:151-154): 截断回退, 只缩不涨
  assert!(s.push(77));
  assert_eq!(s.sp(), 3);
  s.set_sp(1);
  assert_eq!(s.sp(), 1);
  assert_eq!(s.peek(), 0);
  s.set_sp(1); // 幂等截断
  assert_eq!(s.sp(), 1);
  info!("T3 ok");
  OK
}

/// T4 寄存器组 (cc:167-213) + REG-1a 接驳 (捕获对索引公式)
#[test]
fn interpreter_registers_layout() -> Void {
  // 常量面 (macro-assembler.h:37-38)
  assert_eq!(K_MAX_REGISTER_COUNT, 1 << 16);
  assert_eq!(K_MAX_REGISTER, 65535);
  assert_eq!(InterpreterRegisters::K_NO_MATCH_VALUE, -1); // cc:175

  // total=8, output=4 (两捕获对 + 4 内部寄存器), 满足 cc:186-188 前提
  let mut r = InterpreterRegisters::new(8, 4);
  // 初值全 kNoMatchValue (cc:179)
  for i in 0..8 {
    assert_eq!(r.get(i), -1, "寄存器 {i} 初值");
  }
  // SetRegister/读写 (cc:197-200 语义)
  r.set(0, 3);
  r.set(1, 9);
  r.set(5, 42); // 内部寄存器
  assert_eq!(r.get(0), 3);
  assert_eq!(r.get(5), 42);
  // CopyToOutputRegisters (cc:202-205): 只拷 output_register_count=4 槽
  let mut out = [123i32; 6];
  r.copy_to_output_registers(&mut out);
  assert_eq!(&out[..4], &[3, 9, -1, -1]);
  assert_eq!(
    &out[4..],
    &[123, 123],
    "输出区之外不写 (cc:203 定长 MemCopy)"
  );
  assert_eq!(r.output_register_count(), 4);
  // 寄存器↔字节偏移换算表 (RegisterT=int32, cc:174/183-185)
  assert_eq!(InterpreterRegisters::register_byte_offset(0), 0);
  assert_eq!(InterpreterRegisters::register_byte_offset(1), 4);
  assert_eq!(InterpreterRegisters::register_byte_offset(7), 28);
  // REG-1a 接驳: 捕获 i 的 {start,end} = 索引 2i/2i+1 (cc:169-171)
  for capture in 0..2 {
    let start = usize::try_from(capture_start_index(capture)).unwrap_or(usize::MAX);
    let end = usize::try_from(capture_end_index(capture)).unwrap_or(usize::MAX);
    assert_eq!(
      (start, end),
      (usize::try_from(capture * 2).unwrap(), start + 1)
    );
    r.set(start, 10 + capture);
    r.set(end, 20 + capture);
    assert_eq!(r.get(end) - r.get(start), 10);
  }
  info!("T4 ok");
  OK
}

/// T5 IndexIsInBounds (cc:329-332) 回绕换算表
#[test]
fn index_is_in_bounds_table() -> Void {
  assert!(index_is_in_bounds(0, 5));
  assert!(index_is_in_bounds(4, 5));
  assert!(!index_is_in_bounds(5, 5)); // 上界闭
  assert!(!index_is_in_bounds(-1, 5)); // 负 index 回绕 → 巨值
  assert!(!index_is_in_bounds(i32::MIN, 5));
  assert!(!index_is_in_bounds(0, 0));
  assert!(!index_is_in_bounds(1, 0));
  info!("T5 ok");
  OK
}

/// T6 CheckBitInTable (cc:321-326) 位图窗口换算 (16 字节表 = 128 字符)
#[test]
fn check_bit_in_table_window() -> Void {
  let mut table = [0u8; 16]; // SPECIAL kBitTable 定长 16 (bytecodes.h:43)
  table[0] |= 1 << 5; // char 5
  table[1] |= 1; // char 8 → 字节 (8&127)>>3=1, 位 8&7=0
  table[15] |= 1 << 7; // char 127 → 字节 15, 位 7
  assert!(check_bit_in_table(5, &table));
  assert!(!check_bit_in_table(6, &table));
  // 掩码窗口回绕: 133 & 127 = 5 → 同槽同位 (原版 (char&mask)>>log2 语义)
  assert!(check_bit_in_table(133, &table));
  assert!(check_bit_in_table(8, &table));
  assert!(check_bit_in_table(127, &table));
  // 高位截断: 255 & 127 = 127 → 命中 (表仅覆盖 128 窗口, 原版即掩码寻址)
  assert!(check_bit_in_table(255, &table));
  assert!(!check_bit_in_table(254, &table)); // 254&127=126 → 字节15位6 未置
  // 表短防御: 空表恒 false (原版裸指针 UB, Rust get 承接)
  assert!(!check_bit_in_table(5, &[]));
  info!("T6 ok");
  OK
}

/// T7 Load2/4Characters + BitsPerChar (cc:105-122)
#[test]
fn load_characters_assembly() -> Void {
  // 位宽 (cc:105-108): u8→8, u16→16
  assert_eq!(bits_per_char_one_byte(), 8);
  assert_eq!(bits_per_char_two_byte(), 16);
  // Load2Characters (cc:110-113)
  let s8 = [0x41u8, 0x42, 0x43];
  assert_eq!(load_2_characters_one_byte(&s8, 0), 0x41 | (0x42 << 8));
  assert_eq!(load_2_characters_one_byte(&s8, 1), 0x42 | (0x43 << 8));
  let s16 = [0x0041u16, 0x0042, 0xFFFE];
  assert_eq!(load_2_characters_two_byte(&s16, 0), 0x0041 | (0x0042 << 16));
  assert_eq!(
    load_2_characters_two_byte(&s16, 1),
    0x0042 | (0xFFFEu32 << 16)
  );
  // Load4Characters<uint8_t> (cc:115-118): 定宽 8/16/24
  let s4 = [0x61u8, 0x62, 0x63, 0x64, 0x65];
  assert_eq!(
    load_4_characters_one_byte(&s4, 0),
    0x61 | (0x62 << 8) | (0x63 << 16) | (0x64 << 24)
  );
  assert_eq!(
    load_4_characters_one_byte(&s4, 1),
    0x62 | (0x63 << 8) | (0x64 << 16) | (0x65 << 24)
  );
  info!("T7 ok");
  OK
}

/// T8 CheckSpecialClassRanges (cc:418-497) 逐集判定表 (词表参数注入)
#[test]
fn special_class_ranges_tables() -> Void {
  // 词字符表 (macro-assembler.h:435 256 项): 构造最小可用表, map[0]=0 (cc:441)
  let mut word_map = [0u8; 256];
  for c in b'a'..=b'z' {
    word_map[usize::from(c)] = 1;
  }
  for c in b'A'..=b'Z' {
    word_map[usize::from(c)] = 1;
  }
  for c in b'0'..=b'9' {
    word_map[usize::from(c)] = 1;
  }
  word_map[usize::from(b'_')] = 1;

  // kWhitespace (cc:423-429, 一字节模式): ' ' / \t..\r / 0xA0
  let ws = |c: u32| check_special_class_ranges::<true>(c, Scs::Whitespace, &word_map);
  assert!(ws(' ' as u32));
  for c in 0x09..=0x0D {
    assert!(ws(c), "0x{c:x} 应空白");
  }
  assert!(ws(0xA0));
  assert!(!ws('x' as u32));
  assert!(!ws(0));

  // kDigit / kNotDigit (cc:457-466)
  let digit = |c: u32| check_special_class_ranges::<true>(c, Scs::Digit, &word_map);
  let not_digit = |c: u32| check_special_class_ranges::<true>(c, Scs::NotDigit, &word_map);
  for c in 0x30..=0x39 {
    assert!(digit(c) && !not_digit(c));
  }
  assert!(!digit('a' as u32) && not_digit('a' as u32));

  // kWord / kNotWord (cc:432-456): 一字节走表; 两字节 > 'z' 短路 (cc:433-437/446-450)
  let word = |c: u32, one: bool| {
    if one {
      check_special_class_ranges::<true>(c, Scs::Word, &word_map)
    } else {
      check_special_class_ranges::<false>(c, Scs::Word, &word_map)
    }
  };
  let not_word = |c: u32, one: bool| {
    if one {
      check_special_class_ranges::<true>(c, Scs::NotWord, &word_map)
    } else {
      check_special_class_ranges::<false>(c, Scs::NotWord, &word_map)
    }
  };
  assert!(word('a' as u32, true) && word('_' as u32, true));
  assert!(!word(' ' as u32, true) && !word(0, true));
  assert!(not_word(' ' as u32, true));
  assert!(!word(0x7B, false), "两字节 > 'z' 短路 false (cc:434-436)");
  assert!(not_word(0x7B, false), "两字节 > 'z' 短路 true (cc:447-449)");
  assert!(word('w' as u32, false), "0x77 不短路走表");

  // kLineTerminator / kNotLineTerminator (cc:467-492): \n\r 共有,
  // 0x2028/0x2029 仅两字节模式 (cc:471-473)
  let lt = |c: u32, one: bool| {
    if one {
      check_special_class_ranges::<true>(c, Scs::LineTerminator, &word_map)
    } else {
      check_special_class_ranges::<false>(c, Scs::LineTerminator, &word_map)
    }
  };
  let nlt = |c: u32, one: bool| {
    if one {
      check_special_class_ranges::<true>(c, Scs::NotLineTerminator, &word_map)
    } else {
      check_special_class_ranges::<false>(c, Scs::NotLineTerminator, &word_map)
    }
  };
  assert!(lt(0x0A, true) && lt(0x0D, true) && lt(0x0A, false));
  assert!(
    !lt(0x2028, true),
    "一字节模式不含 U+2028 (cc:471 constexpr 门)"
  );
  assert!(lt(0x2028, false) && lt(0x2029, false));
  assert!(!nlt(0x2028, false) && nlt(0x2028, true));
  assert!(!lt(0x20, true) && nlt(0x20, true));

  // kEverything (cc:493-494)
  assert!(check_special_class_ranges::<true>(
    0,
    Scs::Everything,
    &word_map
  ));
  assert!(check_special_class_ranges::<false>(
    0x10FFFF,
    Scs::Everything,
    &word_map
  ));
  info!("T8 ok");
  OK
}

/// T9 反向引用 Latin1 折小写 (cc:63-81)
#[test]
fn backref_no_case_latin1() -> Void {
  // "AbC" vs "aBc": 全等折小写 → true
  let s = *b"AbCaBc";
  assert!(back_ref_matches_no_case_latin1(0, 3, 3, &s));
  // 单字符错位 → false
  let s2 = *b"AbCxBc";
  assert!(!back_ref_matches_no_case_latin1(0, 3, 3, &s2));
  // 零长恒 true (cc:66 空循环)
  assert!(back_ref_matches_no_case_latin1(2, 2, 0, &s));
  // Latin-1 字母区折叠: 0xE0(à) vs 0xC0(À) → |=0x20 后 224==224, 且
  // 224-224=0 ≤ 30 ≠ 247 → 继续 (cc:74-78)
  let s3 = [0xE0u8, 0xC0];
  assert!(back_ref_matches_no_case_latin1(0, 1, 1, &s3));
  // ÷(0xF7)/×(0xD7): 215|0x20=247 折等, 但 old=247 命中 ≠247 排除 → false
  let s4 = [0xF7u8, 0xD7];
  assert!(!back_ref_matches_no_case_latin1(0, 1, 1, &s4));
  info!("T9 ok");
  OK
}

/// T9R 减法下流回归 (审计 D2): 原版 cc:75-76 为 unsigned 回绕语义。|0x20 折等
/// 但折后落入非字母窗 ({/[→0x7B, \x01/!→0x21) 须经回绕判 false; 直减在 debug 构建会下流 panic
#[test]
fn backref_no_case_latin1_underflow_regression() -> Void {
  // '{'(0x7B) vs '['(0x5B): 折后同 0x7B; 0x7B-'a'=26>25 落第二判, 0x7B-224 回绕→>30 → false
  let s = [0x7Bu8, 0x5B];
  assert!(!back_ref_matches_no_case_latin1(0, 1, 1, &s));
  // \x01 vs '!'(0x21): 折后同 0x21; 0x21-'a' 回绕→>25, 0x21-224 回绕→>30 → false
  let s2 = [0x01u8, 0x21];
  assert!(!back_ref_matches_no_case_latin1(0, 1, 1, &s2));
  info!("T9R ok");
  OK
}

/// T10 派发寻址纯函数 (cc:367-381)
#[test]
fn dispatch_addressing() -> Void {
  // ADVANCE (cc:367-369): pc + Size(current_bc) — 定长 4 字节 opcode 语义锚
  assert_eq!(advance_pc(0, 4), 4);
  assert_eq!(advance_pc(8, 8), 16);
  assert_eq!(advance_pc(60, 4), 64);
  // SET_PC_FROM_OFFSET (cc:371-373): 偏移即绝对 pc (code_base 相对寻址)
  assert_eq!(pc_from_offset(0), 0);
  assert_eq!(pc_from_offset(128), 128);
  // SET/ADVANCE_CURRENT_POSITION (cc:376-381): 范围 DCHECK 语义
  assert_eq!(set_current_position(5, 10), 5);
  assert_eq!(set_current_position(10, 10), 10); // 上界闭 (IsInRange 0..=len)
  assert_eq!(advance_current_position(3, 2, 10), 5);
  assert_eq!(advance_current_position(8, 2, 10), 10);
  assert_eq!(advance_current_position(0, 0, 0), 0);
  info!("T10 ok");
  OK
}

// ══ REG-1b2: 操作数解码基座 + RawMatch 派发主体 + P-3a 23 handler ══════════

/// 61 项指令宽度 golden — 原版 detail::kBytecodeSizes (regexp-bytecodes-inl.h:312-316),
/// 逐项 = 各 BytecodeOperands<bc>::kSize (inl.h:149-151)。Rust 侧 BYTECODE_LAYOUTS
/// 为由 operand_layout 算法派生 (非硬编码), 本表为独立 golden; 脚本
/// (tool/src/regexp/regexp-interpreter.cc.js) 从 bytecodes.h V-表重算同值三方对齐。
const K_GOLDEN_SIZES: [u8; 61] = [
  // 0..=9: Break, PushCurrentPosition, PushBacktrack, WriteCPToReg, ReadCPFromReg,
  //        WriteSPToReg, ReadSPFromReg, SetRegister, ClearRegisters, AdvanceRegister
  4, 4, 8, 8, 4, 4, 4, 8, 8, 8,
  // 10..=19: PopCurrentPosition, PushRegister, PopRegister, Fail, Succeed,
  //          AdvanceCurrentPosition, GoTo, LoadCurrentCharacter, CheckPosition,
  //          CheckSpecialClassRanges
  4, 8, 4, 4, 4, 4, 8, 12, 8, 8,
  // 20..=29: CheckCharacter, CheckNotCharacter, CheckCharacterAfterAnd,
  //          CheckNotCharacterAfterAnd, CheckNotCharacterAfterMinusAnd,
  //          CheckCharacterInRange, CheckCharacterNotInRange, CheckCharacterLT,
  //          CheckCharacterGT, IfRegisterLT
  8, 8, 12, 12, 12, 12, 12, 8, 8, 12,
  // 30..=39: IfRegisterGE, IfRegisterEqPos, CheckAtStart, CheckNotAtStart,
  //          CheckFixedLengthLoop, SetCurrentPositionFromEnd, Backtrack,
  //          LoadCurrentCharacterUnchecked, CheckBitInTable, Load2CurrentChars
  12, 8, 8, 8, 8, 4, 4, 4, 24, 12,
  // 40..=49: Load2CurrentCharsUnchecked, Load4CurrentChars, Load4CurrentCharsUnchecked,
  //          Check4Chars, CheckNot4Chars, AndCheck4Chars, AndCheckNot4Chars,
  //          AdvanceCpAndGoto, CheckNotBackRef, CheckNotBackRefNoCase
  4, 12, 4, 12, 12, 16, 16, 8, 8, 8,
  // 50..=60: CheckNotBackRefNoCaseUnicode, CheckNotBackRefBackward,
  //          CheckNotBackRefNoCaseBackward, CheckNotBackRefNoCaseUnicodeBackward,
  //          SkipUntilBitInTable, SkipUntilCharAnd, SkipUntilChar, SkipUntilCharOrChar,
  //          SkipUntilGtOrNotBitInTable, SkipUntilOneOfMasked, SkipUntilOneOfMasked3
  8, 8, 8, 8, 36, 24, 20, 24, 36, 48, 92,
];

/// 手工汇编器 (REG-1b2 测试侧): 按 BYTECODE_LAYOUTS 的 operand_offset/operand_size
/// 编码指令帧 — 与解释器解码侧共用同一张布局表, 故帧宽 ≡ 原版 Bytecodes::Size。
/// 断言面: ops 个数与每项宽度均须与布局表全等 (布局表若有偏差即在汇编期爆红)。
#[derive(Default)]
struct Asm(Vec<u8>);

impl Asm {
  /// 追加一条指令, 返回帧起始偏移 (= 原版 label 的 code_base 相对偏移)
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

  /// 跳转目标二次定值 (原版宏汇编器 label 绑定面)
  fn patch(&mut self, at: usize, bc: Bytecode, op: usize, bytes: &[u8]) {
    let width = usize::from(operand_size(bc, op));
    assert_eq!(bytes.len(), width, "{bc:?} op{op} 宽度");
    let off = at + operand_offset(bc, op);
    self.0[off..off + width].copy_from_slice(bytes);
  }

  fn code(&self) -> &[u8] {
    &self.0
  }
}

/// 操作数编码字面量 (宿主字节序, ≡ 原版 reinterpret_cast 直读)
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

/// 零填操作数占位 (未落地分支的测试只需完整帧, 不需有效值)
fn zero_ops(bc: Bytecode) -> Vec<Vec<u8>> {
  (0..operand_count(bc))
    .map(|i| vec![0u8; usize::from(operand_size(bc, i))])
    .collect()
}

/// 两态 subject 共码 (长 5): Latin1 / UC16 — 原版 base::Vector<const Char> cc:504
const SUBJ8: &[u8] = b"abcde";
const SUBJ16: &[u16] = &[0x61, 0x62, 0x63, 0x64, 0x65];

/// 单指令面参数: 6 寄存器 (4 输出 + 2 内部) / 步数上限 1 (恰执行被测指令)
fn params(current: i32) -> RawMatchParams {
  RawMatchParams {
    total_register_count: 6,
    current,
    step_limit: Some(1),
    ..Default::default()
  }
}

/// 整程参数 (无步数上限): 多指令码流跑到 Succeed/Fail 出口
fn free(current: i32) -> RawMatchParams {
  RawMatchParams {
    total_register_count: 6,
    current,
    ..Default::default()
  }
}

/// Latin1 态机器构造 (output 4 槽 = 原版整体匹配 + 1 捕获的最小形态)
fn mach<'a>(
  code: &'a [u8],
  out: &'a mut [i32],
  p: RawMatchParams,
) -> RawMatch<'a, &'static [u8], TraceHooks> {
  RawMatch::new(code, SUBJ8, out, p, TraceHooks::default())
}

/// T11 指令宽度 golden 表 (inl.h:149-151/312-316) + kBytecodeAlignment 不变式
#[test]
fn bytecode_size_golden_table() -> Void {
  assert_eq!(K_GOLDEN_SIZES.len(), usize::from(K_BYTECODE_COUNT));
  for (i, want) in K_GOLDEN_SIZES.iter().enumerate() {
    let byte = u8::try_from(i).unwrap_or(u8::MAX);
    let bc = Bytecode::from_byte(byte).unwrap_or(Bytecode::Break);
    let got = bytecode_size(bc);
    assert_eq!(got, *want, "#{i} {bc:?} 宽度");
    // 原版 kBytecodeAlignment = 4 (bytecodes.h:348): 指令定宽必为 4 的倍数
    assert_eq!(got % 4, 0, "#{i} {bc:?} 非 4 倍数");
    assert!(got >= 4, "#{i} {bc:?} 短于 opcode");
    // 帧宽 ≡ 汇编侧实际写入长度 (派生自同一张布局表)
    let mut asm = Asm::default();
    asm.insn(bc, &zero_ops(bc));
    assert_eq!(asm.code().len(), usize::from(got), "#{i} {bc:?} 汇编帧宽");
  }
  // 零操作数项 = RoundUp<4>(sizeof(Bytecode)) (inl.h:150 空序列分支)
  for bc in [
    Bytecode::Break,
    Bytecode::PushCurrentPosition,
    Bytecode::PopCurrentPosition,
    Bytecode::Fail,
    Bytecode::Succeed,
  ] {
    assert_eq!(operand_count(bc), 0, "{bc:?} 应无操作数");
    assert_eq!(bytecode_size(bc), 4, "{bc:?} 仅 opcode 块");
  }
  info!("T11 ok");
  OK
}

/// 单条 bytecode 的布局 golden (个数/逐操作数偏移与类型)
struct LayoutCase {
  bc: Bytecode,
  offsets: [u8; 3],
  types: [OperandType; 3],
}

/// T12 操作数排布 golden: P-3a 18 项 + 排布难点项 (inl.h:107-136 算法逐例复算)
#[test]
fn operand_layout_golden_cases() -> Void {
  use OperandType::*;
  // 未用到的第三槽填 Int16 占位 (count 之外不读, 与 inl.h:110-113 等价的变长语义)
  let cases = [
    // P-3a: 2 字节操作数与 opcode 同块 (offset 2), 4 字节另起块 (offset 4)
    LayoutCase {
      bc: Bytecode::PushBacktrack,
      offsets: [4, 0, 0],
      types: [JumpTarget, Int16, Int16],
    },
    LayoutCase {
      bc: Bytecode::WriteCurrentPositionToRegister,
      offsets: [2, 4, 0],
      types: [Register, Offset, Int16],
    },
    LayoutCase {
      bc: Bytecode::ReadCurrentPositionFromRegister,
      offsets: [2, 0, 0],
      types: [Register, Int16, Int16],
    },
    LayoutCase {
      bc: Bytecode::WriteStackPointerToRegister,
      offsets: [2, 0, 0],
      types: [Register, Int16, Int16],
    },
    LayoutCase {
      bc: Bytecode::ReadStackPointerFromRegister,
      offsets: [2, 0, 0],
      types: [Register, Int16, Int16],
    },
    LayoutCase {
      bc: Bytecode::SetRegister,
      offsets: [2, 4, 0],
      types: [Register, Int32, Int16],
    },
    LayoutCase {
      bc: Bytecode::ClearRegisters,
      offsets: [2, 4, 0],
      types: [Register, Register, Int16],
    },
    LayoutCase {
      bc: Bytecode::AdvanceRegister,
      offsets: [2, 4, 0],
      types: [Register, Offset, Int16],
    },
    // StackCheckFlag (enum: uint8_t, 宽 1 对齐 1) 落在 4 字节块首 → offset 4
    LayoutCase {
      bc: Bytecode::PushRegister,
      offsets: [2, 4, 0],
      types: [Register, StackCheckFlag, Int16],
    },
    LayoutCase {
      bc: Bytecode::PopRegister,
      offsets: [2, 0, 0],
      types: [Register, Int16, Int16],
    },
    LayoutCase {
      bc: Bytecode::AdvanceCurrentPosition,
      offsets: [2, 0, 0],
      types: [Offset, Int16, Int16],
    },
    LayoutCase {
      bc: Bytecode::GoTo,
      offsets: [4, 0, 0],
      types: [JumpTarget, Int16, Int16],
    },
    LayoutCase {
      bc: Bytecode::AdvanceCpAndGoto,
      offsets: [2, 4, 0],
      types: [Offset, JumpTarget, Int16],
    },
    LayoutCase {
      bc: Bytecode::CheckFixedLengthLoop,
      offsets: [4, 0, 0],
      types: [JumpTarget, Int16, Int16],
    },
    // Backtrack 的 return_code 为 kInt16 (bytecodes.h:189-190) → 同块 offset 2, 帧宽 4
    LayoutCase {
      bc: Bytecode::Backtrack,
      offsets: [2, 0, 0],
      types: [Int16, Int16, Int16],
    },
    LayoutCase {
      bc: Bytecode::IfRegisterLT,
      offsets: [2, 4, 8],
      types: [Register, Int32, JumpTarget],
    },
    LayoutCase {
      bc: Bytecode::IfRegisterGE,
      offsets: [2, 4, 8],
      types: [Register, Int32, JumpTarget],
    },
    LayoutCase {
      bc: Bytecode::IfRegisterEqPos,
      offsets: [2, 4, 0],
      types: [Register, JumpTarget, Int16],
    },
    // 难点项: StandardCharacterSet (对齐 1) 紧贴 opcode → offset 1 (非 2!)
    LayoutCase {
      bc: Bytecode::CheckSpecialClassRanges,
      offsets: [1, 4, 0],
      types: [StandardCharacterSet, JumpTarget, Int16],
    },
    // 难点项: Char+Char+Char 三连 2 字节填满一块后 JumpTarget 另起块
    LayoutCase {
      bc: Bytecode::CheckNotCharacterAfterMinusAnd,
      offsets: [2, 4, 6],
      types: [Char, Char, Char],
    },
    // 难点项: 16 字节 BitTable 对齐 1 → 紧接 4 字节块之后 (帧宽 24)
    LayoutCase {
      bc: Bytecode::CheckBitInTable,
      offsets: [4, 8, 0],
      types: [JumpTarget, BitTable, Int16],
    },
    LayoutCase {
      bc: Bytecode::LoadCurrentCharacter,
      offsets: [2, 4, 8],
      types: [Offset, BoundsCheckOffset, JumpTarget],
    },
  ];
  assert_eq!(cases.len(), 22);
  for c in cases.iter() {
    let n = operand_count(c.bc);
    assert!(n > 0, "{:?} 本表不覆盖零操作数项", c.bc);
    // 本表逐例记至前 3 个操作数 (4 操作数项的余项由帧宽断言覆盖)
    for (i, off) in c.offsets.iter().enumerate().take(n.min(3)) {
      assert_eq!(
        operand_offset(c.bc, i),
        usize::from(*off),
        "{:?} op{i} 偏移",
        c.bc
      );
      assert_eq!(operand_type(c.bc, i), c.types[i], "{:?} op{i} 类型", c.bc);
      // 对齐不变式 (inl.h:251 DCHECK IsAligned(offset, sizeof(CType)))
      assert_eq!(
        usize::from(*off) % usize::from(c.types[i].alignment()),
        0,
        "{:?} op{i} 未按自身对齐",
        c.bc
      );
      assert_eq!(
        operand_size(c.bc, i),
        c.types[i].size(),
        "{:?} op{i} 宽度",
        c.bc
      );
    }
    // kSize = RoundUp<4>(末操作数偏移 + 末操作数宽度) (inl.h:149-151)
    let last_off = operand_offset(c.bc, n - 1);
    let region_end = last_off + usize::from(operand_size(c.bc, n - 1));
    assert_eq!(
      usize::from(bytecode_size(c.bc)),
      region_end.div_ceil(4) * 4,
      "{:?} 帧宽 ≡ kSize",
      c.bc
    );
    if n <= 3 {
      assert_eq!(
        usize::from(c.offsets[n - 1]),
        last_off,
        "{:?} 末操作数偏移",
        c.bc
      );
    }
  }
  // 操作数上限项 (K_MAX_OPERAND_COUNT = 20, 实测 SkipUntilOneOfMasked3)
  assert_eq!(
    operand_count(Bytecode::SkipUntilOneOfMasked3),
    K_MAX_OPERAND_COUNT
  );
  info!("T12 ok");
  OK
}

/// T13 操作数读取器 Get<op> 往返 (inl.h:244-273) + BytecodeOperands 操作数下标
/// 常量 (inl.h:157-169 `Operand` 枚举 + 292-303 特化) 与 bytecodes.h 声明序全等
#[test]
fn operand_reader_roundtrip() -> Void {
  let mut asm = Asm::default();
  let sr = asm.insn(Bytecode::SetRegister, &[w2(3), w4i(-123_456)]);
  let ar = asm.insn(Bytecode::AdvanceRegister, &[w2(4), w2i(-3)]);
  let pb = asm.insn(Bytecode::PushBacktrack, &[w4(0x1234_5678)]);
  let pr = asm.insn(Bytecode::PushRegister, &[w2(5), w1(1)]);
  let bt = asm.insn(Bytecode::Backtrack, &[w2i(-3)]);
  let wb = asm.insn(Bytecode::WriteCurrentPositionToRegister, &[w2(1), w2i(1)]);
  let tbl = [0xABu8; K_BIT_TABLE_BYTES];
  let bit = asm.insn(Bytecode::CheckBitInTable, &[w4(8), tbl.to_vec()]);
  let code = asm.code().to_vec();
  let pc = |at: usize| u32::try_from(at).unwrap_or(u32::MAX);

  // kRegister = uint16_t / kInt32: 逐宽读取与写入值全等 (宿主序 reinterpret_cast 等价)
  assert_eq!(
    operand_u16(
      &code,
      pc(sr),
      Bytecode::SetRegister,
      bytecode_operands::set_register::REGISTER_INDEX
    ),
    3
  );
  assert_eq!(
    operand_i32(
      &code,
      pc(sr),
      Bytecode::SetRegister,
      bytecode_operands::set_register::VALUE
    ),
    -123_456
  );
  // kOffset = int16_t 带符号 (负 by / 正 cp_offset)
  assert_eq!(
    operand_i16(
      &code,
      pc(ar),
      Bytecode::AdvanceRegister,
      bytecode_operands::advance_register::BY
    ),
    -3
  );
  assert_eq!(
    operand_i16(
      &code,
      pc(wb),
      Bytecode::WriteCurrentPositionToRegister,
      bytecode_operands::write_current_position_to_register::CP_OFFSET
    ),
    1
  );
  // kJumpTarget = uint32_t
  assert_eq!(
    operand_u32(
      &code,
      pc(pb),
      Bytecode::PushBacktrack,
      bytecode_operands::push_backtrack::LABEL
    ),
    0x1234_5678
  );
  // kStackCheckFlag = enum: uint8_t (1 字节, 非 sizeof(int))
  assert_eq!(
    operand_u8(
      &code,
      pc(pr),
      Bytecode::PushRegister,
      bytecode_operands::push_register::STACK_CHECK
    ),
    1
  );
  // kInt16 返回码 (Backtrack 的 return_code)
  assert_eq!(
    operand_i16(
      &code,
      pc(bt),
      Bytecode::Backtrack,
      bytecode_operands::backtrack::RETURN_CODE
    ),
    -3
  );
  // 非 basic 型 (kBitTable): Get 返回 pc+Offset 指针 → 定长切片, 内容原样。
  // 下标 1 = bytecodes.h:189 V(CheckBitInTable, V(JumpTarget, on_bit_set),
  // V(BitTable, table)) 的声明序 (该 handler 属 P-3b, 无下标常量模块, 用实测序号)
  let table = operand_bit_table(&code, pc(bit), Bytecode::CheckBitInTable, 1);
  assert_eq!(table.len(), K_BIT_TABLE_BYTES);
  assert_eq!(table, &[0xABu8; K_BIT_TABLE_BYTES][..]);
  assert_eq!(
    operand_type(Bytecode::CheckBitInTable, 1),
    OperandType::BitTable
  );

  // 下标常量面: 每 handler 的最大下标 + 1 ≡ Operands::kCount (inl.h:177)
  use bytecode_operands as bo;
  let covers: [(&str, usize, usize); 18] = [
    (
      "PushBacktrack",
      bo::push_backtrack::LABEL,
      operand_count(Bytecode::PushBacktrack),
    ),
    (
      "WriteCPToReg",
      bo::write_current_position_to_register::CP_OFFSET,
      operand_count(Bytecode::WriteCurrentPositionToRegister),
    ),
    (
      "ReadCPFromReg",
      bo::read_current_position_from_register::REGISTER_INDEX,
      operand_count(Bytecode::ReadCurrentPositionFromRegister),
    ),
    (
      "WriteSPToReg",
      bo::write_stack_pointer_to_register::REGISTER_INDEX,
      operand_count(Bytecode::WriteStackPointerToRegister),
    ),
    (
      "ReadSPFromReg",
      bo::read_stack_pointer_from_register::REGISTER_INDEX,
      operand_count(Bytecode::ReadStackPointerFromRegister),
    ),
    (
      "SetRegister",
      bo::set_register::VALUE,
      operand_count(Bytecode::SetRegister),
    ),
    (
      "ClearRegisters",
      bo::clear_registers::TO_REGISTER,
      operand_count(Bytecode::ClearRegisters),
    ),
    (
      "AdvanceRegister",
      bo::advance_register::BY,
      operand_count(Bytecode::AdvanceRegister),
    ),
    (
      "PushRegister",
      bo::push_register::STACK_CHECK,
      operand_count(Bytecode::PushRegister),
    ),
    (
      "PopRegister",
      bo::pop_register::REGISTER_INDEX,
      operand_count(Bytecode::PopRegister),
    ),
    (
      "AdvanceCP",
      bo::advance_current_position::BY,
      operand_count(Bytecode::AdvanceCurrentPosition),
    ),
    ("GoTo", bo::go_to::LABEL, operand_count(Bytecode::GoTo)),
    (
      "AdvanceCpAndGoto",
      bo::advance_cp_and_goto::ON_GOTO,
      operand_count(Bytecode::AdvanceCpAndGoto),
    ),
    (
      "CheckFixedLengthLoop",
      bo::check_fixed_length_loop::ON_TOS_EQUALS_CURRENT_POSITION,
      operand_count(Bytecode::CheckFixedLengthLoop),
    ),
    (
      "Backtrack",
      bo::backtrack::RETURN_CODE,
      operand_count(Bytecode::Backtrack),
    ),
    (
      "IfRegisterLT",
      bo::if_register_lt::ON_LESS_THAN,
      operand_count(Bytecode::IfRegisterLT),
    ),
    (
      "IfRegisterGE",
      bo::if_register_ge::ON_GREATER_OR_EQUAL,
      operand_count(Bytecode::IfRegisterGE),
    ),
    (
      "IfRegisterEqPos",
      bo::if_register_eq_pos::ON_EQ,
      operand_count(Bytecode::IfRegisterEqPos),
    ),
  ];
  for (name, max_index, count) in covers {
    assert_eq!(max_index + 1, count, "{name}: 下标集须覆盖 0..kCount");
  }
  // 首下标恒 0 (bytecodes.h V-表声明序)
  assert_eq!(bo::set_register::REGISTER_INDEX, 0);
  assert_eq!(bo::clear_registers::FROM_REGISTER, 0);
  assert_eq!(bo::if_register_lt::REGISTER_INDEX, 0);
  assert_eq!(bo::if_register_lt::COMPARAND, 1);
  assert_eq!(bo::advance_cp_and_goto::BY, 0);
  info!("T13 ok");
  OK
}

/// T14 操作数类型宽度/对齐/kIsBasic 表 (inl.h:23-69 traits) + 限幅常量
/// (macro-assembler.h:42-46 经 bytecodes.h:39-47 入 traits)
#[test]
fn operand_type_size_alignment_table() -> Void {
  let rows: [(OperandType, u8, u8, bool); 11] = [
    (OperandType::Int16, 2, 2, true),                // int16_t
    (OperandType::Int32, 4, 4, true),                // int32_t
    (OperandType::Uint32, 4, 4, true),               // uint32_t
    (OperandType::Char, 2, 2, true),                 // base::uc16
    (OperandType::JumpTarget, 4, 4, true),           // uint32_t
    (OperandType::Offset, 2, 2, true),               // int16_t 限幅 (h:39)
    (OperandType::BoundsCheckOffset, 4, 4, true),    // int32_t 限幅 (h:40)
    (OperandType::Register, 2, 2, true),             // uint16_t 限幅 (h:42)
    (OperandType::StackCheckFlag, 1, 1, true),       // enum: uint8_t (h:43)
    (OperandType::StandardCharacterSet, 1, 1, true), // enum: char (h:44)
    (OperandType::BitTable, 16, 1, false),           // SPECIAL 专列 (h:47)
  ];
  assert_eq!(rows.len(), K_OPERAND_TYPE_COUNT);
  for (idx, (t, size, align, basic)) in rows.iter().enumerate() {
    assert_eq!(t.size(), *size, "{t:?} 宽度");
    assert_eq!(t.alignment(), *align, "{t:?} 对齐");
    assert_eq!(t.is_basic(), *basic, "{t:?} kIsBasic");
    // 编号 ≡ BytecodeOperandType 枚举序 (bytecodes.h:53-60 三表展开)
    assert_eq!(*t as u8, u8::try_from(idx).unwrap_or(u8::MAX), "{t:?} 编号");
  }
  // 位图宽度双引用: kTableSize(128)/kBitsPerByte(8) = 16 = BitTable::size()
  assert_eq!(K_BIT_TABLE_BYTES, 16);
  assert_eq!(usize::from(OperandType::BitTable.size()), K_BIT_TABLE_BYTES);
  // 限幅常量 (macro-assembler.h:42-46; bytecodes.h:40-42 的 BoundsCheckOffset 上界)
  assert_eq!(K_MAX_CP_OFFSET, (1 << 15) - 1);
  assert_eq!(K_MIN_CP_OFFSET, -K_MAX_CP_OFFSET);
  assert_eq!(
    K_MAX_BOUNDS_CHECK_OFFSET,
    K_MAX_CP_OFFSET + i32::from(u8::MAX)
  );
  assert_eq!(K_NO_BACKTRACK_LIMIT, 0); // JSRegExp::kNoBacktrackLimit (js-regexp.h:157)
  info!("T14 ok");
  OK
}

/// T15 [0] Break cc:577 — 派发表尾 kBreak 槽与无效字节的 UNREACHABLE 面。
/// 原版 handler 体为 `UNREACHABLE()` (cc:577), 本票以 v8_fatal 承接 (abort 语义,
/// 禁 panic/空实现伪装); 进程级 abort 不可在单测内触发, 故断言 run() 判定该面的
/// 解码式 (cc:511-530 查表 + from_byte 的 IsValid DCHECK) 输入侧全等。
#[test]
fn p3a_break_cc577_decode_filter_unreachable() -> Void {
  // kBreak 编号 0 (bytecodes.h:90) 且被 filter 排除 → fatal 路径
  assert_eq!(Bytecode::from_byte(0), Some(Bytecode::Break));
  assert!(
    Bytecode::from_byte(0)
      .filter(|b| *b != Bytecode::Break)
      .is_none(),
    "kBreak 须落 cc:577"
  );
  // 域外字节: from_byte 判无效 (原版 DCHECK(IsValid), h:360) → 同归 fatal
  for byte in K_BYTECODE_COUNT..=u8::MAX {
    assert!(Bytecode::from_byte(byte).is_none(), "byte {byte} 须无效");
  }
  // 1..=60 全部落 handler (非 fatal)
  for byte in 1..K_BYTECODE_COUNT {
    assert!(
      Bytecode::from_byte(byte)
        .filter(|b| *b != Bytecode::Break)
        .is_some(),
      "byte {byte} 应为可派发 opcode"
    );
  }
  // 派发表尾 (cc:528-530) 以 kBreak 填充的槽数 = kPaddedBytecodeCount - kCount = 3
  assert_eq!(
    K_PADDED_BYTECODE_COUNT - usize::from(K_BYTECODE_COUNT),
    3,
    "61..=63 落 kBreak 槽 → cc:577"
  );
  info!("T15 ok");
  OK
}

/// T16 [1] PushCurrentPosition cc:578-584 — ADVANCE 先于 push (cc:579/581)
#[test]
fn p3a_push_current_position_cc578() -> Void {
  let mut asm = Asm::default();
  let at = asm.insn(Bytecode::PushCurrentPosition, &[]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, params(3));
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(at, 0);
  assert_eq!(
    m.pc, 4,
    "ADVANCE(): Size(kPushCurrentPosition) = 4 (无操作数)"
  );
  assert_eq!(m.stack.sp(), 1);
  assert_eq!(m.stack.peek(), 3, "push(current) (cc:581)");
  assert_eq!(m.current, 3, "本指令不改位置");
  assert_eq!(
    m.hooks.stack_overflow_calls, 0,
    "预算内不触 MaybeThrowStackOverflow (cc:582)"
  );
  info!("T16 ok");
  OK
}

/// T17 [2] PushBacktrack cc:585-591 — label(kJumpTarget) 以 int 入栈 (cc:126-128)
#[test]
fn p3a_push_backtrack_cc585() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::PushBacktrack, &[w4(16)]);
  asm.insn(Bytecode::PushBacktrack, &[w4(8)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, params(0));
  // 两条连续 PushBacktrack: 帧宽 8 (JumpTarget 对齐 4 → 偏移 4)
  assert_eq!(asm.code().len(), 20, "8+8+4");
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, 8);
  assert_eq!(m.stack.sp(), 1);
  assert_eq!(m.stack.peek(), 16, "label=16 入栈");
  // LIFO: 第二次入栈后 Tos 为 8 (Backtrack 先弹最近者)
  let mut out2 = [0i32; 4];
  let mut m2 = mach(
    asm.code(),
    &mut out2,
    RawMatchParams {
      total_register_count: 6,
      step_limit: Some(2),
      ..Default::default()
    },
  );
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 2 });
  assert_eq!(m2.stack.sp(), 2);
  assert_eq!(m2.stack.peek(), 8);
  assert_eq!(m2.pc, 16);
  info!("T17 ok");
  OK
}

/// T18 [11] PushRegister cc:592-599 — stack_check 为 USE 面 (解释器不消费)
#[test]
fn p3a_push_register_cc592() -> Void {
  use bytecode_operands::push_register::{REGISTER_INDEX, STACK_CHECK};
  let mut asm = Asm::default();
  asm.insn(Bytecode::PushRegister, &[w2(1), w1(1)]);
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();
  assert_eq!(
    operand_u8(&code, 0, Bytecode::PushRegister, STACK_CHECK),
    1,
    "kStackCheckFlag 1 字节@偏移 4"
  );
  let mut out = [0i32; 4];
  let mut m = mach(&code, &mut out, params(2));
  m.registers.set(1, 777);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, 8);
  assert_eq!(m.stack.sp(), 1);
  assert_eq!(m.stack.peek(), 777, "push(registers[reg]) (cc:595)");
  assert_eq!(
    operand_u16(&code, 0, Bytecode::PushRegister, REGISTER_INDEX),
    1
  );
  // stack_check = kDontCheckStack(0) 同码 (USE 面, cc:593)
  asm.patch(0, Bytecode::PushRegister, STACK_CHECK, &w1(0));
  let mut out2 = [0i32; 4];
  let mut m2 = mach(asm.code(), &mut out2, params(2));
  m2.registers.set(1, -1);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m2.stack.peek(), -1, "kNoMatchValue 形态亦可入栈");
  info!("T18 ok");
  OK
}

/// T19 [7] SetRegister cc:600-603 (kRegister + kInt32)
#[test]
fn p3a_set_register_cc600() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::SetRegister, &[w2(5), w4i(-1_000_000)]);
  asm.insn(Bytecode::SetRegister, &[w2(0), w4i(7)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(
    asm.code(),
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      step_limit: Some(2),
      ..Default::default()
    },
  );
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 2 });
  assert_eq!(m.registers.get(5), -1_000_000, "内部寄存器亦可写");
  assert_eq!(m.registers.get(0), 7);
  assert_eq!(m.pc, 16, "两帧 8+8");
  assert_eq!(
    m.registers.get(1),
    InterpreterRegisters::K_NO_MATCH_VALUE,
    "未触及槽保初值"
  );
  info!("T19 ok");
  OK
}

/// T20 [8] ClearRegisters cc:605-612 — 闭区间 [from, to] 置 kNoMatchValue
#[test]
fn p3a_clear_registers_cc605() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::ClearRegisters, &[w2(2), w2(4)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, params(0));
  for i in 0..6 {
    m.registers.set(i, 100 + i32::try_from(i).unwrap_or(0));
  }
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, 8);
  assert_eq!(m.registers.get(0), 100);
  assert_eq!(m.registers.get(1), 101);
  for i in 2..5 {
    assert_eq!(
      m.registers.get(i),
      InterpreterRegisters::K_NO_MATCH_VALUE,
      "区间内 {i} 须清空 (cc:608-610)"
    );
  }
  assert_eq!(m.registers.get(5), 105);
  // from == to 单点清空 (原版闭区间语义)
  let mut asm1 = Asm::default();
  asm1.insn(Bytecode::ClearRegisters, &[w2(3), w2(3)]);
  let mut out1 = [0i32; 4];
  let mut m1 = mach(asm1.code(), &mut out1, params(0));
  m1.registers.set(3, 9);
  assert_eq!(m1.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m1.registers.get(3), -1);
  info!("T20 ok");
  OK
}

/// T21 [9] AdvanceRegister cc:613-616 — registers[reg] += by (int 加法回绕)
#[test]
fn p3a_advance_register_cc613() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::AdvanceRegister, &[w2(2), w2i(-3)]);
  asm.insn(Bytecode::AdvanceRegister, &[w2(2), w2i(1)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(
    asm.code(),
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      step_limit: Some(2),
      ..Default::default()
    },
  );
  m.registers.set(2, 10);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 2 });
  assert_eq!(m.registers.get(2), 8, "10 + (-3) + 1");
  assert_eq!(m.pc, 16);
  // 原版 C++ int 加法溢出为回绕 (禁 debug panic 假绿): Rust wrapping_add 逐位一致
  let mut asm2 = Asm::default();
  asm2.insn(Bytecode::AdvanceRegister, &[w2(1), w2i(1)]);
  let mut out2 = [0i32; 4];
  let mut m2 = mach(asm2.code(), &mut out2, params(0));
  m2.registers.set(1, i32::MAX);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m2.registers.get(1), i32::MIN, "i32::MAX + 1 回绕");
  info!("T21 ok");
  OK
}

/// T22 [3] WriteCurrentPositionToRegister cc:618-622 (cp_offset ∈ [kMinCPOffset, kMaxCPOffset])
#[test]
fn p3a_write_current_position_to_register_cc618() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::WriteCurrentPositionToRegister, &[w2(1), w2i(-2)]);
  asm.insn(Bytecode::WriteCurrentPositionToRegister, &[w2(3), w2i(0)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(
    asm.code(),
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      current: 7,
      step_limit: Some(2),
      ..Default::default()
    },
  );
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 2 });
  assert_eq!(
    m.registers.get(1),
    5,
    "registers[reg] = current + cp_offset (cc:620)"
  );
  assert_eq!(m.registers.get(3), 7, "cp_offset=0 即原样快照");
  assert_eq!(m.pc, 16);
  // 限幅边界操作数 (macro-assembler.h:42-43) 可解码
  let mut asm2 = Asm::default();
  asm2.insn(
    Bytecode::WriteCurrentPositionToRegister,
    &[w2(0), w2i(32767)],
  );
  let mut out2 = [0i32; 4];
  let mut m2 = mach(asm2.code(), &mut out2, params(0));
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m2.registers.get(0), K_MAX_CP_OFFSET);
  info!("T22 ok");
  OK
}

/// T23 [4] ReadCurrentPositionFromRegister cc:623-627 — SET_CURRENT_POSITION(reg)
#[test]
fn p3a_read_current_position_from_register_cc623() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::ReadCurrentPositionFromRegister, &[w2(4)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, params(0));
  m.registers.set(4, 5);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.current, 5, "current = registers[4] (cc:625)");
  assert_eq!(m.pc, 4, "帧宽 4 (kRegister 与 opcode 同块)");
  // 与已落纯函数同式: set_current_position(reg, subject.length())
  assert_eq!(
    set_current_position(m.registers.get(4), SUBJ8.len() as i32),
    5
  );
  info!("T23 ok");
  OK
}

/// T24 [5] WriteStackPointerToRegister cc:628-632 — 栈深快照 (cc:630)
#[test]
fn p3a_write_stack_pointer_to_register_cc628() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::WriteStackPointerToRegister, &[w2(0)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, params(1));
  let _ = m.stack.push(9);
  let _ = m.stack.push(0);
  let _ = m.stack.push(-1);
  assert_eq!(m.stack.sp(), 3);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.registers.get(0), 3, "registers[0] = sp()");
  assert_eq!(m.stack.sp(), 3, "本指令不动栈");
  // 空栈快照 = 0 (原版 sp() 即 data_.size())
  let mut m2 = mach(asm.code(), &mut out, params(0));
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m2.registers.get(0), 0);
  info!("T24 ok");
  OK
}

/// T25 [6] ReadStackPointerFromRegister cc:633-637 — set_sp 截断 (cc:635)
#[test]
fn p3a_read_stack_pointer_from_register_cc633() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::ReadStackPointerFromRegister, &[w2(1)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, params(0));
  for v in [11i32, 22, 33, 44] {
    let _ = m.stack.push(v);
  }
  m.registers.set(1, 2);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.stack.sp(), 2, "sp 截断至 registers[1]=2 (cc:635)");
  assert_eq!(m.stack.peek(), 22, "截断后 Tos 即原索引 1 槽");
  assert_eq!(m.pc, 4);
  // sp=0 全清分支
  let mut m2 = mach(asm.code(), &mut out, params(0));
  let _ = m2.stack.push(7);
  m2.registers.set(1, 0);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m2.stack.sp(), 0);
  info!("T25 ok");
  OK
}

/// T26 [10] PopCurrentPosition cc:638-642 — SET_CURRENT_POSITION(pop())
#[test]
fn p3a_pop_current_position_cc638() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::PopCurrentPosition, &[]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, params(4));
  let _ = m.stack.push(1);
  let _ = m.stack.push(4);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.current, 4, "current = pop() (cc:640)");
  assert_eq!(m.stack.sp(), 1);
  assert_eq!(m.pc, 4);
  // 与 PushCurrentPosition 配对回滚 (cc:578/638 对偶)
  let mut asm2 = Asm::default();
  asm2.insn(Bytecode::PushCurrentPosition, &[]);
  asm2.insn(Bytecode::PopCurrentPosition, &[]);
  asm2.insn(Bytecode::Succeed, &[]);
  let mut m2 = mach(
    asm2.code(),
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      current: 3,
      step_limit: Some(2),
      ..Default::default()
    },
  );
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 2 });
  assert_eq!(m2.stack.sp(), 0, "压弹相消");
  assert_eq!(m2.current, 3);
  assert_eq!(m2.pc, 8, "两帧各 4 字节");
  info!("T26 ok");
  OK
}

/// T27 [36] Backtrack cc:643-655 — 达 limit 返回 return_code / 否则弹栈跳转
#[test]
fn p3a_backtrack_cc643() -> Void {
  // 达 limit: ++count == limit → return (Result)return_code (cc:645-648), 不弹栈
  let mut asm_l1 = Asm::default();
  asm_l1.insn(Bytecode::Backtrack, &[w2i(0)]);
  let mut out_l1 = [0i32; 4];
  let mut m = mach(
    asm_l1.code(),
    &mut out_l1,
    RawMatchParams {
      total_register_count: 6,
      backtrack_limit: 1,
      step_limit: Some(1),
      ..Default::default()
    },
  );
  assert_eq!(m.run(), Step::Done(Result::Failure));
  assert_eq!(m.backtrack_count, 1, "++backtrack_count (cc:645)");
  assert_eq!(
    m.hooks.interrupt_calls, 0,
    "limit 分支先返回, 不经 HandleInterrupts"
  );
  assert_eq!(m.pc, 0, "return 路径不改 pc");
  // 未达 limit (kNoBacktrackLimit=0 永不触发, cc:644 static_assert): 弹栈跳转
  let mut asm = Asm::default();
  let pbb = asm.insn(Bytecode::PushBacktrack, &[w4(0)]);
  let btb = asm.insn(Bytecode::Backtrack, &[w2i(0)]);
  asm.insn(Bytecode::PushCurrentPosition, &[]); // 被跳转命中的帧
  asm.insn(Bytecode::Succeed, &[]);
  asm.patch(
    pbb,
    Bytecode::PushBacktrack,
    bytecode_operands::push_backtrack::LABEL,
    &w4(12),
  );
  let code = asm.code().to_vec();
  let mut out = [0i32; 4];
  let mut m2 = mach(
    &code,
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      current: 1,
      ..Default::default()
    },
  );
  assert_eq!(
    m2.run(),
    Step::Done(Result::Success),
    "弹栈 → PushCurrentPosition → Succeed"
  );
  assert_eq!(m2.pc, 16, "跳转目标 12 执行 4 字节帧后落 16");
  assert_eq!(m2.backtrack_count, 1);
  assert_eq!(
    m2.stack.sp(),
    1,
    "label 已弹出, PushCurrentPosition 重新入栈"
  );
  assert_eq!(m2.stack.peek(), 1);
  assert_eq!(m2.hooks.interrupt_calls, 1, "cc:650 恰一次");
  assert_eq!(btb, 8, "Backtrack 帧宽 4 (kInt16 与 opcode 同块)");
  info!("T27 ok");
  OK
}

/// T28 [12] PopRegister cc:657-661 — registers[reg] = pop()
#[test]
fn p3a_pop_register_cc657() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::PopRegister, &[w2(2)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, params(0));
  let _ = m.stack.push(5);
  let _ = m.stack.push(-88);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.registers.get(2), -88, "registers[2] = pop() (cc:659)");
  assert_eq!(m.stack.sp(), 1);
  assert_eq!(m.pc, 4);
  // 与 PushRegister 配对 (cc:592/657 对偶)
  let mut asm2 = Asm::default();
  asm2.insn(Bytecode::PushRegister, &[w2(1), w1(1)]);
  asm2.insn(Bytecode::SetRegister, &[w2(1), w4i(0)]);
  asm2.insn(Bytecode::PopRegister, &[w2(1)]);
  asm2.insn(Bytecode::Succeed, &[]);
  let mut m2 = mach(
    asm2.code(),
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      step_limit: Some(3),
      ..Default::default()
    },
  );
  m2.registers.set(1, 123);
  assert_eq!(m2.run(), Step::StepLimitExceeded { steps: 3 });
  assert_eq!(m2.registers.get(1), 123, "压栈值穿越中间清零后复原");
  assert_eq!(m2.stack.sp(), 0);
  info!("T28 ok");
  OK
}

/// T29 [13] Fail cc:662-666 — 采样后 FAILURE (cc:663-665)
#[test]
fn p3a_fail_cc662() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::PushCurrentPosition, &[]);
  asm.insn(Bytecode::Fail, &[]);
  let mut out = [7i32; 4];
  let mut m = mach(asm.code(), &mut out, free(2));
  assert_eq!(m.run(), Step::Done(Result::Failure));
  assert_eq!(
    m.hooks.backtrack_samples,
    vec![0],
    "AddSample(backtrack_count) cc:664"
  );
  assert_eq!(m.pc, 4, "return 前不 ADVANCE");
  assert_eq!(m.stack.sp(), 1, "已入栈的当前位置不回滚");
  assert_eq!(
    &m.output[..],
    &[7i32; 4],
    "Fail 不拷输出寄存器 (cc:665 无 CopyToOutput)"
  );
  // 采样值随 backtrack_count 累积 (cc:664 counters 面)
  let mut m2 = mach(asm.code(), &mut out, free(2));
  m2.backtrack_count = 41;
  assert_eq!(m2.run(), Step::Done(Result::Failure));
  assert_eq!(m2.hooks.backtrack_samples, vec![41]);
  info!("T29 ok");
  OK
}

/// T30 [14] Succeed cc:667-671 — 采样 + CopyToOutputRegisters 后 SUCCESS
#[test]
fn p3a_succeed_cc667() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [9i32; 4];
  let mut m = mach(asm.code(), &mut out, params(0));
  m.registers.set(0, 1);
  m.registers.set(1, 6);
  m.registers.set(2, -1);
  m.registers.set(3, 3);
  m.registers.set(4, 999); // 内部寄存器: 越出 output_register_count 不拷
  m.backtrack_count = 3;
  assert_eq!(m.run(), Step::Done(Result::Success));
  assert_eq!(
    &m.output[..],
    &[1, 6, -1, 3],
    "定长 output_register_count=4 槽拷贝 (cc:202-205)"
  );
  assert_eq!(m.hooks.backtrack_samples, vec![3]);
  assert_eq!(m.pc, 0, "return 路径 pc 不动");
  // output_register_count 之外 (cc:203 MemCopy 长度) 绝不被改写
  assert_eq!(m.registers.get(4), 999);
  info!("T30 ok");
  OK
}

/// T31 [15] AdvanceCurrentPosition cc:673-676 — ADVANCE_CURRENT_POSITION(by)
#[test]
fn p3a_advance_current_position_cc673() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::AdvanceCurrentPosition, &[w2i(2)]);
  asm.insn(Bytecode::AdvanceCurrentPosition, &[w2i(-1)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(
    asm.code(),
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      current: 1,
      step_limit: Some(2),
      ..Default::default()
    },
  );
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 2 });
  assert_eq!(m.current, 2, "1 + 2 - 1 (cc:675, by 为 Char 元素数)");
  assert_eq!(m.pc, 8, "两帧各 4 字节 (kOffset 与 opcode 同块)");
  assert_eq!(m.subject.len(), 5, "subject 长度即原版 IsInRange 上界");
  // 与已落纯函数同式 (cc:381)
  assert_eq!(advance_current_position(1, 2, SUBJ8.len() as i32), 3);
  info!("T31 ok");
  OK
}

/// T32 [16] GoTo cc:678-681 — SET_PC_FROM_OFFSET(label), 无 ADVANCE
#[test]
fn p3a_goto_cc678() -> Void {
  let mut asm = Asm::default();
  let at = asm.insn(Bytecode::GoTo, &[w4(0)]);
  asm.insn(Bytecode::PushCurrentPosition, &[]); // 8: 被跳过的帧
  asm.insn(Bytecode::Fail, &[]); // 12: 未跳转的落点
  asm.insn(Bytecode::Succeed, &[]); // 16: 跳转落点
  asm.patch(at, Bytecode::GoTo, bytecode_operands::go_to::LABEL, &w4(16));
  let code = asm.code().to_vec();
  // 步数上限 1: 只看 next_pc ≡ label (≠ pc + Size = 8)
  let mut out = [0i32; 4];
  let mut m = mach(&code, &mut out, params(0));
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, 16, "帧宽 8 ⇒ next_pc ≡ label");
  assert_eq!(m.stack.sp(), 0, "偏移 8 的 PushCurrentPosition 未执行");
  // 整程: 跳转直达 Succeed (若误落 4/8 则分别多一次入栈 / FAILURE)
  let mut m2 = mach(
    &code,
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      ..Default::default()
    },
  );
  assert_eq!(m2.run(), Step::Done(Result::Success));
  assert_eq!(m2.pc, 16, "halt 于 label 落点 16");
  assert_eq!(m2.stack.sp(), 0);
  info!("T32 ok");
  OK
}

/// T33 [47] AdvanceCpAndGoto cc:682-686 — SET_PC_FROM_OFFSET 后 ADVANCE_CURRENT_POSITION
#[test]
fn p3a_advance_cp_and_goto_cc682() -> Void {
  let mut asm = Asm::default();
  let at = asm.insn(Bytecode::AdvanceCpAndGoto, &[w2i(0), w4(0)]);
  asm.insn(Bytecode::PushCurrentPosition, &[]); // 8: 被跳过
  asm.insn(Bytecode::Fail, &[]); // 12: 未跳转落点
  asm.insn(Bytecode::Succeed, &[]); // 16: 跳转落点
  asm.patch(
    at,
    Bytecode::AdvanceCpAndGoto,
    bytecode_operands::advance_cp_and_goto::BY,
    &w2i(2),
  );
  asm.patch(
    at,
    Bytecode::AdvanceCpAndGoto,
    bytecode_operands::advance_cp_and_goto::ON_GOTO,
    &w4(16),
  );
  let code = asm.code().to_vec();
  let mut out = [0i32; 4];
  let mut m = mach(&code, &mut out, params(1));
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, 16, "on_goto 定 next_pc (cc:683)");
  assert_eq!(m.current, 3, "by=2 推进 (cc:684)");
  assert_eq!(m.stack.sp(), 0);
  let mut m2 = mach(
    &code,
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      current: 1,
      ..Default::default()
    },
  );
  assert_eq!(m2.run(), Step::Done(Result::Success));
  assert_eq!(m2.current, 3);
  assert_eq!(m2.pc, 16, "return 路径不改 pc (停在 Succeed 帧自身)");
  info!("T33 ok");
  OK
}

/// T34 [34] CheckFixedLengthLoop cc:687-695 — Tos == current 则跳转并弹出, 否则 ADVANCE
#[test]
fn p3a_check_fixed_length_loop_cc687() -> Void {
  // 命中分支: 弹 Tos + 跳转 (跳过紧随其后的 Fail)
  let mut asm = Asm::default();
  asm.insn(Bytecode::CheckFixedLengthLoop, &[w4(16)]);
  asm.insn(Bytecode::Fail, &[]); // 8: 未命中落点
  asm.insn(Bytecode::PushCurrentPosition, &[]); // 12: 被跳过
  asm.insn(Bytecode::Succeed, &[]); // 16: 命中落点
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, free(2));
  let _ = m.stack.push(2);
  assert_eq!(m.run(), Step::Done(Result::Success));
  assert_eq!(m.pc, 16, "halt 于跳转落点自身");
  assert_eq!(m.stack.sp(), 0, "跳转分支同时弹出 Tos (cc:692-693)");
  // 未命中分支: ADVANCE 且不弹栈
  let mut m2 = mach(asm.code(), &mut out, free(1));
  let _ = m2.stack.push(2);
  assert_eq!(m2.run(), Step::Done(Result::Failure));
  assert_eq!(m2.pc, 8, "ADVANCE(): 帧宽 8 (JumpTarget 另起块)");
  assert_eq!(m2.stack.sp(), 1, "未命中不弹栈");
  assert_eq!(m2.stack.peek(), 2);
  // 循环体: 定长循环推进至 Tos 相等才收敛 (原版 CheckFixedLengthLoop 语义)
  let mut loop_asm = Asm::default();
  let acp = loop_asm.insn(Bytecode::AdvanceCurrentPosition, &[w2i(1)]); // 0
  let cfl = loop_asm.insn(Bytecode::CheckFixedLengthLoop, &[w4(20)]); // 4
  let goto = loop_asm.insn(Bytecode::GoTo, &[w4(0)]); // 12
  loop_asm.insn(Bytecode::Succeed, &[]); // 20
  assert_eq!((acp, cfl, goto), (0, 4, 12));
  let mut m3 = mach(
    loop_asm.code(),
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      current: 0,
      ..Default::default()
    },
  );
  let _ = m3.stack.push(2); // 循环上界 (原版由 PushCurrentPosition 存档)
  assert_eq!(m3.run(), Step::Done(Result::Success));
  assert_eq!(m3.current, 2, "两轮 +1 后与 Tos 相等");
  assert_eq!(m3.pc, 20, "收敛后 halt 于 20 处 Succeed 帧");
  assert_eq!(m3.stack.sp(), 0, "循环出口弹出存档");
  info!("T34 ok");
  OK
}

/// T35 [29] IfRegisterLT cc:856-863 — 有符号比较 + 跳转对齐
#[test]
fn p3a_if_register_lt_cc856() -> Void {
  use bytecode_operands::if_register_lt::{COMPARAND, ON_LESS_THAN, REGISTER_INDEX};
  let mut asm = Asm::default();
  let at = asm.insn(Bytecode::IfRegisterLT, &[w2(0), w4i(0), w4(0)]);
  asm.insn(Bytecode::Fail, &[]); // 12: 未跳落点
  asm.insn(Bytecode::PushCurrentPosition, &[]); // 16: 被跳过
  asm.insn(Bytecode::Succeed, &[]); // 20: 跳转落点
  asm.patch(at, Bytecode::IfRegisterLT, ON_LESS_THAN, &w4(20));
  let code = asm.code().to_vec();
  assert_eq!(
    operand_i32(
      &code,
      u32::try_from(at).unwrap_or(0),
      Bytecode::IfRegisterLT,
      COMPARAND
    ),
    0
  );
  let mut out = [0i32; 4];
  // reg = -1 < 0 → 跳转 (有符号比较, 非无符号回绕)
  let mut m = mach(&code, &mut out, params(0));
  m.registers.set(0, -1);
  assert_eq!(m.run(), Step::StepLimitExceeded { steps: 1 });
  assert_eq!(m.pc, 20, "next_pc ≡ label ≠ pc + Size(12)");
  assert_eq!(m.stack.sp(), 0);
  let mut m2 = mach(&code, &mut out, free(0));
  m2.registers.set(0, -1);
  assert_eq!(m2.run(), Step::Done(Result::Success), "跳转直达 Succeed");
  // reg == comparand → 不跳 (LT 严格)
  let mut m3 = mach(&code, &mut out, free(0));
  m3.registers.set(0, 0);
  assert_eq!(m3.run(), Step::Done(Result::Failure));
  assert_eq!(m3.pc, 12, "ADVANCE(): 帧宽 12");
  // i32::MIN 亦可比较 (原版 int 语义)
  let mut m4 = mach(&code, &mut out, free(0));
  m4.registers.set(0, i32::MIN);
  assert_eq!(m4.run(), Step::Done(Result::Success));
  assert_eq!(
    operand_u16(&code, 0, Bytecode::IfRegisterLT, REGISTER_INDEX),
    0,
    "寄存器编号 @偏移 2"
  );
  info!("T35 ok");
  OK
}

/// T36 [30] IfRegisterGE cc:864-871 — 与 IfRegisterLT 互补 (等号归 GE)
#[test]
fn p3a_if_register_ge_cc864() -> Void {
  let mut asm = Asm::default();
  let at = asm.insn(Bytecode::IfRegisterGE, &[w2(1), w4i(4), w4(20)]);
  asm.insn(Bytecode::Fail, &[]); // 12
  asm.insn(Bytecode::PushCurrentPosition, &[]); // 16
  asm.insn(Bytecode::Succeed, &[]); // 20
  assert_eq!(at, 0);
  let mut out = [0i32; 4];
  // 相等 → 跳转 (GE 含等号, 与 LT 严格互补)
  let mut m = mach(asm.code(), &mut out, free(0));
  m.registers.set(1, 4);
  assert_eq!(m.run(), Step::Done(Result::Success));
  assert_eq!(m.pc, 20, "halt 于 label 落点 20");
  // 大于 → 跳转
  let mut m2 = mach(asm.code(), &mut out, free(0));
  m2.registers.set(1, 5);
  assert_eq!(m2.run(), Step::Done(Result::Success));
  // 小于 → 落空 ADVANCE 至 Fail
  let mut m3 = mach(asm.code(), &mut out, free(0));
  m3.registers.set(1, 3);
  assert_eq!(m3.run(), Step::Done(Result::Failure));
  assert_eq!(m3.pc, 12);
  // 负 comparand (kInt32 带符号)
  let mut asm2 = Asm::default();
  asm2.insn(Bytecode::IfRegisterGE, &[w2(2), w4i(-2), w4(20)]);
  asm2.insn(Bytecode::Fail, &[]);
  asm2.insn(Bytecode::PushCurrentPosition, &[]);
  asm2.insn(Bytecode::Succeed, &[]);
  let mut m4 = mach(asm2.code(), &mut out, free(0));
  m4.registers.set(2, -3);
  assert_eq!(m4.run(), Step::Done(Result::Failure), "-3 >= -2 为假");
  let mut m5 = mach(asm2.code(), &mut out, free(0));
  m5.registers.set(2, -2);
  assert_eq!(m5.run(), Step::Done(Result::Success), "-2 >= -2 为真");
  info!("T36 ok");
  OK
}

/// T37 [31] IfRegisterEqPos cc:872-879 — 与 current 比较 (非与常量)
#[test]
fn p3a_if_register_eq_pos_cc872() -> Void {
  let mut asm = Asm::default();
  let at = asm.insn(Bytecode::IfRegisterEqPos, &[w2(0), w4(0)]);
  asm.insn(Bytecode::Fail, &[]); // 8: 不等落点
  asm.insn(Bytecode::Succeed, &[]); // 12: 相等落点
  asm.patch(
    at,
    Bytecode::IfRegisterEqPos,
    bytecode_operands::if_register_eq_pos::ON_EQ,
    &w4(12),
  );
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, free(5));
  m.registers.set(0, 5);
  assert_eq!(
    m.run(),
    Step::Done(Result::Success),
    "registers[0] == current → 跳 (cc:874)"
  );
  assert_eq!(m.pc, 12, "halt 于 label 落点 12");
  assert_eq!(m.current, 5, "本指令不改位置");
  let mut m2 = mach(asm.code(), &mut out, free(5));
  m2.registers.set(0, 4);
  assert_eq!(m2.run(), Step::Done(Result::Failure));
  assert_eq!(m2.pc, 8, "ADVANCE(): 帧宽 8");
  // 未触及的寄存器保 kNoMatchValue(-1) 初值: -1 ≠ current(0) → 落空 Fail
  let mut m3 = mach(asm.code(), &mut out, free(0));
  assert_eq!(
    m3.run(),
    Step::Done(Result::Failure),
    "初值 -1 ≠ 0 → 不跳 (cc:874 严格相等)"
  );
  info!("T37 ok");
  OK
}

/// T38 栈溢出预算面 (cc:581/587/594 三 push 调用点共用 MaybeThrowStackOverflow
/// cc:227-234; Throw 本体 cc:215-223 属 [P-2] → 以 Hooks 注入承接, 不伪造 panic)
#[test]
fn p3a_backtrack_stack_overflow_limit() -> Void {
  // 原版 push (cc:135-138): 先入栈后判预算 ⇒ 越限值仍在栈上
  let mut st = BacktrackStack::new();
  for _ in 0..BacktrackStack::K_MAX_SIZE {
    assert!(st.push(0), "预算内 push 恒 true");
  }
  assert_eq!(BacktrackStack::K_MAX_SIZE, K_MAXIMUM_STACK_SIZE / 4);
  assert_eq!(
    usize::try_from(st.sp()).unwrap_or(usize::MAX),
    BacktrackStack::K_MAX_SIZE
  );
  assert!(!st.push(1), "越限 push 返回 false (cc:137)");
  assert_eq!(
    usize::try_from(st.sp()).unwrap_or(usize::MAX),
    BacktrackStack::K_MAX_SIZE + 1,
    "先入栈后判限 (原版语义)"
  );
  // PushBacktrack 调用点 (cc:587-590): EXCEPTION + kFromRuntime 归属
  let mut asm = Asm::default();
  asm.insn(Bytecode::PushBacktrack, &[w4(8)]);
  asm.insn(Bytecode::Succeed, &[]);
  let mut out = [0i32; 4];
  let mut m = mach(asm.code(), &mut out, params(0));
  m.stack = st;
  assert_eq!(m.run(), Step::Done(Result::Exception));
  assert_eq!(m.hooks.stack_overflow_calls, 1, "cc:589 恰一次");
  assert_eq!(m.hooks.last_overflow_origin, Some(CallOrigin::FromRuntime));
  assert_eq!(m.pc, 0, "return 路径不 ADVANCE");
  // PushCurrentPosition 调用点 (cc:582) 随 call_origin 归属 (两支同返 EXCEPTION)
  let mut asm2 = Asm::default();
  asm2.insn(Bytecode::PushCurrentPosition, &[]);
  let mut out2 = [0i32; 4];
  let mut m2 = mach(
    asm2.code(),
    &mut out2,
    RawMatchParams {
      total_register_count: 6,
      call_origin: CallOrigin::FromJs,
      step_limit: Some(1),
      ..Default::default()
    },
  );
  m2.stack = m.stack;
  assert_eq!(m2.run(), Step::Done(Result::Exception));
  assert_eq!(m2.hooks.last_overflow_origin, Some(CallOrigin::FromJs));
  info!("T38 ok");
  OK
}

/// T39 Backtrack 的 return_code 协议面 (cc:645-648 原版 static_cast 盲转 ⇒
/// Rust 以 Result::from_code 显式承接, 域外值不伪造枚举)
#[test]
fn p3a_backtrack_return_code_protocol() -> Void {
  let protocol = [
    (0i32, Result::Failure),    // RE_FAILURE (bytecode-generator.cc:307-311)
    (1i32, Result::Success),    // h:23
    (-1i32, Result::Exception), // h:24
    (-2i32, Result::Retry),     // h:25
    (-3i32, Result::FallbackToExperimental), // 生成器另一发射值 (cc:326)
  ];
  for (code, want) in protocol {
    let mut asm = Asm::default();
    asm.insn(
      Bytecode::Backtrack,
      &[w2i(i16::try_from(code).unwrap_or(0))],
    );
    let mut out = [0i32; 4];
    let mut m = mach(
      asm.code(),
      &mut out,
      RawMatchParams {
        total_register_count: 6,
        backtrack_limit: 1,
        ..Default::default()
      },
    );
    assert_eq!(m.run(), Step::Done(want), "return_code {code}");
    assert_eq!(Result::from_code(code), Some(want));
    assert_eq!(m.backtrack_count, 1, "++count 先于比较 (cc:645)");
    assert_eq!(
      m.hooks.interrupt_calls, 0,
      "limit 分支不经 HandleInterrupts"
    );
  }
  // 域外值: 取代未定义枚举值 (禁伪造), 逐值断言原码可复盘
  for code in [2i32, -4, 7, 32_767, -32_768] {
    let mut asm = Asm::default();
    asm.insn(
      Bytecode::Backtrack,
      &[w2i(i16::try_from(code).unwrap_or(0))],
    );
    let mut out = [0i32; 4];
    let mut m = mach(
      asm.code(),
      &mut out,
      RawMatchParams {
        total_register_count: 6,
        backtrack_limit: 1,
        ..Default::default()
      },
    );
    assert_eq!(
      m.run(),
      Step::IllegalResultCode { value: code },
      "域外 {code}"
    );
    assert_eq!(Result::from_code(code), None);
  }
  info!("T39 ok");
  OK
}

/// T40 中断注入传播 (cc:650-652) + 非法码流 (原版定长 TrustedByteArray 前提 cc:560)
#[test]
fn p3a_interrupt_and_illegal_code_paths() -> Void {
  // 码流: PushBacktrack(label=16) / Backtrack / PushCurrentPosition / Succeed
  let mut asm = Asm::default();
  asm.insn(Bytecode::PushBacktrack, &[w4(16)]);
  asm.insn(Bytecode::Backtrack, &[w2i(0)]);
  asm.insn(Bytecode::PushCurrentPosition, &[]);
  asm.insn(Bytecode::Succeed, &[]);
  let code = asm.code().to_vec();
  assert_eq!(code.len(), 20, "8+4+4+4");
  // 常态 (无待处理中断): 回溯续跑
  let mut out = [0i32; 4];
  let mut m = mach(
    &code,
    &mut out,
    RawMatchParams {
      total_register_count: 6,
      current: 2,
      ..Default::default()
    },
  );
  assert_eq!(m.run(), Step::Done(Result::Success));
  assert_eq!(m.hooks.interrupt_calls, 1, "每次 Backtrack 恰一次 (cc:649)");
  assert_eq!(m.current, 2);
  // 有中断 (HandleInterrupts 返非 SUCCESS): 直返, 不弹栈 (cc:650-652)
  let mut out2 = [0i32; 4];
  let mut mi = RawMatch::new(
    &code,
    SUBJ8,
    &mut out2,
    RawMatchParams {
      total_register_count: 6,
      ..Default::default()
    },
    TraceHooks {
      interrupt_after: Some(0),
      ..Default::default()
    },
  );
  assert_eq!(mi.run(), Step::Done(Result::Retry));
  assert_eq!(mi.pc, 8, "return 路径 pc 停在 Backtrack 帧");
  assert_eq!(mi.stack.sp(), 1, "label 未弹出 (可续跑前提)");
  assert_eq!(mi.backtrack_count, 1);
  assert_eq!(mi.hooks.interrupt_calls, 1);
  // 步数护栏 (测试面, 原版无对应物)
  let mut out3 = [0i32; 4];
  let mut ms = mach(
    &code,
    &mut out3,
    RawMatchParams {
      total_register_count: 6,
      step_limit: Some(2),
      ..Default::default()
    },
  );
  assert_eq!(ms.run(), Step::StepLimitExceeded { steps: 2 });
  assert_eq!(ms.pc, 16, "Backtrack 已弹栈跳转 (label=16)");
  // 帧截断: 声明宽 8 只给 4 字节
  let cut = [Bytecode::PushBacktrack.to_byte(), 0, 0, 0];
  let mut out4 = [0i32; 4];
  let mut mt = mach(&cut, &mut out4, params(0));
  assert_eq!(
    mt.run(),
    Step::IllegalCode {
      pc: 0,
      reason: "指令帧越出 code 数组"
    }
  );
  // 跳转目标越界: GoTo 999 → 下一轮 opcode 读不到
  let mut asm2 = Asm::default();
  asm2.insn(Bytecode::GoTo, &[w4(999)]);
  let mut out5 = [0i32; 4];
  let mut mg = mach(asm2.code(), &mut out5, free(0));
  assert_eq!(
    mg.run(),
    Step::IllegalCode {
      pc: 999,
      reason: "opcode 读越出 code 数组 (cc:560 定长前提)"
    }
  );
  info!("T40 ok");
  OK
}

/// P-3c/P-3d 落地台账项 (原版 regexp-interpreter.cc 行号 + Rust 派发分组)
struct LandedCase {
  bc: Bytecode,
  cc: u32,
  group: &'static str,
}

/// T41 P-3c/P-3d 16 项 = 61 − P-3a 23 − P-3b 22 落地台账 (原 "未落地分支逐名
/// NotImplemented" 表随 REG-1b3 exec_backref_ops / REG-1b4 exec_skip_ops 全量落地
/// 而退役; 逐项双臂语义断言迁至 tests/regexp_interpreter_p3c_p3d_unittest.rs)。
/// 本档职责 = 结构性吞账防护: 逐项实跑派发面, 断言出口是真实 Flow
/// (ADVANCE 落点 bytecode_size 或 Jump 落点操作数值), 而非批段 NotImplemented
/// 假绿通道; 起态 current = subject.len() 使 skip 系界检首判即出界 (确定性单步,
/// 无循环), 反向引用系走 cc:883 `from >= 0` 未捕获守卫臂。
#[test]
fn p3c_p3d_handlers_landed_ledger() -> Void {
  use Bytecode::*;
  const CASES: [LandedCase; 16] = [
    // ── P-3c 反向引用/锚点 (9) ──
    LandedCase {
      bc: CheckNotBackRef,
      cc: 880,
      group: "exec_backref_ops",
    },
    LandedCase {
      bc: CheckNotBackRefBackward,
      cc: 894,
      group: "exec_backref_ops",
    },
    LandedCase {
      bc: CheckNotBackRefNoCaseUnicode,
      cc: 908,
      group: "exec_backref_ops",
    },
    LandedCase {
      bc: CheckNotBackRefNoCase,
      cc: 922,
      group: "exec_backref_ops",
    },
    LandedCase {
      bc: CheckNotBackRefNoCaseUnicodeBackward,
      cc: 937,
      group: "exec_backref_ops",
    },
    LandedCase {
      bc: CheckNotBackRefNoCaseBackward,
      cc: 952,
      group: "exec_backref_ops",
    },
    LandedCase {
      bc: CheckAtStart,
      cc: 967,
      group: "exec_backref_ops",
    },
    LandedCase {
      bc: CheckNotAtStart,
      cc: 975,
      group: "exec_backref_ops",
    },
    LandedCase {
      bc: SetCurrentPositionFromEnd,
      cc: 983,
      group: "exec_backref_ops",
    },
    // ── P-3d skip-until peephole (7) ──
    LandedCase {
      bc: SkipUntilChar,
      cc: 1010,
      group: "exec_skip_ops",
    },
    LandedCase {
      bc: SkipUntilCharAnd,
      cc: 1023,
      group: "exec_skip_ops",
    },
    LandedCase {
      bc: SkipUntilBitInTable,
      cc: 1036,
      group: "exec_skip_ops",
    },
    LandedCase {
      bc: SkipUntilGtOrNotBitInTable,
      cc: 1049,
      group: "exec_skip_ops",
    },
    LandedCase {
      bc: SkipUntilCharOrChar,
      cc: 1066,
      group: "exec_skip_ops",
    },
    LandedCase {
      bc: SkipUntilOneOfMasked,
      cc: 1086,
      group: "exec_skip_ops",
    },
    LandedCase {
      bc: SkipUntilOneOfMasked3,
      cc: 1111,
      group: "exec_skip_ops",
    },
  ];
  let mut counts = [0u32; 2];
  for c in CASES.iter() {
    let mut asm = Asm::default();
    let mut ops = zero_ops(c.bc);
    // 原版生成器不变式 DCHECK_GE(max_offset, cp_offset + 3) (cc:1090,
    // SkipUntilOneOfMasked): 零操作数会触达该 DCHECK, 补合法 4 字符窗口
    if c.bc == Bytecode::SkipUntilOneOfMasked {
      ops[4] = w4(3);
    }
    asm.insn(c.bc, &ops);
    let mut out = [0i32; 4];
    // current = 5 = subject.len(): skip 系界检首判即出界; 反向引用系寄存器 kNoMatchValue=-1 走守卫臂
    let mut m = mach(asm.code(), &mut out, params(5));
    let step = m.run();
    assert!(
      matches!(step, Step::StepLimitExceeded { steps: 1 }),
      "{:?} 未走真实 Flow (cc:{}): {step:?}",
      c.bc,
      c.cc
    );
    assert!(
      !matches!(step, Step::NotImplemented { .. }),
      "{:?} 批段假绿通道残留 (cc:{})",
      c.bc,
      c.cc
    );
    // 出口 pc 只可能是 ADVANCE 落点 (指令定宽) 或 Jump 落点 (零操作数值)
    let adv = u32::from(bytecode_size(c.bc));
    assert!(
      m.pc == 0 || m.pc == adv,
      "{:?} 派发出口异常: pc={} (期望 0 或 {adv}, cc:{})",
      c.bc,
      m.pc,
      c.cc
    );
    match c.group {
      "exec_backref_ops" => counts[0] += 1,
      "exec_skip_ops" => counts[1] += 1,
      other => unreachable!("未知派发分组 {other}"),
    }
  }
  assert_eq!(
    counts,
    [9, 7],
    "批段切分 P-3c=9 P-3d=7 (exec_backref_ops / exec_skip_ops)"
  );
  // P-3a 23 项 + P-3b 22 项 + P-3c 9 项 + P-3d 7 项 = 61 全集, 无缺项无余项
  assert_eq!(CASES.len(), 16, "落地台账项 = 61 − 23 − 22 = 16");
  assert_eq!(CASES.len() + 23 + 22, usize::from(K_BYTECODE_COUNT));
  info!("T41 ok");
  OK
}

/// T42 票面差异 ③: AdvanceCpAndGoto 与位宽/字节序无关 — 一/两字节 subject
/// 两态同码 (bits_per_char cc:105-108 只入 Load2/Load4 移位量)
#[test]
fn p3a_advance_cp_and_goto_two_subject_states() -> Void {
  let mut asm = Asm::default();
  asm.insn(Bytecode::AdvanceCpAndGoto, &[w2i(2), w4(16)]); // 0
  asm.insn(Bytecode::Fail, &[]); // 8: 未跳转落点
  asm.insn(Bytecode::PushCurrentPosition, &[]); // 12: 被跳过
  asm.insn(Bytecode::Succeed, &[]); // 16: 跳转落点
  let code = asm.code().to_vec();
  let p = RawMatchParams {
    total_register_count: 6,
    current: 1,
    ..Default::default()
  };
  let mut out8 = [0i32; 4];
  let mut m8 = RawMatch::new(&code, SUBJ8, &mut out8, p, TraceHooks::default());
  let s8 = m8.run();
  let mut out16 = [0i32; 4];
  let mut m16 = RawMatch::new(&code, SUBJ16, &mut out16, p, TraceHooks::default());
  let s16 = m16.run();
  assert_eq!(s8, Step::Done(Result::Success));
  assert_eq!(s8, s16, "两态派发同出口");
  assert_eq!(m8.pc, 16);
  assert_eq!(m8.pc, m16.pc, "next_pc 同值");
  assert_eq!(m8.current, 3, "by=2 为 Char 元素偏移 (cc:684), 非字节宽度");
  assert_eq!(m8.current, m16.current);
  assert_eq!(m8.stack.sp(), m16.stack.sp());
  // Subject 两态视图同长同码位 (原版 template <Char> 的抽象)
  assert_eq!(m8.subject.len(), 5);
  assert_eq!(m16.subject.len(), 5);
  assert_eq!(m8.subject.char_at(0), 0x61);
  assert_eq!(m16.subject.char_at(0), 0x61);
  // 位宽常量仅影响 Load2/Load4 (已落纯函数), 不影响本 handler
  assert_eq!(bits_per_char_one_byte(), 8);
  assert_eq!(bits_per_char_two_byte(), 16);
  info!("T42 ok");
  OK
}
