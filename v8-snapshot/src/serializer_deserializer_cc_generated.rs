// 由 tool/src/snapshot/serializer-deserializer.cc.js 从 v8/src/snapshot/serializer-deserializer.cc 生成; 修订就地改, 复跑覆盖
// 序列化器-反序列化器 cc 面骨架

// 方法注册表 (3 条):
// - IterateStartupObjectCache
// - IterateSharedHeapObjectCache
// - CanBeDeferred
// 不可延迟情形登记 (6 条):
// 不可延迟情形: // - map slots
// - Internalized strings
// - JS objects with embedder fields
// - ByteArray
// - JSArrayBuffer and JSTypedArray
// - empty EmbdderDataArrays
// 语义面 (缓存遍历/豁免判定) 登记跳过。

pub struct SerializerDeserializerCc;

// TODO(dev): 依 serializer_deserializer.rs 补 Iterate*/CanBeDeferred。

// —— 提取不变式断言 ——
const _: () = {};
