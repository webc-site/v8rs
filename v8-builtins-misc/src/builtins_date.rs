// 由 tool/src/builtins/builtins-date.js 从 v8/src/builtins/builtins-date.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 31 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateNow",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateParse",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DateUTC",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetDate",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetFullYear",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetHours",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetMilliseconds",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetMinutes",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetMonth",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetSeconds",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetTime",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetUTCDate",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetUTCFullYear",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetUTCHours",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetUTCMilliseconds",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetUTCMinutes",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetUTCMonth",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetUTCSeconds",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeToDateString",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeToISOString",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeToString",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeToTimeString",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeToLocaleDateString",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeToLocaleString",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeToLocaleTimeString",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeToUTCString",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeGetYear",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeSetYear",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeToJson",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "DatePrototypeToTemporalInstant",
    assembler: "",
  },
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 31);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
