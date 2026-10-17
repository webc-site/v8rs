// 由 tool/src/utils/consts.js 从 v8/src/base/hashing.h;third_party/rapidhash-v8/secret.h;src/common/globals.h 生成; 修订就地改, 复跑覆盖

//! 通用工具函数集, 移植自 v8/src/utils/utils.{h,cc,inl}。
//!
//! 含 base/hashing 的整数散列原语 (hash64/hash32/hash_combine/Hasher) 就地副本,
//! 算法与 v8 base/hashing.h + rapidhash 保持位级一致 (Map/Set 哈希需跨实现一致)。

use std::cmp::Ordering;

/// rapidhash "mum" 混合器 secret1 (kRapidhashSecret1)
const RAPIDHASH_SECRET1: u64 = 0x2d358dccaa6c78a5;
/// rapidhash "mum" 混合器 secret2 (kRapidhashSecret2)
const RAPIDHASH_SECRET2: u64 = 0x8bb84b93962eacc9;
/// hash_combine 64 位乘子 (MurmurHash 尾部混合)
const HASH_COMBINE_M: u64 = 0xC6A4A7935BD1E995;
/// hash_combine 右移位数
const HASH_COMBINE_R: u32 = 47;
/// Smi 整数散列掩码 (kSmiHashMask, kSmiValueSize-1 位 → 非负 Smi 载荷)
pub const SMI_HASH_MASK: u32 = (1u32 << 30) - 1;

/// 32 位循环右移 (bits::RotateRight32)
#[inline]
pub const fn ror32(v: u32, s: u32) -> u32 {
  v.rotate_right(s)
}

/// rapidhash "mum" 混合器: 128 位乘积高低半异或 (rapid_mix, rapidhash.h:204)
#[inline]
const fn rapid_mix(a: u64, b: u64) -> u64 {
  let p = (a as u128) * (b as u128);
  (p as u64) ^ ((p >> 64) as u64)
}

/// 规范整数散列 hash64: rapid_mix((key^s1), (key^s2)) (hashing.h:179, 64 位支)
#[inline]
pub const fn hash64(key: u64) -> u64 {
  rapid_mix(key ^ RAPIDHASH_SECRET1, key ^ RAPIDHASH_SECRET2)
}

/// 规范整数散列 hash32: hash64 截断低 32 位 (hashing.h:196, 64 位支)
#[inline]
pub const fn hash32(key: u32) -> u32 {
  hash64(key as u64) as u32
}

/// 双值合并散列 (MurmurHash 尾部混合, hashing.h:80, 64 位支)
#[inline]
pub const fn hash_combine(seed: usize, hash: usize) -> usize {
  let m = HASH_COMBINE_M as usize;
  let mut h = hash.wrapping_mul(m);
  h ^= h >> HASH_COMBINE_R;
  h = h.wrapping_mul(m);
  (seed ^ h).wrapping_mul(m)
}

/// 多字段增量散列器 (base::Hasher, hashing.h:112)
///
/// 语义与 C++ 逐一对应: 有符号整型 bit_cast 无符号后散列,
/// 单字节类型 (bool/u8/u16) 直接取值不散列 (hashing.h:218-221 trivial 分支)。
#[derive(Debug, Clone, Copy)]
pub struct Hasher {
  hash: usize,
}

impl Default for Hasher {
  fn default() -> Self {
    Self::new()
  }
}

impl Hasher {
  /// 默认零种子构造
  pub const fn new() -> Self {
    Self { hash: 0 }
  }

  /// 指定种子构造
  pub const fn with_seed(seed: usize) -> Self {
    Self { hash: seed }
  }

  /// 当前散列值
  pub const fn hash(&self) -> usize {
    self.hash
  }

  /// 直接合并一个既有散列值 (AddHash)
  pub fn add_hash(&mut self, other: usize) -> &mut Self {
    self.hash = hash_combine(self.hash, other);
    self
  }

