//! 实验性解释器 (experimental-interpreter.{h,cc}) 骨架票规约测试。
//!
//! 规约来源 (实测枚举): v8/test/unittests/regexp/experimental/ 与
//! v8/test/cctest/ 下无 experimental-interpreter 专有用例 (实测目录不存在 +
//! grep 0 命中); 原版引擎仅经 mjsunit regexp-experimental.js /
//! regexp-lookahead-experimental.js / regexp-lookbehind-experimental.js 三个
//! JS 级全链路用例间接消费 —— 依赖编译器 (EXP-2) 与入口面 (EXP-4), 登记推迟
//! 1:1 迁移。本票按任务规范为已落地面构造金标准向量: 逐值依原版 C++ 语义
//! (行号锚见各测试注释) 手工推演, 断言强度不弱化。

use aok::{OK, Void};
use v8_regexp_exec::experimental::{
  bytecode::{Instruction, Opcode, Payload},
  interpreter::{
    ConsumedCharacter, InterpreterError, InterpreterThread, K_UNDEFINED_CLOCK_VALUE,
    K_UNDEFINED_MATCH_INDEX_VALUE, K_UNDEFINED_REGISTER_VALUE, LastInputIndex, LookaroundInfo,
    NfaInterpreter, filter_groups, is_line_terminator, is_regexp_word, satisfies_assertion,
  },
};
use v8_regexp_syntax::{AssertionType, LookaroundType};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ── 常量金标准 (原版 cc:21-23/733 字面量对账, todo 12.2 常量类金标准) ─────────

#[test]
fn constants_golden() -> Void {
  // cc:21 constexpr int = -1; cc:22 同
  assert_eq!(K_UNDEFINED_REGISTER_VALUE, -1);
  assert_eq!(K_UNDEFINED_MATCH_INDEX_VALUE, -1);
  // cc:23 constexpr uint64_t kUndefinedClockValue = -1 — 回绕为全 1
  assert_eq!(K_UNDEFINED_CLOCK_VALUE, 0_u64.wrapping_sub(1));
  assert_eq!(K_UNDEFINED_CLOCK_VALUE, u64::MAX);
  OK
}

// ── 字符谓词 (原版 unicode.h:278-280 / char-predicates-inl.h:82-84) ───────────

#[test]
fn line_terminator_golden() -> Void {
  // 四值全集命中 (unicode.h:279): LF CR LS PS
  for c in [0x000Au16, 0x000D, 0x2028, 0x2029] {
    assert!(is_line_terminator(c), "U+{c:04X} 应为行终止符");
  }
  // 非行终止符邻域: VT/FF/CR-1/LS 区间外
  for c in [0x000Bu16, 0x000C, 0x0009, 0x2027, 0x202A, 0x0000] {
    assert!(!is_line_terminator(c), "U+{c:04X} 不应为行终止符");
  }
  OK
}

#[test]
fn regexp_word_golden() -> Void {
  // IsRegExpWord = IsAlphaNumeric || '_': 0-9 A-Z a-z _ (ASCII 域)
  for c in *b"059AZaz_" {
    assert!(is_regexp_word(u16::from(c)), "{c} 应为 \\w");
  }
  // 邻近非词字符: '/'=0x2F ':'=0x3A '@'=0x40 '['=0x5B '`'=0x60 '{'=0x7B '-' 空格
  for c in [
    0x002Fu16, 0x003A, 0x0040, 0x005B, 0x0060, 0x007B, 0x002D, 0x0020,
  ] {
    assert!(!is_regexp_word(c), "U+{c:04X} 不应为 \\w");
  }
  // 非 ASCII (原版 IsAlphaNumeric 仅 ASCII 域, unicode.h 无折叠): É 全角数字
  for c in [0x00C9u16, 0xFF11, 0x2028] {
    assert!(!is_regexp_word(c), "U+{c:04X} 不应为 \\w");
  }
  OK
}

// ── 断言谓词 (cc:25-61 SatisfiesAssertion 全分支) ─────────────────────────────

/// 混测上下文: [ 'a', '\n', '0', U+2028, U+2029, ' ' ] (w n w n n n)
fn assert_ctx() -> Vec<u16> {
  vec![0x0061u16, 0x000A, 0x0030, 0x2028, 0x2029, 0x0020]
}

#[test]
fn satisfies_assertion_input_bounds() -> Void {
  let ctx = assert_ctx();
  // cc:32-33 START_OF_INPUT / cc:34-35 END_OF_INPUT
  assert!(satisfies_assertion(AssertionType::StartOfInput, &ctx, 0));
  assert!(!satisfies_assertion(AssertionType::StartOfInput, &ctx, 1));
  assert!(satisfies_assertion(
    AssertionType::EndOfInput,
    &ctx,
    ctx.len()
  ));
  assert!(!satisfies_assertion(
    AssertionType::EndOfInput,
    &ctx,
    ctx.len() - 1
  ));
  OK
}

