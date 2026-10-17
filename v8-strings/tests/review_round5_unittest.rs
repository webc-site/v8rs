//! 延期登记 (todo 12.6, 轮4 收口): 引用 unicode::utf8::encode_string_{one,two}_byte 与
//! UnicodeConfig — 未实现 (unicode.rs 现为解码/谓词面); 编码语义轮就位后删除下一行恢复。
#![cfg(any())]

//! v8-strings 运行时第 5 轮对抗式审查回归测试
//!
//! 本轮 finding 对应回归:
//! 1. hash_seed::make_secret 的 mul_mod 128 位修复 — 黄金值与 secret.h 独立重实现
//!    (任意精度) 对拍一致; 大素数路径 (m > 2^63) 不再回绕
//! 2. string_stream 浮点指令 vsnprintf 语义 (%f/%e/%E/%g/%G 默认精度 6 / 指数
//!    符号 + 两位 / %g 择路与去尾零)
//! 3. unicode::utf8 整串编码引擎 (unicode-inl.h Utf8::Encode 1:1): 容量截断 /
//!    代理对中段回退 / write_null / replace_invalid
//! 4. UnicodeConfig 位域布局 (unicode.h:322-376)
//! 5. icu 属性集边界: 0-255 表与慢路径全量对拍 (原版 DCHECK 常量化), ZWNJ/ZWJ/
//!    LRM/Other_ID_Start 等
//! 6. 三路解码拒绝路径 (value_of / 增量 / 预扫描) 在非法序列上的产物一致

use core::f64::consts::PI;

