//! 字符区间与集合代数
//!
//! 原版对应:
//! - `regexp-ast.h` `CharacterRange` 声明
//! - `regexp-compiler-tonode.cc` `CharacterRange::{AddClassEscape, AddUnicodeCaseEquivalents,
//!   IsCanonical, Canonicalize, Negate, Intersect, Intersects, Subtract, ClampToOneByte, Equals}`
//! - `regexp-compiler.h` `compiler_constants` 区间表 (tables_generated.rs)

use crate::{
  ast_generated::StandardCharacterSet,
  tables_generated::{DIGIT_RANGES, LINE_TERMINATOR_RANGES, SPACE_RANGES, WORD_RANGES},
};

/// 最大码点 (原版 kMaxCodePoint)
pub const K_MAX_CODE_POINT: u32 = 0x10FFFF;
/// 非 BMP 起始 (原版 kNonBmpStart)
pub const K_NON_BMP_START: u32 = 0x10000;
/// 前导代理起始 (原版 kLeadSurrogateStart)
pub const K_LEAD_SURROGATE_START: u32 = 0xD800;
/// 前导代理结束 (原版 kLeadSurrogateEnd)
pub const K_LEAD_SURROGATE_END: u32 = 0xDBFF;
/// 后随代理起始 (原版 kTrailSurrogateStart)
pub const K_TRAIL_SURROGATE_START: u32 = 0xDC00;
/// 后随代理结束 (原版 kTrailSurrogateEnd)
pub const K_TRAIL_SURROGATE_END: u32 = 0xDFFF;

/// 闭区间 [from, to] (两端含, 码点至多 0x10FFFF)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterRange {
  pub from: u32,
  pub to: u32,
}

impl CharacterRange {
  /// 单字符区间 (原版 Singleton)
  #[inline]
  #[must_use]
  pub const fn singleton(value: u32) -> Self {
    debug_assert!(value <= K_MAX_CODE_POINT);
    Self {
      from: value,
      to: value,
    }
  }

  /// [from, to] (原版 Range, 前置 from <= to 且 to <= K_MAX_CODE_POINT)
  #[inline]
  #[must_use]
  pub const fn range(from: u32, to: u32) -> Self {
    debug_assert!(from <= to && to <= K_MAX_CODE_POINT);
    Self { from, to }
  }

  /// 安全校验构造 (原版 Range 校验安全封装)
  #[inline]
  #[must_use]
  pub const fn try_range(from: u32, to: u32) -> Option<Self> {
    if from <= to && to <= K_MAX_CODE_POINT {
      Some(Self { from, to })
    } else {
      None
    }
  }

  /// 全集 [0, kMaxCodePoint] (原版 Everything)
  #[inline]
  #[must_use]
  pub const fn everything() -> Self {
    Self {
      from: 0,
      to: K_MAX_CODE_POINT,
    }
  }

  /// 是否包含码点 i
  #[inline]
  #[must_use]
  pub const fn contains(&self, i: u32) -> bool {
    self.from <= i && i <= self.to
  }

  /// 是否覆盖 [0, max] (原版 IsEverything)
  #[inline]
  #[must_use]
  pub const fn is_everything(&self, max: u32) -> bool {
    self.from == 0 && self.to >= max
  }

  /// 是否单字符
  #[inline]
  #[must_use]
  pub const fn is_singleton(&self) -> bool {
    self.from == self.to
  }

