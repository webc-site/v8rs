// 由 tool/src/builtins/builtins-proxy-reflect.js 生成; 来源 v8/src/builtins 下 2 文件
// 模块索引
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

#![forbid(unsafe_code)]

mod error;
mod proxy;
mod reflect_access;

pub use error::{Error, Result};
pub use proxy::{
  Builtin as ProxyBuiltin, TQ_DECLS as PROXY_TQ_DECLS, call_proxy, construct_proxy,
  proxy_constructor, proxy_delete_property, proxy_get_property, proxy_get_property_fast_path,
  proxy_get_property_trap_result_lazy_deopt_continuation, proxy_get_prototype_of,
  proxy_has_property, proxy_is_extensible, proxy_prevent_extensions, proxy_revocable, proxy_revoke,
  proxy_set_property, proxy_set_prototype_of,
};
pub use reflect_access::{
  Builtin as ReflectBuiltin, TQ_DECLS as REFLECT_TQ_DECLS, reflect_apply, reflect_construct,
  reflect_define_property, reflect_delete_property, reflect_get,
  reflect_get_own_property_descriptor, reflect_get_prototype_of, reflect_has,
  reflect_is_extensible, reflect_own_keys, reflect_prevent_extensions, reflect_set,
  reflect_set_prototype_of,
};
