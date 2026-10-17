//! 原版 v8/test/unittests/base/bignum-dtoa-unittest.cc 1:1 迁移
//!
//! BignumDtoaVariousDoubles 断言全量落地 (含 CHECK_GE 位数上界)。
//! BignumDtoaGayShortest/Fixed/Precision 跳过: gay-* 黄金数据属 L1 v8-numbers
//! 验证范围 (v8-base/readme/zh.md 依赖缺口登记), 本 crate 不引入;
//! 该覆盖由 v8-numbers/tests/dtoa_golden.rs 全量 10 万条驱动
//! DoubleToAscii (内部分发 fast_fixed_dtoa/bignum_dtoa 兜底) 实际闭环。

use std::str::from_utf8;

use aok::{OK, Void};
use v8_base::numbers::bignum_dtoa::{BignumDtoaMode, bignum_dtoa};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// TrimRepresentation (bignum-dtoa-unittest.cc:45-49)
fn trimmed(buf: &[u8], length: usize) -> &str {
  let mut end = length;
  while end > 0 && buf[end - 1] == b'0' {
    end -= 1;
  }
  from_utf8(&buf[..end]).unwrap_or("?")
}

/// (v bits, mode, requested_digits, 期望串, 期望 point, 是否断言 point, 是否 Trim 后比较, 是否断言位数上界)
/// 位数上界 = 原版 CHECK_GE: FIXED → requested >= length-point; PRECISION → requested >= length
type Case = (
  u64,
  BignumDtoaMode,
  i32,
  &'static str,
  i32,
  bool,
  bool,
  bool,
);

