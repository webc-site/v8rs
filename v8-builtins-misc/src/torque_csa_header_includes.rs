// 由 tool/src/builtins/torque-csa-header-includes.js 从 v8/src/builtins/torque-csa-header-includes.h 生成; 修订就地改, 复跑覆盖
// include 依赖登记 (Torque 生成 CSA 头的公共前置)
//! 任务映射: v8-builtins-misc; 聚合基线另归 v8-builtins-core (并存不覆盖)

/// 原版 include 清单 (顺序保持)
pub const INCLUDES: &[&str] = &[
  "src/builtins/builtins-promise.h",
  "src/builtins/builtins-proxy-gen.h",
  "src/codegen/code-stub-assembler.h",
  "src/compiler/code-assembler.h",
  "src/utils/utils.h",
  "torque-generated/csa-types.h",
];

/// 该头本身无规则代码 (仅 include 聚合), 迁移语义为零
pub const RULE_CODE_ITEMS: usize = 0;

// —— 对数断言 ——
const _: () = {
  assert!(INCLUDES.len() == 6);
  assert!(INCLUDES[0] == "src/builtins/builtins-promise.h");
};