#[test]
fn satisfies_assertion_line() -> Void {
  let ctx = assert_ctx();
  // cc:40-42 START_OF_LINE: 串首真; 前一码元为行终止符真
  assert!(satisfies_assertion(AssertionType::StartOfLine, &ctx, 0));
  assert!(!satisfies_assertion(AssertionType::StartOfLine, &ctx, 1)); // 前 'a'
  assert!(satisfies_assertion(AssertionType::StartOfLine, &ctx, 2)); // 前 '\n'
  assert!(!satisfies_assertion(AssertionType::StartOfLine, &ctx, 3)); // 前 '0'
  assert!(satisfies_assertion(AssertionType::StartOfLine, &ctx, 4)); // 前 U+2028
  assert!(satisfies_assertion(AssertionType::StartOfLine, &ctx, 5)); // 前 U+2029
  // cc:43-45 END_OF_LINE: 串尾真; 当前码元为行终止符真
  assert!(satisfies_assertion(
    AssertionType::EndOfLine,
    &ctx,
    ctx.len()
  ));
  assert!(!satisfies_assertion(AssertionType::EndOfLine, &ctx, 0)); // 当前 'a'
  assert!(satisfies_assertion(AssertionType::EndOfLine, &ctx, 1)); // 当前 '\n'
  assert!(!satisfies_assertion(AssertionType::EndOfLine, &ctx, 2)); // 当前 '0'
  assert!(satisfies_assertion(AssertionType::EndOfLine, &ctx, 3)); // 当前 U+2028
  assert!(satisfies_assertion(AssertionType::EndOfLine, &ctx, 4)); // 当前 U+2029
  assert!(!satisfies_assertion(AssertionType::EndOfLine, &ctx, 5)); // 当前 ' '
  OK
}

#[test]
fn satisfies_assertion_boundary() -> Void {
  let ctx = assert_ctx(); // 词性 w n w n n n
  // cc:46-56 BOUNDARY: 空串恒假; 串首/串尾看单侧; 中间看异或
  assert!(!satisfies_assertion(AssertionType::Boundary, &[], 0));
  assert!(satisfies_assertion(AssertionType::Boundary, &ctx, 0)); // 首 'a' 词
  assert!(!satisfies_assertion(AssertionType::Boundary, &ctx, 6)); // 尾 ' ' 非词
  assert!(satisfies_assertion(AssertionType::Boundary, &ctx, 1)); // a|↔\n 异或
  assert!(satisfies_assertion(AssertionType::Boundary, &ctx, 2)); // \n↔0 异或
  assert!(satisfies_assertion(AssertionType::Boundary, &ctx, 3)); // 0↔LS 异或
  assert!(!satisfies_assertion(AssertionType::Boundary, &ctx, 4)); // LS↔PS 同非词
  assert!(!satisfies_assertion(AssertionType::Boundary, &ctx, 5)); // PS↔' ' 同非词
  // cc:57-58 NON_BOUNDARY 逐位取反
  for pos in 0..=ctx.len() {
    assert_eq!(
      satisfies_assertion(AssertionType::NonBoundary, &ctx, pos),
      !satisfies_assertion(AssertionType::Boundary, &ctx, pos),
      "pos {pos} NON_BOUNDARY 取反"
    );
  }
  assert!(satisfies_assertion(AssertionType::NonBoundary, &[], 0));
  // 词字符集边界: '_' 数字均词 (IsRegExpWord 表源)
  let under = vec![0x005Fu16, 0x002D]; // '_-'
  assert!(satisfies_assertion(AssertionType::Boundary, &under, 1)); // _|↔-
  assert!(!satisfies_assertion(AssertionType::Boundary, &under, 2)); // 尾 '-' 非词
  OK
}

// ── 捕获过滤 (cc:98-250 FilterGroups 全算法面) ────────────────────────────────

/// cc:180-197 根级 FILTER_GROUP: 时钟有效即拷贝组起止寄存器
#[test]
fn filter_groups_root_group_keep_and_discard() -> Void {
  let bytecode = [Instruction::filter_group(1)];
  let registers = [0, 5, 2, 3, -1, -1];
  let capture_clocks = [0, 0, 9, 9, 0, 0];
  let quantifiers_clocks: [u64; 0] = [];

  // 保留: capture_clocks[2]=9 >= max_clock(0) 且 != kUndefinedClock (cc:185-188)
  let mut filtered = [K_UNDEFINED_REGISTER_VALUE; 6];
  filter_groups(
    0,
    &registers,
    &quantifiers_clocks,
    &capture_clocks,
    None,
    &mut filtered,
    &bytecode,
  )
  .unwrap();
  assert_eq!(filtered, [-1, -1, 2, 3, -1, -1]);

  // 丢弃: 时钟 = kUndefinedClockValue → Up 跳子树 (cc:190-195)
  let capture_clocks_discard = [0, 0, K_UNDEFINED_CLOCK_VALUE, K_UNDEFINED_CLOCK_VALUE, 0, 0];
  let mut filtered = [K_UNDEFINED_REGISTER_VALUE; 6];
  filter_groups(
    0,
    &registers,
    &quantifiers_clocks,
    &capture_clocks_discard,
    None,
    &mut filtered,
    &bytecode,
  )
  .unwrap();
  assert_eq!(filtered, [-1; 6]);
  OK
}

/// cc:199-213 FILTER_QUANTIFIER 刷新 max_clock 后, 子组按新阈值过滤
#[test]
fn filter_groups_quantifier_gating() -> Void {
  // 段结构: [FQ0, FILTER_CHILD→3, <非FC填充>, FG1]
  let bytecode = [
    Instruction::filter_quantifier(0),
    Instruction::filter_child(3),
    Instruction::accept(),
    Instruction::filter_group(1),
  ];
  let registers = [0, 9, 4, 7, -1, -1];
  let quantifiers_clocks = [5];
  // 子组时钟 7 >= 5 保留
  let keep_clocks = [0, 0, 7, 7, 0, 0];
  let mut filtered = [K_UNDEFINED_REGISTER_VALUE; 6];
  filter_groups(
    0,
    &registers,
    &quantifiers_clocks,
    &keep_clocks,
    None,
    &mut filtered,
    &bytecode,
  )
  .unwrap();
  assert_eq!(
    filtered,
    [-1, -1, 4, 7, -1, -1],
    "时钟 7 ≥ 量词末轮 5 应保留"
  );

  // 子组时钟 3 < 5 (上一轮迭代定义) → 整组连同子树丢弃
  let stale_clocks = [0, 0, 3, 3, 0, 0];
  let mut filtered = [K_UNDEFINED_REGISTER_VALUE; 6];
  filter_groups(
    0,
    &registers,
    &quantifiers_clocks,
    &stale_clocks,
    None,
    &mut filtered,
    &bytecode,
  )
  .unwrap();
  assert_eq!(filtered, [-1; 6], "旧迭代时钟应丢弃");
  OK
}

