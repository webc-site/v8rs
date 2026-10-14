// 由 tool/src/builtins/builtins-proxy-reflect.js 生成; 来源 v8/src/builtins 下 4 文件
// 内建名表 + 核心调度元数据 (13 内建)
// 来源: builtins-reflect.cc, accessors.cc, accessors.h, reflect.tq 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// Reflect 核心内建名表 (13 项，对应 ECMAScript 规范 Reflect 全部 13 个静态方法)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  ReflectApply,
  ReflectConstruct,
  ReflectDefineProperty,
  ReflectDeleteProperty,
  ReflectGet,
  ReflectGetOwnPropertyDescriptor,
  ReflectGetPrototypeOf,
  ReflectHas,
  ReflectIsExtensible,
  ReflectOwnKeys,
  ReflectPreventExtensions,
  ReflectSet,
  ReflectSetPrototypeOf,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::ReflectApply => "ReflectApply",
      Self::ReflectConstruct => "ReflectConstruct",
      Self::ReflectDefineProperty => "ReflectDefineProperty",
      Self::ReflectDeleteProperty => "ReflectDeleteProperty",
      Self::ReflectGet => "ReflectGet",
      Self::ReflectGetOwnPropertyDescriptor => "ReflectGetOwnPropertyDescriptor",
      Self::ReflectGetPrototypeOf => "ReflectGetPrototypeOf",
      Self::ReflectHas => "ReflectHas",
      Self::ReflectIsExtensible => "ReflectIsExtensible",
      Self::ReflectOwnKeys => "ReflectOwnKeys",
      Self::ReflectPreventExtensions => "ReflectPreventExtensions",
      Self::ReflectSet => "ReflectSet",
      Self::ReflectSetPrototypeOf => "ReflectSetPrototypeOf",
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

  /// 是否为函数调用或构造反射方法 (apply, construct)
  #[inline]
  pub const fn is_call_or_construct(&self) -> bool {
    matches!(self, Self::ReflectApply | Self::ReflectConstruct)
  }

  /// 是否为属性访问与操作反射方法 (get, set, has, deleteProperty, defineProperty, getOwnPropertyDescriptor)
  #[inline]
  pub const fn is_property_accessor(&self) -> bool {
    matches!(
      self,
      Self::ReflectGet
        | Self::ReflectSet
        | Self::ReflectHas
        | Self::ReflectDeleteProperty
        | Self::ReflectDefineProperty
        | Self::ReflectGetOwnPropertyDescriptor
        | Self::ReflectOwnKeys
    )
  }

  /// 是否为原型或扩展性反射方法 (getPrototypeOf, setPrototypeOf, isExtensible, preventExtensions)
  #[inline]
  pub const fn is_prototype_or_extensible(&self) -> bool {
    matches!(
      self,
      Self::ReflectGetPrototypeOf
        | Self::ReflectSetPrototypeOf
        | Self::ReflectIsExtensible
        | Self::ReflectPreventExtensions
    )
  }
}

// 统一提供 13 项内建工厂函数
pub const fn reflect_apply() -> Builtin {
  Builtin::ReflectApply
}

pub const fn reflect_construct() -> Builtin {
  Builtin::ReflectConstruct
}

pub const fn reflect_define_property() -> Builtin {
  Builtin::ReflectDefineProperty
}

pub const fn reflect_delete_property() -> Builtin {
  Builtin::ReflectDeleteProperty
}

pub const fn reflect_get() -> Builtin {
  Builtin::ReflectGet
}

pub const fn reflect_get_own_property_descriptor() -> Builtin {
  Builtin::ReflectGetOwnPropertyDescriptor
}

pub const fn reflect_get_prototype_of() -> Builtin {
  Builtin::ReflectGetPrototypeOf
}

pub const fn reflect_has() -> Builtin {
  Builtin::ReflectHas
}

pub const fn reflect_is_extensible() -> Builtin {
  Builtin::ReflectIsExtensible
}

pub const fn reflect_own_keys() -> Builtin {
  Builtin::ReflectOwnKeys
}

pub const fn reflect_prevent_extensions() -> Builtin {
  Builtin::ReflectPreventExtensions
}

pub const fn reflect_set() -> Builtin {
  Builtin::ReflectSet
}

pub const fn reflect_set_prototype_of() -> Builtin {
  Builtin::ReflectSetPrototypeOf
}

/// 来自 `reflect.tq` 的声明清单 (共 2 条)
pub const TQ_DECLS: &[(&str, &str)] = &[("type", "OnNonExistent"), ("extern macro", "SmiConstant")];
