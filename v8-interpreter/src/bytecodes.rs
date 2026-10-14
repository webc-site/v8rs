// 由 tool/src/interpreter/bytecodes.h.js 从 v8/src/interpreter/bytecodes.{h,cc} 生成; 修订就地改, 复跑覆盖
// Ignition Bytecode 全表 + 操作数元表 (clang -E 对数: 全表 217 = 200+16+1)

use crate::{
  bytecode_operands::{
    ImplicitRegisterUse, OperandScale, OperandSize, OperandType, OperandTypeInfo, ReceiverMode,
  },
  bytecode_traits::OPERAND_KIND_SIZES,
};

/// 字节码总数 (原版 Bytecodes::kBytecodeCount = kLast + 1; clang -E 展开核对 217)
pub const BYTECODE_COUNT: usize = 217;
/// 单条字节码最大操作数个数 (原版 kMaxOperands)
pub const MAX_OPERANDS: usize = 5;
/// ShortStar 字节码数 (Star15..Star0)
pub const SHORT_STAR_COUNT: usize = 16;

/// 解释器字节码全表 (原版 enum class Bytecode : uint8_t; 顺序敏感 — 范围谓词依赖)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Bytecode {
  /// 原版 kWide (None)
  Wide = 0,
  /// 原版 kExtraWide (None)
  ExtraWide = 1,
  /// 原版 kDebugBreakWide (ReadWriteAccumulator)
  DebugBreakWide = 2,
  /// 原版 kDebugBreakExtraWide (ReadWriteAccumulator)
  DebugBreakExtraWide = 3,
  /// 原版 kDebugBreak0 (ReadWriteAccumulator)
  DebugBreak0 = 4,
  /// 原版 kDebugBreak1 (ReadWriteAccumulator, 操作数 Reg)
  DebugBreak1 = 5,
  /// 原版 kDebugBreak2 (ReadWriteAccumulator, 操作数 Reg+Reg)
  DebugBreak2 = 6,
  /// 原版 kDebugBreak3 (ReadWriteAccumulator, 操作数 Reg+Reg+Reg)
  DebugBreak3 = 7,
  /// 原版 kDebugBreak4 (ReadWriteAccumulator, 操作数 Reg+Reg+Reg+Reg)
  DebugBreak4 = 8,
  /// 原版 kDebugBreak5 (ReadWriteAccumulator, 操作数 RuntimeId+Reg+Reg)
  DebugBreak5 = 9,
  /// 原版 kDebugBreak6 (ReadWriteAccumulator, 操作数 RuntimeId+Reg+Reg+Reg)
  DebugBreak6 = 10,
  /// 原版 kLdar (WriteAccumulator, 操作数 Reg)
  Ldar = 11,
  /// 原版 kLdaZero (WriteAccumulator)
  LdaZero = 12,
  /// 原版 kLdaSmi (WriteAccumulator, 操作数 Imm)
  LdaSmi = 13,
  /// 原版 kLdaUndefined (WriteAccumulator)
  LdaUndefined = 14,
  /// 原版 kLdaNull (WriteAccumulator)
  LdaNull = 15,
  /// 原版 kLdaTheHole (WriteAccumulator)
  LdaTheHole = 16,
  /// 原版 kLdaTdzHole (WriteAccumulator)
  LdaTdzHole = 17,
  /// 原版 kLdaTrue (WriteAccumulator)
  LdaTrue = 18,
  /// 原版 kLdaFalse (WriteAccumulator)
  LdaFalse = 19,
  /// 原版 kLdaConstant (WriteAccumulator, 操作数 ConstantPoolIndex)
  LdaConstant = 20,
  /// 原版 kLdaContextSlotNoCell (WriteAccumulator, 操作数 Reg+ContextSlot+UImm)
  LdaContextSlotNoCell = 21,
  /// 原版 kLdaContextSlot (WriteAccumulator, 操作数 Reg+ContextSlot+UImm)
  LdaContextSlot = 22,
  /// 原版 kLdaImmutableContextSlot (WriteAccumulator, 操作数 Reg+ContextSlot+UImm)
  LdaImmutableContextSlot = 23,
  /// 原版 kLdaCurrentContextSlotNoCell (WriteAccumulator, 操作数 ContextSlot)
  LdaCurrentContextSlotNoCell = 24,
  /// 原版 kLdaCurrentContextSlot (WriteAccumulator, 操作数 ContextSlot)
  LdaCurrentContextSlot = 25,
  /// 原版 kLdaImmutableCurrentContextSlot (WriteAccumulator, 操作数 ContextSlot)
  LdaImmutableCurrentContextSlot = 26,
  /// 原版 kStar (ReadAccumulator, 操作数 RegOut)
  Star = 27,
  /// 原版 kMov (None, 操作数 Reg+RegOut)
  Mov = 28,
  /// 原版 kPushContext (ReadAccumulator, 操作数 RegOut)
  PushContext = 29,
  /// 原版 kPopContext (None, 操作数 Reg)
  PopContext = 30,
  /// 原版 kTestReferenceEqual (ReadWriteAccumulator, 操作数 Reg)
  TestReferenceEqual = 31,
  /// 原版 kTestUndetectable (ReadWriteAccumulator)
  TestUndetectable = 32,
  /// 原版 kTestNull (ReadWriteAccumulator)
  TestNull = 33,
  /// 原版 kTestUndefined (ReadWriteAccumulator)
  TestUndefined = 34,
  /// 原版 kTestTypeOf (ReadWriteAccumulator, 操作数 Flag8)
  TestTypeOf = 35,
  /// 原版 kLdaGlobal (WriteAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot)
  LdaGlobal = 36,
  /// 原版 kLdaGlobalInsideTypeof (WriteAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot)
  LdaGlobalInsideTypeof = 37,
  /// 原版 kStaGlobal (ReadAndClobberAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot)
  StaGlobal = 38,
  /// 原版 kStaContextSlotNoCell (ReadAccumulator, 操作数 Reg+ContextSlot+UImm)
  StaContextSlotNoCell = 39,
  /// 原版 kStaCurrentContextSlotNoCell (ReadAccumulator, 操作数 ContextSlot)
  StaCurrentContextSlotNoCell = 40,
  /// 原版 kStaContextSlot (ReadAccumulator, 操作数 Reg+ContextSlot+UImm)
  StaContextSlot = 41,
  /// 原版 kStaCurrentContextSlot (ReadAccumulator, 操作数 ContextSlot)
  StaCurrentContextSlot = 42,
  /// 原版 kLdaLookupSlot (WriteAccumulator, 操作数 ConstantPoolIndex)
  LdaLookupSlot = 43,
  /// 原版 kLdaLookupContextSlotNoCell (WriteAccumulator, 操作数 ConstantPoolIndex+ContextSlot+UImm)
  LdaLookupContextSlotNoCell = 44,
  /// 原版 kLdaLookupContextSlot (WriteAccumulator, 操作数 ConstantPoolIndex+ContextSlot+UImm)
  LdaLookupContextSlot = 45,
  /// 原版 kLdaLookupGlobalSlot (WriteAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot+UImm)
  LdaLookupGlobalSlot = 46,
  /// 原版 kLdaLookupSlotInsideTypeof (WriteAccumulator, 操作数 ConstantPoolIndex)
  LdaLookupSlotInsideTypeof = 47,
  /// 原版 kLdaLookupContextSlotNoCellInsideTypeof (WriteAccumulator, 操作数 ConstantPoolIndex+ContextSlot+UImm)
  LdaLookupContextSlotNoCellInsideTypeof = 48,
  /// 原版 kLdaLookupContextSlotInsideTypeof (WriteAccumulator, 操作数 ConstantPoolIndex+ContextSlot+UImm)
  LdaLookupContextSlotInsideTypeof = 49,
  /// 原版 kLdaLookupGlobalSlotInsideTypeof (WriteAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot+UImm)
  LdaLookupGlobalSlotInsideTypeof = 50,
  /// 原版 kStaLookupSlot (ReadWriteAccumulator, 操作数 ConstantPoolIndex+Flag8)
  StaLookupSlot = 51,
  /// 原版 kGetNamedProperty (WriteAccumulator, 操作数 Reg+ConstantPoolIndex+FeedbackSlot)
  GetNamedProperty = 52,
  /// 原版 kGetNamedPropertyFromSuper (ReadWriteAccumulator, 操作数 Reg+ConstantPoolIndex+FeedbackSlot)
  GetNamedPropertyFromSuper = 53,
  /// 原版 kGetKeyedProperty (ReadWriteAccumulator, 操作数 Reg+FeedbackSlot)
  GetKeyedProperty = 54,
  /// 原版 kGetEnumeratedKeyedProperty (ReadWriteAccumulator, 操作数 Reg+Reg+Reg+FeedbackSlot)
  GetEnumeratedKeyedProperty = 55,
  /// 原版 kGetPrivateField (WriteAccumulator, 操作数 Reg+ContextSlot+UImm+Reg+FeedbackSlot)
  GetPrivateField = 56,
  /// 原版 kLdaModuleVariable (WriteAccumulator, 操作数 Imm+UImm)
  LdaModuleVariable = 57,
  /// 原版 kStaModuleVariable (ReadAccumulator, 操作数 Imm+UImm)
  StaModuleVariable = 58,
  /// 原版 kSetNamedProperty (ReadAndClobberAccumulator, 操作数 Reg+ConstantPoolIndex+FeedbackSlot)
  SetNamedProperty = 59,
  /// 原版 kDefineNamedOwnProperty (ReadAndClobberAccumulator, 操作数 Reg+ConstantPoolIndex+FeedbackSlot)
  DefineNamedOwnProperty = 60,
  /// 原版 kDefineNamedOwnPropertyInLiteral (ReadAndClobberAccumulator, 操作数 Reg+ConstantPoolIndex+FeedbackSlot)
  DefineNamedOwnPropertyInLiteral = 61,
  /// 原版 kSetKeyedProperty (ReadAndClobberAccumulator, 操作数 Reg+Reg+FeedbackSlot)
  SetKeyedProperty = 62,
  /// 原版 kDefineKeyedOwnProperty (ReadAndClobberAccumulator, 操作数 Reg+Reg+Flag8+FeedbackSlot)
  DefineKeyedOwnProperty = 63,
  /// 原版 kStaInArrayLiteral (ReadAndClobberAccumulator, 操作数 Reg+Reg+FeedbackSlot)
  StaInArrayLiteral = 64,
  /// 原版 kDefineKeyedOwnPropertyInLiteral (ReadAccumulator, 操作数 Reg+Reg+Flag8+FeedbackSlot)
  DefineKeyedOwnPropertyInLiteral = 65,
  /// 原版 kSetPrototypeProperties (ReadAndClobberAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot)
  SetPrototypeProperties = 66,
  /// 原版 kSetPrivateField (ReadAndClobberAccumulator, 操作数 Reg+ContextSlot+UImm+Reg+FeedbackSlot)
  SetPrivateField = 67,
  /// 原版 kAdd (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  Add = 68,
  /// 原版 kSub (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  Sub = 69,
  /// 原版 kMul (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  Mul = 70,
  /// 原版 kDiv (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  Div = 71,
  /// 原版 kMod (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  Mod = 72,
  /// 原版 kExp (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  Exp = 73,
  /// 原版 kBitwiseOr (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  BitwiseOr = 74,
  /// 原版 kBitwiseXor (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  BitwiseXor = 75,
  /// 原版 kBitwiseAnd (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  BitwiseAnd = 76,
  /// 原版 kShiftLeft (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  ShiftLeft = 77,
  /// 原版 kShiftRight (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  ShiftRight = 78,
  /// 原版 kShiftRightLogical (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  ShiftRightLogical = 79,
  /// 原版 kAdd_StringConstant_Internalize (ReadWriteAccumulator, 操作数 Reg+FeedbackSlot+Flag8)
  AddStringConstantInternalize = 80,
  /// 原版 kAddSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  AddSmi = 81,
  /// 原版 kSubSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  SubSmi = 82,
  /// 原版 kMulSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  MulSmi = 83,
  /// 原版 kDivSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  DivSmi = 84,
  /// 原版 kModSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  ModSmi = 85,
  /// 原版 kExpSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  ExpSmi = 86,
  /// 原版 kBitwiseOrSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  BitwiseOrSmi = 87,
  /// 原版 kBitwiseXorSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  BitwiseXorSmi = 88,
  /// 原版 kBitwiseAndSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  BitwiseAndSmi = 89,
  /// 原版 kShiftLeftSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  ShiftLeftSmi = 90,
  /// 原版 kShiftRightSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  ShiftRightSmi = 91,
  /// 原版 kShiftRightLogicalSmi (ReadWriteAccumulator, 操作数 Imm+EmbeddedFeedback)
  ShiftRightLogicalSmi = 92,
  /// 原版 kInc (ReadWriteAccumulator, 操作数 EmbeddedFeedback)
  Inc = 93,
  /// 原版 kDec (ReadWriteAccumulator, 操作数 EmbeddedFeedback)
  Dec = 94,
  /// 原版 kNegate (ReadWriteAccumulator, 操作数 EmbeddedFeedback)
  Negate = 95,
  /// 原版 kBitwiseNot (ReadWriteAccumulator, 操作数 EmbeddedFeedback)
  BitwiseNot = 96,
  /// 原版 kToBooleanLogicalNot (ReadWriteAccumulator)
  ToBooleanLogicalNot = 97,
  /// 原版 kLogicalNot (ReadWriteAccumulator)
  LogicalNot = 98,
  /// 原版 kTypeOf (ReadWriteAccumulator, 操作数 FeedbackSlot)
  TypeOf = 99,
  /// 原版 kDeletePropertyStrict (ReadWriteAccumulator, 操作数 Reg)
  DeletePropertyStrict = 100,
  /// 原版 kDeletePropertySloppy (ReadWriteAccumulator, 操作数 Reg)
  DeletePropertySloppy = 101,
  /// 原版 kGetSuperConstructor (ReadAccumulator, 操作数 RegOut)
  GetSuperConstructor = 102,
  /// 原版 kFindNonDefaultConstructorOrConstruct (None, 操作数 Reg+Reg+RegOutPair)
  FindNonDefaultConstructorOrConstruct = 103,
  /// 原版 kCallAnyReceiver (WriteAccumulator, 操作数 Reg+RegList+RegCount+FeedbackSlot)
  CallAnyReceiver = 104,
  /// 原版 kCallProperty (WriteAccumulator, 操作数 Reg+RegList+RegCount+FeedbackSlot)
  CallProperty = 105,
  /// 原版 kCallProperty0 (WriteAccumulator, 操作数 Reg+Reg+FeedbackSlot)
  CallProperty0 = 106,
  /// 原版 kCallProperty1 (WriteAccumulator, 操作数 Reg+Reg+Reg+FeedbackSlot)
  CallProperty1 = 107,
  /// 原版 kCallProperty2 (WriteAccumulator, 操作数 Reg+Reg+Reg+Reg+FeedbackSlot)
  CallProperty2 = 108,
  /// 原版 kCallUndefinedReceiver (WriteAccumulator, 操作数 Reg+RegList+RegCount+FeedbackSlot)
  CallUndefinedReceiver = 109,
  /// 原版 kCallUndefinedReceiver0 (WriteAccumulator, 操作数 Reg+FeedbackSlot)
  CallUndefinedReceiver0 = 110,
  /// 原版 kCallUndefinedReceiver1 (WriteAccumulator, 操作数 Reg+Reg+FeedbackSlot)
  CallUndefinedReceiver1 = 111,
  /// 原版 kCallUndefinedReceiver2 (WriteAccumulator, 操作数 Reg+Reg+Reg+FeedbackSlot)
  CallUndefinedReceiver2 = 112,
  /// 原版 kCallWithSpread (WriteAccumulator, 操作数 Reg+RegList+RegCount+FeedbackSlot)
  CallWithSpread = 113,
  /// 原版 kCallRuntime (WriteAccumulator, 操作数 RuntimeId+RegList+RegCount)
  CallRuntime = 114,
  /// 原版 kCallRuntimeForPair (ClobberAccumulator, 操作数 RuntimeId+RegList+RegCount+RegOutPair)
  CallRuntimeForPair = 115,
  /// 原版 kCallJSRuntime (WriteAccumulator, 操作数 NativeContextIndex+RegList+RegCount)
  CallJSRuntime = 116,
  /// 原版 kInvokeIntrinsic (WriteAccumulator, 操作数 IntrinsicId+RegList+RegCount)
  InvokeIntrinsic = 117,
  /// 原版 kConstruct (ReadWriteAccumulator, 操作数 Reg+RegList+RegCount+FeedbackSlot)
  Construct = 118,
  /// 原版 kConstructWithSpread (ReadWriteAccumulator, 操作数 Reg+RegList+RegCount+FeedbackSlot)
  ConstructWithSpread = 119,
  /// 原版 kConstructForwardAllArgs (ReadWriteAccumulator, 操作数 Reg+FeedbackSlot)
  ConstructForwardAllArgs = 120,
  /// 原版 kTestEqual (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  TestEqual = 121,
  /// 原版 kTestEqualStrict (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  TestEqualStrict = 122,
  /// 原版 kTestLessThan (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  TestLessThan = 123,
  /// 原版 kTestGreaterThan (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  TestGreaterThan = 124,
  /// 原版 kTestLessThanOrEqual (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  TestLessThanOrEqual = 125,
  /// 原版 kTestGreaterThanOrEqual (ReadWriteAccumulator, 操作数 Reg+EmbeddedFeedback)
  TestGreaterThanOrEqual = 126,
  /// 原版 kTestInstanceOf (ReadWriteAccumulator, 操作数 Reg+FeedbackSlot)
  TestInstanceOf = 127,
  /// 原版 kTestIn (ReadWriteAccumulator, 操作数 Reg+FeedbackSlot)
  TestIn = 128,
  /// 原版 kToName (ReadWriteAccumulator)
  ToName = 129,
  /// 原版 kToNumber (ReadWriteAccumulator, 操作数 FeedbackSlot)
  ToNumber = 130,
  /// 原版 kToNumeric (ReadWriteAccumulator, 操作数 FeedbackSlot)
  ToNumeric = 131,
  /// 原版 kToObject (ReadAccumulator, 操作数 RegOut)
  ToObject = 132,
  /// 原版 kToString (ReadWriteAccumulator)
  ToString = 133,
  /// 原版 kToBoolean (ReadWriteAccumulator)
  ToBoolean = 134,
  /// 原版 kCreateRegExpLiteral (WriteAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot+Flag16)
  CreateRegExpLiteral = 135,
  /// 原版 kCreateArrayLiteral (WriteAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot+Flag8)
  CreateArrayLiteral = 136,
  /// 原版 kCreateArrayFromIterable (ReadWriteAccumulator)
  CreateArrayFromIterable = 137,
  /// 原版 kCreateEmptyArrayLiteral (WriteAccumulator, 操作数 FeedbackSlot)
  CreateEmptyArrayLiteral = 138,
  /// 原版 kCreateObjectLiteral (WriteAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot+Flag8)
  CreateObjectLiteral = 139,
  /// 原版 kCreateEmptyObjectLiteral (WriteAccumulator)
  CreateEmptyObjectLiteral = 140,
  /// 原版 kCloneObject (WriteAccumulator, 操作数 Reg+Flag8+FeedbackSlot)
  CloneObject = 141,
  /// 原版 kGetTemplateObject (WriteAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot)
  GetTemplateObject = 142,
  /// 原版 kCreateClosure (WriteAccumulator, 操作数 ConstantPoolIndex+FeedbackSlot+Flag8)
  CreateClosure = 143,
  /// 原版 kCreateBlockContext (WriteAccumulator, 操作数 ConstantPoolIndex)
  CreateBlockContext = 144,
  /// 原版 kCreateCatchContext (WriteAccumulator, 操作数 Reg+ConstantPoolIndex)
  CreateCatchContext = 145,
  /// 原版 kCreateFunctionContext (WriteAccumulator, 操作数 ConstantPoolIndex+UImm)
  CreateFunctionContext = 146,
  /// 原版 kCreateFunctionContextWithCells (WriteAccumulator, 操作数 ConstantPoolIndex+UImm)
  CreateFunctionContextWithCells = 147,
  /// 原版 kCreateEvalContext (WriteAccumulator, 操作数 ConstantPoolIndex+UImm)
  CreateEvalContext = 148,
  /// 原版 kCreateWithContext (WriteAccumulator, 操作数 Reg+ConstantPoolIndex)
  CreateWithContext = 149,
  /// 原版 kCreateMappedArguments (WriteAccumulator)
  CreateMappedArguments = 150,
  /// 原版 kCreateUnmappedArguments (WriteAccumulator)
  CreateUnmappedArguments = 151,
  /// 原版 kCreateRestParameter (WriteAccumulator)
  CreateRestParameter = 152,
  /// 原版 kJumpLoop (ClobberAccumulator, 操作数 UImm+Imm+FeedbackSlot)
  JumpLoop = 153,
  /// 原版 kJump (None, 操作数 UImm)
  Jump = 154,
  /// 原版 kJumpConstant (None, 操作数 ConstantPoolIndex)
  JumpConstant = 155,
  /// 原版 kJumpIfNullConstant (ReadAccumulator, 操作数 ConstantPoolIndex)
  JumpIfNullConstant = 156,
  /// 原版 kJumpIfNotNullConstant (ReadAccumulator, 操作数 ConstantPoolIndex)
  JumpIfNotNullConstant = 157,
  /// 原版 kJumpIfUndefinedConstant (ReadAccumulator, 操作数 ConstantPoolIndex)
  JumpIfUndefinedConstant = 158,
  /// 原版 kJumpIfNotUndefinedConstant (ReadAccumulator, 操作数 ConstantPoolIndex)
  JumpIfNotUndefinedConstant = 159,
  /// 原版 kJumpIfUndefinedOrNullConstant (ReadAccumulator, 操作数 ConstantPoolIndex)
  JumpIfUndefinedOrNullConstant = 160,
  /// 原版 kJumpIfTrueConstant (ReadAccumulator, 操作数 ConstantPoolIndex)
  JumpIfTrueConstant = 161,
  /// 原版 kJumpIfFalseConstant (ReadAccumulator, 操作数 ConstantPoolIndex)
  JumpIfFalseConstant = 162,
  /// 原版 kJumpIfJSReceiverConstant (ReadAccumulator, 操作数 ConstantPoolIndex)
  JumpIfJSReceiverConstant = 163,
  /// 原版 kJumpIfForInDoneConstant (None, 操作数 ConstantPoolIndex+Reg+Reg)
  JumpIfForInDoneConstant = 164,
  /// 原版 kJumpIfToBooleanTrueConstant (ReadAccumulator, 操作数 ConstantPoolIndex)
  JumpIfToBooleanTrueConstant = 165,
  /// 原版 kJumpIfToBooleanFalseConstant (ReadAccumulator, 操作数 ConstantPoolIndex)
  JumpIfToBooleanFalseConstant = 166,
  /// 原版 kJumpIfToBooleanTrue (ReadAccumulator, 操作数 UImm)
  JumpIfToBooleanTrue = 167,
  /// 原版 kJumpIfToBooleanFalse (ReadAccumulator, 操作数 UImm)
  JumpIfToBooleanFalse = 168,
  /// 原版 kJumpIfTrue (ReadAccumulator, 操作数 UImm)
  JumpIfTrue = 169,
  /// 原版 kJumpIfFalse (ReadAccumulator, 操作数 UImm)
  JumpIfFalse = 170,
  /// 原版 kJumpIfNull (ReadAccumulator, 操作数 UImm)
  JumpIfNull = 171,
  /// 原版 kJumpIfNotNull (ReadAccumulator, 操作数 UImm)
  JumpIfNotNull = 172,
  /// 原版 kJumpIfUndefined (ReadAccumulator, 操作数 UImm)
  JumpIfUndefined = 173,
  /// 原版 kJumpIfNotUndefined (ReadAccumulator, 操作数 UImm)
  JumpIfNotUndefined = 174,
  /// 原版 kJumpIfUndefinedOrNull (ReadAccumulator, 操作数 UImm)
  JumpIfUndefinedOrNull = 175,
  /// 原版 kJumpIfJSReceiver (ReadAccumulator, 操作数 UImm)
  JumpIfJSReceiver = 176,
  /// 原版 kJumpIfForInDone (None, 操作数 UImm+Reg+Reg)
  JumpIfForInDone = 177,
  /// 原版 kSwitchOnSmiNoFeedback (ReadAccumulator, 操作数 ConstantPoolIndex+UImm+Imm)
  SwitchOnSmiNoFeedback = 178,
  /// 原版 kForInEnumerate (WriteAccumulator, 操作数 Reg)
  ForInEnumerate = 179,
  /// 原版 kForInPrepare (ReadAndClobberAccumulator, 操作数 RegOutTriple+FeedbackSlot)
  ForInPrepare = 180,
  /// 原版 kForInNext (WriteAccumulator, 操作数 Reg+Reg+RegPair+FeedbackSlot)
  ForInNext = 181,
  /// 原版 kForInStep (None, 操作数 RegInOut)
  ForInStep = 182,
  /// 原版 kForOfNext (WriteAccumulator, 操作数 Reg+Reg+FeedbackSlot)
  ForOfNext = 183,
  /// 原版 kSetPendingMessage (ReadWriteAccumulator)
  SetPendingMessage = 184,
  /// 原版 kThrow (ReadAccumulator)
  Throw = 185,
  /// 原版 kReThrow (ReadAccumulator)
  ReThrow = 186,
  /// 原版 kReturn (ReadAccumulator)
  Return = 187,
  /// 原版 kThrowReferenceErrorIfTdzHole (ReadAccumulator, 操作数 ConstantPoolIndex)
  ThrowReferenceErrorIfTdzHole = 188,
  /// 原版 kThrowSuperNotCalledIfTdzHole (ReadAccumulator)
  ThrowSuperNotCalledIfTdzHole = 189,
  /// 原版 kThrowSuperAlreadyCalledIfNotTdzHole (ReadAccumulator)
  ThrowSuperAlreadyCalledIfNotTdzHole = 190,
  /// 原版 kThrowIfNotSuperConstructor (None, 操作数 Reg)
  ThrowIfNotSuperConstructor = 191,
  /// 原版 kSwitchOnGeneratorState (None, 操作数 Reg+ConstantPoolIndex+UImm)
  SwitchOnGeneratorState = 192,
  /// 原版 kSuspendGenerator (ReadAccumulator, 操作数 Reg+RegList+RegCount+UImm)
  SuspendGenerator = 193,
  /// 原版 kResumeGenerator (WriteAccumulator, 操作数 Reg+RegOutList+RegCount)
  ResumeGenerator = 194,
  /// 原版 kGetIterator (WriteAccumulator, 操作数 Reg+FeedbackSlot+FeedbackSlot)
  GetIterator = 195,
  /// 原版 kArrayDestructure (ReadAndClobberAccumulator, 操作数 RegOutList+RegCount)
  ArrayDestructure = 196,
  /// 原版 kDebugger (ClobberAccumulator)
  Debugger = 197,
  /// 原版 kIncBlockCounter (None, 操作数 CoverageSlot)
  IncBlockCounter = 198,
  /// 原版 kAbort (None, 操作数 AbortReason)
  Abort = 199,
  /// 原版 kStar15 (ReadAccumulatorWriteShortStar)
  Star15 = 200,
  /// 原版 kStar14 (ReadAccumulatorWriteShortStar)
  Star14 = 201,
  /// 原版 kStar13 (ReadAccumulatorWriteShortStar)
  Star13 = 202,
  /// 原版 kStar12 (ReadAccumulatorWriteShortStar)
  Star12 = 203,
  /// 原版 kStar11 (ReadAccumulatorWriteShortStar)
  Star11 = 204,
  /// 原版 kStar10 (ReadAccumulatorWriteShortStar)
  Star10 = 205,
  /// 原版 kStar9 (ReadAccumulatorWriteShortStar)
  Star9 = 206,
  /// 原版 kStar8 (ReadAccumulatorWriteShortStar)
  Star8 = 207,
  /// 原版 kStar7 (ReadAccumulatorWriteShortStar)
  Star7 = 208,
  /// 原版 kStar6 (ReadAccumulatorWriteShortStar)
  Star6 = 209,
  /// 原版 kStar5 (ReadAccumulatorWriteShortStar)
  Star5 = 210,
  /// 原版 kStar4 (ReadAccumulatorWriteShortStar)
  Star4 = 211,
  /// 原版 kStar3 (ReadAccumulatorWriteShortStar)
  Star3 = 212,
  /// 原版 kStar2 (ReadAccumulatorWriteShortStar)
  Star2 = 213,
  /// 原版 kStar1 (ReadAccumulatorWriteShortStar)
  Star1 = 214,
  /// 原版 kStar0 (ReadAccumulatorWriteShortStar)
  Star0 = 215,
  /// 原版 kIllegal (None)
  Illegal = 216,
}

