// 由 tool/src/deoptimizer/deoptimize-reason.cc.js 从 v8/src/deoptimizer/deoptimize-reason.{h,cc} + deoptimizer.{h,cc} + frame-description.h 生成; 修订就地改, 复跑覆盖
// 反优化原因表与主体骨架

use core::fmt;

/// 反优化原因 (原版 enum class DeoptimizeReason, DEOPTIMIZE_REASON_LIST 1:1 展开, 68 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeoptimizeReason {
  /// "out of bounds"
  OutOfBounds = 0,
  /// "array buffer was detached or immutable"
  ArrayBufferWasDetached = 1,
  /// "the array length changed"
  ArrayLengthChanged = 2,
  /// "BigInt too big"
  BigIntTooBig = 3,
  /// "failed to grow elements store"
  CouldNotGrowElements = 4,
  /// "copy-on-write array's elements changed"
  CowArrayElementsChanged = 5,
  /// "%_DeoptimizeNow"
  DeoptimizeNow = 6,
  /// "division by zero"
  DivisionByZero = 7,
  /// "length is greater than the maximum for fast elements array"
  GreaterThanMaxFastElementArray = 8,
  /// "hole"
  Hole = 9,
  /// "hole or undefined"
  HoleOrUndefined = 10,
  /// "instance migration failed"
  InstanceMigrationFailed = 11,
  /// "Insufficient type feedback for array literal"
  InsufficientTypeFeedbackForArrayLiteral = 12,
  /// "Insufficient type feedback for binary operation"
  InsufficientTypeFeedbackForBinaryOperation = 13,
  /// "Insufficient type feedback for call"
  InsufficientTypeFeedbackForCall = 14,
  /// "Insufficient type feedback for compare operation"
  InsufficientTypeFeedbackForCompareOperation = 15,
  /// "Insufficient type feedback for construct"
  InsufficientTypeFeedbackForConstruct = 16,
  /// "Insufficient type feedback for for-in"
  InsufficientTypeFeedbackForForIn = 17,
  /// "Insufficient type feedback for generic global access"
  InsufficientTypeFeedbackForGenericGlobalAccess = 18,
  /// "Insufficient type feedback for generic keyed access"
  InsufficientTypeFeedbackForGenericKeyedAccess = 19,
  /// "Insufficient type feedback for generic named access"
  InsufficientTypeFeedbackForGenericNamedAccess = 20,
  /// "Insufficient type feedback for instanceof"
  InsufficientTypeFeedbackForInstanceOf = 21,
  /// "Insufficient type feedback for object literal"
  InsufficientTypeFeedbackForObjectLiteral = 22,
  /// "Insufficient type feedback for typeof"
  InsufficientTypeFeedbackForTypeOf = 23,
  /// "Insufficient type feedback for unary operation"
  InsufficientTypeFeedbackForUnaryOperation = 24,
  /// "unexpected name in keyed access"
  KeyedAccessChanged = 25,
  /// "loop speculation failed"
  LoopSpeculationFailed = 26,
  /// "lost precision"
  LostPrecision = 27,
  /// "lost precision or NaN"
  LostPrecisionOrNaN = 28,
  /// "minus zero"
  MinusZero = 29,
  /// "no initial element"
  NoInitialElement = 30,
  /// "not a BigInt"
  NotABigInt = 31,
  /// "not a BigInt64"
  NotABigInt64 = 32,
  /// "not a heap number"
  NotAHeapNumber = 33,
  /// "not a JavaScript object"
  NotAJavaScriptObject = 34,
  /// "not a JavaScript object, Null or Undefined"
  NotAJavaScriptObjectOrNullOrUndefined = 35,
  /// "not a Number"
  NotANumber = 36,
  /// "not a Number or Boolean"
  NotANumberOrBoolean = 37,
  /// "not a Number or Undefined"
  NotANumberOrUndefined = 38,
  /// "not a Number or Oddball"
  NotANumberOrOddball = 39,
  /// "not a Smi"
  NotASmi = 40,
  /// "not a String"
  NotAString = 41,
  /// "not a String or a string wrapper"
  NotAStringOrStringWrapper = 42,
  /// "not a String or oddball"
  NotAStringOrOddball = 43,
  /// "not a Symbol"
  NotASymbol = 44,
  /// "not AdditiveSafeInteger"
  NotAdditiveSafeInteger = 45,
  /// "not an array index"
  NotAnArrayIndex = 46,
  /// "not a detectable receiver"
  NotDetectableReceiver = 47,
  /// "not int32"
  NotInt32 = 48,
  /// "not unsigned int32"
  NotUint32 = 49,
  /// "exit from OSR'd inner loop"
  OSREarlyExit = 50,
  /// "overflow"
  Overflow = 51,
  /// "prepare for on stack replacement (OSR)"
  PrepareForOnStackReplacement = 52,
  /// "Smi"
  Smi = 53,
  /// "Storing to a constant field"
  StoreToConstant = 54,
  /// "Result string larger than String::kMaxLength"
  StringTooLarge = 55,
  /// "unexpected context extension"
  UnexpectedContextExtension = 56,
  /// "(unknown)"
  Unknown = 57,
  /// "value mismatch"
  ValueMismatch = 58,
  /// "wrong call target"
  WrongCallTarget = 59,
  /// "wrong call target constructor"
  WrongConstructor = 60,
  /// "wrong enum indices"
  WrongEnumIndices = 61,
  /// "wrong feedback cell"
  WrongFeedbackCell = 62,
  /// "wrong instance type"
  WrongInstanceType = 63,
  /// "wrong map"
  WrongMap = 64,
  /// "map changed during operation"
  WrongMapDynamic = 65,
  /// "wrong name"
  WrongName = 66,
  /// "wrong value"
  WrongValue = 67,
}

