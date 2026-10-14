#!/usr/bin/env bun
// tool/src/tasks/operations-barrier.h.js
// 机械提取 v8/src/tasks/operations-barrier.{h,cc}: OperationsBarrier 签名骨架与状态常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/tasks/operations-barrier.h");
const ccSrc = readV8("src/tasks/operations-barrier.cc");

// 公开方法清单 (对数守卫: 恰好 3 个公开操作)
const pubOps = ["TryLock", "CancelAndWait", "cancelled"];
for (const op of pubOps) {
  if (!new RegExp(`\\b${op}\\s*\\(`).test(hSrc)) throw new Error(`未找到公开方法 ${op}`);
}

// cc 侧 Release 实现存在性
if (!/\bRelease\s*\(/.test(ccSrc)) throw new Error("cc 未找到 Release 实现");
if (!/cancelled_/.test(ccSrc)) throw new Error("cc 未找到 cancelled_ 状态");

// 成员状态对数
const members = ["mutex_", "release_condition_", "cancelled_", "operations_count_"];
for (const m of members) {
  if (!hSrc.includes(m)) throw new Error(`头文件未找到成员 ${m}`);
}

let rust = banner("src/tasks/operations-barrier.h.js", "src/tasks/operations-barrier.{h,cc}", "多线程操作生命周期屏障: 签名骨架");

rust += `
/// OperationsBarrier — 线程安全屏障, 管理多线程操作生命周期 (签名骨架)。
///
/// 原版协议: TryLock() 成功返回 RAII Token (转 bool 为 true 表示该操作
/// happens-before CancelAndWait()); CancelAndWait() 后禁再放行, 并等待
/// 全部在途操作完成。语义实现见 crate 内手写 operations_barrier.rs。
pub struct OperationsBarrierSkel {
  /// 互斥锁 (原版 base::Mutex mutex_)
  _mutex: (),
  /// 放行条件变量 (原版 base::ConditionVariable release_condition_)
  _release_condition: (),
  /// CancelAndWait 是否已发生 (原版 bool cancelled_, 初始 false)
  _cancelled: bool,
  /// 在途操作计数 (原版 size_t operations_count_, 初始 0)
  _operations_count: usize,
}

impl OperationsBarrierSkel {
  /// 在途操作计数初始值 (原版 operations_count_{0})
  pub const INITIAL_OPERATIONS_COUNT: usize = 0;

  /// 尝试放行一个操作, 返回 RAII Token (原版 Token TryLock())
  /// 返回 None 等价于原版 Token 转 bool == false
  pub fn try_lock(&mut self) -> Option<TokenSkel> {
    None // 骨架: 语义实现由 Developer 修订
  }

  /// 阻止后续放行并等待全部在途操作完成 (原版 void CancelAndWait())
  /// 仅可调用一次
  pub fn cancel_and_wait(&mut self) {
    unimplemented!("骨架")
  }

  /// 是否已取消 (原版 bool cancelled() const)
  pub fn cancelled(&self) -> bool {
    unimplemented!("骨架")
  }

  /// 内部: Token 析构时回收一个在途操作 (原版 void Release(), 私有)
  fn release(&mut self) {
    unimplemented!("骨架")
  }
}

/// TryLock 返回的 RAII 令牌 (原版嵌套类 OperationsBarrier::Token)
pub struct TokenSkel {
  /// 所属屏障 (原版 outer_), None 表示已移交通配的空 Token
  _outer: Option<*mut OperationsBarrierSkel>,
}

impl TokenSkel {
  /// Token 转 bool (原版 operator bool): 持有真实屏障即 true
  pub fn is_active(&self) -> bool {
    self._outer.is_some()
  }
}

impl Drop for TokenSkel {
  /// 析构回收 (原版 ~Token: if (outer_) outer_->Release())
  fn drop(&mut self) {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-tasks", "operations_barrier_generated.rs", rust);
console.log(`written ${p}; 公开方法=${pubOps.length}, 成员=${members.length}`);
