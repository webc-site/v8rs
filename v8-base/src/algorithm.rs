//! v8/src/base/algorithm.h 的 1:1 转写
//! 本快照 algorithm.h 仅含 base::Copy (std::copy 的 memcpy 加速版):
//! - internal::can_use_memcpy_v: Rust 侧 `T: Copy` 即平凡可复制, 编译期由泛型约束承接
//! - 原版禁止区间重叠 (内部按 par_unseq 语义); Rust 借用规则天然禁止别名,
//!   src/dst 分离由 &[T] / &mut [T] 保证
//!
//! 注: 任务提及的 IsAnyOneOf/SetIntersectionSize 不在本快照 base 目录
//! (上游已移除/位于他处), 登记不实现

use std::ptr::copy_nonoverlapping;

/// base::Copy(first, last, d_first) → 切片形态
/// 复制 `first[..count]` 到 `dst[..count]`, 返回写入的元素数 (原版 d_first + count)
///
/// 与 std::copy 的差别 = 与原版一致: 可复制类型直接委托 memcpy 路径
/// (copy_nonoverlapping, 由后端自动向量化; 对应原版 base::MemCopy)
#[inline]
pub fn copy<T: Copy>(first: &[T], last: &[T], dst: &mut [T]) -> usize {
  let count = first.len().min(last.len());
  debug_assert!(dst.len() >= count, "Copy 目标区间不足");
  if count > 0 {
    // SAFETY: T: Copy 平凡可复制; &[T]/&mut [T] 借用规则保证无重叠且均有效初始化;
    // 长度已由 count 与 debug_assert 约束
    unsafe {
      copy_nonoverlapping(first.as_ptr(), dst.as_mut_ptr(), count);
    }
  }
  count
}