impl DeoptimizeReason {
  /// 总数 (对数守卫)
  pub const COUNT: usize = 68;

  /// OSR 反优化不失效代码 (原版 IsDeoptimizationWithoutCodeInvalidation, deoptimize-reason.h:152):
  /// Maglev OSR 进 TurboFan 先反优化恢复未优化帧布局, 代码对象假设未违反, 保留缓存供将来重入
  #[must_use]
  pub const fn is_deoptimization_without_code_invalidation(self) -> bool {
    matches!(
      self,
      Self::PrepareForOnStackReplacement | Self::OSREarlyExit
    )
  }

  /// 恒保留反优化原因 (原版 AlwaysPreserveDeoptReason, deoptimize-reason.h:163):
  /// DEOPTIMIZE_IN_BUILTIN_REASON_LIST (仅 OutOfBounds, 内建反优化不与特定代码绑定)
  /// 或 OSR 族原因 (失效判定依赖原因本身)
  #[must_use]
  pub const fn always_preserve_deopt_reason(self) -> bool {
    matches!(self, Self::OutOfBounds) || self.is_deoptimization_without_code_invalidation()
  }

  /// 消息串 (原版 DeoptimizeReasonToString → kDeoptimizeReasonStrings)
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
      Self::OutOfBounds => "out of bounds",
      Self::ArrayBufferWasDetached => "array buffer was detached or immutable",
      Self::ArrayLengthChanged => "the array length changed",
      Self::BigIntTooBig => "BigInt too big",
      Self::CouldNotGrowElements => "failed to grow elements store",
      Self::CowArrayElementsChanged => "copy-on-write array's elements changed",
      Self::DeoptimizeNow => "%_DeoptimizeNow",
      Self::DivisionByZero => "division by zero",
      Self::GreaterThanMaxFastElementArray => {
        "length is greater than the maximum for fast elements array"
      }
      Self::Hole => "hole",
      Self::HoleOrUndefined => "hole or undefined",
      Self::InstanceMigrationFailed => "instance migration failed",
      Self::InsufficientTypeFeedbackForArrayLiteral => {
        "Insufficient type feedback for array literal"
      }
      Self::InsufficientTypeFeedbackForBinaryOperation => {
        "Insufficient type feedback for binary operation"
      }
      Self::InsufficientTypeFeedbackForCall => "Insufficient type feedback for call",
      Self::InsufficientTypeFeedbackForCompareOperation => {
        "Insufficient type feedback for compare operation"
      }
      Self::InsufficientTypeFeedbackForConstruct => "Insufficient type feedback for construct",
      Self::InsufficientTypeFeedbackForForIn => "Insufficient type feedback for for-in",
      Self::InsufficientTypeFeedbackForGenericGlobalAccess => {
        "Insufficient type feedback for generic global access"
      }
      Self::InsufficientTypeFeedbackForGenericKeyedAccess => {
        "Insufficient type feedback for generic keyed access"
      }
      Self::InsufficientTypeFeedbackForGenericNamedAccess => {
        "Insufficient type feedback for generic named access"
      }
      Self::InsufficientTypeFeedbackForInstanceOf => "Insufficient type feedback for instanceof",
      Self::InsufficientTypeFeedbackForObjectLiteral => {
        "Insufficient type feedback for object literal"
      }
      Self::InsufficientTypeFeedbackForTypeOf => "Insufficient type feedback for typeof",
      Self::InsufficientTypeFeedbackForUnaryOperation => {
        "Insufficient type feedback for unary operation"
      }
      Self::KeyedAccessChanged => "unexpected name in keyed access",
      Self::LoopSpeculationFailed => "loop speculation failed",
      Self::LostPrecision => "lost precision",
      Self::LostPrecisionOrNaN => "lost precision or NaN",
      Self::MinusZero => "minus zero",
      Self::NoInitialElement => "no initial element",
      Self::NotABigInt => "not a BigInt",
      Self::NotABigInt64 => "not a BigInt64",
      Self::NotAHeapNumber => "not a heap number",
      Self::NotAJavaScriptObject => "not a JavaScript object",
      Self::NotAJavaScriptObjectOrNullOrUndefined => "not a JavaScript object, Null or Undefined",
      Self::NotANumber => "not a Number",
      Self::NotANumberOrBoolean => "not a Number or Boolean",
      Self::NotANumberOrUndefined => "not a Number or Undefined",
      Self::NotANumberOrOddball => "not a Number or Oddball",
      Self::NotASmi => "not a Smi",
      Self::NotAString => "not a String",
      Self::NotAStringOrStringWrapper => "not a String or a string wrapper",
      Self::NotAStringOrOddball => "not a String or oddball",
      Self::NotASymbol => "not a Symbol",
      Self::NotAdditiveSafeInteger => "not AdditiveSafeInteger",
      Self::NotAnArrayIndex => "not an array index",
      Self::NotDetectableReceiver => "not a detectable receiver",
      Self::NotInt32 => "not int32",
      Self::NotUint32 => "not unsigned int32",
      Self::OSREarlyExit => "exit from OSR'd inner loop",
      Self::Overflow => "overflow",
      Self::PrepareForOnStackReplacement => "prepare for on stack replacement (OSR)",
      Self::Smi => "Smi",
      Self::StoreToConstant => "Storing to a constant field",
      Self::StringTooLarge => "Result string larger than String::kMaxLength",
      Self::UnexpectedContextExtension => "unexpected context extension",
      Self::Unknown => "(unknown)",
      Self::ValueMismatch => "value mismatch",
      Self::WrongCallTarget => "wrong call target",
      Self::WrongConstructor => "wrong call target constructor",
      Self::WrongEnumIndices => "wrong enum indices",
      Self::WrongFeedbackCell => "wrong feedback cell",
      Self::WrongInstanceType => "wrong instance type",
      Self::WrongMap => "wrong map",
      Self::WrongMapDynamic => "map changed during operation",
      Self::WrongName => "wrong name",
      Self::WrongValue => "wrong value",
    }
  }

  /// 枚举名 (原版 operator<< 输出 #Name)
  #[must_use]
  pub const fn name(self) -> &'static str {
    match self {
      Self::OutOfBounds => "OutOfBounds",
      Self::ArrayBufferWasDetached => "ArrayBufferWasDetached",
      Self::ArrayLengthChanged => "ArrayLengthChanged",
      Self::BigIntTooBig => "BigIntTooBig",
      Self::CouldNotGrowElements => "CouldNotGrowElements",
      Self::CowArrayElementsChanged => "CowArrayElementsChanged",
      Self::DeoptimizeNow => "DeoptimizeNow",
      Self::DivisionByZero => "DivisionByZero",
      Self::GreaterThanMaxFastElementArray => "GreaterThanMaxFastElementArray",
      Self::Hole => "Hole",
      Self::HoleOrUndefined => "HoleOrUndefined",
      Self::InstanceMigrationFailed => "InstanceMigrationFailed",
      Self::InsufficientTypeFeedbackForArrayLiteral => "InsufficientTypeFeedbackForArrayLiteral",
      Self::InsufficientTypeFeedbackForBinaryOperation => {
        "InsufficientTypeFeedbackForBinaryOperation"
      }
      Self::InsufficientTypeFeedbackForCall => "InsufficientTypeFeedbackForCall",
      Self::InsufficientTypeFeedbackForCompareOperation => {
        "InsufficientTypeFeedbackForCompareOperation"
      }
      Self::InsufficientTypeFeedbackForConstruct => "InsufficientTypeFeedbackForConstruct",
      Self::InsufficientTypeFeedbackForForIn => "InsufficientTypeFeedbackForForIn",
      Self::InsufficientTypeFeedbackForGenericGlobalAccess => {
        "InsufficientTypeFeedbackForGenericGlobalAccess"
      }
      Self::InsufficientTypeFeedbackForGenericKeyedAccess => {
        "InsufficientTypeFeedbackForGenericKeyedAccess"
      }
      Self::InsufficientTypeFeedbackForGenericNamedAccess => {
        "InsufficientTypeFeedbackForGenericNamedAccess"
      }
      Self::InsufficientTypeFeedbackForInstanceOf => "InsufficientTypeFeedbackForInstanceOf",
      Self::InsufficientTypeFeedbackForObjectLiteral => "InsufficientTypeFeedbackForObjectLiteral",
      Self::InsufficientTypeFeedbackForTypeOf => "InsufficientTypeFeedbackForTypeOf",
      Self::InsufficientTypeFeedbackForUnaryOperation => {
        "InsufficientTypeFeedbackForUnaryOperation"
      }
      Self::KeyedAccessChanged => "KeyedAccessChanged",
      Self::LoopSpeculationFailed => "LoopSpeculationFailed",
      Self::LostPrecision => "LostPrecision",
      Self::LostPrecisionOrNaN => "LostPrecisionOrNaN",
      Self::MinusZero => "MinusZero",
      Self::NoInitialElement => "NoInitialElement",
      Self::NotABigInt => "NotABigInt",
      Self::NotABigInt64 => "NotABigInt64",
      Self::NotAHeapNumber => "NotAHeapNumber",
      Self::NotAJavaScriptObject => "NotAJavaScriptObject",
      Self::NotAJavaScriptObjectOrNullOrUndefined => "NotAJavaScriptObjectOrNullOrUndefined",
      Self::NotANumber => "NotANumber",
      Self::NotANumberOrBoolean => "NotANumberOrBoolean",
      Self::NotANumberOrUndefined => "NotANumberOrUndefined",
      Self::NotANumberOrOddball => "NotANumberOrOddball",
      Self::NotASmi => "NotASmi",
      Self::NotAString => "NotAString",
      Self::NotAStringOrStringWrapper => "NotAStringOrStringWrapper",
      Self::NotAStringOrOddball => "NotAStringOrOddball",
      Self::NotASymbol => "NotASymbol",
      Self::NotAdditiveSafeInteger => "NotAdditiveSafeInteger",
      Self::NotAnArrayIndex => "NotAnArrayIndex",
      Self::NotDetectableReceiver => "NotDetectableReceiver",
      Self::NotInt32 => "NotInt32",
      Self::NotUint32 => "NotUint32",
      Self::OSREarlyExit => "OSREarlyExit",
      Self::Overflow => "Overflow",
      Self::PrepareForOnStackReplacement => "PrepareForOnStackReplacement",
      Self::Smi => "Smi",
      Self::StoreToConstant => "StoreToConstant",
      Self::StringTooLarge => "StringTooLarge",
      Self::UnexpectedContextExtension => "UnexpectedContextExtension",
      Self::Unknown => "Unknown",
      Self::ValueMismatch => "ValueMismatch",
      Self::WrongCallTarget => "WrongCallTarget",
      Self::WrongConstructor => "WrongConstructor",
      Self::WrongEnumIndices => "WrongEnumIndices",
      Self::WrongFeedbackCell => "WrongFeedbackCell",
      Self::WrongInstanceType => "WrongInstanceType",
      Self::WrongMap => "WrongMap",
      Self::WrongMapDynamic => "WrongMapDynamic",
      Self::WrongName => "WrongName",
      Self::WrongValue => "WrongValue",
    }
  }
}