impl Bytecode {
  /// 字节码总数 (原版 Bytecodes::kBytecodeCount)
  pub const BYTECODE_COUNT: usize = BYTECODE_COUNT;
  /// 单条字节码最大操作数个数 (原版 kMaxOperands)
  pub const MAX_OPERANDS: usize = MAX_OPERANDS;
  /// ShortStar 字节码数
  pub const SHORT_STAR_COUNT: usize = SHORT_STAR_COUNT;

  /// 原版 kLast (最后一个变体)
  pub const LAST: Bytecode = Self::Illegal;
  /// ShortStar 区首 (原版 kFirstShortStar = kStar15)
  pub const FIRST_SHORT_STAR: Bytecode = Self::Star15;
  /// ShortStar 区尾 (原版 kLastShortStar = kStar0)
  pub const LAST_SHORT_STAR: Bytecode = Self::Star0;

  /// 原版名 (bytecodes.cc Bytecodes::ToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    NAMES[self as usize]
  }

  /// 数值反查 (原版 Bytecodes::FromByte; 全表 match 展开)
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::Wide),
      1 => Some(Self::ExtraWide),
      2 => Some(Self::DebugBreakWide),
      3 => Some(Self::DebugBreakExtraWide),
      4 => Some(Self::DebugBreak0),
      5 => Some(Self::DebugBreak1),
      6 => Some(Self::DebugBreak2),
      7 => Some(Self::DebugBreak3),
      8 => Some(Self::DebugBreak4),
      9 => Some(Self::DebugBreak5),
      10 => Some(Self::DebugBreak6),
      11 => Some(Self::Ldar),
      12 => Some(Self::LdaZero),
      13 => Some(Self::LdaSmi),
      14 => Some(Self::LdaUndefined),
      15 => Some(Self::LdaNull),
      16 => Some(Self::LdaTheHole),
      17 => Some(Self::LdaTdzHole),
      18 => Some(Self::LdaTrue),
      19 => Some(Self::LdaFalse),
      20 => Some(Self::LdaConstant),
      21 => Some(Self::LdaContextSlotNoCell),
      22 => Some(Self::LdaContextSlot),
      23 => Some(Self::LdaImmutableContextSlot),
      24 => Some(Self::LdaCurrentContextSlotNoCell),
      25 => Some(Self::LdaCurrentContextSlot),
      26 => Some(Self::LdaImmutableCurrentContextSlot),
      27 => Some(Self::Star),
      28 => Some(Self::Mov),
      29 => Some(Self::PushContext),
      30 => Some(Self::PopContext),
      31 => Some(Self::TestReferenceEqual),
      32 => Some(Self::TestUndetectable),
      33 => Some(Self::TestNull),
      34 => Some(Self::TestUndefined),
      35 => Some(Self::TestTypeOf),
      36 => Some(Self::LdaGlobal),
      37 => Some(Self::LdaGlobalInsideTypeof),
      38 => Some(Self::StaGlobal),
      39 => Some(Self::StaContextSlotNoCell),
      40 => Some(Self::StaCurrentContextSlotNoCell),
      41 => Some(Self::StaContextSlot),
      42 => Some(Self::StaCurrentContextSlot),
      43 => Some(Self::LdaLookupSlot),
      44 => Some(Self::LdaLookupContextSlotNoCell),
      45 => Some(Self::LdaLookupContextSlot),
      46 => Some(Self::LdaLookupGlobalSlot),
      47 => Some(Self::LdaLookupSlotInsideTypeof),
      48 => Some(Self::LdaLookupContextSlotNoCellInsideTypeof),
      49 => Some(Self::LdaLookupContextSlotInsideTypeof),
      50 => Some(Self::LdaLookupGlobalSlotInsideTypeof),
      51 => Some(Self::StaLookupSlot),
      52 => Some(Self::GetNamedProperty),
      53 => Some(Self::GetNamedPropertyFromSuper),
      54 => Some(Self::GetKeyedProperty),
      55 => Some(Self::GetEnumeratedKeyedProperty),
      56 => Some(Self::GetPrivateField),
      57 => Some(Self::LdaModuleVariable),
      58 => Some(Self::StaModuleVariable),
      59 => Some(Self::SetNamedProperty),
      60 => Some(Self::DefineNamedOwnProperty),
      61 => Some(Self::DefineNamedOwnPropertyInLiteral),
      62 => Some(Self::SetKeyedProperty),
      63 => Some(Self::DefineKeyedOwnProperty),
      64 => Some(Self::StaInArrayLiteral),
      65 => Some(Self::DefineKeyedOwnPropertyInLiteral),
      66 => Some(Self::SetPrototypeProperties),
      67 => Some(Self::SetPrivateField),
      68 => Some(Self::Add),
      69 => Some(Self::Sub),
      70 => Some(Self::Mul),
      71 => Some(Self::Div),
      72 => Some(Self::Mod),
      73 => Some(Self::Exp),
      74 => Some(Self::BitwiseOr),
      75 => Some(Self::BitwiseXor),
      76 => Some(Self::BitwiseAnd),
      77 => Some(Self::ShiftLeft),
      78 => Some(Self::ShiftRight),
      79 => Some(Self::ShiftRightLogical),
      80 => Some(Self::AddStringConstantInternalize),
      81 => Some(Self::AddSmi),
      82 => Some(Self::SubSmi),
      83 => Some(Self::MulSmi),
      84 => Some(Self::DivSmi),
      85 => Some(Self::ModSmi),
      86 => Some(Self::ExpSmi),
      87 => Some(Self::BitwiseOrSmi),
      88 => Some(Self::BitwiseXorSmi),
      89 => Some(Self::BitwiseAndSmi),
      90 => Some(Self::ShiftLeftSmi),
      91 => Some(Self::ShiftRightSmi),
      92 => Some(Self::ShiftRightLogicalSmi),
      93 => Some(Self::Inc),
      94 => Some(Self::Dec),
      95 => Some(Self::Negate),
      96 => Some(Self::BitwiseNot),
      97 => Some(Self::ToBooleanLogicalNot),
      98 => Some(Self::LogicalNot),
      99 => Some(Self::TypeOf),
      100 => Some(Self::DeletePropertyStrict),
      101 => Some(Self::DeletePropertySloppy),
      102 => Some(Self::GetSuperConstructor),
      103 => Some(Self::FindNonDefaultConstructorOrConstruct),
      104 => Some(Self::CallAnyReceiver),
      105 => Some(Self::CallProperty),
      106 => Some(Self::CallProperty0),
      107 => Some(Self::CallProperty1),
      108 => Some(Self::CallProperty2),
      109 => Some(Self::CallUndefinedReceiver),
      110 => Some(Self::CallUndefinedReceiver0),
      111 => Some(Self::CallUndefinedReceiver1),
      112 => Some(Self::CallUndefinedReceiver2),
      113 => Some(Self::CallWithSpread),
      114 => Some(Self::CallRuntime),
      115 => Some(Self::CallRuntimeForPair),
      116 => Some(Self::CallJSRuntime),
      117 => Some(Self::InvokeIntrinsic),
      118 => Some(Self::Construct),
      119 => Some(Self::ConstructWithSpread),
      120 => Some(Self::ConstructForwardAllArgs),
      121 => Some(Self::TestEqual),
      122 => Some(Self::TestEqualStrict),
      123 => Some(Self::TestLessThan),
      124 => Some(Self::TestGreaterThan),
      125 => Some(Self::TestLessThanOrEqual),
      126 => Some(Self::TestGreaterThanOrEqual),
      127 => Some(Self::TestInstanceOf),
      128 => Some(Self::TestIn),
      129 => Some(Self::ToName),
      130 => Some(Self::ToNumber),
      131 => Some(Self::ToNumeric),
      132 => Some(Self::ToObject),
      133 => Some(Self::ToString),
      134 => Some(Self::ToBoolean),
      135 => Some(Self::CreateRegExpLiteral),
      136 => Some(Self::CreateArrayLiteral),
      137 => Some(Self::CreateArrayFromIterable),
      138 => Some(Self::CreateEmptyArrayLiteral),
      139 => Some(Self::CreateObjectLiteral),
      140 => Some(Self::CreateEmptyObjectLiteral),
      141 => Some(Self::CloneObject),
      142 => Some(Self::GetTemplateObject),
      143 => Some(Self::CreateClosure),
      144 => Some(Self::CreateBlockContext),
      145 => Some(Self::CreateCatchContext),
      146 => Some(Self::CreateFunctionContext),
      147 => Some(Self::CreateFunctionContextWithCells),
      148 => Some(Self::CreateEvalContext),
      149 => Some(Self::CreateWithContext),
      150 => Some(Self::CreateMappedArguments),
      151 => Some(Self::CreateUnmappedArguments),
      152 => Some(Self::CreateRestParameter),
      153 => Some(Self::JumpLoop),
      154 => Some(Self::Jump),
      155 => Some(Self::JumpConstant),
      156 => Some(Self::JumpIfNullConstant),
      157 => Some(Self::JumpIfNotNullConstant),
      158 => Some(Self::JumpIfUndefinedConstant),
      159 => Some(Self::JumpIfNotUndefinedConstant),
      160 => Some(Self::JumpIfUndefinedOrNullConstant),
      161 => Some(Self::JumpIfTrueConstant),
      162 => Some(Self::JumpIfFalseConstant),
      163 => Some(Self::JumpIfJSReceiverConstant),
      164 => Some(Self::JumpIfForInDoneConstant),
      165 => Some(Self::JumpIfToBooleanTrueConstant),
      166 => Some(Self::JumpIfToBooleanFalseConstant),
      167 => Some(Self::JumpIfToBooleanTrue),
      168 => Some(Self::JumpIfToBooleanFalse),
      169 => Some(Self::JumpIfTrue),
      170 => Some(Self::JumpIfFalse),
      171 => Some(Self::JumpIfNull),
      172 => Some(Self::JumpIfNotNull),
      173 => Some(Self::JumpIfUndefined),
      174 => Some(Self::JumpIfNotUndefined),
      175 => Some(Self::JumpIfUndefinedOrNull),
      176 => Some(Self::JumpIfJSReceiver),
      177 => Some(Self::JumpIfForInDone),
      178 => Some(Self::SwitchOnSmiNoFeedback),
      179 => Some(Self::ForInEnumerate),
      180 => Some(Self::ForInPrepare),
      181 => Some(Self::ForInNext),
      182 => Some(Self::ForInStep),
      183 => Some(Self::ForOfNext),
      184 => Some(Self::SetPendingMessage),
      185 => Some(Self::Throw),
      186 => Some(Self::ReThrow),
      187 => Some(Self::Return),
      188 => Some(Self::ThrowReferenceErrorIfTdzHole),
      189 => Some(Self::ThrowSuperNotCalledIfTdzHole),
      190 => Some(Self::ThrowSuperAlreadyCalledIfNotTdzHole),
      191 => Some(Self::ThrowIfNotSuperConstructor),
      192 => Some(Self::SwitchOnGeneratorState),
      193 => Some(Self::SuspendGenerator),
      194 => Some(Self::ResumeGenerator),
      195 => Some(Self::GetIterator),
      196 => Some(Self::ArrayDestructure),
      197 => Some(Self::Debugger),
      198 => Some(Self::IncBlockCounter),
      199 => Some(Self::Abort),
      200 => Some(Self::Star15),
      201 => Some(Self::Star14),
      202 => Some(Self::Star13),
      203 => Some(Self::Star12),
      204 => Some(Self::Star11),
      205 => Some(Self::Star10),
      206 => Some(Self::Star9),
      207 => Some(Self::Star8),
      208 => Some(Self::Star7),
      209 => Some(Self::Star6),
      210 => Some(Self::Star5),
      211 => Some(Self::Star4),
      212 => Some(Self::Star3),
      213 => Some(Self::Star2),
      214 => Some(Self::Star1),
      215 => Some(Self::Star0),
      216 => Some(Self::Illegal),
      _ => None,
    }
  }

  /// 字节值 (原版 Bytecodes::ToByte)
  #[inline]
  #[must_use]
  pub const fn to_byte(self) -> u8 {
    self as u8
  }

  /// 隐式累加器用法表查询 (原版 GetImplicitRegisterUse)
  #[inline]
  #[must_use]
  pub const fn implicit_register_use(self) -> ImplicitRegisterUse {
    IMPLICIT_REGISTER_USE[self as usize]
  }

  /// 操作数个数 (原版 NumberOfOperands)
  #[inline]
  #[must_use]
  pub const fn number_of_operands(self) -> usize {
    OPERAND_COUNT[self as usize] as usize
  }

  /// 操作数类型列表 (原版 GetOperandTypes)
  #[inline]
  #[must_use]
  pub const fn operand_types(self) -> &'static [OperandType] {
    OPERAND_TYPES[self as usize]
  }

  /// 指定刻度的操作数尺寸列表 (原版 GetOperandSizes; scale.as_index() = scale >> 1)
  #[inline]
  #[must_use]
  pub const fn operand_sizes(self, scale: OperandScale) -> &'static [OperandSize] {
    OPERAND_SIZES[scale.as_index()][self as usize]
  }

  /// 指定刻度的操作数偏移列表, 相对字节码起始, 首操作数恒为 1 (原版 GetOperandOffset)
  #[inline]
  #[must_use]
  pub const fn operand_offsets(self, scale: OperandScale) -> &'static [u8] {
    OPERAND_OFFSETS[scale.as_index()][self as usize]
  }

  /// 含操作数的字节码总尺寸 (原版 Bytecodes::Size)
  #[inline]
  #[must_use]
  pub const fn size(self, scale: OperandScale) -> usize {
    BYTECODE_SIZES[scale.as_index()][self as usize] as usize
  }

  #[inline]
  pub fn from_byte(v: u8) -> Option<Self> {
    Self::from_repr(v)
  }

  #[inline]
  pub fn get_operand_type(self, index: usize) -> OperandType {
    self.operand_types()[index]
  }

  #[inline]
  pub fn get_operand_offset(self, index: usize, scale: OperandScale) -> usize {
    self.operand_offsets(scale)[index] as usize
  }
}

