//! 原版 v8/test/unittests/base/bignum-unittest.cc 1:1 迁移
//!
//! 断言规约守恒: 13 个 TEST_F 全部落地, 黄金十六进制串逐字符保留。
//! Gay 数据不涉本文件。

use std::str::from_utf8;

use aok::{OK, Void};
use v8_base::numbers::bignum::Bignum;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// ToHexString 成功并取 NUL 前字符串 (复用缓冲, 与原版 char buffer[kBufferSize] 同法)
fn hex_of<'a>(bignum: &Bignum, buf: &'a mut [u8]) -> &'a str {
  assert!(bignum.to_hex_string(buf), "ToHexString 失败");
  let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
  from_utf8(&buf[..end]).unwrap_or("?")
}

fn assign_hex(bignum: &mut Bignum, s: &str) {
  bignum.assign_hex_string(s.as_bytes());
}

fn assign_decimal(bignum: &mut Bignum, s: &str) {
  bignum.assign_decimal_string(s.as_bytes());
}

/// BignumTest.Assign
#[test]
fn bignum_test_assign() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();
  let mut bignum2 = Bignum::new();
  bignum.assign_uint16(0);
  assert_eq!(hex_of(&bignum, &mut buf), "0");
  bignum.assign_uint16(0xA);
  assert_eq!(hex_of(&bignum, &mut buf), "A");
  bignum.assign_uint16(0x20);
  assert_eq!(hex_of(&bignum, &mut buf), "20");

  bignum.assign_uint64(0);
  assert_eq!(hex_of(&bignum, &mut buf), "0");
  bignum.assign_uint64(0xA);
  assert_eq!(hex_of(&bignum, &mut buf), "A");
  bignum.assign_uint64(0x20);
  assert_eq!(hex_of(&bignum, &mut buf), "20");
  bignum.assign_uint64(0x100);
  assert_eq!(hex_of(&bignum, &mut buf), "100");

  // 首个跨 bigit 用例
  bignum.assign_uint64(0x12345678);
  assert_eq!(hex_of(&bignum, &mut buf), "12345678");

  bignum.assign_uint64(0xFFFF_FFFF_FFFF_FFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFFFFFFFFFFFF");

  bignum.assign_uint64(0x1234_5678_9ABC_DEF0);
  assert_eq!(hex_of(&bignum, &mut buf), "123456789ABCDEF0");

  bignum2.assign_bignum(&bignum);
  assert_eq!(hex_of(&bignum2, &mut buf), "123456789ABCDEF0");

  assign_decimal(&mut bignum, "0");
  assert_eq!(hex_of(&bignum, &mut buf), "0");

  assign_decimal(&mut bignum, "1");
  assert_eq!(hex_of(&bignum, &mut buf), "1");

  assign_decimal(&mut bignum, "1234567890");
  assert_eq!(hex_of(&bignum, &mut buf), "499602D2");

  assign_hex(&mut bignum, "0");
  assert_eq!(hex_of(&bignum, &mut buf), "0");

  assign_hex(&mut bignum, "123456789ABCDEF0");
  assert_eq!(hex_of(&bignum, &mut buf), "123456789ABCDEF0");
  OK
}

/// BignumTest.ShiftLeft
#[test]
fn bignum_test_shift_left() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();
  assign_hex(&mut bignum, "0");
  bignum.shift_left(100);
  assert_eq!(hex_of(&bignum, &mut buf), "0");

  assign_hex(&mut bignum, "1");
  bignum.shift_left(1);
  assert_eq!(hex_of(&bignum, &mut buf), "2");

  assign_hex(&mut bignum, "1");
  bignum.shift_left(4);
  assert_eq!(hex_of(&bignum, &mut buf), "10");

  assign_hex(&mut bignum, "1");
  bignum.shift_left(32);
  assert_eq!(hex_of(&bignum, &mut buf), "100000000");

  assign_hex(&mut bignum, "1");
  bignum.shift_left(64);
  assert_eq!(hex_of(&bignum, &mut buf), "10000000000000000");

  assign_hex(&mut bignum, "123456789ABCDEF");
  bignum.shift_left(64);
  assert_eq!(hex_of(&bignum, &mut buf), "123456789ABCDEF0000000000000000");
  bignum.shift_left(1);
  assert_eq!(hex_of(&bignum, &mut buf), "2468ACF13579BDE0000000000000000");
  OK
}

/// BignumTest.AddUInt64
#[test]
fn bignum_test_add_uint64() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();
  assign_hex(&mut bignum, "0");
  bignum.add_uint64(0xA);
  assert_eq!(hex_of(&bignum, &mut buf), "A");

  assign_hex(&mut bignum, "1");
  bignum.add_uint64(0xA);
  assert_eq!(hex_of(&bignum, &mut buf), "B");

  assign_hex(&mut bignum, "1");
  bignum.add_uint64(0x100);
  assert_eq!(hex_of(&bignum, &mut buf), "101");

  assign_hex(&mut bignum, "1");
  bignum.add_uint64(0xFFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "10000");

  assign_hex(&mut bignum, "FFFFFFF");
  bignum.add_uint64(0x1);
  assert_eq!(hex_of(&bignum, &mut buf), "10000000");

  assign_hex(&mut bignum, "10000000000000000000000000000000000000000000");
  bignum.add_uint64(0xFFFF);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "1000000000000000000000000000000000000000FFFF"
  );

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
  bignum.add_uint64(0x1);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "100000000000000000000000000000000000000000000"
  );

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.add_uint64(1);
  assert_eq!(hex_of(&bignum, &mut buf), "10000000000000000000000001");

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.add_uint64(0xFFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "1000000000000000000000FFFF");

  assign_hex(&mut bignum, "0");
  bignum.add_uint64(0xA_0000_0000);
  assert_eq!(hex_of(&bignum, &mut buf), "A00000000");

  assign_hex(&mut bignum, "1");
  bignum.add_uint64(0xA_0000_0000);
  assert_eq!(hex_of(&bignum, &mut buf), "A00000001");

  assign_hex(&mut bignum, "1");
  bignum.add_uint64(0x100_0000_0000);
  assert_eq!(hex_of(&bignum, &mut buf), "10000000001");

  assign_hex(&mut bignum, "1");
  bignum.add_uint64(0xFFFF_0000_0000);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFF00000001");

  assign_hex(&mut bignum, "FFFFFFF");
  bignum.add_uint64(0x1_0000_0000);
  assert_eq!(hex_of(&bignum, &mut buf), "10FFFFFFF");

  assign_hex(&mut bignum, "10000000000000000000000000000000000000000000");
  bignum.add_uint64(0xFFFF_0000_0000);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "10000000000000000000000000000000FFFF00000000"
  );

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
  bignum.add_uint64(0x1_0000_0000);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "1000000000000000000000000000000000000FFFFFFFF"
  );

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.add_uint64(0x1_0000_0000);
  assert_eq!(hex_of(&bignum, &mut buf), "10000000000000000100000000");

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.add_uint64(0xFFFF_0000_0000);
  assert_eq!(hex_of(&bignum, &mut buf), "10000000000000FFFF00000000");
  OK
}

