// 由 tool/src/torque/parser.js 从 v8/src/torque/torque-parser.h 生成; 修订就地改, 复跑覆盖
// Torque 文法: 解析入口 + token 表 + 产生式动作全量 (1:1 对齐 torque-parser.cc)

use std::{
  collections::{BTreeMap, BTreeSet},
  mem::take,
};

use crate::{
  ast::{
    AbstractTypeDeclaration, Annotation, AnnotationParameter, AssertKind, AssertStatement,
    AssignmentExpression, AssumeTypeImpossibleExpression, Ast, AstNode, AstNodeKind,
    BasicTypeExpression, BitFieldDeclaration, BitFieldStructDeclaration, BlockStatement,
    BreakStatement, CallExpression, CallMethodExpression, ClassBody, ClassDeclaration,
    ClassFieldExpression, ClassFieldIndexInfo, ConditionalAnnotation, ConditionalAnnotationType,
    ConditionalExpression, ConstDeclaration, ContinueStatement, CppIncludeDeclaration, DebugKind,
    DebugStatement, DereferenceExpression, ElementAccessExpression, EnumDescription, EnumEntryDesc,
    ExpressionStatement, ExternConstDeclaration, ExternalBuiltinDeclaration,
    ExternalMacroDeclaration, ExternalRuntimeDeclaration, FieldAccessExpression,
    FloatingPointLiteralExpression, ForLoopStatement, FunctionTypeExpression,
    GenericCallableDeclaration, GenericParameter, GenericTypeDeclaration, GotoStatement,
    HandlerKind, Identifier, IdentifierExpression, IfStatement, ImplicitKind, ImplicitParameters,
    IncludeSelector, IncrementDecrementExpression, IncrementDecrementOperator,
    InstanceTypeConstraints, IntegerLiteral, IntegerLiteralExpression, IntrinsicCallExpression,
    IntrinsicDeclaration, LabelAndTypes, LogicalAndExpression, LogicalOrExpression,
    NameAndExpression, NameAndTypeExpression, NamespaceDeclaration, NewExpression, NodeHeader,
    NodeId, ParameterList, ReturnStatement, SpecializationDeclaration, SpreadExpression,
    StatementExpression, StringLiteralExpression, StructDeclaration, StructExpression,
    StructFieldExpression, TailCallStatement, TorqueBuiltinDeclaration, TorqueMacroDeclaration,
    TryHandler, TryLabelExpression, TypeAliasDeclaration, TypeswitchCase, UnionTypeExpression,
    VarDeclarationStatement, WhileStatement,
  },
  constants::{
    ANNOTATION_ABSTRACT, ANNOTATION_CPP_ACQUIRE_LOAD, ANNOTATION_CPP_OBJECT_LAYOUT_DEFINITION,
    ANNOTATION_CPP_RELAXED_LOAD, ANNOTATION_CPP_RELAXED_STORE, ANNOTATION_CPP_RELEASE_STORE,
    ANNOTATION_CPP_SCOPE, ANNOTATION_CUSTOM_CPP_CLASS, ANNOTATION_CUSTOM_INTERFACE_DESCRIPTOR,
    ANNOTATION_CUSTOM_MAP, ANNOTATION_CUSTOM_WEAK_MARKING, ANNOTATION_DO_NOT_GENERATE_CAST,
    ANNOTATION_DO_NOT_GENERATE_CPP_CLASS, ANNOTATION_DO_NOT_GENERATE_INSTANCE_TYPE_CHECK,
    ANNOTATION_EXPORT, ANNOTATION_HAS_SAME_INSTANCE_TYPE_AS_PARENT,
    ANNOTATION_HIGHEST_INSTANCE_TYPE_WITHIN_PARENT, ANNOTATION_IF, ANNOTATION_IFNOT,
    ANNOTATION_INCREMENT_USE_COUNTER, ANNOTATION_INSTANCE_TYPE_VALUE,
    ANNOTATION_LOWEST_INSTANCE_TYPE_WITHIN_PARENT, ANNOTATION_RESERVE_BITS_IN_INSTANCE_TYPE,
    ANNOTATION_SAME_ENUM_VALUE_AS, ANNOTATION_SUPPORTS_TSA, ANNOTATION_USE_PARENT_TYPE_CHECKER,
    AbstractTypeFlag, CONST_REFERENCE_TYPE_STRING, CONSTEXPR_TYPE_PREFIX, ClassFlag,
    FieldSynchronization, MUTABLE_REFERENCE_TYPE_STRING, StructFlag,
    TORQUE_INTERNAL_NAMESPACE_STRING, get_constexpr_name,
  },
  context::{
    SourcePositionScope, build_flag, current_source_file, current_source_position, error_continue,
    file_relative_to_v8_root_exists, get_source_id, lint, lint_at, replace_ast, report_error,
    with_ast,
  },
  declarations::{CATCH_LABEL_NAME, NEXT_CASE_LABEL_NAME},
  earley_parser::{
    Action, Grammar, ParseResultIterator, ParseResultValue, SymbolId, default_action,
    yield_matched_input,
  },
  error::{Error, Result},
  source_positions::SourcePosition,
  utils::{
    is_lower_camel_case, is_snake_case, is_upper_camel_case, is_valid_namespace_const_name,
    is_valid_type_name, string_literal_unquote,
  },
};

// 原版 ParseTorque 经 CurrentAst 上下文添加声明 → Rust 显式传 &mut Ast (见 parse_torque)。
// 原版 TorqueGrammar ~200 个 Symbol 成员 → Rust 组合 earley_parser::Grammar,
// 符号随文法构造动态登记 (grammar.tables), token 关键字全量见 GRAMMAR_TOKENS。
/// 原版 using TypeList = std::vector<TypeExpression*>
pub type TypeList = Vec<NodeId>;

/// 原版 ExpressionWithSource 1:1
#[derive(Debug, Clone)]
pub struct ExpressionWithSource {
  pub expression: NodeId,
  pub source: String,
}

/// 原版 EnumEntry 1:1
#[derive(Debug, Clone)]
pub struct EnumEntry {
  pub name: NodeId,
  pub r#type: Option<NodeId>,
  pub alias_entry: Option<String>,
}

/// 文法关键字/操作符 token 全量 (68 个, 抽自 torque-parser.cc Token("...") 首现序)
pub const GRAMMAR_TOKENS: [&str; 68] = [
  "runtime",
  "(",
  ")",
  "::",
  ",",
  "constexpr",
  "builtin",
  "=>",
  "const",
  "&",
  "|",
  ":",
  "type",
  "extends",
  "<",
  ">",
  "...",
  "labels",
  "otherwise",
  "[",
  "]",
  "weak",
  "?",
  ";",
  "bit",
  "++",
  "--",
  ".",
  "{",
  "}",
  "new",
  "Pretenured",
  "ClearPadding",
  "->",
  "*",
  "<<",
  "&&",
  "||",
  "=",
  "deferred",
  "label",
  "catch",
  "return",
  "tail",
  "break",
  "continue",
  "goto",
  "if",
  "else",
  "typeswitch",
  "try",
  "while",
  "for",
  "case",
  "transitioning",
  "operator",
  "macro",
  "generates",
  "extern",
  "transient",
  "struct",
  "bitfield",
  "intrinsic",
  "javascript",
  "#include",
  "enum",
  "namespace",
  "import",
];

// ── 节点构造基建 (原版 MakeNode 经 CurrentAst/CurrentSourcePosition) ──

/// 指定源位置的节点构造
fn make_node_at(node: AstNode, pos: SourcePosition) -> NodeId {
  let kind = node.kind();
  with_ast(|ast| ast.add_node(NodeHeader { kind, pos }, node))
}

/// 原版 MakeNode: pos = 当前源位置
fn make_node(node: AstNode) -> NodeId {
  let pos = current_source_position();
  make_node_at(node, pos)
}

fn node_kind(id: NodeId) -> AstNodeKind {
  with_ast(|ast| ast.node(id).0.kind)
}

fn node_pos(id: NodeId) -> SourcePosition {
  with_ast(|ast| ast.node(id).0.pos)
}

/// Identifier 节点取值
fn identifier_value(id: NodeId) -> String {
  with_ast(|ast| match &ast.node(id).1 {
    AstNode::Identifier(i) => i.value.clone(),
    _ => String::new(),
  })
}

/// 下一子结果的强类型抽取 (类型不符 → Err 通道, 对应原版 Cast CHECK)
macro_rules! next {
  ($it:expr, $variant:ident) => {
    match $it.next_result() {
      Some(ParseResultValue::$variant(v)) => Ok(v),
      Some(other) => Err(Error::Message(format!("子结果类型不符: {other:?}"))),
      None => Err(Error::Message("子结果耗尽".into())),
    }
  };
}

// ── 命名规范 lint (torque-parser.cc:321-341) ──

/// 原版 NamingConventionError
fn naming_convention_error(type_: &str, name: &str, convention: &str, pos: SourcePosition) {
  lint_at(
    format!("{type_} \"{name}\" does not follow \"{convention}\" naming convention."),
    pos,
  );
}

/// 原版 LintGenericParameters
fn lint_generic_parameters(parameters: &[GenericParameter]) {
  for parameter in parameters {
    let value = identifier_value(parameter.name);
    if !is_upper_camel_case(&value) {
      let pos = node_pos(parameter.name);
      naming_convention_error("Generic parameter", &value, "UpperCamelCase", pos);
    }
  }
}

/// 原版 CheckNotDeferredStatement
fn check_not_deferred_statement(statement: NodeId) {
  let deferred = with_ast(|ast| match &ast.node(statement).1 {
    AstNode::BlockStatement(b) => b.deferred,
    _ => false,
  });
  if deferred {
    let _scope = SourcePositionScope::new(node_pos(statement));
    lint("cannot use deferred with a statement block here, it will have no effect".to_owned());
  }
}

/// 原版 AddConstexpr: extends 子句折成 "constexpr X" 基本类型
fn add_constexpr(type_: NodeId) -> Result<NodeId> {
  let (ns, name, generics) = with_ast(|ast| match &ast.node(type_).1 {
    AstNode::BasicTypeExpression(b) => (
      b.namespace_qualification.clone(),
      b.name,
      b.generic_arguments.clone(),
    ),
    _ => (Vec::new(), NodeId::MAX, Vec::new()),
  });
  if name == NodeId::MAX {
    return Err(report_error("Unsupported extends clause.".to_owned()));
  }
  let constexpr_name = make_node_at(
    AstNode::Identifier(Box::new(Identifier {
      value: format!("{}{}", CONSTEXPR_TYPE_PREFIX, identifier_value(name)),
    })),
    node_pos(name),
  );
  Ok(make_node(AstNode::BasicTypeExpression(Box::new(
    BasicTypeExpression {
      namespace_qualification: ns,
      is_constexpr: false,
      name: constexpr_name,
      generic_arguments: generics,
    },
  ))))
}

// ── 词法 pattern (torque-parser.cc:2284-2426) ──

/// 两字节前瞻 (字节级 starts_with, 免切片重入)
#[inline]
fn bytes_at(b: &[u8], pos: usize, x1: u8, x2: u8) -> bool {
  b.len() >= pos + 2 && b[pos] == x1 && b[pos + 1] == x2
}

/// 原版 MatchWhitespace: 空白 + // 行注释 + /* 块注释 */
fn match_whitespace(input: &str, pos: &mut usize) -> bool {
  let b = input.as_bytes();
  loop {
    if *pos < b.len() && b[*pos].is_ascii_whitespace() {
      *pos += 1;
      continue;
    }
    if bytes_at(b, *pos, b'/', b'/') {
      *pos += 2;
      while *pos < b.len() && b[*pos] != b'\n' {
        *pos += 1;
      }
      continue;
    }
    if bytes_at(b, *pos, b'/', b'*') {
      *pos += 2;
      while *pos < b.len() && !bytes_at(b, *pos, b'*', b'/') {
        *pos += 1;
      }
      // 未闭合块注释吞到串尾 (原版越界行为的收敛化)
      if bytes_at(b, *pos, b'*', b'/') {
        *pos += 2;
      }
      continue;
    }
    return true;
  }
}

/// 原版 MatchIdentifier: 可选单下划线 + 必字母 + 字母数字/下划线 (拒双下划线开头)
fn match_identifier(input: &str, pos: &mut usize) -> bool {
  let b = input.as_bytes();
  let mut cur = *pos;
  if cur < b.len() && b[cur] == b'_' {
    cur += 1;
  }
  if cur >= b.len() || !b[cur].is_ascii_alphabetic() {
    return false;
  }
  while cur < b.len() && (b[cur].is_ascii_alphanumeric() || b[cur] == b'_') {
    cur += 1;
  }
  *pos = cur;
  true
}

/// 原版 MatchAnnotation: '@' + identifier
fn match_annotation(input: &str, pos: &mut usize) -> bool {
  let b = input.as_bytes();
  let mut cur = *pos;
  if cur >= b.len() || b[cur] != b'@' {
    return false;
  }
  cur += 1;
  if !match_identifier(input, &mut cur) {
    return false;
  }
  *pos = cur;
  true
}

/// 原版 MatchIntrinsicName: '%' + identifier
fn match_intrinsic_name(input: &str, pos: &mut usize) -> bool {
  let b = input.as_bytes();
  let mut cur = *pos;
  if cur >= b.len() || b[cur] != b'%' {
    return false;
  }
  cur += 1;
  if !match_identifier(input, &mut cur) {
    return false;
  }
  *pos = cur;
  true
}

/// 原版 MatchStringLiteral: "..." / '...' (反斜杠转义任意字符)
fn match_string_literal(input: &str, pos: &mut usize) -> bool {
  let b = input.as_bytes();
  for quote in *b"\"'" {
    let mut cur = *pos;
    if cur >= b.len() || b[cur] != quote {
      continue;
    }
    cur += 1;
    while cur < b.len() {
      if b[cur] == b'\\' && cur + 1 < b.len() {
        cur += 2;
        continue;
      }
      if b[cur] == quote || b[cur] == b'\n' {
        break;
      }
      cur += 1;
    }
    if cur < b.len() && b[cur] == quote {
      *pos = cur + 1;
      return true;
    }
  }
  false
}

/// 原版 MatchHexLiteral: '-'? 0x 十六进制+
fn match_hex_literal(input: &str, pos: &mut usize) -> bool {
  let b = input.as_bytes();
  let mut cur = *pos;
  if cur < b.len() && b[cur] == b'-' {
    cur += 1;
  }
  if !bytes_at(b, cur, b'0', b'x') {
    return false;
  }
  cur += 2;
  if cur >= b.len() || !b[cur].is_ascii_hexdigit() {
    return false;
  }
  while cur < b.len() && b[cur].is_ascii_hexdigit() {
    cur += 1;
  }
  *pos = cur;
  true
}

/// 原版 MatchIntegerLiteral: '-'? 十进制+
fn match_integer_literal(input: &str, pos: &mut usize) -> bool {
  let b = input.as_bytes();
  let mut cur = *pos;
  let mut found_digit = false;
  if cur < b.len() && b[cur] == b'-' {
    cur += 1;
  }
  while cur < b.len() && b[cur].is_ascii_digit() {
    found_digit = true;
    cur += 1;
  }
  if found_digit {
    *pos = cur;
    true
  } else {
    false
  }
}

/// 原版 MatchFloatingPointLiteral: '-'? 数字+ '.' 数字+ (可选指数)
fn match_floating_point_literal(input: &str, pos: &mut usize) -> bool {
  let b = input.as_bytes();
  let mut cur = *pos;
  let mut found_digit = false;
  if cur < b.len() && b[cur] == b'-' {
    cur += 1;
  }
  while cur < b.len() && b[cur].is_ascii_digit() {
    found_digit = true;
    cur += 1;
  }
  if cur >= b.len() || b[cur] != b'.' {
    return false;
  }
  cur += 1;
  while cur < b.len() && b[cur].is_ascii_digit() {
    found_digit = true;
    cur += 1;
  }
  if !found_digit {
    return false;
  }
  if cur < b.len() && (b[cur] == b'e' || b[cur] == b'E') {
    let mut exp = cur + 1;
    if exp < b.len() && (b[exp] == b'+' || b[exp] == b'-') {
      exp += 1;
    }
    if exp < b.len() && b[exp].is_ascii_digit() {
      while exp < b.len() && b[exp].is_ascii_digit() {
        exp += 1;
      }
      cur = exp;
    }
  }
  *pos = cur;
  true
}

// ── 字面量动作 (torque-parser.cc:903-976) ──

/// 剥负号后的无符号绝对值解析 (十进制 / 0x 十六进制)
fn parse_unsigned_abs(value: &str) -> Option<u64> {
  let unsigned = value.strip_prefix('-').unwrap_or(value);
  if let Some(hex) = unsigned
    .strip_prefix("0x")
    .or_else(|| unsigned.strip_prefix("0X"))
  {
    u64::from_str_radix(hex, 16).ok()
  } else {
    unsigned.parse::<u64>().ok()
  }
}

/// 原版 YieldInt32: 十进制/0x 十六进制 → i32
fn yield_int32(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let value = child_results.matched_text();
  match parse_unsigned_abs(value).map(i32::try_from) {
    Some(Ok(v)) => Ok(Some(ParseResultValue::Int32(v))),
    Some(Err(_)) => Err(report_error("Integer out of 32-bit range".to_owned())),
    None => Err(report_error("Expected an integer".to_owned())),
  }
}

/// 原版 YieldDouble
fn yield_double(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  match child_results.matched_text().parse::<f64>() {
    Ok(v) => Ok(Some(ParseResultValue::Double(v))),
    Err(_) => Err(report_error("double literal out-of-range".to_owned())),
  }
}

/// 原版 YieldIntegerLiteral: 符号 + 无符号绝对值 (十进制/0x)
fn yield_integer_literal(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let value = child_results.matched_text();
  let negative = value.starts_with('-');
  let Some(absolute) = parse_unsigned_abs(value) else {
    return Err(report_error(
      "integer literal could not be parsed".to_owned(),
    ));
  };
  Ok(Some(ParseResultValue::IntegerLiteral(IntegerLiteral {
    negative,
    digits: absolute.to_string(),
  })))
}

// ── 泛型组合动作族 (原模板 TryOrDefault/Optional/NonemptyList(List) 具现化) ──

/// Vec 载荷动作族: 单元素成表 / 追加 / 空表 ($elem = 元素变体, $list = 列表变体)
macro_rules! vec_actions {
  ($sing:ident, $extend:ident, $empty:ident, $elem:ident, $list:ident) => {
    fn $sing(it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
      let x = next!(it, $elem)?;
      Ok(Some(ParseResultValue::$list(vec![x])))
    }
    fn $extend(it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
      let mut l = next!(it, $list)?;
      let x = next!(it, $elem)?;
      l.push(x);
      Ok(Some(ParseResultValue::$list(l)))
    }
    fn $empty(_it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
      Ok(Some(ParseResultValue::$list(Vec::new())))
    }
  };
}

/// 仅空表动作 (if 注解列表的 TryOrDefault 空臂)
macro_rules! vec_empty {
  ($empty:ident, $elem:ident, $list:ident) => {
    fn $empty(_it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
      Ok(Some(ParseResultValue::$list(Vec::new())))
    }
  };
}

