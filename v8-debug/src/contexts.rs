// 由 tool/src/debug/contexts.js 从 v8/src/objects/contexts.{cc,tq} 生成; 修订就地改, 复跑覆盖
// 席位表差集: ContextCell 布局与五态 + ContextMode + Context 子类型 (.h 槽位归既有 contexts.h.js)
//! Context 差集 (debug 域): ContextCell 上下文槽盒 (context cells proposal)。
//!
//! ContextCell 承载可优化槽位 (Smi/Int32/Float64 三态迁移 + 依赖代码去优化);
//! Context 槽位下标/偏移常量归既有 contexts.h.js, 本文件只登记 .tq/.cc 差集。

// ── ContextCell::State (原版 enum State : int32_t; tq generates 5 常量) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum ContextCellState {
  /// kConst
  Const = 0,
  /// kSmi
  Smi = 1,
  /// kInt32
  Int32 = 2,
  /// kFloat64
  Float64 = 3,
  /// kDetached
  Detached = 4,
}

impl ContextCellState {
  /// 状态总数
  pub const COUNT: usize = 5;
}

// ── ContextMode (tq extern enum ContextMode extends uint32) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ContextMode {
  /// kNoContextCells
  NoContextCells = 0,
  /// kHasContextCells
  HasContextCells = 1,
}

// ── ContextCell 布局 (tq extern class, 源码序; padding 仅 8B tagged 构建) ──
pub const CONTEXT_CELL_FIELDS: [(&str, &str, bool); 5] = [
  ("tagged_value", "JSAny", false),
  ("dependent_code", "DependentCode", false),
  ("state", "int32", false),
  ("optional_padding", "uint32", true),
  ("double_value", "float64", false),
];

// ── Context 子类型 (tq generates 'Context', 布局同 Context) ──
pub const CONTEXT_SUBCLASSES: [&str; 9] = [
  "AwaitContext",
  "BlockContext",
  "CatchContext",
  "DebugEvaluateContext",
  "EvalContext",
  "ModuleContext",
  "ScriptContext",
  "WithContext",
  "FunctionContext",
];

/// 新建槽位初始值 (原版 kInitialContextSlotValue)。
pub const K_INITIAL_CONTEXT_SLOT_VALUE: i32 = 0;

// ── .cc NativeContext 偏移 static_assert 面 (8 处, 语义锚) ──
//   MIN_CONTEXT_SLOTS == 2; MIN_CONTEXT_EXTENDED_SLOTS == 3;
//   kScopeInfoOffset/kPreviousOffset/kExtensionOffset == OffsetOfElementAt(对应下标);
//   kStartOfWeakFieldsOffset == OffsetOfElementAt(FIRST_WEAK_SLOT);
//   kMicrotaskQueueOffset == Context::SizeFor(NATIVE_CONTEXT_SLOTS);
//   kSize == SizeFor(NATIVE_CONTEXT_SLOTS) + kMicrotaskQueueSlotSize;
//   (数值依赖 NATIVE_CONTEXT_FIELDS 展开, 由 Developer 依宏展开落值)

// ── ContextCell 状态机 ──

/// 原版 `Tagged<JSAny>` 在 ContextCell `tagged_value` 槽的堆无关投影。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContextSlotValue {
  /// `undefined_value()` (空槽)
  Undefined,
  /// `the_hole()` / `tdz_hole()` 等 oddball
  Oddball(u32),
  /// Smi
  Smi(i32),
  /// HeapNumber (取 `value()`)
  HeapNumber(f64),
  /// 其他堆对象 (arena 索引)
  Object(u32),
}

/// 原版 `Tagged<DependentCode>` 的堆无关投影: 只区分 `empty` 与非空,
/// 非空时记录待去优化的依赖组 (`kContextCellChangedGroup`)。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DependentCode {
  /// `DependentCode::empty_dependent_code(ReadOnlyRoots)`
  Empty,
  /// 已有依赖链 (需通知去优化)
  Groups(u32),
}

/// 原版 `ContextCell` (contexts.h / contexts.tq, 见 `CONTEXT_CELL_FIELDS`)
/// 的堆无关投影, 字段序与布局表 1:1。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContextCell {
  /// tagged_value_ (`JSAny`)
  pub tagged_value: ContextSlotValue,
  /// dependent_code_ (`DependentCode`)
  pub dependent_code: DependentCode,
  /// state_ (`int32`)
  pub state: ContextCellState,
  /// optional_padding_ (`uint32`, 仅 8B tagged 构建存在)
  pub padding: u32,
  /// double_value_ (`float64`)
  pub double_value: f64,
}

impl ContextCell {
  /// v8: src/objects/contexts-inl.h:375 `void ContextCell::set_state(State state)`
  /// (写 state_ 的 int32 槽)
  pub fn set_state(&mut self, state: ContextCellState) {
    self.state = state;
  }

