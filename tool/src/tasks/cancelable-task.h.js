#!/usr/bin/env bun
// tool/src/tasks/cancelable-task.h.js
// 机械提取 v8/src/tasks/cancelable-task.{h,cc}:
// TryAbortResult / Cancelable::Status 枚举, kInvalidTaskId 常量, 类签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/tasks/cancelable-task.h");
const ccSrc = readV8("src/tasks/cancelable-task.cc");

// 1. TryAbortResult 枚举
const m1 = hSrc.match(/enum class TryAbortResult\s*\{([^}]+)\};/);
if (!m1) throw new Error("未找到 TryAbortResult 枚举");
const tryAbortResults = m1[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (tryAbortResults.length !== 3) throw new Error(`TryAbortResult 应 3 项, 实得 ${tryAbortResults.length}`);

function snakeToUpperCamel(s) {
  return s
    .replace(/^k/, "")
    .toLowerCase()
    .replace(/(^|_)(\w)/g, (_, __, c) => c.toUpperCase());
}

// 2. Cancelable::Status 枚举 (保护成员, 匿名值枚举)
const m2 = hSrc.match(/enum Status\s*\{([^}]+)\};/);
if (!m2) throw new Error("未找到 Cancelable::Status 枚举");
const statuses = m2[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (statuses.length !== 3) throw new Error(`Status 应 3 项, 实得 ${statuses.length}`);

// 3. 常量
const m3 = hSrc.match(/static constexpr Id kInvalidTaskId = (\d+);/);
if (!m3) throw new Error("未找到 kInvalidTaskId");
const invalidTaskId = m3[1];

// 4. cc 侧方法签名骨架 (逐行拼接 `Class::Method(...) {` 定义头)
const sigs = [];
const ccLines = ccSrc.split("\n");
for (let i = 0; i < ccLines.length; i++) {
  const l = ccLines[i];
  if (/^[A-Za-z_][\w:<>,*&\s]*\b([\w:<>]+)::(~?\w+)\s*\(/.test(l) && !/^\s*\/\//.test(l)) {
    let sig = l;
    let j = i;
    while (!/\{\s*$|;\s*$/.test(sig) && j + 1 < ccLines.length) sig += " " + ccLines[++j].trim();
    sig = sig.replace(/\{\s*$/, "").trim();
    if (sig.endsWith(";")) continue; // 声明非定义
    sigs.push(sig);
  }
}
if (sigs.length < 5) throw new Error(`cc 实现签名应 ≥5, 实得 ${sigs.length}`);

let rust = banner("src/tasks/cancelable-task.h.js", "src/tasks/cancelable-task.{h,cc}", "可取消任务体系: 枚举/常量/签名骨架");

rust += `
/// 尝试中止任务的结局 (对应原版 enum class TryAbortResult)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TryAbortResult {
${tryAbortResults.map((t, i) => `  /// \`${t}\`\n  ${snakeToUpperCamel(t)} = ${i},`).join("\n")}
}

impl TryAbortResult {
  /// 总数对数守卫
  pub const COUNT: usize = ${tryAbortResults.length};
}

/// 可取消任务管理器: 任务 id 类型 (原版 using Id = uint64_t)
pub type TaskId = u64;

/// 非法任务 id 哨兵 (原版 kInvalidTaskId = 0)
pub const INVALID_TASK_ID: TaskId = ${invalidTaskId};

/// Cancelable 任务状态机 (对应原版 enum Status, 保护成员)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Status {
${statuses.map((s, i) => `  /// \`${s}\`\n  ${snakeToUpperCamel(s)} = ${i},`).join("\n")}
}

/// CancelableTaskManager 方法签名骨架 (语义实现见 crate 内手写 cancelable.rs)
pub trait CancelableTaskManagerLike {
  /// 注册新可取消任务, 返回唯一 id
  fn register(&mut self, task_key: usize) -> TaskId;
  /// 按 id 尝试中止
  fn try_abort(&mut self, id: TaskId) -> TryAbortResult;
  /// 尝试取消全部已注册任务
  fn try_abort_all(&mut self) -> TryAbortResult;
  /// 取消全部并等待运行中的任务; 之后禁再注册
  fn cancel_and_wait(&mut self);
  /// 是否已取消
  fn canceled(&self) -> bool;
  /// 仅由 Cancelable 析构调用: 摘除已完成任务
  fn remove_finished_task(&mut self, id: TaskId);
}

/// Cancelable 方法签名骨架
pub trait CancelableLike {
  /// 任务 id (交给平台后禁再调)
  fn id(&self) -> TaskId;
  /// 状态从 Waiting → Running, 成功即获得执行权
  fn try_run(&self) -> bool;
  /// 状态从 Waiting → Canceled (仅管理器可调)
  fn cancel(&self) -> bool;
}

// cc 侧实现定义头对数清单 (原文签名, 供 Developer 修订对照):
${sigs.map((s) => "//   " + s).join("\n")}
`;

// —— 提取不变式断言 (脚本侧) ——
if (tryAbortResults.join() !== "kTaskRemoved,kTaskRunning,kTaskAborted") {
  throw new Error("TryAbortResult 项序漂移: " + tryAbortResults.join());
}
if (statuses.join() !== "kWaiting,kCanceled,kRunning") {
  throw new Error("Status 项序漂移: " + statuses.join());
}

const p = writeSrc("v8-tasks", "cancelable_generated.rs", rust);
console.log(`written ${p}; TryAbortResult=${tryAbortResults.length}, Status=${statuses.length}, cc签名=${sigs.length}`);
