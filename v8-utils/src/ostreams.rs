// 由 tool/src/utils/consts.js 从 v8/src/utils/ostreams.h;src/utils/ostreams.cc 生成; 修订就地改, 复跑覆盖

//! 输出格式化包装器, 移植自 v8/src/utils/ostreams。
//!
//! 上游围绕 std::ostream 的 operator<<; Rust 版将 AsHex/AsUC16 等包装结构
//! 实现为 `Display`。FILE*/调试器流 (OFStream/DbgStreamBuf/AndroidLogStream)
//! 在 Rust 中由 std::io 承担, 见 README 映射表。

use std::fmt;

/// AsHexBytes 的字节序 (ByteOrder)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ByteOrder {
  /// 小端 (默认)
  #[default]
  LittleEndian,
  /// 大端
  BigEndian,
}

// ---- locale 无关谓词 (ostreams.cc:121-125) ----

/// IsPrint (0x20..=0x7E)。
const fn is_print(c: u16) -> bool {
  c >= 0x20 && c <= 0x7E
}

/// IsSpace (0x9..=0xD 或 0x20)。
const fn is_space(c: u16) -> bool {
  (c >= 0x9 && c <= 0xD) || c == 0x20
}

/// IsOK = 可打印或空白, 且非反斜杠。
const fn is_ok(c: u16) -> bool {
  (is_print(c) || is_space(c)) && c != b'\\' as u16
}

/// PrintUC16 (ostreams.cc:127): 谓词通过原样, 否则 \xNN / \uNNNN (小写 hex)。
fn print_uc16(f: &mut fmt::Formatter<'_>, c: u16, pred: fn(u16) -> bool) -> fmt::Result {
  if pred(c) {
    write!(f, "{}", char::from_u32(u32::from(c)).unwrap_or('\u{FFFD}'))
  } else if c <= 0xFF {
    write!(f, "\\x{c:02x}")
  } else {
    write!(f, "\\u{c:04x}")
  }
}

/// PrintUC16ForJSON (ostreams.cc:134): JSON 禁 \xNN, 一律 \uNNNN。
fn print_uc16_for_json(f: &mut fmt::Formatter<'_>, c: u16, pred: fn(u16) -> bool) -> fmt::Result {
  if pred(c) {
    write!(f, "{}", char::from_u32(u32::from(c)).unwrap_or('\u{FFFD}'))
  } else {
    write!(f, "\\u{c:04x}")
  }
}

/// 64 位系统指针十六进制宽 (kSystemPointerHexDigits, v8-internal.h: 压缩指针 12 位)。
pub const SYSTEM_POINTER_HEX_DIGITS: u8 = 12;

/// `AsUC16` (ostreams.h:112): uc16 消歧包装, 可打印原样输出。
#[derive(Debug, Clone, Copy)]
pub struct AsUC16(pub u16);

impl fmt::Display for AsUC16 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    print_uc16(f, self.0, is_print)
  }
}

/// `AsReversiblyEscapedUC16` (ostreams.h:122): 可逆转义 (反斜杠也转义)。
#[derive(Debug, Clone, Copy)]
pub struct AsReversiblyEscapedUC16(pub u16);

impl fmt::Display for AsReversiblyEscapedUC16 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    print_uc16(f, self.0, is_ok)
  }
}

/// `AsEscapedUC16ForJSON` (ostreams.h:127): \n \r \t \" 定字符 + \uNNNN。
#[derive(Debug, Clone, Copy)]
pub struct AsEscapedUC16ForJSON(pub u16);

impl fmt::Display for AsEscapedUC16ForJSON {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.0 {
      c if c == b'\n' as u16 => f.write_str("\\n"),
      c if c == b'\r' as u16 => f.write_str("\\r"),
      c if c == b'\t' as u16 => f.write_str("\\t"),
      c if c == b'"' as u16 => f.write_str("\\\""),
      c => print_uc16_for_json(f, c, is_ok),
    }
  }
}

/// `AsUC32` (ostreams.h:117): 超 BMP 走 `\u{NNNNNN}`。
#[derive(Debug, Clone, Copy)]
pub struct AsUC32(pub i32);

impl fmt::Display for AsUC32 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // String::kMaxUtf16CodeUnit = 0xFFFF
    if self.0 >= 0 && self.0 <= 0xFFFF {
      return print_uc16(f, self.0 as u16, is_print);
    }
    write!(f, "\\u{{{:06x}}}", self.0)
  }
}

/// `AsHex` (ostreams.h:135): 十六进制输出, 最小宽度 + 可选 0x 前缀;
/// `AsHex(23, 3, true)` → "0x017"; min_width 与值均为 0 → 空串。
#[derive(Debug, Clone, Copy)]
pub struct AsHex {
  /// 输出值。
  pub value: u64,
  /// 最小十六进制位宽。
  pub min_width: u8,
  /// 是否带 "0x" 前缀。
  pub with_prefix: bool,
}

impl AsHex {
  /// 构造 (ostreams.h:136 三参默认 min_width=1/无前缀由调用方显式给全)。
  pub const fn new(value: u64, min_width: u8, with_prefix: bool) -> Self {
    Self {
      value,
      min_width,
      with_prefix,
    }
  }

  /// `AsHex::Address` (ostreams.h:142): 指针宽 12 位 + 前缀。
  pub const fn address(a: u64) -> Self {
    Self::new(a, SYSTEM_POINTER_HEX_DIGITS, true)
  }
}

impl fmt::Display for AsHex {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // snprintf("%s%.*lx"): C 的 %.0x 对 0 输出空串, Rust 宽 0 会输出 "0"
    if self.with_prefix {
      f.write_str("0x")?;
    }
    if self.min_width == 0 && self.value == 0 {
      return Ok(());
    }
    write!(
      f,
      "{:0width$x}",
      self.value,
      width = self.min_width as usize
    )
  }
}

/// `AsHexBytes` (ostreams.h:151): 按字节十六进制输出, 空格分隔;
/// `AsHexBytes(0x231712, 4)` 小端 → "12 17 23 00"。
#[derive(Debug, Clone, Copy)]
pub struct AsHexBytes {
  /// 输出值。
  pub value: u64,
  /// 最小字节数 (按值自动扩展)。
  pub min_bytes: u8,
  /// 字节序。
  pub byte_order: ByteOrder,
}

impl AsHexBytes {
  /// 构造 (ostreams.h:153)。
  pub const fn new(value: u64, min_bytes: u8, byte_order: ByteOrder) -> Self {
    Self {
      value,
      min_bytes,
      byte_order,
    }
  }
}

impl fmt::Display for AsHexBytes {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut bytes = self.min_bytes;
    while bytes < 8 && (self.value >> (u32::from(bytes) * 8)) != 0 {
      bytes += 1;
    }
    for b in 0..bytes {
      if b > 0 {
        f.write_str(" ")?;
      }
      let printed_byte = match self.byte_order {
        ByteOrder::LittleEndian => b,
        ByteOrder::BigEndian => bytes - b - 1,
      };
      write!(
        f,
        "{:02x}",
        (self.value >> (u32::from(printed_byte) * 8)) & 0xFF
      )?;
    }
    Ok(())
  }
}
