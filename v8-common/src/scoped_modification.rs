// 由 tool/src/common.js 从 v8/src/common/scoped-modification.h 生成;
// 修订就地改, 复跑覆盖
//
// 修订 (Developer): Drop 恢复语义以 Option<T> + take 承载 (T 无 Default 也可
// 回滚), 析构单次执行由所有权保证, 全安全无 unsafe。

use core::mem::replace;

/// 原版 template ScopedModification<T> (scoped-modification.h:15-29) —
/// 作用域内 *ptr = new_value, 退出时恢复旧值
pub struct ScopedModification<'a, T> {
  ptr: &'a mut T,
  old_value: Option<T>,
}

impl<'a, T> ScopedModification<'a, T> {
  pub fn new(ptr: &'a mut T, new_value: T) -> Self {
    let old_value = replace(ptr, new_value);
    Self {
      ptr,
      old_value: Some(old_value),
    }
  }

  /// 原版 old_value() (scoped-modification.h:24) —
  /// Drop 前恒 Some (由 new 置入、仅 Drop 取出), unwrap 100% 安全
  pub fn old_value(&self) -> &T {
    self.old_value.as_ref().unwrap()
  }
}

impl<T> Drop for ScopedModification<'_, T> {
  fn drop(&mut self) {
    if let Some(old) = self.old_value.take() {
      *self.ptr = old;
    }
  }
}
