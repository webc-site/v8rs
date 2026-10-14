//! 原版 src/zone/zone-list.h + zone-list-inl.h 转写
//!
//! ZoneList 是可增长列表, 常数时间随机访问; 与 ZoneVector 不同,
//! 实例本身尽可能小, 适合嵌入其他 zone 对象。
//! 元素不能被逐个删除释放, 析构函数原版有意不回收 backing store,
//! 因此 Rust 版不实现 Drop (与 ~ZoneList() = default 一致)。
//!
//! 偏差记录 (readme):
//! - C++ AllocateArray 默认构造全部槽位、Add/Set 走赋值; Rust 以
//!   "[0,length) 活元素, [length,capacity) 未初始化" 不变量重写,
//!   shrink/rewind/clear 处补 drop_in_place 维持不变量 (C++ 不调析构,
//!   对 POD 行为等价)。
//! - C++ 模板对 trivially-copyable 走 memcpy; Rust 以 needs_drop 分支保留。
//! - 原版方法需显式传 Zone*; Rust 版持有 &'z Zone 引用 (多一个指针字段)。

use std::{
  cmp::{Ordering, min},
  mem, ptr, slice,
};

use crate::{
  error::{Error, Result},
  zone::core::Zone,
};

/// 原版 ZoneList<T>
pub struct ZoneList<'z, 'a, T> {
  zone: &'z Zone<'a>,
  data: *mut T,
  capacity: i32,
  length: i32,
}

impl<'z, 'a, T> ZoneList<'z, 'a, T> {
  /// 原版 ZoneList(int capacity, Zone* zone)
  pub fn new(capacity: i32, zone: &'z Zone<'a>) -> Result<ZoneList<'z, 'a, T>> {
    debug_assert!(capacity >= 0);
    let data = if capacity > 0 {
      let n = capacity as usize;
      zone.allocate_array::<T>(n)?.as_mut_ptr().cast::<T>()
    } else {
      ptr::null_mut()
    };
    Ok(ZoneList {
      zone,
      data,
      capacity,
      length: 0,
    })
  }

  /// 原版 zone() (Rust 版持有引用, 补充访问口)
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  /// 原版 length()
  pub fn length(&self) -> i32 {
    self.length
  }

  /// 原版 capacity()
  pub fn capacity(&self) -> i32 {
    self.capacity
  }

  /// 原版 is_empty()
  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  /// 原版 operator[](int i) const / at(int i)
  pub fn at(&self, index: i32) -> &T {
    debug_assert!(index >= 0 && index < self.length);
    // SAFETY: index 在活元素区间 [0, length)
    unsafe { &*self.data.add(index as usize) }
  }

  /// 原版 operator[](int i) 可变形态
  pub fn at_mut(&mut self, index: i32) -> &mut T {
    debug_assert!(index >= 0 && index < self.length);
    // SAFETY: 同上, 且持有 &mut self
    unsafe { &mut *self.data.add(index as usize) }
  }

  /// 原版 first()
  pub fn first(&self) -> &T {
    self.at(0)
  }

  /// 原版 last()
  pub fn last(&self) -> &T {
    self.at(self.length - 1)
  }

  /// 原版 ToVector()/ToConstVector()
  pub fn as_slice(&self) -> &[T] {
    // SAFETY: [0, length) 全部为活 T
    unsafe { slice::from_raw_parts(self.data, self.length as usize) }
  }

