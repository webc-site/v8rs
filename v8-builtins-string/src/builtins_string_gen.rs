// 由 tool/src/builtins/builtins-string-gen.js 从 v8/src/builtins/builtins-string-gen.cc + src/builtins/builtins-string-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-string-gen.cc, src/builtins/builtins-string-gen.h)
//! 任务映射: v8-builtins-string; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 17 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringAdd_NoMapCheck",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SubString",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringEqual",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringLessThan",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringLessThanOrEqual",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringGreaterThan",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringCompare",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringGreaterThanOrEqual",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmJSStringEqual",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmStringAdd_NoMapCheck",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmStringCompare",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringFromCodePointAt",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringFromCharCode",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringPrototypeReplace",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringPrototypeMatchAll",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringPrototypeSplit",
    assembler: "StringBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StringSubstring",
    assembler: "StringBuiltinsAssembler",
  },
];

/// 原版 enum StringComparison 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringComparison {
  /// kLessThan
  LessThan,
  /// kLessThanOrEqual
  LessThanOrEqual,
  /// kGreaterThan
  GreaterThan,
  /// kGreaterThanOrEqual
  GreaterThanOrEqual,
  /// kCompare
  Compare,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[
  ("StringBuiltinsAssembler", "CodeStubAssembler"),
  ("StringComparison", ""),
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 17);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 2);
};
