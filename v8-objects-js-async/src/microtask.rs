// 由 tool/src/objects/microtask.h.js 从 v8/src/objects/microtask.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// Microtask 微任务族

/// 异步恢复任务类别 (原版 AsyncResumeTask::Kind)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AsyncResumeKind {
  /// Async generator yield: resolve iterator result with done=false
  Yield = 0,
  /// Async function await: resume the generator with kNext
  AsyncFunctionAwait = 1,
}

/// 微任务形态表 (原版 tq extern class 继承序)
pub const MICROTASK_CLASSES: [(&str, &str); 4] = [
  ("Microtask", "Struct"),
  ("CallbackTask", "Microtask"),
  ("CallableTask", "Microtask"),
  ("AsyncResumeTask", "Microtask"),
];

/// ObjectTraits 偏移登记 (数值依赖布局求解器, 由 Developer 接线)
/// (trait 项, 所属类, 字段名)
pub const TRAIT_OFFSETS: [(&str, &str, &str); 4] = [
  (
    "ContinuationPreservedEmbedderDataOffset",
    "Microtask",
    "continuation_preserved_embedder_data_",
  ),
  ("GeneratorOffset", "AsyncResumeTask", "generator_"),
  ("ValueOffset", "AsyncResumeTask", "value_"),
  ("KindOffset", "AsyncResumeTask", "kind_"),
];

/// 微任务抽象基标记 (V8_IT 标注类)
pub struct Microtask;

// —— 签名骨架 ——
// CallbackTask: callback(Foreign) + data(Object)
// CallableTask: callable(JSReceiver) + context(NativeContext)
// AsyncResumeTask: generator(JSGeneratorObject) + value(Object) + kind(Smi)
// 访问器: generator/value/kind 及各自 setter (写屏障), DECL_VERIFIER/PRINTER
// V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA 分支: continuation_preserved_embedder_data 偏移
pub const MICROTASK_SIGNATURES: [&str; 8] = [
  "callback_task_callback",
  "callback_task_data",
  "callable_task_callable",
  "callable_task_context",
  "async_resume_generator",
  "async_resume_value",
  "async_resume_kind",
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
  assert!(MICROTASK_CLASSES.len() == 4);
  assert!(str_eq(MICROTASK_CLASSES[3].0, "AsyncResumeTask"));
  assert!(TRAIT_OFFSETS.len() == 4);
  assert!(AsyncResumeKind::Yield as u8 == 0);
  assert!(AsyncResumeKind::AsyncFunctionAwait as u8 == 1);
};
