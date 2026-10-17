// 由 tool/src/snapshot/read-only-serializer.js 从 v8/src/snapshot/read-only-serializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 只读空间序列化器

// ReadOnlySerializer (原版, 继承 RootsSerializer)。
// 方法面 (原版 h, 共 2 项):
// - `Serialize`
// - `SerializeObjectImpl`

// cc 实现 (机械提取, 587 行): Serialize
// 契约: RO 快照一次性生成; object_cache 可按
// kReconstructReadOnlyAndSharedObjectCachesForTesting 从现有 RO heap 重建,
// 保证同索引映射; 反序列化端配合 ro::Bytecode (kAllocatePage/kSegment/...)。
