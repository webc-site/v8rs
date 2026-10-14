// 由 tool/src/logging/counters.h.js 从 v8/src/logging/counters.{h,cc} + counters-scopes.h + counters-definitions.h 生成; 修订就地改, 复跑覆盖
// 计数器体系: 分辨率/直方图表/作用域族

/// 计时直方图分辨率 (原版 enum class TimedHistogramResolution)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimedHistogramResolution {
  Millisecond = 0,
  Microsecond = 1,
}

/// counters-definitions.h 宏表机械展开 (对数基准, 共 53 项)
pub mod counter_definitions {
  /// 直方图 HR(name, caption, min, max, num_buckets)
  pub const HISTOGRAMS: [&str; 53] = ["code_cache_reject_reason", "incremental_marking_reason", "incremental_marking_sum", "mark_compact_reason", "gc_finalize_clear", "gc_finalize_epilogue", "gc_finalize_evacuate", "gc_finalize_finish", "gc_finalize_mark", "gc_finalize_prologue", "gc_finalize_sweep", "gc_scavenger_scavenge_main", "gc_scavenger_scavenge_roots", "wasm_functions_per_asm_module", "wasm_functions_per_wasm_module", "array_buffer_big_allocations", "array_buffer_new_size_failures", "wasm_wasm_huge_function_size_bytes", "wasm_wasm_module_size_bytes", "wasm_compile_huge_function_peak_memory_bytes", "asm_module_size_bytes", "compile_script_cache_behaviour", "wasm_memory_allocation_result", "wasm_module_code_size_mb", "wasm_module_code_size_kb", "wasm_module_metadata_size_kb", "wasm_engine_metadata_size_kb", "wasm_module_freed_code_size_percent", "wasm_module_num_triggered_code_gcs", "wasm_module_num_code_spaces", "wasm_deopts_executed", "wasm_deopts_per_function", "wasm_revec_conversion_percent", "wasm_modules_per_isolate", "wasm_modules_per_engine", "liftoff_bailout_reasons", "wasm_memory_protection_keys_support", "wasm_avx2_support", "turbofan_ticks", "regexp_backtracks", "wasm_cache_count", "external_pointers_count", "trusted_pointers_count", "cppheap_pointers_count", "js_dispatch_table_entries_count", "external_pointer_table_compaction_outcome", "wasm_compilation_method", "esm_modules_per_page", "esm_import_graph_depth", "wasm_jit_execution_ratio", "wasm_jit_execution_too_slow", "wasm_jitless_execution_ratio", "wasm_jitless_execution_too_slow"];
}

/// 作用域族 (原版 counters-scopes.h, RAII 计时/计数守卫)
pub mod counter_scopes_skel {
  /// 作用域类清单 (对数: 5)
  pub const SCOPE_CLASSES: [&str; 5] = ["TimedHistogram", "OptionalTimedHistogram", "LazyTimedHistogram", "NestedTimedHistogram", "PauseNestedTimedHistogram"];

  /// 计时守卫语义: 进入 StartTimer, 离开 StopTimer (TimedHistogramScope 族)
  pub fn timed_scope_skel() {
    unimplemented!("骨架")
  }
  /// 计数守卫语义: 进入 Increment, 离开 Decrement (CounterScope 族)
  pub fn counter_scope_skel() {
    unimplemented!("骨架")
  }
}
