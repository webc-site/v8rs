#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod wasm_js;
mod wasm_js_cc;

pub use error::{Error, Result};
pub use wasm_js::{
  ACCESSOR_NAMES, CONDITIONAL_INSTALLERS, CONSTRUCTOR_NAMES, INSTALL_PHASES, STATIC_METHOD_NAMES,
  VALUE_TYPE_NAMES, WASM_JS_EXTERNAL_REFERENCES,
};
pub use wasm_js_cc::{
  API_INSTALLS, ERROR_CONSTRUCTORS, EXCEPTION_PRIVATE_FIELDS, GLOBAL_VALUE_ACCESSOR_PAIR,
  JS_TAG_INITIAL_CANONICAL_INDEX, JS_TAG_SIGNATURE, NAMESPACE_METHODS, OBJECT_NAMES,
  RESOLVER_GLOBAL_HANDLES, RO_ATTRIBUTES, webassembly_api_callbacks,
};

/// 桥接: 在 JS 环境中请求 Wasm 内存扩容 (真实消费 delta_pages 并调用 wasm_objects 容量计算)
pub fn js_wasm_memory_grow(current_pages: u32, delta_pages: u32, max_pages: u32) -> Option<u32> {
  let target_pages = current_pages.checked_add(delta_pages)?;
  if target_pages > max_pages {
    return None;
  }
  if delta_pages == 0 {
    return Some(current_pages);
  }
  let result = v8_wasm_objects::new_capacity_pages(
    max_pages as usize,
    target_pages as usize,
    current_pages as usize,
  );
  u32::try_from(result).ok()
}