  /// v8: src/objects/contexts-inl.h:396 `void ContextCell::set_smi_value(Tagged<Smi>)`
  /// (untagged 槽清空, tagged 槽放 Smi)
  pub fn set_smi_value(&mut self, value: i32) {
    self.tagged_value = ContextSlotValue::Smi(value);
    self.padding = 0;
    self.double_value = 0.0;
  }

  /// v8: src/objects/contexts-inl.h:424 `void ContextCell::set_int32_value(int32_t)`
  /// (`TAGGED_SIZE_8_BYTES` 时 int32 落 double 槽, 32 位构建复用同一 8 字节槽)
  pub fn set_int32_value(&mut self, value: i32) {
    self.double_value = value as f64;
    self.padding = 0;
  }

  /// v8: src/objects/contexts-inl.h:415 `void ContextCell::set_float64_value(double)`
  pub fn set_float64_value(&mut self, value: f64) {
    self.double_value = value;
    self.padding = 0;
  }

  /// v8: src/objects/contexts-inl.h:400 `void ContextCell::clear_tagged_value()`
  pub fn clear_tagged_value(&mut self) {
    self.tagged_value = ContextSlotValue::Undefined;
  }

  /// v8: src/objects/contexts-inl.h:404 `void ContextCell::clear_padding()`
  pub fn clear_padding(&mut self) {
    self.padding = 0;
  }
}

/// 原版 `DoubleFitsInInt32` (contexts.cc:525, 匿名 namespace)。
/// v8: src/objects/contexts.cc:525 `std::optional<int32_t> DoubleFitsInInt32(double value)`
/// — 先排除 `-0.0` (`value == 0.0 && signbit`), 再要求整值且落在 int32 区间。
#[must_use]
pub fn double_fits_in_int32(value: f64) -> Option<i32> {
  const INT32_MIN: f64 = i32::MIN as f64;
  const INT32_MAX: f64 = i32::MAX as f64;
  // Check -0.0 first.
  if value == 0.0 && value.is_sign_negative() {
    return None;
  }
  let trunc_value = value.trunc();
  if (INT32_MIN..=INT32_MAX).contains(&value) && value == trunc_value {
    return Some(trunc_value as i32);
  }
  None
}

/// 原版 `IsEmptyDependentCode` (contexts.cc:537, 匿名 namespace)。
/// v8: src/objects/contexts.cc:537 `bool IsEmptyDependentCode(Tagged<DependentCode>, Isolate*)`
#[must_use]
pub fn is_empty_dependent_code(value: DependentCode) -> bool {
  matches!(value, DependentCode::Empty)
}

/// 原版 TransitionContextCellToUntagged (contexts.cc): HeapNumber → Int32/Float64 迁移。
/// v8: src/objects/contexts.cc:550 `void TransitionContextCellToUntagged(Tagged<HeapNumber> number, DirectHandle<ContextCell> cell)`
/// — 能落 int32 走 `set_int32_value` + `kInt32`, 否则 `set_float64_value` + `kFloat64`;
/// 两条路径都清空 tagged 槽 (原版 `set_*_value` 前置 `clear_tagged_value`)。
pub fn transition_context_cell_to_untagged(number: f64, cell: &mut ContextCell) {
  if let Some(int32_value) = double_fits_in_int32(number) {
    cell.clear_tagged_value();
    cell.set_int32_value(int32_value);
    cell.set_state(ContextCellState::Int32);
  } else {
    cell.clear_tagged_value();
    cell.set_float64_value(number);
    cell.set_state(ContextCellState::Float64);
  }
}

/// 原版 NotifyContextCellStateWillChange (contexts.cc): 通知依赖代码去优化。
/// v8: src/objects/contexts.cc:542 `void NotifyContextCellStateWillChange(DirectHandle<ContextCell> cell, Isolate* isolate)`
/// — 仅当 `dependent_code` 非 empty 才触发
/// `DependentCode::DeoptimizeDependencyGroups(kContextCellChangedGroup)`。
/// 去优化动作在 `deps` 位图返回, 由上层 (isolate 侧) 执行。
#[must_use]
pub fn notify_context_cell_state_will_change(cell: &ContextCell) -> Option<DependentCode> {
  if is_empty_dependent_code(cell.dependent_code) {
    return None;
  }
  Some(cell.dependent_code)
}

/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(ContextCellState::COUNT == 5);
  assert!(ContextCellState::Const as i32 == 0);
  assert!(ContextCellState::Detached as i32 == 4);
  assert!(CONTEXT_CELL_FIELDS.len() == 5);
  assert!(str_eq(CONTEXT_CELL_FIELDS[3].0, "optional_padding"));
  assert!(str_eq(CONTEXT_CELL_FIELDS[4].1, "float64"));
  assert!(CONTEXT_SUBCLASSES.len() == 9);
  assert!(K_INITIAL_CONTEXT_SLOT_VALUE == 0);
};
