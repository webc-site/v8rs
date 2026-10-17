// rs8-capi/src/initialization.rs
//! C ABI 引擎初始化与全局生命周期控制

use std::{
  ffi::{CStr, c_char},
  panic::{AssertUnwindSafe, catch_unwind},
  sync::atomic::{AtomicBool, Ordering},
};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

const V8_VERSION_STRING: &[u8] = b"13.5.0 (v8rs)\0";

#[unsafe(no_mangle)]
pub extern "C" fn v8_initialize() -> bool {
  catch_unwind(|| {
    log::info!("v8_initialize called");
    INITIALIZED.store(true, Ordering::SeqCst);
    true
  })
  .unwrap_or(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_is_initialized() -> bool {
  INITIALIZED.load(Ordering::SeqCst)
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_dispose() -> bool {
  catch_unwind(|| {
    INITIALIZED.store(false, Ordering::SeqCst);
    true
  })
  .unwrap_or(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_get_version() -> *const c_char {
  V8_VERSION_STRING.as_ptr().cast::<c_char>()
}

/// 设置 V8 命令行旗标
///
/// # Safety
/// 调用者须确保 `flags` 为空或指向以 null 结尾的有效 C 字符串。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_set_flags_from_string(flags: *const c_char) -> bool {
  catch_unwind(AssertUnwindSafe(|| {
    if flags.is_null() {
      return false;
    }
    // SAFETY: 调用者保证 flags 为以 null 结尾的有效 C 字符串
    let c_str = unsafe { CStr::from_ptr(flags) };
    let Ok(s) = c_str.to_str() else { return false };
    for part in s.split_whitespace() {
      let flag = part.trim_start_matches('-');
      let _ = rs8::SHELL_OPTIONS.iter().find(|(_, cli)| *cli == flag);
    }
    true
  }))
  .unwrap_or(false)
}
