// 由 tool/src/codegen/bailout-reason.h.js 从 v8/src/codegen/bailout-reason.{h,cc} 生成; 修订就地改, 复跑覆盖
// BailoutReason/AbortReason 枚举与静态消息表 (X-macro 机械展开)

// 展开自 ABORT_MESSAGES_LIST / BAILOUT_MESSAGES_LIST (+3 个 TERMINAL 子宏);
// IF_TARGET_ARCH_X64 条目按 x64 目标展开, arm64 目标登记跳过。

/// BailoutReason (原版 enum class BailoutReason : uint8_t, 共 13 + 1 哨兵)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BailoutReason {
  /// kNoReason: "no reason"
  NoReason,
  /// kFunctionTooBig: "Function is too big to be optimized"
  FunctionTooBig,
  /// kTooManyArguments: "Function contains a call with too many arguments"
  TooManyArguments,
  /// kNativeFunctionLiteral: "Native function literal"
  NativeFunctionLiteral,
  /// kNeverOptimize: "Optimization is always disabled"
  NeverOptimize,
  /// kMaglevGraphBuildingFailed: "Maglev optimized graph construction failed"
  MaglevGraphBuildingFailed,
  /// kMaglevCodeGenerationFailed: "Maglev code generation failed"
  MaglevCodeGenerationFailed,
  /// kTurbofanGraphBuildingFailed: "Turbofan optimized graph construction failed"
  TurbofanGraphBuildingFailed,
  /// kTurbofanCodeGenerationFailed: "Turbofan code generation failed"
  TurbofanCodeGenerationFailed,
  /// kBailedOutDueToDependencyChange: "Bailed out due to dependency change"
  BailedOutDueToDependencyChange,
  /// kConcurrentMapDeprecation: "Maps became deprecated during optimization"
  ConcurrentMapDeprecation,
  /// kDetachedNativeContext: "The native context is detached"
  DetachedNativeContext,
  /// kCancelled: "Job got cancelled"
  Cancelled,
  /// kLastErrorMessage (列表外哨兵, 非法值上界)
  LastErrorMessage,
}

impl BailoutReason {
  /// 合法值总数 (不含哨兵)
  pub const COUNT: usize = 13;
  /// 哨兵 = 上界
  pub const LAST_ERROR_MESSAGE: BailoutReason = BailoutReason::LastErrorMessage;

  /// 静态消息文本 (原版 GetBailoutReason; 沙箱内不可信输入需先校验 < LAST)
  #[inline]
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
      Self::NoReason => "no reason",
      Self::FunctionTooBig => "Function is too big to be optimized",
      Self::TooManyArguments => "Function contains a call with too many arguments",
      Self::NativeFunctionLiteral => "Native function literal",
      Self::NeverOptimize => "Optimization is always disabled",
      Self::MaglevGraphBuildingFailed => "Maglev optimized graph construction failed",
      Self::MaglevCodeGenerationFailed => "Maglev code generation failed",
      Self::TurbofanGraphBuildingFailed => "Turbofan optimized graph construction failed",
      Self::TurbofanCodeGenerationFailed => "Turbofan code generation failed",
      Self::BailedOutDueToDependencyChange => "Bailed out due to dependency change",
      Self::ConcurrentMapDeprecation => "Maps became deprecated during optimization",
      Self::DetachedNativeContext => "The native context is detached",
      Self::Cancelled => "Job got cancelled",
      Self::LastErrorMessage => "<invalid bailout reason>",
    }
  }
}

