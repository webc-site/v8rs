// 由 tool/src/builtins/builtins-collections.js 生成; 来源 v8/src/builtins 下 7 文件
// 内建名表 + 核心调度元数据 (7 内建)
// 来源: set-difference.tq, set-intersection.tq, set-is-disjoint-from.tq, set-is-subset-of.tq, set-is-superset-of.tq, set-symmetric-difference.tq, set-union.tq 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// ECMAScript Set 规范 7 大集合操作内建
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  SetPrototypeDifference,
  SetPrototypeIntersection,
  SetPrototypeIsDisjointFrom,
  SetPrototypeIsSubsetOf,
  SetPrototypeIsSupersetOf,
  SetPrototypeSymmetricDifference,
  SetPrototypeUnion,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::SetPrototypeDifference => "SetPrototypeDifference",
      Self::SetPrototypeIntersection => "SetPrototypeIntersection",
      Self::SetPrototypeIsDisjointFrom => "SetPrototypeIsDisjointFrom",
      Self::SetPrototypeIsSubsetOf => "SetPrototypeIsSubsetOf",
      Self::SetPrototypeIsSupersetOf => "SetPrototypeIsSupersetOf",
      Self::SetPrototypeSymmetricDifference => "SetPrototypeSymmetricDifference",
      Self::SetPrototypeUnion => "SetPrototypeUnion",
    }
  }

  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取对应的接口描述符符号
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 是否为布尔关系判断谓词 (isDisjointFrom, isSubsetOf, isSupersetOf)
  #[inline]
  pub const fn is_boolean_predicate(&self) -> bool {
    matches!(
      self,
      Self::SetPrototypeIsDisjointFrom
        | Self::SetPrototypeIsSubsetOf
        | Self::SetPrototypeIsSupersetOf
    )
  }

  /// 是否为新集合派生操作 (difference, intersection, symmetricDifference, union)
  #[inline]
  pub const fn is_set_generator(&self) -> bool {
    matches!(
      self,
      Self::SetPrototypeDifference
        | Self::SetPrototypeIntersection
        | Self::SetPrototypeSymmetricDifference
        | Self::SetPrototypeUnion
    )
  }
}

// 统一提供 7 项内建工厂函数
pub const fn set_prototype_difference() -> Builtin {
  Builtin::SetPrototypeDifference
}

pub const fn set_prototype_intersection() -> Builtin {
  Builtin::SetPrototypeIntersection
}

pub const fn set_prototype_is_disjoint_from() -> Builtin {
  Builtin::SetPrototypeIsDisjointFrom
}

pub const fn set_prototype_is_subset_of() -> Builtin {
  Builtin::SetPrototypeIsSubsetOf
}

pub const fn set_prototype_is_superset_of() -> Builtin {
  Builtin::SetPrototypeIsSupersetOf
}

pub const fn set_prototype_symmetric_difference() -> Builtin {
  Builtin::SetPrototypeSymmetricDifference
}

pub const fn set_prototype_union() -> Builtin {
  Builtin::SetPrototypeUnion
}

/// 来自 `set-*.tq` 的声明清单 (共 5 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("macro", "FastDifference"),
  ("macro", "FastIntersect"),
  ("macro", "FastIsDisjointFrom"),
  ("struct", "OrderedHashSetAndNumberOfElements"),
  ("macro", "FastSymmetricDifference"),
];
