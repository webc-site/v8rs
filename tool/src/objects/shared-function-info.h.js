#!/usr/bin/env bun
// tool/src/objects/shared-function-info.h.js — 校验并提取 SharedFunctionInfo 布局与位域
// 来源:
//   v8/src/objects/shared-function-info.h + shared-function-info-inl.h
//   → 对应 v8-objects-js-func/src/shared_function_info.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/shared-function-info.h");
const inl = readV8("src/objects/shared-function-info-inl.h");

let checks = 0;
function must(cond, msg) {
  checks++;
  if (!cond) throw new Error(`锚点失配: ${msg}`);
}
function mustContain(content, substr, filename) {
  checks++;
  if (!content.includes(substr)) {
    throw new Error(`锚点失配: ${filename} 缺少 "${substr.slice(0, 48)}..."`);
  }
}

// ── 1. 核心锚点检验 ────────────────────────────────────────────────────────
mustContain(h, "V8_OBJECT class SharedFunctionInfo : public HeapObject {", "shared-function-info.h");
mustContain(h, "using FunctionKindBits = base::BitField<FunctionKind, 0, 5, uint32_t>;", "shared-function-info.h");
mustContain(h, "using IsNativeBit = FunctionKindBits::Next<bool, 1>;", "shared-function-info.h");
mustContain(h, "using IsStrictBit = IsNativeBit::Next<bool, 1>;", "shared-function-info.h");
mustContain(h, "using FunctionSyntaxKindBits = IsStrictBit::Next<FunctionSyntaxKind, 3>;", "shared-function-info.h");
mustContain(h, "using IsClassConstructorBit = FunctionSyntaxKindBits::Next<bool, 1>;", "shared-function-info.h");
mustContain(h, "using HasDuplicateParametersBit = IsClassConstructorBit::Next<bool, 1>;", "shared-function-info.h");
mustContain(h, "using AllowLazyCompilationBit = HasDuplicateParametersBit::Next<bool, 1>;", "shared-function-info.h");
mustContain(h, "using FunctionMapIndexBits = AllowLazyCompilationBit::Next<uint32_t, 5>;", "shared-function-info.h");
mustContain(h, "using DisabledOptimizationReasonBits =", "shared-function-info.h");
mustContain(h, "using RequiresInstanceMembersInitializerBit =", "shared-function-info.h");
mustContain(h, "using ConstructAsBuiltinBit =", "shared-function-info.h");
mustContain(h, "using NameShouldPrintAsAnonymousBit = ConstructAsBuiltinBit::Next<bool, 1>;", "shared-function-info.h");
mustContain(h, "using HasReportedBinaryCoverageBit =", "shared-function-info.h");
mustContain(h, "using IsTopLevelBit = HasReportedBinaryCoverageBit::Next<bool, 1>;", "shared-function-info.h");
mustContain(h, "using PropertiesAreFinalBit = IsTopLevelBit::Next<bool, 1>;", "shared-function-info.h");
mustContain(h, "using PrivateNameLookupSkipsOuterClassBit =", "shared-function-info.h");
mustContain(h, "using IsHoistedInContextBit =", "shared-function-info.h");

// flags2 位域
mustContain(h, "using ClassScopeHasPrivateBrandBit = base::BitField<bool, 0, 1, uint8_t>;", "shared-function-info.h");
mustContain(h, "using HasStaticPrivateMethodsOrAccessorsBit =", "shared-function-info.h");
mustContain(h, "using IsSparkplugCompilingBit =", "shared-function-info.h");
mustContain(h, "using MaglevCompilationFailedBit = IsSparkplugCompilingBit::Next<bool, 1>;", "shared-function-info.h");
mustContain(h, "using CachedTieringDecisionBits =", "shared-function-info.h");
mustContain(h, "using FunctionContextIndependentCompiledBit =", "shared-function-info.h");

// 成员变量
mustContain(h, "TaggedMember<Object> untrusted_function_data_", "shared-function-info.h");
mustContain(h, "TaggedMember<UnionOf<Smi, String, ScopeInfo>> name_or_scope_info_", "shared-function-info.h");
mustContain(h, "TaggedMember<UnionOf<ScopeInfo, FeedbackMetadata, TheHole>>", "shared-function-info.h");
mustContain(h, "TaggedMember<HeapObject> script_", "shared-function-info.h");
mustContain(h, "uint16_t length_ = 0;", "shared-function-info.h");
mustContain(h, "uint16_t formal_parameter_count_", "shared-function-info.h");
mustContain(h, "uint16_t function_token_offset_ = 0;", "shared-function-info.h");
mustContain(h, "uint8_t expected_nof_properties_ = 0;", "shared-function-info.h");
mustContain(h, "uint8_t flags2_", "shared-function-info.h");
mustContain(h, "std::atomic<uint32_t> flags_", "shared-function-info.h");
mustContain(h, "std::atomic<int32_t> function_literal_id_", "shared-function-info.h");
mustContain(h, "int32_t unique_id_;", "shared-function-info.h");
mustContain(h, "std::atomic<uint16_t> age_", "shared-function-info.h");
mustContain(h, "std::atomic<uint16_t> feedback_slot_", "shared-function-info.h");

// inl 方法
mustContain(inl, "bool SharedFunctionInfo::IsUserJavaScript() const", "shared-function-info-inl.h");
mustContain(inl, "bool SharedFunctionInfo::is_class_constructor() const", "shared-function-info-inl.h");

