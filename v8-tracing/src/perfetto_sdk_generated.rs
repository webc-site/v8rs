// 由 tool/src/tracing/code-data-source.h.js 从 v8/src/tracing/perfetto-sdk.h 生成; 修订就地改, 复跑覆盖
// Perfetto SDK 接入分支矩阵

/// Perfetto SDK 接入矩阵 (原版 perfetto-sdk.h 的 #if 展开)
pub mod perfetto_sdk {
  /// V8_USE_PERFETTO: 全量启用 (x64/arm64 默认构建)
  pub const USE_PERFETTO: bool = true;
  /// 前置宏: 沿用 legacy trace events 语义 (原版 PERFETTO_ENABLE_LEGACY_TRACE_EVENTS=1)
  pub const ENABLE_LEGACY_TRACE_EVENTS: u8 = 1;
  /// V8_USE_PERFETTO_SDK: 用整包 perfetto.h; 否则按头清单散引
  pub const USE_PERFETTO_SDK: bool = true;

  /// 散引模式头清单 (12 个, 对数用)
  pub const SPLIT_HEADERS: [&str; 12] = [
    "perfetto/protozero/scattered_heap_buffer.h",
    "perfetto/tracing/data_source.h",
    "perfetto/tracing/track_event.h",
    "perfetto/tracing/track_event_legacy.h",
    "perfetto/builtin_clock.pbzero.h",
    "perfetto/data_source_descriptor.gen.h",
    "perfetto/v8_config.gen.h",
    "perfetto/v8.pbzero.h",
    "perfetto/interned_data.pbzero.h",
    "perfetto/trace_packet.pbzero.h",
    "perfetto/trace_packet_defaults.pbzero.h",
    "perfetto/debug_annotation.pbzero.h",
  ];
}
