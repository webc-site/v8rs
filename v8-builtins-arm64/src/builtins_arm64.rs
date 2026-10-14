// 由 tool/src/builtins/arm64/builtins-arm64.cc.js 从 v8/src/builtins/arm64/builtins-arm64.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-arm64; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类 (汇编体推迟面占位: 登记原版 cc 面信息, 现波不填)
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 0 条) — 汇编体推迟面占位
///
/// 空表是设计态而非漏填: 原版 `LIST_ASSEMBLER_BUILTINS` 类宏登记属汇编体波,
/// 随 Generate_* 函数体填波一并派生; 名面权威清单见 ASM_GENERATE_ENTRIES。
/// 推迟面台账见 readme/zh.md。
pub const CC_BUILTINS: &[CcBuiltin] = &[];

/// 平台汇编入口 Builtins::Generate_<名> 清单 (共 71 条)
pub const ASM_GENERATE_ENTRIES: &[&str] = &[
  "Adaptor",
  "JSConstructStubGeneric",
  "JSBuiltinsConstructStub",
  "ConstructedNonConstructable",
  "ResumeGeneratorTrampoline",
  "JSEntry",
  "JSConstructEntry",
  "JSRunMicrotasksEntry",
  "JSEntryTrampoline",
  "JSConstructEntryTrampoline",
  "RunMicrotasksTrampoline",
  "BaselineOutOfLinePrologue",
  "BaselineOutOfLinePrologueDeopt",
  "InterpreterEntryTrampoline",
  "InterpreterPushArgsThenCallImpl",
  "InterpreterPushArgsThenConstructImpl",
  "ConstructForwardAllArgsImpl",
  "InterpreterPushArgsThenFastConstructFunction",
  "InterpreterEnterAtNextBytecode",
  "InterpreterEnterAtBytecode",
  "ContinueToCodeStubBuiltin",
  "ContinueToCodeStubBuiltinWithResult",
  "ContinueToJavaScriptBuiltin",
  "ContinueToJavaScriptBuiltinWithResult",
  "NotifyDeoptimized",
  "InterpreterOnStackReplacement",
  "BaselineOnStackReplacement",
  "MaglevFunctionEntryStackCheck",
  "FunctionPrototypeApply",
  "FunctionPrototypeCall",
  "ReflectApply",
  "ReflectConstruct",
  "CallOrConstructVarargs",
  "CallOrConstructForwardVarargs",
  "CallFunction",
  "CallBoundFunctionImpl",
  "Call",
  "ConstructFunction",
  "ConstructBoundFunction",
  "Construct",
  "WasmLiftoffFrameSetup",
  "WasmCompileLazy",
  "WasmDebugBreak",
  "WasmTrapHandlerThrowTrap",
  "WasmToJsWrapperAsm",
  "WasmTrapHandlerLandingPad",
  "WasmSuspend",
  "WasmResume",
  "WasmReject",
  "WasmFXResume",
  "WasmFXResumeThrow",
  "WasmFXResumeThrowRef",
  "WasmFXSuspend",
  "WasmFXSwitch",
  "WasmFXReturn",
  "WasmOnStackReplace",
  "JSToWasmWrapperAsm",
  "WasmReturnPromiseOnSuspendAsm",
  "JSToWasmStressSwitchStacksAsm",
  "CEntry",
  "WasmHandleStackOverflow",
  "WasmReturnFromSegment",
  "DoubleToI",
  "CallApiCallbackImpl",
  "CallApiAccessorImpl",
  "DirectCEntry",
  "DeoptimizationEntry_Eager",
  "DeoptimizationEntry_Lazy",
  "DeoptimizationEntry_LazyAfterFastCall",
  "InterpreterOnStackReplacement_ToBaseline",
  "RestartFrameTrampoline",
];

/// 原版 enum OsrSourceTier 直译 (变体不带值, 语义由 Developer 修订)
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter, strum::EnumString,
)]
#[strum(serialize_all = "PascalCase")]
pub enum OsrSourceTier {
  /// kInterpreter
  Interpreter,
  /// kBaseline
  Baseline,
}

/// 原版 enum DebugBreakKind 直译 (变体不带值, 语义由 Developer 修订)
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter, strum::EnumString,
)]
#[strum(serialize_all = "PascalCase")]
pub enum DebugBreakKind {
  /// kBreak
  Break,
  /// kTrap
  Trap,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[
  ("OsrSourceTier", ""),
  ("DebugBreakKind", ""),
  ("RegisterAllocator", ""),
  ("Scoped", ""),
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.is_empty());
  assert!(ASM_GENERATE_ENTRIES.len() == 71);
  assert!(CLASSES.len() == 4);
};
