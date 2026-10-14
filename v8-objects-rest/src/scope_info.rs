// 由 tool/src/objects/scope-info.h.js 从 v8/src/objects/scope-info.{h,-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// ScopeInfo 平铺索引 + tq 枚举全套 + 位段表
//! ScopeInfo: 作用域元数据。
//! scope-info.cc 语义面 (变量解析/创建逻辑, >2k 行) 登记跳过; 本产物仅规则区。

// ── h 侧: 平铺字段索引与引导类型 ────────────────────────────────────────────

/// 平铺字段索引 (原版 enum Fields; 0..4 固定头, >= kVariablePartStart 编码变长尾)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Fields {
  /// kFlags
  Flags = 0,
  /// kParameterCount
  Parametercount = 1,
  /// kContextLocalCount
  Contextlocalcount = 2,
  /// kPositionInfoStart
  Positioninfostart = 3,
  /// kPositionInfoEnd
  Positioninfoend = 4,
  /// kVariablePartStart
  Variablepartstart = 5,
}

/// 变长区起点索引。
pub const VARIABLE_PART_START: u8 = 5;

/// 变量初始化位置最大可存距离 (超出落 kMaxInt)。
pub const MAX_VARIABLE_POSITION_DISTANCE: u32 = 0xFFFF;

/// 引导创建类型 (原版 enum class BootstrappingType)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BootstrappingType {
  /// kScript
  Script = 0,
  /// kFunction
  Function = 1,
  /// kNative
  Native = 2,
  /// kShadowRealm
  Shadowrealm = 3,
}

// ── tq 侧: 枚举全套 ─────────────────────────────────────────────────────────

/// 原版 tq `extern enum ScopeType extends uint32`, 共 10 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ScopeType {
  /// SCRIPT_SCOPE
  ScriptScope = 0,
  /// REPL_MODE_SCOPE
  ReplModeScope = 1,
  /// EVAL_SCOPE
  EvalScope = 2,
  /// MODULE_SCOPE
  ModuleScope = 3,
  /// CLASS_SCOPE
  ClassScope = 4,
  /// FUNCTION_SCOPE
  FunctionScope = 5,
  /// CATCH_SCOPE
  CatchScope = 6,
  /// BLOCK_SCOPE
  BlockScope = 7,
  /// WITH_SCOPE
  WithScope = 8,
  /// SHADOW_REALM_SCOPE
  ShadowRealmScope = 9,
}

impl ScopeType {
  /// 变体总数
  pub const COUNT: usize = 10;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: i32) -> Option<Self> {
    match v {
      0 => Some(Self::ScriptScope),
      1 => Some(Self::ReplModeScope),
      2 => Some(Self::EvalScope),
      3 => Some(Self::ModuleScope),
      4 => Some(Self::ClassScope),
      5 => Some(Self::FunctionScope),
      6 => Some(Self::CatchScope),
      7 => Some(Self::BlockScope),
      8 => Some(Self::WithScope),
      9 => Some(Self::ShadowRealmScope),
      _ => None,
    }
  }
}

/// 原版 tq `extern enum VariableAllocationInfo extends uint32`, 共 4 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum VariableAllocationInfo {
  /// NONE
  None = 0,
  /// STACK
  Stack = 1,
  /// CONTEXT
  Context = 2,
  /// UNUSED
  Unused = 3,
}

impl VariableAllocationInfo {
  /// 变体总数
  pub const COUNT: usize = 4;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: i32) -> Option<Self> {
    match v {
      0 => Some(Self::None),
      1 => Some(Self::Stack),
      2 => Some(Self::Context),
      3 => Some(Self::Unused),
      _ => None,
    }
  }
}

/// 原版 tq `extern enum VariableMode extends int32`, 共 13 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum VariableMode {
  /// kLet
  Let = 0,
  /// kConst
  Const = 1,
  /// kUsing
  Using = 2,
  /// kAwaitUsing
  Awaitusing = 3,
  /// kVar
  Var = 4,
  /// kTemporary
  Temporary = 5,
  /// kDynamic
  Dynamic = 6,
  /// kDynamicGlobal
  Dynamicglobal = 7,
  /// kDynamicLocal
  Dynamiclocal = 8,
  /// kPrivateMethod
  Privatemethod = 9,
  /// kPrivateSetterOnly
  Privatesetteronly = 10,
  /// kPrivateGetterOnly
  Privategetteronly = 11,
  /// kPrivateGetterAndSetter
  Privategetterandsetter = 12,
}

