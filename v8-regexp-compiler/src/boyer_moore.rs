//! Boyer-Moore 跳扫描前瞻的纯数据面 (v8/src/regexp/regexp-compiler.h:173-207 +
//! .cc:3730-3810)。
//!
//! 落地判定: ContainedInLattice 格 / AddRange 判属 / BoyerMoorePositionInfo 位图
//! 不触 MacroAssembler — 落地; BoyerMooreLookahead 容器 (at/Count/Set/SetRest)
//! 纯数据 — 落地; 其 EmitSkipInstructions/BuildSkipTable/GetSkipTable 依赖
//! RegExpMacroAssembler + Handle<ByteArray>, FindWorthwhile/FindBestInterval 依赖
//! Compiler::frequency_collator 注入 — 前者二梯队, 后者以参数注入形态落地。
//!
//! std::bitset<128> → u128 (原版 BitsetFirstSetBit 的双 64 位分段在 u128
//! trailing_zeros 下等价, cc:3757-3781)。

use crate::quick_check::max_code_unit;

/// 原版 ContainedInLattice (h:173-178) — 区间相对字符类格 (lattice) 的包含态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ContainedInLattice {
  /// 尚未判定
  NotYet = 0,
  /// 完全在类内
  LatticeIn = 1,
  /// 完全在类外
  LatticeOut = 2,
  /// 无法判定 (也可表示既内又外)
  LatticeUnknown = 3,
}

/// 原版 Combine(a, b) = static_cast<...>(a | b) (h:180-182)
#[must_use]
pub const fn combine(a: ContainedInLattice, b: ContainedInLattice) -> ContainedInLattice {
  match (a as u8) | (b as u8) {
    0 => ContainedInLattice::NotYet,
    1 => ContainedInLattice::LatticeIn,
    2 => ContainedInLattice::LatticeOut,
    _ => ContainedInLattice::LatticeUnknown,
  }
}

/// 原版 kRangeEndMarker (compiler_constants, h:43) — [from, to+1) 表终结标记
pub const K_RANGE_END_MARKER: u32 = 0x110000;

/// \w 的 [from, to+1) 半开边界序列 (原版 kWordRanges, h:50-51;
/// 与冻结表 v8-regexp-syntax::WORD_RANGES 4 闭区间对逐项对应, 同源性登记 —
/// static 不可进 const 上下文, 对数由单测运行时校验)
const WORD_BOUNDARIES: &[u32] = &[
  48,
  58, // '0'..'9'+1
  65,
  91, // 'A'..'Z'+1
  95,
  96, // '_'.+1
  97,
  123, // 'a'..'z'+1
  K_RANGE_END_MARKER,
];
// 边界序列实数: 4 对 × 2 + 终结标记 = 9 (奇数, ≡ DCHECK_EQ(1, length & 1))
const _: () = assert!(WORD_BOUNDARIES.len() == 9);
const _: () = assert!(WORD_BOUNDARIES[WORD_BOUNDARIES.len() - 1] == K_RANGE_END_MARKER);

/// WORD_BOUNDARIES 只读视图 (同源性对数入口: 单测与冻结表
/// v8-regexp-syntax::tables_generated::WORD_RANGES 逐项校验)
#[must_use]
pub const fn word_boundaries() -> &'static [u32] {
  WORD_BOUNDARIES
}

/// 原版 AddRange (cc:3736-3755) — 区间相对 [from, to+1) 边界表的格包含判定;
/// new_interval 为闭区间 [from, to]
#[must_use]
pub fn add_range_lattice(
  containment: ContainedInLattice,
  boundaries: &[u32],
  from: u32,
  to: u32,
) -> ContainedInLattice {
  debug_assert!(boundaries.len() & 1 == 1, "DCHECK_EQ(1, ranges_length & 1)"); // 奇数: 对+终结
  debug_assert!(boundaries[boundaries.len() - 1] == K_RANGE_END_MARKER);
  if containment == ContainedInLattice::LatticeUnknown {
    return containment;
  }
  let mut inside = false;
  let mut last = 0u32;
  for &boundary in boundaries {
    // cc:3744-3745 逐段推进: 每段 [last, boundary) 的 inside/outside 交替
    let result = if boundary <= from {
      // 还没走到新区间所在段
      None
    } else if last <= from && to < boundary {
      // 新区间完全落在 [last, boundary) 段内 (to 端含, 表值端不含)
      Some(combine(
        containment,
        if inside {
          ContainedInLattice::LatticeIn
        } else {
          ContainedInLattice::LatticeOut
        },
      ))
    } else {
      Some(ContainedInLattice::LatticeUnknown)
    };
    if let Some(r) = result {
      return r;
    }
    inside = !inside;
    last = boundary;
  }
  containment
}