/// AbortReason (原版 enum class AbortReason : uint8_t, 共 90 条, 末位 kLastReason)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AbortReason {
  /// kNoReason: "no reason"
  NoReason,
  /// k32BitValueInRegisterIsNotZeroExtended: "32 bit value in register is not zero-extended"
  32BitValueInRegisterIsNotZeroExtended,
  /// kSignBitOfSmiIsNotZero: "Sign bit of 31 bit smi register is not zero"
  SignBitOfSmiIsNotZero,
  /// kAPICallReturnedInvalidObject: "API call returned invalid object"
  APICallReturnedInvalidObject,
  /// kAccumulatorClobbered: "Accumulator clobbered"
  AccumulatorClobbered,
  /// kAllocatingNonEmptyPackedArray: "Allocating non-empty packed array"
  AllocatingNonEmptyPackedArray,
  /// kAllocationIsNotDoubleAligned: "Allocation is not double aligned"
  AllocationIsNotDoubleAligned,
  /// kExpectedOptimizationSentinel: "Expected optimized code cell or optimization sentinel"
  ExpectedOptimizationSentinel,
  /// kExpectedOsrCode: "Expected OSR code"
  ExpectedOsrCode,
  /// kExpectedUndefinedOrCell: "Expected undefined or cell in register"
  ExpectedUndefinedOrCell,
  /// kExpectedFeedbackCell: "Expected feedback cell"
  ExpectedFeedbackCell,
  /// kExpectedFeedbackVector: "Expected feedback vector"
  ExpectedFeedbackVector,
  /// kExpectedBaselineData: "Expected baseline data"
  ExpectedBaselineData,
  /// kFloat64IsNotAInt32: "Float64 cannot be converted to Int32 without loss of precision"
  Float64IsNotAInt32,
  /// kFunctionDataShouldBeBytecodeArrayOnInterpreterEntry: "The function_data field should be a BytecodeArray on interpreter entry"
  FunctionDataShouldBeBytecodeArrayOnInterpreterEntry,
  /// kInputStringTooLong: "Input string too long"
  InputStringTooLong,
  /// kInputDoesNotFitSmi: "Input number is too large to fit in a Smi"
  InputDoesNotFitSmi,
  /// kInvalidBytecode: "Invalid bytecode"
  InvalidBytecode,
  /// kInvalidBytecodeAdvance: "Cannot advance current bytecode, "
  InvalidBytecodeAdvance,
  /// kInvalidDeoptimizedCode: "Invoked code which is deoptimized"
  InvalidDeoptimizedCode,
  /// kInvalidHandleScopeLevel: "Invalid HandleScope level"
  InvalidHandleScopeLevel,
  /// kInvalidJumpTableIndex: "Invalid jump table index"
  InvalidJumpTableIndex,
  /// kInvalidParametersAndRegistersInGenerator: "invalid parameters and registers in generator"
  InvalidParametersAndRegistersInGenerator,
  /// kMaglevGraphBuildingFailed: "Maglev optimized graph construction failed"
  MaglevGraphBuildingFailed,
  /// kMissingBytecodeArray: "Missing bytecode array from function"
  MissingBytecodeArray,
  /// kObjectNotTagged: "The object is not tagged"
  ObjectNotTagged,
  /// kObjectTagged: "The object is tagged"
  ObjectTagged,
  /// kOffsetOutOfRange: "Offset out of range"
  OffsetOutOfRange,
  /// kOperandIsASmi: "Operand is a smi"
  OperandIsASmi,
  /// kOperandIsASmiAndNotABoundFunction: "Operand is a smi and not a bound function"
  OperandIsASmiAndNotABoundFunction,
  /// kOperandIsASmiAndNotAConstructor: "Operand is a smi and not a constructor"
  OperandIsASmiAndNotAConstructor,
  /// kOperandIsASmiAndNotAFunction: "Operand is a smi and not a function"
  OperandIsASmiAndNotAFunction,
  /// kOperandIsASmiAndNotAGeneratorObject: "Operand is a smi and not a generator object"
  OperandIsASmiAndNotAGeneratorObject,
  /// kOperandIsCleared: "Operand is cleared"
  OperandIsCleared,
  /// kOperandIsNotABoundFunction: "Operand is not a bound function"
  OperandIsNotABoundFunction,
  /// kOperandIsNotAConstructor: "Operand is not a constructor"
  OperandIsNotAConstructor,
  /// kOperandIsNotAFixedArray: "Operand is not a fixed array"
  OperandIsNotAFixedArray,
  /// kOperandIsNotAFunction: "Operand is not a function"
  OperandIsNotAFunction,
  /// kOperandIsNotACallableFunction: "Operand is not a callable function"
  OperandIsNotACallableFunction,
  /// kOperandIsNotAGeneratorObject: "Operand is not a generator object"
  OperandIsNotAGeneratorObject,
  /// kOperandIsNotACode: "Operand is not a Code object"
  OperandIsNotACode,
  /// kOperandIsNotAMap: "Operand is not a Map object"
  OperandIsNotAMap,
  /// kOperandIsNotASmi: "Operand is not a smi"
  OperandIsNotASmi,
  /// kPromiseAlreadySettled: "Promise already settled"
  PromiseAlreadySettled,
  /// kReceivedInvalidReturnAddress: "Received invalid return address"
  ReceivedInvalidReturnAddress,
  /// kRegisterDidNotMatchExpectedRoot: "Register did not match expected root"
  RegisterDidNotMatchExpectedRoot,
  /// kReturnAddressNotFoundInFrame: "Return address not found in frame"
  ReturnAddressNotFoundInFrame,
  /// kShouldNotDirectlyEnterOsrFunction: "Should not directly enter OSR-compiled function"
  ShouldNotDirectlyEnterOsrFunction,
  /// kStackAccessBelowStackPointer: "Stack access below stack pointer"
  StackAccessBelowStackPointer,
  /// kOsrUnexpectedStackSize: "Unexpected stack size on OSR entry"
  OsrUnexpectedStackSize,
  /// kStackFrameTypesMustMatch: "Stack frame types must match"
  StackFrameTypesMustMatch,
  /// kUint32IsNotAInt32: "Uint32 cannot be converted to Int32 without loss of precision"
  Uint32IsNotAInt32,
  /// kUnalignedCellInWriteBarrier: "Unaligned cell in write barrier"
  UnalignedCellInWriteBarrier,
  /// kUnexpectedAdditionalPopValue: "Unexpected additional pop value"
  UnexpectedAdditionalPopValue,
  /// kUnexpectedBigIntTerminationSentinel: "Unexpected BigInt TerminationRequested sentinel"
  UnexpectedBigIntTerminationSentinel,
  /// kUnexpectedElementsKindInArrayConstructor: "Unexpected ElementsKind in array constructor"
  UnexpectedElementsKindInArrayConstructor,
  /// kUnexpectedFPCRMode: "Unexpected FPCR mode."
  UnexpectedFPCRMode,
  /// kUnexpectedFunctionIDForInvokeIntrinsic: "Unexpected runtime function id for the InvokeIntrinsic bytecode"
  UnexpectedFunctionIDForInvokeIntrinsic,
  /// kUnexpectedInitialMapForArrayFunction: "Unexpected initial map for Array function"
  UnexpectedInitialMapForArrayFunction,
  /// kUnexpectedLevelAfterReturnFromApiCall: "Unexpected level after return from api call"
  UnexpectedLevelAfterReturnFromApiCall,
  /// kUnexpectedNegativeValue: "Unexpected negative value"
  UnexpectedNegativeValue,
  /// kUnexpectedReturnFromFrameDropper: "Unexpectedly returned from dropping frames"
  UnexpectedReturnFromFrameDropper,
  /// kUnexpectedReturnFromThrow: "Unexpectedly returned from a throw"
  UnexpectedReturnFromThrow,
  /// kUnexpectedReturnFromWasmTrap: "Should not return after throwing a wasm trap"
  UnexpectedReturnFromWasmTrap,
  /// kUnexpectedStackPointer: "The stack pointer is not the expected value"
  UnexpectedStackPointer,
  /// kUnexpectedValue: "Unexpected value"
  UnexpectedValue,
  /// kUninhabitableType: "Uninhabitable type"
  UninhabitableType,
  /// kUnreachable: "Unreachable code"
  Unreachable,
  /// kUnsupportedDeopt: "Lazy deopt after a fast API call with return value is unsupported"
  UnsupportedDeopt,
  /// kUnsupportedModuleOperation: "Unsupported module operation"
  UnsupportedModuleOperation,
  /// kUnsupportedNonPrimitiveCompare: "Unsupported non-primitive compare"
  UnsupportedNonPrimitiveCompare,
  /// kWrongAddressOrValuePassedToRecordWrite: "Wrong address or value passed to RecordWrite"
  WrongAddressOrValuePassedToRecordWrite,
  /// kWrongArgumentCountForInvokeIntrinsic: "Wrong number of arguments for intrinsic"
  WrongArgumentCountForInvokeIntrinsic,
  /// kWrongFunctionCodeStart: "Wrong value in code start register passed"
  WrongFunctionCodeStart,
  /// kWrongFunctionContext: "Wrong context passed to function"
  WrongFunctionContext,
  /// kWrongFunctionDispatchHandle: "Wrong value in dispatch handle register passed"
  WrongFunctionDispatchHandle,
  /// kInvalidReceiver: "Expected JS object or primitive object"
  InvalidReceiver,
  /// kUnexpectedInstanceType: "Unexpected instance type encountered"
  UnexpectedInstanceType,
  /// kTurboshaftTypeAssertionFailed: "A type assertion failed in Turboshaft-generated code"
  TurboshaftTypeAssertionFailed,
  /// kTurboshaftLoadEliminationError: "Turboshaft's load elimination wrongly eliminated a Load"
  TurboshaftLoadEliminationError,
  /// kMetadataAreaStartDoesNotMatch: "The metadata doesn't belong to the chunk"
  MetadataAreaStartDoesNotMatch,
  /// kExternalPointerTagMismatch: "Tag mismatch during external pointer access"
  ExternalPointerTagMismatch,
  /// kIndirectPointerTagMismatch: "Tag mismatch during indirect pointer access"
  IndirectPointerTagMismatch,
  /// kJSSignatureMismatch: "Signature mismatch during JS function call"
  JSSignatureMismatch,
  /// kWasmSignatureMismatch: "Signature mismatch during Wasm indirect call"
  WasmSignatureMismatch,
  /// kFastCallFallbackInvalid: "Fast call fallback returned incorrect type"
  FastCallFallbackInvalid,
  /// k32BitValueInRegisterIsNotSignExtended: "32 bit value in register is not sign-extended"
  32BitValueInRegisterIsNotSignExtended,
  /// kUnexpectedSandboxMode: "The sandboxing mode is not as expected"
  UnexpectedSandboxMode,
  /// kOSREmptyCheckFailed (IF_TARGET_ARCH_X64: 仅 x64 目标展开; arm64 目标无此条): "OSR target slot should be empty"
  OSREmptyCheckFailed,
  /// kLastReason: ""
  LastReason,
  /// kLastErrorMessage (列表外哨兵, 非法值上界)
  LastErrorMessage,
}

