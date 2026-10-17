#!/usr/bin/env bun
// tool/src/strings/string-hasher.h.js — 覆盖 v8/src/strings/string-hasher.{h,cc,-inl.h}
//        + v8/src/objects/name.h 哈希字段位域 (越界落地, 见 crate README)
//        + v8/src/numbers/hash-seed.{h,cc,-inl.h} (越界落地, 上游 v8-numbers 未开发)
//        + third_party/rapidhash-v8/{rapidhash.h,secret.h}
//        → v8-strings/src/{string_hasher.rs, hash_field.rs, hash_seed.rs, rapidhash.rs}
// 规则部分: name.h 位域常量组 1:1 展开 (kBitsPerInt=32 代入), RAPIDHASH_DEFAULT_SECRET 提取;
//           rapidhash / TryParseArrayIndex / HashSequentialString 语义随脚本产出
import { banner, readV8, writeSrc } from "../../lib.js";

// ---- 从 name.h 抽取并按 kBitsPerInt=32 展开位域常量 (对数核对源) ----
const nameH = readV8("src/objects/name.h");
const expect = {
  kMaxCachedArrayIndexLength: Number(/kMaxCachedArrayIndexLength = (\d+)/.exec(nameH)[1]),
  kMaxArrayIndex: 0xFFFFFFFE, // kMaxUInt32 - 1
  kMaxArrayIndexSize: Number(/kMaxArrayIndexSize = (\d+)/.exec(nameH)[1]),
  kArrayIndexValueBits: Number(/kArrayIndexValueBits = (\d+)/.exec(nameH)[1]),
};
if (expect.kMaxCachedArrayIndexLength !== 7 || expect.kMaxArrayIndexSize !== 10 || expect.kArrayIndexValueBits !== 24) {
  throw new Error(`name.h 常量漂移: ${JSON.stringify(expect)}`);
}
const kBitsPerInt = 32;
const kHashFieldTypeBits = 2;
const kArrayIndexLengthBits = kBitsPerInt - expect.kArrayIndexValueBits - kHashFieldTypeBits; // 6
const kArrayIndexValueShift = kHashFieldTypeBits; // 2
const kArrayIndexLengthShift = kHashFieldTypeBits + expect.kArrayIndexValueBits; // 26
const kHashBitsShift = kHashFieldTypeBits; // 2
const kHashBitsSize = kBitsPerInt - kHashFieldTypeBits; // 30
const kHashBitsMax = (1 << kHashBitsSize) - 1; // 0x3FFFFFFF

// string.h: kMaxHashCalcLength
const stringH = readV8("src/objects/string.h");
const kMaxHashCalcLength = Number(/kMaxHashCalcLength = (\d+)/.exec(stringH)[1]); // 16383
if (kMaxHashCalcLength !== 16383) throw new Error("kMaxHashCalcLength 漂移");

// 64 位平台: kMaxIntegerIndexSize = min(16, 10 * (8/4)) = 20 → min(16,20) = 16? 原版 min(16, 10*(sizeof/4))
const kMaxIntegerIndexSize = Math.min(16, 10 * (8 / 4)); // 64 位: 20 → 16... 10*(8/4)=20, min(16,20)=16?

// ---- rapidhash secret.h: 提取默认 secret (仅 RAPIDHASH_DEFAULT_SECRET 数组字面量内的 3 项) ----
const secretH = readV8("third_party/rapidhash-v8/secret.h");
const secretArraySrc = /RAPIDHASH_DEFAULT_SECRET\[3\] = \{([^}]*)\}/.exec(secretH)[1];
const secrets = [...secretArraySrc.matchAll(/0x[0-9a-fA-F]+/g)].map((m) => m[0]);
if (secrets.length !== 3) throw new Error(`RAPIDHASH_DEFAULT_SECRET 提取异常: ${secrets.length} 项`);

