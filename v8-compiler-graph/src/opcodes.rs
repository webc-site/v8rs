// 由 tool/src/compiler/opcodes.h.js 从 v8/src/compiler/opcodes.h + opcodes.cc + operator-properties.cc (谓词表) 生成; 修订就地改, 复跑覆盖

//! IrOpcode — 全层级 opcode 全局唯一编号 (原版 IrOpcode::Value X-macro 展开)
//!
//! 目标构建: wasm 开 / SIMD128 开 / SIMD256 关 / 硬件沙箱关 / CPED 开。
//! 变体顺序 = ALL_OP_LIST = CONTROL + COMMON + SIMPLIFIED + MACHINE + SIMD128 + JS。
//! 每变体派生属性见 [`OPCODE_PROPS`]; 区间型谓词与表位一致性由生成脚本断言。
//! value/effect/control 输入数属 operator 实例数据 (v8-compiler-ops 层), 不在此表。
//!
//! 展开计数 (对数): 总 1036
//   CONTROL                22
//   CONSTANT               13
//   JS_LEVEL_CONSTANT      5
//   MACHINE_CONSTANT       8
//   INNER                  27
//   COMMON                 47
//   JS                     124
//   JS_COMPARE             6
//   JS_ARITH_BINOP         6
//   JS_ARITH_UNOP          2
//   JS_BITWISE_BINOP       6
//   JS_BITWISE_UNOP        2
//   JS_CONVERSION_UNOP     10
//   MACHINE                234
//   MACHINE_ATOMIC         27
//   MACHINE_COMPARE        16
//   SIMPLIFIED             322
//   SIMPLIFIED_COMPARE     20
//   SIMD128                287
//   needs_exact_context     false=69 true=30 param=2 lazy=23 (JS 全集互斥分区)

use core::{fmt, mem::transmute};

use strum::{EnumCount, EnumIter, FromRepr, IntoStaticStr};

