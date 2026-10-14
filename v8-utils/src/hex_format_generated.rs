// 由 tool/src/utils/hex-format.h.js 从 v8/src/utils/hex-format.{h,cc} 生成; 修订就地改, 复跑覆盖
// 字节十六进制格式化骨架

/// 字节→小写十六进制格式化 (原版 FormatBytesToHex)
pub mod hex_format_skel {
  /// 防溢出上界: 输入长 < 0x20000000 (格式化后 0x40000000 才溢出)
  pub const MAX_VAL_SIZE: usize = 0x2000_0000;

  /// 每字节产出 2 字符 (原版 size_of_formatted >= size_of_val * 2)
  pub const CHARS_PER_BYTE: usize = 2;

  /// 格式化骨架 (语义见手写 hex_format.rs; %02x 小写补零)
  pub fn format_bytes_to_hex(_val: &[u8], _formatted: &mut [u8]) {
    unimplemented!("骨架")
  }

  /// 输出缓冲宽需求
  #[must_use]
  pub const fn formatted_size(val_len: usize) -> usize {
    val_len * CHARS_PER_BYTE
  }
}
