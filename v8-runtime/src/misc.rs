// 由 tool/src/runtime/gen-runtime.js 生成
// 杂项运行时函数 (Scopes, Compiler, Debug, Internal, Wasm, Promise 等)

use strum::{Display, EnumIter, EnumString};

use crate::table::{RuntimeFunction, RuntimeFunctionId, find_runtime_function};

/// 杂项操作运行时函数 (294 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum MiscRuntimeFunction {
  #[strum(
    serialize = "Runtime_ThrowConstAssignError",
    serialize = "ThrowConstAssignError"
  )]
  ThrowConstAssignError,
  #[strum(
    serialize = "Runtime_ThrowUsingAssignError",
    serialize = "ThrowUsingAssignError"
  )]
  ThrowUsingAssignError,
  #[strum(
    serialize = "Runtime_ThrowAwaitUsingAssignError",
    serialize = "ThrowAwaitUsingAssignError"
  )]
  ThrowAwaitUsingAssignError,
  #[strum(
    serialize = "Runtime_DeclareModuleExports",
    serialize = "DeclareModuleExports"
  )]
  DeclareModuleExports,
  #[strum(serialize = "Runtime_DeclareGlobals", serialize = "DeclareGlobals")]
  DeclareGlobals,
  #[strum(
    serialize = "Runtime_InitializeDisposableStack",
    serialize = "InitializeDisposableStack"
  )]
  InitializeDisposableStack,
  #[strum(
    serialize = "Runtime_AddDisposableValue",
    serialize = "AddDisposableValue"
  )]
  AddDisposableValue,
  #[strum(
    serialize = "Runtime_AddAsyncDisposableValue",
    serialize = "AddAsyncDisposableValue"
  )]
  AddAsyncDisposableValue,
  #[strum(
    serialize = "Runtime_DisposeDisposableStack",
    serialize = "DisposeDisposableStack"
  )]
  DisposeDisposableStack,
  #[strum(
    serialize = "Runtime_HandleExceptionsInDisposeDisposableStack",
    serialize = "HandleExceptionsInDisposeDisposableStack"
  )]
  HandleExceptionsInDisposeDisposableStack,
  #[strum(
    serialize = "Runtime_DeclareEvalFunction",
    serialize = "DeclareEvalFunction"
  )]
  DeclareEvalFunction,
  #[strum(serialize = "Runtime_DeclareEvalVar", serialize = "DeclareEvalVar")]
  DeclareEvalVar,
  #[strum(
    serialize = "Runtime_NewSloppyArguments",
    serialize = "NewSloppyArguments"
  )]
  NewSloppyArguments,
  #[strum(
    serialize = "Runtime_NewStrictArguments",
    serialize = "NewStrictArguments"
  )]
  NewStrictArguments,
  #[strum(serialize = "Runtime_NewRestParameter", serialize = "NewRestParameter")]
  NewRestParameter,
  #[strum(serialize = "Runtime_NewClosure", serialize = "NewClosure")]
  NewClosure,
  #[strum(
    serialize = "Runtime_NewClosure_Tenured",
    serialize = "NewClosure_Tenured",
    serialize = "NewClosureTenured"
  )]
  NewClosureTenured,
  #[strum(
    serialize = "Runtime_NewFunctionContext",
    serialize = "NewFunctionContext"
  )]
  NewFunctionContext,
  #[strum(serialize = "Runtime_PushWithContext", serialize = "PushWithContext")]
  PushWithContext,
  #[strum(serialize = "Runtime_PushCatchContext", serialize = "PushCatchContext")]
  PushCatchContext,
  #[strum(serialize = "Runtime_PushBlockContext", serialize = "PushBlockContext")]
  PushBlockContext,
  #[strum(serialize = "Runtime_DeleteLookupSlot", serialize = "DeleteLookupSlot")]
  DeleteLookupSlot,
  #[strum(serialize = "Runtime_LoadLookupSlot", serialize = "LoadLookupSlot")]
  LoadLookupSlot,
  #[strum(
    serialize = "Runtime_LoadLookupSlotInsideTypeof",
    serialize = "LoadLookupSlotInsideTypeof"
  )]
  LoadLookupSlotInsideTypeof,
  #[strum(
    serialize = "Runtime_LoadLookupSlotForCall",
    serialize = "LoadLookupSlotForCall"
  )]
  LoadLookupSlotForCall,
  #[strum(
    serialize = "Runtime_LoadLookupSlotForCall_Baseline",
    serialize = "LoadLookupSlotForCall_Baseline",
    serialize = "LoadLookupSlotForCallBaseline"
  )]
  LoadLookupSlotForCallBaseline,
  #[strum(
    serialize = "Runtime_StoreLookupSlot_Sloppy",
    serialize = "StoreLookupSlot_Sloppy",
    serialize = "StoreLookupSlotSloppy"
  )]
  StoreLookupSlotSloppy,
  #[strum(
    serialize = "Runtime_StoreLookupSlot_Strict",
    serialize = "StoreLookupSlot_Strict",
    serialize = "StoreLookupSlotStrict"
  )]
  StoreLookupSlotStrict,
  #[strum(
    serialize = "Runtime_StoreLookupSlot_SloppyHoisting",
    serialize = "StoreLookupSlot_SloppyHoisting",
    serialize = "StoreLookupSlotSloppyHoisting"
  )]
  StoreLookupSlotSloppyHoisting,
  #[strum(
    serialize = "Runtime_StoreGlobalNoHoleCheckForReplLetOrConst",
    serialize = "StoreGlobalNoHoleCheckForReplLetOrConst"
  )]
  StoreGlobalNoHoleCheckForReplLetOrConst,
  #[strum(
    serialize = "Runtime_DynamicImportCall",
    serialize = "DynamicImportCall"
  )]
  DynamicImportCall,
  #[strum(
    serialize = "Runtime_GetModuleNamespace",
    serialize = "GetModuleNamespace"
  )]
  GetModuleNamespace,
  #[strum(
    serialize = "Runtime_GetImportMetaObject",
    serialize = "GetImportMetaObject"
  )]
  GetImportMetaObject,
  #[strum(
    serialize = "Runtime_GetModuleNamespaceExport",
    serialize = "GetModuleNamespaceExport"
  )]
  GetModuleNamespaceExport,
  #[strum(serialize = "Runtime_IterableForEach", serialize = "IterableForEach")]
  IterableForEach,
  #[strum(
    serialize = "Runtime_DebugBreakOnBytecode",
    serialize = "DebugBreakOnBytecode"
  )]
  DebugBreakOnBytecode,
  #[strum(
    serialize = "Runtime_DebugBreakAtEntry",
    serialize = "DebugBreakAtEntry"
  )]
  DebugBreakAtEntry,
  #[strum(
    serialize = "Runtime_HandleDebuggerStatement",
    serialize = "HandleDebuggerStatement"
  )]
  HandleDebuggerStatement,
  #[strum(serialize = "Runtime_ScheduleBreak", serialize = "ScheduleBreak")]
  ScheduleBreak,
  #[strum(
    serialize = "Runtime_GetGeneratorScopeCount",
    serialize = "GetGeneratorScopeCount"
  )]
  GetGeneratorScopeCount,
  #[strum(
    serialize = "Runtime_GetGeneratorScopeDetails",
    serialize = "GetGeneratorScopeDetails"
  )]
  GetGeneratorScopeDetails,
  #[strum(
    serialize = "Runtime_SetGeneratorScopeVariableValue",
    serialize = "SetGeneratorScopeVariableValue"
  )]
  SetGeneratorScopeVariableValue,
  #[strum(
    serialize = "Runtime_GetBreakLocations",
    serialize = "GetBreakLocations"
  )]
  GetBreakLocations,
  #[strum(
    serialize = "Runtime_IsBreakOnException",
    serialize = "IsBreakOnException"
  )]
  IsBreakOnException,
  #[strum(serialize = "Runtime_ClearStepping", serialize = "ClearStepping")]
  ClearStepping,
  #[strum(
    serialize = "Runtime_DebugGetLoadedScriptIds",
    serialize = "DebugGetLoadedScriptIds"
  )]
  DebugGetLoadedScriptIds,
  #[strum(
    serialize = "Runtime_FunctionGetInferredName",
    serialize = "FunctionGetInferredName"
  )]
  FunctionGetInferredName,
  #[strum(serialize = "Runtime_CollectGarbage", serialize = "CollectGarbage")]
  CollectGarbage,
  #[strum(
    serialize = "Runtime_ScriptLocationFromLine2",
    serialize = "ScriptLocationFromLine2"
  )]
  ScriptLocationFromLine2,
  #[strum(
    serialize = "Runtime_DebugOnFunctionCall",
    serialize = "DebugOnFunctionCall"
  )]
  DebugOnFunctionCall,
  #[strum(
    serialize = "Runtime_DebugPrepareStepInSuspendedGenerator",
    serialize = "DebugPrepareStepInSuspendedGenerator"
  )]
  DebugPrepareStepInSuspendedGenerator,
  #[strum(
    serialize = "Runtime_DebugCollectCoverage",
    serialize = "DebugCollectCoverage"
  )]
  DebugCollectCoverage,
  #[strum(
    serialize = "Runtime_DebugCollectWasmCoverage",
    serialize = "DebugCollectWasmCoverage"
  )]
  DebugCollectWasmCoverage,
  #[strum(
    serialize = "Runtime_DebugTogglePreciseCoverage",
    serialize = "DebugTogglePreciseCoverage"
  )]
  DebugTogglePreciseCoverage,
  #[strum(
    serialize = "Runtime_DebugToggleBlockCoverage",
    serialize = "DebugToggleBlockCoverage"
  )]
  DebugToggleBlockCoverage,
  #[strum(serialize = "Runtime_IncBlockCounter", serialize = "IncBlockCounter")]
  IncBlockCounter,
  #[strum(
    serialize = "Runtime_DebugAsyncFunctionSuspended",
    serialize = "DebugAsyncFunctionSuspended"
  )]
  DebugAsyncFunctionSuspended,
  #[strum(serialize = "Runtime_DebugPromiseThen", serialize = "DebugPromiseThen")]
  DebugPromiseThen,
  #[strum(
    serialize = "Runtime_ProfileCreateSnapshotDataBlob",
    serialize = "ProfileCreateSnapshotDataBlob"
  )]
  ProfileCreateSnapshotDataBlob,
  #[strum(serialize = "Runtime_CompileLazy", serialize = "CompileLazy")]
  CompileLazy,
  #[strum(
    serialize = "Runtime_InstallBaselineCode",
    serialize = "InstallBaselineCode"
  )]
  InstallBaselineCode,
  #[strum(serialize = "Runtime_InstallSFICode", serialize = "InstallSFICode")]
  InstallSFICode,
  #[strum(
    serialize = "Runtime_StartMaglevOptimizeJob",
    serialize = "StartMaglevOptimizeJob"
  )]
  StartMaglevOptimizeJob,
  #[strum(
    serialize = "Runtime_StartTurbofanOptimizeJob",
    serialize = "StartTurbofanOptimizeJob"
  )]
  StartTurbofanOptimizeJob,
  #[strum(
    serialize = "Runtime_OptimizeMaglevEager",
    serialize = "OptimizeMaglevEager"
  )]
  OptimizeMaglevEager,
  #[strum(
    serialize = "Runtime_OptimizeTurbofanEager",
    serialize = "OptimizeTurbofanEager"
  )]
  OptimizeTurbofanEager,
  #[strum(
    serialize = "Runtime_MarkLazyDeoptimizedOrFlushed",
    serialize = "MarkLazyDeoptimizedOrFlushed"
  )]
  MarkLazyDeoptimizedOrFlushed,
  #[strum(
    serialize = "Runtime_FunctionLogNextExecution",
    serialize = "FunctionLogNextExecution"
  )]
  FunctionLogNextExecution,
  #[strum(serialize = "Runtime_PrintDumpedFrame", serialize = "PrintDumpedFrame")]
  PrintDumpedFrame,
  #[strum(
    serialize = "Runtime_NotifyDeoptimized",
    serialize = "NotifyDeoptimized"
  )]
  NotifyDeoptimized,
  #[strum(serialize = "Runtime_ObserveNode", serialize = "ObserveNode")]
  ObserveNode,
  #[strum(serialize = "Runtime_VerifyType", serialize = "VerifyType")]
  VerifyType,
  #[strum(
    serialize = "Runtime_CheckTurboshaftTypeOf",
    serialize = "CheckTurboshaftTypeOf"
  )]
  CheckTurboshaftTypeOf,
  #[strum(
    serialize = "Runtime_CompileOptimizedOSR",
    serialize = "CompileOptimizedOSR"
  )]
  CompileOptimizedOSR,
  #[strum(
    serialize = "Runtime_CompileOptimizedOSRFromMaglev",
    serialize = "CompileOptimizedOSRFromMaglev"
  )]
  CompileOptimizedOSRFromMaglev,
  #[strum(
    serialize = "Runtime_CompileOptimizedOSRFromMaglevInlined",
    serialize = "CompileOptimizedOSRFromMaglevInlined"
  )]
  CompileOptimizedOSRFromMaglevInlined,
  #[strum(
    serialize = "Runtime_LogOrTraceOptimizedOSREntry",
    serialize = "LogOrTraceOptimizedOSREntry"
  )]
  LogOrTraceOptimizedOSREntry,
  #[strum(
    serialize = "Runtime_ResolvePossiblyDirectEval",
    serialize = "ResolvePossiblyDirectEval"
  )]
  ResolvePossiblyDirectEval,
  #[strum(
    serialize = "Runtime_PatchCompareOpBaselineCode",
    serialize = "PatchCompareOpBaselineCode"
  )]
  PatchCompareOpBaselineCode,
  #[strum(
    serialize = "Runtime_PatchCompareOpBaselineCodeAndThrow",
    serialize = "PatchCompareOpBaselineCodeAndThrow"
  )]
  PatchCompareOpBaselineCodeAndThrow,
  #[strum(
    serialize = "Runtime_PatchBinopBaselineCode",
    serialize = "PatchBinopBaselineCode"
  )]
  PatchBinopBaselineCode,
  #[strum(
    serialize = "Runtime_PatchBinopBaselineCodeAndThrow",
    serialize = "PatchBinopBaselineCodeAndThrow"
  )]
  PatchBinopBaselineCodeAndThrow,
  #[strum(
    serialize = "Runtime_PatchUnaryOpBaselineCode",
    serialize = "PatchUnaryOpBaselineCode"
  )]
  PatchUnaryOpBaselineCode,
  #[strum(
    serialize = "Runtime_PatchUnaryOpBaselineCodeAndThrow",
    serialize = "PatchUnaryOpBaselineCodeAndThrow"
  )]
  PatchUnaryOpBaselineCodeAndThrow,
  #[strum(serialize = "Runtime_AccessCheck", serialize = "AccessCheck")]
  AccessCheck,
  #[strum(
    serialize = "Runtime_FatalProcessOutOfMemoryInAllocateRaw",
    serialize = "FatalProcessOutOfMemoryInAllocateRaw"
  )]
  FatalProcessOutOfMemoryInAllocateRaw,
  #[strum(
    serialize = "Runtime_FatalProcessOutOfMemoryInvalidArrayLength",
    serialize = "FatalProcessOutOfMemoryInvalidArrayLength"
  )]
  FatalProcessOutOfMemoryInvalidArrayLength,
  #[strum(serialize = "Runtime_FatalInvalidSize", serialize = "FatalInvalidSize")]
  FatalInvalidSize,
  #[strum(serialize = "Runtime_Throw", serialize = "Throw")]
  Throw,
  #[strum(serialize = "Runtime_ReThrow", serialize = "ReThrow")]
  ReThrow,
  #[strum(
    serialize = "Runtime_ReThrowWithMessage",
    serialize = "ReThrowWithMessage"
  )]
  ReThrowWithMessage,
  #[strum(
    serialize = "Runtime_ThrowStackOverflow",
    serialize = "ThrowStackOverflow"
  )]
  ThrowStackOverflow,
  #[strum(
    serialize = "Runtime_VarargStackOverflow",
    serialize = "VarargStackOverflow"
  )]
  VarargStackOverflow,
  #[strum(
    serialize = "Runtime_ThrowSymbolAsyncIteratorInvalid",
    serialize = "ThrowSymbolAsyncIteratorInvalid"
  )]
  ThrowSymbolAsyncIteratorInvalid,
  #[strum(
    serialize = "Runtime_TerminateExecution",
    serialize = "TerminateExecution"
  )]
  TerminateExecution,
  #[strum(serialize = "Runtime_ThrowRangeError", serialize = "ThrowRangeError")]
  ThrowRangeError,
  #[strum(serialize = "Runtime_ThrowTypeError", serialize = "ThrowTypeError")]
  ThrowTypeError,
  #[strum(
    serialize = "Runtime_ThrowTypeErrorIfStrict",
    serialize = "ThrowTypeErrorIfStrict"
  )]
  ThrowTypeErrorIfStrict,
  #[strum(
    serialize = "Runtime_ThrowInvalidTypedArrayAlignment",
    serialize = "ThrowInvalidTypedArrayAlignment"
  )]
  ThrowInvalidTypedArrayAlignment,
  #[strum(
    serialize = "Runtime_UnwindAndFindExceptionHandler",
    serialize = "UnwindAndFindExceptionHandler"
  )]
  UnwindAndFindExceptionHandler,
  #[strum(
    serialize = "Runtime_PropagateException",
    serialize = "PropagateException"
  )]
  PropagateException,
  #[strum(
    serialize = "Runtime_ThrowReferenceError",
    serialize = "ThrowReferenceError"
  )]
  ThrowReferenceError,
  #[strum(
    serialize = "Runtime_ThrowAccessedUninitializedVariable",
    serialize = "ThrowAccessedUninitializedVariable"
  )]
  ThrowAccessedUninitializedVariable,
  #[strum(serialize = "Runtime_NewError", serialize = "NewError")]
  NewError,
  #[strum(serialize = "Runtime_NewTypeError", serialize = "NewTypeError")]
  NewTypeError,
  #[strum(
    serialize = "Runtime_NewReferenceError",
    serialize = "NewReferenceError"
  )]
  NewReferenceError,
  #[strum(
    serialize = "Runtime_ThrowInvalidStringLength",
    serialize = "ThrowInvalidStringLength"
  )]
  ThrowInvalidStringLength,
  #[strum(
    serialize = "Runtime_ThrowIteratorResultNotAnObject",
    serialize = "ThrowIteratorResultNotAnObject"
  )]
  ThrowIteratorResultNotAnObject,
  #[strum(
    serialize = "Runtime_ThrowThrowMethodMissing",
    serialize = "ThrowThrowMethodMissing"
  )]
  ThrowThrowMethodMissing,
  #[strum(
    serialize = "Runtime_ThrowSymbolIteratorInvalid",
    serialize = "ThrowSymbolIteratorInvalid"
  )]
  ThrowSymbolIteratorInvalid,
  #[strum(serialize = "Runtime_ThrowNoAccess", serialize = "ThrowNoAccess")]
  ThrowNoAccess,
  #[strum(
    serialize = "Runtime_ThrowNotConstructor",
    serialize = "ThrowNotConstructor"
  )]
  ThrowNotConstructor,
  #[strum(
    serialize = "Runtime_ThrowTargetNonFunction",
    serialize = "ThrowTargetNonFunction"
  )]
  ThrowTargetNonFunction,
  #[strum(serialize = "Runtime_StackGuard", serialize = "StackGuard")]
  StackGuard,
  #[strum(
    serialize = "Runtime_HandleNoHeapWritesInterrupts",
    serialize = "HandleNoHeapWritesInterrupts"
  )]
  HandleNoHeapWritesInterrupts,
  #[strum(
    serialize = "Runtime_StackGuardWithGap",
    serialize = "StackGuardWithGap"
  )]
  StackGuardWithGap,
  #[strum(
    serialize = "Runtime_BytecodeBudgetInterruptWithStackCheck_Ignition",
    serialize = "BytecodeBudgetInterruptWithStackCheck_Ignition",
    serialize = "BytecodeBudgetInterruptWithStackCheckIgnition"
  )]
  BytecodeBudgetInterruptWithStackCheckIgnition,
  #[strum(
    serialize = "Runtime_BytecodeBudgetInterrupt_Ignition",
    serialize = "BytecodeBudgetInterrupt_Ignition",
    serialize = "BytecodeBudgetInterruptIgnition"
  )]
  BytecodeBudgetInterruptIgnition,
  #[strum(
    serialize = "Runtime_BytecodeBudgetInterruptWithStackCheck_Sparkplug",
    serialize = "BytecodeBudgetInterruptWithStackCheck_Sparkplug",
    serialize = "BytecodeBudgetInterruptWithStackCheckSparkplug"
  )]
  BytecodeBudgetInterruptWithStackCheckSparkplug,
  #[strum(
    serialize = "Runtime_BytecodeBudgetInterrupt_Sparkplug",
    serialize = "BytecodeBudgetInterrupt_Sparkplug",
    serialize = "BytecodeBudgetInterruptSparkplug"
  )]
  BytecodeBudgetInterruptSparkplug,
  #[strum(
    serialize = "Runtime_BytecodeBudgetInterrupt_Maglev",
    serialize = "BytecodeBudgetInterrupt_Maglev",
    serialize = "BytecodeBudgetInterruptMaglev"
  )]
  BytecodeBudgetInterruptMaglev,
  #[strum(
    serialize = "Runtime_BytecodeBudgetLoopInterrupt_Maglev",
    serialize = "BytecodeBudgetLoopInterrupt_Maglev",
    serialize = "BytecodeBudgetLoopInterruptMaglev"
  )]
  BytecodeBudgetLoopInterruptMaglev,
  #[strum(
    serialize = "Runtime_AllocateInYoungGeneration",
    serialize = "AllocateInYoungGeneration"
  )]
  AllocateInYoungGeneration,
  #[strum(
    serialize = "Runtime_AllocateInOldGeneration",
    serialize = "AllocateInOldGeneration"
  )]
  AllocateInOldGeneration,
  #[strum(
    serialize = "Runtime_AllocateInSharedHeap",
    serialize = "AllocateInSharedHeap"
  )]
  AllocateInSharedHeap,
  #[strum(
    serialize = "Runtime_AllocateByteArray",
    serialize = "AllocateByteArray"
  )]
  AllocateByteArray,
  #[strum(
    serialize = "Runtime_ThrowIteratorError",
    serialize = "ThrowIteratorError"
  )]
  ThrowIteratorError,
  #[strum(
    serialize = "Runtime_ThrowSpreadArgError",
    serialize = "ThrowSpreadArgError"
  )]
  ThrowSpreadArgError,
  #[strum(
    serialize = "Runtime_ThrowCalledNonCallable",
    serialize = "ThrowCalledNonCallable"
  )]
  ThrowCalledNonCallable,
  #[strum(
    serialize = "Runtime_ThrowConstructedNonConstructable",
    serialize = "ThrowConstructedNonConstructable"
  )]
  ThrowConstructedNonConstructable,
  #[strum(
    serialize = "Runtime_ThrowPatternAssignmentNonCoercible",
    serialize = "ThrowPatternAssignmentNonCoercible"
  )]
  ThrowPatternAssignmentNonCoercible,
  #[strum(
    serialize = "Runtime_ThrowConstructorReturnedNonObject",
    serialize = "ThrowConstructorReturnedNonObject"
  )]
  ThrowConstructorReturnedNonObject,
  #[strum(
    serialize = "Runtime_CreateListFromArrayLike",
    serialize = "CreateListFromArrayLike"
  )]
  CreateListFromArrayLike,
  #[strum(
    serialize = "Runtime_IncrementUseCounter",
    serialize = "IncrementUseCounter"
  )]
  IncrementUseCounter,
  #[strum(
    serialize = "Runtime_GetAndResetTurboProfilingData",
    serialize = "GetAndResetTurboProfilingData"
  )]
  GetAndResetTurboProfilingData,
  #[strum(
    serialize = "Runtime_GetAndResetRuntimeCallStats",
    serialize = "GetAndResetRuntimeCallStats"
  )]
  GetAndResetRuntimeCallStats,
  #[strum(
    serialize = "Runtime_OrdinaryHasInstance",
    serialize = "OrdinaryHasInstance"
  )]
  OrdinaryHasInstance,
  #[strum(serialize = "Runtime_Typeof", serialize = "Typeof")]
  Typeof,
  #[strum(
    serialize = "Runtime_AllowDynamicFunction",
    serialize = "AllowDynamicFunction"
  )]
  AllowDynamicFunction,
  #[strum(
    serialize = "Runtime_CreateAsyncFromSyncIterator",
    serialize = "CreateAsyncFromSyncIterator"
  )]
  CreateAsyncFromSyncIterator,
  #[strum(
    serialize = "Runtime_GetTemplateObject",
    serialize = "GetTemplateObject"
  )]
  GetTemplateObject,
  #[strum(
    serialize = "Runtime_ReportMessageFromMicrotask",
    serialize = "ReportMessageFromMicrotask"
  )]
  ReportMessageFromMicrotask,
  #[strum(
    serialize = "Runtime_GetInitializerFunction",
    serialize = "GetInitializerFunction"
  )]
  GetInitializerFunction,
  #[strum(
    serialize = "Runtime_DoubleToStringWithRadix",
    serialize = "DoubleToStringWithRadix"
  )]
  DoubleToStringWithRadix,
  #[strum(
    serialize = "Runtime_SharedValueBarrierSlow",
    serialize = "SharedValueBarrierSlow"
  )]
  SharedValueBarrierSlow,
  #[strum(
    serialize = "Runtime_NotifyContextCellStateWillChange",
    serialize = "NotifyContextCellStateWillChange"
  )]
  NotifyContextCellStateWillChange,
  #[strum(
    serialize = "Runtime_InvalidateStringWrapperToPrimitiveProtector",
    serialize = "InvalidateStringWrapperToPrimitiveProtector"
  )]
  InvalidateStringWrapperToPrimitiveProtector,
  #[strum(
    serialize = "Runtime_AddLhsIsStringConstantInternalize",
    serialize = "AddLhsIsStringConstantInternalize"
  )]
  AddLhsIsStringConstantInternalize,
  #[strum(
    serialize = "Runtime_AddRhsIsStringConstantInternalize",
    serialize = "AddRhsIsStringConstantInternalize"
  )]
  AddRhsIsStringConstantInternalize,
  #[strum(
    serialize = "Runtime_WasmWasmToJSObject",
    serialize = "WasmWasmToJSObject"
  )]
  WasmWasmToJSObject,
  #[strum(
    serialize = "Runtime_WasmJSToWasmObject",
    serialize = "WasmJSToWasmObject"
  )]
  WasmJSToWasmObject,
  #[strum(serialize = "Runtime_WasmMemoryGrow", serialize = "WasmMemoryGrow")]
  WasmMemoryGrow,
  #[strum(serialize = "Runtime_ThrowWasmError", serialize = "ThrowWasmError")]
  ThrowWasmError,
  #[strum(
    serialize = "Runtime_ThrowWasmStackOverflow",
    serialize = "ThrowWasmStackOverflow"
  )]
  ThrowWasmStackOverflow,
  #[strum(
    serialize = "Runtime_WasmThrowJSTypeError",
    serialize = "WasmThrowJSTypeError"
  )]
  WasmThrowJSTypeError,
  #[strum(
    serialize = "Runtime_ThrowWasmJSPISuspendError",
    serialize = "ThrowWasmJSPISuspendError"
  )]
  ThrowWasmJSPISuspendError,
  #[strum(
    serialize = "Runtime_ThrowWasmFXSuspendError",
    serialize = "ThrowWasmFXSuspendError"
  )]
  ThrowWasmFXSuspendError,
  #[strum(
    serialize = "Runtime_WasmThrowRangeError",
    serialize = "WasmThrowRangeError"
  )]
  WasmThrowRangeError,
  #[strum(
    serialize = "Runtime_WasmThrowDataViewTypeError",
    serialize = "WasmThrowDataViewTypeError"
  )]
  WasmThrowDataViewTypeError,
  #[strum(
    serialize = "Runtime_WasmThrowDataViewDetachedError",
    serialize = "WasmThrowDataViewDetachedError"
  )]
  WasmThrowDataViewDetachedError,
  #[strum(
    serialize = "Runtime_WasmThrowTypeError",
    serialize = "WasmThrowTypeError"
  )]
  WasmThrowTypeError,
  #[strum(serialize = "Runtime_WasmThrow", serialize = "WasmThrow")]
  WasmThrow,
  #[strum(serialize = "Runtime_WasmReThrow", serialize = "WasmReThrow")]
  WasmReThrow,
  #[strum(serialize = "Runtime_WasmStackGuard", serialize = "WasmStackGuard")]
  WasmStackGuard,
  #[strum(
    serialize = "Runtime_WasmStackGuardLoop",
    serialize = "WasmStackGuardLoop"
  )]
  WasmStackGuardLoop,
  #[strum(serialize = "Runtime_WasmCompileLazy", serialize = "WasmCompileLazy")]
  WasmCompileLazy,
  #[strum(
    serialize = "Runtime_WasmAllocateFeedbackVector",
    serialize = "WasmAllocateFeedbackVector"
  )]
  WasmAllocateFeedbackVector,
  #[strum(
    serialize = "Runtime_WasmLiftoffDeoptFinish",
    serialize = "WasmLiftoffDeoptFinish"
  )]
  WasmLiftoffDeoptFinish,
  #[strum(
    serialize = "Runtime_TierUpJSToWasmWrapper",
    serialize = "TierUpJSToWasmWrapper"
  )]
  TierUpJSToWasmWrapper,
  #[strum(
    serialize = "Runtime_IsWasmExternalFunction",
    serialize = "IsWasmExternalFunction"
  )]
  IsWasmExternalFunction,
  #[strum(
    serialize = "Runtime_TierUpWasmToJSWrapper",
    serialize = "TierUpWasmToJSWrapper"
  )]
  TierUpWasmToJSWrapper,
  #[strum(
    serialize = "Runtime_WasmTriggerTierUp",
    serialize = "WasmTriggerTierUp"
  )]
  WasmTriggerTierUp,
  #[strum(
    serialize = "Runtime_WasmI32AtomicWait",
    serialize = "WasmI32AtomicWait"
  )]
  WasmI32AtomicWait,
  #[strum(
    serialize = "Runtime_WasmI64AtomicWait",
    serialize = "WasmI64AtomicWait"
  )]
  WasmI64AtomicWait,
  #[strum(
    serialize = "Runtime_WasmManagedObjectWait32",
    serialize = "WasmManagedObjectWait32"
  )]
  WasmManagedObjectWait32,
  #[strum(
    serialize = "Runtime_WasmManagedObjectWait64",
    serialize = "WasmManagedObjectWait64"
  )]
  WasmManagedObjectWait64,
  #[strum(
    serialize = "Runtime_WasmManagedObjectWaitRef",
    serialize = "WasmManagedObjectWaitRef"
  )]
  WasmManagedObjectWaitRef,
  #[strum(serialize = "Runtime_WasmWaitqueueNew", serialize = "WasmWaitqueueNew")]
  WasmWaitqueueNew,
  #[strum(serialize = "Runtime_WasmRefFunc", serialize = "WasmRefFunc")]
  WasmRefFunc,
  #[strum(
    serialize = "Runtime_WasmInternalFunctionCreateExternal",
    serialize = "WasmInternalFunctionCreateExternal"
  )]
  WasmInternalFunctionCreateExternal,
  #[strum(
    serialize = "Runtime_WasmFunctionTableGet",
    serialize = "WasmFunctionTableGet"
  )]
  WasmFunctionTableGet,
  #[strum(
    serialize = "Runtime_WasmFunctionTableSet",
    serialize = "WasmFunctionTableSet"
  )]
  WasmFunctionTableSet,
  #[strum(serialize = "Runtime_WasmTableInit", serialize = "WasmTableInit")]
  WasmTableInit,
  #[strum(serialize = "Runtime_WasmTableCopy", serialize = "WasmTableCopy")]
  WasmTableCopy,
  #[strum(serialize = "Runtime_WasmTableGrow", serialize = "WasmTableGrow")]
  WasmTableGrow,
  #[strum(serialize = "Runtime_WasmTableFill", serialize = "WasmTableFill")]
  WasmTableFill,
  #[strum(serialize = "Runtime_WasmDebugBreak", serialize = "WasmDebugBreak")]
  WasmDebugBreak,
  #[strum(serialize = "Runtime_WasmArrayCopy", serialize = "WasmArrayCopy")]
  WasmArrayCopy,
  #[strum(
    serialize = "Runtime_WasmAllocateDescriptorStruct",
    serialize = "WasmAllocateDescriptorStruct"
  )]
  WasmAllocateDescriptorStruct,
  #[strum(
    serialize = "Runtime_WasmArrayNewSegment",
    serialize = "WasmArrayNewSegment"
  )]
  WasmArrayNewSegment,
  #[strum(
    serialize = "Runtime_WasmArrayInitSegment",
    serialize = "WasmArrayInitSegment"
  )]
  WasmArrayInitSegment,
  #[strum(
    serialize = "Runtime_WasmAllocateSuspender",
    serialize = "WasmAllocateSuspender"
  )]
  WasmAllocateSuspender,
  #[strum(serialize = "Runtime_WasmSuspended", serialize = "WasmSuspended")]
  WasmSuspended,
  #[strum(
    serialize = "Runtime_ClearWasmSuspenderResumeField",
    serialize = "ClearWasmSuspenderResumeField"
  )]
  ClearWasmSuspenderResumeField,
  #[strum(
    serialize = "Runtime_WasmConfigureAllPrototypes",
    serialize = "WasmConfigureAllPrototypes"
  )]
  WasmConfigureAllPrototypes,
  #[strum(
    serialize = "Runtime_WasmConfigureAllPrototypesOpt",
    serialize = "WasmConfigureAllPrototypesOpt"
  )]
  WasmConfigureAllPrototypesOpt,
  #[strum(
    serialize = "Runtime_WasmCastToSpecialPrimitiveArray",
    serialize = "WasmCastToSpecialPrimitiveArray"
  )]
  WasmCastToSpecialPrimitiveArray,
  #[strum(
    serialize = "Runtime_WasmStringNewWtf8",
    serialize = "WasmStringNewWtf8"
  )]
  WasmStringNewWtf8,
  #[strum(
    serialize = "Runtime_WasmStringNewWtf8Array",
    serialize = "WasmStringNewWtf8Array"
  )]
  WasmStringNewWtf8Array,
  #[strum(
    serialize = "Runtime_WasmStringNewWtf16",
    serialize = "WasmStringNewWtf16"
  )]
  WasmStringNewWtf16,
  #[strum(
    serialize = "Runtime_WasmStringNewWtf16Array",
    serialize = "WasmStringNewWtf16Array"
  )]
  WasmStringNewWtf16Array,
  #[strum(serialize = "Runtime_WasmSubstring", serialize = "WasmSubstring")]
  WasmSubstring,
  #[strum(
    serialize = "Runtime_WasmSubstringShared",
    serialize = "WasmSubstringShared"
  )]
  WasmSubstringShared,
  #[strum(serialize = "Runtime_WasmStringConst", serialize = "WasmStringConst")]
  WasmStringConst,
  #[strum(
    serialize = "Runtime_WasmStringNewSegmentWtf8",
    serialize = "WasmStringNewSegmentWtf8"
  )]
  WasmStringNewSegmentWtf8,
  #[strum(
    serialize = "Runtime_WasmStringMeasureUtf8",
    serialize = "WasmStringMeasureUtf8"
  )]
  WasmStringMeasureUtf8,
  #[strum(
    serialize = "Runtime_WasmStringMeasureWtf8",
    serialize = "WasmStringMeasureWtf8"
  )]
  WasmStringMeasureWtf8,
  #[strum(
    serialize = "Runtime_WasmStringEncodeWtf8",
    serialize = "WasmStringEncodeWtf8"
  )]
  WasmStringEncodeWtf8,
  #[strum(
    serialize = "Runtime_WasmStringEncodeWtf8Array",
    serialize = "WasmStringEncodeWtf8Array"
  )]
  WasmStringEncodeWtf8Array,
  #[strum(
    serialize = "Runtime_WasmStringToUtf8Array",
    serialize = "WasmStringToUtf8Array"
  )]
  WasmStringToUtf8Array,
  #[strum(
    serialize = "Runtime_WasmStringEncodeWtf16",
    serialize = "WasmStringEncodeWtf16"
  )]
  WasmStringEncodeWtf16,
  #[strum(serialize = "Runtime_WasmStringAsWtf8", serialize = "WasmStringAsWtf8")]
  WasmStringAsWtf8,
  #[strum(
    serialize = "Runtime_WasmStringViewWtf8Encode",
    serialize = "WasmStringViewWtf8Encode"
  )]
  WasmStringViewWtf8Encode,
  #[strum(
    serialize = "Runtime_WasmStringViewWtf8Slice",
    serialize = "WasmStringViewWtf8Slice"
  )]
  WasmStringViewWtf8Slice,
  #[strum(
    serialize = "Runtime_WasmTraceBeginExecution",
    serialize = "WasmTraceBeginExecution"
  )]
  WasmTraceBeginExecution,
  #[strum(
    serialize = "Runtime_WasmTraceEndExecution",
    serialize = "WasmTraceEndExecution"
  )]
  WasmTraceEndExecution,
  #[strum(
    serialize = "Runtime_WasmStringFromCodePoint",
    serialize = "WasmStringFromCodePoint"
  )]
  WasmStringFromCodePoint,
  #[strum(serialize = "Runtime_WasmStringHash", serialize = "WasmStringHash")]
  WasmStringHash,
  #[strum(
    serialize = "Runtime_WasmStringAdd_NoMapCheck_Shared",
    serialize = "WasmStringAdd_NoMapCheck_Shared",
    serialize = "WasmStringAddNoMapCheckShared"
  )]
  WasmStringAddNoMapCheckShared,
  #[strum(
    serialize = "Runtime_WasmAllocateContinuation",
    serialize = "WasmAllocateContinuation"
  )]
  WasmAllocateContinuation,
  #[strum(
    serialize = "Runtime_WasmTypeAssertionFailed",
    serialize = "WasmTypeAssertionFailed"
  )]
  WasmTypeAssertionFailed,
  #[strum(
    serialize = "Runtime_TsanAcquireForInitializationFence",
    serialize = "TsanAcquireForInitializationFence"
  )]
  TsanAcquireForInitializationFence,
  #[strum(
    serialize = "Runtime_FunctionGetScriptSource",
    serialize = "FunctionGetScriptSource"
  )]
  FunctionGetScriptSource,
  #[strum(
    serialize = "Runtime_FunctionGetScriptId",
    serialize = "FunctionGetScriptId"
  )]
  FunctionGetScriptId,
  #[strum(
    serialize = "Runtime_FunctionGetSourceCode",
    serialize = "FunctionGetSourceCode"
  )]
  FunctionGetSourceCode,
  #[strum(
    serialize = "Runtime_FunctionGetScriptSourcePosition",
    serialize = "FunctionGetScriptSourcePosition"
  )]
  FunctionGetScriptSourcePosition,
  #[strum(
    serialize = "Runtime_FunctionIsAPIFunction",
    serialize = "FunctionIsAPIFunction"
  )]
  FunctionIsAPIFunction,
  #[strum(serialize = "Runtime_Call", serialize = "Call")]
  Call,
  #[strum(
    serialize = "Runtime_AsyncFunctionAwait",
    serialize = "AsyncFunctionAwait"
  )]
  AsyncFunctionAwait,
  #[strum(
    serialize = "Runtime_AsyncFunctionEnter",
    serialize = "AsyncFunctionEnter"
  )]
  AsyncFunctionEnter,
  #[strum(
    serialize = "Runtime_AsyncFunctionReject",
    serialize = "AsyncFunctionReject"
  )]
  AsyncFunctionReject,
  #[strum(
    serialize = "Runtime_AsyncFunctionResolve",
    serialize = "AsyncFunctionResolve"
  )]
  AsyncFunctionResolve,
  #[strum(
    serialize = "Runtime_CreateJSGeneratorObject",
    serialize = "CreateJSGeneratorObject"
  )]
  CreateJSGeneratorObject,
  #[strum(serialize = "Runtime_GeneratorClose", serialize = "GeneratorClose")]
  GeneratorClose,
  #[strum(
    serialize = "Runtime_GeneratorGetFunction",
    serialize = "GeneratorGetFunction"
  )]
  GeneratorGetFunction,
  #[strum(
    serialize = "Runtime_AsyncGeneratorAwait",
    serialize = "AsyncGeneratorAwait"
  )]
  AsyncGeneratorAwait,
  #[strum(
    serialize = "Runtime_AsyncGeneratorResolve",
    serialize = "AsyncGeneratorResolve"
  )]
  AsyncGeneratorResolve,
  #[strum(
    serialize = "Runtime_AsyncGeneratorReject",
    serialize = "AsyncGeneratorReject"
  )]
  AsyncGeneratorReject,
  #[strum(
    serialize = "Runtime_AsyncGeneratorYieldWithAwait",
    serialize = "AsyncGeneratorYieldWithAwait"
  )]
  AsyncGeneratorYieldWithAwait,
  #[strum(
    serialize = "Runtime_GeneratorGetResumeMode",
    serialize = "GeneratorGetResumeMode"
  )]
  GeneratorGetResumeMode,
  #[strum(
    serialize = "Runtime_GeneratorYieldResult",
    serialize = "GeneratorYieldResult"
  )]
  GeneratorYieldResult,
  #[strum(
    serialize = "Runtime_PromiseRejectEventFromStack",
    serialize = "PromiseRejectEventFromStack"
  )]
  PromiseRejectEventFromStack,
  #[strum(
    serialize = "Runtime_PromiseRevokeReject",
    serialize = "PromiseRevokeReject"
  )]
  PromiseRevokeReject,
  #[strum(serialize = "Runtime_EnqueueMicrotask", serialize = "EnqueueMicrotask")]
  EnqueueMicrotask,
  #[strum(
    serialize = "Runtime_PerformMicrotaskCheckpoint",
    serialize = "PerformMicrotaskCheckpoint"
  )]
  PerformMicrotaskCheckpoint,
  #[strum(
    serialize = "Runtime_RunMicrotaskCallback",
    serialize = "RunMicrotaskCallback"
  )]
  RunMicrotaskCallback,
  #[strum(serialize = "Runtime_PromiseHookInit", serialize = "PromiseHookInit")]
  PromiseHookInit,
  #[strum(
    serialize = "Runtime_PromiseHookBefore",
    serialize = "PromiseHookBefore"
  )]
  PromiseHookBefore,
  #[strum(serialize = "Runtime_PromiseHookAfter", serialize = "PromiseHookAfter")]
  PromiseHookAfter,
  #[strum(serialize = "Runtime_RejectPromise", serialize = "RejectPromise")]
  RejectPromise,
  #[strum(serialize = "Runtime_ResolvePromise", serialize = "ResolvePromise")]
  ResolvePromise,
  #[strum(
    serialize = "Runtime_ConstructAggregateErrorHelper",
    serialize = "ConstructAggregateErrorHelper"
  )]
  ConstructAggregateErrorHelper,
  #[strum(
    serialize = "Runtime_ConstructInternalAggregateErrorHelper",
    serialize = "ConstructInternalAggregateErrorHelper"
  )]
  ConstructInternalAggregateErrorHelper,
  #[strum(
    serialize = "Runtime_ConstructSuppressedError",
    serialize = "ConstructSuppressedError"
  )]
  ConstructSuppressedError,
  #[strum(serialize = "Runtime_StringSplit", serialize = "StringSplit")]
  StringSplit,
  #[strum(serialize = "Runtime_RegExpExec", serialize = "RegExpExec")]
  RegExpExec,
  #[strum(
    serialize = "Runtime_RegExpGrowRegExpMatchInfo",
    serialize = "RegExpGrowRegExpMatchInfo"
  )]
  RegExpGrowRegExpMatchInfo,
  #[strum(
    serialize = "Runtime_RegExpExperimentalOneshotExec",
    serialize = "RegExpExperimentalOneshotExec"
  )]
  RegExpExperimentalOneshotExec,
  #[strum(
    serialize = "Runtime_RegExpBuildIndices",
    serialize = "RegExpBuildIndices"
  )]
  RegExpBuildIndices,
  #[strum(
    serialize = "Runtime_RegExpExecMultiple",
    serialize = "RegExpExecMultiple"
  )]
  RegExpExecMultiple,
  #[strum(
    serialize = "Runtime_StringReplaceNonGlobalRegExpWithFunction",
    serialize = "StringReplaceNonGlobalRegExpWithFunction"
  )]
  StringReplaceNonGlobalRegExpWithFunction,
  #[strum(serialize = "Runtime_RegExpSplit", serialize = "RegExpSplit")]
  RegExpSplit,
  #[strum(serialize = "Runtime_RegExpReplaceRT", serialize = "RegExpReplaceRT")]
  RegExpReplaceRT,
  #[strum(
    serialize = "Runtime_RegExpInitializeAndCompile",
    serialize = "RegExpInitializeAndCompile"
  )]
  RegExpInitializeAndCompile,
  #[strum(
    serialize = "Runtime_RegExpStringFromFlags",
    serialize = "RegExpStringFromFlags"
  )]
  RegExpStringFromFlags,
  #[strum(
    serialize = "Runtime_RegExpMatchGlobalAtom",
    serialize = "RegExpMatchGlobalAtom"
  )]
  RegExpMatchGlobalAtom,
  #[strum(
    serialize = "Runtime_CreatePrivateSymbol",
    serialize = "CreatePrivateSymbol"
  )]
  CreatePrivateSymbol,
  #[strum(
    serialize = "Runtime_CreatePrivateBrandSymbol",
    serialize = "CreatePrivateBrandSymbol"
  )]
  CreatePrivateBrandSymbol,
  #[strum(
    serialize = "Runtime_CreatePrivateNameSymbol",
    serialize = "CreatePrivateNameSymbol"
  )]
  CreatePrivateNameSymbol,
  #[strum(
    serialize = "Runtime_SymbolDescriptiveString",
    serialize = "SymbolDescriptiveString"
  )]
  SymbolDescriptiveString,
  #[strum(serialize = "Runtime_SymbolIsPrivate", serialize = "SymbolIsPrivate")]
  SymbolIsPrivate,
  #[strum(
    serialize = "Runtime_CreateObjectLiteral",
    serialize = "CreateObjectLiteral"
  )]
  CreateObjectLiteral,
  #[strum(
    serialize = "Runtime_SetPrototypeProperties",
    serialize = "SetPrototypeProperties"
  )]
  SetPrototypeProperties,
  #[strum(
    serialize = "Runtime_CreateArrayLiteral",
    serialize = "CreateArrayLiteral"
  )]
  CreateArrayLiteral,
  #[strum(
    serialize = "Runtime_CreateRegExpLiteral",
    serialize = "CreateRegExpLiteral"
  )]
  CreateRegExpLiteral,
  #[strum(serialize = "Runtime_ForInEnumerate", serialize = "ForInEnumerate")]
  ForInEnumerate,
  #[strum(serialize = "Runtime_ForInHasProperty", serialize = "ForInHasProperty")]
  ForInHasProperty,
  #[strum(
    serialize = "Runtime_AtomicsNumWaitersForTesting",
    serialize = "AtomicsNumWaitersForTesting"
  )]
  AtomicsNumWaitersForTesting,
  #[strum(
    serialize = "Runtime_AtomicsNumUnresolvedAsyncPromisesForTesting",
    serialize = "AtomicsNumUnresolvedAsyncPromisesForTesting"
  )]
  AtomicsNumUnresolvedAsyncPromisesForTesting,
  #[strum(
    serialize = "Runtime_SetAllowAtomicsWait",
    serialize = "SetAllowAtomicsWait"
  )]
  SetAllowAtomicsWait,
  #[strum(serialize = "Runtime_FormatList", serialize = "FormatList")]
  FormatList,
  #[strum(
    serialize = "Runtime_FormatListToParts",
    serialize = "FormatListToParts"
  )]
  FormatListToParts,
  #[strum(
    serialize = "Runtime_StringToLowerCaseIntl",
    serialize = "StringToLowerCaseIntl"
  )]
  StringToLowerCaseIntl,
  #[strum(
    serialize = "Runtime_StringToUpperCaseIntl",
    serialize = "StringToUpperCaseIntl"
  )]
  StringToUpperCaseIntl,
  #[strum(
    serialize = "Runtime_StringToLocaleLowerCase",
    serialize = "StringToLocaleLowerCase"
  )]
  StringToLocaleLowerCase,
  #[strum(
    serialize = "Runtime_ShadowRealmWrappedFunctionCreate",
    serialize = "ShadowRealmWrappedFunctionCreate"
  )]
  ShadowRealmWrappedFunctionCreate,
  #[strum(
    serialize = "Runtime_ShadowRealmImportValue",
    serialize = "ShadowRealmImportValue"
  )]
  ShadowRealmImportValue,
  #[strum(serialize = "Runtime_ShadowRealmThrow", serialize = "ShadowRealmThrow")]
  ShadowRealmThrow,
  #[strum(
    serialize = "Runtime_TraceUnoptimizedBytecodeEntry",
    serialize = "TraceUnoptimizedBytecodeEntry"
  )]
  TraceUnoptimizedBytecodeEntry,
  #[strum(
    serialize = "Runtime_TraceUnoptimizedBytecodeExit",
    serialize = "TraceUnoptimizedBytecodeExit"
  )]
  TraceUnoptimizedBytecodeExit,
  #[strum(
    serialize = "Runtime_TraceUpdateFeedback",
    serialize = "TraceUpdateFeedback"
  )]
  TraceUpdateFeedback,
  #[strum(
    serialize = "Runtime_DumpExecutionFrame",
    serialize = "DumpExecutionFrame"
  )]
  DumpExecutionFrame,
  #[strum(
    serialize = "Runtime_JSFinalizationRegistryRegisterWeakCellWithUnregisterToken",
    serialize = "JSFinalizationRegistryRegisterWeakCellWithUnregisterToken"
  )]
  JSFinalizationRegistryRegisterWeakCellWithUnregisterToken,
  #[strum(
    serialize = "Runtime_JSWeakRefAddToKeptObjects",
    serialize = "JSWeakRefAddToKeptObjects"
  )]
  JSWeakRefAddToKeptObjects,
}