  /// 追加标准字符类区间 (原版 AddClassEscape)
  ///
  /// `add_unicode_case_equivalents` 仅对 /ui /vi 的 \w \W 生效: 先做大小写闭包再取反;
  /// 无 INTL 配置下闭包为恒等 (AddUnicodeCaseEquivalents 空实现), 行为与原版 no-INTL 一致。
  pub fn add_class_escape(
    standard_character_set: StandardCharacterSet,
    ranges: &mut Vec<Self>,
    add_unicode_case_equivalents: bool,
  ) {
    if add_unicode_case_equivalents
      && matches!(
        standard_character_set,
        StandardCharacterSet::Word | StandardCharacterSet::NotWord
      )
    {
      // https://tc39.es/ecma262/#sec-runtime-semantics-wordcharacters-abstract-operation
      let mut new_ranges: Vec<Self> = Vec::with_capacity(WORD_RANGES.len());
      add_class(&WORD_RANGES, &mut new_ranges);
      Self::add_unicode_case_equivalents(&mut new_ranges);
      if standard_character_set == StandardCharacterSet::NotWord {
        let mut negated = Vec::with_capacity(new_ranges.len() + 1);
        Self::negate(&new_ranges, &mut negated);
        new_ranges = negated;
      }
      ranges.extend_from_slice(&new_ranges);
      return;
    }

    match standard_character_set {
      StandardCharacterSet::Whitespace => add_class(&SPACE_RANGES, ranges),
      StandardCharacterSet::NotWhitespace => add_class_negated(&SPACE_RANGES, ranges),
      StandardCharacterSet::Word => add_class(&WORD_RANGES, ranges),
      StandardCharacterSet::NotWord => add_class_negated(&WORD_RANGES, ranges),
      StandardCharacterSet::Digit => add_class(&DIGIT_RANGES, ranges),
      StandardCharacterSet::NotDigit => add_class_negated(&DIGIT_RANGES, ranges),
      // multiline 下 ^ 与 $ 匹配的字符集
      StandardCharacterSet::LineTerminator => add_class(&LINE_TERMINATOR_RANGES, ranges),
      StandardCharacterSet::NotLineTerminator => add_class_negated(&LINE_TERMINATOR_RANGES, ranges),
      // 匹配任意字符的简写 (非规范字符区间)
      StandardCharacterSet::Everything => ranges.push(Self::everything()),
    }
  }

  /// /ui /vi 大小写等价闭包 (原版 AddUnicodeCaseEquivalents; 无 INTL 时空实现)
  pub fn add_unicode_case_equivalents(_ranges: &mut Vec<Self>) {
    // 原版 #ifdef V8_INTL_SUPPORT 体内走 ICU closeOver; no-INTL 构建为空。
  }

  /// 区间表是否规范 (按 from 升序, 不重叠不邻接)
  #[must_use]
  pub fn is_canonical(ranges: &[Self]) -> bool {
    let n = ranges.len();
    if n <= 1 {
      return true;
    }
    let mut max = ranges[0].to;
    for r in &ranges[1..] {
      if r.from <= max + 1 {
        return false;
      }
      max = r.to;
    }
    true
  }

  /// 就地规范化: 排序 + 合并重叠/邻接区间 (原版 Canonicalize)
  pub fn canonicalize(ranges: &mut Vec<Self>) {
    let n = ranges.len();
    if n <= 1 {
      return;
    }
    // 已规范前缀检测
    let mut max = ranges[0].to;
    let mut i = 1;
    while i < n {
      if ranges[i].from <= max + 1 {
        break;
      }
      max = ranges[i].to;
      i += 1;
    }
    if i == n {
      return;
    }

    // 插入排序等价: 逐个把尾部区间插入已规范前缀
    let mut read = i;
    let mut num_canonical = i;
    while read < n {
      let insert = ranges[read];
      num_canonical = insert_range_in_canonical_list(ranges, num_canonical, insert);
      read += 1;
    }
    ranges.truncate(num_canonical);
    debug_assert!(Self::is_canonical(ranges));
  }

  /// 指定最大上界的补集运算 (输入须规范; 原版 Negate 泛化)
  pub fn negate_with_max(ranges: &[Self], negated: &mut Vec<Self>, max: u32) {
    debug_assert!(Self::is_canonical(ranges) && negated.is_empty());
    let range_count = ranges.len();
    negated.reserve(range_count + 1);
    let mut from = 0_u32;
    let mut i = 0;
    if range_count > 0 && ranges[0].from == 0 {
      from = ranges[0].to + 1;
      i = 1;
    }
    while i < range_count {
      let r = ranges[i];
      if r.from > max {
        break;
      }
      negated.push(Self::range(from, r.from - 1));
      from = r.to.saturating_add(1);
      if from > max {
        break;
      }
      i += 1;
    }
    if from <= max {
      negated.push(Self::range(from, max));
    }
  }

