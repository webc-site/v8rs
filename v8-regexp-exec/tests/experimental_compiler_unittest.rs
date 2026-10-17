//! 实验性正则编译器 lowering 规约测试 (v8-regexp-exec / experimental::compiler)。
//!
//! 规约来源: `v8/src/regexp/experimental/experimental-compiler.cc` 逐行语义。实测
//! `v8/test/unittests/regexp/` 与 `v8/test/cctest/` 下无 experimental-compiler 专有用例
//! (grep 0 命中; 实验引擎经 mjsunit regexp-experimental.js 间接消费), 故按任务规范
//! 依源码语义手工推演期望指令向量; 断言强度守恒 —— 每条用例比对完整
//! `Vec<Instruction>` (opcode + payload 逐值), 不降级为长度/opcode 序列/存在性。
//!
//! 推演约定 (与原版逐字一致):
//! - FORK/JMP/FILTER_CHILD 的 payload.pc: 标签未 Bind 时为补丁链头 (初值 -1,
//!   cc:355-372), Bind 后整链改写为落点下标 (cc:333-350); 断言只比对终值。
//! - 下标自 0 计。非 sticky 且不锚定 ⟹ 前导 /.*?/ (`CompileNonGreedyStar`,
//!   cc:551-555 + cc:863-891), 六连结构为
//!   `FORK 2 / JMP 6 / BEGIN_LOOP / CONSUME_ANY / END_LOOP / FORK 2`。
//! - 字符类相邻区间由 `CharacterRange::Canonicalize` 合并 (syntax crate 承接原版)。

use aok::{OK, Void};
use v8_flags::Flags as V8Flags;
use v8_regexp_exec::experimental::{
  bytecode::{Instruction, Opcode, Payload, dump},
  compiler::{can_be_handled, compile},
};
use v8_regexp_syntax::{
  AssertionType, Disjunction, LookaroundType, RegExpFlags, Tree,
  flags_generated::{GLOBAL, LINEAR, MULTILINE, STICKY, UNICODE},
  parse,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// `experimental_regexp_engine_capture_group_opt = false` (v8-flags defs.rs:5968
/// 默认值; 原版 cc:187,599,1030,1046,1179 读取的全局量)
fn off() -> V8Flags {
  V8Flags::default()
}

/// `experimental_regexp_engine_capture_group_opt = true`
fn on() -> V8Flags {
  let mut f = V8Flags::default();
  f.set_experimental_regexp_engine_capture_group_opt(true)
    .expect("flag setter");
  f
}

fn parse_tree(pattern: &str, flags: RegExpFlags) -> (Tree, i32) {
  let data = parse(pattern, flags).unwrap_or_else(|e| panic!("{pattern:?} 解析失败: {e:?}"));
  (data.tree, data.capture_count as i32)
}

fn handled(pattern: &str, flags: RegExpFlags, v8_flags: &V8Flags) -> bool {
  let (tree, capture_count) = parse_tree(pattern, flags);
  can_be_handled(&tree, flags, capture_count, v8_flags)
}

fn code(pattern: &str, flags: RegExpFlags, v8_flags: &V8Flags) -> Vec<Instruction> {
  let (tree, _) = parse_tree(pattern, flags);
  compile(&tree, flags, v8_flags)
}

/// 简写: CONSUME_RANGE [from, to]
fn cr(from: u16, to: u16) -> Instruction {
  Instruction::consume_range(from, to)
}

/// 简写: CONSUME_ANY_CHAR = CONSUME_RANGE [0x0000, 0xFFFF] (bytecode h:157)
fn any() -> Instruction {
  cr(0x0000, 0xFFFF)
}

const NONE: RegExpFlags = RegExpFlags(0);

/// 前导 /.*?/ 六连 (非 sticky 且未锚定表达式的统一前缀, cc:551-555)
fn prefix_star() -> Vec<Instruction> {
  vec![
    Instruction::fork(2),
    Instruction::jmp(6),
    Instruction::begin_loop(),
    any(),
    Instruction::end_loop(),
    Instruction::fork(2),
  ]
}

// ── CanBeHandled: 标志白名单 (v8: cc:40-50) ─────────────────────────────────

/// 白名单 kGlobal|kSticky|kMultiline|kDotAll|kLinear 内可处理; 任一白名单外
/// 标志 (含 kUnicode) 整体拒绝 (cc:49 `(flags & ~kAllowedFlags) == 0`)
#[test]
fn suitable_flags_whitelist() -> Void {
  for f in [
    NONE,
    RegExpFlags(GLOBAL),
    RegExpFlags(STICKY),
    RegExpFlags(MULTILINE),
    RegExpFlags(LINEAR),
    RegExpFlags(GLOBAL | STICKY | MULTILINE | LINEAR),
  ] {
    assert!(handled("a", f, &off()), "flag {f:?} 应在白名单内");
  }
  for f in [
    RegExpFlags(UNICODE),
    RegExpFlags(GLOBAL | UNICODE),
    RegExpFlags(STICKY | MULTILINE | UNICODE),
  ] {
    assert!(
      !handled("a", f, &off()),
      "flag {f:?} 含白名单外位, 须拒绝 (原版 kSupportsUnicode=false, experimental.h:65)"
    );
  }
  OK
}

// ── CanBeHandled: 不支持构造 (v8: cc:74-83, 144-148, 197-201) ───────────────

/// 回引 / 独占量词恒拒; /v 类运算经标志面白名单先行拒绝 (cc:47-48
/// static_assert 面: 类运算节点只能在带白名单外标志下解析)
#[test]
fn unsupported_constructs_rejected() -> Void {
  // VisitBackReference (cc:197-201): 无回溯不可实现
  assert!(!handled(r"(a)\1", NONE, &off()));
  assert!(!handled(r"(a)(b)\2", NONE, &off()));
  // VisitQuantifier POSSESSIVE (cc:144-148 TODO(mbid,v8:10765))
  for p in ["a?+", "a*+", "a++", "a{2,3}+", "(?:ab)?+"] {
    assert!(!handled(p, NONE, &off()), "{p:?} 独占量词须拒绝");
    assert!(!handled(p, NONE, &on()), "{p:?} 独占量词开 opt flag 仍拒");
  }
  // VisitClassSetExpression (cc:79-83) / 含字符串 ClassSetOperand (cc:74-78):
  // 依赖 /v, 标志先行拒绝
  assert!(!handled("[a&&b]", RegExpFlags(UNICODE), &off()));
  OK
}

/// 复制因子预算 (cc:106 kMaxReplicationFactor=16; cc:111-116 单节点上限;
/// cc:122-132 local_replication 三通道; cc:134-138 嵌套乘积)
#[test]
fn replication_factor_budget() -> Void {
  // 单节点 min/max 超限 (cc:111-116)
  assert!(!handled("a{17}", NONE, &off()));
  assert!(!handled("a{17,}", NONE, &off()));
  assert!(!handled("a{0,17}", NONE, &off()));
  assert!(handled("a{16}", NONE, &off()));
  assert!(handled("a{0,16}", NONE, &off()));
  // max==∞ ∧ min>0 ∧ min_match>0 ⟹ local = max(min,1) (cc:124-126)
  assert!(handled("a{16,}", NONE, &off()));
  // max==∞ 且体可空 (min_match==0) ⟹ local = min+1 (cc:128-130)
  assert!(handled("(?:a?){3,}", NONE, &off())); // local = 4
  assert!(handled("(?:a?){14,}", NONE, &off())); // local = 15 ≤ 16
  assert!(handled("(?:a?){15,}", NONE, &off())); // local = 16, 16 > 16 不成立 ⟹ 放行
  assert!(!handled("(?:a?){16,}", NONE, &off())); // min 超限 (cc:112)
  // 嵌套乘积 (外层 × 内层 > 16 ⟹ 拒; 外层先入因子)
  assert!(handled("(?:a{2}){8}", NONE, &off())); // 8*2 = 16
  assert!(!handled("(?:a{2}){9}", NONE, &off())); // 9*2 = 18
  assert!(!handled("(?:(?:a{2}){3}){9}", NONE, &off())); // 9*3*2 = 54
  // 可空 ∞ 星 local = 0+1 = 1 ⟹ 任意并存不涨预算 (cc:128-130)
  assert!(handled(
    "a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*",
    NONE,
    &off()
  ));
  OK
}

// ── CanBeHandled: 环视与 modifier (v8: cc:161-195) ─────────────────────────

/// global/sticky 与环视互斥 (cc:180-183); capture_group_opt 关时拒全部
/// lookahead 与含捕获的 lookbehind (cc:185-191)
#[test]
fn lookaround_acceptance_matrix() -> Void {
  for (pat, want_off, want_on) in [
    // 非捕获 lookbehind: 两态皆可 (cc:187 条件要求 LOOKAHEAD 或 capture_count>0)
    ("a(?<=b)", true, true),
    ("a(?<!b)", true, true),
    // lookahead: flag 关全拒, flag 开可
    ("a(?=b)", false, true),
    ("a(?!b)", false, true),
    // 含捕获 lookbehind / lookahead: flag 关拒, flag 开可 (捕获自动机相 cc:649-656)
    ("a(?<=(b))", false, true),
    ("a(?=(b))", false, true),
  ] {
    assert_eq!(
      handled(pat, NONE, &off()),
      want_off,
      "{pat:?} @ capture_group_opt=off"
    );
    assert_eq!(
      handled(pat, NONE, &on()),
      want_on,
      "{pat:?} @ capture_group_opt=on"
    );
  }
  for f in [
    RegExpFlags(GLOBAL),
    RegExpFlags(STICKY),
    RegExpFlags(GLOBAL | STICKY),
  ] {
    assert!(!handled("a(?<=b)", f, &off()), "{f:?} + lookbehind 须拒");
    assert!(!handled("a(?=b)", f, &on()), "{f:?} + lookahead 须拒");
  }
  OK
}

/// VisitGroup: modifier 标志须在白名单内 (cc:161-176)。(?i:…) 的 kIgnoreCase
/// 不在白名单 ⟹ 整树拒; (?m:…) / (?s:…) 在白名单内 ⟹ 放行。
/// (注: (?u:…) 在解析层即 InvalidGroup, 属 syntax crate 面, 不到本判定)
#[test]
fn group_modifier_flags_whitelist() -> Void {
  assert!(handled("(?m:a)", NONE, &off()));
  assert!(handled("(?s:a)", NONE, &off()));
  assert!(!handled("(?i:a)", NONE, &off()));
  assert!(!handled("(?m:a)", RegExpFlags(UNICODE), &off()));
  OK
}

/// `capture_count` 形参: 原版 Check(cc:30-35) 从不读取 ⟹ 同一 tree 传不同值结果
/// 不变 (h:24 签名 1:1 承接的登记项)
#[test]
fn capture_count_argument_is_not_consumed() -> Void {
  let (tree, real) = parse_tree("(a)(b)", NONE);
  let want = can_be_handled(&tree, NONE, real, &off());
  assert!(want);
  for bogus in [0, 1, real + 7, i32::MAX] {
    assert_eq!(can_be_handled(&tree, NONE, bogus, &off()), want);
  }
  OK
}

// ── lowering: 主干与原子 (v8: cc:546-607, 808-820) ──────────────────────────

/// sticky + 单字符: 无前导 /.*?/, 仅寄存器 0/1 与 CONSUME_RANGE (cc:550,558-561)
#[test]
fn sticky_literal_program() -> Void {
  assert_eq!(
    code("a", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0),
      cr(0x61, 0x61),
      Instruction::set_register_to_cp(1),
      Instruction::accept(),
    ]
  );
  OK
}

