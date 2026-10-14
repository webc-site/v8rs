// 由 tool/src/builtins/builtins-misc.js 生成; 来源 v8/src/builtins
// 内建名表 + 核心调度元数据 (274 内建)

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// 内建名表 (共 274 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  Abort,
  AbortCSADcheck,
  AbstractModuleSourceToStringTag,
  AdaptorWithBuiltinExitFrame0,
  AdaptorWithBuiltinExitFrame1,
  AdaptorWithBuiltinExitFrame2,
  AdaptorWithBuiltinExitFrame3,
  AdaptorWithBuiltinExitFrame4,
  AdaptorWithBuiltinExitFrame5,
  AllocateInOldGeneration,
  AllocateInYoungGeneration,
  AtomicsConditionConstructor,
  AtomicsConditionIsCondition,
  AtomicsConditionNotify,
  AtomicsConditionWait,
  AtomicsMutexConstructor,
  AtomicsMutexIsMutex,
  AtomicsMutexLock,
  AtomicsMutexLockWithTimeout,
  AtomicsMutexTryLock,
  BaselineLeaveFrame,
  #[strum(serialize = "CEntry_Return1_ArgvInRegister_NoBuiltinExit")]
  CEntryReturn1ArgvInRegisterNoBuiltinExit,
  #[strum(serialize = "CEntry_Return1_ArgvOnStack_BuiltinExit")]
  CEntryReturn1ArgvOnStackBuiltinExit,
  #[strum(serialize = "CEntry_Return1_ArgvOnStack_NoBuiltinExit")]
  CEntryReturn1ArgvOnStackNoBuiltinExit,
  #[strum(serialize = "CEntry_Return2_ArgvInRegister_NoBuiltinExit")]
  CEntryReturn2ArgvInRegisterNoBuiltinExit,
  #[strum(serialize = "CEntry_Return2_ArgvOnStack_NoBuiltinExit")]
  CEntryReturn2ArgvOnStackNoBuiltinExit,
  CallApiCallbackGeneric,
  CallApiCallbackOptimized,
  CallApiCallbackOptimizedNoProfiling,
  CallApiGetter,
  CallAsyncModuleFulfilled,
  CallAsyncModuleRejected,
  CallBoundFunction,
  CallForwardVarargs,
  CallFunctionForwardVarargs,
  #[strum(serialize = "CallFunctionTemplate_CheckAccess")]
  CallFunctionTemplateCheckAccess,
  #[strum(serialize = "CallFunctionTemplate_CheckAccessAndCompatibleReceiver")]
  CallFunctionTemplateCheckAccessAndCompatibleReceiver,
  #[strum(serialize = "CallFunctionTemplate_CheckCompatibleReceiver")]
  CallFunctionTemplateCheckCompatibleReceiver,
  #[strum(serialize = "CallFunctionTemplate_Generic")]
  CallFunctionTemplateGeneric,
  #[strum(serialize = "CallFunction_ReceiverIsAny")]
  CallFunctionReceiverIsAny,
  #[strum(serialize = "CallFunction_ReceiverIsNotNullOrUndefined")]
  CallFunctionReceiverIsNotNullOrUndefined,
  #[strum(serialize = "CallFunction_ReceiverIsNullOrUndefined")]
  CallFunctionReceiverIsNullOrUndefined,
  CallNamedInterceptorGetter,
  CallNamedInterceptorSetter,
  CallVarargs,
  CallWithArrayLike,
  #[strum(serialize = "CallWithArrayLike_WithFeedback")]
  CallWithArrayLikeWithFeedback,
  CallWithSpread,
  #[strum(serialize = "CallWithSpread_Baseline")]
  CallWithSpreadBaseline,
  #[strum(serialize = "CallWithSpread_WithFeedback")]
  CallWithSpreadWithFeedback,
  CallWrappedFunction,
  #[strum(serialize = "Call_ReceiverIsAny")]
  CallReceiverIsAny,
  #[strum(serialize = "Call_ReceiverIsAny_Baseline")]
  CallReceiverIsAnyBaseline,
  #[strum(serialize = "Call_ReceiverIsAny_Baseline_Compact")]
  CallReceiverIsAnyBaselineCompact,
  #[strum(serialize = "Call_ReceiverIsAny_WithFeedback")]
  CallReceiverIsAnyWithFeedback,
  #[strum(serialize = "Call_ReceiverIsNotNullOrUndefined")]
  CallReceiverIsNotNullOrUndefined,
  #[strum(serialize = "Call_ReceiverIsNotNullOrUndefined_Baseline")]
  CallReceiverIsNotNullOrUndefinedBaseline,
  #[strum(serialize = "Call_ReceiverIsNotNullOrUndefined_Baseline_Compact")]
  CallReceiverIsNotNullOrUndefinedBaselineCompact,
  #[strum(serialize = "Call_ReceiverIsNotNullOrUndefined_WithFeedback")]
  CallReceiverIsNotNullOrUndefinedWithFeedback,
  #[strum(serialize = "Call_ReceiverIsNullOrUndefined")]
  CallReceiverIsNullOrUndefined,
  #[strum(serialize = "Call_ReceiverIsNullOrUndefined_Baseline")]
  CallReceiverIsNullOrUndefinedBaseline,
  #[strum(serialize = "Call_ReceiverIsNullOrUndefined_Baseline_Compact")]
  CallReceiverIsNullOrUndefinedBaselineCompact,
  #[strum(serialize = "Call_ReceiverIsNullOrUndefined_WithFeedback")]
  CallReceiverIsNullOrUndefinedWithFeedback,
  CheckMaglevType,
  CloneObjectIC,
  CloneObjectICBaseline,
  #[strum(serialize = "CloneObjectIC_Slow")]
  CloneObjectICSlow,
  CompileLazy,
  CopyDataProperties,
  CopyDataPropertiesWithExcludedProperties,
  CopyDataPropertiesWithExcludedPropertiesOnStack,
  CopyFastSmiOrObjectElements,
  CreateDataProperty,
  DebugBreakTrampoline,
  DefineKeyedOwnIC,
  DefineKeyedOwnICBaseline,
  DefineKeyedOwnICTrampoline,
  #[strum(serialize = "DefineKeyedOwnIC_Megamorphic")]
  DefineKeyedOwnICMegamorphic,
  DefineNamedOwnIC,
  DefineNamedOwnICBaseline,
  DefineNamedOwnICTrampoline,
  #[strum(serialize = "DefineNamedOwnIC_NoFeedback")]
  DefineNamedOwnICNoFeedback,
  DeleteProperty,
  #[strum(serialize = "ElementsTransitionAndStore_InBounds")]
  ElementsTransitionAndStoreInBounds,
  #[strum(serialize = "ElementsTransitionAndStore_NoTransitionGrowAndHandleCOW")]
  ElementsTransitionAndStoreNoTransitionGrowAndHandleCOW,
  #[strum(serialize = "ElementsTransitionAndStore_NoTransitionHandleCOW")]
  ElementsTransitionAndStoreNoTransitionHandleCOW,
  #[strum(serialize = "ElementsTransitionAndStore_NoTransitionIgnoreTypedArrayOOB")]
  ElementsTransitionAndStoreNoTransitionIgnoreTypedArrayOOB,
  EmptyFunction,
  EmptyFunction1,
  EnumeratedKeyedLoadIC,
  EnumeratedKeyedLoadICBaseline,
  EphemeronKeyBarrierIgnoreFP,
  EphemeronKeyBarrierSaveFP,
  FinalizationRegistryUnregister,
  FindNonDefaultConstructorOrConstruct,
  ForInEnumerate,
  ForInFilter,
  ForInPrepare,
  FunctionLogNextExecution,
  GetOwnPropertyDescriptor,
  GetProperty,
  GetPropertyWithReceiver,
  GlobalDecodeURI,
  GlobalDecodeURIComponent,
  GlobalEncodeURI,
  GlobalEncodeURIComponent,
  GlobalEscape,
  GlobalEval,
  GlobalIsFinite,
  GlobalIsNaN,
  GlobalUnescape,
  GrowFastDoubleElements,
  GrowFastSmiOrObjectElements,
  HandleApiCallAsConstructorDelegate,
  HandleApiCallAsFunctionDelegate,
  HandleApiCallOrConstruct,
  HandleApiConstruct,
  HasIndexedInterceptorIC,
  Illegal,
  IllegalInvocationThrower,
  IndirectPointerBarrierIgnoreFP,
  IndirectPointerBarrierSaveFP,
  InterpreterEntryTrampoline,
  InterpreterEntryTrampolineForProfiling,
  InterpreterPushArgsThenCall,
  InterpreterPushArgsThenCallWithFinalSpread,
  InterpreterPushArgsThenConstruct,
  InterpreterPushArgsThenConstructArrayFunction,
  InterpreterPushArgsThenConstructWithFinalSpread,
  InterpreterPushUndefinedAndArgsThenCall,
  IsTraceCategoryEnabled,
  KeyedHasIC,
  KeyedHasICBaseline,
  #[strum(serialize = "KeyedHasIC_Megamorphic")]
  KeyedHasICMegamorphic,
  #[strum(serialize = "KeyedHasIC_PolymorphicName")]
  KeyedHasICPolymorphicName,
  #[strum(serialize = "KeyedHasIC_SloppyArguments")]
  KeyedHasICSloppyArguments,
  KeyedLoadIC,
  KeyedLoadICBaseline,
  KeyedLoadICTrampoline,
  #[strum(serialize = "KeyedLoadICTrampoline_Megamorphic")]
  KeyedLoadICTrampolineMegamorphic,
  #[strum(serialize = "KeyedLoadIC_Megamorphic")]
  KeyedLoadICMegamorphic,
  #[strum(serialize = "KeyedLoadIC_PolymorphicName")]
  KeyedLoadICPolymorphicName,
  #[strum(serialize = "KeyedLoadIC_SloppyArguments")]
  KeyedLoadICSloppyArguments,
  KeyedStoreIC,
  KeyedStoreICBaseline,
  KeyedStoreICTrampoline,
  #[strum(serialize = "KeyedStoreICTrampoline_Megamorphic")]
  KeyedStoreICTrampolineMegamorphic,
  #[strum(serialize = "KeyedStoreIC_Megamorphic")]
  KeyedStoreICMegamorphic,
  #[strum(serialize = "KeyedStoreIC_SloppyArguments_InBounds")]
  KeyedStoreICSloppyArgumentsInBounds,
  #[strum(serialize = "KeyedStoreIC_SloppyArguments_NoTransitionGrowAndHandleCOW")]
  KeyedStoreICSloppyArgumentsNoTransitionGrowAndHandleCOW,
  #[strum(serialize = "KeyedStoreIC_SloppyArguments_NoTransitionHandleCOW")]
  KeyedStoreICSloppyArgumentsNoTransitionHandleCOW,
  #[strum(serialize = "KeyedStoreIC_SloppyArguments_NoTransitionIgnoreTypedArrayOOB")]
  KeyedStoreICSloppyArgumentsNoTransitionIgnoreTypedArrayOOB,
  LoadGlobalIC,
  LoadGlobalICBaseline,
  LoadGlobalICInsideTypeof,
  LoadGlobalICInsideTypeofBaseline,
  LoadGlobalICInsideTypeofTrampoline,
  LoadGlobalICTrampoline,
  #[strum(serialize = "LoadGlobalIC_NoFeedback")]
  LoadGlobalICNoFeedback,
  LoadIC,
  LoadICConstantFromPrototypeBaseline,
  LoadICDoubleFieldBaseline,
  LoadICGenericBaseline,
  LoadICInObjectNonDoubleFieldBaseline,
  LoadICOutOfObjectNonDoubleFieldBaseline,
  LoadICStringLengthBaseline,
  LoadICTrampoline,
  #[strum(serialize = "LoadICTrampoline_Megamorphic")]
  LoadICTrampolineMegamorphic,
  LoadICUninitializedBaseline,
  #[strum(serialize = "LoadIC_FunctionPrototype")]
  LoadICFunctionPrototype,
  #[strum(serialize = "LoadIC_Megamorphic")]
  LoadICMegamorphic,
  #[strum(serialize = "LoadIC_NoFeedback")]
  LoadICNoFeedback,
  #[strum(serialize = "LoadIC_Noninlined")]
  LoadICNoninlined,
  #[strum(serialize = "LoadIC_StringLength")]
  LoadICStringLength,
  #[strum(serialize = "LoadIC_StringWrapperLength")]
  LoadICStringWrapperLength,
  LoadIndexedInterceptorIC,
  LoadSuperIC,
  LoadSuperICBaseline,
  LookupContextInsideTypeofBaseline,
  LookupContextInsideTypeofTrampoline,
  LookupContextNoCellBaseline,
  LookupContextNoCellInsideTypeofBaseline,
  LookupContextNoCellInsideTypeofTrampoline,
  LookupContextNoCellTrampoline,
  LookupContextTrampoline,
  LookupGlobalIC,
  LookupGlobalICBaseline,
  LookupGlobalICInsideTypeof,
  LookupGlobalICInsideTypeofBaseline,
  LookupGlobalICInsideTypeofTrampoline,
  LookupGlobalICTrampoline,
  LookupScriptContextBaseline,
  MaglevFunctionEntryStackCheck,
  #[strum(serialize = "MaglevFunctionEntryStackCheck_WithNewTarget")]
  MaglevFunctionEntryStackCheckWithNewTarget,
  #[strum(serialize = "MaglevFunctionEntryStackCheck_WithoutNewTarget")]
  MaglevFunctionEntryStackCheckWithoutNewTarget,
  MaglevOptimizeCodeOrTailCallOptimizedCodeSlot,
  MarkFlushed,
  MarkLazyDeoptimized,
  MarkReoptimizeLazyDeoptimized,
  NewHeapNumber,
  OptimizeMaglevEager,
  OptimizeTurbofanEager,
  RecordWriteIgnoreFP,
  RecordWriteSaveFP,
  ReturnReceiver,
  SameValue,
  SameValueNumbersOnly,
  SetDataProperties,
  SetProperty,
  ShadowRealmConstructor,
  ShadowRealmGetWrappedValue,
  ShadowRealmImportValueFulfilled,
  ShadowRealmImportValueRejected,
  ShadowRealmPrototypeEvaluate,
  ShadowRealmPrototypeImportValue,
  SharedSpaceJSObjectHasInstance,
  SharedStructConstructor,
  SharedStructTypeConstructor,
  SharedStructTypeIsSharedStruct,
  StartMaglevOptimizeJob,
  StartTurbofanOptimizeJob,
  #[strum(serialize = "StoreFastElementIC_InBounds")]
  StoreFastElementICInBounds,
  #[strum(serialize = "StoreFastElementIC_InBounds_Generic")]
  StoreFastElementICInBoundsGeneric,
  #[strum(serialize = "StoreFastElementIC_InBounds_ISX")]
  StoreFastElementICInBoundsISX,
  #[strum(serialize = "StoreFastElementIC_NoTransitionGrowAndHandleCOW")]
  StoreFastElementICNoTransitionGrowAndHandleCOW,
  #[strum(serialize = "StoreFastElementIC_NoTransitionHandleCOW")]
  StoreFastElementICNoTransitionHandleCOW,
  #[strum(serialize = "StoreFastElementIC_NoTransitionIgnoreTypedArrayOOB")]
  StoreFastElementICNoTransitionIgnoreTypedArrayOOB,
  StoreGlobalIC,
  StoreGlobalICBaseline,
  StoreGlobalICTrampoline,
  #[strum(serialize = "StoreGlobalIC_Slow")]
  StoreGlobalICSlow,
  StoreIC,
  StoreICBaseline,
  StoreICTrampoline,
  #[strum(serialize = "StoreICTrampoline_Megamorphic")]
  StoreICTrampolineMegamorphic,
  #[strum(serialize = "StoreIC_Megamorphic")]
  StoreICMegamorphic,
  #[strum(serialize = "StoreIC_NoFeedback")]
  StoreICNoFeedback,
  StoreInArrayLiteralIC,
  StoreInArrayLiteralICBaseline,
  StrictPoisonPillThrower,
  TSANRelaxedLoad32IgnoreFP,
  TSANRelaxedLoad32SaveFP,
  TSANRelaxedLoad64IgnoreFP,
  TSANRelaxedLoad64SaveFP,
  TSANRelaxedStore16IgnoreFP,
  TSANRelaxedStore16SaveFP,
  TSANRelaxedStore32IgnoreFP,
  TSANRelaxedStore32SaveFP,
  TSANRelaxedStore64IgnoreFP,
  TSANRelaxedStore64SaveFP,
  TSANRelaxedStore8IgnoreFP,
  TSANRelaxedStore8SaveFP,
  TSANReleaseStore16IgnoreFP,
  TSANReleaseStore16SaveFP,
  TSANReleaseStore32IgnoreFP,
  TSANReleaseStore32SaveFP,
  TSANReleaseStore64IgnoreFP,
  TSANReleaseStore64SaveFP,
  TSANReleaseStore8IgnoreFP,
  TSANReleaseStore8SaveFP,
  TSANSeqCstStore16IgnoreFP,
  TSANSeqCstStore16SaveFP,
  TSANSeqCstStore32IgnoreFP,
  TSANSeqCstStore32SaveFP,
  TSANSeqCstStore64IgnoreFP,
  TSANSeqCstStore64SaveFP,
  TSANSeqCstStore8IgnoreFP,
  TSANSeqCstStore8SaveFP,
  Trace,
  UnsupportedThrower,
  WasmAllocateInOldGeneration,
  WasmAllocateInSharedHeap,
  WasmAllocateInYoungGeneration,
  WasmCEntry,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Abort => "Abort",
      Self::AbortCSADcheck => "AbortCSADcheck",
      Self::AbstractModuleSourceToStringTag => "AbstractModuleSourceToStringTag",
      Self::AdaptorWithBuiltinExitFrame0 => "AdaptorWithBuiltinExitFrame0",
      Self::AdaptorWithBuiltinExitFrame1 => "AdaptorWithBuiltinExitFrame1",
      Self::AdaptorWithBuiltinExitFrame2 => "AdaptorWithBuiltinExitFrame2",
      Self::AdaptorWithBuiltinExitFrame3 => "AdaptorWithBuiltinExitFrame3",
      Self::AdaptorWithBuiltinExitFrame4 => "AdaptorWithBuiltinExitFrame4",
      Self::AdaptorWithBuiltinExitFrame5 => "AdaptorWithBuiltinExitFrame5",
      Self::AllocateInOldGeneration => "AllocateInOldGeneration",
      Self::AllocateInYoungGeneration => "AllocateInYoungGeneration",
      Self::AtomicsConditionConstructor => "AtomicsConditionConstructor",
      Self::AtomicsConditionIsCondition => "AtomicsConditionIsCondition",
      Self::AtomicsConditionNotify => "AtomicsConditionNotify",
      Self::AtomicsConditionWait => "AtomicsConditionWait",
      Self::AtomicsMutexConstructor => "AtomicsMutexConstructor",
      Self::AtomicsMutexIsMutex => "AtomicsMutexIsMutex",
      Self::AtomicsMutexLock => "AtomicsMutexLock",
      Self::AtomicsMutexLockWithTimeout => "AtomicsMutexLockWithTimeout",
      Self::AtomicsMutexTryLock => "AtomicsMutexTryLock",
      Self::BaselineLeaveFrame => "BaselineLeaveFrame",
      Self::CEntryReturn1ArgvInRegisterNoBuiltinExit => {
        "CEntry_Return1_ArgvInRegister_NoBuiltinExit"
      }
      Self::CEntryReturn1ArgvOnStackBuiltinExit => "CEntry_Return1_ArgvOnStack_BuiltinExit",
      Self::CEntryReturn1ArgvOnStackNoBuiltinExit => "CEntry_Return1_ArgvOnStack_NoBuiltinExit",
      Self::CEntryReturn2ArgvInRegisterNoBuiltinExit => {
        "CEntry_Return2_ArgvInRegister_NoBuiltinExit"
      }
      Self::CEntryReturn2ArgvOnStackNoBuiltinExit => "CEntry_Return2_ArgvOnStack_NoBuiltinExit",
      Self::CallApiCallbackGeneric => "CallApiCallbackGeneric",
      Self::CallApiCallbackOptimized => "CallApiCallbackOptimized",
      Self::CallApiCallbackOptimizedNoProfiling => "CallApiCallbackOptimizedNoProfiling",
      Self::CallApiGetter => "CallApiGetter",
      Self::CallAsyncModuleFulfilled => "CallAsyncModuleFulfilled",
      Self::CallAsyncModuleRejected => "CallAsyncModuleRejected",
      Self::CallBoundFunction => "CallBoundFunction",
      Self::CallForwardVarargs => "CallForwardVarargs",
      Self::CallFunctionForwardVarargs => "CallFunctionForwardVarargs",
      Self::CallFunctionTemplateCheckAccess => "CallFunctionTemplate_CheckAccess",
      Self::CallFunctionTemplateCheckAccessAndCompatibleReceiver => {
        "CallFunctionTemplate_CheckAccessAndCompatibleReceiver"
      }
      Self::CallFunctionTemplateCheckCompatibleReceiver => {
        "CallFunctionTemplate_CheckCompatibleReceiver"
      }
      Self::CallFunctionTemplateGeneric => "CallFunctionTemplate_Generic",
      Self::CallFunctionReceiverIsAny => "CallFunction_ReceiverIsAny",
      Self::CallFunctionReceiverIsNotNullOrUndefined => "CallFunction_ReceiverIsNotNullOrUndefined",
      Self::CallFunctionReceiverIsNullOrUndefined => "CallFunction_ReceiverIsNullOrUndefined",
      Self::CallNamedInterceptorGetter => "CallNamedInterceptorGetter",
      Self::CallNamedInterceptorSetter => "CallNamedInterceptorSetter",
      Self::CallVarargs => "CallVarargs",
      Self::CallWithArrayLike => "CallWithArrayLike",
      Self::CallWithArrayLikeWithFeedback => "CallWithArrayLike_WithFeedback",
      Self::CallWithSpread => "CallWithSpread",
      Self::CallWithSpreadBaseline => "CallWithSpread_Baseline",
      Self::CallWithSpreadWithFeedback => "CallWithSpread_WithFeedback",
      Self::CallWrappedFunction => "CallWrappedFunction",
      Self::CallReceiverIsAny => "Call_ReceiverIsAny",
      Self::CallReceiverIsAnyBaseline => "Call_ReceiverIsAny_Baseline",
      Self::CallReceiverIsAnyBaselineCompact => "Call_ReceiverIsAny_Baseline_Compact",
      Self::CallReceiverIsAnyWithFeedback => "Call_ReceiverIsAny_WithFeedback",
      Self::CallReceiverIsNotNullOrUndefined => "Call_ReceiverIsNotNullOrUndefined",
      Self::CallReceiverIsNotNullOrUndefinedBaseline => {
        "Call_ReceiverIsNotNullOrUndefined_Baseline"
      }
      Self::CallReceiverIsNotNullOrUndefinedBaselineCompact => {
        "Call_ReceiverIsNotNullOrUndefined_Baseline_Compact"
      }
      Self::CallReceiverIsNotNullOrUndefinedWithFeedback => {
        "Call_ReceiverIsNotNullOrUndefined_WithFeedback"
      }
      Self::CallReceiverIsNullOrUndefined => "Call_ReceiverIsNullOrUndefined",
      Self::CallReceiverIsNullOrUndefinedBaseline => "Call_ReceiverIsNullOrUndefined_Baseline",
      Self::CallReceiverIsNullOrUndefinedBaselineCompact => {
        "Call_ReceiverIsNullOrUndefined_Baseline_Compact"
      }
      Self::CallReceiverIsNullOrUndefinedWithFeedback => {
        "Call_ReceiverIsNullOrUndefined_WithFeedback"
      }
      Self::CheckMaglevType => "CheckMaglevType",
      Self::CloneObjectIC => "CloneObjectIC",
      Self::CloneObjectICBaseline => "CloneObjectICBaseline",
      Self::CloneObjectICSlow => "CloneObjectIC_Slow",
      Self::CompileLazy => "CompileLazy",
      Self::CopyDataProperties => "CopyDataProperties",
      Self::CopyDataPropertiesWithExcludedProperties => "CopyDataPropertiesWithExcludedProperties",
      Self::CopyDataPropertiesWithExcludedPropertiesOnStack => {
        "CopyDataPropertiesWithExcludedPropertiesOnStack"
      }
      Self::CopyFastSmiOrObjectElements => "CopyFastSmiOrObjectElements",
      Self::CreateDataProperty => "CreateDataProperty",
      Self::DebugBreakTrampoline => "DebugBreakTrampoline",
      Self::DefineKeyedOwnIC => "DefineKeyedOwnIC",
      Self::DefineKeyedOwnICBaseline => "DefineKeyedOwnICBaseline",
      Self::DefineKeyedOwnICTrampoline => "DefineKeyedOwnICTrampoline",
      Self::DefineKeyedOwnICMegamorphic => "DefineKeyedOwnIC_Megamorphic",
      Self::DefineNamedOwnIC => "DefineNamedOwnIC",
      Self::DefineNamedOwnICBaseline => "DefineNamedOwnICBaseline",
      Self::DefineNamedOwnICTrampoline => "DefineNamedOwnICTrampoline",
      Self::DefineNamedOwnICNoFeedback => "DefineNamedOwnIC_NoFeedback",
      Self::DeleteProperty => "DeleteProperty",
      Self::ElementsTransitionAndStoreInBounds => "ElementsTransitionAndStore_InBounds",
      Self::ElementsTransitionAndStoreNoTransitionGrowAndHandleCOW => {
        "ElementsTransitionAndStore_NoTransitionGrowAndHandleCOW"
      }
      Self::ElementsTransitionAndStoreNoTransitionHandleCOW => {
        "ElementsTransitionAndStore_NoTransitionHandleCOW"
      }
      Self::ElementsTransitionAndStoreNoTransitionIgnoreTypedArrayOOB => {
        "ElementsTransitionAndStore_NoTransitionIgnoreTypedArrayOOB"
      }
      Self::EmptyFunction => "EmptyFunction",
      Self::EmptyFunction1 => "EmptyFunction1",
      Self::EnumeratedKeyedLoadIC => "EnumeratedKeyedLoadIC",
      Self::EnumeratedKeyedLoadICBaseline => "EnumeratedKeyedLoadICBaseline",
      Self::EphemeronKeyBarrierIgnoreFP => "EphemeronKeyBarrierIgnoreFP",
      Self::EphemeronKeyBarrierSaveFP => "EphemeronKeyBarrierSaveFP",
      Self::FinalizationRegistryUnregister => "FinalizationRegistryUnregister",
      Self::FindNonDefaultConstructorOrConstruct => "FindNonDefaultConstructorOrConstruct",
      Self::ForInEnumerate => "ForInEnumerate",
      Self::ForInFilter => "ForInFilter",
      Self::ForInPrepare => "ForInPrepare",
      Self::FunctionLogNextExecution => "FunctionLogNextExecution",
      Self::GetOwnPropertyDescriptor => "GetOwnPropertyDescriptor",
      Self::GetProperty => "GetProperty",
      Self::GetPropertyWithReceiver => "GetPropertyWithReceiver",
      Self::GlobalDecodeURI => "GlobalDecodeURI",
      Self::GlobalDecodeURIComponent => "GlobalDecodeURIComponent",
      Self::GlobalEncodeURI => "GlobalEncodeURI",
      Self::GlobalEncodeURIComponent => "GlobalEncodeURIComponent",
      Self::GlobalEscape => "GlobalEscape",
      Self::GlobalEval => "GlobalEval",
      Self::GlobalIsFinite => "GlobalIsFinite",
      Self::GlobalIsNaN => "GlobalIsNaN",
      Self::GlobalUnescape => "GlobalUnescape",
      Self::GrowFastDoubleElements => "GrowFastDoubleElements",
      Self::GrowFastSmiOrObjectElements => "GrowFastSmiOrObjectElements",
      Self::HandleApiCallAsConstructorDelegate => "HandleApiCallAsConstructorDelegate",
      Self::HandleApiCallAsFunctionDelegate => "HandleApiCallAsFunctionDelegate",
      Self::HandleApiCallOrConstruct => "HandleApiCallOrConstruct",
      Self::HandleApiConstruct => "HandleApiConstruct",
      Self::HasIndexedInterceptorIC => "HasIndexedInterceptorIC",
      Self::Illegal => "Illegal",
      Self::IllegalInvocationThrower => "IllegalInvocationThrower",
      Self::IndirectPointerBarrierIgnoreFP => "IndirectPointerBarrierIgnoreFP",
      Self::IndirectPointerBarrierSaveFP => "IndirectPointerBarrierSaveFP",
      Self::InterpreterEntryTrampoline => "InterpreterEntryTrampoline",
      Self::InterpreterEntryTrampolineForProfiling => "InterpreterEntryTrampolineForProfiling",
      Self::InterpreterPushArgsThenCall => "InterpreterPushArgsThenCall",
      Self::InterpreterPushArgsThenCallWithFinalSpread => {
        "InterpreterPushArgsThenCallWithFinalSpread"
      }
      Self::InterpreterPushArgsThenConstruct => "InterpreterPushArgsThenConstruct",
      Self::InterpreterPushArgsThenConstructArrayFunction => {
        "InterpreterPushArgsThenConstructArrayFunction"
      }
      Self::InterpreterPushArgsThenConstructWithFinalSpread => {
        "InterpreterPushArgsThenConstructWithFinalSpread"
      }
      Self::InterpreterPushUndefinedAndArgsThenCall => "InterpreterPushUndefinedAndArgsThenCall",
      Self::IsTraceCategoryEnabled => "IsTraceCategoryEnabled",
      Self::KeyedHasIC => "KeyedHasIC",
      Self::KeyedHasICBaseline => "KeyedHasICBaseline",
      Self::KeyedHasICMegamorphic => "KeyedHasIC_Megamorphic",
      Self::KeyedHasICPolymorphicName => "KeyedHasIC_PolymorphicName",
      Self::KeyedHasICSloppyArguments => "KeyedHasIC_SloppyArguments",
      Self::KeyedLoadIC => "KeyedLoadIC",
      Self::KeyedLoadICBaseline => "KeyedLoadICBaseline",
      Self::KeyedLoadICTrampoline => "KeyedLoadICTrampoline",
      Self::KeyedLoadICTrampolineMegamorphic => "KeyedLoadICTrampoline_Megamorphic",
      Self::KeyedLoadICMegamorphic => "KeyedLoadIC_Megamorphic",
      Self::KeyedLoadICPolymorphicName => "KeyedLoadIC_PolymorphicName",
      Self::KeyedLoadICSloppyArguments => "KeyedLoadIC_SloppyArguments",
      Self::KeyedStoreIC => "KeyedStoreIC",
      Self::KeyedStoreICBaseline => "KeyedStoreICBaseline",
      Self::KeyedStoreICTrampoline => "KeyedStoreICTrampoline",
      Self::KeyedStoreICTrampolineMegamorphic => "KeyedStoreICTrampoline_Megamorphic",
      Self::KeyedStoreICMegamorphic => "KeyedStoreIC_Megamorphic",
      Self::KeyedStoreICSloppyArgumentsInBounds => "KeyedStoreIC_SloppyArguments_InBounds",
      Self::KeyedStoreICSloppyArgumentsNoTransitionGrowAndHandleCOW => {
        "KeyedStoreIC_SloppyArguments_NoTransitionGrowAndHandleCOW"
      }
      Self::KeyedStoreICSloppyArgumentsNoTransitionHandleCOW => {
        "KeyedStoreIC_SloppyArguments_NoTransitionHandleCOW"
      }
      Self::KeyedStoreICSloppyArgumentsNoTransitionIgnoreTypedArrayOOB => {
        "KeyedStoreIC_SloppyArguments_NoTransitionIgnoreTypedArrayOOB"
      }
      Self::LoadGlobalIC => "LoadGlobalIC",
      Self::LoadGlobalICBaseline => "LoadGlobalICBaseline",
      Self::LoadGlobalICInsideTypeof => "LoadGlobalICInsideTypeof",
      Self::LoadGlobalICInsideTypeofBaseline => "LoadGlobalICInsideTypeofBaseline",
      Self::LoadGlobalICInsideTypeofTrampoline => "LoadGlobalICInsideTypeofTrampoline",
      Self::LoadGlobalICTrampoline => "LoadGlobalICTrampoline",
      Self::LoadGlobalICNoFeedback => "LoadGlobalIC_NoFeedback",
      Self::LoadIC => "LoadIC",
      Self::LoadICConstantFromPrototypeBaseline => "LoadICConstantFromPrototypeBaseline",
      Self::LoadICDoubleFieldBaseline => "LoadICDoubleFieldBaseline",
      Self::LoadICGenericBaseline => "LoadICGenericBaseline",
      Self::LoadICInObjectNonDoubleFieldBaseline => "LoadICInObjectNonDoubleFieldBaseline",
      Self::LoadICOutOfObjectNonDoubleFieldBaseline => "LoadICOutOfObjectNonDoubleFieldBaseline",
      Self::LoadICStringLengthBaseline => "LoadICStringLengthBaseline",
      Self::LoadICTrampoline => "LoadICTrampoline",
      Self::LoadICTrampolineMegamorphic => "LoadICTrampoline_Megamorphic",
      Self::LoadICUninitializedBaseline => "LoadICUninitializedBaseline",
      Self::LoadICFunctionPrototype => "LoadIC_FunctionPrototype",
      Self::LoadICMegamorphic => "LoadIC_Megamorphic",
      Self::LoadICNoFeedback => "LoadIC_NoFeedback",
      Self::LoadICNoninlined => "LoadIC_Noninlined",
      Self::LoadICStringLength => "LoadIC_StringLength",
      Self::LoadICStringWrapperLength => "LoadIC_StringWrapperLength",
      Self::LoadIndexedInterceptorIC => "LoadIndexedInterceptorIC",
      Self::LoadSuperIC => "LoadSuperIC",
      Self::LoadSuperICBaseline => "LoadSuperICBaseline",
      Self::LookupContextInsideTypeofBaseline => "LookupContextInsideTypeofBaseline",
      Self::LookupContextInsideTypeofTrampoline => "LookupContextInsideTypeofTrampoline",
      Self::LookupContextNoCellBaseline => "LookupContextNoCellBaseline",
      Self::LookupContextNoCellInsideTypeofBaseline => "LookupContextNoCellInsideTypeofBaseline",
      Self::LookupContextNoCellInsideTypeofTrampoline => {
        "LookupContextNoCellInsideTypeofTrampoline"
      }
      Self::LookupContextNoCellTrampoline => "LookupContextNoCellTrampoline",
      Self::LookupContextTrampoline => "LookupContextTrampoline",
      Self::LookupGlobalIC => "LookupGlobalIC",
      Self::LookupGlobalICBaseline => "LookupGlobalICBaseline",
      Self::LookupGlobalICInsideTypeof => "LookupGlobalICInsideTypeof",
      Self::LookupGlobalICInsideTypeofBaseline => "LookupGlobalICInsideTypeofBaseline",
      Self::LookupGlobalICInsideTypeofTrampoline => "LookupGlobalICInsideTypeofTrampoline",
      Self::LookupGlobalICTrampoline => "LookupGlobalICTrampoline",
      Self::LookupScriptContextBaseline => "LookupScriptContextBaseline",
      Self::MaglevFunctionEntryStackCheck => "MaglevFunctionEntryStackCheck",
      Self::MaglevFunctionEntryStackCheckWithNewTarget => {
        "MaglevFunctionEntryStackCheck_WithNewTarget"
      }
      Self::MaglevFunctionEntryStackCheckWithoutNewTarget => {
        "MaglevFunctionEntryStackCheck_WithoutNewTarget"
      }
      Self::MaglevOptimizeCodeOrTailCallOptimizedCodeSlot => {
        "MaglevOptimizeCodeOrTailCallOptimizedCodeSlot"
      }
      Self::MarkFlushed => "MarkFlushed",
      Self::MarkLazyDeoptimized => "MarkLazyDeoptimized",
      Self::MarkReoptimizeLazyDeoptimized => "MarkReoptimizeLazyDeoptimized",
      Self::NewHeapNumber => "NewHeapNumber",
      Self::OptimizeMaglevEager => "OptimizeMaglevEager",
      Self::OptimizeTurbofanEager => "OptimizeTurbofanEager",
      Self::RecordWriteIgnoreFP => "RecordWriteIgnoreFP",
      Self::RecordWriteSaveFP => "RecordWriteSaveFP",
      Self::ReturnReceiver => "ReturnReceiver",
      Self::SameValue => "SameValue",
      Self::SameValueNumbersOnly => "SameValueNumbersOnly",
      Self::SetDataProperties => "SetDataProperties",
      Self::SetProperty => "SetProperty",
      Self::ShadowRealmConstructor => "ShadowRealmConstructor",
      Self::ShadowRealmGetWrappedValue => "ShadowRealmGetWrappedValue",
      Self::ShadowRealmImportValueFulfilled => "ShadowRealmImportValueFulfilled",
      Self::ShadowRealmImportValueRejected => "ShadowRealmImportValueRejected",
      Self::ShadowRealmPrototypeEvaluate => "ShadowRealmPrototypeEvaluate",
      Self::ShadowRealmPrototypeImportValue => "ShadowRealmPrototypeImportValue",
      Self::SharedSpaceJSObjectHasInstance => "SharedSpaceJSObjectHasInstance",
      Self::SharedStructConstructor => "SharedStructConstructor",
      Self::SharedStructTypeConstructor => "SharedStructTypeConstructor",
      Self::SharedStructTypeIsSharedStruct => "SharedStructTypeIsSharedStruct",
      Self::StartMaglevOptimizeJob => "StartMaglevOptimizeJob",
      Self::StartTurbofanOptimizeJob => "StartTurbofanOptimizeJob",
      Self::StoreFastElementICInBounds => "StoreFastElementIC_InBounds",
      Self::StoreFastElementICInBoundsGeneric => "StoreFastElementIC_InBounds_Generic",
      Self::StoreFastElementICInBoundsISX => "StoreFastElementIC_InBounds_ISX",
      Self::StoreFastElementICNoTransitionGrowAndHandleCOW => {
        "StoreFastElementIC_NoTransitionGrowAndHandleCOW"
      }
      Self::StoreFastElementICNoTransitionHandleCOW => "StoreFastElementIC_NoTransitionHandleCOW",
      Self::StoreFastElementICNoTransitionIgnoreTypedArrayOOB => {
        "StoreFastElementIC_NoTransitionIgnoreTypedArrayOOB"
      }
      Self::StoreGlobalIC => "StoreGlobalIC",
      Self::StoreGlobalICBaseline => "StoreGlobalICBaseline",
      Self::StoreGlobalICTrampoline => "StoreGlobalICTrampoline",
      Self::StoreGlobalICSlow => "StoreGlobalIC_Slow",
      Self::StoreIC => "StoreIC",
      Self::StoreICBaseline => "StoreICBaseline",
      Self::StoreICTrampoline => "StoreICTrampoline",
      Self::StoreICTrampolineMegamorphic => "StoreICTrampoline_Megamorphic",
      Self::StoreICMegamorphic => "StoreIC_Megamorphic",
      Self::StoreICNoFeedback => "StoreIC_NoFeedback",
      Self::StoreInArrayLiteralIC => "StoreInArrayLiteralIC",
      Self::StoreInArrayLiteralICBaseline => "StoreInArrayLiteralICBaseline",
      Self::StrictPoisonPillThrower => "StrictPoisonPillThrower",
      Self::TSANRelaxedLoad32IgnoreFP => "TSANRelaxedLoad32IgnoreFP",
      Self::TSANRelaxedLoad32SaveFP => "TSANRelaxedLoad32SaveFP",
      Self::TSANRelaxedLoad64IgnoreFP => "TSANRelaxedLoad64IgnoreFP",
      Self::TSANRelaxedLoad64SaveFP => "TSANRelaxedLoad64SaveFP",
      Self::TSANRelaxedStore16IgnoreFP => "TSANRelaxedStore16IgnoreFP",
      Self::TSANRelaxedStore16SaveFP => "TSANRelaxedStore16SaveFP",
      Self::TSANRelaxedStore32IgnoreFP => "TSANRelaxedStore32IgnoreFP",
      Self::TSANRelaxedStore32SaveFP => "TSANRelaxedStore32SaveFP",
      Self::TSANRelaxedStore64IgnoreFP => "TSANRelaxedStore64IgnoreFP",
      Self::TSANRelaxedStore64SaveFP => "TSANRelaxedStore64SaveFP",
      Self::TSANRelaxedStore8IgnoreFP => "TSANRelaxedStore8IgnoreFP",
      Self::TSANRelaxedStore8SaveFP => "TSANRelaxedStore8SaveFP",
      Self::TSANReleaseStore16IgnoreFP => "TSANReleaseStore16IgnoreFP",
      Self::TSANReleaseStore16SaveFP => "TSANReleaseStore16SaveFP",
      Self::TSANReleaseStore32IgnoreFP => "TSANReleaseStore32IgnoreFP",
      Self::TSANReleaseStore32SaveFP => "TSANReleaseStore32SaveFP",
      Self::TSANReleaseStore64IgnoreFP => "TSANReleaseStore64IgnoreFP",
      Self::TSANReleaseStore64SaveFP => "TSANReleaseStore64SaveFP",
      Self::TSANReleaseStore8IgnoreFP => "TSANReleaseStore8IgnoreFP",
      Self::TSANReleaseStore8SaveFP => "TSANReleaseStore8SaveFP",
      Self::TSANSeqCstStore16IgnoreFP => "TSANSeqCstStore16IgnoreFP",
      Self::TSANSeqCstStore16SaveFP => "TSANSeqCstStore16SaveFP",
      Self::TSANSeqCstStore32IgnoreFP => "TSANSeqCstStore32IgnoreFP",
      Self::TSANSeqCstStore32SaveFP => "TSANSeqCstStore32SaveFP",
      Self::TSANSeqCstStore64IgnoreFP => "TSANSeqCstStore64IgnoreFP",
      Self::TSANSeqCstStore64SaveFP => "TSANSeqCstStore64SaveFP",
      Self::TSANSeqCstStore8IgnoreFP => "TSANSeqCstStore8IgnoreFP",
      Self::TSANSeqCstStore8SaveFP => "TSANSeqCstStore8SaveFP",
      Self::Trace => "Trace",
      Self::UnsupportedThrower => "UnsupportedThrower",
      Self::WasmAllocateInOldGeneration => "WasmAllocateInOldGeneration",
      Self::WasmAllocateInSharedHeap => "WasmAllocateInSharedHeap",
      Self::WasmAllocateInYoungGeneration => "WasmAllocateInYoungGeneration",
      Self::WasmCEntry => "WasmCEntry",
    }
  }

  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取对应的接口描述符符号
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 是否为 IC 内联缓存内建
  #[inline]
  pub const fn is_ic(&self) -> bool {
    matches!(
      self,
      Self::CloneObjectIC
        | Self::CloneObjectICBaseline
        | Self::CloneObjectICSlow
        | Self::DefineKeyedOwnIC
        | Self::DefineKeyedOwnICBaseline
        | Self::DefineKeyedOwnICTrampoline
        | Self::DefineKeyedOwnICMegamorphic
        | Self::DefineNamedOwnIC
        | Self::DefineNamedOwnICBaseline
        | Self::DefineNamedOwnICTrampoline
        | Self::DefineNamedOwnICNoFeedback
        | Self::EnumeratedKeyedLoadIC
        | Self::EnumeratedKeyedLoadICBaseline
        | Self::HasIndexedInterceptorIC
        | Self::KeyedHasIC
        | Self::KeyedHasICBaseline
        | Self::KeyedHasICMegamorphic
        | Self::KeyedHasICPolymorphicName
        | Self::KeyedHasICSloppyArguments
        | Self::KeyedLoadIC
        | Self::KeyedLoadICBaseline
        | Self::KeyedLoadICTrampoline
        | Self::KeyedLoadICTrampolineMegamorphic
        | Self::KeyedLoadICMegamorphic
        | Self::KeyedLoadICPolymorphicName
        | Self::KeyedLoadICSloppyArguments
        | Self::KeyedStoreIC
        | Self::KeyedStoreICBaseline
        | Self::KeyedStoreICTrampoline
        | Self::KeyedStoreICTrampolineMegamorphic
        | Self::KeyedStoreICMegamorphic
        | Self::KeyedStoreICSloppyArgumentsInBounds
        | Self::KeyedStoreICSloppyArgumentsNoTransitionGrowAndHandleCOW
        | Self::KeyedStoreICSloppyArgumentsNoTransitionHandleCOW
        | Self::KeyedStoreICSloppyArgumentsNoTransitionIgnoreTypedArrayOOB
        | Self::LoadGlobalIC
        | Self::LoadGlobalICBaseline
        | Self::LoadGlobalICInsideTypeof
        | Self::LoadGlobalICInsideTypeofBaseline
        | Self::LoadGlobalICInsideTypeofTrampoline
        | Self::LoadGlobalICTrampoline
        | Self::LoadGlobalICNoFeedback
        | Self::LoadIC
        | Self::LoadICConstantFromPrototypeBaseline
        | Self::LoadICDoubleFieldBaseline
        | Self::LoadICGenericBaseline
        | Self::LoadICInObjectNonDoubleFieldBaseline
        | Self::LoadICOutOfObjectNonDoubleFieldBaseline
        | Self::LoadICStringLengthBaseline
        | Self::LoadICTrampoline
        | Self::LoadICTrampolineMegamorphic
        | Self::LoadICUninitializedBaseline
        | Self::LoadICFunctionPrototype
        | Self::LoadICMegamorphic
        | Self::LoadICNoFeedback
        | Self::LoadICNoninlined
        | Self::LoadICStringLength
        | Self::LoadICStringWrapperLength
        | Self::LoadIndexedInterceptorIC
        | Self::LoadSuperIC
        | Self::LoadSuperICBaseline
        | Self::LookupGlobalIC
        | Self::LookupGlobalICBaseline
        | Self::LookupGlobalICInsideTypeof
        | Self::LookupGlobalICInsideTypeofBaseline
        | Self::LookupGlobalICInsideTypeofTrampoline
        | Self::LookupGlobalICTrampoline
        | Self::StoreFastElementICInBounds
        | Self::StoreFastElementICInBoundsGeneric
        | Self::StoreFastElementICInBoundsISX
        | Self::StoreFastElementICNoTransitionGrowAndHandleCOW
        | Self::StoreFastElementICNoTransitionHandleCOW
        | Self::StoreFastElementICNoTransitionIgnoreTypedArrayOOB
        | Self::StoreGlobalIC
        | Self::StoreGlobalICBaseline
        | Self::StoreGlobalICTrampoline
        | Self::StoreGlobalICSlow
        | Self::StoreIC
        | Self::StoreICBaseline
        | Self::StoreICTrampoline
        | Self::StoreICTrampolineMegamorphic
        | Self::StoreICMegamorphic
        | Self::StoreICNoFeedback
        | Self::StoreInArrayLiteralIC
        | Self::StoreInArrayLiteralICBaseline
    )
  }

  /// 是否为蹦床辅助内建
  #[inline]
  pub const fn is_trampoline(&self) -> bool {
    matches!(
      self,
      Self::DebugBreakTrampoline
        | Self::DefineKeyedOwnICTrampoline
        | Self::DefineNamedOwnICTrampoline
        | Self::InterpreterEntryTrampoline
        | Self::InterpreterEntryTrampolineForProfiling
        | Self::KeyedLoadICTrampoline
        | Self::KeyedLoadICTrampolineMegamorphic
        | Self::KeyedStoreICTrampoline
        | Self::KeyedStoreICTrampolineMegamorphic
        | Self::LoadGlobalICInsideTypeofTrampoline
        | Self::LoadGlobalICTrampoline
        | Self::LoadICTrampoline
        | Self::LoadICTrampolineMegamorphic
        | Self::LookupContextInsideTypeofTrampoline
        | Self::LookupContextNoCellInsideTypeofTrampoline
        | Self::LookupContextNoCellTrampoline
        | Self::LookupContextTrampoline
        | Self::LookupGlobalICInsideTypeofTrampoline
        | Self::LookupGlobalICTrampoline
        | Self::StoreGlobalICTrampoline
        | Self::StoreICTrampoline
        | Self::StoreICTrampolineMegamorphic
    )
  }

  /// 是否为函数调用调度内建
  #[inline]
  pub const fn is_call(&self) -> bool {
    matches!(
      self,
      Self::CallApiCallbackGeneric
        | Self::CallApiCallbackOptimized
        | Self::CallApiCallbackOptimizedNoProfiling
        | Self::CallApiGetter
        | Self::CallAsyncModuleFulfilled
        | Self::CallAsyncModuleRejected
        | Self::CallBoundFunction
        | Self::CallForwardVarargs
        | Self::CallFunctionForwardVarargs
        | Self::CallFunctionTemplateCheckAccess
        | Self::CallFunctionTemplateCheckAccessAndCompatibleReceiver
        | Self::CallFunctionTemplateCheckCompatibleReceiver
        | Self::CallFunctionTemplateGeneric
        | Self::CallFunctionReceiverIsAny
        | Self::CallFunctionReceiverIsNotNullOrUndefined
        | Self::CallFunctionReceiverIsNullOrUndefined
        | Self::CallNamedInterceptorGetter
        | Self::CallNamedInterceptorSetter
        | Self::CallVarargs
        | Self::CallWithArrayLike
        | Self::CallWithArrayLikeWithFeedback
        | Self::CallWithSpread
        | Self::CallWithSpreadBaseline
        | Self::CallWithSpreadWithFeedback
        | Self::CallWrappedFunction
        | Self::CallReceiverIsAny
        | Self::CallReceiverIsAnyBaseline
        | Self::CallReceiverIsAnyBaselineCompact
        | Self::CallReceiverIsAnyWithFeedback
        | Self::CallReceiverIsNotNullOrUndefined
        | Self::CallReceiverIsNotNullOrUndefinedBaseline
        | Self::CallReceiverIsNotNullOrUndefinedBaselineCompact
        | Self::CallReceiverIsNotNullOrUndefinedWithFeedback
        | Self::CallReceiverIsNullOrUndefined
        | Self::CallReceiverIsNullOrUndefinedBaseline
        | Self::CallReceiverIsNullOrUndefinedBaselineCompact
        | Self::CallReceiverIsNullOrUndefinedWithFeedback
    )
  }

  /// 是否为解释器入栈/出栈辅助内建
  #[inline]
  pub const fn is_interpreter(&self) -> bool {
    matches!(
      self,
      Self::InterpreterEntryTrampoline
        | Self::InterpreterEntryTrampolineForProfiling
        | Self::InterpreterPushArgsThenCall
        | Self::InterpreterPushArgsThenCallWithFinalSpread
        | Self::InterpreterPushArgsThenConstruct
        | Self::InterpreterPushArgsThenConstructArrayFunction
        | Self::InterpreterPushArgsThenConstructWithFinalSpread
        | Self::InterpreterPushUndefinedAndArgsThenCall
    )
  }

  /// 是否为 ShadowRealm 隔离内建
  #[inline]
  pub const fn is_shadow_realm(&self) -> bool {
    matches!(
      self,
      Self::ShadowRealmConstructor
        | Self::ShadowRealmGetWrappedValue
        | Self::ShadowRealmImportValueFulfilled
        | Self::ShadowRealmImportValueRejected
        | Self::ShadowRealmPrototypeEvaluate
        | Self::ShadowRealmPrototypeImportValue
    )
  }

  /// 是否为 Atomics 原子同步内建
  #[inline]
  pub const fn is_atomics(&self) -> bool {
    matches!(
      self,
      Self::AtomicsConditionConstructor
        | Self::AtomicsConditionIsCondition
        | Self::AtomicsConditionNotify
        | Self::AtomicsConditionWait
        | Self::AtomicsMutexConstructor
        | Self::AtomicsMutexIsMutex
        | Self::AtomicsMutexLock
        | Self::AtomicsMutexLockWithTimeout
        | Self::AtomicsMutexTryLock
    )
  }
}

