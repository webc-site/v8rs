// 由 tool/src/objects/object-type.h.js 从 v8/src/objects/object-type.{h,cc} 生成; 修订就地改, 复跑覆盖
// ObjectType 检查标签枚举 (clang -E 全量展开, INTL+WASM 开, TEMPORAL/METAGEN/DRUMBRAKE 关)

/// 对象检查标签 (原版 enum class ObjectType, 顺序即基线序, 勿重排)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ObjectType {
  /// kObject
  Object = 0,
  /// kSmi
  Smi = 1,
  /// kTaggedIndex
  TaggedIndex = 2,
  /// kHeapObject
  HeapObject = 3,
  /// kHeapObjectReference
  HeapObjectReference = 4,
  /// kPrimitive
  Primitive = 5,
  /// kNumber
  Number = 6,
  /// kNumeric
  Numeric = 7,
  /// kAbstractCode
  AbstractCode = 8,
  /// kAccessorInfo
  AccessorInfo = 9,
  /// kAllocationSite
  AllocationSite = 10,
  /// kBigInt
  BigInt = 11,
  /// kBigIntBase
  BigIntBase = 12,
  /// kBoolean
  Boolean = 13,
  /// kCell
  Cell = 14,
  /// kCompilationCacheTable
  CompilationCacheTable = 15,
  /// kConsString
  ConsString = 16,
  /// kContext
  Context = 17,
  /// kContextCell
  ContextCell = 18,
  /// kCoverageInfo
  CoverageInfo = 19,
  /// kCppHeapExternalObject
  CppHeapExternalObject = 20,
  /// kCppGCManagedBase
  CppGCManagedBase = 21,
  /// kDataHandler
  DataHandler = 22,
  /// kDeoptimizationData
  DeoptimizationData = 23,
  /// kDependentCode
  DependentCode = 24,
  /// kDescriptorArray
  DescriptorArray = 25,
  /// kDictionaryTemplateInfo
  DictionaryTemplateInfo = 26,
  /// kDoubleStringCache
  DoubleStringCache = 27,
  /// kEmbedderDataArray
  EmbedderDataArray = 28,
  /// kEphemeronHashTable
  EphemeronHashTable = 29,
  /// kExternalOneByteString
  ExternalOneByteString = 30,
  /// kExternalString
  ExternalString = 31,
  /// kExternalTwoByteString
  ExternalTwoByteString = 32,
  /// kFeedbackCell
  FeedbackCell = 33,
  /// kFeedbackMetadata
  FeedbackMetadata = 34,
  /// kFeedbackVector
  FeedbackVector = 35,
  /// kFunctionTemplateInfo
  FunctionTemplateInfo = 36,
  /// kFixedArrayBase
  FixedArrayBase = 37,
  /// kFixedArrayExact
  FixedArrayExact = 38,
  /// kForeign
  Foreign = 39,
  /// kFreeSpace
  FreeSpace = 40,
  /// kGcSafeCode
  GcSafeCode = 41,
  /// kGlobalDictionary
  GlobalDictionary = 42,
  /// kHashSeedWrapper
  HashSeedWrapper = 43,
  /// kHeapNumber
  HeapNumber = 44,
  /// kUninitializedHeapNumber
  UninitializedHeapNumber = 45,
  /// kInterceptorInfo
  InterceptorInfo = 46,
  /// kInternalizedString
  InternalizedString = 47,
  /// kJSArgumentsObject
  JSArgumentsObject = 48,
  /// kJSArray
  JSArray = 49,
  /// kJSArrayBuffer
  JSArrayBuffer = 50,
  /// kJSArrayBufferView
  JSArrayBufferView = 51,
  /// kJSArrayIterator
  JSArrayIterator = 52,
  /// kJSAsyncFromSyncIterator
  JSAsyncFromSyncIterator = 53,
  /// kJSAsyncFunctionObject
  JSAsyncFunctionObject = 54,
  /// kJSAsyncGeneratorObject
  JSAsyncGeneratorObject = 55,
  /// kJSAtomicsCondition
  JSAtomicsCondition = 56,
  /// kJSAtomicsMutex
  JSAtomicsMutex = 57,
  /// kJSBoundFunction
  JSBoundFunction = 58,
  /// kJSCollection
  JSCollection = 59,
  /// kJSCollectionIterator
  JSCollectionIterator = 60,
  /// kJSCustomElementsObject
  JSCustomElementsObject = 61,
  /// kJSDataView
  JSDataView = 62,
  /// kJSDataViewOrRabGsabDataView
  JSDataViewOrRabGsabDataView = 63,
  /// kJSDate
  JSDate = 64,
  /// kJSDisposableStackBase
  JSDisposableStackBase = 65,
  /// kJSSyncDisposableStack
  JSSyncDisposableStack = 66,
  /// kJSAsyncDisposableStack
  JSAsyncDisposableStack = 67,
  /// kJSExternalObject
  JSExternalObject = 68,
  /// kJSFinalizationRegistry
  JSFinalizationRegistry = 69,
  /// kJSFunction
  JSFunction = 70,
  /// kJSFunctionWithoutPrototype
  JSFunctionWithoutPrototype = 71,
  /// kJSFunctionWithPrototype
  JSFunctionWithPrototype = 72,
  /// kJSFunctionOrBoundFunctionOrWrappedFunction
  JSFunctionOrBoundFunctionOrWrappedFunction = 73,
  /// kJSGeneratorObject
  JSGeneratorObject = 74,
  /// kJSGlobalObject
  JSGlobalObject = 75,
  /// kJSGlobalProxy
  JSGlobalProxy = 76,
  /// kJSIteratorHelper
  JSIteratorHelper = 77,
  /// kJSIteratorHelperSimple
  JSIteratorHelperSimple = 78,
  /// kJSIteratorFilterHelper
  JSIteratorFilterHelper = 79,
  /// kJSIteratorMapHelper
  JSIteratorMapHelper = 80,
  /// kJSIteratorTakeHelper
  JSIteratorTakeHelper = 81,
  /// kJSIteratorDropHelper
  JSIteratorDropHelper = 82,
  /// kJSIteratorFlatMapHelper
  JSIteratorFlatMapHelper = 83,
  /// kJSIteratorConcatHelper
  JSIteratorConcatHelper = 84,
  /// kJSIteratorZipHelper
  JSIteratorZipHelper = 85,
  /// kJSIteratorZipKeyedHelper
  JSIteratorZipKeyedHelper = 86,
  /// kJSMap
  JSMap = 87,
  /// kJSMapIterator
  JSMapIterator = 88,
  /// kJSMessageObject
  JSMessageObject = 89,
  /// kJSModuleNamespace
  JSModuleNamespace = 90,
  /// kJSDeferredModuleNamespace
  JSDeferredModuleNamespace = 91,
  /// kJSObject
  JSObject = 92,
  /// kJSAPIObjectWithEmbedderSlots
  JSAPIObjectWithEmbedderSlots = 93,
  /// kJSObjectWithEmbedderSlots
  JSObjectWithEmbedderSlots = 94,
  /// kJSPrimitiveWrapper
  JSPrimitiveWrapper = 95,
  /// kJSPromise
  JSPromise = 96,
  /// kJSProxy
  JSProxy = 97,
  /// kJSRabGsabDataView
  JSRabGsabDataView = 98,
  /// kJSRawJson
  JSRawJson = 99,
  /// kJSReceiver
  JSReceiver = 100,
  /// kJSRegExp
  JSRegExp = 101,
  /// kJSRegExpStringIterator
  JSRegExpStringIterator = 102,
  /// kJSSet
  JSSet = 103,
  /// kJSSetIterator
  JSSetIterator = 104,
  /// kJSShadowRealm
  JSShadowRealm = 105,
  /// kJSSharedArray
  JSSharedArray = 106,
  /// kJSSharedStruct
  JSSharedStruct = 107,
  /// kJSSpecialObject
  JSSpecialObject = 108,
  /// kJSStringIterator
  JSStringIterator = 109,
  /// kJSSynchronizationPrimitive
  JSSynchronizationPrimitive = 110,
  /// kJSDetachedTypedArray
  JSDetachedTypedArray = 111,
  /// kJSTypedArray
  JSTypedArray = 112,
  /// kJSValidIteratorWrapper
  JSValidIteratorWrapper = 113,
  /// kJSWeakCollection
  JSWeakCollection = 114,
  /// kJSWeakRef
  JSWeakRef = 115,
  /// kJSWeakMap
  JSWeakMap = 116,
  /// kJSWeakSet
  JSWeakSet = 117,
  /// kJSWrappedFunction
  JSWrappedFunction = 118,
  /// kLoadHandler
  LoadHandler = 119,
  /// kMap
  Map = 120,
  /// kMegaDomHandler
  MegaDomHandler = 121,
  /// kModule
  Module = 122,
  /// kMicrotask
  Microtask = 123,
  /// kName
  Name = 124,
  /// kNameDictionary
  NameDictionary = 125,
  /// kNameToIndexHashTable
  NameToIndexHashTable = 126,
  /// kNativeContext
  NativeContext = 127,
  /// kNormalizedMapCache
  NormalizedMapCache = 128,
  /// kNumberDictionary
  NumberDictionary = 129,
  /// kObjectHashSet
  ObjectHashSet = 130,
  /// kObjectHashTable
  ObjectHashTable = 131,
  /// kObjectTemplateInfo
  ObjectTemplateInfo = 132,
  /// kObjectTwoHashTable
  ObjectTwoHashTable = 133,
  /// kOddball
  Oddball = 134,
  /// kOnHeapBasicBlockProfilerData
  OnHeapBasicBlockProfilerData = 135,
  /// kHole
  Hole = 136,
  /// kOrderedHashMap
  OrderedHashMap = 137,
  /// kOrderedHashSet
  OrderedHashSet = 138,
  /// kOrderedNameDictionary
  OrderedNameDictionary = 139,
  /// kPreparseData
  PreparseData = 140,
  /// kPrimitiveHeapObject
  PrimitiveHeapObject = 141,
  /// kPromiseReactionJobTask
  PromiseReactionJobTask = 142,
  /// kPropertyArray
  PropertyArray = 143,
  /// kPropertyCell
  PropertyCell = 144,
  /// kScopeInfo
  ScopeInfo = 145,
  /// kSeqOneByteString
  SeqOneByteString = 146,
  /// kSeqString
  SeqString = 147,
  /// kSeqTwoByteString
  SeqTwoByteString = 148,
  /// kSharedFunctionInfo
  SharedFunctionInfo = 149,
  /// kSimpleNameDictionary
  SimpleNameDictionary = 150,
  /// kSimpleNumberDictionary
  SimpleNumberDictionary = 151,
  /// kSlicedString
  SlicedString = 152,
  /// kSmallOrderedHashMap
  SmallOrderedHashMap = 153,
  /// kSmallOrderedHashSet
  SmallOrderedHashSet = 154,
  /// kSmallOrderedNameDictionary
  SmallOrderedNameDictionary = 155,
  /// kSortState
  SortState = 156,
  /// kSourceTextModule
  SourceTextModule = 157,
  /// kSourceTextModuleInfo
  SourceTextModuleInfo = 158,
  /// kStoreHandler
  StoreHandler = 159,
  /// kString
  String = 160,
  /// kStringSet
  StringSet = 161,
  /// kRegisteredSymbolTable
  RegisteredSymbolTable = 162,
  /// kStruct
  Struct = 163,
  /// kSwissNameDictionary
  SwissNameDictionary = 164,
  /// kSymbol
  Symbol = 165,
  /// kSyntheticModule
  SyntheticModule = 166,
  /// kTemplateInfo
  TemplateInfo = 167,
  /// kTemplateInfoWithProperties
  TemplateInfoWithProperties = 168,
  /// kTemplateLiteralObject
  TemplateLiteralObject = 169,
  /// kThinString
  ThinString = 170,
  /// kTransitionArray
  TransitionArray = 171,
  /// kTurbofanBitsetType
  TurbofanBitsetType = 172,
  /// kTurbofanHeapConstantType
  TurbofanHeapConstantType = 173,
  /// kTurbofanOtherNumberConstantType
  TurbofanOtherNumberConstantType = 174,
  /// kTurbofanRangeType
  TurbofanRangeType = 175,
  /// kTurbofanType
  TurbofanType = 176,
  /// kTurbofanUnionType
  TurbofanUnionType = 177,
  /// kTurboshaftFloat64RangeType
  TurboshaftFloat64RangeType = 178,
  /// kTurboshaftFloat64SetType
  TurboshaftFloat64SetType = 179,
  /// kTurboshaftFloat64Type
  TurboshaftFloat64Type = 180,
  /// kTurboshaftType
  TurboshaftType = 181,
  /// kTurboshaftWord32RangeType
  TurboshaftWord32RangeType = 182,
  /// kTurboshaftWord32SetType
  TurboshaftWord32SetType = 183,
  /// kTurboshaftWord32Type
  TurboshaftWord32Type = 184,
  /// kTurboshaftWord64RangeType
  TurboshaftWord64RangeType = 185,
  /// kTurboshaftWord64SetType
  TurboshaftWord64SetType = 186,
  /// kTurboshaftWord64Type
  TurboshaftWord64Type = 187,
  /// kWasmArray
  WasmArray = 188,
  /// kWasmContinuationObject
  WasmContinuationObject = 189,
  /// kWasmCustomMap
  WasmCustomMap = 190,
  /// kWasmCustomMapWrapper
  WasmCustomMapWrapper = 191,
  /// kWasmExceptionPackage
  WasmExceptionPackage = 192,
  /// kWasmFastApiCallData
  WasmFastApiCallData = 193,
  /// kWasmFuncRef
  WasmFuncRef = 194,
  /// kWasmGlobalObject
  WasmGlobalObject = 195,
  /// kWasmInstanceObject
  WasmInstanceObject = 196,
  /// kWasmMemoryObject
  WasmMemoryObject = 197,
  /// kWasmModuleObject
  WasmModuleObject = 198,
  /// kWasmNull
  WasmNull = 199,
  /// kWasmObject
  WasmObject = 200,
  /// kWasmResumeData
  WasmResumeData = 201,
  /// kWasmStackObject
  WasmStackObject = 202,
  /// kWasmStringViewIter
  WasmStringViewIter = 203,
  /// kWasmStruct
  WasmStruct = 204,
  /// kWasmSuspendingObject
  WasmSuspendingObject = 205,
  /// kWasmTableObject
  WasmTableObject = 206,
  /// kWasmTagObject
  WasmTagObject = 207,
  /// kWasmTypeInfo
  WasmTypeInfo = 208,
  /// kWasmValueObject
  WasmValueObject = 209,
  /// kWeakCell
  WeakCell = 210,
  /// kArrayList
  ArrayList = 211,
  /// kByteArray
  ByteArray = 212,
  /// kClosureFeedbackCellArray
  ClosureFeedbackCellArray = 213,
  /// kFixedArray
  FixedArray = 214,
  /// kFixedDoubleArray
  FixedDoubleArray = 215,
  /// kObjectBoilerplateDescription
  ObjectBoilerplateDescription = 216,
  /// kRegExpMatchInfo
  RegExpMatchInfo = 217,
  /// kScriptContextTable
  ScriptContextTable = 218,
  /// kSloppyArgumentsElements
  SloppyArgumentsElements = 219,
  /// kWeakArrayList
  WeakArrayList = 220,
  /// kWeakFixedArray
  WeakFixedArray = 221,
  /// kWeakHomomorphicFixedArray
  WeakHomomorphicFixedArray = 222,
  /// kJSV8BreakIterator
  JSV8BreakIterator = 223,
  /// kJSCollator
  JSCollator = 224,
  /// kJSDateTimeFormat
  JSDateTimeFormat = 225,
  /// kJSDisplayNames
  JSDisplayNames = 226,
  /// kJSDurationFormat
  JSDurationFormat = 227,
  /// kJSListFormat
  JSListFormat = 228,
  /// kJSLocale
  JSLocale = 229,
  /// kJSNumberFormat
  JSNumberFormat = 230,
  /// kJSPluralRules
  JSPluralRules = 231,
  /// kJSRelativeTimeFormat
  JSRelativeTimeFormat = 232,
  /// kJSSegmentDataObject
  JSSegmentDataObject = 233,
  /// kJSSegmentDataObjectWithIsWordLike
  JSSegmentDataObjectWithIsWordLike = 234,
  /// kJSSegmentIterator
  JSSegmentIterator = 235,
  /// kJSSegmenter
  JSSegmenter = 236,
  /// kJSSegments
  JSSegments = 237,
  /// kAccessCheckNeeded
  AccessCheckNeeded = 238,
  /// kAlwaysSharedSpaceJSObject
  AlwaysSharedSpaceJSObject = 239,
  /// kBigIntWrapper
  BigIntWrapper = 240,
  /// kBooleanWrapper
  BooleanWrapper = 241,
  /// kCallable
  Callable = 242,
  /// kConstructor
  Constructor = 243,
  /// kFiller
  Filler = 244,
  /// kHandlerTable
  HandlerTable = 245,
  /// kJSContextExtensionObject
  JSContextExtensionObject = 246,
  /// kJSError
  JSError = 247,
  /// kJSInterceptorMap
  JSInterceptorMap = 248,
  /// kMapCache
  MapCache = 249,
  /// kExtendedMap
  ExtendedMap = 250,
  /// kMetaMap
  MetaMap = 251,
  /// kNumberWrapper
  NumberWrapper = 252,
  /// kOSROptimizedCodeCache
  OSROptimizedCodeCache = 253,
  /// kScriptWrapper
  ScriptWrapper = 254,
  /// kSmiStringCache
  SmiStringCache = 255,
  /// kStringWrapper
  StringWrapper = 256,
  /// kSymbolWrapper
  SymbolWrapper = 257,
  /// kUniqueName
  UniqueName = 258,
  /// kUndetectable
  Undetectable = 259,
  /// kTrustedObject
  TrustedObject = 260,
  /// kExposedTrustedObject
  ExposedTrustedObject = 261,
  /// kTrustedFixedArrayBase
  TrustedFixedArrayBase = 262,
  /// kUncompiledData
  UncompiledData = 263,
  /// kWasmFunctionData
  WasmFunctionData = 264,
  /// kBytecodeArray
  BytecodeArray = 265,
  /// kCode
  Code = 266,
  /// kDebugInfo
  DebugInfo = 267,
  /// kInstructionStream
  InstructionStream = 268,
  /// kInterpreterData
  InterpreterData = 269,
  /// kUncompiledDataWithPreparseData
  UncompiledDataWithPreparseData = 270,
  /// kUncompiledDataWithoutPreparseData
  UncompiledDataWithoutPreparseData = 271,
  /// kUncompiledDataWithPreparseDataAndJob
  UncompiledDataWithPreparseDataAndJob = 272,
  /// kUncompiledDataWithoutPreparseDataWithJob
  UncompiledDataWithoutPreparseDataWithJob = 273,
  /// kSharedFunctionInfoWrapper
  SharedFunctionInfoWrapper = 274,
  /// kProtectedFixedArray
  ProtectedFixedArray = 275,
  /// kProtectedWeakFixedArray
  ProtectedWeakFixedArray = 276,
  /// kTrustedByteArray
  TrustedByteArray = 277,
  /// kTrustedFixedArray
  TrustedFixedArray = 278,
  /// kTrustedForeign
  TrustedForeign = 279,
  /// kTrustedWeakFixedArray
  TrustedWeakFixedArray = 280,
  /// kAtomRegExpData
  AtomRegExpData = 281,
  /// kIrRegExpData
  IrRegExpData = 282,
  /// kRegExpData
  RegExpData = 283,
  /// kWasmImportData
  WasmImportData = 284,
  /// kWasmCapiFunctionData
  WasmCapiFunctionData = 285,
  /// kWasmDispatchTable
  WasmDispatchTable = 286,
  /// kWasmDispatchTableForImports
  WasmDispatchTableForImports = 287,
  /// kWasmExportedFunctionData
  WasmExportedFunctionData = 288,
  /// kWasmInternalFunction
  WasmInternalFunction = 289,
  /// kWasmTrustedInstanceData
  WasmTrustedInstanceData = 290,
  /// kWasmSuspenderObject
  WasmSuspenderObject = 291,
  /// kHashTable
  HashTable = 292,
  /// kAwaitContext
  AwaitContext = 293,
  /// kBlockContext
  BlockContext = 294,
  /// kCallableApiObject
  CallableApiObject = 295,
  /// kCallableJSFunction
  CallableJSFunction = 296,
  /// kCallableJSProxy
  CallableJSProxy = 297,
  /// kCatchContext
  CatchContext = 298,
  /// kCppHeapPointerWrapperObject
  CppHeapPointerWrapperObject = 299,
  /// kDebugEvaluateContext
  DebugEvaluateContext = 300,
  /// kEvalContext
  EvalContext = 301,
  /// kFreeSpaceOrFiller
  FreeSpaceOrFiller = 302,
  /// kFunctionContext
  FunctionContext = 303,
  /// kJSApiObject
  JSApiObject = 304,
  /// kJSApiWrapperObject
  JSApiWrapperObject = 305,
  /// kJSClassConstructor
  JSClassConstructor = 306,
  /// kJSLastDummyApiObject
  JSLastDummyApiObject = 307,
  /// kJSPromiseConstructor
  JSPromiseConstructor = 308,
  /// kJSArrayConstructor
  JSArrayConstructor = 309,
  /// kJSRegExpConstructor
  JSRegExpConstructor = 310,
  /// kJSMapKeyIterator
  JSMapKeyIterator = 311,
  /// kJSMapKeyValueIterator
  JSMapKeyValueIterator = 312,
  /// kJSMapValueIterator
  JSMapValueIterator = 313,
  /// kJSSetKeyValueIterator
  JSSetKeyValueIterator = 314,
  /// kJSSetValueIterator
  JSSetValueIterator = 315,
  /// kJSSpecialApiObject
  JSSpecialApiObject = 316,
  /// kMaybeReadOnlyJSObject
  MaybeReadOnlyJSObject = 317,
  /// kModuleContext
  ModuleContext = 318,
  /// kNonNullForeign
  NonNullForeign = 319,
  /// kScriptContext
  ScriptContext = 320,
  /// kWithContext
  WithContext = 321,
  /// kJSInternalPrototypeBase
  JSInternalPrototypeBase = 322,
  /// kJSObjectPrototype
  JSObjectPrototype = 323,
  /// kJSRegExpPrototype
  JSRegExpPrototype = 324,
  /// kJSPromisePrototype
  JSPromisePrototype = 325,
  /// kJSSetPrototype
  JSSetPrototype = 326,
  /// kJSIteratorPrototype
  JSIteratorPrototype = 327,
  /// kJSArrayIteratorPrototype
  JSArrayIteratorPrototype = 328,
  /// kJSMapIteratorPrototype
  JSMapIteratorPrototype = 329,
  /// kJSTypedArrayPrototype
  JSTypedArrayPrototype = 330,
  /// kJSSetIteratorPrototype
  JSSetIteratorPrototype = 331,
  /// kJSStringIteratorPrototype
  JSStringIteratorPrototype = 332,
  /// kTypedArrayConstructor
  TypedArrayConstructor = 333,
  /// kUint8TypedArrayConstructor
  Uint8TypedArrayConstructor = 334,
  /// kInt8TypedArrayConstructor
  Int8TypedArrayConstructor = 335,
  /// kUint16TypedArrayConstructor
  Uint16TypedArrayConstructor = 336,
  /// kInt16TypedArrayConstructor
  Int16TypedArrayConstructor = 337,
  /// kUint32TypedArrayConstructor
  Uint32TypedArrayConstructor = 338,
  /// kInt32TypedArrayConstructor
  Int32TypedArrayConstructor = 339,
  /// kFloat16TypedArrayConstructor
  Float16TypedArrayConstructor = 340,
  /// kFloat32TypedArrayConstructor
  Float32TypedArrayConstructor = 341,
  /// kFloat64TypedArrayConstructor
  Float64TypedArrayConstructor = 342,
  /// kUint8ClampedTypedArrayConstructor
  Uint8ClampedTypedArrayConstructor = 343,
  /// kBiguint64TypedArrayConstructor
  Biguint64TypedArrayConstructor = 344,
  /// kBigint64TypedArrayConstructor
  Bigint64TypedArrayConstructor = 345,
  /// kPromiseFulfillReactionJobTask
  PromiseFulfillReactionJobTask = 346,
  /// kPromiseRejectReactionJobTask
  PromiseRejectReactionJobTask = 347,
  /// kAsyncResumeTask
  AsyncResumeTask = 348,
  /// kCallableTask
  CallableTask = 349,
  /// kCallbackTask
  CallbackTask = 350,
  /// kPromiseResolveThenableJobTask
  PromiseResolveThenableJobTask = 351,
  /// kAccessCheckInfo
  AccessCheckInfo = 352,
  /// kAccessorPair
  AccessorPair = 353,
  /// kAliasedArgumentsEntry
  AliasedArgumentsEntry = 354,
  /// kAllocationMemento
  AllocationMemento = 355,
  /// kArrayBoilerplateDescription
  ArrayBoilerplateDescription = 356,
  /// kAsyncGeneratorRequest
  AsyncGeneratorRequest = 357,
  /// kBreakPoint
  BreakPoint = 358,
  /// kBreakPointInfo
  BreakPointInfo = 359,
  /// kBytecodeWrapper
  BytecodeWrapper = 360,
  /// kCallSiteInfo
  CallSiteInfo = 361,
  /// kClassBoilerplate
  ClassBoilerplate = 362,
  /// kClassPositions
  ClassPositions = 363,
  /// kCodeWrapper
  CodeWrapper = 364,
  /// kDebugScriptScopeInfo
  DebugScriptScopeInfo = 365,
  /// kEnumCache
  EnumCache = 366,
  /// kErrorStackData
  ErrorStackData = 367,
  /// kForInEnumeratorHolder
  ForInEnumeratorHolder = 368,
  /// kFunctionTemplateRareData
  FunctionTemplateRareData = 369,
  /// kModuleRequest
  ModuleRequest = 370,
  /// kPromiseCapability
  PromiseCapability = 371,
  /// kPromiseReaction
  PromiseReaction = 372,
  /// kPropertyDescriptorObject
  PropertyDescriptorObject = 373,
  /// kPrototypeInfo
  PrototypeInfo = 374,
  /// kPrototypeSharedClosureInfo
  PrototypeSharedClosureInfo = 375,
  /// kRegExpBoilerplateDescription
  RegExpBoilerplateDescription = 376,
  /// kRegExpDataWrapper
  RegExpDataWrapper = 377,
  /// kScript
  Script = 378,
  /// kScriptOrModule
  ScriptOrModule = 379,
  /// kSourceTextModuleInfoEntry
  SourceTextModuleInfoEntry = 380,
  /// kStackFrameInfo
  StackFrameInfo = 381,
  /// kStackTraceInfo
  StackTraceInfo = 382,
  /// kTemplateObjectDescription
  TemplateObjectDescription = 383,
  /// kTuple2
  Tuple2 = 384,
  /// kWasmExceptionTag
  WasmExceptionTag = 385,
}

