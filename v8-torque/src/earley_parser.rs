// 由 tool/src/torque/parser.js 从 v8/src/torque/earley-parser.h 生成; 修订就地改, 复跑覆盖
// Earley 解析器 (ParseResultValue 57 变体 + arena)

use std::{
  collections::{BTreeMap, BTreeSet, HashMap},
  mem::swap,
  vec::IntoIter,
};

use crate::{
  ast::{
    Annotation, AnnotationParameter, BitFieldDeclaration, ClassFieldExpression, GenericParameter,
    ImplicitParameters, IncrementDecrementOperator, IntegerLiteral, LabelAndTypes,
    NameAndExpression, NameAndTypeExpression, NodeId, ParameterList, StructFieldExpression,
    TypeswitchCase,
  },
  context::{SourcePositionScope, report_error},
  error::{Error, Result},
  ls::json::{JsonMember, JsonValue},
  source_positions::{LineAndColumn, SourceId, SourcePosition},
  torque_parser::{EnumEntry, ExpressionWithSource, TypeList},
  utils::string_literal_quote,
};

// ParseResult 的 dynamicany → Rust enum ParseResultValue (TypeId 变体 1:1);
// Symbol/Rule/Item 裸指针互联 → SymbolId/RuleId/ItemId arena (GrammarTables);
// InputPosition (const char*) → 字节偏移 usize + 源串显式传参 (登记偏差)。
/// 原版 ParseResultHolderBase::TypeId 1:1 (57 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ParseResultTypeId {
  StdString,
  Bool,
  Int32,
  Double,
  IntegerLiteral,
  StdVectorOfString,
  ExpressionPtr,
  IdentifierPtr,
  OptionalIdentifierPtr,
  StatementPtr,
  DeclarationPtr,
  TypeExpressionPtr,
  OptionalTypeExpressionPtr,
  TryHandlerPtr,
  NameAndTypeExpression,
  EnumEntry,
  StdVectorOfEnumEntry,
  ImplicitParameters,
  OptionalImplicitParameters,
  NameAndExpression,
  Annotation,
  VectorOfAnnotation,
  AnnotationParameter,
  OptionalAnnotationParameter,
  ClassFieldExpression,
  StructFieldExpression,
  BitFieldDeclaration,
  StdVectorOfNameAndTypeExpression,
  StdVectorOfNameAndExpression,
  StdVectorOfClassFieldExpression,
  StdVectorOfStructFieldExpression,
  StdVectorOfBitFieldDeclaration,
  IncrementDecrementOperator,
  OptionalStdString,
  StdVectorOfStatementPtr,
  StdVectorOfDeclarationPtr,
  StdVectorOfStdVectorOfDeclarationPtr,
  StdVectorOfExpressionPtr,
  ExpressionWithSource,
  ParameterList,
  TypeList,
  OptionalTypeList,
  LabelAndTypes,
  StdVectorOfLabelAndTypes,
  StdVectorOfTryHandlerPtr,
  OptionalStatementPtr,
  OptionalExpressionPtr,
  TypeswitchCase,
  StdVectorOfTypeswitchCase,
  StdVectorOfIdentifierPtr,
  OptionalClassBody,
  GenericParameter,
  GenericParameters,
  JsonValue,
  JsonMember,
  StdVectorOfJsonValue,
  StdVectorOfJsonMember,
}