/// 单位置位图信息 (h:184-207 BoyerMoorePositionInfo)。
/// ZoneObject → 普通值类型; Bitset(128) → u128
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoyerMoorePositionInfo {
  /// map_ (h:204)
  map: u128,
  /// map_count_ (h:205) — map 置位数
  map_count: i32,
  /// w_ (h:206) — \w 类格判定缓存
  w: ContainedInLattice,
}

/// 原版 kMapSize (h:188) — 128 字符窗口 (≡ RegExpMacroAssembler::kTableSize)
pub const K_MAP_SIZE: i32 = 128;
/// 原版 kMask (h:189)
pub const K_MAP_MASK: u32 = K_MAP_SIZE as u32 - 1;

impl Default for BoyerMoorePositionInfo {
  fn default() -> Self {
    Self::new()
  }
}

impl BoyerMoorePositionInfo {
  /// 原版默认构造 (h:184 成员初值: map 全 0, map_count_=0, w_=kNotYet)
  #[must_use]
  pub const fn new() -> Self {
    Self {
      map: 0,
      map_count: 0,
      w: ContainedInLattice::NotYet,
    }
  }

  /// 原版 at(i) (h:186)
  #[must_use]
  pub fn at(&self, i: u32) -> bool {
    debug_assert!(i < K_MAP_SIZE as u32);
    self.map & (1u128 << i) != 0
  }

  /// 原版 map_count() (h:191)
  #[must_use]
  pub fn map_count(&self) -> i32 {
    self.map_count
  }

  /// 原版 raw_bitset() (h:200-201)
  #[must_use]
  pub fn raw_bitset(&self) -> u128 {
    self.map
  }

  /// 原版 is_non_word() / is_word() (h:197-198)
  #[must_use]
  pub fn is_non_word(&self) -> bool {
    self.w == ContainedInLattice::LatticeOut
  }

  #[must_use]
  pub fn is_word(&self) -> bool {
    self.w == ContainedInLattice::LatticeIn
  }

  /// 原版 w_ 只读面 (判定台账用)
  #[must_use]
  pub fn containment(&self) -> ContainedInLattice {
    self.w
  }

  /// 原版 Set(character) (cc:3730-3732) = SetInterval([c, c])
  pub fn set(&mut self, character: u32) {
    self.set_interval(character, character);
  }

  /// 原版 SetInterval(Interval) (cc:3785-3802) — 闭区间 [from, to]
  pub fn set_interval(&mut self, from: u32, to: u32) {
    self.w = add_range_lattice(self.w, WORD_BOUNDARIES, from, to);

    let size = to.saturating_sub(from) + 1;
    if size >= K_MAP_SIZE as u32 {
      self.map_count = K_MAP_SIZE;
      self.map = u128::MAX;
      return;
    }

    for i in from..=to {
      let mod_character = i & K_MAP_MASK;
      let bit = 1u128 << mod_character;
      if self.map & bit == 0 {
        self.map_count += 1;
        self.map |= bit;
      }
      if self.map_count == K_MAP_SIZE {
        return;
      }
    }
  }

  /// 原版 SetAll() (cc:3804-3810)
  pub fn set_all(&mut self) {
    self.w = ContainedInLattice::LatticeUnknown;
    if self.map_count != K_MAP_SIZE {
      self.map_count = K_MAP_SIZE;
      self.map = u128::MAX;
    }
  }
}

/// 原版 BitsetFirstSetBit (cc:3757-3781) — 双 64 位分段取最低置位;
/// u128::trailing_zeros 与之逐值一致, 空集返回 -1
#[must_use]
pub fn bitset_first_set_bit(bitset: u128) -> i32 {
  if bitset == 0 {
    -1
  } else {
    bitset.trailing_zeros() as i32
  }
}

/// 原版 BoyerMooreLookahead 的纯容器面 (h:209-291 中不触 masm/Handle 的部分):
/// length 个位置位图 + max_char 截断。Emit*/BuildSkipTable/GetSkipTable 二梯队
/// (宏汇编器 + Handle 耦合); FindBestInterval 以频率回调参数注入落地。
#[derive(Debug, Clone)]
pub struct BoyerMooreLookahead {
  /// length_ (h:281)
  pub length: i32,
  /// max_char_ (h:284): 0xFF 一字节态 / 0xFFFF 两字节态
  pub max_char: u32,
  /// bitmaps_ (h:285) ZoneList → Vec
  bitmaps: Vec<BoyerMoorePositionInfo>,
}

impl BoyerMooreLookahead {
  /// 原版构造 (cc:3812-3822) — compiler->one_byte() 以布尔参数注入
  #[must_use]
  pub fn new(length: i32, one_byte: bool) -> Self {
    Self {
      length,
      max_char: max_code_unit(one_byte),
      bitmaps: vec![BoyerMoorePositionInfo::new(); length.max(0) as usize],
    }
  }

