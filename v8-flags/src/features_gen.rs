// 由 tool/src/flags/flag-definitions.h.js 从 v8/src/flags/feature-flags.h 生成; 修订就地改, 复跑覆盖
// 规范配置: INTL 开 / REVEC 关 / SPARKPLUG_PLUS 关

/// 特性 flag 表 (阶段, 种类, flag 名, 描述)
pub struct Feature {
  pub stage: &'static str,
  pub kind: &'static str,
  pub name: &'static str,
  pub desc: &'static str,
}
pub(crate) static FEATURES: &[Feature] = &[
  Feature {
    stage: "EXPERIMENTAL",
    kind: "js",
    name: "harmony_shadow_realm",
    desc: "harmony ShadowRealm",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "js",
    name: "harmony_struct",
    desc: "harmony structs, shared structs, and shared arrays",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "js",
    name: "harmony_intl_best_fit_matcher",
    desc: "Intl BestFitMatcher",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "js",
    name: "js_decorators",
    desc: "decorators",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "js",
    name: "js_source_phase_imports",
    desc: "source phase imports",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "js",
    name: "js_regexp_buffer_boundaries",
    desc: "RegExp \\\\A \\\\z \\\\Z buffer boundary assertions",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "wasm",
    name: "wasm_instruction_tracing",
    desc: "instruction tracing section",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "wasm",
    name: "wasm_shared",
    desc: "shared-everything threads",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "wasm",
    name: "wasm_fp16",
    desc: "fp16",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "wasm",
    name: "wasm_memory_control",
    desc: "memory control",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "wasm",
    name: "wasm_wasmfx",
    desc: "core stack switching",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "wasm",
    name: "wasm_compilation_hints",
    desc: "compilation hints",
  },
  Feature {
    stage: "EXPERIMENTAL",
    kind: "wasm",
    name: "wasm_growable_stacks",
    desc: "growable stacks for jspi",
  },
  Feature {
    stage: "PRE_STAGED",
    kind: "internal",
    name: "always_specialize_for_script_context",
    desc: "always specialize for script contexts in optimized code",
  },
  Feature {
    stage: "PRE_STAGED",
    kind: "wasm",
    name: "wasm_stringref",
    desc: "reference-typed strings",
  },
  Feature {
    stage: "PRE_STAGED",
    kind: "wasm",
    name: "wasm_imported_strings_utf8",
    desc: "imported strings (utf8 features)",
  },
  Feature {
    stage: "PRE_STAGED",
    kind: "wasm",
    name: "wasm_acquire_release",
    desc: "acquire_release memory ordering",
  },
  Feature {
    stage: "PRE_STAGED",
    kind: "wasm",
    name: "wasm_compact_imports",
    desc: "compact import section",
  },
  Feature {
    stage: "STAGED",
    kind: "js",
    name: "js_immutable_arraybuffer",
    desc: "Immutable ArrayBuffer",
  },
  Feature {
    stage: "STAGED",
    kind: "js",
    name: "js_import_bytes",
    desc: "import bytes",
  },
  Feature {
    stage: "STAGED",
    kind: "js",
    name: "js_defer_import_eval",
    desc: "defer import eval",
  },
  Feature {
    stage: "STAGED",
    kind: "wasm",
    name: "wasm_custom_descriptors",
    desc: "custom descriptors",
  },
  Feature {
    stage: "STAGED",
    kind: "wasm",
    name: "wasm_wide_arithmetic",
    desc: "wide arithmetic",
  },
  Feature {
    stage: "SHIPPED",
    kind: "js",
    name: "harmony_import_attributes",
    desc: "harmony import attributes",
  },
  Feature {
    stage: "SHIPPED",
    kind: "js",
    name: "harmony_temporal",
    desc: "Temporal",
  },
  Feature {
    stage: "SHIPPED",
    kind: "js",
    name: "js_esm_ns_reexport",
    desc: "Support diamond-importing re-expored namespaces (https://github.com/tc39/ecma262/pull/3715)",
  },
  Feature {
    stage: "SHIPPED",
    kind: "js",
    name: "js_iterator_join",
    desc: "Iterator.prototype.join",
  },
  Feature {
    stage: "SHIPPED",
    kind: "js",
    name: "js_iterator_sequencing",
    desc: "iterator sequencing",
  },
  Feature {
    stage: "SHIPPED",
    kind: "js",
    name: "js_joint_iteration",
    desc: "joint iteration",
  },
  Feature {
    stage: "SHIPPED",
    kind: "js",
    name: "js_pr_3883",
    desc: "Promise.try not wrapping the result in an extra promise in the non-throwing case (https://github.com/tc39/ecma262/pull/3883)",
  },
  Feature {
    stage: "SHIPPED",
    kind: "js",
    name: "js_iterator_includes",
    desc: "Iterator.prototype.includes",
  },
  Feature {
    stage: "SHIPPED",
    kind: "js",
    name: "js_import_text",
    desc: "import text",
  },
  Feature {
    stage: "SHIPPED",
    kind: "wasm",
    name: "wasm_legacy_eh",
    desc: "legacy exception handling opcodes",
  },
  Feature {
    stage: "SHIPPED",
    kind: "internal",
    name: "homomorphic_ic",
    desc: "Homomorphic IC state for same-handler highly polymorphic ICs",
  },
];
pub const NUM_FEATURES: usize = 34;
