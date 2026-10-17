// 规约测试: 对应 v8/test/unittests/wasm/trap-handler-native-unittest.cc 的
// posix 可移植部分 (TestTrapHandlerRecovery / TestReleaseHandlerData /
// TestCrashInWasmNoProtectedInstruction 的注册-查询语义);
// 信号真机触发 (JIT 代码 + 真实故障) 登记推迟, 此处验证数据面语义
#[cfg(unix)]
use std::{mem::zeroed, ptr::null};

use aok::{OK, Void};
use log::info;
use v8_trap_handler::{
  K_INVALID_INDEX, TrappingInstructionData, get_recovered_trap_count, is_accessed_memory_covered,
  is_fault_address_covered, register_covered_memory, register_default_trap_handler,
  register_handler_data, register_v8_sandbox, release_handler_data, remove_trap_handler,
  unregister_covered_memory, unregister_v8_sandbox,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn ins(offset: u32) -> TrappingInstructionData {
  TrappingInstructionData {
    instr_offset: offset,
  }
}

/// 注册后 PC 查询命中/未命中; 反注册后不再命中
/// (native-unittest TestTrapHandlerRecovery + TestReleaseHandlerData 语义)
#[test]
fn register_and_query() -> Void {
  let base = 0x1_0000;
  let idx = register_handler_data(base, 0x100, &[ins(8)]).unwrap();
  assert!(idx >= 0);

  // 命中: 范围内且偏移精确匹配受保护指令
  assert!(is_fault_address_covered(base + 8));
  // 范围内但非保护偏移 (TestCrashInWasmNoProtectedInstruction 语义)
  assert!(!is_fault_address_covered(base + 9));
  assert!(!is_fault_address_covered(base));
  // 半开区间: base-1 与 base+size 均界外
  assert!(!is_fault_address_covered(base - 1));
  assert!(!is_fault_address_covered(base + 0x100));

  // 反注册后同一 PC 不再命中 (TestReleaseHandlerData 语义)
  release_handler_data(idx);
  assert!(!is_fault_address_covered(base + 8));
  info!("register_and_query OK");
  OK
}

/// 多条受保护指令的表: 各自命中, 未登记者不命中; 空表则范围内永不命中
#[test]
fn multiple_trapping_instructions() -> Void {
  let base = 0x50_0000;
  let idx = register_handler_data(base, 0x20, &[ins(1), ins(2), ins(0x10)]).unwrap();
  assert!(is_fault_address_covered(base + 1));
  assert!(is_fault_address_covered(base + 2));
  assert!(is_fault_address_covered(base + 0x10));
  assert!(!is_fault_address_covered(base + 3));
  release_handler_data(idx);

  // 空指令表: 代码范围内也不接管 (无受保护指令)
  let idx = register_handler_data(base, 0x10, &[]).unwrap();
  assert!(!is_fault_address_covered(base + 1));
  release_handler_data(idx);
  OK
}

/// 自由链表 LIFO: 释放槽位被下一次注册复用, 槽内换新数据
#[test]
fn index_reuse() -> Void {
  let a = register_handler_data(0x2_0000, 0x10, &[ins(3)]).unwrap();
  let b = register_handler_data(0x3_0000, 0x10, &[ins(5)]).unwrap();
  let c = register_handler_data(0x4_0000, 0x10, &[ins(7)]).unwrap();
  release_handler_data(b);

  // 释放的 b 槽位被 d 复用
  let d = register_handler_data(0x5_0000, 0x10, &[ins(9)]).unwrap();
  assert_eq!(d, b);
  assert_ne!(a, c);

  // 新数据生效, 旧范围失效; 其余槽位不受影响
  assert!(is_fault_address_covered(0x5_0000 + 9));
  assert!(!is_fault_address_covered(0x3_0000 + 5));
  assert!(is_fault_address_covered(0x2_0000 + 3));
  assert!(is_fault_address_covered(0x4_0000 + 7));

  // 无效索引为 no-op (原版 kInvalidIndex 路径)
  release_handler_data(K_INVALID_INDEX);
  OK
}

/// 扩容路径 (kInitialCodeObjectSize=1024 → *2): 无空洞时索引连续分配,
/// 第 1025 个注册触发扩容; 扩容后释放中段槽仍 LIFO 精确复用
/// (handler-outside.cc:157-188 生长分支 + :223 释放链回接)
#[test]
fn table_growth_and_reuse() -> Void {
  const N: i32 = 1025;
  for k in 0..N {
    let base = 0x1_0000_0000usize + k as usize * 0x100;
    let idx = register_handler_data(base, 0x10, &[ins(1)]).unwrap();
    assert_eq!(idx, k); // 无空洞 → 顺序编号
  }
  assert!(is_fault_address_covered(0x1_0000_0000 + 1024 * 0x100 + 1));

  // 释放中段槽 100 → 下一次注册精确复用该槽 (LIFO)
  release_handler_data(100);
  let idx = register_handler_data(0x2_0000_0000, 0x10, &[ins(2)]).unwrap();
  assert_eq!(idx, 100);
  // 复用槽新数据生效, 旧数据失效
  assert!(is_fault_address_covered(0x2_0000_0000 + 2));
  assert!(!is_fault_address_covered(0x1_0000_0000 + 100 * 0x100 + 1));
  OK
}

/// 界外索引反注册为 no-op: 不 panic、不崩溃、不影响既有注册
/// (原版界外索引为 C 数组越界 UB, Rust 安全化语义固化;
/// 注意界内空槽 = 二次释放, debug 下按原版 TH_DCHECK 语义崩溃, 不属本例)
#[test]
fn release_out_of_range_noop() -> Void {
  let idx = register_handler_data(0x60_0000, 0x10, &[ins(1)]).unwrap();
  assert!(is_fault_address_covered(0x60_0000 + 1));
  // 初始表容量 1024, i32::MAX 必在界外
  release_handler_data(i32::MAX);
  assert!(is_fault_address_covered(0x60_0000 + 1)); // 误释放不破坏数据
  release_handler_data(idx);
  assert!(!is_fault_address_covered(0x60_0000 + 1));
  OK
}

/// 覆盖内存注册/反注册后的地址查询 (IsAccessedMemoryCovered 语义)
#[test]
fn covered_memory_query() -> Void {
  let base = 0x10_0000;
  register_covered_memory(base, 0x1000);
  assert!(is_accessed_memory_covered(base));
  assert!(is_accessed_memory_covered(base + 0xFFF));
  assert!(!is_accessed_memory_covered(base - 1));
  // 半开区间: base+size 界外
  assert!(!is_accessed_memory_covered(base + 0x1000));

  unregister_covered_memory(base, 0x1000);
  assert!(!is_accessed_memory_covered(base));
  OK
}

/// sandbox 约束: 注册 sandbox 后, 覆盖内存必须整体落在其中;
/// 查询粒度按覆盖内存判定
#[test]
fn sandbox_constraint() -> Void {
  register_v8_sandbox(0x20_0000, 0x1000);
  register_covered_memory(0x20_0800, 0x100);
  assert!(is_accessed_memory_covered(0x20_0800));
  // sandbox 内但未注册为覆盖内存 → 不命中
  assert!(!is_accessed_memory_covered(0x20_0A00));
  unregister_covered_memory(0x20_0800, 0x100);
  unregister_v8_sandbox(0x20_0000, 0x1000);

  // sandbox 移除后可注册任意区间
  register_covered_memory(0x30_0000, 0x100);
  assert!(is_accessed_memory_covered(0x30_0000));
  OK
}

/// 恢复计数: 命中 +1, 未命中不计 (GetRecoveredTrapCount 语义)
#[test]
fn recovered_count() -> Void {
  assert_eq!(get_recovered_trap_count(), 0);
  let base = 0x40_0000;
  let idx = register_handler_data(base, 0x10, &[ins(4)]).unwrap();
  assert!(is_fault_address_covered(base + 4));
  assert!(is_fault_address_covered(base + 4));
  assert_eq!(get_recovered_trap_count(), 2);
  assert!(!is_fault_address_covered(base + 5));
  assert_eq!(get_recovered_trap_count(), 2);
  release_handler_data(idx);
  OK
}

/// 信号安装/恢复 (handler-outside-posix RegisterDefaultTrapHandler /
/// RemoveTrapHandler; 真机故障触发集成见登记推迟项)
#[cfg(unix)]
#[test]
fn sigaction_install_and_restore() -> Void {
  let sig = if cfg!(target_os = "macos") {
    libc::SIGBUS
  } else {
    libc::SIGSEGV
  };
  // SAFETY: sigaction 查询/安装为配置 API, 非信号上下文
  unsafe {
    let mut before: libc::sigaction = zeroed();
    assert_eq!(libc::sigaction(sig, null(), &mut before), 0);
    let before_handler = before.sa_sigaction;

    assert!(register_default_trap_handler());
    let mut now: libc::sigaction = zeroed();
    assert_eq!(libc::sigaction(sig, null(), &mut now), 0);
    // 安装后处理器已换为 handle_signal
    assert_ne!(now.sa_sigaction, before_handler);

    remove_trap_handler();
    // 幂等: 已移除后再次 remove 为 no-op, 处理器保持旧值
    remove_trap_handler();
    let mut after: libc::sigaction = zeroed();
    assert_eq!(libc::sigaction(sig, null(), &mut after), 0);
    // 旧处理器恢复
    assert_eq!(after.sa_sigaction, before_handler);
  }
  OK
}

#[cfg(not(unix))]
#[test]
fn default_trap_handler_unsupported() -> Void {
  assert!(!register_default_trap_handler());
  remove_trap_handler();
  OK
}