/// 原版 ParseResult (类型擦除 holder) → 值枚举; ParseResult 即其别名
#[derive(Debug, Clone)]
pub enum ParseResultValue {
  StdString(String),
  Bool(bool),
  Int32(i32),
  Double(f64),
  IntegerLiteral(IntegerLiteral),
  StdVectorOfString(Vec<String>),
  ExpressionPtr(NodeId),
  IdentifierPtr(NodeId),
  OptionalIdentifierPtr(Option<NodeId>),
  StatementPtr(NodeId),
  DeclarationPtr(NodeId),
  TypeExpressionPtr(NodeId),
  OptionalTypeExpressionPtr(Option<NodeId>),
  TryHandlerPtr(NodeId),
  NameAndTypeExpression(NameAndTypeExpression),
  EnumEntry(EnumEntry),
  StdVectorOfEnumEntry(Vec<EnumEntry>),
  ImplicitParameters(ImplicitParameters),
  OptionalImplicitParameters(Option<ImplicitParameters>),
  NameAndExpression(NameAndExpression),
  Annotation(Annotation),
  VectorOfAnnotation(Vec<Annotation>),
  AnnotationParameter(AnnotationParameter),
  OptionalAnnotationParameter(Option<AnnotationParameter>),
  ClassFieldExpression(ClassFieldExpression),
  StructFieldExpression(StructFieldExpression),
  BitFieldDeclaration(BitFieldDeclaration),
  StdVectorOfNameAndTypeExpression(Vec<NameAndTypeExpression>),
  StdVectorOfNameAndExpression(Vec<NameAndExpression>),
  StdVectorOfClassFieldExpression(Vec<ClassFieldExpression>),
  StdVectorOfStructFieldExpression(Vec<StructFieldExpression>),
  StdVectorOfBitFieldDeclaration(Vec<BitFieldDeclaration>),
  IncrementDecrementOperator(IncrementDecrementOperator),
  OptionalStdString(Option<String>),
  StdVectorOfStatementPtr(Vec<NodeId>),
  StdVectorOfDeclarationPtr(Vec<NodeId>),
  StdVectorOfStdVectorOfDeclarationPtr(Vec<Vec<NodeId>>),
  StdVectorOfExpressionPtr(Vec<NodeId>),
  ExpressionWithSource(ExpressionWithSource),
  ParameterList(ParameterList),
  TypeList(TypeList),
  OptionalTypeList(Option<TypeList>),
  LabelAndTypes(LabelAndTypes),
  StdVectorOfLabelAndTypes(Vec<LabelAndTypes>),
  StdVectorOfTryHandlerPtr(Vec<NodeId>),
  OptionalStatementPtr(Option<NodeId>),
  OptionalExpressionPtr(Option<NodeId>),
  TypeswitchCase(TypeswitchCase),
  StdVectorOfTypeswitchCase(Vec<TypeswitchCase>),
  StdVectorOfIdentifierPtr(Vec<NodeId>),
  OptionalClassBody(Option<NodeId>),
  GenericParameter(GenericParameter),
  GenericParameters(Vec<GenericParameter>),
  JsonValue(JsonValue),
  JsonMember(JsonMember),
  StdVectorOfJsonValue(Vec<JsonValue>),
  StdVectorOfJsonMember(Vec<JsonMember>),
}

/// 原版 using ParseResultTypeId
pub type ParseResult = ParseResultValue;

/// 原版 using InputPosition = const char* → 字节偏移
pub type InputPosition = usize;

/// 原版 MatchedInput 1:1 (begin/end 为源串字节偏移)
#[derive(Debug, Clone)]
pub struct MatchedInput {
  pub begin: InputPosition,
  pub end: InputPosition,
  pub pos: SourcePosition,
}

impl MatchedInput {
  /// 原版 ToString: 取源串切片
  #[must_use]
  pub fn matched_text<'a>(&self, input: &'a str) -> &'a str {
    &input[self.begin..self.end]
  }
}

/// 原版 ParseResultIterator 1:1 (产生式动作的子结果游标)
/// 登记偏差: 源串随游标携带 (原版 MatchedInput 持裸指针), 供 YieldMatchedInput 取文本
/// results 用 IntoIter 承接: Next 按值移动, 避免 ParseResultValue (String/Vec 载荷) 深拷贝
#[derive(Debug)]
pub struct ParseResultIterator<'a> {
  results: IntoIter<ParseResultValue>,
  matched_input: MatchedInput,
  input: &'a str,
}

impl<'a> ParseResultIterator<'a> {
  #[must_use]
  pub fn new(results: Vec<ParseResultValue>, matched_input: MatchedInput, input: &'a str) -> Self {
    Self {
      results: results.into_iter(),
      matched_input,
      input,
    }
  }

  /// 原版 Next (越界为调用方契约 CHECK; Rust 以 Option 表达)
  pub fn next_result(&mut self) -> Option<ParseResultValue> {
    self.results.next()
  }

  #[must_use]
  pub fn has_next(&self) -> bool {
    !self.results.as_slice().is_empty()
  }

