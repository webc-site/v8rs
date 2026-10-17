//! WasmFeatures 位标志测试: 组合/查询/名称/CompileTimeImports
use aok::{OK, Void};
use v8_base::enum_set::{EnumOrdinal, EnumSet};
use v8_wasm_core::wasm_features::{
  CompileTimeImport, CompileTimeImports, WasmDetectedFeature, WasmDetectedFeatures,
  WasmEnabledFeature, WasmEnabledFeatures, detected_feature_name, enabled_feature_name,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// WasmEnabledFeatures: all/none/位组合/has_* getter
#[test]
fn enabled_features_sets() -> Void {
  let all = WasmEnabledFeatures::all();
  let none = WasmEnabledFeatures::none();
  assert!(!none.has_shared());
  assert!(all.has_shared()); // experimental: shared-everything
  assert!(all.has_legacy_eh()); // shipped
  assert!(all.has_custom_descriptors());
  assert!(all.has_wide_arithmetic());
  assert!(all.has_stringref()); // pre-staged
  // 交集/包含
  assert!(none.0.is_subset_of(all.0));
  // ForAsmjs = 空
  assert_eq!(WasmEnabledFeatures::for_asmjs(), none);
  OK
}

/// 特性组合: from_elements + add/remove
#[test]
fn enabled_features_ops() -> Void {
  let mut s = WasmEnabledFeatures::none();
  s.add(WasmEnabledFeature::Wasmfx);
  s.add(WasmEnabledFeature::Fp16);
  assert!(s.has_wasmfx() && s.has_fp16());
  assert!(!s.has_memory_control());
  s.remove(WasmEnabledFeature::Fp16);
  assert!(!s.has_fp16());
  // 位集形态
  assert_eq!(s.count(), 1);
  OK
}

/// WasmDetectedFeatures: flag + 恒开特性 (u64 位图, 35 项)
#[test]
fn detected_features() -> Void {
  let mut d = WasmDetectedFeatures::default();
  d.add_gc(); // 恒开
  d.add_bulk_memory(); // 恒开
  d.add_legacy_eh(); // flag
  d.add_branch_hinting(); // 恒开
  assert!(d.has_gc() && d.has_bulk_memory() && d.has_legacy_eh() && d.has_branch_hinting());
  assert!(!d.has_simd());
  // add (EnumSet 泛型路径) 与 add_* 等价
  let mut d2 = WasmDetectedFeatures::default();
  d2.add(WasmDetectedFeature::Gc);
  assert!(d2.has_gc());
  // 序号: flag 特性 0..14, 恒开特性 15..35
  assert_eq!(WasmDetectedFeature::LegacyEh.ordinal(), 13);
  assert_eq!(WasmDetectedFeature::GcAllocation.ordinal(), 34);
  OK
}

/// name 函数与 strum Display 同源
#[test]
fn feature_names() -> Void {
  assert_eq!(enabled_feature_name(WasmEnabledFeature::Shared), "shared");
  assert_eq!(
    enabled_feature_name(WasmEnabledFeature::LegacyEh),
    "legacy_eh"
  );
  assert_eq!(
    detected_feature_name(WasmDetectedFeature::SharedMemory),
    "shared_memory"
  );
  assert_eq!(
    detected_feature_name(WasmDetectedFeature::Memory64),
    "memory64"
  );
  // strum Display
  assert_eq!(WasmEnabledFeature::Wasmfx.to_string(), "wasmfx");
  OK
}

/// CompileTimeImports: 位标志 + constants_module
#[test]
fn compile_time_imports() -> Void {
  let mut imports = CompileTimeImports::default();
  assert!(imports.is_empty());
  imports.add(CompileTimeImport::StringConstants);
  imports.set_constants_module("builtins: string-constants");
  assert!(imports.has_string_constants(b"builtins: string-constants"));
  assert!(!imports.has_string_constants(b"other"));
  assert!(imports.contains(CompileTimeImport::StringConstants));
  assert!(!imports.contains(CompileTimeImport::JsString));
  imports.remove(CompileTimeImport::StringConstants);
  assert!(!imports.contains(CompileTimeImport::StringConstants));
  assert_eq!(imports.constants_module(), "builtins: string-constants");
  OK
}

/// EnumSet 位集形态: to_integral/from_integral 往返 (原版位集语义)
#[test]
fn enumset_roundtrip() -> Void {
  let s = EnumSet::from_elements(&[WasmEnabledFeature::Shared, WasmEnabledFeature::Fp16]);
  let bits = s.to_integral();
  let s2: EnumSet<WasmEnabledFeature> = EnumSet::from_integral(bits);
  assert!(s == s2);
  assert_eq!(s.count(), 2);
  OK
}
