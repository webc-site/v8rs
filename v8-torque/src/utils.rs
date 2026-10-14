// 由 tool/src/torque/misc.js 从 v8/src/torque/utils.h 生成; 修订就地改, 复跑覆盖
// 工具集: Stack/StackRange/ResidueClass/Worklist/Deduplicator/TorqueMessage

use std::{
  collections::{HashMap, HashSet, VecDeque},
  fs,
  hash::Hash,
  path::Path,
};

use crate::{error::Result, source_positions::SourcePosition};

// 原版 C++ 流式工具 (PrintList/PrintCommaSeparatedList/NullOStream/IfDefScope/
// NamespaceScope/IncludeGuardScope/IncludeObjectMacrosScope) → Rust 版生成器直接拼
// String, 不引入 std::io::Write 流抽象 (登记偏差, 见 cpp_builder.rs)。
// 原版 EraseIf/TransformVector → Rust 迭代器适配器 (retain/map), 不另设函数。

/// TorqueMessage::Kind 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageKind {
  Error,
  Lint,
}

/// 原版 TorqueMessage 1:1
#[derive(Debug, Clone)]
pub struct TorqueMessage {
  pub message: String,
  pub position: Option<SourcePosition>,
  pub kind: MessageKind,
}

// 原版 TorqueMessages 上下文 → 编译结果收集 (torque_compiler.rs);
// 原版 MessageBuilder/Error()/Lint()/ReportError 异常流 → Result 传播:
//   Error(...).Throw() → return Err(Error::Message(..));
//   TorqueAbortCompilation → Error::Abort。

/// 原版 BottomOffset 1:1 (自栈底偏移)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BottomOffset {
  pub offset: usize,
}

impl BottomOffset {
  /// 原版 operator+(size_t)
  #[must_use]
  pub fn plus(self, x: usize) -> Self {
    Self {
      offset: self.offset + x,
    }
  }

  /// 原版 operator-(size_t), 下溢为调用方契约错误 (原版 DCHECK)
  #[must_use]
  pub fn minus(self, x: usize) -> Self {
    debug_assert!(self.offset >= x);
    Self {
      offset: self.offset - x,
    }
  }
}

/// 原版 StackRange 1:1 (栈顶区间 [begin, end))
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackRange {
  pub begin: BottomOffset,
  pub end: BottomOffset,
}

impl StackRange {
  #[must_use]
  pub const fn new(begin: BottomOffset, end: BottomOffset) -> Self {
    Self { begin, end }
  }

  #[must_use]
  pub fn size(self) -> usize {
    self.end.offset - self.begin.offset
  }

  /// 原版 Extend: 相邻区间合并 (原版 DCHECK_EQ(end, adjacent.begin))
  pub fn extend(&mut self, adjacent: Self) {
    debug_assert_eq!(self.end, adjacent.begin);
    self.end = adjacent.end;
  }
}

/// 原版 Stack<T> 1:1 — 底部偏移寻址栈 (cfg/求值栈共用)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Stack<T> {
  elements: Vec<T>,
}

impl<T> Stack<T> {
  #[must_use]
  pub fn new() -> Self {
    Self {
      elements: Vec::new(),
    }
  }

  #[must_use]
  pub fn from_vec(v: Vec<T>) -> Self {
    Self { elements: v }
  }

  #[must_use]
  pub fn size(&self) -> usize {
    self.elements.len()
  }

  /// 原版 Peek
  #[must_use]
  pub fn peek(&self, from_bottom: BottomOffset) -> &T {
    &self.elements[from_bottom.offset]
  }

  /// 原版 Poke
  pub fn poke(&mut self, from_bottom: BottomOffset, x: T) {
    self.elements[from_bottom.offset] = x;
  }

  pub fn push(&mut self, x: T) {
    self.elements.push(x);
  }

  /// 原版 TopRange
  #[must_use]
  pub fn top_range(&self, slot_count: usize) -> StackRange {
    debug_assert!(self.size() >= slot_count);
    let above = self.above_top();
    StackRange::new(above.minus(slot_count), above)
  }

  /// 原版 PushMany
  pub fn push_many(&mut self, v: &[T]) -> StackRange
  where
    T: Clone,
  {
    for x in v {
      self.push(x.clone());
    }
    self.top_range(v.len())
  }

  #[must_use]
  pub fn top(&self) -> &T {
    self.peek(self.above_top().minus(1))
  }

