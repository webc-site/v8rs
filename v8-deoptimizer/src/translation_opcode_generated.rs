// 由 tool/src/deoptimizer/translation-opcode.h.js 从 v8/src/deoptimizer/translation-opcode.h 生成; 修订就地改, 复跑覆盖
// 翻译字节码操作码表

use core::fmt;

/// 翻译字节码操作码 (原版 enum class TranslationOpcode, TRANSLATION_OPCODE_LIST 1:1 展开;
/// 段序: JS 帧 → 帧 → 数据项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TranslationOpcode {
  /// 操作数 6
  InterpretedFrameWithReturn = 0,
  /// 操作数 4
  InterpretedFrameWithoutReturn = 1,
  /// 操作数 3
  JavascriptBuiltinContinuationFrame = 2,
  /// 操作数 3
  JavascriptBuiltinContinuationWithCatchFrame = 3,
  /// 操作数 2
  ConstructCreateStubFrame = 4,
  /// 操作数 1
  ConstructInvokeStubFrame = 5,
  /// 操作数 3
  BuiltinContinuationFrame = 6,
  /// 操作数 4 (仅 WebAssembly)
  JsToWasmBuiltinContinuationFrame = 7,
  /// 操作数 3 (仅 WebAssembly)
  WasmInlinedIntoJsFrame = 8,
  /// 操作数 3 (仅 WebAssembly)
  LiftoffFrame = 9,
  /// 操作数 3
  InlinedExtraArguments = 10,
  /// 操作数 1
  ArgumentsElements = 11,
  /// 操作数 0
  ArgumentsLength = 12,
  /// 操作数 0
  RestLength = 13,
  /// 操作数 3
  BeginWithoutFeedback = 14,
  /// 操作数 3
  BeginWithFeedback = 15,
  /// 操作数 1
  BoolRegister = 16,
  /// 操作数 1
  BoolStackSlot = 17,
  /// 操作数 1
  CapturedObject = 18,
  /// 操作数 0
  StringConcat = 19,
  /// 操作数 2
  DoubleLiteral = 20,
  /// 操作数 1
  DoubleRegister = 21,
  /// 操作数 1
  DoubleStackSlot = 22,
  /// 操作数 1
  Simd128StackSlot = 23,
  /// 操作数 2
  HoleyDoubleLiteral = 24,
  /// 操作数 1
  HoleyDoubleRegister = 25,
  /// 操作数 1
  HoleyDoubleStackSlot = 26,
  /// 操作数 1
  Simd128Register = 27,
  /// 操作数 1
  DuplicatedObject = 28,
  /// 操作数 1
  FloatRegister = 29,
  /// 操作数 1
  FloatStackSlot = 30,
  /// 操作数 1
  Int32Register = 31,
  /// 操作数 1
  Int32StackSlot = 32,
  /// 操作数 1
  Int64Register = 33,
  /// 操作数 1
  Int64StackSlot = 34,
  /// 操作数 1
  SignedBigint64Register = 35,
  /// 操作数 1
  SignedBigint64StackSlot = 36,
  /// 操作数 1
  UnsignedBigint64Register = 37,
  /// 操作数 1
  UnsignedBigint64StackSlot = 38,
  /// 操作数 0
  OptimizedOut = 39,
  /// 操作数 1
  Literal = 40,
  /// 操作数 1
  Register = 41,
  /// 操作数 1
  TaggedStackSlot = 42,
  /// 操作数 1
  Uint32Register = 43,
  /// 操作数 1
  Uint32StackSlot = 44,
  /// 操作数 2
  UpdateFeedback = 45,
  /// 操作数 1
  MatchPreviousTranslation = 46,
}

