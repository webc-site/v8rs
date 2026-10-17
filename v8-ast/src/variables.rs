//! AST 变量模型 (v8/src/ast/variables.h/.cc)

use v8_common::{
  InitializationFlag, IsStaticFlag, MaybeAssignedFlag, VariableKind, VariableLocation, VariableMode,
};

pub use crate::variables_generated::{
  HOLE_CHECK_BITMAP_BITS, HoleCheckBitmap, HoleCheckState, UNCACHEABLE_HOLE_CHECK_BITMAP_INDEX,
  force_hole_init,
};
use crate::{scopes::ScopeId, value::AstRawString};

/// AST 变量 (原版 Variable)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
  /// 归属作用域 (原版 scope_ 指针 → arena 句柄)
  scope: ScopeId,
  name: AstRawString,
  mode: VariableMode,
  kind: VariableKind,
  initialization_flag: InitializationFlag,
  maybe_assigned_flag: MaybeAssignedFlag,
  is_static_flag: IsStaticFlag,
  location: VariableLocation,
  index: i32,
  is_used: bool,
  /// 强制 context 分配位 (原版 force_context_allocation 位)
  forced_context_allocation: bool,
  /// 空洞检测状态 (原版 hole_check_analysis_bit_field_ 低 2 位)
  hole_check_state: HoleCheckState,
  /// 空洞初始化强制位标志 (原版 ForceHoleInitializationFlag)
  force_hole_initialization: u8,
  /// 空洞检测位图索引 (0 = 不可缓存)
  hole_check_bitmap_index: u8,
}

/// 哨兵作用域句柄 (无归属树时的占位; 由 ScopeTree 分配覆盖)
const SCOPE_SENTINEL: ScopeId = ScopeId(u32::MAX);

impl Variable {
  pub fn new(
    name: AstRawString,
    mode: VariableMode,
    kind: VariableKind,
    initialization_flag: InitializationFlag,
    maybe_assigned_flag: MaybeAssignedFlag,
    is_static_flag: IsStaticFlag,
  ) -> Self {
    Self {
      scope: SCOPE_SENTINEL,
      name,
      mode,
      kind,
      initialization_flag,
      maybe_assigned_flag,
      is_static_flag,
      location: VariableLocation::Unallocated,
      index: -1,
      is_used: false,
      forced_context_allocation: false,
      hole_check_state: HoleCheckState::Uncached,
      force_hole_initialization: force_hole_init::HOLE_INITIALIZATION_NOT_FORCED,
      hole_check_bitmap_index: UNCACHEABLE_HOLE_CHECK_BITMAP_INDEX,
    }
  }

  /// 归属作用域 (原版 scope())
  pub fn scope(&self) -> ScopeId {
    self.scope
  }

  /// 设置归属作用域 (原版构造期挂接; 仅 ScopeTree 分配路径调用)
  pub(super) fn set_scope(&mut self, scope: ScopeId) {
    self.scope = scope;
  }

  pub fn name(&self) -> &AstRawString {
    &self.name
  }

  pub fn mode(&self) -> VariableMode {
    self.mode
  }

  pub fn set_mode(&mut self, mode: VariableMode) {
    self.mode = mode;
  }

  pub fn kind(&self) -> VariableKind {
    self.kind
  }

  pub fn initialization_flag(&self) -> InitializationFlag {
    self.initialization_flag
  }

  pub fn maybe_assigned_flag(&self) -> MaybeAssignedFlag {
    self.maybe_assigned_flag
  }

  /// 标记可能被赋值 (原版 SetMaybeAssigned)
  pub fn set_maybe_assigned(&mut self) {
    self.maybe_assigned_flag = MaybeAssignedFlag::MaybeAssigned;
  }

  pub fn is_static(&self) -> bool {
    matches!(self.is_static_flag, IsStaticFlag::Static)
  }

  pub fn location(&self) -> VariableLocation {
    self.location
  }

  pub fn set_location(&mut self, loc: VariableLocation) {
    self.location = loc;
  }

  pub fn index(&self) -> i32 {
    self.index
  }

  pub fn set_index(&mut self, index: i32) {
    self.index = index;
  }

