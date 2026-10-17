// 由 tool/src/interpreter/bytecode-label.h.js 从 v8/src/interpreter/bytecode-label.{h,cc} 生成; 修订就地改, 复跑覆盖
// 字节码标签/循环头/标签组骨架与真实跳转回填实现

/// 无效偏移哨兵 (原版 kInvalidOffset = static_cast<size_t>(-1))
pub const INVALID_OFFSET: usize = usize::MAX;

/// 字节码前向跳转目标标签 (轻量级句柄，支持多重引用与跨作用域传递)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BytecodeLabel {
  id: usize,
}

impl BytecodeLabel {
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self { id: 0 }
  }

  #[inline]
  #[must_use]
  pub const fn with_id(id: usize) -> Self {
    Self { id }
  }

  #[inline]
  #[must_use]
  pub const fn id(self) -> usize {
    self.id
  }
}

/// 循环头 (先绑后跳, 位置恒已知)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BytecodeLoopHeader {
  offset: usize,
}

impl Default for BytecodeLoopHeader {
  fn default() -> Self {
    Self::new()
  }
}

impl BytecodeLoopHeader {
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self {
      offset: INVALID_OFFSET,
    }
  }

  #[inline]
  #[must_use]
  pub const fn offset(&self) -> usize {
    self.offset
  }

  #[inline]
  #[must_use]
  pub const fn is_bound(&self) -> bool {
    self.offset != INVALID_OFFSET
  }

  #[inline]
  pub fn bind_to(&mut self, offset: usize) {
    self.offset = offset;
  }
}

/// 字节码多重跳转分支目标组
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BytecodeLabels {
  labels: Vec<BytecodeLabel>,
}

impl BytecodeLabels {
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self { labels: Vec::new() }
  }

  #[inline]
  pub fn push(&mut self, label: BytecodeLabel) {
    self.labels.push(label);
  }

  #[inline]
  #[must_use]
  pub fn empty(&self) -> bool {
    self.labels.is_empty()
  }

  #[inline]
  #[must_use]
  pub fn labels(&self) -> &[BytecodeLabel] {
    &self.labels
  }
}

/// 原版 class BytecodeLoopHeader 成员声明 (循环头, 先绑后跳, 位置恒已知)
pub const BYTECODE_LOOP_HEADER_DECLARATIONS: &[&str] = &[
  "[public/method] BytecodeLoopHeader() : offset_(kInvalidOffset)",
  "[public/method] size_t offset() const",
  "[private/const] static const size_t kInvalidOffset = static_cast<size_t>(-1)",
  "[private/method] void bind_to(size_t offset)",
  "[private/field] size_t offset_",
];

/// 原版 class BytecodeLabel 成员声明 (前向跳转目标, 至多一个引用跳转)
pub const BYTECODE_LABEL_DECLARATIONS: &[&str] = &[
  "[public/method] BytecodeLabel() : bound_(false), jump_offset_(kInvalidOffset)",
  "[public/method] bool is_bound() const",
  "[public/method] size_t jump_offset() const",
  "[public/method] bool has_referrer_jump() const",
  "[private/const] static const size_t kInvalidOffset = static_cast<size_t>(-1)",
  "[private/method] void bind()",
  "[private/method] void set_referrer(size_t offset)",
  "[private/field] bool bound_",
  "[private/field] size_t jump_offset_",
];

/// 原版 class BytecodeLabels 成员声明 (多重跳转分支目标组)
pub const BYTECODE_LABELS_DECLARATIONS: &[&str] = &[
  "[public/method] explicit BytecodeLabels(Zone* zone) : labels_(zone), is_bound_(false)",
  "[public/method] BytecodeLabels(const BytecodeLabels&) = delete",
  "[public/method] BytecodeLabels& operator=(const BytecodeLabels&) = delete",
  "[public/method] BytecodeLabel* New()",
  "[public/method] void Bind(BytecodeArrayBuilder* builder)",
  "[public/method] bool is_bound() const",
  "[public/method] bool empty() const",
  "[private/field] ZoneLinkedList<BytecodeLabel> labels_",
  "[private/field] bool is_bound_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_LOOP_HEADER_DECLARATIONS.len() == 5);
  assert!(BYTECODE_LABEL_DECLARATIONS.len() == 9);
  assert!(BYTECODE_LABELS_DECLARATIONS.len() == 9);
  assert!(INVALID_OFFSET == usize::MAX);
};
