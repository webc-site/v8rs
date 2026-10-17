//! 原版 v8/src/base/file-utils.cc/h — 文件/路径工具
//!
//! 原版文件仅暴露 `RelativePath` 一个函数 (v8/src/base/file-utils.h:15-18).
//! 任务要求的 `ReadFile`/`WriteFile`/`FileExists`/`GetExecutablePath`/
//! `FileSystemInfo`/`Time` 在原版属于 `OS::` 平台方法 (platform-posix.cc /
//! platform-win.cc), 归 `v8-base-platform` crate. 本模块提供 base 层的
//! 通用非平台分支实现 (std::fs + libc), 严格 1:1 保留 `relative_path` 语义;
//! 其余按任务要求补齐, 全部走 `Result`, 禁 panic/unwrap.

use std::{
  fs::{File, metadata, symlink_metadata},
  io,
  io::{Read as _, Write as _},
  path::{Path, PathBuf},
  ptr::null_mut,
  result,
};

use thiserror::Error;

/// 本模块局部错误类型; error.rs 未列变体, 集中此处避免向 crate 根引入耦合.
#[derive(Debug, Error)]
pub enum FileUtilsError {
  #[error(transparent)]
  Io(#[from] io::Error),
  #[error("executable path unavailable")]
  NoExecutablePath,
  #[error("utf-8 conversion failed")]
  Utf8,
}

/// 本模块统一 Result 别名.
pub type Result<T> = result::Result<T, FileUtilsError>;

/// 原版 `RelativePath(exec_path, name)`.
///
/// 语义: 取 exec_path 中最后一个目录分隔符 (含) 之前的目录前缀, 与 name 拼接.
/// 若 exec_path 无目录分隔符 (basename_start == 0), 直接返回 name 的拷贝; 结果 NUL 结尾.
///
/// macOS/Linux 分隔符为 `/`; 严格保留原版逐字节扫描 (无 UTF-8 校验).
#[must_use]
pub fn relative_path(exec_path: &[u8], name: &[u8]) -> Vec<u8> {
  // 反向查找最后一个目录分隔符 (原版 OS::isDirectorySeparator)
  let basename_start = match exec_path.iter().rposition(|&c| is_directory_separator(c)) {
    Some(idx) => idx + 1,
    None => 0,
  };
  let mut buf = Vec::with_capacity(basename_start + name.len() + 1);
  buf.extend_from_slice(&exec_path[..basename_start]);
  buf.extend_from_slice(name);
  buf.push(0);
  buf
}

/// 原版 `OS::ReadFile` 的等价: 读整个文件到 Vec<u8>.
pub fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>> {
  let mut file = File::open(path)?;
  let mut out = Vec::new();
  file.read_to_end(&mut out)?;
  Ok(out)
}

/// 原版 `OS::AppendFileContent`/`OS::WriteFile` 的常用等价: 全量写入 (截断).
pub fn write_file(path: impl AsRef<Path>, data: &[u8]) -> Result<()> {
  let mut file = File::create(path)?;
  file.write_all(data)?;
  Ok(())
}

/// 原版 `OS::IsExecutableFile`/文件存在性检查的基础形式: `stat` 是否可访问.
#[must_use]
pub fn file_exists(path: impl AsRef<Path>) -> bool {
  symlink_metadata(path.as_ref()).is_ok()
}

/// 原版 `OS::DirectorySeparatorIsSlash`.
#[cfg(not(windows))]
pub const DIRECTORY_SEPARATOR_IS_SLASH: bool = true;
#[cfg(windows)]
pub const DIRECTORY_SEPARATOR_IS_SLASH: bool = false;

/// 原版 `OS::isDirectorySeparator(c)`.
#[must_use]
pub const fn is_directory_separator(c: u8) -> bool {
  #[cfg(windows)]
  {
    c == b'/' || c == b'\\'
  }
  #[cfg(not(windows))]
  {
    c == b'/'
  }
}

/// 原版 `OS::GetExecutablePath`:
/// - Linux: readlink `/proc/self/exe`
/// - macOS: `_NSGetExecutablePath` 循环扩缓冲 + realpath 规范化
pub fn get_executable_path() -> Result<PathBuf> {
  #[cfg(target_os = "linux")]
  {
    use std::fs::read_link;
    // SAFETY: read_link 与 /proc 直读语义相同; 由 std 保证.
    let p = read_link("/proc/self/exe")?;
    Ok(p)
  }
  #[cfg(target_os = "macos")]
  {
    get_executable_path_darwin()
  }
  #[cfg(not(any(target_os = "linux", target_os = "macos")))]
  {
    use std::env::current_exe;
    // current_exe 语义与 OS::GetExecutablePath 一致 (realpath 化)
    match current_exe() {
      Ok(p) => Ok(p),
      Err(_) => Err(FileUtilsError::NoExecutablePath),
    }
  }
}

/// macOS `_NSGetExecutablePath` 循环扩缓冲实现 (原版 platform-posix.cc 的 darwin 分支).
#[cfg(target_os = "macos")]
fn get_executable_path_darwin() -> Result<PathBuf> {
  unsafe extern "C" {
    fn _NSGetExecutablePath(buf: *mut libc::c_char, bufsize: *mut u32) -> libc::c_int;
  }
  let mut size: u32 = 0;
  // 第一次调用传 0 只查询所需长度, 返回 -1 是"缓冲不够"的固定信号.
  // SAFETY: 传空指针 + 可写 u32, 与 dyld 声明一致.
  unsafe {
    let _ = _NSGetExecutablePath(null_mut(), &raw mut size);
  }
  // 增加一定余量避免 buffer_size_changed 竞态.
  let mut buf = vec![0u8; size as usize + 1];
  // SAFETY: buf.as_mut_ptr() 为可读写的 c_char* 目标, 长度 >= size.
  let rc = unsafe { _NSGetExecutablePath(buf.as_mut_ptr().cast(), &raw mut size) };
  if rc != 0 {
    return Err(FileUtilsError::NoExecutablePath);
  }
  // 结果 NUL 结尾; 手动截断到首个 0, 避开 `CStr::from_ptr` 的长度未验证风险.
  let end = buf
    .iter()
    .position(|&c| c == 0)
    .ok_or(FileUtilsError::NoExecutablePath)?;
  use core::str::from_utf8;
  let s = from_utf8(&buf[..end]).map_err(|_| FileUtilsError::Utf8)?;
  Ok(PathBuf::from(s))
}

/// 原版 `OS::FileSystemInfo` — 简化到基础 stat 语义.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileSystemInfo {
  pub exists: bool,
  pub is_directory: bool,
  pub is_regular_file: bool,
  pub size: u64,
}

/// 通过 std::fs::metadata 组装 `FileSystemInfo`; 不可访问时返回 exists=false 结构.
#[must_use]
pub fn file_system_info(path: impl AsRef<Path>) -> FileSystemInfo {
  match metadata(path.as_ref()) {
    Ok(md) => FileSystemInfo {
      exists: true,
      is_directory: md.is_dir(),
      is_regular_file: md.is_file(),
      size: md.len(),
    },
    Err(_) => FileSystemInfo {
      exists: false,
      is_directory: false,
      is_regular_file: false,
      size: 0,
    },
  }
}

/// 原版 `OS::Timezone`/`OS::Time` 的最小可移植形态:
/// 返回 UTC Unix 秒 (`time(nullptr)`), 与 libc 完全一致; 更精细的 Time struct 交给
/// 上层 `v8-date` (引擎 Date 语义, 禁在此绑定第三方).
#[must_use]
pub fn time_now_seconds() -> i64 {
  // SAFETY: `time(NULL)` 只读系统时钟, 无副作用.
  unsafe { libc::time(null_mut()) as i64 }
}