  /// 原版 ToVector() 可变形态
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    // SAFETY: 同上
    unsafe { slice::from_raw_parts_mut(self.data, self.length as usize) }
  }

  /// 原版 ToVector(int start, int length)
  pub fn to_vector(&mut self, start: i32, len: i32) -> &mut [T] {
    debug_assert!(start <= self.length);
    debug_assert!(start >= 0 && len >= 0);
    let real = min(self.length - start, len) as usize;
    // SAFETY: [start, start+real) 为活元素
    unsafe { slice::from_raw_parts_mut(self.data.add(start as usize), real) }
  }

  /// 原版 begin()/end() 迭代
  pub fn iter(&self) -> slice::Iter<'_, T> {
    self.as_slice().iter()
  }

  /// 原版 Contains(const T& elm): 需要 T 可比较 (对应 C++ operator==)
  pub fn contains(&self, elm: &T) -> bool
  where
    T: PartialEq,
  {
    self.as_slice().contains(elm)
  }

  /// 原版 Resize(int new_capacity, Zone* zone)
  fn resize(&mut self, new_capacity: i32) -> Result<()> {
    debug_assert!(self.length <= new_capacity);
    let n = new_capacity as usize;
    let new_data = self.zone.allocate_array::<T>(n)?.as_mut_ptr().cast::<T>();
    let len = self.length as usize;
    if len > 0 {
      // SAFETY: 旧块 [0, len) 为活元素; 新旧块不重叠
      unsafe {
        if mem::needs_drop::<T>() {
          for i in 0..len {
            let value = ptr::read(self.data.add(i));
            ptr::write(new_data.add(i), value);
          }
        } else {
          ptr::copy_nonoverlapping(self.data, new_data, len);
        }
      }
    }
    if !self.data.is_null() {
      self.zone.delete(self.data, self.capacity as usize);
    }
    self.data = new_data;
    self.capacity = new_capacity;
    Ok(())
  }

  /// 原版 ResizeAddInternal: 容量翻倍加一 (new_capacity = 1 + 2 * capacity_)
  fn resize_add(&mut self, element: &T) -> Result<()>
  where
    T: Clone,
  {
    debug_assert!(self.length >= self.capacity);
    // 原版 T temp = element: 元素引用可能指向旧 backing store, 先复制
    let temp = element.clone();
    let new_capacity = self
      .capacity
      .checked_mul(2)
      .and_then(|c| c.checked_add(1))
      .ok_or(Error::SizeOverflow)?;
    self.resize(new_capacity)?;
    // SAFETY: resize 后 [0, length) 活, length 槽位未初始化
    unsafe { ptr::write(self.data.add(self.length as usize), temp) };
    self.length += 1;
    Ok(())
  }

  /// 原版 Add(const T& element, Zone* zone)
  pub fn add(&mut self, element: &T) -> Result<()>
  where
    T: Clone,
  {
    if self.length < self.capacity {
      // SAFETY: [length, capacity) 槽位未初始化
      unsafe { ptr::write(self.data.add(self.length as usize), element.clone()) };
      self.length += 1;
    } else {
      self.resize_add(element)?;
    }
    Ok(())
  }

  /// 原版 AddAll(base::Vector<const T> other, Zone* zone)
  pub fn add_all_slice(&mut self, other: &[T]) -> Result<()>
  where
    T: Clone,
  {
    let len = other.len();
    if len == 0 {
      return Ok(());
    }
    let len_i = i32::try_from(len).map_err(|_| Error::SizeOverflow)?;
    let result_length = self.length.checked_add(len_i).ok_or(Error::SizeOverflow)?;
    if self.capacity < result_length {
      self.resize(result_length)?;
    }
    let start = self.length as usize;
    for (i, item) in other.iter().enumerate() {
      // SAFETY: [start, start+len) 槽位未初始化
      unsafe { ptr::write(self.data.add(start + i), item.clone()) };
    }
    self.length = result_length;
    Ok(())
  }

  /// 原版 AddAll(const ZoneList<T>& other, Zone* zone)
  pub fn add_all_list(&mut self, other: &ZoneList<'_, '_, T>) -> Result<()>
  where
    T: Clone,
  {
    self.add_all_slice(other.as_slice())
  }

  /// 原版 ZoneList(const ZoneList<T>& other, Zone* zone) 拷贝构造
  pub fn from_list(other: &ZoneList<'z, 'a, T>) -> Result<ZoneList<'z, 'a, T>>
  where
    T: Clone,
  {
    let mut list = ZoneList::new(other.length(), other.zone)?;
    list.add_all_slice(other.as_slice())?;
    Ok(list)
  }

  /// 原版 ZoneList(base::Vector<const T> other, Zone* zone)
  pub fn from_slice(zone: &'z Zone<'a>, src: &[T]) -> Result<ZoneList<'z, 'a, T>>
  where
    T: Clone,
  {
    let n = i32::try_from(src.len()).map_err(|_| Error::SizeOverflow)?;
    let mut list = ZoneList::new(n, zone)?;
    list.add_all_slice(src)?;
    Ok(list)
  }

  /// 原版 AddBlock(T value, int count, Zone* zone):
  /// 返回新增元素区间 (原版 base::Vector<T>; 引用即元素值, 偏差见 readme)
  pub fn add_block(&mut self, value: &T, count: i32) -> Result<&mut [T]>
  where
    T: Clone,
  {
    debug_assert!(count >= 0);
    let start = self.length;
    for _ in 0..count {
      self.add(value)?;
    }
    // SAFETY: 新增 [start, start+count) 为活元素
    Ok(unsafe { slice::from_raw_parts_mut(self.data.add(start as usize), count as usize) })
  }

  /// 原版 Set(int index, const T& elm): 覆盖活元素 (原版 DCHECK 允许
  /// index == length_, 但该槽位未构造, Rust 版收紧为 [0, length))
  pub fn set(&mut self, index: i32, element: &T)
  where
    T: Clone,
  {
    debug_assert!(index >= 0 && index < self.length);
    // SAFETY: 槽位为活元素, 先析构再写入 (等价 C++ 赋值)
    unsafe {
      ptr::drop_in_place(self.data.add(index as usize));
      ptr::write(self.data.add(index as usize), element.clone());
    }
  }

  /// 原版 InsertAt(int index, const T& elm, Zone* zone)
  pub fn insert_at(&mut self, index: i32, element: &T) -> Result<()>
  where
    T: Clone,
  {
    debug_assert!(index >= 0 && index <= self.length);
    let elem = element.clone();
    // 原版先 Add(elm) 再移位; Rust 版仅预留槽位, 跳过冗余尾拷贝
    if self.length < self.capacity {
      self.length += 1;
    } else {
      let new_capacity = self
        .capacity
        .checked_mul(2)
        .and_then(|c| c.checked_add(1))
        .ok_or(Error::SizeOverflow)?;
      self.resize(new_capacity)?;
      self.length += 1;
    }
    let index = index as usize;
    let old = (self.length - 1) as usize;
    if old > index {
      // SAFETY: 降序右移: [index, old) 元素整体向右移动 1 位至 [index+1, old+1);
      // 内存区间重叠, 用 ptr::copy (等价 memmove) 避免中间槽位析构 (无 double-drop)
      unsafe {
        ptr::copy(self.data.add(index), self.data.add(index + 1), old - index);
        ptr::write(self.data.add(index), elem);
      }
    } else {
      // SAFETY: index == old: 尾插
      unsafe { ptr::write(self.data.add(index), elem) };
    }
    Ok(())
  }

  /// 原版 Remove(int i): 移除第 i 个元素并左移, 返回被移值 (不 free 指针元素)
  pub fn remove(&mut self, i: i32) -> T
  where
    T: Clone,
  {
    debug_assert!(i >= 0 && i < self.length);
    let j0 = i as usize;
    // 原版 T element = at(i) 为复制
    let element = self.as_slice()[j0].clone();
    self.length -= 1;
    // SAFETY: [i, length) 左移: 槽位 i 的原值已由 element 复制代表, 析构一次;
    // 后续目标槽位持有将被覆盖的活值, 但 read+write move 链保证每值析构一次
    unsafe {
      ptr::drop_in_place(self.data.add(j0));
      let end = self.length as usize;
      let mut j = j0;
      while j < end {
        ptr::write(self.data.add(j), ptr::read(self.data.add(j + 1)));
        j += 1;
      }
    }
    element
  }

  /// 原版 RemoveLast()
  pub fn remove_last(&mut self) -> T
  where
    T: Clone,
  {
    debug_assert!(!self.is_empty());
    self.remove(self.length - 1)
  }

  /// 原版 Clear(Zone* zone): 释放存储记账并清零 (原版不调元素析构,
  /// Rust 版为维持不变量先析构活元素, 见 readme)
  pub fn clear(&mut self) {
    let len = self.length as usize;
    for i in 0..len {
      // SAFETY: [0, length) 为活元素
      unsafe { ptr::drop_in_place(self.data.add(i)) };
    }
    if !self.data.is_null() {
      self.zone.delete(self.data, self.capacity as usize);
    }
    self.drop_and_clear();
  }

  /// 原版 DropAndClear(): 仅清指针, 不 free 存储
  pub fn drop_and_clear(&mut self) {
    self.data = ptr::null_mut();
    self.capacity = 0;
    self.length = 0;
  }

  /// 原版 Rewind(int pos): 丢弃 pos 之后的元素
  pub fn rewind(&mut self, pos: i32) {
    debug_assert!(0 <= pos && pos <= self.length);
    let from = pos as usize;
    let to = self.length as usize;
    for i in from..to {
      // SAFETY: [pos, length) 为活元素, 丢弃前析构
      unsafe { ptr::drop_in_place(self.data.add(i)) };
    }
    self.length = pos;
  }

  /// 原版 Iterate(Visitor* visitor): Rust 以闭包代替 Visitor
  pub fn iterate<F: FnMut(&mut T)>(&mut self, mut visitor: F) {
    let len = self.length as usize;
    for i in 0..len {
      // SAFETY: [0, length) 为活元素
      visitor(unsafe { &mut *self.data.add(i) });
    }
  }

  /// 原版 Sort(cmp): std::sort (introsort = unstable) + DEBUG 有序断言
  pub fn sort_by<F>(&mut self, mut cmp: F)
  where
    F: FnMut(&T, &T) -> Ordering,
  {
    self.as_mut_slice().sort_unstable_by(|a, b| cmp(a, b));
    if cfg!(debug_assertions) {
      let s = self.as_slice();
      for i in 1..s.len() {
        debug_assert_ne!(cmp(&s[i], &s[i - 1]), Ordering::Less);
      }
    }
  }

  /// 原版 StableSort(cmp, start, length): std::stable_sort
  pub fn stable_sort_by<F>(&mut self, mut cmp: F, start: usize, len: usize)
  where
    F: FnMut(&T, &T) -> Ordering,
  {
    let n = self.length as usize;
    debug_assert!(start <= n && start + len <= n);
    self.as_mut_slice()[start..start + len].sort_by(|a, b| cmp(a, b));
    if cfg!(debug_assertions) {
      let s = self.as_slice();
      for i in start + 1..start + len {
        debug_assert_ne!(cmp(&s[i], &s[i - 1]), Ordering::Less);
      }
    }
  }
}
