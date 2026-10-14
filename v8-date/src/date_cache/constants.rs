// 由 tool/src/date/date.h.js 从 v8/src/date/date.h 生成; 修订就地改, 复跑覆盖

pub const MS_PER_MIN: i32 = 60 * 1000;
pub const SEC_PER_DAY: i32 = 24 * 60 * 60;
pub const MS_PER_DAY: i64 = SEC_PER_DAY as i64 * 1000;
pub const MS_PER_MONTH: i64 = MS_PER_DAY * 30;
pub const MAX_EPOCH_TIME_IN_SEC: i32 = i32::MAX;
pub const MAX_EPOCH_TIME_IN_MS: i64 = (i32::MAX as i64) * 1000;
pub const MAX_TIME_IN_MS: i64 = 864_000_000_i64 * 10_000_000_i64;
pub const MAX_TIME_BEFORE_UTC_IN_MS: i64 = MAX_TIME_IN_MS + MS_PER_MONTH;
pub const INVALID_LOCAL_OFFSET_IN_MS: i32 = i32::MAX;
pub const DEFAULT_TIME_ZONE_OFFSET_DELTA_IN_MS: i32 = 19 * SEC_PER_DAY * 1000;
pub const CACHE_SIZE: usize = 32;
pub const DAYS_IN4_YEARS: i32 = 4 * 365 + 1;
pub const DAYS_IN100_YEARS: i32 = 25 * DAYS_IN4_YEARS - 1;
pub const DAYS_IN400_YEARS: i32 = 4 * DAYS_IN100_YEARS + 1;
pub const DAYS1970TO2000: i32 = 30 * 365 + 7;
pub const DAYS_OFFSET: i32 = 1000 * DAYS_IN400_YEARS + 5 * DAYS_IN400_YEARS - DAYS1970TO2000;
pub const YEARS_OFFSET: i32 = 400000;
pub const YEAR_DELTA: i32 = 399999;
pub const BASE_DAY: i32 = 365 * (1970 + YEAR_DELTA) + (1970 + YEAR_DELTA) / 4
  - (1970 + YEAR_DELTA) / 100
  + (1970 + YEAR_DELTA) / 400;
pub const MS_PER_HOUR: i32 = 3600 * 1000;
pub const MIN_YEAR: f64 = -1_000_000.0;
pub const MAX_YEAR: f64 = -MIN_YEAR;
pub const MIN_MONTH: f64 = -10_000_000.0;
pub const MAX_MONTH: f64 = -MIN_MONTH;
pub const MS_PER_DAY_F64: f64 = 86_400_000.0;
pub const MS_PER_SECOND: f64 = 1_000.0;
pub const MS_PER_MINUTE: f64 = 60_000.0;
pub const MS_PER_HOUR_F64: f64 = 3_600_000.0;

pub const DAYS_IN_MONTHS: [i8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub const DAY_FROM_MONTH: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

pub const DAY_FROM_MONTH_LEAP: [i32; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

pub const SHORT_WEEK_DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
pub const SHORT_MONTHS: [&str; 12] = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToDateStringMode {
  LocalDate,
  LocalTime,
  LocalDateAndTime,
  UTCDateAndTime,
  ISODateAndTime,
}
