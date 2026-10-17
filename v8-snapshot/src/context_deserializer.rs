// 由 tool/src/snapshot/context-deserializer.js 从 v8/src/snapshot/context-deserializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 上下文反序列化器

// ContextDeserializer (原版): 反序列化以给定对象为根的上下文依赖对象图,
// 不应反序列化任何代码对象。方法面 (原版 h, 共 4 项):
// - `DeserializeContext`
// - `Deserialize`
// - `DeserializeEmbedderFields`
// - `DeserializeApiWrapperFields`

// 契约: embedder 字段 (DeserializeEmbedderFields) 与 API wrapper 字段
// (DeserializeApiWrapperFields) 在对象图装载后单独填回。
// cc 实现: DeserializeContext, Deserialize, DeserializeEmbedderFields, DeserializeApiWrapperFields (共 168 行)。