impl MiscRuntimeFunction {
  pub const fn name(&self) -> &'static str {
    match self {
      Self::ThrowConstAssignError => "ThrowConstAssignError",
      Self::ThrowUsingAssignError => "ThrowUsingAssignError",
      Self::ThrowAwaitUsingAssignError => "ThrowAwaitUsingAssignError",
      Self::DeclareModuleExports => "DeclareModuleExports",
      Self::DeclareGlobals => "DeclareGlobals",
      Self::InitializeDisposableStack => "InitializeDisposableStack",
      Self::AddDisposableValue => "AddDisposableValue",
      Self::AddAsyncDisposableValue => "AddAsyncDisposableValue",
      Self::DisposeDisposableStack => "DisposeDisposableStack",
      Self::HandleExceptionsInDisposeDisposableStack => "HandleExceptionsInDisposeDisposableStack",
      Self::DeclareEvalFunction => "DeclareEvalFunction",
      Self::DeclareEvalVar => "DeclareEvalVar",
      Self::NewSloppyArguments => "NewSloppyArguments",
      Self::NewStrictArguments => "NewStrictArguments",
      Self::NewRestParameter => "NewRestParameter",
      Self::NewClosure => "NewClosure",
      Self::NewClosureTenured => "NewClosure_Tenured",
      Self::NewFunctionContext => "NewFunctionContext",
      Self::PushWithContext => "PushWithContext",
      Self::PushCatchContext => "PushCatchContext",
      Self::PushBlockContext => "PushBlockContext",
      Self::DeleteLookupSlot => "DeleteLookupSlot",
      Self::LoadLookupSlot => "LoadLookupSlot",
      Self::LoadLookupSlotInsideTypeof => "LoadLookupSlotInsideTypeof",
      Self::LoadLookupSlotForCall => "LoadLookupSlotForCall",
      Self::LoadLookupSlotForCallBaseline => "LoadLookupSlotForCall_Baseline",
      Self::StoreLookupSlotSloppy => "StoreLookupSlot_Sloppy",
      Self::StoreLookupSlotStrict => "StoreLookupSlot_Strict",
      Self::StoreLookupSlotSloppyHoisting => "StoreLookupSlot_SloppyHoisting",
      Self::StoreGlobalNoHoleCheckForReplLetOrConst => "StoreGlobalNoHoleCheckForReplLetOrConst",
      Self::DynamicImportCall => "DynamicImportCall",
      Self::GetModuleNamespace => "GetModuleNamespace",
      Self::GetImportMetaObject => "GetImportMetaObject",
      Self::GetModuleNamespaceExport => "GetModuleNamespaceExport",
      Self::IterableForEach => "IterableForEach",
      Self::DebugBreakOnBytecode => "DebugBreakOnBytecode",
      Self::DebugBreakAtEntry => "DebugBreakAtEntry",
      Self::HandleDebuggerStatement => "HandleDebuggerStatement",
      Self::ScheduleBreak => "ScheduleBreak",
      Self::GetGeneratorScopeCount => "GetGeneratorScopeCount",
      Self::GetGeneratorScopeDetails => "GetGeneratorScopeDetails",
      Self::SetGeneratorScopeVariableValue => "SetGeneratorScopeVariableValue",
      Self::GetBreakLocations => "GetBreakLocations",
      Self::IsBreakOnException => "IsBreakOnException",
      Self::ClearStepping => "ClearStepping",
      Self::DebugGetLoadedScriptIds => "DebugGetLoadedScriptIds",
      Self::FunctionGetInferredName => "FunctionGetInferredName",
      Self::CollectGarbage => "CollectGarbage",
      Self::ScriptLocationFromLine2 => "ScriptLocationFromLine2",
      Self::DebugOnFunctionCall => "DebugOnFunctionCall",
      Self::DebugPrepareStepInSuspendedGenerator => "DebugPrepareStepInSuspendedGenerator",
      Self::DebugCollectCoverage => "DebugCollectCoverage",
      Self::DebugCollectWasmCoverage => "DebugCollectWasmCoverage",
      Self::DebugTogglePreciseCoverage => "DebugTogglePreciseCoverage",
      Self::DebugToggleBlockCoverage => "DebugToggleBlockCoverage",
      Self::IncBlockCounter => "IncBlockCounter",
      Self::DebugAsyncFunctionSuspended => "DebugAsyncFunctionSuspended",
      Self::DebugPromiseThen => "DebugPromiseThen",
      Self::ProfileCreateSnapshotDataBlob => "ProfileCreateSnapshotDataBlob",
      Self::CompileLazy => "CompileLazy",
      Self::InstallBaselineCode => "InstallBaselineCode",
      Self::InstallSFICode => "InstallSFICode",
      Self::StartMaglevOptimizeJob => "StartMaglevOptimizeJob",
      Self::StartTurbofanOptimizeJob => "StartTurbofanOptimizeJob",
      Self::OptimizeMaglevEager => "OptimizeMaglevEager",
      Self::OptimizeTurbofanEager => "OptimizeTurbofanEager",
      Self::MarkLazyDeoptimizedOrFlushed => "MarkLazyDeoptimizedOrFlushed",
      Self::FunctionLogNextExecution => "FunctionLogNextExecution",
      Self::PrintDumpedFrame => "PrintDumpedFrame",
      Self::NotifyDeoptimized => "NotifyDeoptimized",
      Self::ObserveNode => "ObserveNode",
      Self::VerifyType => "VerifyType",
      Self::CheckTurboshaftTypeOf => "CheckTurboshaftTypeOf",
      Self::CompileOptimizedOSR => "CompileOptimizedOSR",
      Self::CompileOptimizedOSRFromMaglev => "CompileOptimizedOSRFromMaglev",
      Self::CompileOptimizedOSRFromMaglevInlined => "CompileOptimizedOSRFromMaglevInlined",
      Self::LogOrTraceOptimizedOSREntry => "LogOrTraceOptimizedOSREntry",
      Self::ResolvePossiblyDirectEval => "ResolvePossiblyDirectEval",
      Self::PatchCompareOpBaselineCode => "PatchCompareOpBaselineCode",
      Self::PatchCompareOpBaselineCodeAndThrow => "PatchCompareOpBaselineCodeAndThrow",
      Self::PatchBinopBaselineCode => "PatchBinopBaselineCode",
      Self::PatchBinopBaselineCodeAndThrow => "PatchBinopBaselineCodeAndThrow",
      Self::PatchUnaryOpBaselineCode => "PatchUnaryOpBaselineCode",
      Self::PatchUnaryOpBaselineCodeAndThrow => "PatchUnaryOpBaselineCodeAndThrow",
      Self::AccessCheck => "AccessCheck",
      Self::FatalProcessOutOfMemoryInAllocateRaw => "FatalProcessOutOfMemoryInAllocateRaw",
      Self::FatalProcessOutOfMemoryInvalidArrayLength => {
        "FatalProcessOutOfMemoryInvalidArrayLength"
      }
      Self::FatalInvalidSize => "FatalInvalidSize",
      Self::Throw => "Throw",
      Self::ReThrow => "ReThrow",
      Self::ReThrowWithMessage => "ReThrowWithMessage",
      Self::ThrowStackOverflow => "ThrowStackOverflow",
      Self::VarargStackOverflow => "VarargStackOverflow",
      Self::ThrowSymbolAsyncIteratorInvalid => "ThrowSymbolAsyncIteratorInvalid",
      Self::TerminateExecution => "TerminateExecution",
      Self::ThrowRangeError => "ThrowRangeError",
      Self::ThrowTypeError => "ThrowTypeError",
      Self::ThrowTypeErrorIfStrict => "ThrowTypeErrorIfStrict",
      Self::ThrowInvalidTypedArrayAlignment => "ThrowInvalidTypedArrayAlignment",
      Self::UnwindAndFindExceptionHandler => "UnwindAndFindExceptionHandler",
      Self::PropagateException => "PropagateException",
      Self::ThrowReferenceError => "ThrowReferenceError",
      Self::ThrowAccessedUninitializedVariable => "ThrowAccessedUninitializedVariable",
      Self::NewError => "NewError",
      Self::NewTypeError => "NewTypeError",
      Self::NewReferenceError => "NewReferenceError",
      Self::ThrowInvalidStringLength => "ThrowInvalidStringLength",
      Self::ThrowIteratorResultNotAnObject => "ThrowIteratorResultNotAnObject",
      Self::ThrowThrowMethodMissing => "ThrowThrowMethodMissing",
      Self::ThrowSymbolIteratorInvalid => "ThrowSymbolIteratorInvalid",
      Self::ThrowNoAccess => "ThrowNoAccess",
      Self::ThrowNotConstructor => "ThrowNotConstructor",
      Self::ThrowTargetNonFunction => "ThrowTargetNonFunction",
      Self::StackGuard => "StackGuard",
      Self::HandleNoHeapWritesInterrupts => "HandleNoHeapWritesInterrupts",
      Self::StackGuardWithGap => "StackGuardWithGap",
      Self::BytecodeBudgetInterruptWithStackCheckIgnition => {
        "BytecodeBudgetInterruptWithStackCheck_Ignition"
      }
      Self::BytecodeBudgetInterruptIgnition => "BytecodeBudgetInterrupt_Ignition",
      Self::BytecodeBudgetInterruptWithStackCheckSparkplug => {
        "BytecodeBudgetInterruptWithStackCheck_Sparkplug"
      }
      Self::BytecodeBudgetInterruptSparkplug => "BytecodeBudgetInterrupt_Sparkplug",
      Self::BytecodeBudgetInterruptMaglev => "BytecodeBudgetInterrupt_Maglev",
      Self::BytecodeBudgetLoopInterruptMaglev => "BytecodeBudgetLoopInterrupt_Maglev",
      Self::AllocateInYoungGeneration => "AllocateInYoungGeneration",
      Self::AllocateInOldGeneration => "AllocateInOldGeneration",
      Self::AllocateInSharedHeap => "AllocateInSharedHeap",
      Self::AllocateByteArray => "AllocateByteArray",
      Self::ThrowIteratorError => "ThrowIteratorError",
      Self::ThrowSpreadArgError => "ThrowSpreadArgError",
      Self::ThrowCalledNonCallable => "ThrowCalledNonCallable",
      Self::ThrowConstructedNonConstructable => "ThrowConstructedNonConstructable",
      Self::ThrowPatternAssignmentNonCoercible => "ThrowPatternAssignmentNonCoercible",
      Self::ThrowConstructorReturnedNonObject => "ThrowConstructorReturnedNonObject",
      Self::CreateListFromArrayLike => "CreateListFromArrayLike",
      Self::IncrementUseCounter => "IncrementUseCounter",
      Self::GetAndResetTurboProfilingData => "GetAndResetTurboProfilingData",
      Self::GetAndResetRuntimeCallStats => "GetAndResetRuntimeCallStats",
      Self::OrdinaryHasInstance => "OrdinaryHasInstance",
      Self::Typeof => "Typeof",
      Self::AllowDynamicFunction => "AllowDynamicFunction",
      Self::CreateAsyncFromSyncIterator => "CreateAsyncFromSyncIterator",
      Self::GetTemplateObject => "GetTemplateObject",
      Self::ReportMessageFromMicrotask => "ReportMessageFromMicrotask",
      Self::GetInitializerFunction => "GetInitializerFunction",
      Self::DoubleToStringWithRadix => "DoubleToStringWithRadix",
      Self::SharedValueBarrierSlow => "SharedValueBarrierSlow",
      Self::NotifyContextCellStateWillChange => "NotifyContextCellStateWillChange",
      Self::InvalidateStringWrapperToPrimitiveProtector => {
        "InvalidateStringWrapperToPrimitiveProtector"
      }
      Self::AddLhsIsStringConstantInternalize => "AddLhsIsStringConstantInternalize",
      Self::AddRhsIsStringConstantInternalize => "AddRhsIsStringConstantInternalize",
      Self::WasmWasmToJSObject => "WasmWasmToJSObject",
      Self::WasmJSToWasmObject => "WasmJSToWasmObject",
      Self::WasmMemoryGrow => "WasmMemoryGrow",
      Self::ThrowWasmError => "ThrowWasmError",
      Self::ThrowWasmStackOverflow => "ThrowWasmStackOverflow",
      Self::WasmThrowJSTypeError => "WasmThrowJSTypeError",
      Self::ThrowWasmJSPISuspendError => "ThrowWasmJSPISuspendError",
      Self::ThrowWasmFXSuspendError => "ThrowWasmFXSuspendError",
      Self::WasmThrowRangeError => "WasmThrowRangeError",
      Self::WasmThrowDataViewTypeError => "WasmThrowDataViewTypeError",
      Self::WasmThrowDataViewDetachedError => "WasmThrowDataViewDetachedError",
      Self::WasmThrowTypeError => "WasmThrowTypeError",
      Self::WasmThrow => "WasmThrow",
      Self::WasmReThrow => "WasmReThrow",
      Self::WasmStackGuard => "WasmStackGuard",
      Self::WasmStackGuardLoop => "WasmStackGuardLoop",
      Self::WasmCompileLazy => "WasmCompileLazy",
      Self::WasmAllocateFeedbackVector => "WasmAllocateFeedbackVector",
      Self::WasmLiftoffDeoptFinish => "WasmLiftoffDeoptFinish",
      Self::TierUpJSToWasmWrapper => "TierUpJSToWasmWrapper",
      Self::IsWasmExternalFunction => "IsWasmExternalFunction",
      Self::TierUpWasmToJSWrapper => "TierUpWasmToJSWrapper",
      Self::WasmTriggerTierUp => "WasmTriggerTierUp",
      Self::WasmI32AtomicWait => "WasmI32AtomicWait",
      Self::WasmI64AtomicWait => "WasmI64AtomicWait",
      Self::WasmManagedObjectWait32 => "WasmManagedObjectWait32",
      Self::WasmManagedObjectWait64 => "WasmManagedObjectWait64",
      Self::WasmManagedObjectWaitRef => "WasmManagedObjectWaitRef",
      Self::WasmWaitqueueNew => "WasmWaitqueueNew",
      Self::WasmRefFunc => "WasmRefFunc",
      Self::WasmInternalFunctionCreateExternal => "WasmInternalFunctionCreateExternal",
      Self::WasmFunctionTableGet => "WasmFunctionTableGet",
      Self::WasmFunctionTableSet => "WasmFunctionTableSet",
      Self::WasmTableInit => "WasmTableInit",
      Self::WasmTableCopy => "WasmTableCopy",
      Self::WasmTableGrow => "WasmTableGrow",
      Self::WasmTableFill => "WasmTableFill",
      Self::WasmDebugBreak => "WasmDebugBreak",
      Self::WasmArrayCopy => "WasmArrayCopy",
      Self::WasmAllocateDescriptorStruct => "WasmAllocateDescriptorStruct",
      Self::WasmArrayNewSegment => "WasmArrayNewSegment",
      Self::WasmArrayInitSegment => "WasmArrayInitSegment",
      Self::WasmAllocateSuspender => "WasmAllocateSuspender",
      Self::WasmSuspended => "WasmSuspended",
      Self::ClearWasmSuspenderResumeField => "ClearWasmSuspenderResumeField",
      Self::WasmConfigureAllPrototypes => "WasmConfigureAllPrototypes",
      Self::WasmConfigureAllPrototypesOpt => "WasmConfigureAllPrototypesOpt",
      Self::WasmCastToSpecialPrimitiveArray => "WasmCastToSpecialPrimitiveArray",
      Self::WasmStringNewWtf8 => "WasmStringNewWtf8",
      Self::WasmStringNewWtf8Array => "WasmStringNewWtf8Array",
      Self::WasmStringNewWtf16 => "WasmStringNewWtf16",
      Self::WasmStringNewWtf16Array => "WasmStringNewWtf16Array",
      Self::WasmSubstring => "WasmSubstring",
      Self::WasmSubstringShared => "WasmSubstringShared",
      Self::WasmStringConst => "WasmStringConst",
      Self::WasmStringNewSegmentWtf8 => "WasmStringNewSegmentWtf8",
      Self::WasmStringMeasureUtf8 => "WasmStringMeasureUtf8",
      Self::WasmStringMeasureWtf8 => "WasmStringMeasureWtf8",
      Self::WasmStringEncodeWtf8 => "WasmStringEncodeWtf8",
      Self::WasmStringEncodeWtf8Array => "WasmStringEncodeWtf8Array",
      Self::WasmStringToUtf8Array => "WasmStringToUtf8Array",
      Self::WasmStringEncodeWtf16 => "WasmStringEncodeWtf16",
      Self::WasmStringAsWtf8 => "WasmStringAsWtf8",
      Self::WasmStringViewWtf8Encode => "WasmStringViewWtf8Encode",
      Self::WasmStringViewWtf8Slice => "WasmStringViewWtf8Slice",
      Self::WasmTraceBeginExecution => "WasmTraceBeginExecution",
      Self::WasmTraceEndExecution => "WasmTraceEndExecution",
      Self::WasmStringFromCodePoint => "WasmStringFromCodePoint",
      Self::WasmStringHash => "WasmStringHash",
      Self::WasmStringAddNoMapCheckShared => "WasmStringAdd_NoMapCheck_Shared",
      Self::WasmAllocateContinuation => "WasmAllocateContinuation",
      Self::WasmTypeAssertionFailed => "WasmTypeAssertionFailed",
      Self::TsanAcquireForInitializationFence => "TsanAcquireForInitializationFence",
      Self::FunctionGetScriptSource => "FunctionGetScriptSource",
      Self::FunctionGetScriptId => "FunctionGetScriptId",
      Self::FunctionGetSourceCode => "FunctionGetSourceCode",
      Self::FunctionGetScriptSourcePosition => "FunctionGetScriptSourcePosition",
      Self::FunctionIsAPIFunction => "FunctionIsAPIFunction",
      Self::Call => "Call",
      Self::AsyncFunctionAwait => "AsyncFunctionAwait",
      Self::AsyncFunctionEnter => "AsyncFunctionEnter",
      Self::AsyncFunctionReject => "AsyncFunctionReject",
      Self::AsyncFunctionResolve => "AsyncFunctionResolve",
      Self::CreateJSGeneratorObject => "CreateJSGeneratorObject",
      Self::GeneratorClose => "GeneratorClose",
      Self::GeneratorGetFunction => "GeneratorGetFunction",
      Self::AsyncGeneratorAwait => "AsyncGeneratorAwait",
      Self::AsyncGeneratorResolve => "AsyncGeneratorResolve",
      Self::AsyncGeneratorReject => "AsyncGeneratorReject",
      Self::AsyncGeneratorYieldWithAwait => "AsyncGeneratorYieldWithAwait",
      Self::GeneratorGetResumeMode => "GeneratorGetResumeMode",
      Self::GeneratorYieldResult => "GeneratorYieldResult",
      Self::PromiseRejectEventFromStack => "PromiseRejectEventFromStack",
      Self::PromiseRevokeReject => "PromiseRevokeReject",
      Self::EnqueueMicrotask => "EnqueueMicrotask",
      Self::PerformMicrotaskCheckpoint => "PerformMicrotaskCheckpoint",
      Self::RunMicrotaskCallback => "RunMicrotaskCallback",
      Self::PromiseHookInit => "PromiseHookInit",
      Self::PromiseHookBefore => "PromiseHookBefore",
      Self::PromiseHookAfter => "PromiseHookAfter",
      Self::RejectPromise => "RejectPromise",
      Self::ResolvePromise => "ResolvePromise",
      Self::ConstructAggregateErrorHelper => "ConstructAggregateErrorHelper",
      Self::ConstructInternalAggregateErrorHelper => "ConstructInternalAggregateErrorHelper",
      Self::ConstructSuppressedError => "ConstructSuppressedError",
      Self::StringSplit => "StringSplit",
      Self::RegExpExec => "RegExpExec",
      Self::RegExpGrowRegExpMatchInfo => "RegExpGrowRegExpMatchInfo",
      Self::RegExpExperimentalOneshotExec => "RegExpExperimentalOneshotExec",
      Self::RegExpBuildIndices => "RegExpBuildIndices",
      Self::RegExpExecMultiple => "RegExpExecMultiple",
      Self::StringReplaceNonGlobalRegExpWithFunction => "StringReplaceNonGlobalRegExpWithFunction",
      Self::RegExpSplit => "RegExpSplit",
      Self::RegExpReplaceRT => "RegExpReplaceRT",
      Self::RegExpInitializeAndCompile => "RegExpInitializeAndCompile",
      Self::RegExpStringFromFlags => "RegExpStringFromFlags",
      Self::RegExpMatchGlobalAtom => "RegExpMatchGlobalAtom",
      Self::CreatePrivateSymbol => "CreatePrivateSymbol",
      Self::CreatePrivateBrandSymbol => "CreatePrivateBrandSymbol",
      Self::CreatePrivateNameSymbol => "CreatePrivateNameSymbol",
      Self::SymbolDescriptiveString => "SymbolDescriptiveString",
      Self::SymbolIsPrivate => "SymbolIsPrivate",
      Self::CreateObjectLiteral => "CreateObjectLiteral",
      Self::SetPrototypeProperties => "SetPrototypeProperties",
      Self::CreateArrayLiteral => "CreateArrayLiteral",
      Self::CreateRegExpLiteral => "CreateRegExpLiteral",
      Self::ForInEnumerate => "ForInEnumerate",
      Self::ForInHasProperty => "ForInHasProperty",
      Self::AtomicsNumWaitersForTesting => "AtomicsNumWaitersForTesting",
      Self::AtomicsNumUnresolvedAsyncPromisesForTesting => {
        "AtomicsNumUnresolvedAsyncPromisesForTesting"
      }
      Self::SetAllowAtomicsWait => "SetAllowAtomicsWait",
      Self::FormatList => "FormatList",
      Self::FormatListToParts => "FormatListToParts",
      Self::StringToLowerCaseIntl => "StringToLowerCaseIntl",
      Self::StringToUpperCaseIntl => "StringToUpperCaseIntl",
      Self::StringToLocaleLowerCase => "StringToLocaleLowerCase",
      Self::ShadowRealmWrappedFunctionCreate => "ShadowRealmWrappedFunctionCreate",
      Self::ShadowRealmImportValue => "ShadowRealmImportValue",
      Self::ShadowRealmThrow => "ShadowRealmThrow",
      Self::TraceUnoptimizedBytecodeEntry => "TraceUnoptimizedBytecodeEntry",
      Self::TraceUnoptimizedBytecodeExit => "TraceUnoptimizedBytecodeExit",
      Self::TraceUpdateFeedback => "TraceUpdateFeedback",
      Self::DumpExecutionFrame => "DumpExecutionFrame",
      Self::JSFinalizationRegistryRegisterWeakCellWithUnregisterToken => {
        "JSFinalizationRegistryRegisterWeakCellWithUnregisterToken"
      }
      Self::JSWeakRefAddToKeptObjects => "JSWeakRefAddToKeptObjects",
    }
  }

  pub fn descriptor(&self) -> Option<&'static RuntimeFunction> {
    find_runtime_function(self.name())
  }

  pub fn function_id(&self) -> Option<RuntimeFunctionId> {
    self.descriptor().map(|f| f.id)
  }

  pub const fn is_scopes(&self) -> bool {
    matches!(
      self,
      Self::ThrowConstAssignError
        | Self::ThrowUsingAssignError
        | Self::ThrowAwaitUsingAssignError
        | Self::DeclareModuleExports
        | Self::DeclareGlobals
        | Self::InitializeDisposableStack
        | Self::AddDisposableValue
        | Self::AddAsyncDisposableValue
        | Self::DisposeDisposableStack
        | Self::HandleExceptionsInDisposeDisposableStack
        | Self::DeclareEvalFunction
        | Self::DeclareEvalVar
        | Self::NewSloppyArguments
        | Self::NewStrictArguments
        | Self::NewRestParameter
        | Self::NewClosure
        | Self::NewClosureTenured
        | Self::NewFunctionContext
        | Self::PushWithContext
        | Self::PushCatchContext
        | Self::PushBlockContext
        | Self::DeleteLookupSlot
        | Self::LoadLookupSlot
        | Self::LoadLookupSlotInsideTypeof
        | Self::LoadLookupSlotForCall
        | Self::LoadLookupSlotForCallBaseline
        | Self::StoreLookupSlotSloppy
        | Self::StoreLookupSlotStrict
        | Self::StoreLookupSlotSloppyHoisting
        | Self::StoreGlobalNoHoleCheckForReplLetOrConst
    )
  }

  pub const fn is_module(&self) -> bool {
    matches!(
      self,
      Self::DynamicImportCall
        | Self::GetModuleNamespace
        | Self::GetImportMetaObject
        | Self::GetModuleNamespaceExport
    )
  }

  pub const fn is_debug(&self) -> bool {
    matches!(
      self,
      Self::IterableForEach
        | Self::DebugBreakOnBytecode
        | Self::DebugBreakAtEntry
        | Self::HandleDebuggerStatement
        | Self::ScheduleBreak
        | Self::GetGeneratorScopeCount
        | Self::GetGeneratorScopeDetails
        | Self::SetGeneratorScopeVariableValue
        | Self::GetBreakLocations
        | Self::IsBreakOnException
        | Self::ClearStepping
        | Self::DebugGetLoadedScriptIds
        | Self::FunctionGetInferredName
        | Self::CollectGarbage
        | Self::ScriptLocationFromLine2
        | Self::DebugOnFunctionCall
        | Self::DebugPrepareStepInSuspendedGenerator
        | Self::DebugCollectCoverage
        | Self::DebugCollectWasmCoverage
        | Self::DebugTogglePreciseCoverage
        | Self::DebugToggleBlockCoverage
        | Self::IncBlockCounter
        | Self::DebugAsyncFunctionSuspended
        | Self::DebugPromiseThen
        | Self::ProfileCreateSnapshotDataBlob
    )
  }

  pub const fn is_compiler(&self) -> bool {
    matches!(
      self,
      Self::CompileLazy
        | Self::InstallBaselineCode
        | Self::InstallSFICode
        | Self::StartMaglevOptimizeJob
        | Self::StartTurbofanOptimizeJob
        | Self::OptimizeMaglevEager
        | Self::OptimizeTurbofanEager
        | Self::MarkLazyDeoptimizedOrFlushed
        | Self::FunctionLogNextExecution
        | Self::PrintDumpedFrame
        | Self::NotifyDeoptimized
        | Self::ObserveNode
        | Self::VerifyType
        | Self::CheckTurboshaftTypeOf
        | Self::CompileOptimizedOSR
        | Self::CompileOptimizedOSRFromMaglev
        | Self::CompileOptimizedOSRFromMaglevInlined
        | Self::LogOrTraceOptimizedOSREntry
        | Self::ResolvePossiblyDirectEval
        | Self::PatchCompareOpBaselineCode
        | Self::PatchCompareOpBaselineCodeAndThrow
        | Self::PatchBinopBaselineCode
        | Self::PatchBinopBaselineCodeAndThrow
        | Self::PatchUnaryOpBaselineCode
        | Self::PatchUnaryOpBaselineCodeAndThrow
    )
  }

  pub const fn is_internal(&self) -> bool {
    matches!(
      self,
      Self::AccessCheck
        | Self::FatalProcessOutOfMemoryInAllocateRaw
        | Self::FatalProcessOutOfMemoryInvalidArrayLength
        | Self::FatalInvalidSize
        | Self::Throw
        | Self::ReThrow
        | Self::ReThrowWithMessage
        | Self::ThrowStackOverflow
        | Self::VarargStackOverflow
        | Self::ThrowSymbolAsyncIteratorInvalid
        | Self::TerminateExecution
        | Self::ThrowRangeError
        | Self::ThrowTypeError
        | Self::ThrowTypeErrorIfStrict
        | Self::ThrowInvalidTypedArrayAlignment
        | Self::UnwindAndFindExceptionHandler
        | Self::PropagateException
        | Self::ThrowReferenceError
        | Self::ThrowAccessedUninitializedVariable
        | Self::NewError
        | Self::NewTypeError
        | Self::NewReferenceError
        | Self::ThrowInvalidStringLength
        | Self::ThrowIteratorResultNotAnObject
        | Self::ThrowThrowMethodMissing
        | Self::ThrowSymbolIteratorInvalid
        | Self::ThrowNoAccess
        | Self::ThrowNotConstructor
        | Self::ThrowTargetNonFunction
        | Self::StackGuard
        | Self::HandleNoHeapWritesInterrupts
        | Self::StackGuardWithGap
        | Self::BytecodeBudgetInterruptWithStackCheckIgnition
        | Self::BytecodeBudgetInterruptIgnition
        | Self::BytecodeBudgetInterruptWithStackCheckSparkplug
        | Self::BytecodeBudgetInterruptSparkplug
        | Self::BytecodeBudgetInterruptMaglev
        | Self::BytecodeBudgetLoopInterruptMaglev
        | Self::AllocateInYoungGeneration
        | Self::AllocateInOldGeneration
        | Self::AllocateInSharedHeap
        | Self::AllocateByteArray
        | Self::ThrowIteratorError
        | Self::ThrowSpreadArgError
        | Self::ThrowCalledNonCallable
        | Self::ThrowConstructedNonConstructable
        | Self::ThrowPatternAssignmentNonCoercible
        | Self::ThrowConstructorReturnedNonObject
        | Self::CreateListFromArrayLike
        | Self::IncrementUseCounter
        | Self::GetAndResetTurboProfilingData
        | Self::GetAndResetRuntimeCallStats
        | Self::OrdinaryHasInstance
        | Self::Typeof
        | Self::AllowDynamicFunction
        | Self::CreateAsyncFromSyncIterator
        | Self::GetTemplateObject
        | Self::ReportMessageFromMicrotask
        | Self::GetInitializerFunction
        | Self::DoubleToStringWithRadix
        | Self::SharedValueBarrierSlow
        | Self::NotifyContextCellStateWillChange
        | Self::InvalidateStringWrapperToPrimitiveProtector
        | Self::AddLhsIsStringConstantInternalize
        | Self::AddRhsIsStringConstantInternalize
    )
  }

  pub const fn is_wasm(&self) -> bool {
    matches!(
      self,
      Self::WasmWasmToJSObject
        | Self::WasmJSToWasmObject
        | Self::WasmMemoryGrow
        | Self::ThrowWasmError
        | Self::ThrowWasmStackOverflow
        | Self::WasmThrowJSTypeError
        | Self::ThrowWasmJSPISuspendError
        | Self::ThrowWasmFXSuspendError
        | Self::WasmThrowRangeError
        | Self::WasmThrowDataViewTypeError
        | Self::WasmThrowDataViewDetachedError
        | Self::WasmThrowTypeError
        | Self::WasmThrow
        | Self::WasmReThrow
        | Self::WasmStackGuard
        | Self::WasmStackGuardLoop
        | Self::WasmCompileLazy
        | Self::WasmAllocateFeedbackVector
        | Self::WasmLiftoffDeoptFinish
        | Self::TierUpJSToWasmWrapper
        | Self::IsWasmExternalFunction
        | Self::TierUpWasmToJSWrapper
        | Self::WasmTriggerTierUp
        | Self::WasmI32AtomicWait
        | Self::WasmI64AtomicWait
        | Self::WasmManagedObjectWait32
        | Self::WasmManagedObjectWait64
        | Self::WasmManagedObjectWaitRef
        | Self::WasmWaitqueueNew
        | Self::WasmRefFunc
        | Self::WasmInternalFunctionCreateExternal
        | Self::WasmFunctionTableGet
        | Self::WasmFunctionTableSet
        | Self::WasmTableInit
        | Self::WasmTableCopy
        | Self::WasmTableGrow
        | Self::WasmTableFill
        | Self::WasmDebugBreak
        | Self::WasmArrayCopy
        | Self::WasmAllocateDescriptorStruct
        | Self::WasmArrayNewSegment
        | Self::WasmArrayInitSegment
        | Self::WasmAllocateSuspender
        | Self::WasmSuspended
        | Self::ClearWasmSuspenderResumeField
        | Self::WasmConfigureAllPrototypes
        | Self::WasmConfigureAllPrototypesOpt
        | Self::WasmCastToSpecialPrimitiveArray
        | Self::WasmStringNewWtf8
        | Self::WasmStringNewWtf8Array
        | Self::WasmStringNewWtf16
        | Self::WasmStringNewWtf16Array
        | Self::WasmSubstring
        | Self::WasmSubstringShared
        | Self::WasmStringConst
        | Self::WasmStringNewSegmentWtf8
        | Self::WasmStringMeasureUtf8
        | Self::WasmStringMeasureWtf8
        | Self::WasmStringEncodeWtf8
        | Self::WasmStringEncodeWtf8Array
        | Self::WasmStringToUtf8Array
        | Self::WasmStringEncodeWtf16
        | Self::WasmStringAsWtf8
        | Self::WasmStringViewWtf8Encode
        | Self::WasmStringViewWtf8Slice
        | Self::WasmTraceBeginExecution
        | Self::WasmTraceEndExecution
        | Self::WasmStringFromCodePoint
        | Self::WasmStringHash
        | Self::WasmStringAddNoMapCheckShared
        | Self::WasmAllocateContinuation
        | Self::WasmTypeAssertionFailed
        | Self::TsanAcquireForInitializationFence
    )
  }

  pub const fn is_function(&self) -> bool {
    matches!(
      self,
      Self::FunctionGetScriptSource
        | Self::FunctionGetScriptId
        | Self::FunctionGetSourceCode
        | Self::FunctionGetScriptSourcePosition
        | Self::FunctionIsAPIFunction
        | Self::Call
    )
  }

  pub const fn is_generator(&self) -> bool {
    matches!(
      self,
      Self::AsyncFunctionAwait
        | Self::AsyncFunctionEnter
        | Self::AsyncFunctionReject
        | Self::AsyncFunctionResolve
        | Self::CreateJSGeneratorObject
        | Self::GeneratorClose
        | Self::GeneratorGetFunction
        | Self::AsyncGeneratorAwait
        | Self::AsyncGeneratorResolve
        | Self::AsyncGeneratorReject
        | Self::AsyncGeneratorYieldWithAwait
        | Self::GeneratorGetResumeMode
        | Self::GeneratorYieldResult
    )
  }

  pub const fn is_promise(&self) -> bool {
    matches!(
      self,
      Self::PromiseRejectEventFromStack
        | Self::PromiseRevokeReject
        | Self::EnqueueMicrotask
        | Self::PerformMicrotaskCheckpoint
        | Self::RunMicrotaskCallback
        | Self::PromiseHookInit
        | Self::PromiseHookBefore
        | Self::PromiseHookAfter
        | Self::RejectPromise
        | Self::ResolvePromise
        | Self::ConstructAggregateErrorHelper
        | Self::ConstructInternalAggregateErrorHelper
        | Self::ConstructSuppressedError
    )
  }

  pub const fn is_regexp(&self) -> bool {
    matches!(
      self,
      Self::StringSplit
        | Self::RegExpExec
        | Self::RegExpGrowRegExpMatchInfo
        | Self::RegExpExperimentalOneshotExec
        | Self::RegExpBuildIndices
        | Self::RegExpExecMultiple
        | Self::StringReplaceNonGlobalRegExpWithFunction
        | Self::RegExpSplit
        | Self::RegExpReplaceRT
        | Self::RegExpInitializeAndCompile
        | Self::RegExpStringFromFlags
        | Self::RegExpMatchGlobalAtom
    )
  }

  pub const fn is_symbol(&self) -> bool {
    matches!(
      self,
      Self::CreatePrivateSymbol
        | Self::CreatePrivateBrandSymbol
        | Self::CreatePrivateNameSymbol
        | Self::SymbolDescriptiveString
        | Self::SymbolIsPrivate
    )
  }

  pub const fn is_literals(&self) -> bool {
    matches!(
      self,
      Self::CreateObjectLiteral
        | Self::SetPrototypeProperties
        | Self::CreateArrayLiteral
        | Self::CreateRegExpLiteral
    )
  }

  pub const fn is_forin(&self) -> bool {
    matches!(self, Self::ForInEnumerate | Self::ForInHasProperty)
  }

  pub const fn is_futex(&self) -> bool {
    matches!(
      self,
      Self::AtomicsNumWaitersForTesting
        | Self::AtomicsNumUnresolvedAsyncPromisesForTesting
        | Self::SetAllowAtomicsWait
    )
  }

  pub const fn is_intl(&self) -> bool {
    matches!(
      self,
      Self::FormatList
        | Self::FormatListToParts
        | Self::StringToLowerCaseIntl
        | Self::StringToUpperCaseIntl
        | Self::StringToLocaleLowerCase
    )
  }

  pub const fn is_shadow_realm(&self) -> bool {
    matches!(
      self,
      Self::ShadowRealmWrappedFunctionCreate
        | Self::ShadowRealmImportValue
        | Self::ShadowRealmThrow
    )
  }

  pub const fn is_trace(&self) -> bool {
    matches!(
      self,
      Self::TraceUnoptimizedBytecodeEntry
        | Self::TraceUnoptimizedBytecodeExit
        | Self::TraceUpdateFeedback
        | Self::DumpExecutionFrame
    )
  }

  pub const fn is_weak_refs(&self) -> bool {
    matches!(
      self,
      Self::JSFinalizationRegistryRegisterWeakCellWithUnregisterToken
        | Self::JSWeakRefAddToKeptObjects
    )
  }
}

