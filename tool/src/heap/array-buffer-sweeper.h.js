#!/usr/bin/env bun
// tool/src/heap/array-buffer-sweeper.h.js
// 机械提取 v8/src/heap/array-buffer-sweeper.{h,cc}:
// SweepingType/TreatAllYoungAsPromoted 枚举 + ArrayBufferList 记账规则
// (Append/bytes 近似值) + ArrayBufferSweeper 方法台账 + Age 枚举 (js-array-buffer.h)。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/array-buffer-sweeper.h");
const cc = readV8("src/heap/array-buffer-sweeper.cc");
const jab = readV8("src/objects/js-array-buffer.h");

// Age (js-array-buffer.h)
if (!jab.includes("enum class Age : uint8_t { kYoung = 0, kOld = 1 };")) {
  throw new Error("ArrayBufferExtension::Age 值漂移");
}
for (const a of [
  "struct ArrayBufferList final {",
  "using Age = ArrayBufferExtension::Age;",
  "bool IsEmpty() const;",
  "size_t ApproximateBytes() const { return bytes_; }",
  "size_t BytesSlow() const;",
  "size_t Append(ArrayBufferExtension* extension);",
  "void Append(ArrayBufferList& list);",
  "bool ContainsSlow(ArrayBufferExtension* extension) const;",
  "ArrayBufferExtension* head_ = nullptr;",
  "ArrayBufferExtension* tail_ = nullptr;",
  "size_t bytes_ = 0;",
  "class ArrayBufferSweeper final {",
  "enum class SweepingType { kYoung, kFull };",
  "enum class TreatAllYoungAsPromoted { kNo, kYes };",
  "void RequestSweep(SweepingType sweeping_type,",
  "void EnsureFinished();",
  "void Append(ArrayBufferExtension* extension);",
  "void Resize(ArrayBufferExtension* extension, int64_t delta);",
  "void Detach(ArrayBufferExtension* extension);",
  "const ArrayBufferList& young() const { return young_; }",
  "size_t YoungBytes() const { return young().ApproximateBytes(); }",
  "bool sweeping_in_progress() const { return state_.get(); }",
  "int64_t young_bytes_adjustment_while_sweeping_{0};",
  "int64_t old_bytes_adjustment_while_sweeping_{0};",
  "uint64_t total_bytes_{0};",
]) {
  if (!h.includes(a)) throw new Error(`array-buffer-sweeper.h 锚缺失: ${a}`);
}
for (const a of [
  "size_t ArrayBufferList::Append(ArrayBufferExtension* extension) {",
  "void ArrayBufferList::Append(ArrayBufferList& list) {",
  "size_t ArrayBufferList::BytesSlow() const {",
  "void ArrayBufferSweeper::RequestSweep(",
  "void ArrayBufferSweeper::EnsureFinished() {",
  "void ArrayBufferSweeper::Finish() {",
  "void ArrayBufferSweeper::FinishIfDone() {",
  "void ArrayBufferSweeper::UpdateApproximateBytes(",
  "void ArrayBufferSweeper::IncrementExternalMemoryCounters(size_t bytes) {",
]) {
  if (!cc.includes(a)) throw new Error(`array-buffer-sweeper.cc 锚缺失: ${a}`);
}
// cc: Append(list) 记账 = bytes_ += list.ApproximateBytes()
if (!cc.includes("bytes_ += list.ApproximateBytes();")) throw new Error("Append(list) 记账漂移");
console.log("ArrayBufferSweeper: 枚举 3 + List 方法 5 + Sweeper 方法 12");

let rust = banner(
  "src/heap/array-buffer-sweeper.h.js",
  "src/heap/array-buffer-sweeper.{h,cc}",
  "ArrayBuffer 并发清扫: young/old 双链记账",
);