/// 惰性反优化原因 (原版 enum class LazyDeoptimizeReason, 17 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LazyDeoptimizeReason {
  /// "dependent map was deprecated"
  MapDeprecated = 0,
  /// "dependent prototype chain changed"
  PrototypeChange = 1,
  /// "dependent property cell changed"
  PropertyCellChange = 2,
  /// "dependent field type constness changed"
  FieldTypeConstChange = 3,
  /// "dependent field type changed"
  FieldTypeChange = 4,
  /// "dependent field representation changed"
  FieldRepresentationChange = 5,
  /// "dependent initial map changed"
  InitialMapChange = 6,
  /// "dependent allocation site tenuring changed"
  AllocationSiteTenuringChange = 7,
  /// "dependent allocation site transition changed"
  AllocationSiteTransitionChange = 8,
  /// "dependent context cell state changed"
  ContextCellChange = 9,
  /// "dependent empty context extension changed"
  EmptyContextExtensionChange = 10,
  /// "embedded weak objects cleared"
  WeakObjects = 11,
  /// "JS debugger attached"
  Debugger = 12,
  /// "for testing"
  Testing = 13,
  /// "exception with omitted catch handler"
  ExceptionCaught = 14,
  /// "marked due to eager deopt"
  EagerDeopt = 15,
  /// "value in stack frame was materialized"
  FrameValueMaterialized = 16,
}

