//! RegExpStack 族规约测试 (v8-regexp-exec)。
//!
//! 规约来源 (实测枚举 v8/test/unittests/regexp/regexp-unittest.cc, stack 相关用例):
//! - TEST_F(RegExpTest, MacroAssemblerStackOverflow) (:1306) — GrowStack 经 isolate+native
//!   代码生成路径, 本席无 codegen → 跳过, 推迟登记 (REG-1b/codegen 集成后迁)
//! - TEST_F(RegExpTestWithContext, RegExpInterruptReentrantExecution) (:2804) 与
//!   RegExpInterruptStalePrologueBacktrackPointer (:2945) — 依赖 Isolate RequestInterrupt/
//!   同上推迟登记 (二者对 Stack 的断言面: EnsureCapacity 增长搬移 + sp 经
//!   thread_local 直写, 本文件以纯 API + offset 注资等价复刻)
//!   落地为 regexp-stack.{h,cc} 不变式 1:1 用例: 静态区初始态 / 容量钳制与上限 /
//!   增长顶拷贝复用 / limit+slack / sp_top_delta 连续性 / StackScope reset /
//!   归档-恢复线程交接 / 归档强制转动态 / 线程资源释放。

use std::mem::size_of;

use aok::{OK, Void};
use v8_base::memory::Address;
use v8_common::{K_NULL_ADDRESS, KB};
use v8_regexp_exec::regexp_stack::{
  K_LIMIT_OFFSET, K_MAXIMUM_STACK_SIZE, K_MEMORY_TOP_OFFSET, K_SLOT_SIZE, K_STACK_LIMIT_SLACK_SIZE,
  K_STACK_LIMIT_SLACK_SLOT_COUNT, K_STACK_POINTER_OFFSET, K_THREAD_LOCAL_SIZE, Stack, StackScope,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 经 thread_local_address + offset 读写 ThreadLocal 块 (原版生成码 friend 访存面复刻;
/// 槽宽 = lp64 Address/usize)
unsafe fn tl_read(stack: &Stack, offset: usize) -> Address {
  // SAFETY: offset 为编译期布局常量, 块基址由 thread_local_address 提供;
  // 单线程测试内线程独占, 无并发写
  unsafe { ((stack.thread_local_address() + offset) as *const Address).read_unaligned() }
}

unsafe fn tl_write(stack: &Stack, offset: usize, value: Address) {
  // SAFETY: 同上, 写权来自测试线程独占 &Stack (等价原版字段直写)
  unsafe { ((stack.thread_local_address() + offset) as *mut Address).write_unaligned(value) };
}

/// 原版常量金标准 (h:57-126, 默认非 PPC64/S390X 配置)
#[test]
fn constants_golden() -> Void {
  assert_eq!(K_SLOT_SIZE, 4);
  assert_eq!(K_STACK_LIMIT_SLACK_SLOT_COUNT, 32);
  assert_eq!(
    K_STACK_LIMIT_SLACK_SIZE,
    K_STACK_LIMIT_SLACK_SLOT_COUNT * K_SLOT_SIZE
  );
  assert_eq!(K_MAXIMUM_STACK_SIZE, 64 * 1024 * 1024);
  // lp64 实测 kThreadLocalSize: 4 指针 + size_t + Address + bool → 48B
  assert_eq!(K_THREAD_LOCAL_SIZE, 48);
  assert_eq!(K_THREAD_LOCAL_SIZE, 6 * size_of::<Address>());
  // 字段序 offset 金标准 (memory_/memory_top_/memory_size_/stack_pointer_/limit_/owns_memory_)
  assert_eq!(K_MEMORY_TOP_OFFSET, 8);
  assert_eq!(K_STACK_POINTER_OFFSET, 24);
  assert_eq!(K_LIMIT_OFFSET, 32);
  OK
}

/// 初始态 = 静态预留区 (h:110-123: 栈恒可用, sp 顶对齐)
#[test]
fn initial_static_area() -> Void {
  let stack = Stack::new();
  assert_eq!(stack.memory_size(), KB);
  assert_eq!(stack.begin() + KB, stack.end());
  assert_eq!(stack.stack_pointer(), stack.end());
  assert_eq!(stack.memory_top(), stack.end());
  // limit = 基址 + slack (cc:89-90)
  assert_eq!(
    unsafe { tl_read(&stack, K_LIMIT_OFFSET) },
    stack.begin() + K_STACK_LIMIT_SLACK_SIZE
  );
  OK
}

/// EnsureCapacity: 0/小值钳制到最小动态栈, 超 64MB 返回 kNullAddress (cc:142-163)
#[test]
fn ensure_capacity_clamps_and_limit() -> Void {
  let stack = Stack::new();
  // size=0 → 默认缓冲 (静态区), 不重分配
  assert_eq!(stack.ensure_capacity(0), stack.end());
  assert_eq!(stack.memory_size(), KB);
  // 上限外 → kNullAddress, 状态不变
  assert_eq!(
    stack.ensure_capacity(K_MAXIMUM_STACK_SIZE + 1),
    K_NULL_ADDRESS
  );
  assert_eq!(stack.memory_size(), KB);
  // 1500B < 2KB 最小动态栈 → 钳 2048; 空栈 (sp==顶)
  let top = stack.ensure_capacity(1500);
  assert_eq!(stack.memory_size(), 2 * KB);
  assert_eq!(stack.begin() + 2 * KB, top);
  assert_eq!(stack.memory_top(), top);
  assert_eq!(stack.stack_pointer(), top);
  // limit 重置为新基址 + slack (cc:158-159)
  assert_eq!(
    unsafe { tl_read(&stack, K_LIMIT_OFFSET) },
    stack.begin() + K_STACK_LIMIT_SLACK_SIZE
  );
  OK
}

/// 增长复用: 旧内容整体搬新缓冲顶部, sp_top_delta 连续 (cc:147-157)
#[test]
fn grow_copies_content_to_top() -> Void {
  let stack = Stack::new();
  stack.ensure_capacity(2 * KB);
  let top1 = stack.end();
  let slot = size_of::<Address>();
  // 模拟两次 push: 栈自顶向下写槽 (原版生成码 sp 直写面复刻)
  let a: Address = 0xDEAD_BEEF;
  let b: Address = 0xCAFE_F00D;
  // SAFETY: top1 起 2KB 有效区内低地址槽 (Box 分配), 单线程独占
  unsafe {
    ((top1 - slot) as *mut Address).write(a);
    ((top1 - 2 * slot) as *mut Address).write(b);
    tl_write(&stack, K_STACK_POINTER_OFFSET, top1 - 2 * slot);
  }
  assert_eq!(stack.stack_pointer(), top1 - 2 * slot);

  // 4KB 增长: delta 保持, 内容随旧区整体顶对齐搬迁
  let top2 = stack.ensure_capacity(4 * KB);
  assert_eq!(stack.memory_size(), 4 * KB);
  assert_eq!(stack.begin() + 4 * KB, top2);
  assert_eq!(stack.stack_pointer(), top2 - 2 * slot);
  // SAFETY: 新缓冲顶部两槽刚被顶拷贝填充
  let got_a = unsafe { ((top2 - slot) as *const Address).read_unaligned() };
  let got_b = unsafe { ((top2 - 2 * slot) as *const Address).read_unaligned() };
  assert_eq!(got_a, a);
  assert_eq!(got_b, b);
  OK
}

/// StackScope 空栈回落: 动态区 drop 时 reset 回静态 (cc:20-23 + h:189)
#[test]
fn scope_resets_empty_dynamic() -> Void {
  let stack = Stack::new();
  let static_begin = stack.begin();
  {
    let scope = StackScope::new(&stack);
    scope.stack().ensure_capacity(2 * KB);
    // 未 push (sp==顶) → 作用域结束回落静态区并释放动态缓冲
    assert_ne!(stack.begin(), static_begin);
  }
  assert_eq!(stack.memory_size(), KB);
  assert_eq!(stack.begin(), static_begin);
  assert_eq!(stack.stack_pointer(), stack.end());
  OK
}

/// StackScope 非空保留 + free_thread_resources 释放 (h:95 / cc:82-92)
#[test]
fn scope_keeps_nonempty_and_free() -> Void {
  let stack = Stack::new();
  stack.ensure_capacity(2 * KB);
  let top = stack.end();
  unsafe { tl_write(&stack, K_STACK_POINTER_OFFSET, top - size_of::<Address>()) };
  {
    // delta 基线入域, 出域 CHECK_EQ 通过且非空 → 保留动态区
    let _scope = StackScope::new(&stack);
  }
  assert_eq!(stack.memory_size(), 2 * KB);
  assert_eq!(stack.stack_pointer(), top - size_of::<Address>());
  // 线程资源释放: 回落静态区
  stack.free_thread_resources();
  assert_eq!(stack.memory_size(), KB);
  assert_eq!(stack.stack_pointer(), stack.end());
  OK
}

/// 归档/恢复: 线程交接 ThreadLocal 块, 动态缓冲所有权随迁 (cc:62-80)
#[test]
fn archive_restore_roundtrip() -> Void {
  let mut buf = vec![0u8; Stack::archive_space_per_thread() as usize];
  assert_eq!(buf.len(), K_THREAD_LOCAL_SIZE);
  let a = Stack::new();
  a.ensure_capacity(4 * KB);
  let payload: Address = 0x5A5A_1234;
  // SAFETY: a 的动态缓冲顶槽, 单线程独占
  unsafe { ((a.end() - size_of::<Address>()) as *mut Address).write(payload) };

  // 归档 (已持动态栈): 块字节写入 buf, a 回落静态区
  let used_end = unsafe { a.archive_stack(buf.as_mut_ptr()) };
  assert_eq!(
    used_end as usize - buf.as_ptr() as usize,
    K_THREAD_LOCAL_SIZE
  );
  assert_eq!(a.memory_size(), KB);
  // 回落不变式: memory_top_ = memory_ + memory_size_ (h:136-139)
  assert_eq!(
    unsafe { tl_read(&a, K_MEMORY_TOP_OFFSET) },
    a.begin() + a.memory_size()
  );

  // 恢复: 另一栈实例 (模拟新线程) 接移动态缓冲与其内容
  let b = Stack::new();
  let consumed = unsafe { b.restore_stack(buf.as_mut_ptr()) };
  assert_eq!(
    consumed as usize - buf.as_ptr() as usize,
    K_THREAD_LOCAL_SIZE
  );
  assert_eq!(b.memory_size(), 4 * KB);
  assert_eq!(b.begin() + 4 * KB, b.end());
  // SAFETY: 恢复后顶部槽为 a 归档前写入的 payload
  let got = unsafe { ((b.end() - size_of::<Address>()) as *const Address).read_unaligned() };
  assert_eq!(got, payload);
  // 交接方释放 (b 持 owns_memory_ 归还原缓冲)
  b.free_thread_resources();
  assert_eq!(b.memory_size(), KB);
  OK
}

/// 归档强制转动态: 静态区不可归档 (cc:63-70), archive 前须增长
#[test]
fn archive_forces_dynamic_stack() -> Void {
  let mut buf = vec![0u8; K_THREAD_LOCAL_SIZE];
  let a = Stack::new();
  // 静态态直接归档: EnsureCapacity(size+1) 强制钳到最小动态栈 (2KB) 后归档
  let used_end = unsafe { a.archive_stack(buf.as_mut_ptr()) };
  assert_eq!(
    used_end as usize,
    buf.as_ptr() as usize + K_THREAD_LOCAL_SIZE
  );
  assert_eq!(a.memory_size(), KB);
  // 恢复侧见动态尺寸与不变式 (memory_top_ = memory_ + memory_size_)
  let restored = Stack::new();
  unsafe { restored.restore_stack(buf.as_mut_ptr()) };
  assert_eq!(restored.memory_size(), 2 * KB);
  assert_eq!(
    unsafe { tl_read(&restored, K_MEMORY_TOP_OFFSET) },
    restored.begin() + 2 * KB
  );
  OK
}