/// if 注解列表动作族 (@if/@ifnot 过滤后入表)
macro_rules! vec_if_actions {
  ($if_first:ident, $if_extend:ident, $elem:ident, $list:ident) => {
    fn $if_first(it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
      let annotations = next!(it, VectorOfAnnotation)?;
      let x = next!(it, $elem)?;
      let mut l = Vec::new();
      if process_if_annotation_raw(&annotations)? {
        l.push(x);
      }
      Ok(Some(ParseResultValue::$list(l)))
    }
    fn $if_extend(it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
      let mut l = next!(it, $list)?;
      let annotations = next!(it, VectorOfAnnotation)?;
      let x = next!(it, $elem)?;
      if process_if_annotation_raw(&annotations)? {
        l.push(x);
      }
      Ok(Some(ParseResultValue::$list(l)))
    }
  };
}

/// Option 载荷动作族: Some 包装 / None 产出
macro_rules! opt_actions {
  ($wrap:ident, $none:ident, $inner:ident, $opt:ident) => {
    fn $wrap(it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
      let x = next!(it, $inner)?;
      Ok(Some(ParseResultValue::$opt(Some(x))))
    }
    fn $none(_it: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
      Ok(Some(ParseResultValue::$opt(None)))
    }
  };
}

vec_actions!(
  singleton_annotation,
  extend_annotation,
  empty_annotation,
  Annotation,
  VectorOfAnnotation
);
vec_actions!(
  singleton_type_list,
  extend_type_list,
  empty_type_list,
  TypeExpressionPtr,
  TypeList
);
vec_actions!(
  singleton_statement,
  extend_statement,
  empty_statement,
  StatementPtr,
  StdVectorOfStatementPtr
);
vec_actions!(
  singleton_declaration,
  extend_declaration,
  empty_declaration,
  DeclarationPtr,
  StdVectorOfDeclarationPtr
);
vec_actions!(
  singleton_declaration_list,
  extend_declaration_list,
  empty_declaration_list,
  StdVectorOfDeclarationPtr,
  StdVectorOfStdVectorOfDeclarationPtr
);
vec_actions!(
  singleton_expression,
  extend_expression,
  empty_expression,
  ExpressionPtr,
  StdVectorOfExpressionPtr
);
vec_actions!(
  singleton_name_and_type,
  extend_name_and_type,
  empty_name_and_type,
  NameAndTypeExpression,
  StdVectorOfNameAndTypeExpression
);
vec_actions!(
  singleton_name_and_expression,
  extend_name_and_expression,
  empty_name_and_expression,
  NameAndExpression,
  StdVectorOfNameAndExpression
);
vec_actions!(
  singleton_class_field,
  extend_class_field,
  empty_class_field,
  ClassFieldExpression,
  StdVectorOfClassFieldExpression
);
vec_actions!(
  singleton_label_and_types,
  extend_label_and_types,
  empty_label_and_types,
  LabelAndTypes,
  StdVectorOfLabelAndTypes
);
vec_actions!(
  singleton_try_handler,
  extend_try_handler,
  empty_try_handler,
  TryHandlerPtr,
  StdVectorOfTryHandlerPtr
);
vec_actions!(
  singleton_string,
  extend_string,
  empty_string,
  StdString,
  StdVectorOfString
);
vec_actions!(
  singleton_generic_params,
  extend_generic_params,
  empty_generic_params,
  GenericParameter,
  GenericParameters
);

vec_if_actions!(
  first_if_statement,
  extend_if_statement,
  StatementPtr,
  StdVectorOfStatementPtr
);
vec_if_actions!(
  first_if_declaration,
  extend_if_declaration,
  DeclarationPtr,
  StdVectorOfDeclarationPtr
);
vec_empty!(
  empty_struct_field,
  StructFieldExpression,
  StdVectorOfStructFieldExpression
);
vec_if_actions!(
  first_if_struct_field,
  extend_if_struct_field,
  StructFieldExpression,
  StdVectorOfStructFieldExpression
);
vec_empty!(
  empty_bit_field,
  BitFieldDeclaration,
  StdVectorOfBitFieldDeclaration
);
vec_if_actions!(
  first_if_bit_field,
  extend_if_bit_field,
  BitFieldDeclaration,
  StdVectorOfBitFieldDeclaration
);
vec_if_actions!(
  first_if_enum_entry,
  extend_if_enum_entry,
  EnumEntry,
  StdVectorOfEnumEntry
);
vec_if_actions!(
  first_if_typeswitch_case,
  extend_if_typeswitch_case,
  TypeswitchCase,
  StdVectorOfTypeswitchCase
);

opt_actions!(
  wrap_some_annotation_parameter,
  none_annotation_parameter,
  AnnotationParameter,
  OptionalAnnotationParameter
);
opt_actions!(
  wrap_some_identifier,
  none_identifier,
  IdentifierPtr,
  OptionalIdentifierPtr
);
opt_actions!(
  wrap_some_type_expression,
  none_type_expression,
  TypeExpressionPtr,
  OptionalTypeExpressionPtr
);
opt_actions!(
  wrap_some_statement,
  none_statement,
  StatementPtr,
  OptionalStatementPtr
);
opt_actions!(
  wrap_some_expression,
  none_expression,
  ExpressionPtr,
  OptionalExpressionPtr
);
opt_actions!(
  wrap_some_implicit_parameters,
  none_implicit_parameters,
  ImplicitParameters,
  OptionalImplicitParameters
);
opt_actions!(wrap_some_string, none_string, StdString, OptionalStdString);

// ── AnnotationSet (torque-parser.cc:673-739) ──

/// 原版 AnnotationSet: 白名单校验 + 重复检测 + 参数查询
struct AnnotationSet {
  set: BTreeSet<String>,
  map: BTreeMap<String, (AnnotationParameter, SourcePosition)>,
}

impl AnnotationSet {
  fn new(
    it: &mut ParseResultIterator,
    allowed_without_param: &[&str],
    allowed_with_param: &[&str],
  ) -> Result<Self> {
    let mut result = Self {
      set: BTreeSet::new(),
      map: BTreeMap::new(),
    };
    for a in next!(it, VectorOfAnnotation)? {
      let name = identifier_value(a.name);
      let name_pos = node_pos(a.name);
      match &a.param {
        Some(param) => {
          if !allowed_with_param.contains(&name.as_str()) {
            let error_message = if !allowed_without_param.contains(&name.as_str()) {
              " is not allowed here"
            } else {
              " cannot have parameter here"
            };
            lint_at(format!("Annotation {name}{error_message}"), name_pos);
          }
          if result
            .map
            .insert(name.clone(), (param.clone(), name_pos))
            .is_some()
          {
            lint_at(format!("Duplicate annotation {name}"), name_pos);
          }
        }
        None => {
          if !allowed_without_param.contains(&name.as_str()) {
            let error_message = if allowed_with_param.contains(&name.as_str()) {
              " requires a parameter here"
            } else {
              " is not allowed here"
            };
            lint_at(format!("Annotation {name}{error_message}"), name_pos);
          }
          if !result.set.insert(name.clone()) {
            lint_at(format!("Duplicate annotation {name}"), name_pos);
          }
        }
      }
    }
    Ok(result)
  }

  fn contains(&self, s: &str) -> bool {
    self.set.contains(s)
  }

  fn get_string_param(&self, s: &str) -> Result<Option<String>> {
    match self.map.get(s) {
      None => Ok(None),
      Some((param, pos)) => {
        if param.is_int {
          let _scope = SourcePositionScope::new(*pos);
          return Err(report_error(format!(
            "Annotation {s} requires a string parameter but has an int"
          )));
        }
        Ok(Some(param.string_value.clone()))
      }
    }
  }

  fn get_int_param(&self, s: &str) -> Result<Option<i32>> {
    match self.map.get(s) {
      None => Ok(None),
      Some((param, pos)) => {
        if !param.is_int {
          let _scope = SourcePositionScope::new(*pos);
          return Err(report_error(format!(
            "Annotation {s} requires an int parameter but has a string"
          )));
        }
        Ok(Some(param.int_value))
      }
    }
  }
}

/// 原版 ProcessIfAnnotation(const AnnotationSet&): @if/@ifnot 条件求值
fn process_if_annotation_set(annotations: &AnnotationSet) -> Result<bool> {
  if let Some(condition) = annotations.get_string_param(ANNOTATION_IF)?
    && !build_flag(&condition, ANNOTATION_IF)?
  {
    return Ok(false);
  }
  if let Some(condition) = annotations.get_string_param(ANNOTATION_IFNOT)?
    && build_flag(&condition, ANNOTATION_IFNOT)?
  {
    return Ok(false);
  }
  Ok(true)
}

/// 原版 ProcessIfAnnotation(ParseResultIterator*): 无白名单直取版
fn process_if_annotation_raw(annotations: &[Annotation]) -> Result<bool> {
  let mut set = AnnotationSet {
    set: BTreeSet::new(),
    map: BTreeMap::new(),
  };
  for a in annotations {
    let name = identifier_value(a.name);
    match &a.param {
      Some(param) => {
        set.map.insert(name, (param.clone(), node_pos(a.name)));
      }
      None => {
        set.set.insert(name);
      }
    }
  }
  process_if_annotation_set(&set)
}

// ── 顶层与调用链动作 ──

/// 原版 AddGlobalDeclarations
fn add_global_declarations(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let declarations = next!(child_results, StdVectorOfDeclarationPtr)?;
  with_ast(|ast| ast.declarations.extend(declarations));
  Ok(None)
}

/// 原版 ConcatList
fn concat_list(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let list_of_lists = next!(child_results, StdVectorOfStdVectorOfDeclarationPtr)?;
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(
    list_of_lists.into_iter().flatten().collect(),
  )))
}

/// otherwise 语句表 → label 列表 + 临时 TryHandler 列表
/// (原版 MakeCall 内联逻辑, torque-parser.cc:382-405)
fn collect_call_labels(otherwise: &[NodeId]) -> (Vec<NodeId>, Vec<NodeId>) {
  let mut labels = Vec::new();
  let mut temp_labels = Vec::new();
  for &statement in otherwise {
    let plain_label = with_ast(|ast| match &ast.node(statement).1 {
      AstNode::ExpressionStatement(e) => match &ast.node(e.expression).1 {
        AstNode::IdentifierExpression(id) if id.generic_arguments.is_empty() => Some(id.name),
        _ => None,
      },
      _ => None,
    });
    if let Some(id) = plain_label {
      labels.push(id);
      continue;
    }
    let label_name = format!("__label{}", temp_labels.len());
    let label_id = make_node_at(
      AstNode::Identifier(Box::new(Identifier { value: label_name })),
      SourcePosition::invalid(),
    );
    labels.push(label_id);
    temp_labels.push(make_node(AstNode::TryHandler(Box::new(TryHandler {
      handler_kind: HandlerKind::Label,
      label: label_id,
      parameters: ParameterList::default(),
      body: statement,
    }))));
  }
  (labels, temp_labels)
}

/// 原版 MakeCall(Identifier*, generics, args, otherwise) 四参重载
/// (torque-parser.cc:421-427)
fn make_call_from_identifier(
  callee: NodeId,
  generic_arguments: &[NodeId],
  arguments: &[NodeId],
  otherwise: &[NodeId],
) -> NodeId {
  let callee = make_node(AstNode::IdentifierExpression(Box::new(
    IdentifierExpression {
      namespace_qualification: Vec::new(),
      name: callee,
      generic_arguments: generic_arguments.to_vec(),
    },
  )));
  make_call(callee, &[], arguments, otherwise)
}

/// 原版 MakeCall 三参重载核心 (torque-parser.cc:373-420)
pub fn make_call(
  callee: NodeId,
  _generic_arguments: &[NodeId],
  arguments: &[NodeId],
  otherwise: &[NodeId],
) -> NodeId {
  let (labels, temp_labels) = collect_call_labels(otherwise);
  let mut result = make_node(AstNode::CallExpression(Box::new(CallExpression {
    callee,
    arguments: arguments.to_vec(),
    labels,
  })));
  for label in temp_labels {
    result = make_node(AstNode::TryLabelExpression(Box::new(TryLabelExpression {
      try_expression: result,
      label_block: label,
    })));
  }
  result
}

/// 原版 MakeCall(ParseResultIterator*) 规则动作
fn make_call_action(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let callee = next!(child_results, ExpressionPtr)?;
  let args = next!(child_results, StdVectorOfExpressionPtr)?;
  let otherwise = next!(child_results, StdVectorOfStatementPtr)?;
  if node_kind(callee) != AstNodeKind::IdentifierExpression {
    return Err(report_error(
      "call callee must be an identifier expression".to_owned(),
    ));
  }
  Ok(Some(ParseResultValue::ExpressionPtr(make_call(
    callee,
    &[],
    &args,
    &otherwise,
  ))))
}

/// 原版 MakeMethodCall
fn make_method_call(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let this_arg = next!(child_results, ExpressionPtr)?;
  let callee = next!(child_results, IdentifierPtr)?;
  let args = next!(child_results, StdVectorOfExpressionPtr)?;
  let otherwise = next!(child_results, StdVectorOfStatementPtr)?;
  let callee = make_node(AstNode::IdentifierExpression(Box::new(
    IdentifierExpression {
      namespace_qualification: Vec::new(),
      name: callee,
      generic_arguments: Vec::new(),
    },
  )));
  let (labels, temp_labels) = collect_call_labels(&otherwise);
  let mut result = make_node(AstNode::CallMethodExpression(Box::new(
    CallMethodExpression {
      target: this_arg,
      method: callee,
      arguments: args,
      labels,
    },
  )));
  for label in temp_labels {
    result = make_node(AstNode::TryLabelExpression(Box::new(TryLabelExpression {
      try_expression: result,
      label_block: label,
    })));
  }
  Ok(Some(ParseResultValue::ExpressionPtr(result)))
}

/// 原版 MakeNewExpression
fn make_new_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let pretenured = next!(child_results, Bool)?;
  let clear_padding = next!(child_results, Bool)?;
  let type_ = next!(child_results, TypeExpressionPtr)?;
  let initializers = next!(child_results, StdVectorOfNameAndExpression)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::NewExpression(Box::new(NewExpression {
      r#type: type_,
      initializers,
      pretenured,
      clear_padding,
    })),
  ))))
}

/// 原版 MakeBinaryOperator: 双目运算符 → 运算符调用
fn make_binary_operator(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let left = next!(child_results, ExpressionPtr)?;
  let op = next!(child_results, IdentifierPtr)?;
  let right = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(
    make_call_from_identifier(op, &[], &[left, right], &[]),
  )))
}

/// 原版 MakeIntrinsicCallExpression
fn make_intrinsic_call_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let callee = next!(child_results, IdentifierPtr)?;
  let generic_arguments = next!(child_results, TypeList)?;
  let args = next!(child_results, StdVectorOfExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::IntrinsicCallExpression(Box::new(IntrinsicCallExpression {
      name: callee,
      generic_arguments,
      arguments: args,
    })),
  ))))
}

/// 原版 MakeUnaryOperator
fn make_unary_operator(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let op = next!(child_results, IdentifierPtr)?;
  let e = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(
    make_call_from_identifier(op, &[], &[e], &[]),
  )))
}

/// 原版 MakeSpreadExpression
fn make_spread_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let spreadee = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::SpreadExpression(Box::new(SpreadExpression { spreadee })),
  ))))
}

/// 原版 MakeImplicitParameterList
fn make_implicit_parameter_list(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let kind = next!(child_results, IdentifierPtr)?;
  let parameters = next!(child_results, StdVectorOfNameAndTypeExpression)?;
  Ok(Some(ParseResultValue::ImplicitParameters(
    ImplicitParameters { kind, parameters },
  )))
}

/// 原版 AddParameter: 参数命名 lint + 追加
fn add_parameter(parameter_list: &mut ParameterList, param: NameAndTypeExpression) {
  let value = identifier_value(param.name);
  if !is_lower_camel_case(&value) {
    let pos = node_pos(param.name);
    naming_convention_error("Parameter", &value, "lowerCamelCase", pos);
  }
  parameter_list.names.push(param.name);
  parameter_list.types.push(param.r#type);
}

/// 隐式参数段的公共展开 (四实例共享前半)
fn take_implicit_parameters(
  implicit_params: Option<ImplicitParameters>,
  result: &mut ParameterList,
) {
  let Some(implicit) = implicit_params else {
    return;
  };
  result.implicit_count = implicit.parameters.len();
  result.implicit_kind = if identifier_value(implicit.kind) == "implicit" {
    ImplicitKind::Implicit
  } else {
    ImplicitKind::JsImplicit
  };
  result.implicit_kind_pos = node_pos(implicit.kind);
  for implicit_param in implicit.parameters {
    add_parameter(result, implicit_param);
  }
}

/// 原版 MakeParameterList<true, false> (类型 + 变长)
fn make_parameter_list_varargs_types(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let implicit_params = next!(child_results, OptionalImplicitParameters)?;
  let mut result = ParameterList {
    has_varargs: true,
    ..ParameterList::default()
  };
  take_implicit_parameters(implicit_params, &mut result);
  for explicit_type in next!(child_results, TypeList)? {
    result.types.push(explicit_type);
  }
  Ok(Some(ParseResultValue::ParameterList(result)))
}

/// 原版 MakeParameterList<false, false> (纯类型)
fn make_parameter_list_types(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let implicit_params = next!(child_results, OptionalImplicitParameters)?;
  let mut result = ParameterList::default();
  take_implicit_parameters(implicit_params, &mut result);
  for explicit_type in next!(child_results, TypeList)? {
    result.types.push(explicit_type);
  }
  Ok(Some(ParseResultValue::ParameterList(result)))
}

/// 原版 MakeParameterList<false, true> (具名参数)
fn make_parameter_list_named(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let implicit_params = next!(child_results, OptionalImplicitParameters)?;
  let mut result = ParameterList::default();
  take_implicit_parameters(implicit_params, &mut result);
  for param in next!(child_results, StdVectorOfNameAndTypeExpression)? {
    add_parameter(&mut result, param);
  }
  Ok(Some(ParseResultValue::ParameterList(result)))
}

/// 原版 MakeParameterList<true, true> (具名参数 + 变长)
fn make_parameter_list_named_varargs(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let implicit_params = next!(child_results, OptionalImplicitParameters)?;
  let mut result = ParameterList {
    has_varargs: true,
    ..ParameterList::default()
  };
  take_implicit_parameters(implicit_params, &mut result);
  for param in next!(child_results, StdVectorOfNameAndTypeExpression)? {
    add_parameter(&mut result, param);
  }
  result.arguments_variable = next!(child_results, StdString)?;
  Ok(Some(ParseResultValue::ParameterList(result)))
}

/// 原版 MakeAssertStatement
fn make_assert_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let kind_string = identifier_value(next!(child_results, IdentifierPtr)?);
  let expr_with_source = next!(child_results, ExpressionWithSource)?;
  let kind = match kind_string.as_str() {
    "dcheck" => AssertKind::Dcheck,
    "check" => AssertKind::Check,
    "sbxcheck" => {
      if build_flag("V8_ENABLE_SANDBOX", "sbxcheck")? {
        AssertKind::SbxCheck
      } else {
        AssertKind::Dcheck
      }
    }
    "static_assert" => AssertKind::StaticAssert,
    _ => return Err(report_error(format!("unknown assert kind {kind_string}"))),
  };
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::AssertStatement(Box::new(AssertStatement {
      kind,
      expression: expr_with_source.expression,
      source: expr_with_source.source,
    })),
  ))))
}

