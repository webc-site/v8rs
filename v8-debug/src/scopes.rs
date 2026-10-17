// 由 tool/src/debug/scopes.js 生成
// 作用域迭代组

use strum::{Display, EnumIter, EnumString};

/// 作用域类型枚举 (来源: debug-scopes.h:22, 10 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum ScopeType {
  Global,
  Local,
  With,
  Closure,
  Catch,
  Block,
  Script,
  Eval,
  Module,
  WasmExpressionStack,
}

impl ScopeType {
  pub const fn is_global(&self) -> bool {
    matches!(self, Self::Global)
  }

  pub const fn is_local(&self) -> bool {
    matches!(self, Self::Local)
  }

  pub const fn is_closure(&self) -> bool {
    matches!(self, Self::Closure)
  }

  pub const fn is_module(&self) -> bool {
    matches!(self, Self::Module)
  }

  pub const fn is_wasm(&self) -> bool {
    matches!(self, Self::WasmExpressionStack)
  }
}

// ── ScopeDetails 索引常量 (debug-scopes.h:34-39, size=6) ──
pub const K_SCOPE_DETAILS_TYPE_INDEX: usize = 0;
pub const K_SCOPE_DETAILS_OBJECT_INDEX: usize = 1;
pub const K_SCOPE_DETAILS_NAME_INDEX: usize = 2;
pub const K_SCOPE_DETAILS_START_POSITION_INDEX: usize = 3;
pub const K_SCOPE_DETAILS_END_POSITION_INDEX: usize = 4;
pub const K_SCOPE_DETAILS_FUNCTION_INDEX: usize = 5;
pub const K_SCOPE_DETAILS_SIZE: usize = 6;

/// 作用域迭代器模式 (来源: debug-scopes.h:31)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum ScopeIteratorMode {
  #[strum(serialize = "STACK")]
  Stack,
  #[strum(serialize = "ALL")]
  All,
}

/// 作用域变量可用性信息 (来源: debug-scopes.h:32)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum VariableInfo {
  #[strum(serialize = "kEmpty")]
  Empty,
  #[strum(serialize = "kAllUnavailable")]
  AllUnavailable,
  #[strum(serialize = "kAvailable")]
  Available,
}

/// 属性迭代器枚举阶段 (来源: debug-property-iterator.h:39)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[repr(u8)]
pub enum PropertyIteratorPhase {
  #[strum(serialize = "kExoticIndices")]
  ExoticIndices = 0,
  #[strum(serialize = "kEnumerableStrings")]
  EnumerableStrings = 1,
  #[strum(serialize = "kAllProperties")]
  AllProperties = 2,
}
