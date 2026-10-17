// 由 tool/src/builtins/builtins-string-tsa.js 从 v8/src/builtins/builtins-string-tsa-inl.h + src/builtins/builtins-string-tsa.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-string-tsa-inl.h, src/builtins/builtins-string-tsa.cc)
//! 任务映射: v8-builtins-string; 聚合基线脚本另产同源模块 (并存不覆盖)

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
    macro_name: "TS_BUILTIN",
    name: "StringFromCodePointAt",
    assembler: "StringBuiltinsAssemblerTS",
  },
  CcBuiltin {
    macro_name: "TS_BUILTIN",
    name: "StringFromCharCode",
    assembler: "StringBuiltinsAssemblerTS",
  },
  CcBuiltin {
    macro_name: "TS_BUILTIN",
    name: "ToString",
    assembler: "StringBuiltinsAssemblerTS",
  },
];

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("StringBuiltinsReducer", "Next")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 3);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
