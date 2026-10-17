// 由 tool/src/maglev/maglev-ir-opcodes.js 从 v8/src/maglev/maglev-ir.h 生成; 修订就地改, 复跑覆盖
// Opcode 全表 378; 非目标配置计数 379/379/380 (UD-only / INTL-only / UD+INTL)

//! Maglev IR opcode 纯数据面: 枚举、分组区间常量、opcode 集合谓词。
//! 原版锚: maglev-ir.h `enum class Opcode : uint16_t { NODE_BASE_LIST(DEF_OPCODES) }`。
//! 目标配置 V8_INTL_SUPPORT=off / V8_ENABLE_UNDEFINED_DOUBLE=off; 其余组合见台账 readme/zh.md。

/// 原版 `static constexpr int kOpcodeCount`。
pub const OPCODE_COUNT: usize = 378;
/// 原版 `kLastOpcode`: 枚举最大判别值。
pub const LAST_OPCODE: u16 = 377;
/// 原版 `kFirstOpcode`: 枚举从 0 起连续分配。
pub const FIRST_OPCODE: u16 = 0;

/// Opcode 全表, 顺序与判别值 1:1 复刻 `NODE_BASE_LIST(DEF_OPCODES)` 连续分配序。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum Opcode {
  AssertInt32 = 0,
  AssertRangeInt32 = 1,
  AssertRangeFloat64 = 2,
  CheckDynamicValue = 3,
  CheckInt32IsSmi = 4,
  CheckUint32IsSmi = 5,
  CheckIntPtrIsSmi = 6,
  CheckFloat64IsSmi = 7,
  CheckHoleyFloat64IsSmi = 8,
  CheckHeapObject = 9,
  CheckInt32Condition = 10,
  CheckCacheIndicesNotCleared = 11,
  CheckJSDataViewBounds = 12,
  CheckTypedArrayBounds = 13,
  CheckTypedArrayValid = 14,
  CheckMaps = 15,
  CheckHomomorphicMap = 16,
  CheckMapsWithMigrationAndDeopt = 17,
  CheckMapsWithMigration = 18,
  CheckMapsWithAlreadyLoadedMap = 19,
  CheckDetectableCallable = 20,
  CheckJSReceiverOrNullOrUndefined = 21,
  CheckNotHole = 22,
  CheckNotUndefined = 23,
  CheckHoleyFloat64NotHoleOrUndefined = 24,
  CheckNumber = 25,
  CheckSmi = 26,
  CheckString = 27,
  CheckStringOrStringWrapper = 28,
  CheckStringOrOddball = 29,
  CheckSymbol = 30,
  CheckValue = 31,
  CheckValueEqualsInt32 = 32,
  CheckFloat64SameValue = 33,
  CheckValueEqualsString = 34,
  CheckInstanceType = 35,
  Dead = 36,
  DebugBreak = 37,
  MajorGCForCompilerTesting = 38,
  FunctionEntryStackCheck = 39,
  GeneratorStore = 40,
  StoreMap = 41,
  StoreFixedArrayElementWithWriteBarrier = 42,
  StoreFixedArrayElementNoWriteBarrier = 43,
  StoreFixedDoubleArrayElement = 44,
  StoreFixedDoubleArrayHole = 45,
  StoreFixedHoleyDoubleArrayElement = 46,
  StoreInt32 = 47,
  StoreFloat64 = 48,
  StoreIntTypedArrayElement = 49,
  StoreDoubleTypedArrayElement = 50,
  StoreIntConstantTypedArrayElement = 51,
  StoreDoubleConstantTypedArrayElement = 52,
  StoreInt32DataViewElement = 53,
  StoreDoubleDataViewElement = 54,
  StoreTaggedFieldNoWriteBarrier = 55,
  StoreTaggedFieldWithWriteBarrier = 56,
  StoreSmiContextCell = 57,
  StoreInt32ContextCell = 58,
  StoreFloat64ContextCell = 59,
  StoreContextSlotWithWriteBarrier = 60,
  StoreTrustedPointerFieldWithWriteBarrier = 61,
  HandleNoHeapWritesInterrupt = 62,
  ReduceInterruptBudgetForLoop = 63,
  ReduceInterruptBudgetForReturn = 64,
  ThrowReferenceErrorIfTdzHole = 65,
  ThrowSuperNotCalledIfTdzHole = 66,
  ThrowSuperAlreadyCalledIfNotTdzHole = 67,
  ThrowIfNotCallable = 68,
  ThrowIfNotSuperConstructor = 69,
  TransitionElementsKindOrCheckMap = 70,
  SetContinuationPreservedEmbedderData = 71,
  FulfillPromise = 72,
  CheckMaglevType = 73,
  Trap = 74,
  ConstantGapMove = 75,
  GapMove = 76,
  TransitionAndStoreArrayElement = 77,
  TurbofanStaticAssert = 78,
  AssertPeeled = 79,
  AssertEscapeAnalysisElided = 80,
  AssumeMap = 81,
  AssumeType = 82,
  Identity = 83,
  DeadValue = 84,
  AllocationBlock = 85,
  ArgumentsElements = 86,
  ArgumentsLength = 87,
  RestLength = 88,
  Call = 89,
  CallBuiltin = 90,
  CallForwardVarargs = 91,
  CallRuntime = 92,
  CallWithArrayLike = 93,
  CallWithSpread = 94,
  CallKnownApiFunction = 95,
  CallKnownBuiltin = 96,
  CallKnownJSFunction = 97,
  CallSelf = 98,
  Construct = 99,
  ConstructForwardVarargs = 100,
  CheckConstructResult = 101,
  CheckDerivedConstructResult = 102,
  ConstructWithSpread = 103,
  ConvertReceiver = 104,
  ConvertHoleToUndefined = 105,
  CreateArrayLiteral = 106,
  CreateShallowArrayLiteral = 107,
  CreateObjectLiteral = 108,
  CreateShallowObjectLiteral = 109,
  CreateFunctionContext = 110,
  CreateClosure = 111,
  FastCreateClosure = 112,
  CreateRegExpLiteral = 113,
  DeleteProperty = 114,
  EnsureWritableFastElements = 115,
  ExtendPropertiesBackingStore = 116,
  InlinedAllocation = 117,
  ForInPrepare = 118,
  ForInNext = 119,
  GeneratorRestoreRegister = 120,
  GetIterator = 121,
  GetSecondReturnedValue = 122,
  GetTemplateObject = 123,
  HasInPrototypeChain = 124,
  InitialValue = 125,
  LoadTaggedField = 126,
  LoadContextSlotNoCells = 127,
  LoadContextSlot = 128,
  LoadFloat64 = 129,
  LoadInt32 = 130,
  LoadTaggedFieldByFieldIndex = 131,
  LoadFixedArrayElement = 132,
  LoadFixedDoubleArrayElement = 133,
  LoadHoleyFixedDoubleArrayElement = 134,
  LoadInt32DataViewElement = 135,
  LoadUint32DataViewElement = 136,
  LoadDoubleDataViewElement = 137,
  LoadTypedArrayLength = 138,
  LoadDataViewByteLength = 139,
  LoadDataViewDataPointer = 140,
  LoadSignedIntTypedArrayElement = 141,
  LoadUnsignedIntTypedArrayElement = 142,
  LoadDoubleTypedArrayElement = 143,
  LoadSignedIntConstantTypedArrayElement = 144,
  LoadUnsignedIntConstantTypedArrayElement = 145,
  LoadDoubleConstantTypedArrayElement = 146,
  LoadEnumCacheLength = 147,
  LoadGlobal = 148,
  LoadNamedGeneric = 149,
  LoadNamedFromSuperGeneric = 150,
  LoadDictionaryField = 151,
  MaybeGrowFastElements = 152,
  MigrateMapIfNeeded = 153,
  SetNamedGeneric = 154,
  DefineNamedOwnGeneric = 155,
  StoreInArrayLiteralGeneric = 156,
  StoreGlobal = 157,
  GetKeyedGeneric = 158,
  SetKeyedGeneric = 159,
  DefineKeyedOwnGeneric = 160,
  Phi = 161,
  RegisterInput = 162,
  CheckedInternalizedString = 163,
  TruncateCheckedNumberAsSafeIntToInt32 = 164,
  TruncateUnsafeNumberAsSafeIntToInt32 = 165,
  TruncateFloat64AsSafeIntToInt32 = 166,
  TruncateHoleyFloat64AsSafeIntToInt32 = 167,
  TruncateCheckedNumberOrOddballToInt32 = 168,
  TruncateUnsafeNumberOrOddballToInt32 = 169,
  TruncateFloat64ToInt32 = 170,
  TruncateHoleyFloat64ToInt32 = 171,
  TruncateUint32ToInt32 = 172,
  Int32ToUint8Clamped = 173,
  Uint32ToUint8Clamped = 174,
  Float64ToUint8Clamped = 175,
  CheckedNumberOrOddballToUint8Clamped = 176,
  Int32CountLeadingZeros = 177,
  TaggedCountLeadingZeros = 178,
  Float64CountLeadingZeros = 179,
  IntPtrToBoolean = 180,
  CheckedNumberOrOddballToFloat64 = 181,
  CheckedNumberOrOddballToHoleyFloat64 = 182,
  UnsafeNumberOrOddballToFloat64 = 183,
  UnsafeNumberOrOddballToHoleyFloat64 = 184,
  UnsafeHoleyFloat64ToFloat64 = 185,
  UnsafeFloat64ToHoleyFloat64 = 186,
  Float64ToSilencedFloat64 = 187,
  HoleyFloat64IsHole = 188,
  LogicalNot = 189,
  SetPendingMessage = 190,
  StringAt = 191,
  StringEqual = 192,
  StringLength = 193,
  StringConcat = 194,
  StringIndexOf = 195,
  ConsStringMap = 196,
  UnwrapStringWrapper = 197,
  ToBoolean = 198,
  ToBooleanLogicalNot = 199,
  AllocateElementsArray = 200,
  TaggedEqual = 201,
  TaggedNotEqual = 202,
  TestInstanceOf = 203,
  TestUndetectable = 204,
  TestTypeOf = 205,
  ToName = 206,
  ToNumberOrNumeric = 207,
  ToObject = 208,
  ToString = 209,
  TransitionElementsKind = 210,
  Int32ToString = 211,
  Float64ToString = 212,
  SmiToString = 213,
  NumberToString = 214,
  UpdateJSArrayLength = 215,
  VirtualObject = 216,
  GetContinuationPreservedEmbedderData = 217,
  ReturnedValue = 218,
  StringSlice = 219,
  StringSubstring = 220,
  HeapConstant = 221,
  Float64Constant = 222,
  HoleyFloat64Constant = 223,
  Int32Constant = 224,
  Uint32Constant = 225,
  IntPtrConstant = 226,
  RootConstant = 227,
  SmiConstant = 228,
  TaggedIndexConstant = 229,
  TrustedConstant = 230,
  ChangeInt32ToFloat64 = 231,
  ChangeInt32ToHoleyFloat64 = 232,
  ChangeIntPtrToFloat64 = 233,
  ChangeUint32ToFloat64 = 234,
  ChangeUint32ToHoleyFloat64 = 235,
  ChangeFloat64ToHoleyFloat64 = 236,
  CheckedFloat64ToInt32 = 237,
  CheckedFloat64ToUint32 = 238,
  CheckedFloat64ToSmiSizedInt32 = 239,
  CheckedHoleyFloat64ToSmiSizedInt32 = 240,
  CheckedHoleyFloat64ToInt32 = 241,
  CheckedHoleyFloat64ToUint32 = 242,
  CheckedHoleyFloat64ToFloat64 = 243,
  CheckedInt32ToUint32 = 244,
  CheckedIntPtrToInt32 = 245,
  CheckedIntPtrToUint32 = 246,
  CheckedObjectToIndex = 247,
  CheckedNumberToInt32 = 248,
  CheckedNumberToFloat64 = 249,
  CheckedSmiSizedInt32 = 250,
  CheckedSmiTagFloat64 = 251,
  CheckedSmiTagHoleyFloat64 = 252,
  CheckedSmiTagInt32 = 253,
  CheckedSmiTagIntPtr = 254,
  CheckedSmiTagUint32 = 255,
  CheckedSmiUntag = 256,
  CheckedUint32ToInt32 = 257,
  Float64ToTagged = 258,
  HoleyFloat64ToTagged = 259,
  Int32ToNumber = 260,
  IntPtrToNumber = 261,
  Uint32ToNumber = 262,
  UnsafeFloat64ToInt32 = 263,
  UnsafeHoleyFloat64ToInt32 = 264,
  UnsafeInt32ToUint32 = 265,
  UnsafeNumberToFloat64 = 266,
  UnsafeSmiTagInt32 = 267,
  UnsafeSmiTagFloat64 = 268,
  UnsafeSmiTagHoleyFloat64 = 269,
  UnsafeSmiTagIntPtr = 270,
  UnsafeSmiTagUint32 = 271,
  UnsafeSmiUntag = 272,
  Int32BitwiseAnd = 273,
  Int32BitwiseOr = 274,
  Int32BitwiseXor = 275,
  Int32ShiftLeft = 276,
  Int32ShiftRight = 277,
  Int32ShiftRightLogical = 278,
  Int32AbsWithOverflow = 279,
  Int32Add = 280,
  Int32Subtract = 281,
  Int32Multiply = 282,
  Int32MultiplyOverflownBits = 283,
  Int32Divide = 284,
  Int32Increment = 285,
  Int32Decrement = 286,
  Int32AddWithOverflow = 287,
  Int32SubtractWithOverflow = 288,
  Int32MultiplyWithOverflow = 289,
  Int32DivideWithOverflow = 290,
  Int32ModulusWithOverflow = 291,
  Int32BitwiseNot = 292,
  Int32NegateWithOverflow = 293,
  Int32IncrementWithOverflow = 294,
  Int32DecrementWithOverflow = 295,
  Int32Compare = 296,
  Int32ToBoolean = 297,
  Float64Abs = 298,
  Float64Add = 299,
  Float64SpeculateSafeAdd = 300,
  Float64Subtract = 301,
  Float64Multiply = 302,
  Float64Divide = 303,
  Float64Exponentiate = 304,
  Float64Modulus = 305,
  Float64Negate = 306,
  Float64Max = 307,
  Float64Min = 308,
  Float64RoundToFloat32 = 309,
  Float64Round = 310,
  Float64Compare = 311,
  Float64SameValue = 312,
  Float64ToBoolean = 313,
  Float64Ieee754Unary = 314,
  Float64Ieee754Binary = 315,
  Float64Sqrt = 316,
  CheckedSmiIncrement = 317,
  CheckedSmiDecrement = 318,
  GenericAdd = 319,
  GenericSubtract = 320,
  GenericMultiply = 321,
  GenericDivide = 322,
  GenericModulus = 323,
  GenericExponentiate = 324,
  GenericBitwiseAnd = 325,
  GenericBitwiseOr = 326,
  GenericBitwiseXor = 327,
  GenericShiftLeft = 328,
  GenericShiftRight = 329,
  GenericShiftRightLogical = 330,
  GenericBitwiseNot = 331,
  GenericNegate = 332,
  GenericIncrement = 333,
  GenericDecrement = 334,
  GenericEqual = 335,
  GenericStrictEqual = 336,
  GenericLessThan = 337,
  GenericLessThanOrEqual = 338,
  GenericGreaterThan = 339,
  GenericGreaterThanOrEqual = 340,
  BigIntBinaryOperation = 341,
  BigIntCompare = 342,
  BigIntNegate = 343,
  BuiltinStringFromCharCode = 344,
  BuiltinStringPrototypeCharCodeOrCodePointAt = 345,
  CreateFastArrayElements = 346,
  NewConsString = 347,
  MapPrototypeGet = 348,
  MapPrototypeGetInt32Key = 349,
  SetPrototypeHas = 350,
  WeakMapPrototypeGet = 351,
  ObjectIsArray = 352,
  ProcessWasmArgument = 353,
  Abort = 354,
  Return = 355,
  Deopt = 356,
  Throw = 357,
  Switch = 358,
  BranchIfSmi = 359,
  BranchIfRootConstant = 360,
  BranchIfToBooleanTrue = 361,
  BranchIfInt32ToBooleanTrue = 362,
  BranchIfIntPtrToBooleanTrue = 363,
  BranchIfFloat64ToBooleanTrue = 364,
  BranchIfHoleyFloat64ToBooleanTrue = 365,
  BranchIfFloat64IsHole = 366,
  BranchIfReferenceEqual = 367,
  BranchIfInt32Compare = 368,
  BranchIfUint32Compare = 369,
  BranchIfFloat64Compare = 370,
  BranchIfUndefinedOrNull = 371,
  BranchIfUndetectable = 372,
  BranchIfJSReceiver = 373,
  BranchIfTypedArrayBounds = 374,
  Jump = 375,
  CheckpointedJump = 376,
  JumpLoop = 377,
}