  /// 统一分配到指定位置 (原版 AllocateTo: 同槽跳过, 越低位置优先)
  pub fn allocate_to(&mut self, location: VariableLocation, index: i32) {
    // 原版 DCHECK(index < 0 || index >= 0) 恒真; 同位置幂等
    if self.location == location && self.index == index {
      return;
    }
    self.location = location;
    self.index = index;
  }

  /// 是否为动态模式 (原版 is_dynamic)
  pub fn is_dynamic(&self) -> bool {
    self.mode.is_dynamic()
  }

  /// 强制 context 分配 (原版 ForceContextAllocation)
  pub fn force_context_allocation(&mut self) {
    self.forced_context_allocation = true;
  }

  /// 是否已强制 context 分配 (原版 has_forced_context_allocation)
  pub fn has_forced_context_allocation(&self) -> bool {
    self.forced_context_allocation
  }

  pub fn is_used(&self) -> bool {
    self.is_used
  }

  pub fn set_is_used(&mut self) {
    self.is_used = true;
  }

  pub fn is_unallocated(&self) -> bool {
    matches!(self.location, VariableLocation::Unallocated)
  }

  pub fn is_parameter(&self) -> bool {
    matches!(self.location, VariableLocation::Parameter)
  }

  pub fn is_stack_local(&self) -> bool {
    matches!(self.location, VariableLocation::Local)
  }

  pub fn is_context_slot(&self) -> bool {
    matches!(self.location, VariableLocation::Context)
  }

  pub fn is_lookup_slot(&self) -> bool {
    matches!(self.location, VariableLocation::Lookup)
  }

  // ── 空洞检测分析 (原版 hole_check_analysis_bit_field_ 族) ──

  /// 空洞检测状态
  pub fn hole_check_state(&self) -> HoleCheckState {
    self.hole_check_state
  }

  /// 设置空洞检测状态
  pub fn set_hole_check_state(&mut self, state: HoleCheckState) {
    self.hole_check_state = state;
  }

  /// 是否已强制空洞初始化
  pub fn is_hole_initialization_forced(&self) -> bool {
    self.force_hole_initialization != force_hole_init::HOLE_INITIALIZATION_NOT_FORCED
  }

  /// 是否存在同闭包作用域的空洞检测使用
  pub fn has_hole_check_use_in_same_closure_scope(&self) -> bool {
    self.force_hole_initialization & force_hole_init::HAS_HOLE_CHECK_USE_IN_SAME_CLOSURE_SCOPE != 0
  }

  /// 强制空洞初始化 (原版 ForceHoleInitialization, 按位累积)
  pub fn force_hole_initialization(&mut self, flag: u8) {
    self.force_hole_initialization |= flag;
  }

  /// 空洞检测位图索引 (0 = 不可缓存)
  pub fn hole_check_bitmap_index(&self) -> u8 {
    self.hole_check_bitmap_index
  }

  /// 分配空洞检测位图索引 (原版 AssignHoleCheckBitmapIndex, 索引须 < 位图位数)
  pub fn assign_hole_check_bitmap_index(&mut self, index: u8) {
    self.hole_check_bitmap_index = index;
  }

  /// 在位图中记录空洞检测 (原版 RememberHoleCheckInBitmap)
  ///
  /// 返回置位后的位图; 索引不可缓存或位图已满时不置位。
  pub fn remember_hole_check_in_bitmap(&self, bitmap: HoleCheckBitmap) -> HoleCheckBitmap {
    if self.hole_check_bitmap_index == UNCACHEABLE_HOLE_CHECK_BITMAP_INDEX
      || self.hole_check_bitmap_index >= HOLE_CHECK_BITMAP_BITS
    {
      return bitmap;
    }
    bitmap | (1u64 << self.hole_check_bitmap_index)
  }

  /// 位图中是否记录过该变量的空洞检测 (原版 HasRememberedHoleCheck)
  pub fn has_remembered_hole_check(&self, bitmap: HoleCheckBitmap) -> bool {
    self.hole_check_bitmap_index != UNCACHEABLE_HOLE_CHECK_BITMAP_INDEX
      && bitmap & (1u64 << self.hole_check_bitmap_index) != 0
  }
}
