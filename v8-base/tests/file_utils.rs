//! v8-base/src/file_utils.rs 集成测试
//!
//! 原版 `v8/test/unittests/base/file-utils-test.cc` 未在本仓库存在, 因此按 `file-utils.cc`
//! 语义逐条落规约:
//! - `RelativePath(exec, name)`: 取最后一个 `/` (含) 之前的目录前缀, 与 name 拼接, NUL 结尾
//! - 无 `/` 输入 → 结果等于 name + `\0`
//! - `exec_path` 已以 `/` 结尾 → 结果保留前缀 + name
//!
//! `read_file/write_file/file_exists/get_executable_path/file_system_info` 走 std::fs 与
//! libc; 测试使用 std::env::temp_dir 下的一次性子目录, 避免污染沙箱.

use std::{
  env::temp_dir,
  fs::{create_dir_all, remove_dir_all},
  path::PathBuf,
  process::id,
};

use aok::{OK, Void};
use v8_base::file_utils::{
  DIRECTORY_SEPARATOR_IS_SLASH, file_exists, file_system_info, get_executable_path,
  is_directory_separator, read_file, relative_path, write_file,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 生成独立 temp 目录, 每次调用返回新路径, 测试自行清理.
fn unique_temp_dir(tag: &str) -> PathBuf {
  let mut p = temp_dir();
  let nonce = format!("v8base-fileutils-{}-{:x}", tag, id());
  p.push(nonce);
  let _ = create_dir_all(&p);
  p
}

/// 原版 `RelativePath(exec_path="/a/b/prog", name="x.bin")` = "/a/b/x.bin\0"
#[test]
fn file_utils_relative_path_basic() -> Void {
  let out = relative_path(b"/a/b/prog", b"x.bin");
  let expect = b"/a/b/x.bin\0";
  assert_eq!(&out[..], &expect[..]);
  OK
}

/// exec_path 无分隔符 (basename) → 结果 = name + NUL
#[test]
fn file_utils_relative_path_no_separator() -> Void {
  let out = relative_path(b"prog", b"lib.js");
  assert_eq!(&out[..], b"lib.js\0");
  OK
}

/// exec_path 以 / 结尾 (e.g. "/bin/") → 分隔符自身包含进前缀
#[test]
fn file_utils_relative_path_trailing_slash() -> Void {
  let out = relative_path(b"/usr/bin/", b"foo");
  assert_eq!(&out[..], b"/usr/bin/foo\0");
  OK
}

/// 目录分隔符平台特性
#[test]
fn file_utils_directory_separator() -> Void {
  #[cfg(not(windows))]
  {
    const { assert!(DIRECTORY_SEPARATOR_IS_SLASH) };
    assert!(is_directory_separator(b'/'));
    assert!(!is_directory_separator(b'\\'));
  }
  #[cfg(windows)]
  {
    const { assert!(!DIRECTORY_SEPARATOR_IS_SLASH) };
    assert!(is_directory_separator(b'/'));
    assert!(is_directory_separator(b'\\'));
  }
  OK
}

/// Windows 反斜杠相对路径拼接测试
#[cfg(windows)]
#[test]
fn file_utils_relative_path_windows_backslash() -> Void {
  let out = relative_path(b"C:\\a\\b\\prog", b"x.bin");
  assert_eq!(&out[..], b"C:\\a\\b\\x.bin\0");
  OK
}

/// `write_file` + `read_file` 循环, 断言字节级一致
#[test]
fn file_utils_write_read_roundtrip() -> Void {
  let dir = unique_temp_dir("roundtrip");
  let path = dir.join("blob.bin");
  let payload: Vec<u8> = (0..=255u8).cycle().take(1024).collect();
  write_file(&path, &payload)
    .map_err(|e| log::error!("write failed: {e}"))
    .ok();
  assert!(file_exists(&path));
  let got = read_file(&path).expect("read_file");
  assert_eq!(got, payload);
  let _ = remove_dir_all(&dir);
  OK
}

/// `file_system_info`: 目录与常规文件区分, 不存在路径 → exists=false
#[test]
fn file_utils_file_system_info_kinds() -> Void {
  let dir = unique_temp_dir("info");
  let file = dir.join("a.txt");
  write_file(&file, b"hi").ok();

  let dir_info = file_system_info(&dir);
  assert!(dir_info.exists);
  assert!(dir_info.is_directory);
  assert!(!dir_info.is_regular_file);

  let file_info = file_system_info(&file);
  assert!(file_info.exists);
  assert!(!file_info.is_directory);
  assert!(file_info.is_regular_file);
  assert_eq!(file_info.size, 2);

  let missing = file_system_info(dir.join("nope"));
  assert!(!missing.exists);

  let _ = remove_dir_all(&dir);
  OK
}

/// `get_executable_path`: 结果必须非空且可 stat
#[test]
fn file_utils_get_executable_path_sanity() -> Void {
  let p = get_executable_path().expect("executable path");
  assert!(!p.as_os_str().is_empty());
  let info = file_system_info(&p);
  assert!(info.exists, "reported executable {:?} is missing", p);
  OK
}
