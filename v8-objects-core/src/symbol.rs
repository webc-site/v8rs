use crate::{
  map::Map,
  name::Name,
  tagged::{Object, Tagged},
};

/// 符号的私有性类别 (对应 C++ PrivateSymbolKind)
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrivateSymbolKind {
  /// 公开符号 (常规 Symbol 及 Symbol.iterator 等内置全局符号)
  Public = 0,
  /// 虚拟机内部私有符号 (不可通过反射获取，缺少属性不抛错)
  Internal = 1,
  /// 类私有字段符号 (#field)
  FieldName = 2,
  /// 类私有方法/品牌校验符号 (#brand)
  Brand = 3,
}

/// ES6 符号对象 (Symbol)
///
/// 继承自 Name，内部持有 flags 状态位与描述符引用 (description)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
  /// 继承自 Name
  pub name: Name,
  /// 符号标志位 (私有类别、内置符号标记等)
  pub flags: u32,
  /// 描述符对象引用 (String 或 Undefined)
  pub description: Tagged<Object>,
}

impl Symbol {
  // flags 掩码常量
  pub const PRIVATE_SYMBOL_KIND_MASK: u32 = 0b11;
  pub const IS_WELL_KNOWN_SYMBOL_MASK: u32 = 1 << 2;
  pub const IS_IN_PUBLIC_SYMBOL_TABLE_MASK: u32 = 1 << 3;
  pub const IS_INTERESTING_SYMBOL_MASK: u32 = 1 << 4;

  /// 构造 Symbol 对象
  #[inline]
  pub const fn new(map: Tagged<Map>, description: Tagged<Object>) -> Self {
    Self {
      name: Name::new(map),
      flags: 0,
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
