//! RegExpTree 节点体系
//!
//! 原版对应 `regexp-ast.h` + `regexp-ast.cc`:
//! - 原版虚基类 `Tree` + `FOR_EACH_REG_EXP_TREE_TYPE` 派生 → `Tree` 枚举 (enum 归一派发)
//! - 原版 zone 裸指针子节点 → `Box<Tree>` / `Vec<Tree>` 所有权
//! - `Capture`/`BackReference` 互指的原版 Zone 对象 → 捕获组号索引 (语义不变, 见 parser.rs)
//! - ToNode/Compiler 相关 (regexp-compiler-tonode.cc) 属 v8-regexp-compiler, 不在本 crate

use std::{cmp::Ordering, fmt::Write};

use crate::{
  ast_generated::{
    AssertionType, CONTAINS_SPLIT_SURROGATE, ClassRangesFlags, IS_CERTAINLY_ONE_CODE_POINT,
    IS_CERTAINLY_TWO_CODE_POINTS, NEGATED, NO_CASE_FOLDING_NEEDED, StandardCharacterSet, TreeKind,
  },
  char_range::CharacterRange,
  flags::RegExpFlags,
};

/// 匹配长度无穷 (原版 Tree::kInfinity = kMaxInt)
pub const K_INFINITY: i32 = i32::MAX;

/// 简单闭区间 (原版 Interval; 捕获寄存器区间用)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
  from: i32,
  to: i32,
}

impl Interval {
  const K_NONE: i32 = -1;
  const K_INVALID: i32 = -2;

  /// 空区间 (from=kNone, to=kNone-1, size()==0)
  #[inline]
  #[must_use]
  pub const fn empty() -> Self {
    Self {
      from: Self::K_NONE,
      to: Self::K_NONE - 1,
    }
  }

  #[inline]
  #[must_use]
  pub const fn new(from: i32, to: i32) -> Self {
    Self { from, to }
  }

  /// 无效区间 (递归预算耗尽哨兵)
  #[inline]
  #[must_use]
  pub const fn invalid() -> Self {
    Self {
      from: Self::K_INVALID,
      to: Self::K_INVALID,
    }
  }

  /// 并集 (原版 Union)
  #[must_use]
  pub const fn union(self, that: Self) -> Self {
    if !self.is_valid() {
      return self;
    }
    if !that.is_valid() {
      return that;
    }
    if that.from == Self::K_NONE {
      return self;
    }
    if self.from == Self::K_NONE {
      return that;
    }
    // min/max 不能 const 化 (core::cmp const 稳定化不全), 手写
    let from = if self.from < that.from {
      self.from
    } else {
      that.from
    };
    let to = if self.to > that.to { self.to } else { that.to };
    Self { from, to }
  }

  #[inline]
  #[must_use]
  pub const fn contains(&self, value: i32) -> bool {
    self.from <= value && value <= self.to
  }

  #[inline]
  #[must_use]
  pub const fn is_empty(&self) -> bool {
    self.from == Self::K_NONE
  }

  #[inline]
  #[must_use]
  pub const fn is_valid(&self) -> bool {
    self.from != Self::K_INVALID
  }

  #[inline]
  #[must_use]
  pub const fn from(&self) -> i32 {
    self.from
  }

  #[inline]
  #[must_use]
  pub const fn to(&self) -> i32 {
    self.to
  }

  #[inline]
  #[must_use]
  pub const fn size(&self) -> i32 {
    self.to - self.from + 1
  }
}

/// 递归预算限制器 (原版 StackLimiter)
///
/// 原版预算耗尽后查真实线程栈; 无 Isolate 环境, 仅以预算递减近似,
/// 调用方 (编译期 crate) 可按需注入栈探测。
#[derive(Debug, Clone, Copy)]
pub struct StackLimiter {
  budget: i32,
}

impl StackLimiter {
  #[inline]
  #[must_use]
  pub const fn new(budget: i32) -> Self {
    Self { budget }
  }

  /// 预算递减 (原版 operator-)
  #[inline]
  #[must_use]
  pub const fn sub(self, value: i32) -> Self {
    Self {
      budget: self.budget - value,
    }
  }

  /// 是否溢出 (预算耗尽近似)
  #[inline]
  #[must_use]
  pub const fn is_overflowed(&self) -> bool {
    self.budget <= 0
  }
}

/// 枚举递归的默认预算 (原版由调用方传入; 此处给保守值)
pub const DEFAULT_CAPTURE_REGISTERS_BUDGET: i32 = 1000;

// ── 字符类字符串 (仅 /v 模式) ──

/// 排序键: 长者在前, 同长按码点升序 (原版 CharacterClassStringLess)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassString(pub Vec<u32>);

