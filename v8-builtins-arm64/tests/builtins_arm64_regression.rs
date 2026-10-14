// 复审环 round1 回归: 固化「arm64 汇编内建 entry 面」对原版 builtins-arm64.cc 的同名忠实性
// 来源: v8/src/builtins/arm64/builtins-arm64.cc 中 71 个 `void Builtins::Generate_<名>` 定义
// 说明: 本 crate 仅覆盖 entry 名面 (函数体不展开), 此回归守护名面不被增删/改名漂移

use aok::{OK, Void};
use strum::IntoEnumIterator;
use v8_builtins_arm64::*;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 `Builtins::Generate_*` 入口名地面真值 (71, 排序)
const CANONICAL_GENERATE: &[&str] = &[
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

/// 强化原 count-only oracle: 精确钉住 v8-builtins-core 全局表可解析/不可解析的成员集合
/// 不可解析 34 项 = 全部 Wasm* + Call/Construct 的 Impl/变体内建 (非独立 Builtins::Code 项)
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

/// 名面双射: Builtin::iter() 与 ASM_GENERATE_ENTRIES 均须精确等于原版 71 入口集
#[test]
fn test_entry_surface_matches_original_generate_list() -> Void {
  let from_enum = sorted_names(Arm64Builtin::iter().map(|b| b.name()));
  let from_asm = sorted_names(ASM_GENERATE_ENTRIES.iter().copied());

  assert_eq!(from_enum.len(), 71);
  assert_eq!(from_asm.len(), 71);
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

#[test]
fn test_global_registry_membership_exact() -> Void {
  let mut matched = Vec::with_capacity(37);
  let mut unmatched = Vec::with_capacity(34);
  for b in Arm64Builtin::iter() {
    if b.global_id().is_some() {
      matched.push(b.name());
    } else {
      unmatched.push(b.name());
    }
  }
  matched.sort_unstable();
  unmatched.sort_unstable();

  assert_eq!(matched.as_slice(), EXPECT_MATCHED, "可解析集合成员漂移");
  assert_eq!(unmatched.len(), 34);
  assert_eq!(
    unmatched.first().copied(),
    Some("Adaptor"),
    "不可解析集首个应为 Adaptor"
  );
  // 全部 Wasm* 汇编入口均不在全局 Code 表
  assert!(unmatched.iter().all(|n| matched.binary_search(n).is_err()));
  assert!(
    unmatched.iter().filter(|n| n.starts_with("Wasm")).count() >= 19,
    "Wasm* 应集中于不可解析集"
  );

  OK
}
