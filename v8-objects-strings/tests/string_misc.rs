//! 搜索/比较/行结束/标识符/杂项规约 (string.cc:1453-1905)。

pub mod common;

use aok::{OK, Void};
use common::{Arena, Maps};
use v8_objects_core::objects::ComparisonResult;
use v8_objects_strings::string::{
  FlatContent, FlatStringReader, calculate_line_ends, compare_flat,
  does_not_contain_escape_characters, index_of_flat, is_identifier_flat, last_index_of_flat,
  sub_string_range, to_array_index,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// IndexOf/LastIndexOf 展平内容版 (string.cc:1566/1789)。
#[test]
fn index_of_and_last_index_of() -> Void {
  let sub = FlatContent::OneByte(b"the cat in the hat");
  // 首中/后中/未中
  assert_eq!(index_of_flat(&sub, &FlatContent::OneByte(b"the"), 0), 0);
  assert_eq!(index_of_flat(&sub, &FlatContent::OneByte(b"the"), 1), 11);
  assert_eq!(index_of_flat(&sub, &FlatContent::OneByte(b"cat"), 0), 4);
  assert_eq!(index_of_flat(&sub, &FlatContent::OneByte(b"dog"), 0), -1);
  // 越界窗口
  assert_eq!(index_of_flat(&sub, &FlatContent::OneByte(b"hat"), 16), -1);
  // 空搜索串 → start
  assert_eq!(index_of_flat(&sub, &FlatContent::OneByte(b""), 5), 5);
  // 交叉宽度
  let pat16: &'static [u16] = &[b'h' as u16, b'a' as u16, b't' as u16];
  assert_eq!(index_of_flat(&sub, &FlatContent::TwoByte(pat16), 0), 15);
  let sub16: &'static [u16] = &[0x2028, b'a' as u16];
  assert_eq!(
    index_of_flat(&FlatContent::TwoByte(sub16), &FlatContent::OneByte(b"a"), 0),
    1
  );

  // LastIndexOf
  assert_eq!(
    last_index_of_flat(&sub, &FlatContent::OneByte(b"the"), 18),
    11
  );
  assert_eq!(
    last_index_of_flat(&sub, &FlatContent::OneByte(b"the"), 10),
    0
  );
  assert_eq!(
    last_index_of_flat(&sub, &FlatContent::OneByte(b"cat"), 18),
    4
  );
  assert_eq!(
    last_index_of_flat(&sub, &FlatContent::OneByte(b"dog"), 18),
    -1
  );
  assert_eq!(last_index_of_flat(&sub, &FlatContent::OneByte(b""), 18), 18);
  // 窗口起点饱和
  assert_eq!(
    last_index_of_flat(&sub, &FlatContent::OneByte(b"the"), 18),
    11
  );
  // 双字节模式含非 Latin-1 → 单字节主串必 -1 (string.cc:1761)
  let pat_hi: &'static [u16] = &[b'a' as u16, 0x2028];
  assert_eq!(
    last_index_of_flat(&sub, &FlatContent::TwoByte(pat_hi), 18),
    -1
  );
  // 模式长于主串 → -1
  assert_eq!(
    last_index_of_flat(&sub, &FlatContent::OneByte(b"the cat in the hat!"), 0),
    -1
  );
  OK
}

/// String::Compare (string.cc:1453): 字典序与长度倾侧。
#[test]
fn compare_flat_ordering() -> Void {
  let a = FlatContent::OneByte(b"abc");
  let b = FlatContent::OneByte(b"abd");
  let a16: &'static [u16] = &[b'a' as u16, b'b' as u16, b'c' as u16];
  assert_eq!(compare_flat(&a, &b), ComparisonResult::LessThan);
  assert_eq!(compare_flat(&b, &a), ComparisonResult::GreaterThan);
  assert_eq!(compare_flat(&a, &a), ComparisonResult::Equal);
  // 前缀相等 + 长度差
  let ab = FlatContent::OneByte(b"abcd");
  assert_eq!(compare_flat(&a, &ab), ComparisonResult::LessThan);
  assert_eq!(compare_flat(&ab, &a), ComparisonResult::GreaterThan);
  // 码元值序: 'Z'(0x5A) < 'a'(0x61)
  let z = FlatContent::OneByte(b"Z");
  let aa = FlatContent::OneByte(b"a");
  assert_eq!(compare_flat(&z, &aa), ComparisonResult::LessThan);
  // 交叉宽度
  assert_eq!(
    compare_flat(&a, &FlatContent::TwoByte(a16)),
    ComparisonResult::Equal
  );
  // 双字节内容序
  let hi: &'static [u16] = &[0x2028];
  let lo: &'static [u16] = &[0xFFFF];
  assert_eq!(
    compare_flat(&FlatContent::TwoByte(hi), &FlatContent::TwoByte(lo)),
    ComparisonResult::LessThan
  );
  OK
}

/// CalculateLineEnds (string.cc:1172): \n \r \r\n U+2028 + 尾行旗标。
#[test]
fn line_ends() -> Void {
  // \n 与 \r 分行; \r\n 计一行
  // \r\n 按一行的序列计在 \n 位 (IsLineTerminatorSequence: \r 后随 \n 不计)
  let c = FlatContent::OneByte(b"a\nb\r\nc\rd");
  assert_eq!(calculate_line_ends(&c, false), vec![1, 4, 6]);
  assert_eq!(calculate_line_ends(&c, true), vec![1, 4, 6, 8]);
  // 结尾换行
  let t = FlatContent::OneByte(b"x\n");
  assert_eq!(calculate_line_ends(&t, false), vec![1]);
  // 空串
  let e = FlatContent::OneByte(b"");
  assert_eq!(calculate_line_ends(&e, true), vec![0]);
  assert_eq!(calculate_line_ends(&e, false), Vec::<i32>::new());
  // 双字节 U+2028/U+2029 (string.cc:1228 分支)
  let u: &'static [u16] = &[b'a' as u16, 0x2028, b'b' as u16, 0x2029];
  assert_eq!(
    calculate_line_ends(&FlatContent::TwoByte(u), false),
    vec![1, 3]
  );
  OK
}

/// DoesNotContainEscapeCharacters (string.cc:1157)。
#[test]
fn escape_characters() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let clean = arena.seq_one_byte(maps.seq_one, b"plain");
  let quoted = arena.seq_one_byte(maps.seq_one, b"a\"b");
  let back = arena.seq_one_byte(maps.seq_one, b"a\\b");
  let ctrl = arena.seq_one_byte(maps.seq_one, b"a\x01b");
  let units: Vec<u16> = vec![b'a' as u16, 0xD800, b'b' as u16];
  let surrogate = arena.seq_two_byte(maps.seq_two, &units);
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    assert!(does_not_contain_escape_characters(clean));
    assert!(!does_not_contain_escape_characters(quoted));
    assert!(!does_not_contain_escape_characters(back));
    assert!(!does_not_contain_escape_characters(ctrl));
    assert!(!does_not_contain_escape_characters(surrogate));
  }
  OK
}