/// 字节码原版名表 (ToString X-macro 展开, 顺序同枚举)
pub static NAMES: [&str; BYTECODE_COUNT] = [
  "Wide",
  "ExtraWide",
  "DebugBreakWide",
  "DebugBreakExtraWide",
  "DebugBreak0",
  "DebugBreak1",
  "DebugBreak2",
  "DebugBreak3",
  "DebugBreak4",
  "DebugBreak5",
  "DebugBreak6",
  "Ldar",
  "LdaZero",
  "LdaSmi",
  "LdaUndefined",
  "LdaNull",
  "LdaTheHole",
  "LdaTdzHole",
  "LdaTrue",
  "LdaFalse",
  "LdaConstant",
  "LdaContextSlotNoCell",
  "LdaContextSlot",
  "LdaImmutableContextSlot",
  "LdaCurrentContextSlotNoCell",
  "LdaCurrentContextSlot",
  "LdaImmutableCurrentContextSlot",
  "Star",
  "Mov",
  "PushContext",
  "PopContext",
  "TestReferenceEqual",
  "TestUndetectable",
  "TestNull",
  "TestUndefined",
  "TestTypeOf",
  "LdaGlobal",
  "LdaGlobalInsideTypeof",
  "StaGlobal",
  "StaContextSlotNoCell",
  "StaCurrentContextSlotNoCell",
  "StaContextSlot",
  "StaCurrentContextSlot",
  "LdaLookupSlot",
  "LdaLookupContextSlotNoCell",
  "LdaLookupContextSlot",
  "LdaLookupGlobalSlot",
  "LdaLookupSlotInsideTypeof",
  "LdaLookupContextSlotNoCellInsideTypeof",
  "LdaLookupContextSlotInsideTypeof",
  "LdaLookupGlobalSlotInsideTypeof",
  "StaLookupSlot",
  "GetNamedProperty",
  "GetNamedPropertyFromSuper",
  "GetKeyedProperty",
  "GetEnumeratedKeyedProperty",
  "GetPrivateField",
  "LdaModuleVariable",
  "StaModuleVariable",
  "SetNamedProperty",
  "DefineNamedOwnProperty",
  "DefineNamedOwnPropertyInLiteral",
  "SetKeyedProperty",
  "DefineKeyedOwnProperty",
  "StaInArrayLiteral",
  "DefineKeyedOwnPropertyInLiteral",
  "SetPrototypeProperties",
  "SetPrivateField",
  "Add",
  "Sub",
  "Mul",
  "Div",
  "Mod",
  "Exp",
  "BitwiseOr",
  "BitwiseXor",
  "BitwiseAnd",
  "ShiftLeft",
  "ShiftRight",
  "ShiftRightLogical",
  "Add_StringConstant_Internalize",
  "AddSmi",
  "SubSmi",
  "MulSmi",
  "DivSmi",
  "ModSmi",
  "ExpSmi",
  "BitwiseOrSmi",
  "BitwiseXorSmi",
  "BitwiseAndSmi",
  "ShiftLeftSmi",
  "ShiftRightSmi",
  "ShiftRightLogicalSmi",
  "Inc",
  "Dec",
  "Negate",
  "BitwiseNot",
  "ToBooleanLogicalNot",
  "LogicalNot",
  "TypeOf",
  "DeletePropertyStrict",
  "DeletePropertySloppy",
  "GetSuperConstructor",
  "FindNonDefaultConstructorOrConstruct",
  "CallAnyReceiver",
  "CallProperty",
  "CallProperty0",
  "CallProperty1",
  "CallProperty2",
  "CallUndefinedReceiver",
  "CallUndefinedReceiver0",
  "CallUndefinedReceiver1",
  "CallUndefinedReceiver2",
  "CallWithSpread",
  "CallRuntime",
  "CallRuntimeForPair",
  "CallJSRuntime",
  "InvokeIntrinsic",
  "Construct",
  "ConstructWithSpread",
  "ConstructForwardAllArgs",
  "TestEqual",
  "TestEqualStrict",
  "TestLessThan",
  "TestGreaterThan",
  "TestLessThanOrEqual",
  "TestGreaterThanOrEqual",
  "TestInstanceOf",
  "TestIn",
  "ToName",
  "ToNumber",
  "ToNumeric",
  "ToObject",
  "ToString",
  "ToBoolean",
  "CreateRegExpLiteral",
  "CreateArrayLiteral",
  "CreateArrayFromIterable",
  "CreateEmptyArrayLiteral",
  "CreateObjectLiteral",
  "CreateEmptyObjectLiteral",
  "CloneObject",
  "GetTemplateObject",
  "CreateClosure",
  "CreateBlockContext",
  "CreateCatchContext",
  "CreateFunctionContext",
  "CreateFunctionContextWithCells",
  "CreateEvalContext",
  "CreateWithContext",
  "CreateMappedArguments",
  "CreateUnmappedArguments",
  "CreateRestParameter",
  "JumpLoop",
  "Jump",
  "JumpConstant",
  "JumpIfNullConstant",
  "JumpIfNotNullConstant",
  "JumpIfUndefinedConstant",
  "JumpIfNotUndefinedConstant",
  "JumpIfUndefinedOrNullConstant",
  "JumpIfTrueConstant",
  "JumpIfFalseConstant",
  "JumpIfJSReceiverConstant",
  "JumpIfForInDoneConstant",
  "JumpIfToBooleanTrueConstant",
  "JumpIfToBooleanFalseConstant",
  "JumpIfToBooleanTrue",
  "JumpIfToBooleanFalse",
  "JumpIfTrue",
  "JumpIfFalse",
  "JumpIfNull",
  "JumpIfNotNull",
  "JumpIfUndefined",
  "JumpIfNotUndefined",
  "JumpIfUndefinedOrNull",
  "JumpIfJSReceiver",
  "JumpIfForInDone",
  "SwitchOnSmiNoFeedback",
  "ForInEnumerate",
  "ForInPrepare",
  "ForInNext",
  "ForInStep",
  "ForOfNext",
  "SetPendingMessage",
  "Throw",
  "ReThrow",
  "Return",
  "ThrowReferenceErrorIfTdzHole",
  "ThrowSuperNotCalledIfTdzHole",
  "ThrowSuperAlreadyCalledIfNotTdzHole",
  "ThrowIfNotSuperConstructor",
  "SwitchOnGeneratorState",
  "SuspendGenerator",
  "ResumeGenerator",
  "GetIterator",
  "ArrayDestructure",
  "Debugger",
  "IncBlockCounter",
  "Abort",
  "Star15",
  "Star14",
  "Star13",
  "Star12",
  "Star11",
  "Star10",
  "Star9",
  "Star8",
  "Star7",
  "Star6",
  "Star5",
  "Star4",
  "Star3",
  "Star2",
  "Star1",
  "Star0",
  "Illegal",
];