impl LazyDeoptimizeReason {
  pub const COUNT: usize = 17;

  /// 变体名 (枚举名本身, 测试与诊断用)
  #[must_use]
  pub const fn name(self) -> &'static str {
    match self {
      Self::MapDeprecated => "MapDeprecated",
      Self::PrototypeChange => "PrototypeChange",
      Self::PropertyCellChange => "PropertyCellChange",
      Self::FieldTypeConstChange => "FieldTypeConstChange",
      Self::FieldTypeChange => "FieldTypeChange",
      Self::FieldRepresentationChange => "FieldRepresentationChange",
      Self::InitialMapChange => "InitialMapChange",
      Self::AllocationSiteTenuringChange => "AllocationSiteTenuringChange",
      Self::AllocationSiteTransitionChange => "AllocationSiteTransitionChange",
      Self::ContextCellChange => "ContextCellChange",
      Self::EmptyContextExtensionChange => "EmptyContextExtensionChange",
      Self::WeakObjects => "WeakObjects",
      Self::Debugger => "Debugger",
      Self::Testing => "Testing",
      Self::ExceptionCaught => "ExceptionCaught",
      Self::EagerDeopt => "EagerDeopt",
      Self::FrameValueMaterialized => "FrameValueMaterialized",
    }
  }

  /// 消息串 (原版重载 DeoptimizeReasonToString)
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
      Self::MapDeprecated => "dependent map was deprecated",
      Self::PrototypeChange => "dependent prototype chain changed",
      Self::PropertyCellChange => "dependent property cell changed",
      Self::FieldTypeConstChange => "dependent field type constness changed",
      Self::FieldTypeChange => "dependent field type changed",
      Self::FieldRepresentationChange => "dependent field representation changed",
      Self::InitialMapChange => "dependent initial map changed",
      Self::AllocationSiteTenuringChange => "dependent allocation site tenuring changed",
      Self::AllocationSiteTransitionChange => "dependent allocation site transition changed",
      Self::ContextCellChange => "dependent context cell state changed",
      Self::EmptyContextExtensionChange => "dependent empty context extension changed",
      Self::WeakObjects => "embedded weak objects cleared",
      Self::Debugger => "JS debugger attached",
      Self::Testing => "for testing",
      Self::ExceptionCaught => "exception with omitted catch handler",
      Self::EagerDeopt => "marked due to eager deopt",
      Self::FrameValueMaterialized => "value in stack frame was materialized",
    }
  }
}