/// IrOpcode — 原版 `enum Value { k##x }`; Rust 侧变体名去 k 前缀
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromRepr, EnumCount, EnumIter, IntoStaticStr)]
#[repr(u16)]
pub enum IrOpcode {
  // CONTROL_OP_LIST
  Start = 0,
  Loop,
  Branch,
  Switch,
  IfTrue,
  IfFalse,
  IfSuccess,
  IfException,
  IfValue,
  IfDefault,
  Merge,
  Deoptimize,
  DeoptimizeIf,
  DeoptimizeUnless,
  Assert,
  TrapIf,
  TrapUnless,
  Return,
  TailCall,
  Terminate,
  Throw,
  End,
  // COMMON_OP_LIST
  ExternalConstant,
  NumberConstant,
  PointerConstant,
  HeapConstant,
  TrustedHeapConstant,
  Int32Constant,
  Int64Constant,
  TaggedIndexConstant,
  Float32Constant,
  Float64Constant,
  CompressedHeapConstant,
  RelocatableInt32Constant,
  RelocatableInt64Constant,
  Select,
  Phi,
  EffectPhi,
  InductionVariablePhi,
  Checkpoint,
  BeginRegion,
  FinishRegion,
  FrameState,
  StateValues,
  TypedStateValues,
  ArgumentsElementsState,
  ArgumentsLengthState,
  ObjectState,
  ObjectId,
  TypedObjectState,
  Call,
  Parameter,
  OsrValue,
  LoopExit,
  LoopExitValue,
  LoopExitEffect,
  Projection,
  Retain,
  MapGuard,
  TypeGuard,
  EnterMachineGraph,
  ExitMachineGraph,
  Unreachable,
  DeadValue,
  Dead,
  Plug,
  SLVerifierHint,
  MajorGCForCompilerTesting,
  StaticAssert,
  // SIMPLIFIED_OP_LIST
  ChangeTaggedSignedToInt32,
  ChangeTaggedSignedToInt64,
  ChangeTaggedToInt32,
  ChangeTaggedToInt64,
  ChangeTaggedToUint32,
  ChangeTaggedToFloat64,
  ChangeTaggedToTaggedSigned,
  ChangeSmiOrHoleToFloat64,
  ChangeNumberOrHoleToFloat64,
  ChangeInt31ToTaggedSigned,
  ChangeInt32ToTagged,
  ChangeInt64ToTagged,
  ChangeUint32ToTagged,
  ChangeUint64ToTagged,
  ChangeFloat64ToTagged,
  ChangeFloat64ToTaggedPointer,
  ChangeFloat64OrUndefinedToTagged,
  ChangeTaggedToBit,
  ChangeBitToTagged,
  ChangeInt64ToBigInt,
  ChangeUint64ToBigInt,
  TruncateBigIntToWord64,
  TruncateNumberOrOddballToWord32,
  TruncateSmiOrHoleToWord32,
  TruncateNumberOrOddballOrHoleToWord32,
  TruncateTaggedToFloat64,
  TruncateTaggedToFloat64PreserveUndefined,
  TruncateTaggedToBit,
  TruncateTaggedPointerToBit,
  CheckedInt32Add,
  CheckedInt32Sub,
  CheckedInt32Div,
  CheckedInt32Mod,
  CheckedUint32Div,
  CheckedUint32Mod,
  CheckedInt32Mul,
  CheckedInt64Add,
  CheckedAdditiveSafeIntegerAdd,
  CheckedAdditiveSafeIntegerSub,
  CheckedInt64Sub,
  CheckedInt64Mul,
  CheckedInt64Div,
  CheckedInt64Mod,
  CheckedInt32ToTaggedSigned,
  CheckedInt64ToInt32,
  CheckedInt64ToAdditiveSafeInteger,
  CheckedInt64ToTaggedSigned,
  CheckedUint32Bounds,
  CheckedUint32ToInt32,
  CheckedUint32ToTaggedSigned,
  CheckedUint64Bounds,
  CheckedUint64ToInt32,
  CheckedUint64ToInt64,
  CheckedUint64ToTaggedSigned,
  CheckedFloat64ToInt32,
  CheckedFloat64ToInt64,
  CheckedInt32ToUint64,
  CheckedInt64ToUint64,
  CheckedFloat64ToUint64,
  CheckedFloat64ToAdditiveSafeInteger,
  CheckedTaggedSignedToInt32,
  CheckedTaggedToInt32,
  CheckedTaggedToArrayIndex,
  CheckedTruncateTaggedToWord32,
  CheckedTaggedToFloat64,
  CheckedTaggedToAdditiveSafeInteger,
  CheckedTaggedToInt64,
  CheckedTaggedToUint64,
  CheckedTaggedToTaggedSigned,
  CheckedTaggedToTaggedPointer,
  NumberEqual,
  NumberLessThan,
  NumberLessThanOrEqual,
  SpeculativeNumberEqual,
  SpeculativeNumberLessThan,
  SpeculativeNumberLessThanOrEqual,
  ReferenceEqual,
  SameValue,
  SameValueNumbersOnly,
  NumberSameValue,
  StringEqual,
  StringLessThan,
  StringLessThanOrEqual,
  StringOrOddballStrictEqual,
  BigIntEqual,
  BigIntLessThan,
  BigIntLessThanOrEqual,
  SpeculativeBigIntEqual,
  SpeculativeBigIntLessThan,
  SpeculativeBigIntLessThanOrEqual,
  NumberAdd,
  NumberSubtract,
  NumberMultiply,
  NumberDivide,
  NumberModulus,
  NumberBitwiseOr,
  NumberBitwiseXor,
  NumberBitwiseAnd,
  NumberShiftLeft,
  NumberShiftRight,
  NumberShiftRightLogical,
  NumberAtan2,
  NumberImul,
  NumberMax,
  NumberMin,
  NumberPow,
  BigIntAdd,
  BigIntSubtract,
  BigIntMultiply,
  BigIntDivide,
  BigIntModulus,
  BigIntBitwiseAnd,
  BigIntBitwiseOr,
  BigIntBitwiseXor,
  BigIntShiftLeft,
  BigIntShiftRight,
  SpeculativeNumberAdd,
  SpeculativeNumberSubtract,
  SpeculativeNumberMultiply,
  SpeculativeNumberPow,
  SpeculativeNumberDivide,
  SpeculativeNumberModulus,
  SpeculativeNumberBitwiseAnd,
  SpeculativeNumberBitwiseOr,
  SpeculativeNumberBitwiseXor,
  SpeculativeNumberShiftLeft,
  SpeculativeNumberShiftRight,
  SpeculativeNumberShiftRightLogical,
  SpeculativeAdditiveSafeIntegerAdd,
  SpeculativeAdditiveSafeIntegerSubtract,
  SpeculativeSmallIntegerAdd,
  SpeculativeSmallIntegerSubtract,
  NumberAbs,
  NumberAcos,
  NumberAcosh,
  NumberAsin,
  NumberAsinh,
  NumberAtan,
  NumberAtanh,
  NumberCbrt,
  NumberCeil,
  NumberClz32,
  NumberCos,
  NumberCosh,
  NumberExp,
  NumberExpm1,
  NumberFloor,
  NumberFround,
  NumberLog,
  NumberLog1p,
  NumberLog2,
  NumberLog10,
  NumberRound,
  NumberSign,
  NumberSin,
  NumberSinh,
  NumberSqrt,
  NumberTan,
  NumberTanh,
  NumberTrunc,
  NumberToBoolean,
  NumberToInt32,
  NumberToString,
  NumberToUint32,
  NumberToUint8Clamped,
  Integral32OrMinusZeroToBigInt,
  NumberSilenceNaN,
  BigIntNegate,
  CheckBigInt,
  CheckedBigIntToBigInt64,
  SpeculativeToNumber,
  SpeculativeBigIntAsIntN,
  SpeculativeBigIntAsUintN,
  SpeculativeBigIntNegate,
  SpeculativeToBigInt,
  SpeculativeBigIntAdd,
  SpeculativeBigIntSubtract,
  SpeculativeBigIntMultiply,
  SpeculativeBigIntDivide,
  SpeculativeBigIntModulus,
  SpeculativeBigIntBitwiseAnd,
  SpeculativeBigIntBitwiseOr,
  SpeculativeBigIntBitwiseXor,
  SpeculativeBigIntShiftLeft,
  SpeculativeBigIntShiftRight,
  AssertNotNull,
  IsNull,
  IsNotNull,
  Null,
  RttCanon,
  WasmTypeCast,
  WasmTypeCastAbstract,
  WasmTypeCheck,
  WasmTypeCheckAbstract,
  WasmAnyConvertExtern,
  WasmExternConvertAny,
  WasmStructGet,
  WasmStructSet,
  WasmArrayGet,
  WasmArraySet,
  WasmArrayLength,
  WasmArrayInitializeLength,
  StringAsWtf16,
  StringPrepareForGetCodeunit,
  Allocate,
  AllocateRaw,
  ArgumentsLength,
  AssertType,
  BooleanNot,
  ChangeFloat64HoleToTagged,
  ChangeFloat64OrUndefinedOrHoleToTagged,
  CheckBounds,
  CheckClosure,
  CheckEqualsInternalizedString,
  CheckEqualsSymbol,
  CheckFloat64Hole,
  CheckHeapObject,
  CheckIf,
  CheckInternalizedString,
  CheckMaps,
  CheckHomomorphic,
  CheckNotTaggedHole,
  CheckNumber,
  CheckNumberOrUndefined,
  CheckNumberFitsInt32,
  CheckReceiver,
  CheckReceiverOrNullOrUndefined,
  CheckSmi,
  CheckString,
  CheckStringOrStringWrapper,
  CheckStringOrOddball,
  CheckSymbol,
  CheckTurboshaftTypeOf,
  CompareMaps,
  ConvertReceiver,
  ConvertTaggedHoleToUndefined,
  DateNow,
  DoubleArrayMax,
  DoubleArrayMin,
  EnsureWritableFastElements,
  FastApiCall,
  FindOrderedHashMapEntry,
  FindOrderedHashMapEntryForInt32Key,
  FindOrderedHashSetEntry,
  WeakCollectionGet,
  InitializeImmutableInObject,
  LoadDataViewElement,
  LoadDictionaryField,
  LoadElement,
  LoadField,
  LoadFieldByIndex,
  LoadFromObject,
  LoadImmutableFromObject,
  LoadMessage,
  LoadStackArgument,
  LoadTypedElement,
  MaybeGrowFastElements,
  NewArgumentsElements,
  NewConsString,
  NewDoubleElements,
  NewSmiOrObjectElements,
  NumberIsFinite,
  NumberIsFloat64Hole,
  NumberIsInteger,
  NumberIsMinusZero,
  NumberIsNaN,
  NumberIsSafeInteger,
  ObjectIsArrayBufferView,
  ObjectIsBigInt,
  ObjectIsCallable,
  ObjectIsConstructor,
  ObjectIsDetectableCallable,
  ObjectIsFiniteNumber,
  ObjectIsInteger,
  ObjectIsMinusZero,
  ObjectIsNaN,
  ObjectIsNonCallable,
  ObjectIsNumber,
  ObjectIsReceiver,
  ObjectIsSafeInteger,
  ObjectIsSmi,
  ObjectIsString,
  ObjectIsSymbol,
  ObjectIsUndetectable,
  PlainPrimitiveToFloat64,
  PlainPrimitiveToNumber,
  PlainPrimitiveToWord32,
  RestLength,
  RuntimeAbort,
  StoreDataViewElement,
  StoreElement,
  StoreField,
  StoreMessage,
  StoreSignedSmallElement,
  StoreToObject,
  StoreTypedElement,
  StringCharCodeAt,
  StringCodePointAt,
  StringConcat,
  StringFromCodePointAt,
  StringFromSingleCharCode,
  StringFromSingleCodePoint,
  StringIndexOf,
  StringLength,
  StringWrapperLength,
  StringSubstring,
  StringLocaleCompareIntl,
  StringToLowerCaseIntl,
  StringToNumber,
  StringToUpperCaseIntl,
  ToBoolean,
  TransitionAndStoreElement,
  TransitionAndStoreNonNumberElement,
  TransitionAndStoreNumberElement,
  TransitionElementsKind,
  TransitionElementsKindOrCheckMap,
  TypedArrayLength,
  TypeOf,
  Unsigned32Divide,
  VerifyType,
  GetContinuationPreservedEmbedderData,
  SetContinuationPreservedEmbedderData,
  // MACHINE_OP_LIST
  Word32Clz,
  Word32Ctz,
  Int32AbsWithOverflow,
  Word32ReverseBits,
  Word32ReverseBytes,
  Word32And,
  Word32Or,
  Word32Xor,
  Word32Shl,
  Word32Shr,
  Word32Sar,
  Word32Rol,
  Word32Ror,
  Int32Add,
  Int32AddWithOverflow,
  Int32Sub,
  Int32SubWithOverflow,
  Int32Mul,
  Int32MulWithOverflow,
  Int32MulHigh,
  Int32Div,
  Int32Mod,
  Uint32Div,
  Uint32Mod,
  Uint32MulHigh,
  Word64And,
  Word64Or,
  Word64Xor,
  Word64Shl,
  Word64Shr,
  Word64Sar,
  Word64Rol,
  Word64Ror,
  Word64RolLowerable,
  Word64RorLowerable,
  Int64Add,
  Int64AddWithOverflow,
  Int64Sub,
  Int64SubWithOverflow,
  Int64Mul,
  Int64MulHigh,
  Int64MulWithOverflow,
  Int64Div,
  Int64Mod,
  Uint64Div,
  Uint64Mod,
  Uint64MulHigh,
  Word32Equal,
  Word64Equal,
  Int32LessThan,
  Int32LessThanOrEqual,
  Uint32LessThan,
  Uint32LessThanOrEqual,
  Int64LessThan,
  Int64LessThanOrEqual,
  Uint64LessThan,
  Uint64LessThanOrEqual,
  Float32Equal,
  Float32LessThan,
  Float32LessThanOrEqual,
  Float64Equal,
  Float64LessThan,
  Float64LessThanOrEqual,
  Float32Add,
  Float32Sub,
  Float32Mul,
  Float32Div,
  Float32Max,
  Float32Min,
  Float32Abs,
  Float32Neg,
  Float32RoundDown,
  Float32RoundTiesEven,
  Float32RoundTruncate,
  Float32RoundUp,
  Float32Sqrt,
  Float64Atan2,
  Float64Max,
  Float64Min,
  Float64Add,
  Float64Sub,
  Float64Mul,
  Float64Div,
  Float64Mod,
  Float64Pow,
  Float64Abs,
  Float64Acos,
  Float64Acosh,
  Float64Asin,
  Float64Asinh,
  Float64Atan,
  Float64Atanh,
  Float64Cbrt,
  Float64Cos,
  Float64Cosh,
  Float64Exp,
  Float64Expm1,
  Float64Log,
  Float64Log1p,
  Float64Log10,
  Float64Log2,
  Float64Neg,
  Float64RoundDown,
  Float64RoundTiesAway,
  Float64RoundTiesEven,
  Float64RoundTruncate,
  Float64RoundUp,
  Float64Sin,
  Float64Sinh,
  Float64Sqrt,
  Float64Tan,
  Float64Tanh,
  Word32AtomicLoad,
  Word32AtomicStore,
  Word32AtomicExchange,
  Word32AtomicCompareExchange,
  Word32AtomicAdd,
  Word32AtomicSub,
  Word32AtomicAnd,
  Word32AtomicOr,
  Word32AtomicXor,
  Word32AtomicPairLoad,
  Word32AtomicPairStore,
  Word32AtomicPairAdd,
  Word32AtomicPairSub,
  Word32AtomicPairAnd,
  Word32AtomicPairOr,
  Word32AtomicPairXor,
  Word32AtomicPairExchange,
  Word32AtomicPairCompareExchange,
  Word64AtomicLoad,
  Word64AtomicStore,
  Word64AtomicAdd,
  Word64AtomicSub,
  Word64AtomicAnd,
  Word64AtomicOr,
  Word64AtomicXor,
  Word64AtomicExchange,
  Word64AtomicCompareExchange,
  AbortCSADcheck,
  DebugBreak,
  Comment,
  Load,
  LoadImmutable,
  Store,
  StorePair,
  StoreIndirectPointer,
  StackSlot,
  Word32Popcnt,
  Word64Popcnt,
  Word64Clz,
  Word64Ctz,
  Word64ClzLowerable,
  Word64CtzLowerable,
  Word64ReverseBits,
  Word64ReverseBytes,
  Simd128ReverseBytes,
  Int64AbsWithOverflow,
  BitcastTaggedToWord,
  BitcastTaggedToWordForTagAndSmiBits,
  BitcastWordToTagged,
  BitcastWordToTaggedSigned,
  TruncateFloat64ToWord32,
  ChangeFloat32ToFloat64,
  ChangeFloat64ToInt32,
  ChangeFloat64ToInt64,
  ChangeFloat64ToUint32,
  ChangeFloat64ToUint64,
  Float64SilenceNaN,
  TruncateFloat64ToInt64,
  TruncateFloat64ToUint32,
  TruncateFloat32ToInt32,
  TruncateFloat32ToUint32,
  TryTruncateFloat32ToInt64,
  TryTruncateFloat64ToInt64,
  TryTruncateFloat32ToUint64,
  TryTruncateFloat64ToUint64,
  TryTruncateFloat64ToInt32,
  TryTruncateFloat64ToUint32,
  ChangeInt32ToFloat64,
  BitcastWord32ToWord64,
  ChangeInt32ToInt64,
  ChangeInt64ToFloat64,
  ChangeUint32ToFloat64,
  ChangeUint32ToUint64,
  ChangeFloat16RawBitsToFloat64,
  TruncateFloat64ToFloat32,
  TruncateFloat64ToFloat16RawBits,
  TruncateInt64ToInt32,
  RoundFloat64ToInt32,
  RoundInt32ToFloat32,
  RoundInt64ToFloat32,
  RoundInt64ToFloat64,
  RoundUint32ToFloat32,
  RoundUint64ToFloat32,
  RoundUint64ToFloat64,
  BitcastFloat32ToInt32,
  BitcastFloat64ToInt64,
  BitcastInt32ToFloat32,
  BitcastInt64ToFloat64,
  Float64ExtractLowWord32,
  Float64ExtractHighWord32,
  Float64InsertLowWord32,
  Float64InsertHighWord32,
  Word32Select,
  Word64Select,
  Float32Select,
  Float64Select,
  LoadStackCheckOffset,
  LoadFramePointer,
  LoadStackPointer,
  SetStackPointer,
  LoadParentFramePointer,
  LoadRootRegister,
  UnalignedLoad,
  UnalignedStore,
  Int32PairAdd,
  Int32PairSub,
  Int32PairMul,
  Word32PairShl,
  Word32PairShr,
  Word32PairSar,
  TrappingLoad,
  TrappingStore,
  LoadTrapOnNull,
  StoreTrapOnNull,
  MemoryBarrier,
  SignExtendWord8ToInt32,
  SignExtendWord16ToInt32,
  SignExtendWord8ToInt64,
  SignExtendWord16ToInt64,
  SignExtendWord32ToInt64,
  StackPointerGreaterThan,
  TraceInstruction,
  // MACHINE_SIMD128_OP_LIST
  F64x2Splat,
  F64x2ExtractLane,
  F64x2ReplaceLane,
  F64x2MoveLane,
  F64x2Abs,
  F64x2Neg,
  F64x2Sqrt,
  F64x2Add,
  F64x2Sub,
  F64x2Mul,
  F64x2Div,
  F64x2Min,
  F64x2Max,
  F64x2Eq,
  F64x2Ne,
  F64x2Lt,
  F64x2Le,
  F64x2Qfma,
  F64x2Qfms,
  F64x2Pmin,
  F64x2Pmax,
  F64x2Ceil,
  F64x2Floor,
  F64x2Trunc,
  F64x2NearestInt,
  F64x2ConvertLowI32x4S,
  F64x2ConvertLowI32x4U,
  F64x2PromoteLowF32x4,
  F32x4Splat,
  F32x4ExtractLane,
  F32x4ReplaceLane,
  F32x4MoveLane,
  F32x4SConvertI32x4,
  F32x4UConvertI32x4,
  F32x4Abs,
  F32x4Neg,
  F32x4Sqrt,
  F32x4Add,
  F32x4Sub,
  F32x4Mul,
  F32x4Div,
  F32x4Min,
  F32x4Max,
  F32x4Eq,
  F32x4Ne,
  F32x4Lt,
  F32x4Le,
  F32x4Gt,
  F32x4Ge,
  F32x4Qfma,
  F32x4Qfms,
  F32x4Pmin,
  F32x4Pmax,
  F32x4Ceil,
  F32x4Floor,
  F32x4Trunc,
  F32x4NearestInt,
  F32x4DemoteF64x2Zero,
  F16x8Splat,
  F16x8ExtractLane,
  F16x8ReplaceLane,
  F16x8MoveLane,
  F16x8Abs,
  F16x8Neg,
  F16x8Sqrt,
  F16x8Ceil,
  F16x8Floor,
  F16x8Trunc,
  F16x8NearestInt,
  F16x8Add,
  F16x8Sub,
  F16x8Mul,
  F16x8Div,
  F16x8Min,
  F16x8Max,
  F16x8Pmin,
  F16x8Pmax,
  F16x8Eq,
  F16x8Ne,
  F16x8Lt,
  F16x8Le,
  F16x8Gt,
  F16x8Ge,
  I16x8SConvertF16x8,
  I16x8UConvertF16x8,
  F16x8SConvertI16x8,
  F16x8UConvertI16x8,
  F16x8DemoteF32x4Zero,
  F16x8DemoteF64x2Zero,
  F32x4PromoteLowF16x8,
  F16x8Qfma,
  F16x8Qfms,
  I64x2Splat,
  I64x2SplatI32Pair,
  I64x2ExtractLane,
  I64x2ReplaceLane,
  I64x2MoveLane,
  I64x2ReplaceLaneI32Pair,
  I64x2Abs,
  I64x2Neg,
  I64x2SConvertI32x4Low,
  I64x2SConvertI32x4High,
  I64x2UConvertI32x4Low,
  I64x2UConvertI32x4High,
  I64x2BitMask,
  I64x2Shl,
  I64x2ShrS,
  I64x2Add,
  I64x2Sub,
  I64x2Mul,
  I64x2Eq,
  I64x2Ne,
  I64x2GtS,
  I64x2GeS,
  I64x2ShrU,
  I64x2ExtMulLowI32x4S,
  I64x2ExtMulHighI32x4S,
  I64x2ExtMulLowI32x4U,
  I64x2ExtMulHighI32x4U,
  I32x4Splat,
  I32x4ExtractLane,
  I32x4ReplaceLane,
  I32x4MoveLane,
  I32x4SConvertF32x4,
  I32x4SConvertI16x8Low,
  I32x4SConvertI16x8High,
  I32x4Neg,
  I32x4Shl,
  I32x4ShrS,
  I32x4Add,
  I32x4Sub,
  I32x4Mul,
  I32x4MinS,
  I32x4MaxS,
  I32x4Eq,
  I32x4Ne,
  I32x4LtS,
  I32x4LeS,
  I32x4GtS,
  I32x4GeS,
  I32x4UConvertF32x4,
  I32x4UConvertI16x8Low,
  I32x4UConvertI16x8High,
  I32x4ShrU,
  I32x4MinU,
  I32x4MaxU,
  I32x4LtU,
  I32x4LeU,
  I32x4GtU,
  I32x4GeU,
  I32x4Abs,
  I32x4BitMask,
  I32x4DotI16x8S,
  I32x4DotI8x16S,
  I32x4AddPairwise,
  I32x4ExtMulLowI16x8S,
  I32x4ExtMulHighI16x8S,
  I32x4ExtMulLowI16x8U,
  I32x4ExtMulHighI16x8U,
  I32x4ExtAddPairwiseI16x8S,
  I32x4ExtAddPairwiseI16x8U,
  I32x4TruncSatF64x2SZero,
  I32x4TruncSatF64x2UZero,
  I16x8Splat,
  I16x8ExtractLaneU,
  I16x8ExtractLaneS,
  I16x8ReplaceLane,
  I16x8MoveLane,
  I16x8SConvertI8x16Low,
  I16x8SConvertI8x16High,
  I16x8Neg,
  I16x8Shl,
  I16x8ShrS,
  I16x8SConvertI32x4,
  I16x8Add,
  I16x8AddSatS,
  I16x8Sub,
  I16x8SubSatS,
  I16x8Mul,
  I16x8MinS,
  I16x8MaxS,
  I16x8Eq,
  I16x8Ne,
  I16x8LtS,
  I16x8LeS,
  I16x8GtS,
  I16x8GeS,
  I16x8UConvertI8x16Low,
  I16x8UConvertI8x16High,
  I16x8ShrU,
  I16x8UConvertI32x4,
  I16x8AddSatU,
  I16x8SubSatU,
  I16x8MinU,
  I16x8MaxU,
  I16x8LtU,
  I16x8LeU,
  I16x8GtU,
  I16x8GeU,
  I16x8RoundingAverageU,
  I16x8Q15MulRSatS,
  I16x8Abs,
  I16x8BitMask,
  I16x8ExtMulLowI8x16S,
  I16x8ExtMulHighI8x16S,
  I16x8ExtMulLowI8x16U,
  I16x8ExtMulHighI8x16U,
  I16x8ExtAddPairwiseI8x16S,
  I16x8ExtAddPairwiseI8x16U,
  I8x16Splat,
  I8x16ExtractLaneU,
  I8x16ExtractLaneS,
  I8x16ReplaceLane,
  I8x16MoveLane,
  I8x16SConvertI16x8,
  I8x16Neg,
  I8x16Shl,
  I8x16ShrS,
  I8x16Add,
  I8x16AddSatS,
  I8x16Sub,
  I8x16SubSatS,
  I8x16MinS,
  I8x16MaxS,
  I8x16Eq,
  I8x16Ne,
  I8x16LtS,
  I8x16LeS,
  I8x16GtS,
  I8x16GeS,
  I8x16UConvertI16x8,
  I8x16AddSatU,
  I8x16SubSatU,
  I8x16ShrU,
  I8x16MinU,
  I8x16MaxU,
  I8x16LtU,
  I8x16LeU,
  I8x16GtU,
  I8x16GeU,
  I8x16RoundingAverageU,
  I8x16Popcnt,
  I8x16Abs,
  I8x16BitMask,
  S128Zero,
  S128Const,
  S128Not,
  S128And,
  S128Or,
  S128Xor,
  S128Select,
  S128AndNot,
  I8x16Swizzle,
  I8x16RelaxedLaneSelect,
  I16x8RelaxedLaneSelect,
  I32x4RelaxedLaneSelect,
  I64x2RelaxedLaneSelect,
  F32x4RelaxedMin,
  F32x4RelaxedMax,
  F64x2RelaxedMin,
  F64x2RelaxedMax,
  I32x4RelaxedTruncF32x4S,
  I32x4RelaxedTruncF32x4U,
  I32x4RelaxedTruncF64x2SZero,
  I32x4RelaxedTruncF64x2UZero,
  I16x8RelaxedQ15MulRS,
  I16x8DotI8x16I7x16S,
  I32x4DotI8x16I7x16AddS,
  I8x16AddReduce,
  I16x8AddReduce,
  I32x4AddReduce,
  I64x2AddReduce,
  F32x4AddReduce,
  F64x2AddReduce,
  I8x16Shuffle,
  I8x8Shuffle,
  I8x4Shuffle,
  I8x2Shuffle,
  I8x1Shuffle,
  V128AnyTrue,
  I64x2AllTrue,
  I32x4AllTrue,
  I16x8AllTrue,
  I8x16AllTrue,
  LoadTransform,
  LoadLane,
  StoreLane,
  // JS_OP_LIST
  JSEqual,
  JSStrictEqual,
  JSLessThan,
  JSGreaterThan,
  JSLessThanOrEqual,
  JSGreaterThanOrEqual,
  JSBitwiseOr,
  JSBitwiseXor,
  JSBitwiseAnd,
  JSShiftLeft,
  JSShiftRight,
  JSShiftRightLogical,
  JSAdd,
  JSSubtract,
  JSMultiply,
  JSDivide,
  JSModulus,
  JSExponentiate,
  JSHasInPrototypeChain,
  JSInstanceOf,
  JSOrdinaryHasInstance,
  JSDecrement,
  JSIncrement,
  JSBitwiseNot,
  JSNegate,
  JSToLength,
  JSToName,
  JSToNumber,
  JSToNumberConvertBigInt,
  JSToBigInt,
  JSToBigIntConvertNumber,
  JSToNumeric,
  JSToObject,
  JSToString,
  JSParseInt,
  JSCloneObject,
  JSCreate,
  JSCreateArguments,
  JSCreateArray,
  JSCreateArrayFromIterable,
  JSCreateArrayIterator,
  JSCreateAsyncFunctionObject,
  JSCreateBoundFunction,
  JSCreateClosure,
  JSCreateCollectionIterator,
  JSCreateEmptyLiteralArray,
  JSCreateEmptyLiteralObject,
  JSCreateGeneratorObject,
  JSCreateIterResultObject,
  JSCreateKeyValueArray,
  JSCreateLiteralArray,
  JSCreateLiteralObject,
  JSCreateLiteralRegExp,
  JSCreateObject,
  JSCreateStringWrapper,
  JSCreatePromise,
  JSCreateStringIterator,
  JSCreateTypedArray,
  JSGetTemplateObject,
  JSLoadProperty,
  JSLoadNamed,
  JSLoadNamedFromSuper,
  JSLoadGlobal,
  JSSetKeyedProperty,
  JSDefineKeyedOwnProperty,
  JSSetNamedProperty,
  JSSetPrototypeProperties,
  JSDefineNamedOwnProperty,
  JSStoreGlobal,
  JSDefineKeyedOwnPropertyInLiteral,
  JSStoreInArrayLiteral,
  JSDeleteProperty,
  JSHasProperty,
  JSGetSuperConstructor,
  JSFindNonDefaultConstructorOrConstruct,
  JSHasContextExtension,
  JSLoadContextNoCell,
  JSLoadContext,
  JSStoreContextNoCell,
  JSStoreContext,
  JSCreateFunctionContext,
  JSCreateCatchContext,
  JSCreateWithContext,
  JSCreateBlockContext,
  JSCall,
  JSCallForwardVarargs,
  JSCallWithArrayLike,
  JSCallWithSpread,
  JSWasmCall,
  JSConstructForwardVarargs,
  JSConstructForwardAllArgs,
  JSConstruct,
  JSConstructWithArrayLike,
  JSConstructWithSpread,
  JSAsyncFunctionAwait,
  JSAsyncFunctionEnter,
  JSAsyncFunctionReject,
  JSAsyncFunctionResolve,
  JSCallRuntime,
  JSDetachContextCell,
  JSForInEnumerate,
  JSForInNext,
  JSForInPrepare,
  JSGetIterator,
  JSForOfNext,
  JSLoadMessage,
  JSStoreMessage,
  JSLoadModule,
  JSStoreModule,
  JSGetImportMeta,
  JSGeneratorStore,
  JSGeneratorRestoreContinuation,
  JSGeneratorRestoreContextNoCell,
  JSGeneratorRestoreRegister,
  JSGeneratorRestoreInputOrDebugPos,
  JSFulfillPromise,
  JSPerformPromiseThen,
  JSPromiseResolve,
  JSRejectPromise,
  JSResolvePromise,
  JSStackCheck,
  JSObjectIsArray,
  JSRegExpTest,
  JSDebugger,
}