/// C++ 类名表, 下标即枚举值 (原版 `Opcode::k##Name` 去掉 k 前缀)。
pub const OPCODE_NAMES: [&str; OPCODE_COUNT] = [
  "AssertInt32",
  "AssertRangeInt32",
  "AssertRangeFloat64",
  "CheckDynamicValue",
  "CheckInt32IsSmi",
  "CheckUint32IsSmi",
  "CheckIntPtrIsSmi",
  "CheckFloat64IsSmi",
  "CheckHoleyFloat64IsSmi",
  "CheckHeapObject",
  "CheckInt32Condition",
  "CheckCacheIndicesNotCleared",
  "CheckJSDataViewBounds",
  "CheckTypedArrayBounds",
  "CheckTypedArrayValid",
  "CheckMaps",
  "CheckHomomorphicMap",
  "CheckMapsWithMigrationAndDeopt",
  "CheckMapsWithMigration",
  "CheckMapsWithAlreadyLoadedMap",
  "CheckDetectableCallable",
  "CheckJSReceiverOrNullOrUndefined",
  "CheckNotHole",
  "CheckNotUndefined",
  "CheckHoleyFloat64NotHoleOrUndefined",
  "CheckNumber",
  "CheckSmi",
  "CheckString",
  "CheckStringOrStringWrapper",
  "CheckStringOrOddball",
  "CheckSymbol",
  "CheckValue",
  "CheckValueEqualsInt32",
  "CheckFloat64SameValue",
  "CheckValueEqualsString",
  "CheckInstanceType",
  "Dead",
  "DebugBreak",
  "MajorGCForCompilerTesting",
  "FunctionEntryStackCheck",
  "GeneratorStore",
  "StoreMap",
  "StoreFixedArrayElementWithWriteBarrier",
  "StoreFixedArrayElementNoWriteBarrier",
  "StoreFixedDoubleArrayElement",
  "StoreFixedDoubleArrayHole",
  "StoreFixedHoleyDoubleArrayElement",
  "StoreInt32",
  "StoreFloat64",
  "StoreIntTypedArrayElement",
  "StoreDoubleTypedArrayElement",
  "StoreIntConstantTypedArrayElement",
  "StoreDoubleConstantTypedArrayElement",
  "StoreInt32DataViewElement",
  "StoreDoubleDataViewElement",
  "StoreTaggedFieldNoWriteBarrier",
  "StoreTaggedFieldWithWriteBarrier",
  "StoreSmiContextCell",
  "StoreInt32ContextCell",
  "StoreFloat64ContextCell",
  "StoreContextSlotWithWriteBarrier",
  "StoreTrustedPointerFieldWithWriteBarrier",
  "HandleNoHeapWritesInterrupt",
  "ReduceInterruptBudgetForLoop",
  "ReduceInterruptBudgetForReturn",
  "ThrowReferenceErrorIfTdzHole",
  "ThrowSuperNotCalledIfTdzHole",
  "ThrowSuperAlreadyCalledIfNotTdzHole",
  "ThrowIfNotCallable",
  "ThrowIfNotSuperConstructor",
  "TransitionElementsKindOrCheckMap",
  "SetContinuationPreservedEmbedderData",
  "FulfillPromise",
  "CheckMaglevType",
  "Trap",
  "ConstantGapMove",
  "GapMove",
  "TransitionAndStoreArrayElement",
  "TurbofanStaticAssert",
  "AssertPeeled",
  "AssertEscapeAnalysisElided",
  "AssumeMap",
  "AssumeType",
  "Identity",
  "DeadValue",
  "AllocationBlock",
  "ArgumentsElements",
  "ArgumentsLength",
  "RestLength",
  "Call",
  "CallBuiltin",
  "CallForwardVarargs",
  "CallRuntime",
  "CallWithArrayLike",
  "CallWithSpread",
  "CallKnownApiFunction",
  "CallKnownBuiltin",
  "CallKnownJSFunction",
  "CallSelf",
  "Construct",
  "ConstructForwardVarargs",
  "CheckConstructResult",
  "CheckDerivedConstructResult",
  "ConstructWithSpread",
  "ConvertReceiver",
  "ConvertHoleToUndefined",
  "CreateArrayLiteral",
  "CreateShallowArrayLiteral",
  "CreateObjectLiteral",
  "CreateShallowObjectLiteral",
  "CreateFunctionContext",
  "CreateClosure",
  "FastCreateClosure",
  "CreateRegExpLiteral",
  "DeleteProperty",
  "EnsureWritableFastElements",
  "ExtendPropertiesBackingStore",
  "InlinedAllocation",
  "ForInPrepare",
  "ForInNext",
  "GeneratorRestoreRegister",
  "GetIterator",
  "GetSecondReturnedValue",
  "GetTemplateObject",
  "HasInPrototypeChain",
  "InitialValue",
  "LoadTaggedField",
  "LoadContextSlotNoCells",
  "LoadContextSlot",
  "LoadFloat64",
  "LoadInt32",
  "LoadTaggedFieldByFieldIndex",
  "LoadFixedArrayElement",
  "LoadFixedDoubleArrayElement",
  "LoadHoleyFixedDoubleArrayElement",
  "LoadInt32DataViewElement",
  "LoadUint32DataViewElement",
  "LoadDoubleDataViewElement",
  "LoadTypedArrayLength",
  "LoadDataViewByteLength",
  "LoadDataViewDataPointer",
  "LoadSignedIntTypedArrayElement",
  "LoadUnsignedIntTypedArrayElement",
  "LoadDoubleTypedArrayElement",
  "LoadSignedIntConstantTypedArrayElement",
  "LoadUnsignedIntConstantTypedArrayElement",
  "LoadDoubleConstantTypedArrayElement",
  "LoadEnumCacheLength",
  "LoadGlobal",
  "LoadNamedGeneric",
  "LoadNamedFromSuperGeneric",
  "LoadDictionaryField",
  "MaybeGrowFastElements",
  "MigrateMapIfNeeded",
  "SetNamedGeneric",
  "DefineNamedOwnGeneric",
  "StoreInArrayLiteralGeneric",
  "StoreGlobal",
  "GetKeyedGeneric",
  "SetKeyedGeneric",
  "DefineKeyedOwnGeneric",
  "Phi",
  "RegisterInput",
  "CheckedInternalizedString",
  "TruncateCheckedNumberAsSafeIntToInt32",
  "TruncateUnsafeNumberAsSafeIntToInt32",
  "TruncateFloat64AsSafeIntToInt32",
  "TruncateHoleyFloat64AsSafeIntToInt32",
  "TruncateCheckedNumberOrOddballToInt32",
  "TruncateUnsafeNumberOrOddballToInt32",
  "TruncateFloat64ToInt32",
  "TruncateHoleyFloat64ToInt32",
  "TruncateUint32ToInt32",
  "Int32ToUint8Clamped",
  "Uint32ToUint8Clamped",
  "Float64ToUint8Clamped",
  "CheckedNumberOrOddballToUint8Clamped",
  "Int32CountLeadingZeros",
  "TaggedCountLeadingZeros",
  "Float64CountLeadingZeros",
  "IntPtrToBoolean",
  "CheckedNumberOrOddballToFloat64",
  "CheckedNumberOrOddballToHoleyFloat64",
  "UnsafeNumberOrOddballToFloat64",
  "UnsafeNumberOrOddballToHoleyFloat64",
  "UnsafeHoleyFloat64ToFloat64",
  "UnsafeFloat64ToHoleyFloat64",
  "Float64ToSilencedFloat64",
  "HoleyFloat64IsHole",
  "LogicalNot",
  "SetPendingMessage",
  "StringAt",
  "StringEqual",
  "StringLength",
  "StringConcat",
  "StringIndexOf",
  "ConsStringMap",
  "UnwrapStringWrapper",
  "ToBoolean",
  "ToBooleanLogicalNot",
  "AllocateElementsArray",
  "TaggedEqual",
  "TaggedNotEqual",
  "TestInstanceOf",
  "TestUndetectable",
  "TestTypeOf",
  "ToName",
  "ToNumberOrNumeric",
  "ToObject",
  "ToString",
  "TransitionElementsKind",
  "Int32ToString",
  "Float64ToString",
  "SmiToString",
  "NumberToString",
  "UpdateJSArrayLength",
  "VirtualObject",
  "GetContinuationPreservedEmbedderData",
  "ReturnedValue",
  "StringSlice",
  "StringSubstring",
  "HeapConstant",
  "Float64Constant",
  "HoleyFloat64Constant",
  "Int32Constant",
  "Uint32Constant",
  "IntPtrConstant",
  "RootConstant",
  "SmiConstant",
  "TaggedIndexConstant",
  "TrustedConstant",
  "ChangeInt32ToFloat64",
  "ChangeInt32ToHoleyFloat64",
  "ChangeIntPtrToFloat64",
  "ChangeUint32ToFloat64",
  "ChangeUint32ToHoleyFloat64",
  "ChangeFloat64ToHoleyFloat64",
  "CheckedFloat64ToInt32",
  "CheckedFloat64ToUint32",
  "CheckedFloat64ToSmiSizedInt32",
  "CheckedHoleyFloat64ToSmiSizedInt32",
  "CheckedHoleyFloat64ToInt32",
  "CheckedHoleyFloat64ToUint32",
  "CheckedHoleyFloat64ToFloat64",
  "CheckedInt32ToUint32",
  "CheckedIntPtrToInt32",
  "CheckedIntPtrToUint32",
  "CheckedObjectToIndex",
  "CheckedNumberToInt32",
  "CheckedNumberToFloat64",
  "CheckedSmiSizedInt32",
  "CheckedSmiTagFloat64",
  "CheckedSmiTagHoleyFloat64",
  "CheckedSmiTagInt32",
  "CheckedSmiTagIntPtr",
  "CheckedSmiTagUint32",
  "CheckedSmiUntag",
  "CheckedUint32ToInt32",
  "Float64ToTagged",
  "HoleyFloat64ToTagged",
  "Int32ToNumber",
  "IntPtrToNumber",
  "Uint32ToNumber",
  "UnsafeFloat64ToInt32",
  "UnsafeHoleyFloat64ToInt32",
  "UnsafeInt32ToUint32",
  "UnsafeNumberToFloat64",
  "UnsafeSmiTagInt32",
  "UnsafeSmiTagFloat64",
  "UnsafeSmiTagHoleyFloat64",
  "UnsafeSmiTagIntPtr",
  "UnsafeSmiTagUint32",
  "UnsafeSmiUntag",
  "Int32BitwiseAnd",
  "Int32BitwiseOr",
  "Int32BitwiseXor",
  "Int32ShiftLeft",
  "Int32ShiftRight",
  "Int32ShiftRightLogical",
  "Int32AbsWithOverflow",
  "Int32Add",
  "Int32Subtract",
  "Int32Multiply",
  "Int32MultiplyOverflownBits",
  "Int32Divide",
  "Int32Increment",
  "Int32Decrement",
  "Int32AddWithOverflow",
  "Int32SubtractWithOverflow",
  "Int32MultiplyWithOverflow",
  "Int32DivideWithOverflow",
  "Int32ModulusWithOverflow",
  "Int32BitwiseNot",
  "Int32NegateWithOverflow",
  "Int32IncrementWithOverflow",
  "Int32DecrementWithOverflow",
  "Int32Compare",
  "Int32ToBoolean",
  "Float64Abs",
  "Float64Add",
  "Float64SpeculateSafeAdd",
  "Float64Subtract",
  "Float64Multiply",
  "Float64Divide",
  "Float64Exponentiate",
  "Float64Modulus",
  "Float64Negate",
  "Float64Max",
  "Float64Min",
  "Float64RoundToFloat32",
  "Float64Round",
  "Float64Compare",
  "Float64SameValue",
  "Float64ToBoolean",
  "Float64Ieee754Unary",
  "Float64Ieee754Binary",
  "Float64Sqrt",
  "CheckedSmiIncrement",
  "CheckedSmiDecrement",
  "GenericAdd",
  "GenericSubtract",
  "GenericMultiply",
  "GenericDivide",
  "GenericModulus",
  "GenericExponentiate",
  "GenericBitwiseAnd",
  "GenericBitwiseOr",
  "GenericBitwiseXor",
  "GenericShiftLeft",
  "GenericShiftRight",
  "GenericShiftRightLogical",
  "GenericBitwiseNot",
  "GenericNegate",
  "GenericIncrement",
  "GenericDecrement",
  "GenericEqual",
  "GenericStrictEqual",
  "GenericLessThan",
  "GenericLessThanOrEqual",
  "GenericGreaterThan",
  "GenericGreaterThanOrEqual",
  "BigIntBinaryOperation",
  "BigIntCompare",
  "BigIntNegate",
  "BuiltinStringFromCharCode",
  "BuiltinStringPrototypeCharCodeOrCodePointAt",
  "CreateFastArrayElements",
  "NewConsString",
  "MapPrototypeGet",
  "MapPrototypeGetInt32Key",
  "SetPrototypeHas",
  "WeakMapPrototypeGet",
  "ObjectIsArray",
  "ProcessWasmArgument",
  "Abort",
  "Return",
  "Deopt",
  "Throw",
  "Switch",
  "BranchIfSmi",
  "BranchIfRootConstant",
  "BranchIfToBooleanTrue",
  "BranchIfInt32ToBooleanTrue",
  "BranchIfIntPtrToBooleanTrue",
  "BranchIfFloat64ToBooleanTrue",
  "BranchIfHoleyFloat64ToBooleanTrue",
  "BranchIfFloat64IsHole",
  "BranchIfReferenceEqual",
  "BranchIfInt32Compare",
  "BranchIfUint32Compare",
  "BranchIfFloat64Compare",
  "BranchIfUndefinedOrNull",
  "BranchIfUndetectable",
  "BranchIfJSReceiver",
  "BranchIfTypedArrayBounds",
  "Jump",
  "CheckpointedJump",
  "JumpLoop",
];