// 统一提供 274 项零成本内建工厂函数
pub const fn abort() -> Builtin {
  Builtin::Abort
}

pub const fn abort_csa_dcheck() -> Builtin {
  Builtin::AbortCSADcheck
}

pub const fn abstract_module_source_to_string_tag() -> Builtin {
  Builtin::AbstractModuleSourceToStringTag
}

pub const fn adaptor_with_builtin_exit_frame0() -> Builtin {
  Builtin::AdaptorWithBuiltinExitFrame0
}

pub const fn adaptor_with_builtin_exit_frame1() -> Builtin {
  Builtin::AdaptorWithBuiltinExitFrame1
}

pub const fn adaptor_with_builtin_exit_frame2() -> Builtin {
  Builtin::AdaptorWithBuiltinExitFrame2
}

pub const fn adaptor_with_builtin_exit_frame3() -> Builtin {
  Builtin::AdaptorWithBuiltinExitFrame3
}

pub const fn adaptor_with_builtin_exit_frame4() -> Builtin {
  Builtin::AdaptorWithBuiltinExitFrame4
}

pub const fn adaptor_with_builtin_exit_frame5() -> Builtin {
  Builtin::AdaptorWithBuiltinExitFrame5
}

pub const fn allocate_in_old_generation() -> Builtin {
  Builtin::AllocateInOldGeneration
}

