#!/usr/bin/env bun
// v8/src/tracing/{trace-event.h, trace-event.cc} → v8-tracing/src/trace_event.rs
//
// 提取规则代码 + 签名骨架 (语义体由修订阶段就地补):
//   - CategoryGroupEnabledFlags 3 常量 (trace-event.h:35-42)
//   - kZeroNumArgs / kGlobalScope / kNoId (trace-event.h:308-310)
//   - TraceID 全部 9 个构造重载清单 (trace-event.h:332-363) → From impl 骨架
//   - AddTraceEvent / AddTraceEventWithTimestamp 0/1/2 参重载清单 (trace-event.h:476-559)
//     → Rust 收敛为 emit/emit_ts/emit_id/emit_id_ts 4 核 + scoped 2 核
//   - ScopedTracer / CallStatsScopedTracer 签名 (trace-event.h:562-623, trace-event.cc:25-57)
//   - trace-event.cc:19 GetTracingController → 全局 controller 访问骨架
// 复跑幂等; 修订就地改, 复跑覆盖 (仅限重生成基线)。
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-tracing";
const GEN = "src/tracing/trace-event.h.js";
const H = readV8("src/tracing/trace-event.h");
const CC = readV8("src/tracing/trace-event.cc");

// ---- CategoryGroupEnabledFlags: enum 值机械提取 ----
// kEnabledForRecording → ENABLED_FOR_RECORDING (驼峰转蛇口)
const cgFlags = [...H.matchAll(
  /k(\w+)_CategoryGroupEnabledFlags = (1 << \d+)/g,
)].map((m) => [
  `ENABLED_${m[1].replace(/([a-z])([A-Z])/g, "$1_$2").toUpperCase()}`,
  m[2],
]);
if (cgFlags.length !== 3) throw new Error(`CategoryGroupEnabledFlags ${cgFlags.length} != 3`);

// ---- kZeroNumArgs / kNoId ----
const zeroArgs = H.match(/const int kZeroNumArgs = (\d+)/)[1];
const noId = H.match(/const uint64_t kNoId = (\d+)/)[1];

// ---- TraceID 构造重载清单 (登记用): 9 原生类型 + WithScope = 10 ----
const rawCtors = [...H.matchAll(/^  TraceID\(([^)]+), unsigned int\* flags?\)/gm)]
  .map((m) => m[1].trim());
if (rawCtors.length !== 10) throw new Error(`TraceID 重载 ${rawCtors.length} != 10`);

// ---- AddTraceEvent 重载数 ----
const addOverloads = (H.match(/static V8_INLINE uint64_t AddTraceEvent(?:WithTimestamp)?\(/g) || []).length;

let out = `${banner(GEN, "src/tracing/trace-event.h;src/tracing/trace-event.cc", "签名骨架 + 常量, 语义体就地修订")}
//! trace-event.h/.cc 核心管道 (672+63 行)。
//!
//! 原版链路: \`TRACE_EVENT*\` 宏 (→ trace_event_macros) →
//! \`AddTraceEvent[WithTimestamp]\` 模板重载 (${addOverloads} 个, 0/1/2 参, trace-event.h:476-559) →
//! \`AddTraceEventImpl\` (extract convertable, trace-event.h:383) →
//! \`TracingController::AddTraceEvent\` (embedder 虚函数, include/v8-platform.h:419)。
//! Rust 收敛: 重载塌缩为 \`emit\`/\`emit_ts\`/\`emit_id\`/\`emit_id_ts\` 4 核 + \`scoped_add\`(2);
//! \`SetTraceValue\` 类型打标 (trace-event.h:431-468) → \`TraceValue::type_tag\`。
//!
//! \`CategoryGroupEnabledFlags\` (trace-event.h:35-42): GetCategoryGroupEnabled 返回字节的位含义。
//!
//! 【推迟登记】(trace-event.cc:25-57): \`CallStatsScopedTracer\` 依赖
//! \`Isolate::counters()->runtime_call_stats()\` (L3 对象层), 待 v8-execution 回收;
//! \`TraceEventHelper::GetTracingController\` 的 \`V8::GetCurrentPlatform\` 全局链
//! (trace-event.cc:19-21) 以本模块全局 controller 安装点等价承接。

use std::sync::{Mutex, OnceLock};

use crate::platform::{ConvertableToTraceFormat, TracingController};

// ---- CategoryGroupEnabledFlags (trace-event.h:35-42) ----
${cgFlags.map(([n, v]) => `pub const ${n}: u8 = ${v};`).join("\n")}

/// kZeroNumArgs (trace-event.h:308)。
pub const K_ZERO_NUM_ARGS: i32 = ${zeroArgs};
/// kNoId (trace-event.h:310)。
pub const K_NO_ID: u64 = ${noId};
/// kGlobalScope = nullptr scope (trace-event.h:309)。
pub const K_GLOBAL_SCOPE: Option<&'static str> = None;

// TODO(修订): TraceID ${rawCtors.length} 构造重载 → From<impl> 落地
//   ${rawCtors.join(" / ")}
// TODO(修订): SetTraceValue (bool/整型/枚举→INT|UINT|BOOL, double→位模式, ptr, str, copy-str, convertable)
// TODO(修订): AddTraceEventImpl + ${addOverloads} 重载 → emit 核 4 + scoped 2
// TODO(修订): ScopedTracer (trace-event.h:562) Drop 补 UpdateTraceEventDuration
`;

writeSrc(CRATE, "trace_event.rs", out);
console.log(`== trace-event.h.js == cgFlags=3 zeroArgs=${zeroArgs} noId=${noId} traceIdCtors=${rawCtors.length} addOverloads=${addOverloads}`);
console.log(`TraceID 重载: ${rawCtors.join(", ")}`);
