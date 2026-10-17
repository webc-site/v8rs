// 由 tool/src/codegen/x64/assembler-x64.cc.js 从 v8/src/codegen/x64/assembler-x64.{cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// x64 汇编器骨架

// 禁覆写面登记: 手写 assembler.rs / operand.rs 保持权威, 本文件仅登记规则面。
// 语义面: 指令发射器 (cc 5547 行) 登记跳过。

/// 编译器基线 CPU 特征 (原 CpuFeaturesFromCompiler):
/// - __SAHF__ 编译宏触发
/// - __SSE3__ 编译宏触发
/// - __SSSE3__ 编译宏触发
/// - __SSE4_1__ 编译宏触发
/// - __SSE4_2__ 编译宏触发
/// - __AVX__ 编译宏触发
/// - __AVX2__ 编译宏触发
/// - __AVX_VNNI__ 编译宏触发
/// - __AVX_VNNI_INT8__ 编译宏触发
/// - __FMA3__ 编译宏触发
/// - __F16C__ 编译宏触发
/// - __BMI1__ 编译宏触发
/// - __BMI2__ 编译宏触发
/// - __LZCNT__ 编译宏触发
/// - __POPCNT__ 编译宏触发
/// - __APX_F__ 编译宏触发
/// - __AVX10_1__ 编译宏触发
pub const BASELINE_CPU_FEATURES_FROM_COMPILER: &[&str] = &["SAHF", "SSE3", "SSSE3", "SSE4_1", "SSE4_2", "AVX", "AVX2", "AVX_VNNI", "AVX_VNNI_INT8", "FMA3", "F16C", "BMI1", "BMI2", "LZCNT", "POPCNT", "APX_F", "AVX10_1"];

/// x64 无常量池 (原 kConstantPoolSize)
pub const CONSTANT_POOL_SIZE: i32 = 0;

/// inl 内联方法面 (65 条):
// - SupportsOptimizer
// - emit_rex_64
// - emit
// - DCHECK_EQ
// - emit_rex_32
// - emit_optional_rex_32
// - emit_optional_rex_8
// - emit_rex2_prefix
// - emit_rex2_64
// - emit_rex2_32
// - emit_legacy_extended_evex_prefix
// - emit_evex_byte0
// - emit_legacy_extended_evex_byte1
// - emit_legacy_extended_evex_byte2
// - emit_legacy_extended_evex_byte3
// - emit_vex3_byte1
// - emit_vex2_byte1
// - emit_vex3_byte2
// - emit_vex_prefix
// - emit_vex3_byte0
// - emit_vex2_byte0
// - emit_evex_byte1
// - emit_evex_byte2
// - emit_evex_byte3
// - emit_evex_prefix
// - target_address_at
// - set_target_address_at
// - WriteUnalignedValue
// - FlushInstructionCache
// - relative_target_offset
// - DCHECK
// - deserialization_set_target_internal_reference_at
// - deserialization_special_target_size
// - code_target_object_handle_at
// - GetCodeTarget
// - compressed_embedded_object_handle_at
// - GetEmbeddedObject
// - target_builtin_at
// - uint32_constant_at
// - set_uint32_constant_at
// - apply
// - IsWasmStubCall
// - target_address
// - IsWasmCall
// - target_address_address
// - IsFullEmbeddedObject
// - IsExternalReference
// - constant_pool_entry_address
// - target_address_size
// - IsCompressedEmbeddedObject
// - target_object
// - obj
// - target_object_handle
// - target_external_reference
// - set_target_external_reference
// - wasm_code_pointer
// - set_wasm_code_pointer
// - wasm_code_pointer_table_entry
// - set_wasm_code_pointer_table_entry
// - target_internal_reference
// - target_internal_reference_address
// - js_dispatch_handle
// - set_target_object
// - DCHECK_IMPLIES
// - target_off_heap_target

pub struct AssemblerX64Inl;

// TODO(dev): 依手写 assembler.rs 分批补发射器。

// —— 提取不变式断言 ——
const _: () = {
  assert!(CONSTANT_POOL_SIZE == 0);
};
