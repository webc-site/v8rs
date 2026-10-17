// 上游 output-stream 无独立单测 (profiler 序列化链路间接覆盖); 补两条关键路径:
// StringOutputStream 逐段累积 / FileOutputStream 落盘读回。
use std::{
  env::temp_dir,
  fs::{read, remove_file},
};

use aok::{OK, Void};
use v8_utils::output_stream::{FileOutputStream, OutputStream, StringOutputStream, WriteResult};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// StringOutputStream: 分段写入累积, str()/as_bytes() 一致
#[test]
fn string_output_stream_accumulates() -> Void {
  let mut s = StringOutputStream::new();
  assert_eq!(
    s.write_ascii_chunk(b"hello, ").unwrap(),
    WriteResult::Continue
  );
  assert_eq!(
    s.write_ascii_chunk(b"world").unwrap(),
    WriteResult::Continue
  );
  assert_eq!(s.as_bytes(), b"hello, world");
  assert_eq!(s.into_string(), "hello, world");
  OK
}

// FileOutputStream: 写文件 + end_of_stream 刷盘, 读回校验
#[test]
fn file_output_stream_roundtrip() -> Void {
  let path = temp_dir().join("v8-utils-ostream-test.txt");
  let mut f = FileOutputStream::create(path.to_str().unwrap()).expect("创建失败");
  assert!(f.is_open());
  assert_eq!(f.write_ascii_chunk(b"abc").unwrap(), WriteResult::Continue);
  assert_eq!(f.write_ascii_chunk(b"def").unwrap(), WriteResult::Continue);
  f.end_of_stream();
  drop(f);
  assert_eq!(read(&path).unwrap(), b"abcdef");
  let _ = remove_file(&path);
  OK
}
