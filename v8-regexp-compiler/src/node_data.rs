//! v8/src/regexp 节点/Trace 数据面 1:1 转写 (wave1 无指针子集)
//!
//! 来源:
//! - `FOR_EACH_NODE_TYPE` X-macro (regexp-nodes.h:34-44) → 节点访问表 9 项。
//!   票面 "RegExpNodeVisitor" 即此表驱动的 Visitor 双分派 (Visitor 基类在
//!   regexp-nodes.h:1065 DECLARE_VISIT; 虚函数宿主面待二梯队)。
//! - `NodeInfo` (regexp-nodes.h:49-107) — 位间兴趣传播, 纯 bool 语义。
//! - `EatsAtLeastInfo` (regexp-nodes.h:110-139)。
//! - `PreloadState` (regexp-compiler.h:530-538)。
//! - `ParkedGrant` (regexp-nodes.h:262-289)。
//! - `Trace` 数据面 (regexp-compiler.h:304-505): TriBool/FlushMode/
//!   DeferredActionUndoType/RegisterFlushInfo/位域组/CP 偏移界限。
//!   Flush/AdvanceCurrentPositionInTrace/ScanDeferredActions 触
//!   RegExpMacroAssembler 与 ActionNode 链表宿主 → 推迟二梯队; 本模块仅落
//!   flags_ 位域 + 纯计数/布尔字段 + is_trivial 判定。

use v8_base::bit_field::{BitField32, BitFieldCodec};

use crate::quick_check::QuickCheckDetails;

// ---------------------------------------------------------------------------
// 节点访问表 (FOR_EACH_NODE_TYPE)
// ---------------------------------------------------------------------------

/// FOR_EACH_NODE_TYPE 的 9 项 (regexp-nodes.h:34-44), 序即 X-macro 展开序。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType {
  End,
  Action,
  Choice,
  LoopChoice,
  NegativeLookaroundChoice,
  BackReference,
  Assertion,
  Text,
  UnanchoredAdvance,
}

/// 金标准: 条目名表 (测试断言 len == regexp-nodes.h 中 VISIT( 计数)。
pub const NODE_TYPE_NAMES: [&str; NodeType::COUNT] = [
  "End",
  "Action",
  "Choice",
  "LoopChoice",
  "NegativeLookaroundChoice",
  "BackReference",
  "Assertion",
  "Text",
  "UnanchoredAdvance",
];

impl NodeType {
  /// 原版 arraysize(FOR_EACH_NODE_TYPE) = 9。
  pub const COUNT: usize = 9;

  /// C++ 类名拼接 `class type##Node` (regexp-nodes.h:45-47) → "EndNode" 等。
  pub const fn node_class_name(self) -> &'static str {
    match self {
      Self::End => "EndNode",
      Self::Action => "ActionNode",
      Self::Choice => "ChoiceNode",
      Self::LoopChoice => "LoopChoiceNode",
      Self::NegativeLookaroundChoice => "NegativeLookaroundChoiceNode",
      Self::BackReference => "BackReferenceNode",
      Self::Assertion => "AssertionNode",
      Self::Text => "TextNode",
      Self::UnanchoredAdvance => "UnanchoredAdvanceNode",
    }
  }

  pub const fn table_name(self) -> &'static str {
    NODE_TYPE_NAMES[self as usize]
  }
}

// ---------------------------------------------------------------------------
// NodeInfo
// ---------------------------------------------------------------------------

/// NodeInfo (regexp-nodes.h:49-107)。
///
/// 原版 8 个 `bool x : 1` 位段; Rust 以 bool 字段直存 (语义同, 位段仅是
/// 存储压缩, 无可观测行为差异)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeInfo {
  /// bool being_analyzed : 1
  pub being_analyzed: bool,
  /// bool been_analyzed : 1
  pub been_analyzed: bool,
  /// bool follows_word_interest : 1
  pub follows_word_interest: bool,
  /// bool follows_newline_interest : 1
  pub follows_newline_interest: bool,
  /// bool follows_start_interest : 1
  pub follows_start_interest: bool,
  /// bool at_end : 1
  pub at_end: bool,
  /// bool visited : 1
  pub visited: bool,
  /// bool replacement_calculated : 1
  pub replacement_calculated: bool,
}