pub const fn throw_const_assign_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowConstAssignError
}

pub const fn throw_using_assign_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowUsingAssignError
}

pub const fn throw_await_using_assign_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowAwaitUsingAssignError
}

pub const fn declare_module_exports() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DeclareModuleExports
}

pub const fn declare_globals() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DeclareGlobals
}

pub const fn initialize_disposable_stack() -> MiscRuntimeFunction {
  MiscRuntimeFunction::InitializeDisposableStack
}

pub const fn add_disposable_value() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AddDisposableValue
}

pub const fn add_async_disposable_value() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AddAsyncDisposableValue
}

pub const fn dispose_disposable_stack() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DisposeDisposableStack
}

pub const fn handle_exceptions_in_dispose_disposable_stack() -> MiscRuntimeFunction {
  MiscRuntimeFunction::HandleExceptionsInDisposeDisposableStack
}

pub const fn declare_eval_function() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DeclareEvalFunction
}

pub const fn declare_eval_var() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DeclareEvalVar
}

pub const fn new_sloppy_arguments() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NewSloppyArguments
}

pub const fn new_strict_arguments() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NewStrictArguments
}

pub const fn new_rest_parameter() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NewRestParameter
}

pub const fn new_closure() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NewClosure
}

pub const fn new_closure_tenured() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NewClosureTenured
}

