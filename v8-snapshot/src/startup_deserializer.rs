// 由 tool/src/snapshot/startup-deserializer.js 从 v8/src/snapshot/startup-deserializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 启动反序列化器

// StartupDeserializer (原版): 将上下文无关快照数据装入空堆。
// 方法面 (原版 h, 共 4 项):
// - `DeserializeIntoIsolate`
// - `FlushICache`
// - `LogNewMapEvents`
// - `DeserializeAndCheckExternalReferenceTable`

// cc 实现 (机械清点, 131 行): DeserializeIntoIsolate, DeserializeAndCheckExternalReferenceTable, LogNewMapEvents, FlushICache
// 契约: 反序列化完成后 FlushICache; 外部引用表逐项
// DeserializeAndCheckExternalReferenceTable 核对。