impl NodeInfo {
  /// NodeInfo() 构造: 8 位全 false (regexp-nodes.h:50-59)。
  pub const fn new() -> Self {
    Self {
      being_analyzed: false,
      been_analyzed: false,
      follows_word_interest: false,
      follows_newline_interest: false,
      follows_start_interest: false,
      at_end: false,
      visited: false,
      replacement_calculated: false,
    }
  }

  /// bool Matches(NodeInfo* that) (regexp-nodes.h:63-68)。
  /// 仅比较 at_end 与三个 follows_* 兴趣位。
  pub const fn matches(&self, that: &Self) -> bool {
    self.at_end == that.at_end
      && self.follows_word_interest == that.follows_word_interest
      && self.follows_newline_interest == that.follows_newline_interest
      && self.follows_start_interest == that.follows_start_interest
  }

  /// void AddFromPreceding(NodeInfo* that) (regexp-nodes.h:71-76)。
  pub fn add_from_preceding(&mut self, that: &Self) {
    self.at_end |= that.at_end;
    self.follows_word_interest |= that.follows_word_interest;
    self.follows_newline_interest |= that.follows_newline_interest;
    self.follows_start_interest |= that.follows_start_interest;
  }

  /// bool HasLookbehind() (regexp-nodes.h:78-80)。
  pub const fn has_lookbehind(&self) -> bool {
    self.follows_word_interest || self.follows_newline_interest || self.follows_start_interest
  }

  /// void AddFromFollowing(NodeInfo* that) (regexp-nodes.h:83-87)。
  /// 注意: 不含 at_end (与 AddFromPreceding 相区别)。
  pub fn add_from_following(&mut self, that: &Self) {
    self.follows_word_interest |= that.follows_word_interest;
    self.follows_newline_interest |= that.follows_newline_interest;
    self.follows_start_interest |= that.follows_start_interest;
  }

  /// void ResetCompilationState() (regexp-nodes.h:89-92)。
  pub fn reset_compilation_state(&mut self) {
    self.being_analyzed = false;
    self.been_analyzed = false;
  }
}

impl Default for NodeInfo {
  fn default() -> Self {
    Self::new()
  }
}

// ---------------------------------------------------------------------------
// EatsAtLeastInfo / PreloadState
// ---------------------------------------------------------------------------

/// EatsAtLeastInfo (regexp-nodes.h:110-139): 本节点起成功匹配最少吃入字符数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EatsAtLeastInfo {
  /// uint8_t from_possibly_start
  pub from_possibly_start: u8,
  /// uint8_t from_not_start
  pub from_not_start: u8,
}

impl EatsAtLeastInfo {
  /// EatsAtLeastInfo() = EatsAtLeastInfo(0)。
  pub const fn new() -> Self {
    Self {
      from_possibly_start: 0,
      from_not_start: 0,
    }
  }
  /// explicit EatsAtLeastInfo(uint8_t eats)。
  pub const fn with_eats(eats: u8) -> Self {
    Self {
      from_possibly_start: eats,
      from_not_start: eats,
    }
  }
  /// void SetMin(const EatsAtLeastInfo&)。
  pub fn set_min(&mut self, other: &Self) {
    self.from_possibly_start = self.from_possibly_start.min(other.from_possibly_start);
    self.from_not_start = self.from_not_start.min(other.from_not_start);
  }
  /// void SetMax(int other): 原版 saturated_cast<uint8_t> 饱和收宽。
  pub fn set_max(&mut self, other: i32) {
    let max = other.clamp(0, u8::MAX as i32) as u8;
    self.from_possibly_start = self.from_possibly_start.max(max);
    self.from_not_start = self.from_not_start.max(max);
  }
  /// bool IsZero()。
  pub const fn is_zero(&self) -> bool {
    self.from_possibly_start == 0 && self.from_not_start == 0
  }
}

