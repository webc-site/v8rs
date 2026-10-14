//! 原版 src/zone/zone-containers.h 中 ZoneVector<T> 转写
//!
//! std::vector 的 zone 分配替代品。内存模型: [data, end) 为活元素,
//! [end, cap) 为未初始化; 移动一律用 ptr::read/write (Rust move),
//! 复制一律用 clone 写入; 被移出槽位视为 gap, 不再 drop (与 C++
//! "move 后销毁 moved-from" 等价)。
//!
//! 偏差记录 (readme): C++ 按 trivially-copyable/copy-assignable 分支特化,
//! Rust 版以 needs_drop 分支保留 memcpy 快路径, 其余用统一 move/clone。

use std::{
  cmp::Ordering,
  iter,
  mem::{self, MaybeUninit},
  ops,
  ptr::{self, NonNull},
  slice,
};

use crate::{
  error::{Error, Result},
  zone::core::Zone,
};

/// 原版 kMinCapacity
const MIN_CAPACITY: usize = 2;

/// 原版 ZoneVector<T>
pub struct ZoneVector<'z, 'a, T> {
  zone: &'z Zone<'a>,
  data: *mut T,
  end: *mut T,
  cap: *mut T,
}

impl<'z, 'a, T> ZoneVector<'z, 'a, T> {
  /// 原版 explicit ZoneVector(Zone*)
  pub fn new(zone: &'z Zone<'a>) -> ZoneVector<'z, 'a, T> {
    ZoneVector {
      zone,
      data: ptr::null_mut(),
      end: ptr::null_mut(),
      cap: ptr::null_mut(),
    }
  }

  /// 原版 zone()
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  /// 原版 size()
  pub fn size(&self) -> usize {
    // SAFETY: [data, end) 同源且 end >= data (构造不变量)
    unsafe { self.end.offset_from(self.data) as usize }
  }

  /// 原版 capacity()
  pub fn capacity(&self) -> usize {
    // SAFETY: [data, cap) 同源且 cap >= data
    unsafe { self.cap.offset_from(self.data) as usize }
  }

  /// 原版 empty()
  pub fn is_empty(&self) -> bool {
    self.size() == 0
  }

  /// 原版 data()
  pub fn as_slice(&self) -> &[T] {
    // SAFETY: [data, end) 全部为活 T
    unsafe { slice::from_raw_parts(self.data, self.size()) }
  }

