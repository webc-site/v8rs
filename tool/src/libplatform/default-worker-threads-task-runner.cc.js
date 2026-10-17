#!/usr/bin/env bun
// tool/src/libplatform/default-worker-threads-task-runner.cc.js
// 机械提取 v8/src/libplatform/ 任务运行器族: default-worker-threads-task-runner.{h,cc} +
// delayed-task-queue.{h,cc} + default-foreground-task-runner.{h,cc} + default-job.{h,cc}

import { readV8, writeSrc, banner } from "../../lib.js";

const dwH = readV8("src/libplatform/default-worker-threads-task-runner.h");
const dwCc = readV8("src/libplatform/default-worker-threads-task-runner.cc");
const dtH = readV8("src/libplatform/delayed-task-queue.h");
const dtCc = readV8("src/libplatform/delayed-task-queue.cc");
const dfH = readV8("src/libplatform/default-foreground-task-runner.h");
const dfCc = readV8("src/libplatform/default-foreground-task-runner.cc");
const djH = readV8("src/libplatform/default-job.h");
const djCc = readV8("src/libplatform/default-job.cc");

// 1. 运行器面
const dwFns = [...dwH.matchAll(/\n  (?:void|double|std::unique_ptr|bool|bool\*)[\w:<>,*& ]*?(\w+)\(/g)].map((m) => m[1]);
if (dwFns.length < 4) throw new Error(`worker 运行器方法应 ≥4, 实得 ${dwFns.length}`);
for (const k of ["DelayedTaskQueue"]) {
  if (!dtH.includes(k)) throw new Error("delayed-task-queue.h 缺 DelayedTaskQueue");
}
if (!dtCc.includes("DelayedTaskQueue::")) throw new Error("delayed-task-queue.cc 缺实现");
// 前台运行器: 嵌套消息泵控制
const nestedFlags = [...new Set([...dfH.matchAll(/(pending_\w+|nesting_\w+|task_queue_)/g)].map((m) => m[1]))];
if (nestedFlags.length < 2) throw new Error(`前台运行器控制字段应 ≥2, 实得 ${nestedFlags.join(",")}`);
if (!dfCc.includes("DefaultForegroundTaskRunner::")) throw new Error("前台运行器 cc 缺实现");

// 2. job 协作面
for (const k of ["DefaultJobState", "DefaultJobHandle"]) {
  if (!djH.includes(k)) throw new Error(`default-job.h 缺 ${k}`);
}
if (!djCc.includes("DefaultJobState::")) throw new Error("default-job.cc 缺实现");

let rust = banner(
  "src/libplatform/default-worker-threads-task-runner.cc.js",
  "src/libplatform/default-worker-threads-task-runner.{h,cc} + delayed-task-queue.{h,cc} + default-foreground-task-runner.{h,cc} + default-job.{h,cc}",
  "任务运行器族骨架",
);

rust += `
/// Worker 线程运行器方法面 (原版 default-worker-threads-task-runner.h)
pub const WORKER_RUNNER_METHODS: [&str; ${dwFns.length}] = [${dwFns.map((f) => `"${f}"`).join(", ")}];

/// 前台运行器嵌套控制标志 (原版 default-foreground-task-runner.h)
pub const FOREGROUND_NESTING_FLAGS: [&str; ${nestedFlags.length}] = [${nestedFlags.map((f) => `"${f}"`).join(", ")}];

pub mod runner_skel {
  /// 延迟任务队列 (原版 DelayedTaskQueue: 按到期时间排序 + 条件变量唤醒)
  pub fn delayed_task_queue_skel() {
    unimplemented!("骨架")
  }
  /// 任务协作分派 (原版 DefaultJobState/DefaultJobHandle: 加入/退出/等待)
  pub fn job_state_skel() {
    unimplemented!("骨架")
  }
  /// 消息泵嵌套协议 (原台 foreground runner: PostTask 时唤醒已嵌套泵)
  pub fn message_pump_nesting_skel() {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-libplatform", "task_runners_generated.rs", rust);
console.log(`written ${p}; worker方法=${dwFns.length}, 嵌套标志=${nestedFlags.length}`);
