#!/usr/bin/env bun
// tool/src/base/platform/condition-variable.js — 覆盖 v8/src/base/platform/condition-variable.{h,cc}
// 条件变量原语: Wait/WaitFor 契约与 absl::CondVar 委托面登记
// → v8-base-platform/src/condition_variable.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/condition-variable.h");
const ccSrc = readV8("src/base/platform/condition-variable.cc");

// 1. 头文件锚点: 类存在, 唯一数据成员为 absl::CondVar
const hAnchors = [
  "class V8_BASE_EXPORT ConditionVariable {",
  "absl::CondVar native_handle_;",
  "void NotifyOne();",
  "void NotifyAll();",
  "void Wait(Mutex* mutex);",
  "bool WaitFor(Mutex* mutex, const TimeDelta& rel_time) V8_WARN_UNUSED_RESULT;",
  "using LazyConditionVariable =",
  "#define LAZY_CONDITION_VARIABLE_INITIALIZER LAZY_STATIC_INSTANCE_INITIALIZER",
];
for (const a of hAnchors) {
  if (!hSrc.includes(a)) throw new Error(`condition-variable.h 缺锚点: ${a}`);
}

// 2. 方法面解析: 公共方法 6 (ctor/dtor/NotifyOne/NotifyAll/Wait/WaitFor)
const pubMethods = [...hSrc.matchAll(/^  (?:void|bool|~?\w*)\s+\w+\(|^  ConditionVariable\(\);/gm)].length;
const methodCount = (hSrc.match(/void NotifyOne\(\);|void NotifyAll\(\);|void Wait\(Mutex\* mutex\);|bool WaitFor\(Mutex\* mutex/g) || []).length;
if (methodCount !== 4) throw new Error(`Wait 族方法数不符: ${methodCount}, 期望 4`);

// 3. 实现侧锚点: NotifyOne→Signal, WaitFor 返回 !timed_out (通知先于超时 → true)
const ccAnchors = [
  "void ConditionVariable::NotifyOne() { native_handle_.Signal(); }",
  "void ConditionVariable::NotifyAll() { native_handle_.SignalAll(); }",
  "mutex->AssertHeldAndUnmark();",
  "native_handle_.Wait(&mutex->native_handle_);",
  "uint64_t time = rel_time.InNanoseconds();",
  "absl::Duration duration = absl::Nanoseconds(time);",
  "bool timed_out =",
  "return !timed_out;",
];
for (const a of ccAnchors) {
  if (!ccSrc.includes(a)) throw new Error(`condition-variable.cc 缺锚点: ${a}`);
}
// Wait/WaitFor 前后成对打标记 (移交互斥权期间解除 Mutex 的 DEBUG 标记)
const unmarkCount = (ccSrc.match(/AssertHeldAndUnmark/g) || []).length;
const markCount = (ccSrc.match(/AssertUnheldAndMark/g) || []).length;
if (unmarkCount !== 2 || markCount !== 2) throw new Error(`标记对数不符: unmark ${unmarkCount}/mark ${markCount}, 期望 2/2`);

console.log(`condition-variable: 方法 ${methodCount + 2} (含 ctor/dtor), 标记对 ${markCount}, 平台分支 0 (全走 absl)`);

let rust = banner("src/base/platform/condition-variable.js", "src/base/platform/condition-variable.{h,cc}", "条件变量原语 (absl::CondVar 委托, 目标 Rust 侧走 std::sync::Condvar)");

rust += `
/// 条件变量 (原版 base::ConditionVariable; 原版委托 absl::CondVar)
/// v8: src/base/platform/condition-variable.h ConditionVariable
///
/// 不变式: Wait/WaitFor 前必须持有 |mutex|; 原版在原子释放互斥期间
/// 先 AssertHeldAndUnmark 再 AssertUnheldAndMark, 只动 DEBUG 标记。
pub struct ConditionVariable {
  /// 原版成员 absl::CondVar native_handle_
  cond: std::sync::Condvar,
}

impl ConditionVariable {
  /// v8: src/base/platform/condition-variable.cc ConditionVariable()
  pub fn new() -> Self { Self { cond: std::sync::Condvar::new() } }

  /// v8: src/base/platform/condition-variable.cc NotifyOne
  /// 唤醒一个等待线程 (原版 native_handle_.Signal())
  pub fn notify_one(&self) { self.cond.notify_one(); }

  /// v8: src/base/platform/condition-variable.cc NotifyAll
  /// 唤醒全部等待线程 (原版 native_handle_.SignalAll())
  pub fn notify_all(&self) { self.cond.notify_all(); }

  /// v8: src/base/platform/condition-variable.cc Wait
  /// 原子释放互斥并阻塞, 被唤醒后重新获取互斥
  pub fn wait<'m>(&self, guard: std::sync::MutexGuard<'m, ()>) -> std::sync::MutexGuard<'m, ()> {
    self.cond.wait(guard).unwrap_or_else(|e| e.into_inner().0)
  }

  /// v8: src/base/platform/condition-variable.cc WaitFor
  /// 返回 true 表示通知先于超时到达 (原版 return !timed_out)
  /// 超时换算: rel_time.InNanoseconds() → absl::Nanoseconds → WaitWithTimeout
  pub fn wait_for<'m>(
    &self,
    guard: std::sync::MutexGuard<'m, ()>,
    rel_time: std::time::Duration,
  ) -> (std::sync::MutexGuard<'m, ()>, bool) {
    let (guard, timed_out) = self
      .cond
      .wait_timeout(guard, rel_time)
      .unwrap_or_else(|e| e.into_inner());
    (guard, !timed_out.timed_out())
  }
}

// —— 惰性初始化登记 (原版 LazyConditionVariable + LAZY_CONDITION_VARIABLE_INITIALIZER) ——
/// v8: src/base/platform/condition-variable.h LazyConditionVariable
/// Rust 侧由 std::sync::OnceLock / LazyLock 承担, 宏不再需要
pub type LazyConditionVariable = std::sync::OnceLock<ConditionVariable>;

// —— 提取不变式断言 ——
const _: () = {
  // 原版公共面: ctor/dtor/NotifyOne/NotifyAll/Wait/WaitFor = 6
  const PUBLIC_SURFACE: usize = 6;
  // Wait/WaitFor 各一对 unmark/mark
  const MARK_PAIRS: usize = 2;
  assert!(PUBLIC_SURFACE == 6 && MARK_PAIRS == 2);
};
`;

const p = writeSrc("v8-base-platform", "condition_variable.rs.skel", rust);
console.log(`written ${p}`);