  /// 原版 data() 可变形态
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    // SAFETY: 同上, 且持有 &mut self
    unsafe { slice::from_raw_parts_mut(self.data, self.size()) }
  }

  /// 原版 at()/operator[]
  pub fn at(&self, pos: usize) -> &T {
    debug_assert!(pos < self.size());
    // SAFETY: pos 在 [0, size) 内
    unsafe { &*self.data.add(pos) }
  }

  /// 原版 at() 可变形态
  pub fn at_mut(&mut self, pos: usize) -> &mut T {
    debug_assert!(pos < self.size());
    // SAFETY: pos 在 [0, size) 内
    unsafe { &mut *self.data.add(pos) }
  }

  /// 原版 front()
  pub fn front(&self) -> Option<&T> {
    if self.is_empty() {
      None
    } else {
      Some(self.at(0))
    }
  }

  /// 原版 back()
  pub fn back(&self) -> Option<&T> {
    if self.is_empty() {
      None
    } else {
      Some(self.at(self.size() - 1))
    }
  }

  /// 原版 begin()/end() 迭代
  pub fn iter(&self) -> slice::Iter<'_, T> {
    self.as_slice().iter()
  }

  /// 原版 rbegin()/rend() 反向迭代
  pub fn iter_rev(&self) -> iter::Rev<slice::Iter<'_, T>> {
    self.as_slice().iter().rev()
  }

  /// 原版 NewCapacity(minimum): 高水位倍增
  fn new_capacity(&self, minimum: usize) -> usize {
    let doubled = if ptr::eq(self.data, self.cap) {
      MIN_CAPACITY
    } else {
      self.capacity().wrapping_mul(2)
    };
    if doubled < minimum { minimum } else { doubled }
  }

  /// 原版 Grow(minimum)
  fn grow(&mut self, minimum: usize) -> Result<()> {
    let old_data = self.data;
    let old_size = self.size();
    let old_cap = self.capacity();
    let new_capacity = self.new_capacity(minimum);
    let storage = self.zone.allocate_array::<T>(new_capacity)?;
    let new_data = storage.as_mut_ptr().cast::<T>();
    self.data = new_data;
    self.end = new_data.wrapping_add(old_size);
    if !old_data.is_null() {
      // SAFETY: 旧块 [old_data, old_data+old_size) 为活 T; 新块独立且足够大。
      // 逐项 move 后旧槽位成为 gap, 不再 drop (等价 C++ move 后销毁 moved-from)
      unsafe {
        if mem::needs_drop::<T>() {
          for i in 0..old_size {
            let value = ptr::read(old_data.add(i));
            ptr::write(new_data.add(i), value);
          }
        } else {
          ptr::copy_nonoverlapping(old_data, new_data, old_size);
        }
      }
      self.zone.delete(old_data, old_cap);
    }
    self.cap = new_data.wrapping_add(new_capacity);
    Ok(())
  }

  /// 原版 EnsureCapacity(minimum)
  pub fn reserve(&mut self, minimum: usize) -> Result<()> {
    if self.capacity() < minimum {
      self.grow(minimum)?;
    }
    Ok(())
  }

  /// 原版 EnsureOneMoreCapacity
  #[inline]
  fn ensure_one_more(&mut self) -> Result<()> {
    if ptr::eq(self.end, self.cap) {
      let need = self.capacity() + 1;
      self.grow(need)?;
    }
    Ok(())
  }

  /// 原版 push_back(T&&)/emplace: 接管值所有权
  pub fn push_back(&mut self, value: T) -> Result<()> {
    self.ensure_one_more()?;
    // SAFETY: ensure 后 end < cap, 槽位未初始化
    unsafe { ptr::write(self.end, value) };
    self.end = self.end.wrapping_add(1);
    Ok(())
  }

  /// 原版 emplace_back: 闭包惰性构造 (仅扩容成功后写入)
  pub fn emplace_back<F: FnOnce() -> T>(&mut self, f: F) -> Result<&mut T> {
    self.ensure_one_more()?;
    let slot = self.end;
    // SAFETY: 槽位未初始化, 先写后推进 (原版 push_back 语义)
    unsafe { ptr::write(slot, f()) };
    self.end = slot.wrapping_add(1);
    // SAFETY: 刚写入的活元素
    Ok(unsafe { &mut *slot })
  }

  /// 原版 pop_back()
  pub fn pop_back(&mut self) -> Option<T> {
    if self.is_empty() {
      return None;
    }
    self.end = self.end.wrapping_sub(1);
    // SAFETY: 回收末尾活元素
    Some(unsafe { ptr::read(self.end) })
  }

  /// 原版 clear(): 析构全部元素, 保留存储
  pub fn clear(&mut self) {
    self.drop_range(0, self.size());
    self.end = self.data;
  }

  /// 原版 resize(new_size) 默认值形态
  pub fn resize_default(&mut self, new_size: usize) -> Result<()>
  where
    T: Default,
  {
    self.reserve(new_size)?;
    let old_size = self.size();
    if new_size > old_size {
      for i in old_size..new_size {
        // SAFETY: reserve 保证 [old_size, new_size) 槽位未初始化
        unsafe { ptr::write(self.data.add(i), T::default()) };
      }
    } else {
      self.drop_range(new_size, old_size);
    }
    self.end = self.data.wrapping_add(new_size);
    Ok(())
  }

  /// 原版 resize(new_size, value)
  pub fn resize_with(&mut self, new_size: usize, value: &T) -> Result<()>
  where
    T: Clone,
  {
    self.reserve(new_size)?;
    let old_size = self.size();
    if new_size > old_size {
      for i in old_size..new_size {
        unsafe { ptr::write(self.data.add(i), value.clone()) };
      }
    } else {
      self.drop_range(new_size, old_size);
    }
    self.end = self.data.wrapping_add(new_size);
    Ok(())
  }

  /// 原版 assign(new_size, value)
  pub fn assign(&mut self, new_size: usize, value: &T) -> Result<()>
  where
    T: Clone,
  {
    if self.capacity() >= new_size {
      let old_size = self.size();
      let assignable = old_size.min(new_size);
      for i in 0..assignable {
        // SAFETY: [0, assignable) 槽位为活元素, 先析构后写入 (CopyingOverwrite)
        unsafe {
          ptr::drop_in_place(self.data.add(i));
          ptr::write(self.data.add(i), value.clone());
        }
      }
      for i in assignable..new_size {
        // SAFETY: [assignable, new_size) 为未初始化或 gap 槽位
        unsafe { ptr::write(self.data.add(i), value.clone()) };
      }
      self.drop_range(new_size, old_size);
      self.end = self.data.wrapping_add(new_size);
    } else {
      self.clear();
      self.grow(new_size)?;
      for i in 0..new_size {
        unsafe { ptr::write(self.data.add(i), value.clone()) };
      }
      self.end = self.data.wrapping_add(new_size);
    }
    Ok(())
  }

  /// 原版 insert(pos, InputIt first, last): 切片源 (随机访问迭代器等价)
  pub fn insert_slice(&mut self, index: usize, src: &[T]) -> Result<()>
  where
    T: Clone,
  {
    let count = src.len();
    debug_assert!(index <= self.size());
    if count == 0 {
      return Ok(());
    }
    self.prepare_for_insertion(index, count)?;
    for (k, item) in src.iter().enumerate() {
      // SAFETY: prepare 后 [index, index+count) 为 gap/未初始化槽位, 可写
      unsafe { ptr::write(self.data.add(index + k), item.clone()) };
    }
    Ok(())
  }

  /// 原版 insert(pos, count, value)
  pub fn insert_repeat(&mut self, index: usize, count: usize, value: &T) -> Result<()>
  where
    T: Clone,
  {
    debug_assert!(index <= self.size());
    if count == 0 {
      return Ok(());
    }
    self.prepare_for_insertion(index, count)?;
    for k in 0..count {
      unsafe { ptr::write(self.data.add(index + k), value.clone()) };
    }
    Ok(())
  }

  /// 原版 insert(pos, 1, value): 单值插入 (move 语义, 无需 Clone)
  pub fn insert(&mut self, index: usize, value: T) -> Result<()> {
    debug_assert!(index <= self.size());
    self.prepare_for_insertion(index, 1)?;
    // SAFETY: prepare 后 index 槽位为 gap/未初始化
    unsafe { ptr::write(self.data.add(index), value) };
    Ok(())
  }

  /// 原版 emplace(pos, args...)
  pub fn emplace(&mut self, index: usize, value: T) -> Result<&mut T> {
    self.prepare_for_insertion(index, 1)?;
    let slot = self.data.wrapping_add(index);
    unsafe { ptr::write(slot, value) };
    // SAFETY: 刚写入的活元素
    Ok(unsafe { &mut *slot })
  }

  /// 原版 PrepareForInsertion 的 Rust 等价实现:
  /// 保证插入区间 [index, index+count) 全部为可写槽位 (gap 或未初始化),
  /// 原有元素全部迁移到 index+count 之后
  fn prepare_for_insertion(&mut self, index: usize, count: usize) -> Result<()> {
    let old_size = self.size();
    debug_assert!(index <= old_size);
    let to_shift = old_size - index;
    let target_cap = old_size.checked_add(count).ok_or(Error::SizeOverflow)?;
    if self.capacity() < target_cap {
      // 容量不足: 高水位扩块, 迁移 [0,index) 与 [index,old) 两段
      let old_data = self.data;
      let old_cap = self.capacity();
      let new_capacity = self.new_capacity(target_cap);
      let storage = self.zone.allocate_array::<T>(new_capacity)?;
      let new_data = storage.as_mut_ptr().cast::<T>();
      if !old_data.is_null() {
        // SAFETY: 新旧块互不重叠; 旧块 [0, index) 与 [index, old_size) 均为活元素,
        // 新块槽位均未初始化。无 Drop 类型直接位复制 (bit-copy 无所有权歧义);
        // 有 Drop 类型逐项 read+write 迁移, 旧槽位成 gap 不再 drop
        // (等价 C++ "move 后销毁 moved-from")。
        unsafe {
          if mem::needs_drop::<T>() {
            for i in 0..index {
              let value = ptr::read(old_data.add(i));
              ptr::write(new_data.add(i), value);
            }
            for i in index..old_size {
              let value = ptr::read(old_data.add(i));
              ptr::write(new_data.add(i + count), value);
            }
          } else {
            ptr::copy_nonoverlapping(old_data, new_data, index);
            ptr::copy_nonoverlapping(old_data.add(index), new_data.add(index + count), to_shift);
          }
        }
        self.zone.delete(old_data, old_cap);
      }
      self.data = new_data;
      self.cap = new_data.wrapping_add(new_capacity);
      self.end = new_data.wrapping_add(old_size + count);
      return Ok(());
    }
    // 容量足够: 从后向前整体右移 to_shift 个 (read+write = move, 源槽位 gap 化)
    for i in (0..to_shift).rev() {
      // SAFETY: 目标 i+index+count > 源 i+index, 区间不重叠;
      // 降序遍历保证未迁移元素不被覆盖
      unsafe {
        let value = ptr::read(self.data.add(index + i));
        ptr::write(self.data.add(index + i + count), value);
      }
    }
    self.end = self.end.wrapping_add(count);
    Ok(())
  }

  /// 原版 erase(pos): 删除单元素, 返回被删值。
  /// D7 修复: index == size()/越界在 release 构建下原先会对未初始化/
  /// 越界槽位 ptr::read, 现收紧为 None (debug_assert 语义提升为运行时防护)
  pub fn erase(&mut self, index: usize) -> Option<T> {
    if index >= self.size() {
      return None;
    }
    self.erase_range(index..index + 1)
  }

  /// 原版 erase(first, last): 删除区间, 返回首个被删值 (区间为空返回 None)
  pub fn erase_range(&mut self, range: ops::Range<usize>) -> Option<T> {
    let old_size = self.size();
    let (start, end_pos) = (range.start, range.end);
    debug_assert!(start <= end_pos && end_pos <= old_size);
    if start == end_pos {
      return None;
    }
    let count = end_pos - start;
    // 先取出返回值 (erase 后仍为活元素, 用 read 复制所有权)
    // SAFETY: start 在活区间
    let removed = unsafe { ptr::read(self.data.add(start)) };
    // 析构其余被删元素
    for i in start + 1..end_pos {
      // SAFETY: [start+1, end_pos) 为活元素
      unsafe { ptr::drop_in_place(self.data.add(i)) };
    }
    // 左侧压缩: [end_pos, old) move 到 [start, old-count)
    for i in end_pos..old_size {
      // SAFETY: 目标 i-count < 源 i, 区间不重叠且升序遍历安全
      unsafe {
        let value = ptr::read(self.data.add(i));
        ptr::write(self.data.add(i - count), value);
      }
    }
    self.end = self.end.wrapping_sub(count);
    Some(removed)
  }

  /// 原版 swap
  pub fn swap(&mut self, other: &mut ZoneVector<'z, 'a, T>) {
    debug_assert!(ptr::eq(self.zone, other.zone));
    mem::swap(&mut self.data, &mut other.data);
    mem::swap(&mut self.end, &mut other.end);
    mem::swap(&mut self.cap, &mut other.cap);
  }

  /// 原版 Release() &&: 交出存储所有权 (zone 内存保留, 元素不再由本向量析构)
  pub fn release(self) -> (NonNull<T>, usize) {
    let size = self.size();
    let ptr = NonNull::new(self.data).unwrap_or_else(|| {
      // SAFETY: 空向量的 dangling 指针非空且对齐正确, 仅作为占位句柄
      unsafe { NonNull::new_unchecked(ptr::dangling_mut::<T>()) }
    });
    mem::forget(self);
    (ptr, size)
  }

  /// 析构 [from, to) 槽位的活元素
  fn drop_range(&mut self, from: usize, to: usize) {
    for i in from..to {
      // SAFETY: [from, to) 为活元素区间
      unsafe { ptr::drop_in_place(self.data.add(i)) };
    }
  }
}

