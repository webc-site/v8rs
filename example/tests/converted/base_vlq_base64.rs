// 复刻 v8/test/unittests/base/vlq-base64-unittest.cc（1:1）
use std::{cell::RefCell, rc::Rc};

use aok::{OK, Void};
// 同名符号可能被多个 TU 重复产出，走精确模块路径避免 glob 歧义
use v8rs::v8::base_vlq_base64::{VLQBase64Decode_22, charToDigitDecodeForTesting_21};
use v8rs::{AsPointer, Ptr, Value};

fn u8_ptr(data: &[u8]) -> Rc<RefCell<Box<[u8]>>> {
  Rc::new(RefCell::new(data.to_vec().into_boxed_slice()))
}

// 对应 EXPECT_EQ(expected, charToDigitDecodeForTesting(i))，逐字节 0..=255
#[test]
fn char_to_digit() -> Void {
  const kSyms: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  for i in 0..256usize {
    let expected: i8 = match kSyms.iter().position(|&s| s == i as u8) {
      Some(p) if i != 0 => p as i8,
      _ => -1,
    };
    assert_eq!(expected, charToDigitDecodeForTesting_21(i as u8), "i={i}");
  }
  OK
}

// 对应 TestVLQBase64Decode：反复从同一 pos 续读，比对 (pos, result)
fn decode_seq(str_: &str, expected: &[(usize, i32)]) {
  let buf = u8_ptr(str_.as_bytes());
  let sz = buf.borrow().len();
  let pos: Value<usize> = Rc::new(RefCell::new(0));
  for &(expect_pos, expect_result) in expected {
    let result = VLQBase64Decode_22(buf.as_pointer(), sz, pos.as_pointer());
    assert_eq!(expect_result, result, "str={str_} expect_pos={expect_pos}");
    assert_eq!(expect_pos, *pos.borrow(), "str={str_}");
  }
}

const INT32_MIN: i32 = i32::MIN;
const INT32_MAX: i32 = i32::MAX;

#[test]
fn decode_one_segment() -> Void {
  decode_seq("", &[(0, INT32_MIN)]);
  // Unsupported symbol.
  decode_seq("*", &[(0, INT32_MIN)]);
  decode_seq("&", &[(0, INT32_MIN)]);
  decode_seq("kt:", &[(2, INT32_MIN)]);
  decode_seq("k^C", &[(1, INT32_MIN)]);
  // Imcomplete string.
  decode_seq("kth4yp", &[(6, INT32_MIN)]);
  // Interpretable strings.
  decode_seq("A", &[(1, 0)]);
  decode_seq("C", &[(1, 1)]);
  decode_seq("Y", &[(1, 12)]);
  decode_seq("2H", &[(2, 123)]);
  decode_seq("ktC", &[(3, 1234)]);
  decode_seq("yjY", &[(3, 12345)]);
  decode_seq("gkxH", &[(4, 123456)]);
  decode_seq("uorrC", &[(5, 1234567)]);
  decode_seq("80wxX", &[(5, 12345678)]);
  decode_seq("qxmvrH", &[(6, 123456789)]);
  decode_seq("kth4ypC", &[(7, 1234567890)]);
  decode_seq("+/////D", &[(7, INT32_MAX)]);
  decode_seq("D", &[(1, -1)]);
  decode_seq("Z", &[(1, -12)]);
  decode_seq("3H", &[(2, -123)]);
  decode_seq("ltC", &[(3, -1234)]);
  decode_seq("zjY", &[(3, -12345)]);
  decode_seq("hkxH", &[(4, -123456)]);
  decode_seq("vorrC", &[(5, -1234567)]);
  decode_seq("90wxX", &[(5, -12345678)]);
  decode_seq("rxmvrH", &[(6, -123456789)]);
  decode_seq("lth4ypC", &[(7, -1234567890)]);
  decode_seq("//////D", &[(7, -INT32_MAX)]);
  // An overflowed value 12345678901 (0x2DFDC1C35).
  decode_seq("qjuw7/2A", &[(6, INT32_MIN)]);
  // An overflowed value 123456789012 (0x1CBE991A14).
  decode_seq("ohtkz+lH", &[(6, INT32_MIN)]);
  // An overflowed value 4294967296 (0x100000000).
  decode_seq("ggggggE", &[(6, INT32_MIN)]);
  // An overflowed value -12345678901, |value| = (0x2DFDC1C35).
  decode_seq("rjuw7/2A", &[(6, INT32_MIN)]);
  // An overflowed value -123456789012, |value| = (0x1CBE991A14).
  decode_seq("phtkz+lH", &[(6, INT32_MIN)]);
  // An overflowed value -4294967296, |value| = (0x100000000).
  decode_seq("hgggggE", &[(6, INT32_MIN)]);
  OK
}

#[test]
fn decode_two_segment() -> Void {
  decode_seq("AA", &[(1, 0), (2, 0)]);
  decode_seq("KA", &[(1, 5), (2, 0)]);
  decode_seq("AQ", &[(1, 0), (2, 8)]);
  decode_seq("MG", &[(1, 6), (2, 3)]);
  decode_seq("a4E", &[(1, 13), (3, 76)]);
  decode_seq("4GyO", &[(2, 108), (4, 233)]);
  decode_seq("ggEqnD", &[(3, 2048), (6, 1653)]);
  decode_seq("g2/D0ilF", &[(4, 65376), (8, 84522)]);
  decode_seq("ss6gBy0m3B", &[(5, 537798), (10, 904521)]);
  decode_seq("LA", &[(1, -5), (2, 0)]);
  decode_seq("AR", &[(1, 0), (2, -8)]);
  decode_seq("NH", &[(1, -6), (2, -3)]);
  decode_seq("b5E", &[(1, -13), (3, -76)]);
  decode_seq("5GzO", &[(2, -108), (4, -233)]);
  decode_seq("hgErnD", &[(3, -2048), (6, -1653)]);
  decode_seq("h2/D1ilF", &[(4, -65376), (8, -84522)]);
  decode_seq("ts6gBz0m3B", &[(5, -537798), (10, -904521)]);
  decode_seq("4GzO", &[(2, 108), (4, -233)]);
  decode_seq("ggErnD", &[(3, 2048), (6, -1653)]);
  decode_seq("g2/D1ilF", &[(4, 65376), (8, -84522)]);
  decode_seq("ss6gBz0m3B", &[(5, 537798), (10, -904521)]);
  decode_seq("5GyO", &[(2, -108), (4, 233)]);
  decode_seq("hgEqnD", &[(3, -2048), (6, 1653)]);
  decode_seq("h2/D0ilF", &[(4, -65376), (8, 84522)]);
  decode_seq("ts6gBy0m3B", &[(5, -537798), (10, 904521)]);
  OK
}

#[test]
fn decode_four_segment() -> Void {
  decode_seq("AAAA", &[(1, 0), (2, 0), (3, 0), (4, 0)]);
  decode_seq("QADA", &[(1, 8), (2, 0), (3, -1), (4, 0)]);
  decode_seq("ECQY", &[(1, 2), (2, 1), (3, 8), (4, 12)]);
  decode_seq(
    "goGguCioPk9I",
    &[(3, 3200), (6, 1248), (9, 7809), (12, 4562)],
  );
  decode_seq("6/BACA", &[(3, 1021), (4, 0), (5, 1), (6, 0)]);
  decode_seq("urCAQA", &[(3, 1207), (4, 0), (5, 8), (6, 0)]);
  decode_seq("sDACA", &[(2, 54), (3, 0), (4, 1), (5, 0)]);
  OK
}