rust += `
/// 扩展年龄 (对应 src/objects/js-array-buffer.h ArrayBufferExtension::Age)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArrayBufferAge {
  YOUNG = 0,
  OLD = 1,
}

/// 清扫类型 (对应 ArrayBufferSweeper::SweepingType)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SweepingType {
  YOUNG = 0,
  FULL = 1,
}

/// 是否把全部 young 视作已晋升 (对应 TreatAllYoungAsPromoted)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TreatAllYoungAsPromoted {
  NO = 0,
  YES = 1,
}

/// 扩展链表 (对应 struct ArrayBufferList final)。
/// bytes_ 为近似值: 可能在扩展仍在链上时被先行扣减, 下次清扫时摘除。
/// v8: src/heap/array-buffer-sweeper.h ArrayBufferList
#[derive(Debug, Default)]
pub struct ArrayBufferListSkeleton {
  pub head: *mut (),
  pub tail: *mut (),
  pub bytes: usize,
  pub age: Option<ArrayBufferAge>,
}

impl ArrayBufferListSkeleton {
  /// 对应 Append(extension): 挂尾并累加 accounting_length, 返回旧 bytes
  /// v8: src/heap/array-buffer-sweeper.cc ArrayBufferList::Append
  pub fn append_extension(&mut self, extension_bytes: usize) -> usize {
    let old = self.bytes;
    self.bytes += extension_bytes;
    old
  }

  /// 对应 Append(list): 整链并入, bytes_ += list.ApproximateBytes()
  /// v8: src/heap/array-buffer-sweeper.cc ArrayBufferList::Append
  pub fn append_list(&mut self, list_approximate_bytes: usize) {
    self.bytes += list_approximate_bytes;
  }

  /// 对应 ApproximateBytes: 直接读 bytes_
  pub fn approximate_bytes(&self) -> usize {
    self.bytes
  }
}

/// 清扫器骨架 (对应 class ArrayBufferSweeper final)。
/// young_/old_ 双链; 清扫期间的调账挂 young/old_bytes_adjustment_while_sweeping_,
/// Finish 时统一并入。
/// v8: src/heap/array-buffer-sweeper.h ArrayBufferSweeper
pub struct ArrayBufferSweeperSkeleton {
  pub young: ArrayBufferListSkeleton,
  pub old: ArrayBufferListSkeleton,
  pub young_bytes_adjustment_while_sweeping: i64,
  pub old_bytes_adjustment_while_sweeping: i64,
  pub total_bytes: u64,
}

impl ArrayBufferSweeperSkeleton {
  /// 对应 UpdateApproximateBytes: 按 age 路由到 young/old 记账
  /// v8: src/heap/array-buffer-sweeper.cc ArrayBufferSweeper::UpdateApproximateBytes
  pub fn update_approximate_bytes(&mut self, delta: i64, age: ArrayBufferAge) {
    match age {
      ArrayBufferAge::YOUNG => {
        self.young.bytes = (self.young.bytes as i64 + delta).max(0) as usize;
      }
      ArrayBufferAge::OLD => {
        self.old.bytes = (self.old.bytes as i64 + delta).max(0) as usize;
      }
    }
  }

  /// 对应 YoungBytes: young().ApproximateBytes() (Minor GC 触发依据)
  pub fn young_bytes(&self) -> usize {
    self.young.approximate_bytes()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(ArrayBufferAge::YOUNG as u8, 0);
  assert_eq!(ArrayBufferAge::OLD as u8, 1);
  assert_eq!(SweepingType::FULL as u8, 1);
  assert_eq!(TreatAllYoungAsPromoted::YES as u8, 1);
  let mut l = ArrayBufferListSkeleton::default();
  assert_eq!(l.append_extension(100), 0);
  l.append_extension(28);
  assert_eq!(l.approximate_bytes(), 128);
  l.append_list(64);
  assert_eq!(l.approximate_bytes(), 192);
  let mut s = ArrayBufferSweeperSkeleton {
    young: ArrayBufferListSkeleton::default(),
    old: ArrayBufferListSkeleton::default(),
    young_bytes_adjustment_while_sweeping: 0,
    old_bytes_adjustment_while_sweeping: 0,
    total_bytes: 0,
  };
  s.update_approximate_bytes(50, ArrayBufferAge::YOUNG);
  s.update_approximate_bytes(70, ArrayBufferAge::OLD);
  assert_eq!(s.young_bytes(), 50);
  assert_eq!(s.old.approximate_bytes(), 70);
};
`;

const p = writeSrc("v8-heap-core", "array_buffer_sweeper.rs.skel", rust);
console.log(`written ${p}`);
