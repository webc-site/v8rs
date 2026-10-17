// 由 tool/src/builtins/builtins-promise.js 生成; 来源 v8/src/builtins 下 21 文件
// 内建名表 + 核心调度元数据 (34 内建)
// 来源: builtins-promise-gen.cc, builtins-promise-gen.h, promise-*.tq 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// Promise 状态机 (对应 v8/src/objects/promise.h: Promise::PromiseState)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum PromiseState {
  Pending = 0,
  Fulfilled = 1,
  Rejected = 2,
}

impl PromiseState {
  #[inline]
  pub const fn is_pending(&self) -> bool {
    matches!(self, Self::Pending)
  }

  #[inline]
  pub const fn is_settled(&self) -> bool {
    !matches!(self, Self::Pending)
  }

  #[inline]
  pub const fn is_fulfilled(&self) -> bool {
    matches!(self, Self::Fulfilled)
  }

  #[inline]
  pub const fn is_rejected(&self) -> bool {
    matches!(self, Self::Rejected)
  }
}

/// Promise 钩子事件类型 (对应 v8/src/execution/isolate.h: PromiseHookType)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum PromiseHookType {
  Init = 0,
  Resolve = 1,
  Before = 2,
  After = 3,
}

impl PromiseHookType {
  #[inline]
  pub const fn is_init(&self) -> bool {
    matches!(self, Self::Init)
  }

  #[inline]
  pub const fn is_resolve(&self) -> bool {
    matches!(self, Self::Resolve)
  }

  #[inline]
  pub const fn is_before(&self) -> bool {
    matches!(self, Self::Before)
  }

  #[inline]
  pub const fn is_after(&self) -> bool {
    matches!(self, Self::After)
  }
}

