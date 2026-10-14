// 由 tool/src/builtins/builtins-collections-gen.js 从 v8/src/builtins/builtins-collections-gen.cc + src/builtins/builtins-collections-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-collections-gen.cc, src/builtins/builtins-collections-gen.h)
//! 任务映射: v8-builtins-collections; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 41 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapConstructor",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetConstructor",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapIteratorToList",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetOrSetIteratorToList",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OrderedHashTableHealIndex",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeGet",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeHas",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeSet",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeGetOrInsert",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeGetOrInsertComputed",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeDelete",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetPrototypeAdd",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetPrototypeDelete",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeEntries",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeGetSize",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeForEach",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeKeys",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapPrototypeValues",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "MapIteratorPrototypeNext",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetPrototypeHas",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetPrototypeEntries",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetPrototypeGetSize",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetPrototypeForEach",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetPrototypeValues",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "SetIteratorPrototypeNext",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "FindOrderedHashMapEntry",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "FindOrderedHashSetEntry",
    assembler: "CollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakMapConstructor",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakSetConstructor",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakMapLookupHashIndex",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakMapPrototypeGet",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakMapPrototypeHas",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakCollectionDelete",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakCollectionSet",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakMapPrototypeDelete",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakMapPrototypeSet",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakMapPrototypeGetOrInsert",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakMapPrototypeGetOrInsertComputed",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakSetPrototypeAdd",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakSetPrototypeDelete",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "WeakSetPrototypeHas",
    assembler: "WeakCollectionsBuiltinsAssembler",
  },
];

/// 原版 enum Mode 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
  /// kSlow
  Slow,
  /// kFastJSArray
  FastJSArray,
  /// kFastCollection
  FastCollection,
}

/// 原版 enum Variant 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variant {
  /// kMap
  Map,
  /// kSet
  Set,
  /// kWeakMap
  WeakMap,
  /// kWeakSet
  WeakSet,
}

/// 原版 enum GroupByCoercionMode 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroupByCoercionMode {
  /// kZero
  Zero,
  /// kProperty
  Property,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[
  ("BaseCollectionsAssembler", "CodeStubAssembler"),
  ("CollectionsBuiltinsAssembler", "BaseCollectionsAssembler"),
  (
    "WeakCollectionsBuiltinsAssembler",
    "BaseCollectionsAssembler",
  ),
  ("GroupByCoercionMode", ""),
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 41);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 4);
};