impl ObjectType {
  /// 变体总数 (对数基准: clang -E 展开项)
  pub const COUNT: usize = 386;

  /// 原版 TYPE_CASE 期望名 (#Name 字符串化)
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::Object => "Object",
      Self::Smi => "Smi",
      Self::TaggedIndex => "TaggedIndex",
      Self::HeapObject => "HeapObject",
      Self::HeapObjectReference => "HeapObjectReference",
      Self::Primitive => "Primitive",
      Self::Number => "Number",
      Self::Numeric => "Numeric",
      Self::AbstractCode => "AbstractCode",
      Self::AccessorInfo => "AccessorInfo",
      Self::AllocationSite => "AllocationSite",
      Self::BigInt => "BigInt",
      Self::BigIntBase => "BigIntBase",
      Self::Boolean => "Boolean",
      Self::Cell => "Cell",
      Self::CompilationCacheTable => "CompilationCacheTable",
      Self::ConsString => "ConsString",
      Self::Context => "Context",
      Self::ContextCell => "ContextCell",
      Self::CoverageInfo => "CoverageInfo",
      Self::CppHeapExternalObject => "CppHeapExternalObject",
      Self::CppGCManagedBase => "CppGCManagedBase",
      Self::DataHandler => "DataHandler",
      Self::DeoptimizationData => "DeoptimizationData",
      Self::DependentCode => "DependentCode",
      Self::DescriptorArray => "DescriptorArray",
      Self::DictionaryTemplateInfo => "DictionaryTemplateInfo",
      Self::DoubleStringCache => "DoubleStringCache",
      Self::EmbedderDataArray => "EmbedderDataArray",
      Self::EphemeronHashTable => "EphemeronHashTable",
      Self::ExternalOneByteString => "ExternalOneByteString",
      Self::ExternalString => "ExternalString",
      Self::ExternalTwoByteString => "ExternalTwoByteString",
      Self::FeedbackCell => "FeedbackCell",
      Self::FeedbackMetadata => "FeedbackMetadata",
      Self::FeedbackVector => "FeedbackVector",
      Self::FunctionTemplateInfo => "FunctionTemplateInfo",
      Self::FixedArrayBase => "FixedArrayBase",
      Self::FixedArrayExact => "FixedArrayExact",
      Self::Foreign => "Foreign",
      Self::FreeSpace => "FreeSpace",
      Self::GcSafeCode => "GcSafeCode",
      Self::GlobalDictionary => "GlobalDictionary",
      Self::HashSeedWrapper => "HashSeedWrapper",
      Self::HeapNumber => "HeapNumber",
      Self::UninitializedHeapNumber => "UninitializedHeapNumber",
      Self::InterceptorInfo => "InterceptorInfo",
      Self::InternalizedString => "InternalizedString",
      Self::JSArgumentsObject => "JSArgumentsObject",
      Self::JSArray => "JSArray",
      Self::JSArrayBuffer => "JSArrayBuffer",
      Self::JSArrayBufferView => "JSArrayBufferView",
      Self::JSArrayIterator => "JSArrayIterator",
      Self::JSAsyncFromSyncIterator => "JSAsyncFromSyncIterator",
      Self::JSAsyncFunctionObject => "JSAsyncFunctionObject",
      Self::JSAsyncGeneratorObject => "JSAsyncGeneratorObject",
      Self::JSAtomicsCondition => "JSAtomicsCondition",
      Self::JSAtomicsMutex => "JSAtomicsMutex",
      Self::JSBoundFunction => "JSBoundFunction",
      Self::JSCollection => "JSCollection",
      Self::JSCollectionIterator => "JSCollectionIterator",
      Self::JSCustomElementsObject => "JSCustomElementsObject",
      Self::JSDataView => "JSDataView",
      Self::JSDataViewOrRabGsabDataView => "JSDataViewOrRabGsabDataView",
      Self::JSDate => "JSDate",
      Self::JSDisposableStackBase => "JSDisposableStackBase",
      Self::JSSyncDisposableStack => "JSSyncDisposableStack",
      Self::JSAsyncDisposableStack => "JSAsyncDisposableStack",
      Self::JSExternalObject => "JSExternalObject",
      Self::JSFinalizationRegistry => "JSFinalizationRegistry",
      Self::JSFunction => "JSFunction",
      Self::JSFunctionWithoutPrototype => "JSFunctionWithoutPrototype",
      Self::JSFunctionWithPrototype => "JSFunctionWithPrototype",
      Self::JSFunctionOrBoundFunctionOrWrappedFunction => {
        "JSFunctionOrBoundFunctionOrWrappedFunction"
      }
      Self::JSGeneratorObject => "JSGeneratorObject",
      Self::JSGlobalObject => "JSGlobalObject",
      Self::JSGlobalProxy => "JSGlobalProxy",
      Self::JSIteratorHelper => "JSIteratorHelper",
      Self::JSIteratorHelperSimple => "JSIteratorHelperSimple",
      Self::JSIteratorFilterHelper => "JSIteratorFilterHelper",
      Self::JSIteratorMapHelper => "JSIteratorMapHelper",
      Self::JSIteratorTakeHelper => "JSIteratorTakeHelper",
      Self::JSIteratorDropHelper => "JSIteratorDropHelper",
      Self::JSIteratorFlatMapHelper => "JSIteratorFlatMapHelper",
      Self::JSIteratorConcatHelper => "JSIteratorConcatHelper",
      Self::JSIteratorZipHelper => "JSIteratorZipHelper",
      Self::JSIteratorZipKeyedHelper => "JSIteratorZipKeyedHelper",
      Self::JSMap => "JSMap",
      Self::JSMapIterator => "JSMapIterator",
      Self::JSMessageObject => "JSMessageObject",
      Self::JSModuleNamespace => "JSModuleNamespace",
      Self::JSDeferredModuleNamespace => "JSDeferredModuleNamespace",
      Self::JSObject => "JSObject",
      Self::JSAPIObjectWithEmbedderSlots => "JSAPIObjectWithEmbedderSlots",
      Self::JSObjectWithEmbedderSlots => "JSObjectWithEmbedderSlots",
      Self::JSPrimitiveWrapper => "JSPrimitiveWrapper",
      Self::JSPromise => "JSPromise",
      Self::JSProxy => "JSProxy",
      Self::JSRabGsabDataView => "JSRabGsabDataView",
      Self::JSRawJson => "JSRawJson",
      Self::JSReceiver => "JSReceiver",
      Self::JSRegExp => "JSRegExp",
      Self::JSRegExpStringIterator => "JSRegExpStringIterator",
      Self::JSSet => "JSSet",
      Self::JSSetIterator => "JSSetIterator",
      Self::JSShadowRealm => "JSShadowRealm",
      Self::JSSharedArray => "JSSharedArray",
      Self::JSSharedStruct => "JSSharedStruct",
      Self::JSSpecialObject => "JSSpecialObject",
      Self::JSStringIterator => "JSStringIterator",
      Self::JSSynchronizationPrimitive => "JSSynchronizationPrimitive",
      Self::JSDetachedTypedArray => "JSDetachedTypedArray",
      Self::JSTypedArray => "JSTypedArray",
      Self::JSValidIteratorWrapper => "JSValidIteratorWrapper",
      Self::JSWeakCollection => "JSWeakCollection",
      Self::JSWeakRef => "JSWeakRef",
      Self::JSWeakMap => "JSWeakMap",
      Self::JSWeakSet => "JSWeakSet",
      Self::JSWrappedFunction => "JSWrappedFunction",
      Self::LoadHandler => "LoadHandler",
      Self::Map => "Map",
      Self::MegaDomHandler => "MegaDomHandler",
      Self::Module => "Module",
      Self::Microtask => "Microtask",
      Self::Name => "Name",
      Self::NameDictionary => "NameDictionary",
      Self::NameToIndexHashTable => "NameToIndexHashTable",
      Self::NativeContext => "NativeContext",
      Self::NormalizedMapCache => "NormalizedMapCache",
      Self::NumberDictionary => "NumberDictionary",
      Self::ObjectHashSet => "ObjectHashSet",
      Self::ObjectHashTable => "ObjectHashTable",
      Self::ObjectTemplateInfo => "ObjectTemplateInfo",
      Self::ObjectTwoHashTable => "ObjectTwoHashTable",
      Self::Oddball => "Oddball",
      Self::OnHeapBasicBlockProfilerData => "OnHeapBasicBlockProfilerData",
      Self::Hole => "Hole",
      Self::OrderedHashMap => "OrderedHashMap",
      Self::OrderedHashSet => "OrderedHashSet",
      Self::OrderedNameDictionary => "OrderedNameDictionary",
      Self::PreparseData => "PreparseData",
      Self::PrimitiveHeapObject => "PrimitiveHeapObject",
      Self::PromiseReactionJobTask => "PromiseReactionJobTask",
      Self::PropertyArray => "PropertyArray",
      Self::PropertyCell => "PropertyCell",
      Self::ScopeInfo => "ScopeInfo",
      Self::SeqOneByteString => "SeqOneByteString",
      Self::SeqString => "SeqString",
      Self::SeqTwoByteString => "SeqTwoByteString",
      Self::SharedFunctionInfo => "SharedFunctionInfo",
      Self::SimpleNameDictionary => "SimpleNameDictionary",
      Self::SimpleNumberDictionary => "SimpleNumberDictionary",
      Self::SlicedString => "SlicedString",
      Self::SmallOrderedHashMap => "SmallOrderedHashMap",
      Self::SmallOrderedHashSet => "SmallOrderedHashSet",
      Self::SmallOrderedNameDictionary => "SmallOrderedNameDictionary",
      Self::SortState => "SortState",
      Self::SourceTextModule => "SourceTextModule",
      Self::SourceTextModuleInfo => "SourceTextModuleInfo",
      Self::StoreHandler => "StoreHandler",
      Self::String => "String",
      Self::StringSet => "StringSet",
      Self::RegisteredSymbolTable => "RegisteredSymbolTable",
      Self::Struct => "Struct",
      Self::SwissNameDictionary => "SwissNameDictionary",
      Self::Symbol => "Symbol",
      Self::SyntheticModule => "SyntheticModule",
      Self::TemplateInfo => "TemplateInfo",
      Self::TemplateInfoWithProperties => "TemplateInfoWithProperties",
      Self::TemplateLiteralObject => "TemplateLiteralObject",
      Self::ThinString => "ThinString",
      Self::TransitionArray => "TransitionArray",
      Self::TurbofanBitsetType => "TurbofanBitsetType",
      Self::TurbofanHeapConstantType => "TurbofanHeapConstantType",
      Self::TurbofanOtherNumberConstantType => "TurbofanOtherNumberConstantType",
      Self::TurbofanRangeType => "TurbofanRangeType",
      Self::TurbofanType => "TurbofanType",
      Self::TurbofanUnionType => "TurbofanUnionType",
      Self::TurboshaftFloat64RangeType => "TurboshaftFloat64RangeType",
      Self::TurboshaftFloat64SetType => "TurboshaftFloat64SetType",
      Self::TurboshaftFloat64Type => "TurboshaftFloat64Type",
      Self::TurboshaftType => "TurboshaftType",
      Self::TurboshaftWord32RangeType => "TurboshaftWord32RangeType",
      Self::TurboshaftWord32SetType => "TurboshaftWord32SetType",
      Self::TurboshaftWord32Type => "TurboshaftWord32Type",
      Self::TurboshaftWord64RangeType => "TurboshaftWord64RangeType",
      Self::TurboshaftWord64SetType => "TurboshaftWord64SetType",
      Self::TurboshaftWord64Type => "TurboshaftWord64Type",
      Self::WasmArray => "WasmArray",
      Self::WasmContinuationObject => "WasmContinuationObject",
      Self::WasmCustomMap => "WasmCustomMap",
      Self::WasmCustomMapWrapper => "WasmCustomMapWrapper",
      Self::WasmExceptionPackage => "WasmExceptionPackage",
      Self::WasmFastApiCallData => "WasmFastApiCallData",
      Self::WasmFuncRef => "WasmFuncRef",
      Self::WasmGlobalObject => "WasmGlobalObject",
      Self::WasmInstanceObject => "WasmInstanceObject",
      Self::WasmMemoryObject => "WasmMemoryObject",
      Self::WasmModuleObject => "WasmModuleObject",
      Self::WasmNull => "WasmNull",
      Self::WasmObject => "WasmObject",
      Self::WasmResumeData => "WasmResumeData",
      Self::WasmStackObject => "WasmStackObject",
      Self::WasmStringViewIter => "WasmStringViewIter",
      Self::WasmStruct => "WasmStruct",
      Self::WasmSuspendingObject => "WasmSuspendingObject",
      Self::WasmTableObject => "WasmTableObject",
      Self::WasmTagObject => "WasmTagObject",
      Self::WasmTypeInfo => "WasmTypeInfo",
      Self::WasmValueObject => "WasmValueObject",
      Self::WeakCell => "WeakCell",
      Self::ArrayList => "ArrayList",
      Self::ByteArray => "ByteArray",
      Self::ClosureFeedbackCellArray => "ClosureFeedbackCellArray",
      Self::FixedArray => "FixedArray",
      Self::FixedDoubleArray => "FixedDoubleArray",
      Self::ObjectBoilerplateDescription => "ObjectBoilerplateDescription",
      Self::RegExpMatchInfo => "RegExpMatchInfo",
      Self::ScriptContextTable => "ScriptContextTable",
      Self::SloppyArgumentsElements => "SloppyArgumentsElements",
      Self::WeakArrayList => "WeakArrayList",
      Self::WeakFixedArray => "WeakFixedArray",
      Self::WeakHomomorphicFixedArray => "WeakHomomorphicFixedArray",
      Self::JSV8BreakIterator => "JSV8BreakIterator",
      Self::JSCollator => "JSCollator",
      Self::JSDateTimeFormat => "JSDateTimeFormat",
      Self::JSDisplayNames => "JSDisplayNames",
      Self::JSDurationFormat => "JSDurationFormat",
      Self::JSListFormat => "JSListFormat",
      Self::JSLocale => "JSLocale",
      Self::JSNumberFormat => "JSNumberFormat",
      Self::JSPluralRules => "JSPluralRules",
      Self::JSRelativeTimeFormat => "JSRelativeTimeFormat",
      Self::JSSegmentDataObject => "JSSegmentDataObject",
      Self::JSSegmentDataObjectWithIsWordLike => "JSSegmentDataObjectWithIsWordLike",
      Self::JSSegmentIterator => "JSSegmentIterator",
      Self::JSSegmenter => "JSSegmenter",
      Self::JSSegments => "JSSegments",
      Self::AccessCheckNeeded => "AccessCheckNeeded",
      Self::AlwaysSharedSpaceJSObject => "AlwaysSharedSpaceJSObject",
      Self::BigIntWrapper => "BigIntWrapper",
      Self::BooleanWrapper => "BooleanWrapper",
      Self::Callable => "Callable",
      Self::Constructor => "Constructor",
      Self::Filler => "Filler",
      Self::HandlerTable => "HandlerTable",
      Self::JSContextExtensionObject => "JSContextExtensionObject",
      Self::JSError => "JSError",
      Self::JSInterceptorMap => "JSInterceptorMap",
      Self::MapCache => "MapCache",
      Self::ExtendedMap => "ExtendedMap",
      Self::MetaMap => "MetaMap",
      Self::NumberWrapper => "NumberWrapper",
      Self::OSROptimizedCodeCache => "OSROptimizedCodeCache",
      Self::ScriptWrapper => "ScriptWrapper",
      Self::SmiStringCache => "SmiStringCache",
      Self::StringWrapper => "StringWrapper",
      Self::SymbolWrapper => "SymbolWrapper",
      Self::UniqueName => "UniqueName",
      Self::Undetectable => "Undetectable",
      Self::TrustedObject => "TrustedObject",
      Self::ExposedTrustedObject => "ExposedTrustedObject",
      Self::TrustedFixedArrayBase => "TrustedFixedArrayBase",
      Self::UncompiledData => "UncompiledData",
      Self::WasmFunctionData => "WasmFunctionData",
      Self::BytecodeArray => "BytecodeArray",
      Self::Code => "Code",
      Self::DebugInfo => "DebugInfo",
      Self::InstructionStream => "InstructionStream",
      Self::InterpreterData => "InterpreterData",
      Self::UncompiledDataWithPreparseData => "UncompiledDataWithPreparseData",
      Self::UncompiledDataWithoutPreparseData => "UncompiledDataWithoutPreparseData",
      Self::UncompiledDataWithPreparseDataAndJob => "UncompiledDataWithPreparseDataAndJob",
      Self::UncompiledDataWithoutPreparseDataWithJob => "UncompiledDataWithoutPreparseDataWithJob",
      Self::SharedFunctionInfoWrapper => "SharedFunctionInfoWrapper",
      Self::ProtectedFixedArray => "ProtectedFixedArray",
      Self::ProtectedWeakFixedArray => "ProtectedWeakFixedArray",
      Self::TrustedByteArray => "TrustedByteArray",
      Self::TrustedFixedArray => "TrustedFixedArray",
      Self::TrustedForeign => "TrustedForeign",
      Self::TrustedWeakFixedArray => "TrustedWeakFixedArray",
      Self::AtomRegExpData => "AtomRegExpData",
      Self::IrRegExpData => "IrRegExpData",
      Self::RegExpData => "RegExpData",
      Self::WasmImportData => "WasmImportData",
      Self::WasmCapiFunctionData => "WasmCapiFunctionData",
      Self::WasmDispatchTable => "WasmDispatchTable",
      Self::WasmDispatchTableForImports => "WasmDispatchTableForImports",
      Self::WasmExportedFunctionData => "WasmExportedFunctionData",
      Self::WasmInternalFunction => "WasmInternalFunction",
      Self::WasmTrustedInstanceData => "WasmTrustedInstanceData",
      Self::WasmSuspenderObject => "WasmSuspenderObject",
      Self::HashTable => "HashTable",
      Self::AwaitContext => "AwaitContext",
      Self::BlockContext => "BlockContext",
      Self::CallableApiObject => "CallableApiObject",
      Self::CallableJSFunction => "CallableJSFunction",
      Self::CallableJSProxy => "CallableJSProxy",
      Self::CatchContext => "CatchContext",
      Self::CppHeapPointerWrapperObject => "CppHeapPointerWrapperObject",
      Self::DebugEvaluateContext => "DebugEvaluateContext",
      Self::EvalContext => "EvalContext",
      Self::FreeSpaceOrFiller => "FreeSpaceOrFiller",
      Self::FunctionContext => "FunctionContext",
      Self::JSApiObject => "JSApiObject",
      Self::JSApiWrapperObject => "JSApiWrapperObject",
      Self::JSClassConstructor => "JSClassConstructor",
      Self::JSLastDummyApiObject => "JSLastDummyApiObject",
      Self::JSPromiseConstructor => "JSPromiseConstructor",
      Self::JSArrayConstructor => "JSArrayConstructor",
      Self::JSRegExpConstructor => "JSRegExpConstructor",
      Self::JSMapKeyIterator => "JSMapKeyIterator",
      Self::JSMapKeyValueIterator => "JSMapKeyValueIterator",
      Self::JSMapValueIterator => "JSMapValueIterator",
      Self::JSSetKeyValueIterator => "JSSetKeyValueIterator",
      Self::JSSetValueIterator => "JSSetValueIterator",
      Self::JSSpecialApiObject => "JSSpecialApiObject",
      Self::MaybeReadOnlyJSObject => "MaybeReadOnlyJSObject",
      Self::ModuleContext => "ModuleContext",
      Self::NonNullForeign => "NonNullForeign",
      Self::ScriptContext => "ScriptContext",
      Self::WithContext => "WithContext",
      Self::JSInternalPrototypeBase => "JSInternalPrototypeBase",
      Self::JSObjectPrototype => "JSObjectPrototype",
      Self::JSRegExpPrototype => "JSRegExpPrototype",
      Self::JSPromisePrototype => "JSPromisePrototype",
      Self::JSSetPrototype => "JSSetPrototype",
      Self::JSIteratorPrototype => "JSIteratorPrototype",
      Self::JSArrayIteratorPrototype => "JSArrayIteratorPrototype",
      Self::JSMapIteratorPrototype => "JSMapIteratorPrototype",
      Self::JSTypedArrayPrototype => "JSTypedArrayPrototype",
      Self::JSSetIteratorPrototype => "JSSetIteratorPrototype",
      Self::JSStringIteratorPrototype => "JSStringIteratorPrototype",
      Self::TypedArrayConstructor => "TypedArrayConstructor",
      Self::Uint8TypedArrayConstructor => "Uint8TypedArrayConstructor",
      Self::Int8TypedArrayConstructor => "Int8TypedArrayConstructor",
      Self::Uint16TypedArrayConstructor => "Uint16TypedArrayConstructor",
      Self::Int16TypedArrayConstructor => "Int16TypedArrayConstructor",
      Self::Uint32TypedArrayConstructor => "Uint32TypedArrayConstructor",
      Self::Int32TypedArrayConstructor => "Int32TypedArrayConstructor",
      Self::Float16TypedArrayConstructor => "Float16TypedArrayConstructor",
      Self::Float32TypedArrayConstructor => "Float32TypedArrayConstructor",
      Self::Float64TypedArrayConstructor => "Float64TypedArrayConstructor",
      Self::Uint8ClampedTypedArrayConstructor => "Uint8ClampedTypedArrayConstructor",
      Self::Biguint64TypedArrayConstructor => "Biguint64TypedArrayConstructor",
      Self::Bigint64TypedArrayConstructor => "Bigint64TypedArrayConstructor",
      Self::PromiseFulfillReactionJobTask => "PromiseFulfillReactionJobTask",
      Self::PromiseRejectReactionJobTask => "PromiseRejectReactionJobTask",
      Self::AsyncResumeTask => "AsyncResumeTask",
      Self::CallableTask => "CallableTask",
      Self::CallbackTask => "CallbackTask",
      Self::PromiseResolveThenableJobTask => "PromiseResolveThenableJobTask",
      Self::AccessCheckInfo => "AccessCheckInfo",
      Self::AccessorPair => "AccessorPair",
      Self::AliasedArgumentsEntry => "AliasedArgumentsEntry",
      Self::AllocationMemento => "AllocationMemento",
      Self::ArrayBoilerplateDescription => "ArrayBoilerplateDescription",
      Self::AsyncGeneratorRequest => "AsyncGeneratorRequest",
      Self::BreakPoint => "BreakPoint",
      Self::BreakPointInfo => "BreakPointInfo",
      Self::BytecodeWrapper => "BytecodeWrapper",
      Self::CallSiteInfo => "CallSiteInfo",
      Self::ClassBoilerplate => "ClassBoilerplate",
      Self::ClassPositions => "ClassPositions",
      Self::CodeWrapper => "CodeWrapper",
      Self::DebugScriptScopeInfo => "DebugScriptScopeInfo",
      Self::EnumCache => "EnumCache",
      Self::ErrorStackData => "ErrorStackData",
      Self::ForInEnumeratorHolder => "ForInEnumeratorHolder",
      Self::FunctionTemplateRareData => "FunctionTemplateRareData",
      Self::ModuleRequest => "ModuleRequest",
      Self::PromiseCapability => "PromiseCapability",
      Self::PromiseReaction => "PromiseReaction",
      Self::PropertyDescriptorObject => "PropertyDescriptorObject",
      Self::PrototypeInfo => "PrototypeInfo",
      Self::PrototypeSharedClosureInfo => "PrototypeSharedClosureInfo",
      Self::RegExpBoilerplateDescription => "RegExpBoilerplateDescription",
      Self::RegExpDataWrapper => "RegExpDataWrapper",
      Self::Script => "Script",
      Self::ScriptOrModule => "ScriptOrModule",
      Self::SourceTextModuleInfoEntry => "SourceTextModuleInfoEntry",
      Self::StackFrameInfo => "StackFrameInfo",
      Self::StackTraceInfo => "StackTraceInfo",
      Self::TemplateObjectDescription => "TemplateObjectDescription",
      Self::Tuple2 => "Tuple2",
      Self::WasmExceptionTag => "WasmExceptionTag",
    }
  }
}

// —— 签名骨架 (原版 CheckObjectType, DEBUG 构建专用) ——
// Address CheckObjectType(Address raw_value, Address raw_type, Address raw_location);
// 弱引用路径: kHeapObjectReference 直接放行, 其余标签一律报错 (弱引用不可直接 cast)
// 强引用路径: 逐标签调用 Is<Name>(value) 判定, 通过返回 Smi(0), 失败 FATAL 定位
// 发布构建: UNREACHABLE (仅在 DEBUG 生效)
pub const CHECK_OBJECT_TYPE_DEBUG_ONLY: bool = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(ObjectType::COUNT == 386);
  assert!(ObjectType::Object as u16 == 0);
  assert!(ObjectType::HeapObjectReference as u16 == 4);
};
