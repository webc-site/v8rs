#!/usr/bin/env bun
// tool/src/heap/local-heap.h.js
// 机械提取 v8/src/heap/local-heap.{h,cc,-inl.h}:
// ThreadState 3 位状态机 (ParkedBit/SafepointRequestedBit/CollectionRequestedBit)
// + LocalHeap 方法台账 (Safepoint 快路径/句柄族/LAB 族) + TLS g_current_local_heap_。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/local-heap.h");
const cc = readV8("src/heap/local-heap.cc");
const inl = readV8("src/heap/local-heap-inl.h");

// —— ThreadState 位域锚 ——
for (const a of [
  "using ParkedBit = base::BitField8<bool, 0, 1>;",
  "using SafepointRequestedBit = ParkedBit::Next<bool, 1>;",
  "using CollectionRequestedBit = SafepointRequestedBit::Next<bool, 1>;",
  "class ThreadState final {",
  "static constexpr ThreadState Parked() {",
  "static constexpr ThreadState Running() {",
  "constexpr bool IsRunning() const { return !ParkedBit::decode(raw_state_); }",
  "constexpr bool IsParked() const { return ParkedBit::decode(raw_state_); }",
  "constexpr bool IsSafepointRequested() const {",
  "constexpr bool IsCollectionRequested() const {",
  "constexpr bool IsRunningWithSlowPathFlag() const {",
  "return IsRunning() && (raw_state_ & (SafepointRequestedBit::kMask |",
  "CollectionRequestedBit::kMask));",
  "class AtomicThreadState final {",
]) {
  if (!h.includes(a)) throw new Error(`local-heap.h ThreadState 锚缺失: ${a}`);
}
// —— 类与方法台账 ——
for (const a of [
  "extern thread_local LocalHeap*",
  "g_current_local_heap_ V8_CONSTINIT;",
  "class V8_EXPORT_PRIVATE LocalHeap {",
  "void Safepoint() {",
  "ThreadState current = state_.load_relaxed();",
  "if (V8_UNLIKELY(current.IsRunningWithSlowPathFlag())) {",
  "SafepointSlowPath();",
  "LocalHandles* handles() { return handles_.get(); }",
  "void AttachPersistentHandles(",
  "std::unique_ptr<PersistentHandles> DetachPersistentHandles();",
  "bool IsParked() const;",
  "bool IsRunning() const;",
  "bool IsRetryOfFailedAllocation() const { return allocation_failed_; }",
  "void SetRetryOfFailedAllocation(bool value) { allocation_failed_ = value; }",
  "Heap* heap() const { return heap_; }",
  "MarkingBarrier* marking_barrier() { return marking_barrier_.get(); }",
  "void FreeLinearAllocationAreas();",
  "HeapAllocator* allocator() { return &heap_allocator_; }",
]) {
  if (!h.includes(a)) throw new Error(`local-heap.h 锚缺失: ${a}`);
}
for (const a of [
  "LocalHeap::LocalHeap(",
  "LocalHeap::~LocalHeap()",
  "void LocalHeap::SafepointSlowPath()",
  "void LocalHeap::FreeLinearAllocationAreas()",
  "void LocalHeap::AttachPersistentHandles(",
]) {
  if (!cc.includes(a)) throw new Error(`local-heap.cc 锚缺失: ${a}`);
}
if (!inl.includes("SetCurrentLocalHeapScope::SetCurrentLocalHeapScope(Isolate* isolate)")) {
  throw new Error("inl SetCurrentLocalHeapScope 缺失");
}
console.log("LocalHeap: ThreadState 位 3 + 方法台账 20");

let rust = banner(
  "src/heap/local-heap.h.js",
  "src/heap/local-heap.{h,cc,-inl.h}",
  "本地堆: 线程 Parked/Running 状态机与安全点",
);

