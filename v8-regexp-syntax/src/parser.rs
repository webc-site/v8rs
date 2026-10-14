//! 正则表达式语法解析器 (原版 regexp-parser.h + regexp-parser.cc)

use std::mem::take;

use v8_strings::char_predicates::{
  hex_value, is_decimal_digit, is_identifier_part, is_identifier_start,
};

use crate::{
  ast::{
    Alternative, Assertion, Atom, BackReference, Capture, CharacterClassStrings, CharacterSet,
    ClassRanges, ClassSetExpression, ClassSetOperand, ClassSetOperation, ClassString, Disjunction,
    Empty, Group, Interval, K_INFINITY, Lookaround, LookaroundType, Quantifier, QuantifierType,
    Text, Tree,
  },
  ast_generated::{AssertionType, NEGATED, NO_CASE_FOLDING_NEEDED, StandardCharacterSet},
  char_range::{
    CharacterRange, K_LEAD_SURROGATE_END, K_LEAD_SURROGATE_START, K_MAX_CODE_POINT,
    K_NON_BMP_START, K_TRAIL_SURROGATE_END, K_TRAIL_SURROGATE_START,
  },
  error::{Error, Result},
  flags::{RegExpFlags, try_flag_from_char},
  parser_generated::{
    CLASS_SET_DOUBLE_PUNCTUATORS_MASK, CLASS_SET_RESERVED_PUNCTUATORS_MASK,
    CLASS_SET_SYNTAX_CHARS_MASK, SYNTAX_CHARS_OR_SLASH_MASK,
  },
  regexp_error_generated::RegexpError,
};

const K_END_MARKER: u32 = 1 << 21;
const K_MAX_CAPTURES: usize = 1 << 16;

const CH_RPAREN: u32 = b')' as u32;
const CH_PIPE: u32 = b'|' as u32;
const CH_STAR: u32 = b'*' as u32;
const CH_PLUS: u32 = b'+' as u32;
const CH_QUESTION: u32 = b'?' as u32;
const CH_CARET: u32 = b'^' as u32;
const CH_DOLLAR: u32 = b'$' as u32;
const CH_DOT: u32 = b'.' as u32;
const CH_LPAREN: u32 = b'(' as u32;
const CH_LBRACKET: u32 = b'[' as u32;
const CH_RBRACKET: u32 = b']' as u32;
const CH_BACKSLASH: u32 = b'\\' as u32;
const CH_LBRACE: u32 = b'{' as u32;
const CH_RBRACE: u32 = b'}' as u32;
const CH_MINUS: u32 = b'-' as u32;
const CH_AMP: u32 = b'&' as u32;

/// 编译结果数据 (原版 CompileData 子集)
#[derive(Debug)]
pub struct CompileData {
  pub tree: Tree,
  pub simple: bool,
  pub contains_anchor: bool,
  pub capture_count: usize,
  pub named_captures: Option<Vec<(Vec<u16>, Vec<i32>)>>,
}

/// 解析模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubexpressionType {
  Capture,
  Grouping,
  PositiveLookaround,
  NegativeLookaround,
}

/// 字符类操作数类别 (原版 ClassSetOperandType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ClassSetOperandType {
  #[default]
  ClassSetCharacter,
  ClassStringDisjunction,
  NestedClass,
  CharacterClassEscape,
  ClassSetRange,
}

/// 字符类运算结果 (原版 ClassSetResult)
#[derive(Debug)]
struct ClassSetResult {
  tree: Option<Tree>,
  may_contain_strings: bool,
}

impl ClassSetResult {
  fn new(tree: Option<Tree>, may_contain_strings: bool) -> Self {
    Self {
      tree,
      may_contain_strings,
    }
  }
}

/// 字符集操作数累加器
#[derive(Default)]
struct ClassSetAccumulator {
  operand_type: ClassSetOperandType,
  ranges: Vec<CharacterRange>,
  strings: CharacterClassStrings,
  character: u32,
}

impl ClassSetAccumulator {
  fn new() -> Self {
    Self::default()
  }

  fn with_ranges(ranges: Vec<CharacterRange>) -> Self {
    Self {
      operand_type: ClassSetOperandType::ClassSetCharacter,
      ranges,
      strings: CharacterClassStrings::new(),
      character: 0,
    }
  }
}

/// 文本构建器 (原版 TextBuilder)
struct TextBuilder {
  text: Vec<Tree>,
  characters: Vec<u16>,
  flags: RegExpFlags,
}

impl TextBuilder {
  fn new(flags: RegExpFlags) -> Self {
    Self {
      text: Vec::new(),
      characters: Vec::new(),
      flags,
    }
  }

  fn is_unicode_mode(&self) -> bool {
    self.flags.either_unicode()
  }

  fn flush_characters(&mut self) {
    if !self.characters.is_empty() {
      let data = take(&mut self.characters);
      self.text.push(Tree::Atom(Box::new(Atom::new(data))));
    }
  }

  fn flush_text(&mut self, terms: &mut Vec<Tree>) {
    self.flush_characters();
    let num_text = self.text.len();
    if num_text == 0 {
      return;
    }
    if num_text == 1 {
      if let Some(last) = self.text.pop() {
        terms.push(last);
      }
    } else {
      let mut text = Text::new();
      for t in self.text.drain(..) {
        t.append_to_text(&mut text);
      }
      terms.push(Tree::Text(Box::new(text)));
    }
  }

  fn add_character(&mut self, c: u16) {
    self.characters.push(c);
  }

  fn add_unicode_character(&mut self, c: u32) {
    if c > 0xFFFF {
      let lead = ((c - 0x10000) >> 10) as u16 + 0xD800;
      let trail = ((c - 0x10000) & 0x3FF) as u16 + 0xDC00;
      let atom = Tree::Atom(Box::new(Atom::new(vec![lead, trail])));
      self.add_atom(atom);
    } else {
      self.add_character(c as u16);
    }
  }

  fn add_atom(&mut self, atom: Tree) {
    debug_assert!(atom.is_text_element());
    self.flush_characters();
    self.text.push(atom);
  }

  fn add_term(&mut self, term: Tree, terms: &mut Vec<Tree>) {
    debug_assert!(term.is_text_element());
    self.flush_text(terms);
    terms.push(term);
  }

  fn add_class_ranges(&mut self, cr: ClassRanges, terms: &mut Vec<Tree>) {
    if self.needs_desugaring_for_unicode(&cr) {
      self.add_term(Tree::ClassRanges(Box::new(cr)), terms);
    } else {
      self.add_atom(Tree::ClassRanges(Box::new(cr)));
    }
  }

  fn needs_desugaring_for_unicode(&self, cc: &ClassRanges) -> bool {
    if !self.is_unicode_mode() {
      return false;
    }
    if self.flags.ignore_case() {
      return true;
    }
    let mut ranges = cc.set.ranges.clone();
    CharacterRange::canonicalize(&mut ranges);
    if cc.is_negated() {
      let mut negated = Vec::new();
      CharacterRange::negate(&ranges, &mut negated);
      ranges = negated;
    }
    for r in &ranges {
      if r.to >= K_NON_BMP_START {
        return true;
      }
      if r.from <= K_TRAIL_SURROGATE_END && r.to >= K_LEAD_SURROGATE_START {
        return true;
      }
    }
    false
  }

  fn pop_last_atom(&mut self) -> Option<Tree> {
    if let Some(last_char) = self.characters.pop() {
      if !self.characters.is_empty() {
        let prefix = take(&mut self.characters);
        self.text.push(Tree::Atom(Box::new(Atom::new(prefix))));
      }
      Some(Tree::Atom(Box::new(Atom::new(vec![last_char]))))
    } else {
      self.text.pop()
    }
  }
}

/// 项与候选构造器 (原版 Builder)
struct Builder {
  flags: RegExpFlags,
  terms: Vec<Tree>,
  alternatives: Vec<Tree>,
  text_builder: TextBuilder,
  pending_empty: bool,
}

impl Builder {
  fn new(flags: RegExpFlags) -> Self {
    Self {
      flags,
      terms: Vec::new(),
      alternatives: Vec::new(),
      text_builder: TextBuilder::new(flags),
      pending_empty: false,
    }
  }

  fn add_character(&mut self, c: u16) {
    self.pending_empty = false;
    self.text_builder.add_character(c);
  }

  fn add_unicode_character(&mut self, c: u32) {
    self.pending_empty = false;
    self.text_builder.add_unicode_character(c);
  }

  fn add_empty(&mut self) {
    self.pending_empty = true;
  }

  fn add_class_ranges(&mut self, cc: ClassRanges) {
    self.pending_empty = false;
    self.text_builder.add_class_ranges(cc, &mut self.terms);
  }

