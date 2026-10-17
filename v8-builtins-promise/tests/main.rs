use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_builtins_promise::{
  ASYNC_TQ_DECLS, AsyncBuiltin, AwaitBehavior, Error, ITERATOR_TQ_DECLS, IteratorBuiltin,
  PROMISE_TQ_DECLS, PromiseBuiltin, PromiseHookType, PromiseState,
  async_disposable_stack_constructor, async_disposable_stack_on_fulfilled,
  async_disposable_stack_on_rejected, async_disposable_stack_prototype_adopt,
  async_disposable_stack_prototype_defer, async_disposable_stack_prototype_dispose_async,
  async_disposable_stack_prototype_get_disposed, async_disposable_stack_prototype_move,
  async_disposable_stack_prototype_use, async_dispose_from_sync_dispose,
  async_from_sync_iterator_close_sync_and_rethrow, async_from_sync_iterator_prototype_next,
  async_from_sync_iterator_prototype_return, async_from_sync_iterator_prototype_throw,
  async_function_await, async_function_await_reject_closure, async_function_await_resolve_closure,
  async_function_enter, async_function_lazy_deopt_continuation, async_function_reject,
  async_function_resolve, async_generator_await, async_generator_await_reject_closure,
  async_generator_await_resolve_closure, async_generator_prototype_next,
  async_generator_prototype_return, async_generator_prototype_throw, async_generator_reject,
  async_generator_resolve, async_generator_resume_next, async_generator_return,
  async_generator_return_closed_reject_closure, async_generator_return_closed_resolve_closure,
  async_generator_return_resolve_closure, async_generator_yield_with_await,
  async_generator_yield_with_await_resolve_closure, async_iterator_value_unwrap,
  async_module_evaluate, call_async_module_fulfilled, call_async_module_rejected,
  call_iterator_with_feedback_lazy_deopt_continuation, disposable_stack_constructor,
  disposable_stack_prototype_adopt, disposable_stack_prototype_defer,
  disposable_stack_prototype_dispose, disposable_stack_prototype_get_disposed,
  disposable_stack_prototype_move, disposable_stack_prototype_use, enqueue_microtask,
  for_of_next_load_done_lazy_deopt_continuation, for_of_next_load_value_eager_deopt_continuation,
  for_of_next_load_value_lazy_deopt_continuation, for_of_next_result_deopt_continuation,
  fulfill_promise, generator_prototype_next, generator_prototype_return, generator_prototype_throw,
  get_iterator_with_feedback_lazy_deopt_continuation, global_queue_microtask,
  iterable_to_fixed_array, iterable_to_fixed_array_for_wasm,
  iterable_to_fixed_array_with_symbol_lookup_slow, iterable_to_list,
  iterable_to_list_convert_holes, iterable_to_list_may_preserve_holes,
  iterable_to_list_with_symbol_lookup, new_promise_capability, perform_promise_then,
  perform_promise_then_function, promise_all, promise_all_resolve_element_closure,
  promise_all_settled, promise_all_settled_reject_element_closure,
  promise_all_settled_resolve_element_closure, promise_any, promise_any_reject_element_closure,
  promise_capability_default_reject, promise_capability_default_resolve, promise_catch_finally,
  promise_constructor, promise_constructor_lazy_deopt_continuation, promise_fulfill_reaction_job,
  promise_get_capabilities_executor, promise_prototype_catch, promise_prototype_finally,
  promise_prototype_then, promise_race, promise_reject, promise_reject_reaction_job,
  promise_resolve, promise_resolve_thenable_job, promise_resolve_trampoline, promise_then_finally,
  promise_thrower_finally, promise_try, promise_value_thunk_finally, promise_with_resolvers,
  reject_promise, resolve_promise, resume_generator_baseline,
  resume_generator_trampoline_with_catch, run_microtasks, string_list_from_iterable,
  suspend_generator_baseline,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_promise_state_and_hooks() -> Void {
  let pending = PromiseState::Pending;
  let fulfilled = PromiseState::Fulfilled;
  let rejected = PromiseState::Rejected;

  assert!(pending.is_pending());
  assert!(!pending.is_settled());
  assert!(!pending.is_fulfilled());
  assert!(!pending.is_rejected());

  assert!(!fulfilled.is_pending());
  assert!(fulfilled.is_settled());
  assert!(fulfilled.is_fulfilled());
  assert!(!fulfilled.is_rejected());

  assert!(!rejected.is_pending());
  assert!(rejected.is_settled());
  assert!(rejected.is_rejected());
  assert!(!rejected.is_fulfilled());

  assert_eq!(PromiseState::from_str("Pending").unwrap(), pending);
  assert_eq!(PromiseState::from_str("Fulfilled").unwrap(), fulfilled);
  assert_eq!(PromiseState::from_str("Rejected").unwrap(), rejected);

  assert_eq!(PromiseHookType::Init as u8, 0);
  assert_eq!(PromiseHookType::Resolve as u8, 1);
  assert_eq!(PromiseHookType::Before as u8, 2);
  assert_eq!(PromiseHookType::After as u8, 3);

  assert!(PromiseHookType::Init.is_init());
  assert!(PromiseHookType::Resolve.is_resolve());
  assert!(PromiseHookType::Before.is_before());
  assert!(PromiseHookType::After.is_after());

  assert_eq!(
    PromiseHookType::from_str("Init").unwrap(),
    PromiseHookType::Init
  );
  assert_eq!(
    PromiseHookType::from_str("Resolve").unwrap(),
    PromiseHookType::Resolve
  );

  info!("test_promise_state_and_hooks passed");
  OK
}

#[test]
fn test_promise_builtins() -> Void {
  let builtins: Vec<_> = PromiseBuiltin::iter().collect();
  assert_eq!(builtins.len(), 34);

  let mut combinators = 0;
  let mut proto_methods = 0;
  let mut reaction_jobs = 0;
  let mut ctors = 0;

  for b in &builtins {
    let name = b.name();
    let display_str = b.to_string();
    assert_eq!(name, display_str.as_str());

    let parsed = PromiseBuiltin::from_str(name).unwrap();
    assert_eq!(*b, parsed);

    let desc = b.descriptor_name();
    assert!(desc.starts_with("Builtin_"));
    assert!(desc.ends_with("_InterfaceDescriptor"));

    // Promise 内建主要由 Torque DSL 定义，未入全局 C++ BUILTIN_TABLE
    assert_eq!(b.global_id(), None);

    if b.is_combinator() {
      combinators += 1;
    }
    if b.is_prototype_method() {
      proto_methods += 1;
    }
    if b.is_reaction_job() {
      reaction_jobs += 1;
    }
    if b.is_constructor_or_capability() {
      ctors += 1;
    }
  }

  assert_eq!(combinators, 4);
  assert_eq!(proto_methods, 3);
  assert_eq!(reaction_jobs, 3);
  assert_eq!(ctors, 4);

  // 工厂函数全覆盖断言
  assert_eq!(resolve_promise(), PromiseBuiltin::ResolvePromise);
  assert_eq!(fulfill_promise(), PromiseBuiltin::FulfillPromise);
  assert_eq!(reject_promise(), PromiseBuiltin::RejectPromise);
  assert_eq!(
    new_promise_capability(),
    PromiseBuiltin::NewPromiseCapability
  );
  assert_eq!(
    promise_capability_default_reject(),
    PromiseBuiltin::PromiseCapabilityDefaultReject
  );
  assert_eq!(
    promise_capability_default_resolve(),
    PromiseBuiltin::PromiseCapabilityDefaultResolve
  );
  assert_eq!(
    perform_promise_then_function(),
    PromiseBuiltin::PerformPromiseThenFunction
  );
  assert_eq!(perform_promise_then(), PromiseBuiltin::PerformPromiseThen);
  assert_eq!(promise_reject(), PromiseBuiltin::PromiseReject);
  assert_eq!(
    promise_get_capabilities_executor(),
    PromiseBuiltin::PromiseGetCapabilitiesExecutor
  );
  assert_eq!(
    promise_constructor_lazy_deopt_continuation(),
    PromiseBuiltin::PromiseConstructorLazyDeoptContinuation
  );
  assert_eq!(
    promise_all_resolve_element_closure(),
    PromiseBuiltin::PromiseAllResolveElementClosure
  );
  assert_eq!(
    promise_all_settled_resolve_element_closure(),
    PromiseBuiltin::PromiseAllSettledResolveElementClosure
  );
  assert_eq!(
    promise_all_settled_reject_element_closure(),
    PromiseBuiltin::PromiseAllSettledRejectElementClosure
  );
  assert_eq!(promise_all(), PromiseBuiltin::PromiseAll);
  assert_eq!(promise_all_settled(), PromiseBuiltin::PromiseAllSettled);
  assert_eq!(
    promise_any_reject_element_closure(),
    PromiseBuiltin::PromiseAnyRejectElementClosure
  );
  assert_eq!(promise_any(), PromiseBuiltin::PromiseAny);
  assert_eq!(promise_constructor(), PromiseBuiltin::PromiseConstructor);
  assert_eq!(
    promise_prototype_catch(),
    PromiseBuiltin::PromisePrototypeCatch
  );
  assert_eq!(
    promise_value_thunk_finally(),
    PromiseBuiltin::PromiseValueThunkFinally
  );
  assert_eq!(
    promise_thrower_finally(),
    PromiseBuiltin::PromiseThrowerFinally
  );
  assert_eq!(promise_catch_finally(), PromiseBuiltin::PromiseCatchFinally);
  assert_eq!(promise_then_finally(), PromiseBuiltin::PromiseThenFinally);
  assert_eq!(
    promise_prototype_finally(),
    PromiseBuiltin::PromisePrototypeFinally
  );
  assert_eq!(
    promise_resolve_thenable_job(),
    PromiseBuiltin::PromiseResolveThenableJob
  );
  assert_eq!(promise_race(), PromiseBuiltin::PromiseRace);
  assert_eq!(
    promise_fulfill_reaction_job(),
    PromiseBuiltin::PromiseFulfillReactionJob
  );
  assert_eq!(
    promise_reject_reaction_job(),
    PromiseBuiltin::PromiseRejectReactionJob
  );
  assert_eq!(
    promise_resolve_trampoline(),
    PromiseBuiltin::PromiseResolveTrampoline
  );
  assert_eq!(promise_resolve(), PromiseBuiltin::PromiseResolve);
  assert_eq!(
    promise_prototype_then(),
    PromiseBuiltin::PromisePrototypeThen
  );
  assert_eq!(promise_try(), PromiseBuiltin::PromiseTry);
  assert_eq!(
    promise_with_resolvers(),
    PromiseBuiltin::PromiseWithResolvers
  );

  assert_eq!(PROMISE_TQ_DECLS.len(), 80);

  info!("test_promise_builtins passed");
  OK
}

#[test]
fn test_async_builtins() -> Void {
  let builtins: Vec<_> = AsyncBuiltin::iter().collect();
  assert_eq!(builtins.len(), 56);

  let mut async_funcs = 0;
  let mut async_gens = 0;
  let mut sync_gens = 0;
  let mut disposables = 0;
  let mut microtasks = 0;

  for b in &builtins {
    let name = b.name();
    let display_str = b.to_string();
    assert_eq!(name, display_str.as_str());

    let parsed = AsyncBuiltin::from_str(name).unwrap();
    assert_eq!(*b, parsed);

    let desc = b.descriptor_name();
    assert!(desc.starts_with("Builtin_"));
    assert!(desc.ends_with("_InterfaceDescriptor"));

    // Async / Generator / Microtask 内建全量在 BUILTIN_TABLE 登记
    assert!(b.global_id().is_some());

    if b.is_async_function() {
      async_funcs += 1;
    }
    if b.is_async_generator() {
      async_gens += 1;
    }
    if b.is_generator() {
      sync_gens += 1;
    }
    if b.is_disposable_stack() {
      disposables += 1;
    }
    if b.is_microtask() {
      microtasks += 1;
    }
  }

  assert_eq!(async_funcs, 7);
  assert_eq!(async_gens, 15);
  assert_eq!(sync_gens, 6);
  assert_eq!(disposables, 17);
  assert_eq!(microtasks, 3);

  // 特殊变体与 AwaitBehavior
  assert_eq!(
    AsyncBuiltin::ResumeGeneratorTrampolineWithCatch.to_string(),
    "ResumeGeneratorTrampoline_WithCatch"
  );
  assert_eq!(AwaitBehavior::IgnorePredefinedFunction as u8, 0);
  assert_eq!(AwaitBehavior::UsePredefinedFunction as u8, 1);
  assert_eq!(
    AwaitBehavior::NORMAL,
    AwaitBehavior::IgnorePredefinedFunction
  );
  assert_eq!(
    AwaitBehavior::RESUME_ONLY,
    AwaitBehavior::UsePredefinedFunction
  );
  assert!(AwaitBehavior::NORMAL.is_normal());
  assert!(!AwaitBehavior::NORMAL.is_resume_only());
  assert!(AwaitBehavior::RESUME_ONLY.is_resume_only());
  assert!(!AwaitBehavior::RESUME_ONLY.is_normal());
  assert_eq!(
    AwaitBehavior::from_str("Normal").unwrap(),
    AwaitBehavior::NORMAL
  );
  assert_eq!(
    AwaitBehavior::from_str("ResumeOnly").unwrap(),
    AwaitBehavior::RESUME_ONLY
  );

  // 全量覆盖 56 个工厂函数断言
  assert_eq!(
    async_disposable_stack_constructor(),
    AsyncBuiltin::AsyncDisposableStackConstructor
  );
  assert_eq!(
    async_disposable_stack_on_fulfilled(),
    AsyncBuiltin::AsyncDisposableStackOnFulfilled
  );
  assert_eq!(
    async_disposable_stack_on_rejected(),
    AsyncBuiltin::AsyncDisposableStackOnRejected
  );
  assert_eq!(
    async_disposable_stack_prototype_adopt(),
    AsyncBuiltin::AsyncDisposableStackPrototypeAdopt
  );
  assert_eq!(
    async_disposable_stack_prototype_defer(),
    AsyncBuiltin::AsyncDisposableStackPrototypeDefer
  );
  assert_eq!(
    async_disposable_stack_prototype_dispose_async(),
    AsyncBuiltin::AsyncDisposableStackPrototypeDisposeAsync
  );
  assert_eq!(
    async_disposable_stack_prototype_get_disposed(),
    AsyncBuiltin::AsyncDisposableStackPrototypeGetDisposed
  );
  assert_eq!(
    async_disposable_stack_prototype_move(),
    AsyncBuiltin::AsyncDisposableStackPrototypeMove
  );
  assert_eq!(
    async_disposable_stack_prototype_use(),
    AsyncBuiltin::AsyncDisposableStackPrototypeUse
  );
  assert_eq!(
    async_dispose_from_sync_dispose(),
    AsyncBuiltin::AsyncDisposeFromSyncDispose
  );
  assert_eq!(
    async_from_sync_iterator_close_sync_and_rethrow(),
    AsyncBuiltin::AsyncFromSyncIteratorCloseSyncAndRethrow
  );
  assert_eq!(
    async_from_sync_iterator_prototype_next(),
    AsyncBuiltin::AsyncFromSyncIteratorPrototypeNext
  );
  assert_eq!(
    async_from_sync_iterator_prototype_return(),
    AsyncBuiltin::AsyncFromSyncIteratorPrototypeReturn
  );
  assert_eq!(
    async_from_sync_iterator_prototype_throw(),
    AsyncBuiltin::AsyncFromSyncIteratorPrototypeThrow
  );
  assert_eq!(async_function_await(), AsyncBuiltin::AsyncFunctionAwait);
  assert_eq!(
    async_function_await_reject_closure(),
    AsyncBuiltin::AsyncFunctionAwaitRejectClosure
  );
  assert_eq!(
    async_function_await_resolve_closure(),
    AsyncBuiltin::AsyncFunctionAwaitResolveClosure
  );
  assert_eq!(async_function_enter(), AsyncBuiltin::AsyncFunctionEnter);
  assert_eq!(
    async_function_lazy_deopt_continuation(),
    AsyncBuiltin::AsyncFunctionLazyDeoptContinuation
  );
  assert_eq!(async_function_reject(), AsyncBuiltin::AsyncFunctionReject);
  assert_eq!(async_function_resolve(), AsyncBuiltin::AsyncFunctionResolve);
  assert_eq!(async_generator_await(), AsyncBuiltin::AsyncGeneratorAwait);
  assert_eq!(
    async_generator_await_reject_closure(),
    AsyncBuiltin::AsyncGeneratorAwaitRejectClosure
  );
  assert_eq!(
    async_generator_await_resolve_closure(),
    AsyncBuiltin::AsyncGeneratorAwaitResolveClosure
  );
  assert_eq!(
    async_generator_prototype_next(),
    AsyncBuiltin::AsyncGeneratorPrototypeNext
  );
  assert_eq!(
    async_generator_prototype_return(),
    AsyncBuiltin::AsyncGeneratorPrototypeReturn
  );
  assert_eq!(
    async_generator_prototype_throw(),
    AsyncBuiltin::AsyncGeneratorPrototypeThrow
  );
  assert_eq!(async_generator_reject(), AsyncBuiltin::AsyncGeneratorReject);
  assert_eq!(
    async_generator_resolve(),
    AsyncBuiltin::AsyncGeneratorResolve
  );
  assert_eq!(
    async_generator_resume_next(),
    AsyncBuiltin::AsyncGeneratorResumeNext
  );
  assert_eq!(async_generator_return(), AsyncBuiltin::AsyncGeneratorReturn);
  assert_eq!(
    async_generator_return_closed_reject_closure(),
    AsyncBuiltin::AsyncGeneratorReturnClosedRejectClosure
  );
  assert_eq!(
    async_generator_return_closed_resolve_closure(),
    AsyncBuiltin::AsyncGeneratorReturnClosedResolveClosure
  );
  assert_eq!(
    async_generator_return_resolve_closure(),
    AsyncBuiltin::AsyncGeneratorReturnResolveClosure
  );
  assert_eq!(
    async_generator_yield_with_await(),
    AsyncBuiltin::AsyncGeneratorYieldWithAwait
  );
  assert_eq!(
    async_generator_yield_with_await_resolve_closure(),
    AsyncBuiltin::AsyncGeneratorYieldWithAwaitResolveClosure
  );
  assert_eq!(
    async_iterator_value_unwrap(),
    AsyncBuiltin::AsyncIteratorValueUnwrap
  );
  assert_eq!(async_module_evaluate(), AsyncBuiltin::AsyncModuleEvaluate);
  assert_eq!(
    call_async_module_fulfilled(),
    AsyncBuiltin::CallAsyncModuleFulfilled
  );
  assert_eq!(
    call_async_module_rejected(),
    AsyncBuiltin::CallAsyncModuleRejected
  );
  assert_eq!(
    disposable_stack_constructor(),
    AsyncBuiltin::DisposableStackConstructor
  );
  assert_eq!(
    disposable_stack_prototype_adopt(),
    AsyncBuiltin::DisposableStackPrototypeAdopt
  );
  assert_eq!(
    disposable_stack_prototype_defer(),
    AsyncBuiltin::DisposableStackPrototypeDefer
  );
  assert_eq!(
    disposable_stack_prototype_dispose(),
    AsyncBuiltin::DisposableStackPrototypeDispose
  );
  assert_eq!(
    disposable_stack_prototype_get_disposed(),
    AsyncBuiltin::DisposableStackPrototypeGetDisposed
  );
  assert_eq!(
    disposable_stack_prototype_move(),
    AsyncBuiltin::DisposableStackPrototypeMove
  );
  assert_eq!(
    disposable_stack_prototype_use(),
    AsyncBuiltin::DisposableStackPrototypeUse
  );
  assert_eq!(enqueue_microtask(), AsyncBuiltin::EnqueueMicrotask);
  assert_eq!(
    generator_prototype_next(),
    AsyncBuiltin::GeneratorPrototypeNext
  );
  assert_eq!(
    generator_prototype_return(),
    AsyncBuiltin::GeneratorPrototypeReturn
  );
  assert_eq!(
    generator_prototype_throw(),
    AsyncBuiltin::GeneratorPrototypeThrow
  );
  assert_eq!(global_queue_microtask(), AsyncBuiltin::GlobalQueueMicrotask);
  assert_eq!(
    resume_generator_baseline(),
    AsyncBuiltin::ResumeGeneratorBaseline
  );
  assert_eq!(
    resume_generator_trampoline_with_catch(),
    AsyncBuiltin::ResumeGeneratorTrampolineWithCatch
  );
  assert_eq!(run_microtasks(), AsyncBuiltin::RunMicrotasks);
  assert_eq!(
    suspend_generator_baseline(),
    AsyncBuiltin::SuspendGeneratorBaseline
  );

  assert_eq!(ASYNC_TQ_DECLS.len(), 12);

  info!("test_async_builtins passed");
  OK
}

#[test]
fn test_iterator_builtins() -> Void {
  let builtins: Vec<_> = IteratorBuiltin::iter().collect();
  assert_eq!(builtins.len(), 14);

  let mut deopt_conts = 0;
  let mut iter_to_lists = 0;

  for b in &builtins {
    let name = b.name();
    let display_str = b.to_string();
    assert_eq!(name, display_str.as_str());

    let parsed = IteratorBuiltin::from_str(name).unwrap();
    assert_eq!(*b, parsed);

    let desc = b.descriptor_name();
    assert!(desc.starts_with("Builtin_"));
    assert!(desc.ends_with("_InterfaceDescriptor"));

    // 除 IterableToFixedArrayForWasm (IF_WASM 条件编译) 外均在 BUILTIN_TABLE 登记
    if *b == IteratorBuiltin::IterableToFixedArrayForWasm {
      assert_eq!(b.global_id(), None);
    } else {
      assert!(b.global_id().is_some());
    }

    if b.is_deopt_continuation() {
      deopt_conts += 1;
    }
    if b.is_iterable_to_list() {
      iter_to_lists += 1;
    }
  }

  assert_eq!(deopt_conts, 6);
  assert_eq!(iter_to_lists, 8);

  // 全量 14 个工厂函数断言覆盖
  assert_eq!(
    call_iterator_with_feedback_lazy_deopt_continuation(),
    IteratorBuiltin::CallIteratorWithFeedbackLazyDeoptContinuation
  );
  assert_eq!(
    for_of_next_load_done_lazy_deopt_continuation(),
    IteratorBuiltin::ForOfNextLoadDoneLazyDeoptContinuation
  );
  assert_eq!(
    for_of_next_load_value_eager_deopt_continuation(),
    IteratorBuiltin::ForOfNextLoadValueEagerDeoptContinuation
  );
  assert_eq!(
    for_of_next_load_value_lazy_deopt_continuation(),
    IteratorBuiltin::ForOfNextLoadValueLazyDeoptContinuation
  );
  assert_eq!(
    for_of_next_result_deopt_continuation(),
    IteratorBuiltin::ForOfNextResultDeoptContinuation
  );
  assert_eq!(
    get_iterator_with_feedback_lazy_deopt_continuation(),
    IteratorBuiltin::GetIteratorWithFeedbackLazyDeoptContinuation
  );
  assert_eq!(
    iterable_to_fixed_array(),
    IteratorBuiltin::IterableToFixedArray
  );
  assert_eq!(
    iterable_to_fixed_array_for_wasm(),
    IteratorBuiltin::IterableToFixedArrayForWasm
  );
  assert_eq!(
    iterable_to_fixed_array_with_symbol_lookup_slow(),
    IteratorBuiltin::IterableToFixedArrayWithSymbolLookupSlow
  );
  assert_eq!(iterable_to_list(), IteratorBuiltin::IterableToList);
  assert_eq!(
    iterable_to_list_convert_holes(),
    IteratorBuiltin::IterableToListConvertHoles
  );
  assert_eq!(
    iterable_to_list_may_preserve_holes(),
    IteratorBuiltin::IterableToListMayPreserveHoles
  );
  assert_eq!(
    iterable_to_list_with_symbol_lookup(),
    IteratorBuiltin::IterableToListWithSymbolLookup
  );
  assert_eq!(
    string_list_from_iterable(),
    IteratorBuiltin::StringListFromIterable
  );

  assert_eq!(ITERATOR_TQ_DECLS.len(), 27);

  info!("test_iterator_builtins passed");
  OK
}

#[test]
fn test_error_formatting() -> Void {
  let err1 = Error::AlreadyResolvedOrRejected;
  assert_eq!(format!("{err1}"), "Promise already resolved or rejected");

  let err2 = Error::InvalidCapability("missing reject function".into());
  assert_eq!(
    format!("{err2}"),
    "Invalid promise capability: missing reject function"
  );

  let err3 = Error::ReactionFailed("detached context".into());
  assert_eq!(
    format!("{err3}"),
    "Promise reaction failed: detached context"
  );

  let err4 = Error::IteratorClosed;
  assert_eq!(format!("{err4}"), "Iterator closed unexpectedly");

  info!("test_error_formatting passed");
  OK
}
