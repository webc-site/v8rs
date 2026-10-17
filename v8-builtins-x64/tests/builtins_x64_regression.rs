// 复审环 round1 回归: 固化「x64 汇编内建 entry 名面 + 调度元数据」对原版 v8/src/builtins/x64/builtins-x64.cc 的同名忠实性
// 来源: 原版 73 个 `void Builtins::Generate_<名>` 定义 (builtins-x64.cc:44..5741)
// 说明: 本 crate 仅覆盖 entry 名面与元数据表 (函数体不展开, entry.rs:3 / lib.rs:3),
//       此回归守护名面不被增删/改名漂移, 并钉死 global 表可解析/不可解析成员集合与平台独有面事实。
// 轮7 W7BXM 整改追加: 钉死 expected_frame_type 按原版帧宏的精化面与谓词组归属订正面。

use aok::{OK, Void};
use strum::IntoEnumIterator;
use v8_builtins_x64::*;
use v8_execution_frames::StackFrameType as Frame;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 `Builtins::Generate_*` 入口名地面真值 (73, 字节序)
const CANONICAL_GENERATE: &[&str] = &[
  "AdaptShadowStackForDeopt",
  "Adaptor",
  "BaselineOnStackReplacement",
  "BaselineOutOfLinePrologue",
  "BaselineOutOfLinePrologueDeopt",
  "CEntry",
  "Call",
  "CallApiAccessorImpl",
  "CallApiCallbackImpl",
  "CallBoundFunctionImpl",
  "CallFunction",
  "CallOrConstructForwardVarargs",
  "CallOrConstructVarargs",
  "Construct",
  "ConstructBoundFunction",
  "ConstructForwardAllArgsImpl",
  "ConstructFunction",
  "ConstructedNonConstructable",
  "ContinueToCodeStubBuiltin",
  "ContinueToCodeStubBuiltinWithResult",
  "ContinueToJavaScriptBuiltin",
  "ContinueToJavaScriptBuiltinWithResult",
  "DeoptimizationEntry_Eager",
  "DeoptimizationEntry_Lazy",
  "DeoptimizationEntry_LazyAfterFastCall",
  "DirectCEntry",
  "DoubleToI",
  "DumpFrame",
  "FunctionPrototypeApply",
  "FunctionPrototypeCall",
  "InterpreterEnterAtBytecode",
  "InterpreterEnterAtNextBytecode",
  "InterpreterEntryTrampoline",
  "InterpreterOnStackReplacement",
  "InterpreterOnStackReplacement_ToBaseline",
  "InterpreterPushArgsThenCallImpl",
  "InterpreterPushArgsThenConstructImpl",
  "InterpreterPushArgsThenFastConstructFunction",
  "JSBuiltinsConstructStub",
  "JSConstructEntry",
  "JSConstructEntryTrampoline",
  "JSConstructStubGeneric",
  "JSEntry",
  "JSEntryTrampoline",
  "JSRunMicrotasksEntry",
  "JSToWasmStressSwitchStacksAsm",
  "JSToWasmWrapperAsm",
  "MaglevFunctionEntryStackCheck",
  "NotifyDeoptimized",
  "ReflectApply",
  "ReflectConstruct",
  "RestartFrameTrampoline",
  "ResumeGeneratorTrampoline",
  "RunMicrotasksTrampoline",
  "WasmCompileLazy",
  "WasmDebugBreak",
  "WasmFXResume",
  "WasmFXResumeThrow",
  "WasmFXResumeThrowRef",
  "WasmFXReturn",
  "WasmFXSuspend",
  "WasmFXSwitch",
  "WasmHandleStackOverflow",
  "WasmLiftoffFrameSetup",
  "WasmOnStackReplace",
  "WasmReject",
  "WasmResume",
  "WasmReturnFromSegment",
  "WasmReturnPromiseOnSuspendAsm",
  "WasmSuspend",
  "WasmToJsWrapperAsm",
  "WasmTrapHandlerLandingPad",
  "WasmTrapHandlerThrowTrap",
];

fn sorted_names<'a>(it: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
  let mut v: Vec<&str> = it.collect();
  v.sort_unstable();
  v
}

