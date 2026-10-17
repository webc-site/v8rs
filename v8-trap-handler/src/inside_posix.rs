// 由 tool/src/trap-handler/handler-inside-posix.js 从 v8/src/trap-handler/{handler-inside-posix.h,handler-inside-posix.cc} 生成; 修订就地改, 复跑覆盖

//! posix 信号处理器内部 (h + cc 合并转写)
//! 规则区原文行区间: ["kOobSignal","ctxMacro","kernelSig","unmask","tryHandle","handle"]
//! 本模块运行在信号上下文: 禁分配/禁 std 锁/禁 panic, 全程 async-signal-safe;
//! 模拟器分支 (V8_TRAP_HANDLER_VIA_SIMULATOR) 登记推迟
use std::{ffi::c_void, mem::zeroed, ptr::null_mut, sync::atomic::Ordering};

// XSI si_code 用户态来源标记 (IsKernelGeneratedSignal 排除集);
// libc 仅 linux 导出, darwin/freebsd 按 XSI ABI 值补齐
#[cfg(target_os = "linux")]
use libc::{SI_ASYNCIO, SI_MESGQ, SI_QUEUE, SI_TIMER, SI_USER};

use crate::{
  inside::{is_accessed_memory_covered_impl, is_fault_address_covered_impl},
  outside_posix::remove_trap_handler,
  shared::{LANDING_PAD, TrapHandlerGuard},
};
#[cfg(not(target_os = "linux"))]
mod xsi_si_code {
  pub(super) const SI_USER: i32 = 0x10001;
  pub(super) const SI_QUEUE: i32 = 0x10002;
  pub(super) const SI_TIMER: i32 = 0x10003;
  pub(super) const SI_ASYNCIO: i32 = 0x10004;
  pub(super) const SI_MESGQ: i32 = 0x10005;
}
#[cfg(not(target_os = "linux"))]
use xsi_si_code::{SI_ASYNCIO, SI_MESGQ, SI_QUEUE, SI_TIMER, SI_USER};

/// handler-inside-posix.h:17 kOobSignal
/// Linux/FreeBSD 用 SIGSEGV, macOS 用 SIGBUS (原版 #error 平台不走此分支)
pub(crate) const K_OOB_SIGNAL: i32 = if cfg!(any(target_os = "linux", target_os = "freebsd")) {
  libc::SIGSEGV
} else if cfg!(target_os = "macos") {
  libc::SIGBUS
} else {
  libc::SIGSEGV
};

/// handler-inside-posix.cc:79 IsKernelGeneratedSignal
/// macOS 的 si_code 默认 0 (非硬件来源), 其余排除项仅 Linux 语境相关
fn is_kernel_generated_signal(info: &libc::siginfo_t) -> bool {
  let code = info.si_code;
  code > 0
    && code != SI_USER
    && code != SI_QUEUE
    && code != SI_TIMER
    && code != SI_ASYNCIO
    && code != SI_MESGQ
}

/// 取故障访问地址 (info->si_addr); libc 三平台均提供 unsafe 访问器
/// (linux 为 union 重解释, darwin/freebsd 为字段直读)
fn si_addr(info: &libc::siginfo_t) -> usize {
  // SAFETY: 仅读 siginfo 的指针字段, 不涉写
  unsafe { info.si_addr() as usize }
}

/// handler-inside-posix.cc:88 UnmaskOobSignalScope
/// 解除 kOobSignal 屏蔽: 处理器内的二次崩溃可交给崩溃报告器;
/// Drop 恢复屏蔽字 (sigemptyset/sigaddset/pthread_sigmask 均 async-signal-safe)
struct UnmaskOobSignalScope {
  old_mask: libc::sigset_t,
}

impl UnmaskOobSignalScope {
  /// SAFETY: 仅信号处理器上下文调用
  unsafe fn new() -> Self {
    // SAFETY: sig 族调用均 async-signal-safe; zeroed 为 sigset POD 全零初值
    unsafe {
      let mut sigs: libc::sigset_t = zeroed();
      libc::sigemptyset(&mut sigs);
      libc::sigaddset(&mut sigs, K_OOB_SIGNAL);
      let mut old: libc::sigset_t = zeroed();
      libc::pthread_sigmask(libc::SIG_UNBLOCK, &sigs, &mut old);
      Self { old_mask: old }
    }
  }
}