/// BignumTest.AddBignum
#[test]
fn bignum_test_add_bignum() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();
  let mut other = Bignum::new();

  assign_hex(&mut other, "1");
  assign_hex(&mut bignum, "0");
  bignum.add_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "1");

  assign_hex(&mut bignum, "1");
  bignum.add_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "2");

  assign_hex(&mut bignum, "FFFFFFF");
  bignum.add_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "10000000");

  assign_hex(&mut bignum, "FFFFFFFFFFFFFF");
  bignum.add_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "100000000000000");

  assign_hex(&mut bignum, "10000000000000000000000000000000000000000000");
  bignum.add_bignum(&other);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "10000000000000000000000000000000000000000001"
  );

  assign_hex(&mut other, "1000000000000");

  assign_hex(&mut bignum, "1");
  bignum.add_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "1000000000001");

  assign_hex(&mut bignum, "FFFFFFF");
  bignum.add_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "100000FFFFFFF");

  assign_hex(&mut bignum, "10000000000000000000000000000000000000000000");
  bignum.add_bignum(&other);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "10000000000000000000000000000001000000000000"
  );

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
  bignum.add_bignum(&other);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "1000000000000000000000000000000FFFFFFFFFFFF"
  );

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.add_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "10000000000001000000000000");

  other.shift_left(64);
  // other == "10000000000000000000000000000"

  bignum.assign_uint16(0x1);
  bignum.add_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "10000000000000000000000000001");

  assign_hex(&mut bignum, "FFFFFFF");
  bignum.add_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "1000000000000000000000FFFFFFF");

  assign_hex(&mut bignum, "10000000000000000000000000000000000000000000");
  bignum.add_bignum(&other);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "10000000000000010000000000000000000000000000"
  );

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
  bignum.add_bignum(&other);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "100000000000000FFFFFFFFFFFFFFFFFFFFFFFFFFFF"
  );

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.add_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "10010000000000000000000000000");
  OK
}

/// BignumTest.SubtractBignum
#[test]
fn bignum_test_subtract_bignum() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();
  let mut other = Bignum::new();

  assign_hex(&mut bignum, "1");
  assign_hex(&mut other, "0");
  bignum.subtract_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "1");

  assign_hex(&mut bignum, "2");
  assign_hex(&mut other, "0");
  bignum.subtract_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "2");

  assign_hex(&mut bignum, "10000000");
  assign_hex(&mut other, "1");
  bignum.subtract_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFFF");

  assign_hex(&mut bignum, "100000000000000");
  assign_hex(&mut other, "1");
  bignum.subtract_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFFFFFFFFFF");

  assign_hex(&mut bignum, "10000000000000000000000000000000000000000001");
  assign_hex(&mut other, "1");
  bignum.subtract_bignum(&other);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "10000000000000000000000000000000000000000000"
  );

  assign_hex(&mut bignum, "1000000000001");
  assign_hex(&mut other, "1000000000000");
  bignum.subtract_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "1");

  assign_hex(&mut bignum, "100000FFFFFFF");
  assign_hex(&mut other, "1000000000000");
  bignum.subtract_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFFF");

  assign_hex(&mut bignum, "10000000000000000000000000000001000000000000");
  assign_hex(&mut other, "1000000000000");
  bignum.subtract_bignum(&other);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "10000000000000000000000000000000000000000000"
  );

  assign_hex(&mut bignum, "1000000000000000000000000000000FFFFFFFFFFFF");
  assign_hex(&mut other, "1000000000000");
  bignum.subtract_bignum(&other);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
  );

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  assign_hex(&mut other, "1000000000000");
  bignum.subtract_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFFFFFFFFF000000000000");

  assign_hex(&mut other, "1000000000000");
  other.shift_left(48);
  // other == "1000000000000000000000000"

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.subtract_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "F000000000000000000000000");

  other.assign_uint16(0x1);
  other.shift_left(35);
  // other == "800000000"
  assign_hex(&mut bignum, "FFFFFFF");
  bignum.shift_left(60);
  bignum.subtract_bignum(&other);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFFEFFFFFF800000000");

  assign_hex(&mut bignum, "10000000000000000000000000000000000000000000");
  bignum.subtract_bignum(&other);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF800000000"
  );

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
  bignum.subtract_bignum(&other);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7FFFFFFFF"
  );
  OK
}

