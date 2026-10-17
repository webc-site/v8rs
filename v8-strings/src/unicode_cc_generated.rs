// 由 tool/src/strings/unicode.cc.js 从 v8/src/strings/unicode.{cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// Unicode 表驱动与编解码骨架与实现

/// 压缩表编码常量 (原版 unicode.cc 顶部)
pub mod unicode_table_coding {
  /// 起始项位 (原版 kStartBit = 1 << 30)
  pub const START_BIT: i32 = 1 << 30;
  /// 分块位宽 (原版 kChunkBits = 1 << 13)
  pub const CHUNK_BITS: i32 = 1 << 13;
  /// 哨兵码点 (原版 kSentinel = uchar(-1))
  pub const SENTINEL: u32 = u32::MAX;

  /// 项内取值 (原版 GetEntry: entry & (kStartBit-1))
  pub const ENTRY_VALUE_MASK: i32 = (1 << 30) - 1;

  /// 表驱动谓词查找签名 (原版 LookupPredicate; 二级表: 块表+块内偏移)
  #[must_use]
  pub fn lookup_predicate(table: &[i32], size: u16, chr: u32) -> bool {
    let size = size as usize;
    if size == 0 || table.len() < size {
      return false;
    }
    let value = (chr & ((CHUNK_BITS as u32) - 1)) as i32;
    let mut low = 0usize;
    let mut high = size - 1;
    while high != low {
      let mid = low + ((high - low) >> 1);
      let current_value = table[mid] & ENTRY_VALUE_MASK;
      if current_value <= value && (mid + 1 == size || (table[mid + 1] & ENTRY_VALUE_MASK) > value)
      {
        low = mid;
        break;
      } else if current_value < value {
        low = mid + 1;
      } else {
        if mid == 0 {
          break;
        }
        high = mid - 1;
      }
    }
    let field = table[low];
    let entry = field & ENTRY_VALUE_MASK;
    let is_start = (field & START_BIT) != 0;
    (entry == value) || (entry < value && is_start)
  }

  /// 表驱动映射签名 (原版 LookupMapping)
  #[must_use]
  pub fn lookup_mapping(table: &[i32], size: u16, chr: u32) -> u32 {
    let size = size as usize;
    if size == 0 || table.len() < 2 * size {
      return chr;
    }
    let key = (chr & ((CHUNK_BITS as u32) - 1)) as i32;
    let chunk_start = (chr as i32) - key;
    let mut low = 0usize;
    let mut high = size - 1;
    while high != low {
      let mid = low + ((high - low) >> 1);
      let current_value = table[2 * mid] & ENTRY_VALUE_MASK;
      if current_value <= key
        && (mid + 1 == size || (table[2 * (mid + 1)] & ENTRY_VALUE_MASK) > key)
      {
        low = mid;
        break;
      } else if current_value < key {
        low = mid + 1;
      } else {
        if mid == 0 {
          break;
        }
        high = mid - 1;
      }
    }
    let field = table[2 * low];
    let entry = field & ENTRY_VALUE_MASK;
    let is_start = (field & START_BIT) != 0;
    let found = (entry == key) || (entry < key && is_start);
    if found {
      let value = table[2 * low + 1];
      if value == 0 {
        chr
      } else if (value & 3) == 0 {
        (entry + chunk_start + (value >> 2)) as u32
      } else {
        chr
      }
    } else {
      chr
    }
  }

  /// 项是否为块起始 (原版 IsStart: entry & kStartBit)
  #[inline]
  #[must_use]
  pub fn is_start(entry: i32) -> bool {
    entry & START_BIT != 0
  }
}

