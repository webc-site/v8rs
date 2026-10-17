// 迁移自 v8/test/unittests/strings/unicode-unittest.cc
// 测试名保留原名: Utf16BufferReuse / SurrogateOverrunsBuffer /
// IncrementalUTF8DecodingVsNonIncrementalUtf8Decoding
// 黄金数据: tests/unicode_cases.rs (由 tool/src/strings/unittest-data.js 生成)

mod unicode_cases;

use aok::OK;
use unicode_cases::CASES;
use v8_strings::{
  unicode::{utf8, utf16},
  unicode_decoder::Utf8Decoder,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// DecodeNormally: 逐码点 value_of。
fn decode_normally(bytes: &[u8]) -> Vec<u32> {
  let mut output = Vec::new();
  let mut cursor = 0usize;
  while cursor < bytes.len() {
    let rest = &bytes[cursor..];
    output.push(utf8::value_of(rest, &mut cursor));
  }
  output
}

/// DecodeUtf16: Utf8Decoder 预扫描 + 一次性解码回码点。
fn decode_utf16(bytes: &[u8]) -> Vec<u32> {
  let decoder = Utf8Decoder::new(bytes);
  let mut utf16_buf = vec![0u16; decoder.utf16_length() as usize];
  decoder.decode_to_two_byte(&mut utf16_buf, bytes);

  // 合并代理对回码点
  let mut output = Vec::new();
  let mut i = 0usize;
  while i < utf16_buf.len() {
    let b = utf16_buf[i];
    if utf16::is_lead_surrogate(i32::from(b)) && i + 1 < utf16_buf.len() {
      i += 1;
      output.push(utf16::combine_surrogate_pair(
        u32::from(b),
        u32::from(utf16_buf[i]),
      ));
    } else {
      output.push(u32::from(b));
    }
    i += 1;
  }
  output
}

/// DecodeIncrementally: 增量解码 + 收尾。
fn decode_incrementally(bytes: &[u8]) -> Vec<u32> {
  use v8_strings::utf8_dfa::strict::State;
  let mut output = Vec::new();
  let mut buffer = 0u32;
  let mut state = State::Accept;
  let mut cursor = 0usize;
  while cursor < bytes.len() {
    let result = utf8::value_of_incremental(bytes, &mut cursor, &mut state, &mut buffer);
    if result != utf8::INCOMPLETE {
      output.push(result);
    }
  }
  let result = utf8::value_of_incremental_finish(&mut state);
  if result != utf8::BUFFER_EMPTY {
    output.push(result);
  }
  output
}

// TEST(UnicodeTest, Utf16BufferReuse)
#[test]
fn utf16_buffer_reuse() -> aok::Void {
  // 首个非 ASCII 前的各种边界序列 (含 4 字节代理对)
  let data: &[(&[u8], &[u32])] = &[
    (&[0x00], &[0x0]),
    (&[0xC2, 0x80], &[0x80]),
    (&[0xE0, 0xA0, 0x80], &[0x800]),
    (&[0xF0, 0x90, 0x80, 0x80], &[0x10000]),
    (&[0xE0, 0xA0, 0x80], &[0x800]),
    (&[0xC2, 0x80], &[0x80]),
    (&[0x00], &[0x0]),
  ];
  for (bytes, expected) in data {
    let output = decode_utf16(bytes);
    assert_eq!(output.len(), expected.len(), "bytes={bytes:02x?}");
    for (got, want) in output.iter().zip(expected.iter()) {
      assert_eq!(got, want);
    }
  }
  OK
}

// TEST(UnicodeTest, SurrogateOverrunsBuffer)
#[test]
fn surrogate_overruns_buffer() -> aok::Void {
  // 序列尾 continuation 不足
  let output = decode_utf16(&[0x00, 0xF0, 0x90, 0x80, 0x80, 0x00]);
  assert_eq!(output[0], 0x00);
  assert_eq!(output[1], 0x10000);
  assert_eq!(output[2], 0x00);
  OK
}

// TEST(UnicodeTest, IncrementalUTF8DecodingVsNonIncrementalUtf8Decoding)
#[test]
fn incremental_utf8_decoding_vs_non_incremental_utf8_decoding() -> aok::Void {
  // V8 有两个 UTF-8 解码器, 本测试校验三者 (普通/增量/utf16 预扫描) 产出一致
  for (bytes, expected) in CASES {
    let output_normal = decode_normally(bytes);
    assert_eq!(
      output_normal.len(),
      expected.len(),
      "normal bytes={bytes:02x?}"
    );
    assert_eq!(
      output_normal.as_slice(),
      *expected,
      "normal bytes={bytes:02x?}"
    );

    let output_incremental = decode_incrementally(bytes);
    assert_eq!(
      output_incremental.len(),
      expected.len(),
      "incremental bytes={bytes:02x?}"
    );
    assert_eq!(
      output_incremental.as_slice(),
      *expected,
      "incremental bytes={bytes:02x?}"
    );

    let output_utf16 = decode_utf16(bytes);
    assert_eq!(
      output_utf16.len(),
      expected.len(),
      "utf16 bytes={bytes:02x?}"
    );
    assert_eq!(
      output_utf16.as_slice(),
      *expected,
      "utf16 bytes={bytes:02x?}"
    );
  }
  OK
}

#[test]
fn write_leading_ascii_two_byte_test() -> aok::Void {
  let mut dest = [0u8; 16];

  // 全 ASCII 序列
  let src1: [u16; 5] = [
    b'h' as u16,
    b'e' as u16,
    b'l' as u16,
    b'l' as u16,
    b'o' as u16,
  ];
  let written1 = utf8::write_leading_ascii_two_byte(&src1, &mut dest);
  assert_eq!(written1, 5);
  assert_eq!(&dest[..5], b"hello");

  // 中间含非 ASCII 字符 (U+4F60 你) 截断
  let src2: [u16; 5] = [b'a' as u16, b'b' as u16, 0x4f60, b'c' as u16, b'd' as u16];
  let written2 = utf8::write_leading_ascii_two_byte(&src2, &mut dest);
  assert_eq!(written2, 2);
  assert_eq!(&dest[..2], b"ab");

  // 起始即为非 ASCII 字符
  let src3: [u16; 3] = [0x0100, b'x' as u16, b'y' as u16];
  let written3 = utf8::write_leading_ascii_two_byte(&src3, &mut dest);
  assert_eq!(written3, 0);

  // 目标缓冲区过短截断
  let mut small_dest = [0u8; 3];
  let written4 = utf8::write_leading_ascii_two_byte(&src1, &mut small_dest);
  assert_eq!(written4, 3);
  assert_eq!(&small_dest, b"hel");

  OK
}