impl<'z, 'a, T> ZoneVector<'z, 'a, T>
where
  T: Clone,
{
  /// 原版 ZoneVector(size, T def, Zone*)
  pub fn with_value(zone: &'z Zone<'a>, size: usize, def: &T) -> Result<ZoneVector<'z, 'a, T>> {
    let mut vector = Self::new(zone);
    if size > 0 {
      vector.grow_exact(size)?;
      for i in 0..size {
        unsafe { ptr::write(vector.data.add(i), def.clone()) };
      }
      vector.end = vector.data.wrapping_add(size);
    }
    Ok(vector)
  }

  /// 原版 ZoneVector(initializer_list, Zone*)
  pub fn from_slice(zone: &'z Zone<'a>, src: &[T]) -> Result<ZoneVector<'z, 'a, T>> {
    let mut vector = Self::new(zone);
    if !src.is_empty() {
      // 原版 zone-containers.h:87-96 直接 AllocateArray(size), 容量恰为 size
      // (不经高水位倍增, 与 R3-4/R3-5 同口径)
      vector.grow_exact(src.len())?;
      for (i, item) in src.iter().enumerate() {
        // SAFETY: grow_exact 后前 src.len() 槽位未初始化
        unsafe { ptr::write(vector.data.add(i), item.clone()) };
      }
      vector.end = vector.data.wrapping_add(src.len());
    }
    Ok(vector)
  }

  /// 原版 ZoneVector(It first, It last) 非随机访问形态: 逐个 push
  pub fn from_iter<I: Iterator<Item = T>>(
    zone: &'z Zone<'a>,
    iter: I,
  ) -> Result<ZoneVector<'z, 'a, T>> {
    let mut vector = Self::new(zone);
    for item in iter {
      vector.push_back(item)?;
    }
    Ok(vector)
  }

  /// 原版 operator=(const ZoneVector&) 的显式形态 (跨 zone 时在本 zone 复制)
  pub fn assign_from(&mut self, other: &ZoneVector<'_, '_, T>) -> Result<()> {
    if ptr::eq(self, other) {
      return Ok(());
    }
    // 原版 zone-containers.h:133: 容量足够且同 zone 才走就地覆盖分支;
    // 跨 zone 与原版一致落入 realloc 分支 (容量外部可观察)
    let same_zone = ptr::eq(
      ptr::from_ref(self.zone).cast::<()>(),
      ptr::from_ref(other.zone).cast::<()>(),
    );
    if self.capacity() >= other.size() && same_zone {
      let mut src = 0usize;
      let common = self.size().min(other.size());
      while src < common {
        // SAFETY: [0, common) 双方均为活元素, 覆盖写入 = drop + clone
        unsafe {
          ptr::drop_in_place(self.data.add(src));
          ptr::write(self.data.add(src), other.at(src).clone());
        }
        src += 1;
      }
      while src < other.size() {
        self.push_back(other.at(src).clone())?;
        src += 1;
      }
      let old_size = self.size();
      if old_size > other.size() {
        self.drop_range(other.size(), old_size);
        self.end = self.data.wrapping_add(other.size());
      }
      Ok(())
    } else {
      self.clear();
      // 原版 zone-containers.h:151-152: 析构元素后 DeleteArray 旧存储
      // (freed_size_for_tracing 记账, 旧块记账不得遗漏)
      let (old_data, old_cap) = (self.data, self.capacity());
      if !old_data.is_null() {
        self.zone.delete(old_data, old_cap);
      }
      if other.capacity() > 0 {
        // 原版 zone-containers.h:167-170 realloc 分支按 other.capacity() 精确
        // 分配 (AllocateArray 直配, 不走高水位倍增), capacity 外部可观察
        self.grow_exact(other.capacity())?;
        for i in 0..other.size() {
          unsafe { ptr::write(self.data.add(i), other.at(i).clone()) };
        }
        self.end = self.data.wrapping_add(other.size());
      } else {
        // 原版 data_ = nullptr (该分支原版不可达: capacity < size 蕴含
        // other.size() > 0; 防御性对齐 C++ 赋值面)
        self.data = ptr::null_mut();
        self.end = ptr::null_mut();
        self.cap = ptr::null_mut();
      }
      Ok(())
    }
  }
}

