#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap-consistency.js — v8/src/heap/cppgc-internal/heap-consistency.cc → v8-heap-cppgc/src/heap_consistency.rs.skel
// 一致性作用域: DisallowGarbageCollectionScope / NoGarbageCollectionScope 两类 RAII 门
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap-consistency.cc";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`heap-consistency.cc 缺锚 ${label}: ${needle}`);
  n++;
};

// DisallowGarbageCollectionScope: 静态 3 + RAII 2
need("bool DisallowGarbageCollectionScope::IsGarbageCollectionAllowed(", "D::IsGarbageCollectionAllowed");
need("!heap_base.IsGCForbidden();", "IsGCForbidden 取反");
need("void DisallowGarbageCollectionScope::Enter(cppgc::HeapHandle& heap_handle) {", "D::Enter");
need("heap_base.EnterDisallowGCScope();", "EnterDisallowGCScope");
need("void DisallowGarbageCollectionScope::Leave(cppgc::HeapHandle& heap_handle) {", "D::Leave");
need("heap_base.LeaveDisallowGCScope();", "LeaveDisallowGCScope");
need("DisallowGarbageCollectionScope::DisallowGarbageCollectionScope(", "D 构造");
need("DisallowGarbageCollectionScope::~DisallowGarbageCollectionScope() {", "D 析构");
// NoGarbageCollectionScope: 静态 2 + RAII 2
need("void NoGarbageCollectionScope::Enter(cppgc::HeapHandle& heap_handle) {", "N::Enter");
need("heap_base.EnterNoGCScope();", "EnterNoGCScope");
need("void NoGarbageCollectionScope::Leave(cppgc::HeapHandle& heap_handle) {", "N::Leave");
need("heap_base.LeaveNoGCScope();", "LeaveNoGCScope");
need("NoGarbageCollectionScope::NoGarbageCollectionScope(", "N 构造");
need("NoGarbageCollectionScope::~NoGarbageCollectionScope() { Leave(heap_handle_); }", "N 析构内联");

const body = `${banner("src/heap/cppgc-internal/heap-consistency.js", REL, "GC 禁入/禁发 RAII 作用域")}
#![allow(dead_code)]

/// v8: include/cppgc/heap-consistency.h DisallowGarbageCollectionScope — 实现于本 cc
pub struct DisallowGarbageCollectionScope<'a> {
    /// v8: heap_handle_
    heap_handle: &'a mut HeapHandle,
}

impl<'a> DisallowGarbageCollectionScope<'a> {
    /// v8: src/heap/cppgc-internal/heap-consistency.cc DisallowGarbageCollectionScope::IsGarbageCollectionAllowed
    pub fn is_garbage_collection_allowed(heap_handle: &HeapHandle) -> bool {
        !HeapBase::from(heap_handle).is_gc_forbidden()
    }

    /// v8: src/heap/cppgc-internal/heap-consistency.cc DisallowGarbageCollectionScope::Enter
    pub fn enter(heap_handle: &mut HeapHandle) {
        HeapBase::from_mut(heap_handle).enter_disallow_gc_scope();
    }

    /// v8: src/heap/cppgc-internal/heap-consistency.cc DisallowGarbageCollectionScope::Leave
    pub fn leave(heap_handle: &mut HeapHandle) {
        HeapBase::from_mut(heap_handle).leave_disallow_gc_scope();
    }

    /// v8: src/heap/cppgc-internal/heap-consistency.cc DisallowGarbageCollectionScope 构造
    pub fn new(heap_handle: &'a mut HeapHandle) -> Self {
        Self::enter(heap_handle);
        Self { heap_handle }
    }
}

impl Drop for DisallowGarbageCollectionScope<'_> {
    /// v8: src/heap/cppgc-internal/heap-consistency.cc ~DisallowGarbageCollectionScope
    fn drop(&mut self) {
        Self::leave(self.heap_handle);
    }
}

/// v8: include/cppgc/heap-consistency.h NoGarbageCollectionScope — 实现于本 cc
pub struct NoGarbageCollectionScope<'a> {
    /// v8: heap_handle_
    heap_handle: &'a mut HeapHandle,
}

impl<'a> NoGarbageCollectionScope<'a> {
    /// v8: src/heap/cppgc-internal/heap-consistency.cc NoGarbageCollectionScope::Enter
    pub fn enter(heap_handle: &mut HeapHandle) {
        HeapBase::from_mut(heap_handle).enter_no_gc_scope();
    }

    /// v8: src/heap/cppgc-internal/heap-consistency.cc NoGarbageCollectionScope::Leave
    pub fn leave(heap_handle: &mut HeapHandle) {
        HeapBase::from_mut(heap_handle).leave_no_gc_scope();
    }

    /// v8: src/heap/cppgc-internal/heap-consistency.cc NoGarbageCollectionScope 构造
    pub fn new(heap_handle: &'a mut HeapHandle) -> Self {
        Self::enter(heap_handle);
        Self { heap_handle }
    }
}

impl Drop for NoGarbageCollectionScope<'_> {
    /// v8: src/heap/cppgc-internal/heap-consistency.cc ~NoGarbageCollectionScope
    fn drop(&mut self) {
        Self::leave(self.heap_handle);
    }
}
`;

writeSrc("v8-heap-cppgc", "heap_consistency.rs.skel", body);
console.log(`heap-consistency.js: 对数 ${n} 项 (D 8 + N 6) → v8-heap-cppgc/src/heap_consistency.rs.skel`);
