// 由 tool/src/regexp/regexp-flags.h.js 从 v8/src/regexp/regexp-flags.h 生成; 修订就地改, 复跑覆盖
// REGEXP_FLAG_LIST X-macro 展开; 语义方法见 flags.rs

// ── 对数断言: 表实数 == 宏实数 (9) ──
pub const NUM_FLAGS: usize = 9;

// bit 连续性: 0..8 (原版 static_assert "contiguous bits")
const _: () = assert!(
  (1 << NUM_FLAGS) - 1 == {
    1 << 7 | 1 << 0 | 1 << 1 | 1 << 6 | 1 << 2 | 1 << 5 | 1 << 4 | 1 << 8 | 1 << 3
  }
);

// 字符集 (原版 operator<< 顺序 = 宏序, 已按字符升序)
pub const FLAG_CHARS: [char; NUM_FLAGS] = ['d', 'g', 'i', 'l', 'm', 's', 'u', 'v', 'y'];

// ── bit 常量 (原版 enum class Flag, 位序 1:1) ──
/// 'd'
pub const HASINDICES: u16 = 1 << 7;
/// 'g'
pub const GLOBAL: u16 = 1 << 0;
/// 'i'
pub const IGNORECASE: u16 = 1 << 1;
/// 'l'
pub const LINEAR: u16 = 1 << 6;
/// 'm'
pub const MULTILINE: u16 = 1 << 2;
/// 's'
pub const DOTALL: u16 = 1 << 5;
/// 'u'
pub const UNICODE: u16 = 1 << 4;
/// 'v'
pub const UNICODESETS: u16 = 1 << 8;
/// 'y'
pub const STICKY: u16 = 1 << 3;

/// flag 名元组 (lower 形态, 宏序)
pub const FLAG_NAMES: [&str; NUM_FLAGS] = [
  "has_indices",
  "global",
  "ignore_case",
  "linear",
  "multiline",
  "dot_all",
  "unicode",
  "unicode_sets",
  "sticky",
];

/// (flag 字符, 位) 配对表 (宏序; 供 operator<< 与 TryFlagFromChar)
pub const FLAG_SPECS: [(char, u16); NUM_FLAGS] = [
  ('d', HASINDICES),
  ('g', GLOBAL),
  ('i', IGNORECASE),
  ('l', LINEAR),
  ('m', MULTILINE),
  ('s', DOTALL),
  ('u', UNICODE),
  ('v', UNICODESETS),
  ('y', STICKY),
];
