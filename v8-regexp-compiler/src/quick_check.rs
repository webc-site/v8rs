//! 快速掩码比较检查的数据面 (v8/src/regexp/regexp-compiler.h:78-146
//! QuickCheckDetails + .cc:2226-2275 纯逻辑)。
//!
//! 落地判定: 本类不触 MacroAssembler/Node IR — Rationalize/Merge/Advance/Clear
//! 是纯位运算与区间搬运, 属 wave1 可落地; 消费面 GetQuickCheckDetails 族
//! (Node::EmitQuickCheck 等) 依赖 Node IR 遍历, 二梯队。

/// 原版 QuickCheckDetails::kMaxPositions (h:138)
pub const K_MAX_POSITIONS: usize = 4;

/// 原版 String::kMaxOneByteCharCodeU / kMaxUtf16CodeUnitU (globals.h)
pub const K_MAX_ONE_BYTE_CHAR_CODE: u32 = 0xFF;
/// UTF-16 码元上界
pub const K_MAX_UTF16_CODE_UNIT: u32 = 0xFFFF;

/// 原版 MaxCodeUnit (cc:263-270) — 单字节态 0xFF / 两字节态 0xFFFF
#[must_use]
pub const fn max_code_unit(one_byte: bool) -> u32 {
  if one_byte {
    K_MAX_ONE_BYTE_CHAR_CODE
  } else {
    K_MAX_UTF16_CODE_UNIT
  }
}

/// 原版 CharMask (cc:271-276) — 与 MaxCodeUnit 同值 (两者皆 2^n−1)
#[must_use]
pub const fn char_mask(one_byte: bool) -> u32 {
  max_code_unit(one_byte)
}

/// 原版 SmearBitsRight (cc:1835-1842) — 最高置位向下涂抹
#[must_use]
pub const fn smear_bits_right(mut v: u32) -> u32 {
  v |= v >> 1;
  v |= v >> 2;
  v |= v >> 4;
  v |= v >> 8;
  v |= v >> 16;
  v
}

/// 原版 QuickCheckDetails::Position (h:103-116) — 单字符位的 mask 比较信息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
  pub mask: u32,
  pub value: u32,
  pub determines_perfectly: bool,
  pub cannot_match: bool,
}

impl Position {
  /// 原版 Position() 构造 = Clear() (h:104-106)
  #[must_use]
  pub const fn new() -> Self {
    Self {
      mask: 0,
      value: 0,
      determines_perfectly: false,
      cannot_match: false,
    }
  }

  /// 原版 Position::Clear (h:106-110)
  pub fn clear(&mut self) {
    *self = Self::new();
  }
}

impl Default for Position {
  fn default() -> Self {
    Self::new()
  }
}

/// 原版 QuickCheckDetails (h:78-146): 前瞻多字符的掩码比较 condensate;
/// mask_/value_ 为 Rationalize 后的聚合 (h:143-145 注释)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickCheckDetails {
  /// characters_ (h:141) — 有几个位置带 quick check 信息 (同 choice 各分支一致)
  pub characters: i32,
  /// positions_ (h:142)
  pub positions: [Position; K_MAX_POSITIONS],
  /// mask_ (h:144)
  pub mask: u32,
  /// value_ (h:145)
  pub value: u32,
}

impl Default for QuickCheckDetails {
  fn default() -> Self {
    Self::new()
  }
}

impl QuickCheckDetails {
  /// 原版 QuickCheckDetails() (h:80)
  #[must_use]
  pub const fn new() -> Self {
    Self {
      characters: 0,
      positions: [Position::new(); K_MAX_POSITIONS],
      mask: 0,
      value: 0,
    }
  }

  /// 原版 explicit QuickCheckDetails(int characters) (h:81-84);
  /// DCHECK_LE(characters, kMaxPositions) 承接
  #[must_use]
  pub const fn with_characters(characters: i32) -> Self {
    debug_assert!(characters <= K_MAX_POSITIONS as i32);
    Self {
      characters,
      positions: [Position::new(); K_MAX_POSITIONS],
      mask: 0,
      value: 0,
    }
  }

  /// 原版 set_characters (h:118-121) — DCHECK 0 ≤ c ≤ kMaxPositions
  pub fn set_characters(&mut self, characters: i32) {
    debug_assert!((0..=K_MAX_POSITIONS as i32).contains(&characters));
    self.characters = characters;
  }