/// 强化原 count-only oracle: 精确钉住 v8_builtins_core::BUILTIN_TABLE 可解析/不可解析成员集合 (37/36)
/// 不可解析 36 项 = Call/Construct 系 Impl/变体 + Adaptor/CEntry/Maglev 栈检查 +
/// InterpreterPushArgsThen{Call,Construct}Impl + ConstructForwardAllArgsImpl +
/// 全 Wasm*/JSToWasm* 汇编入口 + x64 独有 AdaptShadowStackForDeopt/DumpFrame
const EXPECT_MATCHED: &[&str] = &[
  "BaselineOnStackReplacement",
  "BaselineOutOfLinePrologue",
  "BaselineOutOfLinePrologueDeopt",
  "Construct",
  "ConstructBoundFunction",
  "ConstructFunction",
  "ConstructedNonConstructable",
  "ContinueToCodeStubBuiltin",
  "ContinueToCodeStubBuiltinWithResult",
  "ContinueToJavaScriptBuiltin",
  "ContinueToJavaScriptBuiltinWithResult",
  "DeoptimizationEntry_Eager",
  "DeoptimizationEntry_Lazy",
  "DeoptimizationEntry_LazyAfterFastCall",
  "DirectCEntry",
  "DoubleToI",
  "FunctionPrototypeApply",
  "FunctionPrototypeCall",
  "InterpreterEnterAtBytecode",
  "InterpreterEnterAtNextBytecode",
  "InterpreterEntryTrampoline",
  "InterpreterOnStackReplacement",
  "InterpreterOnStackReplacement_ToBaseline",
  "InterpreterPushArgsThenFastConstructFunction",
  "JSBuiltinsConstructStub",
  "JSConstructEntry",
  "JSConstructEntryTrampoline",
  "JSConstructStubGeneric",
  "JSEntry",
  "JSEntryTrampoline",
  "JSRunMicrotasksEntry",
  "NotifyDeoptimized",
  "ReflectApply",
  "ReflectConstruct",
  "RestartFrameTrampoline",
  "ResumeGeneratorTrampoline",
  "RunMicrotasksTrampoline",
];

const EXPECT_UNMATCHED: &[&str] = &[
  "AdaptShadowStackForDeopt",
  "Adaptor",
  "CEntry",
  "Call",
  "CallApiAccessorImpl",
  "CallApiCallbackImpl",
  "CallBoundFunctionImpl",
  "CallFunction",
  "CallOrConstructForwardVarargs",
  "CallOrConstructVarargs",
  "ConstructForwardAllArgsImpl",
  "DumpFrame",
  "InterpreterPushArgsThenCallImpl",
  "InterpreterPushArgsThenConstructImpl",
  "JSToWasmStressSwitchStacksAsm",
  "JSToWasmWrapperAsm",
  "MaglevFunctionEntryStackCheck",
  "WasmCompileLazy",
  "WasmDebugBreak",
  "WasmFXResume",
  "WasmFXResumeThrow",
  "WasmFXResumeThrowRef",
  "WasmFXReturn",
  "WasmFXSuspend",
  "WasmFXSwitch",
  "WasmHandleStackOverflow",
  "WasmLiftoffFrameSetup",
  "WasmOnStackReplace",
  "WasmReject",
  "WasmResume",
  "WasmReturnFromSegment",
  "WasmReturnPromiseOnSuspendAsm",
  "WasmSuspend",
  "WasmToJsWrapperAsm",
  "WasmTrapHandlerLandingPad",
  "WasmTrapHandlerThrowTrap",
];

/// 名面双射: Builtin::iter() 与 ASM_GENERATE_ENTRIES 均须精确等于原版 73 入口集
#[test]
fn test_entry_surface_matches_original_generate_list() -> Void {
  let from_enum = sorted_names(X64Builtin::iter().map(|b| b.name()));
  let from_asm = sorted_names(ASM_GENERATE_ENTRIES.iter().copied());

  assert_eq!(from_enum.len(), 73);
  assert_eq!(from_asm.len(), 73);
  assert_eq!(
    from_enum.as_slice(),
    CANONICAL_GENERATE,
    "Builtin 枚举名漂移于原版 Generate_ 面"
  );
  assert_eq!(
    from_asm.as_slice(),
    CANONICAL_GENERATE,
    "ASM_GENERATE_ENTRIES 漂移于原版 Generate_ 面"
  );

  // 无重名 (双射前提: 有序切片相邻互异, O(N) 零分配)
  assert!(from_enum.windows(2).all(|w| w[0] != w[1]), "存在重复内建名");

  OK
}

/// 精确钉住 v8_builtins_core::BUILTIN_TABLE 可解析/不可解析成员集合 (37/36)
#[test]
fn test_global_registry_membership_exact() -> Void {
  let mut matched = Vec::with_capacity(37);
  let mut unmatched = Vec::with_capacity(36);
  for b in X64Builtin::iter() {
    if b.global_id().is_some() {
      matched.push(b.name());
    } else {
      unmatched.push(b.name());
    }
  }
  matched.sort_unstable();
  unmatched.sort_unstable();

  assert_eq!(matched.as_slice(), EXPECT_MATCHED, "可解析集合成员漂移");
  assert_eq!(
    unmatched.as_slice(),
    EXPECT_UNMATCHED,
    "不可解析集合成员漂移"
  );
  assert_eq!(unmatched.len(), 36);
  assert_eq!(
    unmatched.first().copied(),
    Some("AdaptShadowStackForDeopt"),
    "不可解析集首个应为 AdaptShadowStackForDeopt"
  );
  assert!(unmatched.iter().all(|n| matched.binary_search(n).is_err()));
  assert!(
    unmatched.iter().filter(|n| n.starts_with("Wasm")).count() >= 19,
    "Wasm* 应集中于不可解析集"
  );

  OK
}