impl PartialOrd for ClassString {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for ClassString {
  fn cmp(&self, other: &Self) -> Ordering {
    // 长者在前, 便于先生成最长匹配; 同长按码点序列升序
    other
      .0
      .len()
      .cmp(&self.0.len())
      .then_with(|| self.0.cmp(&other.0))
  }
}

/// 字符类字符串 → 对应正则子树 (原版 ZoneMap<..., CharacterClassStringLess>)
///
/// 保持有序插入 (原版 map 迭代序); emplace 语义: 已存在键不覆盖。
#[derive(Debug, Default)]
pub struct CharacterClassStrings {
  entries: Vec<(ClassString, Tree)>,
}

impl CharacterClassStrings {
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  pub fn emplace(&mut self, key: ClassString, tree: Tree) {
    match self.entries.binary_search_by(|(k, _)| k.cmp(&key)) {
      Ok(_) => {} // 已存在, 不覆盖 (std::map::emplace)
      Err(pos) => self.entries.insert(pos, (key, tree)),
    }
  }

  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  #[inline]
  #[must_use]
  pub fn len(&self) -> usize {
    self.entries.len()
  }

  /// 有序迭代 (原版 map 迭代序: 长者在前)
  pub fn iter(&self) -> impl Iterator<Item = &(ClassString, Tree)> {
    self.entries.iter()
  }
}

// ── 节点结构体 ──

/// 析取 `a|b|c` (原版 Disjunction; 构造要求 ≥2 分支)
#[derive(Debug)]
pub struct Disjunction {
  pub alternatives: Vec<Tree>,
  pub min_match: i32,
  pub max_match: i32,
}

impl Disjunction {
  #[must_use]
  pub fn new(alternatives: Vec<Tree>) -> Self {
    debug_assert!(!alternatives.is_empty());
    let (min_match, max_match) = alternatives
      .first()
      .map(|first| {
        alternatives[1..].iter().fold(
          (first.min_match(), first.max_match()),
          |(min_m, max_m), alt| (min_m.min(alt.min_match()), max_m.max(alt.max_match())),
        )
      })
      .unwrap_or((0, 0));
    Self {
      alternatives,
      min_match,
      max_match,
    }
  }
}

/// 串联 `ab c` (原版 Alternative; 构造要求 ≥2 项)
#[derive(Debug)]
pub struct Alternative {
  pub nodes: Vec<Tree>,
  pub min_match: i32,
  pub max_match: i32,
}

/// 溢出钳制的加法 (原版 IncreaseBy)
#[inline]
const fn increase_by(previous: i32, increase: i32) -> i32 {
  if K_INFINITY - previous < increase {
    K_INFINITY
  } else {
    previous + increase
  }
}

impl Alternative {
  #[must_use]
  pub fn new(nodes: Vec<Tree>) -> Self {
    debug_assert!(nodes.len() > 1);
    let (min_match, max_match) = nodes.iter().fold((0, 0), |(min_m, max_m), node| {
      (
        increase_by(min_m, node.min_match()),
        increase_by(max_m, node.max_match()),
      )
    });
    Self {
      nodes,
      min_match,
      max_match,
    }
  }
}

/// 断言 `^ $ \b \B` (原版 Assertion)
#[derive(Debug)]
pub struct Assertion {
  pub assertion_type: AssertionType,
}

/// 字符集: 标准集标记或显式区间 (原版 CharacterSet)
#[derive(Debug)]
pub struct CharacterSet {
  pub ranges: Vec<CharacterRange>,
  pub standard_set_type: Option<StandardCharacterSet>,
}

impl CharacterSet {
  /// 显式区间集
  #[must_use]
  pub fn from_ranges(ranges: Vec<CharacterRange>) -> Self {
    Self {
      ranges,
      standard_set_type: None,
    }
  }

  /// 命名标准集 (区间在构造时展开, 等价原版 ranges() 惰性展开)
  #[must_use]
  pub fn from_standard(standard_set_type: StandardCharacterSet) -> Self {
    let mut ranges = Vec::with_capacity(2);
    CharacterRange::add_class_escape(standard_set_type, &mut ranges, false);
    Self {
      ranges,
      standard_set_type: Some(standard_set_type),
    }
  }

  #[inline]
  #[must_use]
  pub fn is_standard(&self) -> bool {
    self.standard_set_type.is_some()
  }

  /// 就地规范化 (原版 CharacterSet::Canonicalize; 标准集恒规范)
  pub fn canonicalize(&mut self) {
    CharacterRange::canonicalize(&mut self.ranges);
  }
}

/// 字符类 `[...]` (原版 ClassRanges)
#[derive(Debug)]
pub struct ClassRanges {
  pub set: CharacterSet,
  pub class_ranges_flags: ClassRangesFlags,
}

impl ClassRanges {
  /// 由区间表构造 (原版 ClassRanges(zone, ranges, flags); 空表补 Everything 并翻转 NEGATED)
  #[must_use]
  pub fn new(ranges: Vec<CharacterRange>, mut class_ranges_flags: ClassRangesFlags) -> Self {
    let mut ranges = ranges;
    if ranges.is_empty() {
      ranges.push(CharacterRange::everything());
      class_ranges_flags ^= NEGATED;
    }
    if !Self::has(class_ranges_flags, NEGATED)
      && !Self::has(class_ranges_flags, IS_CERTAINLY_TWO_CODE_POINTS)
      && Self::has(class_ranges_flags, NO_CASE_FOLDING_NEEDED)
    {
      // 全部区间都在 BMP 外 → 必然两个码点
      let found_basic_plane = ranges.iter().any(|r| r.from < 0x10000);
      if !found_basic_plane {
        class_ranges_flags |= IS_CERTAINLY_TWO_CODE_POINTS;
      }
    }
    Self {
      set: CharacterSet::from_ranges(ranges),
      class_ranges_flags,
    }
  }

