// 由 tool/src/regexp/regexp-ast.cc.js 从 v8/src/regexp/regexp-ast.cc 生成; 本文件为 Developer 就地修订版
// (骨架快照: ast_impl.rs.skel; 复跑脚本仅重生成骨架) — Wave8 W8REPS 收编挂载
//
// v8: src/regexp/regexp-ast.cc 语义面 1:1:
// - CaptureRegisters 六类覆写 (cc:40-80) + ListCaptureRegisters 助手 (cc:47-54)
// - IsCertainlyAnchoredAtStart/End 覆写面 (cc:82-156; Group 头文件内联 h:700-707)
// - 构造器 min/max 装配面 (cc:158-276) + IncreaseBy 溢出钳制 (cc:173-179)
// ast.rs 的 Tree/节点构造器统一委派本模块, 保持单一实现面

use crate::{
  ast::{
    Capture, CharacterClassStrings, ClassRanges, Interval, K_INFINITY, LookaroundType,
    StackLimiter, Tree,
  },
  ast_generated::{
    AssertionType, ClassRangesFlags, IS_CERTAINLY_TWO_CODE_POINTS, NEGATED, NO_CASE_FOLDING_NEEDED,
  },
  char_range::CharacterRange,
};

/// X-macro 展开的树种类全集 (与 FOR_EACH_REG_EXP_TREE_TYPE 同源; cc:18/24/30)
pub const XMACRO_TREE_KINDS: [&str; 14] = [
  "Disjunction",
  "Alternative",
  "Assertion",
  "ClassRanges",
  "ClassSetOperand",
  "ClassSetExpression",
  "Atom",
  "Quantifier",
  "Capture",
  "Group",
  "Lookaround",
  "BackReference",
  "Empty",
  "Text",
];

/// 就地规则锚 (从 cc 提取的字面条件; Rust 实现保持同语义)
pub mod rules {
  /// IncreaseBy 溢出封顶: kInfinity - previous < increase 时取 kInfinity (cc:173-179)
  pub const K_INFINITY_CAP: bool = true;
  /// ClassSetOperand 空 ranges 时 min=0 / max=0 (cc:200-201)
  pub const K_CLASS_SET_EMPTY_MIN_MAX: (i32, i32) = (0, 0);
  /// ClassSetOperand 非空 ranges 时 min=1 / max=2 (cc:202-204)
  pub const K_CLASS_SET_NONEMPTY_MIN_MAX: (i32, i32) = (1, 2);
  /// ClassSetExpression 取反时 max=2 (按 Unicode 字符两码点估, cc:222-227)
  pub const K_NEGATED_MAX_MATCH: i32 = 2;
  /// BMP 探测阈值: 区间 from < 0x10000 视为含基本平面 (cc:267)
  pub const K_BMP_THRESHOLD: u32 = 0x10000;
}

// ── CaptureRegisters (cc:40-80) ──

/// v8: src/regexp/regexp-ast.cc regexp::Group::CaptureRegisters
///
/// 溢出返 Invalid; 否则透传 body 并递减预算 (cc:40-43)
#[must_use]
pub fn capture_registers_group(body: &Tree, limiter: StackLimiter) -> Interval {
  if limiter.is_overflowed() {
    return Interval::invalid();
  }
  body.capture_registers(limiter.sub(1))
}

/// v8: src/regexp/regexp-ast.cc ListCaptureRegisters (匿名命名空间助手)
///
/// 子树列表的捕获寄存器并集 (cc:47-54): 溢出返 Invalid; 逐项 Union, 预算逐项递减
pub(crate) fn list_capture_registers(limiter: StackLimiter, children: &[Tree]) -> Interval {
  if limiter.is_overflowed() {
    return Interval::invalid();
  }
  let next = limiter.sub(1);
  children.iter().fold(Interval::empty(), |acc, child| {
    acc.union(child.capture_registers(next))
  })
}

/// v8: src/regexp/regexp-ast.cc regexp::Alternative::CaptureRegisters
///
/// 列表助手先行递减预算后逐项 Union (cc:58-60)
#[must_use]
pub fn capture_registers_alternative(nodes: &[Tree], limiter: StackLimiter) -> Interval {
  list_capture_registers(limiter.sub(1), nodes)
}

/// v8: src/regexp/regexp-ast.cc regexp::Disjunction::CaptureRegisters
///
/// 列表助手先行递减预算后逐项 Union (cc:62-64)
#[must_use]
pub fn capture_registers_disjunction(alternatives: &[Tree], limiter: StackLimiter) -> Interval {
  list_capture_registers(limiter.sub(1), alternatives)
}

/// v8: src/regexp/regexp-ast.cc regexp::Lookaround::CaptureRegisters
///
/// 溢出返 Invalid; body 递归 budget-1 (cc:66-69)
#[must_use]
pub fn capture_registers_lookaround(body: &Tree, limiter: StackLimiter) -> Interval {
  if limiter.is_overflowed() {
    return Interval::invalid();
  }
  body.capture_registers(limiter.sub(1))
}

