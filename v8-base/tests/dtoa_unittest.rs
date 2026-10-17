//! 原版 v8/test/unittests/base/dtoa-unittest.cc 1:1 迁移
//!
//! DtoaVariousDoubles 断言全量落地 (含 sign 位、CHECK_EQ(1,length) 与 CHECK_GE 位数上界)。
//! DtoaGayShortest/Fixed/Precision 跳过: gay-* 黄金数据属 L1 v8-numbers
//! (v8-base/readme/zh.md 依赖缺口登记), 本 crate 不引入;
//! 该覆盖由 v8-numbers/tests/dtoa_golden.rs 全量 10 万条直接驱动本 crate
//! double_to_ascii 实际闭环。

use std::str::from_utf8;

use aok::{OK, Void};
use v8_base::numbers::dtoa::{DtoaMode, double_to_ascii};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn trimmed(buf: &[u8], length: usize) -> &str {
  let mut end = length;
  while end > 0 && buf[end - 1] == b'0' {
    end -= 1;
  }
  from_utf8(&buf[..end]).unwrap_or("?")
}

/// (v bits, mode, requested, 期望串, 期望 sign, 期望 point, 断言 point, Trim 后比较,
///   断言位数上界, 断言 length==1)
/// 位数上界 = 原版 CHECK_GE: FIXED → requested >= length-point; PRECISION → requested >= length
type Case = (
  u64,
  DtoaMode,
  i32,
  &'static str,
  i32,
  i32,
  bool,
  bool,
  bool,
  bool,
);