  /// 混入 i32: bit_cast 到 u32 后 hash32 (hashing.h:236 signed 分支)
  pub fn add_i32(&mut self, v: i32) -> &mut Self {
    self.add_hash(hash32(v as u32) as usize)
  }

  /// 混入 u64 (hash_value(unsigned long long))
  pub fn add_u64(&mut self, v: u64) -> &mut Self {
    self.add_hash(hash64(v) as usize)
  }

  /// 逐字节混入字节串 (AddRange + OneByteVector; u8 走 trivial 分支直接取值)
  pub fn add_bytes(&mut self, bytes: &[u8]) -> &mut Self {
    for b in bytes {
      self.add_hash(*b as usize);
    }
    self
  }

  /// `Hasher::Combine(int...)` (hashing.h:149-155): 依次 Add 后取散列。
  pub fn combine_i32(vals: &[i32]) -> usize {
    let mut h = Self::new();
    for &v in vals {
      h.add_i32(v);
    }
    h.hash()
  }
}

// ---- Smi 散列族 (utils.h:274-306) ----

/// `SmiHash32` (utils.h:278): hash32 结果收 kSmiHashMask → 非负 Smi 载荷。
#[inline]
#[must_use]
pub const fn smi_hash32(key: u32) -> u32 {
  hash32(key) & SMI_HASH_MASK
}

/// `SmiHash64` (utils.h:285): hash64 截 32 位再收 kSmiHashMask。
#[inline]
#[must_use]
pub const fn smi_hash64(key: u64) -> u32 {
  (hash64(key) as u32) & SMI_HASH_MASK
}

/// `ComputeSeededHash` (utils.h:290): 非 SIPHASH 路径 = SmiHash64(key ^ seed)。
#[inline]
#[must_use]
pub const fn compute_seeded_hash(key: u32, seed: u64) -> u32 {
  smi_hash64(key as u64 ^ seed)
}

/// `ComputeAddressHash` (utils.h:304): 低 32 位截断后 SmiHash32。
#[inline]
#[must_use]
pub const fn compute_address_hash(address: usize) -> u32 {
  smi_hash32((address & 0xFFFF_FFFF) as u32)
}

// ---- 移位 / JS 数值序 (utils.h:58-95) ----

/// `ArithmeticShiftRight` (utils.h:62): 算术右移 (Rust `>>` 对有符号即算术)。
#[inline]
#[must_use]
pub const fn arithmetic_shift_right(x: i64, shift: u32) -> i64 {
  x >> shift
}

/// `JSMax` (utils.h:77): NaN 优先传播; -0 严格小于 +0。
#[inline]
#[must_use]
pub fn js_max(x: f64, y: f64) -> f64 {
  if x.is_nan() {
    return x;
  }
  if y.is_nan() {
    return y;
  }
  if !x.is_sign_negative() & y.is_sign_negative() {
    return x;
  }
  if x > y { x } else { y }
}

/// `JSMin` (utils.h:86): NaN 优先传播; -0 严格小于 +0。
#[inline]
#[must_use]
pub fn js_min(x: f64, y: f64) -> f64 {
  if x.is_nan() {
    return x;
  }
  if y.is_nan() {
    return y;
  }
  if !x.is_sign_negative() & y.is_sign_negative() {
    return y;
  }
  if x > y { y } else { x }
}

// ---- 饱和加减 (utils.h SaturatingAdd/SaturatingSub 模板族) ----

macro_rules! impl_saturating {
  ($add:ident, $sub:ident, $t:ty) => {
    /// SaturatingAdd: 溢出收敛到类型边界 (utils.h 模板族)。
    #[inline]
    #[must_use]
    pub const fn $add(x: $t, y: $t) -> $t {
      x.saturating_add(y)
    }

    /// SaturatingSub: 下溢收敛到类型边界 (utils.h 模板族)。
    #[inline]
    #[must_use]
    pub const fn $sub(x: $t, y: $t) -> $t {
      x.saturating_sub(y)
    }
  };
}