/// 非 sticky 且不锚定: 前导 /.*?/ = CompileNonGreedyStar(ConsumeAnyChar)
/// (cc:551-555 + cc:863-891)
#[test]
fn non_sticky_prefix_star() -> Void {
  let mut expected = prefix_star();
  expected.extend_from_slice(&[
    Instruction::set_register_to_cp(0), // 6
    cr(0x61, 0x61),                     // 7
    Instruction::set_register_to_cp(1), // 8
    Instruction::accept(),              // 9
  ]);
  assert_eq!(code("a", NONE, &off()), expected);
  // sticky 无此前缀: 长度差恰为六连
  assert_eq!(
    code("a", RegExpFlags(STICKY), &off()).len(),
    expected.len() - 6
  );
  OK
}

/// `^a`: 树锚定串首 ⟹ 无前导 /.*?/ (cc:550 IsCertainlyAnchoredAtStart)
#[test]
fn anchored_start_skips_prefix_star() -> Void {
  assert_eq!(
    code("^a", NONE, &off()),
    vec![
      Instruction::set_register_to_cp(0),
      Instruction::assertion(AssertionType::StartOfInput),
      cr(0x61, 0x61),
      Instruction::set_register_to_cp(1),
      Instruction::accept(),
    ]
  );
  OK
}

/// 多字符: VisitAtom 逐字符 CONSUME_RANGE (cc:808-820)
#[test]
fn atom_multi_char_expands_to_ranges() -> Void {
  assert_eq!(
    code("ab", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0),
      cr(0x61, 0x61),
      cr(0x62, 0x62),
      Instruction::set_register_to_cp(1),
      Instruction::accept(),
    ]
  );
  OK
}

// ── lowering: 析取 (v8: cc:661-708) ────────────────────────────────────────