  /// 原版 Pop (空栈为调用方契约, 原版 DCHECK; Rust 以 Option 表达, 登记偏差)
  pub fn pop(&mut self) -> Option<T> {
    self.elements.pop()
  }

  /// 原版 PopMany
  pub fn pop_many(&mut self, count: usize) -> Vec<T> {
    debug_assert!(self.elements.len() >= count);
    self.elements.split_off(self.elements.len() - count)
  }

  /// 原版 AboveTop: 栈顶之上的无效偏移 (供 StackRange 用)
  #[must_use]
  pub fn above_top(&self) -> BottomOffset {
    BottomOffset {
      offset: self.size(),
    }
  }

  /// 原版 DeleteRange: 删区间, 高位槽下移补位
  pub fn delete_range(&mut self, range: StackRange) {
    if range.size() == 0 {
      return;
    }
    self.elements.drain(range.begin.offset..range.end.offset);
  }
}

/// 原版 Worklist<T> 1:1 — 去重工作队列
#[derive(Debug)]
pub struct Worklist<T: Copy + Eq + Hash> {
  queue: VecDeque<T>,
  contained: HashSet<T>,
}

// 手写 Default: derive 会附加 T: Default 约束, 不需要
impl<T: Copy + Eq + Hash> Default for Worklist<T> {
  fn default() -> Self {
    Self {
      queue: VecDeque::new(),
      contained: HashSet::new(),
    }
  }
}

impl<T: Copy + Eq + Hash> Worklist<T> {
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.queue.is_empty()
  }

  /// 返回是否首次入队
  pub fn enqueue(&mut self, value: T) -> bool {
    if self.contained.contains(&value) {
      return false;
    }
    self.queue.push_back(value);
    self.contained.insert(value);
    true
  }

  /// 原版 Dequeue (空队列为调用方契约, 原版 DCHECK)
  pub fn dequeue(&mut self) -> Option<T> {
    let value = self.queue.pop_front()?;
    self.contained.remove(&value);
    Some(value)
  }
}

/// 原版 Deduplicator<T> 1:1 — 去重存储, 返回稳定下标
#[derive(Debug)]
pub struct Deduplicator<T: Clone + Eq + Hash> {
  storage: Vec<T>,
  index: HashMap<T, usize>,
}

impl<T: Clone + Eq + Hash> Default for Deduplicator<T> {
  fn default() -> Self {
    Self {
      storage: Vec::new(),
      index: HashMap::new(),
    }
  }
}

impl<T: Clone + Eq + Hash> Deduplicator<T> {
  /// 原版 Add: 已存在则复用, 否则插入; 返回稳定下标
  /// (原版 unordered_set::insert O(1); 值→下标双向表, 线扫退化已修)
  pub fn intern(&mut self, x: T) -> usize {
    if let Some(&i) = self.index.get(&x) {
      return i;
    }
    let i = self.storage.len();
    self.storage.push(x.clone());
    self.index.insert(x, i);
    i
  }

  #[must_use]
  pub fn get(&self, i: usize) -> &T {
    &self.storage[i]
  }
}

/// 原版 ResidueClass 1:1 — 模 2^m 剩余类 (对齐约束的部分知识)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResidueClass {
  value: usize,
  modulus_log_2: usize,
}

impl ResidueClass {
  pub const K_MAX_MODULUS_LOG2: usize = usize::BITS as usize;

  /// 构造即归一化: 模数钳到上限, 非全模时 value 折入 [0, 2^m) (utils.h 构造函数)
  #[must_use]
  pub fn new(value: usize, modulus_log_2: usize) -> Self {
    let m = modulus_log_2.min(Self::K_MAX_MODULUS_LOG2);
    let value = if m < Self::K_MAX_MODULUS_LOG2 {
      value & ((1usize << m) - 1)
    } else {
      value
    };
    Self {
      value,
      modulus_log_2: m,
    }
  }

  /// 原版 Unknown: 0 mod 1 = 全体整数
  #[must_use]
  pub fn unknown() -> Self {
    Self::new(0, 0)
  }

  /// 剩余类代表值 (恒小于 2^modulus_log_2)
  #[must_use]
  pub const fn value(&self) -> usize {
    self.value
  }

  #[must_use]
  pub const fn modulus_log_2(&self) -> usize {
    self.modulus_log_2
  }