/// cc:168-178 FILTER_CHILD 兄弟回跳帧: 前一子树丢弃不影响兄弟保留
#[test]
fn filter_groups_sibling_resume() -> Void {
  // [FC→3, FC→4, 填充, FG1, FG2]: pc0 压回跳帧 (下条仍 FC), 子树后从 pc1 续兄弟
  let bytecode = [
    Instruction::filter_child(3),
    Instruction::filter_child(4),
    Instruction::accept(),
    Instruction::filter_group(1),
    Instruction::filter_group(2),
  ];
  let registers = [0, 9, 1, 2, 3, 4];
  let quantifiers_clocks: [u64; 0] = [];

  // 双保留: filtered[2..6] 均拷
  let clocks_keep = [0, 0, 1, 1, 1, 1];
  let mut filtered = [K_UNDEFINED_REGISTER_VALUE; 6];
  filter_groups(
    0,
    &registers,
    &quantifiers_clocks,
    &clocks_keep,
    None,
    &mut filtered,
    &bytecode,
  )
  .unwrap();
  assert_eq!(filtered, [-1, -1, 1, 2, 3, 4]);

  // g1 时钟哨兵丢弃, g2 兄弟仍保留 (cc:190-195 仅跳子树)
  let clocks_mix = [0, 0, K_UNDEFINED_CLOCK_VALUE, K_UNDEFINED_CLOCK_VALUE, 1, 1];
  let mut filtered = [K_UNDEFINED_REGISTER_VALUE; 6];
  filter_groups(
    0,
    &registers,
    &quantifiers_clocks,
    &clocks_mix,
    None,
    &mut filtered,
    &bytecode,
  )
  .unwrap();
  assert_eq!(filtered, [-1, -1, -1, -1, 3, 4]);
  OK
}

/// cc:215-229 FILTER_LOOKAROUND: 缺省恒放行; 有时钟按阈值保留/丢整棵子树
#[test]
fn filter_groups_lookaround_track() -> Void {
  // [FQ0, FC→3, 填充, FLA0, FC→6, 填充, FG1]
  let bytecode = [
    Instruction::filter_quantifier(0),
    Instruction::filter_child(3),
    Instruction::accept(),
    Instruction::filter_lookaround(0),
    Instruction::filter_child(6),
    Instruction::accept(),
    Instruction::filter_group(1),
  ];
  let registers = [0, 9, 4, 7, -1, -1];
  let quantifiers_clocks = [5]; // max_clock 刷新为 5
  let capture_clocks = [0, 0, 7, 7, 0, 0];

  // 仅无捕获后视轨 (cc:217 !has_value()): 恒放行 → 子组保留
  let mut filtered = [K_UNDEFINED_REGISTER_VALUE; 6];
  filter_groups(
    0,
    &registers,
    &quantifiers_clocks,
    &capture_clocks,
    None,
    &mut filtered,
    &bytecode,
  )
  .unwrap();
  assert_eq!(
    filtered,
    [-1, -1, 4, 7, -1, -1],
    "lookaround_clocks 缺省恒放行"
  );

  // 环视时钟 9 ≥ 5 保留 → 深入子组
  let mut filtered = [K_UNDEFINED_REGISTER_VALUE; 6];
  filter_groups(
    0,
    &registers,
    &quantifiers_clocks,
    &capture_clocks,
    Some(&[9]),
    &mut filtered,
    &bytecode,
  )
  .unwrap();
  assert_eq!(filtered, [-1, -1, 4, 7, -1, -1]);

  // 环视时钟 2 < 5 → 环视节点连同其子组丢弃
  let mut filtered = [K_UNDEFINED_REGISTER_VALUE; 6];
  filter_groups(
    0,
    &registers,
    &quantifiers_clocks,
    &capture_clocks,
    Some(&[2]),
    &mut filtered,
    &bytecode,
  )
  .unwrap();
  assert_eq!(filtered, [-1; 6], "旧轮环视应连子树丢弃");
  OK
}

/// cc:231-232 非 FILTER_* 混入 → 原版 UNREACHABLE, Rust 加固 Err;
/// cc:166 越界子节点 pc → BadPc
#[test]
fn filter_groups_defense() -> Void {
  let registers = [0, 9, 1, 2, -1, -1];
  let clocks = [0, 0, 1, 1];
  let mut filtered = [K_UNDEFINED_REGISTER_VALUE; 4];

  // 链中 FILTER_CHILD 目标混入 JMP (cc:231-232 原版 UNREACHABLE 面; 原版仅当 pc
  // 落点为非 FILTER_* 才触达 — FG→JMP 路径在 IsAtNodeEnd 即回退, 不落点)
  let bad = [Instruction::filter_child(1), Instruction::jmp(0)];
  let err = filter_groups(0, &registers, &[], &clocks, None, &mut filtered, &bad).unwrap_err();
  assert!(matches!(err, InterpreterError::BadBytecode { .. }), "{err}");

  // FILTER_CHILD 目标越界
  let oob = [Instruction::filter_child(9)];
  let err = filter_groups(0, &registers, &[], &clocks, None, &mut filtered, &oob).unwrap_err();
  assert!(
    matches!(err, InterpreterError::BadPc { pc: 9, len: 1, .. }),
    "{err}"
  );

  // FILTER_CHILD 负目标
  let neg = [Instruction {
    opcode: Opcode::FilterChild,
    payload: Payload::Pc(-2),
  }];
  let err = filter_groups(0, &registers, &[], &clocks, None, &mut filtered, &neg).unwrap_err();
  assert!(matches!(err, InterpreterError::BadBytecode { .. }), "{err}");
  OK
}

