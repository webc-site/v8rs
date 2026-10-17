//! v8/src/regexp/regexp-compiler.h 的 flags 派生与编译常量 1:1 转写
//!
//! - `compiler_constants` 命名空间 (regexp-compiler.h:38-68): Boyer-Moore
//!   前瞻/最短模式常量; kLineTerminatorRanges 区间表在冻结的
//!   v8-regexp-syntax::tables_generated::LINE_TERMINATOR_RANGES 同源维护
//!   (登记: 本 crate 不重复落表, 测试侧对数)。
//! - `NeedsUnicodeCaseEquivalents(Flags)` (regexp-compiler.h:70-74)。

use v8_regexp_syntax::flags::RegExpFlags;

/// compiler_constants::kMaxLookaheadForBoyerMoore (regexp-compiler.h:63)。
/// "More makes code generation slower, less makes V8 benchmark score lower."
pub const K_MAX_LOOKAHEAD_FOR_BOYER_MOORE: u32 = 8;

/// compiler_constants::kPatternTooShortForBoyerMoore (regexp-compiler.h:66)。
/// 3 字符模式最多一次跳 3, 不足以偿付额外逻辑 → length ≤ 2 直接放弃。
pub const K_PATTERN_TOO_SHORT_FOR_BOYER_MOORE: u32 = 2;

/// inline bool NeedsUnicodeCaseEquivalents(Flags flags) (regexp-compiler.h:70-74)。
///
/// unicode (或 unicode sets) 与 ignore_case 同时置位时, 需要大小写等价闭包
/// (原版走 ICU; 本仓 INTL 关闭时闭包为恒等, 判定式不变)。
#[must_use]
pub const fn needs_unicode_case_equivalents(flags: RegExpFlags) -> bool {
  // Both unicode (or unicode sets) and ignore_case flags are set.
  flags.either_unicode() && flags.ignore_case()
}
