// 由 tool/src/wasm/module-compiler.h.js 从 v8/src/wasm/module-compiler.h 生成; 修订就地改, 复跑覆盖
// 编译任务前台调度策略枚举

/// 是否复用已存在的前台任务 (对应原版 CompilationStateImpl::UseExistingForegroundTask, enum : bool)
pub type UseExistingForegroundTask = bool;

/// kUseExistingForegroundTask — 复用, 避免派生重复任务
pub const USE_EXISTING_FOREGROUND_TASK: UseExistingForegroundTask = true;
/// kAssertNoExistingForegroundTask — 断言不存在并新建
pub const ASSERT_NO_EXISTING_FOREGROUND_TASK: UseExistingForegroundTask = false;

// —— 提取不变式断言 ——
const _: () = {
  assert!(USE_EXISTING_FOREGROUND_TASK);
};
