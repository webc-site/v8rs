// 由 tool/src/objects/free-space.h.js 从 v8/src/objects/free-space.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// FreeSpace 空闲块常量与骨架

/// 空闲块 (原版 V8_OBJECT class FreeSpace : HeapObject, V8_IT_NO_AUTO_DISPATCH)
/// 布局 (tq): map + size(int32) + next(Tagged/EncodedNext)
/// 大小上限 (原版 kMaxSizeInBytes = Smi::kMaxValue × kTaggedSize = 2^30-1 × 4)
pub const MAX_SIZE_IN_BYTES: u32 = 4294967292;

/// 填充块标记 (原版 V8_OBJECT class Filler : HeapObject, 无字段)
pub struct Filler;

// —— 签名骨架 ——
// int size(RelaxedLoadTag) const; / set_size / SizeFor(int)
// SetSize(const WritableFreeSpace&, int, RelaxedStoreTag)
// next() / set_next() — 空闲链表串联
// UnlinkedNextField? 不提取, 语义归 Developer
pub const FREE_SPACE_SIGNATURES: [&str; 5] = ["size", "set_size", "size_for", "next", "set_next"];

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_SIZE_IN_BYTES == 4294967292);
};