impl AbortReason {
  /// 合法值总数 (不含哨兵)
  pub const COUNT: usize = 90;
  /// 哨兵 = 上界
  pub const LAST_ERROR_MESSAGE: AbortReason = AbortReason::LastErrorMessage;

  /// 静态消息文本 (原版 GetAbortReason)
  #[inline]
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
      Self::NoReason => "no reason",
      Self::32BitValueInRegisterIsNotZeroExtended => "32 bit value in register is not zero-extended",
      Self::SignBitOfSmiIsNotZero => "Sign bit of 31 bit smi register is not zero",
      Self::APICallReturnedInvalidObject => "API call returned invalid object",
      Self::AccumulatorClobbered => "Accumulator clobbered",
      Self::AllocatingNonEmptyPackedArray => "Allocating non-empty packed array",
      Self::AllocationIsNotDoubleAligned => "Allocation is not double aligned",
      Self::ExpectedOptimizationSentinel => "Expected optimized code cell or optimization sentinel",
      Self::ExpectedOsrCode => "Expected OSR code",
      Self::ExpectedUndefinedOrCell => "Expected undefined or cell in register",
      Self::ExpectedFeedbackCell => "Expected feedback cell",
      Self::ExpectedFeedbackVector => "Expected feedback vector",
      Self::ExpectedBaselineData => "Expected baseline data",
      Self::Float64IsNotAInt32 => "Float64 cannot be converted to Int32 without loss of precision",
      Self::FunctionDataShouldBeBytecodeArrayOnInterpreterEntry => "The function_data field should be a BytecodeArray on interpreter entry",
      Self::InputStringTooLong => "Input string too long",
      Self::InputDoesNotFitSmi => "Input number is too large to fit in a Smi",
      Self::InvalidBytecode => "Invalid bytecode",
      Self::InvalidBytecodeAdvance => "Cannot advance current bytecode, ",
      Self::InvalidDeoptimizedCode => "Invoked code which is deoptimized",
      Self::InvalidHandleScopeLevel => "Invalid HandleScope level",
      Self::InvalidJumpTableIndex => "Invalid jump table index",
      Self::InvalidParametersAndRegistersInGenerator => "invalid parameters and registers in generator",
      Self::MaglevGraphBuildingFailed => "Maglev optimized graph construction failed",
      Self::MissingBytecodeArray => "Missing bytecode array from function",
      Self::ObjectNotTagged => "The object is not tagged",
      Self::ObjectTagged => "The object is tagged",
      Self::OffsetOutOfRange => "Offset out of range",
      Self::OperandIsASmi => "Operand is a smi",
      Self::OperandIsASmiAndNotABoundFunction => "Operand is a smi and not a bound function",
      Self::OperandIsASmiAndNotAConstructor => "Operand is a smi and not a constructor",
      Self::OperandIsASmiAndNotAFunction => "Operand is a smi and not a function",
      Self::OperandIsASmiAndNotAGeneratorObject => "Operand is a smi and not a generator object",
      Self::OperandIsCleared => "Operand is cleared",
      Self::OperandIsNotABoundFunction => "Operand is not a bound function",
      Self::OperandIsNotAConstructor => "Operand is not a constructor",
      Self::OperandIsNotAFixedArray => "Operand is not a fixed array",
      Self::OperandIsNotAFunction => "Operand is not a function",
      Self::OperandIsNotACallableFunction => "Operand is not a callable function",
      Self::OperandIsNotAGeneratorObject => "Operand is not a generator object",
      Self::OperandIsNotACode => "Operand is not a Code object",
      Self::OperandIsNotAMap => "Operand is not a Map object",
      Self::OperandIsNotASmi => "Operand is not a smi",
      Self::PromiseAlreadySettled => "Promise already settled",
      Self::ReceivedInvalidReturnAddress => "Received invalid return address",
      Self::RegisterDidNotMatchExpectedRoot => "Register did not match expected root",
      Self::ReturnAddressNotFoundInFrame => "Return address not found in frame",
      Self::ShouldNotDirectlyEnterOsrFunction => "Should not directly enter OSR-compiled function",
      Self::StackAccessBelowStackPointer => "Stack access below stack pointer",
      Self::OsrUnexpectedStackSize => "Unexpected stack size on OSR entry",
      Self::StackFrameTypesMustMatch => "Stack frame types must match",
      Self::Uint32IsNotAInt32 => "Uint32 cannot be converted to Int32 without loss of precision",
      Self::UnalignedCellInWriteBarrier => "Unaligned cell in write barrier",
      Self::UnexpectedAdditionalPopValue => "Unexpected additional pop value",
      Self::UnexpectedBigIntTerminationSentinel => "Unexpected BigInt TerminationRequested sentinel",
      Self::UnexpectedElementsKindInArrayConstructor => "Unexpected ElementsKind in array constructor",
      Self::UnexpectedFPCRMode => "Unexpected FPCR mode.",
      Self::UnexpectedFunctionIDForInvokeIntrinsic => "Unexpected runtime function id for the InvokeIntrinsic bytecode",
      Self::UnexpectedInitialMapForArrayFunction => "Unexpected initial map for Array function",
      Self::UnexpectedLevelAfterReturnFromApiCall => "Unexpected level after return from api call",
      Self::UnexpectedNegativeValue => "Unexpected negative value",
      Self::UnexpectedReturnFromFrameDropper => "Unexpectedly returned from dropping frames",
      Self::UnexpectedReturnFromThrow => "Unexpectedly returned from a throw",
      Self::UnexpectedReturnFromWasmTrap => "Should not return after throwing a wasm trap",
      Self::UnexpectedStackPointer => "The stack pointer is not the expected value",
      Self::UnexpectedValue => "Unexpected value",
      Self::UninhabitableType => "Uninhabitable type",
      Self::Unreachable => "Unreachable code",
      Self::UnsupportedDeopt => "Lazy deopt after a fast API call with return value is unsupported",
      Self::UnsupportedModuleOperation => "Unsupported module operation",
      Self::UnsupportedNonPrimitiveCompare => "Unsupported non-primitive compare",
      Self::WrongAddressOrValuePassedToRecordWrite => "Wrong address or value passed to RecordWrite",
      Self::WrongArgumentCountForInvokeIntrinsic => "Wrong number of arguments for intrinsic",
      Self::WrongFunctionCodeStart => "Wrong value in code start register passed",
      Self::WrongFunctionContext => "Wrong context passed to function",
      Self::WrongFunctionDispatchHandle => "Wrong value in dispatch handle register passed",
      Self::InvalidReceiver => "Expected JS object or primitive object",
      Self::UnexpectedInstanceType => "Unexpected instance type encountered",
      Self::TurboshaftTypeAssertionFailed => "A type assertion failed in Turboshaft-generated code",
      Self::TurboshaftLoadEliminationError => "Turboshaft's load elimination wrongly eliminated a Load",
      Self::MetadataAreaStartDoesNotMatch => "The metadata doesn't belong to the chunk",
      Self::ExternalPointerTagMismatch => "Tag mismatch during external pointer access",
      Self::IndirectPointerTagMismatch => "Tag mismatch during indirect pointer access",
      Self::JSSignatureMismatch => "Signature mismatch during JS function call",
      Self::WasmSignatureMismatch => "Signature mismatch during Wasm indirect call",
      Self::FastCallFallbackInvalid => "Fast call fallback returned incorrect type",
      Self::32BitValueInRegisterIsNotSignExtended => "32 bit value in register is not sign-extended",
      Self::UnexpectedSandboxMode => "The sandboxing mode is not as expected",
      Self::OSREmptyCheckFailed => "OSR target slot should be empty",
      Self::LastReason => "",
      Self::LastErrorMessage => "<invalid abort reason>",
    }
  }

  /// 原版 IsValidAbortReason: id 落在 [kNoReason, kLastErrorMessage)
  #[inline]
  #[must_use]
  pub const fn is_valid_id(reason_id: i32) -> bool {
    reason_id >= AbortReason::NoReason as i32 && reason_id < AbortReason::LastErrorMessage as i32
  }

  /// 从数值反查 (越界返回 None)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    if (val as usize) < Self::COUNT + 1 { Some(unsafe { std::mem::transmute::<u8, Self>(val) }) } else { None }
  }
}

