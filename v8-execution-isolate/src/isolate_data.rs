// 由 tool/src/execution/isolate.js 从 v8/src/execution/isolate.h, src/execution/isolate.cc, src/execution/isolate-inl.h, src/execution/isolate-data.h, src/execution/isolate-data-fields.h, src/execution/isolate-utils.h, src/execution/isolate-utils-inl.h 生成; 修订就地改, 复跑覆盖
// IsolateData 核心字段标识符与字段布局

/// Isolate 内部数据字段标识符 (对应原版 enum class IsolateFieldId : uint8_t)
/// 按原版 ISOLATE_DATA_FIELDS 与 ISOLATE_DATA_SUBFIELDS 顺序展开
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum IsolateFieldId {
  /// kCageBase = 0
  CageBase = 0,
  /// kStackGuard = 1
  StackGuard = 1,
  /// kIsMarkingFlag = 2
  IsMarkingFlag = 2,
  /// kIsMinorMarkingFlag = 3
  IsMinorMarkingFlag = 3,
  /// kIsSharedSpaceIsolateFlag = 4
  IsSharedSpaceIsolateFlag = 4,
  /// kUsesSharedHeapFlag = 5
  UsesSharedHeapFlag = 5,
  /// kExecutionMode = 6
  ExecutionMode = 6,
  /// kStackIsIterable = 7
  StackIsIterable = 7,
  /// kErrorMessageParam = 8
  ErrorMessageParam = 8,
  /// kHasLazyClosures = 9
  HasLazyClosures = 9,
  /// kTablesAlignmentPadding = 10
  TablesAlignmentPadding = 10,
  /// kRegExpStaticResultOffsetsVector = 11
  RegExpStaticResultOffsetsVector = 11,
  /// kBuiltinTier0EntryTable = 12
  BuiltinTier0EntryTable = 12,
  /// kBuiltinsTier0Table = 13
  BuiltinsTier0Table = 13,
  /// kNewAllocationInfo = 14
  NewAllocationInfo = 14,
  /// kOldAllocationInfo = 15
  OldAllocationInfo = 15,
  /// kLastYoungAllocation = 16
  LastYoungAllocation = 16,
  /// kFastCCallAlignmentPaddingOffset = 17
  FastCCallAlignmentPaddingOffset = 17,
  /// kFastCCallCallerPC = 18
  FastCCallCallerPC = 18,
  /// kFastCCallCallerFP = 19
  FastCCallCallerFP = 19,
  /// kFastApiCallTarget = 20
  FastApiCallTarget = 20,
  /// kLongTaskStatsCounter = 21
  LongTaskStatsCounter = 21,
  /// kThreadLocalTop = 22
  ThreadLocalTop = 22,
  /// kHandleScopeData = 23
  HandleScopeData = 23,
  /// kHandleScopeImplementer = 24
  HandleScopeImplementer = 24,
  /// kEmbedderData = 25
  EmbedderData = 25,
  /// kExternalPointerTable = 26
  ExternalPointerTable = 26,
  /// kSharedExternalPointerTable = 27
  SharedExternalPointerTable = 27,
  /// kCppHeapPointerTable = 28
  CppHeapPointerTable = 28,
  /// kTrustedCageBase = 29
  TrustedCageBase = 29,
  /// kTrustedPointerTable = 30
  TrustedPointerTable = 30,
  /// kSharedTrustedPointerTable = 31
  SharedTrustedPointerTable = 31,
  /// kTrustedPointerPublishingScope = 32
  TrustedPointerPublishingScope = 32,
  /// kJSDispatchTable = 33
  JSDispatchTable = 33,
  /// kApiCallbackThunkArgument = 34
  ApiCallbackThunkArgument = 34,
  /// kRegexpExecVectorArgument = 35
  RegexpExecVectorArgument = 35,
  /// kContinuationPreservedEmbedderData = 36
  ContinuationPreservedEmbedderData = 36,
  /// kCurrentMicrotaskQueue = 37
  CurrentMicrotaskQueue = 37,
  /// kCurrentMicrotaskNativeContext = 38
  CurrentMicrotaskNativeContext = 38,
  /// kRootsTable = 39
  RootsTable = 39,
  /// kExternalReferenceTable = 40
  ExternalReferenceTable = 40,
  /// kBuiltinEntryTable = 41
  BuiltinEntryTable = 41,
  /// kBuiltinTable = 42
  BuiltinTable = 42,
  /// kActiveStack = 43
  ActiveStack = 43,
  /// kActiveSuspender = 44
  ActiveSuspender = 44,
  /// kDateCacheStamp = 45
  DateCacheStamp = 45,
  /// kIsDateCacheUsed = 46
  IsDateCacheUsed = 46,
  /// kDefaultLocaleMayRequireSpecialCaseMapping = 47
  DefaultLocaleMayRequireSpecialCaseMapping = 47,
  /// kRawArgumentsPadding = 48
  RawArgumentsPadding = 48,
  /// kRawArguments = 49
  RawArguments = 49,
  /// kStressDeoptCount = 50
  StressDeoptCount = 50,
  /// kCpuFeatures = 51
  CpuFeatures = 51,
  /// kBuiltinDispatchTable = 52
  BuiltinDispatchTable = 52,
  /// kIsolateAddress = 53
  IsolateAddress = 53,
  /// kContext = 54
  Context = 54,
  /// kLastEnteredContext = 55
  LastEnteredContext = 55,
  /// kException = 56
  Exception = 56,
  /// kTopmostScriptHavingContext = 57
  TopmostScriptHavingContext = 57,
  /// kCEntryFP = 58
  CEntryFP = 58,
  /// kHandler = 59
  Handler = 59,
  /// kCFunction = 60
  CFunction = 60,
  /// kPendingHandlerContext = 61
  PendingHandlerContext = 61,
  /// kPendingHandlerEntrypoint = 62
  PendingHandlerEntrypoint = 62,
  /// kPendingHandlerConstantPool = 63
  PendingHandlerConstantPool = 63,
  /// kPendingHandlerFP = 64
  PendingHandlerFP = 64,
  /// kPendingHandlerSP = 65
  PendingHandlerSP = 65,
  /// kNumFramesAbovePendingHandler = 66
  NumFramesAbovePendingHandler = 66,
  /// kIsOnCentralStackFlag = 67
  IsOnCentralStackFlag = 67,
  /// kJSEntrySP = 68
  JSEntrySP = 68,
  /// kHandleScopeLevel = 69
  HandleScopeLevel = 69,
  /// kHandleScopeNext = 70
  HandleScopeNext = 70,
  /// kHandleScopeLimit = 71
  HandleScopeLimit = 71,
  /// kEnteredContextCount = 72
  EnteredContextCount = 72,
  /// kEnteredContextCapacity = 73
  EnteredContextCapacity = 73,
  /// kEnteredContextData = 74
  EnteredContextData = 74,
  /// kNewAllocationInfoStart = 75
  NewAllocationInfoStart = 75,
  /// kNewAllocationInfoTop = 76
  NewAllocationInfoTop = 76,
  /// kNewAllocationInfoLimit = 77
  NewAllocationInfoLimit = 77,
  /// kOldAllocationInfoStart = 78
  OldAllocationInfoStart = 78,
  /// kOldAllocationInfoTop = 79
  OldAllocationInfoTop = 79,
  /// kOldAllocationInfoLimit = 80
  OldAllocationInfoLimit = 80,
  /// kJsLimit = 81
  JsLimit = 81,
  /// kRealJsLimit = 82
  RealJsLimit = 82,
  /// kNoHeapWriteInterruptRequest = 83
  NoHeapWriteInterruptRequest = 83,
}