/// 空析取 (0 分支, 直接构造 AST 绕过 parser 的 ≥2 分支约束): cc:689-693
/// `if (alt_num == 0) { Fail(); return; }` — 恒不匹配, 不产任何分支体
#[test]
fn empty_disjunction_compiles_to_fail() -> Void {
  let tree = Tree::Disjunction(Box::new(Disjunction {
    alternatives: vec![],
    // min_match/max_match: 编译器全路径不读取 (逐行核对 cc:546-607, 710-715),
    // parser 不可产出该形态, 此处仅为构造面占位
    min_match: 0,
    max_match: 0,
  }));
  assert_eq!(
    compile(&tree, RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::fail(),                // 1 ← alt_num == 0
      Instruction::set_register_to_cp(1), // 2
      Instruction::accept(),              // 3
    ]
  );
  OK
}

/// `a|`: 空分支 (Empty alternative) — 末分支为 Empty ⟹ gen_alt(1) 不产码,
/// FORK(tail) 与 JMP(end) 汇于同一下标 (cc:695-707 Bind 序); VisitEmpty cc:1162 无指令
#[test]
fn empty_alternative_falls_through() -> Void {
  assert_eq!(
    code("a|", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::fork(4),               // 1 → tail
      cr(0x61, 0x61),                     // 2 分支 1 "a"
      Instruction::jmp(4),                // 3 → end
      // tail == end == 4: 空分支直落
      Instruction::set_register_to_cp(1), // 4
      Instruction::accept(),              // 5
    ]
  );
  OK
}

/// `a|b` / `a|b|c`: FORK 下一分支 / 分支体 / JMP 汇合, 末分支直落 (cc:667-687
/// 大注释: FORK 派生线程优先级低 ⟹ 分支序即回溯优先级序)
#[test]
fn disjunction_program() -> Void {
  assert_eq!(
    code("a|b", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::fork(4),               // 1 → 分支 2 起点
      cr(0x61, 0x61),                     // 2 分支 1
      Instruction::jmp(5),                // 3 → 汇合
      cr(0x62, 0x62),                     // 4 分支 2 (直落)
      Instruction::set_register_to_cp(1), // 5
      Instruction::accept(),              // 6
    ]
  );
  assert_eq!(
    code("a|b|c", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::fork(4),               // 1 → 分支 2
      cr(0x61, 0x61),                     // 2
      Instruction::jmp(8),                // 3 → 汇合
      Instruction::fork(7),               // 4 → 分支 3
      cr(0x62, 0x62),                     // 5
      Instruction::jmp(8),                // 6 → 汇合
      cr(0x63, 0x63),                     // 7 末分支直落
      Instruction::set_register_to_cp(1), // 8
      Instruction::accept(),              // 9
    ]
  );
  OK
}

// ── lowering: 量词 (v8: cc:834-1024, 1026-1122) ────────────────────────────

/// `a{3}` / `a{1,3}` / `a{1,3}?`: min 次体复制 + 可选重复模板
/// (cc:1088-1094 + cc:893-924 / cc:926-968)
#[test]
fn finite_repetition_programs() -> Void {
  assert_eq!(
    code("a{3}", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      cr(0x61, 0x61),                     // 1
      cr(0x61, 0x61),                     // 2
      cr(0x61, 0x61),                     // 3
      Instruction::set_register_to_cp(1), // 4
      Instruction::accept(),              // 5
    ]
  );
  // max-min = 2 轮贪婪可选重复: {FORK end / BEGIN_LOOP / 体 / END_LOOP} ×2, 汇合
  assert_eq!(
    code("a{1,3}", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      cr(0x61, 0x61),                     // 1 min 必重复
      Instruction::fork(10),              // 2 → 汇合 10
      Instruction::begin_loop(),          // 3
      cr(0x61, 0x61),                     // 4
      Instruction::end_loop(),            // 5
      Instruction::fork(10),              // 6 → 汇合 10
      Instruction::begin_loop(),          // 7
      cr(0x61, 0x61),                     // 8
      Instruction::end_loop(),            // 9
      Instruction::set_register_to_cp(1), // 10
      Instruction::accept(),              // 11
    ]
  );
  // 非贪婪: {FORK body_i / JMP end / body_i: BEGIN_LOOP / 体 / END_LOOP} ×2
  assert_eq!(
    code("a{1,3}?", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      cr(0x61, 0x61),                     // 1
      Instruction::fork(4),               // 2 → body_0
      Instruction::jmp(12),               // 3 → end
      Instruction::begin_loop(),          // 4 = body_0
      cr(0x61, 0x61),                     // 5
      Instruction::end_loop(),            // 6
      Instruction::fork(9),               // 7 → body_1
      Instruction::jmp(12),               // 8 → end
      Instruction::begin_loop(),          // 9 = body_1
      cr(0x61, 0x61),                     // 10
      Instruction::end_loop(),            // 11
      Instruction::set_register_to_cp(1), // 12 = end
      Instruction::accept(),              // 13
    ]
  );
  OK
}

/// `a+` / `a{2,}` / `a+?` / `a{2,}?`: min>0 ∧ max==∞ ∧ min_match>0 ⟹ 归约非空
/// plus (cc:1055-1086; cc:984-1005 / cc:1007-1024 —— plus 不复制体, 嵌套线性)
#[test]
fn greedy_plus_reduction_program() -> Void {
  assert_eq!(
    code("a+", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      cr(0x61, 0x61),                     // 1 = begin
      Instruction::fork(4),               // 2 → end
      Instruction::jmp(1),                // 3 → begin
      Instruction::set_register_to_cp(1), // 4 = end
      Instruction::accept(),              // 5
    ]
  );
  // min-1 = 1 次必重复, 最后一次并入 plus 循环体 (cc:1065-1069)
  assert_eq!(
    code("a{2,}", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      cr(0x61, 0x61),                     // 1 必重复
      cr(0x61, 0x61),                     // 2 = begin
      Instruction::fork(5),               // 3 → end
      Instruction::jmp(2),                // 4 → begin
      Instruction::set_register_to_cp(1), // 5 = end
      Instruction::accept(),              // 6
    ]
  );
  // 非空非贪婪 plus: begin: <body>; FORK begin (cc:1007-1024)
  assert_eq!(
    code("a+?", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      cr(0x61, 0x61),                     // 1 = begin
      Instruction::fork(1),               // 2 → begin
      Instruction::set_register_to_cp(1), // 3
      Instruction::accept(),              // 4
    ]
  );
  assert_eq!(
    code("a{2,}?", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      cr(0x61, 0x61),                     // 1 必重复
      cr(0x61, 0x61),                     // 2 = begin
      Instruction::fork(2),               // 3 → begin
      Instruction::set_register_to_cp(1), // 4
      Instruction::accept(),              // 5
    ]
  );
  OK
}

