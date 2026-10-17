// 由 tool/src/snapshot/object-deserializer.js 从 v8/src/snapshot/object-deserializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 对象图反序列化器

// ObjectDeserializer (原版, Deserializer<Isolate>): 反序列化以对象为根的图,
// 失败优雅降级。方法面 (共 4 项):
// - `DeserializeSharedFunctionInfo`
// - `Deserialize`
// - `LinkAllocationSites`
// - `CommitPostProcessedObjects`

// OffThreadObjectDeserializer (原版, Deserializer<LocalIsolate>): 后台线程变体,
// 反序列化的脚本句柄外移给调用方。方法面 (共 3 项):
// - `DeserializeSharedFunctionInfo`
// - `Deserialize`
// - `CommitPostProcessedObjects`

// 契约: LinkAllocationSites 把新分配点回链至旧 allocation site;
// CommitPostProcessedObjects 提交脚本/SFI 等后处理产物。
// cc 实现 (机械清点): Deserialize, CommitPostProcessedObjects, LinkAllocationSites, Deserialize, CommitPostProcessedObjects