  /// Unicode 全集取补 ([0, 0x10FFFF]) (原版 Negate)
  #[inline]
  pub fn negate(ranges: &[Self], negated: &mut Vec<Self>) {
    Self::negate_with_max(ranges, negated, K_MAX_CODE_POINT);
  }

  /// BMP 空间取补 ([0, 0xFFFF])
  #[inline]
  pub fn negate_bmp(ranges: &[Self], negated: &mut Vec<Self>) {
    Self::negate_with_max(ranges, negated, 0xFFFF);
  }

  /// 两组规范区间求交集 (原版 Intersect)
  pub fn intersect(lhs: &[Self], rhs: &[Self], intersection: &mut Vec<Self>) {
    debug_assert!(Self::is_canonical(lhs));
    debug_assert!(Self::is_canonical(rhs));
    debug_assert!(intersection.is_empty());
    intersection.reserve(lhs.len().min(rhs.len()));

    let mut lhs_index = 0;
    let mut rhs_index = 0;
    while lhs_index < lhs.len() && rhs_index < rhs.len() {
      if lhs[lhs_index].to < rhs[rhs_index].from {
        lhs_index += 1;
        continue;
      }
      if rhs[rhs_index].to < lhs[lhs_index].from {
        rhs_index += 1;
        continue;
      }
      let from = lhs[lhs_index].from.max(rhs[rhs_index].from);
      let to = lhs[lhs_index].to.min(rhs[rhs_index].to);
      intersection.push(Self::range(from, to));
      if to == lhs[lhs_index].to {
        lhs_index += 1;
      }
      if to == rhs[rhs_index].to {
        rhs_index += 1;
      }
    }
    debug_assert!(Self::is_canonical(intersection));
  }

  /// 两组规范区间是否相交 (原版 Intersects)
  #[must_use]
  pub fn intersects(lhs: &[Self], rhs: &[Self]) -> bool {
    debug_assert!(Self::is_canonical(lhs));
    debug_assert!(Self::is_canonical(rhs));

    let mut lhs_index = 0;
    let mut rhs_index = 0;
    while lhs_index < lhs.len() && rhs_index < rhs.len() {
      let l = lhs[lhs_index];
      let r = rhs[rhs_index];
      if l.to < r.from {
        lhs_index += 1;
      } else if r.to < l.from {
        rhs_index += 1;
      } else {
        return true;
      }
    }
    false
  }

  /// 差集运算: src - to_remove (输入须规范; 原版 Subtract)
  pub fn subtract(src: &[Self], to_remove: &[Self], result: &mut Vec<Self>) {
    debug_assert!(Self::is_canonical(src));
    debug_assert!(Self::is_canonical(to_remove));
    debug_assert!(result.is_empty());

    if src.is_empty() {
      return;
    }
    result.reserve(src.len());

    let mut src_index = 0;
    let mut to_remove_index = 0;
    let mut from = src[0].from;
    let mut to = src[0].to;

    while src_index < src.len() && to_remove_index < to_remove.len() {
      let remove_range = to_remove[to_remove_index];
      if remove_range.to < from {
        // (a) 不重叠, 移除区间在左侧
        to_remove_index += 1;
      } else if to < remove_range.from {
        // (b) 不重叠, 移除区间在右侧, 当前 src 区间完全保留
        result.push(Self::range(from, to));
        safe_advance_range(src, &mut src_index, &mut from, &mut to);
      } else if from >= remove_range.from && to <= remove_range.to {
        // (c) 移除区间完全覆盖当前 src 区间
        safe_advance_range(src, &mut src_index, &mut from, &mut to);
      } else if from < remove_range.from && to > remove_range.to {
        // (d) 移除区间切分当前 src 区间
        result.push(Self::range(from, remove_range.from - 1));
        from = remove_range.to + 1;
        to_remove_index += 1;
      } else if from < remove_range.from {
        // (e) 截断当前区间尾部
        to = remove_range.from - 1;
        result.push(Self::range(from, to));
        safe_advance_range(src, &mut src_index, &mut from, &mut to);
      } else if to > remove_range.to {
        // (f) 截断当前区间头部
        from = remove_range.to + 1;
        to_remove_index += 1;
      }
    }

    if from <= to && from <= K_MAX_CODE_POINT {
      result.push(Self::range(from, to));
    }
    src_index += 1;

    if src_index < src.len() {
      result.extend_from_slice(&src[src_index..]);
    }

    debug_assert!(Self::is_canonical(result));
  }

