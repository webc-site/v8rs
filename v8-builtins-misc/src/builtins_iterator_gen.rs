// 由 tool/src/builtins/builtins-iterator-gen.js 从 v8/src/builtins/builtins-iterator-gen.cc + src/builtins/builtins-iterator-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-iterator-gen.cc, src/builtins/builtins-iterator-gen.h)
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 14 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "IterableToList",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "IterableToFixedArray",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "IterableToFixedArrayForWasm",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringListFromIterable",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "IterableToListMayPreserveHoles",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "IterableToListConvertHoles",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "IterableToListWithSymbolLookup",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "GetIteratorWithFeedbackLazyDeoptContinuation",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallIteratorWithFeedbackLazyDeoptContinuation",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ForOfNextResultDeoptContinuation",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ForOfNextLoadDoneLazyDeoptContinuation",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ForOfNextLoadValueEagerDeoptContinuation",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ForOfNextLoadValueLazyDeoptContinuation",
    assembler: "IteratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "IterableToFixedArrayWithSymbolLookupSlow",
    assembler: "IteratorBuiltinsAssembler",
  },
];

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("IteratorBuiltinsAssembler", "CodeStubAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 14);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
