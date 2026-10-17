// 由 tool/src/codegen/unoptimized-compilation-info.h.js 从 v8/src/codegen/unoptimized-compilation-info.{h,cc} 生成; 修订就地改, 复跑覆盖
// 非优化编译信息骨架

/// 原 class UnoptimizedCompilationInfo (字节码编译期信息: literal/zone/
/// source_range_map/coverageinfo 槽位) 方法面 (22 条):
// - UnoptimizedCompilationInfo
// - flags
// - dispatcher
// - character_stream
// - literal
// - set_literal
// - DCHECK_NOT_NULL
// - ClearLiteral
// - scope
// - num_parameters
// - num_parameters_including_this
// - SourcePositionRecordingMode
// - has_source_range_map
// - source_range_map
// - set_source_range_map
// - has_coverage_info
// - coverage_info
// - set_coverage_info
// - has_bytecode_array
// - bytecode_array
// - SetBytecodeArray
// - feedback_vector_spec

pub struct UnoptimizedCompilationInfo;

// TODO(dev): 补字段与 set/clear 生命周期。

// —— 提取不变式断言 ——
const _: () = {};
