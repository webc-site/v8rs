// 由 tool/src/parsing/token.h.js 从 v8/src/parsing/token.h 生成; 修订就地改, 复跑覆盖
// Token 体系定义与操作符分类

use std::{fmt, mem::transmute};

/// JavaScript Token 枚举 (对应原版 enum Token::Value : uint8_t, 共 120 种)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Token {
  /// [0] TemplateSpan
  TemplateSpan = 0,
  /// [1] TemplateTail
  TemplateTail = 1,
  /// [2] Period `.`
  Period = 2,
  /// [3] LeftBracket `[`
  LeftBracket = 3,
  /// [4] QuestionPeriod `?.`
  QuestionPeriod = 4,
  /// [5] LeftParen `(`
  LeftParen = 5,
  /// [6] RightParen `)`
  RightParen = 6,
  /// [7] RightBracket `]`
  RightBracket = 7,
  /// [8] LeftBrace `{`
  LeftBrace = 8,
  /// [9] Colon `:`
  Colon = 9,
  /// [10] Ellipsis `...`
  Ellipsis = 10,
  /// [11] Conditional `?`
  Conditional = 11,
  /// [12] Semicolon `;`
  Semicolon = 12,
  /// [13] RightBrace `}`
  RightBrace = 13,
  /// [14] Eos `EOS`
  Eos = 14,
  /// [15] Arrow `=>`
  Arrow = 15,
  /// [16] Init `=init`
  Init = 16,
  /// [17] Assign `=`
  Assign = 17,
  /// [18] AssignNullish `??=`
  AssignNullish = 18,
  /// [19] AssignOr `||=`
  AssignOr = 19,
  /// [20] AssignAnd `&&=`
  AssignAnd = 20,
  /// [21] AssignBitOr `|=`
  AssignBitOr = 21,
  /// [22] AssignBitXor `^=`
  AssignBitXor = 22,
  /// [23] AssignBitAnd `&=`
  AssignBitAnd = 23,
  /// [24] AssignShl `<<=`
  AssignShl = 24,
  /// [25] AssignSar `>>=`
  AssignSar = 25,
  /// [26] AssignShr `>>>=`
  AssignShr = 26,
  /// [27] AssignMul `*=`
  AssignMul = 27,
  /// [28] AssignDiv `/=`
  AssignDiv = 28,
  /// [29] AssignMod `%=`
  AssignMod = 29,
  /// [30] AssignExp `**=`
  AssignExp = 30,
  /// [31] AssignAdd `+=`
  AssignAdd = 31,
  /// [32] AssignSub `-=`
  AssignSub = 32,
  /// [33] Comma `,`
  Comma = 33,
  /// [34] Nullish `??`
  Nullish = 34,
  /// [35] Or `||`
  Or = 35,
  /// [36] And `&&`
  And = 36,
  /// [37] BitOr `|`
  BitOr = 37,
  /// [38] BitXor `^`
  BitXor = 38,
  /// [39] BitAnd `&`
  BitAnd = 39,
  /// [40] Shl `<<`
  Shl = 40,
  /// [41] Sar `>>`
  Sar = 41,
  /// [42] Shr `>>>`
  Shr = 42,
  /// [43] Mul `*`
  Mul = 43,
  /// [44] Div `/`
  Div = 44,
  /// [45] Mod `%`
  Mod = 45,
  /// [46] Exp `**`
  Exp = 46,
  /// [47] Add `+`
  Add = 47,
  /// [48] Sub `-`
  Sub = 48,
  /// [49] Not `!`
  Not = 49,
  /// [50] BitNot `~`
  BitNot = 50,
  /// [51] Delete `delete`
  Delete = 51,
  /// [52] TypeOf `typeof`
  TypeOf = 52,
  /// [53] Void `void`
  Void = 53,
  /// [54] Inc `++`
  Inc = 54,
  /// [55] Dec `--`
  Dec = 55,
  /// [56] Eq `==`
  Eq = 56,
  /// [57] EqStrict `===`
  EqStrict = 57,
  /// [58] NotEq `!=`
  NotEq = 58,
  /// [59] NotEqStrict `!==`
  NotEqStrict = 59,
  /// [60] LessThan `<`
  LessThan = 60,
  /// [61] GreaterThan `>`
  GreaterThan = 61,
  /// [62] LessThanEq `<=`
  LessThanEq = 62,
  /// [63] GreaterThanEq `>=`
  GreaterThanEq = 63,
  /// [64] InstanceOf `instanceof`
  InstanceOf = 64,
  /// [65] In `in`
  In = 65,
  /// [66] Break `break`
  Break = 66,
  /// [67] Case `case`
  Case = 67,
  /// [68] Catch `catch`
  Catch = 68,
  /// [69] Continue `continue`
  Continue = 69,
  /// [70] Debugger `debugger`
  Debugger = 70,
  /// [71] Default `default`
  Default = 71,
  /// [72] Do `do`
  Do = 72,
  /// [73] Else `else`
  Else = 73,
  /// [74] Finally `finally`
  Finally = 74,
  /// [75] For `for`
  For = 75,
  /// [76] Function `function`
  Function = 76,
  /// [77] If `if`
  If = 77,
  /// [78] New `new`
  New = 78,
  /// [79] Return `return`
  Return = 79,
  /// [80] Switch `switch`
  Switch = 80,
  /// [81] Throw `throw`
  Throw = 81,
  /// [82] Try `try`
  Try = 82,
  /// [83] Var `var`
  Var = 83,
  /// [84] While `while`
  While = 84,
  /// [85] With `with`
  With = 85,
  /// [86] This `this`
  This = 86,
  /// [87] NullLiteral `null`
  NullLiteral = 87,
  /// [88] TrueLiteral `true`
  TrueLiteral = 88,
  /// [89] FalseLiteral `false`
  FalseLiteral = 89,
  /// [90] Number
  Number = 90,
  /// [91] Smi
  Smi = 91,
  /// [92] BigInt
  BigInt = 92,
  /// [93] String
  String = 93,
  /// [94] Super `super`
  Super = 94,
  /// [95] Identifier
  Identifier = 95,
  /// [96] Get `get`
  Get = 96,
  /// [97] Set `set`
  Set = 97,
  /// [98] Using `using`
  Using = 98,
  /// [99] Of `of`
  Of = 99,
  /// [100] Accessor `accessor`
  Accessor = 100,
  /// [101] Async `async`
  Async = 101,
  /// [102] Await `await`
  Await = 102,
  /// [103] Yield `yield`
  Yield = 103,
  /// [104] Let `let`
  Let = 104,
  /// [105] Static `static`
  Static = 105,
  /// [106] FutureStrictReservedWord
  FutureStrictReservedWord = 106,
  /// [107] EscapedStrictReservedWord
  EscapedStrictReservedWord = 107,
  /// [108] Enum `enum`
  Enum = 108,
  /// [109] Class `class`
  Class = 109,
  /// [110] Const `const`
  Const = 110,
  /// [111] Export `export`
  Export = 111,
  /// [112] Extends `extends`
  Extends = 112,
  /// [113] Import `import`
  Import = 113,
  /// [114] PrivateName
  PrivateName = 114,
  /// [115] Illegal `ILLEGAL`
  Illegal = 115,
  /// [116] EscapedKeyword
  EscapedKeyword = 116,
  /// [117] Whitespace
  Whitespace = 117,
  /// [118] Uninitialized
  #[default]
  Uninitialized = 118,
  /// [119] RegExpLiteral
  RegExpLiteral = 119,
}

