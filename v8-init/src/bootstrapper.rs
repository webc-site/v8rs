// 由 tool/src/init/bootstrapper.js 从 v8/src/init/bootstrapper.{h,cc} 生成; 修订就地改, 复跑覆盖
// 全局上下文引导 (超大 cc, 仅签名骨架)

// Bootstrapper: 创建 JavaScript 全局上下文的公共入口;
// 原版 cc 共 7192 行 (>2k), 按提取规则仅登记签名骨架, 实现由 Developer 落。

/// 默认上下文索引 (原版 kDefaultContextIndex)
pub const DEFAULT_CONTEXT_INDEX: i32 = 0;

// SourceCodeCache: FixedArray 存 (OneByteString, SharedFunctionInfo) 对,
// 将原生扩展代码文件名映射到预编译函数。签名骨架:
// - `Initialize(Isolate* isolate, bool create_heap_objects)`
// - `Iterate(RootVisitor* v)`
// - `Lookup(Isolate* isolate, std::string_view name, DirectHandle<SharedFunctionInfo>* handle)`
// - `Add(Isolate* isolate, std::string_view name, DirectHandle<SharedFunctionInfo> shared)`

// Bootstrapper 公开接口签名骨架 (原版 bootstrapper.h, 共 15 条):
// - `InitializeOncePerProcess`
// - `Initialize`
// - `TearDown`
// - `CreateEnvironment`
// - `CreateEnvironmentForTesting`
// - `NewRemoteContext`
// - `Iterate`
// - `IsActive`
// - `ArchiveSpacePerThread`
// - `ArchiveState`
// - `RestoreState`
// - `FreeThreadResources`
// - `InstallExtensions`
// - `extensions_cache`
// - `InitializeLazyPartOfContext`

// Bootstrapper::IsActive 契约: nesting_ != 0 即引导进行中;
// ArchiveSpacePerThread/ArchiveState/RestoreState/FreeThreadResources 为线程抢占支撑。
