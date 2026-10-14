#!/usr/bin/env bun
// tool/src/heap/collection-barrier.h.js
// 机械提取 v8/src/heap/collection-barrier.{h,cc}:
// CollectionRequest 优先级位状态机 (AtomicU8, kMajor=1 / kLastResort=2, 源自 globals.h
// enum class RequestedGCKind) + CollectionBarrier 方法台账。
// 规则: cc 内 fetch_or/fetch_and 位运算逐字断言后机械转写为 AtomicU8 规则代码。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/collection-barrier.h");
const cc = readV8("src/heap/collection-barrier.cc");

// —— 锚点断言 ——
for (const a of [
  "class CollectionBarrier {",
  "class V8_EXPORT_PRIVATE CollectionRequest {",
  "std::atomic<uint8_t> state_{0};",
  "std::optional<RequestedGCKind> RequestedGC();",
  "bool TryRequestGC(RequestedGCKind kind);",
  "void NotifyShutdownRequested();",
  "void StopTimeToCollectionTimer(RequestedGCKind kind);",
  "void ResumeThreadsAwaitingCollection(RequestedGCKind kind);",
  "void CancelCollectionAndResumeThreads();",
  "bool AwaitCollectionBackground(LocalHeap* local_heap, RequestedGCKind kind);",
]) {
  if (!h.includes(a)) throw new Error(`collection-barrier.h 锚点缺失: ${a}`);
}
// 状态机位运算逐字锚 (cc)
const bits = [
  "state_.fetch_or(static_cast<uint8_t>(kind), std::memory_order_relaxed);",
  "return previous_state >= static_cast<uint8_t>(kind);",
  "state_.fetch_and(",
  "~(static_cast<uint8_t>(kind) | static_cast<uint8_t>(kind) - 1),",
  "return previous_state & static_cast<uint8_t>(kind);",
  "state_.store(0, std::memory_order_relaxed);",
  "return state_.load(std::memory_order_relaxed) & static_cast<uint8_t>(kind);",
  "return state_.load(std::memory_order_relaxed) != 0;",
  "if (state & static_cast<uint8_t>(RequestedGCKind::kLastResort)) {",
];
for (const b of bits) {
  if (!cc.includes(b)) throw new Error(`collection-barrier.cc 位运算锚点缺失: ${b}`);
}
// 原版枚举值锚: globals.h RequestedGCKind { kMajor = 1, kLastResort = 1 << 1 }
const globals = readV8("src/common/globals.h");
if (!globals.includes("enum class RequestedGCKind : uint8_t { kMajor = 1, kLastResort = 1 << 1 };")) {
  throw new Error("globals.h RequestedGCKind 值漂移");
}
console.log("CollectionRequest 状态机 6 方法 + CollectionBarrier 公开 7 方法");

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

let rust = banner(
  "src/heap/collection-barrier.h.js",
  "src/heap/collection-barrier.{h,cc}",
  "后台线程 GC 请求栅栏: RequestedGCKind 优先级位状态机",
);

