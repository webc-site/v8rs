// 由 tool/src/numbers/hash-seed.cc.js 从 v8/src/numbers/hash-seed.h/cc + third_party/rapidhash-v8/secret.h 生成; 修订就地改, 复跑覆盖

//! HashSeed — 只读根 hash_seed 字节区上的轻量视图 (数据面)
//!
//! 默认构建 (v8_use_default_hasher_secret=true): secrets 恒为
//! RAPIDHASH_DEFAULT_SECRET, 仅 meta seed (--hash-seed / RNG) 变化;
//! rapidhash_make_secret 派生路径与 V8_ENABLE_SEEDED_ARRAY_INDEX_HASH 的
//! m1..m3 模逆组在默认构建不编译, 未转写 (翻转配置时补)。
//! Isolate 绑定 (InitializeRoots / ReadOnlyRoots 视图) 推迟到对象层。

/// kSecretsCount — secrets 元组长度 (hash-seed.h)
pub const SECRETS_COUNT: usize = 3;

/// RAPIDHASH_DEFAULT_SECRET — wyhash 式默认密钥 (secret.h, 脚本提取)
pub const RAPIDHASH_DEFAULT_SECRET: [u64; SECRETS_COUNT] =
  [0x2d358dccaa6c78a5, 0x8bb84b93962eacc9, 0x4b33a62ed433d4a3];

/// HashSeed::Data — 覆盖在只读根 hash_seed 字节区上的布局 (8 字节对齐)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashSeedData {
  /// 元种子: 来自 --hash-seed, 0 表示启动时生成
  pub seed: u64,
  /// 哈希密钥元组; 默认构建恒为 RAPIDHASH_DEFAULT_SECRET
  pub secrets: [u64; SECRETS_COUNT],
}

impl HashSeedData {
  /// 静态默认种子 (kDefaultSeed: seed=0 + 默认 secret)
  pub const DEFAULT: Self = Self {
    seed: 0,
    secrets: RAPIDHASH_DEFAULT_SECRET,
  };
}

/// HashSeed — Data 上的轻量视图 (原版为 const Data* 指针包装)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashSeed {
  data: &'static HashSeedData,
}

impl HashSeed {
  /// HashSeed::Default() — 指向静态默认种子
  pub const fn default_seed() -> Self {
    Self {
      data: &HashSeedData::DEFAULT,
    }
  }

  /// seed() — 元种子
  pub const fn seed(&self) -> u64 {
    self.data.seed
  }

  /// secret() — 密钥元组
  pub const fn secret(&self) -> &[u64; SECRETS_COUNT] {
    &self.data.secrets
  }
}

// —— 提取不变式 ——

const _: () = {
  assert!(SECRETS_COUNT == 3);
  assert!(RAPIDHASH_DEFAULT_SECRET[0] == 0x2d35_8dcc_aa6c_78a5);
  assert!(RAPIDHASH_DEFAULT_SECRET[1] == 0x8bb8_4b93_962e_acc9);
  assert!(RAPIDHASH_DEFAULT_SECRET[2] == 0x4b33_a62e_d433_d4a3);
  assert!(HashSeed::default_seed().secret()[0] == RAPIDHASH_DEFAULT_SECRET[0]);
  assert!(HashSeed::default_seed().seed() == 0);
};
