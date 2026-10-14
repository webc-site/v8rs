//! v8/src/base/vector.h 的 1:1 转写
//! Vector<T> = 非拥有的连续内存视图 (std::span)。Rust 表达:
//! - `Vector<'a, T>`: 读写视图, 内建 &mut [T] (保持独占写语义)
//! - `ConstVector<'a, T>`: 只读视图, 内建 &[T] (对应原版 Vector<const U> 转换)
//!
//! 原版隐式 Vector<T>→Vector<const U> 转换 → as_const()/into_const()
//! vector-unittest.cc 的用例均经访问器断言, API 一一对应

use std::{
  iter::Rev,
  mem::{align_of, size_of, size_of_val, take},
  ptr::NonNull,
  slice::{Iter, from_raw_parts, from_raw_parts_mut},
};

use crate::hashing::{HashValue, Hasher};

/// 读写视图 (原版 Vector<T>)
pub struct Vector<'a, T> {
  inner: &'a mut [T],
}

/// 只读视图 (原版 Vector<const T> 语义)
#[derive(Debug, Clone, Copy)]
pub struct ConstVector<'a, T> {
  inner: &'a [T],
}

impl<'a, T> Vector<'a, T> {
  /// Vector(T* data, size_t length): 由可变切片构造
  #[inline]
  pub fn new(data: &'a mut [T]) -> Self {
    Self { inner: data }
  }

