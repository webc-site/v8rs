// 由 tool/src/builtins/builtins-promise.js 生成; 来源 v8/src/builtins 下 12 文件
// 内建名表 + 核心调度元数据 (56 内建)
// 来源: builtins-async-gen.cc, builtins-async-function-gen.cc, builtins-async-generator-gen.cc, builtins-async-iterator-gen.cc, builtins-async-module.cc, builtins-async-disposable-stack.cc 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// Await 行为控制枚举 (对应 v8/src/builtins/builtins-async-gen.h: AwaitBehavior)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum AwaitBehavior {
  #[strum(serialize = "IgnorePredefinedFunction", serialize = "Normal")]
  IgnorePredefinedFunction = 0,
  #[strum(serialize = "UsePredefinedFunction", serialize = "ResumeOnly")]
  UsePredefinedFunction = 1,
}

impl AwaitBehavior {
  /// V8 原版名称别名: kNormal = false
  pub const NORMAL: Self = Self::IgnorePredefinedFunction;
  /// V8 原版名称别名: kResumeOnly = true
  pub const RESUME_ONLY: Self = Self::UsePredefinedFunction;

  #[inline]
  pub const fn is_normal(&self) -> bool {
    matches!(self, Self::IgnorePredefinedFunction)
  }

  #[inline]
  pub const fn is_resume_only(&self) -> bool {
    matches!(self, Self::UsePredefinedFunction)
  }
}

