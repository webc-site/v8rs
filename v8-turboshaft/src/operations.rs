// 由 tool/src/compiler/turboshaft/operations.h.js 从 v8/src/compiler/turboshaft/operations.{h,cc} 生成; 修订就地改, 复跑覆盖
// Turboshaft 操作码表: Opcode 枚举/块终结符前缀/MayThrow 集/各 Op::Kind 子枚举

// 目标配置: WASM=1, SIMD128=1, SIMD256=0 (arm64 无 SIMD256), CPED=1
// kOperationSizeTable / kOperationEffectsTable 依赖 C++ sizeof 与 OpEffects
// 构造, 不可机械展开 — skip 登记, Developer 按 struct 布局补。

/// 原版 enum class Opcode : uint8_t — 操作码 (块终结符占前缀)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Opcode {
  /// kCheckException
  CheckException = 0,
  /// kGoto
  Goto = 1,
  /// kTailCall
  TailCall = 2,
  /// kUnreachable
  Unreachable = 3,
  /// kReturn
  Return = 4,
  /// kBranch
  Branch = 5,
  /// kSwitch
  Switch = 6,
  /// kDeoptimize
  Deoptimize = 7,
  /// kWasmTrap
  WasmTrap = 8,
  /// kWasmStackCheck
  WasmStackCheck = 9,
  /// kWasmIncCoverageCounter
  WasmIncCoverageCounter = 10,
  /// kGlobalGet
  GlobalGet = 11,
  /// kGlobalSet
  GlobalSet = 12,
  /// kRootConstant
  RootConstant = 13,
  /// kIsRootConstant
  IsRootConstant = 14,
  /// kNull
  Null = 15,
  /// kIsNull
  IsNull = 16,
  /// kAssertNotNull
  AssertNotNull = 17,
  /// kRttCanon
  RttCanon = 18,
  /// kWasmTypeCheck
  WasmTypeCheck = 19,
  /// kWasmTypeCast
  WasmTypeCast = 20,
  /// kAnyConvertExtern
  AnyConvertExtern = 21,
  /// kExternConvertAny
  ExternConvertAny = 22,
  /// kWasmTypeAnnotation
  WasmTypeAnnotation = 23,
  /// kStructGet
  StructGet = 24,
  /// kStructSet
  StructSet = 25,
  /// kStructAtomicRMW
  StructAtomicRMW = 26,
  /// kArrayGet
  ArrayGet = 27,
  /// kArraySet
  ArraySet = 28,
  /// kArrayAtomicRMW
  ArrayAtomicRMW = 29,
  /// kArrayLength
  ArrayLength = 30,
  /// kWasmAllocateArray
  WasmAllocateArray = 31,
  /// kWasmAllocateStruct
  WasmAllocateStruct = 32,
  /// kWasmRefFunc
  WasmRefFunc = 33,
  /// kStringAsWtf16
  StringAsWtf16 = 34,
  /// kStringPrepareForGetCodeUnit
  StringPrepareForGetCodeUnit = 35,
  /// kProcessWasmArgument
  ProcessWasmArgument = 36,
  /// kSimd128Constant
  Simd128Constant = 37,
  /// kSimd128Binop
  Simd128Binop = 38,
  /// kSimd128Unary
  Simd128Unary = 39,
  /// kSimd128Reduce
  Simd128Reduce = 40,
  /// kSimd128Shift
  Simd128Shift = 41,
  /// kSimd128Test
  Simd128Test = 42,
  /// kSimd128Splat
  Simd128Splat = 43,
  /// kSimd128Ternary
  Simd128Ternary = 44,
  /// kSimd128ExtractLane
  Simd128ExtractLane = 45,
  /// kSimd128ReplaceLane
  Simd128ReplaceLane = 46,
  /// kSimd128MoveLane
  Simd128MoveLane = 47,
  /// kSimd128LaneMemory
  Simd128LaneMemory = 48,
  /// kSimd128LoadTransform
  Simd128LoadTransform = 49,
  /// kSimd128Shuffle
  Simd128Shuffle = 50,
  /// kSimd128LoadPairDeinterleave
  Simd128LoadPairDeinterleave = 51,
  /// kWordBinop
  WordBinop = 52,
  /// kFloatBinop
  FloatBinop = 53,
  /// kWord32PairBinop
  Word32PairBinop = 54,
  /// kWord64AddSub128Binop
  Word64AddSub128Binop = 55,
  /// kWord64Add3
  Word64Add3 = 56,
  /// kWord64MulWide
  Word64MulWide = 57,
  /// kOverflowCheckedBinop
  OverflowCheckedBinop = 58,
  /// kWordUnary
  WordUnary = 59,
  /// kOverflowCheckedUnary
  OverflowCheckedUnary = 60,
  /// kFloatUnary
  FloatUnary = 61,
  /// kShift
  Shift = 62,
  /// kComparison
  Comparison = 63,
  /// kChange
  Change = 64,
  /// kTryChange
  TryChange = 65,
  /// kBitcastWord32PairToFloat64
  BitcastWord32PairToFloat64 = 66,
  /// kTaggedBitcast
  TaggedBitcast = 67,
  /// kSelect
  Select = 68,
  /// kPendingLoopPhi
  PendingLoopPhi = 69,
  /// kConstant
  Constant = 70,
  /// kLoadRootRegister
  LoadRootRegister = 71,
  /// kLoad
  Load = 72,
  /// kStore
  Store = 73,
  /// kRetain
  Retain = 74,
  /// kParameter
  Parameter = 75,
  /// kOsrValue
  OsrValue = 76,
  /// kStackPointerGreaterThan
  StackPointerGreaterThan = 77,
  /// kStackSlot
  StackSlot = 78,
  /// kFrameConstant
  FrameConstant = 79,
  /// kDeoptimizeIf
  DeoptimizeIf = 80,
  /// kPhi
  Phi = 81,
  /// kFrameState
  FrameState = 82,
  /// kCall
  Call = 83,
  /// kCatchBlockBegin
  CatchBlockBegin = 84,
  /// kDidntThrow
  DidntThrow = 85,
  /// kMakeTuple
  MakeTuple = 86,
  /// kProjection
  Projection = 87,
  /// kDebugBreak
  DebugBreak = 88,
  /// kAssumeMap
  AssumeMap = 89,
  /// kAtomicRMW
  AtomicRMW = 90,
  /// kAtomicWord32Pair
  AtomicWord32Pair = 91,
  /// kMemoryBarrier
  MemoryBarrier = 92,
  /// kComment
  Comment = 93,
  /// kDead
  Dead = 94,
  /// kAbortCSADcheck
  AbortCSADcheck = 95,
  /// kPause
  Pause = 96,
  /// kLoadTrustedPointer
  LoadTrustedPointer = 97,
  /// kTrapIf
  TrapIf = 98,
  /// kLoadStackPointer
  LoadStackPointer = 99,
  /// kSetStackPointer
  SetStackPointer = 100,
  /// kMemoryCopy
  MemoryCopy = 101,
  /// kMemoryFill
  MemoryFill = 102,
  /// kWasmFXArgBuffer
  WasmFXArgBuffer = 103,
  /// kArgumentsLength
  ArgumentsLength = 104,
  /// kBigIntBinop
  BigIntBinop = 105,
  /// kBigIntComparison
  BigIntComparison = 106,
  /// kBigIntUnary
  BigIntUnary = 107,
  /// kCheckedClosure
  CheckedClosure = 108,
  /// kWordBinopDeoptOnOverflow
  WordBinopDeoptOnOverflow = 109,
  /// kCheckEqualsInternalizedString
  CheckEqualsInternalizedString = 110,
  /// kCheckHomomorphic
  CheckHomomorphic = 111,
  /// kCheckMaps
  CheckMaps = 112,
  /// kCompareMaps
  CompareMaps = 113,
  /// kFloatIs
  FloatIs = 114,
  /// kObjectIs
  ObjectIs = 115,
  /// kObjectIsNumericValue
  ObjectIsNumericValue = 116,
  /// kFloat64SameValue
  Float64SameValue = 117,
  /// kSameValue
  SameValue = 118,
  /// kChangeOrDeopt
  ChangeOrDeopt = 119,
  /// kConvert
  Convert = 120,
  /// kConvertJSPrimitiveToObject
  ConvertJSPrimitiveToObject = 121,
  /// kConvertJSPrimitiveToUntagged
  ConvertJSPrimitiveToUntagged = 122,
  /// kConvertJSPrimitiveToUntaggedOrDeopt
  ConvertJSPrimitiveToUntaggedOrDeopt = 123,
  /// kConvertUntaggedToJSPrimitive
  ConvertUntaggedToJSPrimitive = 124,
  /// kConvertWordToSmiOrDeopt
  ConvertWordToSmiOrDeopt = 125,
  /// kTruncateJSPrimitiveToUntagged
  TruncateJSPrimitiveToUntagged = 126,
  /// kTruncateJSPrimitiveToWord32OrDeopt
  TruncateJSPrimitiveToWord32OrDeopt = 127,
  /// kDoubleArrayMinMax
  DoubleArrayMinMax = 128,
  /// kEnsureWritableFastElements
  EnsureWritableFastElements = 129,
  /// kFastApiCall
  FastApiCall = 130,
  /// kFindOrderedHashEntry
  FindOrderedHashEntry = 131,
  /// kWeakCollectionGet
  WeakCollectionGet = 132,
  /// kLoadDataViewElement
  LoadDataViewElement = 133,
  /// kLoadFieldByIndex
  LoadFieldByIndex = 134,
  /// kLoadMessage
  LoadMessage = 135,
  /// kLoadStackArgument
  LoadStackArgument = 136,
  /// kLoadTypedElement
  LoadTypedElement = 137,
  /// kStoreDataViewElement
  StoreDataViewElement = 138,
  /// kStoreMessage
  StoreMessage = 139,
  /// kStoreTypedElement
  StoreTypedElement = 140,
  /// kMaybeGrowFastElements
  MaybeGrowFastElements = 141,
  /// kNewArgumentsElements
  NewArgumentsElements = 142,
  /// kNewArray
  NewArray = 143,
  /// kRuntimeAbort
  RuntimeAbort = 144,
  /// kStaticAssert
  StaticAssert = 145,
  /// kStringAt
  StringAt = 146,
  /// kStringComparison
  StringComparison = 147,
  /// kStringConcat
  StringConcat = 148,
  /// kStringFromCodePointAt
  StringFromCodePointAt = 149,
  /// kStringIndexOf
  StringIndexOf = 150,
  /// kStringLength
  StringLength = 151,
  /// kStringOrOddballStrictEqual
  StringOrOddballStrictEqual = 152,
  /// kTypedArrayLength
  TypedArrayLength = 153,
  /// kStringSubstring
  StringSubstring = 154,
  /// kStringSlice
  StringSlice = 155,
  /// kNewConsString
  NewConsString = 156,
  /// kTransitionAndStoreArrayElement
  TransitionAndStoreArrayElement = 157,
  /// kTransitionElementsKind
  TransitionElementsKind = 158,
  /// kTransitionElementsKindOrCheckMap
  TransitionElementsKindOrCheckMap = 159,
  /// kDebugPrint
  DebugPrint = 160,
  /// kCheckTurboshaftTypeOf
  CheckTurboshaftTypeOf = 161,
  /// kCheckMaglevType
  CheckMaglevType = 162,
  /// kPrepareForLoop
  PrepareForLoop = 163,
  /// kGetContinuationPreservedEmbedderData
  GetContinuationPreservedEmbedderData = 164,
  /// kSetContinuationPreservedEmbedderData
  SetContinuationPreservedEmbedderData = 165,
  /// kGenericBinop
  GenericBinop = 166,
  /// kGenericUnop
  GenericUnop = 167,
  /// kToNumberOrNumeric
  ToNumberOrNumeric = 168,
  /// kLoadDictionaryField
  LoadDictionaryField = 169,
  /// kStringLocaleCompareIntl
  StringLocaleCompareIntl = 170,
  /// kStringToCaseIntl
  StringToCaseIntl = 171,
  /// kAllocate
  Allocate = 172,
  /// kMajorGCForCompilerTesting
  MajorGCForCompilerTesting = 173,
  /// kJSStackCheck
  JSStackCheck = 174,
  /// kLoadExternalPointer
  LoadExternalPointer = 175,
}