impl Default for EatsAtLeastInfo {
  fn default() -> Self {
    Self::new()
  }
}

/// PreloadState (regexp-compiler.h:530-538)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreloadState {
  /// bool preload_is_current_
  pub preload_is_current: bool,
  /// bool preload_has_checked_bounds_
  pub preload_has_checked_bounds: bool,
  /// int preload_characters_
  pub preload_characters: i32,
  /// int eats_at_least_
  pub eats_at_least: i32,
}

impl PreloadState {
  /// kEatsAtLeastNotYetInitialized = -1 (regexp-compiler.h:531)。
  pub const K_EATS_AT_LEAST_NOT_YET_INITIALIZED: i32 = -1;

  /// void init(): eats_at_least_ = kEatsAtLeastNotYetInitialized。
  pub fn init(&mut self) {
    self.eats_at_least = Self::K_EATS_AT_LEAST_NOT_YET_INITIALIZED;
  }
}

impl Default for PreloadState {
  /// POD 成员无显式默认构造 (原版由外层 memset/赋初), 以零值 + init() 流程承接。
  fn default() -> Self {
    Self {
      preload_is_current: false,
      preload_has_checked_bounds: false,
      preload_characters: 0,
      eats_at_least: 0,
    }
  }
}

// ---------------------------------------------------------------------------
// ParkedGrant
// ---------------------------------------------------------------------------

/// ParkedGrant (regexp-nodes.h:262-289): 环路出口回溯目标对"停在贪尽位置"
/// 的容忍等级, 由弱到强排序 (derive 序即语义序, "门接受 ≥ 所需等级")。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ParkedGrant {
  /// 无许可 (kNone = 0)
  None = 0,
  /// 允许停在贪尽处 (隐式搜索循环 advance-and-retry 目标)
  Parked = 1,
  /// 同 kParked, 且有强制统一前缀 (如 /\s+$/ 的前导 \s)
  ParkedUniformPrefix = 2,
  /// 同 kParkedUniformPrefix, 且前缀末字符确已消费 (Flush 已物化 pending advance)
  ParkedNonEmptyUniformPrefix = 3,
}

impl ParkedGrant {
  pub const K_NONE: Self = Self::None;
  pub const K_PARKED: Self = Self::Parked;
  pub const K_PARKED_UNIFORM_PREFIX: Self = Self::ParkedUniformPrefix;
  pub const K_PARKED_NON_EMPTY_UNIFORM_PREFIX: Self = Self::ParkedNonEmptyUniformPrefix;
}

impl BitFieldCodec for ParkedGrant {
  const TRUSTED_TYPE: bool = true; // C++ enum → is_enum_v 可信分支
  #[inline]
  fn to_u64(self) -> u64 {
    self as u64
  }
  #[inline]
  fn from_u64(v: u64) -> Self {
    match v {
      0 => Self::None,
      1 => Self::Parked,
      2 => Self::ParkedUniformPrefix,
      3 => Self::ParkedNonEmptyUniformPrefix,
      // 原版 DCHECK 值域; 越界编码 debug 断言后按 None 兜底 (不可达路径)。
      other => {
        debug_assert!(false, "ParkedGrant 越界编码: {other}");
        Self::None
      }
    }
  }
}

// ---------------------------------------------------------------------------
// Trace 数据面
// ---------------------------------------------------------------------------

/// Trace::TriBool (regexp-compiler.h:308)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TriBool {
  FalseValue = 0,
  TrueValue = 1,
  Unknown = 2,
}

impl TriBool {
  pub const FALSE_VALUE: Self = Self::FalseValue;
  pub const TRUE_VALUE: Self = Self::TrueValue;
  pub const UNKNOWN: Self = Self::Unknown;
}