/// flag 开 + 非空 plus 归约通道: `a{2,}` sticky ⟹ min-1=1 次必重复 (每次体首产
/// 时钟) + CompileNonNullableGreedyPlus (cc:1065-1078 + cc:984-1005)。
/// 时钟重映射 id 同一量词恒同值 (cc:1178-1184 try_emplace); 体无捕获 ⟹ filter 相跳过
#[test]
fn flag_on_reduced_plus_emits_clock_per_repetition() -> Void {
  assert_eq!(
    code("a{2,}", RegExpFlags(STICKY), &on()),
    vec![
      Instruction::set_register_to_cp(0),      // 0
      Instruction::set_quantifier_to_clock(0), // 1 必重复 emit_body
      cr(0x61, 0x61),                          // 2
      Instruction::set_quantifier_to_clock(0), // 3 = begin 处 emit_body
      cr(0x61, 0x61),                          // 4
      Instruction::fork(7),                    // 5 → end
      Instruction::jmp(3),                     // 6 → begin
      Instruction::set_register_to_cp(1),      // 7 = end
      Instruction::accept(),                   // 8
    ]
  );
  // 全程序无 FILTER_* (体内无捕获, cc:464-468 CaptureRegisters 空判)
  let program = code("a{2,}", RegExpFlags(STICKY), &on());
  assert!(!program.iter().any(|i| i.is_filter()));
  OK
}

/// `a*` / `a*?`: min==0 不可归约 ⟹ 0 次必重复 + 星模板 (cc:1088-1118,
/// cc:834-861, cc:863-891)
#[test]
fn star_programs() -> Void {
  assert_eq!(
    code("a*", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::fork(6),               // 1 → end
      Instruction::begin_loop(),          // 2
      cr(0x61, 0x61),                     // 3
      Instruction::end_loop(),            // 4
      Instruction::jmp(1),                // 5 → begin
      Instruction::set_register_to_cp(1), // 6 = end
      Instruction::accept(),              // 7
    ]
  );
  assert_eq!(
    code("a*?", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::fork(3),               // 1 → body
      Instruction::jmp(7),                // 2 → end
      Instruction::begin_loop(),          // 3 = body
      cr(0x61, 0x61),                     // 4
      Instruction::end_loop(),            // 5
      Instruction::fork(3),               // 6 → body
      Instruction::set_register_to_cp(1), // 7 = end
      Instruction::accept(),              // 8
    ]
  );
  OK
}

// ── lowering: 捕获与寄存器 (v8: cc:822-832, 1124-1139) ──────────────────────

/// `(a)`: VisitCapture 产 start/end 寄存器 (cc:1128-1138)。捕获组序号自 1 起
/// (0 留给整段匹配, 与原版 regexp-ast 一致) ⟹ 首组寄存器 = 2/3, 与串首/串尾
/// 寄存器 0/1 互不重合
#[test]
fn capture_program() -> Void {
  assert_eq!(
    code("(a)", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0 串首
      Instruction::set_register_to_cp(2), // 1 捕获 1 begin
      cr(0x61, 0x61),                     // 2
      Instruction::set_register_to_cp(3), // 3 捕获 1 end
      Instruction::set_register_to_cp(1), // 4 串尾
      Instruction::accept(),              // 5
    ]
  );
  // 第二组寄存器为 4/5
  assert_eq!(
    code("(a)(b)", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::set_register_to_cp(2), // 1
      cr(0x61, 0x61),                     // 2
      Instruction::set_register_to_cp(3), // 3
      Instruction::set_register_to_cp(4), // 4
      cr(0x62, 0x62),                     // 5
      Instruction::set_register_to_cp(5), // 6
      Instruction::set_register_to_cp(1), // 7
      Instruction::accept(),              // 8
    ]
  );
  // 非捕获组 (?:…) 不产寄存器 (cc:1141-1144 VisitGroup 直接下钻)
  assert_eq!(
    code("(?:a)", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0),
      cr(0x61, 0x61),
      Instruction::set_register_to_cp(1),
      Instruction::accept(),
    ]
  );
  OK
}

/// flag 关: 量词体含捕获 ⟹ 每轮重复前 CLEAR_REGISTER 体内 begin 寄存器
/// (cc:1045-1053 emit_body → ClearRegisters; cc:822-832 只清 begin:
/// begin 未定义即视为捕获未定义, end 不必清)
#[test]
fn capture_in_star_clears_registers() -> Void {
  // 捕获在量词体内 ⟹ 每轮先清 begin=2
  assert_eq!(
    code("(a)*", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::fork(9),               // 1 → end
      Instruction::begin_loop(),          // 2
      Instruction::clear_register(2),     // 3 emit_body: 清捕获 1 begin
      Instruction::set_register_to_cp(2), // 4
      cr(0x61, 0x61),                     // 5
      Instruction::set_register_to_cp(3), // 6
      Instruction::end_loop(),            // 7
      Instruction::jmp(1),                // 8 → begin
      Instruction::set_register_to_cp(1), // 9 = end
      Instruction::accept(),              // 10
    ]
  );
  // 前序捕获存在 ⟹ 体内区间 [4,5] ⟹ 只清 4
  assert_eq!(
    code("(a)(b)*", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::set_register_to_cp(2), // 1
      cr(0x61, 0x61),                     // 2
      Instruction::set_register_to_cp(3), // 3
      Instruction::fork(12),              // 4 → end
      Instruction::begin_loop(),          // 5
      Instruction::clear_register(4),     // 6
      Instruction::set_register_to_cp(4), // 7
      cr(0x62, 0x62),                     // 8
      Instruction::set_register_to_cp(5), // 9
      Instruction::end_loop(),            // 10
      Instruction::jmp(4),                // 11 → begin
      Instruction::set_register_to_cp(1), // 12 = end
      Instruction::accept(),              // 13
    ]
  );
  // 捕获包住量词 `(a*)`: 量词体 (Atom) 无捕获 ⟹ 无 CLEAR_REGISTER
  assert_eq!(
    code("(a*)", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::set_register_to_cp(2), // 1 捕获 1 begin
      Instruction::fork(7),               // 2 → 内层 end
      Instruction::begin_loop(),          // 3
      cr(0x61, 0x61),                     // 4
      Instruction::end_loop(),            // 5
      Instruction::jmp(2),                // 6 → 内层 begin
      Instruction::set_register_to_cp(3), // 7 = 内层 end; 捕获 1 end
      Instruction::set_register_to_cp(1), // 8 串尾
      Instruction::accept(),              // 9
    ]
  );
  // 嵌套两层量词: 外层体含捕获 ⟹ 清; 内层体 (Atom) 无捕获 ⟹ 不清
  assert_eq!(
    code("(a*)*", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::fork(13),              // 1 → 外层 end
      Instruction::begin_loop(),          // 2
      Instruction::clear_register(2),     // 3 外层 emit_body
      Instruction::set_register_to_cp(2), // 4 捕获 1 begin
      Instruction::fork(10),              // 5 → 内层 end
      Instruction::begin_loop(),          // 6
      cr(0x61, 0x61),                     // 7
      Instruction::end_loop(),            // 8
      Instruction::jmp(5),                // 9 → 内层 begin
      Instruction::set_register_to_cp(3), // 10 = 内层 end; 捕获 1 end
      Instruction::end_loop(),            // 11
      Instruction::jmp(1),                // 12 → 外层 begin
      Instruction::set_register_to_cp(1), // 13 = 外层 end; 串尾
      Instruction::accept(),              // 14
    ]
  );
  OK
}