/// 判别值 → opcode 反查表, 下标即原版枚举序。
pub const OPCODES: [Opcode; OPCODE_COUNT] = [
  Opcode::AssertInt32,
  Opcode::AssertRangeInt32,
  Opcode::AssertRangeFloat64,
  Opcode::CheckDynamicValue,
  Opcode::CheckInt32IsSmi,
  Opcode::CheckUint32IsSmi,
  Opcode::CheckIntPtrIsSmi,
  Opcode::CheckFloat64IsSmi,
  Opcode::CheckHoleyFloat64IsSmi,
  Opcode::CheckHeapObject,
  Opcode::CheckInt32Condition,
  Opcode::CheckCacheIndicesNotCleared,
  Opcode::CheckJSDataViewBounds,
  Opcode::CheckTypedArrayBounds,
  Opcode::CheckTypedArrayValid,
  Opcode::CheckMaps,
  Opcode::CheckHomomorphicMap,
  Opcode::CheckMapsWithMigrationAndDeopt,
  Opcode::CheckMapsWithMigration,
  Opcode::CheckMapsWithAlreadyLoadedMap,
  Opcode::CheckDetectableCallable,
  Opcode::CheckJSReceiverOrNullOrUndefined,
  Opcode::CheckNotHole,
  Opcode::CheckNotUndefined,
  Opcode::CheckHoleyFloat64NotHoleOrUndefined,
  Opcode::CheckNumber,
  Opcode::CheckSmi,
  Opcode::CheckString,
  Opcode::CheckStringOrStringWrapper,
  Opcode::CheckStringOrOddball,
  Opcode::CheckSymbol,
  Opcode::CheckValue,
  Opcode::CheckValueEqualsInt32,
  Opcode::CheckFloat64SameValue,
  Opcode::CheckValueEqualsString,
  Opcode::CheckInstanceType,
  Opcode::Dead,
  Opcode::DebugBreak,
  Opcode::MajorGCForCompilerTesting,
  Opcode::FunctionEntryStackCheck,
  Opcode::GeneratorStore,
  Opcode::StoreMap,
  Opcode::StoreFixedArrayElementWithWriteBarrier,
  Opcode::StoreFixedArrayElementNoWriteBarrier,
  Opcode::StoreFixedDoubleArrayElement,
  Opcode::StoreFixedDoubleArrayHole,
  Opcode::StoreFixedHoleyDoubleArrayElement,
  Opcode::StoreInt32,
  Opcode::StoreFloat64,
  Opcode::StoreIntTypedArrayElement,
  Opcode::StoreDoubleTypedArrayElement,
  Opcode::StoreIntConstantTypedArrayElement,
  Opcode::StoreDoubleConstantTypedArrayElement,
  Opcode::StoreInt32DataViewElement,
  Opcode::StoreDoubleDataViewElement,
  Opcode::StoreTaggedFieldNoWriteBarrier,
  Opcode::StoreTaggedFieldWithWriteBarrier,
  Opcode::StoreSmiContextCell,
  Opcode::StoreInt32ContextCell,
  Opcode::StoreFloat64ContextCell,
  Opcode::StoreContextSlotWithWriteBarrier,
  Opcode::StoreTrustedPointerFieldWithWriteBarrier,
  Opcode::HandleNoHeapWritesInterrupt,
  Opcode::ReduceInterruptBudgetForLoop,
  Opcode::ReduceInterruptBudgetForReturn,
  Opcode::ThrowReferenceErrorIfTdzHole,
  Opcode::ThrowSuperNotCalledIfTdzHole,
  Opcode::ThrowSuperAlreadyCalledIfNotTdzHole,
  Opcode::ThrowIfNotCallable,
  Opcode::ThrowIfNotSuperConstructor,
  Opcode::TransitionElementsKindOrCheckMap,
  Opcode::SetContinuationPreservedEmbedderData,
  Opcode::FulfillPromise,
  Opcode::CheckMaglevType,
  Opcode::Trap,
  Opcode::ConstantGapMove,
  Opcode::GapMove,
  Opcode::TransitionAndStoreArrayElement,
  Opcode::TurbofanStaticAssert,
  Opcode::AssertPeeled,
  Opcode::AssertEscapeAnalysisElided,
  Opcode::AssumeMap,
  Opcode::AssumeType,
  Opcode::Identity,
  Opcode::DeadValue,
  Opcode::AllocationBlock,
  Opcode::ArgumentsElements,
  Opcode::ArgumentsLength,
  Opcode::RestLength,
  Opcode::Call,
  Opcode::CallBuiltin,
  Opcode::CallForwardVarargs,
  Opcode::CallRuntime,
  Opcode::CallWithArrayLike,
  Opcode::CallWithSpread,
  Opcode::CallKnownApiFunction,
  Opcode::CallKnownBuiltin,
  Opcode::CallKnownJSFunction,
  Opcode::CallSelf,
  Opcode::Construct,
  Opcode::ConstructForwardVarargs,
  Opcode::CheckConstructResult,
  Opcode::CheckDerivedConstructResult,
  Opcode::ConstructWithSpread,
  Opcode::ConvertReceiver,
  Opcode::ConvertHoleToUndefined,
  Opcode::CreateArrayLiteral,
  Opcode::CreateShallowArrayLiteral,
  Opcode::CreateObjectLiteral,
  Opcode::CreateShallowObjectLiteral,
  Opcode::CreateFunctionContext,
  Opcode::CreateClosure,
  Opcode::FastCreateClosure,
  Opcode::CreateRegExpLiteral,
  Opcode::DeleteProperty,
  Opcode::EnsureWritableFastElements,
  Opcode::ExtendPropertiesBackingStore,
  Opcode::InlinedAllocation,
  Opcode::ForInPrepare,
  Opcode::ForInNext,
  Opcode::GeneratorRestoreRegister,
  Opcode::GetIterator,
  Opcode::GetSecondReturnedValue,
  Opcode::GetTemplateObject,
  Opcode::HasInPrototypeChain,
  Opcode::InitialValue,
  Opcode::LoadTaggedField,
  Opcode::LoadContextSlotNoCells,
  Opcode::LoadContextSlot,
  Opcode::LoadFloat64,
  Opcode::LoadInt32,
  Opcode::LoadTaggedFieldByFieldIndex,
  Opcode::LoadFixedArrayElement,
  Opcode::LoadFixedDoubleArrayElement,
  Opcode::LoadHoleyFixedDoubleArrayElement,
  Opcode::LoadInt32DataViewElement,
  Opcode::LoadUint32DataViewElement,
  Opcode::LoadDoubleDataViewElement,
  Opcode::LoadTypedArrayLength,
  Opcode::LoadDataViewByteLength,
  Opcode::LoadDataViewDataPointer,
  Opcode::LoadSignedIntTypedArrayElement,
  Opcode::LoadUnsignedIntTypedArrayElement,
  Opcode::LoadDoubleTypedArrayElement,
  Opcode::LoadSignedIntConstantTypedArrayElement,
  Opcode::LoadUnsignedIntConstantTypedArrayElement,
  Opcode::LoadDoubleConstantTypedArrayElement,
  Opcode::LoadEnumCacheLength,
  Opcode::LoadGlobal,
  Opcode::LoadNamedGeneric,
  Opcode::LoadNamedFromSuperGeneric,
  Opcode::LoadDictionaryField,
  Opcode::MaybeGrowFastElements,
  Opcode::MigrateMapIfNeeded,
  Opcode::SetNamedGeneric,
  Opcode::DefineNamedOwnGeneric,
  Opcode::StoreInArrayLiteralGeneric,
  Opcode::StoreGlobal,
  Opcode::GetKeyedGeneric,
  Opcode::SetKeyedGeneric,
  Opcode::DefineKeyedOwnGeneric,
  Opcode::Phi,
  Opcode::RegisterInput,
  Opcode::CheckedInternalizedString,
  Opcode::TruncateCheckedNumberAsSafeIntToInt32,
  Opcode::TruncateUnsafeNumberAsSafeIntToInt32,
  Opcode::TruncateFloat64AsSafeIntToInt32,
  Opcode::TruncateHoleyFloat64AsSafeIntToInt32,
  Opcode::TruncateCheckedNumberOrOddballToInt32,
  Opcode::TruncateUnsafeNumberOrOddballToInt32,
  Opcode::TruncateFloat64ToInt32,
  Opcode::TruncateHoleyFloat64ToInt32,
  Opcode::TruncateUint32ToInt32,
  Opcode::Int32ToUint8Clamped,
  Opcode::Uint32ToUint8Clamped,
  Opcode::Float64ToUint8Clamped,
  Opcode::CheckedNumberOrOddballToUint8Clamped,
  Opcode::Int32CountLeadingZeros,
  Opcode::TaggedCountLeadingZeros,
  Opcode::Float64CountLeadingZeros,
  Opcode::IntPtrToBoolean,
  Opcode::CheckedNumberOrOddballToFloat64,
  Opcode::CheckedNumberOrOddballToHoleyFloat64,
  Opcode::UnsafeNumberOrOddballToFloat64,
  Opcode::UnsafeNumberOrOddballToHoleyFloat64,
  Opcode::UnsafeHoleyFloat64ToFloat64,
  Opcode::UnsafeFloat64ToHoleyFloat64,
  Opcode::Float64ToSilencedFloat64,
  Opcode::HoleyFloat64IsHole,
  Opcode::LogicalNot,
  Opcode::SetPendingMessage,
  Opcode::StringAt,
  Opcode::StringEqual,
  Opcode::StringLength,
  Opcode::StringConcat,
  Opcode::StringIndexOf,
  Opcode::ConsStringMap,
  Opcode::UnwrapStringWrapper,
  Opcode::ToBoolean,
  Opcode::ToBooleanLogicalNot,
  Opcode::AllocateElementsArray,
  Opcode::TaggedEqual,
  Opcode::TaggedNotEqual,
  Opcode::TestInstanceOf,
  Opcode::TestUndetectable,
  Opcode::TestTypeOf,
  Opcode::ToName,
  Opcode::ToNumberOrNumeric,
  Opcode::ToObject,
  Opcode::ToString,
  Opcode::TransitionElementsKind,
  Opcode::Int32ToString,
  Opcode::Float64ToString,
  Opcode::SmiToString,
  Opcode::NumberToString,
  Opcode::UpdateJSArrayLength,
  Opcode::VirtualObject,
  Opcode::GetContinuationPreservedEmbedderData,
  Opcode::ReturnedValue,
  Opcode::StringSlice,
  Opcode::StringSubstring,
  Opcode::HeapConstant,
  Opcode::Float64Constant,
  Opcode::HoleyFloat64Constant,
  Opcode::Int32Constant,
  Opcode::Uint32Constant,
  Opcode::IntPtrConstant,
  Opcode::RootConstant,
  Opcode::SmiConstant,
  Opcode::TaggedIndexConstant,
  Opcode::TrustedConstant,
  Opcode::ChangeInt32ToFloat64,
  Opcode::ChangeInt32ToHoleyFloat64,
  Opcode::ChangeIntPtrToFloat64,
  Opcode::ChangeUint32ToFloat64,
  Opcode::ChangeUint32ToHoleyFloat64,
  Opcode::ChangeFloat64ToHoleyFloat64,
  Opcode::CheckedFloat64ToInt32,
  Opcode::CheckedFloat64ToUint32,
  Opcode::CheckedFloat64ToSmiSizedInt32,
  Opcode::CheckedHoleyFloat64ToSmiSizedInt32,
  Opcode::CheckedHoleyFloat64ToInt32,
  Opcode::CheckedHoleyFloat64ToUint32,
  Opcode::CheckedHoleyFloat64ToFloat64,
  Opcode::CheckedInt32ToUint32,
  Opcode::CheckedIntPtrToInt32,
  Opcode::CheckedIntPtrToUint32,
  Opcode::CheckedObjectToIndex,
  Opcode::CheckedNumberToInt32,
  Opcode::CheckedNumberToFloat64,
  Opcode::CheckedSmiSizedInt32,
  Opcode::CheckedSmiTagFloat64,
  Opcode::CheckedSmiTagHoleyFloat64,
  Opcode::CheckedSmiTagInt32,
  Opcode::CheckedSmiTagIntPtr,
  Opcode::CheckedSmiTagUint32,
  Opcode::CheckedSmiUntag,
  Opcode::CheckedUint32ToInt32,
  Opcode::Float64ToTagged,
  Opcode::HoleyFloat64ToTagged,
  Opcode::Int32ToNumber,
  Opcode::IntPtrToNumber,
  Opcode::Uint32ToNumber,
  Opcode::UnsafeFloat64ToInt32,
  Opcode::UnsafeHoleyFloat64ToInt32,
  Opcode::UnsafeInt32ToUint32,
  Opcode::UnsafeNumberToFloat64,
  Opcode::UnsafeSmiTagInt32,
  Opcode::UnsafeSmiTagFloat64,
  Opcode::UnsafeSmiTagHoleyFloat64,
  Opcode::UnsafeSmiTagIntPtr,
  Opcode::UnsafeSmiTagUint32,
  Opcode::UnsafeSmiUntag,
  Opcode::Int32BitwiseAnd,
  Opcode::Int32BitwiseOr,
  Opcode::Int32BitwiseXor,
  Opcode::Int32ShiftLeft,
  Opcode::Int32ShiftRight,
  Opcode::Int32ShiftRightLogical,
  Opcode::Int32AbsWithOverflow,
  Opcode::Int32Add,
  Opcode::Int32Subtract,
  Opcode::Int32Multiply,
  Opcode::Int32MultiplyOverflownBits,
  Opcode::Int32Divide,
  Opcode::Int32Increment,
  Opcode::Int32Decrement,
  Opcode::Int32AddWithOverflow,
  Opcode::Int32SubtractWithOverflow,
  Opcode::Int32MultiplyWithOverflow,
  Opcode::Int32DivideWithOverflow,
  Opcode::Int32ModulusWithOverflow,
  Opcode::Int32BitwiseNot,
  Opcode::Int32NegateWithOverflow,
  Opcode::Int32IncrementWithOverflow,
  Opcode::Int32DecrementWithOverflow,
  Opcode::Int32Compare,
  Opcode::Int32ToBoolean,
  Opcode::Float64Abs,
  Opcode::Float64Add,
  Opcode::Float64SpeculateSafeAdd,
  Opcode::Float64Subtract,
  Opcode::Float64Multiply,
  Opcode::Float64Divide,
  Opcode::Float64Exponentiate,
  Opcode::Float64Modulus,
  Opcode::Float64Negate,
  Opcode::Float64Max,
  Opcode::Float64Min,
  Opcode::Float64RoundToFloat32,
  Opcode::Float64Round,
  Opcode::Float64Compare,
  Opcode::Float64SameValue,
  Opcode::Float64ToBoolean,
  Opcode::Float64Ieee754Unary,
  Opcode::Float64Ieee754Binary,
  Opcode::Float64Sqrt,
  Opcode::CheckedSmiIncrement,
  Opcode::CheckedSmiDecrement,
  Opcode::GenericAdd,
  Opcode::GenericSubtract,
  Opcode::GenericMultiply,
  Opcode::GenericDivide,
  Opcode::GenericModulus,
  Opcode::GenericExponentiate,
  Opcode::GenericBitwiseAnd,
  Opcode::GenericBitwiseOr,
  Opcode::GenericBitwiseXor,
  Opcode::GenericShiftLeft,
  Opcode::GenericShiftRight,
  Opcode::GenericShiftRightLogical,
  Opcode::GenericBitwiseNot,
  Opcode::GenericNegate,
  Opcode::GenericIncrement,
  Opcode::GenericDecrement,
  Opcode::GenericEqual,
  Opcode::GenericStrictEqual,
  Opcode::GenericLessThan,
  Opcode::GenericLessThanOrEqual,
  Opcode::GenericGreaterThan,
  Opcode::GenericGreaterThanOrEqual,
  Opcode::BigIntBinaryOperation,
  Opcode::BigIntCompare,
  Opcode::BigIntNegate,
  Opcode::BuiltinStringFromCharCode,
  Opcode::BuiltinStringPrototypeCharCodeOrCodePointAt,
  Opcode::CreateFastArrayElements,
  Opcode::NewConsString,
  Opcode::MapPrototypeGet,
  Opcode::MapPrototypeGetInt32Key,
  Opcode::SetPrototypeHas,
  Opcode::WeakMapPrototypeGet,
  Opcode::ObjectIsArray,
  Opcode::ProcessWasmArgument,
  Opcode::Abort,
  Opcode::Return,
  Opcode::Deopt,
  Opcode::Throw,
  Opcode::Switch,
  Opcode::BranchIfSmi,
  Opcode::BranchIfRootConstant,
  Opcode::BranchIfToBooleanTrue,
  Opcode::BranchIfInt32ToBooleanTrue,
  Opcode::BranchIfIntPtrToBooleanTrue,
  Opcode::BranchIfFloat64ToBooleanTrue,
  Opcode::BranchIfHoleyFloat64ToBooleanTrue,
  Opcode::BranchIfFloat64IsHole,
  Opcode::BranchIfReferenceEqual,
  Opcode::BranchIfInt32Compare,
  Opcode::BranchIfUint32Compare,
  Opcode::BranchIfFloat64Compare,
  Opcode::BranchIfUndefinedOrNull,
  Opcode::BranchIfUndetectable,
  Opcode::BranchIfJSReceiver,
  Opcode::BranchIfTypedArrayBounds,
  Opcode::Jump,
  Opcode::CheckpointedJump,
  Opcode::JumpLoop,
];

