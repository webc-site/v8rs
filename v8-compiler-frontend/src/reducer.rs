//! Reduction / Reducer 基础形 — 前端各 reducer 的返回类型
//!
//! 本体在 v8/src/compiler/graph-reducer.h (归 W9CP v8-compiler-passes 域),
//! 本模块只提供前端签名所需的最小 1:1 形。

use v8_compiler_graph::Node;

/// 削减结果 — replacement 非空即 Changed
/// v8: src/compiler/graph-reducer.h Reduction
#[derive(Debug, Clone, Copy)]
pub struct Reduction<'z> {
  /// replacement_
  pub replacement: Option<&'z Node>,
}

impl<'z> Reduction<'z> {
  /// 原版 Reduction(Node* replacement = nullptr)
  /// v8: src/compiler/graph-reducer.h Reduction::Reduction
  pub const fn new(replacement: Option<&'z Node>) -> Self {
    Self { replacement }
  }

  /// v8: src/compiler/graph-reducer.h Reduction::replacement
  pub const fn replacement(&self) -> Option<&'z Node> {
    self.replacement
  }

  /// v8: src/compiler/graph-reducer.h Reduction::Changed
  pub const fn changed(&self) -> bool {
    self.replacement.is_some()
  }

  /// next 有改动则取 next, 否则保留自身
  /// v8: src/compiler/graph-reducer.h Reduction::FollowedBy
  pub const fn followed_by(&self, next: Self) -> Self {
    if next.changed() { next } else { *self }
  }

  /// v8: src/compiler/graph-reducer.h Reducer::NoChange
  pub const fn no_change() -> Self {
    Self { replacement: None }
  }

  /// v8: src/compiler/graph-reducer.h Reducer::Replace
  pub const fn replace(node: &'z Node) -> Self {
    Self {
      replacement: Some(node),
    }
  }

  /// v8: src/compiler/graph-reducer.h Reducer::Changed
  pub const fn changed_from(node: &'z Node) -> Self {
    Self {
      replacement: Some(node),
    }
  }
}
