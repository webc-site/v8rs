// 由 tool/src/strings/char-predicates.cc.js 从 v8/src/strings/char-predicates.cc 生成; 修订就地改, 复跑覆盖
// INTL 慢路径字符谓词骨架与实现

/// 慢路径字符谓词 (原版 char-predicates.cc, 仅 V8_INTL_SUPPORT 构建; x64/arm64 默认开启)。
/// 快路径 (c ≤ 0xFF) 查表见 char_predicates.rs; 慢路径依赖 ICU 二元属性。
pub mod char_predicates_slow_skel {
  use crate::char_predicates;

  /// UnicodeIDStart 或 '$' '_' '\\' (c < 0x60 时; ECMA-262 Names and Keywords)
  #[inline]
  #[must_use]
  pub fn is_identifier_start_slow(c: u32) -> bool {
    char_predicates::is_identifier_start_slow(c)
  }

  /// UnicodeIDContinue 或 '$' '_' '\\' 或 ZWJ(U+200C 前体) 实为
  /// ZWNJ=U+200C, ZWJ=U+200D (c < 0x60 快旁路 + 两个连接符)
  #[inline]
  #[must_use]
  pub fn is_identifier_part_slow(c: u32) -> bool {
    char_predicates::is_identifier_part_slow(c)
  }

  /// White Space: gC=Zs 或 U+0009/000B/000C 或 U+FEFF
  #[inline]
  #[must_use]
  pub fn is_white_space_slow(c: u32) -> bool {
    char_predicates::is_white_space_slow(c)
  }

  /// 慢路径谓词数对数守卫
  pub const SLOW_PREDICATES: [&str; 3] = [
    "IsIdentifierStartSlow",
    "IsIdentifierPartSlow",
    "IsWhiteSpaceSlow",
  ];

  /// ICU 依赖登记
  pub const ICU_DEPS: [&str; 3] = ["UCHAR_ID_START", "UCHAR_ID_CONTINUE", "U_SPACE_SEPARATOR"];
}
