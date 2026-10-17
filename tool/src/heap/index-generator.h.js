#!/usr/bin/env bun
// tool/src/heap/index-generator.h.js
// 机械提取 v8/src/heap/index-generator.{h,cc}: IndexGenerator
// cc 侧仅 30 行, GetNext 为纯机械区间对分规则, 直译为规则代码。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/index-generator.h");
const ccSrc = readV8("src/heap/index-generator.cc");

if (!hSrc.includes("class V8_EXPORT_PRIVATE IndexGenerator")) throw new Error("类锚点缺失");
const ctor = ccSrc.match(/IndexGenerator::IndexGenerator\(size_t size\) : first_use_\(size > 0\)/);
const getNext = ccSrc.match(/std::optional<size_t> IndexGenerator::GetNext\(\)/);
const firstUse = ccSrc.match(/if \(first_use_\) \{\s*\n\s*first_use_ = false;\s*\n\s*return 0;/);
const splitRule = ccSrc.match(/size_t mid = range\.first \+ size \/ 2;/);
const guardA = ccSrc.match(/if \(mid - range\.first > 1\) ranges_to_split_\.emplace\(range\.first, mid\);/);
const guardB = ccSrc.match(/if \(range\.second - mid > 1\) ranges_to_split_\.emplace\(mid, range\.second\);/);
if (!ctor || !getNext || !firstUse || !splitRule || !guardA || !guardB) {
  throw new Error("GetNext 机械规则锚点缺失 (提取规则需复核)");
}

console.log("IndexGenerator: first_use→0, 区间对分取中, 半区间>1 回队");

let rust = banner(
  "src/heap/index-generator.h.js",
  "src/heap/index-generator.{h,cc}",
  "并行启发式下标生成器 (对分取中规则直译)",
);

rust += `
use std::sync::Mutex;

/// 线程安全的启发式起始点生成器 (对应 class IndexGenerator)
/// 规则 (原版 GetNext): 首次使用返回 0; 之后取最老区间中点, 左右半区间宽 >1 时回队。
pub struct IndexGenerator {
  /// base::Mutex lock_
  lock_: Mutex<()>,
  /// bool first_use_ = size > 0
  first_use_: bool,
  /// 待切分的 [start, end) 区间队列 (FIFO)
  ranges_to_split_: std::collections::VecDeque<(usize, usize)>,
}

impl IndexGenerator {
  /// 对应 IndexGenerator(size_t size)
  #[must_use]
  pub fn new(size: usize) -> Self {
    let mut ranges = std::collections::VecDeque::new();
    if size != 0 {
      ranges.push_back((0, size));
    }
    Self { lock_: Mutex::new(()), first_use_: size > 0, ranges_to_split_: ranges }
  }

  /// 对应 GetNext() -> std::optional<size_t>
  pub fn get_next(&mut self) -> Option<usize> {
    let _guard = self.lock_.lock().ok()?;
    if self.first_use_ {
      self.first_use_ = false;
      return Some(0);
    }
    let (first, second) = self.ranges_to_split_.pop_front()?;
    let size = second - first;
    // 对分取中 (原版 size_t mid = range.first + size / 2)
    let mid = first + size / 2;
    if mid - first > 1 {
      self.ranges_to_split_.push_back((first, mid));
    }
    if second - mid > 1 {
      self.ranges_to_split_.push_back((mid, second));
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
`;
;

const p = writeSrc("v8-heap-core", "index_generator.rs", rust);
console.log(`written ${p}`);
