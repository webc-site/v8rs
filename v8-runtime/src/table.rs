// 由 tool/src/runtime/gen-runtime.js 生成; 来源 v8/src/runtime/runtime.h
// Runtime 函数总表 (FunctionId / 描述符面)

use strum::{Display, EnumIter, EnumString};

/// Runtime 函数全局标识枚举 (来源: runtime.h:958)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[repr(i32)]
pub enum RuntimeFunctionId {
  #[strum(
    serialize = "kArrayIncludes_Slow",
    serialize = "ArrayIncludes_Slow",
    serialize = "ArrayIncludesSlow"
  )]
  ArrayIncludesSlow,
  #[strum(serialize = "kArrayIndexOf", serialize = "ArrayIndexOf")]
  ArrayIndexOf,
  #[strum(serialize = "kArrayIsArray", serialize = "ArrayIsArray")]
  ArrayIsArray,
  #[strum(
    serialize = "kArraySpeciesConstructor",
    serialize = "ArraySpeciesConstructor"
  )]
  ArraySpeciesConstructor,
  #[strum(serialize = "kGrowArrayElements", serialize = "GrowArrayElements")]
  GrowArrayElements,
  #[strum(serialize = "kIsArray", serialize = "IsArray")]
  IsArray,
  #[strum(serialize = "kNewArray", serialize = "NewArray")]
  NewArray,
  #[strum(serialize = "kNormalizeElements", serialize = "NormalizeElements")]
  NormalizeElements,
  #[strum(
    serialize = "kTransitionElementsKind",
    serialize = "TransitionElementsKind"
  )]
  TransitionElementsKind,
  #[strum(
    serialize = "kTransitionElementsKindWithKind",
    serialize = "TransitionElementsKindWithKind"
  )]
  TransitionElementsKindWithKind,
  #[strum(serialize = "kAtomicsLoad64", serialize = "AtomicsLoad64")]
  AtomicsLoad64,
  #[strum(serialize = "kAtomicsStore64", serialize = "AtomicsStore64")]
  AtomicsStore64,
  #[strum(serialize = "kAtomicsAdd", serialize = "AtomicsAdd")]
  AtomicsAdd,
  #[strum(serialize = "kAtomicsAnd", serialize = "AtomicsAnd")]
  AtomicsAnd,
  #[strum(
    serialize = "kAtomicsCompareExchange",
    serialize = "AtomicsCompareExchange"
  )]
  AtomicsCompareExchange,
  #[strum(serialize = "kAtomicsExchange", serialize = "AtomicsExchange")]
  AtomicsExchange,
  #[strum(
    serialize = "kAtomicsNumWaitersForTesting",
    serialize = "AtomicsNumWaitersForTesting"
  )]
  AtomicsNumWaitersForTesting,
  #[strum(
    serialize = "kAtomicsNumUnresolvedAsyncPromisesForTesting",
    serialize = "AtomicsNumUnresolvedAsyncPromisesForTesting"
  )]
  AtomicsNumUnresolvedAsyncPromisesForTesting,
  #[strum(serialize = "kAtomicsOr", serialize = "AtomicsOr")]
  AtomicsOr,
  #[strum(serialize = "kAtomicsSub", serialize = "AtomicsSub")]
  AtomicsSub,
  #[strum(serialize = "kAtomicsXor", serialize = "AtomicsXor")]
  AtomicsXor,
  #[strum(serialize = "kSetAllowAtomicsWait", serialize = "SetAllowAtomicsWait")]
  SetAllowAtomicsWait,
  #[strum(
    serialize = "kAtomicsLoadSharedStructOrArray",
    serialize = "AtomicsLoadSharedStructOrArray"
  )]
  AtomicsLoadSharedStructOrArray,
  #[strum(
    serialize = "kAtomicsStoreSharedStructOrArray",
    serialize = "AtomicsStoreSharedStructOrArray"
  )]
  AtomicsStoreSharedStructOrArray,
  #[strum(
    serialize = "kAtomicsExchangeSharedStructOrArray",
    serialize = "AtomicsExchangeSharedStructOrArray"
  )]
  AtomicsExchangeSharedStructOrArray,
  #[strum(
    serialize = "kAtomicsCompareExchangeSharedStructOrArray",
    serialize = "AtomicsCompareExchangeSharedStructOrArray"
  )]
  AtomicsCompareExchangeSharedStructOrArray,
  #[strum(
    serialize = "kAtomicsSynchronizationPrimitiveNumWaitersForTesting",
    serialize = "AtomicsSynchronizationPrimitiveNumWaitersForTesting"
  )]
  AtomicsSynchronizationPrimitiveNumWaitersForTesting,
  #[strum(
    serialize = "kBigIntCompareToNumber",
    serialize = "BigIntCompareToNumber"
  )]
  BigIntCompareToNumber,
  #[strum(
    serialize = "kBigIntCompareToString",
    serialize = "BigIntCompareToString"
  )]
  BigIntCompareToString,
  #[strum(serialize = "kBigIntEqualToBigInt", serialize = "BigIntEqualToBigInt")]
  BigIntEqualToBigInt,
  #[strum(serialize = "kBigIntEqualToNumber", serialize = "BigIntEqualToNumber")]
  BigIntEqualToNumber,
  #[strum(serialize = "kBigIntEqualToString", serialize = "BigIntEqualToString")]
  BigIntEqualToString,
  #[strum(serialize = "kBigIntExponentiate", serialize = "BigIntExponentiate")]
  BigIntExponentiate,
  #[strum(serialize = "kBigIntMaxBits", serialize = "BigIntMaxBits")]
  BigIntMaxBits,
  #[strum(serialize = "kBigIntToNumber", serialize = "BigIntToNumber")]
  BigIntToNumber,
  #[strum(serialize = "kBigIntUnaryOp", serialize = "BigIntUnaryOp")]
  BigIntUnaryOp,
  #[strum(serialize = "kToBigInt", serialize = "ToBigInt")]
  ToBigInt,
  #[strum(
    serialize = "kToBigIntConvertNumber",
    serialize = "ToBigIntConvertNumber"
  )]
  ToBigIntConvertNumber,
  #[strum(
    serialize = "kThrowConstructorNonCallableError",
    serialize = "ThrowConstructorNonCallableError"
  )]
  ThrowConstructorNonCallableError,
  #[strum(
    serialize = "kThrowNotSuperConstructor",
    serialize = "ThrowNotSuperConstructor"
  )]
  ThrowNotSuperConstructor,
  #[strum(
    serialize = "kThrowStaticPrototypeError",
    serialize = "ThrowStaticPrototypeError"
  )]
  ThrowStaticPrototypeError,
  #[strum(
    serialize = "kThrowSuperAlreadyCalledError",
    serialize = "ThrowSuperAlreadyCalledError"
  )]
  ThrowSuperAlreadyCalledError,
  #[strum(serialize = "kThrowSuperNotCalled", serialize = "ThrowSuperNotCalled")]
  ThrowSuperNotCalled,
  #[strum(
    serialize = "kThrowUnsupportedSuperError",
    serialize = "ThrowUnsupportedSuperError"
  )]
  ThrowUnsupportedSuperError,
  #[strum(serialize = "kDefineClass", serialize = "DefineClass")]
  DefineClass,
  #[strum(serialize = "kLoadFromSuper", serialize = "LoadFromSuper")]
  LoadFromSuper,
  #[strum(serialize = "kLoadKeyedFromSuper", serialize = "LoadKeyedFromSuper")]
  LoadKeyedFromSuper,
  #[strum(serialize = "kStoreKeyedToSuper", serialize = "StoreKeyedToSuper")]
  StoreKeyedToSuper,
  #[strum(serialize = "kStoreToSuper", serialize = "StoreToSuper")]
  StoreToSuper,
  #[strum(serialize = "kMapGrow", serialize = "MapGrow")]
  MapGrow,
  #[strum(serialize = "kMapShrink", serialize = "MapShrink")]
  MapShrink,
  #[strum(serialize = "kOrderedHashSetGrow", serialize = "OrderedHashSetGrow")]
  OrderedHashSetGrow,
  #[strum(serialize = "kSetGrow", serialize = "SetGrow")]
  SetGrow,
  #[strum(serialize = "kSetShrink", serialize = "SetShrink")]
  SetShrink,
  #[strum(
    serialize = "kOrderedHashSetShrink",
    serialize = "OrderedHashSetShrink"
  )]
  OrderedHashSetShrink,
  #[strum(serialize = "kTheHole", serialize = "TheHole")]
  TheHole,
  #[strum(
    serialize = "kWeakCollectionDelete",
    serialize = "WeakCollectionDelete"
  )]
  WeakCollectionDelete,
  #[strum(serialize = "kWeakCollectionSet", serialize = "WeakCollectionSet")]
  WeakCollectionSet,
  #[strum(serialize = "kOrderedHashMapGrow", serialize = "OrderedHashMapGrow")]
  OrderedHashMapGrow,
  #[strum(serialize = "kPrintDumpedFrame", serialize = "PrintDumpedFrame")]
  PrintDumpedFrame,
  #[strum(serialize = "kCompileOptimizedOSR", serialize = "CompileOptimizedOSR")]
  CompileOptimizedOSR,
  #[strum(
    serialize = "kCompileOptimizedOSRFromMaglev",
    serialize = "CompileOptimizedOSRFromMaglev"
  )]
  CompileOptimizedOSRFromMaglev,
  #[strum(
    serialize = "kCompileOptimizedOSRFromMaglevInlined",
    serialize = "CompileOptimizedOSRFromMaglevInlined"
  )]
  CompileOptimizedOSRFromMaglevInlined,
  #[strum(
    serialize = "kLogOrTraceOptimizedOSREntry",
    serialize = "LogOrTraceOptimizedOSREntry"
  )]
  LogOrTraceOptimizedOSREntry,
  #[strum(serialize = "kCompileLazy", serialize = "CompileLazy")]
  CompileLazy,
  #[strum(serialize = "kCompileBaseline", serialize = "CompileBaseline")]
  CompileBaseline,
  #[strum(serialize = "kInstallBaselineCode", serialize = "InstallBaselineCode")]
  InstallBaselineCode,
  #[strum(serialize = "kInstallSFICode", serialize = "InstallSFICode")]
  InstallSFICode,
  #[strum(serialize = "kNotifyDeoptimized", serialize = "NotifyDeoptimized")]
  NotifyDeoptimized,
  #[strum(serialize = "kObserveNode", serialize = "ObserveNode")]
  ObserveNode,
  #[strum(
    serialize = "kResolvePossiblyDirectEval",
    serialize = "ResolvePossiblyDirectEval"
  )]
  ResolvePossiblyDirectEval,
  #[strum(serialize = "kVerifyType", serialize = "VerifyType")]
  VerifyType,
  #[strum(
    serialize = "kCheckTurboshaftTypeOf",
    serialize = "CheckTurboshaftTypeOf"
  )]
  CheckTurboshaftTypeOf,
  #[strum(
    serialize = "kPatchCompareOpBaselineCode",
    serialize = "PatchCompareOpBaselineCode"
  )]
  PatchCompareOpBaselineCode,
  #[strum(
    serialize = "kPatchCompareOpBaselineCodeAndThrow",
    serialize = "PatchCompareOpBaselineCodeAndThrow"
  )]
  PatchCompareOpBaselineCodeAndThrow,
  #[strum(
    serialize = "kPatchBinopBaselineCode",
    serialize = "PatchBinopBaselineCode"
  )]
  PatchBinopBaselineCode,
  #[strum(
    serialize = "kPatchBinopBaselineCodeAndThrow",
    serialize = "PatchBinopBaselineCodeAndThrow"
  )]
  PatchBinopBaselineCodeAndThrow,
  #[strum(
    serialize = "kPatchUnaryOpBaselineCode",
    serialize = "PatchUnaryOpBaselineCode"
  )]
  PatchUnaryOpBaselineCode,
  #[strum(
    serialize = "kPatchUnaryOpBaselineCodeAndThrow",
    serialize = "PatchUnaryOpBaselineCodeAndThrow"
  )]
  PatchUnaryOpBaselineCodeAndThrow,
  #[strum(
    serialize = "kFunctionLogNextExecution",
    serialize = "FunctionLogNextExecution"
  )]
  FunctionLogNextExecution,
  #[strum(serialize = "kOptimizeMaglevEager", serialize = "OptimizeMaglevEager")]
  OptimizeMaglevEager,
  #[strum(
    serialize = "kStartMaglevOptimizeJob",
    serialize = "StartMaglevOptimizeJob"
  )]
  StartMaglevOptimizeJob,
  #[strum(
    serialize = "kOptimizeTurbofanEager",
    serialize = "OptimizeTurbofanEager"
  )]
  OptimizeTurbofanEager,
  #[strum(
    serialize = "kStartTurbofanOptimizeJob",
    serialize = "StartTurbofanOptimizeJob"
  )]
  StartTurbofanOptimizeJob,
  #[strum(
    serialize = "kMarkLazyDeoptimizedOrFlushed",
    serialize = "MarkLazyDeoptimizedOrFlushed"
  )]
  MarkLazyDeoptimizedOrFlushed,
  #[strum(serialize = "kDateCurrentTime", serialize = "DateCurrentTime")]
  DateCurrentTime,
  #[strum(serialize = "kClearStepping", serialize = "ClearStepping")]
  ClearStepping,
  #[strum(serialize = "kCollectGarbage", serialize = "CollectGarbage")]
  CollectGarbage,
  #[strum(
    serialize = "kDebugAsyncFunctionSuspended",
    serialize = "DebugAsyncFunctionSuspended"
  )]
  DebugAsyncFunctionSuspended,
  #[strum(serialize = "kDebugBreakAtEntry", serialize = "DebugBreakAtEntry")]
  DebugBreakAtEntry,
  #[strum(
    serialize = "kDebugCollectCoverage",
    serialize = "DebugCollectCoverage"
  )]
  DebugCollectCoverage,
  #[strum(
    serialize = "kDebugGetLoadedScriptIds",
    serialize = "DebugGetLoadedScriptIds"
  )]
  DebugGetLoadedScriptIds,
  #[strum(serialize = "kDebugOnFunctionCall", serialize = "DebugOnFunctionCall")]
  DebugOnFunctionCall,
  #[strum(
    serialize = "kDebugPrepareStepInSuspendedGenerator",
    serialize = "DebugPrepareStepInSuspendedGenerator"
  )]
  DebugPrepareStepInSuspendedGenerator,
  #[strum(serialize = "kDebugPromiseThen", serialize = "DebugPromiseThen")]
  DebugPromiseThen,
  #[strum(
    serialize = "kDebugToggleBlockCoverage",
    serialize = "DebugToggleBlockCoverage"
  )]
  DebugToggleBlockCoverage,
  #[strum(
    serialize = "kDebugTogglePreciseCoverage",
    serialize = "DebugTogglePreciseCoverage"
  )]
  DebugTogglePreciseCoverage,
  #[strum(
    serialize = "kFunctionGetInferredName",
    serialize = "FunctionGetInferredName"
  )]
  FunctionGetInferredName,
  #[strum(serialize = "kGetBreakLocations", serialize = "GetBreakLocations")]
  GetBreakLocations,
  #[strum(
    serialize = "kGetGeneratorScopeCount",
    serialize = "GetGeneratorScopeCount"
  )]
  GetGeneratorScopeCount,
  #[strum(
    serialize = "kGetGeneratorScopeDetails",
    serialize = "GetGeneratorScopeDetails"
  )]
  GetGeneratorScopeDetails,
  #[strum(
    serialize = "kHandleDebuggerStatement",
    serialize = "HandleDebuggerStatement"
  )]
  HandleDebuggerStatement,
  #[strum(serialize = "kIsBreakOnException", serialize = "IsBreakOnException")]
  IsBreakOnException,
  #[strum(serialize = "kIterableForEach", serialize = "IterableForEach")]
  IterableForEach,
  #[strum(
    serialize = "kProfileCreateSnapshotDataBlob",
    serialize = "ProfileCreateSnapshotDataBlob"
  )]
  ProfileCreateSnapshotDataBlob,
  #[strum(serialize = "kScheduleBreak", serialize = "ScheduleBreak")]
  ScheduleBreak,
  #[strum(
    serialize = "kScriptLocationFromLine2",
    serialize = "ScriptLocationFromLine2"
  )]
  ScriptLocationFromLine2,
  #[strum(
    serialize = "kSetGeneratorScopeVariableValue",
    serialize = "SetGeneratorScopeVariableValue"
  )]
  SetGeneratorScopeVariableValue,
  #[strum(serialize = "kIncBlockCounter", serialize = "IncBlockCounter")]
  IncBlockCounter,
  #[strum(serialize = "kForInEnumerate", serialize = "ForInEnumerate")]
  ForInEnumerate,
  #[strum(serialize = "kForInHasProperty", serialize = "ForInHasProperty")]
  ForInHasProperty,
  #[strum(
    serialize = "kTraceUnoptimizedBytecodeEntry",
    serialize = "TraceUnoptimizedBytecodeEntry"
  )]
  TraceUnoptimizedBytecodeEntry,
  #[strum(
    serialize = "kTraceUnoptimizedBytecodeExit",
    serialize = "TraceUnoptimizedBytecodeExit"
  )]
  TraceUnoptimizedBytecodeExit,
  #[strum(serialize = "kDumpExecutionFrame", serialize = "DumpExecutionFrame")]
  DumpExecutionFrame,
  #[strum(serialize = "kTraceUpdateFeedback", serialize = "TraceUpdateFeedback")]
  TraceUpdateFeedback,
  #[strum(serialize = "kCall", serialize = "Call")]
  Call,
  #[strum(
    serialize = "kFunctionGetScriptSource",
    serialize = "FunctionGetScriptSource"
  )]
  FunctionGetScriptSource,
  #[strum(serialize = "kFunctionGetScriptId", serialize = "FunctionGetScriptId")]
  FunctionGetScriptId,
  #[strum(
    serialize = "kFunctionGetScriptSourcePosition",
    serialize = "FunctionGetScriptSourcePosition"
  )]
  FunctionGetScriptSourcePosition,
  #[strum(
    serialize = "kFunctionGetSourceCode",
    serialize = "FunctionGetSourceCode"
  )]
  FunctionGetSourceCode,
  #[strum(
    serialize = "kFunctionIsAPIFunction",
    serialize = "FunctionIsAPIFunction"
  )]
  FunctionIsAPIFunction,
  #[strum(serialize = "kAsyncFunctionAwait", serialize = "AsyncFunctionAwait")]
  AsyncFunctionAwait,
  #[strum(serialize = "kAsyncFunctionEnter", serialize = "AsyncFunctionEnter")]
  AsyncFunctionEnter,
  #[strum(serialize = "kAsyncFunctionReject", serialize = "AsyncFunctionReject")]
  AsyncFunctionReject,
  #[strum(
    serialize = "kAsyncFunctionResolve",
    serialize = "AsyncFunctionResolve"
  )]
  AsyncFunctionResolve,
  #[strum(serialize = "kAsyncGeneratorAwait", serialize = "AsyncGeneratorAwait")]
  AsyncGeneratorAwait,
  #[strum(
    serialize = "kAsyncGeneratorReject",
    serialize = "AsyncGeneratorReject"
  )]
  AsyncGeneratorReject,
  #[strum(
    serialize = "kAsyncGeneratorResolve",
    serialize = "AsyncGeneratorResolve"
  )]
  AsyncGeneratorResolve,
  #[strum(
    serialize = "kAsyncGeneratorYieldWithAwait",
    serialize = "AsyncGeneratorYieldWithAwait"
  )]
  AsyncGeneratorYieldWithAwait,
  #[strum(
    serialize = "kCreateJSGeneratorObject",
    serialize = "CreateJSGeneratorObject"
  )]
  CreateJSGeneratorObject,
  #[strum(serialize = "kGeneratorClose", serialize = "GeneratorClose")]
  GeneratorClose,
  #[strum(
    serialize = "kGeneratorGetFunction",
    serialize = "GeneratorGetFunction"
  )]
  GeneratorGetFunction,
  #[strum(
    serialize = "kGeneratorGetResumeMode",
    serialize = "GeneratorGetResumeMode"
  )]
  GeneratorGetResumeMode,
  #[strum(serialize = "kFormatList", serialize = "FormatList")]
  FormatList,
  #[strum(serialize = "kFormatListToParts", serialize = "FormatListToParts")]
  FormatListToParts,
  #[strum(
    serialize = "kStringToLowerCaseIntl",
    serialize = "StringToLowerCaseIntl"
  )]
  StringToLowerCaseIntl,
  #[strum(
    serialize = "kStringToLocaleLowerCase",
    serialize = "StringToLocaleLowerCase"
  )]
  StringToLocaleLowerCase,
  #[strum(
    serialize = "kStringToUpperCaseIntl",
    serialize = "StringToUpperCaseIntl"
  )]
  StringToUpperCaseIntl,
  #[strum(serialize = "kThrow", serialize = "Throw")]
  Throw,
  #[strum(
    serialize = "kThrowTargetNonFunction",
    serialize = "ThrowTargetNonFunction"
  )]
  ThrowTargetNonFunction,
  #[strum(
    serialize = "kThrowCalledNonCallable",
    serialize = "ThrowCalledNonCallable"
  )]
  ThrowCalledNonCallable,
  #[strum(
    serialize = "kThrowConstructedNonConstructable",
    serialize = "ThrowConstructedNonConstructable"
  )]
  ThrowConstructedNonConstructable,
  #[strum(
    serialize = "kThrowConstructorReturnedNonObject",
    serialize = "ThrowConstructorReturnedNonObject"
  )]
  ThrowConstructorReturnedNonObject,
  #[strum(
    serialize = "kThrowInvalidStringLength",
    serialize = "ThrowInvalidStringLength"
  )]
  ThrowInvalidStringLength,
  #[strum(
    serialize = "kThrowInvalidTypedArrayAlignment",
    serialize = "ThrowInvalidTypedArrayAlignment"
  )]
  ThrowInvalidTypedArrayAlignment,
  #[strum(serialize = "kThrowIteratorError", serialize = "ThrowIteratorError")]
  ThrowIteratorError,
  #[strum(serialize = "kThrowSpreadArgError", serialize = "ThrowSpreadArgError")]
  ThrowSpreadArgError,
  #[strum(
    serialize = "kThrowIteratorResultNotAnObject",
    serialize = "ThrowIteratorResultNotAnObject"
  )]
  ThrowIteratorResultNotAnObject,
  #[strum(serialize = "kThrowNoAccess", serialize = "ThrowNoAccess")]
  ThrowNoAccess,
  #[strum(serialize = "kThrowNotConstructor", serialize = "ThrowNotConstructor")]
  ThrowNotConstructor,
  #[strum(
    serialize = "kThrowPatternAssignmentNonCoercible",
    serialize = "ThrowPatternAssignmentNonCoercible"
  )]
  ThrowPatternAssignmentNonCoercible,
  #[strum(serialize = "kThrowRangeError", serialize = "ThrowRangeError")]
  ThrowRangeError,
  #[strum(serialize = "kThrowReferenceError", serialize = "ThrowReferenceError")]
  ThrowReferenceError,
  #[strum(
    serialize = "kThrowAccessedUninitializedVariable",
    serialize = "ThrowAccessedUninitializedVariable"
  )]
  ThrowAccessedUninitializedVariable,
  #[strum(serialize = "kVarargStackOverflow", serialize = "VarargStackOverflow")]
  VarargStackOverflow,
  #[strum(serialize = "kThrowStackOverflow", serialize = "ThrowStackOverflow")]
  ThrowStackOverflow,
  #[strum(
    serialize = "kThrowSymbolAsyncIteratorInvalid",
    serialize = "ThrowSymbolAsyncIteratorInvalid"
  )]
  ThrowSymbolAsyncIteratorInvalid,
  #[strum(
    serialize = "kThrowSymbolIteratorInvalid",
    serialize = "ThrowSymbolIteratorInvalid"
  )]
  ThrowSymbolIteratorInvalid,
  #[strum(
    serialize = "kThrowThrowMethodMissing",
    serialize = "ThrowThrowMethodMissing"
  )]
  ThrowThrowMethodMissing,
  #[strum(serialize = "kThrowTypeError", serialize = "ThrowTypeError")]
  ThrowTypeError,
  #[strum(
    serialize = "kThrowTypeErrorIfStrict",
    serialize = "ThrowTypeErrorIfStrict"
  )]
  ThrowTypeErrorIfStrict,
  #[strum(serialize = "kReThrow", serialize = "ReThrow")]
  ReThrow,
  #[strum(serialize = "kReThrowWithMessage", serialize = "ReThrowWithMessage")]
  ReThrowWithMessage,
  #[strum(serialize = "kAccessCheck", serialize = "AccessCheck")]
  AccessCheck,
  #[strum(serialize = "kAllocateByteArray", serialize = "AllocateByteArray")]
  AllocateByteArray,
  #[strum(
    serialize = "kAllocateInYoungGeneration",
    serialize = "AllocateInYoungGeneration"
  )]
  AllocateInYoungGeneration,
  #[strum(
    serialize = "kAllocateInOldGeneration",
    serialize = "AllocateInOldGeneration"
  )]
  AllocateInOldGeneration,
  #[strum(
    serialize = "kAllocateInSharedHeap",
    serialize = "AllocateInSharedHeap"
  )]
  AllocateInSharedHeap,
  #[strum(
    serialize = "kAllowDynamicFunction",
    serialize = "AllowDynamicFunction"
  )]
  AllowDynamicFunction,
  #[strum(
    serialize = "kCreateAsyncFromSyncIterator",
    serialize = "CreateAsyncFromSyncIterator"
  )]
  CreateAsyncFromSyncIterator,
  #[strum(
    serialize = "kCreateListFromArrayLike",
    serialize = "CreateListFromArrayLike"
  )]
  CreateListFromArrayLike,
  #[strum(
    serialize = "kDoubleToStringWithRadix",
    serialize = "DoubleToStringWithRadix"
  )]
  DoubleToStringWithRadix,
  #[strum(
    serialize = "kFatalProcessOutOfMemoryInAllocateRaw",
    serialize = "FatalProcessOutOfMemoryInAllocateRaw"
  )]
  FatalProcessOutOfMemoryInAllocateRaw,
  #[strum(
    serialize = "kFatalProcessOutOfMemoryInvalidArrayLength",
    serialize = "FatalProcessOutOfMemoryInvalidArrayLength"
  )]
  FatalProcessOutOfMemoryInvalidArrayLength,
  #[strum(serialize = "kFatalInvalidSize", serialize = "FatalInvalidSize")]
  FatalInvalidSize,
  #[strum(
    serialize = "kGetAndResetRuntimeCallStats",
    serialize = "GetAndResetRuntimeCallStats"
  )]
  GetAndResetRuntimeCallStats,
  #[strum(
    serialize = "kGetAndResetTurboProfilingData",
    serialize = "GetAndResetTurboProfilingData"
  )]
  GetAndResetTurboProfilingData,
  #[strum(serialize = "kGetTemplateObject", serialize = "GetTemplateObject")]
  GetTemplateObject,
  #[strum(serialize = "kIncrementUseCounter", serialize = "IncrementUseCounter")]
  IncrementUseCounter,
  #[strum(
    serialize = "kInvalidateStringWrapperToPrimitiveProtector",
    serialize = "InvalidateStringWrapperToPrimitiveProtector"
  )]
  InvalidateStringWrapperToPrimitiveProtector,
  #[strum(
    serialize = "kBytecodeBudgetInterrupt_Ignition",
    serialize = "BytecodeBudgetInterrupt_Ignition",
    serialize = "BytecodeBudgetInterruptIgnition"
  )]
  BytecodeBudgetInterruptIgnition,
  #[strum(
    serialize = "kBytecodeBudgetInterruptWithStackCheck_Ignition",
    serialize = "BytecodeBudgetInterruptWithStackCheck_Ignition",
    serialize = "BytecodeBudgetInterruptWithStackCheckIgnition"
  )]
  BytecodeBudgetInterruptWithStackCheckIgnition,
  #[strum(
    serialize = "kBytecodeBudgetInterrupt_Sparkplug",
    serialize = "BytecodeBudgetInterrupt_Sparkplug",
    serialize = "BytecodeBudgetInterruptSparkplug"
  )]
  BytecodeBudgetInterruptSparkplug,
  #[strum(
    serialize = "kBytecodeBudgetInterruptWithStackCheck_Sparkplug",
    serialize = "BytecodeBudgetInterruptWithStackCheck_Sparkplug",
    serialize = "BytecodeBudgetInterruptWithStackCheckSparkplug"
  )]
  BytecodeBudgetInterruptWithStackCheckSparkplug,
  #[strum(
    serialize = "kBytecodeBudgetInterrupt_Maglev",
    serialize = "BytecodeBudgetInterrupt_Maglev",
    serialize = "BytecodeBudgetInterruptMaglev"
  )]
  BytecodeBudgetInterruptMaglev,
  #[strum(
    serialize = "kBytecodeBudgetLoopInterrupt_Maglev",
    serialize = "BytecodeBudgetLoopInterrupt_Maglev",
    serialize = "BytecodeBudgetLoopInterruptMaglev"
  )]
  BytecodeBudgetLoopInterruptMaglev,
  #[strum(
    serialize = "kNotifyContextCellStateWillChange",
    serialize = "NotifyContextCellStateWillChange"
  )]
  NotifyContextCellStateWillChange,
  #[strum(serialize = "kNewError", serialize = "NewError")]
  NewError,
  #[strum(serialize = "kNewReferenceError", serialize = "NewReferenceError")]
  NewReferenceError,
  #[strum(serialize = "kNewTypeError", serialize = "NewTypeError")]
  NewTypeError,
  #[strum(serialize = "kOrdinaryHasInstance", serialize = "OrdinaryHasInstance")]
  OrdinaryHasInstance,
  #[strum(serialize = "kPropagateException", serialize = "PropagateException")]
  PropagateException,
  #[strum(
    serialize = "kReportMessageFromMicrotask",
    serialize = "ReportMessageFromMicrotask"
  )]
  ReportMessageFromMicrotask,
  #[strum(
    serialize = "kRunMicrotaskCallback",
    serialize = "RunMicrotaskCallback"
  )]
  RunMicrotaskCallback,
  #[strum(
    serialize = "kPerformMicrotaskCheckpoint",
    serialize = "PerformMicrotaskCheckpoint"
  )]
  PerformMicrotaskCheckpoint,
  #[strum(
    serialize = "kSharedValueBarrierSlow",
    serialize = "SharedValueBarrierSlow"
  )]
  SharedValueBarrierSlow,
  #[strum(serialize = "kStackGuard", serialize = "StackGuard")]
  StackGuard,
  #[strum(
    serialize = "kHandleNoHeapWritesInterrupts",
    serialize = "HandleNoHeapWritesInterrupts"
  )]
  HandleNoHeapWritesInterrupts,
  #[strum(serialize = "kStackGuardWithGap", serialize = "StackGuardWithGap")]
  StackGuardWithGap,
  #[strum(serialize = "kTerminateExecution", serialize = "TerminateExecution")]
  TerminateExecution,
  #[strum(serialize = "kTypeof", serialize = "Typeof")]
  Typeof,
  #[strum(
    serialize = "kUnwindAndFindExceptionHandler",
    serialize = "UnwindAndFindExceptionHandler"
  )]
  UnwindAndFindExceptionHandler,
  #[strum(
    serialize = "kAddLhsIsStringConstantInternalize",
    serialize = "AddLhsIsStringConstantInternalize"
  )]
  AddLhsIsStringConstantInternalize,
  #[strum(
    serialize = "kAddRhsIsStringConstantInternalize",
    serialize = "AddRhsIsStringConstantInternalize"
  )]
  AddRhsIsStringConstantInternalize,
  #[strum(serialize = "kCreateArrayLiteral", serialize = "CreateArrayLiteral")]
  CreateArrayLiteral,
  #[strum(serialize = "kCreateObjectLiteral", serialize = "CreateObjectLiteral")]
  CreateObjectLiteral,
  #[strum(
    serialize = "kSetPrototypeProperties",
    serialize = "SetPrototypeProperties"
  )]
  SetPrototypeProperties,
  #[strum(serialize = "kCreateRegExpLiteral", serialize = "CreateRegExpLiteral")]
  CreateRegExpLiteral,
  #[strum(serialize = "kDynamicImportCall", serialize = "DynamicImportCall")]
  DynamicImportCall,
  #[strum(serialize = "kGetImportMetaObject", serialize = "GetImportMetaObject")]
  GetImportMetaObject,
  #[strum(serialize = "kGetModuleNamespace", serialize = "GetModuleNamespace")]
  GetModuleNamespace,
  #[strum(
    serialize = "kGetModuleNamespaceExport",
    serialize = "GetModuleNamespaceExport"
  )]
  GetModuleNamespaceExport,
  #[strum(
    serialize = "kArrayBufferMaxByteLength",
    serialize = "ArrayBufferMaxByteLength"
  )]
  ArrayBufferMaxByteLength,
  #[strum(serialize = "kIsSmi", serialize = "IsSmi")]
  IsSmi,
  #[strum(serialize = "kMaxSmi", serialize = "MaxSmi")]
  MaxSmi,
  #[strum(serialize = "kNumberToStringSlow", serialize = "NumberToStringSlow")]
  NumberToStringSlow,
  #[strum(serialize = "kFloat64ToStringSlow", serialize = "Float64ToStringSlow")]
  Float64ToStringSlow,
  #[strum(serialize = "kStringParseFloat", serialize = "StringParseFloat")]
  StringParseFloat,
  #[strum(serialize = "kStringParseInt", serialize = "StringParseInt")]
  StringParseInt,
  #[strum(serialize = "kStringToNumber", serialize = "StringToNumber")]
  StringToNumber,
  #[strum(
    serialize = "kAddAsyncDisposableValue",
    serialize = "AddAsyncDisposableValue"
  )]
  AddAsyncDisposableValue,
  #[strum(
    serialize = "kAddDictionaryProperty",
    serialize = "AddDictionaryProperty"
  )]
  AddDictionaryProperty,
  #[strum(serialize = "kAddDisposableValue", serialize = "AddDisposableValue")]
  AddDisposableValue,
  #[strum(serialize = "kAddPrivateBrand", serialize = "AddPrivateBrand")]
  AddPrivateBrand,
  #[strum(serialize = "kAllocateHeapNumber", serialize = "AllocateHeapNumber")]
  AllocateHeapNumber,
  #[strum(
    serialize = "kCompleteInobjectSlackTrackingForMap",
    serialize = "CompleteInobjectSlackTrackingForMap"
  )]
  CompleteInobjectSlackTrackingForMap,
  #[strum(serialize = "kCopyDataProperties", serialize = "CopyDataProperties")]
  CopyDataProperties,
  #[strum(
    serialize = "kCopyDataPropertiesWithExcludedPropertiesOnStack",
    serialize = "CopyDataPropertiesWithExcludedPropertiesOnStack"
  )]
  CopyDataPropertiesWithExcludedPropertiesOnStack,
  #[strum(serialize = "kCreateDataProperty", serialize = "CreateDataProperty")]
  CreateDataProperty,
  #[strum(
    serialize = "kCreateIterResultObject",
    serialize = "CreateIterResultObject"
  )]
  CreateIterResultObject,
  #[strum(
    serialize = "kIterableToListWithInterceptor",
    serialize = "IterableToListWithInterceptor"
  )]
  IterableToListWithInterceptor,
  #[strum(
    serialize = "kCheckFastIterableToListPrototype",
    serialize = "CheckFastIterableToListPrototype"
  )]
  CheckFastIterableToListPrototype,
  #[strum(
    serialize = "kCreatePrivateAccessors",
    serialize = "CreatePrivateAccessors"
  )]
  CreatePrivateAccessors,
  #[strum(
    serialize = "kDefineAccessorPropertyUnchecked",
    serialize = "DefineAccessorPropertyUnchecked"
  )]
  DefineAccessorPropertyUnchecked,
  #[strum(
    serialize = "kDefineKeyedOwnPropertyInLiteral",
    serialize = "DefineKeyedOwnPropertyInLiteral"
  )]
  DefineKeyedOwnPropertyInLiteral,
  #[strum(
    serialize = "kDefineGetterPropertyUnchecked",
    serialize = "DefineGetterPropertyUnchecked"
  )]
  DefineGetterPropertyUnchecked,
  #[strum(
    serialize = "kDefineSetterPropertyUnchecked",
    serialize = "DefineSetterPropertyUnchecked"
  )]
  DefineSetterPropertyUnchecked,
  #[strum(serialize = "kDeleteProperty", serialize = "DeleteProperty")]
  DeleteProperty,
  #[strum(
    serialize = "kDisposeDisposableStack",
    serialize = "DisposeDisposableStack"
  )]
  DisposeDisposableStack,
  #[strum(
    serialize = "kGeneratorYieldResult",
    serialize = "GeneratorYieldResult"
  )]
  GeneratorYieldResult,
  #[strum(serialize = "kGetDerivedMap", serialize = "GetDerivedMap")]
  GetDerivedMap,
  #[strum(serialize = "kGetFunctionName", serialize = "GetFunctionName")]
  GetFunctionName,
  #[strum(
    serialize = "kGetOwnPropertyDescriptorObject",
    serialize = "GetOwnPropertyDescriptorObject"
  )]
  GetOwnPropertyDescriptorObject,
  #[strum(serialize = "kGetOwnPropertyKeys", serialize = "GetOwnPropertyKeys")]
  GetOwnPropertyKeys,
  #[strum(serialize = "kGetPrivateMember", serialize = "GetPrivateMember")]
  GetPrivateMember,
  #[strum(serialize = "kGetProperty", serialize = "GetProperty")]
  GetProperty,
  #[strum(
    serialize = "kHandleExceptionsInDisposeDisposableStack",
    serialize = "HandleExceptionsInDisposeDisposableStack"
  )]
  HandleExceptionsInDisposeDisposableStack,
  #[strum(
    serialize = "kHasFastPackedElements",
    serialize = "HasFastPackedElements"
  )]
  HasFastPackedElements,
  #[strum(serialize = "kHasInPrototypeChain", serialize = "HasInPrototypeChain")]
  HasInPrototypeChain,
  #[strum(serialize = "kHasProperty", serialize = "HasProperty")]
  HasProperty,
  #[strum(
    serialize = "kInitializeDisposableStack",
    serialize = "InitializeDisposableStack"
  )]
  InitializeDisposableStack,
  #[strum(
    serialize = "kInternalSetPrototype",
    serialize = "InternalSetPrototype"
  )]
  InternalSetPrototype,
  #[strum(serialize = "kIsJSReceiver", serialize = "IsJSReceiver")]
  IsJSReceiver,
  #[strum(
    serialize = "kJSReceiverPreventExtensionsDontThrow",
    serialize = "JSReceiverPreventExtensionsDontThrow"
  )]
  JSReceiverPreventExtensionsDontThrow,
  #[strum(
    serialize = "kJSReceiverPreventExtensionsThrow",
    serialize = "JSReceiverPreventExtensionsThrow"
  )]
  JSReceiverPreventExtensionsThrow,
  #[strum(
    serialize = "kJSReceiverGetPrototypeOf",
    serialize = "JSReceiverGetPrototypeOf"
  )]
  JSReceiverGetPrototypeOf,
  #[strum(
    serialize = "kJSReceiverSetPrototypeOfDontThrow",
    serialize = "JSReceiverSetPrototypeOfDontThrow"
  )]
  JSReceiverSetPrototypeOfDontThrow,
  #[strum(
    serialize = "kJSReceiverSetPrototypeOfThrow",
    serialize = "JSReceiverSetPrototypeOfThrow"
  )]
  JSReceiverSetPrototypeOfThrow,
  #[strum(serialize = "kLoadPrivateGetter", serialize = "LoadPrivateGetter")]
  LoadPrivateGetter,
  #[strum(serialize = "kLoadPrivateSetter", serialize = "LoadPrivateSetter")]
  LoadPrivateSetter,
  #[strum(serialize = "kNewObject", serialize = "NewObject")]
  NewObject,
  #[strum(serialize = "kObjectCreate", serialize = "ObjectCreate")]
  ObjectCreate,
  #[strum(serialize = "kObjectEntries", serialize = "ObjectEntries")]
  ObjectEntries,
  #[strum(
    serialize = "kObjectEntriesSkipFastPath",
    serialize = "ObjectEntriesSkipFastPath"
  )]
  ObjectEntriesSkipFastPath,
  #[strum(
    serialize = "kObjectGetOwnPropertyNames",
    serialize = "ObjectGetOwnPropertyNames"
  )]
  ObjectGetOwnPropertyNames,
  #[strum(
    serialize = "kObjectGetOwnPropertyNamesTryFast",
    serialize = "ObjectGetOwnPropertyNamesTryFast"
  )]
  ObjectGetOwnPropertyNamesTryFast,
  #[strum(
    serialize = "kObjectHasOwnProperty",
    serialize = "ObjectHasOwnProperty"
  )]
  ObjectHasOwnProperty,
  #[strum(
    serialize = "kHasOwnConstDataProperty",
    serialize = "HasOwnConstDataProperty"
  )]
  HasOwnConstDataProperty,
  #[strum(serialize = "kObjectIsExtensible", serialize = "ObjectIsExtensible")]
  ObjectIsExtensible,
  #[strum(serialize = "kObjectKeys", serialize = "ObjectKeys")]
  ObjectKeys,
  #[strum(serialize = "kObjectValues", serialize = "ObjectValues")]
  ObjectValues,
  #[strum(
    serialize = "kObjectValuesSkipFastPath",
    serialize = "ObjectValuesSkipFastPath"
  )]
  ObjectValuesSkipFastPath,
  #[strum(
    serialize = "kOptimizeObjectForAddingMultipleProperties",
    serialize = "OptimizeObjectForAddingMultipleProperties"
  )]
  OptimizeObjectForAddingMultipleProperties,
  #[strum(serialize = "kSetDataProperties", serialize = "SetDataProperties")]
  SetDataProperties,
  #[strum(serialize = "kSetFunctionName", serialize = "SetFunctionName")]
  SetFunctionName,
  #[strum(serialize = "kSetKeyedProperty", serialize = "SetKeyedProperty")]
  SetKeyedProperty,
  #[strum(
    serialize = "kDefineObjectOwnProperty",
    serialize = "DefineObjectOwnProperty"
  )]
  DefineObjectOwnProperty,
  #[strum(serialize = "kSetNamedProperty", serialize = "SetNamedProperty")]
  SetNamedProperty,
  #[strum(
    serialize = "kSetOwnPropertyIgnoreAttributes",
    serialize = "SetOwnPropertyIgnoreAttributes"
  )]
  SetOwnPropertyIgnoreAttributes,
  #[strum(
    serialize = "kShrinkNameDictionary",
    serialize = "ShrinkNameDictionary"
  )]
  ShrinkNameDictionary,
  #[strum(
    serialize = "kShrinkSwissNameDictionary",
    serialize = "ShrinkSwissNameDictionary"
  )]
  ShrinkSwissNameDictionary,
  #[strum(serialize = "kToFastProperties", serialize = "ToFastProperties")]
  ToFastProperties,
  #[strum(serialize = "kToLength", serialize = "ToLength")]
  ToLength,
  #[strum(serialize = "kToName", serialize = "ToName")]
  ToName,
  #[strum(serialize = "kToNumber", serialize = "ToNumber")]
  ToNumber,
  #[strum(serialize = "kToNumeric", serialize = "ToNumeric")]
  ToNumeric,
  #[strum(serialize = "kToObject", serialize = "ToObject")]
  ToObject,
  #[strum(serialize = "kToString", serialize = "ToString")]
  ToString,
  #[strum(serialize = "kTryMigrateInstance", serialize = "TryMigrateInstance")]
  TryMigrateInstance,
  #[strum(
    serialize = "kTryMigrateInstanceAndMarkMapAsMigrationTarget",
    serialize = "TryMigrateInstanceAndMarkMapAsMigrationTarget"
  )]
  TryMigrateInstanceAndMarkMapAsMigrationTarget,
  #[strum(serialize = "kSetPrivateMember", serialize = "SetPrivateMember")]
  SetPrivateMember,
  #[strum(serialize = "kSwissTableAdd", serialize = "SwissTableAdd")]
  SwissTableAdd,
  #[strum(serialize = "kSwissTableAllocate", serialize = "SwissTableAllocate")]
  SwissTableAllocate,
  #[strum(serialize = "kSwissTableDelete", serialize = "SwissTableDelete")]
  SwissTableDelete,
  #[strum(serialize = "kSwissTableDetailsAt", serialize = "SwissTableDetailsAt")]
  SwissTableDetailsAt,
  #[strum(
    serialize = "kSwissTableElementsCount",
    serialize = "SwissTableElementsCount"
  )]
  SwissTableElementsCount,
  #[strum(serialize = "kSwissTableEquals", serialize = "SwissTableEquals")]
  SwissTableEquals,
  #[strum(serialize = "kSwissTableFindEntry", serialize = "SwissTableFindEntry")]
  SwissTableFindEntry,
  #[strum(serialize = "kSwissTableUpdate", serialize = "SwissTableUpdate")]
  SwissTableUpdate,
  #[strum(serialize = "kSwissTableValueAt", serialize = "SwissTableValueAt")]
  SwissTableValueAt,
  #[strum(serialize = "kSwissTableKeyAt", serialize = "SwissTableKeyAt")]
  SwissTableKeyAt,
  #[strum(serialize = "kAdd", serialize = "Add")]
  Add,
  #[strum(serialize = "kEqual", serialize = "Equal")]
  Equal,
  #[strum(serialize = "kGreaterThan", serialize = "GreaterThan")]
  GreaterThan,
  #[strum(serialize = "kGreaterThanOrEqual", serialize = "GreaterThanOrEqual")]
  GreaterThanOrEqual,
  #[strum(serialize = "kLessThan", serialize = "LessThan")]
  LessThan,
  #[strum(serialize = "kLessThanOrEqual", serialize = "LessThanOrEqual")]
  LessThanOrEqual,
  #[strum(serialize = "kNotEqual", serialize = "NotEqual")]
  NotEqual,
  #[strum(serialize = "kStrictEqual", serialize = "StrictEqual")]
  StrictEqual,
  #[strum(serialize = "kStrictNotEqual", serialize = "StrictNotEqual")]
  StrictNotEqual,
  #[strum(serialize = "kReferenceEqual", serialize = "ReferenceEqual")]
  ReferenceEqual,
  #[strum(serialize = "kEnqueueMicrotask", serialize = "EnqueueMicrotask")]
  EnqueueMicrotask,
  #[strum(serialize = "kPromiseHookAfter", serialize = "PromiseHookAfter")]
  PromiseHookAfter,
  #[strum(serialize = "kPromiseHookBefore", serialize = "PromiseHookBefore")]
  PromiseHookBefore,
  #[strum(serialize = "kPromiseHookInit", serialize = "PromiseHookInit")]
  PromiseHookInit,
  #[strum(
    serialize = "kPromiseRejectEventFromStack",
    serialize = "PromiseRejectEventFromStack"
  )]
  PromiseRejectEventFromStack,
  #[strum(serialize = "kPromiseRevokeReject", serialize = "PromiseRevokeReject")]
  PromiseRevokeReject,
  #[strum(serialize = "kRejectPromise", serialize = "RejectPromise")]
  RejectPromise,
  #[strum(serialize = "kResolvePromise", serialize = "ResolvePromise")]
  ResolvePromise,
  #[strum(
    serialize = "kConstructSuppressedError",
    serialize = "ConstructSuppressedError"
  )]
  ConstructSuppressedError,
  #[strum(
    serialize = "kConstructAggregateErrorHelper",
    serialize = "ConstructAggregateErrorHelper"
  )]
  ConstructAggregateErrorHelper,
  #[strum(
    serialize = "kConstructInternalAggregateErrorHelper",
    serialize = "ConstructInternalAggregateErrorHelper"
  )]
  ConstructInternalAggregateErrorHelper,
  #[strum(
    serialize = "kCheckProxyGetSetTrapResult",
    serialize = "CheckProxyGetSetTrapResult"
  )]
  CheckProxyGetSetTrapResult,
  #[strum(
    serialize = "kCheckProxyHasTrapResult",
    serialize = "CheckProxyHasTrapResult"
  )]
  CheckProxyHasTrapResult,
  #[strum(
    serialize = "kCheckProxyDeleteTrapResult",
    serialize = "CheckProxyDeleteTrapResult"
  )]
  CheckProxyDeleteTrapResult,
  #[strum(
    serialize = "kGetPropertyWithReceiver",
    serialize = "GetPropertyWithReceiver"
  )]
  GetPropertyWithReceiver,
  #[strum(serialize = "kIsJSProxy", serialize = "IsJSProxy")]
  IsJSProxy,
  #[strum(serialize = "kJSProxyGetHandler", serialize = "JSProxyGetHandler")]
  JSProxyGetHandler,
  #[strum(serialize = "kJSProxyGetTarget", serialize = "JSProxyGetTarget")]
  JSProxyGetTarget,
  #[strum(
    serialize = "kSetPropertyWithReceiver",
    serialize = "SetPropertyWithReceiver"
  )]
  SetPropertyWithReceiver,
  #[strum(serialize = "kRegExpBuildIndices", serialize = "RegExpBuildIndices")]
  RegExpBuildIndices,
  #[strum(
    serialize = "kRegExpGrowRegExpMatchInfo",
    serialize = "RegExpGrowRegExpMatchInfo"
  )]
  RegExpGrowRegExpMatchInfo,
  #[strum(serialize = "kRegExpExecMultiple", serialize = "RegExpExecMultiple")]
  RegExpExecMultiple,
  #[strum(
    serialize = "kRegExpInitializeAndCompile",
    serialize = "RegExpInitializeAndCompile"
  )]
  RegExpInitializeAndCompile,
  #[strum(
    serialize = "kRegExpMatchGlobalAtom",
    serialize = "RegExpMatchGlobalAtom"
  )]
  RegExpMatchGlobalAtom,
  #[strum(serialize = "kRegExpReplaceRT", serialize = "RegExpReplaceRT")]
  RegExpReplaceRT,
  #[strum(serialize = "kRegExpSplit", serialize = "RegExpSplit")]
  RegExpSplit,
  #[strum(
    serialize = "kRegExpStringFromFlags",
    serialize = "RegExpStringFromFlags"
  )]
  RegExpStringFromFlags,
  #[strum(
    serialize = "kStringReplaceNonGlobalRegExpWithFunction",
    serialize = "StringReplaceNonGlobalRegExpWithFunction"
  )]
  StringReplaceNonGlobalRegExpWithFunction,
  #[strum(serialize = "kStringSplit", serialize = "StringSplit")]
  StringSplit,
  #[strum(serialize = "kRegExpExec", serialize = "RegExpExec")]
  RegExpExec,
  #[strum(
    serialize = "kRegExpExperimentalOneshotExec",
    serialize = "RegExpExperimentalOneshotExec"
  )]
  RegExpExperimentalOneshotExec,
  #[strum(
    serialize = "kThrowConstAssignError",
    serialize = "ThrowConstAssignError"
  )]
  ThrowConstAssignError,
  #[strum(
    serialize = "kThrowUsingAssignError",
    serialize = "ThrowUsingAssignError"
  )]
  ThrowUsingAssignError,
  #[strum(
    serialize = "kThrowAwaitUsingAssignError",
    serialize = "ThrowAwaitUsingAssignError"
  )]
  ThrowAwaitUsingAssignError,
  #[strum(serialize = "kDeclareEvalFunction", serialize = "DeclareEvalFunction")]
  DeclareEvalFunction,
  #[strum(serialize = "kDeclareEvalVar", serialize = "DeclareEvalVar")]
  DeclareEvalVar,
  #[strum(serialize = "kDeclareGlobals", serialize = "DeclareGlobals")]
  DeclareGlobals,
  #[strum(
    serialize = "kDeclareModuleExports",
    serialize = "DeclareModuleExports"
  )]
  DeclareModuleExports,
  #[strum(serialize = "kDeleteLookupSlot", serialize = "DeleteLookupSlot")]
  DeleteLookupSlot,
  #[strum(serialize = "kLoadLookupSlot", serialize = "LoadLookupSlot")]
  LoadLookupSlot,
  #[strum(
    serialize = "kLoadLookupSlotInsideTypeof",
    serialize = "LoadLookupSlotInsideTypeof"
  )]
  LoadLookupSlotInsideTypeof,
  #[strum(
    serialize = "kLoadLookupSlotForCall_Baseline",
    serialize = "LoadLookupSlotForCall_Baseline",
    serialize = "LoadLookupSlotForCallBaseline"
  )]
  LoadLookupSlotForCallBaseline,
  #[strum(serialize = "kNewClosure", serialize = "NewClosure")]
  NewClosure,
  #[strum(
    serialize = "kNewClosure_Tenured",
    serialize = "NewClosure_Tenured",
    serialize = "NewClosureTenured"
  )]
  NewClosureTenured,
  #[strum(serialize = "kNewFunctionContext", serialize = "NewFunctionContext")]
  NewFunctionContext,
  #[strum(serialize = "kNewRestParameter", serialize = "NewRestParameter")]
  NewRestParameter,
  #[strum(serialize = "kNewSloppyArguments", serialize = "NewSloppyArguments")]
  NewSloppyArguments,
  #[strum(serialize = "kNewStrictArguments", serialize = "NewStrictArguments")]
  NewStrictArguments,
  #[strum(serialize = "kPushBlockContext", serialize = "PushBlockContext")]
  PushBlockContext,
  #[strum(serialize = "kPushCatchContext", serialize = "PushCatchContext")]
  PushCatchContext,
  #[strum(serialize = "kPushWithContext", serialize = "PushWithContext")]
  PushWithContext,
  #[strum(
    serialize = "kStoreGlobalNoHoleCheckForReplLetOrConst",
    serialize = "StoreGlobalNoHoleCheckForReplLetOrConst"
  )]
  StoreGlobalNoHoleCheckForReplLetOrConst,
  #[strum(
    serialize = "kStoreLookupSlot_Sloppy",
    serialize = "StoreLookupSlot_Sloppy",
    serialize = "StoreLookupSlotSloppy"
  )]
  StoreLookupSlotSloppy,
  #[strum(
    serialize = "kStoreLookupSlot_SloppyHoisting",
    serialize = "StoreLookupSlot_SloppyHoisting",
    serialize = "StoreLookupSlotSloppyHoisting"
  )]
  StoreLookupSlotSloppyHoisting,
  #[strum(
    serialize = "kStoreLookupSlot_Strict",
    serialize = "StoreLookupSlot_Strict",
    serialize = "StoreLookupSlotStrict"
  )]
  StoreLookupSlotStrict,
  #[strum(
    serialize = "kShadowRealmWrappedFunctionCreate",
    serialize = "ShadowRealmWrappedFunctionCreate"
  )]
  ShadowRealmWrappedFunctionCreate,
  #[strum(
    serialize = "kShadowRealmImportValue",
    serialize = "ShadowRealmImportValue"
  )]
  ShadowRealmImportValue,
  #[strum(serialize = "kShadowRealmThrow", serialize = "ShadowRealmThrow")]
  ShadowRealmThrow,
  #[strum(serialize = "kFlattenString", serialize = "FlattenString")]
  FlattenString,
  #[strum(serialize = "kGetSubstitution", serialize = "GetSubstitution")]
  GetSubstitution,
  #[strum(serialize = "kInternalizeString", serialize = "InternalizeString")]
  InternalizeString,
  #[strum(serialize = "kStringAdd", serialize = "StringAdd")]
  StringAdd,
  #[strum(
    serialize = "kStringAdd_LhsIsStringConstant_Internalize",
    serialize = "StringAdd_LhsIsStringConstant_Internalize",
    serialize = "StringAddLhsIsStringConstantInternalize"
  )]
  StringAddLhsIsStringConstantInternalize,
  #[strum(
    serialize = "kStringAdd_RhsIsStringConstant_Internalize",
    serialize = "StringAdd_RhsIsStringConstant_Internalize",
    serialize = "StringAddRhsIsStringConstantInternalize"
  )]
  StringAddRhsIsStringConstantInternalize,
  #[strum(serialize = "kStringBuilderConcat", serialize = "StringBuilderConcat")]
  StringBuilderConcat,
  #[strum(serialize = "kStringCharCodeAt", serialize = "StringCharCodeAt")]
  StringCharCodeAt,
  #[strum(serialize = "kStringCodePointAt", serialize = "StringCodePointAt")]
  StringCodePointAt,
  #[strum(serialize = "kStringCompare", serialize = "StringCompare")]
  StringCompare,
  #[strum(serialize = "kStringEqual", serialize = "StringEqual")]
  StringEqual,
  #[strum(serialize = "kStringEscapeQuotes", serialize = "StringEscapeQuotes")]
  StringEscapeQuotes,
  #[strum(serialize = "kStringGreaterThan", serialize = "StringGreaterThan")]
  StringGreaterThan,
  #[strum(
    serialize = "kStringGreaterThanOrEqual",
    serialize = "StringGreaterThanOrEqual"
  )]
  StringGreaterThanOrEqual,
  #[strum(serialize = "kStringIsWellFormed", serialize = "StringIsWellFormed")]
  StringIsWellFormed,
  #[strum(serialize = "kStringLastIndexOf", serialize = "StringLastIndexOf")]
  StringLastIndexOf,
  #[strum(serialize = "kStringLessThan", serialize = "StringLessThan")]
  StringLessThan,
  #[strum(
    serialize = "kStringLessThanOrEqual",
    serialize = "StringLessThanOrEqual"
  )]
  StringLessThanOrEqual,
  #[strum(serialize = "kStringMaxLength", serialize = "StringMaxLength")]
  StringMaxLength,
  #[strum(
    serialize = "kStringReplaceOneCharWithString",
    serialize = "StringReplaceOneCharWithString"
  )]
  StringReplaceOneCharWithString,
  #[strum(serialize = "kStringSubstring", serialize = "StringSubstring")]
  StringSubstring,
  #[strum(serialize = "kStringToArray", serialize = "StringToArray")]
  StringToArray,
  #[strum(serialize = "kStringToWellFormed", serialize = "StringToWellFormed")]
  StringToWellFormed,
  #[strum(
    serialize = "kCreatePrivateNameSymbol",
    serialize = "CreatePrivateNameSymbol"
  )]
  CreatePrivateNameSymbol,
  #[strum(
    serialize = "kCreatePrivateBrandSymbol",
    serialize = "CreatePrivateBrandSymbol"
  )]
  CreatePrivateBrandSymbol,
  #[strum(serialize = "kCreatePrivateSymbol", serialize = "CreatePrivateSymbol")]
  CreatePrivateSymbol,
  #[strum(
    serialize = "kSymbolDescriptiveString",
    serialize = "SymbolDescriptiveString"
  )]
  SymbolDescriptiveString,
  #[strum(serialize = "kSymbolIsPrivate", serialize = "SymbolIsPrivate")]
  SymbolIsPrivate,
  #[strum(serialize = "kAbort", serialize = "Abort")]
  Abort,
  #[strum(serialize = "kAbortCSADcheck", serialize = "AbortCSADcheck")]
  AbortCSADcheck,
  #[strum(serialize = "kAbortJS", serialize = "AbortJS")]
  AbortJS,
  #[strum(
    serialize = "kActiveTierIsIgnition",
    serialize = "ActiveTierIsIgnition"
  )]
  ActiveTierIsIgnition,
  #[strum(
    serialize = "kActiveTierIsSparkplug",
    serialize = "ActiveTierIsSparkplug"
  )]
  ActiveTierIsSparkplug,
  #[strum(serialize = "kActiveTierIsMaglev", serialize = "ActiveTierIsMaglev")]
  ActiveTierIsMaglev,
  #[strum(
    serialize = "kActiveTierIsTurbofan",
    serialize = "ActiveTierIsTurbofan"
  )]
  ActiveTierIsTurbofan,
  #[strum(
    serialize = "kAllocateHeapNumberWithValue",
    serialize = "AllocateHeapNumberWithValue"
  )]
  AllocateHeapNumberWithValue,
  #[strum(
    serialize = "kArrayBufferDetachForceWasm",
    serialize = "ArrayBufferDetachForceWasm"
  )]
  ArrayBufferDetachForceWasm,
  #[strum(
    serialize = "kArrayIteratorProtector",
    serialize = "ArrayIteratorProtector"
  )]
  ArrayIteratorProtector,
  #[strum(
    serialize = "kArraySpeciesProtector",
    serialize = "ArraySpeciesProtector"
  )]
  ArraySpeciesProtector,
  #[strum(serialize = "kAssertNotPeeled", serialize = "AssertNotPeeled")]
  AssertNotPeeled,
  #[strum(serialize = "kAssertPeeled", serialize = "AssertPeeled")]
  AssertPeeled,
  #[strum(serialize = "kBaselineOsr", serialize = "BaselineOsr")]
  BaselineOsr,
  #[strum(serialize = "kBenchMaglev", serialize = "BenchMaglev")]
  BenchMaglev,
  #[strum(serialize = "kBenchTurbofan", serialize = "BenchTurbofan")]
  BenchTurbofan,
  #[strum(serialize = "kBlockAt", serialize = "BlockAt")]
  BlockAt,
  #[strum(
    serialize = "kVerifyGetJSBuiltinState",
    serialize = "VerifyGetJSBuiltinState"
  )]
  VerifyGetJSBuiltinState,
  #[strum(
    serialize = "kClearFunctionFeedback",
    serialize = "ClearFunctionFeedback"
  )]
  ClearFunctionFeedback,
  #[strum(
    serialize = "kClearMegamorphicStubCache",
    serialize = "ClearMegamorphicStubCache"
  )]
  ClearMegamorphicStubCache,
  #[strum(
    serialize = "kCompleteInobjectSlackTracking",
    serialize = "CompleteInobjectSlackTracking"
  )]
  CompleteInobjectSlackTracking,
  #[strum(serialize = "kConstructConsString", serialize = "ConstructConsString")]
  ConstructConsString,
  #[strum(serialize = "kConstructDouble", serialize = "ConstructDouble")]
  ConstructDouble,
  #[strum(
    serialize = "kConstructInternalizedString",
    serialize = "ConstructInternalizedString"
  )]
  ConstructInternalizedString,
  #[strum(
    serialize = "kConstructSlicedString",
    serialize = "ConstructSlicedString"
  )]
  ConstructSlicedString,
  #[strum(serialize = "kConstructThinString", serialize = "ConstructThinString")]
  ConstructThinString,
  #[strum(
    serialize = "kCurrentFrameIsTurbofan",
    serialize = "CurrentFrameIsTurbofan"
  )]
  CurrentFrameIsTurbofan,
  #[strum(serialize = "kDebugPrint", serialize = "DebugPrint")]
  DebugPrint,
  #[strum(
    serialize = "kDebugPrintCppHeapPointerTable",
    serialize = "DebugPrintCppHeapPointerTable"
  )]
  DebugPrintCppHeapPointerTable,
  #[strum(
    serialize = "kDebugPrintCppHeapPointerTableFilterTag",
    serialize = "DebugPrintCppHeapPointerTableFilterTag"
  )]
  DebugPrintCppHeapPointerTableFilterTag,
  #[strum(
    serialize = "kDebugPrintExternalPointerTable",
    serialize = "DebugPrintExternalPointerTable"
  )]
  DebugPrintExternalPointerTable,
  #[strum(
    serialize = "kDebugPrintExternalPointerTableFilterTag",
    serialize = "DebugPrintExternalPointerTableFilterTag"
  )]
  DebugPrintExternalPointerTableFilterTag,
  #[strum(serialize = "kDebugPrintGeneric", serialize = "DebugPrintGeneric")]
  DebugPrintGeneric,
  #[strum(serialize = "kDebugPrintFloat", serialize = "DebugPrintFloat")]
  DebugPrintFloat,
  #[strum(serialize = "kDebugPrintPtr", serialize = "DebugPrintPtr")]
  DebugPrintPtr,
  #[strum(serialize = "kDebugPrintWord", serialize = "DebugPrintWord")]
  DebugPrintWord,
  #[strum(serialize = "kDebugTrace", serialize = "DebugTrace")]
  DebugTrace,
  #[strum(serialize = "kDebugTraceMinimal", serialize = "DebugTraceMinimal")]
  DebugTraceMinimal,
  #[strum(serialize = "kDeoptimizeFunction", serialize = "DeoptimizeFunction")]
  DeoptimizeFunction,
  #[strum(
    serialize = "kDisableOptimizationFinalization",
    serialize = "DisableOptimizationFinalization"
  )]
  DisableOptimizationFinalization,
  #[strum(
    serialize = "kDisallowCodegenFromStrings",
    serialize = "DisallowCodegenFromStrings"
  )]
  DisallowCodegenFromStrings,
  #[strum(serialize = "kDisassembleFunction", serialize = "DisassembleFunction")]
  DisassembleFunction,
  #[strum(
    serialize = "kEnableCodeLoggingForTesting",
    serialize = "EnableCodeLoggingForTesting"
  )]
  EnableCodeLoggingForTesting,
  #[strum(
    serialize = "kEnsureFeedbackVectorForFunction",
    serialize = "EnsureFeedbackVectorForFunction"
  )]
  EnsureFeedbackVectorForFunction,
  #[strum(
    serialize = "kFinalizeOptimization",
    serialize = "FinalizeOptimization"
  )]
  FinalizeOptimization,
  #[strum(serialize = "kForceFlush", serialize = "ForceFlush")]
  ForceFlush,
  #[strum(
    serialize = "kMajorGCForCompilerTesting",
    serialize = "MajorGCForCompilerTesting"
  )]
  MajorGCForCompilerTesting,
  #[strum(
    serialize = "kGetAbstractModuleSource",
    serialize = "GetAbstractModuleSource"
  )]
  GetAbstractModuleSource,
  #[strum(serialize = "kGetBytecode", serialize = "GetBytecode")]
  GetBytecode,
  #[strum(
    serialize = "kExhaustInterruptBudget",
    serialize = "ExhaustInterruptBudget"
  )]
  ExhaustInterruptBudget,
  #[strum(serialize = "kGetCallable", serialize = "GetCallable")]
  GetCallable,
  #[strum(serialize = "kGetFeedback", serialize = "GetFeedback")]
  GetFeedback,
  #[strum(
    serialize = "kGetFunctionForCurrentFrame",
    serialize = "GetFunctionForCurrentFrame"
  )]
  GetFunctionForCurrentFrame,
  #[strum(
    serialize = "kGetInitializerFunction",
    serialize = "GetInitializerFunction"
  )]
  GetInitializerFunction,
  #[strum(
    serialize = "kGetOptimizationStatus",
    serialize = "GetOptimizationStatus"
  )]
  GetOptimizationStatus,
  #[strum(serialize = "kGetUndetectable", serialize = "GetUndetectable")]
  GetUndetectable,
  #[strum(
    serialize = "kGetWeakCollectionSize",
    serialize = "GetWeakCollectionSize"
  )]
  GetWeakCollectionSize,
  #[strum(serialize = "kGlobalPrint", serialize = "GlobalPrint")]
  GlobalPrint,
  #[strum(serialize = "kHasCowElements", serialize = "HasCowElements")]
  HasCowElements,
  #[strum(
    serialize = "kHasDictionaryElements",
    serialize = "HasDictionaryElements"
  )]
  HasDictionaryElements,
  #[strum(serialize = "kHasDoubleElements", serialize = "HasDoubleElements")]
  HasDoubleElements,
  #[strum(
    serialize = "kHasElementsInALargeObjectSpace",
    serialize = "HasElementsInALargeObjectSpace"
  )]
  HasElementsInALargeObjectSpace,
  #[strum(serialize = "kHasFastElements", serialize = "HasFastElements")]
  HasFastElements,
  #[strum(serialize = "kHasFastProperties", serialize = "HasFastProperties")]
  HasFastProperties,
  #[strum(
    serialize = "kHasFixedBigInt64Elements",
    serialize = "HasFixedBigInt64Elements"
  )]
  HasFixedBigInt64Elements,
  #[strum(
    serialize = "kHasFixedBigUint64Elements",
    serialize = "HasFixedBigUint64Elements"
  )]
  HasFixedBigUint64Elements,
  #[strum(
    serialize = "kHasFixedFloat16Elements",
    serialize = "HasFixedFloat16Elements"
  )]
  HasFixedFloat16Elements,
  #[strum(
    serialize = "kHasFixedFloat32Elements",
    serialize = "HasFixedFloat32Elements"
  )]
  HasFixedFloat32Elements,
  #[strum(
    serialize = "kHasFixedFloat64Elements",
    serialize = "HasFixedFloat64Elements"
  )]
  HasFixedFloat64Elements,
  #[strum(
    serialize = "kHasFixedInt16Elements",
    serialize = "HasFixedInt16Elements"
  )]
  HasFixedInt16Elements,
  #[strum(
    serialize = "kHasFixedInt32Elements",
    serialize = "HasFixedInt32Elements"
  )]
  HasFixedInt32Elements,
  #[strum(
    serialize = "kHasFixedInt8Elements",
    serialize = "HasFixedInt8Elements"
  )]
  HasFixedInt8Elements,
  #[strum(
    serialize = "kHasFixedUint16Elements",
    serialize = "HasFixedUint16Elements"
  )]
  HasFixedUint16Elements,
  #[strum(
    serialize = "kHasFixedUint32Elements",
    serialize = "HasFixedUint32Elements"
  )]
  HasFixedUint32Elements,
  #[strum(
    serialize = "kHasFixedUint8ClampedElements",
    serialize = "HasFixedUint8ClampedElements"
  )]
  HasFixedUint8ClampedElements,
  #[strum(
    serialize = "kHasFixedUint8Elements",
    serialize = "HasFixedUint8Elements"
  )]
  HasFixedUint8Elements,
  #[strum(serialize = "kHasHoleyElements", serialize = "HasHoleyElements")]
  HasHoleyElements,
  #[strum(serialize = "kHasObjectElements", serialize = "HasObjectElements")]
  HasObjectElements,
  #[strum(serialize = "kHasPackedElements", serialize = "HasPackedElements")]
  HasPackedElements,
  #[strum(
    serialize = "kHasSloppyArgumentsElements",
    serialize = "HasSloppyArgumentsElements"
  )]
  HasSloppyArgumentsElements,
  #[strum(serialize = "kHasSmiElements", serialize = "HasSmiElements")]
  HasSmiElements,
  #[strum(
    serialize = "kHasSmiOrObjectElements",
    serialize = "HasSmiOrObjectElements"
  )]
  HasSmiOrObjectElements,
  #[strum(serialize = "kHaveSameMap", serialize = "HaveSameMap")]
  HaveSameMap,
  #[strum(serialize = "kHeapObjectVerify", serialize = "HeapObjectVerify")]
  HeapObjectVerify,
  #[strum(serialize = "kICsAreEnabled", serialize = "ICsAreEnabled")]
  ICsAreEnabled,
  #[strum(serialize = "kInLargeObjectSpace", serialize = "InLargeObjectSpace")]
  InLargeObjectSpace,
  #[strum(serialize = "kInstallBytecode", serialize = "InstallBytecode")]
  InstallBytecode,
  #[strum(serialize = "kInYoungGeneration", serialize = "InYoungGeneration")]
  InYoungGeneration,
  #[strum(serialize = "kIs64Bit", serialize = "Is64Bit")]
  Is64Bit,
  #[strum(
    serialize = "kIsAtomicsWaitAllowed",
    serialize = "IsAtomicsWaitAllowed"
  )]
  IsAtomicsWaitAllowed,
  #[strum(serialize = "kIsBeingInterpreted", serialize = "IsBeingInterpreted")]
  IsBeingInterpreted,
  #[strum(
    serialize = "kIsConcatSpreadableProtector",
    serialize = "IsConcatSpreadableProtector"
  )]
  IsConcatSpreadableProtector,
  #[strum(
    serialize = "kIsConcurrentRecompilationSupported",
    serialize = "IsConcurrentRecompilationSupported"
  )]
  IsConcurrentRecompilationSupported,
  #[strum(
    serialize = "kIsDictPropertyConstTrackingEnabled",
    serialize = "IsDictPropertyConstTrackingEnabled"
  )]
  IsDictPropertyConstTrackingEnabled,
  #[strum(
    serialize = "kIsEfficiencyModeEnabled",
    serialize = "IsEfficiencyModeEnabled"
  )]
  IsEfficiencyModeEnabled,
  #[strum(
    serialize = "kIsInPlaceInternalizableString",
    serialize = "IsInPlaceInternalizableString"
  )]
  IsInPlaceInternalizableString,
  #[strum(
    serialize = "kIsInternalizedString",
    serialize = "IsInternalizedString"
  )]
  IsInternalizedString,
  #[strum(serialize = "kStringToCString", serialize = "StringToCString")]
  StringToCString,
  #[strum(serialize = "kStringUtf8Value", serialize = "StringUtf8Value")]
  StringUtf8Value,
  #[strum(
    serialize = "kIsUndefinedDoubleEnabled",
    serialize = "IsUndefinedDoubleEnabled"
  )]
  IsUndefinedDoubleEnabled,
  #[strum(serialize = "kIsMaglevEnabled", serialize = "IsMaglevEnabled")]
  IsMaglevEnabled,
  #[strum(serialize = "kIsSameHeapObject", serialize = "IsSameHeapObject")]
  IsSameHeapObject,
  #[strum(serialize = "kIsSharedString", serialize = "IsSharedString")]
  IsSharedString,
  #[strum(
    serialize = "kIsInWritableSharedSpace",
    serialize = "IsInWritableSharedSpace"
  )]
  IsInWritableSharedSpace,
  #[strum(serialize = "kIsSparkplugEnabled", serialize = "IsSparkplugEnabled")]
  IsSparkplugEnabled,
  #[strum(serialize = "kIsTurbofanEnabled", serialize = "IsTurbofanEnabled")]
  IsTurbofanEnabled,
  #[strum(
    serialize = "kIsWasmTieringPredictable",
    serialize = "IsWasmTieringPredictable"
  )]
  IsWasmTieringPredictable,
  #[strum(
    serialize = "kMapIteratorProtector",
    serialize = "MapIteratorProtector"
  )]
  MapIteratorProtector,
  #[strum(
    serialize = "kNeverOptimizeFunction",
    serialize = "NeverOptimizeFunction"
  )]
  NeverOptimizeFunction,
  #[strum(
    serialize = "kNewRegExpWithBacktrackLimit",
    serialize = "NewRegExpWithBacktrackLimit"
  )]
  NewRegExpWithBacktrackLimit,
  #[strum(serialize = "kNoElementsProtector", serialize = "NoElementsProtector")]
  NoElementsProtector,
  #[strum(
    serialize = "kNotifyContextDisposed",
    serialize = "NotifyContextDisposed"
  )]
  NotifyContextDisposed,
  #[strum(
    serialize = "kSetPriorityBestEffort",
    serialize = "SetPriorityBestEffort"
  )]
  SetPriorityBestEffort,
  #[strum(
    serialize = "kSetPriorityUserVisible",
    serialize = "SetPriorityUserVisible"
  )]
  SetPriorityUserVisible,
  #[strum(
    serialize = "kSetPriorityUserBlocking",
    serialize = "SetPriorityUserBlocking"
  )]
  SetPriorityUserBlocking,
  #[strum(
    serialize = "kOptimizeMaglevOnNextCall",
    serialize = "OptimizeMaglevOnNextCall"
  )]
  OptimizeMaglevOnNextCall,
  #[strum(
    serialize = "kOptimizeFunctionOnNextCall",
    serialize = "OptimizeFunctionOnNextCall"
  )]
  OptimizeFunctionOnNextCall,
  #[strum(serialize = "kOptimizeOsr", serialize = "OptimizeOsr")]
  OptimizeOsr,
  #[strum(
    serialize = "kPrepareFunctionForOptimization",
    serialize = "PrepareFunctionForOptimization"
  )]
  PrepareFunctionForOptimization,
  #[strum(
    serialize = "kPretenureAllocationSite",
    serialize = "PretenureAllocationSite"
  )]
  PretenureAllocationSite,
  #[strum(
    serialize = "kPrintWithNameForAssert",
    serialize = "PrintWithNameForAssert"
  )]
  PrintWithNameForAssert,
  #[strum(
    serialize = "kPromiseSpeciesProtector",
    serialize = "PromiseSpeciesProtector"
  )]
  PromiseSpeciesProtector,
  #[strum(
    serialize = "kRegExpSpeciesProtector",
    serialize = "RegExpSpeciesProtector"
  )]
  RegExpSpeciesProtector,
  #[strum(serialize = "kRegexpHasBytecode", serialize = "RegexpHasBytecode")]
  RegexpHasBytecode,
  #[strum(serialize = "kRegexpHasNativeCode", serialize = "RegexpHasNativeCode")]
  RegexpHasNativeCode,
  #[strum(serialize = "kRegexpIsUnmodified", serialize = "RegexpIsUnmodified")]
  RegexpIsUnmodified,
  #[strum(
    serialize = "kRegexpQuickCheckRejects",
    serialize = "RegexpQuickCheckRejects"
  )]
  RegexpQuickCheckRejects,
  #[strum(serialize = "kRegexpTypeTag", serialize = "RegexpTypeTag")]
  RegexpTypeTag,
  #[strum(serialize = "kResume", serialize = "Resume")]
  Resume,
  #[strum(serialize = "kRunningInSimulator", serialize = "RunningInSimulator")]
  RunningInSimulator,
  #[strum(serialize = "kRuntimeEvaluateREPL", serialize = "RuntimeEvaluateREPL")]
  RuntimeEvaluateREPL,
  #[strum(
    serialize = "kScheduleGCInStackCheck",
    serialize = "ScheduleGCInStackCheck"
  )]
  ScheduleGCInStackCheck,
  #[strum(
    serialize = "kSerializeDeserializeNow",
    serialize = "SerializeDeserializeNow"
  )]
  SerializeDeserializeNow,
  #[strum(
    serialize = "kSetAllocationTimeout",
    serialize = "SetAllocationTimeout"
  )]
  SetAllocationTimeout,
  #[strum(
    serialize = "kSetDispatchTableGCInterval",
    serialize = "SetDispatchTableGCInterval"
  )]
  SetDispatchTableGCInterval,
  #[strum(serialize = "kSetBatterySaverMode", serialize = "SetBatterySaverMode")]
  SetBatterySaverMode,
  #[strum(serialize = "kSetForceSlowPath", serialize = "SetForceSlowPath")]
  SetForceSlowPath,
  #[strum(
    serialize = "kSetIteratorProtector",
    serialize = "SetIteratorProtector"
  )]
  SetIteratorProtector,
  #[strum(serialize = "kSharedGC", serialize = "SharedGC")]
  SharedGC,
  #[strum(serialize = "kShareObject", serialize = "ShareObject")]
  ShareObject,
  #[strum(
    serialize = "kSimulateNewspaceFull",
    serialize = "SimulateNewspaceFull"
  )]
  SimulateNewspaceFull,
  #[strum(serialize = "kStringIsFlat", serialize = "StringIsFlat")]
  StringIsFlat,
  #[strum(
    serialize = "kStringIsOneByteRepresentation",
    serialize = "StringIsOneByteRepresentation"
  )]
  StringIsOneByteRepresentation,
  #[strum(
    serialize = "kStringIteratorProtector",
    serialize = "StringIteratorProtector"
  )]
  StringIteratorProtector,
  #[strum(
    serialize = "kStringWrapperToPrimitiveProtector",
    serialize = "StringWrapperToPrimitiveProtector"
  )]
  StringWrapperToPrimitiveProtector,
  #[strum(serialize = "kSystemBreak", serialize = "SystemBreak")]
  SystemBreak,
  #[strum(serialize = "kTakeHeapSnapshot", serialize = "TakeHeapSnapshot")]
  TakeHeapSnapshot,
  #[strum(serialize = "kTraceEnter", serialize = "TraceEnter")]
  TraceEnter,
  #[strum(serialize = "kTraceExit", serialize = "TraceExit")]
  TraceExit,
  #[strum(
    serialize = "kTurbofanStaticAssert",
    serialize = "TurbofanStaticAssert"
  )]
  TurbofanStaticAssert,
  #[strum(
    serialize = "kAssertEscapeAnalysisElided",
    serialize = "AssertEscapeAnalysisElided"
  )]
  AssertEscapeAnalysisElided,
  #[strum(
    serialize = "kTypedArraySpeciesProtector",
    serialize = "TypedArraySpeciesProtector"
  )]
  TypedArraySpeciesProtector,
  #[strum(
    serialize = "kWaitForBackgroundOptimization",
    serialize = "WaitForBackgroundOptimization"
  )]
  WaitForBackgroundOptimization,
  #[strum(serialize = "kWaitUntilBlocked", serialize = "WaitUntilBlocked")]
  WaitUntilBlocked,
  #[strum(serialize = "kDeoptimizeNow", serialize = "DeoptimizeNow")]
  DeoptimizeNow,
  #[strum(serialize = "kLeakHole", serialize = "LeakHole")]
  LeakHole,
  #[strum(serialize = "kGetHoleNaNLower", serialize = "GetHoleNaNLower")]
  GetHoleNaNLower,
  #[strum(serialize = "kGetHoleNaNUpper", serialize = "GetHoleNaNUpper")]
  GetHoleNaNUpper,
  #[strum(serialize = "kGetHoleNaN", serialize = "GetHoleNaN")]
  GetHoleNaN,
  #[strum(serialize = "kGetUndefinedNaN", serialize = "GetUndefinedNaN")]
  GetUndefinedNaN,
  #[strum(serialize = "kArrayBufferDetach", serialize = "ArrayBufferDetach")]
  ArrayBufferDetach,
  #[strum(
    serialize = "kArrayBufferSetDetachKey",
    serialize = "ArrayBufferSetDetachKey"
  )]
  ArrayBufferSetDetachKey,
  #[strum(
    serialize = "kGrowableSharedArrayBufferByteLength",
    serialize = "GrowableSharedArrayBufferByteLength"
  )]
  GrowableSharedArrayBufferByteLength,
  #[strum(
    serialize = "kTypedArrayCopyElements",
    serialize = "TypedArrayCopyElements"
  )]
  TypedArrayCopyElements,
  #[strum(serialize = "kTypedArrayGetBuffer", serialize = "TypedArrayGetBuffer")]
  TypedArrayGetBuffer,
  #[strum(serialize = "kTypedArraySet", serialize = "TypedArraySet")]
  TypedArraySet,
  #[strum(serialize = "kTypedArraySortFast", serialize = "TypedArraySortFast")]
  TypedArraySortFast,
  #[strum(serialize = "kWasmRunInterpreter", serialize = "WasmRunInterpreter")]
  WasmRunInterpreter,
  #[strum(
    serialize = "kThrowWasmJSPISuspendError",
    serialize = "ThrowWasmJSPISuspendError"
  )]
  ThrowWasmJSPISuspendError,
  #[strum(
    serialize = "kThrowWasmFXSuspendError",
    serialize = "ThrowWasmFXSuspendError"
  )]
  ThrowWasmFXSuspendError,
  #[strum(serialize = "kThrowWasmError", serialize = "ThrowWasmError")]
  ThrowWasmError,
  #[strum(
    serialize = "kThrowWasmStackOverflow",
    serialize = "ThrowWasmStackOverflow"
  )]
  ThrowWasmStackOverflow,
  #[strum(serialize = "kWasmI32AtomicWait", serialize = "WasmI32AtomicWait")]
  WasmI32AtomicWait,
  #[strum(serialize = "kWasmI64AtomicWait", serialize = "WasmI64AtomicWait")]
  WasmI64AtomicWait,
  #[strum(
    serialize = "kWasmManagedObjectWait32",
    serialize = "WasmManagedObjectWait32"
  )]
  WasmManagedObjectWait32,
  #[strum(
    serialize = "kWasmManagedObjectWait64",
    serialize = "WasmManagedObjectWait64"
  )]
  WasmManagedObjectWait64,
  #[strum(
    serialize = "kWasmManagedObjectWaitRef",
    serialize = "WasmManagedObjectWaitRef"
  )]
  WasmManagedObjectWaitRef,
  #[strum(serialize = "kWasmWaitqueueNew", serialize = "WasmWaitqueueNew")]
  WasmWaitqueueNew,
  #[strum(serialize = "kWasmMemoryGrow", serialize = "WasmMemoryGrow")]
  WasmMemoryGrow,
  #[strum(serialize = "kWasmStackGuard", serialize = "WasmStackGuard")]
  WasmStackGuard,
  #[strum(serialize = "kWasmStackGuardLoop", serialize = "WasmStackGuardLoop")]
  WasmStackGuardLoop,
  #[strum(serialize = "kWasmThrow", serialize = "WasmThrow")]
  WasmThrow,
  #[strum(serialize = "kWasmReThrow", serialize = "WasmReThrow")]
  WasmReThrow,
  #[strum(
    serialize = "kWasmThrowJSTypeError",
    serialize = "WasmThrowJSTypeError"
  )]
  WasmThrowJSTypeError,
  #[strum(serialize = "kWasmThrowTypeError", serialize = "WasmThrowTypeError")]
  WasmThrowTypeError,
  #[strum(serialize = "kWasmThrowRangeError", serialize = "WasmThrowRangeError")]
  WasmThrowRangeError,
  #[strum(
    serialize = "kWasmThrowDataViewTypeError",
    serialize = "WasmThrowDataViewTypeError"
  )]
  WasmThrowDataViewTypeError,
  #[strum(
    serialize = "kWasmThrowDataViewDetachedError",
    serialize = "WasmThrowDataViewDetachedError"
  )]
  WasmThrowDataViewDetachedError,
  #[strum(serialize = "kWasmRefFunc", serialize = "WasmRefFunc")]
  WasmRefFunc,
  #[strum(
    serialize = "kWasmInternalFunctionCreateExternal",
    serialize = "WasmInternalFunctionCreateExternal"
  )]
  WasmInternalFunctionCreateExternal,
  #[strum(
    serialize = "kWasmFunctionTableGet",
    serialize = "WasmFunctionTableGet"
  )]
  WasmFunctionTableGet,
  #[strum(
    serialize = "kWasmFunctionTableSet",
    serialize = "WasmFunctionTableSet"
  )]
  WasmFunctionTableSet,
  #[strum(serialize = "kWasmTableInit", serialize = "WasmTableInit")]
  WasmTableInit,
  #[strum(serialize = "kWasmTableCopy", serialize = "WasmTableCopy")]
  WasmTableCopy,
  #[strum(serialize = "kWasmTableGrow", serialize = "WasmTableGrow")]
  WasmTableGrow,
  #[strum(serialize = "kWasmTableFill", serialize = "WasmTableFill")]
  WasmTableFill,
  #[strum(serialize = "kWasmJSToWasmObject", serialize = "WasmJSToWasmObject")]
  WasmJSToWasmObject,
  #[strum(serialize = "kWasmWasmToJSObject", serialize = "WasmWasmToJSObject")]
  WasmWasmToJSObject,
  #[strum(serialize = "kWasmCompileLazy", serialize = "WasmCompileLazy")]
  WasmCompileLazy,
  #[strum(
    serialize = "kWasmAllocateFeedbackVector",
    serialize = "WasmAllocateFeedbackVector"
  )]
  WasmAllocateFeedbackVector,
  #[strum(
    serialize = "kWasmLiftoffDeoptFinish",
    serialize = "WasmLiftoffDeoptFinish"
  )]
  WasmLiftoffDeoptFinish,
  #[strum(
    serialize = "kTierUpJSToWasmWrapper",
    serialize = "TierUpJSToWasmWrapper"
  )]
  TierUpJSToWasmWrapper,
  #[strum(
    serialize = "kIsWasmExternalFunction",
    serialize = "IsWasmExternalFunction"
  )]
  IsWasmExternalFunction,
  #[strum(
    serialize = "kTierUpWasmToJSWrapper",
    serialize = "TierUpWasmToJSWrapper"
  )]
  TierUpWasmToJSWrapper,
  #[strum(serialize = "kWasmTriggerTierUp", serialize = "WasmTriggerTierUp")]
  WasmTriggerTierUp,
  #[strum(serialize = "kWasmDebugBreak", serialize = "WasmDebugBreak")]
  WasmDebugBreak,
  #[strum(
    serialize = "kWasmAllocateDescriptorStruct",
    serialize = "WasmAllocateDescriptorStruct"
  )]
  WasmAllocateDescriptorStruct,
  #[strum(serialize = "kWasmArrayCopy", serialize = "WasmArrayCopy")]
  WasmArrayCopy,
  #[strum(serialize = "kWasmArrayNewSegment", serialize = "WasmArrayNewSegment")]
  WasmArrayNewSegment,
  #[strum(
    serialize = "kWasmArrayInitSegment",
    serialize = "WasmArrayInitSegment"
  )]
  WasmArrayInitSegment,
  #[strum(
    serialize = "kWasmAllocateSuspender",
    serialize = "WasmAllocateSuspender"
  )]
  WasmAllocateSuspender,
  #[strum(serialize = "kWasmSuspended", serialize = "WasmSuspended")]
  WasmSuspended,
  #[strum(
    serialize = "kWasmAllocateContinuation",
    serialize = "WasmAllocateContinuation"
  )]
  WasmAllocateContinuation,
  #[strum(
    serialize = "kClearWasmSuspenderResumeField",
    serialize = "ClearWasmSuspenderResumeField"
  )]
  ClearWasmSuspenderResumeField,
  #[strum(
    serialize = "kWasmCastToSpecialPrimitiveArray",
    serialize = "WasmCastToSpecialPrimitiveArray"
  )]
  WasmCastToSpecialPrimitiveArray,
  #[strum(
    serialize = "kWasmStringAdd_NoMapCheck_Shared",
    serialize = "WasmStringAdd_NoMapCheck_Shared",
    serialize = "WasmStringAddNoMapCheckShared"
  )]
  WasmStringAddNoMapCheckShared,
  #[strum(
    serialize = "kWasmStringNewSegmentWtf8",
    serialize = "WasmStringNewSegmentWtf8"
  )]
  WasmStringNewSegmentWtf8,
  #[strum(serialize = "kWasmStringNewWtf8", serialize = "WasmStringNewWtf8")]
  WasmStringNewWtf8,
  #[strum(
    serialize = "kWasmStringNewWtf8Array",
    serialize = "WasmStringNewWtf8Array"
  )]
  WasmStringNewWtf8Array,
  #[strum(serialize = "kWasmStringNewWtf16", serialize = "WasmStringNewWtf16")]
  WasmStringNewWtf16,
  #[strum(
    serialize = "kWasmStringNewWtf16Array",
    serialize = "WasmStringNewWtf16Array"
  )]
  WasmStringNewWtf16Array,
  #[strum(serialize = "kWasmStringConst", serialize = "WasmStringConst")]
  WasmStringConst,
  #[strum(
    serialize = "kWasmStringMeasureUtf8",
    serialize = "WasmStringMeasureUtf8"
  )]
  WasmStringMeasureUtf8,
  #[strum(
    serialize = "kWasmStringMeasureWtf8",
    serialize = "WasmStringMeasureWtf8"
  )]
  WasmStringMeasureWtf8,
  #[strum(
    serialize = "kWasmStringEncodeWtf8",
    serialize = "WasmStringEncodeWtf8"
  )]
  WasmStringEncodeWtf8,
  #[strum(
    serialize = "kWasmStringEncodeWtf16",
    serialize = "WasmStringEncodeWtf16"
  )]
  WasmStringEncodeWtf16,
  #[strum(
    serialize = "kWasmStringEncodeWtf8Array",
    serialize = "WasmStringEncodeWtf8Array"
  )]
  WasmStringEncodeWtf8Array,
  #[strum(
    serialize = "kWasmStringToUtf8Array",
    serialize = "WasmStringToUtf8Array"
  )]
  WasmStringToUtf8Array,
  #[strum(serialize = "kWasmStringAsWtf8", serialize = "WasmStringAsWtf8")]
  WasmStringAsWtf8,
  #[strum(
    serialize = "kWasmStringViewWtf8Encode",
    serialize = "WasmStringViewWtf8Encode"
  )]
  WasmStringViewWtf8Encode,
  #[strum(
    serialize = "kWasmStringViewWtf8Slice",
    serialize = "WasmStringViewWtf8Slice"
  )]
  WasmStringViewWtf8Slice,
  #[strum(
    serialize = "kWasmStringFromCodePoint",
    serialize = "WasmStringFromCodePoint"
  )]
  WasmStringFromCodePoint,
  #[strum(serialize = "kWasmStringHash", serialize = "WasmStringHash")]
  WasmStringHash,
  #[strum(serialize = "kWasmSubstring", serialize = "WasmSubstring")]
  WasmSubstring,
  #[strum(serialize = "kWasmSubstringShared", serialize = "WasmSubstringShared")]
  WasmSubstringShared,
  #[strum(
    serialize = "kWasmConfigureAllPrototypes",
    serialize = "WasmConfigureAllPrototypes"
  )]
  WasmConfigureAllPrototypes,
  #[strum(
    serialize = "kWasmConfigureAllPrototypesOpt",
    serialize = "WasmConfigureAllPrototypesOpt"
  )]
  WasmConfigureAllPrototypesOpt,
  #[strum(
    serialize = "kDebugCollectWasmCoverage",
    serialize = "DebugCollectWasmCoverage"
  )]
  DebugCollectWasmCoverage,
  #[strum(
    serialize = "kTsanAcquireForInitializationFence",
    serialize = "TsanAcquireForInitializationFence"
  )]
  TsanAcquireForInitializationFence,
  #[strum(
    serialize = "kWasmTypeAssertionFailed",
    serialize = "WasmTypeAssertionFailed"
  )]
  WasmTypeAssertionFailed,
  #[strum(
    serialize = "kBuildRefTypeBitfield",
    serialize = "BuildRefTypeBitfield"
  )]
  BuildRefTypeBitfield,
  #[strum(
    serialize = "kCheckIsOnCentralStack",
    serialize = "CheckIsOnCentralStack"
  )]
  CheckIsOnCentralStack,
  #[strum(
    serialize = "kCountUnoptimizedWasmToJSWrapper",
    serialize = "CountUnoptimizedWasmToJSWrapper"
  )]
  CountUnoptimizedWasmToJSWrapper,
  #[strum(serialize = "kDisallowWasmCodegen", serialize = "DisallowWasmCodegen")]
  DisallowWasmCodegen,
  #[strum(
    serialize = "kEstimateCurrentMemoryConsumption",
    serialize = "EstimateCurrentMemoryConsumption"
  )]
  EstimateCurrentMemoryConsumption,
  #[strum(serialize = "kFlushLiftoffCode", serialize = "FlushLiftoffCode")]
  FlushLiftoffCode,
  #[strum(
    serialize = "kFreezeWasmLazyCompilation",
    serialize = "FreezeWasmLazyCompilation"
  )]
  FreezeWasmLazyCompilation,
  #[strum(
    serialize = "kGenerateWasmCompilationHints",
    serialize = "GenerateWasmCompilationHints"
  )]
  GenerateWasmCompilationHints,
  #[strum(
    serialize = "kGetWasmExceptionTagId",
    serialize = "GetWasmExceptionTagId"
  )]
  GetWasmExceptionTagId,
  #[strum(
    serialize = "kGetWasmExceptionValues",
    serialize = "GetWasmExceptionValues"
  )]
  GetWasmExceptionValues,
  #[strum(
    serialize = "kGetWasmRecoveredTrapCount",
    serialize = "GetWasmRecoveredTrapCount"
  )]
  GetWasmRecoveredTrapCount,
  #[strum(
    serialize = "kHasUnoptimizedWasmToJSWrapper",
    serialize = "HasUnoptimizedWasmToJSWrapper"
  )]
  HasUnoptimizedWasmToJSWrapper,
  #[strum(serialize = "kIsLiftoffFunction", serialize = "IsLiftoffFunction")]
  IsLiftoffFunction,
  #[strum(serialize = "kIsTurboFanFunction", serialize = "IsTurboFanFunction")]
  IsTurboFanFunction,
  #[strum(
    serialize = "kIsUncompiledWasmFunction",
    serialize = "IsUncompiledWasmFunction"
  )]
  IsUncompiledWasmFunction,
  #[strum(serialize = "kIsWasmCode", serialize = "IsWasmCode")]
  IsWasmCode,
  #[strum(serialize = "kIsWasmDebugFunction", serialize = "IsWasmDebugFunction")]
  IsWasmDebugFunction,
  #[strum(
    serialize = "kIsWasmPartialOOBWriteNoop",
    serialize = "IsWasmPartialOOBWriteNoop"
  )]
  IsWasmPartialOOBWriteNoop,
  #[strum(
    serialize = "kIsWasmTrapHandlerEnabled",
    serialize = "IsWasmTrapHandlerEnabled"
  )]
  IsWasmTrapHandlerEnabled,
  #[strum(
    serialize = "kSetWasmCompileControls",
    serialize = "SetWasmCompileControls"
  )]
  SetWasmCompileControls,
  #[strum(
    serialize = "kSetWasmInstantiateControls",
    serialize = "SetWasmInstantiateControls"
  )]
  SetWasmInstantiateControls,
  #[strum(serialize = "kWasmArray", serialize = "WasmArray")]
  WasmArray,
  #[strum(
    serialize = "kWasmCompiledExportWrappersCount",
    serialize = "WasmCompiledExportWrappersCount"
  )]
  WasmCompiledExportWrappersCount,
  #[strum(
    serialize = "kWasmDeoptsExecutedCount",
    serialize = "WasmDeoptsExecutedCount"
  )]
  WasmDeoptsExecutedCount,
  #[strum(
    serialize = "kWasmDeoptsExecutedForFunction",
    serialize = "WasmDeoptsExecutedForFunction"
  )]
  WasmDeoptsExecutedForFunction,
  #[strum(serialize = "kWasmEnterDebugging", serialize = "WasmEnterDebugging")]
  WasmEnterDebugging,
  #[strum(
    serialize = "kWasmGenerateRandomModule",
    serialize = "WasmGenerateRandomModule"
  )]
  WasmGenerateRandomModule,
  #[strum(
    serialize = "kWasmGetNumberOfInstances",
    serialize = "WasmGetNumberOfInstances"
  )]
  WasmGetNumberOfInstances,
  #[strum(serialize = "kWasmLeaveDebugging", serialize = "WasmLeaveDebugging")]
  WasmLeaveDebugging,
  #[strum(serialize = "kWasmNumCodeSpaces", serialize = "WasmNumCodeSpaces")]
  WasmNumCodeSpaces,
  #[strum(serialize = "kWasmStruct", serialize = "WasmStruct")]
  WasmStruct,
  #[strum(
    serialize = "kWasmSwitchToTheCentralStackCount",
    serialize = "WasmSwitchToTheCentralStackCount"
  )]
  WasmSwitchToTheCentralStackCount,
  #[strum(serialize = "kWasmTierUpFunction", serialize = "WasmTierUpFunction")]
  WasmTierUpFunction,
  #[strum(serialize = "kWasmTraceEnter", serialize = "WasmTraceEnter")]
  WasmTraceEnter,
  #[strum(serialize = "kWasmTraceExit", serialize = "WasmTraceExit")]
  WasmTraceExit,
  #[strum(serialize = "kWasmTraceMemory", serialize = "WasmTraceMemory")]
  WasmTraceMemory,
  #[strum(serialize = "kWasmTriggerCodeGC", serialize = "WasmTriggerCodeGC")]
  WasmTriggerCodeGC,
  #[strum(serialize = "kWasmTraceGlobal", serialize = "WasmTraceGlobal")]
  WasmTraceGlobal,
  #[strum(
    serialize = "kWasmTriggerTierUpForTesting",
    serialize = "WasmTriggerTierUpForTesting"
  )]
  WasmTriggerTierUpForTesting,
  #[strum(
    serialize = "kWasmTraceBeginExecution",
    serialize = "WasmTraceBeginExecution"
  )]
  WasmTraceBeginExecution,
  #[strum(
    serialize = "kWasmTraceEndExecution",
    serialize = "WasmTraceEndExecution"
  )]
  WasmTraceEndExecution,
  #[strum(
    serialize = "kJSFinalizationRegistryRegisterWeakCellWithUnregisterToken",
    serialize = "JSFinalizationRegistryRegisterWeakCellWithUnregisterToken"
  )]
  JSFinalizationRegistryRegisterWeakCellWithUnregisterToken,
  #[strum(
    serialize = "kJSWeakRefAddToKeptObjects",
    serialize = "JSWeakRefAddToKeptObjects"
  )]
  JSWeakRefAddToKeptObjects,
  #[strum(
    serialize = "kDebugBreakOnBytecode",
    serialize = "DebugBreakOnBytecode"
  )]
  DebugBreakOnBytecode,
  #[strum(
    serialize = "kLoadLookupSlotForCall",
    serialize = "LoadLookupSlotForCall"
  )]
  LoadLookupSlotForCall,
  #[strum(
    serialize = "kElementsTransitionAndStoreIC_Miss",
    serialize = "ElementsTransitionAndStoreIC_Miss",
    serialize = "ElementsTransitionAndStoreICMiss"
  )]
  ElementsTransitionAndStoreICMiss,
  #[strum(
    serialize = "kKeyedLoadIC_Miss",
    serialize = "KeyedLoadIC_Miss",
    serialize = "KeyedLoadICMiss"
  )]
  KeyedLoadICMiss,
  #[strum(
    serialize = "kKeyedStoreIC_Miss",
    serialize = "KeyedStoreIC_Miss",
    serialize = "KeyedStoreICMiss"
  )]
  KeyedStoreICMiss,
  #[strum(
    serialize = "kDefineKeyedOwnIC_Miss",
    serialize = "DefineKeyedOwnIC_Miss",
    serialize = "DefineKeyedOwnICMiss"
  )]
  DefineKeyedOwnICMiss,
  #[strum(
    serialize = "kStoreInArrayLiteralIC_Miss",
    serialize = "StoreInArrayLiteralIC_Miss",
    serialize = "StoreInArrayLiteralICMiss"
  )]
  StoreInArrayLiteralICMiss,
  #[strum(
    serialize = "kDefineNamedOwnIC_Slow",
    serialize = "DefineNamedOwnIC_Slow",
    serialize = "DefineNamedOwnICSlow"
  )]
  DefineNamedOwnICSlow,
  #[strum(
    serialize = "kKeyedStoreIC_Slow",
    serialize = "KeyedStoreIC_Slow",
    serialize = "KeyedStoreICSlow"
  )]
  KeyedStoreICSlow,
  #[strum(
    serialize = "kDefineKeyedOwnIC_Slow",
    serialize = "DefineKeyedOwnIC_Slow",
    serialize = "DefineKeyedOwnICSlow"
  )]
  DefineKeyedOwnICSlow,
  #[strum(
    serialize = "kLoadElementWithInterceptor",
    serialize = "LoadElementWithInterceptor"
  )]
  LoadElementWithInterceptor,
  #[strum(
    serialize = "kLoadGlobalIC_Miss",
    serialize = "LoadGlobalIC_Miss",
    serialize = "LoadGlobalICMiss"
  )]
  LoadGlobalICMiss,
  #[strum(
    serialize = "kLoadGlobalIC_Slow",
    serialize = "LoadGlobalIC_Slow",
    serialize = "LoadGlobalICSlow"
  )]
  LoadGlobalICSlow,
  #[strum(
    serialize = "kLoadIC_Miss",
    serialize = "LoadIC_Miss",
    serialize = "LoadICMiss"
  )]
  LoadICMiss,
  #[strum(
    serialize = "kLoadIC_Miss_FromBaseline",
    serialize = "LoadIC_Miss_FromBaseline",
    serialize = "LoadICMissFromBaseline"
  )]
  LoadICMissFromBaseline,
  #[strum(
    serialize = "kPatchLoadICUninitializedBaseline",
    serialize = "PatchLoadICUninitializedBaseline"
  )]
  PatchLoadICUninitializedBaseline,
  #[strum(
    serialize = "kGetStringLengthAndUpdateFeedback",
    serialize = "GetStringLengthAndUpdateFeedback"
  )]
  GetStringLengthAndUpdateFeedback,
  #[strum(
    serialize = "kLoadNoFeedbackIC_Miss",
    serialize = "LoadNoFeedbackIC_Miss",
    serialize = "LoadNoFeedbackICMiss"
  )]
  LoadNoFeedbackICMiss,
  #[strum(
    serialize = "kLoadWithReceiverIC_Miss",
    serialize = "LoadWithReceiverIC_Miss",
    serialize = "LoadWithReceiverICMiss"
  )]
  LoadWithReceiverICMiss,
  #[strum(
    serialize = "kLoadWithReceiverNoFeedbackIC_Miss",
    serialize = "LoadWithReceiverNoFeedbackIC_Miss",
    serialize = "LoadWithReceiverNoFeedbackICMiss"
  )]
  LoadWithReceiverNoFeedbackICMiss,
  #[strum(
    serialize = "kLoadPropertyPastInterceptor",
    serialize = "LoadPropertyPastInterceptor"
  )]
  LoadPropertyPastInterceptor,
  #[strum(
    serialize = "kStoreCallbackProperty",
    serialize = "StoreCallbackProperty"
  )]
  StoreCallbackProperty,
  #[strum(
    serialize = "kStoreGlobalIC_Miss",
    serialize = "StoreGlobalIC_Miss",
    serialize = "StoreGlobalICMiss"
  )]
  StoreGlobalICMiss,
  #[strum(
    serialize = "kStoreGlobalICNoFeedback_Miss",
    serialize = "StoreGlobalICNoFeedback_Miss",
    serialize = "StoreGlobalICNoFeedbackMiss"
  )]
  StoreGlobalICNoFeedbackMiss,
  #[strum(
    serialize = "kStoreGlobalIC_Slow",
    serialize = "StoreGlobalIC_Slow",
    serialize = "StoreGlobalICSlow"
  )]
  StoreGlobalICSlow,
  #[strum(
    serialize = "kStoreIC_Miss",
    serialize = "StoreIC_Miss",
    serialize = "StoreICMiss"
  )]
  StoreICMiss,
  #[strum(
    serialize = "kDefineNamedOwnIC_Miss",
    serialize = "DefineNamedOwnIC_Miss",
    serialize = "DefineNamedOwnICMiss"
  )]
  DefineNamedOwnICMiss,
  #[strum(
    serialize = "kStoreInArrayLiteralIC_Slow",
    serialize = "StoreInArrayLiteralIC_Slow",
    serialize = "StoreInArrayLiteralICSlow"
  )]
  StoreInArrayLiteralICSlow,
  #[strum(
    serialize = "kStorePropertyPastInterceptor",
    serialize = "StorePropertyPastInterceptor"
  )]
  StorePropertyPastInterceptor,
  #[strum(
    serialize = "kCloneObjectIC_Slow",
    serialize = "CloneObjectIC_Slow",
    serialize = "CloneObjectICSlow"
  )]
  CloneObjectICSlow,
  #[strum(
    serialize = "kCloneObjectIC_Miss",
    serialize = "CloneObjectIC_Miss",
    serialize = "CloneObjectICMiss"
  )]
  CloneObjectICMiss,
  #[strum(
    serialize = "kKeyedHasIC_Miss",
    serialize = "KeyedHasIC_Miss",
    serialize = "KeyedHasICMiss"
  )]
  KeyedHasICMiss,
  #[strum(
    serialize = "kHasElementWithInterceptor",
    serialize = "HasElementWithInterceptor"
  )]
  HasElementWithInterceptor,
  #[strum(
    serialize = "kObjectAssignTryFastcase",
    serialize = "ObjectAssignTryFastcase"
  )]
  ObjectAssignTryFastcase,
  #[strum(serialize = "kNumFunctions", serialize = "NumFunctions")]
  NumFunctions,
}