// ---- secret.h rapidhash_make_secret 的平衡字节表 ----
const cTableMatch = /uint8_t c\[\] = \{([^}]*)\}/.exec(secretH);
const cTable = cTableMatch[1].split(",").map((s) => s.trim()).filter(Boolean).map(Number);
if (cTable.length !== 70) throw new Error(`make_secret c 表长异常: ${cTable.length}`);

const hashField = `${banner("src/strings/string-hasher.h.js", "src/objects/name.h + src/objects/string.h 哈希字段位域", "objects 层未落地, 位域语义绑定原地承接; v8-objects-strings 落地后归并")}
//! Name::raw_hash_field 位域布局 (kBitsPerInt=32, HashFieldType 占低 2 位)。
//!
//! 布局 (低位 → 高位):
//! - HashFieldType (2 bit)
//! - ArrayIndexValue (\`K_ARRAY_INDEX_VALUE_BITS\` bit, 仅 integer-index 编码)
//! - ArrayIndexLength (\`K_ARRAY_INDEX_LENGTH_BITS\` bit, 仅 integer-index 编码)
//! - Hash (30 bit, 仅 hash 编码)

/// 哈希字段类型 (HashFieldType)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum HashFieldType {
  /// 普通哈希。
  Hash = 0b10,
  /// 整数索引 (array index 或更长的整数串)。
  IntegerIndex = 0b00,
  /// 转发索引 (内部化)。
  ForwardingIndex = 0b01,
  /// 空 (未计算)。
  Empty = 0b11,
}

pub const HASH_FIELD_TYPE_BITS: u32 = ${kHashFieldTypeBits};
/// 类型位掩码 (HashFieldTypeBits::kMask)。
pub const HASH_FIELD_TYPE_MASK: u32 = (1 << HASH_FIELD_TYPE_BITS) - 1;
/// 哈希值位宽 (kBitsPerInt - 2)。
pub const HASH_BITS_SIZE: u32 = ${kHashBitsSize};
/// 哈希值移位 (HashBits::kShift)。
pub const HASH_BITS_SHIFT: u32 = HASH_FIELD_TYPE_BITS;
/// 哈希值掩码 (HashBits::kMax)。
pub const HASH_BITS_MAX: u32 = 0x${kHashBitsMax.toString(16)};

/// array index 字符串短于此长度时, 索引可缓存在哈希字段内。
pub const MAX_CACHED_ARRAY_INDEX_LENGTH: u32 = ${expect.kMaxCachedArrayIndexLength};
/// 最大 array index 值 (kMaxUInt32 - 1)。
pub const MAX_ARRAY_INDEX: u32 = ${expect.kMaxArrayIndex};
/// array index 十进制串最大字符数。
pub const MAX_ARRAY_INDEX_SIZE: u32 = ${expect.kMaxArrayIndexSize};
/// 整数索引串最大字符数 (64 位: 10 * (usize/4) 与 16 取小)。
pub const MAX_INTEGER_INDEX_SIZE: u32 = ${kMaxIntegerIndexSize};
/// array index 值位宽。
pub const ARRAY_INDEX_VALUE_BITS: u32 = ${expect.kArrayIndexValueBits};
/// array index 值掩码。
pub const ARRAY_INDEX_VALUE_MASK: u32 = 0x${((1 << expect.kArrayIndexValueBits) - 1).toString(16)};
/// array index 长度位宽。
pub const ARRAY_INDEX_LENGTH_BITS: u32 = ${kArrayIndexLengthBits};
/// 值域在 raw_hash_field 中的移位。
pub const ARRAY_INDEX_VALUE_SHIFT: u32 = ${kArrayIndexValueShift};
/// 长度域在 raw_hash_field 中的移位。
pub const ARRAY_INDEX_LENGTH_SHIFT: u32 = ${kArrayIndexLengthShift};
/// 任一位置位 → 哈希字段不含缓存 array index。
pub const DOES_NOT_CONTAIN_CACHED_ARRAY_INDEX_MASK: u32 =
  (!MAX_CACHED_ARRAY_INDEX_LENGTH).wrapping_shl(ARRAY_INDEX_LENGTH_SHIFT) | HASH_FIELD_TYPE_MASK;
/// 任一位置位 → 哈希字段不含整数/转发索引。
pub const DOES_NOT_CONTAIN_INTEGER_OR_FORWARDING_INDEX_MASK: u32 = 0b10;

/// 哈希计算的最大串长; 超出仅按长度取哈希。
pub const MAX_HASH_CALC_LENGTH: u32 = ${kMaxHashCalcLength};

/// Name::CreateHashFieldValue。
#[inline]
#[must_use]
pub fn create_hash_field_value(hash: u32, ty: HashFieldType) -> u32 {
  debug_assert_ne!(ty, HashFieldType::ForwardingIndex);
  (hash & HASH_BITS_MAX) << HASH_BITS_SHIFT | ty as u32
}

/// Name::ContainsCachedArrayIndex。
#[inline]
#[must_use]
pub fn contains_cached_array_index(raw_hash_field: u32) -> bool {
  raw_hash_field & DOES_NOT_CONTAIN_CACHED_ARRAY_INDEX_MASK == 0
}

/// Name::IsIntegerIndex。
#[inline]
#[must_use]
pub fn is_integer_index(raw_hash_field: u32) -> bool {
  raw_hash_field & DOES_NOT_CONTAIN_INTEGER_OR_FORWARDING_INDEX_MASK == 0
}
`;

