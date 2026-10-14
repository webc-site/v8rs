use std::str::FromStr;

use aok::{OK, Void};
use strum::IntoEnumIterator;
use v8_builtins_arm64::*;
use v8_execution_frames::StackFrameType;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_all_factory_functions() -> Void {
  let list = [
    (adaptor(), "Adaptor"),
    (
      baseline_on_stack_replacement(),
      "BaselineOnStackReplacement",
    ),
    (baseline_out_of_line_prologue(), "BaselineOutOfLinePrologue"),
    (
      baseline_out_of_line_prologue_deopt(),
      "BaselineOutOfLinePrologueDeopt",
    ),
    (c_entry(), "CEntry"),
    (call(), "Call"),
    (call_api_accessor_impl(), "CallApiAccessorImpl"),
    (call_api_callback_impl(), "CallApiCallbackImpl"),
    (call_bound_function_impl(), "CallBoundFunctionImpl"),
    (call_function(), "CallFunction"),
    (
      call_or_construct_forward_varargs(),
      "CallOrConstructForwardVarargs",
    ),
    (call_or_construct_varargs(), "CallOrConstructVarargs"),
    (construct(), "Construct"),
    (construct_bound_function(), "ConstructBoundFunction"),
    (
      construct_forward_all_args_impl(),
      "ConstructForwardAllArgsImpl",
    ),
    (construct_function(), "ConstructFunction"),
    (
      constructed_non_constructable(),
      "ConstructedNonConstructable",
    ),
    (continue_to_code_stub_builtin(), "ContinueToCodeStubBuiltin"),
    (
      continue_to_code_stub_builtin_with_result(),
      "ContinueToCodeStubBuiltinWithResult",
    ),
    (
      continue_to_java_script_builtin(),
      "ContinueToJavaScriptBuiltin",
    ),
    (
      continue_to_java_script_builtin_with_result(),
      "ContinueToJavaScriptBuiltinWithResult",
    ),
    (deoptimization_entry_eager(), "DeoptimizationEntry_Eager"),
    (deoptimization_entry_lazy(), "DeoptimizationEntry_Lazy"),
    (
      deoptimization_entry_lazy_after_fast_call(),
      "DeoptimizationEntry_LazyAfterFastCall",
    ),
    (direct_c_entry(), "DirectCEntry"),
    (double_to_i(), "DoubleToI"),
    (function_prototype_apply(), "FunctionPrototypeApply"),
    (function_prototype_call(), "FunctionPrototypeCall"),
    (
      interpreter_enter_at_bytecode(),
      "InterpreterEnterAtBytecode",
    ),
    (
      interpreter_enter_at_next_bytecode(),
      "InterpreterEnterAtNextBytecode",
    ),
    (interpreter_entry_trampoline(), "InterpreterEntryTrampoline"),
    (
      interpreter_on_stack_replacement(),
      "InterpreterOnStackReplacement",
    ),
    (
      interpreter_on_stack_replacement_to_baseline(),
      "InterpreterOnStackReplacement_ToBaseline",
    ),
    (
      interpreter_push_args_then_call_impl(),
      "InterpreterPushArgsThenCallImpl",
    ),
    (
      interpreter_push_args_then_construct_impl(),
      "InterpreterPushArgsThenConstructImpl",
    ),
    (
      interpreter_push_args_then_fast_construct_function(),
      "InterpreterPushArgsThenFastConstructFunction",
    ),
    (js_builtins_construct_stub(), "JSBuiltinsConstructStub"),
    (js_construct_entry(), "JSConstructEntry"),
    (
      js_construct_entry_trampoline(),
      "JSConstructEntryTrampoline",
    ),
    (js_construct_stub_generic(), "JSConstructStubGeneric"),
    (js_entry(), "JSEntry"),
    (js_entry_trampoline(), "JSEntryTrampoline"),
    (js_run_microtasks_entry(), "JSRunMicrotasksEntry"),
    (
      js_to_wasm_stress_switch_stacks_asm(),
      "JSToWasmStressSwitchStacksAsm",
    ),
    (js_to_wasm_wrapper_asm(), "JSToWasmWrapperAsm"),
    (
      maglev_function_entry_stack_check(),
      "MaglevFunctionEntryStackCheck",
    ),
    (notify_deoptimized(), "NotifyDeoptimized"),
    (reflect_apply(), "ReflectApply"),
    (reflect_construct(), "ReflectConstruct"),
    (restart_frame_trampoline(), "RestartFrameTrampoline"),
    (resume_generator_trampoline(), "ResumeGeneratorTrampoline"),
    (run_microtasks_trampoline(), "RunMicrotasksTrampoline"),
    (wasm_compile_lazy(), "WasmCompileLazy"),
    (wasm_debug_break(), "WasmDebugBreak"),
    (wasm_fx_resume(), "WasmFXResume"),
    (wasm_fx_resume_throw(), "WasmFXResumeThrow"),
    (wasm_fx_resume_throw_ref(), "WasmFXResumeThrowRef"),
    (wasm_fx_return(), "WasmFXReturn"),
    (wasm_fx_suspend(), "WasmFXSuspend"),
    (wasm_fx_switch(), "WasmFXSwitch"),
    (wasm_handle_stack_overflow(), "WasmHandleStackOverflow"),
    (wasm_liftoff_frame_setup(), "WasmLiftoffFrameSetup"),
    (wasm_on_stack_replace(), "WasmOnStackReplace"),
    (wasm_reject(), "WasmReject"),
    (wasm_resume(), "WasmResume"),
    (wasm_return_from_segment(), "WasmReturnFromSegment"),
    (
      wasm_return_promise_on_suspend_asm(),
      "WasmReturnPromiseOnSuspendAsm",
    ),
    (wasm_suspend(), "WasmSuspend"),
    (wasm_to_js_wrapper_asm(), "WasmToJsWrapperAsm"),
    (wasm_trap_handler_landing_pad(), "WasmTrapHandlerLandingPad"),
    (wasm_trap_handler_throw_trap(), "WasmTrapHandlerThrowTrap"),
  ];

  assert_eq!(list.len(), 71);
  for (b, expected_name) in list {
    assert_eq!(b.name(), expected_name);
  }
  OK
}

