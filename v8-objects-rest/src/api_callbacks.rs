// 由 tool/src/objects/api-callbacks.h.js 从 v8/src/objects/api-callbacks.{h,tq} 生成; 修订就地改, 复跑覆盖
// API 回调对象: 位域与回调列表

/// AccessorInfo 标志位 (tq bitfield struct AccessorInfoFlags extends uint32)
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AccessorInfoFlags(pub u32);

impl AccessorInfoFlags {
  /// is_sloppy: bool, 位 0..1 (宽 1)
  pub const IsSloppy_SHIFT: u32 = 0;
  pub const IsSloppy_SIZE: u32 = 1;
  pub const IsSloppy_MASK: u32 = 0x1;
  /// replace_on_access: bool, 位 1..2 (宽 1)
  pub const ReplaceOnAccess_SHIFT: u32 = 1;
  pub const ReplaceOnAccess_SIZE: u32 = 1;
  pub const ReplaceOnAccess_MASK: u32 = 0x2;
  /// getter_side_effect_type: SideEffectType, 位 2..4 (宽 2)
  pub const GetterSideEffectType_SHIFT: u32 = 2;
  pub const GetterSideEffectType_SIZE: u32 = 2;
  pub const GetterSideEffectType_MASK: u32 = 0xc;
  /// setter_side_effect_type: SideEffectType, 位 4..6 (宽 2)
  pub const SetterSideEffectType_SHIFT: u32 = 4;
  pub const SetterSideEffectType_SIZE: u32 = 2;
  pub const SetterSideEffectType_MASK: u32 = 0x30;
  /// initial_attributes: PropertyAttributes, 位 6..9 (宽 3)
  pub const InitialAttributes_SHIFT: u32 = 6;
  pub const InitialAttributes_SIZE: u32 = 3;
  pub const InitialAttributes_MASK: u32 = 0x1c0;

  /// 原始位串
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u32 {
    self.0
  }

  /// 读取 is_sloppy 位 (原版 accessorinfois_sloppy)
  #[inline]
  #[must_use]
  pub const fn is_sloppy(self) -> bool {
    self.0 & Self::IsSloppy_MASK != 0
  }

  /// 读取 replace_on_access 位 (原版 accessorinforeplace_on_access)
  #[inline]
  #[must_use]
  pub const fn replace_on_access(self) -> bool {
    self.0 & Self::ReplaceOnAccess_MASK != 0
  }

  /// 读取 getter_side_effect_type 字段 (宽 2, 原版 getter_side_effect_type)
  #[inline]
  #[must_use]
  pub const fn get_getter_side_effect_type(self) -> u32 {
    (self.0 & Self::GetterSideEffectType_MASK) >> Self::GetterSideEffectType_SHIFT
  }

  /// 读取 setter_side_effect_type 字段 (宽 2, 原版 setter_side_effect_type)
  #[inline]
  #[must_use]
  pub const fn get_setter_side_effect_type(self) -> u32 {
    (self.0 & Self::SetterSideEffectType_MASK) >> Self::SetterSideEffectType_SHIFT
  }

  /// 读取 initial_attributes 字段 (宽 3, 原版 initial_attributes)
  #[inline]
  #[must_use]
  pub const fn get_initial_attributes(self) -> u32 {
    (self.0 & Self::InitialAttributes_MASK) >> Self::InitialAttributes_SHIFT
  }
}

/// InterceptorInfo 标志位 (tq bitfield struct InterceptorInfoFlags extends uint31)
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InterceptorInfoFlags(pub u32);

impl InterceptorInfoFlags {
  /// can_intercept_symbols: bool, 位 0..1 (宽 1)
  pub const CanInterceptSymbols_SHIFT: u32 = 0;
  pub const CanInterceptSymbols_SIZE: u32 = 1;
  pub const CanInterceptSymbols_MASK: u32 = 0x1;
  /// non_masking: bool, 位 1..2 (宽 1)
  pub const NonMasking_SHIFT: u32 = 1;
  pub const NonMasking_SIZE: u32 = 1;
  pub const NonMasking_MASK: u32 = 0x2;
  /// named: bool, 位 2..3 (宽 1)
  pub const Named_SHIFT: u32 = 2;
  pub const Named_SIZE: u32 = 1;
  pub const Named_MASK: u32 = 0x4;
  /// has_no_side_effect: bool, 位 3..4 (宽 1)
  pub const HasNoSideEffect_SHIFT: u32 = 3;
  pub const HasNoSideEffect_SIZE: u32 = 1;
  pub const HasNoSideEffect_MASK: u32 = 0x8;
  /// has_new_callbacks_signature: bool, 位 4..5 (宽 1)
  pub const HasNewCallbacksSignature_SHIFT: u32 = 4;
  pub const HasNewCallbacksSignature_SIZE: u32 = 1;
  pub const HasNewCallbacksSignature_MASK: u32 = 0x10;
  /// has_dont_delete_property: bool, 位 5..6 (宽 1)
  pub const HasDontDeleteProperty_SHIFT: u32 = 5;
  pub const HasDontDeleteProperty_SIZE: u32 = 1;
  pub const HasDontDeleteProperty_MASK: u32 = 0x20;