  /// 由命名标准集构造 (原版 ClassRanges(StandardCharacterSet))
  #[must_use]
  pub fn from_standard(standard_set_type: StandardCharacterSet) -> Self {
    Self {
      set: CharacterSet::from_standard(standard_set_type),
      class_ranges_flags: 0,
    }
  }

  #[inline]
  #[must_use]
  pub const fn has_flag(flags: ClassRangesFlags, bit: ClassRangesFlags) -> bool {
    flags & bit != 0
  }

  #[inline]
  fn has(flags: ClassRangesFlags, bit: ClassRangesFlags) -> bool {
    Self::has_flag(flags, bit)
  }

  #[inline]
  #[must_use]
  pub fn is_negated(&self) -> bool {
    Self::has(self.class_ranges_flags, NEGATED)
  }

  #[inline]
  #[must_use]
  pub fn contains_split_surrogate(&self) -> bool {
    Self::has(self.class_ranges_flags, CONTAINS_SPLIT_SURROGATE)
  }

  #[inline]
  #[must_use]
  pub fn no_case_folding_needed(&self) -> bool {
    Self::has(self.class_ranges_flags, NO_CASE_FOLDING_NEEDED)
  }

  #[inline]
  #[must_use]
  pub fn is_certainly_one_code_point(&self) -> bool {
    Self::has(self.class_ranges_flags, IS_CERTAINLY_ONE_CODE_POINT)
  }

  #[inline]
  #[must_use]
  pub fn is_certainly_two_code_points(&self) -> bool {
    Self::has(self.class_ranges_flags, IS_CERTAINLY_TWO_CODE_POINTS)
  }

  /// min: 两码点类为 2 否则 1 (原版 min_match)
  #[inline]
  #[must_use]
  pub fn min_match(&self) -> i32 {
    if self.is_certainly_two_code_points() {
      2
    } else {
      1
    }
  }

  /// max: 单码点类为 1 否则 2 (unicode 下可能两码点, 原版 max_match)
  #[inline]
  #[must_use]
  pub fn max_match(&self) -> i32 {
    if self.is_certainly_one_code_point() {
      1
    } else {
      2
    }
  }

  /// 标准集标记 (is_standard 时有效)
  #[inline]
  #[must_use]
  pub fn standard_type(&self) -> Option<StandardCharacterSet> {
    self.set.standard_set_type
  }
}

/// /v 模式类操作数 (原版 ClassSetOperand)
#[derive(Debug)]
pub struct ClassSetOperand {
  pub ranges: Vec<CharacterRange>,
  pub strings: Option<CharacterClassStrings>,
  pub min_match: i32,
  pub max_match: i32,
}

impl ClassSetOperand {
  #[must_use]
  pub fn new(ranges: Vec<CharacterRange>, strings: Option<CharacterClassStrings>) -> Self {
    let mut min_match = 0;
    let mut max_match = 0;
    if !ranges.is_empty() {
      min_match = 1;
      max_match = 2;
    }
    if let Some(ss) = strings.as_ref()
      && !ss.is_empty()
    {
      for (_, tree) in ss.iter() {
        min_match = min_match.min(tree.min_match());
        max_match = max_match.max(tree.max_match());
      }
    }
    Self {
      ranges,
      strings,
      min_match,
      max_match,
    }
  }

  #[inline]
  #[must_use]
  pub fn has_strings(&self) -> bool {
    self.strings.as_ref().is_some_and(|s| !s.is_empty())
  }
}

/// /v 类运算 (原版 ClassSetExpression)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassSetOperation {
  Union,
  Intersection,
  Subtraction,
}

#[derive(Debug)]
pub struct ClassSetExpression {
  pub operation: ClassSetOperation,
  pub is_negated: bool,
  pub may_contain_strings: bool,
  pub operands: Vec<Tree>,
  pub max_match: i32,
}

impl ClassSetExpression {
  #[must_use]
  pub fn new(
    operation: ClassSetOperation,
    is_negated: bool,
    may_contain_strings: bool,
    operands: Vec<Tree>,
  ) -> Self {
    let max_match = if is_negated {
      // 取反类无字符串, 按可匹配一个 unicode 字符 (两码点) 处理
      2
    } else {
      operands.iter().map(Tree::max_match).max().unwrap_or(0)
    };
    Self {
      operation,
      is_negated,
      may_contain_strings,
      operands,
      max_match,
    }
  }

  /// 空表达式 (取反匹配一切, 否则匹配空) (原版 ClassSetExpression::Empty)
  #[must_use]
  pub fn empty(is_negated: bool) -> Self {
    let operand = Tree::ClassSetOperand(Box::new(ClassSetOperand::new(Vec::new(), None)));
    Self::new(ClassSetOperation::Union, is_negated, false, vec![operand])
  }
}

/// 字面原子串 (原版 Atom; UTF-16 码元序列)
#[derive(Debug, Clone)]
pub struct Atom {
  pub data: Vec<u16>,
}