/// Promise 核心内建名表 (34 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  ResolvePromise,
  FulfillPromise,
  RejectPromise,
  NewPromiseCapability,
  PromiseCapabilityDefaultReject,
  PromiseCapabilityDefaultResolve,
  PerformPromiseThenFunction,
  PerformPromiseThen,
  PromiseReject,
  PromiseGetCapabilitiesExecutor,
  PromiseConstructorLazyDeoptContinuation,
  PromiseAllResolveElementClosure,
  PromiseAllSettledResolveElementClosure,
  PromiseAllSettledRejectElementClosure,
  PromiseAll,
  PromiseAllSettled,
  PromiseAnyRejectElementClosure,
  PromiseAny,
  PromiseConstructor,
  PromisePrototypeCatch,
  PromiseValueThunkFinally,
  PromiseThrowerFinally,
  PromiseCatchFinally,
  PromiseThenFinally,
  PromisePrototypeFinally,
  PromiseResolveThenableJob,
  PromiseRace,
  PromiseFulfillReactionJob,
  PromiseRejectReactionJob,
  PromiseResolveTrampoline,
  PromiseResolve,
  PromisePrototypeThen,
  PromiseTry,
  PromiseWithResolvers,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::ResolvePromise => "ResolvePromise",
      Self::FulfillPromise => "FulfillPromise",
      Self::RejectPromise => "RejectPromise",
      Self::NewPromiseCapability => "NewPromiseCapability",
      Self::PromiseCapabilityDefaultReject => "PromiseCapabilityDefaultReject",
      Self::PromiseCapabilityDefaultResolve => "PromiseCapabilityDefaultResolve",
      Self::PerformPromiseThenFunction => "PerformPromiseThenFunction",
      Self::PerformPromiseThen => "PerformPromiseThen",
      Self::PromiseReject => "PromiseReject",
      Self::PromiseGetCapabilitiesExecutor => "PromiseGetCapabilitiesExecutor",
      Self::PromiseConstructorLazyDeoptContinuation => "PromiseConstructorLazyDeoptContinuation",
      Self::PromiseAllResolveElementClosure => "PromiseAllResolveElementClosure",
      Self::PromiseAllSettledResolveElementClosure => "PromiseAllSettledResolveElementClosure",
      Self::PromiseAllSettledRejectElementClosure => "PromiseAllSettledRejectElementClosure",
      Self::PromiseAll => "PromiseAll",
      Self::PromiseAllSettled => "PromiseAllSettled",
      Self::PromiseAnyRejectElementClosure => "PromiseAnyRejectElementClosure",
      Self::PromiseAny => "PromiseAny",
      Self::PromiseConstructor => "PromiseConstructor",
      Self::PromisePrototypeCatch => "PromisePrototypeCatch",
      Self::PromiseValueThunkFinally => "PromiseValueThunkFinally",
      Self::PromiseThrowerFinally => "PromiseThrowerFinally",
      Self::PromiseCatchFinally => "PromiseCatchFinally",
      Self::PromiseThenFinally => "PromiseThenFinally",
      Self::PromisePrototypeFinally => "PromisePrototypeFinally",
      Self::PromiseResolveThenableJob => "PromiseResolveThenableJob",
      Self::PromiseRace => "PromiseRace",
      Self::PromiseFulfillReactionJob => "PromiseFulfillReactionJob",
      Self::PromiseRejectReactionJob => "PromiseRejectReactionJob",
      Self::PromiseResolveTrampoline => "PromiseResolveTrampoline",
      Self::PromiseResolve => "PromiseResolve",
      Self::PromisePrototypeThen => "PromisePrototypeThen",
      Self::PromiseTry => "PromiseTry",
      Self::PromiseWithResolvers => "PromiseWithResolvers",
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

  /// 是否为 Promise 组合器操作 (all, allSettled, any, race)
  #[inline]
  pub const fn is_combinator(&self) -> bool {
    matches!(
      self,
      Self::PromiseAll | Self::PromiseAllSettled | Self::PromiseAny | Self::PromiseRace
    )
  }

  /// 是否为 Promise 原型链公开方法 (then, catch, finally)
  #[inline]
  pub const fn is_prototype_method(&self) -> bool {
    matches!(
      self,
      Self::PromisePrototypeThen | Self::PromisePrototypeCatch | Self::PromisePrototypeFinally
    )
  }

  /// 是否为异步微任务反应 Job (ReactionJob / ThenableJob)
  #[inline]
  pub const fn is_reaction_job(&self) -> bool {
    matches!(
      self,
      Self::PromiseFulfillReactionJob
        | Self::PromiseRejectReactionJob
        | Self::PromiseResolveThenableJob
    )
  }

  /// 是否为 Promise 构造函数或能力创建入口
  #[inline]
  pub const fn is_constructor_or_capability(&self) -> bool {
    matches!(
      self,
      Self::PromiseConstructor
        | Self::NewPromiseCapability
        | Self::PromiseWithResolvers
        | Self::PromiseGetCapabilitiesExecutor
    )
  }
}

// 统一提供 34 项内建工厂函数
pub const fn resolve_promise() -> Builtin {
  Builtin::ResolvePromise
}

pub const fn fulfill_promise() -> Builtin {
  Builtin::FulfillPromise
}

pub const fn reject_promise() -> Builtin {
  Builtin::RejectPromise
}

pub const fn new_promise_capability() -> Builtin {
  Builtin::NewPromiseCapability
}

pub const fn promise_capability_default_reject() -> Builtin {
  Builtin::PromiseCapabilityDefaultReject
}

pub const fn promise_capability_default_resolve() -> Builtin {
  Builtin::PromiseCapabilityDefaultResolve
}

pub const fn perform_promise_then_function() -> Builtin {
  Builtin::PerformPromiseThenFunction
}

pub const fn perform_promise_then() -> Builtin {
  Builtin::PerformPromiseThen
}

pub const fn promise_reject() -> Builtin {
  Builtin::PromiseReject
}

pub const fn promise_get_capabilities_executor() -> Builtin {
  Builtin::PromiseGetCapabilitiesExecutor
}

