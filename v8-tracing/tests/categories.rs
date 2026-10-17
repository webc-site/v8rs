//! 类别注册表 / trace-id / perfetto-utils 数据面测试。

use aok::{OK, Void};
use v8_tracing::{
  PerfettoV8String, StringEncoding, TEST_CATEGORY_PREFIXES, TRACE_CATEGORIES, next_trace_id,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 32 类别 = 27 单类别 + 5 组 (trace-categories.h PERFETTO_DEFINE_CATEGORIES 展开)。
#[test]
fn trace_categories_registry_counts() -> Void {
  assert_eq!(TRACE_CATEGORIES.len(), 32);
  assert_eq!(TRACE_CATEGORIES.iter().filter(|c| c.group).count(), 5);
  assert_eq!(TRACE_CATEGORIES.iter().filter(|c| !c.group).count(), 27);
  OK
}

/// 关键条目抽查: 首项/描述/slow 标签/disabled-by-default 展开/组拼接。
#[test]
fn trace_categories_entries() -> Void {
  assert_eq!(TRACE_CATEGORIES[0].name, "cppgc");
  let mem = TRACE_CATEGORIES
    .iter()
    .find(|c| c.name == "v8.memory")
    .unwrap();
  assert_eq!(mem.desc, Some("Emits counters related to v8 memory usage."));
  // 21 个 disabled-by-default 项带 slow 标签 (trace-categories.h:28-64)
  assert_eq!(
    TRACE_CATEGORIES
      .iter()
      .filter(|c| c.tags.contains(&"slow"))
      .count(),
    21
  );
  assert!(
    TRACE_CATEGORIES
      .iter()
      .any(|c| c.name == "disabled-by-default-v8.zone_stats")
  );
  assert!(
    TRACE_CATEGORIES
      .iter()
      .any(|c| c.group && c.name == "devtools.timeline,disabled-by-default-v8.gc")
  );
  assert!(
    TRACE_CATEGORIES.iter().any(|c| c.group
      && c.name == "disabled-by-default-v8.turbofan,disabled-by-default-v8.wasm.turbofan")
  );
  OK
}

/// PERFETTO_DEFINE_TEST_CATEGORY_PREFIXES (3 前缀, trace-categories.h:15)。
/// 时钟 id (GetTraceClockId) 平台相关,【推迟登记】不落常量, 无断言。
#[test]
fn trace_categories_test_prefixes() -> Void {
  assert_eq!(TEST_CATEGORY_PREFIXES, ["v8-cat", "cat", "v8.Test2"]);
  OK
}

/// trace_disabled_by_default 前缀展开 (trace-event-no-perfetto.h:138)。
#[test]
fn trace_disabled_by_default_prefix() -> Void {
  assert_eq!(
    v8_tracing::trace_disabled_by_default("v8.gc"),
    "disabled-by-default-v8.gc"
  );
  OK
}

/// PerfettoV8String 数据面: eq(PartialEq) / hash 一致性 / 编码选择 (perfetto-utils.h)。
#[test]
fn perfetto_v8_string_face() -> Void {
  let a = PerfettoV8String::from_flat(true, b"abc");
  let b = PerfettoV8String::from_flat(true, b"abc");
  let c = PerfettoV8String::from_flat(false, b"abc");
  assert_eq!(a, b);
  assert_ne!(a, c);
  assert_eq!(a.hash(), b.hash());
  assert_ne!(a.hash(), c.hash());
  assert_eq!(a.encoding(), StringEncoding::Latin1);
  assert_eq!(c.encoding(), StringEncoding::Utf16Le); // 小端目标 (darwin/arm64)
  assert_eq!(a.size, 3);
  OK
}

/// next_trace_id: relaxed fetch_add 单调 (trace-id.h:12-15)。
#[test]
fn trace_id_sequence_monotonic() -> Void {
  let a = next_trace_id();
  let b = next_trace_id();
  let c = next_trace_id();
  assert_eq!(b, a + 1);
  assert_eq!(c, b + 1);
  OK
}
