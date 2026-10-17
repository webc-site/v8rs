//! ConsStringIterator / StringCharacterStream / WriteToFlat(2) 规约
//! (cctest/test-strings.cc:539 Traverse / 771 边界 / 853 DeepOneByte +
//!  string.cc:781-1123)。

use std::str::from_utf8;
pub mod common;

use aok::{OK, Void};
use common::{Arena, Maps};
use v8_objects_core::tagged::Tagged;
use v8_objects_strings::string::{
  ConsExt, ConsString, ConsStringIterator, String as Vstring, StringCharacterStream, StringExt,
  write_to_flat, write_to_flat2,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 叶串长度和 (测试对数用)。
fn total_len(arena: &mut Arena, maps: &Maps, parts: &[&str]) -> Tagged<Vstring> {
  let mut iter = parts.iter();
  let first = arena.seq_one_byte(maps.seq_one, iter.next().unwrap().as_bytes());
  let mut acc = first;
  for p in iter {
    let s = arena.seq_one_byte(maps.seq_one, p.as_bytes());
    let len = unsafe { acc.length() + s.length() };
    acc = arena.cons(maps.cons_one, len, acc, s);
  }
  acc
}

/// Traverse (cctest test-strings.cc:539): 迭代器拼接 == 原文。
#[test]
fn cons_iterator_traverse() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let cases: Vec<Vec<&str>> = vec![
    vec!["", "", ""],
    vec!["a", ""],
    vec!["", "a"],
    vec!["abc", "def", "ghi"],
    vec!["", "a", "", "b", "", "c", ""],
    vec!["abcdef", "g", "hijk", "", "lmnop"],
  ];
  for parts in cases {
    let root = total_len(&mut arena, &maps, &parts);
    let expect: String = parts.concat();
    // SAFETY: 竞技场对象布局契约成立
    unsafe {
      let cons: Tagged<ConsString> = root.cast();
      let mut it = ConsStringIterator::new(Some(cons), 0);
      let mut got = String::new();
      let mut offset = 0u32;
      while let Some(seg) = it.next(&mut offset) {
        let flat = seg.get_flat_content();
        for i in 0..flat.length() {
          got.push(flat.get(i) as u8 as char);
        }
      }
      assert_eq!(got, expect, "parts={:?}", parts);
    }
  }
  OK
}

/// DeepOneByte (cctest test-strings.cc:853): 深度 > 栈容量 32 触发重启搜索。
#[test]
fn cons_iterator_deep_restart() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  // 40 层右倾树: (((a + b) + c) + ...) — 深度超过 kStackSize=32
  let mut acc = arena.seq_one_byte(maps.seq_one, b"a0");
  for i in 1..40 {
    let s = arena.seq_one_byte(maps.seq_one, format!("a{i}").as_bytes());
    let len = unsafe { acc.length() + s.length() };
    acc = arena.cons(maps.cons_one, len, acc, s);
  }
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    let mut it = ConsStringIterator::new(Some(acc.cast()), 0);
    let mut got = String::new();
    let mut offset = 0u32;
    while let Some(seg) = it.next(&mut offset) {
      got.push_str(from_utf8(seg.get_flat_content().to_one_byte_vector()).unwrap());
    }
    let expect: String = (0..40).map(|i| format!("a{i}")).collect();
    assert_eq!(got, expect);
    assert_eq!(got.len(), expect.len());
  }
  OK
}

/// StringCharacterStream (string-inl.h:1684): 逐字符消费 == 拼接原文。
#[test]
fn character_stream_walk() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let a = arena.seq_one_byte(maps.seq_one, b"Hello, ");
  let b = arena.seq_two_byte(maps.seq_two, &[0x41, 0x2028]);
  let c = arena.seq_one_byte(maps.seq_one, b"world");
  let len = unsafe { a.length() + b.length() + c.length() };
  let cons_ab = arena.cons(maps.cons_two, unsafe { a.length() + b.length() }, a, b);
  let root = arena.cons(maps.cons_two, len, cons_ab, c);
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    let mut stream = StringCharacterStream::new(root, 0);
    let mut got = vec![];
    while stream.has_more() {
      got.push(stream.get_next());
    }
    let expect: Vec<u16> = "Hello, "
      .encode_utf16()
      .chain([0x41, 0x2028])
      .chain("world".encode_utf16())
      .collect();
    assert_eq!(got, expect);

    // 带偏移 reset (段起点)
    let mut stream = StringCharacterStream::new(root, 7);
    let mut got2 = vec![];
    while stream.has_more() {
      got2.push(stream.get_next());
    }
    let expect2: Vec<u16> = [0x41u16, 0x2028]
      .into_iter()
      .chain("world".encode_utf16().collect::<Vec<_>>())
      .collect();
    assert_eq!(got2, expect2);
  }
  OK
}

