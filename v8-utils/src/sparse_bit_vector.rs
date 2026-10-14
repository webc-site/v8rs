// 由 tool/src/utils/consts.js 从 v8/src/utils/sparse-bit-vector.h 生成; 修订就地改, 复跑覆盖

//! 稀疏位向量 (小规模优化), 移植自 v8/src/utils/sparse-bit-vector。
//!
//! 上游 Segment 由 Zone 分配; Rust 版链表节点走 Box, Zone 参数从 API 中移除。

/// 每 Segment 字数 (kNumWordsPerSegment)
pub const NUM_WORDS_PER_SEGMENT: usize = 6;
/// 单字位数 (kBitsPerWord = kBitsPerByte * kSystemPointerSize)
pub const BITS_PER_WORD: u32 = usize::BITS;
/// 每 Segment 位数 (kNumBitsPerSegment)
pub const NUM_BITS_PER_SEGMENT: u32 = BITS_PER_WORD * NUM_WORDS_PER_SEGMENT as u32;
/// 首 Segment 覆盖位数上限 (64 位平台 384)。
const SEGMENT_BITS: usize = NUM_BITS_PER_SEGMENT as usize;
/// 单字位数 (usize 换算)。
const WORD_BITS: usize = BITS_PER_WORD as usize;

/// Segment (sparse-bit-vector.h:42): offset 起的连续 6 字位窗口 + 后继链。
#[derive(Debug, Default)]
struct Segment {
  /// 首位偏移 (恒为 NUM_BITS_PER_SEGMENT 的整倍数)。
  offset: usize,
  /// 覆盖 [offset, offset + NUM_BITS_PER_SEGMENT)。
  words: [usize; NUM_WORDS_PER_SEGMENT],
  /// 后继段 (offset 严格递增)。
  next: Option<Box<Segment>>,
}

impl Segment {
  fn empty(&self) -> bool {
    self.words.iter().all(|&w| w == 0)
  }
}

/// `SparseBitVector` (sparse-bit-vector.h:28): 单链 Segment 承载的稀疏位集;
/// 首 Segment 常驻 offset 0, 绝大多数实例单段即容。
#[derive(Debug, Default)]
pub struct SparseBitVector {
  first_segment: Segment,
}

/// 升序迭代器 (sparse-bit-vector.h:60 Iterator)。
pub struct Iter<'a> {
  segment: Option<&'a Segment>,
  /// 段内位序 (word * BITS_PER_WORD + bit_in_word)。
  bit_in_segment: usize,
}

impl SparseBitVector {
  /// `SparseBitVector(zone)` (sparse-bit-vector.h:103)。
  #[must_use]
  pub const fn new() -> Self {
    Self {
      first_segment: Segment {
        offset: 0,
        words: [0; NUM_WORDS_PER_SEGMENT],
        next: None,
      },
    }
  }

  /// 定位覆盖 i 的段: 沿链找首个 offset > i-kNumBits 的段 (do-while 语义, :110)。
  fn find_segment(&self, i: usize) -> Option<&Segment> {
    if i < SEGMENT_BITS {
      return Some(&self.first_segment);
    }
    let floor = i - SEGMENT_BITS;
    let mut segment = self.first_segment.next.as_deref()?;
    while segment.offset <= floor {
      segment = segment.next.as_deref()?;
    }
    if segment.offset > i {
      None
    } else {
      Some(segment)
    }
  }

  /// `Contains` (sparse-bit-vector.h:107)。
  #[must_use]
  pub fn contains(&self, i: usize) -> bool {
    let Some(segment) = self.find_segment(i) else {
      return false;
    };
    let bit = i - segment.offset;
    segment.words[bit / WORD_BITS] >> (bit % WORD_BITS) & 1 != 0
  }

  /// `Add` (sparse-bit-vector.h:122): i < kNumBitsPerSegment 走首段快路径 (:127)。
  ///
  /// 高位走两遍: 只读定位插入点 (前驱段序 + 候选段 offset), 再单步可变插入 —
  /// 与原版 do-while 的 (last, segment) 前驱推进语义一致。
  pub fn add(&mut self, i: usize) {
    if i < SEGMENT_BITS {
      self.first_segment.words[i / WORD_BITS] |= 1 << (i % WORD_BITS);
      return;
    }
    let (prev, cand) = self.locate_insert(i);
    match cand {
      // 候选段覆盖 i: 直接置位
      Some((idx, noff)) if noff <= i => {
        set_bit(Self::segment_at_mut(&mut self.first_segment, idx), i)
      }
      // 候选段越过后 (noff > i) 或链尾: 插在前驱之后
      _ => insert_bit_after(Self::segment_at_mut(&mut self.first_segment, prev), i),
    }
  }

  /// 插入定位: 返回 (前驱段序, 候选段 (序, offset)); 无候选为链尾。
  /// 候选 = 首个 offset > i-kNumBitsPerSegment 的段 (原版 do-while 退出段)。
  fn locate_insert(&self, i: usize) -> (usize, Option<(usize, usize)>) {
    let floor = i - SEGMENT_BITS;
    let mut prev = 0;
    let mut idx = 1;
    let mut cur = self.first_segment.next.as_deref();
    while let Some(seg) = cur {
      if seg.offset > floor {
        return (prev, Some((idx, seg.offset)));
      }
      prev = idx;
      cur = seg.next.as_deref();
      idx += 1;
    }
    (prev, None)
  }