impl_saturating!(saturate_add_i8, saturate_sub_i8, i8);
impl_saturating!(saturate_add_i16, saturate_sub_i16, i16);
impl_saturating!(saturate_add_i32, saturate_sub_i32, i32);
impl_saturating!(saturate_add_i64, saturate_sub_i64, i64);
impl_saturating!(saturate_add_u8, saturate_sub_u8, u8);
impl_saturating!(saturate_add_u16, saturate_sub_u16, u16);
impl_saturating!(saturate_add_u32, saturate_sub_u32, u32);
impl_saturating!(saturate_add_u64, saturate_sub_u64, u64);

// ---- 位域提取与宽度谓词 (utils.h:656-662 / is_intn 展开) ----

/// `unsigned_bitextract_32` (utils.h:657): [lsb, msb] 闭区间位提取。
#[inline]
#[must_use]
pub const fn unsigned_bitextract_32(msb: u32, lsb: u32, x: u32) -> u32 {
  (x >> lsb) & ((1 << (1 + msb - lsb)) - 1)
}

/// `signed_bitextract_32` (utils.h:662): 提取后按符号位扩展。
#[inline]
#[must_use]
pub const fn signed_bitextract_32(msb: u32, lsb: u32, x: u32) -> i32 {
  ((x << (31 - msb)) as i32) >> (lsb + 31 - msb)
}

/// `IsInt8` (IsIntN<8> 展开, utils.h)。
#[inline]
#[must_use]
pub const fn is_int8(v: i64) -> bool {
  v >= -128 && v <= 127
}

/// `IsInt16` (IsIntN<16> 展开)。
#[inline]
#[must_use]
pub const fn is_int16(v: i64) -> bool {
  v >= -32_768 && v <= 32_767
}

/// `IsUint32` (utils.h, int64_t 重载): 非负且不超 kMaxUInt32。
#[inline]
#[must_use]
pub const fn is_uint32(v: i64) -> bool {
  v >= 0 && v <= u32::MAX as i64
}

/// `TenToThe` (utils.h:646): 10^exponent。
#[inline]
#[must_use]
pub const fn ten_to_the(exponent: u32) -> u64 {
  10u64.pow(exponent)
}

/// `RoundUpToPageSize` (utils.h:919-932): 字节数取整到页, 返回页数;
/// 超出 max_allowed_byte_length 返回 None。
#[inline]
#[must_use]
pub const fn round_up_to_page_size(
  byte_length: usize,
  page_size: usize,
  max_allowed_byte_length: usize,
) -> Option<usize> {
  if byte_length > max_allowed_byte_length {
    return None;
  }
  // RoundUp(byte_length, page_size); 加法先查溢出 (原版注释: 接近 size_t 上限时算术失效)
  let Some(rounded) = byte_length.checked_add(page_size - 1) else {
    return None;
  };
  let bytes_wanted = rounded / page_size * page_size;
  if bytes_wanted > max_allowed_byte_length {
    return None;
  }
  Some(bytes_wanted / page_size)
}

// ---- 字符序列比较 (utils.h:621-642) ----

/// `CompareChars` (utils.h:635): 同宽码元序列字典序 (memcmp 语义)。
#[must_use]
pub fn compare_chars(lhs: &[u16], rhs: &[u16]) -> Ordering {
  for (&l, &r) in lhs.iter().zip(rhs.iter()) {
    if l != r {
      return l.cmp(&r);
    }
  }
  lhs.len().cmp(&rhs.len())
}

/// `CompareCharsEqual` 单/双字节形态 (utils.h:625): 逐码元相等。
#[must_use]
pub fn compare_one_and_two_byte_chars(lhs: &[u8], rhs: &[u16]) -> bool {
  lhs.len() == rhs.len() && lhs.iter().zip(rhs.iter()).all(|(&l, &r)| u16::from(l) == r)
}

// ---- DoubleToBoolean (utils.cc:203) ----