impl Atom {
  #[inline]
  #[must_use]
  pub fn new(data: Vec<u16>) -> Self {
    Self { data }
  }

  #[inline]
  #[must_use]
  pub fn length(&self) -> i32 {
    self.data.len() as i32
  }
}

/// 文本元素类型 (原版 TextElement::TextType)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextElementType {
  Atom,
  ClassRanges,
}

/// 文本元素: 原子或字符类 + 累计码点偏移 (原版 TextElement)
#[derive(Debug)]
pub struct TextElement {
  pub cp_offset: i32,
  pub text_type: TextElementType,
  pub tree: Tree,
}

impl TextElement {
  #[inline]
  #[must_use]
  pub fn from_atom(atom: Tree) -> Self {
    debug_assert!(matches!(atom, Tree::Atom(_)));
    Self {
      cp_offset: -1,
      text_type: TextElementType::Atom,
      tree: atom,
    }
  }

  #[inline]
  #[must_use]
  pub fn from_class_ranges(class_ranges: Tree) -> Self {
    debug_assert!(matches!(class_ranges, Tree::ClassRanges(_)));
    Self {
      cp_offset: -1,
      text_type: TextElementType::ClassRanges,
      tree: class_ranges,
    }
  }

  /// 元素长度 (原版 TextElement::length: 原子取长度, 类恒 1)
  #[inline]
  #[must_use]
  pub fn length(&self) -> i32 {
    match self.text_type {
      TextElementType::Atom => match &self.tree {
        Tree::Atom(a) => a.length(),
        _ => 0,
      },
      TextElementType::ClassRanges => 1,
    }
  }
}

/// 文本序列 (原版 Text; 装配自原子/类)
#[derive(Debug)]
pub struct Text {
  pub elements: Vec<TextElement>,
  pub length: i32,
}

impl Text {
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self {
      elements: Vec::new(),
      length: 0,
    }
  }

  /// 追加元素并累计长度 (原版 AddElement)
  pub fn add_element(&mut self, elm: TextElement) {
    self.length += elm.length();
    self.elements.push(elm);
  }

  #[must_use]
  pub fn starts_with_atom(&self) -> bool {
    self
      .elements
      .first()
      .is_some_and(|e| e.text_type == TextElementType::Atom)
  }

  /// 首个原子 (原版 FirstAtom)
  #[must_use]
  pub fn first_atom(&self) -> Option<&Atom> {
    match &self.elements.first()?.tree {
      Tree::Atom(a) => Some(a.as_ref()),
      _ => None,
    }
  }
}

impl Default for Text {
  fn default() -> Self {
    Self::new()
  }
}

/// 量词类型 (原版 Quantifier::QuantifierType)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantifierType {
  Greedy,
  NonGreedy,
  Possessive,
}

/// 量词 (原版 Quantifier)
#[derive(Debug)]
pub struct Quantifier {
  pub body: Box<Tree>,
  pub min: i32,
  pub max: i32,
  pub min_match: i32,
  pub max_match: i32,
  pub quantifier_type: QuantifierType,
  pub index: i32,
}

impl Quantifier {
  #[must_use]
  pub fn new(min: i32, max: i32, quantifier_type: QuantifierType, index: i32, body: Tree) -> Self {
    // 溢出钳制的乘法 (原版构造体)
    let body_min = body.min_match();
    let body_max = body.max_match();
    let min_match = if min > 0 && body_min > K_INFINITY / min {
      K_INFINITY
    } else {
      min.saturating_mul(body_min)
    };
    let max_match = if max > 0 && body_max > K_INFINITY / max {
      K_INFINITY
    } else {
      max.saturating_mul(body_max)
    };
    Self {
      body: Box::new(body),
      min,
      max,
      min_match,
      max_match,
      quantifier_type,
      index,
    }
  }

  #[inline]
  #[must_use]
  pub const fn is_possessive(&self) -> bool {
    matches!(self.quantifier_type, QuantifierType::Possessive)
  }

  #[inline]
  #[must_use]
  pub const fn is_non_greedy(&self) -> bool {
    matches!(self.quantifier_type, QuantifierType::NonGreedy)
  }

  #[inline]
  #[must_use]
  pub const fn is_greedy(&self) -> bool {
    matches!(self.quantifier_type, QuantifierType::Greedy)
  }
}

/// 捕获组 (原版 Capture; 命名组的 name 为 UTF-16 码元)
#[derive(Debug)]
pub struct Capture {
  pub body: Box<Tree>,
  pub index: i32,
  pub min_match: i32,
  pub max_match: i32,
  pub name: Option<Vec<u16>>,
  /// 回引降低期间置位 (原版 is_backreferenced_)
  pub is_backreferenced: bool,
}

impl Capture {
  /// 由组体装配 (原版构造 + set_body: min/max 随体更新)
  #[must_use]
  pub fn new(index: i32, name: Option<Vec<u16>>, body: Tree) -> Self {
    let min_match = body.min_match();
    let max_match = body.max_match();
    Self {
      body: Box::new(body),
      index,
      min_match,
      max_match,
      name,
      is_backreferenced: false,
    }
  }