pub const fn new_function_context() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NewFunctionContext
}

pub const fn push_with_context() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PushWithContext
}

pub const fn push_catch_context() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PushCatchContext
}

pub const fn push_block_context() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PushBlockContext
}

pub const fn delete_lookup_slot() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DeleteLookupSlot
}

pub const fn load_lookup_slot() -> MiscRuntimeFunction {
  MiscRuntimeFunction::LoadLookupSlot
}

pub const fn load_lookup_slot_inside_typeof() -> MiscRuntimeFunction {
  MiscRuntimeFunction::LoadLookupSlotInsideTypeof
}

pub const fn load_lookup_slot_for_call() -> MiscRuntimeFunction {
  MiscRuntimeFunction::LoadLookupSlotForCall
}

pub const fn load_lookup_slot_for_call_baseline() -> MiscRuntimeFunction {
  MiscRuntimeFunction::LoadLookupSlotForCallBaseline
}

pub const fn store_lookup_slot_sloppy() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StoreLookupSlotSloppy
}

pub const fn store_lookup_slot_strict() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StoreLookupSlotStrict
}

pub const fn store_lookup_slot_sloppy_hoisting() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StoreLookupSlotSloppyHoisting
}

pub const fn store_global_no_hole_check_for_repl_let_or_const() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StoreGlobalNoHoleCheckForReplLetOrConst
}

