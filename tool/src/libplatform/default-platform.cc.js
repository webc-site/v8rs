#!/usr/bin/env bun
// tool/src/libplatform/default-platform.cc.js
// 机械提取 v8/src/libplatform/ 平台族: default-platform.{h,cc} + worker-thread.{h,cc} +
// task-queue.{h,cc} + default-thread-isolated-allocator.{h,cc} + etw/etw-provider-win.h

import { readV8, writeSrc, banner } from "../../lib.js";

const dpH = readV8("src/libplatform/default-platform.h");
const dpCc = readV8("src/libplatform/default-platform.cc");
const wtH = readV8("src/libplatform/worker-thread.h");
const wtCc = readV8("src/libplatform/worker-thread.cc");
const tqH = readV8("src/libplatform/task-queue.h");
const tqCc = readV8("src/libplatform/task-queue.cc");
const taH = readV8("src/libplatform/default-thread-isolated-allocator.h");
const taCc = readV8("src/libplatform/default-thread-isolated-allocator.cc");
const etw = readV8("src/libplatform/etw/etw-provider-win.h");

// 1. default-platform 面
const dpFns = [...dpH.matchAll(/\n  (?:static |virtual )?(?:void|int|double|std::unique_ptr|TaskRunner|TracingController|Platform)[\w:<>,*& ]*?(\w+)\(/g)].map((m) => m[1]);
if (dpFns.length < 8) throw new Error(`default-platform.h 方法应 ≥8, 实得 ${dpFns.length}`);
// cc 侧线程池上限 (原版 kMaxThreadPoolSize = 16)
const threadM = dpCc.match(/constexpr int kMaxThreadPoolSize = (\d+);/);
if (!threadM || threadM[1] !== "16") throw new Error("kMaxThreadPoolSize 漂移");
// 页分配器/时钟虚面
for (const k of ["PageAllocator", "MonotonicallyIncreasingTime", "TracingController"]) {
  if (!dpH.includes(k)) throw new Error(`default-platform.h 缺 ${k}`);
}

// 2. worker-thread / task-queue
for (const k of ["WorkerThread"]) {
  if (!wtH.includes(k)) throw new Error("worker-thread.h 缺 WorkerThread");
}
if (!wtCc.includes("WorkerThread::")) throw new Error("worker-thread.cc 缺实现");
for (const k of ["TaskQueue"]) {
  if (!tqH.includes(k)) throw new Error("task-queue.h 缺 TaskQueue");
}
if (!tqCc.includes("TaskQueue::")) throw new Error("task-queue.cc 缺实现");

// 3. 线程隔离分配器 + ETW (Windows 专用, 登记跳过)
if (!taH.includes("DefaultThreadIsolatedAllocator")) throw new Error("缺线程隔离分配器");
if (!etw.includes("EtwProvider") && !/EVENT_TRACE|TraceLogging/i.test(etw)) {
  throw new Error("etw-provider-win.h 面漂移");
}

let rust = banner(
  "src/libplatform/default-platform.cc.js",
  "src/libplatform/default-platform.{h,cc} + worker-thread.{h,cc} + task-queue.{h,cc} + default-thread-isolated-allocator.{h,cc} + etw/etw-provider-win.h",
  "默认平台族骨架",
);

rust += `
/// DefaultPlatform 方法面 (原版 default-platform.h, ${dpFns.length} 项, 顺序登记)
pub const PLATFORM_METHODS: [&str; ${dpFns.length}] = [${dpFns.map((f) => `"${f}"`).join(", ")}];

/// 支撑组件锚点
pub const PLATFORM_SUPPORTS: [&str; 4] = ["PageAllocator", "MonotonicallyIncreasingTime", "TracingController", "WorkerThreadsTaskRunner"];

pub mod platform_skel {
  /// 工作线程池上限 (原版 kMaxThreadPoolSize)
  pub const MAX_THREAD_POOL_SIZE: usize = 16;

  /// 工作线程池: 每线程一个 WorkerThread 消费 TaskQueue
  pub fn worker_pool_skel(_threads: usize) {
    unimplemented!("骨架")
  }
  /// 任务队列终止协议 (原版 TaskQueue: 终止哨兵 + 条件变量)
  pub fn task_queue_terminate_skel() {
    unimplemented!("骨架")
  }
  /// 线程隔离分配器 (原版 DefaultThreadIsolatedAllocator; PKEY 隔离内存)
  pub fn thread_isolated_allocator_skel() {
    unimplemented!("骨架")
  }
}

/// 平台分支登记: ETW 提供者 (原版 etw/etw-provider-win.h) — 仅 Windows, 当前目标跳过
pub const ETW_PROVIDER: &str = "windows-only; X_SDK ETW TraceLogging, 本目标跳过";
`;

const p = writeSrc("v8-libplatform", "default_platform_generated.rs", rust);
console.log(`written ${p}; 平台方法=${dpFns.length}`);