  #[must_use]
  pub const fn matched_input(&self) -> &MatchedInput {
    &self.matched_input
  }

  /// 本产生式覆盖的源文本 (原版 matched_input().ToString())
  #[must_use]
  pub fn matched_text(&self) -> &'a str {
    &self.input[self.matched_input.begin..self.matched_input.end]
  }
}

/// 原版 LexerResult 1:1
#[derive(Debug, Clone)]
pub struct LexerResult {
  pub token_symbols: Vec<SymbolId>,
  pub token_contents: Vec<MatchedInput>,
}

/// 原版 Action = optional<ParseResult>(*)(ParseResultIterator*)
/// 错误流: 原版动作内 ReportError 抛异常 → Rust 以 Err 传播 (Abort 收敛于编译驱动)
pub type Action = for<'a> fn(&mut ParseResultIterator<'a>) -> Result<Option<ParseResultValue>>;

/// 原版 DefaultAction (earley-parser.h:197-201)
pub fn default_action(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  if !child_results.has_next() {
    return Ok(None);
  }
  Ok(child_results.next_result())
}

/// 符号 arena 索引 (对应 C++ Symbol*)
pub type SymbolId = u32;
/// 规则 arena 索引 (对应 C++ Rule*)
pub type RuleId = u32;
/// 项 arena 索引 (对应 C++ const Item*)
pub type ItemId = u32;

/// 原版 Rule 1:1 (left 由 Symbol::AddRule 回填 → Option)
#[derive(Debug, Clone)]
pub struct Rule {
  pub left: Option<SymbolId>,
  pub right: Vec<SymbolId>,
  pub action: Action,
}

/// 原版 Symbol 1:1 (终结符 = 无规则)
#[derive(Debug, Clone, Default)]
pub struct Symbol {
  pub rules: Vec<RuleId>,
}

impl Symbol {
  #[must_use]
  pub fn is_terminal(&self) -> bool {
    self.rules.is_empty()
  }

  /// 原版 rule_number
  #[must_use]
  pub fn rule_number(&self) -> usize {
    self.rules.len()
  }
}

/// 原版 Item 1:1 (Earley 核心项; prev/child 供 AST 重建)
#[derive(Debug, Clone, Copy)]
pub struct Item {
  pub rule: RuleId,
  pub mark: usize,
  pub start: usize,
  pub pos: usize,
  pub prev: Option<ItemId>,
  pub child: Option<ItemId>,
}

impl Item {
  /// 原版 IsComplete: mark 抵达产生式右端
  #[must_use]
  pub fn is_complete(&self, tables: &GrammarTables) -> bool {
    self.mark == tables.rules[self.rule as usize].right.len()
  }

  /// 原版 NextSymbol: mark 后的第一个符号 (调用方保证未完结)
  #[must_use]
  pub fn next_symbol(&self, tables: &GrammarTables) -> SymbolId {
    tables.rules[self.rule as usize].right[self.mark]
  }

  /// 等价键 (rule/mark/start/pos; prev/child 不参与, 对应原版 operator==)
  #[must_use]
  pub const fn key(&self) -> ItemKey {
    (self.rule, self.mark, self.start, self.pos)
  }
}

/// Item 等价键 (原版 operator==/hash 范围)
pub type ItemKey = (RuleId, usize, usize, usize);

/// 符号/规则 arena (Symbol 稳定地址由下标承担)
#[derive(Debug, Default)]
pub struct GrammarTables {
  pub symbols: Vec<Symbol>,
  pub rules: Vec<Rule>,
}

impl GrammarTables {
  /// 原版 Symbol::AddRule: 建 Rule 并回填 left
  pub fn add_rule(&mut self, left: SymbolId, right: Vec<SymbolId>, action: Action) -> RuleId {
    let r = Rule {
      left: Some(left),
      right,
      action,
    };
    self.rules.push(r);
    let id = (self.rules.len() - 1) as RuleId;
    self.symbols[left as usize].rules.push(id);
    id
  }

  /// 原版 NewSymbol
  pub fn new_symbol(&mut self) -> SymbolId {
    self.symbols.push(Symbol::default());
    (self.symbols.len() - 1) as SymbolId
  }

