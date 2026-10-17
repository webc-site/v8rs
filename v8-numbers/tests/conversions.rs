//! unittests/numbers/conversions-unittest.cc 1:1 迁移 (Isolate 绑定用例除外, 见 README)
//! 测试名保留 gtest 原名; 另含 strtod-unittest.cc 主干与 dtoa-unittest.cc 内核抽样

use aok::{OK, Void};
use v8_base::numbers;
use v8_numbers::conversions::{
  ConversionFlag, binary_string_to_double, double_to_fixed_string_view, double_to_int32,
  double_to_radix_string_view, double_to_string_view, double_to_webidl_int64, hex_string_to_double,
  implicit_octal_string_to_double, int_to_string_view, is_special_index, octal_string_to_double,
  string_to_double, string_to_int, try_double_to_size,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const NO_FLAG: ConversionFlag = ConversionFlag::NoConversionFlag;
const PREFIX: ConversionFlag = ConversionFlag::AllowNonDecimalPrefix;
const JUNK: ConversionFlag = ConversionFlag::AllowTrailingJunk;

fn d2s(s: &str) -> f64 {
  string_to_double(s.as_bytes(), NO_FLAG, 0.0)
}

fn d2s_flag(s: &str, flag: ConversionFlag) -> f64 {
  string_to_double(s.as_bytes(), flag, 0.0)
}

// ConversionsTest.Hex
#[test]
fn hex() -> Void {
  assert_eq!(0.0, d2s_flag("0x0", PREFIX));
  assert_eq!(0.0, d2s_flag("0X0", PREFIX));
  assert_eq!(1.0, d2s_flag("0x1", PREFIX));
  assert_eq!(16.0, d2s_flag("0x10", PREFIX));
  assert_eq!(255.0, d2s_flag("0xFF", PREFIX));
  assert_eq!(175.0, d2s_flag("0xAF", PREFIX));

  assert_eq!(0.0, hex_string_to_double(b"0x0"));
  assert_eq!(0.0, hex_string_to_double(b"0X0"));
  assert_eq!(1.0, hex_string_to_double(b"0x1"));
  assert_eq!(16.0, hex_string_to_double(b"0x10"));
  assert_eq!(255.0, hex_string_to_double(b"0xFF"));
  assert_eq!(175.0, hex_string_to_double(b"0xAF"));
  OK
}

// ConversionsTest.Octal
#[test]
fn octal() -> Void {
  assert_eq!(0.0, d2s_flag("0o0", PREFIX));
  assert_eq!(0.0, d2s_flag("0O0", PREFIX));
  assert_eq!(1.0, d2s_flag("0o1", PREFIX));
  assert_eq!(7.0, d2s_flag("0o7", PREFIX));
  assert_eq!(8.0, d2s_flag("0o10", PREFIX));
  assert_eq!(63.0, d2s_flag("0o77", PREFIX));

  assert_eq!(0.0, octal_string_to_double(b"0o0"));
  assert_eq!(0.0, octal_string_to_double(b"0O0"));
  assert_eq!(1.0, octal_string_to_double(b"0o1"));
  assert_eq!(7.0, octal_string_to_double(b"0o7"));
  assert_eq!(8.0, octal_string_to_double(b"0o10"));
  assert_eq!(63.0, octal_string_to_double(b"0o77"));

  let x = 0o10_000_000_000u64 as f64; // 2 的幂, 无舍入误差
  assert_eq!(
    x * x * x * x * x,
    octal_string_to_double(format!("0o01{}", "0".repeat(50)).as_bytes())
  );
  OK
}

// ConversionsTest.ImplicitOctal
#[test]
fn implicit_octal() -> Void {
  assert_eq!(0.0, implicit_octal_string_to_double(b"0"));
  assert_eq!(0.0, implicit_octal_string_to_double(b"00"));
  assert_eq!(1.0, implicit_octal_string_to_double(b"01"));
  assert_eq!(7.0, implicit_octal_string_to_double(b"07"));
  assert_eq!(8.0, implicit_octal_string_to_double(b"010"));
  assert_eq!(63.0, implicit_octal_string_to_double(b"077"));

  assert_eq!(0.0, d2s_flag("0", PREFIX));
  assert_eq!(0.0, d2s_flag("00", PREFIX));
  assert_eq!(1.0, d2s_flag("01", PREFIX));
  assert_eq!(7.0, d2s_flag("07", PREFIX));
  assert_eq!(10.0, d2s_flag("010", PREFIX)); // 前缀模式: 010 = 10
  assert_eq!(77.0, d2s_flag("077", PREFIX));

  let x = 0o10_000_000_000u64 as f64;
  assert_eq!(
    x * x * x * x * x,
    implicit_octal_string_to_double(format!("01{}", "0".repeat(50)).as_bytes())
  );
  OK
}

// ConversionsTest.Binary
#[test]
fn binary() -> Void {
  assert_eq!(0.0, d2s_flag("0b0", PREFIX));
  assert_eq!(0.0, d2s_flag("0B0", PREFIX));
  assert_eq!(1.0, d2s_flag("0b1", PREFIX));
  assert_eq!(2.0, d2s_flag("0b10", PREFIX));
  assert_eq!(3.0, d2s_flag("0b11", PREFIX));

  assert_eq!(0.0, binary_string_to_double(b"0b0"));
  assert_eq!(0.0, binary_string_to_double(b"0B0"));
  assert_eq!(1.0, binary_string_to_double(b"0b1"));
  assert_eq!(2.0, binary_string_to_double(b"0b10"));
  assert_eq!(3.0, binary_string_to_double(b"0b11"));
  OK
}

// ConversionsTest.MalformedOctal
#[test]
fn malformed_octal() -> Void {
  assert_eq!(8.0, d2s_flag("08", PREFIX));
  assert_eq!(81.0, d2s_flag("081", PREFIX));
  assert_eq!(78.0, d2s_flag("078", PREFIX));

  assert_eq!(7.7, d2s_flag("07.7", PREFIX));
  assert_eq!(7.8, d2s_flag("07.8", PREFIX));
  assert_eq!(7e8, d2s_flag("07e8", PREFIX));
  assert_eq!(7e7, d2s_flag("07e7", PREFIX));

  assert_eq!(8.7, d2s_flag("08.7", PREFIX));
  assert_eq!(8e7, d2s_flag("08e7", PREFIX));

  assert_eq!(0.001, d2s_flag("0.001", PREFIX));
  assert_eq!(0.713, d2s_flag("0.713", PREFIX));
  OK
}

// ConversionsTest.TrailingJunk
#[test]
fn trailing_junk() -> Void {
  assert_eq!(8.0, d2s_flag("8q", JUNK));
  assert_eq!(10.0, d2s_flag("10e", JUNK));
  assert_eq!(10.0, d2s_flag("10e-", JUNK));
  OK
}

// ConversionsTest.NonStrDecimalLiteral
#[test]
fn non_str_decimal_literal() -> Void {
  assert!(string_to_double(b" ", NO_FLAG, f64::NAN).is_nan());
  assert!(string_to_double(b"", NO_FLAG, f64::NAN).is_nan());
  assert!(string_to_double(b" ", NO_FLAG, f64::NAN).is_nan());
  assert_eq!(0.0, string_to_double(b"", NO_FLAG, 0.0));
  assert_eq!(0.0, string_to_double(b" ", NO_FLAG, 0.0));
  OK
}

// ConversionsTest.IntegerStrLiteral
#[test]
fn integer_str_literal() -> Void {
  assert_eq!(0.0, d2s("0.0"));
  assert_eq!(0.0, d2s("0"));
  assert_eq!(0.0, d2s("00"));
  assert_eq!(0.0, d2s("000"));
  assert_eq!(1.0, d2s("1"));
  assert_eq!(-1.0, d2s("-1"));
  assert_eq!(-1.0, d2s("  -1  "));
  assert_eq!(1.0, d2s("  +1  "));
  assert!(d2s("  -  1  ").is_nan());
  assert!(d2s("  +  1  ").is_nan());

  assert_eq!(0.0, d2s_flag("0e0", PREFIX));
  assert_eq!(0.0, d2s_flag("0e1", PREFIX));
  assert_eq!(0.0, d2s_flag("0e-1", PREFIX));
  assert_eq!(0.0, d2s_flag("0e-100000", PREFIX));
  assert_eq!(0.0, d2s_flag("0e+100000", PREFIX));
  assert_eq!(0.0, d2s_flag("0.", PREFIX));
  OK
}

// ConversionsTest.LongNumberStr
#[test]
fn long_number_str() -> Void {
  assert_eq!(1e10, d2s("10000000000"));
  assert_eq!(1e20, d2s("100000000000000000000"));
  assert_eq!(
    1e60,
    d2s("1000000000000000000000000000000000000000000000000000000000000")
  );
  assert_eq!(1e-2, d2s(".01"));
  assert_eq!(1e-11, d2s(&format!(".{}1", "0".repeat(10))));
  assert_eq!(1e-21, d2s(&format!(".{}1", "0".repeat(20))));
  assert_eq!(1e-61, d2s(&format!(".{}1", "0".repeat(60))));

  // x=24414062505131248.0 与 y=24414062505131252.0 均可表示; 中点 z=(x+y)/2 舍到 x
  assert_eq!(24414062505131248.0, d2s("24414062505131250.0"));
  // z + delta 舍到 y
  assert_eq!(24414062505131252.0, d2s("24414062505131250.000000001"));
  OK
}

// ConversionsTest.MaximumSignificantDigits
#[test]
fn maximum_significant_digits() -> Void {
  let mut num = String::from(
    "4.4501477170144020250819966727949918635852426585926051135169509\
     122872622312493126406953054127118942431783801370080830523154578\
     251545303238277269592368457430440993619708911874715081505094180\
     604803751173783204118519353387964161152051487413083163272520124\
     606023105869053620631175265621765214646643181420505164043632222\
     668006474326056011713528291579642227455489682133472873831754840\
     341397809846934151055619529382191981473003234105366170879223151\
     087335413188049110555339027884856781219017754500629806224571029\
     581637117459456877330110324211689177656713705497387108207822477\
     584250967061891687062782163335299376138075114200886249979505279\
     101870966346394401564490729731565935244123171539810221213221201\
     847003580761626016356864581135848683152156368691976240370422601\
     6998291015625000000000000000000000000000000000e-308",
  );
  assert_eq!(4.450147717014402e-308, d2s(&num));

  // 改变末位后 strtod 结果翻转 (glibc 实现下亦翻转)
  let bytes = unsafe { num.as_bytes_mut() };
  let last = bytes.len() - 8;
  bytes[last] = b'1';
  assert_eq!(4.4501477170144023e-308, d2s(&num));
  OK
}

// ConversionsTest.MinimumExponent
#[test]
fn minimum_exponent() -> Void {
  let mut num = String::from(
    "445014771701440202508199667279499186358524265859260511351695091\
     228726223124931264069530541271189424317838013700808305231545782\
     515453032382772695923684574304409936197089118747150815050941806\
     048037511737832041185193533879641611520514874130831632725201246\
     060231058690536206311752656217652146466431814205051640436322226\
     680064743260560117135282915796422274554896821334728738317548403\
     413978098469341510556195293821919814730032341053661708792231510\
     873354131880491105553390278848567812190177545006298062245710295\
     816371174594568773301103242116891776567137054973871082078224775\
     842509670618916870627821633352993761380751142008862499795052791\
     018709663463944015644907297315659352441231715398102212132212018\
     470035807616260163568645811358486831521563686919762403704226016\
     998291015625000000000000000000000000000000000e-1108",
  );
  assert_eq!(4.450147717014402e-308, d2s(&num));

  let bytes = unsafe { num.as_bytes_mut() };
  let last = bytes.len() - 8;
  bytes[last] = b'1';
  assert_eq!(4.4501477170144023e-308, d2s(&num));
  OK
}

// ConversionsTest.MaximumExponent
#[test]
fn maximum_exponent() -> Void {
  assert_eq!(1.6e308, d2s("0.16e309"));
  OK
}

// ConversionsTest.ExponentNumberStr
#[test]
fn exponent_number_str() -> Void {
  assert_eq!(1e1, d2s("1e1"));
  assert_eq!(1e1, d2s("1e+1"));
  assert_eq!(1e-1, d2s("1e-1"));
  assert_eq!(1e100, d2s("1e+100"));
  assert_eq!(1e-100, d2s("1e-100"));
  assert_eq!(1e-106, d2s(".000001e-100"));
  OK
}

// ConversionsTest.IntToStringView
#[test]
fn int_to_string_view_test() -> Void {
  let pairs: [(i32, &str); 9] = [
    (0, "0"),
    (101, "101"),
    (-1, "-1"),
    (1024, "1024"),
    (200000, "200000"),
    (-1024, "-1024"),
    (-200000, "-200000"),
    (i32::MIN, "-2147483648"),
    (i32::MAX, "2147483647"),
  ];
  let mut buf = [0u8; 4096];
  for (n, s) in pairs {
    assert_eq!(s, int_to_string_view(n, &mut buf));
  }
  OK
}

// ConversionsTest.DoubleToStringView
#[test]
fn double_to_string_view_test() -> Void {
  let pairs: [(f64, &str); 6] = [
    (0.0, "0"),
    (i32::MIN as f64, "-2147483648"),
    (i32::MAX as f64, "2147483647"),
    // ES 7.1.12.1: -0 字符串化为 "0"
    (-0.0, "0"),
    (1.1, "1.1"),
    (0.1, "0.1"),
  ];
  let mut buf = [0u8; 4096];
  for (v, s) in pairs {
    assert_eq!(s, double_to_string_view(v, &mut buf));
  }
  OK
}

// ConversionsTest.DoubleToInt32
#[test]
fn double_to_int32_test() -> Void {
  const MAX_SAFE: f64 = 9007199254740991.0; // 2^53 - 1
  const MIN_SAFE: f64 = -MAX_SAFE;
  let pairs: [(f64, i32); 16] = [
    (0.0, 0),
    (-0.0, 0),
    (f64::NAN, 0),
    (f64::INFINITY, 0),
    (f64::NEG_INFINITY, 0),
    (3.5, 3),
    (1.99, 1),
    (-1.99, -1),
    (i32::MIN as f64, i32::MIN),
    (i32::MAX as f64, i32::MAX),
    (MAX_SAFE, -1), // 2^53-1 mod 2^32
    (MIN_SAFE, 1),
    (MAX_SAFE + 1.0, 0), // 2^53 mod 2^32
    (MIN_SAFE - 1.0, 0),
    (4294967295.0, -1),
    (4025962439.0, -269004857),
  ];
  for (v, i) in pairs {
    assert_eq!(i, double_to_int32(v));
  }
  OK
}

// ConversionsTest.DoubleToWebIDLInt64
#[test]
fn double_to_webidl_int64_test() -> Void {
  const MAX_SAFE: f64 = 9007199254740991.0; // 2^53 - 1
  const MAX_SAFE_U: u64 = (1u64 << 53) - 1;
  const MIN_SAFE: f64 = -MAX_SAFE;
  let pairs: [(f64, i64); 8] = [
    (0.0, 0),
    (-0.0, 0),
    (f64::NAN, 0),
    (f64::INFINITY, 0),
    (f64::NEG_INFINITY, 0),
    (3.5, 3),
    (1.99, 1),
    (-1.99, -1),
  ];
  for (v, i) in pairs {
    assert_eq!(i, double_to_webidl_int64(v));
  }
  assert_eq!(MIN_SAFE as i64, double_to_webidl_int64(MIN_SAFE));
  assert_eq!(MAX_SAFE_U as i64, double_to_webidl_int64(MAX_SAFE));
  assert_eq!(MIN_SAFE as i64 - 1, double_to_webidl_int64(MIN_SAFE - 1.0));
  assert_eq!(
    MAX_SAFE_U as i64 + 1,
    double_to_webidl_int64(MAX_SAFE + 1.0)
  );
  assert_eq!(i64::MIN, double_to_webidl_int64(i64::MIN as f64));
  // i64::MAX 不可被 double 表示, 最近的是 -2^63
  assert_eq!(i64::MIN, double_to_webidl_int64(i64::MAX as f64));
  // 可表示的较小值
  assert_eq!(
    ((1u64 << 63) - 1024) as i64,
    double_to_webidl_int64(((1u64 << 63) - 1024) as f64)
  );
  OK
}

// ConversionsTest.SpecialIndexParsing (raw 版; 原版经 Isolate String)
#[test]
fn special_index_parsing() -> Void {
  let u16s = |s: &str| s.encode_utf16().collect::<Vec<u16>>();
  let cases: Vec<(&str, bool)> = vec![
    ("", false),
    ("-", false),
    ("0", true),
    ("-0", true),
    ("01", false),
    ("-01", false),
    ("0.5", true),
    ("-0.5", true),
    ("1", true),
    ("-1", true),
    ("10", true),
    ("-10", true),
    ("NaN", true),
    ("Infinity", true),
    ("-Infinity", true),
    ("4294967295", true),
    ("429496.7295", true),
    ("1.3333333333333333", true),
    ("1.3333333333333339", false),
    ("1.333333333333331e+222", true),
    ("-1.3333333333333211e+222", true),
    ("-1.3333333333333311e+222", false),
    ("43s3", false),
    ("4294967296", true),
    ("-4294967296", true),
    ("999999999999999", true),
    ("9999999999999999", false),
    ("-999999999999999", true),
    ("-9999999999999999", false),
    ("42949672964294967296429496729694966", false),
  ];
  for (s, expected) in cases {
    assert_eq!(expected, is_special_index(&u16s(s)), "input: {s:?}");
  }
  OK
}

// ---------------------------------------------------------------------------
// strtod-unittest.cc 主干 (表驱动规约)
// ---------------------------------------------------------------------------

fn st(s: &str, exponent: i32) -> f64 {
  numbers::strtod::strtod(s.as_bytes(), exponent)
}

// StrtodTest.Strtod
#[test]
fn strtod() -> Void {
  // 单数字 × 指数扫描 (期望用字面量: 运行时乘积有双舍入, 见 9e-20)
  assert_eq!(1e0, st("1", 0));
  assert_eq!(1e1, st("1", 1));
  assert_eq!(1e2, st("1", 2));
  assert_eq!(1e20, st("1", 20));
  assert_eq!(1e22, st("1", 22));
  assert_eq!(1e23, st("1", 23));
  assert_eq!(1e35, st("1", 35));
  assert_eq!(1e36, st("1", 36));
  assert_eq!(1e37, st("1", 37));
  assert_eq!(1e-1, st("1", -1));
  assert_eq!(1e-2, st("1", -2));
  assert_eq!(1e-5, st("1", -5));
  assert_eq!(1e-20, st("1", -20));
  assert_eq!(1e-22, st("1", -22));
  assert_eq!(1e-23, st("1", -23));
  assert_eq!(1e-25, st("1", -25));
  assert_eq!(1e-39, st("1", -39));
  assert_eq!(2e0, st("2", 0));
  assert_eq!(2e1, st("2", 1));
  assert_eq!(2e2, st("2", 2));
  assert_eq!(2e20, st("2", 20));
  assert_eq!(2e22, st("2", 22));
  assert_eq!(2e23, st("2", 23));
  assert_eq!(2e35, st("2", 35));
  assert_eq!(2e36, st("2", 36));
  assert_eq!(2e37, st("2", 37));
  assert_eq!(2e-1, st("2", -1));
  assert_eq!(2e-2, st("2", -2));
  assert_eq!(2e-5, st("2", -5));
  assert_eq!(2e-20, st("2", -20));
  assert_eq!(2e-22, st("2", -22));
  assert_eq!(2e-23, st("2", -23));
  assert_eq!(2e-25, st("2", -25));
  assert_eq!(2e-39, st("2", -39));
  assert_eq!(9e0, st("9", 0));
  assert_eq!(9e1, st("9", 1));
  assert_eq!(9e2, st("9", 2));
  assert_eq!(9e20, st("9", 20));
  assert_eq!(9e22, st("9", 22));
  assert_eq!(9e23, st("9", 23));
  assert_eq!(9e35, st("9", 35));
  assert_eq!(9e36, st("9", 36));
  assert_eq!(9e37, st("9", 37));
  assert_eq!(9e-1, st("9", -1));
  assert_eq!(9e-2, st("9", -2));
  assert_eq!(9e-5, st("9", -5));
  assert_eq!(9e-20, st("9", -20));
  assert_eq!(9e-22, st("9", -22));
  assert_eq!(9e-23, st("9", -23));
  assert_eq!(9e-25, st("9", -25));
  assert_eq!(9e-39, st("9", -39));

  assert_eq!(0.0, st("0", 1));
  assert_eq!(0.0, st("0", 2));
  assert_eq!(0.0, st("0", -2));
  assert_eq!(0.0, st("0", -999));
  assert_eq!(0.0, st("0", 999));

  // 多数字 × 指数扫描 (期望字面量)
  assert_eq!(12345e0, st("12345", 0));
  assert_eq!(12345e1, st("12345", 1));
  assert_eq!(12345e2, st("12345", 2));
  assert_eq!(12345e20, st("12345", 20));
  assert_eq!(12345e22, st("12345", 22));
  assert_eq!(12345e23, st("12345", 23));
  assert_eq!(12345e30, st("12345", 30));
  assert_eq!(12345e31, st("12345", 31));
  assert_eq!(12345e32, st("12345", 32));
  assert_eq!(12345e35, st("12345", 35));
  assert_eq!(12345e36, st("12345", 36));
  assert_eq!(12345e37, st("12345", 37));
  assert_eq!(12345e-1, st("12345", -1));
  assert_eq!(12345e-2, st("12345", -2));
  assert_eq!(12345e-5, st("12345", -5));
  assert_eq!(12345e-20, st("12345", -20));
  assert_eq!(12345e-22, st("12345", -22));
  assert_eq!(12345e-23, st("12345", -23));
  assert_eq!(12345e-25, st("12345", -25));
  assert_eq!(12345e-39, st("12345", -39));
  assert_eq!(12345678901234e0, st("12345678901234", 0));
  assert_eq!(12345678901234e1, st("12345678901234", 1));
  assert_eq!(12345678901234e2, st("12345678901234", 2));
  assert_eq!(12345678901234e20, st("12345678901234", 20));
  assert_eq!(12345678901234e22, st("12345678901234", 22));
  assert_eq!(12345678901234e23, st("12345678901234", 23));
  assert_eq!(12345678901234e30, st("12345678901234", 30));
  assert_eq!(12345678901234e31, st("12345678901234", 31));
  assert_eq!(12345678901234e32, st("12345678901234", 32));
  assert_eq!(12345678901234e35, st("12345678901234", 35));
  assert_eq!(12345678901234e36, st("12345678901234", 36));
  assert_eq!(12345678901234e37, st("12345678901234", 37));
  assert_eq!(12345678901234e-1, st("12345678901234", -1));
  assert_eq!(12345678901234e-2, st("12345678901234", -2));
  assert_eq!(12345678901234e-5, st("12345678901234", -5));
  assert_eq!(12345678901234e-20, st("12345678901234", -20));
  assert_eq!(12345678901234e-22, st("12345678901234", -22));
  assert_eq!(12345678901234e-23, st("12345678901234", -23));
  assert_eq!(12345678901234e-25, st("12345678901234", -25));
  assert_eq!(12345678901234e-39, st("12345678901234", -39));

  // denormal 边界
  assert_eq!(0.0, st("", 1324));
  assert_eq!(0.0, st("000000000", 123));
  assert_eq!(0.0, st("2", -324));
  assert_eq!(4e-324, st("3", -324));
  assert_eq!(0.0, st("1", -325));
  assert_eq!(0.0, st("20000", -328));
  assert_eq!(40000e-328, st("30000", -328));
  assert_eq!(0.0, st("10000", -329));
  assert_eq!(0.0, st("90000", -329));
  assert_eq!(0.0, st("000000001", -325));
  assert_eq!(0.0, st("0000000020000", -328));
  assert_eq!(40000e-328, st("00000030000", -328));
  assert_eq!(0.0, st("0000000010000", -329));
  assert_eq!(0.0, st("0000000090000", -329));

  // 溢出边界
  assert_eq!(f64::INFINITY, st("1", 309));
  assert_eq!(1e308, st("1", 308));
  assert_eq!(1234e305, st("1234", 305));
  assert_eq!(1234e304, st("1234", 304));
  assert_eq!(f64::INFINITY, st("18", 307));
  assert_eq!(17e307, st("17", 307));
  assert_eq!(f64::INFINITY, st("0000001", 309));
  assert_eq!(1e308, st("00000001", 308));
  assert_eq!(1234e305, st("00000001234", 305));
  assert_eq!(1234e304, st("000000001234", 304));
  assert_eq!(f64::INFINITY, st("0000000018", 307));
  assert_eq!(17e307, st("0000000017", 307));
  assert_eq!(f64::INFINITY, st("1000000", 303));
  assert_eq!(1e308, st("100000", 303));
  assert_eq!(1234e305, st("123400000", 300));
  assert_eq!(1234e304, st("123400000", 299));
  assert_eq!(f64::INFINITY, st("180000000", 300));
  assert_eq!(17e307, st("170000000", 300));
  assert_eq!(f64::INFINITY, st("00000001000000", 303));
  assert_eq!(1e308, st("000000000000100000", 303));
  assert_eq!(1234e305, st("00000000123400000", 300));
  assert_eq!(1234e304, st("0000000123400000", 299));
  assert_eq!(f64::INFINITY, st("00000000180000000", 300));
  assert_eq!(17e307, st("00000000170000000", 300));
  assert_eq!(1.7976931348623157e308, st("17976931348623157", 292));
  assert_eq!(1.7976931348623158e308, st("17976931348623158", 292));
  assert_eq!(f64::INFINITY, st("17976931348623159", 292));

  // 双舍入著名用例 + 随机值
  assert_eq!(89255e-22, st("89255", -22));
  assert_eq!(358416272e-33, st("358416272", -33));
  // 21 位随机值 (双精度目标不可 17 位内短写, 以位型断言)
  assert_eq!(
    0x1560_b661_a319_87ae,
    st("104110013277974872254", -225).to_bits()
  );
  assert_eq!(123456789012345e0, st("123456789012345", 0));
  assert_eq!(123456789012345e1, st("123456789012345", 1));
  assert_eq!(123456789012345e2, st("123456789012345", 2));
  assert_eq!(123456789012345e20, st("123456789012345", 20));
  assert_eq!(123456789012345e22, st("123456789012345", 22));
  assert_eq!(123456789012345e23, st("123456789012345", 23));
  assert_eq!(123456789012345e35, st("123456789012345", 35));
  assert_eq!(123456789012345e36, st("123456789012345", 36));
  assert_eq!(123456789012345e37, st("123456789012345", 37));
  assert_eq!(123456789012345e39, st("123456789012345", 39));
  assert_eq!(123456789012345e-1, st("123456789012345", -1));
  assert_eq!(123456789012345e-2, st("123456789012345", -2));
  assert_eq!(123456789012345e-5, st("123456789012345", -5));
  assert_eq!(123456789012345e-20, st("123456789012345", -20));
  assert_eq!(123456789012345e-22, st("123456789012345", -22));
  assert_eq!(123456789012345e-23, st("123456789012345", -23));
  assert_eq!(123456789012345e-25, st("123456789012345", -25));
  assert_eq!(123456789012345e-39, st("123456789012345", -39));

  assert_eq!(
    0x5b5bd4363aa49df4,
    st("1234567890123456789012345", 108).to_bits()
  );
  assert_eq!(
    0x5b5bd4363aa49df4,
    st("1234567890123456789052345", 108).to_bits()
  );
  assert_eq!(
    0x5b9164a1e4a6e2b9,
    st("1234567890123456789012345", 109).to_bits()
  );
  assert_eq!(
    0x5b9164a1e4a6e2b9,
    st("1234567890123456789052345", 109).to_bits()
  );
  assert_eq!(
    0x5bc5bdca5dd09b67,
    st("1234567890123456789012345", 110).to_bits()
  );
  assert_eq!(
    0x5bc5bdca5dd09b67,
    st("1234567890123456789052345", 110).to_bits()
  );
  assert_eq!(
    0x5bfb2d3cf544c240,
    st("1234567890123456789012345", 111).to_bits()
  );
  assert_eq!(
    0x5bfb2d3cf544c240,
    st("1234567890123456789052345", 111).to_bits()
  );
  assert_eq!(
    0x5c30fc46194af968,
    st("1234567890123456789012345", 112).to_bits()
  );
  assert_eq!(
    0x5c30fc46194af968,
    st("1234567890123456789052345", 112).to_bits()
  );
  assert_eq!(
    0x5c653b579f9db7c2,
    st("1234567890123456789012345", 113).to_bits()
  );
  assert_eq!(
    0x5c653b579f9db7c2,
    st("1234567890123456789052345", 113).to_bits()
  );
  assert_eq!(
    0x5c9a8a2d878525b3,
    st("1234567890123456789012345", 114).to_bits()
  );
  assert_eq!(
    0x5c9a8a2d878525b3,
    st("1234567890123456789052345", 114).to_bits()
  );
  assert_eq!(
    0x5cd0965c74b33790,
    st("1234567890123456789012345", 115).to_bits()
  );
  assert_eq!(
    0x5cd0965c74b33790,
    st("1234567890123456789052345", 115).to_bits()
  );

  // 指数扫描 108..=115 (三族尾数)
  assert_eq!(123456789e108, st("123456789", 108));
  assert_eq!(123456789e109, st("123456789", 109));
  assert_eq!(123456789e110, st("123456789", 110));
  assert_eq!(123456789e111, st("123456789", 111));
  assert_eq!(123456789e112, st("123456789", 112));
  assert_eq!(123456789e113, st("123456789", 113));
  assert_eq!(123456789e114, st("123456789", 114));
  assert_eq!(123456789e115, st("123456789", 115));

  // 边界应舍到偶
  assert_eq!(72057594037927928.0, st("72057594037927928", 0));
  assert_eq!(72057594037927936.0, st("72057594037927936", 0));
  assert_eq!(72057594037927936.0, st("72057594037927932", 0));
  assert_eq!(72057594037927928.0, st("7205759403792793199999", -5));
  assert_eq!(72057594037927936.0, st("7205759403792793200001", -5));

  assert_eq!(9223372036854774784.0, st("9223372036854774784", 0));
  assert_eq!(9223372036854775808.0, st("9223372036854775808", 0));
  assert_eq!(9223372036854775808.0, st("9223372036854775296", 0));
  assert_eq!(9223372036854774784.0, st("922337203685477529599999", -5));
  assert_eq!(9223372036854775808.0, st("922337203685477529600001", -5));

  assert_eq!(
    10141204801825834086073718800384.0,
    st("10141204801825834086073718800384", 0)
  );
  assert_eq!(
    10141204801825835211973625643008.0,
    st("10141204801825835211973625643008", 0)
  );
  assert_eq!(
    10141204801825835211973625643008.0,
    st("10141204801825834649023672221696", 0)
  );
  assert_eq!(
    10141204801825834086073718800384.0,
    st("1014120480182583464902367222169599999", -5)
  );
  assert_eq!(
    10141204801825835211973625643008.0,
    st("1014120480182583464902367222169600001", -5)
  );

  assert_eq!(
    5708990770823838890407843763683279797179383808.0,
    st("5708990770823838890407843763683279797179383808", 0)
  );
  assert_eq!(
    5708990770823839524233143877797980545530986496.0,
    st("5708990770823839524233143877797980545530986496", 0)
  );
  assert_eq!(
    5708990770823839524233143877797980545530986496.0,
    st("5708990770823839207320493820740630171355185152", 0)
  );
  assert_eq!(
    5708990770823838890407843763683279797179383808.0,
    st("5708990770823839207320493820740630171355185151999", -3)
  );
  assert_eq!(
    5708990770823839524233143877797980545530986496.0,
    st("5708990770823839207320493820740630171355185152001", -3)
  );

  // 2011 年 Java/PHP 死循环著名用例
  assert_eq!(2.225073858507201e-308, st("22250738585072011", -324));
  assert_eq!(
    2.2250738585072014e-308,
    st(
      "2225073858507201136057409796709131975934819546351645648023426109724822222021076945516529523908135087914149158913039621106870086438694594645527657207407820621743379988141063267329253552286881372149012981122451451889849057222307285255133155755015914397476397983411801999323962548289017107081850690630666655994938275772572015763062690663332647565300009245888316433037779791869612049497390377829704905051080609940730262937128958950003583799967207254304360284078895771796150945516748243471030702609144621572289880258182545180325707018860872113128079512233426288368622321503775666622503982534335974568884423900265498198385487948292206894721689831099698365846814022854243330660339850886445804001034933970427567186443383770486037861622771738545623065874679014086723327636718751",
      -1076
    )
  );
  OK
}

// ---------------------------------------------------------------------------
// mjsunit number-tostring.js 权威向量 (V8 真值, 非 JSC 差分)
// ---------------------------------------------------------------------------

// NumberToString 向量: toString 阈值族 + toString(radix) 整数/小数/大数域
#[test]
fn mjsunit_number_tostring_vectors() -> Void {
  let mut buf = [0u8; 2200];
  let to_s = |v: f64, buf: &mut [u8]| double_to_string_view(v, buf).to_string();
  let to_r = |v: f64, r: i32, buf: &mut [u8]| double_to_radix_string_view(v, r, buf).to_string();

  // toString 十进制 (含指数切换阈值)
  let dec: Vec<(f64, &str)> = vec![
    (-90.12, "-90.12"),
    (-0.1, "-0.1"),
    (-0.01, "-0.01"),
    (-0.0123, "-0.0123"),
    (-111111111111111111111.0, "-111111111111111110000"),
    (-1111111111111111111111.0, "-1.1111111111111111e+21"),
    (-11111111111111111111111.0, "-1.1111111111111111e+22"),
    (-0.00001, "-0.00001"),
    (-0.000001, "-0.000001"),
    (-0.0000001, "-1e-7"),
    (-0.00000012, "-1.2e-7"),
    (-0.000000123, "-1.23e-7"),
    (-0.00000001, "-1e-8"),
    (-0.000000012, "-1.2e-8"),
    (-0.0000000123, "-1.23e-8"),
    (4294967296.0, "4294967296"),
    (1000000000000000128.0, "1000000000000000100"),
    (1000000000000000012800.0, "1e+21"),
    (-1000000000000000012800.0, "-1e+21"),
    (1000000000000000128000.0, "1.0000000000000001e+21"),
    (0.000001, "0.000001"),
    (0.0000001, "1e-7"),
  ];
  for (v, expect) in dec {
    assert_eq!(expect, to_s(v, &mut buf), "toString({v})");
  }

  // toString(16): 特殊值 / 整数 / 小数 / 大数 / 负数
  let hex: Vec<(f64, i32, &str)> = vec![
    (f64::NAN, 16, "NaN"),
    (f64::INFINITY, 16, "Infinity"),
    (f64::NEG_INFINITY, 16, "-Infinity"),
    (0.0, 16, "0"),
    (-0.0, 16, "0"),
    (9.0, 16, "9"),
    (-9.0, 16, "-9"),
    (90.0, 16, "5a"),
    (-90.0, 16, "-5a"),
    (90.12, 16, "5a.1eb851eb852"),
    (-90.12, 16, "-5a.1eb851eb852"),
    (0.1, 16, "0.1999999999999a"),
    (-0.1, 16, "-0.1999999999999a"),
    (0.01, 16, "0.028f5c28f5c28f6"),
    (-0.01, 16, "-0.028f5c28f5c28f6"),
    (0.0123, 16, "0.032617c1bda511a"),
    (-0.0123, 16, "-0.032617c1bda511a"),
    (111111111111111111111.0, 16, "605f9f6dd18bc8000"),
    (-111111111111111111111.0, 16, "-605f9f6dd18bc8000"),
    (1111111111111111111111.0, 16, "3c3bc3a4a2f75c0000"),
    (-1111111111111111111111.0, 16, "-3c3bc3a4a2f75c0000"),
    (11111111111111111111111.0, 16, "25a55a46e5da9a00000"),
    (-11111111111111111111111.0, 16, "-25a55a46e5da9a00000"),
    (0.00001, 16, "0.0000a7c5ac471b4788"),
    (-0.00001, 16, "-0.0000a7c5ac471b4788"),
    (0.000001, 16, "0.000010c6f7a0b5ed8d"),
    (-0.000001, 16, "-0.000010c6f7a0b5ed8d"),
    (0.0000001, 16, "0.000001ad7f29abcaf48"),
    (-0.0000001, 16, "-0.000001ad7f29abcaf48"),
    (0.00000012, 16, "0.000002036565348d256"),
    (-0.00000012, 16, "-0.000002036565348d256"),
    (0.000000123, 16, "0.0000021047ee22aa466"),
    (-0.000000123, 16, "-0.0000021047ee22aa466"),
    (0.00000001, 16, "0.0000002af31dc4611874"),
    (-0.00000001, 16, "-0.0000002af31dc4611874"),
    (0.000000012, 16, "0.000000338a23b87483be"),
    (-0.000000012, 16, "-0.000000338a23b87483be"),
    (0.0000000123, 16, "0.00000034d3fe36aaa0a2"),
    (-0.0000000123, 16, "-0.00000034d3fe36aaa0a2"),
    (4294967295.0, 16, "ffffffff"),
    (4294967296.0, 16, "100000000"),
    (4294967297.0, 16, "100000001"),
    (72057594037928065.0, 16, "100000000000080"),
    (-72057594037928065.0, 16, "-100000000000080"),
    (8.5, 16, "8.8"),
    (-8.5, 16, "-8.8"),
  ];
  for (v, r, expect) in hex {
    assert_eq!(expect, to_r(v, r, &mut buf), "toString({v}, {r})");
  }

  // toString 其它进制
  let misc: Vec<(f64, i32, &str)> = vec![
    (4294967295.0, 2, "11111111111111111111111111111111"),
    (4294967295.0, 36, "1z141z3"),
    (10000007.0, 36, "5yc1z"),
    (-10000007.0, 36, "-5yc1z"),
    (0.0, 36, "0"),
    (0.0, 2, "0"),
    (4294967296.0, 2, "100000000000000000000000000000000"),
    (4294967297.0, 2, "100000000000000000000000000000001"),
    (
      72057594037928065.0,
      2,
      "100000000000000000000000000000000000000000000000010000000",
    ),
    (
      -72057594037928065.0,
      2,
      "-100000000000000000000000000000000000000000000000010000000",
    ),
    (-(4294967295.0), 2, "-11111111111111111111111111111111"),
    (-4294967296.0, 2, "-100000000000000000000000000000000"),
    (-(4294967297.0), 2, "-100000000000000000000000000000001"),
    (4.0 / 3.0, 3, "1.1"),
    (13.0 / 3.0, 3, "11.1"),
    (1.0 / 9.0, 3, "0.01"),
    (81.0, 3, "10000"),
  ];
  for (v, r, expect) in misc {
    assert_eq!(expect, to_r(v, r, &mut buf), "toString({v}, {r})");
  }

  // toFixed 特殊值与回退阈值 (V8 向量; 一般数值路径待 v8-base dtoa 修复)
  assert_eq!("NaN", double_to_fixed_string_view(f64::NAN, 2, &mut buf));
  assert_eq!(
    "Infinity",
    double_to_fixed_string_view(f64::INFINITY, 2, &mut buf)
  );
  assert_eq!(
    "-Infinity",
    double_to_fixed_string_view(f64::NEG_INFINITY, 2, &mut buf)
  );
  assert_eq!(
    "1.1111111111111111e+21",
    double_to_fixed_string_view(1111111111111111111111.0, 8, &mut buf)
  );
  OK
}

// ---------------------------------------------------------------------------
// dtoa-unittest.cc DtoaVariousDoubles 内核抽样 (经 v8-base DoubleToAscii;
// 本组取值不触发上游 dtoa 缺陷, 见 dtoa_golden.rs 注释)
// ---------------------------------------------------------------------------

fn dtoa(v: f64, mode: numbers::dtoa::DtoaMode, digits: i32) -> (String, i32, usize) {
  let mut buf = [0u8; 124];
  let r = numbers::dtoa::double_to_ascii(v, mode, digits, &mut buf);
  (
    String::from_utf8_lossy(&buf[..r.length]).into_owned(),
    r.point,
    r.length,
  )
}

fn trim_rep(mut s: String) -> String {
  while s.ends_with('0') {
    s.pop();
  }
  s
}

// DtoaTest.DtoaVariousDoubles (代表性断言)
#[test]
fn dtoa_various_doubles() -> Void {
  let (s, p, _) = dtoa(0.0, numbers::dtoa::DtoaMode::Shortest, 0);
  assert_eq!(("0".to_string(), 1), (s, p));

  let (s, p, l) = dtoa(0.0, numbers::dtoa::DtoaMode::Fixed, 2);
  assert_eq!(("0".to_string(), 1, 1usize), (s, p, l));

  let (s, p, l) = dtoa(0.0, numbers::dtoa::DtoaMode::Precision, 3);
  assert_eq!(("0".to_string(), 1, 1usize), (s, p, l));

  let (s, p, _) = dtoa(1.5, numbers::dtoa::DtoaMode::Shortest, 0);
  assert_eq!(("15".to_string(), 1), (s, p));

  let min_double = 5e-324;
  let (s, p, _) = dtoa(min_double, numbers::dtoa::DtoaMode::Shortest, 0);
  assert_eq!(("5".to_string(), -323), (s, p));

  let (s, p, _) = dtoa(min_double, numbers::dtoa::DtoaMode::Precision, 5);
  assert_eq!(("49407".to_string(), -323), (trim_rep(s), p));

  let max_double = 1.7976931348623157e308;
  let (s, p, _) = dtoa(max_double, numbers::dtoa::DtoaMode::Shortest, 0);
  assert_eq!(("17976931348623157".to_string(), 309), (s, p));

  let (s, p, _) = dtoa(max_double, numbers::dtoa::DtoaMode::Precision, 7);
  assert_eq!(("1797693".to_string(), 309), (trim_rep(s), p));

  let (s, p, _) = dtoa(4294967272.0, numbers::dtoa::DtoaMode::Shortest, 0);
  assert_eq!(("4294967272".to_string(), 10), (s, p));

  let (s, p, _) = dtoa(4.185580496821357e298, numbers::dtoa::DtoaMode::Shortest, 0);
  assert_eq!(("4185580496821357".to_string(), 299), (s, p));

  let (s, p, _) = dtoa(
    4.185580496821357e298,
    numbers::dtoa::DtoaMode::Precision,
    20,
  );
  assert_eq!(("41855804968213567225".to_string(), 299), (trim_rep(s), p));
  OK
}

// ---------------------------------------------------------------------------
// 格式层直测: 指数切换阈值 / toFixed 阈值 / toPrecision 切换 / radix
// ---------------------------------------------------------------------------

#[test]
fn format_layer_thresholds() -> Void {
  let mut buf = [0u8; 2200];
  // ECMAScript 指数切换阈值: 1e21 与 1e-7
  assert_eq!(
    "100000000000000000000",
    double_to_string_view(1e20, &mut buf)
  );
  assert_eq!("1e+21", double_to_string_view(1e21, &mut buf));
  assert_eq!("0.000001", double_to_string_view(1e-6, &mut buf));
  assert_eq!("1e-7", double_to_string_view(1e-7, &mut buf));
  assert_eq!("-1e-7", double_to_string_view(-1e-7, &mut buf));
  assert_eq!("NaN", double_to_string_view(f64::NAN, &mut buf));
  assert_eq!("Infinity", double_to_string_view(f64::INFINITY, &mut buf));
  assert_eq!(
    "-Infinity",
    double_to_string_view(f64::NEG_INFINITY, &mut buf)
  );
  assert_eq!("1.5e+300", double_to_string_view(1.5e300, &mut buf));
  assert_eq!(
    "123456789012345680000",
    double_to_string_view(123456789012345678905.0, &mut buf)
  );

  // toFixed 阈值: >= 1e21 回退 toString
  assert_eq!("1e+21", double_to_fixed_string_view(1e21, 2, &mut buf));
  assert_eq!("0.00", double_to_fixed_string_view(0.0, 2, &mut buf));
  assert_eq!("3.15", double_to_fixed_string_view(3.145, 2, &mut buf));
  assert_eq!("-3.15", double_to_fixed_string_view(-3.145, 2, &mut buf));
  assert_eq!("0.14", double_to_fixed_string_view(0.14, 2, &mut buf));
  assert_eq!(
    "100000000000000000000",
    double_to_fixed_string_view(1e20, 0, &mut buf)
  );
  OK
}

// parseInt raw 路径 (NumberParseIntHelper 三分派)
#[test]
fn string_to_int_paths() -> Void {
  fn b(s: &str) -> &[u8] {
    s.as_bytes()
  }
  // radix 探测 / 指定
  assert_eq!(255.0, string_to_int(b("0xFF"), 0));
  assert_eq!(255.0, string_to_int(b("0xFF"), 16));
  assert_eq!(255.0, string_to_int(b("FF"), 16));
  assert_eq!(10.0, string_to_int(b("010"), 0)); // 现代语义: 无 legacy 八进制
  assert_eq!(8.0, string_to_int(b("10"), 8));
  assert_eq!(5.0, string_to_int(b("101"), 2));
  assert_eq!(2.0, string_to_int(b("10"), 2));
  assert_eq!(1296.0, string_to_int(b("100"), 36)); // 通用路径
  assert!(string_to_int(b(""), 10).is_nan()); // junk → NaN
  assert_eq!(0.0, string_to_int(b("0"), 10));
  assert_eq!(-255.0, string_to_int(b("-0xFF"), 16));
  assert_eq!(12345.0, string_to_int(b("  12345  "), 10));
  assert_eq!(12345.0, string_to_int(b("+12345"), 10));
  assert_eq!(35.0, string_to_int(b("z"), 36)); // z 是 radix 36 合法数字
  // 通用进制近似路径 (radix 3): 102_3 = 11? 1×9+0×3+2 = 11
  assert_eq!(11.0, string_to_int(b("102"), 3));
  assert_eq!(14.0, string_to_int(b("24"), 5));
  assert_eq!(13.0, string_to_int(b("16"), 7));
  assert_eq!(17.0, string_to_int(b("18"), 9));
  // 非法 radix 规范防护: radix < 2 或 radix > 36 (ECMA-262 18.2.5 步 8) 恒为 NaN
  assert!(string_to_int(b("123"), 1).is_nan());
  assert!(string_to_int(b("123"), 37).is_nan());
  assert!(string_to_int(b("123"), -1).is_nan());
  assert!(string_to_int(b("123"), -100).is_nan());
  // 空/短前缀边界防护 (防止切片越界 panic)
  assert!(hex_string_to_double(b"").is_nan());
  assert!(hex_string_to_double(b"0").is_nan());
  assert!(hex_string_to_double(b"0x").is_nan());
  assert!(binary_string_to_double(b"").is_nan());
  assert!(binary_string_to_double(b"0").is_nan());
  assert!(binary_string_to_double(b"0b").is_nan());
  assert!(octal_string_to_double(b"").is_nan());
  assert!(octal_string_to_double(b"0").is_nan());
  assert!(octal_string_to_double(b"0o").is_nan());
  assert!(implicit_octal_string_to_double(b"").is_nan());
  OK
}

// ---------------------------------------------------------------------------
// mjsunit parse-int-float.js 权威向量 (字符串子集; 对象转换属内建层)
// ---------------------------------------------------------------------------

#[test]
fn mjsunit_parse_int_float_vectors() -> Void {
  fn pi(s: &str, radix: i32) -> f64 {
    string_to_int(s.as_bytes(), radix)
  }
  fn pf(s: &str) -> f64 {
    string_to_double(s.as_bytes(), JUNK, f64::NAN)
  }
  // parseInt 空白/前导零
  assert_eq!(0.0, pi("0", 10));
  assert_eq!(0.0, pi(" 0", 10));
  assert_eq!(0.0, pi(" 0 ", 10));
  assert_eq!(77.0, pi("077", 10));
  assert_eq!(77.0, pi("  077", 10));
  assert_eq!(77.0, pi("  077   ", 10));
  assert_eq!(-77.0, pi("  -077", 10));
  // 进制探测与指定
  assert_eq!(3.0, pi("11", 2));
  assert_eq!(4.0, pi("11", 3));
  assert_eq!(0x12 as f64, pi("0x12", 0));
  assert_eq!(0x12 as f64, pi("0x12", 16));
  // "0x" 悬空前缀 → NaN (含 radix 16)
  assert!(pi("0x  ", 0).is_nan());
  assert!(pi("0x", 0).is_nan());
  assert!(pi("0x  ", 16).is_nan());
  assert!(pi("0x", 16).is_nan());
  // 尾随垃圾截断
  assert_eq!(12.0, pi("12aaa", 10));
  // 超大数 → Infinity
  // mjsunit 原 vector: 1 后跟数百个 0 (≈1e540) → Infinity
  let big_one = format!("1{}", "0".repeat(400));
  assert!(pi(&big_one, 10).is_infinite());
  let big_hex = format!("0x1{}", "0".repeat(300)); // 16^300 ≈ 1e361
  assert!(pi(&big_hex, 0).is_infinite());
  // parseFloat: junk 截断 / 无 0x 前缀语义 / 前导零
  assert_eq!(0.1, pf("0.1"));
  assert_eq!(0.1, pf("0.1aaa"));
  assert_eq!(0.0, pf("0aaa"));
  assert_eq!(0.0, pf("0x12"));
  assert_eq!(77.0, pf("077"));
  OK
}

// ---------------------------------------------------------------------------
// ConversionsTest.NoHandlesForTryNumberToSize / TryNumberToSizeWithMaxSizePlusOne
// 数值分支 (Smi/HeapNumber 对象分派推迟对象层, double 语义在此)
// ---------------------------------------------------------------------------

#[test]
fn try_number_to_size() -> Void {
  // Smi 1 → 1; HeapNumber 2.0 → 2 (数值部分)
  assert_eq!(Some(1), try_double_to_size(1.0));
  assert_eq!(Some(2), try_double_to_size(2.0));
  // size_t::MAX + 10000 (double 化后) → None
  assert_eq!(None, try_double_to_size(usize::MAX as f64 + 10000.0));
  // 1 << 64 超出 size_t 上限 → None
  assert_eq!(None, try_double_to_size(18446744073709551616.0));
  // V8_ENABLE_SANDBOX 分支 (默认配置展开): > 32GB-1 拒绝, 边界内放行
  assert_eq!(None, try_double_to_size((32u64 << 30) as f64));
  assert_eq!(
    Some((32usize << 30) - 1),
    try_double_to_size(((32u64 << 30) - 1) as f64)
  );
  OK
}

#[test]
fn adversarial_leading_zeros_and_special_index() -> Void {
  // 600 个前导零后跟 1: 真实值 1.0 (曾因未削零导致 magnitude > 500 误判为 Infinity)
  let zeros_600 = "0".repeat(600);
  assert_eq!(1.0, d2s(&format!("{zeros_600}1")));
  assert_eq!(-1.0, d2s(&format!("-{zeros_600}1")));
  assert_eq!(0.123, d2s(&format!("{zeros_600}.123")));
  assert_eq!(0.00123, d2s(&format!("{zeros_600}.00123")));
  assert_eq!(1.0, d2s(&format!("1.{}", "0".repeat(600))));
  assert_eq!(-1.5, d2s(&format!("-{zeros_600}1.5")));
  assert_eq!(0.0, d2s(&format!("{zeros_600}0.0")));

  // 数量级真溢出 (600 个零 + 1 + 600 个尾零 = 1e600)
  assert!(d2s(&format!("{zeros_600}1{}", "0".repeat(600))).is_infinite());

  // is_special_index 边界与典型值
  assert!(is_special_index("0".as_bytes()));
  assert!(is_special_index("-0".as_bytes()));
  assert!(!is_special_index("00".as_bytes()));
  assert!(is_special_index("NaN".as_bytes()));
  assert!(is_special_index("Infinity".as_bytes()));
  assert!(is_special_index("-Infinity".as_bytes()));
  assert!(is_special_index("123".as_bytes()));
  assert!(is_special_index("123.456".as_bytes()));
  assert!(is_special_index("0.1".as_bytes()));
  assert!(!is_special_index("0.10".as_bytes()));
  assert!(!is_special_index("".as_bytes()));
  assert!(!is_special_index("1234567890123456789012345".as_bytes())); // > 24 字符

  OK
}

#[test]
fn adversarial_extremes_and_special_branches() -> Void {
  use v8_numbers::{
    ieee754::pow,
    math_random::{default_state_from_seed, murmur_hash3, random_to_double, xor_shift128},
  };

  // 1. 5e-324 最小非正规数与下溢边界
  assert_eq!(5e-324, d2s("5e-324"));
  assert_eq!(5e-324, d2s("4.9406564584124654e-324"));
  assert_eq!(f64::from_bits(1), d2s("5e-324"));
  assert_eq!(0.0, d2s("2.47e-324")); // 向下舍入到 0
  assert_eq!(5e-324, d2s("2.48e-324")); // 向上舍入到 5e-324
  assert_eq!(0.0, d2s("1e-325"));
  assert_eq!(0.0, d2s("1e-400"));
  assert_eq!(-5e-324, d2s("-5e-324"));
  assert_eq!((-0.0f64).to_bits(), d2s("-1e-325").to_bits());
  assert_eq!((-0.0f64).to_bits(), d2s("-1e-400").to_bits());

  // 2. 1.7976931348623157e+308 上溢边界
  // IEEE-754 binary64 到无穷的半途点约为 1.797693134862315808e+308:
  // 1.7976931348623158e+308 < 半途点 → 仍就近舍入到 f64::MAX
  // 1.7976931348623159e+308 > 半途点 → 舍入到 +Infinity
  assert_eq!(f64::MAX, d2s("1.7976931348623157e+308"));
  assert_eq!(f64::MAX, d2s("1.7976931348623158e+308"));
  assert!(d2s("1.7976931348623159e+308").is_infinite());
  assert_eq!(-f64::MAX, d2s("-1.7976931348623157e+308"));
  assert_eq!(-f64::MAX, d2s("-1.7976931348623158e+308"));
  assert_eq!(f64::NEG_INFINITY, d2s("-1.7976931348623159e+308"));
  assert!(d2s("1e309").is_infinite());
  assert_eq!(f64::NEG_INFINITY, d2s("-1e309"));

  // 3. 非纯 ASCII (UTF-16 空白字符与非 ASCII 字符)
  // ECMA-262 空白字符: 0x1680, 0x2000, 0x2028, 0x2029, 0x202F, 0x205F, 0x3000, 0xFEFF
  let spaces: [u16; 8] = [
    0x1680, 0x2000, 0x2028, 0x2029, 0x202F, 0x205F, 0x3000, 0xFEFF,
  ];
  for &sp in &spaces {
    let u16_str: Vec<u16> = vec![sp, b'4' as u16, b'2' as u16, sp];
    assert_eq!(42.0, string_to_double(&u16_str, NO_FLAG, 0.0));
    assert_eq!(42.0, string_to_int(&u16_str, 10));
  }
  // 全角数字 (0xFF10 = '０') 不属于 ASCII 数字, 必须作为非法字符处理
  let fullwidth_zero: Vec<u16> = vec![0xFF10, b'1' as u16];
  assert!(string_to_double(&fullwidth_zero, NO_FLAG, 0.0).is_nan());
  assert!(string_to_int(&fullwidth_zero, 10).is_nan());

  // 4. 空串 / 短串 / 标点边界
  assert_eq!(0.0, string_to_double(b"", NO_FLAG, 0.0));
  assert!(string_to_double(b"", NO_FLAG, f64::NAN).is_nan());
  assert!(string_to_int(b"", 10).is_nan());
  assert!(string_to_double(b"+", NO_FLAG, 0.0).is_nan());
  assert!(string_to_double(b"-", NO_FLAG, 0.0).is_nan());
  assert!(string_to_double(b".", NO_FLAG, 0.0).is_nan());
  assert!(string_to_double(b"+.", NO_FLAG, 0.0).is_nan());
  assert!(string_to_double(b"-.", NO_FLAG, 0.0).is_nan());
  assert!(string_to_double(b"+e1", NO_FLAG, 0.0).is_nan());
  assert!(string_to_double(b"-e1", NO_FLAG, 0.0).is_nan());
  assert!(string_to_double(b"0x", PREFIX, 0.0).is_nan());
  assert!(string_to_double(b"0b", PREFIX, 0.0).is_nan());
  assert!(string_to_double(b"0o", PREFIX, 0.0).is_nan());
  assert!(string_to_int(b"0x", 0).is_nan());
  assert!(string_to_int(b"0x", 16).is_nan());
  // ECMA-262 parseInt 不认 0b/0o 前缀: "0" 消费后 "b"/"o" 作为 trailing junk 截断, 返回 0.0
  assert_eq!(0.0, string_to_int(b"0b", 0));
  assert_eq!(0.0, string_to_int(b"0o", 0));
  assert_eq!(0.0, string_to_double(b"+0", NO_FLAG, f64::NAN));
  assert_eq!(
    (-0.0f64).to_bits(),
    string_to_double(b"-0", NO_FLAG, f64::NAN).to_bits()
  );

  // 5. 各种 Radix 2..=36 边界与非法 Radix
  for radix in 2..=36 {
    // "10" 在 radix 进制下恰等于 radix
    assert_eq!(radix as f64, string_to_int(b"10", radix));
    assert_eq!(-(radix as f64), string_to_int(b"-10", radix));
  }
  // 非法 radix
  assert!(string_to_int(b"10", -1).is_nan());
  assert!(string_to_int(b"10", 1).is_nan());
  assert!(string_to_int(b"10", 37).is_nan());
  assert!(string_to_int(b"10", i32::MIN).is_nan());
  assert!(string_to_int(b"10", i32::MAX).is_nan());

  // 6. IEEE-754 超越函数特化分支 (ECMA-262 历史规则)
  // (±1, ±∞) = NaN
  assert!(pow(1.0, f64::INFINITY).is_nan());
  assert!(pow(1.0, f64::NEG_INFINITY).is_nan());
  assert!(pow(-1.0, f64::INFINITY).is_nan());
  assert!(pow(-1.0, f64::NEG_INFINITY).is_nan());
  // (-Inf)**0.5 = +Inf
  assert_eq!(f64::INFINITY, pow(f64::NEG_INFINITY, 0.5));
  assert_eq!(f64::INFINITY, pow(f64::INFINITY, 0.5));
  // (-0)**0.5 = +0
  assert_eq!(0.0f64.to_bits(), pow(-0.0, 0.5).to_bits());
  // x**2 快速路径
  assert_eq!(9.0, pow(3.0, 2.0));
  assert_eq!(9.0, pow(-3.0, 2.0));
  assert_eq!(0.0f64.to_bits(), pow(-0.0, 2.0).to_bits());
  // 指数为 NaN
  assert!(pow(2.0, f64::NAN).is_nan());
  assert!(pow(1.0, f64::NAN).is_nan());
  assert!(pow(f64::NAN, 2.0).is_nan());

  // 7. XorShift128+ 伪随机数发生器与 MurmurHash3 跨平台确定性
  assert_eq!(0, murmur_hash3(0));
  assert_eq!(0xb456_bcfc_34c2_cb2c, murmur_hash3(1));
  assert_eq!(0x17d2_abfb_f90b_aef9, murmur_hash3(0x3039));
  assert_eq!(0x7082_9950_08f0_c48c, murmur_hash3(0xdead_beef_cafe_babe));
  let state = default_state_from_seed(0x3039);
  assert_eq!(0x17d2_abfb_f90b_aef9, state.s0);
  assert_eq!(0x34d5_7084_f8b8_01cb, state.s1);
  let mut s0 = state.s0;
  let mut s1 = state.s1;
  let r0 = xor_shift128(&mut s0, &mut s1);
  assert_eq!(0x13d0_9c37_5831_ce94, r0);
  assert_eq!(0x874d_6807_ca42_fbd2, xor_shift128(&mut s0, &mut s1));
  let d0 = random_to_double(r0);
  assert!((0.0..1.0).contains(&d0));

  OK
}