impl Drop for UnmaskOobSignalScope {
  fn drop(&mut self) {
    // SAFETY: 恢复进入处理器前的信号屏蔽字; async-signal-safe
    unsafe {
      libc::pthread_sigmask(libc::SIG_SETMASK, &self.old_mask, null_mut());
    }
  }
}

// ---- handler-inside-posix.cc:53-77 CONTEXT_REG/CONTEXT_PC 宏的平台分派 ----
// 宏返回寄存器槽地址 → Rust 拆为按平台 cfg 的小函数, 返回裸指针

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn ip_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*uc).uc_mcontext.gregs[libc::REG_RIP as usize] as *mut libc::c_long).cast() }
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn ip_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*uc).uc_mcontext.pc as *mut u64).cast() }
}

#[cfg(all(target_os = "linux", target_arch = "riscv64"))]
fn ip_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*uc).uc_mcontext.__gregs[libc::REG_PC as usize] as *mut libc::greg_t).cast() }
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn ip_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*(*uc).uc_mcontext).__ss.__rip as *mut u64).cast() }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn ip_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*(*uc).uc_mcontext).__ss.__pc as *mut u64).cast() }
}

#[cfg(all(target_os = "freebsd", target_arch = "x86_64"))]
fn ip_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*uc).uc_mcontext.mc_rip as *mut i64).cast() }
}

/// 非支持平台占位; TRAP_HANDLER_SUPPORTED 为 false 时不会执行到取 IP
#[cfg(not(any(
  all(target_os = "linux", target_arch = "x86_64"),
  all(target_os = "linux", target_arch = "aarch64"),
  all(target_os = "linux", target_arch = "riscv64"),
  all(target_os = "macos", target_arch = "x86_64"),
  all(target_os = "macos", target_arch = "aarch64"),
  all(target_os = "freebsd", target_arch = "x86_64"),
)))]
fn ip_ptr(_uc: *mut libc::ucontext_t) -> *mut usize {
  null_mut()
}

/// 故障地址专用寄存器槽 (x64: r10; arm64: x16; riscv64: t6)
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn fault_reg_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*uc).uc_mcontext.gregs[libc::REG_R10 as usize] as *mut libc::c_long).cast() }
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn fault_reg_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*uc).uc_mcontext.regs[16] as *mut u64).cast() }
}

#[cfg(all(target_os = "linux", target_arch = "riscv64"))]
fn fault_reg_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*uc).uc_mcontext.__gregs[libc::REG_T6 as usize] as *mut libc::greg_t).cast() }
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn fault_reg_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*(*uc).uc_mcontext).__ss.__r10 as *mut u64).cast() }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn fault_reg_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*(*uc).uc_mcontext).__ss.__x[16] as *mut u64).cast() }
}

#[cfg(all(target_os = "freebsd", target_arch = "x86_64"))]
fn fault_reg_ptr(uc: *mut libc::ucontext_t) -> *mut usize {
  // SAFETY: uc 由内核投递, 寄存器槽有效; 仅计算槽地址
  unsafe { (&mut (*uc).uc_mcontext.mc_r10 as *mut i64).cast() }
}

#[cfg(not(any(
  all(target_os = "linux", target_arch = "x86_64"),
  all(target_os = "linux", target_arch = "aarch64"),
  all(target_os = "linux", target_arch = "riscv64"),
  all(target_os = "macos", target_arch = "x86_64"),
  all(target_os = "macos", target_arch = "aarch64"),
  all(target_os = "freebsd", target_arch = "x86_64"),
)))]
fn fault_reg_ptr(_uc: *mut libc::ucontext_t) -> *mut usize {
  null_mut()
}

