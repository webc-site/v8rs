#!/usr/bin/env bun
// tool/src/strings/unicode-decoder.h.js — v8/src/strings/{unicode-decoder.h, unicode-decoder.cc} → v8-strings/src/unicode_decoder.rs
// 规则部分: NonAsciiStart 标量重写 + 三 Decoder 1:1; 状态机驱动与重试语义 1:1
import { banner, readV8, writeSrc } from "../../lib.js";

const decH = readV8("src/strings/unicode-decoder.h");
const decCc = readV8("src/strings/unicode-decoder.cc");

if (!decH.includes("class Utf8DecoderBase") || !decCc.includes("NonAsciiStart")) {
  throw new Error("unicode-decoder.h / unicode-decoder.cc 结构漂移");
}

const body = `${banner("src/strings/unicode-decoder.h.js", "src/strings/unicode-decoder.h + unicode-decoder.cc", "NonAsciiStart 标量重写 + 三 Decoder 1:1")}
//! UTF-8 → UTF-16 预扫描解码器: 先测编码类别与 utf16 长度, 再一次性解码。

use crate::unicode::{latin1, utf16, utf8};
use crate::utf8_dfa::{generalized, strict};

/// 返回第一个非单字节字符位置 (≥ length 表示全 ASCII)。
/// 原版按字对齐 SWAR; 此处 usize 位并行 (kOneInEveryByte 技巧)。
/// word 命中时返回 word 起始位置 (对齐字包含首个非单字节字符), 与原版一致。
#[must_use]
pub fn non_ascii_start(chars: &[u8]) -> u32 {
  const ONE_IN_EVERY_BYTE: usize = usize::MAX / 0xFF;
  const NON_ONE_BYTE_MASK: usize = ONE_IN_EVERY_BYTE * 0x80;
  let chunk_size = core::mem::size_of::<usize>();
  let chunks = chars.chunks_exact(chunk_size);
  let rem = chunks.remainder();
  for (k, w) in chunks.enumerate() {
    // chunks_exact 保证恰 8 字节, try_into 必然成功
    let word = usize::from_le_bytes(w.try_into().unwrap_or_default());
    if word & NON_ONE_BYTE_MASK != 0 {
      return (k * chunk_size) as u32;
    }
  }
  rem.iter().position(|&b| u32::from(b) > utf8::MAX_ONE_BYTE_CHAR).map_or(chars.len() as u32, |p| (chars.len() - rem.len() + p) as u32)
}

/// 编码类别。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Encoding {
  Ascii,
  Latin1,
  Utf16,
  Invalid,
}

/// 解码器特征差异 (DecoderTraits)。
pub trait DecoderTraits {
  fn is_invalid_surrogate_pair(lead: u32, trail: u32) -> bool;
  const ALLOW_INCOMPLETE: bool;
  type State: Copy + PartialEq;
  fn decode(byte: u8, state: &mut Self::State, buffer: &mut u32);
  const ACCEPT: Self::State;
  fn is_reject(state: &Self::State) -> bool;
}

pub struct Utf8Traits;
impl DecoderTraits for Utf8Traits {
  #[inline]
  fn is_invalid_surrogate_pair(_lead: u32, _trail: u32) -> bool {
    false // DFA 只产标量值, 全部合法
  }
  const ALLOW_INCOMPLETE: bool = true;
  type State = strict::State;
  #[inline]
  fn decode(byte: u8, state: &mut Self::State, buffer: &mut u32) {
    strict::decode(byte, state, buffer);
  }
  const ACCEPT: Self::State = strict::State::Accept;
  #[inline]
  fn is_reject(state: &Self::State) -> bool {
    *state == strict::State::Reject
  }
}

pub struct Wtf8Traits;
impl DecoderTraits for Wtf8Traits {
  #[inline]
  fn is_invalid_surrogate_pair(lead: u32, trail: u32) -> bool {
    utf16::is_lead_surrogate(lead as i32) && utf16::is_trail_surrogate(trail as i32)
  }
  const ALLOW_INCOMPLETE: bool = false;
  type State = generalized::State;
  #[inline]
  fn decode(byte: u8, state: &mut Self::State, buffer: &mut u32) {
    generalized::decode(byte, state, buffer);
  }
  const ACCEPT: Self::State = generalized::State::Accept;
  #[inline]
  fn is_reject(state: &Self::State) -> bool {
    *state == generalized::State::Reject
  }
}

pub struct StrictUtf8Traits;
impl DecoderTraits for StrictUtf8Traits {
  #[inline]
  fn is_invalid_surrogate_pair(_lead: u32, _trail: u32) -> bool {
    false
  }
  const ALLOW_INCOMPLETE: bool = false;
  type State = strict::State;
  #[inline]
  fn decode(byte: u8, state: &mut Self::State, buffer: &mut u32) {
    strict::decode(byte, state, buffer);
  }
  const ACCEPT: Self::State = strict::State::Accept;
  #[inline]
  fn is_reject(state: &Self::State) -> bool {
    *state == strict::State::Reject
  }
}

/// Utf8DecoderBase: 扫描给出 encoding / non_ascii_start / utf16_length。
#[derive(Debug, Clone, Copy)]
pub struct Utf8DecoderBase<T: DecoderTraits> {
  encoding: Encoding,
  non_ascii_start: i32,
  utf16_length: i32,
  _marker: std::marker::PhantomData<fn() -> T>,
}

impl<T: DecoderTraits> Utf8DecoderBase<T> {
  pub fn new(data: &[u8]) -> Self {
    let nas = non_ascii_start(data);
    let mut r = Self {
      encoding: Encoding::Ascii,
      non_ascii_start: nas as i32,
      utf16_length: nas as i32,
      _marker: std::marker::PhantomData,
    };
    if nas as usize == data.len() {
      return r;
    }

    let mut is_one_byte = true;
    let mut state = T::ACCEPT;
    let mut current = 0u32;
    let mut previous = 0u32;
    let mut cursor = nas as usize;

    while cursor < data.len() {
      if u32::from(data[cursor]) <= utf8::MAX_ONE_BYTE_CHAR && state == T::ACCEPT {
        previous = u32::from(data[cursor]);
        r.utf16_length += 1;
        cursor += 1;
        continue;
      }
      let previous_state = state;
      T::decode(data[cursor], &mut state, &mut current);
      if T::is_reject(&state) {
        if T::ALLOW_INCOMPLETE {
          state = T::ACCEPT;
          debug_assert!(utf8::BAD_CHAR > u32::from(latin1::MAX_CHAR));
          is_one_byte = false;
          r.utf16_length += 1;
          previous = utf8::BAD_CHAR;
          current = 0;
          // 序列中途失败: 该字节按新序列重试
          if previous_state != T::ACCEPT {
            continue;
          }
        } else {
          r.encoding = Encoding::Invalid;
          return r;
        }
      } else if state == T::ACCEPT {
        if T::is_invalid_surrogate_pair(previous, current) {
          r.encoding = Encoding::Invalid;
          return r;
        }
        is_one_byte &= current <= u32::from(latin1::MAX_CHAR);
        r.utf16_length += 1;
        if current > utf16::MAX_NON_SURROGATE_CHAR_CODE {
          r.utf16_length += 1;
        }
        previous = current;
        current = 0;
      }
      cursor += 1;
    }

    if state == T::ACCEPT {
      r.encoding = if is_one_byte { Encoding::Latin1 } else { Encoding::Utf16 };
    } else if T::ALLOW_INCOMPLETE {
      r.encoding = Encoding::Utf16;
      r.utf16_length += 1;
    } else {
      r.encoding = Encoding::Invalid;
    }
    r
  }

  #[must_use]
  pub fn encoding(&self) -> Encoding {
    self.encoding
  }
  #[must_use]
  pub fn is_ascii(&self) -> bool {
    self.encoding == Encoding::Ascii
  }
  #[must_use]
  pub fn is_one_byte(&self) -> bool {
    self.encoding <= Encoding::Latin1
  }
  #[must_use]
  pub fn utf16_length(&self) -> i32 {
    self.utf16_length
  }
  #[must_use]
  pub fn non_ascii_start(&self) -> i32 {
    self.non_ascii_start
  }

  /// 解码到单字节目标 (原版 Decode<uint8_t>): 全部码点窄化截断 (与原版 static_cast 一致)。
  pub fn decode_to_one_byte(&self, out: &mut [u8], data: &[u8]) {
    let nas = (self.non_ascii_start as usize).min(out.len()).min(data.len());
    out[..nas].copy_from_slice(&data[..nas]);
    let mut state = T::ACCEPT;
    let mut current = 0u32;
    let mut w = nas;

    let mut cursor = nas;
    while cursor < data.len() {
      let byte = data[cursor];
      if u32::from(byte) <= utf8::MAX_ONE_BYTE_CHAR && state == T::ACCEPT {
        if w < out.len() {
          out[w] = byte;
          w += 1;
        }
        cursor += 1;
        continue;
      }
      let previous_state = state;
      T::decode(byte, &mut state, &mut current);
      if T::ALLOW_INCOMPLETE && T::is_reject(&state) {
        state = T::ACCEPT;
        if w < out.len() {
          out[w] = utf8::BAD_CHAR as u8;
          w += 1;
        }
        current = 0;
        if previous_state != T::ACCEPT {
          continue;
        }
      } else if state == T::ACCEPT {
        if w < out.len() {
          out[w] = current as u8;
          w += 1;
        }
        current = 0;
      }
      cursor += 1;
    }
    if T::ALLOW_INCOMPLETE && state != T::ACCEPT && w < out.len() {
      out[w] = utf8::BAD_CHAR as u8;
    }
  }

  /// 解码到双字节目标 (原版 Decode<uint16_t>): 增补平面展开为代理对。
  pub fn decode_to_two_byte(&self, out: &mut [u16], data: &[u8]) {
    let nas = (self.non_ascii_start as usize).min(out.len()).min(data.len());
    for (out_slot, &b) in out[..nas].iter_mut().zip(&data[..nas]) {
      *out_slot = u16::from(b);
    }
    let mut state = T::ACCEPT;
    let mut current = 0u32;
    let mut w = nas;

    let mut cursor = nas;
    while cursor < data.len() {
      let byte = data[cursor];
      if u32::from(byte) <= utf8::MAX_ONE_BYTE_CHAR && state == T::ACCEPT {
        if w < out.len() {
          out[w] = u16::from(byte);
          w += 1;
        }
        cursor += 1;
        continue;
      }
      let previous_state = state;
      T::decode(byte, &mut state, &mut current);
      if T::ALLOW_INCOMPLETE && T::is_reject(&state) {
        state = T::ACCEPT;
        if w < out.len() {
          out[w] = utf8::BAD_CHAR as u16;
          w += 1;
        }
        current = 0;
        if previous_state != T::ACCEPT {
          continue;
        }
      } else if state == T::ACCEPT {
        if current <= utf16::MAX_NON_SURROGATE_CHAR_CODE {
          if w < out.len() {
            out[w] = current as u16;
            w += 1;
          }
        } else if w + 1 < out.len() {
          out[w] = utf16::lead_surrogate(current);
          out[w + 1] = utf16::trail_surrogate(current);
          w += 2;
        }
        current = 0;
      }
      cursor += 1;
    }
    if T::ALLOW_INCOMPLETE && state != T::ACCEPT && w < out.len() {
      out[w] = utf8::BAD_CHAR as u16;
    }
  }
}

/// 有损解码器: 非法序列 → U+FFFD 后继续。
pub type Utf8Decoder = Utf8DecoderBase<Utf8Traits>;
/// WTF-8 解码器: 允许孤立代理, 拒绝代理对。
pub type Wtf8Decoder = Utf8DecoderBase<Wtf8Traits>;
/// 严格解码器: 非法序列 → Invalid。
pub type StrictUtf8Decoder = Utf8DecoderBase<StrictUtf8Traits>;

impl Utf8Decoder {
  /// 恒不失败 (is_invalid 恒 false)。
  #[must_use]
  pub fn is_invalid(&self) -> bool {
    false
  }
}
impl Wtf8Decoder {
  #[must_use]
  pub fn is_invalid(&self) -> bool {
    self.encoding() == Encoding::Invalid
  }
}
impl StrictUtf8Decoder {
  #[must_use]
  pub fn is_invalid(&self) -> bool {
    self.encoding() == Encoding::Invalid
  }
}
`;

writeSrc("v8-strings", "unicode_decoder.rs", body);
console.log("written: unicode_decoder.rs");
