// 由 tool/src/builtins/builtins-proxy-reflect.js 生成; 来源 v8/src/builtins 下 14 文件
// 内建名表 + 核心调度元数据 (15 内建)
// 来源: builtins-proxy-gen.cc, builtins-proxy-gen.h, proxy-*.tq 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// Proxy 核心内建名表 (15 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  CallProxy,
  ConstructProxy,
  ProxyConstructor,
  ProxyDeleteProperty,
  ProxyGetProperty,
  ProxyGetPropertyFastPath,
  ProxyGetPropertyTrapResultLazyDeoptContinuation,
  ProxyGetPrototypeOf,
  ProxyHasProperty,
  ProxyIsExtensible,
  ProxyPreventExtensions,
  ProxyRevocable,
  ProxyRevoke,
  ProxySetProperty,
  ProxySetPrototypeOf,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::CallProxy => "CallProxy",
      Self::ConstructProxy => "ConstructProxy",
      Self::ProxyConstructor => "ProxyConstructor",
      Self::ProxyDeleteProperty => "ProxyDeleteProperty",
      Self::ProxyGetProperty => "ProxyGetProperty",
      Self::ProxyGetPropertyFastPath => "ProxyGetPropertyFastPath",
      Self::ProxyGetPropertyTrapResultLazyDeoptContinuation => {
        "ProxyGetPropertyTrapResultLazyDeoptContinuation"
      }
      Self::ProxyGetPrototypeOf => "ProxyGetPrototypeOf",
      Self::ProxyHasProperty => "ProxyHasProperty",
      Self::ProxyIsExtensible => "ProxyIsExtensible",
      Self::ProxyPreventExtensions => "ProxyPreventExtensions",
      Self::ProxyRevocable => "ProxyRevocable",
      Self::ProxyRevoke => "ProxyRevoke",
      Self::ProxySetProperty => "ProxySetProperty",
      Self::ProxySetPrototypeOf => "ProxySetPrototypeOf",
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

  /// 是否为 Proxy 构造或撤销相关内建
  #[inline]
  pub const fn is_constructor_or_revocable(&self) -> bool {
    matches!(
      self,
      Self::ProxyConstructor | Self::ProxyRevocable | Self::ProxyRevoke
    )
  }

  /// 是否为 Proxy Trap 陷阱调度分发内建
  #[inline]
  pub const fn is_trap_handler(&self) -> bool {
    matches!(
      self,
      Self::ProxyDeleteProperty
        | Self::ProxyGetProperty
        | Self::ProxyGetPrototypeOf
        | Self::ProxyHasProperty
        | Self::ProxyIsExtensible
        | Self::ProxyPreventExtensions
        | Self::ProxySetProperty
        | Self::ProxySetPrototypeOf
    )
  }

  /// 是否为函数/构造调用代理进入点
  #[inline]
  pub const fn is_call_or_construct(&self) -> bool {
    matches!(self, Self::CallProxy | Self::ConstructProxy)
  }
}

// 统一提供 15 项内建工厂函数
pub const fn call_proxy() -> Builtin {
  Builtin::CallProxy
}

pub const fn construct_proxy() -> Builtin {
  Builtin::ConstructProxy
}

pub const fn proxy_constructor() -> Builtin {
  Builtin::ProxyConstructor
}

pub const fn proxy_delete_property() -> Builtin {
  Builtin::ProxyDeleteProperty
}

pub const fn proxy_get_property() -> Builtin {
  Builtin::ProxyGetProperty
}

pub const fn proxy_get_property_fast_path() -> Builtin {
  Builtin::ProxyGetPropertyFastPath
}

pub const fn proxy_get_property_trap_result_lazy_deopt_continuation() -> Builtin {
  Builtin::ProxyGetPropertyTrapResultLazyDeoptContinuation
}

pub const fn proxy_get_prototype_of() -> Builtin {
  Builtin::ProxyGetPrototypeOf
}

pub const fn proxy_has_property() -> Builtin {
  Builtin::ProxyHasProperty
}

pub const fn proxy_is_extensible() -> Builtin {
  Builtin::ProxyIsExtensible
}

pub const fn proxy_prevent_extensions() -> Builtin {
  Builtin::ProxyPreventExtensions
}

pub const fn proxy_revocable() -> Builtin {
  Builtin::ProxyRevocable
}

pub const fn proxy_revoke() -> Builtin {
  Builtin::ProxyRevoke
}

pub const fn proxy_set_property() -> Builtin {
  Builtin::ProxySetProperty
}

pub const fn proxy_set_prototype_of() -> Builtin {
  Builtin::ProxySetPrototypeOf
}

/// 来自 `proxy*.tq` 的声明清单 (共 3 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("extern macro", "ProxiesCodeStubAssembler"),
  ("type", "ProxyRevokeFunctionContext"),
  ("extern macro", "ProxiesCodeStubAssembler"),
];
