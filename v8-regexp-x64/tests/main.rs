use aok::{OK, Void};
use v8_regexp_x64::{
  REGEXP_CODE_SIZE, REGISTERS,
  frame_layout::{sysv, win},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_x64_sysv_frame_layout() -> Void {
  assert_eq!(sysv::FRAME_POINTER_OFFSET, 0);
  assert_eq!(sysv::RETURN_ADDRESS_OFFSET, 8);
  assert_eq!(sysv::FRAME_TYPE_OFFSET, -8);
  assert_eq!(sysv::FRAME_ALIGN, 16);
  assert_eq!(sysv::INPUT_STRING_OFFSET, -16);
  assert_eq!(sysv::START_INDEX_OFFSET, -24);
  assert_eq!(sysv::INPUT_START_OFFSET, -32);
  assert_eq!(sysv::INPUT_END_OFFSET, -40);
  assert_eq!(sysv::REGISTER_OUTPUT_OFFSET, -48);
  assert_eq!(sysv::NUM_OUTPUT_REGISTERS_OFFSET, -56);
  assert_eq!(sysv::DIRECT_CALL_OFFSET, 16);
  assert_eq!(sysv::ISOLATE_OFFSET, 24);
  assert_eq!(sysv::BACKUP_RBX_OFFSET, -64);
  assert_eq!(sysv::BACKUP_R12_OFFSET, -72);
  assert_eq!(sysv::NUM_CALLEE_SAVE_REGISTERS, 2);
  assert_eq!(sysv::LAST_CALLEE_SAVE_REGISTER, -72);
  assert_eq!(sysv::SUCCESSFUL_CAPTURES_OFFSET, -80);
  assert_eq!(sysv::STRING_START_MINUS_ONE_OFFSET, -88);
  assert_eq!(sysv::BACKTRACK_COUNT_OFFSET, -96);
  assert_eq!(sysv::REGEXP_STACK_BASE_POINTER_OFFSET, -104);
  assert_eq!(sysv::REGISTER_ZERO_OFFSET, -112);

  // 内部寄存器寻址计算 (kRegisterZeroOffset - index * 8)
  assert_eq!(sysv::register_location_offset(0), -112);
  assert_eq!(sysv::register_location_offset(1), -120);
  assert_eq!(sysv::register_location_offset(5), -152);
  OK
}

#[test]
fn test_x64_win_frame_layout() -> Void {
  assert_eq!(win::FRAME_POINTER_OFFSET, 0);
  assert_eq!(win::RETURN_ADDRESS_OFFSET, 8);
  assert_eq!(win::FRAME_TYPE_OFFSET, -8);
  assert_eq!(win::FRAME_ALIGN, 16);
  assert_eq!(win::INPUT_STRING_OFFSET, 16);
  assert_eq!(win::START_INDEX_OFFSET, 24);
  assert_eq!(win::INPUT_START_OFFSET, 32);
  assert_eq!(win::INPUT_END_OFFSET, 40);
  assert_eq!(win::REGISTER_OUTPUT_OFFSET, 48);
  assert_eq!(win::NUM_OUTPUT_REGISTERS_OFFSET, 56);
  assert_eq!(win::DIRECT_CALL_OFFSET, 64);
  assert_eq!(win::ISOLATE_OFFSET, 72);
  assert_eq!(win::BACKUP_RSI_OFFSET, -16);
  assert_eq!(win::BACKUP_RDI_OFFSET, -24);
  assert_eq!(win::BACKUP_RBX_OFFSET, -32);
  assert_eq!(win::BACKUP_R12_OFFSET, -40);
  assert_eq!(win::NUM_CALLEE_SAVE_REGISTERS, 4);
  assert_eq!(win::LAST_CALLEE_SAVE_REGISTER, -40);
  assert_eq!(win::SUCCESSFUL_CAPTURES_OFFSET, -48);
  assert_eq!(win::STRING_START_MINUS_ONE_OFFSET, -56);
  assert_eq!(win::BACKTRACK_COUNT_OFFSET, -64);
  assert_eq!(win::REGEXP_STACK_BASE_POINTER_OFFSET, -72);
  assert_eq!(win::REGISTER_ZERO_OFFSET, -80);

  // 内部寄存器寻址计算 (kRegisterZeroOffset - index * 8)
  assert_eq!(win::register_location_offset(0), -80);
  assert_eq!(win::register_location_offset(1), -88);
  assert_eq!(win::register_location_offset(5), -120);
  OK
}

#[test]
fn test_x64_abi_contracts_and_registers() -> Void {
  assert_eq!(REGEXP_CODE_SIZE, 1024);
  assert_eq!(REGISTERS.len(), 8);

  // 校验硬件寄存器分配映射
  assert!(
    REGISTERS
      .iter()
      .any(|&(n, r)| n == "current_character" && r == "rdx")
  );
  assert!(
    REGISTERS
      .iter()
      .any(|&(n, r)| n == "current_position" && r == "rdi")
  );
  assert!(
    REGISTERS
      .iter()
      .any(|&(n, r)| n == "input_end" && r == "rsi")
  );
  assert!(
    REGISTERS
      .iter()
      .any(|&(n, r)| n == "frame_pointer" && r == "rbp")
  );
  assert!(
    REGISTERS
      .iter()
      .any(|&(n, r)| n == "stack_pointer" && r == "rsp")
  );
  assert!(
    REGISTERS
      .iter()
      .any(|&(n, r)| n == "backtrack_stackpointer" && r == "rbx")
  );
  assert!(
    REGISTERS
      .iter()
      .any(|&(n, r)| n == "code_object_pointer" && r == "r8")
  );
  assert!(
    REGISTERS
      .iter()
      .any(|&(n, r)| n == "regexp_stack" && r == "r12")
  );

  // 双 ABI 结构差异：
  // 1. SysV 将参数压入 callee 帧内 (偏移 < 0)，Win 将参数置于 caller 预留区 (偏移 > 0)
  const {
    assert!(sysv::INPUT_STRING_OFFSET < 0);
    assert!(win::INPUT_STRING_OFFSET > 0);
  }

  // 2. Win 多备份 rsi, rdi 两个寄存器 (4 个 vs 2 个，差值 16 字节)
  assert_eq!(
    win::NUM_CALLEE_SAVE_REGISTERS - sysv::NUM_CALLEE_SAVE_REGISTERS,
    2
  );

  // 3. SysV 帧中参数区与 callee-save 累积导致本地变量基准差 32 字节 (-80 vs -48)
  assert_eq!(
    win::SUCCESSFUL_CAPTURES_OFFSET - sysv::SUCCESSFUL_CAPTURES_OFFSET,
    32
  );
  assert_eq!(win::REGISTER_ZERO_OFFSET - sysv::REGISTER_ZERO_OFFSET, 32);

  OK
}
