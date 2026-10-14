#!/usr/bin/env bun
// tool/src/heap/gc-callbacks-inl.h.js
// 机械提取 v8/src/heap/gc-callbacks-inl.h: GCRootsProviderScope ctor/dtor
// (LocalHeap::roots_providers_ 栈式 push/pop + DEBUG back 一致性断言)。
// 注: gc-callbacks.h/cc 由既有 gc-callbacks.h.js 覆盖 (禁碰), 本脚本只做 -inl 差集。

import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/heap/gc-callbacks-inl.h");

for (const a of [
  "GCRootsProviderScope::GCRootsProviderScope(LocalIsolate* local_isolate,",
  "GCRootsProviderScope::GCRootsProviderScope(LocalHeap* local_heap,",
  "GCRootsProviderScope::~GCRootsProviderScope() {",
  "DCHECK_EQ(provider_, local_heap_->roots_providers_.back());",
  "local_heap_->roots_providers_.push_back(provider);",
  "local_heap_->roots_providers_.pop_back();",
]) {
  if (!inl.includes(a)) throw new Error(`gc-callbacks-inl.h 锚点缺失: ${a}`);
}
// DEBUG 条件臂: provider_ 仅 DEBUG 记录
if (!inl.includes("#if DEBUG")) throw new Error("DEBUG 臂缺失");
console.log("GCRootsProviderScope: ctor 2 重载 + dtor 1");

let rust = banner(
  "src/heap/gc-callbacks-inl.h.js",
  "src/heap/gc-callbacks-inl.h",
  "GC 根提供者作用域: roots_providers_ 栈式注册/注销",
);

rust += `
/// 根提供者作用域 (对应 class GCRootsProviderScope, gc-callbacks.h 声明,
/// -inl.h 定义)。RAII: 构造 push 到 local_heap 的 roots_providers_,
/// 析构 pop 并 (DEBUG 下) 断言弹出的正是自己。
/// v8: src/heap/gc-callbacks-inl.h GCRootsProviderScope
pub struct GCRootsProviderScope<'a> {
  local_heap: &'a mut Vec<*mut ()>,
  #[cfg(debug_assertions)]
  provider: *mut (),
}

impl<'a> GCRootsProviderScope<'a> {
  /// 对应 GCRootsProviderScope(LocalHeap*, GCRootsProvider*):
  /// local_heap_->roots_providers_.push_back(provider)
  /// v8: src/heap/gc-callbacks-inl.h GCRootsProviderScope::GCRootsProviderScope
  pub fn enter(local_heap: &'a mut Vec<*mut ()>, provider: *mut ()) -> Self {
    local_heap.push(provider);
    Self {
      local_heap,
      #[cfg(debug_assertions)]
      provider,
    }
  }
}

impl Drop for GCRootsProviderScope<'_> {
  /// 对应 ~GCRootsProviderScope(): DCHECK_EQ(provider_, back()) 后 pop_back()
  /// v8: src/heap/gc-callbacks-inl.h GCRootsProviderScope::~GCRootsProviderScope
  fn drop(&mut self) {
    #[cfg(debug_assertions)]
    {
      debug_assert_eq!(self.local_heap.last().copied(), Some(self.provider));
    }
    self.local_heap.pop();
  }
}

// —— 提取不变式断言 (栈式配对) ——
const _: () = {
  let mut providers: Vec<*mut ()> = Vec::new();
  {
    let _s = GCRootsProviderScope::enter(&mut providers, 7 as *mut ());
    assert_eq!(providers.len(), 1);
  }
  assert!(providers.is_empty());
};
`;

const p = writeSrc("v8-heap-core", "gc_callbacks_inl.rs.skel", rust);
console.log(`written ${p}`);
