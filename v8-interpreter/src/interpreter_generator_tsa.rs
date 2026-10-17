// 由 tool/src/interpreter/interpreter-generator-tsa.h.js 从 v8/src/interpreter/interpreter-generator-tsa.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 头文件为纯引入 (builtin-compiler.h), 无声明面。
/// cc 侧 (347 行) 为 Turboshaft-based handler 生成分支, 语义修订阶段处理。
pub const TSA_REGISTRATION: &str =
  "interpreter-generator-tsa.cc: TSA 分支 handler 生成 (登记不展开)";

// —— 提取不变式断言 ——
const _: () = {
  assert!(!TSA_REGISTRATION.is_empty());
};