/// 原版 MakeDebugStatement
fn make_debug_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let kind = identifier_value(next!(child_results, IdentifierPtr)?);
  let kind = if kind == "unreachable" {
    DebugKind::Unreachable
  } else {
    DebugKind::Debug
  };
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::DebugStatement(Box::new(DebugStatement { kind })),
  ))))
}

/// 原版 DeprecatedMakeVoidType: 记错误 (不中止) + void 基本类型
fn deprecated_make_void_type(
  _child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  error_continue("Default void return types are deprecated. Add `: void`.".to_owned());
  let type_ = basic_type("void".to_owned(), Vec::new());
  Ok(Some(ParseResultValue::TypeExpressionPtr(type_)))
}

/// 原版 MakeExternalMacro
fn make_external_macro(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let enabled = process_if_annotation_raw(&next!(child_results, VectorOfAnnotation)?)?;
  let transitioning = next!(child_results, Bool)?;
  let operator_name = next!(child_results, OptionalStdString)?;
  let external_assembler_name = next!(child_results, OptionalStdString)?;
  let name = next!(child_results, IdentifierPtr)?;
  let generic_parameters = next!(child_results, GenericParameters)?;
  lint_generic_parameters(&generic_parameters);
  let args = next!(child_results, ParameterList)?;
  let return_type = next!(child_results, TypeExpressionPtr)?;
  let labels = next!(child_results, StdVectorOfLabelAndTypes)?;
  let mut result = Vec::new();
  if enabled {
    result.push(make_node(AstNode::ExternalMacroDeclaration(Box::new(
      ExternalMacroDeclaration {
        transitioning,
        name,
        parameters: args,
        return_type,
        labels,
        op: operator_name,
        external_assembler_name: external_assembler_name
          .unwrap_or_else(|| "CodeStubAssembler".to_owned()),
      },
    ))));
  }
  if !generic_parameters.is_empty() {
    return Err(report_error(
      "External builtins cannot be generic.".to_owned(),
    ));
  }
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(result)))
}

/// 原版 MakeIntrinsicDeclaration
fn make_intrinsic_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, IdentifierPtr)?;
  let generic_parameters = next!(child_results, GenericParameters)?;
  lint_generic_parameters(&generic_parameters);
  let args = next!(child_results, ParameterList)?;
  let return_type = next!(child_results, TypeExpressionPtr)?;
  let body = next!(child_results, OptionalStatementPtr)?;
  let declaration = match body {
    Some(body) => make_node(AstNode::TorqueMacroDeclaration(Box::new(
      TorqueMacroDeclaration {
        transitioning: false,
        name,
        parameters: args,
        return_type,
        labels: Vec::new(),
        op: None,
        export_to_csa: false,
        supports_tsa: false,
        body: Some(body),
      },
    ))),
    None => make_node(AstNode::IntrinsicDeclaration(Box::new(
      IntrinsicDeclaration {
        transitioning: false,
        name,
        parameters: args,
        return_type,
        labels: Vec::new(),
      },
    ))),
  };
  let result = if generic_parameters.is_empty() {
    declaration
  } else {
    make_node(AstNode::GenericCallableDeclaration(Box::new(
      GenericCallableDeclaration {
        generic_parameters,
        declaration,
      },
    )))
  };
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(vec![
    result,
  ])))
}

/// 原版 HasAnnotation: 单一注解校验
fn has_annotation(
  child_results: &mut ParseResultIterator,
  annotation: &str,
  declaration: &str,
) -> Result<bool> {
  let annotations = next!(child_results, VectorOfAnnotation)?;
  if !annotations.is_empty() {
    if annotations.len() > 1 || identifier_value(annotations[0].name) != annotation {
      return Err(report_error(format!(
        "{declaration} declarations only support a single {annotation} annotation"
      )));
    }
    return Ok(true);
  }
  Ok(false)
}

/// 原版 MakeTorqueMacroDeclaration
fn make_torque_macro_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let annotations = AnnotationSet::new(
    child_results,
    &[ANNOTATION_EXPORT, ANNOTATION_SUPPORTS_TSA],
    &[ANNOTATION_IF, ANNOTATION_IFNOT],
  )?;
  let enabled = process_if_annotation_set(&annotations)?;
  let export_to_csa = annotations.contains(ANNOTATION_EXPORT);
  let supports_tsa = annotations.contains(ANNOTATION_SUPPORTS_TSA);
  let transitioning = next!(child_results, Bool)?;
  let operator_name = next!(child_results, OptionalStdString)?;
  let name = next!(child_results, IdentifierPtr)?;
  let name_value = identifier_value(name);
  if !is_upper_camel_case(&name_value) {
    let pos = node_pos(name);
    naming_convention_error("Macro", &name_value, "UpperCamelCase", pos);
  }
  let generic_parameters = next!(child_results, GenericParameters)?;
  lint_generic_parameters(&generic_parameters);
  let args = next!(child_results, ParameterList)?;
  let return_type = next!(child_results, TypeExpressionPtr)?;
  let labels = next!(child_results, StdVectorOfLabelAndTypes)?;
  let body = next!(child_results, OptionalStatementPtr)?;
  let mut result = Vec::new();
  if enabled {
    let declaration = make_node(AstNode::TorqueMacroDeclaration(Box::new(
      TorqueMacroDeclaration {
        transitioning,
        name,
        parameters: args,
        return_type,
        labels,
        op: operator_name,
        export_to_csa,
        supports_tsa,
        body,
      },
    )));
    if generic_parameters.is_empty() {
      if body.is_none() {
        return Err(report_error(
          "A non-generic declaration needs a body.".to_owned(),
        ));
      }
      result.push(declaration);
    } else {
      if export_to_csa {
        return Err(report_error("Cannot export generics to CSA.".to_owned()));
      }
      result.push(make_node(AstNode::GenericCallableDeclaration(Box::new(
        GenericCallableDeclaration {
          generic_parameters,
          declaration,
        },
      ))));
    }
  }
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(result)))
}

/// 原版 MakeConstDeclaration
fn make_const_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let enabled = process_if_annotation_raw(&next!(child_results, VectorOfAnnotation)?)?;
  let name = next!(child_results, IdentifierPtr)?;
  let name_value = identifier_value(name);
  if !is_valid_namespace_const_name(&name_value) {
    let pos = node_pos(name);
    naming_convention_error("Constant", &name_value, "kUpperCamelCase", pos);
  }
  let type_ = next!(child_results, TypeExpressionPtr)?;
  let expression = next!(child_results, ExpressionPtr)?;
  let mut result = Vec::new();
  if enabled {
    result.push(make_node(AstNode::ConstDeclaration(Box::new(
      ConstDeclaration {
        name,
        r#type: type_,
        expression,
      },
    ))));
  }
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(result)))
}

/// 原版 MakeExternConstDeclaration
fn make_extern_const_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, IdentifierPtr)?;
  let type_ = next!(child_results, TypeExpressionPtr)?;
  let literal = next!(child_results, StdString)?;
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(vec![
    make_node(AstNode::ExternConstDeclaration(Box::new(
      ExternConstDeclaration {
        name,
        r#type: type_,
        literal,
      },
    ))),
  ])))
}

/// 原版 MakeTypeAliasDeclaration
fn make_type_alias_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let enabled = process_if_annotation_raw(&next!(child_results, VectorOfAnnotation)?)?;
  let name = next!(child_results, IdentifierPtr)?;
  let type_ = next!(child_results, TypeExpressionPtr)?;
  let mut result = Vec::new();
  if enabled {
    result.push(make_node(AstNode::TypeAliasDeclaration(Box::new(
      TypeAliasDeclaration {
        name,
        r#type: type_,
      },
    ))));
  }
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(result)))
}

/// 原版 MakeAbstractTypeDeclaration (含 constexpr 伴生类型)
fn make_abstract_type_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let use_parent_type_checker = has_annotation(
    child_results,
    ANNOTATION_USE_PARENT_TYPE_CHECKER,
    "abstract type",
  )?;
  let transient = next!(child_results, Bool)?;
  let name = next!(child_results, IdentifierPtr)?;
  let name_value = identifier_value(name);
  if !is_valid_type_name(&name_value) {
    let pos = node_pos(name);
    naming_convention_error("Type", &name_value, "UpperCamelCase", pos);
  }
  let generic_parameters = next!(child_results, GenericParameters)?;
  let extends = next!(child_results, OptionalTypeExpressionPtr)?;
  let generates = next!(child_results, OptionalStdString)?;
  let mut flags = 0u32;
  if transient {
    flags |= AbstractTypeFlag::Transient as u32;
  }
  if use_parent_type_checker {
    flags |= AbstractTypeFlag::UseParentTypeChecker as u32;
  }
  let type_decl = make_node(AstNode::AbstractTypeDeclaration(Box::new(
    AbstractTypeDeclaration {
      name,
      flags,
      extends,
      generates,
    },
  )));
  let mut decl = type_decl;
  if !generic_parameters.is_empty() {
    decl = make_node(AstNode::GenericTypeDeclaration(Box::new(
      GenericTypeDeclaration {
        generic_parameters: generic_parameters.clone(),
        declaration: type_decl,
      },
    )));
  }
  let constexpr_generates = next!(child_results, OptionalStdString)?;
  let mut result = vec![decl];
  if let Some(constexpr_generates) = constexpr_generates {
    // constexpr 伴生: "constexpr X" extends constexpr(原 extends)
    let constexpr_name = make_node_at(
      AstNode::Identifier(Box::new(Identifier {
        value: format!("{}{name_value}", CONSTEXPR_TYPE_PREFIX),
      })),
      node_pos(name),
    );
    let constexpr_extends = match extends {
      Some(e) => Some(add_constexpr(e)?),
      None => None,
    };
    let constexpr_decl = make_node_at(
      AstNode::AbstractTypeDeclaration(Box::new(AbstractTypeDeclaration {
        name: constexpr_name,
        flags: flags | AbstractTypeFlag::Constexpr as u32,
        extends: constexpr_extends,
        generates: Some(constexpr_generates),
      })),
      node_pos(name),
    );
    let decl = if generic_parameters.is_empty() {
      constexpr_decl
    } else {
      make_node(AstNode::GenericTypeDeclaration(Box::new(
        GenericTypeDeclaration {
          generic_parameters,
          declaration: constexpr_decl,
        },
      )))
    };
    result.push(decl);
  }
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(result)))
}

/// 原版 MakeMethodDeclaration
fn make_method_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let transitioning = next!(child_results, Bool)?;
  let operator_name = next!(child_results, OptionalStdString)?;
  let name = next!(child_results, IdentifierPtr)?;
  let name_value = identifier_value(name);
  if !is_upper_camel_case(&name_value) {
    let pos = node_pos(name);
    naming_convention_error("Method", &name_value, "UpperCamelCase", pos);
  }
  let args = next!(child_results, ParameterList)?;
  let return_type = next!(child_results, TypeExpressionPtr)?;
  let labels = next!(child_results, StdVectorOfLabelAndTypes)?;
  let body = next!(child_results, StatementPtr)?;
  Ok(Some(ParseResultValue::DeclarationPtr(make_node(
    AstNode::TorqueMacroDeclaration(Box::new(TorqueMacroDeclaration {
      transitioning,
      name,
      parameters: args,
      return_type,
      labels,
      op: operator_name,
      export_to_csa: false,
      supports_tsa: false,
      body: Some(body),
    })),
  ))))
}

/// 原版 MakeClassBody
fn make_class_body(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let methods = next!(child_results, StdVectorOfDeclarationPtr)?;
  let fields = next!(child_results, StdVectorOfClassFieldExpression)?;
  Ok(Some(ParseResultValue::OptionalClassBody(Some(make_node(
    AstNode::ClassBody(Box::new(ClassBody { methods, fields })),
  )))))
}

/// 原版 MakeInstanceTypeConstraints
fn make_instance_type_constraints(annotations: &AnnotationSet) -> Result<InstanceTypeConstraints> {
  Ok(InstanceTypeConstraints {
    value: annotations
      .get_int_param(ANNOTATION_INSTANCE_TYPE_VALUE)?
      .unwrap_or(-1),
    num_flags_bits: annotations
      .get_int_param(ANNOTATION_RESERVE_BITS_IN_INSTANCE_TYPE)?
      .unwrap_or(-1),
  })
}

/// 原版 MakeClassDeclaration (类声明 + constexpr 伴生 + Cast 特化)
fn make_class_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let annotations = AnnotationSet::new(
    child_results,
    &[
      ANNOTATION_ABSTRACT,
      ANNOTATION_HAS_SAME_INSTANCE_TYPE_AS_PARENT,
      ANNOTATION_DO_NOT_GENERATE_CPP_CLASS,
      ANNOTATION_CUSTOM_CPP_CLASS,
      ANNOTATION_CUSTOM_MAP,
      ANNOTATION_EXPORT,
      ANNOTATION_DO_NOT_GENERATE_CAST,
      ANNOTATION_DO_NOT_GENERATE_INSTANCE_TYPE_CHECK,
      ANNOTATION_HIGHEST_INSTANCE_TYPE_WITHIN_PARENT,
      ANNOTATION_LOWEST_INSTANCE_TYPE_WITHIN_PARENT,
      ANNOTATION_CPP_OBJECT_LAYOUT_DEFINITION,
    ],
    &[
      ANNOTATION_RESERVE_BITS_IN_INSTANCE_TYPE,
      ANNOTATION_INSTANCE_TYPE_VALUE,
    ],
  )?;
  let mut flags = 0u32;
  if annotations.contains(ANNOTATION_ABSTRACT) {
    flags |= ClassFlag::Abstract as u32;
  }
  if annotations.contains(ANNOTATION_HAS_SAME_INSTANCE_TYPE_AS_PARENT) {
    flags |= ClassFlag::HasSameInstanceTypeAsParent as u32;
  }
  let do_not_generate_cpp_class = annotations.contains(ANNOTATION_DO_NOT_GENERATE_CPP_CLASS);
  if annotations.contains(ANNOTATION_CUSTOM_CPP_CLASS) {
    error_continue(
      "@customCppClass is deprecated. Use 'extern' instead. @generateUniqueMap accomplishes most of what '@export @customCppClass' used to.".to_owned(),
    );
  }
  if annotations.contains(ANNOTATION_CUSTOM_MAP) {
    error_continue(
      "@customMap is deprecated. Generating a unique map is opt-in now using @generateUniqueMap."
        .to_owned(),
    );
  }
  if annotations.contains(ANNOTATION_DO_NOT_GENERATE_CAST) {
    flags |= ClassFlag::DoNotGenerateCast as u32;
  }
  if annotations.contains(ANNOTATION_DO_NOT_GENERATE_INSTANCE_TYPE_CHECK) {
    flags |= ClassFlag::DoNotGenerateInstanceTypeCheck as u32;
  }
  if annotations.contains(ANNOTATION_EXPORT) {
    flags |= ClassFlag::Export as u32;
  }
  if annotations.contains(ANNOTATION_HIGHEST_INSTANCE_TYPE_WITHIN_PARENT) {
    flags |= ClassFlag::HighestInstanceTypeWithinParent as u32;
  }
  if annotations.contains(ANNOTATION_LOWEST_INSTANCE_TYPE_WITHIN_PARENT) {
    flags |= ClassFlag::LowestInstanceTypeWithinParent as u32;
  }
  if annotations.contains(ANNOTATION_CPP_OBJECT_LAYOUT_DEFINITION) {
    flags |= ClassFlag::CppObjectLayoutDefinition as u32;
  }

  let is_extern = next!(child_results, Bool)?;
  if is_extern {
    flags |= ClassFlag::Extern as u32;
  }
  let transient = next!(child_results, Bool)?;
  if transient {
    flags |= ClassFlag::Transient as u32;
  }
  let kind = identifier_value(next!(child_results, IdentifierPtr)?);
  if kind == "shape" {
    flags |= ClassFlag::IsShape as u32;
    flags |= ClassFlag::Transient as u32;
    flags |= ClassFlag::HasSameInstanceTypeAsParent as u32;
    flags |= ClassFlag::DoNotGenerateCast as u32;
  }
  let name = next!(child_results, IdentifierPtr)?;
  let name_value = identifier_value(name);
  if !is_valid_type_name(&name_value) {
    let pos = node_pos(name);
    naming_convention_error("Type", &name_value, "UpperCamelCase", pos);
  }
  let extends = next!(child_results, TypeExpressionPtr)?;
  if node_kind(extends) != AstNodeKind::BasicTypeExpression {
    return Err(report_error(
      "Expected type name in extends clause.".to_owned(),
    ));
  }
  let generates = next!(child_results, OptionalStdString)?;
  let body = next!(child_results, OptionalClassBody)?;
  let (mut methods, mut fields_raw) = (Vec::new(), Vec::new());
  match body {
    Some(body) => {
      (methods, fields_raw) = with_ast(|ast| match &ast.node(body).1 {
        AstNode::ClassBody(b) => (b.methods.clone(), b.fields.clone()),
        _ => (Vec::new(), Vec::new()),
      });
    }
    None => flags |= ClassFlag::UndefinedLayout as u32,
  }

  let is_shape = flags & ClassFlag::IsShape as u32 != 0;
  if is_extern && body.is_some() && !is_shape {
    if !do_not_generate_cpp_class {
      flags |= ClassFlag::GenerateCppClassDefinitions as u32;
    }
  } else if do_not_generate_cpp_class {
    lint("Annotation @doNotGenerateCppClass has no effect".to_owned());
  }

  // 按条件注解过滤字段
  let fields: Vec<ClassFieldExpression> = fields_raw
    .into_iter()
    .filter(|exp| {
      exp.conditions.iter().all(|condition| {
        let Ok(flag) = build_flag(&condition.condition, ANNOTATION_IF) else {
          return false;
        };
        match condition.r#type {
          ConditionalAnnotationType::Positive => flag,
          ConditionalAnnotationType::Negative => !flag,
        }
      })
    })
    .collect();

  let mut result = Vec::new();
  result.push(make_node(AstNode::ClassDeclaration(Box::new(
    ClassDeclaration {
      name,
      flags,
      super_class: extends,
      generates: generates.clone(),
      methods,
      fields,
      instance_type_constraints: make_instance_type_constraints(&annotations)?,
    },
  ))));

  // constexpr 伴生类型
  let constexpr_name = make_node_at(
    AstNode::Identifier(Box::new(Identifier {
      value: format!("{}{name_value}", CONSTEXPR_TYPE_PREFIX),
    })),
    node_pos(name),
  );
  let constexpr_extends = add_constexpr(extends)?;
  let mut abstract_type_flags = AbstractTypeFlag::Constexpr as u32;
  if transient {
    abstract_type_flags |= AbstractTypeFlag::Transient as u32;
  }
  let constexpr_decl = make_node_at(
    AstNode::AbstractTypeDeclaration(Box::new(AbstractTypeDeclaration {
      name: constexpr_name,
      flags: abstract_type_flags,
      extends: Some(constexpr_extends),
      generates: Some(generates.clone().unwrap_or_else(|| name_value.clone())),
    })),
    node_pos(name),
  );
  result.push(constexpr_decl);

  // Cast<X> 特化 (torque_internal::DownCastForTorqueClass<X> 包裹)
  if flags & ClassFlag::DoNotGenerateCast as u32 == 0 && flags & ClassFlag::IsShape as u32 == 0 {
    let obj_name = make_node(AstNode::Identifier(Box::new(Identifier {
      value: "obj".to_owned(),
    })));
    let obj_type = basic_type("HeapObject".to_owned(), Vec::new());
    let mut parameters = ParameterList::default();
    parameters.names.push(obj_name);
    parameters.types.push(obj_type);
    let cast_error = make_node(AstNode::Identifier(Box::new(Identifier {
      value: "CastError".to_owned(),
    })));
    let labels = vec![LabelAndTypes {
      name: cast_error,
      types: Vec::new(),
    }];
    let class_type = basic_type(name_value.clone(), Vec::new());

    let downcast_target = identifier_expression(
      vec![TORQUE_INTERNAL_NAMESPACE_STRING.to_owned()],
      "DownCastForTorqueClass",
      vec![class_type],
    );
    let downcast_otherwise = identifier_expression(Vec::new(), "CastError", Vec::new());
    let argument = identifier_expression(Vec::new(), "obj", Vec::new());
    let otherwise_stmt = make_node(AstNode::ExpressionStatement(Box::new(
      ExpressionStatement {
        expression: downcast_otherwise,
      },
    )));
    let value = make_call(downcast_target, &[], &[argument], &[otherwise_stmt]);
    let cast_body = make_node(AstNode::ReturnStatement(Box::new(ReturnStatement {
      value: Some(value),
    })));
    let cast_name = make_node(AstNode::Identifier(Box::new(Identifier {
      value: "Cast".to_owned(),
    })));
    let specialization = make_node(AstNode::SpecializationDeclaration(Box::new(
      SpecializationDeclaration {
        transitioning: false,
        name: cast_name,
        parameters,
        return_type: basic_type(name_value.clone(), Vec::new()),
        labels,
        generic_parameters: vec![basic_type(name_value, Vec::new())],
        body: cast_body,
      },
    )));
    result.push(specialization);
  }
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(result)))
}