/// handler-inside-posix.cc:119 TryHandleSignal
/// 判定并接管 wasm OOB 故障: 改写 ucontext IP → landing pad,
/// 故障地址 → 平台专用寄存器; 返回 false 表示非本处理器职责 (调用方 re-raise)
/// SAFETY: 仅信号处理器上下文调用; info/context 必须由内核投递
pub(crate) unsafe fn try_handle_signal(
  signum: i32,
  info: *const libc::siginfo_t,
  context: *mut c_void,
) -> bool {
  // :121 guard 已激活 (嵌套故障/持锁路径) → 放弃处理
  if TrapHandlerGuard::is_active() {
    return false;
  }

  // :124 激活守卫: 本线程后续故障不再进入处理器
  let _active_guard = TrapHandlerGuard::new();

  // :127 非目标信号 → 放弃
  if signum != K_OOB_SIGNAL {
    return false;
  }

  // :130 仅内核生成的信号 (排除用户态注入)
  // SAFETY: info 由内核投递, 只读
  if !is_kernel_generated_signal(unsafe { &*info }) {
    return false;
  }

  // :136 故障访问地址必须在已注册覆盖内存内 (wasm 内存/保护区间)
  // SAFETY: info 由内核投递, 只读
  let access_addr = si_addr(unsafe { &*info });
  if !is_accessed_memory_covered_impl(access_addr) {
    return false;
  }

  // :145 独立作用域: 先恢复信号屏蔽, 再释放 guard (Drop 逆序即原版语义)
  {
    // SAFETY: 信号处理器上下文; 内部仅调用 async-signal-safe 的 sig 族函数
    let _unmask_oob_signal = unsafe { UnmaskOobSignalScope::new() };

    let uc = context.cast::<libc::ucontext_t>();
    // SAFETY: uc 为内核投递的 ucontext; 读当前 IP
    let fault_addr = unsafe { *ip_ptr(uc) };

    // :192 PC 必须命中已注册代码对象的受保护指令
    if !is_fault_address_covered_impl(fault_addr) {
      return false;
    }

    // :193+:195 landing pad 非零仅 debug 断言 (原版 TH_DCHECK); release 下
    // 取值一次复用, 改写返回地址 → landing pad; 故障地址 → 专用寄存器
    let landing = LANDING_PAD.load(Ordering::SeqCst);
    debug_assert!(landing != 0);
    // SAFETY: 目标槽为内核投递 ucontext 的 IP 字段
    unsafe { *ip_ptr(uc) = landing };
    // SAFETY: 目标槽为平台专用暂存寄存器 (x64 r10 / arm64 x16)
    unsafe { *fault_reg_ptr(uc) = fault_addr };
  }
  true
}

/// handler-inside-posix.cc:214 HandleSignal (sigaction 回调)
/// 接管失败时恢复原处理器; 内核故障返回后重演, 用户信号主动 re-raise
pub(crate) extern "C" fn handle_signal(
  signum: i32,
  info: *mut libc::siginfo_t,
  context: *mut c_void,
) {
  // SAFETY: 信号上下文; info/context 由内核投递, 有效期内完成判定
  let handled = unsafe { try_handle_signal(signum, info, context) };
  if !handled {
    // :224 V8 不处理 → 恢复原处理器 (内核故障返回后重演, 走常规处理)
    remove_trap_handler();
    // SAFETY: 读内核投递的 siginfo
    let kernel = unsafe { is_kernel_generated_signal(&*info) };
    if !kernel {
      // 用户态发送的信号: raise 为 async-signal-safe
      unsafe { libc::raise(signum) };
    }
  }
  // :229 接管路径: TryHandleSignal 已改写 context 的返回地址
}

#[cfg(test)]
mod tests {
  use std::{mem::zeroed, ptr::null_mut};

  use super::*;

  fn zeroed_siginfo() -> libc::siginfo_t {
    // SAFETY: siginfo_t 为 POD; zeroed 后仅改 si_code 供分类判定
    unsafe { zeroed() }
  }

  /// 按 mac 平台取 (IP 槽, 故障地址寄存器槽): arm64 (__pc, __x[16]),
  /// x64 (__rip, __r10) —— 与 ip_ptr/fault_reg_ptr 的 cfg 分支一一对应
  #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
  fn ctx_regs(uc: &mut libc::ucontext_t) -> (&mut u64, &mut u64) {
    // SAFETY: uc.uc_mcontext 指向测试栈上的 mcontext 实例
    let ss = unsafe { &mut (*uc.uc_mcontext).__ss };
    (&mut ss.__pc, &mut ss.__x[16])
  }

  #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
  fn ctx_regs(uc: &mut libc::ucontext_t) -> (&mut u64, &mut u64) {
    // SAFETY: 同上
    let ss = unsafe { &mut (*uc.uc_mcontext).__ss };
    (&mut ss.__rip, &mut ss.__r10)
  }

