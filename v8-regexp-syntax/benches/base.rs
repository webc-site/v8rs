//! 核心评测操作 trait：所有参与评测的实现都实现本 trait
//!
//! 评测项设计（对应 v8-regexp-syntax 公开面）：
//! - parse: RegExpParser 解析代表性模式集（字面量/字符类/范围合并规范化/量词嵌套/
//!   捕获组/命名组/反斜杠引用/Unicode property 路径），覆盖 Flags 各组合
//! - char_ranges: CharacterRange 集合代数批量操作（规范化合并/并/交/差/取反）
//! - format_ast: RegExpTree 的 format_ast 序列化输出
//!
//! 对比实现：regex-syntax 的 Rust 正则解析。
//! 红线（todo.md 10.3）：RegExp 语义禁用 regex 类执行库。regex-syntax 仅作解析对比
//! oracle，不进产品路径；其语义（无 V8 flag/捕获区间/ClassSet 语法差异）不可执行替换。

#[cfg(feature = "bench-regex-syntax")]
use regex_syntax::{
  ast::Ast,
  ast::parse::Parser,
  hir::{ClassUnicode, ClassUnicodeRange},
};
use v8_regexp_syntax::{
  CharacterRange, RegExpFlags, Tree, ast::Interval, char_range::K_MAX_CODE_POINT, format_ast, parse,
};

/// 评测操作 id（criterion benchmark group 名，评测 id 基于 Target::NAME 生成）
pub const OPS: [&str; 3] = ["parse", "char_ranges", "format_ast"];

/// 代表性模式集：固定确定性，两种解析实现都可接受
pub const PATTERNS: [&str; 12] = [
  // 字面量与分支
  "abc",
  "a|b|cd",
  // 量词嵌套
  "(a*)*|(b+){2,}(c)?",
  // 捕获组 + 数字反斜杠引用
  "^\\s*([-+]?\\d+)([eE][-+]?\\d+)?\\s*$",
  "(foo)(bar)\\2",
  // 命名捕获组 + 命名引用
  "(?<year>\\d{4})-(?<month>\\d{2})-(?<day>\\d{2})\\k<year>",
  // 字符类与范围
  "[a-z0-9A-F_-]+",
  "[^\\W\\d\\s]",
  // 范围合并规范化（重叠与相邻范围）
  "[a-mcb-ze-zy0-95-9\\x00-\\x7f]",
  // Unicode property 路径
  "\\p{ID_Start}\\p{ID_Continue}*",
  "[[\\p{ID_Start}]--0-9]",
  // 断言
  "\\bfoo\\B|(?=bar)(?!baz)|[a&&b]",
];

/// 标志位或运算（const 环境用）
const fn both(a: RegExpFlags, b: RegExpFlags) -> RegExpFlags {
  RegExpFlags(a.0 | b.0)
}

/// format_ast 输出基准的树取自以下模式 × 标志组合
pub const AST_SAMPLES: [(&str, RegExpFlags); 6] = [
  ("^\\s*([-+]?\\d+)([eE][-+]?\\d+)?\\s*$", RegExpFlags::NONE),
  ("(a*)*|(b+){2,}(c)?", RegExpFlags::NONE),
  ("[a-mcb-ze-z0-95-9]", RegExpFlags::IGNORE_CASE),
  (
    "(?<y>\\d{4})-\\k<y>",
    both(RegExpFlags::MULTILINE, RegExpFlags::STICKY),
  ),
  ("\\p{ID_Start}\\p{ID_Continue}*", RegExpFlags::UNICODE),
  ("[[\\p{ID_Start}]--0-9]", RegExpFlags::UNICODE_SETS),
];

