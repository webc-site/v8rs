//! 原版 src/zone/zone-compact-set.h 转写
//!
//! Zone 分配的集合, 对 0/1 个值有紧凑编码: 空态与单值直接打包进一个
//! 指针字, >= 2 个值以 zone 内有序列表存储, 写时复制 (copy-on-write)
//! 保持 ZoneCompactSet 本体可平凡拷贝。第 2 个及以后的插入会触发分配
//! 与既有元素拷贝 -- 值个数为 0/1 的场景优先使用本结构。
//!
//! 原版 data_ = base::PointerWithPayload<void, Tag, 2> (指针 + 2 bit tag),
//! Rust 侧以 usize 打包 (存储的指针值要求 >= 4 字节对齐, 插入处断言)。
//! List 本体与元素数组均分配在 zone 内 (对应原版 NewList),
//! 集合持有的指针存活期以 Zone 本体为准 (与原版 Zone* 契约一致)。
//!
//! 偏差记录 (readme):
//! - at() 空集/越界返回 Option (原版 DCHECK + 裸引用);
//! - const_iterator 以 iter() 交出值拷贝 (T 为 Copy);
//! - 列表元素以 usize 存储类型擦除指针值 (排序/比较口径不变);
//! - ZoneHandleSet 别名依赖 Handle (L3), 未提供。

use core::hash::{Hash, Hasher};
use std::{fmt, marker::PhantomData, ptr, slice};

use crate::{error::Result, zone::core::Zone};

/// 原版 enum Tag { kSingletonTag = 0, kEmptyTag = 1, kListTag = 2 }
const SINGLETON_TAG: usize = 0;
const EMPTY_TAG: usize = 1;
const LIST_TAG: usize = 2;
/// 2 bit tag 掩码 (PointerWithPayload<void, Tag, 2>)
const TAG_MASK: usize = 0b11;

/// 原版 ZoneCompactSetTraits<T>: Handle 形态与数据指针的互转。
/// 实现者即 Handle 本体 (对应原版对 T 的特化); 原版
/// static_assert is_trivially_copyable_v / is_trivially_destructible_v
/// 由 Copy 约束表达
pub trait ZoneCompactSetTraits: Copy + PartialEq + Sized {
  /// 原版 Traits::HandleToPointer(handle)
  fn handle_to_pointer(self) -> *mut ();
  /// 原版 Traits::PointerToHandle(ptr)
  fn pointer_to_handle(ptr: *mut ()) -> Self;
}

/// 原版 List = base::Vector<data_type*>: {start_, length_} 本体分配在 zone
#[repr(C)]
struct List {
  start: *mut usize,
  length: usize,
}

/// 指针值打包 2 bit tag (要求指针 >= 4 字节对齐, 调用方 debug 断言把关)
#[inline]
const fn pack(pointer: usize, tag: usize) -> usize {
  pointer | tag
}

/// 原版 ZoneCompactSet<T>
pub struct ZoneCompactSet<H: ZoneCompactSetTraits> {
  /// 原版 data_: PointerWithPayload<void, Tag, 2>
  data: usize,
  _p: PhantomData<H>,
}

// 原版拷贝/移动构造 = default (平凡): 仅复制打包字
impl<H: ZoneCompactSetTraits> Clone for ZoneCompactSet<H> {
  fn clone(&self) -> Self {
    *self
  }
}
impl<H: ZoneCompactSetTraits> Copy for ZoneCompactSet<H> {}

impl<H: ZoneCompactSetTraits> Default for ZoneCompactSet<H> {
  /// 原版 ZoneCompactSet(): data_(kEmptyTag)
  fn default() -> Self {
    Self::new()
  }
}

impl<H: ZoneCompactSetTraits> ZoneCompactSet<H> {
  /// 原版 ZoneCompactSet(): 空集
  pub fn new() -> Self {
    ZoneCompactSet {
      data: pack(0, EMPTY_TAG),
      _p: PhantomData,
    }
  }