pub const fn allocate_in_young_generation() -> Builtin {
  Builtin::AllocateInYoungGeneration
}

pub const fn atomics_condition_constructor() -> Builtin {
  Builtin::AtomicsConditionConstructor
}

pub const fn atomics_condition_is_condition() -> Builtin {
  Builtin::AtomicsConditionIsCondition
}

pub const fn atomics_condition_notify() -> Builtin {
  Builtin::AtomicsConditionNotify
}

pub const fn atomics_condition_wait() -> Builtin {
  Builtin::AtomicsConditionWait
}

pub const fn atomics_mutex_constructor() -> Builtin {
  Builtin::AtomicsMutexConstructor
}

pub const fn atomics_mutex_is_mutex() -> Builtin {
  Builtin::AtomicsMutexIsMutex
}

pub const fn atomics_mutex_lock() -> Builtin {
  Builtin::AtomicsMutexLock
}

pub const fn atomics_mutex_lock_with_timeout() -> Builtin {
  Builtin::AtomicsMutexLockWithTimeout
}

pub const fn atomics_mutex_try_lock() -> Builtin {
  Builtin::AtomicsMutexTryLock
}

pub const fn baseline_leave_frame() -> Builtin {
  Builtin::BaselineLeaveFrame
}

pub const fn c_entry_return1_argv_in_register_no_builtin_exit() -> Builtin {
  Builtin::CEntryReturn1ArgvInRegisterNoBuiltinExit
}

