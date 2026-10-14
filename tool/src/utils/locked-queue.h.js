#!/usr/bin/env bun
// tool/src/utils/locked-queue.h.js
// 机械提取 v8/src/utils/locked-queue.{h,inl.h}: 双锁队列协议骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/utils/locked-queue.h");
const inlSrc = readV8("src/utils/locked-queue-inl.h");

// 对数守卫
const ops = ["Enqueue", "Dequeue", "Peek"];
for (const op of ops) {
  if (!new RegExp(`inline (?:void|bool) ${op.replace(/^\w/, (c) => c)}\\(`).test(inlSrc) && !inlSrc.includes(`LockedQueue<Record>::${op}`)) {
    throw new Error(`locked-queue-inl.h 缺 ${op}`);
  }
  if (!hSrc.includes(`inline ${op === "Enqueue" ? "void" : "bool"} ${op}`)) {
    throw new Error(`locked-queue.h 缺 ${op}`);
  }
}
// 双互斥锁布局 (Rochester 双锁队列; 原版 head_mutex_/tail_mutex_/size_ 原子计数)
for (const k of ["head_mutex_", "tail_mutex_", "std::atomic<size_t> size_"]) {
  if (!hSrc.includes(k)) throw new Error(`locked-queue.h 缺 ${k}`);
}
// Peek 为 const
if (!/inline bool Peek\(Record\* record\) const/.test(hSrc.replace(/\n/g, " "))) {
  if (!hSrc.includes("Peek(Record* record) const")) throw new Error("Peek 应为 const");
}

let rust = banner("src/utils/locked-queue.h.js", "src/utils/locked-queue.{h,inl.h}", "双锁线程安全队列骨架");

rust += `
/// LockedQueue — 双互斥锁队列 (原版 locked-queue.h; head/tail 双锁降低争用)
pub mod locked_queue_skel {
  /// 布局字段 (原版 head_mutex_(mutable) / tail_mutex_ / size_ 原子计数;
  /// Rochester 双锁队列伪码, 无界多生产多消费)
  pub const FIELDS: [&str; 3] = ["head_mutex", "tail_mutex", "size_atomic"];

  /// 入队 (原版 Enqueue; 持 tail 锁)
  pub fn enqueue(_record: usize) {
    unimplemented!("骨架")
  }
  /// 出队, 空则 false (原版 Dequeue; 持 head 锁)
  pub fn dequeue() -> Option<usize> {
    unimplemented!("骨架")
  }
  /// 窥视队首不改队 (原版 Peek; const)
  pub fn peek() -> Option<usize> {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-utils", "locked_queue_generated.rs", rust);
console.log(`written ${p}; 操作=${ops.length}`);
