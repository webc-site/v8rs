// 由 tool/src/trap-handler/trap-handler.h.js 从 v8/src/trap-handler/trap-handler.h 生成; 修订就地改, 复跑覆盖

//! wasm OOB 陷阱处理器: SIGSEGV/SIGBUS 拦截 → landing pad 跳转
//! 规则区原文行区间: ["supported","api"]
//!
//! posix 子集落地; windows (handler-inside-win/handler-outside-win) 与
//! simulator (handler-outside-simulator/trap-handler-simulator) 登记推迟
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod inside;
#[cfg(unix)]
mod inside_posix;
mod internal;
mod outside;
#[cfg(unix)]
mod outside_posix;
mod shared;

pub use error::{Error, Result};
pub use inside::{is_accessed_memory_covered, is_fault_address_covered};
pub use outside::{
  enable_trap_handler, get_recovered_trap_count, is_trap_handler_enabled, register_covered_memory,
  register_handler_data, register_v8_sandbox, release_handler_data, set_landing_pad,
  unregister_covered_memory, unregister_v8_sandbox,
};
#[cfg(unix)]
pub use outside_posix::{register_default_trap_handler, remove_trap_handler};
#[cfg(not(unix))]
pub fn register_default_trap_handler() -> bool {
  false
}
#[cfg(not(unix))]
pub fn remove_trap_handler() {}

/// trap-handler.h:102 TrappingInstructionData
/// 陷入指令数据: 指令在代码对象内的偏移
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrappingInstructionData {
  /// 指令距代码对象起点的偏移; wasm 代码 < 2GB, u32 足够
  pub instr_offset: u32,
}

/// trap-handler.h:108 kInvalidIndex: RegisterHandlerData 失败标识
pub const K_INVALID_INDEX: i32 = -1;

/// trap-handler.h:19-53 V8_TRAP_HANDLER_SUPPORTED 的 Rust 对应
/// (Rust 天然 host == target; android 因信号处理安全审查未过而被原版排除)
pub const TRAP_HANDLER_SUPPORTED: bool = cfg!(any(
  all(
    target_arch = "x86_64",
    any(
      target_os = "linux",
      target_os = "macos",
      target_os = "freebsd"
    )
  ),
  all(
    target_arch = "aarch64",
    any(target_os = "linux", target_os = "macos")
  ),
  all(target_arch = "riscv64", target_os = "linux"),
));
