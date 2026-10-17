// 由 tool/src/flags/flags.cc.js 从 v8/src/flags/flags.cc (引用的 v8/src/base/bits.h + include/v8-internal.h 辅助) 生成; 修订就地改, 复跑覆盖
// 自含实现, 后续 v8-base 接管时迁移

/// 2 的幂判定 (原版 base::bits::IsPowerOfTwo)
#[inline]
pub(crate) fn is_pow2(v: u64) -> bool {
  v > 0 && v & (v - 1) == 0
}

/// Smi 值域校验 (原版 Internals::IsValidSmi)
///
/// 规范配置: 指针压缩开 → kSmiValueSize = 31 (v8-internal.h),
/// 值域 [-2^30, 2^30 - 1]。
#[inline]
pub(crate) fn is_valid_smi(v: i64) -> bool {
  const SMI_MIN: i64 = -(1 << 30);
  const SMI_MAX: i64 = (1 << 30) - 1;
  (SMI_MIN..=SMI_MAX).contains(&v)
}
