//! v8/src/base/container-utils.h 的 1:1 转写
//! C++ 容器泛型 (has_find SFINAE) → Rust 以切片/Vec 为主的具体泛型函数
//! std::optional<size_t> → Option<usize>

use std::cmp::Ordering;

/// contains: 元素是否在容器中 (原版对 set 走 find, 此处切片线性查找语义等价)
#[inline]
pub fn contains<T: PartialEq>(container: &[T], element: &T) -> bool {
  container.iter().any(|v| v == element)
}

/// index_of: 首个等于 element 的下标; 不存在返回 None
#[inline]
pub fn index_of<T: PartialEq>(container: &[T], element: &T) -> Option<usize> {
  container.iter().position(|v| v == element)
}

/// index_of_if: 首个满足 predicate 的下标
#[inline]
pub fn index_of_if<T, P: FnMut(&T) -> bool>(container: &[T], predicate: P) -> Option<usize> {
  container.iter().position(predicate)
}

/// erase_at: 自 index 起移除 count 个 (越界截断), 返回实际移除数
#[inline]
pub fn erase_at<T>(container: &mut Vec<T>, index: usize, count: usize) -> usize {
  if container.len() <= index {
    return 0;
  }
  let count = count.min(container.len() - index);
  container.drain(index..index + count);
  count
}

/// 默认可信性 trait: 原版 static_cast<bool>(value) 的对应 (bool/Option)
pub trait Truthy {
  fn truthy(&self) -> bool;
}

impl Truthy for bool {
  #[inline]
  fn truthy(&self) -> bool {
    *self
  }
}

impl<T> Truthy for Option<T> {
  #[inline]
  fn truthy(&self) -> bool {
    self.is_some()
  }
}

/// erase_if: 移除所有满足 predicate 的元素, 返回移除数
#[inline]
pub fn erase_if<T, P: FnMut(&T) -> bool>(container: &mut Vec<T>, mut predicate: P) -> usize {
  let before = container.len();
  container.retain(|v| !predicate(v));
  before - container.len()
}

/// count_if
#[inline]
pub fn count_if<T, P: FnMut(&T) -> bool>(container: &[T], mut predicate: P) -> usize {
  container.iter().filter(|v| predicate(v)).count()
}

/// all_of (谓词版)
#[inline]
pub fn all_of<T, P: FnMut(&T) -> bool>(container: &[T], predicate: P) -> bool {
  container.iter().all(predicate)
}

/// all_of (无谓词版, static_cast<bool>)
#[inline]
pub fn all_of_truthy<T: Truthy>(container: &[T]) -> bool {
  container.iter().all(|v| v.truthy())
}

/// any_of (谓词版)
#[inline]
pub fn any_of<T, P: FnMut(&T) -> bool>(container: &[T], predicate: P) -> bool {
  container.iter().any(predicate)
}

/// any_of (无谓词版)
#[inline]
pub fn any_of_truthy<T: Truthy>(container: &[T]) -> bool {
  container.iter().any(|v| v.truthy())
}

/// none_of
#[inline]
pub fn none_of<T, P: FnMut(&T) -> bool>(container: &[T], mut predicate: P) -> bool {
  container.iter().all(|v| !predicate(v))
}

/// sort (默认序)
#[inline]
pub fn sort<T: Ord>(container: &mut [T]) {
  container.sort();
}

/// sort (比较器版)
#[inline]
pub fn sort_by<T, F: FnMut(&T, &T) -> Ordering>(container: &mut [T], comp: F) {
  container.sort_by(comp);
}

/// all_equal: 所有元素彼此相等
#[inline]
pub fn all_equal<T: PartialEq>(container: &[T]) -> bool {
  if container.len() <= 1 {
    return true;
  }
  let value = &container[0];
  container[1..].iter().all(|v| v == value)
}

/// all_equal (与给定值比较版)
#[inline]
pub fn all_equal_to<T: PartialEq>(container: &[T], value: &T) -> bool {
  container.iter().all(|v| v == value)
}

/// vector_append: 追加区间到 Vec
#[inline]
pub fn vector_append<T: Clone>(v: &mut Vec<T>, container: &[T]) {
  v.extend_from_slice(container);
}