/// 每条字节码的隐式累加器用法 (bytecodes.cc kImplicitRegisterUse)
pub static IMPLICIT_REGISTER_USE: [ImplicitRegisterUse; BYTECODE_COUNT] = [
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::READ_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::WRITE_ACCUMULATOR,
  ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::CLOBBER_ACCUMULATOR,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::NONE,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR,
  ImplicitRegisterUse::NONE,
];

/// 每条字节码的操作数类型 (bytecodes.cc kOperandTypes)
pub static OPERAND_TYPES: [&[OperandType]; BYTECODE_COUNT] = [
  &[],
  &[],
  &[],
  &[],
  &[],
  &[OperandType::Reg],
  &[OperandType::Reg, OperandType::Reg],
  &[OperandType::Reg, OperandType::Reg, OperandType::Reg],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::Reg,
    OperandType::Reg,
  ],
  &[OperandType::RuntimeId, OperandType::Reg, OperandType::Reg],
  &[
    OperandType::RuntimeId,
    OperandType::Reg,
    OperandType::Reg,
    OperandType::Reg,
  ],
  &[OperandType::Reg],
  &[],
  &[OperandType::Imm],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[OperandType::ConstantPoolIndex],
  &[
    OperandType::Reg,
    OperandType::ContextSlot,
    OperandType::UImm,
  ],
  &[
    OperandType::Reg,
    OperandType::ContextSlot,
    OperandType::UImm,
  ],
  &[
    OperandType::Reg,
    OperandType::ContextSlot,
    OperandType::UImm,
  ],
  &[OperandType::ContextSlot],
  &[OperandType::ContextSlot],
  &[OperandType::ContextSlot],
  &[OperandType::RegOut],
  &[OperandType::Reg, OperandType::RegOut],
  &[OperandType::RegOut],
  &[OperandType::Reg],
  &[OperandType::Reg],
  &[],
  &[],
  &[],
  &[OperandType::Flag8],
  &[OperandType::ConstantPoolIndex, OperandType::FeedbackSlot],
  &[OperandType::ConstantPoolIndex, OperandType::FeedbackSlot],
  &[OperandType::ConstantPoolIndex, OperandType::FeedbackSlot],
  &[
    OperandType::Reg,
    OperandType::ContextSlot,
    OperandType::UImm,
  ],
  &[OperandType::ContextSlot],
  &[
    OperandType::Reg,
    OperandType::ContextSlot,
    OperandType::UImm,
  ],
  &[OperandType::ContextSlot],
  &[OperandType::ConstantPoolIndex],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::ContextSlot,
    OperandType::UImm,
  ],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::ContextSlot,
    OperandType::UImm,
  ],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
    OperandType::UImm,
  ],
  &[OperandType::ConstantPoolIndex],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::ContextSlot,
    OperandType::UImm,
  ],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::ContextSlot,
    OperandType::UImm,
  ],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
    OperandType::UImm,
  ],
  &[OperandType::ConstantPoolIndex, OperandType::Flag8],
  &[
    OperandType::Reg,
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
  ],
  &[OperandType::Reg, OperandType::FeedbackSlot],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::ContextSlot,
    OperandType::UImm,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[OperandType::Imm, OperandType::UImm],
  &[OperandType::Imm, OperandType::UImm],
  &[
    OperandType::Reg,
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::Flag8,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::Flag8,
    OperandType::FeedbackSlot,
  ],
  &[OperandType::ConstantPoolIndex, OperandType::FeedbackSlot],
  &[
    OperandType::Reg,
    OperandType::ContextSlot,
    OperandType::UImm,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[
    OperandType::Reg,
    OperandType::FeedbackSlot,
    OperandType::Flag8,
  ],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::Imm, OperandType::EmbeddedFeedback],
  &[OperandType::EmbeddedFeedback],
  &[OperandType::EmbeddedFeedback],
  &[OperandType::EmbeddedFeedback],
  &[OperandType::EmbeddedFeedback],
  &[],
  &[],
  &[OperandType::FeedbackSlot],
  &[OperandType::Reg],
  &[OperandType::Reg],
  &[OperandType::RegOut],
  &[OperandType::Reg, OperandType::Reg, OperandType::RegOutPair],
  &[
    OperandType::Reg,
    OperandType::RegList,
    OperandType::RegCount,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::RegList,
    OperandType::RegCount,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::Reg,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::RegList,
    OperandType::RegCount,
    OperandType::FeedbackSlot,
  ],
  &[OperandType::Reg, OperandType::FeedbackSlot],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::RegList,
    OperandType::RegCount,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::RuntimeId,
    OperandType::RegList,
    OperandType::RegCount,
  ],
  &[
    OperandType::RuntimeId,
    OperandType::RegList,
    OperandType::RegCount,
    OperandType::RegOutPair,
  ],
  &[
    OperandType::NativeContextIndex,
    OperandType::RegList,
    OperandType::RegCount,
  ],
  &[
    OperandType::IntrinsicId,
    OperandType::RegList,
    OperandType::RegCount,
  ],
  &[
    OperandType::Reg,
    OperandType::RegList,
    OperandType::RegCount,
    OperandType::FeedbackSlot,
  ],
  &[
    OperandType::Reg,
    OperandType::RegList,
    OperandType::RegCount,
    OperandType::FeedbackSlot,
  ],
  &[OperandType::Reg, OperandType::FeedbackSlot],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::EmbeddedFeedback],
  &[OperandType::Reg, OperandType::FeedbackSlot],
  &[OperandType::Reg, OperandType::FeedbackSlot],
  &[],
  &[OperandType::FeedbackSlot],
  &[OperandType::FeedbackSlot],
  &[OperandType::RegOut],
  &[],
  &[],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
    OperandType::Flag16,
  ],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
    OperandType::Flag8,
  ],
  &[],
  &[OperandType::FeedbackSlot],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
    OperandType::Flag8,
  ],
  &[],
  &[
    OperandType::Reg,
    OperandType::Flag8,
    OperandType::FeedbackSlot,
  ],
  &[OperandType::ConstantPoolIndex, OperandType::FeedbackSlot],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
    OperandType::Flag8,
  ],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::Reg, OperandType::ConstantPoolIndex],
  &[OperandType::ConstantPoolIndex, OperandType::UImm],
  &[OperandType::ConstantPoolIndex, OperandType::UImm],
  &[OperandType::ConstantPoolIndex, OperandType::UImm],
  &[OperandType::Reg, OperandType::ConstantPoolIndex],
  &[],
  &[],
  &[],
  &[
    OperandType::UImm,
    OperandType::Imm,
    OperandType::FeedbackSlot,
  ],
  &[OperandType::UImm],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::ConstantPoolIndex],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::Reg,
    OperandType::Reg,
  ],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::ConstantPoolIndex],
  &[OperandType::UImm],
  &[OperandType::UImm],
  &[OperandType::UImm],
  &[OperandType::UImm],
  &[OperandType::UImm],
  &[OperandType::UImm],
  &[OperandType::UImm],
  &[OperandType::UImm],
  &[OperandType::UImm],
  &[OperandType::UImm],
  &[OperandType::UImm, OperandType::Reg, OperandType::Reg],
  &[
    OperandType::ConstantPoolIndex,
    OperandType::UImm,
    OperandType::Imm,
  ],
  &[OperandType::Reg],
  &[OperandType::RegOutTriple, OperandType::FeedbackSlot],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::RegPair,
    OperandType::FeedbackSlot,
  ],
  &[OperandType::RegInOut],
  &[
    OperandType::Reg,
    OperandType::Reg,
    OperandType::FeedbackSlot,
  ],
  &[],
  &[],
  &[],
  &[],
  &[OperandType::ConstantPoolIndex],
  &[],
  &[],
  &[OperandType::Reg],
  &[
    OperandType::Reg,
    OperandType::ConstantPoolIndex,
    OperandType::UImm,
  ],
  &[
    OperandType::Reg,
    OperandType::RegList,
    OperandType::RegCount,
    OperandType::UImm,
  ],
  &[
    OperandType::Reg,
    OperandType::RegOutList,
    OperandType::RegCount,
  ],
  &[
    OperandType::Reg,
    OperandType::FeedbackSlot,
    OperandType::FeedbackSlot,
  ],
  &[OperandType::RegOutList, OperandType::RegCount],
  &[],
  &[OperandType::CoverageSlot],
  &[OperandType::AbortReason],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
];

/// 每条字节码的操作数元信息 (bytecodes.cc kOperandTypeInfos)
pub static OPERAND_TYPE_INFOS: [&[OperandTypeInfo]; BYTECODE_COUNT] = [
  &[],
  &[],
  &[],
  &[],
  &[],
  &[OperandTypeInfo::ScalableSignedByte],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[
    OperandTypeInfo::FixedUnsignedShort,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[
    OperandTypeInfo::FixedUnsignedShort,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[OperandTypeInfo::ScalableSignedByte],
  &[],
  &[OperandTypeInfo::ScalableSignedByte],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableSignedByte],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[OperandTypeInfo::ScalableSignedByte],
  &[OperandTypeInfo::ScalableSignedByte],
  &[OperandTypeInfo::ScalableSignedByte],
  &[],
  &[],
  &[],
  &[OperandTypeInfo::FixedUnsignedByte],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[OperandTypeInfo::FixedUnsignedByte],
  &[OperandTypeInfo::FixedUnsignedByte],
  &[OperandTypeInfo::FixedUnsignedByte],
  &[OperandTypeInfo::FixedUnsignedByte],
  &[],
  &[],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableSignedByte],
  &[OperandTypeInfo::ScalableSignedByte],
  &[OperandTypeInfo::ScalableSignedByte],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::FixedUnsignedShort,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::FixedUnsignedShort,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[
    OperandTypeInfo::FixedUnsignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::FixedUnsignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableSignedByte],
  &[],
  &[],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::FixedUnsignedShort,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::FixedUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::FixedUnsignedByte,
  ],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[],
  &[],
  &[],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableSignedByte,
  ],
  &[OperandTypeInfo::ScalableSignedByte],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[OperandTypeInfo::ScalableSignedByte],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[],
  &[],
  &[],
  &[],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[],
  &[],
  &[OperandTypeInfo::ScalableSignedByte],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[
    OperandTypeInfo::ScalableSignedByte,
    OperandTypeInfo::ScalableUnsignedByte,
  ],
  &[],
  &[OperandTypeInfo::ScalableUnsignedByte],
  &[OperandTypeInfo::FixedUnsignedByte],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
  &[],
];