impl Opcode {
  /// 判别值 (= 原版 `static_cast<uint16_t>(opcode)`)。
  pub const fn to_u16(self) -> u16 {
    self as u16
  }

  /// 由判别值还原 opcode (越界返回 None, 对应原版 `OpcodeField::is_valid` 域)。
  pub const fn from_u16(raw: u16) -> Option<Self> {
    let idx = raw as usize;
    if idx < OPCODE_COUNT {
      Some(OPCODES[idx])
    } else {
      None
    }
  }

  /// 原版宏拼写 `k##Name` 去 k 前缀后的 C++ 类名。
  pub const fn cxx_name(self) -> &'static str {
    let idx = self as usize;
    if idx < OPCODE_COUNT {
      OPCODE_NAMES[idx]
    } else {
      "?"
    }
  }
}

// —— 分组区间常量: 复刻 `std::min({LIST(V) kLastOpcode})` / `std::max({LIST(V) kFirstOpcode})` ——

/// VALUE_NODE_LIST (271 项)
pub const FIRST_VALUE_NODE_OPCODE: Opcode = Opcode::Identity;
pub const LAST_VALUE_NODE_OPCODE: Opcode = Opcode::ProcessWasmArgument;
const VALUE_NODE_FIRST_RAW: u16 = 83;
const VALUE_NODE_LAST_RAW: u16 = 353;