impl IsolateFieldId {
  /// 字段总数
  pub const COUNT: usize = 84;

  /// 全部字段 ID 列表
  pub const ALL: [IsolateFieldId; 84] = [
    IsolateFieldId::CageBase,
    IsolateFieldId::StackGuard,
    IsolateFieldId::IsMarkingFlag,
    IsolateFieldId::IsMinorMarkingFlag,
    IsolateFieldId::IsSharedSpaceIsolateFlag,
    IsolateFieldId::UsesSharedHeapFlag,
    IsolateFieldId::ExecutionMode,
    IsolateFieldId::StackIsIterable,
    IsolateFieldId::ErrorMessageParam,
    IsolateFieldId::HasLazyClosures,
    IsolateFieldId::TablesAlignmentPadding,
    IsolateFieldId::RegExpStaticResultOffsetsVector,
    IsolateFieldId::BuiltinTier0EntryTable,
    IsolateFieldId::BuiltinsTier0Table,
    IsolateFieldId::NewAllocationInfo,
    IsolateFieldId::OldAllocationInfo,
    IsolateFieldId::LastYoungAllocation,
    IsolateFieldId::FastCCallAlignmentPaddingOffset,
    IsolateFieldId::FastCCallCallerPC,
    IsolateFieldId::FastCCallCallerFP,
    IsolateFieldId::FastApiCallTarget,
    IsolateFieldId::LongTaskStatsCounter,
    IsolateFieldId::ThreadLocalTop,
    IsolateFieldId::HandleScopeData,
    IsolateFieldId::HandleScopeImplementer,
    IsolateFieldId::EmbedderData,
    IsolateFieldId::ExternalPointerTable,
    IsolateFieldId::SharedExternalPointerTable,
    IsolateFieldId::CppHeapPointerTable,
    IsolateFieldId::TrustedCageBase,
    IsolateFieldId::TrustedPointerTable,
    IsolateFieldId::SharedTrustedPointerTable,
    IsolateFieldId::TrustedPointerPublishingScope,
    IsolateFieldId::JSDispatchTable,
    IsolateFieldId::ApiCallbackThunkArgument,
    IsolateFieldId::RegexpExecVectorArgument,
    IsolateFieldId::ContinuationPreservedEmbedderData,
    IsolateFieldId::CurrentMicrotaskQueue,
    IsolateFieldId::CurrentMicrotaskNativeContext,
    IsolateFieldId::RootsTable,
    IsolateFieldId::ExternalReferenceTable,
    IsolateFieldId::BuiltinEntryTable,
    IsolateFieldId::BuiltinTable,
    IsolateFieldId::ActiveStack,
    IsolateFieldId::ActiveSuspender,
    IsolateFieldId::DateCacheStamp,
    IsolateFieldId::IsDateCacheUsed,
    IsolateFieldId::DefaultLocaleMayRequireSpecialCaseMapping,
    IsolateFieldId::RawArgumentsPadding,
    IsolateFieldId::RawArguments,
    IsolateFieldId::StressDeoptCount,
    IsolateFieldId::CpuFeatures,
    IsolateFieldId::BuiltinDispatchTable,
    IsolateFieldId::IsolateAddress,
    IsolateFieldId::Context,
    IsolateFieldId::LastEnteredContext,
    IsolateFieldId::Exception,
    IsolateFieldId::TopmostScriptHavingContext,
    IsolateFieldId::CEntryFP,
    IsolateFieldId::Handler,
    IsolateFieldId::CFunction,
    IsolateFieldId::PendingHandlerContext,
    IsolateFieldId::PendingHandlerEntrypoint,
    IsolateFieldId::PendingHandlerConstantPool,
    IsolateFieldId::PendingHandlerFP,
    IsolateFieldId::PendingHandlerSP,
    IsolateFieldId::NumFramesAbovePendingHandler,
    IsolateFieldId::IsOnCentralStackFlag,
    IsolateFieldId::JSEntrySP,
    IsolateFieldId::HandleScopeLevel,
    IsolateFieldId::HandleScopeNext,
    IsolateFieldId::HandleScopeLimit,
    IsolateFieldId::EnteredContextCount,
    IsolateFieldId::EnteredContextCapacity,
    IsolateFieldId::EnteredContextData,
    IsolateFieldId::NewAllocationInfoStart,
    IsolateFieldId::NewAllocationInfoTop,
    IsolateFieldId::NewAllocationInfoLimit,
    IsolateFieldId::OldAllocationInfoStart,
    IsolateFieldId::OldAllocationInfoTop,
    IsolateFieldId::OldAllocationInfoLimit,
    IsolateFieldId::JsLimit,
    IsolateFieldId::RealJsLimit,
    IsolateFieldId::NoHeapWriteInterruptRequest,
  ];

