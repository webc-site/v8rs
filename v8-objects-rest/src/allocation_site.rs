// 由 tool/src/objects/allocation-site.h.js 从 v8/src/objects/allocation-site.{h,inl.h,tq} + allocation-site-scopes.{h,inl.h} 生成; 修订就地改, 复跑覆盖
// AllocationSite 预留决策位域与枚举 (AllocationMemento/遍历上下文骨架见尾注)

/// 预留决策 (原版 enum PretenureDecision)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PretenureDecision {
  /// kUndecided
  Undecided = 0,
  /// kDontTenure
  DontTenure = 1,
  /// kMaybeTenure
  MaybeTenure = 2,
  /// kTenure
  Tenure = 3,
}

impl PretenureDecision {
  /// 原版 kLastPretenureDecisionValue
  pub const LAST: Self = Self::Tenure;

  /// 原版 PretenureDecisionName (objects.cc 侧 switch)
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::Undecided => "undecided",
      Self::DontTenure => "don't tenure",
      Self::MaybeTenure => "maybe tenure",
      Self::Tenure => "tenure",
    }
  }
}

/// 触发 pretransition 的数组最大字节数 (原版 kMaximumArrayBytesToPretransition)
pub const MAXIMUM_ARRAY_BYTES_TO_PRETRANSITION: u32 = 8192;

/// 更新模式 (原版 enum class AllocationSiteUpdateMode, 定义于 globals.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AllocationSiteUpdateMode {
  /// kUpdate
  Update = 0,
  /// kCheckOnly
  CheckOnly = 1,
}

/// transition_info 位域 (原版 ElementsKindBits / SpeculationDisabledBit)
pub struct TransitionInfoBits;

impl TransitionInfoBits {
  /// ElementsKind: 位 0, 宽 6
  pub const ElementsKind_SHIFT: u32 = 0;
  pub const ElementsKind_MASK: i32 = 0x3f;
  /// SpeculationDisabled: 位 6, 宽 1
  pub const SpeculationDisabled_SHIFT: u32 = 6;
  pub const SpeculationDisabled_MASK: i32 = 0x40;
}

/// pretenure_data 位域 (原版 MementoFoundCountBits / PretenureDecisionBits / DeoptDependentCodeBit)
pub struct PretenureDataBits;

impl PretenureDataBits {
  /// MementoFoundCountBits: 位 0, 宽 26
  pub const MementoFoundCount_SHIFT: u32 = 0;
  pub const MementoFoundCount_MASK: i32 = 0x3ffffff;
  /// PretenureDecisionBits: 位 26, 宽 3
  pub const PretenureDecision_SHIFT: u32 = 26;
  pub const PretenureDecision_MASK: i32 = 0x1c000000;
  /// DeoptDependentCodeBits: 位 29, 宽 1
  pub const DeoptDependentCode_SHIFT: u32 = 29;
  pub const DeoptDependentCode_MASK: i32 = 0x20000000;
}

/// memento 计数与决策迁移语义见原版 allocation-site-inl.h (比率逻辑在调用方)

/// 遍历上下文常量 (原版 AllocationSiteUsageContext::kCopying)
pub const ALLOCATION_SITE_USAGE_CONTEXT_COPYING: bool = true;

// —— 签名骨架 (原版公开接口, 供 Developer 补语义) ——
// class AllocationSiteContext { top(); current(); ShouldCreateMemento(); InitializeTraversal(); }
// class AllocationSiteUsageContext : AllocationSiteContext { EnterNewScope(); ExitScope(); }
// AllocationSite: Initialize / HasWeakNext / IsNested / GetAllocationType /
//   ResetPretenureDecision / MakePretenureDecision / DigestPretenuringFeedback /
//   DigestTransitionFeedback / ShouldTrack / CanTrack / IncrementMementoFoundCount
// AllocationMemento : Struct { allocation_site: AllocationSite } (tq)
// HasWeakNext 形态: AllocationSiteWithWeakNext 附加 weak_next 字段 (h 侧, 布局待接)

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAXIMUM_ARRAY_BYTES_TO_PRETRANSITION == 8192);
  assert!(PretenureDecision::LAST as u8 == 3);
  assert!(PretenureDecision::Tenure.to_str() == "tenure");
  assert!(TransitionInfoBits::ELEMENTS_KIND_SHIFT == 0);
  assert!(PretenureDataBits::PRETENURE_DECISION_SHIFT == 26);
  assert!(PretenureDataBits::DEOPT_DEPENDENT_CODE_MASK == 0x20000000);
  assert!(ALLOCATION_SITE_USAGE_CONTEXT_COPYING);
};