/// unibrow 谓词/转换族清单 (原版 unicode.cc; INTL 与非 INTL 双实现)
pub mod unicode_families {
  /// 谓词 (Is), 5 项 — INTL 走 ICU, 非 INTL 走内嵌表
  pub const PREDICATES: [&str; 5] = [
    "Uppercase",
    "Letter",
    "ID_Start",
    "ID_Continue",
    "WhiteSpace",
  ];
  /// 转换 (Convert), 5 项
  pub const CONVERTERS: [&str; 5] = [
    "ToLowercase",
    "ToUppercase",
    "Ecma262Canonicalize",
    "Ecma262UnCanonicalize",
    "CanonicalizationRange",
  ];
}

/// 编解码函数骨架与实现 (原版 unicode.cc Utf8/Utf16/Wtf8 段)
pub mod unicode_codec_skel {
  use crate::{
    unicode::{
      utf8,
      utf8::{
        BAD_CHAR, BUFFER_EMPTY, INCOMPLETE, value_of, value_of_incremental,
        write_leading_ascii_one_byte,
      },
      utf16::has_unpaired_surrogate,
      wtf8,
      wtf8::scan_for_surrogates,
    },
    utf8_dfa::strict::State,
  };

  /// 增量解码终态 (原版 State::kAccept; 拒绝为 kReject)
  pub const UTF8_ACCEPT: u8 = 0;

  /// 多字节序列求值 (原版 Utf8::CalculateValue; kBadChar 失败)
  #[must_use]
  pub fn utf8_calculate_value(str: &[u8], max_length: usize) -> u32 {
    let len = str.len().min(max_length);
    let mut cursor = 0usize;
    value_of(&str[..len], &mut cursor)
  }

  /// 增量解码单步 (原版 Utf8::ValueOfIncremental; ASCII 快路径 + 状态机)
  pub fn utf8_value_of_incremental(cursor: &mut &[u8], state: &mut u8, buffer: &mut u32) -> u32 {
    if cursor.is_empty() {
      return INCOMPLETE;
    }
    let mut dfa_state = match *state {
      0 | 12 => State::Accept,
      24 => State::TwoByte,
      36 => State::ThreeByte,
      48 => State::ThreeByteLowMid,
      60 => State::FourByte,
      72 => State::FourByteLow,
      84 => State::ThreeByteHigh,
      96 => State::FourByteMidHigh,
      _ => State::Reject,
    };
    let mut pos = 0usize;
    let res = value_of_incremental(cursor, &mut pos, &mut dfa_state, buffer);
    *cursor = &cursor[pos..];
    *state = if dfa_state == State::Accept {
      0
    } else {
      dfa_state as u8
    };
    res
  }

  /// 流尾收尾 (原版 Utf8::ValueOfIncrementalFinish)
  #[inline]
  #[must_use]
  pub fn utf8_value_of_incremental_finish(state: &mut u8) -> u32 {
    if *state == 0 || *state == 12 {
      BUFFER_EMPTY
    } else {
      *state = 0;
      BAD_CHAR
    }
  }

  /// UTF-8 校验 (原版 simdutf::validate_utf8)
  #[inline]
  #[must_use]
  pub fn utf8_validate_encoding(bytes: &[u8]) -> bool {
    utf8::validate_encoding(bytes)
  }

  /// 未配对代理项检测 (原版 Utf16::HasUnpairedSurrogate → simdutf validate_utf16)
  #[inline]
  #[must_use]
  pub fn utf16_has_unpaired_surrogate(units: &[u16]) -> bool {
    has_unpaired_surrogate(units)
  }

  /// 未配对代理项替换 (原版 Utf16::ReplaceUnpairedSurrogates → U+FFFD)
  /// 纯原地单次扫描, 零堆内存分配
  pub fn utf16_replace_unpaired_surrogates(src: &mut [u16]) {
    let len = src.len();
    let mut i = 0;
    while i < len {
      let c = src[i];
      if (0xd800..0xdc00).contains(&c) {
        if i + 1 < len && (0xdc00..0xe000).contains(&src[i + 1]) {
          i += 2;
          continue;
        }
        src[i] = 0xfffd;
      } else if (0xdc00..0xe000).contains(&c) {
        src[i] = 0xfffd;
      }
      i += 1;
    }
  }