  fn add_atom(&mut self, tree: Tree) {
    if tree.is_empty() {
      self.add_empty();
      return;
    }
    self.pending_empty = false;
    if tree.is_text_element() {
      self.text_builder.add_atom(tree);
    } else {
      self.text_builder.flush_text(&mut self.terms);
      self.terms.push(tree);
    }
  }

  fn add_term(&mut self, term: Tree) {
    self.pending_empty = false;
    if term.is_text_element() {
      self.text_builder.add_term(term, &mut self.terms);
    } else {
      self.text_builder.flush_text(&mut self.terms);
      self.terms.push(term);
    }
  }

  fn add_assertion(&mut self, assertion: Assertion) {
    self.pending_empty = false;
    self.text_builder.flush_text(&mut self.terms);
    self.terms.push(Tree::Assertion(Box::new(assertion)));
  }

  fn new_alternative(&mut self) {
    self.flush_terms();
  }

  fn flush_terms(&mut self) {
    self.text_builder.flush_text(&mut self.terms);
    let num_terms = self.terms.len();
    let alt = match num_terms {
      0 => Tree::Empty(Box::new(Empty)),
      1 => self
        .terms
        .pop()
        .unwrap_or_else(|| Tree::Empty(Box::new(Empty))),
      _ => {
        let nodes = take(&mut self.terms);
        Tree::Alternative(Box::new(Alternative::new(nodes)))
      }
    };
    self.alternatives.push(alt);
    self.terms.clear();
  }

  fn build_reg_exp(&mut self) -> Tree {
    self.flush_terms();
    let num_alts = self.alternatives.len();
    match num_alts {
      0 => Tree::Empty(Box::new(Empty)),
      1 => self
        .alternatives
        .pop()
        .unwrap_or_else(|| Tree::Empty(Box::new(Empty))),
      _ => {
        let alts = take(&mut self.alternatives);
        Tree::Disjunction(Box::new(Disjunction::new(alts)))
      }
    }
  }

  fn add_quantifier_to_atom(
    &mut self,
    min: i32,
    max: i32,
    index: i32,
    quantifier_type: QuantifierType,
  ) -> bool {
    if self.pending_empty {
      self.pending_empty = false;
      return true;
    }
    let atom_opt = self.text_builder.pop_last_atom();
    let atom = if let Some(a) = atom_opt {
      self.text_builder.flush_text(&mut self.terms);
      a
    } else if let Some(last_term) = self.terms.pop() {
      if let Tree::Lookaround(ref l) = last_term {
        if self.flags.either_unicode() {
          return false;
        }
        if l.lookaround_type == LookaroundType::Lookbehind {
          return false;
        }
      }
      if last_term.max_match() == 0 {
        if min == 0 {
          return true;
        }
        self.terms.push(last_term);
        return true;
      }
      last_term
    } else {
      return false;
    };

    self.terms.push(Tree::Quantifier(Box::new(Quantifier::new(
      min,
      max,
      quantifier_type,
      index,
      atom,
    ))));
    true
  }
}

/// 解析状态栈 (原版 ParserState)
struct ParserState {
  group_type: SubexpressionType,
  lookaround_type: LookaroundType,
  capture_index: usize,
  capture_name: Option<Vec<u16>>,
  builder: Builder,
  non_participating_capture_group_intervals: Vec<Interval>,
  previous_state: Option<Box<ParserState>>,
}

impl ParserState {
  fn initial(flags: RegExpFlags) -> Self {
    Self {
      group_type: SubexpressionType::Grouping,
      lookaround_type: LookaroundType::Lookahead,
      capture_index: 0,
      capture_name: None,
      builder: Builder::new(flags),
      non_participating_capture_group_intervals: Vec::new(),
      previous_state: None,
    }
  }

  fn new_child(
    parent: Box<ParserState>,
    group_type: SubexpressionType,
    lookaround_type: LookaroundType,
    capture_index: usize,
    capture_name: Option<Vec<u16>>,
    flags: RegExpFlags,
  ) -> Self {
    let non_participating_capture_group_intervals =
      parent.non_participating_capture_group_intervals.clone();
    Self {
      group_type,
      lookaround_type,
      capture_index,
      capture_name,
      builder: Builder::new(flags),
      non_participating_capture_group_intervals,
      previous_state: Some(parent),
    }
  }

  fn new_alternative(&mut self, captures_started: usize) {
    if captures_started > self.capture_index {
      let from = (self.capture_index + 1) as i32;
      let to = captures_started as i32;
      debug_assert!(from <= to);
      if let Some(last) = self.non_participating_capture_group_intervals.last_mut()
        && last.to() + 1 == to
      {
        debug_assert!(!last.is_empty());
        debug_assert!(from >= last.from());
        *last = last.union(Interval::new(from, to));
      } else {
        self
          .non_participating_capture_group_intervals
          .push(Interval::new(from, to));
      }
    }
  }

  fn is_inside_capture_group(&self, name: &[u16]) -> bool {
    let mut curr = Some(self);
    while let Some(st) = curr {
      if st.group_type == SubexpressionType::Capture && st.capture_name.as_deref() == Some(name) {
        return true;
      }
      curr = st.previous_state.as_deref();
    }
    false
  }

  fn is_inside_capture_group_index(&self, index: usize) -> bool {
    let mut curr = Some(self);
    while let Some(st) = curr {
      if st.group_type == SubexpressionType::Capture && st.capture_index == index {
        return true;
      }
      curr = st.previous_state.as_deref();
    }
    false
  }
}

