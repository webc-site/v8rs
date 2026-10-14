// 由 tool/src/numbers/hash-seed-inl.h.js 从 v8/src/numbers/hash-seed-inl.h 生成; 修订就地改, 复跑覆盖
// HashSeed 内联访问骨架与哈希种子混合计算

use crate::hash_seed_cc_generated::{
  ARRAY_INDEX_VALUE_BITS, ARRAY_INDEX_VALUE_MASK, DEFAULT_SEED,
  HashSeedData as HashSeedDataExtended,
};

/// HashSeed 内联层签名 (原版 hash-seed-inl.h)。
/// Data 布局: { uint64_t seed; uint64_t secrets[kSecretsCount]; ... }
/// 实例只指向只读根或 kDefaultData, 不持所有者。
pub mod hash_seed_inl_skel {
  use std::mem::align_of;

  use super::*;

  /// 平台分支登记: V8_ENABLE_SEEDED_ARRAY_INDEX_HASH
  pub const SEEDED_ARRAY_INDEX_HASH_ENABLED: bool = true;

  /// 数组索引哈希半折位宽 (24 / 2 = 12)
  pub const ARRAY_INDEX_HASH_SHIFT: u32 = ARRAY_INDEX_VALUE_BITS / 2;

  /// 解析传入的 raw data 指针: 0 时兜底至全局默认种子
  #[inline(always)]
  fn resolve_ptr(data: usize) -> *const HashSeedDataExtended {
    if data == 0 {
      &DEFAULT_SEED as *const HashSeedDataExtended
    } else {
      debug_assert_eq!(
        0,
        data % align_of::<HashSeedDataExtended>(),
        "data 必须按 alignof(HashSeedData) 对齐"
      );
      data as *const HashSeedDataExtended
    }
  }

  /// 默认种子 (原版 Default() → kDefaultData; 默认值见表 hash_seed_cc_generated.rs)
  #[inline]
  #[must_use]
  pub fn default_seed() -> usize {
    &DEFAULT_SEED as *const HashSeedDataExtended as usize
  }

  /// 种子值 (原版 seed(): data_->seed)
  #[inline]
  #[must_use]
  pub fn seed(data: usize) -> u64 {
    // SAFETY: data 指向合法的 HashSeedData 结构体或默认全局静态种子
    unsafe { (*resolve_ptr(data)).seed }
  }

  /// 密钥组指针 (原版 secret(): data_->secrets)
  #[inline]
  #[must_use]
  pub fn secret(data: usize) -> *const u64 {
    // SAFETY: data 指向合法的 HashSeedData 结构体或默认全局静态种子
    unsafe { (*resolve_ptr(data)).secrets.as_ptr() }
  }

  /// 密钥组安全切片借用
  #[inline]
  #[must_use]
  pub fn secrets<'a>(data: usize) -> &'a [u64; 3] {
    // SAFETY: data 指向合法的 HashSeedData 结构体或默认全局静态种子
    unsafe { &(*resolve_ptr(data)).secrets }
  }

  /// 从只读根构造 (原版 HashSeed(ReadOnlyRoots)); 对齐断言:
  /// data_ 按 alignof(HashSeed::Data) 对齐
  #[inline]
  #[must_use]
  pub fn from_read_only_roots(roots: usize) -> usize {
    if roots == 0 {
      return default_seed();
    }
    debug_assert_eq!(
      0,
      roots % align_of::<HashSeedDataExtended>(),
      "data_ 按 alignof(HashSeed::Data) 对齐"
    );
    roots
  }

  /// 从 Isolate 构造 (原版 HashSeed(Isolate*) → 委派 ReadOnlyRoots)
  #[inline]
  #[must_use]
  pub fn from_isolate(isolate_key: usize) -> usize {
    from_read_only_roots(isolate_key)
  }

  /// 从 LocalIsolate 构造 (原版 HashSeed(LocalIsolate*) → 委派 ReadOnlyRoots)
  #[inline]
  #[must_use]
  pub fn from_local_isolate(isolate_key: usize) -> usize {
    from_read_only_roots(isolate_key)
  }

  /// m1 访问器
  #[inline]
  #[must_use]
  pub fn m1(data: usize) -> u32 {
    // SAFETY: data 指向合法的 HashSeedData 结构体或默认全局静态种子
    unsafe { (*resolve_ptr(data)).m1 }
  }

  /// m1_inv 访问器
  #[inline]
  #[must_use]
  pub fn m1_inv(data: usize) -> u32 {
    // SAFETY: data 指向合法的 HashSeedData 结构体或默认全局静态种子
    unsafe { (*resolve_ptr(data)).m1_inv }
  }

  /// m2 访问器
  #[inline]
  #[must_use]
  pub fn m2(data: usize) -> u32 {
    // SAFETY: data 指向合法的 HashSeedData 结构体或默认全局静态种子
    unsafe { (*resolve_ptr(data)).m2 }
  }

  /// m2_inv 访问器
  #[inline]
  #[must_use]
  pub fn m2_inv(data: usize) -> u32 {
    // SAFETY: data 指向合法的 HashSeedData 结构体或默认全局静态种子
    unsafe { (*resolve_ptr(data)).m2_inv }
  }