impl<'z, 'a, T: Default> ZoneVector<'z, 'a, T> {
  /// 原版 ZoneVector(size, Zone*): 默认构造填充。
  /// 原版 zone-containers.h:64-69 直接 AllocateArray(size), 容量恰为 size
  /// (不经高水位倍增); grow_exact 与该口径一致
  pub fn with_default(zone: &'z Zone<'a>, size: usize) -> Result<ZoneVector<'z, 'a, T>> {
    let mut vector = Self::new(zone);
    if size > 0 {
      vector.grow_exact(size)?;
      for i in 0..size {
        // SAFETY: grow_exact 后 [0, size) 槽位未初始化
        unsafe { ptr::write(vector.data.add(i), T::default()) };
      }
      vector.end = vector.data.wrapping_add(size);
    }
    Ok(vector)
  }
}

impl<T: Clone> Clone for ZoneVector<'_, '_, T> {
  /// 原版拷贝构造 (同 zone)
  fn clone(&self) -> Self {
    let mut copy = ZoneVector {
      zone: self.zone,
      data: ptr::null_mut(),
      end: ptr::null_mut(),
      cap: ptr::null_mut(),
    };
    let cap = self.capacity();
    if cap > 0 {
      let size = self.size();
      // clone 无法传播分配错误: 与原版一致视分配失败为致命, 容量取源向量 capacity
      if copy.grow_exact(cap).is_err() {
        // 分配失败保持空向量 (禁 panic 偏差, 见 readme)
        return copy;
      }
      for i in 0..size {
        // SAFETY: grow_exact 后 [0, cap) 未初始化, 写入前 size 项
        unsafe { ptr::write(copy.data.add(i), self.at(i).clone()) };
      }
      copy.end = copy.data.wrapping_add(size);
    }
    copy
  }
}

