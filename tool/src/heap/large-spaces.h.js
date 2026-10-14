#!/usr/bin/env bun
// tool/src/heap/large-spaces.h.js
// 机械提取 v8/src/heap/large-spaces.{h,cc}: 大对象空间族类清单 (7 空间类 + 迭代器)
// 与 LargeObjectSpace 方法台账; cc 侧 AddPage/RemovePage/UpdateAccounting 记账规则。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/large-spaces.h");
const cc = readV8("src/heap/large-spaces.cc");

// 类层级: 7 空间类 + 迭代器
for (const a of [
  "class V8_EXPORT_PRIVATE LargeObjectSpace : public Space {",
  "class OldLargeObjectSpace : public LargeObjectSpace {",
  "class SharedLargeObjectSpace : public OldLargeObjectSpace {",
  "class TrustedLargeObjectSpace : public OldLargeObjectSpace {",
  "class SharedTrustedLargeObjectSpace : public OldLargeObjectSpace {",
  "class NewLargeObjectSpace : public LargeObjectSpace {",
  "class CodeLargeObjectSpace : public OldLargeObjectSpace {",
  "class LargeObjectSpaceObjectIterator : public ObjectIterator {",
]) {
  if (!h.includes(a)) throw new Error(`large-spaces.h 类锚缺失: ${a}`);
}
// LargeObjectSpace 方法锚
for (const a of [
  "void TearDown();",
  "size_t Available() const override;",
  "size_t Size() const override { return size_; }",
  "size_t SizeOfObjects() const override { return objects_size_; }",
  "int PageCount() const { return page_count_; }",
  "void ShrinkPageToObjectSize(LargePage* page, Tagged<HeapObject> object,",
  "bool Contains(Tagged<HeapObject> obj) const;",
  "bool ContainsSlow(Address addr) const;",
  "bool IsEmpty() const { return first_page() == nullptr; }",
  "virtual void AddPage(LargePage* page, size_t object_size);",
  "virtual void RemovePage(LargePage* page);",
  "void UpdateAccountingAfterResizingObject(size_t old_size, size_t new_size);",
  "std::atomic<size_t> size_;  // allocated bytes",
  "int page_count_;       // number of chunks",
  "std::atomic<size_t> objects_size_;  // size of objects",
  "base::RecursiveMutex allocation_mutex_;",
  "size_t capacity_;",
]) {
  if (!h.includes(a)) throw new Error(`large-spaces.h 方法锚缺失: ${a}`);
}
// NewLargeObjectSpace 专属: Flip / FreeDeadObjects / SetCapacity / Available
for (const a of [
  "void Flip();",
  "void FreeDeadObjects(const std::function<bool(Tagged<HeapObject>)>& is_dead);",
  "void SetCapacity(size_t capacity);",
]) {
  if (!h.includes(a)) throw new Error(`NewLargeObjectSpace 锚缺失: ${a}`);
}
// cc: 记账规则锚 (AddPage: size_/objects_size_/page_count_ 增; RemovePage 反向;
// UpdateAccountingAfterResizingObject: objects_size_ += new-old)
for (const a of [
  "void LargeObjectSpace::AddPage(LargePage* page, size_t object_size) {",
  "void LargeObjectSpace::RemovePage(LargePage* page) {",
  "void LargeObjectSpace::UpdateAccountingAfterResizingObject(",
  "void NewLargeObjectSpace::Flip() {",
  "void NewLargeObjectSpace::FreeDeadObjects(",
  "void NewLargeObjectSpace::SetCapacity(size_t capacity) {",
]) {
  if (!cc.includes(a)) throw new Error(`large-spaces.cc 锚缺失: ${a}`);
}
console.log("大对象空间族: 类 8 + 方法/字段锚 20");

let rust = banner(
  "src/heap/large-spaces.h.js",
  "src/heap/large-spaces.{h,cc}",
  "大对象空间族: 7 空间类台账与页记账规则",
);