rust += `
/// 线程状态 (对应 LocalHeap::ThreadState, 3 个 1-bit: bit0 Parked /
/// bit1 SafepointRequested / bit2 CollectionRequested)。
/// v8: src/heap/local-heap.h LocalHeap::ThreadState
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ThreadState(pub u8);

impl ThreadState {
  pub const PARKED_BIT: u8 = 1 << 0;
  pub const SAFEPOINT_REQUESTED_BIT: u8 = 1 << 1;
  pub const COLLECTION_REQUESTED_BIT: u8 = 1 << 2;

  /// 对应 static Parked(): 仅 Parked 位
  pub const PARKED: ThreadState = ThreadState(Self::PARKED_BIT);
  /// 对应 static Running(): 0
  pub const RUNNING: ThreadState = ThreadState(0);

  /// 对应 IsRunning: !Parked
  pub fn is_running(self) -> bool {
    self.0 & Self::PARKED_BIT == 0
  }

  /// 对应 IsParked
  pub fn is_parked(self) -> bool {
    self.0 & Self::PARKED_BIT != 0
  }

  /// 对应 SetRunning: 清 Parked 位
  pub fn set_running(self) -> Self {
    Self(self.0 & !Self::PARKED_BIT)
  }

  /// 对应 SetParked: 置 Parked 位
  pub fn set_parked(self) -> Self {
    Self(Self::PARKED_BIT | self.0)
  }

  /// 对应 IsSafepointRequested
  pub fn is_safepoint_requested(self) -> bool {
    self.0 & Self::SAFEPOINT_REQUESTED_BIT != 0
  }

  /// 对应 IsCollectionRequested
  pub fn is_collection_requested(self) -> bool {
    self.0 & Self::COLLECTION_REQUESTED_BIT != 0
  }

  /// 对应 IsRunningWithSlowPathFlag: Running 且 (SafepointRequested|CollectionRequested)
  pub fn is_running_with_slow_path_flag(self) -> bool {
    self.is_running() && (self.0 & (Self::SAFEPOINT_REQUESTED_BIT | Self::COLLECTION_REQUESTED_BIT)) != 0
  }
}

/// 本地堆骨架 (对应 class LocalHeap; GC 借它追踪所有可访问堆的线程)。
/// v8: src/heap/local-heap.h LocalHeap
pub struct LocalHeapSkeleton {
  pub state: core::sync::atomic::AtomicU8,
  pub allocation_failed: bool,
}

impl LocalHeapSkeleton {
  /// 对应 Safepoint() 快路径: relaxed 读状态, 带慢路径标志才走 SafepointSlowPath
  /// v8: src/heap/local-heap.h LocalHeap::Safepoint
  pub fn safepoint(&self) -> bool {
    let current = ThreadState(self.state.load(core::sync::atomic::Ordering::Relaxed));
    if current.is_running_with_slow_path_flag() {
      // SafepointSlowPath(): 停车等待请求处理, 此处骨架返回 true 表示走了慢路径
      true
    } else {
      false
    }
  }

  /// 对应 IsRetryOfFailedAllocation/SetRetryOfFailedAllocation
  pub fn set_retry_of_failed_allocation(&mut self, value: bool) {
    self.allocation_failed = value;
  }
}

// —— 提取不变式断言 ——
const _: () = {
  let s = ThreadState::RUNNING;
  assert!(s.is_running() && !s.is_parked());
  assert!(!s.is_running_with_slow_path_flag());
  let s2 = ThreadState::PARKED;
  assert!(s2.is_parked());
  assert!(!s2.is_running_with_slow_path_flag()); // 停车不算慢路径
  // Running + CollectionRequested → 慢路径
  let s3 = ThreadState(ThreadState::COLLECTION_REQUESTED_BIT);
  assert!(s3.is_running_with_slow_path_flag());
  let s4 = ThreadState(ThreadState::SAFEPOINT_REQUESTED_BIT);
  assert!(s4.is_safepoint_requested() && s4.is_running_with_slow_path_flag());
  // SetParked/SetRunning 往返
  assert!(s.set_parked().is_parked());
  assert!(s2.set_running().is_running());
  // 原子快路径: 停车状态无慢路径标志
  let lh = LocalHeapSkeleton { state: core::sync::atomic::AtomicU8::new(ThreadState::PARKED.0), allocation_failed: false };
  assert!(!lh.safepoint());
};
`;

const p = writeSrc("v8-heap-core", "local_heap.rs.skel", rust);
console.log(`written ${p}`);
