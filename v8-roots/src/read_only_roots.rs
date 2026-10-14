use v8_objects_arrays::{fixed_array::FixedArray, primitive_array::ByteArray};
use v8_objects_core::{
  heap_number::HeapNumber,
  map::Map,
  map_word::MapWord,
  oddball::Oddball,
  tagged::{Object, Tagged},
};
use v8_objects_maps::{DescriptorArray, EnumCache, PropertyArray};
use v8_objects_strings::string::String as V8String;

use crate::{
  error::{Error, Result},
  root_index::RootIndex,
  roots_table::RootsTable,
};

/// 只读根对象集合视图 (对应 C++ `v8::internal::ReadOnlyRoots`)
///
/// 封装指向只读根分区 `[Tagged<Object>]` 的不可变切片，提供便捷强类型的单例与 Map 获取器。
#[derive(Copy, Clone, Debug)]
pub struct ReadOnlyRoots<'a> {
  roots: &'a [Tagged<Object>],
}

impl<'a> ReadOnlyRoots<'a> {
  /// 只读根槽位总数 (对应 C++ `ReadOnlyRoots::kEntriesCount`)
  pub const K_ENTRIES_COUNT: usize = RootIndex::K_READ_ONLY_ROOTS_COUNT;

  /// 从完整的根表构造只读根视图
  #[inline]
  pub fn new(table: &'a RootsTable) -> Self {
    Self {
      roots: table.read_only_slice(),
    }
  }

  /// 直接从只读根切片构造视图
  #[inline]
  pub const fn from_slice(roots: &'a [Tagged<Object>]) -> Self {
    Self { roots }
  }