pub const fn dynamic_import_call() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DynamicImportCall
}

pub const fn get_module_namespace() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GetModuleNamespace
}

pub const fn get_import_meta_object() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GetImportMetaObject
}

pub const fn get_module_namespace_export() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GetModuleNamespaceExport
}

pub const fn iterable_for_each() -> MiscRuntimeFunction {
  MiscRuntimeFunction::IterableForEach
}

pub const fn debug_break_on_bytecode() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugBreakOnBytecode
}

pub const fn debug_break_at_entry() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugBreakAtEntry
}

pub const fn handle_debugger_statement() -> MiscRuntimeFunction {
  MiscRuntimeFunction::HandleDebuggerStatement
}

pub const fn schedule_break() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ScheduleBreak
}

pub const fn get_generator_scope_count() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GetGeneratorScopeCount
}

pub const fn get_generator_scope_details() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GetGeneratorScopeDetails
}

pub const fn set_generator_scope_variable_value() -> MiscRuntimeFunction {
  MiscRuntimeFunction::SetGeneratorScopeVariableValue
}

pub const fn get_break_locations() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GetBreakLocations
}

pub const fn is_break_on_exception() -> MiscRuntimeFunction {
  MiscRuntimeFunction::IsBreakOnException
}

pub const fn clear_stepping() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ClearStepping
}

