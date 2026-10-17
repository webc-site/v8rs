// 由 tool/src/builtins/builtins-internal-gen.js 从 v8/src/builtins/builtins-internal-gen.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 70 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CopyFastSmiOrObjectElements",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "GrowFastDoubleElements",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "GrowFastSmiOrObjectElements",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ReturnReceiver",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DebugBreakTrampoline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "RecordWriteSaveFP",
    assembler: "WriteBarrierCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "RecordWriteIgnoreFP",
    assembler: "WriteBarrierCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "IndirectPointerBarrierSaveFP",
    assembler: "WriteBarrierCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "IndirectPointerBarrierIgnoreFP",
    assembler: "WriteBarrierCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "EphemeronKeyBarrierSaveFP",
    assembler: "WriteBarrierCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "EphemeronKeyBarrierIgnoreFP",
    assembler: "WriteBarrierCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedStore8IgnoreFP",
    assembler: "TSANRelaxedStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedStore8SaveFP",
    assembler: "TSANRelaxedStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedStore16IgnoreFP",
    assembler: "TSANRelaxedStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedStore16SaveFP",
    assembler: "TSANRelaxedStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedStore32IgnoreFP",
    assembler: "TSANRelaxedStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedStore32SaveFP",
    assembler: "TSANRelaxedStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedStore64IgnoreFP",
    assembler: "TSANRelaxedStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedStore64SaveFP",
    assembler: "TSANRelaxedStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANSeqCstStore8IgnoreFP",
    assembler: "TSANSeqCstStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANSeqCstStore8SaveFP",
    assembler: "TSANSeqCstStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANSeqCstStore16IgnoreFP",
    assembler: "TSANSeqCstStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANSeqCstStore16SaveFP",
    assembler: "TSANSeqCstStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANSeqCstStore32IgnoreFP",
    assembler: "TSANSeqCstStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANSeqCstStore32SaveFP",
    assembler: "TSANSeqCstStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANSeqCstStore64IgnoreFP",
    assembler: "TSANSeqCstStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANSeqCstStore64SaveFP",
    assembler: "TSANSeqCstStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANReleaseStore8IgnoreFP",
    assembler: "TSANReleaseStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANReleaseStore8SaveFP",
    assembler: "TSANReleaseStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANReleaseStore16IgnoreFP",
    assembler: "TSANReleaseStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANReleaseStore16SaveFP",
    assembler: "TSANReleaseStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANReleaseStore32IgnoreFP",
    assembler: "TSANReleaseStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANReleaseStore32SaveFP",
    assembler: "TSANReleaseStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANReleaseStore64IgnoreFP",
    assembler: "TSANReleaseStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANReleaseStore64SaveFP",
    assembler: "TSANReleaseStoreCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedLoad32IgnoreFP",
    assembler: "TSANRelaxedLoadCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedLoad32SaveFP",
    assembler: "TSANRelaxedLoadCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedLoad64IgnoreFP",
    assembler: "TSANRelaxedLoadCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TSANRelaxedLoad64SaveFP",
    assembler: "TSANRelaxedLoadCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DeleteProperty",
    assembler: "DeletePropertyBaseAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CopyDataPropertiesWithExcludedPropertiesOnStack",
    assembler: "SetOrCopyDataPropertiesAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CopyDataPropertiesWithExcludedProperties",
    assembler: "SetOrCopyDataPropertiesAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CopyDataProperties",
    assembler: "SetOrCopyDataPropertiesAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetDataProperties",
    assembler: "SetOrCopyDataPropertiesAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ForInEnumerate",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ForInPrepare",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ForInFilter",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SameValue",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SameValueNumbersOnly",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AdaptorWithBuiltinExitFrame0",
    assembler: "CppBuiltinsAdaptorAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AdaptorWithBuiltinExitFrame1",
    assembler: "CppBuiltinsAdaptorAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AdaptorWithBuiltinExitFrame2",
    assembler: "CppBuiltinsAdaptorAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AdaptorWithBuiltinExitFrame3",
    assembler: "CppBuiltinsAdaptorAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AdaptorWithBuiltinExitFrame4",
    assembler: "CppBuiltinsAdaptorAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AdaptorWithBuiltinExitFrame5",
    assembler: "CppBuiltinsAdaptorAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "NewHeapNumber",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AllocateInYoungGeneration",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AllocateInOldGeneration",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmAllocateInYoungGeneration",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmAllocateInOldGeneration",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmAllocateInSharedHeap",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Abort",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AbortCSADcheck",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "GetProperty",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "GetPropertyWithReceiver",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetProperty",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CreateDataProperty",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "FindNonDefaultConstructorOrConstruct",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "GetOwnPropertyDescriptor",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CheckMaglevType",
    assembler: "CodeStubAssembler",
  },
];

/// 平台汇编入口 Builtins::Generate_<名> 清单 (共 11 条)
pub const ASM_GENERATE_ENTRIES: &[&str] = &[
  "CEntry_Return1_ArgvOnStack_NoBuiltinExit",
  "CEntry_Return1_ArgvOnStack_BuiltinExit",
  "CEntry_Return1_ArgvInRegister_NoBuiltinExit",
  "CEntry_Return2_ArgvOnStack_NoBuiltinExit",
  "CEntry_Return2_ArgvInRegister_NoBuiltinExit",
  "WasmCEntry",
  "BaselineLeaveFrame",
  "MaglevOptimizeCodeOrTailCallOptimizedCodeSlot",
  "MaglevFunctionEntryStackCheck",
  "MaglevFunctionEntryStackCheck_WithoutNewTarget",
  "MaglevFunctionEntryStackCheck_WithNewTarget",
];

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[
  ("WriteBarrierCodeStubAssembler", "CodeStubAssembler"),
  ("TSANRelaxedStoreCodeStubAssembler", "CodeStubAssembler"),
  ("TSANSeqCstStoreCodeStubAssembler", "CodeStubAssembler"),
  ("TSANReleaseStoreCodeStubAssembler", "CodeStubAssembler"),
  ("TSANRelaxedLoadCodeStubAssembler", "CodeStubAssembler"),
  ("DeletePropertyBaseAssembler", "AccessorAssembler"),
  ("SetOrCopyDataPropertiesAssembler", "CodeStubAssembler"),
  ("CppBuiltinsAdaptorAssembler", "CodeStubAssembler"),
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 70);
  assert!(ASM_GENERATE_ENTRIES.len() == 11);
};