  #[must_use]
  pub fn rule_right(&self, r: RuleId) -> &[SymbolId] {
    &self.rules[r as usize].right
  }

  /// 规则左部 (AddRule 后必有值)
  #[must_use]
  pub fn rule_left(&self, r: RuleId) -> SymbolId {
    self.rules[r as usize].left.unwrap_or(u32::MAX)
  }
}

/// 原版 node_hash_set<Item> processed → 项 arena + 键索引
/// (指针稳定性由下标承担; CheckAmbiguity 比对所需的 prev/child 原样保留)
#[derive(Debug, Default)]
pub struct ItemArena {
  items: Vec<Item>,
  index: HashMap<ItemKey, ItemId>,
}

impl ItemArena {
  /// 插入; 返回 (既存或新下标, 是否新项)
  fn insert(&mut self, item: Item) -> (ItemId, bool) {
    let key = item.key();
    if let Some(&id) = self.index.get(&key) {
      return (id, false);
    }
    let id = self.items.len() as ItemId;
    self.items.push(item);
    self.index.insert(key, id);
    (id, true)
  }

  /// 原版 processed->find(Item{...})
  #[must_use]
  pub fn find(&self, key: ItemKey) -> Option<ItemId> {
    self.index.get(&key).copied()
  }

  #[must_use]
  pub fn get(&self, id: ItemId) -> Item {
    self.items[id as usize]
  }
}

/// 原版 Item::Children (earley-parser.cc:72-81): 沿 prev 链收集 child, 逆序还原
fn item_children(arena: &ItemArena, item: Item) -> Vec<Option<ItemId>> {
  let mut children = Vec::new();
  let mut current = item;
  while let Some(prev) = current.prev {
    children.push(current.child);
    current = arena.items[prev as usize];
  }
  children.reverse();
  children
}

/// 原版 Item::Advance (earley-parser.h:306-317): mark 前移, prev 指回源项
fn advance(src: Item, src_id: ItemId, new_pos: usize, child: Option<ItemId>) -> Item {
  Item {
    rule: src.rule,
    mark: src.mark + 1,
    start: src.start,
    pos: new_pos,
    prev: Some(src_id),
    child,
  }
}

/// 原版 Item::GetMatchedInput (earley-parser.h:326-334): 项覆盖的源区间
#[must_use]
pub fn item_matched_input(item: &Item, tokens: &LexerResult) -> MatchedInput {
  let start = &tokens.token_contents[item.start];
  let end = if item.start == item.pos {
    &tokens.token_contents[item.start]
  } else {
    &tokens.token_contents[item.pos - 1]
  };
  MatchedInput {
    begin: start.begin,
    end: end.end,
    pos: SourcePosition {
      source: start.pos.source,
      start: start.pos.start,
      end: end.pos.end,
    },
  }
}

/// 原版 Item::SplitByChildren (earley-parser.cc:83-98): 按下一层子项切分源文本
fn split_by_children(
  tables: &GrammarTables,
  arena: &ItemArena,
  tokens: &LexerResult,
  input: &str,
  item: Item,
) -> String {
  if tables.rule_right(item.rule).len() == 1
    && let Some(Some(child)) = item_children(arena, item).first()
  {
    return split_by_children(tables, arena, tokens, input, arena.get(*child));
  }
  let mut parts = Vec::new();
  for child in item_children(arena, item).into_iter().flatten() {
    let matched = item_matched_input(&arena.get(child), tokens);
    parts.push(input[matched.begin..matched.end].to_owned());
  }
  parts.join("  ")
}

