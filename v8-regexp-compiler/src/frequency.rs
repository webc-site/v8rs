//! v8/src/regexp/regexp-compiler.h 的 FrequencyCollator 数据面 1:1 转写
//!
//! 对应 C++ `regexp::FrequencyCollator` (regexp-compiler.h:544-587)。
//! 纯统计逻辑, 不触 MacroAssembler/Compiler, 属 wave1 可落地面。
//! 表尺寸常量来自 RegExpMacroAssembler (regexp-macro-assembler.h:48-50):
//! kTableSizeBits=7, kTableSize=128, kTableMask=127。

use std::array::from_fn;

/// RegExpMacroAssembler::kTableSizeBits (regexp-macro-assembler.h:48)
pub const K_TABLE_SIZE_BITS: i32 = 7;
/// RegExpMacroAssembler::kTableSize = 1 << 7 = 128 (regexp-macro-assembler.h:49)
pub const K_TABLE_SIZE: usize = 1 << K_TABLE_SIZE_BITS;
/// RegExpMacroAssembler::kTableMask (regexp-macro-assembler.h:50)
pub const K_TABLE_MASK: i32 = K_TABLE_SIZE as i32 - 1;

/// C++ 内部类 FrequencyCollator::CharacterFrequency (regexp-compiler.h:569-582)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterFrequency {
  counter: i32,
  character: i32,
}

impl CharacterFrequency {
  /// 默认构造: counter_=0, character_=-1 (仅用于 zero-init 前的占位;
  /// FrequencyCollator 构造会用显式 character 覆写全表)。
  pub const fn new() -> Self {
    Self {
      counter: 0,
      character: -1,
    }
  }
  /// explicit CharacterFrequency(int character)
  pub const fn with_character(character: i32) -> Self {
    Self {
      counter: 0,
      character,
    }
  }
  /// void Increment()
  pub fn increment(&mut self) {
    self.counter += 1;
  }
  /// int counter()
  pub const fn counter(&self) -> i32 {
    self.counter
  }
  /// int character()
  pub const fn character(&self) -> i32 {
    self.character
  }
}

impl Default for CharacterFrequency {
  fn default() -> Self {
    Self::new()
  }
}

/// FrequencyCollator (regexp-compiler.h:544-566)。
///
/// 按 128 桶统计字符频率; Frequency 以 per-128 (而非百分比) 计量,
/// 分母为总样本数。
#[derive(Debug, Clone)]
pub struct FrequencyCollator {
  frequencies: [CharacterFrequency; K_TABLE_SIZE],
  total_samples: i32,
}

impl FrequencyCollator {
  /// 构造函数: 全表以 CharacterFrequency(i) 初始化, total_samples_=0。
  pub fn new() -> Self {
    let frequencies = from_fn(|i| CharacterFrequency::with_character(i as i32));
    Self {
      frequencies,
      total_samples: 0,
    }
  }

  /// void CountCharacter(int character) (regexp-compiler.h:552-556)。
  /// 原版以 character & kTableMask 索引, 负字符 (如 -1) 经按位与同样落入 0..127。
  pub fn count_character(&mut self, character: i32) {
    let index = (character & K_TABLE_MASK) as usize;
    self.frequencies[index].increment();
    self.total_samples += 1;
  }

  /// int Frequency(int in_character) (regexp-compiler.h:560-566)。
  ///
  /// 原版 DCHECK((in_character & kTableMask) == in_character) → debug_assert。
  /// total_samples_ < 1 时返回 1 (防除零); 否则 counter*128/total (per-128)。
  pub fn frequency(&mut self, in_character: i32) -> i32 {
    debug_assert!((in_character & K_TABLE_MASK) == in_character);
    if self.total_samples < 1 {
      return 1; // Division by zero.
    }

    (self.frequencies[in_character as usize].counter() * 128) / self.total_samples
  }

  /// 只读访问底层桶 (金标准对数用; 原版为 private)。
  pub fn bucket(&self, index: usize) -> &CharacterFrequency {
    &self.frequencies[index]
  }

  /// total_samples_ 只读访问 (原版无公开 getter, 供测试/BoyerMoore 决策对数)。
  pub const fn total_samples(&self) -> i32 {
    self.total_samples
  }
}

impl Default for FrequencyCollator {
  fn default() -> Self {
    Self::new()
  }
}