/// 原版 kNumberOfBlockTerminatorOpcodes
pub const NUMBER_OF_BLOCK_TERMINATOR_OPCODES: u16 = 9;

/// 原版 kNumberOfOpcodes
pub const NUMBER_OF_OPCODES: u16 = 176;

impl Opcode {
  /// 原版 OpcodeIndex
  #[inline]
  #[must_use]
  pub const fn index(self) -> u8 {
    self as u8
  }

  /// 原版 IsBlockTerminator — 块终结符占码表前缀
  #[inline]
  #[must_use]
  pub const fn is_block_terminator(self) -> bool {
    (self as u16) < NUMBER_OF_BLOCK_TERMINATOR_OPCODES
  }

  /// 原版 OpcodeName (operations.cc 表)
  #[must_use]
  pub const fn name(self) -> &'static str {
    OPCODE_NAMES[self as usize]
  }

  /// 原版 MayThrow — 后随 DidntThrow 的操作
  #[inline]
  #[must_use]
  pub const fn may_throw(self) -> bool {
    matches!(
      self,
      Opcode::GenericBinop
        | Opcode::GenericUnop
        | Opcode::ToNumberOrNumeric
        | Opcode::LoadDictionaryField
        | Opcode::StringLocaleCompareIntl
        | Opcode::StringToCaseIntl
        | Opcode::Call
        | Opcode::FastApiCall
    )
  }
}

