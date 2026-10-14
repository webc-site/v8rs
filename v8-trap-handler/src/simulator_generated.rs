// 由 tool/src/trap-handler/handler-inside-win.h.js 从 v8/src/trap-handler/trap-handler-simulator.h + handler-outside-simulator.cc 生成; 修订就地改, 复跑覆盖
// 模拟器 ProbeMemory 探测协议

/// 模拟器探测协议 (V8_TRAP_HANDLER_VIA_SIMULATOR 下生效)
pub mod simulator_protocol {
  /// ProbeMemory 符号名 (darwin 前缀 '_', 其余裸名; 原版 SYMBOL 宏)
  pub const PROBE_MEMORY_SYMBOL: &str = if cfg!(target_os = "macos") {
    "_v8_internal_simulator_ProbeMemory"
  } else {
    "v8_internal_simulator_ProbeMemory"
  };
  /// 续接点符号名
  pub const CONTINUATION_SYMBOL: &str = if cfg!(target_os = "macos") {
    "_v8_simulator_probe_memory_continuation"
  } else {
    "v8_simulator_probe_memory_continuation"
  };

  /// ProbeMemory 语义: 对 address 做 1 字节读, 不可读则触发陷阱;
  /// 处理器按 pc 查 landing pad, 经 continuation 返回该 pad; 可读则返回 0
  pub const PROBE_READ_BYTES: usize = 1;

  /// x64 探测序列 (寄存器传参: win=%rcx, sysv=%rdi; 清零 %eax; ret;
  /// 另在 continuation 处独立 ret 防 Mac subsections 剥离)
  pub const X64_PROBE_STEPS: [&str; 3] = ["movb (%arg), %al", "xorl %eax, %eax", "ret"];
  /// arm64 探测序列 (x0 传参; wzr 读; 清零 x0; ret)
  pub const ARM64_PROBE_STEPS: [&str; 3] = ["ldrb wzr, [x0]", "mov x0, xzr", "ret"];

  /// 续接点语义: 处理器已把 landing pad 写入返回值寄存器 (%rax / x0), 直接 ret
  pub const CONTINUATION_STEPS: [&str; 1] = ["ret"];
}

/// ProbeMemory 签名骨架 (原版 trap-handler-simulator.h 声明, asm 符号绑定)
pub fn probe_memory(_address: usize, _pc: usize) -> usize {
  unimplemented!("骨架")
}
