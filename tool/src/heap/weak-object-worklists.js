#!/usr/bin/env bun
// tool/src/heap/weak-object-worklists.js
// 机械提取 v8/src/heap/weak-object-worklists{.h,.cc} 族:
// WEAK_OBJECT_WORKLISTS X-macro 展开 → WeakObjects/Local 结构

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/weak-object-worklists.h");
const ccSrc = readV8("src/heap/weak-object-worklists.cc");

const anchors = [
  [hSrc, "struct Ephemeron {"],
  [hSrc, "struct HeapObjectAndCode {"],
  [hSrc, "struct DispatchHandleAndCode {"],
  [hSrc, "#define WEAK_OBJECT_WORKLISTS(F)"],
  [hSrc, "F(Tagged<TransitionArray>, transition_arrays, TransitionArrays)"],
  [hSrc, "F(Ephemeron, current_ephemerons, CurrentEphemerons)"],
  [hSrc, "F(Ephemeron, next_ephemerons, NextEphemerons)"],
  [hSrc, "F(Tagged<JSFunction>, flushed_js_functions, FlushedJSFunctions)"],
  [hSrc, "using WeakObjectWorklist = ::heap::base::Worklist<Type, 64>;"],
  [hSrc, "class Local final : public UnusedBase {"],
  [ccSrc, "void WeakObjects::Clear() {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// X-macro 展开 (跨行 F(...) 形态)
const def = hSrc.match(/#define WEAK_OBJECT_WORKLISTS\(F\)([\s\S]*?)\n\nclass WeakObjects/);
if (!def) throw new Error("WEAK_OBJECT_WORKLISTS 体未匹配");
const clean = def[1].replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/[^\n]*/g, "");
const items = [...clean.matchAll(/F\(([^)]*)\)/g)].map((m) =>
  m[1].split(",").map((s) => s.trim()).filter(Boolean)
);
if (items.length !== 14) throw new Error(`worklist 计数 ${items.length} != 14`);
for (const it of items) if (it.length !== 3) throw new Error(`F 项形参错: ${it}`);
console.log(`weak-object-worklists: worklist ${items.length} 项, 段大小 64, 结构 4 (Ephemeron/HeapObjectAndCode/DispatchHandleAndCode/POD)`);

let rust = banner(
  "src/heap/weak-object-worklists.js",
  "src/heap/weak-object-worklists.h, src/heap/weak-object-worklists.cc",
  "弱对象工作表集合 (X-macro 展开)",
);

rust += `/// v8: src/heap/weak-object-worklists.h Ephemeron
pub struct Ephemeron {
  pub key: TaggedHeapObject,
  pub value: TaggedHeapObject,
}

/// v8: src/heap/weak-object-worklists.h HeapObjectAndCode
pub struct HeapObjectAndCode {
  pub heap_object: TaggedHeapObject,
  pub code: TaggedCode,
}

/// v8: src/heap/weak-object-worklists.h DispatchHandleAndCode
pub struct DispatchHandleAndCode {
  pub dispatch_handle: JSDispatchHandle,
  pub code: TaggedCode,
}

/// v8: WEAK_OBJECT_WORKLISTS 展开 — ${items.length} 个 worklist, 段 64:
`;
for (const [type, name] of items) {
  rust += `///   ${name}: Worklist<${type.replace(/Tagged<(\w+)>/, "$1")}, 64>\n`;
}

rust += `
/// v8: src/heap/weak-object-worklists.h WeakObjects — 上述全部全局弱表
pub struct WeakObjects {
`;
for (const [, name] of items) rust += `  /// v8: ${name}\n  pub ${name}: WeakObjectWorklistEntry,\n`;
rust += `}

/// v8: src/heap/weak-object-worklists.h WeakObjects::Local — 线程本地 + Publish
pub struct WeakObjectsLocal {
`;
for (const [, name] of items) rust += `  /// v8: ${name}_local\n  pub ${name}_local: WeakObjectWorklistLocalEntry,\n`;
rust += `}

impl WeakObjects {
  /// v8: src/heap/weak-object-worklists.cc WeakObjects::Clear
  pub fn clear(&mut self) { todo!() }
}

// —— 提取不变式断言 ——
const _: () = {
  // WEAK_OBJECT_WORKLISTS 展开项数固定 ${items.length}
};
`;

const p = writeSrc("v8-heap-core", "weak_object_worklists.rs.skel", rust);
console.log(`written ${p}`);
