// v8-objects-rest/src/symbol.rs
// 对应原版 v8/src/objects/symbol.h (与 name.h:315-388) 及 symbol-inl.h

use std::mem::{offset_of, size_of};

pub use v8_objects_core::symbol::PrivateSymbolKind;
use v8_objects_core::{
  map::Map,
  name::Name,
  tagged::{Object, Tagged},
};

/// 众所周知符号与公共符号标识符 (原版 v8/src/init/heap-symbols.h)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum WellKnownSymbolId {
  /// Symbol.asyncIterator
  AsyncIterator,
  /// Symbol.hasInstance
  HasInstance,
  /// Symbol.isConcatSpreadable
  IsConcatSpreadable,
  /// Symbol.iterator
  Iterator,
  /// Symbol.match
  Match,
  /// Symbol.matchAll
  MatchAll,
  /// Symbol.replace
  Replace,
  /// Symbol.search
  Search,
  /// Symbol.species
  Species,
  /// Symbol.split
  Split,
  /// Symbol.toPrimitive
  ToPrimitive,
  /// Symbol.toStringTag
  ToStringTag,
  /// Symbol.unscopables
  Unscopables,
  /// Symbol.dispose
  Dispose,
  /// Symbol.asyncDispose
  AsyncDispose,
}

impl WellKnownSymbolId {
  /// 获取规范描述文本
  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::AsyncIterator => "Symbol.asyncIterator",
      Self::HasInstance => "Symbol.hasInstance",
      Self::IsConcatSpreadable => "Symbol.isConcatSpreadable",
      Self::Iterator => "Symbol.iterator",
      Self::Match => "Symbol.match",
      Self::MatchAll => "Symbol.matchAll",
      Self::Replace => "Symbol.replace",
      Self::Search => "Symbol.search",
      Self::Species => "Symbol.species",
      Self::Split => "Symbol.split",
      Self::ToPrimitive => "Symbol.toPrimitive",
      Self::ToStringTag => "Symbol.toStringTag",
      Self::Unscopables => "Symbol.unscopables",
      Self::Dispose => "Symbol.dispose",
      Self::AsyncDispose => "Symbol.asyncDispose",
    }
  }
}

/// ES6 符号对象 (Symbol)
///
/// 继承自 Name，内部持有 flags 状态位与描述符引用 (description)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
  /// 继承自 Name (HeapObject 8B + raw_hash_field 4B)
  pub name: Name,
  /// 符号标志位 (私有类别、众所周知符号标记等)
  pub flags: u32,
  /// 描述符对象引用 (String 或 Undefined)
  pub description: Tagged<Object>,
}

// C++ 原版中 Symbol 继承自 Name，其 flags_ 成员被放置于 Name 结构尾填充中 (offset 12)
// 在 Rust repr(C) 视图中，子结构体 Name 包含 HeapObject (8B) 与 raw_hash_field (4B) 整体按 8B 对齐为 16B
pub const K_C_FLAGS_OFFSET: usize = 12;
pub const K_C_DESCRIPTION_OFFSET: usize = 16;
pub const K_C_SIZE: usize = 24;

// 静态断言保证 Rust 逻辑视图 (字段序 1:1 原版) 尺寸与偏移
const _: () = assert!(size_of::<Symbol>() == 32);
const _: () = assert!(offset_of!(Symbol, flags) == 16);
const _: () = assert!(offset_of!(Symbol, description) == 24);

impl Symbol {
  /// 私有类别掩码 (bit 0..2)
  pub const PRIVATE_SYMBOL_KIND_MASK: u32 = 0b11;
  /// 众所周知符号掩码 (bit 2)
  pub const IS_WELL_KNOWN_SYMBOL_MASK: u32 = 1 << 2;
  /// 公共符号表登记掩码 (bit 3, Symbol.for)
  pub const IS_IN_PUBLIC_SYMBOL_TABLE_MASK: u32 = 1 << 3;
  /// 高频优化观察符号掩码 (bit 4, 如 @@toStringTag)
  pub const IS_INTERESTING_SYMBOL_MASK: u32 = 1 << 4;

  /// 构造普通未标记符号
  #[inline]
  pub const fn new(map: Tagged<Map>, description: Tagged<Object>) -> Self {
    Self {
      name: Name::new(map),
      flags: 0,
      description,
    }
  }

  /// 构造规范定义的众所周知符号 (WellKnownSymbol)
  #[inline]
  pub const fn well_known(map: Tagged<Map>, description: Tagged<Object>) -> Self {
    Self {
      name: Name::new(map),
      flags: Self::IS_WELL_KNOWN_SYMBOL_MASK,
      description,
    }
  }

  /// 构造注册于全局公共符号表中的符号 (Symbol.for)
  #[inline]
  pub const fn public(map: Tagged<Map>, description: Tagged<Object>) -> Self {
    Self {
      name: Name::new(map),
      flags: Self::IS_IN_PUBLIC_SYMBOL_TABLE_MASK,
      description,
    }
  }

