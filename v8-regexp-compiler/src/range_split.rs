//! v8/src/regexp 的 UnicodeRangeSplitter 与 Latin1 等价判定 1:1 转写
//!
//! - `UnicodeRangeSplitter` 声明于 regexp-compiler.h:755-777, 实现于
//!   regexp-compiler-tonode.cc:210-264 (构造函数 + AddRange)。
//! - `RangeContainsLatin1Equivalents` / `RangesContainLatin1Equivalents`
//!   实现于 regexp-compiler.cc:2290-2305。
//!
//! 纯分类逻辑, 不触 Zone/Parser/Compiler, 属 wave1 可落地面。
//!
//! 原 ZoneList/base::SmallVector 以 Vec 承接 (kInitialSize=8 仅作容量提示)。

use v8_regexp_syntax::char_range::{
  CharacterRange, K_LEAD_SURROGATE_END, K_LEAD_SURROGATE_START, K_MAX_CODE_POINT, K_NON_BMP_START,
  K_TRAIL_SURROGATE_END, K_TRAIL_SURROGATE_START,
};

/// UnicodeRangeSplitter::kInitialSize (regexp-compiler.h:760) — Vec 容量提示。
pub const K_INITIAL_SIZE: usize = 8;

/// 将字符区间按 BMP / 孤立 lead / 孤立 trail / 非 BMP 四桶分类
/// (UnicodeRangeSplitter, regexp-compiler-tonode.cc:223-264)。
#[derive(Debug, Clone, Default)]
pub struct UnicodeRangeSplitter {
  bmp: Vec<CharacterRange>,
  lead_surrogates: Vec<CharacterRange>,
  trail_surrogates: Vec<CharacterRange>,
  non_bmp: Vec<CharacterRange>,
}

impl UnicodeRangeSplitter {
  /// 构造: 逐个 AddRange 基区间表
  /// (regexp-compiler-tonode.cc:210-221)。
  pub fn new(base: &[CharacterRange]) -> Self {
    let mut splitter = Self::default();
    for range in base {
      splitter.add_range(*range);
    }
    splitter
  }

  /// const CharacterRangeVector* bmp() / lead_surrogates() /
  /// trail_surrogates() / non_bmp() 访问器 (regexp-compiler.h:762-770)。
  pub fn bmp(&self) -> &[CharacterRange] {
    &self.bmp
  }
  pub fn lead_surrogates(&self) -> &[CharacterRange] {
    &self.lead_surrogates
  }
  pub fn trail_surrogates(&self) -> &[CharacterRange] {
    &self.trail_surrogates
  }
  pub fn non_bmp(&self) -> &[CharacterRange] {
    &self.non_bmp
  }

  /// void AddRange(CharacterRange range) (regexp-compiler-tonode.cc:223-264)。
  ///
  /// 五段切分: [kBmp1Start,kBmp1End] [lead] [trail] [kBmp2Start,kBmp2End]
  /// [非BMP]; lead/trail 段进 lead/trail 桶, BMP 两段合并进 bmp 桶。
  /// 原版静态区间端点含全部 assert 以 const 断言承接。
  pub fn add_range(&mut self, range: CharacterRange) {
    // Ends are all inclusive. (regexp-compiler-tonode.cc:224-239)
    const BMP1_START: u32 = 0;
    const BMP1_END: u32 = K_LEAD_SURROGATE_START - 1;
    const BMP2_START: u32 = K_TRAIL_SURROGATE_END + 1;
    const BMP2_END: u32 = K_NON_BMP_START - 1;
    const _: () = assert!(BMP1_START == 0);
    const _: () = assert!(BMP1_START < BMP1_END);
    const _: () = assert!(BMP1_END + 1 == K_LEAD_SURROGATE_START);
    const _: () = assert!(K_LEAD_SURROGATE_START < K_LEAD_SURROGATE_END);
    const _: () = assert!(K_LEAD_SURROGATE_END + 1 == K_TRAIL_SURROGATE_START);
    const _: () = assert!(K_TRAIL_SURROGATE_START < K_TRAIL_SURROGATE_END);
    const _: () = assert!(K_TRAIL_SURROGATE_END + 1 == BMP2_START);
    const _: () = assert!(BMP2_START < BMP2_END);
    const _: () = assert!(BMP2_END + 1 == K_NON_BMP_START);
    const _: () = assert!(K_NON_BMP_START < K_MAX_CODE_POINT);

    const STARTS: [u32; 5] = [
      BMP1_START,
      K_LEAD_SURROGATE_START,
      K_TRAIL_SURROGATE_START,
      BMP2_START,
      K_NON_BMP_START,
    ];
    const ENDS: [u32; 5] = [
      BMP1_END,
      K_LEAD_SURROGATE_END,
      K_TRAIL_SURROGATE_END,
      BMP2_END,
      K_MAX_CODE_POINT,
    ];
    // kTargets: 0/3 段同入 bmp (原版指针数组)。
    const TARGETS: [u8; 5] = [0, 1, 2, 0, 3];

    for i in 0..STARTS.len() {
      if STARTS[i] > range.to {
        break;
      }
      let from = STARTS[i].max(range.from);
      let to = ENDS[i].min(range.to);
      if from > to {
        continue;
      }
      let piece = CharacterRange::range(from, to);
      match TARGETS[i] {
        0 => self.bmp.push(piece),
        1 => self.lead_surrogates.push(piece),
        2 => self.trail_surrogates.push(piece),
        _ => self.non_bmp.push(piece),
      }
    }
  }
}

/// bool RangeContainsLatin1Equivalents(CharacterRange range)
/// (regexp-compiler.cc:2289-2294)。
///
/// 注释原文: 需要检查 0x39C 0x3BC 0x178 三个字符
/// (µ 的希腊小写/页面标记微符号与 Ÿ 的 Latin1/Yen 等价纠缠)。
pub fn range_contains_latin1_equivalents(range: CharacterRange) -> bool {
  // TODO(dcarney): this could be a lot more efficient. (原版注释保留)
  range.contains(0x039C) || range.contains(0x03BC) || range.contains(0x0178)
}

/// bool RangesContainLatin1Equivalents(ZoneList<CharacterRange>*)
/// (regexp-compiler.cc:2297-2305, 匿名命名空间)。
pub fn ranges_contain_latin1_equivalents(ranges: &[CharacterRange]) -> bool {
  for r in ranges {
    // TODO(dcarney): this could be a lot more efficient.
    if range_contains_latin1_equivalents(*r) {
      return true;
    }
  }
  false
}