  /// 原版 explicit ZoneCompactSet(T handle): 单值
  pub fn from_handle(handle: H) -> Self {
    let pointer = H::handle_to_pointer(handle);
    // 暴露 raw 指针 provenance: at()/contains 的 usize→ptr 回转经
    // with_exposed_provenance 取回 (与 accounting.rs 同一 strict-provenance 机制)
    let _ = pointer.expose_provenance();
    debug_assert_eq!(
      pointer as usize & TAG_MASK,
      0,
      "handle 指针须 >= 4 字节对齐"
    );
    ZoneCompactSet {
      data: pack(pointer as usize, SINGLETON_TAG),
      _p: PhantomData,
    }
  }

  /// 原版 ZoneCompactSet(It first, last, zone) / initializer_list 形态:
  /// 0/1 值紧凑编码, 多值建表后排序
  pub fn from_iter<I>(handles: I, zone: &Zone<'_>) -> Result<Self>
  where
    I: IntoIterator<Item = H>,
    I::IntoIter: ExactSizeIterator,
  {
    let mut handles = handles.into_iter();
    let mut set = Self::new();
    match handles.len() {
      0 => {}
      1 => {
        // ExactSizeIterator: len==1 时 next 必有值
        if let Some(handle) = handles.next() {
          set = Self::from_handle(handle);
        }
      }
      size => {
        let list = Self::new_list(zone, size)?;
        // SAFETY: new_list 保证 [start, start+size) 为本 zone 分配的槽位
        let slots = unsafe { slice::from_raw_parts_mut((*list).start, size) };
        for (slot, handle) in slots.iter_mut().zip(handles) {
          let pointer = H::handle_to_pointer(handle);
          let _ = pointer.expose_provenance();
          *slot = pointer as usize;
        }
        slots.sort_unstable();
        set.data = pack(list.expose_provenance(), LIST_TAG);
      }
    }
    Ok(set)
  }

  /// 原版 Clone(Zone*): 以 begin()/end() 重建
  pub fn clone_into(&self, zone: &Zone<'_>) -> Result<Self> {
    Self::from_iter(self.iter(), zone)
  }

  /// 原版 is_empty()
  pub fn is_empty(&self) -> bool {
    self.tag() == EMPTY_TAG
  }

  /// 原版 size()
  pub fn size(&self) -> usize {
    match self.tag() {
      EMPTY_TAG => 0,
      SINGLETON_TAG => 1,
      _ => unsafe { (*self.list()).length },
    }
  }

  /// 原版 at(size_t i) / operator[]: 空集或越界返回 None
  pub fn at(&self, i: usize) -> Option<H> {
    match self.tag() {
      EMPTY_TAG => None,
      SINGLETON_TAG => (i == 0).then(|| H::pointer_to_handle(self.singleton_ptr())),
      _ => self
        .list_slice()
        .get(i)
        .map(|&pointer| H::pointer_to_handle(ptr::with_exposed_provenance_mut(pointer))),
    }
  }

  /// 原版 insert(T handle, Zone*): 去重插入, 多值路径写时复制
  pub fn insert(&mut self, handle: H, zone: &Zone<'_>) -> Result<()> {
    let value_ptr = H::handle_to_pointer(handle);
    let _ = value_ptr.expose_provenance();
    let value = value_ptr as usize;
    debug_assert_eq!(value & TAG_MASK, 0, "handle 指针须 >= 4 字节对齐");
    if self.is_empty() {
      self.data = pack(value, SINGLETON_TAG);
    } else if self.tag() == SINGLETON_TAG {
      let singleton = self.singleton();
      if singleton == value {
        return Ok(());
      }
      // 原版: 单值升列表, 两个值按序落位
      let list = Self::new_list(zone, 2)?;
      // SAFETY: 新表 [0, 2) 槽位刚分配
      unsafe {
        let (lo, hi) = if singleton < value {
          (singleton, value)
        } else {
          (value, singleton)
        };
        *(*list).start.add(0) = lo;
        *(*list).start.add(1) = hi;
      }
      self.data = pack(list.expose_provenance(), LIST_TAG);
    } else {
      let current = self.list_slice();
      self.debug_assert_sorted(current);
      // 原版 lower_bound: 有序表二分定位插入点 (其后元素严格大于 value)
      let pos = current.partition_point(|&pointer| pointer < value);
      if pos < current.len() && current[pos] == value {
        // Already in the list.
        return Ok(());
      }
      // 写时复制: data_ 的平凡拷贝不得观察到既有表变更
      let new_list = Self::new_list(zone, current.len() + 1)?;
      // SAFETY: [0, pos) 与 [pos, len) 两段整体搬入新表, 新旧表互不重叠
      unsafe {
        ptr::copy_nonoverlapping(current.as_ptr(), (*new_list).start, pos);
        *(*new_list).start.add(pos) = value;
        ptr::copy_nonoverlapping(
          current.as_ptr().add(pos),
          (*new_list).start.add(pos + 1),
          current.len() - pos,
        );
      }
      self.data = pack(new_list.expose_provenance(), LIST_TAG);
    }
    Ok(())
  }

  /// 原版 Union(other, zone)
  pub fn union(&mut self, other: &Self, zone: &Zone<'_>) -> Result<()> {
    for handle in other.iter() {
      self.insert(handle, zone)?;
    }
    Ok(())
  }

  /// 原版 contains(ZoneCompactSet const& other): 子集判定
  pub fn contains_set(&self, other: &Self) -> bool {
    if self.data == other.data {
      return true;
    }
    if self.is_empty() {
      return false;
    }
    if other.is_empty() {
      return true;
    }
    if self.tag() == SINGLETON_TAG {
      // data_ 已判不等: other 单值则必为不同值 (原版 DCHECK_IMPLIES)
      debug_assert!(other.tag() != SINGLETON_TAG || other.singleton() != self.singleton());
      return false;
    }
    let list = self.list_slice();
    self.debug_assert_sorted(list);
    if other.tag() == SINGLETON_TAG {
      // 原版 binary_search
      return list.binary_search(&other.singleton()).is_ok();
    }
    // 双侧有序: 每个候选大于上一个, 已找到的位置即下一个的下界起点
    let other_list = other.list_slice();
    self.debug_assert_sorted(other_list);
    let mut from = 0usize;
    for &pointer in other_list {
      let pos = from + list[from..].partition_point(|&p| p < pointer);
      if pos == list.len() || list[pos] != pointer {
        return false;
      }
      from = pos;
    }
    true
  }

  /// 原版 contains(T handle)
  pub fn contains(&self, handle: H) -> bool {
    if self.is_empty() {
      return false;
    }
    let pointer = H::handle_to_pointer(handle) as usize;
    if self.tag() == SINGLETON_TAG {
      return self.singleton() == pointer;
    }
    let list = self.list_slice();
    self.debug_assert_sorted(list);
    list.binary_search(&pointer).is_ok()
  }

  /// 原版 remove(T handle, Zone*): 不存在时无操作;
  /// 2 值列表降回单值 (保证比较口径, 永不降到 0)
  pub fn remove(&mut self, handle: H, zone: &Zone<'_>) -> Result<()> {
    if self.is_empty() {
      return Ok(());
    }
    let pointer = H::handle_to_pointer(handle) as usize;
    if self.tag() == SINGLETON_TAG {
      if self.singleton() == pointer {
        self.data = pack(0, EMPTY_TAG);
      }
      return Ok(());
    }
    let current = self.list_slice();
    self.debug_assert_sorted(current);
    // 原版 lower_bound 定位
    let pos = current.partition_point(|&p| p < pointer);
    if pos == current.len() || current[pos] != pointer {
      // Not in the list.
      return Ok(());
    }
    debug_assert!(current.len() >= 2);
    if current.len() == 2 {
      // Drop back down to singleton mode (剔除后余一值)
      let other_value = current[1 - pos];
      self.data = pack(other_value, SINGLETON_TAG);
      return Ok(());
    }
    // 写时复制剔除
    let new_list = Self::new_list(zone, current.len() - 1)?;
    // SAFETY: [0, pos) 与 [pos+1, len) 两段搬入新表, 互不重叠
    unsafe {
      ptr::copy_nonoverlapping(current.as_ptr(), (*new_list).start, pos);
      ptr::copy_nonoverlapping(
        current.as_ptr().add(pos + 1),
        (*new_list).start.add(pos),
        current.len() - pos - 1,
      );
    }
    self.data = pack(new_list.expose_provenance(), LIST_TAG);
    Ok(())
  }

  /// 原版 clear()
  pub fn clear(&mut self) {
    self.data = pack(0, EMPTY_TAG);
  }

  /// 原版 begin()/end(): 交出值拷贝的迭代器 (const_iterator)
  pub fn iter(&self) -> ZoneCompactSetIter<'_, H> {
    ZoneCompactSetIter {
      set: self,
      current: 0,
      end: self.size(),
    }
  }

