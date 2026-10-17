// 由 tool/src/snapshot/startup-serializer.js 从 v8/src/snapshot/startup-serializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 启动序列化器

// StartupSerializer (原版, 继承 RootsSerializer)。
// 堆序列化次序契约 (原版注释机械登记):
// 1) 强根 → 2) builtins 与字节码 handler → 3) 启动对象缓存 → 4) 弱引用 (字符串表等)
// 方法面 (原版 h, 共 5 项):
// - `SerializeStrongReferences`
// - `SerializeWeakReferencesAndDeferred`
// - `SerializeUsingSharedHeapObjectCache`
// - `SerializeUsingStartupObjectCache`
// - `CheckNoDirtyFinalizationRegistries`

// FinalizationRegistry 契约: 脏 FinalizationRegistry 列表为弱引用不入快照,
// 启动期禁用 (CheckNoDirtyFinalizationRegistries)。

// SerializedHandleChecker (原版): 校验全局/永恒句柄均已被序列化。
// 方法面 (共 3 项): VisitRootPointers, CheckGlobalAndEternalHandles, AddToSet

// cc 规模: 253 行 (语义由 Developer 落)。