/// NeedsExactContext 的 opcode 面分解 (operator-properties.cc:23-143)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactContext {
  /// 任意 context 即可 (二元/一元/调用/构造 + 显式组)
  AnyContext,
  /// 需要精确 context
  Exact,
  /// 需 operator 参数 (JSCallRuntime→Runtime 表 / JSCreateArguments→参数)
  NeedsParam,
  /// 未登记 JS 码 (原版 break→UNREACHABLE)
  Unreachable,
  /// 非 JS 码 (原版 UNREACHABLE)
  NotJs,
}

impl IrOpcode {
  /// 原版 kLast = 展开计数 - 1
  pub const LAST: Self = Self::JSDebugger;
  /// 原版 kLast 的裸值 (operator opcode 域上界)
  pub const LAST_U16: u16 = 1035;

  /// 原版 static_cast<Value>(opcode) 的无检查形态 (node.opcode 热路径)
  ///
  /// # Safety
  /// v 必须 <= [`LAST_U16`] (Operator 构造已校验)
  #[inline]
  pub const unsafe fn from_repr_unchecked(v: u16) -> Self {
    // SAFETY: repr(u16) 判别值连续 0..=LAST_U16, transmute 值域合法
    unsafe { transmute(v) }
  }

  /// 原版 IrOpcode::Mnemonic — 变体名字符串 (opcodes.cc kMnemonics)
  #[inline]
  pub fn mnemonic(self) -> &'static str {
    self.into()
  }
}

