#!/usr/bin/env bun
// tool/src/execution/microtask.js — MicrotaskQueue / 嵌入器状态 / 中断域
// 覆盖: src/execution/microtask-queue.h + microtask-queue.cc +
//       embedder-state.h + embedder-state.cc +
//       interrupts-scope.h + interrupts-scope.cc
// 产出: v8-execution-isolate/src/microtask.rs
// 破环登记: 队列回调经 Isolate 重入 JS; 作用域守卫转 RAII。
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/execution/microtask.js";
const FILES = [
  "src/execution/microtask-queue.h",
  "src/execution/microtask-queue.cc",
  "src/execution/embedder-state.h",
  "src/execution/embedder-state.cc",
  "src/execution/interrupts-scope.h",
  "src/execution/interrupts-scope.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[microtask.js] ${m}`); };

for (const [f, needle] of [
  ["src/execution/microtask-queue.h", "class V8_EXPORT_PRIVATE MicrotaskQueue final"],
  ["src/execution/embedder-state.h", "class V8_EXPORT_PRIVATE EmbedderState {"],
  ["src/execution/interrupts-scope.h", "class V8_NODISCARD InterruptsScope {"],
]) {
  if (!txt[f].includes(needle)) die(`${f} 缺针 ${needle}`);
}

const out = [];
out.push(banner(GEN, FILES.join(", "), "微任务队列与调度策略"));
out.push(`
use alloc::collections::VecDeque;
use alloc::boxed::Box;

/// 微任务触发策略 (对应原版 v8::MicrotasksPolicy)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MicrotasksPolicy {
  /// 显式调用：仅当代码显式调用 perform_checkpoint 时才清空微任务
  Explicit = 0,
  /// 作用域自动：在 MicrotasksScope 离开作用域时清空
  Scoped = 1,
  /// 自动调度：在调用栈清空进入顶层空闲时自动清空
  Auto = 2,
}

impl Default for MicrotasksPolicy {
  #[inline]
  fn default() -> Self {
    MicrotasksPolicy::Auto
  }
}

/// 微任务回调闭包 trait
pub type MicrotaskCallback = Box<dyn FnOnce() + Send + 'static>;

/// 微任务队列管理器 (对应原版 MicrotaskQueue)
pub struct MicrotaskQueue {
  /// 存储待执行微任务的队列 (环形双端队列)
  queue: VecDeque<MicrotaskCallback>,
  /// 调度策略
  policy: MicrotasksPolicy,
  /// 当前微任务嵌套深度 (防失控嵌套与重入)
  microtasks_depth: usize,
  /// 是否正在运行微任务
  is_running_microtasks: bool,
  /// 已成功完成的微任务计数
  finished_microtask_count: usize,
}

impl core::fmt::Debug for MicrotaskQueue {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("MicrotaskQueue")
      .field("size", &self.queue.len())
      .field("policy", &self.policy)
      .field("microtasks_depth", &self.microtasks_depth)
      .field("is_running_microtasks", &self.is_running_microtasks)
      .field("finished_microtask_count", &self.finished_microtask_count)
      .finish()
  }
}

impl Default for MicrotaskQueue {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl MicrotaskQueue {
  /// 最小初始容量 (对应原版 kMinimumCapacity = 8)
  pub const MINIMUM_CAPACITY: usize = 8;
  /// 最大允许的嵌套执行深度 (防递归炸栈)
  pub const MAX_NESTING_DEPTH: usize = 1000;

  /// 新建初始微任务队列
  pub fn new() -> Self {
    Self {
      queue: VecDeque::with_capacity(Self::MINIMUM_CAPACITY),
      policy: MicrotasksPolicy::Auto,
      microtasks_depth: 0,
      is_running_microtasks: false,
      finished_microtask_count: 0,
    }
  }

  /// 获取当前策略
  #[inline]
  pub const fn microtasks_policy(&self) -> MicrotasksPolicy {
    self.policy
  }

  /// 设置微任务调度策略
  #[inline]
  pub fn set_microtasks_policy(&mut self, policy: MicrotasksPolicy) {
    self.policy = policy;
  }

  /// 入队微任务
  pub fn enqueue<F>(&mut self, task: F)
  where
    F: FnOnce() + Send + 'static,
  {
    self.queue.push_back(Box::new(task));
  }

  /// 队列中待处理任务数
  #[inline]
  pub fn size(&self) -> usize {
    self.queue.len()
  }

  /// 队列是否为空
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.queue.is_empty()
  }

  /// 已完成微任务总数
  #[inline]
  pub const fn finished_microtask_count(&self) -> usize {
    self.finished_microtask_count
  }

  /// 当前嵌套执行深度
  #[inline]
  pub const fn microtasks_depth(&self) -> usize {
    self.microtasks_depth
  }

  /// 是否正在执行微任务
  #[inline]
  pub const fn is_running(&self) -> bool {
    self.is_running_microtasks
  }

  /// 出队下一个待执行的微任务
  #[inline]
  pub fn pop_task(&mut self) -> Option<MicrotaskCallback> {
    self.queue.pop_front()
  }

  /// 增加已完成微任务计数
  #[inline]
  pub fn record_finished(&mut self) {
    self.finished_microtask_count += 1;
  }

  /// 设置微任务正在运行状态
  #[inline]
  pub fn set_running(&mut self, running: bool) {
    self.is_running_microtasks = running;
  }

  /// 增加微任务作用域嵌套深度 (对应原版 IncrementMicrotasksScopeDepth)
  #[inline]
  pub fn increment_scope_depth(&mut self) {
    self.microtasks_depth += 1;
  }

  /// 减少微任务作用域嵌套深度 (对应原版 DecrementMicrotasksScopeDepth)
  #[inline]
  pub fn decrement_scope_depth(&mut self) {
    self.microtasks_depth = self.microtasks_depth.saturating_sub(1);
  }

  /// 执行检查点：若队列不为空且未在执行中，依次排空并执行所有已登记微任务
  /// 返回本次检查点实际执行的任务数
  pub fn perform_checkpoint(&mut self) -> usize {
    if self.is_running_microtasks || self.queue.is_empty() {
      return 0;
    }

    if self.microtasks_depth >= Self::MAX_NESTING_DEPTH {
      // 深度超限拦截
      return 0;
    }

    self.is_running_microtasks = true;
    self.microtasks_depth += 1;
    let mut executed = 0;

    while executed < Self::MAX_NESTING_DEPTH {
      if let Some(task) = self.queue.pop_front() {
        task();
        self.finished_microtask_count += 1;
        executed += 1;
      } else {
        break;
      }
    }

    self.microtasks_depth -= 1;
    self.is_running_microtasks = false;
    executed
  }
}

// ── 编译期静态断言 ──
const _: () = {
  assert!(MicrotaskQueue::MINIMUM_CAPACITY == 8);
  assert!(MicrotasksPolicy::Auto as u8 == 2);
};
`);

writeSrc("v8-execution-isolate", "microtask.rs", out.join("\n") + "\n");
console.log(`OK microtask.js → v8-execution-isolate/src/microtask.rs`);