/// 每条字节码的操作数个数 (bytecodes.cc kOperandCount)
#[rustfmt::skip]
pub static OPERAND_COUNT: [u8; BYTECODE_COUNT] = [
  0, 0, 0, 0, 0, 1, 2, 3, 4, 3, 4, 1, 0, 1, 0, 0, 0, 0, 0, 0,
  1, 3, 3, 3, 1, 1, 1, 1, 2, 1, 1, 1, 0, 0, 0, 1, 2, 2, 2, 3,
  1, 3, 1, 1, 3, 3, 3, 1, 3, 3, 3, 2, 3, 3, 2, 4, 5, 2, 2, 3,
  3, 3, 3, 4, 3, 4, 2, 5, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 0, 0, 1,
  1, 1, 1, 3, 4, 4, 3, 4, 5, 4, 2, 3, 4, 4, 3, 4, 3, 3, 4, 4,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 1, 0, 0, 3, 3, 0, 1, 3,
  0, 3, 2, 3, 1, 2, 2, 2, 2, 2, 0, 0, 0, 3, 1, 1, 1, 1, 1, 1,
  1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 1,
  2, 4, 1, 3, 0, 0, 0, 0, 1, 0, 0, 1, 3, 4, 3, 3, 2, 0, 1, 1,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// 三档刻度下的字节码总尺寸 (bytecodes.cc kBytecodeSizes[3][N]; 行 = Single/Double/Quadruple)
#[rustfmt::skip]
pub static BYTECODE_SIZES: [[u8; BYTECODE_COUNT]; 3] = [
  [1, 1, 1, 1, 1, 2, 3, 4, 5, 5, 6, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 4, 4, 4, 2, 2, 2, 2, 3, 2, 2, 2, 1, 1, 1, 2, 3, 3, 3, 4, 2, 4, 2, 2, 4, 4, 4, 2, 4, 4, 4, 3, 4, 4, 3, 5, 6, 3, 3, 4, 4, 4, 4, 5, 4, 5, 3, 6, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 4, 5, 5, 4, 5, 6, 5, 3, 4, 5, 5, 5, 6, 4, 4, 5, 5, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 2, 2, 2, 1, 1, 5, 4, 1, 2, 4, 1, 4, 3, 4, 2, 3, 3, 3, 3, 3, 1, 1, 1, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 4, 2, 3, 5, 2, 4, 1, 1, 1, 1, 2, 1, 1, 2, 4, 5, 4, 4, 3, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
  [1, 1, 1, 1, 1, 3, 5, 7, 9, 7, 9, 3, 1, 3, 1, 1, 1, 1, 1, 1, 3, 7, 7, 7, 3, 3, 3, 3, 5, 3, 3, 3, 1, 1, 1, 2, 5, 5, 5, 7, 3, 7, 3, 3, 7, 7, 7, 3, 7, 7, 7, 4, 7, 7, 5, 9, 11, 5, 5, 7, 7, 7, 7, 8, 7, 8, 5, 11, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 6, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 1, 1, 3, 3, 3, 3, 7, 9, 9, 7, 9, 11, 9, 5, 7, 9, 9, 7, 9, 6, 6, 9, 9, 5, 4, 4, 4, 4, 4, 4, 5, 5, 1, 3, 3, 3, 1, 1, 7, 6, 1, 3, 6, 1, 6, 5, 6, 3, 5, 5, 5, 5, 5, 1, 1, 1, 7, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 7, 7, 3, 5, 9, 3, 7, 1, 1, 1, 1, 3, 1, 1, 3, 7, 9, 7, 7, 5, 1, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
  [1, 1, 1, 1, 1, 5, 9, 13, 17, 11, 15, 5, 1, 5, 1, 1, 1, 1, 1, 1, 5, 13, 13, 13, 5, 5, 5, 5, 9, 5, 5, 5, 1, 1, 1, 2, 9, 9, 9, 13, 5, 13, 5, 5, 13, 13, 13, 5, 13, 13, 13, 6, 13, 13, 9, 17, 21, 9, 9, 13, 13, 13, 13, 14, 13, 14, 9, 21, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 10, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 2, 2, 2, 2, 1, 1, 5, 5, 5, 5, 13, 17, 17, 13, 17, 21, 17, 9, 13, 17, 17, 11, 15, 10, 10, 17, 17, 9, 6, 6, 6, 6, 6, 6, 9, 9, 1, 5, 5, 5, 1, 1, 11, 10, 1, 5, 10, 1, 10, 9, 10, 5, 9, 9, 9, 9, 9, 1, 1, 1, 13, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 13, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 13, 13, 5, 9, 17, 5, 13, 1, 1, 1, 1, 5, 1, 1, 5, 13, 17, 13, 13, 9, 1, 5, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

/// 三档刻度下每操作数尺寸 (bytecodes.cc kOperandSizes[3][N])
pub static OPERAND_SIZES: [[&[OperandSize]; BYTECODE_COUNT]; 3] = [
  [
    &[],
    &[],
    &[],
    &[],
    &[],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Short, OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Short,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte],
    &[],
    &[OperandSize::Byte],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[],
    &[],
    &[],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[],
    &[],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Short, OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Short,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[],
    &[],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Short],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[],
    &[],
    &[],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[],
    &[],
    &[],
    &[],
    &[OperandSize::Byte],
    &[],
    &[],
    &[OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
      OperandSize::Byte,
    ],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte, OperandSize::Byte],
    &[OperandSize::Byte, OperandSize::Byte],
    &[],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
  ],
  [
    &[],
    &[],
    &[],
    &[],
    &[],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Short],
    &[],
    &[OperandSize::Short],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[],
    &[],
    &[],
    &[OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Byte,
      OperandSize::Short,
    ],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Byte,
      OperandSize::Short,
    ],
    &[OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[],
    &[],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Byte, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Byte, OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[],
    &[],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Byte],
    &[],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Byte],
    &[],
    &[OperandSize::Short, OperandSize::Byte, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Byte],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[],
    &[],
    &[],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[],
    &[],
    &[],
    &[],
    &[OperandSize::Short],
    &[],
    &[],
    &[OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
      OperandSize::Short,
    ],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short, OperandSize::Short],
    &[OperandSize::Short, OperandSize::Short],
    &[],
    &[OperandSize::Short],
    &[OperandSize::Byte],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
  ],
  [
    &[],
    &[],
    &[],
    &[],
    &[],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Short, OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Short,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Quad],
    &[],
    &[OperandSize::Quad],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[],
    &[],
    &[],
    &[OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Byte,
      OperandSize::Quad,
    ],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Byte,
      OperandSize::Quad,
    ],
    &[OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[OperandSize::Byte],
    &[],
    &[],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Short, OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Short,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Byte, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Byte, OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[],
    &[],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Short],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Byte],
    &[],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Byte],
    &[],
    &[OperandSize::Quad, OperandSize::Byte, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Byte],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[],
    &[],
    &[],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[],
    &[],
    &[],
    &[],
    &[OperandSize::Quad],
    &[],
    &[],
    &[OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
      OperandSize::Quad,
    ],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad, OperandSize::Quad],
    &[OperandSize::Quad, OperandSize::Quad],
    &[],
    &[OperandSize::Quad],
    &[OperandSize::Byte],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
  ],
];

/// 三档刻度下每操作数偏移 (bytecodes.cc kOperandOffsets[3][N]; 首操作数恒为 1, 紧跟操作码)
pub static OPERAND_OFFSETS: [[&[u8]; BYTECODE_COUNT]; 3] = [
  [
    &[],
    &[],
    &[],
    &[],
    &[],
    &[1],
    &[1, 2],
    &[1, 2, 3],
    &[1, 2, 3, 4],
    &[1, 3, 4],
    &[1, 3, 4, 5],
    &[1],
    &[],
    &[1],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[1],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 2],
    &[1],
    &[1],
    &[1],
    &[],
    &[],
    &[],
    &[1],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2, 3],
    &[1],
    &[1, 2, 3],
    &[1],
    &[1],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1, 2],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1, 2],
    &[1, 2, 3, 4],
    &[1, 2, 3, 4, 5],
    &[1, 2],
    &[1, 2],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1, 2, 3, 4],
    &[1, 2, 3],
    &[1, 2, 3, 4],
    &[1, 2],
    &[1, 2, 3, 4, 5],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2, 3],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1],
    &[1],
    &[1],
    &[1],
    &[],
    &[],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 2, 3],
    &[1, 2, 3, 4],
    &[1, 2, 3, 4],
    &[1, 2, 3],
    &[1, 2, 3, 4],
    &[1, 2, 3, 4, 5],
    &[1, 2, 3, 4],
    &[1, 2],
    &[1, 2, 3],
    &[1, 2, 3, 4],
    &[1, 2, 3, 4],
    &[1, 3, 4],
    &[1, 3, 4, 5],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1, 2, 3, 4],
    &[1, 2, 3, 4],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[],
    &[1],
    &[1],
    &[1],
    &[],
    &[],
    &[1, 2, 3],
    &[1, 2, 3],
    &[],
    &[1],
    &[1, 2, 3],
    &[],
    &[1, 2, 3],
    &[1, 2],
    &[1, 2, 3],
    &[1],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[1, 2],
    &[],
    &[],
    &[],
    &[1, 2, 3],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 2, 3],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1],
    &[1, 2],
    &[1, 2, 3, 4],
    &[1],
    &[1, 2, 3],
    &[],
    &[],
    &[],
    &[],
    &[1],
    &[],
    &[],
    &[1],
    &[1, 2, 3],
    &[1, 2, 3, 4],
    &[1, 2, 3],
    &[1, 2, 3],
    &[1, 2],
    &[],
    &[1],
    &[1],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
  ],
  [
    &[],
    &[],
    &[],
    &[],
    &[],
    &[1],
    &[1, 3],
    &[1, 3, 5],
    &[1, 3, 5, 7],
    &[1, 3, 5],
    &[1, 3, 5, 7],
    &[1],
    &[],
    &[1],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[1],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 3],
    &[1],
    &[1],
    &[1],
    &[],
    &[],
    &[],
    &[1],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3, 5],
    &[1],
    &[1, 3, 5],
    &[1],
    &[1],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1, 3],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1, 3],
    &[1, 3, 5, 7],
    &[1, 3, 5, 7, 9],
    &[1, 3],
    &[1, 3],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1, 3, 5, 6],
    &[1, 3, 5],
    &[1, 3, 5, 6],
    &[1, 3],
    &[1, 3, 5, 7, 9],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3, 5],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1],
    &[1],
    &[1],
    &[1],
    &[],
    &[],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 3, 5],
    &[1, 3, 5, 7],
    &[1, 3, 5, 7],
    &[1, 3, 5],
    &[1, 3, 5, 7],
    &[1, 3, 5, 7, 9],
    &[1, 3, 5, 7],
    &[1, 3],
    &[1, 3, 5],
    &[1, 3, 5, 7],
    &[1, 3, 5, 7],
    &[1, 3, 5],
    &[1, 3, 5, 7],
    &[1, 2, 4],
    &[1, 2, 4],
    &[1, 3, 5, 7],
    &[1, 3, 5, 7],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[],
    &[1],
    &[1],
    &[1],
    &[],
    &[],
    &[1, 3, 5],
    &[1, 3, 5],
    &[],
    &[1],
    &[1, 3, 5],
    &[],
    &[1, 3, 4],
    &[1, 3],
    &[1, 3, 5],
    &[1],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[1, 3],
    &[],
    &[],
    &[],
    &[1, 3, 5],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 3, 5],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1],
    &[1, 3],
    &[1, 3, 5, 7],
    &[1],
    &[1, 3, 5],
    &[],
    &[],
    &[],
    &[],
    &[1],
    &[],
    &[],
    &[1],
    &[1, 3, 5],
    &[1, 3, 5, 7],
    &[1, 3, 5],
    &[1, 3, 5],
    &[1, 3],
    &[],
    &[1],
    &[1],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
  ],
  [
    &[],
    &[],
    &[],
    &[],
    &[],
    &[1],
    &[1, 5],
    &[1, 5, 9],
    &[1, 5, 9, 13],
    &[1, 3, 7],
    &[1, 3, 7, 11],
    &[1],
    &[],
    &[1],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[1],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 5],
    &[1],
    &[1],
    &[1],
    &[],
    &[],
    &[],
    &[1],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5, 9],
    &[1],
    &[1, 5, 9],
    &[1],
    &[1],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1, 5],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1, 5],
    &[1, 5, 9, 13],
    &[1, 5, 9, 13, 17],
    &[1, 5],
    &[1, 5],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1, 5, 9, 10],
    &[1, 5, 9],
    &[1, 5, 9, 10],
    &[1, 5],
    &[1, 5, 9, 13, 17],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5, 9],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1],
    &[1],
    &[1],
    &[1],
    &[],
    &[],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 5, 9],
    &[1, 5, 9, 13],
    &[1, 5, 9, 13],
    &[1, 5, 9],
    &[1, 5, 9, 13],
    &[1, 5, 9, 13, 17],
    &[1, 5, 9, 13],
    &[1, 5],
    &[1, 5, 9],
    &[1, 5, 9, 13],
    &[1, 5, 9, 13],
    &[1, 3, 7],
    &[1, 3, 7, 11],
    &[1, 2, 6],
    &[1, 2, 6],
    &[1, 5, 9, 13],
    &[1, 5, 9, 13],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[],
    &[1],
    &[1],
    &[1],
    &[],
    &[],
    &[1, 5, 9],
    &[1, 5, 9],
    &[],
    &[1],
    &[1, 5, 9],
    &[],
    &[1, 5, 6],
    &[1, 5],
    &[1, 5, 9],
    &[1],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[1, 5],
    &[],
    &[],
    &[],
    &[1, 5, 9],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 5, 9],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1],
    &[1, 5],
    &[1, 5, 9, 13],
    &[1],
    &[1, 5, 9],
    &[],
    &[],
    &[],
    &[],
    &[1],
    &[],
    &[],
    &[1],
    &[1, 5, 9],
    &[1, 5, 9, 13],
    &[1, 5, 9],
    &[1, 5, 9],
    &[1, 5],
    &[],
    &[1],
    &[1],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
  ],
];

// —— 分类宏展开 (bytecodes.h X-macro → 常量数组) ——

/// DebugBreak 平铺族 (原版 DEBUG_BREAK_PLAIN_BYTECODE_LIST, 7 条)
pub const DEBUG_BREAK_PLAIN: [Bytecode; 7] = [
  Bytecode::DebugBreak0,
  Bytecode::DebugBreak1,
  Bytecode::DebugBreak2,
  Bytecode::DebugBreak3,
  Bytecode::DebugBreak4,
  Bytecode::DebugBreak5,
  Bytecode::DebugBreak6,
];

/// DebugBreak 前缀族 (原版 DEBUG_BREAK_PREFIX_BYTECODE_LIST)
pub const DEBUG_BREAK_PREFIX: [Bytecode; 2] =
  [Bytecode::DebugBreakWide, Bytecode::DebugBreakExtraWide];

/// DebugBreak 全族 (原版 DEBUG_BREAK_BYTECODE_LIST)
pub const DEBUG_BREAK_ALL: [Bytecode; 9] = [
  Bytecode::DebugBreak0,
  Bytecode::DebugBreak1,
  Bytecode::DebugBreak2,
  Bytecode::DebugBreak3,
  Bytecode::DebugBreak4,
  Bytecode::DebugBreak5,
  Bytecode::DebugBreak6,
  Bytecode::DebugBreakWide,
  Bytecode::DebugBreakExtraWide,
];

/// 无条件立即跳转 (原版 JUMP_UNCONDITIONAL_IMMEDIATE_BYTECODE_LIST)
pub const JUMP_UNCONDITIONAL_IMMEDIATE: [Bytecode; 2] = [Bytecode::JumpLoop, Bytecode::Jump];

/// 无条件常量池跳转 (原版 JUMP_UNCONDITIONAL_CONSTANT_BYTECODE_LIST)
pub const JUMP_UNCONDITIONAL_CONSTANT: [Bytecode; 1] = [Bytecode::JumpConstant];

/// ToBoolean 条件立即跳转 (原版 JUMP_TOBOOLEAN_CONDITIONAL_IMMEDIATE_BYTECODE_LIST)
pub const JUMP_TOBOOLEAN_CONDITIONAL_IMMEDIATE: [Bytecode; 2] = [
  Bytecode::JumpIfToBooleanTrue,
  Bytecode::JumpIfToBooleanFalse,
];

/// ToBoolean 条件常量跳转 (原版 JUMP_TOBOOLEAN_CONDITIONAL_CONSTANT_BYTECODE_LIST)
pub const JUMP_TOBOOLEAN_CONDITIONAL_CONSTANT: [Bytecode; 2] = [
  Bytecode::JumpIfToBooleanTrueConstant,
  Bytecode::JumpIfToBooleanFalseConstant,
];

/// 条件立即跳转 (原版 JUMP_CONDITIONAL_IMMEDIATE_BYTECODE_LIST, 含 ToBoolean 展开)
pub const JUMP_CONDITIONAL_IMMEDIATE: [Bytecode; 11] = [
  Bytecode::JumpIfToBooleanTrue,
  Bytecode::JumpIfToBooleanFalse,
  Bytecode::JumpIfTrue,
  Bytecode::JumpIfFalse,
  Bytecode::JumpIfNull,
  Bytecode::JumpIfNotNull,
  Bytecode::JumpIfUndefined,
  Bytecode::JumpIfNotUndefined,
  Bytecode::JumpIfUndefinedOrNull,
  Bytecode::JumpIfJSReceiver,
  Bytecode::JumpIfForInDone,
];

/// 条件常量跳转 (原版 JUMP_CONDITIONAL_CONSTANT_BYTECODE_LIST, 含 ToBoolean 展开)
pub const JUMP_CONDITIONAL_CONSTANT: [Bytecode; 11] = [
  Bytecode::JumpIfToBooleanTrueConstant,
  Bytecode::JumpIfToBooleanFalseConstant,
  Bytecode::JumpIfNullConstant,
  Bytecode::JumpIfNotNullConstant,
  Bytecode::JumpIfUndefinedConstant,
  Bytecode::JumpIfNotUndefinedConstant,
  Bytecode::JumpIfUndefinedOrNullConstant,
  Bytecode::JumpIfTrueConstant,
  Bytecode::JumpIfFalseConstant,
  Bytecode::JumpIfJSReceiverConstant,
  Bytecode::JumpIfForInDoneConstant,
];

