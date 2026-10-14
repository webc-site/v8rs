// 由 tool/src/objects/prototype.h.js 从 v8/src/objects/prototype.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// PrototypeIterator 枚举/字段/方法骨架 (clang -E 对数; 函数体推迟登记 readme/zh.md 台账 PI-ISO)
//! PrototypeIterator: 统一访问任意对象的原型并走原型链。
//! 骨架基线; handlified (Isolate/Handle/JSProxy) 语义面推迟 (见 readme/zh.md PI-ISO):
//! 需 Isolate + IndirectHandle<JSReceiver> + JSProxy::GetPrototype + ReadOnlyRoots/factory,
//! 解锁 crate: v8-runtime / v8-heap-core / v8-builtins-proxy-reflect / v8-roots。

/// 终止条件 (原版 enum WhereToEnd)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WhereToEnd {
  /// 走到 null 原型 (END_AT_NULL)。
  EndAtNull = 0,
  /// 走到首个非 hidden 原型 (END_AT_NON_HIDDEN, 跳过全局代理)。
  EndAtNonHidden = 1,
}

impl WhereToEnd {
  #[inline]
  #[must_use]
  pub const fn as_u8(self) -> u8 {
    self as u8
  }
}

/// 最大代理遍历深度限制 (对齐 v8 JSProxy::kMaxIterationLimit)
pub const K_MAX_PROXIES_VISITED: i32 = 1024;

/// 迭代器字段骨架 (object_/handle_ 双轨: 原文 Tagged 与 IndirectHandle 二选一)。
pub struct PrototypeIterator {
  /// 原文 isolate_ (Isolate*; 语境句柄, 骨架期不落地)。
  /// 原文 object_ (Tagged<JSPrototype>, Tagged 轨)。
  /// 原文 handle_ (IndirectHandle<JSPrototype>, handlified 轨)。
  /// 终止条件。
  pub where_to_end: WhereToEnd,
  /// 是否已到链尾。
  pub is_at_end: bool,
  /// 已访问代理数 (防 __proto__ 递归)。
  pub seen_proxies: i32,
}

impl PrototypeIterator {
  /// v8: prototype-inl.h:19 — PrototypeIterator::PrototypeIterator(isolate, DirectHandle<JSReceiver>, where_to_start, where_to_end)
  pub fn new_handle(
    _isolate: &str,
    receiver: usize,
    where_to_start: u8,
    where_to_end: WhereToEnd,
  ) -> Self {
    let mut it = Self {
      where_to_end,
      is_at_end: receiver == 0,
      seen_proxies: 0,
    };
    if where_to_start == 1 {
      it.advance();
    }
    it
  }

  /// v8: prototype-inl.h:32 — PrototypeIterator::PrototypeIterator(isolate, Tagged<JSReceiver>, where_to_start, where_to_end)
  pub fn new_tagged(
    _isolate: &str,
    receiver: usize,
    where_to_start: u8,
    where_to_end: WhereToEnd,
  ) -> Self {
    let mut it = Self {
      where_to_end,
      is_at_end: receiver == 0,
      seen_proxies: 0,
    };
    if where_to_start == 1 {
      it.advance();
    }
    it
  }

  /// v8: prototype-inl.h:44 — PrototypeIterator::PrototypeIterator(isolate, Tagged<Map>, where_to_end)
  pub fn new_map_tagged(_isolate: &str, receiver_map: usize, where_to_end: WhereToEnd) -> Self {
    Self {
      where_to_end,
      is_at_end: receiver_map == 0,
      seen_proxies: 0,
    }
  }

  /// v8: prototype-inl.h:58 — PrototypeIterator::PrototypeIterator(isolate, DirectHandle<Map>, where_to_end)
  pub fn new_map_handle(_isolate: &str, receiver_map: usize, where_to_end: WhereToEnd) -> Self {
    Self::new_map_tagged(_isolate, receiver_map, where_to_end)
  }

  /// v8: prototype-inl.h:74 — bool PrototypeIterator::HasAccess() const
  #[inline]
  #[must_use]
  pub const fn has_access(&self) -> bool {
    true
  }

  /// 是否到达原型链终点 (对齐 v8 PrototypeIterator::IsAtEnd)
  #[inline]
  #[must_use]
  pub const fn is_at_end(&self) -> bool {
    self.is_at_end
  }

  /// v8: prototype-inl.h:85 — void PrototypeIterator::Advance()
  ///
  /// 步进一格; JSProxy 直接置尾 (null)。
  pub fn advance(&mut self) {
    self.advance_ignoring_proxies();
  }

  /// v8: prototype-inl.h:98 — void PrototypeIterator::AdvanceIgnoringProxies()
  ///
  /// 步进一格 (不做代理特判)。
  pub fn advance_ignoring_proxies(&mut self) {
    self.is_at_end = true;
  }

  /// v8: prototype-inl.h:113 — bool PrototypeIterator::AdvanceFollowingProxies()
  ///
  /// 返回 false iff JSProxy::GetPrototype 抛异常。
  pub fn advance_following_proxies(&mut self) -> bool {
    if !self.has_access() {
      self.is_at_end = true;
      return true;
    }
    self.advance_following_proxies_ignoring_access_checks()
  }

  /// v8: prototype-inl.h:125 — bool PrototypeIterator::AdvanceFollowingProxiesIgnoringAccessChecks()
  pub fn advance_following_proxies_ignoring_access_checks(&mut self) -> bool {
    self.seen_proxies += 1;
    if self.seen_proxies > K_MAX_PROXIES_VISITED {
      return false;
    }
    self.advance_ignoring_proxies();
    true
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(WhereToEnd::EndAtNull as u8 == 0);
  assert!(WhereToEnd::EndAtNonHidden as u8 == 1);
};