  #[inline]
  pub fn set_body(&mut self, body: Tree) {
    self.min_match = body.min_match();
    self.max_match = body.max_match();
    *self.body = body;
  }

  #[inline]
  pub fn set_name(&mut self, name: Vec<u16>) {
    self.name = Some(name);
  }

  #[inline]
  #[must_use]
  pub const fn start_register(index: i32) -> i32 {
    index * 2
  }

  #[inline]
  #[must_use]
  pub const fn end_register(index: i32) -> i32 {
    index * 2 + 1
  }
}

/// 非捕获组 `(?i:...)` (原版 Group)
#[derive(Debug)]
pub struct Group {
  pub body: Box<Tree>,
  pub flags: RegExpFlags,
  pub min_match: i32,
  pub max_match: i32,
}

impl Group {
  #[must_use]
  pub fn new(body: Tree, flags: RegExpFlags) -> Self {
    let min_match = body.min_match();
    let max_match = body.max_match();
    Self {
      body: Box::new(body),
      flags,
      min_match,
      max_match,
    }
  }
}

/// 环视类型 (原版 Lookaround::Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookaroundType {
  Lookahead,
  Lookbehind,
}

/// 环视断言 (原版 Lookaround)
#[derive(Debug)]
pub struct Lookaround {
  pub body: Box<Tree>,
  pub is_positive: bool,
  pub capture_count: i32,
  pub capture_from: i32,
  pub lookaround_type: LookaroundType,
  pub index: i32,
}

impl Lookaround {
  #[must_use]
  pub fn new(
    body: Tree,
    is_positive: bool,
    capture_count: i32,
    capture_from: i32,
    lookaround_type: LookaroundType,
    index: i32,
  ) -> Self {
    Self {
      body: Box::new(body),
      is_positive,
      capture_count,
      capture_from,
      lookaround_type,
      index,
    }
  }
}

/// 回引 (原版 BackReference)
///
/// 原版持有 `ZoneList<Capture*>`; 本实现持捕获组号 (1-based),
/// 与 Capture 节点解耦所有权, 装配语义不变 (见 parser.rs 装配流程)。
#[derive(Debug)]
pub struct BackReference {
  pub captures: Vec<i32>,
  pub name: Option<Vec<u16>>,
}

impl BackReference {
  /// 单捕获回引
  #[must_use]
  pub fn single(capture_index: i32, name: Option<Vec<u16>>) -> Self {
    Self {
      captures: vec![capture_index],
      name,
    }
  }

  /// 命名回引占位 (捕获号在 ParseNamedBackReference 装配后由 PatchNamedBackReferences 补齐)
  #[must_use]
  pub fn named(name: Vec<u16>) -> Self {
    Self {
      captures: Vec::new(),
      name: Some(name),
    }
  }

  /// 追加捕获组号 (原版 add_capture)
  pub fn add_capture(&mut self, capture_index: i32) {
    self.captures.push(capture_index);
  }
}

/// 空表达式 (原版 Empty)
#[derive(Debug, Default)]
pub struct Empty;

// ── 统一枚举 ──

/// RegExpTree 统一节点 (原版 Tree 虚基类; 14 种, 见 TreeKind)
#[derive(Debug)]
pub enum Tree {
  Disjunction(Box<Disjunction>),
  Alternative(Box<Alternative>),
  Assertion(Box<Assertion>),
  ClassRanges(Box<ClassRanges>),
  ClassSetOperand(Box<ClassSetOperand>),
  ClassSetExpression(Box<ClassSetExpression>),
  Atom(Box<Atom>),
  Quantifier(Box<Quantifier>),
  Capture(Box<Capture>),
  Group(Box<Group>),
  Lookaround(Box<Lookaround>),
  BackReference(Box<BackReference>),
  Empty(Box<Empty>),
  Text(Box<Text>),
}

impl Tree {
  /// 节点种类
  #[inline]
  #[must_use]
  pub const fn kind(&self) -> TreeKind {
    match self {
      Self::Disjunction(_) => TreeKind::Disjunction,
      Self::Alternative(_) => TreeKind::Alternative,
      Self::Assertion(_) => TreeKind::Assertion,
      Self::ClassRanges(_) => TreeKind::ClassRanges,
      Self::ClassSetOperand(_) => TreeKind::ClassSetOperand,
      Self::ClassSetExpression(_) => TreeKind::ClassSetExpression,
      Self::Atom(_) => TreeKind::Atom,
      Self::Quantifier(_) => TreeKind::Quantifier,
      Self::Capture(_) => TreeKind::Capture,
      Self::Group(_) => TreeKind::Group,
      Self::Lookaround(_) => TreeKind::Lookaround,
      Self::BackReference(_) => TreeKind::BackReference,
      Self::Empty(_) => TreeKind::Empty,
      Self::Text(_) => TreeKind::Text,
    }
  }

