#!/usr/bin/env bun
// v8/src/tracing/trace-id.h → v8-tracing/src/trace_id.rs (19 行, 全量规则代码)
import { readV8, writeSrc, banner } from "../../lib.js";

const H = readV8("src/tracing/trace-id.h");
if (!/V8_INLINE uint64_t TraceId\(\)/.test(H)) throw new Error("TraceId 函数未找到");
// fetch_add(1, memory_order_relaxed) 语义核对
if (!/fetch_add\(1, std::memory_order_relaxed\)/.test(H)) throw new Error("fetch_add 语义不符");

let out = `${banner("src/tracing/trace-id.h.js", "src/tracing/trace-id.h", "全量转写 (19 行)")}
//! trace-id.h: 进程级单调 trace id 序列 (v8::tracing::TraceId())。

use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE_NUMBER: AtomicU64 = AtomicU64::new(0);

/// 原版 \`v8::tracing::TraceId()\` (trace-id.h:12-15): relaxed fetch_add 自增。
#[inline]
pub fn next_trace_id() -> u64 {
  SEQUENCE_NUMBER.fetch_add(1, Ordering::Relaxed)
}
`;

writeSrc("v8-tracing", "trace_id.rs", out);
console.log("== trace-id.h.js == 全量 1 函数");