// ── 线程状态 (cc:544-589 + 工厂 cc:1337-1378) ─────────────────────────────────

#[test]
fn thread_new_empty_golden() -> Void {
  // flag 开 + 环视轨 (cc:1340-1349): 全数组在场并按哨兵/0 填充
  let t = InterpreterThread::new_empty(7, 4, 2, 3, true, false);
  assert_eq!(t.pc, 7);
  assert_eq!(t.register_array, vec![K_UNDEFINED_REGISTER_VALUE; 4]);
  assert_eq!(
    t.lookaround_match_index_array,
    Some(vec![K_UNDEFINED_MATCH_INDEX_VALUE; 3])
  );
  assert_eq!(t.quantifier_clock_array, Some(vec![0; 2]));
  assert_eq!(t.capture_clock_array, Some(vec![0; 4]));
  assert_eq!(t.lookaround_clock_array, Some(vec![0; 3]));
  assert_eq!(
    t.consumed_since_last_quantifier,
    ConsumedCharacter::DidConsume
  );

  // flag 开 + 仅无捕获后视轨 (cc:1344-1348): 环视两数组 nullptr → None
  let t = InterpreterThread::new_empty(0, 2, 1, 3, true, true);
  assert_eq!(t.lookaround_match_index_array, None);
  assert_eq!(t.lookaround_clock_array, None);
  assert_eq!(t.quantifier_clock_array, Some(vec![0; 1]));
  assert_eq!(t.capture_clock_array, Some(vec![0; 2]));

  // flag 关 (cc:1350-1354): 除寄存器外全 nullptr → None
  let t = InterpreterThread::new_empty(1, 2, 5, 5, false, false);
  assert_eq!(t.register_array, vec![K_UNDEFINED_REGISTER_VALUE; 2]);
  assert!(
    t.lookaround_match_index_array.is_none()
      && t.quantifier_clock_array.is_none()
      && t.capture_clock_array.is_none()
      && t.lookaround_clock_array.is_none()
  );

  // 未初始化版 (cc:1357-1378): 数组在场性与空版同构; 占位值不承诺 (FORK 先全拷后读)
  let u = InterpreterThread::new_uninitialized(2, 3, 1, 2, true, false);
  assert_eq!(u.register_array.len(), 3);
  assert_eq!(u.quantifier_clock_array.as_ref().map(Vec::len), Some(1));
  assert_eq!(u.capture_clock_array.as_ref().map(Vec::len), Some(3));
  assert_eq!(u.lookaround_clock_array.as_ref().map(Vec::len), Some(2));
  assert_eq!(
    u.lookaround_match_index_array.as_ref().map(Vec::len),
    Some(2)
  );
  let u = InterpreterThread::new_uninitialized(2, 3, 1, 2, true, true);
  assert!(u.lookaround_clock_array.is_none() && u.lookaround_match_index_array.is_none());
  OK
}

// ── pc 去重表 (cc:1492-1502 + cc:1427-1465) ──────────────────────────────────

#[test]
fn last_input_index_golden() -> Void {
  // cc:1493-1494 默认 (-1,-1); cc:1495-1498 双参构造
  assert_eq!(LastInputIndex::default(), LastInputIndex::new(-1, -1));
  let e = LastInputIndex::new(3, 7);
  assert_eq!(
    (e.having_consumed_character, e.not_having_consumed_character),
    (3, 7)
  );
  OK
}

#[test]
fn pc_processed_roundtrip() -> Void {
  let bytecode = [Instruction::accept(), Instruction::jmp(0)];
  // 输入非空: set_input_index 前置 DCHECK 域 (cc:820-821)
  let mut nfa = NfaInterpreter::new(&bytecode, &[0x0061u16; 8], 0, 2, false).unwrap();
  assert_eq!(
    nfa.pc_last_input_index.len(),
    2,
    "去重表长 = 指令数 (cc:362-364)"
  );

  // cc:1440-1451: 双槽独立; 仅当前 input_index 命中
  assert!(!nfa.is_pc_processed(0, ConsumedCharacter::DidConsume));
  nfa.set_input_index(3);
  nfa.mark_pc_processed(0, ConsumedCharacter::DidConsume);
  assert!(nfa.is_pc_processed(0, ConsumedCharacter::DidConsume));
  assert!(
    !nfa.is_pc_processed(0, ConsumedCharacter::DidNotConsume),
    "另一槽不受影响"
  );
  assert!(
    !nfa.is_pc_processed(1, ConsumedCharacter::DidConsume),
    "其他 pc 不受影响"
  );

  // input_index 推进 (新一轮字符) 后旧登记失效
  nfa.set_input_index(4);
  assert!(!nfa.is_pc_processed(0, ConsumedCharacter::DidConsume));

  // DidNotConsume 槽独立登记
  nfa.mark_pc_processed(1, ConsumedCharacter::DidNotConsume);
  assert!(nfa.is_pc_processed(1, ConsumedCharacter::DidNotConsume));
  assert!(!nfa.is_pc_processed(1, ConsumedCharacter::DidConsume));
  OK
}