rust += `
/// 大对象空间记账状态 (对应 LargeObjectSpace 字段组)。
/// size_ = 已分配字节; objects_size_ = 对象实体字节; page_count_ = 页数。
/// v8: src/heap/large-spaces.h LargeObjectSpace
#[derive(Debug, Default)]
pub struct LargeSpaceAccounting {
  pub size: core::sync::atomic::AtomicUsize,
  pub objects_size: core::sync::atomic::AtomicUsize,
  pub page_count: i32,
}

impl LargeSpaceAccounting {
  /// 对应 AddPage: 三计数随页加入递增
  /// v8: src/heap/large-spaces.cc LargeObjectSpace::AddPage
  pub fn add_page(&self, object_size: usize) {
    self.size.fetch_add(object_size, core::sync::atomic::Ordering::Relaxed);
    self.objects_size.fetch_add(object_size, core::sync::atomic::Ordering::Relaxed);
    self.page_count += 1;
  }

  /// 对应 RemovePage: 反向递减
  /// v8: src/heap/large-spaces.cc LargeObjectSpace::RemovePage
  pub fn remove_page(&self, object_size: usize) {
    self.size.fetch_sub(object_size, core::sync::atomic::Ordering::Relaxed);
    self.objects_size.fetch_sub(object_size, core::sync::atomic::Ordering::Relaxed);
    self.page_count -= 1;
  }

  /// 对应 UpdateAccountingAfterResizingObject: objects_size_ += new - old
  /// (对象原地改大小后补差)
  /// v8: src/heap/large-spaces.cc LargeObjectSpace::UpdateAccountingAfterResizingObject
  pub fn update_after_resizing_object(&self, old_size: usize, new_size: usize) {
    self.objects_size.fetch_add(new_size - old_size, core::sync::atomic::Ordering::Relaxed);
  }

  /// 对应 Size() { return size_; }
  pub fn size(&self) -> usize {
    self.size.load(core::sync::atomic::Ordering::Relaxed)
  }

  /// 对应 SizeOfObjects() { return objects_size_; }
  pub fn size_of_objects(&self) -> usize {
    self.objects_size.load(core::sync::atomic::Ordering::Relaxed)
  }

  /// 对应 PageCount()
  pub fn page_count(&self) -> i32 {
    self.page_count
  }
}

/// 新生代大对象空间骨架 (对应 class NewLargeObjectSpace)。
/// Flip (Scavenge 换 semispace) / FreeDeadObjects / SetCapacity 台账。
/// v8: src/heap/large-spaces.h NewLargeObjectSpace
pub struct NewLargeObjectSpaceSkeleton {
  pub accounting: LargeSpaceAccounting,
  pub capacity: usize,
}

impl NewLargeObjectSpaceSkeleton {
  /// 对应 Available: capacity - objects_size_ (不足则 0)
  /// v8: src/heap/large-spaces.cc NewLargeObjectSpace::Available
  #[must_use]
  pub fn available(&self) -> usize {
    self.capacity.saturating_sub(self.accounting.size_of_objects())
  }

  /// 对应 SetCapacity
  /// v8: src/heap/large-spaces.cc NewLargeObjectSpace::SetCapacity
  pub fn set_capacity(&mut self, capacity: usize) {
    self.capacity = capacity;
  }
}

// —— 提取不变式断言 ——
const _: () = {
  let a = LargeSpaceAccounting::default();
  a.add_page(100);
  a.add_page(28);
  assert_eq!(a.size(), 128);
  assert_eq!(a.size_of_objects(), 128);
  assert_eq!(a.page_count(), 2);
  a.update_after_resizing_object(100, 60);
  assert_eq!(a.size_of_objects(), 88);
  a.remove_page(60);
  assert_eq!(a.page_count(), 1);
  let s = NewLargeObjectSpaceSkeleton { accounting: LargeSpaceAccounting::default(), capacity: 256 };
  s.accounting.add_page(300);
  assert_eq!(s.available(), 0);
};
`;

const p = writeSrc("v8-heap-core", "large_spaces.rs.skel", rust);
console.log(`written ${p}`);