impl VariableMode {
  /// 变体总数
  pub const COUNT: usize = 13;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: i32) -> Option<Self> {
    match v {
      0 => Some(Self::Let),
      1 => Some(Self::Const),
      2 => Some(Self::Using),
      3 => Some(Self::Awaitusing),
      4 => Some(Self::Var),
      5 => Some(Self::Temporary),
      6 => Some(Self::Dynamic),
      7 => Some(Self::Dynamicglobal),
      8 => Some(Self::Dynamiclocal),
      9 => Some(Self::Privatemethod),
      10 => Some(Self::Privatesetteronly),
      11 => Some(Self::Privategetteronly),
      12 => Some(Self::Privategetterandsetter),
      _ => None,
    }
  }
}

/// 原版 tq `extern enum InitializationFlag extends uint32`, 共 2 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum InitializationFlag {
  /// kNeedsInitialization
  Needsinitialization = 0,
  /// kCreatedInitialized
  Createdinitialized = 1,
}

impl InitializationFlag {
  /// 变体总数
  pub const COUNT: usize = 2;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: i32) -> Option<Self> {
    match v {
      0 => Some(Self::Needsinitialization),
      1 => Some(Self::Createdinitialized),
      _ => None,
    }
  }
}

/// 原版 tq `extern enum IsStaticFlag extends uint32`, 共 2 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum IsStaticFlag {
  /// kNotStatic
  Notstatic = 0,
  /// kStatic
  Static = 1,
}

impl IsStaticFlag {
  /// 变体总数
  pub const COUNT: usize = 2;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: i32) -> Option<Self> {
    match v {
      0 => Some(Self::Notstatic),
      1 => Some(Self::Static),
      _ => None,
    }
  }
}

/// 原版 tq `extern enum MaybeAssignedFlag extends uint32`, 共 2 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MaybeAssignedFlag {
  /// kNotAssigned
  Notassigned = 0,
  /// kMaybeAssigned
  Maybeassigned = 1,
}

impl MaybeAssignedFlag {
  /// 变体总数
  pub const COUNT: usize = 2;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: i32) -> Option<Self> {
    match v {
      0 => Some(Self::Notassigned),
      1 => Some(Self::Maybeassigned),
      _ => None,
    }
  }
}

