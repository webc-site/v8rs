#!/usr/bin/env bun
// tool/src/heap/finalization-registry-cleanup-task.h.js
// 机械提取 v8/src/heap/finalization-registry-cleanup-task.{h,cc}:
// FinalizationRegistryCleanupTask 台账 (GC 调度的脏注册表清理任务)。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/finalization-registry-cleanup-task.h");
const cc = readV8("src/heap/finalization-registry-cleanup-task.cc");

for (const a of [
  "class FinalizationRegistryCleanupTask : public CancelableTask {",
  "explicit FinalizationRegistryCleanupTask(Heap* heap);",
  "~FinalizationRegistryCleanupTask() override = default;",
  "void RunInternal() override;",
  "void SlowAssertNoActiveJavaScript();",
  "Heap* heap_;",
]) {
  if (!h.includes(a)) throw new Error(`finalization-registry-cleanup-task.h 锚点缺失: ${a}`);
}
for (const a of [
  "FinalizationRegistryCleanupTask::FinalizationRegistryCleanupTask(Heap* heap)",
  "void FinalizationRegistryCleanupTask::SlowAssertNoActiveJavaScript() {",
  "#ifdef ENABLE_SLOW_DCHECKS",
  "void FinalizationRegistryCleanupTask::RunInternal() {",
]) {
  if (!cc.includes(a)) throw new Error(`finalization-registry-cleanup-task.cc 锚点缺失: ${a}`);
}
console.log("FinalizationRegistryCleanupTask: 方法 3 (ctor/SlowAssert/RunInternal)");

let rust = banner(
  "src/heap/finalization-registry-cleanup-task.h.js",
  "src/heap/finalization-registry-cleanup-task.{h,cc}",
  "FinalizationRegistry 脏表清理任务骨架",
);

rust += `
/// 清理任务 (对应 class FinalizationRegistryCleanupTask : public CancelableTask)。
/// GC 发现脏 FinalizationRegistry 链非空时调度; RunInternal 处理一个注册表,
/// 若仍有脏表则再排一个清理任务。
/// v8: src/heap/finalization-registry-cleanup-task.h FinalizationRegistryCleanupTask
pub struct FinalizationRegistryCleanupTaskSkeleton {
  /// 对应 Heap* heap_
  pub heap: *mut (),
}

impl FinalizationRegistryCleanupTaskSkeleton {
  /// 对应 RunInternal: 先 SlowAssertNoActiveJavaScript, 再清理一个脏注册表,
  /// 剩余脏表时 post 新任务
  /// v8: src/heap/finalization-registry-cleanup-task.cc RunInternal
  pub fn run_internal(&mut self) {
    self.slow_assert_no_active_javascript();
    unimplemented!("骨架: dirty_js_finalization_registries 处理待 weak-refs 对象族就位");
  }

  /// 对应 SlowAssertNoActiveJavaScript (仅 ENABLE_SLOW_DCHECKS 生效):
  /// 遍历所有线程栈帧断言无 JS 帧在跑
  /// v8: src/heap/finalization-registry-cleanup-task.cc SlowAssertNoActiveJavaScript
  pub fn slow_assert_no_active_javascript(&self) {
    // 骨架: ThreadVisitor + StackFrameIterator 断言 it.frame()->is_javascript() 全假
  }
}
`;

const p = writeSrc("v8-heap-core", "finalization_registry_cleanup_task.rs.skel", rust);
console.log(`written ${p}`);