/// WriteToFlat (string.cc:790): u8/u16 汇, Cons/Sliced/Thin 组合。
#[test]
fn write_to_flat_shapes() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let a = arena.seq_one_byte(maps.seq_one, b"foo-");
  let b = arena.seq_one_byte(maps.seq_one, b"bar-");
  let c = arena.seq_one_byte(maps.seq_one, b"baz");
  let ab = arena.cons(maps.cons_one, 8, a, b);
  let abc = arena.cons(maps.cons_one, 11, ab, c);
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    // u8 汇全串
    let mut sink = vec![0u8; 11];
    write_to_flat(abc, &mut sink, 0, 11).ok();
    assert_eq!(&sink, b"foo-bar-baz");
    // 区间 [4..11) (右子树为主)
    let mut sink = vec![0u8; 7];
    write_to_flat(abc, &mut sink, 4, 7).ok();
    assert_eq!(&sink, b"bar-baz");
    // 区间 [2..9) 跨边界
    let mut sink = vec![0u8; 7];
    write_to_flat(abc, &mut sink, 2, 7).ok();
    assert_eq!(&sink, b"o-bar-b");
    // u16 汇 (双字节内容)
    let x = arena.seq_two_byte(maps.seq_two, &[0x111, 0x122]);
    let y = arena.seq_two_byte(maps.seq_two, &[0x133]);
    let xy = arena.cons(maps.cons_two, 3, x, y);
    let mut sink16 = vec![0u16; 3];
    write_to_flat(xy, &mut sink16, 0, 3).ok();
    assert_eq!(&sink16, &[0x111, 0x122, 0x133]);
    // 单字节源 → u16 汇零扩展
    let mut sink16 = vec![0u16; 11];
    write_to_flat(abc, &mut sink16, 0, 11).ok();
    assert_eq!(sink16, "foo-bar-baz".encode_utf16().collect::<Vec<_>>());
    // Sliced of Cons(展平) — SliceFromCons 语义 (cctest:1298)
    let sliced = arena.sliced(maps.sliced_one, 4, abc, 4);
    let mut sink = vec![0u8; 4];
    write_to_flat(sliced, &mut sink, 0, 4).ok();
    assert_eq!(&sink, b"bar-");
    // Thin 穿透
    let thin = arena.thin(maps.thin_one, 11, abc);
    let mut sink = vec![0u8; 11];
    write_to_flat(thin, &mut sink, 0, 11).ok();
    assert_eq!(&sink, b"foo-bar-baz");
    // 空 length no-op
    let mut sink = vec![0u8; 0];
    write_to_flat(abc, &mut sink, 0, 0).ok();
  }
  OK
}

/// WriteToFlat2 (string.cc:1081): 反向 DFS + 幂等 Cons 重复优化。
#[test]
fn write_to_flat2_variants() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  // 左倾链 (append 模式): (((d + c) + b) + a) 反向游标栈深恒 1
  let leaf = arena.seq_one_byte(maps.seq_one, b"ab");
  let mut acc = leaf;
  for i in 0..8 {
    let s = arena.seq_one_byte(maps.seq_one, format!(".{i}").as_bytes());
    let len = unsafe { acc.length() + s.length() };
    acc = arena.cons(maps.cons_one, len, acc, s);
  }
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    let mut dst = vec![0u8; acc.length() as usize];
    write_to_flat2(&mut dst, acc.cast()).ok();
    let mut expect = String::from("ab");
    for i in 0..8 {
      expect.push_str(&format!(".{i}"));
    }
    assert_eq!(dst, expect.as_bytes());

    // 幂等 Cons (first == second): 首现复本消除
    let x = arena.seq_one_byte(maps.seq_one, b"xy");
    let dup = arena.cons(maps.cons_one, 4, x, x);
    let mut dst = vec![0u8; 4];
    write_to_flat2(&mut dst, dup.cast()).ok();
    assert_eq!(&dst, b"xyxy");
    // 与 WriteToFlat 结果一致
    let mut dst2 = vec![0u8; 4];
    write_to_flat(dup, &mut dst2, 0, 4).ok();
    assert_eq!(dst, dst2);

    // 嵌套幂等: ((xy + xy) + (xy + xy))
    let inner = arena.cons(maps.cons_one, 4, x, x);
    let outer = arena.cons(maps.cons_one, 8, inner, inner);
    let mut dst = vec![0u8; 8];
    write_to_flat2(&mut dst, outer.cast()).ok();
    assert_eq!(&dst, b"xyxyxyxy");

    // 混合双字节叶 → u16 汇
    let u = arena.seq_two_byte(maps.seq_two, &[0x201, 0x202]);
    let v = arena.seq_one_byte(maps.seq_one, b"q");
    let mix = arena.cons(maps.cons_two, 3, u, v);
    let mut dst16 = vec![0u16; 3];
    write_to_flat2(&mut dst16, mix.cast()).ok();
    assert_eq!(&dst16, &[0x201, 0x202, u16::from(b'q')]);
  }
  OK
}

/// ConsString::Get 树内定位 + unchecked_second Relaxed 读 (string.cc:2135)。
#[test]
fn cons_get_and_flat_flag() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let a = arena.seq_one_byte(maps.seq_one, b"abc");
  let b = arena.seq_one_byte(maps.seq_one, b"de");
  let cons = arena.cons(maps.cons_one, 5, a, b);
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    let cons: Tagged<ConsString> = cons.cast();
    assert!(!cons.is_flat());
    assert_eq!(cons.get(0), u16::from(b'a'));
    assert_eq!(cons.get(2), u16::from(b'c'));
    assert_eq!(cons.get(3), u16::from(b'd'));
    assert_eq!(cons.get(4), u16::from(b'e'));
    assert_eq!(cons.unchecked_second().length(), 2);
    // 展平后 IsFlat → Get 落左子树
    let empty = arena.seq_one_byte(maps.seq_one, b"");
    let flat = arena.cons(maps.cons_one, 3, a, empty);
    let flat: Tagged<ConsString> = flat.cast();
    assert!(flat.is_flat());
    assert_eq!(flat.get(1), u16::from(b'b'));
  }
  OK
}