  /// 模数达到 usize 位宽时代表具体值
  #[must_use]
  pub const fn single_value(&self) -> Option<usize> {
    if self.modulus_log_2 == Self::K_MAX_MODULUS_LOG2 {
      return Some(self.value);
    }
    None
  }

  /// 原版 AlignmentLog2 (utils.cc:385-388): value==0 → modulus_log_2, 否则尾零数
  #[must_use]
  pub const fn alignment_log2(&self) -> usize {
    if self.value == 0 {
      self.modulus_log_2
    } else {
      self.value.trailing_zeros() as usize
    }
  }

  /// 原版 Alignment: 2^AlignmentLog2
  #[must_use]
  pub const fn alignment(&self) -> usize {
    1usize << self.alignment_log2()
  }

  /// 原版 operator+: 值相加、模数取小 (归一化由构造承担)
  #[must_use]
  pub fn add(&self, rhs: &Self) -> Self {
    Self::new(
      self.value.wrapping_add(rhs.value),
      self.modulus_log_2.min(rhs.modulus_log_2),
    )
  }

  /// 原版 operator*: 新模数 = min(a.m + b.align, b.m + a.align)
  /// ({x+k2^a} * {y+l2^b} ⊆ {xy + k2^c}, c 取 x2^b 与 y2^a 的 gcd 的 log2)
  #[must_use]
  pub fn mul(&self, rhs: &Self) -> Self {
    Self::new(
      self.value.wrapping_mul(rhs.value),
      (self.modulus_log_2 + rhs.alignment_log2()).min(rhs.modulus_log_2 + self.alignment_log2()),
    )
  }

  /// 原版 operator<< (utils.cc:392-394): 具体值直出, 否则 "[v mod 2^m]"
  #[must_use]
  pub fn to_display_string(&self) -> String {
    if let Some(v) = self.single_value() {
      return v.to_string();
    }
    format!("[{} mod 2^{}]", self.value, self.modulus_log_2)
  }
}

/// 原版 StringLiteralQuote (utils.cc:55-78)
#[must_use]
pub fn string_literal_quote(s: &str) -> String {
  let mut result = String::with_capacity(s.len() + 2);
  result.push('"');
  for c in s.chars() {
    match c {
      '\n' => result.push_str("\\n"),
      '\r' => result.push_str("\\r"),
      '\t' => result.push_str("\\t"),
      '"' | '\\' => {
        result.push('\\');
        result.push(c);
      }
      _ => result.push(c),
    }
  }
  result.push('"');
  result
}

/// 原版 StringLiteralUnquote (utils.cc:24-53): 剥首尾引号 + 反转义
#[must_use]
pub fn string_literal_unquote(s: &str) -> String {
  let body = s
    .strip_prefix('"')
    .and_then(|x| x.strip_suffix('"'))
    .unwrap_or(
      s.strip_prefix('\'')
        .and_then(|x| x.strip_suffix('\''))
        .unwrap_or(s),
    );
  let mut result = String::with_capacity(body.len());
  let mut chars = body.chars();
  while let Some(c) = chars.next() {
    if c != '\\' {
      result.push(c);
      continue;
    }
    match chars.next() {
      Some('n') => result.push('\n'),
      Some('r') => result.push('\r'),
      Some('t') => result.push('\t'),
      Some(escaped) => result.push(escaped),
      None => break,
    }
  }
  result
}

/// 原版 FileUriDecode (utils.cc:95-123): 非 file:// 前缀或残缺 %XX 一律 None
#[must_use]
pub fn file_uri_decode(uri: &str) -> Option<String> {
  // 平台前缀 cfg 分派: WIN 分支仅 windows 目标编译 (非 windows 下不实例化)
  #[cfg(windows)]
  const PREFIX: &str = "file:///";
  #[cfg(not(windows))]
  const PREFIX: &str = "file://";

  let path = uri.strip_prefix(PREFIX)?;
  let bytes = path.as_bytes();
  let mut decoded = String::with_capacity(path.len());
  let mut i = 0;
  while i < bytes.len() {
    if bytes[i] != b'%' {
      decoded.push(bytes[i] as char);
      i += 1;
      continue;
    }
    // '%' 后不足两字符即失败
    if bytes.len() - i <= 2 {
      return None;
    }
    let first = bytes[i + 1];
    let second = bytes[i + 2];
    if !first.is_ascii_hexdigit() || !second.is_ascii_hexdigit() {
      return None;
    }
    let hex = |c: u8| (c as char).to_digit(16).unwrap_or(0);
    let v = hex(first) * 16 + hex(second);
    decoded.push(v as u8 as char);
    i += 3;
  }
  Some(decoded)
}