  /// 字段名称字面量
  #[inline]
  pub const fn name(self) -> &'static str {
    match self {
      IsolateFieldId::CageBase => "CageBase",
      IsolateFieldId::StackGuard => "StackGuard",
      IsolateFieldId::IsMarkingFlag => "IsMarkingFlag",
      IsolateFieldId::IsMinorMarkingFlag => "IsMinorMarkingFlag",
      IsolateFieldId::IsSharedSpaceIsolateFlag => "IsSharedSpaceIsolateFlag",
      IsolateFieldId::UsesSharedHeapFlag => "UsesSharedHeapFlag",
      IsolateFieldId::ExecutionMode => "ExecutionMode",
      IsolateFieldId::StackIsIterable => "StackIsIterable",
      IsolateFieldId::ErrorMessageParam => "ErrorMessageParam",
      IsolateFieldId::HasLazyClosures => "HasLazyClosures",
      IsolateFieldId::TablesAlignmentPadding => "TablesAlignmentPadding",
      IsolateFieldId::RegExpStaticResultOffsetsVector => "RegExpStaticResultOffsetsVector",
      IsolateFieldId::BuiltinTier0EntryTable => "BuiltinTier0EntryTable",
      IsolateFieldId::BuiltinsTier0Table => "BuiltinsTier0Table",
      IsolateFieldId::NewAllocationInfo => "NewAllocationInfo",
      IsolateFieldId::OldAllocationInfo => "OldAllocationInfo",
      IsolateFieldId::LastYoungAllocation => "LastYoungAllocation",
      IsolateFieldId::FastCCallAlignmentPaddingOffset => "FastCCallAlignmentPaddingOffset",
      IsolateFieldId::FastCCallCallerPC => "FastCCallCallerPC",
      IsolateFieldId::FastCCallCallerFP => "FastCCallCallerFP",
      IsolateFieldId::FastApiCallTarget => "FastApiCallTarget",
      IsolateFieldId::LongTaskStatsCounter => "LongTaskStatsCounter",
      IsolateFieldId::ThreadLocalTop => "ThreadLocalTop",
      IsolateFieldId::HandleScopeData => "HandleScopeData",
      IsolateFieldId::HandleScopeImplementer => "HandleScopeImplementer",
      IsolateFieldId::EmbedderData => "EmbedderData",
      IsolateFieldId::ExternalPointerTable => "ExternalPointerTable",
      IsolateFieldId::SharedExternalPointerTable => "SharedExternalPointerTable",
      IsolateFieldId::CppHeapPointerTable => "CppHeapPointerTable",
      IsolateFieldId::TrustedCageBase => "TrustedCageBase",
      IsolateFieldId::TrustedPointerTable => "TrustedPointerTable",
      IsolateFieldId::SharedTrustedPointerTable => "SharedTrustedPointerTable",
      IsolateFieldId::TrustedPointerPublishingScope => "TrustedPointerPublishingScope",
      IsolateFieldId::JSDispatchTable => "JSDispatchTable",
      IsolateFieldId::ApiCallbackThunkArgument => "ApiCallbackThunkArgument",
      IsolateFieldId::RegexpExecVectorArgument => "RegexpExecVectorArgument",
      IsolateFieldId::ContinuationPreservedEmbedderData => "ContinuationPreservedEmbedderData",
      IsolateFieldId::CurrentMicrotaskQueue => "CurrentMicrotaskQueue",
      IsolateFieldId::CurrentMicrotaskNativeContext => "CurrentMicrotaskNativeContext",
      IsolateFieldId::RootsTable => "RootsTable",
      IsolateFieldId::ExternalReferenceTable => "ExternalReferenceTable",
      IsolateFieldId::BuiltinEntryTable => "BuiltinEntryTable",
      IsolateFieldId::BuiltinTable => "BuiltinTable",
      IsolateFieldId::ActiveStack => "ActiveStack",
      IsolateFieldId::ActiveSuspender => "ActiveSuspender",
      IsolateFieldId::DateCacheStamp => "DateCacheStamp",
      IsolateFieldId::IsDateCacheUsed => "IsDateCacheUsed",
      IsolateFieldId::DefaultLocaleMayRequireSpecialCaseMapping => {
        "DefaultLocaleMayRequireSpecialCaseMapping"
      }
      IsolateFieldId::RawArgumentsPadding => "RawArgumentsPadding",
      IsolateFieldId::RawArguments => "RawArguments",
      IsolateFieldId::StressDeoptCount => "StressDeoptCount",
      IsolateFieldId::CpuFeatures => "CpuFeatures",
      IsolateFieldId::BuiltinDispatchTable => "BuiltinDispatchTable",
      IsolateFieldId::IsolateAddress => "IsolateAddress",
      IsolateFieldId::Context => "Context",
      IsolateFieldId::LastEnteredContext => "LastEnteredContext",
      IsolateFieldId::Exception => "Exception",
      IsolateFieldId::TopmostScriptHavingContext => "TopmostScriptHavingContext",
      IsolateFieldId::CEntryFP => "CEntryFP",
      IsolateFieldId::Handler => "Handler",
      IsolateFieldId::CFunction => "CFunction",
      IsolateFieldId::PendingHandlerContext => "PendingHandlerContext",
      IsolateFieldId::PendingHandlerEntrypoint => "PendingHandlerEntrypoint",
      IsolateFieldId::PendingHandlerConstantPool => "PendingHandlerConstantPool",
      IsolateFieldId::PendingHandlerFP => "PendingHandlerFP",
      IsolateFieldId::PendingHandlerSP => "PendingHandlerSP",
      IsolateFieldId::NumFramesAbovePendingHandler => "NumFramesAbovePendingHandler",
      IsolateFieldId::IsOnCentralStackFlag => "IsOnCentralStackFlag",
      IsolateFieldId::JSEntrySP => "JSEntrySP",
      IsolateFieldId::HandleScopeLevel => "HandleScopeLevel",
      IsolateFieldId::HandleScopeNext => "HandleScopeNext",
      IsolateFieldId::HandleScopeLimit => "HandleScopeLimit",
      IsolateFieldId::EnteredContextCount => "EnteredContextCount",
      IsolateFieldId::EnteredContextCapacity => "EnteredContextCapacity",
      IsolateFieldId::EnteredContextData => "EnteredContextData",
      IsolateFieldId::NewAllocationInfoStart => "NewAllocationInfoStart",
      IsolateFieldId::NewAllocationInfoTop => "NewAllocationInfoTop",
      IsolateFieldId::NewAllocationInfoLimit => "NewAllocationInfoLimit",
      IsolateFieldId::OldAllocationInfoStart => "OldAllocationInfoStart",
      IsolateFieldId::OldAllocationInfoTop => "OldAllocationInfoTop",
      IsolateFieldId::OldAllocationInfoLimit => "OldAllocationInfoLimit",
      IsolateFieldId::JsLimit => "JsLimit",
      IsolateFieldId::RealJsLimit => "RealJsLimit",
      IsolateFieldId::NoHeapWriteInterruptRequest => "NoHeapWriteInterruptRequest",
    }
  }

  /// 转换为 u8 原始值
  #[inline]
  pub const fn as_u8(self) -> u8 {
    self as u8
  }
}

// ── 编译期静态断言 ──
const _: () = {
  assert!(IsolateFieldId::COUNT == 84);
  assert!(IsolateFieldId::ALL.len() == 84);
};
