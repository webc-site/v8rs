// 由 tool/src/regexp/regexp-compiler.h.js 从 v8/src/regexp/regexp-compiler.h 生成; 修订就地改, 复跑覆盖
// compiler_constants 标准类区间表; 消费方 char_range.rs

/// 原版 kRangeEndMarker (终结标记, 展开时已剥除)
pub const RANGE_END_MARKER: u32 = 1114112;

/// kSpaceRanges: [from, to] 闭区间对 (原版 [from, to+1) + 终结标记展开)
pub static SPACE_RANGES: [(u32, u32); 10] = [
  (9, 13),
  (32, 32),
  (160, 160),
  (5760, 5760),
  (8192, 8202),
  (8232, 8233),
  (8239, 8239),
  (8287, 8287),
  (12288, 12288),
  (65279, 65279),
];

/// kWordRanges: [from, to] 闭区间对 (原版 [from, to+1) + 终结标记展开)
pub static WORD_RANGES: [(u32, u32); 4] = [(48, 57), (65, 90), (95, 95), (97, 122)];

/// kDigitRanges: [from, to] 闭区间对 (原版 [from, to+1) + 终结标记展开)
pub static DIGIT_RANGES: [(u32, u32); 1] = [(48, 57)];

/// kLineTerminatorRanges: [from, to] 闭区间对 (原版 [from, to+1) + 终结标记展开)
pub static LINE_TERMINATOR_RANGES: [(u32, u32); 3] = [(10, 10), (13, 13), (8232, 8233)];

/// kSurrogateRanges: [from, to] 闭区间对 (原版 [from, to+1) + 终结标记展开)
pub static SURROGATE_RANGES: [(u32, u32); 1] = [(55296, 55296)];

// ── 对数断言: 表项数 == 原版数组长度 - 1 (终结标记) 且为偶数 ──
const _: () = assert!(SPACE_RANGES.len() * 2 + 1 == 21);
const _: () = assert!(WORD_RANGES.len() * 2 + 1 == 9);
const _: () = assert!(DIGIT_RANGES.len() * 2 + 1 == 3);
const _: () = assert!(LINE_TERMINATOR_RANGES.len() * 2 + 1 == 7);
const _: () = assert!(SURROGATE_RANGES.len() * 2 + 1 == 3);
