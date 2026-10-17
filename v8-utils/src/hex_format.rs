// 由 tool/src/utils/consts.js 从 v8/src/utils/hex-format.h;src/utils/hex-format.cc 生成; 修订就地改, 复跑覆盖

//! 字节序列转十六进制串, 移植自 v8/src/utils/hex-format。
//!
//! 上游写入调用方缓冲 (formatted); Rust 版直接返回 `String`,
//! 缓冲容量契约 (>= 2x) 由返回值分配天然满足。

/// 单字节 → 两位小写十六进制 ("0e")
#[inline]
fn hex_byte(b: u8) -> [u8; 2] {
  const HEX: &[u8; 16] = b"0123456789abcdef";
  [HEX[(b >> 4) as usize], HEX[(b & 0xf) as usize]]
}

/// FormatBytesToHex: 字节数组 → 十六进制串
///
/// 上游 CHECK: size_of_val < 0x20000000 (防 2x 溢出), 超限视为契约违约。
pub fn format_bytes_to_hex(val: &[u8]) -> String {
  assert!(val.len() < 0x2000_0000, "hex 输入超长");
  // 单次遍历直出, 无中间分配; HEX 表恒 ASCII, as char 安全
  let mut out = String::with_capacity(val.len() * 2);
  for &b in val {
    let [hi, lo] = hex_byte(b);
    out.push(hi as char);
    out.push(lo as char);
  }
  out
}

/// FormatBytesToHex 的缓冲写入形态 (对齐上游签名)
pub fn format_bytes_to_hex_buf(formatted: &mut [u8], val: &[u8]) {
  assert!(val.len() < 0x2000_0000, "hex 输入超长");
  assert!(formatted.len() >= val.len() * 2, "hex 缓冲不足");
  for (i, &b) in val.iter().enumerate() {
    formatted[i * 2..i * 2 + 2].copy_from_slice(&hex_byte(b));
  }
}