/// 原版 tq `bitfield struct ScopeFlags` 位段表 (顺序 1:1, 共 22 段 / 31 位)。
pub mod scope_flags {
  /// scope_type: ScopeType (bit 0..3)。
  pub const SCOPE_TYPE_SHIFT: u32 = 0;
  pub const SCOPE_TYPE_WIDTH: u32 = 4;
  pub const SCOPE_TYPE_MASK: u32 = 0xf;
  /// sloppy_eval_can_extend_vars: bool (bit 4..4)。
  pub const SLOPPY_EVAL_CAN_EXTEND_VARS_SHIFT: u32 = 4;
  pub const SLOPPY_EVAL_CAN_EXTEND_VARS_WIDTH: u32 = 1;
  pub const SLOPPY_EVAL_CAN_EXTEND_VARS_MASK: u32 = 0x10;
  /// language_mode: LanguageMode (bit 5..5)。
  pub const LANGUAGE_MODE_SHIFT: u32 = 5;
  pub const LANGUAGE_MODE_WIDTH: u32 = 1;
  pub const LANGUAGE_MODE_MASK: u32 = 0x20;
  /// declaration_scope: bool (bit 6..6)。
  pub const DECLARATION_SCOPE_SHIFT: u32 = 6;
  pub const DECLARATION_SCOPE_WIDTH: u32 = 1;
  pub const DECLARATION_SCOPE_MASK: u32 = 0x40;
  /// receiver_variable: VariableAllocationInfo (bit 7..8)。
  pub const RECEIVER_VARIABLE_SHIFT: u32 = 7;
  pub const RECEIVER_VARIABLE_WIDTH: u32 = 2;
  pub const RECEIVER_VARIABLE_MASK: u32 = 0x180;
  /// class_scope_has_private_brand: bool (bit 9..9)。
  pub const CLASS_SCOPE_HAS_PRIVATE_BRAND_SHIFT: u32 = 9;
  pub const CLASS_SCOPE_HAS_PRIVATE_BRAND_WIDTH: u32 = 1;
  pub const CLASS_SCOPE_HAS_PRIVATE_BRAND_MASK: u32 = 0x200;
  /// has_saved_class_variable: bool (bit 10..10)。
  pub const HAS_SAVED_CLASS_VARIABLE_SHIFT: u32 = 10;
  pub const HAS_SAVED_CLASS_VARIABLE_WIDTH: u32 = 1;
  pub const HAS_SAVED_CLASS_VARIABLE_MASK: u32 = 0x400;
  /// allocates_arguments: bool (bit 11..11)。
  pub const ALLOCATES_ARGUMENTS_SHIFT: u32 = 11;
  pub const ALLOCATES_ARGUMENTS_WIDTH: u32 = 1;
  pub const ALLOCATES_ARGUMENTS_MASK: u32 = 0x800;
  /// function_variable: VariableAllocationInfo (bit 12..13)。
  pub const FUNCTION_VARIABLE_SHIFT: u32 = 12;
  pub const FUNCTION_VARIABLE_WIDTH: u32 = 2;
  pub const FUNCTION_VARIABLE_MASK: u32 = 0x3000;
  /// has_inferred_function_name: bool (bit 14..14)。
  pub const HAS_INFERRED_FUNCTION_NAME_SHIFT: u32 = 14;
  pub const HAS_INFERRED_FUNCTION_NAME_WIDTH: u32 = 1;
  pub const HAS_INFERRED_FUNCTION_NAME_MASK: u32 = 0x4000;
  /// has_simple_parameters: bool (bit 15..15)。
  pub const HAS_SIMPLE_PARAMETERS_SHIFT: u32 = 15;
  pub const HAS_SIMPLE_PARAMETERS_WIDTH: u32 = 1;
  pub const HAS_SIMPLE_PARAMETERS_MASK: u32 = 0x8000;
  /// function_kind: FunctionKind (bit 16..20)。
  pub const FUNCTION_KIND_SHIFT: u32 = 16;
  pub const FUNCTION_KIND_WIDTH: u32 = 5;
  pub const FUNCTION_KIND_MASK: u32 = 0x1f0000;
  /// has_outer_scope_info: bool (bit 21..21)。
  pub const HAS_OUTER_SCOPE_INFO_SHIFT: u32 = 21;
  pub const HAS_OUTER_SCOPE_INFO_WIDTH: u32 = 1;
  pub const HAS_OUTER_SCOPE_INFO_MASK: u32 = 0x200000;
  /// is_debug_evaluate_scope: bool (bit 22..22)。
  pub const IS_DEBUG_EVALUATE_SCOPE_SHIFT: u32 = 22;
  pub const IS_DEBUG_EVALUATE_SCOPE_WIDTH: u32 = 1;
  pub const IS_DEBUG_EVALUATE_SCOPE_MASK: u32 = 0x400000;
  /// force_context_allocation: bool (bit 23..23)。
  pub const FORCE_CONTEXT_ALLOCATION_SHIFT: u32 = 23;
  pub const FORCE_CONTEXT_ALLOCATION_WIDTH: u32 = 1;
  pub const FORCE_CONTEXT_ALLOCATION_MASK: u32 = 0x800000;
  /// private_name_lookup_skips_outer_class: bool (bit 24..24)。
  pub const PRIVATE_NAME_LOOKUP_SKIPS_OUTER_CLASS_SHIFT: u32 = 24;
  pub const PRIVATE_NAME_LOOKUP_SKIPS_OUTER_CLASS_WIDTH: u32 = 1;
  pub const PRIVATE_NAME_LOOKUP_SKIPS_OUTER_CLASS_MASK: u32 = 0x1000000;
  /// has_context_extension_slot: bool (bit 25..25)。
  pub const HAS_CONTEXT_EXTENSION_SLOT_SHIFT: u32 = 25;
  pub const HAS_CONTEXT_EXTENSION_SLOT_WIDTH: u32 = 1;
  pub const HAS_CONTEXT_EXTENSION_SLOT_MASK: u32 = 0x2000000;
  /// some_context_has_extension: bool (bit 26..26)。
  pub const SOME_CONTEXT_HAS_EXTENSION_SHIFT: u32 = 26;
  pub const SOME_CONTEXT_HAS_EXTENSION_WIDTH: u32 = 1;
  pub const SOME_CONTEXT_HAS_EXTENSION_MASK: u32 = 0x4000000;
  /// is_hidden: bool (bit 27..27)。
  pub const IS_HIDDEN_SHIFT: u32 = 27;
  pub const IS_HIDDEN_WIDTH: u32 = 1;
  pub const IS_HIDDEN_MASK: u32 = 0x8000000;
  /// is_wrapped_function: bool (bit 28..28)。
  pub const IS_WRAPPED_FUNCTION_SHIFT: u32 = 28;
  pub const IS_WRAPPED_FUNCTION_WIDTH: u32 = 1;
  pub const IS_WRAPPED_FUNCTION_MASK: u32 = 0x10000000;
  /// has_context_cells: bool (bit 29..29)。
  pub const HAS_CONTEXT_CELLS_SHIFT: u32 = 29;
  pub const HAS_CONTEXT_CELLS_WIDTH: u32 = 1;
  pub const HAS_CONTEXT_CELLS_MASK: u32 = 0x20000000;
  /// is_hoisted_in_context: bool (bit 30..30)。
  pub const IS_HOISTED_IN_CONTEXT_SHIFT: u32 = 30;
  pub const IS_HOISTED_IN_CONTEXT_WIDTH: u32 = 1;
  pub const IS_HOISTED_IN_CONTEXT_MASK: u32 = 0x40000000;
}

