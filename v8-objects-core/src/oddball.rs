use crate::{
  heap_object::HeapObject,
  map::Map,
  tagged::{Object, Tagged},
};

/// Oddball 种类枚举 (对应 C++ Oddball 常量与类型)
///
/// 判别值 1:1 对齐原版 (oddball.h:58-62): kFalse=0, kTrue=1, kNull=3,
/// kUndefined=4 (2 未用); the_hole/uninitialized 在本版本为独立 Hole 类
/// (hole.h), 不是 Oddball kind — 勿再发明 5/6 变体。
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OddballKind {
  /// 布尔值 false
  False = 0,
  /// 布尔值 true
  True = 1,
  /// 空指针值 null
  Null = 3,
  /// 未定义值 undefined
  Undefined = 4,
}

/// 特殊单例值对象 (Oddball)
///
/// 描述 JavaScript 中的全局常量单例，如 undefined, null, true, false, the hole 等。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Oddball {
  /// 继承自 HeapObject
  pub header: HeapObject,
  /// 预先缓存的转数值原始 float64 值 (如 true 为 1.0, false/null 为 0.0, undefined 为 NaN)
  pub to_number_raw: f64,
  /// 预先缓存的 toString 字符串对象指针
  pub to_string: Tagged<Object>,
  /// 预先缓存的 toNumber 数字对象指针
  pub to_number: Tagged<Object>,
  /// 预先缓存的 typeof 类型字符串对象指针
  pub type_of: Tagged<Object>,
  /// 具体的 Oddball 单例类别
  pub kind: OddballKind,
}

impl Oddball {
  pub const K_FALSE: u8 = OddballKind::False as u8;
  pub const K_TRUE: u8 = OddballKind::True as u8;
  pub const K_NOT_BOOLEAN_MASK: u8 = !1;
  pub const K_NULL: u8 = OddballKind::Null as u8;
  pub const K_UNDEFINED: u8 = OddballKind::Undefined as u8;

  /// 构造 Oddball 实例
  #[inline]
  pub const fn new(
    map: Tagged<Map>,
    kind: OddballKind,
    to_number_raw: f64,
    to_string: Tagged<Object>,
    to_number: Tagged<Object>,
    type_of: Tagged<Object>,
  ) -> Self {
    Self {
      header: HeapObject::new(map),
      to_number_raw,
      to_string,
      to_number,
      type_of,
      kind,
    }
  }

  /// 获取 Oddball 类别
  #[inline]
  pub const fn kind(&self) -> OddballKind {
    self.kind
  }

  /// 获取预计算的原始浮点数形式
  #[inline]
  pub const fn to_number_raw(&self) -> f64 {
    self.to_number_raw
  }

  /// 以位模式设置预计算浮点数 (原版 set_to_number_raw_as_bits)。
  #[inline]
  pub fn set_to_number_raw_as_bits(&mut self, bits: u64) {
    self.to_number_raw = f64::from_bits(bits);
  }

  /// 获取 toString 字符串对象指针
  #[inline]
  pub const fn to_string(&self) -> Tagged<Object> {
    self.to_string
  }

  /// 获取 toNumber 对象指针
  #[inline]
  pub const fn to_number(&self) -> Tagged<Object> {
    self.to_number
  }

  /// 获取 typeof 结果字符串对象指针
  #[inline]
  pub const fn type_of(&self) -> Tagged<Object> {
    self.type_of
  }

  /// 是否为 boolean 类型 (true 或 false)
  #[inline]
  pub const fn is_boolean(&self) -> bool {
    matches!(self.kind, OddballKind::False | OddballKind::True)
  }

  /// 转换为布尔值，非布尔返回 None
  #[inline]
  pub const fn to_bool(&self) -> Option<bool> {
    match self.kind {
      OddballKind::True => Some(true),
      OddballKind::False => Some(false),
      _ => None,
    }
  }

  /// 是否为 false
  #[inline]
  pub const fn is_false(&self) -> bool {
    matches!(self.kind, OddballKind::False)
  }

  /// 是否为 true
  #[inline]
  pub const fn is_true(&self) -> bool {
    matches!(self.kind, OddballKind::True)
  }

  /// 是否为 null
  #[inline]
  pub const fn is_null(&self) -> bool {
    matches!(self.kind, OddballKind::Null)
  }

  /// 是否为 undefined
  #[inline]
  pub const fn is_undefined(&self) -> bool {
    matches!(self.kind, OddballKind::Undefined)
  }
}
