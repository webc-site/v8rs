# v8-tracing (English)

Rust port of `v8/src/tracing` (18 files, 3654 lines, plus the tracing surface of include/v8-platform.h). Logging goes through the `log` crate.

## File → module map

| Rust module | Upstream | Lines | Scope |
|---|---|---|---|
| `src/trace_event_macros.rs` | `trace-event-no-perfetto.h` | 982 | X-macro expansion: 55 constants (54 upstream: phase 23 / flag 16 / value_type 9 / scope 6, plus 1 Rust-side prefix) + 123 macro functions (118 full + 5 typed no-ops) + 2 registered-not-expanded |
| `src/trace_event.rs` | `trace-event.h` + `trace-event.cc` | 672+63 | CategoryGroupEnabledFlags (3 bits) / kGlobalScope/kNoId / TraceId (10 ctor overloads → From) / TraceStringWithCopy / TraceValue (SetTraceValue typing) / emit×4 + scoped×2 core / ScopedTracer (Drop updates duration) / global controller install point (TraceEventHelper equivalent) |
| `src/platform.rs` | `include/v8-platform.h` (389-468) | ~80 | Embedder surface: `TracingController` trait (default no-op = upstream inline stubs) / `TraceStateObserver` / `ConvertableToTraceFormat` / `NoopController` |
| `src/traced_value.rs` | `traced-value.h` + `.cc` | 89+231 | Full `TracedValue` JSON builder: 7 escapes + `\uXXXX` uppercase hex / ECMA-262 9.8.1 shortest double (v8-base dtoa SHORTEST + conversions.cc:1178-1207 segmentation) / nesting stack |
| `src/trace_categories.rs` | `trace-categories.h` + `.cc` | 85+33 | PERFETTO_DEFINE_CATEGORIES X-macro expansion: 32 categories (27 single + 5 groups, disabled-by-default prefixes expanded in place) + 3 test prefixes; clock id (GetTraceClockId) is platform-dependent【deferred】, no constant |
| `src/category_observer.rs` | `tracing-category-observer.h` + `.cc` | 47+104 | `TracingCategoryObserver`: Mode 3 bits + 6 set / 4 clear mappings (zone_stats not cleared, upstream behavior), wired to v8-logging `TracingFlags` atomics |
| `src/trace_id.rs` | `trace-id.h` | 19 | `TraceId()` relaxed fetch_add monotonic sequence (complete) |
| `src/perfetto_utils.rs` | `perfetto-utils.h` + `.cc` | 72+33 | `PerfettoV8String` data plane: is_one_byte/size/buffer/eq (derive PartialEq)/hash (v8-base hash_range+combine)/3 encodings (Latin1/LE/BE); `Tagged<String>` readout deferred |
| (no code) | `perfetto-sdk.h` | 42 | Pure perfetto SDK include shell — registered only |
| (no code) | `code-trace-context.h` | 106 | Depends on perfetto protos — deferred |
| (no code) | `code-data-source.h` + `.cc` | 163+302 | Depends on perfetto DataSource + L3 (Isolate/Script/SharedFunctionInfo); contains `GetJsScriptType` (5 mappings) / `GetJsFunctionKind` (28 mappings) — tables land with the object layer |
| (no code) | `perfetto-logger.h` + `.cc` | 86+525 | Depends on code-events `LogEventListener` full interface + L3 — deferred |

## X-macro expansion registry highlights

- 118 expanded + 5 typed no-ops = 123 macro functions; `tool/src/tracing/trace-event-no-perfetto.h.js` is the expander, each fn doc cites the upstream macro and line
- Family folding: `ADD`→`emit` / `ADD_WITH_ID`→`emit_id` (auto `HAS_ID`) / `ADD_WITH_TIMESTAMP`→`emit_ts` / `ADD_WITH_ID_TID_AND_TIMESTAMP`→`emit_id_ts` (thread_id explicitly dropped upstream, trace-event.h:274-280) / `ADD_SCOPED`(+FLOW)→`scoped_add*` returning a `ScopedTracer` guard
- Fixed argument names preserved: counter `"value"` / async step `"step"` / clock sync `"__metadata"`, `"clock_sync"`, `"sync_id"`, `"issue_ts"` / object snapshot `"snapshot"`
- Counter values are `static_cast<int>` → Rust `i32` (upstream truncation)
- `TRACE_EVENT_COPY_NESTABLE_ASYNC_*_WITH_TTS2` keeps the literal `ASYNC_TTS|COPY` combo
- Registered-not-expanded (2): `TRACE_EVENT_METADATA1` (its `INTERNAL_TRACE_EVENT_METADATA_ADD` is undefined tree-wide) / `TRACE_EVENT_IS_NEW_TRACE` (`TRACE_EVENT_API_GET_NUM_TRACES_RECORDED` = `UNIMPLEMENTED()`, trace-event.h:87)
- 9 perfetto stub types are all empty-bodied; not generated (zero dead code), registered in the module footer
- Counting baseline: 180 `#define`s = 125 user macros (123 expanded + 2 registered) + 54 constants + 1 INTERNAL
- Macro argument evaluation timing【registered deviation】: upstream argument expressions sit inside the `if (category enabled)` block (trace-event.h:181-190) and are skipped when disabled; Rust parameters are evaluated exactly once at the call site (no multi-evaluation) — laziness would require closing over all 123 signatures, revisit with the L4 call surface
- `PerfettoV8String::hash`【registered deviation】: upstream `hash.Combine(is_one_byte_)` is a static member called through an instance with its result discarded (base/hashing.h:151), so only AddRange takes effect; this port keeps Combine per the code's intent — process-local bucketing only, no golden data bound
- `TracedValue::Default` mirrors the private upstream ctor: root container is always a dict (traced-value.cc:78-80)

## Deferred (perfetto SDK / L3 object layer)

- `CallStatsScopedTracer` (trace-event.cc:25-57) and the perfetto `TRACE_EVENT_CALL_STATS_SCOPED` branch (trace-event.h:631-669)
- perfetto four-piece set (perfetto-sdk / code-trace-context / code-data-source / perfetto-logger): protos + DataSource + Isolate/Script chain
- `TracingCategoryObserver` perfetto branch (TrackEventSessionObserver, tracing-category-observer.cc:20-27/40/85)
- `GetTraceClockId` clock id (trace-categories.cc:27): platform-dependent (linux/android/apple = kClockMonotonic, windows = kClockBootTime); no constant landed
- headless VirtualTime override point of `TRACE_TIME_NOW` (mentioned upstream, no definition body)

## Tests (34, nextest green)

Source of truth `v8/test/unittests/tracing/traced-value-unittest.cc` — all 7 cases migrated 1:1 with original gtest names, plus 17-value double-formatting golden set (conversions.cc segmentation) and self-authored coverage for the macro pipeline (8), observer lifecycle, category registry counts, perfetto-utils data plane, TracedValue default root dict. Tests rely on nextest process isolation (global controller install point).

## Conversion scripts (ch. 11 registry)

7 scripts under `tool/src/tracing/` mirroring upstream paths; skeleton-producing scripts overwrite in-place revisions on rerun — rerun only to regenerate baselines (`tool/lib.js` convention). `trace_event_macros.rs` needs `cargo fmt -p v8-tracing` after a rerun.