/// 原版 tq `bitfield struct VariableProperties` 位段表 (顺序 1:1, 共 6 段 / 24 位)。
pub mod variable_properties {
  /// variable_mode: VariableMode (bit 0..3)。
  pub const VARIABLE_MODE_SHIFT: u32 = 0;
  pub const VARIABLE_MODE_WIDTH: u32 = 4;
  pub const VARIABLE_MODE_MASK: u32 = 0xf;
  /// init_flag: InitializationFlag (bit 4..4)。
  pub const INIT_FLAG_SHIFT: u32 = 4;
  pub const INIT_FLAG_WIDTH: u32 = 1;
  pub const INIT_FLAG_MASK: u32 = 0x10;
  /// maybe_assigned_flag: MaybeAssignedFlag (bit 5..5)。
  pub const MAYBE_ASSIGNED_FLAG_SHIFT: u32 = 5;
  pub const MAYBE_ASSIGNED_FLAG_WIDTH: u32 = 1;
  pub const MAYBE_ASSIGNED_FLAG_MASK: u32 = 0x20;
  /// is_parameter: bool (bit 6..6)。
  pub const IS_PARAMETER_SHIFT: u32 = 6;
  pub const IS_PARAMETER_WIDTH: u32 = 1;
  pub const IS_PARAMETER_MASK: u32 = 0x40;
  /// parameter_number_or_position: uint32 (bit 7..22)。
  pub const PARAMETER_NUMBER_OR_POSITION_SHIFT: u32 = 7;
  pub const PARAMETER_NUMBER_OR_POSITION_WIDTH: u32 = 16;
  pub const PARAMETER_NUMBER_OR_POSITION_MASK: u32 = 0x7fff80;
  /// is_static_flag: IsStaticFlag (bit 23..23)。
  pub const IS_STATIC_FLAG_SHIFT: u32 = 23;
  pub const IS_STATIC_FLAG_WIDTH: u32 = 1;
  pub const IS_STATIC_FLAG_MASK: u32 = 0x800000;
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(VARIABLE_PART_START == 5);
  assert!(Fields::PositionInfoEnd as u8 == 4);
  assert!(MAX_VARIABLE_POSITION_DISTANCE == 0xFFFF);
  assert!(BootstrappingType::ShadowRealm as u8 == 3);
  assert!(ScopeType::COUNT == 10);
  assert!(ScopeType::ScriptScope as i32 == 0);
  assert!(ScopeType::ShadowRealmScope as i32 == 9);
  assert!(VariableMode::COUNT == 13);
  assert!(VariableAllocationInfo::COUNT == 4);
  assert!(ScopeFlags::SCOPE_TYPE_MASK == 0xF);
  assert!(ScopeFlags::IS_HOISTED_IN_CONTEXT_SHIFT == 30);
  assert!(VariableProperties::PARAMETER_NUMBER_OR_POSITION_WIDTH == 16);
  assert!(VariableProperties::IS_STATIC_FLAG_SHIFT == 23);
};