impl RuntimeFunctionId {
  pub const fn name(&self) -> &'static str {
    match self {
      Self::ArrayIncludesSlow => "ArrayIncludes_Slow",
      Self::ArrayIndexOf => "ArrayIndexOf",
      Self::ArrayIsArray => "ArrayIsArray",
      Self::ArraySpeciesConstructor => "ArraySpeciesConstructor",
      Self::GrowArrayElements => "GrowArrayElements",
      Self::IsArray => "IsArray",
      Self::NewArray => "NewArray",
      Self::NormalizeElements => "NormalizeElements",
      Self::TransitionElementsKind => "TransitionElementsKind",
      Self::TransitionElementsKindWithKind => "TransitionElementsKindWithKind",
      Self::AtomicsLoad64 => "AtomicsLoad64",
      Self::AtomicsStore64 => "AtomicsStore64",
      Self::AtomicsAdd => "AtomicsAdd",
      Self::AtomicsAnd => "AtomicsAnd",
      Self::AtomicsCompareExchange => "AtomicsCompareExchange",
      Self::AtomicsExchange => "AtomicsExchange",
      Self::AtomicsNumWaitersForTesting => "AtomicsNumWaitersForTesting",
      Self::AtomicsNumUnresolvedAsyncPromisesForTesting => {
        "AtomicsNumUnresolvedAsyncPromisesForTesting"
      }
      Self::AtomicsOr => "AtomicsOr",
      Self::AtomicsSub => "AtomicsSub",
      Self::AtomicsXor => "AtomicsXor",
      Self::SetAllowAtomicsWait => "SetAllowAtomicsWait",
      Self::AtomicsLoadSharedStructOrArray => "AtomicsLoadSharedStructOrArray",
      Self::AtomicsStoreSharedStructOrArray => "AtomicsStoreSharedStructOrArray",
      Self::AtomicsExchangeSharedStructOrArray => "AtomicsExchangeSharedStructOrArray",
      Self::AtomicsCompareExchangeSharedStructOrArray => {
        "AtomicsCompareExchangeSharedStructOrArray"
      }
      Self::AtomicsSynchronizationPrimitiveNumWaitersForTesting => {
        "AtomicsSynchronizationPrimitiveNumWaitersForTesting"
      }
      Self::BigIntCompareToNumber => "BigIntCompareToNumber",
      Self::BigIntCompareToString => "BigIntCompareToString",
      Self::BigIntEqualToBigInt => "BigIntEqualToBigInt",
      Self::BigIntEqualToNumber => "BigIntEqualToNumber",
      Self::BigIntEqualToString => "BigIntEqualToString",
      Self::BigIntExponentiate => "BigIntExponentiate",
      Self::BigIntMaxBits => "BigIntMaxBits",
      Self::BigIntToNumber => "BigIntToNumber",
      Self::BigIntUnaryOp => "BigIntUnaryOp",
      Self::ToBigInt => "ToBigInt",
      Self::ToBigIntConvertNumber => "ToBigIntConvertNumber",
      Self::ThrowConstructorNonCallableError => "ThrowConstructorNonCallableError",
      Self::ThrowNotSuperConstructor => "ThrowNotSuperConstructor",
      Self::ThrowStaticPrototypeError => "ThrowStaticPrototypeError",
      Self::ThrowSuperAlreadyCalledError => "ThrowSuperAlreadyCalledError",
      Self::ThrowSuperNotCalled => "ThrowSuperNotCalled",
      Self::ThrowUnsupportedSuperError => "ThrowUnsupportedSuperError",
      Self::DefineClass => "DefineClass",
      Self::LoadFromSuper => "LoadFromSuper",
      Self::LoadKeyedFromSuper => "LoadKeyedFromSuper",
      Self::StoreKeyedToSuper => "StoreKeyedToSuper",
      Self::StoreToSuper => "StoreToSuper",
      Self::MapGrow => "MapGrow",
      Self::MapShrink => "MapShrink",
      Self::OrderedHashSetGrow => "OrderedHashSetGrow",
      Self::SetGrow => "SetGrow",
      Self::SetShrink => "SetShrink",
      Self::OrderedHashSetShrink => "OrderedHashSetShrink",
      Self::TheHole => "TheHole",
      Self::WeakCollectionDelete => "WeakCollectionDelete",
      Self::WeakCollectionSet => "WeakCollectionSet",
      Self::OrderedHashMapGrow => "OrderedHashMapGrow",
      Self::PrintDumpedFrame => "PrintDumpedFrame",
      Self::CompileOptimizedOSR => "CompileOptimizedOSR",
      Self::CompileOptimizedOSRFromMaglev => "CompileOptimizedOSRFromMaglev",
      Self::CompileOptimizedOSRFromMaglevInlined => "CompileOptimizedOSRFromMaglevInlined",
      Self::LogOrTraceOptimizedOSREntry => "LogOrTraceOptimizedOSREntry",
      Self::CompileLazy => "CompileLazy",
      Self::CompileBaseline => "CompileBaseline",
      Self::InstallBaselineCode => "InstallBaselineCode",
      Self::InstallSFICode => "InstallSFICode",
      Self::NotifyDeoptimized => "NotifyDeoptimized",
      Self::ObserveNode => "ObserveNode",
      Self::ResolvePossiblyDirectEval => "ResolvePossiblyDirectEval",
      Self::VerifyType => "VerifyType",
      Self::CheckTurboshaftTypeOf => "CheckTurboshaftTypeOf",
      Self::PatchCompareOpBaselineCode => "PatchCompareOpBaselineCode",
      Self::PatchCompareOpBaselineCodeAndThrow => "PatchCompareOpBaselineCodeAndThrow",
      Self::PatchBinopBaselineCode => "PatchBinopBaselineCode",
      Self::PatchBinopBaselineCodeAndThrow => "PatchBinopBaselineCodeAndThrow",
      Self::PatchUnaryOpBaselineCode => "PatchUnaryOpBaselineCode",
      Self::PatchUnaryOpBaselineCodeAndThrow => "PatchUnaryOpBaselineCodeAndThrow",
      Self::FunctionLogNextExecution => "FunctionLogNextExecution",
      Self::OptimizeMaglevEager => "OptimizeMaglevEager",
      Self::StartMaglevOptimizeJob => "StartMaglevOptimizeJob",
      Self::OptimizeTurbofanEager => "OptimizeTurbofanEager",
      Self::StartTurbofanOptimizeJob => "StartTurbofanOptimizeJob",
      Self::MarkLazyDeoptimizedOrFlushed => "MarkLazyDeoptimizedOrFlushed",
      Self::DateCurrentTime => "DateCurrentTime",
      Self::ClearStepping => "ClearStepping",
      Self::CollectGarbage => "CollectGarbage",
      Self::DebugAsyncFunctionSuspended => "DebugAsyncFunctionSuspended",
      Self::DebugBreakAtEntry => "DebugBreakAtEntry",
      Self::DebugCollectCoverage => "DebugCollectCoverage",
      Self::DebugGetLoadedScriptIds => "DebugGetLoadedScriptIds",
      Self::DebugOnFunctionCall => "DebugOnFunctionCall",
      Self::DebugPrepareStepInSuspendedGenerator => "DebugPrepareStepInSuspendedGenerator",
      Self::DebugPromiseThen => "DebugPromiseThen",
      Self::DebugToggleBlockCoverage => "DebugToggleBlockCoverage",
      Self::DebugTogglePreciseCoverage => "DebugTogglePreciseCoverage",
      Self::FunctionGetInferredName => "FunctionGetInferredName",
      Self::GetBreakLocations => "GetBreakLocations",
      Self::GetGeneratorScopeCount => "GetGeneratorScopeCount",
      Self::GetGeneratorScopeDetails => "GetGeneratorScopeDetails",
      Self::HandleDebuggerStatement => "HandleDebuggerStatement",
      Self::IsBreakOnException => "IsBreakOnException",
      Self::IterableForEach => "IterableForEach",
      Self::ProfileCreateSnapshotDataBlob => "ProfileCreateSnapshotDataBlob",
      Self::ScheduleBreak => "ScheduleBreak",
      Self::ScriptLocationFromLine2 => "ScriptLocationFromLine2",
      Self::SetGeneratorScopeVariableValue => "SetGeneratorScopeVariableValue",
      Self::IncBlockCounter => "IncBlockCounter",
      Self::ForInEnumerate => "ForInEnumerate",
      Self::ForInHasProperty => "ForInHasProperty",
      Self::TraceUnoptimizedBytecodeEntry => "TraceUnoptimizedBytecodeEntry",
      Self::TraceUnoptimizedBytecodeExit => "TraceUnoptimizedBytecodeExit",
      Self::DumpExecutionFrame => "DumpExecutionFrame",
      Self::TraceUpdateFeedback => "TraceUpdateFeedback",
      Self::Call => "Call",
      Self::FunctionGetScriptSource => "FunctionGetScriptSource",
      Self::FunctionGetScriptId => "FunctionGetScriptId",
      Self::FunctionGetScriptSourcePosition => "FunctionGetScriptSourcePosition",
      Self::FunctionGetSourceCode => "FunctionGetSourceCode",
      Self::FunctionIsAPIFunction => "FunctionIsAPIFunction",
      Self::AsyncFunctionAwait => "AsyncFunctionAwait",
      Self::AsyncFunctionEnter => "AsyncFunctionEnter",
      Self::AsyncFunctionReject => "AsyncFunctionReject",
      Self::AsyncFunctionResolve => "AsyncFunctionResolve",
      Self::AsyncGeneratorAwait => "AsyncGeneratorAwait",
      Self::AsyncGeneratorReject => "AsyncGeneratorReject",
      Self::AsyncGeneratorResolve => "AsyncGeneratorResolve",
      Self::AsyncGeneratorYieldWithAwait => "AsyncGeneratorYieldWithAwait",
      Self::CreateJSGeneratorObject => "CreateJSGeneratorObject",
      Self::GeneratorClose => "GeneratorClose",
      Self::GeneratorGetFunction => "GeneratorGetFunction",
      Self::GeneratorGetResumeMode => "GeneratorGetResumeMode",
      Self::FormatList => "FormatList",
      Self::FormatListToParts => "FormatListToParts",
      Self::StringToLowerCaseIntl => "StringToLowerCaseIntl",
      Self::StringToLocaleLowerCase => "StringToLocaleLowerCase",
      Self::StringToUpperCaseIntl => "StringToUpperCaseIntl",
      Self::Throw => "Throw",
      Self::ThrowTargetNonFunction => "ThrowTargetNonFunction",
      Self::ThrowCalledNonCallable => "ThrowCalledNonCallable",
      Self::ThrowConstructedNonConstructable => "ThrowConstructedNonConstructable",
      Self::ThrowConstructorReturnedNonObject => "ThrowConstructorReturnedNonObject",
      Self::ThrowInvalidStringLength => "ThrowInvalidStringLength",
      Self::ThrowInvalidTypedArrayAlignment => "ThrowInvalidTypedArrayAlignment",
      Self::ThrowIteratorError => "ThrowIteratorError",
      Self::ThrowSpreadArgError => "ThrowSpreadArgError",
      Self::ThrowIteratorResultNotAnObject => "ThrowIteratorResultNotAnObject",
      Self::ThrowNoAccess => "ThrowNoAccess",
      Self::ThrowNotConstructor => "ThrowNotConstructor",
      Self::ThrowPatternAssignmentNonCoercible => "ThrowPatternAssignmentNonCoercible",
      Self::ThrowRangeError => "ThrowRangeError",
      Self::ThrowReferenceError => "ThrowReferenceError",
      Self::ThrowAccessedUninitializedVariable => "ThrowAccessedUninitializedVariable",
      Self::VarargStackOverflow => "VarargStackOverflow",
      Self::ThrowStackOverflow => "ThrowStackOverflow",
      Self::ThrowSymbolAsyncIteratorInvalid => "ThrowSymbolAsyncIteratorInvalid",
      Self::ThrowSymbolIteratorInvalid => "ThrowSymbolIteratorInvalid",
      Self::ThrowThrowMethodMissing => "ThrowThrowMethodMissing",
      Self::ThrowTypeError => "ThrowTypeError",
      Self::ThrowTypeErrorIfStrict => "ThrowTypeErrorIfStrict",
      Self::ReThrow => "ReThrow",
      Self::ReThrowWithMessage => "ReThrowWithMessage",
      Self::AccessCheck => "AccessCheck",
      Self::AllocateByteArray => "AllocateByteArray",
      Self::AllocateInYoungGeneration => "AllocateInYoungGeneration",
      Self::AllocateInOldGeneration => "AllocateInOldGeneration",
      Self::AllocateInSharedHeap => "AllocateInSharedHeap",
      Self::AllowDynamicFunction => "AllowDynamicFunction",
      Self::CreateAsyncFromSyncIterator => "CreateAsyncFromSyncIterator",
      Self::CreateListFromArrayLike => "CreateListFromArrayLike",
      Self::DoubleToStringWithRadix => "DoubleToStringWithRadix",
      Self::FatalProcessOutOfMemoryInAllocateRaw => "FatalProcessOutOfMemoryInAllocateRaw",
      Self::FatalProcessOutOfMemoryInvalidArrayLength => {
        "FatalProcessOutOfMemoryInvalidArrayLength"
      }
      Self::FatalInvalidSize => "FatalInvalidSize",
      Self::GetAndResetRuntimeCallStats => "GetAndResetRuntimeCallStats",
      Self::GetAndResetTurboProfilingData => "GetAndResetTurboProfilingData",
      Self::GetTemplateObject => "GetTemplateObject",
      Self::IncrementUseCounter => "IncrementUseCounter",
      Self::InvalidateStringWrapperToPrimitiveProtector => {
        "InvalidateStringWrapperToPrimitiveProtector"
      }
      Self::BytecodeBudgetInterruptIgnition => "BytecodeBudgetInterrupt_Ignition",
      Self::BytecodeBudgetInterruptWithStackCheckIgnition => {
        "BytecodeBudgetInterruptWithStackCheck_Ignition"
      }
      Self::BytecodeBudgetInterruptSparkplug => "BytecodeBudgetInterrupt_Sparkplug",
      Self::BytecodeBudgetInterruptWithStackCheckSparkplug => {
        "BytecodeBudgetInterruptWithStackCheck_Sparkplug"
      }
      Self::BytecodeBudgetInterruptMaglev => "BytecodeBudgetInterrupt_Maglev",
      Self::BytecodeBudgetLoopInterruptMaglev => "BytecodeBudgetLoopInterrupt_Maglev",
      Self::NotifyContextCellStateWillChange => "NotifyContextCellStateWillChange",
      Self::NewError => "NewError",
      Self::NewReferenceError => "NewReferenceError",
      Self::NewTypeError => "NewTypeError",
      Self::OrdinaryHasInstance => "OrdinaryHasInstance",
      Self::PropagateException => "PropagateException",
      Self::ReportMessageFromMicrotask => "ReportMessageFromMicrotask",
      Self::RunMicrotaskCallback => "RunMicrotaskCallback",
      Self::PerformMicrotaskCheckpoint => "PerformMicrotaskCheckpoint",
      Self::SharedValueBarrierSlow => "SharedValueBarrierSlow",
      Self::StackGuard => "StackGuard",
      Self::HandleNoHeapWritesInterrupts => "HandleNoHeapWritesInterrupts",
      Self::StackGuardWithGap => "StackGuardWithGap",
      Self::TerminateExecution => "TerminateExecution",
      Self::Typeof => "Typeof",
      Self::UnwindAndFindExceptionHandler => "UnwindAndFindExceptionHandler",
      Self::AddLhsIsStringConstantInternalize => "AddLhsIsStringConstantInternalize",
      Self::AddRhsIsStringConstantInternalize => "AddRhsIsStringConstantInternalize",
      Self::CreateArrayLiteral => "CreateArrayLiteral",
      Self::CreateObjectLiteral => "CreateObjectLiteral",
      Self::SetPrototypeProperties => "SetPrototypeProperties",
      Self::CreateRegExpLiteral => "CreateRegExpLiteral",
      Self::DynamicImportCall => "DynamicImportCall",
      Self::GetImportMetaObject => "GetImportMetaObject",
      Self::GetModuleNamespace => "GetModuleNamespace",
      Self::GetModuleNamespaceExport => "GetModuleNamespaceExport",
      Self::ArrayBufferMaxByteLength => "ArrayBufferMaxByteLength",
      Self::IsSmi => "IsSmi",
      Self::MaxSmi => "MaxSmi",
      Self::NumberToStringSlow => "NumberToStringSlow",
      Self::Float64ToStringSlow => "Float64ToStringSlow",
      Self::StringParseFloat => "StringParseFloat",
      Self::StringParseInt => "StringParseInt",
      Self::StringToNumber => "StringToNumber",
      Self::AddAsyncDisposableValue => "AddAsyncDisposableValue",
      Self::AddDictionaryProperty => "AddDictionaryProperty",
      Self::AddDisposableValue => "AddDisposableValue",
      Self::AddPrivateBrand => "AddPrivateBrand",
      Self::AllocateHeapNumber => "AllocateHeapNumber",
      Self::CompleteInobjectSlackTrackingForMap => "CompleteInobjectSlackTrackingForMap",
      Self::CopyDataProperties => "CopyDataProperties",
      Self::CopyDataPropertiesWithExcludedPropertiesOnStack => {
        "CopyDataPropertiesWithExcludedPropertiesOnStack"
      }
      Self::CreateDataProperty => "CreateDataProperty",
      Self::CreateIterResultObject => "CreateIterResultObject",
      Self::IterableToListWithInterceptor => "IterableToListWithInterceptor",
      Self::CheckFastIterableToListPrototype => "CheckFastIterableToListPrototype",
      Self::CreatePrivateAccessors => "CreatePrivateAccessors",
      Self::DefineAccessorPropertyUnchecked => "DefineAccessorPropertyUnchecked",
      Self::DefineKeyedOwnPropertyInLiteral => "DefineKeyedOwnPropertyInLiteral",
      Self::DefineGetterPropertyUnchecked => "DefineGetterPropertyUnchecked",
      Self::DefineSetterPropertyUnchecked => "DefineSetterPropertyUnchecked",
      Self::DeleteProperty => "DeleteProperty",
      Self::DisposeDisposableStack => "DisposeDisposableStack",
      Self::GeneratorYieldResult => "GeneratorYieldResult",
      Self::GetDerivedMap => "GetDerivedMap",
      Self::GetFunctionName => "GetFunctionName",
      Self::GetOwnPropertyDescriptorObject => "GetOwnPropertyDescriptorObject",
      Self::GetOwnPropertyKeys => "GetOwnPropertyKeys",
      Self::GetPrivateMember => "GetPrivateMember",
      Self::GetProperty => "GetProperty",
      Self::HandleExceptionsInDisposeDisposableStack => "HandleExceptionsInDisposeDisposableStack",
      Self::HasFastPackedElements => "HasFastPackedElements",
      Self::HasInPrototypeChain => "HasInPrototypeChain",
      Self::HasProperty => "HasProperty",
      Self::InitializeDisposableStack => "InitializeDisposableStack",
      Self::InternalSetPrototype => "InternalSetPrototype",
      Self::IsJSReceiver => "IsJSReceiver",
      Self::JSReceiverPreventExtensionsDontThrow => "JSReceiverPreventExtensionsDontThrow",
      Self::JSReceiverPreventExtensionsThrow => "JSReceiverPreventExtensionsThrow",
      Self::JSReceiverGetPrototypeOf => "JSReceiverGetPrototypeOf",
      Self::JSReceiverSetPrototypeOfDontThrow => "JSReceiverSetPrototypeOfDontThrow",
      Self::JSReceiverSetPrototypeOfThrow => "JSReceiverSetPrototypeOfThrow",
      Self::LoadPrivateGetter => "LoadPrivateGetter",
      Self::LoadPrivateSetter => "LoadPrivateSetter",
      Self::NewObject => "NewObject",
      Self::ObjectCreate => "ObjectCreate",
      Self::ObjectEntries => "ObjectEntries",
      Self::ObjectEntriesSkipFastPath => "ObjectEntriesSkipFastPath",
      Self::ObjectGetOwnPropertyNames => "ObjectGetOwnPropertyNames",
      Self::ObjectGetOwnPropertyNamesTryFast => "ObjectGetOwnPropertyNamesTryFast",
      Self::ObjectHasOwnProperty => "ObjectHasOwnProperty",
      Self::HasOwnConstDataProperty => "HasOwnConstDataProperty",
      Self::ObjectIsExtensible => "ObjectIsExtensible",
      Self::ObjectKeys => "ObjectKeys",
      Self::ObjectValues => "ObjectValues",
      Self::ObjectValuesSkipFastPath => "ObjectValuesSkipFastPath",
      Self::OptimizeObjectForAddingMultipleProperties => {
        "OptimizeObjectForAddingMultipleProperties"
      }
      Self::SetDataProperties => "SetDataProperties",
      Self::SetFunctionName => "SetFunctionName",
      Self::SetKeyedProperty => "SetKeyedProperty",
      Self::DefineObjectOwnProperty => "DefineObjectOwnProperty",
      Self::SetNamedProperty => "SetNamedProperty",
      Self::SetOwnPropertyIgnoreAttributes => "SetOwnPropertyIgnoreAttributes",
      Self::ShrinkNameDictionary => "ShrinkNameDictionary",
      Self::ShrinkSwissNameDictionary => "ShrinkSwissNameDictionary",
      Self::ToFastProperties => "ToFastProperties",
      Self::ToLength => "ToLength",
      Self::ToName => "ToName",
      Self::ToNumber => "ToNumber",
      Self::ToNumeric => "ToNumeric",
      Self::ToObject => "ToObject",
      Self::ToString => "ToString",
      Self::TryMigrateInstance => "TryMigrateInstance",
      Self::TryMigrateInstanceAndMarkMapAsMigrationTarget => {
        "TryMigrateInstanceAndMarkMapAsMigrationTarget"
      }
      Self::SetPrivateMember => "SetPrivateMember",
      Self::SwissTableAdd => "SwissTableAdd",
      Self::SwissTableAllocate => "SwissTableAllocate",
      Self::SwissTableDelete => "SwissTableDelete",
      Self::SwissTableDetailsAt => "SwissTableDetailsAt",
      Self::SwissTableElementsCount => "SwissTableElementsCount",
      Self::SwissTableEquals => "SwissTableEquals",
      Self::SwissTableFindEntry => "SwissTableFindEntry",
      Self::SwissTableUpdate => "SwissTableUpdate",
      Self::SwissTableValueAt => "SwissTableValueAt",
      Self::SwissTableKeyAt => "SwissTableKeyAt",
      Self::Add => "Add",
      Self::Equal => "Equal",
      Self::GreaterThan => "GreaterThan",
      Self::GreaterThanOrEqual => "GreaterThanOrEqual",
      Self::LessThan => "LessThan",
      Self::LessThanOrEqual => "LessThanOrEqual",
      Self::NotEqual => "NotEqual",
      Self::StrictEqual => "StrictEqual",
      Self::StrictNotEqual => "StrictNotEqual",
      Self::ReferenceEqual => "ReferenceEqual",
      Self::EnqueueMicrotask => "EnqueueMicrotask",
      Self::PromiseHookAfter => "PromiseHookAfter",
      Self::PromiseHookBefore => "PromiseHookBefore",
      Self::PromiseHookInit => "PromiseHookInit",
      Self::PromiseRejectEventFromStack => "PromiseRejectEventFromStack",
      Self::PromiseRevokeReject => "PromiseRevokeReject",
      Self::RejectPromise => "RejectPromise",
      Self::ResolvePromise => "ResolvePromise",
      Self::ConstructSuppressedError => "ConstructSuppressedError",
      Self::ConstructAggregateErrorHelper => "ConstructAggregateErrorHelper",
      Self::ConstructInternalAggregateErrorHelper => "ConstructInternalAggregateErrorHelper",
      Self::CheckProxyGetSetTrapResult => "CheckProxyGetSetTrapResult",
      Self::CheckProxyHasTrapResult => "CheckProxyHasTrapResult",
      Self::CheckProxyDeleteTrapResult => "CheckProxyDeleteTrapResult",
      Self::GetPropertyWithReceiver => "GetPropertyWithReceiver",
      Self::IsJSProxy => "IsJSProxy",
      Self::JSProxyGetHandler => "JSProxyGetHandler",
      Self::JSProxyGetTarget => "JSProxyGetTarget",
      Self::SetPropertyWithReceiver => "SetPropertyWithReceiver",
      Self::RegExpBuildIndices => "RegExpBuildIndices",
      Self::RegExpGrowRegExpMatchInfo => "RegExpGrowRegExpMatchInfo",
      Self::RegExpExecMultiple => "RegExpExecMultiple",
      Self::RegExpInitializeAndCompile => "RegExpInitializeAndCompile",
      Self::RegExpMatchGlobalAtom => "RegExpMatchGlobalAtom",
      Self::RegExpReplaceRT => "RegExpReplaceRT",
      Self::RegExpSplit => "RegExpSplit",
      Self::RegExpStringFromFlags => "RegExpStringFromFlags",
      Self::StringReplaceNonGlobalRegExpWithFunction => "StringReplaceNonGlobalRegExpWithFunction",
      Self::StringSplit => "StringSplit",
      Self::RegExpExec => "RegExpExec",
      Self::RegExpExperimentalOneshotExec => "RegExpExperimentalOneshotExec",
      Self::ThrowConstAssignError => "ThrowConstAssignError",
      Self::ThrowUsingAssignError => "ThrowUsingAssignError",
      Self::ThrowAwaitUsingAssignError => "ThrowAwaitUsingAssignError",
      Self::DeclareEvalFunction => "DeclareEvalFunction",
      Self::DeclareEvalVar => "DeclareEvalVar",
      Self::DeclareGlobals => "DeclareGlobals",
      Self::DeclareModuleExports => "DeclareModuleExports",
      Self::DeleteLookupSlot => "DeleteLookupSlot",
      Self::LoadLookupSlot => "LoadLookupSlot",
      Self::LoadLookupSlotInsideTypeof => "LoadLookupSlotInsideTypeof",
      Self::LoadLookupSlotForCallBaseline => "LoadLookupSlotForCall_Baseline",
      Self::NewClosure => "NewClosure",
      Self::NewClosureTenured => "NewClosure_Tenured",
      Self::NewFunctionContext => "NewFunctionContext",
      Self::NewRestParameter => "NewRestParameter",
      Self::NewSloppyArguments => "NewSloppyArguments",
      Self::NewStrictArguments => "NewStrictArguments",
      Self::PushBlockContext => "PushBlockContext",
      Self::PushCatchContext => "PushCatchContext",
      Self::PushWithContext => "PushWithContext",
      Self::StoreGlobalNoHoleCheckForReplLetOrConst => "StoreGlobalNoHoleCheckForReplLetOrConst",
      Self::StoreLookupSlotSloppy => "StoreLookupSlot_Sloppy",
      Self::StoreLookupSlotSloppyHoisting => "StoreLookupSlot_SloppyHoisting",
      Self::StoreLookupSlotStrict => "StoreLookupSlot_Strict",
      Self::ShadowRealmWrappedFunctionCreate => "ShadowRealmWrappedFunctionCreate",
      Self::ShadowRealmImportValue => "ShadowRealmImportValue",
      Self::ShadowRealmThrow => "ShadowRealmThrow",
      Self::FlattenString => "FlattenString",
      Self::GetSubstitution => "GetSubstitution",
      Self::InternalizeString => "InternalizeString",
      Self::StringAdd => "StringAdd",
      Self::StringAddLhsIsStringConstantInternalize => "StringAdd_LhsIsStringConstant_Internalize",
      Self::StringAddRhsIsStringConstantInternalize => "StringAdd_RhsIsStringConstant_Internalize",
      Self::StringBuilderConcat => "StringBuilderConcat",
      Self::StringCharCodeAt => "StringCharCodeAt",
      Self::StringCodePointAt => "StringCodePointAt",
      Self::StringCompare => "StringCompare",
      Self::StringEqual => "StringEqual",
      Self::StringEscapeQuotes => "StringEscapeQuotes",
      Self::StringGreaterThan => "StringGreaterThan",
      Self::StringGreaterThanOrEqual => "StringGreaterThanOrEqual",
      Self::StringIsWellFormed => "StringIsWellFormed",
      Self::StringLastIndexOf => "StringLastIndexOf",
      Self::StringLessThan => "StringLessThan",
      Self::StringLessThanOrEqual => "StringLessThanOrEqual",
      Self::StringMaxLength => "StringMaxLength",
      Self::StringReplaceOneCharWithString => "StringReplaceOneCharWithString",
      Self::StringSubstring => "StringSubstring",
      Self::StringToArray => "StringToArray",
      Self::StringToWellFormed => "StringToWellFormed",
      Self::CreatePrivateNameSymbol => "CreatePrivateNameSymbol",
      Self::CreatePrivateBrandSymbol => "CreatePrivateBrandSymbol",
      Self::CreatePrivateSymbol => "CreatePrivateSymbol",
      Self::SymbolDescriptiveString => "SymbolDescriptiveString",
      Self::SymbolIsPrivate => "SymbolIsPrivate",
      Self::Abort => "Abort",
      Self::AbortCSADcheck => "AbortCSADcheck",
      Self::AbortJS => "AbortJS",
      Self::ActiveTierIsIgnition => "ActiveTierIsIgnition",
      Self::ActiveTierIsSparkplug => "ActiveTierIsSparkplug",
      Self::ActiveTierIsMaglev => "ActiveTierIsMaglev",
      Self::ActiveTierIsTurbofan => "ActiveTierIsTurbofan",
      Self::AllocateHeapNumberWithValue => "AllocateHeapNumberWithValue",
      Self::ArrayBufferDetachForceWasm => "ArrayBufferDetachForceWasm",
      Self::ArrayIteratorProtector => "ArrayIteratorProtector",
      Self::ArraySpeciesProtector => "ArraySpeciesProtector",
      Self::AssertNotPeeled => "AssertNotPeeled",
      Self::AssertPeeled => "AssertPeeled",
      Self::BaselineOsr => "BaselineOsr",
      Self::BenchMaglev => "BenchMaglev",
      Self::BenchTurbofan => "BenchTurbofan",
      Self::BlockAt => "BlockAt",
      Self::VerifyGetJSBuiltinState => "VerifyGetJSBuiltinState",
      Self::ClearFunctionFeedback => "ClearFunctionFeedback",
      Self::ClearMegamorphicStubCache => "ClearMegamorphicStubCache",
      Self::CompleteInobjectSlackTracking => "CompleteInobjectSlackTracking",
      Self::ConstructConsString => "ConstructConsString",
      Self::ConstructDouble => "ConstructDouble",
      Self::ConstructInternalizedString => "ConstructInternalizedString",
      Self::ConstructSlicedString => "ConstructSlicedString",
      Self::ConstructThinString => "ConstructThinString",
      Self::CurrentFrameIsTurbofan => "CurrentFrameIsTurbofan",
      Self::DebugPrint => "DebugPrint",
      Self::DebugPrintCppHeapPointerTable => "DebugPrintCppHeapPointerTable",
      Self::DebugPrintCppHeapPointerTableFilterTag => "DebugPrintCppHeapPointerTableFilterTag",
      Self::DebugPrintExternalPointerTable => "DebugPrintExternalPointerTable",
      Self::DebugPrintExternalPointerTableFilterTag => "DebugPrintExternalPointerTableFilterTag",
      Self::DebugPrintGeneric => "DebugPrintGeneric",
      Self::DebugPrintFloat => "DebugPrintFloat",
      Self::DebugPrintPtr => "DebugPrintPtr",
      Self::DebugPrintWord => "DebugPrintWord",
      Self::DebugTrace => "DebugTrace",
      Self::DebugTraceMinimal => "DebugTraceMinimal",
      Self::DeoptimizeFunction => "DeoptimizeFunction",
      Self::DisableOptimizationFinalization => "DisableOptimizationFinalization",
      Self::DisallowCodegenFromStrings => "DisallowCodegenFromStrings",
      Self::DisassembleFunction => "DisassembleFunction",
      Self::EnableCodeLoggingForTesting => "EnableCodeLoggingForTesting",
      Self::EnsureFeedbackVectorForFunction => "EnsureFeedbackVectorForFunction",
      Self::FinalizeOptimization => "FinalizeOptimization",
      Self::ForceFlush => "ForceFlush",
      Self::MajorGCForCompilerTesting => "MajorGCForCompilerTesting",
      Self::GetAbstractModuleSource => "GetAbstractModuleSource",
      Self::GetBytecode => "GetBytecode",
      Self::ExhaustInterruptBudget => "ExhaustInterruptBudget",
      Self::GetCallable => "GetCallable",
      Self::GetFeedback => "GetFeedback",
      Self::GetFunctionForCurrentFrame => "GetFunctionForCurrentFrame",
      Self::GetInitializerFunction => "GetInitializerFunction",
      Self::GetOptimizationStatus => "GetOptimizationStatus",
      Self::GetUndetectable => "GetUndetectable",
      Self::GetWeakCollectionSize => "GetWeakCollectionSize",
      Self::GlobalPrint => "GlobalPrint",
      Self::HasCowElements => "HasCowElements",
      Self::HasDictionaryElements => "HasDictionaryElements",
      Self::HasDoubleElements => "HasDoubleElements",
      Self::HasElementsInALargeObjectSpace => "HasElementsInALargeObjectSpace",
      Self::HasFastElements => "HasFastElements",
      Self::HasFastProperties => "HasFastProperties",
      Self::HasFixedBigInt64Elements => "HasFixedBigInt64Elements",
      Self::HasFixedBigUint64Elements => "HasFixedBigUint64Elements",
      Self::HasFixedFloat16Elements => "HasFixedFloat16Elements",
      Self::HasFixedFloat32Elements => "HasFixedFloat32Elements",
      Self::HasFixedFloat64Elements => "HasFixedFloat64Elements",
      Self::HasFixedInt16Elements => "HasFixedInt16Elements",
      Self::HasFixedInt32Elements => "HasFixedInt32Elements",
      Self::HasFixedInt8Elements => "HasFixedInt8Elements",
      Self::HasFixedUint16Elements => "HasFixedUint16Elements",
      Self::HasFixedUint32Elements => "HasFixedUint32Elements",
      Self::HasFixedUint8ClampedElements => "HasFixedUint8ClampedElements",
      Self::HasFixedUint8Elements => "HasFixedUint8Elements",
      Self::HasHoleyElements => "HasHoleyElements",
      Self::HasObjectElements => "HasObjectElements",
      Self::HasPackedElements => "HasPackedElements",
      Self::HasSloppyArgumentsElements => "HasSloppyArgumentsElements",
      Self::HasSmiElements => "HasSmiElements",
      Self::HasSmiOrObjectElements => "HasSmiOrObjectElements",
      Self::HaveSameMap => "HaveSameMap",
      Self::HeapObjectVerify => "HeapObjectVerify",
      Self::ICsAreEnabled => "ICsAreEnabled",
      Self::InLargeObjectSpace => "InLargeObjectSpace",
      Self::InstallBytecode => "InstallBytecode",
      Self::InYoungGeneration => "InYoungGeneration",
      Self::Is64Bit => "Is64Bit",
      Self::IsAtomicsWaitAllowed => "IsAtomicsWaitAllowed",
      Self::IsBeingInterpreted => "IsBeingInterpreted",
      Self::IsConcatSpreadableProtector => "IsConcatSpreadableProtector",
      Self::IsConcurrentRecompilationSupported => "IsConcurrentRecompilationSupported",
      Self::IsDictPropertyConstTrackingEnabled => "IsDictPropertyConstTrackingEnabled",
      Self::IsEfficiencyModeEnabled => "IsEfficiencyModeEnabled",
      Self::IsInPlaceInternalizableString => "IsInPlaceInternalizableString",
      Self::IsInternalizedString => "IsInternalizedString",
      Self::StringToCString => "StringToCString",
      Self::StringUtf8Value => "StringUtf8Value",
      Self::IsUndefinedDoubleEnabled => "IsUndefinedDoubleEnabled",
      Self::IsMaglevEnabled => "IsMaglevEnabled",
      Self::IsSameHeapObject => "IsSameHeapObject",
      Self::IsSharedString => "IsSharedString",
      Self::IsInWritableSharedSpace => "IsInWritableSharedSpace",
      Self::IsSparkplugEnabled => "IsSparkplugEnabled",
      Self::IsTurbofanEnabled => "IsTurbofanEnabled",
      Self::IsWasmTieringPredictable => "IsWasmTieringPredictable",
      Self::MapIteratorProtector => "MapIteratorProtector",
      Self::NeverOptimizeFunction => "NeverOptimizeFunction",
      Self::NewRegExpWithBacktrackLimit => "NewRegExpWithBacktrackLimit",
      Self::NoElementsProtector => "NoElementsProtector",
      Self::NotifyContextDisposed => "NotifyContextDisposed",
      Self::SetPriorityBestEffort => "SetPriorityBestEffort",
      Self::SetPriorityUserVisible => "SetPriorityUserVisible",
      Self::SetPriorityUserBlocking => "SetPriorityUserBlocking",
      Self::OptimizeMaglevOnNextCall => "OptimizeMaglevOnNextCall",
      Self::OptimizeFunctionOnNextCall => "OptimizeFunctionOnNextCall",
      Self::OptimizeOsr => "OptimizeOsr",
      Self::PrepareFunctionForOptimization => "PrepareFunctionForOptimization",
      Self::PretenureAllocationSite => "PretenureAllocationSite",
      Self::PrintWithNameForAssert => "PrintWithNameForAssert",
      Self::PromiseSpeciesProtector => "PromiseSpeciesProtector",
      Self::RegExpSpeciesProtector => "RegExpSpeciesProtector",
      Self::RegexpHasBytecode => "RegexpHasBytecode",
      Self::RegexpHasNativeCode => "RegexpHasNativeCode",
      Self::RegexpIsUnmodified => "RegexpIsUnmodified",
      Self::RegexpQuickCheckRejects => "RegexpQuickCheckRejects",
      Self::RegexpTypeTag => "RegexpTypeTag",
      Self::Resume => "Resume",
      Self::RunningInSimulator => "RunningInSimulator",
      Self::RuntimeEvaluateREPL => "RuntimeEvaluateREPL",
      Self::ScheduleGCInStackCheck => "ScheduleGCInStackCheck",
      Self::SerializeDeserializeNow => "SerializeDeserializeNow",
      Self::SetAllocationTimeout => "SetAllocationTimeout",
      Self::SetDispatchTableGCInterval => "SetDispatchTableGCInterval",
      Self::SetBatterySaverMode => "SetBatterySaverMode",
      Self::SetForceSlowPath => "SetForceSlowPath",
      Self::SetIteratorProtector => "SetIteratorProtector",
      Self::SharedGC => "SharedGC",
      Self::ShareObject => "ShareObject",
      Self::SimulateNewspaceFull => "SimulateNewspaceFull",
      Self::StringIsFlat => "StringIsFlat",
      Self::StringIsOneByteRepresentation => "StringIsOneByteRepresentation",
      Self::StringIteratorProtector => "StringIteratorProtector",
      Self::StringWrapperToPrimitiveProtector => "StringWrapperToPrimitiveProtector",
      Self::SystemBreak => "SystemBreak",
      Self::TakeHeapSnapshot => "TakeHeapSnapshot",
      Self::TraceEnter => "TraceEnter",
      Self::TraceExit => "TraceExit",
      Self::TurbofanStaticAssert => "TurbofanStaticAssert",
      Self::AssertEscapeAnalysisElided => "AssertEscapeAnalysisElided",
      Self::TypedArraySpeciesProtector => "TypedArraySpeciesProtector",
      Self::WaitForBackgroundOptimization => "WaitForBackgroundOptimization",
      Self::WaitUntilBlocked => "WaitUntilBlocked",
      Self::DeoptimizeNow => "DeoptimizeNow",
      Self::LeakHole => "LeakHole",
      Self::GetHoleNaNLower => "GetHoleNaNLower",
      Self::GetHoleNaNUpper => "GetHoleNaNUpper",
      Self::GetHoleNaN => "GetHoleNaN",
      Self::GetUndefinedNaN => "GetUndefinedNaN",
      Self::ArrayBufferDetach => "ArrayBufferDetach",
      Self::ArrayBufferSetDetachKey => "ArrayBufferSetDetachKey",
      Self::GrowableSharedArrayBufferByteLength => "GrowableSharedArrayBufferByteLength",
      Self::TypedArrayCopyElements => "TypedArrayCopyElements",
      Self::TypedArrayGetBuffer => "TypedArrayGetBuffer",
      Self::TypedArraySet => "TypedArraySet",
      Self::TypedArraySortFast => "TypedArraySortFast",
      Self::WasmRunInterpreter => "WasmRunInterpreter",
      Self::ThrowWasmJSPISuspendError => "ThrowWasmJSPISuspendError",
      Self::ThrowWasmFXSuspendError => "ThrowWasmFXSuspendError",
      Self::ThrowWasmError => "ThrowWasmError",
      Self::ThrowWasmStackOverflow => "ThrowWasmStackOverflow",
      Self::WasmI32AtomicWait => "WasmI32AtomicWait",
      Self::WasmI64AtomicWait => "WasmI64AtomicWait",
      Self::WasmManagedObjectWait32 => "WasmManagedObjectWait32",
      Self::WasmManagedObjectWait64 => "WasmManagedObjectWait64",
      Self::WasmManagedObjectWaitRef => "WasmManagedObjectWaitRef",
      Self::WasmWaitqueueNew => "WasmWaitqueueNew",
      Self::WasmMemoryGrow => "WasmMemoryGrow",
      Self::WasmStackGuard => "WasmStackGuard",
      Self::WasmStackGuardLoop => "WasmStackGuardLoop",
      Self::WasmThrow => "WasmThrow",
      Self::WasmReThrow => "WasmReThrow",
      Self::WasmThrowJSTypeError => "WasmThrowJSTypeError",
      Self::WasmThrowTypeError => "WasmThrowTypeError",
      Self::WasmThrowRangeError => "WasmThrowRangeError",
      Self::WasmThrowDataViewTypeError => "WasmThrowDataViewTypeError",
      Self::WasmThrowDataViewDetachedError => "WasmThrowDataViewDetachedError",
      Self::WasmRefFunc => "WasmRefFunc",
      Self::WasmInternalFunctionCreateExternal => "WasmInternalFunctionCreateExternal",
      Self::WasmFunctionTableGet => "WasmFunctionTableGet",
      Self::WasmFunctionTableSet => "WasmFunctionTableSet",
      Self::WasmTableInit => "WasmTableInit",
      Self::WasmTableCopy => "WasmTableCopy",
      Self::WasmTableGrow => "WasmTableGrow",
      Self::WasmTableFill => "WasmTableFill",
      Self::WasmJSToWasmObject => "WasmJSToWasmObject",
      Self::WasmWasmToJSObject => "WasmWasmToJSObject",
      Self::WasmCompileLazy => "WasmCompileLazy",
      Self::WasmAllocateFeedbackVector => "WasmAllocateFeedbackVector",
      Self::WasmLiftoffDeoptFinish => "WasmLiftoffDeoptFinish",
      Self::TierUpJSToWasmWrapper => "TierUpJSToWasmWrapper",
      Self::IsWasmExternalFunction => "IsWasmExternalFunction",
      Self::TierUpWasmToJSWrapper => "TierUpWasmToJSWrapper",
      Self::WasmTriggerTierUp => "WasmTriggerTierUp",
      Self::WasmDebugBreak => "WasmDebugBreak",
      Self::WasmAllocateDescriptorStruct => "WasmAllocateDescriptorStruct",
      Self::WasmArrayCopy => "WasmArrayCopy",
      Self::WasmArrayNewSegment => "WasmArrayNewSegment",
      Self::WasmArrayInitSegment => "WasmArrayInitSegment",
      Self::WasmAllocateSuspender => "WasmAllocateSuspender",
      Self::WasmSuspended => "WasmSuspended",
      Self::WasmAllocateContinuation => "WasmAllocateContinuation",
      Self::ClearWasmSuspenderResumeField => "ClearWasmSuspenderResumeField",
      Self::WasmCastToSpecialPrimitiveArray => "WasmCastToSpecialPrimitiveArray",
      Self::WasmStringAddNoMapCheckShared => "WasmStringAdd_NoMapCheck_Shared",
      Self::WasmStringNewSegmentWtf8 => "WasmStringNewSegmentWtf8",
      Self::WasmStringNewWtf8 => "WasmStringNewWtf8",
      Self::WasmStringNewWtf8Array => "WasmStringNewWtf8Array",
      Self::WasmStringNewWtf16 => "WasmStringNewWtf16",
      Self::WasmStringNewWtf16Array => "WasmStringNewWtf16Array",
      Self::WasmStringConst => "WasmStringConst",
      Self::WasmStringMeasureUtf8 => "WasmStringMeasureUtf8",
      Self::WasmStringMeasureWtf8 => "WasmStringMeasureWtf8",
      Self::WasmStringEncodeWtf8 => "WasmStringEncodeWtf8",
      Self::WasmStringEncodeWtf16 => "WasmStringEncodeWtf16",
      Self::WasmStringEncodeWtf8Array => "WasmStringEncodeWtf8Array",
      Self::WasmStringToUtf8Array => "WasmStringToUtf8Array",
      Self::WasmStringAsWtf8 => "WasmStringAsWtf8",
      Self::WasmStringViewWtf8Encode => "WasmStringViewWtf8Encode",
      Self::WasmStringViewWtf8Slice => "WasmStringViewWtf8Slice",
      Self::WasmStringFromCodePoint => "WasmStringFromCodePoint",
      Self::WasmStringHash => "WasmStringHash",
      Self::WasmSubstring => "WasmSubstring",
      Self::WasmSubstringShared => "WasmSubstringShared",
      Self::WasmConfigureAllPrototypes => "WasmConfigureAllPrototypes",
      Self::WasmConfigureAllPrototypesOpt => "WasmConfigureAllPrototypesOpt",
      Self::DebugCollectWasmCoverage => "DebugCollectWasmCoverage",
      Self::TsanAcquireForInitializationFence => "TsanAcquireForInitializationFence",
      Self::WasmTypeAssertionFailed => "WasmTypeAssertionFailed",
      Self::BuildRefTypeBitfield => "BuildRefTypeBitfield",
      Self::CheckIsOnCentralStack => "CheckIsOnCentralStack",
      Self::CountUnoptimizedWasmToJSWrapper => "CountUnoptimizedWasmToJSWrapper",
      Self::DisallowWasmCodegen => "DisallowWasmCodegen",
      Self::EstimateCurrentMemoryConsumption => "EstimateCurrentMemoryConsumption",
      Self::FlushLiftoffCode => "FlushLiftoffCode",
      Self::FreezeWasmLazyCompilation => "FreezeWasmLazyCompilation",
      Self::GenerateWasmCompilationHints => "GenerateWasmCompilationHints",
      Self::GetWasmExceptionTagId => "GetWasmExceptionTagId",
      Self::GetWasmExceptionValues => "GetWasmExceptionValues",
      Self::GetWasmRecoveredTrapCount => "GetWasmRecoveredTrapCount",
      Self::HasUnoptimizedWasmToJSWrapper => "HasUnoptimizedWasmToJSWrapper",
      Self::IsLiftoffFunction => "IsLiftoffFunction",
      Self::IsTurboFanFunction => "IsTurboFanFunction",
      Self::IsUncompiledWasmFunction => "IsUncompiledWasmFunction",
      Self::IsWasmCode => "IsWasmCode",
      Self::IsWasmDebugFunction => "IsWasmDebugFunction",
      Self::IsWasmPartialOOBWriteNoop => "IsWasmPartialOOBWriteNoop",
      Self::IsWasmTrapHandlerEnabled => "IsWasmTrapHandlerEnabled",
      Self::SetWasmCompileControls => "SetWasmCompileControls",
      Self::SetWasmInstantiateControls => "SetWasmInstantiateControls",
      Self::WasmArray => "WasmArray",
      Self::WasmCompiledExportWrappersCount => "WasmCompiledExportWrappersCount",
      Self::WasmDeoptsExecutedCount => "WasmDeoptsExecutedCount",
      Self::WasmDeoptsExecutedForFunction => "WasmDeoptsExecutedForFunction",
      Self::WasmEnterDebugging => "WasmEnterDebugging",
      Self::WasmGenerateRandomModule => "WasmGenerateRandomModule",
      Self::WasmGetNumberOfInstances => "WasmGetNumberOfInstances",
      Self::WasmLeaveDebugging => "WasmLeaveDebugging",
      Self::WasmNumCodeSpaces => "WasmNumCodeSpaces",
      Self::WasmStruct => "WasmStruct",
      Self::WasmSwitchToTheCentralStackCount => "WasmSwitchToTheCentralStackCount",
      Self::WasmTierUpFunction => "WasmTierUpFunction",
      Self::WasmTraceEnter => "WasmTraceEnter",
      Self::WasmTraceExit => "WasmTraceExit",
      Self::WasmTraceMemory => "WasmTraceMemory",
      Self::WasmTriggerCodeGC => "WasmTriggerCodeGC",
      Self::WasmTraceGlobal => "WasmTraceGlobal",
      Self::WasmTriggerTierUpForTesting => "WasmTriggerTierUpForTesting",
      Self::WasmTraceBeginExecution => "WasmTraceBeginExecution",
      Self::WasmTraceEndExecution => "WasmTraceEndExecution",
      Self::JSFinalizationRegistryRegisterWeakCellWithUnregisterToken => {
        "JSFinalizationRegistryRegisterWeakCellWithUnregisterToken"
      }
      Self::JSWeakRefAddToKeptObjects => "JSWeakRefAddToKeptObjects",
      Self::DebugBreakOnBytecode => "DebugBreakOnBytecode",
      Self::LoadLookupSlotForCall => "LoadLookupSlotForCall",
      Self::ElementsTransitionAndStoreICMiss => "ElementsTransitionAndStoreIC_Miss",
      Self::KeyedLoadICMiss => "KeyedLoadIC_Miss",
      Self::KeyedStoreICMiss => "KeyedStoreIC_Miss",
      Self::DefineKeyedOwnICMiss => "DefineKeyedOwnIC_Miss",
      Self::StoreInArrayLiteralICMiss => "StoreInArrayLiteralIC_Miss",
      Self::DefineNamedOwnICSlow => "DefineNamedOwnIC_Slow",
      Self::KeyedStoreICSlow => "KeyedStoreIC_Slow",
      Self::DefineKeyedOwnICSlow => "DefineKeyedOwnIC_Slow",
      Self::LoadElementWithInterceptor => "LoadElementWithInterceptor",
      Self::LoadGlobalICMiss => "LoadGlobalIC_Miss",
      Self::LoadGlobalICSlow => "LoadGlobalIC_Slow",
      Self::LoadICMiss => "LoadIC_Miss",
      Self::LoadICMissFromBaseline => "LoadIC_Miss_FromBaseline",
      Self::PatchLoadICUninitializedBaseline => "PatchLoadICUninitializedBaseline",
      Self::GetStringLengthAndUpdateFeedback => "GetStringLengthAndUpdateFeedback",
      Self::LoadNoFeedbackICMiss => "LoadNoFeedbackIC_Miss",
      Self::LoadWithReceiverICMiss => "LoadWithReceiverIC_Miss",
      Self::LoadWithReceiverNoFeedbackICMiss => "LoadWithReceiverNoFeedbackIC_Miss",
      Self::LoadPropertyPastInterceptor => "LoadPropertyPastInterceptor",
      Self::StoreCallbackProperty => "StoreCallbackProperty",
      Self::StoreGlobalICMiss => "StoreGlobalIC_Miss",
      Self::StoreGlobalICNoFeedbackMiss => "StoreGlobalICNoFeedback_Miss",
      Self::StoreGlobalICSlow => "StoreGlobalIC_Slow",
      Self::StoreICMiss => "StoreIC_Miss",
      Self::DefineNamedOwnICMiss => "DefineNamedOwnIC_Miss",
      Self::StoreInArrayLiteralICSlow => "StoreInArrayLiteralIC_Slow",
      Self::StorePropertyPastInterceptor => "StorePropertyPastInterceptor",
      Self::CloneObjectICSlow => "CloneObjectIC_Slow",
      Self::CloneObjectICMiss => "CloneObjectIC_Miss",
      Self::KeyedHasICMiss => "KeyedHasIC_Miss",
      Self::HasElementWithInterceptor => "HasElementWithInterceptor",
      Self::ObjectAssignTryFastcase => "ObjectAssignTryFastcase",
      Self::NumFunctions => "NumFunctions",
    }
  }

  pub const fn c_enum_name(&self) -> &'static str {
    match self {
      Self::ArrayIncludesSlow => "kArrayIncludes_Slow",
      Self::ArrayIndexOf => "kArrayIndexOf",
      Self::ArrayIsArray => "kArrayIsArray",
      Self::ArraySpeciesConstructor => "kArraySpeciesConstructor",
      Self::GrowArrayElements => "kGrowArrayElements",
      Self::IsArray => "kIsArray",
      Self::NewArray => "kNewArray",
      Self::NormalizeElements => "kNormalizeElements",
      Self::TransitionElementsKind => "kTransitionElementsKind",
      Self::TransitionElementsKindWithKind => "kTransitionElementsKindWithKind",
      Self::AtomicsLoad64 => "kAtomicsLoad64",
      Self::AtomicsStore64 => "kAtomicsStore64",
      Self::AtomicsAdd => "kAtomicsAdd",
      Self::AtomicsAnd => "kAtomicsAnd",
      Self::AtomicsCompareExchange => "kAtomicsCompareExchange",
      Self::AtomicsExchange => "kAtomicsExchange",
      Self::AtomicsNumWaitersForTesting => "kAtomicsNumWaitersForTesting",
      Self::AtomicsNumUnresolvedAsyncPromisesForTesting => {
        "kAtomicsNumUnresolvedAsyncPromisesForTesting"
      }
      Self::AtomicsOr => "kAtomicsOr",
      Self::AtomicsSub => "kAtomicsSub",
      Self::AtomicsXor => "kAtomicsXor",
      Self::SetAllowAtomicsWait => "kSetAllowAtomicsWait",
      Self::AtomicsLoadSharedStructOrArray => "kAtomicsLoadSharedStructOrArray",
      Self::AtomicsStoreSharedStructOrArray => "kAtomicsStoreSharedStructOrArray",
      Self::AtomicsExchangeSharedStructOrArray => "kAtomicsExchangeSharedStructOrArray",
      Self::AtomicsCompareExchangeSharedStructOrArray => {
        "kAtomicsCompareExchangeSharedStructOrArray"
      }
      Self::AtomicsSynchronizationPrimitiveNumWaitersForTesting => {
        "kAtomicsSynchronizationPrimitiveNumWaitersForTesting"
      }
      Self::BigIntCompareToNumber => "kBigIntCompareToNumber",
      Self::BigIntCompareToString => "kBigIntCompareToString",
      Self::BigIntEqualToBigInt => "kBigIntEqualToBigInt",
      Self::BigIntEqualToNumber => "kBigIntEqualToNumber",
      Self::BigIntEqualToString => "kBigIntEqualToString",
      Self::BigIntExponentiate => "kBigIntExponentiate",
      Self::BigIntMaxBits => "kBigIntMaxBits",
      Self::BigIntToNumber => "kBigIntToNumber",
      Self::BigIntUnaryOp => "kBigIntUnaryOp",
      Self::ToBigInt => "kToBigInt",
      Self::ToBigIntConvertNumber => "kToBigIntConvertNumber",
      Self::ThrowConstructorNonCallableError => "kThrowConstructorNonCallableError",
      Self::ThrowNotSuperConstructor => "kThrowNotSuperConstructor",
      Self::ThrowStaticPrototypeError => "kThrowStaticPrototypeError",
      Self::ThrowSuperAlreadyCalledError => "kThrowSuperAlreadyCalledError",
      Self::ThrowSuperNotCalled => "kThrowSuperNotCalled",
      Self::ThrowUnsupportedSuperError => "kThrowUnsupportedSuperError",
      Self::DefineClass => "kDefineClass",
      Self::LoadFromSuper => "kLoadFromSuper",
      Self::LoadKeyedFromSuper => "kLoadKeyedFromSuper",
      Self::StoreKeyedToSuper => "kStoreKeyedToSuper",
      Self::StoreToSuper => "kStoreToSuper",
      Self::MapGrow => "kMapGrow",
      Self::MapShrink => "kMapShrink",
      Self::OrderedHashSetGrow => "kOrderedHashSetGrow",
      Self::SetGrow => "kSetGrow",
      Self::SetShrink => "kSetShrink",
      Self::OrderedHashSetShrink => "kOrderedHashSetShrink",
      Self::TheHole => "kTheHole",
      Self::WeakCollectionDelete => "kWeakCollectionDelete",
      Self::WeakCollectionSet => "kWeakCollectionSet",
      Self::OrderedHashMapGrow => "kOrderedHashMapGrow",
      Self::PrintDumpedFrame => "kPrintDumpedFrame",
      Self::CompileOptimizedOSR => "kCompileOptimizedOSR",
      Self::CompileOptimizedOSRFromMaglev => "kCompileOptimizedOSRFromMaglev",
      Self::CompileOptimizedOSRFromMaglevInlined => "kCompileOptimizedOSRFromMaglevInlined",
      Self::LogOrTraceOptimizedOSREntry => "kLogOrTraceOptimizedOSREntry",
      Self::CompileLazy => "kCompileLazy",
      Self::CompileBaseline => "kCompileBaseline",
      Self::InstallBaselineCode => "kInstallBaselineCode",
      Self::InstallSFICode => "kInstallSFICode",
      Self::NotifyDeoptimized => "kNotifyDeoptimized",
      Self::ObserveNode => "kObserveNode",
      Self::ResolvePossiblyDirectEval => "kResolvePossiblyDirectEval",
      Self::VerifyType => "kVerifyType",
      Self::CheckTurboshaftTypeOf => "kCheckTurboshaftTypeOf",
      Self::PatchCompareOpBaselineCode => "kPatchCompareOpBaselineCode",
      Self::PatchCompareOpBaselineCodeAndThrow => "kPatchCompareOpBaselineCodeAndThrow",
      Self::PatchBinopBaselineCode => "kPatchBinopBaselineCode",
      Self::PatchBinopBaselineCodeAndThrow => "kPatchBinopBaselineCodeAndThrow",
      Self::PatchUnaryOpBaselineCode => "kPatchUnaryOpBaselineCode",
      Self::PatchUnaryOpBaselineCodeAndThrow => "kPatchUnaryOpBaselineCodeAndThrow",
      Self::FunctionLogNextExecution => "kFunctionLogNextExecution",
      Self::OptimizeMaglevEager => "kOptimizeMaglevEager",
      Self::StartMaglevOptimizeJob => "kStartMaglevOptimizeJob",
      Self::OptimizeTurbofanEager => "kOptimizeTurbofanEager",
      Self::StartTurbofanOptimizeJob => "kStartTurbofanOptimizeJob",
      Self::MarkLazyDeoptimizedOrFlushed => "kMarkLazyDeoptimizedOrFlushed",
      Self::DateCurrentTime => "kDateCurrentTime",
      Self::ClearStepping => "kClearStepping",
      Self::CollectGarbage => "kCollectGarbage",
      Self::DebugAsyncFunctionSuspended => "kDebugAsyncFunctionSuspended",
      Self::DebugBreakAtEntry => "kDebugBreakAtEntry",
      Self::DebugCollectCoverage => "kDebugCollectCoverage",
      Self::DebugGetLoadedScriptIds => "kDebugGetLoadedScriptIds",
      Self::DebugOnFunctionCall => "kDebugOnFunctionCall",
      Self::DebugPrepareStepInSuspendedGenerator => "kDebugPrepareStepInSuspendedGenerator",
      Self::DebugPromiseThen => "kDebugPromiseThen",
      Self::DebugToggleBlockCoverage => "kDebugToggleBlockCoverage",
      Self::DebugTogglePreciseCoverage => "kDebugTogglePreciseCoverage",
      Self::FunctionGetInferredName => "kFunctionGetInferredName",
      Self::GetBreakLocations => "kGetBreakLocations",
      Self::GetGeneratorScopeCount => "kGetGeneratorScopeCount",
      Self::GetGeneratorScopeDetails => "kGetGeneratorScopeDetails",
      Self::HandleDebuggerStatement => "kHandleDebuggerStatement",
      Self::IsBreakOnException => "kIsBreakOnException",
      Self::IterableForEach => "kIterableForEach",
      Self::ProfileCreateSnapshotDataBlob => "kProfileCreateSnapshotDataBlob",
      Self::ScheduleBreak => "kScheduleBreak",
      Self::ScriptLocationFromLine2 => "kScriptLocationFromLine2",
      Self::SetGeneratorScopeVariableValue => "kSetGeneratorScopeVariableValue",
      Self::IncBlockCounter => "kIncBlockCounter",
      Self::ForInEnumerate => "kForInEnumerate",
      Self::ForInHasProperty => "kForInHasProperty",
      Self::TraceUnoptimizedBytecodeEntry => "kTraceUnoptimizedBytecodeEntry",
      Self::TraceUnoptimizedBytecodeExit => "kTraceUnoptimizedBytecodeExit",
      Self::DumpExecutionFrame => "kDumpExecutionFrame",
      Self::TraceUpdateFeedback => "kTraceUpdateFeedback",
      Self::Call => "kCall",
      Self::FunctionGetScriptSource => "kFunctionGetScriptSource",
      Self::FunctionGetScriptId => "kFunctionGetScriptId",
      Self::FunctionGetScriptSourcePosition => "kFunctionGetScriptSourcePosition",
      Self::FunctionGetSourceCode => "kFunctionGetSourceCode",
      Self::FunctionIsAPIFunction => "kFunctionIsAPIFunction",
      Self::AsyncFunctionAwait => "kAsyncFunctionAwait",
      Self::AsyncFunctionEnter => "kAsyncFunctionEnter",
      Self::AsyncFunctionReject => "kAsyncFunctionReject",
      Self::AsyncFunctionResolve => "kAsyncFunctionResolve",
      Self::AsyncGeneratorAwait => "kAsyncGeneratorAwait",
      Self::AsyncGeneratorReject => "kAsyncGeneratorReject",
      Self::AsyncGeneratorResolve => "kAsyncGeneratorResolve",
      Self::AsyncGeneratorYieldWithAwait => "kAsyncGeneratorYieldWithAwait",
      Self::CreateJSGeneratorObject => "kCreateJSGeneratorObject",
      Self::GeneratorClose => "kGeneratorClose",
      Self::GeneratorGetFunction => "kGeneratorGetFunction",
      Self::GeneratorGetResumeMode => "kGeneratorGetResumeMode",
      Self::FormatList => "kFormatList",
      Self::FormatListToParts => "kFormatListToParts",
      Self::StringToLowerCaseIntl => "kStringToLowerCaseIntl",
      Self::StringToLocaleLowerCase => "kStringToLocaleLowerCase",
      Self::StringToUpperCaseIntl => "kStringToUpperCaseIntl",
      Self::Throw => "kThrow",
      Self::ThrowTargetNonFunction => "kThrowTargetNonFunction",
      Self::ThrowCalledNonCallable => "kThrowCalledNonCallable",
      Self::ThrowConstructedNonConstructable => "kThrowConstructedNonConstructable",
      Self::ThrowConstructorReturnedNonObject => "kThrowConstructorReturnedNonObject",
      Self::ThrowInvalidStringLength => "kThrowInvalidStringLength",
      Self::ThrowInvalidTypedArrayAlignment => "kThrowInvalidTypedArrayAlignment",
      Self::ThrowIteratorError => "kThrowIteratorError",
      Self::ThrowSpreadArgError => "kThrowSpreadArgError",
      Self::ThrowIteratorResultNotAnObject => "kThrowIteratorResultNotAnObject",
      Self::ThrowNoAccess => "kThrowNoAccess",
      Self::ThrowNotConstructor => "kThrowNotConstructor",
      Self::ThrowPatternAssignmentNonCoercible => "kThrowPatternAssignmentNonCoercible",
      Self::ThrowRangeError => "kThrowRangeError",
      Self::ThrowReferenceError => "kThrowReferenceError",
      Self::ThrowAccessedUninitializedVariable => "kThrowAccessedUninitializedVariable",
      Self::VarargStackOverflow => "kVarargStackOverflow",
      Self::ThrowStackOverflow => "kThrowStackOverflow",
      Self::ThrowSymbolAsyncIteratorInvalid => "kThrowSymbolAsyncIteratorInvalid",
      Self::ThrowSymbolIteratorInvalid => "kThrowSymbolIteratorInvalid",
      Self::ThrowThrowMethodMissing => "kThrowThrowMethodMissing",
      Self::ThrowTypeError => "kThrowTypeError",
      Self::ThrowTypeErrorIfStrict => "kThrowTypeErrorIfStrict",
      Self::ReThrow => "kReThrow",
      Self::ReThrowWithMessage => "kReThrowWithMessage",
      Self::AccessCheck => "kAccessCheck",
      Self::AllocateByteArray => "kAllocateByteArray",
      Self::AllocateInYoungGeneration => "kAllocateInYoungGeneration",
      Self::AllocateInOldGeneration => "kAllocateInOldGeneration",
      Self::AllocateInSharedHeap => "kAllocateInSharedHeap",
      Self::AllowDynamicFunction => "kAllowDynamicFunction",
      Self::CreateAsyncFromSyncIterator => "kCreateAsyncFromSyncIterator",
      Self::CreateListFromArrayLike => "kCreateListFromArrayLike",
      Self::DoubleToStringWithRadix => "kDoubleToStringWithRadix",
      Self::FatalProcessOutOfMemoryInAllocateRaw => "kFatalProcessOutOfMemoryInAllocateRaw",
      Self::FatalProcessOutOfMemoryInvalidArrayLength => {
        "kFatalProcessOutOfMemoryInvalidArrayLength"
      }
      Self::FatalInvalidSize => "kFatalInvalidSize",
      Self::GetAndResetRuntimeCallStats => "kGetAndResetRuntimeCallStats",
      Self::GetAndResetTurboProfilingData => "kGetAndResetTurboProfilingData",
      Self::GetTemplateObject => "kGetTemplateObject",
      Self::IncrementUseCounter => "kIncrementUseCounter",
      Self::InvalidateStringWrapperToPrimitiveProtector => {
        "kInvalidateStringWrapperToPrimitiveProtector"
      }
      Self::BytecodeBudgetInterruptIgnition => "kBytecodeBudgetInterrupt_Ignition",
      Self::BytecodeBudgetInterruptWithStackCheckIgnition => {
        "kBytecodeBudgetInterruptWithStackCheck_Ignition"
      }
      Self::BytecodeBudgetInterruptSparkplug => "kBytecodeBudgetInterrupt_Sparkplug",
      Self::BytecodeBudgetInterruptWithStackCheckSparkplug => {
        "kBytecodeBudgetInterruptWithStackCheck_Sparkplug"
      }
      Self::BytecodeBudgetInterruptMaglev => "kBytecodeBudgetInterrupt_Maglev",
      Self::BytecodeBudgetLoopInterruptMaglev => "kBytecodeBudgetLoopInterrupt_Maglev",
      Self::NotifyContextCellStateWillChange => "kNotifyContextCellStateWillChange",
      Self::NewError => "kNewError",
      Self::NewReferenceError => "kNewReferenceError",
      Self::NewTypeError => "kNewTypeError",
      Self::OrdinaryHasInstance => "kOrdinaryHasInstance",
      Self::PropagateException => "kPropagateException",
      Self::ReportMessageFromMicrotask => "kReportMessageFromMicrotask",
      Self::RunMicrotaskCallback => "kRunMicrotaskCallback",
      Self::PerformMicrotaskCheckpoint => "kPerformMicrotaskCheckpoint",
      Self::SharedValueBarrierSlow => "kSharedValueBarrierSlow",
      Self::StackGuard => "kStackGuard",
      Self::HandleNoHeapWritesInterrupts => "kHandleNoHeapWritesInterrupts",
      Self::StackGuardWithGap => "kStackGuardWithGap",
      Self::TerminateExecution => "kTerminateExecution",
      Self::Typeof => "kTypeof",
      Self::UnwindAndFindExceptionHandler => "kUnwindAndFindExceptionHandler",
      Self::AddLhsIsStringConstantInternalize => "kAddLhsIsStringConstantInternalize",
      Self::AddRhsIsStringConstantInternalize => "kAddRhsIsStringConstantInternalize",
      Self::CreateArrayLiteral => "kCreateArrayLiteral",
      Self::CreateObjectLiteral => "kCreateObjectLiteral",
      Self::SetPrototypeProperties => "kSetPrototypeProperties",
      Self::CreateRegExpLiteral => "kCreateRegExpLiteral",
      Self::DynamicImportCall => "kDynamicImportCall",
      Self::GetImportMetaObject => "kGetImportMetaObject",
      Self::GetModuleNamespace => "kGetModuleNamespace",
      Self::GetModuleNamespaceExport => "kGetModuleNamespaceExport",
      Self::ArrayBufferMaxByteLength => "kArrayBufferMaxByteLength",
      Self::IsSmi => "kIsSmi",
      Self::MaxSmi => "kMaxSmi",
      Self::NumberToStringSlow => "kNumberToStringSlow",
      Self::Float64ToStringSlow => "kFloat64ToStringSlow",
      Self::StringParseFloat => "kStringParseFloat",
      Self::StringParseInt => "kStringParseInt",
      Self::StringToNumber => "kStringToNumber",
      Self::AddAsyncDisposableValue => "kAddAsyncDisposableValue",
      Self::AddDictionaryProperty => "kAddDictionaryProperty",
      Self::AddDisposableValue => "kAddDisposableValue",
      Self::AddPrivateBrand => "kAddPrivateBrand",
      Self::AllocateHeapNumber => "kAllocateHeapNumber",
      Self::CompleteInobjectSlackTrackingForMap => "kCompleteInobjectSlackTrackingForMap",
      Self::CopyDataProperties => "kCopyDataProperties",
      Self::CopyDataPropertiesWithExcludedPropertiesOnStack => {
        "kCopyDataPropertiesWithExcludedPropertiesOnStack"
      }
      Self::CreateDataProperty => "kCreateDataProperty",
      Self::CreateIterResultObject => "kCreateIterResultObject",
      Self::IterableToListWithInterceptor => "kIterableToListWithInterceptor",
      Self::CheckFastIterableToListPrototype => "kCheckFastIterableToListPrototype",
      Self::CreatePrivateAccessors => "kCreatePrivateAccessors",
      Self::DefineAccessorPropertyUnchecked => "kDefineAccessorPropertyUnchecked",
      Self::DefineKeyedOwnPropertyInLiteral => "kDefineKeyedOwnPropertyInLiteral",
      Self::DefineGetterPropertyUnchecked => "kDefineGetterPropertyUnchecked",
      Self::DefineSetterPropertyUnchecked => "kDefineSetterPropertyUnchecked",
      Self::DeleteProperty => "kDeleteProperty",
      Self::DisposeDisposableStack => "kDisposeDisposableStack",
      Self::GeneratorYieldResult => "kGeneratorYieldResult",
      Self::GetDerivedMap => "kGetDerivedMap",
      Self::GetFunctionName => "kGetFunctionName",
      Self::GetOwnPropertyDescriptorObject => "kGetOwnPropertyDescriptorObject",
      Self::GetOwnPropertyKeys => "kGetOwnPropertyKeys",
      Self::GetPrivateMember => "kGetPrivateMember",
      Self::GetProperty => "kGetProperty",
      Self::HandleExceptionsInDisposeDisposableStack => "kHandleExceptionsInDisposeDisposableStack",
      Self::HasFastPackedElements => "kHasFastPackedElements",
      Self::HasInPrototypeChain => "kHasInPrototypeChain",
      Self::HasProperty => "kHasProperty",
      Self::InitializeDisposableStack => "kInitializeDisposableStack",
      Self::InternalSetPrototype => "kInternalSetPrototype",
      Self::IsJSReceiver => "kIsJSReceiver",
      Self::JSReceiverPreventExtensionsDontThrow => "kJSReceiverPreventExtensionsDontThrow",
      Self::JSReceiverPreventExtensionsThrow => "kJSReceiverPreventExtensionsThrow",
      Self::JSReceiverGetPrototypeOf => "kJSReceiverGetPrototypeOf",
      Self::JSReceiverSetPrototypeOfDontThrow => "kJSReceiverSetPrototypeOfDontThrow",
      Self::JSReceiverSetPrototypeOfThrow => "kJSReceiverSetPrototypeOfThrow",
      Self::LoadPrivateGetter => "kLoadPrivateGetter",
      Self::LoadPrivateSetter => "kLoadPrivateSetter",
      Self::NewObject => "kNewObject",
      Self::ObjectCreate => "kObjectCreate",
      Self::ObjectEntries => "kObjectEntries",
      Self::ObjectEntriesSkipFastPath => "kObjectEntriesSkipFastPath",
      Self::ObjectGetOwnPropertyNames => "kObjectGetOwnPropertyNames",
      Self::ObjectGetOwnPropertyNamesTryFast => "kObjectGetOwnPropertyNamesTryFast",
      Self::ObjectHasOwnProperty => "kObjectHasOwnProperty",
      Self::HasOwnConstDataProperty => "kHasOwnConstDataProperty",
      Self::ObjectIsExtensible => "kObjectIsExtensible",
      Self::ObjectKeys => "kObjectKeys",
      Self::ObjectValues => "kObjectValues",
      Self::ObjectValuesSkipFastPath => "kObjectValuesSkipFastPath",
      Self::OptimizeObjectForAddingMultipleProperties => {
        "kOptimizeObjectForAddingMultipleProperties"
      }
      Self::SetDataProperties => "kSetDataProperties",
      Self::SetFunctionName => "kSetFunctionName",
      Self::SetKeyedProperty => "kSetKeyedProperty",
      Self::DefineObjectOwnProperty => "kDefineObjectOwnProperty",
      Self::SetNamedProperty => "kSetNamedProperty",
      Self::SetOwnPropertyIgnoreAttributes => "kSetOwnPropertyIgnoreAttributes",
      Self::ShrinkNameDictionary => "kShrinkNameDictionary",
      Self::ShrinkSwissNameDictionary => "kShrinkSwissNameDictionary",
      Self::ToFastProperties => "kToFastProperties",
      Self::ToLength => "kToLength",
      Self::ToName => "kToName",
      Self::ToNumber => "kToNumber",
      Self::ToNumeric => "kToNumeric",
      Self::ToObject => "kToObject",
      Self::ToString => "kToString",
      Self::TryMigrateInstance => "kTryMigrateInstance",
      Self::TryMigrateInstanceAndMarkMapAsMigrationTarget => {
        "kTryMigrateInstanceAndMarkMapAsMigrationTarget"
      }
      Self::SetPrivateMember => "kSetPrivateMember",
      Self::SwissTableAdd => "kSwissTableAdd",
      Self::SwissTableAllocate => "kSwissTableAllocate",
      Self::SwissTableDelete => "kSwissTableDelete",
      Self::SwissTableDetailsAt => "kSwissTableDetailsAt",
      Self::SwissTableElementsCount => "kSwissTableElementsCount",
      Self::SwissTableEquals => "kSwissTableEquals",
      Self::SwissTableFindEntry => "kSwissTableFindEntry",
      Self::SwissTableUpdate => "kSwissTableUpdate",
      Self::SwissTableValueAt => "kSwissTableValueAt",
      Self::SwissTableKeyAt => "kSwissTableKeyAt",
      Self::Add => "kAdd",
      Self::Equal => "kEqual",
      Self::GreaterThan => "kGreaterThan",
      Self::GreaterThanOrEqual => "kGreaterThanOrEqual",
      Self::LessThan => "kLessThan",
      Self::LessThanOrEqual => "kLessThanOrEqual",
      Self::NotEqual => "kNotEqual",
      Self::StrictEqual => "kStrictEqual",
      Self::StrictNotEqual => "kStrictNotEqual",
      Self::ReferenceEqual => "kReferenceEqual",
      Self::EnqueueMicrotask => "kEnqueueMicrotask",
      Self::PromiseHookAfter => "kPromiseHookAfter",
      Self::PromiseHookBefore => "kPromiseHookBefore",
      Self::PromiseHookInit => "kPromiseHookInit",
      Self::PromiseRejectEventFromStack => "kPromiseRejectEventFromStack",
      Self::PromiseRevokeReject => "kPromiseRevokeReject",
      Self::RejectPromise => "kRejectPromise",
      Self::ResolvePromise => "kResolvePromise",
      Self::ConstructSuppressedError => "kConstructSuppressedError",
      Self::ConstructAggregateErrorHelper => "kConstructAggregateErrorHelper",
      Self::ConstructInternalAggregateErrorHelper => "kConstructInternalAggregateErrorHelper",
      Self::CheckProxyGetSetTrapResult => "kCheckProxyGetSetTrapResult",
      Self::CheckProxyHasTrapResult => "kCheckProxyHasTrapResult",
      Self::CheckProxyDeleteTrapResult => "kCheckProxyDeleteTrapResult",
      Self::GetPropertyWithReceiver => "kGetPropertyWithReceiver",
      Self::IsJSProxy => "kIsJSProxy",
      Self::JSProxyGetHandler => "kJSProxyGetHandler",
      Self::JSProxyGetTarget => "kJSProxyGetTarget",
      Self::SetPropertyWithReceiver => "kSetPropertyWithReceiver",
      Self::RegExpBuildIndices => "kRegExpBuildIndices",
      Self::RegExpGrowRegExpMatchInfo => "kRegExpGrowRegExpMatchInfo",
      Self::RegExpExecMultiple => "kRegExpExecMultiple",
      Self::RegExpInitializeAndCompile => "kRegExpInitializeAndCompile",
      Self::RegExpMatchGlobalAtom => "kRegExpMatchGlobalAtom",
      Self::RegExpReplaceRT => "kRegExpReplaceRT",
      Self::RegExpSplit => "kRegExpSplit",
      Self::RegExpStringFromFlags => "kRegExpStringFromFlags",
      Self::StringReplaceNonGlobalRegExpWithFunction => "kStringReplaceNonGlobalRegExpWithFunction",
      Self::StringSplit => "kStringSplit",
      Self::RegExpExec => "kRegExpExec",
      Self::RegExpExperimentalOneshotExec => "kRegExpExperimentalOneshotExec",
      Self::ThrowConstAssignError => "kThrowConstAssignError",
      Self::ThrowUsingAssignError => "kThrowUsingAssignError",
      Self::ThrowAwaitUsingAssignError => "kThrowAwaitUsingAssignError",
      Self::DeclareEvalFunction => "kDeclareEvalFunction",
      Self::DeclareEvalVar => "kDeclareEvalVar",
      Self::DeclareGlobals => "kDeclareGlobals",
      Self::DeclareModuleExports => "kDeclareModuleExports",
      Self::DeleteLookupSlot => "kDeleteLookupSlot",
      Self::LoadLookupSlot => "kLoadLookupSlot",
      Self::LoadLookupSlotInsideTypeof => "kLoadLookupSlotInsideTypeof",
      Self::LoadLookupSlotForCallBaseline => "kLoadLookupSlotForCall_Baseline",
      Self::NewClosure => "kNewClosure",
      Self::NewClosureTenured => "kNewClosure_Tenured",
      Self::NewFunctionContext => "kNewFunctionContext",
      Self::NewRestParameter => "kNewRestParameter",
      Self::NewSloppyArguments => "kNewSloppyArguments",
      Self::NewStrictArguments => "kNewStrictArguments",
      Self::PushBlockContext => "kPushBlockContext",
      Self::PushCatchContext => "kPushCatchContext",
      Self::PushWithContext => "kPushWithContext",
      Self::StoreGlobalNoHoleCheckForReplLetOrConst => "kStoreGlobalNoHoleCheckForReplLetOrConst",
      Self::StoreLookupSlotSloppy => "kStoreLookupSlot_Sloppy",
      Self::StoreLookupSlotSloppyHoisting => "kStoreLookupSlot_SloppyHoisting",
      Self::StoreLookupSlotStrict => "kStoreLookupSlot_Strict",
      Self::ShadowRealmWrappedFunctionCreate => "kShadowRealmWrappedFunctionCreate",
      Self::ShadowRealmImportValue => "kShadowRealmImportValue",
      Self::ShadowRealmThrow => "kShadowRealmThrow",
      Self::FlattenString => "kFlattenString",
      Self::GetSubstitution => "kGetSubstitution",
      Self::InternalizeString => "kInternalizeString",
      Self::StringAdd => "kStringAdd",
      Self::StringAddLhsIsStringConstantInternalize => "kStringAdd_LhsIsStringConstant_Internalize",
      Self::StringAddRhsIsStringConstantInternalize => "kStringAdd_RhsIsStringConstant_Internalize",
      Self::StringBuilderConcat => "kStringBuilderConcat",
      Self::StringCharCodeAt => "kStringCharCodeAt",
      Self::StringCodePointAt => "kStringCodePointAt",
      Self::StringCompare => "kStringCompare",
      Self::StringEqual => "kStringEqual",
      Self::StringEscapeQuotes => "kStringEscapeQuotes",
      Self::StringGreaterThan => "kStringGreaterThan",
      Self::StringGreaterThanOrEqual => "kStringGreaterThanOrEqual",
      Self::StringIsWellFormed => "kStringIsWellFormed",
      Self::StringLastIndexOf => "kStringLastIndexOf",
      Self::StringLessThan => "kStringLessThan",
      Self::StringLessThanOrEqual => "kStringLessThanOrEqual",
      Self::StringMaxLength => "kStringMaxLength",
      Self::StringReplaceOneCharWithString => "kStringReplaceOneCharWithString",
      Self::StringSubstring => "kStringSubstring",
      Self::StringToArray => "kStringToArray",
      Self::StringToWellFormed => "kStringToWellFormed",
      Self::CreatePrivateNameSymbol => "kCreatePrivateNameSymbol",
      Self::CreatePrivateBrandSymbol => "kCreatePrivateBrandSymbol",
      Self::CreatePrivateSymbol => "kCreatePrivateSymbol",
      Self::SymbolDescriptiveString => "kSymbolDescriptiveString",
      Self::SymbolIsPrivate => "kSymbolIsPrivate",
      Self::Abort => "kAbort",
      Self::AbortCSADcheck => "kAbortCSADcheck",
      Self::AbortJS => "kAbortJS",
      Self::ActiveTierIsIgnition => "kActiveTierIsIgnition",
      Self::ActiveTierIsSparkplug => "kActiveTierIsSparkplug",
      Self::ActiveTierIsMaglev => "kActiveTierIsMaglev",
      Self::ActiveTierIsTurbofan => "kActiveTierIsTurbofan",
      Self::AllocateHeapNumberWithValue => "kAllocateHeapNumberWithValue",
      Self::ArrayBufferDetachForceWasm => "kArrayBufferDetachForceWasm",
      Self::ArrayIteratorProtector => "kArrayIteratorProtector",
      Self::ArraySpeciesProtector => "kArraySpeciesProtector",
      Self::AssertNotPeeled => "kAssertNotPeeled",
      Self::AssertPeeled => "kAssertPeeled",
      Self::BaselineOsr => "kBaselineOsr",
      Self::BenchMaglev => "kBenchMaglev",
      Self::BenchTurbofan => "kBenchTurbofan",
      Self::BlockAt => "kBlockAt",
      Self::VerifyGetJSBuiltinState => "kVerifyGetJSBuiltinState",
      Self::ClearFunctionFeedback => "kClearFunctionFeedback",
      Self::ClearMegamorphicStubCache => "kClearMegamorphicStubCache",
      Self::CompleteInobjectSlackTracking => "kCompleteInobjectSlackTracking",
      Self::ConstructConsString => "kConstructConsString",
      Self::ConstructDouble => "kConstructDouble",
      Self::ConstructInternalizedString => "kConstructInternalizedString",
      Self::ConstructSlicedString => "kConstructSlicedString",
      Self::ConstructThinString => "kConstructThinString",
      Self::CurrentFrameIsTurbofan => "kCurrentFrameIsTurbofan",
      Self::DebugPrint => "kDebugPrint",
      Self::DebugPrintCppHeapPointerTable => "kDebugPrintCppHeapPointerTable",
      Self::DebugPrintCppHeapPointerTableFilterTag => "kDebugPrintCppHeapPointerTableFilterTag",
      Self::DebugPrintExternalPointerTable => "kDebugPrintExternalPointerTable",
      Self::DebugPrintExternalPointerTableFilterTag => "kDebugPrintExternalPointerTableFilterTag",
      Self::DebugPrintGeneric => "kDebugPrintGeneric",
      Self::DebugPrintFloat => "kDebugPrintFloat",
      Self::DebugPrintPtr => "kDebugPrintPtr",
      Self::DebugPrintWord => "kDebugPrintWord",
      Self::DebugTrace => "kDebugTrace",
      Self::DebugTraceMinimal => "kDebugTraceMinimal",
      Self::DeoptimizeFunction => "kDeoptimizeFunction",
      Self::DisableOptimizationFinalization => "kDisableOptimizationFinalization",
      Self::DisallowCodegenFromStrings => "kDisallowCodegenFromStrings",
      Self::DisassembleFunction => "kDisassembleFunction",
      Self::EnableCodeLoggingForTesting => "kEnableCodeLoggingForTesting",
      Self::EnsureFeedbackVectorForFunction => "kEnsureFeedbackVectorForFunction",
      Self::FinalizeOptimization => "kFinalizeOptimization",
      Self::ForceFlush => "kForceFlush",
      Self::MajorGCForCompilerTesting => "kMajorGCForCompilerTesting",
      Self::GetAbstractModuleSource => "kGetAbstractModuleSource",
      Self::GetBytecode => "kGetBytecode",
      Self::ExhaustInterruptBudget => "kExhaustInterruptBudget",
      Self::GetCallable => "kGetCallable",
      Self::GetFeedback => "kGetFeedback",
      Self::GetFunctionForCurrentFrame => "kGetFunctionForCurrentFrame",
      Self::GetInitializerFunction => "kGetInitializerFunction",
      Self::GetOptimizationStatus => "kGetOptimizationStatus",
      Self::GetUndetectable => "kGetUndetectable",
      Self::GetWeakCollectionSize => "kGetWeakCollectionSize",
      Self::GlobalPrint => "kGlobalPrint",
      Self::HasCowElements => "kHasCowElements",
      Self::HasDictionaryElements => "kHasDictionaryElements",
      Self::HasDoubleElements => "kHasDoubleElements",
      Self::HasElementsInALargeObjectSpace => "kHasElementsInALargeObjectSpace",
      Self::HasFastElements => "kHasFastElements",
      Self::HasFastProperties => "kHasFastProperties",
      Self::HasFixedBigInt64Elements => "kHasFixedBigInt64Elements",
      Self::HasFixedBigUint64Elements => "kHasFixedBigUint64Elements",
      Self::HasFixedFloat16Elements => "kHasFixedFloat16Elements",
      Self::HasFixedFloat32Elements => "kHasFixedFloat32Elements",
      Self::HasFixedFloat64Elements => "kHasFixedFloat64Elements",
      Self::HasFixedInt16Elements => "kHasFixedInt16Elements",
      Self::HasFixedInt32Elements => "kHasFixedInt32Elements",
      Self::HasFixedInt8Elements => "kHasFixedInt8Elements",
      Self::HasFixedUint16Elements => "kHasFixedUint16Elements",
      Self::HasFixedUint32Elements => "kHasFixedUint32Elements",
      Self::HasFixedUint8ClampedElements => "kHasFixedUint8ClampedElements",
      Self::HasFixedUint8Elements => "kHasFixedUint8Elements",
      Self::HasHoleyElements => "kHasHoleyElements",
      Self::HasObjectElements => "kHasObjectElements",
      Self::HasPackedElements => "kHasPackedElements",
      Self::HasSloppyArgumentsElements => "kHasSloppyArgumentsElements",
      Self::HasSmiElements => "kHasSmiElements",
      Self::HasSmiOrObjectElements => "kHasSmiOrObjectElements",
      Self::HaveSameMap => "kHaveSameMap",
      Self::HeapObjectVerify => "kHeapObjectVerify",
      Self::ICsAreEnabled => "kICsAreEnabled",
      Self::InLargeObjectSpace => "kInLargeObjectSpace",
      Self::InstallBytecode => "kInstallBytecode",
      Self::InYoungGeneration => "kInYoungGeneration",
      Self::Is64Bit => "kIs64Bit",
      Self::IsAtomicsWaitAllowed => "kIsAtomicsWaitAllowed",
      Self::IsBeingInterpreted => "kIsBeingInterpreted",
      Self::IsConcatSpreadableProtector => "kIsConcatSpreadableProtector",
      Self::IsConcurrentRecompilationSupported => "kIsConcurrentRecompilationSupported",
      Self::IsDictPropertyConstTrackingEnabled => "kIsDictPropertyConstTrackingEnabled",
      Self::IsEfficiencyModeEnabled => "kIsEfficiencyModeEnabled",
      Self::IsInPlaceInternalizableString => "kIsInPlaceInternalizableString",
      Self::IsInternalizedString => "kIsInternalizedString",
      Self::StringToCString => "kStringToCString",
      Self::StringUtf8Value => "kStringUtf8Value",
      Self::IsUndefinedDoubleEnabled => "kIsUndefinedDoubleEnabled",
      Self::IsMaglevEnabled => "kIsMaglevEnabled",
      Self::IsSameHeapObject => "kIsSameHeapObject",
      Self::IsSharedString => "kIsSharedString",
      Self::IsInWritableSharedSpace => "kIsInWritableSharedSpace",
      Self::IsSparkplugEnabled => "kIsSparkplugEnabled",
      Self::IsTurbofanEnabled => "kIsTurbofanEnabled",
      Self::IsWasmTieringPredictable => "kIsWasmTieringPredictable",
      Self::MapIteratorProtector => "kMapIteratorProtector",
      Self::NeverOptimizeFunction => "kNeverOptimizeFunction",
      Self::NewRegExpWithBacktrackLimit => "kNewRegExpWithBacktrackLimit",
      Self::NoElementsProtector => "kNoElementsProtector",
      Self::NotifyContextDisposed => "kNotifyContextDisposed",
      Self::SetPriorityBestEffort => "kSetPriorityBestEffort",
      Self::SetPriorityUserVisible => "kSetPriorityUserVisible",
      Self::SetPriorityUserBlocking => "kSetPriorityUserBlocking",
      Self::OptimizeMaglevOnNextCall => "kOptimizeMaglevOnNextCall",
      Self::OptimizeFunctionOnNextCall => "kOptimizeFunctionOnNextCall",
      Self::OptimizeOsr => "kOptimizeOsr",
      Self::PrepareFunctionForOptimization => "kPrepareFunctionForOptimization",
      Self::PretenureAllocationSite => "kPretenureAllocationSite",
      Self::PrintWithNameForAssert => "kPrintWithNameForAssert",
      Self::PromiseSpeciesProtector => "kPromiseSpeciesProtector",
      Self::RegExpSpeciesProtector => "kRegExpSpeciesProtector",
      Self::RegexpHasBytecode => "kRegexpHasBytecode",
      Self::RegexpHasNativeCode => "kRegexpHasNativeCode",
      Self::RegexpIsUnmodified => "kRegexpIsUnmodified",
      Self::RegexpQuickCheckRejects => "kRegexpQuickCheckRejects",
      Self::RegexpTypeTag => "kRegexpTypeTag",
      Self::Resume => "kResume",
      Self::RunningInSimulator => "kRunningInSimulator",
      Self::RuntimeEvaluateREPL => "kRuntimeEvaluateREPL",
      Self::ScheduleGCInStackCheck => "kScheduleGCInStackCheck",
      Self::SerializeDeserializeNow => "kSerializeDeserializeNow",
      Self::SetAllocationTimeout => "kSetAllocationTimeout",
      Self::SetDispatchTableGCInterval => "kSetDispatchTableGCInterval",
      Self::SetBatterySaverMode => "kSetBatterySaverMode",
      Self::SetForceSlowPath => "kSetForceSlowPath",
      Self::SetIteratorProtector => "kSetIteratorProtector",
      Self::SharedGC => "kSharedGC",
      Self::ShareObject => "kShareObject",
      Self::SimulateNewspaceFull => "kSimulateNewspaceFull",
      Self::StringIsFlat => "kStringIsFlat",
      Self::StringIsOneByteRepresentation => "kStringIsOneByteRepresentation",
      Self::StringIteratorProtector => "kStringIteratorProtector",
      Self::StringWrapperToPrimitiveProtector => "kStringWrapperToPrimitiveProtector",
      Self::SystemBreak => "kSystemBreak",
      Self::TakeHeapSnapshot => "kTakeHeapSnapshot",
      Self::TraceEnter => "kTraceEnter",
      Self::TraceExit => "kTraceExit",
      Self::TurbofanStaticAssert => "kTurbofanStaticAssert",
      Self::AssertEscapeAnalysisElided => "kAssertEscapeAnalysisElided",
      Self::TypedArraySpeciesProtector => "kTypedArraySpeciesProtector",
      Self::WaitForBackgroundOptimization => "kWaitForBackgroundOptimization",
      Self::WaitUntilBlocked => "kWaitUntilBlocked",
      Self::DeoptimizeNow => "kDeoptimizeNow",
      Self::LeakHole => "kLeakHole",
      Self::GetHoleNaNLower => "kGetHoleNaNLower",
      Self::GetHoleNaNUpper => "kGetHoleNaNUpper",
      Self::GetHoleNaN => "kGetHoleNaN",
      Self::GetUndefinedNaN => "kGetUndefinedNaN",
      Self::ArrayBufferDetach => "kArrayBufferDetach",
      Self::ArrayBufferSetDetachKey => "kArrayBufferSetDetachKey",
      Self::GrowableSharedArrayBufferByteLength => "kGrowableSharedArrayBufferByteLength",
      Self::TypedArrayCopyElements => "kTypedArrayCopyElements",
      Self::TypedArrayGetBuffer => "kTypedArrayGetBuffer",
      Self::TypedArraySet => "kTypedArraySet",
      Self::TypedArraySortFast => "kTypedArraySortFast",
      Self::WasmRunInterpreter => "kWasmRunInterpreter",
      Self::ThrowWasmJSPISuspendError => "kThrowWasmJSPISuspendError",
      Self::ThrowWasmFXSuspendError => "kThrowWasmFXSuspendError",
      Self::ThrowWasmError => "kThrowWasmError",
      Self::ThrowWasmStackOverflow => "kThrowWasmStackOverflow",
      Self::WasmI32AtomicWait => "kWasmI32AtomicWait",
      Self::WasmI64AtomicWait => "kWasmI64AtomicWait",
      Self::WasmManagedObjectWait32 => "kWasmManagedObjectWait32",
      Self::WasmManagedObjectWait64 => "kWasmManagedObjectWait64",
      Self::WasmManagedObjectWaitRef => "kWasmManagedObjectWaitRef",
      Self::WasmWaitqueueNew => "kWasmWaitqueueNew",
      Self::WasmMemoryGrow => "kWasmMemoryGrow",
      Self::WasmStackGuard => "kWasmStackGuard",
      Self::WasmStackGuardLoop => "kWasmStackGuardLoop",
      Self::WasmThrow => "kWasmThrow",
      Self::WasmReThrow => "kWasmReThrow",
      Self::WasmThrowJSTypeError => "kWasmThrowJSTypeError",
      Self::WasmThrowTypeError => "kWasmThrowTypeError",
      Self::WasmThrowRangeError => "kWasmThrowRangeError",
      Self::WasmThrowDataViewTypeError => "kWasmThrowDataViewTypeError",
      Self::WasmThrowDataViewDetachedError => "kWasmThrowDataViewDetachedError",
      Self::WasmRefFunc => "kWasmRefFunc",
      Self::WasmInternalFunctionCreateExternal => "kWasmInternalFunctionCreateExternal",
      Self::WasmFunctionTableGet => "kWasmFunctionTableGet",
      Self::WasmFunctionTableSet => "kWasmFunctionTableSet",
      Self::WasmTableInit => "kWasmTableInit",
      Self::WasmTableCopy => "kWasmTableCopy",
      Self::WasmTableGrow => "kWasmTableGrow",
      Self::WasmTableFill => "kWasmTableFill",
      Self::WasmJSToWasmObject => "kWasmJSToWasmObject",
      Self::WasmWasmToJSObject => "kWasmWasmToJSObject",
      Self::WasmCompileLazy => "kWasmCompileLazy",
      Self::WasmAllocateFeedbackVector => "kWasmAllocateFeedbackVector",
      Self::WasmLiftoffDeoptFinish => "kWasmLiftoffDeoptFinish",
      Self::TierUpJSToWasmWrapper => "kTierUpJSToWasmWrapper",
      Self::IsWasmExternalFunction => "kIsWasmExternalFunction",
      Self::TierUpWasmToJSWrapper => "kTierUpWasmToJSWrapper",
      Self::WasmTriggerTierUp => "kWasmTriggerTierUp",
      Self::WasmDebugBreak => "kWasmDebugBreak",
      Self::WasmAllocateDescriptorStruct => "kWasmAllocateDescriptorStruct",
      Self::WasmArrayCopy => "kWasmArrayCopy",
      Self::WasmArrayNewSegment => "kWasmArrayNewSegment",
      Self::WasmArrayInitSegment => "kWasmArrayInitSegment",
      Self::WasmAllocateSuspender => "kWasmAllocateSuspender",
      Self::WasmSuspended => "kWasmSuspended",
      Self::WasmAllocateContinuation => "kWasmAllocateContinuation",
      Self::ClearWasmSuspenderResumeField => "kClearWasmSuspenderResumeField",
      Self::WasmCastToSpecialPrimitiveArray => "kWasmCastToSpecialPrimitiveArray",
      Self::WasmStringAddNoMapCheckShared => "kWasmStringAdd_NoMapCheck_Shared",
      Self::WasmStringNewSegmentWtf8 => "kWasmStringNewSegmentWtf8",
      Self::WasmStringNewWtf8 => "kWasmStringNewWtf8",
      Self::WasmStringNewWtf8Array => "kWasmStringNewWtf8Array",
      Self::WasmStringNewWtf16 => "kWasmStringNewWtf16",
      Self::WasmStringNewWtf16Array => "kWasmStringNewWtf16Array",
      Self::WasmStringConst => "kWasmStringConst",
      Self::WasmStringMeasureUtf8 => "kWasmStringMeasureUtf8",
      Self::WasmStringMeasureWtf8 => "kWasmStringMeasureWtf8",
      Self::WasmStringEncodeWtf8 => "kWasmStringEncodeWtf8",
      Self::WasmStringEncodeWtf16 => "kWasmStringEncodeWtf16",
      Self::WasmStringEncodeWtf8Array => "kWasmStringEncodeWtf8Array",
      Self::WasmStringToUtf8Array => "kWasmStringToUtf8Array",
      Self::WasmStringAsWtf8 => "kWasmStringAsWtf8",
      Self::WasmStringViewWtf8Encode => "kWasmStringViewWtf8Encode",
      Self::WasmStringViewWtf8Slice => "kWasmStringViewWtf8Slice",
      Self::WasmStringFromCodePoint => "kWasmStringFromCodePoint",
      Self::WasmStringHash => "kWasmStringHash",
      Self::WasmSubstring => "kWasmSubstring",
      Self::WasmSubstringShared => "kWasmSubstringShared",
      Self::WasmConfigureAllPrototypes => "kWasmConfigureAllPrototypes",
      Self::WasmConfigureAllPrototypesOpt => "kWasmConfigureAllPrototypesOpt",
      Self::DebugCollectWasmCoverage => "kDebugCollectWasmCoverage",
      Self::TsanAcquireForInitializationFence => "kTsanAcquireForInitializationFence",
      Self::WasmTypeAssertionFailed => "kWasmTypeAssertionFailed",
      Self::BuildRefTypeBitfield => "kBuildRefTypeBitfield",
      Self::CheckIsOnCentralStack => "kCheckIsOnCentralStack",
      Self::CountUnoptimizedWasmToJSWrapper => "kCountUnoptimizedWasmToJSWrapper",
      Self::DisallowWasmCodegen => "kDisallowWasmCodegen",
      Self::EstimateCurrentMemoryConsumption => "kEstimateCurrentMemoryConsumption",
      Self::FlushLiftoffCode => "kFlushLiftoffCode",
      Self::FreezeWasmLazyCompilation => "kFreezeWasmLazyCompilation",
      Self::GenerateWasmCompilationHints => "kGenerateWasmCompilationHints",
      Self::GetWasmExceptionTagId => "kGetWasmExceptionTagId",
      Self::GetWasmExceptionValues => "kGetWasmExceptionValues",
      Self::GetWasmRecoveredTrapCount => "kGetWasmRecoveredTrapCount",
      Self::HasUnoptimizedWasmToJSWrapper => "kHasUnoptimizedWasmToJSWrapper",
      Self::IsLiftoffFunction => "kIsLiftoffFunction",
      Self::IsTurboFanFunction => "kIsTurboFanFunction",
      Self::IsUncompiledWasmFunction => "kIsUncompiledWasmFunction",
      Self::IsWasmCode => "kIsWasmCode",
      Self::IsWasmDebugFunction => "kIsWasmDebugFunction",
      Self::IsWasmPartialOOBWriteNoop => "kIsWasmPartialOOBWriteNoop",
      Self::IsWasmTrapHandlerEnabled => "kIsWasmTrapHandlerEnabled",
      Self::SetWasmCompileControls => "kSetWasmCompileControls",
      Self::SetWasmInstantiateControls => "kSetWasmInstantiateControls",
      Self::WasmArray => "kWasmArray",
      Self::WasmCompiledExportWrappersCount => "kWasmCompiledExportWrappersCount",
      Self::WasmDeoptsExecutedCount => "kWasmDeoptsExecutedCount",
      Self::WasmDeoptsExecutedForFunction => "kWasmDeoptsExecutedForFunction",
      Self::WasmEnterDebugging => "kWasmEnterDebugging",
      Self::WasmGenerateRandomModule => "kWasmGenerateRandomModule",
      Self::WasmGetNumberOfInstances => "kWasmGetNumberOfInstances",
      Self::WasmLeaveDebugging => "kWasmLeaveDebugging",
      Self::WasmNumCodeSpaces => "kWasmNumCodeSpaces",
      Self::WasmStruct => "kWasmStruct",
      Self::WasmSwitchToTheCentralStackCount => "kWasmSwitchToTheCentralStackCount",
      Self::WasmTierUpFunction => "kWasmTierUpFunction",
      Self::WasmTraceEnter => "kWasmTraceEnter",
      Self::WasmTraceExit => "kWasmTraceExit",
      Self::WasmTraceMemory => "kWasmTraceMemory",
      Self::WasmTriggerCodeGC => "kWasmTriggerCodeGC",
      Self::WasmTraceGlobal => "kWasmTraceGlobal",
      Self::WasmTriggerTierUpForTesting => "kWasmTriggerTierUpForTesting",
      Self::WasmTraceBeginExecution => "kWasmTraceBeginExecution",
      Self::WasmTraceEndExecution => "kWasmTraceEndExecution",
      Self::JSFinalizationRegistryRegisterWeakCellWithUnregisterToken => {
        "kJSFinalizationRegistryRegisterWeakCellWithUnregisterToken"
      }
      Self::JSWeakRefAddToKeptObjects => "kJSWeakRefAddToKeptObjects",
      Self::DebugBreakOnBytecode => "kDebugBreakOnBytecode",
      Self::LoadLookupSlotForCall => "kLoadLookupSlotForCall",
      Self::ElementsTransitionAndStoreICMiss => "kElementsTransitionAndStoreIC_Miss",
      Self::KeyedLoadICMiss => "kKeyedLoadIC_Miss",
      Self::KeyedStoreICMiss => "kKeyedStoreIC_Miss",
      Self::DefineKeyedOwnICMiss => "kDefineKeyedOwnIC_Miss",
      Self::StoreInArrayLiteralICMiss => "kStoreInArrayLiteralIC_Miss",
      Self::DefineNamedOwnICSlow => "kDefineNamedOwnIC_Slow",
      Self::KeyedStoreICSlow => "kKeyedStoreIC_Slow",
      Self::DefineKeyedOwnICSlow => "kDefineKeyedOwnIC_Slow",
      Self::LoadElementWithInterceptor => "kLoadElementWithInterceptor",
      Self::LoadGlobalICMiss => "kLoadGlobalIC_Miss",
      Self::LoadGlobalICSlow => "kLoadGlobalIC_Slow",
      Self::LoadICMiss => "kLoadIC_Miss",
      Self::LoadICMissFromBaseline => "kLoadIC_Miss_FromBaseline",
      Self::PatchLoadICUninitializedBaseline => "kPatchLoadICUninitializedBaseline",
      Self::GetStringLengthAndUpdateFeedback => "kGetStringLengthAndUpdateFeedback",
      Self::LoadNoFeedbackICMiss => "kLoadNoFeedbackIC_Miss",
      Self::LoadWithReceiverICMiss => "kLoadWithReceiverIC_Miss",
      Self::LoadWithReceiverNoFeedbackICMiss => "kLoadWithReceiverNoFeedbackIC_Miss",
      Self::LoadPropertyPastInterceptor => "kLoadPropertyPastInterceptor",
      Self::StoreCallbackProperty => "kStoreCallbackProperty",
      Self::StoreGlobalICMiss => "kStoreGlobalIC_Miss",
      Self::StoreGlobalICNoFeedbackMiss => "kStoreGlobalICNoFeedback_Miss",
      Self::StoreGlobalICSlow => "kStoreGlobalIC_Slow",
      Self::StoreICMiss => "kStoreIC_Miss",
      Self::DefineNamedOwnICMiss => "kDefineNamedOwnIC_Miss",
      Self::StoreInArrayLiteralICSlow => "kStoreInArrayLiteralIC_Slow",
      Self::StorePropertyPastInterceptor => "kStorePropertyPastInterceptor",
      Self::CloneObjectICSlow => "kCloneObjectIC_Slow",
      Self::CloneObjectICMiss => "kCloneObjectIC_Miss",
      Self::KeyedHasICMiss => "kKeyedHasIC_Miss",
      Self::HasElementWithInterceptor => "kHasElementWithInterceptor",
      Self::ObjectAssignTryFastcase => "kObjectAssignTryFastcase",
      Self::NumFunctions => "kNumFunctions",
    }
  }

  pub fn descriptor(&self) -> Option<&'static RuntimeFunction> {
    find_runtime_function(self.name())
  }
}