/// Async / Generator / Disposable / Microtask 内建名表 (56 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  AsyncDisposableStackConstructor,
  AsyncDisposableStackOnFulfilled,
  AsyncDisposableStackOnRejected,
  AsyncDisposableStackPrototypeAdopt,
  AsyncDisposableStackPrototypeDefer,
  AsyncDisposableStackPrototypeDisposeAsync,
  AsyncDisposableStackPrototypeGetDisposed,
  AsyncDisposableStackPrototypeMove,
  AsyncDisposableStackPrototypeUse,
  AsyncDisposeFromSyncDispose,
  AsyncFromSyncIteratorCloseSyncAndRethrow,
  AsyncFromSyncIteratorPrototypeNext,
  AsyncFromSyncIteratorPrototypeReturn,
  AsyncFromSyncIteratorPrototypeThrow,
  AsyncFunctionAwait,
  AsyncFunctionAwaitRejectClosure,
  AsyncFunctionAwaitResolveClosure,
  AsyncFunctionEnter,
  AsyncFunctionLazyDeoptContinuation,
  AsyncFunctionReject,
  AsyncFunctionResolve,
  AsyncGeneratorAwait,
  AsyncGeneratorAwaitRejectClosure,
  AsyncGeneratorAwaitResolveClosure,
  AsyncGeneratorPrototypeNext,
  AsyncGeneratorPrototypeReturn,
  AsyncGeneratorPrototypeThrow,
  AsyncGeneratorReject,
  AsyncGeneratorResolve,
  AsyncGeneratorResumeNext,
  AsyncGeneratorReturn,
  AsyncGeneratorReturnClosedRejectClosure,
  AsyncGeneratorReturnClosedResolveClosure,
  AsyncGeneratorReturnResolveClosure,
  AsyncGeneratorYieldWithAwait,
  AsyncGeneratorYieldWithAwaitResolveClosure,
  AsyncIteratorValueUnwrap,
  AsyncModuleEvaluate,
  CallAsyncModuleFulfilled,
  CallAsyncModuleRejected,
  DisposableStackConstructor,
  DisposableStackPrototypeAdopt,
  DisposableStackPrototypeDefer,
  DisposableStackPrototypeDispose,
  DisposableStackPrototypeGetDisposed,
  DisposableStackPrototypeMove,
  DisposableStackPrototypeUse,
  EnqueueMicrotask,
  GeneratorPrototypeNext,
  GeneratorPrototypeReturn,
  GeneratorPrototypeThrow,
  GlobalQueueMicrotask,
  ResumeGeneratorBaseline,
  #[strum(serialize = "ResumeGeneratorTrampoline_WithCatch")]
  ResumeGeneratorTrampolineWithCatch,
  RunMicrotasks,
  SuspendGeneratorBaseline,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::AsyncDisposableStackConstructor => "AsyncDisposableStackConstructor",
      Self::AsyncDisposableStackOnFulfilled => "AsyncDisposableStackOnFulfilled",
      Self::AsyncDisposableStackOnRejected => "AsyncDisposableStackOnRejected",
      Self::AsyncDisposableStackPrototypeAdopt => "AsyncDisposableStackPrototypeAdopt",
      Self::AsyncDisposableStackPrototypeDefer => "AsyncDisposableStackPrototypeDefer",
      Self::AsyncDisposableStackPrototypeDisposeAsync => {
        "AsyncDisposableStackPrototypeDisposeAsync"
      }
      Self::AsyncDisposableStackPrototypeGetDisposed => "AsyncDisposableStackPrototypeGetDisposed",
      Self::AsyncDisposableStackPrototypeMove => "AsyncDisposableStackPrototypeMove",
      Self::AsyncDisposableStackPrototypeUse => "AsyncDisposableStackPrototypeUse",
      Self::AsyncDisposeFromSyncDispose => "AsyncDisposeFromSyncDispose",
      Self::AsyncFromSyncIteratorCloseSyncAndRethrow => "AsyncFromSyncIteratorCloseSyncAndRethrow",
      Self::AsyncFromSyncIteratorPrototypeNext => "AsyncFromSyncIteratorPrototypeNext",
      Self::AsyncFromSyncIteratorPrototypeReturn => "AsyncFromSyncIteratorPrototypeReturn",
      Self::AsyncFromSyncIteratorPrototypeThrow => "AsyncFromSyncIteratorPrototypeThrow",
      Self::AsyncFunctionAwait => "AsyncFunctionAwait",
      Self::AsyncFunctionAwaitRejectClosure => "AsyncFunctionAwaitRejectClosure",
      Self::AsyncFunctionAwaitResolveClosure => "AsyncFunctionAwaitResolveClosure",
      Self::AsyncFunctionEnter => "AsyncFunctionEnter",
      Self::AsyncFunctionLazyDeoptContinuation => "AsyncFunctionLazyDeoptContinuation",
      Self::AsyncFunctionReject => "AsyncFunctionReject",
      Self::AsyncFunctionResolve => "AsyncFunctionResolve",
      Self::AsyncGeneratorAwait => "AsyncGeneratorAwait",
      Self::AsyncGeneratorAwaitRejectClosure => "AsyncGeneratorAwaitRejectClosure",
      Self::AsyncGeneratorAwaitResolveClosure => "AsyncGeneratorAwaitResolveClosure",
      Self::AsyncGeneratorPrototypeNext => "AsyncGeneratorPrototypeNext",
      Self::AsyncGeneratorPrototypeReturn => "AsyncGeneratorPrototypeReturn",
      Self::AsyncGeneratorPrototypeThrow => "AsyncGeneratorPrototypeThrow",
      Self::AsyncGeneratorReject => "AsyncGeneratorReject",
      Self::AsyncGeneratorResolve => "AsyncGeneratorResolve",
      Self::AsyncGeneratorResumeNext => "AsyncGeneratorResumeNext",
      Self::AsyncGeneratorReturn => "AsyncGeneratorReturn",
      Self::AsyncGeneratorReturnClosedRejectClosure => "AsyncGeneratorReturnClosedRejectClosure",
      Self::AsyncGeneratorReturnClosedResolveClosure => "AsyncGeneratorReturnClosedResolveClosure",
      Self::AsyncGeneratorReturnResolveClosure => "AsyncGeneratorReturnResolveClosure",
      Self::AsyncGeneratorYieldWithAwait => "AsyncGeneratorYieldWithAwait",
      Self::AsyncGeneratorYieldWithAwaitResolveClosure => {
        "AsyncGeneratorYieldWithAwaitResolveClosure"
      }
      Self::AsyncIteratorValueUnwrap => "AsyncIteratorValueUnwrap",
      Self::AsyncModuleEvaluate => "AsyncModuleEvaluate",
      Self::CallAsyncModuleFulfilled => "CallAsyncModuleFulfilled",
      Self::CallAsyncModuleRejected => "CallAsyncModuleRejected",
      Self::DisposableStackConstructor => "DisposableStackConstructor",
      Self::DisposableStackPrototypeAdopt => "DisposableStackPrototypeAdopt",
      Self::DisposableStackPrototypeDefer => "DisposableStackPrototypeDefer",
      Self::DisposableStackPrototypeDispose => "DisposableStackPrototypeDispose",
      Self::DisposableStackPrototypeGetDisposed => "DisposableStackPrototypeGetDisposed",
      Self::DisposableStackPrototypeMove => "DisposableStackPrototypeMove",
      Self::DisposableStackPrototypeUse => "DisposableStackPrototypeUse",
      Self::EnqueueMicrotask => "EnqueueMicrotask",
      Self::GeneratorPrototypeNext => "GeneratorPrototypeNext",
      Self::GeneratorPrototypeReturn => "GeneratorPrototypeReturn",
      Self::GeneratorPrototypeThrow => "GeneratorPrototypeThrow",
      Self::GlobalQueueMicrotask => "GlobalQueueMicrotask",
      Self::ResumeGeneratorBaseline => "ResumeGeneratorBaseline",
      Self::ResumeGeneratorTrampolineWithCatch => "ResumeGeneratorTrampoline_WithCatch",
      Self::RunMicrotasks => "RunMicrotasks",
      Self::SuspendGeneratorBaseline => "SuspendGeneratorBaseline",
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

  /// 是否为 AsyncFunction 核心内建
  #[inline]
  pub const fn is_async_function(&self) -> bool {
    matches!(
      self,
      Self::AsyncFunctionAwait
        | Self::AsyncFunctionAwaitRejectClosure
        | Self::AsyncFunctionAwaitResolveClosure
        | Self::AsyncFunctionEnter
        | Self::AsyncFunctionLazyDeoptContinuation
        | Self::AsyncFunctionReject
        | Self::AsyncFunctionResolve
    )
  }

  /// 是否为 AsyncGenerator 核心内建
  #[inline]
  pub const fn is_async_generator(&self) -> bool {
    matches!(
      self,
      Self::AsyncGeneratorAwait
        | Self::AsyncGeneratorAwaitRejectClosure
        | Self::AsyncGeneratorAwaitResolveClosure
        | Self::AsyncGeneratorPrototypeNext
        | Self::AsyncGeneratorPrototypeReturn
        | Self::AsyncGeneratorPrototypeThrow
        | Self::AsyncGeneratorReject
        | Self::AsyncGeneratorResolve
        | Self::AsyncGeneratorResumeNext
        | Self::AsyncGeneratorReturn
        | Self::AsyncGeneratorReturnClosedRejectClosure
        | Self::AsyncGeneratorReturnClosedResolveClosure
        | Self::AsyncGeneratorReturnResolveClosure
        | Self::AsyncGeneratorYieldWithAwait
        | Self::AsyncGeneratorYieldWithAwaitResolveClosure
    )
  }

  /// 是否为标准同步 Generator 核心内建
  #[inline]
  pub const fn is_generator(&self) -> bool {
    matches!(
      self,
      Self::GeneratorPrototypeNext
        | Self::GeneratorPrototypeReturn
        | Self::GeneratorPrototypeThrow
        | Self::ResumeGeneratorBaseline
        | Self::ResumeGeneratorTrampolineWithCatch
        | Self::SuspendGeneratorBaseline
    )
  }

  /// 是否为 DisposableStack / AsyncDisposableStack 资源释放栈内建
  #[inline]
  pub const fn is_disposable_stack(&self) -> bool {
    matches!(
      self,
      Self::AsyncDisposableStackConstructor
        | Self::AsyncDisposableStackOnFulfilled
        | Self::AsyncDisposableStackOnRejected
        | Self::AsyncDisposableStackPrototypeAdopt
        | Self::AsyncDisposableStackPrototypeDefer
        | Self::AsyncDisposableStackPrototypeDisposeAsync
        | Self::AsyncDisposableStackPrototypeGetDisposed
        | Self::AsyncDisposableStackPrototypeMove
        | Self::AsyncDisposableStackPrototypeUse
        | Self::AsyncDisposeFromSyncDispose
        | Self::DisposableStackConstructor
        | Self::DisposableStackPrototypeAdopt
        | Self::DisposableStackPrototypeDefer
        | Self::DisposableStackPrototypeDispose
        | Self::DisposableStackPrototypeGetDisposed
        | Self::DisposableStackPrototypeMove
        | Self::DisposableStackPrototypeUse
    )
  }

  /// 是否为微任务调度内建
  #[inline]
  pub const fn is_microtask(&self) -> bool {
    matches!(
      self,
      Self::EnqueueMicrotask | Self::GlobalQueueMicrotask | Self::RunMicrotasks
    )
  }
}

