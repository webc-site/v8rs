// 诊断字符串流测试 (v8/src/strings/string-stream.h 语义)

use aok::OK;
use v8_strings::string_stream::{FixedStringAllocator, FmtElm, HeapStringAllocator, StringStream};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn string_stream_heap_formatting() -> aok::Void {
  let mut alloc = HeapStringAllocator::default();
  let mut ss = StringStream::new(&mut alloc);

  ss.add(
    "Hello, %s! Value: %d (0x%x)",
    &[FmtElm::CStr("World"), FmtElm::Int(42), FmtElm::Int(255)],
  );

  assert_eq!(ss.to_string_lossy(), "Hello, World! Value: 42 (0xff)");

  ss.reset();
  ss.add(
    "Float: %f, Char: %k",
    &[FmtElm::Double(2.5), FmtElm::Char(65)],
  );
  // %f 为 vsnprintf 语义: 默认精度 6 位小数 (原版 base::SNPrintF)
  assert_eq!(ss.to_string_lossy(), "Float: 2.500000, Char: A");
  OK
}

#[test]
fn string_stream_fixed_truncation() -> aok::Void {
  let mut buf = [0u8; 16];
  let mut alloc = FixedStringAllocator::new(&mut buf);
  let mut ss = StringStream::new(&mut alloc);

  // 写入超出容量触发 "...\n" 截断
  ss.add("This is a very long string that must be truncated.", &[]);
  let s = ss.to_string_lossy();
  assert!(
    s.ends_with("...\n"),
    "Expected truncation marker '...\\n', got: {s}"
  );
  OK
}

#[test]
fn string_stream_tiny_buffer_truncation() -> aok::Void {
  // 小于 5 字节缓冲截断防护: 不发生减法溢出 panic
  let mut buf = [0u8; 3];
  let mut alloc = FixedStringAllocator::new(&mut buf);
  let mut ss = StringStream::new(&mut alloc);

  ss.add("hello", &[]);
  assert_eq!(ss.length(), 2);
  OK
}