// ── lowering: 字符类 (v8: cc:736-789) ───────────────────────────────────────

/// 多区间 ⟹ RANGE_COUNT + 逐区间 CONSUME_RANGE (cc:757-760); 相邻区间经
/// Canonicalize 合并为单区间后省 RANGE_COUNT (cc:741)
#[test]
fn class_ranges_program() -> Void {
  assert_eq!(
    code("[ac]", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::range_count(2),        // 1
      cr(0x61, 0x61),                     // 2
      cr(0x63, 0x63),                     // 3
      Instruction::set_register_to_cp(1), // 4
      Instruction::accept(),              // 5
    ]
  );
  // [ab] 相邻合并为 [a, b] 单区间
  assert_eq!(
    code("[ab]", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0),
      cr(0x61, 0x62),
      Instruction::set_register_to_cp(1),
      Instruction::accept(),
    ]
  );
  OK
}

/// 取补类: Negate 在 [0, kMaxCodePoint] 全域求补, 再按 kMaxSupportedCodepoint
/// 截断上界 (cc:742-749 + cc:782-785)
#[test]
fn negated_class_truncates_to_bmp() -> Void {
  assert_eq!(
    code("[^x]", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0), // 0
      Instruction::range_count(2),        // 1
      cr(0x0000, 0x0077),                 // 2 [0, 'x'-1]
      cr(0x0079, 0xFFFF),                 // 3 ['x'+1, 0x10FFFF] 截断 0xFFFF
      Instruction::set_register_to_cp(1), // 4
      Instruction::accept(),              // 5
    ]
  );
  // 补集单区间 (下界已在 BMP 内)
  assert_eq!(
    code("[^\\x00-\\x7f]", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0),
      cr(0x0080, 0xFFFF), // [0x80, 0x10FFFF] 截断
      Instruction::set_register_to_cp(1),
      Instruction::accept(),
    ]
  );
  OK
}

/// `[^\x00-\uFFFF]`: 补集仅 BMP 外 ⟹ from/to 均 > kMaxSupportedCodepoint,
/// 原版 cc:774-778 特例直排 FAIL (空区间 CONSUME_RANGE, 恒不匹配)
#[test]
fn negated_full_bmp_class_always_fails() -> Void {
  assert_eq!(
    code("[^\\x00-\\uFFFF]", RegExpFlags(STICKY), &off()),
    vec![
      Instruction::set_register_to_cp(0),
      Instruction::fail(), // CONSUME_RANGE [0xFFFF, 0x0000] (bytecode h:159-163)
      Instruction::set_register_to_cp(1),
      Instruction::accept(),
    ]
  );
  OK
}

// ── lowering: 环视两相 (v8: cc:625-659) ─────────────────────────────────────

/// `a(?=b)` @ flag 开: 主表达式只 READ_LOOKAROUND_TABLE 并入队; 环视相 =
/// START / 前导 /.*?/ (体不锚定串尾) / 反相 (lookahead) 匹配自动机 / WRITE /
/// 无捕获 ⟹ 省第二段 / END (cc:563-597 注释 + cc:625-659)
#[test]
fn lookahead_two_phase_program() -> Void {
  let mut expected = prefix_star();
  expected.extend_from_slice(&[
    Instruction::set_register_to_cp(0), // 6
    cr(0x61, 0x61),                     // 7
    Instruction::read_look_table(0, true, LookaroundType::Lookahead), // 8
    Instruction::set_register_to_cp(1), // 9
    Instruction::accept(),              // 10
    // 环视相 (remap id 0)
    Instruction::start_lookaround(0, true, LookaroundType::Lookahead), // 11
    Instruction::fork(14),                                             // 12 → 段内 body
    Instruction::jmp(18),                                              // 13 → 段内 end
    Instruction::begin_loop(),                                         // 14
    any(),                                                             // 15
    Instruction::end_loop(),                                           // 16
    Instruction::fork(14),                                             // 17
    cr(0x62, 0x62),                   // 18 匹配自动机 (reverse=true)
    Instruction::write_look_table(0), // 19
    Instruction::end_lookaround(),    // 20
  ]);
  // 体 "b" 无捕获 ⟹ filter 相不入队 (cc:501-504), 全程序无 FILTER_*
  assert!(!expected.iter().any(|i| i.is_filter()));
  assert_eq!(code("a(?=b)", NONE, &on()), expected);
  OK
}

/// `a(?<=b)`: lookbehind 匹配自动机正相 (cc:641 `reverse = type == LOOKAHEAD`);
/// 体不锚定串首 ⟹ 同样加前导 /.*?/ (cc:636-638)
#[test]
fn lookbehind_program() -> Void {
  let mut expected = prefix_star();
  expected.extend_from_slice(&[
    Instruction::set_register_to_cp(0), // 6
    cr(0x61, 0x61),                     // 7
    Instruction::read_look_table(0, true, LookaroundType::Lookbehind), // 8
    Instruction::set_register_to_cp(1), // 9
    Instruction::accept(),              // 10
    Instruction::start_lookaround(0, true, LookaroundType::Lookbehind), // 11
    Instruction::fork(14),              // 12
    Instruction::jmp(18),               // 13
    Instruction::begin_loop(),          // 14
    any(),                              // 15
    Instruction::end_loop(),            // 16
    Instruction::fork(14),              // 17
    cr(0x62, 0x62),                     // 18 正相 (未反放)
    Instruction::write_look_table(0),   // 19
    Instruction::end_lookaround(),      // 20
  ]);
  assert_eq!(code("a(?<=b)", NONE, &on()), expected);
  // 体锚定串首 ⟹ 段内不再加前导 /.*?/ (cc:636-638 取反分支)
  let mut anchored_expected = prefix_star();
  anchored_expected.extend_from_slice(&[
    Instruction::set_register_to_cp(0), // 6
    cr(0x61, 0x61),                     // 7
    Instruction::read_look_table(0, true, LookaroundType::Lookbehind), // 8
    Instruction::set_register_to_cp(1), // 9
    Instruction::accept(),              // 10
    Instruction::start_lookaround(0, true, LookaroundType::Lookbehind), // 11
    Instruction::assertion(AssertionType::StartOfInput), // 12 体首断言直落
    cr(0x62, 0x62),                     // 13
    Instruction::write_look_table(0),   // 14
    Instruction::end_lookaround(),      // 15
  ]);
  assert_eq!(code("a(?<=^b)", NONE, &on()), anchored_expected);
  OK
}