#[test]
fn test_strum_roundtrip_and_names() -> Void {
  let mut count = 0;
  for b in Arm64Builtin::iter() {
    count += 1;
    let name = b.name();
    let parsed = Arm64Builtin::from_str(name)?;
    assert_eq!(b, parsed);
  }
  assert_eq!(count, 71);
  OK
}

#[test]
fn test_predicate_partition() -> Void {
  let mut total = 0;
  let mut js_entry_count = 0;
  let mut interp_count = 0;
  let mut call_count = 0;
  let mut deopt_count = 0;
  let mut c_entry_count = 0;
  let mut wasm_count = 0;

  for b in Arm64Builtin::iter() {
    total += 1;
    let mut hits = 0;
    if b.is_js_entry_or_trampoline() {
      js_entry_count += 1;
      hits += 1;
    }
    if b.is_interpreter_or_baseline() {
      interp_count += 1;
      hits += 1;
    }
    if b.is_call_or_construct() {
      call_count += 1;
      hits += 1;
    }
    if b.is_deopt_or_osr() {
      deopt_count += 1;
      hits += 1;
    }
    if b.is_c_entry_or_api() {
      c_entry_count += 1;
      hits += 1;
    }
    if b.is_wasm() {
      wasm_count += 1;
      hits += 1;
    }

    assert_eq!(
      hits, 1,
      "Builtin {:?} must match exactly one primary group, matched {}",
      b, hits
    );
  }

  assert_eq!(total, 71);
  assert_eq!(js_entry_count, 8);
  assert_eq!(interp_count, 10);
  assert_eq!(call_count, 15);
  assert_eq!(deopt_count, 11);
  assert_eq!(c_entry_count, 6);
  assert_eq!(wasm_count, 21);
  assert_eq!(
    js_entry_count + interp_count + call_count + deopt_count + c_entry_count + wasm_count,
    71
  );

  OK
}

#[test]
fn test_global_registry_link() -> Void {
  let mut global_matched = 0;

  for b in Arm64Builtin::iter() {
    if let Some(gid) = b.global_id() {
      global_matched += 1;
      assert!(gid < 1200);
    }
    // 所有内建均支持 descriptor_name 获取
    let desc = b.descriptor_name();
    assert!(!desc.is_empty());
  }

  // arm64 71 项中有 37 项在 v8-builtins-core 的 BUILTIN_TABLE 中有对应内建定义
  assert_eq!(global_matched, 37);
  assert!(js_entry().global_id().is_some());
  assert!(direct_c_entry().global_id().is_some());
  assert!(construct().global_id().is_some());
  assert!(c_entry().global_id().is_none());
  assert!(wasm_compile_lazy().global_id().is_none());
  assert!(adaptor().global_id().is_none());
  OK
}

#[test]
fn test_architecture_artifacts() -> Void {
  assert_eq!(ASM_GENERATE_ENTRIES.len(), 71);
  assert!(CC_BUILTINS.is_empty());
  assert_eq!(CLASSES.len(), 4);

  let osr_interp = OsrSourceTier::Interpreter;
  let osr_base = OsrSourceTier::Baseline;
  assert_ne!(osr_interp, osr_base);
  assert_eq!(osr_interp.to_string(), "Interpreter");
  assert_eq!(osr_base.to_string(), "Baseline");

  let dbg_break = DebugBreakKind::Break;
  let dbg_trap = DebugBreakKind::Trap;
  assert_ne!(dbg_break, dbg_trap);
  assert_eq!(dbg_break.to_string(), "Break");
  assert_eq!(dbg_trap.to_string(), "Trap");

  // 关联联动验证 v8-codegen-arm64 与 v8-execution-frames
  let root_reg = Arm64Builtin::root_register();
  assert_eq!(root_reg, v8_codegen_arm64::K_ROOT_REGISTER);
  assert_eq!(root_reg.code(), 26);

  assert!(StackFrameType::Interpreted.is_javascript());

  assert_eq!(
    js_entry().expected_frame_type(),
    Some(StackFrameType::Entry)
  );
  assert_eq!(
    interpreter_entry_trampoline().expected_frame_type(),
    Some(StackFrameType::Interpreted)
  );
  assert_eq!(c_entry().expected_frame_type(), None);

  OK
}
