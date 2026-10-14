#!/usr/bin/env bun
// tool/src/parsing/token.h.js
// 机械提取 v8/src/parsing/token.h 中的 Token 体系与常量表

import { readV8, writeSrc, banner } from "../../lib.js";

const tokenSrc = readV8("src/parsing/token.h");

// 1. 提取二元操作符
const binOpRe = /#define\s+BINARY_OP_TOKEN_LIST\(T,\s*E\)([\s\S]*?)(?=\n#define|$)/;
const binMatch = tokenSrc.match(binOpRe);
if (!binMatch) throw new Error("未找到 BINARY_OP_TOKEN_LIST");
const binOps = [];
const binLineRe = /E\(\s*T\s*,\s*(\w+)\s*,\s*"([^"]+)"\s*,\s*(\d+)\s*\)/g;
let bm;
while ((bm = binLineRe.exec(binMatch[1])) !== null) {
  binOps.push({ name: bm[1], symbol: bm[2], precedence: parseInt(bm[3], 10) });
}

// 2. 模拟 TOKEN_LIST 展开
// BINARY_OP_TOKEN_LIST(T, EXPAND_BINOP_ASSIGN_TOKEN) -> Assign##name, symbol + "=", 2
// BINARY_OP_TOKEN_LIST(T, EXPAND_BINOP_TOKEN) -> name, symbol, precedence
const tokens = [];

function addToken(name, symbol, precedence, isKeyword) {
  tokens.push({
    name,
    symbol: symbol === "nullptr" ? null : symbol,
    precedence: precedence || 0,
    isKeyword,
  });
}

// 解析 TOKEN_LIST(T, K)
const tokenListRe = /#define\s+TOKEN_LIST\(T,\s*K\)([\s\S]*?)(?=\nclass\s+V8_EXPORT_PRIVATE\s+Token|$)/;
const tokenListMatch = tokenSrc.match(tokenListRe);
if (!tokenListMatch) throw new Error("未找到 TOKEN_LIST");
const tokenListBody = tokenListMatch[1];

