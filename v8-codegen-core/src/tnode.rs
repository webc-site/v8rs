// 由 tool/src/codegen/tnode.h.js 从 v8/src/codegen/tnode.{h,cc} 生成; 修订就地改, 复跑覆盖
// TNode 类型层级表 (phantom 类型 → 机器表示映射)

// 三元组: (类型名, 直接基类, MachineType 构造名; 空串 = 无对应项)。
// 原版以 C++ 继承 + 幻影类型承载静态机器表示; Rust 侧由 Developer 定型为
// 枚举或 marker trait, 本表保证层级与表示映射 1:1。

/// TNode 类型层级表 (按原版声明顺序)
pub const TNODE_TYPE_HIERARCHY: &[(&str, &str, &str)] = &[
  ("UntaggedT", "", ""),
  ("IntegralT", "UntaggedT", ""),
  ("WordT", "IntegralT", ""),
  ("RawPtrT", "WordT", "Pointer"),
  ("SandboxedPtrT", "WordT", "SandboxedPointer"),
  ("RawPtr", "RawPtrT", ""),
  ("Word32T", "IntegralT", ""),
  ("Int32T", "Word32T", "Int32"),
  ("Uint32T", "Word32T", "Uint32"),
  ("Int16T", "Int32T", "Int16"),
  ("Uint16T", "Uint32T, Int32T", "Uint16"),
  ("Int8T", "Int16T", "Int8"),
  ("Uint8T", "Uint16T, Int16T", "Uint8"),
  ("Word64T", "IntegralT", ""),
  ("AdditiveSafeIntegerT", "Word64T", "Int64"),
  ("Int64T", "Word64T", "Int64"),
  ("Uint64T", "Word64T", "Uint64"),
  ("IntPtrT", "WordT", "IntPtr"),
  ("UintPtrT", "WordT", "UintPtr"),
  ("ExternalPointerHandleT", "Uint32T", "Uint32"),
  ("CppHeapPointerHandleT", "Uint32T", "Uint32"),
  ("IndirectPointerHandleT", "Uint32T", "Uint32"),
  ("JSDispatchHandleT", "Uint32T", "Uint32"),
  ("ExternalPointerT", "Uint32T", "Uint32"),
  ("ExternalPointerT", "UntaggedT", "Pointer"),
  ("CppHeapPointerT", "Uint32T", "Uint32"),
  ("CppHeapPointerT", "UntaggedT", "Pointer"),
  ("Float16RawBitsT", "Word32T", "Uint16"),
  ("Float32T", "UntaggedT", "Float32"),
  ("Float64T", "UntaggedT", "Float64"),
  ("BoolT", "Word32T", "Int32"),
  ("Simd128T", "UntaggedT", "Simd128"),
  ("I8x16T", "Simd128T", ""),
  ("I16x8T", "Simd128T", ""),
  ("I32x2T", "Simd128T", ""),
];

/// 查类型机器表示 (空返回 None)
#[inline]
#[must_use]
pub fn machine_type_of(name: &str) -> Option<&'static str> {
  TNODE_TYPE_HIERARCHY
    .iter()
    .find(|(n, _, m)| *n == name && !m.is_empty())
    .map(|(_, _, m)| *m)
}

/// 查类型直接基类 (根返回 None)
#[inline]
#[must_use]
pub fn base_of(name: &str) -> Option<&'static str> {
  TNODE_TYPE_HIERARCHY
    .iter()
    .find(|(n, b, _)| *n == name && !b.is_empty())
    .map(|(_, b, _)| *b)
}

/// using 别名 (原版 TaggedT 按条件分支二选一; TrustedPointerT 派生)
pub const TNODE_USING_ALIASES: &[(&str, &str)] = &[
  ("TaggedT", "Int32T"),
  ("TaggedT", "IntPtrT"),
  ("TrustedPointerT", "IndirectPointerHandleT"),
  ("TrustedPointerT", "TaggedT"),
  ("BuiltinPtr", "Smi"),
];

// —— 提取不变式断言 ——
const _: () => {
  assert!(TNODE_TYPE_HIERARCHY.len() == 35);
  assert!(TNODE_USING_ALIASES.len() == 5);
  assert!(TNODE_TYPE_HIERARCHY[0].0 == "UntaggedT");
  assert!(machine_type_of("Int32T") == Some("Int32"));
  assert!(machine_type_of("Float64T") == Some("Float64"));
  assert!(base_of("IntegralT") == Some("UntaggedT"));
  assert!(machine_type_of("UntaggedT").is_none());
};