// 统一提供 56 项内建工厂函数
pub const fn async_disposable_stack_constructor() -> Builtin {
  Builtin::AsyncDisposableStackConstructor
}

pub const fn async_disposable_stack_on_fulfilled() -> Builtin {
  Builtin::AsyncDisposableStackOnFulfilled
}

pub const fn async_disposable_stack_on_rejected() -> Builtin {
  Builtin::AsyncDisposableStackOnRejected
}

pub const fn async_disposable_stack_prototype_adopt() -> Builtin {
  Builtin::AsyncDisposableStackPrototypeAdopt
}

pub const fn async_disposable_stack_prototype_defer() -> Builtin {
  Builtin::AsyncDisposableStackPrototypeDefer
}

pub const fn async_disposable_stack_prototype_dispose_async() -> Builtin {
  Builtin::AsyncDisposableStackPrototypeDisposeAsync
}

pub const fn async_disposable_stack_prototype_get_disposed() -> Builtin {
  Builtin::AsyncDisposableStackPrototypeGetDisposed
}

pub const fn async_disposable_stack_prototype_move() -> Builtin {
  Builtin::AsyncDisposableStackPrototypeMove
}

pub const fn async_disposable_stack_prototype_use() -> Builtin {
  Builtin::AsyncDisposableStackPrototypeUse
}