  /// SubVector(from, to): [from, to) — 消费自身拆出新视图
  /// (原版 const 方法可别名共享, Rust 以 split 系列保持唯一写借用)
  #[inline]
  pub fn sub_vector(self, from: usize, to: usize) -> Vector<'a, T> {
    debug_assert!(from <= to && to <= self.inner.len());
    let to = to.min(self.inner.len());
    Vector {
      inner: &mut self.inner[from.min(to)..to],
    }
  }

  /// SubVectorFrom(from)
  #[inline]
  pub fn sub_vector_from(self, from: usize) -> Vector<'a, T> {
    let len = self.inner.len();
    debug_assert!(from <= len);
    Vector {
      inner: &mut self.inner[from.min(len)..],
    }
  }

  /// first(count)
  #[inline]
  pub fn first(self, count: usize) -> Vector<'a, T> {
    debug_assert!(count <= self.inner.len());
    let count = count.min(self.inner.len());
    Vector {
      inner: &mut self.inner[..count],
    }
  }

  /// last(count)
  #[inline]
  pub fn last(self, count: usize) -> Vector<'a, T> {
    let len = self.inner.len();
    debug_assert!(count <= len);
    Vector {
      inner: &mut self.inner[len - count.min(len)..],
    }
  }

  /// OverwriteWith(等长 &[T]): 原版 base::Copy (memcpy 语义)
  #[inline]
  pub fn overwrite_with(&mut self, other: &[T])
  where
    T: Copy,
  {
    debug_assert_eq!(self.inner.len(), other.len());
    let n = self.inner.len().min(other.len());
    self.inner[..n].copy_from_slice(&other[..n]);
  }

  /// Truncate(length)
  ///
  /// 切片长度收缩但借用生命周期 `'a` 不变; `split_at_mut` 会缩短借用寿命,
  /// 故沿用原指针 + 收缩长度 (等价 `span_ = span_.first(length)`)。
  #[inline]
  pub fn truncate(&mut self, length: usize) {
    debug_assert!(length <= self.inner.len());
    let len = length.min(self.inner.len());
    let inner: &'a mut [T] = take(&mut self.inner);
    // SAFETY: 指针取自原切片, 新长度 len <= inner.len(), 因此区间仍在原借用
    // 的合法范围内; 原切片所有权已被 take 移交给本次 reborrow, 无别名冲突。
    self.inner = unsafe { from_raw_parts_mut(inner.as_mut_ptr(), len) };
  }

  /// as_const(): 转只读视图 (原版隐式转换)
  #[inline]
  pub fn as_const(&self) -> ConstVector<'_, T> {
    ConstVector { inner: self.inner }
  }

  /// into_const(): 消耗自身转只读
  #[inline]
  pub fn into_const(self) -> ConstVector<'a, T> {
    ConstVector { inner: self.inner }
  }

  /// 内建读写切片的借用入口 (Rust 惯用法, 便于下游迭代/排序)
  #[inline]
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    self.inner
  }

  #[inline]
  pub fn as_slice(&self) -> &[T] {
    self.inner
  }

  // ---- 共享只读访问器 (与 ConstVector 同语义) ----

  /// size()
  #[inline]
  pub fn size(&self) -> usize {
    self.inner.len()
  }

  /// length(): 需 int 返回值的场合
  /// 原版 vector.h:97 CHECK_GE(int::max, size()): release 亦执行, 超界不静默截断
  #[inline]
  pub fn length(&self) -> i32 {
    assert!(
      self.inner.len() <= i32::MAX as usize,
      "length(): 长度超出 i32 上限"
    );
    self.inner.len() as i32
  }

  /// empty()
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  /// operator[] (越界走显式返回; 原版为 DCHECK + 裸解引用)
  #[inline]
  pub fn at(&self, index: usize) -> Option<&T> {
    // 越界以 None 承接 (原版 DCHECK 路径的 Rust 化, 见模块注释)
    self.inner.get(index)
  }

  /// operator[] 可变版
  #[inline]
  pub fn at_mut(&mut self, index: usize) -> Option<&mut T> {
    debug_assert!(index < self.inner.len());
    self.inner.get_mut(index)
  }

  /// at() (原版 const T& 路径, 越界 None)
  #[inline]
  pub fn at_ref(&self, index: usize) -> Option<&T> {
    self.at(index)
  }

  /// front() (空 → None; 原版为 UB)
  #[inline]
  pub fn front(&self) -> Option<&T> {
    debug_assert!(!self.inner.is_empty());
    self.inner.first()
  }

  /// back() (空 → None)
  #[inline]
  pub fn back(&self) -> Option<&T> {
    debug_assert!(!self.inner.is_empty());
    self.inner.last()
  }

  /// begin()/data(): 原版裸指针 → 切片首元素指针
  #[inline]
  pub fn data(&self) -> *const T {
    self.inner.as_ptr()
  }

  /// begin()/end() 的 Rust 惯用等价 (range-for)
  #[inline]
  pub fn iter(&self) -> Iter<'_, T> {
    self.inner.iter()
  }

  /// rbegin()/rend()
  #[inline]
  pub fn riter(&self) -> Rev<Iter<'_, T>> {
    self.inner.iter().rev()
  }

  /// const T* cbegin() / cend() 等价只读入口
  #[inline]
  pub fn cbegin(&self) -> *const T {
    self.inner.as_ptr()
  }

  #[inline]
  pub fn cend(&self) -> *const T {
    self.inner.as_ptr().wrapping_add(self.inner.len())
  }

  /// empty Vector() = default: 空视图 (dangling 非零对齐指针, 零长度)
  #[inline]
  pub fn empty() -> Self {
    // SAFETY: 长度 0, 悬垂对齐指针不构成任何访问; 与 std span 默认构造等价
    Self {
      inner: unsafe { from_raw_parts_mut(NonNull::dangling().as_ptr(), 0) },
    }
  }

  /// operator[](index) 直取 (原版 DCHECK + 裸下标; 越界行为交调试断言)
  #[inline]
  pub fn get_unchecked(&self, index: usize) -> &T {
    debug_assert!(index < self.inner.len());
    // SAFETY: 上方 debug_assert 与原版 DCHECK_LT(index, size()) 同前提,
    // 调用方在 release 下自行保证界内 (原版 operator[] 即无检查解引用)
    unsafe { &*self.inner.as_ptr().add(index) }
  }

  /// subspan(offset, count)
  #[inline]
  pub fn subspan(self, offset: usize, count: usize) -> Vector<'a, T> {
    let len = self.inner.len();
    debug_assert!(offset <= len);
    let count = count.min(len - offset.min(len));
    Vector {
      inner: &mut self.inner[offset.min(len)..offset.min(len) + count],
    }
  }

  /// const Vector<T> operator+(size_t offset)
  #[inline]
  pub fn offset(self, offset: usize) -> Vector<'a, T> {
    self.sub_vector_from(offset)
  }

  /// Vector<T>& operator+=(size_t offset)
  #[inline]
  pub fn add_offset(&mut self, offset: usize) {
    debug_assert!(offset <= self.inner.len());
    let inner: &'a mut [T] = take(&mut self.inner);
    // SAFETY: 同 truncate, 区间收缩仍在原借用范围内
    let len = inner.len();
    self.inner = unsafe {
      from_raw_parts_mut(
        inner.as_mut_ptr().add(offset.min(len)),
        len - offset.min(len),
      )
    };
  }

  /// 元素拷贝自 other (等长; 原版 Copy(other.begin(), other.end(), begin()))
  #[inline]
  pub fn copy_from(&mut self, other: ConstVector<'_, T>)
  where
    T: Copy,
  {
    self.overwrite_with(other.as_slice());
  }

  /// 元素拷贝自不可变数组 (对齐原版 OverwriteWith(const std::array<U,n>&))
  #[inline]
  pub fn copy_from_array(&mut self, other: &[T])
  where
    T: Copy,
  {
    self.overwrite_with(other);
  }

  /// static Vector<T> cast<Vector<S>>(input): 字节重解释 (读写版)
  #[inline]
  pub fn cast<U>(self) -> Vector<'a, U> {
    let bytes = size_of_val(self.inner);
    debug_assert_eq!(bytes % size_of::<U>(), 0);
    let ptr = self.inner.as_mut_ptr().cast::<U>();
    debug_assert_eq!((ptr as usize) % align_of::<U>(), 0);
    // SAFETY: 由调用方维持原版 cast 前提 (trivial/standard layout/对齐),
    // 与 ConstVector::cast 同一约定
    Vector {
      inner: unsafe { from_raw_parts_mut(ptr, bytes / size_of::<U>()) },
    }
  }
}