/// 原版 OpcodeName 表 (operations.cc, 由宏展开生成, 顺序同 Opcode)
#[rustfmt::skip]
pub static OPCODE_NAMES: [&str; NUMBER_OF_OPCODES as usize] = [
  "CheckException", "Goto", "TailCall", "Unreachable", "Return", "Branch", "Switch", "Deoptimize",
  "WasmTrap", "WasmStackCheck", "WasmIncCoverageCounter", "GlobalGet", "GlobalSet", "RootConstant", "IsRootConstant", "Null",
  "IsNull", "AssertNotNull", "RttCanon", "WasmTypeCheck", "WasmTypeCast", "AnyConvertExtern", "ExternConvertAny", "WasmTypeAnnotation",
  "StructGet", "StructSet", "StructAtomicRMW", "ArrayGet", "ArraySet", "ArrayAtomicRMW", "ArrayLength", "WasmAllocateArray",
  "WasmAllocateStruct", "WasmRefFunc", "StringAsWtf16", "StringPrepareForGetCodeUnit", "ProcessWasmArgument", "Simd128Constant", "Simd128Binop", "Simd128Unary",
  "Simd128Reduce", "Simd128Shift", "Simd128Test", "Simd128Splat", "Simd128Ternary", "Simd128ExtractLane", "Simd128ReplaceLane", "Simd128MoveLane",
  "Simd128LaneMemory", "Simd128LoadTransform", "Simd128Shuffle", "Simd128LoadPairDeinterleave", "WordBinop", "FloatBinop", "Word32PairBinop", "Word64AddSub128Binop",
  "Word64Add3", "Word64MulWide", "OverflowCheckedBinop", "WordUnary", "OverflowCheckedUnary", "FloatUnary", "Shift", "Comparison",
  "Change", "TryChange", "BitcastWord32PairToFloat64", "TaggedBitcast", "Select", "PendingLoopPhi", "Constant", "LoadRootRegister",
  "Load", "Store", "Retain", "Parameter", "OsrValue", "StackPointerGreaterThan", "StackSlot", "FrameConstant",
  "DeoptimizeIf", "Phi", "FrameState", "Call", "CatchBlockBegin", "DidntThrow", "MakeTuple", "Projection",
  "DebugBreak", "AssumeMap", "AtomicRMW", "AtomicWord32Pair", "MemoryBarrier", "Comment", "Dead", "AbortCSADcheck",
  "Pause", "LoadTrustedPointer", "TrapIf", "LoadStackPointer", "SetStackPointer", "MemoryCopy", "MemoryFill", "WasmFXArgBuffer",
  "ArgumentsLength", "BigIntBinop", "BigIntComparison", "BigIntUnary", "CheckedClosure", "WordBinopDeoptOnOverflow", "CheckEqualsInternalizedString", "CheckHomomorphic",
  "CheckMaps", "CompareMaps", "FloatIs", "ObjectIs", "ObjectIsNumericValue", "Float64SameValue", "SameValue", "ChangeOrDeopt",
  "Convert", "ConvertJSPrimitiveToObject", "ConvertJSPrimitiveToUntagged", "ConvertJSPrimitiveToUntaggedOrDeopt", "ConvertUntaggedToJSPrimitive", "ConvertWordToSmiOrDeopt", "TruncateJSPrimitiveToUntagged", "TruncateJSPrimitiveToWord32OrDeopt",
  "DoubleArrayMinMax", "EnsureWritableFastElements", "FastApiCall", "FindOrderedHashEntry", "WeakCollectionGet", "LoadDataViewElement", "LoadFieldByIndex", "LoadMessage",
  "LoadStackArgument", "LoadTypedElement", "StoreDataViewElement", "StoreMessage", "StoreTypedElement", "MaybeGrowFastElements", "NewArgumentsElements", "NewArray",
  "RuntimeAbort", "StaticAssert", "StringAt", "StringComparison", "StringConcat", "StringFromCodePointAt", "StringIndexOf", "StringLength",
  "StringOrOddballStrictEqual", "TypedArrayLength", "StringSubstring", "StringSlice", "NewConsString", "TransitionAndStoreArrayElement", "TransitionElementsKind", "TransitionElementsKindOrCheckMap",
  "DebugPrint", "CheckTurboshaftTypeOf", "CheckMaglevType", "PrepareForLoop", "GetContinuationPreservedEmbedderData", "SetContinuationPreservedEmbedderData", "GenericBinop", "GenericUnop",
  "ToNumberOrNumeric", "LoadDictionaryField", "StringLocaleCompareIntl", "StringToCaseIntl", "Allocate", "MajorGCForCompilerTesting", "JSStackCheck", "LoadExternalPointer",
];

/// 原版 enum class HashingStrategy — 图哈希策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HashingStrategy {
  /// kDefault
  Default = 0,
  /// kMakeSnapshotStable — mksnapshot 复跑同哈希 (禁指针/外部常量入哈希)
  MakeSnapshotStable = 1,
}

// ---------------------------------------------------------------------------
// 各操作 Kind 子枚举 (struct 内 enum class Kind; 显式赋值已展开)
// ---------------------------------------------------------------------------

/// 原版 WordBinopOp::Kind (12 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum WordBinopKind {
  /// kAdd
  Add = 0,
  /// kMul
  Mul = 1,
  /// kSignedMulOverflownBits
  SignedMulOverflownBits = 2,
  /// kUnsignedMulOverflownBits
  UnsignedMulOverflownBits = 3,
  /// kBitwiseAnd
  BitwiseAnd = 4,
  /// kBitwiseOr
  BitwiseOr = 5,
  /// kBitwiseXor
  BitwiseXor = 6,
  /// kSub
  Sub = 7,
  /// kSignedDiv
  SignedDiv = 8,
  /// kUnsignedDiv
  UnsignedDiv = 9,
  /// kSignedMod
  SignedMod = 10,
  /// kUnsignedMod
  UnsignedMod = 11,
}

impl WordBinopKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Add;
    }
    if val == 1 {
      return Self::Mul;
    }
    if val == 2 {
      return Self::SignedMulOverflownBits;
    }
    if val == 3 {
      return Self::UnsignedMulOverflownBits;
    }
    if val == 4 {
      return Self::BitwiseAnd;
    }
    if val == 5 {
      return Self::BitwiseOr;
    }
    if val == 6 {
      return Self::BitwiseXor;
    }
    if val == 7 {
      return Self::Sub;
    }
    if val == 8 {
      return Self::SignedDiv;
    }
    if val == 9 {
      return Self::UnsignedDiv;
    }
    if val == 10 {
      return Self::SignedMod;
    }
    if val == 11 {
      return Self::UnsignedMod;
    }
    Self::Add
  }
}

/// 原版 FloatBinopOp::Kind (9 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum FloatBinopKind {
  /// kAdd
  Add = 0,
  /// kMul
  Mul = 1,
  /// kMin
  Min = 2,
  /// kMax
  Max = 3,
  /// kSub
  Sub = 4,
  /// kDiv
  Div = 5,
  /// kMod
  Mod = 6,
  /// kPower
  Power = 7,
  /// kAtan2
  Atan2 = 8,
}

impl FloatBinopKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Add;
    }
    if val == 1 {
      return Self::Mul;
    }
    if val == 2 {
      return Self::Min;
    }
    if val == 3 {
      return Self::Max;
    }
    if val == 4 {
      return Self::Sub;
    }
    if val == 5 {
      return Self::Div;
    }
    if val == 6 {
      return Self::Mod;
    }
    if val == 7 {
      return Self::Power;
    }
    if val == 8 {
      return Self::Atan2;
    }
    Self::Add
  }
}

/// 原版 Word32PairBinopOp::Kind (6 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Word32PairBinopKind {
  /// kAdd
  Add = 0,
  /// kSub
  Sub = 1,
  /// kMul
  Mul = 2,
  /// kShiftLeft
  ShiftLeft = 3,
  /// kShiftRightArithmetic
  ShiftRightArithmetic = 4,
  /// kShiftRightLogical
  ShiftRightLogical = 5,
}

impl Word32PairBinopKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Add;
    }
    if val == 1 {
      return Self::Sub;
    }
    if val == 2 {
      return Self::Mul;
    }
    if val == 3 {
      return Self::ShiftLeft;
    }
    if val == 4 {
      return Self::ShiftRightArithmetic;
    }
    if val == 5 {
      return Self::ShiftRightLogical;
    }
    Self::Add
  }
}

/// 原版 Word64MulWideOp::Kind (2 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Word64MulWideKind {
  /// kSigned
  Signed = 0,
  /// kUnsigned
  Unsigned = 1,
}

impl Word64MulWideKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Signed;
    }
    if val == 1 {
      return Self::Unsigned;
    }
    Self::Signed
  }
}

/// 原版 Word64AddSub128BinopOp::Kind (2 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Word64AddSub128BinopKind {
  /// kAdd
  Add = 0,
  /// kSub
  Sub = 1,
}

impl Word64AddSub128BinopKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Add;
    }
    if val == 1 {
      return Self::Sub;
    }
    Self::Add
  }
}

/// 原版 WordBinopDeoptOnOverflowOp::Kind (7 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum WordBinopDeoptOnOverflowKind {
  /// kSignedAdd
  SignedAdd = 0,
  /// kSignedMul
  SignedMul = 1,
  /// kSignedSub
  SignedSub = 2,
  /// kSignedDiv
  SignedDiv = 3,
  /// kSignedMod
  SignedMod = 4,
  /// kUnsignedDiv
  UnsignedDiv = 5,
  /// kUnsignedMod
  UnsignedMod = 6,
}

impl WordBinopDeoptOnOverflowKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::SignedAdd;
    }
    if val == 1 {
      return Self::SignedMul;
    }
    if val == 2 {
      return Self::SignedSub;
    }
    if val == 3 {
      return Self::SignedDiv;
    }
    if val == 4 {
      return Self::SignedMod;
    }
    if val == 5 {
      return Self::UnsignedDiv;
    }
    if val == 6 {
      return Self::UnsignedMod;
    }
    Self::SignedAdd
  }
}

/// 原版 OverflowCheckedBinopOp::Kind (3 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum OverflowCheckedBinopKind {
  /// kSignedAdd
  SignedAdd = 0,
  /// kSignedMul
  SignedMul = 1,
  /// kSignedSub
  SignedSub = 2,
}