  /// 构造 V8 虚拟机内部私有符号
  #[inline]
  pub const fn private_internal(map: Tagged<Map>, description: Tagged<Object>) -> Self {
    Self {
      name: Name::new(map),
      flags: PrivateSymbolKind::Internal as u32,
      description,
    }
  }

  /// 构造私有字段名符号 (#field)
  #[inline]
  pub const fn private_field(map: Tagged<Map>, description: Tagged<Object>) -> Self {
    Self {
      name: Name::new(map),
      flags: PrivateSymbolKind::FieldName as u32,
      description,
    }
  }

  /// 构造私有品牌校验符号 (#brand)
  #[inline]
  pub const fn private_brand(map: Tagged<Map>, description: Tagged<Object>) -> Self {
    Self {
      name: Name::new(map),
      flags: PrivateSymbolKind::Brand as u32,
      description,
    }
  }

  /// 获取符号描述符指针
  #[inline]
  pub const fn description(&self) -> Tagged<Object> {
    self.description
  }

  /// 设置符号描述符指针
  #[inline]
  pub fn set_description(&mut self, description: Tagged<Object>) {
    self.description = description;
  }

  /// 获取 flags 原始值
  #[inline]
  pub const fn flags(&self) -> u32 {
    self.flags
  }

  /// 设置 flags 原始值
  #[inline]
  pub fn set_flags(&mut self, flags: u32) {
    self.flags = flags;
  }

  /// 获取符号私有性类别
  #[inline]
  pub const fn private_symbol_kind(&self) -> PrivateSymbolKind {
    match self.flags & Self::PRIVATE_SYMBOL_KIND_MASK {
      0 => PrivateSymbolKind::Public,
      1 => PrivateSymbolKind::Internal,
      2 => PrivateSymbolKind::FieldName,
      _ => PrivateSymbolKind::Brand,
    }
  }

  /// 设置符号私有性类别
  #[inline]
  pub fn set_private_symbol_kind(&mut self, kind: PrivateSymbolKind) {
    self.flags = (self.flags & !Self::PRIVATE_SYMBOL_KIND_MASK) | (kind as u32);
  }

  /// 判断是否为任意类型的私有符号
  #[inline]
  pub const fn is_any_private(&self) -> bool {
    !matches!(self.private_symbol_kind(), PrivateSymbolKind::Public)
  }

  /// 判断是否为 V8 内部使用的私有符号
  #[inline]
  pub const fn is_private_internal(&self) -> bool {
    matches!(self.private_symbol_kind(), PrivateSymbolKind::Internal)
  }

  /// 判断是否为类私有成员符号 (字段或品牌)
  #[inline]
  pub const fn is_any_private_name(&self) -> bool {
    matches!(
      self.private_symbol_kind(),
      PrivateSymbolKind::FieldName | PrivateSymbolKind::Brand
    )
  }

  /// 判断是否为品牌校验符号
  #[inline]
  pub const fn is_private_brand(&self) -> bool {
    matches!(self.private_symbol_kind(), PrivateSymbolKind::Brand)
  }

  /// 判断是否为规范定义的众所周知符号 (WellKnownSymbol)
  #[inline]
  pub const fn is_well_known_symbol(&self) -> bool {
    (self.flags & Self::IS_WELL_KNOWN_SYMBOL_MASK) != 0
  }

  /// 设置众所周知符号标志
  #[inline]
  pub fn set_is_well_known_symbol(&mut self, value: bool) {
    if value {
      self.flags |= Self::IS_WELL_KNOWN_SYMBOL_MASK;
    } else {
      self.flags &= !Self::IS_WELL_KNOWN_SYMBOL_MASK;
    }
  }

  /// 判断是否位于公共符号表中 (Symbol.for 注册)
  #[inline]
  pub const fn is_in_public_symbol_table(&self) -> bool {
    (self.flags & Self::IS_IN_PUBLIC_SYMBOL_TABLE_MASK) != 0
  }

  /// 设置公共符号表标志
  #[inline]
  pub fn set_is_in_public_symbol_table(&mut self, value: bool) {
    if value {
      self.flags |= Self::IS_IN_PUBLIC_SYMBOL_TABLE_MASK;
    } else {
      self.flags &= !Self::IS_IN_PUBLIC_SYMBOL_TABLE_MASK;
    }
  }

  /// 判断是否为高频优化观察的特殊符号 (InterestingSymbol，如 @@toStringTag)
  #[inline]
  pub const fn is_interesting_symbol(&self) -> bool {
    (self.flags & Self::IS_INTERESTING_SYMBOL_MASK) != 0
  }

  /// 设置高频优化观察符号标志
  #[inline]
  pub fn set_is_interesting_symbol(&mut self, value: bool) {
    if value {
      self.flags |= Self::IS_INTERESTING_SYMBOL_MASK;
    } else {
      self.flags &= !Self::IS_INTERESTING_SYMBOL_MASK;
    }
  }
}