// ── NfaInterpreter 构造器扫描面 (cc:348-479) ─────────────────────────────────

#[test]
fn nfa_ctor_captureless_lookbehind_track() -> Void {
  // [START_LB(0), 体, WRITE_TABLE(0), END, ACCEPT] — flag 关合法 (仅无捕获后视轨)
  let bytecode = [
    Instruction::start_lookaround(0, false, LookaroundType::Lookbehind),
    Instruction::consume_range(0x0061, 0x007A),
    Instruction::write_look_table(0),
    Instruction::end_lookaround(),
    Instruction::accept(),
  ];
  let nfa = NfaInterpreter::new(&bytecode, &[0x0061], 0, 2, false).unwrap();
  assert!(nfa.only_captureless_lookbehinds);
  // cc:400-402 match_pc=START 下标; cc:417 capture_pc=WRITE 下标+1 (指向 END_LOOKAROUND 落点)
  assert_eq!(
    nfa.lookarounds,
    vec![LookaroundInfo {
      match_pc: 0,
      capture_pc: 3,
      type_: LookaroundType::Lookbehind
    }]
  );
  // cc:447-449 lookbehind_table 预置全 false, 长 = 环视数
  assert_eq!(nfa.lookbehind_table, Some(vec![false]));
  assert_eq!(nfa.lookaround_table, None);
  assert_eq!(nfa.filter_groups_pc, None);
  assert_eq!(nfa.quantifier_count, 0);
  assert_eq!(
    nfa.memory_consumption_per_thread, 0,
    "flag 关原版未初始化, Rust 置 0"
  );
  assert_eq!(nfa.clock, 0);
  assert_eq!(nfa.current_lookaround, -1);
  assert!(!nfa.reverse);
  OK
}

#[test]
fn nfa_ctor_lookahead_switches_track() -> Void {
  // START_LOOKAHEAD → 脱优化轨 (cc:404-406); 需 flag 开 (cc:451 DCHECK)
  let bytecode = [
    Instruction::start_lookaround(1, true, LookaroundType::Lookahead),
    Instruction::consume_range(0x0030, 0x0039),
    Instruction::write_look_table(1),
    Instruction::end_lookaround(),
    Instruction::accept(),
  ];
  let input = [0x0061u16, 0x0062];
  let nfa = NfaInterpreter::new(&bytecode, &input, 0, 2, true).unwrap();
  assert!(!nfa.only_captureless_lookbehinds);
  // 索引 1 → cc:421-423 先补占位 {-1,-1,LOOKBEHIND} 再落位
  assert_eq!(nfa.lookarounds.len(), 2);
  assert_eq!(nfa.lookarounds[0], LookaroundInfo::default());
  assert_eq!(
    nfa.lookarounds[1],
    LookaroundInfo {
      match_pc: 0,
      capture_pc: 3,
      type_: LookaroundType::Lookahead
    }
  );
  assert_eq!(nfa.lookbehind_table, None);
  // cc:457-459 行数 = 环视数, 每行 input_length+1 (含串尾位)
  assert_eq!(
    nfa.lookaround_table,
    Some(vec![vec![false; 3], vec![false; 3]])
  );
  // flag 开单线程预算非零 (cc:462-475)
  assert!(nfa.memory_consumption_per_thread > 0);
  OK
}

#[test]
fn nfa_ctor_capture_in_lookaround_switches_track() -> Void {
  // 环视体内 SET_REGISTER_TO_CP → 脱优化轨 (cc:409-411), 即使方向是 LOOKBEHIND
  let bytecode = [
    Instruction::start_lookaround(0, false, LookaroundType::Lookbehind),
    Instruction::set_register_to_cp(2),
    Instruction::write_look_table(0),
    Instruction::end_lookaround(),
    Instruction::accept(),
  ];
  let nfa = NfaInterpreter::new(&bytecode, &[], 0, 4, true).unwrap();
  assert!(!nfa.only_captureless_lookbehinds, "环视内捕获应脱优化轨");
  assert_eq!(nfa.lookbehind_table, None);
  assert_eq!(
    nfa.lookaround_table,
    Some(vec![vec![false; 1]]),
    "输入长 0 → 每行 1 列"
  );
  // 体内 SET_REGISTER_TO_CP 不影响量词计数与过滤段
  assert_eq!(nfa.quantifier_count, 0);
  assert_eq!(nfa.filter_groups_pc, None);
  OK
}

#[test]
fn nfa_ctor_quantifier_count_and_filter_pc() -> Void {
  // 量词计数 = max(id)+1 (cc:439-443); FILTER 段首 pc 取首条 is_filter 命中
  // (cc:432-437, Instruction::is_filter 白名单不含 FILTER_LOOKAROUND — 陷阱面)
  let bytecode = [
    Instruction::set_quantifier_to_clock(3),
    Instruction::filter_lookaround(0),
    Instruction::set_quantifier_to_clock(0),
    Instruction::filter_group(1),
    Instruction::accept(),
  ];
  let nfa = NfaInterpreter::new(&bytecode, &[], 0, 2, true).unwrap();
  assert_eq!(nfa.quantifier_count, 4);
  // 首条 FILTER_LOOKAROUND (i=1) 不触发登记; 首条 FILTER_GROUP (i=3) 才是段首
  assert_eq!(
    nfa.filter_groups_pc,
    Some(3),
    "FILTER_LOOKAROUND 不入 is_filter 白名单"
  );
  OK
}

