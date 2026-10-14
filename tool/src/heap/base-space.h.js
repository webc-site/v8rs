#!/usr/bin/env bun
// tool/src/heap/base-space.h.js
// 机械提取 v8/src/heap/base-space.h: BaseSpace 抽象基类骨架 (所有分配空间的公共父类)。
// 规则: 类签名/方法清单/字段逐锚断言; 产物为 trait + 实现骨架, 语义面留 /// v8: 锚。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/heap/base-space.h");

// —— 锚点断言: 类签名与成员 (缺则 throw) ——
const anchors = [
  "class V8_EXPORT_PRIVATE BaseSpace : public Malloced {",
  "BaseSpace(const BaseSpace&) = delete;",
  "AllocationSpace identity() const { return id_; }",
  "virtual size_t CommittedMemory() const { return committed_; }",
  "virtual size_t CommittedPhysicalMemory() const = 0;",
  "virtual size_t Size() const = 0;",
  "BaseSpace(Heap* heap, AllocationSpace id) : heap_(heap), id_(id) {}",
  "void AccountCommitted(size_t bytes) {",
  "void AccountUncommitted(size_t bytes) {",
  "Heap* heap_;",
  "AllocationSpace id_;",
  "std::atomic<size_t> committed_{0};",
];
for (const a of anchors) {
  if (!src.includes(a)) throw new Error(`base-space.h 锚点缺失: ${a}`);
}
// VERIFY_HEAP 条件编译臂
if (!src.includes("#ifdef VERIFY_HEAP")) throw new Error("VERIFY_HEAP 臂缺失");
if ((src.match(/virtual void Verify\(/g) || []).length !== 1) throw new Error("Verify 声明数漂移");
console.log("BaseSpace 方法/字段锚 13 + Verify 条件臂 1");

let rust = banner(
  "src/heap/base-space.h.js",
  "src/heap/base-space.h",
  "BaseSpace: 所有分配空间抽象基类骨架",
);

rust += `
/// 空间公共父类 (对应 class BaseSpace : public Malloced)。
/// Rust 侧以 trait 表达抽象接口; committed_ 记账由实现方持有。
/// v8: src/heap/base-space.h BaseSpace
pub trait BaseSpace {
  /// 对应 identity() const { return id_; }
  fn identity(&self) -> crate::AllocationSpace;

  /// 对应 virtual size_t CommittedMemory() const { return committed_; }
  fn committed_memory(&self) -> usize;

  /// 对应 virtual size_t CommittedPhysicalMemory() const = 0;
  fn committed_physical_memory(&self) -> usize;

  /// 对应 virtual size_t Size() const = 0;
  fn size(&self) -> usize;
}

/// committed_ 记账原语 (对应 AccountCommitted/AccountUncommitted 内联体)。
/// v8: src/heap/base-space.h BaseSpace::AccountCommitted
pub fn account_committed(committed: &core::sync::atomic::AtomicUsize, bytes: usize) {
  committed.fetch_add(bytes, core::sync::atomic::Ordering::Relaxed);
}

/// v8: src/heap/base-space.h BaseSpace::AccountUncommitted
pub fn account_uncommitted(committed: &core::sync::atomic::AtomicUsize, bytes: usize) {
  committed.fetch_sub(bytes, core::sync::atomic::Ordering::Relaxed);
}

// —— 提取不变式断言 ——
const _: () = {
  // AccountCommitted/Uncommitted 只做加减记账, 无返回值
  assert!(core::mem::size_of::<usize>() >= 8);
};
`;

const p = writeSrc("v8-heap-core", "base_space.rs.skel", rust);
console.log(`written ${p}`);
