//! v8/src/base/small-vector.h 的 1:1 转写
//! 内联存储优先 (Inline 段), 溢出切换动态存储 (Heap 段), 对应原版
//! `begin_/end_/end_of_storage_ + inline_storage_` 三段指针布局。
//! C++ Allocator 模板形参 → Rust 统一走全局分配器 + `crate::free_deleter::Deleter`
//! 语义位 (堆段生命周期由本类型独占管理, Drop 释放, 与 std::allocator 行为等价)。
//! 与原版差异 (登记): C++ 移动会析构 moved-from 元素, Rust 移动即所有权转移
//! (memcpy 语义), 不重复跑析构; 可观察的 size/capacity 状态机保持一致。

use std::{
  alloc::{Layout, alloc, dealloc, handle_alloc_error},
  fmt::{self, Debug, Formatter},
  iter::Rev,
  mem::{MaybeUninit, forget, replace, size_of},
  ptr::{self, NonNull},
  slice,
};

use crate::vector::{ConstVector, Vector};

/// 存储两段布局 (原版 inline_storage_ 与 allocator 动态段的互斥状态)
enum Storage<T, const K: usize> {
  /// 内联段: buf 固定 K 槽, len 为已初始化前缀
  Inline {
    len: usize,
    buf: [MaybeUninit<T>; K],
  },
  /// 动态段: ptr 指向 cap 槽的堆分配 (容量恒为 2 的幂, 见 Grow)
  Heap {
    len: usize,
    cap: usize,
    ptr: NonNull<T>,
  },
}

/// 原版 `SmallVector<T, kSize>`; kInlineSize = K
pub struct SmallVector<T, const K: usize> {
  storage: Storage<T, K>,
}

impl<T, const K: usize> SmallVector<T, K> {
  /// 编译期内联容量 (kInlineSize)
  pub const K_INLINE_SIZE: usize = K;

  /// SmallVector() = default
  #[inline]
  pub fn new() -> Self {
    Self {
      storage: Storage::Inline {
        len: 0,
        buf: [const { MaybeUninit::uninit() }; K],
      },
    }
  }

  // ---- 内部指针/长度工具 (begin_/end_/end_of_storage_ 的等价拆解) ----

  #[inline]
  fn len(&self) -> usize {
    match &self.storage {
      Storage::Inline { len, .. } => *len,
      Storage::Heap { len, .. } => *len,
    }
  }

  #[inline]
  fn set_len(&mut self, n: usize) {
    match &mut self.storage {
      Storage::Inline { len, .. } => {
        debug_assert!(*len <= K && n <= K);
        *len = n;
      }
      Storage::Heap { len, cap, .. } => {
        debug_assert!(n <= *cap);
        *len = n;
      }
    }
  }

  /// begin_ (可能悬垂但永不解引用零长度)
  #[inline]
  fn as_mut_ptr(&mut self) -> *mut T {
    match &mut self.storage {
      Storage::Inline { buf, .. } => buf.as_mut_ptr().cast::<T>(),
      Storage::Heap { ptr, .. } => ptr.as_ptr(),
    }
  }

  #[inline]
  fn as_const_ptr(&self) -> *const T {
    match &self.storage {
      Storage::Inline { buf, .. } => buf.as_ptr().cast::<T>(),
      Storage::Heap { ptr, .. } => ptr.as_ptr() as *const T,
    }
  }

  /// is_big(): 是否已切换动态存储
  #[inline]
  pub fn is_big(&self) -> bool {
    matches!(self.storage, Storage::Heap { .. })
  }

  /// size_t capacity() = end_of_storage_ - begin_
  #[inline]
  pub fn capacity(&self) -> usize {
    match &self.storage {
      Storage::Inline { .. } => K,
      Storage::Heap { cap, .. } => *cap,
    }
  }

  /// size_t size()
  #[inline]
  pub fn size(&self) -> usize {
    self.len()
  }

  /// bool empty()
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// const T* data()
  #[inline]
  pub fn data(&self) -> *const T {
    self.as_const_ptr()
  }

  /// as_slice()/begin()..end() (原版 const T* begin/end)
  #[inline]
  pub fn as_slice(&self) -> &[T] {
    let len = self.len();
    // SAFETY: 不变量 1: [begin_, end_) 恒已初始化 (见原版注释)
    unsafe { slice::from_raw_parts(self.as_const_ptr(), len) }
  }

