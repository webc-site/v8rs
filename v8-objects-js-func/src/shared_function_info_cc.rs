// 由 tool/src/objects/shared-function-info.cc.js 从 v8/src/objects/shared-function-info.{cc,tq} 生成; 修订就地改, 复跑覆盖
// tq 布局位段/类清单 + cc 方法清单 + formal-parameter-count macro 语义 (h 差集归 shared-function-info.h.js 席)

//! 差集骨架: shared-function-info.cc + .tq 的规则代码。
//! 位段经 .h BitField 链 (clang -E) 对数; 大函数体语义面留 Developer。

/// v8: src/objects/shared-function-info.tq bitfield struct SharedFunctionInfoFlags extends uint32
///   (段名, shift, width) — 与 .h FunctionKindBits..IsHoistedInContextBit 链 1:1, 总宽 30
pub const SFI_TQ_FLAGS_SEGMENTS: [(&str, u8, u8); 17] = [
  ("function_kind", 0, 5),
  ("is_native", 5, 1),
  ("is_strict", 6, 1),
  ("function_syntax_kind", 7, 3),
  ("is_class_constructor", 10, 1),
  ("has_duplicate_parameters", 11, 1),
  ("allow_lazy_compilation", 12, 1),
  ("function_map_index", 13, 5),
  ("disabled_optimization_reason", 18, 4),
  ("requires_instance_members_initializer", 22, 1),
  ("construct_as_builtin", 23, 1),
  ("name_should_print_as_anonymous", 24, 1),
  ("has_reported_binary_coverage", 25, 1),
  ("is_top_level", 26, 1),
  ("properties_are_final", 27, 1),
  ("private_name_lookup_skips_outer_class", 28, 1),
  ("is_hoisted_in_context", 29, 1),
];

/// v8: src/objects/shared-function-info.tq bitfield struct SharedFunctionInfoFlags2 extends uint8
///   总宽 8 (uint8 全占)
pub const SFI_TQ_FLAGS2_SEGMENTS: [(&str, u8, u8); 6] = [
  ("class_scope_has_private_brand", 0, 1),
  ("has_static_private_methods_or_accessors", 1, 1),
  ("is_sparkplug_compiling", 2, 1),
  ("maglev_compilation_failed", 3, 1),
  ("cached_tiering_decision", 4, 3),
  ("function_context_independent_compiled", 7, 1),
];

/// v8: src/objects/shared-function-info.tq @cppObjectLayoutDefinition 类清单 (类名, 字段数)
pub const SFI_TQ_CLASSES: &[(&str, usize)] = &[
  ("PreparseData", 2),
  ("InterpreterData", 2),
  ("SharedFunctionInfo", 15),
  ("SharedFunctionInfoWrapper", 1),
  ("UncompiledData", 3),
  ("UncompiledDataWithoutPreparseData", 1),
  ("UncompiledDataWithPreparseData", 1),
  ("UncompiledDataWithoutPreparseDataWithJob", 1),
  ("UncompiledDataWithPreparseDataAndJob", 1),
  ("OnHeapBasicBlockProfilerData", 7),
];

/// v8: src/objects/shared-function-info.tq extern class SharedFunctionInfo 字段序 (布局序)
pub const SFI_TQ_SFI_FIELDS: &[&str] = &[
  "trusted_function_data",
  "untrusted_function_data",
  "name_or_scope_info",
  "outer_scope_info_or_feedback_metadata",
  "script",
  "length",
  "formal_parameter_count",
  "function_token_offset",
  "expected_nof_properties",
  "flags2",
  "flags",
  "function_literal_id",
  "unique_id",
  "age",
  "feedback_slot",
];

// ── cc 方法清单 (43 个定义, oracle: clang -E 尾部区段) ──
/// v8: src/objects/shared-function-info.cc
pub const SHARED_FUNCTION_INFO_CC_METHODS: &[&str] = &[
  "ReplacementStringBuilder::AddSubjectSlice",
  "IncrementalStringBuilder::Append",
  "IncrementalStringBuilder::ChangeEncoding",
  "SharedFunctionInfo::GetTrustedData",
  "SharedFunctionInfo::Hash",
  "SharedFunctionInfo::GetCode",
  "ScriptIterator::ScriptIterator",
  "ScriptIterator::Next",
  "ScriptIterator::Reset",
  "SharedFunctionInfo::SetScript",
  "SharedFunctionInfo::CopyFrom",
  "SharedFunctionInfo::HasDebugInfo",
  "SharedFunctionInfo::UniqueIdInScript",
  "SharedFunctionInfo::GetDebugInfo",
  "SharedFunctionInfo::TryGetDebugInfo",
  "SharedFunctionInfo::HasBreakInfo",
  "SharedFunctionInfo::BreakAtEntry",
  "SharedFunctionInfo::HasCoverageInfo",
  "SharedFunctionInfo::GetCoverageInfo",
  "SharedFunctionInfo::DebugName",
  "SharedFunctionInfo::PassesFilter",
  "SharedFunctionInfo::HasSourceCode",
  "SharedFunctionInfo::DiscardCompiledMetadata",
  "SharedFunctionInfo::DiscardCompiled",
  "SharedFunctionInfo::GetSourceCode",
  "SharedFunctionInfo::GetSourceCodeHarmony",
  "SharedFunctionInfo::SourceSize",
  "SharedFunctionInfo::DisableOptimization",
  "SharedFunctionInfo::InitFromFunctionLiteral",
  "SharedFunctionInfo::CreateAndSetUncompiledData",
  "SharedFunctionInfo::get_property_estimate_from_literal",
  "SharedFunctionInfo::UpdateExpectedNofPropertiesFromEstimate",
  "SharedFunctionInfo::UpdateAndFinalizeExpectedNofPropertiesFromEstimate",
  "SharedFunctionInfo::SetFunctionTokenPosition",
  "SharedFunctionInfo::StartPosition",
  "SharedFunctionInfo::EndPosition",
  "SharedFunctionInfo::cached_tiering_decision",
  "SharedFunctionInfo::set_cached_tiering_decision",
  "SharedFunctionInfo::EnsureBytecodeArrayAvailable",
  "SharedFunctionInfo::EnsureSourcePositionsAvailable",
  "SharedFunctionInfo::InstallDebugBytecode",
  "SharedFunctionInfo::UninstallDebugBytecode",
  "SharedFunctionInfo::EnsureOldForTesting",
];