pub const fn promise_constructor_lazy_deopt_continuation() -> Builtin {
  Builtin::PromiseConstructorLazyDeoptContinuation
}

pub const fn promise_all_resolve_element_closure() -> Builtin {
  Builtin::PromiseAllResolveElementClosure
}

pub const fn promise_all_settled_resolve_element_closure() -> Builtin {
  Builtin::PromiseAllSettledResolveElementClosure
}

pub const fn promise_all_settled_reject_element_closure() -> Builtin {
  Builtin::PromiseAllSettledRejectElementClosure
}

pub const fn promise_all() -> Builtin {
  Builtin::PromiseAll
}

pub const fn promise_all_settled() -> Builtin {
  Builtin::PromiseAllSettled
}

pub const fn promise_any_reject_element_closure() -> Builtin {
  Builtin::PromiseAnyRejectElementClosure
}

pub const fn promise_any() -> Builtin {
  Builtin::PromiseAny
}

pub const fn promise_constructor() -> Builtin {
  Builtin::PromiseConstructor
}

pub const fn promise_prototype_catch() -> Builtin {
  Builtin::PromisePrototypeCatch
}

pub const fn promise_value_thunk_finally() -> Builtin {
  Builtin::PromiseValueThunkFinally
}

pub const fn promise_thrower_finally() -> Builtin {
  Builtin::PromiseThrowerFinally
}

pub const fn promise_catch_finally() -> Builtin {
  Builtin::PromiseCatchFinally
}

pub const fn promise_then_finally() -> Builtin {
  Builtin::PromiseThenFinally
}

pub const fn promise_prototype_finally() -> Builtin {
  Builtin::PromisePrototypeFinally
}

pub const fn promise_resolve_thenable_job() -> Builtin {
  Builtin::PromiseResolveThenableJob
}

pub const fn promise_race() -> Builtin {
  Builtin::PromiseRace
}

pub const fn promise_fulfill_reaction_job() -> Builtin {
  Builtin::PromiseFulfillReactionJob
}

pub const fn promise_reject_reaction_job() -> Builtin {
  Builtin::PromiseRejectReactionJob
}

pub const fn promise_resolve_trampoline() -> Builtin {
  Builtin::PromiseResolveTrampoline
}

pub const fn promise_resolve() -> Builtin {
  Builtin::PromiseResolve
}

pub const fn promise_prototype_then() -> Builtin {
  Builtin::PromisePrototypeThen
}

pub const fn promise_try() -> Builtin {
  Builtin::PromiseTry
}

pub const fn promise_with_resolvers() -> Builtin {
  Builtin::PromiseWithResolvers
}

