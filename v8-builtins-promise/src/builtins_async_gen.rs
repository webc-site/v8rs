// 由 tool/src/builtins/builtins-async-gen.js 从 v8/src/builtins/builtins-async-gen.cc + src/builtins/builtins-async-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-async-gen.cc, src/builtins/builtins-async-gen.h)
//! 任务映射: v8-builtins-promise; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 1 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[CcBuiltin {
  macro_name: "TF_BUILTIN",
  name: "AsyncIteratorValueUnwrap",
  assembler: "AsyncBuiltinsAssembler",
}];

/// 原版 enum Fields 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fields {
  /// kDoneSlot = Context::MIN_CONTEXT_SLOTS
  DoneSlot,
  /// kLength
  Length,
}

/// 原版 enum AwaitBehavior : bool 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AwaitBehavior {
  /// kNormal = false
  Normal,
  /// kResumeOnly = true
  ResumeOnly,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[
  ("ValueUnwrapContext", ""),
  ("AsyncBuiltinsAssembler", "PromiseBuiltinsAssembler"),
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 1);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 2);
};