const rapidhash = `${banner("src/strings/string-hasher.h.js", "third_party/rapidhash-v8/rapidhash.h", "字符串 hash 语义绑定 (seed+迭代序), 1:1 自写, 禁库替换")}
//! rapidhash — wyhash 派生的高速哈希; u128 乘法内核。
//! 偏移算术: 产出侧字节偏移直接作切片索引 — PlainReader 输入 u8 (索引=字节偏移),
//! ConvertTo8BitReader 输入 u16 (压缩 2:1, 索引=产出偏移); 与原版 \`off * x / y\` 等价。

/// 读取器: 从输入单元流产出哈希位, 支持 16→8 位压缩 (Latin-1 内容的双字节串
/// 与单字节串哈希一致) 或原样读取。
pub trait HashReader<C> {
  /// 产出 64 bit。
  fn read64(p: &[C]) -> u64;
  /// 产出 32 bit (返回 u64, 哈希器自由移位)。
  fn read32(p: &[C]) -> u64;
  /// 读 1-3 单元并散布到 64 bit。
  fn read_small(p: &[C], k: usize) -> u64;
}

/// 原样字节读取 (PlainHashReader)。
pub struct PlainReader;
impl HashReader<u8> for PlainReader {
  #[inline]
  fn read64(p: &[u8]) -> u64 {
    let mut b = [0u8; 8];
    b.copy_from_slice(&p[..8]);
    u64::from_le_bytes(b)
  }

  #[inline]
  fn read32(p: &[u8]) -> u64 {
    let mut b = [0u8; 4];
    b.copy_from_slice(&p[..4]);
    u64::from(u32::from_le_bytes(b))
  }

  #[inline]
  fn read_small(p: &[u8], k: usize) -> u64 {
    (u64::from(p[0]) << 56) | (u64::from(p[k >> 1]) << 32) | u64::from(p[k - 1])
  }
}

/// Latin-1 压缩读取 (ConvertTo8BitHashReader): 每 u16 取低字节。
pub struct ConvertTo8BitReader;
impl HashReader<u16> for ConvertTo8BitReader {
  #[inline]
  fn read64(p: &[u16]) -> u64 {
    // 调用方 IsOnly8Bit 预检: 全部 ≤ 0xFF, 低字节即完整值
    let mut b = [0u8; 8];
    for (i, o) in b.iter_mut().enumerate() {
      *o = p[i] as u8;
    }
    u64::from_le_bytes(b)
  }

  #[inline]
  fn read32(p: &[u16]) -> u64 {
    let mut b = [0u8; 4];
    for (i, o) in b.iter_mut().enumerate() {
      *o = p[i] as u8;
    }
    u64::from(u32::from_le_bytes(b))
  }

  #[inline]
  fn read_small(p: &[u16], k: usize) -> u64 {
    (u64::from(p[0] as u8) << 56) | (u64::from(p[k >> 1] as u8) << 32) | u64::from(p[k - 1] as u8)
  }
}

/// 64×64 → 128 乘法 (rapid_mul128)。
#[inline]
fn mul128(a: u64, b: u64) -> (u64, u64) {
  let r = u128::from(a) * u128::from(b);
  (r as u64, (r >> 64) as u64)
}

/// 乘 xor 混合 (rapid_mix): 128 积高低半异或。
#[inline]
pub fn mix(a: u64, b: u64) -> u64 {
  let (lo, hi) = mul128(a, b);
  lo ^ hi
}

/// rapidhash 主体; len 为产出侧字节数 (压缩时 = u16 单元数)。
/// 数据流与指针隔离: a/b/seed/secret 与 see1/see2 仅受输入数据影响。
pub fn rapidhash<C, R: HashReader<C>>(p: &[C], len: usize, mut seed: u64, secret: &[u64; 3]) -> u64 {
  seed ^= mix(seed ^ secret[0], secret[1]) ^ len as u64;
  let (mut a, mut b);
  if len <= 16 {
    if len >= 4 {
      // 首尾 32 bit (可重叠)
      let last = len - 4;
      a = (R::read32(p) << 32) | R::read32(&p[last..]);
      // delta = (len >= 8) ? 4 : 0
      let delta = (len & 24) >> (len >> 3);
      b = (R::read32(&p[delta..]) << 32) | R::read32(&p[last - delta..]);
    } else if len > 0 {
      a = R::read_small(p, len);
      b = 0;
    } else {
      a = 0;
      b = 0;
    }
  } else {
    let mut i = len;
    let mut off = 0usize;
    if i > 48 {
      let mut see1 = seed;
      let mut see2 = seed;
      loop {
        seed = mix(R::read64(&p[off..]) ^ secret[0], R::read64(&p[off + 8..]) ^ seed);
        see1 = mix(R::read64(&p[off + 16..]) ^ secret[1], R::read64(&p[off + 24..]) ^ see1);
        see2 = mix(R::read64(&p[off + 32..]) ^ secret[2], R::read64(&p[off + 40..]) ^ see2);
        off += 48;
        i -= 48;
        if i < 48 {
          break;
        }
      }
      seed ^= see1 ^ see2;
    }
    if i > 16 {
      seed = mix(R::read64(&p[off..]) ^ secret[2], R::read64(&p[off + 8..]) ^ seed ^ secret[1]);
      if i > 32 {
        seed = mix(R::read64(&p[off + 16..]) ^ secret[2], R::read64(&p[off + 24..]) ^ seed);
      }
    }
    // 末尾 2×64 bit
    a = R::read64(&p[off + i - 16..]);
    b = R::read64(&p[off + i - 8..]);
  }
  a ^= secret[1];
  b ^= seed;
  let (lo, hi) = mul128(a, b);
  mix(lo ^ secret[0] ^ len as u64, hi ^ secret[1])
}
`;

