#!/usr/bin/env bun
// tool/src/strings/string-case.h.js — v8/src/strings/{string-case.h, string-case.cc} → v8-strings/src/string_case.rs
// SWAR 常量 1:1 (usize word); 转换语义就地补写
import { banner, readV8, writeSrc } from "../../lib.js";

const caseH = readV8("src/strings/string-case.h");
const caseCc = readV8("src/strings/string-case.cc");

if (!caseH.includes("FastAsciiConvert") || !caseCc.includes("FastAsciiConvert")) {
  throw new Error("string-case.h / string-case.cc 结构漂移");
}

const body = `${banner("src/strings/string-case.h.js", "src/strings/string-case.h + string-case.cc", "SWAR 常量 1:1 (usize word); 转换语义就地补写")}
//! ASCII 大小写快速转换 (word 位并行)。

/// 字节内每位遍布 (kOneInEveryByte)。
const ONE_IN_EVERY_BYTE: usize = usize::MAX / 0xFF;
/// 每字节最高位遍布 (kAsciiMask)。
const ASCII_MASK: usize = ONE_IN_EVERY_BYTE << 7;

/// 大小写方向 (unibrow::ToLowercase / ToUppercase 的 kIsToLower)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseMapping {
  ToLower,
  ToUpper,
}

impl CaseMapping {
  /// 是否转小写 (kIsToLower)。
  #[inline]
  pub const fn is_to_lower(self) -> bool {
    matches!(self, CaseMapping::ToLower)
  }

  /// 转换区间的下界 - 1 (lo)。
  #[inline]
  pub const fn lo(self) -> u8 {
    if self.is_to_lower() { b'A' - 1 } else { b'a' - 1 }
  }

  /// 转换区间的上界 + 1 (hi)。
  #[inline]
  pub const fn hi(self) -> u8 {
    if self.is_to_lower() { b'Z' + 1 } else { b'z' + 1 }
  }
}

/// word 中字节严格落在 (low, high) 开区间者, 置其最高位 (AsciiRangeMask)。
/// 要求输入与边界均 ASCII; 仅内联使用 (边界静态已知)。
#[inline]
const fn ascii_range_mask(low: u8, high: u8, w: usize) -> usize {
  let tmp1 = ONE_IN_EVERY_BYTE
    .wrapping_mul(0x7F + high as usize)
    .wrapping_sub(w);
  let tmp2 = w.wrapping_add(ONE_IN_EVERY_BYTE * (0x7F - low as usize));
  tmp1 & tmp2 & (ONE_IN_EVERY_BYTE * 0x80)
}

const WORD_BYTES: usize = core::mem::size_of::<usize>();

/// 读取 8 字节 word (LE); 尾部不足 8 字节由调用方保证不进入 word 路径。
#[inline]
fn load_word(src: &[u8], i: usize) -> usize {
  usize::from_le_bytes(src[i..i + WORD_BYTES].try_into().unwrap_or_default())
}

/// 计算 src 中已是目标大小写 (且均为 ASCII) 的前缀长度 (FastAsciiCasePrefixLength)。
/// 遇非 ASCII 或需转换的字符即停; word 命中后回退到 word 内精确定位
/// (原版返回 word 起点的保守下界, 调用方逐字符兜底; 精确值与其语义兼容)。
#[must_use]
pub fn fast_ascii_case_prefix_length(case: CaseMapping, src: &[u8]) -> usize {
  let (lo, hi) = (case.lo(), case.hi());
  let mut i = 0;
  while i + WORD_BYTES <= src.len() {
    let w = load_word(src, i);
    if w & ASCII_MASK != 0 || ascii_range_mask(lo, hi, w) != 0 {
      break;
    }
    i += WORD_BYTES;
  }
  while i < src.len() {
    let c = src[i];
    if c & 0x80 != 0 || (lo < c && c < hi) {
      return i;
    }
    i += 1;
  }
  src.len()
}

/// ASCII 大小写转换 (FastAsciiConvert): 遇非 ASCII 提前停, 返回已处理前缀长度。
/// 三阶段 word SWAR: 直拷 → 原地翻转 → 标量尾。
#[must_use]
pub fn fast_ascii_convert(case: CaseMapping, dst: &mut [u8], src: &[u8]) -> usize {
  let (lo, hi) = (case.lo(), case.hi());
  let limit = src.len().min(dst.len());
  let mut i = 0;
  // 阶段 1: 无需转换的 ASCII word 直接拷贝
  while i + WORD_BYTES <= limit {
    let w = load_word(src, i);
    if w & ASCII_MASK != 0 {
      return i;
    }
    if ascii_range_mask(lo, hi, w) != 0 {
      break;
    }
    dst[i..i + WORD_BYTES].copy_from_slice(&src[i..i + WORD_BYTES]);
    i += WORD_BYTES;
  }
  // 阶段 2: 逐 word 翻转 (大小写距离恰 1<<5, mask 高位对准后右移 2)
  while i + WORD_BYTES <= limit {
    let w = load_word(src, i);
    if w & ASCII_MASK != 0 {
      return i;
    }
    let m = ascii_range_mask(lo, hi, w);
    dst[i..i + WORD_BYTES].copy_from_slice(&(w ^ (m >> 2)).to_le_bytes());
    i += WORD_BYTES;
  }
  // 阶段 3: 标量尾
  while i < limit {
    let mut c = src[i];
    if c & 0x80 != 0 {
      return i;
    }
    if lo < c && c < hi {
      c ^= 1 << 5;
    }
    dst[i] = c;
    i += 1;
  }
  limit
}
`;

writeSrc("v8-strings", "string_case.rs", body);
console.log("written: string_case.rs");
