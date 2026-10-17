// 由 tool/src/objects/embedder-data-slot.h.js 从 v8/src/objects/embedder-data-slot.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// EmbedderDataSlot 偏移常量与 RawData 分支
//! EmbedderDataSlot: 嵌入器数据槽 (tagged 载荷 + raw 指针双段)。
//! 目标配置: 指针压缩开。

/// tagged 载荷段偏移 (原版 kTaggedPayloadOffset)。
pub const TAGGED_PAYLOAD_OFFSET: usize = 0;
/// raw 段偏移 (原版 kCppHeapPointerOffset = kTaggedSize, 压缩下 4)。
pub const CPP_HEAP_POINTER_OFFSET: usize = 4;
/// 指针对齐要求 (原版 kRequiredPtrAlignment = kSmiTagSize = 1)。
pub const REQUIRED_PTR_ALIGNMENT: usize = 1;

/// 原始嵌入数据类型 (压缩分支: 原版 `using RawData = uint64_t`。
/// 非压缩分支为双 Address 结构体 — 配置分支登记跳过)。
pub type RawData = u64;

// —— 提取不变式断言 ——
const _: () = {
  assert!(TAGGED_PAYLOAD_OFFSET == 0);
  assert!(CPP_HEAP_POINTER_OFFSET == 4);
  assert!(REQUIRED_PTR_ALIGNMENT == 1);
  assert!(size_of::<RawData>() == 8);
};
