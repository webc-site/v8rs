// 由 tool/src/strings/string-search.h.js 从 v8/src/strings/string-search.h 生成; 修订就地改, 复跑覆盖
// BM 常量 1:1; Isolate 三表内联为结构体成员; 语义体已就地补写

//! 字符串搜索: 单字符 / 线性 / Boyer-Moore / Boyer-Moore-Horspool 策略自动升降级。
//! 原版把 bad-char / good-suffix / suffix 三表挂在 Isolate (线程局部复用);
//! Rust 版内联到结构体, 算法与升降级阈值 1:1; FindFirstCharacter 的单字节底层
//! 由编译器自动向量化承担 (原版 memchr 位点), 双字节 NUL 特判保留。

/// Boyer-Moore 最大位移上限 (kBMMaxShift)。
pub const BM_MAX_SHIFT: usize = 250;
/// Latin-1 字母表缩减规模 (kLatin1AlphabetSize)。
pub const LATIN1_ALPHABET_SIZE: usize = 256;
/// UC16 字母表缩减规模 (kUC16AlphabetSize; 等价类 = c mod 256)。
pub const UC16_ALPHABET_SIZE: usize = 256;
/// 低于该 pattern 长度时 BM 位移收益抵不过建表开销 (kBMMinPatternLength)。
pub const BM_MIN_PATTERN_LENGTH: usize = 7;

/// 搜索策略 (原版函数指针 strategy_ 的具象化)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
  /// pattern 含 Latin-1 外字符而 subject 为单字节 → 必败。
  Fail,
  /// 单字符 pattern。
  SingleChar,
  /// 短 pattern 线性搜索 (永不升降级)。
  Linear,
  /// 首次搜索: 线性 + badness 超限升级 BMH。
  Initial,
  BoyerMooreHorspool,
  BoyerMoore,
}

/// 宽度组合: (pattern 单字节?, subject 单字节?) — CharOccurrence 分派依据。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Widths {
  pattern_one_byte: bool,
  subject_one_byte: bool,
}

/// StringSearch<PatternChar, SubjectChar> — 4 组合 (u8/u16 × u8/u16)。
#[derive(Debug)]
pub struct StringSearch<'p, P> {
  pattern: &'p [P],
  /// max(0, len - BM_MAX_SHIFT): 建表只看 pattern 末段。
  start: usize,
  strategy: Strategy,
  /// bad-char 位移表 (长度 = 字母表缩减规模; 原版 isolate_->bad_char_shift_table)。
  bad_char_occurrence: Vec<i32>,
  /// good-suffix 位移表 (索引 start..=len; 原版 isolate_->good_suffix_shift_table 偏移 start 的 biased 视图)。
  good_suffix_shift: Vec<i32>,
  /// 建表用 suffix 链 (原版 isolate_->suffix_table)。
  suffix_table: Vec<i32>,
}

impl<'p> StringSearch<'p, u8> {
  /// Latin-1 pattern 构造。
  #[must_use]
  pub fn new_u8(pattern: &'p [u8]) -> Self {
    Self::build(pattern, true)
  }

  /// 在单字节 subject 中搜索。
  pub fn search(&mut self, subject: &[u8], index: usize) -> Option<usize> {
    if self.strategy == Strategy::Fail {
      return None;
    }
    self.dispatch(
      subject,
      index,
      Widths {
        pattern_one_byte: true,
        subject_one_byte: true,
      },
    )
  }

  /// 在双字节 subject 中搜索。
  pub fn search_u16(&mut self, subject: &[u16], index: usize) -> Option<usize> {
    if self.strategy == Strategy::Fail {
      return None;
    }
    self.dispatch(
      subject,
      index,
      Widths {
        pattern_one_byte: true,
        subject_one_byte: false,
      },
    )
  }
}