#[test]
fn nfa_ctor_skips_last_instruction() -> Void {
  // 原版 cc:390 循环界 `i < length() - 1`: 最后一条指令不参与扫描 (真实行为固化)
  let bytecode = [
    Instruction::accept(),
    Instruction::start_lookaround(0, true, LookaroundType::Lookahead),
  ];
  let nfa = NfaInterpreter::new(&bytecode, &[], 0, 2, true).unwrap();
  assert!(
    nfa.lookarounds.is_empty(),
    "末条 START_LOOKAROUND 不应被扫描 (cc:390 边界)"
  );
  assert!(nfa.only_captureless_lookbehinds, "未扫描即不改轨");
  OK
}

#[test]
fn nfa_ctor_defense() -> Void {
  // WRITE 无前置 START (cc:414 DCHECK) → Rust Err 加固
  let bytecode = [Instruction::write_look_table(0), Instruction::accept()];
  let Err(err) = NfaInterpreter::new(&bytecode, &[], 0, 2, false) else {
    panic!("WRITE 无前置 START 应构造失败");
  };
  assert!(matches!(err, InterpreterError::BadBytecode { .. }), "{err}");

  // 负索引量词: cc:441 id+1 不增 (原版 std::max 语义保留)
  let neg = [
    Instruction {
      opcode: Opcode::SetQuantifierToClock,
      payload: Payload::QuantifierId(-3),
    },
    Instruction {
      opcode: Opcode::SetQuantifierToClock,
      payload: Payload::QuantifierId(2),
    },
    Instruction::accept(),
  ];
  let nfa = NfaInterpreter::new(&neg, &[], 0, 2, true).unwrap();
  assert_eq!(nfa.quantifier_count, 3, "负 id 不回落 (std::max 语义)");
  OK
}

// ── FlushBlockedThreads (cc:1129-1172) ───────────────────────────────────────

fn fresh_nfa(bytecode: &[Instruction]) -> NfaInterpreter<'_> {
  NfaInterpreter::new(bytecode, &[], 0, 0, false).unwrap()
}

fn bare_thread(pc: i32) -> InterpreterThread {
  InterpreterThread {
    pc,
    register_array: vec![],
    lookaround_match_index_array: None,
    quantifier_clock_array: None,
    capture_clock_array: None,
    lookaround_clock_array: None,
    consumed_since_last_quantifier: ConsumedCharacter::DidNotConsume,
  }
}

#[test]
fn flush_consume_range_hit_and_miss() -> Void {
  let bytecode = [
    Instruction::consume_range(0x0061, 0x007A),
    Instruction::accept(),
  ];
  let mut nfa = fresh_nfa(&bytecode);

  // 命中: pc 跳至区间后一条, consumed 置 DidConsume (cc:1156-1162)
  nfa.blocked_threads.push(bare_thread(0));
  nfa.flush_blocked_threads(0x0062).unwrap();
  assert_eq!(nfa.blocked_threads.len(), 0);
  assert_eq!(nfa.active_threads.len(), 1);
  assert_eq!(
    (
      nfa.active_threads[0].pc,
      nfa.active_threads[0].consumed_since_last_quantifier
    ),
    (1, ConsumedCharacter::DidConsume)
  );

  // 未命中: 线程销毁 (cc:1167-1168 DestroyThread → Rust drop)
  nfa.active_threads.clear();
  nfa.blocked_threads.push(bare_thread(0));
  nfa.flush_blocked_threads(0x005A).unwrap(); // 'Z' 不在 [a,z]
  assert!(nfa.active_threads.is_empty() && nfa.blocked_threads.is_empty());

  // 闭区间双端命中
  for edge in [0x0061u16, 0x007A] {
    nfa.blocked_threads.push(bare_thread(0));
    nfa.flush_blocked_threads(edge).unwrap();
    assert_eq!(nfa.active_threads.len(), 1, "边界 {edge:#04X} 闭区间应命中");
    nfa.active_threads.clear();
  }
  OK
}

#[test]
fn flush_range_count_multi_range() -> Void {
  // [RANGE_COUNT 2, '0'-'9', 'a'-'f', ACCEPT]: 命中任一范围, pc 跳区间末 3
  let bytecode = [
    Instruction::range_count(2),
    Instruction::consume_range(0x0030, 0x0039),
    Instruction::consume_range(0x0061, 0x0066),
    Instruction::accept(),
  ];
  let mut nfa = fresh_nfa(&bytecode);

  nfa.blocked_threads.push(bare_thread(0));
  nfa.flush_blocked_threads(0x0066).unwrap(); // 命中第二范围
  assert_eq!(
    nfa.active_threads[0].pc, 3,
    "RANGE_COUNT 命中后 pc = 全区间末 (cc:1148-1158)"
  );
  assert_eq!(
    nfa.active_threads[0].consumed_since_last_quantifier,
    ConsumedCharacter::DidConsume
  );

  nfa.active_threads.clear();
  nfa.blocked_threads.push(bare_thread(0));
  nfa.flush_blocked_threads(0x0067).unwrap(); // 'g' 两范围皆不中 → 销毁
  assert!(nfa.active_threads.is_empty());
  OK
}

