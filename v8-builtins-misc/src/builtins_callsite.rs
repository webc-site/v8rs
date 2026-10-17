// 由 tool/src/builtins/builtins-callsite.js 从 v8/src/builtins/builtins-callsite.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 22 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetColumnNumber",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetEnclosingColumnNumber",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetEnclosingLineNumber",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetEvalOrigin",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetFileName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetFunction",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetFunctionName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetLineNumber",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetMethodName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetPosition",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetPromiseIndex",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetScriptHash",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetScriptNameOrSourceURL",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetThis",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeGetTypeName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeIsAsync",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeIsConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeIsEval",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeIsNative",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeIsPromiseAll",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeIsToplevel",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "CallSitePrototypeToString",
    assembler: "",
  },
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 22);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