impl fmt::Display for IrOpcode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.mnemonic())
  }
}

impl IrOpcode {
  /// 原版 Mnemonic 的越界钳制形态 (kMnemonics 末位 "UnknownOpcode")
  pub fn mnemonic_of(v: u16) -> &'static str {
    Self::from_repr(v).map_or(Self::UNKNOWN, Into::into)
  }

  /// 原版 IsCommonOpcode: kStart <= v <= kStaticAssert (control + common 区间)
  #[inline]
  pub fn is_common(self) -> bool {
    (Self::Start as u16..=Self::StaticAssert as u16).contains(&(self as u16))
  }

  /// 原版 IsControlOpcode: kStart <= v <= kEnd
  #[inline]
  pub fn is_control(self) -> bool {
    (Self::Start as u16..=Self::End as u16).contains(&(self as u16))
  }

  /// 原版 IsJsOpcode: kJSEqual <= v <= kJSDebugger (⟺ 有 context 输入)
  #[inline]
  pub fn is_js(self) -> bool {
    (Self::JSEqual as u16..=Self::JSDebugger as u16).contains(&(self as u16))
  }

  /// 原版 IsMachineOpcode: kWord32Clz <= v <= kTraceInstruction
  #[inline]
  pub fn is_machine(self) -> bool {
    (Self::Word32Clz as u16..=Self::TraceInstruction as u16).contains(&(self as u16))
  }

  /// 原版 IsSimd128Opcode (wasm 开构建)
  #[inline]
  pub fn is_simd128(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_SIMD128 != 0
  }

  /// 原版 IsMachineConstantOpcode
  #[inline]
  pub fn is_machine_constant(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_MACHINE_CONSTANT != 0
  }

  /// 原版 IsConstantOpcode
  #[inline]
  pub fn is_constant(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_CONSTANT != 0
  }

  /// 原版 IsPhiOpcode
  #[inline]
  pub fn is_phi(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_PHI != 0
  }

  /// 原版 IsMergeOpcode
  #[inline]
  pub fn is_merge(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_MERGE != 0
  }

  /// 原版 IsIfProjectionOpcode: kIfTrue <= v <= kIfDefault
  #[inline]
  pub fn is_if_projection(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_IF_PROJECTION != 0
  }

  /// 原版 IsGraphTerminator
  #[inline]
  pub fn is_graph_terminator(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_GRAPH_TERMINATOR != 0
  }

  /// 原版 IsInlineeOpcode
  #[inline]
  pub fn is_inlinee(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_INLINEE != 0
  }

  /// 原版 IsComparisonOpcode
  #[inline]
  pub fn is_comparison(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_COMPARISON != 0
  }

  /// 原版 IsContextChainExtendingOpcode
  #[inline]
  pub fn is_context_chain_extending(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_CONTEXT_CHAIN_EXTENDING != 0
  }

  /// 原版 IsFeedbackCollectingOpcode
  #[inline]
  pub fn is_feedback_collecting(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_FEEDBACK != 0
  }

  /// 原版 isAtomicOpOpcode
  #[inline]
  pub fn is_atomic_op(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_ATOMIC != 0
  }

  /// OperatorProperties::HasContextInput 的 opcode 面 (⟺ is_js)
  #[inline]
  pub fn has_context_input(self) -> bool {
    self.is_js()
  }

  /// OperatorProperties::HasFrameStateInput 的 opcode 面;
  /// JSCallRuntime 依赖 CallRuntime 参数 (Linkage 表), 由调用方参数化判定
  #[inline]
  pub fn has_frame_state_input(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_FRAME_STATE_INPUT != 0
  }

  /// JSCallRuntime: HasFrameStateInput 需查 CallRuntime 参数
  #[inline]
  pub fn frame_state_input_needs_param(self) -> bool {
    self == Self::JSCallRuntime
  }

  /// OperatorProperties::IsBasicBlockBegin 的 opcode 面
  #[inline]
  pub fn is_basic_block_begin(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_BASIC_BLOCK_BEGIN != 0
  }

  /// OperatorProperties::NeedsExactContext 的 opcode 面 (JS 全集四组互斥分解)
  ///
  /// AnyContext(69) | Exact(30) | NeedsParam(2) | Unreachable(23);
  /// 非 JS 码原版走 UNREACHABLE → NotJs
  #[inline]
  pub const fn exact_context(self) -> ExactContext {
    let p = OPCODE_PROPS[self as usize];
    if p & OP_EXACT_CONTEXT_PARAM != 0 {
      ExactContext::NeedsParam
    } else if p & OP_EXACT_CONTEXT_TRUE != 0 {
      ExactContext::Exact
    } else if p & OP_EXACT_CONTEXT_LAZY != 0 {
      ExactContext::Unreachable
    } else if p & OP_JS != 0 {
      ExactContext::AnyContext
    } else {
      ExactContext::NotJs
    }
  }

  /// 未知 opcode 的助记符 (原版 kMnemonics 槽位 "UnknownOpcode")
  pub const UNKNOWN: &'static str = "UnknownOpcode";
}

