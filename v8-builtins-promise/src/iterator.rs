// 由 tool/src/builtins/builtins-promise.js 生成; 来源 v8/src/builtins 下 7 文件
// 内建名表 + 核心调度元数据 (14 内建)
// 来源: builtins-iterator-gen.cc, builtins-iterator-gen.h, builtins-iterator.h, iterator.tq 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// Iterator 核心内建名表 (14 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  CallIteratorWithFeedbackLazyDeoptContinuation,
  ForOfNextLoadDoneLazyDeoptContinuation,
  ForOfNextLoadValueEagerDeoptContinuation,
  ForOfNextLoadValueLazyDeoptContinuation,
  ForOfNextResultDeoptContinuation,
  GetIteratorWithFeedbackLazyDeoptContinuation,
  IterableToFixedArray,
  IterableToFixedArrayForWasm,
  IterableToFixedArrayWithSymbolLookupSlow,
  IterableToList,
  IterableToListConvertHoles,
  IterableToListMayPreserveHoles,
  IterableToListWithSymbolLookup,
  StringListFromIterable,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::CallIteratorWithFeedbackLazyDeoptContinuation => {
        "CallIteratorWithFeedbackLazyDeoptContinuation"
      }
      Self::ForOfNextLoadDoneLazyDeoptContinuation => "ForOfNextLoadDoneLazyDeoptContinuation",
      Self::ForOfNextLoadValueEagerDeoptContinuation => "ForOfNextLoadValueEagerDeoptContinuation",
      Self::ForOfNextLoadValueLazyDeoptContinuation => "ForOfNextLoadValueLazyDeoptContinuation",
      Self::ForOfNextResultDeoptContinuation => "ForOfNextResultDeoptContinuation",
      Self::GetIteratorWithFeedbackLazyDeoptContinuation => {
        "GetIteratorWithFeedbackLazyDeoptContinuation"
      }
      Self::IterableToFixedArray => "IterableToFixedArray",
      Self::IterableToFixedArrayForWasm => "IterableToFixedArrayForWasm",
      Self::IterableToFixedArrayWithSymbolLookupSlow => "IterableToFixedArrayWithSymbolLookupSlow",
      Self::IterableToList => "IterableToList",
      Self::IterableToListConvertHoles => "IterableToListConvertHoles",
      Self::IterableToListMayPreserveHoles => "IterableToListMayPreserveHoles",
      Self::IterableToListWithSymbolLookup => "IterableToListWithSymbolLookup",
      Self::StringListFromIterable => "StringListFromIterable",
    }
  }

  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取对应的接口描述符符号
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 是否为去优化延续栈帧 (DeoptContinuation)
  #[inline]
  pub const fn is_deopt_continuation(&self) -> bool {
    matches!(
      self,
      Self::CallIteratorWithFeedbackLazyDeoptContinuation
        | Self::ForOfNextLoadDoneLazyDeoptContinuation
        | Self::ForOfNextLoadValueEagerDeoptContinuation
        | Self::ForOfNextLoadValueLazyDeoptContinuation
        | Self::ForOfNextResultDeoptContinuation
        | Self::GetIteratorWithFeedbackLazyDeoptContinuation
    )
  }

  /// 是否为可迭代对象转列表/数组操作 (IterableTo*)
  #[inline]
  pub const fn is_iterable_to_list(&self) -> bool {
    matches!(
      self,
      Self::IterableToFixedArray
        | Self::IterableToFixedArrayForWasm
        | Self::IterableToFixedArrayWithSymbolLookupSlow
        | Self::IterableToList
        | Self::IterableToListConvertHoles
        | Self::IterableToListMayPreserveHoles
        | Self::IterableToListWithSymbolLookup
        | Self::StringListFromIterable
    )
  }
}

// 统一提供 14 项内建工厂函数
pub const fn call_iterator_with_feedback_lazy_deopt_continuation() -> Builtin {
  Builtin::CallIteratorWithFeedbackLazyDeoptContinuation
}

pub const fn for_of_next_load_done_lazy_deopt_continuation() -> Builtin {
  Builtin::ForOfNextLoadDoneLazyDeoptContinuation
}

pub const fn for_of_next_load_value_eager_deopt_continuation() -> Builtin {
  Builtin::ForOfNextLoadValueEagerDeoptContinuation
}

pub const fn for_of_next_load_value_lazy_deopt_continuation() -> Builtin {
  Builtin::ForOfNextLoadValueLazyDeoptContinuation
}

pub const fn for_of_next_result_deopt_continuation() -> Builtin {
  Builtin::ForOfNextResultDeoptContinuation
}

pub const fn get_iterator_with_feedback_lazy_deopt_continuation() -> Builtin {
  Builtin::GetIteratorWithFeedbackLazyDeoptContinuation
}

pub const fn iterable_to_fixed_array() -> Builtin {
  Builtin::IterableToFixedArray
}

pub const fn iterable_to_fixed_array_for_wasm() -> Builtin {
  Builtin::IterableToFixedArrayForWasm
}

pub const fn iterable_to_fixed_array_with_symbol_lookup_slow() -> Builtin {
  Builtin::IterableToFixedArrayWithSymbolLookupSlow
}

pub const fn iterable_to_list() -> Builtin {
  Builtin::IterableToList
}

pub const fn iterable_to_list_convert_holes() -> Builtin {
  Builtin::IterableToListConvertHoles
}

pub const fn iterable_to_list_may_preserve_holes() -> Builtin {
  Builtin::IterableToListMayPreserveHoles
}

pub const fn iterable_to_list_with_symbol_lookup() -> Builtin {
  Builtin::IterableToListWithSymbolLookup
}

pub const fn string_list_from_iterable() -> Builtin {
  Builtin::StringListFromIterable
}

/// 来自 `iterator*.tq` 的声明清单 (共 27 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("struct", "IteratorRecord"),
  ("extern macro", "IteratorBuiltinsAssembler"),
  ("extern macro", "LoadContextFromBaseline"),
  ("extern macro", "LoadFeedbackVectorFromBaseline"),
  ("macro", "GetLazyReceiver"),
  ("extern macro", "AllocateRootFunctionWithContext"),
  (
    "extern macro",
    "AsyncIteratorPrototypeAsyncDisposeResolveClosureSharedFunConstant",
  ),
  ("type", "AsyncIteratorPrototypeAsyncDisposeResolveContext"),
  (
    "javascript builtin",
    "AsyncIteratorPrototypeAsyncDisposeResolveClosure",
  ),
  ("macro", "StoreRegisterToRawAddr"),
  ("macro", "StoreRegisterToParentFrame"),
  ("macro", "IsIteratorHelperExhausted"),
  ("macro", "MarkIteratorHelperAsExhausted"),
  ("macro", "IsIteratorHelperExecuting"),
  ("macro", "ThrowIfIteratorHelperExecuting"),
  ("macro", "MarkIteratorHelperAsExecuting"),
  ("macro", "IteratorHelperIsSuspendedStart"),
  ("macro", "MarkIteratorHelperAsFinishedExecuting"),
  ("macro", "NewJSIteratorMapHelper"),
  ("macro", "NewJSIteratorFilterHelper"),
  ("macro", "NewJSIteratorTakeHelper"),
  ("macro", "NewJSIteratorDropHelper"),
  ("macro", "NewJSIteratorFlatMapHelper"),
  ("macro", "NewJSIteratorConcatHelper"),
  ("macro", "NewJSIteratorZipHelper"),
  ("macro", "NewJSIteratorZipKeyedHelper"),
  ("macro", "NewJSValidIteratorWrapper"),
];