const hashSeed = `${banner("src/strings/string-hasher.h.js", "src/numbers/hash-seed.h + hash-seed.cc + hash-seed-inl.h + third_party/rapidhash-v8/secret.h", "上游 v8-numbers 未开发, 越界落地; 归并点登记 crate README")}
//! HashSeed — 字符串哈希种子视图 (seed + rapidhash secrets)。
//! 默认配置 (V8_USE_DEFAULT_HASHER_SECRET=true): secrets 固定 RAPIDHASH_DEFAULT_SECRET。

/// rapidhash 默认 secret (平衡字节 / 汉明距 32 / 素数)。
pub const RAPIDHASH_DEFAULT_SECRET: [u64; 3] = [
  ${secrets.map((s) => "0x" + s.replace("0x", "").toLowerCase()).join(", ")},
];

/// 种子数据 (HashSeed::Data)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashSeedData {
  /// 元种子 (--hash-seed, 0 = 启动时生成)。
  pub seed: u64,
  /// rapidhash secrets (默认配置下即默认 secret)。
  pub secrets: [u64; 3],
}

impl HashSeedData {
  /// 默认种子 (HashSeed::Default)。
  pub const DEFAULT: Self = Self { seed: 0, secrets: RAPIDHASH_DEFAULT_SECRET };

  /// 由元种子派生 secrets (rapidhash_make_secret, 非 V8_USE_DEFAULT_HASHER_SECRET 路径)。
  pub fn from_seed(seed: u64) -> Self {
    Self { seed, secrets: make_secret(seed) }
  }
}

/// 平衡字节候选 (每字节恰 4 bit 置位)。
static BALANCED_BYTES: [u8; ${cTable.length}] = [${cTable.join(", ")}];

/// wyrand 单步。
#[inline]
fn wyrand(seed: &mut u64) -> u64 {
  *seed = seed.wrapping_add(RAPIDHASH_DEFAULT_SECRET[0]);
  crate::rapidhash::mix(*seed, *seed ^ RAPIDHASH_DEFAULT_SECRET[1])
}

/// 64 位模乘 (mul_mod)。
fn mul_mod(mut a: u64, mut b: u64, m: u64) -> u64 {
  let mut r = 0u64;
  while b != 0 {
    if b & 1 != 0 {
      r = (r.wrapping_add(a)) % m;
    }
    b >>= 1;
    if b != 0 {
      a = (a.wrapping_add(a)) % m;
    }
  }
  r
}

/// 快速幂模 (pow_mod)。
fn pow_mod(mut a: u64, mut b: u64, m: u64) -> u64 {
  let mut r = 1u64;
  while b != 0 {
    if b & 1 != 0 {
      r = mul_mod(r, a, m);
    }
    b >>= 1;
    if b != 0 {
      a = mul_mod(a, a, m);
    }
  }
  r
}

/// 强伪素数测试 (sprp)。
fn sprp(n: u64, a: u64) -> bool {
  let mut d = n - 1;
  let mut s = 0u8;
  while d & 0xff == 0 {
    d >>= 8;
    s += 8;
  }
  if d & 0xf == 0 {
    d >>= 4;
    s += 4;
  }
  if d & 0x3 == 0 {
    d >>= 2;
    s += 2;
  }
  if d & 0x1 == 0 {
    d >>= 1;
    s += 1;
  }
  let mut b = pow_mod(a, d, n);
  if b == 1 || b == n - 1 {
    return true;
  }
  for _ in 1..s {
    b = mul_mod(b, b, n);
    if b <= 1 {
      return false;
    }
    if b == n - 1 {
      return true;
    }
  }
  false
}

/// Miller-Rabin 素性测试 (is_prime)。
fn is_prime(n: u64) -> bool {
  if n < 2 || n & 1 == 0 {
    return false;
  }
  if n < 4 {
    return true;
  }
  if !sprp(n, 2) {
    return false;
  }
  if n < 2047 {
    return true;
  }
  [3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37].iter().all(|&a| sprp(n, a))
}

/// rapidhash_make_secret: 拒绝采样产出 3 个 (奇数 / 两两汉明距 32 / 素数) secret。
pub fn make_secret(mut seed: u64) -> [u64; 3] {
  let mut secret = [0u64; 3];
  for i in 0..3 {
    loop {
      let mut ok = true;
      secret[i] = 0;
      for j in (0..64).step_by(8) {
        secret[i] |= u64::from(BALANCED_BYTES[(wyrand(&mut seed) % BALANCED_BYTES.len() as u64) as usize]) << j;
      }
      if secret[i] % 2 == 0 {
        continue;
      }
      for prev in &secret[..i] {
        if (*prev ^ secret[i]).count_ones() != 32 {
          ok = false;
          break;
        }
      }
      if !ok {
        continue;
      }
      if !is_prime(secret[i]) {
        continue;
      }
      break;
    }
  }
  secret
}
`;