  /// 当前 tag (低 2 bit)
  fn tag(&self) -> usize {
    self.data & TAG_MASK
  }

  /// 单值指针值 (原版 singleton() 的整数形态; 供比较与散列, 不回转指针)
  fn singleton(&self) -> usize {
    debug_assert_eq!(self.tag(), SINGLETON_TAG);
    self.data & !TAG_MASK
  }

  /// 单值指针回转 (仅 SINGLETON_TAG 态调用; 经 exposed provenance 还原)
  fn singleton_ptr(&self) -> *mut () {
    ptr::with_exposed_provenance_mut(self.singleton())
  }

  /// 列表头指针 (原版 list(); 仅 LIST_TAG 态调用; 经 exposed provenance 还原)
  fn list(&self) -> *mut List {
    debug_assert_eq!(self.tag(), LIST_TAG);
    ptr::with_exposed_provenance_mut(self.data & !TAG_MASK)
  }

  /// 列表元素切片视图 (仅 LIST_TAG 态调用)
  fn list_slice(&self) -> &[usize] {
    // SAFETY: 指针来自建表时的 zone 分配, [start, start+length) 全部已写入
    unsafe {
      let list = &*self.list();
      slice::from_raw_parts(list.start, list.length)
    }
  }

  /// 原版 DCHECK(std::is_sorted(...)): debug 构建校验有序不变量
  fn debug_assert_sorted(&self, list: &[usize]) {
    debug_assert!(list.windows(2).all(|w| w[0] <= w[1]));
  }

