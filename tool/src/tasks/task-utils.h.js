#!/usr/bin/env bun
// tool/src/tasks/task-utils.h.js
// 机械提取 v8/src/tasks/task-utils.{h,cc}: 工厂函数签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/tasks/task-utils.h");
const ccSrc = readV8("src/tasks/task-utils.cc");

// 对数守卫: 恰好 4 个工厂声明 (2 造 CancelableTask + 2 造 CancelableIdleTask)
const decls = hSrc.match(/std::unique_ptr<Cancelable(?:Idle)?Task>\s+Make\w+\(/g);
if (!decls || decls.length !== 4) throw new Error(`工厂声明应 4, 实得 ${decls ? decls.length : 0}`);

// cc 侧实现对数
const impls = ccSrc.match(/^std::unique_ptr<Cancelable(?:Idle)?Task> Make\w+\(/gm);
if (!impls || impls.length !== 4) throw new Error(`工厂实现应 4, 实得 ${impls ? impls.length : 0}`);

let rust = banner("src/tasks/task-utils.h.js", "src/tasks/task-utils.{h,cc}", "可取消任务工厂: 签名骨架");

rust += `
/// 任务工厂签名骨架 (原版 MakeCancelableTask / MakeCancelableIdleTask ×2 重载)。
/// 语义: 包装 Isolate 或裸管理器 → 构造对应 Cancelable 任务并装入 unique_ptr。
pub mod task_utils_skel {
  use crate::cancelable_generated::{TaskId, TryAbortResult};

  /// 原版: MakeCancelableTask(Isolate*, std::function<void()>)
  pub fn make_cancelable_task_from_isolate(isolate_key: usize, run: impl FnOnce()) -> CancelableTaskBox {
    let _ = (isolate_key, run);
    unimplemented!("骨架")
  }

  /// 原版: MakeCancelableTask(CancelableTaskManager*, std::function<void()>)
  pub fn make_cancelable_task_from_manager(manager_key: usize, run: impl FnOnce()) -> CancelableTaskBox {
    let _ = (manager_key, run);
    unimplemented!("骨架")
  }

  /// 原版: MakeCancelableIdleTask(Isolate*, std::function<void(double)>)
  pub fn make_cancelable_idle_task_from_isolate(isolate_key: usize, run: impl FnOnce(f64)) -> CancelableIdleTaskBox {
    let _ = (isolate_key, run);
    unimplemented!("骨架")
  }

  /// 原版: MakeCancelableIdleTask(CancelableTaskManager*, std::function<void(double)>)
  pub fn make_cancelable_idle_task_from_manager(manager_key: usize, run: impl FnOnce(f64)) -> CancelableIdleTaskBox {
    let _ = (manager_key, run);
    unimplemented!("骨架")
  }

  /// 原版 std::unique_ptr<CancelableTask> 的占位别名
  pub struct CancelableTaskBox {
    pub manager_key: usize,
    pub task_id: TaskId,
  }

  /// 原版 std::unique_ptr<CancelableIdleTask> 的占位别名
  pub struct CancelableIdleTaskBox {
    pub manager_key: usize,
    pub task_id: TaskId,
    /// 空闲任务终结结果占位 (对齐 TryAbortResult 协议)
    pub last_abort: Option<TryAbortResult>,
  }
}
`;

const p = writeSrc("v8-tasks", "task_utils_generated.rs", rust);
console.log(`written ${p}; 工厂声明=${decls.length}, 实现=${impls.length}`);
