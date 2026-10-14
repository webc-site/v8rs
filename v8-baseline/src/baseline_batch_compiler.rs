// 由 tool/src/baseline/baseline-batch-compiler.h.js 从 v8/src/baseline/baseline-batch-compiler.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 编译队列初始容量 (原版 kInitialQueueSize)
pub const INITIAL_QUEUE_SIZE: u32 = 32;

/// 原版 class/struct BaselineBatchCompiler 成员声明 (机械 1:1 提取, 22 条)
pub const BASELINE_BATCH_COMPILER_DECLARATIONS: &[&str] = &[
  "[public/const] static const uint32_t kInitialQueueSize = 32",
  "[public/method] explicit BaselineBatchCompiler(Isolate* isolate)",
  "[public/method] ~BaselineBatchCompiler()",
  "[public/method] void EnqueueFunction(DirectHandle<JSFunction> function)",
  "[public/method] void EnqueueSFI(Tagged<SharedFunctionInfo> shared)",
  "[public/method] void set_enabled(bool enabled)",
  "[public/method] bool is_enabled()",
  "[public/method] void InstallBatch()",
  "[private/method] bool concurrent() const",
  "[private/method] void EnsureQueueCapacity()",
  "[private/method] void Enqueue(DirectHandle<SharedFunctionInfo> shared)",
  "[private/method] bool ShouldCompileBatch(Tagged<SharedFunctionInfo> shared)",
  "[private/method] void CompileBatch(DirectHandle<JSFunction> function)",
  "[private/method] void CompileBatchConcurrent(Tagged<SharedFunctionInfo> shared)",
  "[private/method] void ClearBatch()",
  "[private/method] bool MaybeCompileFunction(Tagged<MaybeObject> maybe_sfi)",
  "[private/field] Isolate* isolate_",
  "[private/field] Handle<WeakFixedArray> compilation_queue_",
  "[private/field] uint32_t last_index_",
  "[private/field] int estimated_instruction_size_",
  "[private/field] bool enabled_",
  "[private/field] std::unique_ptr<ConcurrentBaselineCompiler> concurrent_compiler_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BASELINE_BATCH_COMPILER_DECLARATIONS.len() == 22);
  assert!(INITIAL_QUEUE_SIZE == 32);
};