  /// TryHandleSignal 接管改写: 伪造内核投递的 siginfo/ucontext,
  /// 断言 IP 槽 ← landing pad、平台故障寄存器槽 ← 故障 PC;
  /// 非保护偏移则不接管且上下文原样 (linux 的 siginfo union 字段不可写,
  /// 真机路径由 tests/signal.rs 覆盖)
  #[cfg(all(
    target_os = "macos",
    any(target_arch = "aarch64", target_arch = "x86_64")
  ))]
  #[test]
  fn try_handle_rewrites_context() {
    use crate::{
      TrappingInstructionData,
      outside::{
        get_recovered_trap_count, register_covered_memory, register_handler_data,
        release_handler_data, set_landing_pad, unregister_covered_memory,
      },
    };

    // SAFETY: POD 全零初值; mctx 由 uc.uc_mcontext 指向, 生命周期覆盖调用
    let mut mctx: libc::__darwin_mcontext64 = unsafe { zeroed() };
    let mut uc: libc::ucontext_t = unsafe { zeroed() };
    uc.uc_mcontext = &mut mctx;
    {
      let (ip, fault_reg) = ctx_regs(&mut uc);
      *ip = 0x2050;
      *fault_reg = 0xFFFF_0000; // 脏值: 验证被覆写
    }
    let mut info = zeroed_siginfo();
    info.si_code = libc::BUS_ADRERR; // 内核生成 (> 0)
    info.si_addr = 0x1080usize as *mut libc::c_void; // 落在覆盖内存内

    register_covered_memory(0x1000, 0x1000);
    let idx = register_handler_data(
      0x2000,
      0x100,
      &[TrappingInstructionData { instr_offset: 0x50 }],
    )
    .unwrap();
    set_landing_pad(0xABCD);

    let before = get_recovered_trap_count();
    // SAFETY: siginfo/ucontext 为本测试构造, 调用期间存活
    let handled =
      unsafe { try_handle_signal(K_OOB_SIGNAL, &info, &mut uc as *mut _ as *mut c_void) };
    assert!(handled);
    {
      let (ip, fault_reg) = ctx_regs(&mut uc);
      assert_eq!(*ip, 0xABCD); // IP ← landing pad
      assert_eq!(*fault_reg, 0x2050); // 故障寄存器 ← 故障 PC
    }
    assert_eq!(get_recovered_trap_count(), before + 1);

    // 范围内但非保护偏移 (0x2000+0x51) → 不接管, 上下文不被触碰
    {
      let (ip, fault_reg) = ctx_regs(&mut uc);
      *ip = 0x2051;
      *fault_reg = 0;
    }
    // SAFETY: 同上
    let handled =
      unsafe { try_handle_signal(K_OOB_SIGNAL, &info, &mut uc as *mut _ as *mut c_void) };
    assert!(!handled);
    {
      let (ip, fault_reg) = ctx_regs(&mut uc);
      assert_eq!(*ip, 0x2051);
      assert_eq!(*fault_reg, 0);
    }

    release_handler_data(idx);
    unregister_covered_memory(0x1000, 0x1000);
    set_landing_pad(0);
  }

  /// handler-inside-posix.cc:79 内核信号判定: si_code 排除集
  #[test]
  fn kernel_generated_signal() {
    let mut info = zeroed_siginfo();
    info.si_code = 1; // 如 SEGV_MAPERR
    assert!(is_kernel_generated_signal(&info));
    info.si_code = SI_USER;
    assert!(!is_kernel_generated_signal(&info));
    info.si_code = 0; // macOS 非硬件来源默认值
    assert!(!is_kernel_generated_signal(&info));
    info.si_code = SI_QUEUE; // 用户态注入 (linux 为负值, darwin 为 0x10002)
    assert!(!is_kernel_generated_signal(&info));
  }

  /// TryHandleSignal 拒绝路径: guard 激活时嵌套信号直接放弃 (防死锁);
  /// 非目标信号、非内核信号同样放弃
  #[test]
  fn try_handle_rejects() {
    let info = zeroed_siginfo();
    // guard 激活期间: 第一道检查短路, 不触 context
    let guard = TrapHandlerGuard::new();
    // SAFETY: 拒绝路径不访问 context (guard 检查在最前)
    assert!(!unsafe { try_handle_signal(K_OOB_SIGNAL, &info, null_mut()) });
    drop(guard);
    // 非目标信号
    // SAFETY: signum 检查先于 context 访问
    assert!(!unsafe { try_handle_signal(K_OOB_SIGNAL + 1, &info, null_mut()) });
    // 目标信号但 si_code = 0 (非内核生成)
    // SAFETY: si_code 检查先于 context 访问
    assert!(!unsafe { try_handle_signal(K_OOB_SIGNAL, &info, null_mut()) });
  }
}