impl TranslationOpcode {
  /// 操作码名 (原版 TRANSLATION_OPCODE_LIST 的 #name 展开, SCREAMING_SNAKE)
  #[must_use]
  pub const fn name(self) -> &'static str {
    match self {
      Self::InterpretedFrameWithReturn => "INTERPRETED_FRAME_WITH_RETURN",
      Self::InterpretedFrameWithoutReturn => "INTERPRETED_FRAME_WITHOUT_RETURN",
      Self::JavascriptBuiltinContinuationFrame => "JAVASCRIPT_BUILTIN_CONTINUATION_FRAME",
      Self::JavascriptBuiltinContinuationWithCatchFrame => {
        "JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH_FRAME"
      }
      Self::ConstructCreateStubFrame => "CONSTRUCT_CREATE_STUB_FRAME",
      Self::ConstructInvokeStubFrame => "CONSTRUCT_INVOKE_STUB_FRAME",
      Self::BuiltinContinuationFrame => "BUILTIN_CONTINUATION_FRAME",
      Self::JsToWasmBuiltinContinuationFrame => "JS_TO_WASM_BUILTIN_CONTINUATION_FRAME",
      Self::WasmInlinedIntoJsFrame => "WASM_INLINED_INTO_JS_FRAME",
      Self::LiftoffFrame => "LIFTOFF_FRAME",
      Self::InlinedExtraArguments => "INLINED_EXTRA_ARGUMENTS",
      Self::ArgumentsElements => "ARGUMENTS_ELEMENTS",
      Self::ArgumentsLength => "ARGUMENTS_LENGTH",
      Self::RestLength => "REST_LENGTH",
      Self::BeginWithoutFeedback => "BEGIN_WITHOUT_FEEDBACK",
      Self::BeginWithFeedback => "BEGIN_WITH_FEEDBACK",
      Self::BoolRegister => "BOOL_REGISTER",
      Self::BoolStackSlot => "BOOL_STACK_SLOT",
      Self::CapturedObject => "CAPTURED_OBJECT",
      Self::StringConcat => "STRING_CONCAT",
      Self::DoubleLiteral => "DOUBLE_LITERAL",
      Self::DoubleRegister => "DOUBLE_REGISTER",
      Self::DoubleStackSlot => "DOUBLE_STACK_SLOT",
      Self::Simd128StackSlot => "SIMD_1_2_8_STACK_SLOT",
      Self::HoleyDoubleLiteral => "HOLEY_DOUBLE_LITERAL",
      Self::HoleyDoubleRegister => "HOLEY_DOUBLE_REGISTER",
      Self::HoleyDoubleStackSlot => "HOLEY_DOUBLE_STACK_SLOT",
      Self::Simd128Register => "SIMD_1_2_8_REGISTER",
      Self::DuplicatedObject => "DUPLICATED_OBJECT",
      Self::FloatRegister => "FLOAT_REGISTER",
      Self::FloatStackSlot => "FLOAT_STACK_SLOT",
      Self::Int32Register => "INT_3_2_REGISTER",
      Self::Int32StackSlot => "INT_3_2_STACK_SLOT",
      Self::Int64Register => "INT_6_4_REGISTER",
      Self::Int64StackSlot => "INT_6_4_STACK_SLOT",
      Self::SignedBigint64Register => "SIGNED_BIGINT_6_4_REGISTER",
      Self::SignedBigint64StackSlot => "SIGNED_BIGINT_6_4_STACK_SLOT",
      Self::UnsignedBigint64Register => "UNSIGNED_BIGINT_6_4_REGISTER",
      Self::UnsignedBigint64StackSlot => "UNSIGNED_BIGINT_6_4_STACK_SLOT",
      Self::OptimizedOut => "OPTIMIZED_OUT",
      Self::Literal => "LITERAL",
      Self::Register => "REGISTER",
      Self::TaggedStackSlot => "TAGGED_STACK_SLOT",
      Self::Uint32Register => "UINT_3_2_REGISTER",
      Self::Uint32StackSlot => "UINT_3_2_STACK_SLOT",
      Self::UpdateFeedback => "UPDATE_FEEDBACK",
      Self::MatchPreviousTranslation => "MATCH_PREVIOUS_TRANSLATION",
    }
  }
  /// JS 帧操作码数 (原版 kNumTranslationJsFrameOpcodes)
  pub const NUM_JS_FRAME: usize = 4;
  /// 帧操作码总数 = JS 帧 + 通用帧 (原版 kNumTranslationFrameOpcodes)
  pub const NUM_FRAME: usize = 11;
  /// 全部操作码数 (原版 kNumTranslationOpcodes)
  pub const NUM_ALL: usize = 47;
  /// 单操作码操作数上限 (原版 kMaxTranslationOperandCount)
  pub const MAX_OPERAND_COUNT: usize = 6;

  /// 操作数计数表 (原版 TranslationOpcodeOperandCount)
  #[must_use]
  pub const fn operand_count(self) -> usize {
    match self {
      Self::InterpretedFrameWithReturn => 6,
      Self::InterpretedFrameWithoutReturn => 4,
      Self::JavascriptBuiltinContinuationFrame => 3,
      Self::JavascriptBuiltinContinuationWithCatchFrame => 3,
      Self::ConstructCreateStubFrame => 2,
      Self::ConstructInvokeStubFrame => 1,
      Self::BuiltinContinuationFrame => 3,
      Self::JsToWasmBuiltinContinuationFrame => 4,
      Self::WasmInlinedIntoJsFrame => 3,
      Self::LiftoffFrame => 3,
      Self::InlinedExtraArguments => 3,
      Self::ArgumentsElements => 1,
      Self::ArgumentsLength => 0,
      Self::RestLength => 0,
      Self::BeginWithoutFeedback => 3,
      Self::BeginWithFeedback => 3,
      Self::BoolRegister => 1,
      Self::BoolStackSlot => 1,
      Self::CapturedObject => 1,
      Self::StringConcat => 0,
      Self::DoubleLiteral => 2,
      Self::DoubleRegister => 1,
      Self::DoubleStackSlot => 1,
      Self::Simd128StackSlot => 1,
      Self::HoleyDoubleLiteral => 2,
      Self::HoleyDoubleRegister => 1,
      Self::HoleyDoubleStackSlot => 1,
      Self::Simd128Register => 1,
      Self::DuplicatedObject => 1,
      Self::FloatRegister => 1,
      Self::FloatStackSlot => 1,
      Self::Int32Register => 1,
      Self::Int32StackSlot => 1,
      Self::Int64Register => 1,
      Self::Int64StackSlot => 1,
      Self::SignedBigint64Register => 1,
      Self::SignedBigint64StackSlot => 1,
      Self::UnsignedBigint64Register => 1,
      Self::UnsignedBigint64StackSlot => 1,
      Self::OptimizedOut => 0,
      Self::Literal => 1,
      Self::Register => 1,
      Self::TaggedStackSlot => 1,
      Self::Uint32Register => 1,
      Self::Uint32StackSlot => 1,
      Self::UpdateFeedback => 2,
      Self::MatchPreviousTranslation => 1,
    }
  }

  /// 是否帧操作码 (原版 IsTranslationFrameOpcode: < NUM_FRAME)
  #[must_use]
  pub const fn is_frame_opcode(self) -> bool {
    (self as usize) < Self::NUM_FRAME
  }

  /// 是否 JS 帧操作码 (原版 IsTranslationJsFrameOpcode: < NUM_JS_FRAME;
  /// 边界断言锚点 CONSTRUCT_CREATE_STUB_FRAME)
  #[must_use]
  pub const fn is_js_frame_opcode(self) -> bool {
    (self as usize) < Self::NUM_JS_FRAME
  }

  /// 是否解释器帧 (原版 IsTranslationInterpreterFrameOpcode)
  #[must_use]
  pub const fn is_interpreter_frame_opcode(self) -> bool {
    matches!(
      self,
      Self::InterpretedFrameWithReturn | Self::InterpretedFrameWithoutReturn
    )
  }

  /// 是否 BEGIN (原版 TranslationOpcodeIsBegin)
  #[must_use]
  pub const fn is_begin(self) -> bool {
    matches!(self, Self::BeginWithFeedback | Self::BeginWithoutFeedback)
  }
}

/// WebAssembly 专属操作码 (原版 IF_WASM 展开, 3 项; x64/arm64 默认启用 WASM)
pub const WASM_OPCODES: [&str; 3] = [
  "JS_TO_WASM_BUILTIN_CONTINUATION_FRAME",
  "WASM_INLINED_INTO_JS_FRAME",
  "LIFTOFF_FRAME",
];

impl fmt::Display for TranslationOpcode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.name())
  }
}