impl BitFieldCodec for TriBool {
  const TRUSTED_TYPE: bool = true;
  #[inline]
  fn to_u64(self) -> u64 {
    self as u64
  }
  #[inline]
  fn from_u64(v: u64) -> Self {
    match v {
      0 => Self::FalseValue,
      1 => Self::TrueValue,
      2 => Self::Unknown,
      other => {
        debug_assert!(false, "TriBool 越界编码: {other}");
        Self::Unknown
      }
    }
  }
}

/// Trace::FlushMode (regexp-compiler.h:339-345)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushMode {
  /// 常规 flush: 生成回溯代码。
  FlushFull,
  /// 匹配已成功: 当前位置与回溯栈无需写回。
  FlushSuccess,
}

/// Trace::DeferredActionUndoType (regexp-compiler.h:465, private)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeferredActionUndoType {
  Ignore = 0,
  Restore = 1,
  Clear = 2,
}

impl DeferredActionUndoType {
  pub const IGNORE: Self = Self::Ignore;
  pub const RESTORE: Self = Self::Restore;
  pub const CLEAR: Self = Self::Clear;
}

/// Trace::kNoStore = kMinInt (regexp-compiler.h:466)。
pub const K_NO_STORE: i32 = i32::MIN;

/// Trace::RegisterFlushInfo (regexp-compiler.h:469-476)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterFlushInfo {
  /// DeferredActionUndoType undo_action = IGNORE
  pub undo_action: DeferredActionUndoType,
  /// int value = 0
  pub value: i32,
  /// bool absolute — 直接置寄存器为 value
  pub absolute: bool,
  /// bool clear — 清零寄存器
  pub clear: bool,
  /// int store_position = kNoStore — 存"当前位置+value"到寄存器
  pub store_position: i32,
}

impl Default for RegisterFlushInfo {
  fn default() -> Self {
    Self {
      undo_action: DeferredActionUndoType::Ignore,
      value: 0,
      absolute: false,
      clear: false,
      store_position: K_NO_STORE,
    }
  }
}

/// Trace::AtStartField = base::BitField<TriBool, 0, 2> (regexp-compiler.h:489)。
pub type AtStartField = BitField32<TriBool, 0, 2>;
/// Trace::HasAnyActionsField = AtStartField::Next<bool, 1> → BitField<bool, 2, 1>
/// (regexp-compiler.h:491)。
pub type HasAnyActionsField = BitField32<bool, 2, 1>;
/// Trace::ParkedGrantField = HasAnyActionsField::Next<ParkedGrant, 2>
/// → BitField<ParkedGrant, 3, 2> (regexp-compiler.h:493)。
pub type ParkedGrantField = BitField32<ParkedGrant, 3, 2>;

/// RegExpMacroAssembler::kMaxCPOffset (regexp-macro-assembler.h:42),
/// Trace::AdvanceCurrentPositionInTrace 的收口界限。
pub const K_MAX_CP_OFFSET: i32 = (1 << 15) - 1;
/// RegExpMacroAssembler::kMinCPOffset = -kMaxCPOffset (regexp-macro-assembler.h:43)。
pub const K_MIN_CP_OFFSET: i32 = -K_MAX_CP_OFFSET;

/// Trace 数据面无指针子集 (regexp-compiler.h:304-505)。
///
/// ActionNode*/Label*/SpecialLoopState* 三个裸指针以 bool 存在位承接
/// (仅 is_trivial/set_backtrack 判定使用, 宿主链表遍历面推迟二梯队)。
#[derive(Debug, Clone)]
pub struct Trace {
  /// int cp_offset_
  cp_offset: i32,
  /// uint16_t flush_budget_ = 100 (构造注释: this is a 16 bit field)
  flush_budget: u16,
  /// uint32_t flags_ (三字段位域)
  flags: u32,
  /// ActionNode* action_ 的存在位
  has_action: bool,
  /// Label* backtrack_ 的存在位
  has_backtrack: bool,
  /// SpecialLoopState* special_loop_state_ 的存在位
  has_special_loop_state: bool,
  /// int characters_preloaded_
  characters_preloaded: i32,
  /// int bound_checked_up_to_
  bound_checked_up_to: i32,
  /// QuickCheckDetails quick_check_performed_ (按值成员, 1:1)
  quick_check_performed: QuickCheckDetails,
}

