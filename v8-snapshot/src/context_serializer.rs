// 由 tool/src/snapshot/context-serializer.js 从 v8/src/snapshot/context-serializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 上下文序列化器

// ContextSerializer (原版, 继承 RootsSerializer): 序列化上下文依赖对象图
// (不含代码对象)。方法面 (原版 h, 共 7 项):
// - `Serialize`
// - `can_be_rehashed`
// - `SerializeObjectImpl`
// - `ShouldBeInTheStartupObjectCache`
// - `CheckRehashability`
// - `SerializeObjectWithEmbedderFields`
// - `SerializeApiWrapperFields`

// 契约: embedder 字段与 API wrapper 字段单独序列化
// (SerializeObjectWithEmbedderFields / SerializeApiWrapperFields);
// 启动对象缓存准入由 ShouldBeInTheStartupObjectCache 判定。
// cc 实现 (机械清点, 362 行): Serialize, SerializeObjectImpl, ShouldBeInTheStartupObjectCache, SerializeApiWrapperFields, SerializeObjectWithEmbedderFields, CheckRehashability