pub const fn debug_get_loaded_script_ids() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugGetLoadedScriptIds
}

pub const fn function_get_inferred_name() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FunctionGetInferredName
}

pub const fn collect_garbage() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CollectGarbage
}

pub const fn script_location_from_line2() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ScriptLocationFromLine2
}

pub const fn debug_on_function_call() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugOnFunctionCall
}

pub const fn debug_prepare_step_in_suspended_generator() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugPrepareStepInSuspendedGenerator
}

pub const fn debug_collect_coverage() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugCollectCoverage
}

pub const fn debug_collect_wasm_coverage() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugCollectWasmCoverage
}

pub const fn debug_toggle_precise_coverage() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugTogglePreciseCoverage
}

pub const fn debug_toggle_block_coverage() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugToggleBlockCoverage
}

pub const fn inc_block_counter() -> MiscRuntimeFunction {
  MiscRuntimeFunction::IncBlockCounter
}

pub const fn debug_async_function_suspended() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugAsyncFunctionSuspended
}

pub const fn debug_promise_then() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DebugPromiseThen
}

pub const fn profile_create_snapshot_data_blob() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ProfileCreateSnapshotDataBlob
}

pub const fn compile_lazy() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CompileLazy
}

pub const fn install_baseline_code() -> MiscRuntimeFunction {
  MiscRuntimeFunction::InstallBaselineCode
}