/// 正则解析器 (原版 ParserImpl)
pub struct Parser<'a> {
  input: &'a [u16],
  next_pos: usize,
  current: u32,
  has_more: bool,
  flags: RegExpFlags,
  force_unicode: bool,
  captures_started: usize,
  capture_count: usize,
  quantifier_count: i32,
  lookaround_count: i32,
  simple: bool,
  contains_anchor: bool,
  is_scanned_for_captures: bool,
  has_named_captures: bool,
  named_captures: Vec<(Vec<u16>, Vec<i32>)>,
  named_back_references: Vec<Vec<u16>>,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a [u16], flags: RegExpFlags) -> Self {
    let mut parser = Self {
      input,
      next_pos: 0,
      current: K_END_MARKER,
      has_more: true,
      flags,
      force_unicode: false,
      captures_started: 0,
      capture_count: 0,
      quantifier_count: 0,
      lookaround_count: 0,
      simple: false,
      contains_anchor: false,
      is_scanned_for_captures: false,
      has_named_captures: false,
      named_captures: Vec::new(),
      named_back_references: Vec::new(),
    };
    parser.advance();
    parser
  }

  #[inline]
  fn is_unicode_mode(&self) -> bool {
    self.flags.either_unicode() || self.force_unicode
  }

  #[inline]
  fn unicode_sets(&self) -> bool {
    self.flags.unicode_sets()
  }

  #[inline]
  fn ignore_case(&self) -> bool {
    self.flags.ignore_case()
  }

  #[inline]
  fn position(&self) -> usize {
    if self.current != K_END_MARKER && self.current > 0xFFFF {
      self.next_pos.saturating_sub(2)
    } else {
      self.next_pos.saturating_sub(1)
    }
  }

  #[inline]
  fn has_next(&self) -> bool {
    self.next_pos < self.input.len()
  }

  fn peek_next(&self) -> u32 {
    let mut pos = self.next_pos;
    if pos >= self.input.len() {
      return K_END_MARKER;
    }
    let c0 = self.input[pos] as u32;
    pos += 1;
    let mut res = c0;
    if self.is_unicode_mode()
      && (K_LEAD_SURROGATE_START..=K_LEAD_SURROGATE_END).contains(&c0)
      && pos < self.input.len()
    {
      let c1 = self.input[pos] as u32;
      if (K_TRAIL_SURROGATE_START..=K_TRAIL_SURROGATE_END).contains(&c1) {
        res = ((c0 - K_LEAD_SURROGATE_START) << 10) + (c1 - K_TRAIL_SURROGATE_START) + 0x10000;
      }
    }
    res
  }

  #[inline]
  fn next(&self) -> u32 {
    if self.has_next() {
      self.peek_next()
    } else {
      K_END_MARKER
    }
  }

  fn advance(&mut self) {
    if self.has_next() {
      let mut pos = self.next_pos;
      let c0 = self.input[pos] as u32;
      pos += 1;
      let mut res = c0;
      if self.is_unicode_mode()
        && (K_LEAD_SURROGATE_START..=K_LEAD_SURROGATE_END).contains(&c0)
        && pos < self.input.len()
      {
        let c1 = self.input[pos] as u32;
        if (K_TRAIL_SURROGATE_START..=K_TRAIL_SURROGATE_END).contains(&c1) {
          res = ((c0 - K_LEAD_SURROGATE_START) << 10) + (c1 - K_TRAIL_SURROGATE_START) + 0x10000;
          pos += 1;
        }
      }
      self.next_pos = pos;
      self.current = res;
    } else {
      self.current = K_END_MARKER;
      self.next_pos = self.input.len() + 1;
      self.has_more = false;
    }
  }

  fn advance_by(&mut self, dist: usize) {
    if dist > 0 {
      self.next_pos += dist - 1;
      self.advance();
    }
  }

  fn rewind_by_one_codepoint(&mut self) {
    if !self.has_more {
      return;
    }
    let rewind_by = if self.current > 0xFFFF { 2 } else { 1 };
    self.reset(self.next_pos.saturating_sub(rewind_by + 1));
  }

  fn reset(&mut self, pos: usize) {
    self.next_pos = pos;
    self.has_more = pos < self.input.len();
    self.advance();
  }

  fn error(&self, kind: RegexpError) -> Error {
    Error::parse(kind, self.position())
  }

  // 语法字符集判定 (使用编译期 ASCII 掩码, O(1) 且防截断)
  #[inline]
  fn is_syntax_character_or_slash(c: u32) -> bool {
    c < 128 && (SYNTAX_CHARS_OR_SLASH_MASK & (1u128 << c)) != 0
  }

  #[inline]
  fn is_class_set_syntax_character(c: u32) -> bool {
    c < 128 && (CLASS_SET_SYNTAX_CHARS_MASK & (1u128 << c)) != 0
  }

  #[inline]
  fn is_class_set_reserved_punctuator(c: u32) -> bool {
    c < 128 && (CLASS_SET_RESERVED_PUNCTUATORS_MASK & (1u128 << c)) != 0
  }

  #[inline]
  fn is_class_set_reserved_double_punctuator(&self, c: u32) -> bool {
    let next_char = self.next();
    c == next_char && c < 128 && (CLASS_SET_DOUBLE_PUNCTUATORS_MASK & (1u128 << c)) != 0
  }

  // 主解析入口
  pub fn parse(&mut self) -> Result<CompileData> {
    let tree = self.parse_pattern()?;
    Ok(CompileData {
      simple: self.simple,
      contains_anchor: self.contains_anchor,
      capture_count: self.captures_started,
      named_captures: if self.named_captures.is_empty() {
        None
      } else {
        Some(self.named_captures.clone())
      },
      tree,
    })
  }

  fn parse_pattern(&mut self) -> Result<Tree> {
    let mut result = self.parse_disjunction()?;
    self.patch_named_back_references(&mut result)?;
    if let Tree::Atom(ref a) = result
      && a.length() as usize == self.input.len()
      && self.captures_started == 0
    {
      self.simple = true;
    }
    Ok(result)
  }

  fn patch_named_back_references(&self, tree: &mut Tree) -> Result<()> {
    if self.named_back_references.is_empty() {
      return Ok(());
    }
    for ref_name in &self.named_back_references {
      let found = self.named_captures.iter().any(|(name, _)| name == ref_name);
      if !found {
        return Err(self.error(RegexpError::InvalidNamedCaptureReference));
      }
    }
    self.patch_back_references_in_tree(tree)
  }

  fn patch_back_references_in_tree(&self, tree: &mut Tree) -> Result<()> {
    match tree {
      Tree::BackReference(b) => {
        if let Some(ref name) = b.name {
          if let Some((_, indices)) = self.named_captures.iter().find(|(n, _)| n == name) {
            b.captures = indices.clone();
          } else {
            return Err(self.error(RegexpError::InvalidNamedCaptureReference));
          }
        }
      }
      Tree::Disjunction(d) => {
        for alt in &mut d.alternatives {
          self.patch_back_references_in_tree(alt)?;
        }
      }
      Tree::Alternative(a) => {
        for node in &mut a.nodes {
          self.patch_back_references_in_tree(node)?;
        }
      }
      Tree::Capture(c) => {
        self.patch_back_references_in_tree(&mut c.body)?;
      }
      Tree::Group(g) => {
        self.patch_back_references_in_tree(&mut g.body)?;
      }
      Tree::Lookaround(l) => {
        self.patch_back_references_in_tree(&mut l.body)?;
      }
      Tree::Quantifier(q) => {
        self.patch_back_references_in_tree(&mut q.body)?;
      }
      Tree::Text(t) => {
        for el in &mut t.elements {
          self.patch_back_references_in_tree(&mut el.tree)?;
        }
      }
      _ => {}
    }
    Ok(())
  }

  fn parse_disjunction(&mut self) -> Result<Tree> {
    let mut state = ParserState::initial(self.flags);

    while self.has_more {
      match self.current {
        CH_RPAREN => {
          if state.previous_state.is_none() {
            return Err(self.error(RegexpError::UnmatchedParen));
          }
          self.advance();
          let mut body = state.builder.build_reg_exp();
          let end_capture_index = self.captures_started;
          let capture_index = state.capture_index;
          let group_type = state.group_type;

          match group_type {
            SubexpressionType::Capture => {
              if let Some(ref name) = state.capture_name {
                self.create_named_capture_at_index(&state, capture_index, name.clone())?;
              }
              body = Tree::Capture(Box::new(Capture::new(
                capture_index as i32,
                state.capture_name.clone(),
                body,
              )));
            }
            SubexpressionType::Grouping => {
              body = Tree::Group(Box::new(Group::new(body, state.builder.flags)));
            }
            SubexpressionType::PositiveLookaround | SubexpressionType::NegativeLookaround => {
              let is_positive = group_type == SubexpressionType::PositiveLookaround;
              body = Tree::Lookaround(Box::new(Lookaround::new(
                body,
                is_positive,
                (end_capture_index - capture_index) as i32,
                capture_index as i32,
                state.lookaround_type,
                self.lookaround_count,
              )));
              self.lookaround_count += 1;
            }
          }

          let Some(prev_state) = state.previous_state.take() else {
            return Err(self.error(RegexpError::UnmatchedParen));
          };
          state = *prev_state;
          self.flags = state.builder.flags;
          state.builder.add_atom(body);
        }
        CH_PIPE => {
          self.advance();
          state.new_alternative(self.captures_started);
          state.builder.new_alternative();
          continue;
        }
        CH_STAR | CH_PLUS | CH_QUESTION => {
          return Err(self.error(RegexpError::NothingToRepeat));
        }
        CH_CARET => {
          self.advance();
          let assertion_type = if state.builder.flags.multiline() {
            AssertionType::StartOfLine
          } else {
            AssertionType::StartOfInput
          };
          state.builder.add_assertion(Assertion { assertion_type });
          self.contains_anchor = true;
          continue;
        }
        CH_DOLLAR => {
          self.advance();
          let assertion_type = if state.builder.flags.multiline() {
            AssertionType::EndOfLine
          } else {
            AssertionType::EndOfInput
          };
          state.builder.add_assertion(Assertion { assertion_type });
          continue;
        }
        CH_DOT => {
          self.advance();
          let set = if state.builder.flags.dot_all() {
            CharacterSet::from_standard(StandardCharacterSet::Everything)
          } else {
            CharacterSet::from_standard(StandardCharacterSet::NotLineTerminator)
          };
          state.builder.add_class_ranges(ClassRanges {
            set,
            class_ranges_flags: 0,
          });
        }
        CH_LPAREN => {
          state = self.parse_open_parenthesis(Box::new(state))?;
          self.flags = state.builder.flags;
          continue;
        }
        CH_LBRACKET => {
          let res = self.parse_character_class(&state.builder)?;
          if let Some(tree) = res.tree {
            if let Tree::ClassRanges(cr) = tree {
              state.builder.add_class_ranges(*cr);
            } else {
              state.builder.add_term(tree);
            }
          }
        }
        CH_BACKSLASH => {
          self.parse_backslash(&mut state)?;
        }
        CH_LBRACE => {
          if self.parse_interval_quantifier().is_some() {
            return Err(self.error(RegexpError::NothingToRepeat));
          }
          if self.is_unicode_mode() {
            return Err(self.error(RegexpError::LoneQuantifierBrackets));
          }
          state.builder.add_unicode_character(self.current);
          self.advance();
        }
        CH_RBRACE | CH_RBRACKET => {
          if self.is_unicode_mode() {
            return Err(self.error(RegexpError::LoneQuantifierBrackets));
          }
          state.builder.add_unicode_character(self.current);
          self.advance();
        }
        _ => {
          let c = self.current;
          state.builder.add_unicode_character(c);
          self.advance();
        }
      }

      // 量词检查
      if self.has_more {
        let (min, max) = match self.current {
          CH_STAR => {
            self.advance();
            (0, K_INFINITY)
          }
          CH_PLUS => {
            self.advance();
            (1, K_INFINITY)
          }
          CH_QUESTION => {
            self.advance();
            (0, 1)
          }
          CH_LBRACE => {
            if let Some((min_val, max_val)) = self.parse_interval_quantifier() {
              if max_val < min_val {
                return Err(self.error(RegexpError::RangeOutOfOrder));
              }
              (min_val, max_val)
            } else if self.is_unicode_mode() {
              return Err(self.error(RegexpError::IncompleteQuantifier));
            } else {
              continue;
            }
          }
          _ => continue,
        };

        let mut quantifier_type = QuantifierType::Greedy;
        if self.current == CH_QUESTION {
          quantifier_type = QuantifierType::NonGreedy;
          self.advance();
        } else if self.current == CH_PLUS {
          quantifier_type = QuantifierType::Possessive;
          self.advance();
        }

        if !state
          .builder
          .add_quantifier_to_atom(min, max, self.quantifier_count, quantifier_type)
        {
          return Err(self.error(RegexpError::InvalidQuantifier));
        }
        self.quantifier_count += 1;
      }
    }

    if state.previous_state.is_some() {
      return Err(self.error(RegexpError::UnterminatedGroup));
    }

    Ok(state.builder.build_reg_exp())
  }

  fn parse_backslash(&mut self, state: &mut ParserState) -> Result<()> {
    let next_char = self.next();
    if next_char == K_END_MARKER {
      return Err(self.error(RegexpError::EscapeAtEndOfPattern));
    }

    match u8::try_from(next_char).ok().map(char::from) {
      Some('1'..='9') => {
        let saved_pos = self.position();
        if let Some(index) = self.parse_back_reference_index() {
          if state.is_inside_capture_group_index(index) {
            state.builder.add_empty();
          } else {
            state
              .builder
              .add_atom(Tree::BackReference(Box::new(BackReference::single(
                index as i32,
                None,
              ))));
          }
          return Ok(());
        }
        self.reset(saved_pos);
        if self.is_unicode_mode() {
          return Err(self.error(RegexpError::InvalidEscape));
        }
        if next_char == b'8' as u32 || next_char == b'9' as u32 {
          state.builder.add_character(next_char as u16);
          self.advance_by(2);
          return Ok(());
        }
        // 八进制字面量
        self.advance();
        let octal = self.parse_octal_literal();
        state.builder.add_character(octal as u16);
      }
      Some('0') => {
        self.advance();
        if self.is_unicode_mode() && self.next() >= b'0' as u32 && self.next() <= b'9' as u32 {
          return Err(self.error(RegexpError::InvalidDecimalEscape));
        }
        let octal = self.parse_octal_literal();
        state.builder.add_character(octal as u16);
      }
      Some('b') => {
        self.advance_by(2);
        state.builder.add_assertion(Assertion {
          assertion_type: AssertionType::Boundary,
        });
      }
      Some('B') => {
        self.advance_by(2);
        state.builder.add_assertion(Assertion {
          assertion_type: AssertionType::NonBoundary,
        });
      }
      Some(ch @ ('d' | 'D' | 's' | 'S' | 'w' | 'W')) => {
        if let Some(std_set) = StandardCharacterSet::from_escape(ch) {
          self.advance_by(2);
          state
            .builder
            .add_class_ranges(ClassRanges::from_standard(std_set));
        }
      }
      Some('p' | 'P') => {
        if !self.is_unicode_mode() {
          let c = self.parse_character_escape(false)?;
          state.builder.add_unicode_character(c);
          return Ok(());
        }
        let negate = next_char == b'P' as u32;
        self.advance_by(2);
        let mut ranges = Vec::new();
        let mut strings = if self.unicode_sets() {
          Some(CharacterClassStrings::new())
        } else {
          None
        };
        let _ = self.parse_property_escape(&mut ranges, &mut strings, negate, false)?;
        if self.unicode_sets() {
          state
            .builder
            .add_term(Tree::ClassSetOperand(Box::new(ClassSetOperand::new(
              ranges, strings,
            ))));
        } else {
          state
            .builder
            .add_term(Tree::ClassRanges(Box::new(ClassRanges::new(ranges, 0))));
        }
      }
      Some('k') => {
        let has_named = self.has_named_captures(false);
        if self.is_unicode_mode() || has_named {
          self.advance_by(2);
          self.parse_named_back_reference(state)?;
        } else {
          let c = self.parse_character_escape(false)?;
          state.builder.add_unicode_character(c);
        }
      }
      _ => {
        let c = self.parse_character_escape(false)?;
        state.builder.add_unicode_character(c);
      }
    }
    Ok(())
  }

  fn parse_named_back_reference(&mut self, state: &mut ParserState) -> Result<()> {
    if self.current != b'<' as u32 {
      return Err(self.error(RegexpError::InvalidNamedReference));
    }
    self.advance();
    let name = self.parse_capture_group_name()?;
    if state.is_inside_capture_group(&name) {
      state.builder.add_empty();
    } else {
      let mut backref = BackReference::named(name.clone());
      if let Some((_, indices)) = self.named_captures.iter().find(|(n, _)| n == &name) {
        for &idx in indices {
          backref.add_capture(idx);
        }
      }
      state
        .builder
        .add_atom(Tree::BackReference(Box::new(backref)));
      self.named_back_references.push(name);
    }
    Ok(())
  }

  fn parse_open_parenthesis(&mut self, parent: Box<ParserState>) -> Result<ParserState> {
    let mut lookaround_type = parent.lookaround_type;
    let mut is_named_capture = false;
    let mut capture_name = None;
    let mut subexpr_type = SubexpressionType::Capture;
    let mut flags = parent.builder.flags;
    let mut parsing_modifiers = false;
    let mut modifiers_polarity = true;
    let mut modifiers = RegExpFlags::NONE;

    self.advance(); // consume '('
    if self.current == b'?' as u32 {
      loop {
        let next_char = self.next();
        match u8::try_from(next_char).ok().map(char::from) {
          Some('-') => {
            self.advance();
            parsing_modifiers = true;
            if !modifiers_polarity {
              return Err(self.error(RegexpError::MultipleFlagDashes));
            }
            modifiers_polarity = false;
          }
          Some(ch @ ('m' | 'i' | 's')) => {
            self.advance();
            parsing_modifiers = true;
            let Some(flag) = try_flag_from_char(ch) else {
              return Err(self.error(RegexpError::InvalidFlagGroup));
            };
            if modifiers.contains(flag) {
              return Err(self.error(RegexpError::RepeatedFlag));
            }
            modifiers |= flag;
            flags.set(flag, modifiers_polarity);
          }
          Some(':') => {
            self.advance_by(2);
            subexpr_type = SubexpressionType::Grouping;
            break;
          }
          Some('=') => {
            self.advance_by(2);
            if parsing_modifiers {
              return Err(self.error(RegexpError::InvalidGroup));
            }
            lookaround_type = LookaroundType::Lookahead;
            subexpr_type = SubexpressionType::PositiveLookaround;
            break;
          }
          Some('!') => {
            self.advance_by(2);
            if parsing_modifiers {
              return Err(self.error(RegexpError::InvalidGroup));
            }
            lookaround_type = LookaroundType::Lookahead;
            subexpr_type = SubexpressionType::NegativeLookaround;
            break;
          }
          Some('<') => {
            self.advance();
            if parsing_modifiers {
              return Err(self.error(RegexpError::InvalidGroup));
            }
            if self.next() == b'=' as u32 {
              self.advance_by(2);
              lookaround_type = LookaroundType::Lookbehind;
              subexpr_type = SubexpressionType::PositiveLookaround;
              break;
            } else if self.next() == b'!' as u32 {
              self.advance_by(2);
              lookaround_type = LookaroundType::Lookbehind;
              subexpr_type = SubexpressionType::NegativeLookaround;
              break;
            }
            is_named_capture = true;
            self.has_named_captures = true;
            self.advance(); // consume '<'
            break;
          }
          _ => return Err(self.error(RegexpError::InvalidGroup)),
        }
      }
    }

    if !modifiers_polarity && modifiers == RegExpFlags::NONE {
      return Err(self.error(RegexpError::InvalidFlagGroup));
    }

    if subexpr_type == SubexpressionType::Capture {
      if self.captures_started >= K_MAX_CAPTURES {
        return Err(self.error(RegexpError::TooManyCaptures));
      }
      self.captures_started += 1;
      if is_named_capture {
        capture_name = Some(self.parse_capture_group_name()?);
      }
    }

    Ok(ParserState::new_child(
      parent,
      subexpr_type,
      lookaround_type,
      self.captures_started,
      capture_name,
      flags,
    ))
  }

  fn parse_capture_group_name(&mut self) -> Result<Vec<u16>> {
    self.rewind_by_one_codepoint();
    let mut name = Vec::new();
    self.force_unicode = true;

    let mut at_start = true;
    loop {
      self.advance();
      let mut c = self.current;
      if c == b'\\' as u32 && self.next() == b'u' as u32 {
        self.advance_by(2);
        c = self.parse_unicode_escape()?;
        self.rewind_by_one_codepoint();
      }

      if c == b'\\' as u32 {
        self.force_unicode = false;
        return Err(self.error(RegexpError::InvalidCaptureGroupName));
      }

      if at_start {
        if !is_identifier_start(c) {
          self.force_unicode = false;
          return Err(self.error(RegexpError::InvalidCaptureGroupName));
        }
        push_code_unit(&mut name, c);
        at_start = false;
      } else if c == b'>' as u32 {
        break;
      } else if is_identifier_part(c) {
        push_code_unit(&mut name, c);
      } else {
        self.force_unicode = false;
        return Err(self.error(RegexpError::InvalidCaptureGroupName));
      }
    }

    self.force_unicode = false;
    self.advance();
    Ok(name)
  }

  fn create_named_capture_at_index(
    &mut self,
    state: &ParserState,
    index: usize,
    name: Vec<u16>,
  ) -> Result<()> {
    if let Some((_, indices)) = self.named_captures.iter().find(|(n, _)| n == &name) {
      for &named_index in indices {
        let mut is_duplicate = true;
        for interval in &state.non_participating_capture_group_intervals {
          if interval.contains(named_index) {
            is_duplicate = false;
            break;
          }
          if named_index <= interval.from() {
            break;
          }
        }
        if is_duplicate {
          return Err(self.error(RegexpError::DuplicateCaptureGroupName));
        }
      }
    }

    if let Some(parent) = state.previous_state.as_deref()
      && parent.is_inside_capture_group(&name)
    {
      return Err(self.error(RegexpError::DuplicateCaptureGroupName));
    }

    if let Some((_, indices)) = self.named_captures.iter_mut().find(|(n, _)| n == &name) {
      indices.push(index as i32);
    } else {
      self.named_captures.push((name, vec![index as i32]));
    }
    Ok(())
  }

  fn parse_character_escape(&mut self, in_class: bool) -> Result<u32> {
    self.advance(); // consume '\'
    let c = self.current;
    if !self.has_more || c == K_END_MARKER {
      return Err(self.error(RegexpError::EscapeAtEndOfPattern));
    }
    match c {
      0x66 /* 'f' */ => {
        self.advance();
        Ok(0x0C)
      }
      0x6E /* 'n' */ => {
        self.advance();
        Ok(0x0A)
      }
      0x72 /* 'r' */ => {
        self.advance();
        Ok(0x0D)
      }
      0x74 /* 't' */ => {
        self.advance();
        Ok(0x09)
      }
      0x76 /* 'v' */ => {
        self.advance();
        Ok(0x0B)
      }
      0x63 /* 'c' */ => {
        let control_letter = self.next();
        let letter = control_letter & !('A' as u32 ^ 'a' as u32);
        if (b'A' as u32..=b'Z' as u32).contains(&letter) {
          self.advance_by(2);
          return Ok(control_letter & 0x1F);
        }
        if self.is_unicode_mode() {
          return Err(self.error(RegexpError::InvalidUnicodeEscape));
        }
        if in_class
          && ((b'0' as u32..=b'9' as u32).contains(&control_letter)
            || control_letter == b'_' as u32)
        {
          self.advance_by(2);
          return Ok(control_letter & 0x1F);
        }
        Ok(b'\\' as u32)
      }
      0x30 /* '0' */ => {
        if self.next() < b'0' as u32 || self.next() > b'9' as u32 {
          self.advance();
          return Ok(0);
        }
        if self.is_unicode_mode() {
          return Err(self.error(RegexpError::InvalidDecimalEscape));
        }
        Ok(self.parse_octal_literal())
      }
      0x31..=0x37 /* '1'..='7' */ => {
        if self.is_unicode_mode() {
          return Err(self.error(RegexpError::InvalidDecimalEscape));
        }
        Ok(self.parse_octal_literal())
      }
      0x78 /* 'x' */ => {
        self.advance();
        if let Ok(val) = self.parse_hex_escape(2) {
          Ok(val)
        } else if self.is_unicode_mode() {
          Err(self.error(RegexpError::InvalidEscape))
        } else {
          Ok(b'x' as u32)
        }
      }
      0x75 /* 'u' */ => {
        self.advance();
        if let Ok(val) = self.parse_unicode_escape() {
          Ok(val)
        } else if self.is_unicode_mode() {
          Err(self.error(RegexpError::InvalidUnicodeEscape))
        } else {
          Ok(b'u' as u32)
        }
      }
      _ => {
        if self.unicode_sets() && in_class && Self::is_class_set_reserved_punctuator(c) {
          self.advance();
          return Ok(c);
        }
        if self.is_unicode_mode() {
          if !Self::is_syntax_character_or_slash(c) {
            return Err(self.error(RegexpError::InvalidEscape));
          }
          self.advance();
          return Ok(c);
        }
        if c == b'c' as u32 {
          return Err(self.error(RegexpError::InvalidEscape));
        }
        self.advance();
        if c == b'k' as u32 && self.has_named_captures(in_class) {
          return Err(self.error(RegexpError::InvalidEscape));
        }
        Ok(c)
      }
    }
  }

  fn parse_octal_literal(&mut self) -> u32 {
    let mut val = self.current - b'0' as u32;
    self.advance();
    if self.current >= b'0' as u32 && self.current <= b'7' as u32 {
      val = val * 8 + (self.current - b'0' as u32);
      self.advance();
      if val < 32 && self.current >= b'0' as u32 && self.current <= b'7' as u32 {
        val = val * 8 + (self.current - b'0' as u32);
        self.advance();
      }
    }
    val
  }

  fn parse_hex_escape(&mut self, length: usize) -> Result<u32> {
    let start = self.position();
    let mut val = 0_u32;
    for _ in 0..length {
      let c = self.current;
      let Some(d) = hex_value(c) else {
        self.reset(start);
        return Err(self.error(RegexpError::InvalidEscape));
      };
      val = val * 16 + d;
      self.advance();
    }
    Ok(val)
  }

  fn parse_unicode_escape(&mut self) -> Result<u32> {
    if self.current == b'{' as u32 && self.is_unicode_mode() {
      let start = self.position();
      self.advance();
      let mut val = 0_u32;
      let mut has_digit = false;
      while let Some(d) = hex_value(self.current) {
        has_digit = true;
        val = val * 16 + d;
        if val > K_MAX_CODE_POINT {
          self.reset(start);
          return Err(self.error(RegexpError::InvalidUnicodeEscape));
        }
        self.advance();
      }
      if has_digit && self.current == b'}' as u32 {
        self.advance();
        return Ok(val);
      }
      self.reset(start);
      return Err(self.error(RegexpError::InvalidUnicodeEscape));
    }

    let mut val = self.parse_hex_escape(4)?;
    if self.is_unicode_mode()
      && (K_LEAD_SURROGATE_START..=K_LEAD_SURROGATE_END).contains(&val)
      && self.current == b'\\' as u32
    {
      let start = self.position();
      if self.next() == b'u' as u32 {
        self.advance_by(2);
        if let Ok(trail) = self.parse_hex_escape(4)
          && (K_TRAIL_SURROGATE_START..=K_TRAIL_SURROGATE_END).contains(&trail)
        {
          val =
            ((val - K_LEAD_SURROGATE_START) << 10) + (trail - K_TRAIL_SURROGATE_START) + 0x10000;
          return Ok(val);
        }
      }
      self.reset(start);
    }
    Ok(val)
  }

  fn parse_interval_quantifier(&mut self) -> Option<(i32, i32)> {
    if self.current != b'{' as u32 {
      return None;
    }
    let start = self.position();
    self.advance();
    if !is_decimal_digit(self.current) {
      self.reset(start);
      return None;
    }
    let mut min = 0_i32;
    while is_decimal_digit(self.current) {
      let d = (self.current - b'0' as u32) as i32;
      min = min.saturating_mul(10).saturating_add(d);
      self.advance();
    }
    let max = if self.current == b'}' as u32 {
      self.advance();
      min
    } else if self.current == b',' as u32 {
      self.advance();
      if self.current == b'}' as u32 {
        self.advance();
        K_INFINITY
      } else {
        if !is_decimal_digit(self.current) {
          self.reset(start);
          return None;
        }
        let mut m = 0_i32;
        while is_decimal_digit(self.current) {
          let d = (self.current - b'0' as u32) as i32;
          m = m.saturating_mul(10).saturating_add(d);
          self.advance();
        }
        if self.current != b'}' as u32 {
          self.reset(start);
          return None;
        }
        self.advance();
        m
      }
    } else {
      self.reset(start);
      return None;
    };

    Some((min, max))
  }

  fn parse_back_reference_index(&mut self) -> Option<usize> {
    debug_assert!(self.current == b'\\' as u32);
    let next_digit = self.next();
    if !(b'1' as u32..=b'9' as u32).contains(&next_digit) {
      return None;
    }
    let start = self.position();
    let mut val = (next_digit - b'0' as u32) as usize;
    self.advance_by(2);
    while is_decimal_digit(self.current) {
      val = val * 10 + (self.current - b'0' as u32) as usize;
      if val > K_MAX_CAPTURES {
        self.reset(start);
        return None;
      }
      self.advance();
    }
    if val > self.captures_started {
      if !self.is_scanned_for_captures {
        self.scan_for_captures(false);
      }
      if val > self.capture_count {
        self.reset(start);
        return None;
      }
    }
    Some(val)
  }

  fn has_named_captures(&mut self, in_class: bool) -> bool {
    if self.has_named_captures || self.is_scanned_for_captures {
      return self.has_named_captures;
    }
    self.scan_for_captures(in_class);
    self.has_named_captures
  }

  fn scan_for_captures(&mut self, in_class: bool) {
    let saved_pos = self.position();
    let mut count = self.captures_started;
    if in_class {
      while self.has_more {
        let c = self.current;
        self.advance();
        if c == b'\\' as u32 {
          self.advance();
        } else if c == b']' as u32 {
          break;
        }
      }
    }

    while self.has_more {
      let n = self.current;
      self.advance();
      match u8::try_from(n).ok().map(char::from) {
        Some('\\') => {
          self.advance();
        }
        Some('[') => {
          let mut nest = 0;
          while self.has_more {
            let c = self.current;
            self.advance();
            if c == b'\\' as u32 {
              self.advance();
            } else if c == b'[' as u32 {
              if self.unicode_sets() {
                nest += 1;
              }
            } else if c == b']' as u32 {
              if nest == 0 {
                break;
              }
              nest -= 1;
            }
          }
        }
        Some('(') => {
          if self.current == b'?' as u32 {
            self.advance();
            if self.current == b'<' as u32 {
              self.advance();
              if self.current != b'=' as u32 && self.current != b'!' as u32 {
                self.has_named_captures = true;
                count += 1;
              }
            }
          } else {
            count += 1;
          }
        }
        _ => {}
      }
    }

    self.capture_count = count;
    self.is_scanned_for_captures = true;
    self.reset(saved_pos);
  }

  // 字符类解析
  fn parse_character_class(&mut self, builder: &Builder) -> Result<ClassSetResult> {
    self.advance(); // consume '['
    let mut is_negated = false;
    if self.current == b'^' as u32 {
      is_negated = true;
      self.advance();
    }
    let mut ranges = Vec::new();

    if self.current == b']' as u32 {
      self.advance();
      let tree = if self.unicode_sets() {
        Tree::ClassSetExpression(Box::new(ClassSetExpression::empty(is_negated)))
      } else {
        let mut flags = 0;
        if is_negated {
          flags |= NEGATED;
        }
        Tree::ClassRanges(Box::new(ClassRanges::new(ranges, flags)))
      };
      return Ok(ClassSetResult::new(Some(tree), false));
    }

    if !self.unicode_sets() {
      self.parse_class_ranges(&mut ranges)?;
      if !self.has_more {
        return Err(self.error(RegexpError::UnterminatedCharacterClass));
      }
      debug_assert_eq!(self.current, b']' as u32);
      self.advance();

      let mut class_ranges_flags = 0;
      if is_negated {
        class_ranges_flags |= NEGATED;
      }
      if !self.ignore_case() {
        class_ranges_flags |= NO_CASE_FOLDING_NEEDED;
      }
      let tree = Tree::ClassRanges(Box::new(ClassRanges::new(ranges, class_ranges_flags)));
      Ok(ClassSetResult::new(Some(tree), false))
    } else {
      let mut acc = ClassSetAccumulator::with_ranges(ranges);
      let operand = self.parse_class_set_operand(builder, &mut acc)?;

      match self.current {
        CH_MINUS if self.next() == CH_MINUS => {
          let op_tree = match operand.tree {
            Some(t) => t,
            None => {
              if acc.operand_type == ClassSetOperandType::ClassSetCharacter {
                self.add_maybe_case_folded_range(
                  &mut acc.ranges,
                  CharacterRange::singleton(acc.character),
                );
              }
              Tree::ClassSetOperand(Box::new(ClassSetOperand::new(
                acc.ranges,
                Some(acc.strings),
              )))
            }
          };
          self.parse_class_subtraction(builder, is_negated, op_tree, acc.operand_type)
        }
        CH_AMP if self.next() == CH_AMP => {
          let op_tree = match operand.tree {
            Some(t) => t,
            None => {
              if acc.operand_type == ClassSetOperandType::ClassSetCharacter {
                self.add_maybe_case_folded_range(
                  &mut acc.ranges,
                  CharacterRange::singleton(acc.character),
                );
              }
              Tree::ClassSetOperand(Box::new(ClassSetOperand::new(
                acc.ranges,
                Some(acc.strings),
              )))
            }
          };
          self.parse_class_intersection(builder, is_negated, op_tree, acc.operand_type)
        }
        _ => self.parse_class_union(builder, is_negated, operand, acc),
      }
    }
  }

  fn parse_class_ranges(&mut self, ranges: &mut Vec<CharacterRange>) -> Result<()> {
    while self.has_more && self.current != b']' as u32 {
      let (char_1, is_class_1) = self.parse_class_escape(ranges)?;
      if self.current == b'-' as u32 {
        self.advance();
        if !self.has_more {
          break;
        } else if self.current == b']' as u32 {
          if !is_class_1 {
            ranges.push(CharacterRange::singleton(char_1));
          }
          ranges.push(CharacterRange::singleton(b'-' as u32));
          break;
        }
        let (char_2, is_class_2) = self.parse_class_escape(ranges)?;
        if is_class_1 || is_class_2 {
          if self.is_unicode_mode() {
            return Err(self.error(RegexpError::InvalidCharacterClass));
          }
          if !is_class_1 {
            ranges.push(CharacterRange::singleton(char_1));
          }
          ranges.push(CharacterRange::singleton(b'-' as u32));
          if !is_class_2 {
            ranges.push(CharacterRange::singleton(char_2));
          }
          continue;
        }
        if char_1 > char_2 {
          return Err(self.error(RegexpError::OutOfOrderCharacterClass));
        }
        ranges.push(CharacterRange::range(char_1, char_2));
      } else if !is_class_1 {
        ranges.push(CharacterRange::singleton(char_1));
      }
    }
    Ok(())
  }

  fn parse_class_escape(&mut self, ranges: &mut Vec<CharacterRange>) -> Result<(u32, bool)> {
    if self.current != b'\\' as u32 {
      let c = self.current;
      self.advance();
      return Ok((c, false));
    }

    let next_char = self.next();
    if next_char == K_END_MARKER {
      return Err(self.error(RegexpError::EscapeAtEndOfPattern));
    }
    match next_char {
      0x62 /* 'b' */ => {
        self.advance_by(2);
        Ok((0x08, false))
      }
      0x2D /* '-' */ => {
        if self.is_unicode_mode() {
          self.advance_by(2);
          return Ok((b'-' as u32, false));
        }
        let c = self.parse_character_escape(true)?;
        Ok((c, false))
      }
      0x64 /* 'd' */ | 0x44 /* 'D' */ | 0x73 /* 's' */ | 0x53 /* 'S' */ | 0x77 /* 'w' */ | 0x57 /* 'W' */ => {
        let Some(std_set) = StandardCharacterSet::from_escape(next_char as u8 as char) else {
          return Err(self.error(RegexpError::InvalidEscape));
        };
        self.advance_by(2);
        CharacterRange::add_class_escape(
          std_set,
          ranges,
          self.is_unicode_mode() && self.ignore_case(),
        );
        Ok((0, true))
      }
      0x70 /* 'p' */ | 0x50 /* 'P' */ => {
        if !self.is_unicode_mode() {
          let c = self.parse_character_escape(true)?;
          return Ok((c, false));
        }
        let negate = next_char == b'P' as u32;
        self.advance_by(2);
        let mut strings = None;
        self.parse_property_escape(ranges, &mut strings, negate, true)?;
        Ok((0, true))
      }
      _ => {
        let c = self.parse_character_escape(true)?;
        Ok((c, false))
      }
    }
  }

  fn parse_property_escape(
    &mut self,
    ranges: &mut Vec<CharacterRange>,
    _strings: &mut Option<CharacterClassStrings>,
    negate: bool,
    in_class: bool,
  ) -> Result<ClassSetResult> {
    if self.current != b'{' as u32 {
      return Err(self.error(if in_class {
        RegexpError::InvalidClassPropertyName
      } else {
        RegexpError::InvalidPropertyName
      }));
    }
    self.advance();
    let mut name = String::new();
    while self.has_more && self.current != b'}' as u32 && self.current != b'=' as u32 {
      if let Ok(ch) = char::try_from(self.current)
        && (ch.is_ascii_alphanumeric() || ch == '_')
      {
        name.push(ch);
        self.advance();
        continue;
      }
      return Err(self.error(if in_class {
        RegexpError::InvalidClassPropertyName
      } else {
        RegexpError::InvalidPropertyName
      }));
    }

    if self.current == b'=' as u32 {
      self.advance();
      let mut val = String::new();
      while self.has_more && self.current != b'}' as u32 {
        if let Ok(ch) = char::try_from(self.current)
          && (ch.is_ascii_alphanumeric() || ch == '_')
        {
          val.push(ch);
          self.advance();
          continue;
        }
        return Err(self.error(if in_class {
          RegexpError::InvalidClassPropertyName
        } else {
          RegexpError::InvalidPropertyName
        }));
      }
      if self.current != b'}' as u32 {
        return Err(self.error(if in_class {
          RegexpError::InvalidClassPropertyName
        } else {
          RegexpError::InvalidPropertyName
        }));
      }
      self.advance();
      return self.lookup_property_value_name(&name, &val, negate, ranges);
    }

    if self.current != b'}' as u32 {
      return Err(self.error(if in_class {
        RegexpError::InvalidClassPropertyName
      } else {
        RegexpError::InvalidPropertyName
      }));
    }
    self.advance();

    // 常用属性查找
    if name == "Any" {
      if !negate {
        ranges.push(CharacterRange::everything());
      }
      return Ok(ClassSetResult::new(None, false));
    }
    if name == "ASCII" {
      if negate {
        ranges.push(CharacterRange::range(0x80, K_MAX_CODE_POINT));
      } else {
        ranges.push(CharacterRange::range(0x00, 0x7F));
      }
      return Ok(ClassSetResult::new(None, false));
    }

    self.lookup_property_name(&name, negate, ranges)
  }

  fn lookup_property_name(
    &self,
    name: &str,
    negate: bool,
    ranges: &mut Vec<CharacterRange>,
  ) -> Result<ClassSetResult> {
    match name {
      "ID_Start" => {
        let mut sub = Vec::new();
        let mut range_start = None;
        for cp in 0..=K_MAX_CODE_POINT {
          if is_identifier_start(cp) {
            if range_start.is_none() {
              range_start = Some(cp);
            }
          } else if let Some(start) = range_start.take() {
            sub.push(CharacterRange::range(start, cp - 1));
          }
        }
        if let Some(start) = range_start {
          sub.push(CharacterRange::range(start, K_MAX_CODE_POINT));
        }
        if negate {
          let mut neg = Vec::new();
          CharacterRange::negate(&sub, &mut neg);
          ranges.extend(neg);
        } else {
          ranges.extend(sub);
        }
        Ok(ClassSetResult::new(None, false))
      }
      "ID_Continue" => {
        let mut sub = Vec::new();
        let mut range_start = None;
        for cp in 0..=K_MAX_CODE_POINT {
          if is_identifier_part(cp) {
            if range_start.is_none() {
              range_start = Some(cp);
            }
          } else if let Some(start) = range_start.take() {
            sub.push(CharacterRange::range(start, cp - 1));
          }
        }
        if let Some(start) = range_start {
          sub.push(CharacterRange::range(start, K_MAX_CODE_POINT));
        }
        if negate {
          let mut neg = Vec::new();
          CharacterRange::negate(&sub, &mut neg);
          ranges.extend(neg);
        } else {
          ranges.extend(sub);
        }
        Ok(ClassSetResult::new(None, false))
      }
      _ => Err(self.error(RegexpError::InvalidPropertyName)),
    }
  }

  fn lookup_property_value_name(
    &self,
    _name: &str,
    _val: &str,
    _negate: bool,
    _ranges: &mut Vec<CharacterRange>,
  ) -> Result<ClassSetResult> {
    Err(self.error(RegexpError::InvalidPropertyName))
  }

  fn add_maybe_case_folded_range(&self, ranges: &mut Vec<CharacterRange>, range: CharacterRange) {
    ranges.push(range);
    CharacterRange::canonicalize(ranges);
  }

  // /v 模式下的集合操作
  fn parse_class_set_operand(
    &mut self,
    builder: &Builder,
    acc: &mut ClassSetAccumulator,
  ) -> Result<ClassSetResult> {
    let c = self.current;
    if c == b'\\' as u32 {
      let next = self.next();
      if next == b'q' as u32 {
        acc.operand_type = ClassSetOperandType::ClassStringDisjunction;
        return self.parse_class_string_disjunction(&mut acc.ranges, &mut acc.strings);
      }
      let (ch, is_escape) = self.parse_class_escape(&mut acc.ranges)?;
      if is_escape {
        acc.operand_type = ClassSetOperandType::CharacterClassEscape;
        return Ok(ClassSetResult::new(None, false));
      }
      acc.operand_type = ClassSetOperandType::ClassSetCharacter;
      acc.character = ch;
      return Ok(ClassSetResult::new(None, false));
    }

    if c == b'[' as u32 {
      acc.operand_type = ClassSetOperandType::NestedClass;
      return self.parse_character_class(builder);
    }

    acc.operand_type = ClassSetOperandType::ClassSetCharacter;
    acc.character = self.parse_class_set_character()?;
    Ok(ClassSetResult::new(None, false))
  }

  fn parse_class_set_character(&mut self) -> Result<u32> {
    if !self.has_more || self.current == K_END_MARKER {
      return Err(self.error(RegexpError::UnterminatedCharacterClass));
    }
    let c = self.current;
    if c == b'\\' as u32 {
      let next = self.next();
      if next == b'b' as u32 {
        self.advance_by(2);
        return Ok(0x08);
      }
      if next == K_END_MARKER {
        return Err(self.error(RegexpError::EscapeAtEndOfPattern));
      }
      return self.parse_character_escape(true);
    }

    if Self::is_class_set_syntax_character(c) {
      return Err(self.error(RegexpError::InvalidCharacterInClass));
    }
    if self.is_class_set_reserved_double_punctuator(c) {
      return Err(self.error(RegexpError::InvalidClassSetOperation));
    }
    self.advance();
    Ok(c)
  }

  fn parse_class_string_disjunction(
    &mut self,
    ranges: &mut Vec<CharacterRange>,
    strings: &mut CharacterClassStrings,
  ) -> Result<ClassSetResult> {
    self.advance_by(2); // consume \q
    if self.current != b'{' as u32 {
      return Err(self.error(RegexpError::InvalidEscape));
    }
    self.advance(); // consume '{'

    let mut string_buf = Vec::new();
    let mut contains_strings = false;

    while self.has_more && self.current != b'}' as u32 {
      if self.current == b'|' as u32 {
        contains_strings |= add_class_string(&string_buf, ranges, strings);
        string_buf.clear();
        self.advance();
      } else {
        let mut c = self.parse_class_set_character()?;
        if self.ignore_case()
          && let Some(ch) = char::from_u32(c)
        {
          c = ch.to_lowercase().next().unwrap_or(ch) as u32;
        }
        string_buf.push(c);
      }
    }

    contains_strings |= add_class_string(&string_buf, ranges, strings);
    CharacterRange::canonicalize(ranges);

    if self.current != b'}' as u32 {
      return Err(self.error(RegexpError::UnterminatedCharacterClass));
    }
    self.advance();
    Ok(ClassSetResult::new(None, contains_strings))
  }

  fn parse_class_union(
    &mut self,
    builder: &Builder,
    is_negated: bool,
    first_operand: ClassSetResult,
    mut acc: ClassSetAccumulator,
  ) -> Result<ClassSetResult> {
    let mut operands = Vec::new();
    let mut may_contain_strings = check_may_contain_strings(acc.operand_type, &first_operand);
    if let Some(tree) = first_operand.tree {
      operands.push(tree);
    }
    let mut last_type = acc.operand_type;

    while self.has_more && self.current != b']' as u32 {
      if self.current == b'-' as u32 {
        if self.next() == b'-' as u32 {
          return Err(self.error(RegexpError::InvalidClassSetOperation));
        }
        self.advance();
        if !self.has_more {
          break;
        }
        if last_type != ClassSetOperandType::ClassSetCharacter {
          return Err(self.error(RegexpError::InvalidCharacterClass));
        }
        let from = acc.character;
        self.parse_class_set_operand(builder, &mut acc)?;
        if acc.operand_type != ClassSetOperandType::ClassSetCharacter {
          return Err(self.error(RegexpError::InvalidCharacterClass));
        }
        if from > acc.character {
          return Err(self.error(RegexpError::OutOfOrderCharacterClass));
        }
        self
          .add_maybe_case_folded_range(&mut acc.ranges, CharacterRange::range(from, acc.character));
        last_type = ClassSetOperandType::ClassSetRange;
      } else {
        if last_type == ClassSetOperandType::ClassSetCharacter {
          self
            .add_maybe_case_folded_range(&mut acc.ranges, CharacterRange::singleton(acc.character));
        }
        let operand = self.parse_class_set_operand(builder, &mut acc)?;
        last_type = acc.operand_type;
        may_contain_strings |= check_may_contain_strings(last_type, &operand);
        if let Some(op_tree) = operand.tree {
          if !acc.ranges.is_empty() || !acc.strings.is_empty() {
            may_contain_strings |= !acc.strings.is_empty();
            let ranges = take(&mut acc.ranges);
            let strings = take(&mut acc.strings);
            let operand_node =
              Tree::ClassSetOperand(Box::new(ClassSetOperand::new(ranges, Some(strings))));
            operands.push(operand_node);
          }
          operands.push(op_tree);
        }
      }
    }

    if !self.has_more {
      return Err(self.error(RegexpError::UnterminatedCharacterClass));
    }

    if last_type == ClassSetOperandType::ClassSetCharacter {
      self.add_maybe_case_folded_range(&mut acc.ranges, CharacterRange::singleton(acc.character));
    }

    if !acc.ranges.is_empty() || !acc.strings.is_empty() {
      may_contain_strings |= !acc.strings.is_empty();
      operands.push(Tree::ClassSetOperand(Box::new(ClassSetOperand::new(
        acc.ranges,
        Some(acc.strings),
      ))));
    }

    debug_assert_eq!(self.current, b']' as u32);
    self.advance();

    if is_negated && may_contain_strings {
      return Err(self.error(RegexpError::NegatedCharacterClassWithStrings));
    }

    let tree = if operands.is_empty() {
      Tree::ClassSetExpression(Box::new(ClassSetExpression::empty(is_negated)))
    } else {
      Tree::ClassSetExpression(Box::new(ClassSetExpression::new(
        ClassSetOperation::Union,
        is_negated,
        may_contain_strings,
        operands,
      )))
    };

    Ok(ClassSetResult::new(Some(tree), may_contain_strings))
  }

  fn parse_class_intersection(
    &mut self,
    builder: &Builder,
    is_negated: bool,
    first_operand: Tree,
    first_operand_type: ClassSetOperandType,
  ) -> Result<ClassSetResult> {
    let mut may_contain_strings = may_contain_strings_tree(first_operand_type, &first_operand);
    let mut operands = vec![first_operand];

    while self.has_more && self.current != b']' as u32 {
      if self.current != b'&' as u32 || self.next() != b'&' as u32 {
        return Err(self.error(RegexpError::InvalidClassSetOperation));
      }
      self.advance_by(2);
      if self.current == b'&' as u32 {
        return Err(self.error(RegexpError::InvalidCharacterInClass));
      }

      let mut acc = ClassSetAccumulator::new();
      let operand = self.parse_class_set_operand(builder, &mut acc)?;

      let op_tree = match operand.tree {
        Some(t) => t,
        None => {
          if acc.operand_type == ClassSetOperandType::ClassSetCharacter {
            self.add_maybe_case_folded_range(
              &mut acc.ranges,
              CharacterRange::singleton(acc.character),
            );
          }
          Tree::ClassSetOperand(Box::new(ClassSetOperand::new(
            acc.ranges,
            Some(acc.strings),
          )))
        }
      };

      may_contain_strings &= may_contain_strings_tree(acc.operand_type, &op_tree);
      operands.push(op_tree);
    }

    if !self.has_more {
      return Err(self.error(RegexpError::UnterminatedCharacterClass));
    }
    if is_negated && may_contain_strings {
      return Err(self.error(RegexpError::NegatedCharacterClassWithStrings));
    }
    self.advance();

    let tree = Tree::ClassSetExpression(Box::new(ClassSetExpression::new(
      ClassSetOperation::Intersection,
      is_negated,
      may_contain_strings,
      operands,
    )));
    Ok(ClassSetResult::new(Some(tree), may_contain_strings))
  }

  fn parse_class_subtraction(
    &mut self,
    builder: &Builder,
    is_negated: bool,
    first_operand: Tree,
    first_operand_type: ClassSetOperandType,
  ) -> Result<ClassSetResult> {
    let may_contain_strings = may_contain_strings_tree(first_operand_type, &first_operand);
    if is_negated && may_contain_strings {
      return Err(self.error(RegexpError::NegatedCharacterClassWithStrings));
    }
    let mut operands = vec![first_operand];

    while self.has_more && self.current != b']' as u32 {
      if self.current != b'-' as u32 || self.next() != b'-' as u32 {
        return Err(self.error(RegexpError::InvalidClassSetOperation));
      }
      self.advance_by(2);

      let mut acc = ClassSetAccumulator::new();
      let operand = self.parse_class_set_operand(builder, &mut acc)?;

      let op_tree = match operand.tree {
        Some(t) => t,
        None => {
          if acc.operand_type == ClassSetOperandType::ClassSetCharacter {
            self.add_maybe_case_folded_range(
              &mut acc.ranges,
              CharacterRange::singleton(acc.character),
            );
          }
          Tree::ClassSetOperand(Box::new(ClassSetOperand::new(
            acc.ranges,
            Some(acc.strings),
          )))
        }
      };

      operands.push(op_tree);
    }

    if !self.has_more {
      return Err(self.error(RegexpError::UnterminatedCharacterClass));
    }
    self.advance();

    let tree = Tree::ClassSetExpression(Box::new(ClassSetExpression::new(
      ClassSetOperation::Subtraction,
      is_negated,
      may_contain_strings,
      operands,
    )));
    Ok(ClassSetResult::new(Some(tree), may_contain_strings))
  }
}

