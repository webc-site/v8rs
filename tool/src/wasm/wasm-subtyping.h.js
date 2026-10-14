#!/usr/bin/env bun
// tool/src/wasm/wasm-subtyping.h.js
// 机械提取 v8/src/wasm/wasm-subtyping.{h,cc}: 子类型判定 API 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-subtyping.h");
const ccSrc = readV8("src/wasm/wasm-subtyping.cc");

// 头侧公开函数清单
const decls = [...hSrc.matchAll(
  /^(?:V8_EXPORT_PRIVATE\s+)?(?:bool|TypeInModule|void|size_t|uint32_t)\s+(\w+)\(/gm,
)].map((m) => m[1]);
const REQUIRED = ["ValidSubtypeDefinition", "IsSubtypeOfImpl", "Union", "Intersection", "IsSameTypeHierarchy", "EquivalentTypes", "HeapTypesUnrelated"];
for (const r of REQUIRED) {
  if (!decls.includes(r) && !hSrc.includes(r)) throw new Error(`声明缺失: ${r}`);
}
// TypeInModule 结构锚点
if (!hSrc.includes("struct TypeInModule")) throw new Error("未找到 TypeInModule");
// cc 侧锚点
if (!ccSrc.includes("EquivalentIndices") || !ccSrc.includes("ComputeIsSubtype")) {
  throw new Error("cc 锚点缺失: EquivalentIndices / ComputeIsSubtype");
}

console.log(`头侧声明 ${decls.length} 个: ${decls.join(", ")}`);

let rust = banner(
  "src/wasm/wasm-subtyping.h.js",
  "src/wasm/wasm-subtyping.{h,cc}",
  "子类型判定 API 签名骨架",
);

rust += `
use crate::value_type::{ModuleTypeIndex, ValueType};

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

/// 类型定义合法性: subtype_index 是否可为 supertype 的子类型
/// (对应 ValidSubtypeDefinition, 骨架)
pub fn valid_subtype_definition(
  _subtype_index: ModuleTypeIndex,
  _supertype_index: ModuleTypeIndex,
  _subtyp_module: ModuleRef,
  _super_module: ModuleRef,
) -> bool {
  unimplemented!("骨架: 参照原版 wasm-subtyping.cc ValidSubtypeDefinition 修订")
}

/// 慢路径子类型判定 (对应 IsSubtypeOfImpl 三重载, 骨架)
pub fn is_subtype_of_impl(
  _subtype: TypeInModule,
  _supertype: TypeInModule,
) -> bool {
  unimplemented!("骨架: 参照原版 IsSubtypeOfImpl (ValueType/HeapType/CanonicalGroup 重载) 修订")
}

/// 堆类型不相关判定 (对应 HeapTypesUnrelated = 互不为子类型)
#[inline]
pub fn heap_types_unrelated(_heap1: TypeInModule, _heap2: TypeInModule) -> bool {
  unimplemented!("骨架: !IsHeapSubtypeOf(a,b) && !IsHeapSubtypeOf(b,a)")
}

/// 子类型判定 (对应 IsSubtypeOf 多态族, 骨架)
pub fn is_subtype_of(
  _subtype: TypeInModule,
  _supertype: TypeInModule,
) -> bool {
  unimplemented!("骨架: 参照原版 IsSubtypeOf 修订")
}

/// 最小公共超类型 (对应 Union, 骨架)
pub fn union(_type1: TypeInModule, _type2: TypeInModule) -> TypeInModule {
  unimplemented!("骨架: 参照原版 Union 修订")
}

/// 最大公共子类型 (对应 Intersection, 骨架)
pub fn intersection(_type1: TypeInModule, _type2: TypeInModule) -> TypeInModule {
  unimplemented!("骨架: 参照原版 Intersection 修订")
}

/// 同层判定 (对应 IsSameTypeHierarchy, 骨架)
pub fn is_same_type_hierarchy(_type1: TypeInModule, _type2: TypeInModule) -> bool {
  unimplemented!("骨架: 参照原版 IsSameTypeHierarchy 修订")
}

/// 类型等价 (对应 EquivalentTypes, 骨架)
pub fn equivalent_types(_type1: TypeInModule, _type2: TypeInModule) -> bool {
  unimplemented!("骨架: 参照原版 EquivalentTypes 修订")
}
`;
;

const p = writeSrc("v8-wasm-core", "wasm_subtyping.rs", rust);
console.log(`written ${p}`);
