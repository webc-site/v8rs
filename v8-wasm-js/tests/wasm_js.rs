//! WebAssembly JS API 安装表与外部引用集成测试
//! 对应原版 v8/src/wasm/wasm-js.{h,cc} 规范

use aok::{OK, Void};
use v8_wasm_js::{
  ACCESSOR_NAMES, API_INSTALLS, CONDITIONAL_INSTALLERS, CONSTRUCTOR_NAMES, ERROR_CONSTRUCTORS,
  EXCEPTION_PRIVATE_FIELDS, GLOBAL_VALUE_ACCESSOR_PAIR, INSTALL_PHASES,
  JS_TAG_INITIAL_CANONICAL_INDEX, JS_TAG_SIGNATURE, NAMESPACE_METHODS, OBJECT_NAMES,
  RESOLVER_GLOBAL_HANDLES, RO_ATTRIBUTES, STATIC_METHOD_NAMES, VALUE_TYPE_NAMES,
  WASM_JS_EXTERNAL_REFERENCES, webassembly_api_callbacks,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_external_references_and_callbacks() -> Void {
  assert_eq!(WASM_JS_EXTERNAL_REFERENCES.len(), 32);
  let callbacks = webassembly_api_callbacks();
  assert_eq!(callbacks.len(), 32);
  assert_eq!(callbacks, WASM_JS_EXTERNAL_REFERENCES);

  assert!(WASM_JS_EXTERNAL_REFERENCES.contains(&"WebAssemblyCompile"));
  assert!(WASM_JS_EXTERNAL_REFERENCES.contains(&"WebAssemblyInstantiate"));
  assert!(WASM_JS_EXTERNAL_REFERENCES.contains(&"WebAssemblyMemory"));
  assert!(WASM_JS_EXTERNAL_REFERENCES.contains(&"WebAssemblyTable"));
  assert!(WASM_JS_EXTERNAL_REFERENCES.contains(&"WebAssemblyValidate"));
  OK
}

#[test]
fn test_constructor_and_method_names() -> Void {
  assert_eq!(CONSTRUCTOR_NAMES.len(), 11);
  assert!(CONSTRUCTOR_NAMES.contains(&"Module"));
  assert!(CONSTRUCTOR_NAMES.contains(&"Instance"));
  assert!(CONSTRUCTOR_NAMES.contains(&"Memory"));
  assert!(CONSTRUCTOR_NAMES.contains(&"Table"));
  assert!(CONSTRUCTOR_NAMES.contains(&"Global"));
  assert!(CONSTRUCTOR_NAMES.contains(&"Tag"));
  assert!(CONSTRUCTOR_NAMES.contains(&"Exception"));

  assert_eq!(STATIC_METHOD_NAMES.len(), 5);
  assert!(STATIC_METHOD_NAMES.contains(&"compile"));
  assert!(STATIC_METHOD_NAMES.contains(&"compileStreaming"));
  assert!(STATIC_METHOD_NAMES.contains(&"instantiate"));
  assert!(STATIC_METHOD_NAMES.contains(&"instantiateStreaming"));
  assert!(STATIC_METHOD_NAMES.contains(&"validate"));

  assert_eq!(ACCESSOR_NAMES.len(), 17);
  assert!(ACCESSOR_NAMES.contains(&"buffer"));
  assert!(ACCESSOR_NAMES.contains(&"length"));
  assert!(ACCESSOR_NAMES.contains(&"value"));

  assert_eq!(VALUE_TYPE_NAMES.len(), 8);
  assert_eq!(INSTALL_PHASES.len(), 3);
  assert_eq!(CONDITIONAL_INSTALLERS.len(), 3);
  OK
}

#[test]
fn test_api_installs_and_bindings() -> Void {
  assert_eq!(OBJECT_NAMES.len(), 5);
  assert_eq!(NAMESPACE_METHODS.len(), 6);
  assert_eq!(API_INSTALLS.len(), 22);
  assert_eq!(ERROR_CONSTRUCTORS.len(), 3);
  assert_eq!(EXCEPTION_PRIVATE_FIELDS.len(), 2);

  assert_eq!(
    GLOBAL_VALUE_ACCESSOR_PAIR,
    ("WebAssemblyGlobalGetValue", "WebAssemblyGlobalSetValue")
  );
  assert_eq!(JS_TAG_INITIAL_CANONICAL_INDEX, 0);
  assert_eq!(JS_TAG_SIGNATURE, "() -> (externref)");
  assert_eq!(RO_ATTRIBUTES, "DONT_ENUM | READ_ONLY");
  assert_eq!(RESOLVER_GLOBAL_HANDLES, 6);

  // 关键原型绑定检查
  assert!(API_INSTALLS.contains(&("table_proto", "grow", "WebAssemblyTableGrow")));
  assert!(API_INSTALLS.contains(&("memory_proto", "buffer", "WebAssemblyMemoryGetBuffer")));
  assert!(API_INSTALLS.contains(&("instance_proto", "exports", "WebAssemblyInstanceGetExports")));

  OK
}
