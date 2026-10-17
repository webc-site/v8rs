use aok::{OK, Void};
use log::info;
use v8_deoptimizer::{
  DEOPTIMIZE_KINDS, DEOPTIMIZER_METHODS, DeoptimizeKind, DeoptimizeReason, DeoptimizedFrameInfo,
  LazyDeoptimizeReason, MaterializedObjectStore, TRANSLATED_STATE_METHODS,
  TRANSLATION_BUILDER_METHODS, TranslationOpcode, WASM_OPCODES, arch_dispatch, arm64,
  translated_state_skel::{deoptimized_frame_info_skel, init, store_materialized_objects},
  x64,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_deoptimize_reasons_and_kinds() -> Void {
  info!("Testing DeoptimizeReason and DeoptimizeKind");

  // DeoptimizeReason 68 个
  assert_eq!(DeoptimizeReason::COUNT, 68);
  assert_eq!(DeoptimizeReason::OutOfBounds as u8, 0);
  assert_eq!(DeoptimizeReason::OutOfBounds.name(), "OutOfBounds");
  assert_eq!(DeoptimizeReason::OutOfBounds.message(), "out of bounds");

  assert_eq!(
    DeoptimizeReason::DivisionByZero.message(),
    "division by zero"
  );
  assert_eq!(DeoptimizeReason::Hole.name(), "Hole");
  assert_eq!(DeoptimizeReason::WrongValue as u8, 67);
  assert_eq!(DeoptimizeReason::WrongValue.name(), "WrongValue");
  assert_eq!(DeoptimizeReason::WrongValue.message(), "wrong value");

  // OSR 与保留反优化原因判定
  assert!(
    DeoptimizeReason::PrepareForOnStackReplacement.is_deoptimization_without_code_invalidation()
  );
  assert!(DeoptimizeReason::OSREarlyExit.is_deoptimization_without_code_invalidation());
  assert!(!DeoptimizeReason::OutOfBounds.is_deoptimization_without_code_invalidation());
  assert!(!DeoptimizeReason::DivisionByZero.is_deoptimization_without_code_invalidation());

  assert!(DeoptimizeReason::OutOfBounds.always_preserve_deopt_reason());
  assert!(DeoptimizeReason::PrepareForOnStackReplacement.always_preserve_deopt_reason());
  assert!(DeoptimizeReason::OSREarlyExit.always_preserve_deopt_reason());
  assert!(!DeoptimizeReason::Hole.always_preserve_deopt_reason());

  // LazyDeoptimizeReason 17 个
  assert_eq!(LazyDeoptimizeReason::COUNT, 17);
  assert_eq!(LazyDeoptimizeReason::MapDeprecated as u8, 0);
  assert_eq!(LazyDeoptimizeReason::MapDeprecated.name(), "MapDeprecated");
  assert_eq!(
    LazyDeoptimizeReason::MapDeprecated.message(),
    "dependent map was deprecated"
  );
  assert_eq!(LazyDeoptimizeReason::FrameValueMaterialized as u8, 16);
  assert_eq!(
    LazyDeoptimizeReason::FrameValueMaterialized.name(),
    "FrameValueMaterialized"
  );
  assert_eq!(
    LazyDeoptimizeReason::FrameValueMaterialized.message(),
    "value in stack frame was materialized"
  );

  // DeoptimizeKind
  assert_eq!(DeoptimizeKind::Eager as u8, 0);
  assert_eq!(DeoptimizeKind::LazyAfterFastCall as u8, 1);
  assert_eq!(DeoptimizeKind::Lazy as u8, 2);
  assert_eq!(DeoptimizeKind::Eager.name(), "kEager");
  assert_eq!(DeoptimizeKind::Eager.message(), "deopt-eager");
  assert_eq!(
    DeoptimizeKind::LazyAfterFastCall.name(),
    "kLazyAfterFastCall"
  );
  assert_eq!(
    DeoptimizeKind::LazyAfterFastCall.message(),
    "deopt-lazy-after-fastcall"
  );
  assert_eq!(DeoptimizeKind::Lazy.name(), "kLazy");
  assert_eq!(DeoptimizeKind::Lazy.message(), "deopt-lazy");

  // Display 格式化
  assert_eq!(format!("{}", DeoptimizeReason::OutOfBounds), "OutOfBounds");
  assert_eq!(
    format!("{}", LazyDeoptimizeReason::MapDeprecated),
    "MapDeprecated"
  );
  assert_eq!(format!("{}", DeoptimizeKind::Eager), "kEager");

  assert_eq!(DEOPTIMIZE_KINDS.len(), 3);
  assert_eq!(DEOPTIMIZE_KINDS[0], "kEager");
  assert_eq!(DEOPTIMIZE_KINDS[1], "kLazyAfterFastCall");
  assert_eq!(DEOPTIMIZE_KINDS[2], "kLazy");

  OK
}

#[test]
fn test_translation_opcodes() -> Void {
  info!("Testing TranslationOpcode");

  assert_eq!(TranslationOpcode::NUM_JS_FRAME, 4);
  assert_eq!(TranslationOpcode::NUM_FRAME, 11);
  assert_eq!(TranslationOpcode::NUM_ALL, 47);
  assert_eq!(TranslationOpcode::MAX_OPERAND_COUNT, 6);

  // JS 帧
  let frame_with_ret = TranslationOpcode::InterpretedFrameWithReturn;
  assert!(frame_with_ret.is_frame_opcode());
  assert!(frame_with_ret.is_js_frame_opcode());
  assert!(frame_with_ret.is_interpreter_frame_opcode());
  assert_eq!(frame_with_ret.operand_count(), 6);
  assert_eq!(frame_with_ret.name(), "INTERPRETED_FRAME_WITH_RETURN");
  assert_eq!(format!("{frame_with_ret}"), "INTERPRETED_FRAME_WITH_RETURN");

  let frame_without_ret = TranslationOpcode::InterpretedFrameWithoutReturn;
  assert!(frame_without_ret.is_frame_opcode());
  assert!(frame_without_ret.is_js_frame_opcode());
  assert!(frame_without_ret.is_interpreter_frame_opcode());
  assert_eq!(frame_without_ret.operand_count(), 4);

  let js_builtin_cont = TranslationOpcode::JavascriptBuiltinContinuationFrame;
  assert!(js_builtin_cont.is_frame_opcode());
  assert!(js_builtin_cont.is_js_frame_opcode());
  assert!(!js_builtin_cont.is_interpreter_frame_opcode());
  assert_eq!(js_builtin_cont.operand_count(), 3);

  let js_builtin_cont_catch = TranslationOpcode::JavascriptBuiltinContinuationWithCatchFrame;
  assert!(js_builtin_cont_catch.is_frame_opcode());
  assert!(js_builtin_cont_catch.is_js_frame_opcode());
  assert_eq!(js_builtin_cont_catch.operand_count(), 3);

  // 非 JS 帧 (4..11)
  let construct_create = TranslationOpcode::ConstructCreateStubFrame;
  assert!(construct_create.is_frame_opcode());
  assert!(!construct_create.is_js_frame_opcode());
  assert_eq!(construct_create.operand_count(), 2);

  let inlined_extra = TranslationOpcode::InlinedExtraArguments;
  assert_eq!(inlined_extra as usize, 10);
  assert!(inlined_extra.is_frame_opcode());
  assert!(!inlined_extra.is_js_frame_opcode());
  assert_eq!(inlined_extra.operand_count(), 3);

  // 首个非帧数据项 (11)
  let args_elements = TranslationOpcode::ArgumentsElements;
  assert_eq!(args_elements as usize, 11);
  assert!(!args_elements.is_frame_opcode());
  assert_eq!(args_elements.operand_count(), 1);

  // 数据项
  let bool_reg = TranslationOpcode::BoolRegister;
  assert!(!bool_reg.is_frame_opcode());
  assert!(!bool_reg.is_js_frame_opcode());
  assert_eq!(bool_reg.operand_count(), 1);

  // 末尾数据项 (46)
  let match_prev = TranslationOpcode::MatchPreviousTranslation;
  assert_eq!(match_prev as usize, 46);
  assert!(!match_prev.is_frame_opcode());
  assert_eq!(match_prev.name(), "MATCH_PREVIOUS_TRANSLATION");

  // Begin 判定
  assert!(TranslationOpcode::BeginWithFeedback.is_begin());
  assert!(TranslationOpcode::BeginWithoutFeedback.is_begin());
  assert!(!bool_reg.is_begin());

  // Wasm 操作码
  assert_eq!(WASM_OPCODES.len(), 3);
  assert_eq!(WASM_OPCODES[0], "JS_TO_WASM_BUILTIN_CONTINUATION_FRAME");
  assert_eq!(WASM_OPCODES[1], "WASM_INLINED_INTO_JS_FRAME");
  assert_eq!(WASM_OPCODES[2], "LIFTOFF_FRAME");

  OK
}

#[test]
fn test_frame_info_and_architectures() -> Void {
  info!("Testing DeoptimizedFrameInfo and Arch Dispatch");

  let info = DeoptimizedFrameInfo::new(3, 12);
  assert_eq!(info.parameter_count, 3);
  assert_eq!(info.stack_height, 12);
  let default_info = DeoptimizedFrameInfo::default();
  assert_eq!(default_info.parameter_count, 0);
  assert_eq!(default_info.stack_height, 0);

  let _store = MaterializedObjectStore::new();

  // 架构常量
  assert_eq!(arch_dispatch::TARGET_ARCHES.len(), 2);
  assert_eq!(arch_dispatch::SKIPPED_ARCHES.len(), 7);
  assert_eq!(arch_dispatch::FN_COUNTS.len(), 9);

  assert_eq!(arch_dispatch::x64_consts::EAGER_DEOPT_EXIT_SIZE, 4);
  assert_eq!(arch_dispatch::x64_consts::LAZY_DEOPT_EXIT_SIZE, 8);
  assert_eq!(arch_dispatch::x64_consts::LAZY_DEOPT_EXIT_SIZE_ALT2, 4);
  assert_eq!(arch_dispatch::arm64_consts::INSTR_SIZE, 4);
  assert_eq!(arch_dispatch::arm64_consts::EAGER_DEOPT_EXIT_SIZE, 4);
  assert_eq!(arch_dispatch::arm64_consts::LAZY_DEOPT_EXIT_SIZE, 4);
  assert_eq!(arch_dispatch::arm64_consts::LAZY_DEOPT_EXIT_SIZE_CFI, 8);
  assert_eq!(
    arch_dispatch::arm64_consts::ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT,
    0
  );

  // x64 与 arm64 独立模块常量
  assert_eq!(x64::EAGER_DEOPT_EXIT_SIZE, 4);
  assert_eq!(x64::LAZY_DEOPT_EXIT_SIZE, 4);
  assert_eq!(x64::LAZY_DEOPT_EXIT_SIZE_CET_IBT, 8);
  assert_eq!(x64::ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT, 0);
  assert_eq!(x64::ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT_CET, 7);
  assert_eq!(x64::PATCH_TO_JUMP_BUFFER_SIZE, 32);
  assert_eq!(x64::ZAP_CODE_EXTRA, 32);

  assert_eq!(arm64::INSTR_SIZE, 4);
  assert_eq!(arm64::EAGER_DEOPT_EXIT_SIZE, 4);
  assert_eq!(arm64::LAZY_DEOPT_EXIT_SIZE, 4);
  assert_eq!(arm64::LAZY_DEOPT_EXIT_SIZE_CFI, 8);
  assert_eq!(arm64::ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT, 0);
  assert_eq!(arm64::PATCH_TO_JUMP_BUFFER_SIZE, 128);

  // 结构锚点与骨架
  assert_eq!(v8_deoptimizer::FRAME_DESCRIPTION_ANCHORS.len(), 2);
  assert_eq!(v8_deoptimizer::VLQ_ANCHOR, "WriteVLQ");
  assert_eq!(v8_deoptimizer::STRUCT_ANCHORS.len(), 2);
  init();
  store_materialized_objects();
  deoptimized_frame_info_skel();

  // 方法面清单
  assert!(DEOPTIMIZER_METHODS.len() >= 10);
  assert!(TRANSLATED_STATE_METHODS.len() >= 15);
  assert!(TRANSLATION_BUILDER_METHODS.len() >= 15);

  OK
}