  /// 最短匹配长度 (原版 min_match)
  #[must_use]
  pub fn min_match(&self) -> i32 {
    match self {
      Self::Disjunction(d) => d.min_match,
      Self::Alternative(a) => a.min_match,
      Self::Assertion(_) | Self::Lookaround(_) | Self::Empty(_) => 0,
      Self::ClassRanges(c) => c.min_match(),
      Self::ClassSetOperand(o) => o.min_match,
      // 取反类运算可能含字符串已被禁止; 非取反时按 0 (原版)
      Self::ClassSetExpression(_) => 0,
      Self::Atom(a) => a.length(),
      Self::Quantifier(q) => q.min_match,
      Self::Capture(c) => c.min_match,
      Self::Group(g) => g.min_match,
      // 回引可递归 (如 /(\2)(\1)/), 原版放弃追踪给无穷
      Self::BackReference(_) => 0,
      Self::Text(t) => t.length,
    }
  }

  /// 最长匹配长度 (原版 max_match)
  #[must_use]
  pub fn max_match(&self) -> i32 {
    match self {
      Self::Disjunction(d) => d.max_match,
      Self::Alternative(a) => a.max_match,
      Self::Assertion(_) | Self::Lookaround(_) | Self::Empty(_) => 0,
      Self::ClassRanges(c) => c.max_match(),
      Self::ClassSetOperand(o) => o.max_match,
      Self::ClassSetExpression(e) => e.max_match,
      Self::Atom(a) => a.length(),
      Self::Quantifier(q) => q.max_match,
      Self::Capture(c) => c.max_match,
      Self::Group(g) => g.max_match,
      Self::BackReference(_) => K_INFINITY,
      Self::Text(t) => t.length,
    }
  }

  /// 是否文本元素 (原版 IsTextElement)
  #[inline]
  #[must_use]
  pub const fn is_text_element(&self) -> bool {
    matches!(
      self,
      Self::ClassRanges(_)
        | Self::ClassSetOperand(_)
        | Self::ClassSetExpression(_)
        | Self::Atom(_)
        | Self::Text(_)
    )
  }

  /// 空表达式判定
  #[inline]
  #[must_use]
  pub const fn is_empty(&self) -> bool {
    matches!(self, Self::Empty(_))
  }

  /// 环视判定
  #[inline]
  #[must_use]
  pub const fn is_lookaround(&self) -> bool {
    matches!(self, Self::Lookaround(_))
  }

  /// 原子判定
  #[inline]
  #[must_use]
  pub const fn is_atom(&self) -> bool {
    matches!(self, Self::Atom(_))
  }

  /// 一定锚定于串首 (原版 IsCertainlyAnchoredAtStart, budget 限递归)
  pub fn is_certainly_anchored_at_start(&self, budget: i32) -> bool {
    if budget < 0 {
      return false;
    }
    match self {
      Self::Assertion(a) => a.assertion_type == AssertionType::StartOfInput,
      Self::Alternative(alt) => {
        for node in &alt.nodes {
          if node.is_certainly_anchored_at_start(budget - 1) {
            return true;
          }
          if node.max_match() > 0 {
            return false;
          }
        }
        false
      }
      Self::Disjunction(d) => d
        .alternatives
        .iter()
        .all(|alt| alt.is_certainly_anchored_at_start(budget - 1)),
      Self::Lookaround(l) => {
        l.is_positive
          && l.lookaround_type == LookaroundType::Lookahead
          && l.body.is_certainly_anchored_at_start(budget - 1)
      }
      Self::Capture(c) => c.body.is_certainly_anchored_at_start(budget - 1),
      Self::Group(g) => g.body.is_certainly_anchored_at_start(budget - 1),
      _ => false,
    }
  }

  /// 一定锚定于串尾 (原版 IsCertainlyAnchoredAtEnd, 逆序扫描)
  pub fn is_certainly_anchored_at_end(&self, budget: i32) -> bool {
    if budget < 0 {
      return false;
    }
    match self {
      Self::Assertion(a) => a.assertion_type == AssertionType::EndOfInput,
      Self::Alternative(alt) => {
        for node in alt.nodes.iter().rev() {
          if node.is_certainly_anchored_at_end(budget - 1) {
            return true;
          }
          if node.max_match() > 0 {
            return false;
          }
        }
        false
      }
      Self::Disjunction(d) => d
        .alternatives
        .iter()
        .all(|alt| alt.is_certainly_anchored_at_end(budget - 1)),
      Self::Capture(c) => c.body.is_certainly_anchored_at_end(budget - 1),
      Self::Group(g) => g.body.is_certainly_anchored_at_end(budget - 1),
      _ => false,
    }
  }

  /// 本表达式内捕获寄存器区间 (原版 CaptureRegisters)
  pub fn capture_registers(&self, limiter: StackLimiter) -> Interval {
    if limiter.is_overflowed() {
      return Interval::invalid();
    }
    let next = limiter.sub(1);
    match self {
      Self::Disjunction(d) => list_capture_registers(next, &d.alternatives),
      Self::Alternative(a) => list_capture_registers(next, &a.nodes),
      Self::Group(g) => g.body.capture_registers(next),
      Self::Lookaround(l) => l.body.capture_registers(next),
      Self::Capture(c) => {
        let self_interval = Interval::new(
          Capture::start_register(c.index),
          Capture::end_register(c.index),
        );
        self_interval.union(c.body.capture_registers(next))
      }
      Self::Quantifier(q) => q.body.capture_registers(next),
      _ => Interval::empty(),
    }
  }