/// CONSTANT_VALUE_NODE_LIST (10 项)
pub const FIRST_CONSTANT_NODE_OPCODE: Opcode = Opcode::HeapConstant;
pub const LAST_CONSTANT_NODE_OPCODE: Opcode = Opcode::TrustedConstant;
const CONSTANT_NODE_FIRST_RAW: u16 = 221;
const CONSTANT_NODE_LAST_RAW: u16 = 230;

/// GAP_MOVE_NODE_LIST (2 项)
pub const FIRST_GAP_MOVE_NODE_OPCODE: Opcode = Opcode::ConstantGapMove;
pub const LAST_GAP_MOVE_NODE_OPCODE: Opcode = Opcode::GapMove;
const GAP_MOVE_NODE_FIRST_RAW: u16 = 75;
const GAP_MOVE_NODE_LAST_RAW: u16 = 76;

/// CONVERSION_NODE_LIST (42 项)
pub const FIRST_CONVERSION_NODE_OPCODE: Opcode = Opcode::ChangeInt32ToFloat64;
pub const LAST_CONVERSION_NODE_OPCODE: Opcode = Opcode::UnsafeSmiUntag;
const CONVERSION_NODE_FIRST_RAW: u16 = 231;
const CONVERSION_NODE_LAST_RAW: u16 = 272;