/// 常量池跳转全族 (原版 JUMP_CONSTANT_BYTECODE_LIST)
pub const JUMP_CONSTANT_ALL: [Bytecode; 12] = [
  Bytecode::JumpConstant,
  Bytecode::JumpIfToBooleanTrueConstant,
  Bytecode::JumpIfToBooleanFalseConstant,
  Bytecode::JumpIfNullConstant,
  Bytecode::JumpIfNotNullConstant,
  Bytecode::JumpIfUndefinedConstant,
  Bytecode::JumpIfNotUndefinedConstant,
  Bytecode::JumpIfUndefinedOrNullConstant,
  Bytecode::JumpIfTrueConstant,
  Bytecode::JumpIfFalseConstant,
  Bytecode::JumpIfJSReceiverConstant,
  Bytecode::JumpIfForInDoneConstant,
];

/// 立即跳转全族 (原版 JUMP_IMMEDIATE_BYTECODE_LIST)
pub const JUMP_IMMEDIATE_ALL: [Bytecode; 13] = [
  Bytecode::JumpLoop,
  Bytecode::Jump,
  Bytecode::JumpIfToBooleanTrue,
  Bytecode::JumpIfToBooleanFalse,
  Bytecode::JumpIfTrue,
  Bytecode::JumpIfFalse,
  Bytecode::JumpIfNull,
  Bytecode::JumpIfNotNull,
  Bytecode::JumpIfUndefined,
  Bytecode::JumpIfNotUndefined,
  Bytecode::JumpIfUndefinedOrNull,
  Bytecode::JumpIfJSReceiver,
  Bytecode::JumpIfForInDone,
];

/// ToBoolean 跳转 (原版 JUMP_TO_BOOLEAN_BYTECODE_LIST)
pub const JUMP_TO_BOOLEAN: [Bytecode; 4] = [
  Bytecode::JumpIfToBooleanTrue,
  Bytecode::JumpIfToBooleanFalse,
  Bytecode::JumpIfToBooleanTrueConstant,
  Bytecode::JumpIfToBooleanFalseConstant,
];

/// 无条件跳转全族 (原版 JUMP_UNCONDITIONAL_BYTECODE_LIST)
pub const JUMP_UNCONDITIONAL_ALL: [Bytecode; 3] =
  [Bytecode::JumpLoop, Bytecode::Jump, Bytecode::JumpConstant];

/// 条件跳转全族 (原版 JUMP_CONDITIONAL_BYTECODE_LIST)
pub const JUMP_CONDITIONAL_ALL: [Bytecode; 22] = [
  Bytecode::JumpIfToBooleanTrue,
  Bytecode::JumpIfToBooleanFalse,
  Bytecode::JumpIfTrue,
  Bytecode::JumpIfFalse,
  Bytecode::JumpIfNull,
  Bytecode::JumpIfNotNull,
  Bytecode::JumpIfUndefined,
  Bytecode::JumpIfNotUndefined,
  Bytecode::JumpIfUndefinedOrNull,
  Bytecode::JumpIfJSReceiver,
  Bytecode::JumpIfForInDone,
  Bytecode::JumpIfToBooleanTrueConstant,
  Bytecode::JumpIfToBooleanFalseConstant,
  Bytecode::JumpIfNullConstant,
  Bytecode::JumpIfNotNullConstant,
  Bytecode::JumpIfUndefinedConstant,
  Bytecode::JumpIfNotUndefinedConstant,
  Bytecode::JumpIfUndefinedOrNullConstant,
  Bytecode::JumpIfTrueConstant,
  Bytecode::JumpIfFalseConstant,
  Bytecode::JumpIfJSReceiverConstant,
  Bytecode::JumpIfForInDoneConstant,
];

/// 前向跳转 (原版 JUMP_FORWARD_BYTECODE_LIST)
pub const JUMP_FORWARD: [Bytecode; 24] = [
  Bytecode::Jump,
  Bytecode::JumpConstant,
  Bytecode::JumpIfToBooleanTrue,
  Bytecode::JumpIfToBooleanFalse,
  Bytecode::JumpIfTrue,
  Bytecode::JumpIfFalse,
  Bytecode::JumpIfNull,
  Bytecode::JumpIfNotNull,
  Bytecode::JumpIfUndefined,
  Bytecode::JumpIfNotUndefined,
  Bytecode::JumpIfUndefinedOrNull,
  Bytecode::JumpIfJSReceiver,
  Bytecode::JumpIfForInDone,
  Bytecode::JumpIfToBooleanTrueConstant,
  Bytecode::JumpIfToBooleanFalseConstant,
  Bytecode::JumpIfNullConstant,
  Bytecode::JumpIfNotNullConstant,
  Bytecode::JumpIfUndefinedConstant,
  Bytecode::JumpIfNotUndefinedConstant,
  Bytecode::JumpIfUndefinedOrNullConstant,
  Bytecode::JumpIfTrueConstant,
  Bytecode::JumpIfFalseConstant,
  Bytecode::JumpIfJSReceiverConstant,
  Bytecode::JumpIfForInDoneConstant,
];

/// 跳转全族 (原版 JUMP_BYTECODE_LIST = 前向 + JumpLoop)
pub const JUMP_ALL: [Bytecode; 25] = [
  Bytecode::Jump,
  Bytecode::JumpConstant,
  Bytecode::JumpIfToBooleanTrue,
  Bytecode::JumpIfToBooleanFalse,
  Bytecode::JumpIfTrue,
  Bytecode::JumpIfFalse,
  Bytecode::JumpIfNull,
  Bytecode::JumpIfNotNull,
  Bytecode::JumpIfUndefined,
  Bytecode::JumpIfNotUndefined,
  Bytecode::JumpIfUndefinedOrNull,
  Bytecode::JumpIfJSReceiver,
  Bytecode::JumpIfForInDone,
  Bytecode::JumpIfToBooleanTrueConstant,
  Bytecode::JumpIfToBooleanFalseConstant,
  Bytecode::JumpIfNullConstant,
  Bytecode::JumpIfNotNullConstant,
  Bytecode::JumpIfUndefinedConstant,
  Bytecode::JumpIfNotUndefinedConstant,
  Bytecode::JumpIfUndefinedOrNullConstant,
  Bytecode::JumpIfTrueConstant,
  Bytecode::JumpIfFalseConstant,
  Bytecode::JumpIfJSReceiverConstant,
  Bytecode::JumpIfForInDoneConstant,
  Bytecode::JumpLoop,
];

/// 返回族 (原版 RETURN_BYTECODE_LIST)
pub const RETURN_ALL: [Bytecode; 2] = [Bytecode::Return, Bytecode::SuspendGenerator];

/// 无条件抛出族 (原版 UNCONDITIONAL_THROW_BYTECODE_LIST)
pub const UNCONDITIONAL_THROW: [Bytecode; 2] = [Bytecode::Throw, Bytecode::ReThrow];

/// 静态谓词集合 (原版 class Bytecodes : AllStatic)
pub struct Bytecodes;

impl Bytecodes {
  /// 刻度 -> 前缀字节码 (原版 OperandScaleToPrefixBytecode)
  #[inline]
  #[must_use]
  pub const fn operand_scale_to_prefix_bytecode(scale: OperandScale) -> Option<Bytecode> {
    match scale {
      OperandScale::Quadruple => Some(Bytecode::ExtraWide),
      OperandScale::Double => Some(Bytecode::Wide),
      // 原版其余刻度走 UNREACHABLE; 此处保守返回 None
      _ => None,
    }
  }

  /// 刻度是否需要前缀字节码 (原版 OperandScaleRequiresPrefixBytecode)
  #[inline]
  #[must_use]
  pub fn operand_scale_requires_prefix_bytecode(scale: OperandScale) -> bool {
    !matches!(scale, OperandScale::Single)
  }

  /// 前缀字节码 -> 刻度 (原版 PrefixBytecodeToOperandScale)
  /// 注: 原版有 V8_TARGET_OS_ANDROID 分支 (crbug.com/1379788 workaround), 语义与非安卓分支一致
  #[inline]
  #[must_use]
  pub fn prefix_bytecode_to_operand_scale(b: Bytecode) -> Option<OperandScale> {
    match b {
      Bytecode::ExtraWide | Bytecode::DebugBreakExtraWide => Some(OperandScale::Quadruple),
      Bytecode::Wide | Bytecode::DebugBreakWide => Some(OperandScale::Double),
      _ => None,
    }
  }

  /// 是否读累加器 (原版 ReadsAccumulator)
  #[inline]
  #[must_use]
  pub fn reads_accumulator(b: Bytecode) -> bool {
    b.implicit_register_use()
      .contains(ImplicitRegisterUse::READ_ACCUMULATOR)
  }

  /// 是否写累加器 (原版 WritesAccumulator)
  #[inline]
  #[must_use]
  pub fn writes_accumulator(b: Bytecode) -> bool {
    b.implicit_register_use()
      .contains(ImplicitRegisterUse::WRITE_ACCUMULATOR)
  }

  /// 是否覆写累加器 (原版 ClobbersAccumulator)
  #[inline]
  #[must_use]
  pub fn clobbers_accumulator(b: Bytecode) -> bool {
    b.implicit_register_use()
      .contains(ImplicitRegisterUse::CLOBBER_ACCUMULATOR)
  }

  /// 是否写或覆写累加器 (原版 WritesOrClobbersAccumulator)
  #[inline]
  #[must_use]
  pub fn writes_or_clobbers_accumulator(b: Bytecode) -> bool {
    b.implicit_register_use()
      .intersects(ImplicitRegisterUse::WRITE_ACCUMULATOR)
      || b
        .implicit_register_use()
        .intersects(ImplicitRegisterUse::CLOBBER_ACCUMULATOR)
  }

  /// 是否隐式写寄存器 (原版 WritesImplicitRegister)
  #[inline]
  #[must_use]
  pub fn writes_implicit_register(b: Bytecode) -> bool {
    b.implicit_register_use()
      .contains(ImplicitRegisterUse::WRITE_SHORT_STAR)
  }

  /// 无副作用累加器加载 (原版 IsAccumulatorLoadWithoutEffects; 依赖 kLdar..kLdaImmutableCurrentContextSlot 次序)
  #[inline]
  #[must_use]
  pub fn is_accumulator_load_without_effects(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::Ldar as u8 && (b as u8) <= Bytecode::LdaImmutableCurrentContextSlot as u8
  }

  /// 无副作用比较 (原版 IsCompareWithoutEffects; 依赖 kTestReferenceEqual..kTestTypeOf 次序)
  #[inline]
  #[must_use]
  pub fn is_compare_without_effects(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::TestReferenceEqual as u8 && (b as u8) <= Bytecode::TestTypeOf as u8
  }

  /// 是否 ShortStar (原版 IsShortStar)
  #[inline]
  #[must_use]
  pub fn is_short_star(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::FIRST_SHORT_STAR as u8 && (b as u8) <= Bytecode::LAST_SHORT_STAR as u8
  }

  /// 是否任意 Star (原版 IsAnyStar)
  #[inline]
  #[must_use]
  pub fn is_any_star(b: Bytecode) -> bool {
    b == Bytecode::Star || Self::is_short_star(b)
  }

  /// 无副作用寄存器加载 (原版 IsRegisterLoadWithoutEffects; kStar..=kPopContext)
  #[inline]
  #[must_use]
  pub fn is_register_load_without_effects(b: Bytecode) -> bool {
    Self::is_short_star(b)
      || ((b as u8) >= Bytecode::Star as u8 && (b as u8) <= Bytecode::PopContext as u8)
  }

  /// 条件立即跳转 (原版 IsConditionalJumpImmediate)
  #[inline]
  #[must_use]
  pub fn is_conditional_jump_immediate(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpIfToBooleanTrue as u8 && (b as u8) <= Bytecode::JumpIfForInDone as u8
  }

  /// 条件常量跳转 (原版 IsConditionalJumpConstant)
  #[inline]
  #[must_use]
  pub fn is_conditional_jump_constant(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpIfNullConstant as u8
      && (b as u8) <= Bytecode::JumpIfToBooleanFalseConstant as u8
  }

  /// 条件跳转 (原版 IsConditionalJump)
  #[inline]
  #[must_use]
  pub fn is_conditional_jump(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpIfNullConstant as u8 && (b as u8) <= Bytecode::JumpIfForInDone as u8
  }

  /// 无条件跳转 (原版 IsUnconditionalJump; kJumpLoop..=kJumpConstant)
  #[inline]
  #[must_use]
  pub fn is_unconditional_jump(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpLoop as u8 && (b as u8) <= Bytecode::JumpConstant as u8
  }

  /// 立即跳转 (原版 IsJumpImmediate)
  #[inline]
  #[must_use]
  pub fn is_jump_immediate(b: Bytecode) -> bool {
    b == Bytecode::Jump || b == Bytecode::JumpLoop || Self::is_conditional_jump_immediate(b)
  }

  /// 常量跳转 (原版 IsJumpConstant)
  #[inline]
  #[must_use]
  pub fn is_jump_constant(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpConstant as u8
      && (b as u8) <= Bytecode::JumpIfToBooleanFalseConstant as u8
  }

  /// 内部做 ToBoolean 强转的跳转 (原版 IsJumpIfToBoolean)
  #[inline]
  #[must_use]
  pub fn is_jump_if_to_boolean(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpIfToBooleanTrueConstant as u8
      && (b as u8) <= Bytecode::JumpIfToBooleanFalse as u8
  }

  /// 是否跳转 (原版 IsJump; kJumpLoop..=kJumpIfForInDone)
  #[inline]
  #[must_use]
  pub fn is_jump(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpLoop as u8 && (b as u8) <= Bytecode::JumpIfForInDone as u8
  }

  /// 前向跳转 (原版 IsForwardJump; kJump..=kJumpIfForInDone)
  #[inline]
  #[must_use]
  pub fn is_forward_jump(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::Jump as u8 && (b as u8) <= Bytecode::JumpIfForInDone as u8
  }

  /// 无副作用跳转 (原版 IsJumpWithoutEffects; JumpLoop 隐含 StackCheck 除外)
  #[inline]
  #[must_use]
  pub fn is_jump_without_effects(b: Bytecode) -> bool {
    Self::is_jump(b) && b != Bytecode::JumpLoop
  }

  /// 是否 switch (原版 IsSwitch)
  #[inline]
  #[must_use]
  pub fn is_switch(b: Bytecode) -> bool {
    b == Bytecode::SwitchOnSmiNoFeedback || b == Bytecode::SwitchOnGeneratorState
  }

  /// 无外部副作用 (原版 IsWithoutExternalSideEffects)
  #[inline]
  #[must_use]
  pub fn is_without_external_side_effects(b: Bytecode) -> bool {
    Self::is_accumulator_load_without_effects(b)
      || Self::is_register_load_without_effects(b)
      || Self::is_compare_without_effects(b)
      || Self::is_jump_without_effects(b)
      || Self::is_switch(b)
      || b == Bytecode::Return
  }

  /// 是否 Ldar 或 Star (原版 IsLdarOrStar)
  #[inline]
  #[must_use]
  pub fn is_ldar_or_star(b: Bytecode) -> bool {
    b == Bytecode::Ldar || Self::is_any_star(b)
  }

  /// 是否调用/构造 (原版 IsCallOrConstruct)
  #[inline]
  #[must_use]
  pub fn is_call_or_construct(b: Bytecode) -> bool {
    matches!(
      b,
      Bytecode::CallAnyReceiver
        | Bytecode::CallProperty
        | Bytecode::CallProperty0
        | Bytecode::CallProperty1
        | Bytecode::CallProperty2
        | Bytecode::CallUndefinedReceiver
        | Bytecode::CallUndefinedReceiver0
        | Bytecode::CallUndefinedReceiver1
        | Bytecode::CallUndefinedReceiver2
        | Bytecode::Construct
        | Bytecode::CallWithSpread
        | Bytecode::ConstructWithSpread
        | Bytecode::ConstructForwardAllArgs
        | Bytecode::CallJSRuntime
    )
  }

