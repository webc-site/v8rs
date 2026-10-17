use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_builtins_proxy_reflect::{
  Error, PROXY_TQ_DECLS, ProxyBuiltin, REFLECT_TQ_DECLS, ReflectBuiltin, call_proxy,
  construct_proxy, proxy_constructor, proxy_delete_property, proxy_get_property,
  proxy_get_property_fast_path, proxy_get_property_trap_result_lazy_deopt_continuation,
  proxy_get_prototype_of, proxy_has_property, proxy_is_extensible, proxy_prevent_extensions,
  proxy_revocable, proxy_revoke, proxy_set_property, proxy_set_prototype_of, reflect_apply,
  reflect_construct, reflect_define_property, reflect_delete_property, reflect_get,
  reflect_get_own_property_descriptor, reflect_get_prototype_of, reflect_has,
  reflect_is_extensible, reflect_own_keys, reflect_prevent_extensions, reflect_set,
  reflect_set_prototype_of,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_proxy_builtins() -> Void {
  let builtins: Vec<_> = ProxyBuiltin::iter().collect();
  assert_eq!(builtins.len(), 15);

  let mut ctors = 0;
  let mut traps = 0;
  let mut calls = 0;

  for b in &builtins {
    let name = b.name();
    let display_str = b.to_string();
    assert_eq!(name, display_str.as_str());

    let parsed = ProxyBuiltin::from_str(name)?;
    assert_eq!(*b, parsed);

    let desc = b.descriptor_name();
    assert!(desc.starts_with("Builtin_"));
    assert!(desc.ends_with("_InterfaceDescriptor"));

    if b.is_constructor_or_revocable() {
      ctors += 1;
    }
    if b.is_trap_handler() {
      traps += 1;
    }
    if b.is_call_or_construct() {
      calls += 1;
    }
  }

  assert_eq!(ctors, 3);
  assert_eq!(traps, 8);
  assert_eq!(calls, 2);

  // 全量 15 个工厂函数断言覆盖
  assert_eq!(call_proxy(), ProxyBuiltin::CallProxy);
  assert_eq!(construct_proxy(), ProxyBuiltin::ConstructProxy);
  assert_eq!(proxy_constructor(), ProxyBuiltin::ProxyConstructor);
  assert_eq!(proxy_delete_property(), ProxyBuiltin::ProxyDeleteProperty);
  assert_eq!(proxy_get_property(), ProxyBuiltin::ProxyGetProperty);
  assert_eq!(
    proxy_get_property_fast_path(),
    ProxyBuiltin::ProxyGetPropertyFastPath
  );
  assert_eq!(
    proxy_get_property_trap_result_lazy_deopt_continuation(),
    ProxyBuiltin::ProxyGetPropertyTrapResultLazyDeoptContinuation
  );
  assert_eq!(proxy_get_prototype_of(), ProxyBuiltin::ProxyGetPrototypeOf);
  assert_eq!(proxy_has_property(), ProxyBuiltin::ProxyHasProperty);
  assert_eq!(proxy_is_extensible(), ProxyBuiltin::ProxyIsExtensible);
  assert_eq!(
    proxy_prevent_extensions(),
    ProxyBuiltin::ProxyPreventExtensions
  );
  assert_eq!(proxy_revocable(), ProxyBuiltin::ProxyRevocable);
  assert_eq!(proxy_revoke(), ProxyBuiltin::ProxyRevoke);
  assert_eq!(proxy_set_property(), ProxyBuiltin::ProxySetProperty);
  assert_eq!(proxy_set_prototype_of(), ProxyBuiltin::ProxySetPrototypeOf);

  assert!(call_proxy().global_id().is_some());
  assert!(construct_proxy().global_id().is_some());

  assert_eq!(PROXY_TQ_DECLS.len(), 3);

  info!("test_proxy_builtins passed");
  OK
}

#[test]
fn test_reflect_builtins() -> Void {
  let builtins: Vec<_> = ReflectBuiltin::iter().collect();
  assert_eq!(builtins.len(), 13);

  let mut call_constructs = 0;
  let mut props = 0;
  let mut proto_exts = 0;

  for b in &builtins {
    let name = b.name();
    let display_str = b.to_string();
    assert_eq!(name, display_str.as_str());

    let parsed = ReflectBuiltin::from_str(name)?;
    assert_eq!(*b, parsed);

    let desc = b.descriptor_name();
    assert!(desc.starts_with("Builtin_"));
    assert!(desc.ends_with("_InterfaceDescriptor"));

    if b.is_call_or_construct() {
      call_constructs += 1;
    }
    if b.is_property_accessor() {
      props += 1;
    }
    if b.is_prototype_or_extensible() {
      proto_exts += 1;
    }
  }

  assert_eq!(call_constructs, 2);
  assert_eq!(props, 7);
  assert_eq!(proto_exts, 4);

  // 全量 13 个工厂函数断言覆盖
  assert_eq!(reflect_apply(), ReflectBuiltin::ReflectApply);
  assert_eq!(reflect_construct(), ReflectBuiltin::ReflectConstruct);
  assert_eq!(
    reflect_define_property(),
    ReflectBuiltin::ReflectDefineProperty
  );
  assert_eq!(
    reflect_delete_property(),
    ReflectBuiltin::ReflectDeleteProperty
  );
  assert_eq!(reflect_get(), ReflectBuiltin::ReflectGet);
  assert_eq!(
    reflect_get_own_property_descriptor(),
    ReflectBuiltin::ReflectGetOwnPropertyDescriptor
  );
  assert_eq!(
    reflect_get_prototype_of(),
    ReflectBuiltin::ReflectGetPrototypeOf
  );
  assert_eq!(reflect_has(), ReflectBuiltin::ReflectHas);
  assert_eq!(reflect_is_extensible(), ReflectBuiltin::ReflectIsExtensible);
  assert_eq!(reflect_own_keys(), ReflectBuiltin::ReflectOwnKeys);
  assert_eq!(
    reflect_prevent_extensions(),
    ReflectBuiltin::ReflectPreventExtensions
  );
  assert_eq!(reflect_set(), ReflectBuiltin::ReflectSet);
  assert_eq!(
    reflect_set_prototype_of(),
    ReflectBuiltin::ReflectSetPrototypeOf
  );

  assert!(reflect_apply().global_id().is_some());
  assert!(reflect_construct().global_id().is_some());

  assert_eq!(REFLECT_TQ_DECLS.len(), 2);

  info!("test_reflect_builtins passed");
  OK
}

#[test]
fn test_error_formatting() -> Void {
  let err1 = Error::TargetOrHandlerNotObject;
  assert_eq!(
    format!("{err1}"),
    "Cannot create proxy with a non-object target or handler"
  );

  let err2 = Error::ProxyRevoked;
  assert_eq!(format!("{err2}"), "Proxy revoked: cannot perform operation");

  let err3 = Error::TrapInvariantViolated("property is non-configurable".into());
  assert_eq!(
    format!("{err3}"),
    "Trap returned false or invalid result: property is non-configurable"
  );

  info!("test_error_formatting passed");
  OK
}