pub const fn install_sfi_code() -> MiscRuntimeFunction {
  MiscRuntimeFunction::InstallSFICode
}

pub const fn start_maglev_optimize_job() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StartMaglevOptimizeJob
}

pub const fn start_turbofan_optimize_job() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StartTurbofanOptimizeJob
}

pub const fn optimize_maglev_eager() -> MiscRuntimeFunction {
  MiscRuntimeFunction::OptimizeMaglevEager
}

pub const fn optimize_turbofan_eager() -> MiscRuntimeFunction {
  MiscRuntimeFunction::OptimizeTurbofanEager
}

pub const fn mark_lazy_deoptimized_or_flushed() -> MiscRuntimeFunction {
  MiscRuntimeFunction::MarkLazyDeoptimizedOrFlushed
}

pub const fn function_log_next_execution() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FunctionLogNextExecution
}

pub const fn print_dumped_frame() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PrintDumpedFrame
}

pub const fn notify_deoptimized() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NotifyDeoptimized
}

pub const fn observe_node() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ObserveNode
}

pub const fn verify_type() -> MiscRuntimeFunction {
  MiscRuntimeFunction::VerifyType
}

pub const fn check_turboshaft_type_of() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CheckTurboshaftTypeOf
}

pub const fn compile_optimized_osr() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CompileOptimizedOSR
}

pub const fn compile_optimized_osr_from_maglev() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CompileOptimizedOSRFromMaglev
}

pub const fn compile_optimized_osr_from_maglev_inlined() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CompileOptimizedOSRFromMaglevInlined
}

pub const fn log_or_trace_optimized_osr_entry() -> MiscRuntimeFunction {
  MiscRuntimeFunction::LogOrTraceOptimizedOSREntry
}

pub const fn resolve_possibly_direct_eval() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ResolvePossiblyDirectEval
}

pub const fn patch_compare_op_baseline_code() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PatchCompareOpBaselineCode
}

pub const fn patch_compare_op_baseline_code_and_throw() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PatchCompareOpBaselineCodeAndThrow
}

pub const fn patch_binop_baseline_code() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PatchBinopBaselineCode
}

pub const fn patch_binop_baseline_code_and_throw() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PatchBinopBaselineCodeAndThrow
}

pub const fn patch_unary_op_baseline_code() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PatchUnaryOpBaselineCode
}

pub const fn patch_unary_op_baseline_code_and_throw() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PatchUnaryOpBaselineCodeAndThrow
}

pub const fn access_check() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AccessCheck
}

pub const fn fatal_process_out_of_memory_in_allocate_raw() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FatalProcessOutOfMemoryInAllocateRaw
}

pub const fn fatal_process_out_of_memory_invalid_array_length() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FatalProcessOutOfMemoryInvalidArrayLength
}

pub const fn fatal_invalid_size() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FatalInvalidSize
}

pub const fn throw() -> MiscRuntimeFunction {
  MiscRuntimeFunction::Throw
}

pub const fn re_throw() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ReThrow
}

pub const fn re_throw_with_message() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ReThrowWithMessage
}

pub const fn throw_stack_overflow() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowStackOverflow
}

pub const fn vararg_stack_overflow() -> MiscRuntimeFunction {
  MiscRuntimeFunction::VarargStackOverflow
}

pub const fn throw_symbol_async_iterator_invalid() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowSymbolAsyncIteratorInvalid
}

pub const fn terminate_execution() -> MiscRuntimeFunction {
  MiscRuntimeFunction::TerminateExecution
}

pub const fn throw_range_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowRangeError
}

pub const fn throw_type_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowTypeError
}

pub const fn throw_type_error_if_strict() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowTypeErrorIfStrict
}

pub const fn throw_invalid_typed_array_alignment() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowInvalidTypedArrayAlignment
}

pub const fn unwind_and_find_exception_handler() -> MiscRuntimeFunction {
  MiscRuntimeFunction::UnwindAndFindExceptionHandler
}

pub const fn propagate_exception() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PropagateException
}

pub const fn throw_reference_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowReferenceError
}

pub const fn throw_accessed_uninitialized_variable() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowAccessedUninitializedVariable
}

pub const fn new_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NewError
}

pub const fn new_type_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NewTypeError
}

pub const fn new_reference_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NewReferenceError
}

pub const fn throw_invalid_string_length() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowInvalidStringLength
}

pub const fn throw_iterator_result_not_an_object() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowIteratorResultNotAnObject
}

pub const fn throw_throw_method_missing() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowThrowMethodMissing
}

pub const fn throw_symbol_iterator_invalid() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowSymbolIteratorInvalid
}

pub const fn throw_no_access() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowNoAccess
}

pub const fn throw_not_constructor() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowNotConstructor
}

pub const fn throw_target_non_function() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowTargetNonFunction
}

pub const fn stack_guard() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StackGuard
}

pub const fn handle_no_heap_writes_interrupts() -> MiscRuntimeFunction {
  MiscRuntimeFunction::HandleNoHeapWritesInterrupts
}

pub const fn stack_guard_with_gap() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StackGuardWithGap
}

pub const fn bytecode_budget_interrupt_with_stack_check_ignition() -> MiscRuntimeFunction {
  MiscRuntimeFunction::BytecodeBudgetInterruptWithStackCheckIgnition
}

pub const fn bytecode_budget_interrupt_ignition() -> MiscRuntimeFunction {
  MiscRuntimeFunction::BytecodeBudgetInterruptIgnition
}

pub const fn bytecode_budget_interrupt_with_stack_check_sparkplug() -> MiscRuntimeFunction {
  MiscRuntimeFunction::BytecodeBudgetInterruptWithStackCheckSparkplug
}

pub const fn bytecode_budget_interrupt_sparkplug() -> MiscRuntimeFunction {
  MiscRuntimeFunction::BytecodeBudgetInterruptSparkplug
}

pub const fn bytecode_budget_interrupt_maglev() -> MiscRuntimeFunction {
  MiscRuntimeFunction::BytecodeBudgetInterruptMaglev
}

pub const fn bytecode_budget_loop_interrupt_maglev() -> MiscRuntimeFunction {
  MiscRuntimeFunction::BytecodeBudgetLoopInterruptMaglev
}

pub const fn allocate_in_young_generation() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AllocateInYoungGeneration
}

pub const fn allocate_in_old_generation() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AllocateInOldGeneration
}

pub const fn allocate_in_shared_heap() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AllocateInSharedHeap
}

pub const fn allocate_byte_array() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AllocateByteArray
}

pub const fn throw_iterator_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowIteratorError
}

pub const fn throw_spread_arg_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowSpreadArgError
}

pub const fn throw_called_non_callable() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowCalledNonCallable
}

pub const fn throw_constructed_non_constructable() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowConstructedNonConstructable
}

pub const fn throw_pattern_assignment_non_coercible() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowPatternAssignmentNonCoercible
}

pub const fn throw_constructor_returned_non_object() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowConstructorReturnedNonObject
}

pub const fn create_list_from_array_like() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CreateListFromArrayLike
}

