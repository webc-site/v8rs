// 由 tool/src/libsampler/sampler.js 从 v8/src/libsampler/sampler.h, src/libsampler/sampler.cc 生成; 修订就地改, 复跑覆盖
// 库级采样器; USE_SIGNALS 在 x64+arm64 的 linux/darwin 目标恒开, 信号路径为唯一采样路径

// 对数: 帧上限针 2 + 类面针 19 + FillRegisterState 架构分支 10/系统分支 10 +
//       gregs 索引 10/15/16 + arm64 regs 29/30 + SIGPROF 引用 6 (代码 4 + 注释 2) 全过

use core::sync::atomic::AtomicBool;

/// 采样帧上限 log2 (原版 Sampler::kMaxFramesCountLog2)
pub const MAX_FRAMES_COUNT_LOG2: u32 = 8;
/// 采样帧上限 (原版 Sampler::kMaxFramesCount = (1u << 8) - 1 展开)
pub const MAX_FRAMES_COUNT: usize = 255;

/// USE_SIGNALS 展开: 原 #if V8_OS_POSIX && !V8_OS_CYGWIN && !V8_OS_FUCHSIA
/// x64+arm64 目标仅 linux/darwin(均 POSIX, 非 Cygwin/Fuchsia) → 恒真
pub const USE_SIGNALS: bool = true;

// 平台分支登记 (规则 9: 按 x64+arm64 展开, 其余跳过):
// - 架构跳过 (FillRegisterState 内): ARM / IA32 / LOONG64 / MIPS64 / PPC64 / RISCV32 / RISCV64 / S390X
// - 系统跳过 (FillRegisterState 内): AIX / FREEBSD / IOS / NETBSD / OPENBSD / QNX / SOLARIS / ZOS
// - FUCHSIA zx_* 路径 / WIN DuplicateHandle+CONTEXT 路径整体跳过 (非 POSIX 信号路径)
// - ANDROID 兼容 ucontext 补丁块 (cc:81-182) 跳过; SIGPROF=27 (linux/macos 系统头同值)

/// SIGPROF 信号编号 (linux 与 darwin 同为 27; 原版引用 6 处 = 代码 4 + 注释 2)
pub const SIGPROF: i32 = 27;

/// sigaction.sa_flags 展开 (原版 sa.sa_flags = SA_RESTART | SA_SIGINFO | SA_ONSTACK)
#[cfg(target_os = "linux")]
pub const SIGACTION_FLAGS: i32 = 0x1000_0000 | 0x0000_0004 | 0x0800_0000; // RESTART|SIGINFO|ONSTACK
#[cfg(target_os = "macos")]
pub const SIGACTION_FLAGS: i32 = 0x0000_0002 | 0x0000_0040 | 0x0000_0001; // RESTART|SIGINFO|ONSTACK

/// linux-x64 寄存器提取 gregs[] 索引 (原版 mcontext.gregs[REG_R*];
/// 取自 cc:179 枚举, glibc ucontext 同值; 原版引用 cc:421-423)
pub mod linux_x64 {
  /// RBP → pc 用 RIP
  pub const RBP: usize = 10;
  pub const RSP: usize = 15;
  pub const RIP: usize = 16;
}

/// linux-arm64 寄存器提取索引 (原版 cc:439-444: pc=mcontext.pc, sp=mcontext.sp,
/// FP 是 x29 别名, LR 是 x30 别名)
pub mod linux_arm64 {
  pub const FP_REG: usize = 29;
  pub const LR_REG: usize = 30;
}

// darwin/win 寄存器提取面 (accessor/字段直取, 无索引表):
//   darwin-x64: __ss.__rip / __ss.__rsp / __ss.__rbp (cc:503-505)
//   darwin-arm64: arm_thread_state64_get_{pc,sp,fp,lr} (cc:511-518)
//   win-x64: CONTEXT.Rip / Rsp / Rbp (DoSample, cc:637-639)
//   win-arm64: CONTEXT.Pc / Sp / Fp (DoSample, cc:641-643)

/// 原版 using AtomicMutex = std::atomic_bool (h:100)
pub type AtomicMutex = AtomicBool;

/// 采样寄存器状态骨架 (正主 v8::RegisterState 定义在 include/v8-unwinder.h;
/// 此处仅采样面: pc/sp/fp 必有, lr 仅 arm64/win32 路径写回)
#[derive(Debug, Clone, Copy, Default)]
pub struct RegisterState {
  pub pc: usize,
  pub sp: usize,
  pub fp: usize,
  pub lr: Option<usize>,
}

/// 采样回调面 (原版 virtual void SampleStack(const v8::RegisterState&) = 0; 客户端覆写)
pub trait SampleStackHook {
  fn sample_stack(&mut self, regs: &RegisterState);
}

// ── 类面骨架 (修订就地补语义) ──
//   Sampler (isolate_; active_/record_sample_: AtomicMutex 即 atomic_bool;
//     Start/Stop/IsActive(relaxed load)/ShouldRecordSample(exchange(false))/
//     DoSample/SetActive/SetShouldRecordSample;
//     计数面: is_counting_samples_/js_sample_count_/external_sample_count_ + StartCountingSamples)
//   AtomicGuard (AtomicMutex* + is_success_; 构造 compare_exchange_strong 抢锁,
//     is_blocking=false 只试一次; 析构持锁才放)
//   SamplerManager (sampler_map_: 按 vm_tid i32 分桶的 SamplerList;
//     AddSampler/RemoveSampler 受 AtomicGuard; DoSample 非阻塞抢锁失败即返回;
//     instance() = base::LeakyObject 全局泄漏单例)
//   SignalHandler (client_count_ 0→1 Install / →0 Restore; sigaction(SIGPROF) 装/卸;
//     HandleProfilerSignal → FillRegisterState → SamplerManager::instance()->DoSample)
//   Sampler::DoSample 信号路径 (cc:614-619): RecursiveMutexGuard + Installed 检查 +
//     SetShouldRecordSample + pthread_kill(vm_tself, SIGPROF)

// ── 提取不变式断言 ──
const _: () = {
  assert!(MAX_FRAMES_COUNT_LOG2 == 8);
  assert!(MAX_FRAMES_COUNT == 255);
  assert!(USE_SIGNALS);
  assert!(SIGPROF == 27);
  assert!(linux_x64::RBP == 10 && linux_x64::RSP == 15 && linux_x64::RIP == 16);
  assert!(linux_arm64::FP_REG == 29 && linux_arm64::LR_REG == 30);
};
