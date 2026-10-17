// 由 tool/src/d8/hardware-watchpoints.h.js 从 v8/src/d8/hardware-watchpoints.{h,cc} 生成; 修订就地改, 复跑覆盖
// d8 硬件观察点: 门控宏 + DR 寄存器面 + 签名骨架

//! d8 硬件观察点 — 原版 v8::(hardware-watchpoints 族)
//!
//! 语义: 沙箱内内存操纵 API 的调试器进程对 — fork 出 tracer 进程经 ptrace
//! 操作 x64 调试寄存器 DR0-DR3 (观察点地址) + DR7 (使能), 命中时改写读值。

/// 使能门控 (原版 #define V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT):
/// V8_ENABLE_MEMORY_CORRUPTION_API && V8_OS_LINUX && V8_TARGET_ARCH_X64 —
/// arm64 目标: 条件不成立 → 本模块整体空转 (登记跳过, 无 arm64 调试寄存器等价物)
pub const ENABLED: bool = false;

/// 硬件观察点槽数 (原版 kNumWatchpoints, DR0-DR3)
pub const NUM_WATCHPOINTS: usize = 4;

// —— 签名骨架 (h 3 公开函数 / cc 定义 1:1) ——
// setup_for_hardware_watchpoints(enable_tracing: bool) — 进程早期调用:
//   fork 双进程 (d8 / debugger), 经 SharedPart 握手 initialization_step
// reset_all_hardware_watchpoints() — 退出时清全部观察点
// set_hardware_watchpoint_callback(info: &FunctionCallbackInfo) — d8 侧 JS 入口

// —— 结构台账 ——
// WatchpointInfo { address: usize } — TODO: 长度字段 (传 DR7) / 读值操纵信息
// HardwareWatchpointSupport:
//   tracing_enabled: bool — --trace-in-sandbox-manipulation-via-watchpoints
//   d8_pid: i32 — 仅 debugger 进程使用
//   rng: RandomNumberGenerator — v8_flags.random_seed 初始化, 随机改写读值
//   kept_alive: Vec<Global<Value>> — 有观察点的对象保活
//   SharedPart (共享内存):
//     has_changed_watchpoints: AtomicBool — d8 写 / debugger 读后复位
//     watchpoints: [WatchpointInfo; 4]
//     ignore_watchpoints: AtomicBool — 可信代码段 (如 object->Size()) 期间忽略命中
//     initialization_step: AtomicI32 — 双进程初始握手
// IgnoreWatchpointsScope — RAII 置/复位 ignore_watchpoints, 不可重入 (DCHECK)

// ptrace 面:
//   PTRACE_POKEUSER u_debugreg[0..3] ← 观察点地址; DR7 控制位按槽使能
//   TRACE(...) 宏 — tracing_enabled 门控的调试打印

// 平台分支台账:
//   V8_OS_LINUX + V8_TARGET_ARCH_X64 双条件 (linux/elf.h, sys/ptrace.h, sys/user.h)
//   arm64: 无硬件观察点等价机制 — 目标架构跳过, 整体登记
