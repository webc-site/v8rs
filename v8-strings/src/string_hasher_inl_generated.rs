// 由 tool/src/strings/string-hasher.cc.js 从 v8/src/strings/string-hasher.{cc,inl.h} 生成; 修订就地改, 复跑覆盖
// 字符串哈希 .cc/-inl 层骨架与实现

use crate::{
  hash_field::{HashFieldType, create_hash_field_value},
  hash_seed::HashSeedData,
  rapidhash::{ConvertTo8BitReader, rapidhash},
  string_hasher::{
    decode_array_index_from_hash_field as decode_index_raw, get_trivial_hash as trivial_hash_raw,
    hash_sequential_one_byte, is_only_8_bit, make_array_index_hash_raw,
  },
};

/// 字符串哈希 .cc/-inl 层 (原版 string-hasher-inl.h / string-hasher.cc)
pub mod string_hasher_inl_skel {
  use super::*;
  use crate::hash_field::MAX_HASH_CALC_LENGTH;

  /// 全零哈希哨兵 (原版 kZeroHash): 计算值恰为 0 时以该哨兵替换
  pub const ZERO_HASH: u32 = 27;

  /// 长串平凡哈希: hash = length (原版 GetTrivialHash; 长度 > kMaxHashCalcLength)
  #[inline]
  #[must_use]
  pub fn get_trivial_hash(length: u32) -> u32 {
    if length > MAX_HASH_CALC_LENGTH {
      trivial_hash_raw(length)
    } else {
      create_hash_field_value(length, HashFieldType::Hash)
    }
  }

  /// 数组索引哈希打包 (原版 MakeArrayIndexHash):
  /// value 左移 ArrayIndexValueBits::kShift, 或上 length 左移 ArrayIndexLengthBits::kShift
  #[inline]
  #[must_use]
  pub fn make_array_index_hash(value: u32, length: u32) -> u32 {
    make_array_index_hash_raw(value, length)
  }

  /// 从 hash 字段解回数组索引 (原版 DecodeArrayIndexFromHashField)
  #[inline]
  #[must_use]
  pub fn decode_array_index_from_hash_field(raw_hash_field: u32) -> u32 {
    decode_index_raw(raw_hash_field)
  }

  /// 顺序串哈希入口 (原版 HashSequentialString<char_t>; 长度 0/1/短/长分层)
  #[inline]
  #[must_use]
  pub fn hash_sequential_string(chars: &[u8], length: u32, seed: u64) -> u32 {
    let len = (length as usize).min(chars.len());
    let seed_data = HashSeedData::from_seed(seed);
    hash_sequential_one_byte(&chars[..len], &seed_data, None)
  }

  /// 增量混合 (原版 TryAddToHash / AddToHash; 0 值翻 ZERO_HASH)
  #[inline]
  #[must_use]
  pub fn add_to_hash(hash: u32, key: u64) -> u32 {
    let mut h = hash;
    running_add_character(&mut h, (key & 0xffff) as u16);
    running_add_character(&mut h, ((key >> 16) & 0xffff) as u16);
    running_add_character(&mut h, ((key >> 32) & 0xffff) as u16);
    running_add_character(&mut h, ((key >> 48) & 0xffff) as u16);
    if h == 0 { ZERO_HASH } else { h }
  }

  /// 逐字符运行态哈希 (原版 RunningStringHasher::AddCharacter/Finalize)
  #[inline]
  pub fn running_add_character(hash: &mut u32, c: u16) {
    *hash = hash.wrapping_add(u32::from(c));
    *hash = hash.wrapping_add(*hash << 10);
    *hash ^= *hash >> 6;
  }

  /// 哈希字段计算主入口 (原版 string-hasher.cc ComputeHashField 模板)
  #[inline]
  #[must_use]
  pub fn compute_hash_field(key: usize, seed: u64) -> u32 {
    let raw = add_to_hash(seed as u32, key as u64);
    create_hash_field_value(raw, HashFieldType::Hash)
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
  #[inline]
  #[must_use]
  pub fn hash_converting_to_8bit(chars: &[u16], length: u32, seed: u64, secret: &[u64; 3]) -> u64 {
    let len = (length as usize).min(chars.len());
    rapidhash::<u16, ConvertTo8BitReader>(&chars[..len], len, seed, secret)
  }

  /// 纯 8bit 判定 (原版 detail::IsOnly8BitSIMD; highway 向量化扫描高字节)
  #[inline]
  #[must_use]
  pub fn is_only_8bit_simd(chars: &[u16], len: u32) -> bool {
    let len = (len as usize).min(chars.len());
    is_only_8_bit(&chars[..len])
  }

  /// 平台分支登记: V8_ENABLE_SEEDED_ARRAY_INDEX_HASH (x64/arm64 默认关闭)
  /// 启用时 SeedArrayIndexValue/UnseedArrayIndexValue 经 HashSeed m1..m3 域乘逆
  pub const SEEDED_ARRAY_INDEX_HASH_ENABLED: bool = true;

  /// 模板显式化清单 (原版 string-hasher.cc 压缩读取原语)
  pub const CC_TEMPLATE_INSTANTIATIONS: [&str; 3] = ["Read64", "Read32", "ReadSmall"];
}