/// 终止型 Bailout 原因 (原版 IsTerminalBailoutReason: TERMINAL_BAILOUT_MESSAGES_LIST)
pub const TERMINAL_BAILOUT_REASONS: &[BailoutReason] = &[
  BailoutReason::FunctionTooBig,
  BailoutReason::TooManyArguments,
  BailoutReason::NativeFunctionLiteral,
  BailoutReason::NeverOptimize,
];

/// Maglev 终止型 (原版 IsTerminalBailoutReasonForMaglev: TERMINAL_BAILOUT + TERMINAL_MAGLEV)
pub const TERMINAL_BAILOUT_REASONS_FOR_MAGLEV: &[BailoutReason] = &[
  BailoutReason::FunctionTooBig,
  BailoutReason::TooManyArguments,
  BailoutReason::NativeFunctionLiteral,
  BailoutReason::NeverOptimize,
  BailoutReason::MaglevGraphBuildingFailed,
  BailoutReason::MaglevCodeGenerationFailed,
];

/// Turbofan 终止型 (原版 IsTerminalBailoutReasonForTurbofan: 同 Maglev 集合)
pub const TERMINAL_BAILOUT_REASONS_FOR_TURBOFAN: &[BailoutReason] = &[
  BailoutReason::FunctionTooBig,
  BailoutReason::TooManyArguments,
  BailoutReason::NativeFunctionLiteral,
  BailoutReason::NeverOptimize,
  BailoutReason::MaglevGraphBuildingFailed,
  BailoutReason::MaglevCodeGenerationFailed,
  BailoutReason::TurbofanGraphBuildingFailed,
  BailoutReason::TurbofanCodeGenerationFailed,
];