/// constexpr Vector() = default
impl<T> Default for Vector<'_, T> {
  #[inline]
  fn default() -> Self {
    Self::empty()
  }
}

impl<T> Default for ConstVector<'_, T> {
  #[inline]
  fn default() -> Self {
    // SAFETY: 长度 0, 悬垂对齐指针不构成访问
    Self {
      inner: unsafe { from_raw_parts(NonNull::dangling().as_ptr(), 0) },
    }
  }
}

impl<'a, T> ConstVector<'a, T> {
  /// begin()/data(): 原版裸指针 → 切片首元素指针 (const 形态)
  #[inline]
  pub fn data(&self) -> *const T {
    self.inner.as_ptr()
  }

  /// VectorOf(const 数据)
  #[inline]
  pub fn new(data: &'a [T]) -> Self {
    Self { inner: data }
  }

  #[inline]
  pub fn sub_vector(&self, from: usize, to: usize) -> ConstVector<'a, T> {
    debug_assert!(from <= to && to <= self.inner.len());
    let to = to.min(self.inner.len());
    ConstVector {
      inner: &self.inner[from.min(to)..to],
    }
  }

  #[inline]
  pub fn sub_vector_from(&self, from: usize) -> ConstVector<'a, T> {
    debug_assert!(from <= self.inner.len());
    ConstVector {
      inner: &self.inner[from.min(self.inner.len())..],
    }
  }

  #[inline]
  pub fn first(&self, count: usize) -> ConstVector<'a, T> {
    debug_assert!(count <= self.inner.len());
    ConstVector {
      inner: &self.inner[..count.min(self.inner.len())],
    }
  }

  #[inline]
  pub fn last(&self, count: usize) -> ConstVector<'a, T> {
    let len = self.inner.len();
    debug_assert!(count <= len);
    ConstVector {
      inner: &self.inner[len - count.min(len)..],
    }
  }

  #[inline]
  pub fn size(&self) -> usize {
    self.inner.len()
  }

  #[inline]
  pub fn length(&self) -> i32 {
    debug_assert!(self.inner.len() <= i32::MAX as usize);
    self.inner.len().min(i32::MAX as usize) as i32
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[inline]
  pub fn at(&self, index: usize) -> Option<&T> {
    // 越界以 None 承接 (原版 DCHECK 路径的 Rust 化)
    self.inner.get(index)
  }

  #[inline]
  pub fn front(&self) -> Option<&T> {
    debug_assert!(!self.inner.is_empty());
    self.inner.first()
  }

  #[inline]
  pub fn back(&self) -> Option<&T> {
    debug_assert!(!self.inner.is_empty());
    self.inner.last()
  }

  #[inline]
  pub fn iter(&self) -> Iter<'_, T> {
    self.inner.iter()
  }

  #[inline]
  pub fn riter(&self) -> Rev<Iter<'_, T>> {
    self.inner.iter().rev()
  }

  #[inline]
  pub fn as_slice(&self) -> &'a [T] {
    self.inner
  }

  /// Vector<T>::cast<Vector<S>>: 字节重解释 (原版 static_assert trivial + 对齐 DCHECK)
  /// SAFETY: 调用方保证底层字节是合法的 T 序列且对齐满足 T::align
  #[inline]
  pub fn cast<U>(&self) -> ConstVector<'a, U> {
    let bytes = size_of_val(self.inner);
    debug_assert_eq!(bytes % size_of::<U>(), 0);
    let ptr = self.inner.as_ptr().cast::<U>();
    debug_assert_eq!((ptr as usize) % align_of::<U>(), 0);
    // SAFETY: 由调用方维持原版的 cast 前提 (trivial/standard layout/对齐)
    unsafe {
      ConstVector {
        inner: from_raw_parts(ptr, bytes / size_of::<U>()),
      }
    }
  }
}

