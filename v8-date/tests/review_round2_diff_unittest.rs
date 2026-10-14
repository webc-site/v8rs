// review_round2_diff_unittest.rs — round2 全语法差分矩阵 (以原版 dateparser-inl.h/dateparser.cc/dateparser.h 逐行走查手工推演为准)
// 覆盖: ES5 合法/非法、legacy 变体 (月名/时区/括号注释/垃圾词)、毫秒缩放、时区回绕端到端、
//       DateCache 缓存窗口滑动语义 (跨 DST 段扩展/收缩/合并/大跳/回退)、关键字表冲突面
// UTC_OFFSET 期望值含 NaN/-0.0, 用 to_bits 精确对拍 (C++ double 逐位语义)
use aok::{OK, Void};
use v8_date::{
  algorithms::days_from_year_month,
  date_cache::{
    cache::DateCache,
    constants::MS_PER_DAY,
    provider::{TimeZoneDetection, TimezoneProvider},
  },
  dateparser::{
    parser::{DateParser, parse_date_time_string},
    tables::*,
  },
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 固定 UTC+8 且无夏令时的确定性 mock (端到端时间戳断言不依赖宿主时区)
#[derive(Debug, Default)]
struct Utc8MockProvider;

impl TimezoneProvider for Utc8MockProvider {
  fn get_daylight_savings_offset_from_os(&mut self, _time_sec: i64) -> i32 {
    0
  }
  fn get_local_offset_from_os(&mut self, _time_ms: i64, _is_utc: bool) -> i32 {
    8 * 3600 * 1000
  }
  fn local_timezone(&mut self, _time_ms: f64) -> &str {
    "CST"
  }
  fn clear(&mut self, _detection: TimeZoneDetection) {}
}

/// 阶跃 DST mock: [t1, t2) 秒区间内 +1h, 其余 0 (窗口滑动语义测试)
#[derive(Debug, Clone, Copy)]
struct StepDstProvider {
  t1_sec: i64,
  t2_sec: i64,
}

impl StepDstProvider {
  /// 黑盒 oracle: 该时刻期望的本地偏移
  fn expect_offset_ms(&self, time_ms: i64) -> i32 {
    let sec = time_ms.div_euclid(1000);
    if (self.t1_sec..self.t2_sec).contains(&sec) {
      3_600_000
    } else {
      0
    }
  }
}

impl TimezoneProvider for StepDstProvider {
  fn get_daylight_savings_offset_from_os(&mut self, time_sec: i64) -> i32 {
    if (self.t1_sec..self.t2_sec).contains(&time_sec) {
      3_600_000
    } else {
      0
    }
  }
  fn get_local_offset_from_os(&mut self, time_ms: i64, _is_utc: bool) -> i32 {
    self.expect_offset_ms(time_ms)
  }
  fn local_timezone(&mut self, _time_ms: f64) -> &str {
    "MOCK"
  }
  fn clear(&mut self, _detection: TimeZoneDetection) {}
}

/// 组件级断言: 解析成功且 output 逐槽位等于期望 (UTC_OFFSET 逐位对拍, 兼容 NaN/-0.0)
/// want 按 [YEAR, MONTH, DAY, HOUR, MINUTE, SECOND, MILLISECOND, UTC_OFFSET] 槽位序
fn expect_out(s: &str, want: [f64; OUTPUT_SIZE]) {
  let mut out = [0.0f64; OUTPUT_SIZE];
  assert!(DateParser::parse(s, &mut out), "parse failed: {s:?}");
  for (i, w) in want.iter().enumerate() {
    if i == UTC_OFFSET {
      assert_eq!(out[i].to_bits(), w.to_bits(), "slot {i} of {s:?}");
    } else {
      assert_eq!(out[i], *w, "slot {i} of {s:?}");
    }
  }
}

/// 组件级断言: 解析必须失败
fn expect_fail(s: &str) {
  let mut out = [0.0f64; OUTPUT_SIZE];
  assert!(!DateParser::parse(s, &mut out), "expected fail: {s:?}");
}

/// ES5 合法差分 (组件 + UTC 偏移; 每条按原版 ParseES5DateTime 状态机手工推演)
#[test]
fn test_diff_es5_valid() -> Void {
  // 基础完整形式与省略形式
  expect_out(
    "2023-10-18T14:30:45.123Z",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 123.0, 0.0],
  );
  expect_out(
    "2023-10-18t14:30:45.123Z",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 123.0, 0.0],
  ); // 小写 t
  expect_out(
    "2023-10-18T14:30:45Z",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 0.0, 0.0],
  );
  expect_out(
    "2023-10-18T14:30Z",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 0.0, 0.0, 0.0],
  );
  expect_out("2023-10-18", [2023.0, 9.0, 18.0, 0.0, 0.0, 0.0, 0.0, 0.0]); // date-only 默认 UTC
  expect_out("2023-10", [2023.0, 9.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
  expect_out("2023", [2023.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
  expect_out(
    "0000-01-01T00:00:00Z",
    [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  );

  // date-time 无时区 → UTC_OFFSET NaN (本地语义)
  expect_out(
    "2023-10-18T14:30:45",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 0.0, f64::NAN],
  );
  expect_out(
    "2023-10-18T14:30",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 0.0, 0.0, f64::NAN],
  );

  // 毫秒缩放: 前导零占长度, 非零位定数值 (原版 ReadMilliseconds + ReadUnsignedNumeral)
  expect_out(
    "1970-01-01T00:00:00.5Z",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 500.0, 0.0],
  ); // 1 位 ×100
  expect_out(
    "1970-01-01T00:00:00.05Z",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 50.0, 0.0],
  ); // 2 位 ×10 (n=5)
  expect_out(
    "1970-01-01T00:00:00.005Z",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 5.0, 0.0],
  ); // 3 位原值 (n=5)
  expect_out(
    "1970-01-01T00:00:00.0005Z",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  ); // 4 位 ÷10
  expect_out(
    "1970-01-01T00:00:00.123456Z",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 123.0, 0.0],
  ); // 6 位 ÷10^3
  expect_out(
    "1970-01-01T00:00:00.123456789Z",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 123.0, 0.0],
  ); // 9 位 ÷10^6
  expect_out(
    "1970-01-01T00:00:00.123456789012Z",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 123.0, 0.0],
  ); // >9 位截 9
  expect_out(
    "2023-01-01T00:00:00.0000000001Z",
    [2023.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  ); // n=1 len10 → 0
  expect_out(
    "2023-01-01T24:00:00.0Z",
    [2023.0, 0.0, 1.0, 24.0, 0.0, 0.0, 0.0, 0.0],
  ); // 24:00 允许 0 毫秒

  // 24:00 系列 (一天结束的午夜; 秒/分/毫秒全零)
  expect_out(
    "2023-01-01T24:00Z",
    [2023.0, 0.0, 1.0, 24.0, 0.0, 0.0, 0.0, 0.0],
  );
  expect_out(
    "2023-01-01T24:00:00Z",
    [2023.0, 0.0, 1.0, 24.0, 0.0, 0.0, 0.0, 0.0],
  );
  expect_out(
    "2023-01-01T24:00:00.000Z",
    [2023.0, 0.0, 1.0, 24.0, 0.0, 0.0, 0.0, 0.0],
  );
  expect_out(
    "1970-01-01T24:00:00+01:00",
    [1970.0, 0.0, 1.0, 24.0, 0.0, 0.0, 0.0, 3600.0],
  ); // 24:00 可带时区

  // 扩展年 ±6 位; -000000 拒绝 (年零须正), +000000 合法
  expect_out(
    "+002023-10-18T00:00:00Z",
    [2023.0, 9.0, 18.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  );
  expect_out(
    "-002023-01-01",
    [-2023.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  );
  expect_out("+000000-01-01", [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
  expect_out("+002023", [2023.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]); // 裸扩展年
  // -000000-01-01: ES5 拒 (年零须正, 但 6 位数字已被消费) → legacy 落 [1,1] MD(Y) → 2001
  expect_out(
    "-000000-01-01",
    [2001.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  );

  // 时区三形态: Z / ±hh:mm / ±hhmm; 越界分量组合
  expect_out(
    "1995-12-25T13:30:00+05:30",
    [1995.0, 11.0, 25.0, 13.0, 30.0, 0.0, 0.0, 19800.0],
  );
  expect_out(
    "1995-12-25T13:30:00-05:30",
    [1995.0, 11.0, 25.0, 13.0, 30.0, 0.0, 0.0, -19800.0],
  );
  expect_out(
    "2023-10-18T14:30:45+0500",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 0.0, 18000.0],
  );
  expect_out(
    "2023-10-18T14:30:45-0500",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 0.0, -18000.0],
  );
  expect_out(
    "2023-10-18T14:30:45+2359",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 0.0, 86340.0],
  );
  expect_out(
    "2023-10-18T00:00:00+00:00",
    [2023.0, 9.0, 18.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  );
  expect_out(
    "2023-10-18T14:30:45.500-08:00",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 500.0, -28800.0],
  );
  expect_out(
    "2023-06-15T00:00:00.000+14:00",
    [2023.0, 5.0, 15.0, 0.0, 0.0, 0.0, 0.0, 50400.0],
  ); // 最大 +14
  expect_out(
    "2023-06-15T00:00:00.000-12:01",
    [2023.0, 5.0, 15.0, 0.0, 0.0, 0.0, 0.0, -43260.0],
  ); // -12 后分钟不限
  expect_out(
    "2023-01-01T24:00:00.000+15:00",
    [2023.0, 0.0, 1.0, 24.0, 0.0, 0.0, 0.0, 54000.0],
  ); // 24:00 带大偏移

  // 规范边界日期 (TimeRange: ±100000000 天)
  expect_out(
    "1969-12-31T23:59:59.999Z",
    [1969.0, 11.0, 31.0, 23.0, 59.0, 59.0, 999.0, 0.0],
  );
  expect_out(
    "2000-02-29T00:00:00Z",
    [2000.0, 1.0, 29.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  ); // 闰年 2/29
  expect_out(
    "2024-02-29T12:00:00Z",
    [2024.0, 1.0, 29.0, 12.0, 0.0, 0.0, 0.0, 0.0],
  );
  expect_out(
    "1900-02-28T00:00:00Z",
    [1900.0, 1.0, 28.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  ); // 平年世纪
  expect_out(
    "9999-12-31T23:59:59.999Z",
    [9999.0, 11.0, 31.0, 23.0, 59.0, 59.0, 999.0, 0.0],
  );
  expect_out(
    "+275760-09-13T00:00:00Z",
    [275760.0, 8.0, 13.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  ); // 正时刻上界 (带 + 号)
  expect_out(
    "-271821-04-20T00:00:00Z",
    [-271821.0, 3.0, 20.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  ); // 负时刻下界

  // 平年 2/29: 解析层接受 (IsDay 29), 语义滚动 (e2e 组验证 → 3 月 1 日)
  expect_out(
    "2023-02-29T00:00:00Z",
    [2023.0, 1.0, 29.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  );

  OK
}

/// ES5 非法差分 (必须失败; ES5 失败后 legacy 回退也必须失败)
#[test]
fn test_diff_es5_invalid() -> Void {
  // 越界分量
  expect_fail("2023-13-01");
  expect_fail("2023-00-01");
  expect_fail("2023-01-00");
  expect_fail("2023-01-32");
  expect_fail("2023-13-32");
  expect_fail("2023-01-01T24:01:00Z");
  expect_fail("2023-01-01T24:00:01Z");
  expect_fail("2023-01-01T24:00:00.001Z");
  expect_fail("2023-01-01T25:00:00Z");
  expect_fail("2023-01-01T00:60:00Z");
  expect_fail("2023-01-01T00:00:60Z");
  expect_fail("2023-01-01T24:00:00.000+25:00"); // hh=25 超 IsHour 上界
  expect_fail("2023-01-01T00:00:00+99:00"); // hh=99 非法

  // 结构残缺
  expect_fail("2023-01-01T"); // T 后 EOF
  expect_fail("2023-01-01t"); // 小写同
  expect_fail("2023-01-01T14Z"); // T 后缺 :mm
  expect_fail("2023-01-01T14:30.000Z"); // 缺 :ss 却有毫秒
  expect_fail("2023-01-01T14:30:45.Z"); // '.' 后非数字
  expect_fail("2023-01-01T00:00:00ZZ"); // 双 Z
  expect_fail("2023-01-01T00:00:00Z "); // 尾 ASCII 空格 (ES5 强制 EOF)
  expect_fail("2023-01-01T00:00:00Z\u{00A0}"); // 尾 NBSP 同拒 (Z 后非 EOF)
  expect_fail("  2023-01-01T00:00:00Z"); // 前导空格使 ES5 首段失配, legacy 遇 T 拒绝
  expect_fail("2023-01-01T00:00:00Zfoo");
  expect_fail("2023-1-1T00:00:00Z"); // MM/DD 恒 2 位; legacy 遇 T 拒绝
  expect_fail("x2023-01-01"); // 垃圾词紧邻数字

  // 时区形态错误
  expect_fail("2023-01-01T00:00:00+24:00");
  expect_fail("2023-01-01T00:00:00+00:60");
  expect_fail("2023-01-01T00:00:00+2:30"); // hh 非 2 位
  expect_fail("2023-01-01T00:00:00+2400"); // hhmm 中 hh=24
  expect_fail("2023-01-01T00:00:00+240"); // 3 位非 hh/hhmm
  expect_fail("2023-01-01T00:00:00+");

  // -000000 带时间部分: ES5 拒后 legacy 遇 T 拒绝
  expect_fail("-000000-01-01T00:00:00Z");

  OK
}

/// legacy 合法差分 (Safari 兼容; 组件推演对齐 Day/Time/TimeZoneComposer)
#[test]
fn test_diff_legacy_valid() -> Void {
  // 命名月 + 年份紧邻: write 开头 fill 把 index 推到 3, named 分支走
  // !IsDay(comp[0]) → year=1970, day=fill 的 1 (V8 接受 "May 1970")
  expect_out("May 1970", [1970.0, 4.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN]);
  expect_out("May-1970", [1970.0, 4.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN]);

  // 月名三态: 前缀/全名/混合大小写; DMY 语义
  expect_out(
    "Dec 25 1995",
    [1995.0, 11.0, 25.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "december 25 1995",
    [1995.0, 11.0, 25.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "DEC 25 1995",
    [1995.0, 11.0, 25.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "Dec 25 13:30:00 PST 1995",
    [1995.0, 11.0, 25.0, 13.0, 30.0, 0.0, 0.0, -28800.0],
  );
  expect_out(
    "May 1, 1970",
    [1970.0, 4.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "june 15 2020 07:08:09",
    [2020.0, 5.0, 15.0, 7.0, 8.0, 9.0, 0.0, f64::NAN],
  );
  expect_out(
    "18 SePtEmBeR 2023",
    [2023.0, 8.0, 18.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "1995 Dec 25",
    [1995.0, 11.0, 25.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  ); // YMD 词序

  // 命名月 + 单数字: fill 后 DMY 分支 year 取 comp[1] (fill 的 1) → 2001 (KJS 兼容怪例)
  expect_out(
    "Dec 25 13:30:00 PST",
    [2001.0, 11.0, 25.0, 13.0, 30.0, 0.0, 0.0, -28800.0],
  );

  // RFC 2822 / Netscape: 垃圾星期词 + 逗号 + 短横线日 + 命名月 + 时区
  expect_out(
    "Wed, 18 Oct 2023 14:30:45 GMT",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 0.0, 0.0],
  );
  expect_out(
    "Sat, 01-Jan-2000 08:00:00 GMT",
    [2000.0, 0.0, 1.0, 8.0, 0.0, 0.0, 0.0, 0.0],
  );
  expect_out(
    "Thu, 01 Jan 1970 00:00:00 GMT",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  );
  expect_out(
    "1 Jan 1970 00:00:00 GMT",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  );
  // GMT+0200 后随括号注释 — 经典 JS toString 往返格式
  expect_out(
    "Tue Oct 18 2022 12:00:00 GMT+0200 (CEST)",
    [2022.0, 9.0, 18.0, 12.0, 0.0, 0.0, 0.0, 7200.0],
  );

  // MD(Y) 斜杠序 + AM/PM (12 AM→0, 12 PM→12)
  expect_out(
    "10/18/2023 02:30:45 PM",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 0.0, f64::NAN],
  );
  expect_out(
    "10/18/2023 12:00:00 AM",
    [2023.0, 9.0, 18.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "01/01/1970",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "12/25/1995 12:00:00",
    [1995.0, 11.0, 25.0, 12.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "1970/01/01",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  );

  // 两位年: 0..49 → 2000s; 50..99 → 1900s
  expect_out(
    "Oct 18 23 12:00:00",
    [2023.0, 9.0, 18.0, 12.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "Oct 18 49 12:00:00",
    [2049.0, 9.0, 18.0, 12.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "Oct 18 50 12:00:00",
    [1950.0, 9.0, 18.0, 12.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "Oct 18 99 12:00:00",
    [1999.0, 9.0, 18.0, 12.0, 0.0, 0.0, 0.0, f64::NAN],
  );

  // n+':' / n+'::' / n+'.'+ms / legacy 24:00
  expect_out(
    "2020 12::",
    [2020.0, 0.0, 1.0, 12.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "2020 12:34",
    [2020.0, 0.0, 1.0, 12.0, 34.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "1970-01-01 24:00",
    [1970.0, 0.0, 1.0, 24.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "Jan 1 00:00:00 1970",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  ); // 时间在前年份在后

  // 括号注释 (前缀/中缀/嵌套) 与垃圾词前缀 — 空格使垃圾词 peek 非数字, 不触发邻数字拒绝
  expect_out("(c) 1970", [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN]);
  expect_out(
    "2023-10-18 (China Standard Time) 12:00",
    [2023.0, 9.0, 18.0, 12.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "2023-10-18 ((nested (comment))) 12:00",
    [2023.0, 9.0, 18.0, 12.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out(
    "Wed(thu) 18 Oct 2023 14:30:45 GMT",
    [2023.0, 9.0, 18.0, 14.0, 30.0, 45.0, 0.0, 0.0],
  );
  expect_out(
    "foo bar baz 1970-01-01",
    [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN],
  );
  expect_out("x 1970", [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN]);
  expect_out("GMT 1970", [1970.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN]); // 数字前 GMT 未生效 (无 has_read_number)
  // '.' 被 skip 后 time 非 expecting → 落 day → [1970,5] 构成 YMD (1970-05-01, out 0-based 4)
  expect_out("1970.5", [1970.0, 4.0, 1.0, 0.0, 0.0, 0.0, 0.0, f64::NAN]);

  // ES5 时间部分解析后遇非 EOF → Invalid 终止 (dateparser-inl.h:344), 不落 legacy
  expect_fail("1995-12-25T13:30:00 PST");

  // 时区 ±hh:mm / ±hh / ±hhmm (legacy 符号路径)
  expect_out(
    "Jan 1 1970 10:00:00 -05:30",
    [1970.0, 0.0, 1.0, 10.0, 0.0, 0.0, 0.0, -19800.0],
  );
  expect_out(
    "Jan 1 1970 10:00:00 +0530",
    [1970.0, 0.0, 1.0, 10.0, 0.0, 0.0, 0.0, 19800.0],
  );
  expect_out(
    "Jan 1 1970 10:00:00 +05",
    [1970.0, 0.0, 1.0, 10.0, 0.0, 0.0, 0.0, 18000.0],
  );
  expect_out(
    "December 25, 1995 08:30:00 -0500",
    [1995.0, 11.0, 25.0, 8.0, 30.0, 0.0, 0.0, -18000.0],
  );

  // legacy 毫秒: 秒 + '.' + 1/2 位
  expect_out(
    "Jan 1 1970 10:00:00.5",
    [1970.0, 0.0, 1.0, 10.0, 0.0, 0.0, 500.0, f64::NAN],
  );
  expect_out(
    "Jan 1 1970 10:00:00.05",
    [1970.0, 0.0, 1.0, 10.0, 0.0, 0.0, 50.0, f64::NAN],
  );

  OK
}

/// legacy 非法差分 (数字后垃圾词/多符号/容量溢出)
#[test]
fn test_diff_legacy_invalid() -> Void {
  // 垃圾词规则: 垃圾词尾紧邻数字才拒 (dateparser-inl.h:134); 隔空格不拒
  expect_fail("foo1970-01-01");
  expect_fail("x2023-01-01");
  expect_fail("1970 garbage");
  expect_fail("1970-01-01 extra");
  expect_fail("Jan 1 1970 extra");

  // 数字已读后的符号/右括号
  expect_fail("1970-01-01 )");
  expect_fail("1970-01-01 + extra");
  expect_fail("1970-01-01 - x");

  // 容量溢出
  expect_fail("1 2 3 4"); // DayComposer 容量 3
  expect_fail("1/1/1/1/1");
  expect_fail("1970 Jan Jan 1 2 3 4");
  expect_fail("2020 1:2:3:4:5:6"); // TimeComposer 容量 4

  // 结构性拒绝
  expect_fail("13:30"); // 无日期纯时间 → day.write 拒绝
  expect_fail("10:00:00 AM"); // 同上
  expect_fail("12::"); // 同上
  expect_fail("u 1 1970");
  expect_fail("1970 u 1 2 3 4");
  expect_fail("0x10"); // 0 后词 x → 垃圾词邻数字
  expect_fail("2020-01-01 12:00:00 +12345678"); // 长度 8 非 hh(:mm)/hhmm → 拒
  expect_fail("2020-01-01 12:00:00 GMTX"); // 长词非月名 → 数字后垃圾
  expect_fail("2020-01-01 12:00:00 UTCX");

  OK
}

/// 端到端时间戳差分 (固定 UTC+8 mock; 权威值与 JS Date.UTC 对应)
#[test]
fn test_diff_end_to_end_timestamps() -> Void {
  let mut dc = DateCache::new(Utc8MockProvider);

  // ES5 date-only → UTC 语义 (与本地时区无关)
  assert_eq!(
    parse_date_time_string(&mut dc, "1995-12-25"),
    819_849_600_000.0
  );
  assert_eq!(parse_date_time_string(&mut dc, "1970-01-01"), 0.0);
  assert_eq!(
    parse_date_time_string(&mut dc, "2000-01-01"),
    946_684_800_000.0
  );

  // ES5 date-time 带 Z → UTC
  assert_eq!(
    parse_date_time_string(&mut dc, "1995-12-25T13:30:00Z"),
    819_898_200_000.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "2000-02-29T00:00:00Z"),
    951_782_400_000.0
  );

  // ES5 date-time 带显式偏移: UTC 时刻 − offset
  assert_eq!(
    parse_date_time_string(&mut dc, "1995-12-25T13:30:00+05:30"),
    819_878_400_000.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "1995-12-25T13:30:00-05:30"),
    819_918_000_000.0
  );

  // ES5 date-time 无时区 → 本地语义 (−8h)
  assert_eq!(
    parse_date_time_string(&mut dc, "1995-12-25T13:30:00"),
    819_869_400_000.0
  );

  // legacy 命名月无时区 → 本地语义
  assert_eq!(
    parse_date_time_string(&mut dc, "Dec 25 1995"),
    819_820_800_000.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "May 1, 1970"),
    10_339_200_000.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "May 1970"),
    10_339_200_000.0
  ); // 1970-05-01 本地
  assert_eq!(
    parse_date_time_string(&mut dc, "-000000-01-01"),
    978_307_200_000.0 - 28_800_000.0
  );

  // legacy 带时区名 (PST = −8h) / RFC 2822 / 时区回绕端到端
  assert_eq!(
    parse_date_time_string(&mut dc, "Dec 25 13:30:00 PST 1995"),
    819_927_000_000.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "Sat, 01-Jan-2000 08:00:00 GMT"),
    946_713_600_000.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "1970-01-01 24:00"),
    57_600_000.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "Wed, 18 Oct 2023 14:30:45 GMT"),
    1_697_639_445_000.0
  );

  // 回绕: 999999999:00 → hour=999999999, minute=0 → (999999999*3600) mod 2^32 = 817402352 ≤ SmiMax
  // UTC(2020-01-01 12:00) = 1577880000000; offset 取负 → ts = base + 817402352 * 1000
  assert_eq!(
    parse_date_time_string(&mut dc, "2020-01-01 12:00:00 -999999999:00"),
    1_577_880_000_000.0 + 817_402_352_000.0
  );

  // 毫秒缩放端到端
  assert_eq!(
    parse_date_time_string(&mut dc, "1970-01-01T00:00:00.5Z"),
    500.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "1970-01-01T00:00:00.05Z"),
    50.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "1970-01-01T00:00:00.005Z"),
    5.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "1970-01-01T00:00:00.0005Z"),
    0.0
  );

  // 负时刻 / TimeClip 上界 / 上界溢出
  assert_eq!(
    parse_date_time_string(&mut dc, "1969-12-31T23:59:59Z"),
    -1000.0
  );
  assert_eq!(
    parse_date_time_string(&mut dc, "+275760-09-13T00:00:00Z"),
    8_640_000_000_000_000.0
  );
  assert!(parse_date_time_string(&mut dc, "+275760-09-13T00:00:00.001Z").is_nan());
  assert!(parse_date_time_string(&mut dc, "+275761-01-01T00:00:00Z").is_nan());

  // 平年 2/29 滚动到 3/1 (UTC)
  assert_eq!(
    parse_date_time_string(&mut dc, "2023-02-29T00:00:00Z"),
    1_677_628_800_000.0
  );

  // 非法 → NaN
  assert!(parse_date_time_string(&mut dc, "").is_nan());
  assert!(parse_date_time_string(&mut dc, "invalid").is_nan());

  OK
}

/// DateCache 缓存窗口滑动语义: 跨 DST 段的扩展/收缩/合并/大跳/回退状态序列
#[test]
fn test_date_cache_window_slide_semantics() -> Void {
  // 阶跃 DST: 2010-03-14T10:00Z 进入, 2010-11-07T10:00Z 退出 (秒级边界)
  let t1_sec = (days_from_year_month(2010, 2) as i64 + 13) * 86400 + 10 * 3600;
  let t2_sec = (days_from_year_month(2010, 10) as i64 + 6) * 86400 + 10 * 3600;
  let mut dc = DateCache::new(StepDstProvider { t1_sec, t2_sec });

  // 状态 1: 首查建立 before 段 (miss → 单点段)
  let t = (t1_sec - 10 * 86400) * 1000;
  assert_eq!(dc.local_offset_in_ms(t, true), 0);

  // 状态 2: 窗口内逐日推进 → before 段经二分逐步扩展 (offset 恒 0)
  for i in 1..10_i64 {
    let probe = t + i * MS_PER_DAY;
    assert_eq!(
      dc.local_offset_in_ms(probe, true),
      0,
      "slide forward day {i}"
    );
  }

  // 状态 3: 反复命中乐观快速路径 (同一时刻 8 次结果稳定)
  for _ in 0..8 {
    assert_eq!(dc.local_offset_in_ms(t + 9 * MS_PER_DAY, true), 0);
  }

  // 状态 4: 逼近跃变点 — 边界毫秒级采样 (before 段推至 T1 前, after 段收缩)
  for off_ms in [-3_600_000i64, -60_000, -1000, -1] {
    let probe = t1_sec * 1000 + off_ms;
    assert_eq!(
      dc.local_offset_in_ms(probe, true),
      0,
      "just before T1 ({off_ms})"
    );
  }
  for off_ms in [0i64, 1, 1000, 3_600_000] {
    let probe = t1_sec * 1000 + off_ms;
    assert_eq!(
      dc.local_offset_in_ms(probe, true),
      3_600_000,
      "just after T1 (+{off_ms})"
    );
  }

  // 状态 5: DST 段内窗口继续滑动
  for i in 1..10_i64 {
    let probe = t1_sec * 1000 + i * MS_PER_DAY;
    assert_eq!(
      dc.local_offset_in_ms(probe, true),
      3_600_000,
      "in DST day {i}"
    );
  }

  // 状态 6: 跨退出点 T2 — 段收缩/交换; 边界采样
  for off_ms in [-3_600_000i64, -1000, -1] {
    let probe = t2_sec * 1000 + off_ms;
    assert_eq!(
      dc.local_offset_in_ms(probe, true),
      3_600_000,
      "just before T2 ({off_ms})"
    );
  }
  for off_ms in [0i64, 1, 1000] {
    let probe = t2_sec * 1000 + off_ms;
    assert_eq!(
      dc.local_offset_in_ms(probe, true),
      0,
      "just after T2 (+{off_ms})"
    );
  }

  // 状态 7: 大间隔前跳 (>19 天) → ExtendTheAfterSegment 新段 + swap 路径
  let far_future = (t2_sec + 400 * 86400) * 1000;
  assert_eq!(dc.local_offset_in_ms(far_future, true), 0);
  let far_future2 = (t2_sec + 800 * 86400) * 1000;
  assert_eq!(dc.local_offset_in_ms(far_future2, true), 0);

  // 状态 8: 大间隔回跳 → probe_cache before fallback (LRU 淘汰) + 回跳后前推
  let far_past = (t1_sec - 400 * 86400) * 1000;
  assert_eq!(dc.local_offset_in_ms(far_past, true), 0);
  assert_eq!(dc.local_offset_in_ms(far_past + MS_PER_DAY, true), 0);

  // 状态 9: 二分命中 swap 后回到 DST 段内部 — offset 记忆正确
  let in_dst_mid = (t1_sec + 90 * 86400) * 1000;
  assert_eq!(dc.local_offset_in_ms(in_dst_mid, true), 3_600_000);

  // 状态 10: to_local/to_utc 与无状态直查 oracle 全量对拍 (每个样本全新缓存实例)
  let samples = [
    t1_sec * 1000 - 3_600_000,
    t1_sec * 1000 - 1,
    t1_sec * 1000,
    t1_sec * 1000 + 1,
    t2_sec * 1000 - 1,
    t2_sec * 1000,
    t2_sec * 1000 + 1,
    far_past,
    far_future,
  ];
  for &s in &samples {
    let mut oracle = DateCache::new(StepDstProvider { t1_sec, t2_sec });
    let want_local = s + oracle.get_local_offset_from_os(s, true) as i64;
    let want_utc = s - oracle.get_local_offset_from_os(s, false) as i64;
    let mut a = DateCache::new(StepDstProvider { t1_sec, t2_sec });
    assert_eq!(a.to_local(s), want_local, "to_local at {s}");
    let mut b = DateCache::new(StepDstProvider { t1_sec, t2_sec });
    assert_eq!(b.to_utc(s), want_utc, "to_utc at {s}");
  }

  OK
}

/// 关键字表冲突面: 前缀 0 填充 / 长度规则 / 长词放行 / 28 条前缀唯一性
/// (对拍原版 KeywordTable::Lookup, dateparser.cc:153-168)
#[test]
fn test_diff_keyword_table_conflict_surface() -> Void {
  // 28 条与原版同序同值 (结构对拍)
  assert_eq!(KEYWORD_TABLE.len(), 28);
  assert_eq!(KEYWORD_TABLE[0].prefix, *b"jan");
  assert_eq!(KEYWORD_TABLE[11].prefix, *b"dec");
  assert_eq!(KEYWORD_TABLE[12].kw_type, KeywordType::AmPm);
  assert_eq!(KEYWORD_TABLE[13].value, 12); // pm
  assert_eq!(KEYWORD_TABLE[26].kw_type, KeywordType::TimeSeparator);
  assert_eq!(KEYWORD_TABLE[27].kw_type, KeywordType::Invalid);

  let lk = |word: &str| {
    let mut pre = [0u32; 3];
    for (i, c) in word.chars().take(3).enumerate() {
      pre[i] = c as u32 | 0x20;
    }
    lookup_keyword(&pre, word.chars().count())
  };

  // 单字符 0 填充: 仅表项本身单字符 (z, t) 命中; "u" 的填充 0 与 'ut' 第二位 't' 不等
  assert_eq!(lk("u"), None);
  assert_eq!(lk("ut"), Some(14));
  assert_eq!(lk("utc"), Some(15));
  assert_eq!(lk("utca"), None); // 长词仅 MONTH_NAME 放行
  assert_eq!(lk("z"), Some(16));
  assert_eq!(lk("zz"), None);
  assert_eq!(lk("t"), Some(26));
  assert_eq!(lk("s"), None); // 'sep' 三字符, 单 s 不匹配
  assert_eq!(lk("p"), None);
  assert_eq!(lk("pm"), Some(13));
  assert_eq!(lk("a"), None);
  assert_eq!(lk("am"), Some(12));

  // 月名长词放行 (len > 3 且 MONTH_NAME)
  assert_eq!(lk("jun"), Some(5));
  assert_eq!(lk("june"), Some(5));
  assert_eq!(lk("junebug"), Some(5));
  assert_eq!(lk("ju"), None); // 0 填充后第三位 0 ≠ 'n'
  assert_eq!(lk("sept"), Some(8));
  assert_eq!(lk("december"), Some(11));
  assert_eq!(lk("mayor"), Some(4));
  assert_eq!(lk("mayhem"), Some(4));

  // 非月名时区长词拒绝
  assert_eq!(lk("gmt"), Some(17));
  assert_eq!(lk("gmtx"), None);
  assert_eq!(lk("est"), Some(21));
  assert_eq!(lk("estx"), None);

  // 非 ASCII (≥128) 短路: 与全部 ASCII 表项不等价 → None
  assert_eq!(lk("j\u{00e4}n"), None);
  assert_eq!(lk("\u{00e4}bc"), None);

  // 前 27 条 (有效关键字) 两两前缀唯一 — 线性扫描无冲突匹配面
  for (i, a) in KEYWORD_TABLE.iter().enumerate().take(27) {
    for b in &KEYWORD_TABLE[i + 1..27] {
      assert_ne!(a.prefix, b.prefix, "prefix conflict at {i}");
    }
  }

  OK
}