  /// 获取只读根底层的切片引用
  #[inline]
  pub const fn as_slice(&self) -> &'a [Tagged<Object>] {
    self.roots
  }

  /// 获取只读根槽位数量
  #[inline]
  pub const fn len(&self) -> usize {
    self.roots.len()
  }

  /// 判断只读根切片是否为空
  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.roots.is_empty()
  }

  /// 获取指定只读根索引处的通用 Tagged 对象
  #[inline]
  pub fn object_at(&self, index: RootIndex) -> Tagged<Object> {
    self
      .roots
      .get(index as usize)
      .copied()
      .unwrap_or(Tagged::NULL)
  }

  /// 获取指定只读根索引处的裸内存地址
  #[inline]
  pub fn address_at(&self, index: RootIndex) -> usize {
    self.object_at(index).ptr()
  }

  /// 检查指定只读根索引是否已有效初始化 (非空指针)
  #[inline]
  pub fn is_initialized(&self, index: RootIndex) -> bool {
    !self.object_at(index).is_null()
  }

  /// 在固定 HeapNumber 根分区按 64 位比特值查找已存在的 HeapNumber (对应 C++ `RootsTable::FindHeapNumber`)
  ///
  /// # SAFETY
  /// 若槽位已初始化，其内部对象必须是指向有效 `HeapNumber` 的指针。
  pub unsafe fn find_heap_number(&self, value: f64) -> Option<Tagged<HeapNumber>> {
    let bits = value.to_bits();
    for idx in
      (RootIndex::K_FIRST_HEAP_NUMBER_ROOT as usize)..=(RootIndex::K_LAST_HEAP_NUMBER_ROOT as usize)
    {
      if let Some(root_idx) = RootIndex::from_usize(idx) {
        let tagged = self.object_at(root_idx);
        if !tagged.is_null() && tagged.is_heap_object() {
          let num: Tagged<HeapNumber> = tagged.cast();
          // SAFETY: 调用方保证已初始化的 HeapNumber 槽位指向有效内存
          let ptr = num.address() as *const HeapNumber;
          if unsafe { (*ptr).value_as_bits() } == bits {
            return Some(num);
          }
        }
      }
    }
    None
  }

  /// 获取已初始化的只读根对象指针；若未初始化则返回错误
  #[inline]
  pub fn require(&self, index: RootIndex) -> Result<Tagged<Object>> {
    let val = self.object_at(index);
    if val.is_null() {
      Err(Error::UninitializedRoot(index.name()))
    } else {
      Ok(val)
    }
  }

  // ── 基础单例值 Getter ──

  /// JavaScript `undefined` 单例对象
  #[inline]
  pub fn undefined_value(&self) -> Tagged<Oddball> {
    self.object_at(RootIndex::UndefinedValue).cast()
  }

  /// 内部占位符 `the_hole` 单例对象
  #[inline]
  pub fn the_hole_value(&self) -> Tagged<Object> {
    self.object_at(RootIndex::TheHoleValue)
  }

  /// JavaScript `null` 单例对象
  #[inline]
  pub fn null_value(&self) -> Tagged<Oddball> {
    self.object_at(RootIndex::NullValue).cast()
  }

  /// JavaScript `true` 布尔单例对象
  #[inline]
  pub fn true_value(&self) -> Tagged<Oddball> {
    self.object_at(RootIndex::TrueValue).cast()
  }

  /// JavaScript `false` 布尔单例对象
  #[inline]
  pub fn false_value(&self) -> Tagged<Oddball> {
    self.object_at(RootIndex::FalseValue).cast()
  }

  /// 根据 Rust 原生布尔值返回对应的 `true` 或 `false` Oddball 单例
  #[inline]
  pub fn boolean_value(&self, value: bool) -> Tagged<Oddball> {
    if value {
      self.true_value()
    } else {
      self.false_value()
    }
  }

  /// NaN 堆数字单例 (对应 C++ `ReadOnlyRoots::nan_value`)
  #[inline]
  pub fn nan_value(&self) -> Tagged<HeapNumber> {
    self.object_at(RootIndex::NanValue).cast()
  }

  /// Hole NaN 堆数字单例 (对应 C++ `ReadOnlyRoots::hole_nan_value`)
  #[inline]
  pub fn hole_nan_value(&self) -> Tagged<HeapNumber> {
    self.object_at(RootIndex::HoleNanValue).cast()
  }

  /// +Infinity 堆数字单例 (对应 C++ `ReadOnlyRoots::infinity_value`)
  #[inline]
  pub fn infinity_value(&self) -> Tagged<HeapNumber> {
    self.object_at(RootIndex::InfinityValue).cast()
  }

  /// -Infinity 堆数字单例 (对应 C++ `ReadOnlyRoots::minus_infinity_value`)
  #[inline]
  pub fn minus_infinity_value(&self) -> Tagged<HeapNumber> {
    self.object_at(RootIndex::MinusInfinityValue).cast()
  }

  /// -0.0 堆数字单例 (对应 C++ `ReadOnlyRoots::minus_zero_value`)
  #[inline]
  pub fn minus_zero_value(&self) -> Tagged<HeapNumber> {
    self.object_at(RootIndex::MinusZeroValue).cast()
  }

  /// 空字符串单例 (`""`)
  #[inline]
  pub fn empty_string(&self) -> Tagged<V8String> {
    self.object_at(RootIndex::EmptyString).cast()
  }

  /// 空 FixedArray 单例
  #[inline]
  pub fn empty_fixed_array(&self) -> Tagged<FixedArray> {
    self.object_at(RootIndex::EmptyFixedArray).cast()
  }

  /// 空 ByteArray 单例
  #[inline]
  pub fn empty_byte_array(&self) -> Tagged<ByteArray> {
    self.object_at(RootIndex::EmptyByteArray).cast()
  }

  /// 空 ScopeInfo 单例
  #[inline]
  pub fn empty_scope_info(&self) -> Tagged<Object> {
    self.object_at(RootIndex::EmptyScopeInfo)
  }

  /// 空 PropertyArray 单例
  #[inline]
  pub fn empty_property_array(&self) -> Tagged<PropertyArray> {
    self.object_at(RootIndex::EmptyPropertyArray).cast()
  }

  /// 空 DescriptorArray 单例
  #[inline]
  pub fn empty_descriptor_array(&self) -> Tagged<DescriptorArray> {
    self.object_at(RootIndex::EmptyDescriptorArray).cast()
  }

  /// 空 WeakFixedArray 单例
  #[inline]
  pub fn empty_weak_fixed_array(&self) -> Tagged<Object> {
    self.object_at(RootIndex::EmptyWeakFixedArray)
  }

  /// 空 WeakArrayList 单例
  #[inline]
  pub fn empty_weak_array_list(&self) -> Tagged<Object> {
    self.object_at(RootIndex::EmptyWeakArrayList)
  }

  /// 空 EnumCache 单例
  #[inline]
  pub fn empty_enum_cache(&self) -> Tagged<EnumCache> {
    self.object_at(RootIndex::EmptyEnumCache).cast()
  }

  /// 获取单字节 ASCII / Latin1 单字符内部化字符串单例
  #[inline]
  pub fn single_character_string(&self, code: u8) -> Tagged<V8String> {
    self
      .object_at(RootIndex::single_character_string_index(code))
      .cast()
  }

  // ── 常用核心 Map Getter ──

  /// 元 Map (MetaMap / Map of Map)
  #[inline]
  pub fn meta_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::MetaMap).cast()
  }

  /// ByteArray 的 Map
  #[inline]
  pub fn byte_array_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::ByteArrayMap).cast()
  }

  /// FixedArray 的 Map
  #[inline]
  pub fn fixed_array_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::FixedArrayMap).cast()
  }

  /// 写时复制 (COW) FixedArray 的 Map
  #[inline]
  pub fn fixed_cow_array_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::FixedCOWArrayMap).cast()
  }

  /// FixedDoubleArray 的 Map
  #[inline]
  pub fn fixed_double_array_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::FixedDoubleArrayMap).cast()
  }

  /// HashTable 的 Map
  #[inline]
  pub fn hash_table_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::HashTableMap).cast()
  }

  /// Symbol 的 Map
  #[inline]
  pub fn symbol_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::SymbolMap).cast()
  }

  /// 顺序单字节字符串 Map
  #[inline]
  pub fn seq_one_byte_string_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::SeqOneByteStringMap).cast()
  }

  /// 内部化单字节字符串 Map
  #[inline]
  pub fn internalized_one_byte_string_map(&self) -> Tagged<Map> {
    self
      .object_at(RootIndex::InternalizedOneByteStringMap)
      .cast()
  }

  /// Undefined 单例的 Map
  #[inline]
  pub fn undefined_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::UndefinedMap).cast()
  }

  /// Null 单例的 Map
  #[inline]
  pub fn null_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::NullMap).cast()
  }

  /// Boolean 单例的 Map
  #[inline]
  pub fn boolean_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::BooleanMap).cast()
  }

  /// Hole 单例的 Map
  #[inline]
  pub fn hole_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::HoleMap).cast()
  }

  /// 空闲空间 (FreeSpace) 的 Map
  #[inline]
  pub fn free_space_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::FreeSpaceMap).cast()
  }

  /// 单指针填充物 (OnePointerFiller) 的 Map
  #[inline]
  pub fn one_pointer_filler_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::OnePointerFillerMap).cast()
  }

  /// 获取单指针填充物 (OnePointerFiller) 的 MapWord (对应 C++ `ReadOnlyRoots::one_pointer_filler_map_word`)
  #[inline]
  pub fn one_pointer_filler_map_word(&self) -> MapWord {
    MapWord::from_map(self.one_pointer_filler_map())
  }

  /// 双指针填充物 (TwoPointerFiller) 的 Map
  #[inline]
  pub fn two_pointer_filler_map(&self) -> Tagged<Map> {
    self.object_at(RootIndex::TwoPointerFillerMap).cast()
  }

  // ── Protector 范围检测 ──

  /// Protector 标识符名称范围首地址
  #[inline]
  pub fn first_name_for_protector(&self) -> usize {
    self.address_at(RootIndex::K_FIRST_NAME_FOR_PROTECTOR)
  }

  /// Protector 标识符名称范围末地址
  #[inline]
  pub fn last_name_for_protector(&self) -> usize {
    self.address_at(RootIndex::K_LAST_NAME_FOR_PROTECTOR)
  }

  /// 检查给定对象地址是否落在 Protector 敏感标识符连续区间内
  #[inline]
  pub fn is_name_for_protector(&self, object: Tagged<Object>) -> bool {
    let ptr = object.ptr();
    let first = self.first_name_for_protector();
    let last = self.last_name_for_protector();
    first != 0 && ptr >= first && ptr <= last
  }
}