/// Flags 组合矩阵：覆盖 ECMAScript 2024 全标志位的关键路径
pub const FLAG_MATRIX: [RegExpFlags; 10] = [
  RegExpFlags::NONE,
  RegExpFlags::IGNORE_CASE,
  RegExpFlags::GLOBAL,
  RegExpFlags::MULTILINE,
  RegExpFlags::DOT_ALL,
  RegExpFlags::STICKY,
  RegExpFlags::UNICODE,
  RegExpFlags::UNICODE_SETS,
  both(RegExpFlags::UNICODE, RegExpFlags::IGNORE_CASE),
  both(RegExpFlags::UNICODE_SETS, RegExpFlags::IGNORE_CASE),
];

/// 固定确定性伪随机区间（xorshift64，两实现共用同一数据）
///
/// 码点避开代理区，保证两侧都能无损构造
pub fn sample_pairs(seed: u64, count: usize) -> Vec<(u32, u32)> {
  let mut state = seed | 1;
  let mut next = move || {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
  };
  let mut result = Vec::with_capacity(count);
  for _ in 0..count {
    let from = fix_cp(next() % (K_MAX_CODE_POINT as u64 + 1));
    let to = fix_cp(from as u64 + next() % 0x2000);
    result.push((from, to));
  }
  result
}

/// 代理区码点平移，并夹紧到最大码点
fn fix_cp(cp: u64) -> u32 {
  let cp = if (0xD800..=0xDFFF).contains(&cp) {
    cp + 0x800
  } else {
    cp
  };
  cp.min(K_MAX_CODE_POINT as u64) as u32
}

/// 评测目标 trait：每个方法一次完整批量操作，返回校验和防死代码消除
pub trait Target: Default {
  /// 实现名，评测 id 基于它生成
  const NAME: &'static str;

  /// 解析代表性模式集 × Flags 组合
  fn parse_suite(&mut self) -> u64;

  /// CharacterRange 集合代数批量操作
  fn char_ranges(&mut self) -> u64;

  /// format_ast 输出
  fn format(&mut self) -> u64;
}

/// v8rs 实现（被测产品面）
pub struct V8Target {
  trees: Vec<Tree>,
  lhs: Vec<CharacterRange>,
  rhs: Vec<CharacterRange>,
}

impl Target for V8Target {
  const NAME: &'static str = "v8rs";

  fn parse_suite(&mut self) -> u64 {
    let mut sum = 0u64;
    for pattern in PATTERNS {
      for flags in FLAG_MATRIX {
        // 解析失败路径同样被测（错误对象构造成本计入），不允许 panic
        sum += match parse(pattern, flags) {
          Ok(data) => data.tree.kind() as u64 + data.capture_count as u64,
          Err(_) => 1,
        };
      }
    }
    sum
  }

  fn char_ranges(&mut self) -> u64 {
    let mut sum = 0u64;
    // 并集 = 拼接后规范化合并（范围合并规范化路径）
    let mut union = self.lhs.clone();
    CharacterRange::canonicalize(&mut union);
    union.extend(self.rhs.iter().copied());
    CharacterRange::canonicalize(&mut union);
    sum += union.len() as u64;
    let mut inter = Vec::new();
    CharacterRange::intersect(&self.lhs, &self.rhs, &mut inter);
    sum += inter.len() as u64;
    let mut diff = Vec::new();
    CharacterRange::subtract(&self.lhs, &self.rhs, &mut diff);
    sum += diff.len() as u64;
    let mut neg = Vec::new();
    CharacterRange::negate(&self.lhs, &mut neg);
    sum += neg.len() as u64;
    // 区间代数（Interval::union 合并）与并集等价对照
    let iv_a: Vec<Interval> = self
      .lhs
      .iter()
      .map(|r| Interval::new(r.from as i32, r.to as i32))
      .collect();
    let merged = iv_a
      .into_iter()
      .fold(Interval::empty(), |acc, iv| acc.union(iv));
    sum += (merged.from().max(0) as u32) as u64 + merged.to().max(0) as u64;
    sum
  }

  fn format(&mut self) -> u64 {
    self
      .trees
      .iter()
      .map(|tree| format_ast(tree).len() as u64)
      .sum()
  }
}