// —— OPCODE_PROPS 表的派生属性位标志 ——
/// CONTROL_OP_LIST 成员 (is_control 区间 [Start,End])
pub const OP_CONTROL: u32 = 1;
/// IsCommonOpcode 区间 [Start,StaticAssert] = COMMON_OP_LIST ∪ CONTROL_OP_LIST
pub const OP_COMMON: u32 = 2;
/// SIMPLIFIED_OP_LIST 成员
pub const OP_SIMPLIFIED: u32 = 4;
/// MACHINE_OP_LIST 成员 (is_machine 区间 [Word32Clz,TraceInstruction])
pub const OP_MACHINE: u32 = 8;
/// MACHINE_SIMD128_OP_LIST 成员 (wasm 开构建)
pub const OP_SIMD128: u32 = 16;
/// JS_OP_LIST 成员 (is_js 区间 [JSEqual,JSDebugger]; 有 context 输入)
pub const OP_JS: u32 = 32;
/// CONSTANT_OP_LIST 成员
pub const OP_CONSTANT: u32 = 64;
/// MACHINE_LEVEL_CONSTANT_OP_LIST 成员
pub const OP_MACHINE_CONSTANT: u32 = 128;
/// JS/SIMPLIFIED/MACHINE 三表 COMPARE_BINOP 并集
pub const OP_COMPARISON: u32 = 256;
/// IsFeedbackCollectingOpcode 表
pub const OP_FEEDBACK: u32 = 512;
/// MACHINE_ATOMIC_OP_LIST 成员
pub const OP_ATOMIC: u32 = 1024;
/// Phi | EffectPhi
pub const OP_PHI: u32 = 2048;
/// Merge | Loop
pub const OP_MERGE: u32 = 4096;
/// IfTrue..IfDefault 区间
pub const OP_IF_PROJECTION: u32 = 8192;
/// Deoptimize|Return|TailCall|Terminate|Throw
pub const OP_GRAPH_TERMINATOR: u32 = 16384;
/// JSConstruct | JSCall
pub const OP_INLINEE: u32 = 32768;
/// JSCreateFunctionContext..JSCreateBlockContext 区间
pub const OP_CONTEXT_CHAIN_EXTENDING: u32 = 65536;
/// OperatorProperties::HasFrameStateInput 真 (JSCallRuntime 参数化, 除外)
pub const OP_FRAME_STATE_INPUT: u32 = 131072;
/// OperatorProperties::IsBasicBlockBegin 真
pub const OP_BASIC_BLOCK_BEGIN: u32 = 262144;
/// OperatorProperties::NeedsExactContext 真组
pub const OP_EXACT_CONTEXT_TRUE: u32 = 524288;
/// NeedsExactContext 参数化组 (JSCallRuntime|JSCreateArguments)
pub const OP_EXACT_CONTEXT_PARAM: u32 = 1048576;
/// NeedsExactContext break→UNREACHABLE 组
pub const OP_EXACT_CONTEXT_LAZY: u32 = 2097152;