/// 平台独有面与根寄存器事实 (原版对证):
/// - is_x64_specific 恰为 {AdaptShadowStackForDeopt, DumpFrame}: 二者仅存在于 builtins-x64.cc
///   (adapt_shadow_stack_for_deopt@cc:3168, dump_frame@cc:5388; builtins-arm64.cc 无同名 Generate_;
///   definitions.h:1615 IF_SHADOW_STACK / :2024 BUILTIN_LIST_DUMPLING)。
/// - root_register = r13 (v8/src/codegen/x64/register-x64.h:387 kRootRegister = r13), code 13。
#[test]
fn test_x64_specific_surface_and_root_register() -> Void {
  let specific = sorted_names(
    X64Builtin::iter()
      .filter(|b| b.is_x64_specific())
      .map(|b| b.name()),
  );
  assert_eq!(
    specific.as_slice(),
    &["AdaptShadowStackForDeopt", "DumpFrame"]
  );

  let root = X64Builtin::root_register();
  assert_eq!(root, v8_codegen_x64::R13);
  assert_eq!(
    root.code(),
    13,
    "原版 kRootRegister = r13 (register-x64.h:387)"
  );

  OK
}

/// 轮7 W7BXM 整改: 钉死 expected_frame_type 按原版 builtins-x64.cc 帧宏的精化面
/// (逐条出处见 entry.rs expected_frame_type 臂注释), 未精化成员为启发式兜底:
/// 兜底面随汇编体填波替换, 本用例同时守护两侧不漂移。
#[test]
fn test_expected_frame_type_refined_surface() -> Void {
  let refined = [
    (js_entry(), Some(Frame::Entry)),                    // cc:557-559
    (js_run_microtasks_entry(), Some(Frame::Entry)),     // cc:566-568
    (js_construct_entry(), Some(Frame::ConstructEntry)), // cc:561-563
    (js_entry_trampoline(), Some(Frame::Internal)),      // cc:620/694-695
    (js_construct_entry_trampoline(), Some(Frame::Internal)), // cc:620/698-699
    (run_microtasks_trampoline(), None),                 // cc:702-706 无帧尾调
    (resume_generator_trampoline(), Some(Frame::Internal)), // cc:894/908/919
    (restart_frame_trampoline(), Some(Frame::Interpreted)), // cc:5764
    (baseline_out_of_line_prologue(), Some(Frame::Baseline)), // cc:1899
    (baseline_out_of_line_prologue_deopt(), Some(Frame::Baseline)), // cc:2003
    (baseline_on_stack_replacement(), Some(Frame::Baseline)), // cc:3060-3067
    (maglev_function_entry_stack_check(), Some(Frame::Maglev)), // cc:3073-3080 frames.h:125
    (interpreter_entry_trampoline(), Some(Frame::Interpreted)), // cc:1139-1144
    (interpreter_enter_at_bytecode(), Some(Frame::Interpreted)),
    (
      interpreter_enter_at_next_bytecode(),
      Some(Frame::Interpreted),
    ),
    (
      interpreter_push_args_then_fast_construct_function(),
      Some(Frame::FastConstruct),
    ), // cc:1598-1599
    (
      continue_to_code_stub_builtin(),
      Some(Frame::BuiltinContinuation),
    ), // cc:2009-2085 frames.h:129
    (
      continue_to_code_stub_builtin_with_result(),
      Some(Frame::BuiltinContinuation),
    ),
    (
      continue_to_java_script_builtin(),
      Some(Frame::BuiltinContinuation),
    ),
    (
      continue_to_java_script_builtin_with_result(),
      Some(Frame::BuiltinContinuation),
    ),
    (notify_deoptimized(), Some(Frame::Internal)), // cc:2089
  ];
  for (b, expect) in refined {
    assert_eq!(b.expected_frame_type(), expect, "{} 精化帧型漂移", b.name());
  }

  // 启发式兜底面 (interpreter/baseline 组未逐臂精化成员): 真帧型随汇编体填波替换
  for b in [
    interpreter_push_args_then_call_impl(),
    interpreter_push_args_then_construct_impl(),
    construct_forward_all_args_impl(),
  ] {
    assert_eq!(
      b.expected_frame_type(),
      Some(Frame::Interpreted),
      "{} 兜底面漂移",
      b.name()
    );
  }

  // 非帧面组保持 None (粗类兜底)
  for b in [
    c_entry(),
    double_to_i(),
    call(),
    wasm_suspend(),
    adapt_shadow_stack_for_deopt(),
  ] {
    assert_eq!(
      b.expected_frame_type(),
      None,
      "{} 应保持无预期帧型",
      b.name()
    );
  }

  // 谓词组归属订正 (nit4): ContinueTo* 四项归 runtime 辅助组, 不再入 deopt 组
  for b in [
    continue_to_code_stub_builtin(),
    continue_to_code_stub_builtin_with_result(),
    continue_to_java_script_builtin(),
    continue_to_java_script_builtin_with_result(),
  ] {
    assert!(
      b.is_runtime_or_aux() && !b.is_deopt_or_osr(),
      "{} 组归属漂移",
      b.name()
    );
  }

  OK
}
