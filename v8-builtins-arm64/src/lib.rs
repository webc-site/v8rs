// 由 tool/src/builtins/builtins-arm64.js 生成; 来源 v8/src/builtins 下 1 文件
// 模块索引
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

#![forbid(unsafe_code)]

mod builtins_arm64;
mod entry;

pub use builtins_arm64::{
  ASM_GENERATE_ENTRIES, CC_BUILTINS, CLASSES, CcBuiltin, DebugBreakKind, OsrSourceTier,
};
pub use entry::{
  Builtin as Arm64Builtin, adaptor, baseline_on_stack_replacement, baseline_out_of_line_prologue,
  baseline_out_of_line_prologue_deopt, c_entry, call, call_api_accessor_impl,
  call_api_callback_impl, call_bound_function_impl, call_function,
  call_or_construct_forward_varargs, call_or_construct_varargs, construct,
  construct_bound_function, construct_forward_all_args_impl, construct_function,
  constructed_non_constructable, continue_to_code_stub_builtin,
  continue_to_code_stub_builtin_with_result, continue_to_java_script_builtin,
  continue_to_java_script_builtin_with_result, deoptimization_entry_eager,
  deoptimization_entry_lazy, deoptimization_entry_lazy_after_fast_call, direct_c_entry,
  double_to_i, function_prototype_apply, function_prototype_call, interpreter_enter_at_bytecode,
  interpreter_enter_at_next_bytecode, interpreter_entry_trampoline,
  interpreter_on_stack_replacement, interpreter_on_stack_replacement_to_baseline,
  interpreter_push_args_then_call_impl, interpreter_push_args_then_construct_impl,
  interpreter_push_args_then_fast_construct_function, js_builtins_construct_stub,
  js_construct_entry, js_construct_entry_trampoline, js_construct_stub_generic, js_entry,
  js_entry_trampoline, js_run_microtasks_entry, js_to_wasm_stress_switch_stacks_asm,
  js_to_wasm_wrapper_asm, maglev_function_entry_stack_check, notify_deoptimized, reflect_apply,
  reflect_construct, restart_frame_trampoline, resume_generator_trampoline,
  run_microtasks_trampoline, wasm_compile_lazy, wasm_debug_break, wasm_fx_resume,
  wasm_fx_resume_throw, wasm_fx_resume_throw_ref, wasm_fx_return, wasm_fx_suspend, wasm_fx_switch,
  wasm_handle_stack_overflow, wasm_liftoff_frame_setup, wasm_on_stack_replace, wasm_reject,
  wasm_resume, wasm_return_from_segment, wasm_return_promise_on_suspend_asm, wasm_suspend,
  wasm_to_js_wrapper_asm, wasm_trap_handler_landing_pad, wasm_trap_handler_throw_trap,
};

// 死面导出与冗余依赖已清理: Error/Result 全 src 零构造零消费 (轮7 W7BAM 整改 nit2)
