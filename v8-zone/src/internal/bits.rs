//! v8-base 最小私有替身 (待回收: 下一轮替换为 v8-base 对应模块)
//!
//! 对应原版: src/base/macros.h (kKB/kMB)、RoundUp/RoundDown (utils/allocation.h)、
//! kAlignmentInBytes (zone.h 私有常量, 此处共享)

/// 原版 Zone::kAlignmentInBytes — 所有指针 8 字节对齐
pub(crate) const ALIGNMENT: usize = 8;

pub(crate) const KB: usize = 1024;
pub(crate) const MB: usize = 1024 * KB;

/// 原版 RoundUp(v, 2^m)
#[inline]
pub(crate) const fn align_up(v: usize, a: usize) -> usize {
  (v + (a - 1)) & !(a - 1)
}

/// 原版 RoundDown(v, 2^m)
#[inline]
pub(crate) const fn align_down(v: usize, a: usize) -> usize {
  v & !(a - 1)
}

/// 原版 IsAligned(v, 2^m)
#[inline]
pub(crate) const fn is_aligned(v: usize, a: usize) -> bool {
  v & (a - 1) == 0
}