/// 原版 MakeNamespaceDeclaration
fn make_namespace_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, StdString)?;
  if !is_snake_case(&name) {
    naming_convention_error("Namespace", &name, "snake_case", current_source_position());
  }
  let declarations = next!(child_results, StdVectorOfDeclarationPtr)?;
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(vec![
    make_node(AstNode::NamespaceDeclaration(Box::new(
      NamespaceDeclaration { declarations, name },
    ))),
  ])))
}

/// 原版 MakeSpecializationDeclaration
fn make_specialization_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let enabled = process_if_annotation_raw(&next!(child_results, VectorOfAnnotation)?)?;
  let transitioning = next!(child_results, Bool)?;
  let name = next!(child_results, IdentifierPtr)?;
  let generic_parameters = next!(child_results, TypeList)?;
  let parameters = next!(child_results, ParameterList)?;
  let return_type = next!(child_results, TypeExpressionPtr)?;
  let labels = next!(child_results, StdVectorOfLabelAndTypes)?;
  let body = next!(child_results, StatementPtr)?;
  check_not_deferred_statement(body);
  let mut result = Vec::new();
  if enabled {
    result.push(make_node(AstNode::SpecializationDeclaration(Box::new(
      SpecializationDeclaration {
        transitioning,
        name,
        parameters,
        return_type,
        labels,
        generic_parameters,
        body,
      },
    ))));
  }
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(result)))
}

/// 原版 MakeStructDeclaration
fn make_struct_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let is_export = has_annotation(child_results, ANNOTATION_EXPORT, "Struct")?;
  let mut flags = 0u32;
  if is_export {
    flags |= StructFlag::Export as u32;
  }
  let name = next!(child_results, IdentifierPtr)?;
  let name_value = identifier_value(name);
  if !is_valid_type_name(&name_value) {
    let pos = node_pos(name);
    naming_convention_error("Struct", &name_value, "UpperCamelCase", pos);
  }
  let generic_parameters = next!(child_results, GenericParameters)?;
  lint_generic_parameters(&generic_parameters);
  let methods = next!(child_results, StdVectorOfDeclarationPtr)?;
  let fields = next!(child_results, StdVectorOfStructFieldExpression)?;
  let struct_decl = make_node(AstNode::StructDeclaration(Box::new(StructDeclaration {
    name,
    flags,
    methods,
    fields,
  })));
  let result = if generic_parameters.is_empty() {
    struct_decl
  } else {
    make_node(AstNode::GenericTypeDeclaration(Box::new(
      GenericTypeDeclaration {
        generic_parameters,
        declaration: struct_decl,
      },
    )))
  };
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(vec![
    result,
  ])))
}

/// 原版 MakeBitFieldStructDeclaration
fn make_bit_field_struct_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let annotations = AnnotationSet::new(child_results, &[], &[ANNOTATION_CPP_SCOPE])?;
  let cpp_scope = annotations.get_string_param(ANNOTATION_CPP_SCOPE)?;
  let name = next!(child_results, IdentifierPtr)?;
  let name_value = identifier_value(name);
  if !is_valid_type_name(&name_value) {
    let pos = node_pos(name);
    naming_convention_error("Bitfield struct", &name_value, "UpperCamelCase", pos);
  }
  let extends = next!(child_results, TypeExpressionPtr)?;
  let fields = next!(child_results, StdVectorOfBitFieldDeclaration)?;
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(vec![
    make_node(AstNode::BitFieldStructDeclaration(Box::new(
      BitFieldStructDeclaration {
        name,
        parent: extends,
        fields,
        cpp_scope,
      },
    ))),
  ])))
}

/// 原版 MakeCppIncludeDeclaration
fn make_cpp_include_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let include_selector = next!(child_results, OptionalStdString)?;
  let include_path = next!(child_results, StdString)?;
  let selector = match include_selector.as_deref() {
    None => IncludeSelector::Any,
    Some("csa") => IncludeSelector::Csa,
    Some("tsa") => IncludeSelector::Tsa,
    Some(other) => {
      return Err(report_error(format!(
        "'{other}' is not a valid include selector"
      )));
    }
  };
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(vec![
    make_node(AstNode::CppIncludeDeclaration(Box::new(
      CppIncludeDeclaration {
        include_path,
        include_selector: selector,
      },
    ))),
  ])))
}

/// 原版 ProcessTorqueImportDeclaration
fn process_torque_import_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let import_path = next!(child_results, StdString)?;
  if !file_relative_to_v8_root_exists(&import_path) {
    error_continue(format!("File '{import_path}' not found."));
  }
  let Some(import_id) = get_source_id(&import_path) else {
    // 原版消息拼写 (缺空格) 保留
    return Err(report_error(format!(
      "File '{import_path}'is not part of the source set."
    )));
  };
  let current = current_source_file();
  with_ast(|ast| ast.declare_import_for_current_file(current, import_id));
  Ok(None)
}

/// 原版 MakeExternalBuiltin
fn make_external_builtin(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let annotations = AnnotationSet::new(child_results, &[], &[ANNOTATION_IF, ANNOTATION_IFNOT])?;
  let enabled = process_if_annotation_set(&annotations)?;
  let transitioning = next!(child_results, Bool)?;
  let js_linkage = next!(child_results, Bool)?;
  let name = next!(child_results, IdentifierPtr)?;
  let generic_parameters = next!(child_results, GenericParameters)?;
  lint_generic_parameters(&generic_parameters);
  let args = next!(child_results, ParameterList)?;
  let return_type = next!(child_results, TypeExpressionPtr)?;
  let result = make_node(AstNode::ExternalBuiltinDeclaration(Box::new(
    ExternalBuiltinDeclaration {
      transitioning,
      javascript_linkage: js_linkage,
      name,
      parameters: args,
      return_type,
      labels: Vec::new(),
    },
  )));
  if !generic_parameters.is_empty() {
    return Err(report_error(
      "External builtins cannot be generic.".to_owned(),
    ));
  }
  let mut results = Vec::new();
  if enabled {
    results.push(result);
  }
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(results)))
}

/// 原版 MakeExternalRuntime
fn make_external_runtime(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let transitioning = next!(child_results, Bool)?;
  let name = next!(child_results, IdentifierPtr)?;
  let args = next!(child_results, ParameterList)?;
  let return_type = next!(child_results, TypeExpressionPtr)?;
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(vec![
    make_node(AstNode::ExternalRuntimeDeclaration(Box::new(
      ExternalRuntimeDeclaration {
        transitioning,
        name,
        parameters: args,
        return_type,
        labels: Vec::new(),
      },
    ))),
  ])))
}

/// 原版 StringLiteralUnquoteAction
fn string_literal_unquote_action(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let literal = next!(child_results, StdString)?;
  Ok(Some(ParseResultValue::StdString(string_literal_unquote(
    &literal,
  ))))
}

/// 原版 MakeBasicTypeExpression
fn make_basic_type_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let namespace_qualification = next!(child_results, StdVectorOfString)?;
  let is_constexpr = next!(child_results, Bool)?;
  let name = next!(child_results, StdString)?;
  let generic_arguments = next!(child_results, TypeList)?;
  let name = if is_constexpr {
    get_constexpr_name(&name)
  } else {
    name
  };
  let type_ = basic_type_qualified(namespace_qualification, name, generic_arguments);
  Ok(Some(ParseResultValue::TypeExpressionPtr(type_)))
}

/// 原版 MakeFunctionTypeExpression
fn make_function_type_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let parameters = next!(child_results, TypeList)?;
  let return_type = next!(child_results, TypeExpressionPtr)?;
  Ok(Some(ParseResultValue::TypeExpressionPtr(make_node(
    AstNode::FunctionTypeExpression(Box::new(FunctionTypeExpression {
      parameters,
      return_type,
    })),
  ))))
}

/// 原版 MakeReferenceTypeExpression: &T / const &T → torque_internal::Reference 族
fn make_reference_type_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let is_const = next!(child_results, Bool)?;
  let referenced_type = next!(child_results, TypeExpressionPtr)?;
  let name = if is_const {
    CONST_REFERENCE_TYPE_STRING
  } else {
    MUTABLE_REFERENCE_TYPE_STRING
  };
  Ok(Some(ParseResultValue::TypeExpressionPtr(
    basic_type_qualified(
      vec![TORQUE_INTERNAL_NAMESPACE_STRING.to_owned()],
      name.to_owned(),
      vec![referenced_type],
    ),
  )))
}

/// 原版 MakeUnionTypeExpression
fn make_union_type_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let a = next!(child_results, TypeExpressionPtr)?;
  let b = next!(child_results, TypeExpressionPtr)?;
  Ok(Some(ParseResultValue::TypeExpressionPtr(make_node(
    AstNode::UnionTypeExpression(Box::new(UnionTypeExpression { a, b })),
  ))))
}

/// 原版 MakeGenericParameter
fn make_generic_parameter(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, IdentifierPtr)?;
  let constraint = next!(child_results, OptionalTypeExpressionPtr)?;
  Ok(Some(ParseResultValue::GenericParameter(GenericParameter {
    name,
    constraint,
  })))
}

/// 原版 MakeExpressionStatement
fn make_expression_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let expression = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::ExpressionStatement(Box::new(ExpressionStatement { expression })),
  ))))
}

/// 原版 MakeIfStatement
fn make_if_statement(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let is_constexpr = next!(child_results, Bool)?;
  let condition = next!(child_results, ExpressionPtr)?;
  let if_true = next!(child_results, StatementPtr)?;
  let if_false = next!(child_results, OptionalStatementPtr)?;
  if let Some(f) = if_false {
    let if_true_block = node_kind(if_true) == AstNodeKind::BlockStatement;
    let if_false_ok =
      node_kind(f) == AstNodeKind::BlockStatement || node_kind(f) == AstNodeKind::IfStatement;
    if !(if_true_block && if_false_ok) {
      return Err(report_error(
        "if-else statements require curly braces".to_owned(),
      ));
    }
  }
  if is_constexpr {
    check_not_deferred_statement(if_true);
  }
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::IfStatement(Box::new(IfStatement {
      condition,
      is_constexpr,
      if_true,
      if_false,
    })),
  ))))
}

/// 原版 MakeEnumDeclaration: enum 展开 (开放/闭集 + constexpr 侧 + FromConstexpr 特化)
fn make_enum_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let is_extern = next!(child_results, Bool)?;
  let name_identifier = next!(child_results, IdentifierPtr)?;
  let name = identifier_value(name_identifier);
  let base_type_expression = next!(child_results, OptionalTypeExpressionPtr)?;
  let constexpr_generates_opt = next!(child_results, OptionalStdString)?;
  let entries = next!(child_results, StdVectorOfEnumEntry)?;
  let is_open = next!(child_results, Bool)?;
  let _scope = SourcePositionScope::new(child_results.matched_input().pos);

  if !is_extern {
    return Err(report_error(
      "non-extern enums are not supported yet".to_owned(),
    ));
  }
  if !is_valid_type_name(&name) {
    naming_convention_error("Type", &name, "UpperCamelCase", current_source_position());
  }
  if constexpr_generates_opt.as_deref() == Some(name.as_str()) {
    lint(format!("Unnecessary 'constexpr' clause for enum {name}"));
  }
  let constexpr_generates = constexpr_generates_opt.unwrap_or_else(|| name.clone());
  let base = base_type_expression;

  let mut result: Vec<NodeId> = Vec::new();
  if let Some(base_val) = base {
    if is_open {
      // 开放: type Enum extends Base + namespace Enum { type kEntry extends Enum; ... }
      result.push(make_node(AstNode::AbstractTypeDeclaration(Box::new(
        AbstractTypeDeclaration {
          name: name_identifier,
          flags: 0,
          extends: Some(base_val),
          generates: None,
        },
      ))));
      let name_type_expression = basic_type_qualified(Vec::new(), name.clone(), Vec::new());
      let mut entry_decls = Vec::new();
      for entry in &entries {
        entry_decls.push(make_node(AstNode::AbstractTypeDeclaration(Box::new(
          AbstractTypeDeclaration {
            name: entry.name,
            flags: 0,
            extends: Some(entry.r#type.unwrap_or(name_type_expression)),
            generates: None,
          },
        ))));
      }
      result.push(make_node(AstNode::NamespaceDeclaration(Box::new(
        NamespaceDeclaration {
          declarations: entry_decls,
          name: name.clone(),
        },
      ))));
    } else {
      // 闭集: namespace Enum { type kEntry extends Base; ... } + type Enum = 联合
      let mut union_type: Option<NodeId> = None;
      let mut entry_decls = Vec::new();
      for entry in &entries {
        entry_decls.push(make_node(AstNode::AbstractTypeDeclaration(Box::new(
          AbstractTypeDeclaration {
            name: entry.name,
            flags: 0,
            extends: Some(entry.r#type.unwrap_or(base_val)),
            generates: None,
          },
        ))));
        let entry_value = identifier_value(entry.name);
        let entry_type = basic_type_qualified(vec![name.clone()], entry_value, Vec::new());
        union_type = Some(match union_type {
          Some(u) => make_node(AstNode::UnionTypeExpression(Box::new(
            UnionTypeExpression {
              a: u,
              b: entry_type,
            },
          ))),
          None => entry_type,
        });
      }
      result.push(make_node(AstNode::NamespaceDeclaration(Box::new(
        NamespaceDeclaration {
          declarations: entry_decls,
          name: name.clone(),
        },
      ))));
      if let Some(union_type) = union_type {
        result.push(make_node(AstNode::TypeAliasDeclaration(Box::new(
          TypeAliasDeclaration {
            name: name_identifier,
            r#type: union_type,
          },
        ))));
      }
    }
  }

  // constexpr 侧
  {
    let constexpr_type_identifier = make_node(AstNode::Identifier(Box::new(Identifier {
      value: format!("{}{name}", CONSTEXPR_TYPE_PREFIX),
    })));
    let constexpr_type_expression = basic_type_qualified(
      Vec::new(),
      format!("{}{name}", CONSTEXPR_TYPE_PREFIX),
      Vec::new(),
    );
    let base_constexpr_type_expression = match base {
      Some(b) => Some(add_constexpr(b)?),
      None => None,
    };
    result.push(make_node(AstNode::AbstractTypeDeclaration(Box::new(
      AbstractTypeDeclaration {
        name: constexpr_type_identifier,
        flags: AbstractTypeFlag::Constexpr as u32,
        extends: base_constexpr_type_expression,
        generates: Some(constexpr_generates.clone()),
      },
    ))));

    let mut type_expr: Option<NodeId> = None;
    let mut fromconstexpr_identifier: Option<NodeId> = None;
    let mut fromconstexpr_parameter_identifier: Option<NodeId> = None;
    let mut fromconstexpr_body: Option<NodeId> = None;
    if let Some(base_val) = base {
      let te = basic_type_qualified(Vec::new(), name.clone(), Vec::new());
      let fid = identifier_node("FromConstexpr");
      let fpid = identifier_node("o");
      // return %RawDownCast<Enum>(%FromConstexpr<Base>(o))
      let inner = make_node(AstNode::IntrinsicCallExpression(Box::new(
        IntrinsicCallExpression {
          name: identifier_node("%FromConstexpr"),
          generic_arguments: vec![base_val],
          arguments: vec![identifier_expression(Vec::new(), "o", Vec::new())],
        },
      )));
      let fbody = make_node(AstNode::ReturnStatement(Box::new(ReturnStatement {
        value: Some(make_node(AstNode::IntrinsicCallExpression(Box::new(
          IntrinsicCallExpression {
            name: identifier_node("%RawDownCast"),
            generic_arguments: vec![te],
            arguments: vec![inner],
          },
        )))),
      })));
      type_expr = Some(te);
      fromconstexpr_identifier = Some(fid);
      fromconstexpr_parameter_identifier = Some(fpid);
      fromconstexpr_body = Some(fbody);
    }

    let mut enum_description = EnumDescription {
      pos: current_source_position(),
      name: name.clone(),
      constexpr_generates: constexpr_generates.clone(),
      is_open,
      entries: Vec::new(),
    };
    let mut entry_decls = Vec::new();
    for entry in &entries {
      let entry_name = identifier_value(entry.name);
      let entry_constexpr_type = format!("{}{entry_name}", CONSTEXPR_TYPE_PREFIX);
      let alias_entry = entry
        .alias_entry
        .as_ref()
        .map(|a| format!("{constexpr_generates}::{a}"))
        .unwrap_or_default();
      enum_description.entries.push(EnumEntryDesc {
        name: format!("{constexpr_generates}::{entry_name}"),
        alias_entry,
      });
      entry_decls.push(make_node(AstNode::AbstractTypeDeclaration(Box::new(
        AbstractTypeDeclaration {
          name: identifier_node(&entry_constexpr_type),
          flags: AbstractTypeFlag::Constexpr as u32,
          extends: Some(constexpr_type_expression),
          generates: Some(constexpr_generates.clone()),
        },
      ))));
      match entry.r#type {
        Some(entry_type) => {
          // 带类型 entry: constexpr 常量 + RawDownCast 常量
          let Some(base_val) = base else {
            let _s = SourcePositionScope::new(node_pos(entry_type));
            return Err(report_error(
              "Enum constants with custom types require an enum with an extends clause.".to_owned(),
            ));
          };
          let constexpr_constant_name =
            identifier_node(&format!("constexpr constant {entry_name}"));
          entry_decls.push(make_node(AstNode::ExternConstDeclaration(Box::new(
            ExternConstDeclaration {
              name: constexpr_constant_name,
              r#type: basic_type_qualified(Vec::new(), entry_constexpr_type.clone(), Vec::new()),
              literal: format!("{constexpr_generates}::{entry_name}"),
            },
          ))));
          if let (Some(fid), Some(te), Some(fpid)) = (
            fromconstexpr_identifier,
            type_expr,
            fromconstexpr_parameter_identifier,
          ) {
            let fromconstexpr_call =
              make_call_from_identifier(fid, &[te], &[identifier_expression_nd(fpid)], &[]);
            entry_decls.push(make_node(AstNode::ConstDeclaration(Box::new(
              ConstDeclaration {
                name: entry.name,
                r#type: entry_type,
                expression: make_node(AstNode::IntrinsicCallExpression(Box::new(
                  IntrinsicCallExpression {
                    name: identifier_node("%RawDownCast"),
                    generic_arguments: vec![entry_type, base_val],
                    arguments: vec![fromconstexpr_call],
                  },
                ))),
              },
            ))));
          }
        }
        None => {
          entry_decls.push(make_node(AstNode::ExternConstDeclaration(Box::new(
            ExternConstDeclaration {
              name: entry.name,
              r#type: basic_type_qualified(Vec::new(), entry_constexpr_type.clone(), Vec::new()),
              literal: format!("{constexpr_generates}::{entry_name}"),
            },
          ))));
        }
      }
      if let (Some(fpid), Some(fid), Some(te), Some(fbody)) = (
        fromconstexpr_parameter_identifier,
        fromconstexpr_identifier,
        type_expr,
        fromconstexpr_body,
      ) {
        let entry_constexpr_type_expr =
          basic_type_qualified(vec![name.clone()], entry_constexpr_type.clone(), Vec::new());
        let mut parameters = ParameterList::default();
        parameters.names.push(fpid);
        parameters.types.push(entry_constexpr_type_expr);
        result.push(make_node(AstNode::SpecializationDeclaration(Box::new(
          SpecializationDeclaration {
            transitioning: false,
            name: fid,
            parameters,
            return_type: te,
            labels: Vec::new(),
            generic_parameters: vec![te, entry_constexpr_type_expr],
            body: fbody,
          },
        ))));
      }
    }
    result.push(make_node(AstNode::NamespaceDeclaration(Box::new(
      NamespaceDeclaration {
        declarations: entry_decls,
        name: name.clone(),
      },
    ))));
    with_ast(|ast| ast.add_enum_description(enum_description));
  }
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(result)))
}

