// 由 tool/src/builtins/builtins-object.js 从 v8/src/builtins/builtins-object.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

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
    macro_name: "BUILTIN",
    name: "ObjectPrototypePropertyIsEnumerable",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectDefineProperties",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectDefineProperty",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectDefineGetter",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectDefineSetter",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectLookupGetter",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectLookupSetter",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectFreeze",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectPrototypeGetProto",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectPrototypeSetProto",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectGetOwnPropertySymbols",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectIsFrozen",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectIsSealed",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectGetOwnPropertyDescriptors",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "BUILTIN",
    name: "ObjectSeal",
    assembler: "",
  },
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 15);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
