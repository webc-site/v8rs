// 由 tool/src/builtins/builtins-async-function-gen.js 从 v8/src/builtins/builtins-async-function-gen.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-promise; 聚合基线脚本另产同源模块 (并存不覆盖)

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
    macro_name: "TF_BUILTIN",
    name: "AsyncFunctionEnter",
    assembler: "AsyncFunctionBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncFunctionReject",
    assembler: "AsyncFunctionBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncFunctionResolve",
    assembler: "AsyncFunctionBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncFunctionLazyDeoptContinuation",
    assembler: "AsyncFunctionBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncFunctionAwaitRejectClosure",
    assembler: "AsyncFunctionBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncFunctionAwaitResolveClosure",
    assembler: "AsyncFunctionBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncFunctionAwait",
    assembler: "AsyncFunctionBuiltinsAssembler",
  },
];

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] =
  &[("AsyncFunctionBuiltinsAssembler", "AsyncBuiltinsAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 7);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