  /// begin()/end() 可变版
  #[inline]
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    let len = self.len();
    // SAFETY: 不变量 1: [begin_, end_) 恒已初始化
    unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), len) }
  }

  /// ToVector 语义: 原版 `base::Vector<const T>(data(), size())` 只读视图
  #[inline]
  pub fn as_vector(&self) -> ConstVector<'_, T> {
    ConstVector::new(self.as_slice())
  }

  /// 读写版 Vector<T> 视图
  #[inline]
  pub fn as_mut_vector(&mut self) -> Vector<'_, T> {
    Vector::new(self.as_mut_slice())
  }

  /// T& front() (空 → None; 原版 DCHECK)
  #[inline]
  pub fn front(&self) -> Option<&T> {
    self.as_slice().first()
  }

  /// T& back()
  #[inline]
  pub fn back(&self) -> Option<&T> {
    self.as_slice().last()
  }

  /// T& at(size_t index)
  #[inline]
  pub fn at(&self, index: usize) -> Option<&T> {
    self.as_slice().get(index)
  }

  /// T& at(size_t index) 可变版
  #[inline]
  pub fn at_mut(&mut self, index: usize) -> Option<&mut T> {
    self.as_mut_slice().get_mut(index)
  }

  // ---- 增长 (原版 Grow) ----

  /// Grow(min_capacity): 容量翻倍并向上取 2 的幂, 至少 min_capacity
  /// (原版 std::bit_ceil(std::max(min_capacity, 2 * capacity())))
  fn grow(&mut self, min_capacity: usize) {
    let in_use = self.len();
    // 容量溢出 (bit_ceil 无表示 / 布局超地址空间): 原版为 FatalOOM abort,
    // Rust 禁 panic → 走 handle_alloc_error 同款终止路径
    let new_capacity = match (min_capacity.max(2 * self.capacity())).checked_next_power_of_two() {
      Some(cap) => cap,
      None => oom_abort(),
    };
    let layout = match Layout::array::<T>(new_capacity) {
      Ok(layout) => layout,
      Err(_) => oom_abort(),
    };
    let new_ptr = if size_of::<T>() == 0 {
      // ZST: 无需真实分配, 悬垂对齐指针即可 (容量语义由 cap 承载)
      NonNull::dangling()
    } else {
      // SAFETY: layout 非零大小; 分配失败走 handle_alloc_error (原版 FatalOOM 对应路径)
      let p = unsafe { alloc(layout) }.cast::<T>();
      match NonNull::new(p) {
        Some(n) => n,
        None => handle_alloc_error(layout),
      }
    };
    // SAFETY: 原区间 [0, in_use) 已初始化; 新段容量 >= in_use (bit_ceil 下界);
    // 两段不重叠 (新段刚分配); 按位搬移 = Rust 移动语义, 源段不再析构
    unsafe {
      let src = self.as_const_ptr();
      ptr::copy_nonoverlapping(src, new_ptr.as_ptr(), in_use);
    }
    let old = replace(
      &mut self.storage,
      Storage::Heap {
        len: in_use,
        cap: new_capacity,
        ptr: new_ptr,
      },
    );
    // 释放旧堆段 (元素已按位搬走, 只回收内存, 不再跑析构)
    if let Storage::Heap { cap, ptr, .. } = old {
      free_heap::<T>(cap, ptr);
    }
  }

  /// reserve(size_t new_capacity)
  #[inline]
  pub fn reserve(&mut self, new_capacity: usize) {
    if new_capacity > self.capacity() {
      self.grow(new_capacity);
    }
  }

  // ---- 构造族 ----

  /// SmallVector(size) — default 初始化 (原版要求 default_initializable)
  pub fn with_size(size: usize) -> Self
  where
    T: Default,
  {
    let mut v = Self::new();
    v.resize(size);
    v
  }

  /// SmallVector(size, initial_value)
  pub fn with_size_init(size: usize, initial_value: &T) -> Self
  where
    T: Clone,
  {
    let mut v = Self::new();
    v.resize_with_value(size, initial_value);
    v
  }

  /// SmallVector(base::Vector<const T> init) — 拷贝构造
  pub fn from_slice(init: &[T]) -> Self
  where
    T: Clone,
  {
    let mut v = Self::new();
    v.reserve_exact_storage(init.len());
    v.extend_from_slice(init);
    v
  }

  /// 精确容量预留 (构造路径用; 公开 reserve 保持原版翻倍策略)
  fn reserve_exact_storage(&mut self, min_capacity: usize) {
    if min_capacity > self.capacity() {
      self.grow(min_capacity);
    }
  }

  /// 批量追加 (insert-end 路径)
  pub fn extend_from_slice(&mut self, src: &[T])
  where
    T: Clone,
  {
    if src.is_empty() {
      return;
    }
    let len = self.len();
    if len + src.len() > self.capacity() {
      self.grow(len + src.len());
    }
    // SAFETY: grow 后容量 >= len + src.len(), [len, len+src.len()) 未初始化槽
    // 写入 clone; src 为外部借用, 与 self 不重叠 (借用检查保证)
    unsafe {
      let p = self.as_mut_ptr().add(len);
      for (i, v) in src.iter().enumerate() {
        ptr::write(p.add(i), v.clone());
      }
      self.set_len(len + src.len());
    }
  }

  // ---- 修改族 ----

  /// push_back(T x): 满则 Grow 后放入
  #[inline]
  pub fn push_back(&mut self, value: T) {
    if self.len() == self.capacity() {
      self.grow(0);
    }
    let len = self.len();
    // SAFETY: len < capacity, 槽位 [len] 未初始化, write 移交所有权
    unsafe { self.as_mut_ptr().add(len).write(value) };
    self.set_len(len + 1);
  }

  /// emplace_back(args...): Rust 无变参转发, 直接接收构造好的元素
  #[inline]
  pub fn emplace_back(&mut self, value: T) {
    self.push_back(value);
  }

  /// clear() — 原版 destroy(begin_, end_); end_ = begin_
  pub fn clear(&mut self) {
    let len = self.len();
    if len == 0 {
      return;
    }
    // SAFETY: [0, len) 已初始化; drop_in_place 后 set_len(0) 标记全段未初始化
    unsafe { ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), len)) };
    self.set_len(0);
  }

  /// pop_back(count = 1): 自尾部销毁 count 个 (原版 end_ -= count; destroy_n)
  pub fn pop_back(&mut self, count: usize) {
    debug_assert!(count <= self.len());
    let count = count.min(self.len());
    let new_len = self.len() - count;
    // SAFETY: [new_len, len) 已初始化; drop_in_place 后该段转为未初始化,
    // set_len 收缩保证不再被读取
    unsafe {
      let p = self.as_mut_ptr().add(new_len);
      ptr::drop_in_place(ptr::slice_from_raw_parts_mut(p, count));
    }
    self.set_len(new_len);
  }

  /// insert(pos, count, value) — 索引化 (原版 T* pos 迭代器 → usize 下标)
  pub fn insert_at(&mut self, index: usize, count: usize, value: &T)
  where
    T: Clone,
  {
    debug_assert!(index <= self.len());
    let index = index.min(self.len());
    let len = self.len();
    if len + count > self.capacity() {
      self.grow(len + count);
    }
    // SAFETY: 容量充足; 尾部 move_backward 腾出 [index, index+count);
    // 源与目标不重叠 (目标整体右移 count), copy (可重叠版) 保守等价
    unsafe {
      let p = self.as_mut_ptr();
      ptr::copy(p.add(index), p.add(index + count), len - index);
      for i in 0..count {
        p.add(index + i).write(value.clone());
      }
    }
    self.set_len(len + count);
  }

  /// insert(pos, 1, value) 便捷版
  #[inline]
  pub fn insert_one(&mut self, index: usize, value: T) {
    debug_assert!(index <= self.len());
    let index = index.min(self.len());
    let len = self.len();
    if len + 1 > self.capacity() {
      self.grow(len + 1);
    }
    // SAFETY: 同 insert_at 的尾部右移路径, 单元素版免 clone
    unsafe {
      let p = self.as_mut_ptr();
      ptr::copy(p.add(index), p.add(index + 1), len - index);
      p.add(index).write(value);
    }
    self.set_len(len + 1);
  }

  /// insert(pos, It begin, It end) — 切片源
  pub fn insert_slice_at(&mut self, index: usize, src: &[T])
  where
    T: Clone,
  {
    debug_assert!(index <= self.len());
    let index = index.min(self.len());
    let len = self.len();
    if len + src.len() > self.capacity() {
      self.grow(len + src.len());
    }
    // SAFETY: 同 insert_at; src 为借用检查保证的外部不重叠源
    unsafe {
      let p = self.as_mut_ptr();
      ptr::copy(p.add(index), p.add(index + src.len()), len - index);
      ptr::copy_nonoverlapping(src.as_ptr(), p.add(index), src.len());
    }
    self.set_len(len + src.len());
  }

  /// erase(erase_start, erase_end) — 索引化; 返回新 end 位置
  /// (原版 std::move(erase_end, end_, erase_start); destroy; end_ = new_end)
  pub fn erase(&mut self, start: usize, end: usize) -> usize {
    let len = self.len();
    debug_assert!(start <= end && end <= len);
    let start = start.min(len);
    let end = end.clamp(start, len);
    let tail = len - end;
    // SAFETY: [end, len) 已初始化元素按位左移到 [start, start+tail);
    // 尾端 [start+tail, len) 转为可销毁的冗余副本后原地析构
    unsafe {
      let p = self.as_mut_ptr();
      ptr::copy(p.add(end), p.add(start), tail);
      ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
        p.add(start + tail),
        len - (start + tail),
      ));
    }
    self.set_len(start + tail);
    start
  }

  /// erase(pos) 单元素版
  #[inline]
  pub fn erase_one(&mut self, index: usize) -> usize {
    self.erase(index, index + 1)
  }

  /// resize(new_size) — 新段 default 初始化 (原版 default-initialize 语义)
  pub fn resize(&mut self, new_size: usize)
  where
    T: Default,
  {
    if new_size > self.capacity() {
      self.grow(new_size);
    }
    let len = self.len();
    // SAFETY: 容量 >= new_size; 增长段 [len, new_size) 逐槽 Default 构造,
    // 收缩段 [new_size, len) 原地析构 (原版 uninitialized_default_construct / destroy)
    unsafe {
      let p = self.as_mut_ptr();
      if new_size > len {
        for i in 0..new_size - len {
          ptr::write(p.add(len + i), T::default());
        }
      } else {
        ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
          p.add(new_size),
          len - new_size,
        ));
      }
    }
    self.set_len(new_size);
  }

  /// resize_no_init(new_size) — 原版仅允许 kHasTrivialElement (隐式生命周期类型);
  /// 新增元素不初始化, 读取前必须覆盖写入
  ///
  /// # Safety
  /// T 必须是隐式生命周期类型 (原版 is_trivially_copyable && trivially_destructible),
  /// 且调用方保证在读取任何新尾部元素前先写入
  pub unsafe fn resize_no_init(&mut self, new_size: usize) {
    if new_size > self.capacity() {
      self.grow(new_size);
    }
    // SAFETY: 调用方契约覆盖 T 的隐式生命周期要求; 仅移动 end_ 指针等价 set_len
    self.set_len(new_size);
  }

  /// 迭代入口 (begin/end)
  #[inline]
  pub fn iter(&self) -> slice::Iter<'_, T> {
    self.as_slice().iter()
  }

  /// rbegin()/rend()
  #[inline]
  pub fn riter(&self) -> Rev<slice::Iter<'_, T>> {
    self.as_slice().iter().rev()
  }

  /// 收缩回内联段 (仅当元素数 <= K; 堆段被释放) — ShrinkToFit 语义对齐
  pub fn shrink_to_fit(&mut self)
  where
    T: Copy,
  {
    if !self.is_big() || self.len() > K {
      return;
    }
    let old = replace(
      &mut self.storage,
      Storage::Inline {
        len: 0,
        buf: [const { MaybeUninit::uninit() }; K],
      },
    );
    let Storage::Heap { len, cap, ptr } = old else {
      unreachable!()
    };
    // SAFETY: [0, len) 已初始化且 T: Copy, 按位拷回内联段后释放堆段;
    // Copy 类型无自定义析构, 无双重释放风险
    unsafe {
      let dst = self.as_mut_ptr();
      ptr::copy_nonoverlapping(ptr.as_ptr(), dst, len);
      self.set_len(len);
    }
    free_heap::<T>(cap, ptr);
  }

  /// 转 std Vec (消耗自身)
  pub fn into_vec(self) -> Vec<T> {
    let mut out = Vec::with_capacity(self.len());
    // SAFETY: 逐槽 read 移交所有权, 随后 self 的 Drop 看到 len = 0
    unsafe {
      let len = self.len();
      let p = self.as_const_ptr();
      for i in 0..len {
        out.push(ptr::read(p.add(i)));
      }
      forget(self);
    }
    out
  }
}