#[test]
fn bignum_dtoa_various_doubles() -> Void {
  let mut buf = [0u8; 100];

  let cases: [Case; 32] = [
    (
      f64::to_bits(1.0),
      BignumDtoaMode::Shortest,
      0,
      "1",
      1,
      true,
      false,
      false,
    ),
    (
      f64::to_bits(1.0),
      BignumDtoaMode::Fixed,
      3,
      "1",
      1,
      true,
      true,
      true,
    ),
    (
      f64::to_bits(1.0),
      BignumDtoaMode::Precision,
      3,
      "1",
      1,
      true,
      true,
      true,
    ),
    (
      f64::to_bits(1.5),
      BignumDtoaMode::Shortest,
      0,
      "15",
      1,
      true,
      false,
      false,
    ),
    (
      f64::to_bits(1.5),
      BignumDtoaMode::Fixed,
      10,
      "15",
      1,
      true,
      true,
      true,
    ),
    (
      f64::to_bits(1.5),
      BignumDtoaMode::Precision,
      10,
      "15",
      1,
      true,
      true,
      true,
    ),
    // min_double = 5e-324 (FIXED 的 GE 与空串断言在表外单独核)
    (
      1,
      BignumDtoaMode::Shortest,
      0,
      "5",
      -323,
      true,
      false,
      false,
    ),
    (
      1,
      BignumDtoaMode::Precision,
      5,
      "49407",
      -323,
      true,
      true,
      true,
    ),
    // max_double = 1.7976931348623157e308
    (
      0x7FEF_FFFF_FFFF_FFFF,
      BignumDtoaMode::Shortest,
      0,
      "17976931348623157",
      309,
      true,
      false,
      false,
    ),
    (
      0x7FEF_FFFF_FFFF_FFFF,
      BignumDtoaMode::Precision,
      7,
      "1797693",
      309,
      true,
      true,
      true,
    ),
    (
      f64::to_bits(4294967272.0),
      BignumDtoaMode::Shortest,
      0,
      "4294967272",
      10,
      true,
      false,
      false,
    ),
    // 原版此处无 CHECK_GE
    (
      f64::to_bits(4294967272.0),
      BignumDtoaMode::Fixed,
      5,
      "429496727200000",
      10,
      true,
      false,
      false,
    ),
    (
      f64::to_bits(4294967272.0),
      BignumDtoaMode::Precision,
      14,
      "4294967272",
      10,
      true,
      true,
      true,
    ),
    (
      0x7DF0_0000_0000_0000, /* 4.1855804968213567e298 */
      BignumDtoaMode::Shortest,
      0,
      "4185580496821357",
      299,
      true,
      false,
      false,
    ),
    (
      0x7DF0_0000_0000_0000, /* 4.1855804968213567e298 */
      BignumDtoaMode::Precision,
      20,
      "41855804968213567225",
      299,
      true,
      true,
      true,
    ),
    (
      0x0004_0000_0000_0000, /* 5.5626846462680035e-309 */
      BignumDtoaMode::Shortest,
      0,
      "5562684646268003",
      -308,
      true,
      false,
      false,
    ),
    (
      0x0004_0000_0000_0000, /* 5.5626846462680035e-309 */
      BignumDtoaMode::Precision,
      1,
      "6",
      -308,
      true,
      true,
      true,
    ),
    (
      f64::to_bits(2147483648.0),
      BignumDtoaMode::Shortest,
      0,
      "2147483648",
      10,
      true,
      false,
      false,
    ),
    (
      f64::to_bits(2147483648.0),
      BignumDtoaMode::Fixed,
      2,
      "2147483648",
      10,
      true,
      true,
      true,
    ),
    (
      f64::to_bits(2147483648.0),
      BignumDtoaMode::Precision,
      5,
      "21475",
      10,
      true,
      true,
      true,
    ),
    (
      0x7DEB_6776_9DB9_17D9, /* 3.5844466002796428e298 */
      BignumDtoaMode::Shortest,
      0,
      "35844466002796428",
      299,
      true,
      false,
      false,
    ),
    (
      0x7DEB_6776_9DB9_17D9, /* 3.5844466002796428e298 */
      BignumDtoaMode::Precision,
      10,
      "35844466",
      299,
      true,
      true,
      true,
    ),
    // smallest normal
    (
      0x0010_0000_0000_0000,
      BignumDtoaMode::Shortest,
      0,
      "22250738585072014",
      -307,
      true,
      false,
      false,
    ),
    (
      0x0010_0000_0000_0000,
      BignumDtoaMode::Precision,
      20,
      "22250738585072013831",
      -307,
      true,
      true,
      true,
    ),
    // largest denormal
    (
      0x000F_FFFF_FFFF_FFFF,
      BignumDtoaMode::Shortest,
      0,
      "2225073858507201",
      -307,
      true,
      false,
      false,
    ),
    (
      0x000F_FFFF_FFFF_FFFF,
      BignumDtoaMode::Precision,
      20,
      "2225073858507200889",
      -307,
      true,
      true,
      true,
    ),
    (
      f64::to_bits(4128420500802942e-24),
      BignumDtoaMode::Shortest,
      0,
      "4128420500802942",
      -8,
      true,
      false,
      false,
    ),
    // 原版未断言 point
    (
      0x3DFB_0053_14E2_421F, /* 3.9292015898194142585311918e-10 */
      BignumDtoaMode::Shortest,
      0,
      "39292015898194143",
      0,
      false,
      false,
      false,
    ),
    // 原版未断言 point
    (
      f64::to_bits(4194304.0),
      BignumDtoaMode::Fixed,
      5,
      "4194304",
      0,
      false,
      true,
      true,
    ),
    (
      0x0ED5_9801_D6E5_F2D6, /* 3.3161339052167390562200598e-237 */
      BignumDtoaMode::Precision,
      19,
      "3316133905216739056",
      -236,
      true,
      true,
      true,
    ),
    (
      0x67C6_6973_51FF_4AEC, /* 7.9885183916008099497815232e191 */
      BignumDtoaMode::Precision,
      4,
      "7989",
      192,
      true,
      true,
      true,
    ),
    (
      0x4376_3457_85D8_A008, /* 1.0000000000000012800000000e17 */
      BignumDtoaMode::Fixed,
      1,
      "100000000000000128",
      18,
      true,
      true,
      true,
    ),
  ];

  for (bits, mode, requested, want, want_point, assert_point, use_trim, check_bound) in cases {
    let v = f64::from_bits(bits);
    let r = bignum_dtoa(v, mode, requested, &mut buf);
    let got = if use_trim {
      trimmed(&buf, r.length)
    } else {
      from_utf8(&buf[..r.length]).unwrap_or("?")
    };
    assert_eq!(got, want, "v={v:e} mode={mode:?}");
    if assert_point {
      assert_eq!(r.point, want_point, "point: v={v:e} mode={mode:?}");
    }
    if check_bound {
      // 原版 CHECK_GE: FIXED 约束小数位数, PRECISION 约束总位数
      match mode {
        BignumDtoaMode::Fixed => {
          assert!(
            requested >= r.length as i32 - r.point,
            "GE({requested}, length-point): v={v:e}"
          );
        }
        BignumDtoaMode::Precision => {
          assert!(
            requested >= r.length as i32,
            "GE({requested}, length): v={v:e}"
          );
        }
        BignumDtoaMode::Shortest => {}
      }
    }
  }

  // 表外单独规约 (原版逐条):
  // min_double FIXED 5: 串为空且 5 >= length - point
  let r = bignum_dtoa(5e-324, BignumDtoaMode::Fixed, 5, &mut buf);
  assert!(5 >= r.length as i32 - r.point);
  assert_eq!(trimmed(&buf, r.length), "");
  OK
}
