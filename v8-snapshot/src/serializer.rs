// 由 tool/src/snapshot/serializer.js 从 v8/src/snapshot/serializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 序列化器常量与方法面

/// v8: src/snapshot/serializer.h RecursionDetector::kMaxRecursionDepth
/// 递归深度上限 (原版 RecursionDetector::kMaxRecursionDepth)
pub const MAX_RECURSION_DEPTH: i32 = 32;

/// v8: src/snapshot/serializer.h Serializer::HotObjectsList
/// 热对象环 (原版 Serializer::HotObjectsList / Deserializer::HotObjectsList 共用布局):
/// 环形队列, 容量 = kHotObjectCount, 查找未中哨兵 = -1
/// v8: src/snapshot/serializer.h HotObjectsList
pub mod hot_objects_list {
  /// 容量 (原版 kSize = kHotObjectCount, 对齐 serializer_deserializer.rs)
  pub const SIZE: usize = 8;
  /// 掩码 (原版 kSizeMask = kSize - 1; 容量保证 2 的幂)
  pub const SIZE_MASK: usize = 7;
  /// 查找未中 (原版 kNotFound)
  pub const NOT_FOUND: i32 = -1;
}

// ObjectCacheIndexMap 方法面 (原版 startup/共享堆对象缓存索引):
// - `LookupOrInsert`
// - `LookupOrInsert`
// - `Lookup`
// - `Values`
// - `size`

// Serializer 方法面 (原版 h, public + protected, 共 48 项, 实现于
// serializer.cc 1482 行, 语义由 Developer 落):
// - `Payload`
// - `ReferenceMapContains`
// - `isolate`
// - `TotalAllocationSize`
// - `IsNotMappedSymbol`
// - `SerializeDeferredObjects`
// - `SerializeObject`
// - `SerializeObjectImpl`
// - `MustBeDeferred`
// - `VisitRootPointers`
// - `SerializeRootObject`
// - `PutRoot`
// - `PutSmiRoot`
// - `PutBackReference`
// - `PutAttachedReference`
// - `PutNextChunk`
// - `PutRepeatRoot`
// - `PutPendingForwardReference`
// - `ResolvePendingForwardReference`
// - `SerializeRoot`
// - `SerializeHotObject`
// - `SerializeBackReference`
// - `SerializePendingObject`
// - `ObjectIsBytecodeHandler`
// - `EncodeExternalReference`
// - `TryEncodeExternalReference`
// - `SerializeReadOnlyObjectReference`
// - `Pad`
// - `InitializeCodeAddressMap`
// - `CopyCode`
// - `QueueDeferredObject`
// - `RegisterObjectIsPending`
// - `ResolvePendingObject`
// - `OutputStatistics`
// - `CountAllocation`
// - `PushStack`
// - `PopStack`
// - `PrintStack`
// - `PrintStack`
// - `reference_map`
// - `root_index_map`
// - `allow_unknown_external_references_for_testing`
// - `allow_active_isolate_for_testing`
// - `reconstruct_read_only_and_shared_object_caches_for_testing`
// - `allow_serializing_all_trusted_objects`
// - `deferred_objects_empty`
// - `serializer_tracks_serialization_statistics`
// - `set_serializer_tracks_serialization_statistics`

// cc 内统计空间表 (原版 kAllSnapshotSpaces, PrintStatistics 遍历):
// SnapshotSpace::ReadOnlyHeap, SnapshotSpace::Old, SnapshotSpace::Code
// (不含 Trusted — 统计输出仅覆盖 RO/Old/Code 三空间)
// 条件分支登记: OBJECT_PRINT → VERBOSE_SERIALIZATION_STATISTICS (实例类型统计表,
// kInstanceTypes = LAST_TYPE + 1), DEBUG → back_refs_/stack_ 全局句柄向量。