  /// 原始位串
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u32 {
    self.0
  }

  /// 读取 can_intercept_symbols 位 (原版 interceptorinfocan_intercept_symbols)
  #[inline]
  #[must_use]
  pub const fn can_intercept_symbols(self) -> bool {
    self.0 & Self::CanInterceptSymbols_MASK != 0
  }

  /// 读取 non_masking 位 (原版 interceptorinfonon_masking)
  #[inline]
  #[must_use]
  pub const fn non_masking(self) -> bool {
    self.0 & Self::NonMasking_MASK != 0
  }

  /// 读取 named 位 (原版 interceptorinfonamed)
  #[inline]
  #[must_use]
  pub const fn named(self) -> bool {
    self.0 & Self::Named_MASK != 0
  }

  /// 读取 has_no_side_effect 位 (原版 interceptorinfohas_no_side_effect)
  #[inline]
  #[must_use]
  pub const fn has_no_side_effect(self) -> bool {
    self.0 & Self::HasNoSideEffect_MASK != 0
  }

  /// 读取 has_new_callbacks_signature 位 (原版 interceptorinfohas_new_callbacks_signature)
  #[inline]
  #[must_use]
  pub const fn has_new_callbacks_signature(self) -> bool {
    self.0 & Self::HasNewCallbacksSignature_MASK != 0
  }

  /// 读取 has_dont_delete_property 位 (原版 interceptorinfohas_dont_delete_property)
  #[inline]
  #[must_use]
  pub const fn has_dont_delete_property(self) -> bool {
    self.0 & Self::HasDontDeleteProperty_MASK != 0
  }
}

/// InterceptorInfo::Flag 掩码枚举 (原版 enum Flag : uint32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct InterceptorFlag(pub u32);

impl InterceptorFlag {
  /// kNone
  pub const NONE: Self = Self(0);
  /// kCanInterceptSymbols = 对应位域掩码
  pub const CanInterceptSymbols: Self = Self(0x1);
  /// kNonMasking = 对应位域掩码
  pub const NonMasking: Self = Self(0x2);
  /// kNamed = 对应位域掩码
  pub const Named: Self = Self(0x4);
  /// kHasNoSideEffect = 对应位域掩码
  pub const HasNoSideEffect: Self = Self(0x8);
  /// kHasNewCallbacksSignature = 对应位域掩码
  pub const HasNewCallbacksSignature: Self = Self(0x10);
  /// kHasDontDeleteProperty = 对应位域掩码
  pub const HasDontDeleteProperty: Self = Self(0x20);

  /// 位串
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u32 {
    self.0
  }

  /// 合并
  #[inline]
  #[must_use]
  pub const fn union(self, other: Self) -> Self {
    Self(self.0 | other.0)
  }

  /// 含判定
  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }
}

/// 位域总数 (原版 InterceptorInfo::kFlagCount)
pub const INTERCEPTOR_FLAG_COUNT: usize = 6;

/// 命名/索引拦截器共有回调 (原版 COMMON_INTERCEPTOR_INFO_CALLBACK_LIST 展开)
pub const COMMON_INTERCEPTOR_CALLBACKS: [&str; 7] = [
  "Getter",
  "Setter",
  "Query",
  "Descriptor",
  "Deleter",
  "Enumerator",
  "Definer",
];

/// 索引拦截器全量回调 (原版 INDEXED_INTERCEPTOR_INFO_CALLBACK_LIST 展开)
pub const INDEXED_INTERCEPTOR_CALLBACKS: [&str; 9] = [
  "Getter",
  "Setter",
  "Query",
  "Descriptor",
  "Deleter",
  "Enumerator",
  "Definer",
  "IndexOf",
  "IterableToList",
];

/// 索引拦截器独有回调 (IndexOf / IterableToList)
pub const INDEXED_ONLY_CALLBACKS: [&str; 2] = [
  "IndexOf",
  "IterableToList",
];

/// 副作用类型别名 (原版 type SideEffectType extends int32, 枚举本体在 debug-interface.h)
pub type SideEffectType = i32;

// —— 提取不变式断言 ——
const _: () = {
  assert!(INTERCEPTOR_FLAG_COUNT == 6);
  assert!(InterceptorFlag::NONE.bits() == 0);
  assert!(InterceptorFlag::NON_MASKING.contains(InterceptorFlag::NON_MASKING));
  assert!(InterceptorInfoFlags::default().bits() == 0);
  assert!(AccessorInfoFlags::IS_SLOPPY_MASK == 0x1);
  assert!(COMMON_INTERCEPTOR_CALLBACKS.len() == 7);
  assert!(INDEXED_ONLY_CALLBACKS.len() == 2);
};