/// 反优化种类 (原版 globals.h enum class DeoptimizeKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeoptimizeKind {
  Eager = 0,
  LazyAfterFastCall = 1,
  Lazy = 2,
}

impl DeoptimizeKind {
  /// 种类名 (原版 globals.h DEOPTIMIZE_KINDS 表, 带 k 前缀)
  #[must_use]
  pub const fn name(self) -> &'static str {
    DEOPTIMIZE_KINDS[self as usize]
  }

  /// trace 消息串 (原版 deoptimizer.cc:625-630 Deoptimizer::KindToMessage)
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
      Self::Eager => "deopt-eager",
      Self::LazyAfterFastCall => "deopt-lazy-after-fastcall",
      Self::Lazy => "deopt-lazy",
    }
  }
}

impl fmt::Display for DeoptimizeKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.name())
  }
}

impl fmt::Display for LazyDeoptimizeReason {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.name())
  }
}

impl fmt::Display for DeoptimizeReason {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.name())
  }
}

/// 反优化种类字符串表 (定义于 src/common/globals.h, 此处跨文件登记, 3 项)
pub const DEOPTIMIZE_KINDS: [&str; 3] = ["kEager", "kLazyAfterFastCall", "kLazy"];

/// Deoptimizer 方法面 (原版 deoptimizer.h, 39 项, 顺序登记)
pub const DEOPTIMIZER_METHODS: [&str; 39] = [
  "GetOutermostOuterLoopWithCodeKind",
  "function",
  "compiled_code",
  "output_count",
  "VirtualMaterializeAndPrint",
  "DeoptimizeFunction",
  "DeoptimizeMarkedCode",
  "DeoptimizeAllOptimizedCodeWithFunction",
  "EnsureValidReturnAddress",
  "MaterializeHeapObjects",
  "ProcessDeoptReason",
  "ComputeOutputFrames",
  "input_offset",
  "output_count_offset",
  "output_offset",
  "caller_frame_top_offset",
  "TraceMarkForDeoptimization",
  "TraceEvictFromOptimizedCodeCache",
  "PatchToJump",
  "ZapCode",
  "DeoptExitIsInsideOsrLoop",
  "QueueValueForMaterialization",
  "QueueFeedbackVectorForMaterialization",
  "DeleteFrameDescriptions",
  "DoComputeOutputFrames",
  "DoComputeOutputFramesWasmImpl",
  "GetWasmStackSlotsCounts",
  "DoComputeUnoptimizedFrame",
  "DoComputeInlinedExtraArguments",
  "DoComputeConstructCreateStubFrame",
  "DoComputeConstructInvokeStubFrame",
  "DoComputeBuiltinContinuation",
  "tracing_enabled",
  "verbose_tracing_enabled",
  "TraceDeoptBegin",
  "TraceDeoptEnd",
  "TraceFoundActivation",
  "TraceDeoptAll",
  "is_restart_frame",
];