/// 原版 Item::CheckAmbiguity (earley-parser.cc:100-117):
/// 同键不同推导 (child 或 prev 分叉) → 二义文法错误 (原版拼写 "grammer" 保留)
fn check_ambiguity(
  tables: &GrammarTables,
  arena: &ItemArena,
  tokens: &LexerResult,
  input: &str,
  existing: Item,
  new: Item,
) -> Result<()> {
  if existing.child != new.child {
    let (Some(old_child), Some(new_child)) = (existing.child, new.child) else {
      return Ok(());
    };
    let old_match = item_matched_input(&arena.get(old_child), tokens);
    let text = &input[old_match.begin..old_match.end];
    return Err(Error::Message(format!(
      "ambiguous grammer rules for \"{text}\":\n   {}\nvs\n   {}",
      split_by_children(tables, arena, tokens, input, arena.get(old_child)),
      split_by_children(tables, arena, tokens, input, arena.get(new_child)),
    )));
  }
  if existing.prev != new.prev {
    let matched = item_matched_input(&existing, tokens);
    let text = &input[matched.begin..matched.end];
    return Err(Error::Message(format!(
      "ambiguous grammer rules for \"{text}\":\n   {}\nvs\n   {}",
      split_by_children(tables, arena, tokens, input, existing),
      split_by_children(tables, arena, tokens, input, new),
    )));
  }
  Ok(())
}

/// 原版 Lexer 1:1 (patterns/keywords 存 SymbolId, arena 在 GrammarTables)
#[derive(Debug, Default)]
pub struct Lexer {
  pub whitespace: Option<PatternFunction>,
  pub patterns: Vec<(PatternFunction, SymbolId)>,
  pub keywords: BTreeMap<String, SymbolId>,
}

/// 原版 PatternFunction = bool(*)(InputPosition*) → (源串, 游标) 版
pub type PatternFunction = fn(&str, &mut InputPosition) -> bool;

/// 原版 LineAndColumnTracker (earley-parser.cc:22-42): 字节区间 → 行列推进
struct LineAndColumnTracker {
  source: SourceId,
  previous: LineAndColumn,
  current: LineAndColumn,
}

impl LineAndColumnTracker {
  fn new(origin: SourcePosition) -> Self {
    Self {
      source: origin.source,
      previous: origin.start,
      current: origin.start,
    }
  }

  fn advance(&mut self, input: &str, from: usize, to: usize) {
    self.previous = self.current;
    self.current.offset += (to - from) as i32;
    for &b in &input.as_bytes()[from..to] {
      if b == b'\n' {
        self.current.line += 1;
        self.current.column = 0;
      } else {
        self.current.column += 1;
      }
    }
  }

  fn to_source_position(&self) -> SourcePosition {
    SourcePosition {
      source: self.source,
      start: self.previous,
      end: self.current,
    }
  }
}

impl Lexer {
  /// 原版 SetWhitespace
  pub fn set_whitespace(&mut self, ws: PatternFunction) {
    self.whitespace = Some(ws);
  }

  /// 原版 Pattern: 登记 pattern 终结符
  pub fn pattern(&mut self, tables: &mut GrammarTables, f: PatternFunction) -> SymbolId {
    let s = tables.new_symbol();
    self.patterns.push((f, s));
    s
  }

  /// 原版 Token: 关键字终结符 (同字面量复用)
  pub fn token(&mut self, tables: &mut GrammarTables, keyword: &str) -> SymbolId {
    if let Some(&s) = self.keywords.get(keyword) {
      return s;
    }
    let s = tables.new_symbol();
    self.keywords.insert(keyword.to_owned(), s);
    s
  }

  fn skip_whitespace(&self, input: &str, pos: &mut usize) {
    if let Some(ws) = self.whitespace {
      ws(input, pos);
    }
  }

  /// 原版 MatchToken (earley-parser.cc:163-190): pattern 最长匹配,
  /// 关键字在不短于 pattern 时优先 (倒序使长关键字先试)
  fn match_token(&self, input: &str, pos: &mut usize) -> Option<SymbolId> {
    let token_start = *pos;
    let mut symbol = None;
    for &(f, s) in &self.patterns {
      let mut token_end = token_start;
      if f(input, &mut token_end) && token_end > *pos {
        *pos = token_end;
        symbol = Some(s);
      }
    }
    let pattern_size = *pos - token_start;

    for (keyword, &keyword_symbol) in self.keywords.iter().rev() {
      if input.len() - token_start < keyword.len() {
        continue;
      }
      if keyword.len() >= pattern_size && input[token_start..].starts_with(keyword.as_str()) {
        *pos = token_start + keyword.len();
        return Some(keyword_symbol);
      }
    }
    if pattern_size > 0 { symbol } else { None }
  }