impl Default for V8Target {
  fn default() -> Self {
    let trees = AST_SAMPLES
      .iter()
      .filter_map(|(pattern, flags)| parse(pattern, *flags).ok())
      .map(|data| data.tree)
      .collect();
    let to_ranges = |pairs: Vec<(u32, u32)>| -> Vec<CharacterRange> {
      pairs
        .into_iter()
        .map(|(from, to)| CharacterRange::range(from, to))
        .collect()
    };
    Self {
      trees,
      lhs: to_ranges(sample_pairs(0x9E37_79B9_7F4A_7C15, 128)),
      rhs: to_ranges(sample_pairs(0xC2B2_AE3D_27D4_EB4F, 128)),
    }
  }
}

/// regex-syntax 对比实现（仅解析 oracle，不进产品路径，语义差异不可执行替换）
///
/// 注意：regex-syntax 的 ast::parse::Parser 是一次性对象（重复使用会内部断言失败），
/// 每次解析都新建
#[cfg(feature = "bench-regex-syntax")]
pub struct RegexSyntaxTarget {
  asts: Vec<Ast>,
  lhs: ClassUnicode,
  rhs: ClassUnicode,
}

#[cfg(feature = "bench-regex-syntax")]
fn parse_ast(pattern: &str) -> Option<Ast> {
  Parser::new().parse(pattern).ok()
}

#[cfg(feature = "bench-regex-syntax")]
impl Default for RegexSyntaxTarget {
  fn default() -> Self {
    Self {
      asts: AST_SAMPLES
        .iter()
        .filter_map(|(pattern, _)| parse_ast(pattern))
        .collect(),
      lhs: to_class(sample_pairs(0x9E37_79B9_7F4A_7C15, 128)),
      rhs: to_class(sample_pairs(0xC2B2_AE3D_27D4_EB4F, 128)),
    }
  }
}

/// 与 v8rs 相同的确定性码点区间数据映射到 regex-syntax 的 Unicode 字符类区间
#[cfg(feature = "bench-regex-syntax")]
fn to_class(pairs: Vec<(u32, u32)>) -> ClassUnicode {
  ClassUnicode::new(pairs.into_iter().filter_map(|(from, to)| {
    let start = char::from_u32(from)?;
    let end = char::from_u32(to)?;
    Some(ClassUnicodeRange::new(start, end))
  }))
}

#[cfg(feature = "bench-regex-syntax")]
impl Target for RegexSyntaxTarget {
  const NAME: &'static str = "regex-syntax";

  fn parse_suite(&mut self) -> u64 {
    let mut sum = 0u64;
    // oracle 无 V8 Flags 概念：同一模式按标志组合次数重复解析，保持操作数一致
    for pattern in PATTERNS {
      for _ in FLAG_MATRIX {
        sum += match parse_ast(pattern) {
          Some(ast) => format!("{ast:?}").len() as u64,
          None => 1,
        };
      }
    }
    sum
  }

  fn char_ranges(&mut self) -> u64 {
    let mut sum = 0u64;
    let mut union = self.lhs.clone();
    union.union(&self.rhs);
    sum += union.ranges().len() as u64;
    let mut inter = self.lhs.clone();
    inter.intersect(&self.rhs);
    sum += inter.ranges().len() as u64;
    let mut diff = self.lhs.clone();
    diff.difference(&self.rhs);
    sum += diff.ranges().len() as u64;
    let mut neg = self.lhs.clone();
    neg.negate();
    sum += neg.ranges().len() as u64;
    // ClassUnicode::new 构造即规范化，与 v8rs 侧 canonicalize 对照
    let re = to_class(sample_pairs(0x9E37_79B9_7F4A_7C15, 128));
    sum += re.ranges().len() as u64;
    sum
  }

  fn format(&mut self) -> u64 {
    // Ast Display 重渲作为 format_ast 的 oracle 对照
    self
      .asts
      .iter()
      .map(|ast| ast.to_string().len() as u64)
      .sum()
  }
}