/// `a(?!b)`: 负向 ⟹ READ/START 的 is_positive=false; 非捕获仍单段
#[test]
fn negative_lookaround_program() -> Void {
  let mut expected = prefix_star();
  expected.extend_from_slice(&[
    Instruction::set_register_to_cp(0), // 6
    cr(0x61, 0x61),                     // 7
    Instruction::read_look_table(0, false, LookaroundType::Lookahead), // 8
    Instruction::set_register_to_cp(1), // 9
    Instruction::accept(),              // 10
    Instruction::start_lookaround(0, false, LookaroundType::Lookahead), // 11
    Instruction::fork(14),              // 12
    Instruction::jmp(18),               // 13
    Instruction::begin_loop(),          // 14
    any(),                              // 15
    Instruction::end_loop(),            // 16
    Instruction::fork(14),              // 17
    cr(0x62, 0x62),                     // 18
    Instruction::write_look_table(0),   // 19
    Instruction::end_lookaround(),      // 20
  ]);
  assert_eq!(code("a(?!b)", NONE, &on()), expected);
  OK
}

/// 含捕获 lookahead 全程序: 匹配自动机相忽略寄存器 (cc:641-645 ignore_captures)
/// 且整体倒放 (reverse ⟹ 逆序下钻, cc:717-729 / 808-820); 捕获自动机相正相产
/// 寄存器; 收尾 filter 相为环视与其体内捕获产
/// FILTER_CHILD/FILTER_LOOKAROUND/FILTER_GROUP (cc:495-508)
#[test]
fn capturing_lookaround_program() -> Void {
  let mut expected = prefix_star();
  expected.extend_from_slice(&[
    Instruction::set_register_to_cp(0), // 6
    cr(0x61, 0x61),                     // 7
    Instruction::read_look_table(0, true, LookaroundType::Lookahead), // 8
    Instruction::set_register_to_cp(1), // 9
    Instruction::accept(),              // 10
    // 1. 匹配自动机 (reverse=true, ignore_captures=true)
    Instruction::start_lookaround(0, true, LookaroundType::Lookahead), // 11
    Instruction::fork(14),                                             // 12
    Instruction::jmp(18),                                              // 13
    Instruction::begin_loop(),                                         // 14
    any(),                                                             // 15
    Instruction::end_loop(),                                           // 16
    Instruction::fork(14),                                             // 17
    cr(0x63, 0x63),                   // 18 反相 ⟹ 逆序: 先捕获体 "c" (无寄存器)
    cr(0x62, 0x62),                   // 19 再 "b"
    Instruction::write_look_table(0), // 20
    // 2. 捕获自动机 (lookahead ⟹ reverse=false 正相)
    cr(0x62, 0x62),                     // 21
    Instruction::set_register_to_cp(2), // 22 捕获 1 begin
    cr(0x63, 0x63),                     // 23
    Instruction::set_register_to_cp(3), // 24 捕获 1 end
    Instruction::end_lookaround(),      // 25
    // 3. FILTER_* 相 (BFS: 环视 → 其体内捕获)
    Instruction::filter_child(27),     // 26
    Instruction::filter_lookaround(0), // 27
    Instruction::filter_child(29),     // 28
    Instruction::filter_group(1),      // 29
  ]);
  assert_eq!(code("a(?=b(c))", NONE, &on()), expected);
  OK
}

/// 环视嵌套 BFS (cc:593-597 队列循环): `x(?=(?=y)z)` — 主相只 READ 父环视; 父出队
/// 编译时其匹配自动机相 (ignore_lookarounds=false) 遇子环视 ⟹ READ(子) + 入队;
/// 父无捕获 ⟹ 无第二段; 子随后出队编译。remap 序 = 发现序 (父 0 子 1)
#[test]
fn nested_lookaround_bfs_queue() -> Void {
  let mut expected = prefix_star();
  expected.extend_from_slice(&[
    Instruction::set_register_to_cp(0), // 6
    cr(0x78, 0x78),                     // 7 "x"
    Instruction::read_look_table(0, true, LookaroundType::Lookahead), // 8 父 READ
    Instruction::set_register_to_cp(1), // 9
    Instruction::accept(),              // 10
    // 父环视段 (remap 0)
    Instruction::start_lookaround(0, true, LookaroundType::Lookahead), // 11
    Instruction::fork(14),                                             // 12
    Instruction::jmp(18),                                              // 13
    Instruction::begin_loop(),                                         // 14
    any(),                                                             // 15
    Instruction::end_loop(),                                           // 16
    Instruction::fork(14),                                             // 17
    cr(0x7a, 0x7a), // 18 "z" 反相 ⟹ Alternative 逆序先体尾
    Instruction::read_look_table(1, true, LookaroundType::Lookahead), // 19 子 READ (相中入队)
    Instruction::write_look_table(0), // 20
    Instruction::end_lookaround(), // 21 父无捕获 ⟹ 单段
    // 子环视段 (remap 1, BFS 后于父)
    Instruction::start_lookaround(1, true, LookaroundType::Lookahead), // 22
    Instruction::fork(25),                                             // 23
    Instruction::jmp(29),                                              // 24
    Instruction::begin_loop(),                                         // 25
    any(),                                                             // 26
    Instruction::end_loop(),                                           // 27
    Instruction::fork(25),                                             // 28
    cr(0x79, 0x79),                                                    // 29 "y"
    Instruction::write_look_table(1),                                  // 30
    Instruction::end_lookaround(),                                     // 31
  ]);
  assert_eq!(code("x(?=(?=y)z)", NONE, &on()), expected);
  OK
}

/// 环视捕获自动机相不再入队子环视 (cc:653-655 ignore_lookarounds, 子已在匹配相入过队):
/// `x(?=(?=y)(b))` — 父含捕获 ⟹ 两段; 第二段 (正相) 再遇子环视 ⟹ 只 READ 不入队;
/// READ id 序 = [主相父 0, 父第一段子 1, 父第二段子 1 (同值重发)], START/WRITE 各 2
#[test]
fn capturing_lookaround_second_phase_does_not_reenqueue() -> Void {
  let mut expected = prefix_star();
  expected.extend_from_slice(&[
    Instruction::set_register_to_cp(0), // 6
    cr(0x78, 0x78),                     // 7 "x"
    Instruction::read_look_table(0, true, LookaroundType::Lookahead), // 8
    Instruction::set_register_to_cp(1), // 9
    Instruction::accept(),              // 10
    // 父段 1: 匹配自动机 (reverse=true, ignore_captures=true)
    Instruction::start_lookaround(0, true, LookaroundType::Lookahead), // 11
    Instruction::fork(14),                                             // 12
    Instruction::jmp(18),                                              // 13
    Instruction::begin_loop(),                                         // 14
    any(),                                                             // 15
    Instruction::end_loop(),                                           // 16
    Instruction::fork(14),                                             // 17
    cr(0x62, 0x62), // 18 "b" 逆序先体尾捕获 (寄存器被忽略)
    Instruction::read_look_table(1, true, LookaroundType::Lookahead), // 19 子 READ + 入队
    Instruction::write_look_table(0), // 20
    // 父段 2: 捕获自动机 (lookahead ⟹ 正相; ignore_lookarounds=true)
    Instruction::read_look_table(1, true, LookaroundType::Lookahead), // 21 子 READ 重发 (不入队)
    Instruction::set_register_to_cp(2),                               // 22 捕获 1 begin
    cr(0x62, 0x62),                                                   // 23
    Instruction::set_register_to_cp(3),                               // 24 捕获 1 end
    Instruction::end_lookaround(),                                    // 25
    // 子段 (随后出队)
    Instruction::start_lookaround(1, true, LookaroundType::Lookahead), // 26
    Instruction::fork(29),                                             // 27
    Instruction::jmp(33),                                              // 28
    Instruction::begin_loop(),                                         // 29
    any(),                                                             // 30
    Instruction::end_loop(),                                           // 31
    Instruction::fork(29),                                             // 32
    cr(0x79, 0x79),                                                    // 33 "y"
    Instruction::write_look_table(1),                                  // 34
    Instruction::end_lookaround(),                                     // 35
    // filter 相: 父环视 (有捕获) → 其体内捕获; 子环视无捕获不入队 (cc:501-504)
    Instruction::filter_child(37),     // 36
    Instruction::filter_lookaround(0), // 37
    Instruction::filter_child(39),     // 38
    Instruction::filter_group(1),      // 39
  ]);
  assert_eq!(code("x(?=(?=y)(b))", NONE, &on()), expected);
  // 子环视段恰编译一次: START/WRITE 各 ×2, END ×2
  let program = code("x(?=(?=y)(b))", NONE, &on());
  for op in [
    Opcode::StartLookaround,
    Opcode::WriteLookaroundTable,
    Opcode::EndLookaround,
  ] {
    assert_eq!(
      program.iter().filter(|i| i.opcode == op).count(),
      2,
      "{op:?} 段计数"
    );
  }
  OK
}