pub const fn c_entry_return1_argv_on_stack_builtin_exit() -> Builtin {
  Builtin::CEntryReturn1ArgvOnStackBuiltinExit
}

pub const fn c_entry_return1_argv_on_stack_no_builtin_exit() -> Builtin {
  Builtin::CEntryReturn1ArgvOnStackNoBuiltinExit
}

pub const fn c_entry_return2_argv_in_register_no_builtin_exit() -> Builtin {
  Builtin::CEntryReturn2ArgvInRegisterNoBuiltinExit
}

pub const fn c_entry_return2_argv_on_stack_no_builtin_exit() -> Builtin {
  Builtin::CEntryReturn2ArgvOnStackNoBuiltinExit
}

pub const fn call_api_callback_generic() -> Builtin {
  Builtin::CallApiCallbackGeneric
}

pub const fn call_api_callback_optimized() -> Builtin {
  Builtin::CallApiCallbackOptimized
}

pub const fn call_api_callback_optimized_no_profiling() -> Builtin {
  Builtin::CallApiCallbackOptimizedNoProfiling
}

pub const fn call_api_getter() -> Builtin {
  Builtin::CallApiGetter
}

pub const fn call_async_module_fulfilled() -> Builtin {
  Builtin::CallAsyncModuleFulfilled
}