impl OverflowCheckedBinopKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::SignedAdd;
    }
    if val == 1 {
      return Self::SignedMul;
    }
    if val == 2 {
      return Self::SignedSub;
    }
    Self::SignedAdd
  }
}

/// 原版 WordUnaryOp::Kind (6 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum WordUnaryKind {
  /// kReverseBytes
  ReverseBytes = 0,
  /// kCountLeadingZeros
  CountLeadingZeros = 1,
  /// kCountTrailingZeros
  CountTrailingZeros = 2,
  /// kPopCount
  PopCount = 3,
  /// kSignExtend8
  SignExtend8 = 4,
  /// kSignExtend16
  SignExtend16 = 5,
}

impl WordUnaryKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::ReverseBytes;
    }
    if val == 1 {
      return Self::CountLeadingZeros;
    }
    if val == 2 {
      return Self::CountTrailingZeros;
    }
    if val == 3 {
      return Self::PopCount;
    }
    if val == 4 {
      return Self::SignExtend8;
    }
    if val == 5 {
      return Self::SignExtend16;
    }
    Self::ReverseBytes
  }

  /// 原版 operator<< 名 (operations.cc, 6 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ReverseBytes => "ReverseBytes",
      Self::CountLeadingZeros => "CountLeadingZeros",
      Self::CountTrailingZeros => "CountTrailingZeros",
      Self::PopCount => "PopCount",
      Self::SignExtend8 => "SignExtend8",
      Self::SignExtend16 => "SignExtend16",
    }
  }
}

/// 原版 OverflowCheckedUnaryOp::Kind (1 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum OverflowCheckedUnaryKind {
  /// kAbs
  Abs = 0,
}

impl OverflowCheckedUnaryKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Abs;
    }
    Self::Abs
  }

  /// 原版 operator<< 名 (operations.cc, 1 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Abs => "kAbs",
    }
  }
}

/// 原版 FloatUnaryOp::Kind (27 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum FloatUnaryKind {
  /// kAbs
  Abs = 0,
  /// kNegate
  Negate = 1,
  /// kSilenceNaN
  SilenceNaN = 2,
  /// kRoundDown
  RoundDown = 3,
  /// kRoundUp
  RoundUp = 4,
  /// kRoundToZero
  RoundToZero = 5,
  /// kRoundTiesEven
  RoundTiesEven = 6,
  /// kLog
  Log = 7,
  /// kLog2
  Log2 = 8,
  /// kLog10
  Log10 = 9,
  /// kLog1p
  Log1p = 10,
  /// kSqrt
  Sqrt = 11,
  /// kCbrt
  Cbrt = 12,
  /// kExp
  Exp = 13,
  /// kExpm1
  Expm1 = 14,
  /// kSin
  Sin = 15,
  /// kCos
  Cos = 16,
  /// kSinh
  Sinh = 17,
  /// kCosh
  Cosh = 18,
  /// kAcos
  Acos = 19,
  /// kAsin
  Asin = 20,
  /// kAsinh
  Asinh = 21,
  /// kAcosh
  Acosh = 22,
  /// kTan
  Tan = 23,
  /// kTanh
  Tanh = 24,
  /// kAtan
  Atan = 25,
  /// kAtanh
  Atanh = 26,
}

impl FloatUnaryKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Abs;
    }
    if val == 1 {
      return Self::Negate;
    }
    if val == 2 {
      return Self::SilenceNaN;
    }
    if val == 3 {
      return Self::RoundDown;
    }
    if val == 4 {
      return Self::RoundUp;
    }
    if val == 5 {
      return Self::RoundToZero;
    }
    if val == 6 {
      return Self::RoundTiesEven;
    }
    if val == 7 {
      return Self::Log;
    }
    if val == 8 {
      return Self::Log2;
    }
    if val == 9 {
      return Self::Log10;
    }
    if val == 10 {
      return Self::Log1p;
    }
    if val == 11 {
      return Self::Sqrt;
    }
    if val == 12 {
      return Self::Cbrt;
    }
    if val == 13 {
      return Self::Exp;
    }
    if val == 14 {
      return Self::Expm1;
    }
    if val == 15 {
      return Self::Sin;
    }
    if val == 16 {
      return Self::Cos;
    }
    if val == 17 {
      return Self::Sinh;
    }
    if val == 18 {
      return Self::Cosh;
    }
    if val == 19 {
      return Self::Acos;
    }
    if val == 20 {
      return Self::Asin;
    }
    if val == 21 {
      return Self::Asinh;
    }
    if val == 22 {
      return Self::Acosh;
    }
    if val == 23 {
      return Self::Tan;
    }
    if val == 24 {
      return Self::Tanh;
    }
    if val == 25 {
      return Self::Atan;
    }
    if val == 26 {
      return Self::Atanh;
    }
    Self::Abs
  }

  /// 原版 operator<< 名 (operations.cc, 27 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Abs => "Abs",
      Self::Negate => "Negate",
      Self::SilenceNaN => "SilenceNaN",
      Self::RoundUp => "RoundUp",
      Self::RoundDown => "RoundDown",
      Self::RoundToZero => "RoundToZero",
      Self::RoundTiesEven => "RoundTiesEven",
      Self::Log => "Log",
      Self::Log2 => "Log2",
      Self::Log10 => "Log10",
      Self::Log1p => "Log1p",
      Self::Sqrt => "Sqrt",
      Self::Cbrt => "Cbrt",
      Self::Exp => "Exp",
      Self::Expm1 => "Expm1",
      Self::Sin => "Sin",
      Self::Cos => "Cos",
      Self::Asin => "Asin",
      Self::Acos => "Acos",
      Self::Sinh => "Sinh",
      Self::Cosh => "Cosh",
      Self::Asinh => "Asinh",
      Self::Acosh => "Acosh",
      Self::Tan => "Tan",
      Self::Tanh => "Tanh",
      Self::Atan => "Atan",
      Self::Atanh => "Atanh",
    }
  }
}

/// 原版 ShiftOp::Kind (6 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ShiftKind {
  /// kShiftRightArithmeticShiftOutZeros
  ShiftRightArithmeticShiftOutZeros = 0,
  /// kShiftRightArithmetic
  ShiftRightArithmetic = 1,
  /// kShiftRightLogical
  ShiftRightLogical = 2,
  /// kShiftLeft
  ShiftLeft = 3,
  /// kRotateRight
  RotateRight = 4,
  /// kRotateLeft
  RotateLeft = 5,
}

impl ShiftKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::ShiftRightArithmeticShiftOutZeros;
    }
    if val == 1 {
      return Self::ShiftRightArithmetic;
    }
    if val == 2 {
      return Self::ShiftRightLogical;
    }
    if val == 3 {
      return Self::ShiftLeft;
    }
    if val == 4 {
      return Self::RotateRight;
    }
    if val == 5 {
      return Self::RotateLeft;
    }
    Self::ShiftRightArithmeticShiftOutZeros
  }

  /// 原版 operator<< 名 (operations.cc, 6 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ShiftRightArithmeticShiftOutZeros => "ShiftRightArithmeticShiftOutZeros",
      Self::ShiftRightArithmetic => "ShiftRightArithmetic",
      Self::ShiftRightLogical => "ShiftRightLogical",
      Self::ShiftLeft => "ShiftLeft",
      Self::RotateRight => "RotateRight",
      Self::RotateLeft => "RotateLeft",
    }
  }
}

/// 原版 ComparisonOp::Kind (5 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ComparisonKind {
  /// kEqual
  Equal = 0,
  /// kSignedLessThan
  SignedLessThan = 1,
  /// kSignedLessThanOrEqual
  SignedLessThanOrEqual = 2,
  /// kUnsignedLessThan
  UnsignedLessThan = 3,
  /// kUnsignedLessThanOrEqual
  UnsignedLessThanOrEqual = 4,
}

