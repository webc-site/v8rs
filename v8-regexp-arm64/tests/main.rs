use aok::{OK, Void};
use v8_regexp_arm64::{
  INITIAL_BUFFER_SIZE, NUM_CACHED_REGISTERS, NUM_REGISTERS_TO_UNROLL, REGISTERS, frame_layout,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_arm64_frame_layout() -> Void {
  assert_eq!(frame_layout::K_SYSTEM_POINTER_SIZE, 8);
  assert_eq!(frame_layout::K_W_REG_SIZE, 4);
  assert_eq!(frame_layout::FRAME_POINTER_OFFSET, 0);
  assert_eq!(frame_layout::RETURN_ADDRESS_OFFSET, 8);
  assert_eq!(frame_layout::NUM_CALLEE_SAVED_REGISTERS, 10);
  assert_eq!(frame_layout::CALLEE_SAVED_REGISTERS_OFFSET, 16);
  assert_eq!(frame_layout::FRAME_TYPE_OFFSET, -8);
  assert_eq!(frame_layout::PADDING_AFTER_FRAME_TYPE, 8);
  assert_eq!(frame_layout::ISOLATE_OFFSET, -24);
  assert_eq!(frame_layout::DIRECT_CALL_OFFSET, -32);
  assert_eq!(frame_layout::NUM_OUTPUT_REGISTERS_OFFSET, -40);
  assert_eq!(frame_layout::INPUT_STRING_OFFSET, -48);
  assert_eq!(frame_layout::SUCCESSFUL_CAPTURES_OFFSET, -56);
  assert_eq!(frame_layout::BACKTRACK_COUNT_OFFSET, -64);
  assert_eq!(frame_layout::REGEXP_STACK_BASE_POINTER_OFFSET, -72);
  assert_eq!(frame_layout::STACK_LOCAL_PADDING, -80);
  assert_eq!(frame_layout::NUMBER_OF_STACK_LOCALS, 4);
  assert_eq!(frame_layout::FIRST_REGISTER_ON_STACK_OFFSET, -84);
  assert_eq!(frame_layout::FIRST_CAPTURE_ON_STACK_OFFSET, -88);
  OK
}

#[test]
fn test_arm64_registers_and_constants() -> Void {
  assert_eq!(INITIAL_BUFFER_SIZE, 1024);
  assert_eq!(NUM_REGISTERS_TO_UNROLL, 16);
  assert_eq!(NUM_CACHED_REGISTERS, 16);
  assert_eq!(REGISTERS.len(), 10);

  let expected = [
    ("current_input_offset", "w21"),
    ("current_character", "w22"),
    ("input_end", "x25"),
    ("input_start", "x26"),
    ("start_offset", "w27"),
    ("output_array", "x28"),
    ("frame_pointer", "fp"),
    ("backtrack_stackpointer", "x23"),
    ("code_pointer", "x20"),
    ("regexp_stack", "x19"),
  ];
  for entry in expected {
    assert!(REGISTERS.contains(&entry), "未找到寄存器映射: {entry:?}");
  }

  // 验证栈帧结构相对偏移不变式
  assert_eq!(
    frame_layout::CALLEE_SAVED_REGISTERS_OFFSET,
    frame_layout::RETURN_ADDRESS_OFFSET + frame_layout::K_SYSTEM_POINTER_SIZE
  );
  assert_eq!(
    frame_layout::FIRST_REGISTER_ON_STACK_OFFSET,
    frame_layout::STACK_LOCAL_PADDING - frame_layout::K_W_REG_SIZE
  );
  assert_eq!(
    frame_layout::FIRST_CAPTURE_ON_STACK_OFFSET,
    frame_layout::STACK_LOCAL_PADDING - frame_layout::K_SYSTEM_POINTER_SIZE
  );
  OK
}