/// 每变体派生属性表 — 与枚举判别值同序 (索引即 opcode)
pub static OPCODE_PROPS: [u32; IrOpcode::COUNT] = [
  0x40003, 0x41003, 0x00003, 0x00003, 0x42003, 0x42003, // Start..IfFalse
  0x42003, 0x42003, 0x42003, 0x42003, 0x41003, 0x04003, // IfSuccess..Deoptimize
  0x00003, 0x00003, 0x00003, 0x00003, 0x00003, 0x04003, // DeoptimizeIf..Return
  0x04003, 0x04003, 0x04003, 0x40003, 0x00042, 0x00042, // TailCall..NumberConstant
  0x00042, 0x00042, 0x00042, 0x000c2, 0x000c2,
  0x000c2, // PointerConstant..TaggedIndexConstant
  0x000c2, 0x000c2, 0x000c2, 0x000c2, 0x000c2, 0x00002, // Float32Constant..Select
  0x00802, 0x00802, 0x00002, 0x20002, 0x00002, 0x00002, // Phi..FinishRegion
  0x20002, 0x00002, 0x00002, 0x00002, 0x00002, 0x00002, // FrameState..ObjectState
  0x00002, 0x00002, 0x00002, 0x00002, 0x00002, 0x00002, // ObjectId..LoopExit
  0x00002, 0x00002, 0x00002, 0x00002, 0x00002, 0x00002, // LoopExitValue..TypeGuard
  0x00002, 0x00002, 0x00002, 0x00002, 0x40002, 0x00002, // EnterMachineGraph..Plug
  0x00002, 0x00002, 0x00002, 0x00004, 0x00004, 0x00004, // SLVerifierHint..ChangeTaggedToInt32
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // ChangeTaggedToInt64..ChangeNumberOrHoleToFloat64
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // ChangeInt31ToTaggedSigned..ChangeFloat64ToTagged
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // ChangeFloat64ToTaggedPointer..ChangeUint64ToBigInt
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // TruncateBigIntToWord64..TruncateTaggedToFloat64PreserveUndefined
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // TruncateTaggedToBit..CheckedInt32Mod
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // CheckedUint32Div..CheckedAdditiveSafeIntegerSub
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // CheckedInt64Sub..CheckedInt64ToInt32
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // CheckedInt64ToAdditiveSafeInteger..CheckedUint64Bounds
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // CheckedUint64ToInt32..CheckedInt32ToUint64
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // CheckedInt64ToUint64..CheckedTaggedToArrayIndex
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // CheckedTruncateTaggedToWord32..CheckedTaggedToTaggedSigned
  0x00004, 0x00104, 0x00104, 0x00104, 0x00104,
  0x00104, // CheckedTaggedToTaggedPointer..SpeculativeNumberLessThan
  0x00104, 0x00104, 0x00104, 0x00104, 0x00104,
  0x00104, // SpeculativeNumberLessThanOrEqual..StringEqual
  0x00104, 0x00104, 0x00104, 0x00104, 0x00104,
  0x00104, // StringLessThan..BigIntLessThanOrEqual
  0x00104, 0x00104, 0x00104, 0x00004, 0x00004,
  0x00004, // SpeculativeBigIntEqual..NumberMultiply
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // NumberDivide..NumberShiftLeft
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // NumberShiftRight..NumberMin
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // NumberPow..BigIntModulus
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // BigIntBitwiseAnd..SpeculativeNumberAdd
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // SpeculativeNumberSubtract..SpeculativeNumberBitwiseAnd
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // SpeculativeNumberBitwiseOr..SpeculativeAdditiveSafeIntegerAdd
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // SpeculativeAdditiveSafeIntegerSubtract..NumberAcosh
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // NumberAsin..NumberCeil
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // NumberClz32..NumberFloor
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // NumberFround..NumberRound
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // NumberSign..NumberTanh
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // NumberTrunc..NumberToUint8Clamped
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // Integral32OrMinusZeroToBigInt..SpeculativeToNumber
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // SpeculativeBigIntAsIntN..SpeculativeBigIntSubtract
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // SpeculativeBigIntMultiply..SpeculativeBigIntBitwiseXor
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // SpeculativeBigIntShiftLeft..Null
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // RttCanon..WasmAnyConvertExtern
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // WasmExternConvertAny..WasmArrayLength
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // WasmArrayInitializeLength..ArgumentsLength
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // AssertType..CheckClosure
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // CheckEqualsInternalizedString..CheckInternalizedString
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // CheckMaps..CheckNumberFitsInt32
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // CheckReceiver..CheckStringOrOddball
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // CheckSymbol..DateNow
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // DoubleArrayMax..FindOrderedHashMapEntryForInt32Key
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // FindOrderedHashSetEntry..LoadElement
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // LoadField..LoadStackArgument
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // LoadTypedElement..NewSmiOrObjectElements
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // NumberIsFinite..NumberIsSafeInteger
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // ObjectIsArrayBufferView..ObjectIsFiniteNumber
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // ObjectIsInteger..ObjectIsReceiver
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // ObjectIsSafeInteger..PlainPrimitiveToFloat64
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // PlainPrimitiveToNumber..StoreElement
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // StoreField..StringCharCodeAt
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // StringCodePointAt..StringIndexOf
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004, 0x00004, // StringLength..StringToNumber
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // StringToUpperCaseIntl..TransitionElementsKind
  0x00004, 0x00004, 0x00004, 0x00004, 0x00004,
  0x00004, // TransitionElementsKindOrCheckMap..GetContinuationPreservedEmbedderData
  0x00004, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // SetContinuationPreservedEmbedderData..Word32ReverseBytes
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Word32And..Word32Sar
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Word32Rol..Int32SubWithOverflow
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Int32Mul..Uint32Div
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Uint32Mod..Word64Shl
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Word64Shr..Word64RorLowerable
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Int64Add..Int64MulHigh
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Int64MulWithOverflow..Uint64MulHigh
  0x00108, 0x00108, 0x00108, 0x00108, 0x00108, 0x00108, // Word32Equal..Uint32LessThanOrEqual
  0x00108, 0x00108, 0x00108, 0x00108, 0x00108, 0x00108, // Int64LessThan..Float32LessThan
  0x00108, 0x00108, 0x00108, 0x00108, 0x00008, 0x00008, // Float32LessThanOrEqual..Float32Sub
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Float32Mul..Float32Neg
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Float32RoundDown..Float64Atan2
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Float64Max..Float64Div
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Float64Mod..Float64Asin
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Float64Asinh..Float64Cosh
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Float64Exp..Float64Log2
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Float64Neg..Float64RoundUp
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00408, // Float64Sin..Word32AtomicLoad
  0x00408, 0x00408, 0x00408, 0x00408, 0x00408, 0x00408, // Word32AtomicStore..Word32AtomicAnd
  0x00408, 0x00408, 0x00408, 0x00408, 0x00408, 0x00408, // Word32AtomicOr..Word32AtomicPairSub
  0x00408, 0x00408, 0x00408, 0x00408, 0x00408,
  0x00408, // Word32AtomicPairAnd..Word64AtomicLoad
  0x00408, 0x00408, 0x00408, 0x00408, 0x00408, 0x00408, // Word64AtomicStore..Word64AtomicXor
  0x00408, 0x00408, 0x00008, 0x00008, 0x00008, 0x00008, // Word64AtomicExchange..Load
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // LoadImmutable..Word32Popcnt
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Word64Popcnt..Word64ReverseBits
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // Word64ReverseBytes..BitcastWordToTagged
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // BitcastWordToTaggedSigned..ChangeFloat64ToUint32
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // ChangeFloat64ToUint64..TruncateFloat32ToUint32
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // TryTruncateFloat32ToInt64..TryTruncateFloat64ToUint32
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // ChangeInt32ToFloat64..ChangeUint32ToUint64
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // ChangeFloat16RawBitsToFloat64..RoundInt32ToFloat32
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // RoundInt64ToFloat32..BitcastFloat32ToInt32
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // BitcastFloat64ToInt64..Float64InsertLowWord32
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // Float64InsertHighWord32..LoadStackCheckOffset
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // LoadFramePointer..UnalignedLoad
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // UnalignedStore..Word32PairShr
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008, 0x00008, // Word32PairSar..MemoryBarrier
  0x00008, 0x00008, 0x00008, 0x00008, 0x00008,
  0x00008, // SignExtendWord8ToInt32..StackPointerGreaterThan
  0x00008, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // TraceInstruction..F64x2Abs
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F64x2Neg..F64x2Div
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F64x2Min..F64x2Le
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F64x2Qfma..F64x2Floor
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F64x2Trunc..F32x4Splat
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F32x4ExtractLane..F32x4Abs
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F32x4Neg..F32x4Div
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F32x4Min..F32x4Le
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F32x4Gt..F32x4Pmax
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F32x4Ceil..F16x8Splat
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F16x8ExtractLane..F16x8Sqrt
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F16x8Ceil..F16x8Sub
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F16x8Mul..F16x8Pmax
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // F16x8Eq..F16x8Ge
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010,
  0x00010, // I16x8SConvertF16x8..F16x8DemoteF64x2Zero
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010,
  0x00010, // F32x4PromoteLowF16x8..I64x2ExtractLane
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010,
  0x00010, // I64x2ReplaceLane..I64x2SConvertI32x4Low
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I64x2SConvertI32x4High..I64x2ShrS
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I64x2Add..I64x2GtS
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I64x2GeS..I64x2ExtMulHighI32x4U
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I32x4Splat..I32x4SConvertI16x8Low
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I32x4SConvertI16x8High..I32x4Sub
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I32x4Mul..I32x4LtS
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I32x4LeS..I32x4UConvertI16x8High
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I32x4ShrU..I32x4GtU
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I32x4GeU..I32x4AddPairwise
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010,
  0x00010, // I32x4ExtMulLowI16x8S..I32x4ExtAddPairwiseI16x8U
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010,
  0x00010, // I32x4TruncSatF64x2SZero..I16x8ReplaceLane
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I16x8MoveLane..I16x8ShrS
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I16x8SConvertI32x4..I16x8Mul
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I16x8MinS..I16x8LeS
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I16x8GtS..I16x8UConvertI32x4
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I16x8AddSatU..I16x8LeU
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I16x8GtU..I16x8BitMask
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010,
  0x00010, // I16x8ExtMulLowI8x16S..I16x8ExtAddPairwiseI8x16U
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I8x16Splat..I8x16SConvertI16x8
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I8x16Neg..I8x16Sub
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I8x16SubSatS..I8x16LtS
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I8x16LeS..I8x16SubSatU
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I8x16ShrU..I8x16GtU
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I8x16GeU..S128Zero
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // S128Const..S128Select
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // S128AndNot..I64x2RelaxedLaneSelect
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010,
  0x00010, // F32x4RelaxedMin..I32x4RelaxedTruncF32x4U
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010,
  0x00010, // I32x4RelaxedTruncF64x2SZero..I8x16AddReduce
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I16x8AddReduce..I8x16Shuffle
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I8x8Shuffle..I64x2AllTrue
  0x00010, 0x00010, 0x00010, 0x00010, 0x00010, 0x00010, // I32x4AllTrue..StoreLane
  0x20320, 0x00320, 0x20320, 0x20320, 0x20320, 0x20320, // JSEqual..JSGreaterThanOrEqual
  0x20220, 0x20220, 0x20220, 0x20220, 0x20220, 0x20220, // JSBitwiseOr..JSShiftRightLogical
  0x20220, 0x20220, 0x20220, 0x20220, 0x20220, 0x20220, // JSAdd..JSExponentiate
  0x20020, 0x20220, 0x20020, 0x20220, 0x20220, 0x20220, // JSHasInPrototypeChain..JSBitwiseNot
  0x20220, 0x20020, 0x20020, 0x20020, 0x20020, 0x20020, // JSNegate..JSToBigInt
  0x20020, 0x20020, 0x20020, 0x20020, 0x20020,
  0x20220, // JSToBigIntConvertNumber..JSCloneObject
  0x20020, 0x120020, 0x220020, 0x20020, 0x200020,
  0x200020, // JSCreate..JSCreateAsyncFunctionObject
  0x200020, 0x80020, 0x200020, 0x00220, 0x00020,
  0xa0020, // JSCreateBoundFunction..JSCreateGeneratorObject
  0x200020, 0x200020, 0x20220, 0x20220, 0x20220,
  0x220020, // JSCreateIterResultObject..JSCreateObject
  0x200020, 0x200020, 0x200020, 0x220020, 0x00220,
  0xa0220, // JSCreateStringWrapper..JSLoadProperty
  0xa0220, 0xa0220, 0x20220, 0xa0220, 0xa0220, 0xa0220, // JSLoadNamed..JSSetNamedProperty
  0xa0220, 0xa0220, 0xa0220, 0xa0220, 0xa0220,
  0xa0020, // JSSetPrototypeProperties..JSDeleteProperty
  0xa0220, 0x20020, 0xa0020, 0x80020, 0x80020, 0x80020, // JSHasProperty..JSLoadContext
  0x80020, 0xa0020, 0x90020, 0x90020, 0x90020,
  0x90020, // JSStoreContextNoCell..JSCreateBlockContext
  0x28220, 0x20020, 0x20220, 0x20220, 0x20020, 0x20020, // JSCall..JSConstructForwardVarargs
  0x20020, 0x28220, 0x20220, 0x20220, 0x220020,
  0x220020, // JSConstructForwardAllArgs..JSAsyncFunctionEnter
  0x220020, 0x220020, 0x100020, 0x20020, 0x20020,
  0x20220, // JSAsyncFunctionReject..JSForInNext
  0x00220, 0x20220, 0x20020, 0x00020, 0x00020, 0x80020, // JSForInPrepare..JSLoadModule
  0x80020, 0xa0020, 0x80020, 0x00020, 0x00020,
  0x00020, // JSStoreModule..JSGeneratorRestoreRegister
  0x00020, 0x200020, 0x220020, 0x220020, 0x220020,
  0x220020, // JSGeneratorRestoreInputOrDebugPos..JSResolvePromise
  0x20020, 0x220020, 0x220020, 0xa0020, // JSStackCheck..JSDebugger
];

// —— 生成不变式 (判别值; 谓词抽样对数在 tests/opcodes_unittest.rs) ——

const _: () = {
  assert!(IrOpcode::COUNT == 1036);
  assert!(IrOpcode::LAST as u16 == 1035);
  assert!(IrOpcode::LAST_U16 == 1035);
  assert!(IrOpcode::Start as u16 == 0);
  assert!(IrOpcode::End as u16 == 21);
};
