//! 原版 v8/test/unittests/base/fast-dtoa-unittest.cc 1:1 迁移
//!
//! FastDtoaShortestVariousDoubles / FastDtoaPrecisionVariousDoubles 落地,
//! 条件断言 (if status) 语义保留。
//! FastDtoaGayShortest / FastDtoaGayPrecision 跳过: gay-* 黄金数据属 L1
//! v8-numbers (v8-base/readme/zh.md 依赖缺口登记); 其 0.99/0.85 成功率规约
//! 由 fast_dtoa_sweep.rs 全指数域回读扫描佐证。

use std::str::from_utf8;

use aok::{OK, Void};
use v8_base::numbers::fast_dtoa::{FAST_DTOA_MAXIMAL_LENGTH, FastDtoaMode, fast_dtoa};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// FastDtoaTest.FastDtoaShortestVariousDoubles
#[test]
fn fast_dtoa_shortest_various_doubles() -> Void {
  let mut buf = [0u8; 100];

  // 必成功用例: (v, 期望串, 期望 point)
  for (v, want, want_point) in [
    (5e-324, "5", -323),
    (1.7976931348623157e308, "17976931348623157", 309),
    (4294967272.0, "4294967272", 10),
    (
      f64::from_bits(0x7DF0_0000_0000_0000), /* 4.1855804968213567e298 */
      "4185580496821357",
      299,
    ),
    (
      f64::from_bits(0x0004_0000_0000_0000), /* 5.5626846462680035e-309 */
      "5562684646268003",
      -308,
    ),
    (2147483648.0, "2147483648", 10),
  ] {
    let r = fast_dtoa(v, FastDtoaMode::Shortest, 0, &mut buf)
      .unwrap_or_else(|| panic!("FastDtoa SHORTEST({v:e}) 失败"));
    assert_eq!(from_utf8(&buf[..r.length]).unwrap_or("?"), want, "v={v:e}");
    assert_eq!(r.decimal_point, want_point, "point: v={v:e}");
  }

  // 条件成功用例 (原版 if (status) 才断言): 失败则跳过, 成功须精确
  for (bits, want, want_point) in [
    (
      f64::to_bits(
        f64::from_bits(0x7DEB_6776_9DB9_17D9), /* 3.5844466002796428e298 */
      ),
      "35844466002796428",
      299,
    ),
    (0x0010_0000_0000_0000, "22250738585072014", -307),
    (0x000F_FFFF_FFFF_FFFF, "2225073858507201", -307),
  ] {
    if let Some(r) = fast_dtoa(f64::from_bits(bits), FastDtoaMode::Shortest, 0, &mut buf) {
      assert_eq!(
        from_utf8(&buf[..r.length]).unwrap_or("?"),
        want,
        "bits={bits:#x}"
      );
      assert_eq!(r.decimal_point, want_point, "point: bits={bits:#x}");
    }
  }
  OK
}

/// FastDtoaTest.FastDtoaPrecisionVariousDoubles
///
/// 表列 (v, requested, 期望串, 期望 point, 必成功, Trim 后比较, CHECK_GE(n, length) 的 n):
/// 原版逐条对照 fast-dtoa-unittest.cc:128-218 — CHECK(status) → 必成功;
/// if (status) → 条件成功; 1.5/10 与 4294967272/14 仅此两条为条件。
#[test]
fn fast_dtoa_precision_various_doubles() -> Void {
  let mut buf = [0u8; 100];

  type P = (f64, i32, &'static str, i32, bool, bool, i32);
  let cases: [P; 13] = [
    (1.0, 3, "1", 1, true, true, 3),
    (1.5, 10, "15", 1, false, true, 10),
    // min_double = 5e-324
    (5e-324, 5, "49407", -323, true, false, 0),
    // max_double
    (1.7976931348623157e308, 7, "1797693", 309, true, false, 0),
    (4294967272.0, 14, "4294967272", 10, false, true, 14),
    (
      f64::from_bits(0x7DF0_0000_0000_0000), /* 4.1855804968213567e298 */
      17,
      "41855804968213567",
      299,
      true,
      false,
      0,
    ),
    (
      f64::from_bits(0x0004_0000_0000_0000), /* 5.5626846462680035e-309 */
      1,
      "6",
      -308,
      true,
      false,
      0,
    ),
    (2147483648.0, 5, "21475", 10, true, false, 0),
    (
      f64::from_bits(0x7DEB_6776_9DB9_17D9), /* 3.5844466002796428e298 */
      10,
      "35844466",
      299,
      true,
      true,
      10,
    ),
    // smallest normal: 17 位全量
    (
      f64::from_bits(0x0010_0000_0000_0000),
      17,
      "22250738585072014",
      -307,
      true,
      false,
      0,
    ),
    // largest denormal: GE(20, length) + Trim
    (
      f64::from_bits(0x000F_FFFF_FFFF_FFFF),
      17,
      "22250738585072009",
      -307,
      true,
      true,
      20,
    ),
    (
      f64::from_bits(0x0ED5_9801_D6E5_F2D6), /* 3.3161339052167390562200598e-237 */
      18,
      "331613390521673906",
      -236,
      true,
      false,
      0,
    ),
    (
      f64::from_bits(0x67C6_6973_51FF_4AEC), /* 7.9885183916008099497815232e191 */
      4,
      "7989",
      192,
      true,
      false,
      0,
    ),
  ];

  for (v, requested, want, want_point, required, use_trim, ge) in cases {
    let Some(r) = fast_dtoa(v, FastDtoaMode::Precision, requested, &mut buf) else {
      assert!(!required, "FastDtoa PRECISION({v:e},{requested}) 须成功");
      continue;
    };
    if ge > 0 {
      assert!(ge >= r.length as i32, "length: v={v:e}");
    }
    let end = if use_trim {
      let mut end = r.length;
      while end > 0 && buf[end - 1] == b'0' {
        end -= 1;
      }
      end
    } else {
      r.length
    };
    assert_eq!(from_utf8(&buf[..end]).unwrap_or("?"), want, "v={v:e}");
    assert_eq!(r.decimal_point, want_point, "point: v={v:e}");
  }

  // SHORTEST 长度上限规约 (原版 Gay 循环内 CHECK_GE(kFastDtoaMaximalLength, length))
  assert_eq!(FAST_DTOA_MAXIMAL_LENGTH, 17);
  OK
}
