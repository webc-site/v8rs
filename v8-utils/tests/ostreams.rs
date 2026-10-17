// 上游无 ostreams 单测; 断言取自 ostreams.h 结构体文档中的示例
// (AsHex(23, 3, true) → "0x017", AsHexBytes(0x231712, 4) → "12 17 23 00")。
use aok::{OK, Void};
use v8_utils::ostreams::{
  AsEscapedUC16ForJSON, AsHex, AsHexBytes, AsReversiblyEscapedUC16, AsUC16, AsUC32, ByteOrder,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn as_hex_display() -> Void {
  // 文档示例
  assert_eq!(AsHex::new(23, 3, true).to_string(), "0x017");
  // 无前缀 + 默认宽度
  assert_eq!(AsHex::new(0xFF, 1, false).to_string(), "ff");
  // min_width 与值均为 0 → 空串 (无前缀)
  assert_eq!(AsHex::new(0, 0, false).to_string(), "");
  // min_width 与值均为 0 且带前缀 → 仅前缀 (上游 snprintf "%s%.0x" 行为)
  assert_eq!(AsHex::new(0, 0, true).to_string(), "0x");
  // 宽度补零
  assert_eq!(AsHex::new(1, 4, false).to_string(), "0001");
  // 指针宽度 (kSystemPointerHexDigits: 64 位 → 12 位十六进制)
  assert_eq!(AsHex::address(0x1234).to_string(), "0x000000001234");
  OK
}

#[test]
fn as_hex_bytes_display() -> Void {
  // 文档示例: 小端字节分隔
  assert_eq!(
    AsHexBytes::new(0x231712, 4, ByteOrder::LittleEndian).to_string(),
    "12 17 23 00"
  );
  // 大端
  assert_eq!(
    AsHexBytes::new(0x231712, 4, ByteOrder::BigEndian).to_string(),
    "00 23 17 12"
  );
  // min_bytes 与值均为 0 → 空串
  assert_eq!(
    AsHexBytes::new(0, 0, ByteOrder::LittleEndian).to_string(),
    ""
  );
  // 位数按值自动扩展
  assert_eq!(
    AsHexBytes::new(0x1FF, 1, ByteOrder::LittleEndian).to_string(),
    "ff 01"
  );
  OK
}

#[test]
fn uc16_uc32_escapes() -> Void {
  // AsUC16: 可打印原样, 否则 \xNN / \uNNNN
  assert_eq!(AsUC16(b'A' as u16).to_string(), "A");
  assert_eq!(AsUC16(0x01).to_string(), "\\x01");
  assert_eq!(AsUC16(0x100).to_string(), "\\u0100");
  // AsReversiblyEscapedUC16: '\\' 被转义
  assert_eq!(AsReversiblyEscapedUC16(b'\\' as u16).to_string(), "\\x5c");
  assert_eq!(AsReversiblyEscapedUC16(b' ' as u16).to_string(), " ");
  // AsEscapedUC16ForJSON: \n \r \t " 定字符
  assert_eq!(AsEscapedUC16ForJSON(b'\n' as u16).to_string(), "\\n");
  assert_eq!(AsEscapedUC16ForJSON(b'\r' as u16).to_string(), "\\r");
  assert_eq!(AsEscapedUC16ForJSON(b'\t' as u16).to_string(), "\\t");
  assert_eq!(AsEscapedUC16ForJSON(b'"' as u16).to_string(), "\\\"");
  // JSON 禁 \xNN: 一律 \uNNNN
  assert_eq!(AsEscapedUC16ForJSON(0x01).to_string(), "\\u0001");
  // AsUC32: 超 BMP 走 \u{NNNNNN}
  assert_eq!(AsUC32(0x10FFFF).to_string(), "\\u{10ffff}");
  assert_eq!(AsUC32(0x20).to_string(), " ");
  OK
}