impl ComparisonKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Equal;
    }
    if val == 1 {
      return Self::SignedLessThan;
    }
    if val == 2 {
      return Self::SignedLessThanOrEqual;
    }
    if val == 3 {
      return Self::UnsignedLessThan;
    }
    if val == 4 {
      return Self::UnsignedLessThanOrEqual;
    }
    Self::Equal
  }

  /// 原版 operator<< 名 (operations.cc, 5 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Equal => "Equal",
      Self::SignedLessThan => "SignedLessThan",
      Self::SignedLessThanOrEqual => "SignedLessThanOrEqual",
      Self::UnsignedLessThan => "UnsignedLessThan",
      Self::UnsignedLessThanOrEqual => "UnsignedLessThanOrEqual",
    }
  }
}

/// 原版 ChangeOp::Kind (14 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ChangeKind {
  /// kFloatConversion
  FloatConversion = 0,
  /// kSignedFloatTruncateOverflowToMin
  SignedFloatTruncateOverflowToMin = 1,
  /// kUnsignedFloatTruncateOverflowToMin
  UnsignedFloatTruncateOverflowToMin = 2,
  /// kJSFloatTruncate
  JSFloatTruncate = 3,
  /// kJSFloat16TruncateWithBitcast
  JSFloat16TruncateWithBitcast = 4,
  /// kJSFloat16ChangeWithBitcast
  JSFloat16ChangeWithBitcast = 5,
  /// kSignedToFloat
  SignedToFloat = 6,
  /// kUnsignedToFloat
  UnsignedToFloat = 7,
  /// kExtractHighHalf
  ExtractHighHalf = 8,
  /// kExtractLowHalf
  ExtractLowHalf = 9,
  /// kZeroExtend
  ZeroExtend = 10,
  /// kSignExtend
  SignExtend = 11,
  /// kTruncate
  Truncate = 12,
  /// kBitcast
  Bitcast = 13,
}

impl ChangeKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::FloatConversion;
    }
    if val == 1 {
      return Self::SignedFloatTruncateOverflowToMin;
    }
    if val == 2 {
      return Self::UnsignedFloatTruncateOverflowToMin;
    }
    if val == 3 {
      return Self::JSFloatTruncate;
    }
    if val == 4 {
      return Self::JSFloat16TruncateWithBitcast;
    }
    if val == 5 {
      return Self::JSFloat16ChangeWithBitcast;
    }
    if val == 6 {
      return Self::SignedToFloat;
    }
    if val == 7 {
      return Self::UnsignedToFloat;
    }
    if val == 8 {
      return Self::ExtractHighHalf;
    }
    if val == 9 {
      return Self::ExtractLowHalf;
    }
    if val == 10 {
      return Self::ZeroExtend;
    }
    if val == 11 {
      return Self::SignExtend;
    }
    if val == 12 {
      return Self::Truncate;
    }
    if val == 13 {
      return Self::Bitcast;
    }
    Self::FloatConversion
  }

  /// 原版 operator<< 名 (operations.cc, 14 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FloatConversion => "FloatConversion",
      Self::JSFloatTruncate => "JSFloatTruncate",
      Self::JSFloat16ChangeWithBitcast => "JSFloat16ChangeWithBitcast",
      Self::JSFloat16TruncateWithBitcast => "JSFloat16TruncateWithBitcast",
      Self::SignedFloatTruncateOverflowToMin => "SignedFloatTruncateOverflowToMin",
      Self::UnsignedFloatTruncateOverflowToMin => "UnsignedFloatTruncateOverflowToMin",
      Self::SignedToFloat => "SignedToFloat",
      Self::UnsignedToFloat => "UnsignedToFloat",
      Self::ExtractHighHalf => "ExtractHighHalf",
      Self::ExtractLowHalf => "ExtractLowHalf",
      Self::ZeroExtend => "ZeroExtend",
      Self::SignExtend => "SignExtend",
      Self::Truncate => "Truncate",
      Self::Bitcast => "Bitcast",
    }
  }
}

/// 原版 ChangeOrDeoptOp::Kind (13 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ChangeOrDeoptKind {
  /// kUint32ToInt32
  Uint32ToInt32 = 0,
  /// kInt64ToInt32
  Int64ToInt32 = 1,
  /// kInt64ToAdditiveSafeInteger
  Int64ToAdditiveSafeInteger = 2,
  /// kUint64ToInt32
  Uint64ToInt32 = 3,
  /// kUint64ToInt64
  Uint64ToInt64 = 4,
  /// kFloat64ToInt32
  Float64ToInt32 = 5,
  /// kFloat64ToUint32
  Float64ToUint32 = 6,
  /// kInt32ToUint64
  Int32ToUint64 = 7,
  /// kInt64ToUint64
  Int64ToUint64 = 8,
  /// kFloat64ToUint64
  Float64ToUint64 = 9,
  /// kFloat64ToAdditiveSafeInteger
  Float64ToAdditiveSafeInteger = 10,
  /// kFloat64ToInt64
  Float64ToInt64 = 11,
  /// kFloat64NotHole
  Float64NotHole = 12,
}

impl ChangeOrDeoptKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Uint32ToInt32;
    }
    if val == 1 {
      return Self::Int64ToInt32;
    }
    if val == 2 {
      return Self::Int64ToAdditiveSafeInteger;
    }
    if val == 3 {
      return Self::Uint64ToInt32;
    }
    if val == 4 {
      return Self::Uint64ToInt64;
    }
    if val == 5 {
      return Self::Float64ToInt32;
    }
    if val == 6 {
      return Self::Float64ToUint32;
    }
    if val == 7 {
      return Self::Int32ToUint64;
    }
    if val == 8 {
      return Self::Int64ToUint64;
    }
    if val == 9 {
      return Self::Float64ToUint64;
    }
    if val == 10 {
      return Self::Float64ToAdditiveSafeInteger;
    }
    if val == 11 {
      return Self::Float64ToInt64;
    }
    if val == 12 {
      return Self::Float64NotHole;
    }
    Self::Uint32ToInt32
  }

  /// 原版 operator<< 名 (operations.cc, 13 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Uint32ToInt32 => "Uint32ToInt32",
      Self::Int64ToInt32 => "Int64ToInt32",
      Self::Int64ToAdditiveSafeInteger => "Int64ToAdditiveSafeInteger",
      Self::Uint64ToInt32 => "Uint64ToInt32",
      Self::Uint64ToInt64 => "Uint64ToInt64",
      Self::Float64ToInt32 => "Float64ToInt32",
      Self::Float64ToUint32 => "Float64ToUint32",
      Self::Float64ToAdditiveSafeInteger => "Float64ToAdditiveSafeInteger",
      Self::Float64ToInt64 => "Float64ToInt64",
      Self::Float64NotHole => "Float64NotHole",
      Self::Int32ToUint64 => "Int32ToUint64",
      Self::Int64ToUint64 => "Int64ToUint64",
      Self::Float64ToUint64 => "Float64ToUint64",
    }
  }
}

/// 原版 TryChangeOp::Kind (2 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum TryChangeKind {
  /// kSignedFloatTruncateOverflowUndefined
  SignedFloatTruncateOverflowUndefined = 0,
  /// kUnsignedFloatTruncateOverflowUndefined
  UnsignedFloatTruncateOverflowUndefined = 1,
}

impl TryChangeKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::SignedFloatTruncateOverflowUndefined;
    }
    if val == 1 {
      return Self::UnsignedFloatTruncateOverflowUndefined;
    }
    Self::SignedFloatTruncateOverflowUndefined
  }

  /// 原版 operator<< 名 (operations.cc, 2 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SignedFloatTruncateOverflowUndefined => "SignedFloatTruncateOverflowUndefined",
      Self::UnsignedFloatTruncateOverflowUndefined => "UnsignedFloatTruncateOverflowUndefined",
    }
  }
}

/// 原版 TaggedBitcastOp::Kind (4 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum TaggedBitcastKind {
  /// kSmi
  Smi = 0,
  /// kHeapObject
  HeapObject = 1,
  /// kTagAndSmiBits
  TagAndSmiBits = 2,
  /// kAny
  Any = 3,
}

