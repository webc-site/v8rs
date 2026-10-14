#![cfg(unix)]

// 真机信号集成测试:
// - 接管路径 = trap-handler-native-unittest TestTrapHandlerRecovery 的
//   最小本机复刻 (真实故障 + 数据注册 + landing pad 跳转);
// - fallback 路径 = trap-handler-posix-unittest SignalHandlerFallbackTest 的
//   1:1 语义复刻 (不接管 → 恢复原处理器 → 重演 → 预装 fallback 收到信号)。
// PROT_NONE 页访问在 darwin 产生 SIGBUS、在 linux/freebsd 产生 SIGSEGV,
// 均与各自平台的 kOobSignal 对齐 (wasm guard region 的真实故障形态)
use std::{
  ffi::c_void,
  mem::zeroed,
  ptr::{null_mut, write_volatile},
};

use aok::{OK, Void};
use v8_trap_handler::{
  TrappingInstructionData, get_recovered_trap_count, register_covered_memory,
  register_default_trap_handler, register_handler_data, release_handler_data, remove_trap_handler,
  set_landing_pad, unregister_covered_memory,
};

// libc 不绑 setjmp/longjmp (跨 Rust 栈长跳转), 直接绑系统符号;
// 仅测试内使用: 跳转不跨越任何活跃的 guard/锁
unsafe extern "C" {
  fn _setjmp(env: *mut c_void) -> i32;
  fn _longjmp(env: *mut c_void, val: i32) -> !;
}

static mut JMP: *mut c_void = null_mut();

/// landing pad 跳回标记 / fallback 跳回标记
const LANDED: i32 = 7;
const FELLBACK: i32 = 9;

/// 各平台 kOobSignal (darwin=SIGBUS, 其余 posix=SIGSEGV)
fn oob_sig() -> i32 {
  if cfg!(target_os = "macos") {
    libc::SIGBUS
  } else {
    libc::SIGSEGV
  }
}

/// landing pad: 接管成功后故障线程 IP 被改写到此处, 跳回断言点
#[unsafe(no_mangle)]
extern "C" fn landing() {
  unsafe { _longjmp(JMP, LANDED) }
}

/// 真实触发故障: 写 PROT_NONE 页
#[inline(never)]
fn crash(p: *mut u8) {
  // SAFETY: 故障注入即目的本身; p 来自 mmap 的 PROT_NONE 页
  unsafe { write_volatile(p, 1) };
}

/// 模拟 wasm 内存页尺寸
const PAGE_SIZE: usize = 0x1000;
/// setjmp 环境缓冲区字数
const JMP_BUF_WORDS: usize = 64;

/// 映射一页 PROT_NONE (wasm guard region 的最小形态)
fn prot_none_page() -> *mut u8 {
  // SAFETY: 常规匿名映射; 失败即断言
  unsafe {
    let p = libc::mmap(
      null_mut(),
      PAGE_SIZE,
      libc::PROT_NONE,
      libc::MAP_ANON | libc::MAP_PRIVATE,
      -1,
      0,
    );
    assert_ne!(p, libc::MAP_FAILED);
    p as *mut u8
  }
}

/// 接管路径: 真实 kOobSignal 故障 → 覆盖内存命中 → PC 命中受保护指令 →
/// IP 被改写跳入 landing pad; 恢复计数 +1
#[test]
fn real_signal_recovery() -> Void {
  let page = prot_none_page();
  register_covered_memory(page as usize, PAGE_SIZE);

  // 故障指令在 crash 函数内的偏移编译期未知 → 全函数范围注册为受保护
  // (native-unittest 的通用手法)
  let base = crash as *const () as usize;
  let all: Vec<TrappingInstructionData> = (0..PAGE_SIZE as u32)
    .map(|o| TrappingInstructionData { instr_offset: o })
    .collect();
  let idx = register_handler_data(base, PAGE_SIZE, &all).unwrap();
  set_landing_pad(landing as *const () as usize);
  assert!(register_default_trap_handler());

  let mut buf = [0u64; JMP_BUF_WORDS];
  // SAFETY: setjmp 语义上返回两次: 0 = 首次进入; LANDED = landing pad 内
  // longjmp 跳回。两点间无活跃 guard/锁, 栈帧 (buf) 全程存活
  let r = unsafe { _setjmp(buf.as_mut_ptr().cast()) };
  unsafe { JMP = buf.as_mut_ptr().cast() };
  if r == 0 {
    crash(page);
    unreachable!("故障未被接管 (信号已被恢复并重演, 进程应已终止)");
  }
  assert_eq!(r, LANDED);
  assert!(get_recovered_trap_count() >= 1);

  release_handler_data(idx);
  unregister_covered_memory(page as usize, PAGE_SIZE);
  remove_trap_handler();
  OK
}

/// fallback 路径 (原版 SignalHandlerFallbackTest): 未注册任何数据 →
/// 处理器不接管 → 恢复安装前的原处理器 → 故障重演 → 预装 fallback 收到信号
#[test]
fn real_signal_fallback() -> Void {
  let page = prot_none_page();

  /// 预装的"旧处理器" (模拟 ASan 等早期注册者): 跳回断言点
  unsafe extern "C" fn fallback(_s: i32, _i: *mut libc::siginfo_t, _c: *mut c_void) {
    unsafe { _longjmp(JMP, FELLBACK) }
  }

  // SAFETY: sigaction 配置 API, 非信号上下文
  unsafe {
    let mut action: libc::sigaction = zeroed();
    action.sa_sigaction = fallback as *const () as usize;
    action.sa_flags = libc::SA_SIGINFO | libc::SA_ONSTACK;
    libc::sigemptyset(&mut action.sa_mask);
    assert_eq!(libc::sigaction(oob_sig(), &action, null_mut()), 0);
  }

  // 安装 V8 处理器 (上面的 fallback 被保存为旧处理器);
  // 未注册覆盖内存 → is_accessed_memory_covered 判否 → 必不接管
  assert!(register_default_trap_handler());

  let mut buf = [0u64; JMP_BUF_WORDS];
  // SAFETY: 同 real_signal_recovery; 跳回值 FELLBACK 来自 fallback 处理器
  let r = unsafe { _setjmp(buf.as_mut_ptr().cast()) };
  unsafe { JMP = buf.as_mut_ptr().cast() };
  if r == 0 {
    crash(page);
    unreachable!("处理器恢复失败, 故障重演未进入 fallback");
  }
  assert_eq!(r, FELLBACK);
  assert_eq!(get_recovered_trap_count(), 0);

  // 不接管路径已由 handle_signal 自动 remove_trap_handler (恢复了 fallback);
  // 测试尾部把该信号礼貌恢复默认处理器
  // SAFETY: sigaction 配置 API
  unsafe {
    let mut dfl: libc::sigaction = zeroed();
    dfl.sa_sigaction = libc::SIG_DFL;
    libc::sigaction(oob_sig(), &dfl, null_mut());
  }
  OK
}
