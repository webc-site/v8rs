#!/usr/bin/env bun
// tool/src/objects/waiter-queue-node.h.js — 覆盖 v8/src/objects/waiter-queue-node.{h,cc}
// WaiterQueueNode: 进程内每等待者双向侵入链表 (非堆对象), 方法面骨架
// → v8-objects-js-async/src/waiter_queue_node.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/waiter-queue-node.h");
const ccSrc = readV8("src/objects/waiter-queue-node.cc");

// 1. 方法面对数 (h 侧 public 区静态/实例方法)
const methods = [...hSrc.matchAll(/static [\w*]+ (\w+)\(|(?:^  )~?(\w+)\(|(?:  )(?:bool|int|uint32_t|void) (\w+)\(/gm)]
  .map((m) => m[1] ?? m[2] ?? m[3])
  .filter((n) => n && !["WaiterQueueNode", "wait_lock_", "wait_cond_var_"].includes(n));
const uniq = [...new Set(methods)];
for (const need of ["Enqueue", "DequeueMatching", "Dequeue", "Split", "LengthFromHead", "Wait", "WaitFor", "Notify", "NotifyAllInList"]) {
  if (!uniq.includes(need)) throw new Error(`缺方法 ${need}`);
}

// 2. cc 侧实现对数
for (const need of ["Enqueue", "DequeueMatching", "LengthFromHead", "NotifyAllInList"]) {
  if (!ccSrc.includes(`WaiterQueueNode::${need}`)) throw new Error(`cc 缺实现 ${need}`);
}

// 3. 字段面
const fields = ["requester_", "next_", "prev_", "should_wait_", "wait_lock_", "wait_cond_var_"];
for (const f of fields) {
  if (!hSrc.includes(`${f} =`) && !hSrc.includes(`${f};`)) throw new Error(`缺字段 ${f}`);
}

console.log(`WaiterQueueNode 方法 ${uniq.length}, 字段 ${fields.length}`);

let rust = banner("src/objects/waiter-queue-node.h.js", "src/objects/waiter-queue-node.{h,cc}", "WaiterQueueNode 等待者侵入链表骨架");

rust += `
/// 等待者队列节点 (原版 detail::WaiterQueueNode, 栈上分配, 非堆对象)
/// 每同步原语 (mutex/条件变量) 一条进程级双向侵入链; 头 prev=尾, 尾 next=头 (环绕)
/// 指针压缩/沙箱下经共享外部指针表间接访问栈上节点
pub struct WaiterQueueNode;

/// 静态链表操作 (原版 public 静态区)
pub enum WaiterQueueOp {
  /// Enqueue: 尾插并改头
  Enqueue,
  /// DequeueMatching: 取首个满足 matcher 的等待者
  DequeueMatching,
  /// Dequeue
  Dequeue,
  /// Split: 切出至多 count 个节点为独立链
  Split,
  /// LengthFromHead: 自头遍历计数 (错误编码会死循环, 原版注明)
  LengthFromHead,
}

/// 实例等待面
/// Wait() 睡眠; WaitFor(TimeDelta) 超时返回 false; Notify() 唤醒; NotifyAllInList() 返回唤醒数
pub const INSTANCE_OPS: [&str; 4] = ["wait", "wait_for", "notify", "notify_all_in_list"];

/// 字段面 (私有)
pub const FIELDS: [&str; ${fields.length}] = [
${fields.map((f) => `  "${f}",`).join("\n")}
];

// —— 签名骨架 (供 Developer 补语义) ——
// WaiterQueueNode(Isolate* requester); ~WaiterQueueNode();
// using DequeueMatcher = std::function<bool(WaiterQueueNode*)>;
// 调用约束: Dequeue* 须持有所属同步原语的队列锁
pub const SIGNATURES: [&str; 11] = [
  "ctor", "dtor", "enqueue", "dequeue_matching", "dequeue", "split",
  "length_from_head", "wait", "wait_for", "notify", "notify_all_in_list",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(FIELDS.len() == 6);
  assert!(INSTANCE_OPS.len() == 4);
};
`;

const p = writeSrc("v8-objects-js-async", "waiter_queue_node.rs", rust);
console.log(`written ${p}`);