  /// 原版 NewList(size, zone): List 头 + 元素数组都在 zone 内
  fn new_list(zone: &Zone<'_>, size: usize) -> Result<*mut List> {
    let storage = zone.allocate_array::<usize>(size)?;
    let header = zone.make(List {
      start: storage.as_mut_ptr().cast::<usize>(),
      length: size,
    })?;
    Ok(header as *mut List)
  }
}

/// 原版 ZoneCompactSet<T>::const_iterator (前向, 值语义)
pub struct ZoneCompactSetIter<'s, H: ZoneCompactSetTraits> {
  set: &'s ZoneCompactSet<H>,
  current: usize,
  end: usize,
}

impl<H: ZoneCompactSetTraits> Iterator for ZoneCompactSetIter<'_, H> {
  type Item = H;

  fn next(&mut self) -> Option<H> {
    let item = self.set.at(self.current)?;
    self.current += 1;
    Some(item)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let remain = self.end - self.current;
    (remain, Some(remain))
  }
}

impl<H: ZoneCompactSetTraits> ExactSizeIterator for ZoneCompactSetIter<'_, H> {}

// 原版 operator==(lhs, rhs): data_ 相等, 或双列表逐元素相等
impl<H: ZoneCompactSetTraits> PartialEq for ZoneCompactSet<H> {
  fn eq(&self, other: &Self) -> bool {
    if self.data == other.data {
      return true;
    }
    if self.tag() == LIST_TAG && other.tag() == LIST_TAG {
      return self.list_slice() == other.list_slice();
    }
    false
  }
}

impl<H: ZoneCompactSetTraits> Eq for ZoneCompactSet<H> {}

// 原版 operator<<(std::ostream&, ZoneCompactSet): 逗号连接 at(i)
impl<H: ZoneCompactSetTraits> fmt::Debug for ZoneCompactSet<H> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("{")?;
    for i in 0..self.size() {
      if i > 0 {
        f.write_str(", ")?;
      }
      if let Some(handle) = self.at(i) {
        write!(f, "{:p}", H::handle_to_pointer(handle))?;
      }
    }
    f.write_str("}")
  }
}

// 原版 friend hash_value(ZoneCompactSet) = data_.raw() (zone-compact-set.h:264-266,
// 下游经 base::hash_value 分发, 如 maglev-cse.h gvn_hash_value); Rust 侧以
// std::hash::Hash 对打包字散列, 与 operator==/Eq 语义一致
impl<H: ZoneCompactSetTraits> Hash for ZoneCompactSet<H> {
  fn hash<S: Hasher>(&self, state: &mut S) {
    self.data.hash(state);
  }
}
