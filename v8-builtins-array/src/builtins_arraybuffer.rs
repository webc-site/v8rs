// 由 tool/src/builtins/builtins-arraybuffer.js 从 v8/src/builtins/builtins-arraybuffer.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-array; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 11 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ArrayBufferConstructor",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ArrayBufferConstructor_DoNotInitialize",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "SharedArrayBufferPrototypeSlice",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ArrayBufferPrototypeSlice",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "SharedArrayBufferPrototypeGetByteLength",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ArrayBufferPrototypeResize",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ArrayBufferPrototypeTransfer",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ArrayBufferPrototypeTransferToFixedLength",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "SharedArrayBufferPrototypeGrow",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ArrayBufferPrototypeTransferToImmutable",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ArrayBufferPrototypeSliceToImmutable",
    assembler: "",
  },
];

/// 原版 enum PreserveResizability 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreserveResizability {
  /// kToFixedLength
  ToFixedLength,
  /// kPreserveResizability
  PreserveResizability,
  /// kToImmutable
  ToImmutable,
}

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 11);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 0);
};
