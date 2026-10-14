#!/usr/bin/env bun
// tool/src/heap/pending-allocations.js
// 机械提取 v8/src/heap/pending-allocations{.h,.cc} 族:
// YoungPendingAllocations — 快照版本化的新生代待定分配

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/pending-allocations.h");
const ccSrc = readV8("src/heap/pending-allocations.cc");

const anchors = [
  [hSrc, "class V8_EXPORT_PRIVATE YoungPendingAllocations final {"],
  [hSrc, "Address top = kNullAddress;"],
  [hSrc, "Address large_object = kNullAddress;"],
  [hSrc, "uintptr_t version = 0;"],
  [hSrc, "if (top <= addr && addr < limit) return true;"],
  [hSrc, "if (large_object == addr) return true;"],
  [hSrc, "void UpdateLab(Address top, Address limit);"],
  [hSrc, "void UpdateLargeObject(Address addr);"],
  [hSrc, "version_.load(std::memory_order_acquire) != snapshot->version"],
  [hSrc, "std::atomic<uintptr_t> version_{1};"],
  [ccSrc, "bool YoungPendingAllocations::ContainsSynchronized("],
  [ccSrc, "void YoungPendingAllocations::UpdateSnapshot("],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

const ccFns = ["ContainsSynchronized", "UpdateLab", "RemoveLab", "UpdateLargeObject", "RemoveLargeObject", "UpdateSnapshot", "ResetVersion", "BumpVersion"];
for (const f of ccFns) if (!ccSrc.includes(`YoungPendingAllocations::${f}(`)) throw new Error(`cc 缺 ${f}`);
console.log(`pending-allocations: cc 方法 ${ccFns.length}, Snapshot 含 Contains 内联 + 4 字段`);

let rust = banner(
  "src/heap/pending-allocations.js",
  "src/heap/pending-allocations.h, src/heap/pending-allocations.cc",
  "新生代待定分配快照 (版本号失效机制)",
);

rust += `use std::sync::atomic::{AtomicUsize, Ordering};

/// v8: src/heap/pending-allocations.h YoungPendingAllocations::Snapshot
pub struct Snapshot {
  /// v8: top / limit / large_object (均 kNullAddress 起始)
  pub top: Address,
  pub limit: Address,
  pub large_object: Address,
  /// v8: version = 0
  pub version: usize,
}

impl Snapshot {
  /// v8: Snapshot::Contains
  /// (top <= addr < limit) || large_object == addr
  pub fn contains(&self, addr: Address) -> bool {
    if self.top <= addr && addr < self.limit { return true; }
    if self.large_object == addr { return true; }
    false
  }
}

/// v8: src/heap/pending-allocations.h YoungPendingAllocations
pub struct YoungPendingAllocations {
  /// v8: heap_ (const)
  pub heap: *mut Heap,
  /// v8: top_ / limit_ / large_object_ = kNullAddress (mutable, mutex 保护)
  pub top: Address,
  pub limit: Address,
  pub large_object: Address,
  /// v8: version_{1} — acquire 读, 写侧 bump
  pub version: AtomicUsize,
}

impl YoungPendingAllocations {
  /// v8: ContainsSynchronized — 互斥锁下 Contains
  pub fn contains_synchronized(&self, addr: Address) -> bool { todo!() }
  /// v8: UpdateLab(top, limit) / RemoveLab
  pub fn update_lab(&mut self, top: Address, limit: Address) { todo!() }
  pub fn remove_lab(&mut self) { todo!() }
  /// v8: UpdateLargeObject(addr) / RemoveLargeObject
  pub fn update_large_object(&mut self, addr: Address) { todo!() }
  pub fn remove_large_object(&mut self) { todo!() }
  /// v8: UpdateSnapshot — 锁内重填快照并同步 version
  pub fn update_snapshot(&self, snapshot: &mut Snapshot) { todo!() }
  /// v8: UpdateSnapshotIfOutdated — acquire 读 version 不同才刷新
  pub fn update_snapshot_if_outdated(&self, snapshot: &mut Snapshot) {
    if self.version.load(Ordering::Acquire) != snapshot->version {
      self.update_snapshot(snapshot);
    }
  }
  /// v8: ResetVersion — 重置为 1 (起始值)
  pub fn reset_version(&mut self) { todo!() }
  /// v8: BumpVersion / is_marking (private, cc)
}
`;

const p = writeSrc("v8-heap-core", "pending_allocations.rs.skel", rust);
console.log(`written ${p}`);