#[test]
fn flush_blocked_priority_reverse_activation() -> Void {
  // blocked 高→低序, 逆序激活使 active 呈低→高 (cc:1133-1136)
  let bytecode = [Instruction::consume_any_char(), Instruction::accept()];
  let mut nfa = fresh_nfa(&bytecode);
  let mut hi = bare_thread(0);
  hi.register_array = vec![1, 1]; // 以寄存器内容区分线程
  let mut lo = bare_thread(0);
  lo.register_array = vec![2, 2];
  nfa.blocked_threads.extend([hi, lo]);
  nfa.flush_blocked_threads(0x0061).unwrap();
  assert_eq!(
    nfa
      .active_threads
      .iter()
      .map(|t| t.register_array[0])
      .collect::<Vec<_>>(),
    vec![2, 1],
    "active 需低→高优先级 (最后入队最高优先)"
  );
  OK
}

#[test]
fn flush_blocked_defense() -> Void {
  let bytecode = [
    Instruction::consume_range(0x0061, 0x007A),
    Instruction::accept(),
  ];
  let mut nfa = fresh_nfa(&bytecode);
  // 越界 pc → BadPc (原版编译器不变量, Rust 显式拒)
  nfa.blocked_threads.push(bare_thread(9));
  assert!(matches!(
    nfa.flush_blocked_threads(0x0061).unwrap_err(),
    InterpreterError::BadPc { pc: 9, .. }
  ));
  // RANGE_COUNT 负 num_ranges → BadBytecode (cc:1144)
  let rc = [
    Instruction {
      opcode: Opcode::RangeCount,
      payload: Payload::NumRanges(-1),
    },
    Instruction::accept(),
  ];
  let mut nfa = fresh_nfa(&rc);
  nfa.blocked_threads.push(bare_thread(0));
  assert!(matches!(
    nfa.flush_blocked_threads(0x0061).unwrap_err(),
    InterpreterError::BadBytecode { .. }
  ));
  OK
}

// ── 主循环路由/防御核验 (cc:886-1115 EXP-3b 批段 1 + EXP-3c 批段 2 落地后路由) ──

/// 主循环探针: flag 可选 (cc:435/440/451 DCHECK 域指令 FILTER_*/SET_QUANTIFIER 需
/// flag 开构造), 线程按 new_empty 装配 (flag 开含时钟数组, 原版 cc:886 按值收线程)。
/// 返回主循环原始 Result, 由用例各自断言 Ok/Err 形态 (禁在此 expect 掉 Ok 通道)。
fn probe(thread_pc: i32, insts: &[Instruction], flag: bool) -> Result<(), InterpreterError> {
  let mut nfa = NfaInterpreter::new(insts, &[], 0, 2, flag).unwrap();
  let t = InterpreterThread::new_empty(
    thread_pc,
    2,
    nfa.quantifier_count,
    nfa.lookarounds.len(),
    flag,
    nfa.only_captureless_lookbehinds,
  );
  nfa.run_active_thread(t)
}

#[test]
fn run_active_thread_batch_routing() -> Void {
  // 主循环推迟臂: 全部落地 (EXP-3d 环视表读写已就地完成)
  let deferred_cases: [(&str, Vec<Instruction>, i32, &str, &str); 0] = [];
  for (item, _bytecode, _pc, _orig, _batch) in deferred_cases {
    match probe(_pc, &_bytecode, true) {
      Err(InterpreterError::NotImplemented {
        item: got_item,
        orig: got_orig,
        batch: got_batch,
      }) => {
        assert_eq!(got_item, item, "推迟臂 item 台账对账");
        assert_eq!(got_orig, _orig, "{item} 原版行锚");
        assert_eq!(got_batch, _batch, "{item} 批段路由");
      }
      Ok(()) => panic!("用例 {item}: 推迟臂不应执行成功 (假绿)"),
      Err(other) => panic_expect(item, &other),
    }
  }

  // EXP-3b/EXP-3c/EXP-3d 实现臂路由核验: 全部脱离推迟通道
  let landed_cases: Vec<(&str, Vec<Instruction>, bool)> = vec![
    (
      "WRITE_LOOKAROUND_TABLE",
      vec![
        Instruction::start_lookaround(0, false, LookaroundType::Lookbehind),
        Instruction::write_look_table(0),
        Instruction::accept(),
      ],
      true,
    ),
    (
      "READ_LOOKAROUND_TABLE",
      vec![
        Instruction::read_look_table(0, false, LookaroundType::Lookbehind),
        Instruction::start_lookaround(0, false, LookaroundType::Lookbehind),
        Instruction::write_look_table(0),
        Instruction::accept(),
      ],
      true,
    ),
    (
      "CONSUME_RANGE",
      vec![Instruction::consume_any_char(), Instruction::accept()],
      false,
    ),
    (
      "RANGE_COUNT",
      vec![Instruction::range_count(1), Instruction::accept()],
      false,
    ),
    (
      "ASSERTION",
      vec![
        Instruction::assertion(AssertionType::Boundary),
        Instruction::accept(),
      ],
      false,
    ),
    (
      "FORK",
      vec![Instruction::fork(1), Instruction::accept()],
      false,
    ),
    (
      "JMP",
      vec![Instruction::jmp(1), Instruction::accept()],
      false,
    ),
    ("ACCEPT", vec![Instruction::accept()], false),
    (
      "SET_QUANTIFIER_TO_CLOCK",
      vec![
        Instruction::set_quantifier_to_clock(0),
        Instruction::accept(),
      ],
      true,
    ),
    (
      "CLEAR_REGISTER",
      vec![Instruction::clear_register(0), Instruction::accept()],
      false,
    ),
    (
      "SET_REGISTER_TO_CP",
      vec![Instruction::set_register_to_cp(0), Instruction::accept()],
      false,
    ),
    (
      "BEGIN_LOOP",
      vec![Instruction::begin_loop(), Instruction::accept()],
      false,
    ),
    (
      "END_LOOP",
      vec![Instruction::end_loop(), Instruction::accept()],
      false,
    ),
    (
      "START_LOOKAROUND",
      vec![
        Instruction::start_lookaround(0, true, LookaroundType::Lookahead),
        Instruction::accept(),
      ],
      // 前视脱优化轨需 flag 开 (cc:451 DCHECK 域, 构造器轨初始化)
      true,
    ),
    (
      "END_LOOKAROUND",
      vec![Instruction::end_lookaround(), Instruction::accept()],
      false,
    ),
  ];
  for (label, bytecode, flag) in landed_cases {
    let res = probe(0, &bytecode, flag);
    if let Err(InterpreterError::NotImplemented { batch, .. }) = &res {
      panic!("用例 {label}: EXP-3b 已落臂不应走推迟通道 (batch={batch})");
    }
    assert!(res.is_ok(), "{label} 已落臂应执行成功: {res:?}");
  }
  OK
}

