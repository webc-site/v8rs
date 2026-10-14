#!/usr/bin/env bun
// tool/src/strings/unicode.cc.js
// 机械提取 v8/src/strings/unicode.{cc,-inl.h}: 表驱动谓词/转换清单, UTF8/UTF16/WTF8 编解码骨架
// (>2k 行, 按规则只提取常量/谓词清单/签名骨架; unicode.h 已由既有 unicode.h.js 产出)

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/strings/unicode.cc");
const inlSrc = readV8("src/strings/unicode-inl.h");

// 1. 表驱动核心常量 (cc 顶部)
const startBitM = ccSrc.match(/static const int kStartBit = \(1 << (\d+)\);/);
const chunkBitsM = ccSrc.match(/static const int kChunkBits = \(1 << (\d+)\);/);
if (!startBitM || startBitM[1] !== "30") throw new Error("kStartBit 漂移");
if (!chunkBitsM || chunkBitsM[1] !== "13") throw new Error("kChunkBits 漂移");
if (!/static const uchar kSentinel = static_cast<uchar>\(-1\);/.test(ccSrc)) throw new Error("kSentinel 漂移");

// 2. 表访问原语
for (const k of ["TableGet", "GetEntry", "IsStart", "LookupPredicate", "LookupMapping"]) {
  if (!new RegExp(`\\b${k}\\(`).test(ccSrc)) throw new Error(`unicode.cc 未找到表原语 ${k}`);
}