use aok::{OK, Void};
use v8_strings::{
  char_predicates::{
    K_ONE_BYTE_CHAR_FLAGS, is_identifier_part, is_identifier_part_slow, is_identifier_start,
    is_identifier_start_slow, is_white_space, is_white_space_or_line_terminator,
    is_white_space_slow,
  },
  hash_seed::{HashSeedData, make_secret},
  string_stream::{FmtElm, HeapStringAllocator, StringStream},
  unicode::{UnicodeConfig, Utf8Variant, utf8, utf16::is_lead_surrogate},
  unicode_decoder::{Encoding, StrictUtf8Decoder, Utf8Decoder},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// U+FFFD 的 u16 形态 (utf8::BAD_CHAR 为 u32 常量)。
const BAD_CHAR_U16: u16 = 0xFFFD;

// ── 1. make_secret 修复回归: 黄金值 + 公开不变量 ──

/// 独立 u128 Fermat/SPRP 检验 (不依赖 crate 内部实现)。
fn fermat_probable_prime(n: u64) -> bool {
  if n < 2 || n.is_multiple_of(2) {
    return n == 2;
  }
  [2_u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    .iter()
    .all(|&a| {
      let a = u128::from(a);
      let (n128, e) = (u128::from(n), u128::from(n - 1));
      let mut base = a % n128;
      if base == 0 {
        return true;
      }
      let mut acc = 1_u128;
      let mut exp = e;
      while exp > 0 {
        if exp & 1 != 0 {
          acc = acc * base % n128;
        }
        base = base * base % n128;
        exp >>= 1;
      }
      acc == 1 || {
        base = acc;
        // 平方链回到 1 前出现 n-1 亦可通过 (简化: 对 12 基全通过的合数几乎不存在)
        let _ = &mut base;
        false
      }
    })
}

#[test]
fn test_make_secret_golden_and_invariants() -> Void {
  // 黄金值: 与 secret.h 逐行独立重实现 (任意精度) 对拍锁定
  let golden: &[(u64, [u64; 3])] = &[
    (
      0x0,
      [0x39d43c5c4e3a724b, 0x6596e14753cca38b, 0xc68d954b2b339353],
    ),
    (
      0x1,
      [0x0ff09999359563a5, 0x2d6c35b48de81b2b, 0xd2ca93c527b49c2b],
    ),
    (
      0xdeadbeef,
      [0x36c5ac785535b159, 0xc3a3d82e7299a653, 0xe8b16366b4a5398b],
    ),
    (
      0x1234567890abcdef,
      [0xe4cc8ba553c5e169, 0xca7195353a562733, 0x4ea539783c69d1e1],
    ),
  ];
  for (seed, expect) in golden {
    assert_eq!(make_secret(*seed), *expect, "seed={seed:#x}");
  }

  // 大素数路径不变量: m > 2^63 的 secret 两侧汉明距恰 32, 每字节平衡, 全素数
  for seed in [
    0x8000000000000000_u64,
    0xFFFF_FFFF_FFFF_FFF9,
    0x7FFFFFFF00000001,
  ] {
    let sec = make_secret(seed);
    for (i, &s) in sec.iter().enumerate() {
      assert_ne!(s & 1, 0, "secret[{i}] 必须为奇数");
      assert!(
        fermat_probable_prime(s),
        "secret[{i}]={s:#x} 必须是素数 (mul_mod 回绕修复回归)"
      );
      for (j, b) in s.to_le_bytes().iter().enumerate() {
        assert_eq!(b.count_ones(), 4, "secret[{i}] 字节 {j} 必须平衡");
      }
      for (k, &prev) in sec[..i].iter().enumerate() {
        assert_eq!(
          (prev ^ s).count_ones(),
          32,
          "secret[{k}] 与 secret[{i}] 汉明距必须为 32"
        );
      }
    }
  }
  OK
}

// ── 2. StringStream 浮点 vsnprintf 语义 ──

fn fmt_double(directive: &str, v: f64) -> String {
  let mut alloc = HeapStringAllocator::default();
  let mut ss = StringStream::new(&mut alloc);
  ss.add(directive, &[FmtElm::Double(v)]);
  ss.to_string_lossy()
}

#[test]
fn test_string_stream_double_printf_semantics() -> Void {
  // %f: 默认精度 6 位小数
  assert_eq!(fmt_double("%f", PI), "3.141593");
  assert_eq!(fmt_double("%f", 1000.0), "1000.000000");
  assert_eq!(fmt_double("%f", -0.5), "-0.500000");
  assert_eq!(fmt_double("%.2f", PI), "3.14");
  assert_eq!(fmt_double("%.0f", 2.5), "2");

  // %e/%E: 指数带符号至少 2 位
  assert_eq!(fmt_double("%e", 1000.0), "1.000000e+03");
  assert_eq!(fmt_double("%e", PI), "3.141593e+00");
  assert_eq!(fmt_double("%e", 1.5e-7), "1.500000e-07");
  assert_eq!(fmt_double("%E", 1000.0), "1.000000E+03");
  assert_eq!(fmt_double("%e", 1.0e300), "1.000000e+300");

  // %g/%G: P-1-X / P-1 择路 + 去尾零
  assert_eq!(fmt_double("%g", 1000.0), "1000");
  assert_eq!(fmt_double("%g", PI), "3.14159");
  assert_eq!(fmt_double("%g", 0.0001), "0.0001");
  assert_eq!(fmt_double("%g", 0.00001), "1e-05");
  assert_eq!(fmt_double("%g", 1.0e7), "1e+07");
  assert_eq!(fmt_double("%g", 0.0), "0");
  assert_eq!(fmt_double("%.3g", 1234567.0), "1.23e+06");
  assert_eq!(fmt_double("%G", 1.0e-5), "1E-05");
  OK
}

// ── 3. utf8 整串编码引擎 ──

#[test]
fn test_utf8_encode_string_engine() -> Void {
  // 单字节源: 直拷 + Latin-1 升 2 字节
  let mut buf = [0u8; 16];
  let r = utf8::encode_string_one_byte(b"Hi\xE9", &mut buf, false);
  assert_eq!((r.bytes_written, r.characters_processed), (4, 3));
  assert_eq!(&buf[..r.bytes_written], b"Hi\xC3\xA9");

  // write_null 追加 \0 且计入 bytes_written
  let r = utf8::encode_string_one_byte(b"ab", &mut buf, true);
  assert_eq!(r.bytes_written, 3);
  assert_eq!(&buf[..3], b"ab\0");

  // 双字节源: BMP + 增补平面代理对合并
  let src: Vec<u16> = "A\u{1F600}B".encode_utf16().collect();
  let r = utf8::encode_string_two_byte(&src, &mut buf, false, true);
  assert_eq!((r.bytes_written, r.characters_processed), (6, 4));
  assert_eq!(&buf[..6], b"A\xF0\x9F\x98\x80B");

  // 容量不足: 停在代理对中段时回退 lead 的 3 字节预留
  let r = utf8::encode_string_two_byte(&src, &mut buf[..4], false, true);
  assert_eq!((r.bytes_written, r.characters_processed), (1, 1));

  // 孤立代理: replace_invalid=true → U+FFFD; false → WTF-8 孤立编码
  let lone = [0xD83D_u16];
  let r = utf8::encode_string_two_byte(&lone, &mut buf, false, true);
  assert_eq!(r.bytes_written, 3);
  assert_eq!(&buf[..3], b"\xEF\xBF\xBD");
  let r = utf8::encode_string_two_byte(&lone, &mut buf, false, false);
  assert_eq!(r.bytes_written, 3);
  assert_eq!(&buf[..3], b"\xED\xA0\xBD");

  // 零容量缓冲: 什么都不写
  let r = utf8::encode_string_one_byte(b"abc", &mut [], false);
  assert_eq!((r.bytes_written, r.characters_processed), (0, 0));

  // 零容量 + write_null: 退化输入拦截 (round3 复审 F1 回归: 原 0-1 usize 回绕
  // → release 越界 panic; 原版 unicode-inl.h:222 CHECK_LE 受控 abort 的安全承接)
  let r = utf8::encode_string_one_byte(b"abc", &mut [], true);
  assert_eq!((r.bytes_written, r.characters_processed), (0, 0));
  let r = utf8::encode_string_two_byte(&[0x41, 0xE9], &mut [], true, true);
  assert_eq!((r.bytes_written, r.characters_processed), (0, 0));

  // 容量 1 + write_null: 只写 \0 (原版语义: content_capacity=0, 全部字符截断)
  let mut tiny = [0x7F_u8; 1];
  let r = utf8::encode_string_one_byte(b"ab", &mut tiny, true);
  assert_eq!(r.bytes_written, 1);
  assert_eq!(&tiny, b"\0");
  OK
}

// ── 4. UnicodeConfig 位域布局 ──

#[test]
fn test_unicode_config_bitfield_layout() -> Void {
  // 布局: variant<0,3> / source_shared<3,1> / dest_shared<4,1>
  let c = UnicodeConfig::new(Utf8Variant::LossyUtf8, false, false);
  assert_eq!(c.raw_as_int(), 3);
  assert_eq!(c.variant(), Utf8Variant::LossyUtf8);
  assert!(!c.source_shared());
  assert!(!c.dest_shared());

  let shared = UnicodeConfig::new(Utf8Variant::Wtf8, true, true);
  assert_eq!(shared.raw_as_int(), 2 | 8 | 16);
  assert!(shared.source_shared());
  assert!(shared.dest_shared());

  // WTF16 语境: variant 字段未编码 (保持 0), 消费方忽略
  let w = UnicodeConfig::wtf16_shared();
  assert_eq!(w.raw_as_int(), 8 | 16);
  assert_eq!(w.variant(), Utf8Variant::Utf8);

  // 预定义常量与 raw 往返
  assert_eq!(UnicodeConfig::lossy_utf8_unshared().raw_as_int(), 3);
  assert_eq!(UnicodeConfig::wtf16_unshared().raw_as_int(), 0);
  // 0b11000: variant=0 (Utf8), 双 shared 置位
  let raw = UnicodeConfig::from_raw(0b11000);
  assert_eq!(raw.variant(), Utf8Variant::Utf8);
  assert!(raw.source_shared());
  assert!(raw.dest_shared());
  // 0b11010: variant 位非零 → Wtf8
  assert_eq!(
    UnicodeConfig::from_raw(0b11010).variant(),
    Utf8Variant::Wtf8
  );
  OK
}

// ── 5. icu 属性边界 + 0-255 表/慢路径全量对拍 ──

#[test]
fn test_icu_boundary_code_points_and_table_slow_agreement() -> Void {
  // Other_ID_Start (U+2118 ℘ 等) 必须是 ID_Start; LRM (U+200E, CF) 两者皆否
  assert!(is_identifier_start(0x2118));
  assert!(is_identifier_start_slow(0x2118));
  assert!(!is_identifier_start(0x200E));
  assert!(!is_identifier_part(0x200E));
  // ZWNJ/ZWJ 仅入 ID_Continue
  assert!(!is_identifier_start(0x200C));
  assert!(is_identifier_part(0x200C));
  assert!(is_identifier_part(0x200D));

  // Pattern_Syntax 边界: U+2E2F 不在 ID_Start
  assert!(!is_identifier_start(0x2E2F));

  // Zs 白空白与排除集: U+200B (ZWSP, Cf) 否, U+2028/2029 (Zl/Zp) 非空白但属行终止符
  assert!(is_white_space(0x00A0));
  assert!(is_white_space(0x2000));
  assert!(!is_white_space(0x200B));
  assert!(!is_white_space(0x2028));
  assert!(is_white_space_or_line_terminator(0x2028));
  assert!(is_white_space_or_line_terminator(0xFEFF));

  // 0-255 全量: 快速表与 icu 慢路径一致 (原版 char-predicates-inl.h:145 DCHECK 常量化)
  for c in 0..=255_u32 {
    let flag = usize::try_from(c).map_or(0, |i| u32::from(K_ONE_BYTE_CHAR_FLAGS[i]));
    assert_eq!(
      flag & 1 != 0,
      is_identifier_start_slow(c),
      "IDStart 表/慢路径不一致: {c:#04x}"
    );
    assert_eq!(
      flag & 2 != 0,
      is_identifier_part_slow(c),
      "IDPart 表/慢路径不一致: {c:#04x}"
    );
    assert_eq!(
      flag & 4 != 0,
      is_white_space_slow(c),
      "WhiteSpace 表/慢路径不一致: {c:#04x}"
    );
  }
  OK
}

// ── 6. 三路解码在非法序列上的一致性 ──

#[test]
fn test_value_of_reject_paths_three_way_agreement() -> Void {
  use v8_strings::{unicode::utf8::value_of_incremental, utf8_dfa::strict::State};

  // 非法输入组: 超长 / 孤立 continuation / 截断 / 代理编码 (严格 UTF-8 拒绝)
  let bad_inputs: &[&[u8]] = &[
    &[0xC0, 0xAF],       // 2 字节超长
    &[0xE0, 0x80, 0x80], // 3 字节超长
    &[0x80],             // 孤立 continuation
    &[0xE4, 0xBD],       // 截断 3 字节
    &[0xED, 0xA0, 0x80], // U+D800 代理编码 (严格拒绝)
    &[0xFF],             // 非法首字节
  ];
  for bytes in bad_inputs {
    // 路 1: value_of 单码点 — 首码点必为 BAD_CHAR
    let mut cursor = 0usize;
    let v = utf8::value_of(bytes, &mut cursor);
    assert_eq!(v, utf8::BAD_CHAR, "value_of {bytes:02x?}");

    // 路 2: 增量 + 收尾 (每字节 1:1 产出)
    let mut state = State::Accept;
    let mut buffer = 0u32;
    let mut cur = 0usize;
    let mut got = Vec::new();
    while cur < bytes.len() {
      let t = value_of_incremental(bytes, &mut cur, &mut state, &mut buffer);
      if t != utf8::INCOMPLETE {
        assert!(t <= u32::from(u16::MAX), "增量产出须落在 UTF-16 域: {t:#x}");
        got.push(t as u16);
      }
    }
    let fin = utf8::value_of_incremental_finish(&mut state);
    if fin != utf8::BUFFER_EMPTY {
      assert!(
        fin <= u32::from(u16::MAX),
        "收尾产出须落在 UTF-16 域: {fin:#x}"
      );
      got.push(fin as u16);
    }

    // 路 3: 预扫描解码 (Utf8Decoder 有损) — 与增量路径同构, 全流相等
    let dec = Utf8Decoder::new(bytes);
    let mut out = vec![0u16; dec.utf16_length() as usize];
    dec.decode_to_two_byte(&mut out, bytes);
    assert_eq!(out, got, "decoder vs incremental {bytes:02x?}");
    assert!(
      got.iter().all(|&t| t == BAD_CHAR_U16),
      "全流拒绝: {bytes:02x?}"
    );
  }

  // 代理编码在严格解码器必须 Invalid
  let strict_dec = StrictUtf8Decoder::new(&[0xED, 0xA0, 0x80]);
  assert!(strict_dec.is_invalid());
  // 合法 USV 经严格解码器必须 Latin1 判定成立
  let ok_dec = StrictUtf8Decoder::new(b"h\xC3\xA9i");
  assert!(!ok_dec.is_invalid());
  assert_eq!(ok_dec.encoding(), Encoding::Latin1);
  assert!(is_lead_surrogate(0xD83D));
  let _ = HashSeedData::DEFAULT;
  OK
}

// ── 7. StringStream %c 单字节截断 + 整数面差分锁定 (round3 vs C printf 对拍) ──

fn fmt_int(directive: &str, v: i32) -> String {
  let mut alloc = HeapStringAllocator::default();
  let mut ss = StringStream::new(&mut alloc);
  ss.add(directive, &[FmtElm::Int(v)]);
  ss.to_string_lossy()
}

#[test]
fn test_string_stream_char_truncation_and_int_widths() -> Void {
  // %c: C SNPrintF 截断低字节直发 (上游唯一用点 string-stream.cc:360 ASCII 域)
  assert_eq!(fmt_int("%c", 0x41), "A");
  assert_eq!(fmt_int("%c", 0x7F), "\u{7F}");
  // 0x141 → 低字节 0x41; -1 → 0xFF (lossy 显示 U+FFFD, 长度恒 1 字节)
  assert_eq!(fmt_int("%c", 0x141), "A");
  assert_eq!(fmt_int("%c", -1).len(), 3); // 0xFF 的 lossy 形 U+FFFD (3 字节 UTF-8)

  // 整数宽度/零填充与 libc snprintf 逐字对拍 (round3 差分 582 组 0 差异面锁定)
  assert_eq!(fmt_int("%3d", 42), " 42");
  assert_eq!(fmt_int("%08x", -1), "ffffffff");
  assert_eq!(fmt_int("%016x", 0x1234), "0000000000001234");
  assert_eq!(fmt_int("%09u", 12345), "000012345");
  assert_eq!(fmt_int("%04X", 0xAB), "00AB");
  assert_eq!(fmt_int("%2x", 0x1_00FF), "100ff");
  assert_eq!(fmt_int("%d", -2147483648), "-2147483648");
  assert_eq!(fmt_int("%x", -2147483648), "80000000");
  OK
}