/// FrameDescription / TranslatedFrame 结构锚点 (原版 frame-description.h)
pub const FRAME_DESCRIPTION_ANCHORS: [&str; 2] = ["FrameDescription", "RegisterValues"];

// deoptimizer.cc 定义头清单 (对数基准, 45 项):
// Deoptimizer* Deoptimizer::New(Address raw_function, DeoptimizeKind kind, Address from, int fp_to_sp_delta, Isolate* isolate)
// Deoptimizer* Deoptimizer::Grab(Isolate* isolate)
// size_t Deoptimizer::DeleteForWasm(Isolate* isolate)
// DeoptimizedFrameInfo* Deoptimizer::DebuggerInspectableFrame( JavaScriptFrame* frame, int jsframe_index, Isolate* isolate)
// void Deoptimizer::DeoptimizeMarkedCode(Isolate* isolate)
// void Deoptimizer::DeoptimizeAll(Isolate* isolate)
// void Deoptimizer::DeoptimizeFunction(Tagged<JSFunction> function, LazyDeoptimizeReason reason, Tagged<Code> code)
// void Deoptimizer::DeoptimizeAllOptimizedCodeWithFunction( Isolate* isolate, DirectHandle<SharedFunctionInfo> function)
// Address Deoptimizer::EnsureValidReturnAddress(Isolate* isolate, Address address)
// void Deoptimizer::ComputeOutputFrames(Deoptimizer* deoptimizer)
// const char* Deoptimizer::MessageFor(DeoptimizeKind kind)
// Deoptimizer::Deoptimizer(Isolate* isolate, Tagged<JSFunction> function, DeoptimizeKind kind, Address from, int fp_to_sp_delta) : isolate_(isolate), function_(function), deopt_exit_index_(kFixedExitSizeMarker), deopt_kind_(kind), from_(from), fp_to_sp_delta_(fp_to_sp_delta), deoptimizing_throw_(false), catch_handler_data_(-1), catch_handler_pc_offset_(-1), restart_frame_index_(-1), input_(nullptr), output_count_(0), output_(nullptr), caller_frame_top_(0), caller_fp_(0), caller_pc_(0), caller_constant_pool_(0), actual_argument_count_(0), stack_fp_(0), trace_scope_(v8_flags.trace_deopt ? new CodeTracer::Scope(isolate->GetCodeTracer()) : nullptr)
// DirectHandle<JSFunction> Deoptimizer::function() const
// DirectHandle<Code> Deoptimizer::compiled_code() const
// Deoptimizer::~Deoptimizer()
// void Deoptimizer::DeleteFrameDescriptions()
// Builtin Deoptimizer::GetDeoptimizationEntry(DeoptimizeKind kind)
// void Deoptimizer::TraceDeoptBegin(int optimization_id, BytecodeOffset bytecode_offset)
// void Deoptimizer::TraceDeoptEnd(double deopt_duration)
// void Deoptimizer::TraceMarkForDeoptimization(Isolate* isolate, Tagged<Code> code, LazyDeoptimizeReason reason)
// void Deoptimizer::TraceEvictFromOptimizedCodeCache( Isolate* isolate, Tagged<SharedFunctionInfo> sfi, const char* reason)
// void Deoptimizer::TraceFoundActivation(Isolate* isolate, Tagged<JSFunction> function)
// void Deoptimizer::TraceDeoptAll(Isolate* isolate)
// FrameDescription* Deoptimizer::DoComputeWasmLiftoffFrame( TranslatedFrame& frame, wasm::NativeModule* native_module, Tagged<WasmTrustedInstanceData> wasm_trusted_instance, int frame_index, std::stack<intptr_t>& shadow_stack)
// void Deoptimizer::DoComputeOutputFramesWasmImpl()
// void Deoptimizer::GetWasmStackSlotsCounts(const wasm::FunctionSig* sig, int* parameter_stack_slots, int* return_stack_slots)
// void Deoptimizer::VirtualMaterializeAndPrint()
// void Deoptimizer::DoComputeOutputFrames()
// bool Deoptimizer::DeoptExitIsInsideOsrLoop(Isolate* isolate, Tagged<JSFunction> function, BytecodeOffset deopt_exit_offset, BytecodeOffset osr_offset, CodeKind code_kind)
// bool Deoptimizer::GetOutermostOuterLoopWithCodeKind( Isolate* isolate, Tagged<JSFunction> function, BytecodeOffset osr_offset, CodeKind outer_loop_code_kind, BytecodeOffset* outer_loop_osr_offset)
// void Deoptimizer::DoComputeUnoptimizedFrame(TranslatedFrame* translated_frame, int frame_index, bool goto_catch_handler)
// void Deoptimizer::DoComputeInlinedExtraArguments( TranslatedFrame* translated_frame, int frame_index)
// void Deoptimizer::DoComputeConstructCreateStubFrame( TranslatedFrame* translated_frame, int frame_index)
// void Deoptimizer::DoComputeConstructInvokeStubFrame( TranslatedFrame* translated_frame, int frame_index)
// Builtin Deoptimizer::TrampolineForBuiltinContinuation( BuiltinContinuationMode mode, bool must_handle_result)
// TranslatedValue Deoptimizer::TranslatedValueForWasmReturnKind( std::optional<wasm::ValueKind> wasm_call_return_kind)
// void Deoptimizer::DoComputeBuiltinContinuation( TranslatedFrame* translated_frame, int frame_index, BuiltinContinuationMode mode)
// void Deoptimizer::MaterializeHeapObjects()
// void Deoptimizer::ProcessDeoptReason(DeoptimizeReason reason)
// void Deoptimizer::QueueValueForMaterialization( Address output_address, Tagged<Object> obj, const TranslatedFrame::iterator& iterator)
// void Deoptimizer::QueueFeedbackVectorForMaterialization( Address output_address, const TranslatedFrame::iterator& iterator)
// unsigned Deoptimizer::ComputeInputFrameAboveFpFixedSize() const
// unsigned Deoptimizer::ComputeInputFrameSize() const
// unsigned Deoptimizer::ComputeIncomingArgumentSize(Tagged<Code> code)
// Deoptimizer::DeoptInfo Deoptimizer::ComputeDeoptInfo(Tagged<Code> code, Address pc)
