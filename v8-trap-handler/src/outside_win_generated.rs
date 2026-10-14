// 由 tool/src/trap-handler/handler-inside-win.h.js 从 v8/src/trap-handler/handler-outside-win.cc 生成; 修订就地改, 复跑覆盖
// Windows 处理器注册

/// Windows 注册骨架 (原版 handler-outside-win.cc)
pub mod handler_outside_win_skel {
  /// AddVectoredExceptionHandler(first=TRUE, HandleWasmTrap) — 插队链首
  pub fn register_default_trap_handler() -> bool {
    unimplemented!("骨架")
  }

  /// RemoveVectoredExceptionHandler(句柄), 未注册则空转
  pub fn remove_trap_handler() {
    unimplemented!("骨架")
  }

  /// first 参数 (TRUE: 链首)
  pub const FIRST_HANDLER: u32 = 1;
}