  /// 装配进 Text (原版 AppendToText; 消费 self)
  pub fn append_to_text(self, text: &mut Text) {
    match self {
      Self::Atom(_) => text.add_element(TextElement::from_atom(self)),
      Self::ClassRanges(_) => text.add_element(TextElement::from_class_ranges(self)),
      Self::Text(mut t) => {
        text.length += t.length;
        text.elements.append(&mut t.elements);
      }
      other => debug_assert!(
        false,
        "append_to_text 仅接受文本元素, 得到 {kind:?}",
        kind = other.kind()
      ),
    }
  }

  #[inline]
  pub fn as_atom(&self) -> Option<&Atom> {
    match self {
      Self::Atom(a) => Some(a),
      _ => None,
    }
  }

  #[inline]
  pub fn as_class_ranges(&self) -> Option<&ClassRanges> {
    match self {
      Self::ClassRanges(c) => Some(c),
      _ => None,
    }
  }

  #[inline]
  pub fn as_class_set_expression(&self) -> Option<&ClassSetExpression> {
    match self {
      Self::ClassSetExpression(e) => Some(e),
      _ => None,
    }
  }

  #[inline]
  pub fn as_lookaround(&self) -> Option<&Lookaround> {
    match self {
      Self::Lookaround(l) => Some(l),
      _ => None,
    }
  }

  /// 接受 Visitor 遍历 (原版 Accept)
  pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
    match self {
      Self::Disjunction(d) => visitor.visit_disjunction(d),
      Self::Alternative(a) => visitor.visit_alternative(a),
      Self::Assertion(a) => visitor.visit_assertion(a),
      Self::ClassRanges(c) => visitor.visit_class_ranges(c),
      Self::ClassSetOperand(o) => visitor.visit_class_set_operand(o),
      Self::ClassSetExpression(e) => visitor.visit_class_set_expression(e),
      Self::Atom(a) => visitor.visit_atom(a),
      Self::Quantifier(q) => visitor.visit_quantifier(q),
      Self::Capture(c) => visitor.visit_capture(c),
      Self::Group(g) => visitor.visit_group(g),
      Self::Lookaround(l) => visitor.visit_lookaround(l),
      Self::BackReference(b) => visitor.visit_back_reference(b),
      Self::Empty(e) => visitor.visit_empty(e),
      Self::Text(t) => visitor.visit_text(t),
    }
  }
}

/// 原版 RegExpTree 类型别名
pub type RegExpTree = Tree;

/// AST 访问者模式 trait (原版 RegExpVisitor / Visitor)
pub trait Visitor {
  type Output;
  fn visit_disjunction(&mut self, node: &Disjunction) -> Self::Output;
  fn visit_alternative(&mut self, node: &Alternative) -> Self::Output;
  fn visit_assertion(&mut self, node: &Assertion) -> Self::Output;
  fn visit_class_ranges(&mut self, node: &ClassRanges) -> Self::Output;
  fn visit_class_set_operand(&mut self, node: &ClassSetOperand) -> Self::Output;
  fn visit_class_set_expression(&mut self, node: &ClassSetExpression) -> Self::Output;
  fn visit_atom(&mut self, node: &Atom) -> Self::Output;
  fn visit_quantifier(&mut self, node: &Quantifier) -> Self::Output;
  fn visit_capture(&mut self, node: &Capture) -> Self::Output;
  fn visit_group(&mut self, node: &Group) -> Self::Output;
  fn visit_lookaround(&mut self, node: &Lookaround) -> Self::Output;
  fn visit_back_reference(&mut self, node: &BackReference) -> Self::Output;
  fn visit_empty(&mut self, node: &Empty) -> Self::Output;
  fn visit_text(&mut self, node: &Text) -> Self::Output;
}

/// 原版 AstNodePrinter 实现: 输出 S-expression 表达式
#[derive(Debug, Default)]
pub struct AstPrinter {
  output: String,
}

impl AstPrinter {
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  #[must_use]
  pub fn print(mut self, tree: &Tree) -> String {
    tree.accept(&mut self);
    self.output
  }

  fn print_uc32(c: u32, buf: &mut String) {
    if (0x20..=0x7E).contains(&c) {
      buf.push(c as u8 as char);
    } else if c <= 0xFF {
      let _ = write!(buf, "\\x{c:02x}");
    } else if c <= 0xFFFF {
      let _ = write!(buf, "\\u{c:04x}");
    } else {
      let _ = write!(buf, "\\u{{{c:06x}}}");
    }
  }

  fn print_uc16(c: u16, buf: &mut String) {
    Self::print_uc32(c as u32, buf);
  }

  fn print_character_range(range: &CharacterRange, buf: &mut String) {
    Self::print_uc32(range.from, buf);
    if !range.is_singleton() {
      buf.push('-');
      Self::print_uc32(range.to, buf);
    }
  }
}

impl Visitor for AstPrinter {
  type Output = ();

  fn visit_disjunction(&mut self, node: &Disjunction) -> Self::Output {
    self.output.push_str("(|");
    for alt in &node.alternatives {
      self.output.push(' ');
      alt.accept(self);
    }
    self.output.push(')');
  }

