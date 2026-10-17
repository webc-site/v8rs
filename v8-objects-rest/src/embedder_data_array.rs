// 由 tool/src/objects/embedder-data-array.h.js 从 v8/src/objects/embedder-data-array.{h,-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// EmbedderDataArray 布局公式与上限
//! EmbedderDataArray: 嵌入器数据数组 (length + 内联槽)。
//! 目标配置: 指针压缩开 → kEmbedderDataSlotSize = 8 (2 * kTaggedSize)。

/// 槽大小 (原版 kEmbedderDataSlotSize; TAGGED_SIZE_8_BYTES=false 分支)。
pub const EMBEDDER_DATA_SLOT_SIZE: usize = 8;

/// 头部对齐锚 (原版 static_assert: OFFSET_OF_DATA_START == Internals::kFixedArrayHeaderSize;
/// 压缩下 FixedArray 头 = map + length = 8)。
pub const DATA_START_ANCHOR: &str = "Internals::kFixedArrayHeaderSize";

/// 数组总大小 (原版 SizeFor: OFFSET_OF_DATA_START + length * kEmbedderDataSlotSize)。
#[inline]
#[must_use]
pub const fn size_for(data_start: usize, length: usize) -> usize {
  data_start + length * EMBEDDER_DATA_SLOT_SIZE
}

/// 第 index 槽偏移 (原版 OffsetOfElementAt = SizeFor(index))。
#[inline]
#[must_use]
pub const fn offset_of_element_at(data_start: usize, index: usize) -> usize {
  size_for(data_start, index)
}

/// 数组大小上限 (原版 kEmbedderDataArrayMaxSize = kMaxRegularHeapObjectSize)。
pub const EMBEDDER_DATA_ARRAY_MAX_SIZE_ANCHOR: &str = "kMaxRegularHeapObjectSize";

/// 最大槽数 (原版 kEmbedderDataArrayMaxLength =
/// (kEmbedderDataArrayMaxSize - OFFSET_OF_DATA_START) / kEmbedderDataSlotSize)。
#[inline]
#[must_use]
pub const fn max_length(data_start: usize, max_regular_heap_object_size: usize) -> usize {
  (max_regular_heap_object_size - data_start) / EMBEDDER_DATA_SLOT_SIZE
}

/// 布局字段表 (tq 顺序 1:1)。
pub const LAYOUT_FIELDS: [(&str, &str); 1] = [
  ("length", "Smi"),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(EMBEDDER_DATA_SLOT_SIZE == 8);
  assert!(size_for(8, 3) == 32);
  assert!(offset_of_element_at(8, 0) == 8);
  assert!(max_length(8, 128) == 15);
  assert!(LAYOUT_FIELDS.len() == 1);
};
