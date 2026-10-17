// 由 tool/src/objects/waiter-queue-node.h.js 从 v8/src/objects/waiter-queue-node.{h,cc} 生成; 修订就地改, 复跑覆盖
// WaiterQueueNode 等待者侵入链表骨架

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
pub const FIELDS: [&str; 6] = [
  "requester_",
  "next_",
  "prev_",
  "should_wait_",
  "wait_lock_",
  "wait_cond_var_",
];

// —— 签名骨架 (供 Developer 补语义) ——
// WaiterQueueNode(Isolate* requester); ~WaiterQueueNode();
// using DequeueMatcher = std::function<bool(WaiterQueueNode*)>;
// 调用约束: Dequeue* 须持有所属同步原语的队列锁
pub const SIGNATURES: [&str; 11] = [
  "ctor",
  "dtor",
  "enqueue",
  "dequeue_matching",
  "dequeue",
  "split",
  "length_from_head",
  "wait",
  "wait_for",
  "notify",
  "notify_all_in_list",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(FIELDS.len() == 6);
  assert!(INSTANCE_OPS.len() == 4);
};