  /// 链上第 pos 段 (0 = 传入段); 只读遍历保证不越界, None 截断为纯防御。
  fn segment_at_mut(mut seg: &mut Segment, mut pos: usize) -> &mut Segment {
    while pos > 0 && seg.next.is_some() {
      seg = seg.next.as_deref_mut().unwrap();
      pos -= 1;
    }
    seg
  }
  /// `Remove` (sparse-bit-vector.h:142)。
  pub fn remove(&mut self, i: usize) {
    let Some(segment) = self.find_segment_mut(i) else {
      return;
    };
    let bit = i - segment.offset;
    segment.words[bit / WORD_BITS] &= !(1 << (bit % WORD_BITS));
  }

  fn find_segment_mut(&mut self, i: usize) -> Option<&mut Segment> {
    if i < SEGMENT_BITS {
      return Some(&mut self.first_segment);
    }
    let floor = i - SEGMENT_BITS;
    let mut segment = self.first_segment.next.as_deref_mut()?;
    while segment.offset <= floor {
      segment = segment.next.as_deref_mut()?;
    }
    if segment.offset > i {
      None
    } else {
      Some(segment)
    }
  }

  /// `Union` (sparse-bit-vector.h:158): 逐段合并; 空段跳过; 新段按序插入。
  ///
  /// 首段 (offset 恒 0) 直接字或; 其余非空段按 offset 有序插入 —— 零临时堆分配。
  pub fn union(&mut self, other: &Self) {
    for (dst, src) in self
      .first_segment
      .words
      .iter_mut()
      .zip(other.first_segment.words.iter())
    {
      *dst |= src;
    }
    let mut cur = other.first_segment.next.as_deref();
    while let Some(os) = cur {
      if !os.empty() {
        self.insert_segment_sorted(os.offset, os.words);
      }
      cur = os.next.as_deref();
    }
  }

  /// 按 offset 有序插入/合并一段 (Union 的单段步)。
  fn insert_segment_sorted(&mut self, offset: usize, words: [usize; NUM_WORDS_PER_SEGMENT]) {
    // 只读定位: 首个 offset >= 目标的后继段
    let mut prev = 0;
    let mut idx = 1;
    let mut cur = self.first_segment.next.as_deref();
    let mut after = usize::MAX;
    while let Some(seg) = cur {
      if seg.offset >= offset {
        after = idx;
        break;
      }
      prev = idx;
      cur = seg.next.as_deref();
      idx += 1;
    }
    if after != usize::MAX {
      let seg = Self::segment_at_mut(&mut self.first_segment, after);
      if seg.offset == offset {
        for (dst, src) in seg.words.iter_mut().zip(words.iter()) {
          *dst |= src;
        }
        return;
      }
    }
    // 无同位段: 插在前驱之后
    let p = Self::segment_at_mut(&mut self.first_segment, prev);
    let mut seg = Box::<Segment>::default();
    seg.offset = offset;
    seg.words = words;
    seg.next = p.next.take();
    p.next = Some(seg);
  }

  /// `begin/end` (sparse-bit-vector.h:225)。
  pub fn iter(&self) -> Iter<'_> {
    Iter {
      segment: Some(&self.first_segment),
      bit_in_segment: 0,
    }
  }
}

/// `InsertBitAfter` (sparse-bit-vector.h:270): 新段 (offset 对齐) 接在 slot 后。
fn insert_bit_after(slot: &mut Segment, i: usize) {
  let mut new_segment = Box::<Segment>::default();
  new_segment.offset = i / SEGMENT_BITS * SEGMENT_BITS;
  set_bit(&mut new_segment, i);
  new_segment.next = slot.next.take();
  slot.next = Some(new_segment);
}

/// `set` (sparse-bit-vector.h:286): 段内置位。
fn set_bit(segment: &mut Segment, i: usize) {
  let bit = i - segment.offset;
  segment.words[bit / WORD_BITS] |= 1 << (bit % WORD_BITS);
}

impl Iterator for Iter<'_> {
  type Item = usize;

  /// 段内逐字取最低置位, 段尾换后继段 (operator++ 语义)。
  fn next(&mut self) -> Option<usize> {
    let mut segment = self.segment?;
    loop {
      let word_idx = self.bit_in_segment / WORD_BITS;
      let rest = segment.words[word_idx] >> (self.bit_in_segment % WORD_BITS);
      if rest != 0 {
        let bit = self.bit_in_segment + rest.trailing_zeros() as usize;
        self.bit_in_segment = bit + 1;
        return Some(segment.offset + bit);
      }
      self.bit_in_segment = (word_idx + 1) * WORD_BITS;
      if self.bit_in_segment >= SEGMENT_BITS {
        segment = segment.next.as_deref()?;
        self.segment = Some(segment);
        self.bit_in_segment = 0;
      }
    }
  }
}