/// v8: src/regexp/regexp-ast.cc regexp::Capture::CaptureRegisters
///
/// 溢出返 Invalid; 自身区间 [StartRegister, EndRegister] ∪ body (cc:71-75)
#[must_use]
pub fn capture_registers_capture(index: i32, body: &Tree, limiter: StackLimiter) -> Interval {
  if limiter.is_overflowed() {
    return Interval::invalid();
  }
  let self_interval = Interval::new(Capture::start_register(index), Capture::end_register(index));
  self_interval.union(body.capture_registers(limiter.sub(1)))
}

/// v8: src/regexp/regexp-ast.cc regexp::Quantifier::CaptureRegisters
///
/// 溢出返 Invalid; 透传 body (cc:77-80)
#[must_use]
pub fn capture_registers_quantifier(body: &Tree, limiter: StackLimiter) -> Interval {
  if limiter.is_overflowed() {
    return Interval::invalid();
  }
  body.capture_registers(limiter.sub(1))
}

// ── IsCertainlyAnchoredAtStart/End (cc:82-156; 基类默认 false, h:243-244) ──

/// v8: src/regexp/regexp-ast.cc regexp::Assertion::IsCertainlyAnchoredAtStart / AtEnd
///
/// Start 仅 START_OF_INPUT; End 仅 END_OF_INPUT。
/// 原版覆写无 budget<0 检查 (cc:82-88), 保持同语义
#[must_use]
pub fn anchored_assertion(assertion_type: AssertionType, at_start: bool) -> bool {
  if at_start {
    assertion_type == AssertionType::StartOfInput
  } else {
    assertion_type == AssertionType::EndOfInput
  }
}

/// v8: src/regexp/regexp-ast.cc regexp::Alternative::IsCertainlyAnchoredAtStart / AtEnd
///
/// budget<0 返 false; Start 正序 / End 逆序逐元素,
/// 命中即真, 元素 max_match>0 即假 (cc:90-118)
#[must_use]
pub fn anchored_alternative(nodes: &[Tree], at_start: bool, budget: i32) -> bool {
  if budget < 0 {
    return false;
  }
  if at_start {
    for node in nodes {
      if node.is_certainly_anchored_at_start(budget - 1) {
        return true;
      }
      if node.max_match() > 0 {
        return false;
      }
    }
  } else {
    for node in nodes.iter().rev() {
      if node.is_certainly_anchored_at_end(budget - 1) {
        return true;
      }
      if node.max_match() > 0 {
        return false;
      }
    }
  }
  false
}

/// v8: src/regexp/regexp-ast.cc regexp::Disjunction::IsCertainlyAnchoredAtStart / AtEnd
///
/// budget<0 返 false; 全部分支锚定才锚定 (cc:120-140; 空分支表 vacuous true, 同原版循环)
#[must_use]
pub fn anchored_disjunction(alternatives: &[Tree], at_start: bool, budget: i32) -> bool {
  if budget < 0 {
    return false;
  }
  if at_start {
    alternatives
      .iter()
      .all(|alt| alt.is_certainly_anchored_at_start(budget - 1))
  } else {
    alternatives
      .iter()
      .all(|alt| alt.is_certainly_anchored_at_end(budget - 1))
  }
}

/// v8: src/regexp/regexp-ast.cc regexp::Lookaround::IsCertainlyAnchoredAtStart
///
/// 仅正向前瞻 (LOOKAHEAD) 传递起始锚定 (cc:142-146); End 无覆写走基类 false
#[must_use]
pub fn anchored_lookaround_at_start(
  is_positive: bool,
  lookaround_type: LookaroundType,
  body: &Tree,
  budget: i32,
) -> bool {
  if budget < 0 {
    return false;
  }
  is_positive
    && lookaround_type == LookaroundType::Lookahead
    && body.is_certainly_anchored_at_start(budget - 1)
}

/// v8: src/regexp/regexp-ast.cc regexp::Capture::IsCertainlyAnchoredAtStart / AtEnd
///
/// Start/End 同型透传 body, budget<0 返 false (cc:148-156)
#[must_use]
pub fn anchored_capture(body: &Tree, at_start: bool, budget: i32) -> bool {
  if budget < 0 {
    return false;
  }
  if at_start {
    body.is_certainly_anchored_at_start(budget - 1)
  } else {
    body.is_certainly_anchored_at_end(budget - 1)
  }
}

/// v8: src/regexp/regexp-ast.h regexp::Group::IsCertainlyAnchoredAtStart / AtEnd
///
/// 头文件内联覆写 (h:700-707): Start/End 同型透传 body, budget<0 返 false
#[must_use]
pub fn anchored_group(body: &Tree, at_start: bool, budget: i32) -> bool {
  if budget < 0 {
    return false;
  }
  if at_start {
    body.is_certainly_anchored_at_start(budget - 1)
  } else {
    body.is_certainly_anchored_at_end(budget - 1)
  }
}