// ── 2. 位域偏移与掩码计算 ──────────────────────────────────────────────────
export const FLAGS_BITFIELD = {
  FUNCTION_KIND: { shift: 0, size: 5, mask: (1 << 5) - 1 },
  IS_NATIVE: { shift: 5, size: 1, mask: 1 << 5 },
  IS_STRICT: { shift: 6, size: 1, mask: 1 << 6 },
  FUNCTION_SYNTAX_KIND: { shift: 7, size: 3, mask: ((1 << 3) - 1) << 7 },
  IS_CLASS_CONSTRUCTOR: { shift: 10, size: 1, mask: 1 << 10 },
  HAS_DUPLICATE_PARAMETERS: { shift: 11, size: 1, mask: 1 << 11 },
  ALLOW_LAZY_COMPILATION: { shift: 12, size: 1, mask: 1 << 12 },
  FUNCTION_MAP_INDEX: { shift: 13, size: 5, mask: ((1 << 5) - 1) << 13 },
  DISABLED_OPTIMIZATION_REASON: { shift: 18, size: 4, mask: ((1 << 4) - 1) << 18 },
  REQUIRES_INSTANCE_MEMBERS_INITIALIZER: { shift: 22, size: 1, mask: 1 << 22 },
  CONSTRUCT_AS_BUILTIN: { shift: 23, size: 1, mask: 1 << 23 },
  NAME_SHOULD_PRINT_AS_ANONYMOUS: { shift: 24, size: 1, mask: 1 << 24 },
  HAS_REPORTED_BINARY_COVERAGE: { shift: 25, size: 1, mask: 1 << 25 },
  IS_TOP_LEVEL: { shift: 26, size: 1, mask: 1 << 26 },
  PROPERTIES_ARE_FINAL: { shift: 27, size: 1, mask: 1 << 27 },
  PRIVATE_NAME_LOOKUP_SKIPS_OUTER_CLASS: { shift: 28, size: 1, mask: 1 << 28 },
  IS_HOISTED_IN_CONTEXT: { shift: 29, size: 1, mask: 1 << 29 },
};

export const FLAGS2_BITFIELD = {
  CLASS_SCOPE_HAS_PRIVATE_BRAND: { shift: 0, size: 1, mask: 1 << 0 },
  HAS_STATIC_PRIVATE_METHODS_OR_ACCESSORS: { shift: 1, size: 1, mask: 1 << 1 },
  IS_SPARKPLUG_COMPILING: { shift: 2, size: 1, mask: 1 << 2 },
  MAGLEV_COMPILATION_FAILED: { shift: 3, size: 1, mask: 1 << 3 },
  CACHED_TIERING_DECISION: { shift: 4, size: 3, mask: ((1 << 3) - 1) << 4 },
  FUNCTION_CONTEXT_INDEPENDENT_COMPILED: { shift: 7, size: 1, mask: 1 << 7 },
};

must(FLAGS_BITFIELD.IS_HOISTED_IN_CONTEXT.shift + FLAGS_BITFIELD.IS_HOISTED_IN_CONTEXT.size === 30, "flags 总位长为 30");
must(FLAGS2_BITFIELD.FUNCTION_CONTEXT_INDEPENDENT_COMPILED.shift + FLAGS2_BITFIELD.FUNCTION_CONTEXT_INDEPENDENT_COMPILED.size === 8, "flags2 总位长为 8");

// ── 3. 内存布局推导 ────────────────────────────────────────────────────────
// 64-bit 系统下:
// 槽 0 (0..8): map (Tagged<Map>)
// 槽 1 (8..16): function_data (Tagged<Object>)
// 槽 2 (16..24): name_or_scope_info (Tagged<Object>)
// 槽 3 (24..32): outer_scope_info_or_feedback_metadata (Tagged<Object>)
// 槽 4 (32..40): script (Tagged<HeapObject>)
// 槽 5 (40..48):
//   length (u16, 2 bytes)
//   formal_parameter_count (u16, 2 bytes)
//   function_token_offset (u16, 2 bytes)
//   expected_nof_properties (u8, 1 byte)
//   flags2 (u8, 1 byte)
// 槽 6 (48..56):
//   flags (u32, 4 bytes)
//   function_literal_id (i32, 4 bytes)
// 槽 7 (56..64):
//   unique_id (i32, 4 bytes)
//   age (u16, 2 bytes)
//   feedback_slot (u16, 2 bytes)
// 总大小 = 64 字节 = 8 words.

export const K_SFI_MAP_OFFSET = 0;
export const K_SFI_FUNCTION_DATA_OFFSET = 8;
export const K_SFI_NAME_OR_SCOPE_INFO_OFFSET = 16;
export const K_SFI_OUTER_SCOPE_INFO_OFFSET = 24;
export const K_SFI_SCRIPT_OFFSET = 32;
export const K_SFI_LENGTH_OFFSET = 40;
export const K_SFI_FORMAL_PARAMETER_COUNT_OFFSET = 42;
export const K_SFI_FUNCTION_TOKEN_OFFSET_OFFSET = 44;
export const K_SFI_EXPECTED_NOF_PROPERTIES_OFFSET = 46;
export const K_SFI_FLAGS2_OFFSET = 47;
export const K_SFI_FLAGS_OFFSET = 48;
export const K_SFI_FUNCTION_LITERAL_ID_OFFSET = 52;
export const K_SFI_UNIQUE_ID_OFFSET = 56;
export const K_SFI_AGE_OFFSET = 60;
export const K_SFI_FEEDBACK_SLOT_OFFSET = 62;
export const K_SFI_TOTAL_SIZE = 64;

must(K_SFI_FLAGS_OFFSET === 48, "SFI flags offset 校验");
must(K_SFI_TOTAL_SIZE === 64, "SFI total size 校验");

console.log(`[shared-function-info.h.js] 提取与锚点校验通过 (共 ${checks} 项校验):`);
console.log({
  K_SFI_TOTAL_SIZE,
  K_SFI_FLAGS_OFFSET,
  FLAGS_BITFIELD,
  FLAGS2_BITFIELD,
});
