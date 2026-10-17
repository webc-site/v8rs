// 由 tool/src/builtins/builtins-wasm-gen.js 从 v8/src/builtins/builtins-wasm-gen.cc + src/builtins/builtins-wasm-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-wasm-gen.cc, src/builtins/builtins-wasm-gen.h)
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 9 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmFloat32ToNumber",
    assembler: "WasmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmFloat64ToNumber",
    assembler: "WasmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmFloat64ToString",
    assembler: "WasmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "JSToWasmLazyDeoptContinuation",
    assembler: "WasmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmToJsWrapperCSA",
    assembler: "WasmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmToJsWrapperInvalidSig",
    assembler: "WasmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmConstructorWrapper",
    assembler: "WasmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmMethodWrapper",
    assembler: "WasmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WasmGetOwnProperty",
    assembler: "WasmBuiltinsAssembler",
  },
];

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("WasmBuiltinsAssembler", "CodeStubAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 9);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
