// 由 tool/src/snapshot/shared-heap-serializer.js 从 v8/src/snapshot/shared-heap-serializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 共享堆序列化器

// SharedHeapSerializer (原版, 继承 RootsSerializer): 共享空间对象序列化。
// 方法面 (原版 h, 共 8 项):
// - `FinalizeSerialization`
// - `SerializeUsingSharedHeapObjectCache`
// - `CanBeInSharedOldSpace`
// - `ShouldBeInSharedHeapObjectCache`
// - `ShouldReconstructSharedHeapObjectCacheForTesting`
// - `ReconstructSharedHeapObjectCacheForTesting`
// - `SerializeStringTable`
// - `SerializeObjectImpl`

// cc 实现 (机械清点, 221 行): CanBeInSharedOldSpace, ShouldBeInSharedHeapObjectCache, FinalizeSerialization, SerializeUsingSharedHeapObjectCache, SerializeStringTable, SerializeObjectImpl, ShouldReconstructSharedHeapObjectCacheForTesting, ReconstructSharedHeapObjectCacheForTesting
// 契约: 对象可共享性判定 (字符串表/ETC) 与共享对象缓存重建
// (配合 kReconstructReadOnlyAndSharedObjectCachesForTesting)。
