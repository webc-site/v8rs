// GC 内核布局常量面: 本 crate 分代 GC 算法只认 ID 与布局 (todo.md 五 破环策略),
// 这些是与 heap-core 空间实现解耦的编译期布局常量, 逐项锚定原版头。

/// 平台地址 (`Address` = `uintptr_t`)。
/// v8: src/common/globals.h Address
pub type Address = usize;

/// 页大小位数 (目标配置 x64/arm64 默认, 非 hugepage/非 PPC)。
/// v8: src/base/build_config.h:80 kPageSizeBits
pub const K_PAGE_SIZE_BITS: u32 = 18;

/// 常规页字节数 (kRegularPageSize = 1 << kPageSizeBits)。
/// v8: src/base/build_config.h:83 kRegularPageSize
pub const K_REGULAR_PAGE_SIZE: usize = 1usize << K_PAGE_SIZE_BITS;

/// 页内偏移掩码 (MemoryChunk::kAlignmentMask = kRegularPageSize - 1)。
/// v8: src/heap/memory-chunk.h kAlignmentMask
pub const K_PAGE_ALIGNMENT_MASK: usize = K_REGULAR_PAGE_SIZE - 1;

/// 指针压缩下 Tagged 宽度位数 (kTaggedSizeLog2 = 2, kTaggedSize = 4)。
/// v8: src/common/globals.h:581 kTaggedSizeLog2 (V8_COMPRESS_POINTERS 生效支)
pub const K_TAGGED_SIZE_LOG2: u32 = 2;

/// 最大常规堆对象字节数 (kMaxRegularHeapObjectSize = 1 << (kPageSizeBits - 1))。
/// v8: src/common/globals.h:755 kMaxRegularHeapObjectSize
pub const K_MAX_REGULAR_HEAP_OBJECT_SIZE: usize = 1usize << (K_PAGE_SIZE_BITS - 1);

/// 每字节位数。
/// v8: src/base/bitfield.h kBitsPerByte
pub const K_BITS_PER_BYTE: u32 = 8;
/// 每字节位数取 log2。
/// v8: src/base/bitfield.h kBitsPerByteLog2
pub const K_BITS_PER_BYTE_LOG2: u32 = 3;

/// 标记位 CellType 全 1 值 (`kAllBitsSetInCellValue`)。
/// v8: src/heap/marking.cc:13 kAllBitsSetInCellValue
pub const K_ALL_BITS_SET_IN_CELL_VALUE: usize = usize::MAX;

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_REGULAR_PAGE_SIZE == 262144);
  assert!(K_PAGE_ALIGNMENT_MASK == 262143);
  assert!(K_MAX_REGULAR_HEAP_OBJECT_SIZE == 131072);
  assert!(K_TAGGED_SIZE_LOG2 == 2);
  assert!((1usize << K_TAGGED_SIZE_LOG2) == 4);
};