/// Identifier 节点速造
fn identifier_node(value: &str) -> NodeId {
  make_node(AstNode::Identifier(Box::new(Identifier {
    value: value.to_owned(),
  })))
}

/// 带命名空间的基本类型
fn basic_type_qualified(
  namespace_qualification: Vec<String>,
  name: String,
  generic_arguments: Vec<NodeId>,
) -> NodeId {
  let name = make_node(AstNode::Identifier(Box::new(Identifier { value: name })));
  make_node(AstNode::BasicTypeExpression(Box::new(
    BasicTypeExpression {
      namespace_qualification,
      is_constexpr: false,
      name,
      generic_arguments,
    },
  )))
}

/// 无命名空间的基本类型
fn basic_type(name: String, generic_arguments: Vec<NodeId>) -> NodeId {
  basic_type_qualified(Vec::new(), name, generic_arguments)
}

/// IdentifierExpression 构造
fn identifier_expression(
  namespace_qualification: Vec<String>,
  name: &str,
  generic_arguments: Vec<NodeId>,
) -> NodeId {
  let name = identifier_node(name);
  make_node(AstNode::IdentifierExpression(Box::new(
    IdentifierExpression {
      namespace_qualification,
      name,
      generic_arguments,
    },
  )))
}

/// 由既有 Identifier 节点造 IdentifierExpression
fn identifier_expression_nd(name: NodeId) -> NodeId {
  make_node(AstNode::IdentifierExpression(Box::new(
    IdentifierExpression {
      namespace_qualification: Vec::new(),
      name,
      generic_arguments: Vec::new(),
    },
  )))
}

/// 原版 MakeTypeswitchStatement: 逐 case 脱糖为嵌套 try/label 链
fn make_typeswitch_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let expression = next!(child_results, ExpressionPtr)?;
  let cases = next!(child_results, StdVectorOfTypeswitchCase)?;
  let _scope = SourcePositionScope::new(child_results.matched_input().pos);

  let empty_block = || {
    make_node(AstNode::BlockStatement(Box::new(BlockStatement {
      deferred: false,
      statements: Vec::new(),
    })))
  };
  // result = { const __value = expression; ... } 起始块
  let root_block = empty_block();
  let result = root_block;
  {
    let _s = SourcePositionScope::new(node_pos(expression));
    let var_decl = make_node(AstNode::VarDeclarationStatement(Box::new(
      VarDeclarationStatement {
        const_qualified: true,
        name: identifier_node("__value"),
        r#type: None,
        initializer: Some(expression),
      },
    )));
    push_statement(root_block, var_decl);
  }

  let mut current_block = root_block;
  let mut accumulated_types: Option<NodeId> = None;
  let case_count = cases.len();
  for (i, case) in cases.iter().enumerate() {
    let _s = SourcePositionScope::new(case.pos);
    let mut value = identifier_expression(Vec::new(), "__value", Vec::new());
    if i >= 1
      && let Some(excluded) = accumulated_types
    {
      value = make_node(AstNode::AssumeTypeImpossibleExpression(Box::new(
        AssumeTypeImpossibleExpression {
          excluded_type: excluded,
          expression: value,
        },
      )));
    }
    let case_block = if i < case_count - 1 {
      let otherwise_stmt = make_node(AstNode::ExpressionStatement(Box::new(
        ExpressionStatement {
          expression: identifier_expression(Vec::new(), NEXT_CASE_LABEL_NAME, Vec::new()),
        },
      )));
      value = make_call_from_identifier(
        identifier_node("Cast"),
        &[case.r#type],
        &[value],
        &[otherwise_stmt],
      );
      empty_block()
    } else {
      current_block
    };
    let name = case.name.unwrap_or_else(|| identifier_node("__case_value"));
    let var_decl = make_node(AstNode::VarDeclarationStatement(Box::new(
      VarDeclarationStatement {
        const_qualified: true,
        name,
        r#type: Some(case.r#type),
        initializer: Some(value),
      },
    )));
    push_statement(case_block, var_decl);
    push_statement(case_block, case.block);
    if i < case_count - 1 {
      let next_block = empty_block();
      let handler = make_node(AstNode::TryHandler(Box::new(TryHandler {
        handler_kind: HandlerKind::Label,
        label: identifier_node(NEXT_CASE_LABEL_NAME),
        parameters: ParameterList::default(),
        body: next_block,
      })));
      let stmt_expr = make_node(AstNode::StatementExpression(Box::new(
        StatementExpression {
          statement: case_block,
        },
      )));
      let try_label = make_node(AstNode::TryLabelExpression(Box::new(TryLabelExpression {
        try_expression: stmt_expr,
        label_block: handler,
      })));
      let try_stmt = make_node(AstNode::ExpressionStatement(Box::new(
        ExpressionStatement {
          expression: try_label,
        },
      )));
      push_statement(current_block, try_stmt);
      current_block = next_block;
    }
    accumulated_types = Some(match accumulated_types {
      Some(a) => make_node(AstNode::UnionTypeExpression(Box::new(
        UnionTypeExpression { a, b: case.r#type },
      ))),
      None => case.r#type,
    });
  }
  Ok(Some(ParseResultValue::StatementPtr(result)))
}

/// 块尾追加语句
fn push_statement(block: NodeId, statement: NodeId) {
  with_ast(|ast| {
    if let AstNode::BlockStatement(b) = &mut ast.nodes[block as usize].1 {
      b.statements.push(statement);
    }
  });
}

/// 原版 MakeTypeswitchCase
fn make_typeswitch_case(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, OptionalIdentifierPtr)?;
  let type_ = next!(child_results, TypeExpressionPtr)?;
  let block = next!(child_results, StatementPtr)?;
  Ok(Some(ParseResultValue::TypeswitchCase(TypeswitchCase {
    pos: child_results.matched_input().pos,
    name,
    r#type: type_,
    block,
  })))
}

/// 原版 MakeWhileStatement
fn make_while_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let condition = next!(child_results, ExpressionPtr)?;
  let body = next!(child_results, StatementPtr)?;
  let result = make_node(AstNode::WhileStatement(Box::new(WhileStatement {
    condition,
    body,
  })));
  check_not_deferred_statement(result);
  Ok(Some(ParseResultValue::StatementPtr(result)))
}

/// 原版 MakeReturnStatement
fn make_return_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let value = next!(child_results, OptionalExpressionPtr)?;
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::ReturnStatement(Box::new(ReturnStatement { value })),
  ))))
}

/// 原版 MakeTailCallStatement
fn make_tail_call_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let value = next!(child_results, ExpressionPtr)?;
  if node_kind(value) != AstNodeKind::CallExpression {
    return Err(report_error("tail calls must be calls".to_owned()));
  }
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::TailCallStatement(Box::new(TailCallStatement { call: value })),
  ))))
}

/// 原版 MakeVarDeclarationStatement (两条规则共用)
fn make_var_declaration_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let kind = next!(child_results, IdentifierPtr)?;
  let const_qualified = identifier_value(kind) == "const";
  let name = next!(child_results, IdentifierPtr)?;
  let name_value = identifier_value(name);
  if !is_lower_camel_case(&name_value) {
    let pos = node_pos(name);
    naming_convention_error("Variable", &name_value, "lowerCamelCase", pos);
  }
  let type_ = next!(child_results, OptionalTypeExpressionPtr)?;
  let initializer = child_results
    .has_next()
    .then(|| next!(child_results, ExpressionPtr))
    .transpose()?;
  if initializer.is_none() && type_.is_none() {
    return Err(report_error("Declaration is missing a type.".to_owned()));
  }
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::VarDeclarationStatement(Box::new(VarDeclarationStatement {
      const_qualified,
      name,
      r#type: type_,
      initializer,
    })),
  ))))
}

/// 原版 MakeBreakStatement / MakeContinueStatement
fn make_break_statement(
  _child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::BreakStatement(Box::new(BreakStatement {})),
  ))))
}

fn make_continue_statement(
  _child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::ContinueStatement(Box::new(ContinueStatement {})),
  ))))
}

/// 原版 MakeGotoStatement
fn make_goto_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let label = next!(child_results, IdentifierPtr)?;
  let arguments = next!(child_results, StdVectorOfExpressionPtr)?;
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::GotoStatement(Box::new(GotoStatement { label, arguments })),
  ))))
}

/// 原版 MakeBlockStatement
fn make_block_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let deferred = next!(child_results, Bool)?;
  let statements = next!(child_results, StdVectorOfStatementPtr)?;
  for &statement in &statements {
    check_not_deferred_statement(statement);
  }
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::BlockStatement(Box::new(BlockStatement {
      deferred,
      statements,
    })),
  ))))
}

/// 原版 MakeTryLabelExpression (catch 位次校验在此, torque-parser.cc:1867-1889)
fn make_try_label_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let try_block = next!(child_results, StatementPtr)?;
  check_not_deferred_statement(try_block);
  let mut result = try_block;
  let handlers = next!(child_results, StdVectorOfTryHandlerPtr)?;
  if handlers.is_empty() {
    error_continue("Try blocks without catch or label don't make sense.".to_owned());
  }
  for (i, &handler) in handlers.iter().enumerate() {
    let (kind, pos) = with_ast(|ast| {
      let node = ast.node(handler);
      match &node.1 {
        AstNode::TryHandler(h) => (h.handler_kind, node.0.pos),
        _ => (HandlerKind::Label, node.0.pos),
      }
    });
    if i != 0 && kind == HandlerKind::Catch {
      let _s = SourcePositionScope::new(pos);
      return Err(report_error(
        "A catch handler always has to be first, before any label handler, to avoid ambiguity about whether it catches exceptions from preceding handlers or not.".to_owned(),
      ));
    }
    let stmt_expr = make_node(AstNode::StatementExpression(Box::new(
      StatementExpression { statement: result },
    )));
    let try_label = make_node(AstNode::TryLabelExpression(Box::new(TryLabelExpression {
      try_expression: stmt_expr,
      label_block: handler,
    })));
    result = make_node(AstNode::ExpressionStatement(Box::new(
      ExpressionStatement {
        expression: try_label,
      },
    )));
  }
  Ok(Some(ParseResultValue::StatementPtr(result)))
}

/// 原版 MakeForLoopStatement
fn make_for_loop_statement(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let var_declaration = next!(child_results, OptionalStatementPtr)?;
  let test = next!(child_results, OptionalExpressionPtr)?;
  let action = next!(child_results, OptionalExpressionPtr)?;
  let action_stmt = action.map(|a| {
    make_node(AstNode::ExpressionStatement(Box::new(
      ExpressionStatement { expression: a },
    )))
  });
  let body = next!(child_results, StatementPtr)?;
  check_not_deferred_statement(body);
  Ok(Some(ParseResultValue::StatementPtr(make_node(
    AstNode::ForLoopStatement(Box::new(ForLoopStatement {
      var_declaration,
      test,
      action: action_stmt,
      body,
    })),
  ))))
}

/// 原版 MakeLabelBlock
fn make_label_block(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let label = next!(child_results, IdentifierPtr)?;
  let label_value = identifier_value(label);
  if !is_upper_camel_case(&label_value) {
    let pos = node_pos(label);
    naming_convention_error("Label", &label_value, "UpperCamelCase", pos);
  }
  let parameters = next!(child_results, ParameterList)?;
  let body = next!(child_results, StatementPtr)?;
  Ok(Some(ParseResultValue::TryHandlerPtr(make_node(
    AstNode::TryHandler(Box::new(TryHandler {
      handler_kind: HandlerKind::Label,
      label,
      parameters,
      body,
    })),
  ))))
}

/// 原版 MakeCatchBlock
fn make_catch_block(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let parameter_names = next!(child_results, StdVectorOfString)?;
  let body = next!(child_results, StatementPtr)?;
  for variable in &parameter_names {
    if !is_lower_camel_case(variable) {
      naming_convention_error(
        "Exception",
        variable,
        "lowerCamelCase",
        current_source_position(),
      );
    }
  }
  if parameter_names.len() != 2 {
    return Err(report_error(
      "A catch clause needs to have exactly two parameters: The exception and the message. How about: \"catch (exception, message) { ...\".".to_owned(),
    ));
  }
  let mut parameters = ParameterList::default();
  parameters.names.push(identifier_node(&parameter_names[0]));
  parameters
    .types
    .push(basic_type("JSAny".to_owned(), Vec::new()));
  parameters.names.push(identifier_node(&parameter_names[1]));
  parameters
    .types
    .push(make_node(AstNode::UnionTypeExpression(Box::new(
      UnionTypeExpression {
        a: basic_type("JSMessageObject".to_owned(), Vec::new()),
        b: basic_type("TheHole".to_owned(), Vec::new()),
      },
    ))));
  parameters.has_varargs = false;
  Ok(Some(ParseResultValue::TryHandlerPtr(make_node(
    AstNode::TryHandler(Box::new(TryHandler {
      handler_kind: HandlerKind::Catch,
      label: identifier_node(CATCH_LABEL_NAME),
      parameters,
      body,
    })),
  ))))
}

/// 原版 MakeExpressionWithSource
fn make_expression_with_source(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let e = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionWithSource(
    ExpressionWithSource {
      expression: e,
      source: child_results.matched_text().to_owned(),
    },
  )))
}

/// 原版 MakeIdentifier
fn make_identifier(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, StdString)?;
  Ok(Some(ParseResultValue::IdentifierPtr(identifier_node(
    &name,
  ))))
}

/// 原版 MakeIdentifierFromMatchedInput
fn make_identifier_from_matched_input(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  Ok(Some(ParseResultValue::IdentifierPtr(identifier_node(
    child_results.matched_text(),
  ))))
}

/// 原版 MakeRightShiftIdentifier
fn make_right_shift_identifier(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let text = child_results.matched_text();
  if text.chars().any(|c| c != '>') {
    return Err(report_error(
      "right-shift operators may not contain any whitespace".to_owned(),
    ));
  }
  Ok(Some(ParseResultValue::IdentifierPtr(identifier_node(text))))
}

/// 原版 MakeNamespaceQualification
fn make_namespace_qualification(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let global_namespace = next!(child_results, Bool)?;
  let mut namespace_qualification = next!(child_results, StdVectorOfString)?;
  if global_namespace {
    namespace_qualification.insert(0, String::new());
  }
  Ok(Some(ParseResultValue::StdVectorOfString(
    namespace_qualification,
  )))
}

/// 原版 MakeIdentifierExpression
fn make_identifier_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let namespace_qualification = next!(child_results, StdVectorOfString)?;
  let name = next!(child_results, IdentifierPtr)?;
  let generic_arguments = next!(child_results, TypeList)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::IdentifierExpression(Box::new(IdentifierExpression {
      namespace_qualification,
      name,
      generic_arguments,
    })),
  ))))
}

/// 原版 MakeFieldAccessExpression
fn make_field_access_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let object = next!(child_results, ExpressionPtr)?;
  let field = next!(child_results, IdentifierPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::FieldAccessExpression(Box::new(FieldAccessExpression { object, field })),
  ))))
}

/// 原版 MakeReferenceFieldAccessExpression: a->b ≡ (*a).b
fn make_reference_field_access_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let object = next!(child_results, ExpressionPtr)?;
  let field = next!(child_results, IdentifierPtr)?;
  let deref = make_node(AstNode::DereferenceExpression(Box::new(
    DereferenceExpression { reference: object },
  )));
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::FieldAccessExpression(Box::new(FieldAccessExpression {
      object: deref,
      field,
    })),
  ))))
}

/// 原版 MakeElementAccessExpression
fn make_element_access_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let array = next!(child_results, ExpressionPtr)?;
  let index = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::ElementAccessExpression(Box::new(ElementAccessExpression { array, index })),
  ))))
}

/// 原版 MakeDereferenceExpression
fn make_dereference_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let reference = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::DereferenceExpression(Box::new(DereferenceExpression { reference })),
  ))))
}

/// 原版 MakeStructExpression
fn make_struct_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let type_ = next!(child_results, TypeExpressionPtr)?;
  let initializers = next!(child_results, StdVectorOfNameAndExpression)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::StructExpression(Box::new(StructExpression {
      r#type: type_,
      initializers,
    })),
  ))))
}

/// 原版 MakeAssignmentExpression
fn make_assignment_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let location = next!(child_results, ExpressionPtr)?;
  let op = next!(child_results, OptionalStdString)?;
  let value = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::AssignmentExpression(Box::new(AssignmentExpression {
      location,
      op,
      value,
    })),
  ))))
}

/// 原版 MakeFloatingPointLiteralExpression
fn make_floating_point_literal_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let value = next!(child_results, Double)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::FloatingPointLiteralExpression(Box::new(FloatingPointLiteralExpression { value })),
  ))))
}

/// 原版 MakeIntegerLiteralExpression
fn make_integer_literal_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let value = next!(child_results, IntegerLiteral)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::IntegerLiteralExpression(Box::new(IntegerLiteralExpression { value })),
  ))))
}

/// 原版 MakeStringLiteralExpression
fn make_string_literal_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let literal = next!(child_results, StdString)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::StringLiteralExpression(Box::new(StringLiteralExpression { literal })),
  ))))
}

