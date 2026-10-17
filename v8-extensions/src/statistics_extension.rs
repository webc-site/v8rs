// 由 tool/src/extensions/statistics-extension.h.js 从 v8/src/extensions/statistics-extension.{h,cc} 生成; 修订就地改, 复跑覆盖
// Statistics 扩展元数据 + 堆统计键名表 + 签名骨架

//! Statistics 扩展 — 原版 namespace v8::internal::StatisticsExtension
//!
//! 语义: 注入 getV8Statistics() — 收集计数器与分代堆尺寸, 以对象键值返回。

/// Statistics 扩展注册名
pub const STATISTICS_EXTENSION_NAME: &str = "v8/statistics";

/// 注入 JS 的源码 (原版 kSource)
pub const STATISTICS_SOURCE: &str = "native function getV8Statistics();";

/// 堆统计键名全集 (原版 StatisticNumber numbers[22], 顺序 1:1)
pub const STAT_NUMBER_KEYS: [&str; 22] = [
  "total_committed_bytes",
  "new_space_live_bytes",
  "new_space_available_bytes",
  "new_space_commited_bytes",
  "old_space_live_bytes",
  "old_space_available_bytes",
  "old_space_commited_bytes",
  "code_space_live_bytes",
  "code_space_available_bytes",
  "code_space_commited_bytes",
  "lo_space_live_bytes",
  "lo_space_available_bytes",
  "lo_space_commited_bytes",
  "code_lo_space_live_bytes",
  "code_lo_space_available_bytes",
  "code_lo_space_commited_bytes",
  "trusted_space_live_bytes",
  "trusted_space_available_bytes",
  "trusted_space_commited_bytes",
  "trusted_lo_space_live_bytes",
  "trusted_lo_space_available_bytes",
  "trusted_lo_space_commited_bytes",
];

/// 对应取值表达式 (原版 numbers[] 值位, 顺序与 STAT_NUMBER_KEYS 一致; 供 Developer 侧对接 Heap)
pub const STAT_NUMBER_EXPRS: [&str; 22] = [
  "heap->memory_allocator()->Size()",
  "new_space_size",
  "new_space_available",
  "new_space_committed_memory",
  "heap->old_space()->Size()",
  "heap->old_space()->Available()",
  "heap->old_space()->CommittedMemory()",
  "heap->code_space()->Size()",
  "heap->code_space()->Available()",
  "heap->code_space()->CommittedMemory()",
  "heap->lo_space()->Size()",
  "heap->lo_space()->Available()",
  "heap->lo_space()->CommittedMemory()",
  "heap->code_lo_space()->Size()",
  "heap->code_lo_space()->Available()",
  "heap->code_lo_space()->CommittedMemory()",
  "heap->trusted_space()->Size()",
  "heap->trusted_space()->Available()",
  "heap->trusted_space()->CommittedMemory()",
  "heap->trusted_lo_space()->Size()",
  "heap->trusted_lo_space()->Available()",
  "heap->trusted_lo_space()->CommittedMemory()",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(STAT_NUMBER_KEYS.len() == 22);
  assert!(STAT_NUMBER_EXPRS.len() == 22);
};

// —— 签名骨架 (原版声明 cc 1:1) ——
// public: GetNativeFunctionTemplate(isolate, name) -> FunctionTemplate
// public static: GetCounters(info) — info[0] 为 true 时先 CollectAllGarbage(GCFlag::kNoFlags,
//                GarbageCollectionReason::kCountersExtension), 再枚举计数器与 numbers 表
// static: AddCounter(isolate, object, counter, name) — 仅 counter->Enabled() 时写属性
// static: AddNumber(isolate, object, value: f64, name)
// static: AddNumber64(isolate, object, value: i64, name)

// X-macro 台账:
//   counter_list[] = STATS_COUNTER_LIST(ADD_COUNTER) + STATS_COUNTER_NATIVE_CODE_LIST(ADD_COUNTER)
//   两表定义于 src/logging/counters.h — 运行时计数器名全集, 由 counters 模块脚本落地, 此处登记引用
