// 由 tool/src/flags/flag-definitions.h.js 从 v8/src/flags/flag-definitions.h 生成; 修订就地改, 复跑覆盖
// 规范配置: macOS arm64 release (见脚本 CONFIG); 生成物, 修订就地改

use crate::value::{Cow, FlagType, FlagValue, Flags};

/// 每 flag 的静态描述 (对应原版 Flag + kFlagsMetadata)
pub struct FlagDef {
  pub name: &'static str,
  pub ty: FlagType,
  pub readonly: bool,
  pub comment: &'static str,
  pub default: fn() -> FlagValue,
}
// 别名不单列表: ALL_ENTRIES 的 flag_index/canonical_index 已完整表达别名→主 flag 映射
#[derive(Copy, Clone)]
pub struct EntryMeta {
  pub name: &'static str,
  pub comment: Option<&'static str>,
  pub flag_index: u32,
  pub canonical_index: u32,
}

pub static FLAGS: &[FlagDef] = &[
  FlagDef {
    name: "experimental",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Indicates that V8 is running with experimental features enabled. This flag is typically not set explicitly but instead enabled as an implication of other flags which enable experimental features.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "array_destructure_bytecode",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable experimental ArrayDestructure bytecode (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "test_only_unsafe",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Indicates that V8 is running in an unsupported and unsafe configuration, e.g. used for internal testing. This flag is typically not set explicitly but instead enabled as an implication of other flags.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "disallow_unsafe_flags",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Prevents the use of flags that are considered unsafe. Setting this flag will make V8 treat unsafe flags as flag contradictions and either crash, exit gracefully with a message, or reset them (depending on the current FlagProcessingMode).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "developer_only_features",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Indicates that V8 is running with developer-only features enabled.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "disallow_developer_only_features",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Disallow developer-only-features.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "flag_processing_mode",
    ty: FlagType::Str,
    readonly: false,
    comment: "Control behavior for flag errors. Possible values: abort-on-error, exit-on-error, ignore-contradictions.",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "use_strict",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enforce strict mode",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_temporal",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace temporal code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "js_postmessage_share_immutable_arraybuffer",
    ty: FlagType::Bool,
    readonly: false,
    comment: "share immutable array buffers in postMessage instead of copying",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "builtin_subclassing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "subclassing support in built-in methods",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_sharedarraybuffer_per_context",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable the SharedArrayBuffer constructor per context",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "js_nonextensible_applies_to_private",
    ty: FlagType::Bool,
    readonly: false,
    comment: "non-extensibility includes private fields",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "for_of_optimization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Experimental flag for improving ForOf loops on ArrayIterator (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_queue_microtask",
    ty: FlagType::Bool,
    readonly: false,
    comment: "ship a native queueMicrotask implementation on the global object (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "icu_timezone_data",
    ty: FlagType::Bool,
    readonly: false,
    comment: "get information about timezones from ICU",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "icu_datetime_compat_lang",
    ty: FlagType::Str,
    readonly: false,
    comment: "limits ICU date time compat changes to the given language",
    default: || FlagValue::Str(Some(Cow::Borrowed("*"))),
  },
  FlagDef {
    name: "intl_date_time_pattern_generator_cache_eviction",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable 9-entry cache eviction in DateTimePatternGeneratorCache",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "use_icu4x_normalizer",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use ICU4X normalizer",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_snapshot",
    ty: FlagType::Bool,
    readonly: false,
    comment: "disables sharing of the read-only heap for testing",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "lite_mode",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enables trade-off of performance for memory savings",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_allocation_folding",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Use allocation folding globally",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "disable_write_barriers",
    ty: FlagType::Bool,
    readonly: true,
    comment: "disable write barriers when GC is non-incremental and heap contains single generation.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_unconditional_write_barriers",
    ty: FlagType::Bool,
    readonly: true,
    comment: "always use full write barriers",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "single_generation",
    ty: FlagType::Bool,
    readonly: true,
    comment: "allocate all objects from young generation to old generation",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "conservative_stack_scanning",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use conservative stack scanning (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "direct_handle",
    ty: FlagType::Bool,
    readonly: true,
    comment: "use direct handles with conservative stack scanning",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "scavenger_conservative_object_pinning",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Objects reachable from the native stack during scavenge will be pinned and won't move.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_scavenger_conservative_object_pinning",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Treat some precise references as conservative references to stress test object pinning in Scavenger",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_scavenger_conservative_object_pinning_random",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enables random stressing of object pinning in Scavenger, such that each GC would randomly pick a subset of the precise references to treat conservatively",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "scavenger_precise_object_pinning",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Objects reachable from handles during scavenge will be pinned and won't move.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "scavenger_updates_allocation_limit",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Scavenger updates allocation limits if allocation rate is low",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "precise_object_pinning",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Objects reachable from handles during GC will be pinned and won't move.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "scavenger_promote_quarantined_pages",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Quarantined pages in the intermediate generation will be promoted to old space",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "scavenger_chaos_mode",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Scavenger will ignore age when making promotion decisions and instead choose objects to be promoted at random. This only applies to non-large objects.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "scavenger_chaos_mode_threshold",
    ty: FlagType::Uint,
    readonly: false,
    comment: "Percentage of non-large young objects that will be promoted during Scavenger in chaos mode",
    default: || FlagValue::Uint(50u32),
  },
  FlagDef {
    name: "local_off_stack_check",
    ty: FlagType::Bool,
    readonly: true,
    comment: "check for off-stack allocation of v8::Local",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "force_emit_interrupt_budget_checks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "force emit tier-up logic from all non-turbofan code, even if it is the top enabled tier",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable the maglev optimizing compiler",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_future",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable maglev features that we want to ship in the not-too-far future (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "optimize_on_next_call_optimizes_to_maglev",
    ty: FlagType::Bool,
    readonly: false,
    comment: "make OptimizeFunctionOnNextCall optimize to maglev instead of turbofan",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_maglev",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trigger maglev compilation earlier",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_max_small_graph_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "max inlined maglev graph size that counts as a small function",
    default: || FlagValue::Int(10i32),
  },
  FlagDef {
    name: "maglev_as_top_tier",
    ty: FlagType::Bool,
    readonly: false,
    comment: "disable Turbofan/Turbolev, make Maglev optimize more aggressively",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_inlining",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable inlining in the maglev optimizing compiler",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_disable_builtin_reducers",
    ty: FlagType::Bool,
    readonly: false,
    comment: "disable eager builtin reducers in the maglev graph builder to flush out production bugs in post-inlining builtin reduction",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_loop_peeling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable loop peeling in the maglev optimizing compiler",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_optimistic_peeled_loops",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable aggressive optimizations for loops (loop SPeeling) in the maglev optimizing compiler",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_loop_peeling_max_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "max loop size for loop peeling in the maglev optimizing compiler",
    default: || FlagValue::Int(400i32),
  },
  FlagDef {
    name: "maglev_loop_peeling_max_size_cumulative",
    ty: FlagType::Int,
    readonly: false,
    comment: "max cumulative size for loop peeling in the maglev optimizing compiler",
    default: || FlagValue::Int(900i32),
  },
  FlagDef {
    name: "maglev_deopt_data_on_background",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Generate deopt data on background thread",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_build_code_on_background",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Generate code on background thread",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_destroy_on_background",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Destroy compilation jobs on background thread",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_inline_api_calls",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Inline CallApiCallback builtin into generated code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_cons_string_elision",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Native support for cons strings and their elision in maglev.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_pretenure_store_values",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Recursively pretenure values which are stored into pretenured allocation sites.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_poly_calls",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Support (inlining) polymorphic calls",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_truncation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable Maglev truncation pass",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_maglev_truncation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace maglev truncation pass (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_licm",
    ty: FlagType::Bool,
    readonly: false,
    comment: "loop invariant code motion (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbolev_truncated_int32_phis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable truncated to int32 representation in the phi selector",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_range_analysis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable Maglev range value analysis pass",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_maglev_range_analysis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Trace Maglev range value analysis pass (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev_kna",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Trace Maglev known node aspects (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_use_unreliable_maps",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use unreliable unstable maps in Maglev",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_range_verification",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Run integer range verifiction pass in Turbolev frontend pipeline",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_verify_dominance",
    ty: FlagType::Bool,
    readonly: false,
    comment: "verify dominance of eager deopt frame inputs",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "concurrent_maglev_max_threads",
    ty: FlagType::Uint,
    readonly: false,
    comment: "max number of threads that concurrent Maglev can use (0 for unbounded)",
    default: || FlagValue::Uint(2u32),
  },
  FlagDef {
    name: "concurrent_maglev_high_priority_threads",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use high priority compiler threads for concurrent Maglev",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "max_maglev_inline_depth",
    ty: FlagType::Int,
    readonly: false,
    comment: "max depth of functions that Maglev will inline excl. small functions",
    default: || FlagValue::Int(1i32),
  },
  FlagDef {
    name: "max_maglev_hard_inline_depth",
    ty: FlagType::Int,
    readonly: false,
    comment: "max depth of functions that Maglev will inline incl. small functions",
    default: || FlagValue::Int(10i32),
  },
  FlagDef {
    name: "max_maglev_inlined_bytecode_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of bytecode for a single inlining",
    default: || FlagValue::Int(100i32),
  },
  FlagDef {
    name: "max_maglev_inlined_bytecode_size_cumulative",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum cumulative size of bytecode considered for inlining excl. small functions",
    default: || FlagValue::Int(920i32),
  },
  FlagDef {
    name: "max_maglev_inlined_bytecode_size_small_total",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of small functions' bytecode that can be inlining",
    default: || FlagValue::Int(3000i32),
  },
  FlagDef {
    name: "max_maglev_inlined_bytecode_size_small",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of bytecode considered for small function inlining",
    default: || FlagValue::Int(27i32),
  },
  FlagDef {
    name: "max_maglev_inlined_bytecode_size_small_with_heapnum_in_out",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of bytecode considered for small function inlining, when the function call has a HeapNumber as input or output",
    default: || FlagValue::Int(75i32),
  },
  FlagDef {
    name: "min_maglev_inlining_frequency",
    ty: FlagType::Float,
    readonly: false,
    comment: "minimum frequency for inlining",
    default: || FlagValue::Float(0.95),
  },
  FlagDef {
    name: "max_turbolev_inline_depth",
    ty: FlagType::Int,
    readonly: false,
    comment: "max depth of functions that Maglev will inline excl. small functions",
    default: || FlagValue::Int(50i32),
  },
  FlagDef {
    name: "maglev_reuse_stack_slots",
    ty: FlagType::Bool,
    readonly: false,
    comment: "reuse stack slots in the maglev optimizing compiler",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_untagged_phis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable phi untagging in the maglev optimizing compiler",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_untagged_phis_bisect_limit",
    ty: FlagType::Int,
    readonly: false,
    comment: "limit the number of Phis we untag in Maglev phi untagging (-1 for unbounded)",
    default: || FlagValue::Int(-1i32),
  },
  FlagDef {
    name: "turbolev_untagged_phis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable phi untagging in the Turbolev optimizing compiler",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_hoist_osr_value_phi_untagging",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable phi untagging to hoist untagging of osr values",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_speculative_hoist_phi_untagging",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable phi untagging to hoist untagging of loop phi inputs (could still cause deopt loops) (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_cse",
    ty: FlagType::Bool,
    readonly: false,
    comment: "common subexpression elimination",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_non_eager_inlining",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Maglev non-eager inlining",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbolev_non_eager_inlining",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turbolev non-eager inlining",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbolev_non_eager_loop_peeling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turbolev non-eager loop peeling",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbolev_eager_loop_peeling_osr",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use eager loop peeling in Turbolev OSR compiles",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbolev_trace_loop_peeling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace turbolev non-eager loop peeling decisions (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_filter",
    ty: FlagType::Str,
    readonly: false,
    comment: "optimization filter for the maglev compiler",
    default: || FlagValue::Str(Some(Cow::Borrowed("*"))),
  },
  FlagDef {
    name: "maglev_print_filter",
    ty: FlagType::Str,
    readonly: false,
    comment: "filter for maglev's tracing/printing options",
    default: || FlagValue::Str(Some(Cow::Borrowed("*"))),
  },
  FlagDef {
    name: "maglev_assert",
    ty: FlagType::Bool,
    readonly: false,
    comment: "insert extra assertion in maglev code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_assert_stack_size",
    ty: FlagType::Bool,
    readonly: true,
    comment: "insert stack size checks before every IR node",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_assert_types",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable additional type assertions in generated code (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_break_on_entry",
    ty: FlagType::Bool,
    readonly: false,
    comment: "insert an int3 on maglev entries",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_print_bytecode",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print bytecode for maglev compiled code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_print_feedback",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print feedback vector for maglev compiled code",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_print_inlined",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print bytecode / feedback vectors also for inlined code",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_print_provenance",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print Maglev node provenance, ie, file name, bytecode and offset",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "print_maglev_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print maglev code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_with_compilation_id",
    ty: FlagType::Bool,
    readonly: false,
    comment: "prefix maglev/turbolev trace line with a compilation id",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_maglev_graph_building",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace maglev graph building (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev_graph_optimizer",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace maglev graph optimizer (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev_loop_speeling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace maglev loop SPeeling (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev_inlining",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace maglev inlining (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev_kna_processor",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace maglev known node aspects processor (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev_scope_info",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace maglev scope info (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace Maglev IR to JSON (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev_file_prefix",
    ty: FlagType::Str,
    readonly: false,
    comment: "write maglev trace files with given prefix",
    default: || FlagValue::Str(Some(Cow::Borrowed("maglev"))),
  },
  FlagDef {
    name: "print_maglev_deopt_verbose",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print verbose deopt info",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_maglev_graph",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print the final maglev graph",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_maglev_graphs",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print maglev graph across all phases",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev_phi_untagging",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace maglev phi untagging",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev_regalloc",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace maglev register allocation",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_stats",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print Maglev statistics (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_stats_nvp",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print Maglev statistics in machine-readable format (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_function_context_specialization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable function context specialization in maglev",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "additive_safe_int_feedback",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Record AdditiveSafeInteger feedback",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbolev_additive_safe_int_feedback",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable the use of AdditiveSafeInteger feedback for Turbolev",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbofan_additive_safe_int_feedback",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Enable the use of AdditiveSafeInteger feedback for TurboFan",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_enumerated_keyed_access_bytecode",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable generating GetEnumeratedKeyedProperty bytecode for keyed access",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "dict_property_const_tracking",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Use const tracking on dictionary properties",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "script_context_cells",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use context cells in script contexts, ie, const tracking let and mutable numbers",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "function_context_cells",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use context cells in function contexts",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "function_context_cells_max_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "Max number of context slots allowed in a function context with context cells",
    default: || FlagValue::Int(2i32),
  },
  FlagDef {
    name: "empty_context_extension_dep",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use compilation dependency to avoid dynamic checks for non-empty context extensions",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "json_stringify_fast_path",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable JSON.stringify fast-path",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "json_parse_max_heuristically_internalized_strings",
    ty: FlagType::Uint,
    readonly: false,
    comment: "Maximum number of strings that are eagerly internalized during JSON.parse() based on heuristics. 0 means only strings that must be internalized e.g. property keys will be internalized.",
    default: || FlagValue::Uint(100000u32),
  },
  FlagDef {
    name: "trace_json_parse_internalization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Print heuristic internalization stats after each JSON.parse() (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "cache_property_key_string_adds",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable caching property keys created by concatenating strings",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "smi_string_cache_size",
    ty: FlagType::Uint,
    readonly: false,
    comment: "Full size of SmiStringCache",
    default: || FlagValue::Uint(8192u32),
  },
  FlagDef {
    name: "double_string_cache_size",
    ty: FlagType::Uint,
    readonly: false,
    comment: "Full size of DoubleStringCache",
    default: || FlagValue::Uint(4096u32),
  },
  FlagDef {
    name: "trace_number_string_cache",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Print NumberString caches stats on GC and at the end (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_inline_date_accessors",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Inline Date accessors by Maglev",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbofan_inline_date_accessors",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Inline Date accessors by TurboFan",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "inline_date_accessors",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Inline Date accessors",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "extensible_ro_snapshot",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Whether custom embedder snapshots may extend ReadOnlySpace",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "max_opt",
    ty: FlagType::Uint,
    readonly: false,
    comment: "Set the maximal optimisation tier: > 3 == any, 0 == ignition/interpreter, 1 == sparkplug/baseline, 2 == maglev, 3 == turbofan",
    default: || FlagValue::Uint(999u32),
  },
  FlagDef {
    name: "efficiency_mode",
    ty: FlagType::MaybeBool,
    readonly: false,
    comment: "Forces efficiency mode on or off, disregarding any dynamic signals. Efficiency mode is optimized for situations with no latency requirements and uses fewer threads.",
    default: || FlagValue::MaybeBool(None),
  },
  FlagDef {
    name: "battery_saver_mode",
    ty: FlagType::MaybeBool,
    readonly: false,
    comment: "Forces battery saver mode on or off, disregarding any dynamic signals. Battery saver tries to conserve overall cpu cycles spent.",
    default: || FlagValue::MaybeBool(None),
  },
  FlagDef {
    name: "memory_saver_mode",
    ty: FlagType::MaybeBool,
    readonly: false,
    comment: "Forces memory saver mode on or off, disregarding any dynamic signals. Memory saver tries to keep memory footprint low at the expense of extra cpu cycles.",
    default: || FlagValue::MaybeBool(None),
  },
  FlagDef {
    name: "efficiency_mode_disable_turbofan",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Defer tier-up to turbofan while in efficiency mode.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "efficiency_mode_delay_turbofan_multiply",
    ty: FlagType::Int,
    readonly: false,
    comment: "Delay tier-up to turbofan to a certain invocation count multipier while in efficiency mode.",
    default: || FlagValue::Int(3i32),
  },
  FlagDef {
    name: "wasm_trace_native",
    ty: FlagType::Str,
    readonly: false,
    comment: "Select which native code sequence to use for wasm trace instruction: default or cpuid",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "jitless",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Disable runtime allocation of executable memory.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "disable_optimizing_compilers",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Disable all optimizing compilers while leaving baseline compilers enabled",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "memory_protection_keys",
    ty: FlagType::Bool,
    readonly: false,
    comment: "protect code memory with PKU if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "force_memory_protection_keys",
    ty: FlagType::Bool,
    readonly: false,
    comment: "fail if memory protection key support is not available",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "assert_types",
    ty: FlagType::Bool,
    readonly: false,
    comment: "generate runtime type assertions to test the typer",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_assert_types",
    ty: FlagType::Bool,
    readonly: false,
    comment: "generate runtime type assertions to test the turboshaft type system (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "verify_simplified_lowering",
    ty: FlagType::Bool,
    readonly: false,
    comment: "verify graph generated by simplified lowering",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_compilation_dependencies",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace code dependencies (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "unmap_holes",
    ty: FlagType::Bool,
    readonly: false,
    comment: "unmap the page containing the holes.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "allocation_site_tracking",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Enable allocation site tracking",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "allocation_site_pretenuring",
    ty: FlagType::Bool,
    readonly: false,
    comment: "pretenure with allocation sites",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "page_promotion",
    ty: FlagType::Bool,
    readonly: false,
    comment: "promote pages based on utilization",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "page_promotion_threshold",
    ty: FlagType::Int,
    readonly: false,
    comment: "min percentage of live bytes on a page to enable fast evacuation in full GCs",
    default: || FlagValue::Int(70i32),
  },
  FlagDef {
    name: "minor_ms_page_promotion_threshold",
    ty: FlagType::Int,
    readonly: false,
    comment: "min percentage of live bytes on a page to enable fast evacuation in MinorMS",
    default: || FlagValue::Int(50i32),
  },
  FlagDef {
    name: "minor_ms_page_promotion_max_lab_threshold",
    ty: FlagType::Int,
    readonly: false,
    comment: "max percentage of labs out of a page to still be considered for page promotion",
    default: || FlagValue::Int(30i32),
  },
  FlagDef {
    name: "minor_ms_max_page_age",
    ty: FlagType::Uint,
    readonly: false,
    comment: "max age for a page after which it is force promoted to old space",
    default: || FlagValue::Uint(4u32),
  },
  FlagDef {
    name: "trace_page_promotions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace page promotion decisions (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_pretenuring",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace pretenuring decisions of HAllocate instructions (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_pretenuring_statistics",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace allocation site pretenuring statistics (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_resize_large_object",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace resizing of large objects (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "track_field_types",
    ty: FlagType::Bool,
    readonly: false,
    comment: "track field types",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_block_coverage",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace collected block coverage information (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_protector_invalidation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace protector cell invalidations (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "zero_unused_memory",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Zero unused memory (except for memory which was discarded) on memory reducing GCs.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "high_end_android",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enables high-end mode unconditionally for Android.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "high_end_android_physical_memory_threshold",
    ty: FlagType::Uint,
    readonly: false,
    comment: "Enables high-end mode for devices with more than X GB of physical memory (if X greater than 0).",
    default: || FlagValue::Uint(8u32),
  },
  FlagDef {
    name: "minor_ms_shortcut_strings",
    ty: FlagType::Bool,
    readonly: true,
    comment: "short cut strings during marking",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "feedback_normalization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "feed back normalization to constructors (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "internalize_on_the_fly",
    ty: FlagType::Bool,
    readonly: true,
    comment: "internalize string keys for generic keyed ICs on the fly",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "unbox_double_arrays",
    ty: FlagType::Bool,
    readonly: false,
    comment: "automatically unbox arrays of doubles",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "string_slices",
    ty: FlagType::Bool,
    readonly: true,
    comment: "use string slices",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "invocation_count_for_feedback_allocation",
    ty: FlagType::Int,
    readonly: false,
    comment: "invocation count required for allocating feedback vectors",
    default: || FlagValue::Int(8i32),
  },
  FlagDef {
    name: "invocation_count_for_maglev",
    ty: FlagType::Int,
    readonly: false,
    comment: "invocation count required for optimizing with Maglev",
    default: || FlagValue::Int(400i32),
  },
  FlagDef {
    name: "invocation_count_for_maglev_osr",
    ty: FlagType::Int,
    readonly: false,
    comment: "invocation count required for maglev OSR",
    default: || FlagValue::Int(100i32),
  },
  FlagDef {
    name: "osr_from_maglev",
    ty: FlagType::Int,
    readonly: false,
    comment: "bitset mode for OSR from Maglev to Turbofan (0=off, 1=on OSR compile, 2=if loop was OSR'd, 4=always)",
    default: || FlagValue::Int(3i32),
  },
  FlagDef {
    name: "osr_from_maglev_interrupt_scale_factor",
    ty: FlagType::Float,
    readonly: false,
    comment: "Scale interrupt budget reduction for OSR from Maglev vs. OSR to Maglev",
    default: || FlagValue::Float(0.8),
  },
  FlagDef {
    name: "always_osr_from_maglev",
    ty: FlagType::Bool,
    readonly: false,
    comment: "whether we try to OSR to Turbofan from any Maglev (alias for --osr-from-maglev=4)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "invocation_count_for_turbofan",
    ty: FlagType::Int,
    readonly: false,
    comment: "invocation count required for optimizing with TurboFan",
    default: || FlagValue::Int(3000i32),
  },
  FlagDef {
    name: "invocation_count_for_osr",
    ty: FlagType::Int,
    readonly: false,
    comment: "invocation count required for OSR",
    default: || FlagValue::Int(500i32),
  },
  FlagDef {
    name: "invocation_count_for_osr_factor_while_tiering_in_progress",
    ty: FlagType::Float,
    readonly: false,
    comment: "invocation count factor while waiting for code to compile",
    default: || FlagValue::Float(3.0),
  },
  FlagDef {
    name: "osr_to_tierup",
    ty: FlagType::Uint,
    readonly: false,
    comment: "number to decrease the invocation budget by when we follow OSR",
    default: || FlagValue::Uint(1u32),
  },
  FlagDef {
    name: "minimum_invocations_after_ic_update",
    ty: FlagType::Int,
    readonly: false,
    comment: "How long to minimally wait after IC update before tier up",
    default: || FlagValue::Int(500i32),
  },
  FlagDef {
    name: "minimum_invocations_before_optimization",
    ty: FlagType::Int,
    readonly: false,
    comment: "Minimum number of invocations we need before non-OSR optimization",
    default: || FlagValue::Int(2i32),
  },
  FlagDef {
    name: "jit_fuzzing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Set JIT tiering thresholds suitable for JIT fuzzing",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "use_std_math_pow",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use std::pow instead of our custom implementation",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "use_ic",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use inline caching",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "lazy_feedback_allocation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Allocate feedback vectors lazily",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "stress_ic",
    ty: FlagType::Bool,
    readonly: false,
    comment: "exercise interesting paths in ICs more often",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "ignition_elide_noneffectful_bytecodes",
    ty: FlagType::Bool,
    readonly: false,
    comment: "elide bytecodes which won't have any external effect",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "ignition_reo",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use ignition register equivalence optimizer",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "ignition_filter_expression_positions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "filter expression positions before the bytecode pipeline",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "ignition_share_named_property_feedback",
    ty: FlagType::Bool,
    readonly: false,
    comment: "share feedback slots when loading the same named property from the same object",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "ignition_elide_redundant_tdz_checks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "elide TDZ checks dominated by other TDZ checks",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "print_bytecode",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print bytecode generated by ignition interpreter (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_lazy_source_positions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "skip generating source positions during initial compile but regenerate when actually required",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_lazy_source_positions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "collect lazy source positions immediately after lazy compile",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_lazy",
    ty: FlagType::Bool,
    readonly: false,
    comment: "stress lazy compilation",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_bytecode_filter",
    ty: FlagType::Str,
    readonly: false,
    comment: "filter for selecting which functions to print bytecode",
    default: || FlagValue::Str(Some(Cow::Borrowed("*"))),
  },
  FlagDef {
    name: "omit_default_ctors",
    ty: FlagType::Bool,
    readonly: false,
    comment: "omit calling default ctors in bytecode",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_ignition_codegen",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace the codegen of ignition interpreter bytecode handlers (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_track_allocation_sites",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace the tracking of allocation sites (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_migration",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace object migration (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_generalization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace map generalization (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sparkplug",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Sparkplug baseline compiler",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "always_sparkplug",
    ty: FlagType::Bool,
    readonly: false,
    comment: "directly tier up to Sparkplug code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "baseline_batch_compilation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "batch compile Sparkplug code",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "concurrent_sparkplug",
    ty: FlagType::Bool,
    readonly: true,
    comment: "compile Sparkplug code in a background thread",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "concurrent_sparkplug_max_threads",
    ty: FlagType::Uint,
    readonly: false,
    comment: "max number of threads that concurrent Sparkplug can use (0 for unbounded)",
    default: || FlagValue::Uint(1u32),
  },
  FlagDef {
    name: "concurrent_sparkplug_high_priority_threads",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use high priority compiler threads for concurrent Sparkplug",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sparkplug_filter",
    ty: FlagType::Str,
    readonly: false,
    comment: "filter for Sparkplug baseline compiler",
    default: || FlagValue::Str(Some(Cow::Borrowed("*"))),
  },
  FlagDef {
    name: "sparkplug_needs_short_builtins",
    ty: FlagType::Bool,
    readonly: false,
    comment: "only enable Sparkplug baseline compiler when --short-builtin-calls are also enabled",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "baseline_batch_compilation_threshold",
    ty: FlagType::Int,
    readonly: false,
    comment: "the estimated instruction size of a batch to trigger compilation",
    default: || FlagValue::Int(4096i32),
  },
  FlagDef {
    name: "trace_baseline",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace baseline compilation (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_baseline_batch_compilation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace baseline batch compilation (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "shared_strings",
    ty: FlagType::Bool,
    readonly: false,
    comment: "allow sharing of strings",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "shared_string_table",
    ty: FlagType::Bool,
    readonly: false,
    comment: "internalize strings into shared table (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "always_use_string_forwarding_table",
    ty: FlagType::Bool,
    readonly: true,
    comment: "use string forwarding table instead of thin strings for all strings (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "transition_strings_during_gc_with_stack",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Transition strings during a full GC with stack",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "initial_shared_heap_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "initial size of the shared heap (in Mbytes); other heap size flags (e.g. initial_heap_size) take precedence",
    default: || FlagValue::SizeT(0usize),
  },
  FlagDef {
    name: "max_shared_heap_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "max size of the shared heap (in Mbytes); other heap size flags (e.g. max_shared_heap_size) take precedence",
    default: || FlagValue::SizeT(0usize),
  },
  FlagDef {
    name: "concurrent_builtin_generation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "generate builtins concurrently on separate threads in mksnapshot",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "concurrent_recompilation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "optimizing hot functions asynchronously on a separate thread",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_concurrent_recompilation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "track concurrent recompilation (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "concurrent_recompilation_queue_length",
    ty: FlagType::Int,
    readonly: false,
    comment: "the length of the concurrent compilation queue",
    default: || FlagValue::Int(8i32),
  },
  FlagDef {
    name: "concurrent_recompilation_delay",
    ty: FlagType::Int,
    readonly: false,
    comment: "artificial compilation delay in ms",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "concurrent_recompilation_front_running",
    ty: FlagType::Bool,
    readonly: false,
    comment: "move compile jobs to the front if recompilation is requested multiple times",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "concurrent_turbofan_max_threads",
    ty: FlagType::Uint,
    readonly: false,
    comment: "max number of threads that concurrent Turbofan can use (0 for unbounded)",
    default: || FlagValue::Uint(4u32),
  },
  FlagDef {
    name: "stress_concurrent_inlining",
    ty: FlagType::Bool,
    readonly: false,
    comment: "create additional concurrent optimization jobs but throw away result",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_overwrite_budget",
    ty: FlagType::Bool,
    readonly: false,
    comment: "whether maglev resets the interrupt budget",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_overwrite_osr_budget",
    ty: FlagType::Bool,
    readonly: false,
    comment: "whether maglev resets the OSR interrupt budget",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_concurrent_inlining_attach_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "create additional concurrent optimization jobs",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "max_serializer_nesting",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum levels for nesting child serializers",
    default: || FlagValue::Int(25i32),
  },
  FlagDef {
    name: "trace_heap_broker_verbose",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace the heap broker verbosely (all reports) (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_heap_broker",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace the heap broker (reports on missing data only) (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_runs",
    ty: FlagType::Int,
    readonly: false,
    comment: "number of stress runs",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "deopt_every_n_times",
    ty: FlagType::Int,
    readonly: false,
    comment: "deoptimize every n times a deopt point is passed",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "print_deopt_stress",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print number of possible deopt points",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "disable_loop_stack_checks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "disable loop stack checks (for testing/fuzzing only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbofan",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use the Turbofan optimizing compiler for JavaScript code (Wasm uses separate flags)",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_always_optimize_spills",
    ty: FlagType::Bool,
    readonly: false,
    comment: "optimize placement of all spill instructions, not just loop-top phis",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_wasm_address_reassociation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "refactor address components for immediate indexing",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "concurrent_turbo_tracing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "allow concurrent compilation to happen in combination with trace-turbo* flags",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "optimize_maglev_optimizes_to_turbofan",
    ty: FlagType::Bool,
    readonly: false,
    comment: "make OptimizeMaglevOnNextCall optimize to turbofan instead of maglev",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_filter",
    ty: FlagType::Str,
    readonly: false,
    comment: "optimization filter for TurboFan compiler",
    default: || FlagValue::Str(Some(Cow::Borrowed("*"))),
  },
  FlagDef {
    name: "trace_turbo",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace generated TurboFan IR (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_path",
    ty: FlagType::Str,
    readonly: false,
    comment: "directory to dump generated TurboFan IR to",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "trace_turbo_filter",
    ty: FlagType::Str,
    readonly: false,
    comment: "filter for tracing turbofan compilation",
    default: || FlagValue::Str(Some(Cow::Borrowed("*"))),
  },
  FlagDef {
    name: "trace_turbo_graph",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace generated TurboFan graphs (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_scheduled",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace TurboFan IR with schedule (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_file_prefix",
    ty: FlagType::Str,
    readonly: false,
    comment: "trace turbo graph to a file with given prefix",
    default: || FlagValue::Str(Some(Cow::Borrowed("turbo"))),
  },
  FlagDef {
    name: "trace_turbo_cfg_file",
    ty: FlagType::Str,
    readonly: false,
    comment: "trace turbo cfg graph (for C1 visualizer) to a given file name",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "trace_turbo_types",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace TurboFan's types",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_turbo_scheduler",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace TurboFan's scheduler",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_reduction",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace TurboFan's various reducers",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_bailouts",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace TurboFan's various bailouts",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_trimming",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace TurboFan's graph trimmer",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_jt",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace TurboFan's jump threading",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_ceq",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace TurboFan's control equivalence",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_loop",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace TurboFan's loop optimizations",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_alloc",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace TurboFan's register allocator",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_all_uses",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace all use positions",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_representation",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace representation types",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_stack_accesses",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace stack load/store counters for optimized code in run-time (x64 only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "fuzzing_and_concurrent_recompilation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "fuzzing && concurrent_recompilation",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_verify",
    ty: FlagType::Bool,
    readonly: false,
    comment: "verify TurboFan graphs at each phase",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_verify_machine_graph",
    ty: FlagType::Str,
    readonly: false,
    comment: "verify TurboFan machine graph before instruction selection",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "verify_csa",
    ty: FlagType::Bool,
    readonly: true,
    comment: "verify TurboFan machine graph of code stubs",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_verify_csa",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace code stubs verification (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "csa_trap_on_node",
    ty: FlagType::Str,
    readonly: false,
    comment: "trigger break point when a node with given id is created in given stub. The format is: StubName,NodeId",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "builtins_effects_src",
    ty: FlagType::Str,
    readonly: false,
    comment: "path where to generate builtins-effects.h (used for mksnapshot only)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "fixed_array_bounds_checks",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable FixedArray bounds checks",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_stats",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print TurboFan statistics",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_stats_nvp",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print TurboFan statistics in machine-readable format",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_stats_wasm",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print TurboFan statistics of wasm compilations",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_splitting",
    ty: FlagType::Bool,
    readonly: true,
    comment: "split nodes during scheduling in TurboFan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_inlining",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable inlining in TurboFan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "forin_enumerator_holder",
    ty: FlagType::Bool,
    readonly: false,
    comment: "optimize for-in over packed elements using ForInEnumeratorHolder",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_elide_frames",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable frame elision in TurboFan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "max_inlined_bytecode_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of bytecode for a single inlining",
    default: || FlagValue::Int(460i32),
  },
  FlagDef {
    name: "max_inlined_bytecode_size_cumulative",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum cumulative size of bytecode considered for inlining",
    default: || FlagValue::Int(920i32),
  },
  FlagDef {
    name: "max_inlined_bytecode_size_absolute",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum absolute size of bytecode considered for inlining",
    default: || FlagValue::Int(4600i32),
  },
  FlagDef {
    name: "max_inlined_bytecode_size_small_total",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of small functions' bytecode that can be inlining",
    default: || FlagValue::Int(30000i32),
  },
  FlagDef {
    name: "reserve_inline_budget_scale_factor",
    ty: FlagType::Float,
    readonly: false,
    comment: "scale factor of bytecode size used to calculate the inlining budget",
    default: || FlagValue::Float(1.2),
  },
  FlagDef {
    name: "max_inlined_bytecode_size_small",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of bytecode considered for small function inlining",
    default: || FlagValue::Int(30i32),
  },
  FlagDef {
    name: "max_inlined_bytecode_size_small_with_heapnum_in_out",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of bytecode considered for small function inlining, when the function call has a HeapNumber as input or output",
    default: || FlagValue::Int(75i32),
  },
  FlagDef {
    name: "max_optimized_bytecode_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum bytecode size to be considered for turbofan optimization; too high values may cause the compiler to hit (release) assertions",
    default: || FlagValue::Int(61440i32),
  },
  FlagDef {
    name: "max_maglev_optimized_bytecode_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum bytecode size to be considered for maglev optimization",
    default: || FlagValue::Int(524288i32),
  },
  FlagDef {
    name: "min_inlining_frequency",
    ty: FlagType::Float,
    readonly: false,
    comment: "minimum frequency for inlining",
    default: || FlagValue::Float(0.15),
  },
  FlagDef {
    name: "stress_inline",
    ty: FlagType::Bool,
    readonly: false,
    comment: "set high thresholds for inlining to inline as much as possible",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_inlining",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace TurboFan inlining (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_inline_array_builtins",
    ty: FlagType::Bool,
    readonly: false,
    comment: "inline array builtins in TurboFan code",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "use_osr",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use on-stack replacement",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_osr",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use maglev as on-stack replacement target",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "max_maglev_eager_inlined_bytecode_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of bytecode considered for eager inlining",
    default: || FlagValue::Int(30i32),
  },
  FlagDef {
    name: "max_turbolev_eager_inlined_bytecode_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of bytecode considered for eager inlining",
    default: || FlagValue::Int(30i32),
  },
  FlagDef {
    name: "concurrent_osr",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable concurrent OSR",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "maglev_allocation_folding",
    ty: FlagType::Int,
    readonly: false,
    comment: "maglev allocation folding level",
    default: || FlagValue::Int(2i32),
  },
  FlagDef {
    name: "maglev_escape_analysis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "avoid inlined allocation of objects that cannot escape",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_maglev_escape_analysis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace maglev escape analysis (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maglev_object_tracking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "track object changes to avoid escaping them (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_maglev_object_tracking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace load/stores from maglev virtual objects (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_string_builder",
    ty: FlagType::Bool,
    readonly: true,
    comment: "use TurboFan fast string builder",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_osr",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace on-stack replacement (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_or_trace_osr",
    ty: FlagType::Bool,
    readonly: false,
    comment: "internal helper flag, please use --trace-osr instead.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "analyze_environment_liveness",
    ty: FlagType::Bool,
    readonly: false,
    comment: "analyze liveness of environment slots and zap dead values",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_environment_liveness",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace liveness of local variable slots (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_load_elimination",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable load elimination in TurboFan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_turbo_load_elimination",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace TurboFan load elimination (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_profiling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable basic block profiling in TurboFan",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_profiling_verbose",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable basic block profiling in TurboFan, and include each function's schedule and disassembly in the output",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_profiling_output",
    ty: FlagType::Str,
    readonly: false,
    comment: "emit data about basic block usage in builtins to this file (requires that V8 was built with v8_enable_builtins_profiling=true)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "reorder_builtins",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable builtin reordering when run mksnapshot.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "abort_on_bad_builtin_profile_data",
    ty: FlagType::Bool,
    readonly: false,
    comment: "flag for mksnapshot, abort if builtins profile can't be applied",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "warn_about_builtin_profile_data",
    ty: FlagType::Bool,
    readonly: false,
    comment: "flag for mksnapshot, emit warnings when applying builtin profile data",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "dump_builtins_hashes_to_file",
    ty: FlagType::Str,
    readonly: false,
    comment: "flag for mksnapshot, dump CSA builtins graph hashes to this file",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "turbo_verify_allocation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "verify register allocation in TurboFan",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_move_optimization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "optimize gap moves in TurboFan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_jt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable jump threading in TurboFan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_loop_peeling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "TurboFan loop peeling",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_loop_variable",
    ty: FlagType::Bool,
    readonly: false,
    comment: "TurboFan loop variable optimization",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_loop_rotation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "TurboFan loop rotation",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_cf_optimization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "optimize control flow in TurboFan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_escape",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable escape analysis",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_allocation_folding",
    ty: FlagType::Bool,
    readonly: false,
    comment: "TurboFan allocation folding",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_instruction_scheduling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable instruction scheduling in TurboFan",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "experimental_turbo_instruction_scheduling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable experimental instruction scheduling in TurboFan",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_stress_instruction_scheduling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "randomly schedule instructions to stress dependency tracking",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_instruction_scheduling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace TurboShaft's instruction scheduler",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_store_elimination",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable store-store elimination in TurboFan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_store_elimination",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace store elimination (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_typer_hardening",
    ty: FlagType::Bool,
    readonly: true,
    comment: "extra bounds checks to protect against some known typer mismatch exploit techniques (best effort)",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_rewrite_far_jumps",
    ty: FlagType::Bool,
    readonly: true,
    comment: "rewrite far to near jumps (ia32,x64)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_gc_during_compilation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "simulate GC/compiler thread race related to https://crbug.com/v8/8520",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_fast_api_calls",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable fast API calls from TurboFan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "fast_api_allow_float_in_sim",
    ty: FlagType::Bool,
    readonly: false,
    comment: "allow float parameters to be passed in simulator mode",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_inline_js_wasm_calls",
    ty: FlagType::Bool,
    readonly: false,
    comment: "inline JS->Wasm calls (specifically: inline JS-to-Wasm wrappers and then the body of the Wasm function, if applicable)",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_optimize_inlined_js_wasm_wrappers",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Run additional optimizations (especially load-elimination) on inlined JS-to-Wasm wrappers",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_optimize_apply",
    ty: FlagType::Bool,
    readonly: false,
    comment: "optimize Function.prototype.apply",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turbo_optimize_math_minmax",
    ty: FlagType::Bool,
    readonly: false,
    comment: "optimize call math.min/max with double array",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turboshaft",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable TurboFan's Turboshaft phases for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turboshaft_enable_debug_features",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enables Turboshaft's DebugPrint, StaticAssert and CheckTurboshaftTypeOf operations",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_wasm_load_elimination",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turboshaft's WasmLoadElimination",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_in_js_inlining_body",
    ty: FlagType::Bool,
    readonly: false,
    comment: "inline Wasm code into JS functions via Turboshaft. This requires inlining the wrapper, see --wasm-in-js-inlining-wrapper. For controlling the inlining in the old Turbofan pipeline, see --turbo-inline-js-wasm-calls.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_in_js_inlining_wrapper",
    ty: FlagType::Bool,
    readonly: false,
    comment: "inline JS-to-Wasm wrappers via Turboshaft/Turbolev.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_in_js_inlining_opt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable optimizations for Wasm-in-JS inlining in Turboshaft (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_load_elimination",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turboshaft's low-level load elimination for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turboshaft_loop_unrolling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turboshaft's loop unrolling",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turboshaft_loop_optimization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turboshaft's loop optimization phase (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_random_rescheduling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turboshaft's random rescheduling phase (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_string_concat_escape_analysis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turboshaft's escape analysis for string concatenation",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turboshaft_trusted_load_elimination",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turboshaft's low level load elimination for trusted loads (JS and Wasm)",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "turboshaft_typed_optimizations",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable an additional Turboshaft phase that performs optimizations based on type information (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_simd_opt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable optimizations for Webassembly SIMD",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "future_wasm_simd_opt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable extra optimizations for Webassembly SIMD (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_deinterleave_loads",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable deinterleaving loads for Webassembly SIMD (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbolev",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use Turbolev (≈ Maglev + Turboshaft combined) as the 4th tier compiler instead of Turbofan",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_turbolev_frontend",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print Turbolev frontend (Maglev graphs) (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbolev_phase_filter",
    ty: FlagType::Str,
    readonly: false,
    comment: "filter for turbolev phases (comma-separated list of phase names)",
    default: || FlagValue::Str(Some(Cow::Borrowed("*"))),
  },
  FlagDef {
    name: "print_turbolev_inline_functions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print Turbolev inline functions (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbolev_future",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turbolev features that we want to ship in the not-too-far future (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbolev_escape_analysis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable escape analysis for Turbolev (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbolev_escape_analysis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable tracing for Turbolev's escape analysis phase (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "typed_array_length_loading",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable specializing loading the TypedArray length in Maglev / Turbofan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "deduplicate_heap_number_requests",
    ty: FlagType::Bool,
    readonly: false,
    comment: "avoid duplicated HeapNumber materializations in HeapNumberRequests",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_trace_reduction",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace individual Turboshaft reduction steps",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_trace_emitted",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace emitted Turboshaft instructions",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_trace_intermediate_reductions",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace intermediate Turboshaft reduction steps",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_trace_load_elimination",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace Turboshaft's late load elimination",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_trace_store_store_elimination",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace Turboshaft's store store elimination",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turboshaft_trace_if_else_to_switch",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace Turboshaft's if-else to switch reducer",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "verify_turboshaft",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable various verifications on Turboshaft",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "profile_guided_optimization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "profile guided optimization",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "profile_guided_optimization_for_empty_feedback_vector",
    ty: FlagType::Bool,
    readonly: false,
    comment: "profile guided optimization for empty feedback vector",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "invocation_count_for_early_optimization",
    ty: FlagType::Int,
    readonly: false,
    comment: "invocation count threshold for early optimization",
    default: || FlagValue::Int(30i32),
  },
  FlagDef {
    name: "invocation_count_for_maglev_with_delay",
    ty: FlagType::Int,
    readonly: false,
    comment: "invocation count for maglev for functions which according to profile_guided_optimization are likely to deoptimize before reaching this invocation count",
    default: || FlagValue::Int(600i32),
  },
  FlagDef {
    name: "optimize_for_size",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enables optimizations which favor memory size over execution speed",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "reopt_after_lazy_deopts",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Immediately re-optimize code after some lazy deopts",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "verify_get_js_builtin_state",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable verification of Builtins::GetJSBuiltinState().",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "experimental_fuzzing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Implies all experimental pre-staged features that can already be fuzzed but are not ready for staging yet. (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "future",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Implies all staged features that we want to ship in the not-too-far future",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "harmony",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable all completed harmony features",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "js_staging",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable all completed JavaScript features",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_staging",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable staged wasm features",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "js_shipping",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable all shipped JavaScript features",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "harmony_shipping",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable all shipped harmony features",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "harmony_shadow_realm",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable harmony ShadowRealm for JS (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "harmony_struct",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable harmony structs, shared structs, and shared arrays for JS (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "harmony_intl_best_fit_matcher",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Intl BestFitMatcher for JS (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "js_decorators",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable decorators for JS (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "js_source_phase_imports",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable source phase imports for JS (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "js_regexp_buffer_boundaries",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable RegExp \\A \\z \\Z buffer boundary assertions for JS (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_instruction_tracing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable instruction tracing section for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_shared",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable shared-everything threads for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_fp16",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable fp16 for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_memory_control",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable memory control for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_wasmfx",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable core stack switching for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_compilation_hints",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable compilation hints for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_growable_stacks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable growable stacks for jspi for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "always_specialize_for_script_context",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable always specialize for script contexts in optimized code (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_stringref",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable reference-typed strings for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_imported_strings_utf8",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable imported strings (utf8 features) for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_acquire_release",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable acquire_release memory ordering for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_compact_imports",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable compact import section for Wasm (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "js_immutable_arraybuffer",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Immutable ArrayBuffer for JS",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "js_import_bytes",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable import bytes for JS",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "js_defer_import_eval",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable defer import eval for JS",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_custom_descriptors",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable custom descriptors for Wasm",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_wide_arithmetic",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable wide arithmetic for Wasm",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "harmony_import_attributes",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable harmony import attributes for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "harmony_temporal",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Temporal for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "js_esm_ns_reexport",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Support diamond-importing re-expored namespaces (https://github.com/tc39/ecma262/pull/3715) for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "js_iterator_join",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Iterator.prototype.join for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "js_iterator_sequencing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable iterator sequencing for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "js_joint_iteration",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable joint iteration for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "js_pr_3883",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Promise.try not wrapping the result in an extra promise in the non-throwing case (https://github.com/tc39/ecma262/pull/3883) for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "js_iterator_includes",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Iterator.prototype.includes for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "js_import_text",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable import text for JS",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_legacy_eh",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable legacy exception handling opcodes for Wasm",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "homomorphic_ic",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Homomorphic IC state for same-handler highly polymorphic ICs",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_generic_wrapper",
    ty: FlagType::Bool,
    readonly: false,
    comment: "allow use of the generic js-to-wasm wrapper instead of per-signature wrappers",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_unsafe_fast_api_wrapper",
    ty: FlagType::Bool,
    readonly: false,
    comment: "allow use of the unsafe fast API wrapper (test-only / unsafe)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_num_compilation_tasks",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum number of parallel compilation tasks for wasm",
    default: || FlagValue::Int(128i32),
  },
  FlagDef {
    name: "trace_wasm_native_heap",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace wasm native heap events",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_offheap_memory",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print details of wasm off-heap memory when the memory measurement API is used (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_wasm_offheap_memory_size",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print off-heap memory consumption per module and engine when they die",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_serialization",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace serialization/deserialization",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_test_streaming",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use streaming compilation instead of async compilation for tests",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_native_module_cache",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable the native module cache",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_max_mem_pages",
    ty: FlagType::Uint,
    readonly: false,
    comment: "maximum number of 64KiB memory pages per wasm memory",
    default: || FlagValue::Uint(4294967295u32),
  },
  FlagDef {
    name: "wasm_max_table_size",
    ty: FlagType::Uint,
    readonly: false,
    comment: "maximum table size of a wasm instance",
    default: || FlagValue::Uint(10000000u32),
  },
  FlagDef {
    name: "wasm_max_committed_code_mb",
    ty: FlagType::Uint,
    readonly: false,
    comment: "maximum committed code space for wasm (in MB)",
    default: || FlagValue::Uint(4095u32),
  },
  FlagDef {
    name: "wasm_max_code_space_size_mb",
    ty: FlagType::Uint,
    readonly: false,
    comment: "maximum size of a single wasm code space",
    default: || FlagValue::Uint(128u32),
  },
  FlagDef {
    name: "wasm_tier_up",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable tier up to the optimizing compiler (requires --liftoff to have an effect)",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_dynamic_tiering",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable dynamic tier up to the optimizing compiler",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_sync_tier_up",
    ty: FlagType::Bool,
    readonly: false,
    comment: "run Wasm tier up jobs synchronously",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_tiering_budget",
    ty: FlagType::Int,
    readonly: false,
    comment: "budget for dynamic tiering (rough approximation of bytes executed",
    default: || FlagValue::Int(13000000i32),
  },
  FlagDef {
    name: "wasm_wrapper_tiering_budget",
    ty: FlagType::Int,
    readonly: false,
    comment: "budget for wrapper tierup (number of calls until tier-up)",
    default: || FlagValue::Int(1000i32),
  },
  FlagDef {
    name: "max_wasm_functions",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum number of wasm functions defined in a module",
    default: || FlagValue::Int(1000000i32),
  },
  FlagDef {
    name: "wasm_caching_threshold",
    ty: FlagType::Int,
    readonly: false,
    comment: "the amount of wasm top tier code that triggers the next caching event",
    default: || FlagValue::Int(1000i32),
  },
  FlagDef {
    name: "wasm_caching_hard_threshold",
    ty: FlagType::Int,
    readonly: false,
    comment: "the amount of wasm top tier code that triggers caching immediately, ignoring the --wasm-caching-timeout-ms",
    default: || FlagValue::Int(1000000i32),
  },
  FlagDef {
    name: "wasm_caching_timeout_ms",
    ty: FlagType::Int,
    readonly: false,
    comment: "only trigger caching if no new code was compiled within this timeout (0 to disable this logic and only use --wasm-caching-threshold)",
    default: || FlagValue::Int(2000i32),
  },
  FlagDef {
    name: "trace_wasm_compilation_times",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print how long it took to compile each wasm function (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_tier_up_filter",
    ty: FlagType::Int,
    readonly: false,
    comment: "only tier-up function with this index",
    default: || FlagValue::Int(-1i32),
  },
  FlagDef {
    name: "wasm_eager_tier_up_function",
    ty: FlagType::Int,
    readonly: false,
    comment: "eagerly tier-up function with this index",
    default: || FlagValue::Int(-1i32),
  },
  FlagDef {
    name: "trace_wasm_decoder",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace decoding of wasm code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_compiler",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace compiling of wasm code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_streaming",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace streaming compilation of wasm code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_stack_switching",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace wasm stack switching",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_wasm_stack_switching",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Always run wasm on a secondary stack, even when it is called with a regular (non-JSPI) export",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_stack_switching_stack_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "default size of stacks for wasm stack-switching (in kB)",
    default: || FlagValue::Int(984i32),
  },
  FlagDef {
    name: "wasm_stack_pool_capacity_mb",
    ty: FlagType::Int,
    readonly: false,
    comment: "default capacity for the wasm stack pool in MB, -1 for unlimited",
    default: || FlagValue::Int(500i32),
  },
  FlagDef {
    name: "liftoff",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Liftoff, the baseline compiler for WebAssembly",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "liftoff_only",
    ty: FlagType::Bool,
    readonly: false,
    comment: "disallow TurboFan compilation for WebAssembly (for testing)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_testing_opcode_in_wasm",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enables a testing opcode in wasm that is only implemented in TurboFan",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_liftoff",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace Liftoff, the baseline compiler for WebAssembly",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_memory",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print all memory updates performed in wasm code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_globals",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print all global variable updates performed in wasm code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_tier_mask_for_testing",
    ty: FlagType::Int,
    readonly: false,
    comment: "bitmask of declared(!) function indices to compile with TurboFan instead of Liftoff",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "wasm_debug_mask_for_testing",
    ty: FlagType::Int,
    readonly: false,
    comment: "bitmask of declared(!) function indices to compile for debugging, only applies if the tier is Liftoff",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "wasm_pgo_to_file",
    ty: FlagType::Bool,
    readonly: false,
    comment: "experimental: dump Wasm PGO information to a local file (test-only / unsafe)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_pgo_from_file",
    ty: FlagType::Bool,
    readonly: false,
    comment: "experimental: read and use Wasm PGO data from a local file (test-only / unsafe)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "dump_wasm_module",
    ty: FlagType::Bool,
    readonly: true,
    comment: "dump wasm module bytes",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "dump_wasm_module_path",
    ty: FlagType::Str,
    readonly: false,
    comment: "directory to dump wasm modules to",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "wasm_fast_api",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable direct calls from wasm to fast API functions with bound call function to pass the receiver as first parameter (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_assert_types",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable additional type assertions in generated code (e.g. turbofan) (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_deopt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable deopts in optimized wasm functions",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_deopts_per_function_limit",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "limit of wasm deopts for a single function after which no further deopt points are emitted in Turbofan",
    default: || FlagValue::SizeT(10usize),
  },
  FlagDef {
    name: "wasm_assume_ref_cast_succeeds",
    ty: FlagType::Bool,
    readonly: false,
    comment: "assume ref.cast always succeeds and skip the related type check (test-only / unsafe)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_ref_cast_nop",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable unsafe ref.cast_nop instruction (test-only / unsafe)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_skip_null_checks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "skip null checks for call.ref and array and struct operations (test-only / unsafe)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_skip_bounds_checks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "skip array bounds checks (test-only / unsafe)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_js_interop",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable JS Interop part of Custom Descriptors proposal (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_custom_descriptors_permitted",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Emergency off-switch for Custom Descriptors Origin Trial",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_merged_descriptors",
    ty: FlagType::Bool,
    readonly: false,
    comment: "merge Custom Descriptor into v8::internal::Map (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_opt",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable optimization when compiling Wasm functions with Turbofan",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_bounds_checks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable bounds checks (disable for performance testing only)",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_stack_checks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable stack checks (disable for performance testing only)",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_enforce_bounds_checks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enforce explicit bounds check even if the trap handler is available",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_partial_oob_writes_are_noops",
    ty: FlagType::Bool,
    readonly: false,
    comment: "assume partially out-of-bounds writes are no-ops and can use the trap handler. Note that explicitly enabling this flag (if not enabled by default) can affect the spec-compliance of V8.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_math_intrinsics",
    ty: FlagType::Bool,
    readonly: false,
    comment: "intrinsify some Math imports into wasm",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_inlining",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable inlining of Wasm functions into Wasm functions",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_inlining_budget",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "maximum graph size (in TF nodes) that allows inlining more",
    default: || FlagValue::SizeT(5000usize),
  },
  FlagDef {
    name: "wasm_inlining_max_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "maximum function size (in wire bytes) that may be inlined",
    default: || FlagValue::SizeT(500usize),
  },
  FlagDef {
    name: "wasm_inlining_factor",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "maximum multiple graph size (in TF nodes) in comparison to initial size",
    default: || FlagValue::SizeT(3usize),
  },
  FlagDef {
    name: "wasm_inlining_min_budget",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "minimum graph size budget (in TF nodes) for which the wasm_inlinining_factor does not apply",
    default: || FlagValue::SizeT(50usize),
  },
  FlagDef {
    name: "wasm_inlining_ignore_call_counts",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Ignore call counts when considering inlining candidates. The flag is supposed to be used for fuzzing",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_inlining",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace wasm inlining (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_typer",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace wasm typer (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_simd_shuffle",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace wasm simd shuffle (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_inlining_call_indirect",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable speculative inlining of Wasm indirect calls",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_bulkmem_inlining",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable inline code generation for small bulk memory operations",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_loop_unrolling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable loop unrolling for wasm functions",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_random_rescheduling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Turbofan's random rescheduling phase for wasm functions (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_loop_peeling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable loop peeling for wasm functions",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_loop_peeling_max_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "maximum size for peeling",
    default: || FlagValue::SizeT(1000usize),
  },
  FlagDef {
    name: "trace_wasm_loop_peeling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace wasm loop peeling (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_fuzzer_gen_test",
    ty: FlagType::Bool,
    readonly: false,
    comment: "generate a test case when running a wasm fuzzer",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_wasm_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print WebAssembly code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_wasm_code_function_index",
    ty: FlagType::Int,
    readonly: false,
    comment: "print WebAssembly code for function at index",
    default: || FlagValue::Int(-1i32),
  },
  FlagDef {
    name: "print_wasm_stub_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print WebAssembly stub code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_lazy_compilation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable lazy compilation for all wasm modules",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_wasm_lazy_compilation",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace lazy compilation of wasm functions",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_code_gc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable garbage collection of wasm code",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_wasm_code_gc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace garbage collection of wasm code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_wasm_code_gc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "stress test garbage collection of wasm code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_max_initial_code_space_reservation",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum size of the initial wasm code space reservation (in MB)",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "stress_wasm_memory_moving",
    ty: FlagType::Bool,
    readonly: false,
    comment: "always move non-shared bounds-checked Wasm memory on grow",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "flush_liftoff_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable flushing Liftoff code on memory pressure signal",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "stress_branch_hinting",
    ty: FlagType::Bool,
    readonly: false,
    comment: "stress branch hinting by generating a random hint for each branch instruction",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_max_module_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "maximum allowed size of wasm modules",
    default: || FlagValue::SizeT(1073741824usize),
  },
  FlagDef {
    name: "wasm_disassembly_max_mb",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "maximum size of produced disassembly (in MB, approximate)",
    default: || FlagValue::SizeT(1000usize),
  },
  FlagDef {
    name: "wasm_capi_thread_pool_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "size of the thread pool used by the wasm C API, with 0 meaning the maximum number of threads",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "trace_wasm",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace wasm function calls (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_instances",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace creation and collection of wasm instances",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_memory64_trap_handling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use trap handling for Wasm memory64 bounds checks",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "wasm_jitless",
    ty: FlagType::Bool,
    readonly: true,
    comment: "execute all Wasm code in the Wasm interpreter",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_jitless_if_available_for_testing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_allow_mixed_eh_for_testing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Allow mixed legacy and new exception handling instructions in the same wasm module",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_code_coverage",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable Wasm code coverage",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_compilation_hints",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace compilation hints parsing and usage (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "wasm_generate_compilation_hints",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable emitting compilation-hints sections for Wasm to a file (test-only / unsafe)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_wasm_generate_compilation_hints",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable tracing wasm compilation hints generation (test-only / unsafe)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_sampling_allocation_profiler",
    ty: FlagType::Int,
    readonly: false,
    comment: "Enables sampling allocation profiler with X as a sample interval",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "lazy_new_space_shrinking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enables the lazy new space shrinking strategy",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "min_semi_space_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "min size of a semi-space (in MBytes), the new space consists of two semi-spaces",
    default: || FlagValue::SizeT(0usize),
  },
  FlagDef {
    name: "max_semi_space_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "max size of a semi-space (in MBytes), the new space consists of two semi-spaces",
    default: || FlagValue::SizeT(0usize),
  },
  FlagDef {
    name: "semi_space_growth_factor",
    ty: FlagType::Int,
    readonly: false,
    comment: "factor by which to grow the new space",
    default: || FlagValue::Int(2i32),
  },
  FlagDef {
    name: "max_old_space_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "max size of the old space (in Mbytes)",
    default: || FlagValue::SizeT(0usize),
  },
  FlagDef {
    name: "max_heap_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "max size of the heap (in Mbytes) both max_semi_space_size and max_old_space_size take precedence. All three flags cannot be specified at the same time.",
    default: || FlagValue::SizeT(0usize),
  },
  FlagDef {
    name: "initial_heap_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "initial size of the heap (in Mbytes)",
    default: || FlagValue::SizeT(0usize),
  },
  FlagDef {
    name: "initial_old_space_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "initial old space size (in Mbytes)",
    default: || FlagValue::SizeT(0usize),
  },
  FlagDef {
    name: "preconfigured_old_space_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "preconfigured old space size (in Mbytes)",
    default: || FlagValue::SizeT(32usize),
  },
  FlagDef {
    name: "gc_global",
    ty: FlagType::Bool,
    readonly: false,
    comment: "always perform global GCs",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "random_gc_interval",
    ty: FlagType::Int,
    readonly: false,
    comment: "Collect garbage after random(0, X) V8 allocations. It overrides gc_interval.",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "gc_interval",
    ty: FlagType::Int,
    readonly: false,
    comment: "garbage collect after <n> allocations",
    default: || FlagValue::Int(-1i32),
  },
  FlagDef {
    name: "cppgc_random_gc_interval",
    ty: FlagType::Int,
    readonly: false,
    comment: "Collect garbage after random(0, X) cppgc allocations.",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "retain_maps_for_n_gc",
    ty: FlagType::Int,
    readonly: false,
    comment: "keeps maps alive for <n> old space garbage collections",
    default: || FlagValue::Int(2i32),
  },
  FlagDef {
    name: "trace_gc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print one trace line following each garbage collection (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_gc_nvp",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print one detailed trace line in name=value format after each garbage collection (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_gc_ignore_scavenger",
    ty: FlagType::Bool,
    readonly: false,
    comment: "do not print trace line after scavenger collection (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_memory_reducer",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print memory reducer behavior (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_gc_verbose",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print more details following each garbage collection (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_gc_freelists",
    ty: FlagType::Bool,
    readonly: false,
    comment: "prints details of each freelist before and after each major garbage collection (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_gc_freelists_verbose",
    ty: FlagType::Bool,
    readonly: false,
    comment: "prints details of freelists of each page before and after each major garbage collection (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_gc_heap_layout",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print layout of pages in heap before and after gc (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_gc_heap_layout_ignore_minor_gc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "do not print trace line before and after minor-gc",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_evacuation_candidates",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Show statistics about the pages evacuation by the compaction (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_pending_allocations",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace calls to Heap::IsAllocationPending that return true (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_allocation_stack_interval",
    ty: FlagType::Int,
    readonly: false,
    comment: "print stack trace after <n> free-list allocations",
    default: || FlagValue::Int(-1i32),
  },
  FlagDef {
    name: "trace_duplicate_threshold_kb",
    ty: FlagType::Int,
    readonly: false,
    comment: "print duplicate objects in the heap if their size is more than given threshold",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "trace_fragmentation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "report fragmentation for old space (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_fragmentation_verbose",
    ty: FlagType::Bool,
    readonly: false,
    comment: "report fragmentation for old space (detailed) (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "minor_ms_trace_fragmentation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace fragmentation after marking (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_evacuation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "report evacuation statistics (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_mutator_utilization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print mutator utilization, allocation speed, gc speed (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "incremental_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use incremental marking",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "incremental_marking_task",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use tasks for incremental marking",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "incremental_marking_soft_trigger",
    ty: FlagType::Int,
    readonly: false,
    comment: "threshold for starting incremental marking via a task in percent of available space: limit - size",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "incremental_marking_hard_trigger",
    ty: FlagType::Int,
    readonly: false,
    comment: "threshold for starting incremental marking immediately in percent of available space: limit - size",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "incremental_marking_unified_schedule",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use a single schedule for determining a marking schedule between JS and C++ objects.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_unmapper",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Trace the unmapping (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "parallel_scavenge",
    ty: FlagType::Bool,
    readonly: false,
    comment: "parallel scavenge",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "minor_gc_task",
    ty: FlagType::Bool,
    readonly: false,
    comment: "schedule minor GC tasks",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "minor_gc_task_trigger",
    ty: FlagType::Uint,
    readonly: false,
    comment: "minor GC task trigger in percent of the current heap limit",
    default: || FlagValue::Uint(80u32),
  },
  FlagDef {
    name: "minor_gc_task_with_lower_priority",
    ty: FlagType::Bool,
    readonly: false,
    comment: "schedules the minor GC task with kUserVisible priority.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_parallel_scavenge",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace parallel scavenge (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "cppgc_young_generation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "run young generation garbage collections in Oilpan (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "optimize_gc_for_battery",
    ty: FlagType::Bool,
    readonly: false,
    comment: "optimize GC for battery",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "concurrent_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use concurrent marking",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "concurrent_marking_max_worker_num",
    ty: FlagType::Uint,
    readonly: false,
    comment: "max worker number of concurrent marking, 0 for NumberOfWorkerThreads",
    default: || FlagValue::Uint(7u32),
  },
  FlagDef {
    name: "concurrent_array_buffer_sweeping",
    ty: FlagType::Bool,
    readonly: false,
    comment: "concurrently sweep array buffers",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "stress_concurrent_allocation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "start background threads that allocate memory",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "parallel_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use parallel marking in atomic pause",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "ephemeron_fixpoint_iterations",
    ty: FlagType::Int,
    readonly: false,
    comment: "number of fixpoint iterations it takes to switch to linear ephemeron algorithm",
    default: || FlagValue::Int(10i32),
  },
  FlagDef {
    name: "trace_concurrent_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace concurrent marking (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "concurrent_sweeping",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use concurrent sweeping",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "parallel_compaction",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use parallel compaction",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "parallel_pointer_update",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use parallel pointer update during compaction",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "parallel_gc_clearing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use parallel threads to clear weak refs in the atomic pause.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "detect_ineffective_gcs_near_heap_limit",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trigger out-of-memory failure to avoid GC storm near heap limit",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enforce_global_heap_limit",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enforce global size by causing an out-of-memory failure above heap limit",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "maximum_global_heap_limit_factor",
    ty: FlagType::Int,
    readonly: false,
    comment: "Ratio from v8 to global maximum heap size",
    default: || FlagValue::Int(8i32),
  },
  FlagDef {
    name: "ineffective_gc_size_threshold",
    ty: FlagType::Float,
    readonly: false,
    comment: "Threshold on heap size to trigger out-of-memory failure near heap limit.",
    default: || FlagValue::Float(0.95),
  },
  FlagDef {
    name: "ineffective_gc_mutator_utilization_threshold",
    ty: FlagType::Float,
    readonly: false,
    comment: "Threshold on mutator utilization to trigger out-of-memory failure near heap limit.",
    default: || FlagValue::Float(0.4),
  },
  FlagDef {
    name: "trace_incremental_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace progress of the incremental marking (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_gc_clearing_dependency_graph",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print clearing dependency graph in dot format (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_stress_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace stress marking progress (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_stress_scavenge",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace stress scavenge progress (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "track_gc_object_stats",
    ty: FlagType::Bool,
    readonly: false,
    comment: "track object counts and memory usage (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_gc_object_stats",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace object counts and memory usage (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_zone_stats",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace zone memory usage (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "zone_stats_tolerance",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "report a tick only when allocated zone memory changes by this amount",
    default: || FlagValue::SizeT(1048576usize),
  },
  FlagDef {
    name: "trace_zone_type_stats",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace per-type zone memory usage (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_backing_store",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace backing store events",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "gc_stats",
    ty: FlagType::Int,
    readonly: false,
    comment: "Used by tracing internally to enable gc statistics",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "trace_gc_object_stats_all_objects",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace all objects on each gc (warning: large output) (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "track_detached_contexts",
    ty: FlagType::Bool,
    readonly: false,
    comment: "track native contexts that are expected to be garbage collected",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_detached_contexts",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace native contexts that are expected to be garbage collected (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "verify_heap",
    ty: FlagType::Bool,
    readonly: true,
    comment: "verify heap pointers before and after GC",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "verify_write_barriers",
    ty: FlagType::Bool,
    readonly: true,
    comment: "verify skipped write barriers",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "safepoint_bump_qos_class",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Bump priority for running threads to reach safepoint",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "freeze_forces_marking_finalization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Forces finalizing marking when frozen state is notified",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "move_object_start",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable moving of object starts",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "memory_reducer",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use memory reducer",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "memory_reducer_for_small_heaps",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use memory reducer for small heaps",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "memory_reducer_gc_count",
    ty: FlagType::Uint,
    readonly: false,
    comment: "Maximum number of memory reducer GCs scheduled",
    default: || FlagValue::Uint(2u32),
  },
  FlagDef {
    name: "memory_reducer_delay_ms",
    ty: FlagType::Uint,
    readonly: false,
    comment: "Delay before memory reducer start",
    default: || FlagValue::Uint(8000u32),
  },
  FlagDef {
    name: "gc_memory_reducer_start_delay_ms",
    ty: FlagType::Uint,
    readonly: false,
    comment: "Delay before memory reducer start",
    default: || FlagValue::Uint(30000u32),
  },
  FlagDef {
    name: "memory_reducer_limit_based",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use the limit-based memory reducer that updates allocation limits after a delay to trigger GC",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "external_memory_max_growing_factor",
    ty: FlagType::Float,
    readonly: false,
    comment: "This is the upper bound for growing factor imposed on external memory.",
    default: || FlagValue::Float(1.1),
  },
  FlagDef {
    name: "external_memory_max_reasonable_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "Max external memory value (in GB) checked for a reasonable size, 0 to disable the check.",
    default: || FlagValue::Int(32i32),
  },
  FlagDef {
    name: "sqrt_allocation_limits",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Old generation and global heap limit growth scale based on the sqrt of estimated GC cost.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sqrt_allocation_limits_factor",
    ty: FlagType::Int,
    readonly: false,
    comment: "When the sqrt heap limit is enabled, multiplier on the heap growth.",
    default: || FlagValue::Int(80i32),
  },
  FlagDef {
    name: "sqrt_allocation_limits_slow_factor",
    ty: FlagType::Int,
    readonly: false,
    comment: "When the sqrt heap limit is enabled, multiplier on the heap growth for HeapGrowingMode::kSlow.",
    default: || FlagValue::Int(80i32),
  },
  FlagDef {
    name: "sqrt_allocation_limits_minimal_factor",
    ty: FlagType::Int,
    readonly: false,
    comment: "When the sqrt heap limit is enabled, multiplier on the heap growth for HeapGrowingMode::kMinimal.",
    default: || FlagValue::Int(20i32),
  },
  FlagDef {
    name: "sqrt_allocation_limits_min_growing_factor",
    ty: FlagType::Float,
    readonly: false,
    comment: "This is the lower bound for growing factor when using sqrt heap limit.",
    default: || FlagValue::Float(1.1),
  },
  FlagDef {
    name: "sqrt_allocation_limits_max_growing_factor",
    ty: FlagType::Float,
    readonly: false,
    comment: "This is the upper bound for growing factor when using sqrt heap limit.",
    default: || FlagValue::Float(4.0),
  },
  FlagDef {
    name: "gc_speed_uses_counters",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Old gen GC speed is computed directly from gc tracer counters.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "heap_growing_percent",
    ty: FlagType::Int,
    readonly: false,
    comment: "specifies heap growing factor as (1 + heap_growing_percent/100)",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "v8_os_page_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "override OS page size (in KBytes)",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "allocation_buffer_parking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "allocation buffer parking",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "compact",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Perform compaction on full GCs based on V8's default heuristics",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "compact_code_space",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Perform code space compaction on full collections.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "compact_on_every_full_gc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Perform compaction on every full GC",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "compaction_on_regular_gcs",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Perform compaction on regular GCs",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "compact_with_stack",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Perform compaction when finalizing a full GC with stack",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "compaction_max_evacuated_bytes_mb",
    ty: FlagType::Int,
    readonly: false,
    comment: "Max evacuated bytes during compaction",
    default: || FlagValue::Int(4i32),
  },
  FlagDef {
    name: "compaction_max_evacuated_bytes_mb_for_reduce_memory",
    ty: FlagType::Int,
    readonly: false,
    comment: "Max evacuated bytes during compaction for reduce memory GCs",
    default: || FlagValue::Int(12i32),
  },
  FlagDef {
    name: "compaction_max_evacuated_bytes_mb_for_optimize_memory",
    ty: FlagType::Int,
    readonly: false,
    comment: "Max evacuated bytes during compaction for GCs when ShouldOptimizeForMemoryUsage() = true",
    default: || FlagValue::Int(6i32),
  },
  FlagDef {
    name: "compaction_target_fragmentation_percent_for_reduce_memory",
    ty: FlagType::Int,
    readonly: false,
    comment: "Target fragmentation % during compaction for reduce memory GCs",
    default: || FlagValue::Int(20i32),
  },
  FlagDef {
    name: "compaction_target_fragmentation_percent_for_optimize_memory",
    ty: FlagType::Int,
    readonly: false,
    comment: "Target fragmentation % during compaction for GCs when ShouldOptimizeForMemoryUsage() = true",
    default: || FlagValue::Int(20i32),
  },
  FlagDef {
    name: "shortcut_strings_with_stack",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Shortcut Strings during GC with stack",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "stress_compaction",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Stress GC compaction to flush out bugs with moving objects",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "resize_large_object",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Support resizing of large objects",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "stress_compaction_random",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Stress GC compaction by selecting random percent of pages as evacuation candidates. Overrides stress_compaction.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_descriptor_array_trimming",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Stress descriptor array trimming on every stackless GC.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "flush_baseline_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "flush of baseline code when it has not been executed recently",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "flush_bytecode",
    ty: FlagType::Bool,
    readonly: false,
    comment: "flush of bytecode when it has not been executed recently",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "bytecode_old_time",
    ty: FlagType::Int,
    readonly: false,
    comment: "number of seconds before we flush code",
    default: || FlagValue::Int(180i32),
  },
  FlagDef {
    name: "stress_flush_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "stress code flushing",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_flush_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace bytecode flushing (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "use_marking_progress_bar",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use a progress bar to scan large objects in increments when incremental marking is active.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "fixed_margin_for_input_handling",
    ty: FlagType::Int,
    readonly: false,
    comment: "how much we are allowed to overshoot the heap limit by during input mode, in MiB.",
    default: || FlagValue::Int(32i32),
  },
  FlagDef {
    name: "optimize_for_input_handling",
    ty: FlagType::Bool,
    readonly: false,
    comment: "whether to use input hints to optimize for responsiveness.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "stress_per_context_marking_worklist",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use per-context worklist for marking",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_incremental_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "force incremental marking for small heaps and run it more often",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "memory_pool",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Pool memory globally before returning it to the operating system",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "memory_pool_share_memory_on_teardown",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Share memory on Isolate teardown using other Isolate's task runners",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "memory_pool_release_on_malloc_failures",
    ty: FlagType::Bool,
    readonly: false,
    comment: "discard the memory pool on malloc retries",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "memory_pool_timeout",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "Release pooled pages after X seconds.",
    default: || FlagValue::SizeT(8usize),
  },
  FlagDef {
    name: "large_page_pool",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Add large pages to the page pool",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "max_large_page_pool_size",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "Maximum size of pooled large pages in MB.",
    default: || FlagValue::SizeT(32usize),
  },
  FlagDef {
    name: "managed_zone_memory",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Manage zone memory in V8 instead of using malloc().",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "managed_zone_memory_for_isolate_independent_memory",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Pool zone memory for isolate independent memory (nullptr-isolate) (e.g. Wasm).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "fuzzer_gc_analysis",
    ty: FlagType::Bool,
    readonly: false,
    comment: "prints number of allocations and enables analysis mode for gc fuzz testing, e.g. --stress-marking, --stress-scavenge",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stress_marking",
    ty: FlagType::Uint,
    readonly: false,
    comment: "force marking at random points between 0 and X (inclusive) percent of the regular marking start limit",
    default: || FlagValue::Uint(0u32),
  },
  FlagDef {
    name: "stress_scavenge",
    ty: FlagType::Uint,
    readonly: false,
    comment: "force scavenge at random points between 0 and X (inclusive) percent of the new space capacity",
    default: || FlagValue::Uint(0u32),
  },
  FlagDef {
    name: "reclaim_unmodified_wrappers",
    ty: FlagType::Bool,
    readonly: false,
    comment: "reclaim otherwise unreachable unmodified wrapper objects when possible",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "parallel_reclaim_unmodified_wrappers",
    ty: FlagType::Bool,
    readonly: false,
    comment: "reclaim wrapper objects in parallel",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "concurrent_marking_high_priority_threads",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use high priority threads for concurrent Marking",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "disable_abortjs",
    ty: FlagType::Bool,
    readonly: false,
    comment: "disables AbortJS runtime function",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "randomize_all_allocations",
    ty: FlagType::Bool,
    readonly: false,
    comment: "randomize virtual memory reservations by ignoring any hints passed when allocating pages",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "manual_evacuation_candidates_selection",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Test mode only flag. It allows an unit test to select evacuation candidates pages (requires --stress_compaction).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "clear_free_memory",
    ty: FlagType::Bool,
    readonly: false,
    comment: "initialize free memory with 0",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "idle_gc_on_context_disposal",
    ty: FlagType::Bool,
    readonly: false,
    comment: "idle gc on context disposal",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_context_disposal",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace context disposal (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "cppheap_incremental_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use incremental marking for CppHeap",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "cppheap_concurrent_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use concurrent marking for CppHeap",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "memory_balancer",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use membalancer, a new heap limit balancing algorithm",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "memory_balancer_c_value",
    ty: FlagType::Float,
    readonly: false,
    comment: "c value for membalancer. A special constant to balance between memory and space tradeoff. The smaller the more memory it uses.",
    default: || FlagValue::Float(3e-10),
  },
  FlagDef {
    name: "trace_memory_balancer",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print memory balancer behavior. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "late_heap_limit_check",
    ty: FlagType::Bool,
    readonly: false,
    comment: "skips heap limit check for early GCs on allocation failure.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "debug_code",
    ty: FlagType::Bool,
    readonly: true,
    comment: "",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trap_on_abort",
    ty: FlagType::Bool,
    readonly: true,
    comment: "",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "slow_debug_code",
    ty: FlagType::Bool,
    readonly: true,
    comment: "",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "code_comments",
    ty: FlagType::Bool,
    readonly: true,
    comment: "",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_sse3",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of SSE3 instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_ssse3",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of SSSE3 instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_sse4_1",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of SSE4.1 instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_sse4_2",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of SSE4.2 instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_sahf",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of SAHF instruction if available (X64 only)",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_avx",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of AVX instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_avx2",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of AVX2 instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_avx_vnni",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of AVX-VNNI instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_avx_vnni_int8",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of AVX-VNNI-INT8 instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_fma3",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of FMA3 instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_f16c",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of F16C instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_bmi1",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of BMI1 instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_bmi2",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of BMI2 instructions if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_lzcnt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of LZCNT instruction if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_popcnt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of POPCNT instruction if available",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "arm_arch",
    ty: FlagType::Str,
    readonly: false,
    comment: "generate instructions for the selected ARM architecture if available: armv6, armv7, armv7+sudiv or armv8",
    default: || FlagValue::Str(Some(Cow::Borrowed("armv8"))),
  },
  FlagDef {
    name: "force_long_branches",
    ty: FlagType::Bool,
    readonly: false,
    comment: "force all emitted branches to be in long mode (MIPS/PPC only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "mcpu",
    ty: FlagType::Str,
    readonly: false,
    comment: "enable optimization for specific cpu",
    default: || FlagValue::Str(Some(Cow::Borrowed("auto"))),
  },
  FlagDef {
    name: "partial_constant_pool",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable use of partial constant pools (x64 only)",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "sim_arm64_optional_features",
    ty: FlagType::Str,
    readonly: false,
    comment: "enable optional features on the simulator for testing: none or all",
    default: || FlagValue::Str(Some(Cow::Borrowed("none"))),
  },
  FlagDef {
    name: "sim_arm64_tbi",
    ty: FlagType::Bool,
    readonly: false,
    comment: "simulate arm64 Top Byte Ignore (TBI) for data accesses",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "intel_jcc_erratum_mitigation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable mitigation for Intel JCC erratum on affected CPUs",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "enable_source_at_csa_bind",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Include source information in the binary at CSA bind locations.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_armv7",
    ty: FlagType::MaybeBool,
    readonly: false,
    comment: "deprecated (use --arm_arch instead)",
    default: || FlagValue::MaybeBool(None),
  },
  FlagDef {
    name: "enable_vfp3",
    ty: FlagType::MaybeBool,
    readonly: false,
    comment: "deprecated (use --arm_arch instead)",
    default: || FlagValue::MaybeBool(None),
  },
  FlagDef {
    name: "enable_32dregs",
    ty: FlagType::MaybeBool,
    readonly: false,
    comment: "deprecated (use --arm_arch instead)",
    default: || FlagValue::MaybeBool(None),
  },
  FlagDef {
    name: "enable_neon",
    ty: FlagType::MaybeBool,
    readonly: false,
    comment: "deprecated (use --arm_arch instead)",
    default: || FlagValue::MaybeBool(None),
  },
  FlagDef {
    name: "enable_sudiv",
    ty: FlagType::MaybeBool,
    readonly: false,
    comment: "deprecated (use --arm_arch instead)",
    default: || FlagValue::MaybeBool(None),
  },
  FlagDef {
    name: "enable_armv8",
    ty: FlagType::MaybeBool,
    readonly: false,
    comment: "deprecated (use --arm_arch instead)",
    default: || FlagValue::MaybeBool(None),
  },
  FlagDef {
    name: "enable_regexp_unaligned_accesses",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable unaligned accesses for the regexp engine",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "script_streaming",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable parsing on background",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "stress_background_compile",
    ty: FlagType::Bool,
    readonly: false,
    comment: "stress test parsing on background",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "concurrent_cache_deserialization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable deserializing code caches on background",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "merge_background_deserialized_script_with_compilation_cache",
    ty: FlagType::Bool,
    readonly: false,
    comment: "After deserializing code cache data on a background thread, merge it into an existing Script if one is found in the Isolate compilation cache",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "verify_code_merge",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Verify scope infos after merge",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "expose_gc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "expose gc extension",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "expose_gc_as",
    ty: FlagType::Str,
    readonly: false,
    comment: "expose gc extension under the specified name",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "expose_externalize_string",
    ty: FlagType::Bool,
    readonly: false,
    comment: "expose externalize string extension",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "expose_statistics",
    ty: FlagType::Bool,
    readonly: false,
    comment: "expose statistics extension",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "expose_trigger_failure",
    ty: FlagType::Bool,
    readonly: false,
    comment: "expose trigger-failure extension",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stack_trace_limit",
    ty: FlagType::Int,
    readonly: false,
    comment: "number of stack frames to capture",
    default: || FlagValue::Int(10i32),
  },
  FlagDef {
    name: "builtins_in_stack_traces",
    ty: FlagType::Bool,
    readonly: false,
    comment: "show built-in functions in stack traces",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "experimental_stack_trace_frames",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable experimental frames (API/Builtins) and stack trace layout",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "disallow_code_generation_from_strings",
    ty: FlagType::Bool,
    readonly: false,
    comment: "disallow eval and friends",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "expose_async_hooks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "expose async_hooks object",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "expose_cputracemark_as",
    ty: FlagType::Str,
    readonly: false,
    comment: "expose cputracemark extension under the specified name",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "experimental_report_exceptions_from_callbacks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Notify Api callback about exceptions thrown in Api callbacks",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "allow_unsafe_function_constructor",
    ty: FlagType::Bool,
    readonly: false,
    comment: "allow invoking the function constructor without security checks",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "force_slow_path",
    ty: FlagType::Bool,
    readonly: false,
    comment: "always take the slow path for builtins",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "test_small_max_function_context_stub_size",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable testing the function context size overflow path by making the maximum size smaller",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "fast_api_indexof",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable using indexOf Api callbacks",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "fast_api_iterable_to_list",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable fast path for IterableToList for indexed interceptors",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "inline_new",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use fast inline allocation",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "switch_table_spread_threshold",
    ty: FlagType::Int,
    readonly: false,
    comment: "allow the jump table used for switch statements to span a range of integers roughly equal to this number times the number of clauses in the switch",
    default: || FlagValue::Int(3i32),
  },
  FlagDef {
    name: "switch_table_min_cases",
    ty: FlagType::Uint,
    readonly: false,
    comment: "the number of Smi integer cases present in the switch statement before using the jump table optimization",
    default: || FlagValue::Uint(6u32),
  },
  FlagDef {
    name: "stress_lazy_compilation",
    ty: FlagType::Uint,
    readonly: false,
    comment: "stress lazy compilation by simulating stack overflow during unoptimized bytecode generation with 1/n-th probability, do nothing on 0",
    default: || FlagValue::Uint(0u32),
  },
  FlagDef {
    name: "trace",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace javascript function calls",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "lazy",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use lazy compilation",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "lazy_eval",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use lazy compilation during eval",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "lazy_streaming",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use lazy compilation during streaming compilation",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "max_lazy",
    ty: FlagType::Bool,
    readonly: false,
    comment: "ignore eager compilation hints",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "compile_hints_magic",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable magic compile hints comments",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_opt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace optimized compilation (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_opt_status",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace the optimization status of functions during tiering events (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_opt_verbose",
    ty: FlagType::Bool,
    readonly: false,
    comment: "extra verbose optimized compilation tracing (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_opt_stats",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace optimized compilation statistics (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_deopt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace deoptimization (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_deopt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "log deoptimization (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_deopt_verbose",
    ty: FlagType::Bool,
    readonly: false,
    comment: "extra verbose deoptimization tracing (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_file_names",
    ty: FlagType::Bool,
    readonly: false,
    comment: "include file names in trace-opt/trace-deopt output (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "always_osr",
    ty: FlagType::Bool,
    readonly: false,
    comment: "always try to OSR functions",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_serializer",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print code serializer trace (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "compilation_cache",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable compilation cache",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "cache_prototype_transitions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "cache prototype transitions",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "lazy_compile_dispatcher",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable compiler dispatcher (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "lazy_compile_dispatcher_max_threads",
    ty: FlagType::Uint,
    readonly: false,
    comment: "max threads for compiler dispatcher (0 for unbounded)",
    default: || FlagValue::Uint(0u32),
  },
  FlagDef {
    name: "trace_compiler_dispatcher",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace compiler dispatcher activity (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "parallel_compile_tasks_for_eager_toplevel",
    ty: FlagType::Bool,
    readonly: false,
    comment: "spawn parallel compile tasks for eagerly compiled, top-level functions (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "parallel_compile_tasks_for_lazy",
    ty: FlagType::Bool,
    readonly: false,
    comment: "spawn parallel compile tasks for all lazily compiled functions (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "cpu_profiler_sampling_interval",
    ty: FlagType::Uint,
    readonly: false,
    comment: "CPU profiler sampling interval in microseconds",
    default: || FlagValue::Uint(1000u32),
  },
  FlagDef {
    name: "simulator_debugger",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Start a debugger when a simulator hits a breakpoint",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_side_effect_free_debug_evaluate",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print debug messages for side-effect-free debug-evaluate for testing",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "hard_abort",
    ty: FlagType::Bool,
    readonly: false,
    comment: "abort by crashing",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "log_colour",
    ty: FlagType::Bool,
    readonly: false,
    comment: "When logging, try to use coloured output.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "expose_inspector_scripts",
    ty: FlagType::Bool,
    readonly: false,
    comment: "expose injected-script-source.js for debugging",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "stack_size",
    ty: FlagType::Int,
    readonly: false,
    comment: "default size of stack region v8 is allowed to use (in kBytes)",
    default: || FlagValue::Int(984i32),
  },
  FlagDef {
    name: "max_stack_trace_source_length",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum length of function source code printed in a stack trace.",
    default: || FlagValue::Int(300i32),
  },
  FlagDef {
    name: "clear_exceptions_on_js_entry",
    ty: FlagType::Bool,
    readonly: false,
    comment: "clear exceptions when entering JavaScript",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "use_original_message_for_stack_trace",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use the message with which the Error constructor was called rather than the value of the \"message\" property for Error.stack",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "histogram_interval",
    ty: FlagType::Int,
    readonly: false,
    comment: "time interval in ms for aggregating memory histograms",
    default: || FlagValue::Int(600000i32),
  },
  FlagDef {
    name: "heap_profiler_trace_objects",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Dump heap object allocations/movements/size_updates",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "heap_profiler_use_embedder_graph",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use the new EmbedderGraph API to get embedder nodes",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "heap_snapshot_on_oom",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Write a heap snapshot to disk on last-resort GCs",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "heap_snapshot_path",
    ty: FlagType::Str,
    readonly: false,
    comment: "Directory to write heap snapshots to. (If not set, the snapshot is written to the current working directory.)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "heap_snapshot_on_gc",
    ty: FlagType::Int,
    readonly: false,
    comment: "Write a heap snapshot to disk on a certain GC invocation. 0 results in taking a snapshot on every invocation.",
    default: || FlagValue::Int(-1i32),
  },
  FlagDef {
    name: "heap_snapshot_string_limit",
    ty: FlagType::Uint,
    readonly: false,
    comment: "truncate strings to this length in the heap snapshot",
    default: || FlagValue::Uint(1024u32),
  },
  FlagDef {
    name: "profile_heap_snapshot",
    ty: FlagType::Bool,
    readonly: false,
    comment: "dump time spent on heap snapshot",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sampling_heap_profiler_suppress_randomness",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use constant sample intervals to eliminate test flakiness",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_ic",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log inline cache state transitions for tools/ic-processor",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "fast_map_update",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable fast map update by caching the migration target",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "max_valid_polymorphic_map_count",
    ty: FlagType::Int,
    readonly: false,
    comment: "maximum number of valid maps to track in POLYMORPHIC state",
    default: || FlagValue::Int(4i32),
  },
  FlagDef {
    name: "clone_object_sidestep_transitions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "support sidestep transitions for dependency tracking object clone maps",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "fast_properties_soft_limit",
    ty: FlagType::Int,
    readonly: false,
    comment: "limits the number of properties that can be added to an object using keyed store before transitioning to dictionary mode",
    default: || FlagValue::Int(12i32),
  },
  FlagDef {
    name: "native_code_counters",
    ty: FlagType::Bool,
    readonly: false,
    comment: "generate extra code for manipulating stats counters",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sparkplug_plus",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable dynamic patching on JS baseline code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sparkplug_inline_smi",
    ty: FlagType::Bool,
    readonly: true,
    comment: "inline the Smi fast path of embedded feedback operations into baseline code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "super_ic",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use an IC for super property loads",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "mega_dom_ic",
    ty: FlagType::Bool,
    readonly: false,
    comment: "use MegaDOM IC state for API objects (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "homomorphic_ic_count",
    ty: FlagType::Uint,
    readonly: false,
    comment: "local cache size in homomorphic ICs",
    default: || FlagValue::Uint(8u32),
  },
  FlagDef {
    name: "trace_prototype_users",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Trace updates to prototype user tracking (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_for_in_enumerate",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Trace for-in enumerate slow-paths (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_maps",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log map creation",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_maps_details",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Also log map details",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "allow_natives_syntax",
    ty: FlagType::Bool,
    readonly: false,
    comment: "allow natives syntax",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "allow_natives_for_differential_fuzzing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "allow only natives explicitly allowlisted for differential fuzzers",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "parse_only",
    ty: FlagType::Bool,
    readonly: false,
    comment: "only parse the sources",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sim_abort_on_bad_auth",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Stop execution when a pointer authentication fails in the ARM64 simulator.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "async_stack_traces",
    ty: FlagType::Bool,
    readonly: false,
    comment: "include async stack traces in Error.stack",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "stack_trace_on_illegal",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print stack trace when an illegal exception is thrown",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "abort_on_uncaught_exception",
    ty: FlagType::Bool,
    readonly: false,
    comment: "abort program (dump core) when an uncaught exception is thrown",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "correctness_fuzzer_suppressions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Suppress certain unspecified behaviors to ease correctness fuzzing: Abort program when the stack overflows or a string exceeds maximum length (as opposed to throwing RangeError). Use a fixed suppression string for error messages. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "correctness_fuzzer_cross_arch_suppressions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Minimize cross-architecture differences for correctness fuzzing. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "rehash_snapshot",
    ty: FlagType::Bool,
    readonly: false,
    comment: "rehash strings from the snapshot to override the baked-in seed",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "hash_seed",
    ty: FlagType::Uint64,
    readonly: false,
    comment: "Fixed seed to use to hash property keys (0 means random)(with snapshots this option cannot override the baked-in seed)",
    default: || FlagValue::Uint64(0u64),
  },
  FlagDef {
    name: "random_seed",
    ty: FlagType::Int,
    readonly: false,
    comment: "Default seed for initializing random generator (0, the default, means to use system random).",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "fuzzer_random_seed",
    ty: FlagType::Int,
    readonly: false,
    comment: "Default seed for initializing fuzzer random generator (0, the default, means to use v8's random number generator seed).",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "trace_rail",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace RAIL mode (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_all_exceptions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print exception object and stack trace on each thrown exception (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "adjust_os_scheduling_parameters",
    ty: FlagType::Bool,
    readonly: false,
    comment: "adjust OS specific scheduling params for the isolate",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "experimental_flush_embedded_blob_icache",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Used in an experiment to evaluate icache flushing on certain CPUs",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "short_builtin_calls",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Put embedded builtins code into the code range for shorter builtin calls/jumps if system has >=4GB memory",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_code_range_allocation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Trace code range allocation process. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "better_code_range_allocation",
    ty: FlagType::Bool,
    readonly: false,
    comment: "This mode tries harder to allocate code range near .text section. Works only for configurations with external code space and shared pointer compression cage.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "abort_on_far_code_range",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Abort if code range is allocated further away than 4GB from the.text section",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "runtime_call_stats",
    ty: FlagType::Bool,
    readonly: false,
    comment: "report runtime call counts and times",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "rcs",
    ty: FlagType::Bool,
    readonly: false,
    comment: "report runtime call counts and times",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "rcs_cpu_time",
    ty: FlagType::Bool,
    readonly: false,
    comment: "report runtime times in cpu time (the default is wall time)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "verify_snapshot_checksum",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Verify snapshot checksums when deserializing snapshots. Enable checksum creation and verification for code caches. Enabled by default in debug builds and once per process for Android.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "code_cache_source_hash_sha256",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Hash code cache source string with SHA256 instead of string length",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "profile_deserialization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Print the time it takes to deserialize the snapshot. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_deserialization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Trace the snapshot deserialization. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "serialization_statistics",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Collect statistics on serialized objects. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "regexp_optimization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "generate optimized regexp code",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "regexp_atomic_loop",
    ty: FlagType::Bool,
    readonly: false,
    comment: "optimize atomic regexp loops",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "regexp_unroll",
    ty: FlagType::Bool,
    readonly: false,
    comment: "unroll small {} repeats when optimizing",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "regexp_masked_dispatch",
    ty: FlagType::Bool,
    readonly: false,
    comment: "dispatch disjoint regexp alternations on the masked quick-check value",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "regexp_quick_check",
    ty: FlagType::Bool,
    readonly: false,
    comment: "generate quickcheck code when optimizing",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "regexp_interpret_all",
    ty: FlagType::Bool,
    readonly: false,
    comment: "interpret all regexp code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "regexp_tier_up",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable regexp interpreter and tier up to the compiler after the number of executions set by the tier up ticks flag",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "regexp_tier_up_ticks",
    ty: FlagType::Int,
    readonly: false,
    comment: "set the number of executions for the regexp interpreter before tiering-up to the compiler",
    default: || FlagValue::Int(0i32),
  },
  FlagDef {
    name: "regexp_jit_all",
    ty: FlagType::Bool,
    readonly: false,
    comment: "compile all regexp patterns directly to native code, skipping the interpreter (equivalent to --regexp-tier-up --regexp-tier-up-ticks=0, which is also the default)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "regexp_peephole_optimization",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable peephole optimization for regexp bytecode",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "regexp_results_cache",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable the regexp results cache",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "regexp_assemble_from_bytecode",
    ty: FlagType::Bool,
    readonly: false,
    comment: "assemble regexp JIT-code from bytecode",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "regexp_simd_in_rc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "emit the SIMD scans directly in the regexp compiler, instead of via the bytecode peephole pass",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_regexp_peephole_optimization",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace regexp bytecode peephole optimization (requires v8_enable_disassembler = true)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_regexp_bytecodes",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace regexp bytecode execution (requires v8_enable_disassembler = true)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_regexp_exec",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace regexp execution (requires v8_enable_regexp_diagnostics = true).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_regexp_exec_ool_subjects",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace regexp execution out-of-line subject strings (requires v8_enable_regexp_diagnostics = true).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_regexp_assembler",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace regexp macro assembler calls (requires v8_enable_regexp_diagnostics = true).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_regexp_compiler",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace regexp compilation (requires v8_enable_regexp_diagnostics = true)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_regexp_graph_building",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace regexp AST to graph conversion (requires v8_enable_regexp_diagnostics = true)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_regexp_parser",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace regexp parsing (requires v8_enable_regexp_diagnostics = true)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_regexp_graph",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace the regexp graph (requires v8_enable_regexp_diagnostics = true)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_regexp_graph",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print the regexp graph (requires v8_enable_regexp_diagnostics = true)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_regexp_tier_up",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace regexp tiering up execution (requires v8_enable_regexp_diagnostics = true)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_experimental_regexp_engine",
    ty: FlagType::Bool,
    readonly: false,
    comment: "recognize regexps with 'l' flag, run them on experimental engine (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "default_to_experimental_regexp_engine",
    ty: FlagType::Bool,
    readonly: false,
    comment: "run regexps with the experimental engine where possible",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "experimental_regexp_engine_capture_group_opt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable time optimizations for the experimental regexp engine",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "experimental_regexp_engine_capture_group_opt_max_memory_usage",
    ty: FlagType::Uint64,
    readonly: false,
    comment: "maximum memory usage in MB allowed for experimental engine",
    default: || FlagValue::Uint64(1024u64),
  },
  FlagDef {
    name: "trace_experimental_regexp_engine",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace execution of experimental regexp engine (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_experimental_regexp_engine_on_excessive_backtracks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "fall back to a breadth-first regexp engine on excessive backtracking (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "regexp_backtracks_before_fallback",
    ty: FlagType::Uint,
    readonly: false,
    comment: "number of backtracks during regexp execution before fall back to experimental engine if enable_experimental_regexp_engine_on_excessive_backtracks is set",
    default: || FlagValue::Uint(50000u32),
  },
  FlagDef {
    name: "regexp_simd",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable SIMD for regexp jit code",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_read_only_promotion",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace the read-only promotion pass (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_read_only_promotion_verbose",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace the read-only promotion pass (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "testing_bool_flag",
    ty: FlagType::Bool,
    readonly: false,
    comment: "testing_bool_flag",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "testing_maybe_bool_flag",
    ty: FlagType::MaybeBool,
    readonly: false,
    comment: "testing_maybe_bool_flag",
    default: || FlagValue::MaybeBool(None),
  },
  FlagDef {
    name: "testing_int_flag",
    ty: FlagType::Int,
    readonly: false,
    comment: "testing_int_flag",
    default: || FlagValue::Int(13i32),
  },
  FlagDef {
    name: "testing_float_flag",
    ty: FlagType::Float,
    readonly: false,
    comment: "float-flag",
    default: || FlagValue::Float(2.5),
  },
  FlagDef {
    name: "testing_string_flag",
    ty: FlagType::Str,
    readonly: false,
    comment: "string-flag",
    default: || FlagValue::Str(Some(Cow::Borrowed("Hello, world!"))),
  },
  FlagDef {
    name: "testing_prng_seed",
    ty: FlagType::Int,
    readonly: false,
    comment: "Seed used for threading test randomness",
    default: || FlagValue::Int(42i32),
  },
  FlagDef {
    name: "testing_bool_flag_A",
    ty: FlagType::Bool,
    readonly: false,
    comment: "testing_bool_flag_A",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "testing_bool_flag_B",
    ty: FlagType::Bool,
    readonly: false,
    comment: "testing_bool_flag_B",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "testing_bool_flag_C",
    ty: FlagType::Bool,
    readonly: false,
    comment: "testing_bool_flag_C",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "strict_termination_checks",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable strict terminating DCHECKs to prevent accidentally keeping on executing JS after terminating V8. (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "fuzzing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Fuzzers use this flag to signal that they are ... fuzzing. This causes intrinsics to fail silently (e.g. return undefined) on invalid usage.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "hole_fuzzing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Fuzzers use this flag to turn DCHECKs into NOPs  and CHECK failures into silent exits. This is useful if we want to find memory corruption primitives with a leaked hole, where the engine is already in a weird state",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sandbox_testing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable sandbox testing mode. This exposes the memory corruption API (if available) and enables the sandbox crash filter to terminate the process (with status zero) if a crash that does not represent a sandbox violation is detected.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "validate_generated_code",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Enable generated code verifier",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "validate_generated_code_include_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Print the disassembled code as part of the generated code validator's output in case of violations. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "validate_generated_code_non_fatal",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Treat generated code validator violations as non-fatal. Can be used to find all issues rather than exiting after the first one. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sandbox_fuzzing",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Enable sandbox fuzzing mode. This exposes the memory corruption API and enables the sandbox crash filter to terminate the process (with non-zero status) if a crash that does not represent a sandbox violation is detected.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sandbox_trap_fuzzing",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable the trap-based sandbox fuzzing mode.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "verify_bytecode_light",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Perform lightweight bytecode verification to ensure basic safety properties of generated bytecode, mostly around control-flow.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "verify_bytecode_full",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Perform full bytecode verification to ensure generated bytecode is safe to execute when the sandbox is enabled (i.e. does not lead to out-of-sandbox memory corruption). Full verification is a strict superset of the lightweight verification.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "expose_memory_corruption_api",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Exposes the memory corruption API. Set automatically by --sandbox-testing and --sandbox-fuzzing.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sandbox_prohibit_insecure_mode",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Prohibits an insecure sandbox setup.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "freeze_flags_after_init",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Disallow changes to flag values after initializing V8",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "cet_compatible",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Intel CET is only supported on x64.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "embedded_src",
    ty: FlagType::Str,
    readonly: false,
    comment: "Path for the generated embedded data file. (mksnapshot only)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "embedded_variant",
    ty: FlagType::Str,
    readonly: false,
    comment: "Label to disambiguate symbols in embedded data file. (mksnapshot only)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "startup_src",
    ty: FlagType::Str,
    readonly: false,
    comment: "Write V8 startup as C++ src. (mksnapshot only)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "startup_blob",
    ty: FlagType::Str,
    readonly: false,
    comment: "Write V8 startup blob file. (mksnapshot only)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "target_arch",
    ty: FlagType::Str,
    readonly: false,
    comment: "The mksnapshot target arch. (mksnapshot only)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "target_os",
    ty: FlagType::Str,
    readonly: false,
    comment: "The mksnapshot target os. (mksnapshot only)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "target_is_simulator",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Instruct mksnapshot that the target is meant to run in the simulator and it can generate simulator-specific instructions. (mksnapshot only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "turbo_profiling_input",
    ty: FlagType::Str,
    readonly: false,
    comment: "Path of the input file containing basic information for builtins. (mksnapshot only)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "turbo_log_builtins_count_input",
    ty: FlagType::Str,
    readonly: false,
    comment: "Path of the input file containing basic block counters for builtins for logging in turbolizer. (mksnapshot only)",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "text_is_readable",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Whether the .text section of binary can be read",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "trace_minor_ms_parallel_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "trace parallel marking for the young generation (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "minor_ms",
    ty: FlagType::Bool,
    readonly: false,
    comment: "perform young generation mark sweep GCs (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "concurrent_minor_ms_marking",
    ty: FlagType::Bool,
    readonly: false,
    comment: "perform young generation marking concurrently",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "black_allocated_pages",
    ty: FlagType::Bool,
    readonly: true,
    comment: "allocate non-young objects during incremental marking on separate pages",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "sticky_mark_bits",
    ty: FlagType::Bool,
    readonly: true,
    comment: "use sticky mark bits for separation of generations",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "minor_ms_min_new_space_capacity_for_concurrent_marking_mb",
    ty: FlagType::Uint,
    readonly: false,
    comment: "min new space capacity in MBs for using young generation concurrent marking.",
    default: || FlagValue::Uint(8u32),
  },
  FlagDef {
    name: "minor_ms_concurrent_marking_trigger",
    ty: FlagType::Uint,
    readonly: false,
    comment: "minor ms concurrent marking trigger in percent of the current new space capacity",
    default: || FlagValue::Uint(90u32),
  },
  FlagDef {
    name: "minor_ms_min_lab_size_kb",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "override for the minimum lab size in KB to be used for new space allocations with minor ms. ",
    default: || FlagValue::SizeT(0usize),
  },
  FlagDef {
    name: "handle_weak_ref_weakly_in_minor_gc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enables weak handling of WeakRef and FinalizationRegistry in minor GCs.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "help",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Print usage message, including flags, on console",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_flag_values",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Print all flag values of V8",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_feature_flags_json",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Print JS and Wasm feature flags grouped by in-progress, staged, and shipped",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "slow_histograms",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable slow histograms with more overhead.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "use_external_strings",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use external strings for source code",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "map_counters",
    ty: FlagType::Str,
    readonly: false,
    comment: "Map counters to a file",
    default: || FlagValue::Str(Some(Cow::Borrowed(""))),
  },
  FlagDef {
    name: "mock_arraybuffer_allocator",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use a mock ArrayBuffer allocator for testing. (test-only / unsafe)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "mock_arraybuffer_allocator_limit",
    ty: FlagType::SizeT,
    readonly: false,
    comment: "Memory limit for mock ArrayBuffer allocator used to simulate OOM for testing.",
    default: || FlagValue::SizeT(0usize),
  },
  FlagDef {
    name: "gdbjit",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable GDBJIT interface",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "gdbjit_full",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable GDBJIT interface for all code objects",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "gdbjit_dump",
    ty: FlagType::Bool,
    readonly: true,
    comment: "dump elf objects with debug info to disk",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "torque_dwarf",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable DWARF line info for Torque files",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "gdbjit_dump_filter",
    ty: FlagType::Str,
    readonly: true,
    comment: "dump only objects containing this substring",
    default: || FlagValue::Str(Some(Cow::Borrowed(""))),
  },
  FlagDef {
    name: "enable_slow_asserts",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable asserts that are slow to execute",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_ast",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print source AST (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_scopes",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print scopes (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_contexts",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace contexts operations (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "code_stats",
    ty: FlagType::Bool,
    readonly: true,
    comment: "report code statistics after GC (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_handles",
    ty: FlagType::Bool,
    readonly: true,
    comment: "report handles after GC (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "check_handle_count",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Check that there are not too many handles at GC",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_global_handles",
    ty: FlagType::Bool,
    readonly: true,
    comment: "report global handles after GC (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_turbo_escape",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable tracing in escape analysis (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_module_status",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Trace status transitions of ECMAScript modules (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_normalization",
    ty: FlagType::Bool,
    readonly: true,
    comment: "prints when objects are turned into dictionaries. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_lazy",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace lazy compilation (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_isolates",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace isolate state changes (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "regexp_possessive_quantifier",
    ty: FlagType::Bool,
    readonly: true,
    comment: "enable possessive quantifier syntax for testing",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_break_location",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print source location on debug break (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "logfile",
    ty: FlagType::Str,
    readonly: false,
    comment: "Specify the name of the log file, use '-' for console, '+' for a temporary file.",
    default: || FlagValue::Str(Some(Cow::Borrowed("v8.log"))),
  },
  FlagDef {
    name: "logfile_per_isolate",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Separate log files for each isolate.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "log",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Minimal logging (no API, code, GC, suspect, or handles samples).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_all",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log all events to the log file.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_internal_timer_events",
    ty: FlagType::Bool,
    readonly: false,
    comment: "See --log-timer-events",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_timer_events",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log timer events (incl. console.time* and Date.now).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_source_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log source code.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_source_position",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log detailed source information.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_code",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log code events to the log file without profiling.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_feedback_vector",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log FeedbackVectors on first creation",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_code_disassemble",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log all disassembled code to the log file.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "log_function_events",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log function events (parse, compile, execute) separately.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "detailed_line_info",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Always generate detailed line information for CPU profiling.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "prof_include_idle",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Include idle samples in the profile",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "perfetto_code_logger",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Enable the Perfetto code data source (requires the v8_use_perfetto gn arg)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "prof_sampling_interval",
    ty: FlagType::Int,
    readonly: false,
    comment: "Interval for --prof samples (in microseconds).",
    default: || FlagValue::Int(1000i32),
  },
  FlagDef {
    name: "prof_cpp",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Like --prof, but ignore generated code.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "prof_browser_mode",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Used with --prof, turns on browser-compatible mode for profiling.",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "prof",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Log statistical profiling information (implies --log-code).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "ll_prof",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable low-level linux profiler. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "perf_basic_prof",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable basic support for perf profiler. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "perf_basic_prof_path",
    ty: FlagType::Str,
    readonly: false,
    comment: "directory to write perf-<pid>.map symbol file to",
    default: || FlagValue::Str(Some(Cow::Borrowed("/tmp"))),
  },
  FlagDef {
    name: "perf_basic_prof_only_functions",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Only report function code ranges to perf (i.e. no stubs). (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "perf_prof",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable experimental annotate support for perf profiler. (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "perf_prof_path",
    ty: FlagType::Str,
    readonly: false,
    comment: "directory to write jit-<pid>.dump symbol file to",
    default: || FlagValue::Str(Some(Cow::Borrowed("."))),
  },
  FlagDef {
    name: "perf_prof_annotate_wasm",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Used with --perf-prof, load wasm source map and provide annotate support (experimental). (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "perf_prof_delete_file",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Remove the perf file right after creating it (for testing only). (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "perf_prof_unwinding_info",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable unwinding info for perf linux profiler (experimental). (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "gc_fake_mmap",
    ty: FlagType::Str,
    readonly: false,
    comment: "Specify the name of the file for fake gc mmap used in ll_prof",
    default: || FlagValue::Str(Some(Cow::Borrowed("/tmp/__v8_gc__"))),
  },
  FlagDef {
    name: "redirect_code_traces",
    ty: FlagType::Bool,
    readonly: false,
    comment: "output deopt information and disassembly into file code-<pid>-<isolate id>.asm",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "redirect_code_traces_to",
    ty: FlagType::Str,
    readonly: false,
    comment: "output deopt information and disassembly into the given file",
    default: || FlagValue::Str(None),
  },
  FlagDef {
    name: "print_opt_source",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print source code of optimized and inlined functions (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "vtune_prof_annotate_wasm",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Used when v8_enable_vtunejit is enabled, load wasm source map and provide annotate support (experimental).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "win64_unwinding_info",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable unwinding info for Windows/x64",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "interpreted_frames_native_stack",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Show interpreted frames on the native stack (useful for external profilers).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_etw_stack_walking",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Enable etw stack walking for windows",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "etw_trace_debug",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Enable etw debug logging (only on debug builds)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "enable_etw_by_custom_filter_only",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Enable etw stack walking for windows, but only if explicitly specified in a WPR profile",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_builtin_size",
    ty: FlagType::Bool,
    readonly: false,
    comment: "print code size for builtins",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_elements_transitions",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace elements transitions (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "trace_creation_allocation_sites",
    ty: FlagType::Bool,
    readonly: true,
    comment: "trace the creation of allocation sites (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_code",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print generated code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_opt_code",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print optimized code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_opt_code_filter",
    ty: FlagType::Str,
    readonly: true,
    comment: "filter for printing optimized code",
    default: || FlagValue::Str(Some(Cow::Borrowed("*"))),
  },
  FlagDef {
    name: "print_code_verbose",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print more information for code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_builtin_code",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print generated code for builtins (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_builtin_code_filter",
    ty: FlagType::Str,
    readonly: true,
    comment: "filter for printing builtin code",
    default: || FlagValue::Str(Some(Cow::Borrowed("*"))),
  },
  FlagDef {
    name: "print_regexp_code",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print generated regexp code (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "print_regexp_bytecode",
    ty: FlagType::Bool,
    readonly: true,
    comment: "print generated regexp bytecode (developer-only)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "predictable",
    ty: FlagType::Bool,
    readonly: false,
    comment: "enable predictable mode",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "predictable_gc_schedule",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Predictable garbage collection schedule. Fixes heap growing, idle, and memory reducing behavior.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "single_threaded",
    ty: FlagType::Bool,
    readonly: false,
    comment: "disable the use of background tasks",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "single_threaded_gc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "disable the use of background gc tasks",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "single_threaded_gc_in_background",
    ty: FlagType::Bool,
    readonly: false,
    comment: "disable the use of background gc tasks when in background",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "parallel_pause_for_gc_in_background",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use parallel threads in the atomic pause for background GCs",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "incremental_marking_for_gc_in_background",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use parallel threads in the atomic pause for background GCs",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "shared_heap",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Enables a shared heap between isolates.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "empty_shared_heap",
    ty: FlagType::Bool,
    readonly: true,
    comment: "Enables a shared heap, but not shared allocations",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "proto_assign_seq_opt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Enable optimizing a sequence of `Class_X.prototype.[key] = ...`by replacing it by a runtime code somewhat equivalent to `Object.assign(Class_X.prototype, [boilerplate_obj])`",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "proto_assign_seq_opt_count",
    ty: FlagType::Uint,
    readonly: false,
    comment: "The minimum number of consecutive property assignments on the prototype object for replacing it with a single byte code",
    default: || FlagValue::Uint(2u32),
  },
  FlagDef {
    name: "proto_assign_seq_lazy_func_opt",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Functions bulk assigned to prototype via SetPrototypeProperties will remain in SharedFunctionInfo (Rather than JSFunction) State until first access (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "private_field_bytecodes",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Use specialized bytecodes for Get/Set Private Fields (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "track_array_buffer_views",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Track TypedArrays and DataViews attached to array buffers and update them in-place when the buffer detaches",
    default: || FlagValue::Bool(true),
  },
  FlagDef {
    name: "superspreading",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Allow arbitrary length spread arguments on supported builtins (experimental)",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "fast_proxy_ic",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Support proxy traps in ICs",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "is_standalone_d8_shell",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Tells V8 it's running as part of the d8. This flag should not be set in other cases.",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "run_as_security_poc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Run programs as security proof of concept (POC).",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "run_as_sandbox_security_poc",
    ty: FlagType::Bool,
    readonly: false,
    comment: "Run programs as security proof of concept (POC) for the V8 sandbox",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "verify_predictable",
    ty: FlagType::Bool,
    readonly: true,
    comment: "this mode is used for checking that V8 behaves predictably",
    default: || FlagValue::Bool(false),
  },
  FlagDef {
    name: "dump_allocations_digest_at_alloc",
    ty: FlagType::Int,
    readonly: true,
    comment: "dump allocations digest each n-th allocation",
    default: || FlagValue::Int(-1i32),
  },
];

pub static ALL_ENTRIES: &[EntryMeta] = &[
  EntryMeta {
    name: "experimental",
    comment: Some(
      "Indicates that V8 is running with experimental features enabled. This flag is typically not set explicitly but instead enabled as an implication of other flags which enable experimental features.",
    ),
    flag_index: 0,
    canonical_index: 0,
  },
  EntryMeta {
    name: "array_destructure_bytecode",
    comment: Some("enable experimental ArrayDestructure bytecode (experimental)"),
    flag_index: 1,
    canonical_index: 1,
  },
  EntryMeta {
    name: "test_only_unsafe",
    comment: Some(
      "Indicates that V8 is running in an unsupported and unsafe configuration, e.g. used for internal testing. This flag is typically not set explicitly but instead enabled as an implication of other flags.",
    ),
    flag_index: 2,
    canonical_index: 2,
  },
  EntryMeta {
    name: "disallow_unsafe_flags",
    comment: Some(
      "Prevents the use of flags that are considered unsafe. Setting this flag will make V8 treat unsafe flags as flag contradictions and either crash, exit gracefully with a message, or reset them (depending on the current FlagProcessingMode).",
    ),
    flag_index: 3,
    canonical_index: 3,
  },
  EntryMeta {
    name: "developer_only_features",
    comment: Some("Indicates that V8 is running with developer-only features enabled."),
    flag_index: 4,
    canonical_index: 4,
  },
  EntryMeta {
    name: "disallow_developer_only_features",
    comment: Some("Disallow developer-only-features."),
    flag_index: 5,
    canonical_index: 5,
  },
  EntryMeta {
    name: "flag_processing_mode",
    comment: Some(
      "Control behavior for flag errors. Possible values: abort-on-error, exit-on-error, ignore-contradictions.",
    ),
    flag_index: 6,
    canonical_index: 6,
  },
  EntryMeta {
    name: "use_strict",
    comment: Some("enforce strict mode"),
    flag_index: 7,
    canonical_index: 7,
  },
  EntryMeta {
    name: "trace_temporal",
    comment: Some("trace temporal code (developer-only)"),
    flag_index: 8,
    canonical_index: 8,
  },
  EntryMeta {
    name: "js_postmessage_share_immutable_arraybuffer",
    comment: Some("share immutable array buffers in postMessage instead of copying"),
    flag_index: 9,
    canonical_index: 9,
  },
  EntryMeta {
    name: "builtin_subclassing",
    comment: Some("subclassing support in built-in methods"),
    flag_index: 10,
    canonical_index: 10,
  },
  EntryMeta {
    name: "enable_sharedarraybuffer_per_context",
    comment: Some("enable the SharedArrayBuffer constructor per context"),
    flag_index: 11,
    canonical_index: 11,
  },
  EntryMeta {
    name: "js_nonextensible_applies_to_private",
    comment: Some("non-extensibility includes private fields"),
    flag_index: 12,
    canonical_index: 12,
  },
  EntryMeta {
    name: "for_of_optimization",
    comment: Some("Experimental flag for improving ForOf loops on ArrayIterator (experimental)"),
    flag_index: 13,
    canonical_index: 13,
  },
  EntryMeta {
    name: "enable_queue_microtask",
    comment: Some(
      "ship a native queueMicrotask implementation on the global object (experimental)",
    ),
    flag_index: 14,
    canonical_index: 14,
  },
  EntryMeta {
    name: "icu_timezone_data",
    comment: Some("get information about timezones from ICU"),
    flag_index: 15,
    canonical_index: 15,
  },
  EntryMeta {
    name: "icu_datetime_compat_lang",
    comment: Some("limits ICU date time compat changes to the given language"),
    flag_index: 16,
    canonical_index: 16,
  },
  EntryMeta {
    name: "intl_date_time_pattern_generator_cache_eviction",
    comment: Some("enable 9-entry cache eviction in DateTimePatternGeneratorCache"),
    flag_index: 17,
    canonical_index: 17,
  },
  EntryMeta {
    name: "use_icu4x_normalizer",
    comment: Some("use ICU4X normalizer"),
    flag_index: 18,
    canonical_index: 18,
  },
  EntryMeta {
    name: "stress_snapshot",
    comment: Some("disables sharing of the read-only heap for testing"),
    flag_index: 19,
    canonical_index: 19,
  },
  EntryMeta {
    name: "lite_mode",
    comment: Some("enables trade-off of performance for memory savings"),
    flag_index: 20,
    canonical_index: 20,
  },
  EntryMeta {
    name: "enable_allocation_folding",
    comment: Some("Use allocation folding globally"),
    flag_index: 21,
    canonical_index: 21,
  },
  EntryMeta {
    name: "disable_write_barriers",
    comment: Some(
      "disable write barriers when GC is non-incremental and heap contains single generation.",
    ),
    flag_index: 22,
    canonical_index: 22,
  },
  EntryMeta {
    name: "enable_unconditional_write_barriers",
    comment: Some("always use full write barriers"),
    flag_index: 23,
    canonical_index: 23,
  },
  EntryMeta {
    name: "single_generation",
    comment: Some("allocate all objects from young generation to old generation"),
    flag_index: 24,
    canonical_index: 24,
  },
  EntryMeta {
    name: "conservative_stack_scanning",
    comment: Some("use conservative stack scanning (experimental)"),
    flag_index: 25,
    canonical_index: 25,
  },
  EntryMeta {
    name: "direct_handle",
    comment: Some("use direct handles with conservative stack scanning"),
    flag_index: 26,
    canonical_index: 26,
  },
  EntryMeta {
    name: "scavenger_conservative_object_pinning",
    comment: Some(
      "Objects reachable from the native stack during scavenge will be pinned and won't move.",
    ),
    flag_index: 27,
    canonical_index: 27,
  },
  EntryMeta {
    name: "stress_scavenger_conservative_object_pinning",
    comment: Some(
      "Treat some precise references as conservative references to stress test object pinning in Scavenger",
    ),
    flag_index: 28,
    canonical_index: 28,
  },
  EntryMeta {
    name: "stress_scavenger_conservative_object_pinning_random",
    comment: Some(
      "Enables random stressing of object pinning in Scavenger, such that each GC would randomly pick a subset of the precise references to treat conservatively",
    ),
    flag_index: 29,
    canonical_index: 29,
  },
  EntryMeta {
    name: "scavenger_precise_object_pinning",
    comment: Some("Objects reachable from handles during scavenge will be pinned and won't move."),
    flag_index: 30,
    canonical_index: 30,
  },
  EntryMeta {
    name: "scavenger_updates_allocation_limit",
    comment: Some("Scavenger updates allocation limits if allocation rate is low"),
    flag_index: 31,
    canonical_index: 31,
  },
  EntryMeta {
    name: "precise_object_pinning",
    comment: Some("Objects reachable from handles during GC will be pinned and won't move."),
    flag_index: 32,
    canonical_index: 32,
  },
  EntryMeta {
    name: "scavenger_promote_quarantined_pages",
    comment: Some("Quarantined pages in the intermediate generation will be promoted to old space"),
    flag_index: 33,
    canonical_index: 33,
  },
  EntryMeta {
    name: "scavenger_chaos_mode",
    comment: Some(
      "Scavenger will ignore age when making promotion decisions and instead choose objects to be promoted at random. This only applies to non-large objects.",
    ),
    flag_index: 34,
    canonical_index: 34,
  },
  EntryMeta {
    name: "scavenger_chaos_mode_threshold",
    comment: Some(
      "Percentage of non-large young objects that will be promoted during Scavenger in chaos mode",
    ),
    flag_index: 35,
    canonical_index: 35,
  },
  EntryMeta {
    name: "local_off_stack_check",
    comment: Some("check for off-stack allocation of v8::Local"),
    flag_index: 36,
    canonical_index: 36,
  },
  EntryMeta {
    name: "force_emit_interrupt_budget_checks",
    comment: Some(
      "force emit tier-up logic from all non-turbofan code, even if it is the top enabled tier",
    ),
    flag_index: 37,
    canonical_index: 37,
  },
  EntryMeta {
    name: "maglev",
    comment: Some("enable the maglev optimizing compiler"),
    flag_index: 38,
    canonical_index: 38,
  },
  EntryMeta {
    name: "maglev_future",
    comment: Some(
      "enable maglev features that we want to ship in the not-too-far future (experimental)",
    ),
    flag_index: 39,
    canonical_index: 39,
  },
  EntryMeta {
    name: "optimize_on_next_call_optimizes_to_maglev",
    comment: Some("make OptimizeFunctionOnNextCall optimize to maglev instead of turbofan"),
    flag_index: 40,
    canonical_index: 40,
  },
  EntryMeta {
    name: "stress_maglev",
    comment: Some("trigger maglev compilation earlier"),
    flag_index: 41,
    canonical_index: 41,
  },
  EntryMeta {
    name: "maglev_max_small_graph_size",
    comment: Some("max inlined maglev graph size that counts as a small function"),
    flag_index: 42,
    canonical_index: 42,
  },
  EntryMeta {
    name: "maglev_as_top_tier",
    comment: Some("disable Turbofan/Turbolev, make Maglev optimize more aggressively"),
    flag_index: 43,
    canonical_index: 43,
  },
  EntryMeta {
    name: "maglev_inlining",
    comment: Some("enable inlining in the maglev optimizing compiler"),
    flag_index: 44,
    canonical_index: 44,
  },
  EntryMeta {
    name: "maglev_disable_builtin_reducers",
    comment: Some(
      "disable eager builtin reducers in the maglev graph builder to flush out production bugs in post-inlining builtin reduction",
    ),
    flag_index: 45,
    canonical_index: 45,
  },
  EntryMeta {
    name: "maglev_loop_peeling",
    comment: Some("enable loop peeling in the maglev optimizing compiler"),
    flag_index: 46,
    canonical_index: 46,
  },
  EntryMeta {
    name: "maglev_optimistic_peeled_loops",
    comment: Some(
      "enable aggressive optimizations for loops (loop SPeeling) in the maglev optimizing compiler",
    ),
    flag_index: 47,
    canonical_index: 47,
  },
  EntryMeta {
    name: "maglev_loop_peeling_max_size",
    comment: Some("max loop size for loop peeling in the maglev optimizing compiler"),
    flag_index: 48,
    canonical_index: 48,
  },
  EntryMeta {
    name: "maglev_loop_peeling_max_size_cumulative",
    comment: Some("max cumulative size for loop peeling in the maglev optimizing compiler"),
    flag_index: 49,
    canonical_index: 49,
  },
  EntryMeta {
    name: "maglev_deopt_data_on_background",
    comment: Some("Generate deopt data on background thread"),
    flag_index: 50,
    canonical_index: 50,
  },
  EntryMeta {
    name: "maglev_build_code_on_background",
    comment: Some("Generate code on background thread"),
    flag_index: 51,
    canonical_index: 51,
  },
  EntryMeta {
    name: "maglev_destroy_on_background",
    comment: Some("Destroy compilation jobs on background thread"),
    flag_index: 52,
    canonical_index: 52,
  },
  EntryMeta {
    name: "maglev_inline_api_calls",
    comment: Some("Inline CallApiCallback builtin into generated code"),
    flag_index: 53,
    canonical_index: 53,
  },
  EntryMeta {
    name: "maglev_cons_string_elision",
    comment: Some("Native support for cons strings and their elision in maglev."),
    flag_index: 54,
    canonical_index: 54,
  },
  EntryMeta {
    name: "maglev_pretenure_store_values",
    comment: Some(
      "Recursively pretenure values which are stored into pretenured allocation sites.",
    ),
    flag_index: 55,
    canonical_index: 55,
  },
  EntryMeta {
    name: "maglev_poly_calls",
    comment: Some("Support (inlining) polymorphic calls"),
    flag_index: 56,
    canonical_index: 56,
  },
  EntryMeta {
    name: "maglev_truncation",
    comment: Some("Enable Maglev truncation pass"),
    flag_index: 57,
    canonical_index: 57,
  },
  EntryMeta {
    name: "trace_maglev_truncation",
    comment: Some("trace maglev truncation pass (developer-only)"),
    flag_index: 58,
    canonical_index: 58,
  },
  EntryMeta {
    name: "maglev_licm",
    comment: Some("loop invariant code motion (experimental)"),
    flag_index: 59,
    canonical_index: 59,
  },
  EntryMeta {
    name: "turbolev_truncated_int32_phis",
    comment: Some("Enable truncated to int32 representation in the phi selector"),
    flag_index: 60,
    canonical_index: 60,
  },
  EntryMeta {
    name: "maglev_range_analysis",
    comment: Some("Enable Maglev range value analysis pass"),
    flag_index: 61,
    canonical_index: 61,
  },
  EntryMeta {
    name: "trace_maglev_range_analysis",
    comment: Some("Trace Maglev range value analysis pass (developer-only)"),
    flag_index: 62,
    canonical_index: 62,
  },
  EntryMeta {
    name: "trace_maglev_kna",
    comment: Some("Trace Maglev known node aspects (developer-only)"),
    flag_index: 63,
    canonical_index: 63,
  },
  EntryMeta {
    name: "maglev_use_unreliable_maps",
    comment: Some("Use unreliable unstable maps in Maglev"),
    flag_index: 64,
    canonical_index: 64,
  },
  EntryMeta {
    name: "maglev_range_verification",
    comment: Some("Run integer range verifiction pass in Turbolev frontend pipeline"),
    flag_index: 65,
    canonical_index: 65,
  },
  EntryMeta {
    name: "maglev_verify_dominance",
    comment: Some("verify dominance of eager deopt frame inputs"),
    flag_index: 66,
    canonical_index: 66,
  },
  EntryMeta {
    name: "concurrent_maglev_max_threads",
    comment: Some("max number of threads that concurrent Maglev can use (0 for unbounded)"),
    flag_index: 67,
    canonical_index: 67,
  },
  EntryMeta {
    name: "concurrent_maglev_high_priority_threads",
    comment: Some("use high priority compiler threads for concurrent Maglev"),
    flag_index: 68,
    canonical_index: 68,
  },
  EntryMeta {
    name: "max_maglev_inline_depth",
    comment: Some("max depth of functions that Maglev will inline excl. small functions"),
    flag_index: 69,
    canonical_index: 69,
  },
  EntryMeta {
    name: "max_maglev_hard_inline_depth",
    comment: Some("max depth of functions that Maglev will inline incl. small functions"),
    flag_index: 70,
    canonical_index: 70,
  },
  EntryMeta {
    name: "max_maglev_inlined_bytecode_size",
    comment: Some("maximum size of bytecode for a single inlining"),
    flag_index: 71,
    canonical_index: 71,
  },
  EntryMeta {
    name: "max_maglev_inlined_bytecode_size_cumulative",
    comment: Some(
      "maximum cumulative size of bytecode considered for inlining excl. small functions",
    ),
    flag_index: 72,
    canonical_index: 72,
  },
  EntryMeta {
    name: "max_maglev_inlined_bytecode_size_small_total",
    comment: Some("maximum size of small functions' bytecode that can be inlining"),
    flag_index: 73,
    canonical_index: 73,
  },
  EntryMeta {
    name: "max_maglev_inlined_bytecode_size_small",
    comment: Some("maximum size of bytecode considered for small function inlining"),
    flag_index: 74,
    canonical_index: 74,
  },
  EntryMeta {
    name: "max_maglev_inlined_bytecode_size_small_with_heapnum_in_out",
    comment: Some(
      "maximum size of bytecode considered for small function inlining, when the function call has a HeapNumber as input or output",
    ),
    flag_index: 75,
    canonical_index: 75,
  },
  EntryMeta {
    name: "min_maglev_inlining_frequency",
    comment: Some("minimum frequency for inlining"),
    flag_index: 76,
    canonical_index: 76,
  },
  EntryMeta {
    name: "max_turbolev_inline_depth",
    comment: Some("max depth of functions that Maglev will inline excl. small functions"),
    flag_index: 77,
    canonical_index: 77,
  },
  EntryMeta {
    name: "maglev_reuse_stack_slots",
    comment: Some("reuse stack slots in the maglev optimizing compiler"),
    flag_index: 78,
    canonical_index: 78,
  },
  EntryMeta {
    name: "maglev_untagged_phis",
    comment: Some("enable phi untagging in the maglev optimizing compiler"),
    flag_index: 79,
    canonical_index: 79,
  },
  EntryMeta {
    name: "maglev_untagged_phis_bisect_limit",
    comment: Some("limit the number of Phis we untag in Maglev phi untagging (-1 for unbounded)"),
    flag_index: 80,
    canonical_index: 80,
  },
  EntryMeta {
    name: "turbolev_untagged_phis",
    comment: Some("enable phi untagging in the Turbolev optimizing compiler"),
    flag_index: 81,
    canonical_index: 81,
  },
  EntryMeta {
    name: "maglev_hoist_osr_value_phi_untagging",
    comment: Some("enable phi untagging to hoist untagging of osr values"),
    flag_index: 82,
    canonical_index: 82,
  },
  EntryMeta {
    name: "maglev_speculative_hoist_phi_untagging",
    comment: Some(
      "enable phi untagging to hoist untagging of loop phi inputs (could still cause deopt loops) (experimental)",
    ),
    flag_index: 83,
    canonical_index: 83,
  },
  EntryMeta {
    name: "maglev_cse",
    comment: Some("common subexpression elimination"),
    flag_index: 84,
    canonical_index: 84,
  },
  EntryMeta {
    name: "maglev_non_eager_inlining",
    comment: Some("enable Maglev non-eager inlining"),
    flag_index: 85,
    canonical_index: 85,
  },
  EntryMeta {
    name: "turbolev_non_eager_inlining",
    comment: Some("enable Turbolev non-eager inlining"),
    flag_index: 86,
    canonical_index: 86,
  },
  EntryMeta {
    name: "turbolev_non_eager_loop_peeling",
    comment: Some("enable Turbolev non-eager loop peeling"),
    flag_index: 87,
    canonical_index: 87,
  },
  EntryMeta {
    name: "turbolev_eager_loop_peeling_osr",
    comment: Some("use eager loop peeling in Turbolev OSR compiles"),
    flag_index: 88,
    canonical_index: 88,
  },
  EntryMeta {
    name: "turbolev_trace_loop_peeling",
    comment: Some("trace turbolev non-eager loop peeling decisions (developer-only)"),
    flag_index: 89,
    canonical_index: 89,
  },
  EntryMeta {
    name: "maglev_filter",
    comment: Some("optimization filter for the maglev compiler"),
    flag_index: 90,
    canonical_index: 90,
  },
  EntryMeta {
    name: "maglev_print_filter",
    comment: Some("filter for maglev's tracing/printing options"),
    flag_index: 91,
    canonical_index: 91,
  },
  EntryMeta {
    name: "maglev_assert",
    comment: Some("insert extra assertion in maglev code"),
    flag_index: 92,
    canonical_index: 92,
  },
  EntryMeta {
    name: "maglev_assert_stack_size",
    comment: Some("insert stack size checks before every IR node"),
    flag_index: 93,
    canonical_index: 93,
  },
  EntryMeta {
    name: "maglev_assert_types",
    comment: Some("Enable additional type assertions in generated code (experimental)"),
    flag_index: 94,
    canonical_index: 94,
  },
  EntryMeta {
    name: "maglev_break_on_entry",
    comment: Some("insert an int3 on maglev entries"),
    flag_index: 95,
    canonical_index: 95,
  },
  EntryMeta {
    name: "maglev_print_bytecode",
    comment: Some("print bytecode for maglev compiled code"),
    flag_index: 96,
    canonical_index: 96,
  },
  EntryMeta {
    name: "maglev_print_feedback",
    comment: Some("print feedback vector for maglev compiled code"),
    flag_index: 97,
    canonical_index: 97,
  },
  EntryMeta {
    name: "maglev_print_inlined",
    comment: Some("print bytecode / feedback vectors also for inlined code"),
    flag_index: 98,
    canonical_index: 98,
  },
  EntryMeta {
    name: "maglev_print_provenance",
    comment: Some("print Maglev node provenance, ie, file name, bytecode and offset"),
    flag_index: 99,
    canonical_index: 99,
  },
  EntryMeta {
    name: "print_maglev_code",
    comment: Some("print maglev code (developer-only)"),
    flag_index: 100,
    canonical_index: 100,
  },
  EntryMeta {
    name: "trace_with_compilation_id",
    comment: Some("prefix maglev/turbolev trace line with a compilation id"),
    flag_index: 101,
    canonical_index: 101,
  },
  EntryMeta {
    name: "trace_maglev_graph_building",
    comment: Some("trace maglev graph building (developer-only)"),
    flag_index: 102,
    canonical_index: 102,
  },
  EntryMeta {
    name: "trace_maglev_graph_optimizer",
    comment: Some("trace maglev graph optimizer (developer-only)"),
    flag_index: 103,
    canonical_index: 103,
  },
  EntryMeta {
    name: "trace_maglev_loop_speeling",
    comment: Some("trace maglev loop SPeeling (developer-only)"),
    flag_index: 104,
    canonical_index: 104,
  },
  EntryMeta {
    name: "trace_maglev_inlining",
    comment: Some("trace maglev inlining (developer-only)"),
    flag_index: 105,
    canonical_index: 105,
  },
  EntryMeta {
    name: "trace_maglev_kna_processor",
    comment: Some("trace maglev known node aspects processor (developer-only)"),
    flag_index: 106,
    canonical_index: 106,
  },
  EntryMeta {
    name: "trace_maglev_scope_info",
    comment: Some("trace maglev scope info (developer-only)"),
    flag_index: 107,
    canonical_index: 107,
  },
  EntryMeta {
    name: "trace_maglev",
    comment: Some("trace Maglev IR to JSON (developer-only)"),
    flag_index: 108,
    canonical_index: 108,
  },
  EntryMeta {
    name: "trace_maglev_file_prefix",
    comment: Some("write maglev trace files with given prefix"),
    flag_index: 109,
    canonical_index: 109,
  },
  EntryMeta {
    name: "print_maglev_deopt_verbose",
    comment: Some("print verbose deopt info"),
    flag_index: 110,
    canonical_index: 110,
  },
  EntryMeta {
    name: "print_maglev_graph",
    comment: Some("print the final maglev graph"),
    flag_index: 111,
    canonical_index: 111,
  },
  EntryMeta {
    name: "print_maglev_graphs",
    comment: Some("print maglev graph across all phases"),
    flag_index: 112,
    canonical_index: 112,
  },
  EntryMeta {
    name: "trace_maglev_phi_untagging",
    comment: Some("trace maglev phi untagging"),
    flag_index: 113,
    canonical_index: 113,
  },
  EntryMeta {
    name: "trace_maglev_regalloc",
    comment: Some("trace maglev register allocation"),
    flag_index: 114,
    canonical_index: 114,
  },
  EntryMeta {
    name: "maglev_stats",
    comment: Some("print Maglev statistics (developer-only)"),
    flag_index: 115,
    canonical_index: 115,
  },
  EntryMeta {
    name: "maglev_stats_nvp",
    comment: Some("print Maglev statistics in machine-readable format (developer-only)"),
    flag_index: 116,
    canonical_index: 116,
  },
  EntryMeta {
    name: "maglev_function_context_specialization",
    comment: Some("enable function context specialization in maglev"),
    flag_index: 117,
    canonical_index: 117,
  },
  EntryMeta {
    name: "additive_safe_int_feedback",
    comment: Some("Record AdditiveSafeInteger feedback"),
    flag_index: 118,
    canonical_index: 118,
  },
  EntryMeta {
    name: "turbolev_additive_safe_int_feedback",
    comment: Some("Enable the use of AdditiveSafeInteger feedback for Turbolev"),
    flag_index: 119,
    canonical_index: 119,
  },
  EntryMeta {
    name: "turbofan_additive_safe_int_feedback",
    comment: Some("Enable the use of AdditiveSafeInteger feedback for TurboFan"),
    flag_index: 120,
    canonical_index: 120,
  },
  EntryMeta {
    name: "enable_enumerated_keyed_access_bytecode",
    comment: Some("enable generating GetEnumeratedKeyedProperty bytecode for keyed access"),
    flag_index: 121,
    canonical_index: 121,
  },
  EntryMeta {
    name: "dict_property_const_tracking",
    comment: Some("Use const tracking on dictionary properties"),
    flag_index: 122,
    canonical_index: 122,
  },
  EntryMeta {
    name: "script_context_cells",
    comment: Some(
      "Use context cells in script contexts, ie, const tracking let and mutable numbers",
    ),
    flag_index: 123,
    canonical_index: 123,
  },
  EntryMeta {
    name: "function_context_cells",
    comment: Some("Use context cells in function contexts"),
    flag_index: 124,
    canonical_index: 124,
  },
  EntryMeta {
    name: "function_context_cells_max_size",
    comment: Some("Max number of context slots allowed in a function context with context cells"),
    flag_index: 125,
    canonical_index: 125,
  },
  EntryMeta {
    name: "empty_context_extension_dep",
    comment: Some(
      "Use compilation dependency to avoid dynamic checks for non-empty context extensions",
    ),
    flag_index: 126,
    canonical_index: 126,
  },
  EntryMeta {
    name: "json_stringify_fast_path",
    comment: Some("Enable JSON.stringify fast-path"),
    flag_index: 127,
    canonical_index: 127,
  },
  EntryMeta {
    name: "json_parse_max_heuristically_internalized_strings",
    comment: Some(
      "Maximum number of strings that are eagerly internalized during JSON.parse() based on heuristics. 0 means only strings that must be internalized e.g. property keys will be internalized.",
    ),
    flag_index: 128,
    canonical_index: 128,
  },
  EntryMeta {
    name: "trace_json_parse_internalization",
    comment: Some("Print heuristic internalization stats after each JSON.parse() (developer-only)"),
    flag_index: 129,
    canonical_index: 129,
  },
  EntryMeta {
    name: "cache_property_key_string_adds",
    comment: Some("Enable caching property keys created by concatenating strings"),
    flag_index: 130,
    canonical_index: 130,
  },
  EntryMeta {
    name: "smi_string_cache_size",
    comment: Some("Full size of SmiStringCache"),
    flag_index: 131,
    canonical_index: 131,
  },
  EntryMeta {
    name: "double_string_cache_size",
    comment: Some("Full size of DoubleStringCache"),
    flag_index: 132,
    canonical_index: 132,
  },
  EntryMeta {
    name: "trace_number_string_cache",
    comment: Some("Print NumberString caches stats on GC and at the end (developer-only)"),
    flag_index: 133,
    canonical_index: 133,
  },
  EntryMeta {
    name: "maglev_inline_date_accessors",
    comment: Some("Inline Date accessors by Maglev"),
    flag_index: 134,
    canonical_index: 134,
  },
  EntryMeta {
    name: "turbofan_inline_date_accessors",
    comment: Some("Inline Date accessors by TurboFan"),
    flag_index: 135,
    canonical_index: 135,
  },
  EntryMeta {
    name: "inline_date_accessors",
    comment: Some("Inline Date accessors"),
    flag_index: 136,
    canonical_index: 136,
  },
  EntryMeta {
    name: "extensible_ro_snapshot",
    comment: Some("Whether custom embedder snapshots may extend ReadOnlySpace"),
    flag_index: 137,
    canonical_index: 137,
  },
  EntryMeta {
    name: "max_opt",
    comment: Some(
      "Set the maximal optimisation tier: > 3 == any, 0 == ignition/interpreter, 1 == sparkplug/baseline, 2 == maglev, 3 == turbofan",
    ),
    flag_index: 138,
    canonical_index: 138,
  },
  EntryMeta {
    name: "efficiency_mode",
    comment: Some(
      "Forces efficiency mode on or off, disregarding any dynamic signals. Efficiency mode is optimized for situations with no latency requirements and uses fewer threads.",
    ),
    flag_index: 139,
    canonical_index: 139,
  },
  EntryMeta {
    name: "battery_saver_mode",
    comment: Some(
      "Forces battery saver mode on or off, disregarding any dynamic signals. Battery saver tries to conserve overall cpu cycles spent.",
    ),
    flag_index: 140,
    canonical_index: 140,
  },
  EntryMeta {
    name: "memory_saver_mode",
    comment: Some(
      "Forces memory saver mode on or off, disregarding any dynamic signals. Memory saver tries to keep memory footprint low at the expense of extra cpu cycles.",
    ),
    flag_index: 141,
    canonical_index: 141,
  },
  EntryMeta {
    name: "efficiency_mode_disable_turbofan",
    comment: Some("Defer tier-up to turbofan while in efficiency mode."),
    flag_index: 142,
    canonical_index: 142,
  },
  EntryMeta {
    name: "efficiency_mode_delay_turbofan_multiply",
    comment: Some(
      "Delay tier-up to turbofan to a certain invocation count multipier while in efficiency mode.",
    ),
    flag_index: 143,
    canonical_index: 143,
  },
  EntryMeta {
    name: "wasm_trace_native",
    comment: Some(
      "Select which native code sequence to use for wasm trace instruction: default or cpuid",
    ),
    flag_index: 144,
    canonical_index: 144,
  },
  EntryMeta {
    name: "jitless",
    comment: Some("Disable runtime allocation of executable memory."),
    flag_index: 145,
    canonical_index: 145,
  },
  EntryMeta {
    name: "disable_optimizing_compilers",
    comment: Some("Disable all optimizing compilers while leaving baseline compilers enabled"),
    flag_index: 146,
    canonical_index: 146,
  },
  EntryMeta {
    name: "memory_protection_keys",
    comment: Some("protect code memory with PKU if available"),
    flag_index: 147,
    canonical_index: 147,
  },
  EntryMeta {
    name: "force_memory_protection_keys",
    comment: Some("fail if memory protection key support is not available"),
    flag_index: 148,
    canonical_index: 148,
  },
  EntryMeta {
    name: "assert_types",
    comment: Some("generate runtime type assertions to test the typer"),
    flag_index: 149,
    canonical_index: 149,
  },
  EntryMeta {
    name: "turboshaft_assert_types",
    comment: Some(
      "generate runtime type assertions to test the turboshaft type system (experimental)",
    ),
    flag_index: 150,
    canonical_index: 150,
  },
  EntryMeta {
    name: "verify_simplified_lowering",
    comment: Some("verify graph generated by simplified lowering"),
    flag_index: 151,
    canonical_index: 151,
  },
  EntryMeta {
    name: "trace_compilation_dependencies",
    comment: Some("trace code dependencies (developer-only)"),
    flag_index: 152,
    canonical_index: 152,
  },
  EntryMeta {
    name: "unmap_holes",
    comment: Some("unmap the page containing the holes."),
    flag_index: 153,
    canonical_index: 153,
  },
  EntryMeta {
    name: "allocation_site_tracking",
    comment: Some("Enable allocation site tracking"),
    flag_index: 154,
    canonical_index: 154,
  },
  EntryMeta {
    name: "allocation_site_pretenuring",
    comment: Some("pretenure with allocation sites"),
    flag_index: 155,
    canonical_index: 155,
  },
  EntryMeta {
    name: "page_promotion",
    comment: Some("promote pages based on utilization"),
    flag_index: 156,
    canonical_index: 156,
  },
  EntryMeta {
    name: "page_promotion_threshold",
    comment: Some("min percentage of live bytes on a page to enable fast evacuation in full GCs"),
    flag_index: 157,
    canonical_index: 157,
  },
  EntryMeta {
    name: "minor_ms_page_promotion_threshold",
    comment: Some("min percentage of live bytes on a page to enable fast evacuation in MinorMS"),
    flag_index: 158,
    canonical_index: 158,
  },
  EntryMeta {
    name: "minor_ms_page_promotion_max_lab_threshold",
    comment: Some("max percentage of labs out of a page to still be considered for page promotion"),
    flag_index: 159,
    canonical_index: 159,
  },
  EntryMeta {
    name: "minor_ms_max_page_age",
    comment: Some("max age for a page after which it is force promoted to old space"),
    flag_index: 160,
    canonical_index: 160,
  },
  EntryMeta {
    name: "trace_page_promotions",
    comment: Some("trace page promotion decisions (developer-only)"),
    flag_index: 161,
    canonical_index: 161,
  },
  EntryMeta {
    name: "trace_pretenuring",
    comment: Some("trace pretenuring decisions of HAllocate instructions (developer-only)"),
    flag_index: 162,
    canonical_index: 162,
  },
  EntryMeta {
    name: "trace_pretenuring_statistics",
    comment: Some("trace allocation site pretenuring statistics (developer-only)"),
    flag_index: 163,
    canonical_index: 163,
  },
  EntryMeta {
    name: "trace_resize_large_object",
    comment: Some("trace resizing of large objects (developer-only)"),
    flag_index: 164,
    canonical_index: 164,
  },
  EntryMeta {
    name: "track_field_types",
    comment: Some("track field types"),
    flag_index: 165,
    canonical_index: 165,
  },
  EntryMeta {
    name: "trace_block_coverage",
    comment: Some("trace collected block coverage information (developer-only)"),
    flag_index: 166,
    canonical_index: 166,
  },
  EntryMeta {
    name: "trace_protector_invalidation",
    comment: Some("trace protector cell invalidations (developer-only)"),
    flag_index: 167,
    canonical_index: 167,
  },
  EntryMeta {
    name: "zero_unused_memory",
    comment: Some(
      "Zero unused memory (except for memory which was discarded) on memory reducing GCs.",
    ),
    flag_index: 168,
    canonical_index: 168,
  },
  EntryMeta {
    name: "high_end_android",
    comment: Some("Enables high-end mode unconditionally for Android."),
    flag_index: 169,
    canonical_index: 169,
  },
  EntryMeta {
    name: "high_end_android_physical_memory_threshold",
    comment: Some(
      "Enables high-end mode for devices with more than X GB of physical memory (if X greater than 0).",
    ),
    flag_index: 170,
    canonical_index: 170,
  },
  EntryMeta {
    name: "minor_ms_shortcut_strings",
    comment: Some("short cut strings during marking"),
    flag_index: 171,
    canonical_index: 171,
  },
  EntryMeta {
    name: "feedback_normalization",
    comment: Some("feed back normalization to constructors (experimental)"),
    flag_index: 172,
    canonical_index: 172,
  },
  EntryMeta {
    name: "internalize_on_the_fly",
    comment: Some("internalize string keys for generic keyed ICs on the fly"),
    flag_index: 173,
    canonical_index: 173,
  },
  EntryMeta {
    name: "unbox_double_arrays",
    comment: Some("automatically unbox arrays of doubles"),
    flag_index: 174,
    canonical_index: 174,
  },
  EntryMeta {
    name: "string_slices",
    comment: Some("use string slices"),
    flag_index: 175,
    canonical_index: 175,
  },
  EntryMeta {
    name: "invocation_count_for_feedback_allocation",
    comment: Some("invocation count required for allocating feedback vectors"),
    flag_index: 176,
    canonical_index: 176,
  },
  EntryMeta {
    name: "invocation_count_for_maglev",
    comment: Some("invocation count required for optimizing with Maglev"),
    flag_index: 177,
    canonical_index: 177,
  },
  EntryMeta {
    name: "invocation_count_for_maglev_osr",
    comment: Some("invocation count required for maglev OSR"),
    flag_index: 178,
    canonical_index: 178,
  },
  EntryMeta {
    name: "osr_from_maglev",
    comment: Some(
      "bitset mode for OSR from Maglev to Turbofan (0=off, 1=on OSR compile, 2=if loop was OSR'd, 4=always)",
    ),
    flag_index: 179,
    canonical_index: 179,
  },
  EntryMeta {
    name: "osr_from_maglev_interrupt_scale_factor",
    comment: Some("Scale interrupt budget reduction for OSR from Maglev vs. OSR to Maglev"),
    flag_index: 180,
    canonical_index: 180,
  },
  EntryMeta {
    name: "always_osr_from_maglev",
    comment: Some(
      "whether we try to OSR to Turbofan from any Maglev (alias for --osr-from-maglev=4)",
    ),
    flag_index: 181,
    canonical_index: 181,
  },
  EntryMeta {
    name: "invocation_count_for_turbofan",
    comment: Some("invocation count required for optimizing with TurboFan"),
    flag_index: 182,
    canonical_index: 182,
  },
  EntryMeta {
    name: "invocation_count_for_osr",
    comment: Some("invocation count required for OSR"),
    flag_index: 183,
    canonical_index: 183,
  },
  EntryMeta {
    name: "invocation_count_for_osr_factor_while_tiering_in_progress",
    comment: Some("invocation count factor while waiting for code to compile"),
    flag_index: 184,
    canonical_index: 184,
  },
  EntryMeta {
    name: "osr_to_tierup",
    comment: Some("number to decrease the invocation budget by when we follow OSR"),
    flag_index: 185,
    canonical_index: 185,
  },
  EntryMeta {
    name: "minimum_invocations_after_ic_update",
    comment: Some("How long to minimally wait after IC update before tier up"),
    flag_index: 186,
    canonical_index: 186,
  },
  EntryMeta {
    name: "minimum_invocations_before_optimization",
    comment: Some("Minimum number of invocations we need before non-OSR optimization"),
    flag_index: 187,
    canonical_index: 187,
  },
  EntryMeta {
    name: "jit_fuzzing",
    comment: Some("Set JIT tiering thresholds suitable for JIT fuzzing"),
    flag_index: 188,
    canonical_index: 188,
  },
  EntryMeta {
    name: "use_std_math_pow",
    comment: Some("use std::pow instead of our custom implementation"),
    flag_index: 189,
    canonical_index: 189,
  },
  EntryMeta {
    name: "use_ic",
    comment: Some("use inline caching"),
    flag_index: 190,
    canonical_index: 190,
  },
  EntryMeta {
    name: "lazy_feedback_allocation",
    comment: Some("Allocate feedback vectors lazily"),
    flag_index: 191,
    canonical_index: 191,
  },
  EntryMeta {
    name: "stress_ic",
    comment: Some("exercise interesting paths in ICs more often"),
    flag_index: 192,
    canonical_index: 192,
  },
  EntryMeta {
    name: "ignition_elide_noneffectful_bytecodes",
    comment: Some("elide bytecodes which won't have any external effect"),
    flag_index: 193,
    canonical_index: 193,
  },
  EntryMeta {
    name: "ignition_reo",
    comment: Some("use ignition register equivalence optimizer"),
    flag_index: 194,
    canonical_index: 194,
  },
  EntryMeta {
    name: "ignition_filter_expression_positions",
    comment: Some("filter expression positions before the bytecode pipeline"),
    flag_index: 195,
    canonical_index: 195,
  },
  EntryMeta {
    name: "ignition_share_named_property_feedback",
    comment: Some("share feedback slots when loading the same named property from the same object"),
    flag_index: 196,
    canonical_index: 196,
  },
  EntryMeta {
    name: "ignition_elide_redundant_tdz_checks",
    comment: Some("elide TDZ checks dominated by other TDZ checks"),
    flag_index: 197,
    canonical_index: 197,
  },
  EntryMeta {
    name: "print_bytecode",
    comment: Some("print bytecode generated by ignition interpreter (developer-only)"),
    flag_index: 198,
    canonical_index: 198,
  },
  EntryMeta {
    name: "enable_lazy_source_positions",
    comment: Some(
      "skip generating source positions during initial compile but regenerate when actually required",
    ),
    flag_index: 199,
    canonical_index: 199,
  },
  EntryMeta {
    name: "stress_lazy_source_positions",
    comment: Some("collect lazy source positions immediately after lazy compile"),
    flag_index: 200,
    canonical_index: 200,
  },
  EntryMeta {
    name: "stress_lazy",
    comment: Some("stress lazy compilation"),
    flag_index: 201,
    canonical_index: 201,
  },
  EntryMeta {
    name: "print_bytecode_filter",
    comment: Some("filter for selecting which functions to print bytecode"),
    flag_index: 202,
    canonical_index: 202,
  },
  EntryMeta {
    name: "omit_default_ctors",
    comment: Some("omit calling default ctors in bytecode"),
    flag_index: 203,
    canonical_index: 203,
  },
  EntryMeta {
    name: "trace_ignition_codegen",
    comment: Some("trace the codegen of ignition interpreter bytecode handlers (developer-only)"),
    flag_index: 204,
    canonical_index: 204,
  },
  EntryMeta {
    name: "trace_track_allocation_sites",
    comment: Some("trace the tracking of allocation sites (developer-only)"),
    flag_index: 205,
    canonical_index: 205,
  },
  EntryMeta {
    name: "trace_migration",
    comment: Some("trace object migration (developer-only)"),
    flag_index: 206,
    canonical_index: 206,
  },
  EntryMeta {
    name: "trace_generalization",
    comment: Some("trace map generalization (developer-only)"),
    flag_index: 207,
    canonical_index: 207,
  },
  EntryMeta {
    name: "sparkplug",
    comment: Some("enable Sparkplug baseline compiler"),
    flag_index: 208,
    canonical_index: 208,
  },
  EntryMeta {
    name: "always_sparkplug",
    comment: Some("directly tier up to Sparkplug code"),
    flag_index: 209,
    canonical_index: 209,
  },
  EntryMeta {
    name: "baseline_batch_compilation",
    comment: Some("batch compile Sparkplug code"),
    flag_index: 210,
    canonical_index: 210,
  },
  EntryMeta {
    name: "concurrent_sparkplug",
    comment: Some("compile Sparkplug code in a background thread"),
    flag_index: 211,
    canonical_index: 211,
  },
  EntryMeta {
    name: "concurrent_sparkplug_max_threads",
    comment: Some("max number of threads that concurrent Sparkplug can use (0 for unbounded)"),
    flag_index: 212,
    canonical_index: 212,
  },
  EntryMeta {
    name: "concurrent_sparkplug_high_priority_threads",
    comment: Some("use high priority compiler threads for concurrent Sparkplug"),
    flag_index: 213,
    canonical_index: 213,
  },
  EntryMeta {
    name: "sparkplug_filter",
    comment: Some("filter for Sparkplug baseline compiler"),
    flag_index: 214,
    canonical_index: 214,
  },
  EntryMeta {
    name: "sparkplug_needs_short_builtins",
    comment: Some(
      "only enable Sparkplug baseline compiler when --short-builtin-calls are also enabled",
    ),
    flag_index: 215,
    canonical_index: 215,
  },
  EntryMeta {
    name: "baseline_batch_compilation_threshold",
    comment: Some("the estimated instruction size of a batch to trigger compilation"),
    flag_index: 216,
    canonical_index: 216,
  },
  EntryMeta {
    name: "trace_baseline",
    comment: Some("trace baseline compilation (developer-only)"),
    flag_index: 217,
    canonical_index: 217,
  },
  EntryMeta {
    name: "trace_baseline_batch_compilation",
    comment: Some("trace baseline batch compilation (developer-only)"),
    flag_index: 218,
    canonical_index: 218,
  },
  EntryMeta {
    name: "shared_strings",
    comment: Some("allow sharing of strings"),
    flag_index: 219,
    canonical_index: 219,
  },
  EntryMeta {
    name: "shared_string_table",
    comment: Some("internalize strings into shared table (experimental)"),
    flag_index: 220,
    canonical_index: 220,
  },
  EntryMeta {
    name: "always_use_string_forwarding_table",
    comment: Some(
      "use string forwarding table instead of thin strings for all strings (experimental)",
    ),
    flag_index: 221,
    canonical_index: 221,
  },
  EntryMeta {
    name: "transition_strings_during_gc_with_stack",
    comment: Some("Transition strings during a full GC with stack"),
    flag_index: 222,
    canonical_index: 222,
  },
  EntryMeta {
    name: "initial_shared_heap_size",
    comment: Some(
      "initial size of the shared heap (in Mbytes); other heap size flags (e.g. initial_heap_size) take precedence",
    ),
    flag_index: 223,
    canonical_index: 223,
  },
  EntryMeta {
    name: "max_shared_heap_size",
    comment: Some(
      "max size of the shared heap (in Mbytes); other heap size flags (e.g. max_shared_heap_size) take precedence",
    ),
    flag_index: 224,
    canonical_index: 224,
  },
  EntryMeta {
    name: "concurrent_builtin_generation",
    comment: Some("generate builtins concurrently on separate threads in mksnapshot"),
    flag_index: 225,
    canonical_index: 225,
  },
  EntryMeta {
    name: "concurrent_recompilation",
    comment: Some("optimizing hot functions asynchronously on a separate thread"),
    flag_index: 226,
    canonical_index: 226,
  },
  EntryMeta {
    name: "trace_concurrent_recompilation",
    comment: Some("track concurrent recompilation (developer-only)"),
    flag_index: 227,
    canonical_index: 227,
  },
  EntryMeta {
    name: "concurrent_recompilation_queue_length",
    comment: Some("the length of the concurrent compilation queue"),
    flag_index: 228,
    canonical_index: 228,
  },
  EntryMeta {
    name: "concurrent_recompilation_delay",
    comment: Some("artificial compilation delay in ms"),
    flag_index: 229,
    canonical_index: 229,
  },
  EntryMeta {
    name: "concurrent_recompilation_front_running",
    comment: Some("move compile jobs to the front if recompilation is requested multiple times"),
    flag_index: 230,
    canonical_index: 230,
  },
  EntryMeta {
    name: "concurrent_turbofan_max_threads",
    comment: Some("max number of threads that concurrent Turbofan can use (0 for unbounded)"),
    flag_index: 231,
    canonical_index: 231,
  },
  EntryMeta {
    name: "stress_concurrent_inlining",
    comment: Some("create additional concurrent optimization jobs but throw away result"),
    flag_index: 232,
    canonical_index: 232,
  },
  EntryMeta {
    name: "maglev_overwrite_budget",
    comment: Some("whether maglev resets the interrupt budget"),
    flag_index: 233,
    canonical_index: 233,
  },
  EntryMeta {
    name: "maglev_overwrite_osr_budget",
    comment: Some("whether maglev resets the OSR interrupt budget"),
    flag_index: 234,
    canonical_index: 234,
  },
  EntryMeta {
    name: "stress_concurrent_inlining_attach_code",
    comment: Some("create additional concurrent optimization jobs"),
    flag_index: 235,
    canonical_index: 235,
  },
  EntryMeta {
    name: "max_serializer_nesting",
    comment: Some("maximum levels for nesting child serializers"),
    flag_index: 236,
    canonical_index: 236,
  },
  EntryMeta {
    name: "trace_heap_broker_verbose",
    comment: Some("trace the heap broker verbosely (all reports) (developer-only)"),
    flag_index: 237,
    canonical_index: 237,
  },
  EntryMeta {
    name: "trace_heap_broker",
    comment: Some("trace the heap broker (reports on missing data only) (developer-only)"),
    flag_index: 238,
    canonical_index: 238,
  },
  EntryMeta {
    name: "stress_runs",
    comment: Some("number of stress runs"),
    flag_index: 239,
    canonical_index: 239,
  },
  EntryMeta {
    name: "deopt_every_n_times",
    comment: Some("deoptimize every n times a deopt point is passed"),
    flag_index: 240,
    canonical_index: 240,
  },
  EntryMeta {
    name: "print_deopt_stress",
    comment: Some("print number of possible deopt points"),
    flag_index: 241,
    canonical_index: 241,
  },
  EntryMeta {
    name: "disable_loop_stack_checks",
    comment: Some("disable loop stack checks (for testing/fuzzing only)"),
    flag_index: 242,
    canonical_index: 242,
  },
  EntryMeta {
    name: "turbofan",
    comment: Some(
      "use the Turbofan optimizing compiler for JavaScript code (Wasm uses separate flags)",
    ),
    flag_index: 243,
    canonical_index: 243,
  },
  EntryMeta {
    name: "opt",
    comment: None,
    flag_index: 243,
    canonical_index: 243,
  },
  EntryMeta {
    name: "turbo_always_optimize_spills",
    comment: Some("optimize placement of all spill instructions, not just loop-top phis"),
    flag_index: 244,
    canonical_index: 245,
  },
  EntryMeta {
    name: "turbo_wasm_address_reassociation",
    comment: Some("refactor address components for immediate indexing"),
    flag_index: 245,
    canonical_index: 246,
  },
  EntryMeta {
    name: "concurrent_turbo_tracing",
    comment: Some("allow concurrent compilation to happen in combination with trace-turbo* flags"),
    flag_index: 246,
    canonical_index: 247,
  },
  EntryMeta {
    name: "optimize_maglev_optimizes_to_turbofan",
    comment: Some("make OptimizeMaglevOnNextCall optimize to turbofan instead of maglev"),
    flag_index: 247,
    canonical_index: 248,
  },
  EntryMeta {
    name: "turbo_filter",
    comment: Some("optimization filter for TurboFan compiler"),
    flag_index: 248,
    canonical_index: 249,
  },
  EntryMeta {
    name: "trace_turbo",
    comment: Some("trace generated TurboFan IR (developer-only)"),
    flag_index: 249,
    canonical_index: 250,
  },
  EntryMeta {
    name: "trace_turbo_path",
    comment: Some("directory to dump generated TurboFan IR to"),
    flag_index: 250,
    canonical_index: 251,
  },
  EntryMeta {
    name: "trace_turbo_filter",
    comment: Some("filter for tracing turbofan compilation"),
    flag_index: 251,
    canonical_index: 252,
  },
  EntryMeta {
    name: "trace_turbo_graph",
    comment: Some("trace generated TurboFan graphs (developer-only)"),
    flag_index: 252,
    canonical_index: 253,
  },
  EntryMeta {
    name: "trace_turbo_scheduled",
    comment: Some("trace TurboFan IR with schedule (developer-only)"),
    flag_index: 253,
    canonical_index: 254,
  },
  EntryMeta {
    name: "trace_turbo_file_prefix",
    comment: Some("trace turbo graph to a file with given prefix"),
    flag_index: 254,
    canonical_index: 255,
  },
  EntryMeta {
    name: "trace_turbo_cfg_file",
    comment: Some("trace turbo cfg graph (for C1 visualizer) to a given file name"),
    flag_index: 255,
    canonical_index: 256,
  },
  EntryMeta {
    name: "trace_turbo_types",
    comment: Some("trace TurboFan's types"),
    flag_index: 256,
    canonical_index: 257,
  },
  EntryMeta {
    name: "trace_turbo_scheduler",
    comment: Some("trace TurboFan's scheduler"),
    flag_index: 257,
    canonical_index: 258,
  },
  EntryMeta {
    name: "trace_turbo_reduction",
    comment: Some("trace TurboFan's various reducers"),
    flag_index: 258,
    canonical_index: 259,
  },
  EntryMeta {
    name: "trace_turbo_bailouts",
    comment: Some("trace TurboFan's various bailouts"),
    flag_index: 259,
    canonical_index: 260,
  },
  EntryMeta {
    name: "trace_turbo_trimming",
    comment: Some("trace TurboFan's graph trimmer"),
    flag_index: 260,
    canonical_index: 261,
  },
  EntryMeta {
    name: "trace_turbo_jt",
    comment: Some("trace TurboFan's jump threading"),
    flag_index: 261,
    canonical_index: 262,
  },
  EntryMeta {
    name: "trace_turbo_ceq",
    comment: Some("trace TurboFan's control equivalence"),
    flag_index: 262,
    canonical_index: 263,
  },
  EntryMeta {
    name: "trace_turbo_loop",
    comment: Some("trace TurboFan's loop optimizations"),
    flag_index: 263,
    canonical_index: 264,
  },
  EntryMeta {
    name: "trace_turbo_alloc",
    comment: Some("trace TurboFan's register allocator"),
    flag_index: 264,
    canonical_index: 265,
  },
  EntryMeta {
    name: "trace_all_uses",
    comment: Some("trace all use positions"),
    flag_index: 265,
    canonical_index: 266,
  },
  EntryMeta {
    name: "trace_representation",
    comment: Some("trace representation types"),
    flag_index: 266,
    canonical_index: 267,
  },
  EntryMeta {
    name: "trace_turbo_stack_accesses",
    comment: Some("trace stack load/store counters for optimized code in run-time (x64 only)"),
    flag_index: 267,
    canonical_index: 268,
  },
  EntryMeta {
    name: "fuzzing_and_concurrent_recompilation",
    comment: Some("fuzzing && concurrent_recompilation"),
    flag_index: 268,
    canonical_index: 269,
  },
  EntryMeta {
    name: "turbo_verify",
    comment: Some("verify TurboFan graphs at each phase"),
    flag_index: 269,
    canonical_index: 270,
  },
  EntryMeta {
    name: "turbo_verify_machine_graph",
    comment: Some("verify TurboFan machine graph before instruction selection"),
    flag_index: 270,
    canonical_index: 271,
  },
  EntryMeta {
    name: "verify_csa",
    comment: Some("verify TurboFan machine graph of code stubs"),
    flag_index: 271,
    canonical_index: 272,
  },
  EntryMeta {
    name: "trace_verify_csa",
    comment: Some("trace code stubs verification (developer-only)"),
    flag_index: 272,
    canonical_index: 273,
  },
  EntryMeta {
    name: "csa_trap_on_node",
    comment: Some(
      "trigger break point when a node with given id is created in given stub. The format is: StubName,NodeId",
    ),
    flag_index: 273,
    canonical_index: 274,
  },
  EntryMeta {
    name: "builtins_effects_src",
    comment: Some("path where to generate builtins-effects.h (used for mksnapshot only)"),
    flag_index: 274,
    canonical_index: 275,
  },
  EntryMeta {
    name: "fixed_array_bounds_checks",
    comment: Some("enable FixedArray bounds checks"),
    flag_index: 275,
    canonical_index: 276,
  },
  EntryMeta {
    name: "turbo_stats",
    comment: Some("print TurboFan statistics"),
    flag_index: 276,
    canonical_index: 277,
  },
  EntryMeta {
    name: "turbo_stats_nvp",
    comment: Some("print TurboFan statistics in machine-readable format"),
    flag_index: 277,
    canonical_index: 278,
  },
  EntryMeta {
    name: "turbo_stats_wasm",
    comment: Some("print TurboFan statistics of wasm compilations"),
    flag_index: 278,
    canonical_index: 279,
  },
  EntryMeta {
    name: "turbo_splitting",
    comment: Some("split nodes during scheduling in TurboFan"),
    flag_index: 279,
    canonical_index: 280,
  },
  EntryMeta {
    name: "turbo_inlining",
    comment: Some("enable inlining in TurboFan"),
    flag_index: 280,
    canonical_index: 281,
  },
  EntryMeta {
    name: "forin_enumerator_holder",
    comment: Some("optimize for-in over packed elements using ForInEnumeratorHolder"),
    flag_index: 281,
    canonical_index: 282,
  },
  EntryMeta {
    name: "turbo_elide_frames",
    comment: Some("enable frame elision in TurboFan"),
    flag_index: 282,
    canonical_index: 283,
  },
  EntryMeta {
    name: "max_inlined_bytecode_size",
    comment: Some("maximum size of bytecode for a single inlining"),
    flag_index: 283,
    canonical_index: 284,
  },
  EntryMeta {
    name: "max_inlined_bytecode_size_cumulative",
    comment: Some("maximum cumulative size of bytecode considered for inlining"),
    flag_index: 284,
    canonical_index: 285,
  },
  EntryMeta {
    name: "max_inlined_bytecode_size_absolute",
    comment: Some("maximum absolute size of bytecode considered for inlining"),
    flag_index: 285,
    canonical_index: 286,
  },
  EntryMeta {
    name: "max_inlined_bytecode_size_small_total",
    comment: Some("maximum size of small functions' bytecode that can be inlining"),
    flag_index: 286,
    canonical_index: 287,
  },
  EntryMeta {
    name: "reserve_inline_budget_scale_factor",
    comment: Some("scale factor of bytecode size used to calculate the inlining budget"),
    flag_index: 287,
    canonical_index: 288,
  },
  EntryMeta {
    name: "max_inlined_bytecode_size_small",
    comment: Some("maximum size of bytecode considered for small function inlining"),
    flag_index: 288,
    canonical_index: 289,
  },
  EntryMeta {
    name: "max_inlined_bytecode_size_small_with_heapnum_in_out",
    comment: Some(
      "maximum size of bytecode considered for small function inlining, when the function call has a HeapNumber as input or output",
    ),
    flag_index: 289,
    canonical_index: 290,
  },
  EntryMeta {
    name: "max_optimized_bytecode_size",
    comment: Some(
      "maximum bytecode size to be considered for turbofan optimization; too high values may cause the compiler to hit (release) assertions",
    ),
    flag_index: 290,
    canonical_index: 291,
  },
  EntryMeta {
    name: "max_maglev_optimized_bytecode_size",
    comment: Some("maximum bytecode size to be considered for maglev optimization"),
    flag_index: 291,
    canonical_index: 292,
  },
  EntryMeta {
    name: "min_inlining_frequency",
    comment: Some("minimum frequency for inlining"),
    flag_index: 292,
    canonical_index: 293,
  },
  EntryMeta {
    name: "stress_inline",
    comment: Some("set high thresholds for inlining to inline as much as possible"),
    flag_index: 293,
    canonical_index: 294,
  },
  EntryMeta {
    name: "trace_turbo_inlining",
    comment: Some("trace TurboFan inlining (developer-only)"),
    flag_index: 294,
    canonical_index: 295,
  },
  EntryMeta {
    name: "turbo_inline_array_builtins",
    comment: Some("inline array builtins in TurboFan code"),
    flag_index: 295,
    canonical_index: 296,
  },
  EntryMeta {
    name: "use_osr",
    comment: Some("use on-stack replacement"),
    flag_index: 296,
    canonical_index: 297,
  },
  EntryMeta {
    name: "maglev_osr",
    comment: Some("use maglev as on-stack replacement target"),
    flag_index: 297,
    canonical_index: 298,
  },
  EntryMeta {
    name: "max_maglev_eager_inlined_bytecode_size",
    comment: Some("maximum size of bytecode considered for eager inlining"),
    flag_index: 298,
    canonical_index: 299,
  },
  EntryMeta {
    name: "max_turbolev_eager_inlined_bytecode_size",
    comment: Some("maximum size of bytecode considered for eager inlining"),
    flag_index: 299,
    canonical_index: 300,
  },
  EntryMeta {
    name: "concurrent_osr",
    comment: Some("enable concurrent OSR"),
    flag_index: 300,
    canonical_index: 301,
  },
  EntryMeta {
    name: "maglev_allocation_folding",
    comment: Some("maglev allocation folding level"),
    flag_index: 301,
    canonical_index: 302,
  },
  EntryMeta {
    name: "maglev_escape_analysis",
    comment: Some("avoid inlined allocation of objects that cannot escape"),
    flag_index: 302,
    canonical_index: 303,
  },
  EntryMeta {
    name: "trace_maglev_escape_analysis",
    comment: Some("trace maglev escape analysis (developer-only)"),
    flag_index: 303,
    canonical_index: 304,
  },
  EntryMeta {
    name: "maglev_object_tracking",
    comment: Some("track object changes to avoid escaping them (experimental)"),
    flag_index: 304,
    canonical_index: 305,
  },
  EntryMeta {
    name: "trace_maglev_object_tracking",
    comment: Some("trace load/stores from maglev virtual objects (developer-only)"),
    flag_index: 305,
    canonical_index: 306,
  },
  EntryMeta {
    name: "turbo_string_builder",
    comment: Some("use TurboFan fast string builder"),
    flag_index: 306,
    canonical_index: 307,
  },
  EntryMeta {
    name: "trace_osr",
    comment: Some("trace on-stack replacement (developer-only)"),
    flag_index: 307,
    canonical_index: 308,
  },
  EntryMeta {
    name: "log_or_trace_osr",
    comment: Some("internal helper flag, please use --trace-osr instead."),
    flag_index: 308,
    canonical_index: 309,
  },
  EntryMeta {
    name: "analyze_environment_liveness",
    comment: Some("analyze liveness of environment slots and zap dead values"),
    flag_index: 309,
    canonical_index: 310,
  },
  EntryMeta {
    name: "trace_environment_liveness",
    comment: Some("trace liveness of local variable slots (developer-only)"),
    flag_index: 310,
    canonical_index: 311,
  },
  EntryMeta {
    name: "turbo_load_elimination",
    comment: Some("enable load elimination in TurboFan"),
    flag_index: 311,
    canonical_index: 312,
  },
  EntryMeta {
    name: "trace_turbo_load_elimination",
    comment: Some("trace TurboFan load elimination (developer-only)"),
    flag_index: 312,
    canonical_index: 313,
  },
  EntryMeta {
    name: "turbo_profiling",
    comment: Some("enable basic block profiling in TurboFan"),
    flag_index: 313,
    canonical_index: 314,
  },
  EntryMeta {
    name: "turbo_profiling_verbose",
    comment: Some(
      "enable basic block profiling in TurboFan, and include each function's schedule and disassembly in the output",
    ),
    flag_index: 314,
    canonical_index: 315,
  },
  EntryMeta {
    name: "turbo_profiling_output",
    comment: Some(
      "emit data about basic block usage in builtins to this file (requires that V8 was built with v8_enable_builtins_profiling=true)",
    ),
    flag_index: 315,
    canonical_index: 316,
  },
  EntryMeta {
    name: "reorder_builtins",
    comment: Some("enable builtin reordering when run mksnapshot."),
    flag_index: 316,
    canonical_index: 317,
  },
  EntryMeta {
    name: "abort_on_bad_builtin_profile_data",
    comment: Some("flag for mksnapshot, abort if builtins profile can't be applied"),
    flag_index: 317,
    canonical_index: 318,
  },
  EntryMeta {
    name: "warn_about_builtin_profile_data",
    comment: Some("flag for mksnapshot, emit warnings when applying builtin profile data"),
    flag_index: 318,
    canonical_index: 319,
  },
  EntryMeta {
    name: "dump_builtins_hashes_to_file",
    comment: Some("flag for mksnapshot, dump CSA builtins graph hashes to this file"),
    flag_index: 319,
    canonical_index: 320,
  },
  EntryMeta {
    name: "turbo_verify_allocation",
    comment: Some("verify register allocation in TurboFan"),
    flag_index: 320,
    canonical_index: 321,
  },
  EntryMeta {
    name: "turbo_move_optimization",
    comment: Some("optimize gap moves in TurboFan"),
    flag_index: 321,
    canonical_index: 322,
  },
  EntryMeta {
    name: "turbo_jt",
    comment: Some("enable jump threading in TurboFan"),
    flag_index: 322,
    canonical_index: 323,
  },
  EntryMeta {
    name: "turbo_loop_peeling",
    comment: Some("TurboFan loop peeling"),
    flag_index: 323,
    canonical_index: 324,
  },
  EntryMeta {
    name: "turbo_loop_variable",
    comment: Some("TurboFan loop variable optimization"),
    flag_index: 324,
    canonical_index: 325,
  },
  EntryMeta {
    name: "turbo_loop_rotation",
    comment: Some("TurboFan loop rotation"),
    flag_index: 325,
    canonical_index: 326,
  },
  EntryMeta {
    name: "turbo_cf_optimization",
    comment: Some("optimize control flow in TurboFan"),
    flag_index: 326,
    canonical_index: 327,
  },
  EntryMeta {
    name: "turbo_escape",
    comment: Some("enable escape analysis"),
    flag_index: 327,
    canonical_index: 328,
  },
  EntryMeta {
    name: "turbo_allocation_folding",
    comment: Some("TurboFan allocation folding"),
    flag_index: 328,
    canonical_index: 329,
  },
  EntryMeta {
    name: "turbo_instruction_scheduling",
    comment: Some("enable instruction scheduling in TurboFan"),
    flag_index: 329,
    canonical_index: 330,
  },
  EntryMeta {
    name: "experimental_turbo_instruction_scheduling",
    comment: Some("enable experimental instruction scheduling in TurboFan"),
    flag_index: 330,
    canonical_index: 331,
  },
  EntryMeta {
    name: "turbo_stress_instruction_scheduling",
    comment: Some("randomly schedule instructions to stress dependency tracking"),
    flag_index: 331,
    canonical_index: 332,
  },
  EntryMeta {
    name: "trace_turbo_instruction_scheduling",
    comment: Some("trace TurboShaft's instruction scheduler"),
    flag_index: 332,
    canonical_index: 333,
  },
  EntryMeta {
    name: "turbo_store_elimination",
    comment: Some("enable store-store elimination in TurboFan"),
    flag_index: 333,
    canonical_index: 334,
  },
  EntryMeta {
    name: "trace_store_elimination",
    comment: Some("trace store elimination (developer-only)"),
    flag_index: 334,
    canonical_index: 335,
  },
  EntryMeta {
    name: "turbo_typer_hardening",
    comment: Some(
      "extra bounds checks to protect against some known typer mismatch exploit techniques (best effort)",
    ),
    flag_index: 335,
    canonical_index: 336,
  },
  EntryMeta {
    name: "turbo_rewrite_far_jumps",
    comment: Some("rewrite far to near jumps (ia32,x64)"),
    flag_index: 336,
    canonical_index: 337,
  },
  EntryMeta {
    name: "stress_gc_during_compilation",
    comment: Some("simulate GC/compiler thread race related to https://crbug.com/v8/8520"),
    flag_index: 337,
    canonical_index: 338,
  },
  EntryMeta {
    name: "turbo_fast_api_calls",
    comment: Some("enable fast API calls from TurboFan"),
    flag_index: 338,
    canonical_index: 339,
  },
  EntryMeta {
    name: "fast_api_allow_float_in_sim",
    comment: Some("allow float parameters to be passed in simulator mode"),
    flag_index: 339,
    canonical_index: 340,
  },
  EntryMeta {
    name: "turbo_inline_js_wasm_calls",
    comment: Some(
      "inline JS->Wasm calls (specifically: inline JS-to-Wasm wrappers and then the body of the Wasm function, if applicable)",
    ),
    flag_index: 340,
    canonical_index: 341,
  },
  EntryMeta {
    name: "turbo_optimize_inlined_js_wasm_wrappers",
    comment: Some(
      "Run additional optimizations (especially load-elimination) on inlined JS-to-Wasm wrappers",
    ),
    flag_index: 341,
    canonical_index: 342,
  },
  EntryMeta {
    name: "turbo_optimize_apply",
    comment: Some("optimize Function.prototype.apply"),
    flag_index: 342,
    canonical_index: 343,
  },
  EntryMeta {
    name: "turbo_optimize_math_minmax",
    comment: Some("optimize call math.min/max with double array"),
    flag_index: 343,
    canonical_index: 344,
  },
  EntryMeta {
    name: "turboshaft",
    comment: Some("enable TurboFan's Turboshaft phases for JS"),
    flag_index: 344,
    canonical_index: 345,
  },
  EntryMeta {
    name: "turboshaft_enable_debug_features",
    comment: Some(
      "enables Turboshaft's DebugPrint, StaticAssert and CheckTurboshaftTypeOf operations",
    ),
    flag_index: 345,
    canonical_index: 346,
  },
  EntryMeta {
    name: "turboshaft_wasm_load_elimination",
    comment: Some("enable Turboshaft's WasmLoadElimination"),
    flag_index: 346,
    canonical_index: 347,
  },
  EntryMeta {
    name: "wasm_in_js_inlining_body",
    comment: Some(
      "inline Wasm code into JS functions via Turboshaft. This requires inlining the wrapper, see --wasm-in-js-inlining-wrapper. For controlling the inlining in the old Turbofan pipeline, see --turbo-inline-js-wasm-calls.",
    ),
    flag_index: 347,
    canonical_index: 348,
  },
  EntryMeta {
    name: "wasm_in_js_inlining_wrapper",
    comment: Some("inline JS-to-Wasm wrappers via Turboshaft/Turbolev."),
    flag_index: 348,
    canonical_index: 349,
  },
  EntryMeta {
    name: "wasm_in_js_inlining_opt",
    comment: Some("enable optimizations for Wasm-in-JS inlining in Turboshaft (experimental)"),
    flag_index: 349,
    canonical_index: 350,
  },
  EntryMeta {
    name: "turboshaft_load_elimination",
    comment: Some("enable Turboshaft's low-level load elimination for JS"),
    flag_index: 350,
    canonical_index: 351,
  },
  EntryMeta {
    name: "turboshaft_loop_unrolling",
    comment: Some("enable Turboshaft's loop unrolling"),
    flag_index: 351,
    canonical_index: 352,
  },
  EntryMeta {
    name: "turboshaft_loop_optimization",
    comment: Some("enable Turboshaft's loop optimization phase (experimental)"),
    flag_index: 352,
    canonical_index: 353,
  },
  EntryMeta {
    name: "turboshaft_random_rescheduling",
    comment: Some("enable Turboshaft's random rescheduling phase (experimental)"),
    flag_index: 353,
    canonical_index: 354,
  },
  EntryMeta {
    name: "turboshaft_string_concat_escape_analysis",
    comment: Some("enable Turboshaft's escape analysis for string concatenation"),
    flag_index: 354,
    canonical_index: 355,
  },
  EntryMeta {
    name: "turboshaft_trusted_load_elimination",
    comment: Some("enable Turboshaft's low level load elimination for trusted loads (JS and Wasm)"),
    flag_index: 355,
    canonical_index: 356,
  },
  EntryMeta {
    name: "turboshaft_typed_optimizations",
    comment: Some(
      "enable an additional Turboshaft phase that performs optimizations based on type information (experimental)",
    ),
    flag_index: 356,
    canonical_index: 357,
  },
  EntryMeta {
    name: "wasm_simd_opt",
    comment: Some("enable optimizations for Webassembly SIMD"),
    flag_index: 357,
    canonical_index: 358,
  },
  EntryMeta {
    name: "future_wasm_simd_opt",
    comment: Some("enable extra optimizations for Webassembly SIMD (experimental)"),
    flag_index: 358,
    canonical_index: 359,
  },
  EntryMeta {
    name: "wasm_deinterleave_loads",
    comment: Some("enable deinterleaving loads for Webassembly SIMD (experimental)"),
    flag_index: 359,
    canonical_index: 360,
  },
  EntryMeta {
    name: "turbolev",
    comment: Some(
      "use Turbolev (≈ Maglev + Turboshaft combined) as the 4th tier compiler instead of Turbofan",
    ),
    flag_index: 360,
    canonical_index: 361,
  },
  EntryMeta {
    name: "print_turbolev_frontend",
    comment: Some("print Turbolev frontend (Maglev graphs) (developer-only)"),
    flag_index: 361,
    canonical_index: 362,
  },
  EntryMeta {
    name: "turbolev_phase_filter",
    comment: Some("filter for turbolev phases (comma-separated list of phase names)"),
    flag_index: 362,
    canonical_index: 363,
  },
  EntryMeta {
    name: "print_turbolev_inline_functions",
    comment: Some("print Turbolev inline functions (developer-only)"),
    flag_index: 363,
    canonical_index: 364,
  },
  EntryMeta {
    name: "turbolev_future",
    comment: Some(
      "enable Turbolev features that we want to ship in the not-too-far future (experimental)",
    ),
    flag_index: 364,
    canonical_index: 365,
  },
  EntryMeta {
    name: "turbolev_escape_analysis",
    comment: Some("enable escape analysis for Turbolev (experimental)"),
    flag_index: 365,
    canonical_index: 366,
  },
  EntryMeta {
    name: "trace_turbolev_escape_analysis",
    comment: Some("enable tracing for Turbolev's escape analysis phase (developer-only)"),
    flag_index: 366,
    canonical_index: 367,
  },
  EntryMeta {
    name: "typed_array_length_loading",
    comment: Some("Enable specializing loading the TypedArray length in Maglev / Turbofan"),
    flag_index: 367,
    canonical_index: 368,
  },
  EntryMeta {
    name: "deduplicate_heap_number_requests",
    comment: Some("avoid duplicated HeapNumber materializations in HeapNumberRequests"),
    flag_index: 368,
    canonical_index: 369,
  },
  EntryMeta {
    name: "turboshaft_trace_reduction",
    comment: Some("trace individual Turboshaft reduction steps"),
    flag_index: 369,
    canonical_index: 370,
  },
  EntryMeta {
    name: "turboshaft_trace_emitted",
    comment: Some("trace emitted Turboshaft instructions"),
    flag_index: 370,
    canonical_index: 371,
  },
  EntryMeta {
    name: "turboshaft_trace_intermediate_reductions",
    comment: Some("trace intermediate Turboshaft reduction steps"),
    flag_index: 371,
    canonical_index: 372,
  },
  EntryMeta {
    name: "turboshaft_trace_load_elimination",
    comment: Some("trace Turboshaft's late load elimination"),
    flag_index: 372,
    canonical_index: 373,
  },
  EntryMeta {
    name: "turboshaft_trace_store_store_elimination",
    comment: Some("trace Turboshaft's store store elimination"),
    flag_index: 373,
    canonical_index: 374,
  },
  EntryMeta {
    name: "turboshaft_trace_if_else_to_switch",
    comment: Some("trace Turboshaft's if-else to switch reducer"),
    flag_index: 374,
    canonical_index: 375,
  },
  EntryMeta {
    name: "verify_turboshaft",
    comment: Some("enable various verifications on Turboshaft"),
    flag_index: 375,
    canonical_index: 376,
  },
  EntryMeta {
    name: "profile_guided_optimization",
    comment: Some("profile guided optimization"),
    flag_index: 376,
    canonical_index: 377,
  },
  EntryMeta {
    name: "profile_guided_optimization_for_empty_feedback_vector",
    comment: Some("profile guided optimization for empty feedback vector"),
    flag_index: 377,
    canonical_index: 378,
  },
  EntryMeta {
    name: "invocation_count_for_early_optimization",
    comment: Some("invocation count threshold for early optimization"),
    flag_index: 378,
    canonical_index: 379,
  },
  EntryMeta {
    name: "invocation_count_for_maglev_with_delay",
    comment: Some(
      "invocation count for maglev for functions which according to profile_guided_optimization are likely to deoptimize before reaching this invocation count",
    ),
    flag_index: 379,
    canonical_index: 380,
  },
  EntryMeta {
    name: "optimize_for_size",
    comment: Some("Enables optimizations which favor memory size over execution speed"),
    flag_index: 380,
    canonical_index: 381,
  },
  EntryMeta {
    name: "reopt_after_lazy_deopts",
    comment: Some("Immediately re-optimize code after some lazy deopts"),
    flag_index: 381,
    canonical_index: 382,
  },
  EntryMeta {
    name: "verify_get_js_builtin_state",
    comment: Some("Enable verification of Builtins::GetJSBuiltinState()."),
    flag_index: 382,
    canonical_index: 383,
  },
  EntryMeta {
    name: "experimental_fuzzing",
    comment: Some(
      "Implies all experimental pre-staged features that can already be fuzzed but are not ready for staging yet. (experimental)",
    ),
    flag_index: 383,
    canonical_index: 384,
  },
  EntryMeta {
    name: "future",
    comment: Some("Implies all staged features that we want to ship in the not-too-far future"),
    flag_index: 384,
    canonical_index: 385,
  },
  EntryMeta {
    name: "harmony",
    comment: Some("enable all completed harmony features"),
    flag_index: 385,
    canonical_index: 386,
  },
  EntryMeta {
    name: "js_staging",
    comment: Some("enable all completed JavaScript features"),
    flag_index: 386,
    canonical_index: 387,
  },
  EntryMeta {
    name: "wasm_staging",
    comment: Some("enable staged wasm features"),
    flag_index: 387,
    canonical_index: 388,
  },
  EntryMeta {
    name: "js_shipping",
    comment: Some("enable all shipped JavaScript features"),
    flag_index: 388,
    canonical_index: 389,
  },
  EntryMeta {
    name: "harmony_shipping",
    comment: Some("enable all shipped harmony features"),
    flag_index: 389,
    canonical_index: 390,
  },
  EntryMeta {
    name: "harmony_shadow_realm",
    comment: Some("enable harmony ShadowRealm for JS (experimental)"),
    flag_index: 390,
    canonical_index: 391,
  },
  EntryMeta {
    name: "harmony_struct",
    comment: Some(
      "enable harmony structs, shared structs, and shared arrays for JS (experimental)",
    ),
    flag_index: 391,
    canonical_index: 392,
  },
  EntryMeta {
    name: "harmony_intl_best_fit_matcher",
    comment: Some("enable Intl BestFitMatcher for JS (experimental)"),
    flag_index: 392,
    canonical_index: 393,
  },
  EntryMeta {
    name: "js_decorators",
    comment: Some("enable decorators for JS (experimental)"),
    flag_index: 393,
    canonical_index: 394,
  },
  EntryMeta {
    name: "js_source_phase_imports",
    comment: Some("enable source phase imports for JS (experimental)"),
    flag_index: 394,
    canonical_index: 395,
  },
  EntryMeta {
    name: "js_regexp_buffer_boundaries",
    comment: Some("enable RegExp \\A \\z \\Z buffer boundary assertions for JS (experimental)"),
    flag_index: 395,
    canonical_index: 396,
  },
  EntryMeta {
    name: "wasm_instruction_tracing",
    comment: Some("enable instruction tracing section for Wasm (experimental)"),
    flag_index: 396,
    canonical_index: 397,
  },
  EntryMeta {
    name: "wasm_shared",
    comment: Some("enable shared-everything threads for Wasm (experimental)"),
    flag_index: 397,
    canonical_index: 398,
  },
  EntryMeta {
    name: "wasm_fp16",
    comment: Some("enable fp16 for Wasm (experimental)"),
    flag_index: 398,
    canonical_index: 399,
  },
  EntryMeta {
    name: "wasm_memory_control",
    comment: Some("enable memory control for Wasm (experimental)"),
    flag_index: 399,
    canonical_index: 400,
  },
  EntryMeta {
    name: "wasm_wasmfx",
    comment: Some("enable core stack switching for Wasm (experimental)"),
    flag_index: 400,
    canonical_index: 401,
  },
  EntryMeta {
    name: "wasm_compilation_hints",
    comment: Some("enable compilation hints for Wasm (experimental)"),
    flag_index: 401,
    canonical_index: 402,
  },
  EntryMeta {
    name: "wasm_growable_stacks",
    comment: Some("enable growable stacks for jspi for Wasm (experimental)"),
    flag_index: 402,
    canonical_index: 403,
  },
  EntryMeta {
    name: "always_specialize_for_script_context",
    comment: Some("enable always specialize for script contexts in optimized code (experimental)"),
    flag_index: 403,
    canonical_index: 404,
  },
  EntryMeta {
    name: "wasm_stringref",
    comment: Some("enable reference-typed strings for Wasm (experimental)"),
    flag_index: 404,
    canonical_index: 405,
  },
  EntryMeta {
    name: "wasm_imported_strings_utf8",
    comment: Some("enable imported strings (utf8 features) for Wasm (experimental)"),
    flag_index: 405,
    canonical_index: 406,
  },
  EntryMeta {
    name: "wasm_acquire_release",
    comment: Some("enable acquire_release memory ordering for Wasm (experimental)"),
    flag_index: 406,
    canonical_index: 407,
  },
  EntryMeta {
    name: "wasm_compact_imports",
    comment: Some("enable compact import section for Wasm (experimental)"),
    flag_index: 407,
    canonical_index: 408,
  },
  EntryMeta {
    name: "js_immutable_arraybuffer",
    comment: Some("enable Immutable ArrayBuffer for JS"),
    flag_index: 408,
    canonical_index: 409,
  },
  EntryMeta {
    name: "js_import_bytes",
    comment: Some("enable import bytes for JS"),
    flag_index: 409,
    canonical_index: 410,
  },
  EntryMeta {
    name: "js_defer_import_eval",
    comment: Some("enable defer import eval for JS"),
    flag_index: 410,
    canonical_index: 411,
  },
  EntryMeta {
    name: "wasm_custom_descriptors",
    comment: Some("enable custom descriptors for Wasm"),
    flag_index: 411,
    canonical_index: 412,
  },
  EntryMeta {
    name: "wasm_wide_arithmetic",
    comment: Some("enable wide arithmetic for Wasm"),
    flag_index: 412,
    canonical_index: 413,
  },
  EntryMeta {
    name: "harmony_import_attributes",
    comment: Some("enable harmony import attributes for JS"),
    flag_index: 413,
    canonical_index: 414,
  },
  EntryMeta {
    name: "harmony_temporal",
    comment: Some("enable Temporal for JS"),
    flag_index: 414,
    canonical_index: 415,
  },
  EntryMeta {
    name: "js_esm_ns_reexport",
    comment: Some(
      "enable Support diamond-importing re-expored namespaces (https://github.com/tc39/ecma262/pull/3715) for JS",
    ),
    flag_index: 415,
    canonical_index: 416,
  },
  EntryMeta {
    name: "js_iterator_join",
    comment: Some("enable Iterator.prototype.join for JS"),
    flag_index: 416,
    canonical_index: 417,
  },
  EntryMeta {
    name: "js_iterator_sequencing",
    comment: Some("enable iterator sequencing for JS"),
    flag_index: 417,
    canonical_index: 418,
  },
  EntryMeta {
    name: "js_joint_iteration",
    comment: Some("enable joint iteration for JS"),
    flag_index: 418,
    canonical_index: 419,
  },
  EntryMeta {
    name: "js_pr_3883",
    comment: Some(
      "enable Promise.try not wrapping the result in an extra promise in the non-throwing case (https://github.com/tc39/ecma262/pull/3883) for JS",
    ),
    flag_index: 419,
    canonical_index: 420,
  },
  EntryMeta {
    name: "js_iterator_includes",
    comment: Some("enable Iterator.prototype.includes for JS"),
    flag_index: 420,
    canonical_index: 421,
  },
  EntryMeta {
    name: "js_import_text",
    comment: Some("enable import text for JS"),
    flag_index: 421,
    canonical_index: 422,
  },
  EntryMeta {
    name: "wasm_legacy_eh",
    comment: Some("enable legacy exception handling opcodes for Wasm"),
    flag_index: 422,
    canonical_index: 423,
  },
  EntryMeta {
    name: "homomorphic_ic",
    comment: Some("enable Homomorphic IC state for same-handler highly polymorphic ICs"),
    flag_index: 423,
    canonical_index: 424,
  },
  EntryMeta {
    name: "wasm_generic_wrapper",
    comment: Some("allow use of the generic js-to-wasm wrapper instead of per-signature wrappers"),
    flag_index: 424,
    canonical_index: 425,
  },
  EntryMeta {
    name: "wasm_unsafe_fast_api_wrapper",
    comment: Some("allow use of the unsafe fast API wrapper (test-only / unsafe)"),
    flag_index: 425,
    canonical_index: 426,
  },
  EntryMeta {
    name: "wasm_num_compilation_tasks",
    comment: Some("maximum number of parallel compilation tasks for wasm"),
    flag_index: 426,
    canonical_index: 427,
  },
  EntryMeta {
    name: "trace_wasm_native_heap",
    comment: Some("trace wasm native heap events"),
    flag_index: 427,
    canonical_index: 428,
  },
  EntryMeta {
    name: "trace_wasm_offheap_memory",
    comment: Some(
      "print details of wasm off-heap memory when the memory measurement API is used (developer-only)",
    ),
    flag_index: 428,
    canonical_index: 429,
  },
  EntryMeta {
    name: "print_wasm_offheap_memory_size",
    comment: Some("print off-heap memory consumption per module and engine when they die"),
    flag_index: 429,
    canonical_index: 430,
  },
  EntryMeta {
    name: "trace_wasm_serialization",
    comment: Some("trace serialization/deserialization"),
    flag_index: 430,
    canonical_index: 431,
  },
  EntryMeta {
    name: "wasm_test_streaming",
    comment: Some("use streaming compilation instead of async compilation for tests"),
    flag_index: 431,
    canonical_index: 432,
  },
  EntryMeta {
    name: "wasm_native_module_cache",
    comment: Some("enable the native module cache"),
    flag_index: 432,
    canonical_index: 433,
  },
  EntryMeta {
    name: "wasm_max_mem_pages",
    comment: Some("maximum number of 64KiB memory pages per wasm memory"),
    flag_index: 433,
    canonical_index: 434,
  },
  EntryMeta {
    name: "wasm_max_table_size",
    comment: Some("maximum table size of a wasm instance"),
    flag_index: 434,
    canonical_index: 435,
  },
  EntryMeta {
    name: "wasm_max_committed_code_mb",
    comment: Some("maximum committed code space for wasm (in MB)"),
    flag_index: 435,
    canonical_index: 436,
  },
  EntryMeta {
    name: "wasm_max_code_space_size_mb",
    comment: Some("maximum size of a single wasm code space"),
    flag_index: 436,
    canonical_index: 437,
  },
  EntryMeta {
    name: "wasm_tier_up",
    comment: Some(
      "enable tier up to the optimizing compiler (requires --liftoff to have an effect)",
    ),
    flag_index: 437,
    canonical_index: 438,
  },
  EntryMeta {
    name: "wasm_dynamic_tiering",
    comment: Some("enable dynamic tier up to the optimizing compiler"),
    flag_index: 438,
    canonical_index: 439,
  },
  EntryMeta {
    name: "wasm_sync_tier_up",
    comment: Some("run Wasm tier up jobs synchronously"),
    flag_index: 439,
    canonical_index: 440,
  },
  EntryMeta {
    name: "wasm_tiering_budget",
    comment: Some("budget for dynamic tiering (rough approximation of bytes executed"),
    flag_index: 440,
    canonical_index: 441,
  },
  EntryMeta {
    name: "wasm_wrapper_tiering_budget",
    comment: Some("budget for wrapper tierup (number of calls until tier-up)"),
    flag_index: 441,
    canonical_index: 442,
  },
  EntryMeta {
    name: "max_wasm_functions",
    comment: Some("maximum number of wasm functions defined in a module"),
    flag_index: 442,
    canonical_index: 443,
  },
  EntryMeta {
    name: "wasm_caching_threshold",
    comment: Some("the amount of wasm top tier code that triggers the next caching event"),
    flag_index: 443,
    canonical_index: 444,
  },
  EntryMeta {
    name: "wasm_caching_hard_threshold",
    comment: Some(
      "the amount of wasm top tier code that triggers caching immediately, ignoring the --wasm-caching-timeout-ms",
    ),
    flag_index: 444,
    canonical_index: 445,
  },
  EntryMeta {
    name: "wasm_caching_timeout_ms",
    comment: Some(
      "only trigger caching if no new code was compiled within this timeout (0 to disable this logic and only use --wasm-caching-threshold)",
    ),
    flag_index: 445,
    canonical_index: 446,
  },
  EntryMeta {
    name: "trace_wasm_compilation_times",
    comment: Some("print how long it took to compile each wasm function (developer-only)"),
    flag_index: 446,
    canonical_index: 447,
  },
  EntryMeta {
    name: "wasm_tier_up_filter",
    comment: Some("only tier-up function with this index"),
    flag_index: 447,
    canonical_index: 448,
  },
  EntryMeta {
    name: "wasm_eager_tier_up_function",
    comment: Some("eagerly tier-up function with this index"),
    flag_index: 448,
    canonical_index: 449,
  },
  EntryMeta {
    name: "trace_wasm_decoder",
    comment: Some("trace decoding of wasm code"),
    flag_index: 449,
    canonical_index: 450,
  },
  EntryMeta {
    name: "trace_wasm_compiler",
    comment: Some("trace compiling of wasm code"),
    flag_index: 450,
    canonical_index: 451,
  },
  EntryMeta {
    name: "trace_wasm_streaming",
    comment: Some("trace streaming compilation of wasm code"),
    flag_index: 451,
    canonical_index: 452,
  },
  EntryMeta {
    name: "trace_wasm_stack_switching",
    comment: Some("trace wasm stack switching"),
    flag_index: 452,
    canonical_index: 453,
  },
  EntryMeta {
    name: "stress_wasm_stack_switching",
    comment: Some(
      "Always run wasm on a secondary stack, even when it is called with a regular (non-JSPI) export",
    ),
    flag_index: 453,
    canonical_index: 454,
  },
  EntryMeta {
    name: "wasm_stack_switching_stack_size",
    comment: Some("default size of stacks for wasm stack-switching (in kB)"),
    flag_index: 454,
    canonical_index: 455,
  },
  EntryMeta {
    name: "wasm_stack_pool_capacity_mb",
    comment: Some("default capacity for the wasm stack pool in MB, -1 for unlimited"),
    flag_index: 455,
    canonical_index: 456,
  },
  EntryMeta {
    name: "liftoff",
    comment: Some("enable Liftoff, the baseline compiler for WebAssembly"),
    flag_index: 456,
    canonical_index: 457,
  },
  EntryMeta {
    name: "liftoff_only",
    comment: Some("disallow TurboFan compilation for WebAssembly (for testing)"),
    flag_index: 457,
    canonical_index: 458,
  },
  EntryMeta {
    name: "enable_testing_opcode_in_wasm",
    comment: Some("enables a testing opcode in wasm that is only implemented in TurboFan"),
    flag_index: 458,
    canonical_index: 459,
  },
  EntryMeta {
    name: "trace_liftoff",
    comment: Some("trace Liftoff, the baseline compiler for WebAssembly"),
    flag_index: 459,
    canonical_index: 460,
  },
  EntryMeta {
    name: "trace_wasm_memory",
    comment: Some("print all memory updates performed in wasm code (developer-only)"),
    flag_index: 460,
    canonical_index: 461,
  },
  EntryMeta {
    name: "trace_wasm_globals",
    comment: Some("print all global variable updates performed in wasm code (developer-only)"),
    flag_index: 461,
    canonical_index: 462,
  },
  EntryMeta {
    name: "wasm_tier_mask_for_testing",
    comment: Some(
      "bitmask of declared(!) function indices to compile with TurboFan instead of Liftoff",
    ),
    flag_index: 462,
    canonical_index: 463,
  },
  EntryMeta {
    name: "wasm_debug_mask_for_testing",
    comment: Some(
      "bitmask of declared(!) function indices to compile for debugging, only applies if the tier is Liftoff",
    ),
    flag_index: 463,
    canonical_index: 464,
  },
  EntryMeta {
    name: "wasm_pgo_to_file",
    comment: Some("experimental: dump Wasm PGO information to a local file (test-only / unsafe)"),
    flag_index: 464,
    canonical_index: 465,
  },
  EntryMeta {
    name: "wasm_pgo_from_file",
    comment: Some(
      "experimental: read and use Wasm PGO data from a local file (test-only / unsafe)",
    ),
    flag_index: 465,
    canonical_index: 466,
  },
  EntryMeta {
    name: "dump_wasm_module",
    comment: Some("dump wasm module bytes"),
    flag_index: 466,
    canonical_index: 467,
  },
  EntryMeta {
    name: "dump_wasm_module_path",
    comment: Some("directory to dump wasm modules to"),
    flag_index: 467,
    canonical_index: 468,
  },
  EntryMeta {
    name: "wasm_fast_api",
    comment: Some(
      "Enable direct calls from wasm to fast API functions with bound call function to pass the receiver as first parameter (experimental)",
    ),
    flag_index: 468,
    canonical_index: 469,
  },
  EntryMeta {
    name: "wasm_assert_types",
    comment: Some(
      "Enable additional type assertions in generated code (e.g. turbofan) (experimental)",
    ),
    flag_index: 469,
    canonical_index: 470,
  },
  EntryMeta {
    name: "wasm_deopt",
    comment: Some("enable deopts in optimized wasm functions"),
    flag_index: 470,
    canonical_index: 471,
  },
  EntryMeta {
    name: "wasm_deopts_per_function_limit",
    comment: Some(
      "limit of wasm deopts for a single function after which no further deopt points are emitted in Turbofan",
    ),
    flag_index: 471,
    canonical_index: 472,
  },
  EntryMeta {
    name: "wasm_assume_ref_cast_succeeds",
    comment: Some(
      "assume ref.cast always succeeds and skip the related type check (test-only / unsafe)",
    ),
    flag_index: 472,
    canonical_index: 473,
  },
  EntryMeta {
    name: "wasm_ref_cast_nop",
    comment: Some("enable unsafe ref.cast_nop instruction (test-only / unsafe)"),
    flag_index: 473,
    canonical_index: 474,
  },
  EntryMeta {
    name: "wasm_skip_null_checks",
    comment: Some(
      "skip null checks for call.ref and array and struct operations (test-only / unsafe)",
    ),
    flag_index: 474,
    canonical_index: 475,
  },
  EntryMeta {
    name: "wasm_skip_bounds_checks",
    comment: Some("skip array bounds checks (test-only / unsafe)"),
    flag_index: 475,
    canonical_index: 476,
  },
  EntryMeta {
    name: "wasm_js_interop",
    comment: Some("enable JS Interop part of Custom Descriptors proposal (experimental)"),
    flag_index: 476,
    canonical_index: 477,
  },
  EntryMeta {
    name: "wasm_custom_descriptors_permitted",
    comment: Some("Emergency off-switch for Custom Descriptors Origin Trial"),
    flag_index: 477,
    canonical_index: 478,
  },
  EntryMeta {
    name: "wasm_merged_descriptors",
    comment: Some("merge Custom Descriptor into v8::internal::Map (experimental)"),
    flag_index: 478,
    canonical_index: 479,
  },
  EntryMeta {
    name: "wasm_opt",
    comment: Some("enable optimization when compiling Wasm functions with Turbofan"),
    flag_index: 479,
    canonical_index: 480,
  },
  EntryMeta {
    name: "wasm_bounds_checks",
    comment: Some("enable bounds checks (disable for performance testing only)"),
    flag_index: 480,
    canonical_index: 481,
  },
  EntryMeta {
    name: "wasm_stack_checks",
    comment: Some("enable stack checks (disable for performance testing only)"),
    flag_index: 481,
    canonical_index: 482,
  },
  EntryMeta {
    name: "wasm_enforce_bounds_checks",
    comment: Some("enforce explicit bounds check even if the trap handler is available"),
    flag_index: 482,
    canonical_index: 483,
  },
  EntryMeta {
    name: "wasm_partial_oob_writes_are_noops",
    comment: Some(
      "assume partially out-of-bounds writes are no-ops and can use the trap handler. Note that explicitly enabling this flag (if not enabled by default) can affect the spec-compliance of V8.",
    ),
    flag_index: 483,
    canonical_index: 484,
  },
  EntryMeta {
    name: "wasm_math_intrinsics",
    comment: Some("intrinsify some Math imports into wasm"),
    flag_index: 484,
    canonical_index: 485,
  },
  EntryMeta {
    name: "wasm_inlining",
    comment: Some("enable inlining of Wasm functions into Wasm functions"),
    flag_index: 485,
    canonical_index: 486,
  },
  EntryMeta {
    name: "wasm_inlining_budget",
    comment: Some("maximum graph size (in TF nodes) that allows inlining more"),
    flag_index: 486,
    canonical_index: 487,
  },
  EntryMeta {
    name: "wasm_inlining_max_size",
    comment: Some("maximum function size (in wire bytes) that may be inlined"),
    flag_index: 487,
    canonical_index: 488,
  },
  EntryMeta {
    name: "wasm_inlining_factor",
    comment: Some("maximum multiple graph size (in TF nodes) in comparison to initial size"),
    flag_index: 488,
    canonical_index: 489,
  },
  EntryMeta {
    name: "wasm_inlining_min_budget",
    comment: Some(
      "minimum graph size budget (in TF nodes) for which the wasm_inlinining_factor does not apply",
    ),
    flag_index: 489,
    canonical_index: 490,
  },
  EntryMeta {
    name: "wasm_inlining_ignore_call_counts",
    comment: Some(
      "Ignore call counts when considering inlining candidates. The flag is supposed to be used for fuzzing",
    ),
    flag_index: 490,
    canonical_index: 491,
  },
  EntryMeta {
    name: "trace_wasm_inlining",
    comment: Some("trace wasm inlining (developer-only)"),
    flag_index: 491,
    canonical_index: 492,
  },
  EntryMeta {
    name: "trace_wasm_typer",
    comment: Some("trace wasm typer (developer-only)"),
    flag_index: 492,
    canonical_index: 493,
  },
  EntryMeta {
    name: "trace_wasm_simd_shuffle",
    comment: Some("trace wasm simd shuffle (developer-only)"),
    flag_index: 493,
    canonical_index: 494,
  },
  EntryMeta {
    name: "wasm_inlining_call_indirect",
    comment: Some("enable speculative inlining of Wasm indirect calls"),
    flag_index: 494,
    canonical_index: 495,
  },
  EntryMeta {
    name: "wasm_bulkmem_inlining",
    comment: Some("enable inline code generation for small bulk memory operations"),
    flag_index: 495,
    canonical_index: 496,
  },
  EntryMeta {
    name: "wasm_loop_unrolling",
    comment: Some("enable loop unrolling for wasm functions"),
    flag_index: 496,
    canonical_index: 497,
  },
  EntryMeta {
    name: "wasm_random_rescheduling",
    comment: Some("enable Turbofan's random rescheduling phase for wasm functions (experimental)"),
    flag_index: 497,
    canonical_index: 498,
  },
  EntryMeta {
    name: "wasm_loop_peeling",
    comment: Some("enable loop peeling for wasm functions"),
    flag_index: 498,
    canonical_index: 499,
  },
  EntryMeta {
    name: "wasm_loop_peeling_max_size",
    comment: Some("maximum size for peeling"),
    flag_index: 499,
    canonical_index: 500,
  },
  EntryMeta {
    name: "trace_wasm_loop_peeling",
    comment: Some("trace wasm loop peeling (developer-only)"),
    flag_index: 500,
    canonical_index: 501,
  },
  EntryMeta {
    name: "wasm_fuzzer_gen_test",
    comment: Some("generate a test case when running a wasm fuzzer"),
    flag_index: 501,
    canonical_index: 502,
  },
  EntryMeta {
    name: "print_wasm_code",
    comment: Some("print WebAssembly code (developer-only)"),
    flag_index: 502,
    canonical_index: 503,
  },
  EntryMeta {
    name: "print_wasm_code_function_index",
    comment: Some("print WebAssembly code for function at index"),
    flag_index: 503,
    canonical_index: 504,
  },
  EntryMeta {
    name: "print_wasm_stub_code",
    comment: Some("print WebAssembly stub code (developer-only)"),
    flag_index: 504,
    canonical_index: 505,
  },
  EntryMeta {
    name: "wasm_lazy_compilation",
    comment: Some("enable lazy compilation for all wasm modules"),
    flag_index: 505,
    canonical_index: 506,
  },
  EntryMeta {
    name: "trace_wasm_lazy_compilation",
    comment: Some("trace lazy compilation of wasm functions"),
    flag_index: 506,
    canonical_index: 507,
  },
  EntryMeta {
    name: "wasm_code_gc",
    comment: Some("enable garbage collection of wasm code"),
    flag_index: 507,
    canonical_index: 508,
  },
  EntryMeta {
    name: "trace_wasm_code_gc",
    comment: Some("trace garbage collection of wasm code (developer-only)"),
    flag_index: 508,
    canonical_index: 509,
  },
  EntryMeta {
    name: "stress_wasm_code_gc",
    comment: Some("stress test garbage collection of wasm code"),
    flag_index: 509,
    canonical_index: 510,
  },
  EntryMeta {
    name: "wasm_max_initial_code_space_reservation",
    comment: Some("maximum size of the initial wasm code space reservation (in MB)"),
    flag_index: 510,
    canonical_index: 511,
  },
  EntryMeta {
    name: "stress_wasm_memory_moving",
    comment: Some("always move non-shared bounds-checked Wasm memory on grow"),
    flag_index: 511,
    canonical_index: 512,
  },
  EntryMeta {
    name: "flush_liftoff_code",
    comment: Some("enable flushing Liftoff code on memory pressure signal"),
    flag_index: 512,
    canonical_index: 513,
  },
  EntryMeta {
    name: "stress_branch_hinting",
    comment: Some("stress branch hinting by generating a random hint for each branch instruction"),
    flag_index: 513,
    canonical_index: 514,
  },
  EntryMeta {
    name: "wasm_max_module_size",
    comment: Some("maximum allowed size of wasm modules"),
    flag_index: 514,
    canonical_index: 515,
  },
  EntryMeta {
    name: "wasm_disassembly_max_mb",
    comment: Some("maximum size of produced disassembly (in MB, approximate)"),
    flag_index: 515,
    canonical_index: 516,
  },
  EntryMeta {
    name: "wasm_capi_thread_pool_size",
    comment: Some(
      "size of the thread pool used by the wasm C API, with 0 meaning the maximum number of threads",
    ),
    flag_index: 516,
    canonical_index: 517,
  },
  EntryMeta {
    name: "trace_wasm",
    comment: Some("trace wasm function calls (developer-only)"),
    flag_index: 517,
    canonical_index: 518,
  },
  EntryMeta {
    name: "trace_wasm_instances",
    comment: Some("trace creation and collection of wasm instances"),
    flag_index: 518,
    canonical_index: 519,
  },
  EntryMeta {
    name: "wasm_memory64_trap_handling",
    comment: Some("Use trap handling for Wasm memory64 bounds checks"),
    flag_index: 519,
    canonical_index: 520,
  },
  EntryMeta {
    name: "wasm_jitless",
    comment: Some("execute all Wasm code in the Wasm interpreter"),
    flag_index: 520,
    canonical_index: 521,
  },
  EntryMeta {
    name: "wasm_jitless_if_available_for_testing",
    comment: None,
    flag_index: 521,
    canonical_index: 522,
  },
  EntryMeta {
    name: "wasm_allow_mixed_eh_for_testing",
    comment: Some(
      "Allow mixed legacy and new exception handling instructions in the same wasm module",
    ),
    flag_index: 522,
    canonical_index: 523,
  },
  EntryMeta {
    name: "wasm_code_coverage",
    comment: Some("enable Wasm code coverage"),
    flag_index: 523,
    canonical_index: 524,
  },
  EntryMeta {
    name: "trace_wasm_compilation_hints",
    comment: Some("trace compilation hints parsing and usage (developer-only)"),
    flag_index: 524,
    canonical_index: 525,
  },
  EntryMeta {
    name: "wasm_generate_compilation_hints",
    comment: Some(
      "enable emitting compilation-hints sections for Wasm to a file (test-only / unsafe)",
    ),
    flag_index: 525,
    canonical_index: 526,
  },
  EntryMeta {
    name: "trace_wasm_generate_compilation_hints",
    comment: Some("enable tracing wasm compilation hints generation (test-only / unsafe)"),
    flag_index: 526,
    canonical_index: 527,
  },
  EntryMeta {
    name: "stress_sampling_allocation_profiler",
    comment: Some("Enables sampling allocation profiler with X as a sample interval"),
    flag_index: 527,
    canonical_index: 528,
  },
  EntryMeta {
    name: "lazy_new_space_shrinking",
    comment: Some("Enables the lazy new space shrinking strategy"),
    flag_index: 528,
    canonical_index: 529,
  },
  EntryMeta {
    name: "min_semi_space_size",
    comment: Some(
      "min size of a semi-space (in MBytes), the new space consists of two semi-spaces",
    ),
    flag_index: 529,
    canonical_index: 530,
  },
  EntryMeta {
    name: "max_semi_space_size",
    comment: Some(
      "max size of a semi-space (in MBytes), the new space consists of two semi-spaces",
    ),
    flag_index: 530,
    canonical_index: 531,
  },
  EntryMeta {
    name: "semi_space_growth_factor",
    comment: Some("factor by which to grow the new space"),
    flag_index: 531,
    canonical_index: 532,
  },
  EntryMeta {
    name: "max_old_space_size",
    comment: Some("max size of the old space (in Mbytes)"),
    flag_index: 532,
    canonical_index: 533,
  },
  EntryMeta {
    name: "max_heap_size",
    comment: Some(
      "max size of the heap (in Mbytes) both max_semi_space_size and max_old_space_size take precedence. All three flags cannot be specified at the same time.",
    ),
    flag_index: 533,
    canonical_index: 534,
  },
  EntryMeta {
    name: "initial_heap_size",
    comment: Some("initial size of the heap (in Mbytes)"),
    flag_index: 534,
    canonical_index: 535,
  },
  EntryMeta {
    name: "initial_old_space_size",
    comment: Some("initial old space size (in Mbytes)"),
    flag_index: 535,
    canonical_index: 536,
  },
  EntryMeta {
    name: "preconfigured_old_space_size",
    comment: Some("preconfigured old space size (in Mbytes)"),
    flag_index: 536,
    canonical_index: 537,
  },
  EntryMeta {
    name: "gc_global",
    comment: Some("always perform global GCs"),
    flag_index: 537,
    canonical_index: 538,
  },
  EntryMeta {
    name: "random_gc_interval",
    comment: Some("Collect garbage after random(0, X) V8 allocations. It overrides gc_interval."),
    flag_index: 538,
    canonical_index: 539,
  },
  EntryMeta {
    name: "gc_interval",
    comment: Some("garbage collect after <n> allocations"),
    flag_index: 539,
    canonical_index: 540,
  },
  EntryMeta {
    name: "cppgc_random_gc_interval",
    comment: Some("Collect garbage after random(0, X) cppgc allocations."),
    flag_index: 540,
    canonical_index: 541,
  },
  EntryMeta {
    name: "retain_maps_for_n_gc",
    comment: Some("keeps maps alive for <n> old space garbage collections"),
    flag_index: 541,
    canonical_index: 542,
  },
  EntryMeta {
    name: "trace_gc",
    comment: Some("print one trace line following each garbage collection (developer-only)"),
    flag_index: 542,
    canonical_index: 543,
  },
  EntryMeta {
    name: "trace_gc_nvp",
    comment: Some(
      "print one detailed trace line in name=value format after each garbage collection (developer-only)",
    ),
    flag_index: 543,
    canonical_index: 544,
  },
  EntryMeta {
    name: "trace_gc_ignore_scavenger",
    comment: Some("do not print trace line after scavenger collection (developer-only)"),
    flag_index: 544,
    canonical_index: 545,
  },
  EntryMeta {
    name: "trace_memory_reducer",
    comment: Some("print memory reducer behavior (developer-only)"),
    flag_index: 545,
    canonical_index: 546,
  },
  EntryMeta {
    name: "trace_gc_verbose",
    comment: Some("print more details following each garbage collection (developer-only)"),
    flag_index: 546,
    canonical_index: 547,
  },
  EntryMeta {
    name: "trace_gc_freelists",
    comment: Some(
      "prints details of each freelist before and after each major garbage collection (developer-only)",
    ),
    flag_index: 547,
    canonical_index: 548,
  },
  EntryMeta {
    name: "trace_gc_freelists_verbose",
    comment: Some(
      "prints details of freelists of each page before and after each major garbage collection (developer-only)",
    ),
    flag_index: 548,
    canonical_index: 549,
  },
  EntryMeta {
    name: "trace_gc_heap_layout",
    comment: Some("print layout of pages in heap before and after gc (developer-only)"),
    flag_index: 549,
    canonical_index: 550,
  },
  EntryMeta {
    name: "trace_gc_heap_layout_ignore_minor_gc",
    comment: Some("do not print trace line before and after minor-gc"),
    flag_index: 550,
    canonical_index: 551,
  },
  EntryMeta {
    name: "trace_evacuation_candidates",
    comment: Some("Show statistics about the pages evacuation by the compaction (developer-only)"),
    flag_index: 551,
    canonical_index: 552,
  },
  EntryMeta {
    name: "trace_pending_allocations",
    comment: Some("trace calls to Heap::IsAllocationPending that return true (developer-only)"),
    flag_index: 552,
    canonical_index: 553,
  },
  EntryMeta {
    name: "trace_allocation_stack_interval",
    comment: Some("print stack trace after <n> free-list allocations"),
    flag_index: 553,
    canonical_index: 554,
  },
  EntryMeta {
    name: "trace_duplicate_threshold_kb",
    comment: Some("print duplicate objects in the heap if their size is more than given threshold"),
    flag_index: 554,
    canonical_index: 555,
  },
  EntryMeta {
    name: "trace_fragmentation",
    comment: Some("report fragmentation for old space (developer-only)"),
    flag_index: 555,
    canonical_index: 556,
  },
  EntryMeta {
    name: "trace_fragmentation_verbose",
    comment: Some("report fragmentation for old space (detailed) (developer-only)"),
    flag_index: 556,
    canonical_index: 557,
  },
  EntryMeta {
    name: "minor_ms_trace_fragmentation",
    comment: Some("trace fragmentation after marking (developer-only)"),
    flag_index: 557,
    canonical_index: 558,
  },
  EntryMeta {
    name: "trace_evacuation",
    comment: Some("report evacuation statistics (developer-only)"),
    flag_index: 558,
    canonical_index: 559,
  },
  EntryMeta {
    name: "trace_mutator_utilization",
    comment: Some("print mutator utilization, allocation speed, gc speed (developer-only)"),
    flag_index: 559,
    canonical_index: 560,
  },
  EntryMeta {
    name: "incremental_marking",
    comment: Some("use incremental marking"),
    flag_index: 560,
    canonical_index: 561,
  },
  EntryMeta {
    name: "incremental_marking_task",
    comment: Some("use tasks for incremental marking"),
    flag_index: 561,
    canonical_index: 562,
  },
  EntryMeta {
    name: "incremental_marking_soft_trigger",
    comment: Some(
      "threshold for starting incremental marking via a task in percent of available space: limit - size",
    ),
    flag_index: 562,
    canonical_index: 563,
  },
  EntryMeta {
    name: "incremental_marking_hard_trigger",
    comment: Some(
      "threshold for starting incremental marking immediately in percent of available space: limit - size",
    ),
    flag_index: 563,
    canonical_index: 564,
  },
  EntryMeta {
    name: "incremental_marking_unified_schedule",
    comment: Some(
      "Use a single schedule for determining a marking schedule between JS and C++ objects.",
    ),
    flag_index: 564,
    canonical_index: 565,
  },
  EntryMeta {
    name: "trace_unmapper",
    comment: Some("Trace the unmapping (developer-only)"),
    flag_index: 565,
    canonical_index: 566,
  },
  EntryMeta {
    name: "parallel_scavenge",
    comment: Some("parallel scavenge"),
    flag_index: 566,
    canonical_index: 567,
  },
  EntryMeta {
    name: "minor_gc_task",
    comment: Some("schedule minor GC tasks"),
    flag_index: 567,
    canonical_index: 568,
  },
  EntryMeta {
    name: "minor_gc_task_trigger",
    comment: Some("minor GC task trigger in percent of the current heap limit"),
    flag_index: 568,
    canonical_index: 569,
  },
  EntryMeta {
    name: "minor_gc_task_with_lower_priority",
    comment: Some("schedules the minor GC task with kUserVisible priority."),
    flag_index: 569,
    canonical_index: 570,
  },
  EntryMeta {
    name: "trace_parallel_scavenge",
    comment: Some("trace parallel scavenge (developer-only)"),
    flag_index: 570,
    canonical_index: 571,
  },
  EntryMeta {
    name: "cppgc_young_generation",
    comment: Some("run young generation garbage collections in Oilpan (experimental)"),
    flag_index: 571,
    canonical_index: 572,
  },
  EntryMeta {
    name: "optimize_gc_for_battery",
    comment: Some("optimize GC for battery"),
    flag_index: 572,
    canonical_index: 573,
  },
  EntryMeta {
    name: "concurrent_marking",
    comment: Some("use concurrent marking"),
    flag_index: 573,
    canonical_index: 574,
  },
  EntryMeta {
    name: "concurrent_marking_max_worker_num",
    comment: Some("max worker number of concurrent marking, 0 for NumberOfWorkerThreads"),
    flag_index: 574,
    canonical_index: 575,
  },
  EntryMeta {
    name: "concurrent_array_buffer_sweeping",
    comment: Some("concurrently sweep array buffers"),
    flag_index: 575,
    canonical_index: 576,
  },
  EntryMeta {
    name: "stress_concurrent_allocation",
    comment: Some("start background threads that allocate memory"),
    flag_index: 576,
    canonical_index: 577,
  },
  EntryMeta {
    name: "parallel_marking",
    comment: Some("use parallel marking in atomic pause"),
    flag_index: 577,
    canonical_index: 578,
  },
  EntryMeta {
    name: "ephemeron_fixpoint_iterations",
    comment: Some("number of fixpoint iterations it takes to switch to linear ephemeron algorithm"),
    flag_index: 578,
    canonical_index: 579,
  },
  EntryMeta {
    name: "trace_concurrent_marking",
    comment: Some("trace concurrent marking (developer-only)"),
    flag_index: 579,
    canonical_index: 580,
  },
  EntryMeta {
    name: "concurrent_sweeping",
    comment: Some("use concurrent sweeping"),
    flag_index: 580,
    canonical_index: 581,
  },
  EntryMeta {
    name: "parallel_compaction",
    comment: Some("use parallel compaction"),
    flag_index: 581,
    canonical_index: 582,
  },
  EntryMeta {
    name: "parallel_pointer_update",
    comment: Some("use parallel pointer update during compaction"),
    flag_index: 582,
    canonical_index: 583,
  },
  EntryMeta {
    name: "parallel_gc_clearing",
    comment: Some("use parallel threads to clear weak refs in the atomic pause."),
    flag_index: 583,
    canonical_index: 584,
  },
  EntryMeta {
    name: "detect_ineffective_gcs_near_heap_limit",
    comment: Some("trigger out-of-memory failure to avoid GC storm near heap limit"),
    flag_index: 584,
    canonical_index: 585,
  },
  EntryMeta {
    name: "enforce_global_heap_limit",
    comment: Some("Enforce global size by causing an out-of-memory failure above heap limit"),
    flag_index: 585,
    canonical_index: 586,
  },
  EntryMeta {
    name: "maximum_global_heap_limit_factor",
    comment: Some("Ratio from v8 to global maximum heap size"),
    flag_index: 586,
    canonical_index: 587,
  },
  EntryMeta {
    name: "ineffective_gc_size_threshold",
    comment: Some("Threshold on heap size to trigger out-of-memory failure near heap limit."),
    flag_index: 587,
    canonical_index: 588,
  },
  EntryMeta {
    name: "ineffective_gc_mutator_utilization_threshold",
    comment: Some(
      "Threshold on mutator utilization to trigger out-of-memory failure near heap limit.",
    ),
    flag_index: 588,
    canonical_index: 589,
  },
  EntryMeta {
    name: "trace_incremental_marking",
    comment: Some("trace progress of the incremental marking (developer-only)"),
    flag_index: 589,
    canonical_index: 590,
  },
  EntryMeta {
    name: "print_gc_clearing_dependency_graph",
    comment: Some("print clearing dependency graph in dot format (developer-only)"),
    flag_index: 590,
    canonical_index: 591,
  },
  EntryMeta {
    name: "trace_stress_marking",
    comment: Some("trace stress marking progress (developer-only)"),
    flag_index: 591,
    canonical_index: 592,
  },
  EntryMeta {
    name: "trace_stress_scavenge",
    comment: Some("trace stress scavenge progress (developer-only)"),
    flag_index: 592,
    canonical_index: 593,
  },
  EntryMeta {
    name: "track_gc_object_stats",
    comment: Some("track object counts and memory usage (developer-only)"),
    flag_index: 593,
    canonical_index: 594,
  },
  EntryMeta {
    name: "trace_gc_object_stats",
    comment: Some("trace object counts and memory usage (developer-only)"),
    flag_index: 594,
    canonical_index: 595,
  },
  EntryMeta {
    name: "trace_zone_stats",
    comment: Some("trace zone memory usage (developer-only)"),
    flag_index: 595,
    canonical_index: 596,
  },
  EntryMeta {
    name: "zone_stats_tolerance",
    comment: Some("report a tick only when allocated zone memory changes by this amount"),
    flag_index: 596,
    canonical_index: 597,
  },
  EntryMeta {
    name: "trace_zone_type_stats",
    comment: Some("trace per-type zone memory usage (developer-only)"),
    flag_index: 597,
    canonical_index: 598,
  },
  EntryMeta {
    name: "trace_backing_store",
    comment: Some("trace backing store events"),
    flag_index: 598,
    canonical_index: 599,
  },
  EntryMeta {
    name: "gc_stats",
    comment: Some("Used by tracing internally to enable gc statistics"),
    flag_index: 599,
    canonical_index: 600,
  },
  EntryMeta {
    name: "trace_gc_object_stats_all_objects",
    comment: Some("trace all objects on each gc (warning: large output) (developer-only)"),
    flag_index: 600,
    canonical_index: 601,
  },
  EntryMeta {
    name: "track_detached_contexts",
    comment: Some("track native contexts that are expected to be garbage collected"),
    flag_index: 601,
    canonical_index: 602,
  },
  EntryMeta {
    name: "trace_detached_contexts",
    comment: Some(
      "trace native contexts that are expected to be garbage collected (developer-only)",
    ),
    flag_index: 602,
    canonical_index: 603,
  },
  EntryMeta {
    name: "verify_heap",
    comment: Some("verify heap pointers before and after GC"),
    flag_index: 603,
    canonical_index: 604,
  },
  EntryMeta {
    name: "verify_write_barriers",
    comment: Some("verify skipped write barriers"),
    flag_index: 604,
    canonical_index: 605,
  },
  EntryMeta {
    name: "safepoint_bump_qos_class",
    comment: Some("Bump priority for running threads to reach safepoint"),
    flag_index: 605,
    canonical_index: 606,
  },
  EntryMeta {
    name: "freeze_forces_marking_finalization",
    comment: Some("Forces finalizing marking when frozen state is notified"),
    flag_index: 606,
    canonical_index: 607,
  },
  EntryMeta {
    name: "move_object_start",
    comment: Some("enable moving of object starts"),
    flag_index: 607,
    canonical_index: 608,
  },
  EntryMeta {
    name: "memory_reducer",
    comment: Some("use memory reducer"),
    flag_index: 608,
    canonical_index: 609,
  },
  EntryMeta {
    name: "memory_reducer_for_small_heaps",
    comment: Some("use memory reducer for small heaps"),
    flag_index: 609,
    canonical_index: 610,
  },
  EntryMeta {
    name: "memory_reducer_gc_count",
    comment: Some("Maximum number of memory reducer GCs scheduled"),
    flag_index: 610,
    canonical_index: 611,
  },
  EntryMeta {
    name: "memory_reducer_delay_ms",
    comment: Some("Delay before memory reducer start"),
    flag_index: 611,
    canonical_index: 612,
  },
  EntryMeta {
    name: "gc_memory_reducer_start_delay_ms",
    comment: Some("Delay before memory reducer start"),
    flag_index: 612,
    canonical_index: 613,
  },
  EntryMeta {
    name: "memory_reducer_limit_based",
    comment: Some(
      "Use the limit-based memory reducer that updates allocation limits after a delay to trigger GC",
    ),
    flag_index: 613,
    canonical_index: 614,
  },
  EntryMeta {
    name: "external_memory_max_growing_factor",
    comment: Some("This is the upper bound for growing factor imposed on external memory."),
    flag_index: 614,
    canonical_index: 615,
  },
  EntryMeta {
    name: "external_memory_max_reasonable_size",
    comment: Some(
      "Max external memory value (in GB) checked for a reasonable size, 0 to disable the check.",
    ),
    flag_index: 615,
    canonical_index: 616,
  },
  EntryMeta {
    name: "sqrt_allocation_limits",
    comment: Some(
      "Old generation and global heap limit growth scale based on the sqrt of estimated GC cost.",
    ),
    flag_index: 616,
    canonical_index: 617,
  },
  EntryMeta {
    name: "sqrt_allocation_limits_factor",
    comment: Some("When the sqrt heap limit is enabled, multiplier on the heap growth."),
    flag_index: 617,
    canonical_index: 618,
  },
  EntryMeta {
    name: "sqrt_allocation_limits_slow_factor",
    comment: Some(
      "When the sqrt heap limit is enabled, multiplier on the heap growth for HeapGrowingMode::kSlow.",
    ),
    flag_index: 618,
    canonical_index: 619,
  },
  EntryMeta {
    name: "sqrt_allocation_limits_minimal_factor",
    comment: Some(
      "When the sqrt heap limit is enabled, multiplier on the heap growth for HeapGrowingMode::kMinimal.",
    ),
    flag_index: 619,
    canonical_index: 620,
  },
  EntryMeta {
    name: "sqrt_allocation_limits_min_growing_factor",
    comment: Some("This is the lower bound for growing factor when using sqrt heap limit."),
    flag_index: 620,
    canonical_index: 621,
  },
  EntryMeta {
    name: "sqrt_allocation_limits_max_growing_factor",
    comment: Some("This is the upper bound for growing factor when using sqrt heap limit."),
    flag_index: 621,
    canonical_index: 622,
  },
  EntryMeta {
    name: "gc_speed_uses_counters",
    comment: Some("Old gen GC speed is computed directly from gc tracer counters."),
    flag_index: 622,
    canonical_index: 623,
  },
  EntryMeta {
    name: "heap_growing_percent",
    comment: Some("specifies heap growing factor as (1 + heap_growing_percent/100)"),
    flag_index: 623,
    canonical_index: 624,
  },
  EntryMeta {
    name: "v8_os_page_size",
    comment: Some("override OS page size (in KBytes)"),
    flag_index: 624,
    canonical_index: 625,
  },
  EntryMeta {
    name: "allocation_buffer_parking",
    comment: Some("allocation buffer parking"),
    flag_index: 625,
    canonical_index: 626,
  },
  EntryMeta {
    name: "compact",
    comment: Some("Perform compaction on full GCs based on V8's default heuristics"),
    flag_index: 626,
    canonical_index: 627,
  },
  EntryMeta {
    name: "compact_code_space",
    comment: Some("Perform code space compaction on full collections."),
    flag_index: 627,
    canonical_index: 628,
  },
  EntryMeta {
    name: "compact_on_every_full_gc",
    comment: Some("Perform compaction on every full GC"),
    flag_index: 628,
    canonical_index: 629,
  },
  EntryMeta {
    name: "compaction_on_regular_gcs",
    comment: Some("Perform compaction on regular GCs"),
    flag_index: 629,
    canonical_index: 630,
  },
  EntryMeta {
    name: "compact_with_stack",
    comment: Some("Perform compaction when finalizing a full GC with stack"),
    flag_index: 630,
    canonical_index: 631,
  },
  EntryMeta {
    name: "compaction_max_evacuated_bytes_mb",
    comment: Some("Max evacuated bytes during compaction"),
    flag_index: 631,
    canonical_index: 632,
  },
  EntryMeta {
    name: "compaction_max_evacuated_bytes_mb_for_reduce_memory",
    comment: Some("Max evacuated bytes during compaction for reduce memory GCs"),
    flag_index: 632,
    canonical_index: 633,
  },
  EntryMeta {
    name: "compaction_max_evacuated_bytes_mb_for_optimize_memory",
    comment: Some(
      "Max evacuated bytes during compaction for GCs when ShouldOptimizeForMemoryUsage() = true",
    ),
    flag_index: 633,
    canonical_index: 634,
  },
  EntryMeta {
    name: "compaction_target_fragmentation_percent_for_reduce_memory",
    comment: Some("Target fragmentation % during compaction for reduce memory GCs"),
    flag_index: 634,
    canonical_index: 635,
  },
  EntryMeta {
    name: "compaction_target_fragmentation_percent_for_optimize_memory",
    comment: Some(
      "Target fragmentation % during compaction for GCs when ShouldOptimizeForMemoryUsage() = true",
    ),
    flag_index: 635,
    canonical_index: 636,
  },
  EntryMeta {
    name: "shortcut_strings_with_stack",
    comment: Some("Shortcut Strings during GC with stack"),
    flag_index: 636,
    canonical_index: 637,
  },
  EntryMeta {
    name: "stress_compaction",
    comment: Some("Stress GC compaction to flush out bugs with moving objects"),
    flag_index: 637,
    canonical_index: 638,
  },
  EntryMeta {
    name: "resize_large_object",
    comment: Some("Support resizing of large objects"),
    flag_index: 638,
    canonical_index: 639,
  },
  EntryMeta {
    name: "stress_compaction_random",
    comment: Some(
      "Stress GC compaction by selecting random percent of pages as evacuation candidates. Overrides stress_compaction.",
    ),
    flag_index: 639,
    canonical_index: 640,
  },
  EntryMeta {
    name: "stress_descriptor_array_trimming",
    comment: Some("Stress descriptor array trimming on every stackless GC."),
    flag_index: 640,
    canonical_index: 641,
  },
  EntryMeta {
    name: "flush_baseline_code",
    comment: Some("flush of baseline code when it has not been executed recently"),
    flag_index: 641,
    canonical_index: 642,
  },
  EntryMeta {
    name: "flush_bytecode",
    comment: Some("flush of bytecode when it has not been executed recently"),
    flag_index: 642,
    canonical_index: 643,
  },
  EntryMeta {
    name: "bytecode_old_time",
    comment: Some("number of seconds before we flush code"),
    flag_index: 643,
    canonical_index: 644,
  },
  EntryMeta {
    name: "stress_flush_code",
    comment: Some("stress code flushing"),
    flag_index: 644,
    canonical_index: 645,
  },
  EntryMeta {
    name: "trace_flush_code",
    comment: Some("trace bytecode flushing (developer-only)"),
    flag_index: 645,
    canonical_index: 646,
  },
  EntryMeta {
    name: "use_marking_progress_bar",
    comment: Some(
      "Use a progress bar to scan large objects in increments when incremental marking is active.",
    ),
    flag_index: 646,
    canonical_index: 647,
  },
  EntryMeta {
    name: "fixed_margin_for_input_handling",
    comment: Some(
      "how much we are allowed to overshoot the heap limit by during input mode, in MiB.",
    ),
    flag_index: 647,
    canonical_index: 648,
  },
  EntryMeta {
    name: "optimize_for_input_handling",
    comment: Some("whether to use input hints to optimize for responsiveness."),
    flag_index: 648,
    canonical_index: 649,
  },
  EntryMeta {
    name: "stress_per_context_marking_worklist",
    comment: Some("Use per-context worklist for marking"),
    flag_index: 649,
    canonical_index: 650,
  },
  EntryMeta {
    name: "stress_incremental_marking",
    comment: Some("force incremental marking for small heaps and run it more often"),
    flag_index: 650,
    canonical_index: 651,
  },
  EntryMeta {
    name: "memory_pool",
    comment: Some("Pool memory globally before returning it to the operating system"),
    flag_index: 651,
    canonical_index: 652,
  },
  EntryMeta {
    name: "memory_pool_share_memory_on_teardown",
    comment: Some("Share memory on Isolate teardown using other Isolate's task runners"),
    flag_index: 652,
    canonical_index: 653,
  },
  EntryMeta {
    name: "memory_pool_release_on_malloc_failures",
    comment: Some("discard the memory pool on malloc retries"),
    flag_index: 653,
    canonical_index: 654,
  },
  EntryMeta {
    name: "memory_pool_timeout",
    comment: Some("Release pooled pages after X seconds."),
    flag_index: 654,
    canonical_index: 655,
  },
  EntryMeta {
    name: "large_page_pool",
    comment: Some("Add large pages to the page pool"),
    flag_index: 655,
    canonical_index: 656,
  },
  EntryMeta {
    name: "max_large_page_pool_size",
    comment: Some("Maximum size of pooled large pages in MB."),
    flag_index: 656,
    canonical_index: 657,
  },
  EntryMeta {
    name: "managed_zone_memory",
    comment: Some("Manage zone memory in V8 instead of using malloc()."),
    flag_index: 657,
    canonical_index: 658,
  },
  EntryMeta {
    name: "managed_zone_memory_for_isolate_independent_memory",
    comment: Some("Pool zone memory for isolate independent memory (nullptr-isolate) (e.g. Wasm)."),
    flag_index: 658,
    canonical_index: 659,
  },
  EntryMeta {
    name: "fuzzer_gc_analysis",
    comment: Some(
      "prints number of allocations and enables analysis mode for gc fuzz testing, e.g. --stress-marking, --stress-scavenge",
    ),
    flag_index: 659,
    canonical_index: 660,
  },
  EntryMeta {
    name: "stress_marking",
    comment: Some(
      "force marking at random points between 0 and X (inclusive) percent of the regular marking start limit",
    ),
    flag_index: 660,
    canonical_index: 661,
  },
  EntryMeta {
    name: "stress_scavenge",
    comment: Some(
      "force scavenge at random points between 0 and X (inclusive) percent of the new space capacity",
    ),
    flag_index: 661,
    canonical_index: 662,
  },
  EntryMeta {
    name: "reclaim_unmodified_wrappers",
    comment: Some("reclaim otherwise unreachable unmodified wrapper objects when possible"),
    flag_index: 662,
    canonical_index: 663,
  },
  EntryMeta {
    name: "parallel_reclaim_unmodified_wrappers",
    comment: Some("reclaim wrapper objects in parallel"),
    flag_index: 663,
    canonical_index: 664,
  },
  EntryMeta {
    name: "concurrent_marking_high_priority_threads",
    comment: Some("use high priority threads for concurrent Marking"),
    flag_index: 664,
    canonical_index: 665,
  },
  EntryMeta {
    name: "disable_abortjs",
    comment: Some("disables AbortJS runtime function"),
    flag_index: 665,
    canonical_index: 666,
  },
  EntryMeta {
    name: "randomize_all_allocations",
    comment: Some(
      "randomize virtual memory reservations by ignoring any hints passed when allocating pages",
    ),
    flag_index: 666,
    canonical_index: 667,
  },
  EntryMeta {
    name: "manual_evacuation_candidates_selection",
    comment: Some(
      "Test mode only flag. It allows an unit test to select evacuation candidates pages (requires --stress_compaction).",
    ),
    flag_index: 667,
    canonical_index: 668,
  },
  EntryMeta {
    name: "clear_free_memory",
    comment: Some("initialize free memory with 0"),
    flag_index: 668,
    canonical_index: 669,
  },
  EntryMeta {
    name: "idle_gc_on_context_disposal",
    comment: Some("idle gc on context disposal"),
    flag_index: 669,
    canonical_index: 670,
  },
  EntryMeta {
    name: "trace_context_disposal",
    comment: Some("trace context disposal (developer-only)"),
    flag_index: 670,
    canonical_index: 671,
  },
  EntryMeta {
    name: "cppheap_incremental_marking",
    comment: Some("use incremental marking for CppHeap"),
    flag_index: 671,
    canonical_index: 672,
  },
  EntryMeta {
    name: "cppheap_concurrent_marking",
    comment: Some("use concurrent marking for CppHeap"),
    flag_index: 672,
    canonical_index: 673,
  },
  EntryMeta {
    name: "memory_balancer",
    comment: Some("use membalancer, a new heap limit balancing algorithm"),
    flag_index: 673,
    canonical_index: 674,
  },
  EntryMeta {
    name: "memory_balancer_c_value",
    comment: Some(
      "c value for membalancer. A special constant to balance between memory and space tradeoff. The smaller the more memory it uses.",
    ),
    flag_index: 674,
    canonical_index: 675,
  },
  EntryMeta {
    name: "trace_memory_balancer",
    comment: Some("print memory balancer behavior. (developer-only)"),
    flag_index: 675,
    canonical_index: 676,
  },
  EntryMeta {
    name: "late_heap_limit_check",
    comment: Some("skips heap limit check for early GCs on allocation failure."),
    flag_index: 676,
    canonical_index: 677,
  },
  EntryMeta {
    name: "debug_code",
    comment: None,
    flag_index: 677,
    canonical_index: 678,
  },
  EntryMeta {
    name: "trap_on_abort",
    comment: None,
    flag_index: 678,
    canonical_index: 679,
  },
  EntryMeta {
    name: "slow_debug_code",
    comment: None,
    flag_index: 679,
    canonical_index: 680,
  },
  EntryMeta {
    name: "code_comments",
    comment: None,
    flag_index: 680,
    canonical_index: 681,
  },
  EntryMeta {
    name: "enable_sse3",
    comment: Some("enable use of SSE3 instructions if available"),
    flag_index: 681,
    canonical_index: 682,
  },
  EntryMeta {
    name: "enable_ssse3",
    comment: Some("enable use of SSSE3 instructions if available"),
    flag_index: 682,
    canonical_index: 683,
  },
  EntryMeta {
    name: "enable_sse4_1",
    comment: Some("enable use of SSE4.1 instructions if available"),
    flag_index: 683,
    canonical_index: 684,
  },
  EntryMeta {
    name: "enable_sse4_2",
    comment: Some("enable use of SSE4.2 instructions if available"),
    flag_index: 684,
    canonical_index: 685,
  },
  EntryMeta {
    name: "enable_sahf",
    comment: Some("enable use of SAHF instruction if available (X64 only)"),
    flag_index: 685,
    canonical_index: 686,
  },
  EntryMeta {
    name: "enable_avx",
    comment: Some("enable use of AVX instructions if available"),
    flag_index: 686,
    canonical_index: 687,
  },
  EntryMeta {
    name: "enable_avx2",
    comment: Some("enable use of AVX2 instructions if available"),
    flag_index: 687,
    canonical_index: 688,
  },
  EntryMeta {
    name: "enable_avx_vnni",
    comment: Some("enable use of AVX-VNNI instructions if available"),
    flag_index: 688,
    canonical_index: 689,
  },
  EntryMeta {
    name: "enable_avx_vnni_int8",
    comment: Some("enable use of AVX-VNNI-INT8 instructions if available"),
    flag_index: 689,
    canonical_index: 690,
  },
  EntryMeta {
    name: "enable_fma3",
    comment: Some("enable use of FMA3 instructions if available"),
    flag_index: 690,
    canonical_index: 691,
  },
  EntryMeta {
    name: "enable_f16c",
    comment: Some("enable use of F16C instructions if available"),
    flag_index: 691,
    canonical_index: 692,
  },
  EntryMeta {
    name: "enable_bmi1",
    comment: Some("enable use of BMI1 instructions if available"),
    flag_index: 692,
    canonical_index: 693,
  },
  EntryMeta {
    name: "enable_bmi2",
    comment: Some("enable use of BMI2 instructions if available"),
    flag_index: 693,
    canonical_index: 694,
  },
  EntryMeta {
    name: "enable_lzcnt",
    comment: Some("enable use of LZCNT instruction if available"),
    flag_index: 694,
    canonical_index: 695,
  },
  EntryMeta {
    name: "enable_popcnt",
    comment: Some("enable use of POPCNT instruction if available"),
    flag_index: 695,
    canonical_index: 696,
  },
  EntryMeta {
    name: "arm_arch",
    comment: Some(
      "generate instructions for the selected ARM architecture if available: armv6, armv7, armv7+sudiv or armv8",
    ),
    flag_index: 696,
    canonical_index: 697,
  },
  EntryMeta {
    name: "force_long_branches",
    comment: Some("force all emitted branches to be in long mode (MIPS/PPC only)"),
    flag_index: 697,
    canonical_index: 698,
  },
  EntryMeta {
    name: "mcpu",
    comment: Some("enable optimization for specific cpu"),
    flag_index: 698,
    canonical_index: 699,
  },
  EntryMeta {
    name: "partial_constant_pool",
    comment: Some("enable use of partial constant pools (x64 only)"),
    flag_index: 699,
    canonical_index: 700,
  },
  EntryMeta {
    name: "sim_arm64_optional_features",
    comment: Some("enable optional features on the simulator for testing: none or all"),
    flag_index: 700,
    canonical_index: 701,
  },
  EntryMeta {
    name: "sim_arm64_tbi",
    comment: Some("simulate arm64 Top Byte Ignore (TBI) for data accesses"),
    flag_index: 701,
    canonical_index: 702,
  },
  EntryMeta {
    name: "intel_jcc_erratum_mitigation",
    comment: Some("enable mitigation for Intel JCC erratum on affected CPUs"),
    flag_index: 702,
    canonical_index: 703,
  },
  EntryMeta {
    name: "enable_source_at_csa_bind",
    comment: Some("Include source information in the binary at CSA bind locations."),
    flag_index: 703,
    canonical_index: 704,
  },
  EntryMeta {
    name: "enable_armv7",
    comment: Some("deprecated (use --arm_arch instead)"),
    flag_index: 704,
    canonical_index: 705,
  },
  EntryMeta {
    name: "enable_vfp3",
    comment: Some("deprecated (use --arm_arch instead)"),
    flag_index: 705,
    canonical_index: 706,
  },
  EntryMeta {
    name: "enable_32dregs",
    comment: Some("deprecated (use --arm_arch instead)"),
    flag_index: 706,
    canonical_index: 707,
  },
  EntryMeta {
    name: "enable_neon",
    comment: Some("deprecated (use --arm_arch instead)"),
    flag_index: 707,
    canonical_index: 708,
  },
  EntryMeta {
    name: "enable_sudiv",
    comment: Some("deprecated (use --arm_arch instead)"),
    flag_index: 708,
    canonical_index: 709,
  },
  EntryMeta {
    name: "enable_armv8",
    comment: Some("deprecated (use --arm_arch instead)"),
    flag_index: 709,
    canonical_index: 710,
  },
  EntryMeta {
    name: "enable_regexp_unaligned_accesses",
    comment: Some("enable unaligned accesses for the regexp engine"),
    flag_index: 710,
    canonical_index: 711,
  },
  EntryMeta {
    name: "script_streaming",
    comment: Some("enable parsing on background"),
    flag_index: 711,
    canonical_index: 712,
  },
  EntryMeta {
    name: "stress_background_compile",
    comment: Some("stress test parsing on background"),
    flag_index: 712,
    canonical_index: 713,
  },
  EntryMeta {
    name: "concurrent_cache_deserialization",
    comment: Some("enable deserializing code caches on background"),
    flag_index: 713,
    canonical_index: 714,
  },
  EntryMeta {
    name: "merge_background_deserialized_script_with_compilation_cache",
    comment: Some(
      "After deserializing code cache data on a background thread, merge it into an existing Script if one is found in the Isolate compilation cache",
    ),
    flag_index: 714,
    canonical_index: 715,
  },
  EntryMeta {
    name: "verify_code_merge",
    comment: Some("Verify scope infos after merge"),
    flag_index: 715,
    canonical_index: 716,
  },
  EntryMeta {
    name: "expose_gc",
    comment: Some("expose gc extension"),
    flag_index: 716,
    canonical_index: 717,
  },
  EntryMeta {
    name: "expose_gc_as",
    comment: Some("expose gc extension under the specified name"),
    flag_index: 717,
    canonical_index: 718,
  },
  EntryMeta {
    name: "expose_externalize_string",
    comment: Some("expose externalize string extension"),
    flag_index: 718,
    canonical_index: 719,
  },
  EntryMeta {
    name: "expose_statistics",
    comment: Some("expose statistics extension"),
    flag_index: 719,
    canonical_index: 720,
  },
  EntryMeta {
    name: "expose_trigger_failure",
    comment: Some("expose trigger-failure extension"),
    flag_index: 720,
    canonical_index: 721,
  },
  EntryMeta {
    name: "stack_trace_limit",
    comment: Some("number of stack frames to capture"),
    flag_index: 721,
    canonical_index: 722,
  },
  EntryMeta {
    name: "builtins_in_stack_traces",
    comment: Some("show built-in functions in stack traces"),
    flag_index: 722,
    canonical_index: 723,
  },
  EntryMeta {
    name: "experimental_stack_trace_frames",
    comment: Some("enable experimental frames (API/Builtins) and stack trace layout"),
    flag_index: 723,
    canonical_index: 724,
  },
  EntryMeta {
    name: "disallow_code_generation_from_strings",
    comment: Some("disallow eval and friends"),
    flag_index: 724,
    canonical_index: 725,
  },
  EntryMeta {
    name: "expose_async_hooks",
    comment: Some("expose async_hooks object"),
    flag_index: 725,
    canonical_index: 726,
  },
  EntryMeta {
    name: "expose_cputracemark_as",
    comment: Some("expose cputracemark extension under the specified name"),
    flag_index: 726,
    canonical_index: 727,
  },
  EntryMeta {
    name: "experimental_report_exceptions_from_callbacks",
    comment: Some("Notify Api callback about exceptions thrown in Api callbacks"),
    flag_index: 727,
    canonical_index: 728,
  },
  EntryMeta {
    name: "allow_unsafe_function_constructor",
    comment: Some("allow invoking the function constructor without security checks"),
    flag_index: 728,
    canonical_index: 729,
  },
  EntryMeta {
    name: "force_slow_path",
    comment: Some("always take the slow path for builtins"),
    flag_index: 729,
    canonical_index: 730,
  },
  EntryMeta {
    name: "test_small_max_function_context_stub_size",
    comment: Some(
      "enable testing the function context size overflow path by making the maximum size smaller",
    ),
    flag_index: 730,
    canonical_index: 731,
  },
  EntryMeta {
    name: "fast_api_indexof",
    comment: Some("enable using indexOf Api callbacks"),
    flag_index: 731,
    canonical_index: 732,
  },
  EntryMeta {
    name: "fast_api_iterable_to_list",
    comment: Some("enable fast path for IterableToList for indexed interceptors"),
    flag_index: 732,
    canonical_index: 733,
  },
  EntryMeta {
    name: "inline_new",
    comment: Some("use fast inline allocation"),
    flag_index: 733,
    canonical_index: 734,
  },
  EntryMeta {
    name: "switch_table_spread_threshold",
    comment: Some(
      "allow the jump table used for switch statements to span a range of integers roughly equal to this number times the number of clauses in the switch",
    ),
    flag_index: 734,
    canonical_index: 735,
  },
  EntryMeta {
    name: "switch_table_min_cases",
    comment: Some(
      "the number of Smi integer cases present in the switch statement before using the jump table optimization",
    ),
    flag_index: 735,
    canonical_index: 736,
  },
  EntryMeta {
    name: "stress_lazy_compilation",
    comment: Some(
      "stress lazy compilation by simulating stack overflow during unoptimized bytecode generation with 1/n-th probability, do nothing on 0",
    ),
    flag_index: 736,
    canonical_index: 737,
  },
  EntryMeta {
    name: "trace",
    comment: Some("trace javascript function calls"),
    flag_index: 737,
    canonical_index: 738,
  },
  EntryMeta {
    name: "lazy",
    comment: Some("use lazy compilation"),
    flag_index: 738,
    canonical_index: 739,
  },
  EntryMeta {
    name: "lazy_eval",
    comment: Some("use lazy compilation during eval"),
    flag_index: 739,
    canonical_index: 740,
  },
  EntryMeta {
    name: "lazy_streaming",
    comment: Some("use lazy compilation during streaming compilation"),
    flag_index: 740,
    canonical_index: 741,
  },
  EntryMeta {
    name: "max_lazy",
    comment: Some("ignore eager compilation hints"),
    flag_index: 741,
    canonical_index: 742,
  },
  EntryMeta {
    name: "compile_hints_magic",
    comment: Some("enable magic compile hints comments"),
    flag_index: 742,
    canonical_index: 743,
  },
  EntryMeta {
    name: "trace_opt",
    comment: Some("trace optimized compilation (developer-only)"),
    flag_index: 743,
    canonical_index: 744,
  },
  EntryMeta {
    name: "trace_opt_status",
    comment: Some(
      "trace the optimization status of functions during tiering events (developer-only)",
    ),
    flag_index: 744,
    canonical_index: 745,
  },
  EntryMeta {
    name: "trace_opt_verbose",
    comment: Some("extra verbose optimized compilation tracing (developer-only)"),
    flag_index: 745,
    canonical_index: 746,
  },
  EntryMeta {
    name: "trace_opt_stats",
    comment: Some("trace optimized compilation statistics (developer-only)"),
    flag_index: 746,
    canonical_index: 747,
  },
  EntryMeta {
    name: "trace_deopt",
    comment: Some("trace deoptimization (developer-only)"),
    flag_index: 747,
    canonical_index: 748,
  },
  EntryMeta {
    name: "log_deopt",
    comment: Some("log deoptimization (developer-only)"),
    flag_index: 748,
    canonical_index: 749,
  },
  EntryMeta {
    name: "trace_deopt_verbose",
    comment: Some("extra verbose deoptimization tracing (developer-only)"),
    flag_index: 749,
    canonical_index: 750,
  },
  EntryMeta {
    name: "trace_file_names",
    comment: Some("include file names in trace-opt/trace-deopt output (developer-only)"),
    flag_index: 750,
    canonical_index: 751,
  },
  EntryMeta {
    name: "always_osr",
    comment: Some("always try to OSR functions"),
    flag_index: 751,
    canonical_index: 752,
  },
  EntryMeta {
    name: "trace_serializer",
    comment: Some("print code serializer trace (developer-only)"),
    flag_index: 752,
    canonical_index: 753,
  },
  EntryMeta {
    name: "compilation_cache",
    comment: Some("enable compilation cache"),
    flag_index: 753,
    canonical_index: 754,
  },
  EntryMeta {
    name: "cache_prototype_transitions",
    comment: Some("cache prototype transitions"),
    flag_index: 754,
    canonical_index: 755,
  },
  EntryMeta {
    name: "lazy_compile_dispatcher",
    comment: Some("enable compiler dispatcher (experimental)"),
    flag_index: 755,
    canonical_index: 756,
  },
  EntryMeta {
    name: "lazy_compile_dispatcher_max_threads",
    comment: Some("max threads for compiler dispatcher (0 for unbounded)"),
    flag_index: 756,
    canonical_index: 757,
  },
  EntryMeta {
    name: "trace_compiler_dispatcher",
    comment: Some("trace compiler dispatcher activity (developer-only)"),
    flag_index: 757,
    canonical_index: 758,
  },
  EntryMeta {
    name: "parallel_compile_tasks_for_eager_toplevel",
    comment: Some(
      "spawn parallel compile tasks for eagerly compiled, top-level functions (experimental)",
    ),
    flag_index: 758,
    canonical_index: 759,
  },
  EntryMeta {
    name: "parallel_compile_tasks_for_lazy",
    comment: Some("spawn parallel compile tasks for all lazily compiled functions (experimental)"),
    flag_index: 759,
    canonical_index: 760,
  },
  EntryMeta {
    name: "cpu_profiler_sampling_interval",
    comment: Some("CPU profiler sampling interval in microseconds"),
    flag_index: 760,
    canonical_index: 761,
  },
  EntryMeta {
    name: "simulator_debugger",
    comment: Some("Start a debugger when a simulator hits a breakpoint"),
    flag_index: 761,
    canonical_index: 762,
  },
  EntryMeta {
    name: "trace_side_effect_free_debug_evaluate",
    comment: Some("print debug messages for side-effect-free debug-evaluate for testing"),
    flag_index: 762,
    canonical_index: 763,
  },
  EntryMeta {
    name: "hard_abort",
    comment: Some("abort by crashing"),
    flag_index: 763,
    canonical_index: 764,
  },
  EntryMeta {
    name: "log_colour",
    comment: Some("When logging, try to use coloured output."),
    flag_index: 764,
    canonical_index: 765,
  },
  EntryMeta {
    name: "expose_inspector_scripts",
    comment: Some("expose injected-script-source.js for debugging"),
    flag_index: 765,
    canonical_index: 766,
  },
  EntryMeta {
    name: "stack_size",
    comment: Some("default size of stack region v8 is allowed to use (in kBytes)"),
    flag_index: 766,
    canonical_index: 767,
  },
  EntryMeta {
    name: "max_stack_trace_source_length",
    comment: Some("maximum length of function source code printed in a stack trace."),
    flag_index: 767,
    canonical_index: 768,
  },
  EntryMeta {
    name: "clear_exceptions_on_js_entry",
    comment: Some("clear exceptions when entering JavaScript"),
    flag_index: 768,
    canonical_index: 769,
  },
  EntryMeta {
    name: "use_original_message_for_stack_trace",
    comment: Some(
      "use the message with which the Error constructor was called rather than the value of the \"message\" property for Error.stack",
    ),
    flag_index: 769,
    canonical_index: 770,
  },
  EntryMeta {
    name: "histogram_interval",
    comment: Some("time interval in ms for aggregating memory histograms"),
    flag_index: 770,
    canonical_index: 771,
  },
  EntryMeta {
    name: "heap_profiler_trace_objects",
    comment: Some("Dump heap object allocations/movements/size_updates"),
    flag_index: 771,
    canonical_index: 772,
  },
  EntryMeta {
    name: "heap_profiler_use_embedder_graph",
    comment: Some("Use the new EmbedderGraph API to get embedder nodes"),
    flag_index: 772,
    canonical_index: 773,
  },
  EntryMeta {
    name: "heap_snapshot_on_oom",
    comment: Some("Write a heap snapshot to disk on last-resort GCs"),
    flag_index: 773,
    canonical_index: 774,
  },
  EntryMeta {
    name: "heap_snapshot_path",
    comment: Some(
      "Directory to write heap snapshots to. (If not set, the snapshot is written to the current working directory.)",
    ),
    flag_index: 774,
    canonical_index: 775,
  },
  EntryMeta {
    name: "heap_snapshot_on_gc",
    comment: Some(
      "Write a heap snapshot to disk on a certain GC invocation. 0 results in taking a snapshot on every invocation.",
    ),
    flag_index: 775,
    canonical_index: 776,
  },
  EntryMeta {
    name: "heap_snapshot_string_limit",
    comment: Some("truncate strings to this length in the heap snapshot"),
    flag_index: 776,
    canonical_index: 777,
  },
  EntryMeta {
    name: "profile_heap_snapshot",
    comment: Some("dump time spent on heap snapshot"),
    flag_index: 777,
    canonical_index: 778,
  },
  EntryMeta {
    name: "sampling_heap_profiler_suppress_randomness",
    comment: Some("Use constant sample intervals to eliminate test flakiness"),
    flag_index: 778,
    canonical_index: 779,
  },
  EntryMeta {
    name: "log_ic",
    comment: Some("Log inline cache state transitions for tools/ic-processor"),
    flag_index: 779,
    canonical_index: 780,
  },
  EntryMeta {
    name: "fast_map_update",
    comment: Some("enable fast map update by caching the migration target"),
    flag_index: 780,
    canonical_index: 781,
  },
  EntryMeta {
    name: "max_valid_polymorphic_map_count",
    comment: Some("maximum number of valid maps to track in POLYMORPHIC state"),
    flag_index: 781,
    canonical_index: 782,
  },
  EntryMeta {
    name: "clone_object_sidestep_transitions",
    comment: Some("support sidestep transitions for dependency tracking object clone maps"),
    flag_index: 782,
    canonical_index: 783,
  },
  EntryMeta {
    name: "fast_properties_soft_limit",
    comment: Some(
      "limits the number of properties that can be added to an object using keyed store before transitioning to dictionary mode",
    ),
    flag_index: 783,
    canonical_index: 784,
  },
  EntryMeta {
    name: "native_code_counters",
    comment: Some("generate extra code for manipulating stats counters"),
    flag_index: 784,
    canonical_index: 785,
  },
  EntryMeta {
    name: "sparkplug_plus",
    comment: Some("enable dynamic patching on JS baseline code"),
    flag_index: 785,
    canonical_index: 786,
  },
  EntryMeta {
    name: "sparkplug_inline_smi",
    comment: Some("inline the Smi fast path of embedded feedback operations into baseline code"),
    flag_index: 786,
    canonical_index: 787,
  },
  EntryMeta {
    name: "super_ic",
    comment: Some("use an IC for super property loads"),
    flag_index: 787,
    canonical_index: 788,
  },
  EntryMeta {
    name: "mega_dom_ic",
    comment: Some("use MegaDOM IC state for API objects (experimental)"),
    flag_index: 788,
    canonical_index: 789,
  },
  EntryMeta {
    name: "homomorphic_ic_count",
    comment: Some("local cache size in homomorphic ICs"),
    flag_index: 789,
    canonical_index: 790,
  },
  EntryMeta {
    name: "trace_prototype_users",
    comment: Some("Trace updates to prototype user tracking (developer-only)"),
    flag_index: 790,
    canonical_index: 791,
  },
  EntryMeta {
    name: "trace_for_in_enumerate",
    comment: Some("Trace for-in enumerate slow-paths (developer-only)"),
    flag_index: 791,
    canonical_index: 792,
  },
  EntryMeta {
    name: "log_maps",
    comment: Some("Log map creation"),
    flag_index: 792,
    canonical_index: 793,
  },
  EntryMeta {
    name: "log_maps_details",
    comment: Some("Also log map details"),
    flag_index: 793,
    canonical_index: 794,
  },
  EntryMeta {
    name: "allow_natives_syntax",
    comment: Some("allow natives syntax"),
    flag_index: 794,
    canonical_index: 795,
  },
  EntryMeta {
    name: "allow_natives_for_differential_fuzzing",
    comment: Some("allow only natives explicitly allowlisted for differential fuzzers"),
    flag_index: 795,
    canonical_index: 796,
  },
  EntryMeta {
    name: "parse_only",
    comment: Some("only parse the sources"),
    flag_index: 796,
    canonical_index: 797,
  },
  EntryMeta {
    name: "sim_abort_on_bad_auth",
    comment: Some("Stop execution when a pointer authentication fails in the ARM64 simulator."),
    flag_index: 797,
    canonical_index: 798,
  },
  EntryMeta {
    name: "async_stack_traces",
    comment: Some("include async stack traces in Error.stack"),
    flag_index: 798,
    canonical_index: 799,
  },
  EntryMeta {
    name: "stack_trace_on_illegal",
    comment: Some("print stack trace when an illegal exception is thrown"),
    flag_index: 799,
    canonical_index: 800,
  },
  EntryMeta {
    name: "abort_on_uncaught_exception",
    comment: Some("abort program (dump core) when an uncaught exception is thrown"),
    flag_index: 800,
    canonical_index: 801,
  },
  EntryMeta {
    name: "correctness_fuzzer_suppressions",
    comment: Some(
      "Suppress certain unspecified behaviors to ease correctness fuzzing: Abort program when the stack overflows or a string exceeds maximum length (as opposed to throwing RangeError). Use a fixed suppression string for error messages. (developer-only)",
    ),
    flag_index: 801,
    canonical_index: 802,
  },
  EntryMeta {
    name: "correctness_fuzzer_cross_arch_suppressions",
    comment: Some(
      "Minimize cross-architecture differences for correctness fuzzing. (developer-only)",
    ),
    flag_index: 802,
    canonical_index: 803,
  },
  EntryMeta {
    name: "rehash_snapshot",
    comment: Some("rehash strings from the snapshot to override the baked-in seed"),
    flag_index: 803,
    canonical_index: 804,
  },
  EntryMeta {
    name: "hash_seed",
    comment: Some(
      "Fixed seed to use to hash property keys (0 means random)(with snapshots this option cannot override the baked-in seed)",
    ),
    flag_index: 804,
    canonical_index: 805,
  },
  EntryMeta {
    name: "random_seed",
    comment: Some(
      "Default seed for initializing random generator (0, the default, means to use system random).",
    ),
    flag_index: 805,
    canonical_index: 806,
  },
  EntryMeta {
    name: "fuzzer_random_seed",
    comment: Some(
      "Default seed for initializing fuzzer random generator (0, the default, means to use v8's random number generator seed).",
    ),
    flag_index: 806,
    canonical_index: 807,
  },
  EntryMeta {
    name: "trace_rail",
    comment: Some("trace RAIL mode (developer-only)"),
    flag_index: 807,
    canonical_index: 808,
  },
  EntryMeta {
    name: "print_all_exceptions",
    comment: Some(
      "print exception object and stack trace on each thrown exception (developer-only)",
    ),
    flag_index: 808,
    canonical_index: 809,
  },
  EntryMeta {
    name: "adjust_os_scheduling_parameters",
    comment: Some("adjust OS specific scheduling params for the isolate"),
    flag_index: 809,
    canonical_index: 810,
  },
  EntryMeta {
    name: "experimental_flush_embedded_blob_icache",
    comment: Some("Used in an experiment to evaluate icache flushing on certain CPUs"),
    flag_index: 810,
    canonical_index: 811,
  },
  EntryMeta {
    name: "short_builtin_calls",
    comment: Some(
      "Put embedded builtins code into the code range for shorter builtin calls/jumps if system has >=4GB memory",
    ),
    flag_index: 811,
    canonical_index: 812,
  },
  EntryMeta {
    name: "trace_code_range_allocation",
    comment: Some("Trace code range allocation process. (developer-only)"),
    flag_index: 812,
    canonical_index: 813,
  },
  EntryMeta {
    name: "better_code_range_allocation",
    comment: Some(
      "This mode tries harder to allocate code range near .text section. Works only for configurations with external code space and shared pointer compression cage.",
    ),
    flag_index: 813,
    canonical_index: 814,
  },
  EntryMeta {
    name: "abort_on_far_code_range",
    comment: Some("Abort if code range is allocated further away than 4GB from the.text section"),
    flag_index: 814,
    canonical_index: 815,
  },
  EntryMeta {
    name: "runtime_call_stats",
    comment: Some("report runtime call counts and times"),
    flag_index: 815,
    canonical_index: 816,
  },
  EntryMeta {
    name: "rcs",
    comment: Some("report runtime call counts and times"),
    flag_index: 816,
    canonical_index: 817,
  },
  EntryMeta {
    name: "rcs_cpu_time",
    comment: Some("report runtime times in cpu time (the default is wall time)"),
    flag_index: 817,
    canonical_index: 818,
  },
  EntryMeta {
    name: "verify_snapshot_checksum",
    comment: Some(
      "Verify snapshot checksums when deserializing snapshots. Enable checksum creation and verification for code caches. Enabled by default in debug builds and once per process for Android.",
    ),
    flag_index: 818,
    canonical_index: 819,
  },
  EntryMeta {
    name: "code_cache_source_hash_sha256",
    comment: Some("Hash code cache source string with SHA256 instead of string length"),
    flag_index: 819,
    canonical_index: 820,
  },
  EntryMeta {
    name: "profile_deserialization",
    comment: Some("Print the time it takes to deserialize the snapshot. (developer-only)"),
    flag_index: 820,
    canonical_index: 821,
  },
  EntryMeta {
    name: "trace_deserialization",
    comment: Some("Trace the snapshot deserialization. (developer-only)"),
    flag_index: 821,
    canonical_index: 822,
  },
  EntryMeta {
    name: "serialization_statistics",
    comment: Some("Collect statistics on serialized objects. (developer-only)"),
    flag_index: 822,
    canonical_index: 823,
  },
  EntryMeta {
    name: "regexp_optimization",
    comment: Some("generate optimized regexp code"),
    flag_index: 823,
    canonical_index: 824,
  },
  EntryMeta {
    name: "regexp_atomic_loop",
    comment: Some("optimize atomic regexp loops"),
    flag_index: 824,
    canonical_index: 825,
  },
  EntryMeta {
    name: "regexp_unroll",
    comment: Some("unroll small {} repeats when optimizing"),
    flag_index: 825,
    canonical_index: 826,
  },
  EntryMeta {
    name: "regexp_masked_dispatch",
    comment: Some("dispatch disjoint regexp alternations on the masked quick-check value"),
    flag_index: 826,
    canonical_index: 827,
  },
  EntryMeta {
    name: "regexp_quick_check",
    comment: Some("generate quickcheck code when optimizing"),
    flag_index: 827,
    canonical_index: 828,
  },
  EntryMeta {
    name: "regexp_interpret_all",
    comment: Some("interpret all regexp code"),
    flag_index: 828,
    canonical_index: 829,
  },
  EntryMeta {
    name: "regexp_tier_up",
    comment: Some(
      "enable regexp interpreter and tier up to the compiler after the number of executions set by the tier up ticks flag",
    ),
    flag_index: 829,
    canonical_index: 830,
  },
  EntryMeta {
    name: "regexp_tier_up_ticks",
    comment: Some(
      "set the number of executions for the regexp interpreter before tiering-up to the compiler",
    ),
    flag_index: 830,
    canonical_index: 831,
  },
  EntryMeta {
    name: "regexp_jit_all",
    comment: Some(
      "compile all regexp patterns directly to native code, skipping the interpreter (equivalent to --regexp-tier-up --regexp-tier-up-ticks=0, which is also the default)",
    ),
    flag_index: 831,
    canonical_index: 832,
  },
  EntryMeta {
    name: "regexp_peephole_optimization",
    comment: Some("enable peephole optimization for regexp bytecode"),
    flag_index: 832,
    canonical_index: 833,
  },
  EntryMeta {
    name: "regexp_results_cache",
    comment: Some("enable the regexp results cache"),
    flag_index: 833,
    canonical_index: 834,
  },
  EntryMeta {
    name: "regexp_assemble_from_bytecode",
    comment: Some("assemble regexp JIT-code from bytecode"),
    flag_index: 834,
    canonical_index: 835,
  },
  EntryMeta {
    name: "regexp_simd_in_rc",
    comment: Some(
      "emit the SIMD scans directly in the regexp compiler, instead of via the bytecode peephole pass",
    ),
    flag_index: 835,
    canonical_index: 836,
  },
  EntryMeta {
    name: "trace_regexp_peephole_optimization",
    comment: Some(
      "trace regexp bytecode peephole optimization (requires v8_enable_disassembler = true)",
    ),
    flag_index: 836,
    canonical_index: 837,
  },
  EntryMeta {
    name: "trace_regexp_bytecodes",
    comment: Some("trace regexp bytecode execution (requires v8_enable_disassembler = true)"),
    flag_index: 837,
    canonical_index: 838,
  },
  EntryMeta {
    name: "trace_regexp_exec",
    comment: Some("trace regexp execution (requires v8_enable_regexp_diagnostics = true)."),
    flag_index: 838,
    canonical_index: 839,
  },
  EntryMeta {
    name: "trace_regexp_exec_ool_subjects",
    comment: Some(
      "trace regexp execution out-of-line subject strings (requires v8_enable_regexp_diagnostics = true).",
    ),
    flag_index: 839,
    canonical_index: 840,
  },
  EntryMeta {
    name: "trace_regexp_assembler",
    comment: Some(
      "trace regexp macro assembler calls (requires v8_enable_regexp_diagnostics = true).",
    ),
    flag_index: 840,
    canonical_index: 841,
  },
  EntryMeta {
    name: "trace_regexp_compiler",
    comment: Some("trace regexp compilation (requires v8_enable_regexp_diagnostics = true)"),
    flag_index: 841,
    canonical_index: 842,
  },
  EntryMeta {
    name: "trace_regexp_graph_building",
    comment: Some(
      "trace regexp AST to graph conversion (requires v8_enable_regexp_diagnostics = true)",
    ),
    flag_index: 842,
    canonical_index: 843,
  },
  EntryMeta {
    name: "trace_regexp_parser",
    comment: Some("trace regexp parsing (requires v8_enable_regexp_diagnostics = true)"),
    flag_index: 843,
    canonical_index: 844,
  },
  EntryMeta {
    name: "trace_regexp_graph",
    comment: Some("trace the regexp graph (requires v8_enable_regexp_diagnostics = true)"),
    flag_index: 844,
    canonical_index: 845,
  },
  EntryMeta {
    name: "print_regexp_graph",
    comment: Some("print the regexp graph (requires v8_enable_regexp_diagnostics = true)"),
    flag_index: 845,
    canonical_index: 846,
  },
  EntryMeta {
    name: "trace_regexp_tier_up",
    comment: Some(
      "trace regexp tiering up execution (requires v8_enable_regexp_diagnostics = true)",
    ),
    flag_index: 846,
    canonical_index: 847,
  },
  EntryMeta {
    name: "enable_experimental_regexp_engine",
    comment: Some(
      "recognize regexps with 'l' flag, run them on experimental engine (experimental)",
    ),
    flag_index: 847,
    canonical_index: 848,
  },
  EntryMeta {
    name: "default_to_experimental_regexp_engine",
    comment: Some("run regexps with the experimental engine where possible"),
    flag_index: 848,
    canonical_index: 849,
  },
  EntryMeta {
    name: "experimental_regexp_engine_capture_group_opt",
    comment: Some("enable time optimizations for the experimental regexp engine"),
    flag_index: 849,
    canonical_index: 850,
  },
  EntryMeta {
    name: "experimental_regexp_engine_capture_group_opt_max_memory_usage",
    comment: Some("maximum memory usage in MB allowed for experimental engine"),
    flag_index: 850,
    canonical_index: 851,
  },
  EntryMeta {
    name: "trace_experimental_regexp_engine",
    comment: Some("trace execution of experimental regexp engine (developer-only)"),
    flag_index: 851,
    canonical_index: 852,
  },
  EntryMeta {
    name: "enable_experimental_regexp_engine_on_excessive_backtracks",
    comment: Some(
      "fall back to a breadth-first regexp engine on excessive backtracking (experimental)",
    ),
    flag_index: 852,
    canonical_index: 853,
  },
  EntryMeta {
    name: "regexp_backtracks_before_fallback",
    comment: Some(
      "number of backtracks during regexp execution before fall back to experimental engine if enable_experimental_regexp_engine_on_excessive_backtracks is set",
    ),
    flag_index: 853,
    canonical_index: 854,
  },
  EntryMeta {
    name: "regexp_simd",
    comment: Some("enable SIMD for regexp jit code"),
    flag_index: 854,
    canonical_index: 855,
  },
  EntryMeta {
    name: "trace_read_only_promotion",
    comment: Some("trace the read-only promotion pass (developer-only)"),
    flag_index: 855,
    canonical_index: 856,
  },
  EntryMeta {
    name: "trace_read_only_promotion_verbose",
    comment: Some("trace the read-only promotion pass (developer-only)"),
    flag_index: 856,
    canonical_index: 857,
  },
  EntryMeta {
    name: "testing_bool_flag",
    comment: Some("testing_bool_flag"),
    flag_index: 857,
    canonical_index: 858,
  },
  EntryMeta {
    name: "testing_maybe_bool_flag",
    comment: Some("testing_maybe_bool_flag"),
    flag_index: 858,
    canonical_index: 859,
  },
  EntryMeta {
    name: "testing_int_flag",
    comment: Some("testing_int_flag"),
    flag_index: 859,
    canonical_index: 860,
  },
  EntryMeta {
    name: "testing_float_flag",
    comment: Some("float-flag"),
    flag_index: 860,
    canonical_index: 861,
  },
  EntryMeta {
    name: "testing_string_flag",
    comment: Some("string-flag"),
    flag_index: 861,
    canonical_index: 862,
  },
  EntryMeta {
    name: "testing_prng_seed",
    comment: Some("Seed used for threading test randomness"),
    flag_index: 862,
    canonical_index: 863,
  },
  EntryMeta {
    name: "testing_bool_flag_A",
    comment: Some("testing_bool_flag_A"),
    flag_index: 863,
    canonical_index: 864,
  },
  EntryMeta {
    name: "testing_bool_flag_B",
    comment: Some("testing_bool_flag_B"),
    flag_index: 864,
    canonical_index: 865,
  },
  EntryMeta {
    name: "testing_bool_flag_C",
    comment: Some("testing_bool_flag_C"),
    flag_index: 865,
    canonical_index: 866,
  },
  EntryMeta {
    name: "strict_termination_checks",
    comment: Some(
      "Enable strict terminating DCHECKs to prevent accidentally keeping on executing JS after terminating V8. (experimental)",
    ),
    flag_index: 866,
    canonical_index: 867,
  },
  EntryMeta {
    name: "fuzzing",
    comment: Some(
      "Fuzzers use this flag to signal that they are ... fuzzing. This causes intrinsics to fail silently (e.g. return undefined) on invalid usage.",
    ),
    flag_index: 867,
    canonical_index: 868,
  },
  EntryMeta {
    name: "hole_fuzzing",
    comment: Some(
      "Fuzzers use this flag to turn DCHECKs into NOPs  and CHECK failures into silent exits. This is useful if we want to find memory corruption primitives with a leaked hole, where the engine is already in a weird state",
    ),
    flag_index: 868,
    canonical_index: 869,
  },
  EntryMeta {
    name: "sandbox_testing",
    comment: Some(
      "Enable sandbox testing mode. This exposes the memory corruption API (if available) and enables the sandbox crash filter to terminate the process (with status zero) if a crash that does not represent a sandbox violation is detected.",
    ),
    flag_index: 869,
    canonical_index: 870,
  },
  EntryMeta {
    name: "validate_generated_code",
    comment: Some("Enable generated code verifier"),
    flag_index: 870,
    canonical_index: 871,
  },
  EntryMeta {
    name: "validate_generated_code_include_code",
    comment: Some(
      "Print the disassembled code as part of the generated code validator's output in case of violations. (developer-only)",
    ),
    flag_index: 871,
    canonical_index: 872,
  },
  EntryMeta {
    name: "validate_generated_code_non_fatal",
    comment: Some(
      "Treat generated code validator violations as non-fatal. Can be used to find all issues rather than exiting after the first one. (developer-only)",
    ),
    flag_index: 872,
    canonical_index: 873,
  },
  EntryMeta {
    name: "sandbox_fuzzing",
    comment: Some(
      "Enable sandbox fuzzing mode. This exposes the memory corruption API and enables the sandbox crash filter to terminate the process (with non-zero status) if a crash that does not represent a sandbox violation is detected.",
    ),
    flag_index: 873,
    canonical_index: 874,
  },
  EntryMeta {
    name: "sandbox_trap_fuzzing",
    comment: Some("Enable the trap-based sandbox fuzzing mode."),
    flag_index: 874,
    canonical_index: 875,
  },
  EntryMeta {
    name: "verify_bytecode_light",
    comment: Some(
      "Perform lightweight bytecode verification to ensure basic safety properties of generated bytecode, mostly around control-flow.",
    ),
    flag_index: 875,
    canonical_index: 876,
  },
  EntryMeta {
    name: "verify_bytecode_full",
    comment: Some(
      "Perform full bytecode verification to ensure generated bytecode is safe to execute when the sandbox is enabled (i.e. does not lead to out-of-sandbox memory corruption). Full verification is a strict superset of the lightweight verification.",
    ),
    flag_index: 876,
    canonical_index: 877,
  },
  EntryMeta {
    name: "expose_memory_corruption_api",
    comment: Some(
      "Exposes the memory corruption API. Set automatically by --sandbox-testing and --sandbox-fuzzing.",
    ),
    flag_index: 877,
    canonical_index: 878,
  },
  EntryMeta {
    name: "sandbox_prohibit_insecure_mode",
    comment: Some("Prohibits an insecure sandbox setup."),
    flag_index: 878,
    canonical_index: 879,
  },
  EntryMeta {
    name: "freeze_flags_after_init",
    comment: Some("Disallow changes to flag values after initializing V8"),
    flag_index: 879,
    canonical_index: 880,
  },
  EntryMeta {
    name: "cet_compatible",
    comment: Some("Intel CET is only supported on x64."),
    flag_index: 880,
    canonical_index: 881,
  },
  EntryMeta {
    name: "embedded_src",
    comment: Some("Path for the generated embedded data file. (mksnapshot only)"),
    flag_index: 881,
    canonical_index: 882,
  },
  EntryMeta {
    name: "embedded_variant",
    comment: Some("Label to disambiguate symbols in embedded data file. (mksnapshot only)"),
    flag_index: 882,
    canonical_index: 883,
  },
  EntryMeta {
    name: "startup_src",
    comment: Some("Write V8 startup as C++ src. (mksnapshot only)"),
    flag_index: 883,
    canonical_index: 884,
  },
  EntryMeta {
    name: "startup_blob",
    comment: Some("Write V8 startup blob file. (mksnapshot only)"),
    flag_index: 884,
    canonical_index: 885,
  },
  EntryMeta {
    name: "target_arch",
    comment: Some("The mksnapshot target arch. (mksnapshot only)"),
    flag_index: 885,
    canonical_index: 886,
  },
  EntryMeta {
    name: "target_os",
    comment: Some("The mksnapshot target os. (mksnapshot only)"),
    flag_index: 886,
    canonical_index: 887,
  },
  EntryMeta {
    name: "target_is_simulator",
    comment: Some(
      "Instruct mksnapshot that the target is meant to run in the simulator and it can generate simulator-specific instructions. (mksnapshot only)",
    ),
    flag_index: 887,
    canonical_index: 888,
  },
  EntryMeta {
    name: "turbo_profiling_input",
    comment: Some(
      "Path of the input file containing basic information for builtins. (mksnapshot only)",
    ),
    flag_index: 888,
    canonical_index: 889,
  },
  EntryMeta {
    name: "turbo_log_builtins_count_input",
    comment: Some(
      "Path of the input file containing basic block counters for builtins for logging in turbolizer. (mksnapshot only)",
    ),
    flag_index: 889,
    canonical_index: 890,
  },
  EntryMeta {
    name: "text_is_readable",
    comment: Some("Whether the .text section of binary can be read"),
    flag_index: 890,
    canonical_index: 891,
  },
  EntryMeta {
    name: "trace_minor_ms_parallel_marking",
    comment: Some("trace parallel marking for the young generation (developer-only)"),
    flag_index: 891,
    canonical_index: 892,
  },
  EntryMeta {
    name: "minor_ms",
    comment: Some("perform young generation mark sweep GCs (experimental)"),
    flag_index: 892,
    canonical_index: 893,
  },
  EntryMeta {
    name: "concurrent_minor_ms_marking",
    comment: Some("perform young generation marking concurrently"),
    flag_index: 893,
    canonical_index: 894,
  },
  EntryMeta {
    name: "black_allocated_pages",
    comment: Some("allocate non-young objects during incremental marking on separate pages"),
    flag_index: 894,
    canonical_index: 895,
  },
  EntryMeta {
    name: "sticky_mark_bits",
    comment: Some("use sticky mark bits for separation of generations"),
    flag_index: 895,
    canonical_index: 896,
  },
  EntryMeta {
    name: "minor_ms_min_new_space_capacity_for_concurrent_marking_mb",
    comment: Some("min new space capacity in MBs for using young generation concurrent marking."),
    flag_index: 896,
    canonical_index: 897,
  },
  EntryMeta {
    name: "minor_ms_concurrent_marking_trigger",
    comment: Some(
      "minor ms concurrent marking trigger in percent of the current new space capacity",
    ),
    flag_index: 897,
    canonical_index: 898,
  },
  EntryMeta {
    name: "minor_ms_min_lab_size_kb",
    comment: Some(
      "override for the minimum lab size in KB to be used for new space allocations with minor ms. ",
    ),
    flag_index: 898,
    canonical_index: 899,
  },
  EntryMeta {
    name: "handle_weak_ref_weakly_in_minor_gc",
    comment: Some("Enables weak handling of WeakRef and FinalizationRegistry in minor GCs."),
    flag_index: 899,
    canonical_index: 900,
  },
  EntryMeta {
    name: "help",
    comment: Some("Print usage message, including flags, on console"),
    flag_index: 900,
    canonical_index: 901,
  },
  EntryMeta {
    name: "print_flag_values",
    comment: Some("Print all flag values of V8"),
    flag_index: 901,
    canonical_index: 902,
  },
  EntryMeta {
    name: "print_feature_flags_json",
    comment: Some("Print JS and Wasm feature flags grouped by in-progress, staged, and shipped"),
    flag_index: 902,
    canonical_index: 903,
  },
  EntryMeta {
    name: "slow_histograms",
    comment: Some("Enable slow histograms with more overhead."),
    flag_index: 903,
    canonical_index: 904,
  },
  EntryMeta {
    name: "use_external_strings",
    comment: Some("Use external strings for source code"),
    flag_index: 904,
    canonical_index: 905,
  },
  EntryMeta {
    name: "map_counters",
    comment: Some("Map counters to a file"),
    flag_index: 905,
    canonical_index: 906,
  },
  EntryMeta {
    name: "mock_arraybuffer_allocator",
    comment: Some("Use a mock ArrayBuffer allocator for testing. (test-only / unsafe)"),
    flag_index: 906,
    canonical_index: 907,
  },
  EntryMeta {
    name: "mock_arraybuffer_allocator_limit",
    comment: Some("Memory limit for mock ArrayBuffer allocator used to simulate OOM for testing."),
    flag_index: 907,
    canonical_index: 908,
  },
  EntryMeta {
    name: "gdbjit",
    comment: Some("enable GDBJIT interface"),
    flag_index: 908,
    canonical_index: 909,
  },
  EntryMeta {
    name: "gdbjit_full",
    comment: Some("enable GDBJIT interface for all code objects"),
    flag_index: 909,
    canonical_index: 910,
  },
  EntryMeta {
    name: "gdbjit_dump",
    comment: Some("dump elf objects with debug info to disk"),
    flag_index: 910,
    canonical_index: 911,
  },
  EntryMeta {
    name: "torque_dwarf",
    comment: Some("enable DWARF line info for Torque files"),
    flag_index: 911,
    canonical_index: 912,
  },
  EntryMeta {
    name: "gdbjit_dump_filter",
    comment: Some("dump only objects containing this substring"),
    flag_index: 912,
    canonical_index: 913,
  },
  EntryMeta {
    name: "enable_slow_asserts",
    comment: Some("enable asserts that are slow to execute"),
    flag_index: 913,
    canonical_index: 914,
  },
  EntryMeta {
    name: "print_ast",
    comment: Some("print source AST (developer-only)"),
    flag_index: 914,
    canonical_index: 915,
  },
  EntryMeta {
    name: "print_scopes",
    comment: Some("print scopes (developer-only)"),
    flag_index: 915,
    canonical_index: 916,
  },
  EntryMeta {
    name: "trace_contexts",
    comment: Some("trace contexts operations (developer-only)"),
    flag_index: 916,
    canonical_index: 917,
  },
  EntryMeta {
    name: "code_stats",
    comment: Some("report code statistics after GC (developer-only)"),
    flag_index: 917,
    canonical_index: 918,
  },
  EntryMeta {
    name: "print_handles",
    comment: Some("report handles after GC (developer-only)"),
    flag_index: 918,
    canonical_index: 919,
  },
  EntryMeta {
    name: "check_handle_count",
    comment: Some("Check that there are not too many handles at GC"),
    flag_index: 919,
    canonical_index: 920,
  },
  EntryMeta {
    name: "print_global_handles",
    comment: Some("report global handles after GC (developer-only)"),
    flag_index: 920,
    canonical_index: 921,
  },
  EntryMeta {
    name: "trace_turbo_escape",
    comment: Some("enable tracing in escape analysis (developer-only)"),
    flag_index: 921,
    canonical_index: 922,
  },
  EntryMeta {
    name: "trace_module_status",
    comment: Some("Trace status transitions of ECMAScript modules (developer-only)"),
    flag_index: 922,
    canonical_index: 923,
  },
  EntryMeta {
    name: "trace_normalization",
    comment: Some("prints when objects are turned into dictionaries. (developer-only)"),
    flag_index: 923,
    canonical_index: 924,
  },
  EntryMeta {
    name: "trace_lazy",
    comment: Some("trace lazy compilation (developer-only)"),
    flag_index: 924,
    canonical_index: 925,
  },
  EntryMeta {
    name: "trace_isolates",
    comment: Some("trace isolate state changes (developer-only)"),
    flag_index: 925,
    canonical_index: 926,
  },
  EntryMeta {
    name: "regexp_possessive_quantifier",
    comment: Some("enable possessive quantifier syntax for testing"),
    flag_index: 926,
    canonical_index: 927,
  },
  EntryMeta {
    name: "print_break_location",
    comment: Some("print source location on debug break (developer-only)"),
    flag_index: 927,
    canonical_index: 928,
  },
  EntryMeta {
    name: "logfile",
    comment: Some(
      "Specify the name of the log file, use '-' for console, '+' for a temporary file.",
    ),
    flag_index: 928,
    canonical_index: 929,
  },
  EntryMeta {
    name: "logfile_per_isolate",
    comment: Some("Separate log files for each isolate."),
    flag_index: 929,
    canonical_index: 930,
  },
  EntryMeta {
    name: "log",
    comment: Some("Minimal logging (no API, code, GC, suspect, or handles samples)."),
    flag_index: 930,
    canonical_index: 931,
  },
  EntryMeta {
    name: "log_all",
    comment: Some("Log all events to the log file."),
    flag_index: 931,
    canonical_index: 932,
  },
  EntryMeta {
    name: "log_internal_timer_events",
    comment: Some("See --log-timer-events"),
    flag_index: 932,
    canonical_index: 933,
  },
  EntryMeta {
    name: "log_timer_events",
    comment: Some("Log timer events (incl. console.time* and Date.now)."),
    flag_index: 933,
    canonical_index: 934,
  },
  EntryMeta {
    name: "log_source_code",
    comment: Some("Log source code."),
    flag_index: 934,
    canonical_index: 935,
  },
  EntryMeta {
    name: "log_source_position",
    comment: Some("Log detailed source information."),
    flag_index: 935,
    canonical_index: 936,
  },
  EntryMeta {
    name: "log_code",
    comment: Some("Log code events to the log file without profiling."),
    flag_index: 936,
    canonical_index: 937,
  },
  EntryMeta {
    name: "log_feedback_vector",
    comment: Some("Log FeedbackVectors on first creation"),
    flag_index: 937,
    canonical_index: 938,
  },
  EntryMeta {
    name: "log_code_disassemble",
    comment: Some("Log all disassembled code to the log file."),
    flag_index: 938,
    canonical_index: 939,
  },
  EntryMeta {
    name: "log_function_events",
    comment: Some("Log function events (parse, compile, execute) separately."),
    flag_index: 939,
    canonical_index: 940,
  },
  EntryMeta {
    name: "detailed_line_info",
    comment: Some("Always generate detailed line information for CPU profiling."),
    flag_index: 940,
    canonical_index: 941,
  },
  EntryMeta {
    name: "prof_include_idle",
    comment: Some("Include idle samples in the profile"),
    flag_index: 941,
    canonical_index: 942,
  },
  EntryMeta {
    name: "perfetto_code_logger",
    comment: Some("Enable the Perfetto code data source (requires the v8_use_perfetto gn arg)"),
    flag_index: 942,
    canonical_index: 943,
  },
  EntryMeta {
    name: "prof_sampling_interval",
    comment: Some("Interval for --prof samples (in microseconds)."),
    flag_index: 943,
    canonical_index: 944,
  },
  EntryMeta {
    name: "prof_cpp",
    comment: Some("Like --prof, but ignore generated code."),
    flag_index: 944,
    canonical_index: 945,
  },
  EntryMeta {
    name: "prof_browser_mode",
    comment: Some("Used with --prof, turns on browser-compatible mode for profiling."),
    flag_index: 945,
    canonical_index: 946,
  },
  EntryMeta {
    name: "prof",
    comment: Some("Log statistical profiling information (implies --log-code)."),
    flag_index: 946,
    canonical_index: 947,
  },
  EntryMeta {
    name: "ll_prof",
    comment: Some("Enable low-level linux profiler. (developer-only)"),
    flag_index: 947,
    canonical_index: 948,
  },
  EntryMeta {
    name: "perf_basic_prof",
    comment: Some("Enable basic support for perf profiler. (developer-only)"),
    flag_index: 948,
    canonical_index: 949,
  },
  EntryMeta {
    name: "perf_basic_prof_path",
    comment: Some("directory to write perf-<pid>.map symbol file to"),
    flag_index: 949,
    canonical_index: 950,
  },
  EntryMeta {
    name: "perf_basic_prof_only_functions",
    comment: Some("Only report function code ranges to perf (i.e. no stubs). (developer-only)"),
    flag_index: 950,
    canonical_index: 951,
  },
  EntryMeta {
    name: "perf_prof",
    comment: Some("Enable experimental annotate support for perf profiler. (developer-only)"),
    flag_index: 951,
    canonical_index: 952,
  },
  EntryMeta {
    name: "perf_prof_path",
    comment: Some("directory to write jit-<pid>.dump symbol file to"),
    flag_index: 952,
    canonical_index: 953,
  },
  EntryMeta {
    name: "perf_prof_annotate_wasm",
    comment: Some(
      "Used with --perf-prof, load wasm source map and provide annotate support (experimental). (developer-only)",
    ),
    flag_index: 953,
    canonical_index: 954,
  },
  EntryMeta {
    name: "perf_prof_delete_file",
    comment: Some(
      "Remove the perf file right after creating it (for testing only). (developer-only)",
    ),
    flag_index: 954,
    canonical_index: 955,
  },
  EntryMeta {
    name: "perf_prof_unwinding_info",
    comment: Some("Enable unwinding info for perf linux profiler (experimental). (developer-only)"),
    flag_index: 955,
    canonical_index: 956,
  },
  EntryMeta {
    name: "gc_fake_mmap",
    comment: Some("Specify the name of the file for fake gc mmap used in ll_prof"),
    flag_index: 956,
    canonical_index: 957,
  },
  EntryMeta {
    name: "redirect_code_traces",
    comment: Some("output deopt information and disassembly into file code-<pid>-<isolate id>.asm"),
    flag_index: 957,
    canonical_index: 958,
  },
  EntryMeta {
    name: "redirect_code_traces_to",
    comment: Some("output deopt information and disassembly into the given file"),
    flag_index: 958,
    canonical_index: 959,
  },
  EntryMeta {
    name: "print_opt_source",
    comment: Some("print source code of optimized and inlined functions (developer-only)"),
    flag_index: 959,
    canonical_index: 960,
  },
  EntryMeta {
    name: "vtune_prof_annotate_wasm",
    comment: Some(
      "Used when v8_enable_vtunejit is enabled, load wasm source map and provide annotate support (experimental).",
    ),
    flag_index: 960,
    canonical_index: 961,
  },
  EntryMeta {
    name: "win64_unwinding_info",
    comment: Some("Enable unwinding info for Windows/x64"),
    flag_index: 961,
    canonical_index: 962,
  },
  EntryMeta {
    name: "interpreted_frames_native_stack",
    comment: Some("Show interpreted frames on the native stack (useful for external profilers)."),
    flag_index: 962,
    canonical_index: 963,
  },
  EntryMeta {
    name: "enable_etw_stack_walking",
    comment: Some("Enable etw stack walking for windows"),
    flag_index: 963,
    canonical_index: 964,
  },
  EntryMeta {
    name: "etw_trace_debug",
    comment: Some("Enable etw debug logging (only on debug builds)"),
    flag_index: 964,
    canonical_index: 965,
  },
  EntryMeta {
    name: "enable_etw_by_custom_filter_only",
    comment: Some(
      "Enable etw stack walking for windows, but only if explicitly specified in a WPR profile",
    ),
    flag_index: 965,
    canonical_index: 966,
  },
  EntryMeta {
    name: "print_builtin_size",
    comment: Some("print code size for builtins"),
    flag_index: 966,
    canonical_index: 967,
  },
  EntryMeta {
    name: "trace_elements_transitions",
    comment: Some("trace elements transitions (developer-only)"),
    flag_index: 967,
    canonical_index: 968,
  },
  EntryMeta {
    name: "trace_creation_allocation_sites",
    comment: Some("trace the creation of allocation sites (developer-only)"),
    flag_index: 968,
    canonical_index: 969,
  },
  EntryMeta {
    name: "print_code",
    comment: Some("print generated code (developer-only)"),
    flag_index: 969,
    canonical_index: 970,
  },
  EntryMeta {
    name: "print_opt_code",
    comment: Some("print optimized code (developer-only)"),
    flag_index: 970,
    canonical_index: 971,
  },
  EntryMeta {
    name: "print_opt_code_filter",
    comment: Some("filter for printing optimized code"),
    flag_index: 971,
    canonical_index: 972,
  },
  EntryMeta {
    name: "print_code_verbose",
    comment: Some("print more information for code (developer-only)"),
    flag_index: 972,
    canonical_index: 973,
  },
  EntryMeta {
    name: "print_builtin_code",
    comment: Some("print generated code for builtins (developer-only)"),
    flag_index: 973,
    canonical_index: 974,
  },
  EntryMeta {
    name: "print_builtin_code_filter",
    comment: Some("filter for printing builtin code"),
    flag_index: 974,
    canonical_index: 975,
  },
  EntryMeta {
    name: "print_regexp_code",
    comment: Some("print generated regexp code (developer-only)"),
    flag_index: 975,
    canonical_index: 976,
  },
  EntryMeta {
    name: "print_regexp_bytecode",
    comment: Some("print generated regexp bytecode (developer-only)"),
    flag_index: 976,
    canonical_index: 977,
  },
  EntryMeta {
    name: "predictable",
    comment: Some("enable predictable mode"),
    flag_index: 977,
    canonical_index: 978,
  },
  EntryMeta {
    name: "predictable_gc_schedule",
    comment: Some(
      "Predictable garbage collection schedule. Fixes heap growing, idle, and memory reducing behavior.",
    ),
    flag_index: 978,
    canonical_index: 979,
  },
  EntryMeta {
    name: "single_threaded",
    comment: Some("disable the use of background tasks"),
    flag_index: 979,
    canonical_index: 980,
  },
  EntryMeta {
    name: "single_threaded_gc",
    comment: Some("disable the use of background gc tasks"),
    flag_index: 980,
    canonical_index: 981,
  },
  EntryMeta {
    name: "single_threaded_gc_in_background",
    comment: Some("disable the use of background gc tasks when in background"),
    flag_index: 981,
    canonical_index: 982,
  },
  EntryMeta {
    name: "parallel_pause_for_gc_in_background",
    comment: Some("Use parallel threads in the atomic pause for background GCs"),
    flag_index: 982,
    canonical_index: 983,
  },
  EntryMeta {
    name: "incremental_marking_for_gc_in_background",
    comment: Some("Use parallel threads in the atomic pause for background GCs"),
    flag_index: 983,
    canonical_index: 984,
  },
  EntryMeta {
    name: "shared_heap",
    comment: Some("Enables a shared heap between isolates."),
    flag_index: 984,
    canonical_index: 985,
  },
  EntryMeta {
    name: "empty_shared_heap",
    comment: Some("Enables a shared heap, but not shared allocations"),
    flag_index: 985,
    canonical_index: 986,
  },
  EntryMeta {
    name: "proto_assign_seq_opt",
    comment: Some(
      "Enable optimizing a sequence of `Class_X.prototype.[key] = ...`by replacing it by a runtime code somewhat equivalent to `Object.assign(Class_X.prototype, [boilerplate_obj])`",
    ),
    flag_index: 986,
    canonical_index: 987,
  },
  EntryMeta {
    name: "proto_assign_seq_opt_count",
    comment: Some(
      "The minimum number of consecutive property assignments on the prototype object for replacing it with a single byte code",
    ),
    flag_index: 987,
    canonical_index: 988,
  },
  EntryMeta {
    name: "proto_assign_seq_lazy_func_opt",
    comment: Some(
      "Functions bulk assigned to prototype via SetPrototypeProperties will remain in SharedFunctionInfo (Rather than JSFunction) State until first access (experimental)",
    ),
    flag_index: 988,
    canonical_index: 989,
  },
  EntryMeta {
    name: "private_field_bytecodes",
    comment: Some("Use specialized bytecodes for Get/Set Private Fields (experimental)"),
    flag_index: 989,
    canonical_index: 990,
  },
  EntryMeta {
    name: "track_array_buffer_views",
    comment: Some(
      "Track TypedArrays and DataViews attached to array buffers and update them in-place when the buffer detaches",
    ),
    flag_index: 990,
    canonical_index: 991,
  },
  EntryMeta {
    name: "superspreading",
    comment: Some("Allow arbitrary length spread arguments on supported builtins (experimental)"),
    flag_index: 991,
    canonical_index: 992,
  },
  EntryMeta {
    name: "fast_proxy_ic",
    comment: Some("Support proxy traps in ICs"),
    flag_index: 992,
    canonical_index: 993,
  },
  EntryMeta {
    name: "is_standalone_d8_shell",
    comment: Some(
      "Tells V8 it's running as part of the d8. This flag should not be set in other cases.",
    ),
    flag_index: 993,
    canonical_index: 994,
  },
  EntryMeta {
    name: "run_as_security_poc",
    comment: Some("Run programs as security proof of concept (POC)."),
    flag_index: 994,
    canonical_index: 995,
  },
  EntryMeta {
    name: "run_as_sandbox_security_poc",
    comment: Some("Run programs as security proof of concept (POC) for the V8 sandbox"),
    flag_index: 995,
    canonical_index: 996,
  },
  EntryMeta {
    name: "verify_predictable",
    comment: Some("this mode is used for checking that V8 behaves predictably"),
    flag_index: 996,
    canonical_index: 997,
  },
  EntryMeta {
    name: "dump_allocations_digest_at_alloc",
    comment: Some("dump allocations digest each n-th allocation"),
    flag_index: 997,
    canonical_index: 998,
  },
];

/// 按归一化名升序的 ALL_ENTRIES 下标 (原版 kSortedFlagIndices; 名称唯一, 稳定排序等价)
pub(crate) static SORTED_ALL: &[u32] = &[
  318u32, 815u32, 801u32, 118u32, 810u32, 626u32, 155u32, 154u32, 796u32, 795u32, 729u32, 752u32,
  181u32, 209u32, 404u32, 221u32, 310u32, 697u32, 1u32, 149u32, 799u32, 210u32, 216u32, 140u32,
  814u32, 895u32, 10u32, 275u32, 723u32, 644u32, 130u32, 755u32, 881u32, 920u32, 769u32, 669u32,
  783u32, 820u32, 681u32, 918u32, 627u32, 628u32, 629u32, 631u32, 632u32, 634u32, 633u32, 630u32,
  636u32, 635u32, 754u32, 743u32, 576u32, 225u32, 714u32, 68u32, 67u32, 574u32, 665u32, 575u32,
  894u32, 301u32, 226u32, 229u32, 230u32, 228u32, 211u32, 213u32, 212u32, 581u32, 247u32, 231u32,
  25u32, 803u32, 802u32, 541u32, 572u32, 673u32, 672u32, 761u32, 274u32, 678u32, 369u32, 849u32,
  240u32, 941u32, 585u32, 4u32, 122u32, 26u32, 666u32, 242u32, 146u32, 22u32, 725u32, 5u32, 3u32,
  132u32, 998u32, 320u32, 467u32, 468u32, 139u32, 143u32, 142u32, 882u32, 883u32, 126u32, 986u32,
  707u32, 21u32, 705u32, 710u32, 687u32, 689u32, 690u32, 688u32, 693u32, 694u32, 121u32, 966u32,
  964u32, 848u32, 853u32, 692u32, 691u32, 199u32, 695u32, 708u32, 696u32, 14u32, 711u32, 686u32,
  11u32, 914u32, 704u32, 682u32, 684u32, 685u32, 683u32, 709u32, 459u32, 23u32, 706u32, 586u32,
  579u32, 965u32, 0u32, 811u32, 384u32, 850u32, 851u32, 728u32, 724u32, 331u32, 726u32, 727u32,
  719u32, 717u32, 718u32, 766u32, 878u32, 720u32, 721u32, 137u32, 615u32, 616u32, 340u32, 732u32,
  733u32, 781u32, 784u32, 993u32, 172u32, 276u32, 648u32, 6u32, 642u32, 643u32, 513u32, 13u32,
  37u32, 698u32, 148u32, 730u32, 282u32, 880u32, 607u32, 124u32, 125u32, 385u32, 359u32, 660u32,
  807u32, 868u32, 269u32, 957u32, 538u32, 540u32, 613u32, 623u32, 600u32, 909u32, 911u32, 913u32,
  910u32, 900u32, 764u32, 386u32, 414u32, 393u32, 391u32, 390u32, 392u32, 415u32, 805u32, 624u32,
  772u32, 773u32, 776u32, 774u32, 775u32, 777u32, 901u32, 169u32, 170u32, 771u32, 869u32, 424u32,
  790u32, 16u32, 15u32, 670u32, 193u32, 197u32, 195u32, 194u32, 196u32, 561u32, 984u32, 564u32,
  563u32, 562u32, 565u32, 589u32, 588u32, 535u32, 536u32, 223u32, 136u32, 734u32, 703u32, 173u32,
  963u32, 17u32, 379u32, 176u32, 177u32, 178u32, 380u32, 183u32, 184u32, 182u32, 994u32, 188u32,
  145u32, 394u32, 411u32, 416u32, 409u32, 410u32, 422u32, 421u32, 417u32, 418u32, 419u32, 12u32,
  9u32, 420u32, 396u32, 389u32, 395u32, 387u32, 128u32, 127u32, 656u32, 677u32, 739u32, 756u32,
  757u32, 740u32, 191u32, 529u32, 741u32, 457u32, 458u32, 20u32, 948u32, 36u32, 931u32, 932u32,
  937u32, 939u32, 765u32, 749u32, 938u32, 940u32, 780u32, 933u32, 793u32, 794u32, 309u32, 935u32,
  936u32, 934u32, 929u32, 930u32, 38u32, 302u32, 43u32, 92u32, 93u32, 94u32, 95u32, 51u32, 54u32,
  84u32, 50u32, 52u32, 45u32, 303u32, 90u32, 117u32, 39u32, 82u32, 53u32, 134u32, 44u32, 59u32,
  46u32, 48u32, 49u32, 42u32, 85u32, 305u32, 47u32, 298u32, 233u32, 234u32, 56u32, 55u32, 96u32,
  97u32, 91u32, 98u32, 99u32, 61u32, 65u32, 78u32, 83u32, 115u32, 116u32, 57u32, 79u32, 80u32,
  64u32, 66u32, 658u32, 659u32, 668u32, 906u32, 534u32, 284u32, 286u32, 285u32, 289u32, 287u32,
  290u32, 657u32, 742u32, 299u32, 70u32, 69u32, 71u32, 72u32, 74u32, 73u32, 75u32, 292u32, 533u32,
  138u32, 291u32, 531u32, 236u32, 224u32, 768u32, 300u32, 77u32, 782u32, 443u32, 587u32, 699u32,
  789u32, 674u32, 675u32, 652u32, 654u32, 653u32, 655u32, 147u32, 609u32, 612u32, 610u32, 611u32,
  614u32, 141u32, 715u32, 293u32, 76u32, 530u32, 186u32, 187u32, 568u32, 569u32, 570u32, 893u32,
  898u32, 160u32, 899u32, 897u32, 159u32, 158u32, 171u32, 558u32, 907u32, 908u32, 608u32, 785u32,
  203u32, 244u32, 649u32, 381u32, 573u32, 248u32, 40u32, 179u32, 180u32, 185u32, 156u32, 157u32,
  582u32, 759u32, 760u32, 584u32, 578u32, 983u32, 583u32, 664u32, 567u32, 797u32, 700u32, 949u32,
  951u32, 950u32, 952u32, 954u32, 955u32, 953u32, 956u32, 943u32, 32u32, 537u32, 978u32, 979u32,
  809u32, 915u32, 928u32, 974u32, 975u32, 967u32, 198u32, 202u32, 970u32, 973u32, 241u32, 903u32,
  902u32, 591u32, 921u32, 919u32, 100u32, 110u32, 111u32, 112u32, 971u32, 972u32, 960u32, 977u32,
  976u32, 846u32, 916u32, 362u32, 364u32, 503u32, 504u32, 430u32, 505u32, 990u32, 947u32, 946u32,
  945u32, 942u32, 944u32, 821u32, 377u32, 378u32, 778u32, 989u32, 987u32, 988u32, 539u32, 806u32,
  667u32, 817u32, 818u32, 663u32, 958u32, 959u32, 835u32, 825u32, 854u32, 829u32, 832u32, 827u32,
  824u32, 833u32, 927u32, 828u32, 834u32, 855u32, 836u32, 830u32, 831u32, 826u32, 804u32, 382u32,
  317u32, 288u32, 639u32, 542u32, 996u32, 995u32, 816u32, 606u32, 779u32, 874u32, 879u32, 870u32,
  875u32, 34u32, 35u32, 27u32, 30u32, 33u32, 31u32, 123u32, 712u32, 532u32, 823u32, 985u32, 220u32,
  219u32, 812u32, 637u32, 798u32, 701u32, 702u32, 762u32, 24u32, 980u32, 981u32, 982u32, 680u32,
  904u32, 131u32, 208u32, 214u32, 787u32, 215u32, 786u32, 617u32, 618u32, 622u32, 621u32, 620u32,
  619u32, 767u32, 722u32, 800u32, 885u32, 884u32, 896u32, 713u32, 514u32, 638u32, 640u32, 577u32,
  232u32, 235u32, 641u32, 645u32, 338u32, 192u32, 651u32, 294u32, 201u32, 737u32, 200u32, 41u32,
  661u32, 650u32, 239u32, 528u32, 662u32, 28u32, 29u32, 19u32, 510u32, 512u32, 454u32, 867u32,
  175u32, 788u32, 992u32, 736u32, 735u32, 886u32, 888u32, 887u32, 2u32, 731u32, 858u32, 864u32,
  865u32, 866u32, 861u32, 860u32, 859u32, 863u32, 862u32, 891u32, 912u32, 738u32, 266u32, 554u32,
  599u32, 217u32, 218u32, 166u32, 813u32, 152u32, 758u32, 580u32, 227u32, 671u32, 917u32, 969u32,
  748u32, 750u32, 822u32, 603u32, 555u32, 968u32, 311u32, 559u32, 552u32, 852u32, 751u32, 646u32,
  792u32, 556u32, 557u32, 543u32, 548u32, 549u32, 550u32, 551u32, 545u32, 544u32, 595u32, 601u32,
  547u32, 207u32, 238u32, 237u32, 204u32, 590u32, 926u32, 129u32, 925u32, 460u32, 108u32, 304u32,
  109u32, 102u32, 103u32, 105u32, 63u32, 106u32, 104u32, 306u32, 113u32, 62u32, 114u32, 107u32,
  58u32, 676u32, 546u32, 206u32, 892u32, 923u32, 560u32, 924u32, 133u32, 744u32, 747u32, 745u32,
  746u32, 308u32, 161u32, 571u32, 553u32, 162u32, 163u32, 167u32, 791u32, 808u32, 856u32, 857u32,
  841u32, 838u32, 842u32, 839u32, 840u32, 845u32, 843u32, 844u32, 837u32, 847u32, 267u32, 164u32,
  753u32, 763u32, 335u32, 592u32, 593u32, 8u32, 205u32, 250u32, 265u32, 260u32, 263u32, 256u32,
  922u32, 255u32, 252u32, 253u32, 295u32, 333u32, 262u32, 313u32, 264u32, 251u32, 259u32, 254u32,
  258u32, 268u32, 261u32, 257u32, 367u32, 566u32, 273u32, 518u32, 509u32, 525u32, 447u32, 451u32,
  450u32, 527u32, 462u32, 492u32, 519u32, 507u32, 501u32, 461u32, 428u32, 429u32, 431u32, 494u32,
  453u32, 452u32, 493u32, 101u32, 596u32, 598u32, 991u32, 602u32, 165u32, 594u32, 222u32, 679u32,
  329u32, 245u32, 327u32, 283u32, 328u32, 339u32, 249u32, 296u32, 341u32, 281u32, 330u32, 323u32,
  312u32, 890u32, 324u32, 326u32, 325u32, 322u32, 343u32, 342u32, 344u32, 314u32, 889u32, 316u32,
  315u32, 337u32, 280u32, 277u32, 278u32, 279u32, 334u32, 332u32, 307u32, 336u32, 270u32, 321u32,
  271u32, 246u32, 243u32, 120u32, 135u32, 361u32, 119u32, 88u32, 366u32, 365u32, 86u32, 87u32,
  363u32, 89u32, 60u32, 81u32, 345u32, 150u32, 346u32, 351u32, 353u32, 352u32, 354u32, 355u32,
  371u32, 375u32, 372u32, 373u32, 370u32, 374u32, 356u32, 357u32, 347u32, 368u32, 174u32, 153u32,
  905u32, 190u32, 18u32, 647u32, 770u32, 297u32, 189u32, 7u32, 625u32, 871u32, 872u32, 873u32,
  877u32, 876u32, 716u32, 272u32, 383u32, 604u32, 997u32, 151u32, 819u32, 376u32, 605u32, 961u32,
  319u32, 407u32, 523u32, 470u32, 473u32, 481u32, 496u32, 445u32, 444u32, 446u32, 517u32, 524u32,
  508u32, 408u32, 402u32, 412u32, 478u32, 464u32, 360u32, 471u32, 472u32, 516u32, 439u32, 449u32,
  483u32, 469u32, 399u32, 502u32, 526u32, 425u32, 403u32, 406u32, 348u32, 350u32, 349u32, 486u32,
  487u32, 495u32, 489u32, 491u32, 488u32, 490u32, 397u32, 521u32, 522u32, 477u32, 506u32, 423u32,
  499u32, 500u32, 497u32, 485u32, 437u32, 436u32, 511u32, 434u32, 515u32, 435u32, 400u32, 520u32,
  479u32, 433u32, 427u32, 480u32, 484u32, 466u32, 465u32, 498u32, 474u32, 398u32, 358u32, 476u32,
  475u32, 482u32, 456u32, 455u32, 388u32, 405u32, 440u32, 432u32, 463u32, 438u32, 448u32, 441u32,
  144u32, 426u32, 401u32, 413u32, 442u32, 962u32, 168u32, 597u32,
];

/// 主 flag 下标 → ALL_ENTRIES 下标 (原版 kPrimaryToAllIndices, 即该主名的首个条目)
pub(crate) static PRIMARY_TO_ALL: &[u32] = &[
  0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32, 13u32, 14u32,
  15u32, 16u32, 17u32, 18u32, 19u32, 20u32, 21u32, 22u32, 23u32, 24u32, 25u32, 26u32, 27u32, 28u32,
  29u32, 30u32, 31u32, 32u32, 33u32, 34u32, 35u32, 36u32, 37u32, 38u32, 39u32, 40u32, 41u32, 42u32,
  43u32, 44u32, 45u32, 46u32, 47u32, 48u32, 49u32, 50u32, 51u32, 52u32, 53u32, 54u32, 55u32, 56u32,
  57u32, 58u32, 59u32, 60u32, 61u32, 62u32, 63u32, 64u32, 65u32, 66u32, 67u32, 68u32, 69u32, 70u32,
  71u32, 72u32, 73u32, 74u32, 75u32, 76u32, 77u32, 78u32, 79u32, 80u32, 81u32, 82u32, 83u32, 84u32,
  85u32, 86u32, 87u32, 88u32, 89u32, 90u32, 91u32, 92u32, 93u32, 94u32, 95u32, 96u32, 97u32, 98u32,
  99u32, 100u32, 101u32, 102u32, 103u32, 104u32, 105u32, 106u32, 107u32, 108u32, 109u32, 110u32,
  111u32, 112u32, 113u32, 114u32, 115u32, 116u32, 117u32, 118u32, 119u32, 120u32, 121u32, 122u32,
  123u32, 124u32, 125u32, 126u32, 127u32, 128u32, 129u32, 130u32, 131u32, 132u32, 133u32, 134u32,
  135u32, 136u32, 137u32, 138u32, 139u32, 140u32, 141u32, 142u32, 143u32, 144u32, 145u32, 146u32,
  147u32, 148u32, 149u32, 150u32, 151u32, 152u32, 153u32, 154u32, 155u32, 156u32, 157u32, 158u32,
  159u32, 160u32, 161u32, 162u32, 163u32, 164u32, 165u32, 166u32, 167u32, 168u32, 169u32, 170u32,
  171u32, 172u32, 173u32, 174u32, 175u32, 176u32, 177u32, 178u32, 179u32, 180u32, 181u32, 182u32,
  183u32, 184u32, 185u32, 186u32, 187u32, 188u32, 189u32, 190u32, 191u32, 192u32, 193u32, 194u32,
  195u32, 196u32, 197u32, 198u32, 199u32, 200u32, 201u32, 202u32, 203u32, 204u32, 205u32, 206u32,
  207u32, 208u32, 209u32, 210u32, 211u32, 212u32, 213u32, 214u32, 215u32, 216u32, 217u32, 218u32,
  219u32, 220u32, 221u32, 222u32, 223u32, 224u32, 225u32, 226u32, 227u32, 228u32, 229u32, 230u32,
  231u32, 232u32, 233u32, 234u32, 235u32, 236u32, 237u32, 238u32, 239u32, 240u32, 241u32, 242u32,
  243u32, 245u32, 246u32, 247u32, 248u32, 249u32, 250u32, 251u32, 252u32, 253u32, 254u32, 255u32,
  256u32, 257u32, 258u32, 259u32, 260u32, 261u32, 262u32, 263u32, 264u32, 265u32, 266u32, 267u32,
  268u32, 269u32, 270u32, 271u32, 272u32, 273u32, 274u32, 275u32, 276u32, 277u32, 278u32, 279u32,
  280u32, 281u32, 282u32, 283u32, 284u32, 285u32, 286u32, 287u32, 288u32, 289u32, 290u32, 291u32,
  292u32, 293u32, 294u32, 295u32, 296u32, 297u32, 298u32, 299u32, 300u32, 301u32, 302u32, 303u32,
  304u32, 305u32, 306u32, 307u32, 308u32, 309u32, 310u32, 311u32, 312u32, 313u32, 314u32, 315u32,
  316u32, 317u32, 318u32, 319u32, 320u32, 321u32, 322u32, 323u32, 324u32, 325u32, 326u32, 327u32,
  328u32, 329u32, 330u32, 331u32, 332u32, 333u32, 334u32, 335u32, 336u32, 337u32, 338u32, 339u32,
  340u32, 341u32, 342u32, 343u32, 344u32, 345u32, 346u32, 347u32, 348u32, 349u32, 350u32, 351u32,
  352u32, 353u32, 354u32, 355u32, 356u32, 357u32, 358u32, 359u32, 360u32, 361u32, 362u32, 363u32,
  364u32, 365u32, 366u32, 367u32, 368u32, 369u32, 370u32, 371u32, 372u32, 373u32, 374u32, 375u32,
  376u32, 377u32, 378u32, 379u32, 380u32, 381u32, 382u32, 383u32, 384u32, 385u32, 386u32, 387u32,
  388u32, 389u32, 390u32, 391u32, 392u32, 393u32, 394u32, 395u32, 396u32, 397u32, 398u32, 399u32,
  400u32, 401u32, 402u32, 403u32, 404u32, 405u32, 406u32, 407u32, 408u32, 409u32, 410u32, 411u32,
  412u32, 413u32, 414u32, 415u32, 416u32, 417u32, 418u32, 419u32, 420u32, 421u32, 422u32, 423u32,
  424u32, 425u32, 426u32, 427u32, 428u32, 429u32, 430u32, 431u32, 432u32, 433u32, 434u32, 435u32,
  436u32, 437u32, 438u32, 439u32, 440u32, 441u32, 442u32, 443u32, 444u32, 445u32, 446u32, 447u32,
  448u32, 449u32, 450u32, 451u32, 452u32, 453u32, 454u32, 455u32, 456u32, 457u32, 458u32, 459u32,
  460u32, 461u32, 462u32, 463u32, 464u32, 465u32, 466u32, 467u32, 468u32, 469u32, 470u32, 471u32,
  472u32, 473u32, 474u32, 475u32, 476u32, 477u32, 478u32, 479u32, 480u32, 481u32, 482u32, 483u32,
  484u32, 485u32, 486u32, 487u32, 488u32, 489u32, 490u32, 491u32, 492u32, 493u32, 494u32, 495u32,
  496u32, 497u32, 498u32, 499u32, 500u32, 501u32, 502u32, 503u32, 504u32, 505u32, 506u32, 507u32,
  508u32, 509u32, 510u32, 511u32, 512u32, 513u32, 514u32, 515u32, 516u32, 517u32, 518u32, 519u32,
  520u32, 521u32, 522u32, 523u32, 524u32, 525u32, 526u32, 527u32, 528u32, 529u32, 530u32, 531u32,
  532u32, 533u32, 534u32, 535u32, 536u32, 537u32, 538u32, 539u32, 540u32, 541u32, 542u32, 543u32,
  544u32, 545u32, 546u32, 547u32, 548u32, 549u32, 550u32, 551u32, 552u32, 553u32, 554u32, 555u32,
  556u32, 557u32, 558u32, 559u32, 560u32, 561u32, 562u32, 563u32, 564u32, 565u32, 566u32, 567u32,
  568u32, 569u32, 570u32, 571u32, 572u32, 573u32, 574u32, 575u32, 576u32, 577u32, 578u32, 579u32,
  580u32, 581u32, 582u32, 583u32, 584u32, 585u32, 586u32, 587u32, 588u32, 589u32, 590u32, 591u32,
  592u32, 593u32, 594u32, 595u32, 596u32, 597u32, 598u32, 599u32, 600u32, 601u32, 602u32, 603u32,
  604u32, 605u32, 606u32, 607u32, 608u32, 609u32, 610u32, 611u32, 612u32, 613u32, 614u32, 615u32,
  616u32, 617u32, 618u32, 619u32, 620u32, 621u32, 622u32, 623u32, 624u32, 625u32, 626u32, 627u32,
  628u32, 629u32, 630u32, 631u32, 632u32, 633u32, 634u32, 635u32, 636u32, 637u32, 638u32, 639u32,
  640u32, 641u32, 642u32, 643u32, 644u32, 645u32, 646u32, 647u32, 648u32, 649u32, 650u32, 651u32,
  652u32, 653u32, 654u32, 655u32, 656u32, 657u32, 658u32, 659u32, 660u32, 661u32, 662u32, 663u32,
  664u32, 665u32, 666u32, 667u32, 668u32, 669u32, 670u32, 671u32, 672u32, 673u32, 674u32, 675u32,
  676u32, 677u32, 678u32, 679u32, 680u32, 681u32, 682u32, 683u32, 684u32, 685u32, 686u32, 687u32,
  688u32, 689u32, 690u32, 691u32, 692u32, 693u32, 694u32, 695u32, 696u32, 697u32, 698u32, 699u32,
  700u32, 701u32, 702u32, 703u32, 704u32, 705u32, 706u32, 707u32, 708u32, 709u32, 710u32, 711u32,
  712u32, 713u32, 714u32, 715u32, 716u32, 717u32, 718u32, 719u32, 720u32, 721u32, 722u32, 723u32,
  724u32, 725u32, 726u32, 727u32, 728u32, 729u32, 730u32, 731u32, 732u32, 733u32, 734u32, 735u32,
  736u32, 737u32, 738u32, 739u32, 740u32, 741u32, 742u32, 743u32, 744u32, 745u32, 746u32, 747u32,
  748u32, 749u32, 750u32, 751u32, 752u32, 753u32, 754u32, 755u32, 756u32, 757u32, 758u32, 759u32,
  760u32, 761u32, 762u32, 763u32, 764u32, 765u32, 766u32, 767u32, 768u32, 769u32, 770u32, 771u32,
  772u32, 773u32, 774u32, 775u32, 776u32, 777u32, 778u32, 779u32, 780u32, 781u32, 782u32, 783u32,
  784u32, 785u32, 786u32, 787u32, 788u32, 789u32, 790u32, 791u32, 792u32, 793u32, 794u32, 795u32,
  796u32, 797u32, 798u32, 799u32, 800u32, 801u32, 802u32, 803u32, 804u32, 805u32, 806u32, 807u32,
  808u32, 809u32, 810u32, 811u32, 812u32, 813u32, 814u32, 815u32, 816u32, 817u32, 818u32, 819u32,
  820u32, 821u32, 822u32, 823u32, 824u32, 825u32, 826u32, 827u32, 828u32, 829u32, 830u32, 831u32,
  832u32, 833u32, 834u32, 835u32, 836u32, 837u32, 838u32, 839u32, 840u32, 841u32, 842u32, 843u32,
  844u32, 845u32, 846u32, 847u32, 848u32, 849u32, 850u32, 851u32, 852u32, 853u32, 854u32, 855u32,
  856u32, 857u32, 858u32, 859u32, 860u32, 861u32, 862u32, 863u32, 864u32, 865u32, 866u32, 867u32,
  868u32, 869u32, 870u32, 871u32, 872u32, 873u32, 874u32, 875u32, 876u32, 877u32, 878u32, 879u32,
  880u32, 881u32, 882u32, 883u32, 884u32, 885u32, 886u32, 887u32, 888u32, 889u32, 890u32, 891u32,
  892u32, 893u32, 894u32, 895u32, 896u32, 897u32, 898u32, 899u32, 900u32, 901u32, 902u32, 903u32,
  904u32, 905u32, 906u32, 907u32, 908u32, 909u32, 910u32, 911u32, 912u32, 913u32, 914u32, 915u32,
  916u32, 917u32, 918u32, 919u32, 920u32, 921u32, 922u32, 923u32, 924u32, 925u32, 926u32, 927u32,
  928u32, 929u32, 930u32, 931u32, 932u32, 933u32, 934u32, 935u32, 936u32, 937u32, 938u32, 939u32,
  940u32, 941u32, 942u32, 943u32, 944u32, 945u32, 946u32, 947u32, 948u32, 949u32, 950u32, 951u32,
  952u32, 953u32, 954u32, 955u32, 956u32, 957u32, 958u32, 959u32, 960u32, 961u32, 962u32, 963u32,
  964u32, 965u32, 966u32, 967u32, 968u32, 969u32, 970u32, 971u32, 972u32, 973u32, 974u32, 975u32,
  976u32, 977u32, 978u32, 979u32, 980u32, 981u32, 982u32, 983u32, 984u32, 985u32, 986u32, 987u32,
  988u32, 989u32, 990u32, 991u32, 992u32, 993u32, 994u32, 995u32, 996u32, 997u32, 998u32,
];

/// 归一化名 (含别名) → 主 flag 下标 (原版 FindFlagByName 精确查表; phf 编译期完美散列)
pub(crate) static NAME_MAP: phf::Map<&'static str, u32> = phf::phf_map! {
  "experimental" => 0u32,
  "array-destructure-bytecode" => 1u32,
  "test-only-unsafe" => 2u32,
  "disallow-unsafe-flags" => 3u32,
  "developer-only-features" => 4u32,
  "disallow-developer-only-features" => 5u32,
  "flag-processing-mode" => 6u32,
  "use-strict" => 7u32,
  "trace-temporal" => 8u32,
  "js-postmessage-share-immutable-arraybuffer" => 9u32,
  "builtin-subclassing" => 10u32,
  "enable-sharedarraybuffer-per-context" => 11u32,
  "js-nonextensible-applies-to-private" => 12u32,
  "for-of-optimization" => 13u32,
  "enable-queue-microtask" => 14u32,
  "icu-timezone-data" => 15u32,
  "icu-datetime-compat-lang" => 16u32,
  "intl-date-time-pattern-generator-cache-eviction" => 17u32,
  "use-icu4x-normalizer" => 18u32,
  "stress-snapshot" => 19u32,
  "lite-mode" => 20u32,
  "enable-allocation-folding" => 21u32,
  "disable-write-barriers" => 22u32,
  "enable-unconditional-write-barriers" => 23u32,
  "single-generation" => 24u32,
  "conservative-stack-scanning" => 25u32,
  "direct-handle" => 26u32,
  "scavenger-conservative-object-pinning" => 27u32,
  "stress-scavenger-conservative-object-pinning" => 28u32,
  "stress-scavenger-conservative-object-pinning-random" => 29u32,
  "scavenger-precise-object-pinning" => 30u32,
  "scavenger-updates-allocation-limit" => 31u32,
  "precise-object-pinning" => 32u32,
  "scavenger-promote-quarantined-pages" => 33u32,
  "scavenger-chaos-mode" => 34u32,
  "scavenger-chaos-mode-threshold" => 35u32,
  "local-off-stack-check" => 36u32,
  "force-emit-interrupt-budget-checks" => 37u32,
  "maglev" => 38u32,
  "maglev-future" => 39u32,
  "optimize-on-next-call-optimizes-to-maglev" => 40u32,
  "stress-maglev" => 41u32,
  "maglev-max-small-graph-size" => 42u32,
  "maglev-as-top-tier" => 43u32,
  "maglev-inlining" => 44u32,
  "maglev-disable-builtin-reducers" => 45u32,
  "maglev-loop-peeling" => 46u32,
  "maglev-optimistic-peeled-loops" => 47u32,
  "maglev-loop-peeling-max-size" => 48u32,
  "maglev-loop-peeling-max-size-cumulative" => 49u32,
  "maglev-deopt-data-on-background" => 50u32,
  "maglev-build-code-on-background" => 51u32,
  "maglev-destroy-on-background" => 52u32,
  "maglev-inline-api-calls" => 53u32,
  "maglev-cons-string-elision" => 54u32,
  "maglev-pretenure-store-values" => 55u32,
  "maglev-poly-calls" => 56u32,
  "maglev-truncation" => 57u32,
  "trace-maglev-truncation" => 58u32,
  "maglev-licm" => 59u32,
  "turbolev-truncated-int32-phis" => 60u32,
  "maglev-range-analysis" => 61u32,
  "trace-maglev-range-analysis" => 62u32,
  "trace-maglev-kna" => 63u32,
  "maglev-use-unreliable-maps" => 64u32,
  "maglev-range-verification" => 65u32,
  "maglev-verify-dominance" => 66u32,
  "concurrent-maglev-max-threads" => 67u32,
  "concurrent-maglev-high-priority-threads" => 68u32,
  "max-maglev-inline-depth" => 69u32,
  "max-maglev-hard-inline-depth" => 70u32,
  "max-maglev-inlined-bytecode-size" => 71u32,
  "max-maglev-inlined-bytecode-size-cumulative" => 72u32,
  "max-maglev-inlined-bytecode-size-small-total" => 73u32,
  "max-maglev-inlined-bytecode-size-small" => 74u32,
  "max-maglev-inlined-bytecode-size-small-with-heapnum-in-out" => 75u32,
  "min-maglev-inlining-frequency" => 76u32,
  "max-turbolev-inline-depth" => 77u32,
  "maglev-reuse-stack-slots" => 78u32,
  "maglev-untagged-phis" => 79u32,
  "maglev-untagged-phis-bisect-limit" => 80u32,
  "turbolev-untagged-phis" => 81u32,
  "maglev-hoist-osr-value-phi-untagging" => 82u32,
  "maglev-speculative-hoist-phi-untagging" => 83u32,
  "maglev-cse" => 84u32,
  "maglev-non-eager-inlining" => 85u32,
  "turbolev-non-eager-inlining" => 86u32,
  "turbolev-non-eager-loop-peeling" => 87u32,
  "turbolev-eager-loop-peeling-osr" => 88u32,
  "turbolev-trace-loop-peeling" => 89u32,
  "maglev-filter" => 90u32,
  "maglev-print-filter" => 91u32,
  "maglev-assert" => 92u32,
  "maglev-assert-stack-size" => 93u32,
  "maglev-assert-types" => 94u32,
  "maglev-break-on-entry" => 95u32,
  "maglev-print-bytecode" => 96u32,
  "maglev-print-feedback" => 97u32,
  "maglev-print-inlined" => 98u32,
  "maglev-print-provenance" => 99u32,
  "print-maglev-code" => 100u32,
  "trace-with-compilation-id" => 101u32,
  "trace-maglev-graph-building" => 102u32,
  "trace-maglev-graph-optimizer" => 103u32,
  "trace-maglev-loop-speeling" => 104u32,
  "trace-maglev-inlining" => 105u32,
  "trace-maglev-kna-processor" => 106u32,
  "trace-maglev-scope-info" => 107u32,
  "trace-maglev" => 108u32,
  "trace-maglev-file-prefix" => 109u32,
  "print-maglev-deopt-verbose" => 110u32,
  "print-maglev-graph" => 111u32,
  "print-maglev-graphs" => 112u32,
  "trace-maglev-phi-untagging" => 113u32,
  "trace-maglev-regalloc" => 114u32,
  "maglev-stats" => 115u32,
  "maglev-stats-nvp" => 116u32,
  "maglev-function-context-specialization" => 117u32,
  "additive-safe-int-feedback" => 118u32,
  "turbolev-additive-safe-int-feedback" => 119u32,
  "turbofan-additive-safe-int-feedback" => 120u32,
  "enable-enumerated-keyed-access-bytecode" => 121u32,
  "dict-property-const-tracking" => 122u32,
  "script-context-cells" => 123u32,
  "function-context-cells" => 124u32,
  "function-context-cells-max-size" => 125u32,
  "empty-context-extension-dep" => 126u32,
  "json-stringify-fast-path" => 127u32,
  "json-parse-max-heuristically-internalized-strings" => 128u32,
  "trace-json-parse-internalization" => 129u32,
  "cache-property-key-string-adds" => 130u32,
  "smi-string-cache-size" => 131u32,
  "double-string-cache-size" => 132u32,
  "trace-number-string-cache" => 133u32,
  "maglev-inline-date-accessors" => 134u32,
  "turbofan-inline-date-accessors" => 135u32,
  "inline-date-accessors" => 136u32,
  "extensible-ro-snapshot" => 137u32,
  "max-opt" => 138u32,
  "efficiency-mode" => 139u32,
  "battery-saver-mode" => 140u32,
  "memory-saver-mode" => 141u32,
  "efficiency-mode-disable-turbofan" => 142u32,
  "efficiency-mode-delay-turbofan-multiply" => 143u32,
  "wasm-trace-native" => 144u32,
  "jitless" => 145u32,
  "disable-optimizing-compilers" => 146u32,
  "memory-protection-keys" => 147u32,
  "force-memory-protection-keys" => 148u32,
  "assert-types" => 149u32,
  "turboshaft-assert-types" => 150u32,
  "verify-simplified-lowering" => 151u32,
  "trace-compilation-dependencies" => 152u32,
  "unmap-holes" => 153u32,
  "allocation-site-tracking" => 154u32,
  "allocation-site-pretenuring" => 155u32,
  "page-promotion" => 156u32,
  "page-promotion-threshold" => 157u32,
  "minor-ms-page-promotion-threshold" => 158u32,
  "minor-ms-page-promotion-max-lab-threshold" => 159u32,
  "minor-ms-max-page-age" => 160u32,
  "trace-page-promotions" => 161u32,
  "trace-pretenuring" => 162u32,
  "trace-pretenuring-statistics" => 163u32,
  "trace-resize-large-object" => 164u32,
  "track-field-types" => 165u32,
  "trace-block-coverage" => 166u32,
  "trace-protector-invalidation" => 167u32,
  "zero-unused-memory" => 168u32,
  "high-end-android" => 169u32,
  "high-end-android-physical-memory-threshold" => 170u32,
  "minor-ms-shortcut-strings" => 171u32,
  "feedback-normalization" => 172u32,
  "internalize-on-the-fly" => 173u32,
  "unbox-double-arrays" => 174u32,
  "string-slices" => 175u32,
  "invocation-count-for-feedback-allocation" => 176u32,
  "invocation-count-for-maglev" => 177u32,
  "invocation-count-for-maglev-osr" => 178u32,
  "osr-from-maglev" => 179u32,
  "osr-from-maglev-interrupt-scale-factor" => 180u32,
  "always-osr-from-maglev" => 181u32,
  "invocation-count-for-turbofan" => 182u32,
  "invocation-count-for-osr" => 183u32,
  "invocation-count-for-osr-factor-while-tiering-in-progress" => 184u32,
  "osr-to-tierup" => 185u32,
  "minimum-invocations-after-ic-update" => 186u32,
  "minimum-invocations-before-optimization" => 187u32,
  "jit-fuzzing" => 188u32,
  "use-std-math-pow" => 189u32,
  "use-ic" => 190u32,
  "lazy-feedback-allocation" => 191u32,
  "stress-ic" => 192u32,
  "ignition-elide-noneffectful-bytecodes" => 193u32,
  "ignition-reo" => 194u32,
  "ignition-filter-expression-positions" => 195u32,
  "ignition-share-named-property-feedback" => 196u32,
  "ignition-elide-redundant-tdz-checks" => 197u32,
  "print-bytecode" => 198u32,
  "enable-lazy-source-positions" => 199u32,
  "stress-lazy-source-positions" => 200u32,
  "stress-lazy" => 201u32,
  "print-bytecode-filter" => 202u32,
  "omit-default-ctors" => 203u32,
  "trace-ignition-codegen" => 204u32,
  "trace-track-allocation-sites" => 205u32,
  "trace-migration" => 206u32,
  "trace-generalization" => 207u32,
  "sparkplug" => 208u32,
  "always-sparkplug" => 209u32,
  "baseline-batch-compilation" => 210u32,
  "concurrent-sparkplug" => 211u32,
  "concurrent-sparkplug-max-threads" => 212u32,
  "concurrent-sparkplug-high-priority-threads" => 213u32,
  "sparkplug-filter" => 214u32,
  "sparkplug-needs-short-builtins" => 215u32,
  "baseline-batch-compilation-threshold" => 216u32,
  "trace-baseline" => 217u32,
  "trace-baseline-batch-compilation" => 218u32,
  "shared-strings" => 219u32,
  "shared-string-table" => 220u32,
  "always-use-string-forwarding-table" => 221u32,
  "transition-strings-during-gc-with-stack" => 222u32,
  "initial-shared-heap-size" => 223u32,
  "max-shared-heap-size" => 224u32,
  "concurrent-builtin-generation" => 225u32,
  "concurrent-recompilation" => 226u32,
  "trace-concurrent-recompilation" => 227u32,
  "concurrent-recompilation-queue-length" => 228u32,
  "concurrent-recompilation-delay" => 229u32,
  "concurrent-recompilation-front-running" => 230u32,
  "concurrent-turbofan-max-threads" => 231u32,
  "stress-concurrent-inlining" => 232u32,
  "maglev-overwrite-budget" => 233u32,
  "maglev-overwrite-osr-budget" => 234u32,
  "stress-concurrent-inlining-attach-code" => 235u32,
  "max-serializer-nesting" => 236u32,
  "trace-heap-broker-verbose" => 237u32,
  "trace-heap-broker" => 238u32,
  "stress-runs" => 239u32,
  "deopt-every-n-times" => 240u32,
  "print-deopt-stress" => 241u32,
  "disable-loop-stack-checks" => 242u32,
  "turbofan" => 243u32,
  "opt" => 243u32,
  "turbo-always-optimize-spills" => 244u32,
  "turbo-wasm-address-reassociation" => 245u32,
  "concurrent-turbo-tracing" => 246u32,
  "optimize-maglev-optimizes-to-turbofan" => 247u32,
  "turbo-filter" => 248u32,
  "trace-turbo" => 249u32,
  "trace-turbo-path" => 250u32,
  "trace-turbo-filter" => 251u32,
  "trace-turbo-graph" => 252u32,
  "trace-turbo-scheduled" => 253u32,
  "trace-turbo-file-prefix" => 254u32,
  "trace-turbo-cfg-file" => 255u32,
  "trace-turbo-types" => 256u32,
  "trace-turbo-scheduler" => 257u32,
  "trace-turbo-reduction" => 258u32,
  "trace-turbo-bailouts" => 259u32,
  "trace-turbo-trimming" => 260u32,
  "trace-turbo-jt" => 261u32,
  "trace-turbo-ceq" => 262u32,
  "trace-turbo-loop" => 263u32,
  "trace-turbo-alloc" => 264u32,
  "trace-all-uses" => 265u32,
  "trace-representation" => 266u32,
  "trace-turbo-stack-accesses" => 267u32,
  "fuzzing-and-concurrent-recompilation" => 268u32,
  "turbo-verify" => 269u32,
  "turbo-verify-machine-graph" => 270u32,
  "verify-csa" => 271u32,
  "trace-verify-csa" => 272u32,
  "csa-trap-on-node" => 273u32,
  "builtins-effects-src" => 274u32,
  "fixed-array-bounds-checks" => 275u32,
  "turbo-stats" => 276u32,
  "turbo-stats-nvp" => 277u32,
  "turbo-stats-wasm" => 278u32,
  "turbo-splitting" => 279u32,
  "turbo-inlining" => 280u32,
  "forin-enumerator-holder" => 281u32,
  "turbo-elide-frames" => 282u32,
  "max-inlined-bytecode-size" => 283u32,
  "max-inlined-bytecode-size-cumulative" => 284u32,
  "max-inlined-bytecode-size-absolute" => 285u32,
  "max-inlined-bytecode-size-small-total" => 286u32,
  "reserve-inline-budget-scale-factor" => 287u32,
  "max-inlined-bytecode-size-small" => 288u32,
  "max-inlined-bytecode-size-small-with-heapnum-in-out" => 289u32,
  "max-optimized-bytecode-size" => 290u32,
  "max-maglev-optimized-bytecode-size" => 291u32,
  "min-inlining-frequency" => 292u32,
  "stress-inline" => 293u32,
  "trace-turbo-inlining" => 294u32,
  "turbo-inline-array-builtins" => 295u32,
  "use-osr" => 296u32,
  "maglev-osr" => 297u32,
  "max-maglev-eager-inlined-bytecode-size" => 298u32,
  "max-turbolev-eager-inlined-bytecode-size" => 299u32,
  "concurrent-osr" => 300u32,
  "maglev-allocation-folding" => 301u32,
  "maglev-escape-analysis" => 302u32,
  "trace-maglev-escape-analysis" => 303u32,
  "maglev-object-tracking" => 304u32,
  "trace-maglev-object-tracking" => 305u32,
  "turbo-string-builder" => 306u32,
  "trace-osr" => 307u32,
  "log-or-trace-osr" => 308u32,
  "analyze-environment-liveness" => 309u32,
  "trace-environment-liveness" => 310u32,
  "turbo-load-elimination" => 311u32,
  "trace-turbo-load-elimination" => 312u32,
  "turbo-profiling" => 313u32,
  "turbo-profiling-verbose" => 314u32,
  "turbo-profiling-output" => 315u32,
  "reorder-builtins" => 316u32,
  "abort-on-bad-builtin-profile-data" => 317u32,
  "warn-about-builtin-profile-data" => 318u32,
  "dump-builtins-hashes-to-file" => 319u32,
  "turbo-verify-allocation" => 320u32,
  "turbo-move-optimization" => 321u32,
  "turbo-jt" => 322u32,
  "turbo-loop-peeling" => 323u32,
  "turbo-loop-variable" => 324u32,
  "turbo-loop-rotation" => 325u32,
  "turbo-cf-optimization" => 326u32,
  "turbo-escape" => 327u32,
  "turbo-allocation-folding" => 328u32,
  "turbo-instruction-scheduling" => 329u32,
  "experimental-turbo-instruction-scheduling" => 330u32,
  "turbo-stress-instruction-scheduling" => 331u32,
  "trace-turbo-instruction-scheduling" => 332u32,
  "turbo-store-elimination" => 333u32,
  "trace-store-elimination" => 334u32,
  "turbo-typer-hardening" => 335u32,
  "turbo-rewrite-far-jumps" => 336u32,
  "stress-gc-during-compilation" => 337u32,
  "turbo-fast-api-calls" => 338u32,
  "fast-api-allow-float-in-sim" => 339u32,
  "turbo-inline-js-wasm-calls" => 340u32,
  "turbo-optimize-inlined-js-wasm-wrappers" => 341u32,
  "turbo-optimize-apply" => 342u32,
  "turbo-optimize-math-minmax" => 343u32,
  "turboshaft" => 344u32,
  "turboshaft-enable-debug-features" => 345u32,
  "turboshaft-wasm-load-elimination" => 346u32,
  "wasm-in-js-inlining-body" => 347u32,
  "wasm-in-js-inlining-wrapper" => 348u32,
  "wasm-in-js-inlining-opt" => 349u32,
  "turboshaft-load-elimination" => 350u32,
  "turboshaft-loop-unrolling" => 351u32,
  "turboshaft-loop-optimization" => 352u32,
  "turboshaft-random-rescheduling" => 353u32,
  "turboshaft-string-concat-escape-analysis" => 354u32,
  "turboshaft-trusted-load-elimination" => 355u32,
  "turboshaft-typed-optimizations" => 356u32,
  "wasm-simd-opt" => 357u32,
  "future-wasm-simd-opt" => 358u32,
  "wasm-deinterleave-loads" => 359u32,
  "turbolev" => 360u32,
  "print-turbolev-frontend" => 361u32,
  "turbolev-phase-filter" => 362u32,
  "print-turbolev-inline-functions" => 363u32,
  "turbolev-future" => 364u32,
  "turbolev-escape-analysis" => 365u32,
  "trace-turbolev-escape-analysis" => 366u32,
  "typed-array-length-loading" => 367u32,
  "deduplicate-heap-number-requests" => 368u32,
  "turboshaft-trace-reduction" => 369u32,
  "turboshaft-trace-emitted" => 370u32,
  "turboshaft-trace-intermediate-reductions" => 371u32,
  "turboshaft-trace-load-elimination" => 372u32,
  "turboshaft-trace-store-store-elimination" => 373u32,
  "turboshaft-trace-if-else-to-switch" => 374u32,
  "verify-turboshaft" => 375u32,
  "profile-guided-optimization" => 376u32,
  "profile-guided-optimization-for-empty-feedback-vector" => 377u32,
  "invocation-count-for-early-optimization" => 378u32,
  "invocation-count-for-maglev-with-delay" => 379u32,
  "optimize-for-size" => 380u32,
  "reopt-after-lazy-deopts" => 381u32,
  "verify-get-js-builtin-state" => 382u32,
  "experimental-fuzzing" => 383u32,
  "future" => 384u32,
  "harmony" => 385u32,
  "js-staging" => 386u32,
  "wasm-staging" => 387u32,
  "js-shipping" => 388u32,
  "harmony-shipping" => 389u32,
  "harmony-shadow-realm" => 390u32,
  "harmony-struct" => 391u32,
  "harmony-intl-best-fit-matcher" => 392u32,
  "js-decorators" => 393u32,
  "js-source-phase-imports" => 394u32,
  "js-regexp-buffer-boundaries" => 395u32,
  "wasm-instruction-tracing" => 396u32,
  "wasm-shared" => 397u32,
  "wasm-fp16" => 398u32,
  "wasm-memory-control" => 399u32,
  "wasm-wasmfx" => 400u32,
  "wasm-compilation-hints" => 401u32,
  "wasm-growable-stacks" => 402u32,
  "always-specialize-for-script-context" => 403u32,
  "wasm-stringref" => 404u32,
  "wasm-imported-strings-utf8" => 405u32,
  "wasm-acquire-release" => 406u32,
  "wasm-compact-imports" => 407u32,
  "js-immutable-arraybuffer" => 408u32,
  "js-import-bytes" => 409u32,
  "js-defer-import-eval" => 410u32,
  "wasm-custom-descriptors" => 411u32,
  "wasm-wide-arithmetic" => 412u32,
  "harmony-import-attributes" => 413u32,
  "harmony-temporal" => 414u32,
  "js-esm-ns-reexport" => 415u32,
  "js-iterator-join" => 416u32,
  "js-iterator-sequencing" => 417u32,
  "js-joint-iteration" => 418u32,
  "js-pr-3883" => 419u32,
  "js-iterator-includes" => 420u32,
  "js-import-text" => 421u32,
  "wasm-legacy-eh" => 422u32,
  "homomorphic-ic" => 423u32,
  "wasm-generic-wrapper" => 424u32,
  "wasm-unsafe-fast-api-wrapper" => 425u32,
  "wasm-num-compilation-tasks" => 426u32,
  "trace-wasm-native-heap" => 427u32,
  "trace-wasm-offheap-memory" => 428u32,
  "print-wasm-offheap-memory-size" => 429u32,
  "trace-wasm-serialization" => 430u32,
  "wasm-test-streaming" => 431u32,
  "wasm-native-module-cache" => 432u32,
  "wasm-max-mem-pages" => 433u32,
  "wasm-max-table-size" => 434u32,
  "wasm-max-committed-code-mb" => 435u32,
  "wasm-max-code-space-size-mb" => 436u32,
  "wasm-tier-up" => 437u32,
  "wasm-dynamic-tiering" => 438u32,
  "wasm-sync-tier-up" => 439u32,
  "wasm-tiering-budget" => 440u32,
  "wasm-wrapper-tiering-budget" => 441u32,
  "max-wasm-functions" => 442u32,
  "wasm-caching-threshold" => 443u32,
  "wasm-caching-hard-threshold" => 444u32,
  "wasm-caching-timeout-ms" => 445u32,
  "trace-wasm-compilation-times" => 446u32,
  "wasm-tier-up-filter" => 447u32,
  "wasm-eager-tier-up-function" => 448u32,
  "trace-wasm-decoder" => 449u32,
  "trace-wasm-compiler" => 450u32,
  "trace-wasm-streaming" => 451u32,
  "trace-wasm-stack-switching" => 452u32,
  "stress-wasm-stack-switching" => 453u32,
  "wasm-stack-switching-stack-size" => 454u32,
  "wasm-stack-pool-capacity-mb" => 455u32,
  "liftoff" => 456u32,
  "liftoff-only" => 457u32,
  "enable-testing-opcode-in-wasm" => 458u32,
  "trace-liftoff" => 459u32,
  "trace-wasm-memory" => 460u32,
  "trace-wasm-globals" => 461u32,
  "wasm-tier-mask-for-testing" => 462u32,
  "wasm-debug-mask-for-testing" => 463u32,
  "wasm-pgo-to-file" => 464u32,
  "wasm-pgo-from-file" => 465u32,
  "dump-wasm-module" => 466u32,
  "dump-wasm-module-path" => 467u32,
  "wasm-fast-api" => 468u32,
  "wasm-assert-types" => 469u32,
  "wasm-deopt" => 470u32,
  "wasm-deopts-per-function-limit" => 471u32,
  "wasm-assume-ref-cast-succeeds" => 472u32,
  "wasm-ref-cast-nop" => 473u32,
  "wasm-skip-null-checks" => 474u32,
  "wasm-skip-bounds-checks" => 475u32,
  "wasm-js-interop" => 476u32,
  "wasm-custom-descriptors-permitted" => 477u32,
  "wasm-merged-descriptors" => 478u32,
  "wasm-opt" => 479u32,
  "wasm-bounds-checks" => 480u32,
  "wasm-stack-checks" => 481u32,
  "wasm-enforce-bounds-checks" => 482u32,
  "wasm-partial-oob-writes-are-noops" => 483u32,
  "wasm-math-intrinsics" => 484u32,
  "wasm-inlining" => 485u32,
  "wasm-inlining-budget" => 486u32,
  "wasm-inlining-max-size" => 487u32,
  "wasm-inlining-factor" => 488u32,
  "wasm-inlining-min-budget" => 489u32,
  "wasm-inlining-ignore-call-counts" => 490u32,
  "trace-wasm-inlining" => 491u32,
  "trace-wasm-typer" => 492u32,
  "trace-wasm-simd-shuffle" => 493u32,
  "wasm-inlining-call-indirect" => 494u32,
  "wasm-bulkmem-inlining" => 495u32,
  "wasm-loop-unrolling" => 496u32,
  "wasm-random-rescheduling" => 497u32,
  "wasm-loop-peeling" => 498u32,
  "wasm-loop-peeling-max-size" => 499u32,
  "trace-wasm-loop-peeling" => 500u32,
  "wasm-fuzzer-gen-test" => 501u32,
  "print-wasm-code" => 502u32,
  "print-wasm-code-function-index" => 503u32,
  "print-wasm-stub-code" => 504u32,
  "wasm-lazy-compilation" => 505u32,
  "trace-wasm-lazy-compilation" => 506u32,
  "wasm-code-gc" => 507u32,
  "trace-wasm-code-gc" => 508u32,
  "stress-wasm-code-gc" => 509u32,
  "wasm-max-initial-code-space-reservation" => 510u32,
  "stress-wasm-memory-moving" => 511u32,
  "flush-liftoff-code" => 512u32,
  "stress-branch-hinting" => 513u32,
  "wasm-max-module-size" => 514u32,
  "wasm-disassembly-max-mb" => 515u32,
  "wasm-capi-thread-pool-size" => 516u32,
  "trace-wasm" => 517u32,
  "trace-wasm-instances" => 518u32,
  "wasm-memory64-trap-handling" => 519u32,
  "wasm-jitless" => 520u32,
  "wasm-jitless-if-available-for-testing" => 521u32,
  "wasm-allow-mixed-eh-for-testing" => 522u32,
  "wasm-code-coverage" => 523u32,
  "trace-wasm-compilation-hints" => 524u32,
  "wasm-generate-compilation-hints" => 525u32,
  "trace-wasm-generate-compilation-hints" => 526u32,
  "stress-sampling-allocation-profiler" => 527u32,
  "lazy-new-space-shrinking" => 528u32,
  "min-semi-space-size" => 529u32,
  "max-semi-space-size" => 530u32,
  "semi-space-growth-factor" => 531u32,
  "max-old-space-size" => 532u32,
  "max-heap-size" => 533u32,
  "initial-heap-size" => 534u32,
  "initial-old-space-size" => 535u32,
  "preconfigured-old-space-size" => 536u32,
  "gc-global" => 537u32,
  "random-gc-interval" => 538u32,
  "gc-interval" => 539u32,
  "cppgc-random-gc-interval" => 540u32,
  "retain-maps-for-n-gc" => 541u32,
  "trace-gc" => 542u32,
  "trace-gc-nvp" => 543u32,
  "trace-gc-ignore-scavenger" => 544u32,
  "trace-memory-reducer" => 545u32,
  "trace-gc-verbose" => 546u32,
  "trace-gc-freelists" => 547u32,
  "trace-gc-freelists-verbose" => 548u32,
  "trace-gc-heap-layout" => 549u32,
  "trace-gc-heap-layout-ignore-minor-gc" => 550u32,
  "trace-evacuation-candidates" => 551u32,
  "trace-pending-allocations" => 552u32,
  "trace-allocation-stack-interval" => 553u32,
  "trace-duplicate-threshold-kb" => 554u32,
  "trace-fragmentation" => 555u32,
  "trace-fragmentation-verbose" => 556u32,
  "minor-ms-trace-fragmentation" => 557u32,
  "trace-evacuation" => 558u32,
  "trace-mutator-utilization" => 559u32,
  "incremental-marking" => 560u32,
  "incremental-marking-task" => 561u32,
  "incremental-marking-soft-trigger" => 562u32,
  "incremental-marking-hard-trigger" => 563u32,
  "incremental-marking-unified-schedule" => 564u32,
  "trace-unmapper" => 565u32,
  "parallel-scavenge" => 566u32,
  "minor-gc-task" => 567u32,
  "minor-gc-task-trigger" => 568u32,
  "minor-gc-task-with-lower-priority" => 569u32,
  "trace-parallel-scavenge" => 570u32,
  "cppgc-young-generation" => 571u32,
  "optimize-gc-for-battery" => 572u32,
  "concurrent-marking" => 573u32,
  "concurrent-marking-max-worker-num" => 574u32,
  "concurrent-array-buffer-sweeping" => 575u32,
  "stress-concurrent-allocation" => 576u32,
  "parallel-marking" => 577u32,
  "ephemeron-fixpoint-iterations" => 578u32,
  "trace-concurrent-marking" => 579u32,
  "concurrent-sweeping" => 580u32,
  "parallel-compaction" => 581u32,
  "parallel-pointer-update" => 582u32,
  "parallel-gc-clearing" => 583u32,
  "detect-ineffective-gcs-near-heap-limit" => 584u32,
  "enforce-global-heap-limit" => 585u32,
  "maximum-global-heap-limit-factor" => 586u32,
  "ineffective-gc-size-threshold" => 587u32,
  "ineffective-gc-mutator-utilization-threshold" => 588u32,
  "trace-incremental-marking" => 589u32,
  "print-gc-clearing-dependency-graph" => 590u32,
  "trace-stress-marking" => 591u32,
  "trace-stress-scavenge" => 592u32,
  "track-gc-object-stats" => 593u32,
  "trace-gc-object-stats" => 594u32,
  "trace-zone-stats" => 595u32,
  "zone-stats-tolerance" => 596u32,
  "trace-zone-type-stats" => 597u32,
  "trace-backing-store" => 598u32,
  "gc-stats" => 599u32,
  "trace-gc-object-stats-all-objects" => 600u32,
  "track-detached-contexts" => 601u32,
  "trace-detached-contexts" => 602u32,
  "verify-heap" => 603u32,
  "verify-write-barriers" => 604u32,
  "safepoint-bump-qos-class" => 605u32,
  "freeze-forces-marking-finalization" => 606u32,
  "move-object-start" => 607u32,
  "memory-reducer" => 608u32,
  "memory-reducer-for-small-heaps" => 609u32,
  "memory-reducer-gc-count" => 610u32,
  "memory-reducer-delay-ms" => 611u32,
  "gc-memory-reducer-start-delay-ms" => 612u32,
  "memory-reducer-limit-based" => 613u32,
  "external-memory-max-growing-factor" => 614u32,
  "external-memory-max-reasonable-size" => 615u32,
  "sqrt-allocation-limits" => 616u32,
  "sqrt-allocation-limits-factor" => 617u32,
  "sqrt-allocation-limits-slow-factor" => 618u32,
  "sqrt-allocation-limits-minimal-factor" => 619u32,
  "sqrt-allocation-limits-min-growing-factor" => 620u32,
  "sqrt-allocation-limits-max-growing-factor" => 621u32,
  "gc-speed-uses-counters" => 622u32,
  "heap-growing-percent" => 623u32,
  "v8-os-page-size" => 624u32,
  "allocation-buffer-parking" => 625u32,
  "compact" => 626u32,
  "compact-code-space" => 627u32,
  "compact-on-every-full-gc" => 628u32,
  "compaction-on-regular-gcs" => 629u32,
  "compact-with-stack" => 630u32,
  "compaction-max-evacuated-bytes-mb" => 631u32,
  "compaction-max-evacuated-bytes-mb-for-reduce-memory" => 632u32,
  "compaction-max-evacuated-bytes-mb-for-optimize-memory" => 633u32,
  "compaction-target-fragmentation-percent-for-reduce-memory" => 634u32,
  "compaction-target-fragmentation-percent-for-optimize-memory" => 635u32,
  "shortcut-strings-with-stack" => 636u32,
  "stress-compaction" => 637u32,
  "resize-large-object" => 638u32,
  "stress-compaction-random" => 639u32,
  "stress-descriptor-array-trimming" => 640u32,
  "flush-baseline-code" => 641u32,
  "flush-bytecode" => 642u32,
  "bytecode-old-time" => 643u32,
  "stress-flush-code" => 644u32,
  "trace-flush-code" => 645u32,
  "use-marking-progress-bar" => 646u32,
  "fixed-margin-for-input-handling" => 647u32,
  "optimize-for-input-handling" => 648u32,
  "stress-per-context-marking-worklist" => 649u32,
  "stress-incremental-marking" => 650u32,
  "memory-pool" => 651u32,
  "memory-pool-share-memory-on-teardown" => 652u32,
  "memory-pool-release-on-malloc-failures" => 653u32,
  "memory-pool-timeout" => 654u32,
  "large-page-pool" => 655u32,
  "max-large-page-pool-size" => 656u32,
  "managed-zone-memory" => 657u32,
  "managed-zone-memory-for-isolate-independent-memory" => 658u32,
  "fuzzer-gc-analysis" => 659u32,
  "stress-marking" => 660u32,
  "stress-scavenge" => 661u32,
  "reclaim-unmodified-wrappers" => 662u32,
  "parallel-reclaim-unmodified-wrappers" => 663u32,
  "concurrent-marking-high-priority-threads" => 664u32,
  "disable-abortjs" => 665u32,
  "randomize-all-allocations" => 666u32,
  "manual-evacuation-candidates-selection" => 667u32,
  "clear-free-memory" => 668u32,
  "idle-gc-on-context-disposal" => 669u32,
  "trace-context-disposal" => 670u32,
  "cppheap-incremental-marking" => 671u32,
  "cppheap-concurrent-marking" => 672u32,
  "memory-balancer" => 673u32,
  "memory-balancer-c-value" => 674u32,
  "trace-memory-balancer" => 675u32,
  "late-heap-limit-check" => 676u32,
  "debug-code" => 677u32,
  "trap-on-abort" => 678u32,
  "slow-debug-code" => 679u32,
  "code-comments" => 680u32,
  "enable-sse3" => 681u32,
  "enable-ssse3" => 682u32,
  "enable-sse4-1" => 683u32,
  "enable-sse4-2" => 684u32,
  "enable-sahf" => 685u32,
  "enable-avx" => 686u32,
  "enable-avx2" => 687u32,
  "enable-avx-vnni" => 688u32,
  "enable-avx-vnni-int8" => 689u32,
  "enable-fma3" => 690u32,
  "enable-f16c" => 691u32,
  "enable-bmi1" => 692u32,
  "enable-bmi2" => 693u32,
  "enable-lzcnt" => 694u32,
  "enable-popcnt" => 695u32,
  "arm-arch" => 696u32,
  "force-long-branches" => 697u32,
  "mcpu" => 698u32,
  "partial-constant-pool" => 699u32,
  "sim-arm64-optional-features" => 700u32,
  "sim-arm64-tbi" => 701u32,
  "intel-jcc-erratum-mitigation" => 702u32,
  "enable-source-at-csa-bind" => 703u32,
  "enable-armv7" => 704u32,
  "enable-vfp3" => 705u32,
  "enable-32dregs" => 706u32,
  "enable-neon" => 707u32,
  "enable-sudiv" => 708u32,
  "enable-armv8" => 709u32,
  "enable-regexp-unaligned-accesses" => 710u32,
  "script-streaming" => 711u32,
  "stress-background-compile" => 712u32,
  "concurrent-cache-deserialization" => 713u32,
  "merge-background-deserialized-script-with-compilation-cache" => 714u32,
  "verify-code-merge" => 715u32,
  "expose-gc" => 716u32,
  "expose-gc-as" => 717u32,
  "expose-externalize-string" => 718u32,
  "expose-statistics" => 719u32,
  "expose-trigger-failure" => 720u32,
  "stack-trace-limit" => 721u32,
  "builtins-in-stack-traces" => 722u32,
  "experimental-stack-trace-frames" => 723u32,
  "disallow-code-generation-from-strings" => 724u32,
  "expose-async-hooks" => 725u32,
  "expose-cputracemark-as" => 726u32,
  "experimental-report-exceptions-from-callbacks" => 727u32,
  "allow-unsafe-function-constructor" => 728u32,
  "force-slow-path" => 729u32,
  "test-small-max-function-context-stub-size" => 730u32,
  "fast-api-indexof" => 731u32,
  "fast-api-iterable-to-list" => 732u32,
  "inline-new" => 733u32,
  "switch-table-spread-threshold" => 734u32,
  "switch-table-min-cases" => 735u32,
  "stress-lazy-compilation" => 736u32,
  "trace" => 737u32,
  "lazy" => 738u32,
  "lazy-eval" => 739u32,
  "lazy-streaming" => 740u32,
  "max-lazy" => 741u32,
  "compile-hints-magic" => 742u32,
  "trace-opt" => 743u32,
  "trace-opt-status" => 744u32,
  "trace-opt-verbose" => 745u32,
  "trace-opt-stats" => 746u32,
  "trace-deopt" => 747u32,
  "log-deopt" => 748u32,
  "trace-deopt-verbose" => 749u32,
  "trace-file-names" => 750u32,
  "always-osr" => 751u32,
  "trace-serializer" => 752u32,
  "compilation-cache" => 753u32,
  "cache-prototype-transitions" => 754u32,
  "lazy-compile-dispatcher" => 755u32,
  "lazy-compile-dispatcher-max-threads" => 756u32,
  "trace-compiler-dispatcher" => 757u32,
  "parallel-compile-tasks-for-eager-toplevel" => 758u32,
  "parallel-compile-tasks-for-lazy" => 759u32,
  "cpu-profiler-sampling-interval" => 760u32,
  "simulator-debugger" => 761u32,
  "trace-side-effect-free-debug-evaluate" => 762u32,
  "hard-abort" => 763u32,
  "log-colour" => 764u32,
  "expose-inspector-scripts" => 765u32,
  "stack-size" => 766u32,
  "max-stack-trace-source-length" => 767u32,
  "clear-exceptions-on-js-entry" => 768u32,
  "use-original-message-for-stack-trace" => 769u32,
  "histogram-interval" => 770u32,
  "heap-profiler-trace-objects" => 771u32,
  "heap-profiler-use-embedder-graph" => 772u32,
  "heap-snapshot-on-oom" => 773u32,
  "heap-snapshot-path" => 774u32,
  "heap-snapshot-on-gc" => 775u32,
  "heap-snapshot-string-limit" => 776u32,
  "profile-heap-snapshot" => 777u32,
  "sampling-heap-profiler-suppress-randomness" => 778u32,
  "log-ic" => 779u32,
  "fast-map-update" => 780u32,
  "max-valid-polymorphic-map-count" => 781u32,
  "clone-object-sidestep-transitions" => 782u32,
  "fast-properties-soft-limit" => 783u32,
  "native-code-counters" => 784u32,
  "sparkplug-plus" => 785u32,
  "sparkplug-inline-smi" => 786u32,
  "super-ic" => 787u32,
  "mega-dom-ic" => 788u32,
  "homomorphic-ic-count" => 789u32,
  "trace-prototype-users" => 790u32,
  "trace-for-in-enumerate" => 791u32,
  "log-maps" => 792u32,
  "log-maps-details" => 793u32,
  "allow-natives-syntax" => 794u32,
  "allow-natives-for-differential-fuzzing" => 795u32,
  "parse-only" => 796u32,
  "sim-abort-on-bad-auth" => 797u32,
  "async-stack-traces" => 798u32,
  "stack-trace-on-illegal" => 799u32,
  "abort-on-uncaught-exception" => 800u32,
  "correctness-fuzzer-suppressions" => 801u32,
  "correctness-fuzzer-cross-arch-suppressions" => 802u32,
  "rehash-snapshot" => 803u32,
  "hash-seed" => 804u32,
  "random-seed" => 805u32,
  "fuzzer-random-seed" => 806u32,
  "trace-rail" => 807u32,
  "print-all-exceptions" => 808u32,
  "adjust-os-scheduling-parameters" => 809u32,
  "experimental-flush-embedded-blob-icache" => 810u32,
  "short-builtin-calls" => 811u32,
  "trace-code-range-allocation" => 812u32,
  "better-code-range-allocation" => 813u32,
  "abort-on-far-code-range" => 814u32,
  "runtime-call-stats" => 815u32,
  "rcs" => 816u32,
  "rcs-cpu-time" => 817u32,
  "verify-snapshot-checksum" => 818u32,
  "code-cache-source-hash-sha256" => 819u32,
  "profile-deserialization" => 820u32,
  "trace-deserialization" => 821u32,
  "serialization-statistics" => 822u32,
  "regexp-optimization" => 823u32,
  "regexp-atomic-loop" => 824u32,
  "regexp-unroll" => 825u32,
  "regexp-masked-dispatch" => 826u32,
  "regexp-quick-check" => 827u32,
  "regexp-interpret-all" => 828u32,
  "regexp-tier-up" => 829u32,
  "regexp-tier-up-ticks" => 830u32,
  "regexp-jit-all" => 831u32,
  "regexp-peephole-optimization" => 832u32,
  "regexp-results-cache" => 833u32,
  "regexp-assemble-from-bytecode" => 834u32,
  "regexp-simd-in-rc" => 835u32,
  "trace-regexp-peephole-optimization" => 836u32,
  "trace-regexp-bytecodes" => 837u32,
  "trace-regexp-exec" => 838u32,
  "trace-regexp-exec-ool-subjects" => 839u32,
  "trace-regexp-assembler" => 840u32,
  "trace-regexp-compiler" => 841u32,
  "trace-regexp-graph-building" => 842u32,
  "trace-regexp-parser" => 843u32,
  "trace-regexp-graph" => 844u32,
  "print-regexp-graph" => 845u32,
  "trace-regexp-tier-up" => 846u32,
  "enable-experimental-regexp-engine" => 847u32,
  "default-to-experimental-regexp-engine" => 848u32,
  "experimental-regexp-engine-capture-group-opt" => 849u32,
  "experimental-regexp-engine-capture-group-opt-max-memory-usage" => 850u32,
  "trace-experimental-regexp-engine" => 851u32,
  "enable-experimental-regexp-engine-on-excessive-backtracks" => 852u32,
  "regexp-backtracks-before-fallback" => 853u32,
  "regexp-simd" => 854u32,
  "trace-read-only-promotion" => 855u32,
  "trace-read-only-promotion-verbose" => 856u32,
  "testing-bool-flag" => 857u32,
  "testing-maybe-bool-flag" => 858u32,
  "testing-int-flag" => 859u32,
  "testing-float-flag" => 860u32,
  "testing-string-flag" => 861u32,
  "testing-prng-seed" => 862u32,
  "testing-bool-flag-A" => 863u32,
  "testing-bool-flag-B" => 864u32,
  "testing-bool-flag-C" => 865u32,
  "strict-termination-checks" => 866u32,
  "fuzzing" => 867u32,
  "hole-fuzzing" => 868u32,
  "sandbox-testing" => 869u32,
  "validate-generated-code" => 870u32,
  "validate-generated-code-include-code" => 871u32,
  "validate-generated-code-non-fatal" => 872u32,
  "sandbox-fuzzing" => 873u32,
  "sandbox-trap-fuzzing" => 874u32,
  "verify-bytecode-light" => 875u32,
  "verify-bytecode-full" => 876u32,
  "expose-memory-corruption-api" => 877u32,
  "sandbox-prohibit-insecure-mode" => 878u32,
  "freeze-flags-after-init" => 879u32,
  "cet-compatible" => 880u32,
  "embedded-src" => 881u32,
  "embedded-variant" => 882u32,
  "startup-src" => 883u32,
  "startup-blob" => 884u32,
  "target-arch" => 885u32,
  "target-os" => 886u32,
  "target-is-simulator" => 887u32,
  "turbo-profiling-input" => 888u32,
  "turbo-log-builtins-count-input" => 889u32,
  "text-is-readable" => 890u32,
  "trace-minor-ms-parallel-marking" => 891u32,
  "minor-ms" => 892u32,
  "concurrent-minor-ms-marking" => 893u32,
  "black-allocated-pages" => 894u32,
  "sticky-mark-bits" => 895u32,
  "minor-ms-min-new-space-capacity-for-concurrent-marking-mb" => 896u32,
  "minor-ms-concurrent-marking-trigger" => 897u32,
  "minor-ms-min-lab-size-kb" => 898u32,
  "handle-weak-ref-weakly-in-minor-gc" => 899u32,
  "help" => 900u32,
  "print-flag-values" => 901u32,
  "print-feature-flags-json" => 902u32,
  "slow-histograms" => 903u32,
  "use-external-strings" => 904u32,
  "map-counters" => 905u32,
  "mock-arraybuffer-allocator" => 906u32,
  "mock-arraybuffer-allocator-limit" => 907u32,
  "gdbjit" => 908u32,
  "gdbjit-full" => 909u32,
  "gdbjit-dump" => 910u32,
  "torque-dwarf" => 911u32,
  "gdbjit-dump-filter" => 912u32,
  "enable-slow-asserts" => 913u32,
  "print-ast" => 914u32,
  "print-scopes" => 915u32,
  "trace-contexts" => 916u32,
  "code-stats" => 917u32,
  "print-handles" => 918u32,
  "check-handle-count" => 919u32,
  "print-global-handles" => 920u32,
  "trace-turbo-escape" => 921u32,
  "trace-module-status" => 922u32,
  "trace-normalization" => 923u32,
  "trace-lazy" => 924u32,
  "trace-isolates" => 925u32,
  "regexp-possessive-quantifier" => 926u32,
  "print-break-location" => 927u32,
  "logfile" => 928u32,
  "logfile-per-isolate" => 929u32,
  "log" => 930u32,
  "log-all" => 931u32,
  "log-internal-timer-events" => 932u32,
  "log-timer-events" => 933u32,
  "log-source-code" => 934u32,
  "log-source-position" => 935u32,
  "log-code" => 936u32,
  "log-feedback-vector" => 937u32,
  "log-code-disassemble" => 938u32,
  "log-function-events" => 939u32,
  "detailed-line-info" => 940u32,
  "prof-include-idle" => 941u32,
  "perfetto-code-logger" => 942u32,
  "prof-sampling-interval" => 943u32,
  "prof-cpp" => 944u32,
  "prof-browser-mode" => 945u32,
  "prof" => 946u32,
  "ll-prof" => 947u32,
  "perf-basic-prof" => 948u32,
  "perf-basic-prof-path" => 949u32,
  "perf-basic-prof-only-functions" => 950u32,
  "perf-prof" => 951u32,
  "perf-prof-path" => 952u32,
  "perf-prof-annotate-wasm" => 953u32,
  "perf-prof-delete-file" => 954u32,
  "perf-prof-unwinding-info" => 955u32,
  "gc-fake-mmap" => 956u32,
  "redirect-code-traces" => 957u32,
  "redirect-code-traces-to" => 958u32,
  "print-opt-source" => 959u32,
  "vtune-prof-annotate-wasm" => 960u32,
  "win64-unwinding-info" => 961u32,
  "interpreted-frames-native-stack" => 962u32,
  "enable-etw-stack-walking" => 963u32,
  "etw-trace-debug" => 964u32,
  "enable-etw-by-custom-filter-only" => 965u32,
  "print-builtin-size" => 966u32,
  "trace-elements-transitions" => 967u32,
  "trace-creation-allocation-sites" => 968u32,
  "print-code" => 969u32,
  "print-opt-code" => 970u32,
  "print-opt-code-filter" => 971u32,
  "print-code-verbose" => 972u32,
  "print-builtin-code" => 973u32,
  "print-builtin-code-filter" => 974u32,
  "print-regexp-code" => 975u32,
  "print-regexp-bytecode" => 976u32,
  "predictable" => 977u32,
  "predictable-gc-schedule" => 978u32,
  "single-threaded" => 979u32,
  "single-threaded-gc" => 980u32,
  "single-threaded-gc-in-background" => 981u32,
  "parallel-pause-for-gc-in-background" => 982u32,
  "incremental-marking-for-gc-in-background" => 983u32,
  "shared-heap" => 984u32,
  "empty-shared-heap" => 985u32,
  "proto-assign-seq-opt" => 986u32,
  "proto-assign-seq-opt-count" => 987u32,
  "proto-assign-seq-lazy-func-opt" => 988u32,
  "private-field-bytecodes" => 989u32,
  "track-array-buffer-views" => 990u32,
  "superspreading" => 991u32,
  "fast-proxy-ic" => 992u32,
  "is-standalone-d8-shell" => 993u32,
  "run-as-security-poc" => 994u32,
  "run-as-sandbox-security-poc" => 995u32,
  "verify-predictable" => 996u32,
  "dump-allocations-digest-at-alloc" => 997u32,
};

// 逐 flag 类型化存取器 (对应原版 v8_flags.xxx 字段读写)
impl Flags {
  /// Indicates that V8 is running with experimental features enabled. This flag is typically not set explicitly but instead enabled as an implication of other flags which enable experimental features.
  #[inline]
  pub fn experimental(&self) -> bool {
    self.get_bool(0)
  }
  #[inline]
  pub fn set_experimental(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(0, v)
  }

  /// enable experimental ArrayDestructure bytecode (experimental)
  #[inline]
  pub fn array_destructure_bytecode(&self) -> bool {
    self.get_bool(1)
  }
  #[inline]
  pub fn set_array_destructure_bytecode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(1, v)
  }

  /// Indicates that V8 is running in an unsupported and unsafe configuration, e.g. used for internal testing. This flag is typically not set explicitly but instead enabled as an implication of other flags.
  #[inline]
  pub fn test_only_unsafe(&self) -> bool {
    self.get_bool(2)
  }
  #[inline]
  pub fn set_test_only_unsafe(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(2, v)
  }

  /// Prevents the use of flags that are considered unsafe. Setting this flag will make V8 treat unsafe flags as flag contradictions and either crash, exit gracefully with a message, or reset them (depending on the current FlagProcessingMode).
  #[inline]
  pub fn disallow_unsafe_flags(&self) -> bool {
    self.get_bool(3)
  }
  #[inline]
  pub fn set_disallow_unsafe_flags(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(3, v)
  }

  /// Indicates that V8 is running with developer-only features enabled.
  #[inline]
  pub fn developer_only_features(&self) -> bool {
    self.get_bool(4)
  }
  #[inline]
  pub fn set_developer_only_features(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(4, v)
  }

  /// Disallow developer-only-features.
  #[inline]
  pub fn disallow_developer_only_features(&self) -> bool {
    self.get_bool(5)
  }
  #[inline]
  pub fn set_disallow_developer_only_features(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(5, v)
  }

  /// Control behavior for flag errors. Possible values: abort-on-error, exit-on-error, ignore-contradictions.
  #[inline]
  pub fn flag_processing_mode(&self) -> Option<&str> {
    self.get_str(6)
  }
  #[inline]
  pub fn set_flag_processing_mode(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(6, Some(Cow::Owned(v.to_string())))
  }

  /// enforce strict mode
  #[inline]
  pub fn use_strict(&self) -> bool {
    self.get_bool(7)
  }
  #[inline]
  pub fn set_use_strict(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(7, v)
  }

  /// trace temporal code (developer-only)
  #[inline]
  pub fn trace_temporal(&self) -> bool {
    self.get_bool(8)
  }
  #[inline]
  pub fn set_trace_temporal(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(8, v)
  }

  /// share immutable array buffers in postMessage instead of copying
  #[inline]
  pub fn js_postmessage_share_immutable_arraybuffer(&self) -> bool {
    self.get_bool(9)
  }
  #[inline]
  pub fn set_js_postmessage_share_immutable_arraybuffer(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(9, v)
  }

  /// subclassing support in built-in methods
  #[inline]
  pub fn builtin_subclassing(&self) -> bool {
    self.get_bool(10)
  }
  #[inline]
  pub fn set_builtin_subclassing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(10, v)
  }

  /// enable the SharedArrayBuffer constructor per context
  #[inline]
  pub fn enable_sharedarraybuffer_per_context(&self) -> bool {
    self.get_bool(11)
  }
  #[inline]
  pub fn set_enable_sharedarraybuffer_per_context(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(11, v)
  }

  /// non-extensibility includes private fields
  #[inline]
  pub fn js_nonextensible_applies_to_private(&self) -> bool {
    self.get_bool(12)
  }
  #[inline]
  pub fn set_js_nonextensible_applies_to_private(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(12, v)
  }

  /// Experimental flag for improving ForOf loops on ArrayIterator (experimental)
  #[inline]
  pub fn for_of_optimization(&self) -> bool {
    self.get_bool(13)
  }
  #[inline]
  pub fn set_for_of_optimization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(13, v)
  }

  /// ship a native queueMicrotask implementation on the global object (experimental)
  #[inline]
  pub fn enable_queue_microtask(&self) -> bool {
    self.get_bool(14)
  }
  #[inline]
  pub fn set_enable_queue_microtask(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(14, v)
  }

  /// get information about timezones from ICU
  #[inline]
  pub fn icu_timezone_data(&self) -> bool {
    self.get_bool(15)
  }
  #[inline]
  pub fn set_icu_timezone_data(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(15, v)
  }

  /// limits ICU date time compat changes to the given language
  #[inline]
  pub fn icu_datetime_compat_lang(&self) -> Option<&str> {
    self.get_str(16)
  }
  #[inline]
  pub fn set_icu_datetime_compat_lang(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(16, Some(Cow::Owned(v.to_string())))
  }

  /// enable 9-entry cache eviction in DateTimePatternGeneratorCache
  #[inline]
  pub fn intl_date_time_pattern_generator_cache_eviction(&self) -> bool {
    self.get_bool(17)
  }
  #[inline]
  pub fn set_intl_date_time_pattern_generator_cache_eviction(
    &mut self,
    v: bool,
  ) -> crate::Result<()> {
    self.assign_bool(17, v)
  }

  /// use ICU4X normalizer
  #[inline]
  pub fn use_icu4x_normalizer(&self) -> bool {
    self.get_bool(18)
  }
  #[inline]
  pub fn set_use_icu4x_normalizer(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(18, v)
  }

  /// disables sharing of the read-only heap for testing
  #[inline]
  pub fn stress_snapshot(&self) -> bool {
    self.get_bool(19)
  }
  #[inline]
  pub fn set_stress_snapshot(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(19, v)
  }

  /// enables trade-off of performance for memory savings
  #[inline]
  pub fn lite_mode(&self) -> bool {
    self.get_bool(20)
  }
  #[inline]
  pub fn set_lite_mode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(20, v)
  }

  /// Use allocation folding globally
  #[inline]
  pub fn enable_allocation_folding(&self) -> bool {
    self.get_bool(21)
  }
  #[inline]
  pub fn set_enable_allocation_folding(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(21, v)
  }

  /// disable write barriers when GC is non-incremental and heap contains single generation.
  #[inline]
  pub fn disable_write_barriers(&self) -> bool {
    self.get_bool(22)
  }
  #[inline]
  pub fn set_disable_write_barriers(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(22, v)
  }

  /// always use full write barriers
  #[inline]
  pub fn enable_unconditional_write_barriers(&self) -> bool {
    self.get_bool(23)
  }
  #[inline]
  pub fn set_enable_unconditional_write_barriers(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(23, v)
  }

  /// allocate all objects from young generation to old generation
  #[inline]
  pub fn single_generation(&self) -> bool {
    self.get_bool(24)
  }
  #[inline]
  pub fn set_single_generation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(24, v)
  }

  /// use conservative stack scanning (experimental)
  #[inline]
  pub fn conservative_stack_scanning(&self) -> bool {
    self.get_bool(25)
  }
  #[inline]
  pub fn set_conservative_stack_scanning(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(25, v)
  }

  /// use direct handles with conservative stack scanning
  #[inline]
  pub fn direct_handle(&self) -> bool {
    self.get_bool(26)
  }
  #[inline]
  pub fn set_direct_handle(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(26, v)
  }

  /// Objects reachable from the native stack during scavenge will be pinned and won't move.
  #[inline]
  pub fn scavenger_conservative_object_pinning(&self) -> bool {
    self.get_bool(27)
  }
  #[inline]
  pub fn set_scavenger_conservative_object_pinning(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(27, v)
  }

  /// Treat some precise references as conservative references to stress test object pinning in Scavenger
  #[inline]
  pub fn stress_scavenger_conservative_object_pinning(&self) -> bool {
    self.get_bool(28)
  }
  #[inline]
  pub fn set_stress_scavenger_conservative_object_pinning(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(28, v)
  }

  /// Enables random stressing of object pinning in Scavenger, such that each GC would randomly pick a subset of the precise references to treat conservatively
  #[inline]
  pub fn stress_scavenger_conservative_object_pinning_random(&self) -> bool {
    self.get_bool(29)
  }
  #[inline]
  pub fn set_stress_scavenger_conservative_object_pinning_random(
    &mut self,
    v: bool,
  ) -> crate::Result<()> {
    self.assign_bool(29, v)
  }

  /// Objects reachable from handles during scavenge will be pinned and won't move.
  #[inline]
  pub fn scavenger_precise_object_pinning(&self) -> bool {
    self.get_bool(30)
  }
  #[inline]
  pub fn set_scavenger_precise_object_pinning(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(30, v)
  }

  /// Scavenger updates allocation limits if allocation rate is low
  #[inline]
  pub fn scavenger_updates_allocation_limit(&self) -> bool {
    self.get_bool(31)
  }
  #[inline]
  pub fn set_scavenger_updates_allocation_limit(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(31, v)
  }

  /// Objects reachable from handles during GC will be pinned and won't move.
  #[inline]
  pub fn precise_object_pinning(&self) -> bool {
    self.get_bool(32)
  }
  #[inline]
  pub fn set_precise_object_pinning(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(32, v)
  }

  /// Quarantined pages in the intermediate generation will be promoted to old space
  #[inline]
  pub fn scavenger_promote_quarantined_pages(&self) -> bool {
    self.get_bool(33)
  }
  #[inline]
  pub fn set_scavenger_promote_quarantined_pages(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(33, v)
  }

  /// Scavenger will ignore age when making promotion decisions and instead choose objects to be promoted at random. This only applies to non-large objects.
  #[inline]
  pub fn scavenger_chaos_mode(&self) -> bool {
    self.get_bool(34)
  }
  #[inline]
  pub fn set_scavenger_chaos_mode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(34, v)
  }

  /// Percentage of non-large young objects that will be promoted during Scavenger in chaos mode
  #[inline]
  pub fn scavenger_chaos_mode_threshold(&self) -> u32 {
    self.get_u32(35)
  }
  #[inline]
  pub fn set_scavenger_chaos_mode_threshold(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(35, v)
  }

  /// check for off-stack allocation of v8::Local
  #[inline]
  pub fn local_off_stack_check(&self) -> bool {
    self.get_bool(36)
  }
  #[inline]
  pub fn set_local_off_stack_check(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(36, v)
  }

  /// force emit tier-up logic from all non-turbofan code, even if it is the top enabled tier
  #[inline]
  pub fn force_emit_interrupt_budget_checks(&self) -> bool {
    self.get_bool(37)
  }
  #[inline]
  pub fn set_force_emit_interrupt_budget_checks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(37, v)
  }

  /// enable the maglev optimizing compiler
  #[inline]
  pub fn maglev(&self) -> bool {
    self.get_bool(38)
  }
  #[inline]
  pub fn set_maglev(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(38, v)
  }

  /// enable maglev features that we want to ship in the not-too-far future (experimental)
  #[inline]
  pub fn maglev_future(&self) -> bool {
    self.get_bool(39)
  }
  #[inline]
  pub fn set_maglev_future(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(39, v)
  }

  /// make OptimizeFunctionOnNextCall optimize to maglev instead of turbofan
  #[inline]
  pub fn optimize_on_next_call_optimizes_to_maglev(&self) -> bool {
    self.get_bool(40)
  }
  #[inline]
  pub fn set_optimize_on_next_call_optimizes_to_maglev(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(40, v)
  }

  /// trigger maglev compilation earlier
  #[inline]
  pub fn stress_maglev(&self) -> bool {
    self.get_bool(41)
  }
  #[inline]
  pub fn set_stress_maglev(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(41, v)
  }

  /// max inlined maglev graph size that counts as a small function
  #[inline]
  pub fn maglev_max_small_graph_size(&self) -> i32 {
    self.get_i32(42)
  }
  #[inline]
  pub fn set_maglev_max_small_graph_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(42, v)
  }

  /// disable Turbofan/Turbolev, make Maglev optimize more aggressively
  #[inline]
  pub fn maglev_as_top_tier(&self) -> bool {
    self.get_bool(43)
  }
  #[inline]
  pub fn set_maglev_as_top_tier(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(43, v)
  }

  /// enable inlining in the maglev optimizing compiler
  #[inline]
  pub fn maglev_inlining(&self) -> bool {
    self.get_bool(44)
  }
  #[inline]
  pub fn set_maglev_inlining(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(44, v)
  }

  /// disable eager builtin reducers in the maglev graph builder to flush out production bugs in post-inlining builtin reduction
  #[inline]
  pub fn maglev_disable_builtin_reducers(&self) -> bool {
    self.get_bool(45)
  }
  #[inline]
  pub fn set_maglev_disable_builtin_reducers(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(45, v)
  }

  /// enable loop peeling in the maglev optimizing compiler
  #[inline]
  pub fn maglev_loop_peeling(&self) -> bool {
    self.get_bool(46)
  }
  #[inline]
  pub fn set_maglev_loop_peeling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(46, v)
  }

  /// enable aggressive optimizations for loops (loop SPeeling) in the maglev optimizing compiler
  #[inline]
  pub fn maglev_optimistic_peeled_loops(&self) -> bool {
    self.get_bool(47)
  }
  #[inline]
  pub fn set_maglev_optimistic_peeled_loops(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(47, v)
  }

  /// max loop size for loop peeling in the maglev optimizing compiler
  #[inline]
  pub fn maglev_loop_peeling_max_size(&self) -> i32 {
    self.get_i32(48)
  }
  #[inline]
  pub fn set_maglev_loop_peeling_max_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(48, v)
  }

  /// max cumulative size for loop peeling in the maglev optimizing compiler
  #[inline]
  pub fn maglev_loop_peeling_max_size_cumulative(&self) -> i32 {
    self.get_i32(49)
  }
  #[inline]
  pub fn set_maglev_loop_peeling_max_size_cumulative(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(49, v)
  }

  /// Generate deopt data on background thread
  #[inline]
  pub fn maglev_deopt_data_on_background(&self) -> bool {
    self.get_bool(50)
  }
  #[inline]
  pub fn set_maglev_deopt_data_on_background(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(50, v)
  }

  /// Generate code on background thread
  #[inline]
  pub fn maglev_build_code_on_background(&self) -> bool {
    self.get_bool(51)
  }
  #[inline]
  pub fn set_maglev_build_code_on_background(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(51, v)
  }

  /// Destroy compilation jobs on background thread
  #[inline]
  pub fn maglev_destroy_on_background(&self) -> bool {
    self.get_bool(52)
  }
  #[inline]
  pub fn set_maglev_destroy_on_background(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(52, v)
  }

  /// Inline CallApiCallback builtin into generated code
  #[inline]
  pub fn maglev_inline_api_calls(&self) -> bool {
    self.get_bool(53)
  }
  #[inline]
  pub fn set_maglev_inline_api_calls(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(53, v)
  }

  /// Native support for cons strings and their elision in maglev.
  #[inline]
  pub fn maglev_cons_string_elision(&self) -> bool {
    self.get_bool(54)
  }
  #[inline]
  pub fn set_maglev_cons_string_elision(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(54, v)
  }

  /// Recursively pretenure values which are stored into pretenured allocation sites.
  #[inline]
  pub fn maglev_pretenure_store_values(&self) -> bool {
    self.get_bool(55)
  }
  #[inline]
  pub fn set_maglev_pretenure_store_values(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(55, v)
  }

  /// Support (inlining) polymorphic calls
  #[inline]
  pub fn maglev_poly_calls(&self) -> bool {
    self.get_bool(56)
  }
  #[inline]
  pub fn set_maglev_poly_calls(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(56, v)
  }

  /// Enable Maglev truncation pass
  #[inline]
  pub fn maglev_truncation(&self) -> bool {
    self.get_bool(57)
  }
  #[inline]
  pub fn set_maglev_truncation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(57, v)
  }

  /// trace maglev truncation pass (developer-only)
  #[inline]
  pub fn trace_maglev_truncation(&self) -> bool {
    self.get_bool(58)
  }
  #[inline]
  pub fn set_trace_maglev_truncation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(58, v)
  }

  /// loop invariant code motion (experimental)
  #[inline]
  pub fn maglev_licm(&self) -> bool {
    self.get_bool(59)
  }
  #[inline]
  pub fn set_maglev_licm(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(59, v)
  }

  /// Enable truncated to int32 representation in the phi selector
  #[inline]
  pub fn turbolev_truncated_int32_phis(&self) -> bool {
    self.get_bool(60)
  }
  #[inline]
  pub fn set_turbolev_truncated_int32_phis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(60, v)
  }

  /// Enable Maglev range value analysis pass
  #[inline]
  pub fn maglev_range_analysis(&self) -> bool {
    self.get_bool(61)
  }
  #[inline]
  pub fn set_maglev_range_analysis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(61, v)
  }

  /// Trace Maglev range value analysis pass (developer-only)
  #[inline]
  pub fn trace_maglev_range_analysis(&self) -> bool {
    self.get_bool(62)
  }
  #[inline]
  pub fn set_trace_maglev_range_analysis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(62, v)
  }

  /// Trace Maglev known node aspects (developer-only)
  #[inline]
  pub fn trace_maglev_kna(&self) -> bool {
    self.get_bool(63)
  }
  #[inline]
  pub fn set_trace_maglev_kna(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(63, v)
  }

  /// Use unreliable unstable maps in Maglev
  #[inline]
  pub fn maglev_use_unreliable_maps(&self) -> bool {
    self.get_bool(64)
  }
  #[inline]
  pub fn set_maglev_use_unreliable_maps(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(64, v)
  }

  /// Run integer range verifiction pass in Turbolev frontend pipeline
  #[inline]
  pub fn maglev_range_verification(&self) -> bool {
    self.get_bool(65)
  }
  #[inline]
  pub fn set_maglev_range_verification(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(65, v)
  }

  /// verify dominance of eager deopt frame inputs
  #[inline]
  pub fn maglev_verify_dominance(&self) -> bool {
    self.get_bool(66)
  }
  #[inline]
  pub fn set_maglev_verify_dominance(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(66, v)
  }

  /// max number of threads that concurrent Maglev can use (0 for unbounded)
  #[inline]
  pub fn concurrent_maglev_max_threads(&self) -> u32 {
    self.get_u32(67)
  }
  #[inline]
  pub fn set_concurrent_maglev_max_threads(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(67, v)
  }

  /// use high priority compiler threads for concurrent Maglev
  #[inline]
  pub fn concurrent_maglev_high_priority_threads(&self) -> bool {
    self.get_bool(68)
  }
  #[inline]
  pub fn set_concurrent_maglev_high_priority_threads(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(68, v)
  }

  /// max depth of functions that Maglev will inline excl. small functions
  #[inline]
  pub fn max_maglev_inline_depth(&self) -> i32 {
    self.get_i32(69)
  }
  #[inline]
  pub fn set_max_maglev_inline_depth(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(69, v)
  }

  /// max depth of functions that Maglev will inline incl. small functions
  #[inline]
  pub fn max_maglev_hard_inline_depth(&self) -> i32 {
    self.get_i32(70)
  }
  #[inline]
  pub fn set_max_maglev_hard_inline_depth(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(70, v)
  }

  /// maximum size of bytecode for a single inlining
  #[inline]
  pub fn max_maglev_inlined_bytecode_size(&self) -> i32 {
    self.get_i32(71)
  }
  #[inline]
  pub fn set_max_maglev_inlined_bytecode_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(71, v)
  }

  /// maximum cumulative size of bytecode considered for inlining excl. small functions
  #[inline]
  pub fn max_maglev_inlined_bytecode_size_cumulative(&self) -> i32 {
    self.get_i32(72)
  }
  #[inline]
  pub fn set_max_maglev_inlined_bytecode_size_cumulative(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(72, v)
  }

  /// maximum size of small functions' bytecode that can be inlining
  #[inline]
  pub fn max_maglev_inlined_bytecode_size_small_total(&self) -> i32 {
    self.get_i32(73)
  }
  #[inline]
  pub fn set_max_maglev_inlined_bytecode_size_small_total(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(73, v)
  }

  /// maximum size of bytecode considered for small function inlining
  #[inline]
  pub fn max_maglev_inlined_bytecode_size_small(&self) -> i32 {
    self.get_i32(74)
  }
  #[inline]
  pub fn set_max_maglev_inlined_bytecode_size_small(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(74, v)
  }

  /// maximum size of bytecode considered for small function inlining, when the function call has a HeapNumber as input or output
  #[inline]
  pub fn max_maglev_inlined_bytecode_size_small_with_heapnum_in_out(&self) -> i32 {
    self.get_i32(75)
  }
  #[inline]
  pub fn set_max_maglev_inlined_bytecode_size_small_with_heapnum_in_out(
    &mut self,
    v: i32,
  ) -> crate::Result<()> {
    self.assign_i32(75, v)
  }

  /// minimum frequency for inlining
  #[inline]
  pub fn min_maglev_inlining_frequency(&self) -> f64 {
    self.get_f64(76)
  }
  #[inline]
  pub fn set_min_maglev_inlining_frequency(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(76, v)
  }

  /// max depth of functions that Maglev will inline excl. small functions
  #[inline]
  pub fn max_turbolev_inline_depth(&self) -> i32 {
    self.get_i32(77)
  }
  #[inline]
  pub fn set_max_turbolev_inline_depth(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(77, v)
  }

  /// reuse stack slots in the maglev optimizing compiler
  #[inline]
  pub fn maglev_reuse_stack_slots(&self) -> bool {
    self.get_bool(78)
  }
  #[inline]
  pub fn set_maglev_reuse_stack_slots(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(78, v)
  }

  /// enable phi untagging in the maglev optimizing compiler
  #[inline]
  pub fn maglev_untagged_phis(&self) -> bool {
    self.get_bool(79)
  }
  #[inline]
  pub fn set_maglev_untagged_phis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(79, v)
  }

  /// limit the number of Phis we untag in Maglev phi untagging (-1 for unbounded)
  #[inline]
  pub fn maglev_untagged_phis_bisect_limit(&self) -> i32 {
    self.get_i32(80)
  }
  #[inline]
  pub fn set_maglev_untagged_phis_bisect_limit(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(80, v)
  }

  /// enable phi untagging in the Turbolev optimizing compiler
  #[inline]
  pub fn turbolev_untagged_phis(&self) -> bool {
    self.get_bool(81)
  }
  #[inline]
  pub fn set_turbolev_untagged_phis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(81, v)
  }

  /// enable phi untagging to hoist untagging of osr values
  #[inline]
  pub fn maglev_hoist_osr_value_phi_untagging(&self) -> bool {
    self.get_bool(82)
  }
  #[inline]
  pub fn set_maglev_hoist_osr_value_phi_untagging(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(82, v)
  }

  /// enable phi untagging to hoist untagging of loop phi inputs (could still cause deopt loops) (experimental)
  #[inline]
  pub fn maglev_speculative_hoist_phi_untagging(&self) -> bool {
    self.get_bool(83)
  }
  #[inline]
  pub fn set_maglev_speculative_hoist_phi_untagging(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(83, v)
  }

  /// common subexpression elimination
  #[inline]
  pub fn maglev_cse(&self) -> bool {
    self.get_bool(84)
  }
  #[inline]
  pub fn set_maglev_cse(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(84, v)
  }

  /// enable Maglev non-eager inlining
  #[inline]
  pub fn maglev_non_eager_inlining(&self) -> bool {
    self.get_bool(85)
  }
  #[inline]
  pub fn set_maglev_non_eager_inlining(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(85, v)
  }

  /// enable Turbolev non-eager inlining
  #[inline]
  pub fn turbolev_non_eager_inlining(&self) -> bool {
    self.get_bool(86)
  }
  #[inline]
  pub fn set_turbolev_non_eager_inlining(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(86, v)
  }

  /// enable Turbolev non-eager loop peeling
  #[inline]
  pub fn turbolev_non_eager_loop_peeling(&self) -> bool {
    self.get_bool(87)
  }
  #[inline]
  pub fn set_turbolev_non_eager_loop_peeling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(87, v)
  }

  /// use eager loop peeling in Turbolev OSR compiles
  #[inline]
  pub fn turbolev_eager_loop_peeling_osr(&self) -> bool {
    self.get_bool(88)
  }
  #[inline]
  pub fn set_turbolev_eager_loop_peeling_osr(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(88, v)
  }

  /// trace turbolev non-eager loop peeling decisions (developer-only)
  #[inline]
  pub fn turbolev_trace_loop_peeling(&self) -> bool {
    self.get_bool(89)
  }
  #[inline]
  pub fn set_turbolev_trace_loop_peeling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(89, v)
  }

  /// optimization filter for the maglev compiler
  #[inline]
  pub fn maglev_filter(&self) -> Option<&str> {
    self.get_str(90)
  }
  #[inline]
  pub fn set_maglev_filter(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(90, Some(Cow::Owned(v.to_string())))
  }

  /// filter for maglev's tracing/printing options
  #[inline]
  pub fn maglev_print_filter(&self) -> Option<&str> {
    self.get_str(91)
  }
  #[inline]
  pub fn set_maglev_print_filter(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(91, Some(Cow::Owned(v.to_string())))
  }

  /// insert extra assertion in maglev code
  #[inline]
  pub fn maglev_assert(&self) -> bool {
    self.get_bool(92)
  }
  #[inline]
  pub fn set_maglev_assert(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(92, v)
  }

  /// insert stack size checks before every IR node
  #[inline]
  pub fn maglev_assert_stack_size(&self) -> bool {
    self.get_bool(93)
  }
  #[inline]
  pub fn set_maglev_assert_stack_size(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(93, v)
  }

  /// Enable additional type assertions in generated code (experimental)
  #[inline]
  pub fn maglev_assert_types(&self) -> bool {
    self.get_bool(94)
  }
  #[inline]
  pub fn set_maglev_assert_types(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(94, v)
  }

  /// insert an int3 on maglev entries
  #[inline]
  pub fn maglev_break_on_entry(&self) -> bool {
    self.get_bool(95)
  }
  #[inline]
  pub fn set_maglev_break_on_entry(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(95, v)
  }

  /// print bytecode for maglev compiled code
  #[inline]
  pub fn maglev_print_bytecode(&self) -> bool {
    self.get_bool(96)
  }
  #[inline]
  pub fn set_maglev_print_bytecode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(96, v)
  }

  /// print feedback vector for maglev compiled code
  #[inline]
  pub fn maglev_print_feedback(&self) -> bool {
    self.get_bool(97)
  }
  #[inline]
  pub fn set_maglev_print_feedback(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(97, v)
  }

  /// print bytecode / feedback vectors also for inlined code
  #[inline]
  pub fn maglev_print_inlined(&self) -> bool {
    self.get_bool(98)
  }
  #[inline]
  pub fn set_maglev_print_inlined(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(98, v)
  }

  /// print Maglev node provenance, ie, file name, bytecode and offset
  #[inline]
  pub fn maglev_print_provenance(&self) -> bool {
    self.get_bool(99)
  }
  #[inline]
  pub fn set_maglev_print_provenance(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(99, v)
  }

  /// print maglev code (developer-only)
  #[inline]
  pub fn print_maglev_code(&self) -> bool {
    self.get_bool(100)
  }
  #[inline]
  pub fn set_print_maglev_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(100, v)
  }

  /// prefix maglev/turbolev trace line with a compilation id
  #[inline]
  pub fn trace_with_compilation_id(&self) -> bool {
    self.get_bool(101)
  }
  #[inline]
  pub fn set_trace_with_compilation_id(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(101, v)
  }

  /// trace maglev graph building (developer-only)
  #[inline]
  pub fn trace_maglev_graph_building(&self) -> bool {
    self.get_bool(102)
  }
  #[inline]
  pub fn set_trace_maglev_graph_building(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(102, v)
  }

  /// trace maglev graph optimizer (developer-only)
  #[inline]
  pub fn trace_maglev_graph_optimizer(&self) -> bool {
    self.get_bool(103)
  }
  #[inline]
  pub fn set_trace_maglev_graph_optimizer(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(103, v)
  }

  /// trace maglev loop SPeeling (developer-only)
  #[inline]
  pub fn trace_maglev_loop_speeling(&self) -> bool {
    self.get_bool(104)
  }
  #[inline]
  pub fn set_trace_maglev_loop_speeling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(104, v)
  }

  /// trace maglev inlining (developer-only)
  #[inline]
  pub fn trace_maglev_inlining(&self) -> bool {
    self.get_bool(105)
  }
  #[inline]
  pub fn set_trace_maglev_inlining(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(105, v)
  }

  /// trace maglev known node aspects processor (developer-only)
  #[inline]
  pub fn trace_maglev_kna_processor(&self) -> bool {
    self.get_bool(106)
  }
  #[inline]
  pub fn set_trace_maglev_kna_processor(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(106, v)
  }

  /// trace maglev scope info (developer-only)
  #[inline]
  pub fn trace_maglev_scope_info(&self) -> bool {
    self.get_bool(107)
  }
  #[inline]
  pub fn set_trace_maglev_scope_info(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(107, v)
  }

  /// trace Maglev IR to JSON (developer-only)
  #[inline]
  pub fn trace_maglev(&self) -> bool {
    self.get_bool(108)
  }
  #[inline]
  pub fn set_trace_maglev(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(108, v)
  }

  /// write maglev trace files with given prefix
  #[inline]
  pub fn trace_maglev_file_prefix(&self) -> Option<&str> {
    self.get_str(109)
  }
  #[inline]
  pub fn set_trace_maglev_file_prefix(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(109, Some(Cow::Owned(v.to_string())))
  }

  /// print verbose deopt info
  #[inline]
  pub fn print_maglev_deopt_verbose(&self) -> bool {
    self.get_bool(110)
  }
  #[inline]
  pub fn set_print_maglev_deopt_verbose(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(110, v)
  }

  /// print the final maglev graph
  #[inline]
  pub fn print_maglev_graph(&self) -> bool {
    self.get_bool(111)
  }
  #[inline]
  pub fn set_print_maglev_graph(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(111, v)
  }

  /// print maglev graph across all phases
  #[inline]
  pub fn print_maglev_graphs(&self) -> bool {
    self.get_bool(112)
  }
  #[inline]
  pub fn set_print_maglev_graphs(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(112, v)
  }

  /// trace maglev phi untagging
  #[inline]
  pub fn trace_maglev_phi_untagging(&self) -> bool {
    self.get_bool(113)
  }
  #[inline]
  pub fn set_trace_maglev_phi_untagging(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(113, v)
  }

  /// trace maglev register allocation
  #[inline]
  pub fn trace_maglev_regalloc(&self) -> bool {
    self.get_bool(114)
  }
  #[inline]
  pub fn set_trace_maglev_regalloc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(114, v)
  }

  /// print Maglev statistics (developer-only)
  #[inline]
  pub fn maglev_stats(&self) -> bool {
    self.get_bool(115)
  }
  #[inline]
  pub fn set_maglev_stats(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(115, v)
  }

  /// print Maglev statistics in machine-readable format (developer-only)
  #[inline]
  pub fn maglev_stats_nvp(&self) -> bool {
    self.get_bool(116)
  }
  #[inline]
  pub fn set_maglev_stats_nvp(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(116, v)
  }

  /// enable function context specialization in maglev
  #[inline]
  pub fn maglev_function_context_specialization(&self) -> bool {
    self.get_bool(117)
  }
  #[inline]
  pub fn set_maglev_function_context_specialization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(117, v)
  }

  /// Record AdditiveSafeInteger feedback
  #[inline]
  pub fn additive_safe_int_feedback(&self) -> bool {
    self.get_bool(118)
  }
  #[inline]
  pub fn set_additive_safe_int_feedback(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(118, v)
  }

  /// Enable the use of AdditiveSafeInteger feedback for Turbolev
  #[inline]
  pub fn turbolev_additive_safe_int_feedback(&self) -> bool {
    self.get_bool(119)
  }
  #[inline]
  pub fn set_turbolev_additive_safe_int_feedback(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(119, v)
  }

  /// Enable the use of AdditiveSafeInteger feedback for TurboFan
  #[inline]
  pub fn turbofan_additive_safe_int_feedback(&self) -> bool {
    self.get_bool(120)
  }
  #[inline]
  pub fn set_turbofan_additive_safe_int_feedback(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(120, v)
  }

  /// enable generating GetEnumeratedKeyedProperty bytecode for keyed access
  #[inline]
  pub fn enable_enumerated_keyed_access_bytecode(&self) -> bool {
    self.get_bool(121)
  }
  #[inline]
  pub fn set_enable_enumerated_keyed_access_bytecode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(121, v)
  }

  /// Use const tracking on dictionary properties
  #[inline]
  pub fn dict_property_const_tracking(&self) -> bool {
    self.get_bool(122)
  }
  #[inline]
  pub fn set_dict_property_const_tracking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(122, v)
  }

  /// Use context cells in script contexts, ie, const tracking let and mutable numbers
  #[inline]
  pub fn script_context_cells(&self) -> bool {
    self.get_bool(123)
  }
  #[inline]
  pub fn set_script_context_cells(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(123, v)
  }

  /// Use context cells in function contexts
  #[inline]
  pub fn function_context_cells(&self) -> bool {
    self.get_bool(124)
  }
  #[inline]
  pub fn set_function_context_cells(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(124, v)
  }

  /// Max number of context slots allowed in a function context with context cells
  #[inline]
  pub fn function_context_cells_max_size(&self) -> i32 {
    self.get_i32(125)
  }
  #[inline]
  pub fn set_function_context_cells_max_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(125, v)
  }

  /// Use compilation dependency to avoid dynamic checks for non-empty context extensions
  #[inline]
  pub fn empty_context_extension_dep(&self) -> bool {
    self.get_bool(126)
  }
  #[inline]
  pub fn set_empty_context_extension_dep(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(126, v)
  }

  /// Enable JSON.stringify fast-path
  #[inline]
  pub fn json_stringify_fast_path(&self) -> bool {
    self.get_bool(127)
  }
  #[inline]
  pub fn set_json_stringify_fast_path(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(127, v)
  }

  /// Maximum number of strings that are eagerly internalized during JSON.parse() based on heuristics. 0 means only strings that must be internalized e.g. property keys will be internalized.
  #[inline]
  pub fn json_parse_max_heuristically_internalized_strings(&self) -> u32 {
    self.get_u32(128)
  }
  #[inline]
  pub fn set_json_parse_max_heuristically_internalized_strings(
    &mut self,
    v: u32,
  ) -> crate::Result<()> {
    self.assign_u32(128, v)
  }

  /// Print heuristic internalization stats after each JSON.parse() (developer-only)
  #[inline]
  pub fn trace_json_parse_internalization(&self) -> bool {
    self.get_bool(129)
  }
  #[inline]
  pub fn set_trace_json_parse_internalization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(129, v)
  }

  /// Enable caching property keys created by concatenating strings
  #[inline]
  pub fn cache_property_key_string_adds(&self) -> bool {
    self.get_bool(130)
  }
  #[inline]
  pub fn set_cache_property_key_string_adds(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(130, v)
  }

  /// Full size of SmiStringCache
  #[inline]
  pub fn smi_string_cache_size(&self) -> u32 {
    self.get_u32(131)
  }
  #[inline]
  pub fn set_smi_string_cache_size(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(131, v)
  }

  /// Full size of DoubleStringCache
  #[inline]
  pub fn double_string_cache_size(&self) -> u32 {
    self.get_u32(132)
  }
  #[inline]
  pub fn set_double_string_cache_size(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(132, v)
  }

  /// Print NumberString caches stats on GC and at the end (developer-only)
  #[inline]
  pub fn trace_number_string_cache(&self) -> bool {
    self.get_bool(133)
  }
  #[inline]
  pub fn set_trace_number_string_cache(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(133, v)
  }

  /// Inline Date accessors by Maglev
  #[inline]
  pub fn maglev_inline_date_accessors(&self) -> bool {
    self.get_bool(134)
  }
  #[inline]
  pub fn set_maglev_inline_date_accessors(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(134, v)
  }

  /// Inline Date accessors by TurboFan
  #[inline]
  pub fn turbofan_inline_date_accessors(&self) -> bool {
    self.get_bool(135)
  }
  #[inline]
  pub fn set_turbofan_inline_date_accessors(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(135, v)
  }

  /// Inline Date accessors
  #[inline]
  pub fn inline_date_accessors(&self) -> bool {
    self.get_bool(136)
  }
  #[inline]
  pub fn set_inline_date_accessors(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(136, v)
  }

  /// Whether custom embedder snapshots may extend ReadOnlySpace
  #[inline]
  pub fn extensible_ro_snapshot(&self) -> bool {
    self.get_bool(137)
  }
  #[inline]
  pub fn set_extensible_ro_snapshot(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(137, v)
  }

  /// Set the maximal optimisation tier: > 3 == any, 0 == ignition/interpreter, 1 == sparkplug/baseline, 2 == maglev, 3 == turbofan
  #[inline]
  pub fn max_opt(&self) -> u32 {
    self.get_u32(138)
  }
  #[inline]
  pub fn set_max_opt(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(138, v)
  }

  /// Forces efficiency mode on or off, disregarding any dynamic signals. Efficiency mode is optimized for situations with no latency requirements and uses fewer threads.
  #[inline]
  pub fn efficiency_mode(&self) -> Option<bool> {
    self.get_mb(139)
  }
  #[inline]
  pub fn set_efficiency_mode(&mut self, v: Option<bool>) -> crate::Result<()> {
    self.assign_mb(139, v)
  }

  /// Forces battery saver mode on or off, disregarding any dynamic signals. Battery saver tries to conserve overall cpu cycles spent.
  #[inline]
  pub fn battery_saver_mode(&self) -> Option<bool> {
    self.get_mb(140)
  }
  #[inline]
  pub fn set_battery_saver_mode(&mut self, v: Option<bool>) -> crate::Result<()> {
    self.assign_mb(140, v)
  }

  /// Forces memory saver mode on or off, disregarding any dynamic signals. Memory saver tries to keep memory footprint low at the expense of extra cpu cycles.
  #[inline]
  pub fn memory_saver_mode(&self) -> Option<bool> {
    self.get_mb(141)
  }
  #[inline]
  pub fn set_memory_saver_mode(&mut self, v: Option<bool>) -> crate::Result<()> {
    self.assign_mb(141, v)
  }

  /// Defer tier-up to turbofan while in efficiency mode.
  #[inline]
  pub fn efficiency_mode_disable_turbofan(&self) -> bool {
    self.get_bool(142)
  }
  #[inline]
  pub fn set_efficiency_mode_disable_turbofan(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(142, v)
  }

  /// Delay tier-up to turbofan to a certain invocation count multipier while in efficiency mode.
  #[inline]
  pub fn efficiency_mode_delay_turbofan_multiply(&self) -> i32 {
    self.get_i32(143)
  }
  #[inline]
  pub fn set_efficiency_mode_delay_turbofan_multiply(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(143, v)
  }

  /// Select which native code sequence to use for wasm trace instruction: default or cpuid
  #[inline]
  pub fn wasm_trace_native(&self) -> Option<&str> {
    self.get_str(144)
  }
  #[inline]
  pub fn set_wasm_trace_native(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(144, Some(Cow::Owned(v.to_string())))
  }

  /// Disable runtime allocation of executable memory.
  #[inline]
  pub fn jitless(&self) -> bool {
    self.get_bool(145)
  }
  #[inline]
  pub fn set_jitless(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(145, v)
  }

  /// Disable all optimizing compilers while leaving baseline compilers enabled
  #[inline]
  pub fn disable_optimizing_compilers(&self) -> bool {
    self.get_bool(146)
  }
  #[inline]
  pub fn set_disable_optimizing_compilers(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(146, v)
  }

  /// protect code memory with PKU if available
  #[inline]
  pub fn memory_protection_keys(&self) -> bool {
    self.get_bool(147)
  }
  #[inline]
  pub fn set_memory_protection_keys(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(147, v)
  }

  /// fail if memory protection key support is not available
  #[inline]
  pub fn force_memory_protection_keys(&self) -> bool {
    self.get_bool(148)
  }
  #[inline]
  pub fn set_force_memory_protection_keys(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(148, v)
  }

  /// generate runtime type assertions to test the typer
  #[inline]
  pub fn assert_types(&self) -> bool {
    self.get_bool(149)
  }
  #[inline]
  pub fn set_assert_types(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(149, v)
  }

  /// generate runtime type assertions to test the turboshaft type system (experimental)
  #[inline]
  pub fn turboshaft_assert_types(&self) -> bool {
    self.get_bool(150)
  }
  #[inline]
  pub fn set_turboshaft_assert_types(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(150, v)
  }

  /// verify graph generated by simplified lowering
  #[inline]
  pub fn verify_simplified_lowering(&self) -> bool {
    self.get_bool(151)
  }
  #[inline]
  pub fn set_verify_simplified_lowering(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(151, v)
  }

  /// trace code dependencies (developer-only)
  #[inline]
  pub fn trace_compilation_dependencies(&self) -> bool {
    self.get_bool(152)
  }
  #[inline]
  pub fn set_trace_compilation_dependencies(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(152, v)
  }

  /// unmap the page containing the holes.
  #[inline]
  pub fn unmap_holes(&self) -> bool {
    self.get_bool(153)
  }
  #[inline]
  pub fn set_unmap_holes(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(153, v)
  }

  /// Enable allocation site tracking
  #[inline]
  pub fn allocation_site_tracking(&self) -> bool {
    self.get_bool(154)
  }
  #[inline]
  pub fn set_allocation_site_tracking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(154, v)
  }

  /// pretenure with allocation sites
  #[inline]
  pub fn allocation_site_pretenuring(&self) -> bool {
    self.get_bool(155)
  }
  #[inline]
  pub fn set_allocation_site_pretenuring(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(155, v)
  }

  /// promote pages based on utilization
  #[inline]
  pub fn page_promotion(&self) -> bool {
    self.get_bool(156)
  }
  #[inline]
  pub fn set_page_promotion(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(156, v)
  }

  /// min percentage of live bytes on a page to enable fast evacuation in full GCs
  #[inline]
  pub fn page_promotion_threshold(&self) -> i32 {
    self.get_i32(157)
  }
  #[inline]
  pub fn set_page_promotion_threshold(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(157, v)
  }

  /// min percentage of live bytes on a page to enable fast evacuation in MinorMS
  #[inline]
  pub fn minor_ms_page_promotion_threshold(&self) -> i32 {
    self.get_i32(158)
  }
  #[inline]
  pub fn set_minor_ms_page_promotion_threshold(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(158, v)
  }

  /// max percentage of labs out of a page to still be considered for page promotion
  #[inline]
  pub fn minor_ms_page_promotion_max_lab_threshold(&self) -> i32 {
    self.get_i32(159)
  }
  #[inline]
  pub fn set_minor_ms_page_promotion_max_lab_threshold(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(159, v)
  }

  /// max age for a page after which it is force promoted to old space
  #[inline]
  pub fn minor_ms_max_page_age(&self) -> u32 {
    self.get_u32(160)
  }
  #[inline]
  pub fn set_minor_ms_max_page_age(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(160, v)
  }

  /// trace page promotion decisions (developer-only)
  #[inline]
  pub fn trace_page_promotions(&self) -> bool {
    self.get_bool(161)
  }
  #[inline]
  pub fn set_trace_page_promotions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(161, v)
  }

  /// trace pretenuring decisions of HAllocate instructions (developer-only)
  #[inline]
  pub fn trace_pretenuring(&self) -> bool {
    self.get_bool(162)
  }
  #[inline]
  pub fn set_trace_pretenuring(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(162, v)
  }

  /// trace allocation site pretenuring statistics (developer-only)
  #[inline]
  pub fn trace_pretenuring_statistics(&self) -> bool {
    self.get_bool(163)
  }
  #[inline]
  pub fn set_trace_pretenuring_statistics(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(163, v)
  }

  /// trace resizing of large objects (developer-only)
  #[inline]
  pub fn trace_resize_large_object(&self) -> bool {
    self.get_bool(164)
  }
  #[inline]
  pub fn set_trace_resize_large_object(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(164, v)
  }

  /// track field types
  #[inline]
  pub fn track_field_types(&self) -> bool {
    self.get_bool(165)
  }
  #[inline]
  pub fn set_track_field_types(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(165, v)
  }

  /// trace collected block coverage information (developer-only)
  #[inline]
  pub fn trace_block_coverage(&self) -> bool {
    self.get_bool(166)
  }
  #[inline]
  pub fn set_trace_block_coverage(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(166, v)
  }

  /// trace protector cell invalidations (developer-only)
  #[inline]
  pub fn trace_protector_invalidation(&self) -> bool {
    self.get_bool(167)
  }
  #[inline]
  pub fn set_trace_protector_invalidation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(167, v)
  }

  /// Zero unused memory (except for memory which was discarded) on memory reducing GCs.
  #[inline]
  pub fn zero_unused_memory(&self) -> bool {
    self.get_bool(168)
  }
  #[inline]
  pub fn set_zero_unused_memory(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(168, v)
  }

  /// Enables high-end mode unconditionally for Android.
  #[inline]
  pub fn high_end_android(&self) -> bool {
    self.get_bool(169)
  }
  #[inline]
  pub fn set_high_end_android(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(169, v)
  }

  /// Enables high-end mode for devices with more than X GB of physical memory (if X greater than 0).
  #[inline]
  pub fn high_end_android_physical_memory_threshold(&self) -> u32 {
    self.get_u32(170)
  }
  #[inline]
  pub fn set_high_end_android_physical_memory_threshold(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(170, v)
  }

  /// short cut strings during marking
  #[inline]
  pub fn minor_ms_shortcut_strings(&self) -> bool {
    self.get_bool(171)
  }
  #[inline]
  pub fn set_minor_ms_shortcut_strings(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(171, v)
  }

  /// feed back normalization to constructors (experimental)
  #[inline]
  pub fn feedback_normalization(&self) -> bool {
    self.get_bool(172)
  }
  #[inline]
  pub fn set_feedback_normalization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(172, v)
  }

  /// internalize string keys for generic keyed ICs on the fly
  #[inline]
  pub fn internalize_on_the_fly(&self) -> bool {
    self.get_bool(173)
  }
  #[inline]
  pub fn set_internalize_on_the_fly(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(173, v)
  }

  /// automatically unbox arrays of doubles
  #[inline]
  pub fn unbox_double_arrays(&self) -> bool {
    self.get_bool(174)
  }
  #[inline]
  pub fn set_unbox_double_arrays(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(174, v)
  }

  /// use string slices
  #[inline]
  pub fn string_slices(&self) -> bool {
    self.get_bool(175)
  }
  #[inline]
  pub fn set_string_slices(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(175, v)
  }

  /// invocation count required for allocating feedback vectors
  #[inline]
  pub fn invocation_count_for_feedback_allocation(&self) -> i32 {
    self.get_i32(176)
  }
  #[inline]
  pub fn set_invocation_count_for_feedback_allocation(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(176, v)
  }

  /// invocation count required for optimizing with Maglev
  #[inline]
  pub fn invocation_count_for_maglev(&self) -> i32 {
    self.get_i32(177)
  }
  #[inline]
  pub fn set_invocation_count_for_maglev(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(177, v)
  }

  /// invocation count required for maglev OSR
  #[inline]
  pub fn invocation_count_for_maglev_osr(&self) -> i32 {
    self.get_i32(178)
  }
  #[inline]
  pub fn set_invocation_count_for_maglev_osr(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(178, v)
  }

  /// bitset mode for OSR from Maglev to Turbofan (0=off, 1=on OSR compile, 2=if loop was OSR'd, 4=always)
  #[inline]
  pub fn osr_from_maglev(&self) -> i32 {
    self.get_i32(179)
  }
  #[inline]
  pub fn set_osr_from_maglev(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(179, v)
  }

  /// Scale interrupt budget reduction for OSR from Maglev vs. OSR to Maglev
  #[inline]
  pub fn osr_from_maglev_interrupt_scale_factor(&self) -> f64 {
    self.get_f64(180)
  }
  #[inline]
  pub fn set_osr_from_maglev_interrupt_scale_factor(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(180, v)
  }

  /// whether we try to OSR to Turbofan from any Maglev (alias for --osr-from-maglev=4)
  #[inline]
  pub fn always_osr_from_maglev(&self) -> bool {
    self.get_bool(181)
  }
  #[inline]
  pub fn set_always_osr_from_maglev(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(181, v)
  }

  /// invocation count required for optimizing with TurboFan
  #[inline]
  pub fn invocation_count_for_turbofan(&self) -> i32 {
    self.get_i32(182)
  }
  #[inline]
  pub fn set_invocation_count_for_turbofan(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(182, v)
  }

  /// invocation count required for OSR
  #[inline]
  pub fn invocation_count_for_osr(&self) -> i32 {
    self.get_i32(183)
  }
  #[inline]
  pub fn set_invocation_count_for_osr(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(183, v)
  }

  /// invocation count factor while waiting for code to compile
  #[inline]
  pub fn invocation_count_for_osr_factor_while_tiering_in_progress(&self) -> f64 {
    self.get_f64(184)
  }
  #[inline]
  pub fn set_invocation_count_for_osr_factor_while_tiering_in_progress(
    &mut self,
    v: f64,
  ) -> crate::Result<()> {
    self.assign_f64(184, v)
  }

  /// number to decrease the invocation budget by when we follow OSR
  #[inline]
  pub fn osr_to_tierup(&self) -> u32 {
    self.get_u32(185)
  }
  #[inline]
  pub fn set_osr_to_tierup(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(185, v)
  }

  /// How long to minimally wait after IC update before tier up
  #[inline]
  pub fn minimum_invocations_after_ic_update(&self) -> i32 {
    self.get_i32(186)
  }
  #[inline]
  pub fn set_minimum_invocations_after_ic_update(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(186, v)
  }

  /// Minimum number of invocations we need before non-OSR optimization
  #[inline]
  pub fn minimum_invocations_before_optimization(&self) -> i32 {
    self.get_i32(187)
  }
  #[inline]
  pub fn set_minimum_invocations_before_optimization(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(187, v)
  }

  /// Set JIT tiering thresholds suitable for JIT fuzzing
  #[inline]
  pub fn jit_fuzzing(&self) -> bool {
    self.get_bool(188)
  }
  #[inline]
  pub fn set_jit_fuzzing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(188, v)
  }

  /// use std::pow instead of our custom implementation
  #[inline]
  pub fn use_std_math_pow(&self) -> bool {
    self.get_bool(189)
  }
  #[inline]
  pub fn set_use_std_math_pow(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(189, v)
  }

  /// use inline caching
  #[inline]
  pub fn use_ic(&self) -> bool {
    self.get_bool(190)
  }
  #[inline]
  pub fn set_use_ic(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(190, v)
  }

  /// Allocate feedback vectors lazily
  #[inline]
  pub fn lazy_feedback_allocation(&self) -> bool {
    self.get_bool(191)
  }
  #[inline]
  pub fn set_lazy_feedback_allocation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(191, v)
  }

  /// exercise interesting paths in ICs more often
  #[inline]
  pub fn stress_ic(&self) -> bool {
    self.get_bool(192)
  }
  #[inline]
  pub fn set_stress_ic(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(192, v)
  }

  /// elide bytecodes which won't have any external effect
  #[inline]
  pub fn ignition_elide_noneffectful_bytecodes(&self) -> bool {
    self.get_bool(193)
  }
  #[inline]
  pub fn set_ignition_elide_noneffectful_bytecodes(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(193, v)
  }

  /// use ignition register equivalence optimizer
  #[inline]
  pub fn ignition_reo(&self) -> bool {
    self.get_bool(194)
  }
  #[inline]
  pub fn set_ignition_reo(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(194, v)
  }

  /// filter expression positions before the bytecode pipeline
  #[inline]
  pub fn ignition_filter_expression_positions(&self) -> bool {
    self.get_bool(195)
  }
  #[inline]
  pub fn set_ignition_filter_expression_positions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(195, v)
  }

  /// share feedback slots when loading the same named property from the same object
  #[inline]
  pub fn ignition_share_named_property_feedback(&self) -> bool {
    self.get_bool(196)
  }
  #[inline]
  pub fn set_ignition_share_named_property_feedback(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(196, v)
  }

  /// elide TDZ checks dominated by other TDZ checks
  #[inline]
  pub fn ignition_elide_redundant_tdz_checks(&self) -> bool {
    self.get_bool(197)
  }
  #[inline]
  pub fn set_ignition_elide_redundant_tdz_checks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(197, v)
  }

  /// print bytecode generated by ignition interpreter (developer-only)
  #[inline]
  pub fn print_bytecode(&self) -> bool {
    self.get_bool(198)
  }
  #[inline]
  pub fn set_print_bytecode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(198, v)
  }

  /// skip generating source positions during initial compile but regenerate when actually required
  #[inline]
  pub fn enable_lazy_source_positions(&self) -> bool {
    self.get_bool(199)
  }
  #[inline]
  pub fn set_enable_lazy_source_positions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(199, v)
  }

  /// collect lazy source positions immediately after lazy compile
  #[inline]
  pub fn stress_lazy_source_positions(&self) -> bool {
    self.get_bool(200)
  }
  #[inline]
  pub fn set_stress_lazy_source_positions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(200, v)
  }

  /// stress lazy compilation
  #[inline]
  pub fn stress_lazy(&self) -> bool {
    self.get_bool(201)
  }
  #[inline]
  pub fn set_stress_lazy(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(201, v)
  }

  /// filter for selecting which functions to print bytecode
  #[inline]
  pub fn print_bytecode_filter(&self) -> Option<&str> {
    self.get_str(202)
  }
  #[inline]
  pub fn set_print_bytecode_filter(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(202, Some(Cow::Owned(v.to_string())))
  }

  /// omit calling default ctors in bytecode
  #[inline]
  pub fn omit_default_ctors(&self) -> bool {
    self.get_bool(203)
  }
  #[inline]
  pub fn set_omit_default_ctors(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(203, v)
  }

  /// trace the codegen of ignition interpreter bytecode handlers (developer-only)
  #[inline]
  pub fn trace_ignition_codegen(&self) -> bool {
    self.get_bool(204)
  }
  #[inline]
  pub fn set_trace_ignition_codegen(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(204, v)
  }

  /// trace the tracking of allocation sites (developer-only)
  #[inline]
  pub fn trace_track_allocation_sites(&self) -> bool {
    self.get_bool(205)
  }
  #[inline]
  pub fn set_trace_track_allocation_sites(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(205, v)
  }

  /// trace object migration (developer-only)
  #[inline]
  pub fn trace_migration(&self) -> bool {
    self.get_bool(206)
  }
  #[inline]
  pub fn set_trace_migration(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(206, v)
  }

  /// trace map generalization (developer-only)
  #[inline]
  pub fn trace_generalization(&self) -> bool {
    self.get_bool(207)
  }
  #[inline]
  pub fn set_trace_generalization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(207, v)
  }

  /// enable Sparkplug baseline compiler
  #[inline]
  pub fn sparkplug(&self) -> bool {
    self.get_bool(208)
  }
  #[inline]
  pub fn set_sparkplug(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(208, v)
  }

  /// directly tier up to Sparkplug code
  #[inline]
  pub fn always_sparkplug(&self) -> bool {
    self.get_bool(209)
  }
  #[inline]
  pub fn set_always_sparkplug(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(209, v)
  }

  /// batch compile Sparkplug code
  #[inline]
  pub fn baseline_batch_compilation(&self) -> bool {
    self.get_bool(210)
  }
  #[inline]
  pub fn set_baseline_batch_compilation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(210, v)
  }

  /// compile Sparkplug code in a background thread
  #[inline]
  pub fn concurrent_sparkplug(&self) -> bool {
    self.get_bool(211)
  }
  #[inline]
  pub fn set_concurrent_sparkplug(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(211, v)
  }

  /// max number of threads that concurrent Sparkplug can use (0 for unbounded)
  #[inline]
  pub fn concurrent_sparkplug_max_threads(&self) -> u32 {
    self.get_u32(212)
  }
  #[inline]
  pub fn set_concurrent_sparkplug_max_threads(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(212, v)
  }

  /// use high priority compiler threads for concurrent Sparkplug
  #[inline]
  pub fn concurrent_sparkplug_high_priority_threads(&self) -> bool {
    self.get_bool(213)
  }
  #[inline]
  pub fn set_concurrent_sparkplug_high_priority_threads(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(213, v)
  }

  /// filter for Sparkplug baseline compiler
  #[inline]
  pub fn sparkplug_filter(&self) -> Option<&str> {
    self.get_str(214)
  }
  #[inline]
  pub fn set_sparkplug_filter(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(214, Some(Cow::Owned(v.to_string())))
  }

  /// only enable Sparkplug baseline compiler when --short-builtin-calls are also enabled
  #[inline]
  pub fn sparkplug_needs_short_builtins(&self) -> bool {
    self.get_bool(215)
  }
  #[inline]
  pub fn set_sparkplug_needs_short_builtins(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(215, v)
  }

  /// the estimated instruction size of a batch to trigger compilation
  #[inline]
  pub fn baseline_batch_compilation_threshold(&self) -> i32 {
    self.get_i32(216)
  }
  #[inline]
  pub fn set_baseline_batch_compilation_threshold(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(216, v)
  }

  /// trace baseline compilation (developer-only)
  #[inline]
  pub fn trace_baseline(&self) -> bool {
    self.get_bool(217)
  }
  #[inline]
  pub fn set_trace_baseline(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(217, v)
  }

  /// trace baseline batch compilation (developer-only)
  #[inline]
  pub fn trace_baseline_batch_compilation(&self) -> bool {
    self.get_bool(218)
  }
  #[inline]
  pub fn set_trace_baseline_batch_compilation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(218, v)
  }

  /// allow sharing of strings
  #[inline]
  pub fn shared_strings(&self) -> bool {
    self.get_bool(219)
  }
  #[inline]
  pub fn set_shared_strings(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(219, v)
  }

  /// internalize strings into shared table (experimental)
  #[inline]
  pub fn shared_string_table(&self) -> bool {
    self.get_bool(220)
  }
  #[inline]
  pub fn set_shared_string_table(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(220, v)
  }

  /// use string forwarding table instead of thin strings for all strings (experimental)
  #[inline]
  pub fn always_use_string_forwarding_table(&self) -> bool {
    self.get_bool(221)
  }
  #[inline]
  pub fn set_always_use_string_forwarding_table(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(221, v)
  }

  /// Transition strings during a full GC with stack
  #[inline]
  pub fn transition_strings_during_gc_with_stack(&self) -> bool {
    self.get_bool(222)
  }
  #[inline]
  pub fn set_transition_strings_during_gc_with_stack(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(222, v)
  }

  /// initial size of the shared heap (in Mbytes); other heap size flags (e.g. initial_heap_size) take precedence
  #[inline]
  pub fn initial_shared_heap_size(&self) -> usize {
    self.get_usize(223)
  }
  #[inline]
  pub fn set_initial_shared_heap_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(223, v)
  }

  /// max size of the shared heap (in Mbytes); other heap size flags (e.g. max_shared_heap_size) take precedence
  #[inline]
  pub fn max_shared_heap_size(&self) -> usize {
    self.get_usize(224)
  }
  #[inline]
  pub fn set_max_shared_heap_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(224, v)
  }

  /// generate builtins concurrently on separate threads in mksnapshot
  #[inline]
  pub fn concurrent_builtin_generation(&self) -> bool {
    self.get_bool(225)
  }
  #[inline]
  pub fn set_concurrent_builtin_generation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(225, v)
  }

  /// optimizing hot functions asynchronously on a separate thread
  #[inline]
  pub fn concurrent_recompilation(&self) -> bool {
    self.get_bool(226)
  }
  #[inline]
  pub fn set_concurrent_recompilation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(226, v)
  }

  /// track concurrent recompilation (developer-only)
  #[inline]
  pub fn trace_concurrent_recompilation(&self) -> bool {
    self.get_bool(227)
  }
  #[inline]
  pub fn set_trace_concurrent_recompilation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(227, v)
  }

  /// the length of the concurrent compilation queue
  #[inline]
  pub fn concurrent_recompilation_queue_length(&self) -> i32 {
    self.get_i32(228)
  }
  #[inline]
  pub fn set_concurrent_recompilation_queue_length(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(228, v)
  }

  /// artificial compilation delay in ms
  #[inline]
  pub fn concurrent_recompilation_delay(&self) -> i32 {
    self.get_i32(229)
  }
  #[inline]
  pub fn set_concurrent_recompilation_delay(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(229, v)
  }

  /// move compile jobs to the front if recompilation is requested multiple times
  #[inline]
  pub fn concurrent_recompilation_front_running(&self) -> bool {
    self.get_bool(230)
  }
  #[inline]
  pub fn set_concurrent_recompilation_front_running(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(230, v)
  }

  /// max number of threads that concurrent Turbofan can use (0 for unbounded)
  #[inline]
  pub fn concurrent_turbofan_max_threads(&self) -> u32 {
    self.get_u32(231)
  }
  #[inline]
  pub fn set_concurrent_turbofan_max_threads(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(231, v)
  }

  /// create additional concurrent optimization jobs but throw away result
  #[inline]
  pub fn stress_concurrent_inlining(&self) -> bool {
    self.get_bool(232)
  }
  #[inline]
  pub fn set_stress_concurrent_inlining(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(232, v)
  }

  /// whether maglev resets the interrupt budget
  #[inline]
  pub fn maglev_overwrite_budget(&self) -> bool {
    self.get_bool(233)
  }
  #[inline]
  pub fn set_maglev_overwrite_budget(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(233, v)
  }

  /// whether maglev resets the OSR interrupt budget
  #[inline]
  pub fn maglev_overwrite_osr_budget(&self) -> bool {
    self.get_bool(234)
  }
  #[inline]
  pub fn set_maglev_overwrite_osr_budget(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(234, v)
  }

  /// create additional concurrent optimization jobs
  #[inline]
  pub fn stress_concurrent_inlining_attach_code(&self) -> bool {
    self.get_bool(235)
  }
  #[inline]
  pub fn set_stress_concurrent_inlining_attach_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(235, v)
  }

  /// maximum levels for nesting child serializers
  #[inline]
  pub fn max_serializer_nesting(&self) -> i32 {
    self.get_i32(236)
  }
  #[inline]
  pub fn set_max_serializer_nesting(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(236, v)
  }

  /// trace the heap broker verbosely (all reports) (developer-only)
  #[inline]
  pub fn trace_heap_broker_verbose(&self) -> bool {
    self.get_bool(237)
  }
  #[inline]
  pub fn set_trace_heap_broker_verbose(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(237, v)
  }

  /// trace the heap broker (reports on missing data only) (developer-only)
  #[inline]
  pub fn trace_heap_broker(&self) -> bool {
    self.get_bool(238)
  }
  #[inline]
  pub fn set_trace_heap_broker(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(238, v)
  }

  /// number of stress runs
  #[inline]
  pub fn stress_runs(&self) -> i32 {
    self.get_i32(239)
  }
  #[inline]
  pub fn set_stress_runs(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(239, v)
  }

  /// deoptimize every n times a deopt point is passed
  #[inline]
  pub fn deopt_every_n_times(&self) -> i32 {
    self.get_i32(240)
  }
  #[inline]
  pub fn set_deopt_every_n_times(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(240, v)
  }

  /// print number of possible deopt points
  #[inline]
  pub fn print_deopt_stress(&self) -> bool {
    self.get_bool(241)
  }
  #[inline]
  pub fn set_print_deopt_stress(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(241, v)
  }

  /// disable loop stack checks (for testing/fuzzing only)
  #[inline]
  pub fn disable_loop_stack_checks(&self) -> bool {
    self.get_bool(242)
  }
  #[inline]
  pub fn set_disable_loop_stack_checks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(242, v)
  }

  /// use the Turbofan optimizing compiler for JavaScript code (Wasm uses separate flags)
  #[inline]
  pub fn turbofan(&self) -> bool {
    self.get_bool(243)
  }
  #[inline]
  pub fn set_turbofan(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(243, v)
  }

  /// optimize placement of all spill instructions, not just loop-top phis
  #[inline]
  pub fn turbo_always_optimize_spills(&self) -> bool {
    self.get_bool(244)
  }
  #[inline]
  pub fn set_turbo_always_optimize_spills(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(244, v)
  }

  /// refactor address components for immediate indexing
  #[inline]
  pub fn turbo_wasm_address_reassociation(&self) -> bool {
    self.get_bool(245)
  }
  #[inline]
  pub fn set_turbo_wasm_address_reassociation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(245, v)
  }

  /// allow concurrent compilation to happen in combination with trace-turbo* flags
  #[inline]
  pub fn concurrent_turbo_tracing(&self) -> bool {
    self.get_bool(246)
  }
  #[inline]
  pub fn set_concurrent_turbo_tracing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(246, v)
  }

  /// make OptimizeMaglevOnNextCall optimize to turbofan instead of maglev
  #[inline]
  pub fn optimize_maglev_optimizes_to_turbofan(&self) -> bool {
    self.get_bool(247)
  }
  #[inline]
  pub fn set_optimize_maglev_optimizes_to_turbofan(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(247, v)
  }

  /// optimization filter for TurboFan compiler
  #[inline]
  pub fn turbo_filter(&self) -> Option<&str> {
    self.get_str(248)
  }
  #[inline]
  pub fn set_turbo_filter(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(248, Some(Cow::Owned(v.to_string())))
  }

  /// trace generated TurboFan IR (developer-only)
  #[inline]
  pub fn trace_turbo(&self) -> bool {
    self.get_bool(249)
  }
  #[inline]
  pub fn set_trace_turbo(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(249, v)
  }

  /// directory to dump generated TurboFan IR to
  #[inline]
  pub fn trace_turbo_path(&self) -> Option<&str> {
    self.get_str(250)
  }
  #[inline]
  pub fn set_trace_turbo_path(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(250, Some(Cow::Owned(v.to_string())))
  }

  /// filter for tracing turbofan compilation
  #[inline]
  pub fn trace_turbo_filter(&self) -> Option<&str> {
    self.get_str(251)
  }
  #[inline]
  pub fn set_trace_turbo_filter(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(251, Some(Cow::Owned(v.to_string())))
  }

  /// trace generated TurboFan graphs (developer-only)
  #[inline]
  pub fn trace_turbo_graph(&self) -> bool {
    self.get_bool(252)
  }
  #[inline]
  pub fn set_trace_turbo_graph(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(252, v)
  }

  /// trace TurboFan IR with schedule (developer-only)
  #[inline]
  pub fn trace_turbo_scheduled(&self) -> bool {
    self.get_bool(253)
  }
  #[inline]
  pub fn set_trace_turbo_scheduled(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(253, v)
  }

  /// trace turbo graph to a file with given prefix
  #[inline]
  pub fn trace_turbo_file_prefix(&self) -> Option<&str> {
    self.get_str(254)
  }
  #[inline]
  pub fn set_trace_turbo_file_prefix(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(254, Some(Cow::Owned(v.to_string())))
  }

  /// trace turbo cfg graph (for C1 visualizer) to a given file name
  #[inline]
  pub fn trace_turbo_cfg_file(&self) -> Option<&str> {
    self.get_str(255)
  }
  #[inline]
  pub fn set_trace_turbo_cfg_file(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(255, Some(Cow::Owned(v.to_string())))
  }

  /// trace TurboFan's types
  #[inline]
  pub fn trace_turbo_types(&self) -> bool {
    self.get_bool(256)
  }
  #[inline]
  pub fn set_trace_turbo_types(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(256, v)
  }

  /// trace TurboFan's scheduler
  #[inline]
  pub fn trace_turbo_scheduler(&self) -> bool {
    self.get_bool(257)
  }
  #[inline]
  pub fn set_trace_turbo_scheduler(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(257, v)
  }

  /// trace TurboFan's various reducers
  #[inline]
  pub fn trace_turbo_reduction(&self) -> bool {
    self.get_bool(258)
  }
  #[inline]
  pub fn set_trace_turbo_reduction(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(258, v)
  }

  /// trace TurboFan's various bailouts
  #[inline]
  pub fn trace_turbo_bailouts(&self) -> bool {
    self.get_bool(259)
  }
  #[inline]
  pub fn set_trace_turbo_bailouts(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(259, v)
  }

  /// trace TurboFan's graph trimmer
  #[inline]
  pub fn trace_turbo_trimming(&self) -> bool {
    self.get_bool(260)
  }
  #[inline]
  pub fn set_trace_turbo_trimming(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(260, v)
  }

  /// trace TurboFan's jump threading
  #[inline]
  pub fn trace_turbo_jt(&self) -> bool {
    self.get_bool(261)
  }
  #[inline]
  pub fn set_trace_turbo_jt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(261, v)
  }

  /// trace TurboFan's control equivalence
  #[inline]
  pub fn trace_turbo_ceq(&self) -> bool {
    self.get_bool(262)
  }
  #[inline]
  pub fn set_trace_turbo_ceq(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(262, v)
  }

  /// trace TurboFan's loop optimizations
  #[inline]
  pub fn trace_turbo_loop(&self) -> bool {
    self.get_bool(263)
  }
  #[inline]
  pub fn set_trace_turbo_loop(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(263, v)
  }

  /// trace TurboFan's register allocator
  #[inline]
  pub fn trace_turbo_alloc(&self) -> bool {
    self.get_bool(264)
  }
  #[inline]
  pub fn set_trace_turbo_alloc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(264, v)
  }

  /// trace all use positions
  #[inline]
  pub fn trace_all_uses(&self) -> bool {
    self.get_bool(265)
  }
  #[inline]
  pub fn set_trace_all_uses(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(265, v)
  }

  /// trace representation types
  #[inline]
  pub fn trace_representation(&self) -> bool {
    self.get_bool(266)
  }
  #[inline]
  pub fn set_trace_representation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(266, v)
  }

  /// trace stack load/store counters for optimized code in run-time (x64 only)
  #[inline]
  pub fn trace_turbo_stack_accesses(&self) -> bool {
    self.get_bool(267)
  }
  #[inline]
  pub fn set_trace_turbo_stack_accesses(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(267, v)
  }

  /// fuzzing && concurrent_recompilation
  #[inline]
  pub fn fuzzing_and_concurrent_recompilation(&self) -> bool {
    self.get_bool(268)
  }
  #[inline]
  pub fn set_fuzzing_and_concurrent_recompilation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(268, v)
  }

  /// verify TurboFan graphs at each phase
  #[inline]
  pub fn turbo_verify(&self) -> bool {
    self.get_bool(269)
  }
  #[inline]
  pub fn set_turbo_verify(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(269, v)
  }

  /// verify TurboFan machine graph before instruction selection
  #[inline]
  pub fn turbo_verify_machine_graph(&self) -> Option<&str> {
    self.get_str(270)
  }
  #[inline]
  pub fn set_turbo_verify_machine_graph(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(270, Some(Cow::Owned(v.to_string())))
  }

  /// verify TurboFan machine graph of code stubs
  #[inline]
  pub fn verify_csa(&self) -> bool {
    self.get_bool(271)
  }
  #[inline]
  pub fn set_verify_csa(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(271, v)
  }

  /// trace code stubs verification (developer-only)
  #[inline]
  pub fn trace_verify_csa(&self) -> bool {
    self.get_bool(272)
  }
  #[inline]
  pub fn set_trace_verify_csa(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(272, v)
  }

  /// trigger break point when a node with given id is created in given stub. The format is: StubName,NodeId
  #[inline]
  pub fn csa_trap_on_node(&self) -> Option<&str> {
    self.get_str(273)
  }
  #[inline]
  pub fn set_csa_trap_on_node(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(273, Some(Cow::Owned(v.to_string())))
  }

  /// path where to generate builtins-effects.h (used for mksnapshot only)
  #[inline]
  pub fn builtins_effects_src(&self) -> Option<&str> {
    self.get_str(274)
  }
  #[inline]
  pub fn set_builtins_effects_src(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(274, Some(Cow::Owned(v.to_string())))
  }

  /// enable FixedArray bounds checks
  #[inline]
  pub fn fixed_array_bounds_checks(&self) -> bool {
    self.get_bool(275)
  }
  #[inline]
  pub fn set_fixed_array_bounds_checks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(275, v)
  }

  /// print TurboFan statistics
  #[inline]
  pub fn turbo_stats(&self) -> bool {
    self.get_bool(276)
  }
  #[inline]
  pub fn set_turbo_stats(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(276, v)
  }

  /// print TurboFan statistics in machine-readable format
  #[inline]
  pub fn turbo_stats_nvp(&self) -> bool {
    self.get_bool(277)
  }
  #[inline]
  pub fn set_turbo_stats_nvp(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(277, v)
  }

  /// print TurboFan statistics of wasm compilations
  #[inline]
  pub fn turbo_stats_wasm(&self) -> bool {
    self.get_bool(278)
  }
  #[inline]
  pub fn set_turbo_stats_wasm(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(278, v)
  }

  /// split nodes during scheduling in TurboFan
  #[inline]
  pub fn turbo_splitting(&self) -> bool {
    self.get_bool(279)
  }
  #[inline]
  pub fn set_turbo_splitting(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(279, v)
  }

  /// enable inlining in TurboFan
  #[inline]
  pub fn turbo_inlining(&self) -> bool {
    self.get_bool(280)
  }
  #[inline]
  pub fn set_turbo_inlining(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(280, v)
  }

  /// optimize for-in over packed elements using ForInEnumeratorHolder
  #[inline]
  pub fn forin_enumerator_holder(&self) -> bool {
    self.get_bool(281)
  }
  #[inline]
  pub fn set_forin_enumerator_holder(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(281, v)
  }

  /// enable frame elision in TurboFan
  #[inline]
  pub fn turbo_elide_frames(&self) -> bool {
    self.get_bool(282)
  }
  #[inline]
  pub fn set_turbo_elide_frames(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(282, v)
  }

  /// maximum size of bytecode for a single inlining
  #[inline]
  pub fn max_inlined_bytecode_size(&self) -> i32 {
    self.get_i32(283)
  }
  #[inline]
  pub fn set_max_inlined_bytecode_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(283, v)
  }

  /// maximum cumulative size of bytecode considered for inlining
  #[inline]
  pub fn max_inlined_bytecode_size_cumulative(&self) -> i32 {
    self.get_i32(284)
  }
  #[inline]
  pub fn set_max_inlined_bytecode_size_cumulative(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(284, v)
  }

  /// maximum absolute size of bytecode considered for inlining
  #[inline]
  pub fn max_inlined_bytecode_size_absolute(&self) -> i32 {
    self.get_i32(285)
  }
  #[inline]
  pub fn set_max_inlined_bytecode_size_absolute(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(285, v)
  }

  /// maximum size of small functions' bytecode that can be inlining
  #[inline]
  pub fn max_inlined_bytecode_size_small_total(&self) -> i32 {
    self.get_i32(286)
  }
  #[inline]
  pub fn set_max_inlined_bytecode_size_small_total(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(286, v)
  }

  /// scale factor of bytecode size used to calculate the inlining budget
  #[inline]
  pub fn reserve_inline_budget_scale_factor(&self) -> f64 {
    self.get_f64(287)
  }
  #[inline]
  pub fn set_reserve_inline_budget_scale_factor(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(287, v)
  }

  /// maximum size of bytecode considered for small function inlining
  #[inline]
  pub fn max_inlined_bytecode_size_small(&self) -> i32 {
    self.get_i32(288)
  }
  #[inline]
  pub fn set_max_inlined_bytecode_size_small(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(288, v)
  }

  /// maximum size of bytecode considered for small function inlining, when the function call has a HeapNumber as input or output
  #[inline]
  pub fn max_inlined_bytecode_size_small_with_heapnum_in_out(&self) -> i32 {
    self.get_i32(289)
  }
  #[inline]
  pub fn set_max_inlined_bytecode_size_small_with_heapnum_in_out(
    &mut self,
    v: i32,
  ) -> crate::Result<()> {
    self.assign_i32(289, v)
  }

  /// maximum bytecode size to be considered for turbofan optimization; too high values may cause the compiler to hit (release) assertions
  #[inline]
  pub fn max_optimized_bytecode_size(&self) -> i32 {
    self.get_i32(290)
  }
  #[inline]
  pub fn set_max_optimized_bytecode_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(290, v)
  }

  /// maximum bytecode size to be considered for maglev optimization
  #[inline]
  pub fn max_maglev_optimized_bytecode_size(&self) -> i32 {
    self.get_i32(291)
  }
  #[inline]
  pub fn set_max_maglev_optimized_bytecode_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(291, v)
  }

  /// minimum frequency for inlining
  #[inline]
  pub fn min_inlining_frequency(&self) -> f64 {
    self.get_f64(292)
  }
  #[inline]
  pub fn set_min_inlining_frequency(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(292, v)
  }

  /// set high thresholds for inlining to inline as much as possible
  #[inline]
  pub fn stress_inline(&self) -> bool {
    self.get_bool(293)
  }
  #[inline]
  pub fn set_stress_inline(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(293, v)
  }

  /// trace TurboFan inlining (developer-only)
  #[inline]
  pub fn trace_turbo_inlining(&self) -> bool {
    self.get_bool(294)
  }
  #[inline]
  pub fn set_trace_turbo_inlining(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(294, v)
  }

  /// inline array builtins in TurboFan code
  #[inline]
  pub fn turbo_inline_array_builtins(&self) -> bool {
    self.get_bool(295)
  }
  #[inline]
  pub fn set_turbo_inline_array_builtins(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(295, v)
  }

  /// use on-stack replacement
  #[inline]
  pub fn use_osr(&self) -> bool {
    self.get_bool(296)
  }
  #[inline]
  pub fn set_use_osr(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(296, v)
  }

  /// use maglev as on-stack replacement target
  #[inline]
  pub fn maglev_osr(&self) -> bool {
    self.get_bool(297)
  }
  #[inline]
  pub fn set_maglev_osr(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(297, v)
  }

  /// maximum size of bytecode considered for eager inlining
  #[inline]
  pub fn max_maglev_eager_inlined_bytecode_size(&self) -> i32 {
    self.get_i32(298)
  }
  #[inline]
  pub fn set_max_maglev_eager_inlined_bytecode_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(298, v)
  }

  /// maximum size of bytecode considered for eager inlining
  #[inline]
  pub fn max_turbolev_eager_inlined_bytecode_size(&self) -> i32 {
    self.get_i32(299)
  }
  #[inline]
  pub fn set_max_turbolev_eager_inlined_bytecode_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(299, v)
  }

  /// enable concurrent OSR
  #[inline]
  pub fn concurrent_osr(&self) -> bool {
    self.get_bool(300)
  }
  #[inline]
  pub fn set_concurrent_osr(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(300, v)
  }

  /// maglev allocation folding level
  #[inline]
  pub fn maglev_allocation_folding(&self) -> i32 {
    self.get_i32(301)
  }
  #[inline]
  pub fn set_maglev_allocation_folding(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(301, v)
  }

  /// avoid inlined allocation of objects that cannot escape
  #[inline]
  pub fn maglev_escape_analysis(&self) -> bool {
    self.get_bool(302)
  }
  #[inline]
  pub fn set_maglev_escape_analysis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(302, v)
  }

  /// trace maglev escape analysis (developer-only)
  #[inline]
  pub fn trace_maglev_escape_analysis(&self) -> bool {
    self.get_bool(303)
  }
  #[inline]
  pub fn set_trace_maglev_escape_analysis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(303, v)
  }

  /// track object changes to avoid escaping them (experimental)
  #[inline]
  pub fn maglev_object_tracking(&self) -> bool {
    self.get_bool(304)
  }
  #[inline]
  pub fn set_maglev_object_tracking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(304, v)
  }

  /// trace load/stores from maglev virtual objects (developer-only)
  #[inline]
  pub fn trace_maglev_object_tracking(&self) -> bool {
    self.get_bool(305)
  }
  #[inline]
  pub fn set_trace_maglev_object_tracking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(305, v)
  }

  /// use TurboFan fast string builder
  #[inline]
  pub fn turbo_string_builder(&self) -> bool {
    self.get_bool(306)
  }
  #[inline]
  pub fn set_turbo_string_builder(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(306, v)
  }

  /// trace on-stack replacement (developer-only)
  #[inline]
  pub fn trace_osr(&self) -> bool {
    self.get_bool(307)
  }
  #[inline]
  pub fn set_trace_osr(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(307, v)
  }

  /// internal helper flag, please use --trace-osr instead.
  #[inline]
  pub fn log_or_trace_osr(&self) -> bool {
    self.get_bool(308)
  }
  #[inline]
  pub fn set_log_or_trace_osr(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(308, v)
  }

  /// analyze liveness of environment slots and zap dead values
  #[inline]
  pub fn analyze_environment_liveness(&self) -> bool {
    self.get_bool(309)
  }
  #[inline]
  pub fn set_analyze_environment_liveness(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(309, v)
  }

  /// trace liveness of local variable slots (developer-only)
  #[inline]
  pub fn trace_environment_liveness(&self) -> bool {
    self.get_bool(310)
  }
  #[inline]
  pub fn set_trace_environment_liveness(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(310, v)
  }

  /// enable load elimination in TurboFan
  #[inline]
  pub fn turbo_load_elimination(&self) -> bool {
    self.get_bool(311)
  }
  #[inline]
  pub fn set_turbo_load_elimination(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(311, v)
  }

  /// trace TurboFan load elimination (developer-only)
  #[inline]
  pub fn trace_turbo_load_elimination(&self) -> bool {
    self.get_bool(312)
  }
  #[inline]
  pub fn set_trace_turbo_load_elimination(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(312, v)
  }

  /// enable basic block profiling in TurboFan
  #[inline]
  pub fn turbo_profiling(&self) -> bool {
    self.get_bool(313)
  }
  #[inline]
  pub fn set_turbo_profiling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(313, v)
  }

  /// enable basic block profiling in TurboFan, and include each function's schedule and disassembly in the output
  #[inline]
  pub fn turbo_profiling_verbose(&self) -> bool {
    self.get_bool(314)
  }
  #[inline]
  pub fn set_turbo_profiling_verbose(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(314, v)
  }

  /// emit data about basic block usage in builtins to this file (requires that V8 was built with v8_enable_builtins_profiling=true)
  #[inline]
  pub fn turbo_profiling_output(&self) -> Option<&str> {
    self.get_str(315)
  }
  #[inline]
  pub fn set_turbo_profiling_output(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(315, Some(Cow::Owned(v.to_string())))
  }

  /// enable builtin reordering when run mksnapshot.
  #[inline]
  pub fn reorder_builtins(&self) -> bool {
    self.get_bool(316)
  }
  #[inline]
  pub fn set_reorder_builtins(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(316, v)
  }

  /// flag for mksnapshot, abort if builtins profile can't be applied
  #[inline]
  pub fn abort_on_bad_builtin_profile_data(&self) -> bool {
    self.get_bool(317)
  }
  #[inline]
  pub fn set_abort_on_bad_builtin_profile_data(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(317, v)
  }

  /// flag for mksnapshot, emit warnings when applying builtin profile data
  #[inline]
  pub fn warn_about_builtin_profile_data(&self) -> bool {
    self.get_bool(318)
  }
  #[inline]
  pub fn set_warn_about_builtin_profile_data(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(318, v)
  }

  /// flag for mksnapshot, dump CSA builtins graph hashes to this file
  #[inline]
  pub fn dump_builtins_hashes_to_file(&self) -> Option<&str> {
    self.get_str(319)
  }
  #[inline]
  pub fn set_dump_builtins_hashes_to_file(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(319, Some(Cow::Owned(v.to_string())))
  }

  /// verify register allocation in TurboFan
  #[inline]
  pub fn turbo_verify_allocation(&self) -> bool {
    self.get_bool(320)
  }
  #[inline]
  pub fn set_turbo_verify_allocation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(320, v)
  }

  /// optimize gap moves in TurboFan
  #[inline]
  pub fn turbo_move_optimization(&self) -> bool {
    self.get_bool(321)
  }
  #[inline]
  pub fn set_turbo_move_optimization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(321, v)
  }

  /// enable jump threading in TurboFan
  #[inline]
  pub fn turbo_jt(&self) -> bool {
    self.get_bool(322)
  }
  #[inline]
  pub fn set_turbo_jt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(322, v)
  }

  /// TurboFan loop peeling
  #[inline]
  pub fn turbo_loop_peeling(&self) -> bool {
    self.get_bool(323)
  }
  #[inline]
  pub fn set_turbo_loop_peeling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(323, v)
  }

  /// TurboFan loop variable optimization
  #[inline]
  pub fn turbo_loop_variable(&self) -> bool {
    self.get_bool(324)
  }
  #[inline]
  pub fn set_turbo_loop_variable(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(324, v)
  }

  /// TurboFan loop rotation
  #[inline]
  pub fn turbo_loop_rotation(&self) -> bool {
    self.get_bool(325)
  }
  #[inline]
  pub fn set_turbo_loop_rotation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(325, v)
  }

  /// optimize control flow in TurboFan
  #[inline]
  pub fn turbo_cf_optimization(&self) -> bool {
    self.get_bool(326)
  }
  #[inline]
  pub fn set_turbo_cf_optimization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(326, v)
  }

  /// enable escape analysis
  #[inline]
  pub fn turbo_escape(&self) -> bool {
    self.get_bool(327)
  }
  #[inline]
  pub fn set_turbo_escape(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(327, v)
  }

  /// TurboFan allocation folding
  #[inline]
  pub fn turbo_allocation_folding(&self) -> bool {
    self.get_bool(328)
  }
  #[inline]
  pub fn set_turbo_allocation_folding(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(328, v)
  }

  /// enable instruction scheduling in TurboFan
  #[inline]
  pub fn turbo_instruction_scheduling(&self) -> bool {
    self.get_bool(329)
  }
  #[inline]
  pub fn set_turbo_instruction_scheduling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(329, v)
  }

  /// enable experimental instruction scheduling in TurboFan
  #[inline]
  pub fn experimental_turbo_instruction_scheduling(&self) -> bool {
    self.get_bool(330)
  }
  #[inline]
  pub fn set_experimental_turbo_instruction_scheduling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(330, v)
  }

  /// randomly schedule instructions to stress dependency tracking
  #[inline]
  pub fn turbo_stress_instruction_scheduling(&self) -> bool {
    self.get_bool(331)
  }
  #[inline]
  pub fn set_turbo_stress_instruction_scheduling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(331, v)
  }

  /// trace TurboShaft's instruction scheduler
  #[inline]
  pub fn trace_turbo_instruction_scheduling(&self) -> bool {
    self.get_bool(332)
  }
  #[inline]
  pub fn set_trace_turbo_instruction_scheduling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(332, v)
  }

  /// enable store-store elimination in TurboFan
  #[inline]
  pub fn turbo_store_elimination(&self) -> bool {
    self.get_bool(333)
  }
  #[inline]
  pub fn set_turbo_store_elimination(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(333, v)
  }

  /// trace store elimination (developer-only)
  #[inline]
  pub fn trace_store_elimination(&self) -> bool {
    self.get_bool(334)
  }
  #[inline]
  pub fn set_trace_store_elimination(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(334, v)
  }

  /// extra bounds checks to protect against some known typer mismatch exploit techniques (best effort)
  #[inline]
  pub fn turbo_typer_hardening(&self) -> bool {
    self.get_bool(335)
  }
  #[inline]
  pub fn set_turbo_typer_hardening(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(335, v)
  }

  /// rewrite far to near jumps (ia32,x64)
  #[inline]
  pub fn turbo_rewrite_far_jumps(&self) -> bool {
    self.get_bool(336)
  }
  #[inline]
  pub fn set_turbo_rewrite_far_jumps(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(336, v)
  }

  /// simulate GC/compiler thread race related to https://crbug.com/v8/8520
  #[inline]
  pub fn stress_gc_during_compilation(&self) -> bool {
    self.get_bool(337)
  }
  #[inline]
  pub fn set_stress_gc_during_compilation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(337, v)
  }

  /// enable fast API calls from TurboFan
  #[inline]
  pub fn turbo_fast_api_calls(&self) -> bool {
    self.get_bool(338)
  }
  #[inline]
  pub fn set_turbo_fast_api_calls(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(338, v)
  }

  /// allow float parameters to be passed in simulator mode
  #[inline]
  pub fn fast_api_allow_float_in_sim(&self) -> bool {
    self.get_bool(339)
  }
  #[inline]
  pub fn set_fast_api_allow_float_in_sim(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(339, v)
  }

  /// inline JS->Wasm calls (specifically: inline JS-to-Wasm wrappers and then the body of the Wasm function, if applicable)
  #[inline]
  pub fn turbo_inline_js_wasm_calls(&self) -> bool {
    self.get_bool(340)
  }
  #[inline]
  pub fn set_turbo_inline_js_wasm_calls(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(340, v)
  }

  /// Run additional optimizations (especially load-elimination) on inlined JS-to-Wasm wrappers
  #[inline]
  pub fn turbo_optimize_inlined_js_wasm_wrappers(&self) -> bool {
    self.get_bool(341)
  }
  #[inline]
  pub fn set_turbo_optimize_inlined_js_wasm_wrappers(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(341, v)
  }

  /// optimize Function.prototype.apply
  #[inline]
  pub fn turbo_optimize_apply(&self) -> bool {
    self.get_bool(342)
  }
  #[inline]
  pub fn set_turbo_optimize_apply(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(342, v)
  }

  /// optimize call math.min/max with double array
  #[inline]
  pub fn turbo_optimize_math_minmax(&self) -> bool {
    self.get_bool(343)
  }
  #[inline]
  pub fn set_turbo_optimize_math_minmax(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(343, v)
  }

  /// enable TurboFan's Turboshaft phases for JS
  #[inline]
  pub fn turboshaft(&self) -> bool {
    self.get_bool(344)
  }
  #[inline]
  pub fn set_turboshaft(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(344, v)
  }

  /// enables Turboshaft's DebugPrint, StaticAssert and CheckTurboshaftTypeOf operations
  #[inline]
  pub fn turboshaft_enable_debug_features(&self) -> bool {
    self.get_bool(345)
  }
  #[inline]
  pub fn set_turboshaft_enable_debug_features(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(345, v)
  }

  /// enable Turboshaft's WasmLoadElimination
  #[inline]
  pub fn turboshaft_wasm_load_elimination(&self) -> bool {
    self.get_bool(346)
  }
  #[inline]
  pub fn set_turboshaft_wasm_load_elimination(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(346, v)
  }

  /// inline Wasm code into JS functions via Turboshaft. This requires inlining the wrapper, see --wasm-in-js-inlining-wrapper. For controlling the inlining in the old Turbofan pipeline, see --turbo-inline-js-wasm-calls.
  #[inline]
  pub fn wasm_in_js_inlining_body(&self) -> bool {
    self.get_bool(347)
  }
  #[inline]
  pub fn set_wasm_in_js_inlining_body(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(347, v)
  }

  /// inline JS-to-Wasm wrappers via Turboshaft/Turbolev.
  #[inline]
  pub fn wasm_in_js_inlining_wrapper(&self) -> bool {
    self.get_bool(348)
  }
  #[inline]
  pub fn set_wasm_in_js_inlining_wrapper(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(348, v)
  }

  /// enable optimizations for Wasm-in-JS inlining in Turboshaft (experimental)
  #[inline]
  pub fn wasm_in_js_inlining_opt(&self) -> bool {
    self.get_bool(349)
  }
  #[inline]
  pub fn set_wasm_in_js_inlining_opt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(349, v)
  }

  /// enable Turboshaft's low-level load elimination for JS
  #[inline]
  pub fn turboshaft_load_elimination(&self) -> bool {
    self.get_bool(350)
  }
  #[inline]
  pub fn set_turboshaft_load_elimination(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(350, v)
  }

  /// enable Turboshaft's loop unrolling
  #[inline]
  pub fn turboshaft_loop_unrolling(&self) -> bool {
    self.get_bool(351)
  }
  #[inline]
  pub fn set_turboshaft_loop_unrolling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(351, v)
  }

  /// enable Turboshaft's loop optimization phase (experimental)
  #[inline]
  pub fn turboshaft_loop_optimization(&self) -> bool {
    self.get_bool(352)
  }
  #[inline]
  pub fn set_turboshaft_loop_optimization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(352, v)
  }

  /// enable Turboshaft's random rescheduling phase (experimental)
  #[inline]
  pub fn turboshaft_random_rescheduling(&self) -> bool {
    self.get_bool(353)
  }
  #[inline]
  pub fn set_turboshaft_random_rescheduling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(353, v)
  }

  /// enable Turboshaft's escape analysis for string concatenation
  #[inline]
  pub fn turboshaft_string_concat_escape_analysis(&self) -> bool {
    self.get_bool(354)
  }
  #[inline]
  pub fn set_turboshaft_string_concat_escape_analysis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(354, v)
  }

  /// enable Turboshaft's low level load elimination for trusted loads (JS and Wasm)
  #[inline]
  pub fn turboshaft_trusted_load_elimination(&self) -> bool {
    self.get_bool(355)
  }
  #[inline]
  pub fn set_turboshaft_trusted_load_elimination(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(355, v)
  }

  /// enable an additional Turboshaft phase that performs optimizations based on type information (experimental)
  #[inline]
  pub fn turboshaft_typed_optimizations(&self) -> bool {
    self.get_bool(356)
  }
  #[inline]
  pub fn set_turboshaft_typed_optimizations(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(356, v)
  }

  /// enable optimizations for Webassembly SIMD
  #[inline]
  pub fn wasm_simd_opt(&self) -> bool {
    self.get_bool(357)
  }
  #[inline]
  pub fn set_wasm_simd_opt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(357, v)
  }

  /// enable extra optimizations for Webassembly SIMD (experimental)
  #[inline]
  pub fn future_wasm_simd_opt(&self) -> bool {
    self.get_bool(358)
  }
  #[inline]
  pub fn set_future_wasm_simd_opt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(358, v)
  }

  /// enable deinterleaving loads for Webassembly SIMD (experimental)
  #[inline]
  pub fn wasm_deinterleave_loads(&self) -> bool {
    self.get_bool(359)
  }
  #[inline]
  pub fn set_wasm_deinterleave_loads(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(359, v)
  }

  /// use Turbolev (≈ Maglev + Turboshaft combined) as the 4th tier compiler instead of Turbofan
  #[inline]
  pub fn turbolev(&self) -> bool {
    self.get_bool(360)
  }
  #[inline]
  pub fn set_turbolev(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(360, v)
  }

  /// print Turbolev frontend (Maglev graphs) (developer-only)
  #[inline]
  pub fn print_turbolev_frontend(&self) -> bool {
    self.get_bool(361)
  }
  #[inline]
  pub fn set_print_turbolev_frontend(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(361, v)
  }

  /// filter for turbolev phases (comma-separated list of phase names)
  #[inline]
  pub fn turbolev_phase_filter(&self) -> Option<&str> {
    self.get_str(362)
  }
  #[inline]
  pub fn set_turbolev_phase_filter(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(362, Some(Cow::Owned(v.to_string())))
  }

  /// print Turbolev inline functions (developer-only)
  #[inline]
  pub fn print_turbolev_inline_functions(&self) -> bool {
    self.get_bool(363)
  }
  #[inline]
  pub fn set_print_turbolev_inline_functions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(363, v)
  }

  /// enable Turbolev features that we want to ship in the not-too-far future (experimental)
  #[inline]
  pub fn turbolev_future(&self) -> bool {
    self.get_bool(364)
  }
  #[inline]
  pub fn set_turbolev_future(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(364, v)
  }

  /// enable escape analysis for Turbolev (experimental)
  #[inline]
  pub fn turbolev_escape_analysis(&self) -> bool {
    self.get_bool(365)
  }
  #[inline]
  pub fn set_turbolev_escape_analysis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(365, v)
  }

  /// enable tracing for Turbolev's escape analysis phase (developer-only)
  #[inline]
  pub fn trace_turbolev_escape_analysis(&self) -> bool {
    self.get_bool(366)
  }
  #[inline]
  pub fn set_trace_turbolev_escape_analysis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(366, v)
  }

  /// Enable specializing loading the TypedArray length in Maglev / Turbofan
  #[inline]
  pub fn typed_array_length_loading(&self) -> bool {
    self.get_bool(367)
  }
  #[inline]
  pub fn set_typed_array_length_loading(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(367, v)
  }

  /// avoid duplicated HeapNumber materializations in HeapNumberRequests
  #[inline]
  pub fn deduplicate_heap_number_requests(&self) -> bool {
    self.get_bool(368)
  }
  #[inline]
  pub fn set_deduplicate_heap_number_requests(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(368, v)
  }

  /// trace individual Turboshaft reduction steps
  #[inline]
  pub fn turboshaft_trace_reduction(&self) -> bool {
    self.get_bool(369)
  }
  #[inline]
  pub fn set_turboshaft_trace_reduction(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(369, v)
  }

  /// trace emitted Turboshaft instructions
  #[inline]
  pub fn turboshaft_trace_emitted(&self) -> bool {
    self.get_bool(370)
  }
  #[inline]
  pub fn set_turboshaft_trace_emitted(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(370, v)
  }

  /// trace intermediate Turboshaft reduction steps
  #[inline]
  pub fn turboshaft_trace_intermediate_reductions(&self) -> bool {
    self.get_bool(371)
  }
  #[inline]
  pub fn set_turboshaft_trace_intermediate_reductions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(371, v)
  }

  /// trace Turboshaft's late load elimination
  #[inline]
  pub fn turboshaft_trace_load_elimination(&self) -> bool {
    self.get_bool(372)
  }
  #[inline]
  pub fn set_turboshaft_trace_load_elimination(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(372, v)
  }

  /// trace Turboshaft's store store elimination
  #[inline]
  pub fn turboshaft_trace_store_store_elimination(&self) -> bool {
    self.get_bool(373)
  }
  #[inline]
  pub fn set_turboshaft_trace_store_store_elimination(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(373, v)
  }

  /// trace Turboshaft's if-else to switch reducer
  #[inline]
  pub fn turboshaft_trace_if_else_to_switch(&self) -> bool {
    self.get_bool(374)
  }
  #[inline]
  pub fn set_turboshaft_trace_if_else_to_switch(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(374, v)
  }

  /// enable various verifications on Turboshaft
  #[inline]
  pub fn verify_turboshaft(&self) -> bool {
    self.get_bool(375)
  }
  #[inline]
  pub fn set_verify_turboshaft(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(375, v)
  }

  /// profile guided optimization
  #[inline]
  pub fn profile_guided_optimization(&self) -> bool {
    self.get_bool(376)
  }
  #[inline]
  pub fn set_profile_guided_optimization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(376, v)
  }

  /// profile guided optimization for empty feedback vector
  #[inline]
  pub fn profile_guided_optimization_for_empty_feedback_vector(&self) -> bool {
    self.get_bool(377)
  }
  #[inline]
  pub fn set_profile_guided_optimization_for_empty_feedback_vector(
    &mut self,
    v: bool,
  ) -> crate::Result<()> {
    self.assign_bool(377, v)
  }

  /// invocation count threshold for early optimization
  #[inline]
  pub fn invocation_count_for_early_optimization(&self) -> i32 {
    self.get_i32(378)
  }
  #[inline]
  pub fn set_invocation_count_for_early_optimization(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(378, v)
  }

  /// invocation count for maglev for functions which according to profile_guided_optimization are likely to deoptimize before reaching this invocation count
  #[inline]
  pub fn invocation_count_for_maglev_with_delay(&self) -> i32 {
    self.get_i32(379)
  }
  #[inline]
  pub fn set_invocation_count_for_maglev_with_delay(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(379, v)
  }

  /// Enables optimizations which favor memory size over execution speed
  #[inline]
  pub fn optimize_for_size(&self) -> bool {
    self.get_bool(380)
  }
  #[inline]
  pub fn set_optimize_for_size(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(380, v)
  }

  /// Immediately re-optimize code after some lazy deopts
  #[inline]
  pub fn reopt_after_lazy_deopts(&self) -> bool {
    self.get_bool(381)
  }
  #[inline]
  pub fn set_reopt_after_lazy_deopts(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(381, v)
  }

  /// Enable verification of Builtins::GetJSBuiltinState().
  #[inline]
  pub fn verify_get_js_builtin_state(&self) -> bool {
    self.get_bool(382)
  }
  #[inline]
  pub fn set_verify_get_js_builtin_state(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(382, v)
  }

  /// Implies all experimental pre-staged features that can already be fuzzed but are not ready for staging yet. (experimental)
  #[inline]
  pub fn experimental_fuzzing(&self) -> bool {
    self.get_bool(383)
  }
  #[inline]
  pub fn set_experimental_fuzzing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(383, v)
  }

  /// Implies all staged features that we want to ship in the not-too-far future
  #[inline]
  pub fn future(&self) -> bool {
    self.get_bool(384)
  }
  #[inline]
  pub fn set_future(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(384, v)
  }

  /// enable all completed harmony features
  #[inline]
  pub fn harmony(&self) -> bool {
    self.get_bool(385)
  }
  #[inline]
  pub fn set_harmony(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(385, v)
  }

  /// enable all completed JavaScript features
  #[inline]
  pub fn js_staging(&self) -> bool {
    self.get_bool(386)
  }
  #[inline]
  pub fn set_js_staging(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(386, v)
  }

  /// enable staged wasm features
  #[inline]
  pub fn wasm_staging(&self) -> bool {
    self.get_bool(387)
  }
  #[inline]
  pub fn set_wasm_staging(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(387, v)
  }

  /// enable all shipped JavaScript features
  #[inline]
  pub fn js_shipping(&self) -> bool {
    self.get_bool(388)
  }
  #[inline]
  pub fn set_js_shipping(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(388, v)
  }

  /// enable all shipped harmony features
  #[inline]
  pub fn harmony_shipping(&self) -> bool {
    self.get_bool(389)
  }
  #[inline]
  pub fn set_harmony_shipping(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(389, v)
  }

  /// enable harmony ShadowRealm for JS (experimental)
  #[inline]
  pub fn harmony_shadow_realm(&self) -> bool {
    self.get_bool(390)
  }
  #[inline]
  pub fn set_harmony_shadow_realm(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(390, v)
  }

  /// enable harmony structs, shared structs, and shared arrays for JS (experimental)
  #[inline]
  pub fn harmony_struct(&self) -> bool {
    self.get_bool(391)
  }
  #[inline]
  pub fn set_harmony_struct(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(391, v)
  }

  /// enable Intl BestFitMatcher for JS (experimental)
  #[inline]
  pub fn harmony_intl_best_fit_matcher(&self) -> bool {
    self.get_bool(392)
  }
  #[inline]
  pub fn set_harmony_intl_best_fit_matcher(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(392, v)
  }

  /// enable decorators for JS (experimental)
  #[inline]
  pub fn js_decorators(&self) -> bool {
    self.get_bool(393)
  }
  #[inline]
  pub fn set_js_decorators(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(393, v)
  }

  /// enable source phase imports for JS (experimental)
  #[inline]
  pub fn js_source_phase_imports(&self) -> bool {
    self.get_bool(394)
  }
  #[inline]
  pub fn set_js_source_phase_imports(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(394, v)
  }

  /// enable RegExp \\A \\z \\Z buffer boundary assertions for JS (experimental)
  #[inline]
  pub fn js_regexp_buffer_boundaries(&self) -> bool {
    self.get_bool(395)
  }
  #[inline]
  pub fn set_js_regexp_buffer_boundaries(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(395, v)
  }

  /// enable instruction tracing section for Wasm (experimental)
  #[inline]
  pub fn wasm_instruction_tracing(&self) -> bool {
    self.get_bool(396)
  }
  #[inline]
  pub fn set_wasm_instruction_tracing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(396, v)
  }

  /// enable shared-everything threads for Wasm (experimental)
  #[inline]
  pub fn wasm_shared(&self) -> bool {
    self.get_bool(397)
  }
  #[inline]
  pub fn set_wasm_shared(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(397, v)
  }

  /// enable fp16 for Wasm (experimental)
  #[inline]
  pub fn wasm_fp16(&self) -> bool {
    self.get_bool(398)
  }
  #[inline]
  pub fn set_wasm_fp16(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(398, v)
  }

  /// enable memory control for Wasm (experimental)
  #[inline]
  pub fn wasm_memory_control(&self) -> bool {
    self.get_bool(399)
  }
  #[inline]
  pub fn set_wasm_memory_control(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(399, v)
  }

  /// enable core stack switching for Wasm (experimental)
  #[inline]
  pub fn wasm_wasmfx(&self) -> bool {
    self.get_bool(400)
  }
  #[inline]
  pub fn set_wasm_wasmfx(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(400, v)
  }

  /// enable compilation hints for Wasm (experimental)
  #[inline]
  pub fn wasm_compilation_hints(&self) -> bool {
    self.get_bool(401)
  }
  #[inline]
  pub fn set_wasm_compilation_hints(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(401, v)
  }

  /// enable growable stacks for jspi for Wasm (experimental)
  #[inline]
  pub fn wasm_growable_stacks(&self) -> bool {
    self.get_bool(402)
  }
  #[inline]
  pub fn set_wasm_growable_stacks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(402, v)
  }

  /// enable always specialize for script contexts in optimized code (experimental)
  #[inline]
  pub fn always_specialize_for_script_context(&self) -> bool {
    self.get_bool(403)
  }
  #[inline]
  pub fn set_always_specialize_for_script_context(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(403, v)
  }

  /// enable reference-typed strings for Wasm (experimental)
  #[inline]
  pub fn wasm_stringref(&self) -> bool {
    self.get_bool(404)
  }
  #[inline]
  pub fn set_wasm_stringref(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(404, v)
  }

  /// enable imported strings (utf8 features) for Wasm (experimental)
  #[inline]
  pub fn wasm_imported_strings_utf8(&self) -> bool {
    self.get_bool(405)
  }
  #[inline]
  pub fn set_wasm_imported_strings_utf8(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(405, v)
  }

  /// enable acquire_release memory ordering for Wasm (experimental)
  #[inline]
  pub fn wasm_acquire_release(&self) -> bool {
    self.get_bool(406)
  }
  #[inline]
  pub fn set_wasm_acquire_release(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(406, v)
  }

  /// enable compact import section for Wasm (experimental)
  #[inline]
  pub fn wasm_compact_imports(&self) -> bool {
    self.get_bool(407)
  }
  #[inline]
  pub fn set_wasm_compact_imports(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(407, v)
  }

  /// enable Immutable ArrayBuffer for JS
  #[inline]
  pub fn js_immutable_arraybuffer(&self) -> bool {
    self.get_bool(408)
  }
  #[inline]
  pub fn set_js_immutable_arraybuffer(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(408, v)
  }

  /// enable import bytes for JS
  #[inline]
  pub fn js_import_bytes(&self) -> bool {
    self.get_bool(409)
  }
  #[inline]
  pub fn set_js_import_bytes(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(409, v)
  }

  /// enable defer import eval for JS
  #[inline]
  pub fn js_defer_import_eval(&self) -> bool {
    self.get_bool(410)
  }
  #[inline]
  pub fn set_js_defer_import_eval(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(410, v)
  }

  /// enable custom descriptors for Wasm
  #[inline]
  pub fn wasm_custom_descriptors(&self) -> bool {
    self.get_bool(411)
  }
  #[inline]
  pub fn set_wasm_custom_descriptors(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(411, v)
  }

  /// enable wide arithmetic for Wasm
  #[inline]
  pub fn wasm_wide_arithmetic(&self) -> bool {
    self.get_bool(412)
  }
  #[inline]
  pub fn set_wasm_wide_arithmetic(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(412, v)
  }

  /// enable harmony import attributes for JS
  #[inline]
  pub fn harmony_import_attributes(&self) -> bool {
    self.get_bool(413)
  }
  #[inline]
  pub fn set_harmony_import_attributes(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(413, v)
  }

  /// enable Temporal for JS
  #[inline]
  pub fn harmony_temporal(&self) -> bool {
    self.get_bool(414)
  }
  #[inline]
  pub fn set_harmony_temporal(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(414, v)
  }

  /// enable Support diamond-importing re-expored namespaces (https://github.com/tc39/ecma262/pull/3715) for JS
  #[inline]
  pub fn js_esm_ns_reexport(&self) -> bool {
    self.get_bool(415)
  }
  #[inline]
  pub fn set_js_esm_ns_reexport(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(415, v)
  }

  /// enable Iterator.prototype.join for JS
  #[inline]
  pub fn js_iterator_join(&self) -> bool {
    self.get_bool(416)
  }
  #[inline]
  pub fn set_js_iterator_join(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(416, v)
  }

  /// enable iterator sequencing for JS
  #[inline]
  pub fn js_iterator_sequencing(&self) -> bool {
    self.get_bool(417)
  }
  #[inline]
  pub fn set_js_iterator_sequencing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(417, v)
  }

  /// enable joint iteration for JS
  #[inline]
  pub fn js_joint_iteration(&self) -> bool {
    self.get_bool(418)
  }
  #[inline]
  pub fn set_js_joint_iteration(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(418, v)
  }

  /// enable Promise.try not wrapping the result in an extra promise in the non-throwing case (https://github.com/tc39/ecma262/pull/3883) for JS
  #[inline]
  pub fn js_pr_3883(&self) -> bool {
    self.get_bool(419)
  }
  #[inline]
  pub fn set_js_pr_3883(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(419, v)
  }

  /// enable Iterator.prototype.includes for JS
  #[inline]
  pub fn js_iterator_includes(&self) -> bool {
    self.get_bool(420)
  }
  #[inline]
  pub fn set_js_iterator_includes(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(420, v)
  }

  /// enable import text for JS
  #[inline]
  pub fn js_import_text(&self) -> bool {
    self.get_bool(421)
  }
  #[inline]
  pub fn set_js_import_text(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(421, v)
  }

  /// enable legacy exception handling opcodes for Wasm
  #[inline]
  pub fn wasm_legacy_eh(&self) -> bool {
    self.get_bool(422)
  }
  #[inline]
  pub fn set_wasm_legacy_eh(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(422, v)
  }

  /// enable Homomorphic IC state for same-handler highly polymorphic ICs
  #[inline]
  pub fn homomorphic_ic(&self) -> bool {
    self.get_bool(423)
  }
  #[inline]
  pub fn set_homomorphic_ic(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(423, v)
  }

  /// allow use of the generic js-to-wasm wrapper instead of per-signature wrappers
  #[inline]
  pub fn wasm_generic_wrapper(&self) -> bool {
    self.get_bool(424)
  }
  #[inline]
  pub fn set_wasm_generic_wrapper(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(424, v)
  }

  /// allow use of the unsafe fast API wrapper (test-only / unsafe)
  #[inline]
  pub fn wasm_unsafe_fast_api_wrapper(&self) -> bool {
    self.get_bool(425)
  }
  #[inline]
  pub fn set_wasm_unsafe_fast_api_wrapper(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(425, v)
  }

  /// maximum number of parallel compilation tasks for wasm
  #[inline]
  pub fn wasm_num_compilation_tasks(&self) -> i32 {
    self.get_i32(426)
  }
  #[inline]
  pub fn set_wasm_num_compilation_tasks(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(426, v)
  }

  /// trace wasm native heap events
  #[inline]
  pub fn trace_wasm_native_heap(&self) -> bool {
    self.get_bool(427)
  }
  #[inline]
  pub fn set_trace_wasm_native_heap(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(427, v)
  }

  /// print details of wasm off-heap memory when the memory measurement API is used (developer-only)
  #[inline]
  pub fn trace_wasm_offheap_memory(&self) -> bool {
    self.get_bool(428)
  }
  #[inline]
  pub fn set_trace_wasm_offheap_memory(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(428, v)
  }

  /// print off-heap memory consumption per module and engine when they die
  #[inline]
  pub fn print_wasm_offheap_memory_size(&self) -> bool {
    self.get_bool(429)
  }
  #[inline]
  pub fn set_print_wasm_offheap_memory_size(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(429, v)
  }

  /// trace serialization/deserialization
  #[inline]
  pub fn trace_wasm_serialization(&self) -> bool {
    self.get_bool(430)
  }
  #[inline]
  pub fn set_trace_wasm_serialization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(430, v)
  }

  /// use streaming compilation instead of async compilation for tests
  #[inline]
  pub fn wasm_test_streaming(&self) -> bool {
    self.get_bool(431)
  }
  #[inline]
  pub fn set_wasm_test_streaming(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(431, v)
  }

  /// enable the native module cache
  #[inline]
  pub fn wasm_native_module_cache(&self) -> bool {
    self.get_bool(432)
  }
  #[inline]
  pub fn set_wasm_native_module_cache(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(432, v)
  }

  /// maximum number of 64KiB memory pages per wasm memory
  #[inline]
  pub fn wasm_max_mem_pages(&self) -> u32 {
    self.get_u32(433)
  }
  #[inline]
  pub fn set_wasm_max_mem_pages(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(433, v)
  }

  /// maximum table size of a wasm instance
  #[inline]
  pub fn wasm_max_table_size(&self) -> u32 {
    self.get_u32(434)
  }
  #[inline]
  pub fn set_wasm_max_table_size(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(434, v)
  }

  /// maximum committed code space for wasm (in MB)
  #[inline]
  pub fn wasm_max_committed_code_mb(&self) -> u32 {
    self.get_u32(435)
  }
  #[inline]
  pub fn set_wasm_max_committed_code_mb(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(435, v)
  }

  /// maximum size of a single wasm code space
  #[inline]
  pub fn wasm_max_code_space_size_mb(&self) -> u32 {
    self.get_u32(436)
  }
  #[inline]
  pub fn set_wasm_max_code_space_size_mb(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(436, v)
  }

  /// enable tier up to the optimizing compiler (requires --liftoff to have an effect)
  #[inline]
  pub fn wasm_tier_up(&self) -> bool {
    self.get_bool(437)
  }
  #[inline]
  pub fn set_wasm_tier_up(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(437, v)
  }

  /// enable dynamic tier up to the optimizing compiler
  #[inline]
  pub fn wasm_dynamic_tiering(&self) -> bool {
    self.get_bool(438)
  }
  #[inline]
  pub fn set_wasm_dynamic_tiering(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(438, v)
  }

  /// run Wasm tier up jobs synchronously
  #[inline]
  pub fn wasm_sync_tier_up(&self) -> bool {
    self.get_bool(439)
  }
  #[inline]
  pub fn set_wasm_sync_tier_up(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(439, v)
  }

  /// budget for dynamic tiering (rough approximation of bytes executed
  #[inline]
  pub fn wasm_tiering_budget(&self) -> i32 {
    self.get_i32(440)
  }
  #[inline]
  pub fn set_wasm_tiering_budget(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(440, v)
  }

  /// budget for wrapper tierup (number of calls until tier-up)
  #[inline]
  pub fn wasm_wrapper_tiering_budget(&self) -> i32 {
    self.get_i32(441)
  }
  #[inline]
  pub fn set_wasm_wrapper_tiering_budget(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(441, v)
  }

  /// maximum number of wasm functions defined in a module
  #[inline]
  pub fn max_wasm_functions(&self) -> i32 {
    self.get_i32(442)
  }
  #[inline]
  pub fn set_max_wasm_functions(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(442, v)
  }

  /// the amount of wasm top tier code that triggers the next caching event
  #[inline]
  pub fn wasm_caching_threshold(&self) -> i32 {
    self.get_i32(443)
  }
  #[inline]
  pub fn set_wasm_caching_threshold(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(443, v)
  }

  /// the amount of wasm top tier code that triggers caching immediately, ignoring the --wasm-caching-timeout-ms
  #[inline]
  pub fn wasm_caching_hard_threshold(&self) -> i32 {
    self.get_i32(444)
  }
  #[inline]
  pub fn set_wasm_caching_hard_threshold(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(444, v)
  }

  /// only trigger caching if no new code was compiled within this timeout (0 to disable this logic and only use --wasm-caching-threshold)
  #[inline]
  pub fn wasm_caching_timeout_ms(&self) -> i32 {
    self.get_i32(445)
  }
  #[inline]
  pub fn set_wasm_caching_timeout_ms(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(445, v)
  }

  /// print how long it took to compile each wasm function (developer-only)
  #[inline]
  pub fn trace_wasm_compilation_times(&self) -> bool {
    self.get_bool(446)
  }
  #[inline]
  pub fn set_trace_wasm_compilation_times(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(446, v)
  }

  /// only tier-up function with this index
  #[inline]
  pub fn wasm_tier_up_filter(&self) -> i32 {
    self.get_i32(447)
  }
  #[inline]
  pub fn set_wasm_tier_up_filter(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(447, v)
  }

  /// eagerly tier-up function with this index
  #[inline]
  pub fn wasm_eager_tier_up_function(&self) -> i32 {
    self.get_i32(448)
  }
  #[inline]
  pub fn set_wasm_eager_tier_up_function(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(448, v)
  }

  /// trace decoding of wasm code
  #[inline]
  pub fn trace_wasm_decoder(&self) -> bool {
    self.get_bool(449)
  }
  #[inline]
  pub fn set_trace_wasm_decoder(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(449, v)
  }

  /// trace compiling of wasm code
  #[inline]
  pub fn trace_wasm_compiler(&self) -> bool {
    self.get_bool(450)
  }
  #[inline]
  pub fn set_trace_wasm_compiler(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(450, v)
  }

  /// trace streaming compilation of wasm code
  #[inline]
  pub fn trace_wasm_streaming(&self) -> bool {
    self.get_bool(451)
  }
  #[inline]
  pub fn set_trace_wasm_streaming(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(451, v)
  }

  /// trace wasm stack switching
  #[inline]
  pub fn trace_wasm_stack_switching(&self) -> bool {
    self.get_bool(452)
  }
  #[inline]
  pub fn set_trace_wasm_stack_switching(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(452, v)
  }

  /// Always run wasm on a secondary stack, even when it is called with a regular (non-JSPI) export
  #[inline]
  pub fn stress_wasm_stack_switching(&self) -> bool {
    self.get_bool(453)
  }
  #[inline]
  pub fn set_stress_wasm_stack_switching(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(453, v)
  }

  /// default size of stacks for wasm stack-switching (in kB)
  #[inline]
  pub fn wasm_stack_switching_stack_size(&self) -> i32 {
    self.get_i32(454)
  }
  #[inline]
  pub fn set_wasm_stack_switching_stack_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(454, v)
  }

  /// default capacity for the wasm stack pool in MB, -1 for unlimited
  #[inline]
  pub fn wasm_stack_pool_capacity_mb(&self) -> i32 {
    self.get_i32(455)
  }
  #[inline]
  pub fn set_wasm_stack_pool_capacity_mb(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(455, v)
  }

  /// enable Liftoff, the baseline compiler for WebAssembly
  #[inline]
  pub fn liftoff(&self) -> bool {
    self.get_bool(456)
  }
  #[inline]
  pub fn set_liftoff(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(456, v)
  }

  /// disallow TurboFan compilation for WebAssembly (for testing)
  #[inline]
  pub fn liftoff_only(&self) -> bool {
    self.get_bool(457)
  }
  #[inline]
  pub fn set_liftoff_only(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(457, v)
  }

  /// enables a testing opcode in wasm that is only implemented in TurboFan
  #[inline]
  pub fn enable_testing_opcode_in_wasm(&self) -> bool {
    self.get_bool(458)
  }
  #[inline]
  pub fn set_enable_testing_opcode_in_wasm(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(458, v)
  }

  /// trace Liftoff, the baseline compiler for WebAssembly
  #[inline]
  pub fn trace_liftoff(&self) -> bool {
    self.get_bool(459)
  }
  #[inline]
  pub fn set_trace_liftoff(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(459, v)
  }

  /// print all memory updates performed in wasm code (developer-only)
  #[inline]
  pub fn trace_wasm_memory(&self) -> bool {
    self.get_bool(460)
  }
  #[inline]
  pub fn set_trace_wasm_memory(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(460, v)
  }

  /// print all global variable updates performed in wasm code (developer-only)
  #[inline]
  pub fn trace_wasm_globals(&self) -> bool {
    self.get_bool(461)
  }
  #[inline]
  pub fn set_trace_wasm_globals(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(461, v)
  }

  /// bitmask of declared(!) function indices to compile with TurboFan instead of Liftoff
  #[inline]
  pub fn wasm_tier_mask_for_testing(&self) -> i32 {
    self.get_i32(462)
  }
  #[inline]
  pub fn set_wasm_tier_mask_for_testing(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(462, v)
  }

  /// bitmask of declared(!) function indices to compile for debugging, only applies if the tier is Liftoff
  #[inline]
  pub fn wasm_debug_mask_for_testing(&self) -> i32 {
    self.get_i32(463)
  }
  #[inline]
  pub fn set_wasm_debug_mask_for_testing(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(463, v)
  }

  /// experimental: dump Wasm PGO information to a local file (test-only / unsafe)
  #[inline]
  pub fn wasm_pgo_to_file(&self) -> bool {
    self.get_bool(464)
  }
  #[inline]
  pub fn set_wasm_pgo_to_file(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(464, v)
  }

  /// experimental: read and use Wasm PGO data from a local file (test-only / unsafe)
  #[inline]
  pub fn wasm_pgo_from_file(&self) -> bool {
    self.get_bool(465)
  }
  #[inline]
  pub fn set_wasm_pgo_from_file(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(465, v)
  }

  /// dump wasm module bytes
  #[inline]
  pub fn dump_wasm_module(&self) -> bool {
    self.get_bool(466)
  }
  #[inline]
  pub fn set_dump_wasm_module(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(466, v)
  }

  /// directory to dump wasm modules to
  #[inline]
  pub fn dump_wasm_module_path(&self) -> Option<&str> {
    self.get_str(467)
  }
  #[inline]
  pub fn set_dump_wasm_module_path(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(467, Some(Cow::Owned(v.to_string())))
  }

  /// Enable direct calls from wasm to fast API functions with bound call function to pass the receiver as first parameter (experimental)
  #[inline]
  pub fn wasm_fast_api(&self) -> bool {
    self.get_bool(468)
  }
  #[inline]
  pub fn set_wasm_fast_api(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(468, v)
  }

  /// Enable additional type assertions in generated code (e.g. turbofan) (experimental)
  #[inline]
  pub fn wasm_assert_types(&self) -> bool {
    self.get_bool(469)
  }
  #[inline]
  pub fn set_wasm_assert_types(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(469, v)
  }

  /// enable deopts in optimized wasm functions
  #[inline]
  pub fn wasm_deopt(&self) -> bool {
    self.get_bool(470)
  }
  #[inline]
  pub fn set_wasm_deopt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(470, v)
  }

  /// limit of wasm deopts for a single function after which no further deopt points are emitted in Turbofan
  #[inline]
  pub fn wasm_deopts_per_function_limit(&self) -> usize {
    self.get_usize(471)
  }
  #[inline]
  pub fn set_wasm_deopts_per_function_limit(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(471, v)
  }

  /// assume ref.cast always succeeds and skip the related type check (test-only / unsafe)
  #[inline]
  pub fn wasm_assume_ref_cast_succeeds(&self) -> bool {
    self.get_bool(472)
  }
  #[inline]
  pub fn set_wasm_assume_ref_cast_succeeds(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(472, v)
  }

  /// enable unsafe ref.cast_nop instruction (test-only / unsafe)
  #[inline]
  pub fn wasm_ref_cast_nop(&self) -> bool {
    self.get_bool(473)
  }
  #[inline]
  pub fn set_wasm_ref_cast_nop(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(473, v)
  }

  /// skip null checks for call.ref and array and struct operations (test-only / unsafe)
  #[inline]
  pub fn wasm_skip_null_checks(&self) -> bool {
    self.get_bool(474)
  }
  #[inline]
  pub fn set_wasm_skip_null_checks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(474, v)
  }

  /// skip array bounds checks (test-only / unsafe)
  #[inline]
  pub fn wasm_skip_bounds_checks(&self) -> bool {
    self.get_bool(475)
  }
  #[inline]
  pub fn set_wasm_skip_bounds_checks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(475, v)
  }

  /// enable JS Interop part of Custom Descriptors proposal (experimental)
  #[inline]
  pub fn wasm_js_interop(&self) -> bool {
    self.get_bool(476)
  }
  #[inline]
  pub fn set_wasm_js_interop(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(476, v)
  }

  /// Emergency off-switch for Custom Descriptors Origin Trial
  #[inline]
  pub fn wasm_custom_descriptors_permitted(&self) -> bool {
    self.get_bool(477)
  }
  #[inline]
  pub fn set_wasm_custom_descriptors_permitted(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(477, v)
  }

  /// merge Custom Descriptor into v8::internal::Map (experimental)
  #[inline]
  pub fn wasm_merged_descriptors(&self) -> bool {
    self.get_bool(478)
  }
  #[inline]
  pub fn set_wasm_merged_descriptors(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(478, v)
  }

  /// enable optimization when compiling Wasm functions with Turbofan
  #[inline]
  pub fn wasm_opt(&self) -> bool {
    self.get_bool(479)
  }
  #[inline]
  pub fn set_wasm_opt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(479, v)
  }

  /// enable bounds checks (disable for performance testing only)
  #[inline]
  pub fn wasm_bounds_checks(&self) -> bool {
    self.get_bool(480)
  }
  #[inline]
  pub fn set_wasm_bounds_checks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(480, v)
  }

  /// enable stack checks (disable for performance testing only)
  #[inline]
  pub fn wasm_stack_checks(&self) -> bool {
    self.get_bool(481)
  }
  #[inline]
  pub fn set_wasm_stack_checks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(481, v)
  }

  /// enforce explicit bounds check even if the trap handler is available
  #[inline]
  pub fn wasm_enforce_bounds_checks(&self) -> bool {
    self.get_bool(482)
  }
  #[inline]
  pub fn set_wasm_enforce_bounds_checks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(482, v)
  }

  /// assume partially out-of-bounds writes are no-ops and can use the trap handler. Note that explicitly enabling this flag (if not enabled by default) can affect the spec-compliance of V8.
  #[inline]
  pub fn wasm_partial_oob_writes_are_noops(&self) -> bool {
    self.get_bool(483)
  }
  #[inline]
  pub fn set_wasm_partial_oob_writes_are_noops(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(483, v)
  }

  /// intrinsify some Math imports into wasm
  #[inline]
  pub fn wasm_math_intrinsics(&self) -> bool {
    self.get_bool(484)
  }
  #[inline]
  pub fn set_wasm_math_intrinsics(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(484, v)
  }

  /// enable inlining of Wasm functions into Wasm functions
  #[inline]
  pub fn wasm_inlining(&self) -> bool {
    self.get_bool(485)
  }
  #[inline]
  pub fn set_wasm_inlining(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(485, v)
  }

  /// maximum graph size (in TF nodes) that allows inlining more
  #[inline]
  pub fn wasm_inlining_budget(&self) -> usize {
    self.get_usize(486)
  }
  #[inline]
  pub fn set_wasm_inlining_budget(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(486, v)
  }

  /// maximum function size (in wire bytes) that may be inlined
  #[inline]
  pub fn wasm_inlining_max_size(&self) -> usize {
    self.get_usize(487)
  }
  #[inline]
  pub fn set_wasm_inlining_max_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(487, v)
  }

  /// maximum multiple graph size (in TF nodes) in comparison to initial size
  #[inline]
  pub fn wasm_inlining_factor(&self) -> usize {
    self.get_usize(488)
  }
  #[inline]
  pub fn set_wasm_inlining_factor(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(488, v)
  }

  /// minimum graph size budget (in TF nodes) for which the wasm_inlinining_factor does not apply
  #[inline]
  pub fn wasm_inlining_min_budget(&self) -> usize {
    self.get_usize(489)
  }
  #[inline]
  pub fn set_wasm_inlining_min_budget(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(489, v)
  }

  /// Ignore call counts when considering inlining candidates. The flag is supposed to be used for fuzzing
  #[inline]
  pub fn wasm_inlining_ignore_call_counts(&self) -> bool {
    self.get_bool(490)
  }
  #[inline]
  pub fn set_wasm_inlining_ignore_call_counts(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(490, v)
  }

  /// trace wasm inlining (developer-only)
  #[inline]
  pub fn trace_wasm_inlining(&self) -> bool {
    self.get_bool(491)
  }
  #[inline]
  pub fn set_trace_wasm_inlining(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(491, v)
  }

  /// trace wasm typer (developer-only)
  #[inline]
  pub fn trace_wasm_typer(&self) -> bool {
    self.get_bool(492)
  }
  #[inline]
  pub fn set_trace_wasm_typer(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(492, v)
  }

  /// trace wasm simd shuffle (developer-only)
  #[inline]
  pub fn trace_wasm_simd_shuffle(&self) -> bool {
    self.get_bool(493)
  }
  #[inline]
  pub fn set_trace_wasm_simd_shuffle(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(493, v)
  }

  /// enable speculative inlining of Wasm indirect calls
  #[inline]
  pub fn wasm_inlining_call_indirect(&self) -> bool {
    self.get_bool(494)
  }
  #[inline]
  pub fn set_wasm_inlining_call_indirect(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(494, v)
  }

  /// enable inline code generation for small bulk memory operations
  #[inline]
  pub fn wasm_bulkmem_inlining(&self) -> bool {
    self.get_bool(495)
  }
  #[inline]
  pub fn set_wasm_bulkmem_inlining(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(495, v)
  }

  /// enable loop unrolling for wasm functions
  #[inline]
  pub fn wasm_loop_unrolling(&self) -> bool {
    self.get_bool(496)
  }
  #[inline]
  pub fn set_wasm_loop_unrolling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(496, v)
  }

  /// enable Turbofan's random rescheduling phase for wasm functions (experimental)
  #[inline]
  pub fn wasm_random_rescheduling(&self) -> bool {
    self.get_bool(497)
  }
  #[inline]
  pub fn set_wasm_random_rescheduling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(497, v)
  }

  /// enable loop peeling for wasm functions
  #[inline]
  pub fn wasm_loop_peeling(&self) -> bool {
    self.get_bool(498)
  }
  #[inline]
  pub fn set_wasm_loop_peeling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(498, v)
  }

  /// maximum size for peeling
  #[inline]
  pub fn wasm_loop_peeling_max_size(&self) -> usize {
    self.get_usize(499)
  }
  #[inline]
  pub fn set_wasm_loop_peeling_max_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(499, v)
  }

  /// trace wasm loop peeling (developer-only)
  #[inline]
  pub fn trace_wasm_loop_peeling(&self) -> bool {
    self.get_bool(500)
  }
  #[inline]
  pub fn set_trace_wasm_loop_peeling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(500, v)
  }

  /// generate a test case when running a wasm fuzzer
  #[inline]
  pub fn wasm_fuzzer_gen_test(&self) -> bool {
    self.get_bool(501)
  }
  #[inline]
  pub fn set_wasm_fuzzer_gen_test(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(501, v)
  }

  /// print WebAssembly code (developer-only)
  #[inline]
  pub fn print_wasm_code(&self) -> bool {
    self.get_bool(502)
  }
  #[inline]
  pub fn set_print_wasm_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(502, v)
  }

  /// print WebAssembly code for function at index
  #[inline]
  pub fn print_wasm_code_function_index(&self) -> i32 {
    self.get_i32(503)
  }
  #[inline]
  pub fn set_print_wasm_code_function_index(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(503, v)
  }

  /// print WebAssembly stub code (developer-only)
  #[inline]
  pub fn print_wasm_stub_code(&self) -> bool {
    self.get_bool(504)
  }
  #[inline]
  pub fn set_print_wasm_stub_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(504, v)
  }

  /// enable lazy compilation for all wasm modules
  #[inline]
  pub fn wasm_lazy_compilation(&self) -> bool {
    self.get_bool(505)
  }
  #[inline]
  pub fn set_wasm_lazy_compilation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(505, v)
  }

  /// trace lazy compilation of wasm functions
  #[inline]
  pub fn trace_wasm_lazy_compilation(&self) -> bool {
    self.get_bool(506)
  }
  #[inline]
  pub fn set_trace_wasm_lazy_compilation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(506, v)
  }

  /// enable garbage collection of wasm code
  #[inline]
  pub fn wasm_code_gc(&self) -> bool {
    self.get_bool(507)
  }
  #[inline]
  pub fn set_wasm_code_gc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(507, v)
  }

  /// trace garbage collection of wasm code (developer-only)
  #[inline]
  pub fn trace_wasm_code_gc(&self) -> bool {
    self.get_bool(508)
  }
  #[inline]
  pub fn set_trace_wasm_code_gc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(508, v)
  }

  /// stress test garbage collection of wasm code
  #[inline]
  pub fn stress_wasm_code_gc(&self) -> bool {
    self.get_bool(509)
  }
  #[inline]
  pub fn set_stress_wasm_code_gc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(509, v)
  }

  /// maximum size of the initial wasm code space reservation (in MB)
  #[inline]
  pub fn wasm_max_initial_code_space_reservation(&self) -> i32 {
    self.get_i32(510)
  }
  #[inline]
  pub fn set_wasm_max_initial_code_space_reservation(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(510, v)
  }

  /// always move non-shared bounds-checked Wasm memory on grow
  #[inline]
  pub fn stress_wasm_memory_moving(&self) -> bool {
    self.get_bool(511)
  }
  #[inline]
  pub fn set_stress_wasm_memory_moving(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(511, v)
  }

  /// enable flushing Liftoff code on memory pressure signal
  #[inline]
  pub fn flush_liftoff_code(&self) -> bool {
    self.get_bool(512)
  }
  #[inline]
  pub fn set_flush_liftoff_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(512, v)
  }

  /// stress branch hinting by generating a random hint for each branch instruction
  #[inline]
  pub fn stress_branch_hinting(&self) -> bool {
    self.get_bool(513)
  }
  #[inline]
  pub fn set_stress_branch_hinting(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(513, v)
  }

  /// maximum allowed size of wasm modules
  #[inline]
  pub fn wasm_max_module_size(&self) -> usize {
    self.get_usize(514)
  }
  #[inline]
  pub fn set_wasm_max_module_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(514, v)
  }

  /// maximum size of produced disassembly (in MB, approximate)
  #[inline]
  pub fn wasm_disassembly_max_mb(&self) -> usize {
    self.get_usize(515)
  }
  #[inline]
  pub fn set_wasm_disassembly_max_mb(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(515, v)
  }

  /// size of the thread pool used by the wasm C API, with 0 meaning the maximum number of threads
  #[inline]
  pub fn wasm_capi_thread_pool_size(&self) -> i32 {
    self.get_i32(516)
  }
  #[inline]
  pub fn set_wasm_capi_thread_pool_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(516, v)
  }

  /// trace wasm function calls (developer-only)
  #[inline]
  pub fn trace_wasm(&self) -> bool {
    self.get_bool(517)
  }
  #[inline]
  pub fn set_trace_wasm(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(517, v)
  }

  /// trace creation and collection of wasm instances
  #[inline]
  pub fn trace_wasm_instances(&self) -> bool {
    self.get_bool(518)
  }
  #[inline]
  pub fn set_trace_wasm_instances(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(518, v)
  }

  /// Use trap handling for Wasm memory64 bounds checks
  #[inline]
  pub fn wasm_memory64_trap_handling(&self) -> bool {
    self.get_bool(519)
  }
  #[inline]
  pub fn set_wasm_memory64_trap_handling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(519, v)
  }

  /// execute all Wasm code in the Wasm interpreter
  #[inline]
  pub fn wasm_jitless(&self) -> bool {
    self.get_bool(520)
  }
  #[inline]
  pub fn set_wasm_jitless(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(520, v)
  }

  #[inline]
  pub fn wasm_jitless_if_available_for_testing(&self) -> bool {
    self.get_bool(521)
  }
  #[inline]
  pub fn set_wasm_jitless_if_available_for_testing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(521, v)
  }

  /// Allow mixed legacy and new exception handling instructions in the same wasm module
  #[inline]
  pub fn wasm_allow_mixed_eh_for_testing(&self) -> bool {
    self.get_bool(522)
  }
  #[inline]
  pub fn set_wasm_allow_mixed_eh_for_testing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(522, v)
  }

  /// enable Wasm code coverage
  #[inline]
  pub fn wasm_code_coverage(&self) -> bool {
    self.get_bool(523)
  }
  #[inline]
  pub fn set_wasm_code_coverage(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(523, v)
  }

  /// trace compilation hints parsing and usage (developer-only)
  #[inline]
  pub fn trace_wasm_compilation_hints(&self) -> bool {
    self.get_bool(524)
  }
  #[inline]
  pub fn set_trace_wasm_compilation_hints(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(524, v)
  }

  /// enable emitting compilation-hints sections for Wasm to a file (test-only / unsafe)
  #[inline]
  pub fn wasm_generate_compilation_hints(&self) -> bool {
    self.get_bool(525)
  }
  #[inline]
  pub fn set_wasm_generate_compilation_hints(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(525, v)
  }

  /// enable tracing wasm compilation hints generation (test-only / unsafe)
  #[inline]
  pub fn trace_wasm_generate_compilation_hints(&self) -> bool {
    self.get_bool(526)
  }
  #[inline]
  pub fn set_trace_wasm_generate_compilation_hints(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(526, v)
  }

  /// Enables sampling allocation profiler with X as a sample interval
  #[inline]
  pub fn stress_sampling_allocation_profiler(&self) -> i32 {
    self.get_i32(527)
  }
  #[inline]
  pub fn set_stress_sampling_allocation_profiler(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(527, v)
  }

  /// Enables the lazy new space shrinking strategy
  #[inline]
  pub fn lazy_new_space_shrinking(&self) -> bool {
    self.get_bool(528)
  }
  #[inline]
  pub fn set_lazy_new_space_shrinking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(528, v)
  }

  /// min size of a semi-space (in MBytes), the new space consists of two semi-spaces
  #[inline]
  pub fn min_semi_space_size(&self) -> usize {
    self.get_usize(529)
  }
  #[inline]
  pub fn set_min_semi_space_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(529, v)
  }

  /// max size of a semi-space (in MBytes), the new space consists of two semi-spaces
  #[inline]
  pub fn max_semi_space_size(&self) -> usize {
    self.get_usize(530)
  }
  #[inline]
  pub fn set_max_semi_space_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(530, v)
  }

  /// factor by which to grow the new space
  #[inline]
  pub fn semi_space_growth_factor(&self) -> i32 {
    self.get_i32(531)
  }
  #[inline]
  pub fn set_semi_space_growth_factor(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(531, v)
  }

  /// max size of the old space (in Mbytes)
  #[inline]
  pub fn max_old_space_size(&self) -> usize {
    self.get_usize(532)
  }
  #[inline]
  pub fn set_max_old_space_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(532, v)
  }

  /// max size of the heap (in Mbytes) both max_semi_space_size and max_old_space_size take precedence. All three flags cannot be specified at the same time.
  #[inline]
  pub fn max_heap_size(&self) -> usize {
    self.get_usize(533)
  }
  #[inline]
  pub fn set_max_heap_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(533, v)
  }

  /// initial size of the heap (in Mbytes)
  #[inline]
  pub fn initial_heap_size(&self) -> usize {
    self.get_usize(534)
  }
  #[inline]
  pub fn set_initial_heap_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(534, v)
  }

  /// initial old space size (in Mbytes)
  #[inline]
  pub fn initial_old_space_size(&self) -> usize {
    self.get_usize(535)
  }
  #[inline]
  pub fn set_initial_old_space_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(535, v)
  }

  /// preconfigured old space size (in Mbytes)
  #[inline]
  pub fn preconfigured_old_space_size(&self) -> usize {
    self.get_usize(536)
  }
  #[inline]
  pub fn set_preconfigured_old_space_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(536, v)
  }

  /// always perform global GCs
  #[inline]
  pub fn gc_global(&self) -> bool {
    self.get_bool(537)
  }
  #[inline]
  pub fn set_gc_global(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(537, v)
  }

  /// Collect garbage after random(0, X) V8 allocations. It overrides gc_interval.
  #[inline]
  pub fn random_gc_interval(&self) -> i32 {
    self.get_i32(538)
  }
  #[inline]
  pub fn set_random_gc_interval(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(538, v)
  }

  /// garbage collect after <n> allocations
  #[inline]
  pub fn gc_interval(&self) -> i32 {
    self.get_i32(539)
  }
  #[inline]
  pub fn set_gc_interval(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(539, v)
  }

  /// Collect garbage after random(0, X) cppgc allocations.
  #[inline]
  pub fn cppgc_random_gc_interval(&self) -> i32 {
    self.get_i32(540)
  }
  #[inline]
  pub fn set_cppgc_random_gc_interval(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(540, v)
  }

  /// keeps maps alive for <n> old space garbage collections
  #[inline]
  pub fn retain_maps_for_n_gc(&self) -> i32 {
    self.get_i32(541)
  }
  #[inline]
  pub fn set_retain_maps_for_n_gc(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(541, v)
  }

  /// print one trace line following each garbage collection (developer-only)
  #[inline]
  pub fn trace_gc(&self) -> bool {
    self.get_bool(542)
  }
  #[inline]
  pub fn set_trace_gc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(542, v)
  }

  /// print one detailed trace line in name=value format after each garbage collection (developer-only)
  #[inline]
  pub fn trace_gc_nvp(&self) -> bool {
    self.get_bool(543)
  }
  #[inline]
  pub fn set_trace_gc_nvp(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(543, v)
  }

  /// do not print trace line after scavenger collection (developer-only)
  #[inline]
  pub fn trace_gc_ignore_scavenger(&self) -> bool {
    self.get_bool(544)
  }
  #[inline]
  pub fn set_trace_gc_ignore_scavenger(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(544, v)
  }

  /// print memory reducer behavior (developer-only)
  #[inline]
  pub fn trace_memory_reducer(&self) -> bool {
    self.get_bool(545)
  }
  #[inline]
  pub fn set_trace_memory_reducer(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(545, v)
  }

  /// print more details following each garbage collection (developer-only)
  #[inline]
  pub fn trace_gc_verbose(&self) -> bool {
    self.get_bool(546)
  }
  #[inline]
  pub fn set_trace_gc_verbose(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(546, v)
  }

  /// prints details of each freelist before and after each major garbage collection (developer-only)
  #[inline]
  pub fn trace_gc_freelists(&self) -> bool {
    self.get_bool(547)
  }
  #[inline]
  pub fn set_trace_gc_freelists(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(547, v)
  }

  /// prints details of freelists of each page before and after each major garbage collection (developer-only)
  #[inline]
  pub fn trace_gc_freelists_verbose(&self) -> bool {
    self.get_bool(548)
  }
  #[inline]
  pub fn set_trace_gc_freelists_verbose(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(548, v)
  }

  /// print layout of pages in heap before and after gc (developer-only)
  #[inline]
  pub fn trace_gc_heap_layout(&self) -> bool {
    self.get_bool(549)
  }
  #[inline]
  pub fn set_trace_gc_heap_layout(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(549, v)
  }

  /// do not print trace line before and after minor-gc
  #[inline]
  pub fn trace_gc_heap_layout_ignore_minor_gc(&self) -> bool {
    self.get_bool(550)
  }
  #[inline]
  pub fn set_trace_gc_heap_layout_ignore_minor_gc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(550, v)
  }

  /// Show statistics about the pages evacuation by the compaction (developer-only)
  #[inline]
  pub fn trace_evacuation_candidates(&self) -> bool {
    self.get_bool(551)
  }
  #[inline]
  pub fn set_trace_evacuation_candidates(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(551, v)
  }

  /// trace calls to Heap::IsAllocationPending that return true (developer-only)
  #[inline]
  pub fn trace_pending_allocations(&self) -> bool {
    self.get_bool(552)
  }
  #[inline]
  pub fn set_trace_pending_allocations(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(552, v)
  }

  /// print stack trace after <n> free-list allocations
  #[inline]
  pub fn trace_allocation_stack_interval(&self) -> i32 {
    self.get_i32(553)
  }
  #[inline]
  pub fn set_trace_allocation_stack_interval(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(553, v)
  }

  /// print duplicate objects in the heap if their size is more than given threshold
  #[inline]
  pub fn trace_duplicate_threshold_kb(&self) -> i32 {
    self.get_i32(554)
  }
  #[inline]
  pub fn set_trace_duplicate_threshold_kb(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(554, v)
  }

  /// report fragmentation for old space (developer-only)
  #[inline]
  pub fn trace_fragmentation(&self) -> bool {
    self.get_bool(555)
  }
  #[inline]
  pub fn set_trace_fragmentation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(555, v)
  }

  /// report fragmentation for old space (detailed) (developer-only)
  #[inline]
  pub fn trace_fragmentation_verbose(&self) -> bool {
    self.get_bool(556)
  }
  #[inline]
  pub fn set_trace_fragmentation_verbose(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(556, v)
  }

  /// trace fragmentation after marking (developer-only)
  #[inline]
  pub fn minor_ms_trace_fragmentation(&self) -> bool {
    self.get_bool(557)
  }
  #[inline]
  pub fn set_minor_ms_trace_fragmentation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(557, v)
  }

  /// report evacuation statistics (developer-only)
  #[inline]
  pub fn trace_evacuation(&self) -> bool {
    self.get_bool(558)
  }
  #[inline]
  pub fn set_trace_evacuation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(558, v)
  }

  /// print mutator utilization, allocation speed, gc speed (developer-only)
  #[inline]
  pub fn trace_mutator_utilization(&self) -> bool {
    self.get_bool(559)
  }
  #[inline]
  pub fn set_trace_mutator_utilization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(559, v)
  }

  /// use incremental marking
  #[inline]
  pub fn incremental_marking(&self) -> bool {
    self.get_bool(560)
  }
  #[inline]
  pub fn set_incremental_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(560, v)
  }

  /// use tasks for incremental marking
  #[inline]
  pub fn incremental_marking_task(&self) -> bool {
    self.get_bool(561)
  }
  #[inline]
  pub fn set_incremental_marking_task(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(561, v)
  }

  /// threshold for starting incremental marking via a task in percent of available space: limit - size
  #[inline]
  pub fn incremental_marking_soft_trigger(&self) -> i32 {
    self.get_i32(562)
  }
  #[inline]
  pub fn set_incremental_marking_soft_trigger(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(562, v)
  }

  /// threshold for starting incremental marking immediately in percent of available space: limit - size
  #[inline]
  pub fn incremental_marking_hard_trigger(&self) -> i32 {
    self.get_i32(563)
  }
  #[inline]
  pub fn set_incremental_marking_hard_trigger(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(563, v)
  }

  /// Use a single schedule for determining a marking schedule between JS and C++ objects.
  #[inline]
  pub fn incremental_marking_unified_schedule(&self) -> bool {
    self.get_bool(564)
  }
  #[inline]
  pub fn set_incremental_marking_unified_schedule(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(564, v)
  }

  /// Trace the unmapping (developer-only)
  #[inline]
  pub fn trace_unmapper(&self) -> bool {
    self.get_bool(565)
  }
  #[inline]
  pub fn set_trace_unmapper(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(565, v)
  }

  /// parallel scavenge
  #[inline]
  pub fn parallel_scavenge(&self) -> bool {
    self.get_bool(566)
  }
  #[inline]
  pub fn set_parallel_scavenge(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(566, v)
  }

  /// schedule minor GC tasks
  #[inline]
  pub fn minor_gc_task(&self) -> bool {
    self.get_bool(567)
  }
  #[inline]
  pub fn set_minor_gc_task(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(567, v)
  }

  /// minor GC task trigger in percent of the current heap limit
  #[inline]
  pub fn minor_gc_task_trigger(&self) -> u32 {
    self.get_u32(568)
  }
  #[inline]
  pub fn set_minor_gc_task_trigger(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(568, v)
  }

  /// schedules the minor GC task with kUserVisible priority.
  #[inline]
  pub fn minor_gc_task_with_lower_priority(&self) -> bool {
    self.get_bool(569)
  }
  #[inline]
  pub fn set_minor_gc_task_with_lower_priority(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(569, v)
  }

  /// trace parallel scavenge (developer-only)
  #[inline]
  pub fn trace_parallel_scavenge(&self) -> bool {
    self.get_bool(570)
  }
  #[inline]
  pub fn set_trace_parallel_scavenge(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(570, v)
  }

  /// run young generation garbage collections in Oilpan (experimental)
  #[inline]
  pub fn cppgc_young_generation(&self) -> bool {
    self.get_bool(571)
  }
  #[inline]
  pub fn set_cppgc_young_generation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(571, v)
  }

  /// optimize GC for battery
  #[inline]
  pub fn optimize_gc_for_battery(&self) -> bool {
    self.get_bool(572)
  }
  #[inline]
  pub fn set_optimize_gc_for_battery(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(572, v)
  }

  /// use concurrent marking
  #[inline]
  pub fn concurrent_marking(&self) -> bool {
    self.get_bool(573)
  }
  #[inline]
  pub fn set_concurrent_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(573, v)
  }

  /// max worker number of concurrent marking, 0 for NumberOfWorkerThreads
  #[inline]
  pub fn concurrent_marking_max_worker_num(&self) -> u32 {
    self.get_u32(574)
  }
  #[inline]
  pub fn set_concurrent_marking_max_worker_num(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(574, v)
  }

  /// concurrently sweep array buffers
  #[inline]
  pub fn concurrent_array_buffer_sweeping(&self) -> bool {
    self.get_bool(575)
  }
  #[inline]
  pub fn set_concurrent_array_buffer_sweeping(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(575, v)
  }

  /// start background threads that allocate memory
  #[inline]
  pub fn stress_concurrent_allocation(&self) -> bool {
    self.get_bool(576)
  }
  #[inline]
  pub fn set_stress_concurrent_allocation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(576, v)
  }

  /// use parallel marking in atomic pause
  #[inline]
  pub fn parallel_marking(&self) -> bool {
    self.get_bool(577)
  }
  #[inline]
  pub fn set_parallel_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(577, v)
  }

  /// number of fixpoint iterations it takes to switch to linear ephemeron algorithm
  #[inline]
  pub fn ephemeron_fixpoint_iterations(&self) -> i32 {
    self.get_i32(578)
  }
  #[inline]
  pub fn set_ephemeron_fixpoint_iterations(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(578, v)
  }

  /// trace concurrent marking (developer-only)
  #[inline]
  pub fn trace_concurrent_marking(&self) -> bool {
    self.get_bool(579)
  }
  #[inline]
  pub fn set_trace_concurrent_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(579, v)
  }

  /// use concurrent sweeping
  #[inline]
  pub fn concurrent_sweeping(&self) -> bool {
    self.get_bool(580)
  }
  #[inline]
  pub fn set_concurrent_sweeping(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(580, v)
  }

  /// use parallel compaction
  #[inline]
  pub fn parallel_compaction(&self) -> bool {
    self.get_bool(581)
  }
  #[inline]
  pub fn set_parallel_compaction(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(581, v)
  }

  /// use parallel pointer update during compaction
  #[inline]
  pub fn parallel_pointer_update(&self) -> bool {
    self.get_bool(582)
  }
  #[inline]
  pub fn set_parallel_pointer_update(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(582, v)
  }

  /// use parallel threads to clear weak refs in the atomic pause.
  #[inline]
  pub fn parallel_gc_clearing(&self) -> bool {
    self.get_bool(583)
  }
  #[inline]
  pub fn set_parallel_gc_clearing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(583, v)
  }

  /// trigger out-of-memory failure to avoid GC storm near heap limit
  #[inline]
  pub fn detect_ineffective_gcs_near_heap_limit(&self) -> bool {
    self.get_bool(584)
  }
  #[inline]
  pub fn set_detect_ineffective_gcs_near_heap_limit(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(584, v)
  }

  /// Enforce global size by causing an out-of-memory failure above heap limit
  #[inline]
  pub fn enforce_global_heap_limit(&self) -> bool {
    self.get_bool(585)
  }
  #[inline]
  pub fn set_enforce_global_heap_limit(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(585, v)
  }

  /// Ratio from v8 to global maximum heap size
  #[inline]
  pub fn maximum_global_heap_limit_factor(&self) -> i32 {
    self.get_i32(586)
  }
  #[inline]
  pub fn set_maximum_global_heap_limit_factor(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(586, v)
  }

  /// Threshold on heap size to trigger out-of-memory failure near heap limit.
  #[inline]
  pub fn ineffective_gc_size_threshold(&self) -> f64 {
    self.get_f64(587)
  }
  #[inline]
  pub fn set_ineffective_gc_size_threshold(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(587, v)
  }

  /// Threshold on mutator utilization to trigger out-of-memory failure near heap limit.
  #[inline]
  pub fn ineffective_gc_mutator_utilization_threshold(&self) -> f64 {
    self.get_f64(588)
  }
  #[inline]
  pub fn set_ineffective_gc_mutator_utilization_threshold(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(588, v)
  }

  /// trace progress of the incremental marking (developer-only)
  #[inline]
  pub fn trace_incremental_marking(&self) -> bool {
    self.get_bool(589)
  }
  #[inline]
  pub fn set_trace_incremental_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(589, v)
  }

  /// print clearing dependency graph in dot format (developer-only)
  #[inline]
  pub fn print_gc_clearing_dependency_graph(&self) -> bool {
    self.get_bool(590)
  }
  #[inline]
  pub fn set_print_gc_clearing_dependency_graph(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(590, v)
  }

  /// trace stress marking progress (developer-only)
  #[inline]
  pub fn trace_stress_marking(&self) -> bool {
    self.get_bool(591)
  }
  #[inline]
  pub fn set_trace_stress_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(591, v)
  }

  /// trace stress scavenge progress (developer-only)
  #[inline]
  pub fn trace_stress_scavenge(&self) -> bool {
    self.get_bool(592)
  }
  #[inline]
  pub fn set_trace_stress_scavenge(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(592, v)
  }

  /// track object counts and memory usage (developer-only)
  #[inline]
  pub fn track_gc_object_stats(&self) -> bool {
    self.get_bool(593)
  }
  #[inline]
  pub fn set_track_gc_object_stats(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(593, v)
  }

  /// trace object counts and memory usage (developer-only)
  #[inline]
  pub fn trace_gc_object_stats(&self) -> bool {
    self.get_bool(594)
  }
  #[inline]
  pub fn set_trace_gc_object_stats(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(594, v)
  }

  /// trace zone memory usage (developer-only)
  #[inline]
  pub fn trace_zone_stats(&self) -> bool {
    self.get_bool(595)
  }
  #[inline]
  pub fn set_trace_zone_stats(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(595, v)
  }

  /// report a tick only when allocated zone memory changes by this amount
  #[inline]
  pub fn zone_stats_tolerance(&self) -> usize {
    self.get_usize(596)
  }
  #[inline]
  pub fn set_zone_stats_tolerance(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(596, v)
  }

  /// trace per-type zone memory usage (developer-only)
  #[inline]
  pub fn trace_zone_type_stats(&self) -> bool {
    self.get_bool(597)
  }
  #[inline]
  pub fn set_trace_zone_type_stats(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(597, v)
  }

  /// trace backing store events
  #[inline]
  pub fn trace_backing_store(&self) -> bool {
    self.get_bool(598)
  }
  #[inline]
  pub fn set_trace_backing_store(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(598, v)
  }

  /// Used by tracing internally to enable gc statistics
  #[inline]
  pub fn gc_stats(&self) -> i32 {
    self.get_i32(599)
  }
  #[inline]
  pub fn set_gc_stats(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(599, v)
  }

  /// trace all objects on each gc (warning: large output) (developer-only)
  #[inline]
  pub fn trace_gc_object_stats_all_objects(&self) -> bool {
    self.get_bool(600)
  }
  #[inline]
  pub fn set_trace_gc_object_stats_all_objects(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(600, v)
  }

  /// track native contexts that are expected to be garbage collected
  #[inline]
  pub fn track_detached_contexts(&self) -> bool {
    self.get_bool(601)
  }
  #[inline]
  pub fn set_track_detached_contexts(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(601, v)
  }

  /// trace native contexts that are expected to be garbage collected (developer-only)
  #[inline]
  pub fn trace_detached_contexts(&self) -> bool {
    self.get_bool(602)
  }
  #[inline]
  pub fn set_trace_detached_contexts(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(602, v)
  }

  /// verify heap pointers before and after GC
  #[inline]
  pub fn verify_heap(&self) -> bool {
    self.get_bool(603)
  }
  #[inline]
  pub fn set_verify_heap(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(603, v)
  }

  /// verify skipped write barriers
  #[inline]
  pub fn verify_write_barriers(&self) -> bool {
    self.get_bool(604)
  }
  #[inline]
  pub fn set_verify_write_barriers(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(604, v)
  }

  /// Bump priority for running threads to reach safepoint
  #[inline]
  pub fn safepoint_bump_qos_class(&self) -> bool {
    self.get_bool(605)
  }
  #[inline]
  pub fn set_safepoint_bump_qos_class(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(605, v)
  }

  /// Forces finalizing marking when frozen state is notified
  #[inline]
  pub fn freeze_forces_marking_finalization(&self) -> bool {
    self.get_bool(606)
  }
  #[inline]
  pub fn set_freeze_forces_marking_finalization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(606, v)
  }

  /// enable moving of object starts
  #[inline]
  pub fn move_object_start(&self) -> bool {
    self.get_bool(607)
  }
  #[inline]
  pub fn set_move_object_start(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(607, v)
  }

  /// use memory reducer
  #[inline]
  pub fn memory_reducer(&self) -> bool {
    self.get_bool(608)
  }
  #[inline]
  pub fn set_memory_reducer(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(608, v)
  }

  /// use memory reducer for small heaps
  #[inline]
  pub fn memory_reducer_for_small_heaps(&self) -> bool {
    self.get_bool(609)
  }
  #[inline]
  pub fn set_memory_reducer_for_small_heaps(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(609, v)
  }

  /// Maximum number of memory reducer GCs scheduled
  #[inline]
  pub fn memory_reducer_gc_count(&self) -> u32 {
    self.get_u32(610)
  }
  #[inline]
  pub fn set_memory_reducer_gc_count(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(610, v)
  }

  /// Delay before memory reducer start
  #[inline]
  pub fn memory_reducer_delay_ms(&self) -> u32 {
    self.get_u32(611)
  }
  #[inline]
  pub fn set_memory_reducer_delay_ms(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(611, v)
  }

  /// Delay before memory reducer start
  #[inline]
  pub fn gc_memory_reducer_start_delay_ms(&self) -> u32 {
    self.get_u32(612)
  }
  #[inline]
  pub fn set_gc_memory_reducer_start_delay_ms(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(612, v)
  }

  /// Use the limit-based memory reducer that updates allocation limits after a delay to trigger GC
  #[inline]
  pub fn memory_reducer_limit_based(&self) -> bool {
    self.get_bool(613)
  }
  #[inline]
  pub fn set_memory_reducer_limit_based(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(613, v)
  }

  /// This is the upper bound for growing factor imposed on external memory.
  #[inline]
  pub fn external_memory_max_growing_factor(&self) -> f64 {
    self.get_f64(614)
  }
  #[inline]
  pub fn set_external_memory_max_growing_factor(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(614, v)
  }

  /// Max external memory value (in GB) checked for a reasonable size, 0 to disable the check.
  #[inline]
  pub fn external_memory_max_reasonable_size(&self) -> i32 {
    self.get_i32(615)
  }
  #[inline]
  pub fn set_external_memory_max_reasonable_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(615, v)
  }

  /// Old generation and global heap limit growth scale based on the sqrt of estimated GC cost.
  #[inline]
  pub fn sqrt_allocation_limits(&self) -> bool {
    self.get_bool(616)
  }
  #[inline]
  pub fn set_sqrt_allocation_limits(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(616, v)
  }

  /// When the sqrt heap limit is enabled, multiplier on the heap growth.
  #[inline]
  pub fn sqrt_allocation_limits_factor(&self) -> i32 {
    self.get_i32(617)
  }
  #[inline]
  pub fn set_sqrt_allocation_limits_factor(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(617, v)
  }

  /// When the sqrt heap limit is enabled, multiplier on the heap growth for HeapGrowingMode::kSlow.
  #[inline]
  pub fn sqrt_allocation_limits_slow_factor(&self) -> i32 {
    self.get_i32(618)
  }
  #[inline]
  pub fn set_sqrt_allocation_limits_slow_factor(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(618, v)
  }

  /// When the sqrt heap limit is enabled, multiplier on the heap growth for HeapGrowingMode::kMinimal.
  #[inline]
  pub fn sqrt_allocation_limits_minimal_factor(&self) -> i32 {
    self.get_i32(619)
  }
  #[inline]
  pub fn set_sqrt_allocation_limits_minimal_factor(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(619, v)
  }

  /// This is the lower bound for growing factor when using sqrt heap limit.
  #[inline]
  pub fn sqrt_allocation_limits_min_growing_factor(&self) -> f64 {
    self.get_f64(620)
  }
  #[inline]
  pub fn set_sqrt_allocation_limits_min_growing_factor(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(620, v)
  }

  /// This is the upper bound for growing factor when using sqrt heap limit.
  #[inline]
  pub fn sqrt_allocation_limits_max_growing_factor(&self) -> f64 {
    self.get_f64(621)
  }
  #[inline]
  pub fn set_sqrt_allocation_limits_max_growing_factor(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(621, v)
  }

  /// Old gen GC speed is computed directly from gc tracer counters.
  #[inline]
  pub fn gc_speed_uses_counters(&self) -> bool {
    self.get_bool(622)
  }
  #[inline]
  pub fn set_gc_speed_uses_counters(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(622, v)
  }

  /// specifies heap growing factor as (1 + heap_growing_percent/100)
  #[inline]
  pub fn heap_growing_percent(&self) -> i32 {
    self.get_i32(623)
  }
  #[inline]
  pub fn set_heap_growing_percent(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(623, v)
  }

  /// override OS page size (in KBytes)
  #[inline]
  pub fn v8_os_page_size(&self) -> i32 {
    self.get_i32(624)
  }
  #[inline]
  pub fn set_v8_os_page_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(624, v)
  }

  /// allocation buffer parking
  #[inline]
  pub fn allocation_buffer_parking(&self) -> bool {
    self.get_bool(625)
  }
  #[inline]
  pub fn set_allocation_buffer_parking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(625, v)
  }

  /// Perform compaction on full GCs based on V8's default heuristics
  #[inline]
  pub fn compact(&self) -> bool {
    self.get_bool(626)
  }
  #[inline]
  pub fn set_compact(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(626, v)
  }

  /// Perform code space compaction on full collections.
  #[inline]
  pub fn compact_code_space(&self) -> bool {
    self.get_bool(627)
  }
  #[inline]
  pub fn set_compact_code_space(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(627, v)
  }

  /// Perform compaction on every full GC
  #[inline]
  pub fn compact_on_every_full_gc(&self) -> bool {
    self.get_bool(628)
  }
  #[inline]
  pub fn set_compact_on_every_full_gc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(628, v)
  }

  /// Perform compaction on regular GCs
  #[inline]
  pub fn compaction_on_regular_gcs(&self) -> bool {
    self.get_bool(629)
  }
  #[inline]
  pub fn set_compaction_on_regular_gcs(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(629, v)
  }

  /// Perform compaction when finalizing a full GC with stack
  #[inline]
  pub fn compact_with_stack(&self) -> bool {
    self.get_bool(630)
  }
  #[inline]
  pub fn set_compact_with_stack(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(630, v)
  }

  /// Max evacuated bytes during compaction
  #[inline]
  pub fn compaction_max_evacuated_bytes_mb(&self) -> i32 {
    self.get_i32(631)
  }
  #[inline]
  pub fn set_compaction_max_evacuated_bytes_mb(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(631, v)
  }

  /// Max evacuated bytes during compaction for reduce memory GCs
  #[inline]
  pub fn compaction_max_evacuated_bytes_mb_for_reduce_memory(&self) -> i32 {
    self.get_i32(632)
  }
  #[inline]
  pub fn set_compaction_max_evacuated_bytes_mb_for_reduce_memory(
    &mut self,
    v: i32,
  ) -> crate::Result<()> {
    self.assign_i32(632, v)
  }

  /// Max evacuated bytes during compaction for GCs when ShouldOptimizeForMemoryUsage() = true
  #[inline]
  pub fn compaction_max_evacuated_bytes_mb_for_optimize_memory(&self) -> i32 {
    self.get_i32(633)
  }
  #[inline]
  pub fn set_compaction_max_evacuated_bytes_mb_for_optimize_memory(
    &mut self,
    v: i32,
  ) -> crate::Result<()> {
    self.assign_i32(633, v)
  }

  /// Target fragmentation % during compaction for reduce memory GCs
  #[inline]
  pub fn compaction_target_fragmentation_percent_for_reduce_memory(&self) -> i32 {
    self.get_i32(634)
  }
  #[inline]
  pub fn set_compaction_target_fragmentation_percent_for_reduce_memory(
    &mut self,
    v: i32,
  ) -> crate::Result<()> {
    self.assign_i32(634, v)
  }

  /// Target fragmentation % during compaction for GCs when ShouldOptimizeForMemoryUsage() = true
  #[inline]
  pub fn compaction_target_fragmentation_percent_for_optimize_memory(&self) -> i32 {
    self.get_i32(635)
  }
  #[inline]
  pub fn set_compaction_target_fragmentation_percent_for_optimize_memory(
    &mut self,
    v: i32,
  ) -> crate::Result<()> {
    self.assign_i32(635, v)
  }

  /// Shortcut Strings during GC with stack
  #[inline]
  pub fn shortcut_strings_with_stack(&self) -> bool {
    self.get_bool(636)
  }
  #[inline]
  pub fn set_shortcut_strings_with_stack(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(636, v)
  }

  /// Stress GC compaction to flush out bugs with moving objects
  #[inline]
  pub fn stress_compaction(&self) -> bool {
    self.get_bool(637)
  }
  #[inline]
  pub fn set_stress_compaction(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(637, v)
  }

  /// Support resizing of large objects
  #[inline]
  pub fn resize_large_object(&self) -> bool {
    self.get_bool(638)
  }
  #[inline]
  pub fn set_resize_large_object(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(638, v)
  }

  /// Stress GC compaction by selecting random percent of pages as evacuation candidates. Overrides stress_compaction.
  #[inline]
  pub fn stress_compaction_random(&self) -> bool {
    self.get_bool(639)
  }
  #[inline]
  pub fn set_stress_compaction_random(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(639, v)
  }

  /// Stress descriptor array trimming on every stackless GC.
  #[inline]
  pub fn stress_descriptor_array_trimming(&self) -> bool {
    self.get_bool(640)
  }
  #[inline]
  pub fn set_stress_descriptor_array_trimming(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(640, v)
  }

  /// flush of baseline code when it has not been executed recently
  #[inline]
  pub fn flush_baseline_code(&self) -> bool {
    self.get_bool(641)
  }
  #[inline]
  pub fn set_flush_baseline_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(641, v)
  }

  /// flush of bytecode when it has not been executed recently
  #[inline]
  pub fn flush_bytecode(&self) -> bool {
    self.get_bool(642)
  }
  #[inline]
  pub fn set_flush_bytecode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(642, v)
  }

  /// number of seconds before we flush code
  #[inline]
  pub fn bytecode_old_time(&self) -> i32 {
    self.get_i32(643)
  }
  #[inline]
  pub fn set_bytecode_old_time(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(643, v)
  }

  /// stress code flushing
  #[inline]
  pub fn stress_flush_code(&self) -> bool {
    self.get_bool(644)
  }
  #[inline]
  pub fn set_stress_flush_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(644, v)
  }

  /// trace bytecode flushing (developer-only)
  #[inline]
  pub fn trace_flush_code(&self) -> bool {
    self.get_bool(645)
  }
  #[inline]
  pub fn set_trace_flush_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(645, v)
  }

  /// Use a progress bar to scan large objects in increments when incremental marking is active.
  #[inline]
  pub fn use_marking_progress_bar(&self) -> bool {
    self.get_bool(646)
  }
  #[inline]
  pub fn set_use_marking_progress_bar(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(646, v)
  }

  /// how much we are allowed to overshoot the heap limit by during input mode, in MiB.
  #[inline]
  pub fn fixed_margin_for_input_handling(&self) -> i32 {
    self.get_i32(647)
  }
  #[inline]
  pub fn set_fixed_margin_for_input_handling(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(647, v)
  }

  /// whether to use input hints to optimize for responsiveness.
  #[inline]
  pub fn optimize_for_input_handling(&self) -> bool {
    self.get_bool(648)
  }
  #[inline]
  pub fn set_optimize_for_input_handling(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(648, v)
  }

  /// Use per-context worklist for marking
  #[inline]
  pub fn stress_per_context_marking_worklist(&self) -> bool {
    self.get_bool(649)
  }
  #[inline]
  pub fn set_stress_per_context_marking_worklist(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(649, v)
  }

  /// force incremental marking for small heaps and run it more often
  #[inline]
  pub fn stress_incremental_marking(&self) -> bool {
    self.get_bool(650)
  }
  #[inline]
  pub fn set_stress_incremental_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(650, v)
  }

  /// Pool memory globally before returning it to the operating system
  #[inline]
  pub fn memory_pool(&self) -> bool {
    self.get_bool(651)
  }
  #[inline]
  pub fn set_memory_pool(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(651, v)
  }

  /// Share memory on Isolate teardown using other Isolate's task runners
  #[inline]
  pub fn memory_pool_share_memory_on_teardown(&self) -> bool {
    self.get_bool(652)
  }
  #[inline]
  pub fn set_memory_pool_share_memory_on_teardown(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(652, v)
  }

  /// discard the memory pool on malloc retries
  #[inline]
  pub fn memory_pool_release_on_malloc_failures(&self) -> bool {
    self.get_bool(653)
  }
  #[inline]
  pub fn set_memory_pool_release_on_malloc_failures(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(653, v)
  }

  /// Release pooled pages after X seconds.
  #[inline]
  pub fn memory_pool_timeout(&self) -> usize {
    self.get_usize(654)
  }
  #[inline]
  pub fn set_memory_pool_timeout(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(654, v)
  }

  /// Add large pages to the page pool
  #[inline]
  pub fn large_page_pool(&self) -> bool {
    self.get_bool(655)
  }
  #[inline]
  pub fn set_large_page_pool(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(655, v)
  }

  /// Maximum size of pooled large pages in MB.
  #[inline]
  pub fn max_large_page_pool_size(&self) -> usize {
    self.get_usize(656)
  }
  #[inline]
  pub fn set_max_large_page_pool_size(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(656, v)
  }

  /// Manage zone memory in V8 instead of using malloc().
  #[inline]
  pub fn managed_zone_memory(&self) -> bool {
    self.get_bool(657)
  }
  #[inline]
  pub fn set_managed_zone_memory(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(657, v)
  }

  /// Pool zone memory for isolate independent memory (nullptr-isolate) (e.g. Wasm).
  #[inline]
  pub fn managed_zone_memory_for_isolate_independent_memory(&self) -> bool {
    self.get_bool(658)
  }
  #[inline]
  pub fn set_managed_zone_memory_for_isolate_independent_memory(
    &mut self,
    v: bool,
  ) -> crate::Result<()> {
    self.assign_bool(658, v)
  }

  /// prints number of allocations and enables analysis mode for gc fuzz testing, e.g. --stress-marking, --stress-scavenge
  #[inline]
  pub fn fuzzer_gc_analysis(&self) -> bool {
    self.get_bool(659)
  }
  #[inline]
  pub fn set_fuzzer_gc_analysis(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(659, v)
  }

  /// force marking at random points between 0 and X (inclusive) percent of the regular marking start limit
  #[inline]
  pub fn stress_marking(&self) -> u32 {
    self.get_u32(660)
  }
  #[inline]
  pub fn set_stress_marking(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(660, v)
  }

  /// force scavenge at random points between 0 and X (inclusive) percent of the new space capacity
  #[inline]
  pub fn stress_scavenge(&self) -> u32 {
    self.get_u32(661)
  }
  #[inline]
  pub fn set_stress_scavenge(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(661, v)
  }

  /// reclaim otherwise unreachable unmodified wrapper objects when possible
  #[inline]
  pub fn reclaim_unmodified_wrappers(&self) -> bool {
    self.get_bool(662)
  }
  #[inline]
  pub fn set_reclaim_unmodified_wrappers(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(662, v)
  }

  /// reclaim wrapper objects in parallel
  #[inline]
  pub fn parallel_reclaim_unmodified_wrappers(&self) -> bool {
    self.get_bool(663)
  }
  #[inline]
  pub fn set_parallel_reclaim_unmodified_wrappers(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(663, v)
  }

  /// use high priority threads for concurrent Marking
  #[inline]
  pub fn concurrent_marking_high_priority_threads(&self) -> bool {
    self.get_bool(664)
  }
  #[inline]
  pub fn set_concurrent_marking_high_priority_threads(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(664, v)
  }

  /// disables AbortJS runtime function
  #[inline]
  pub fn disable_abortjs(&self) -> bool {
    self.get_bool(665)
  }
  #[inline]
  pub fn set_disable_abortjs(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(665, v)
  }

  /// randomize virtual memory reservations by ignoring any hints passed when allocating pages
  #[inline]
  pub fn randomize_all_allocations(&self) -> bool {
    self.get_bool(666)
  }
  #[inline]
  pub fn set_randomize_all_allocations(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(666, v)
  }

  /// Test mode only flag. It allows an unit test to select evacuation candidates pages (requires --stress_compaction).
  #[inline]
  pub fn manual_evacuation_candidates_selection(&self) -> bool {
    self.get_bool(667)
  }
  #[inline]
  pub fn set_manual_evacuation_candidates_selection(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(667, v)
  }

  /// initialize free memory with 0
  #[inline]
  pub fn clear_free_memory(&self) -> bool {
    self.get_bool(668)
  }
  #[inline]
  pub fn set_clear_free_memory(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(668, v)
  }

  /// idle gc on context disposal
  #[inline]
  pub fn idle_gc_on_context_disposal(&self) -> bool {
    self.get_bool(669)
  }
  #[inline]
  pub fn set_idle_gc_on_context_disposal(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(669, v)
  }

  /// trace context disposal (developer-only)
  #[inline]
  pub fn trace_context_disposal(&self) -> bool {
    self.get_bool(670)
  }
  #[inline]
  pub fn set_trace_context_disposal(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(670, v)
  }

  /// use incremental marking for CppHeap
  #[inline]
  pub fn cppheap_incremental_marking(&self) -> bool {
    self.get_bool(671)
  }
  #[inline]
  pub fn set_cppheap_incremental_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(671, v)
  }

  /// use concurrent marking for CppHeap
  #[inline]
  pub fn cppheap_concurrent_marking(&self) -> bool {
    self.get_bool(672)
  }
  #[inline]
  pub fn set_cppheap_concurrent_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(672, v)
  }

  /// use membalancer, a new heap limit balancing algorithm
  #[inline]
  pub fn memory_balancer(&self) -> bool {
    self.get_bool(673)
  }
  #[inline]
  pub fn set_memory_balancer(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(673, v)
  }

  /// c value for membalancer. A special constant to balance between memory and space tradeoff. The smaller the more memory it uses.
  #[inline]
  pub fn memory_balancer_c_value(&self) -> f64 {
    self.get_f64(674)
  }
  #[inline]
  pub fn set_memory_balancer_c_value(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(674, v)
  }

  /// print memory balancer behavior. (developer-only)
  #[inline]
  pub fn trace_memory_balancer(&self) -> bool {
    self.get_bool(675)
  }
  #[inline]
  pub fn set_trace_memory_balancer(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(675, v)
  }

  /// skips heap limit check for early GCs on allocation failure.
  #[inline]
  pub fn late_heap_limit_check(&self) -> bool {
    self.get_bool(676)
  }
  #[inline]
  pub fn set_late_heap_limit_check(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(676, v)
  }

  #[inline]
  pub fn debug_code(&self) -> bool {
    self.get_bool(677)
  }
  #[inline]
  pub fn set_debug_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(677, v)
  }

  #[inline]
  pub fn trap_on_abort(&self) -> bool {
    self.get_bool(678)
  }
  #[inline]
  pub fn set_trap_on_abort(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(678, v)
  }

  #[inline]
  pub fn slow_debug_code(&self) -> bool {
    self.get_bool(679)
  }
  #[inline]
  pub fn set_slow_debug_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(679, v)
  }

  #[inline]
  pub fn code_comments(&self) -> bool {
    self.get_bool(680)
  }
  #[inline]
  pub fn set_code_comments(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(680, v)
  }

  /// enable use of SSE3 instructions if available
  #[inline]
  pub fn enable_sse3(&self) -> bool {
    self.get_bool(681)
  }
  #[inline]
  pub fn set_enable_sse3(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(681, v)
  }

  /// enable use of SSSE3 instructions if available
  #[inline]
  pub fn enable_ssse3(&self) -> bool {
    self.get_bool(682)
  }
  #[inline]
  pub fn set_enable_ssse3(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(682, v)
  }

  /// enable use of SSE4.1 instructions if available
  #[inline]
  pub fn enable_sse4_1(&self) -> bool {
    self.get_bool(683)
  }
  #[inline]
  pub fn set_enable_sse4_1(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(683, v)
  }

  /// enable use of SSE4.2 instructions if available
  #[inline]
  pub fn enable_sse4_2(&self) -> bool {
    self.get_bool(684)
  }
  #[inline]
  pub fn set_enable_sse4_2(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(684, v)
  }

  /// enable use of SAHF instruction if available (X64 only)
  #[inline]
  pub fn enable_sahf(&self) -> bool {
    self.get_bool(685)
  }
  #[inline]
  pub fn set_enable_sahf(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(685, v)
  }

  /// enable use of AVX instructions if available
  #[inline]
  pub fn enable_avx(&self) -> bool {
    self.get_bool(686)
  }
  #[inline]
  pub fn set_enable_avx(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(686, v)
  }

  /// enable use of AVX2 instructions if available
  #[inline]
  pub fn enable_avx2(&self) -> bool {
    self.get_bool(687)
  }
  #[inline]
  pub fn set_enable_avx2(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(687, v)
  }

  /// enable use of AVX-VNNI instructions if available
  #[inline]
  pub fn enable_avx_vnni(&self) -> bool {
    self.get_bool(688)
  }
  #[inline]
  pub fn set_enable_avx_vnni(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(688, v)
  }

  /// enable use of AVX-VNNI-INT8 instructions if available
  #[inline]
  pub fn enable_avx_vnni_int8(&self) -> bool {
    self.get_bool(689)
  }
  #[inline]
  pub fn set_enable_avx_vnni_int8(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(689, v)
  }

  /// enable use of FMA3 instructions if available
  #[inline]
  pub fn enable_fma3(&self) -> bool {
    self.get_bool(690)
  }
  #[inline]
  pub fn set_enable_fma3(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(690, v)
  }

  /// enable use of F16C instructions if available
  #[inline]
  pub fn enable_f16c(&self) -> bool {
    self.get_bool(691)
  }
  #[inline]
  pub fn set_enable_f16c(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(691, v)
  }

  /// enable use of BMI1 instructions if available
  #[inline]
  pub fn enable_bmi1(&self) -> bool {
    self.get_bool(692)
  }
  #[inline]
  pub fn set_enable_bmi1(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(692, v)
  }

  /// enable use of BMI2 instructions if available
  #[inline]
  pub fn enable_bmi2(&self) -> bool {
    self.get_bool(693)
  }
  #[inline]
  pub fn set_enable_bmi2(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(693, v)
  }

  /// enable use of LZCNT instruction if available
  #[inline]
  pub fn enable_lzcnt(&self) -> bool {
    self.get_bool(694)
  }
  #[inline]
  pub fn set_enable_lzcnt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(694, v)
  }

  /// enable use of POPCNT instruction if available
  #[inline]
  pub fn enable_popcnt(&self) -> bool {
    self.get_bool(695)
  }
  #[inline]
  pub fn set_enable_popcnt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(695, v)
  }

  /// generate instructions for the selected ARM architecture if available: armv6, armv7, armv7+sudiv or armv8
  #[inline]
  pub fn arm_arch(&self) -> Option<&str> {
    self.get_str(696)
  }
  #[inline]
  pub fn set_arm_arch(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(696, Some(Cow::Owned(v.to_string())))
  }

  /// force all emitted branches to be in long mode (MIPS/PPC only)
  #[inline]
  pub fn force_long_branches(&self) -> bool {
    self.get_bool(697)
  }
  #[inline]
  pub fn set_force_long_branches(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(697, v)
  }

  /// enable optimization for specific cpu
  #[inline]
  pub fn mcpu(&self) -> Option<&str> {
    self.get_str(698)
  }
  #[inline]
  pub fn set_mcpu(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(698, Some(Cow::Owned(v.to_string())))
  }

  /// enable use of partial constant pools (x64 only)
  #[inline]
  pub fn partial_constant_pool(&self) -> bool {
    self.get_bool(699)
  }
  #[inline]
  pub fn set_partial_constant_pool(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(699, v)
  }

  /// enable optional features on the simulator for testing: none or all
  #[inline]
  pub fn sim_arm64_optional_features(&self) -> Option<&str> {
    self.get_str(700)
  }
  #[inline]
  pub fn set_sim_arm64_optional_features(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(700, Some(Cow::Owned(v.to_string())))
  }

  /// simulate arm64 Top Byte Ignore (TBI) for data accesses
  #[inline]
  pub fn sim_arm64_tbi(&self) -> bool {
    self.get_bool(701)
  }
  #[inline]
  pub fn set_sim_arm64_tbi(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(701, v)
  }

  /// enable mitigation for Intel JCC erratum on affected CPUs
  #[inline]
  pub fn intel_jcc_erratum_mitigation(&self) -> bool {
    self.get_bool(702)
  }
  #[inline]
  pub fn set_intel_jcc_erratum_mitigation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(702, v)
  }

  /// Include source information in the binary at CSA bind locations.
  #[inline]
  pub fn enable_source_at_csa_bind(&self) -> bool {
    self.get_bool(703)
  }
  #[inline]
  pub fn set_enable_source_at_csa_bind(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(703, v)
  }

  /// deprecated (use --arm_arch instead)
  #[inline]
  pub fn enable_armv7(&self) -> Option<bool> {
    self.get_mb(704)
  }
  #[inline]
  pub fn set_enable_armv7(&mut self, v: Option<bool>) -> crate::Result<()> {
    self.assign_mb(704, v)
  }

  /// deprecated (use --arm_arch instead)
  #[inline]
  pub fn enable_vfp3(&self) -> Option<bool> {
    self.get_mb(705)
  }
  #[inline]
  pub fn set_enable_vfp3(&mut self, v: Option<bool>) -> crate::Result<()> {
    self.assign_mb(705, v)
  }

  /// deprecated (use --arm_arch instead)
  #[inline]
  pub fn enable_32dregs(&self) -> Option<bool> {
    self.get_mb(706)
  }
  #[inline]
  pub fn set_enable_32dregs(&mut self, v: Option<bool>) -> crate::Result<()> {
    self.assign_mb(706, v)
  }

  /// deprecated (use --arm_arch instead)
  #[inline]
  pub fn enable_neon(&self) -> Option<bool> {
    self.get_mb(707)
  }
  #[inline]
  pub fn set_enable_neon(&mut self, v: Option<bool>) -> crate::Result<()> {
    self.assign_mb(707, v)
  }

  /// deprecated (use --arm_arch instead)
  #[inline]
  pub fn enable_sudiv(&self) -> Option<bool> {
    self.get_mb(708)
  }
  #[inline]
  pub fn set_enable_sudiv(&mut self, v: Option<bool>) -> crate::Result<()> {
    self.assign_mb(708, v)
  }

  /// deprecated (use --arm_arch instead)
  #[inline]
  pub fn enable_armv8(&self) -> Option<bool> {
    self.get_mb(709)
  }
  #[inline]
  pub fn set_enable_armv8(&mut self, v: Option<bool>) -> crate::Result<()> {
    self.assign_mb(709, v)
  }

  /// enable unaligned accesses for the regexp engine
  #[inline]
  pub fn enable_regexp_unaligned_accesses(&self) -> bool {
    self.get_bool(710)
  }
  #[inline]
  pub fn set_enable_regexp_unaligned_accesses(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(710, v)
  }

  /// enable parsing on background
  #[inline]
  pub fn script_streaming(&self) -> bool {
    self.get_bool(711)
  }
  #[inline]
  pub fn set_script_streaming(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(711, v)
  }

  /// stress test parsing on background
  #[inline]
  pub fn stress_background_compile(&self) -> bool {
    self.get_bool(712)
  }
  #[inline]
  pub fn set_stress_background_compile(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(712, v)
  }

  /// enable deserializing code caches on background
  #[inline]
  pub fn concurrent_cache_deserialization(&self) -> bool {
    self.get_bool(713)
  }
  #[inline]
  pub fn set_concurrent_cache_deserialization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(713, v)
  }

  /// After deserializing code cache data on a background thread, merge it into an existing Script if one is found in the Isolate compilation cache
  #[inline]
  pub fn merge_background_deserialized_script_with_compilation_cache(&self) -> bool {
    self.get_bool(714)
  }
  #[inline]
  pub fn set_merge_background_deserialized_script_with_compilation_cache(
    &mut self,
    v: bool,
  ) -> crate::Result<()> {
    self.assign_bool(714, v)
  }

  /// Verify scope infos after merge
  #[inline]
  pub fn verify_code_merge(&self) -> bool {
    self.get_bool(715)
  }
  #[inline]
  pub fn set_verify_code_merge(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(715, v)
  }

  /// expose gc extension
  #[inline]
  pub fn expose_gc(&self) -> bool {
    self.get_bool(716)
  }
  #[inline]
  pub fn set_expose_gc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(716, v)
  }

  /// expose gc extension under the specified name
  #[inline]
  pub fn expose_gc_as(&self) -> Option<&str> {
    self.get_str(717)
  }
  #[inline]
  pub fn set_expose_gc_as(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(717, Some(Cow::Owned(v.to_string())))
  }

  /// expose externalize string extension
  #[inline]
  pub fn expose_externalize_string(&self) -> bool {
    self.get_bool(718)
  }
  #[inline]
  pub fn set_expose_externalize_string(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(718, v)
  }

  /// expose statistics extension
  #[inline]
  pub fn expose_statistics(&self) -> bool {
    self.get_bool(719)
  }
  #[inline]
  pub fn set_expose_statistics(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(719, v)
  }

  /// expose trigger-failure extension
  #[inline]
  pub fn expose_trigger_failure(&self) -> bool {
    self.get_bool(720)
  }
  #[inline]
  pub fn set_expose_trigger_failure(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(720, v)
  }

  /// number of stack frames to capture
  #[inline]
  pub fn stack_trace_limit(&self) -> i32 {
    self.get_i32(721)
  }
  #[inline]
  pub fn set_stack_trace_limit(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(721, v)
  }

  /// show built-in functions in stack traces
  #[inline]
  pub fn builtins_in_stack_traces(&self) -> bool {
    self.get_bool(722)
  }
  #[inline]
  pub fn set_builtins_in_stack_traces(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(722, v)
  }

  /// enable experimental frames (API/Builtins) and stack trace layout
  #[inline]
  pub fn experimental_stack_trace_frames(&self) -> bool {
    self.get_bool(723)
  }
  #[inline]
  pub fn set_experimental_stack_trace_frames(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(723, v)
  }

  /// disallow eval and friends
  #[inline]
  pub fn disallow_code_generation_from_strings(&self) -> bool {
    self.get_bool(724)
  }
  #[inline]
  pub fn set_disallow_code_generation_from_strings(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(724, v)
  }

  /// expose async_hooks object
  #[inline]
  pub fn expose_async_hooks(&self) -> bool {
    self.get_bool(725)
  }
  #[inline]
  pub fn set_expose_async_hooks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(725, v)
  }

  /// expose cputracemark extension under the specified name
  #[inline]
  pub fn expose_cputracemark_as(&self) -> Option<&str> {
    self.get_str(726)
  }
  #[inline]
  pub fn set_expose_cputracemark_as(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(726, Some(Cow::Owned(v.to_string())))
  }

  /// Notify Api callback about exceptions thrown in Api callbacks
  #[inline]
  pub fn experimental_report_exceptions_from_callbacks(&self) -> bool {
    self.get_bool(727)
  }
  #[inline]
  pub fn set_experimental_report_exceptions_from_callbacks(
    &mut self,
    v: bool,
  ) -> crate::Result<()> {
    self.assign_bool(727, v)
  }

  /// allow invoking the function constructor without security checks
  #[inline]
  pub fn allow_unsafe_function_constructor(&self) -> bool {
    self.get_bool(728)
  }
  #[inline]
  pub fn set_allow_unsafe_function_constructor(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(728, v)
  }

  /// always take the slow path for builtins
  #[inline]
  pub fn force_slow_path(&self) -> bool {
    self.get_bool(729)
  }
  #[inline]
  pub fn set_force_slow_path(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(729, v)
  }

  /// enable testing the function context size overflow path by making the maximum size smaller
  #[inline]
  pub fn test_small_max_function_context_stub_size(&self) -> bool {
    self.get_bool(730)
  }
  #[inline]
  pub fn set_test_small_max_function_context_stub_size(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(730, v)
  }

  /// enable using indexOf Api callbacks
  #[inline]
  pub fn fast_api_indexof(&self) -> bool {
    self.get_bool(731)
  }
  #[inline]
  pub fn set_fast_api_indexof(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(731, v)
  }

  /// enable fast path for IterableToList for indexed interceptors
  #[inline]
  pub fn fast_api_iterable_to_list(&self) -> bool {
    self.get_bool(732)
  }
  #[inline]
  pub fn set_fast_api_iterable_to_list(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(732, v)
  }

  /// use fast inline allocation
  #[inline]
  pub fn inline_new(&self) -> bool {
    self.get_bool(733)
  }
  #[inline]
  pub fn set_inline_new(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(733, v)
  }

  /// allow the jump table used for switch statements to span a range of integers roughly equal to this number times the number of clauses in the switch
  #[inline]
  pub fn switch_table_spread_threshold(&self) -> i32 {
    self.get_i32(734)
  }
  #[inline]
  pub fn set_switch_table_spread_threshold(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(734, v)
  }

  /// the number of Smi integer cases present in the switch statement before using the jump table optimization
  #[inline]
  pub fn switch_table_min_cases(&self) -> u32 {
    self.get_u32(735)
  }
  #[inline]
  pub fn set_switch_table_min_cases(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(735, v)
  }

  /// stress lazy compilation by simulating stack overflow during unoptimized bytecode generation with 1/n-th probability, do nothing on 0
  #[inline]
  pub fn stress_lazy_compilation(&self) -> u32 {
    self.get_u32(736)
  }
  #[inline]
  pub fn set_stress_lazy_compilation(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(736, v)
  }

  /// trace javascript function calls
  #[inline]
  pub fn trace(&self) -> bool {
    self.get_bool(737)
  }
  #[inline]
  pub fn set_trace(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(737, v)
  }

  /// use lazy compilation
  #[inline]
  pub fn lazy(&self) -> bool {
    self.get_bool(738)
  }
  #[inline]
  pub fn set_lazy(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(738, v)
  }

  /// use lazy compilation during eval
  #[inline]
  pub fn lazy_eval(&self) -> bool {
    self.get_bool(739)
  }
  #[inline]
  pub fn set_lazy_eval(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(739, v)
  }

  /// use lazy compilation during streaming compilation
  #[inline]
  pub fn lazy_streaming(&self) -> bool {
    self.get_bool(740)
  }
  #[inline]
  pub fn set_lazy_streaming(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(740, v)
  }

  /// ignore eager compilation hints
  #[inline]
  pub fn max_lazy(&self) -> bool {
    self.get_bool(741)
  }
  #[inline]
  pub fn set_max_lazy(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(741, v)
  }

  /// enable magic compile hints comments
  #[inline]
  pub fn compile_hints_magic(&self) -> bool {
    self.get_bool(742)
  }
  #[inline]
  pub fn set_compile_hints_magic(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(742, v)
  }

  /// trace optimized compilation (developer-only)
  #[inline]
  pub fn trace_opt(&self) -> bool {
    self.get_bool(743)
  }
  #[inline]
  pub fn set_trace_opt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(743, v)
  }

  /// trace the optimization status of functions during tiering events (developer-only)
  #[inline]
  pub fn trace_opt_status(&self) -> bool {
    self.get_bool(744)
  }
  #[inline]
  pub fn set_trace_opt_status(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(744, v)
  }

  /// extra verbose optimized compilation tracing (developer-only)
  #[inline]
  pub fn trace_opt_verbose(&self) -> bool {
    self.get_bool(745)
  }
  #[inline]
  pub fn set_trace_opt_verbose(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(745, v)
  }

  /// trace optimized compilation statistics (developer-only)
  #[inline]
  pub fn trace_opt_stats(&self) -> bool {
    self.get_bool(746)
  }
  #[inline]
  pub fn set_trace_opt_stats(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(746, v)
  }

  /// trace deoptimization (developer-only)
  #[inline]
  pub fn trace_deopt(&self) -> bool {
    self.get_bool(747)
  }
  #[inline]
  pub fn set_trace_deopt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(747, v)
  }

  /// log deoptimization (developer-only)
  #[inline]
  pub fn log_deopt(&self) -> bool {
    self.get_bool(748)
  }
  #[inline]
  pub fn set_log_deopt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(748, v)
  }

  /// extra verbose deoptimization tracing (developer-only)
  #[inline]
  pub fn trace_deopt_verbose(&self) -> bool {
    self.get_bool(749)
  }
  #[inline]
  pub fn set_trace_deopt_verbose(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(749, v)
  }

  /// include file names in trace-opt/trace-deopt output (developer-only)
  #[inline]
  pub fn trace_file_names(&self) -> bool {
    self.get_bool(750)
  }
  #[inline]
  pub fn set_trace_file_names(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(750, v)
  }

  /// always try to OSR functions
  #[inline]
  pub fn always_osr(&self) -> bool {
    self.get_bool(751)
  }
  #[inline]
  pub fn set_always_osr(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(751, v)
  }

  /// print code serializer trace (developer-only)
  #[inline]
  pub fn trace_serializer(&self) -> bool {
    self.get_bool(752)
  }
  #[inline]
  pub fn set_trace_serializer(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(752, v)
  }

  /// enable compilation cache
  #[inline]
  pub fn compilation_cache(&self) -> bool {
    self.get_bool(753)
  }
  #[inline]
  pub fn set_compilation_cache(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(753, v)
  }

  /// cache prototype transitions
  #[inline]
  pub fn cache_prototype_transitions(&self) -> bool {
    self.get_bool(754)
  }
  #[inline]
  pub fn set_cache_prototype_transitions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(754, v)
  }

  /// enable compiler dispatcher (experimental)
  #[inline]
  pub fn lazy_compile_dispatcher(&self) -> bool {
    self.get_bool(755)
  }
  #[inline]
  pub fn set_lazy_compile_dispatcher(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(755, v)
  }

  /// max threads for compiler dispatcher (0 for unbounded)
  #[inline]
  pub fn lazy_compile_dispatcher_max_threads(&self) -> u32 {
    self.get_u32(756)
  }
  #[inline]
  pub fn set_lazy_compile_dispatcher_max_threads(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(756, v)
  }

  /// trace compiler dispatcher activity (developer-only)
  #[inline]
  pub fn trace_compiler_dispatcher(&self) -> bool {
    self.get_bool(757)
  }
  #[inline]
  pub fn set_trace_compiler_dispatcher(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(757, v)
  }

  /// spawn parallel compile tasks for eagerly compiled, top-level functions (experimental)
  #[inline]
  pub fn parallel_compile_tasks_for_eager_toplevel(&self) -> bool {
    self.get_bool(758)
  }
  #[inline]
  pub fn set_parallel_compile_tasks_for_eager_toplevel(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(758, v)
  }

  /// spawn parallel compile tasks for all lazily compiled functions (experimental)
  #[inline]
  pub fn parallel_compile_tasks_for_lazy(&self) -> bool {
    self.get_bool(759)
  }
  #[inline]
  pub fn set_parallel_compile_tasks_for_lazy(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(759, v)
  }

  /// CPU profiler sampling interval in microseconds
  #[inline]
  pub fn cpu_profiler_sampling_interval(&self) -> u32 {
    self.get_u32(760)
  }
  #[inline]
  pub fn set_cpu_profiler_sampling_interval(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(760, v)
  }

  /// Start a debugger when a simulator hits a breakpoint
  #[inline]
  pub fn simulator_debugger(&self) -> bool {
    self.get_bool(761)
  }
  #[inline]
  pub fn set_simulator_debugger(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(761, v)
  }

  /// print debug messages for side-effect-free debug-evaluate for testing
  #[inline]
  pub fn trace_side_effect_free_debug_evaluate(&self) -> bool {
    self.get_bool(762)
  }
  #[inline]
  pub fn set_trace_side_effect_free_debug_evaluate(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(762, v)
  }

  /// abort by crashing
  #[inline]
  pub fn hard_abort(&self) -> bool {
    self.get_bool(763)
  }
  #[inline]
  pub fn set_hard_abort(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(763, v)
  }

  /// When logging, try to use coloured output.
  #[inline]
  pub fn log_colour(&self) -> bool {
    self.get_bool(764)
  }
  #[inline]
  pub fn set_log_colour(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(764, v)
  }

  /// expose injected-script-source.js for debugging
  #[inline]
  pub fn expose_inspector_scripts(&self) -> bool {
    self.get_bool(765)
  }
  #[inline]
  pub fn set_expose_inspector_scripts(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(765, v)
  }

  /// default size of stack region v8 is allowed to use (in kBytes)
  #[inline]
  pub fn stack_size(&self) -> i32 {
    self.get_i32(766)
  }
  #[inline]
  pub fn set_stack_size(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(766, v)
  }

  /// maximum length of function source code printed in a stack trace.
  #[inline]
  pub fn max_stack_trace_source_length(&self) -> i32 {
    self.get_i32(767)
  }
  #[inline]
  pub fn set_max_stack_trace_source_length(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(767, v)
  }

  /// clear exceptions when entering JavaScript
  #[inline]
  pub fn clear_exceptions_on_js_entry(&self) -> bool {
    self.get_bool(768)
  }
  #[inline]
  pub fn set_clear_exceptions_on_js_entry(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(768, v)
  }

  /// use the message with which the Error constructor was called rather than the value of the \"message\" property for Error.stack
  #[inline]
  pub fn use_original_message_for_stack_trace(&self) -> bool {
    self.get_bool(769)
  }
  #[inline]
  pub fn set_use_original_message_for_stack_trace(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(769, v)
  }

  /// time interval in ms for aggregating memory histograms
  #[inline]
  pub fn histogram_interval(&self) -> i32 {
    self.get_i32(770)
  }
  #[inline]
  pub fn set_histogram_interval(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(770, v)
  }

  /// Dump heap object allocations/movements/size_updates
  #[inline]
  pub fn heap_profiler_trace_objects(&self) -> bool {
    self.get_bool(771)
  }
  #[inline]
  pub fn set_heap_profiler_trace_objects(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(771, v)
  }

  /// Use the new EmbedderGraph API to get embedder nodes
  #[inline]
  pub fn heap_profiler_use_embedder_graph(&self) -> bool {
    self.get_bool(772)
  }
  #[inline]
  pub fn set_heap_profiler_use_embedder_graph(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(772, v)
  }

  /// Write a heap snapshot to disk on last-resort GCs
  #[inline]
  pub fn heap_snapshot_on_oom(&self) -> bool {
    self.get_bool(773)
  }
  #[inline]
  pub fn set_heap_snapshot_on_oom(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(773, v)
  }

  /// Directory to write heap snapshots to. (If not set, the snapshot is written to the current working directory.)
  #[inline]
  pub fn heap_snapshot_path(&self) -> Option<&str> {
    self.get_str(774)
  }
  #[inline]
  pub fn set_heap_snapshot_path(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(774, Some(Cow::Owned(v.to_string())))
  }

  /// Write a heap snapshot to disk on a certain GC invocation. 0 results in taking a snapshot on every invocation.
  #[inline]
  pub fn heap_snapshot_on_gc(&self) -> i32 {
    self.get_i32(775)
  }
  #[inline]
  pub fn set_heap_snapshot_on_gc(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(775, v)
  }

  /// truncate strings to this length in the heap snapshot
  #[inline]
  pub fn heap_snapshot_string_limit(&self) -> u32 {
    self.get_u32(776)
  }
  #[inline]
  pub fn set_heap_snapshot_string_limit(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(776, v)
  }

  /// dump time spent on heap snapshot
  #[inline]
  pub fn profile_heap_snapshot(&self) -> bool {
    self.get_bool(777)
  }
  #[inline]
  pub fn set_profile_heap_snapshot(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(777, v)
  }

  /// Use constant sample intervals to eliminate test flakiness
  #[inline]
  pub fn sampling_heap_profiler_suppress_randomness(&self) -> bool {
    self.get_bool(778)
  }
  #[inline]
  pub fn set_sampling_heap_profiler_suppress_randomness(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(778, v)
  }

  /// Log inline cache state transitions for tools/ic-processor
  #[inline]
  pub fn log_ic(&self) -> bool {
    self.get_bool(779)
  }
  #[inline]
  pub fn set_log_ic(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(779, v)
  }

  /// enable fast map update by caching the migration target
  #[inline]
  pub fn fast_map_update(&self) -> bool {
    self.get_bool(780)
  }
  #[inline]
  pub fn set_fast_map_update(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(780, v)
  }

  /// maximum number of valid maps to track in POLYMORPHIC state
  #[inline]
  pub fn max_valid_polymorphic_map_count(&self) -> i32 {
    self.get_i32(781)
  }
  #[inline]
  pub fn set_max_valid_polymorphic_map_count(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(781, v)
  }

  /// support sidestep transitions for dependency tracking object clone maps
  #[inline]
  pub fn clone_object_sidestep_transitions(&self) -> bool {
    self.get_bool(782)
  }
  #[inline]
  pub fn set_clone_object_sidestep_transitions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(782, v)
  }

  /// limits the number of properties that can be added to an object using keyed store before transitioning to dictionary mode
  #[inline]
  pub fn fast_properties_soft_limit(&self) -> i32 {
    self.get_i32(783)
  }
  #[inline]
  pub fn set_fast_properties_soft_limit(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(783, v)
  }

  /// generate extra code for manipulating stats counters
  #[inline]
  pub fn native_code_counters(&self) -> bool {
    self.get_bool(784)
  }
  #[inline]
  pub fn set_native_code_counters(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(784, v)
  }

  /// enable dynamic patching on JS baseline code
  #[inline]
  pub fn sparkplug_plus(&self) -> bool {
    self.get_bool(785)
  }
  #[inline]
  pub fn set_sparkplug_plus(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(785, v)
  }

  /// inline the Smi fast path of embedded feedback operations into baseline code
  #[inline]
  pub fn sparkplug_inline_smi(&self) -> bool {
    self.get_bool(786)
  }
  #[inline]
  pub fn set_sparkplug_inline_smi(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(786, v)
  }

  /// use an IC for super property loads
  #[inline]
  pub fn super_ic(&self) -> bool {
    self.get_bool(787)
  }
  #[inline]
  pub fn set_super_ic(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(787, v)
  }

  /// use MegaDOM IC state for API objects (experimental)
  #[inline]
  pub fn mega_dom_ic(&self) -> bool {
    self.get_bool(788)
  }
  #[inline]
  pub fn set_mega_dom_ic(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(788, v)
  }

  /// local cache size in homomorphic ICs
  #[inline]
  pub fn homomorphic_ic_count(&self) -> u32 {
    self.get_u32(789)
  }
  #[inline]
  pub fn set_homomorphic_ic_count(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(789, v)
  }

  /// Trace updates to prototype user tracking (developer-only)
  #[inline]
  pub fn trace_prototype_users(&self) -> bool {
    self.get_bool(790)
  }
  #[inline]
  pub fn set_trace_prototype_users(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(790, v)
  }

  /// Trace for-in enumerate slow-paths (developer-only)
  #[inline]
  pub fn trace_for_in_enumerate(&self) -> bool {
    self.get_bool(791)
  }
  #[inline]
  pub fn set_trace_for_in_enumerate(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(791, v)
  }

  /// Log map creation
  #[inline]
  pub fn log_maps(&self) -> bool {
    self.get_bool(792)
  }
  #[inline]
  pub fn set_log_maps(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(792, v)
  }

  /// Also log map details
  #[inline]
  pub fn log_maps_details(&self) -> bool {
    self.get_bool(793)
  }
  #[inline]
  pub fn set_log_maps_details(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(793, v)
  }

  /// allow natives syntax
  #[inline]
  pub fn allow_natives_syntax(&self) -> bool {
    self.get_bool(794)
  }
  #[inline]
  pub fn set_allow_natives_syntax(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(794, v)
  }

  /// allow only natives explicitly allowlisted for differential fuzzers
  #[inline]
  pub fn allow_natives_for_differential_fuzzing(&self) -> bool {
    self.get_bool(795)
  }
  #[inline]
  pub fn set_allow_natives_for_differential_fuzzing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(795, v)
  }

  /// only parse the sources
  #[inline]
  pub fn parse_only(&self) -> bool {
    self.get_bool(796)
  }
  #[inline]
  pub fn set_parse_only(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(796, v)
  }

  /// Stop execution when a pointer authentication fails in the ARM64 simulator.
  #[inline]
  pub fn sim_abort_on_bad_auth(&self) -> bool {
    self.get_bool(797)
  }
  #[inline]
  pub fn set_sim_abort_on_bad_auth(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(797, v)
  }

  /// include async stack traces in Error.stack
  #[inline]
  pub fn async_stack_traces(&self) -> bool {
    self.get_bool(798)
  }
  #[inline]
  pub fn set_async_stack_traces(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(798, v)
  }

  /// print stack trace when an illegal exception is thrown
  #[inline]
  pub fn stack_trace_on_illegal(&self) -> bool {
    self.get_bool(799)
  }
  #[inline]
  pub fn set_stack_trace_on_illegal(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(799, v)
  }

  /// abort program (dump core) when an uncaught exception is thrown
  #[inline]
  pub fn abort_on_uncaught_exception(&self) -> bool {
    self.get_bool(800)
  }
  #[inline]
  pub fn set_abort_on_uncaught_exception(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(800, v)
  }

  /// Suppress certain unspecified behaviors to ease correctness fuzzing: Abort program when the stack overflows or a string exceeds maximum length (as opposed to throwing RangeError). Use a fixed suppression string for error messages. (developer-only)
  #[inline]
  pub fn correctness_fuzzer_suppressions(&self) -> bool {
    self.get_bool(801)
  }
  #[inline]
  pub fn set_correctness_fuzzer_suppressions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(801, v)
  }

  /// Minimize cross-architecture differences for correctness fuzzing. (developer-only)
  #[inline]
  pub fn correctness_fuzzer_cross_arch_suppressions(&self) -> bool {
    self.get_bool(802)
  }
  #[inline]
  pub fn set_correctness_fuzzer_cross_arch_suppressions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(802, v)
  }

  /// rehash strings from the snapshot to override the baked-in seed
  #[inline]
  pub fn rehash_snapshot(&self) -> bool {
    self.get_bool(803)
  }
  #[inline]
  pub fn set_rehash_snapshot(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(803, v)
  }

  /// Fixed seed to use to hash property keys (0 means random)(with snapshots this option cannot override the baked-in seed)
  #[inline]
  pub fn hash_seed(&self) -> u64 {
    self.get_u64(804)
  }
  #[inline]
  pub fn set_hash_seed(&mut self, v: u64) -> crate::Result<()> {
    self.assign_u64(804, v)
  }

  /// Default seed for initializing random generator (0, the default, means to use system random).
  #[inline]
  pub fn random_seed(&self) -> i32 {
    self.get_i32(805)
  }
  #[inline]
  pub fn set_random_seed(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(805, v)
  }

  /// Default seed for initializing fuzzer random generator (0, the default, means to use v8's random number generator seed).
  #[inline]
  pub fn fuzzer_random_seed(&self) -> i32 {
    self.get_i32(806)
  }
  #[inline]
  pub fn set_fuzzer_random_seed(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(806, v)
  }

  /// trace RAIL mode (developer-only)
  #[inline]
  pub fn trace_rail(&self) -> bool {
    self.get_bool(807)
  }
  #[inline]
  pub fn set_trace_rail(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(807, v)
  }

  /// print exception object and stack trace on each thrown exception (developer-only)
  #[inline]
  pub fn print_all_exceptions(&self) -> bool {
    self.get_bool(808)
  }
  #[inline]
  pub fn set_print_all_exceptions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(808, v)
  }

  /// adjust OS specific scheduling params for the isolate
  #[inline]
  pub fn adjust_os_scheduling_parameters(&self) -> bool {
    self.get_bool(809)
  }
  #[inline]
  pub fn set_adjust_os_scheduling_parameters(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(809, v)
  }

  /// Used in an experiment to evaluate icache flushing on certain CPUs
  #[inline]
  pub fn experimental_flush_embedded_blob_icache(&self) -> bool {
    self.get_bool(810)
  }
  #[inline]
  pub fn set_experimental_flush_embedded_blob_icache(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(810, v)
  }

  /// Put embedded builtins code into the code range for shorter builtin calls/jumps if system has >=4GB memory
  #[inline]
  pub fn short_builtin_calls(&self) -> bool {
    self.get_bool(811)
  }
  #[inline]
  pub fn set_short_builtin_calls(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(811, v)
  }

  /// Trace code range allocation process. (developer-only)
  #[inline]
  pub fn trace_code_range_allocation(&self) -> bool {
    self.get_bool(812)
  }
  #[inline]
  pub fn set_trace_code_range_allocation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(812, v)
  }

  /// This mode tries harder to allocate code range near .text section. Works only for configurations with external code space and shared pointer compression cage.
  #[inline]
  pub fn better_code_range_allocation(&self) -> bool {
    self.get_bool(813)
  }
  #[inline]
  pub fn set_better_code_range_allocation(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(813, v)
  }

  /// Abort if code range is allocated further away than 4GB from the.text section
  #[inline]
  pub fn abort_on_far_code_range(&self) -> bool {
    self.get_bool(814)
  }
  #[inline]
  pub fn set_abort_on_far_code_range(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(814, v)
  }

  /// report runtime call counts and times
  #[inline]
  pub fn runtime_call_stats(&self) -> bool {
    self.get_bool(815)
  }
  #[inline]
  pub fn set_runtime_call_stats(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(815, v)
  }

  /// report runtime call counts and times
  #[inline]
  pub fn rcs(&self) -> bool {
    self.get_bool(816)
  }
  #[inline]
  pub fn set_rcs(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(816, v)
  }

  /// report runtime times in cpu time (the default is wall time)
  #[inline]
  pub fn rcs_cpu_time(&self) -> bool {
    self.get_bool(817)
  }
  #[inline]
  pub fn set_rcs_cpu_time(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(817, v)
  }

  /// Verify snapshot checksums when deserializing snapshots. Enable checksum creation and verification for code caches. Enabled by default in debug builds and once per process for Android.
  #[inline]
  pub fn verify_snapshot_checksum(&self) -> bool {
    self.get_bool(818)
  }
  #[inline]
  pub fn set_verify_snapshot_checksum(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(818, v)
  }

  /// Hash code cache source string with SHA256 instead of string length
  #[inline]
  pub fn code_cache_source_hash_sha256(&self) -> bool {
    self.get_bool(819)
  }
  #[inline]
  pub fn set_code_cache_source_hash_sha256(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(819, v)
  }

  /// Print the time it takes to deserialize the snapshot. (developer-only)
  #[inline]
  pub fn profile_deserialization(&self) -> bool {
    self.get_bool(820)
  }
  #[inline]
  pub fn set_profile_deserialization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(820, v)
  }

  /// Trace the snapshot deserialization. (developer-only)
  #[inline]
  pub fn trace_deserialization(&self) -> bool {
    self.get_bool(821)
  }
  #[inline]
  pub fn set_trace_deserialization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(821, v)
  }

  /// Collect statistics on serialized objects. (developer-only)
  #[inline]
  pub fn serialization_statistics(&self) -> bool {
    self.get_bool(822)
  }
  #[inline]
  pub fn set_serialization_statistics(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(822, v)
  }

  /// generate optimized regexp code
  #[inline]
  pub fn regexp_optimization(&self) -> bool {
    self.get_bool(823)
  }
  #[inline]
  pub fn set_regexp_optimization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(823, v)
  }

  /// optimize atomic regexp loops
  #[inline]
  pub fn regexp_atomic_loop(&self) -> bool {
    self.get_bool(824)
  }
  #[inline]
  pub fn set_regexp_atomic_loop(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(824, v)
  }

  /// unroll small {} repeats when optimizing
  #[inline]
  pub fn regexp_unroll(&self) -> bool {
    self.get_bool(825)
  }
  #[inline]
  pub fn set_regexp_unroll(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(825, v)
  }

  /// dispatch disjoint regexp alternations on the masked quick-check value
  #[inline]
  pub fn regexp_masked_dispatch(&self) -> bool {
    self.get_bool(826)
  }
  #[inline]
  pub fn set_regexp_masked_dispatch(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(826, v)
  }

  /// generate quickcheck code when optimizing
  #[inline]
  pub fn regexp_quick_check(&self) -> bool {
    self.get_bool(827)
  }
  #[inline]
  pub fn set_regexp_quick_check(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(827, v)
  }

  /// interpret all regexp code
  #[inline]
  pub fn regexp_interpret_all(&self) -> bool {
    self.get_bool(828)
  }
  #[inline]
  pub fn set_regexp_interpret_all(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(828, v)
  }

  /// enable regexp interpreter and tier up to the compiler after the number of executions set by the tier up ticks flag
  #[inline]
  pub fn regexp_tier_up(&self) -> bool {
    self.get_bool(829)
  }
  #[inline]
  pub fn set_regexp_tier_up(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(829, v)
  }

  /// set the number of executions for the regexp interpreter before tiering-up to the compiler
  #[inline]
  pub fn regexp_tier_up_ticks(&self) -> i32 {
    self.get_i32(830)
  }
  #[inline]
  pub fn set_regexp_tier_up_ticks(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(830, v)
  }

  /// compile all regexp patterns directly to native code, skipping the interpreter (equivalent to --regexp-tier-up --regexp-tier-up-ticks=0, which is also the default)
  #[inline]
  pub fn regexp_jit_all(&self) -> bool {
    self.get_bool(831)
  }
  #[inline]
  pub fn set_regexp_jit_all(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(831, v)
  }

  /// enable peephole optimization for regexp bytecode
  #[inline]
  pub fn regexp_peephole_optimization(&self) -> bool {
    self.get_bool(832)
  }
  #[inline]
  pub fn set_regexp_peephole_optimization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(832, v)
  }

  /// enable the regexp results cache
  #[inline]
  pub fn regexp_results_cache(&self) -> bool {
    self.get_bool(833)
  }
  #[inline]
  pub fn set_regexp_results_cache(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(833, v)
  }

  /// assemble regexp JIT-code from bytecode
  #[inline]
  pub fn regexp_assemble_from_bytecode(&self) -> bool {
    self.get_bool(834)
  }
  #[inline]
  pub fn set_regexp_assemble_from_bytecode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(834, v)
  }

  /// emit the SIMD scans directly in the regexp compiler, instead of via the bytecode peephole pass
  #[inline]
  pub fn regexp_simd_in_rc(&self) -> bool {
    self.get_bool(835)
  }
  #[inline]
  pub fn set_regexp_simd_in_rc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(835, v)
  }

  /// trace regexp bytecode peephole optimization (requires v8_enable_disassembler = true)
  #[inline]
  pub fn trace_regexp_peephole_optimization(&self) -> bool {
    self.get_bool(836)
  }
  #[inline]
  pub fn set_trace_regexp_peephole_optimization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(836, v)
  }

  /// trace regexp bytecode execution (requires v8_enable_disassembler = true)
  #[inline]
  pub fn trace_regexp_bytecodes(&self) -> bool {
    self.get_bool(837)
  }
  #[inline]
  pub fn set_trace_regexp_bytecodes(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(837, v)
  }

  /// trace regexp execution (requires v8_enable_regexp_diagnostics = true).
  #[inline]
  pub fn trace_regexp_exec(&self) -> bool {
    self.get_bool(838)
  }
  #[inline]
  pub fn set_trace_regexp_exec(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(838, v)
  }

  /// trace regexp execution out-of-line subject strings (requires v8_enable_regexp_diagnostics = true).
  #[inline]
  pub fn trace_regexp_exec_ool_subjects(&self) -> bool {
    self.get_bool(839)
  }
  #[inline]
  pub fn set_trace_regexp_exec_ool_subjects(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(839, v)
  }

  /// trace regexp macro assembler calls (requires v8_enable_regexp_diagnostics = true).
  #[inline]
  pub fn trace_regexp_assembler(&self) -> bool {
    self.get_bool(840)
  }
  #[inline]
  pub fn set_trace_regexp_assembler(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(840, v)
  }

  /// trace regexp compilation (requires v8_enable_regexp_diagnostics = true)
  #[inline]
  pub fn trace_regexp_compiler(&self) -> bool {
    self.get_bool(841)
  }
  #[inline]
  pub fn set_trace_regexp_compiler(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(841, v)
  }

  /// trace regexp AST to graph conversion (requires v8_enable_regexp_diagnostics = true)
  #[inline]
  pub fn trace_regexp_graph_building(&self) -> bool {
    self.get_bool(842)
  }
  #[inline]
  pub fn set_trace_regexp_graph_building(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(842, v)
  }

  /// trace regexp parsing (requires v8_enable_regexp_diagnostics = true)
  #[inline]
  pub fn trace_regexp_parser(&self) -> bool {
    self.get_bool(843)
  }
  #[inline]
  pub fn set_trace_regexp_parser(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(843, v)
  }

  /// trace the regexp graph (requires v8_enable_regexp_diagnostics = true)
  #[inline]
  pub fn trace_regexp_graph(&self) -> bool {
    self.get_bool(844)
  }
  #[inline]
  pub fn set_trace_regexp_graph(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(844, v)
  }

  /// print the regexp graph (requires v8_enable_regexp_diagnostics = true)
  #[inline]
  pub fn print_regexp_graph(&self) -> bool {
    self.get_bool(845)
  }
  #[inline]
  pub fn set_print_regexp_graph(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(845, v)
  }

  /// trace regexp tiering up execution (requires v8_enable_regexp_diagnostics = true)
  #[inline]
  pub fn trace_regexp_tier_up(&self) -> bool {
    self.get_bool(846)
  }
  #[inline]
  pub fn set_trace_regexp_tier_up(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(846, v)
  }

  /// recognize regexps with 'l' flag, run them on experimental engine (experimental)
  #[inline]
  pub fn enable_experimental_regexp_engine(&self) -> bool {
    self.get_bool(847)
  }
  #[inline]
  pub fn set_enable_experimental_regexp_engine(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(847, v)
  }

  /// run regexps with the experimental engine where possible
  #[inline]
  pub fn default_to_experimental_regexp_engine(&self) -> bool {
    self.get_bool(848)
  }
  #[inline]
  pub fn set_default_to_experimental_regexp_engine(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(848, v)
  }

  /// enable time optimizations for the experimental regexp engine
  #[inline]
  pub fn experimental_regexp_engine_capture_group_opt(&self) -> bool {
    self.get_bool(849)
  }
  #[inline]
  pub fn set_experimental_regexp_engine_capture_group_opt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(849, v)
  }

  /// maximum memory usage in MB allowed for experimental engine
  #[inline]
  pub fn experimental_regexp_engine_capture_group_opt_max_memory_usage(&self) -> u64 {
    self.get_u64(850)
  }
  #[inline]
  pub fn set_experimental_regexp_engine_capture_group_opt_max_memory_usage(
    &mut self,
    v: u64,
  ) -> crate::Result<()> {
    self.assign_u64(850, v)
  }

  /// trace execution of experimental regexp engine (developer-only)
  #[inline]
  pub fn trace_experimental_regexp_engine(&self) -> bool {
    self.get_bool(851)
  }
  #[inline]
  pub fn set_trace_experimental_regexp_engine(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(851, v)
  }

  /// fall back to a breadth-first regexp engine on excessive backtracking (experimental)
  #[inline]
  pub fn enable_experimental_regexp_engine_on_excessive_backtracks(&self) -> bool {
    self.get_bool(852)
  }
  #[inline]
  pub fn set_enable_experimental_regexp_engine_on_excessive_backtracks(
    &mut self,
    v: bool,
  ) -> crate::Result<()> {
    self.assign_bool(852, v)
  }

  /// number of backtracks during regexp execution before fall back to experimental engine if enable_experimental_regexp_engine_on_excessive_backtracks is set
  #[inline]
  pub fn regexp_backtracks_before_fallback(&self) -> u32 {
    self.get_u32(853)
  }
  #[inline]
  pub fn set_regexp_backtracks_before_fallback(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(853, v)
  }

  /// enable SIMD for regexp jit code
  #[inline]
  pub fn regexp_simd(&self) -> bool {
    self.get_bool(854)
  }
  #[inline]
  pub fn set_regexp_simd(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(854, v)
  }

  /// trace the read-only promotion pass (developer-only)
  #[inline]
  pub fn trace_read_only_promotion(&self) -> bool {
    self.get_bool(855)
  }
  #[inline]
  pub fn set_trace_read_only_promotion(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(855, v)
  }

  /// trace the read-only promotion pass (developer-only)
  #[inline]
  pub fn trace_read_only_promotion_verbose(&self) -> bool {
    self.get_bool(856)
  }
  #[inline]
  pub fn set_trace_read_only_promotion_verbose(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(856, v)
  }

  /// testing_bool_flag
  #[inline]
  pub fn testing_bool_flag(&self) -> bool {
    self.get_bool(857)
  }
  #[inline]
  pub fn set_testing_bool_flag(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(857, v)
  }

  /// testing_maybe_bool_flag
  #[inline]
  pub fn testing_maybe_bool_flag(&self) -> Option<bool> {
    self.get_mb(858)
  }
  #[inline]
  pub fn set_testing_maybe_bool_flag(&mut self, v: Option<bool>) -> crate::Result<()> {
    self.assign_mb(858, v)
  }

  /// testing_int_flag
  #[inline]
  pub fn testing_int_flag(&self) -> i32 {
    self.get_i32(859)
  }
  #[inline]
  pub fn set_testing_int_flag(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(859, v)
  }

  /// float-flag
  #[inline]
  pub fn testing_float_flag(&self) -> f64 {
    self.get_f64(860)
  }
  #[inline]
  pub fn set_testing_float_flag(&mut self, v: f64) -> crate::Result<()> {
    self.assign_f64(860, v)
  }

  /// string-flag
  #[inline]
  pub fn testing_string_flag(&self) -> Option<&str> {
    self.get_str(861)
  }
  #[inline]
  pub fn set_testing_string_flag(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(861, Some(Cow::Owned(v.to_string())))
  }

  /// Seed used for threading test randomness
  #[inline]
  pub fn testing_prng_seed(&self) -> i32 {
    self.get_i32(862)
  }
  #[inline]
  pub fn set_testing_prng_seed(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(862, v)
  }

  /// testing_bool_flag_A
  #[inline]
  pub fn testing_bool_flag_a(&self) -> bool {
    self.get_bool(863)
  }
  #[inline]
  pub fn set_testing_bool_flag_a(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(863, v)
  }

  /// testing_bool_flag_B
  #[inline]
  pub fn testing_bool_flag_b(&self) -> bool {
    self.get_bool(864)
  }
  #[inline]
  pub fn set_testing_bool_flag_b(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(864, v)
  }

  /// testing_bool_flag_C
  #[inline]
  pub fn testing_bool_flag_c(&self) -> bool {
    self.get_bool(865)
  }
  #[inline]
  pub fn set_testing_bool_flag_c(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(865, v)
  }

  /// Enable strict terminating DCHECKs to prevent accidentally keeping on executing JS after terminating V8. (experimental)
  #[inline]
  pub fn strict_termination_checks(&self) -> bool {
    self.get_bool(866)
  }
  #[inline]
  pub fn set_strict_termination_checks(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(866, v)
  }

  /// Fuzzers use this flag to signal that they are ... fuzzing. This causes intrinsics to fail silently (e.g. return undefined) on invalid usage.
  #[inline]
  pub fn fuzzing(&self) -> bool {
    self.get_bool(867)
  }
  #[inline]
  pub fn set_fuzzing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(867, v)
  }

  /// Fuzzers use this flag to turn DCHECKs into NOPs  and CHECK failures into silent exits. This is useful if we want to find memory corruption primitives with a leaked hole, where the engine is already in a weird state
  #[inline]
  pub fn hole_fuzzing(&self) -> bool {
    self.get_bool(868)
  }
  #[inline]
  pub fn set_hole_fuzzing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(868, v)
  }

  /// Enable sandbox testing mode. This exposes the memory corruption API (if available) and enables the sandbox crash filter to terminate the process (with status zero) if a crash that does not represent a sandbox violation is detected.
  #[inline]
  pub fn sandbox_testing(&self) -> bool {
    self.get_bool(869)
  }
  #[inline]
  pub fn set_sandbox_testing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(869, v)
  }

  /// Enable generated code verifier
  #[inline]
  pub fn validate_generated_code(&self) -> bool {
    self.get_bool(870)
  }
  #[inline]
  pub fn set_validate_generated_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(870, v)
  }

  /// Print the disassembled code as part of the generated code validator's output in case of violations. (developer-only)
  #[inline]
  pub fn validate_generated_code_include_code(&self) -> bool {
    self.get_bool(871)
  }
  #[inline]
  pub fn set_validate_generated_code_include_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(871, v)
  }

  /// Treat generated code validator violations as non-fatal. Can be used to find all issues rather than exiting after the first one. (developer-only)
  #[inline]
  pub fn validate_generated_code_non_fatal(&self) -> bool {
    self.get_bool(872)
  }
  #[inline]
  pub fn set_validate_generated_code_non_fatal(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(872, v)
  }

  /// Enable sandbox fuzzing mode. This exposes the memory corruption API and enables the sandbox crash filter to terminate the process (with non-zero status) if a crash that does not represent a sandbox violation is detected.
  #[inline]
  pub fn sandbox_fuzzing(&self) -> bool {
    self.get_bool(873)
  }
  #[inline]
  pub fn set_sandbox_fuzzing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(873, v)
  }

  /// Enable the trap-based sandbox fuzzing mode.
  #[inline]
  pub fn sandbox_trap_fuzzing(&self) -> bool {
    self.get_bool(874)
  }
  #[inline]
  pub fn set_sandbox_trap_fuzzing(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(874, v)
  }

  /// Perform lightweight bytecode verification to ensure basic safety properties of generated bytecode, mostly around control-flow.
  #[inline]
  pub fn verify_bytecode_light(&self) -> bool {
    self.get_bool(875)
  }
  #[inline]
  pub fn set_verify_bytecode_light(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(875, v)
  }

  /// Perform full bytecode verification to ensure generated bytecode is safe to execute when the sandbox is enabled (i.e. does not lead to out-of-sandbox memory corruption). Full verification is a strict superset of the lightweight verification.
  #[inline]
  pub fn verify_bytecode_full(&self) -> bool {
    self.get_bool(876)
  }
  #[inline]
  pub fn set_verify_bytecode_full(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(876, v)
  }

  /// Exposes the memory corruption API. Set automatically by --sandbox-testing and --sandbox-fuzzing.
  #[inline]
  pub fn expose_memory_corruption_api(&self) -> bool {
    self.get_bool(877)
  }
  #[inline]
  pub fn set_expose_memory_corruption_api(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(877, v)
  }

  /// Prohibits an insecure sandbox setup.
  #[inline]
  pub fn sandbox_prohibit_insecure_mode(&self) -> bool {
    self.get_bool(878)
  }
  #[inline]
  pub fn set_sandbox_prohibit_insecure_mode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(878, v)
  }

  /// Disallow changes to flag values after initializing V8
  #[inline]
  pub fn freeze_flags_after_init(&self) -> bool {
    self.get_bool(879)
  }
  #[inline]
  pub fn set_freeze_flags_after_init(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(879, v)
  }

  /// Intel CET is only supported on x64.
  #[inline]
  pub fn cet_compatible(&self) -> bool {
    self.get_bool(880)
  }
  #[inline]
  pub fn set_cet_compatible(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(880, v)
  }

  /// Path for the generated embedded data file. (mksnapshot only)
  #[inline]
  pub fn embedded_src(&self) -> Option<&str> {
    self.get_str(881)
  }
  #[inline]
  pub fn set_embedded_src(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(881, Some(Cow::Owned(v.to_string())))
  }

  /// Label to disambiguate symbols in embedded data file. (mksnapshot only)
  #[inline]
  pub fn embedded_variant(&self) -> Option<&str> {
    self.get_str(882)
  }
  #[inline]
  pub fn set_embedded_variant(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(882, Some(Cow::Owned(v.to_string())))
  }

  /// Write V8 startup as C++ src. (mksnapshot only)
  #[inline]
  pub fn startup_src(&self) -> Option<&str> {
    self.get_str(883)
  }
  #[inline]
  pub fn set_startup_src(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(883, Some(Cow::Owned(v.to_string())))
  }

  /// Write V8 startup blob file. (mksnapshot only)
  #[inline]
  pub fn startup_blob(&self) -> Option<&str> {
    self.get_str(884)
  }
  #[inline]
  pub fn set_startup_blob(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(884, Some(Cow::Owned(v.to_string())))
  }

  /// The mksnapshot target arch. (mksnapshot only)
  #[inline]
  pub fn target_arch(&self) -> Option<&str> {
    self.get_str(885)
  }
  #[inline]
  pub fn set_target_arch(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(885, Some(Cow::Owned(v.to_string())))
  }

  /// The mksnapshot target os. (mksnapshot only)
  #[inline]
  pub fn target_os(&self) -> Option<&str> {
    self.get_str(886)
  }
  #[inline]
  pub fn set_target_os(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(886, Some(Cow::Owned(v.to_string())))
  }

  /// Instruct mksnapshot that the target is meant to run in the simulator and it can generate simulator-specific instructions. (mksnapshot only)
  #[inline]
  pub fn target_is_simulator(&self) -> bool {
    self.get_bool(887)
  }
  #[inline]
  pub fn set_target_is_simulator(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(887, v)
  }

  /// Path of the input file containing basic information for builtins. (mksnapshot only)
  #[inline]
  pub fn turbo_profiling_input(&self) -> Option<&str> {
    self.get_str(888)
  }
  #[inline]
  pub fn set_turbo_profiling_input(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(888, Some(Cow::Owned(v.to_string())))
  }

  /// Path of the input file containing basic block counters for builtins for logging in turbolizer. (mksnapshot only)
  #[inline]
  pub fn turbo_log_builtins_count_input(&self) -> Option<&str> {
    self.get_str(889)
  }
  #[inline]
  pub fn set_turbo_log_builtins_count_input(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(889, Some(Cow::Owned(v.to_string())))
  }

  /// Whether the .text section of binary can be read
  #[inline]
  pub fn text_is_readable(&self) -> bool {
    self.get_bool(890)
  }
  #[inline]
  pub fn set_text_is_readable(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(890, v)
  }

  /// trace parallel marking for the young generation (developer-only)
  #[inline]
  pub fn trace_minor_ms_parallel_marking(&self) -> bool {
    self.get_bool(891)
  }
  #[inline]
  pub fn set_trace_minor_ms_parallel_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(891, v)
  }

  /// perform young generation mark sweep GCs (experimental)
  #[inline]
  pub fn minor_ms(&self) -> bool {
    self.get_bool(892)
  }
  #[inline]
  pub fn set_minor_ms(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(892, v)
  }

  /// perform young generation marking concurrently
  #[inline]
  pub fn concurrent_minor_ms_marking(&self) -> bool {
    self.get_bool(893)
  }
  #[inline]
  pub fn set_concurrent_minor_ms_marking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(893, v)
  }

  /// allocate non-young objects during incremental marking on separate pages
  #[inline]
  pub fn black_allocated_pages(&self) -> bool {
    self.get_bool(894)
  }
  #[inline]
  pub fn set_black_allocated_pages(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(894, v)
  }

  /// use sticky mark bits for separation of generations
  #[inline]
  pub fn sticky_mark_bits(&self) -> bool {
    self.get_bool(895)
  }
  #[inline]
  pub fn set_sticky_mark_bits(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(895, v)
  }

  /// min new space capacity in MBs for using young generation concurrent marking.
  #[inline]
  pub fn minor_ms_min_new_space_capacity_for_concurrent_marking_mb(&self) -> u32 {
    self.get_u32(896)
  }
  #[inline]
  pub fn set_minor_ms_min_new_space_capacity_for_concurrent_marking_mb(
    &mut self,
    v: u32,
  ) -> crate::Result<()> {
    self.assign_u32(896, v)
  }

  /// minor ms concurrent marking trigger in percent of the current new space capacity
  #[inline]
  pub fn minor_ms_concurrent_marking_trigger(&self) -> u32 {
    self.get_u32(897)
  }
  #[inline]
  pub fn set_minor_ms_concurrent_marking_trigger(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(897, v)
  }

  /// override for the minimum lab size in KB to be used for new space allocations with minor ms.
  #[inline]
  pub fn minor_ms_min_lab_size_kb(&self) -> usize {
    self.get_usize(898)
  }
  #[inline]
  pub fn set_minor_ms_min_lab_size_kb(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(898, v)
  }

  /// Enables weak handling of WeakRef and FinalizationRegistry in minor GCs.
  #[inline]
  pub fn handle_weak_ref_weakly_in_minor_gc(&self) -> bool {
    self.get_bool(899)
  }
  #[inline]
  pub fn set_handle_weak_ref_weakly_in_minor_gc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(899, v)
  }

  /// Print usage message, including flags, on console
  #[inline]
  pub fn help(&self) -> bool {
    self.get_bool(900)
  }
  #[inline]
  pub fn set_help(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(900, v)
  }

  /// Print all flag values of V8
  #[inline]
  pub fn print_flag_values(&self) -> bool {
    self.get_bool(901)
  }
  #[inline]
  pub fn set_print_flag_values(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(901, v)
  }

  /// Print JS and Wasm feature flags grouped by in-progress, staged, and shipped
  #[inline]
  pub fn print_feature_flags_json(&self) -> bool {
    self.get_bool(902)
  }
  #[inline]
  pub fn set_print_feature_flags_json(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(902, v)
  }

  /// Enable slow histograms with more overhead.
  #[inline]
  pub fn slow_histograms(&self) -> bool {
    self.get_bool(903)
  }
  #[inline]
  pub fn set_slow_histograms(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(903, v)
  }

  /// Use external strings for source code
  #[inline]
  pub fn use_external_strings(&self) -> bool {
    self.get_bool(904)
  }
  #[inline]
  pub fn set_use_external_strings(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(904, v)
  }

  /// Map counters to a file
  #[inline]
  pub fn map_counters(&self) -> Option<&str> {
    self.get_str(905)
  }
  #[inline]
  pub fn set_map_counters(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(905, Some(Cow::Owned(v.to_string())))
  }

  /// Use a mock ArrayBuffer allocator for testing. (test-only / unsafe)
  #[inline]
  pub fn mock_arraybuffer_allocator(&self) -> bool {
    self.get_bool(906)
  }
  #[inline]
  pub fn set_mock_arraybuffer_allocator(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(906, v)
  }

  /// Memory limit for mock ArrayBuffer allocator used to simulate OOM for testing.
  #[inline]
  pub fn mock_arraybuffer_allocator_limit(&self) -> usize {
    self.get_usize(907)
  }
  #[inline]
  pub fn set_mock_arraybuffer_allocator_limit(&mut self, v: usize) -> crate::Result<()> {
    self.assign_usize(907, v)
  }

  /// enable GDBJIT interface
  #[inline]
  pub fn gdbjit(&self) -> bool {
    self.get_bool(908)
  }
  #[inline]
  pub fn set_gdbjit(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(908, v)
  }

  /// enable GDBJIT interface for all code objects
  #[inline]
  pub fn gdbjit_full(&self) -> bool {
    self.get_bool(909)
  }
  #[inline]
  pub fn set_gdbjit_full(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(909, v)
  }

  /// dump elf objects with debug info to disk
  #[inline]
  pub fn gdbjit_dump(&self) -> bool {
    self.get_bool(910)
  }
  #[inline]
  pub fn set_gdbjit_dump(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(910, v)
  }

  /// enable DWARF line info for Torque files
  #[inline]
  pub fn torque_dwarf(&self) -> bool {
    self.get_bool(911)
  }
  #[inline]
  pub fn set_torque_dwarf(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(911, v)
  }

  /// dump only objects containing this substring
  #[inline]
  pub fn gdbjit_dump_filter(&self) -> Option<&str> {
    self.get_str(912)
  }
  #[inline]
  pub fn set_gdbjit_dump_filter(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(912, Some(Cow::Owned(v.to_string())))
  }

  /// enable asserts that are slow to execute
  #[inline]
  pub fn enable_slow_asserts(&self) -> bool {
    self.get_bool(913)
  }
  #[inline]
  pub fn set_enable_slow_asserts(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(913, v)
  }

  /// print source AST (developer-only)
  #[inline]
  pub fn print_ast(&self) -> bool {
    self.get_bool(914)
  }
  #[inline]
  pub fn set_print_ast(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(914, v)
  }

  /// print scopes (developer-only)
  #[inline]
  pub fn print_scopes(&self) -> bool {
    self.get_bool(915)
  }
  #[inline]
  pub fn set_print_scopes(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(915, v)
  }

  /// trace contexts operations (developer-only)
  #[inline]
  pub fn trace_contexts(&self) -> bool {
    self.get_bool(916)
  }
  #[inline]
  pub fn set_trace_contexts(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(916, v)
  }

  /// report code statistics after GC (developer-only)
  #[inline]
  pub fn code_stats(&self) -> bool {
    self.get_bool(917)
  }
  #[inline]
  pub fn set_code_stats(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(917, v)
  }

  /// report handles after GC (developer-only)
  #[inline]
  pub fn print_handles(&self) -> bool {
    self.get_bool(918)
  }
  #[inline]
  pub fn set_print_handles(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(918, v)
  }

  /// Check that there are not too many handles at GC
  #[inline]
  pub fn check_handle_count(&self) -> bool {
    self.get_bool(919)
  }
  #[inline]
  pub fn set_check_handle_count(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(919, v)
  }

  /// report global handles after GC (developer-only)
  #[inline]
  pub fn print_global_handles(&self) -> bool {
    self.get_bool(920)
  }
  #[inline]
  pub fn set_print_global_handles(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(920, v)
  }

  /// enable tracing in escape analysis (developer-only)
  #[inline]
  pub fn trace_turbo_escape(&self) -> bool {
    self.get_bool(921)
  }
  #[inline]
  pub fn set_trace_turbo_escape(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(921, v)
  }

  /// Trace status transitions of ECMAScript modules (developer-only)
  #[inline]
  pub fn trace_module_status(&self) -> bool {
    self.get_bool(922)
  }
  #[inline]
  pub fn set_trace_module_status(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(922, v)
  }

  /// prints when objects are turned into dictionaries. (developer-only)
  #[inline]
  pub fn trace_normalization(&self) -> bool {
    self.get_bool(923)
  }
  #[inline]
  pub fn set_trace_normalization(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(923, v)
  }

  /// trace lazy compilation (developer-only)
  #[inline]
  pub fn trace_lazy(&self) -> bool {
    self.get_bool(924)
  }
  #[inline]
  pub fn set_trace_lazy(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(924, v)
  }

  /// trace isolate state changes (developer-only)
  #[inline]
  pub fn trace_isolates(&self) -> bool {
    self.get_bool(925)
  }
  #[inline]
  pub fn set_trace_isolates(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(925, v)
  }

  /// enable possessive quantifier syntax for testing
  #[inline]
  pub fn regexp_possessive_quantifier(&self) -> bool {
    self.get_bool(926)
  }
  #[inline]
  pub fn set_regexp_possessive_quantifier(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(926, v)
  }

  /// print source location on debug break (developer-only)
  #[inline]
  pub fn print_break_location(&self) -> bool {
    self.get_bool(927)
  }
  #[inline]
  pub fn set_print_break_location(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(927, v)
  }

  /// Specify the name of the log file, use '-' for console, '+' for a temporary file.
  #[inline]
  pub fn logfile(&self) -> Option<&str> {
    self.get_str(928)
  }
  #[inline]
  pub fn set_logfile(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(928, Some(Cow::Owned(v.to_string())))
  }

  /// Separate log files for each isolate.
  #[inline]
  pub fn logfile_per_isolate(&self) -> bool {
    self.get_bool(929)
  }
  #[inline]
  pub fn set_logfile_per_isolate(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(929, v)
  }

  /// Minimal logging (no API, code, GC, suspect, or handles samples).
  #[inline]
  pub fn log(&self) -> bool {
    self.get_bool(930)
  }
  #[inline]
  pub fn set_log(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(930, v)
  }

  /// Log all events to the log file.
  #[inline]
  pub fn log_all(&self) -> bool {
    self.get_bool(931)
  }
  #[inline]
  pub fn set_log_all(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(931, v)
  }

  /// See --log-timer-events
  #[inline]
  pub fn log_internal_timer_events(&self) -> bool {
    self.get_bool(932)
  }
  #[inline]
  pub fn set_log_internal_timer_events(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(932, v)
  }

  /// Log timer events (incl. console.time* and Date.now).
  #[inline]
  pub fn log_timer_events(&self) -> bool {
    self.get_bool(933)
  }
  #[inline]
  pub fn set_log_timer_events(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(933, v)
  }

  /// Log source code.
  #[inline]
  pub fn log_source_code(&self) -> bool {
    self.get_bool(934)
  }
  #[inline]
  pub fn set_log_source_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(934, v)
  }

  /// Log detailed source information.
  #[inline]
  pub fn log_source_position(&self) -> bool {
    self.get_bool(935)
  }
  #[inline]
  pub fn set_log_source_position(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(935, v)
  }

  /// Log code events to the log file without profiling.
  #[inline]
  pub fn log_code(&self) -> bool {
    self.get_bool(936)
  }
  #[inline]
  pub fn set_log_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(936, v)
  }

  /// Log FeedbackVectors on first creation
  #[inline]
  pub fn log_feedback_vector(&self) -> bool {
    self.get_bool(937)
  }
  #[inline]
  pub fn set_log_feedback_vector(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(937, v)
  }

  /// Log all disassembled code to the log file.
  #[inline]
  pub fn log_code_disassemble(&self) -> bool {
    self.get_bool(938)
  }
  #[inline]
  pub fn set_log_code_disassemble(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(938, v)
  }

  /// Log function events (parse, compile, execute) separately.
  #[inline]
  pub fn log_function_events(&self) -> bool {
    self.get_bool(939)
  }
  #[inline]
  pub fn set_log_function_events(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(939, v)
  }

  /// Always generate detailed line information for CPU profiling.
  #[inline]
  pub fn detailed_line_info(&self) -> bool {
    self.get_bool(940)
  }
  #[inline]
  pub fn set_detailed_line_info(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(940, v)
  }

  /// Include idle samples in the profile
  #[inline]
  pub fn prof_include_idle(&self) -> bool {
    self.get_bool(941)
  }
  #[inline]
  pub fn set_prof_include_idle(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(941, v)
  }

  /// Enable the Perfetto code data source (requires the v8_use_perfetto gn arg)
  #[inline]
  pub fn perfetto_code_logger(&self) -> bool {
    self.get_bool(942)
  }
  #[inline]
  pub fn set_perfetto_code_logger(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(942, v)
  }

  /// Interval for --prof samples (in microseconds).
  #[inline]
  pub fn prof_sampling_interval(&self) -> i32 {
    self.get_i32(943)
  }
  #[inline]
  pub fn set_prof_sampling_interval(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(943, v)
  }

  /// Like --prof, but ignore generated code.
  #[inline]
  pub fn prof_cpp(&self) -> bool {
    self.get_bool(944)
  }
  #[inline]
  pub fn set_prof_cpp(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(944, v)
  }

  /// Used with --prof, turns on browser-compatible mode for profiling.
  #[inline]
  pub fn prof_browser_mode(&self) -> bool {
    self.get_bool(945)
  }
  #[inline]
  pub fn set_prof_browser_mode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(945, v)
  }

  /// Log statistical profiling information (implies --log-code).
  #[inline]
  pub fn prof(&self) -> bool {
    self.get_bool(946)
  }
  #[inline]
  pub fn set_prof(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(946, v)
  }

  /// Enable low-level linux profiler. (developer-only)
  #[inline]
  pub fn ll_prof(&self) -> bool {
    self.get_bool(947)
  }
  #[inline]
  pub fn set_ll_prof(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(947, v)
  }

  /// Enable basic support for perf profiler. (developer-only)
  #[inline]
  pub fn perf_basic_prof(&self) -> bool {
    self.get_bool(948)
  }
  #[inline]
  pub fn set_perf_basic_prof(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(948, v)
  }

  /// directory to write perf-<pid>.map symbol file to
  #[inline]
  pub fn perf_basic_prof_path(&self) -> Option<&str> {
    self.get_str(949)
  }
  #[inline]
  pub fn set_perf_basic_prof_path(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(949, Some(Cow::Owned(v.to_string())))
  }

  /// Only report function code ranges to perf (i.e. no stubs). (developer-only)
  #[inline]
  pub fn perf_basic_prof_only_functions(&self) -> bool {
    self.get_bool(950)
  }
  #[inline]
  pub fn set_perf_basic_prof_only_functions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(950, v)
  }

  /// Enable experimental annotate support for perf profiler. (developer-only)
  #[inline]
  pub fn perf_prof(&self) -> bool {
    self.get_bool(951)
  }
  #[inline]
  pub fn set_perf_prof(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(951, v)
  }

  /// directory to write jit-<pid>.dump symbol file to
  #[inline]
  pub fn perf_prof_path(&self) -> Option<&str> {
    self.get_str(952)
  }
  #[inline]
  pub fn set_perf_prof_path(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(952, Some(Cow::Owned(v.to_string())))
  }

  /// Used with --perf-prof, load wasm source map and provide annotate support (experimental). (developer-only)
  #[inline]
  pub fn perf_prof_annotate_wasm(&self) -> bool {
    self.get_bool(953)
  }
  #[inline]
  pub fn set_perf_prof_annotate_wasm(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(953, v)
  }

  /// Remove the perf file right after creating it (for testing only). (developer-only)
  #[inline]
  pub fn perf_prof_delete_file(&self) -> bool {
    self.get_bool(954)
  }
  #[inline]
  pub fn set_perf_prof_delete_file(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(954, v)
  }

  /// Enable unwinding info for perf linux profiler (experimental). (developer-only)
  #[inline]
  pub fn perf_prof_unwinding_info(&self) -> bool {
    self.get_bool(955)
  }
  #[inline]
  pub fn set_perf_prof_unwinding_info(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(955, v)
  }

  /// Specify the name of the file for fake gc mmap used in ll_prof
  #[inline]
  pub fn gc_fake_mmap(&self) -> Option<&str> {
    self.get_str(956)
  }
  #[inline]
  pub fn set_gc_fake_mmap(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(956, Some(Cow::Owned(v.to_string())))
  }

  /// output deopt information and disassembly into file code-<pid>-<isolate id>.asm
  #[inline]
  pub fn redirect_code_traces(&self) -> bool {
    self.get_bool(957)
  }
  #[inline]
  pub fn set_redirect_code_traces(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(957, v)
  }

  /// output deopt information and disassembly into the given file
  #[inline]
  pub fn redirect_code_traces_to(&self) -> Option<&str> {
    self.get_str(958)
  }
  #[inline]
  pub fn set_redirect_code_traces_to(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(958, Some(Cow::Owned(v.to_string())))
  }

  /// print source code of optimized and inlined functions (developer-only)
  #[inline]
  pub fn print_opt_source(&self) -> bool {
    self.get_bool(959)
  }
  #[inline]
  pub fn set_print_opt_source(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(959, v)
  }

  /// Used when v8_enable_vtunejit is enabled, load wasm source map and provide annotate support (experimental).
  #[inline]
  pub fn vtune_prof_annotate_wasm(&self) -> bool {
    self.get_bool(960)
  }
  #[inline]
  pub fn set_vtune_prof_annotate_wasm(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(960, v)
  }

  /// Enable unwinding info for Windows/x64
  #[inline]
  pub fn win64_unwinding_info(&self) -> bool {
    self.get_bool(961)
  }
  #[inline]
  pub fn set_win64_unwinding_info(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(961, v)
  }

  /// Show interpreted frames on the native stack (useful for external profilers).
  #[inline]
  pub fn interpreted_frames_native_stack(&self) -> bool {
    self.get_bool(962)
  }
  #[inline]
  pub fn set_interpreted_frames_native_stack(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(962, v)
  }

  /// Enable etw stack walking for windows
  #[inline]
  pub fn enable_etw_stack_walking(&self) -> bool {
    self.get_bool(963)
  }
  #[inline]
  pub fn set_enable_etw_stack_walking(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(963, v)
  }

  /// Enable etw debug logging (only on debug builds)
  #[inline]
  pub fn etw_trace_debug(&self) -> bool {
    self.get_bool(964)
  }
  #[inline]
  pub fn set_etw_trace_debug(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(964, v)
  }

  /// Enable etw stack walking for windows, but only if explicitly specified in a WPR profile
  #[inline]
  pub fn enable_etw_by_custom_filter_only(&self) -> bool {
    self.get_bool(965)
  }
  #[inline]
  pub fn set_enable_etw_by_custom_filter_only(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(965, v)
  }

  /// print code size for builtins
  #[inline]
  pub fn print_builtin_size(&self) -> bool {
    self.get_bool(966)
  }
  #[inline]
  pub fn set_print_builtin_size(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(966, v)
  }

  /// trace elements transitions (developer-only)
  #[inline]
  pub fn trace_elements_transitions(&self) -> bool {
    self.get_bool(967)
  }
  #[inline]
  pub fn set_trace_elements_transitions(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(967, v)
  }

  /// trace the creation of allocation sites (developer-only)
  #[inline]
  pub fn trace_creation_allocation_sites(&self) -> bool {
    self.get_bool(968)
  }
  #[inline]
  pub fn set_trace_creation_allocation_sites(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(968, v)
  }

  /// print generated code (developer-only)
  #[inline]
  pub fn print_code(&self) -> bool {
    self.get_bool(969)
  }
  #[inline]
  pub fn set_print_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(969, v)
  }

  /// print optimized code (developer-only)
  #[inline]
  pub fn print_opt_code(&self) -> bool {
    self.get_bool(970)
  }
  #[inline]
  pub fn set_print_opt_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(970, v)
  }

  /// filter for printing optimized code
  #[inline]
  pub fn print_opt_code_filter(&self) -> Option<&str> {
    self.get_str(971)
  }
  #[inline]
  pub fn set_print_opt_code_filter(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(971, Some(Cow::Owned(v.to_string())))
  }

  /// print more information for code (developer-only)
  #[inline]
  pub fn print_code_verbose(&self) -> bool {
    self.get_bool(972)
  }
  #[inline]
  pub fn set_print_code_verbose(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(972, v)
  }

  /// print generated code for builtins (developer-only)
  #[inline]
  pub fn print_builtin_code(&self) -> bool {
    self.get_bool(973)
  }
  #[inline]
  pub fn set_print_builtin_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(973, v)
  }

  /// filter for printing builtin code
  #[inline]
  pub fn print_builtin_code_filter(&self) -> Option<&str> {
    self.get_str(974)
  }
  #[inline]
  pub fn set_print_builtin_code_filter(&mut self, v: &str) -> crate::Result<()> {
    self.assign_str(974, Some(Cow::Owned(v.to_string())))
  }

  /// print generated regexp code (developer-only)
  #[inline]
  pub fn print_regexp_code(&self) -> bool {
    self.get_bool(975)
  }
  #[inline]
  pub fn set_print_regexp_code(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(975, v)
  }

  /// print generated regexp bytecode (developer-only)
  #[inline]
  pub fn print_regexp_bytecode(&self) -> bool {
    self.get_bool(976)
  }
  #[inline]
  pub fn set_print_regexp_bytecode(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(976, v)
  }

  /// enable predictable mode
  #[inline]
  pub fn predictable(&self) -> bool {
    self.get_bool(977)
  }
  #[inline]
  pub fn set_predictable(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(977, v)
  }

  /// Predictable garbage collection schedule. Fixes heap growing, idle, and memory reducing behavior.
  #[inline]
  pub fn predictable_gc_schedule(&self) -> bool {
    self.get_bool(978)
  }
  #[inline]
  pub fn set_predictable_gc_schedule(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(978, v)
  }

  /// disable the use of background tasks
  #[inline]
  pub fn single_threaded(&self) -> bool {
    self.get_bool(979)
  }
  #[inline]
  pub fn set_single_threaded(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(979, v)
  }

  /// disable the use of background gc tasks
  #[inline]
  pub fn single_threaded_gc(&self) -> bool {
    self.get_bool(980)
  }
  #[inline]
  pub fn set_single_threaded_gc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(980, v)
  }

  /// disable the use of background gc tasks when in background
  #[inline]
  pub fn single_threaded_gc_in_background(&self) -> bool {
    self.get_bool(981)
  }
  #[inline]
  pub fn set_single_threaded_gc_in_background(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(981, v)
  }

  /// Use parallel threads in the atomic pause for background GCs
  #[inline]
  pub fn parallel_pause_for_gc_in_background(&self) -> bool {
    self.get_bool(982)
  }
  #[inline]
  pub fn set_parallel_pause_for_gc_in_background(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(982, v)
  }

  /// Use parallel threads in the atomic pause for background GCs
  #[inline]
  pub fn incremental_marking_for_gc_in_background(&self) -> bool {
    self.get_bool(983)
  }
  #[inline]
  pub fn set_incremental_marking_for_gc_in_background(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(983, v)
  }

  /// Enables a shared heap between isolates.
  #[inline]
  pub fn shared_heap(&self) -> bool {
    self.get_bool(984)
  }
  #[inline]
  pub fn set_shared_heap(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(984, v)
  }

  /// Enables a shared heap, but not shared allocations
  #[inline]
  pub fn empty_shared_heap(&self) -> bool {
    self.get_bool(985)
  }
  #[inline]
  pub fn set_empty_shared_heap(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(985, v)
  }

  /// Enable optimizing a sequence of `Class_X.prototype.[key] = ...`by replacing it by a runtime code somewhat equivalent to `Object.assign(Class_X.prototype, [boilerplate_obj])`
  #[inline]
  pub fn proto_assign_seq_opt(&self) -> bool {
    self.get_bool(986)
  }
  #[inline]
  pub fn set_proto_assign_seq_opt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(986, v)
  }

  /// The minimum number of consecutive property assignments on the prototype object for replacing it with a single byte code
  #[inline]
  pub fn proto_assign_seq_opt_count(&self) -> u32 {
    self.get_u32(987)
  }
  #[inline]
  pub fn set_proto_assign_seq_opt_count(&mut self, v: u32) -> crate::Result<()> {
    self.assign_u32(987, v)
  }

  /// Functions bulk assigned to prototype via SetPrototypeProperties will remain in SharedFunctionInfo (Rather than JSFunction) State until first access (experimental)
  #[inline]
  pub fn proto_assign_seq_lazy_func_opt(&self) -> bool {
    self.get_bool(988)
  }
  #[inline]
  pub fn set_proto_assign_seq_lazy_func_opt(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(988, v)
  }

  /// Use specialized bytecodes for Get/Set Private Fields (experimental)
  #[inline]
  pub fn private_field_bytecodes(&self) -> bool {
    self.get_bool(989)
  }
  #[inline]
  pub fn set_private_field_bytecodes(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(989, v)
  }

  /// Track TypedArrays and DataViews attached to array buffers and update them in-place when the buffer detaches
  #[inline]
  pub fn track_array_buffer_views(&self) -> bool {
    self.get_bool(990)
  }
  #[inline]
  pub fn set_track_array_buffer_views(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(990, v)
  }

  /// Allow arbitrary length spread arguments on supported builtins (experimental)
  #[inline]
  pub fn superspreading(&self) -> bool {
    self.get_bool(991)
  }
  #[inline]
  pub fn set_superspreading(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(991, v)
  }

  /// Support proxy traps in ICs
  #[inline]
  pub fn fast_proxy_ic(&self) -> bool {
    self.get_bool(992)
  }
  #[inline]
  pub fn set_fast_proxy_ic(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(992, v)
  }

  /// Tells V8 it's running as part of the d8. This flag should not be set in other cases.
  #[inline]
  pub fn is_standalone_d8_shell(&self) -> bool {
    self.get_bool(993)
  }
  #[inline]
  pub fn set_is_standalone_d8_shell(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(993, v)
  }

  /// Run programs as security proof of concept (POC).
  #[inline]
  pub fn run_as_security_poc(&self) -> bool {
    self.get_bool(994)
  }
  #[inline]
  pub fn set_run_as_security_poc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(994, v)
  }

  /// Run programs as security proof of concept (POC) for the V8 sandbox
  #[inline]
  pub fn run_as_sandbox_security_poc(&self) -> bool {
    self.get_bool(995)
  }
  #[inline]
  pub fn set_run_as_sandbox_security_poc(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(995, v)
  }

  /// this mode is used for checking that V8 behaves predictably
  #[inline]
  pub fn verify_predictable(&self) -> bool {
    self.get_bool(996)
  }
  #[inline]
  pub fn set_verify_predictable(&mut self, v: bool) -> crate::Result<()> {
    self.assign_bool(996, v)
  }

  /// dump allocations digest each n-th allocation
  #[inline]
  pub fn dump_allocations_digest_at_alloc(&self) -> i32 {
    self.get_i32(997)
  }
  #[inline]
  pub fn set_dump_allocations_digest_at_alloc(&mut self, v: i32) -> crate::Result<()> {
    self.assign_i32(997, v)
  }
}

// ── 常量 (原版 flags.cc kNumFlags 等; 分类计数见 counts.rs) ──
pub const NUM_FLAGS: usize = 998; // 主 flag 数 (原版 kNumFlags)
pub const NUM_ALL_FLAGS: usize = 999; // 主 flag + 别名 (原版 kNumAllFlags)