/// BignumTest.MultiplyUInt32
#[test]
fn bignum_test_multiply_uint32() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();

  assign_hex(&mut bignum, "0");
  bignum.multiply_by_uint32(0x25);
  assert_eq!(hex_of(&bignum, &mut buf), "0");

  assign_hex(&mut bignum, "2");
  bignum.multiply_by_uint32(0x5);
  assert_eq!(hex_of(&bignum, &mut buf), "A");

  assign_hex(&mut bignum, "10000000");
  bignum.multiply_by_uint32(0x9);
  assert_eq!(hex_of(&bignum, &mut buf), "90000000");

  assign_hex(&mut bignum, "100000000000000");
  bignum.multiply_by_uint32(0xFFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFF00000000000000");

  assign_hex(&mut bignum, "100000000000000");
  bignum.multiply_by_uint32(0xFFFFFFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFFFF00000000000000");

  assign_hex(&mut bignum, "1234567ABCD");
  bignum.multiply_by_uint32(0xFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "12333335552433");

  assign_hex(&mut bignum, "1234567ABCD");
  bignum.multiply_by_uint32(0xFFFFFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "12345679998A985433");

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFF");
  bignum.multiply_by_uint32(0x2);
  assert_eq!(hex_of(&bignum, &mut buf), "1FFFFFFFFFFFFFFFE");

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFF");
  bignum.multiply_by_uint32(0x4);
  assert_eq!(hex_of(&bignum, &mut buf), "3FFFFFFFFFFFFFFFC");

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFF");
  bignum.multiply_by_uint32(0xF);
  assert_eq!(hex_of(&bignum, &mut buf), "EFFFFFFFFFFFFFFF1");

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFF");
  bignum.multiply_by_uint32(0xFFFFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFEFFFFFFFFFF000001");

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.multiply_by_uint32(2);
  assert_eq!(hex_of(&bignum, &mut buf), "20000000000000000000000000");

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.multiply_by_uint32(0xF);
  assert_eq!(hex_of(&bignum, &mut buf), "F0000000000000000000000000");

  bignum.assign_uint16(0xFFFF);
  bignum.shift_left(100);
  bignum.multiply_by_uint32(0xFFFF);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "FFFE00010000000000000000000000000"
  );

  bignum.assign_uint16(0xFFFF);
  bignum.shift_left(100);
  bignum.multiply_by_uint32(0xFFFFFFFF);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "FFFEFFFF00010000000000000000000000000"
  );

  assign_decimal(&mut bignum, "15611230384529777");
  bignum.multiply_by_uint32(10000000);
  assert_eq!(hex_of(&bignum, &mut buf), "210EDD6D4CDD2580EE80");
  OK
}

/// BignumTest.MultiplyUInt64
#[test]
fn bignum_test_multiply_uint64() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();

  assign_hex(&mut bignum, "0");
  bignum.multiply_by_uint64(0x25);
  assert_eq!(hex_of(&bignum, &mut buf), "0");

  assign_hex(&mut bignum, "2");
  bignum.multiply_by_uint64(0x5);
  assert_eq!(hex_of(&bignum, &mut buf), "A");

  assign_hex(&mut bignum, "10000000");
  bignum.multiply_by_uint64(0x9);
  assert_eq!(hex_of(&bignum, &mut buf), "90000000");

  assign_hex(&mut bignum, "100000000000000");
  bignum.multiply_by_uint64(0xFFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFF00000000000000");

  assign_hex(&mut bignum, "100000000000000");
  bignum.multiply_by_uint64(0xFFFF_FFFF_FFFF_FFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFFFFFFFFFFFF00000000000000");

  assign_hex(&mut bignum, "1234567ABCD");
  bignum.multiply_by_uint64(0xFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "12333335552433");

  assign_hex(&mut bignum, "1234567ABCD");
  bignum.multiply_by_uint64(0xFF_FFFF_FFFF);
  assert_eq!(hex_of(&bignum, &mut buf), "1234567ABCBDCBA985433");

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFF");
  bignum.multiply_by_uint64(0x2);
  assert_eq!(hex_of(&bignum, &mut buf), "1FFFFFFFFFFFFFFFE");

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFF");
  bignum.multiply_by_uint64(0x4);
  assert_eq!(hex_of(&bignum, &mut buf), "3FFFFFFFFFFFFFFFC");

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFF");
  bignum.multiply_by_uint64(0xF);
  assert_eq!(hex_of(&bignum, &mut buf), "EFFFFFFFFFFFFFFF1");

  assign_hex(&mut bignum, "FFFFFFFFFFFFFFFF");
  bignum.multiply_by_uint64(0xFFFF_FFFF_FFFF_FFFF);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "FFFFFFFFFFFFFFFE0000000000000001"
  );

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.multiply_by_uint64(2);
  assert_eq!(hex_of(&bignum, &mut buf), "20000000000000000000000000");

  bignum.assign_uint16(0x1);
  bignum.shift_left(100);
  bignum.multiply_by_uint64(0xF);
  assert_eq!(hex_of(&bignum, &mut buf), "F0000000000000000000000000");

  bignum.assign_uint16(0xFFFF);
  bignum.shift_left(100);
  bignum.multiply_by_uint64(0xFFFF);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "FFFE00010000000000000000000000000"
  );

  bignum.assign_uint16(0xFFFF);
  bignum.shift_left(100);
  bignum.multiply_by_uint64(0xFFFFFFFF);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "FFFEFFFF00010000000000000000000000000"
  );

  bignum.assign_uint16(0xFFFF);
  bignum.shift_left(100);
  bignum.multiply_by_uint64(0xFFFF_FFFF_FFFF_FFFF);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "FFFEFFFFFFFFFFFF00010000000000000000000000000"
  );

  assign_decimal(&mut bignum, "15611230384529777");
  bignum.multiply_by_uint64(0x8AC7_2304_89E8_0000);
  assert_eq!(hex_of(&bignum, &mut buf), "1E10EE4B11D15A7F3DE7F3C7680000");
  OK
}