/// 原版 ContainsUnderscore (utils.cc:167-170)
fn contains_underscore(s: &str) -> bool {
  s.contains('_')
}

/// 原版 ContainsUpperCase (utils.cc:172-175)
fn contains_upper_case(s: &str) -> bool {
  s.chars().any(char::is_uppercase)
}

/// 原版 keyword-like 常量白名单 (utils.cc:180-188), 编译期完美散列
static KEYWORD_LIKE_CONSTANTS: phf::Set<&'static str> = phf::phf_set! {
  "True", "False", "TheHole", "TdzHole", "PromiseHole", "Null", "Undefined",
};

/// 原版 IsKeywordLikeName (utils.cc:180-188)
fn is_keyword_like_name(s: &str) -> bool {
  KEYWORD_LIKE_CONSTANTS.contains(s)
}

/// 原版机器类型白名单 (utils.cc:192-219, IsMachineType), 编译期完美散列
static MACHINE_TYPES: phf::Set<&'static str> = phf::phf_set! {
  "void", "never", "int8", "uint8", "int16", "uint16", "int31", "uint31",
  "int32", "uint32", "int64", "uint64", "intptr", "uintptr",
  "float16_raw_bits", "float32", "float64", "float64_or_undefined_or_hole",
  "bool", "string", "bint", "char8", "char16",
};

fn is_machine_type(s: &str) -> bool {
  MACHINE_TYPES.contains(s)
}

/// 原版 IsLowerCamelCase (utils.cc:223-228): 允许一个前导下划线, 余段无下划线
#[must_use]
pub fn is_lower_camel_case(s: &str) -> bool {
  let body = s.strip_prefix('_').unwrap_or(s);
  match body.chars().next() {
    Some(c) if c.is_lowercase() => !contains_underscore(body),
    _ => false,
  }
}

/// 原版 IsUpperCamelCase (utils.cc:230-235): 仅查剥前导下划线后的首字符
#[must_use]
pub fn is_upper_camel_case(s: &str) -> bool {
  let body = s.strip_prefix('_').unwrap_or(s);
  body.chars().next().is_some_and(char::is_uppercase)
}

/// 原版 IsSnakeCase (utils.cc:237-240): 无大写即 snake
#[must_use]
pub fn is_snake_case(s: &str) -> bool {
  !s.is_empty() && !contains_upper_case(s)
}

/// 原版 IsValidNamespaceConstName (utils.cc:242-247)
#[must_use]
pub fn is_valid_namespace_const_name(s: &str) -> bool {
  if s.is_empty() {
    return false;
  }
  if is_keyword_like_name(s) {
    return true;
  }
  s.starts_with('k') && is_upper_camel_case(&s[1..])
}

/// 原版 IsValidTypeName (utils.cc:249-254)
#[must_use]
pub fn is_valid_type_name(s: &str) -> bool {
  if s.is_empty() {
    return false;
  }
  is_machine_type(s) || is_upper_camel_case(s)
}

/// 原版 CapifyStringWithUnderscores (utils.cc:256-278)
#[must_use]
pub fn capify_string_with_underscores(s: &str) -> String {
  // 特例: "JSAbc" 断词为 "JS_ABC" (首个 "JS" 之后强制断词)
  let js_position = s.find("JS");
  let mut result = String::with_capacity(s.len() + 4);
  let mut previous_was_lower_or_digit = false;
  for (index, current) in s.char_indices() {
    if (previous_was_lower_or_digit && current.is_uppercase())
      || js_position.is_some_and(|js| index == js + "JS".len())
    {
      result.push('_');
    }
    if current == '.' || current == '-' {
      result.push('_');
      previous_was_lower_or_digit = false;
      continue;
    }
    for upper in current.to_uppercase() {
      result.push(upper);
    }
    previous_was_lower_or_digit = current.is_lowercase() || current.is_ascii_digit();
  }
  result
}

/// 原版 CamelifyString (utils.cc:280-295)
#[must_use]
pub fn camelify_string(s: &str) -> String {
  let mut result = String::with_capacity(s.len());
  let mut word_beginning = true;
  for mut current in s.chars() {
    if current == '_' || current == '-' {
      word_beginning = true;
      continue;
    }
    if word_beginning {
      current = current.to_ascii_uppercase();
    }
    result.push(current);
    word_beginning = false;
  }
  result
}

