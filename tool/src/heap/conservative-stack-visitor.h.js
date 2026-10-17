#!/usr/bin/env bun
// tool/src/heap/conservative-stack-visitor.h.js
// 机械提取 v8/src/heap/conservative-stack-visitor.{h,-inl.h}:
// ConservativeStackVisitorBase CRTP 骨架 → Rust trait; 5 个定制点
// (OnlyVisitMainV8Heap/FilterPage/FilterLargeObject/FilterNormalObject/HandleObjectFound)
// + ConservativeStackVisitor 具体实现; IsInterestingCage 三 cage 判定。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/conservative-stack-visitor.h");
const inl = readV8("src/heap/conservative-stack-visitor-inl.h");

// —— 锚点断言: 基类签名与定制点 (h 头注释列 5 点) ——
for (const a of [
  "template <typename ConcreteVisitor>",
  "class V8_EXPORT_PRIVATE ConservativeStackVisitorBase",
  "Address FindBasePtr(Address maybe_inner_ptr,",
  "void VisitPointer(const void* pointer) final;",
  "void VisitConservativelyIfPointer(Address address);",
  "bool IsInterestingCage(PtrComprCageBase cage_base) const;",
  "const PtrComprCageBase cage_base_;",
  "const PtrComprCageBase code_cage_base_;",
  "const PtrComprCageBase trusted_cage_base_;",
]) {
  if (!h.includes(a)) throw new Error(`conservative-stack-visitor.h 锚点缺失: ${a}`);
}
// 具体实现的 5 个定制点
for (const a of [
  "class V8_EXPORT_PRIVATE ConservativeStackVisitor",
  "static constexpr bool kOnlyVisitMainV8Cage = false;",
  "static bool FilterPage(const MemoryChunk* chunk) {",
  "return v8_flags.sticky_mark_bits || !chunk->IsFromPage();",
  "static bool FilterLargeObject(Tagged<HeapObject>, MapWord) { return true; }",
  "static bool FilterNormalObject(Tagged<HeapObject>, MapWord, MarkingBitmap*) {",
  "static void HandleObjectFound(Tagged<HeapObject>, size_t, MarkingBitmap*) {}",
]) {
  if (!h.includes(a)) throw new Error(`ConservativeStackVisitor 定制点锚缺失: ${a}`);
}
// inl: FindBasePtr 主流程锚
for (const a of [
  "ConservativeStackVisitorBase<ConcreteVisitor>::FindBasePtr(",
  "if (!ConcreteVisitor::FilterPage(chunk)) {",
  "ConcreteVisitor::FilterLargeObject(obj, map_word)",
  "ConcreteVisitor::FilterNormalObject(obj, map_word, bitmap)",
  "ConcreteVisitor::HandleObjectFound(obj, size, bitmap);",
  "MarkingBitmap::FindPreviousValidObject(page, maybe_inner_ptr);",
  "::VisitPointer(",
  "ConcreteVisitor::kOnlyVisitMainV8Cage",
  "if (cage_base == cage_base_) {",
  "if (cage_base == code_cage_base_) {",
  "if (cage_base == trusted_cage_base_) {",
]) {
  if (!inl.includes(a)) throw new Error(`conservative-stack-visitor-inl.h 锚点缺失: ${a}`);
}
console.log("CSVBase 方法 4 + 定制点 5 + IsInterestingCage 3 cage");

let rust = banner(
  "src/heap/conservative-stack-visitor.h.js",
  "src/heap/conservative-stack-visitor.{h,-inl.h}",
  "保守栈扫描: 基址归属判定骨架与定制点 trait",
);

rust += `
/// 保守栈访问者定制点 (对应 CRTP ConcreteVisitor 的 5 个 static 钩子)。
/// v8: src/heap/conservative-stack-visitor.h ConservativeStackVisitorBase 头注释
pub trait ConservativeVisitorHooks {
  /// 对应 static constexpr bool kOnlyVisitMainV8Cage (默认 false:
  /// 同时处理外部代码空间与可信空间 cage)
  const ONLY_VISIT_MAIN_V8_CAGE: bool = false;

  /// 对应 FilterPage: sticky_mark_bits 或非 from-page 才扫
  /// (具体 ConservativeStackVisitor: v8_flags.sticky_mark_bits || !chunk->IsFromPage())
  fn filter_page(sticky_mark_bits: bool, is_from_page: bool) -> bool {
    sticky_mark_bits || !is_from_page
  }

  /// 对应 FilterLargeObject(Tagged<HeapObject>, MapWord) — 默认全收
  fn filter_large_object() -> bool {
    true
  }

  /// 对应 FilterNormalObject(Tagged<HeapObject>, MapWord, MarkingBitmap*) — 默认全收
  fn filter_normal_object() -> bool {
    true
  }

  /// 对应 HandleObjectFound(Tagged<HeapObject>, size_t, MarkingBitmap*) — 默认空
  fn handle_object_found(_object: (), _size: usize, _bitmap: ()) {}
}

/// 保守栈访问者基类骨架 (对应 ConservativeStackVisitorBase<ConcreteVisitor>)。
/// v8: src/heap/conservative-stack-visitor.h ConservativeStackVisitorBase
pub struct ConservativeStackVisitorBaseSkeleton {
  /// 对应 cage_base_ (V8 主 cage)
  pub cage_base: u64,
  /// 对应 code_cage_base_ (V8_EXTERNAL_CODE_SPACE)
  pub code_cage_base: u64,
  /// 对应 trusted_cage_base_ (V8_ENABLE_SANDBOX)
  pub trusted_cage_base: u64,
}

impl ConservativeStackVisitorBaseSkeleton {
  /// 对应 IsInterestingCage: 主/代码/可信三 cage 任一相等即真
  /// v8: src/heap/conservative-stack-visitor-inl.h IsInterestingCage
  #[must_use]
  pub fn is_interesting_cage(&self, cage_base: u64) -> bool {
    cage_base == self.cage_base
      || cage_base == self.code_cage_base
      || cage_base == self.trusted_cage_base
  }

  /// 对应 FindBasePtr: 内部指针 → 对象头; 命中 FilterPage/LargeObject/
  /// NormalObject 三过滤后 HandleObjectFound
  /// v8: src/heap/conservative-stack-visitor-inl.h FindBasePtr
  pub fn find_base_ptr(&self, _maybe_inner_ptr: u64) -> u64 {
    unimplemented!("骨架: 页归属/位图回溯待 heap-layout 与 marking 族就位");
  }
}

/// 默认具体访问者 (对应 class ConservativeStackVisitor)。
pub type DefaultHooks = ();

impl ConservativeVisitorHooks for () {}

// —— 提取不变式断言 ——
const _: () = {
  struct H;
  impl ConservativeVisitorHooks for H {}
  assert!(!H::ONLY_VISIT_MAIN_V8_CAGE);
  assert!(H::filter_page(false, false)); // 非 from-page → 扫
  assert!(H::filter_page(true, true)); // sticky_mark_bits → 扫
  assert!(!H::filter_page(false, true)); // from-page 且非 sticky → 跳过
  assert!(H::filter_large_object());
  assert!(H::filter_normal_object());
  let b = ConservativeStackVisitorBaseSkeleton {
    cage_base: 0x1000,
    code_cage_base: 0x2000,
    trusted_cage_base: 0x3000,
  };
  assert!(b.is_interesting_cage(0x1000));
  assert!(b.is_interesting_cage(0x3000));
  assert!(!b.is_interesting_cage(0x4000));
};
`;

const p = writeSrc("v8-heap-core", "conservative_stack_visitor.rs.skel", rust);
console.log(`written ${p}`);
