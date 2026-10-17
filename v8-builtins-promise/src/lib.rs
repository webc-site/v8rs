// 由 tool/src/builtins/builtins-promise.js 生成; 来源 v8/src/builtins 下 3 文件
// 模块索引
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

#![forbid(unsafe_code)]

mod async_gen;
mod error;
mod iterator;
mod promise;

pub use async_gen::{
  AwaitBehavior, Builtin as AsyncBuiltin, TQ_DECLS as ASYNC_TQ_DECLS,
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
  disposable_stack_constructor, disposable_stack_prototype_adopt, disposable_stack_prototype_defer,
  disposable_stack_prototype_dispose, disposable_stack_prototype_get_disposed,
  disposable_stack_prototype_move, disposable_stack_prototype_use, enqueue_microtask,
  generator_prototype_next, generator_prototype_return, generator_prototype_throw,
  global_queue_microtask, resume_generator_baseline, resume_generator_trampoline_with_catch,
  run_microtasks, suspend_generator_baseline,
};
pub use error::{Error, Result};
pub use iterator::{
  Builtin as IteratorBuiltin, TQ_DECLS as ITERATOR_TQ_DECLS,
  call_iterator_with_feedback_lazy_deopt_continuation,
  for_of_next_load_done_lazy_deopt_continuation, for_of_next_load_value_eager_deopt_continuation,
  for_of_next_load_value_lazy_deopt_continuation, for_of_next_result_deopt_continuation,
  get_iterator_with_feedback_lazy_deopt_continuation, iterable_to_fixed_array,
  iterable_to_fixed_array_for_wasm, iterable_to_fixed_array_with_symbol_lookup_slow,
  iterable_to_list, iterable_to_list_convert_holes, iterable_to_list_may_preserve_holes,
  iterable_to_list_with_symbol_lookup, string_list_from_iterable,
};
pub use promise::{
  Builtin as PromiseBuiltin, PromiseHookType, PromiseState, TQ_DECLS as PROMISE_TQ_DECLS,
  fulfill_promise, new_promise_capability, perform_promise_then, perform_promise_then_function,
  promise_all, promise_all_resolve_element_closure, promise_all_settled,
  promise_all_settled_reject_element_closure, promise_all_settled_resolve_element_closure,
  promise_any, promise_any_reject_element_closure, promise_capability_default_reject,
  promise_capability_default_resolve, promise_catch_finally, promise_constructor,
  promise_constructor_lazy_deopt_continuation, promise_fulfill_reaction_job,
  promise_get_capabilities_executor, promise_prototype_catch, promise_prototype_finally,
  promise_prototype_then, promise_race, promise_reject, promise_reject_reaction_job,
  promise_resolve, promise_resolve_thenable_job, promise_resolve_trampoline, promise_then_finally,
  promise_thrower_finally, promise_try, promise_value_thunk_finally, promise_with_resolvers,
  reject_promise, resolve_promise,
};
