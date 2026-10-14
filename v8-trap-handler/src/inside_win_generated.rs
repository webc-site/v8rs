// 由 tool/src/trap-handler/handler-inside-win.h.js 从 v8/src/trap-handler/handler-inside-win.{h,cc} 生成; 修订就地改, 复跑覆盖
// Windows 向量异常处理协议

/// Windows 例外码 (winnt.h 显式展开)
pub mod win_exceptions {
  /// EXCEPTION_ACCESS_VIOLATION
  pub const ACCESS_VIOLATION: u32 = 0xC000_0005;
  /// EXCEPTION_CONTINUE_EXECUTION (-1: 已修复, 回到触发点)
  pub const CONTINUE_EXECUTION: i32 = -1;
  /// EXCEPTION_CONTINUE_SEARCH (0: 不处理, 继续搜寻)
  pub const CONTINUE_SEARCH: i32 = 0;
}

/// TEB 布局 (原版 struct TEB): 11 个保留指针后是 TLS 指针;
/// 64 位平台 TLS 偏移 0x58, 32 位 0x2c (x86/x64/ARM/ARM64 通用, 编进二进制不可改)
pub mod teb_layout {
  pub const RESERVED_POINTERS: usize = 11;
  /// TLS 指针槽序号 (0-based, 第 12 个指针)
  pub const TLS_POINTER_SLOT: usize = 11;
  /// 64 位偏移 (字节)
  pub const TLS_OFFSET_64BIT: usize = 0x58;
  /// 32 位偏移 (字节)
  pub const TLS_OFFSET_32BIT: usize = 0x2c;
}

/// 恢复协议寄存器面 (原版 TryHandleWasmTrap, x64/arm64 展开)
pub mod win_restore_protocol {
  /// x64: Rip ← landing pad; 故障地址走 R10
  pub const X64_RESTORE: [&str; 2] = ["Rip=gLandingPad", "R10=fault_addr"];
  /// arm64: Pc ← landing pad; 故障地址走 X16
  pub const ARM64_RESTORE: [&str; 2] = ["Pc=gLandingPad", "X16=fault_addr"];
  /// 模拟器分支: Rax ← landing pad; Rip ← probe_memory_continuation;
  /// simulated_ip 自第二参数寄存器 Rdx 读取
  pub const SIM_RESTORE: [&str; 3] = ["Rax=gLandingPad", "Rip=continuation", "simulated_ip=Rdx"];
}

/// Windows 处理链签名骨架 (语义见 crate 内手写 inside.rs)
pub mod handler_inside_win_skel {
  /// 拒处理快路径 (原版 TryHandleWasmTrap): 非 ACCESS_VIOLATION /
  /// TEB TLS 未分配 / TrapHandlerGuard 活跃 → false; 之后挂 guard 再判覆盖
  pub fn try_handle_wasm_trap(_exception: usize) -> bool {
    unimplemented!("骨架")
  }

  /// 向量异常入口 (原版 HandleWasmTrap): 处理→CONTINUE_EXECUTION, 否则 CONTINUE_SEARCH
  pub fn handle_wasm_trap(_exception: usize) -> i32 {
    unimplemented!("骨架")
  }
}
