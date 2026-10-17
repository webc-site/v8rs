// 由 tool/src/regexp/regexp-nodes.h.js 从 v8/src/regexp/regexp-nodes.h 生成; 修订就地改, 复跑覆盖
// 正则节点图枚举与预算常量 (枚举 1:1; 语义实现面跳过)

/// 定长循环排空策略 (DrainMode) (共 4 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DrainMode {
  /// kFull
  Full = 0,
  /// kRetryAtEntry
  RetryAtEntry = 1,
  /// kRestoreOnly
  RestoreOnly = 2,
  /// kOmit
  Omit = 3,
}

/// 原子循环续体类别 (AtomicLoopKind) (共 5 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AtomicLoopKind {
  /// kNone
  None = 0,
  /// kAtEnd
  AtEnd = 1,
  /// kTotal
  Total = 2,
  /// kBoundary
  Boundary = 3,
  /// kDisjoint
  Disjoint = 4,
}

/// 循环出口位置停留许可 (ParkedGrant) (共 4 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ParkedGrant {
  /// kNone
  None = 0,
  /// kParked
  Parked = 1,
  /// kParkedUniformPrefix
  ParkedUniformPrefix = 2,
  /// kParkedNonEmptyUniformPrefix
  ParkedNonEmptyUniformPrefix = 3,
}

/// 版本限制结果 (RegExpNode::LimitResult) (共 2 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LimitResult {
  /// DONE
  DONE = 0,
  /// CONTINUE
  CONTINUE = 1,
}

/// 延迟动作类别 (Trace::ActionType) (共 10 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ActionType {
  /// SET_REGISTER_FOR_LOOP
  SET_REGISTER_FOR_LOOP = 0,
  /// INCREMENT_REGISTER
  INCREMENT_REGISTER = 1,
  /// STORE_POSITION
  STORE_POSITION = 2,
  /// RESTORE_POSITION
  RESTORE_POSITION = 3,
  /// BEGIN_POSITIVE_SUBMATCH
  BEGIN_POSITIVE_SUBMATCH = 4,
  /// BEGIN_NEGATIVE_SUBMATCH
  BEGIN_NEGATIVE_SUBMATCH = 5,
  /// POSITIVE_SUBMATCH_SUCCESS
  POSITIVE_SUBMATCH_SUCCESS = 6,
  /// EMPTY_MATCH_CHECK
  EMPTY_MATCH_CHECK = 7,
  /// CLEAR_CAPTURES
  CLEAR_CAPTURES = 8,
  /// EATS_AT_LEAST
  EATS_AT_LEAST = 9,
}

/// 文本发射趟次 (TextNode::TextEmitPassType) (共 5 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TextEmitPassType {
  /// NON_LATIN1_MATCH
  NON_LATIN1_MATCH = 0,
  /// SIMPLE_CHARACTER_MATCH
  SIMPLE_CHARACTER_MATCH = 1,
  /// NON_LETTER_CHARACTER_MATCH
  NON_LETTER_CHARACTER_MATCH = 2,
  /// CASE_CHARACTER_MATCH
  CASE_CHARACTER_MATCH = 3,
  /// CHARACTER_CLASS_MATCH
  CHARACTER_CLASS_MATCH = 4,
}

/// 断言类别 (AssertionNode::AssertionType) (共 5 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AssertionType {
  /// AT_END
  AT_END = 0,
  /// AT_START
  AT_START = 1,
  /// AT_BOUNDARY
  AT_BOUNDARY = 2,
  /// AT_NON_BOUNDARY
  AT_NON_BOUNDARY = 3,
  /// AFTER_NEWLINE
  AFTER_NEWLINE = 4,
}

/// 回溯前提 (AssertionNode::IfPrevious) (共 2 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IfPrevious {
  /// kIsNonWord
  IsNonWord = 0,
  /// kIsWord
  IsWord = 1,
}

/// 终止节点动作 (EndNode::Action) (共 3 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Action {
  /// ACCEPT
  ACCEPT = 0,
  /// BACKTRACK
  BACKTRACK = 1,
  /// NEGATIVE_SUBMATCH_SUCCESS
  NEGATIVE_SUBMATCH_SUCCESS = 2,
}

/// 守卫比较关系 (Guard::Relation) (共 2 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Relation {
  /// LT
  LT = 0,
  /// GEQ
  GEQ = 1,
}

/// "吃入数"哨兵 (原版 RegExpNode::kLargeEatsAtLeastValue; 超此值走保守路径)
pub const LARGE_EATS_AT_LEAST_VALUE: u32 = 255;

/// 定长循环复杂度哨兵 (原版 kNodeIsTooComplexForFixedLengthLoops = kMinInt)
pub const NODE_IS_TOO_COMPLEX_FOR_FIXED_LENGTH_LOOPS: i32 = i32::MIN;

/// 递归预算 (原版 kRecursionBudget)
pub const RECURSION_BUDGET: i32 = 200;

/// 单节点代码生成拷贝上限 (原版 kMaxCopiesCodeGenerated; 达到即禁内联)
pub const MAX_COPIES_CODE_GENERATED: i32 = 10;

/// 首字符预算 (原版 Trace::kFirstCharBudget)
pub const FIRST_CHAR_BUDGET: i32 = 10;

/// 前瞻环视槽位索引 (原版 NegativeLookaroundChoiceNode::kLookaroundIndex)
pub const LOOKAROUND_INDEX: i32 = 0;

/// 续体槽位索引 (原版 kContinueIndex)
pub const CONTINUE_INDEX: i32 = 1;

// —— 提取不变式断言 ——
const _: () = {
  assert!(LARGE_EATS_AT_LEAST_VALUE == 255);
  assert!(RECURSION_BUDGET == 200);
  assert!(MAX_COPIES_CODE_GENERATED == 10);
  assert!(FIRST_CHAR_BUDGET == 10);
  assert!(LOOKAROUND_INDEX == 0 && CONTINUE_INDEX == 1);
  assert!(DrainMode::Full as u8 == 0);
  assert!(EndAction::Accept as u8 == 0);
};
