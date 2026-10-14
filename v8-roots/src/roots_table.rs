use std::ops::{Index, IndexMut};

use v8_objects_core::{
  heap_number::HeapNumber,
  tagged::{Object, Tagged},
};

use crate::{
  error::{Error, Result},
  read_only_roots::ReadOnlyRoots,
  root_index::RootIndex,
};

/// V8 根对象表 (对应 C++ `v8::internal::RootsTable`)
///
/// 维护全局固定根对象指针数组 `[Tagged<Object>; K_ENTRIES_COUNT]`。
/// 提供安全索引读取、写保护判断以及初始化辅助能力。
#[derive(Clone, Debug)]
pub struct RootsTable {
  roots: [Tagged<Object>; RootIndex::K_ENTRIES_COUNT],
  read_only_write_protected: bool,
}

impl Default for RootsTable {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl RootsTable {
  /// 总根对象槽位数 (对应 C++ `RootsTable::kEntriesCount`)
  pub const K_ENTRIES_COUNT: usize = RootIndex::K_ENTRIES_COUNT;

  /// 只读根对象总数 (对应 C++ `RootIndex::kReadOnlyRootsCount`)
  pub const K_READ_ONLY_ROOTS_COUNT: usize = RootIndex::K_READ_ONLY_ROOTS_COUNT;

  /// 不可移动根对象总数 (对应 C++ `RootIndex::kImmortalImmovableRootsCount`)
  pub const K_IMMORTAL_IMMOVABLE_ROOTS_COUNT: usize = RootIndex::K_IMMORTAL_IMMOVABLE_ROOTS_COUNT;

  /// 可变强根对象总数 (对应 C++ `RootIndex::kMutableRootsCount`)
  pub const K_MUTABLE_ROOTS_COUNT: usize = RootIndex::K_MUTABLE_ROOTS_COUNT;

  /// Smi 标记根对象总数 (对应 C++ `SMI_ROOT_LIST`)
  pub const K_SMI_ROOTS_COUNT: usize = RootIndex::K_SMI_ROOTS_COUNT;

  /// 构造未初始化的根对象表 (所有槽位为 NULL 指针，写保护关闭)
  #[inline]
  pub const fn new() -> Self {
    Self {
      roots: [Tagged::NULL; RootIndex::K_ENTRIES_COUNT],
      read_only_write_protected: false,
    }
  }

  /// 获取指定根索引处的对象指针
  #[inline]
  pub const fn get(&self, index: RootIndex) -> Tagged<Object> {
    self.roots[index as usize]
  }

  /// 尝试根据原始 usize 索引获取对象指针
  #[inline]
  pub fn get_at(&self, index: usize) -> Option<Tagged<Object>> {
    self.roots.get(index).copied()
  }

  /// 在固定 HeapNumber 根分区按 64 位比特值查找已存在的 HeapNumber (对应 C++ `RootsTable::FindHeapNumber`)
  ///
  /// # SAFETY
  /// 若槽位已初始化，其内部对象必须是指向有效 `HeapNumber` 的指针。
  pub unsafe fn find_heap_number(&self, value: f64) -> Option<Tagged<HeapNumber>> {
    let bits = value.to_bits();
    let start = RootIndex::K_FIRST_HEAP_NUMBER_ROOT as usize;
    let end = RootIndex::K_LAST_HEAP_NUMBER_ROOT as usize;
    self.roots[start..=end].iter().copied().find_map(|tagged| {
      if !tagged.is_null() && tagged.is_heap_object() {
        let num: Tagged<HeapNumber> = tagged.cast();
        // SAFETY: 调用方保证已初始化的 HeapNumber 槽位指向有效内存
        let ptr = num.address() as *const HeapNumber;
        if unsafe { (*ptr).value_as_bits() } == bits {
          return Some(num);
        }
      }
      None
    })
  }

  /// 获取已初始化的根对象指针；若尚未初始化 (为 NULL 指针) 则返回错误
  #[inline]
  pub fn require(&self, index: RootIndex) -> Result<Tagged<Object>> {
    let val = self.get(index);
    if val.is_null() {
      Err(Error::UninitializedRoot(index.name()))
    } else {
      Ok(val)
    }
  }

  /// 安全写入根对象指针；若只读根已被写保护锁定，则返回错误
  #[inline]
  pub fn set(&mut self, index: RootIndex, value: Tagged<Object>) -> Result<()> {
    if self.read_only_write_protected && index.is_read_only() {
      return Err(Error::ReadOnlyRootMutation(index.name()));
    }
    self.roots[index as usize] = value;
    Ok(())
  }

  /// 无检查直接写入根对象槽位 (常用于堆初始化和反序列化阶段)
  #[inline]
  pub fn unchecked_set(&mut self, index: RootIndex, value: Tagged<Object>) {
    self.roots[index as usize] = value;
  }

  /// 启用只读根写保护锁定 (堆初始化完成或快照反序列化后调用)
  #[inline]
  pub fn write_protect_read_only_roots(&mut self) {
    self.read_only_write_protected = true;
  }

  /// 解除只读根写保护锁定
  #[inline]
  pub fn unprotect_read_only_roots(&mut self) {
    self.read_only_write_protected = false;
  }

  /// 查询当前只读根是否处于写保护锁定状态
  #[inline]
  pub const fn is_read_only_write_protected(&self) -> bool {
    self.read_only_write_protected
  }

  /// 检查指定根槽位是否已被有效初始化 (非 NULL 指针)
  #[inline]
  pub const fn is_initialized(&self, index: RootIndex) -> bool {
    !self.roots[index as usize].is_null()
  }