/// BignumTest.MultiplyPowerOfTen — 含 10^50/100/200/500/1000 黄金串
#[test]
fn bignum_test_multiply_power_of_ten() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();

  // 1234 × 10^1..10^31
  let golden: [(&str, i32); 31] = [
    ("3034", 1),
    ("1E208", 2),
    ("12D450", 3),
    ("BC4B20", 4),
    ("75AEF40", 5),
    ("498D5880", 6),
    ("2DF857500", 7),
    ("1CBB369200", 8),
    ("11F5021B400", 9),
    ("B3921510800", 10),
    ("703B4D2A5000", 11),
    ("4625103A72000", 12),
    ("2BD72A24874000", 13),
    ("1B667A56D488000", 14),
    ("11200C7644D50000", 15),
    ("AB407C9EB0520000", 16),
    ("6B084DE32E3340000", 17),
    ("42E530ADFCE0080000", 18),
    ("29CF3E6CBE0C0500000", 19),
    ("1A218703F6C783200000", 20),
    ("1054F4627A3CB1F400000", 21),
    ("A3518BD8C65EF38800000", 22),
    ("6612F7677BFB5835000000", 23),
    ("3FCBDAA0AD7D17212000000", 24),
    ("27DF68A46C6E2E74B4000000", 25),
    ("18EBA166C3C4DD08F08000000", 26),
    ("F9344E03A5B0A259650000000", 27),
    ("9BC0B0C2478E6577DF20000000", 28),
    ("61586E796CB8FF6AEB740000000", 29),
    ("3CD7450BE3F39FA2D32880000000", 30),
    ("26068B276E7843C5C3F9500000000", 31),
  ];
  for (want, exp) in golden {
    assign_decimal(&mut bignum, "1234");
    bignum.multiply_by_power_of_ten(exp);
    assert_eq!(hex_of(&bignum, &mut buf), want, "10^{exp}");
  }

  assign_decimal(&mut bignum, "1234");
  bignum.multiply_by_power_of_ten(50);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    "149D1B4CFED03B23AB5F4E1196EF45C08000000000000"
  );

  assign_decimal(&mut bignum, "1234");
  bignum.multiply_by_power_of_ten(100);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    concat!(
      "5827249F27165024FBC47DFCA9359BF316332D1B91ACEECF471FBAB06D9B2",
      "0000000000000000000000000"
    )
  );

  assign_decimal(&mut bignum, "1234");
  bignum.multiply_by_power_of_ten(200);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    concat!(
      "64C1F5C06C3816AFBF8DAFD5A3D756365BB0FD020E6F084E759C1F7C99E4F",
      "55B9ACC667CEC477EB958C2AEEB3C6C19BA35A1AD30B35C51EB72040920000",
      "0000000000000000000000000000000000000000000000"
    )
  );

  assign_decimal(&mut bignum, "1234");
  bignum.multiply_by_power_of_ten(500);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    concat!(
      "96741A625EB5D7C91039FEB5C5ACD6D9831EDA5B083D800E6019442C8C8223",
      "3EAFB3501FE2058062221E15121334928880827DEE1EC337A8B26489F3A40A",
      "CB440A2423734472D10BFCE886F41B3AF9F9503013D86D088929CA86EEB4D8",
      "B9C831D0BD53327B994A0326227CFD0ECBF2EB48B02387AAE2D4CCCDF1F1A1",
      "B8CC4F1FA2C56AD40D0E4DAA9C28CDBF0A549098EA13200000000000000000",
      "00000000000000000000000000000000000000000000000000000000000000",
      "0000000000000000000000000000000000000000000000"
    )
  );

  assign_decimal(&mut bignum, "1234");
  bignum.multiply_by_power_of_ten(1000);
  assert_eq!(
    hex_of(&bignum, &mut buf),
    concat!(
      "1258040F99B1CD1CC9819C676D413EA50E4A6A8F114BB0C65418C62D399B81",
      "6361466CA8E095193E1EE97173553597C96673AF67FAFE27A66E7EF2E5EF2E",
      "E3F5F5070CC17FE83BA53D40A66A666A02F9E00B0E11328D2224B8694C7372",
      "F3D536A0AD1985911BD361496F268E8B23112500EAF9B88A9BC67B2AB04D38",
      "7FEFACD00F5AF4F764F9ABC3ABCDE54612DE38CD90CB6647CA389EA0E86B16",
      "BF7A1F34086E05ADBE00BD1673BE00FAC4B34AF1091E8AD50BA675E0381440",
      "EA8E9D93E75D816BAB37C9844B1441C38FC65CF30ABB71B36433AF26DD97BD",
      "ABBA96C03B4919B8F3515B92826B85462833380DC193D79F69D20DD6038C99",
      "6114EF6C446F0BA28CC772ACBA58B81C04F8FFDE7B18C4E5A3ABC51E637FDF",
      "6E37FDFF04C940919390F4FF92000000000000000000000000000000000000",
      "00000000000000000000000000000000000000000000000000000000000000",
      "00000000000000000000000000000000000000000000000000000000000000",
      "00000000000000000000000000000000000000000000000000000000000000",
      "0000000000000000000000000000"
    )
  );

  let bignum2_hex = concat!(
    "3DA774C07FB5DF54284D09C675A492165B830D5DAAEB2A7501",
    "DA17CF9DFA1CA2282269F92A25A97314296B717E3DCBB9FE17",
    "41A842FE2913F540F40796F2381155763502C58B15AF7A7F88",
    "6F744C9164FF409A28F7FA0C41F89ED79C1BE9F322C8578B97",
    "841F1CBAA17D901BE1230E3C00E1C643AF32638B5674E01FEA",
    "96FC90864E621B856A9E1CE56E6EB545B9C2F8F0CC10DDA88D",
    "CC6D282605F8DB67044F2DFD3695E7BA63877AE16701536AE6",
    "567C794D0BFE338DFBB42D92D4215AF3BB22BF0A8B283FDDC2",
    "C667A10958EA6D2"
  );
  let mut bignum2 = Bignum::new();
  assign_hex(&mut bignum2, bignum2_hex);
  assert_eq!(hex_of(&bignum2, &mut buf), bignum2_hex);

  for (exp, want) in [
    (
      1,
      concat!(
        "2688A8F84FD1AB949930261C0986DB4DF931E85A8AD2FA8921284EE1C2BC51",
        "E55915823BBA5789E7EC99E326EEE69F543ECE890929DED9AC79489884BE57",
        "630AD569E121BB76ED8DAC8FB545A8AFDADF1F8860599AFC47A93B6346C191",
        "7237F5BD36B73EB29371F4A4EE7A116CB5E8E5808D1BEA4D7F7E3716090C13",
        "F29E5DDA53F0FD513362A2D20F6505314B9419DB967F8A8A89589FC43917C3",
        "BB892062B17CBE421DB0D47E34ACCCE060D422CFF60DCBD0277EE038BD509C",
        "7BC494D8D854F5B76696F927EA99BC00C4A5D7928434"
      ),
    ),
    (
      2,
      concat!(
        "1815699B31E30B3CDFBE17D185F44910BBBF313896C3DC95B4B9314D19B5B32",
        "F57AD71655476B630F3E02DF855502394A74115A5BA2B480BCBCD5F52F6F69D",
        "E6C5622CB5152A54788BD9D14B896DE8CB73B53C3800DDACC9C51E0C38FAE76",
        "2F9964232872F9C2738E7150C4AE3F1B18F70583172706FAEE26DC5A78C77A2",
        "FAA874769E52C01DA5C3499F233ECF3C90293E0FB69695D763DAA3AEDA5535B",
        "43DAEEDF6E9528E84CEE0EC000C3C8495C1F9C89F6218AF4C23765261CD5ADD",
        "0787351992A01E5BB8F2A015807AE7A6BB92A08"
      ),
    ),
    (
      5,
      concat!(
        "5E13A4863ADEE3E5C9FE8D0A73423D695D62D8450CED15A8C9F368952C6DC3",
        "F0EE7D82F3D1EFB7AF38A3B3920D410AFCAD563C8F5F39116E141A3C5C14B3",
        "58CD73077EA35AAD59F6E24AD98F10D5555ABBFBF33AC361EAF429FD5FBE94",
        "17DA9EF2F2956011F9F93646AA38048A681D984ED88127073443247CCC167C",
        "B354A32206EF5A733E73CF82D795A1AD598493211A6D613C39515E0E0F6304",
        "DCD9C810F3518C7F6A7CB6C81E99E02FCC65E8FDB7B7AE97306CC16A8631CE",
        "0A2AEF6568276BE4C176964A73C153FDE018E34CB4C2F40"
      ),
    ),
    (
      10,
      concat!(
        "8F8CB8EB51945A7E815809F6121EF2F4E61EF3405CD9432CAD2709749EEAFD",
        "1B81E843F14A3667A7BDCCC9E0BB795F63CDFDB62844AC7438976C885A0116",
        "29607DA54F9C023CC366570B7637ED0F855D931752038A614922D0923E382C",
        "B8E5F6C975672DB76E0DE471937BB9EDB11E28874F1C122D5E1EF38CECE9D0",
        "0723056BCBD4F964192B76830634B1D322B7EB0062F3267E84F5C824343A77",
        "4B7DCEE6DD464F01EBDC8C671BB18BB4EF4300A42474A6C77243F2A12B03BF",
        "0443C38A1C0D2701EDB393135AE0DEC94211F9D4EB51F990800"
      ),
    ),
    (
      50,
      concat!(
        "107A8BE345E24407372FC1DE442CBA696BC23C4FFD5B4BDFD9E5C39559815",
        "86628CF8472D2D589F2FC2BAD6E0816EC72CBF85CCA663D8A1EC6C51076D8",
        "2D247E6C26811B7EC4D4300FB1F91028DCB7B2C4E7A60C151161AA7E65E79",
        "B40917B12B2B5FBE7745984D4E8EFA31F9AE6062427B068B144A9CB155873",
        "E7C0C9F0115E5AC72DC5A73C4796DB970BF9205AB8C77A6996EB1B417F9D1",
        "6232431E6313C392203601B9C22CC10DDA88DCC6D282605F8DB67044F2DFD",
        "3695E7BA63877AE16701536AE6567C794D0BFE338DFBB42D924CF964BD2C0",
        "F586E03A2FCD35A408000000000000"
      ),
    ),
    (
      100,
      concat!(
        "46784A90ACD0ED3E7759CC585FB32D36EB6034A6F78D92604E3BAA5ED3D8B",
        "6E60E854439BE448897FB4B7EA5A3D873AA0FCB3CFFD80D0530880E45F511",
        "722A50CE7E058B5A6F5464DB7500E34984EE3202A9441F44FA1554C0CEA96",
        "B438A36F25E7C9D56D71AE2CD313EC37534DA299AC0854FC48591A7CF3171",
        "31265AA4AE62DE32344CE7BEEEF894AE686A2DAAFE5D6D9A10971FFD9C064",
        "5079B209E1048F58B5192D41D84336AC4C8C489EEF00939CFC9D55C122036",
        "01B9C22CC10DDA88DCC6D282605F8DB67044F2DFD3695E7BA3F67B96D3A32",
        "E11FB5561B68744C4035B0800DC166D49D98E3FD1D5BB2000000000000000",
        "0000000000"
      ),
    ),
    (
      200,
      concat!(
        "508BD351221DF139D72D88CDC0416845A53EE2D0E6B98352509A9AC312F8C",
        "6CB1A144889416201E0B6CE66EA3EBE259B5FD79ECFC1FD77963CE516CC7E",
        "2FE73D4B5B710C19F6BCB092C7A2FD76286543B8DBD2C596DFF2C896720BA",
        "DFF7BC9C366ACEA3A880AEC287C5E6207DF2739B5326FC19D773BD830B109",
        "ED36C7086544BF8FDB9D4B73719C2B5BC2F571A5937EC46876CD428281F6B",
        "F287E1E07F25C1B1D46BC37324FF657A8B2E0071DB83B86123CA34004F406",
        "001082D7945E90C6E8C9A9FEC2B44BE0DDA46E9F52B152E4D1336D2FCFBC9",
        "96E30CA0082256737365158FE36482AA7EB9DAF2AB128F10E7551A3CD5BE6",
        "0A922F3A7D5EED38B634A7EC95BCF7021BA6820A292000000000000000000",
        "00000000000000000000000000000000"
      ),
    ),
    (
      500,
      concat!(
        "7845F900E475B5086885BAAAE67C8E85185ACFE4633727F82A4B06B5582AC",
        "BE933C53357DA0C98C20C5AC900C4D76A97247DF52B79F48F9E35840FB715",
        "D392CE303E22622B0CF82D9471B398457DD3196F639CEE8BBD2C146873841",
        "F0699E6C41F04FC7A54B48CEB995BEB6F50FE81DE9D87A8D7F849CC523553",
        "7B7BBBC1C7CAAFF6E9650BE03B308C6D31012AEF9580F70D3EE2083ADE126",
        "8940FA7D6308E239775DFD2F8C97FF7EBD525DAFA6512216F7047A62A93DC",
        "38A0165BDC67E250DCC96A0181DE935A70B38704DC71819F02FC5261FF7E1",
        "E5F11907678B0A3E519FF4C10A867B0C26CE02BE6960BA8621A87303C101C",
        "3F88798BB9F7739655946F8B5744E6B1EAF10B0C5621330F0079209033C69",
        "20DE2E2C8D324F0624463735D482BF291926C22A910F5B80FA25170B6B57D",
        "8D5928C7BCA3FE87461275F69BD5A1B83181DAAF43E05FC3C72C4E93111B6",
        "6205EBF49B28FEDFB7E7526CBDA658A332000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000"
      ),
    ),
  ] {
    bignum.assign_bignum(&bignum2);
    bignum.multiply_by_power_of_ten(exp);
    assert_eq!(hex_of(&bignum, &mut buf), want, "bignum2 × 10^{exp}");
  }
  OK
}

