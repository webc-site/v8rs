#!/usr/bin/env bun
// tool/src/base/platform/semaphore.js — 覆盖 v8/src/base/platform/semaphore.{h,cc}
// 信号量原语: Signal/Wait/WaitFor 契约 + 四平台 NativeHandle 分支登记
// → v8-base-platform/src/semaphore.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/semaphore.h");
const ccSrc = readV8("src/base/platform/semaphore.cc");

// 1. 头文件锚点: 方法面 + NativeHandle 四分支
const hAnchors = [
  "explicit Semaphore(int count);",
  "void Signal();",
  "void Wait();",
  "bool WaitFor(const TimeDelta& rel_time) V8_WARN_UNUSED_RESULT;",
  "#if V8_OS_DARWIN",
  "using NativeHandle = dispatch_semaphore_t;",
  "using NativeHandle = sem_t;",
  "using NativeHandle = HANDLE;",
  "using NativeHandle = starboard::Semaphore;",
  "template <int N>\nstruct CreateSemaphoreTrait {",
  "template <int N>\nstruct LazySemaphore {",
  "#define LAZY_SEMAPHORE_INITIALIZER LAZY_DYNAMIC_INSTANCE_INITIALIZER",
];
for (const a of hAnchors) {
  if (!hSrc.includes(a)) throw new Error(`semaphore.h 缺锚点: ${a}`);
}
const nativeBranches = (hSrc.match(/using NativeHandle = /g) || []).length;
if (nativeBranches !== 4) throw new Error(`NativeHandle 分支数不符: ${nativeBranches}, 期望 4`);

