//! stack API 语义测试 (纯栈抽象; 原版用例在 heap 层, 此处按迭代协议补)。
use aok::{OK, Void};
use v8_base_platform::Stack as PlatformStack;
use v8_heap_base::{Stack, StackSegment, StackVisitor, is_on_current_stack};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

struct Collector {
  addrs: Vec<usize>,
}

impl StackVisitor for Collector {
  fn visit_pointer(&mut self, address: usize) {
    self.addrs.push(address);
  }
}

// Segment::contains: start >= address >= top (原 Contains)
#[test]
fn segment_contains() -> Void {
  let seg = StackSegment::new(1000, 100);
  assert!(seg.contains(1000));
  assert!(seg.contains(100));
  assert!(seg.contains(555));
  assert!(!seg.contains(1001));
  assert!(!seg.contains(99));
  // top=0 (未设置) 时任何地址 >= 0 均在 [top, start] 内
  let unset = StackSegment::new(1000, 0);
  assert!(unset.contains(0));
  assert!(!unset.contains(1001));
  OK
}

// 栈起点/标记生命周期: iterate_pointers_for_testing 内部设 marker, 结束后复原
#[test]
fn marker_and_iterate() -> Void {
  let mut stack = Stack::new();
  assert!(!stack.is_marker_set());
  stack.set_stack_start(PlatformStack::get_stack_start().value());
  assert_eq!(
    stack.stack_start(),
    PlatformStack::get_stack_start().value()
  );

  let mut collector = Collector { addrs: Vec::new() };
  stack.iterate_pointers_for_testing(&mut collector).unwrap();
  // 扫描区间横跨 main 帧与 env/argv 区, 必然读到非零字
  assert!(!collector.addrs.is_empty());
  assert!(collector.addrs.iter().all(|a| {
    let top = PlatformStack::get_current_stack_position().value();
    // 各槽值非零 (null 已跳过)
    *a != 0 && top > 0
  }));
  // marker 复原
  assert!(!stack.is_marker_set());
  OK
}

// 未设栈起点 → 迭代报错 (原 CHECK(start 非空) → Err)
#[test]
fn iterate_without_start_errors() -> Void {
  let stack = Stack::new();
  let mut collector = Collector { addrs: Vec::new() };
  assert!(stack.iterate_pointers_until_marker(&mut collector).is_err());
  OK
}

// 标记未设 (top=0) → 迭代报错 (原 CHECK(top 非空) → Err)
#[test]
fn iterate_without_marker_errors() -> Void {
  let mut stack = Stack::new();
  stack.set_stack_start(PlatformStack::get_stack_start().value());
  let mut collector = Collector { addrs: Vec::new() };
  assert!(stack.iterate_pointers_until_marker(&mut collector).is_err());
  OK
}

// IsOnStack / IsOnCurrentStack: 当前帧局部变量在栈上
#[test]
fn is_on_stack_current_frame() -> Void {
  let local: u64 = 0;
  let addr = &local as *const u64 as usize;
  assert!(Stack::is_on_stack(addr));
  assert!(is_on_current_stack(addr));
  // 栈起点之上 (更高地址) 不在栈内
  let above = PlatformStack::get_stack_start().value() + 8;
  assert!(!Stack::is_on_stack(above));
  OK
}

// 后台线程标记生命周期: 回调后注销 (原 SetMarkerForBackgroundThreadAndCallbackImpl)
#[test]
fn background_marker_lifecycle() -> Void {
  let mut stack = Stack::new();
  assert!(!stack.has_background_stacks());
  stack.set_marker_for_background_thread_and_callback(7, || {});
  // 无先前段 → 回调后注销
  assert!(!stack.is_marker_set_for_background_thread(7));
  assert!(!stack.has_background_stacks());
  // 空栈后台迭代无异常
  let mut collector = Collector { addrs: Vec::new() };
  stack.iterate_background_stacks(&mut collector).unwrap();
  assert!(collector.addrs.is_empty());
  OK
}
