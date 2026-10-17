// 上游 hex-format 无独立单测 (snapshot 序列化间接使用); 补两个形态的关键路径。
use aok::{OK, Void};
use v8_utils::hex_format::{format_bytes_to_hex, format_bytes_to_hex_buf};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// String 形态: 空输入空串 / 常规两位小写 / 0x00 与 0xff 边界
#[test]
fn format_bytes_to_hex_basic() -> Void {
  assert_eq!(format_bytes_to_hex(&[]), "");
  assert_eq!(format_bytes_to_hex(&[0x00]), "00");
  assert_eq!(format_bytes_to_hex(&[0x0e, 0xff]), "0eff");
  assert_eq!(format_bytes_to_hex(b"abc"), "616263");
  OK
}

// 缓冲形态: 与 String 形态逐字节一致 (上游同源实现)
#[test]
fn format_bytes_to_hex_buf_matches() -> Void {
  const VAL: &[u8] = &[0x00, 0x0e, 0xff, 0x61];
  let mut buf = [0u8; VAL.len() * 2];
  format_bytes_to_hex_buf(&mut buf, VAL);
  assert_eq!(&buf[..], format_bytes_to_hex(VAL).as_bytes());
  // 恰好 2x 容量
  let mut tight = [0u8; 2];
  format_bytes_to_hex_buf(&mut tight, &[0xab]);
  assert_eq!(&tight, b"ab");
  OK
}
