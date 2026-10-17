#!/usr/bin/env bun
// tool/src/strings/string-hasher.cc.js
// 机械提取 v8/src/strings/string-hasher.{cc,inl.h}: 数组索引哈希协议与顺序串哈希骨架
// (string-hasher.h 已由 tool/src/strings/string-hasher.h.js 产出 string_hasher.rs, 不重复)

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/strings/string-hasher.cc");
const inlSrc = readV8("src/strings/string-hasher-inl.h");
const hSrc = readV8("src/strings/string-hasher.h");

// 1. 哨兵常量
const zeroM = hSrc.match(/static const int kZeroHash = (\d+);/);
if (!zeroM) throw new Error("string-hasher.h 未找到 kZeroHash");
const zeroHash = parseInt(zeroM[1], 10);

// 2. inl 方法面对数
const inlFns = [
  "GetTrivialHash",
  "MakeArrayIndexHash",
  "DecodeArrayIndexFromHashField",
  "HashSequentialString",
  "RunningStringHasher::AddCharacter",
  "RunningStringHasher::Finalize",
];
for (const f of inlFns) {
  if (!inlSrc.includes(f)) throw new Error(`string-hasher-inl.h 未找到 ${f}`);
}

// 3. 索引哈希协议 (value<<shift | length<<shift2)
if (!/value <<= String::ArrayIndexValueBits::kShift;/.test(inlSrc)) throw new Error("索引值移位协议漂移");
if (!/value \|= length << String::ArrayIndexLengthBits::kShift;/.test(inlSrc)) throw new Error("索引长度移位协议漂移");

// 4. cc 侧: 8bit 压缩哈希读取层 (ConvertTo8BitHashReader)
const compM = ccSrc.match(/static constexpr unsigned kCompressionFactor = (\d+);/);
const expM = ccSrc.match(/static constexpr unsigned kExpansionFactor = (\d+);/);
if (!compM || compM[1] !== "2") throw new Error("kCompressionFactor 漂移");
if (!expM || expM[1] !== "1") throw new Error("kExpansionFactor 漂移");
for (const k of ["Read64", "Read32", "ReadSmall", "HashConvertingTo8Bit", "IsOnly8BitSIMD", "rapidhash<"]) {
  if (!ccSrc.includes(k)) throw new Error(`string-hasher.cc 未找到 ${k}`);
}
// 三路 SIMD 分支登记
for (const k of ["__SSE2__", "__ARM_NEON__"]) {
  if (!ccSrc.includes(k)) throw new Error(`string-hasher.cc 缺 ${k} 分支`);
}

// 5. 种子化索引分支登记 (x64/arm64 默认关闭)
const seeded = inlSrc.includes("#ifdef V8_ENABLE_SEEDED_ARRAY_INDEX_HASH");

let rust = banner("src/strings/string-hasher.cc.js", "src/strings/string-hasher.{cc,inl.h}", "字符串哈希 .cc/-inl 层骨架");

rust += `
/// 字符串哈希 .cc/-inl 层 (原版 string-hasher-inl.h / string-hasher.cc)
pub mod string_hasher_inl_skel {
  /// 全零哈希哨兵 (原版 kZeroHash): 计算值恰为 0 时以该哨兵替换
  pub const ZERO_HASH: u32 = ${zeroHash};

  /// 长串平凡哈希: hash = length (原版 GetTrivialHash; 长度 > kMaxHashCalcLength)
  pub fn get_trivial_hash(_length: u32) -> u32 {
    unimplemented!("骨架")
  }

  /// 数组索引哈希打包 (原版 MakeArrayIndexHash):
  /// value 左移 ArrayIndexValueBits::kShift, 或上 length 左移 ArrayIndexLengthBits::kShift
  pub fn make_array_index_hash(_value: u32, _length: u32) -> u32 {
    unimplemented!("骨架")
  }

  /// 从 hash 字段解回数组索引 (原版 DecodeArrayIndexFromHashField)
  pub fn decode_array_index_from_hash_field(_raw_hash_field: u32) -> u32 {
    unimplemented!("骨架")
  }

  /// 顺序串哈希入口 (原版 HashSequentialString<char_t>; 长度 0/1/短/长分层)
  pub fn hash_sequential_string(_chars: &[u8], _length: u32, _seed: u64) -> u32 {
    unimplemented!("骨架")
  }

  /// 增量混合 (原版 TryAddToHash / AddToHash; 0 值翻 ZERO_HASH)
  pub fn add_to_hash(_hash: u32, _key: u64) -> u32 {
    unimplemented!("骨架")
  }

  /// 逐字符运行态哈希 (原版 RunningStringHasher::AddCharacter/Finalize)
  pub fn running_add_character(_hash: &mut u32, _c: u16) {
    unimplemented!("骨架")
  }

  /// 哈希字段计算主入口 (原版 string-hasher.cc ComputeHashField 模板)
  pub fn compute_hash_field(_key: usize, _seed: u64) -> u32 {
    unimplemented!("骨架")
  }

  /// 双字节串压缩读取层 (原版 ConvertTo8BitHashReader):
  /// kCompressionFactor=2 (u16→u8 压缩), kExpansionFactor=1;
  /// Read64/Read32 按 SSE2 / NEON / 纯量三路, ReadSmall 取首/中/尾三字节
  pub mod convert_to_8bit_reader {
    pub const COMPRESSION_FACTOR: u32 = 2;
    pub const EXPANSION_FACTOR: u32 = 1;
    /// 读取原语清单 (对数)
    pub const READERS: [&str; 3] = ["Read64", "Read32", "ReadSmall"];
    /// SIMD 分支登记: __SSE2__ (x64) / __ARM_NEON__ (arm64)
    pub const SIMD_BRANCHES: [&str; 2] = ["SSE2", "NEON"];
  }

  /// 双字节→8bit 压缩哈希入口 (原版 detail::HashConvertingTo8Bit → rapidhash)
  pub fn hash_converting_to_8bit(_chars: &[u16], _length: u32, _seed: u64, _secret: &[u64; 3]) -> u64 {
    unimplemented!("骨架")
  }

  /// 纯 8bit 判定 (原版 detail::IsOnly8BitSIMD; highway 向量化扫描高字节)
  pub fn is_only_8bit_simd(_chars: &[u16], _len: u32) -> bool {
    unimplemented!("骨架")
  }

  /// 平台分支登记: V8_ENABLE_SEEDED_ARRAY_INDEX_HASH (x64/arm64 默认关闭)
  /// 启用时 SeedArrayIndexValue/UnseedArrayIndexValue 经 HashSeed m1..m3 域乘逆
  pub const SEEDED_ARRAY_INDEX_HASH_ENABLED: bool = ${seeded};

  /// 模板显式化清单 (原版 string-hasher.cc 压缩读取原语)
  pub const CC_TEMPLATE_INSTANTIATIONS: [&str; 3] = ["Read64", "Read32", "ReadSmall"];
}
`;

const p = writeSrc("v8-strings", "string_hasher_inl_generated.rs", rust);
console.log(`written ${p}; ZERO_HASH=${zeroHash}, inl方法=${inlFns.length}, 压缩读取=3`);