impl Trace {
  /// Trace::kCPOffsetSlack = 1 (regexp-compiler.h:356): 调用方在
  /// kMinCPOffset/kMaxCPOffset 上再做的安全余量 (cp_offset += ±1 仍须可判)。
  pub const K_CP_OFFSET_SLACK: i32 = 1;

  /// Trace() 构造 (regexp-compiler.h:310-321)。
  pub fn new() -> Self {
    Self {
      cp_offset: 0,
      flush_budget: 100, // Note: this is a 16 bit field.
      flags: AtStartField::encode(TriBool::UNKNOWN)
        | HasAnyActionsField::encode(false)
        | ParkedGrantField::encode(ParkedGrant::None),
      has_action: false,
      has_backtrack: false,
      has_special_loop_state: false,
      characters_preloaded: 0,
      bound_checked_up_to: 0,
      quick_check_performed: QuickCheckDetails::new(),
    }
  }

  /// 拷贝构造 Trace(const Trace&) (regexp-compiler.h:323-332) 数据面承接:
  /// 原版 action_ 置 nullptr 而其余按值复制 (含 quick_check_performed_ 与
  /// next_ = &other); Rust 无裸指针链, next_ 链宿主面推迟。
  pub fn clone_for_copy(other: &Self) -> Self {
    Self {
      cp_offset: other.cp_offset,
      flush_budget: other.flush_budget,
      flags: other.flags,
      has_action: false,
      has_backtrack: other.has_backtrack,
      has_special_loop_state: other.has_special_loop_state,
      characters_preloaded: other.characters_preloaded,
      bound_checked_up_to: other.bound_checked_up_to,
      quick_check_performed: other.quick_check_performed,
    }
  }

  /// int cp_offset() const (regexp-compiler.h:357)。
  pub const fn cp_offset(&self) -> i32 {
    self.cp_offset
  }

  /// bool has_any_actions() const — 链上任意 trace 有 action (链宿主以本
  /// flags 位承接) (regexp-compiler.h:360)。
  pub fn has_any_actions(&self) -> bool {
    HasAnyActionsField::decode(self.flags)
  }

  /// bool has_action() const — 本 trace 是否有 action (regexp-compiler.h:362)。
  pub const fn has_action(&self) -> bool {
    self.has_action
  }

  /// bool is_trivial() const (regexp-compiler.h:374-378)。
  pub fn is_trivial(&self) -> bool {
    !self.has_backtrack
      && !self.has_any_actions()
      && self.cp_offset == 0
      && self.characters_preloaded == 0
      && self.bound_checked_up_to == 0
      && self.quick_check_performed.characters == 0
      && self.at_start() == TriBool::UNKNOWN
  }

  /// TriBool at_start() const / void set_at_start(TriBool) (regexp-compiler.h:379-382)。
  pub fn at_start(&self) -> TriBool {
    AtStartField::decode(self.flags)
  }
  pub fn set_at_start(&mut self, at_start: TriBool) {
    self.flags = AtStartField::update(self.flags, at_start);
  }

  /// ParkedGrant parked_grant() const (regexp-compiler.h:396)。
  pub fn parked_grant(&self) -> ParkedGrant {
    ParkedGrantField::decode(self.flags)
  }

  /// int characters_preloaded() / bound_checked_up_to() / flush_budget()
  /// (regexp-compiler.h:397-400)。
  pub const fn characters_preloaded(&self) -> i32 {
    self.characters_preloaded
  }
  pub const fn bound_checked_up_to(&self) -> i32 {
    self.bound_checked_up_to
  }
  pub const fn flush_budget(&self) -> u16 {
    self.flush_budget
  }

