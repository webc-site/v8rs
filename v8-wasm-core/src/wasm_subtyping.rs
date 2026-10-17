// 由 tool/src/wasm/wasm-subtyping.h.js 从 v8/src/wasm/wasm-subtyping.{h,cc} 生成; 修订就地改, 复跑覆盖
// 子类型判定 API 签名骨架
// 深水推迟 (W10WC 台账): ValidSubtypeDefinition/IsSubtypeOfImpl/HeapTypesUnrelated/IsSubtypeOf/
// Union/Intersection/IsSameTypeHierarchy/EquivalentTypes 函数族依赖 struct types 完整面 +
// TypeCanonicalizer + CanonicalGroup, 前置: v8-wasm-objects 类型面与 canonicalizer 语义轮;
// 本轮仅挂载纯数据件 TypeInModule/ModuleRef, 骨架函数已移除待补波 1:1 填体。

use crate::value_type::ValueType;

/// 带所属模块的类型 (对应 struct TypeInModule)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeInModule {
  pub type_: ValueType,
  pub module: ModuleRef,
}

/// 模块引用骨架 (原版为 const WasmModule*)
pub type ModuleRef = usize;

impl TypeInModule {
  /// 对应 TypeInModule(type, module)
  #[must_use]
  pub const fn new(type_: ValueType, module: ModuleRef) -> Self {
    Self { type_, module }
  }
}
