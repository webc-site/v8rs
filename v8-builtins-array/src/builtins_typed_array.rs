// 由 tool/src/builtins/builtins-typed-array.js 从 v8/src/builtins/builtins-typed-array.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-array; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 13 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "TypedArrayPrototypeBuffer",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "TypedArrayPrototypeCopyWithin",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "TypedArrayPrototypeFill",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "TypedArrayPrototypeIncludes",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "TypedArrayPrototypeIndexOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "TypedArrayPrototypeLastIndexOf",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "TypedArrayPrototypeReverse",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "Uint8ArrayFromBase64",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "Uint8ArrayPrototypeSetFromBase64",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "Uint8ArrayPrototypeToBase64",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "Uint8ArrayFromHex",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "Uint8ArrayPrototypeSetFromHex",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "Uint8ArrayPrototypeToHex",
    assembler: "",
  },
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 13);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