  /// 原版 positions(index) (h:122-131) — DCHECK 界内
  #[must_use]
  pub fn position(&self, index: usize) -> &Position {
    debug_assert!((index as i32) < self.characters);
    &self.positions[index]
  }

  #[must_use]
  pub fn position_mut(&mut self, index: usize) -> &mut Position {
    debug_assert!((index as i32) < self.characters);
    &mut self.positions[index]
  }

  /// 原版 cannot_match() (h:91-96)
  #[must_use]
  pub fn cannot_match(&self) -> bool {
    self.positions[..self.characters.max(0) as usize]
      .iter()
      .any(|p| p.cannot_match)
  }

  /// 原版 set_cannot_match_from(index) (h:97-102); DCHECK_GE(index, 0)
  pub fn set_cannot_match_from(&mut self, index: i32) {
    debug_assert!(index >= 0);
    for i in index.max(0) as usize..self.characters.max(0) as usize {
      self.positions[i].cannot_match = true;
    }
  }

  /// 原版 Clear() (cc:2226-2231)
  pub fn clear(&mut self) {
    for i in 0..self.characters.max(0) as usize {
      self.positions[i].clear();
    }
    self.characters = 0;
  }

  /// 原版 Advance(by, one_byte) (cc:2233-2251) — 窗口前移 by, 尾部清空
  pub fn advance(&mut self, by: i32, one_byte: bool) {
    if by >= self.characters || by < 0 {
      // DCHECK_IMPLIES(by < 0, characters_ == 0) (cc:2235)
      debug_assert!(by >= 0 || self.characters == 0);
      self.clear();
      return;
    }
    debug_assert!(self.characters - by <= K_MAX_POSITIONS as i32);
    debug_assert!(self.characters <= K_MAX_POSITIONS as i32);
    for i in 0..self.characters - by {
      self.positions[i as usize] = self.positions[(by + i) as usize];
    }
    for i in self.characters - by..self.characters {
      self.positions[i as usize].clear();
    }
    self.characters -= by;
    // 原版 cc:2248-2250 注释: mask_/value_ 不更新 (用过即弃)
    let _ = one_byte;
  }

  /// 原版 Rationalize(asc) (cc:1846-1862) — 各位置 mask/value 拼成宽字比较常量。
  /// 端序登记: 一字节态移位 0/8/16/24, 两字节态 0/16/32/48; 后者 ≥32 在原版为
  /// UB (uint32_t <<), 本实现按 x86/arm64 实际语义取 shift mod 32
  /// (wrapping_shl), 引擎两字节态 characters ≤ 2 (in_quickcheck_range 佐证,
  /// cc:3926-3929), 溢出臂仅保真兜底
  pub fn rationalize(&mut self, one_byte: bool) -> bool {
    let mut found_useful_op = false;
    let mask_unit = char_mask(one_byte);
    self.mask = 0;
    self.value = 0;
    let mut char_shift = 0u32;
    for i in 0..self.characters.max(0) as usize {
      let pos = &mut self.positions[i];
      if pos.mask & K_MAX_ONE_BYTE_CHAR_CODE != 0 {
        found_useful_op = true;
      }
      self.mask |= (pos.mask & mask_unit).wrapping_shl(char_shift);
      self.value |= (pos.value & mask_unit).wrapping_shl(char_shift);
      char_shift += if one_byte { 8 } else { 16 };
    }
    found_useful_op
  }

  /// 原版 Merge(other, from_index) (cc:2253-2275) — choice 两分支信息合并,
  /// 双方 value 均被掩到新公共 mask (other 亦被改写, 与 C++ 逐字一致)
  pub fn merge(&mut self, other: &mut Self, from_index: i32) {
    debug_assert!(self.characters == other.characters);
    for i in from_index.max(0) as usize..self.characters.max(0) as usize {
      let pos = &mut self.positions[i];
      let other_pos = &mut other.positions[i];
      if pos.cannot_match {
        *pos = *other_pos;
      } else if !other_pos.cannot_match {
        if pos.mask != other_pos.mask
          || pos.value != other_pos.value
          || !other_pos.determines_perfectly
        {
          // 两侧掩码比较不同 → 近似
          pos.determines_perfectly = false;
        }
        pos.mask &= other_pos.mask;
        pos.value &= pos.mask;
        other_pos.value &= pos.mask;
        let differing_bits = pos.value ^ other_pos.value;
        pos.mask &= !differing_bits;
        pos.value &= pos.mask;
      }
    }
  }
}