rust += `
/// 请求的 GC 种类 (对应 src/common/globals.h enum class RequestedGCKind : uint8_t)。
/// kMajor = 1, kLastResort = 1 << 1; 数值即优先级 (大者优先)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RequestedGCKind {
  /// kMajor = 1
  MAJOR = 1,
  /// kLastResort = 1 << 1
  LAST_RESORT = 1 << 1,
}

/// 后台线程的 GC 请求位标志 (对应 CollectionBarrier::CollectionRequest)。
/// state_ 位语义: bit0 = kMajor, bit1 = kLastResort; 数值大者优先。
/// v8: src/heap/collection-barrier.h CollectionBarrier::CollectionRequest
#[derive(Debug, Default)]
pub struct CollectionRequest {
  state: core::sync::atomic::AtomicU8,
}

impl CollectionRequest {
  /// 对应 Set: fetch_or(kind); 返回此前是否已有同级或更高优先级请求
  /// (previous_state >= kind, 因为低位被设后数值不小于任一已设位)
  /// v8: src/heap/collection-barrier.cc CollectionRequest::Set
  pub fn set(&self, kind: RequestedGCKind) -> bool {
    let previous_state =
      self.state.fetch_or(kind as u8, core::sync::atomic::Ordering::Relaxed);
    previous_state >= kind as u8
  }

  /// 对应 Clear: fetch_and(!(kind | kind - 1)); 清 kind 及所有更低优先级位,
  /// 返回 kind 位此前是否被设
  /// v8: src/heap/collection-barrier.cc CollectionRequest::Clear
  pub fn clear(&self, kind: RequestedGCKind) -> bool {
    let mask = !(kind as u8 | (kind as u8 - 1));
    let previous_state = self.state.fetch_and(mask, core::sync::atomic::Ordering::Relaxed);
    (previous_state & kind as u8) != 0
  }

  /// 对应 ClearAll: state_.store(0)
  /// v8: src/heap/collection-barrier.cc CollectionRequest::ClearAll
  pub fn clear_all(&self) {
    self.state.store(0, core::sync::atomic::Ordering::Relaxed);
  }

  /// 对应 Has: state & kind
  /// v8: src/heap/collection-barrier.cc CollectionRequest::Has
  pub fn has(&self, kind: RequestedGCKind) -> bool {
    (self.state.load(core::sync::atomic::Ordering::Relaxed) & kind as u8) != 0
  }

  /// 对应 Get: 先查 kLastResort 再查 kMajor, 皆无则 None
  /// v8: src/heap/collection-barrier.cc CollectionRequest::Get
  pub fn get(&self) -> Option<RequestedGCKind> {
    let state = self.state.load(core::sync::atomic::Ordering::Relaxed);
    if (state & RequestedGCKind::LAST_RESORT as u8) != 0 {
      Some(RequestedGCKind::LAST_RESORT)
    } else if (state & RequestedGCKind::MAJOR as u8) != 0 {
      Some(RequestedGCKind::MAJOR)
    } else {
      None
    }
  }

  /// 对应 HasAny: state != 0
  /// v8: src/heap/collection-barrier.cc CollectionRequest::HasAny
  pub fn has_any(&self) -> bool {
    self.state.load(core::sync::atomic::Ordering::Relaxed) != 0
  }
}

/// 停放/唤醒所有等待 GC 的后台线程 (对应 class CollectionBarrier)。
/// 互斥量/条件变量/计时器/TaskRunner 语义面留 Developer。
/// v8: src/heap/collection-barrier.h CollectionBarrier
pub struct CollectionBarrierSkeleton {
  pub collection_performed: bool,
  pub shutdown_requested: bool,
}

// —— 提取不变式断言 (原版位语义) ——
const _: () = {
  assert!(RequestedGCKind::MAJOR as u8 == 1);
  assert!(RequestedGCKind::LAST_RESORT as u8 == 2);
  let r = CollectionRequest::default();
  assert!(!r.has_any());
  assert!(r.set(RequestedGCKind::MAJOR) == false);
  assert!(r.set(RequestedGCKind::MAJOR) == true);
  assert!(r.get() == Some(RequestedGCKind::MAJOR));
  assert!(r.set(RequestedGCKind::LAST_RESORT) == false);
  assert!(r.get() == Some(RequestedGCKind::LAST_RESORT));
  assert!(r.clear(RequestedGCKind::LAST_RESORT) == true);
  // Clear(LAST_RESORT) 同时清了 MAJOR 位 (kind | kind-1 掩码)
  assert!(!r.has(RequestedGCKind::MAJOR));
  assert!(!r.has_any());
};
`;

const p = writeSrc("v8-heap-core", "collection_barrier.rs.skel", rust);
console.log(`written ${p}`);
