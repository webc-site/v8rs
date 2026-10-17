// 由 tool/src/snapshot/sort-builtins.js 从 v8/src/snapshot/sort-builtins.{h,cc} 生成; 修订就地改, 复跑覆盖
// 内建函数内嵌快照排序器

/// BuiltinsSorter 聚类常量 (原版 h, 类内 const, 机械展开):
/// 相邻聚类合并的最小边概率阈值 (千分比)
pub const MIN_EDGE_PROBABILITY_THRESHOLD: i32 = 10;
/// 单聚类大小上限 (原版 1 * MB)
pub const MAX_CLUSTER_SIZE: u32 = 1024 * 1024;
/// 密度允许下降阈值
pub const MAX_DENSITY_DECREASE_THRESHOLD: u32 = 8;

/// 性能日志 marker (原版 std::string 常量):
pub const BUILTIN_CALL_BLOCK_DENSITY_MARKER: &str = "block_count";
pub const BUILTIN_DENSITY_MARKER: &str = "builtin_count";

// 类型别名族 (原版 using, 语义由 Developer 落型):
// - `CallProbabilities`
// - `CallGraph`
// - `BuiltinDensityMap`
// - `BuiltinSize`
// - `BuiltinClusterMap`

// BuiltinsSorter 方法面 (原版 h, 共 8 项):
// - `SortBuiltins`
// - `InitializeCallGraph`
// - `InitializeClusters`
// - `MergeBestPredecessors`
// - `SortClusters`
// - `FindBestPredecessorOf`
// - `ProcessBlockCountLineInfo`
// - `ProcessBuiltinDensityLineInfo`

// 排序契约: 读 profiling 日志 (块计数/内建计数 marker) → 建调用图 →
// 按最佳前驱聚类合并 (上限 MAX_CLUSTER_SIZE) → 聚类内按密度排序输出
// 内嵌快照序 (embedded snapshot order, 对齐 embedded-data.rs 布局)。
