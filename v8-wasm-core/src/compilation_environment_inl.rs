// 由 tool/src/wasm/compilation-environment-inl.h.js 从 v8/src/wasm/compilation-environment-inl.h 生成; 修订就地改, 复跑覆盖
// CompilationEnv 从 NativeModule 的构造字段表

//! 编译环境构造规则。

/// ForModule 从 NativeModule 取的四字段 (按构造序)
/// v8: src/wasm/compilation-environment-inl.h CompilationEnv::ForModule
#[rustfmt::skip]
pub static FOR_MODULE_FIELDS: &[&str] = &[
    "module()",
    "enabled_features()",
    "fast_api_data()",
    "coverage_data()",
];

// —— 提取不变式断言 ——
const _: () = {
  use crate::table_eq::str_eq;
  assert!(FOR_MODULE_FIELDS.len() == 4);
  assert!(str_eq(FOR_MODULE_FIELDS[0], "module()"));
  assert!(str_eq(FOR_MODULE_FIELDS[3], "coverage_data()"));
};
