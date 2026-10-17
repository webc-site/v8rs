//! 展平内容与 Seq 尺寸/字符访问规约 (string-inl.h:1307-1405 + string.cc:2092-2113)。

pub mod common;

use aok::{OK, Void};
use common::{Arena, Maps, SeqOneByteExt, SeqTwoByteExt};
use v8_objects_core::tagged::Tagged;
use v8_objects_strings::string::{
  SeqOneByteString, SeqTwoByteString, String as Vstring, StringExt, write_to_flat,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// SeqOneByteString/SeqTwoByteString 尺寸公式 (string-inl.h:1388-1405)。
#[test]
fn seq_size_for_alignment() -> Void {
  // size_for = align4(头 + payload); padding ∈ [0,3]
  for n in [0u32, 1, 2, 3, 4, 5, 7, 8, 13, 100] {
    let d1 = SeqOneByteString::data_size_for(n);
    let s1 = SeqOneByteString::size_for(n);
    assert_eq!(s1 % 4, 0);
    assert_eq!(s1 - d1, (4 - d1 % 4) % 4);
    let d2 = SeqTwoByteString::data_size_for(n);
    let s2 = SeqTwoByteString::size_for(n);
    assert_eq!(s2 % 4, 0);
    assert_eq!(s2 - d2, (4 - d2 % 4) % 4);
    assert_eq!(d2 - d1, n as usize); // 双字节 payload 恰多 n 字节
  }
  // size_for(0) 至少覆盖头 + 对齐块 (clear_padding_destructively 前提)
  assert!(SeqOneByteString::size_for(0) >= 8);
  OK
}

/// Seq 字符读写 (Get/Set/SetChars/GetChars; string-inl.h:1307-1353)。
#[test]
fn seq_chars_read_write() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let s = arena.seq_one_byte(maps.seq_one, b"hello");
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    let seq = s.cast::<SeqOneByteString>();
    assert_eq!(seq.chars(), b"hello");
    assert_eq!(seq.get(0), b'h');
    assert_eq!(seq.get(4), b'o');
    assert_eq!(s.length(), 5);
    seq.set(1, u16::from(b'E'));
    assert_eq!(seq.get(1), b'E');
    seq.set_chars(2, b"LL");
    assert_eq!(seq.chars(), b"hELLo");
    // 越值仅 debug 断言; 正常路径 ≤ 0xFF
    seq.set(0, 0xFF);
    assert_eq!(seq.get(0), 0xFF);
  }
  let t = arena.seq_two_byte(maps.seq_two, &[0x2028, u16::from(b'x'), 0xFFFF]);
  // SAFETY: 同上
  unsafe {
    let seq = t.cast::<SeqTwoByteString>();
    assert_eq!(seq.chars(), &[0x2028, b'x' as u16, 0xFFFF]);
    seq.set(1, 0x1234);
    assert_eq!(seq.get(1), 0x1234);
    assert_eq!(t.length(), 3);
  }
  OK
}

/// GetFlatContent: Seq/展平 Cons/Sliced/Thin 穿透 (string.cc:695-730)。
#[test]
fn flat_content_walk() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let seq = arena.seq_one_byte(maps.seq_one, b"abcdef");
  let seq2 = arena.seq_two_byte(maps.seq_two, &[0x100, 0x200, 0x300, 0x400]);

  // Seq 直取
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    let flat = seq.get_flat_content();
    assert!(flat.is_flat() && flat.is_one_byte());
    assert_eq!(flat.to_one_byte_vector(), b"abcdef");
    assert_eq!(flat.length(), 6);
    assert_eq!(flat.get(0), u16::from(b'a'));
    assert_eq!(flat.get(5), u16::from(b'f'));

    // 展平 Cons → first
    let right = arena.seq_one_byte(maps.seq_one, b"");
    let cons_flat = arena.cons(maps.cons_one, 6, seq, right);
    let flat = cons_flat.get_flat_content();
    assert!(flat.is_one_byte());
    assert_eq!(flat.to_one_byte_vector(), b"abcdef");

    // 非展平 Cons → NonFlat
    let tail = arena.seq_one_byte(maps.seq_one, b"xyz");
    let cons_deep = arena.cons(maps.cons_one, 9, seq, tail);
    assert!(!cons_deep.get_flat_content().is_flat());

    // Sliced → parent + offset
    let sliced = arena.sliced(maps.sliced_one, 3, seq, 2);
    let flat = sliced.get_flat_content();
    assert_eq!(flat.to_one_byte_vector(), b"cde");

    // Thin → actual
    let actual = arena.seq_two_byte(maps.int_two, &[0x100, 0x200, 0x300]);
    let thin = arena.thin(maps.thin_two, 3, actual);
    let flat = thin.get_flat_content();
    assert!(flat.is_two_byte());
    assert_eq!(flat.to_uc16_vector(), &[0x100, 0x200, 0x300]);

    // FlatContent 同源判定 (UsesSameString)
    let a = seq.get_flat_content();
    let b = seq.get_flat_content();
    assert!(a.uses_same_string(&b));
    let c = seq2.get_flat_content();
    assert!(!a.uses_same_string(&c));
  }
  OK
}

