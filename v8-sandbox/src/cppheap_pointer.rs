// 由 tool/src/sandbox/cppheap-pointer.h.js 从 v8/src/sandbox/cppheap-pointer.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// CppHeap 指针句柄骨架

// Tag 枚举 (CppHeapPointerTag) 与 tagging 常量在 src/sandbox/indirect-pointer-tag.h,
// 由既有 tags/tables 产物覆盖, 此处不重复登记。

/// 原 CppHeapPointerHandle (外部队表索引)
pub type CppHeapPointerHandle = u32;

/// 原 CppHeapPointer_t — 沙箱关闭时退化为裸指针, 开启时为句柄
pub type CppHeapPointerT = usize;

// 原版头文件函数面 (3 条):
// - ReadCppHeapPointerField
// - ReadCppHeapPointerField
// - WriteLazilyInitializedCppHeapPointerField
// 原版 -inl.h 函数面 (3 条):
// - ReadCppHeapPointerField
// - ReadCppHeapPointerField
// - WriteLazilyInitializedCppHeapPointerField

// TODO(dev): 依函数面补读写路径实现 (Tag 校验/表访问)。

// —— 提取不变式断言 ——
const _: () = {
  let _h: CppHeapPointerHandle = 0;
  let _p: CppHeapPointerT = 0;
};
