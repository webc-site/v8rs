// 由 tool/src/builtins/builtins-async-generator-gen.js 从 v8/src/builtins/builtins-async-generator-gen.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-promise; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 15 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorPrototypeNext",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorPrototypeReturn",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorPrototypeThrow",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorAwaitResolveClosure",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorAwaitRejectClosure",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorAwait",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorResumeNext",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorResolve",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorReject",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorYieldWithAwait",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorYieldWithAwaitResolveClosure",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorReturn",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorReturnResolveClosure",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorReturnClosedResolveClosure",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AsyncGeneratorReturnClosedRejectClosure",
    assembler: "AsyncGeneratorBuiltinsAssembler",
  },
];

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] =
  &[("AsyncGeneratorBuiltinsAssembler", "AsyncBuiltinsAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 15);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