  fn visit_alternative(&mut self, node: &Alternative) -> Self::Output {
    self.output.push_str("(:");
    for n in &node.nodes {
      self.output.push(' ');
      n.accept(self);
    }
    self.output.push(')');
  }

  fn visit_assertion(&mut self, node: &Assertion) -> Self::Output {
    let s = match node.assertion_type {
      AssertionType::StartOfInput => "@^i",
      AssertionType::EndOfInput => "@$i",
      AssertionType::EndOfBuffer => "@$Z",
      AssertionType::StartOfLine => "@^l",
      AssertionType::EndOfLine => "@$l",
      AssertionType::Boundary => "@b",
      AssertionType::NonBoundary => "@B",
    };
    self.output.push_str(s);
  }

  fn visit_class_ranges(&mut self, node: &ClassRanges) -> Self::Output {
    if node.is_negated() {
      self.output.push('^');
    }
    self.output.push('[');
    for (i, r) in node.set.ranges.iter().enumerate() {
      if i > 0 {
        self.output.push(' ');
      }
      Self::print_character_range(r, &mut self.output);
    }
    self.output.push(']');
  }

  fn visit_class_set_operand(&mut self, node: &ClassSetOperand) -> Self::Output {
    self.output.push_str("![");
    for (i, r) in node.ranges.iter().enumerate() {
      if i > 0 {
        self.output.push(' ');
      }
      Self::print_character_range(r, &mut self.output);
    }
    if let Some(strings) = &node.strings {
      for (class_string, _) in strings.iter() {
        self.output.push_str(" '");
        for &cp in &class_string.0 {
          if let Some(ch) = char::from_u32(cp) {
            self.output.push(ch);
          }
        }
        self.output.push('\'');
      }
    }
    self.output.push(']');
  }

  fn visit_class_set_expression(&mut self, node: &ClassSetExpression) -> Self::Output {
    match node.operation {
      ClassSetOperation::Union => self.output.push_str("++"),
      ClassSetOperation::Intersection => self.output.push_str("&&"),
      ClassSetOperation::Subtraction => self.output.push_str("--"),
    }
    if node.is_negated {
      self.output.push('^');
    }
    self.output.push('[');
    for (i, op) in node.operands.iter().enumerate() {
      if i > 0 {
        self.output.push(' ');
      }
      op.accept(self);
    }
    self.output.push(']');
  }

  fn visit_atom(&mut self, node: &Atom) -> Self::Output {
    self.output.push('\'');
    for &c in &node.data {
      Self::print_uc16(c, &mut self.output);
    }
    self.output.push('\'');
  }

  fn visit_quantifier(&mut self, node: &Quantifier) -> Self::Output {
    let _ = write!(self.output, "(# {} ", node.min);
    if node.max == K_INFINITY {
      self.output.push_str("- ");
    } else {
      let _ = write!(self.output, "{} ", node.max);
    }
    match node.quantifier_type {
      QuantifierType::Greedy => self.output.push_str("g "),
      QuantifierType::Possessive => self.output.push_str("p "),
      QuantifierType::NonGreedy => self.output.push_str("n "),
    }
    node.body.accept(self);
    self.output.push(')');
  }

  fn visit_capture(&mut self, node: &Capture) -> Self::Output {
    self.output.push_str("(^ ");
    node.body.accept(self);
    self.output.push(')');
  }

  fn visit_group(&mut self, node: &Group) -> Self::Output {
    let _ = write!(self.output, "(?{}: ", node.flags);
    node.body.accept(self);
    self.output.push(')');
  }

  fn visit_lookaround(&mut self, node: &Lookaround) -> Self::Output {
    self.output.push('(');
    self.output.push_str(match node.lookaround_type {
      LookaroundType::Lookahead => "->",
      LookaroundType::Lookbehind => "<-",
    });
    self
      .output
      .push_str(if node.is_positive { " + " } else { " - " });
    node.body.accept(self);
    self.output.push(')');
  }

  fn visit_back_reference(&mut self, node: &BackReference) -> Self::Output {
    self.output.push_str("(<- ");
    for (i, &c) in node.captures.iter().enumerate() {
      if i > 0 {
        self.output.push(',');
      }
      let _ = write!(self.output, "{c}");
    }
    self.output.push(')');
  }

  fn visit_empty(&mut self, _node: &Empty) -> Self::Output {
    self.output.push('%');
  }

  fn visit_text(&mut self, node: &Text) -> Self::Output {
    if node.elements.len() == 1 {
      node.elements[0].tree.accept(self);
    } else {
      self.output.push_str("(!");
      for elm in &node.elements {
        self.output.push(' ');
        elm.tree.accept(self);
      }
      self.output.push(')');
    }
  }
}

/// 快捷 AST 格式化函数
#[must_use]
pub fn format_ast(tree: &Tree) -> String {
  AstPrinter::new().print(tree)
}

/// 子树列表的捕获寄存器并集 (原版 ListCaptureRegisters)
fn list_capture_registers(limiter: StackLimiter, children: &[Tree]) -> Interval {
  if limiter.is_overflowed() {
    return Interval::invalid();
  }
  let next = limiter.sub(1);
  children.iter().fold(Interval::empty(), |acc, child| {
    acc.union(child.capture_registers(next))
  })
}
