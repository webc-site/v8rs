use aok::{OK, Void};
use log::info;
use v8_execution_frames::{
  BuiltinContinuationMode, FrameInfoKind, JsFrameMarker, SYSTEM_POINTER_SIZE, StackFrame,
  StackFrameType, common_frame_constants, frame_constants_arm64, frame_constants_x64,
  standard_frame_constants, typed_frame_constants,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_stack_frame_types_and_classification() -> Void {
  // 总数校验 (23 项标准帧 + 11 项 wasm 帧 = 34 项)
  assert_eq!(StackFrameType::COUNT, 34);
  assert_eq!(StackFrameType::ALL.len(), 34);

  // JS 帧严格转型分类 (对应原版 IsJavaScript)
  assert!(StackFrameType::Interpreted.is_javascript());
  assert!(StackFrameType::Baseline.is_javascript());
  assert!(StackFrameType::Maglev.is_javascript());
  assert!(StackFrameType::TurbofanJs.is_javascript());
  assert!(!StackFrameType::JavascriptBuiltinContinuation.is_javascript());
  assert!(!StackFrameType::JavascriptBuiltinContinuationWithCatch.is_javascript());
  assert!(!StackFrameType::Entry.is_javascript());
  assert!(!StackFrameType::Wasm.is_javascript());

  // JS 广义家族分类
  assert!(StackFrameType::Interpreted.is_js_family());
  assert!(StackFrameType::JavascriptBuiltinContinuation.is_js_family());
  assert!(StackFrameType::JavascriptBuiltinContinuationWithCatch.is_js_family());
  assert!(!StackFrameType::Entry.is_js_family());

  // WASM 帧严格分类 (对应原版 is_wasm())
  assert!(StackFrameType::Wasm.is_wasm());
  assert!(StackFrameType::WasmInterpreterEntry.is_wasm());
  assert!(!StackFrameType::WasmToJs.is_wasm());
  assert!(!StackFrameType::JsToWasm.is_wasm());

  // WASM 广义家族分类
  assert!(StackFrameType::Wasm.is_wasm_family());
  assert!(StackFrameType::WasmToJs.is_wasm_family());
  assert!(StackFrameType::JsToWasm.is_wasm_family());
  assert!(StackFrameType::WasmLiftoffSetup.is_wasm_family());
  assert!(!StackFrameType::Exit.is_wasm_family());

  // 退出帧严格分类 (对应原版 is_exit())
  assert!(StackFrameType::Exit.is_exit());
  assert!(!StackFrameType::BuiltinExit.is_exit());
  assert!(!StackFrameType::ApiCallbackExit.is_exit());
  assert!(!StackFrameType::WasmExit.is_exit());

  // 退出帧广义家族分类
  assert!(StackFrameType::Exit.is_exit_family());
  assert!(StackFrameType::BuiltinExit.is_exit_family());
  assert!(StackFrameType::ApiCallbackExit.is_exit_family());
  assert!(StackFrameType::WasmExit.is_exit_family());
  assert!(!StackFrameType::Interpreted.is_exit_family());

  // 名称映射
  assert_eq!(StackFrameType::Entry.name(), "ENTRY");
  assert_eq!(StackFrameType::Interpreted.name(), "INTERPRETED");
  assert_eq!(StackFrameType::Wasm.name(), "WASM");
  assert_eq!(StackFrameType::NoFrameType.name(), "NO_FRAME_TYPE");

  info!("StackFrameType 34 项分类与属性测试通过");
  OK
}

#[test]
fn test_auxiliary_enums_and_stack_frame_traversal() -> Void {
  // BuiltinContinuationMode
  assert_eq!(BuiltinContinuationMode::Stub as u8, 0);
  assert_eq!(BuiltinContinuationMode::JavaScript as u8, 1);
  assert_eq!(BuiltinContinuationMode::JavaScriptWithCatch as u8, 2);
  assert_eq!(BuiltinContinuationMode::JavaScriptHandleException as u8, 3);

  // FrameInfoKind
  assert_eq!(FrameInfoKind::Precise as u8, 0);
  assert_eq!(FrameInfoKind::Conservative as u8, 1);

  // JsFrameMarker (Smi 编码: 最外层为 (1 << 1) | 0 = 2)
  assert_eq!(JsFrameMarker::InnerJsEntryFrame as u8, 0);
  assert_eq!(JsFrameMarker::OutermostJsEntryFrame as u8, 2);

  // StackFrame 寻址
  let frame = StackFrame::new(0x7FFF_0000, 0x7FFE_FF00, StackFrameType::Interpreted);
  assert_eq!(frame.fp(), 0x7FFF_0000);
  assert_eq!(frame.sp(), 0x7FFE_FF00);
  assert_eq!(frame.frame_type(), StackFrameType::Interpreted);

  // 遍历标准调用者槽
  assert_eq!(frame.caller_fp_address(), 0x7FFF_0000);
  assert_eq!(frame.caller_pc_address(), 0x7FFF_0008);
  assert_eq!(frame.caller_sp_address(), 0x7FFF_0010);

  info!("附属枚举与 StackFrame 栈槽遍历测试通过");
  OK
}

#[test]
fn test_common_and_standard_frame_constants() -> Void {
  assert_eq!(SYSTEM_POINTER_SIZE, 8);

  // CommonFrameConstants
  assert_eq!(common_frame_constants::CALLER_FP_OFFSET, 0);
  assert_eq!(common_frame_constants::CALLER_PC_OFFSET, 8);
  assert_eq!(common_frame_constants::CALLER_SP_OFFSET, 16);
  assert_eq!(common_frame_constants::FIXED_FRAME_SIZE_ABOVE_FP, 16);
  assert_eq!(common_frame_constants::CONTEXT_OR_FRAME_TYPE_OFFSET, -8);

  // StandardFrameConstants
  assert_eq!(standard_frame_constants::FIXED_FRAME_SIZE_FROM_FP, 24);
  assert_eq!(standard_frame_constants::FIXED_FRAME_SIZE, 40);
  assert_eq!(standard_frame_constants::FUNCTION_OFFSET, -16);
  assert_eq!(standard_frame_constants::CONTEXT_OFFSET, -8);
  assert_eq!(standard_frame_constants::ARG_C_OFFSET, -24);
  assert_eq!(standard_frame_constants::EXPRESSIONS_OFFSET, -32);
  assert_eq!(
    standard_frame_constants::FIRST_PUSHED_FRAME_VALUE_OFFSET,
    -32
  );

  // TypedFrameConstants
  assert_eq!(typed_frame_constants::FRAME_TYPE_OFFSET, -8);
  assert_eq!(typed_frame_constants::FIRST_PUSHED_FRAME_VALUE_OFFSET, -16);
  assert_eq!(typed_frame_constants::pushed_value_offset(0), -16);
  assert_eq!(typed_frame_constants::pushed_value_offset(1), -24);
  assert_eq!(typed_frame_constants::pushed_value_offset(2), -32);

  info!("通用与标准帧常量测试通过");
  OK
}

#[test]
fn test_architecture_specific_frame_constants() -> Void {
  // x64 帧常量
  assert_eq!(
    frame_constants_x64::entry_frame_constants::NEXT_EXIT_FRAME_FP_OFFSET,
    -64
  );
  assert_eq!(
    frame_constants_x64::entry_frame_constants::NEXT_FAST_CALL_FRAME_FP_OFFSET,
    -72
  );
  assert_eq!(
    frame_constants_x64::wasm_liftoff_setup_frame_constants::NUMBER_OF_SAVED_GP_PARAM_REGS,
    5
  );
  assert_eq!(
    frame_constants_x64::wasm_liftoff_setup_frame_constants::NUMBER_OF_SAVED_FP_PARAM_REGS,
    6
  );
  assert_eq!(
    frame_constants_x64::wasm_liftoff_setup_frame_constants::CALLING_PC_OFFSET,
    -16
  );

  // arm64 帧常量
  assert_eq!(
    frame_constants_arm64::entry_frame_constants::NEXT_EXIT_FRAME_FP_OFFSET,
    -24
  );
  assert_eq!(
    frame_constants_arm64::entry_frame_constants::NEXT_FAST_CALL_FRAME_FP_OFFSET,
    -40
  );
  assert_eq!(
    frame_constants_arm64::entry_frame_constants::FIXED_FRAME_SIZE,
    48
  );
  assert_eq!(
    frame_constants_arm64::wasm_liftoff_setup_frame_constants::NUMBER_OF_SAVED_GP_PARAM_REGS,
    6
  );
  assert_eq!(
    frame_constants_arm64::wasm_liftoff_setup_frame_constants::NUMBER_OF_SAVED_FP_PARAM_REGS,
    8
  );
  assert_eq!(
    frame_constants_arm64::wasm_liftoff_setup_frame_constants::CALLING_PC_OFFSET,
    -200
  );

  info!("x64 与 arm64 架构帧常量测试通过");
  OK
}