  /// 是否运行时调用 (原版 IsCallRuntime)
  #[inline]
  #[must_use]
  pub fn is_call_runtime(b: Bytecode) -> bool {
    matches!(
      b,
      Bytecode::CallRuntime | Bytecode::CallRuntimeForPair | Bytecode::InvokeIntrinsic
    )
  }

  /// 是否缩放前缀字节码 (原版 IsPrefixScalingBytecode)
  #[inline]
  #[must_use]
  pub fn is_prefix_scaling_bytecode(b: Bytecode) -> bool {
    matches!(
      b,
      Bytecode::ExtraWide
        | Bytecode::Wide
        | Bytecode::DebugBreakExtraWide
        | Bytecode::DebugBreakWide
    )
  }

  /// 比较类内嵌 feedback (原版 IsCompareWithEmbeddedFeedback)
  #[inline]
  #[must_use]
  pub fn is_compare_with_embedded_feedback(b: Bytecode) -> bool {
    matches!(
      b,
      Bytecode::TestEqualStrict
        | Bytecode::TestEqual
        | Bytecode::TestLessThan
        | Bytecode::TestGreaterThan
        | Bytecode::TestLessThanOrEqual
        | Bytecode::TestGreaterThanOrEqual
    )
  }

  /// 二元运算内嵌 feedback (原版 IsBinaryOpWithEmbeddedFeedback)
  #[inline]
  #[must_use]
  pub fn is_binary_op_with_embedded_feedback(b: Bytecode) -> bool {
    matches!(
      b,
      Bytecode::Add
        | Bytecode::Sub
        | Bytecode::Mul
        | Bytecode::Div
        | Bytecode::Mod
        | Bytecode::Exp
        | Bytecode::BitwiseOr
        | Bytecode::BitwiseXor
        | Bytecode::BitwiseAnd
        | Bytecode::ShiftLeft
        | Bytecode::ShiftRight
        | Bytecode::ShiftRightLogical
        | Bytecode::AddSmi
        | Bytecode::SubSmi
        | Bytecode::MulSmi
        | Bytecode::DivSmi
        | Bytecode::ModSmi
        | Bytecode::ExpSmi
        | Bytecode::BitwiseOrSmi
        | Bytecode::BitwiseXorSmi
        | Bytecode::BitwiseAndSmi
        | Bytecode::ShiftLeftSmi
        | Bytecode::ShiftRightSmi
        | Bytecode::ShiftRightLogicalSmi
    )
  }

  /// 一元运算内嵌 feedback (原版 IsUnaryOpWithEmbeddedFeedback)
  #[inline]
  #[must_use]
  pub fn is_unary_op_with_embedded_feedback(b: Bytecode) -> bool {
    matches!(
      b,
      Bytecode::Inc | Bytecode::Dec | Bytecode::Negate | Bytecode::BitwiseNot
    )
  }

  /// 是否内嵌 feedback 字节码 (原版 IsEmbeddedFeedbackBytecode)
  #[inline]
  #[must_use]
  pub fn is_embedded_feedback_bytecode(b: Bytecode) -> bool {
    Self::is_compare_with_embedded_feedback(b)
      || Self::is_binary_op_with_embedded_feedback(b)
      || Self::is_unary_op_with_embedded_feedback(b)
  }

  /// 是否返回 (原版 Returns; RETURN_BYTECODE_LIST 展开)
  #[inline]
  #[must_use]
  pub fn returns(b: Bytecode) -> bool {
    matches!(b, Bytecode::Return | Bytecode::SuspendGenerator)
  }

  /// 是否无条件抛出 (原版 UnconditionallyThrows)
  #[inline]
  #[must_use]
  pub fn unconditionally_throws(b: Bytecode) -> bool {
    matches!(b, Bytecode::Throw | Bytecode::ReThrow)
  }

  /// 操作数是否可缩放有符号 (原版 OperandIsScalableSignedByte)
  #[inline]
  #[must_use]
  pub fn operand_is_scalable_signed(b: Bytecode, i: usize) -> bool {
    b.operand_types()[i].type_info() == OperandTypeInfo::ScalableSignedByte
  }

  /// 操作数是否可缩放无符号 (原版 OperandIsScalableUnsignedByte)
  #[inline]
  #[must_use]
  pub fn operand_is_scalable_unsigned(b: Bytecode, i: usize) -> bool {
    b.operand_types()[i].type_info() == OperandTypeInfo::ScalableUnsignedByte
  }

  /// 操作数是否可缩放 (原版 OperandIsScalable)
  #[inline]
  #[must_use]
  pub fn operand_is_scalable(b: Bytecode, i: usize) -> bool {
    Self::operand_is_scalable_signed(b, i) || Self::operand_is_scalable_unsigned(b, i)
  }

  /// 是否存在可缩放操作数 (原版 IsBytecodeWithScalableOperands)
  #[inline]
  #[must_use]
  pub fn is_bytecode_with_scalable_operands(b: Bytecode) -> bool {
    let mut i = 0;
    while i < b.number_of_operands() {
      if Self::operand_is_scalable(b, i) {
        return true;
      }
      i += 1;
    }
    false
  }

  /// 是否有该刻度的 handler (原版 BytecodeHasHandler)
  #[inline]
  #[must_use]
  pub fn bytecode_has_handler(b: Bytecode, scale: OperandScale) -> bool {
    (scale == OperandScale::Single && (!Self::is_short_star(b) || b == Bytecode::Star0))
      || Self::is_bytecode_with_scalable_operands(b)
  }

  /// 有符号立即数的最低适配刻度 (原版 ScaleForSignedOperand; kMin/MaxInt8/16 = -128/127/-32768/32767)
  #[inline]
  #[must_use]
  pub fn scale_for_signed_operand(v: i32) -> OperandScale {
    if (-128..=127).contains(&v) {
      OperandScale::Single
    } else if (-32768..=32767).contains(&v) {
      OperandScale::Double
    } else {
      OperandScale::Quadruple
    }
  }

  /// 无符号立即数的最低适配刻度 (原版 ScaleForUnsignedOperand; kMaxUInt8/16 = 255/65535)
  #[inline]
  #[must_use]
  pub fn scale_for_unsigned_operand(v: u32) -> OperandScale {
    if v <= 255 {
      OperandScale::Single
    } else if v <= 65535 {
      OperandScale::Double
    } else {
      OperandScale::Quadruple
    }
  }

  /// 无符号立即数的最低适配尺寸 (原版 SizeForUnsignedOperand)
  #[inline]
  #[must_use]
  pub fn size_for_unsigned_operand(v: u32) -> OperandSize {
    if v <= 255 {
      OperandSize::Byte
    } else if v <= 65535 {
      OperandSize::Short
    } else {
      OperandSize::Quad
    }
  }

  /// 调用类字节码的 receiver 模式 (原版 GetReceiverMode; 类型就地落位, 原版 src/common/globals.h ConvertReceiverMode)
  #[inline]
  #[must_use]
  pub fn get_receiver_mode(b: Bytecode) -> Option<ReceiverMode> {
    use crate::bytecode_operands::ReceiverMode;
    match b {
      Bytecode::CallProperty => Some(ReceiverMode::NotNullOrUndefined),
      Bytecode::CallProperty0 => Some(ReceiverMode::NotNullOrUndefined),
      Bytecode::CallProperty1 => Some(ReceiverMode::NotNullOrUndefined),
      Bytecode::CallProperty2 => Some(ReceiverMode::NotNullOrUndefined),
      Bytecode::CallUndefinedReceiver => Some(ReceiverMode::NullOrUndefined),
      Bytecode::CallUndefinedReceiver0 => Some(ReceiverMode::NullOrUndefined),
      Bytecode::CallUndefinedReceiver1 => Some(ReceiverMode::NullOrUndefined),
      Bytecode::CallUndefinedReceiver2 => Some(ReceiverMode::NullOrUndefined),
      Bytecode::CallJSRuntime => Some(ReceiverMode::NullOrUndefined),
      Bytecode::CallAnyReceiver => Some(ReceiverMode::Any),
      Bytecode::Construct => Some(ReceiverMode::Any),
      Bytecode::CallWithSpread => Some(ReceiverMode::Any),
      Bytecode::ConstructWithSpread => Some(ReceiverMode::Any),
      Bytecode::InvokeIntrinsic => Some(ReceiverMode::Any),
      _ => None,
    }
  }

