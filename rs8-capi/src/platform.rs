// rs8-capi/src/platform.rs
//! C ABI 平台 (Platform) 实例与调度接口

use std::{
  os::raw::c_void,
  panic::{AssertUnwindSafe, catch_unwind},
  ptr::null_mut,
};

#[repr(C)]
pub struct V8Platform {
  pub thread_pool_size: i32,
  pub idle_task_support: bool,
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_platform_new_default(
  thread_pool_size: i32,
  idle_task_support: bool,
) -> *mut V8Platform {
  catch_unwind(|| {
    let boxed = Box::new(V8Platform {
      thread_pool_size,
      idle_task_support,
    });
    Box::into_raw(boxed)
  })
  .unwrap_or(null_mut())
}

/// 销毁平台实例
///
/// # Safety
/// 调用者须确保 `platform` 为空或指向由 `v8_platform_new_default` 创建且未被释放的平台实例。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_platform_dispose(platform: *mut V8Platform) {
  let _ = catch_unwind(AssertUnwindSafe(|| {
    if !platform.is_null() {
      // SAFETY: platform 由 v8_platform_new_default 分配
      unsafe {
        drop(Box::from_raw(platform));
      }
    }
  }));
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_platform_pump_message_loop(
  platform: *mut V8Platform,
  isolate: *mut c_void,
) -> bool {
  let _ = (platform, isolate);
  true
}