pub const fn async_dispose_from_sync_dispose() -> Builtin {
  Builtin::AsyncDisposeFromSyncDispose
}

pub const fn async_from_sync_iterator_close_sync_and_rethrow() -> Builtin {
  Builtin::AsyncFromSyncIteratorCloseSyncAndRethrow
}

pub const fn async_from_sync_iterator_prototype_next() -> Builtin {
  Builtin::AsyncFromSyncIteratorPrototypeNext
}

pub const fn async_from_sync_iterator_prototype_return() -> Builtin {
  Builtin::AsyncFromSyncIteratorPrototypeReturn
}

pub const fn async_from_sync_iterator_prototype_throw() -> Builtin {
  Builtin::AsyncFromSyncIteratorPrototypeThrow
}

pub const fn async_function_await() -> Builtin {
  Builtin::AsyncFunctionAwait
}

pub const fn async_function_await_reject_closure() -> Builtin {
  Builtin::AsyncFunctionAwaitRejectClosure
}

pub const fn async_function_await_resolve_closure() -> Builtin {
  Builtin::AsyncFunctionAwaitResolveClosure
}

pub const fn async_function_enter() -> Builtin {
  Builtin::AsyncFunctionEnter
}

pub const fn async_function_lazy_deopt_continuation() -> Builtin {
  Builtin::AsyncFunctionLazyDeoptContinuation
}

pub const fn async_function_reject() -> Builtin {
  Builtin::AsyncFunctionReject
}

pub const fn async_function_resolve() -> Builtin {
  Builtin::AsyncFunctionResolve
}

pub const fn async_generator_await() -> Builtin {
  Builtin::AsyncGeneratorAwait
}

pub const fn async_generator_await_reject_closure() -> Builtin {
  Builtin::AsyncGeneratorAwaitRejectClosure
}

pub const fn async_generator_await_resolve_closure() -> Builtin {
  Builtin::AsyncGeneratorAwaitResolveClosure
}

pub const fn async_generator_prototype_next() -> Builtin {
  Builtin::AsyncGeneratorPrototypeNext
}

pub const fn async_generator_prototype_return() -> Builtin {
  Builtin::AsyncGeneratorPrototypeReturn
}

pub const fn async_generator_prototype_throw() -> Builtin {
  Builtin::AsyncGeneratorPrototypeThrow
}

pub const fn async_generator_reject() -> Builtin {
  Builtin::AsyncGeneratorReject
}

pub const fn async_generator_resolve() -> Builtin {
  Builtin::AsyncGeneratorResolve
}

pub const fn async_generator_resume_next() -> Builtin {
  Builtin::AsyncGeneratorResumeNext
}

pub const fn async_generator_return() -> Builtin {
  Builtin::AsyncGeneratorReturn
}