fn add_class_string(
  string: &[u32],
  ranges: &mut Vec<CharacterRange>,
  strings: &mut CharacterClassStrings,
) -> bool {
  if string.len() == 1 {
    ranges.push(CharacterRange::singleton(string[0]));
    false
  } else {
    let class_str = ClassString(string.to_vec());
    let mut atom_data = Vec::with_capacity(string.len() * 2);
    for &cp in string {
      push_code_unit(&mut atom_data, cp);
    }
    strings.emplace(class_str, Tree::Atom(Box::new(Atom::new(atom_data))));
    true
  }
}

fn check_may_contain_strings(operand_type: ClassSetOperandType, operand: &ClassSetResult) -> bool {
  match operand_type {
    ClassSetOperandType::ClassSetCharacter | ClassSetOperandType::ClassSetRange => false,
    ClassSetOperandType::CharacterClassEscape
    | ClassSetOperandType::ClassStringDisjunction
    | ClassSetOperandType::NestedClass => operand.may_contain_strings,
  }
}

fn may_contain_strings_tree(operand_type: ClassSetOperandType, tree: &Tree) -> bool {
  match operand_type {
    ClassSetOperandType::ClassSetCharacter | ClassSetOperandType::ClassSetRange => false,
    ClassSetOperandType::CharacterClassEscape | ClassSetOperandType::ClassStringDisjunction => {
      if let Tree::ClassSetOperand(o) = tree {
        o.has_strings()
      } else {
        false
      }
    }
    ClassSetOperandType::NestedClass => {
      if let Tree::ClassSetExpression(e) = tree {
        e.may_contain_strings
      } else {
        false
      }
    }
  }
}

fn push_code_unit(v: &mut Vec<u16>, code_unit: u32) {
  if code_unit <= 0xFFFF {
    v.push(code_unit as u16);
  } else {
    let lead = ((code_unit - 0x10000) >> 10) as u16 + 0xD800;
    let trail = ((code_unit - 0x10000) & 0x3FF) as u16 + 0xDC00;
    v.push(lead);
    v.push(trail);
  }
}

/// 解析 UTF-16 正则表达式字符串
pub fn parse_utf16(input: &[u16], flags: RegExpFlags) -> Result<CompileData> {
  flags.validate()?;
  let mut parser = Parser::new(input, flags);
  parser.parse()
}

/// 解析 UTF-8 正则表达式字符串
pub fn parse(input: &str, flags: RegExpFlags) -> Result<CompileData> {
  let utf16: Vec<u16> = input.encode_utf16().collect();
  parse_utf16(&utf16, flags)
}