/// 原版 MakeIncrementDecrementExpressionPostfix
fn make_increment_decrement_expression_postfix(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let location = next!(child_results, ExpressionPtr)?;
  let op = next!(child_results, IncrementDecrementOperator)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::IncrementDecrementExpression(Box::new(IncrementDecrementExpression {
      location,
      op,
      postfix: true,
    })),
  ))))
}

/// 原版 MakeIncrementDecrementExpressionPrefix
fn make_increment_decrement_expression_prefix(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let op = next!(child_results, IncrementDecrementOperator)?;
  let location = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::IncrementDecrementExpression(Box::new(IncrementDecrementExpression {
      location,
      op,
      postfix: false,
    })),
  ))))
}

/// 原版 MakeLogicalOrExpression / MakeLogicalAndExpression
fn make_logical_or_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let left = next!(child_results, ExpressionPtr)?;
  let right = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::LogicalOrExpression(Box::new(LogicalOrExpression { left, right })),
  ))))
}

fn make_logical_and_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let left = next!(child_results, ExpressionPtr)?;
  let right = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::LogicalAndExpression(Box::new(LogicalAndExpression { left, right })),
  ))))
}

/// 原版 MakeConditionalExpression
fn make_conditional_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let condition = next!(child_results, ExpressionPtr)?;
  let if_true = next!(child_results, ExpressionPtr)?;
  let if_false = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::ExpressionPtr(make_node(
    AstNode::ConditionalExpression(Box::new(ConditionalExpression {
      condition,
      if_true,
      if_false,
    })),
  ))))
}

/// 原版 MakeLabelAndTypes
fn make_label_and_types(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, IdentifierPtr)?;
  let name_value = identifier_value(name);
  if !is_upper_camel_case(&name_value) {
    let pos = node_pos(name);
    naming_convention_error("Label", &name_value, "UpperCamelCase", pos);
  }
  let types = next!(child_results, TypeList)?;
  Ok(Some(ParseResultValue::LabelAndTypes(LabelAndTypes {
    name,
    types,
  })))
}

/// 原版 MakeNameAndType
fn make_name_and_type(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, IdentifierPtr)?;
  let r#type = next!(child_results, TypeExpressionPtr)?;
  Ok(Some(ParseResultValue::NameAndTypeExpression(
    NameAndTypeExpression { name, r#type },
  )))
}

/// 原版 MakeEnumEntry
fn make_enum_entry(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let annotations = AnnotationSet::new(child_results, &[], &[ANNOTATION_SAME_ENUM_VALUE_AS])?;
  let alias_entry = annotations.get_string_param(ANNOTATION_SAME_ENUM_VALUE_AS)?;
  let name = next!(child_results, IdentifierPtr)?;
  let r#type = next!(child_results, OptionalTypeExpressionPtr)?;
  Ok(Some(ParseResultValue::EnumEntry(EnumEntry {
    name,
    r#type,
    alias_entry,
  })))
}

/// 原版 MakeNameAndExpression
fn make_name_and_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, IdentifierPtr)?;
  let expression = next!(child_results, ExpressionPtr)?;
  Ok(Some(ParseResultValue::NameAndExpression(
    NameAndExpression { name, expression },
  )))
}

/// 原版 MakeNameAndExpressionFromExpression
fn make_name_and_expression_from_expression(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let expression = next!(child_results, ExpressionPtr)?;
  let plain = with_ast(|ast| match &ast.node(expression).1 {
    AstNode::IdentifierExpression(id)
      if id.generic_arguments.is_empty() && id.namespace_qualification.is_empty() =>
    {
      Some(id.name)
    }
    _ => None,
  });
  match plain {
    Some(name) => Ok(Some(ParseResultValue::NameAndExpression(
      NameAndExpression { name, expression },
    ))),
    None => Err(report_error(
      "Constructor parameters need to be named.".to_owned(),
    )),
  }
}

/// 原版 MakeAnnotation
fn make_annotation(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, IdentifierPtr)?;
  let param = next!(child_results, OptionalAnnotationParameter)?;
  Ok(Some(ParseResultValue::Annotation(Annotation {
    name,
    param,
  })))
}

/// 原版 MakeStringAnnotationParameter
fn make_string_annotation_parameter(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let value = next!(child_results, StdString)?;
  Ok(Some(ParseResultValue::AnnotationParameter(
    AnnotationParameter {
      string_value: value,
      int_value: 0,
      is_int: false,
    },
  )))
}

/// 原版 MakeIntAnnotationParameter
fn make_int_annotation_parameter(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let value = next!(child_results, Int32)?;
  Ok(Some(ParseResultValue::AnnotationParameter(
    AnnotationParameter {
      string_value: String::new(),
      int_value: value,
      is_int: true,
    },
  )))
}