// 3. INTL/非 INTL 双路径谓词与转换清单
const isFns = [...ccSrc.matchAll(/^bool (\w+)::Is\(uchar c\) \{/gm)].map((m) => m[1]);
const convFns = [...ccSrc.matchAll(/^int (\w+)::Convert\(uchar c, uchar n, uchar\* result,/gm)].map((m) => m[1]);
const isSet = [...new Set(isFns)];
const convSet = [...new Set(convFns)];
if (isSet.join() !== "Uppercase,Letter,ID_Start,ID_Continue,WhiteSpace") {
  throw new Error(`谓词清单漂移: ${isSet.join()}`);
}
if (convSet.join() !== "ToLowercase,ToUppercase,Ecma262Canonicalize,Ecma262UnCanonicalize,CanonicalizationRange") {
  throw new Error(`转换清单漂移: ${convSet.join()}`);
}

// 4. 编解码函数面 (HasUnpairedSurrogate 在 -inl)
const codecFns = [
  "Utf8::CalculateValue",
  "Utf8::ValueOfIncremental",
  "Utf8::ValueOfIncrementalFinish",
  "Utf8::ValidateEncoding",
  "Utf16::ReplaceUnpairedSurrogates",
  "Wtf8::ValidateEncoding",
  "Wtf8::ScanForSurrogates",
  "Utf8::WriteLeadingAscii",
];
for (const k of codecFns) {
  if (!ccSrc.includes(k)) throw new Error(`unicode.cc 未找到 ${k}`);
}
if (!inlSrc.includes("Utf16::HasUnpairedSurrogate")) throw new Error("-inl 缺 HasUnpairedSurrogate");

// 5. UTF8 状态机
if (!/State state = State::kAccept;/.test(ccSrc)) throw new Error("UTF8 kAccept 状态漂移");

// 6. -inl: 非 INTL 缓存协议 (kSize/kMask 定义在 unicode.h, -inl 用取模寻址)
const maskM = inlSrc.match(/entries_\[code_point & kMask\]/);
if (!maskM) throw new Error("-inl 缓存 kMask 协议漂移");
if (!inlSrc.includes("simdutf::validate_utf16")) throw new Error("-inl 缺 simdutf utf16 校验");
// 双模态: 非 INTL 才有 Predicate/Mapping 缓存
if (!inlSrc.includes("#ifndef V8_INTL_SUPPORT")) throw new Error("-inl 缺非 INTL 模态标记");

let rust = banner("src/strings/unicode.cc.js", "src/strings/unicode.{cc,-inl.h}", "Unicode 表驱动与编解码骨架");

rust += `
/// 压缩表编码常量 (原版 unicode.cc 顶部)
pub mod unicode_table_coding {
  /// 起始项位 (原版 kStartBit = 1 << 30)
  pub const START_BIT: i32 = 1 << ${startBitM[1]};
  /// 分块位宽 (原版 kChunkBits = 1 << 13)
  pub const CHUNK_BITS: i32 = 1 << ${chunkBitsM[1]};
  /// 哨兵码点 (原版 kSentinel = uchar(-1))
  pub const SENTINEL: u32 = u32::MAX;

  /// 项内取值 (原版 GetEntry: entry & (kStartBit-1))
  pub const ENTRY_VALUE_MASK: i32 = (1 << ${startBitM[1]}) - 1;

  /// 表驱动谓词查找签名 (原版 LookupPredicate; 二级表: 块表+块内偏移)
  pub fn lookup_predicate(_table: &[i32], _size: u16, _chr: u32) -> bool {
    unimplemented!("骨架")
  }

  /// 表驱动映射签名 (原版 LookupMapping)
  pub fn lookup_mapping(_table: &[i32], _size: u16, _chr: u32) -> u32 {
    unimplemented!("骨架")
  }

  /// 项是否为块起始 (原版 IsStart: entry & kStartBit)
  pub fn is_start(_entry: i32) -> bool {
    _entry & START_BIT != 0
  }
}

/// unibrow 谓词/转换族清单 (原版 unicode.cc; INTL 与非 INTL 双实现)
pub mod unicode_families {
  /// 谓词 (Is), ${isSet.length} 项 — INTL 走 ICU, 非 INTL 走内嵌表
  pub const PREDICATES: [&str; ${isSet.length}] = [${isSet.map((f) => `"${f}"`).join(", ")}];
  /// 转换 (Convert), ${convSet.length} 项
  pub const CONVERTERS: [&str; ${convSet.length}] = [${convSet.map((f) => `"${f}"`).join(", ")}];
}

/// 编解码函数骨架 (原版 unicode.cc Utf8/Utf16/Wtf8 段)
pub mod unicode_codec_skel {
  /// 增量解码终态 (原版 State::kAccept; 拒绝为 kReject)
  pub const UTF8_ACCEPT: u8 = 0;

  /// 多字节序列求值 (原版 Utf8::CalculateValue; kBadChar 失败)
  pub fn utf8_calculate_value(_str: &[u8], _max_length: usize) -> u32 {
    unimplemented!("骨架")
  }
  /// 增量解码单步 (原版 Utf8::ValueOfIncremental; ASCII 快路径 + 状态机)
  pub fn utf8_value_of_incremental(_cursor: &mut &[u8], _state: &mut u8, _buffer: &mut u32) -> u32 {
    unimplemented!("骨架")
  }
  /// 流尾收尾 (原版 Utf8::ValueOfIncrementalFinish)
  pub fn utf8_value_of_incremental_finish(_state: &mut u8) -> u32 {
    unimplemented!("骨架")
  }
  /// UTF-8 校验 (原版 simdutf::validate_utf8)
  pub fn utf8_validate_encoding(_bytes: &[u8]) -> bool {
    unimplemented!("骨架")
  }
  /// 未配对代理项检测 (原版 Utf16::HasUnpairedSurrogate → simdutf validate_utf16)
  pub fn utf16_has_unpaired_surrogate(_units: &[u16]) -> bool {
    unimplemented!("骨架")
  }
  /// 未配对代理项替换 (原版 Utf16::ReplaceUnpairedSurrogates → U+FFFD)
  pub fn utf16_replace_unpaired_surrogates(_src: &mut [u16]) {
    unimplemented!("骨架")
  }
  /// WTF-8 校验 (原版 Wtf8::ValidateEncoding)
  pub fn wtf8_validate_encoding(_bytes: &[u8]) -> bool {
    unimplemented!("骨架")
  }
  /// WTF-8 代理扫描 (原版 Wtf8::ScanForSurrogates)
  pub fn wtf8_scan_for_surrogates(_wtf8: &[u8]) {
    unimplemented!("骨架")
  }
  /// ASCII 前导批量写 (原版 Utf8::WriteLeadingAscii, uint8/uint16 双特化)
  pub fn utf8_write_leading_ascii(_src: &[u8], _dest: &mut [u8]) -> usize {
    unimplemented!("骨架")
  }
}

/// 非 INTL 单项缓存协议 (原版 unicode-inl.h #ifndef V8_INTL_SUPPORT 段)
pub mod unibrow_cache_skel {
  use super::unibrow_cache_const::*;

  /// 谓词缓存查询 (原版 Predicate::get; 命中即取, 未命中算后写回)
  pub fn predicate_get(_entries: &mut [CacheEntry], _code_point: u32) -> bool {
    unimplemented!("骨架")
  }
  /// 映射缓存查询 (原版 Mapping::get; offset=0 表示不可映射)
  pub fn mapping_get(_entries: &mut [CacheEntry], _c: u32, _n: u32, _result: &mut [u32]) -> i32 {
    unimplemented!("骨架")
  }
}

/// 缓存布局常量 (原版 unicode.h 模板: kMask = kSize-1, 直接取模寻址)
pub mod unibrow_cache_const {
  /// 缓存条目 (code_point + 值/偏移)
  pub struct CacheEntry {
    pub code_point: u32,
    pub value: i32,
  }
}
`;

const p = writeSrc("v8-strings", "unicode_cc_generated.rs", rust);
console.log(`written ${p}; 谓词=${isSet.length}, 转换=${convSet.length}, 编解码=${codecFns.length}`);
