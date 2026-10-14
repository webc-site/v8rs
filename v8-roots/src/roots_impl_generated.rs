// 由 tool/src/roots/roots.cc.js 从 v8/src/roots/roots.{cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// 根表实现层: 断言/名称表/签名骨架

use super::root_index::RootIndex;

/// RootIndex 序与公开 API 索引常量的静态对齐 (原版 roots.cc 顶部 static_assert, 6 条)
pub mod root_index_assertions {
  /// RootIndex::UndefinedValue == v8-internal.h kUndefinedValueRootIndex = 0
  pub const UNDEFINED_VALUE_ROOT_INDEX: u32 = 0;
  /// RootIndex::TheHoleValue == v8-internal.h kTheHoleValueRootIndex = 1
  pub const THE_HOLE_VALUE_ROOT_INDEX: u32 = 1;
  /// RootIndex::NullValue == v8-internal.h kNullValueRootIndex = 2
  pub const NULL_VALUE_ROOT_INDEX: u32 = 2;
  /// RootIndex::TrueValue == v8-internal.h kTrueValueRootIndex = 3
  pub const TRUE_VALUE_ROOT_INDEX: u32 = 3;
  /// RootIndex::FalseValue == v8-internal.h kFalseValueRootIndex = 4
  pub const FALSE_VALUE_ROOT_INDEX: u32 = 4;
  /// RootIndex::EmptyString == v8-internal.h kEmptyStringRootIndex = 5
  pub const EMPTY_STRING_ROOT_INDEX: u32 = 5;

  /// 总数守卫: 6 条断言
  pub const ASSERT_COUNT: usize = 6;
}

/// 根名称表 (原版 RootsTable::root_names_[kEntriesCount], 宏 ROOT_NAME 展开
/// 由 tool/src/roots/roots.h.js 的 root_index.rs 承接, 此处登记签名)
pub mod roots_table_skel {
  /// 根下标比较 (原版 -inl operator<)
  pub fn root_index_lt(_lhs: RootIndex, _rhs: RootIndex) -> bool {
    unimplemented!("骨架")
  }

  /// 根下标自增 (原版 -inl operator++, 越末项为 UB)
  pub fn root_index_next(_index: &mut RootIndex) {
    unimplemented!("骨架")
  }

  /// 判定句柄位置是否落在根表内并回填下标 (原版 IsRootHandleLocation, -inl)
  pub fn is_root_handle_location(_location: usize) -> Option<RootIndex> {
    unimplemented!("骨架")
  }

  /// 间接句柄版 (原版 IsRootHandle, -inl)
  pub fn is_root_handle(_handle_addr: usize) -> Option<RootIndex> {
    unimplemented!("骨架")
  }

  /// 按下标取根槽地址 (原版 RootsTable::operator[])
  pub fn root_at(_index: RootIndex) -> usize {
    unimplemented!("骨架")
  }

  /// 按下标取根句柄 (原版 handle_at, -inl)
  pub fn handle_at(_index: RootIndex) -> usize {
    unimplemented!("骨架")
  }

  /// 在 HeapNumber 只读根区间内按位型查找 (原版 FindHeapNumber, bit_cast 比对)
  pub fn find_heap_number(_value: f64) -> Option<usize> {
    unimplemented!("骨架")
  }

  /// 根槽是否落在根表范围内 (原版 IsRootHandleLocation, -inl)
  pub fn is_root_slot(_slot: usize) -> bool {
    unimplemented!("骨架")
  }
}

/// 只读根实现骨架 (原版 ReadOnlyRoots)
pub mod read_only_roots_impl_skel {
  /// one_pointer_filler 的 MapWord (原版 one_pointer_filler_map_word)
  pub fn one_pointer_filler_map_word() -> usize {
    unimplemented!("骨架")
  }

  /// 遍历只读根 (原版 Iterate: VisitRootPointers + Synchronize(kReadOnlyRootList))
  pub fn iterate(_visitor: &mut dyn FnMut(usize, usize)) {
    unimplemented!("骨架")
  }

  /// protector 命名区同页校验 (原版 VerifyNameForProtectorsPages)
  pub fn verify_name_for_protectors_pages() {
    unimplemented!("骨架")
  }

  /// DEBUG: protector 命名相邻性逐项校验 (原版 VerifyNameForProtectors)
  pub fn verify_name_for_protectors() {
    unimplemented!("骨架")
  }

  /// DEBUG: 只读根类型校验 (原版 VerifyTypes, 含 oddball kind 慢检与 trusted map 检)
  pub fn verify_types() {
    unimplemented!("骨架")
  }

  /// 从静态根表初始化 (原版 InitFromStaticRootsTable:
  /// 逐项 DecompressTagged 填 read_only_roots_, 末项必须 == kLastReadOnlyRoot)
  pub fn init_from_static_roots_table(_cage_base: usize, _table: &[u32]) {
    unimplemented!("骨架")
  }
}