  /// 原版 RunLexer(input, origin) (earley-parser.cc:124-161)
  pub fn run_lexer(&self, input: &str, origin: SourcePosition) -> Result<LexerResult> {
    let mut result = LexerResult {
      token_symbols: Vec::new(),
      token_contents: Vec::new(),
    };
    let mut tracker = LineAndColumnTracker::new(origin);
    let mut pos = 0usize;
    let mut token_start = 0usize;
    self.skip_whitespace(input, &mut pos);
    tracker.advance(input, token_start, pos);
    while pos < input.len() {
      token_start = pos;
      let symbol = self.match_token(input, &mut pos);
      let token_end = pos;
      tracker.advance(input, token_start, token_end);
      let Some(symbol) = symbol else {
        // 原版取至多 10 字符的片段入消息 (earley-parser.cc:144-147)
        let snippet_end = (token_start + 10).min(input.len());
        let scope = SourcePositionScope::new(tracker.to_source_position());
        let err = report_error(format!(
          "Lexer Error: unknown token {}",
          string_literal_quote(&input[token_start..snippet_end])
        ));
        drop(scope);
        return Err(err);
      };
      result.token_symbols.push(symbol);
      result.token_contents.push(MatchedInput {
        begin: token_start,
        end: token_end,
        pos: tracker.to_source_position(),
      });
      self.skip_whitespace(input, &mut pos);
      tracker.advance(input, token_end, pos);
    }
    // 追加哨兵 token 位置 (原版 corner-case 简化)
    tracker.advance(input, token_start, pos);
    result.token_contents.push(MatchedInput {
      begin: pos,
      end: pos,
      pos: tracker.to_source_position(),
    });
    Ok(result)
  }
}

/// 原版 Grammar 1:1 (子类化模式 → TorqueGrammar 组合 Grammar, 见 torque_parser.rs)
#[derive(Debug, Default)]
pub struct Grammar {
  pub tables: GrammarTables,
  pub lexer: Lexer,
  pub generated_symbols: Vec<SymbolId>,
  pub start: Option<SymbolId>,
}

impl Grammar {
  /// 原版 Token/Pattern/SetWhitespace 转发
  pub fn token(&mut self, s: &str) -> SymbolId {
    let tables = &mut self.tables;
    self.lexer.token(tables, s)
  }

  pub fn pattern(&mut self, f: PatternFunction) -> SymbolId {
    let tables = &mut self.tables;
    self.lexer.pattern(tables, f)
  }

  pub fn set_whitespace(&mut self, ws: PatternFunction) {
    self.lexer.set_whitespace(ws);
  }

  /// 原版 NewSymbol: 造生成符号 (存 generated_symbols)
  pub fn new_symbol(&mut self) -> SymbolId {
    let s = self.tables.new_symbol();
    self.generated_symbols.push(s);
    s
  }

  /// 原版 Sequence: 符号序列符号 (at most one 结果)
  pub fn sequence(&mut self, symbols: Vec<SymbolId>) -> SymbolId {
    let s = self.new_symbol();
    self.tables.add_rule(s, symbols, yield_matched_input);
    s
  }

  /// 原版 NonemptyList (T 参数经动作函数指针编码 → 显式传 singleton/extend 动作)
  pub fn nonempty_list(
    &mut self,
    element: SymbolId,
    separator: Option<SymbolId>,
    singleton: Action,
    extend: Action,
  ) -> SymbolId {
    let list = self.new_symbol();
    let r1 = Rule {
      left: Some(list),
      right: vec![element],
      action: singleton,
    };
    let r2 = Rule {
      left: Some(list),
      right: separator.map_or(vec![list, element], |sep| vec![list, sep, element]),
      action: extend,
    };
    self.tables.rules.push(r1);
    self.tables.symbols[list as usize]
      .rules
      .push((self.tables.rules.len() - 1) as RuleId);
    self.tables.rules.push(r2);
    self.tables.symbols[list as usize]
      .rules
      .push((self.tables.rules.len() - 1) as RuleId);
    list
  }

  /// 原版 CheckIf
  pub fn check_if(&mut self, x: SymbolId) -> SymbolId {
    let s = self.new_symbol();
    self.tables.add_rule(s, vec![x], yield_true);
    self.tables.add_rule(s, vec![], yield_false);
    s
  }

