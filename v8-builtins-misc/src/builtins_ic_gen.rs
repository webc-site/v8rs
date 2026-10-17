// 由 tool/src/builtins/builtins-ic-gen.js 从 v8/src/builtins/builtins-ic-gen.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 0 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[];

/// 平台汇编入口 Builtins::Generate_<名> 清单 (共 71 条)
pub const ASM_GENERATE_ENTRIES: &[&str] = &[
  "LoadIC",
  "LoadIC_Megamorphic",
  "LoadIC_Noninlined",
  "LoadIC_NoFeedback",
  "LoadICTrampoline",
  "LoadICUninitializedBaseline",
  "LoadICInObjectNonDoubleFieldBaseline",
  "LoadICOutOfObjectNonDoubleFieldBaseline",
  "LoadICDoubleFieldBaseline",
  "LoadICConstantFromPrototypeBaseline",
  "LoadICStringLengthBaseline",
  "LoadICGenericBaseline",
  "LoadICTrampoline_Megamorphic",
  "LoadSuperIC",
  "LoadSuperICBaseline",
  "KeyedLoadIC",
  "EnumeratedKeyedLoadIC",
  "EnumeratedKeyedLoadICBaseline",
  "KeyedLoadIC_Megamorphic",
  "KeyedLoadIC_PolymorphicName",
  "KeyedLoadICTrampoline",
  "KeyedLoadICBaseline",
  "KeyedLoadICTrampoline_Megamorphic",
  "LoadGlobalIC_NoFeedback",
  "StoreGlobalIC",
  "StoreGlobalICTrampoline",
  "StoreGlobalICBaseline",
  "StoreIC",
  "StoreIC_Megamorphic",
  "StoreICTrampoline",
  "StoreICTrampoline_Megamorphic",
  "StoreICBaseline",
  "DefineNamedOwnIC",
  "DefineNamedOwnICTrampoline",
  "DefineNamedOwnICBaseline",
  "KeyedStoreIC",
  "KeyedStoreICTrampoline",
  "KeyedStoreICTrampoline_Megamorphic",
  "KeyedStoreICBaseline",
  "DefineKeyedOwnIC",
  "DefineKeyedOwnICTrampoline",
  "DefineKeyedOwnICBaseline",
  "StoreInArrayLiteralIC",
  "StoreInArrayLiteralICBaseline",
  "CloneObjectIC",
  "CloneObjectICBaseline",
  "CloneObjectIC_Slow",
  "KeyedHasIC",
  "KeyedHasICBaseline",
  "KeyedHasIC_Megamorphic",
  "KeyedHasIC_PolymorphicName",
  "LoadGlobalIC",
  "LoadGlobalICInsideTypeof",
  "LoadGlobalICTrampoline",
  "LoadGlobalICInsideTypeofTrampoline",
  "LoadGlobalICBaseline",
  "LoadGlobalICInsideTypeofBaseline",
  "LookupGlobalIC",
  "LookupGlobalICTrampoline",
  "LookupGlobalICBaseline",
  "LookupGlobalICInsideTypeof",
  "LookupGlobalICInsideTypeofTrampoline",
  "LookupGlobalICInsideTypeofBaseline",
  "LookupContextNoCellTrampoline",
  "LookupContextTrampoline",
  "LookupContextNoCellBaseline",
  "LookupScriptContextBaseline",
  "LookupContextNoCellInsideTypeofTrampoline",
  "LookupContextInsideTypeofTrampoline",
  "LookupContextNoCellInsideTypeofBaseline",
  "LookupContextInsideTypeofBaseline",
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 0);
  assert!(ASM_GENERATE_ENTRIES.len() == 71);
};
