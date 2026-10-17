// 由 tool/src/heap/index-generator.h.js 从 v8/src/heap/index-generator.{h,cc} 生成; 修订就地改, 复跑覆盖
// 并行启发式下标生成器 (对分取中规则直译)

use std::{collections::VecDeque, sync::Mutex};

struct IndexGeneratorState {
  first_use: bool,
  ranges_to_split: VecDeque<(usize, usize)>,
}

/// 线程安全的启发式起始点生成器 (对应 class IndexGenerator)
/// 规则 (原版 GetNext): 首次使用返回 0; 之后取最老区间中点, 左右半区间宽 >1 时回队。
pub struct IndexGenerator {
  state: Mutex<IndexGeneratorState>,
}

impl IndexGenerator {
  /// 对应 IndexGenerator(size_t size)
  #[must_use]
  pub fn new(size: usize) -> Self {
    let mut ranges = VecDeque::new();
    if size != 0 {
      ranges.push_back((0, size));
    }
    Self {
      state: Mutex::new(IndexGeneratorState {
        first_use: size > 0,
        ranges_to_split: ranges,
      }),
    }
  }

  /// 对应 GetNext() -> std::optional<size_t>
  pub fn get_next(&self) -> Option<usize> {
    let mut state = self.state.lock().ok()?;
    if state.first_use {
      state.first_use = false;
      return Some(0);
    }
    let (first, second) = state.ranges_to_split.pop_front()?;
    let size = second - first;
    // 对分取中 (原版 size_t mid = range.first + size / 2)
    let mid = first + size / 2;
    if mid - first > 1 {
      state.ranges_to_split.push_back((first, mid));
    }
    if second - mid > 1 {
      state.ranges_to_split.push_back((mid, second));
    }
    Some(mid)
  }
}

// —— 提取不变式断言 ——
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn first_use_returns_zero() {
    let mut g = IndexGenerator::new(10);
    assert_eq!(g.get_next(), Some(0));
  }

  #[test]
  fn empty_size_never_yields() {
    let mut g = IndexGenerator::new(0);
    assert_eq!(g.get_next(), None);
  }

  #[test]
  fn splits_at_middle() {
    let mut g = IndexGenerator::new(10);
    assert_eq!(g.get_next(), Some(0)); // first_use
    assert_eq!(g.get_next(), Some(5)); // [0,10) 中点
  }

  #[test]
  fn eventually_exhausts() {
    let mut g = IndexGenerator::new(4);
    let mut seen = Vec::new();
    while let Some(i) = g.get_next() {
      seen.push(i);
      assert!(seen.len() < 100);
    }
    assert!(seen.contains(&0));
  }
}