impl<T: Clone, const K: usize> SmallVector<T, K> {
  /// resize(new_size, initial_value)
  pub fn resize_with_value(&mut self, new_size: usize, initial_value: &T) {
    if new_size > self.capacity() {
      self.grow(new_size);
    }
    let len = self.len();
    // SAFETY: 容量 >= new_size; 增长段 [len, new) 写入 clone,
    // 收缩段 [new, len) 原地析构 (原版 uninitialized_fill / destroy)
    unsafe {
      let p = self.as_mut_ptr();
      if new_size > len {
        for i in 0..new_size - len {
          ptr::write(p.add(len + i), initial_value.clone());
        }
      } else {
        ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
          p.add(new_size),
          len - new_size,
        ));
      }
    }
    self.set_len(new_size);
  }
}

/// 释放堆段 (只回收内存; 元素须已被移动或析构)
fn free_heap<T>(cap: usize, ptr: NonNull<T>) {
  if size_of::<T>() != 0 {
    // cap 来自此前同 layout 的成功分配, 重算不可能溢出; 兜底走 abort (禁 panic)
    let Ok(layout) = Layout::array::<T>(cap) else {
      oom_abort();
    };
    // SAFETY: ptr/cap 来自 grow 中同一 layout 的 alloc
    unsafe { dealloc(ptr.as_ptr().cast(), layout) };
  }
}