  /// 截断到单字节范围 (<= 0xFF) (原版 ClampToOneByte)
  pub fn clamp_to_one_byte(ranges: &mut Vec<Self>) {
    debug_assert!(Self::is_canonical(ranges));
    const MAX_CHAR: u32 = 0xFF;
    let mut n = ranges.len();
    while n > 0 {
      let r = &mut ranges[n - 1];
      if r.from <= MAX_CHAR {
        r.to = r.to.min(MAX_CHAR);
        break;
      }
      n -= 1;
    }
    ranges.truncate(n);
  }

  /// 两组规范区间是否完全一致 (原版 Equals)
  #[must_use]
  pub fn equals(lhs: &[Self], rhs: &[Self]) -> bool {
    debug_assert!(Self::is_canonical(lhs));
    debug_assert!(Self::is_canonical(rhs));
    lhs == rhs
  }
}

/// 推进当前区间指针 (原版 SafeAdvanceRange)
fn safe_advance_range(range: &[CharacterRange], index: &mut usize, from: &mut u32, to: &mut u32) {
  *index += 1;
  if *index < range.len() {
    *from = range[*index].from;
    *to = range[*index].to;
  } else {
    *from = K_MAX_CODE_POINT + 2;
    *to = K_MAX_CODE_POINT + 1;
  }
}

/// 追加闭区间表 (原版 AddClass)
fn add_class(table: &[(u32, u32)], ranges: &mut Vec<CharacterRange>) {
  ranges.extend(
    table
      .iter()
      .map(|&(from, to)| CharacterRange::range(from, to)),
  );
}

/// 追加闭区间表的补集 (原版 AddClassNegated)
fn add_class_negated(table: &[(u32, u32)], ranges: &mut Vec<CharacterRange>) {
  let mut last = 0_u32;
  for &(from, to) in table {
    debug_assert!(last <= from && from <= to);
    ranges.push(CharacterRange::range(last, from - 1));
    last = to + 1;
  }
  ranges.push(CharacterRange::range(last, K_MAX_CODE_POINT));
}

/// 把区间插入已规范前缀 list[0..count], 返回新长度 (原版 InsertRangeInCanonicalList)
fn insert_range_in_canonical_list(
  list: &mut [CharacterRange],
  count: usize,
  insert: CharacterRange,
) -> usize {
  let from = insert.from;
  let to = insert.to;
  let mut start_pos = 0;
  let mut end_pos = count;
  for i in (0..count).rev() {
    let current = list[i];
    if current.from > to + 1 {
      end_pos = i;
    } else if current.to + 1 < from {
      start_pos = i + 1;
      break;
    }
  }

  if start_pos == end_pos {
    // 插入空隙
    if start_pos < count {
      list.copy_within(start_pos..count, start_pos + 1);
    }
    list[start_pos] = insert;
    return count + 1;
  }
  if start_pos + 1 == end_pos {
    // 吞并单个既有区间
    let to_replace = list[start_pos];
    let new_from = to_replace.from.min(from);
    let new_to = to_replace.to.max(to);
    list[start_pos] = CharacterRange::range(new_from, new_to);
    return count;
  }
  // 吞并 [start_pos, end_pos) 内全部区间
  let new_from = list[start_pos].from.min(from);
  let new_to = list[end_pos - 1].to.max(to);
  if end_pos < count {
    list.copy_within(end_pos..count, start_pos + 1);
  }
  list[start_pos] = CharacterRange::range(new_from, new_to);
  count - (end_pos - start_pos) + 1
}