/// NaN 与 ±0 为假, 其余为真 (位域检查的等价谓词)。
#[inline]
#[must_use]
pub const fn double_to_boolean(d: f64) -> bool {
  !d.is_nan() && d != 0.0
}

// ---- PassesFilter (utils.cc:232-265) ----

/// 函数名过滤 (utils.cc 注释):
/// `"*"` 全部; `"-"` 除顶层; `"-name"` 除 name; `""` 仅顶层;
/// `"name"` 仅 name; `"name*"` name 前缀; `"~"` 无。
#[must_use]
pub fn passes_filter(name: &str, filter: &str) -> bool {
  let (n, f) = (name.as_bytes(), filter.as_bytes());
  if f.is_empty() {
    return n.is_empty();
  }
  let mut i = 0;
  let mut positive = true;
  if f[0] == b'-' {
    i = 1;
    positive = false;
  }
  if i == f.len() {
    return !n.is_empty();
  }
  if f[i] == b'*' {
    return positive;
  }
  if f[i] == b'~' {
    return !positive;
  }

  let prefix_match = f[f.len() - 1] == b'*';
  let mut min_match_length = f.len();
  if !positive {
    min_match_length -= 1; // 前导 '-'
  }
  if prefix_match {
    min_match_length -= 1; // 尾 '*'
  }
  if n.len() < min_match_length {
    return !positive;
  }

  // std::mismatch: 公共前缀长度
  let common = n
    .iter()
    .zip(f[i..].iter())
    .take_while(|(a, b)| a == b)
    .count();
  if i + common == f.len() {
    if common == n.len() {
      return positive; // 全串相等
    }
    return !positive; // name 更长
  }
  if f[i + common] == b'*' {
    return positive; // 匹配到通配
  }
  !positive // 不匹配
}

// ---- StringToIndex (utils-inl.h:59 + utils.h:854) ----

/// 索引转换模式 (utils.h:854)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToIndexMode {
  /// kToArrayIndex: 数组索引, 上限 2^32-2 (kMaxUInt32-1)。
  ToArrayIndex,
  /// kToIntegerIndex: 整数索引, 上限 2^53-1 (kMaxSafeInteger)。
  ToIntegerIndex,
}

/// kMaxSafeIntegerUint64 (2^53-1)。
pub const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;
/// TryAddArrayIndexChar 的上限基准 (utils-inl.h:45 的 429496729)。
const MAX_ARRAY_INDEX_STEP: u64 = 429_496_729;

/// `StringToIndex` (utils-inl.h:59): 十进制数字符流转索引;
/// 前导 '0' 仅 "0" 合法; 非数字/超上限返回 None。
pub fn string_to_index(stream: &mut impl Iterator<Item = u16>, mode: ToIndexMode) -> Option<u64> {
  const ZERO: u16 = b'0' as u16;
  const NINE: u16 = b'9' as u16;
  let digit = |c: u16| (ZERO..=NINE).contains(&c);

  let ch = stream.next()?;
  if ch == ZERO {
    // '0' 开头: 整串必须仅为 "0" (utils-inl.h:65-68)
    return stream.next().is_none().then_some(0);
  }
  if !digit(ch) {
    return None;
  }
  let mut result = u64::from(ch - ZERO);
  for c in stream {
    if !digit(c) {
      return None;
    }
    let d = u64::from(c - ZERO);
    match mode {
      ToIndexMode::ToArrayIndex => {
        // TryAddArrayIndexChar (utils-inl.h:45): (d+3)>>3 无分支上界
        if result > MAX_ARRAY_INDEX_STEP - ((d + 3) >> 3) {
          return None;
        }
        result = result * 10 + d;
      }
      ToIndexMode::ToIntegerIndex => {
        // TryAddIntegerIndexChar (utils-inl.h:50): 溢出与超 kMaxSafeInteger 双拒
        result = result.checked_mul(10)?.checked_add(d)?;
        if result > MAX_SAFE_INTEGER {
          return None;
        }
      }
    }
  }
  Some(result)
}