/// FatalOOM 对应路径: 分配规模超出地址空间时无 panic 终止
/// (handle_alloc_error 默认 handler 即 abort, 与原版 FatalOOM(OOMType::kProcess) 一致)
#[cold]
fn oom_abort() -> ! {
  handle_alloc_error(Layout::new::<u8>())
}

impl<T, const K: usize> Drop for SmallVector<T, K> {
  /// ~SmallVector() { FreeStorage(); } — 析构 [begin_, end_) 后释放堆段
  fn drop(&mut self) {
    let len = self.len();
    // SAFETY: [0, len) 已初始化 (不变量 1), 逐元素原地析构
    unsafe { ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), len)) };
    // 取出的 ptr/cap 归本 Drop 独占释放
    if let Storage::Heap { cap, ptr, .. } = &self.storage {
      free_heap::<T>(*cap, *ptr);
    }
  }
}

impl<T, const K: usize> Default for SmallVector<T, K> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

/// 移动赋值语义由 Rust move 承担; std::mem::take/replace 对应原版
/// `trg = std::move(src)` (src 复位内联, big 段指针窃取 = 位移动)
impl<T: Clone, const K: usize> Clone for SmallVector<T, K> {
  /// SmallVector(const SmallVector&) — 深拷贝 (CopySeparation 语义)
  fn clone(&self) -> Self {
    let mut out = Self::new();
    out.extend_from_slice(self.as_slice());
    out
  }
}

impl<T: PartialEq, const K: usize> PartialEq for SmallVector<T, K> {
  fn eq(&self, other: &Self) -> bool {
    self.as_slice() == other.as_slice()
  }
}

/// range-based for (const T* begin/end)
impl<'a, T, const K: usize> IntoIterator for &'a SmallVector<T, K> {
  type Item = &'a T;
  type IntoIter = slice::Iter<'a, T>;
  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

/// range-based for 可变版
impl<'a, T, const K: usize> IntoIterator for &'a mut SmallVector<T, K> {
  type Item = &'a mut T;
  type IntoIter = slice::IterMut<'a, T>;
  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.as_mut_slice().iter_mut()
  }
}

impl<T: Debug, const K: usize> Debug for SmallVector<T, K> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_list().entries(self.as_slice()).finish()
  }
}
