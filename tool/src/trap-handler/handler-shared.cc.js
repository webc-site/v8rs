#!/usr/bin/env bun
// handler-shared.cc → v8-trap-handler/src/shared.rs 转换脚本
// 规则: 跨内外共享的全局状态实现侧。thread_local is_active_ → const 初始化 TLS
// (零开销无懒初始化, 信号上下文安全); gNumCodeObjects/gCodeObjects/gNextCodeObject
// 聚合为 Metadata (Vec 形态, 遍历语义 1:1); gRecoveredTrapCount/gLandingPad 原子。
import { readV8, writeSrc, banner, lines } from "../../lib.js";

const src = readV8("src/trap-handler/handler-shared.cc");

const regions = {
  tls: lines(src, 27, 31),
  globals: lines(src, 33, 38),
  locks: lines(src, 51, 97),
};

function must(cond, msg) {
  if (!cond) throw new Error(`handler-shared.cc 对数核对失败: ${msg}`);
}
must(
  regions.tls.join("\n").includes("thread_local bool TrapHandlerGuard::is_active_"),
  "TLS 区间漂移"
);
must(
  regions.globals.join("\n").includes("std::atomic<uintptr_t> gLandingPad"),
  "全局状态区间漂移"
);
must(
  regions.locks.filter((l) => l.includes("spinlock_.clear")).length === 3,
  "三把锁析构数不符"
);

const body = `${banner("src/trap-handler/handler-shared.cc.js", "src/trap-handler/handler-shared.cc")}
//! 跨内外共享的全局状态实现; 类型见 internal.rs
//! 规则区原文行区间: ${JSON.stringify(Object.keys(regions))}
use std::cell::Cell;
use std::sync::atomic::AtomicUsize;

use crate::internal::{CodeEntry, CoveredMemRec, Locked, SandboxRec, th_check};

// handler-shared.cc:30 TrapHandlerGuard::is_active_
// const 初始化: 无懒初始化、无析构, TLS 段级就位 → 信号上下文访问安全
// (原版 thread_local bool = 0 同语义; EnableTrapHandler 的 warm-up 见 outside.rs)
thread_local! {
  static IS_ACTIVE: Cell<bool> = const { Cell::new(false) };
}

/// trap-handler-internal.h:32 TrapHandlerGuard
/// 线程级防嵌套/防死锁守卫: 激活期间本线程进入信号处理器即放弃处理。
/// (原版 thread_in_wasm 标志在本版 V8 已由该机制取代)
pub(crate) struct TrapHandlerGuard {
  _priv: (),
}

impl TrapHandlerGuard {
  /// 嵌套激活是程序错误 → 立即崩溃 (原版 TH_CHECK(!is_active_))
  pub(crate) fn new() -> Self {
    th_check!(!Self::is_active());
    IS_ACTIVE.with(|f| f.set(true));
    Self { _priv: () }
  }

  /// trap-handler-internal.h:43 IsActiveOnCurrentThread
  pub(crate) fn is_active() -> bool {
    IS_ACTIVE.with(Cell::get)
  }
}

impl Drop for TrapHandlerGuard {
  fn drop(&mut self) {
    IS_ACTIVE.with(|f| f.set(false));
  }
}

/// trap-handler-internal.h:90 gNumCodeObjects/gCodeObjects +
/// handler-outside.cc:35 gNextCodeObject 的聚合 (Vec 形态, 遍历语义 1:1)
pub(crate) struct Metadata {
  /// 自由链表游标 gNextCodeObject
  pub(crate) next: usize,
  /// gCodeObjects 表; table.len() == gNumCodeObjects
  pub(crate) table: Vec<CodeEntry>,
}

/// handler-shared.cc:34 gCodeObjects + gNumCodeObjects
pub(crate) static METADATA: Locked<Metadata> =
  Locked::new(Metadata { next: 0, table: Vec::new() });
/// handler-shared.cc:35 gSandboxRecordsHead (侵入式链表 → Vec, 遍历语义 1:1)
pub(crate) static SANDBOX_RECORDS: Locked<Vec<SandboxRec>> = Locked::new(Vec::new());
/// handler-shared.cc:36 gCoveredMemoryRecordsHead
pub(crate) static COVERED_MEMORY: Locked<Vec<CoveredMemRec>> = Locked::new(Vec::new());
/// handler-shared.cc:37 gRecoveredTrapCount
pub(crate) static RECOVERED_TRAP_COUNT: AtomicUsize = AtomicUsize::new(0);
/// handler-shared.cc:38 gLandingPad (原版默认 seq_cst, 保持)
pub(crate) static LANDING_PAD: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::atomic::Ordering;

  use crate::outside::set_landing_pad;

  /// landing pad 表查询语义: set 后可查、可清零 (trap-handler.h:163 SetLandingPad)
  #[test]
  fn landing_pad_store_load() {
    set_landing_pad(0xDEAD_BEEF);
    assert_eq!(LANDING_PAD.load(Ordering::SeqCst), 0xDEAD_BEEF);
    set_landing_pad(0);
    assert_eq!(LANDING_PAD.load(Ordering::SeqCst), 0);
  }

  /// TrapHandlerGuard set/reset 嵌套语义 (对应原版 thread-in-wasm 标志语义):
  /// 激活期间 is_active 为真, 释放后为假; 嵌套构造会 abort, 不可在测试中触发
  #[test]
  fn guard_set_reset() {
    assert!(!TrapHandlerGuard::is_active());
    let guard = TrapHandlerGuard::new();
    assert!(TrapHandlerGuard::is_active());
    drop(guard);
    assert!(!TrapHandlerGuard::is_active());
  }
}
`;

const p = writeSrc("v8-trap-handler", "shared.rs", body);
console.log(`handler-shared.cc.js: shared.rs 已产出 ${p} (static 5 + guard + 测试 2)`);