/// BignumTest.DivideModuloIntBignum
#[test]
fn bignum_test_divide_modulo_int_bignum() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();
  let mut other = Bignum::new();
  let mut third = Bignum::new();

  bignum.assign_uint16(10);
  other.assign_uint16(2);
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 5);
  assert_eq!(hex_of(&bignum, &mut buf), "0");

  bignum.assign_uint16(10);
  bignum.shift_left(500);
  other.assign_uint16(2);
  other.shift_left(500);
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 5);
  assert_eq!(hex_of(&bignum, &mut buf), "0");

  bignum.assign_uint16(11);
  other.assign_uint16(2);
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 5);
  assert_eq!(hex_of(&bignum, &mut buf), "1");

  bignum.assign_uint16(10);
  bignum.shift_left(500);
  other.assign_uint16(1);
  bignum.add_bignum(&other);
  other.assign_uint16(2);
  other.shift_left(500);
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 5);
  assert_eq!(hex_of(&bignum, &mut buf), "1");

  bignum.assign_uint16(10);
  bignum.shift_left(500);
  other.assign_bignum(&bignum);
  bignum.multiply_by_uint32(0x1234);
  third.assign_uint16(0xFFF);
  bignum.add_bignum(&third);
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 0x1234);
  assert_eq!(hex_of(&bignum, &mut buf), "FFF");

  bignum.assign_uint16(10);
  assign_hex(&mut other, "1234567890");
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 0);
  assert_eq!(hex_of(&bignum, &mut buf), "A");

  assign_hex(&mut bignum, "12345678");
  assign_hex(&mut other, "3789012");
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 5);
  assert_eq!(hex_of(&bignum, &mut buf), "D9861E");

  assign_hex(&mut bignum, "70000001");
  assign_hex(&mut other, "1FFFFFFF");
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 3);
  assert_eq!(hex_of(&bignum, &mut buf), "10000004");

  assign_hex(&mut bignum, "28000000");
  assign_hex(&mut other, "12A05F20");
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 2);
  assert_eq!(hex_of(&bignum, &mut buf), "2BF41C0");

  bignum.assign_uint16(10);
  bignum.shift_left(500);
  other.assign_bignum(&bignum);
  bignum.multiply_by_uint32(0x1234);
  third.assign_uint16(0xFFF);
  other.subtract_bignum(&third);
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 0x1234);
  assert_eq!(hex_of(&bignum, &mut buf), "1232DCC");
  assert_eq!(bignum.divide_modulo_int_bignum(&other), 0);
  assert_eq!(hex_of(&bignum, &mut buf), "1232DCC");
  OK
}