/// NODE_LIST (354 项)
pub const FIRST_NODE_OPCODE: Opcode = Opcode::AssertInt32;
pub const LAST_NODE_OPCODE: Opcode = Opcode::ProcessWasmArgument;
const NODE_FIRST_RAW: u16 = 0;
const NODE_LAST_RAW: u16 = 353;

/// BRANCH_CONTROL_NODE_LIST (16 项)
pub const FIRST_BRANCH_CONTROL_NODE_OPCODE: Opcode = Opcode::BranchIfSmi;
pub const LAST_BRANCH_CONTROL_NODE_OPCODE: Opcode = Opcode::BranchIfTypedArrayBounds;
const BRANCH_CONTROL_NODE_FIRST_RAW: u16 = 359;
const BRANCH_CONTROL_NODE_LAST_RAW: u16 = 374;

/// CONDITIONAL_CONTROL_NODE_LIST (17 项)
pub const FIRST_CONDITIONAL_CONTROL_NODE_OPCODE: Opcode = Opcode::Switch;
pub const LAST_CONDITIONAL_CONTROL_NODE_OPCODE: Opcode = Opcode::BranchIfTypedArrayBounds;
const CONDITIONAL_CONTROL_NODE_FIRST_RAW: u16 = 358;
const CONDITIONAL_CONTROL_NODE_LAST_RAW: u16 = 374;