  /// QuickCheckDetails* quick_check_performed() (regexp-compiler.h:401)。
  pub const fn quick_check_performed(&self) -> &QuickCheckDetails {
    &self.quick_check_performed
  }
  pub fn quick_check_performed_mut(&mut self) -> &mut QuickCheckDetails {
    &mut self.quick_check_performed
  }

  /// void add_action(ActionNode*) (regexp-compiler.cc:466-475) 数据面承接:
  /// DCHECK(action_ == nullptr) → debug_assert; HasAnyActions 置位;
  /// new_action->stores_backreferenced_capture() → 入参布尔 (宿主面解耦);
  /// 置位时 reset_parked_grant。
  pub fn add_action(&mut self, stores_backreferenced_capture: bool) {
    debug_assert!(!self.has_action); // Otherwise we lose an action.
    self.has_action = true;
    self.flags = HasAnyActionsField::update(self.flags, true);
    if stores_backreferenced_capture {
      self.reset_parked_grant();
    }
  }

  /// void reset_parked_grant() (regexp-compiler.h:412-414)。
  pub fn reset_parked_grant(&mut self) {
    self.flags = ParkedGrantField::update(self.flags, ParkedGrant::None);
  }

  /// void set_backtrack(Label*) (regexp-compiler.h:415-420):
  /// 设回溯目标并自动撤销 parked grant。
  pub fn set_backtrack(&mut self, has_backtrack: bool) {
    self.has_backtrack = has_backtrack;
    self.reset_parked_grant();
  }

  /// void set_parked_grant(ParkedGrant) (regexp-compiler.h:421-425):
  /// DCHECK_NOT_NULL(backtrack_) && DCHECK_NE(grant, kNone) → debug_assert。
  pub fn set_parked_grant(&mut self, grant: ParkedGrant) {
    debug_assert!(self.has_backtrack);
    debug_assert!(grant != ParkedGrant::None);
    self.flags = ParkedGrantField::update(self.flags, grant);
  }

  /// set_special_loop_state / set_characters_preloaded /
  /// set_bound_checked_up_to (regexp-compiler.h:426-430)。
  pub fn set_special_loop_state(&mut self, state: bool) {
    self.has_special_loop_state = state;
  }
  pub fn set_characters_preloaded(&mut self, count: i32) {
    self.characters_preloaded = count;
  }
  pub fn set_bound_checked_up_to(&mut self, to: i32) {
    self.bound_checked_up_to = to;
  }

  /// void set_flush_budget(int) (regexp-compiler.h:431-434):
  /// DCHECK(to <= UINT16_MAX) → debug_assert。
  pub fn set_flush_budget(&mut self, to: i32) {
    debug_assert!(to <= u16::MAX as i32); // Flush-budget is 16 bit.
    self.flush_budget = to as u16;
  }

  /// void set_quick_check_performed(QuickCheckDetails*) (regexp-compiler.h:435-437,
  /// 原版按值拷贝)。
  pub fn set_quick_check_performed(&mut self, d: &QuickCheckDetails) {
    self.quick_check_performed = *d;
  }

  /// Trace::AdvanceCurrentPositionInTrace 的界限判定子集 (regexp-compiler.cc
  /// 对应函数在原版承担 abort + InvalidateCurrentCharacter, 均触 Compiler/
  /// MacroAssembler → 推迟; 本函数只回答"±1 步进后 cp_offset 是否仍在
  /// [kMinCPOffset, kMaxCPOffset] 且留 slack")。
  pub fn can_advance_cp_offset(&self, by: i32) -> bool {
    let slack = Self::K_CP_OFFSET_SLACK;
    matches!(by, -1 | 1)
      && self.cp_offset.saturating_add(by) >= K_MIN_CP_OFFSET + slack
      && self.cp_offset.saturating_add(by) <= K_MAX_CP_OFFSET - slack
  }

  /// flags_ 原始值 (金标准对数: 默认 flags == AtStart=UNKNOWN 编码 == 2)。
  pub const fn flags_raw(&self) -> u32 {
    self.flags
  }
}

impl Default for Trace {
  fn default() -> Self {
    Self::new()
  }
}
