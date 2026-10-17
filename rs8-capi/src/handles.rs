// rs8-capi/src/handles.rs
//! C ABI 句柄包装与生命周期管理

use std::{
  os::raw::c_void,
  panic::{AssertUnwindSafe, catch_unwind},
  ptr::null_mut,
};

/// 不透明隔离区指针
#[repr(C)]
pub struct V8Isolate {
  pub array_buffer_allocator: *mut c_void,
}

/// 不透明上下文指针
pub struct V8Context {
  pub isolate: *mut V8Isolate,
  pub interpreter: rs8::Interpreter,
}

/// 不透明值句柄指针
#[repr(C)]
pub struct V8Value {
  _private: [u8; 0],
}

/// C API 本地句柄包装 (Local<T>)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct V8Local {
  pub ptr: *mut c_void,
}

impl V8Local {
  pub const fn empty() -> Self {
    Self { ptr: null_mut() }
  }

  pub fn is_empty(self) -> bool {
    self.ptr.is_null()
  }

  pub fn from_ptr(ptr: *mut c_void) -> Self {
    Self { ptr }
  }
}

/// C API 持久句柄包装 (Global<T>)
#[repr(C)]
pub struct V8Global {
  pub ptr: *mut c_void,
}

impl V8Global {
  pub const fn empty() -> Self {
    Self { ptr: null_mut() }
  }

  pub fn is_empty(&self) -> bool {
    self.ptr.is_null()
  }
}

/// C API 句柄作用域包装 (HandleScope)
#[repr(C)]
pub struct V8HandleScope {
  pub isolate: *mut V8Isolate,
  pub prev_scope: *mut c_void,
}

/// 查询类型是否支持 Local 句柄转换
pub fn supports_local_conversion(type_name: &str) -> bool {
  v8_api::TO_LOCAL_LIST
    .iter()
    .any(|(_, internal, api, _)| *internal == type_name || *api == type_name)
}

// —— extern "C" 接口实现 ——

#[unsafe(no_mangle)]
pub extern "C" fn v8_local_is_empty(local: V8Local) -> bool {
  local.is_empty()
}

/// 判断全局句柄是否为空
///
/// # Safety
/// 调用者须确保 `global` 为空或指向有效的 `V8Global` 实例。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_global_is_empty(global: *const V8Global) -> bool {
  catch_unwind(AssertUnwindSafe(|| {
    if global.is_null() {
      return true;
    }
    // SAFETY: 指针非空校验已前置
    unsafe { (*global).is_empty() }
  }))
  .unwrap_or(true)
}

/// 重置全局句柄为 null
///
/// # Safety
/// 调用者须确保 `global` 为空或指向有效的可变 `V8Global` 实例。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_global_reset(global: *mut V8Global) {
  let _ = catch_unwind(AssertUnwindSafe(|| {
    if !global.is_null() {
      // SAFETY: 指针非空校验已前置
      unsafe {
        (*global).ptr = null_mut();
      }
    }
  }));
}
