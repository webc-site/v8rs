// 由 tool/src/builtins/builtins-console.js 从 v8/src/builtins/builtins-console.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 7 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "Console",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "Console",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ConsoleTime",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ConsoleTimeEnd",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ConsoleTimeLog",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ConsoleTimeStamp",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ConsoleContext",
    assembler: "",
  },
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 7);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
