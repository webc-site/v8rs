#!/usr/bin/env bun
// v8/src/tracing/{perfetto-utils.h, perfetto-utils.cc} → v8-tracing/src/perfetto_utils.rs
//
// PerfettoV8String 数据面规则代码 (Tagged<String> 读出部分依赖对象层, 推迟):
//   - is_one_byte 判定 + 每字符字节数 (1 / sizeof(uc16)=2)
//   - WriteToProto 编码选择: one-byte → latin1, 双字节 → utf16le (大端目标 → utf16be)
//   - operator== (is_one_byte + size + memcmp) 与 Hasher (bytes + is_one_byte)
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-tracing";
const GEN = "src/tracing/perfetto-utils.h.js";
const H = readV8("src/tracing/perfetto-utils.h");
const CC = readV8("src/tracing/perfetto-utils.cc");

// 字节宽核对
if (!/sizeof\(base::uc16\)/.test(CC)) throw new Error("uc16 宽度语义未找到");
if (!H.includes("set_latin1") || !H.includes("set_utf16_be")) throw new Error("编码选择未找到");

let out = `${banner(GEN, "src/tracing/perfetto-utils.h;src/tracing/perfetto-utils.cc", "数据面规则代码, Tagged<String> 读出推迟")}
//! PerfettoV8String (perfetto-utils.h/.cc, 72+33 行): V8 String → Perfetto proto
//! 字符串写入辅助 (字符编码 + 多 slice 拼平)。
//!
//! 数据面 (is_one_byte / size / buffer / eq / hash / 编码选择) 全量落地;
//! 构造入参 \`Tagged<String>\` 依赖对象层 (L3), 以扁平字节切片注入等价承接;
//! \`WriteToProto(proto&)\` 的 proto 写入依赖 perfetto protos【推迟登记】,
//! 编码选择以 \`StringEncoding\` 枚举暴露。

/// 双字节字符串单字符字节数 (base::uc16 = 2)。
pub const UC16_SIZE: usize = 2;

/// proto 写入编码选择 (perfetto-utils.h:35-45 展开)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringEncoding {
  /// 单字节 → set_latin1。
  Latin1,
  /// 双字节小端目标 → set_utf16_le。
  Utf16Le,
  /// 双字节大端目标 (V8_TARGET_BIG_ENDIAN) → set_utf16_be。
  Utf16Be,
}

/// 对应 \`PerfettoV8String\` (perfetto-utils.h:23): 持扁平化字节 + 编码标记。
#[derive(Debug)]
pub struct PerfettoV8String {
  pub is_one_byte: bool,
  pub size: usize,
  pub buffer: Vec<u8>,
}

impl PerfettoV8String {
  /// 构造 (perfetto-utils.cc:15-30): 长度 0 → 空缓冲;
  /// 单字节宽 1, 双字节宽 2 (uc16), WriteToFlat 拼平 — Rust 以入参字节承接。
  pub fn from_flat(is_one_byte: bool, bytes: &[u8]) -> Self {
    Self { is_one_byte, size: bytes.len(), buffer: bytes.to_vec() }
  }

  /// 编码选择 (WriteToProto 展开): 小端目标 LE, 大端 BE。
  pub fn encoding(&self) -> StringEncoding {
    if self.is_one_byte {
      StringEncoding::Latin1
    } else if cfg!(target_endian = "big") {
      StringEncoding::Utf16Be
    } else {
      StringEncoding::Utf16Le
    }
  }

  /// operator== (perfetto-utils.h:47-50): is_one_byte + size + memcmp。
  pub fn eq(&self, o: &Self) -> bool {
    self.is_one_byte == o.is_one_byte && self.size == o.size && self.buffer == o.buffer
  }

  /// Hasher (perfetto-utils.h:54-61): Hasher{AddRange(bytes), Combine(is_one_byte)}。
  pub fn hash(&self) -> usize {
    v8_base::hashing::hash_combine(
      v8_base::hashing::hash_range(self.buffer.iter().copied()),
      self.is_one_byte as usize,
    )
  }
}
`;

writeSrc(CRATE, "perfetto_utils.rs", out);
console.log("== perfetto-utils.h.js == 数据面 1 结构体 + 编码 3 态");