/// 含捕获 lookbehind 第二段反相 (cc:649-656 `reverse_ = type == LOOKBEHIND` +
/// cc:1129-1135 寄存器互换): `x(?<=(bc))` — 第一段 (匹配, 正相、忽略捕获) 正序 b,c;
/// 第二段 (捕获, 反相) ⟹ SET_REGISTER_TO_CP end 先 / 体内 Atom "bc" 倒放 c,b / start 后
#[test]
fn capturing_lookbehind_second_phase_reversed() -> Void {
  let mut expected = prefix_star();
  expected.extend_from_slice(&[
    Instruction::set_register_to_cp(0), // 6
    cr(0x78, 0x78),                     // 7 "x"
    Instruction::read_look_table(0, true, LookaroundType::Lookbehind), // 8
    Instruction::set_register_to_cp(1), // 9
    Instruction::accept(),              // 10
    // 段 1: 匹配自动机 (lookbehind ⟹ reverse=false 正相, ignore_captures 只下钻)
    Instruction::start_lookaround(0, true, LookaroundType::Lookbehind), // 11
    Instruction::fork(14),                                              // 12
    Instruction::jmp(18),                                               // 13
    Instruction::begin_loop(),                                          // 14
    any(),                                                              // 15
    Instruction::end_loop(),                                            // 16
    Instruction::fork(14),                                              // 17
    cr(0x62, 0x62),                                                     // 18 "b"
    cr(0x63, 0x63),                                                     // 19 "c"
    Instruction::write_look_table(0),                                   // 20
    // 段 2: 捕获自动机 (reverse=true ⟹ end 寄存器先, 字符倒放)
    Instruction::set_register_to_cp(3), // 21 reverse ⟹ end_register 先 (cc:1133)
    cr(0x63, 0x63),                     // 22 "c" (Atom 倒放)
    cr(0x62, 0x62),                     // 23 "b"
    Instruction::set_register_to_cp(2), // 24 start_register 后 (cc:1135)
    Instruction::end_lookaround(),      // 25
    // filter 相: 环视 + 其捕获
    Instruction::filter_child(27),     // 26
    Instruction::filter_lookaround(0), // 27
    Instruction::filter_child(29),     // 28
    Instruction::filter_group(1),      // 29
  ]);
  assert_eq!(code("x(?<=(bc))", NONE, &on()), expected);
  OK
}

/// 环视编译与 capture_group_opt 无关 (cc:593-597 队列循环不受 flag 门控, 仅
/// cc:599 filter 相受控): 非捕获 lookbehind @ flag 关与开产物全等
#[test]
fn lookaround_compilation_is_flag_independent() -> Void {
  for p in ["a(?<=b)", "a(?<=^b)"] {
    assert_eq!(
      code(p, NONE, &off()),
      code(p, NONE, &on()),
      "{p:?} 无捕获环视: 两 flag 态程序全等"
    );
  }
  // flag 关 ⟹ 无 FILTER_* 亦无时钟 (cc:599-604 门控仅滤 filter 相)
  assert!(!code("a(?<=b)", NONE, &off()).iter().any(|i| i.is_filter()));
  OK
}

/// 环视 id 稠密化 (cc:1186-1190): 主相 READ / 段首 START / 段内 WRITE 三处同值,
/// 按 BFS 入队序连续编号
#[test]
fn lookaround_id_remapping_is_dense() -> Void {
  let program = code("(?=a)(?<=b)(?=c)", NONE, &on());
  let ids = |op: Opcode| -> Vec<i32> {
    program
      .iter()
      .filter(|i| i.opcode == op)
      .map(|i| match i.payload {
        Payload::Lookaround(p) => p.index() as i32,
        Payload::LookaroundId(id) => id,
        _ => panic!("{op:?} 载荷类型不符"),
      })
      .collect()
  };
  assert_eq!(ids(Opcode::ReadLookaroundTable), vec![0, 1, 2]);
  assert_eq!(ids(Opcode::StartLookaround), vec![0, 1, 2]);
  assert_eq!(ids(Opcode::WriteLookaroundTable), vec![0, 1, 2]);
  assert_eq!(
    program
      .iter()
      .filter(|i| i.opcode == Opcode::EndLookaround)
      .count(),
    3
  );
  let starts: Vec<usize> = program
    .iter()
    .enumerate()
    .filter(|(_, i)| i.opcode == Opcode::StartLookaround)
    .map(|(idx, _)| idx)
    .collect();
  assert!(starts.windows(2).all(|w| w[0] < w[1]), "环视段序单调");
  OK
}

// ── lowering: FILTER_* 相 (v8: cc:378-542) ─────────────────────────────────