/// IsIdentifier (string.cc:1866-1893)。
#[test]
fn identifier_check() -> Void {
  assert!(is_identifier_flat(&FlatContent::OneByte(b"abc")));
  assert!(is_identifier_flat(&FlatContent::OneByte(b"_x1")));
  assert!(is_identifier_flat(&FlatContent::OneByte(b"$")));
  assert!(!is_identifier_flat(&FlatContent::OneByte(b"")));
  assert!(!is_identifier_flat(&FlatContent::OneByte(b"1a")));
  assert!(!is_identifier_flat(&FlatContent::OneByte(b"a-b")));
  assert!(!is_identifier_flat(&FlatContent::OneByte(b"a b")));
  let id: &'static [u16] = &[b'a' as u16, 0x0301]; // a + combining accent
  assert!(is_identifier_flat(&FlatContent::TwoByte(id)));
  OK
}

/// ToArrayIndex (string.cc:669) + SubStringRange + FlatStringReader。
#[test]
fn misc_apis() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let s = arena.seq_one_byte(maps.seq_one, b"12345");
  let big = arena.seq_one_byte(maps.seq_one, b"2147483648"); // 2^31 > INT_MAX
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    assert_eq!(to_array_index(s.cast()), Some(12345));
    assert_eq!(to_array_index(big.cast()), None);
    let neg = arena.seq_one_byte(maps.seq_one, b"abc");
    assert_eq!(to_array_index(neg.cast()), None);

    // SubStringRange: 码元迭代
    let padded = arena.seq_one_byte(maps.seq_one, b"xxabcdefxx");
    let sliced = arena.sliced(maps.sliced_one, 6, padded, 2);
    let got: Vec<u16> = sub_string_range(sliced, 0, None).collect();
    assert_eq!(got, "abcdef".encode_utf16().collect::<Vec<_>>());
    let got: Vec<u16> = sub_string_range(sliced, 1, Some(3)).collect();
    assert_eq!(got, "bcd".encode_utf16().collect::<Vec<_>>());

    // FlatStringReader
    let reader = FlatStringReader::new(sliced.cast());
    assert_eq!(reader.length(), 6);
    assert_eq!(reader.get(0), u32::from(b'a'));
    assert_eq!(reader.get(5), u32::from(b'f'));
  }
  OK
}