pub const fn async_generator_return_closed_reject_closure() -> Builtin {
  Builtin::AsyncGeneratorReturnClosedRejectClosure
}

pub const fn async_generator_return_closed_resolve_closure() -> Builtin {
  Builtin::AsyncGeneratorReturnClosedResolveClosure
}

pub const fn async_generator_return_resolve_closure() -> Builtin {
  Builtin::AsyncGeneratorReturnResolveClosure
}

pub const fn async_generator_yield_with_await() -> Builtin {
  Builtin::AsyncGeneratorYieldWithAwait
}

pub const fn async_generator_yield_with_await_resolve_closure() -> Builtin {
  Builtin::AsyncGeneratorYieldWithAwaitResolveClosure
}

pub const fn async_iterator_value_unwrap() -> Builtin {
  Builtin::AsyncIteratorValueUnwrap
}

pub const fn async_module_evaluate() -> Builtin {
  Builtin::AsyncModuleEvaluate
}

pub const fn call_async_module_fulfilled() -> Builtin {
  Builtin::CallAsyncModuleFulfilled
}

pub const fn call_async_module_rejected() -> Builtin {
  Builtin::CallAsyncModuleRejected
}

pub const fn disposable_stack_constructor() -> Builtin {
  Builtin::DisposableStackConstructor
}

pub const fn disposable_stack_prototype_adopt() -> Builtin {
  Builtin::DisposableStackPrototypeAdopt
}

pub const fn disposable_stack_prototype_defer() -> Builtin {
  Builtin::DisposableStackPrototypeDefer
}

pub const fn disposable_stack_prototype_dispose() -> Builtin {
  Builtin::DisposableStackPrototypeDispose
}

pub const fn disposable_stack_prototype_get_disposed() -> Builtin {
  Builtin::DisposableStackPrototypeGetDisposed
}

pub const fn disposable_stack_prototype_move() -> Builtin {
  Builtin::DisposableStackPrototypeMove
}

pub const fn disposable_stack_prototype_use() -> Builtin {
  Builtin::DisposableStackPrototypeUse
}

pub const fn enqueue_microtask() -> Builtin {
  Builtin::EnqueueMicrotask
}

pub const fn generator_prototype_next() -> Builtin {
  Builtin::GeneratorPrototypeNext
}

pub const fn generator_prototype_return() -> Builtin {
  Builtin::GeneratorPrototypeReturn
}

pub const fn generator_prototype_throw() -> Builtin {
  Builtin::GeneratorPrototypeThrow
}

pub const fn global_queue_microtask() -> Builtin {
  Builtin::GlobalQueueMicrotask
}

pub const fn resume_generator_baseline() -> Builtin {
  Builtin::ResumeGeneratorBaseline
}

pub const fn resume_generator_trampoline_with_catch() -> Builtin {
  Builtin::ResumeGeneratorTrampolineWithCatch
}

pub const fn run_microtasks() -> Builtin {
  Builtin::RunMicrotasks
}

pub const fn suspend_generator_baseline() -> Builtin {
  Builtin::SuspendGeneratorBaseline
}

/// 来自 `generator.tq` / `array-from-async.tq` 的声明清单 (共 12 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("struct", "ArrayFromAsyncIterableResumeState"),
  ("type", "ArrayFromAsyncIterableResolveContext"),
  ("extern macro", "AllocateRootFunctionWithContext"),
  ("macro", "CreateArrayFromAsyncIterableResolveContext"),
  (
    "macro",
    "GetIteratorRecordFromArrayFromAsyncIterableResolveContext",
  ),
  (
    "extern macro",
    "ArrayFromAsyncIterableOnFulfilledSharedFunConstant",
  ),
  (
    "extern macro",
    "ArrayFromAsyncIterableOnRejectedSharedFunConstant",
  ),
  ("struct", "ArrayFromAsyncArrayLikeResumeState"),
  ("type", "ArrayFromAsyncArrayLikeResolveContext"),
  ("macro", "CreateArrayFromAsyncArrayLikeResolveContext"),
  (
    "extern macro",
    "ArrayFromAsyncArrayLikeOnFulfilledSharedFunConstant",
  ),
  (
    "extern macro",
    "ArrayFromAsyncArrayLikeOnRejectedSharedFunConstant",
  ),
];