pub const fn call_async_module_rejected() -> Builtin {
  Builtin::CallAsyncModuleRejected
}

pub const fn call_bound_function() -> Builtin {
  Builtin::CallBoundFunction
}

pub const fn call_forward_varargs() -> Builtin {
  Builtin::CallForwardVarargs
}

pub const fn call_function_forward_varargs() -> Builtin {
  Builtin::CallFunctionForwardVarargs
}

pub const fn call_function_template_check_access() -> Builtin {
  Builtin::CallFunctionTemplateCheckAccess
}

pub const fn call_function_template_check_access_and_compatible_receiver() -> Builtin {
  Builtin::CallFunctionTemplateCheckAccessAndCompatibleReceiver
}

pub const fn call_function_template_check_compatible_receiver() -> Builtin {
  Builtin::CallFunctionTemplateCheckCompatibleReceiver
}

pub const fn call_function_template_generic() -> Builtin {
  Builtin::CallFunctionTemplateGeneric
}

pub const fn call_function_receiver_is_any() -> Builtin {
  Builtin::CallFunctionReceiverIsAny
}

pub const fn call_function_receiver_is_not_null_or_undefined() -> Builtin {
  Builtin::CallFunctionReceiverIsNotNullOrUndefined
}

pub const fn call_function_receiver_is_null_or_undefined() -> Builtin {
  Builtin::CallFunctionReceiverIsNullOrUndefined
}

