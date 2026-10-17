// 由 tool/src/tracing/trace-id.h.js 从 v8/src/tracing/trace-id.h 生成; 修订就地改, 复跑覆盖
// 全量转写 (19 行)

//! trace-id.h: 进程级单调 trace id 序列 (v8::tracing::TraceId())。

use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE_NUMBER: AtomicU64 = AtomicU64::new(0);

/// 原版 `v8::tracing::TraceId()` (trace-id.h:12-15): relaxed fetch_add 自增。
#[inline]
pub fn next_trace_id() -> u64 {
  SEQUENCE_NUMBER.fetch_add(1, Ordering::Relaxed)
}