impl<'p> StringSearch<'p, u16> {
  /// UC16 pattern 构造。
  #[must_use]
  pub fn new_u16(pattern: &'p [u16]) -> Self {
    Self::build(pattern, false)
  }

  /// 在单字节 subject 中搜索: pattern 含 Latin-1 外字符时必败 (FailSearch)。
  pub fn search_u8(&mut self, subject: &[u8], index: usize) -> Option<usize> {
    if self.strategy == Strategy::Fail || self.pattern.iter().any(|&c| c > 0xFF) {
      self.strategy = Strategy::Fail;
      return None;
    }
    self.dispatch(
      subject,
      index,
      Widths {
        pattern_one_byte: false,
        subject_one_byte: true,
      },
    )
  }

  /// 在双字节 subject 中搜索。
  pub fn search_u16(&mut self, subject: &[u16], index: usize) -> Option<usize> {
    if self.strategy == Strategy::Fail {
      return None;
    }
    self.dispatch(
      subject,
      index,
      Widths {
        pattern_one_byte: false,
        subject_one_byte: false,
      },
    )
  }
}

impl<'p, P: Copy> StringSearch<'p, P> {
  /// 共用构造 (原版构造函数): 策略按长度选择。
  fn build(pattern: &'p [P], _pattern_one_byte: bool) -> StringSearch<'p, P> {
    let pattern_length = pattern.len();
    let start = pattern_length.saturating_sub(BM_MAX_SHIFT);
    let strategy = if pattern_length < BM_MIN_PATTERN_LENGTH {
      if pattern_length == 1 {
        Strategy::SingleChar
      } else {
        Strategy::Linear
      }
    } else {
      Strategy::Initial
    };
    StringSearch {
      pattern,
      start,
      strategy,
      bad_char_occurrence: Vec::new(),
      good_suffix_shift: Vec::new(),
      suffix_table: Vec::new(),
    }
  }
}

/// pattern 字符等价类桶: u8 → 字节值, u16 → c mod 256。
#[inline]
fn bucket_of(c: u32, pattern_one_byte: bool) -> usize {
  if pattern_one_byte {
    c as usize
  } else {
    (c % UC16_ALPHABET_SIZE as u32) as usize
  }
}

impl<'p, P: Copy + Into<u32> + PartialEq> StringSearch<'p, P> {
  /// 当前策略。
  #[must_use]
  pub fn strategy(&self) -> Strategy {
    self.strategy
  }

  /// 置为必败策略 (原版构造函数中 pattern 放不进 subject 的分支)。
  pub fn fail(&mut self) {
    self.strategy = Strategy::Fail;
  }

  /// bad-char 表查询 (CharOccurrence): 语义随宽度组合分派。
  #[inline]
  fn occ(&self, subject_char: u32, wd: Widths) -> i32 {
    let idx = if wd.subject_one_byte {
      subject_char as usize
    } else if wd.pattern_one_byte {
      if subject_char > 0xFF {
        return -1;
      }
      subject_char as usize
    } else {
      (subject_char % UC16_ALPHABET_SIZE as u32) as usize
    };
    self.bad_char_occurrence.get(idx).copied().unwrap_or(-1)
  }

  /// 填充 bad-char 表 (PopulateBoyerMooreHorspoolTable)。
  fn populate_horspool(&mut self, wd: Widths) {
    let pattern_length = self.pattern.len();
    let table_size = if wd.pattern_one_byte {
      LATIN1_ALPHABET_SIZE
    } else {
      UC16_ALPHABET_SIZE
    };
    if self.bad_char_occurrence.len() < table_size {
      self.bad_char_occurrence.resize(table_size, 0);
    }
    let start = self.start;
    if start == 0 {
      // 全 -1 (原版 memset)
      self.bad_char_occurrence.iter_mut().for_each(|v| *v = -1);
    } else {
      self.bad_char_occurrence[..table_size]
        .iter_mut()
        .for_each(|v| *v = start as i32 - 1);
    }
    // 正向扫描: 每个等价类记录最后出现 (不含末字符)
    if pattern_length > 0 {
      for i in start..pattern_length - 1 {
        let c = self.pattern[i];
        self.bad_char_occurrence[bucket_of((c).into(), wd.pattern_one_byte)] = i as i32;
      }
    }
  }