/// operator== (原版 std::equal)
impl<T: PartialEq> PartialEq for ConstVector<'_, T> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.inner == other.inner
  }
}

impl<T: PartialEq> PartialEq for Vector<'_, T> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.inner == other.inner
  }
}

impl<T: PartialEq> PartialEq<ConstVector<'_, T>> for Vector<'_, T> {
  #[inline]
  fn eq(&self, other: &ConstVector<T>) -> bool {
    self.inner == other.inner
  }
}

impl<T: PartialEq> PartialEq<Vector<'_, T>> for ConstVector<'_, T> {
  #[inline]
  fn eq(&self, other: &Vector<T>) -> bool {
    self.inner == other.inner
  }
}

/// hash_value(Vector<T>): hash_range(v.begin(), v.end())
#[inline]
pub fn hash_value<T: HashValue>(v: ConstVector<'_, T>) -> usize {
  let mut hasher = Hasher::new(0);
  for t in v.iter() {
    hasher.add(t);
  }
  hasher.hash()
}

// ---------------------------------------------------------------------------
// 字符串辅助构造函数 (对应 StaticCharVector / CStrVector / StrVector / ...)
// ---------------------------------------------------------------------------

/// StaticCharVector("lit") / ArrayVector: 字面量 (不含 \0), 长度 N-1
#[inline]
pub fn static_char_vector<const N: usize>(array: &[u8; N]) -> ConstVector<'_, u8> {
  ConstVector {
    inner: &array[..N - 1],
  }
}

/// CStrVector: strlen 求长 (输入须以 0 结尾)
#[inline]
pub fn cstr_vector(data: &[u8]) -> ConstVector<'_, u8> {
  let len = data.iter().position(|&c| c == 0).unwrap_or(data.len());
  ConstVector {
    inner: &data[..len],
  }
}

/// StrVector(std::string_view)
#[inline]
pub fn str_vector(s: &str) -> ConstVector<'_, u8> {
  ConstVector {
    inner: s.as_bytes(),
  }
}

/// OneByteVector(const char* data, length) — Rust 输入已是字节切片, 直接返回
#[inline]
pub fn one_byte_vector(data: &[u8]) -> ConstVector<'_, u8> {
  ConstVector { inner: data }
}

/// StaticOneByteVector
#[inline]
pub fn static_one_byte_vector<const N: usize>(array: &[u8; N]) -> ConstVector<'_, u8> {
  ConstVector {
    inner: &array[..N - 1],
  }
}

/// ArrayVector("foo") → ['f','o','o',\0] 长度 4
#[inline]
pub fn array_vector<const N: usize>(array: &[u8; N]) -> ConstVector<'_, u8> {
  ConstVector { inner: array }
}

/// VectorOf(start, size) — 由可变切片构造读写视图
#[inline]
pub fn vector_of<'a, T>(data: &'a mut [T]) -> Vector<'a, T> {
  Vector::new(data)
}