/// Get 派发与 IsOneByteRepresentationUnderneath (string-inl.h:529/1149)。
#[test]
fn get_dispatch_and_underneath() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let seq = arena.seq_one_byte(maps.seq_one, b"0123456789");
  let static_u16: &'static [u16] = &[0x41, 0x2028, 0x43];
  let ext = arena.external_two_byte(maps.ext_two, static_u16);
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    // Seq 派发
    assert_eq!(seq.get(3), u16::from(b'3'));
    // External (cached) 派发
    assert_eq!(ext.get(1), 0x2028);
    assert_eq!(ext.get(2), 0x43);
    // Sliced 派发 = parent.get(offset + i)
    let sliced = arena.sliced(maps.sliced_one, 4, seq, 6);
    assert_eq!(sliced.get(0), u16::from(b'6'));
    assert_eq!(sliced.get(3), u16::from(b'9'));
    // Thin 派发 = actual.get
    let thin = arena.thin(maps.thin_one, 10, seq);
    assert_eq!(thin.get(0), u16::from(b'0'));
    // Cons 派发: 左叶/右叶定位 (string.cc:2135)
    let left = arena.seq_one_byte(maps.seq_one, b"ab");
    let right = arena.seq_one_byte(maps.seq_one, b"cde");
    let cons = arena.cons(maps.cons_one, 5, left, right);
    assert_eq!(cons.get(0), u16::from(b'a'));
    assert_eq!(cons.get(1), u16::from(b'b'));
    assert_eq!(cons.get(2), u16::from(b'c'));
    assert_eq!(cons.get(4), u16::from(b'e'));

    // Underneath: 直接串 = 自身编码位
    assert!(seq.is_one_byte_representation_underneath());
    // 双字节 Sliced 的 underneath 双字节
    let seq2 = arena.seq_two_byte(maps.seq_two, &[0x111, 0x222]);
    let sliced2 = arena.sliced(maps.sliced_two, 1, seq2, 1);
    assert!(!sliced2.is_one_byte_representation_underneath());
    // 单字节 Thin 转发到内部化单字节
    let actual = arena.seq_one_byte(maps.int_one, b"zz");
    let thin1 = arena.thin(maps.thin_one, 2, actual);
    assert!(thin1.is_one_byte_representation_underneath());
  }
  OK
}

/// try_flat_content_of_direct 越界契约 + 间接串 NonFlat (string-inl.h:1002)。
#[test]
fn direct_flat_content_bounds() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let seq = arena.seq_one_byte(maps.seq_one, b"abcdef");
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    let flat = <Tagged<Vstring> as StringExt>::try_flat_content_of_direct(seq, 2, 3);
    assert_eq!(flat.to_one_byte_vector(), b"cde");
    // 间接串直接取 → NonFlat
    let sliced = arena.sliced(maps.sliced_one, 3, seq, 1);
    let flat = <Tagged<Vstring> as StringExt>::try_flat_content_of_direct(sliced, 0, 3);
    assert!(!flat.is_flat());
  }
  OK
}

/// 多层切片嵌套与偏移累积 (SlicedString 递归解嵌套与 WriteToFlat/Get 语义)。
#[test]
fn nested_sliced_string_offset_accumulation() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let seq = arena.seq_one_byte(maps.seq_one, b"0123456789ABCDEF");

  // 3 层切片嵌套:
  // s1 = seq[2..14) -> "23456789ABCD" (len 12)
  // s2 = s1[3..10)  -> "56789AB"      (len 7)
  // s3 = s2[1..5)   -> "6789"         (len 4)
  let s1 = arena.sliced(maps.sliced_one, 12, seq, 2);
  let s2 = arena.sliced(maps.sliced_one, 7, s1, 3);
  let s3 = arena.sliced(maps.sliced_one, 4, s2, 1);

  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    assert_eq!(s3.length(), 4);
    assert_eq!(s3.get(0), u16::from(b'6'));
    assert_eq!(s3.get(1), u16::from(b'7'));
    assert_eq!(s3.get(2), u16::from(b'8'));
    assert_eq!(s3.get(3), u16::from(b'9'));

    let mut buf = vec![0u8; 4];
    write_to_flat(s3, &mut buf, 0, 4).ok();
    assert_eq!(&buf, b"6789");

    let target = arena.seq_one_byte(maps.seq_one, b"6789");
    assert!(s3.equals(target));
  }
  OK
}
