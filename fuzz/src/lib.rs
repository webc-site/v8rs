//! V8rs 跨组件模糊测试 Harness (1:1 对标 C++ v8/test/fuzzer/)
//!
//! 提供 JSON、RegExp、Parser 与 Fuzzilli 目标入口，供 libFuzzer 与全平台语料回归复用。

#![allow(unused_crate_dependencies)]

use std::{io, str::from_utf8, sync::LazyLock};

use v8_fuzzilli::FuzzilliExtension;
use v8_parsing::{Scanner, ScannerFlags, ScannerStream};

/// 最大 JSON 输入长度 (16 KiB，与 C++ v8/test/fuzzer/json.cc 对齐)
pub const MAX_JSON_INPUT_SIZE: usize = 16 * 1024;

/// 最大正则输入长度 (16 KiB，与 C++ v8/test/fuzzer/regexp.cc 对齐)
pub const MAX_REGEXP_INPUT_SIZE: usize = 16 * 1024;

/// 最大代码解析输入长度 (4 KiB，与 C++ v8/test/fuzzer/parser.cc 对齐)
pub const MAX_PARSER_INPUT_SIZE: usize = 4096;

/// 最大 Fuzzilli 输入长度 (1 KiB，与 CI -max_len=1024 对齐)
pub const MAX_FUZZILLI_INPUT_SIZE: usize = 1024;

/// 最大扫描 Token 步数，防止 ReDoS 与极端深度回溯
pub const MAX_SCAN_STEPS: usize = 10_000;

/// Fuzzilli 静态单例扩展 (避免每轮模糊测试重复分配字符串)
static FUZZILLI_EXT: LazyLock<v8_fuzzilli::FuzzilliExtension> =
  LazyLock::new(|| FuzzilliExtension::new("fuzzilli"));

/// JSON 模糊测试执行体 (对应 v8/test/fuzzer/json.cc)
///
/// 规约保证:
/// - 零拷贝校验 UTF-8
/// - 成功解析时同步覆盖紧凑与缩进序列化分支，保证无 panic
#[inline]
pub fn fuzz_json(data: &[u8]) {
  if data.is_empty() || data.len() > MAX_JSON_INPUT_SIZE {
    return;
  }
  if let Ok(source) = from_utf8(data)
    && let Ok(value) = v8_json::parse(source)
  {
    let _ = v8_json::stringify(&value);
    let _ = v8_json::stringify_indent(&value, "  ");
  }
}

/// RegExp 模糊测试执行体 (对应 v8/test/fuzzer/regexp.cc)
///
/// 规约保证:
/// - 首字节低 9 位映射 RegExpFlags (HAS_INDICES 到 STICKY)
/// - 零拷贝切片载荷
/// - 双通道测试: UTF-8 语法解析通道与基于字节/UTF-16 码元的底层解析
#[inline]
pub fn fuzz_regexp(data: &[u8]) {
  if data.is_empty() || data.len() > MAX_REGEXP_INPUT_SIZE {
    return;
  }
  // 首字节低 9 位映射标志位组合
  let flags = v8_regexp_syntax::RegExpFlags((data[0] as u16) & 0x01FF);
  let payload = &data[1..];

  // 通道 1: 若为合法 UTF-8，测试标准 parse
  if let Ok(pattern) = from_utf8(payload) {
    let _ = v8_regexp_syntax::parse(pattern, flags);
  }

  // 通道 2: 无论是否为 UTF-8，零拷贝扩展至 u16 Latin-1 码元，测试 parse_utf16
  // (1:1 对标 V8 NewStringFromOneByte 探测非 UTF-8 正则语法)
  if payload.len() <= 2048 {
    let utf16_chars: Vec<u16> = payload.iter().map(|&b| b as u16).collect();
    let _ = v8_regexp_syntax::parse_utf16(&utf16_chars, flags);
  }
}

/// Parser 模糊测试执行体 (对应 v8/test/fuzzer/parser.cc)
///
/// 规约保证:
/// - 零拷贝流扫描，1:1 对标 V8 NewStringFromOneByte Latin1 与 UTF-8 双通道
/// - 基于 Iterator::take 单次迭代，避免手动计数与冗余分支
#[inline]
pub fn fuzz_parser(data: &[u8]) {
  if data.is_empty() || data.len() > MAX_PARSER_INPUT_SIZE {
    return;
  }
  // 通道 1: OneByte 零拷贝流 (1:1 对标 V8 NewStringFromOneByte，覆盖全部二进制输入)
  let mut scanner_one_byte =
    Scanner::with_flags(ScannerStream::for_one_byte(data), ScannerFlags::default());
  scanner_one_byte.initialize();
  for _ in scanner_one_byte.take(MAX_SCAN_STEPS) {}

  // 通道 2: UTF-8 / UTF-16 流 (若输入合法，额外覆盖多字节字符分支)
  if let Ok(source) = from_utf8(data) {
    let mut scanner_utf =
      Scanner::with_flags(ScannerStream::for_testing(source), ScannerFlags::default());
    scanner_utf.initialize();
    for _ in scanner_utf.take(MAX_SCAN_STEPS) {}
  }
}

/// Fuzzilli 扩展分派模糊测试 (对应 v8/src/fuzzilli/fuzzilli.cc)
///
/// 规约保证:
/// - split_once 单次扫描消除 splitn 迭代器
/// - 复用 LazyLock 扩展单例，避免高频堆分配
#[inline]
pub fn fuzz_fuzzilli(data: &[u8]) {
  if data.is_empty() || data.len() > MAX_FUZZILLI_INPUT_SIZE {
    return;
  }
  if let Ok(source) = from_utf8(data) {
    let (op, arg) = match source.split_once(' ') {
      Some((o, a)) => (o, Some(a)),
      None => (source, None),
    };
    let mut sink = io::sink();
    let _ = FUZZILLI_EXT.dispatch(op, arg, &mut sink);
  }
}