  /// 原版 Parse(input) (earley-parser.h:425-428)
  pub fn parse(&mut self, input: &str, origin: SourcePosition) -> Result<Option<ParseResultValue>> {
    let Some(start) = self.start else {
      return Err(Error::Message("grammar has no start symbol".into()));
    };
    self.parse_from(start, input, origin)
  }

  /// 原版 Parse(start, input, origin) (earley-parser.h:430-434): 指定起始符号解析
  pub fn parse_from(
    &mut self,
    start: SymbolId,
    input: &str,
    origin: SourcePosition,
  ) -> Result<Option<ParseResultValue>> {
    let tokens = self.lexer.run_lexer(input, origin)?;
    parse_tokens(start, &tokens, &mut self.tables, input)
  }
}

/// 原版 YieldMatchedInput (earley-parser.h:461-464): 取规则覆盖的源文本
pub fn yield_matched_input(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  Ok(Some(ParseResultValue::StdString(
    child_results.matched_text().to_owned(),
  )))
}

/// 原版 YieldIntegralConstant<bool, true/false>
pub fn yield_true(_child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  Ok(Some(ParseResultValue::Bool(true)))
}

pub fn yield_false(_child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  Ok(Some(ParseResultValue::Bool(false)))
}

/// 原版 Rule::RunAction (earley-parser.cc:46-62): 递归跑子项动作,
/// CurrentSourcePosition 作用于动作期间; 动作须耗尽全部子结果 (原 CHECK)
fn run_rule_action(
  tables: &GrammarTables,
  arena: &ItemArena,
  tokens: &LexerResult,
  input: &str,
  item_id: ItemId,
) -> Result<Option<ParseResultValue>> {
  let item = arena.get(item_id);
  let mut results = Vec::new();
  for child in item_children(arena, item) {
    let Some(child) = child else { continue };
    if let Some(r) = run_rule_action(tables, arena, tokens, input, child)? {
      results.push(r);
    }
  }
  let matched = item_matched_input(&item, tokens);
  let scope = SourcePositionScope::new(matched.pos);
  let mut iter = ParseResultIterator::new(results, matched, input);
  let result = (tables.rules[item.rule as usize].action)(&mut iter)?;
  let consumed = !iter.has_next();
  drop(scope);
  if !consumed {
    return Err(Error::Message(
      "parse action did not consume all child results".into(),
    ));
  }
  Ok(result)
}