/// UNCONDITIONAL_CONTROL_NODE_LIST (3 项)
pub const FIRST_UNCONDITIONAL_CONTROL_NODE_OPCODE: Opcode = Opcode::Jump;
pub const LAST_UNCONDITIONAL_CONTROL_NODE_OPCODE: Opcode = Opcode::JumpLoop;
const UNCONDITIONAL_CONTROL_NODE_FIRST_RAW: u16 = 375;
const UNCONDITIONAL_CONTROL_NODE_LAST_RAW: u16 = 377;

/// TERMINAL_CONTROL_NODE_LIST (4 项)
pub const FIRST_TERMINAL_CONTROL_NODE_OPCODE: Opcode = Opcode::Abort;
pub const LAST_TERMINAL_CONTROL_NODE_OPCODE: Opcode = Opcode::Throw;
const TERMINAL_CONTROL_NODE_FIRST_RAW: u16 = 354;
const TERMINAL_CONTROL_NODE_LAST_RAW: u16 = 357;

/// CONTROL_NODE_LIST (24 项)
pub const FIRST_CONTROL_NODE_OPCODE: Opcode = Opcode::Abort;
pub const LAST_CONTROL_NODE_OPCODE: Opcode = Opcode::JumpLoop;
const CONTROL_NODE_FIRST_RAW: u16 = 354;
const CONTROL_NODE_LAST_RAW: u16 = 377;

// —— 区间谓词 (原版 `constexpr bool IsXxxNode(Opcode)`) ——
/// 原版 `constexpr bool IsValueNode(Opcode)`.
pub const fn is_value_node(opcode: Opcode) -> bool {
  in_range(opcode, VALUE_NODE_FIRST_RAW, VALUE_NODE_LAST_RAW)
}

/// 原版 `constexpr bool IsConstantNode(Opcode)`.
pub const fn is_constant_node(opcode: Opcode) -> bool {
  in_range(opcode, CONSTANT_NODE_FIRST_RAW, CONSTANT_NODE_LAST_RAW)
}

/// 原版 `constexpr bool IsGapMoveNode(Opcode)`.
pub const fn is_gap_move_node(opcode: Opcode) -> bool {
  in_range(opcode, GAP_MOVE_NODE_FIRST_RAW, GAP_MOVE_NODE_LAST_RAW)
}

/// 原版 `constexpr bool IsConversionNode(Opcode)`.
pub const fn is_conversion_node(opcode: Opcode) -> bool {
  in_range(opcode, CONVERSION_NODE_FIRST_RAW, CONVERSION_NODE_LAST_RAW)
}

/// 原版 `constexpr bool IsNode(Opcode)`.
pub const fn is_node(opcode: Opcode) -> bool {
  in_range(opcode, NODE_FIRST_RAW, NODE_LAST_RAW)
}

/// 原版 `constexpr bool IsBranchControlNode(Opcode)`.
pub const fn is_branch_control_node(opcode: Opcode) -> bool {
  in_range(
    opcode,
    BRANCH_CONTROL_NODE_FIRST_RAW,
    BRANCH_CONTROL_NODE_LAST_RAW,
  )
}

/// 原版 `constexpr bool IsConditionalControlNode(Opcode)`.
pub const fn is_conditional_control_node(opcode: Opcode) -> bool {
  in_range(
    opcode,
    CONDITIONAL_CONTROL_NODE_FIRST_RAW,
    CONDITIONAL_CONTROL_NODE_LAST_RAW,
  )
}

/// 原版 `constexpr bool IsUnconditionalControlNode(Opcode)`.
pub const fn is_unconditional_control_node(opcode: Opcode) -> bool {
  in_range(
    opcode,
    UNCONDITIONAL_CONTROL_NODE_FIRST_RAW,
    UNCONDITIONAL_CONTROL_NODE_LAST_RAW,
  )
}

/// 原版 `constexpr bool IsTerminalControlNode(Opcode)`.
pub const fn is_terminal_control_node(opcode: Opcode) -> bool {
  in_range(
    opcode,
    TERMINAL_CONTROL_NODE_FIRST_RAW,
    TERMINAL_CONTROL_NODE_LAST_RAW,
  )
}

/// 原版 `constexpr bool IsControlNode(Opcode)`.
pub const fn is_control_node(opcode: Opcode) -> bool {
  in_range(opcode, CONTROL_NODE_FIRST_RAW, CONTROL_NODE_LAST_RAW)
}

/// 闭区间判定, 等价原版 `FIRST <= opcode && opcode <= LAST`。
const fn in_range(opcode: Opcode, first: u16, last: u16) -> bool {
  let v = opcode.to_u16();
  v >= first && v <= last
}

