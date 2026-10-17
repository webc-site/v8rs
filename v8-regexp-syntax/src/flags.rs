//! RegExp 标志位语义层 (原版 base::Flags<Flag> + Is##Camel / TryFlagFromChar / operator<<)

use std::{
  fmt,
  ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
  str::FromStr,
};

use crate::{
  error::Error,
  flags_generated::{
    DOTALL, FLAG_SPECS, GLOBAL, HASINDICES, IGNORECASE, LINEAR, MULTILINE, NUM_FLAGS, STICKY,
    UNICODE, UNICODESETS,
  },
  regexp_error_generated::RegexpError,
};

/// 正则标志集合 (原版 `using Flags = base::Flags<Flag>`, 位序 1:1)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct RegExpFlags(pub u16);

impl RegExpFlags {
  /// 空标志集
  pub const NONE: Self = Self(0);
  pub const HAS_INDICES: Self = Self(HASINDICES);
  pub const GLOBAL: Self = Self(GLOBAL);
  pub const IGNORE_CASE: Self = Self(IGNORECASE);
  pub const LINEAR: Self = Self(LINEAR);
  pub const MULTILINE: Self = Self(MULTILINE);
  pub const DOT_ALL: Self = Self(DOTALL);
  pub const UNICODE: Self = Self(UNICODE);
  pub const UNICODE_SETS: Self = Self(UNICODESETS);
  pub const STICKY: Self = Self(STICKY);

  #[inline]
  pub const fn has_indices(self) -> bool {
    self.0 & HASINDICES != 0
  }
  #[inline]
  pub const fn is_global(self) -> bool {
    self.0 & GLOBAL != 0
  }
  #[inline]
  pub const fn ignore_case(self) -> bool {
    self.0 & IGNORECASE != 0
  }
  #[inline]
  pub const fn linear(self) -> bool {
    self.0 & LINEAR != 0
  }
  #[inline]
  pub const fn multiline(self) -> bool {
    self.0 & MULTILINE != 0
  }
  #[inline]
  pub const fn dot_all(self) -> bool {
    self.0 & DOTALL != 0
  }
  #[inline]
  pub const fn unicode(self) -> bool {
    self.0 & UNICODE != 0
  }
  #[inline]
  pub const fn unicode_sets(self) -> bool {
    self.0 & UNICODESETS != 0
  }
  #[inline]
  pub const fn sticky(self) -> bool {
    self.0 & STICKY != 0
  }

  /// /u 或 /v (原版 IsEitherUnicode)
  #[inline]
  pub const fn either_unicode(self) -> bool {
    self.unicode() || self.unicode_sets()
  }

  /// 是否需要在代理对中间回退索引 (原版 ShouldOptionallyStepBackToLeadSurrogate)
  #[inline]
  pub const fn should_step_back_to_lead_surrogate(self) -> bool {
    self.either_unicode() && (self.is_global() || self.sticky())
  }

  /// 是否包含指定标志
  #[inline]
  pub const fn contains(self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }

  /// 置位/清位 (原版 flags.set(flag, polarity))
  #[inline]
  pub fn set(&mut self, flag: Self, polarity: bool) {
    if polarity {
      self.0 |= flag.0;
    } else {
      self.0 &= !flag.0;
    }
  }

  /// 标志集校验 (如 u 与 v 互斥)
  pub fn validate(self) -> Result<(), Error> {
    if self.unicode() && self.unicode_sets() {
      return Err(Error::parse(RegexpError::InvalidFlagGroup, 0));
    }
    Ok(())
  }

  /// 从字符串解析标志位集合 (如 "gimsuy")
  pub fn parse(s: &str) -> Result<Self, Error> {
    let mut flags = Self::NONE;
    for (pos, c) in s.char_indices() {
      let Some(flag) = try_flag_from_char(c) else {
        return Err(Error::parse(RegexpError::InvalidFlagGroup, pos));
      };
      if flags.contains(flag) {
        return Err(Error::parse(RegexpError::RepeatedFlag, pos));
      }
      flags |= flag;
    }
    flags.validate()?;
    Ok(flags)
  }
}

impl FromStr for RegExpFlags {
  type Err = Error;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parse(s)
  }
}

impl BitOr for RegExpFlags {
  type Output = Self;
  #[inline]
  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}

impl BitOrAssign for RegExpFlags {
  #[inline]
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 |= rhs.0;
  }
}

impl BitAnd for RegExpFlags {
  type Output = Self;
  #[inline]
  fn bitand(self, rhs: Self) -> Self {
    Self(self.0 & rhs.0)
  }
}

impl BitAndAssign for RegExpFlags {
  #[inline]
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 &= rhs.0;
  }
}

impl Not for RegExpFlags {
  type Output = Self;
  #[inline]
  fn not(self) -> Self {
    Self(!self.0 & ((1 << NUM_FLAGS) - 1))
  }
}

/// 按字符查询单个标志 (原版 TryFlagFromChar; 'd''g''i''l''m''s''u''v''y')
#[inline]
#[must_use]
pub const fn try_flag_from_char(c: char) -> Option<RegExpFlags> {
  let bit = match c {
    'd' => HASINDICES,
    'g' => GLOBAL,
    'i' => IGNORECASE,
    'l' => LINEAR,
    'm' => MULTILINE,
    's' => DOTALL,
    'u' => UNICODE,
    'v' => UNICODESETS,
    'y' => STICKY,
    _ => return None,
  };
  Some(RegExpFlags(bit))
}

/// 字符串形态 (原版 operator<<: 按宏序即字符升序输出)
#[inline]
#[must_use]
pub fn flags_to_string(flags: RegExpFlags) -> String {
  let mut s = String::with_capacity(NUM_FLAGS);
  for &(c, bit) in &FLAG_SPECS {
    if flags.0 & bit != 0 {
      s.push(c);
    }
  }
  s
}

impl fmt::Display for RegExpFlags {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for &(c, bit) in &FLAG_SPECS {
      if self.0 & bit != 0 {
        f.write_str(c.encode_utf8(&mut [0; 4]))?;
      }
    }
    Ok(())
  }
}
