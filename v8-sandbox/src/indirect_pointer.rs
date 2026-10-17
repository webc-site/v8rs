// 由 tool/src/sandbox/indirect-pointer.h.js 从 v8/src/sandbox/indirect-pointer.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// 间接指针函数面骨架

// IndirectPointerTag/句柄常量在 src/sandbox/indirect-pointer-tag.h (既有覆盖面)。

/// 原 IndirectPointerHandle (uint32_t 表索引)
pub type IndirectPointerHandle = u32;

// 原版头文件函数面 (3 条):
// - InitSelfIndirectPointerField
// - ReadIndirectPointerField
// - WriteIndirectPointerField
// 原版 -inl.h 函数面 (5 条):
// - InitSelfIndirectPointerField
// - ResolveIndirectPointerHandle
// - ReadIndirectPointerHandle
// - ReadIndirectPointerField
// - WriteIndirectPointerField

// TODO(dev): 补 ReadIndirectPointerField/WriteIndirectPointerField 沙箱开闭双路径。

// —— 提取不变式断言 ——
const _: () = {
  let _h: IndirectPointerHandle = 0;
};
