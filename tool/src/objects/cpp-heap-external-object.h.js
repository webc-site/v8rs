#!/usr/bin/env bun
// tool/src/objects/cpp-heap-external-object.h.js — 覆盖 v8/src/objects/cpp-heap-external-object.{h,tq}
// → v8-objects-js-async/src/cpp_heap_external_object.rs
// 机械提取: CppHeapExternalObject 布局 (h ↔ tq 交叉, 1 字段) + kHeaderSize 公式

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/cpp-heap-external-object.h");
const tq = readV8("src/objects/cpp-heap-external-object.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[cpp-heap-external-object.h.js] ${m}`); };

// ── 1. .h 类面 ───────────────────────────────────────────────────────────
must(h.includes("V8_OBJECT class CppHeapExternalObject : public HeapObject {"), "h 缺 CppHeapExternalObject");
must(h.includes("  CppHeapPointerMember cpp_heap_wrappable_;"), "h 缺 cpp_heap_wrappable_ 成员");
must(h.includes("inline constexpr int CppHeapExternalObject::kHeaderSize =\n    sizeof(CppHeapExternalObject);"),
  "h 缺 kHeaderSize = sizeof 公式");
for (const s of ["DECL_PRINTER(CppHeapExternalObject)", "DECL_VERIFIER(CppHeapExternalObject)", "class BodyDescriptor;"]) {
  must(h.includes(s), `h 缺 ${s}`);
}

// ── 2. tq 布局交叉 (1 字段) ──────────────────────────────────────────────
const cls = tq.match(/extern class CppHeapExternalObject extends HeapObject \{([\s\S]*?)\}/);
must(cls, "tq 缺 CppHeapExternalObject");
const fields = [...cls[1].matchAll(/^  (\w+): ([^;]+);$/gm)].map((m) => [m[1], m[2].trim()]);
must(fields.length === 1, `tq 字段数 ${fields.length} != 1`);
must(fields[0][0] === "cpp_heap_wrappable" && fields[0][1] === "CppHeapPointer", "tq 布局漂移");

let rust = banner("src/objects/cpp-heap-external-object.h.js", "src/objects/cpp-heap-external-object.{h,tq}",
  "CppHeap 外部对象: 纯 C++ 堆包装指针的最小堆载体");
rust += `//! CppHeapExternalObject: 无 JS 字段的 cppgc 对象包装载体。
//!
//! 仅一个 CppHeapPointerMember 槽 (经 cpp 堆指针表间接寻址), 供
//! CppHeapObjectWrapper 按偏移统一读写 (见 cpp_heap_object_wrapper.rs)。

// ── 布局 (h 与 tq 1:1) ──
pub const CPP_HEAP_EXTERNAL_OBJECT_FIELDS: [(&str, &str); ${fields.length}] = [
`;
for (const f of fields) rust += `  ("${f[0]}", "${f[1]}"),\n`;
rust += `];

/// 头大小公式 (原版 kHeaderSize = sizeof(CppHeapExternalObject); 唯一字段即尾)。
#[inline]
#[must_use]
pub const fn k_header_size(cpp_heap_pointer_size: usize) -> usize {
  cpp_heap_pointer_size
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(CPP_HEAP_EXTERNAL_OBJECT_FIELDS.len() == 1);
  assert!(CPP_HEAP_EXTERNAL_OBJECT_FIELDS[0].1 == "CppHeapPointer");
  assert!(k_header_size(8) == 8);
};
`;

const p = writeSrc("v8-objects-js-async", "cpp_heap_external_object.rs", rust);
console.log(`cpp-heap-external-object.h.js: ${p} — 布局 ${fields.length} 字段 + kHeaderSize 公式, 断言 ${checks} 项全过`);