impl TaggedBitcastKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Smi;
    }
    if val == 1 {
      return Self::HeapObject;
    }
    if val == 2 {
      return Self::TagAndSmiBits;
    }
    if val == 3 {
      return Self::Any;
    }
    Self::Smi
  }

  /// 原版 operator<< 名 (operations.cc, 4 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Smi => "Smi",
      Self::HeapObject => "HeapObject",
      Self::TagAndSmiBits => "TagAndSmiBits",
      Self::Any => "Any",
    }
  }
}

/// 原版 ConstantOp::Kind (16 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ConstantKind {
  /// kWord32
  Word32 = 0,
  /// kWord64
  Word64 = 1,
  /// kFloat32
  Float32 = 2,
  /// kFloat64
  Float64 = 3,
  /// kSmi
  Smi = 4,
  /// kNumber
  Number = 5,
  /// kTaggedIndex
  TaggedIndex = 6,
  /// kExternal
  External = 7,
  /// kHeapObject
  HeapObject = 8,
  /// kCompressedHeapObject
  CompressedHeapObject = 9,
  /// kTrustedHeapObject
  TrustedHeapObject = 10,
  /// kRelocatableWasmCall
  RelocatableWasmCall = 11,
  /// kRelocatableWasmStubCall
  RelocatableWasmStubCall = 12,
  /// kRelocatableWasmIndirectCallTarget
  RelocatableWasmIndirectCallTarget = 13,
  /// kRelocatableWasmCanonicalSignatureId
  RelocatableWasmCanonicalSignatureId = 14,
  /// kRelocatableWasmCodePointer
  RelocatableWasmCodePointer = 15,
}

impl ConstantKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Word32;
    }
    if val == 1 {
      return Self::Word64;
    }
    if val == 2 {
      return Self::Float32;
    }
    if val == 3 {
      return Self::Float64;
    }
    if val == 4 {
      return Self::Smi;
    }
    if val == 5 {
      return Self::Number;
    }
    if val == 6 {
      return Self::TaggedIndex;
    }
    if val == 7 {
      return Self::External;
    }
    if val == 8 {
      return Self::HeapObject;
    }
    if val == 9 {
      return Self::CompressedHeapObject;
    }
    if val == 10 {
      return Self::TrustedHeapObject;
    }
    if val == 11 {
      return Self::RelocatableWasmCall;
    }
    if val == 12 {
      return Self::RelocatableWasmStubCall;
    }
    if val == 13 {
      return Self::RelocatableWasmIndirectCallTarget;
    }
    if val == 14 {
      return Self::RelocatableWasmCanonicalSignatureId;
    }
    if val == 15 {
      return Self::RelocatableWasmCodePointer;
    }
    Self::Word32
  }
}

/// 原版 AtomicWord32PairOp::Kind (9 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum AtomicWord32PairKind {
  /// kAdd
  Add = 0,
  /// kSub
  Sub = 1,
  /// kAnd
  And = 2,
  /// kOr
  Or = 3,
  /// kXor
  Xor = 4,
  /// kExchange
  Exchange = 5,
  /// kCompareExchange
  CompareExchange = 6,
  /// kLoad
  Load = 7,
  /// kStore
  Store = 8,
}

impl AtomicWord32PairKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Add;
    }
    if val == 1 {
      return Self::Sub;
    }
    if val == 2 {
      return Self::And;
    }
    if val == 3 {
      return Self::Or;
    }
    if val == 4 {
      return Self::Xor;
    }
    if val == 5 {
      return Self::Exchange;
    }
    if val == 6 {
      return Self::CompareExchange;
    }
    if val == 7 {
      return Self::Load;
    }
    if val == 8 {
      return Self::Store;
    }
    Self::Add
  }
}

/// 原版 JSStackCheckOp::Kind (3 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum JSStackCheckKind {
  /// kFunctionEntry
  FunctionEntry = 0,
  /// kBuiltinEntry
  BuiltinEntry = 1,
  /// kLoop
  Loop = 2,
}

impl JSStackCheckKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::FunctionEntry;
    }
    if val == 1 {
      return Self::BuiltinEntry;
    }
    if val == 2 {
      return Self::Loop;
    }
    Self::FunctionEntry
  }

  /// 原版 operator<< 名 (operations.cc, 3 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FunctionEntry => "function-entry",
      Self::BuiltinEntry => "builtin-entry",
      Self::Loop => "loop",
    }
  }
}

/// 原版 FrameConstantOp::Kind (3 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum FrameConstantKind {
  /// kStackCheckOffset
  StackCheckOffset = 0,
  /// kFramePointer
  FramePointer = 1,
  /// kParentFramePointer
  ParentFramePointer = 2,
}

impl FrameConstantKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::StackCheckOffset;
    }
    if val == 1 {
      return Self::FramePointer;
    }
    if val == 2 {
      return Self::ParentFramePointer;
    }
    Self::StackCheckOffset
  }

  /// 原版 operator<< 名 (operations.cc, 3 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::StackCheckOffset => "stack check offset",
      Self::FramePointer => "frame pointer",
      Self::ParentFramePointer => "parent frame pointer",
    }
  }
}

/// 原版 ObjectIsOp::Kind (20 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ObjectIsKind {
  /// kArrayBufferView
  ArrayBufferView = 0,
  /// kBigInt
  BigInt = 1,
  /// kBigInt64
  BigInt64 = 2,
  /// kCallable
  Callable = 3,
  /// kConstructor
  Constructor = 4,
  /// kDetectableCallable
  DetectableCallable = 5,
  /// kInternalizedString
  InternalizedString = 6,
  /// kNonCallable
  NonCallable = 7,
  /// kNumber
  Number = 8,
  /// kNumberOrUndefined
  NumberOrUndefined = 9,
  /// kNumberFitsInt32
  NumberFitsInt32 = 10,
  /// kNumberOrBigInt
  NumberOrBigInt = 11,
  /// kReceiver
  Receiver = 12,
  /// kReceiverOrNullOrUndefined
  ReceiverOrNullOrUndefined = 13,
  /// kSmi
  Smi = 14,
  /// kString
  String = 15,
  /// kStringOrStringWrapper
  StringOrStringWrapper = 16,
  /// kStringOrOddball
  StringOrOddball = 17,
  /// kSymbol
  Symbol = 18,
  /// kUndetectable
  Undetectable = 19,
}

impl ObjectIsKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::ArrayBufferView;
    }
    if val == 1 {
      return Self::BigInt;
    }
    if val == 2 {
      return Self::BigInt64;
    }
    if val == 3 {
      return Self::Callable;
    }
    if val == 4 {
      return Self::Constructor;
    }
    if val == 5 {
      return Self::DetectableCallable;
    }
    if val == 6 {
      return Self::InternalizedString;
    }
    if val == 7 {
      return Self::NonCallable;
    }
    if val == 8 {
      return Self::Number;
    }
    if val == 9 {
      return Self::NumberOrUndefined;
    }
    if val == 10 {
      return Self::NumberFitsInt32;
    }
    if val == 11 {
      return Self::NumberOrBigInt;
    }
    if val == 12 {
      return Self::Receiver;
    }
    if val == 13 {
      return Self::ReceiverOrNullOrUndefined;
    }
    if val == 14 {
      return Self::Smi;
    }
    if val == 15 {
      return Self::String;
    }
    if val == 16 {
      return Self::StringOrStringWrapper;
    }
    if val == 17 {
      return Self::StringOrOddball;
    }
    if val == 18 {
      return Self::Symbol;
    }
    if val == 19 {
      return Self::Undetectable;
    }
    Self::ArrayBufferView
  }

  /// 原版 operator<< 名 (operations.cc, 20 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ArrayBufferView => "ArrayBufferView",
      Self::BigInt => "BigInt",
      Self::BigInt64 => "BigInt64",
      Self::Callable => "Callable",
      Self::Constructor => "Constructor",
      Self::DetectableCallable => "DetectableCallable",
      Self::InternalizedString => "InternalizedString",
      Self::NonCallable => "NonCallable",
      Self::Number => "Number",
      Self::NumberOrUndefined => "NumberOrUndefined",
      Self::NumberFitsInt32 => "NumberFitsInt32",
      Self::NumberOrBigInt => "NumberOrBigInt",
      Self::Receiver => "Receiver",
      Self::ReceiverOrNullOrUndefined => "ReceiverOrNullOrUndefined",
      Self::Smi => "Smi",
      Self::String => "String",
      Self::StringOrStringWrapper => "StringOrStringWrapper",
      Self::StringOrOddball => "StringOrOddball",
      Self::Symbol => "Symbol",
      Self::Undetectable => "Undetectable",
    }
  }
}

