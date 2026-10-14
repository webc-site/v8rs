// rs8-capi/src/snapshot.rs
//! C ABI 快照 Blob 结构与操作

use std::panic::{AssertUnwindSafe, catch_unwind};

#[repr(C)]
pub struct V8StartupData {
  pub data: *const u8,
  pub raw_size: i32,
}

/// 查询快照启动数据是否为空
///
/// # Safety
/// 调用者须确保 `data` 为空或指向有效的 `V8StartupData` 结构。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_startup_data_is_empty(data: *const V8StartupData) -> bool {
  catch_unwind(AssertUnwindSafe(|| {
    if data.is_null() {
      return true;
    }
    // SAFETY: 指针非空校验已前置
    unsafe { (*data).data.is_null() || (*data).raw_size <= 0 }
  }))
  .unwrap_or(true)
}