/// Runtime 函数描述符
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeFunction {
  pub id: RuntimeFunctionId,
  pub name: &'static str,
  pub nargs: i8,
  pub result_size: i8,
}

/// Runtime 函数全量总表
pub const RUNTIME_FUNCTION_COUNT: usize = 714;
pub const RUNTIME_FUNCTIONS: &[RuntimeFunction] = &[
  RuntimeFunction {
    id: RuntimeFunctionId::ArrayIncludesSlow,
    name: "ArrayIncludes_Slow",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ArrayIndexOf,
    name: "ArrayIndexOf",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ArrayIsArray,
    name: "ArrayIsArray",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ArraySpeciesConstructor,
    name: "ArraySpeciesConstructor",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GrowArrayElements,
    name: "GrowArrayElements",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsArray,
    name: "IsArray",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewArray,
    name: "NewArray",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NormalizeElements,
    name: "NormalizeElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TransitionElementsKind,
    name: "TransitionElementsKind",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TransitionElementsKindWithKind,
    name: "TransitionElementsKindWithKind",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsLoad64,
    name: "AtomicsLoad64",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsStore64,
    name: "AtomicsStore64",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsAdd,
    name: "AtomicsAdd",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsAnd,
    name: "AtomicsAnd",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsCompareExchange,
    name: "AtomicsCompareExchange",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsExchange,
    name: "AtomicsExchange",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsNumWaitersForTesting,
    name: "AtomicsNumWaitersForTesting",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsNumUnresolvedAsyncPromisesForTesting,
    name: "AtomicsNumUnresolvedAsyncPromisesForTesting",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsOr,
    name: "AtomicsOr",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsSub,
    name: "AtomicsSub",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsXor,
    name: "AtomicsXor",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetAllowAtomicsWait,
    name: "SetAllowAtomicsWait",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsLoadSharedStructOrArray,
    name: "AtomicsLoadSharedStructOrArray",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsStoreSharedStructOrArray,
    name: "AtomicsStoreSharedStructOrArray",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsExchangeSharedStructOrArray,
    name: "AtomicsExchangeSharedStructOrArray",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsCompareExchangeSharedStructOrArray,
    name: "AtomicsCompareExchangeSharedStructOrArray",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AtomicsSynchronizationPrimitiveNumWaitersForTesting,
    name: "AtomicsSynchronizationPrimitiveNumWaitersForTesting",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BigIntCompareToNumber,
    name: "BigIntCompareToNumber",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BigIntCompareToString,
    name: "BigIntCompareToString",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BigIntEqualToBigInt,
    name: "BigIntEqualToBigInt",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BigIntEqualToNumber,
    name: "BigIntEqualToNumber",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BigIntEqualToString,
    name: "BigIntEqualToString",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BigIntExponentiate,
    name: "BigIntExponentiate",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BigIntMaxBits,
    name: "BigIntMaxBits",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BigIntToNumber,
    name: "BigIntToNumber",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BigIntUnaryOp,
    name: "BigIntUnaryOp",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ToBigInt,
    name: "ToBigInt",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ToBigIntConvertNumber,
    name: "ToBigIntConvertNumber",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowConstructorNonCallableError,
    name: "ThrowConstructorNonCallableError",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowNotSuperConstructor,
    name: "ThrowNotSuperConstructor",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowStaticPrototypeError,
    name: "ThrowStaticPrototypeError",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowSuperAlreadyCalledError,
    name: "ThrowSuperAlreadyCalledError",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowSuperNotCalled,
    name: "ThrowSuperNotCalled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowUnsupportedSuperError,
    name: "ThrowUnsupportedSuperError",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DefineClass,
    name: "DefineClass",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadFromSuper,
    name: "LoadFromSuper",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadKeyedFromSuper,
    name: "LoadKeyedFromSuper",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreKeyedToSuper,
    name: "StoreKeyedToSuper",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreToSuper,
    name: "StoreToSuper",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::MapGrow,
    name: "MapGrow",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::MapShrink,
    name: "MapShrink",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::OrderedHashSetGrow,
    name: "OrderedHashSetGrow",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetGrow,
    name: "SetGrow",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetShrink,
    name: "SetShrink",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::OrderedHashSetShrink,
    name: "OrderedHashSetShrink",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TheHole,
    name: "TheHole",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WeakCollectionDelete,
    name: "WeakCollectionDelete",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WeakCollectionSet,
    name: "WeakCollectionSet",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::OrderedHashMapGrow,
    name: "OrderedHashMapGrow",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PrintDumpedFrame,
    name: "PrintDumpedFrame",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CompileOptimizedOSR,
    name: "CompileOptimizedOSR",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CompileOptimizedOSRFromMaglev,
    name: "CompileOptimizedOSRFromMaglev",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CompileOptimizedOSRFromMaglevInlined,
    name: "CompileOptimizedOSRFromMaglevInlined",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LogOrTraceOptimizedOSREntry,
    name: "LogOrTraceOptimizedOSREntry",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CompileLazy,
    name: "CompileLazy",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CompileBaseline,
    name: "CompileBaseline",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::InstallBaselineCode,
    name: "InstallBaselineCode",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::InstallSFICode,
    name: "InstallSFICode",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NotifyDeoptimized,
    name: "NotifyDeoptimized",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObserveNode,
    name: "ObserveNode",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ResolvePossiblyDirectEval,
    name: "ResolvePossiblyDirectEval",
    nargs: 6,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::VerifyType,
    name: "VerifyType",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CheckTurboshaftTypeOf,
    name: "CheckTurboshaftTypeOf",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PatchCompareOpBaselineCode,
    name: "PatchCompareOpBaselineCode",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PatchCompareOpBaselineCodeAndThrow,
    name: "PatchCompareOpBaselineCodeAndThrow",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PatchBinopBaselineCode,
    name: "PatchBinopBaselineCode",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PatchBinopBaselineCodeAndThrow,
    name: "PatchBinopBaselineCodeAndThrow",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PatchUnaryOpBaselineCode,
    name: "PatchUnaryOpBaselineCode",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PatchUnaryOpBaselineCodeAndThrow,
    name: "PatchUnaryOpBaselineCodeAndThrow",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FunctionLogNextExecution,
    name: "FunctionLogNextExecution",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::OptimizeMaglevEager,
    name: "OptimizeMaglevEager",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StartMaglevOptimizeJob,
    name: "StartMaglevOptimizeJob",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::OptimizeTurbofanEager,
    name: "OptimizeTurbofanEager",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StartTurbofanOptimizeJob,
    name: "StartTurbofanOptimizeJob",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::MarkLazyDeoptimizedOrFlushed,
    name: "MarkLazyDeoptimizedOrFlushed",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DateCurrentTime,
    name: "DateCurrentTime",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ClearStepping,
    name: "ClearStepping",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CollectGarbage,
    name: "CollectGarbage",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugAsyncFunctionSuspended,
    name: "DebugAsyncFunctionSuspended",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugBreakAtEntry,
    name: "DebugBreakAtEntry",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugCollectCoverage,
    name: "DebugCollectCoverage",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugGetLoadedScriptIds,
    name: "DebugGetLoadedScriptIds",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugOnFunctionCall,
    name: "DebugOnFunctionCall",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPrepareStepInSuspendedGenerator,
    name: "DebugPrepareStepInSuspendedGenerator",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPromiseThen,
    name: "DebugPromiseThen",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugToggleBlockCoverage,
    name: "DebugToggleBlockCoverage",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugTogglePreciseCoverage,
    name: "DebugTogglePreciseCoverage",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FunctionGetInferredName,
    name: "FunctionGetInferredName",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetBreakLocations,
    name: "GetBreakLocations",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetGeneratorScopeCount,
    name: "GetGeneratorScopeCount",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetGeneratorScopeDetails,
    name: "GetGeneratorScopeDetails",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HandleDebuggerStatement,
    name: "HandleDebuggerStatement",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsBreakOnException,
    name: "IsBreakOnException",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IterableForEach,
    name: "IterableForEach",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ProfileCreateSnapshotDataBlob,
    name: "ProfileCreateSnapshotDataBlob",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ScheduleBreak,
    name: "ScheduleBreak",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ScriptLocationFromLine2,
    name: "ScriptLocationFromLine2",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetGeneratorScopeVariableValue,
    name: "SetGeneratorScopeVariableValue",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IncBlockCounter,
    name: "IncBlockCounter",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ForInEnumerate,
    name: "ForInEnumerate",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ForInHasProperty,
    name: "ForInHasProperty",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TraceUnoptimizedBytecodeEntry,
    name: "TraceUnoptimizedBytecodeEntry",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TraceUnoptimizedBytecodeExit,
    name: "TraceUnoptimizedBytecodeExit",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DumpExecutionFrame,
    name: "DumpExecutionFrame",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TraceUpdateFeedback,
    name: "TraceUpdateFeedback",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::Call,
    name: "Call",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FunctionGetScriptSource,
    name: "FunctionGetScriptSource",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FunctionGetScriptId,
    name: "FunctionGetScriptId",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FunctionGetScriptSourcePosition,
    name: "FunctionGetScriptSourcePosition",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FunctionGetSourceCode,
    name: "FunctionGetSourceCode",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FunctionIsAPIFunction,
    name: "FunctionIsAPIFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AsyncFunctionAwait,
    name: "AsyncFunctionAwait",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AsyncFunctionEnter,
    name: "AsyncFunctionEnter",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AsyncFunctionReject,
    name: "AsyncFunctionReject",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AsyncFunctionResolve,
    name: "AsyncFunctionResolve",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AsyncGeneratorAwait,
    name: "AsyncGeneratorAwait",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AsyncGeneratorReject,
    name: "AsyncGeneratorReject",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AsyncGeneratorResolve,
    name: "AsyncGeneratorResolve",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AsyncGeneratorYieldWithAwait,
    name: "AsyncGeneratorYieldWithAwait",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreateJSGeneratorObject,
    name: "CreateJSGeneratorObject",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GeneratorClose,
    name: "GeneratorClose",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GeneratorGetFunction,
    name: "GeneratorGetFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GeneratorGetResumeMode,
    name: "GeneratorGetResumeMode",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FormatList,
    name: "FormatList",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FormatListToParts,
    name: "FormatListToParts",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringToLowerCaseIntl,
    name: "StringToLowerCaseIntl",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringToLocaleLowerCase,
    name: "StringToLocaleLowerCase",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringToUpperCaseIntl,
    name: "StringToUpperCaseIntl",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::Throw,
    name: "Throw",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowTargetNonFunction,
    name: "ThrowTargetNonFunction",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowCalledNonCallable,
    name: "ThrowCalledNonCallable",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowConstructedNonConstructable,
    name: "ThrowConstructedNonConstructable",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowConstructorReturnedNonObject,
    name: "ThrowConstructorReturnedNonObject",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowInvalidStringLength,
    name: "ThrowInvalidStringLength",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowInvalidTypedArrayAlignment,
    name: "ThrowInvalidTypedArrayAlignment",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowIteratorError,
    name: "ThrowIteratorError",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowSpreadArgError,
    name: "ThrowSpreadArgError",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowIteratorResultNotAnObject,
    name: "ThrowIteratorResultNotAnObject",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowNoAccess,
    name: "ThrowNoAccess",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowNotConstructor,
    name: "ThrowNotConstructor",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowPatternAssignmentNonCoercible,
    name: "ThrowPatternAssignmentNonCoercible",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowRangeError,
    name: "ThrowRangeError",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowReferenceError,
    name: "ThrowReferenceError",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowAccessedUninitializedVariable,
    name: "ThrowAccessedUninitializedVariable",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::VarargStackOverflow,
    name: "VarargStackOverflow",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowStackOverflow,
    name: "ThrowStackOverflow",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowSymbolAsyncIteratorInvalid,
    name: "ThrowSymbolAsyncIteratorInvalid",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowSymbolIteratorInvalid,
    name: "ThrowSymbolIteratorInvalid",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowThrowMethodMissing,
    name: "ThrowThrowMethodMissing",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowTypeError,
    name: "ThrowTypeError",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowTypeErrorIfStrict,
    name: "ThrowTypeErrorIfStrict",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ReThrow,
    name: "ReThrow",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ReThrowWithMessage,
    name: "ReThrowWithMessage",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AccessCheck,
    name: "AccessCheck",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AllocateByteArray,
    name: "AllocateByteArray",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AllocateInYoungGeneration,
    name: "AllocateInYoungGeneration",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AllocateInOldGeneration,
    name: "AllocateInOldGeneration",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AllocateInSharedHeap,
    name: "AllocateInSharedHeap",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AllowDynamicFunction,
    name: "AllowDynamicFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreateAsyncFromSyncIterator,
    name: "CreateAsyncFromSyncIterator",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreateListFromArrayLike,
    name: "CreateListFromArrayLike",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DoubleToStringWithRadix,
    name: "DoubleToStringWithRadix",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FatalProcessOutOfMemoryInAllocateRaw,
    name: "FatalProcessOutOfMemoryInAllocateRaw",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FatalProcessOutOfMemoryInvalidArrayLength,
    name: "FatalProcessOutOfMemoryInvalidArrayLength",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FatalInvalidSize,
    name: "FatalInvalidSize",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetAndResetRuntimeCallStats,
    name: "GetAndResetRuntimeCallStats",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetAndResetTurboProfilingData,
    name: "GetAndResetTurboProfilingData",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetTemplateObject,
    name: "GetTemplateObject",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IncrementUseCounter,
    name: "IncrementUseCounter",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::InvalidateStringWrapperToPrimitiveProtector,
    name: "InvalidateStringWrapperToPrimitiveProtector",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BytecodeBudgetInterruptIgnition,
    name: "BytecodeBudgetInterrupt_Ignition",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BytecodeBudgetInterruptWithStackCheckIgnition,
    name: "BytecodeBudgetInterruptWithStackCheck_Ignition",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BytecodeBudgetInterruptSparkplug,
    name: "BytecodeBudgetInterrupt_Sparkplug",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BytecodeBudgetInterruptWithStackCheckSparkplug,
    name: "BytecodeBudgetInterruptWithStackCheck_Sparkplug",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BytecodeBudgetInterruptMaglev,
    name: "BytecodeBudgetInterrupt_Maglev",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BytecodeBudgetLoopInterruptMaglev,
    name: "BytecodeBudgetLoopInterrupt_Maglev",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NotifyContextCellStateWillChange,
    name: "NotifyContextCellStateWillChange",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewError,
    name: "NewError",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewReferenceError,
    name: "NewReferenceError",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewTypeError,
    name: "NewTypeError",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::OrdinaryHasInstance,
    name: "OrdinaryHasInstance",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PropagateException,
    name: "PropagateException",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ReportMessageFromMicrotask,
    name: "ReportMessageFromMicrotask",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RunMicrotaskCallback,
    name: "RunMicrotaskCallback",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PerformMicrotaskCheckpoint,
    name: "PerformMicrotaskCheckpoint",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SharedValueBarrierSlow,
    name: "SharedValueBarrierSlow",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StackGuard,
    name: "StackGuard",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HandleNoHeapWritesInterrupts,
    name: "HandleNoHeapWritesInterrupts",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StackGuardWithGap,
    name: "StackGuardWithGap",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TerminateExecution,
    name: "TerminateExecution",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::Typeof,
    name: "Typeof",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::UnwindAndFindExceptionHandler,
    name: "UnwindAndFindExceptionHandler",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AddLhsIsStringConstantInternalize,
    name: "AddLhsIsStringConstantInternalize",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AddRhsIsStringConstantInternalize,
    name: "AddRhsIsStringConstantInternalize",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreateArrayLiteral,
    name: "CreateArrayLiteral",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreateObjectLiteral,
    name: "CreateObjectLiteral",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetPrototypeProperties,
    name: "SetPrototypeProperties",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreateRegExpLiteral,
    name: "CreateRegExpLiteral",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DynamicImportCall,
    name: "DynamicImportCall",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetImportMetaObject,
    name: "GetImportMetaObject",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetModuleNamespace,
    name: "GetModuleNamespace",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetModuleNamespaceExport,
    name: "GetModuleNamespaceExport",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ArrayBufferMaxByteLength,
    name: "ArrayBufferMaxByteLength",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsSmi,
    name: "IsSmi",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::MaxSmi,
    name: "MaxSmi",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NumberToStringSlow,
    name: "NumberToStringSlow",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::Float64ToStringSlow,
    name: "Float64ToStringSlow",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringParseFloat,
    name: "StringParseFloat",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringParseInt,
    name: "StringParseInt",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringToNumber,
    name: "StringToNumber",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AddAsyncDisposableValue,
    name: "AddAsyncDisposableValue",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AddDictionaryProperty,
    name: "AddDictionaryProperty",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AddDisposableValue,
    name: "AddDisposableValue",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AddPrivateBrand,
    name: "AddPrivateBrand",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AllocateHeapNumber,
    name: "AllocateHeapNumber",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CompleteInobjectSlackTrackingForMap,
    name: "CompleteInobjectSlackTrackingForMap",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CopyDataProperties,
    name: "CopyDataProperties",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CopyDataPropertiesWithExcludedPropertiesOnStack,
    name: "CopyDataPropertiesWithExcludedPropertiesOnStack",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreateDataProperty,
    name: "CreateDataProperty",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreateIterResultObject,
    name: "CreateIterResultObject",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IterableToListWithInterceptor,
    name: "IterableToListWithInterceptor",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CheckFastIterableToListPrototype,
    name: "CheckFastIterableToListPrototype",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreatePrivateAccessors,
    name: "CreatePrivateAccessors",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DefineAccessorPropertyUnchecked,
    name: "DefineAccessorPropertyUnchecked",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DefineKeyedOwnPropertyInLiteral,
    name: "DefineKeyedOwnPropertyInLiteral",
    nargs: 6,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DefineGetterPropertyUnchecked,
    name: "DefineGetterPropertyUnchecked",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DefineSetterPropertyUnchecked,
    name: "DefineSetterPropertyUnchecked",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DeleteProperty,
    name: "DeleteProperty",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DisposeDisposableStack,
    name: "DisposeDisposableStack",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GeneratorYieldResult,
    name: "GeneratorYieldResult",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetDerivedMap,
    name: "GetDerivedMap",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetFunctionName,
    name: "GetFunctionName",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetOwnPropertyDescriptorObject,
    name: "GetOwnPropertyDescriptorObject",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetOwnPropertyKeys,
    name: "GetOwnPropertyKeys",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetPrivateMember,
    name: "GetPrivateMember",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetProperty,
    name: "GetProperty",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HandleExceptionsInDisposeDisposableStack,
    name: "HandleExceptionsInDisposeDisposableStack",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFastPackedElements,
    name: "HasFastPackedElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasInPrototypeChain,
    name: "HasInPrototypeChain",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasProperty,
    name: "HasProperty",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::InitializeDisposableStack,
    name: "InitializeDisposableStack",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::InternalSetPrototype,
    name: "InternalSetPrototype",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsJSReceiver,
    name: "IsJSReceiver",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::JSReceiverPreventExtensionsDontThrow,
    name: "JSReceiverPreventExtensionsDontThrow",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::JSReceiverPreventExtensionsThrow,
    name: "JSReceiverPreventExtensionsThrow",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::JSReceiverGetPrototypeOf,
    name: "JSReceiverGetPrototypeOf",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::JSReceiverSetPrototypeOfDontThrow,
    name: "JSReceiverSetPrototypeOfDontThrow",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::JSReceiverSetPrototypeOfThrow,
    name: "JSReceiverSetPrototypeOfThrow",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadPrivateGetter,
    name: "LoadPrivateGetter",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadPrivateSetter,
    name: "LoadPrivateSetter",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewObject,
    name: "NewObject",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectCreate,
    name: "ObjectCreate",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectEntries,
    name: "ObjectEntries",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectEntriesSkipFastPath,
    name: "ObjectEntriesSkipFastPath",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectGetOwnPropertyNames,
    name: "ObjectGetOwnPropertyNames",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectGetOwnPropertyNamesTryFast,
    name: "ObjectGetOwnPropertyNamesTryFast",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectHasOwnProperty,
    name: "ObjectHasOwnProperty",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasOwnConstDataProperty,
    name: "HasOwnConstDataProperty",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectIsExtensible,
    name: "ObjectIsExtensible",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectKeys,
    name: "ObjectKeys",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectValues,
    name: "ObjectValues",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectValuesSkipFastPath,
    name: "ObjectValuesSkipFastPath",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::OptimizeObjectForAddingMultipleProperties,
    name: "OptimizeObjectForAddingMultipleProperties",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetDataProperties,
    name: "SetDataProperties",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetFunctionName,
    name: "SetFunctionName",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetKeyedProperty,
    name: "SetKeyedProperty",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DefineObjectOwnProperty,
    name: "DefineObjectOwnProperty",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetNamedProperty,
    name: "SetNamedProperty",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetOwnPropertyIgnoreAttributes,
    name: "SetOwnPropertyIgnoreAttributes",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ShrinkNameDictionary,
    name: "ShrinkNameDictionary",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ShrinkSwissNameDictionary,
    name: "ShrinkSwissNameDictionary",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ToFastProperties,
    name: "ToFastProperties",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ToLength,
    name: "ToLength",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ToName,
    name: "ToName",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ToNumber,
    name: "ToNumber",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ToNumeric,
    name: "ToNumeric",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ToObject,
    name: "ToObject",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ToString,
    name: "ToString",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TryMigrateInstance,
    name: "TryMigrateInstance",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TryMigrateInstanceAndMarkMapAsMigrationTarget,
    name: "TryMigrateInstanceAndMarkMapAsMigrationTarget",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetPrivateMember,
    name: "SetPrivateMember",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SwissTableAdd,
    name: "SwissTableAdd",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SwissTableAllocate,
    name: "SwissTableAllocate",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SwissTableDelete,
    name: "SwissTableDelete",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SwissTableDetailsAt,
    name: "SwissTableDetailsAt",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SwissTableElementsCount,
    name: "SwissTableElementsCount",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SwissTableEquals,
    name: "SwissTableEquals",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SwissTableFindEntry,
    name: "SwissTableFindEntry",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SwissTableUpdate,
    name: "SwissTableUpdate",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SwissTableValueAt,
    name: "SwissTableValueAt",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SwissTableKeyAt,
    name: "SwissTableKeyAt",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::Add,
    name: "Add",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::Equal,
    name: "Equal",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GreaterThan,
    name: "GreaterThan",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GreaterThanOrEqual,
    name: "GreaterThanOrEqual",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LessThan,
    name: "LessThan",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LessThanOrEqual,
    name: "LessThanOrEqual",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NotEqual,
    name: "NotEqual",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StrictEqual,
    name: "StrictEqual",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StrictNotEqual,
    name: "StrictNotEqual",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ReferenceEqual,
    name: "ReferenceEqual",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::EnqueueMicrotask,
    name: "EnqueueMicrotask",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PromiseHookAfter,
    name: "PromiseHookAfter",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PromiseHookBefore,
    name: "PromiseHookBefore",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PromiseHookInit,
    name: "PromiseHookInit",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PromiseRejectEventFromStack,
    name: "PromiseRejectEventFromStack",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PromiseRevokeReject,
    name: "PromiseRevokeReject",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RejectPromise,
    name: "RejectPromise",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ResolvePromise,
    name: "ResolvePromise",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ConstructSuppressedError,
    name: "ConstructSuppressedError",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ConstructAggregateErrorHelper,
    name: "ConstructAggregateErrorHelper",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ConstructInternalAggregateErrorHelper,
    name: "ConstructInternalAggregateErrorHelper",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CheckProxyGetSetTrapResult,
    name: "CheckProxyGetSetTrapResult",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CheckProxyHasTrapResult,
    name: "CheckProxyHasTrapResult",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CheckProxyDeleteTrapResult,
    name: "CheckProxyDeleteTrapResult",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetPropertyWithReceiver,
    name: "GetPropertyWithReceiver",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsJSProxy,
    name: "IsJSProxy",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::JSProxyGetHandler,
    name: "JSProxyGetHandler",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::JSProxyGetTarget,
    name: "JSProxyGetTarget",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetPropertyWithReceiver,
    name: "SetPropertyWithReceiver",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpBuildIndices,
    name: "RegExpBuildIndices",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpGrowRegExpMatchInfo,
    name: "RegExpGrowRegExpMatchInfo",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpExecMultiple,
    name: "RegExpExecMultiple",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpInitializeAndCompile,
    name: "RegExpInitializeAndCompile",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpMatchGlobalAtom,
    name: "RegExpMatchGlobalAtom",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpReplaceRT,
    name: "RegExpReplaceRT",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpSplit,
    name: "RegExpSplit",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpStringFromFlags,
    name: "RegExpStringFromFlags",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringReplaceNonGlobalRegExpWithFunction,
    name: "StringReplaceNonGlobalRegExpWithFunction",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringSplit,
    name: "StringSplit",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpExec,
    name: "RegExpExec",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpExperimentalOneshotExec,
    name: "RegExpExperimentalOneshotExec",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowConstAssignError,
    name: "ThrowConstAssignError",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowUsingAssignError,
    name: "ThrowUsingAssignError",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowAwaitUsingAssignError,
    name: "ThrowAwaitUsingAssignError",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DeclareEvalFunction,
    name: "DeclareEvalFunction",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DeclareEvalVar,
    name: "DeclareEvalVar",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DeclareGlobals,
    name: "DeclareGlobals",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DeclareModuleExports,
    name: "DeclareModuleExports",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DeleteLookupSlot,
    name: "DeleteLookupSlot",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadLookupSlot,
    name: "LoadLookupSlot",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadLookupSlotInsideTypeof,
    name: "LoadLookupSlotInsideTypeof",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadLookupSlotForCallBaseline,
    name: "LoadLookupSlotForCall_Baseline",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewClosure,
    name: "NewClosure",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewClosureTenured,
    name: "NewClosure_Tenured",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewFunctionContext,
    name: "NewFunctionContext",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewRestParameter,
    name: "NewRestParameter",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewSloppyArguments,
    name: "NewSloppyArguments",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewStrictArguments,
    name: "NewStrictArguments",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PushBlockContext,
    name: "PushBlockContext",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PushCatchContext,
    name: "PushCatchContext",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PushWithContext,
    name: "PushWithContext",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreGlobalNoHoleCheckForReplLetOrConst,
    name: "StoreGlobalNoHoleCheckForReplLetOrConst",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreLookupSlotSloppy,
    name: "StoreLookupSlot_Sloppy",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreLookupSlotSloppyHoisting,
    name: "StoreLookupSlot_SloppyHoisting",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreLookupSlotStrict,
    name: "StoreLookupSlot_Strict",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ShadowRealmWrappedFunctionCreate,
    name: "ShadowRealmWrappedFunctionCreate",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ShadowRealmImportValue,
    name: "ShadowRealmImportValue",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ShadowRealmThrow,
    name: "ShadowRealmThrow",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FlattenString,
    name: "FlattenString",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetSubstitution,
    name: "GetSubstitution",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::InternalizeString,
    name: "InternalizeString",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringAdd,
    name: "StringAdd",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringAddLhsIsStringConstantInternalize,
    name: "StringAdd_LhsIsStringConstant_Internalize",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringAddRhsIsStringConstantInternalize,
    name: "StringAdd_RhsIsStringConstant_Internalize",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringBuilderConcat,
    name: "StringBuilderConcat",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringCharCodeAt,
    name: "StringCharCodeAt",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringCodePointAt,
    name: "StringCodePointAt",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringCompare,
    name: "StringCompare",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringEqual,
    name: "StringEqual",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringEscapeQuotes,
    name: "StringEscapeQuotes",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringGreaterThan,
    name: "StringGreaterThan",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringGreaterThanOrEqual,
    name: "StringGreaterThanOrEqual",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringIsWellFormed,
    name: "StringIsWellFormed",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringLastIndexOf,
    name: "StringLastIndexOf",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringLessThan,
    name: "StringLessThan",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringLessThanOrEqual,
    name: "StringLessThanOrEqual",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringMaxLength,
    name: "StringMaxLength",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringReplaceOneCharWithString,
    name: "StringReplaceOneCharWithString",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringSubstring,
    name: "StringSubstring",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringToArray,
    name: "StringToArray",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringToWellFormed,
    name: "StringToWellFormed",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreatePrivateNameSymbol,
    name: "CreatePrivateNameSymbol",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreatePrivateBrandSymbol,
    name: "CreatePrivateBrandSymbol",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CreatePrivateSymbol,
    name: "CreatePrivateSymbol",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SymbolDescriptiveString,
    name: "SymbolDescriptiveString",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SymbolIsPrivate,
    name: "SymbolIsPrivate",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::Abort,
    name: "Abort",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AbortCSADcheck,
    name: "AbortCSADcheck",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AbortJS,
    name: "AbortJS",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ActiveTierIsIgnition,
    name: "ActiveTierIsIgnition",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ActiveTierIsSparkplug,
    name: "ActiveTierIsSparkplug",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ActiveTierIsMaglev,
    name: "ActiveTierIsMaglev",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ActiveTierIsTurbofan,
    name: "ActiveTierIsTurbofan",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AllocateHeapNumberWithValue,
    name: "AllocateHeapNumberWithValue",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ArrayBufferDetachForceWasm,
    name: "ArrayBufferDetachForceWasm",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ArrayIteratorProtector,
    name: "ArrayIteratorProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ArraySpeciesProtector,
    name: "ArraySpeciesProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AssertNotPeeled,
    name: "AssertNotPeeled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AssertPeeled,
    name: "AssertPeeled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BaselineOsr,
    name: "BaselineOsr",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BenchMaglev,
    name: "BenchMaglev",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BenchTurbofan,
    name: "BenchTurbofan",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BlockAt,
    name: "BlockAt",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::VerifyGetJSBuiltinState,
    name: "VerifyGetJSBuiltinState",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ClearFunctionFeedback,
    name: "ClearFunctionFeedback",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ClearMegamorphicStubCache,
    name: "ClearMegamorphicStubCache",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CompleteInobjectSlackTracking,
    name: "CompleteInobjectSlackTracking",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ConstructConsString,
    name: "ConstructConsString",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ConstructDouble,
    name: "ConstructDouble",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ConstructInternalizedString,
    name: "ConstructInternalizedString",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ConstructSlicedString,
    name: "ConstructSlicedString",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ConstructThinString,
    name: "ConstructThinString",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CurrentFrameIsTurbofan,
    name: "CurrentFrameIsTurbofan",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPrint,
    name: "DebugPrint",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPrintCppHeapPointerTable,
    name: "DebugPrintCppHeapPointerTable",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPrintCppHeapPointerTableFilterTag,
    name: "DebugPrintCppHeapPointerTableFilterTag",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPrintExternalPointerTable,
    name: "DebugPrintExternalPointerTable",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPrintExternalPointerTableFilterTag,
    name: "DebugPrintExternalPointerTableFilterTag",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPrintGeneric,
    name: "DebugPrintGeneric",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPrintFloat,
    name: "DebugPrintFloat",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPrintPtr,
    name: "DebugPrintPtr",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugPrintWord,
    name: "DebugPrintWord",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugTrace,
    name: "DebugTrace",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugTraceMinimal,
    name: "DebugTraceMinimal",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DeoptimizeFunction,
    name: "DeoptimizeFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DisableOptimizationFinalization,
    name: "DisableOptimizationFinalization",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DisallowCodegenFromStrings,
    name: "DisallowCodegenFromStrings",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DisassembleFunction,
    name: "DisassembleFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::EnableCodeLoggingForTesting,
    name: "EnableCodeLoggingForTesting",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::EnsureFeedbackVectorForFunction,
    name: "EnsureFeedbackVectorForFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FinalizeOptimization,
    name: "FinalizeOptimization",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ForceFlush,
    name: "ForceFlush",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::MajorGCForCompilerTesting,
    name: "MajorGCForCompilerTesting",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetAbstractModuleSource,
    name: "GetAbstractModuleSource",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetBytecode,
    name: "GetBytecode",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ExhaustInterruptBudget,
    name: "ExhaustInterruptBudget",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetCallable,
    name: "GetCallable",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetFeedback,
    name: "GetFeedback",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetFunctionForCurrentFrame,
    name: "GetFunctionForCurrentFrame",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetInitializerFunction,
    name: "GetInitializerFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetOptimizationStatus,
    name: "GetOptimizationStatus",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetUndetectable,
    name: "GetUndetectable",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetWeakCollectionSize,
    name: "GetWeakCollectionSize",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GlobalPrint,
    name: "GlobalPrint",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasCowElements,
    name: "HasCowElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasDictionaryElements,
    name: "HasDictionaryElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasDoubleElements,
    name: "HasDoubleElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasElementsInALargeObjectSpace,
    name: "HasElementsInALargeObjectSpace",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFastElements,
    name: "HasFastElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFastProperties,
    name: "HasFastProperties",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedBigInt64Elements,
    name: "HasFixedBigInt64Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedBigUint64Elements,
    name: "HasFixedBigUint64Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedFloat16Elements,
    name: "HasFixedFloat16Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedFloat32Elements,
    name: "HasFixedFloat32Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedFloat64Elements,
    name: "HasFixedFloat64Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedInt16Elements,
    name: "HasFixedInt16Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedInt32Elements,
    name: "HasFixedInt32Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedInt8Elements,
    name: "HasFixedInt8Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedUint16Elements,
    name: "HasFixedUint16Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedUint32Elements,
    name: "HasFixedUint32Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedUint8ClampedElements,
    name: "HasFixedUint8ClampedElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasFixedUint8Elements,
    name: "HasFixedUint8Elements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasHoleyElements,
    name: "HasHoleyElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasObjectElements,
    name: "HasObjectElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasPackedElements,
    name: "HasPackedElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasSloppyArgumentsElements,
    name: "HasSloppyArgumentsElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasSmiElements,
    name: "HasSmiElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasSmiOrObjectElements,
    name: "HasSmiOrObjectElements",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HaveSameMap,
    name: "HaveSameMap",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HeapObjectVerify,
    name: "HeapObjectVerify",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ICsAreEnabled,
    name: "ICsAreEnabled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::InLargeObjectSpace,
    name: "InLargeObjectSpace",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::InstallBytecode,
    name: "InstallBytecode",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::InYoungGeneration,
    name: "InYoungGeneration",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::Is64Bit,
    name: "Is64Bit",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsAtomicsWaitAllowed,
    name: "IsAtomicsWaitAllowed",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsBeingInterpreted,
    name: "IsBeingInterpreted",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsConcatSpreadableProtector,
    name: "IsConcatSpreadableProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsConcurrentRecompilationSupported,
    name: "IsConcurrentRecompilationSupported",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsDictPropertyConstTrackingEnabled,
    name: "IsDictPropertyConstTrackingEnabled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsEfficiencyModeEnabled,
    name: "IsEfficiencyModeEnabled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsInPlaceInternalizableString,
    name: "IsInPlaceInternalizableString",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsInternalizedString,
    name: "IsInternalizedString",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringToCString,
    name: "StringToCString",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringUtf8Value,
    name: "StringUtf8Value",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsUndefinedDoubleEnabled,
    name: "IsUndefinedDoubleEnabled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsMaglevEnabled,
    name: "IsMaglevEnabled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsSameHeapObject,
    name: "IsSameHeapObject",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsSharedString,
    name: "IsSharedString",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsInWritableSharedSpace,
    name: "IsInWritableSharedSpace",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsSparkplugEnabled,
    name: "IsSparkplugEnabled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsTurbofanEnabled,
    name: "IsTurbofanEnabled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsWasmTieringPredictable,
    name: "IsWasmTieringPredictable",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::MapIteratorProtector,
    name: "MapIteratorProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NeverOptimizeFunction,
    name: "NeverOptimizeFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NewRegExpWithBacktrackLimit,
    name: "NewRegExpWithBacktrackLimit",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NoElementsProtector,
    name: "NoElementsProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::NotifyContextDisposed,
    name: "NotifyContextDisposed",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetPriorityBestEffort,
    name: "SetPriorityBestEffort",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetPriorityUserVisible,
    name: "SetPriorityUserVisible",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetPriorityUserBlocking,
    name: "SetPriorityUserBlocking",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::OptimizeMaglevOnNextCall,
    name: "OptimizeMaglevOnNextCall",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::OptimizeFunctionOnNextCall,
    name: "OptimizeFunctionOnNextCall",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::OptimizeOsr,
    name: "OptimizeOsr",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PrepareFunctionForOptimization,
    name: "PrepareFunctionForOptimization",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PretenureAllocationSite,
    name: "PretenureAllocationSite",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PrintWithNameForAssert,
    name: "PrintWithNameForAssert",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PromiseSpeciesProtector,
    name: "PromiseSpeciesProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegExpSpeciesProtector,
    name: "RegExpSpeciesProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegexpHasBytecode,
    name: "RegexpHasBytecode",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegexpHasNativeCode,
    name: "RegexpHasNativeCode",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegexpIsUnmodified,
    name: "RegexpIsUnmodified",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegexpQuickCheckRejects,
    name: "RegexpQuickCheckRejects",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RegexpTypeTag,
    name: "RegexpTypeTag",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::Resume,
    name: "Resume",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RunningInSimulator,
    name: "RunningInSimulator",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::RuntimeEvaluateREPL,
    name: "RuntimeEvaluateREPL",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ScheduleGCInStackCheck,
    name: "ScheduleGCInStackCheck",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SerializeDeserializeNow,
    name: "SerializeDeserializeNow",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetAllocationTimeout,
    name: "SetAllocationTimeout",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetDispatchTableGCInterval,
    name: "SetDispatchTableGCInterval",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetBatterySaverMode,
    name: "SetBatterySaverMode",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetForceSlowPath,
    name: "SetForceSlowPath",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetIteratorProtector,
    name: "SetIteratorProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SharedGC,
    name: "SharedGC",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ShareObject,
    name: "ShareObject",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SimulateNewspaceFull,
    name: "SimulateNewspaceFull",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringIsFlat,
    name: "StringIsFlat",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringIsOneByteRepresentation,
    name: "StringIsOneByteRepresentation",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringIteratorProtector,
    name: "StringIteratorProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StringWrapperToPrimitiveProtector,
    name: "StringWrapperToPrimitiveProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SystemBreak,
    name: "SystemBreak",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TakeHeapSnapshot,
    name: "TakeHeapSnapshot",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TraceEnter,
    name: "TraceEnter",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TraceExit,
    name: "TraceExit",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TurbofanStaticAssert,
    name: "TurbofanStaticAssert",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::AssertEscapeAnalysisElided,
    name: "AssertEscapeAnalysisElided",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TypedArraySpeciesProtector,
    name: "TypedArraySpeciesProtector",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WaitForBackgroundOptimization,
    name: "WaitForBackgroundOptimization",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WaitUntilBlocked,
    name: "WaitUntilBlocked",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DeoptimizeNow,
    name: "DeoptimizeNow",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LeakHole,
    name: "LeakHole",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetHoleNaNLower,
    name: "GetHoleNaNLower",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetHoleNaNUpper,
    name: "GetHoleNaNUpper",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetHoleNaN,
    name: "GetHoleNaN",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetUndefinedNaN,
    name: "GetUndefinedNaN",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ArrayBufferDetach,
    name: "ArrayBufferDetach",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ArrayBufferSetDetachKey,
    name: "ArrayBufferSetDetachKey",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GrowableSharedArrayBufferByteLength,
    name: "GrowableSharedArrayBufferByteLength",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TypedArrayCopyElements,
    name: "TypedArrayCopyElements",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TypedArrayGetBuffer,
    name: "TypedArrayGetBuffer",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TypedArraySet,
    name: "TypedArraySet",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TypedArraySortFast,
    name: "TypedArraySortFast",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmRunInterpreter,
    name: "WasmRunInterpreter",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowWasmJSPISuspendError,
    name: "ThrowWasmJSPISuspendError",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowWasmFXSuspendError,
    name: "ThrowWasmFXSuspendError",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowWasmError,
    name: "ThrowWasmError",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ThrowWasmStackOverflow,
    name: "ThrowWasmStackOverflow",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmI32AtomicWait,
    name: "WasmI32AtomicWait",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmI64AtomicWait,
    name: "WasmI64AtomicWait",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmManagedObjectWait32,
    name: "WasmManagedObjectWait32",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmManagedObjectWait64,
    name: "WasmManagedObjectWait64",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmManagedObjectWaitRef,
    name: "WasmManagedObjectWaitRef",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmWaitqueueNew,
    name: "WasmWaitqueueNew",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmMemoryGrow,
    name: "WasmMemoryGrow",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStackGuard,
    name: "WasmStackGuard",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStackGuardLoop,
    name: "WasmStackGuardLoop",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmThrow,
    name: "WasmThrow",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmReThrow,
    name: "WasmReThrow",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmThrowJSTypeError,
    name: "WasmThrowJSTypeError",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmThrowTypeError,
    name: "WasmThrowTypeError",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmThrowRangeError,
    name: "WasmThrowRangeError",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmThrowDataViewTypeError,
    name: "WasmThrowDataViewTypeError",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmThrowDataViewDetachedError,
    name: "WasmThrowDataViewDetachedError",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmRefFunc,
    name: "WasmRefFunc",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmInternalFunctionCreateExternal,
    name: "WasmInternalFunctionCreateExternal",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmFunctionTableGet,
    name: "WasmFunctionTableGet",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmFunctionTableSet,
    name: "WasmFunctionTableSet",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTableInit,
    name: "WasmTableInit",
    nargs: 6,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTableCopy,
    name: "WasmTableCopy",
    nargs: 6,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTableGrow,
    name: "WasmTableGrow",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTableFill,
    name: "WasmTableFill",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmJSToWasmObject,
    name: "WasmJSToWasmObject",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmWasmToJSObject,
    name: "WasmWasmToJSObject",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmCompileLazy,
    name: "WasmCompileLazy",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmAllocateFeedbackVector,
    name: "WasmAllocateFeedbackVector",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmLiftoffDeoptFinish,
    name: "WasmLiftoffDeoptFinish",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TierUpJSToWasmWrapper,
    name: "TierUpJSToWasmWrapper",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsWasmExternalFunction,
    name: "IsWasmExternalFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TierUpWasmToJSWrapper,
    name: "TierUpWasmToJSWrapper",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTriggerTierUp,
    name: "WasmTriggerTierUp",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmDebugBreak,
    name: "WasmDebugBreak",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmAllocateDescriptorStruct,
    name: "WasmAllocateDescriptorStruct",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmArrayCopy,
    name: "WasmArrayCopy",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmArrayNewSegment,
    name: "WasmArrayNewSegment",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmArrayInitSegment,
    name: "WasmArrayInitSegment",
    nargs: 6,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmAllocateSuspender,
    name: "WasmAllocateSuspender",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmSuspended,
    name: "WasmSuspended",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmAllocateContinuation,
    name: "WasmAllocateContinuation",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ClearWasmSuspenderResumeField,
    name: "ClearWasmSuspenderResumeField",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmCastToSpecialPrimitiveArray,
    name: "WasmCastToSpecialPrimitiveArray",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringAddNoMapCheckShared,
    name: "WasmStringAdd_NoMapCheck_Shared",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringNewSegmentWtf8,
    name: "WasmStringNewSegmentWtf8",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringNewWtf8,
    name: "WasmStringNewWtf8",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringNewWtf8Array,
    name: "WasmStringNewWtf8Array",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringNewWtf16,
    name: "WasmStringNewWtf16",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringNewWtf16Array,
    name: "WasmStringNewWtf16Array",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringConst,
    name: "WasmStringConst",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringMeasureUtf8,
    name: "WasmStringMeasureUtf8",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringMeasureWtf8,
    name: "WasmStringMeasureWtf8",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringEncodeWtf8,
    name: "WasmStringEncodeWtf8",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringEncodeWtf16,
    name: "WasmStringEncodeWtf16",
    nargs: 6,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringEncodeWtf8Array,
    name: "WasmStringEncodeWtf8Array",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringToUtf8Array,
    name: "WasmStringToUtf8Array",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringAsWtf8,
    name: "WasmStringAsWtf8",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringViewWtf8Encode,
    name: "WasmStringViewWtf8Encode",
    nargs: 7,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringViewWtf8Slice,
    name: "WasmStringViewWtf8Slice",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringFromCodePoint,
    name: "WasmStringFromCodePoint",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStringHash,
    name: "WasmStringHash",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmSubstring,
    name: "WasmSubstring",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmSubstringShared,
    name: "WasmSubstringShared",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmConfigureAllPrototypes,
    name: "WasmConfigureAllPrototypes",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmConfigureAllPrototypesOpt,
    name: "WasmConfigureAllPrototypesOpt",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugCollectWasmCoverage,
    name: "DebugCollectWasmCoverage",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::TsanAcquireForInitializationFence,
    name: "TsanAcquireForInitializationFence",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTypeAssertionFailed,
    name: "WasmTypeAssertionFailed",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::BuildRefTypeBitfield,
    name: "BuildRefTypeBitfield",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CheckIsOnCentralStack,
    name: "CheckIsOnCentralStack",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CountUnoptimizedWasmToJSWrapper,
    name: "CountUnoptimizedWasmToJSWrapper",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DisallowWasmCodegen,
    name: "DisallowWasmCodegen",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::EstimateCurrentMemoryConsumption,
    name: "EstimateCurrentMemoryConsumption",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FlushLiftoffCode,
    name: "FlushLiftoffCode",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::FreezeWasmLazyCompilation,
    name: "FreezeWasmLazyCompilation",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GenerateWasmCompilationHints,
    name: "GenerateWasmCompilationHints",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetWasmExceptionTagId,
    name: "GetWasmExceptionTagId",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetWasmExceptionValues,
    name: "GetWasmExceptionValues",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetWasmRecoveredTrapCount,
    name: "GetWasmRecoveredTrapCount",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasUnoptimizedWasmToJSWrapper,
    name: "HasUnoptimizedWasmToJSWrapper",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsLiftoffFunction,
    name: "IsLiftoffFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsTurboFanFunction,
    name: "IsTurboFanFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsUncompiledWasmFunction,
    name: "IsUncompiledWasmFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsWasmCode,
    name: "IsWasmCode",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsWasmDebugFunction,
    name: "IsWasmDebugFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsWasmPartialOOBWriteNoop,
    name: "IsWasmPartialOOBWriteNoop",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::IsWasmTrapHandlerEnabled,
    name: "IsWasmTrapHandlerEnabled",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetWasmCompileControls,
    name: "SetWasmCompileControls",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::SetWasmInstantiateControls,
    name: "SetWasmInstantiateControls",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmArray,
    name: "WasmArray",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmCompiledExportWrappersCount,
    name: "WasmCompiledExportWrappersCount",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmDeoptsExecutedCount,
    name: "WasmDeoptsExecutedCount",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmDeoptsExecutedForFunction,
    name: "WasmDeoptsExecutedForFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmEnterDebugging,
    name: "WasmEnterDebugging",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmGenerateRandomModule,
    name: "WasmGenerateRandomModule",
    nargs: -1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmGetNumberOfInstances,
    name: "WasmGetNumberOfInstances",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmLeaveDebugging,
    name: "WasmLeaveDebugging",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmNumCodeSpaces,
    name: "WasmNumCodeSpaces",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmStruct,
    name: "WasmStruct",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmSwitchToTheCentralStackCount,
    name: "WasmSwitchToTheCentralStackCount",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTierUpFunction,
    name: "WasmTierUpFunction",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTraceEnter,
    name: "WasmTraceEnter",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTraceExit,
    name: "WasmTraceExit",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTraceMemory,
    name: "WasmTraceMemory",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTriggerCodeGC,
    name: "WasmTriggerCodeGC",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTraceGlobal,
    name: "WasmTraceGlobal",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTriggerTierUpForTesting,
    name: "WasmTriggerTierUpForTesting",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTraceBeginExecution,
    name: "WasmTraceBeginExecution",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::WasmTraceEndExecution,
    name: "WasmTraceEndExecution",
    nargs: 0,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::JSFinalizationRegistryRegisterWeakCellWithUnregisterToken,
    name: "JSFinalizationRegistryRegisterWeakCellWithUnregisterToken",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::JSWeakRefAddToKeptObjects,
    name: "JSWeakRefAddToKeptObjects",
    nargs: 1,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DebugBreakOnBytecode,
    name: "DebugBreakOnBytecode",
    nargs: 1,
    result_size: 2,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadLookupSlotForCall,
    name: "LoadLookupSlotForCall",
    nargs: 1,
    result_size: 2,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ElementsTransitionAndStoreICMiss,
    name: "ElementsTransitionAndStoreIC_Miss",
    nargs: 6,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::KeyedLoadICMiss,
    name: "KeyedLoadIC_Miss",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::KeyedStoreICMiss,
    name: "KeyedStoreIC_Miss",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DefineKeyedOwnICMiss,
    name: "DefineKeyedOwnIC_Miss",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreInArrayLiteralICMiss,
    name: "StoreInArrayLiteralIC_Miss",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DefineNamedOwnICSlow,
    name: "DefineNamedOwnIC_Slow",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::KeyedStoreICSlow,
    name: "KeyedStoreIC_Slow",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DefineKeyedOwnICSlow,
    name: "DefineKeyedOwnIC_Slow",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadElementWithInterceptor,
    name: "LoadElementWithInterceptor",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadGlobalICMiss,
    name: "LoadGlobalIC_Miss",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadGlobalICSlow,
    name: "LoadGlobalIC_Slow",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadICMiss,
    name: "LoadIC_Miss",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadICMissFromBaseline,
    name: "LoadIC_Miss_FromBaseline",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::PatchLoadICUninitializedBaseline,
    name: "PatchLoadICUninitializedBaseline",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::GetStringLengthAndUpdateFeedback,
    name: "GetStringLengthAndUpdateFeedback",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadNoFeedbackICMiss,
    name: "LoadNoFeedbackIC_Miss",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadWithReceiverICMiss,
    name: "LoadWithReceiverIC_Miss",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadWithReceiverNoFeedbackICMiss,
    name: "LoadWithReceiverNoFeedbackIC_Miss",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::LoadPropertyPastInterceptor,
    name: "LoadPropertyPastInterceptor",
    nargs: 6,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreCallbackProperty,
    name: "StoreCallbackProperty",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreGlobalICMiss,
    name: "StoreGlobalIC_Miss",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreGlobalICNoFeedbackMiss,
    name: "StoreGlobalICNoFeedback_Miss",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreGlobalICSlow,
    name: "StoreGlobalIC_Slow",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreICMiss,
    name: "StoreIC_Miss",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::DefineNamedOwnICMiss,
    name: "DefineNamedOwnIC_Miss",
    nargs: 5,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StoreInArrayLiteralICSlow,
    name: "StoreInArrayLiteralIC_Slow",
    nargs: 3,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::StorePropertyPastInterceptor,
    name: "StorePropertyPastInterceptor",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CloneObjectICSlow,
    name: "CloneObjectIC_Slow",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::CloneObjectICMiss,
    name: "CloneObjectIC_Miss",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::KeyedHasICMiss,
    name: "KeyedHasIC_Miss",
    nargs: 4,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::HasElementWithInterceptor,
    name: "HasElementWithInterceptor",
    nargs: 2,
    result_size: 1,
  },
  RuntimeFunction {
    id: RuntimeFunctionId::ObjectAssignTryFastcase,
    name: "ObjectAssignTryFastcase",
    nargs: 2,
    result_size: 1,
  },
];

/// 根据名称检索运行时函数描述符
pub fn find_runtime_function(name: &str) -> Option<&'static RuntimeFunction> {
  RUNTIME_FUNCTIONS.iter().find(|f| f.name == name)
}

/// Runtime 函数调用属性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum RuntimeCallProperty {
  #[strum(serialize = "kCannotTriggerGC")]
  CannotTriggerGC,
}

/// 优化状态枚举 (来源: runtime.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum OptimizationStatus {
  IsFunction,
  NeverOptimize,
  AlwaysOptimize,
  MaybeDeopted,
  Optimized,
  Turbofanned,
  Maglevved,
  Interpreted,
}

/// 调试打印值类型 (来源: runtime.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[repr(u16)]
pub enum DebugPrintValueType {
  Normal = 0,
  Smi = 1,
  HeapObject = 2,
}

/// 双返回值调用约定结构体 (来源: runtime-utils.h ObjectPair)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ObjectPair {
  pub x: u64,
  pub y: u64,
}

/// Runtime 隔离窄环境 trait
pub trait RuntimeEnv {
  fn isolate_address(&self) -> u64;
  fn has_pending_exception(&self) -> bool;
}
