#!/usr/bin/env bun
// trap-handler-internal.h → v8-trap-handler/src/internal.rs 转换脚本
// 规则: 内部状态类型侧。提取 TrapHandlerGuard / CodeProtectionInfo /
// 自由链表 CodeProtectionInfoListEntry / SandboxRecord / CoveredMemoryRecord /
// 三把 spinlock 锁类 (MetadataLock/SandboxRecordsLock/CoveredMemoryRecordsLock)。
// C++ 柔性数组 instructions[1] → Box<[TrappingInstructionData]>;
// atomic_flag test_and_set/clear → AtomicBool swap/store (Acquire/Release) 1:1。
// 锁构造的 TH_CHECK(TrapHandlerGuard active) 契约保留在 Locked::lock。
import { readV8, writeSrc, banner, lines } from "../../lib.js";

const src = readV8("src/trap-handler/trap-handler-internal.h");

const regions = {
  guard: lines(src, 32, 53),
  codeInfo: lines(src, 60, 65),
  metadataLock: lines(src, 67, 76),
  freeList: lines(src, 78, 88),
  records: lines(src, 93, 104),
  locks: lines(src, 106, 126),
  query: lines(src, 135, 141),
};

function must(cond, msg) {
  if (!cond) throw new Error(`trap-handler-internal.h 对数核对失败: ${msg}`);
}
must(
  regions.guard.join("\n").includes("static bool IsActiveOnCurrentThread()"),
  "TrapHandlerGuard 区间漂移"
);
must(
  regions.metadataLock.join("\n").includes("std::atomic_flag spinlock_"),
  "MetadataLock 区间漂移"
);
must(
  regions.freeList.join("\n").includes("size_t next_free"),
  "自由链表区间漂移"
);

const body = `${banner("src/trap-handler/trap-handler-internal.h.js", "src/trap-handler/trap-handler-internal.h")}
//! 内部状态与锁 (类型侧); 实现见 shared.rs
//! 规则区原文行区间: ${JSON.stringify(Object.keys(regions))}
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::TrappingInstructionData;

/// trap-handler-internal.h:84 TH_CHECK → IMMEDIATE_CRASH
/// 立即终止进程: 无 unwinding、async-signal-safe, 信号路径禁 panic 的崩溃出口
macro_rules! th_check {
  ($cond:expr) => {
    if !$cond {
      std::process::abort()
    }
  };
}
pub(crate) use th_check;

/// trap-handler-internal.h:60 CodeProtectionInfo
/// 一段可被陷阱处理器接管的代码: base/size + 受保护访问指令表
pub(crate) struct CodeProtectionInfo {
  pub(crate) base: usize,
  pub(crate) size: usize,
  /// 受保护指令 (C 柔性数组 instructions[1] 的 Rust 形态);
  /// num_trapping_instructions == instructions.len()
  pub(crate) instructions: Box<[TrappingInstructionData]>,
}

/// trap-handler-internal.h:85 CodeProtectionInfoListEntry
/// 自由链表项。next_free 即"下一个插入槽"索引 (handler-outside.cc:184 扩容时
/// 置为 j+1, :224 释放槽时置为当前游标), 游标推进直接取用、不减 1;
/// 游标 == table.len() 表示无空洞需扩容
pub(crate) struct CodeEntry {
  pub(crate) code_info: Option<Box<CodeProtectionInfo>>,
  pub(crate) next_free: usize,
}

/// trap-handler-internal.h:94 SandboxRecord (侵入式链表节点 → 扁平字段)
pub(crate) struct SandboxRec {
  pub(crate) base: usize,
  pub(crate) size: usize,
}

/// trap-handler-internal.h:100 CoveredMemoryRecord
pub(crate) struct CoveredMemRec {
  pub(crate) base: usize,
  pub(crate) size: usize,
}

/// handler-shared.cc:57 atomic_flag test_and_set/clear 的 1:1 自旋锁。
/// 信号处理器内也会获取: 纯原子自旋, 无分配、无系统调用、无 panic,
/// 满足 async-signal-safety
pub(crate) struct SpinLock {
  locked: AtomicBool,
}

impl SpinLock {
  pub(crate) const fn new() -> Self {
    Self { locked: AtomicBool::new(false) }
  }

  fn acquire(&self) {
    while self.locked.swap(true, Ordering::Acquire) {}
  }

  fn release(&self) {
    self.locked.store(false, Ordering::Release);
  }
}

/// MetadataLock / SandboxRecordsLock / CoveredMemoryRecordsLock 的通用底座:
/// 自旋锁保护的内部数据
pub(crate) struct Locked<T> {
  lock: SpinLock,
  data: UnsafeCell<T>,
}

// SAFETY: data 仅在持有 SpinLock 时经 lock() 访问 (独占);
// T: Send 保证数据本身可跨线程转移, 不引入数据竞争
unsafe impl<T: Send> Sync for Locked<T> {}

impl<T> Locked<T> {
  pub(crate) const fn new(data: T) -> Self {
    Self { lock: SpinLock::new(), data: UnsafeCell::new(data) }
  }

  /// handler-shared.cc:51 MetadataLock 构造契约:
  /// 该锁也会在信号处理器内获取, 必须在 TrapHandlerGuard 激活时加锁;
  /// 否则处理器持锁期间再次故障 → 死锁, 违反契约即崩溃
  pub(crate) fn lock(&self) -> LockGuard<'_, T> {
    th_check!(crate::shared::TrapHandlerGuard::is_active());
    self.lock.acquire();
    LockGuard { owner: self, guarded: true }
  }

  /// 处理器外数据锁 (如 g_old_handler): 不施加 TrapHandlerGuard 契约。
  /// 注意 remove_trap_handler 会在信号处理器内被调 (接管失败路径) 且取此锁,
  /// 无 guard 可依——调用方必须自行屏蔽 kOobSignal 消除"持锁线程被本信号
  /// 打断 → 处理器内重入取同锁"的自旋死锁 (见 outside_posix::with_old_handler)
  pub(crate) fn lock_outside(&self) -> LockGuard<'_, T> {
    self.lock.acquire();
    LockGuard { owner: self, guarded: false }
  }
}

/// 持锁守卫: Drop 时释放 (对应 C++ 锁析构)
pub(crate) struct LockGuard<'a, T> {
  owner: &'a Locked<T>,
  /// 是否受 TrapHandlerGuard 契约保护 (决定 drop 时的契约检查)
  guarded: bool,
}

impl<T> Deref for LockGuard<'_, T> {
  type Target = T;

  fn deref(&self) -> &T {
    // SAFETY: 持锁独占; 信号处理器路径仅经此只读遍历 (无分配/无增长)
    unsafe { &*self.owner.data.get() }
  }
}

impl<T> DerefMut for LockGuard<'_, T> {
  fn deref_mut(&mut self) -> &mut T {
    // SAFETY: 持锁独占; mut 仅出现在处理器外路径 (注册/反注册)
    unsafe { &mut *self.owner.data.get() }
  }
}

impl<T> Drop for LockGuard<'_, T> {
  fn drop(&mut self) {
    // handler-shared.cc:61 析构契约: 契约锁释放时 guard 必须仍激活
    if self.guarded {
      th_check!(crate::shared::TrapHandlerGuard::is_active());
    }
    self.owner.lock.release();
  }
}
`;

const p = writeSrc("v8-trap-handler", "internal.rs", body);
console.log(`trap-handler-internal.h.js: internal.rs 已产出 ${p} (类型 5 + 锁 2)`);