/// 原版 MakeClassField (torque-parser.cc:2174-2258)
fn make_class_field(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let annotations = AnnotationSet::new(
    child_results,
    &[
      ANNOTATION_CPP_RELAXED_STORE,
      ANNOTATION_CPP_RELAXED_LOAD,
      ANNOTATION_CPP_RELEASE_STORE,
      ANNOTATION_CPP_ACQUIRE_LOAD,
      ANNOTATION_CUSTOM_WEAK_MARKING,
    ],
    &[ANNOTATION_IF, ANNOTATION_IFNOT],
  )?;
  let synchronization = if annotations.contains(ANNOTATION_CPP_RELEASE_STORE) {
    FieldSynchronization::AcquireRelease
  } else if annotations.contains(ANNOTATION_CPP_RELAXED_STORE) {
    FieldSynchronization::Relaxed
  } else {
    FieldSynchronization::None
  };
  {
    let read_synchronization = if annotations.contains(ANNOTATION_CPP_ACQUIRE_LOAD) {
      FieldSynchronization::AcquireRelease
    } else if annotations.contains(ANNOTATION_CPP_RELAXED_LOAD) {
      FieldSynchronization::Relaxed
    } else {
      FieldSynchronization::None
    };
    if read_synchronization != synchronization {
      return Err(report_error(
        "Incompatible read/write synchronization annotations for a field.".to_owned(),
      ));
    }
  }
  let mut conditions = Vec::new();
  if let Some(condition) = annotations.get_string_param(ANNOTATION_IF)? {
    conditions.push(ConditionalAnnotation {
      condition,
      r#type: ConditionalAnnotationType::Positive,
    });
  }
  if let Some(condition) = annotations.get_string_param(ANNOTATION_IFNOT)? {
    conditions.push(ConditionalAnnotation {
      condition,
      r#type: ConditionalAnnotationType::Negative,
    });
  }
  let mut custom_weak_marking = annotations.contains(ANNOTATION_CUSTOM_WEAK_MARKING);
  let deprecated_weak = next!(child_results, Bool)?;
  if deprecated_weak {
    error_continue(
      "The keyword 'weak' is deprecated. For a field that can contain a normal weak pointer, use type Weak<T>. For a field that should be marked in some custom way, use @customWeakMarking.".to_owned(),
    );
    custom_weak_marking = true;
  }
  let const_qualified = next!(child_results, Bool)?;
  let name = next!(child_results, IdentifierPtr)?;
  let optional = next!(child_results, Bool)?;
  let index = next!(child_results, OptionalExpressionPtr)?;
  if optional && index.is_none() {
    return Err(report_error(
      "Fields using optional specifier must also provide an expression indicating the condition for whether the field is present".to_owned(),
    ));
  }
  let mut index_info = None;
  if let Some(idx) = index {
    let idx = if optional {
      // 可选字段内部为 0/1 计数的索引字段
      let intptr_type = basic_type("intptr".to_owned(), Vec::new());
      let one = make_node(AstNode::IntegerLiteralExpression(Box::new(
        IntegerLiteralExpression {
          value: IntegerLiteral {
            negative: false,
            digits: "1".to_owned(),
          },
        },
      )));
      let zero = make_node(AstNode::IntegerLiteralExpression(Box::new(
        IntegerLiteralExpression {
          value: IntegerLiteral {
            negative: false,
            digits: "0".to_owned(),
          },
        },
      )));
      let fromconstexpr = identifier_node("FromConstexpr");
      let one_call = make_call_from_identifier(fromconstexpr, &[intptr_type], &[one], &[]);
      let zero_call = make_call_from_identifier(
        identifier_node("FromConstexpr"),
        &[intptr_type],
        &[zero],
        &[],
      );
      make_node(AstNode::ConditionalExpression(Box::new(
        ConditionalExpression {
          condition: idx,
          if_true: one_call,
          if_false: zero_call,
        },
      )))
    } else {
      idx
    };
    index_info = Some(ClassFieldIndexInfo {
      expr: idx,
      optional,
    });
  }
  let r#type = next!(child_results, TypeExpressionPtr)?;
  Ok(Some(ParseResultValue::ClassFieldExpression(
    ClassFieldExpression {
      name_and_type: NameAndTypeExpression { name, r#type },
      index: index_info,
      conditions,
      custom_weak_marking,
      const_qualified,
      synchronization,
    },
  )))
}

/// 原版 MakeStructField
fn make_struct_field(child_results: &mut ParseResultIterator) -> Result<Option<ParseResultValue>> {
  let const_qualified = next!(child_results, Bool)?;
  let name = next!(child_results, IdentifierPtr)?;
  let r#type = next!(child_results, TypeExpressionPtr)?;
  Ok(Some(ParseResultValue::StructFieldExpression(
    StructFieldExpression {
      name_and_type: NameAndTypeExpression { name, r#type },
      const_qualified,
    },
  )))
}

/// 原版 MakeBitFieldDeclaration
fn make_bit_field_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let name = next!(child_results, IdentifierPtr)?;
  let r#type = next!(child_results, TypeExpressionPtr)?;
  let num_bits = next!(child_results, Int32)?;
  Ok(Some(ParseResultValue::BitFieldDeclaration(
    BitFieldDeclaration {
      name_and_type: NameAndTypeExpression { name, r#type },
      num_bits,
    },
  )))
}

/// 原版 ExtractAssignmentOperator: 复合赋值 → 基础运算符
fn extract_assignment_operator(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let op = next!(child_results, IdentifierPtr)?;
  let value = identifier_value(op);
  Ok(Some(ParseResultValue::OptionalStdString(Some(
    value[..value.len().saturating_sub(1)].to_owned(),
  ))))
}

/// 原版 MakeTorqueBuiltinDeclaration
fn make_torque_builtin_declaration(
  child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  let annotations = AnnotationSet::new(
    child_results,
    &[
      ANNOTATION_CUSTOM_INTERFACE_DESCRIPTOR,
      ANNOTATION_SUPPORTS_TSA,
    ],
    &[
      ANNOTATION_IF,
      ANNOTATION_IFNOT,
      ANNOTATION_INCREMENT_USE_COUNTER,
    ],
  )?;
  let has_custom_interface_descriptor =
    annotations.contains(ANNOTATION_CUSTOM_INTERFACE_DESCRIPTOR);
  let use_counter_name = annotations.get_string_param(ANNOTATION_INCREMENT_USE_COUNTER)?;
  let enabled = process_if_annotation_set(&annotations)?;
  let supports_tsa = annotations.contains(ANNOTATION_SUPPORTS_TSA);
  let transitioning = next!(child_results, Bool)?;
  let javascript_linkage = next!(child_results, Bool)?;
  let name = next!(child_results, IdentifierPtr)?;
  let name_value = identifier_value(name);
  if !is_upper_camel_case(&name_value) {
    let pos = node_pos(name);
    naming_convention_error("Builtin", &name_value, "UpperCamelCase", pos);
  }
  let generic_parameters = next!(child_results, GenericParameters)?;
  lint_generic_parameters(&generic_parameters);
  let args = next!(child_results, ParameterList)?;
  let return_type = next!(child_results, TypeExpressionPtr)?;
  let body = next!(child_results, OptionalStatementPtr)?;
  let declaration = make_node(AstNode::TorqueBuiltinDeclaration(Box::new(
    TorqueBuiltinDeclaration {
      transitioning,
      javascript_linkage,
      name,
      parameters: args,
      return_type,
      labels: Vec::new(),
      has_custom_interface_descriptor,
      supports_tsa,
      use_counter_name: use_counter_name.clone(),
      body,
    },
  )));
  let result = if generic_parameters.is_empty() {
    if body.is_none() {
      return Err(report_error(
        "A non-generic declaration needs a body.".to_owned(),
      ));
    }
    declaration
  } else {
    make_node(AstNode::GenericCallableDeclaration(Box::new(
      GenericCallableDeclaration {
        generic_parameters,
        declaration,
      },
    )))
  };
  if use_counter_name.is_some() && body.is_none() {
    return Err(report_error(
      "@incrementUseCounter needs a body.".to_owned(),
    ));
  }
  let mut results = Vec::new();
  if enabled {
    results.push(result);
  }
  Ok(Some(ParseResultValue::StdVectorOfDeclarationPtr(results)))
}

// ── earley-parser.h 模板助手具现化动作 (torque-parser.cc:2628-2634) ──

/// 原版 YieldIntegralConstant<IncrementDecrementOperator, kIncrement>
pub fn yield_increment(
  _child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  Ok(Some(ParseResultValue::IncrementDecrementOperator(
    IncrementDecrementOperator::Increment,
  )))
}

/// 原版 YieldIntegralConstant<IncrementDecrementOperator, kDecrement>
pub fn yield_decrement(
  _child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  Ok(Some(ParseResultValue::IncrementDecrementOperator(
    IncrementDecrementOperator::Decrement,
  )))
}

/// 原版 YieldDefaultValue<TypeList> (labelParameter 空 "(...)" 规则)
pub fn yield_empty_label_types_vec(
  _child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  Ok(Some(ParseResultValue::TypeList(TypeList::new())))
}

/// 原版 YieldDefaultValue<ParameterList> = ParameterList::Empty()
pub fn yield_default_parameter_list(
  _child_results: &mut ParseResultIterator,
) -> Result<Option<ParseResultValue>> {
  Ok(Some(ParseResultValue::ParameterList(
    ParameterList::default(),
  )))
}

// ── 文法组合器 (earley-parser.h 模板助手的符号构造面) ──

/// 原版 Sequence (无动作 → DefaultAction)
fn seq(g: &mut Grammar, symbols: Vec<SymbolId>) -> SymbolId {
  let s = g.new_symbol();
  g.tables.add_rule(s, symbols, default_action);
  s
}

/// 原版 TryOrDefault<T>: {x 透传} | {空 → 默认值}
fn try_or_default(g: &mut Grammar, x: SymbolId, empty: Action) -> SymbolId {
  let s = g.new_symbol();
  g.tables.add_rule(s, vec![x], default_action);
  g.tables.add_rule(s, vec![], empty);
  s
}

/// 原版 Optional<T>: {x → Some} | {空 → None}
fn optional_symbol(g: &mut Grammar, x: SymbolId, wrap: Action, none: Action) -> SymbolId {
  let s = g.new_symbol();
  g.tables.add_rule(s, vec![x], wrap);
  g.tables.add_rule(s, vec![], none);
  s
}

/// 原版 List<T> = TryOrDefault<Vec<T>>(NonemptyList<T>)
fn list_sym(
  g: &mut Grammar,
  element: SymbolId,
  separator: Option<SymbolId>,
  singleton: Action,
  extend: Action,
  empty: Action,
) -> SymbolId {
  let nonempty = g.nonempty_list(element, separator, singleton, extend);
  try_or_default(g, nonempty, empty)
}

/// 原版 NonemptyListAllowIfAnnotation<T> (torque-parser.cc:2378-2388)
fn nonempty_list_allow_if(
  g: &mut Grammar,
  element: SymbolId,
  separator: Option<SymbolId>,
  annotations: SymbolId,
  first_if: Action,
  extend_if: Action,
) -> SymbolId {
  let list = g.new_symbol();
  g.tables
    .add_rule(list, vec![annotations, element], first_if);
  let extend_right = separator.map_or(vec![list, annotations, element], |sep| {
    vec![list, annotations, sep, element]
  });
  g.tables.add_rule(list, extend_right, extend_if);
  list
}

/// 原版 ListAllowIfAnnotation<T> = TryOrDefault<Vec<T>>(NonemptyListAllowIfAnnotation)
fn list_allow_if(
  g: &mut Grammar,
  element: SymbolId,
  separator: Option<SymbolId>,
  annotations: SymbolId,
  first_if: Action,
  extend_if: Action,
  empty: Action,
) -> SymbolId {
  let nonempty = nonempty_list_allow_if(g, element, separator, annotations, first_if, extend_if);
  try_or_default(g, nonempty, empty)
}

/// 原版 OneOf(alternatives)
fn one_of(g: &mut Grammar, alternatives: &[&str]) -> SymbolId {
  let result = g.new_symbol();
  for a in alternatives {
    let token = g.token(a);
    g.tables
      .add_rule(result, vec![token], make_identifier_from_matched_input);
  }
  result
}

/// 原版 BinaryOperator(nextLevel, op)
fn binary_operator(g: &mut Grammar, next_level: SymbolId, op: SymbolId) -> SymbolId {
  let result = g.new_symbol();
  g.tables.add_rule(result, vec![next_level], default_action);
  g.tables
    .add_rule(result, vec![result, op, next_level], make_binary_operator);
  result
}

/// 原版 TorqueGrammar (struct : Grammar → 组合)
/// 附注解参数子文法入口 (type/expression/classFieldList 三个成员符号)
#[derive(Debug, Default)]
pub struct TorqueGrammar {
  pub grammar: Grammar,
  pub type_symbol: SymbolId,
  pub expression_symbol: SymbolId,
  pub class_field_list_symbol: SymbolId,
}

impl TorqueGrammar {
  /// 原版 TorqueGrammar(): 全文法构造 (torque-parser.cc:2428-2981)
  #[must_use]
  pub fn new() -> Self {
    let mut g = Grammar::default();
    g.set_whitespace(match_whitespace);

    // ── 字面量与基础符号 ──
    let identifier = g.tables.new_symbol();
    let ident_pattern = g.pattern(match_identifier);
    let runtime_token = g.token("runtime");
    g.tables
      .add_rule(identifier, vec![ident_pattern], yield_matched_input);
    g.tables
      .add_rule(identifier, vec![runtime_token], yield_matched_input);

    let name = g.tables.new_symbol();
    g.tables.add_rule(name, vec![identifier], make_identifier);

    let annotation_name = g.tables.new_symbol();
    let annotation_pattern = g.pattern(match_annotation);
    g.tables.add_rule(
      annotation_name,
      vec![annotation_pattern],
      make_identifier_from_matched_input,
    );

    let intrinsic_name = g.tables.new_symbol();
    let intrinsic_pattern = g.pattern(match_intrinsic_name);
    g.tables.add_rule(
      intrinsic_name,
      vec![intrinsic_pattern],
      make_identifier_from_matched_input,
    );

    let string_literal = g.tables.new_symbol();
    let string_pattern = g.pattern(match_string_literal);
    g.tables
      .add_rule(string_literal, vec![string_pattern], yield_matched_input);

    let external_string = g.tables.new_symbol();
    g.tables.add_rule(
      external_string,
      vec![string_literal],
      string_literal_unquote_action,
    );

    let integer_literal = g.tables.new_symbol();
    let int_pattern = g.pattern(match_integer_literal);
    let hex_pattern = g.pattern(match_hex_literal);
    g.tables
      .add_rule(integer_literal, vec![int_pattern], yield_integer_literal);
    g.tables
      .add_rule(integer_literal, vec![hex_pattern], yield_integer_literal);

    let floating_point_literal = g.tables.new_symbol();
    let float_pattern = g.pattern(match_floating_point_literal);
    g.tables
      .add_rule(floating_point_literal, vec![float_pattern], yield_double);

    let int32_literal = g.tables.new_symbol();
    g.tables
      .add_rule(int32_literal, vec![int_pattern], yield_int32);
    g.tables
      .add_rule(int32_literal, vec![hex_pattern], yield_int32);

    let annotation_parameter = g.tables.new_symbol();
    g.tables.add_rule(
      annotation_parameter,
      vec![identifier],
      make_string_annotation_parameter,
    );
    g.tables.add_rule(
      annotation_parameter,
      vec![int32_literal],
      make_int_annotation_parameter,
    );
    g.tables.add_rule(
      annotation_parameter,
      vec![external_string],
      make_string_annotation_parameter,
    );

    let annotation_parameters = g.tables.new_symbol();
    let paren_l = g.token("(");
    let paren_r = g.token(")");
    g.tables.add_rule(
      annotation_parameters,
      vec![paren_l, annotation_parameter, paren_r],
      default_action,
    );

    let annotation = g.tables.new_symbol();
    let opt_annotation_param = optional_symbol(
      &mut g,
      annotation_parameters,
      wrap_some_annotation_parameter,
      none_annotation_parameter,
    );
    g.tables.add_rule(
      annotation,
      vec![annotation_name, opt_annotation_param],
      make_annotation,
    );

    let annotations = list_sym(
      &mut g,
      annotation,
      None,
      singleton_annotation,
      extend_annotation,
      empty_annotation,
    );

    let namespace_qualification = g.tables.new_symbol();
    let colon_colon = g.token("::");
    let check_global = g.check_if(colon_colon);
    let ident_seq = seq(&mut g, vec![identifier, colon_colon]);
    let ns_list = list_sym(
      &mut g,
      ident_seq,
      None,
      singleton_string,
      extend_string,
      empty_string,
    );
    g.tables.add_rule(
      namespace_qualification,
      vec![check_global, ns_list],
      make_namespace_qualification,
    );

    // ── 类型面 ──
    let type_ = g.tables.new_symbol();
    let simple_type = g.tables.new_symbol();
    let comma = g.token(",");
    let type_list = list_sym(
      &mut g,
      type_,
      Some(comma),
      singleton_type_list,
      extend_type_list,
      empty_type_list,
    );

    let generic_specialization_type_list = g.tables.new_symbol();
    let angle_l = g.token("<");
    let angle_r = g.token(">");
    g.tables.add_rule(
      generic_specialization_type_list,
      vec![angle_l, type_list, angle_r],
      default_action,
    );

    let opt_generic_args =
      try_or_default(&mut g, generic_specialization_type_list, empty_type_list);
    let constexpr_tok = g.token("constexpr");
    let check_constexpr = g.check_if(constexpr_tok);
    g.tables
      .add_rule(simple_type, vec![paren_l, type_, paren_r], default_action);
    g.tables.add_rule(
      simple_type,
      vec![
        namespace_qualification,
        check_constexpr,
        identifier,
        opt_generic_args,
      ],
      make_basic_type_expression,
    );
    let builtin_token = g.token("builtin");
    let arrow = g.token("=>");
    g.tables.add_rule(
      simple_type,
      vec![
        builtin_token,
        paren_l,
        type_list,
        paren_r,
        arrow,
        simple_type,
      ],
      make_function_type_expression,
    );
    let const_tok = g.token("const");
    let check_const = g.check_if(const_tok);
    let amp = g.token("&");
    g.tables.add_rule(
      simple_type,
      vec![check_const, amp, simple_type],
      make_reference_type_expression,
    );
    let pipe = g.token("|");
    g.tables.add_rule(type_, vec![simple_type], default_action);
    g.tables.add_rule(
      type_,
      vec![type_, pipe, simple_type],
      make_union_type_expression,
    );

    let generic_parameter = g.tables.new_symbol();
    let type_token = g.token("type");
    let extends_token = g.token("extends");
    let extends_seq = seq(&mut g, vec![extends_token, type_]);
    let opt_extends = optional_symbol(
      &mut g,
      extends_seq,
      wrap_some_type_expression,
      none_type_expression,
    );
    let colon = g.token(":");
    g.tables.add_rule(
      generic_parameter,
      vec![name, colon, type_token, opt_extends],
      make_generic_parameter,
    );

    let generic_parameters = g.tables.new_symbol();
    let comma = g.token(",");
    let gp_list = list_sym(
      &mut g,
      generic_parameter,
      Some(comma),
      singleton_generic_params,
      extend_generic_params,
      empty_generic_params,
    );
    g.tables.add_rule(
      generic_parameters,
      vec![angle_l, gp_list, angle_r],
      default_action,
    );

    // ── 参数面 ──
    let name_and_type = g.tables.new_symbol();
    g.tables
      .add_rule(name_and_type, vec![name, colon, type_], make_name_and_type);

    let implicit_parameter_list = g.tables.new_symbol();
    let implicit_kw = one_of(&mut g, &["implicit", "js-implicit"]);
    let nat_list = list_sym(
      &mut g,
      name_and_type,
      Some(comma),
      singleton_name_and_type,
      extend_name_and_type,
      empty_name_and_type,
    );
    g.tables.add_rule(
      implicit_parameter_list,
      vec![paren_l, implicit_kw, nat_list, paren_r],
      make_implicit_parameter_list,
    );
    let optional_implicit_parameter_list = optional_symbol(
      &mut g,
      implicit_parameter_list,
      wrap_some_implicit_parameters,
      none_implicit_parameters,
    );

    let type_list_maybe_var_args = g.tables.new_symbol();
    let ellipsis = g.token("...");
    let type_seq = seq(&mut g, vec![type_, comma]);
    let type_trailing_list = list_sym(
      &mut g,
      type_seq,
      None,
      singleton_type_list,
      extend_type_list,
      empty_type_list,
    );
    g.tables.add_rule(
      type_list_maybe_var_args,
      vec![
        optional_implicit_parameter_list,
        paren_l,
        type_trailing_list,
        ellipsis,
        paren_r,
      ],
      make_parameter_list_varargs_types,
    );
    g.tables.add_rule(
      type_list_maybe_var_args,
      vec![
        optional_implicit_parameter_list,
        paren_l,
        type_list,
        paren_r,
      ],
      make_parameter_list_types,
    );

    let label_parameter = g.tables.new_symbol();
    let paren_type_seq = seq(&mut g, vec![paren_l, type_list, paren_r]);
    let opt_label_types = try_or_default(&mut g, paren_type_seq, yield_empty_label_types_vec);
    g.tables.add_rule(
      label_parameter,
      vec![name, opt_label_types],
      make_label_and_types,
    );

    let return_type = g.tables.new_symbol();
    g.tables
      .add_rule(return_type, vec![colon, type_], default_action);
    g.tables
      .add_rule(return_type, vec![], deprecated_make_void_type);

    let labels_token = g.token("labels");
    let label_list = g.nonempty_list(
      label_parameter,
      Some(comma),
      singleton_label_and_types,
      extend_label_and_types,
    );
    let labels_seq = seq(&mut g, vec![labels_token, label_list]);
    let optional_label_list = try_or_default(&mut g, labels_seq, empty_label_and_types);

    // ── 语句/表达式互递归壳 ──
    let atomar_statement = g.tables.new_symbol();
    let statement = g.tables.new_symbol();
    let block = g.tables.new_symbol();
    let expression = g.tables.new_symbol(); // 即 assignmentExpression

    let otherwise_token = g.token("otherwise");
    let stmt_list = g.nonempty_list(
      atomar_statement,
      Some(comma),
      singleton_statement,
      extend_statement,
    );
    let otherwise_seq = seq(&mut g, vec![otherwise_token, stmt_list]);
    let optional_otherwise = try_or_default(&mut g, otherwise_seq, empty_statement);

    let colon_type_seq = seq(&mut g, vec![colon, type_]);
    let optional_type_specifier = optional_symbol(
      &mut g,
      colon_type_seq,
      wrap_some_type_expression,
      none_type_expression,
    );

    let bracket_l = g.token("[");
    let bracket_r = g.token("]");
    let array_spec_seq = seq(&mut g, vec![bracket_l, expression, bracket_r]);
    let optional_array_specifier = optional_symbol(
      &mut g,
      array_spec_seq,
      wrap_some_expression,
      none_expression,
    );

    // ── 类/结构体字段面 ──
    let class_field = g.tables.new_symbol();
    let struct_field = g.tables.new_symbol();
    let bit_field_declaration = g.tables.new_symbol();
    let weak_t = g.token("weak");
    let weak_check = g.check_if(weak_t);
    let const_t = g.token("const");
    let const_check = g.check_if(const_t);
    let __t = g.token("?");
    let optional_check = g.check_if(__t);
    let semicolon = g.token(";");
    g.tables.add_rule(
      class_field,
      vec![
        annotations,
        weak_check,
        const_check,
        name,
        optional_check,
        optional_array_specifier,
        colon,
        type_,
        semicolon,
      ],
      make_class_field,
    );
    let class_field_list = list_sym(
      &mut g,
      class_field,
      None,
      singleton_class_field,
      extend_class_field,
      empty_class_field,
    );
    g.tables.add_rule(
      struct_field,
      vec![const_check, name, colon, type_, semicolon],
      make_struct_field,
    );
    let bit_token = g.token("bit");
    g.tables.add_rule(
      bit_field_declaration,
      vec![
        name,
        colon,
        type_,
        colon,
        int32_literal,
        bit_token,
        semicolon,
      ],
      make_bit_field_declaration,
    );

    let parameter_list_no_vararg = g.tables.new_symbol();
    let parameter_list_allow_vararg = g.tables.new_symbol();
    g.tables.add_rule(
      parameter_list_no_vararg,
      vec![optional_implicit_parameter_list, paren_l, nat_list, paren_r],
      make_parameter_list_named,
    );
    let nat_seq = seq(&mut g, vec![name_and_type, comma]);
    let nat_trailing_list = list_sym(
      &mut g,
      nat_seq,
      None,
      singleton_name_and_type,
      extend_name_and_type,
      empty_name_and_type,
    );
    g.tables.add_rule(
      parameter_list_allow_vararg,
      vec![parameter_list_no_vararg],
      default_action,
    );
    g.tables.add_rule(
      parameter_list_allow_vararg,
      vec![
        optional_implicit_parameter_list,
        paren_l,
        nat_trailing_list,
        ellipsis,
        identifier,
        paren_r,
      ],
      make_parameter_list_named_varargs,
    );

    // ── 表达式优先级链 ──
    let increment_decrement_operator = g.tables.new_symbol();
    let ___t = g.token("++");
    g.tables
      .add_rule(increment_decrement_operator, vec![___t], yield_increment);
    let ___t2 = g.token("--");
    g.tables
      .add_rule(increment_decrement_operator, vec![___t2], yield_decrement);

    let identifier_expression = g.tables.new_symbol();
    let opt_id_generic_args =
      try_or_default(&mut g, generic_specialization_type_list, empty_type_list);
    g.tables.add_rule(
      identifier_expression,
      vec![namespace_qualification, name, opt_id_generic_args],
      make_identifier_expression,
    );

    let argument_list = g.tables.new_symbol();
    let expr_list = list_sym(
      &mut g,
      expression,
      Some(comma),
      singleton_expression,
      extend_expression,
      empty_expression,
    );
    g.tables.add_rule(
      argument_list,
      vec![paren_l, expr_list, paren_r],
      default_action,
    );

    let call_expression = g.tables.new_symbol();
    g.tables.add_rule(
      call_expression,
      vec![identifier_expression, argument_list, optional_otherwise],
      make_call_action,
    );

    let call_method_expression = g.tables.new_symbol();
    let primary_expression = g.tables.new_symbol();
    let dot = g.token(".");
    let arrow_r = g.token("->");
    g.tables.add_rule(
      call_method_expression,
      vec![
        primary_expression,
        dot,
        name,
        argument_list,
        optional_otherwise,
      ],
      make_method_call,
    );

    let named_expression = g.tables.new_symbol();
    g.tables.add_rule(
      named_expression,
      vec![name, colon, expression],
      make_name_and_expression,
    );
    g.tables.add_rule(
      named_expression,
      vec![expression],
      make_name_and_expression_from_expression,
    );

    let initializer_list = g.tables.new_symbol();
    let brace_l = g.token("{");
    let brace_r = g.token("}");
    let ne_list = list_sym(
      &mut g,
      named_expression,
      Some(comma),
      singleton_name_and_expression,
      extend_name_and_expression,
      empty_name_and_expression,
    );
    g.tables.add_rule(
      initializer_list,
      vec![brace_l, ne_list, brace_r],
      default_action,
    );

    let intrinsic_call_expression = g.tables.new_symbol();
    g.tables.add_rule(
      intrinsic_call_expression,
      vec![intrinsic_name, opt_generic_args, argument_list],
      make_intrinsic_call_expression,
    );

    let new_expression = g.tables.new_symbol();
    let new_token = g.token("new");
    let pretenured_tok = g.token("Pretenured");
    let pretenured_seq = seq(&mut g, vec![paren_l, pretenured_tok, paren_r]);
    let pretenured_check = g.check_if(pretenured_seq);
    let clear_padding_tok = g.token("ClearPadding");
    let clear_padding_seq = seq(&mut g, vec![paren_l, clear_padding_tok, paren_r]);
    let clear_padding_check = g.check_if(clear_padding_seq);
    g.tables.add_rule(
      new_expression,
      vec![
        new_token,
        pretenured_check,
        clear_padding_check,
        simple_type,
        initializer_list,
      ],
      make_new_expression,
    );

    let unary_expression = g.tables.new_symbol();
    let unary_ops = one_of(&mut g, &["+", "-", "!", "~", "&"]);
    let star = g.token("*");
    g.tables
      .add_rule(primary_expression, vec![call_expression], default_action);
    g.tables.add_rule(
      primary_expression,
      vec![call_method_expression],
      default_action,
    );
    g.tables.add_rule(
      primary_expression,
      vec![intrinsic_call_expression],
      default_action,
    );
    g.tables.add_rule(
      primary_expression,
      vec![identifier_expression],
      default_action,
    );
    g.tables.add_rule(
      primary_expression,
      vec![primary_expression, dot, name],
      make_field_access_expression,
    );
    g.tables.add_rule(
      primary_expression,
      vec![primary_expression, arrow_r, name],
      make_reference_field_access_expression,
    );
    let bracket_l = g.token("[");
    let bracket_r = g.token("]");
    g.tables.add_rule(
      primary_expression,
      vec![primary_expression, bracket_l, expression, bracket_r],
      make_element_access_expression,
    );
    g.tables.add_rule(
      primary_expression,
      vec![integer_literal],
      make_integer_literal_expression,
    );
    g.tables.add_rule(
      primary_expression,
      vec![floating_point_literal],
      make_floating_point_literal_expression,
    );
    g.tables.add_rule(
      primary_expression,
      vec![string_literal],
      make_string_literal_expression,
    );
    g.tables.add_rule(
      primary_expression,
      vec![simple_type, initializer_list],
      make_struct_expression,
    );
    g.tables
      .add_rule(primary_expression, vec![new_expression], default_action);
    g.tables.add_rule(
      primary_expression,
      vec![paren_l, expression, paren_r],
      default_action,
    );

    g.tables
      .add_rule(unary_expression, vec![primary_expression], default_action);
    g.tables.add_rule(
      unary_expression,
      vec![unary_ops, unary_expression],
      make_unary_operator,
    );
    g.tables.add_rule(
      unary_expression,
      vec![star, unary_expression],
      make_dereference_expression,
    );
    g.tables.add_rule(
      unary_expression,
      vec![ellipsis, unary_expression],
      make_spread_expression,
    );
    g.tables.add_rule(
      unary_expression,
      vec![increment_decrement_operator, unary_expression],
      make_increment_decrement_expression_prefix,
    );
    g.tables.add_rule(
      unary_expression,
      vec![unary_expression, increment_decrement_operator],
      make_increment_decrement_expression_postfix,
    );

    let multiplicative_ops = one_of(&mut g, &["*", "/", "%"]);
    let multiplicative_expression = binary_operator(&mut g, unary_expression, multiplicative_ops);
    let additive_ops = one_of(&mut g, &["+", "-"]);
    let additive_expression = binary_operator(&mut g, multiplicative_expression, additive_ops);

    let shift_operator = g.tables.new_symbol();
    let shift_l = g.token("<<");
    let gt = g.token(">");
    g.tables.add_rule(
      shift_operator,
      vec![shift_l],
      make_identifier_from_matched_input,
    );
    g.tables
      .add_rule(shift_operator, vec![gt, gt], make_right_shift_identifier);
    g.tables.add_rule(
      shift_operator,
      vec![gt, gt, gt],
      make_right_shift_identifier,
    );
    let shift_expression = binary_operator(&mut g, additive_expression, shift_operator);

    let relational_expression = g.tables.new_symbol();
    let relational_ops = one_of(&mut g, &["<", ">", "<=", ">="]);
    g.tables.add_rule(
      relational_expression,
      vec![shift_expression],
      default_action,
    );
    g.tables.add_rule(
      relational_expression,
      vec![shift_expression, relational_ops, shift_expression],
      make_binary_operator,
    );

    let equality_ops = one_of(&mut g, &["==", "!="]);
    let equality_expression = binary_operator(&mut g, relational_expression, equality_ops);
    let bitwise_ops = one_of(&mut g, &["&", "|"]);
    let bitwise_expression = binary_operator(&mut g, equality_expression, bitwise_ops);

    let logical_and_expression = g.tables.new_symbol();
    let and_and = g.token("&&");
    g.tables.add_rule(
      logical_and_expression,
      vec![bitwise_expression],
      default_action,
    );
    g.tables.add_rule(
      logical_and_expression,
      vec![logical_and_expression, and_and, bitwise_expression],
      make_logical_and_expression,
    );

    let logical_or_expression = g.tables.new_symbol();
    let or_or = g.token("||");
    g.tables.add_rule(
      logical_or_expression,
      vec![logical_and_expression],
      default_action,
    );
    g.tables.add_rule(
      logical_or_expression,
      vec![logical_or_expression, or_or, logical_and_expression],
      make_logical_or_expression,
    );

    let conditional_expression = g.tables.new_symbol();
    let question = g.token("?");
    g.tables.add_rule(
      conditional_expression,
      vec![logical_or_expression],
      default_action,
    );
    g.tables.add_rule(
      conditional_expression,
      vec![
        logical_or_expression,
        question,
        expression,
        colon,
        conditional_expression,
      ],
      make_conditional_expression,
    );

    let assignment_operator = g.tables.new_symbol();
    let eq = g.token("=");
    let compound_ops = one_of(
      &mut g,
      &[
        "*=", "/=", "%=", "+=", "-=", "<<=", ">>=", ">>>=", "&=", "^=", "|=",
      ],
    );
    g.tables
      .add_rule(assignment_operator, vec![eq], none_string);
    g.tables.add_rule(
      assignment_operator,
      vec![compound_ops],
      extract_assignment_operator,
    );

    g.tables
      .add_rule(expression, vec![conditional_expression], default_action);
    g.tables.add_rule(
      expression,
      vec![conditional_expression, assignment_operator, expression],
      make_assignment_expression,
    );

    // ── 语句面 ──
    let deferred_t = g.token("deferred");
    let deferred_check = g.check_if(deferred_t);
    let stmt_allow_if_list = list_allow_if(
      &mut g,
      statement,
      None,
      annotations,
      first_if_statement,
      extend_if_statement,
      empty_statement,
    );
    g.tables.add_rule(
      block,
      vec![deferred_check, brace_l, stmt_allow_if_list, brace_r],
      make_block_statement,
    );

    let try_handler = g.tables.new_symbol();
    let label_token = g.token("label");
    let catch_token = g.token("catch");
    let opt_handler_params = try_or_default(
      &mut g,
      parameter_list_no_vararg,
      yield_default_parameter_list,
    );
    g.tables.add_rule(
      try_handler,
      vec![label_token, name, opt_handler_params, block],
      make_label_block,
    );
    let ident_list = list_sym(
      &mut g,
      identifier,
      Some(comma),
      singleton_string,
      extend_string,
      empty_string,
    );
    g.tables.add_rule(
      try_handler,
      vec![catch_token, paren_l, ident_list, paren_r, block],
      make_catch_block,
    );

    let expression_with_source = g.tables.new_symbol();
    g.tables.add_rule(
      expression_with_source,
      vec![expression],
      make_expression_with_source,
    );

    let enum_entry = g.tables.new_symbol();
    g.tables.add_rule(
      enum_entry,
      vec![annotations, name, optional_type_specifier],
      make_enum_entry,
    );

    let var_declaration = g.tables.new_symbol();
    let var_declaration_with_initialization = g.tables.new_symbol();
    let let_or_const = one_of(&mut g, &["let", "const"]);
    g.tables.add_rule(
      var_declaration,
      vec![let_or_const, name, optional_type_specifier],
      make_var_declaration_statement,
    );
    let assign = g.token("=");
    g.tables.add_rule(
      var_declaration_with_initialization,
      vec![
        let_or_const,
        name,
        optional_type_specifier,
        assign,
        expression,
      ],
      make_var_declaration_statement,
    );

    let return_token = g.token("return");
    let opt_return_value =
      optional_symbol(&mut g, expression, wrap_some_expression, none_expression);
    g.tables.add_rule(
      atomar_statement,
      vec![expression],
      make_expression_statement,
    );
    g.tables.add_rule(
      atomar_statement,
      vec![return_token, opt_return_value],
      make_return_statement,
    );
    let tail_t = g.token("tail");
    g.tables.add_rule(
      atomar_statement,
      vec![tail_t, call_expression],
      make_tail_call_statement,
    );
    let break_t = g.token("break");
    g.tables
      .add_rule(atomar_statement, vec![break_t], make_break_statement);
    let continue_t = g.token("continue");
    g.tables
      .add_rule(atomar_statement, vec![continue_t], make_continue_statement);
    let opt_goto_args = try_or_default(&mut g, argument_list, empty_expression);
    let goto_t = g.token("goto");
    g.tables.add_rule(
      atomar_statement,
      vec![goto_t, name, opt_goto_args],
      make_goto_statement,
    );
    let debug_or_unreachable = one_of(&mut g, &["debug", "unreachable"]);
    g.tables.add_rule(
      atomar_statement,
      vec![debug_or_unreachable],
      make_debug_statement,
    );

    let if_token = g.token("if");
    let else_token = g.token("else");
    let constexpr_t = g.token("constexpr");
    let constexpr_if_check = g.check_if(constexpr_t);
    let else_seq = seq(&mut g, vec![else_token, statement]);
    let opt_else = optional_symbol(&mut g, else_seq, wrap_some_statement, none_statement);
    g.tables.add_rule(statement, vec![block], default_action);
    g.tables
      .add_rule(statement, vec![atomar_statement, semicolon], default_action);
    g.tables
      .add_rule(statement, vec![var_declaration, semicolon], default_action);
    g.tables.add_rule(
      statement,
      vec![var_declaration_with_initialization, semicolon],
      default_action,
    );
    g.tables.add_rule(
      statement,
      vec![
        if_token,
        constexpr_if_check,
        paren_l,
        expression,
        paren_r,
        statement,
        opt_else,
      ],
      make_if_statement,
    );

    let typeswitch_case = g.tables.new_symbol();
    let typeswitch_token = g.token("typeswitch");
    let case_token = g.token("case");
    let name_colon_seq = seq(&mut g, vec![name, colon]);
    let opt_case_name = optional_symbol(
      &mut g,
      name_colon_seq,
      wrap_some_identifier,
      none_identifier,
    );
    g.tables.add_rule(
      typeswitch_case,
      vec![
        case_token,
        paren_l,
        opt_case_name,
        type_,
        paren_r,
        colon,
        block,
      ],
      make_typeswitch_case,
    );
    let case_allow_if_list = nonempty_list_allow_if(
      &mut g,
      typeswitch_case,
      None,
      annotations,
      first_if_typeswitch_case,
      extend_if_typeswitch_case,
    );
    g.tables.add_rule(
      statement,
      vec![
        typeswitch_token,
        paren_l,
        expression,
        paren_r,
        brace_l,
        case_allow_if_list,
        brace_r,
      ],
      make_typeswitch_statement,
    );

    let try_token = g.token("try");
    let handler_list = list_sym(
      &mut g,
      try_handler,
      None,
      singleton_try_handler,
      extend_try_handler,
      empty_try_handler,
    );
    g.tables.add_rule(
      statement,
      vec![try_token, block, handler_list],
      make_try_label_expression,
    );

    let assert_kinds = one_of(&mut g, &["dcheck", "check", "sbxcheck", "static_assert"]);
    g.tables.add_rule(
      statement,
      vec![
        assert_kinds,
        paren_l,
        expression_with_source,
        paren_r,
        semicolon,
      ],
      make_assert_statement,
    );

    let while_token = g.token("while");
    g.tables.add_rule(
      statement,
      vec![while_token, paren_l, expression, paren_r, statement],
      make_while_statement,
    );

    let for_token = g.token("for");
    let opt_var_decl = optional_symbol(
      &mut g,
      var_declaration_with_initialization,
      wrap_some_statement,
      none_statement,
    );
    let opt_test = optional_symbol(&mut g, expression, wrap_some_expression, none_expression);
    g.tables.add_rule(
      statement,
      vec![
        for_token,
        paren_l,
        opt_var_decl,
        semicolon,
        opt_test,
        semicolon,
        opt_test,
        paren_r,
        statement,
      ],
      make_for_loop_statement,
    );

    // ── 声明面 ──
    let optional_body = g.tables.new_symbol();
    g.tables
      .add_rule(optional_body, vec![block], wrap_some_statement);
    g.tables
      .add_rule(optional_body, vec![semicolon], none_statement);

    let method = g.tables.new_symbol();
    let transitioning_t = g.token("transitioning");
    let transitioning_check = g.check_if(transitioning_t);
    let operator_token = g.token("operator");
    let operator_seq = seq(&mut g, vec![operator_token, external_string]);
    let opt_operator_name = optional_symbol(&mut g, operator_seq, wrap_some_string, none_string);
    let macro_token = g.token("macro");
    g.tables.add_rule(
      method,
      vec![
        transitioning_check,
        opt_operator_name,
        macro_token,
        name,
        parameter_list_no_vararg,
        return_type,
        optional_label_list,
        block,
      ],
      make_method_declaration,
    );

    let optional_class_body = g.tables.new_symbol();
    let method_list = list_sym(
      &mut g,
      method,
      None,
      singleton_declaration,
      extend_declaration,
      empty_declaration,
    );
    g.tables.add_rule(
      optional_class_body,
      vec![brace_l, method_list, class_field_list, brace_r],
      make_class_body,
    );
    g.tables
      .add_rule(optional_class_body, vec![semicolon], none_statement);

    let declaration = g.tables.new_symbol();
    let extern_token = g.token("extern");
    let extern_check_tok = g.check_if(extern_token);
    let transient_t = g.token("transient");
    let transient_check = g.check_if(transient_t);
    let class_or_shape = one_of(&mut g, &["class", "shape"]);
    let generates_tok = g.token("generates");
    let generates_seq = seq(&mut g, vec![generates_tok, external_string]);
    let opt_generates = optional_symbol(&mut g, generates_seq, wrap_some_string, none_string);
    let constexpr_kw = g.token("constexpr");
    let constexpr_seq = seq(&mut g, vec![constexpr_kw, external_string]);
    let opt_constexpr_generates =
      optional_symbol(&mut g, constexpr_seq, wrap_some_string, none_string);
    let opt_generic_parameters = try_or_default(&mut g, generic_parameters, empty_generic_params);
    let extends_type_seq = seq(&mut g, vec![extends_token, type_]);
    let opt_extends_type = optional_symbol(
      &mut g,
      extends_type_seq,
      wrap_some_type_expression,
      none_type_expression,
    );

    // const X: T = expr;
    let const_t = g.token("const");
    g.tables.add_rule(
      declaration,
      vec![
        annotations,
        const_t,
        name,
        colon,
        type_,
        assign,
        expression,
        semicolon,
      ],
      make_const_declaration,
    );
    // const X: T generates '...';
    let const_t = g.token("const");
    let generates_t = g.token("generates");
    g.tables.add_rule(
      declaration,
      vec![
        const_t,
        name,
        colon,
        type_,
        generates_t,
        external_string,
        semicolon,
      ],
      make_extern_const_declaration,
    );
    // [extern] [transient] class/shape X extends T [generates '...'] body
    g.tables.add_rule(
      declaration,
      vec![
        annotations,
        extern_check_tok,
        transient_check,
        class_or_shape,
        name,
        extends_token,
        type_,
        opt_generates,
        optional_class_body,
      ],
      make_class_declaration,
    );
    // struct X [generics] { methods fields }
    let decl_allow_if_list = list_allow_if(
      &mut g,
      method,
      None,
      annotations,
      first_if_declaration,
      extend_if_declaration,
      empty_declaration,
    );
    let struct_field_allow_if_list = list_allow_if(
      &mut g,
      struct_field,
      None,
      annotations,
      first_if_struct_field,
      extend_if_struct_field,
      empty_struct_field,
    );
    let struct_token = g.token("struct");
    g.tables.add_rule(
      declaration,
      vec![
        annotations,
        struct_token,
        name,
        opt_generic_parameters,
        brace_l,
        decl_allow_if_list,
        struct_field_allow_if_list,
        brace_r,
      ],
      make_struct_declaration,
    );
    // bitfield struct X extends T { fields }
    let bit_field_allow_if_list = list_allow_if(
      &mut g,
      bit_field_declaration,
      None,
      annotations,
      first_if_bit_field,
      extend_if_bit_field,
      empty_bit_field,
    );
    let bitfield_token = g.token("bitfield");
    g.tables.add_rule(
      declaration,
      vec![
        annotations,
        bitfield_token,
        struct_token,
        name,
        extends_token,
        type_,
        brace_l,
        bit_field_allow_if_list,
        brace_r,
      ],
      make_bit_field_struct_declaration,
    );
    // [transient] type X [generics] [extends T] [generates '...'] [constexpr '...'] ;
    g.tables.add_rule(
      declaration,
      vec![
        annotations,
        transient_check,
        type_token,
        name,
        opt_generic_parameters,
        opt_extends_type,
        opt_generates,
        opt_constexpr_generates,
        semicolon,
      ],
      make_abstract_type_declaration,
    );
    // type X = T;
    g.tables.add_rule(
      declaration,
      vec![annotations, type_token, name, assign, type_, semicolon],
      make_type_alias_declaration,
    );
    // intrinsic %X [generics] (params): ret [body]
    let intrinsic_token = g.token("intrinsic");
    g.tables.add_rule(
      declaration,
      vec![
        intrinsic_token,
        intrinsic_name,
        opt_generic_parameters,
        parameter_list_no_vararg,
        return_type,
        optional_body,
      ],
      make_intrinsic_declaration,
    );
    // extern [transitioning] [operator '...'] macro [ns::] X [generics] (params): ret labels... ;
    let ns_seq = seq(&mut g, vec![identifier, colon_colon]);
    let opt_ns = optional_symbol(&mut g, ns_seq, wrap_some_string, none_string);
    g.tables.add_rule(
      declaration,
      vec![
        annotations,
        extern_token,
        transitioning_check,
        opt_operator_name,
        macro_token,
        opt_ns,
        name,
        opt_generic_parameters,
        type_list_maybe_var_args,
        return_type,
        optional_label_list,
        semicolon,
      ],
      make_external_macro,
    );
    // extern [transitioning] [javascript] builtin X [generics] (params): ret ;
    let javascript_t = g.token("javascript");
    let javascript_check = g.check_if(javascript_t);
    let builtin_kw = g.token("builtin");
    g.tables.add_rule(
      declaration,
      vec![
        annotations,
        extern_token,
        transitioning_check,
        javascript_check,
        builtin_kw,
        name,
        opt_generic_parameters,
        type_list_maybe_var_args,
        return_type,
        semicolon,
      ],
      make_external_builtin,
    );
    // extern [transitioning] runtime X (params): ret ;
    let runtime_t = g.token("runtime");
    g.tables.add_rule(
      declaration,
      vec![
        extern_token,
        transitioning_check,
        runtime_t,
        name,
        type_list_maybe_var_args,
        return_type,
        semicolon,
      ],
      make_external_runtime,
    );
    // [transitioning] [operator '...'] macro X [generics] (params): ret labels... [body]
    g.tables.add_rule(
      declaration,
      vec![
        annotations,
        transitioning_check,
        opt_operator_name,
        macro_token,
        name,
        opt_generic_parameters,
        parameter_list_no_vararg,
        return_type,
        optional_label_list,
        optional_body,
      ],
      make_torque_macro_declaration,
    );
    // [transitioning] [javascript] builtin X [generics] (params): ret [body]
    g.tables.add_rule(
      declaration,
      vec![
        annotations,
        transitioning_check,
        javascript_check,
        builtin_kw,
        name,
        opt_generic_parameters,
        parameter_list_allow_vararg,
        return_type,
        optional_body,
      ],
      make_torque_builtin_declaration,
    );
    // [transitioning] X<generics>(params): ret labels... { body }
    g.tables.add_rule(
      declaration,
      vec![
        annotations,
        transitioning_check,
        name,
        generic_specialization_type_list,
        parameter_list_allow_vararg,
        return_type,
        optional_label_list,
        block,
      ],
      make_specialization_declaration,
    );
    // #include [selector] 'path'
    let include_sel_seq = seq(&mut g, vec![bracket_l, external_string, bracket_r]);
    let opt_include_selector =
      optional_symbol(&mut g, include_sel_seq, wrap_some_string, none_string);
    let include_t = g.token("#include");
    g.tables.add_rule(
      declaration,
      vec![include_t, opt_include_selector, external_string],
      make_cpp_include_declaration,
    );
    // [extern] enum X [extends T] [constexpr '...'] { entries [, ...] }
    let enum_token = g.token("enum");
    let enum_entry_allow_if_list = nonempty_list_allow_if(
      &mut g,
      enum_entry,
      Some(comma),
      annotations,
      first_if_enum_entry,
      extend_if_enum_entry,
    );
    let open_enum_seq = seq(&mut g, vec![comma, ellipsis]);
    let open_enum_check = g.check_if(open_enum_seq);
    g.tables.add_rule(
      declaration,
      vec![
        extern_check_tok,
        enum_token,
        name,
        opt_extends_type,
        opt_constexpr_generates,
        brace_l,
        enum_entry_allow_if_list,
        open_enum_check,
        brace_r,
      ],
      make_enum_declaration,
    );
    // namespace x { ... }
    let declaration_list = g.tables.new_symbol();
    let decl_list = list_sym(
      &mut g,
      declaration,
      None,
      singleton_declaration_list,
      extend_declaration_list,
      empty_declaration_list,
    );
    g.tables
      .add_rule(declaration_list, vec![decl_list], concat_list);
    let namespace_token = g.token("namespace");
    g.tables.add_rule(
      declaration,
      vec![
        namespace_token,
        identifier,
        brace_l,
        declaration_list,
        brace_r,
      ],
      make_namespace_declaration,
    );

    // file: 左递归 import/declaration + 空规则
    let file = g.tables.new_symbol();
    let import_token = g.token("import");
    g.tables.add_rule(
      file,
      vec![file, import_token, external_string],
      process_torque_import_declaration,
    );
    g.tables
      .add_rule(file, vec![file, declaration], add_global_declarations);
    g.tables.add_rule(file, vec![], default_action);

    g.start = Some(file);
    Self {
      grammar: g,
      type_symbol: type_,
      expression_symbol: expression,
      class_field_list_symbol: class_field_list,
    }
  }
}

/// 原版 ParseTorque: 源码 → 追加到 Ast
pub fn parse_torque(input: &str, ast: &mut Ast, origin: SourcePosition) -> Result<()> {
  let previous = replace_ast(take(ast));
  let result = {
    let mut grammar = TorqueGrammar::new();
    grammar.grammar.parse(input, origin).map(|_| ())
  };
  *ast = replace_ast(previous);
  result
}

/// 原版 ParseTorqueTypeExpression (V8_TQ_* 注解参数用)
pub fn parse_torque_type_expression(input: &str, ast: &mut Ast) -> Result<NodeId> {
  let previous = replace_ast(take(ast));
  let result = (|| {
    let mut grammar = TorqueGrammar::new();
    let origin = current_origin();
    let parsed = grammar
      .grammar
      .parse_from(grammar.type_symbol, input, origin)?;
    match parsed {
      Some(ParseResultValue::TypeExpressionPtr(id)) => Ok(id),
      _ => Err(report_error(format!(
        "cannot parse Torque type expression: {input}"
      ))),
    }
  })();
  *ast = replace_ast(previous);
  result
}

/// 原版 ParseTorqueExpression
pub fn parse_torque_expression(input: &str, ast: &mut Ast) -> Result<NodeId> {
  let previous = replace_ast(take(ast));
  let result = (|| {
    let mut grammar = TorqueGrammar::new();
    let origin = current_origin();
    let parsed = grammar
      .grammar
      .parse_from(grammar.expression_symbol, input, origin)?;
    match parsed {
      Some(ParseResultValue::ExpressionPtr(id)) => Ok(id),
      _ => Err(report_error(format!(
        "cannot parse Torque expression: {input}"
      ))),
    }
  })();
  *ast = replace_ast(previous);
  result
}

/// 原版 ParseTorqueClassFields (V8_TQ_TAIL_SECTIONS 类字段序列)
pub fn parse_torque_class_fields(input: &str, ast: &mut Ast) -> Result<Vec<ClassFieldExpression>> {
  let previous = replace_ast(take(ast));
  let result = (|| {
    let mut grammar = TorqueGrammar::new();
    let origin = current_origin();
    let parsed = grammar
      .grammar
      .parse_from(grammar.class_field_list_symbol, input, origin)?;
    match parsed {
      Some(ParseResultValue::StdVectorOfClassFieldExpression(fields)) => Ok(fields),
      _ => Err(report_error(format!(
        "cannot parse Torque field declarations: {input}"
      ))),
    }
  })();
  *ast = replace_ast(previous);
  result
}

/// 注解参数解析的源位置 (CurrentSourceFile + 无效行列)
fn current_origin() -> SourcePosition {
  SourcePosition {
    source: current_source_file(),
    ..Default::default()
  }
}