  /// 原版 at(i) (h:219-220)
  #[must_use]
  pub fn at(&self, i: usize) -> &BoyerMoorePositionInfo {
    &self.bitmaps[i]
  }

  #[must_use]
  pub fn at_mut(&mut self, i: usize) -> &mut BoyerMoorePositionInfo {
    &mut self.bitmaps[i]
  }

  /// 原版 Count(map_number) (h:217)
  #[must_use]
  pub fn count(&self, map_number: usize) -> i32 {
    self.bitmaps[map_number].map_count()
  }

  /// 原版 Set(map_number, character) (h:222-226) — 超 max_char 直弃
  pub fn set(&mut self, map_number: usize, character: u32) {
    if character > self.max_char {
      return;
    }
    self.bitmaps[map_number].set(character);
  }

  /// 原版 SetInterval(map_number, interval) (h:228-236) — 区间与 max_char 截断
  pub fn set_interval(&mut self, map_number: usize, from: u32, to: u32) {
    if from > self.max_char {
      return;
    }
    if to > self.max_char {
      self.bitmaps[map_number].set_interval(from, self.max_char);
    } else {
      self.bitmaps[map_number].set_interval(from, to);
    }
  }

  /// 原版 SetAll(map_number) (h:238)
  pub fn set_all(&mut self, map_number: usize) {
    self.bitmaps[map_number].set_all();
  }

  /// 原版 SetRest(from_map) (h:240-242)
  pub fn set_rest(&mut self, from_map: usize) {
    for i in from_map..self.length.max(0) as usize {
      self.set_all(i);
    }
  }

  /// 原版 FindWorthwhileInterval (cc:3826-3845) — 扫描 kMaxMax=32 内翻倍预算;
  /// frequency(char) 回调注入 FrequencyCollator::Frequency 语义
  /// (原版经 compiler_->frequency_collator()->Frequency(j), j 为位图字符;
  /// 此处参数化以解耦 Compiler)
  #[must_use]
  pub fn find_worthwhile_interval<F>(&self, one_byte: bool, frequency: F) -> Option<(i32, i32)>
  where
    F: Fn(u32) -> i32,
  {
    let mut biggest_points = 0;
    let mut from = 0i32;
    let mut to = 0i32;
    let mut max_number_of_chars = 4;
    while max_number_of_chars < 32 {
      biggest_points = self.find_best_interval(
        max_number_of_chars,
        biggest_points,
        one_byte,
        &frequency,
        &mut from,
        &mut to,
      );
      max_number_of_chars *= 2;
    }
    if biggest_points == 0 {
      None
    } else {
      Some((from, to))
    }
  }

  /// 原版 FindBestInterval (cc:3846-3894) — 概率×跨度取最高分区间;
  /// 出参 from/to 仅在刷新纪录处覆写 (与 C++ 指针出参逐字一致)
  #[must_use]
  fn find_best_interval<F>(
    &self,
    max_number_of_chars: i32,
    old_biggest_points: i32,
    one_byte: bool,
    frequency: &F,
    from: &mut i32,
    to: &mut i32,
  ) -> i32
  where
    F: Fn(u32) -> i32,
  {
    let mut biggest_points = old_biggest_points;
    let length = self.length;
    let mut i = 0i32;
    while i < length {
      while i < length && self.count(i as usize) > max_number_of_chars {
        i += 1;
      }
      if i == length {
        break;
      }
      let remembered_from = i;
      let mut union_bitset = 0u128;
      while i < length && self.count(i as usize) <= max_number_of_chars {
        union_bitset |= self.at(i as usize).raw_bitset();
        i += 1;
      }

      // 原版逐置位迭代 (cc:3866-3875, 每字符 +1 提升采样不足)
      let mut freq_total = 0i32;
      let mut j = bitset_first_set_bit(union_bitset);
      while j >= 0 {
        debug_assert!(union_bitset & (1u128 << j) != 0);
        freq_total += frequency(j as u32) + 1;
        union_bitset &= !(1u128 << j);
        j = bitset_first_set_bit(union_bitset);
      }

      // cc:3880-3882: 快速检查窗口内 (跨度 <4 或起点 ≤4/≤2) 用半概率基准
      let in_quickcheck_range = (i - remembered_from < 4)
        || (if one_byte {
          remembered_from <= 4
        } else {
          remembered_from <= 2
        });
      let probability = (if in_quickcheck_range {
        K_MAP_SIZE / 2
      } else {
        K_MAP_SIZE
      }) - freq_total;
      let points = (i - remembered_from) * probability;
      if points > biggest_points {
        *from = remembered_from;
        *to = i - 1;
        biggest_points = points;
      }
    }
    biggest_points
  }
}