/// BignumTest.Compare
#[test]
fn bignum_test_compare() -> Void {
  let mut bignum1 = Bignum::new();
  let mut bignum2 = Bignum::new();
  bignum1.assign_uint16(1);
  bignum2.assign_uint16(1);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), 0);
  assert!(Bignum::equal(&bignum1, &bignum2));
  assert!(Bignum::less_equal(&bignum1, &bignum2));
  assert!(!Bignum::less(&bignum1, &bignum2));

  bignum1.assign_uint16(0);
  bignum2.assign_uint16(1);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), -1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 1);
  assert!(!Bignum::equal(&bignum1, &bignum2));
  assert!(!Bignum::equal(&bignum2, &bignum1));
  assert!(Bignum::less_equal(&bignum1, &bignum2));
  assert!(!Bignum::less_equal(&bignum2, &bignum1));
  assert!(Bignum::less(&bignum1, &bignum2));
  assert!(!Bignum::less(&bignum2, &bignum1));

  assign_hex(&mut bignum1, "1234567890ABCDEF12345");
  assign_hex(&mut bignum2, "1234567890ABCDEF12345");
  assert_eq!(Bignum::compare(&bignum1, &bignum2), 0);

  assign_hex(&mut bignum1, "1234567890ABCDEF12345");
  assign_hex(&mut bignum2, "1234567890ABCDEF12346");
  assert_eq!(Bignum::compare(&bignum1, &bignum2), -1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 1);

  assign_hex(&mut bignum1, "1234567890ABCDEF12345");
  bignum1.shift_left(500);
  assign_hex(&mut bignum2, "1234567890ABCDEF12345");
  bignum2.shift_left(500);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), 0);

  assign_hex(&mut bignum1, "1234567890ABCDEF12345");
  bignum1.shift_left(500);
  assign_hex(&mut bignum2, "1234567890ABCDEF12346");
  bignum2.shift_left(500);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), -1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 1);

  bignum1.assign_uint16(1);
  bignum1.shift_left(64);
  assign_hex(&mut bignum2, "10000000000000000");
  assert_eq!(Bignum::compare(&bignum1, &bignum2), 0);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 0);

  bignum1.assign_uint16(1);
  bignum1.shift_left(64);
  assign_hex(&mut bignum2, "10000000000000001");
  assert_eq!(Bignum::compare(&bignum1, &bignum2), -1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 1);

  bignum1.assign_uint16(1);
  bignum1.shift_left(96);
  assign_hex(&mut bignum2, "10000000000000001");
  bignum2.shift_left(32);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), -1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 1);

  assign_hex(&mut bignum1, "FFFFFFFFFFFFFFFF");
  bignum2.assign_uint16(1);
  bignum2.shift_left(64);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), -1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 1);

  assign_hex(&mut bignum1, "FFFFFFFFFFFFFFFF");
  bignum1.shift_left(32);
  bignum2.assign_uint16(1);
  bignum2.shift_left(96);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), -1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 1);

  assign_hex(&mut bignum1, "FFFFFFFFFFFFFFFF");
  bignum1.shift_left(32);
  bignum2.assign_uint16(1);
  bignum2.shift_left(95);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), 1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), -1);

  assign_hex(&mut bignum1, "FFFFFFFFFFFFFFFF");
  bignum1.shift_left(32);
  bignum2.assign_uint16(1);
  bignum2.shift_left(100);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), -1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 1);

  assign_hex(&mut bignum1, "100000000000000");
  bignum2.assign_uint16(1);
  bignum2.shift_left(14 * 4);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), 0);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 0);

  assign_hex(&mut bignum1, "100000000000001");
  bignum2.assign_uint16(1);
  bignum2.shift_left(14 * 4);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), 1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), -1);

  assign_hex(&mut bignum1, "200000000000000");
  bignum2.assign_uint16(3);
  bignum2.shift_left(14 * 4);
  assert_eq!(Bignum::compare(&bignum1, &bignum2), -1);
  assert_eq!(Bignum::compare(&bignum2, &bignum1), 1);
  OK
}

