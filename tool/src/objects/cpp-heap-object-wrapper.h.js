#!/usr/bin/env bun
// tool/src/objects/cpp-heap-object-wrapper.h.js — 覆盖 v8/src/objects/cpp-heap-object-wrapper.{h,-inl.h}
// → v8-objects-js-async/src/cpp_heap_object_wrapper.rs
// 机械提取: CppHeapPointerWrapperObjectT union 5 成员 + CppHeapObjectWrapper 类面
//   + 三源 offset 表 (inl: From 分派 + 按对象类别取 cpp_heap_wrappable_ 偏移)

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/cpp-heap-object-wrapper.h");
const inl = readV8("src/objects/cpp-heap-object-wrapper-inl.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[cpp-heap-object-wrapper.h.js] ${m}`); };

// ── 1. 可包装对象 union (5 类型) ─────────────────────────────────────────
const uni = h.match(/using CppHeapPointerWrapperObjectT =\s*\n\s*UnionOf<([\s\S]*?)>;/);
must(uni, "h 缺 CppHeapPointerWrapperObjectT union");
const uniTypes = uni[1].split(",").map((s) => s.trim()).filter(Boolean);
must(uniTypes.length === 5, `union 成员数 ${uniTypes.length} != 5`);
must(uniTypes.join(",") === "JSObject,CppHeapExternalObject,NativeContext,CppGCManagedBase,EmbedderDataArray",
  `union 成员漂移: ${uniTypes.join(",")}`);

// ── 2. CppHeapObjectWrapper 类面 (2 字段: object_ + offset_) ──────────────
must(h.includes("class CppHeapObjectWrapper {"), "h 缺 CppHeapObjectWrapper");
must(h.includes("  Tagged<CppHeapPointerWrapperObjectT> object_;"), "h 缺 object_ 字段");
must(h.includes("  int offset_;"), "h 缺 offset_ 字段");
for (const s of [
  "V8_INLINE static CppHeapObjectWrapper From(",
  "V8_INLINE explicit CppHeapObjectWrapper(Tagged<JSObject> object);",
  "V8_INLINE explicit CppHeapObjectWrapper(Tagged<CppHeapExternalObject> object);",
  "V8_INLINE explicit CppHeapObjectWrapper(Tagged<CppGCManagedBase> object);",
  "V8_INLINE void InitializeCppHeapWrapper();",
  "V8_INLINE void SetCppHeapWrappable(IsolateForPointerCompression isolate,",
  "V8_INLINE void* GetCppHeapWrappable(",
]) must(h.includes(s), `h 缺 ${s}`);

// ── 3. .h static_assert: JSAPIObjectWithEmbedderSlots 与 JSSpecialObject 槽位对齐 ──
must(h.includes("static_assert(offsetof(JSAPIObjectWithEmbedderSlots, cpp_heap_wrappable_) =="), "h 缺 offset 对齐断言");
must(h.includes("offsetof(JSSpecialObject, cpp_heap_wrappable_));"), "h 缺 offset 对齐断言第二行");

// ── 4. -inl.h: 三源 offset + From 分派 ──────────────────────────────────
must(inl.includes("CppHeapObjectWrapper CppHeapObjectWrapper::From("), "inl 缺 From");
const fromBranches = [...inl.matchAll(/if \(Is(?:CppHeapExternalObject|CppGCManagedBase)\(object\)\)/g)].length;
must(fromBranches === 2, `From 分派分支数 ${fromBranches} != 2 (ExternalObject + CppGCManagedBase)`);
for (const s of [
  "offsetof(CppHeapExternalObject, cpp_heap_wrappable_)",
  "offsetof(CppGCManagedBase, cpp_gc_wrapper_)",
  "offsetof(JSAPIObjectWithEmbedderSlots, cpp_heap_wrappable_)",
  "object_->SetupLazilyInitializedCppHeapPointerField(offset_);",
  "object_->ReadCppHeapPointerField<lower_bound, upper_bound>(offset_,",
  "object_->WriteLazilyInitializedCppHeapPointerField(",
  "WriteBarrier::ForCppHeapPointer(",
]) must(inl.includes(s), `inl 缺 ${s}`);

let rust = banner("src/objects/cpp-heap-object-wrapper.h.js", "src/objects/cpp-heap-object-wrapper.{h,-inl.h}",
  "cpp 堆包装统一读写面: union 5 类对象按各自偏移访问同一指针槽");
rust += `//! CppHeapObjectWrapper: 按 (object, offset) 二元组统一读写 cpp 堆指针。
//!
//! 五种宿主对象各有 cpp_heap_wrappable 等价槽; 包装器记录对象类别对应的
//! 字段偏移, 读写与惰性初始化全部转发到该偏移。

// ── 可包装对象 union (原版 CppHeapPointerWrapperObjectT, 5 类型) ──
pub const CPP_HEAP_POINTER_WRAPPER_OBJECT_TYPES: [&str; ${uniTypes.length}] = [
`;
for (const t of uniTypes) rust += `  "${t}",\n`;
rust += `];

// ── 包装器两字段 (object_ + offset_) ──
pub const CPP_HEAP_OBJECT_WRAPPER_MEMBERS: [(&str, &str); 2] = [
  ("object_", "Tagged<CppHeapPointerWrapperObjectT>"),
  ("offset_", "int"),
];

// ── 三源 offset 表 (inl 构造器: 类别 → cpp 堆指针槽偏移) ──
/// JSAPIObjectWithEmbedderSlots / JSSpecialObject 共用槽 (h static_assert 对齐)。
pub const OFFSET_SOURCE_JS_OBJECT: &str = "offsetof(JSAPIObjectWithEmbedderSlots, cpp_heap_wrappable_)";
pub const OFFSET_SOURCE_CPP_HEAP_EXTERNAL_OBJECT: &str = "offsetof(CppHeapExternalObject, cpp_heap_wrappable_)";
pub const OFFSET_SOURCE_CPP_GC_MANAGED_BASE: &str = "offsetof(CppGCManagedBase, cpp_gc_wrapper_)";

// ── 类面骨架 (语义就地修订) ──
/// 原版 CppHeapObjectWrapper::From (inl: ExternalObject → CppGCManagedBase → JSObject 三路分派)。
pub fn from() {
  todo!("就地修订: cpp-heap-object-wrapper-inl.h From")
}

/// 原版 InitializeCppHeapWrapper (inl: SetupLazilyInitializedCppHeapPointerField(offset_))。
pub fn initialize_cpp_heap_wrapper() {
  todo!("就地修订: cpp-heap-object-wrapper-inl.h InitializeCppHeapWrapper")
}

/// 原版 GetCppHeapWrappable (inl: ReadCppHeapPointerField, tag 范围校验)。
pub fn get_cpp_heap_wrappable() {
  todo!("就地修订: cpp-heap-object-wrapper-inl.h GetCppHeapWrappable")
}

/// 原版 SetCppHeapWrappable (inl: WriteLazilyInitializedCppHeapPointerField + ForCppHeapPointer 屏障)。
pub fn set_cpp_heap_wrappable() {
  todo!("就地修订: cpp-heap-object-wrapper-inl.h SetCppHeapWrappable")
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(CPP_HEAP_POINTER_WRAPPER_OBJECT_TYPES.len() == 5);
  assert!(CPP_HEAP_POINTER_WRAPPER_OBJECT_TYPES[1] == "CppHeapExternalObject");
  assert!(CPP_HEAP_OBJECT_WRAPPER_MEMBERS.len() == 2);
};
`;

const p = writeSrc("v8-objects-js-async", "cpp_heap_object_wrapper.rs", rust);
console.log(`cpp-heap-object-wrapper.h.js: ${p} — union ${uniTypes.length} 类型 + 2 字段 + 三源 offset + From 分派 ${fromBranches} 路, 断言 ${checks} 项全过`);
