//! v8/src/base/strong-alias.h 的 1:1 转写
//! Chromium StrongAlias 移植: 类型安全的 typedef 替身, 零成本包装
//! C++ TagType(phantom class) → Rust 空 tag 类型 + PhantomData
//! operator* / operator-> → Deref / DerefMut; value() → value()/value_mut()
//! is_strong_alias_v → IsStrongAlias marker trait
//! Hasher → std::hash::Hash 实现 (按内层值哈希, 语义等价)

use std::{
  fmt,
  hash::{Hash, Hasher},
  marker::PhantomData,
  ops::{Deref, DerefMut},
};

/// 类型别名标记: 禁止直接以内部值构造以外的隐式转换
/// (C++ 的 explicit 构造在 Rust 由 new() 显式调用保证)
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct StrongAlias<T, U> {
  value: U,
  _tag: PhantomData<T>,
}

/// IsStrongAlias 标记 (原版 is_strong_alias<T>::value == true 的 trait 化)
pub trait IsStrongAlias {
  type Underlying;
  fn value(self) -> Self::Underlying;
}

impl<T, U> StrongAlias<T, U> {
  /// explicit StrongAlias(v)
  #[inline]
  pub const fn new(value: U) -> Self {
    Self {
      value,
      _tag: PhantomData,
    }
  }

  /// value() const& — 按值返回内层引用 (Copy 类型场景)
  #[inline]
  pub const fn value(&self) -> &U {
    &self.value
  }

  /// value() & — 可变引用
  #[inline]
  pub fn value_mut(&mut self) -> &mut U {
    &mut self.value
  }

  /// 消耗包装并移动出内层值 (原版 value() &&)
  #[inline]
  pub fn into_value(self) -> U {
    self.value
  }
}

/// operator-> / operator* — 解引用到内层值
impl<T, U> Deref for StrongAlias<T, U> {
  type Target = U;
  #[inline]
  fn deref(&self) -> &U {
    &self.value
  }
}

impl<T, U> DerefMut for StrongAlias<T, U> {
  #[inline]
  fn deref_mut(&mut self) -> &mut U {
    &mut self.value
  }
}

/// explicit operator const UnderlyingType& — Rust 无隐式转换, 提供 as_ref
impl<T, U> AsRef<U> for StrongAlias<T, U> {
  #[inline]
  fn as_ref(&self) -> &U {
    &self.value
  }
}

/// Hasher: std::hash<UnderlyingType>()(id.value())
impl<T, U: Hash> Hash for StrongAlias<T, U> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.value.hash(state);
  }
}

/// 原版流输出 operator<< (requires stream << value) → Display 条件实现不可, 用 Debug 透传
impl<T, U: fmt::Display> fmt::Display for StrongAlias<T, U> {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&self.value, f)
  }
}

/// 原版 is_strong_alias trait 的 Rust 对应: 通过本 impl 识别
impl<T, U> IsStrongAlias for StrongAlias<T, U> {
  type Underlying = U;
  #[inline]
  fn value(self) -> U {
    self.value
  }
}

/// 生成 tag 空结构 + 别名的便捷宏 (对应 using X = StrongAlias<XTag, T>)
#[macro_export]
macro_rules! strong_alias {
  ($vis:vis $alias:ident = $tag:ident, $ty:ty) => {
    /// phantom tag (原版空 tag class)

    $vis struct $tag(());
    $vis type $alias = $crate::strong_alias::StrongAlias<$tag, $ty>;
  };
}