  /// 判断给定指针是否指向根表内部槽位，若在则返回其对应的 RootIndex
  #[inline]
  pub fn is_root_handle_location(
    &self,
    handle_location: *const Tagged<Object>,
  ) -> Option<RootIndex> {
    let loc = handle_location as usize;
    let first = self.roots.as_ptr() as usize;
    let entry_size = size_of::<Tagged<Object>>();
    let total_size = RootIndex::K_ENTRIES_COUNT * entry_size;
    let last = first + total_size;
    if loc < first || loc >= last || !(loc - first).is_multiple_of(entry_size) {
      return None;
    }
    let offset = (loc - first) / entry_size;
    RootIndex::from_usize(offset)
  }

  /// 获取指定根槽位的裸指针 (可供 Handle/Slot 引用)
  #[inline]
  pub fn slot(&mut self, index: RootIndex) -> *mut Tagged<Object> {
    &mut self.roots[index as usize] as *mut Tagged<Object>
  }

  /// 借用只读根视图 (ReadOnlyRoots)
  #[inline]
  pub fn read_only_roots(&self) -> ReadOnlyRoots<'_> {
    ReadOnlyRoots::new(self)
  }

  /// 获取只读根部分的不可变切片
  #[inline]
  pub fn read_only_slice(&self) -> &[Tagged<Object>] {
    &self.roots[..RootIndex::K_READ_ONLY_ROOTS_COUNT]
  }

  /// 获取可变强根部分的不可变切片
  #[inline]
  pub fn mutable_slice(&self) -> &[Tagged<Object>] {
    let start = RootIndex::K_FIRST_STRONG_ROOT as usize;
    let end = start + RootIndex::K_MUTABLE_ROOTS_COUNT;
    &self.roots[start..end]
  }

  /// 获取可变强根部分的可变切片
  #[inline]
  pub fn mutable_slice_mut(&mut self) -> &mut [Tagged<Object>] {
    let start = RootIndex::K_FIRST_STRONG_ROOT as usize;
    let end = start + RootIndex::K_MUTABLE_ROOTS_COUNT;
    &mut self.roots[start..end]
  }

  /// 获取 Smi 标记根部分的不可变切片
  #[inline]
  pub fn smi_slice(&self) -> &[Tagged<Object>] {
    let start = RootIndex::K_FIRST_SMI_ROOT as usize;
    let end = start + RootIndex::K_SMI_ROOTS_COUNT;
    &self.roots[start..end]
  }

  /// 获取 Smi 标记根部分的可变切片
  #[inline]
  pub fn smi_slice_mut(&mut self) -> &mut [Tagged<Object>] {
    let start = RootIndex::K_FIRST_SMI_ROOT as usize;
    let end = start + RootIndex::K_SMI_ROOTS_COUNT;
    &mut self.roots[start..end]
  }

  /// 获取只读根加可变强根的不可变切片 (包含 GC 遍历的所有根)
  #[inline]
  pub fn strong_or_read_only_slice(&self) -> &[Tagged<Object>] {
    &self.roots[..RootIndex::K_FIRST_SMI_ROOT as usize]
  }

  /// 获取全量根数组切片
  #[inline]
  pub const fn as_slice(&self) -> &[Tagged<Object>] {
    &self.roots
  }

  /// 获取全量根数组可变切片
  #[inline]
  pub fn as_mut_slice(&mut self) -> &mut [Tagged<Object>] {
    &mut self.roots
  }

  /// 查询指定根是否为只读根
  #[inline]
  pub const fn is_read_only(index: RootIndex) -> bool {
    index.is_read_only()
  }

  /// 查询指定根是否为不可移动根 (Immortal Immovable)
  #[inline]
  pub const fn is_immortal_immovable(index: RootIndex) -> bool {
    index.is_immortal_immovable()
  }

  /// 判断指定根是否属于受信任对象 Map 列表 (对应 C++ `RootsTable::IsInTrustedObjectMapList`)
  #[inline]
  pub const fn is_in_trusted_object_map_list(index: RootIndex) -> bool {
    index.is_in_trusted_object_map_list()
  }

  /// 判断指定根是否属于可序列化受信任对象 Map 列表 (对应 C++ `RootsTable::IsInSerializableTrustedObjectMapList`)
  #[inline]
  pub const fn is_in_serializable_trusted_object_map_list(index: RootIndex) -> bool {
    index.is_in_serializable_trusted_object_map_list()
  }

  /// 根据单字节字符 code 计算其在根表中的 RootIndex (对应 C++ `RootsTable::SingleCharacterStringIndex`)
  #[inline]
  pub const fn single_character_string_index(c: u8) -> RootIndex {
    RootIndex::single_character_string_index(c)
  }

  /// 获取指定根的英文标识名
  #[inline]
  pub const fn name(index: RootIndex) -> &'static str {
    index.name()
  }

  /// 获取根对象在根表内部以字节计量的偏移量 (对应 C++ `RootsTable::offset_of`)
  #[inline]
  pub const fn offset_of(index: RootIndex) -> usize {
    (index as usize) * size_of::<usize>()
  }
}

impl Index<RootIndex> for RootsTable {
  type Output = Tagged<Object>;

  #[inline]
  fn index(&self, index: RootIndex) -> &Self::Output {
    &self.roots[index as usize]
  }
}

impl IndexMut<RootIndex> for RootsTable {
  #[inline]
  fn index_mut(&mut self, index: RootIndex) -> &mut Self::Output {
    &mut self.roots[index as usize]
  }
}
