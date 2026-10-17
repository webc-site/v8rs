use std::fmt;

/// JSON 值模型 (对应 ECMAScript JSON AST 节点)
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
  /// null
  Null,
  /// 布尔值
  Bool(bool),
  /// 数值 (包含整数与双精度浮点)
  Number(f64),
  /// 字符串
  String(String),
  /// 数组
  Array(Vec<Self>),
  /// 对象 (保持原版有序键值对列表)
  Object(Vec<(String, Self)>),
}

impl JsonValue {
  /// 是否为 Null
  #[inline]
  #[must_use]
  pub const fn is_null(&self) -> bool {
    matches!(self, Self::Null)
  }

  /// 是否为布尔
  #[inline]
  #[must_use]
  pub const fn is_bool(&self) -> bool {
    matches!(self, Self::Bool(_))
  }

  /// 是否为数值
  #[inline]
  #[must_use]
  pub const fn is_number(&self) -> bool {
    matches!(self, Self::Number(_))
  }

  /// 是否为字符串
  #[inline]
  #[must_use]
  pub const fn is_string(&self) -> bool {
    matches!(self, Self::String(_))
  }

  /// 是否为数组
  #[inline]
  #[must_use]
  pub const fn is_array(&self) -> bool {
    matches!(self, Self::Array(_))
  }

  /// 是否为对象
  #[inline]
  #[must_use]
  pub const fn is_object(&self) -> bool {
    matches!(self, Self::Object(_))
  }

  /// 尝试获取布尔借用
  #[inline]
  #[must_use]
  pub const fn as_bool(&self) -> Option<bool> {
    if let Self::Bool(b) = *self {
      Some(b)
    } else {
      None
    }
  }

  /// 尝试获取浮点数值
  #[inline]
  #[must_use]
  pub const fn as_f64(&self) -> Option<f64> {
    if let Self::Number(n) = *self {
      Some(n)
    } else {
      None
    }
  }

  /// 尝试获取字符串切片
  #[inline]
  #[must_use]
  pub fn as_str(&self) -> Option<&str> {
    if let Self::String(s) = self {
      Some(s.as_str())
    } else {
      None
    }
  }

  /// 尝试获取数组切片
  #[inline]
  #[must_use]
  pub fn as_array(&self) -> Option<&[Self]> {
    if let Self::Array(arr) = self {
      Some(arr.as_slice())
    } else {
      None
    }
  }

  /// 尝试获取对象键值切片
  #[inline]
  #[must_use]
  pub fn as_object(&self) -> Option<&[(String, Self)]> {
    if let Self::Object(obj) = self {
      Some(obj.as_slice())
    } else {
      None
    }
  }

  /// 尝试根据键名查询对象属性
  #[inline]
  #[must_use]
  pub fn get(&self, key: &str) -> Option<&Self> {
    if let Self::Object(obj) = self {
      obj.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    } else {
      None
    }
  }

  /// 尝试根据索引查询数组元素
  #[inline]
  #[must_use]
  pub fn get_index(&self, index: usize) -> Option<&Self> {
    if let Self::Array(arr) = self {
      arr.get(index)
    } else {
      None
    }
  }
}

impl fmt::Display for JsonValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Null => write!(f, "null"),
      Self::Bool(b) => write!(f, "{b}"),
      Self::Number(n) => {
        if n.is_nan() || n.is_infinite() {
          write!(f, "null")
        } else if *n == 0.0 && n.is_sign_negative() {
          write!(f, "-0")
        } else {
          write!(f, "{n}")
        }
      }
      Self::String(s) => write!(f, "\"{s}\""),
      Self::Array(arr) => {
        write!(f, "[")?;
        for (i, elem) in arr.iter().enumerate() {
          if i > 0 {
            write!(f, ",")?;
          }
          write!(f, "{elem}")?;
        }
        write!(f, "]")
      }
      Self::Object(entries) => {
        write!(f, "{{")?;
        for (i, (k, v)) in entries.iter().enumerate() {
          if i > 0 {
            write!(f, ",")?;
          }
          write!(f, "\"{k}\":{v}")?;
        }
        write!(f, "}}")
      }
    }
  }
}