/// 原版 ConvertOp::Kind (7 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ConvertKind {
  /// kObject
  Object = 0,
  /// kBoolean
  Boolean = 1,
  /// kNumber
  Number = 2,
  /// kNumberOrOddball
  NumberOrOddball = 3,
  /// kPlainPrimitive
  PlainPrimitive = 4,
  /// kString
  String = 5,
  /// kSmi
  Smi = 6,
}

impl ConvertKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Object;
    }
    if val == 1 {
      return Self::Boolean;
    }
    if val == 2 {
      return Self::Number;
    }
    if val == 3 {
      return Self::NumberOrOddball;
    }
    if val == 4 {
      return Self::PlainPrimitive;
    }
    if val == 5 {
      return Self::String;
    }
    if val == 6 {
      return Self::Smi;
    }
    Self::Object
  }

  /// 原版 operator<< 名 (operations.cc, 7 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Object => "Object",
      Self::Boolean => "Boolean",
      Self::Number => "Number",
      Self::NumberOrOddball => "NumberOrOddball",
      Self::PlainPrimitive => "PlainPrimitive",
      Self::String => "String",
      Self::Smi => "Smi",
    }
  }
}

/// 原版 NewArrayOp::Kind (2 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum NewArrayKind {
  /// kDouble
  Double = 0,
  /// kObject
  Object = 1,
}

impl NewArrayKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Double;
    }
    if val == 1 {
      return Self::Object;
    }
    Self::Double
  }

  /// 原版 operator<< 名 (operations.cc, 2 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Double => "Double",
      Self::Object => "Object",
    }
  }
}

/// 原版 DoubleArrayMinMaxOp::Kind (2 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum DoubleArrayMinMaxKind {
  /// kMin
  Min = 0,
  /// kMax
  Max = 1,
}

impl DoubleArrayMinMaxKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Min;
    }
    if val == 1 {
      return Self::Max;
    }
    Self::Min
  }

  /// 原版 operator<< 名 (operations.cc, 2 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Min => "Min",
      Self::Max => "Max",
    }
  }
}

/// 原版 BigIntBinopOp::Kind (10 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum BigIntBinopKind {
  /// kAdd
  Add = 0,
  /// kSub
  Sub = 1,
  /// kMul
  Mul = 2,
  /// kDiv
  Div = 3,
  /// kMod
  Mod = 4,
  /// kBitwiseAnd
  BitwiseAnd = 5,
  /// kBitwiseOr
  BitwiseOr = 6,
  /// kBitwiseXor
  BitwiseXor = 7,
  /// kShiftLeft
  ShiftLeft = 8,
  /// kShiftRightArithmetic
  ShiftRightArithmetic = 9,
}

impl BigIntBinopKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Add;
    }
    if val == 1 {
      return Self::Sub;
    }
    if val == 2 {
      return Self::Mul;
    }
    if val == 3 {
      return Self::Div;
    }
    if val == 4 {
      return Self::Mod;
    }
    if val == 5 {
      return Self::BitwiseAnd;
    }
    if val == 6 {
      return Self::BitwiseOr;
    }
    if val == 7 {
      return Self::BitwiseXor;
    }
    if val == 8 {
      return Self::ShiftLeft;
    }
    if val == 9 {
      return Self::ShiftRightArithmetic;
    }
    Self::Add
  }

  /// 原版 operator<< 名 (operations.cc, 10 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Add => "Add",
      Self::Sub => "Sub",
      Self::Mul => "Mul",
      Self::Div => "Div",
      Self::Mod => "Mod",
      Self::BitwiseAnd => "BitwiseAnd",
      Self::BitwiseOr => "BitwiseOr",
      Self::BitwiseXor => "BitwiseXor",
      Self::ShiftLeft => "ShiftLeft",
      Self::ShiftRightArithmetic => "ShiftRightArithmetic",
    }
  }
}

/// 原版 BigIntComparisonOp::Kind (3 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum BigIntComparisonKind {
  /// kEqual
  Equal = 0,
  /// kLessThan
  LessThan = 1,
  /// kLessThanOrEqual
  LessThanOrEqual = 2,
}

impl BigIntComparisonKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Equal;
    }
    if val == 1 {
      return Self::LessThan;
    }
    if val == 2 {
      return Self::LessThanOrEqual;
    }
    Self::Equal
  }

  /// 原版 operator<< 名 (operations.cc, 3 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Equal => "Equal",
      Self::LessThan => "LessThan",
      Self::LessThanOrEqual => "LessThanOrEqual",
    }
  }
}

/// 原版 BigIntUnaryOp::Kind (1 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum BigIntUnaryKind {
  /// kNegate
  Negate = 0,
}

impl BigIntUnaryKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Negate;
    }
    Self::Negate
  }

  /// 原版 operator<< 名 (operations.cc, 1 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Negate => "Negate",
    }
  }
}

/// 原版 StringAtOp::Kind (2 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum StringAtKind {
  /// kCharCode
  CharCode = 0,
  /// kCodePoint
  CodePoint = 1,
}

impl StringAtKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::CharCode;
    }
    if val == 1 {
      return Self::CodePoint;
    }
    Self::CharCode
  }

  /// 原版 operator<< 名 (operations.cc, 2 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::CharCode => "CharCode",
      Self::CodePoint => "CodePoint",
    }
  }
}

/// 原版 StringToCaseIntlOp::Kind (2 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum StringToCaseIntlKind {
  /// kLower
  Lower = 0,
  /// kUpper
  Upper = 1,
}

impl StringToCaseIntlKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Lower;
    }
    if val == 1 {
      return Self::Upper;
    }
    Self::Lower
  }

  /// 原版 operator<< 名 (operations.cc, 2 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Lower => "Lower",
      Self::Upper => "Upper",
    }
  }
}

/// 原版 StringComparisonOp::Kind (3 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum StringComparisonKind {
  /// kEqual
  Equal = 0,
  /// kLessThan
  LessThan = 1,
  /// kLessThanOrEqual
  LessThanOrEqual = 2,
}

impl StringComparisonKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Equal;
    }
    if val == 1 {
      return Self::LessThan;
    }
    if val == 2 {
      return Self::LessThanOrEqual;
    }
    Self::Equal
  }

  /// 原版 operator<< 名 (operations.cc, 3 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Equal => "Equal",
      Self::LessThan => "LessThan",
      Self::LessThanOrEqual => "LessThanOrEqual",
    }
  }
}

/// 原版 ArgumentsLengthOp::Kind (2 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ArgumentsLengthKind {
  /// kArguments
  Arguments = 0,
  /// kRest
  Rest = 1,
}

impl ArgumentsLengthKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Arguments;
    }
    if val == 1 {
      return Self::Rest;
    }
    Self::Arguments
  }

  /// 原版 operator<< 名 (operations.cc, 2 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Arguments => "Arguments",
      Self::Rest => "Rest",
    }
  }
}

/// 原版 TransitionAndStoreArrayElementOp::Kind (5 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum TransitionAndStoreArrayElementKind {
  /// kElement
  Element = 0,
  /// kNumberElement
  NumberElement = 1,
  /// kOddballElement
  OddballElement = 2,
  /// kNonNumberElement
  NonNumberElement = 3,
  /// kSignedSmallElement
  SignedSmallElement = 4,
}

impl TransitionAndStoreArrayElementKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Element;
    }
    if val == 1 {
      return Self::NumberElement;
    }
    if val == 2 {
      return Self::OddballElement;
    }
    if val == 3 {
      return Self::NonNumberElement;
    }
    if val == 4 {
      return Self::SignedSmallElement;
    }
    Self::Element
  }
}