/// 原版 IsTerminalBailoutReason
#[inline]
#[must_use]
pub fn is_terminal_bailout_reason(reason: BailoutReason) -> bool {
  TERMINAL_BAILOUT_REASONS.contains(&reason)
}

/// 原版 IsTerminalBailoutReasonForMaglev
#[inline]
#[must_use]
pub fn is_terminal_bailout_reason_for_maglev(reason: BailoutReason) -> bool {
  TERMINAL_BAILOUT_REASONS_FOR_MAGLEV.contains(&reason)
}

/// 原版 IsTerminalBailoutReasonForTurbofan
#[inline]
#[must_use]
pub fn is_terminal_bailout_reason_for_turbofan(reason: BailoutReason) -> bool {
  TERMINAL_BAILOUT_REASONS_FOR_TURBOFAN.contains(&reason)
}

/// 原版 GetBailoutReason (自由函数形态)
#[inline]
#[must_use]
pub const fn get_bailout_reason(reason: BailoutReason) -> &'static str {
  reason.message()
}

/// 原版 GetAbortReason (自由函数形态)
#[inline]
#[must_use]
pub const fn get_abort_reason(reason: AbortReason) -> &'static str {
  reason.message()
}

// —— 提取不变式断言 (对数守卫) ——
const _: () = {
  assert!(BailoutReason::COUNT == 13);
  assert!(AbortReason::COUNT == 90);
  assert!(AbortReason::COUNT == 90);
  assert!(BailoutReason::NoReason as u8 == 0);
  assert!(AbortReason::NoReason as u8 == 0);
  assert!((AbortReason::LastErrorMessage as u8) == AbortReason::COUNT as u8);
  assert!((BailoutReason::LastErrorMessage as u8) == BailoutReason::COUNT as u8);
  assert!(AbortReason::LastReason.message().is_empty());
};