// ── 构造器 min/max 装配面 (cc:158-276) ──

/// v8: src/regexp/regexp-ast.cc regexp::Disjunction::Disjunction
///
/// min 取分支最小, max 取分支最大; 首分支起算, 自 1 循环 (cc:158-169)。
/// 原版 DCHECK_LT(1, length); 本实现随 parser 折叠语义放宽为非空 (debug_assert)
#[must_use]
pub fn ctor_disjunction(alternatives: &[Tree]) -> (i32, i32) {
  debug_assert!(!alternatives.is_empty());
  let first_alternative = &alternatives[0];
  let mut min_match = first_alternative.min_match();
  let mut max_match = first_alternative.max_match();
  for alternative in &alternatives[1..] {
    min_match = min_match.min(alternative.min_match());
    max_match = max_match.max(alternative.max_match());
  }
  (min_match, max_match)
}

/// v8: src/regexp/regexp-ast.cc IncreaseBy (匿名命名空间, cc:173-179)
///
/// 溢出钳制的累加: kInfinity - previous < increase 时封顶 kInfinity
#[inline]
#[must_use]
pub const fn increase_by(previous: i32, increase: i32) -> i32 {
  if K_INFINITY - previous < increase {
    K_INFINITY
  } else {
    previous + increase
  }
}

/// v8: src/regexp/regexp-ast.cc regexp::Alternative::Alternative
///
/// min/max 逐节点 IncreaseBy 累加, 0 起算 (cc:183-194)
#[must_use]
pub fn ctor_alternative(nodes: &[Tree]) -> (i32, i32) {
  debug_assert!(nodes.len() > 1);
  let mut min_match = 0;
  let mut max_match = 0;
  for node in nodes {
    min_match = increase_by(min_match, node.min_match());
    max_match = increase_by(max_match, node.max_match());
  }
  (min_match, max_match)
}

/// v8: src/regexp/regexp-ast.cc regexp::ClassSetOperand::ClassSetOperand
///
/// min/max 装配 (cc:196-212): 空 ranges 且无 strings → (0,0);
/// 非空 ranges → (1,2); 有 strings 时 min 取小 / max 取大 (原版 has_strings 判空表)
#[must_use]
pub fn ctor_class_set_operand(
  ranges: &[CharacterRange],
  strings: Option<&CharacterClassStrings>,
) -> (i32, i32) {
  let mut min_match = 0;
  let mut max_match = 0;
  if !ranges.is_empty() {
    min_match = 1;
    max_match = 2;
  }
  if let Some(strings) = strings
    && !strings.is_empty()
  {
    for (_, tree) in strings.iter() {
      min_match = min_match.min(tree.min_match());
      max_match = max_match.max(tree.max_match());
    }
  }
  (min_match, max_match)
}

/// v8: src/regexp/regexp-ast.cc regexp::ClassSetExpression::ClassSetExpression
///
/// max_match 装配 (cc:214-234): 取反恒 2 (无字符串参与, 按一 Unicode 字符两码点估);
/// 否则 operands 的 max 取大, 0 起算
#[must_use]
pub fn ctor_class_set_expression(is_negated: bool, operands: &[Tree]) -> i32 {
  if is_negated {
    2
  } else {
    let mut max_match = 0;
    for operand in operands {
      max_match = max_match.max(operand.max_match());
    }
    max_match
  }
}

/// v8: src/regexp/regexp-ast.cc regexp::ClassRanges::ClassRanges
///
/// 构造体规范化 (cc:254-276): 空区间表补 Everything 并翻转 NEGATED;
/// 未取反且未标两码点且已免大小写折叠时, 全区间 ≥0x10000 → 置 IS_CERTAINLY_TWO_CODE_POINTS
#[must_use]
pub fn ctor_class_ranges(
  mut ranges: Vec<CharacterRange>,
  mut class_ranges_flags: ClassRangesFlags,
) -> (Vec<CharacterRange>, ClassRangesFlags) {
  if ranges.is_empty() {
    ranges.push(CharacterRange::everything());
    class_ranges_flags ^= NEGATED;
  }
  if !ClassRanges::has_flag(class_ranges_flags, NEGATED)
    && !ClassRanges::has_flag(class_ranges_flags, IS_CERTAINLY_TWO_CODE_POINTS)
    && ClassRanges::has_flag(class_ranges_flags, NO_CASE_FOLDING_NEEDED)
  {
    // 基本平面探测: 任一区间 from < 0x10000 即含 BMP
    let found_basic_plane = ranges.iter().any(|r| r.from < rules::K_BMP_THRESHOLD);
    if !found_basic_plane {
      class_ranges_flags |= IS_CERTAINLY_TWO_CODE_POINTS;
    }
  }
  (ranges, class_ranges_flags)
}

/// 对数断言: X-macro 展开种类数 == 14 (FOR_EACH_REG_EXP_TREE_TYPE)
const _: () = assert!(XMACRO_TREE_KINDS.len() == 14);
