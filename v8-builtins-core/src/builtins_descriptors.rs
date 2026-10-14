// 由 tool/src/builtins/builtins-descriptors.js 从 v8/src/builtins/builtins-descriptors.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-core; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 0 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[];

/// 原版 enum ParameterIndices 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterIndices {}

/// 原版 enum ParameterIndices 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterIndices {}

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 0);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 0);
};