  /// 填充 good-suffix 表 (PopulateBoyerMooreTable)。
  fn populate_boyer_moore(&mut self) {
    let pattern_length = self.pattern.len();
    if self.good_suffix_shift.len() <= pattern_length {
      self.good_suffix_shift.resize(pattern_length + 1, 0);
    }
    if self.suffix_table.len() <= pattern_length {
      self.suffix_table.resize(pattern_length + 1, 0);
    }
    let start = self.start;
    let length = pattern_length - start;

    for v in &mut self.good_suffix_shift[start..=pattern_length] {
      *v = length as i32;
    }
    self.good_suffix_shift[pattern_length] = 1;
    self.suffix_table[pattern_length] = pattern_length as i32 + 1;

    if pattern_length <= start {
      return;
    }

    let last_char = self.pattern[pattern_length - 1];
    let mut sfx = pattern_length + 1usize;
    let mut i = pattern_length;
    while i > start {
      let c = self.pattern[i - 1];
      while sfx <= pattern_length && (c).into() != (self.pattern[sfx - 1]).into() {
        if self.good_suffix_shift[sfx] == length as i32 {
          self.good_suffix_shift[sfx] = (sfx - i) as i32;
        }
        sfx = self.suffix_table[sfx] as usize;
      }
      i -= 1;
      sfx -= 1;
      self.suffix_table[i] = sfx as i32;
      if sfx == pattern_length {
        // 无后缀可扩展: 仅对 last_char 检查
        while i > start && self.pattern[i - 1] != last_char {
          if self.good_suffix_shift[pattern_length] == length as i32 {
            self.good_suffix_shift[pattern_length] = (pattern_length - i) as i32;
          }
          i -= 1;
          self.suffix_table[i] = pattern_length as i32;
        }
        if i > start {
          i -= 1;
          sfx -= 1;
          self.suffix_table[i] = sfx as i32;
        }
      }
    }
    // 用后缀链回填
    if sfx < pattern_length {
      for i in start..=pattern_length {
        if self.good_suffix_shift[i] == length as i32 {
          self.good_suffix_shift[i] = (sfx - start) as i32;
        }
        if i == sfx {
          sfx = self.suffix_table[sfx] as usize;
        }
      }
    }
  }

  /// 单字符策略 (SingleCharSearch)。
  fn single_char<S: SubjectBytes>(&self, subject: &[S], index: usize, wd: Widths) -> Option<usize> {
    let p = (self.pattern[0]).into();
    if wd.subject_one_byte && p > 0xFF {
      // pattern 字符放不进单字节 subject
      return None;
    }
    let max_n = subject.len() + 1 - self.pattern.len();
    if index >= max_n {
      return None;
    }
    S::find_first(p, subject, index, max_n)
  }

  /// 线性策略 (LinearSearch)。
  fn linear<S: SubjectBytes>(&self, subject: &[S], index: usize, wd: Widths) -> Option<usize> {
    let pattern = self.pattern;
    let pattern_length = pattern.len();
    let n = subject.len() + 1 - pattern_length;
    let mut i = index;
    while i < n {
      let pos = S::find_first((pattern[0]).into(), subject, i, n)?;
      // 首字符已匹配, 比较其余
      if pattern[1..]
        .iter()
        .zip(&subject[pos + 1..])
        .all(|(&p, &s)| (p).into() == (s).into())
      {
        return Some(pos);
      }
      i = pos + 1;
    }
    let _ = wd;
    None
  }

