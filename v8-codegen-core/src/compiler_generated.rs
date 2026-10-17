// 由 tool/src/codegen/compiler.h.js 从 v8/src/codegen/compiler.{h,cc} 生成; 修订就地改, 复跑覆盖
// 编译器接口与作业状态机骨架

// 语义面: 编译管线 (cc 4773 行) 登记跳过。

/// 异常清除策略 (原 enum ClearExceptionFlag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClearExceptionFlag {
  KeepException = 0,
  ClearException = 1,
}

/// 作业状态机 (原 enum class State)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum State {
  /// kkReadyToPrepare
  kReadyToPrepare = 0,
  /// kkReadyToExecute
  kReadyToExecute = 1,
  /// kkReadyToFinalize
  kReadyToFinalize = 2,
  /// kkSucceeded
  kSucceeded = 3,
  /// kkFailed
  kFailed = 4,
}

/// 作业结果 (原 enum Status)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
  Succeeded = 0,
  Failed = 1,
  RetryOnMainThread = 2,
}

/// 原 class Compiler : AllStatic 静态接口 (27 条):
// - Compile
// - CompileToplevel
// - CompileSharedWithBaseline
// - CompileBaseline
// - CompileOptimized
// - CompileOptimizedOSR
// - CollectSourcePositions
// - FinalizeBackgroundCompileTask
// - DisposeTurbofanCompilationJob
// - FinalizeTurbofanCompilationJob
// - FinalizeMaglevCompilationJob
// - DisposeMaglevCompilationJob
// - PostInstantiation
// - GetFunctionFromEval
// - GetWrappedFunction
// - GetFunctionFromString
// - ValidateDynamicCompilationSource
// - GetFunctionFromValidatedString
// - GetSharedFunctionInfoForScript
// - GetSharedFunctionInfoForScriptWithExtension
// - GetSharedFunctionInfoForScriptWithCachedData
// - GetSharedFunctionInfoForScriptWithDeserializeTask
// - GetSharedFunctionInfoForScriptWithCompileHints
// - GetSharedFunctionInfoForStreamedScript
// - GetSharedFunctionInfo
// - LogFunctionCompilation
// - InstallInterpreterTrampolineCopy

/// 派生作业类登记:
// - class TurbofanCompilationJob
// - class MaglevCompilationJob

/// 原 class CompilationJob 骨架
pub struct CompilationJob;

// TODO(dev): 补 Prepare/Execute/Finalize 三相与状态迁移。

// —— 提取不变式断言 ——
const _: () = {
  assert!(State::ReadyToPrepare as u8 == 0);
  assert!(Status::RetryOnMainThread as u8 == 2);
};