/// DtoaTest.DtoaVariousDoubles
#[test]
fn dtoa_various_doubles() -> Void {
  let mut buf = [0u8; 100];

  let cases: [Case; 33] = [
    // 0.0 三模式 (FIXED/PRECISION 原版另有 CHECK_EQ(1, length))
    (
      f64::to_bits(0.0),
      DtoaMode::Shortest,
      0,
      "0",
      0,
      1,
      true,
      false,
      false,
      false,
    ),
    (
      f64::to_bits(0.0),
      DtoaMode::Fixed,
      2,
      "0",
      0,
      1,
      true,
      false,
      false,
      true,
    ),
    (
      f64::to_bits(0.0),
      DtoaMode::Precision,
      3,
      "0",
      0,
      1,
      true,
      false,
      false,
      true,
    ),
    (
      f64::to_bits(1.0),
      DtoaMode::Shortest,
      0,
      "1",
      0,
      1,
      true,
      false,
      false,
      false,
    ),
    (
      f64::to_bits(1.0),
      DtoaMode::Fixed,
      3,
      "1",
      0,
      1,
      true,
      true,
      true,
      false,
    ),
    (
      f64::to_bits(1.0),
      DtoaMode::Precision,
      3,
      "1",
      0,
      1,
      true,
      true,
      true,
      false,
    ),
    (
      f64::to_bits(1.5),
      DtoaMode::Shortest,
      0,
      "15",
      0,
      1,
      true,
      false,
      false,
      false,
    ),
    (
      f64::to_bits(1.5),
      DtoaMode::Fixed,
      10,
      "15",
      0,
      1,
      true,
      true,
      true,
      false,
    ),
    (
      f64::to_bits(1.5),
      DtoaMode::Precision,
      10,
      "15",
      0,
      1,
      true,
      true,
      true,
      false,
    ),
    // min_double = 5e-324 (FIXED 的 GE(5,·) 与 -5 >= point 表外核)
    (
      1,
      DtoaMode::Shortest,
      0,
      "5",
      0,
      -323,
      true,
      false,
      false,
      false,
    ),
    (
      1,
      DtoaMode::Precision,
      5,
      "49407",
      0,
      -323,
      true,
      true,
      true,
      false,
    ),
    // max_double
    (
      0x7FEF_FFFF_FFFF_FFFF,
      DtoaMode::Shortest,
      0,
      "17976931348623157",
      0,
      309,
      true,
      false,
      false,
      false,
    ),
    (
      0x7FEF_FFFF_FFFF_FFFF,
      DtoaMode::Precision,
      7,
      "1797693",
      0,
      309,
      true,
      true,
      true,
      false,
    ),
    (
      f64::to_bits(4294967272.0),
      DtoaMode::Shortest,
      0,
      "4294967272",
      0,
      10,
      true,
      false,
      false,
      false,
    ),
    (
      f64::to_bits(4294967272.0),
      DtoaMode::Fixed,
      5,
      "4294967272",
      0,
      10,
      true,
      true,
      true,
      false,
    ),
    (
      f64::to_bits(4294967272.0),
      DtoaMode::Precision,
      14,
      "4294967272",
      0,
      10,
      true,
      true,
      true,
      false,
    ),
    (
      0x7DF0_0000_0000_0000, /* 4.1855804968213567e298 */
      DtoaMode::Shortest,
      0,
      "4185580496821357",
      0,
      299,
      true,
      false,
      false,
      false,
    ),
    (
      0x7DF0_0000_0000_0000, /* 4.1855804968213567e298 */
      DtoaMode::Precision,
      20,
      "41855804968213567225",
      0,
      299,
      true,
      true,
      true,
      false,
    ),
    (
      0x0004_0000_0000_0000, /* 5.5626846462680035e-309 */
      DtoaMode::Shortest,
      0,
      "5562684646268003",
      0,
      -308,
      true,
      false,
      false,
      false,
    ),
    (
      0x0004_0000_0000_0000, /* 5.5626846462680035e-309 */
      DtoaMode::Precision,
      1,
      "6",
      0,
      -308,
      true,
      true,
      true,
      false,
    ),
    // 负数: sign = 1
    (
      f64::to_bits(-2147483648.0),
      DtoaMode::Shortest,
      0,
      "2147483648",
      1,
      10,
      true,
      false,
      false,
      false,
    ),
    (
      f64::to_bits(-2147483648.0),
      DtoaMode::Fixed,
      2,
      "2147483648",
      1,
      10,
      true,
      true,
      true,
      false,
    ),
    (
      f64::to_bits(-2147483648.0),
      DtoaMode::Precision,
      5,
      "21475",
      1,
      10,
      true,
      true,
      true,
      false,
    ),
    (
      f64::to_bits(
        -f64::from_bits(0x7DEB_6776_9DB9_17D9), /* 3.5844466002796428e298 */
      ),
      DtoaMode::Shortest,
      0,
      "35844466002796428",
      1,
      299,
      true,
      false,
      false,
      false,
    ),
    (
      f64::to_bits(
        -f64::from_bits(0x7DEB_6776_9DB9_17D9), /* 3.5844466002796428e298 */
      ),
      DtoaMode::Precision,
      10,
      "35844466",
      1,
      299,
      true,
      true,
      true,
      false,
    ),
    // smallest normal
    (
      0x0010_0000_0000_0000,
      DtoaMode::Shortest,
      0,
      "22250738585072014",
      0,
      -307,
      true,
      false,
      false,
      false,
    ),
    (
      0x0010_0000_0000_0000,
      DtoaMode::Precision,
      20,
      "22250738585072013831",
      0,
      -307,
      true,
      true,
      true,
      false,
    ),
    // largest denormal
    (
      0x000F_FFFF_FFFF_FFFF,
      DtoaMode::Shortest,
      0,
      "2225073858507201",
      0,
      -307,
      true,
      false,
      false,
      false,
    ),
    (
      0x000F_FFFF_FFFF_FFFF,
      DtoaMode::Precision,
      20,
      "2225073858507200889",
      0,
      -307,
      true,
      true,
      true,
      false,
    ),
    (
      f64::to_bits(4128420500802942e-24),
      DtoaMode::Shortest,
      0,
      "4128420500802942",
      0,
      -8,
      true,
      false,
      false,
      false,
    ),
    // 原版未断言 point
    (
      f64::to_bits(-f64::from_bits(0x3DFB_0053_14E2_421F)), /* -3.9292015898194142585311918e-10 */
      DtoaMode::Shortest,
      0,
      "39292015898194143",
      1,
      0,
      false,
      false,
      false,
      false,
    ),
    (
      f64::to_bits(4194304.0),
      DtoaMode::Fixed,
      5,
      "4194304",
      0,
      0,
      false,
      true,
      true,
      false,
    ),
    (
      0x0ED5_9801_D6E5_F2D6, /* 3.3161339052167390562200598e-237 */
      DtoaMode::Precision,
      19,
      "3316133905216739056",
      0,
      -236,
      true,
      true,
      true,
      false,
    ),
  ];

  for (
    bits,
    mode,
    requested,
    want,
    want_sign,
    want_point,
    assert_point,
    use_trim,
    check_bound,
    len1,
  ) in cases
  {
    let v = f64::from_bits(bits);
    let r = double_to_ascii(v, mode, requested, &mut buf);
    let got = if use_trim {
      trimmed(&buf, r.length)
    } else {
      from_utf8(&buf[..r.length]).unwrap_or("?")
    };
    assert_eq!(got, want, "v={v:e} mode={mode:?}");
    assert_eq!(r.sign, want_sign, "sign: v={v:e} mode={mode:?}");
    if assert_point {
      assert_eq!(r.point, want_point, "point: v={v:e} mode={mode:?}");
    }
    if len1 {
      // 原版 0.0 FIXED/PRECISION: CHECK_EQ(1, length)
      assert_eq!(r.length, 1, "length: v={v:e} mode={mode:?}");
    }
    if check_bound {
      // 原版 CHECK_GE: FIXED 约束小数位数, PRECISION 约束总位数
      match mode {
        DtoaMode::Fixed => {
          assert!(
            requested >= r.length as i32 - r.point,
            "GE({requested}, length-point): v={v:e}"
          );
        }
        DtoaMode::Precision => {
          assert!(
            requested >= r.length as i32,
            "GE({requested}, length): v={v:e}"
          );
        }
        DtoaMode::Shortest => {}
      }
    }
  }

  // 表外规约: min_double FIXED 5 — GE(5, length-point), 串空, -5 >= point
  let r = double_to_ascii(5e-324, DtoaMode::Fixed, 5, &mut buf);
  assert!(5 >= r.length as i32 - r.point, "GE(5, length-point)");
  assert!(r.point <= -5);
  assert_eq!(trimmed(&buf, r.length), "");
  OK
}