/// BignumTest.PlusCompare
#[test]
fn bignum_test_plus_compare() -> Void {
  let mut a = Bignum::new();
  let mut b = Bignum::new();
  let mut c = Bignum::new();
  a.assign_uint16(1);
  b.assign_uint16(0);
  c.assign_uint16(1);
  assert_eq!(Bignum::plus_compare(&a, &b, &c), 0);
  assert!(Bignum::plus_equal(&a, &b, &c));
  assert!(Bignum::plus_less_equal(&a, &b, &c));
  assert!(!Bignum::plus_less(&a, &b, &c));

  a.assign_uint16(0);
  b.assign_uint16(0);
  c.assign_uint16(1);
  assert_eq!(Bignum::plus_compare(&a, &b, &c), -1);
  assert_eq!(Bignum::plus_compare(&c, &b, &a), 1);
  assert!(!Bignum::plus_equal(&a, &b, &c));
  assert!(!Bignum::plus_equal(&c, &b, &a));
  assert!(Bignum::plus_less_equal(&a, &b, &c));
  assert!(!Bignum::plus_less_equal(&c, &b, &a));
  assert!(Bignum::plus_less(&a, &b, &c));
  assert!(!Bignum::plus_less(&c, &b, &a));

  assign_hex(&mut a, "1234567890ABCDEF12345");
  b.assign_uint16(1);
  assign_hex(&mut c, "1234567890ABCDEF12345");
  assert_eq!(Bignum::plus_compare(&a, &b, &c), 1);

  assign_hex(&mut a, "1234567890ABCDEF12344");
  b.assign_uint16(1);
  assign_hex(&mut c, "1234567890ABCDEF12345");
  assert_eq!(Bignum::plus_compare(&a, &b, &c), 0);

  for (sa, sb, sc, shift_a, shift_b, shift_c, want) in [
    (
      "1234567890",
      "ABCDEF12345",
      "1234567890ABCDEF12345",
      11 * 4,
      0,
      0,
      0,
    ),
    (
      "1234567890",
      "ABCDEF12344",
      "1234567890ABCDEF12345",
      11 * 4,
      0,
      0,
      -1,
    ),
    (
      "1234567890",
      "ABCDEF12346",
      "1234567890ABCDEF12345",
      11 * 4,
      0,
      0,
      1,
    ),
    (
      "1234567891",
      "ABCDEF12345",
      "1234567890ABCDEF12345",
      11 * 4,
      0,
      0,
      1,
    ),
    (
      "1234567889",
      "ABCDEF12345",
      "1234567890ABCDEF12345",
      11 * 4,
      0,
      0,
      -1,
    ),
    (
      "1234567890",
      "ABCDEF12345",
      "1234567890ABCDEF12345",
      11 * 4 + 32,
      32,
      32,
      0,
    ),
    (
      "1234567890",
      "ABCDEF12344",
      "1234567890ABCDEF12345",
      11 * 4 + 32,
      32,
      32,
      -1,
    ),
    (
      "1234567890",
      "ABCDEF12346",
      "1234567890ABCDEF12345",
      11 * 4 + 32,
      32,
      32,
      1,
    ),
    (
      "1234567891",
      "ABCDEF12345",
      "1234567890ABCDEF12345",
      11 * 4 + 32,
      32,
      32,
      1,
    ),
    (
      "1234567889",
      "ABCDEF12345",
      "1234567890ABCDEF12345",
      11 * 4 + 32,
      32,
      32,
      -1,
    ),
    (
      "1234567890",
      "ABCDEF12345",
      "1234567890ABCDEF1234500000000",
      11 * 4 + 32,
      32,
      0,
      0,
    ),
    (
      "1234567890",
      "ABCDEF12344",
      "1234567890ABCDEF1234500000000",
      11 * 4 + 32,
      32,
      0,
      -1,
    ),
    (
      "1234567890",
      "ABCDEF12346",
      "1234567890ABCDEF1234500000000",
      11 * 4 + 32,
      32,
      0,
      1,
    ),
    (
      "1234567891",
      "ABCDEF12345",
      "1234567890ABCDEF1234500000000",
      11 * 4 + 32,
      32,
      0,
      1,
    ),
    (
      "1234567889",
      "ABCDEF12345",
      "1234567890ABCDEF1234500000000",
      11 * 4 + 32,
      32,
      0,
      -1,
    ),
    (
      "1234567890",
      "ABCDEF12345",
      "123456789000000000ABCDEF12345",
      11 * 4 + 32,
      0,
      0,
      0,
    ),
    (
      "1234567890",
      "ABCDEF12346",
      "123456789000000000ABCDEF12345",
      11 * 4 + 32,
      0,
      0,
      1,
    ),
    (
      "1234567890",
      "ABCDEF12344",
      "123456789000000000ABCDEF12345",
      11 * 4 + 32,
      0,
      0,
      -1,
    ),
    (
      "1234567890",
      "ABCDEF12345",
      "12345678900000ABCDEF123450000",
      11 * 4 + 32,
      16,
      0,
      0,
    ),
    (
      "1234567890",
      "ABCDEF12344",
      "12345678900000ABCDEF123450000",
      11 * 4 + 32,
      16,
      0,
      -1,
    ),
    (
      "1234567890",
      "ABCDEF12345",
      "12345678900000ABCDEF123450001",
      11 * 4 + 32,
      16,
      0,
      -1,
    ),
    (
      "1234567890",
      "ABCDEF12346",
      "12345678900000ABCDEF123450000",
      11 * 4 + 32,
      16,
      0,
      1,
    ),
  ] {
    assign_hex(&mut a, sa);
    a.shift_left(shift_a);
    assign_hex(&mut b, sb);
    if shift_b > 0 {
      b.shift_left(shift_b);
    }
    assign_hex(&mut c, sc);
    if shift_c > 0 {
      c.shift_left(shift_c);
    }
    assert_eq!(Bignum::plus_compare(&a, &b, &c), want, "{sa}+{sb} vs {sc}");
  }
  OK
}