pub const fn call_named_interceptor_getter() -> Builtin {
  Builtin::CallNamedInterceptorGetter
}

pub const fn call_named_interceptor_setter() -> Builtin {
  Builtin::CallNamedInterceptorSetter
}

pub const fn call_varargs() -> Builtin {
  Builtin::CallVarargs
}

pub const fn call_with_array_like() -> Builtin {
  Builtin::CallWithArrayLike
}

pub const fn call_with_array_like_with_feedback() -> Builtin {
  Builtin::CallWithArrayLikeWithFeedback
}

pub const fn call_with_spread() -> Builtin {
  Builtin::CallWithSpread
}

pub const fn call_with_spread_baseline() -> Builtin {
  Builtin::CallWithSpreadBaseline
}

pub const fn call_with_spread_with_feedback() -> Builtin {
  Builtin::CallWithSpreadWithFeedback
}

pub const fn call_wrapped_function() -> Builtin {
  Builtin::CallWrappedFunction
}

pub const fn call_receiver_is_any() -> Builtin {
  Builtin::CallReceiverIsAny
}

pub const fn call_receiver_is_any_baseline() -> Builtin {
  Builtin::CallReceiverIsAnyBaseline
}

pub const fn call_receiver_is_any_baseline_compact() -> Builtin {
  Builtin::CallReceiverIsAnyBaselineCompact
}

pub const fn call_receiver_is_any_with_feedback() -> Builtin {
  Builtin::CallReceiverIsAnyWithFeedback
}

pub const fn call_receiver_is_not_null_or_undefined() -> Builtin {
  Builtin::CallReceiverIsNotNullOrUndefined
}

pub const fn call_receiver_is_not_null_or_undefined_baseline() -> Builtin {
  Builtin::CallReceiverIsNotNullOrUndefinedBaseline
}

pub const fn call_receiver_is_not_null_or_undefined_baseline_compact() -> Builtin {
  Builtin::CallReceiverIsNotNullOrUndefinedBaselineCompact
}

pub const fn call_receiver_is_not_null_or_undefined_with_feedback() -> Builtin {
  Builtin::CallReceiverIsNotNullOrUndefinedWithFeedback
}

pub const fn call_receiver_is_null_or_undefined() -> Builtin {
  Builtin::CallReceiverIsNullOrUndefined
}

pub const fn call_receiver_is_null_or_undefined_baseline() -> Builtin {
  Builtin::CallReceiverIsNullOrUndefinedBaseline
}

pub const fn call_receiver_is_null_or_undefined_baseline_compact() -> Builtin {
  Builtin::CallReceiverIsNullOrUndefinedBaselineCompact
}

pub const fn call_receiver_is_null_or_undefined_with_feedback() -> Builtin {
  Builtin::CallReceiverIsNullOrUndefinedWithFeedback
}

pub const fn check_maglev_type() -> Builtin {
  Builtin::CheckMaglevType
}

pub const fn clone_object_ic() -> Builtin {
  Builtin::CloneObjectIC
}

pub const fn clone_object_ic_baseline() -> Builtin {
  Builtin::CloneObjectICBaseline
}

pub const fn clone_object_ic_slow() -> Builtin {
  Builtin::CloneObjectICSlow
}

pub const fn compile_lazy() -> Builtin {
  Builtin::CompileLazy
}

pub const fn copy_data_properties() -> Builtin {
  Builtin::CopyDataProperties
}

pub const fn copy_data_properties_with_excluded_properties() -> Builtin {
  Builtin::CopyDataPropertiesWithExcludedProperties
}

pub const fn copy_data_properties_with_excluded_properties_on_stack() -> Builtin {
  Builtin::CopyDataPropertiesWithExcludedPropertiesOnStack
}

pub const fn copy_fast_smi_or_object_elements() -> Builtin {
  Builtin::CopyFastSmiOrObjectElements
}

pub const fn create_data_property() -> Builtin {
  Builtin::CreateDataProperty
}

pub const fn debug_break_trampoline() -> Builtin {
  Builtin::DebugBreakTrampoline
}

pub const fn define_keyed_own_ic() -> Builtin {
  Builtin::DefineKeyedOwnIC
}

pub const fn define_keyed_own_ic_baseline() -> Builtin {
  Builtin::DefineKeyedOwnICBaseline
}

pub const fn define_keyed_own_ic_trampoline() -> Builtin {
  Builtin::DefineKeyedOwnICTrampoline
}

pub const fn define_keyed_own_ic_megamorphic() -> Builtin {
  Builtin::DefineKeyedOwnICMegamorphic
}

pub const fn define_named_own_ic() -> Builtin {
  Builtin::DefineNamedOwnIC
}

pub const fn define_named_own_ic_baseline() -> Builtin {
  Builtin::DefineNamedOwnICBaseline
}

pub const fn define_named_own_ic_trampoline() -> Builtin {
  Builtin::DefineNamedOwnICTrampoline
}

pub const fn define_named_own_ic_no_feedback() -> Builtin {
  Builtin::DefineNamedOwnICNoFeedback
}

pub const fn delete_property() -> Builtin {
  Builtin::DeleteProperty
}

pub const fn elements_transition_and_store_in_bounds() -> Builtin {
  Builtin::ElementsTransitionAndStoreInBounds
}

pub const fn elements_transition_and_store_no_transition_grow_and_handle_cow() -> Builtin {
  Builtin::ElementsTransitionAndStoreNoTransitionGrowAndHandleCOW
}

pub const fn elements_transition_and_store_no_transition_handle_cow() -> Builtin {
  Builtin::ElementsTransitionAndStoreNoTransitionHandleCOW
}

pub const fn elements_transition_and_store_no_transition_ignore_typed_array_oob() -> Builtin {
  Builtin::ElementsTransitionAndStoreNoTransitionIgnoreTypedArrayOOB
}

pub const fn empty_function() -> Builtin {
  Builtin::EmptyFunction
}

pub const fn empty_function1() -> Builtin {
  Builtin::EmptyFunction1
}

pub const fn enumerated_keyed_load_ic() -> Builtin {
  Builtin::EnumeratedKeyedLoadIC
}

pub const fn enumerated_keyed_load_ic_baseline() -> Builtin {
  Builtin::EnumeratedKeyedLoadICBaseline
}

pub const fn ephemeron_key_barrier_ignore_fp() -> Builtin {
  Builtin::EphemeronKeyBarrierIgnoreFP
}

pub const fn ephemeron_key_barrier_save_fp() -> Builtin {
  Builtin::EphemeronKeyBarrierSaveFP
}

pub const fn finalization_registry_unregister() -> Builtin {
  Builtin::FinalizationRegistryUnregister
}

pub const fn find_non_default_constructor_or_construct() -> Builtin {
  Builtin::FindNonDefaultConstructorOrConstruct
}

pub const fn for_in_enumerate() -> Builtin {
  Builtin::ForInEnumerate
}

pub const fn for_in_filter() -> Builtin {
  Builtin::ForInFilter
}

pub const fn for_in_prepare() -> Builtin {
  Builtin::ForInPrepare
}

pub const fn function_log_next_execution() -> Builtin {
  Builtin::FunctionLogNextExecution
}

pub const fn get_own_property_descriptor() -> Builtin {
  Builtin::GetOwnPropertyDescriptor
}

pub const fn get_property() -> Builtin {
  Builtin::GetProperty
}

pub const fn get_property_with_receiver() -> Builtin {
  Builtin::GetPropertyWithReceiver
}

pub const fn global_decode_uri() -> Builtin {
  Builtin::GlobalDecodeURI
}

pub const fn global_decode_uri_component() -> Builtin {
  Builtin::GlobalDecodeURIComponent
}

pub const fn global_encode_uri() -> Builtin {
  Builtin::GlobalEncodeURI
}

pub const fn global_encode_uri_component() -> Builtin {
  Builtin::GlobalEncodeURIComponent
}

pub const fn global_escape() -> Builtin {
  Builtin::GlobalEscape
}

pub const fn global_eval() -> Builtin {
  Builtin::GlobalEval
}

pub const fn global_is_finite() -> Builtin {
  Builtin::GlobalIsFinite
}

pub const fn global_is_na_n() -> Builtin {
  Builtin::GlobalIsNaN
}

pub const fn global_unescape() -> Builtin {
  Builtin::GlobalUnescape
}