impl Token {
  /// Token 总数 (原版 kNumTokens)
  pub const COUNT: usize = 120;

  /// 从数值反查 Token (原版 Token::Value 转换)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    if (val as usize) < Self::COUNT {
      // SAFETY: Token 枚举值紧凑分布在 0..COUNT 之间且 repr(u8)
      Some(unsafe { transmute::<u8, Self>(val) })
    } else {
      None
    }
  }

  /// Token 标识名称 (原版 Token::Name)
  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::TemplateSpan => "TemplateSpan",
      Self::TemplateTail => "TemplateTail",
      Self::Period => "Period",
      Self::LeftBracket => "LeftBracket",
      Self::QuestionPeriod => "QuestionPeriod",
      Self::LeftParen => "LeftParen",
      Self::RightParen => "RightParen",
      Self::RightBracket => "RightBracket",
      Self::LeftBrace => "LeftBrace",
      Self::Colon => "Colon",
      Self::Ellipsis => "Ellipsis",
      Self::Conditional => "Conditional",
      Self::Semicolon => "Semicolon",
      Self::RightBrace => "RightBrace",
      Self::Eos => "Eos",
      Self::Arrow => "Arrow",
      Self::Init => "Init",
      Self::Assign => "Assign",
      Self::AssignNullish => "AssignNullish",
      Self::AssignOr => "AssignOr",
      Self::AssignAnd => "AssignAnd",
      Self::AssignBitOr => "AssignBitOr",
      Self::AssignBitXor => "AssignBitXor",
      Self::AssignBitAnd => "AssignBitAnd",
      Self::AssignShl => "AssignShl",
      Self::AssignSar => "AssignSar",
      Self::AssignShr => "AssignShr",
      Self::AssignMul => "AssignMul",
      Self::AssignDiv => "AssignDiv",
      Self::AssignMod => "AssignMod",
      Self::AssignExp => "AssignExp",
      Self::AssignAdd => "AssignAdd",
      Self::AssignSub => "AssignSub",
      Self::Comma => "Comma",
      Self::Nullish => "Nullish",
      Self::Or => "Or",
      Self::And => "And",
      Self::BitOr => "BitOr",
      Self::BitXor => "BitXor",
      Self::BitAnd => "BitAnd",
      Self::Shl => "Shl",
      Self::Sar => "Sar",
      Self::Shr => "Shr",
      Self::Mul => "Mul",
      Self::Div => "Div",
      Self::Mod => "Mod",
      Self::Exp => "Exp",
      Self::Add => "Add",
      Self::Sub => "Sub",
      Self::Not => "Not",
      Self::BitNot => "BitNot",
      Self::Delete => "Delete",
      Self::TypeOf => "TypeOf",
      Self::Void => "Void",
      Self::Inc => "Inc",
      Self::Dec => "Dec",
      Self::Eq => "Eq",
      Self::EqStrict => "EqStrict",
      Self::NotEq => "NotEq",
      Self::NotEqStrict => "NotEqStrict",
      Self::LessThan => "LessThan",
      Self::GreaterThan => "GreaterThan",
      Self::LessThanEq => "LessThanEq",
      Self::GreaterThanEq => "GreaterThanEq",
      Self::InstanceOf => "InstanceOf",
      Self::In => "In",
      Self::Break => "Break",
      Self::Case => "Case",
      Self::Catch => "Catch",
      Self::Continue => "Continue",
      Self::Debugger => "Debugger",
      Self::Default => "Default",
      Self::Do => "Do",
      Self::Else => "Else",
      Self::Finally => "Finally",
      Self::For => "For",
      Self::Function => "Function",
      Self::If => "If",
      Self::New => "New",
      Self::Return => "Return",
      Self::Switch => "Switch",
      Self::Throw => "Throw",
      Self::Try => "Try",
      Self::Var => "Var",
      Self::While => "While",
      Self::With => "With",
      Self::This => "This",
      Self::NullLiteral => "NullLiteral",
      Self::TrueLiteral => "TrueLiteral",
      Self::FalseLiteral => "FalseLiteral",
      Self::Number => "Number",
      Self::Smi => "Smi",
      Self::BigInt => "BigInt",
      Self::String => "String",
      Self::Super => "Super",
      Self::Identifier => "Identifier",
      Self::Get => "Get",
      Self::Set => "Set",
      Self::Using => "Using",
      Self::Of => "Of",
      Self::Accessor => "Accessor",
      Self::Async => "Async",
      Self::Await => "Await",
      Self::Yield => "Yield",
      Self::Let => "Let",
      Self::Static => "Static",
      Self::FutureStrictReservedWord => "FutureStrictReservedWord",
      Self::EscapedStrictReservedWord => "EscapedStrictReservedWord",
      Self::Enum => "Enum",
      Self::Class => "Class",
      Self::Const => "Const",
      Self::Export => "Export",
      Self::Extends => "Extends",
      Self::Import => "Import",
      Self::PrivateName => "PrivateName",
      Self::Illegal => "Illegal",
      Self::EscapedKeyword => "EscapedKeyword",
      Self::Whitespace => "Whitespace",
      Self::Uninitialized => "Uninitialized",
      Self::RegExpLiteral => "RegExpLiteral",
    }
  }

  /// Token 字符表示 (原版 Token::String)
  pub const fn as_str(&self) -> Option<&'static str> {
    match self {
      Self::Period => Some("."),
      Self::LeftBracket => Some("["),
      Self::QuestionPeriod => Some("?."),
      Self::LeftParen => Some("("),
      Self::RightParen => Some(")"),
      Self::RightBracket => Some("]"),
      Self::LeftBrace => Some("{"),
      Self::Colon => Some(":"),
      Self::Ellipsis => Some("..."),
      Self::Conditional => Some("?"),
      Self::Semicolon => Some(";"),
      Self::RightBrace => Some("}"),
      Self::Eos => Some("EOS"),
      Self::Arrow => Some("=>"),
      Self::Init => Some("=init"),
      Self::Assign => Some("="),
      Self::AssignNullish => Some("??="),
      Self::AssignOr => Some("||="),
      Self::AssignAnd => Some("&&="),
      Self::AssignBitOr => Some("|="),
      Self::AssignBitXor => Some("^="),
      Self::AssignBitAnd => Some("&="),
      Self::AssignShl => Some("<<="),
      Self::AssignSar => Some(">>="),
      Self::AssignShr => Some(">>>="),
      Self::AssignMul => Some("*="),
      Self::AssignDiv => Some("/="),
      Self::AssignMod => Some("%="),
      Self::AssignExp => Some("**="),
      Self::AssignAdd => Some("+="),
      Self::AssignSub => Some("-="),
      Self::Comma => Some(","),
      Self::Nullish => Some("??"),
      Self::Or => Some("||"),
      Self::And => Some("&&"),
      Self::BitOr => Some("|"),
      Self::BitXor => Some("^"),
      Self::BitAnd => Some("&"),
      Self::Shl => Some("<<"),
      Self::Sar => Some(">>"),
      Self::Shr => Some(">>>"),
      Self::Mul => Some("*"),
      Self::Div => Some("/"),
      Self::Mod => Some("%"),
      Self::Exp => Some("**"),
      Self::Add => Some("+"),
      Self::Sub => Some("-"),
      Self::Not => Some("!"),
      Self::BitNot => Some("~"),
      Self::Delete => Some("delete"),
      Self::TypeOf => Some("typeof"),
      Self::Void => Some("void"),
      Self::Inc => Some("++"),
      Self::Dec => Some("--"),
      Self::Eq => Some("=="),
      Self::EqStrict => Some("==="),
      Self::NotEq => Some("!="),
      Self::NotEqStrict => Some("!=="),
      Self::LessThan => Some("<"),
      Self::GreaterThan => Some(">"),
      Self::LessThanEq => Some("<="),
      Self::GreaterThanEq => Some(">="),
      Self::InstanceOf => Some("instanceof"),
      Self::In => Some("in"),
      Self::Break => Some("break"),
      Self::Case => Some("case"),
      Self::Catch => Some("catch"),
      Self::Continue => Some("continue"),
      Self::Debugger => Some("debugger"),
      Self::Default => Some("default"),
      Self::Do => Some("do"),
      Self::Else => Some("else"),
      Self::Finally => Some("finally"),
      Self::For => Some("for"),
      Self::Function => Some("function"),
      Self::If => Some("if"),
      Self::New => Some("new"),
      Self::Return => Some("return"),
      Self::Switch => Some("switch"),
      Self::Throw => Some("throw"),
      Self::Try => Some("try"),
      Self::Var => Some("var"),
      Self::While => Some("while"),
      Self::With => Some("with"),
      Self::This => Some("this"),
      Self::NullLiteral => Some("null"),
      Self::TrueLiteral => Some("true"),
      Self::FalseLiteral => Some("false"),
      Self::Super => Some("super"),
      Self::Get => Some("get"),
      Self::Set => Some("set"),
      Self::Using => Some("using"),
      Self::Of => Some("of"),
      Self::Accessor => Some("accessor"),
      Self::Async => Some("async"),
      Self::Await => Some("await"),
      Self::Yield => Some("yield"),
      Self::Let => Some("let"),
      Self::Static => Some("static"),
      Self::Enum => Some("enum"),
      Self::Class => Some("class"),
      Self::Const => Some("const"),
      Self::Export => Some("export"),
      Self::Extends => Some("extends"),
      Self::Import => Some("import"),
      Self::Illegal => Some("ILLEGAL"),
      _ => None,
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
      Self::Delete
        | Self::TypeOf
        | Self::Void
        | Self::InstanceOf
        | Self::In
        | Self::Break
        | Self::Case
        | Self::Catch
        | Self::Continue
        | Self::Debugger
        | Self::Default
        | Self::Do
        | Self::Else
        | Self::Finally
        | Self::For
        | Self::Function
        | Self::If
        | Self::New
        | Self::Return
        | Self::Switch
        | Self::Throw
        | Self::Try
        | Self::Var
        | Self::While
        | Self::With
        | Self::This
        | Self::NullLiteral
        | Self::TrueLiteral
        | Self::FalseLiteral
        | Self::Super
        | Self::Get
        | Self::Set
        | Self::Using
        | Self::Of
        | Self::Accessor
        | Self::Async
        | Self::Await
        | Self::Yield
        | Self::Let
        | Self::Static
        | Self::Enum
        | Self::Class
        | Self::Const
        | Self::Export
        | Self::Extends
        | Self::Import
    )
  }

  /// 操作符优先级 (原版 Token::Precedence)
  /// `accept_in`: 是否接受 `in` 作为关系运算符
  pub const fn precedence(&self, accept_in: bool) -> u8 {
    match self {
      Self::In => {
        if accept_in {
          10
        } else {
          0
        }
      }
      Self::Conditional => 3,
      Self::Init => 2,
      Self::Assign => 2,
      Self::AssignNullish => 2,
      Self::AssignOr => 2,
      Self::AssignAnd => 2,
      Self::AssignBitOr => 2,
      Self::AssignBitXor => 2,
      Self::AssignBitAnd => 2,
      Self::AssignShl => 2,
      Self::AssignSar => 2,
      Self::AssignShr => 2,
      Self::AssignMul => 2,
      Self::AssignDiv => 2,
      Self::AssignMod => 2,
      Self::AssignExp => 2,
      Self::AssignAdd => 2,
      Self::AssignSub => 2,
      Self::Comma => 1,
      Self::Nullish => 3,
      Self::Or => 4,
      Self::And => 5,
      Self::BitOr => 6,
      Self::BitXor => 7,
      Self::BitAnd => 8,
      Self::Shl => 11,
      Self::Sar => 11,
      Self::Shr => 11,
      Self::Mul => 13,
      Self::Div => 13,
      Self::Mod => 13,
      Self::Exp => 14,
      Self::Add => 12,
      Self::Sub => 12,
      Self::Eq => 9,
      Self::EqStrict => 9,
      Self::NotEq => 9,
      Self::NotEqStrict => 9,
      Self::LessThan => 10,
      Self::GreaterThan => 10,
      Self::LessThanEq => 10,
      Self::GreaterThanEq => 10,
      Self::InstanceOf => 10,
      _ => 0,
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

  /// 是否为模板片段 (原版 IsTemplate: IsInRange(kTemplateSpan, kTemplateTail))
  pub const fn is_template(&self) -> bool {
    (*self as u8) >= (Self::TemplateSpan as u8) && (*self as u8) <= (Self::TemplateTail as u8)
  }

  /// 是否为严格模式保留字 (原版 IsStrictReservedWord: IsInRange(kYield, kEscapedStrictReservedWord))
  pub const fn is_strict_reserved_word(&self) -> bool {
    (*self as u8) >= (Self::Yield as u8) && (*self as u8) <= (Self::EscapedStrictReservedWord as u8)
  }

  /// 是否可作标识符 (原版 IsValidIdentifier, token.h:238)
  /// is_strict 对应 LanguageMode, is_generator/disallow_await 为解析上下文
  #[must_use]
  pub const fn is_valid_identifier(
    &self,
    is_strict: bool,
    is_generator: bool,
    disallow_await: bool,
  ) -> bool {
    let v = *self as u8;
    if v >= (Self::Identifier as u8) && v <= (Self::Async as u8) {
      return true;
    }
    if matches!(self, Self::Await) {
      return !disallow_await;
    }
    if matches!(self, Self::Yield) {
      return !is_generator && !is_strict;
    }
    self.is_strict_reserved_word() && !is_strict
  }

  /// 是否可作属性名 (原版 IsPropertyName, flags 表 = 关键字恒真 | AnyIdentifier | EscapedKeyword)
  #[must_use]
  pub const fn is_property_name(&self) -> bool {
    if self.is_keyword() {
      return true;
    }
    let v = *self as u8;
    (v >= (Self::Identifier as u8) && v <= (Self::EscapedStrictReservedWord as u8))
      || matches!(self, Self::EscapedKeyword)
  }

  /// 是否为位运算符 (原版 IsBitOp: IsInRange(kBitOr, kShr) || kBitNot, token.h:327)
  #[must_use]
  pub const fn is_bit_op(&self) -> bool {
    let v = *self as u8;
    (v >= (Self::BitOr as u8) && v <= (Self::Shr as u8)) || matches!(self, Self::BitNot)
  }

  /// 是否为移位运算符 (原版 IsShiftOp: IsInRange(kShl, kShr), token.h:336)
  #[must_use]
  pub const fn is_shift_op(&self) -> bool {
    let v = *self as u8;
    v >= (Self::Shl as u8) && v <= (Self::Shr as u8)
  }

  /// 是否为一元/增减操作符 (原版 IsUnaryOrCountOp = IsUnaryOp || IsCountOp)
  #[must_use]
  pub const fn is_unary_or_count_op(&self) -> bool {
    self.is_unary_op() || self.is_count_op()
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
    matches!(
      self,
      Self::LessThan | Self::GreaterThan | Self::LessThanEq | Self::GreaterThanEq
    )
  }

  /// 是否为相等性操作符 (原版 IsEqualityOp)
  pub const fn is_equality_op(&self) -> bool {
    matches!(self, Self::Eq | Self::EqStrict)
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
