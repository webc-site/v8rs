// 由 tool/src/objects/cpp-heap-object-wrapper.h.js 从 v8/src/objects/cpp-heap-object-wrapper.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// cpp 堆包装统一读写面: union 5 类对象按各自偏移访问同一指针槽
//! CppHeapObjectWrapper: 按 (object, offset) 二元组统一读写 cpp 堆指针。
//!
//! 五种宿主对象各有 cpp_heap_wrappable 等价槽; 包装器记录对象类别对应的
//! 字段偏移, 读写与惰性初始化全部转发到该偏移。

// ── 可包装对象 union (原版 CppHeapPointerWrapperObjectT, 5 类型) ──
pub const CPP_HEAP_POINTER_WRAPPER_OBJECT_TYPES: [&str; 5] = [
  "JSObject",
  "CppHeapExternalObject",
  "NativeContext",
  "CppGCManagedBase",
  "EmbedderDataArray",
];

// ── 包装器两字段 (object_ + offset_) ──
pub const CPP_HEAP_OBJECT_WRAPPER_MEMBERS: [(&str, &str); 2] = [
  ("object_", "Tagged<CppHeapPointerWrapperObjectT>"),
  ("offset_", "int"),
];

// ── 三源 offset 表 (inl 构造器: 类别 → cpp 堆指针槽偏移) ──
/// JSAPIObjectWithEmbedderSlots / JSSpecialObject 共用槽 (h static_assert 对齐)。
pub const OFFSET_SOURCE_JS_OBJECT: &str =
  "offsetof(JSAPIObjectWithEmbedderSlots, cpp_heap_wrappable_)";
pub const OFFSET_SOURCE_CPP_HEAP_EXTERNAL_OBJECT: &str =
  "offsetof(CppHeapExternalObject, cpp_heap_wrappable_)";
pub const OFFSET_SOURCE_CPP_GC_MANAGED_BASE: &str = "offsetof(CppGCManagedBase, cpp_gc_wrapper_)";

// ── 类面骨架 (深水推迟台账, 依 readme/zh.md) ──
/// v8: src/objects/cpp-heap-object-wrapper-inl.h CppHeapObjectWrapper::From。
pub fn from() {
  todo!(
    "CppHeapObjectWrapper::From: 依赖 ExternalObject/CppGCManagedBase/JSObject 三路分派 + Isolate + offsetof 表; 解锁 v8-objects-core + v8-objects-js-core + v8-execution-isolate"
  )
}

/// v8: src/objects/cpp-heap-object-wrapper-inl.h InitializeCppHeapWrapper。
pub fn initialize_cpp_heap_wrapper() {
  todo!(
    "InitializeCppHeapWrapper: 依赖 SetupLazilyInitializedCppHeapPointerField + 惰性初始化; 解锁 v8-sandbox"
  )
}

/// v8: src/objects/cpp-heap-object-wrapper-inl.h GetCppHeapWrappable。
pub fn get_cpp_heap_wrappable() {
  todo!("GetCppHeapWrappable: 依赖 ReadCppHeapPointerField + tag 范围校验; 解锁 v8-sandbox")
}

/// v8: src/objects/cpp-heap-object-wrapper-inl.h SetCppHeapWrappable。
pub fn set_cpp_heap_wrappable() {
  todo!(
    "SetCppHeapWrappable: 依赖 WriteLazilyInitializedCppHeapPointerField + ForCppHeapPointer 写屏障; 解锁 v8-sandbox + v8-heap-cppgc"
  )
}

// ── 提取不变式断言 ── (const 上下文无 PartialEq<&str>, 用字节循环 const fn)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}
const _: () = {
  assert!(CPP_HEAP_POINTER_WRAPPER_OBJECT_TYPES.len() == 5);
  assert!(str_eq(
    CPP_HEAP_POINTER_WRAPPER_OBJECT_TYPES[1],
    "CppHeapExternalObject"
  ));
  assert!(CPP_HEAP_OBJECT_WRAPPER_MEMBERS.len() == 2);
};
