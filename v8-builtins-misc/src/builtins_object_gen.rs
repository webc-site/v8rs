// 由 tool/src/builtins/builtins-object-gen.js 从 v8/src/builtins/builtins-object-gen.cc + src/builtins/builtins-object-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-object-gen.cc, src/builtins/builtins-object-gen.h)
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 20 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectPrototypeHasOwnProperty",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectAssign",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectKeys",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectHasOwn",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectGetOwnPropertyNames",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectValues",
    assembler: "ObjectEntriesValuesBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectEntries",
    assembler: "ObjectEntriesValuesBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectPrototypeIsPrototypeOf",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectToString",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectCreate",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectIs",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CreateIterResultObject",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "HasProperty",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "InstanceOf",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "InstanceOf_WithFeedback",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "InstanceOf_Baseline",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OrdinaryHasInstance",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CreateGeneratorObject",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OrdinaryGetOwnPropertyDescriptor",
    assembler: "ObjectBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ObjectGetOwnPropertyDescriptor",
    assembler: "ObjectBuiltinsAssembler",
  },
];

/// 原版 enum CollectType 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectType {
  /// kEntries
  Entries,
  /// kValues
  Values,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[
  (
    "ObjectEntriesValuesBuiltinsAssembler",
    "ObjectBuiltinsAssembler",
  ),
  ("ObjectBuiltinsAssembler", "CodeStubAssembler"),
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 20);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 2);
};