// —— opcode 集合谓词 (原版 switch(case)/等式链 白名单枚举) ——
/// 原版 `constexpr bool IsInt32BitwiseBinaryOperationNode(Opcode)` — 6 项。
pub const fn is_int32_bitwise_binary_operation_node(opcode: Opcode) -> bool {
  matches!(
    opcode,
    Opcode::Int32BitwiseAnd
      | Opcode::Int32BitwiseOr
      | Opcode::Int32BitwiseXor
      | Opcode::Int32ShiftLeft
      | Opcode::Int32ShiftRight
      | Opcode::Int32ShiftRightLogical
  )
}

/// 原版 `constexpr bool IsCommutativeNode(Opcode)` — 17 项。
pub const fn is_commutative_node(opcode: Opcode) -> bool {
  matches!(
    opcode,
    Opcode::Float64Add
      | Opcode::Float64SpeculateSafeAdd
      | Opcode::Float64Min
      | Opcode::Float64Max
      | Opcode::Float64Multiply
      | Opcode::GenericStrictEqual
      | Opcode::Int32Add
      | Opcode::Int32AddWithOverflow
      | Opcode::Int32BitwiseAnd
      | Opcode::Int32BitwiseOr
      | Opcode::Int32BitwiseXor
      | Opcode::Int32Multiply
      | Opcode::Int32MultiplyOverflownBits
      | Opcode::Int32MultiplyWithOverflow
      | Opcode::StringEqual
      | Opcode::TaggedEqual
      | Opcode::TaggedNotEqual
  )
}

/// 原版 `constexpr bool IsZeroCostNode(Opcode)` — 4 项。
pub const fn is_zero_cost_node(opcode: Opcode) -> bool {
  matches!(
    opcode,
    Opcode::TruncateUint32ToInt32
      | Opcode::UnsafeFloat64ToHoleyFloat64
      | Opcode::UnsafeHoleyFloat64ToFloat64
      | Opcode::Identity
  )
}

/// 原版 `constexpr bool IsTruncatingToInt32(Opcode)` — 9 项。
pub const fn is_truncating_to_int32(opcode: Opcode) -> bool {
  matches!(
    opcode,
    Opcode::TruncateUint32ToInt32
      | Opcode::TruncateFloat64ToInt32
      | Opcode::TruncateHoleyFloat64ToInt32
      | Opcode::TruncateCheckedNumberAsSafeIntToInt32
      | Opcode::TruncateUnsafeNumberAsSafeIntToInt32
      | Opcode::TruncateFloat64AsSafeIntToInt32
      | Opcode::TruncateHoleyFloat64AsSafeIntToInt32
      | Opcode::TruncateCheckedNumberOrOddballToInt32
      | Opcode::TruncateUnsafeNumberOrOddballToInt32
  )
}

/// 原版 `constexpr bool CanBeStoreToNonEscapedObject(Opcode)` — 7 项。
pub const fn can_be_store_to_non_escaped_object(opcode: Opcode) -> bool {
  matches!(
    opcode,
    Opcode::StoreMap
      | Opcode::StoreInt32
      | Opcode::StoreTrustedPointerFieldWithWriteBarrier
      | Opcode::StoreTaggedFieldWithWriteBarrier
      | Opcode::StoreTaggedFieldNoWriteBarrier
      | Opcode::StoreContextSlotWithWriteBarrier
      | Opcode::StoreFloat64
  )
}

/// 原版 `constexpr bool HasRangeField(Opcode)` — 5 项。
pub const fn has_range_field(opcode: Opcode) -> bool {
  matches!(
    opcode,
    Opcode::Float64Add
      | Opcode::Float64SpeculateSafeAdd
      | Opcode::Float64Subtract
      | Opcode::Float64Multiply
      | Opcode::Float64Divide
  )
}

/// 原版 `constexpr bool IsSimpleFieldStore(Opcode)` — 15 项。
pub const fn is_simple_field_store(opcode: Opcode) -> bool {
  matches!(
    opcode,
    Opcode::StoreTaggedFieldWithWriteBarrier
      | Opcode::StoreTaggedFieldNoWriteBarrier
      | Opcode::StoreFloat64
      | Opcode::StoreInt32
      | Opcode::UpdateJSArrayLength
      | Opcode::StoreFixedArrayElementWithWriteBarrier
      | Opcode::StoreFixedArrayElementNoWriteBarrier
      | Opcode::StoreFixedDoubleArrayElement
      | Opcode::StoreFixedDoubleArrayHole
      | Opcode::StoreFixedHoleyDoubleArrayElement
      | Opcode::StoreTrustedPointerFieldWithWriteBarrier
      | Opcode::StoreContextSlotWithWriteBarrier
      | Opcode::StoreSmiContextCell
      | Opcode::StoreInt32ContextCell
      | Opcode::StoreFloat64ContextCell
  )
}

/// 原版 `constexpr bool IsElementsArrayWrite(Opcode)` — 2 项。
pub const fn is_elements_array_write(opcode: Opcode) -> bool {
  matches!(
    opcode,
    Opcode::MaybeGrowFastElements | Opcode::EnsureWritableFastElements
  )
}

/// 原版 `constexpr bool PreservesTaggedKeyedProperties(Opcode)` — 5 项。
pub const fn preserves_tagged_keyed_properties(opcode: Opcode) -> bool {
  matches!(
    opcode,
    Opcode::StoreFixedDoubleArrayElement
      | Opcode::StoreFixedDoubleArrayHole
      | Opcode::StoreFixedHoleyDoubleArrayElement
      | Opcode::TransitionElementsKind
      | Opcode::TransitionElementsKindOrCheckMap
  )
}

/// 原版 `constexpr bool IsTypedArrayStore(Opcode)` — 6 项。
pub const fn is_typed_array_store(opcode: Opcode) -> bool {
  matches!(
    opcode,
    Opcode::StoreIntTypedArrayElement
      | Opcode::StoreDoubleTypedArrayElement
      | Opcode::StoreIntConstantTypedArrayElement
      | Opcode::StoreDoubleConstantTypedArrayElement
      | Opcode::StoreInt32DataViewElement
      | Opcode::StoreDoubleDataViewElement
  )
}

/// 原版 `constexpr bool CanTriggerTruncationPass(Opcode)`: 先查 int32 位运算集合, 再查表。
pub const fn can_trigger_truncation_pass(opcode: Opcode) -> bool {
  is_int32_bitwise_binary_operation_node(opcode)
    || matches!(
      opcode,
      Opcode::TruncateFloat64ToInt32
        | Opcode::TruncateHoleyFloat64ToInt32
        | Opcode::CheckedFloat64ToInt32
        | Opcode::CheckedHoleyFloat64ToInt32
        | Opcode::UnsafeFloat64ToInt32
        | Opcode::UnsafeHoleyFloat64ToInt32
    )
}

/// 原版 `RootIndex` 的 hole 二态 (零耦合本地建模, 对应 kTheHoleValue / kTdzHoleValue)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoleIndex {
  /// RootIndex::kTheHoleValue
  TheHole,
  /// RootIndex::kTdzHoleValue
  TdzHole,
}

/// 原版 `constexpr bool CanBeHoleValue(RootIndex, Opcode)` — 11 项分三桶。
pub const fn can_be_hole_value(hole_index: HoleIndex, opcode: Opcode) -> bool {
  match opcode {
    Opcode::CallBuiltin
    | Opcode::Identity
    | Opcode::LoadTaggedField
    | Opcode::Phi
    | Opcode::RootConstant => true,
    Opcode::CallRuntime | Opcode::LoadFixedArrayElement => matches!(hole_index, HoleIndex::TheHole),
    Opcode::GeneratorRestoreRegister
    | Opcode::InitialValue
    | Opcode::LoadContextSlot
    | Opcode::LoadContextSlotNoCells => matches!(hole_index, HoleIndex::TdzHole),
    _ => false,
  }
}

// 对数: 宏 21 + 各宏展开计数全锚 + 非目标配置 3 锚 + 区间组 10 +
//   switch 集合 6 + 等式链集合 4 + hole 三桶(5/2/4) = 全过; 缺项即 throw。
