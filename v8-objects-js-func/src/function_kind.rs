// 由 tool/src/objects/function-kind.h.js 从 v8/src/objects/function-kind.h 生成; 修订就地改, 复跑覆盖
// FunctionKind 位域枚举与谓词族

/// 函数种类 (原版 enum class FunctionKind : uint8_t, 分组注释保留原版结构)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FunctionKind {
  // END concise methods 2
  /// kInvalid 哨兵
  Invalid = -1,
}

impl FunctionKind {
  /// 位宽 (原版 kFunctionKindBitSize)
  pub const BIT_SIZE: u32 = 5;
  /// 原版 kLastFunctionKind 锚点
  pub const LAST: Self = Self::ClassStaticInitializerFunctionPrecededByMember;

  /// 区间包含 (原版 base::IsInRange, const 可内联)
  #[inline]
  #[must_use]
  pub const fn in_range(kind: Self, lo: Self, hi: Self) -> bool {
    kind as u8 >= lo as u8 && kind as u8 <= hi as u8
  }
  /// 原版 IsArrowFunction
  #[inline]
  #[must_use]
  pub const fn is_arrow_function(self) -> bool {
    in_range(kind, Self::ArrowFunction, Self::AsyncArrowFunction)
  }

  /// 原版 IsModule
  #[inline]
  #[must_use]
  pub const fn is_module(self) -> bool {
    in_range(kind, Self::Module, Self::ModuleWithTopLevelAwait)
  }

  /// 原版 IsModuleWithTopLevelAwait
  #[inline]
  #[must_use]
  pub const fn is_module_with_top_level_await(self) -> bool {
    kind == Self::ModuleWithTopLevelAwait
  }

  /// 原版 IsAsyncGeneratorFunction
  #[inline]
  #[must_use]
  pub const fn is_async_generator_function(self) -> bool {
    in_range(kind, Self::AsyncConciseGeneratorMethod, Self::AsyncGeneratorFunction)
  }

  /// 原版 IsGeneratorFunction
  #[inline]
  #[must_use]
  pub const fn is_generator_function(self) -> bool {
    in_range(kind, Self::AsyncConciseGeneratorMethod, Self::StaticConciseGeneratorMethod)
  }

  /// 原版 IsAsyncFunction
  #[inline]
  #[must_use]
  pub const fn is_async_function(self) -> bool {
    in_range(kind, Self::AsyncArrowFunction, Self::AsyncGeneratorFunction)
  }

  /// 原版 IsResumableFunction
  #[inline]
  #[must_use]
  pub const fn is_resumable_function(self) -> bool {
    kind.is_generator_function() || kind.is_async_function() || kind.is_module()
  }

  /// 原版 IsConciseMethod
  #[inline]
  #[must_use]
  pub const fn is_concise_method(self) -> bool {
    in_range(kind, Self::AsyncConciseMethod, Self::StaticAsyncConciseGeneratorMethod) || in_range(kind, Self::ConciseGeneratorMethod, Self::ClassStaticInitializerFunctionPrecededByMember)
  }

  /// 原版 IsStrictFunctionWithoutPrototype
  #[inline]
  #[must_use]
  pub const fn is_strict_function_without_prototype(self) -> bool {
    in_range(kind, Self::GetterFunction, Self::AsyncArrowFunction) || in_range(kind, Self::AsyncConciseMethod, Self::StaticAsyncConciseGeneratorMethod) || in_range(kind, Self::ConciseGeneratorMethod, Self::ClassStaticInitializerFunctionPrecededByMember)
  }

  /// 原版 IsGetterFunction
  #[inline]
  #[must_use]
  pub const fn is_getter_function(self) -> bool {
    in_range(kind, Self::GetterFunction, Self::StaticGetterFunction)
  }

  /// 原版 IsSetterFunction
  #[inline]
  #[must_use]
  pub const fn is_setter_function(self) -> bool {
    in_range(kind, Self::SetterFunction, Self::StaticSetterFunction)
  }

  /// 原版 IsAccessorFunction
  #[inline]
  #[must_use]
  pub const fn is_accessor_function(self) -> bool {
    in_range(kind, Self::GetterFunction, Self::StaticSetterFunction)
  }

  /// 原版 IsDefaultConstructor
  #[inline]
  #[must_use]
  pub const fn is_default_constructor(self) -> bool {
    in_range(kind, Self::DefaultBaseConstructor, Self::DefaultDerivedConstructor)
  }

  /// 原版 IsBaseConstructor
  #[inline]
  #[must_use]
  pub const fn is_base_constructor(self) -> bool {
    in_range(kind, Self::BaseConstructor, Self::DefaultBaseConstructor)
  }

  /// 原版 IsDerivedConstructor
  #[inline]
  #[must_use]
  pub const fn is_derived_constructor(self) -> bool {
    in_range(kind, Self::DefaultDerivedConstructor, Self::DerivedConstructor)
  }

  /// 原版 IsClassConstructor
  #[inline]
  #[must_use]
  pub const fn is_class_constructor(self) -> bool {
    in_range(kind, Self::BaseConstructor, Self::DerivedConstructor)
  }

  /// 原版 IsClassInitializerFunction
  #[inline]
  #[must_use]
  pub const fn is_class_initializer_function(self) -> bool {
    in_range(kind, Self::ClassMembersInitializerFunction, Self::ClassStaticInitializerFunctionPrecededByMember)
  }

  /// 原版 IsClassInstanceInitializerFunction
  #[inline]
  #[must_use]
  pub const fn is_class_instance_initializer_function(self) -> bool {
    in_range(kind, Self::ClassMembersInitializerFunction, Self::ClassMembersInitializerFunctionPrecededByStatic)
  }

  /// 原版 IsClassStaticInitializerFunction
  #[inline]
  #[must_use]
  pub const fn is_class_static_initializer_function(self) -> bool {
    in_range(kind, Self::ClassStaticInitializerFunction, Self::ClassStaticInitializerFunctionPrecededByMember)
  }

  /// 原版 IsConstructable
  #[inline]
  #[must_use]
  pub const fn is_constructable(self) -> bool {
    in_range(kind, Self::NormalFunction, Self::DerivedConstructor)
  }

  /// 原版 IsStatic (switch 展开为 matches!)
  #[inline]
  #[must_use]
  pub const fn is_static(self) -> bool {
    matches!(self, Self::StaticGetterFunction | Self::StaticSetterFunction | Self::StaticConciseMethod | Self::StaticConciseGeneratorMethod | Self::StaticAsyncConciseMethod | Self::StaticAsyncConciseGeneratorMethod | Self::ClassStaticInitializerFunction | Self::ClassStaticInitializerFunctionPrecededByMember)
  }

  /// 原版 BindsSuper
  #[inline]
  #[must_use]
  pub const fn binds_super(self) -> bool {
    kind.is_concise_method() || kind.is_accessor_function() || kind.is_class_constructor()
  }

  /// 原版 FunctionKind2String
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FunctionKind::LAST as u8 < (1 << FunctionKind::BIT_SIZE));
  assert!(FunctionKind::in_range(FunctionKind::ArrowFunction, FunctionKind::ArrowFunction, FunctionKind::AsyncArrowFunction));
  assert!(FunctionKind::NormalFunction.is_constructable());
  assert!(!FunctionKind::ArrowFunction.is_constructable());
  assert!(FunctionKind::StaticConciseMethod.is_static());
  assert!(!FunctionKind::ConciseMethod.is_static());
};