impl<T> ZoneVector<'_, '_, T> {
  /// 精确容量扩容 (copy 路径专用, 不做倍增)
  fn grow_exact(&mut self, capacity: usize) -> Result<()> {
    debug_assert!(ptr::eq(self.data, self.end));
    let storage: &mut [MaybeUninit<T>] = self.zone.allocate_array::<T>(capacity)?;
    let new_data = storage.as_mut_ptr().cast::<T>();
    self.data = new_data;
    self.end = new_data;
    self.cap = new_data.wrapping_add(capacity);
    Ok(())
  }
}

impl<T: PartialEq> PartialEq for ZoneVector<'_, '_, T> {
  /// 原版 operator==
  fn eq(&self, other: &Self) -> bool {
    self.as_slice() == other.as_slice()
  }
}

impl<T: Eq> Eq for ZoneVector<'_, '_, T> {}

impl<T: PartialOrd> PartialOrd for ZoneVector<'_, '_, T> {
  /// 原版 operator< (字典序)
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.as_slice().partial_cmp(other.as_slice())
  }
}

impl<T: Ord> Ord for ZoneVector<'_, '_, T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.as_slice().cmp(other.as_slice())
  }
}

impl<'z, 'a, T> Drop for ZoneVector<'z, 'a, T> {
  /// 原版 ~ZoneVector(): 析构元素并归还存储 (仅记账)
  fn drop(&mut self) {
    let size = self.size();
    for i in 0..size {
      // SAFETY: [0, size) 为活元素
      unsafe { ptr::drop_in_place(self.data.add(i)) };
    }
    let cap = self.capacity();
    if !self.data.is_null() && cap > 0 {
      self.zone.delete(self.data, cap);
    }
  }
}