  /// 首轮策略 (InitialSearch): badness 超限升级 BMH。
  fn initial<S: SubjectBytes>(&mut self, subject: &[S], index: usize, wd: Widths) -> Option<usize> {
    let pattern_length = self.pattern.len();
    let mut badness: isize = -10 - (pattern_length << 2) as isize;
    let n = subject.len() + 1 - pattern_length;
    let mut i = index;
    while i < n {
      badness += 1;
      if badness <= 0 {
        let pos = S::find_first((self.pattern[0]).into(), subject, i, n)?;
        i = pos;
        let mut j = 1;
        while j < pattern_length && (self.pattern[j]).into() == (subject[i + j]).into() {
          j += 1;
        }
        if j == pattern_length {
          return Some(i);
        }
        badness += j as isize;
      } else {
        self.populate_horspool(wd);
        self.strategy = Strategy::BoyerMooreHorspool;
        return self.horspool(subject, i, wd);
      }
      i += 1;
    }
    None
  }

  /// BMH 策略 (BoyerMooreHorspoolSearch): badness 超限升级 BM。
  fn horspool<S: Copy + Into<u32>>(
    &mut self,
    subject: &[S],
    start_index: usize,
    wd: Widths,
  ) -> Option<usize> {
    let pattern_length = self.pattern.len();
    let mut badness: isize = -(pattern_length as isize);
    let last_char = (self.pattern[pattern_length - 1]).into();
    let last_char_shift = pattern_length as i32 - 1 - self.occ(last_char, wd);
    let n = subject.len() + 1 - pattern_length;
    let mut index = start_index;
    while index < n {
      let j = pattern_length - 1;
      // 对齐窗口末字符 (j 在此循环内恒为 pattern_length - 1)
      let mut sc = (subject[index + j]).into();
      while last_char != sc {
        let shift = j as isize - self.occ(sc, wd) as isize;
        index = (index as isize + shift) as usize;
        badness += 1 - shift;
        if index >= n {
          return None;
        }
        sc = (subject[index + j]).into();
      }
      // 逆向验证其余字符
      let mut j = pattern_length as isize - 2;
      while j >= 0 && (self.pattern[j as usize]).into() == (subject[index + j as usize]).into() {
        j -= 1;
      }
      if j < 0 {
        return Some(index);
      }
      index = (index as isize + last_char_shift as isize) as usize;
      badness += pattern_length as isize - j - last_char_shift as isize;
      if badness > 0 {
        self.populate_boyer_moore();
        self.strategy = Strategy::BoyerMoore;
        return self.boyer_moore(subject, index, wd);
      }
    }
    None
  }

  /// BM 策略 (BoyerMooreSearch)。
  fn boyer_moore<S: Copy + Into<u32>>(
    &mut self,
    subject: &[S],
    start_index: usize,
    wd: Widths,
  ) -> Option<usize> {
    let pattern_length = self.pattern.len();
    let start = self.start;
    let last_char = (self.pattern[pattern_length - 1]).into();
    let n = subject.len() + 1 - pattern_length;
    let mut index = start_index;
    while index < n {
      let j = pattern_length - 1;
      let mut c = (subject[index + j]).into();
      while last_char != c {
        let shift = j as isize - self.occ(c, wd) as isize;
        index = (index as isize + shift) as usize;
        if index >= n {
          return None;
        }
        c = (subject[index + j]).into();
      }
      let mut j = pattern_length as isize - 1;
      let mut c: u32 = 0;
      while j >= 0 {
        let sc = (subject[index + j as usize]).into();
        c = sc;
        if (self.pattern[j as usize]).into() != sc {
          break;
        }
        j -= 1;
      }
      if j < 0 {
        return Some(index);
      }
      if (j as usize) < start {
        // 匹配超出建表范围: 回退 BMH 位移
        let shift = pattern_length as isize - 1 - self.occ(last_char, wd) as isize;
        index = (index as isize + shift) as usize;
      } else {
        let gs_shift = self.good_suffix_shift[j as usize + 1];
        let bc_shift = j - self.occ(c, wd) as isize;
        let shift = (gs_shift as isize).max(bc_shift);
        index = (index as isize + shift) as usize;
      }
    }
    None
  }