fn panic_expect(label: &str, err: &InterpreterError) -> ! {
  panic!("用例 {label}: 期望指定变体, 实得 {err}");
}

#[test]
fn run_active_thread_dispatch_defense() -> Void {
  let bytecode = [Instruction::accept(), Instruction::jmp(0)];

  // pc 越界/负 → BadPc (cc:888-889 SBXCHECK 等价)
  match probe(9, &bytecode, false) {
    Err(InterpreterError::BadPc { pc: 9, len: 2, .. }) => {}
    Ok(()) => panic!("BadPc: pc 越界不应执行成功"),
    Err(other) => panic_expect("BadPc", &other),
  }
  match probe(-1, &bytecode, false) {
    Err(InterpreterError::BadPc { pc: -1, .. }) => {}
    Ok(()) => panic!("BadPc 负 pc: 不应执行成功"),
    Err(other) => panic_expect("BadPc 负 pc", &other),
  }

  // opcode/payload 错配 (C++ union 无此态, tagged Payload 显式拒)
  let mismatched = [
    Instruction {
      opcode: Opcode::Accept,
      payload: Payload::Pc(0),
    },
    Instruction::accept(),
  ];
  match probe(0, &mismatched, false) {
    Err(InterpreterError::BadBytecode { .. }) => {}
    Ok(()) => panic!("payload 错配: 不应执行成功"),
    Err(other) => panic_expect("payload 错配", &other),
  }

  // FILTER_* 防御臂 (EXP-3b 落地面: 原版 cc:1020-1024 UNREACHABLE 收敛
  // BadBytecode 加固, 与 FilterGroups::run 先例同款; flag 开 — cc:435 DCHECK 域)
  let filter_arm = [Instruction::filter_group(0), Instruction::accept()];
  match probe(0, &filter_arm, true) {
    Err(InterpreterError::BadBytecode { item, orig }) => {
      assert!(item.contains("主循环混入 FILTER_*"), "{item}");
      assert_eq!(orig, "cc:1020-1024");
    }
    Ok(()) => panic!("FILTER_* 防御臂: 不应执行成功"),
    Err(other) => panic_expect("FILTER_* 防御臂", &other),
  }

  // FORK 负目标 → BadBytecode (cc:922 装配点加固, 未入队状态未变)
  let neg_fork = [Instruction::fork(-1), Instruction::accept()];
  match probe(0, &neg_fork, false) {
    Err(InterpreterError::BadBytecode { item, .. }) => {
      assert!(item.contains("FORK 臂 目标 pc 为负"), "{item}");
    }
    Ok(()) => panic!("FORK 负目标: 不应执行成功"),
    Err(other) => panic_expect("FORK 负目标", &other),
  }
  OK
}

// ── 调度查询面 (cc:818-824/1174/1176-1179) ───────────────────────────────────

#[test]
fn match_and_memory_surface() -> Void {
  let bytecode = [
    Instruction::accept(),
    Instruction::set_quantifier_to_clock(1),
    Instruction::accept(),
  ];
  let mut nfa = NfaInterpreter::new(&bytecode, &[0x0061, 0x0062], 1, 2, true).unwrap();

  // cc:818-824 SetInputIndex
  nfa.set_input_index(2);
  assert_eq!(nfa.input_index, 2);

  // cc:1174 FoundMatch = best_match_thread 在场
  assert!(!nfa.found_match());
  nfa.best_match_thread = Some(bare_thread(0));
  assert!(nfa.found_match());
  nfa.best_match_thread = None;

  // cc:1176-1179 预算 × 线程数 (flag 开, 量词计 2 = max(1)+1)
  assert_eq!(nfa.quantifier_count, 2);
  assert!(nfa.memory_consumption_per_thread >= size_of::<InterpreterThread>() as u64);
  assert_eq!(nfa.approximate_total_memory_usage(), 0);
  nfa.active_threads.push(bare_thread(0));
  nfa.blocked_threads.push(bare_thread(1));
  assert_eq!(
    nfa.approximate_total_memory_usage(),
    2 * nfa.memory_consumption_per_thread
  );

  // flag 关预算 0 (原版未初始化面, 消费仅 flag 开); 量词指令 cc:440 DCHECK 需 flag 开,
  // 此处用无段指令字节码 (原版编译器不变量)
  let plain = [Instruction::accept(), Instruction::accept()];
  let off = NfaInterpreter::new(&plain, &[], 0, 2, false).unwrap();
  assert_eq!(off.memory_consumption_per_thread, 0);
  assert_eq!(off.approximate_total_memory_usage(), 0);
  OK
}