  /// WTF-8 校验 (原版 Wtf8::ValidateEncoding)
  #[inline]
  #[must_use]
  pub fn wtf8_validate_encoding(bytes: &[u8]) -> bool {
    wtf8::validate_encoding(bytes)
  }

  /// WTF-8 代理扫描 (原版 Wtf8::ScanForSurrogates)
  #[inline]
  pub fn wtf8_scan_for_surrogates(wtf8: &[u8]) -> Vec<usize> {
    let mut offsets = Vec::new();
    scan_for_surrogates(wtf8, &mut offsets);
    offsets
  }

  /// ASCII 前导批量写 (原版 Utf8::WriteLeadingAscii, uint8/uint16 双特化)
  #[inline]
  pub fn utf8_write_leading_ascii(src: &[u8], dest: &mut [u8]) -> usize {
    write_leading_ascii_one_byte(src, dest)
  }
}

/// 非 INTL 单项缓存协议 (原版 unicode-inl.h #ifndef V8_INTL_SUPPORT 段)
pub mod unibrow_cache_skel {
  use super::unibrow_cache_const::*;
  use crate::unicode::letter::is;

  /// 谓词缓存查询 (原版 Predicate::get; 命中即取, 未命中算后写回)
  #[inline]
  pub fn predicate_get(entries: &mut [CacheEntry], code_point: u32) -> bool {
    if entries.is_empty() {
      return is(code_point);
    }
    let mask = entries.len() - 1;
    let idx = (code_point as usize) & mask;
    if entries[idx].code_point == code_point {
      return entries[idx].value != 0;
    }
    let res = is(code_point);
    entries[idx] = CacheEntry {
      code_point,
      value: if res { 1 } else { 0 },
    };
    res
  }

  /// 映射缓存查询 (原版 Mapping::get; offset=0 表示不可映射)
  /// 栈上定长缓冲, 零堆内存分配
  pub fn mapping_get(entries: &mut [CacheEntry], c: u32, _n: u32, result: &mut [u32]) -> i32 {
    if entries.is_empty() {
      return 0;
    }
    let mask = entries.len() - 1;
    let idx = (c as usize) & mask;
    if entries[idx].code_point == c {
      if entries[idx].value == 0 {
        0
      } else {
        if !result.is_empty() {
          result[0] = (c as i32 + entries[idx].value) as u32;
        }
        1
      }
    } else if let Some(ch) = char::from_u32(c) {
      let mut mapped_chars = [0u32; 3];
      let mut count = 0usize;
      for lower_ch in ch.to_lowercase() {
        if count < 3 {
          mapped_chars[count] = lower_ch as u32;
          count += 1;
        }
      }
      if count == 1 {
        let target = mapped_chars[0];
        let offset = (target as i32).wrapping_sub(c as i32);
        entries[idx] = CacheEntry {
          code_point: c,
          value: offset,
        };
        if !result.is_empty() {
          result[0] = target;
        }
        1
      } else if count > 1 {
        entries[idx] = CacheEntry {
          code_point: c,
          value: 0,
        };
        let len = count.min(result.len());
        result[..len].copy_from_slice(&mapped_chars[..len]);
        len as i32
      } else {
        entries[idx] = CacheEntry {
          code_point: c,
          value: 0,
        };
        0
      }
    } else {
      entries[idx] = CacheEntry {
        code_point: c,
        value: 0,
      };
      0
    }
  }
}

/// 缓存布局常量 (原版 unicode.h 模板: kMask = kSize-1, 直接取模寻址)
pub mod unibrow_cache_const {
  /// 缓存条目 (code_point + 值/偏移)
  #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
  pub struct CacheEntry {
    pub code_point: u32,
    pub value: i32,
  }
}