const stringHasher = `${banner("src/strings/string-hasher.h.js", "src/strings/string-hasher.h + string-hasher.cc + string-hasher-inl.h", "hash 算法语义绑定 1:1; SIMD 位点标量化 (IsOnly8BitSIMD/HashConvertingTo8Bit)")}
//! 字符串哈希: array index 哈希 / rapidhash 全串哈希 / 增量 RunningStringHasher。
//! V8_ENABLE_SEEDED_ARRAY_INDEX_HASH 默认关闭, 未承接 (见 README 遗留清单)。

use crate::hash_field::{
  contains_cached_array_index, create_hash_field_value, HashFieldType, ARRAY_INDEX_LENGTH_SHIFT,
  ARRAY_INDEX_VALUE_MASK, ARRAY_INDEX_VALUE_SHIFT, MAX_ARRAY_INDEX, MAX_ARRAY_INDEX_SIZE,
  MAX_CACHED_ARRAY_INDEX_LENGTH, MAX_HASH_CALC_LENGTH, MAX_INTEGER_INDEX_SIZE,
};
use crate::hash_seed::HashSeedData;
use crate::rapidhash::{rapidhash, ConvertTo8BitReader, PlainReader};

/// 哈希为 0 时以 27 代替 (kZeroHash; 0 保留给内部属性)。
pub const ZERO_HASH: u32 = 27;

/// 限制到支持位宽并兜底 kZeroHash (ConvertRawHashToUsableHash)。
#[inline]
fn convert_raw_to_usable(raw_hash: u64) -> u32 {
  let hash = (raw_hash & u64::from(crate::hash_field::HASH_BITS_MAX)) as u32;
  if hash == 0 { ZERO_HASH } else { hash }
}

/// 简单增量哈希器 (RunningStringHasher) — 允许逐字符特殊处理。
#[derive(Debug, Clone)]
pub struct RunningStringHasher {
  running_hash: u32,
}

impl RunningStringHasher {
  #[must_use]
  pub fn new(seed: u32) -> Self {
    Self { running_hash: seed }
  }

  /// 追加一个字符。
  #[inline]
  pub fn add_character(&mut self, c: u16) {
    self.running_hash = self.running_hash.wrapping_add(u32::from(c));
    self.running_hash = self.running_hash.wrapping_add(self.running_hash << 10);
    self.running_hash ^= self.running_hash >> 6;
  }

  /// 终结并产出可用哈希。
  #[inline]
  pub fn finalize(&mut self) -> u32 {
    self.running_hash = self.running_hash.wrapping_add(self.running_hash << 3);
    self.running_hash ^= self.running_hash >> 11;
    self.running_hash = self.running_hash.wrapping_add(self.running_hash << 15);
    convert_raw_to_usable(u64::from(self.running_hash))
  }
}

/// u16 串是否全在 Latin-1 (≤0xFF) — IsOnly8BitSIMD 标量化。
#[must_use]
pub fn is_only_8_bit(chars: &[u16]) -> bool {
  chars.iter().all(|&c| c <= 0xFF)
}

/// 索引解析结果 (IndexParseResult)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndexParseResult {
  Success,
  NonIndex,
  Overflow,
}

/// 数字符 → 数字值; 非数字返回 >9 (与 c - '0' 的回绕语义一致)。
#[inline]
fn digit_val(c: u32) -> u32 {
  c.wrapping_sub(u32::from(b'0'))
}

/// TryParseArrayIndex: 无前导零 (除 "0") 的数字串 → index 值。
/// 返回 (结果, 已消费位置 i, 解析值)。
fn try_parse_array_index<C: Copy + Into<u32>>(chars: &[C], length: usize) -> (IndexParseResult, usize, u64) {
  let mut index = u64::from(digit_val(chars[0].into()));
  let mut i = 1usize;
  if index > 9 {
    return (IndexParseResult::NonIndex, i, index);
  }
  if index == 0 {
    if length > 1 {
      return (IndexParseResult::NonIndex, i, index);
    }
    return (IndexParseResult::Success, i, index);
  }
  if length > MAX_ARRAY_INDEX_SIZE as usize {
    return (IndexParseResult::Overflow, i, index);
  }
  while i < length {
    let val = digit_val(chars[i].into());
    if val > 9 {
      return (IndexParseResult::NonIndex, i, index);
    }
    index = 10 * index + u64::from(val);
    i += 1;
  }
  // 64 位: index 用 u64 承载, 仅需上限比较
  if index > u64::from(MAX_ARRAY_INDEX) {
    return (IndexParseResult::Overflow, i, index);
  }
  (IndexParseResult::Success, i, index)
}

/// TryParseIntegerIndex: array index 溢出后的整数索引解析 (≤ 2^53-1)。
fn try_parse_integer_index<C: Copy + Into<u32>>(chars: &[C], length: usize, mut i: usize, mut index: u64) -> IndexParseResult {
  const MAX_SAFE_INTEGER_UINT64: u64 = (1u64 << 53) - 1;
  while i < length {
    let val = digit_val(chars[i].into());
    if val > 9 {
      return IndexParseResult::NonIndex;
    }
    index = 10 * index + u64::from(val);
    i += 1;
  }
  if index > MAX_SAFE_INTEGER_UINT64 {
    return IndexParseResult::Overflow;
  }
  IndexParseResult::Success
}

/// MakeArrayIndexHash (raw 编码): value << VALUE_SHIFT | length << LENGTH_SHIFT。
#[inline]
#[must_use]
pub fn make_array_index_hash_raw(value: u32, length: u32) -> u32 {
  value << ARRAY_INDEX_VALUE_SHIFT | length << ARRAY_INDEX_LENGTH_SHIFT
}

/// DecodeArrayIndexFromHashField (raw 解码)。
#[inline]
#[must_use]
pub fn decode_array_index_from_hash_field(raw_hash_field: u32) -> u32 {
  (raw_hash_field >> ARRAY_INDEX_VALUE_SHIFT) & ARRAY_INDEX_VALUE_MASK
}

/// GetTrivialHash: 长串 (>kMaxHashCalcLength) 仅按长度取哈希。
#[inline]
#[must_use]
pub fn get_trivial_hash(length: u32) -> u32 {
  debug_assert!(length > MAX_HASH_CALC_LENGTH);
  create_hash_field_value(length, HashFieldType::Hash)
}

/// GetUsableRapidHash — 单字节内容。
fn usable_rapid_one_byte(chars: &[u8], seed: &HashSeedData) -> u32 {
  convert_raw_to_usable(rapidhash::<u8, PlainReader>(chars, chars.len(), seed.seed, &seed.secrets))
}

/// GetUsableRapidHash — 双字节内容; 返回 (hash, 是否全 Latin-1)。
fn usable_rapid_two_byte(chars: &[u16], seed: &HashSeedData) -> (u32, bool) {
  let one_byte = is_only_8_bit(chars);
  let h = if one_byte {
    // Latin-1 内容压缩到 8 bit: 与单字节串哈希一致
    rapidhash::<u16, ConvertTo8BitReader>(chars, chars.len(), seed.seed, &seed.secrets)
  } else {
    // SAFETY: chars 是已初始化的有效 u16 切片, u8 对齐要求 (1) 严格弱于 u16 对齐 (2),
    // 且长度 chars.len() * 2 恰覆盖全部底层字节; 小端目标内存序与 UTF-16 字节序天然一致。
    #[cfg(target_endian = "little")]
    let bytes: &[u8] = unsafe {
      core::slice::from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len() * 2)
    };
    #[cfg(not(target_endian = "little"))]
    let bytes_vec = chars.iter().flat_map(|&c| c.to_le_bytes()).collect::<Vec<u8>>();
    #[cfg(not(target_endian = "little"))]
    let bytes = &bytes_vec[..];
    rapidhash::<u8, PlainReader>(bytes, bytes.len(), seed.seed, &seed.secrets)
  };
  (convert_raw_to_usable(h), one_byte)
}

/// HashSequentialString — 单字节内容。
/// 返回 raw_hash_field; out_one_byte_content 在真正扫描内容的路径上被置位。
pub fn hash_sequential_one_byte(chars: &[u8], seed: &HashSeedData, mut out_one_byte_content: Option<&mut bool>) -> u32 {
  let length = chars.len();
  if length >= 1 && length <= MAX_INTEGER_INDEX_SIZE as usize {
    match try_parse_array_index(chars, length) {
      (IndexParseResult::Success, _, index) => {
        if let Some(out) = out_one_byte_content.as_deref_mut() {
          *out = true;
        }
        return make_array_index_hash_raw(index as u32, length as u32);
      }
      (IndexParseResult::Overflow, i, index)
        if try_parse_integer_index(chars, length, i, index) == IndexParseResult::Success =>
      {
        let hash = create_hash_field_value(usable_rapid_one_byte(chars, seed), HashFieldType::IntegerIndex);
        if let Some(out) = out_one_byte_content {
          *out = true;
        }
        return fix_cached_index_lookalike(hash);
      }
      _ => {}
    }
  } else if length > MAX_HASH_CALC_LENGTH as usize {
    return get_trivial_hash(length as u32);
  }
  // 非索引哈希
  if let Some(out) = out_one_byte_content {
    *out = true;
  }
  create_hash_field_value(usable_rapid_one_byte(chars, seed), HashFieldType::Hash)
}

/// HashSequentialString — 双字节内容。
pub fn hash_sequential_two_byte(chars: &[u16], seed: &HashSeedData, mut out_one_byte_content: Option<&mut bool>) -> u32 {
  let length = chars.len();
  if length >= 1 && length <= MAX_INTEGER_INDEX_SIZE as usize {
    match try_parse_array_index(chars, length) {
      (IndexParseResult::Success, _, index) => {
        // 数字串必然全 Latin-1
        if let Some(out) = out_one_byte_content.as_deref_mut() {
          *out = true;
        }
        return make_array_index_hash_raw(index as u32, length as u32);
      }
      (IndexParseResult::Overflow, i, index)
        if try_parse_integer_index(chars, length, i, index) == IndexParseResult::Success =>
      {
        let (h, one_byte) = usable_rapid_two_byte(chars, seed);
        if let Some(out) = out_one_byte_content {
          *out = one_byte;
        }
        return fix_cached_index_lookalike(create_hash_field_value(h, HashFieldType::IntegerIndex));
      }
      _ => {}
    }
  } else if length > MAX_HASH_CALC_LENGTH as usize {
    return get_trivial_hash(length as u32);
  }
  let (h, one_byte) = usable_rapid_two_byte(chars, seed);
  if let Some(out) = out_one_byte_content {
    *out = one_byte;
  }
  create_hash_field_value(h, HashFieldType::Hash)
}

/// 整数索引哈希碰巧形如缓存 array index 时, 置长度位规避。
#[inline]
fn fix_cached_index_lookalike(mut hash: u32) -> u32 {
  if contains_cached_array_index(hash) {
    hash |= (MAX_CACHED_ARRAY_INDEX_LENGTH + 1) << ARRAY_INDEX_LENGTH_SHIFT;
  }
  debug_assert!(!contains_cached_array_index(hash));
  hash
}
`;

writeSrc("v8-strings", "hash_field.rs", hashField);
writeSrc("v8-strings", "rapidhash.rs", rapidhash);
writeSrc("v8-strings", "hash_seed.rs", hashSeed);
writeSrc("v8-strings", "string_hasher.rs", stringHasher);
console.log("written: hash_field.rs, rapidhash.rs, hash_seed.rs, string_hasher.rs");
console.log(`secrets: ${secrets.join(", ")}; balanced bytes: ${cTable.length}`);