  /// 替换为对应 DebugBreak 字节码 (原版 GetDebugBreak; Wide/ExtraWide 特判 + 按单刻度尺寸首个匹配 DEBUG_BREAK_PLAIN)
  #[inline]
  #[must_use]
  pub fn get_debug_break(b: Bytecode) -> Bytecode {
    match b {
      Bytecode::Wide => Bytecode::DebugBreakWide,
      Bytecode::ExtraWide => Bytecode::DebugBreakExtraWide,
      Bytecode::DebugBreakWide => Bytecode::DebugBreak0,
      Bytecode::DebugBreakExtraWide => Bytecode::DebugBreak0,
      Bytecode::DebugBreak0 => Bytecode::DebugBreak0,
      Bytecode::DebugBreak1 => Bytecode::DebugBreak1,
      Bytecode::DebugBreak2 => Bytecode::DebugBreak2,
      Bytecode::DebugBreak3 => Bytecode::DebugBreak3,
      Bytecode::DebugBreak4 => Bytecode::DebugBreak4,
      Bytecode::DebugBreak5 => Bytecode::DebugBreak4,
      Bytecode::DebugBreak6 => Bytecode::DebugBreak6,
      Bytecode::Ldar => Bytecode::DebugBreak1,
      Bytecode::LdaZero => Bytecode::DebugBreak0,
      Bytecode::LdaSmi => Bytecode::DebugBreak1,
      Bytecode::LdaUndefined => Bytecode::DebugBreak0,
      Bytecode::LdaNull => Bytecode::DebugBreak0,
      Bytecode::LdaTheHole => Bytecode::DebugBreak0,
      Bytecode::LdaTdzHole => Bytecode::DebugBreak0,
      Bytecode::LdaTrue => Bytecode::DebugBreak0,
      Bytecode::LdaFalse => Bytecode::DebugBreak0,
      Bytecode::LdaConstant => Bytecode::DebugBreak1,
      Bytecode::LdaContextSlotNoCell => Bytecode::DebugBreak3,
      Bytecode::LdaContextSlot => Bytecode::DebugBreak3,
      Bytecode::LdaImmutableContextSlot => Bytecode::DebugBreak3,
      Bytecode::LdaCurrentContextSlotNoCell => Bytecode::DebugBreak1,
      Bytecode::LdaCurrentContextSlot => Bytecode::DebugBreak1,
      Bytecode::LdaImmutableCurrentContextSlot => Bytecode::DebugBreak1,
      Bytecode::Star => Bytecode::DebugBreak1,
      Bytecode::Mov => Bytecode::DebugBreak2,
      Bytecode::PushContext => Bytecode::DebugBreak1,
      Bytecode::PopContext => Bytecode::DebugBreak1,
      Bytecode::TestReferenceEqual => Bytecode::DebugBreak1,
      Bytecode::TestUndetectable => Bytecode::DebugBreak0,
      Bytecode::TestNull => Bytecode::DebugBreak0,
      Bytecode::TestUndefined => Bytecode::DebugBreak0,
      Bytecode::TestTypeOf => Bytecode::DebugBreak1,
      Bytecode::LdaGlobal => Bytecode::DebugBreak2,
      Bytecode::LdaGlobalInsideTypeof => Bytecode::DebugBreak2,
      Bytecode::StaGlobal => Bytecode::DebugBreak2,
      Bytecode::StaContextSlotNoCell => Bytecode::DebugBreak3,
      Bytecode::StaCurrentContextSlotNoCell => Bytecode::DebugBreak1,
      Bytecode::StaContextSlot => Bytecode::DebugBreak3,
      Bytecode::StaCurrentContextSlot => Bytecode::DebugBreak1,
      Bytecode::LdaLookupSlot => Bytecode::DebugBreak1,
      Bytecode::LdaLookupContextSlotNoCell => Bytecode::DebugBreak3,
      Bytecode::LdaLookupContextSlot => Bytecode::DebugBreak3,
      Bytecode::LdaLookupGlobalSlot => Bytecode::DebugBreak3,
      Bytecode::LdaLookupSlotInsideTypeof => Bytecode::DebugBreak1,
      Bytecode::LdaLookupContextSlotNoCellInsideTypeof => Bytecode::DebugBreak3,
      Bytecode::LdaLookupContextSlotInsideTypeof => Bytecode::DebugBreak3,
      Bytecode::LdaLookupGlobalSlotInsideTypeof => Bytecode::DebugBreak3,
      Bytecode::StaLookupSlot => Bytecode::DebugBreak2,
      Bytecode::GetNamedProperty => Bytecode::DebugBreak3,
      Bytecode::GetNamedPropertyFromSuper => Bytecode::DebugBreak3,
      Bytecode::GetKeyedProperty => Bytecode::DebugBreak2,
      Bytecode::GetEnumeratedKeyedProperty => Bytecode::DebugBreak4,
      Bytecode::GetPrivateField => Bytecode::DebugBreak6,
      Bytecode::LdaModuleVariable => Bytecode::DebugBreak2,
      Bytecode::StaModuleVariable => Bytecode::DebugBreak2,
      Bytecode::SetNamedProperty => Bytecode::DebugBreak3,
      Bytecode::DefineNamedOwnProperty => Bytecode::DebugBreak3,
      Bytecode::DefineNamedOwnPropertyInLiteral => Bytecode::DebugBreak3,
      Bytecode::SetKeyedProperty => Bytecode::DebugBreak3,
      Bytecode::DefineKeyedOwnProperty => Bytecode::DebugBreak4,
      Bytecode::StaInArrayLiteral => Bytecode::DebugBreak3,
      Bytecode::DefineKeyedOwnPropertyInLiteral => Bytecode::DebugBreak4,
      Bytecode::SetPrototypeProperties => Bytecode::DebugBreak2,
      Bytecode::SetPrivateField => Bytecode::DebugBreak6,
      Bytecode::Add => Bytecode::DebugBreak2,
      Bytecode::Sub => Bytecode::DebugBreak2,
      Bytecode::Mul => Bytecode::DebugBreak2,
      Bytecode::Div => Bytecode::DebugBreak2,
      Bytecode::Mod => Bytecode::DebugBreak2,
      Bytecode::Exp => Bytecode::DebugBreak2,
      Bytecode::BitwiseOr => Bytecode::DebugBreak2,
      Bytecode::BitwiseXor => Bytecode::DebugBreak2,
      Bytecode::BitwiseAnd => Bytecode::DebugBreak2,
      Bytecode::ShiftLeft => Bytecode::DebugBreak2,
      Bytecode::ShiftRight => Bytecode::DebugBreak2,
      Bytecode::ShiftRightLogical => Bytecode::DebugBreak2,
      Bytecode::AddStringConstantInternalize => Bytecode::DebugBreak3,
      Bytecode::AddSmi => Bytecode::DebugBreak2,
      Bytecode::SubSmi => Bytecode::DebugBreak2,
      Bytecode::MulSmi => Bytecode::DebugBreak2,
      Bytecode::DivSmi => Bytecode::DebugBreak2,
      Bytecode::ModSmi => Bytecode::DebugBreak2,
      Bytecode::ExpSmi => Bytecode::DebugBreak2,
      Bytecode::BitwiseOrSmi => Bytecode::DebugBreak2,
      Bytecode::BitwiseXorSmi => Bytecode::DebugBreak2,
      Bytecode::BitwiseAndSmi => Bytecode::DebugBreak2,
      Bytecode::ShiftLeftSmi => Bytecode::DebugBreak2,
      Bytecode::ShiftRightSmi => Bytecode::DebugBreak2,
      Bytecode::ShiftRightLogicalSmi => Bytecode::DebugBreak2,
      Bytecode::Inc => Bytecode::DebugBreak1,
      Bytecode::Dec => Bytecode::DebugBreak1,
      Bytecode::Negate => Bytecode::DebugBreak1,
      Bytecode::BitwiseNot => Bytecode::DebugBreak1,
      Bytecode::ToBooleanLogicalNot => Bytecode::DebugBreak0,
      Bytecode::LogicalNot => Bytecode::DebugBreak0,
      Bytecode::TypeOf => Bytecode::DebugBreak1,
      Bytecode::DeletePropertyStrict => Bytecode::DebugBreak1,
      Bytecode::DeletePropertySloppy => Bytecode::DebugBreak1,
      Bytecode::GetSuperConstructor => Bytecode::DebugBreak1,
      Bytecode::FindNonDefaultConstructorOrConstruct => Bytecode::DebugBreak3,
      Bytecode::CallAnyReceiver => Bytecode::DebugBreak4,
      Bytecode::CallProperty => Bytecode::DebugBreak4,
      Bytecode::CallProperty0 => Bytecode::DebugBreak3,
      Bytecode::CallProperty1 => Bytecode::DebugBreak4,
      Bytecode::CallProperty2 => Bytecode::DebugBreak6,
      Bytecode::CallUndefinedReceiver => Bytecode::DebugBreak4,
      Bytecode::CallUndefinedReceiver0 => Bytecode::DebugBreak2,
      Bytecode::CallUndefinedReceiver1 => Bytecode::DebugBreak3,
      Bytecode::CallUndefinedReceiver2 => Bytecode::DebugBreak4,
      Bytecode::CallWithSpread => Bytecode::DebugBreak4,
      Bytecode::CallRuntime => Bytecode::DebugBreak4,
      Bytecode::CallRuntimeForPair => Bytecode::DebugBreak6,
      Bytecode::CallJSRuntime => Bytecode::DebugBreak3,
      Bytecode::InvokeIntrinsic => Bytecode::DebugBreak3,
      Bytecode::Construct => Bytecode::DebugBreak4,
      Bytecode::ConstructWithSpread => Bytecode::DebugBreak4,
      Bytecode::ConstructForwardAllArgs => Bytecode::DebugBreak2,
      Bytecode::TestEqual => Bytecode::DebugBreak2,
      Bytecode::TestEqualStrict => Bytecode::DebugBreak2,
      Bytecode::TestLessThan => Bytecode::DebugBreak2,
      Bytecode::TestGreaterThan => Bytecode::DebugBreak2,
      Bytecode::TestLessThanOrEqual => Bytecode::DebugBreak2,
      Bytecode::TestGreaterThanOrEqual => Bytecode::DebugBreak2,
      Bytecode::TestInstanceOf => Bytecode::DebugBreak2,
      Bytecode::TestIn => Bytecode::DebugBreak2,
      Bytecode::ToName => Bytecode::DebugBreak0,
      Bytecode::ToNumber => Bytecode::DebugBreak1,
      Bytecode::ToNumeric => Bytecode::DebugBreak1,
      Bytecode::ToObject => Bytecode::DebugBreak1,
      Bytecode::ToString => Bytecode::DebugBreak0,
      Bytecode::ToBoolean => Bytecode::DebugBreak0,
      Bytecode::CreateRegExpLiteral => Bytecode::DebugBreak4,
      Bytecode::CreateArrayLiteral => Bytecode::DebugBreak3,
      Bytecode::CreateArrayFromIterable => Bytecode::DebugBreak0,
      Bytecode::CreateEmptyArrayLiteral => Bytecode::DebugBreak1,
      Bytecode::CreateObjectLiteral => Bytecode::DebugBreak3,
      Bytecode::CreateEmptyObjectLiteral => Bytecode::DebugBreak0,
      Bytecode::CloneObject => Bytecode::DebugBreak3,
      Bytecode::GetTemplateObject => Bytecode::DebugBreak2,
      Bytecode::CreateClosure => Bytecode::DebugBreak3,
      Bytecode::CreateBlockContext => Bytecode::DebugBreak1,
      Bytecode::CreateCatchContext => Bytecode::DebugBreak2,
      Bytecode::CreateFunctionContext => Bytecode::DebugBreak2,
      Bytecode::CreateFunctionContextWithCells => Bytecode::DebugBreak2,
      Bytecode::CreateEvalContext => Bytecode::DebugBreak2,
      Bytecode::CreateWithContext => Bytecode::DebugBreak2,
      Bytecode::CreateMappedArguments => Bytecode::DebugBreak0,
      Bytecode::CreateUnmappedArguments => Bytecode::DebugBreak0,
      Bytecode::CreateRestParameter => Bytecode::DebugBreak0,
      Bytecode::JumpLoop => Bytecode::DebugBreak3,
      Bytecode::Jump => Bytecode::DebugBreak1,
      Bytecode::JumpConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfNullConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfNotNullConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfUndefinedConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfNotUndefinedConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfUndefinedOrNullConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfTrueConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfFalseConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfJSReceiverConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfForInDoneConstant => Bytecode::DebugBreak3,
      Bytecode::JumpIfToBooleanTrueConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfToBooleanFalseConstant => Bytecode::DebugBreak1,
      Bytecode::JumpIfToBooleanTrue => Bytecode::DebugBreak1,
      Bytecode::JumpIfToBooleanFalse => Bytecode::DebugBreak1,
      Bytecode::JumpIfTrue => Bytecode::DebugBreak1,
      Bytecode::JumpIfFalse => Bytecode::DebugBreak1,
      Bytecode::JumpIfNull => Bytecode::DebugBreak1,
      Bytecode::JumpIfNotNull => Bytecode::DebugBreak1,
      Bytecode::JumpIfUndefined => Bytecode::DebugBreak1,
      Bytecode::JumpIfNotUndefined => Bytecode::DebugBreak1,
      Bytecode::JumpIfUndefinedOrNull => Bytecode::DebugBreak1,
      Bytecode::JumpIfJSReceiver => Bytecode::DebugBreak1,
      Bytecode::JumpIfForInDone => Bytecode::DebugBreak3,
      Bytecode::SwitchOnSmiNoFeedback => Bytecode::DebugBreak3,
      Bytecode::ForInEnumerate => Bytecode::DebugBreak1,
      Bytecode::ForInPrepare => Bytecode::DebugBreak2,
      Bytecode::ForInNext => Bytecode::DebugBreak4,
      Bytecode::ForInStep => Bytecode::DebugBreak1,
      Bytecode::ForOfNext => Bytecode::DebugBreak3,
      Bytecode::SetPendingMessage => Bytecode::DebugBreak0,
      Bytecode::Throw => Bytecode::DebugBreak0,
      Bytecode::ReThrow => Bytecode::DebugBreak0,
      Bytecode::Return => Bytecode::DebugBreak0,
      Bytecode::ThrowReferenceErrorIfTdzHole => Bytecode::DebugBreak1,
      Bytecode::ThrowSuperNotCalledIfTdzHole => Bytecode::DebugBreak0,
      Bytecode::ThrowSuperAlreadyCalledIfNotTdzHole => Bytecode::DebugBreak0,
      Bytecode::ThrowIfNotSuperConstructor => Bytecode::DebugBreak1,
      Bytecode::SwitchOnGeneratorState => Bytecode::DebugBreak3,
      Bytecode::SuspendGenerator => Bytecode::DebugBreak4,
      Bytecode::ResumeGenerator => Bytecode::DebugBreak3,
      Bytecode::GetIterator => Bytecode::DebugBreak3,
      Bytecode::ArrayDestructure => Bytecode::DebugBreak2,
      Bytecode::Debugger => Bytecode::DebugBreak0,
      Bytecode::IncBlockCounter => Bytecode::DebugBreak1,
      Bytecode::Abort => Bytecode::DebugBreak1,
      Bytecode::Star15 => Bytecode::DebugBreak0,
      Bytecode::Star14 => Bytecode::DebugBreak0,
      Bytecode::Star13 => Bytecode::DebugBreak0,
      Bytecode::Star12 => Bytecode::DebugBreak0,
      Bytecode::Star11 => Bytecode::DebugBreak0,
      Bytecode::Star10 => Bytecode::DebugBreak0,
      Bytecode::Star9 => Bytecode::DebugBreak0,
      Bytecode::Star8 => Bytecode::DebugBreak0,
      Bytecode::Star7 => Bytecode::DebugBreak0,
      Bytecode::Star6 => Bytecode::DebugBreak0,
      Bytecode::Star5 => Bytecode::DebugBreak0,
      Bytecode::Star4 => Bytecode::DebugBreak0,
      Bytecode::Star3 => Bytecode::DebugBreak0,
      Bytecode::Star2 => Bytecode::DebugBreak0,
      Bytecode::Star1 => Bytecode::DebugBreak0,
      Bytecode::Star0 => Bytecode::DebugBreak0,
      Bytecode::Illegal => Bytecode::DebugBreak0,
    }
  }

  /// 是否 DebugBreak (原版 IsDebugBreak)
  #[inline]
  #[must_use]
  pub fn is_debug_break(b: Bytecode) -> bool {
    matches!(
      b,
      Bytecode::DebugBreak0
        | Bytecode::DebugBreak1
        | Bytecode::DebugBreak2
        | Bytecode::DebugBreak3
        | Bytecode::DebugBreak4
        | Bytecode::DebugBreak5
        | Bytecode::DebugBreak6
        | Bytecode::DebugBreakWide
        | Bytecode::DebugBreakExtraWide
    )
  }

  /// Star 前瞻内联白名单 (原版 IsStarLookahead; 仅 Single 刻度, kDebugBreak0 为正确性必需)
  #[inline]
  #[must_use]
  pub fn is_star_lookahead(b: Bytecode, scale: OperandScale) -> bool {
    scale == OperandScale::Single
      && matches!(
        b,
        Bytecode::DebugBreak0
          | Bytecode::LdaZero
          | Bytecode::LdaSmi
          | Bytecode::LdaNull
          | Bytecode::LdaTheHole
          | Bytecode::LdaTdzHole
          | Bytecode::LdaConstant
          | Bytecode::LdaUndefined
          | Bytecode::LdaGlobal
          | Bytecode::GetNamedProperty
          | Bytecode::GetKeyedProperty
          | Bytecode::LdaContextSlotNoCell
          | Bytecode::LdaImmutableContextSlot
          | Bytecode::LdaCurrentContextSlotNoCell
          | Bytecode::LdaImmutableCurrentContextSlot
          | Bytecode::Add
          | Bytecode::Sub
          | Bytecode::Mul
          | Bytecode::AddSmi
          | Bytecode::SubSmi
          | Bytecode::Inc
          | Bytecode::Dec
          | Bytecode::TypeOf
          | Bytecode::CallAnyReceiver
          | Bytecode::CallProperty
          | Bytecode::CallProperty0
          | Bytecode::CallProperty1
          | Bytecode::CallProperty2
          | Bytecode::CallUndefinedReceiver
          | Bytecode::CallUndefinedReceiver0
          | Bytecode::CallUndefinedReceiver1
          | Bytecode::CallUndefinedReceiver2
          | Bytecode::Construct
          | Bytecode::ConstructWithSpread
          | Bytecode::CreateObjectLiteral
          | Bytecode::CreateArrayLiteral
          | Bytecode::ThrowReferenceErrorIfTdzHole
          | Bytecode::GetTemplateObject
      )
  }

  /// handler 关键路径是否有调用 (原版 MakesCallAlongCriticalPath)
  #[inline]
  #[must_use]
  pub fn makes_call_along_critical_path(b: Bytecode) -> bool {
    Self::is_call_or_construct(b)
      || Self::is_call_runtime(b)
      || matches!(
        b,
        Bytecode::CreateWithContext
          | Bytecode::CreateBlockContext
          | Bytecode::CreateCatchContext
          | Bytecode::CreateRegExpLiteral
          | Bytecode::GetIterator
          | Bytecode::ArrayDestructure
      )
  }

  /// 寄存器操作数代表的寄存器数 (原版 GetNumberOfRegistersRepresentedBy)
  #[inline]
  #[must_use]
  pub fn number_of_registers_represented_by(t: OperandType) -> Option<u32> {
    match t {
      OperandType::Reg | OperandType::RegOut | OperandType::RegInOut => Some(1),
      OperandType::RegPair | OperandType::RegOutPair => Some(2),
      OperandType::RegOutTriple => Some(3),
      // RegList/RegOutList 数目由后续 RegCount 操作数决定, 原版 UNREACHABLE
      _ => None,
    }
  }

  /// 操作数类型的适配尺寸 (原版 SizeOfOperand; 表见 bytecode_traits::OPERAND_KIND_SIZES)
  #[inline]
  #[must_use]
  pub fn size_of_operand(t: OperandType, scale: OperandScale) -> OperandSize {
    OPERAND_KIND_SIZES[scale.as_index()][t as usize]
  }

  /// 是否寄存器操作数 (原版 IsRegisterOperandType)
  #[inline]
  #[must_use]
  pub fn is_register_operand_type(t: OperandType) -> bool {
    matches!(
      t,
      OperandType::Reg
        | OperandType::RegList
        | OperandType::RegPair
        | OperandType::RegOut
        | OperandType::RegOutList
        | OperandType::RegOutPair
        | OperandType::RegOutTriple
        | OperandType::RegInOut
    )
  }

  /// 是否寄存器输入操作数 (原版 IsRegisterInputOperandType; RegInOut 双向)
  #[inline]
  #[must_use]
  pub fn is_register_input_operand_type(t: OperandType) -> bool {
    matches!(
      t,
      OperandType::Reg | OperandType::RegList | OperandType::RegPair | OperandType::RegInOut
    )
  }

  /// 是否寄存器输出操作数 (原版 IsRegisterOutputOperandType; RegInOut 双向)
  #[inline]
  #[must_use]
  pub fn is_register_output_operand_type(t: OperandType) -> bool {
    matches!(
      t,
      OperandType::RegOut
        | OperandType::RegOutList
        | OperandType::RegOutPair
        | OperandType::RegOutTriple
        | OperandType::RegInOut
    )
  }

  /// 是否寄存器列表操作数 (原版 IsRegisterListOperandType)
  #[inline]
  #[must_use]
  pub fn is_register_list_operand_type(t: OperandType) -> bool {
    matches!(t, OperandType::RegList | OperandType::RegOutList)
  }

  /// 是否运行时 id 操作数 (原版 IsRuntimeIdOperandType)
  #[inline]
  #[must_use]
  pub fn is_runtime_id_operand_type(t: OperandType) -> bool {
    t == OperandType::RuntimeId
  }

  /// 操作数类型是否无符号 (原版 IsUnsignedOperandType)
  #[inline]
  #[must_use]
  pub fn is_unsigned_operand_type(t: OperandType) -> bool {
    t.type_info().is_unsigned()
  }
}

// —— 提取不变式断言 (clang -E 对数) ——
const _: () = {
  assert!(BYTECODE_COUNT == 217);
  assert!(SHORT_STAR_COUNT == 16);
  assert!(Bytecode::Wide as u8 == 0);
  assert!(Bytecode::ExtraWide as u8 == 1);
  assert!(Bytecode::Star15 as u8 == 200);
  assert!(Bytecode::Star0 as u8 == 215);
  assert!(Bytecode::Illegal as u8 == 216);
  assert!(Bytecode::LAST as u8 == Bytecode::Illegal as u8);
  // header 内次序 static_assert (范围谓词前提)
  const _: () = assert!((Bytecode::Ldar as u8) < (Bytecode::LdaImmutableCurrentContextSlot as u8));
  const _: () = assert!((Bytecode::TestReferenceEqual as u8) < (Bytecode::TestTypeOf as u8));
};