// ── tq @export macro 语义 (真实现) ──
/// v8: src/objects/shared-function-info.tq const kDontAdaptArgumentsSentinel
pub const K_DONT_ADAPT_ARGUMENTS_SENTINEL: u16 = 0;

/// v8: src/objects/shared-function-info.tq LoadSharedFunctionInfoFormalParameterCountWithoutReceiver
///   参数不适配哨兵时原样返回, 否则扣去 receiver 槽位 (kJSArgcReceiverSlots = 1)
#[inline]
#[must_use]
pub const fn formal_parameter_count_without_receiver(formal_parameter_count: u16) -> u16 {
  if formal_parameter_count != K_DONT_ADAPT_ARGUMENTS_SENTINEL {
    formal_parameter_count - 1
  } else {
    formal_parameter_count
  }
}

/// v8: src/objects/shared-function-info.tq LoadSharedFunctionInfoFormalParameterCountWithReceiver
#[inline]
#[must_use]
pub const fn formal_parameter_count_with_receiver(formal_parameter_count: u16) -> u16 {
  formal_parameter_count
}

/// v8: src/objects/shared-function-info.tq IsSharedFunctionInfoDontAdaptArguments
#[inline]
#[must_use]
pub const fn is_dont_adapt_arguments(formal_parameter_count: u16) -> bool {
  formal_parameter_count == K_DONT_ADAPT_ARGUMENTS_SENTINEL
}

// ── 对数不变式 ──
const fn width_sum(segs: &[(&str, u8, u8)]) -> u32 {
  let mut s = 0u32;
  let mut i = 0;
  while i < segs.len() {
    s += segs[i].2 as u32;
    i += 1;
  }
  s
}

const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

const _: () = {
  assert!(SFI_TQ_FLAGS_SEGMENTS.len() == 17);
  let s0 = SFI_TQ_FLAGS_SEGMENTS[0];
  assert!(str_eq(s0.0, "function_kind") && s0.1 == 0 && s0.2 == 5);
  let s3 = SFI_TQ_FLAGS_SEGMENTS[3];
  assert!(str_eq(s3.0, "function_syntax_kind") && s3.1 == 7 && s3.2 == 3);
  let s4 = SFI_TQ_FLAGS_SEGMENTS[4];
  assert!(str_eq(s4.0, "is_class_constructor") && s4.1 == 10 && s4.2 == 1);
  let s16 = SFI_TQ_FLAGS_SEGMENTS[16];
  assert!(str_eq(s16.0, "is_hoisted_in_context") && s16.1 == 29 && s16.2 == 1);
  assert!(width_sum(&SFI_TQ_FLAGS_SEGMENTS) == 30);
  assert!(SFI_TQ_FLAGS2_SEGMENTS.len() == 6);
  let f2 = SFI_TQ_FLAGS2_SEGMENTS[4];
  assert!(str_eq(f2.0, "cached_tiering_decision") && f2.1 == 4 && f2.2 == 3);
  assert!(width_sum(&SFI_TQ_FLAGS2_SEGMENTS) == 8);
  assert!(SFI_TQ_CLASSES.len() == 10);
  assert!(SFI_TQ_SFI_FIELDS.len() == 15);
  assert!(str_eq(SFI_TQ_SFI_FIELDS[0], "trusted_function_data"));
  assert!(str_eq(SFI_TQ_SFI_FIELDS[14], "feedback_slot"));
  assert!(SHARED_FUNCTION_INFO_CC_METHODS.len() >= 40);
  assert!(K_DONT_ADAPT_ARGUMENTS_SENTINEL == 0);
  assert!(formal_parameter_count_without_receiver(3) == 2);
  assert!(formal_parameter_count_without_receiver(0) == 0);
  assert!(is_dont_adapt_arguments(0));
  assert!(!is_dont_adapt_arguments(4));
};
