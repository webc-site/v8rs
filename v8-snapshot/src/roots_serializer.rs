// 由 tool/src/snapshot/roots-serializer.js 从 v8/src/snapshot/roots-serializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 根遍历序列化基类

// RootsSerializer (原版, 继承 Serializer): 遍历根的序列化器基类,
// 维护可与非序列化器共享的对象缓存。方法面 (原版 h, 共 6 项):
// - `can_be_rehashed`
// - `root_has_been_serialized`
// - `IsRootAndHasBeenSerialized`
// - `CheckRehashability`
// - `SerializeInObjectCache`
// - `object_cache_empty`

// 字段骨架:
// - first_root_to_be_serialized_: 构造时约定此前所有根已序列化
// - root_has_been_serialized_: bitset<RootsTable::kEntriesCount> 根标记位集
// - object_cache_index_map_: 对象缓存索引 (对齐 serializer.rs ObjectCacheIndexMap)
// - can_be_rehashed_: 仅序列化可重散列哈希表时为真
// cc 实现 (机械清点): SerializeInObjectCache, Synchronize, VisitRootPointers, CheckRehashability
