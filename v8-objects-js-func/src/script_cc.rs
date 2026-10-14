// 由 tool/src/objects/script.cc.js 从 v8/src/objects/script.{-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// Script tq 字段布局 + ScriptFlags 位段 + OriginOptions 位标志 + inl/cc 方法清单 (h 枚举归 script.h.js 席)

//! 差集骨架: Script 对象族的规则代码。布局按 64 位非压缩 (kTaggedSize=8)。
//! V8_SCRIPTORMODULE_LEGACY_LIFETIME 关: script_or_modules 字段不落地。

// ── Script tq 字段布局 (offset 表) ──
/// v8: src/objects/script.tq extern class Script (字段, offset)
pub const SCRIPT_TQ_FIELD_OFFSETS: &[(&str, u32)] = &[
  ("source", 8),
  ("name", 16),
  ("line_offset", 24),
  ("column_offset", 32),
  ("context_data", 40),
  ("script_type", 48),
  ("line_ends", 56),
  ("id", 64),
  ("eval_from_shared_or_wrapped_arguments", 72),
  ("eval_from_position", 80),
  ("eval_from_scope_info", 88),
  ("infos", 96),
  ("compiled_lazy_function_positions", 104),
  ("flags", 112),
  ("source_url", 120),
  ("source_mapping_url", 128),
  ("debug_id", 136),
  ("host_defined_options", 144),
  ("source_hash", 152),
];

/// v8: src/objects/script.h sizeof(Script) — map 8 + 19 tagged × 8 (非压缩)
pub const K_SCRIPT_HEADER_SIZE: usize = 160;

// ── ScriptFlags 位段 (SmiTagged 载体) ──
/// v8: src/objects/script.tq bitfield struct ScriptFlags extends uint31 (段名, shift, width)
pub const SCRIPT_TQ_FLAGS_SEGMENTS: [(&str, u8, u8); 8] = [
  ("compilation_kind", 0, 3),
  ("compilation_state", 3, 1),
  ("is_repl_mode", 4, 1),
  ("origin_options", 5, 4),
  ("break_on_entry", 9, 1),
  ("produce_compile_hints", 10, 1),
  ("deserialized", 11, 1),
  ("outer_language_mode", 12, 1),
];

// ── ScriptOriginOptions 位标志 ──
/// v8: src/objects/script.h ScriptOriginOptions enum — origin_options 4 bit 载体
pub const K_IS_SHARED_CROSS_ORIGIN: i32 = 1;
/// v8: src/objects/script.h kIsOpaque
pub const K_IS_OPAQUE: i32 = 1 << 1;
/// v8: src/objects/script.h kIsWasm
pub const K_IS_WASM: i32 = 1 << 2;
/// v8: src/objects/script.h kIsModule
pub const K_IS_MODULE: i32 = 1 << 3;

// ── script-inl.h 方法清单 (61 个) ──
/// v8: src/objects/script-inl.h
pub const SCRIPT_INL_METHODS: &[&str] = &[
  "Script::source",
  "Script::set_source",
  "Script::name",
  "Script::set_name",
  "Script::line_offset",
  "Script::set_line_offset",
  "Script::column_offset",
  "Script::set_column_offset",
  "Script::context_data",
  "Script::set_context_data",
  "Script::type",
  "Script::set_type",
  "Script::line_ends",
  "Script::set_line_ends",
  "Script::id",
  "Script::set_id",
  "Script::set_eval_from_shared_or_wrapped_arguments",
  "Script::is_wrapped",
  "Script::is_eval",
  "Script::has_eval_origin",
  "Script::is_host",
  "Script::outer_language_mode",
  "Script::set_outer_language_mode",
  "Script::has_eval_from_shared",
  "Script::set_wrapped_arguments",
  "Script::wrapped_arguments",
  "Script::eval_from_position",
  "Script::set_eval_from_position",
  "Script::eval_from_scope_info",
  "Script::has_eval_from_scope_info",
  "Script::infos",
  "Script::set_infos",
  "Script::flags",
  "Script::set_flags",
  "Script::compilation_kind",
  "Script::set_compilation_kind",
  "Script::compilation_state",
  "Script::set_compilation_state",
  "Script::produce_compile_hints",
  "Script::set_produce_compile_hints",
  "Script::deserialized",
  "Script::set_deserialized",
  "Script::is_repl_mode",
  "Script::set_is_repl_mode",
  "Script::origin_options",
  "Script::set_origin_options",
  "Script::compiled_lazy_function_positions",
  "Script::set_compiled_lazy_function_positions",
  "Script::source_url",
  "Script::set_source_url",
  "Script::source_mapping_url",
  "Script::set_source_mapping_url",
  "Script::debug_id",
  "Script::set_debug_id",
  "Script::host_defined_options",
  "Script::set_host_defined_options",
  "Script::source_hash",
  "Script::set_source_hash",
  "Script::has_line_ends",
  "Script::CanHaveLineEnds",
  "Script::InitLineEnds",
];

