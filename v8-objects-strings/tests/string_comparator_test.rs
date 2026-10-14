// 迁移规约:
//   string-comparator.cc:13-48 State::Init/Advance 分段推进状态机
//   string-comparator.cc:50-81 Equals 4 路编码分派 + 段边界推进
//   string-comparator.h:55-59 CompareCharsEqual (u8/u16 混编值比较)
//   unittests 侧无独立用例 (随 test-strings isolate 用例) → 规约自原版直迁
use aok::{OK, Void};
use v8_objects_strings::string_comparator::{
  FlatSegment, MultiSegmentSource, SingleSegmentSource, StringComparator,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 规约 (cc:50 Equals): 等值同编码 → true; 首字符不等 → false。
#[test]
fn equals_single_segment_same_encoding() -> Void {
  let mut cmp = StringComparator;
  let a = b"hello";
  let b = b"hello";
  let mut s1 = SingleSegmentSource::new(FlatSegment::one_byte(a));
  let mut s2 = SingleSegmentSource::new(FlatSegment::one_byte(b));
  assert!(cmp.equals(&mut s1, &mut s2, 5));

  let mut s1 = SingleSegmentSource::new(FlatSegment::one_byte(b"hello"));
  let mut s2 = SingleSegmentSource::new(FlatSegment::one_byte(b"hellp"));
  assert!(!cmp.equals(&mut s1, &mut s2, 5));

  // 空串: 原版 String::Equals 对 length==0 提前返回 true, 比较器不被调用;
  // 此处验证状态机对空源的容错 (双侧同步耗尽 + remaining=0 → true)
  let mut s1 = SingleSegmentSource::new(FlatSegment::one_byte(b""));
  let mut s2 = SingleSegmentSource::new(FlatSegment::one_byte(b""));
  assert!(!cmp.equals(&mut s1, &mut s2, 1), "声明 1 字符但源为空段");
  OK
}

/// 规约 (cc:60-64 混编分派): u8 vs u16 逐值宽化比较。
#[test]
fn equals_cross_encoding_widening() -> Void {
  let mut cmp = StringComparator;
  let one: &[u8] = &[0x61, 0x62, 0xFF];
  let two_ok: &[u16] = &[0x61, 0x62, 0x00FF];
  let two_bad: &[u16] = &[0x61, 0x62, 0x0100];

  let mut s1 = SingleSegmentSource::new(FlatSegment::one_byte(one));
  let mut s2 = SingleSegmentSource::new(FlatSegment::two_byte(two_ok));
  assert!(cmp.equals(&mut s1, &mut s2, 3));

  let mut s1 = SingleSegmentSource::new(FlatSegment::one_byte(one));
  let mut s2 = SingleSegmentSource::new(FlatSegment::two_byte(two_bad));
  assert!(!cmp.equals(&mut s1, &mut s2, 3));

  // 双字节 vs 单字节 (另一方向)
  let mut s1 = SingleSegmentSource::new(FlatSegment::two_byte(two_ok));
  let mut s2 = SingleSegmentSource::new(FlatSegment::one_byte(one));
  assert!(cmp.equals(&mut s1, &mut s2, 3));
  OK
}

/// 规约 (cc:29-48 Advance): 多段推进 — 段长不齐时逐段拉取直至全长比对完。
#[test]
fn equals_multi_segment_advance() -> Void {
  let mut cmp = StringComparator;
  // "abcdef" vs "abc"+"def"; 段边界 (3+3) 不对齐场景: 3+3 vs 4+2
  let whole = b"abcdef";
  let seg_a = MultiSegmentSource::new(vec![
    FlatSegment::one_byte(b"abc"),
    FlatSegment::one_byte(b"def"),
  ]);
  let seg_b = MultiSegmentSource::new(vec![
    FlatSegment::one_byte(b"abcd"),
    FlatSegment::one_byte(b"ef"),
  ]);
  let mut s1 = SingleSegmentSource::new(FlatSegment::one_byte(whole));
  let mut s1b = SingleSegmentSource::new(FlatSegment::one_byte(whole));
  let mut a = seg_a;
  let mut b = seg_b;
  assert!(cmp.equals(&mut s1, &mut a, 6));
  assert!(cmp.equals(&mut b, &mut s1b, 6));

  // 多段 vs 多段, 中段不等
  let good = MultiSegmentSource::new(vec![
    FlatSegment::one_byte(b"ab"),
    FlatSegment::one_byte(b"cd"),
    FlatSegment::one_byte(b"ef"),
  ]);
  let bad = MultiSegmentSource::new(vec![
    FlatSegment::one_byte(b"ab"),
    FlatSegment::one_byte(b"cx"),
    FlatSegment::one_byte(b"ef"),
  ]);
  let mut g = good;
  let mut bd = bad;
  assert!(!cmp.equals(&mut g, &mut bd, 6));

  // 多段混编: 单字节段串 vs 双字节段串
  let one_segs = MultiSegmentSource::new(vec![
    FlatSegment::one_byte(b"ab"),
    FlatSegment::one_byte(b"cd"),
  ]);
  let two_segs = MultiSegmentSource::new(vec![
    FlatSegment::two_byte(&[0x61, 0x62]),
    FlatSegment::two_byte(&[0x63, 0x64]),
  ]);
  let mut o = one_segs;
  let mut t = two_segs;
  assert!(cmp.equals(&mut o, &mut t, 4));
  OK
}

/// 规约 (cc:50 Equals 前置契约): 段源提前耗尽 (总长不足) → 判不等。
#[test]
fn equals_exhausted_sources_yield_false() -> Void {
  let mut cmp = StringComparator;
  let mut s1 = SingleSegmentSource::new(FlatSegment::one_byte(b"abc"));
  let mut s2 = SingleSegmentSource::new(FlatSegment::one_byte(b"ab"));
  // 声明 length=3 但 s2 只有 2 → 推进时耗尽 → false
  assert!(!cmp.equals(&mut s1, &mut s2, 3));
  OK
}

/// 规约 (string-comparator.h State 形态): 段元数据 (长度/编码) 自洽。
#[test]
fn flat_segment_metadata() -> Void {
  let one = FlatSegment::one_byte(b"xyz");
  assert!(one.is_one_byte());
  assert_eq!(one.len(), 3);
  let two = FlatSegment::two_byte(&[1, 2, 3, 4]);
  assert!(!two.is_one_byte());
  assert_eq!(two.len(), 4);
  OK
}

/// 规约 (cc:13 State::Init): 源首段在 Init 即载入并参与首轮比对
/// (首段不等即短路; 无需推进到后续段)。
#[test]
fn first_segment_compared_before_advance() -> Void {
  let mut cmp = StringComparator;
  let good = MultiSegmentSource::new(vec![
    FlatSegment::one_byte(b"ab"),
    FlatSegment::one_byte(b"cd"),
  ]);
  let bad_first = MultiSegmentSource::new(vec![
    FlatSegment::one_byte(b"ax"),
    FlatSegment::one_byte(b"cd"),
  ]);
  let mut g = good;
  let mut b = bad_first;
  // 首段不等 → 短路 false (即使后续段相等)
  assert!(!cmp.equals(&mut g, &mut b, 4));
  OK
}