/// BignumTest.Square
#[test]
fn bignum_test_square() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();

  bignum.assign_uint16(1);
  bignum.square();
  assert_eq!(hex_of(&bignum, &mut buf), "1");

  bignum.assign_uint16(2);
  bignum.square();
  assert_eq!(hex_of(&bignum, &mut buf), "4");

  bignum.assign_uint16(10);
  bignum.square();
  assert_eq!(hex_of(&bignum, &mut buf), "64");

  assign_hex(&mut bignum, "FFFFFFF");
  bignum.square();
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFFE0000001");

  assign_hex(&mut bignum, "FFFFFFFFFFFFFF");
  bignum.square();
  assert_eq!(hex_of(&bignum, &mut buf), "FFFFFFFFFFFFFE00000000000001");
  OK
}

/// BignumTest.AssignPowerUInt16
#[test]
fn bignum_test_assign_power_uint16() -> Void {
  let mut buf = [0u8; 2048];
  let mut bignum = Bignum::new();

  for (base, exp, want) in [
    (1u16, 0, "1"),
    (1, 1, "1"),
    (1, 2, "1"),
    (2, 0, "1"),
    (2, 1, "2"),
    (2, 2, "4"),
    (16, 1, "10"),
    (16, 2, "100"),
    (16, 5, "100000"),
    (16, 8, "100000000"),
    (16, 16, "10000000000000000"),
    (16, 30, "1000000000000000000000000000000"),
    (10, 0, "1"),
    (10, 1, "A"),
    (10, 2, "64"),
    (10, 5, "186A0"),
    (10, 8, "5F5E100"),
    (10, 16, "2386F26FC10000"),
    (10, 30, "C9F2C9CD04674EDEA40000000"),
    (10, 31, "7E37BE2022C0914B2680000000"),
    (2, 0, "1"),
    (2, 100, "10000000000000000000000000"),
    (17, 0, "1"),
    (
      17,
      99,
      concat!(
        "1942BB9853FAD924A3D4DD92B89B940E0207BEF05DB9C26BC1B757",
        "80BE0C5A2C2990E02A681224F34ED68558CE4C6E33760931"
      ),
    ),
    (
      0xFFFF,
      99,
      concat!(
        "FF9D12F09B886C54E77E7439C7D2DED2D34F669654C0C2B6B8C288250",
        "5A2211D0E3DC9A61831349EAE674B11D56E3049D7BD79DAAD6C9FA2BA",
        "528E3A794299F2EE9146A324DAFE3E88967A0358233B543E233E575B9",
        "DD4E3AA7942146426C328FF55BFD5C45E0901B1629260AF9AE2F310C5",
        "50959FAF305C30116D537D80CF6EBDBC15C5694062AF1AC3D956D0A41",
        "B7E1B79FF11E21D83387A1CE1F5882B31E4B5D8DE415BDBE6854466DF",
        "343362267A7E8833119D31D02E18DB5B0E8F6A64B0ED0D0062FFFF"
      ),
    ),
  ] {
    bignum.assign_power_uint16(base, exp);
    assert_eq!(hex_of(&bignum, &mut buf), want, "{base}^{exp}");
  }
  OK
}
