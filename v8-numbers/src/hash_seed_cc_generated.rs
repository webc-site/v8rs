// 由 tool/src/numbers/hash-seed.h.js 从 v8/src/numbers/hash-seed.{h,cc} 生成; 修订就地改, 复跑覆盖
// 默认哈希种子数据与派生算法

// 注: 种子布局常量由 hash_seed.rs (tool/src/numbers/hash-seed.cc.js) 覆盖。

/// 原 RAPIDHASH_DEFAULT_SECRET[3] 展开
pub const RAPIDHASH_DEFAULT_SECRET: [u64; 3] = [
  0x2D358DCCAA6C78A5_u64,
  0x8BB84B93962EACC9_u64,
  0x4B33A62ED433D4A3_u64,
];

/// 数组索引哈希截断位宽 (原 Name::kArrayIndexValueBits, src/objects/name.h)
pub const ARRAY_INDEX_VALUE_BITS: u32 = 24;
/// 截断掩码 (原 Name::kArrayIndexValueMask)
pub const ARRAY_INDEX_VALUE_MASK: u32 = (1 << 24) - 1;

/// 原 kDefaultSeed (V8_ENABLE_SEEDED_ARRAY_INDEX_HASH 分支, 含派生乘数):
/// m = truncate(secret) | 1; m_inv = modular_inverse(m) mod 2^26
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashSeedData {
  pub seed: u64,
  pub secrets: [u64; 3],
  pub m1: u32,
  pub m1_inv: u32,
  pub m2: u32,
  pub m2_inv: u32,
  pub m3: u32,
  pub m3_inv: u32,
}

/// 原版 kDefaultSeed 终态 (seed = 0, secrets 取默认三元组)
pub const DEFAULT_SEED: HashSeedData = HashSeedData {
  seed: 0,
  secrets: RAPIDHASH_DEFAULT_SECRET,
  m1: 0x6C78A5,
  m1_inv: 0x8CBEAF2D,
  m2: 0x2EACC9,
  m2_inv: 0x4E892D79,
  m3: 0x33D4A3,
  m3_inv: 0x47237F0B,
};

/// 原 modular_inverse (Newton 迭代, 模 2^32)
#[inline]
#[must_use]
pub const fn modular_inverse(m: u32) -> u32 {
  let mut x = (3u32.wrapping_mul(m)) ^ 2;
  x = x.wrapping_mul(2u32.wrapping_sub(m.wrapping_mul(x)));
  x = x.wrapping_mul(2u32.wrapping_sub(m.wrapping_mul(x)));
  x = x.wrapping_mul(2u32.wrapping_sub(m.wrapping_mul(x)));
  x.wrapping_mul(2u32.wrapping_sub(m.wrapping_mul(x)))
}

/// 原 is_modular_inverse 校验
#[inline]
#[must_use]
pub const fn is_modular_inverse(m: u32, m_inv: u32) -> bool {
  (m.wrapping_mul(m_inv) & ARRAY_INDEX_VALUE_MASK) == 1
}

// 原版 HashSeed::InitializeRoots 语义面 (随机种子/标志覆盖/根写入) 登记跳过。

// —— 提取不变式断言 ——
const _: () = {
  assert!(RAPIDHASH_DEFAULT_SECRET[0] == 0x2d358dccaa6c78a5);
  assert!(DEFAULT_SEED.m1 & 1 == 1);
  assert!(is_modular_inverse(DEFAULT_SEED.m1, DEFAULT_SEED.m1_inv));
  assert!(is_modular_inverse(DEFAULT_SEED.m2, DEFAULT_SEED.m2_inv));
  assert!(is_modular_inverse(DEFAULT_SEED.m3, DEFAULT_SEED.m3_inv));
};