  /// m3 访问器
  #[inline]
  #[must_use]
  pub fn m3(data: usize) -> u32 {
    // SAFETY: data 指向合法的 HashSeedData 结构体或默认全局静态种子
    unsafe { (*resolve_ptr(data)).m3 }
  }

  /// m3_inv 访问器
  #[inline]
  #[must_use]
  pub fn m3_inv(data: usize) -> u32 {
    // SAFETY: data 指向合法的 HashSeedData 结构体或默认全局静态种子
    unsafe { (*resolve_ptr(data)).m3_inv }
  }

  /// 数组索引哈希加盐混合 (原版 StringHasher::SeedArrayIndexValue)
  /// 3 轮 xorshift-multiply:
  /// x ^= x >> 12; x = (x * m1) & mask;
  /// x ^= x >> 12; x = (x * m2) & mask;
  /// x ^= x >> 12; x = (x * m3) & mask;
  /// x ^= x >> 12;
  #[inline]
  #[must_use]
  pub fn seed_array_index_value(value: u32, data: usize) -> u32 {
    let ptr = resolve_ptr(data);
    // 单次解引用批量取出乘数, 避免 3 次条件分支与独立加载
    let (m1_val, m2_val, m3_val) = unsafe { ((*ptr).m1, (*ptr).m2, (*ptr).m3) };
    let mut x = value;
    x ^= x >> ARRAY_INDEX_HASH_SHIFT;
    x = x.wrapping_mul(m1_val) & ARRAY_INDEX_VALUE_MASK;
    x ^= x >> ARRAY_INDEX_HASH_SHIFT;
    x = x.wrapping_mul(m2_val) & ARRAY_INDEX_VALUE_MASK;
    x ^= x >> ARRAY_INDEX_HASH_SHIFT;
    x = x.wrapping_mul(m3_val) & ARRAY_INDEX_VALUE_MASK;
    x ^= x >> ARRAY_INDEX_HASH_SHIFT;
    x
  }

  /// 数组索引哈希解盐逆向恢复 (原版 StringHasher::UnseedArrayIndexValue)
  #[inline]
  #[must_use]
  pub fn unseed_array_index_value(value: u32, data: usize) -> u32 {
    let ptr = resolve_ptr(data);
    // 单次解引用批量取出模逆, 避免 3 次条件分支与独立加载
    let (m1_inv_val, m2_inv_val, m3_inv_val) =
      unsafe { ((*ptr).m1_inv, (*ptr).m2_inv, (*ptr).m3_inv) };
    let mut x = value;
    x ^= x >> ARRAY_INDEX_HASH_SHIFT;
    x = x.wrapping_mul(m3_inv_val) & ARRAY_INDEX_VALUE_MASK;
    x ^= x >> ARRAY_INDEX_HASH_SHIFT;
    x = x.wrapping_mul(m2_inv_val) & ARRAY_INDEX_VALUE_MASK;
    x ^= x >> ARRAY_INDEX_HASH_SHIFT;
    x = x.wrapping_mul(m1_inv_val) & ARRAY_INDEX_VALUE_MASK;
    x ^= x >> ARRAY_INDEX_HASH_SHIFT;
    x
  }

  /// 原版 HashSeed 轻量只读包装 (按值传递 1 个指针宽)
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct HashSeed {
    ptr: *const HashSeedDataExtended,
  }

  impl Default for HashSeed {
    #[inline]
    fn default() -> Self {
      Self::from_data(default_seed())
    }
  }

  impl HashSeed {
    #[inline]
    #[must_use]
    pub const fn from_data(data: usize) -> Self {
      Self {
        ptr: if data == 0 {
          &DEFAULT_SEED as *const HashSeedDataExtended
        } else {
          data as *const HashSeedDataExtended
        },
      }
    }

    #[inline]
    #[must_use]
    pub fn seed(&self) -> u64 {
      // SAFETY: ptr 指向合法的 HashSeedData 静态或对齐区
      unsafe { (*self.ptr).seed }
    }

    #[inline]
    #[must_use]
    pub fn secrets(&self) -> &[u64; 3] {
      unsafe { &(*self.ptr).secrets }
    }

    #[inline]
    #[must_use]
    pub fn m1(&self) -> u32 {
      unsafe { (*self.ptr).m1 }
    }

    #[inline]
    #[must_use]
    pub fn m1_inv(&self) -> u32 {
      unsafe { (*self.ptr).m1_inv }
    }

    #[inline]
    #[must_use]
    pub fn m2(&self) -> u32 {
      unsafe { (*self.ptr).m2 }
    }

    #[inline]
    #[must_use]
    pub fn m2_inv(&self) -> u32 {
      unsafe { (*self.ptr).m2_inv }
    }

    #[inline]
    #[must_use]
    pub fn m3(&self) -> u32 {
      unsafe { (*self.ptr).m3 }
    }

    #[inline]
    #[must_use]
    pub fn m3_inv(&self) -> u32 {
      unsafe { (*self.ptr).m3_inv }
    }

    #[inline]
    #[must_use]
    pub fn seed_array_index_value(&self, value: u32) -> u32 {
      seed_array_index_value(value, self.ptr as usize)
    }

    #[inline]
    #[must_use]
    pub fn unseed_array_index_value(&self, value: u32) -> u32 {
      unseed_array_index_value(value, self.ptr as usize)
    }
  }
}
