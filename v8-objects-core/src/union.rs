// 由 tool/src/objects/union.h.js 从 v8/src/objects/union.h 生成; 修订就地改, 复跑覆盖
// Union<Ts...> 类型级机制登记; Rust 无可变参泛型, 语义实现归 Developer

//! Union<Ts...>: 多 V8 类型的标签联合类型级表示。
//!
//! 原版约束: 联合不可嵌套 (无 Union of Union)、成员不重复、
//! 禁止从 Union 继承 (破坏 is_subtype 检查); `UnionOf<Ts...>` 负责
//! 展平嵌套 + 去重, 且含 Smi 时归一化到 Smi 前置。
//! Rust 无可变参泛型, 本模块登记概念清单与判定语义骨架,
//! 具体建模 (enum 闭集/ trait) 归 Developer。

/// 概念清单 (union.h 类型级实体, 1:1 登记)。
pub const UNION_CONCEPTS: [&str; 13] = [
  "Union|class 模板 (final AllStatic; 成员别名 Without<U>)",
  "is_union|类型 trait (主模板 false + Union 特化 true)",
  "is_union_v|变量模板",
  "detail::TypeList|类型列表 (kSize = sizeof...)",
  "detail::ToUnion|TypeList → Union 折叠 (3 特化: 空→void/单→T/多→Union)",
  "detail::PopFrontUnion|DEDUP_PACK 分支: 弹头 (2 特化)",
  "detail::UnionWithoutHelper|非 DEDUP_PACK 分支: 累积剔除 (3 特化)",
  "detail::Deduplicate|非 DEDUP_PACK 分支: 顺序去重 (2 特化)",
  "detail::AsTypeList|2 重载: T*→TypeList<T>, Union<Us...>*→TypeList<Us...>",
  "detail::operator+|TypeList 拼接 (折叠表达式, 免深递归特化)",
  "detail::DedupAndFinalize|去重收尾 (kHasSmi 时 Smi 前置归一)",
  "detail::FlattenUnionHelper|AsTypeList 折叠拼接 + 去重收尾",
  "UnionOf|别名模板: 展平嵌套 + 去重",
];

/// 原版对 Union 的静态约束 (static_assert 组)。
pub const UNION_INVARIANTS: [&str; 5] = [
  "非嵌套: Union 的成员不得再是 Union",
  "不重复: 每个类型在 Union 中至多一次",
  "非继承: 禁止从 Union<Ts...> 继承 (干扰 is_subtype 判定)",
  "Smi 归一: 含 Smi 的 UnionOf 结果把 Smi 置于首位",
  "Without 无命中时为 no-op (Union<Smi, HeapObject>::Without<HeapNumber> 原样返回)",
];

/// 联合类型特征标记
pub trait UnionMarker {
  /// 是否为 Union 类型
  const IS_UNION: bool = false;
}

impl<T> UnionMarker for T {}

/// 判定 T 是否 Union 类型 (对应 is_union_v<T>)。
///
/// v8: src/objects/union.h is_union
#[inline]
pub const fn is_union<T: UnionMarker>() -> bool {
  T::IS_UNION
}

/// 展平嵌套联合并去重 (对应 UnionOf<Ts...> 别名语义的值级探针)。
///
/// v8: src/objects/union.h UnionOf
#[inline]
pub const fn union_flatten<T>(value: T) -> T {
  value
}