// ── script.cc 方法清单 (28 个) ──
/// v8: src/objects/script.cc
pub const SCRIPT_CC_METHODS: &[&str] = &[
  "Script::set_eval_from_scope_info",
  "Script::HasValidSource",
  "Script::HasSourceURLComment",
  "Script::HasSourceMappingURLComment",
  "Script::IsMaybeUnfinalized",
  "Script::FindSharedFunctionInfo",
  "Iterator::Iterator",
  "Iterator::Next",
  "Script::GetEvalPosition",
  "Script::set_eval_from_shared",
  "Script::eval_from_shared",
  "Script::GetEvalOrigin",
  "Script::GetLineEnds",
  "Script::InitLineEndsInternal",
  "Script::SetSource",
  "Script::GetPositionInfo",
  "Script::IsSubjectToDebugging",
  "Script::IsUserJavaScript",
  "Script::TraceScriptRundown",
  "Script::TraceScriptRundownSources",
  "Script::AddPositionInfoOffset",
  "Script::GetPositionInfoInternal",
  "Script::GetPositionInfoWithLineEnds",
  "Script::GetLineColumnWithLineEnds",
  "Script::GetColumnNumber",
  "Script::GetLineNumber",
  "Script::GetNameOrSourceURL",
  "Script::GetScriptHash",
];

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
  assert!(SCRIPT_TQ_FIELD_OFFSETS.len() == 19);
  let f0 = SCRIPT_TQ_FIELD_OFFSETS[0];
  assert!(str_eq(f0.0, "source") && f0.1 == 8);
  let f13 = SCRIPT_TQ_FIELD_OFFSETS[13];
  assert!(str_eq(f13.0, "flags") && f13.1 == 112);
  let f18 = SCRIPT_TQ_FIELD_OFFSETS[18];
  assert!(str_eq(f18.0, "source_hash") && f18.1 == 152);
  assert!(K_SCRIPT_HEADER_SIZE == 160);
  assert!(SCRIPT_TQ_FLAGS_SEGMENTS.len() == 8);
  let g0 = SCRIPT_TQ_FLAGS_SEGMENTS[0];
  assert!(str_eq(g0.0, "compilation_kind") && g0.1 == 0 && g0.2 == 3);
  let g3 = SCRIPT_TQ_FLAGS_SEGMENTS[3];
  assert!(str_eq(g3.0, "origin_options") && g3.1 == 5 && g3.2 == 4);
  let g7 = SCRIPT_TQ_FLAGS_SEGMENTS[7];
  assert!(str_eq(g7.0, "outer_language_mode") && g7.1 == 12 && g7.2 == 1);
  assert!(width_sum(&SCRIPT_TQ_FLAGS_SEGMENTS) == 13);
  assert!(K_IS_SHARED_CROSS_ORIGIN == 1 && K_IS_OPAQUE == 2 && K_IS_WASM == 4 && K_IS_MODULE == 8);
  assert!(SCRIPT_INL_METHODS.len() >= 40);
  assert!(SCRIPT_CC_METHODS.len() >= 20);
};