/// 原版 FindOrderedHashEntryOp::Kind (3 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum FindOrderedHashEntryKind {
  /// kFindOrderedHashMapEntry
  FindOrderedHashMapEntry = 0,
  /// kFindOrderedHashMapEntryForInt32Key
  FindOrderedHashMapEntryForInt32Key = 1,
  /// kFindOrderedHashSetEntry
  FindOrderedHashSetEntry = 2,
}

impl FindOrderedHashEntryKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::FindOrderedHashMapEntry;
    }
    if val == 1 {
      return Self::FindOrderedHashMapEntryForInt32Key;
    }
    if val == 2 {
      return Self::FindOrderedHashSetEntry;
    }
    Self::FindOrderedHashMapEntry
  }

  /// 原版 operator<< 名 (operations.cc, 3 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FindOrderedHashMapEntry => "FindOrderedHashMapEntry",
      Self::FindOrderedHashMapEntryForInt32Key => "FindOrderedHashMapEntryForInt32Key",
      Self::FindOrderedHashSetEntry => "FindOrderedHashSetEntry",
    }
  }
}

/// 原版 StructSetOp::Kind (2 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum StructSetKind {
  /// kInitialize
  Initialize = 0,
  /// kAssign
  Assign = 1,
}

impl StructSetKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::Initialize;
    }
    if val == 1 {
      return Self::Assign;
    }
    Self::Initialize
  }
}

/// 原版 Simd128BinopOp::Kind (4 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Simd128BinopKind {
  /// kFirstExtensionOp
  FirstExtensionOp = 0,
  /// kI16x8ExtMulLowI8x16S
  I16x8ExtMulLowI8x16S = 1,
  /// kLastExtensionOp
  LastExtensionOp = 2,
  /// kI64x2ExtMulHighI32x4U
  I64x2ExtMulHighI32x4U = 3,
}

impl Simd128BinopKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::FirstExtensionOp;
    }
    if val == 1 {
      return Self::I16x8ExtMulLowI8x16S;
    }
    if val == 2 {
      return Self::LastExtensionOp;
    }
    if val == 3 {
      return Self::I64x2ExtMulHighI32x4U;
    }
    Self::FirstExtensionOp
  }
}

/// 原版 Simd128UnaryOp::Kind (4 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Simd128UnaryKind {
  /// kFirstExtensionOp
  FirstExtensionOp = 0,
  /// kI16x8SConvertI8x16Low
  I16x8SConvertI8x16Low = 1,
  /// kLastExtensionOp
  LastExtensionOp = 2,
  /// kI64x2UConvertI32x4High
  I64x2UConvertI32x4High = 3,
}

impl Simd128UnaryKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::FirstExtensionOp;
    }
    if val == 1 {
      return Self::I16x8SConvertI8x16Low;
    }
    if val == 2 {
      return Self::LastExtensionOp;
    }
    if val == 3 {
      return Self::I64x2UConvertI32x4High;
    }
    Self::FirstExtensionOp
  }
}

/// 原版 Simd128ExtractLaneOp::Kind (9 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Simd128ExtractLaneKind {
  /// kI8x16S
  I8x16S = 0,
  /// kI8x16U
  I8x16U = 1,
  /// kI16x8S
  I16x8S = 2,
  /// kI16x8U
  I16x8U = 3,
  /// kI32x4
  I32x4 = 4,
  /// kI64x2
  I64x2 = 5,
  /// kF16x8
  F16x8 = 6,
  /// kF32x4
  F32x4 = 7,
  /// kF64x2
  F64x2 = 8,
}

impl Simd128ExtractLaneKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::I8x16S;
    }
    if val == 1 {
      return Self::I8x16U;
    }
    if val == 2 {
      return Self::I16x8S;
    }
    if val == 3 {
      return Self::I16x8U;
    }
    if val == 4 {
      return Self::I32x4;
    }
    if val == 5 {
      return Self::I64x2;
    }
    if val == 6 {
      return Self::F16x8;
    }
    if val == 7 {
      return Self::F32x4;
    }
    if val == 8 {
      return Self::F64x2;
    }
    Self::I8x16S
  }
}

/// 原版 Simd128ReplaceLaneOp::Kind (7 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Simd128ReplaceLaneKind {
  /// kI8x16
  I8x16 = 0,
  /// kI16x8
  I16x8 = 1,
  /// kI32x4
  I32x4 = 2,
  /// kI64x2
  I64x2 = 3,
  /// kF16x8
  F16x8 = 4,
  /// kF32x4
  F32x4 = 5,
  /// kF64x2
  F64x2 = 6,
}

impl Simd128ReplaceLaneKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::I8x16;
    }
    if val == 1 {
      return Self::I16x8;
    }
    if val == 2 {
      return Self::I32x4;
    }
    if val == 3 {
      return Self::I64x2;
    }
    if val == 4 {
      return Self::F16x8;
    }
    if val == 5 {
      return Self::F32x4;
    }
    if val == 6 {
      return Self::F64x2;
    }
    Self::I8x16
  }
}

/// 原版 Simd128ShuffleOp::Kind (5 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Simd128ShuffleKind {
  /// kI8x1
  I8x1 = 0,
  /// kI8x2
  I8x2 = 1,
  /// kI8x4
  I8x4 = 2,
  /// kI8x8
  I8x8 = 3,
  /// kI8x16
  I8x16 = 4,
}

impl Simd128ShuffleKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::I8x1;
    }
    if val == 1 {
      return Self::I8x2;
    }
    if val == 2 {
      return Self::I8x4;
    }
    if val == 3 {
      return Self::I8x8;
    }
    if val == 4 {
      return Self::I8x16;
    }
    Self::I8x1
  }
}

/// 原版 Simd256UnaryOp::Kind (4 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Simd256UnaryKind {
  /// kFirstExtensionOp
  FirstExtensionOp = 0,
  /// kI16x16SConvertI8x16
  I16x16SConvertI8x16 = 1,
  /// kLastExtensionOp
  LastExtensionOp = 2,
  /// kI64x4UConvertI32x4
  I64x4UConvertI32x4 = 3,
}

impl Simd256UnaryKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::FirstExtensionOp;
    }
    if val == 1 {
      return Self::I16x16SConvertI8x16;
    }
    if val == 2 {
      return Self::LastExtensionOp;
    }
    if val == 3 {
      return Self::I64x4UConvertI32x4;
    }
    Self::FirstExtensionOp
  }
}

/// 原版 Simd256BinopOp::Kind (4 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Simd256BinopKind {
  /// kFirstExtensionOp
  FirstExtensionOp = 0,
  /// kI64x4ExtMulI32x4S
  I64x4ExtMulI32x4S = 1,
  /// kLastExtensionOp
  LastExtensionOp = 2,
  /// kI16x16ExtMulI8x16U
  I16x16ExtMulI8x16U = 3,
}

impl Simd256BinopKind {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    if val == 0 {
      return Self::FirstExtensionOp;
    }
    if val == 1 {
      return Self::I64x4ExtMulI32x4S;
    }
    if val == 2 {
      return Self::LastExtensionOp;
    }
    if val == 3 {
      return Self::I16x16ExtMulI8x16U;
    }
    Self::FirstExtensionOp
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(NUMBER_OF_OPCODES == 176);
  assert!(NUMBER_OF_BLOCK_TERMINATOR_OPCODES == 9);
  assert!(Opcode::CheckException.is_block_terminator());
  assert!(Opcode::WasmTrap.is_block_terminator());
  assert!(!Opcode::Call.is_block_terminator());
  assert!(Opcode::Call.may_throw());
  assert!(Opcode::Goto.is_block_terminator());
  assert!(Opcode::index(Opcode::Call) >= 1);
  assert!(OPCODE_NAMES.len() == NUMBER_OF_OPCODES as usize);
};
// OPCODE_NAMES[0] == "CheckException" — 字符串相等入集成测试 (const 上下文不可比)