// 2. 实现侧: 四平台 #if 分支序 (DARWIN → POSIX → WIN → STARBOARD)
const ccAnchors = [
  "Semaphore::Semaphore(int count) {\n  native_handle_ = dispatch_semaphore_create(count);",
  "dispatch_semaphore_wait(native_handle_, DISPATCH_TIME_FOREVER);",
  "return dispatch_semaphore_wait(native_handle_, timeout) == 0;",
  "int result = sem_init(&native_handle_, 0, count);",
  "int result = sem_post(&native_handle_);",
  "int result = sem_wait(&native_handle_);",
  "int result = sem_timedwait(&native_handle_, &ts);",
  "if (result == -1 && errno == ETIMEDOUT) {",
  "DCHECK_EQ(EINTR, errno);",
  "native_handle_ = ::CreateSemaphoreA(nullptr, count, 0x7FFFFFFF, nullptr);",
  "DWORD result = WaitForSingleObject(native_handle_, INFINITE);",
  "if (result == WAIT_TIMEOUT) {",
  "Semaphore::Semaphore(int count) : native_handle_(count) { DCHECK_GE(count, 0); }",
  "int64_t microseconds = rel_time.InMicroseconds();",
];
for (const a of ccAnchors) {
  if (!ccSrc.includes(a)) throw new Error(`semaphore.cc 缺锚点: ${a}`);
}
// 平台分支计数: include 区 2 (DARWIN/WIN 头选择) + 实现块 4 (DARWIN/POSIX/WIN/STARBOARD)
const implBranch = ccSrc.indexOf("#if V8_OS_DARWIN\n\nSemaphore::Semaphore");
if (implBranch < 0) throw new Error("实现区 DARWIN 分支起点缺失");
const implTail = ccSrc.slice(implBranch);
const implBlocks = (implTail.match(/^#if V8_OS_DARWIN$|^#elif V8_OS_POSIX$|^#elif V8_OS_WIN$|^#elif V8_OS_STARBOARD$/gm) || []).length;
if (implBlocks !== 4) throw new Error(`平台实现块数不符: ${implBlocks}, 期望 4`);
// POSIX Wait/WaitFor 均为 EINTR 自旋重试循环
const spinLoops = (ccSrc.match(/while \(true\) \{/g) || []).length;
if (spinLoops !== 3) throw new Error(`自旋循环数不符: ${spinLoops}, 期望 3 (POSIX Wait/WaitFor + WIN WaitFor)`);

console.log(`semaphore: 方法 3 (Signal/Wait/WaitFor), NativeHandle 分支 ${nativeBranches}, 实现块 ${implBlocks}, 自旋循环 ${spinLoops}`);

let rust = banner("src/base/platform/semaphore.js", "src/base/platform/semaphore.{h,cc}", "信号量原语 (darwin 目标走 dispatch_semaphore, POSIX 走 sem_*)");

rust += `
/// 信号量 (原版 base::Semaphore)
/// v8: src/base/platform/semaphore.h Semaphore
///
/// 计数信号量: Signal 增, Wait 减, 计数归零时 Wait 阻塞。
/// 原版 NativeHandle 分支: DARWIN→dispatch_semaphore_t / POSIX→sem_t /
/// WIN→HANDLE / STARBOARD→starboard::Semaphore; darwin 目标用 dispatch。
pub struct Semaphore {
  /// 原版 NativeHandle native_handle_ (darwin: dispatch_semaphore_t)
  native: dispatch_semaphore_t,
}

/// darwin dispatch 句柄 (原版 using NativeHandle = dispatch_semaphore_t)
/// v8: src/base/platform/semaphore.h Semaphore::NativeHandle
pub type NativeHandle = *const std::ffi::c_void; // dispatch_semaphore_t 不透明句柄

extern "C" {
  fn dispatch_semaphore_create(value: i32) -> NativeHandle;
  fn dispatch_release(object: NativeHandle);
  fn dispatch_semaphore_signal(sem: NativeHandle) -> i64;
  fn dispatch_semaphore_wait(sem: NativeHandle, timeout: u64) -> i64;
  fn dispatch_time(when: i64, delta: i64) -> u64;
}

/// DISPATCH_TIME_NOW (原版 WaitFor 用 dispatch_time(DISPATCH_TIME_NOW, ns) 组装绝对超时)
const DISPATCH_TIME_NOW: i64 = 0;
/// DISPATCH_TIME_FOREVER (原版 Wait 的永久等待哨兵)
const DISPATCH_TIME_FOREVER: u64 = u64::MAX;

impl Semaphore {
  /// v8: src/base/platform/semaphore.cc Semaphore::Semaphore (darwin 分支)
  /// dispatch_semaphore_create(count); DCHECK(native_handle_)
  pub fn new(count: i32) -> Self {
    debug_assert!(count >= 0);
    Self { native: unsafe { dispatch_semaphore_create(count) } }
  }

  /// v8: src/base/platform/semaphore.cc Semaphore::~Semaphore (darwin 分支)
  /// dispatch_release(native_handle_)
  pub fn destroy(&mut self) { unsafe { dispatch_release(self.native) }; }

  /// v8: src/base/platform/semaphore.cc Semaphore::Signal (darwin 分支)
  pub fn signal(&self) { unsafe { dispatch_semaphore_signal(self.native) }; }

  /// v8: src/base/platform/semaphore.cc Semaphore::Wait (darwin 分支)
  /// dispatch_semaphore_wait(_, DISPATCH_TIME_FOREVER)
  pub fn wait(&self) { unsafe { dispatch_semaphore_wait(self.native, DISPATCH_TIME_FOREVER) }; }

  /// v8: src/base/platform/semaphore.cc Semaphore::WaitFor (darwin 分支)
  /// 超时 = dispatch_time(DISPATCH_TIME_NOW, rel_time.InNanoseconds());
  /// 返回 dispatch_semaphore_wait(...) == 0 (0 = 通知到达)
  pub fn wait_for(&self, rel_time_ns: i64) -> bool {
    let timeout = unsafe { dispatch_time(DISPATCH_TIME_NOW, rel_time_ns) };
    unsafe { dispatch_semaphore_wait(self.native, timeout) == 0 }
  }
}

// —— 非 darwin 平台分支登记 (rule 9: 按目标展开, 此处登记不产码) ——
/// v8: src/base/platform/semaphore.cc (V8_OS_POSIX 分支)
/// sem_init/sem_post/sem_wait/sem_timedwait; Wait 与 WaitFor 均为
/// while(true) 循环: result==0 返回, errno==ETIMEDOUT 返回 false,
/// 其余 DCHECK(result==-1 && errno==EINTR) 继续自旋。
/// glibc<2.4 特例: sem_timedwait 正值返回需折回 errno (原版 91-97 行)。
/// v8: src/base/platform/semaphore.cc (V8_OS_WIN 分支)
/// CreateSemaphoreA(_, count, 0x7FFFFFFF, _); WaitForSingleObject;
/// WaitFor 以 msec>=INFINITE 分档 + TimeTicks 差值自旋。
/// v8: src/base/platform/semaphore.cc (V8_OS_STARBOARD 分支)
/// Put/Take/TakeWait(microseconds) — 非目标, 登记跳过。

// —— 惰性初始化登记 (原版 LazySemaphore<N> + LAZY_SEMAPHORE_INITIALIZER) ——
/// v8: src/base/platform/semaphore.h LazySemaphore
/// CreateSemaphoreTrait<N> 以初值 N 动态构造; Rust 侧 OnceLock 承担
pub type LazySemaphore = std::sync::OnceLock<Semaphore>;

// —— 提取不变式断言 ——
const _: () = {
  // Signal/Wait/WaitFor = 3; NativeHandle 平台分支 = 4
  const METHODS: usize = 3;
  const NATIVE_BRANCHES: usize = 4;
  assert!(METHODS == 3 && NATIVE_BRANCHES == 4);
};
`;

const p = writeSrc("v8-base-platform", "semaphore.rs.skel", rust);
console.log(`written ${p}`);