pub const fn increment_use_counter() -> MiscRuntimeFunction {
  MiscRuntimeFunction::IncrementUseCounter
}

pub const fn get_and_reset_turbo_profiling_data() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GetAndResetTurboProfilingData
}

pub const fn get_and_reset_runtime_call_stats() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GetAndResetRuntimeCallStats
}

pub const fn ordinary_has_instance() -> MiscRuntimeFunction {
  MiscRuntimeFunction::OrdinaryHasInstance
}

pub const fn r#typeof() -> MiscRuntimeFunction {
  MiscRuntimeFunction::Typeof
}

pub const fn allow_dynamic_function() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AllowDynamicFunction
}

pub const fn create_async_from_sync_iterator() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CreateAsyncFromSyncIterator
}

pub const fn get_template_object() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GetTemplateObject
}

pub const fn report_message_from_microtask() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ReportMessageFromMicrotask
}

pub const fn get_initializer_function() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GetInitializerFunction
}

pub const fn double_to_string_with_radix() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DoubleToStringWithRadix
}

pub const fn shared_value_barrier_slow() -> MiscRuntimeFunction {
  MiscRuntimeFunction::SharedValueBarrierSlow
}

pub const fn notify_context_cell_state_will_change() -> MiscRuntimeFunction {
  MiscRuntimeFunction::NotifyContextCellStateWillChange
}

pub const fn invalidate_string_wrapper_to_primitive_protector() -> MiscRuntimeFunction {
  MiscRuntimeFunction::InvalidateStringWrapperToPrimitiveProtector
}

pub const fn add_lhs_is_string_constant_internalize() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AddLhsIsStringConstantInternalize
}

pub const fn add_rhs_is_string_constant_internalize() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AddRhsIsStringConstantInternalize
}

pub const fn wasm_wasm_to_js_object() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmWasmToJSObject
}

pub const fn wasm_js_to_wasm_object() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmJSToWasmObject
}

pub const fn wasm_memory_grow() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmMemoryGrow
}

pub const fn throw_wasm_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowWasmError
}

pub const fn throw_wasm_stack_overflow() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowWasmStackOverflow
}

pub const fn wasm_throw_js_type_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmThrowJSTypeError
}

pub const fn throw_wasm_jspi_suspend_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowWasmJSPISuspendError
}

pub const fn throw_wasm_fx_suspend_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ThrowWasmFXSuspendError
}

pub const fn wasm_throw_range_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmThrowRangeError
}

pub const fn wasm_throw_data_view_type_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmThrowDataViewTypeError
}

pub const fn wasm_throw_data_view_detached_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmThrowDataViewDetachedError
}

pub const fn wasm_throw_type_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmThrowTypeError
}

pub const fn wasm_throw() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmThrow
}

pub const fn wasm_re_throw() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmReThrow
}

pub const fn wasm_stack_guard() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStackGuard
}

pub const fn wasm_stack_guard_loop() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStackGuardLoop
}

pub const fn wasm_compile_lazy() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmCompileLazy
}

pub const fn wasm_allocate_feedback_vector() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmAllocateFeedbackVector
}

pub const fn wasm_liftoff_deopt_finish() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmLiftoffDeoptFinish
}

pub const fn tier_up_js_to_wasm_wrapper() -> MiscRuntimeFunction {
  MiscRuntimeFunction::TierUpJSToWasmWrapper
}

pub const fn is_wasm_external_function() -> MiscRuntimeFunction {
  MiscRuntimeFunction::IsWasmExternalFunction
}

pub const fn tier_up_wasm_to_js_wrapper() -> MiscRuntimeFunction {
  MiscRuntimeFunction::TierUpWasmToJSWrapper
}

pub const fn wasm_trigger_tier_up() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmTriggerTierUp
}

pub const fn wasm_i32_atomic_wait() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmI32AtomicWait
}

pub const fn wasm_i64_atomic_wait() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmI64AtomicWait
}

pub const fn wasm_managed_object_wait32() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmManagedObjectWait32
}

pub const fn wasm_managed_object_wait64() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmManagedObjectWait64
}

pub const fn wasm_managed_object_wait_ref() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmManagedObjectWaitRef
}

pub const fn wasm_waitqueue_new() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmWaitqueueNew
}

pub const fn wasm_ref_func() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmRefFunc
}

pub const fn wasm_internal_function_create_external() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmInternalFunctionCreateExternal
}

pub const fn wasm_function_table_get() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmFunctionTableGet
}

pub const fn wasm_function_table_set() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmFunctionTableSet
}

pub const fn wasm_table_init() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmTableInit
}

pub const fn wasm_table_copy() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmTableCopy
}

pub const fn wasm_table_grow() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmTableGrow
}

pub const fn wasm_table_fill() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmTableFill
}

pub const fn wasm_debug_break() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmDebugBreak
}

pub const fn wasm_array_copy() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmArrayCopy
}

pub const fn wasm_allocate_descriptor_struct() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmAllocateDescriptorStruct
}

pub const fn wasm_array_new_segment() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmArrayNewSegment
}

pub const fn wasm_array_init_segment() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmArrayInitSegment
}

pub const fn wasm_allocate_suspender() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmAllocateSuspender
}

pub const fn wasm_suspended() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmSuspended
}

pub const fn clear_wasm_suspender_resume_field() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ClearWasmSuspenderResumeField
}

pub const fn wasm_configure_all_prototypes() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmConfigureAllPrototypes
}

pub const fn wasm_configure_all_prototypes_opt() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmConfigureAllPrototypesOpt
}

pub const fn wasm_cast_to_special_primitive_array() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmCastToSpecialPrimitiveArray
}

pub const fn wasm_string_new_wtf8() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringNewWtf8
}

pub const fn wasm_string_new_wtf8_array() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringNewWtf8Array
}

pub const fn wasm_string_new_wtf16() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringNewWtf16
}

pub const fn wasm_string_new_wtf16_array() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringNewWtf16Array
}

pub const fn wasm_substring() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmSubstring
}

pub const fn wasm_substring_shared() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmSubstringShared
}

pub const fn wasm_string_const() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringConst
}

pub const fn wasm_string_new_segment_wtf8() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringNewSegmentWtf8
}

pub const fn wasm_string_measure_utf8() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringMeasureUtf8
}

pub const fn wasm_string_measure_wtf8() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringMeasureWtf8
}

pub const fn wasm_string_encode_wtf8() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringEncodeWtf8
}

pub const fn wasm_string_encode_wtf8_array() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringEncodeWtf8Array
}

pub const fn wasm_string_to_utf8_array() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringToUtf8Array
}

pub const fn wasm_string_encode_wtf16() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringEncodeWtf16
}

pub const fn wasm_string_as_wtf8() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringAsWtf8
}

pub const fn wasm_string_view_wtf8_encode() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringViewWtf8Encode
}

pub const fn wasm_string_view_wtf8_slice() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringViewWtf8Slice
}

pub const fn wasm_trace_begin_execution() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmTraceBeginExecution
}

pub const fn wasm_trace_end_execution() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmTraceEndExecution
}

pub const fn wasm_string_from_code_point() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringFromCodePoint
}

pub const fn wasm_string_hash() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringHash
}

pub const fn wasm_string_add_no_map_check_shared() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmStringAddNoMapCheckShared
}

pub const fn wasm_allocate_continuation() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmAllocateContinuation
}

pub const fn wasm_type_assertion_failed() -> MiscRuntimeFunction {
  MiscRuntimeFunction::WasmTypeAssertionFailed
}

pub const fn tsan_acquire_for_initialization_fence() -> MiscRuntimeFunction {
  MiscRuntimeFunction::TsanAcquireForInitializationFence
}

pub const fn function_get_script_source() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FunctionGetScriptSource
}

pub const fn function_get_script_id() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FunctionGetScriptId
}

pub const fn function_get_source_code() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FunctionGetSourceCode
}

pub const fn function_get_script_source_position() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FunctionGetScriptSourcePosition
}

pub const fn function_is_api_function() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FunctionIsAPIFunction
}

pub const fn call() -> MiscRuntimeFunction {
  MiscRuntimeFunction::Call
}

pub const fn async_function_await() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AsyncFunctionAwait
}

pub const fn async_function_enter() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AsyncFunctionEnter
}

pub const fn async_function_reject() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AsyncFunctionReject
}

pub const fn async_function_resolve() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AsyncFunctionResolve
}

pub const fn create_js_generator_object() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CreateJSGeneratorObject
}

pub const fn generator_close() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GeneratorClose
}

pub const fn generator_get_function() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GeneratorGetFunction
}

pub const fn async_generator_await() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AsyncGeneratorAwait
}

pub const fn async_generator_resolve() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AsyncGeneratorResolve
}

pub const fn async_generator_reject() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AsyncGeneratorReject
}

pub const fn async_generator_yield_with_await() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AsyncGeneratorYieldWithAwait
}

pub const fn generator_get_resume_mode() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GeneratorGetResumeMode
}

pub const fn generator_yield_result() -> MiscRuntimeFunction {
  MiscRuntimeFunction::GeneratorYieldResult
}

pub const fn promise_reject_event_from_stack() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PromiseRejectEventFromStack
}

pub const fn promise_revoke_reject() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PromiseRevokeReject
}

pub const fn enqueue_microtask() -> MiscRuntimeFunction {
  MiscRuntimeFunction::EnqueueMicrotask
}

pub const fn perform_microtask_checkpoint() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PerformMicrotaskCheckpoint
}

pub const fn run_microtask_callback() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RunMicrotaskCallback
}

pub const fn promise_hook_init() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PromiseHookInit
}

pub const fn promise_hook_before() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PromiseHookBefore
}

pub const fn promise_hook_after() -> MiscRuntimeFunction {
  MiscRuntimeFunction::PromiseHookAfter
}

pub const fn reject_promise() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RejectPromise
}

pub const fn resolve_promise() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ResolvePromise
}

pub const fn construct_aggregate_error_helper() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ConstructAggregateErrorHelper
}

pub const fn construct_internal_aggregate_error_helper() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ConstructInternalAggregateErrorHelper
}

pub const fn construct_suppressed_error() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ConstructSuppressedError
}

pub const fn string_split() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StringSplit
}

pub const fn reg_exp_exec() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RegExpExec
}

pub const fn reg_exp_grow_reg_exp_match_info() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RegExpGrowRegExpMatchInfo
}

pub const fn reg_exp_experimental_oneshot_exec() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RegExpExperimentalOneshotExec
}

pub const fn reg_exp_build_indices() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RegExpBuildIndices
}

pub const fn reg_exp_exec_multiple() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RegExpExecMultiple
}

pub const fn string_replace_non_global_reg_exp_with_function() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StringReplaceNonGlobalRegExpWithFunction
}

pub const fn reg_exp_split() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RegExpSplit
}

pub const fn reg_exp_replace_rt() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RegExpReplaceRT
}

pub const fn reg_exp_initialize_and_compile() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RegExpInitializeAndCompile
}

pub const fn reg_exp_string_from_flags() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RegExpStringFromFlags
}

pub const fn reg_exp_match_global_atom() -> MiscRuntimeFunction {
  MiscRuntimeFunction::RegExpMatchGlobalAtom
}

pub const fn create_private_symbol() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CreatePrivateSymbol
}

pub const fn create_private_brand_symbol() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CreatePrivateBrandSymbol
}

pub const fn create_private_name_symbol() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CreatePrivateNameSymbol
}

pub const fn symbol_descriptive_string() -> MiscRuntimeFunction {
  MiscRuntimeFunction::SymbolDescriptiveString
}

pub const fn symbol_is_private() -> MiscRuntimeFunction {
  MiscRuntimeFunction::SymbolIsPrivate
}

pub const fn create_object_literal() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CreateObjectLiteral
}

pub const fn set_prototype_properties() -> MiscRuntimeFunction {
  MiscRuntimeFunction::SetPrototypeProperties
}

pub const fn create_array_literal() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CreateArrayLiteral
}

pub const fn create_reg_exp_literal() -> MiscRuntimeFunction {
  MiscRuntimeFunction::CreateRegExpLiteral
}

pub const fn for_in_enumerate() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ForInEnumerate
}

pub const fn for_in_has_property() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ForInHasProperty
}

pub const fn atomics_num_waiters_for_testing() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AtomicsNumWaitersForTesting
}

pub const fn atomics_num_unresolved_async_promises_for_testing() -> MiscRuntimeFunction {
  MiscRuntimeFunction::AtomicsNumUnresolvedAsyncPromisesForTesting
}

pub const fn set_allow_atomics_wait() -> MiscRuntimeFunction {
  MiscRuntimeFunction::SetAllowAtomicsWait
}

pub const fn format_list() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FormatList
}

pub const fn format_list_to_parts() -> MiscRuntimeFunction {
  MiscRuntimeFunction::FormatListToParts
}

pub const fn string_to_lower_case_intl() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StringToLowerCaseIntl
}

pub const fn string_to_upper_case_intl() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StringToUpperCaseIntl
}

pub const fn string_to_locale_lower_case() -> MiscRuntimeFunction {
  MiscRuntimeFunction::StringToLocaleLowerCase
}

pub const fn shadow_realm_wrapped_function_create() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ShadowRealmWrappedFunctionCreate
}

pub const fn shadow_realm_import_value() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ShadowRealmImportValue
}

pub const fn shadow_realm_throw() -> MiscRuntimeFunction {
  MiscRuntimeFunction::ShadowRealmThrow
}

pub const fn trace_unoptimized_bytecode_entry() -> MiscRuntimeFunction {
  MiscRuntimeFunction::TraceUnoptimizedBytecodeEntry
}

pub const fn trace_unoptimized_bytecode_exit() -> MiscRuntimeFunction {
  MiscRuntimeFunction::TraceUnoptimizedBytecodeExit
}

pub const fn trace_update_feedback() -> MiscRuntimeFunction {
  MiscRuntimeFunction::TraceUpdateFeedback
}

pub const fn dump_execution_frame() -> MiscRuntimeFunction {
  MiscRuntimeFunction::DumpExecutionFrame
}

pub const fn js_finalization_registry_register_weak_cell_with_unregister_token()
-> MiscRuntimeFunction {
  MiscRuntimeFunction::JSFinalizationRegistryRegisterWeakCellWithUnregisterToken
}

pub const fn js_weak_ref_add_to_kept_objects() -> MiscRuntimeFunction {
  MiscRuntimeFunction::JSWeakRefAddToKeptObjects
}
