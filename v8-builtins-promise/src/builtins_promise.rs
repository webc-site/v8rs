// 由 tool/src/builtins/builtins-promise.js 从 v8/src/builtins/builtins-promise.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-promise; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 0 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[];

/// 原版 enum PromiseResolvingFunctionContextSlot 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromiseResolvingFunctionContextSlot {
  /// kPromiseIfNotResolvedSlot = Context::MIN_CONTEXT_SLOTS
  PromiseIfNotResolvedSlot,
  /// kDebugEventSlot
  DebugEventSlot,
  /// kPromiseContextLength
  PromiseContextLength,
}

/// 原版 enum PromiseAllResolveElementContextSlots 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromiseAllResolveElementContextSlots {
  /// kPromiseAllResolveElementRemainingSlot = Context::MIN_CONTEXT_SLOTS
  PromiseAllResolveElementRemainingSlot,
  /// kPromiseAllResolveElementCapabilitySlot
  PromiseAllResolveElementCapabilitySlot,
  /// kPromiseAllResolveElementValuesSlot
  PromiseAllResolveElementValuesSlot,
  /// kPromiseAllResolveElementLength
  PromiseAllResolveElementLength,
}

/// 原版 enum PromiseAnyRejectElementContextSlots 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromiseAnyRejectElementContextSlots {
  /// kPromiseAnyRejectElementRemainingSlot = Context::MIN_CONTEXT_SLOTS
  PromiseAnyRejectElementRemainingSlot,
  /// kPromiseAnyRejectElementCapabilitySlot
  PromiseAnyRejectElementCapabilitySlot,
  /// kPromiseAnyRejectElementErrorsSlot
  PromiseAnyRejectElementErrorsSlot,
  /// kPromiseAnyRejectElementLength
  PromiseAnyRejectElementLength,
}

/// 原版 enum FunctionContextSlot 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionContextSlot {
  /// kCapabilitySlot = Context::MIN_CONTEXT_SLOTS
  CapabilitySlot,
  /// kCapabilitiesContextLength
  CapabilitiesContextLength,
}

/// 原版 enum PromiseFinallyContextSlot 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromiseFinallyContextSlot {
  /// kOnFinallySlot = Context::MIN_CONTEXT_SLOTS
  OnFinallySlot,
  /// kConstructorSlot
  ConstructorSlot,
  /// kPromiseFinallyContextLength
  PromiseFinallyContextLength,
}

/// 原版 enum PromiseValueThunkOrReasonContextSlot 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromiseValueThunkOrReasonContextSlot {
  /// kValueSlot = Context::MIN_CONTEXT_SLOTS
  ValueSlot,
  /// kPromiseValueThunkOrReasonContextLength
  PromiseValueThunkOrReasonContextLength,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("PromiseBuiltins", "")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 0);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 1);
};
