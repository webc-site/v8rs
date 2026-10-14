// 由 tool/src/date/dateparser.h.js 从 v8/src/date/dateparser.h 生成; 修订就地改, 复跑覆盖

pub const YEAR: usize = 0;
pub const MONTH: usize = 1;
pub const DAY: usize = 2;
pub const HOUR: usize = 3;
pub const MINUTE: usize = 4;
pub const SECOND: usize = 5;
pub const MILLISECOND: usize = 6;
pub const UTC_OFFSET: usize = 7;
pub const OUTPUT_SIZE: usize = 8;

pub const K_NONE: i32 = i32::MAX;
pub const MAX_SIGNIFICANT_DIGITS: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordType {
  Invalid = 0,
  MonthName = 1,
  TimeZoneName = 2,
  TimeSeparator = 3,
  AmPm = 4,
}

#[derive(Debug, Clone, Copy)]
pub struct KeywordEntry {
  pub prefix: [u8; 3],
  pub kw_type: KeywordType,
  pub value: i32,
}

pub const KEYWORD_TABLE: [KeywordEntry; 28] = [
  KeywordEntry {
    prefix: *b"jan",
    kw_type: KeywordType::MonthName,
    value: 1,
  },
  KeywordEntry {
    prefix: *b"feb",
    kw_type: KeywordType::MonthName,
    value: 2,
  },
  KeywordEntry {
    prefix: *b"mar",
    kw_type: KeywordType::MonthName,
    value: 3,
  },
  KeywordEntry {
    prefix: *b"apr",
    kw_type: KeywordType::MonthName,
    value: 4,
  },
  KeywordEntry {
    prefix: *b"may",
    kw_type: KeywordType::MonthName,
    value: 5,
  },
  KeywordEntry {
    prefix: *b"jun",
    kw_type: KeywordType::MonthName,
    value: 6,
  },
  KeywordEntry {
    prefix: *b"jul",
    kw_type: KeywordType::MonthName,
    value: 7,
  },
  KeywordEntry {
    prefix: *b"aug",
    kw_type: KeywordType::MonthName,
    value: 8,
  },
  KeywordEntry {
    prefix: *b"sep",
    kw_type: KeywordType::MonthName,
    value: 9,
  },
  KeywordEntry {
    prefix: *b"oct",
    kw_type: KeywordType::MonthName,
    value: 10,
  },
  KeywordEntry {
    prefix: *b"nov",
    kw_type: KeywordType::MonthName,
    value: 11,
  },
  KeywordEntry {
    prefix: *b"dec",
    kw_type: KeywordType::MonthName,
    value: 12,
  },
  KeywordEntry {
    prefix: *b"am\0",
    kw_type: KeywordType::AmPm,
    value: 0,
  },
  KeywordEntry {
    prefix: *b"pm\0",
    kw_type: KeywordType::AmPm,
    value: 12,
  },
  KeywordEntry {
    prefix: *b"ut\0",
    kw_type: KeywordType::TimeZoneName,
    value: 0,
  },
  KeywordEntry {
    prefix: *b"utc",
    kw_type: KeywordType::TimeZoneName,
    value: 0,
  },
  KeywordEntry {
    prefix: *b"z\0\0",
    kw_type: KeywordType::TimeZoneName,
    value: 0,
  },
  KeywordEntry {
    prefix: *b"gmt",
    kw_type: KeywordType::TimeZoneName,
    value: 0,
  },
  KeywordEntry {
    prefix: *b"cdt",
    kw_type: KeywordType::TimeZoneName,
    value: -5,
  },
  KeywordEntry {
    prefix: *b"cst",
    kw_type: KeywordType::TimeZoneName,
    value: -6,
  },
  KeywordEntry {
    prefix: *b"edt",
    kw_type: KeywordType::TimeZoneName,
    value: -4,
  },
  KeywordEntry {
    prefix: *b"est",
    kw_type: KeywordType::TimeZoneName,
    value: -5,
  },
  KeywordEntry {
    prefix: *b"mdt",
    kw_type: KeywordType::TimeZoneName,
    value: -6,
  },
  KeywordEntry {
    prefix: *b"mst",
    kw_type: KeywordType::TimeZoneName,
    value: -7,
  },
  KeywordEntry {
    prefix: *b"pdt",
    kw_type: KeywordType::TimeZoneName,
    value: -7,
  },
  KeywordEntry {
    prefix: *b"pst",
    kw_type: KeywordType::TimeZoneName,
    value: -8,
  },
  KeywordEntry {
    prefix: *b"t\0\0",
    kw_type: KeywordType::TimeSeparator,
    value: 0,
  },
  KeywordEntry {
    prefix: *b"\0\0\0",
    kw_type: KeywordType::Invalid,
    value: 0,
  },
];

pub const PREFIX_LENGTH: usize = 3;

/// KeywordTable::Lookup
pub fn lookup_keyword(pre: &[u32; 3], len: usize) -> Option<usize> {
  if pre[0] >= 128 || pre[1] >= 128 || pre[2] >= 128 {
    return None;
  }
  let p = [pre[0] as u8, pre[1] as u8, pre[2] as u8];
  for (i, entry) in KEYWORD_TABLE.iter().enumerate() {
    if entry.kw_type == KeywordType::Invalid {
      break;
    }
    if entry.prefix == p && (len <= PREFIX_LENGTH || entry.kw_type == KeywordType::MonthName) {
      return Some(i);
    }
  }
  None
}