  /// 分派到当前策略 (Search)。
  fn dispatch<S: SubjectBytes>(
    &mut self,
    subject: &[S],
    index: usize,
    wd: Widths,
  ) -> Option<usize> {
    let pattern_length = self.pattern.len();
    if pattern_length == 0
      || subject.len() < pattern_length
      || index > subject.len() - pattern_length
    {
      return None;
    }
    match self.strategy {
      Strategy::Fail => None,
      Strategy::SingleChar => self.single_char(subject, index, wd),
      Strategy::Linear => self.linear(subject, index, wd),
      Strategy::Initial => self.initial(subject, index, wd),
      Strategy::BoyerMooreHorspool => self.horspool(subject, index, wd),
      Strategy::BoyerMoore => self.boyer_moore(subject, index, wd),
    }
  }
}

/// subject 字符定位钩子 (FindFirstCharacter): 在 subject[index..max_n) 中找 pattern 首字符。
/// 单字节走 memchr (原版 memchr 位点), 双字节线性 (原版 memchr+AlignDown 的等价语义;
/// 双字节找 NUL 的 memchr 误报特判由线性实现天然覆盖)。
trait SubjectBytes: Copy + Into<u32> {
  fn find_first(pattern_first: u32, subject: &[Self], index: usize, max_n: usize) -> Option<usize>;
}

impl SubjectBytes for u8 {
  #[inline]
  fn find_first(pattern_first: u32, subject: &[Self], index: usize, max_n: usize) -> Option<usize> {
    // pattern 超 0xFF 时由上层 Fail 策略拦截, 此处防御
    if pattern_first > 0xFF {
      return None;
    }
    memchr::memchr(pattern_first as u8, &subject[index..max_n]).map(|p| p + index)
  }
}

impl SubjectBytes for u16 {
  #[inline]
  fn find_first(pattern_first: u32, subject: &[Self], index: usize, max_n: usize) -> Option<usize> {
    subject[index..max_n]
      .iter()
      .position(|&s| u32::from(s) == pattern_first)
      .map(|p| p + index)
  }
}

/// 一次性搜索: 单字节 subject + 单字节 pattern (SearchString<uint8_t, uint8_t>)。
pub fn search_string_u8(subject: &[u8], pattern: &[u8], start_index: usize) -> Option<usize> {
  if pattern.is_empty() || pattern.len() > subject.len() {
    return None;
  }
  StringSearch::new_u8(pattern).search(subject, start_index)
}

/// 一次性搜索: 单字节 subject + 双字节 pattern。
pub fn search_string_u16_in_u8(
  subject: &[u8],
  pattern: &[u16],
  start_index: usize,
) -> Option<usize> {
  if pattern.is_empty() || pattern.len() > subject.len() || pattern.iter().any(|&c| c > 0xFF) {
    return None;
  }
  StringSearch::new_u16(pattern).search_u8(subject, start_index)
}

/// 一次性搜索: 双字节 subject + 单字节 pattern。
pub fn search_string_u8_in_u16(
  subject: &[u16],
  pattern: &[u8],
  start_index: usize,
) -> Option<usize> {
  if pattern.is_empty() || pattern.len() > subject.len() {
    return None;
  }
  StringSearch::new_u8(pattern).search_u16(subject, start_index)
}

/// 一次性搜索: 双字节 subject + 双字节 pattern。
pub fn search_string_u16(subject: &[u16], pattern: &[u16], start_index: usize) -> Option<usize> {
  if pattern.is_empty() || pattern.len() > subject.len() {
    return None;
  }
  StringSearch::new_u16(pattern).search_u16(subject, start_index)
}