/// 来自 `promise*.tq` 的声明清单 (共 80 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("extern macro", "PromiseForwardingHandlerSymbolConstant"),
  ("extern macro", "PromiseHandledBySymbolConstant"),
  ("extern macro", "ResolveStringConstant"),
  ("extern macro", "IsPromiseResolveProtectorCellInvalid"),
  ("extern macro", "AllocateRootFunctionWithContext"),
  ("extern macro", "PromiseReactionMapConstant"),
  ("extern macro", "PromiseFulfillReactionJobTaskMapConstant"),
  ("extern macro", "PromiseRejectReactionJobTaskMapConstant"),
  ("macro", "ExtractHandlerContextInternal"),
  ("macro", "ExtractHandlerContext"),
  ("macro", "ExtractHandlerContext"),
  ("extern macro", "PromiseBuiltinsAssembler"),
  ("extern macro", "PromiseBuiltinsAssembler"),
  ("type", "PromiseResolvingFunctionContext"),
  ("type", "PromiseCapabilitiesExecutorContext"),
  ("macro", "CreatePromiseCapabilitiesExecutorContext"),
  ("macro", "CreatePromiseCapability"),
  ("struct", "PromiseResolvingFunctions"),
  ("macro", "CreatePromiseResolvingFunctions"),
  ("macro", "IsPromiseResolveLookupChainIntact"),
  (
    "extern macro",
    "PromiseCapabilityDefaultRejectSharedFunConstant",
  ),
  (
    "extern macro",
    "PromiseCapabilityDefaultResolveSharedFunConstant",
  ),
  (
    "extern macro",
    "PromiseGetCapabilitiesExecutorSharedFunConstant",
  ),
  ("macro", "CreatePromiseAllResolveElementContext"),
  ("macro", "CreatePromiseAllResolveElementFunction"),
  ("macro", "CreatePromiseResolvingFunctionsContext"),
  ("macro", "IsPromiseThenLookupChainIntact"),
  ("struct", "PromiseAllResolveElementFunctor"),
  ("struct", "PromiseAllRejectElementFunctor"),
  ("struct", "PromiseAllSettledResolveElementFunctor"),
  ("struct", "PromiseAllSettledRejectElementFunctor"),
  ("extern macro", "MakeFixedArrayCOW"),
  ("struct", "PromiseAllWrapResultAsFulfilledFunctor"),
  ("struct", "PromiseAllSettledWrapResultAsFulfilledFunctor"),
  ("struct", "PromiseAllSettledWrapResultAsRejectedFunctor"),
  ("extern macro", "LoadJSReceiverIdentityHash"),
  ("type", "PromiseAllResolveElementContext"),
  ("type", "PromiseAnyRejectElementContext"),
  ("macro", "CreatePromiseAnyRejectElementFunction"),
  ("macro", "IsPromiseHookEnabledOrHasAsyncEventDelegate"),
  ("macro", "PromiseHasHandler"),
  ("macro", "PromiseInit"),
  ("macro", "BranchIfFastPath"),
  ("macro", "BranchIfFastPath"),
  ("macro", "BranchIfFastPath"),
  ("macro", "IsPromiseThenProtectorCellInvalid"),
  ("macro", "CreatePromiseFinallyContext"),
  ("macro", "CreatePromiseReactionJobTask"),
  ("macro", "RunContext"),
  ("macro", "MicrotaskQueueBuiltinsAssembler"),
  ("macro", "MicrotaskQueueBuiltinsAssembler"),
  ("macro", "MicrotaskQueueBuiltinsAssembler"),
  ("macro", "MicrotaskQueueBuiltinsAssembler"),
  ("extern macro", "MicrotaskQueueBuiltinsAssembler"),
  ("extern macro", "MicrotaskQueueBuiltinsAssembler"),
  ("extern macro", "MicrotaskQueueBuiltinsAssembler"),
  ("extern macro", "MicrotaskQueueBuiltinsAssembler"),
  ("extern macro", "MicrotaskQueueBuiltinsAssembler"),
  ("extern macro", "MicrotaskQueueBuiltinsAssembler"),
  ("extern macro", "MicrotaskQueueBuiltinsAssembler"),
  ("extern macro", "MicrotaskQueueBuiltinsAssembler"),
  ("macro", "IsPromiseSpeciesLookupChainIntact"),
  ("macro", "PromiseReactionJob"),
  ("macro", "BranchIfFastPath"),
  ("extern macro", "IsPromiseThenProtectorCellInvalid"),
  ("macro", "PerformPromiseThenImpl"),
  ("extern macro", "HasExistingFalsePromiseHook"),
  ("struct", "PromiseResolvingFunctionPair"),
  ("type", "PromiseResolvingFunctionPair"),
  ("macro", "CreatePromiseResolvingFunctions"),
  ("macro", "NewPromiseResolvingFunctionsPair"),
  ("macro", "CreatePromiseCapability"),
  ("macro", "CreatePromiseCapability"),
  ("macro", "NewPromiseCapability"),
  ("macro", "NewPromiseCapability"),
  ("struct", "PromiseRecord"),
  ("macro", "CreatePromiseRecord"),
  ("macro", "IsPromise"),
  ("macro", "IsPromise"),
  ("macro", "IsPromise"),
];
