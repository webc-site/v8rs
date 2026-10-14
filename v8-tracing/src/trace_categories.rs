// 由 tool/src/tracing/trace-categories.h.js 从 v8/src/tracing/trace-categories.h;src/tracing/trace-categories.cc 生成; 修订就地改, 复跑覆盖
// PERFETTO_DEFINE_CATEGORIES X-macro 展开

//! trace-categories.h/.cc: V8 内建 trace 类别注册表 (85+33 行)。
//!
//! 原版 PERFETTO_DEFINE_CATEGORIES_IN_NAMESPACE_WITH_ATTRS(v8, ...) 声明
//! 32 个类别项 (单类别 27 + 类别组 5);
//! `TRACE_DISABLED_BY_DEFAULT(name)` 就地展开为 "disabled-by-default-${name}"。
//! trace-categories.cc: PERFETTO_TRACK_EVENT_STATIC_STORAGE + GetTrackEventCategoryRegistry
//! (perfetto SDK 编译面)【推迟登记: 依赖 perfetto protos】。

/// 单条类别定义 (Category / Category::Group 展开产物)。
pub struct TraceCategoryDef {
  /// 完整名 (Group 为逗号拼接名, disabled-by-default 已展开)。
  pub name: &'static str,
  /// SetDescription 描述 (可选)。
  pub desc: Option<&'static str>,
  /// SetTags 标签 (如 "slow")。
  pub tags: &'static [&'static str],
  /// 是否为 Category::Group。
  pub group: bool,
}

/// V8 内建类别全表 (32 项, 定义序)。
pub const TRACE_CATEGORIES: &[TraceCategoryDef] = &[
  TraceCategoryDef {
    name: "cppgc",
    desc: None,
    tags: &[],
    group: false,
  },
  TraceCategoryDef {
    name: "v8",
    desc: None,
    tags: &[],
    group: false,
  },
  TraceCategoryDef {
    name: "v8.console",
    desc: None,
    tags: &[],
    group: false,
  },
  TraceCategoryDef {
    name: "v8.execute",
    desc: None,
    tags: &[],
    group: false,
  },
  TraceCategoryDef {
    name: "v8.memory",
    desc: Some("Emits counters related to v8 memory usage."),
    tags: &[],
    group: false,
  },
  TraceCategoryDef {
    name: "v8.wasm",
    desc: None,
    tags: &[],
    group: false,
  },
  TraceCategoryDef {
    name: "devtools.timeline,v8",
    desc: None,
    tags: &[],
    group: true,
  },
  TraceCategoryDef {
    name: "devtools.timeline,disabled-by-default-v8.gc",
    desc: None,
    tags: &[],
    group: true,
  },
  TraceCategoryDef {
    name: "disabled-by-default-cppgc",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-devtools.timeline",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-devtools.v8-source-rundown",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-devtools.v8-source-rundown-sources",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.compile",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.cpu_profiler",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.gc",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.gc_extra",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.gc_stats",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.inspector",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.ic_stats",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.maglev",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.runtime",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.runtime_stats",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.runtime_stats_sampling",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.stack_trace",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.turbofan",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.wasm.detailed",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.wasm.turbofan",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.zone_stats",
    desc: None,
    tags: &["slow"],
    group: false,
  },
  TraceCategoryDef {
    name: "v8,devtools.timeline",
    desc: None,
    tags: &[],
    group: true,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.turbofan,disabled-by-default-v8.wasm.turbofan",
    desc: None,
    tags: &[],
    group: true,
  },
  TraceCategoryDef {
    name: "disabled-by-default-v8.inspector,disabled-by-default-v8.stack_trace",
    desc: None,
    tags: &[],
    group: true,
  },
];

/// PERFETTO_DEFINE_TEST_CATEGORY_PREFIXES (测试用类别前缀, 3 项)。
pub const TEST_CATEGORY_PREFIXES: &[&str] = &["v8-cat", "cat", "v8.Test2"];
// TraceTimestampTraits<TimeTicks>::ConvertTimestampToTraceTimeNs (trace-categories.cc:24-29)
// 【推迟登记: 依赖 perfetto TrackEvent 时钟面】。
