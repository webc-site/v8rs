// 由 tool/src/objects/cpp-heap-external-object.h.js 从 v8/src/objects/cpp-heap-external-object.{h,tq} 生成; 修订就地改, 复跑覆盖
// CppHeap 外部对象: 纯 C++ 堆包装指针的最小堆载体
//! CppHeapExternalObject: 无 JS 字段的 cppgc 对象包装载体。
//!
//! 仅一个 CppHeapPointerMember 槽 (经 cpp 堆指针表间接寻址), 供
//! CppHeapObjectWrapper 按偏移统一读写 (见 cpp_heap_object_wrapper.rs)。

// ── 布局 (h 与 tq 1:1) ──
pub const CPP_HEAP_EXTERNAL_OBJECT_FIELDS: [(&str, &str); 1] =
  [("cpp_heap_wrappable", "CppHeapPointer")];

/// 头大小公式 (原版 kHeaderSize = sizeof(CppHeapExternalObject); 唯一字段即尾)。
#[inline]
#[must_use]
pub const fn k_header_size(cpp_heap_pointer_size: usize) -> usize {
  cpp_heap_pointer_size
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
  assert!(CPP_HEAP_EXTERNAL_OBJECT_FIELDS.len() == 1);
  assert!(str_eq(
    CPP_HEAP_EXTERNAL_OBJECT_FIELDS[0].1,
    "CppHeapPointer"
  ));
  assert!(k_header_size(8) == 8);
};