/// 原版 RunEarleyAlgorithm (earley-parser.cc:194-290) — Earley 解析核心:
/// top_level 包装符号 + Complete/Scan/Predict 三相 + 空产生式即时完结
/// (Aycock-Horspool 2002 §3); 同键不同推导 → CheckAmbiguity
pub fn run_earley_algorithm(
  start: SymbolId,
  tokens: &LexerResult,
  tables: &mut GrammarTables,
  arena: &mut ItemArena,
  input: &str,
) -> Result<ItemId> {
  let mut worklist: Vec<Item> = Vec::new();
  let mut future_items: Vec<Item> = Vec::new();
  let mut waiting: HashMap<(usize, SymbolId), BTreeSet<ItemId>> = HashMap::new();
  // 仅失败诊断需要「最后处理的项」; 不随项数增长 (替代原 debug_trace 全量记录)
  let mut last_id: Option<ItemId> = None;

  // top_level: 单一规则包住 start, 屏蔽 start 的多规则性
  let top_symbol = tables.new_symbol();
  let top_rule = tables.add_rule(top_symbol, vec![start], default_action);
  worklist.push(Item {
    rule: top_rule,
    mark: 0,
    start: 0,
    pos: 0,
    prev: None,
    child: None,
  });

  let input_length = tokens.token_symbols.len();

  for pos in 0..=input_length {
    while let Some(new_item) = worklist.pop() {
      let (item_id, is_new) = arena.insert(new_item);
      if !is_new {
        check_ambiguity(tables, arena, tokens, input, arena.get(item_id), new_item)?;
        continue;
      }
      last_id = Some(item_id);
      let current = arena.get(item_id);
      if current.is_complete(tables) {
        // Complete: 唤醒所有等此符号完结的项
        let left = tables.rule_left(current.rule);
        if let Some(waiters) = waiting.get(&(current.start, left)) {
          for &parent in waiters {
            worklist.push(advance(arena.get(parent), parent, pos, Some(item_id)));
          }
        }
      } else {
        let next = current.next_symbol(tables);
        // Scan: 终结符直接对齐输入
        if pos < input_length && tokens.token_symbols[pos] == next {
          future_items.push(advance(current, item_id, pos + 1, None));
        }
        // Predict: 非终结符登记等待 + 展开全部规则
        if !tables.symbols[next as usize].is_terminal() {
          waiting.entry((pos, next)).or_default().insert(item_id);
        }
        for &rule in &tables.symbols[next as usize].rules {
          let already_completed = arena.find((rule, tables.rule_right(rule).len(), pos, pos));
          match already_completed {
            // 空产生式已在当前位完结 → 立即前进 (Aycock-Horspool)
            Some(completed) => worklist.push(advance(current, item_id, pos, Some(completed))),
            None => worklist.push(Item {
              rule,
              mark: 0,
              start: pos,
              pos,
              prev: None,
              child: None,
            }),
          }
        }
      }
    }
    swap(&mut worklist, &mut future_items);
  }

  if let Some(final_item) = arena.find((top_rule, 1, 0, input_length)) {
    // 成功: top_level 规则覆盖全输入, 返回其唯一子项 (left == start)
    return item_children(arena, arena.get(final_item))
      .into_iter()
      .flatten()
      .next()
      .ok_or_else(|| Error::Message("start symbol produced no completed item".into()));
  }
  // 失败: 取最后处理到的项定位下一个 token (earley-parser.cc:281-289)
  let last = last_id.map(|id| arena.get(id));
  let reason = match last {
    Some(last) if last.pos < tokens.token_symbols.len() => {
      let matched = &tokens.token_contents[last.pos];
      format!(
        "unexpected token \"{}\"",
        &input[matched.begin..matched.end]
      )
    }
    _ => "unexpected end of input".to_owned(),
  };
  let scope = SourcePositionScope::new(
    last
      .map(|l| tokens.token_contents[l.pos].pos)
      .unwrap_or_default(),
  );
  let err = report_error(format!("Parser Error: {reason}"));
  drop(scope);
  Err(err)
}

/// 原版 ParseTokens (earley-parser.h:374-379): Earley 主流程 + 起始符号动作
pub fn parse_tokens(
  start: SymbolId,
  tokens: &LexerResult,
  tables: &mut GrammarTables,
  input: &str,
) -> Result<Option<ParseResultValue>> {
  let mut arena = ItemArena::default();
  let final_child = run_earley_algorithm(start, tokens, tables, &mut arena, input)?;
  run_rule_action(tables, &arena, tokens, input, final_child)
}

// ── 原版 src/torque/earley-parser.cc ──
// L23: SourceId source = CurrentSourceFile::Get()
// L27: void Advance(InputPosition from, InputPosition to)
// L41: SourcePosition ToSourcePosition()
// L46: std::optional<ParseResult> Rule::RunAction(const Item* completed_item, const LexerResult& tokens) const
// L64: Symbol& Symbol::operator=(std::initializer_list<Rule> rules)
// L72: std::vector<const Item*> Item::Children() const
// L83: std::string Item::SplitByChildren(const LexerResult& tokens) const
// L100: void Item::CheckAmbiguity(const Item& other, const LexerResult& tokens) const
// L119: LexerResult Lexer::RunLexer(const std::string& input)
// L124: LexerResult Lexer::RunLexer(const std::string& input, SourcePosition origin)
// L163: Symbol* Lexer::MatchToken(InputPosition* pos, InputPosition end)
// L194: const Item* RunEarleyAlgorithm( Symbol* start, const LexerResult& tokens, absl::node_hash_set<Item, base::hash<Item>>* processed)
// L294: bool Grammar::MatchChar(int (*char_class)(int), InputPosition* pos)
// L303: bool Grammar::MatchChar(bool (*char_class)(char), InputPosition* pos)
// L312: bool Grammar::MatchString(const char* s, InputPosition* pos)
// L322: bool Grammar::MatchAnyChar(InputPosition* pos)
