// 由 tool/src/strings/string-hasher.h.js 从 v8/src/numbers/hash-seed.h + hash-seed.cc + hash-seed-inl.h + third_party/rapidhash-v8/secret.h 生成; 修订就地改, 复跑覆盖
// 上游 v8-numbers 未开发, 越界落地; 归并点登记 crate README

//! HashSeed — 字符串哈希种子视图 (seed + rapidhash secrets)。
//! 默认配置 (V8_USE_DEFAULT_HASHER_SECRET=true): secrets 固定 RAPIDHASH_DEFAULT_SECRET。

use crate::rapidhash::mix;

/// rapidhash 默认 secret (平衡字节 / 汉明距 32 / 素数)。
pub const RAPIDHASH_DEFAULT_SECRET: [u64; 3] =
  [0x2d358dccaa6c78a5, 0x8bb84b93962eacc9, 0x4b33a62ed433d4a3];

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
  pub const DEFAULT: Self = Self {
    seed: 0,
    secrets: RAPIDHASH_DEFAULT_SECRET,
  };

  /// 由元种子派生 secrets (rapidhash_make_secret, 非 V8_USE_DEFAULT_HASHER_SECRET 路径)。
  pub fn from_seed(seed: u64) -> Self {
    Self {
      seed,
      secrets: make_secret(seed),
    }
  }
}

/// 平衡字节候选 (每字节恰 4 bit 置位)。
static BALANCED_BYTES: [u8; 70] = [
  15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 86, 89, 90,
  92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 147, 149, 150, 153, 154,
  156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 197, 198, 201, 202, 204, 209, 210,
  212, 216, 225, 226, 228, 232, 240,
];

/// wyrand 单步。
#[inline]
fn wyrand(seed: &mut u64) -> u64 {
  *seed = seed.wrapping_add(RAPIDHASH_DEFAULT_SECRET[0]);
  mix(*seed, *seed ^ RAPIDHASH_DEFAULT_SECRET[1])
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
  [3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    .iter()
    .all(|&a| sprp(n, a))
}

/// rapidhash_make_secret: 拒绝采样产出 3 个 (奇数 / 两两汉明距 32 / 素数) secret。
pub fn make_secret(mut seed: u64) -> [u64; 3] {
  let mut secret = [0u64; 3];
  for i in 0..3 {
    loop {
      let mut ok = true;
      secret[i] = 0;
      for j in (0..64).step_by(8) {
        secret[i] |=
          u64::from(BALANCED_BYTES[(wyrand(&mut seed) % BALANCED_BYTES.len() as u64) as usize])
            << j;
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
