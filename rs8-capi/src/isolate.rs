// rs8-capi/src/isolate.rs
//! C ABI Isolate 隔离区实例控制

use std::{
  cell::Cell,
  os::raw::c_void,
  panic::{AssertUnwindSafe, catch_unwind},
  ptr::null_mut,
};

use crate::handles::V8Isolate;

thread_local! {
  static CURRENT_ISOLATE: Cell<*mut V8Isolate> = const { Cell::new(null_mut()) };
}

#[repr(C)]
pub struct IsolateCreateParams {
  pub array_buffer_allocator: *mut c_void,
  pub snapshot_blob: *const c_void,
}

/// 创建隔离区实例
///
/// # Safety
/// 调用者须确保 `params` 为空或指向有效的 `IsolateCreateParams`。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_isolate_new(params: *const IsolateCreateParams) -> *mut V8Isolate {
  catch_unwind(AssertUnwindSafe(|| {
    let allocator = if params.is_null() {
      null_mut()
    } else {
      // SAFETY: 指针非空且由调用者保证有效
      unsafe { (*params).array_buffer_allocator }
    };
    let boxed = Box::new(V8Isolate {
      array_buffer_allocator: allocator,
    });
    Box::into_raw(boxed)
  }))
  .unwrap_or(null_mut())
}

/// 销毁隔离区实例
///
/// # Safety
/// 调用者须确保 `isolate` 为空或指向由 `v8_isolate_new` 创建且未被释放的实例。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_isolate_dispose(isolate: *mut V8Isolate) {
  let _ = catch_unwind(AssertUnwindSafe(|| {
    if !isolate.is_null() {
      // SAFETY: isolate 由 v8_isolate_new 分配
      unsafe {
        drop(Box::from_raw(isolate));
      }
    }
  }));
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_isolate_enter(isolate: *mut V8Isolate) {
  let _ = catch_unwind(AssertUnwindSafe(|| {
    if !isolate.is_null() {
      CURRENT_ISOLATE.with(|cell| cell.set(isolate));
    }
  }));
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_isolate_exit(isolate: *mut V8Isolate) {
  let _ = catch_unwind(AssertUnwindSafe(|| {
    CURRENT_ISOLATE.with(|cell| {
      if cell.get() == isolate {
        cell.set(null_mut());
      }
    });
  }));
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_isolate_get_current() -> *mut V8Isolate {
  catch_unwind(|| CURRENT_ISOLATE.with(|cell| cell.get())).unwrap_or(null_mut())
}