pub const fn grow_fast_double_elements() -> Builtin {
  Builtin::GrowFastDoubleElements
}

pub const fn grow_fast_smi_or_object_elements() -> Builtin {
  Builtin::GrowFastSmiOrObjectElements
}

pub const fn handle_api_call_as_constructor_delegate() -> Builtin {
  Builtin::HandleApiCallAsConstructorDelegate
}

pub const fn handle_api_call_as_function_delegate() -> Builtin {
  Builtin::HandleApiCallAsFunctionDelegate
}

pub const fn handle_api_call_or_construct() -> Builtin {
  Builtin::HandleApiCallOrConstruct
}

pub const fn handle_api_construct() -> Builtin {
  Builtin::HandleApiConstruct
}

pub const fn has_indexed_interceptor_ic() -> Builtin {
  Builtin::HasIndexedInterceptorIC
}

pub const fn illegal() -> Builtin {
  Builtin::Illegal
}

pub const fn illegal_invocation_thrower() -> Builtin {
  Builtin::IllegalInvocationThrower
}

pub const fn indirect_pointer_barrier_ignore_fp() -> Builtin {
  Builtin::IndirectPointerBarrierIgnoreFP
}

pub const fn indirect_pointer_barrier_save_fp() -> Builtin {
  Builtin::IndirectPointerBarrierSaveFP
}

pub const fn interpreter_entry_trampoline() -> Builtin {
  Builtin::InterpreterEntryTrampoline
}

pub const fn interpreter_entry_trampoline_for_profiling() -> Builtin {
  Builtin::InterpreterEntryTrampolineForProfiling
}

pub const fn interpreter_push_args_then_call() -> Builtin {
  Builtin::InterpreterPushArgsThenCall
}

pub const fn interpreter_push_args_then_call_with_final_spread() -> Builtin {
  Builtin::InterpreterPushArgsThenCallWithFinalSpread
}

pub const fn interpreter_push_args_then_construct() -> Builtin {
  Builtin::InterpreterPushArgsThenConstruct
}

pub const fn interpreter_push_args_then_construct_array_function() -> Builtin {
  Builtin::InterpreterPushArgsThenConstructArrayFunction
}

pub const fn interpreter_push_args_then_construct_with_final_spread() -> Builtin {
  Builtin::InterpreterPushArgsThenConstructWithFinalSpread
}

pub const fn interpreter_push_undefined_and_args_then_call() -> Builtin {
  Builtin::InterpreterPushUndefinedAndArgsThenCall
}

pub const fn is_trace_category_enabled() -> Builtin {
  Builtin::IsTraceCategoryEnabled
}

pub const fn keyed_has_ic() -> Builtin {
  Builtin::KeyedHasIC
}

pub const fn keyed_has_ic_baseline() -> Builtin {
  Builtin::KeyedHasICBaseline
}

pub const fn keyed_has_ic_megamorphic() -> Builtin {
  Builtin::KeyedHasICMegamorphic
}

pub const fn keyed_has_ic_polymorphic_name() -> Builtin {
  Builtin::KeyedHasICPolymorphicName
}

pub const fn keyed_has_ic_sloppy_arguments() -> Builtin {
  Builtin::KeyedHasICSloppyArguments
}

pub const fn keyed_load_ic() -> Builtin {
  Builtin::KeyedLoadIC
}

pub const fn keyed_load_ic_baseline() -> Builtin {
  Builtin::KeyedLoadICBaseline
}

pub const fn keyed_load_ic_trampoline() -> Builtin {
  Builtin::KeyedLoadICTrampoline
}

pub const fn keyed_load_ic_trampoline_megamorphic() -> Builtin {
  Builtin::KeyedLoadICTrampolineMegamorphic
}

pub const fn keyed_load_ic_megamorphic() -> Builtin {
  Builtin::KeyedLoadICMegamorphic
}

pub const fn keyed_load_ic_polymorphic_name() -> Builtin {
  Builtin::KeyedLoadICPolymorphicName
}

pub const fn keyed_load_ic_sloppy_arguments() -> Builtin {
  Builtin::KeyedLoadICSloppyArguments
}

pub const fn keyed_store_ic() -> Builtin {
  Builtin::KeyedStoreIC
}

pub const fn keyed_store_ic_baseline() -> Builtin {
  Builtin::KeyedStoreICBaseline
}

pub const fn keyed_store_ic_trampoline() -> Builtin {
  Builtin::KeyedStoreICTrampoline
}

pub const fn keyed_store_ic_trampoline_megamorphic() -> Builtin {
  Builtin::KeyedStoreICTrampolineMegamorphic
}

pub const fn keyed_store_ic_megamorphic() -> Builtin {
  Builtin::KeyedStoreICMegamorphic
}

pub const fn keyed_store_ic_sloppy_arguments_in_bounds() -> Builtin {
  Builtin::KeyedStoreICSloppyArgumentsInBounds
}

pub const fn keyed_store_ic_sloppy_arguments_no_transition_grow_and_handle_cow() -> Builtin {
  Builtin::KeyedStoreICSloppyArgumentsNoTransitionGrowAndHandleCOW
}

pub const fn keyed_store_ic_sloppy_arguments_no_transition_handle_cow() -> Builtin {
  Builtin::KeyedStoreICSloppyArgumentsNoTransitionHandleCOW
}

pub const fn keyed_store_ic_sloppy_arguments_no_transition_ignore_typed_array_oob() -> Builtin {
  Builtin::KeyedStoreICSloppyArgumentsNoTransitionIgnoreTypedArrayOOB
}

pub const fn load_global_ic() -> Builtin {
  Builtin::LoadGlobalIC
}

pub const fn load_global_ic_baseline() -> Builtin {
  Builtin::LoadGlobalICBaseline
}

pub const fn load_global_ic_inside_typeof() -> Builtin {
  Builtin::LoadGlobalICInsideTypeof
}

pub const fn load_global_ic_inside_typeof_baseline() -> Builtin {
  Builtin::LoadGlobalICInsideTypeofBaseline
}

pub const fn load_global_ic_inside_typeof_trampoline() -> Builtin {
  Builtin::LoadGlobalICInsideTypeofTrampoline
}

pub const fn load_global_ic_trampoline() -> Builtin {
  Builtin::LoadGlobalICTrampoline
}

pub const fn load_global_ic_no_feedback() -> Builtin {
  Builtin::LoadGlobalICNoFeedback
}

pub const fn load_ic() -> Builtin {
  Builtin::LoadIC
}

pub const fn load_ic_constant_from_prototype_baseline() -> Builtin {
  Builtin::LoadICConstantFromPrototypeBaseline
}

pub const fn load_ic_double_field_baseline() -> Builtin {
  Builtin::LoadICDoubleFieldBaseline
}

pub const fn load_ic_generic_baseline() -> Builtin {
  Builtin::LoadICGenericBaseline
}

pub const fn load_ic_in_object_non_double_field_baseline() -> Builtin {
  Builtin::LoadICInObjectNonDoubleFieldBaseline
}

pub const fn load_ic_out_of_object_non_double_field_baseline() -> Builtin {
  Builtin::LoadICOutOfObjectNonDoubleFieldBaseline
}

pub const fn load_ic_string_length_baseline() -> Builtin {
  Builtin::LoadICStringLengthBaseline
}

pub const fn load_ic_trampoline() -> Builtin {
  Builtin::LoadICTrampoline
}

pub const fn load_ic_trampoline_megamorphic() -> Builtin {
  Builtin::LoadICTrampolineMegamorphic
}

pub const fn load_ic_uninitialized_baseline() -> Builtin {
  Builtin::LoadICUninitializedBaseline
}

pub const fn load_ic_function_prototype() -> Builtin {
  Builtin::LoadICFunctionPrototype
}

pub const fn load_ic_megamorphic() -> Builtin {
  Builtin::LoadICMegamorphic
}

pub const fn load_ic_no_feedback() -> Builtin {
  Builtin::LoadICNoFeedback
}

pub const fn load_ic_noninlined() -> Builtin {
  Builtin::LoadICNoninlined
}

pub const fn load_ic_string_length() -> Builtin {
  Builtin::LoadICStringLength
}

pub const fn load_ic_string_wrapper_length() -> Builtin {
  Builtin::LoadICStringWrapperLength
}

pub const fn load_indexed_interceptor_ic() -> Builtin {
  Builtin::LoadIndexedInterceptorIC
}

pub const fn load_super_ic() -> Builtin {
  Builtin::LoadSuperIC
}

pub const fn load_super_ic_baseline() -> Builtin {
  Builtin::LoadSuperICBaseline
}

pub const fn lookup_context_inside_typeof_baseline() -> Builtin {
  Builtin::LookupContextInsideTypeofBaseline
}

pub const fn lookup_context_inside_typeof_trampoline() -> Builtin {
  Builtin::LookupContextInsideTypeofTrampoline
}

pub const fn lookup_context_no_cell_baseline() -> Builtin {
  Builtin::LookupContextNoCellBaseline
}

pub const fn lookup_context_no_cell_inside_typeof_baseline() -> Builtin {
  Builtin::LookupContextNoCellInsideTypeofBaseline
}

pub const fn lookup_context_no_cell_inside_typeof_trampoline() -> Builtin {
  Builtin::LookupContextNoCellInsideTypeofTrampoline
}

pub const fn lookup_context_no_cell_trampoline() -> Builtin {
  Builtin::LookupContextNoCellTrampoline
}

pub const fn lookup_context_trampoline() -> Builtin {
  Builtin::LookupContextTrampoline
}

pub const fn lookup_global_ic() -> Builtin {
  Builtin::LookupGlobalIC
}

pub const fn lookup_global_ic_baseline() -> Builtin {
  Builtin::LookupGlobalICBaseline
}

pub const fn lookup_global_ic_inside_typeof() -> Builtin {
  Builtin::LookupGlobalICInsideTypeof
}

pub const fn lookup_global_ic_inside_typeof_baseline() -> Builtin {
  Builtin::LookupGlobalICInsideTypeofBaseline
}

pub const fn lookup_global_ic_inside_typeof_trampoline() -> Builtin {
  Builtin::LookupGlobalICInsideTypeofTrampoline
}

pub const fn lookup_global_ic_trampoline() -> Builtin {
  Builtin::LookupGlobalICTrampoline
}

pub const fn lookup_script_context_baseline() -> Builtin {
  Builtin::LookupScriptContextBaseline
}

pub const fn maglev_function_entry_stack_check() -> Builtin {
  Builtin::MaglevFunctionEntryStackCheck
}

pub const fn maglev_function_entry_stack_check_with_new_target() -> Builtin {
  Builtin::MaglevFunctionEntryStackCheckWithNewTarget
}

pub const fn maglev_function_entry_stack_check_without_new_target() -> Builtin {
  Builtin::MaglevFunctionEntryStackCheckWithoutNewTarget
}

pub const fn maglev_optimize_code_or_tail_call_optimized_code_slot() -> Builtin {
  Builtin::MaglevOptimizeCodeOrTailCallOptimizedCodeSlot
}

pub const fn mark_flushed() -> Builtin {
  Builtin::MarkFlushed
}