// 逐行匹配 T(...) / K(...) / BINARY_OP_TOKEN_LIST(...)
const lines = tokenListBody.split("\n");
for (const line of lines) {
  const trimmed = line.trim().replace(/\\$/, "").trim();
  if (!trimmed || trimmed.startsWith("/*")) continue;

  if (trimmed.includes("BINARY_OP_TOKEN_LIST(T, EXPAND_BINOP_ASSIGN_TOKEN)")) {
    for (const op of binOps) {
      addToken(`Assign${op.name}`, `${op.symbol}=`, 2, false);
    }
    continue;
  }
  if (trimmed.includes("BINARY_OP_TOKEN_LIST(T, EXPAND_BINOP_TOKEN)")) {
    for (const op of binOps) {
      addToken(op.name, op.symbol, op.precedence, false);
    }
    continue;
  }

  const m = trimmed.match(/^([TK])\(k(\w+),\s*("([^"]*)"|nullptr),\s*(\d+)\)/);
  if (m) {
    const isK = m[1] === "K";
    const name = m[2];
    const sym = m[3] === "nullptr" ? null : m[4];
    const prec = parseInt(m[5], 10);
    addToken(name, sym, prec, isK);
  }
}

console.log(`成功提取 ${tokens.length} 个 Token 定义`);

// 3. 生成 Rust 模块
let rust = banner("src/parsing/token.h.js", "src/parsing/token.h", "Token 体系定义与操作符分类");

rust += `
use std::fmt;

/// JavaScript Token 枚举 (对应原版 enum Token::Value : uint8_t, 共 ${tokens.length} 种)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Token {
`;

for (let i = 0; i < tokens.length; i++) {
  const t = tokens[i];
  const symStr = t.symbol ? ` \`${t.symbol}\`` : "";
  const def = t.name === "Uninitialized" ? "  #[default]\n" : "";
  rust += `  /// [${i}] ${t.name}${symStr}\n${def}  ${t.name} = ${i},\n`;
}

rust += `}

impl Token {
  /// Token 总数 (原版 kNumTokens)
  pub const COUNT: usize = ${tokens.length};

  /// 从数值反查 Token (原版 Token::Value 转换)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    if (val as usize) < Self::COUNT {
      // SAFETY: Token 枚举值紧凑分布在 0..COUNT 之间且 repr(u8)
      Some(unsafe { std::mem::transmute::<u8, Self>(val) })
    } else {
      None
    }
  }

  /// Token 标识名称 (原版 Token::Name)
  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match self {
`;

for (const t of tokens) {
  rust += `      Self::${t.name} => "${t.name}",\n`;
}

rust += `    }
  }

  /// Token 字符表示 (原版 Token::String)
  pub const fn as_str(&self) -> Option<&'static str> {
    match self {
`;

for (const t of tokens) {
  if (t.symbol) {
    rust += `      Self::${t.name} => Some("${t.symbol}"),\n`;
  }
}
rust += `      _ => None,
    }
  }

  /// Token 字符长度 (原版 Token::StringLength)
  pub const fn string_length(&self) -> u8 {
    match self.as_str() {
      Some(s) => s.len() as u8,
      None => 0,
    }
  }

  /// 是否为关键字 (原版 Token::IsKeyword)
  pub const fn is_keyword(&self) -> bool {
    matches!(
      self,
`;

const keywordTokens = tokens.filter(t => t.isKeyword);
rust += `      ` + keywordTokens.map(t => `Self::${t.name}`).join(" | ") + `\n`;
rust += `    )
  }

  /// 操作符优先级 (原版 Token::Precedence)
  /// \`accept_in\`: 是否接受 \`in\` 作为关系运算符
  pub const fn precedence(&self, accept_in: bool) -> u8 {
    match self {
      Self::In => if accept_in { 10 } else { 0 },
`;

for (const t of tokens) {
  if (t.name === "In") continue;
  if (t.precedence > 0) {
    rust += `      Self::${t.name} => ${t.precedence},\n`;
  }
}

rust += `      _ => 0,
    }
  }

  /// 是否为属性或调用运算符 (原版 IsPropertyOrCall)
  pub const fn is_property_or_call(&self) -> bool {
    (*self as u8) >= (Self::TemplateSpan as u8) && (*self as u8) <= (Self::LeftParen as u8)
  }

  /// 是否为成员访问运算符 (原版 IsMember)
  pub const fn is_member(&self) -> bool {
    (*self as u8) >= (Self::TemplateSpan as u8) && (*self as u8) <= (Self::LeftBracket as u8)
  }

  /// 是否为属性访问运算符 (原版 IsProperty)
  pub const fn is_property(&self) -> bool {
    (*self as u8) >= (Self::Period as u8) && (*self as u8) <= (Self::LeftBracket as u8)
  }

  /// 是否为自动分号候选 (原版 IsAutoSemicolon)
  pub const fn is_auto_semicolon(&self) -> bool {
    (*self as u8) >= (Self::Semicolon as u8) && (*self as u8) <= (Self::Eos as u8)
  }

  /// 是否为箭头或赋值操作符 (原版 IsArrowOrAssignmentOp)
  pub const fn is_arrow_or_assignment_op(&self) -> bool {
    (*self as u8) >= (Self::Arrow as u8) && (*self as u8) <= (Self::AssignSub as u8)
  }

  /// 是否为赋值操作符 (原版 IsAssignmentOp)
  pub const fn is_assignment_op(&self) -> bool {
    (*self as u8) >= (Self::Init as u8) && (*self as u8) <= (Self::AssignSub as u8)
  }

  /// 是否为逻辑赋值操作符 (原版 IsLogicalAssignmentOp: ??=, ||=, &&=)
  pub const fn is_logical_assignment_op(&self) -> bool {
    matches!(self, Self::AssignNullish | Self::AssignOr | Self::AssignAnd)
  }

  /// 是否为二元操作符 (原版 IsBinaryOp)
  pub const fn is_binary_op(&self) -> bool {
    (*self as u8) >= (Self::Comma as u8) && (*self as u8) <= (Self::Sub as u8)
  }

  /// 是否为比较操作符 (原版 IsCompareOp)
  pub const fn is_compare_op(&self) -> bool {
    (*self as u8) >= (Self::Eq as u8) && (*self as u8) <= (Self::In as u8)
  }

  /// 是否为有序关系比较操作符 (原版 IsOrderedRelationalCompareOp)
  pub const fn is_ordered_relational_compare_op(&self) -> bool {
    matches!(self, Self::LessThan | Self::GreaterThan | Self::LessThanEq | Self::GreaterThanEq)
  }

  /// 是否为相等性操作符 (原版 IsEqualityOp)
  pub const fn is_equality_op(&self) -> bool {
    matches!(self, Self::Eq | Self::EqStrict | Self::NotEq | Self::NotEqStrict)
  }

  /// 赋值操作符对应的二元操作符 (原版 BinaryOpForAssignment)
  pub const fn binary_op_for_assignment(&self) -> Option<Self> {
    if !self.is_assignment_op() || matches!(self, Self::Init | Self::Assign) {
      return None;
    }
    let offset = (*self as u8) - (Self::AssignNullish as u8);
    Self::from_repr((Self::Nullish as u8) + offset)
  }

  /// 是否为一元操作符 (原版 IsUnaryOp)
  pub const fn is_unary_op(&self) -> bool {
    matches!(
      self,
      Self::Add | Self::Sub | Self::Not | Self::BitNot | Self::Delete | Self::TypeOf | Self::Void
    )
  }

  /// 是否为自增/自减操作符 (原版 IsCountOp)
  pub const fn is_count_op(&self) -> bool {
    matches!(self, Self::Inc | Self::Dec)
  }

  /// 是否为字面量 (原版 IsLiteral)
  pub const fn is_literal(&self) -> bool {
    matches!(
      self,
      Self::NullLiteral
        | Self::TrueLiteral
        | Self::FalseLiteral
        | Self::Number
        | Self::Smi
        | Self::BigInt
        | Self::String
    )
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name())
  }
}
`;

writeSrc("v8-parsing", "token_generated.rs", rust);
console.log("写入完成: v8-parsing/src/token_generated.rs");
