// 由 tool/src/base/platform/wrappers.h.js 从 v8/src/base/platform/wrappers.h 生成; 修订就地改, 复跑覆盖
// Fopen/Fclose 包装 (V8_OS_STARBOARD 分支登记)

use std::ffi::CStr;

/// 文件打开包装 (原版 base::Fopen; 非 Starboard 直通 fopen)
/// V8_OS_STARBOARD 分支: 返回 NULL — 本目标不启用 Starboard, 登记不产码
pub fn fopen(filename: &CStr, mode: &CStr) -> *mut libc::FILE {
  unsafe { libc::fopen(filename.as_ptr(), mode.as_ptr()) }
}

/// 文件关闭包装 (原版 base::Fclose; 非 Starboard 直通 fclose)
pub fn fclose(stream: *mut libc::FILE) -> i32 {
  unsafe { libc::fclose(stream) }
}

// —— 提取不变式断言 ——
const _: () = {
  // Starboard 分支哨兵值登记 (NULL / -1)
  const SB_FOPEN: i32 = 0; // NULL
  const SB_FCLOSE: i32 = -1;
  assert!(SB_FOPEN == 0 && SB_FCLOSE == -1);
};