/// `(a*)*` @ flag 开: 主相量词体首产 SET_QUANTIFIER_TO_CLOCK(重映射 id —— 外层
/// 先访问故外层得 0, cc:1046-1049 + cc:1178-1184), filter 相 BFS 产
/// FILTER_CHILD/FILTER_QUANTIFIER/FILTER_GROUP (cc:380-411, 454-488)
#[test]
fn filter_phase_program() -> Void {
  assert_eq!(
    code("(a*)*", RegExpFlags(STICKY), &on()),
    vec![
      Instruction::set_register_to_cp(0),      // 0
      Instruction::fork(14),                   // 1 → 外层 end
      Instruction::begin_loop(),               // 2
      Instruction::set_quantifier_to_clock(0), // 3 外层 index 1 → remap 0
      Instruction::set_register_to_cp(2),      // 4 捕获 1 begin (parser 组号从 1 起)
      Instruction::fork(11),                   // 5 → 内层 end
      Instruction::begin_loop(),               // 6
      Instruction::set_quantifier_to_clock(1), // 7 内层 index 0 → remap 1
      cr(0x61, 0x61),                          // 8
      Instruction::end_loop(),                 // 9
      Instruction::jmp(5),                     // 10 → 内层 begin
      Instruction::set_register_to_cp(3),      // 11 = 内层 end; 捕获 1 end
      Instruction::end_loop(),                 // 12
      Instruction::jmp(1),                     // 13 → 外层 begin
      Instruction::set_register_to_cp(1),      // 14 = 外层 end; 寄存器 1
      Instruction::accept(),                   // 15
      // filter 相: 外层量词入队 ⟹ FILTER_CHILD; 出队编 FILTER_QUANTIFIER,
      // 其体捕获再入队 ⟹ 第二个 FILTER_CHILD + FILTER_GROUP
      Instruction::filter_child(17),     // 16
      Instruction::filter_quantifier(0), // 17
      Instruction::filter_child(19),     // 18
      Instruction::filter_group(1),      // 19 捕获 1
    ]
  );
  OK
}

/// 量词体无捕获 @ flag 开 ⟹ filter 相不入队 (cc:464-467 CaptureRegisters 空判),
/// 主相仍产时钟
#[test]
fn filter_phase_skips_captureless_quantifiers() -> Void {
  assert_eq!(
    code("a*b", RegExpFlags(STICKY), &on()),
    vec![
      Instruction::set_register_to_cp(0),      // 0
      Instruction::fork(7),                    // 1 → end
      Instruction::begin_loop(),               // 2
      Instruction::set_quantifier_to_clock(0), // 3
      cr(0x61, 0x61),                          // 4
      Instruction::end_loop(),                 // 5
      Instruction::jmp(1),                     // 6 → begin
      cr(0x62, 0x62),                          // 7 "b"
      Instruction::set_register_to_cp(1),      // 8
      Instruction::accept(),                   // 9
    ]
  );
  // 有限重复: 每轮体首各一条时钟 (同一重映射 id), 无 filter
  assert_eq!(
    code("a{2}b", RegExpFlags(STICKY), &on()),
    vec![
      Instruction::set_register_to_cp(0),      // 0
      Instruction::set_quantifier_to_clock(0), // 1
      cr(0x61, 0x61),                          // 2
      Instruction::set_quantifier_to_clock(0), // 3
      cr(0x61, 0x61),                          // 4
      cr(0x62, 0x62),                          // 5 "b"
      Instruction::set_register_to_cp(1),      // 6
      Instruction::accept(),                   // 7
    ]
  );
  OK
}

/// flag 开且 max==0: 体完全不产码, 时钟与 filter 一并跳过 (cc:1027-1033, cc:455)
#[test]
fn zero_max_quantifier_emits_nothing() -> Void {
  assert_eq!(
    code("a{0}", RegExpFlags(STICKY), &on()),
    vec![
      Instruction::set_register_to_cp(0),
      Instruction::set_register_to_cp(1),
      Instruction::accept(),
    ]
  );
  // flag 关: min=0 次必重复 + 可选重复 max-min=0 轮 ⟹ 同样零体指令
  assert_eq!(
    code("a{0}", RegExpFlags(STICKY), &off()),
    code("a{0}", RegExpFlags(STICKY), &on())
  );
  // 前导 /.*?/ 不受影响 (非 sticky) ⟹ max==0 只吞体
  let mut expected = prefix_star();
  expected.extend_from_slice(&[
    Instruction::set_register_to_cp(0), // 6
    Instruction::set_register_to_cp(1), // 7
    Instruction::accept(),              // 8
  ]);
  assert_eq!(code("a{0}", NONE, &on()), expected);
  OK
}

/// flag 关 ⟹ 主相无时钟, 收尾也不跑 filter 相 (cc:599-604)
#[test]
fn filter_phase_absent_without_flag() -> Void {
  for p in ["(a*)*", "a*b", "(?=a)", "(a)", "a(?=b(c))"] {
    let program = code(p, RegExpFlags(STICKY), &off());
    assert!(
      !program
        .iter()
        .any(|i| i.opcode == Opcode::SetQuantifierToClock),
      "{p:?} flag 关不应产时钟"
    );
    assert!(
      !program.iter().any(|i| i.is_filter()),
      "{p:?} flag 关不应产 FILTER_*"
    );
  }
  OK
}

// ── Dump 金标准 (v8: experimental-bytecode.cc 的 ostream 语义, 编译器产物) ───

/// 逐字符推演自 cc:27-118 + cc:136-147: 每行 `setfill('0') setw(digits) <i>:
/// <inst>`; `std::hex` 一经非可打印码点触发即粘性保留 (不复原), 其后行号与
/// 数值一律按 hex 输出 (原版真实行为)
#[test]
fn compiled_program_dump_golden() -> Void {
  // 纯 ASCII ⟹ 全程十进制; 4 条指令 ⟹ 行号宽 1 位 (cc:136 DigitsRequiredBelow)
  assert_eq!(
    dump(&code("a", RegExpFlags(STICKY), &off())),
    "0: SET_REGISTER_TO_CP 0\n\
     1: CONSUME_RANGE [a, a]\n\
     2: SET_REGISTER_TO_CP 1\n\
     3: ACCEPT\n"
  );
  // [^x]: 区间下界 0 非可打印 ⟹ 触发 hex 粘性; 0x77='w' 0x79='y' 仍可打印
  assert_eq!(
    dump(&code("[^x]", RegExpFlags(STICKY), &off())),
    "0: SET_REGISTER_TO_CP 0\n\
     1: RANGE_COUNT 2\n\
     2: CONSUME_RANGE [0x0, w]\n\
     3: CONSUME_RANGE [y, 0xffff]\n\
     4: SET_REGISTER_TO_CP 1\n\
     5: ACCEPT\n"
  );
  // 前导 /.*?/ 的 CONSUME_ANY 触发 hex 粘性 ⟹ 后续数值同流进 hex
  let mut program = prefix_star();
  program.extend_from_slice(&[
    Instruction::set_register_to_cp(0),
    cr(0x61, 0x61),
    Instruction::set_register_to_cp(1),
    Instruction::accept(),
  ]);
  assert_eq!(
    dump(&program),
    "0: FORK 2\n\
     1: JMP 6\n\
     2: BEGIN_LOOP\n\
     3: CONSUME_RANGE [0x0, 0xffff]\n\
     4: END_LOOP\n\
     5: FORK 2\n\
     6: SET_REGISTER_TO_CP 0\n\
     7: CONSUME_RANGE [a, a]\n\
     8: SET_REGISTER_TO_CP 1\n\
     9: ACCEPT\n"
  );
  OK
}
