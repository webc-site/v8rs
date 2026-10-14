#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap-visitor.js — v8/src/heap/cppgc-internal/heap-visitor.h → v8-heap-cppgc/src/heap_visitor.rs.skel
// CRTP 堆访问者: Traverse 3 + 钩子 6 (返回 true 即剪枝) + 转发层 6
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap-visitor.h";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`heap-visitor.h 缺锚 ${label}: ${needle}`);
  n++;
};

need("template <typename Derived>\nclass HeapVisitor {", "HeapVisitor CRTP 模板");
// Traverse 3 重载
need("void Traverse(RawHeap& heap) {", "Traverse(RawHeap)");
need("void Traverse(BaseSpace& space) {", "Traverse(BaseSpace)");
need("void Traverse(BasePage& page) {", "Traverse(BasePage)");
// 钩子 6 (protected, 返回 true 停止下钻)
const HOOKS = ["bool VisitHeap(RawHeap&) { return false; }",
  "bool VisitNormalPageSpace(NormalPageSpace&) { return false; }",
  "bool VisitLargePageSpace(LargePageSpace&) { return false; }",
  "bool VisitNormalPage(NormalPage&) { return false; }",
  "bool VisitLargePage(LargePage&) { return false; }",
  "bool VisitHeapObjectHeader(HeapObjectHeader&) { return false; }"];
for (const h of HOOKS) need(h, "Visit 钩子");
// 转发层 6
const IMPLS = ["bool VisitHeapImpl(RawHeap& heap)", "bool VisitNormalPageSpaceImpl(NormalPageSpace& space)",
  "bool VisitLargePageSpaceImpl(LargePageSpace& space)", "bool VisitNormalPageImpl(NormalPage& page)",
  "bool VisitLargePageImpl(LargePage& page)", "bool VisitHeapObjectHeaderImpl(HeapObjectHeader& header)"];
for (const i of IMPLS) need(i, "Impl 转发");
need("Derived& ToDerived() { return static_cast<Derived&>(*this); }", "ToDerived");
// 遍历序断言: page.is_large() 分支 + header 区间循环
need("space.is_large()", "space 分派");
need("for (auto& header : *normal_page)", "normal page header 迭代");

const body = `${banner("src/heap/cppgc-internal/heap-visitor.js", REL, "CRTP 先序堆遍历骨架")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/heap-visitor.h HeapVisitor
/// CRTP: Rust 侧以 trait + 泛型自由函数呈现; 访问顺序: heap → space → page → header。
/// 钩子返回 true 表示不再下钻。
pub trait HeapVisitor {
    /// v8: HeapVisitor::VisitHeap
    fn visit_heap(&mut self, _heap: &mut RawHeap) -> bool { false }
    /// v8: HeapVisitor::VisitNormalPageSpace
    fn visit_normal_page_space(&mut self, _space: &mut NormalPageSpace) -> bool { false }
    /// v8: HeapVisitor::VisitLargePageSpace
    fn visit_large_page_space(&mut self, _space: &mut LargePageSpace) -> bool { false }
    /// v8: HeapVisitor::VisitNormalPage
    fn visit_normal_page(&mut self, _page: &mut NormalPage) -> bool { false }
    /// v8: HeapVisitor::VisitLargePage
    fn visit_large_page(&mut self, _page: &mut LargePage) -> bool { false }
    /// v8: HeapVisitor::VisitHeapObjectHeader
    fn visit_heap_object_header(&mut self, _header: &mut HeapObjectHeader) -> bool { false }
}

/// v8: src/heap/cppgc-internal/heap-visitor.h HeapVisitor::Traverse(RawHeap&)
pub fn traverse_heap<V: HeapVisitor>(v: &mut V, heap: &mut RawHeap) {
    if v.visit_heap(heap) { return; }
    for space in heap.spaces_mut() {
        traverse_space(v, space);
    }
}

/// v8: src/heap/cppgc-internal/heap-visitor.h HeapVisitor::Traverse(BaseSpace&)
pub fn traverse_space<V: HeapVisitor>(v: &mut V, space: &mut BaseSpace) {
    let stopped = if space.is_large() {
        v.visit_large_page_space(space.as_large().unwrap())
    } else {
        v.visit_normal_page_space(space.as_normal().unwrap())
    };
    if stopped { return; }
    for page in space.pages_mut() {
        traverse_page(v, page);
    }
}

/// v8: src/heap/cppgc-internal/heap-visitor.h HeapVisitor::Traverse(BasePage&)
pub fn traverse_page<V: HeapVisitor>(v: &mut V, page: &mut BasePage) {
    if page.is_large() {
        let large = page.as_large_mut().unwrap();
        if v.visit_large_page(large) { return; }
        v.visit_heap_object_header(large.object_header());
    } else {
        let normal = page.as_normal_mut().unwrap();
        if v.visit_normal_page(normal) { return; }
        for header in normal.iter_mut() {
            v.visit_heap_object_header(header);
        }
    }
}
`;

writeSrc("v8-heap-cppgc", "heap_visitor.rs.skel", body);
console.log(`heap-visitor.js: 对数 ${n} 项 (声明 1 + Traverse 3 + 钩子 6 + 转发 6 + 分派 2 + ToDerived 1) → v8-heap-cppgc/src/heap_visitor.rs.skel`);
