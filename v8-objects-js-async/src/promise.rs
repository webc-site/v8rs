// 由 tool/src/objects/promise.h.js 从 v8/src/objects/promise.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// Promise 结构族

/// 反应类型 (原版 PromiseReaction::Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PromiseReactionType {
  /// kFulfill
  Fulfill = 0,
  /// kReject
  Reject = 1,
}

/// Promise 结构形态表 (原版 tq extern class 继承序)
pub const PROMISE_CLASSES: [(&str, &str); 6] = [
  ("PromiseCapability", "Struct"),
  ("PromiseReaction", "Struct"),
  ("PromiseReactionJobTask", "Microtask"),
  ("PromiseFulfillReactionJobTask", "PromiseReactionJobTask"),
  ("PromiseRejectReactionJobTask", "PromiseReactionJobTask"),
  ("PromiseResolveThenableJobTask", "Microtask"),
];

/// ObjectTraits 偏移登记 (数值依赖布局求解器, 由 Developer 接线)
/// (trait 项, 所属类, 字段名)
pub const TRAIT_OFFSETS: [(&str, &str, &str); 5] = [
  (
    "FulfillHandlerOffset",
    "PromiseReaction",
    "fulfill_handler_",
  ),
  (
    "PromiseOrCapabilityOffset",
    "PromiseReaction",
    "promise_or_capability_",
  ),
  (
    "ContinuationPreservedEmbedderDataOffset",
    "PromiseReaction",
    "continuation_preserved_embedder_data_",
  ),
  ("HandlerOffset", "PromiseReactionJobTask", "handler_"),
  (
    "PromiseOrCapabilityOffset",
    "PromiseReactionJobTask",
    "promise_or_capability_",
  ),
];

// —— 签名骨架 ——
// PromiseCapability: promise(JSReceiver|Undefined) / resolve(JSAny) / reject(JSAny)
// PromiseReaction: next / reject_handler / fulfill_handler / promise_or_capability
// PromiseReactionJobTask(@abstract): argument / context / handler / promise_or_capability
// PromiseFulfillReactionJobTask / PromiseRejectReactionJobTask: 空扩展
// PromiseResolveThenableJobTask: context / promise_to_resolve / thenable / then
// V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA 分支字段登记于 TRAIT_OFFSETS
pub const PROMISE_SIGNATURES: [&str; 6] = [
  "capability_promise_resolve_reject",
  "reaction_next_handlers",
  "reaction_job_task_argument_context_handler",
  "fulfill_reject_job_tasks",
  "resolve_thenable_job",
  "verify_printer",
];

// —— 提取不变式断言 —— (const 上下文无 PartialEq<&str>, 用字节循环 const fn)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}
const _: () = {
  assert!(PROMISE_CLASSES.len() == 6);
  assert!(str_eq(PROMISE_CLASSES[0].0, "PromiseCapability"));
  assert!(str_eq(PROMISE_CLASSES[2].0, "PromiseReactionJobTask"));
  assert!(TRAIT_OFFSETS.len() == 5);
  assert!(PromiseReactionType::Fulfill as u8 == 0);
};
