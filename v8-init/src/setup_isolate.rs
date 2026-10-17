// 由 tool/src/init/setup-isolate.js 从 v8/src/init/setup-isolate.{h,-deserialize.cc,-full.cc} 生成; 修订就地改, 复跑覆盖
// Isolate 初始化委托: 快照反序列化 / 全量构建双实现

/// SetupIsolateDelegate (原版 class SetupIsolateDelegate)
/// 抽象层: 组件 (builtins / 字节码 handler) 既可从快照反序列化, 也可从零构建。
/// 链接期二选一:
/// - setup-isolate-deserialize.cc: 只从快照装载
/// - setup-isolate-full.cc: 按标志从快照装载或现场构建 (mksnapshot 用)
pub trait SetupIsolateDelegate {
  /// 原版签名: virtual SetupHeap(Isolate* isolate, bool create_heap_objects)
  /// 原版签名: virtual SetupBuiltins(Isolate* isolate, bool compile_builtins)
  fn setup_heap(&mut self, isolate: &mut Isolate, create_heap_objects: bool) -> bool;
  fn setup_builtins(&mut self, isolate: &mut Isolate, compile_builtins: bool);
}

/// 原版 protected 静态签名: SetupBuiltinsInternal(Isolate* isolate, bool compute_builtins_effects) — 由 Developer 落为关联函数
/// 原版 protected 静态签名: AddBuiltin(Builtins* builtins, Builtin builtin, Tagged<Code> code) — 由 Developer 落为关联函数
/// 原版 protected 静态签名: PopulateWithPlaceholders(Isolate* isolate) — 由 Developer 落为关联函数
/// 原版 protected 静态签名: ReplacePlaceholders(Isolate* isolate) — 由 Developer 落为关联函数
/// 原版 protected 静态签名: SetupHeapInternal(Isolate* isolate) — 由 Developer 落为关联函数

/// deserialize 实现 (setup-isolate-deserialize.cc) 契约 — 机械提取 CHECK:
/// - CHECK(!create_heap_objects, "Heap setup supported only in mksnapshot")
/// - CHECK(!compile_builtins, "Builtin compilation supported only in mksnapshot")

/// full 实现 (setup-isolate-full.cc) 契约 — compile_builtins=false 时
/// CHECK(snapshot_available()); 否则走内部构建路径 (机械提取调用链): create_heap_objects → SetupHeapInternal → compile_builtins → SetupBuiltinsInternal → VerifyTransitiveBuiltins;
/// DEBUG 构建追加 DebugEvaluate::VerifyTransitiveBuiltins。