pub const fn mark_lazy_deoptimized() -> Builtin {
  Builtin::MarkLazyDeoptimized
}

pub const fn mark_reoptimize_lazy_deoptimized() -> Builtin {
  Builtin::MarkReoptimizeLazyDeoptimized
}

pub const fn new_heap_number() -> Builtin {
  Builtin::NewHeapNumber
}

pub const fn optimize_maglev_eager() -> Builtin {
  Builtin::OptimizeMaglevEager
}

pub const fn optimize_turbofan_eager() -> Builtin {
  Builtin::OptimizeTurbofanEager
}

pub const fn record_write_ignore_fp() -> Builtin {
  Builtin::RecordWriteIgnoreFP
}

pub const fn record_write_save_fp() -> Builtin {
  Builtin::RecordWriteSaveFP
}

pub const fn return_receiver() -> Builtin {
  Builtin::ReturnReceiver
}

pub const fn same_value() -> Builtin {
  Builtin::SameValue
}

pub const fn same_value_numbers_only() -> Builtin {
  Builtin::SameValueNumbersOnly
}

pub const fn set_data_properties() -> Builtin {
  Builtin::SetDataProperties
}

pub const fn set_property() -> Builtin {
  Builtin::SetProperty
}

pub const fn shadow_realm_constructor() -> Builtin {
  Builtin::ShadowRealmConstructor
}

pub const fn shadow_realm_get_wrapped_value() -> Builtin {
  Builtin::ShadowRealmGetWrappedValue
}

pub const fn shadow_realm_import_value_fulfilled() -> Builtin {
  Builtin::ShadowRealmImportValueFulfilled
}

pub const fn shadow_realm_import_value_rejected() -> Builtin {
  Builtin::ShadowRealmImportValueRejected
}

pub const fn shadow_realm_prototype_evaluate() -> Builtin {
  Builtin::ShadowRealmPrototypeEvaluate
}

pub const fn shadow_realm_prototype_import_value() -> Builtin {
  Builtin::ShadowRealmPrototypeImportValue
}

pub const fn shared_space_js_object_has_instance() -> Builtin {
  Builtin::SharedSpaceJSObjectHasInstance
}

pub const fn shared_struct_constructor() -> Builtin {
  Builtin::SharedStructConstructor
}

pub const fn shared_struct_type_constructor() -> Builtin {
  Builtin::SharedStructTypeConstructor
}

pub const fn shared_struct_type_is_shared_struct() -> Builtin {
  Builtin::SharedStructTypeIsSharedStruct
}

pub const fn start_maglev_optimize_job() -> Builtin {
  Builtin::StartMaglevOptimizeJob
}

pub const fn start_turbofan_optimize_job() -> Builtin {
  Builtin::StartTurbofanOptimizeJob
}

pub const fn store_fast_element_ic_in_bounds() -> Builtin {
  Builtin::StoreFastElementICInBounds
}

pub const fn store_fast_element_ic_in_bounds_generic() -> Builtin {
  Builtin::StoreFastElementICInBoundsGeneric
}

pub const fn store_fast_element_ic_in_bounds_isx() -> Builtin {
  Builtin::StoreFastElementICInBoundsISX
}

pub const fn store_fast_element_ic_no_transition_grow_and_handle_cow() -> Builtin {
  Builtin::StoreFastElementICNoTransitionGrowAndHandleCOW
}

pub const fn store_fast_element_ic_no_transition_handle_cow() -> Builtin {
  Builtin::StoreFastElementICNoTransitionHandleCOW
}

pub const fn store_fast_element_ic_no_transition_ignore_typed_array_oob() -> Builtin {
  Builtin::StoreFastElementICNoTransitionIgnoreTypedArrayOOB
}

pub const fn store_global_ic() -> Builtin {
  Builtin::StoreGlobalIC
}

pub const fn store_global_ic_baseline() -> Builtin {
  Builtin::StoreGlobalICBaseline
}

pub const fn store_global_ic_trampoline() -> Builtin {
  Builtin::StoreGlobalICTrampoline
}

pub const fn store_global_ic_slow() -> Builtin {
  Builtin::StoreGlobalICSlow
}

pub const fn store_ic() -> Builtin {
  Builtin::StoreIC
}

pub const fn store_ic_baseline() -> Builtin {
  Builtin::StoreICBaseline
}

pub const fn store_ic_trampoline() -> Builtin {
  Builtin::StoreICTrampoline
}

pub const fn store_ic_trampoline_megamorphic() -> Builtin {
  Builtin::StoreICTrampolineMegamorphic
}

pub const fn store_ic_megamorphic() -> Builtin {
  Builtin::StoreICMegamorphic
}

pub const fn store_ic_no_feedback() -> Builtin {
  Builtin::StoreICNoFeedback
}

pub const fn store_in_array_literal_ic() -> Builtin {
  Builtin::StoreInArrayLiteralIC
}

pub const fn store_in_array_literal_ic_baseline() -> Builtin {
  Builtin::StoreInArrayLiteralICBaseline
}

pub const fn strict_poison_pill_thrower() -> Builtin {
  Builtin::StrictPoisonPillThrower
}

pub const fn tsan_relaxed_load32_ignore_fp() -> Builtin {
  Builtin::TSANRelaxedLoad32IgnoreFP
}

pub const fn tsan_relaxed_load32_save_fp() -> Builtin {
  Builtin::TSANRelaxedLoad32SaveFP
}

pub const fn tsan_relaxed_load64_ignore_fp() -> Builtin {
  Builtin::TSANRelaxedLoad64IgnoreFP
}

pub const fn tsan_relaxed_load64_save_fp() -> Builtin {
  Builtin::TSANRelaxedLoad64SaveFP
}

pub const fn tsan_relaxed_store16_ignore_fp() -> Builtin {
  Builtin::TSANRelaxedStore16IgnoreFP
}

pub const fn tsan_relaxed_store16_save_fp() -> Builtin {
  Builtin::TSANRelaxedStore16SaveFP
}

pub const fn tsan_relaxed_store32_ignore_fp() -> Builtin {
  Builtin::TSANRelaxedStore32IgnoreFP
}

pub const fn tsan_relaxed_store32_save_fp() -> Builtin {
  Builtin::TSANRelaxedStore32SaveFP
}

pub const fn tsan_relaxed_store64_ignore_fp() -> Builtin {
  Builtin::TSANRelaxedStore64IgnoreFP
}

pub const fn tsan_relaxed_store64_save_fp() -> Builtin {
  Builtin::TSANRelaxedStore64SaveFP
}

pub const fn tsan_relaxed_store8_ignore_fp() -> Builtin {
  Builtin::TSANRelaxedStore8IgnoreFP
}

pub const fn tsan_relaxed_store8_save_fp() -> Builtin {
  Builtin::TSANRelaxedStore8SaveFP
}

pub const fn tsan_release_store16_ignore_fp() -> Builtin {
  Builtin::TSANReleaseStore16IgnoreFP
}

pub const fn tsan_release_store16_save_fp() -> Builtin {
  Builtin::TSANReleaseStore16SaveFP
}

pub const fn tsan_release_store32_ignore_fp() -> Builtin {
  Builtin::TSANReleaseStore32IgnoreFP
}

pub const fn tsan_release_store32_save_fp() -> Builtin {
  Builtin::TSANReleaseStore32SaveFP
}

pub const fn tsan_release_store64_ignore_fp() -> Builtin {
  Builtin::TSANReleaseStore64IgnoreFP
}

pub const fn tsan_release_store64_save_fp() -> Builtin {
  Builtin::TSANReleaseStore64SaveFP
}

pub const fn tsan_release_store8_ignore_fp() -> Builtin {
  Builtin::TSANReleaseStore8IgnoreFP
}

pub const fn tsan_release_store8_save_fp() -> Builtin {
  Builtin::TSANReleaseStore8SaveFP
}

pub const fn tsan_seq_cst_store16_ignore_fp() -> Builtin {
  Builtin::TSANSeqCstStore16IgnoreFP
}

pub const fn tsan_seq_cst_store16_save_fp() -> Builtin {
  Builtin::TSANSeqCstStore16SaveFP
}

pub const fn tsan_seq_cst_store32_ignore_fp() -> Builtin {
  Builtin::TSANSeqCstStore32IgnoreFP
}

pub const fn tsan_seq_cst_store32_save_fp() -> Builtin {
  Builtin::TSANSeqCstStore32SaveFP
}

pub const fn tsan_seq_cst_store64_ignore_fp() -> Builtin {
  Builtin::TSANSeqCstStore64IgnoreFP
}

pub const fn tsan_seq_cst_store64_save_fp() -> Builtin {
  Builtin::TSANSeqCstStore64SaveFP
}

pub const fn tsan_seq_cst_store8_ignore_fp() -> Builtin {
  Builtin::TSANSeqCstStore8IgnoreFP
}

pub const fn tsan_seq_cst_store8_save_fp() -> Builtin {
  Builtin::TSANSeqCstStore8SaveFP
}

pub const fn trace() -> Builtin {
  Builtin::Trace
}

pub const fn unsupported_thrower() -> Builtin {
  Builtin::UnsupportedThrower
}

pub const fn wasm_allocate_in_old_generation() -> Builtin {
  Builtin::WasmAllocateInOldGeneration
}

pub const fn wasm_allocate_in_shared_heap() -> Builtin {
  Builtin::WasmAllocateInSharedHeap
}

pub const fn wasm_allocate_in_young_generation() -> Builtin {
  Builtin::WasmAllocateInYoungGeneration
}

pub const fn wasm_c_entry() -> Builtin {
  Builtin::WasmCEntry
}

/// 来自 system 相关 .tq 的声明元数据 (共 27 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("macro", "CollectCallFeedback"),
  ("macro", "CollectInstanceOfFeedback"),
  ("macro", "CollectConstructFeedback"),
  ("extern macro", "MegamorphicSymbolConstant"),
  ("extern macro", "UninitializedSymbolConstant"),
  ("macro", "IsMegamorphic"),
  ("macro", "IsUninitialized"),
  ("extern macro", "LoadFeedbackVectorSlot"),
  ("extern macro", "LoadFeedbackVectorSlot"),
  ("extern macro", "StoreFeedbackVectorSlot"),
  ("extern macro", "StoreFeedbackVectorSlot"),
  ("extern macro", "StoreWeakReferenceInFeedbackVector"),
  ("extern macro", "ReportFeedbackUpdate"),
  ("extern macro", "IncrementCallCount"),
  ("macro", "IsMonomorphic"),
  ("macro", "InSameNativeContext"),
  ("macro", "TryInitializeAsMonomorphic"),
  ("macro", "TransitionToMegamorphic"),
  ("macro", "TaggedEqualPrototypeApplyFunction"),
  ("macro", "FeedbackValueIsReceiver"),
  ("macro", "SetCallFeedbackContent"),
  ("macro", "CollectCallFeedback"),
  ("macro", "CollectInstanceOfFeedback"),
  ("macro", "BothTaggedEqualArrayFunction"),
  ("extern macro", "CreateAllocationSiteInFeedbackVector"),
  ("macro", "CastFeedbackVector"),
  ("macro", "CollectConstructFeedback"),
];