/// 原版 SnakeifyString (utils.cc:297-308)
#[must_use]
pub fn snakeify_string(s: &str) -> String {
  let mut result = String::with_capacity(s.len() + 4);
  let mut previous_was_lower = false;
  for current in s.chars() {
    if previous_was_lower && current.is_uppercase() {
      result.push('_');
    }
    result.push(current.to_ascii_lowercase());
    previous_was_lower = current.is_lowercase();
  }
  result
}

/// 原版 DashifyString (utils.cc:310-314)
#[must_use]
pub fn dashify_string(s: &str) -> String {
  s.replace('_', "-")
}

/// 原版 UnderlinifyPath (utils.cc:316-323)
#[must_use]
pub fn underlinify_path(path: &str) -> String {
  path
    .chars()
    .map(|c| match c {
      '-' | '/' | '\\' | '.' => '_',
      _ => c,
    })
    .collect::<String>()
    .to_uppercase()
}

/// 原版 StartsWithSingleUnderscore (utils.cc:325-327)
#[must_use]
pub fn starts_with_single_underscore(s: &str) -> bool {
  s.len() >= 2 && s.starts_with('_') && !s[1..].starts_with('_')
}

/// 原版 ReplaceFileContentsIfDifferent: 内容不同才覆写; 返回是否写入
pub fn replace_file_contents_if_different(file_path: &str, contents: &str) -> Result<bool> {
  let path = Path::new(file_path);
  let old = fs::read(path).ok();
  if old.as_deref() == Some(contents.as_bytes()) {
    return Ok(false);
  }
  fs::write(path, contents)?;
  Ok(true)
}

// 原版 kBaseNamespaceName / kTestNamespaceName
pub const K_BASE_NAMESPACE_NAME: &str = "base";
pub const K_TEST_NAMESPACE_NAME: &str = "test";

// ── 原版 src/torque/utils.cc ──
// 签名清单
// L24: std::string StringLiteralUnquote(const std::string& s)
// L55: std::string StringLiteralQuote(const std::string& s)
// L88: static int HexCharToInt(unsigned char c)
// L95: std::optional<std::string> FileUriDecode(std::string_view uri)
// L126: MessageBuilder::MessageBuilder(const std::string& message, TorqueMessage::Kind kind)
// L154: void MessageBuilder::Report() const
// L161: [[noreturn]] void MessageBuilder::Throw() const
// L167: bool ContainsUnderscore(std::string_view s)
// L172: bool ContainsUpperCase(const std::string& s)
// L180: bool IsKeywordLikeName(std::string_view s)
// L192: bool IsMachineType(const std::string& s)
// L223: bool IsLowerCamelCase(std::string_view s)
// L230: bool IsUpperCamelCase(std::string_view s)
// L237: bool IsSnakeCase(const std::string& s)
// L242: bool IsValidNamespaceConstName(std::string_view s)
// L249: bool IsValidTypeName(const std::string& s)
// L256: std::string CapifyStringWithUnderscores(const std::string& camellified_string)
// L280: std::string CamelifyString(const std::string& underscore_string)
// L297: std::string SnakeifyString(const std::string& camel_string)
// L310: std::string DashifyString(const std::string& underscore_string)
// L316: std::string UnderlinifyPath(std::string path)
// L325: bool StartsWithSingleUnderscore(const std::string& str)
// L329: void ReplaceFileContentsIfDifferent(const std::string& file_path, const std::string& contents)
// L349: IfDefScope::IfDefScope(std::ostream& os, std::string d) : os_(os), d_(std::move(d))
// L353: IfDefScope::~IfDefScope()
// L355: NamespaceScope::NamespaceScope(std::ostream& os, std::initializer_list<std::string> namespaces) : os_(os), d_(std::move(namespaces))
// L362: NamespaceScope::~NamespaceScope()
// L368: IncludeGuardScope::IncludeGuardScope(std::ostream& os, std::string file_name) : os_(os), d_("" + CapifyStringWithUnderscores(file_name) + "")
// L375: IncludeGuardScope::~IncludeGuardScope()
// L377: IncludeObjectMacrosScope::IncludeObjectMacrosScope(std::ostream& os) : os_(os)
// L381: IncludeObjectMacrosScope::~IncludeObjectMacrosScope()
// L385: size_t ResidueClass::AlignmentLog2() const
// L392: std::ostream& operator<<(std::ostream& os, const ResidueClass& a)
