// 由 tool/src/builtins/builtins-reflect.js 从 v8/src/builtins/builtins-reflect.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-proxy-reflect; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 3 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ReflectDefineProperty",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ReflectOwnKeys",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ReflectSet",
    assembler: "",
  },
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 3);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
