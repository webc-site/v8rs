// 由 tool/src/wasm/wasm-features.cc.js 从 v8/src/wasm/wasm-features.{h,cc} 生成; 修订就地改, 复跑覆盖
// 特性启用集 FromFlags 规则表 + 骨架 (枚举本体在 wasm_features.rs)
// 深水推迟 (W10WC 台账): FromIsolate/FromContext 依赖 Isolate/Context 对象面
// (前置: v8-objects 上下文 + custom_descriptors 上下文门控), 骨架函数已移除, 待补波填体。

use crate::wasm_features::{WasmEnabledFeature, WasmEnabledFeatures};

/// 特性 → v8 flag 名映射 (对应 CHECK_FEATURE_FLAG 展开中的 v8_flags.wasm_##feat)
/// 顺序 = FOREACH_WASM_FEATURE_FLAG 展开序, 共 14 条
pub static FEATURE_FLAG_NAMES: &[(WasmEnabledFeature, &str)] = &[
  (
    WasmEnabledFeature::InstructionTracing,
    "wasm_instruction_tracing",
  ),
  (WasmEnabledFeature::Shared, "wasm_shared"),
  (WasmEnabledFeature::Fp16, "wasm_fp16"),
  (WasmEnabledFeature::MemoryControl, "wasm_memory_control"),
  (WasmEnabledFeature::Wasmfx, "wasm_wasmfx"),
  (
    WasmEnabledFeature::CompilationHints,
    "wasm_compilation_hints",
  ),
  (WasmEnabledFeature::GrowableStacks, "wasm_growable_stacks"),
  (WasmEnabledFeature::Stringref, "wasm_stringref"),
  (
    WasmEnabledFeature::ImportedStringsUtf8,
    "wasm_imported_strings_utf8",
  ),
  (WasmEnabledFeature::AcquireRelease, "wasm_acquire_release"),
  (WasmEnabledFeature::CompactImports, "wasm_compact_imports"),
  (
    WasmEnabledFeature::CustomDescriptors,
    "wasm_custom_descriptors",
  ),
  (WasmEnabledFeature::WideArithmetic, "wasm_wide_arithmetic"),
  (WasmEnabledFeature::LegacyEh, "wasm_legacy_eh"),
];

/// jitless (DrumBrake) 模式下强制启用的特性 (对应 FromFlags 中 V8_ENABLE_DRUMBRAKE 分支)
pub static JITLESS_FORCED_FEATURES: &[WasmEnabledFeature] = &[WasmEnabledFeature::LegacyEh];

/// 上下文侧额外启用 (对应 FromContext: IsWasmCustomDescriptorsEnabled)
pub static CONTEXT_GATED_FEATURES: &[WasmEnabledFeature] = &[WasmEnabledFeature::CustomDescriptors];

/// 从 v8_flags 构建 (对应 WasmEnabledFeatures::FromFlags, 1:1)
/// flag_enabled 注入式读取 v8 flag (v8-flags 面接入后可直连); jitless 对应 v8_flags.wasm_jitless。
#[must_use]
pub fn from_flags(flag_enabled: impl Fn(&str) -> bool, jitless: bool) -> WasmEnabledFeatures {
  let mut features = WasmEnabledFeatures::none();
  // 原版: #if V8_ENABLE_DRUMBRAKE if (v8_flags.wasm_jitless) features.Add(legacy_eh)
  if jitless {
    for f in JITLESS_FORCED_FEATURES {
      features.add(*f);
    }
  }
  // 原版: CHECK_FEATURE_FLAG: if (!v8_flags.wasm_jitless && v8_flags.wasm_##feat) features.Add(feat)
  if !jitless {
    for (feat, flag) in FEATURE_FLAG_NAMES {
      if flag_enabled(flag) {
        features.add(*feat);
      }
    }
  }
  features
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FEATURE_FLAG_NAMES.len() == 14);
  assert!(JITLESS_FORCED_FEATURES.len() == 1);
  assert!(CONTEXT_GATED_FEATURES.len() == 1);
};
