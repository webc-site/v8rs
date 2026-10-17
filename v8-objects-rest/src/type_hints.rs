// 由 tool/src/objects/type-hints.h.js 从 v8/src/objects/type-hints.{h,cc} 生成; 修订就地改, 复跑覆盖
// 类型反馈提示枚举与显示名表
//! 类型提示 (Type Hints): 二元/比较/for-in 操作的 IC 反馈类别。

/// 对应原版 enum BinaryOperationHint : u8, 共 11 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BinaryOperationHint {
  /// kNone → "None"
  None = 0,
  /// kSignedSmall → "SignedSmall"
  Signedsmall = 1,
  /// kSignedSmallInputs → "SignedSmallInputs"
  Signedsmallinputs = 2,
  /// kAdditiveSafeInteger → "AdditiveSafeInteger"
  Additivesafeinteger = 3,
  /// kNumber → "Number"
  Number = 4,
  /// kNumberOrOddball → "NumberOrOddball"
  Numberoroddball = 5,
  /// kString → "String"
  String = 6,
  /// kStringOrStringWrapper → "StringOrStringWrapper"
  Stringorstringwrapper = 7,
  /// kBigInt → "BigInt"
  Bigint = 8,
  /// kBigInt64 → "BigInt64"
  Bigint64 = 9,
  /// kAny → "Any"
  Any = 10,
}

impl BinaryOperationHint {
  /// 变体总数
  pub const COUNT: usize = 11;

  /// 原版 operator<< 显示名
  #[inline]
  #[must_use]
  pub const fn display_name(self) -> &'static str {
    match self {
      Self::None => "None",
      Self::Signedsmall => "SignedSmall",
      Self::Signedsmallinputs => "SignedSmallInputs",
      Self::Additivesafeinteger => "AdditiveSafeInteger",
      Self::Number => "Number",
      Self::Numberoroddball => "NumberOrOddball",
      Self::String => "String",
      Self::Stringorstringwrapper => "StringOrStringWrapper",
      Self::Bigint => "BigInt",
      Self::Bigint64 => "BigInt64",
      Self::Any => "Any",
    }
  }

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::None),
      1 => Some(Self::Signedsmall),
      2 => Some(Self::Signedsmallinputs),
      3 => Some(Self::Additivesafeinteger),
      4 => Some(Self::Number),
      5 => Some(Self::Numberoroddball),
      6 => Some(Self::String),
      7 => Some(Self::Stringorstringwrapper),
      8 => Some(Self::Bigint),
      9 => Some(Self::Bigint64),
      10 => Some(Self::Any),
      _ => None,
    }
  }
}

/// 对应原版 enum CompareOperationHint : u8, 共 14 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CompareOperationHint {
  /// kNone → "None"
  None = 0,
  /// kSignedSmall → "SignedSmall"
  Signedsmall = 1,
  /// kNumber → "Number"
  Number = 2,
  /// kNumberOrBoolean → "NumberOrBoolean"
  Numberorboolean = 3,
  /// kNumberOrOddball → "NumberOrOddball"
  Numberoroddball = 4,
  /// kInternalizedString → "InternalizedString"
  Internalizedstring = 5,
  /// kString → "String"
  String = 6,
  /// kSymbol → "Symbol"
  Symbol = 7,
  /// kBigInt → "BigInt"
  Bigint = 8,
  /// kBigInt64 → "BigInt64"
  Bigint64 = 9,
  /// kReceiver → "Receiver"
  Receiver = 10,
  /// kReceiverOrNullOrUndefined → "ReceiverOrNullOrUndefined"
  Receiverornullorundefined = 11,
  /// kStringOrOddball → "StringOrOddball"
  Stringoroddball = 12,
  /// kAny → "Any"
  Any = 13,
}

impl CompareOperationHint {
  /// 变体总数
  pub const COUNT: usize = 14;

  /// 原版 operator<< 显示名
  #[inline]
  #[must_use]
  pub const fn display_name(self) -> &'static str {
    match self {
      Self::None => "None",
      Self::Signedsmall => "SignedSmall",
      Self::Number => "Number",
      Self::Numberorboolean => "NumberOrBoolean",
      Self::Numberoroddball => "NumberOrOddball",
      Self::Internalizedstring => "InternalizedString",
      Self::String => "String",
      Self::Symbol => "Symbol",
      Self::Bigint => "BigInt",
      Self::Bigint64 => "BigInt64",
      Self::Receiver => "Receiver",
      Self::Receiverornullorundefined => "ReceiverOrNullOrUndefined",
      Self::Stringoroddball => "StringOrOddball",
      Self::Any => "Any",
    }
  }

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::None),
      1 => Some(Self::Signedsmall),
      2 => Some(Self::Number),
      3 => Some(Self::Numberorboolean),
      4 => Some(Self::Numberoroddball),
      5 => Some(Self::Internalizedstring),
      6 => Some(Self::String),
      7 => Some(Self::Symbol),
      8 => Some(Self::Bigint),
      9 => Some(Self::Bigint64),
      10 => Some(Self::Receiver),
      11 => Some(Self::Receiverornullorundefined),
      12 => Some(Self::Stringoroddball),
      13 => Some(Self::Any),
      _ => None,
    }
  }
}

/// 对应原版 enum ForInHint : u8, 共 4 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ForInHint {
  /// kNone → "None"
  None = 0,
  /// kEnumCacheKeysAndIndices → "EnumCacheKeysAndIndices"
  Enumcachekeysandindices = 1,
  /// kEnumCacheKeys → "EnumCacheKeys"
  Enumcachekeys = 2,
  /// kAny → "Any"
  Any = 3,
}

impl ForInHint {
  /// 变体总数
  pub const COUNT: usize = 4;

  /// 原版 operator<< 显示名
  #[inline]
  #[must_use]
  pub const fn display_name(self) -> &'static str {
    match self {
      Self::None => "None",
      Self::Enumcachekeysandindices => "EnumCacheKeysAndIndices",
      Self::Enumcachekeys => "EnumCacheKeys",
      Self::Any => "Any",
    }
  }

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::None),
      1 => Some(Self::Enumcachekeysandindices),
      2 => Some(Self::Enumcachekeys),
      3 => Some(Self::Any),
      _ => None,
    }
  }
}

/// 字符串拼接参数检查标志 (原版 enum StringAddFlags, 非 class 枚举)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum StringAddFlags {
  /// STRING_ADD_CHECK_NONE → "CheckNone"
  StringAddCheckNone = 0,
  /// STRING_ADD_CONVERT_LEFT → "ConvertLeft"
  StringAddConvertLeft = 1,
  /// STRING_ADD_CONVERT_RIGHT → "ConvertRight"
  StringAddConvertRight = 2,
}

impl StringAddFlags {
  /// 变体总数
  pub const COUNT: usize = 3;
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BinaryOperationHint::COUNT == 11);
  assert!(CompareOperationHint::COUNT == 14);
  assert!(ForInHint::COUNT == 4);
  assert!(StringAddFlags::COUNT == 3);
  assert!(BinaryOperationHint::None as u8 == 0);
  assert!(CompareOperationHint::Any as u8 == 13);
  assert!(ForInHint::Any as u8 == 3);
  assert!(StringAddFlags::StringAddConvertRight as u8 == 2);
};
