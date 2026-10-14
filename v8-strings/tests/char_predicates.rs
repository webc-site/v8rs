// 迁移自 v8/test/unittests/strings/char-predicates-unittest.cc (V8_INTL_SUPPORT 路径)
// 测试名保留原名: WhiteSpace / WhiteSpaceOrLineTerminator / IdentifierStart / IdentifierPart / SupplementaryPlaneIdentifiers
// 规约: 每条 EXPECT 断言 → 一个 assert (断言强度守恒)

use aok::OK;
use v8_strings::char_predicates::{
  is_identifier_part, is_identifier_start, is_white_space, is_white_space_or_line_terminator,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

// TEST(CharPredicatesTest, WhiteSpace)
#[test]
fn white_space() -> aok::Void {
  for c in [
    0x0009, 0x000B, 0x000C, 0x0020, 0x00A0, 0x1680, 0x2000, 0x2007, 0x202F, 0x205F, 0x3000, 0xFEFF,
  ] {
    assert!(is_white_space(c), "IsWhiteSpace({c:#06x}) 应为 true");
  }
  assert!(!is_white_space(0x180E));
  OK
}

// TEST(CharPredicatesTest, WhiteSpaceOrLineTerminator)
#[test]
fn white_space_or_line_terminator() -> aok::Void {
  for c in [
    0x0009, 0x000B, 0x000C, 0x0020, 0x00A0, 0x1680, 0x2000, 0x2007, 0x202F, 0x205F, 0xFEFF,
  ] {
    assert!(
      is_white_space_or_line_terminator(c),
      "IsWhiteSpaceOrLineTerminator({c:#06x}) 应为 true"
    );
  }
  // Line terminators
  for c in [0x000A, 0x000D, 0x2028, 0x2029] {
    assert!(is_white_space_or_line_terminator(c));
  }
  assert!(!is_white_space_or_line_terminator(0x180E));
  OK
}

// TEST(CharPredicatesTest, IdentifierStart)
#[test]
fn identifier_start() -> aok::Void {
  assert!(is_identifier_start(u32::from(b'$')));
  assert!(is_identifier_start(u32::from(b'_')));
  assert!(is_identifier_start(u32::from(b'\\')));

  // Other_ID_Start
  for c in [0x1885, 0x1886, 0x2118, 0x212E, 0x309B, 0x309C] {
    assert!(is_identifier_start(c), "Other_ID_Start {c:#06x}");
  }

  // Issue 2892: \u2E2F 有 Pattern_Syntax, 不在 ID_Start
  assert!(!is_identifier_start(0x2E2F));

  // INTL: Unicode 8.0 新增
  for c in [
    0x08B3, 0x0AF9, 0x13F8, 0x9FCD, 0xAB60, 0x10CC0, 0x108E0, 0x2B820,
  ] {
    assert!(is_identifier_start(c), "Unicode 8.0 {c:#06x}");
  }
  // INTL: Unicode 9.0 新增
  for c in [0x1C80, 0x104DB, 0x1E922] {
    assert!(is_identifier_start(c), "Unicode 9.0 {c:#06x}");
  }
  OK
}

// TEST(CharPredicatesTest, IdentifierPart)
#[test]
fn identifier_part() -> aok::Void {
  assert!(is_identifier_part(u32::from(b'$')));
  assert!(is_identifier_part(u32::from(b'_')));
  assert!(is_identifier_part(u32::from(b'\\')));
  // ZWNJ / ZWJ
  assert!(is_identifier_part(0x200C));
  assert!(is_identifier_part(0x200D));

  // INTL: Unicode 8.0 (ID_Start 新增)
  for c in [
    0x08B3, 0x0AF9, 0x13F8, 0x9FCD, 0xAB60, 0x10CC0, 0x108E0, 0x2B820,
  ] {
    assert!(is_identifier_part(c), "Unicode 8.0 start {c:#06x}");
  }
  // INTL: Unicode 8.0 (ID_Continue - ID_Start)
  for c in [0x08E3, 0xA69E, 0x11730] {
    assert!(is_identifier_part(c), "Unicode 8.0 continue {c:#06x}");
  }
  // INTL: Unicode 9.0 (ID_Start 新增)
  for c in [0x1C80, 0x104DB, 0x1E922] {
    assert!(is_identifier_part(c), "Unicode 9.0 start {c:#06x}");
  }
  // INTL: Unicode 9.0 (ID_Continue - ID_Start)
  for c in [0x08D4, 0x1DFB, 0xA8C5, 0x11450] {
    assert!(is_identifier_part(c), "Unicode 9.0 continue {c:#06x}");
  }

  // Other_ID_Start
  for c in [0x1885, 0x1886, 0x2118, 0x212E, 0x309B, 0x309C] {
    assert!(is_identifier_part(c), "Other_ID_Start {c:#06x}");
  }
  // Other_ID_Continue
  for c in [0x00B7, 0x0387, 0x1369, 0x1370, 0x1371, 0x19DA] {
    assert!(is_identifier_part(c), "Other_ID_Continue {c:#06x}");
  }

  assert!(!is_identifier_part(0x2E2F));
  OK
}

// TEST(CharPredicatesTest, SupplementaryPlaneIdentifiers) [V8_INTL_SUPPORT]
#[test]
fn supplementary_plane_identifiers() -> aok::Void {
  // ID_Start 且 ID_Continue
  assert!(is_identifier_start(0x10403)); // Lu
  assert!(is_identifier_part(0x10403));
  assert!(is_identifier_start(0x1043C)); // Ll
  assert!(is_identifier_part(0x1043C));
  assert!(is_identifier_start(0x16F9C)); // Lm
  assert!(is_identifier_part(0x16F9C));
  assert!(is_identifier_start(0x10048)); // Lo
  assert!(is_identifier_part(0x10048));
  assert!(is_identifier_start(0x1014D)); // Nl
  assert!(is_identifier_part(0x1014D));

  // Unicode 8.0
  assert!(is_identifier_start(0x108E0));
  assert!(is_identifier_start(0x10C80));

  // 仅 ID_Continue
  assert!(!is_identifier_start(0x101FD)); // Mn
  assert!(is_identifier_part(0x101FD));
  assert!(!is_identifier_start(0x11002)); // Mc
  assert!(is_identifier_part(0x11002));
  assert!(!is_identifier_start(0x104A9)); // Nd
  assert!(is_identifier_part(0x104A9));

  // 两者皆非
  assert!(!is_identifier_start(0x10111)); // No
  assert!(!is_identifier_part(0x10111));
  assert!(!is_identifier_start(0x1F4A9)); // So
  assert!(!is_identifier_part(0x1F4A9));
  OK
}
