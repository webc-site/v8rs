//! 相等比较规约 (string-inl.h:680-820 + string.cc:1304 + cctest:1919 StringEquals)。

use v8_strings::hash_seed::HashSeedData;
pub mod common;

use aok::{OK, Void};
use common::{Arena, Maps};
use v8_objects_strings::string::{EqualityType, StringExt};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// StringEquals (cctest:1919): 同内容跨形态相等 / 异内容不等。
#[test]
fn equals_across_shapes() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let text = b"the quick brown fox";
  let seq1 = arena.seq_one_byte(maps.seq_one, text);
  let seq1_b = arena.seq_one_byte(maps.seq_one, text);
  // 双字节但内容同 (全 Latin-1)
  let units: Vec<u16> = text.iter().map(|&c| u16::from(c)).collect();
  let seq2 = arena.seq_two_byte(maps.seq_two, &units);
  // Cons: "the quick " + "brown fox"
  let left = arena.seq_one_byte(maps.seq_one, b"the quick ");
  let right = arena.seq_one_byte(maps.seq_one, b"brown fox");
  let cons = arena.cons(maps.cons_one, 19, left, right);
  // Sliced: parent 多垫前后缀
  let padded = arena.seq_one_byte(maps.seq_one, b"[[the quick brown fox]]");
  let sliced = arena.sliced(maps.sliced_one, 19, padded, 2);
  // Thin 转发
  let thin = arena.thin(maps.thin_one, 19, seq1_b);

  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    for other in [
      seq1_b.cast(),
      seq2.cast(),
      cons.cast(),
      sliced.cast(),
      thin.cast(),
    ] {
      assert!(seq1.equals(other), "seq1 vs ptr={:#x}", other.ptr());
      assert!(other.equals(seq1.cast()));
    }
    // 指针同一 → 真
    assert!(seq1.equals(seq1.cast()));
    // 长度不同 → 假
    let short = arena.seq_one_byte(maps.seq_one, b"the quick brown fo");
    assert!(!seq1.equals(short.cast()));
    // 首字符不同 → 假
    let diff_first = arena.seq_one_byte(maps.seq_one, b"uhe quick brown fox");
    assert!(!seq1.equals(diff_first.cast()));
    // 空串
    let e1 = arena.seq_one_byte(maps.seq_one, b"");
    let e2 = arena.seq_one_byte(maps.seq_one, b"");
    assert!(e1.equals(e2.cast()));
    assert!(!e1.equals(seq1.cast()));
    // 哈希快否: 预算两个不同哈希的等长串
    let h1 = arena.seq_one_byte(maps.seq_one, b"aaaa");
    let h2 = arena.seq_one_byte(maps.seq_one, b"aaab");
    h1.compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
      .ok();
    h2.compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
      .ok();
    assert_ne!(h1.try_get_hash(), h2.try_get_hash());
    assert!(!h1.equals(h2.cast()));
  }
  OK
}

/// 双内部化不同指针 → false 快路径 (string-inl.h:680: 内部化唯一性不变量)。
#[test]
fn equals_internalized_fast_false() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let a = arena.seq_one_byte(maps.int_one, b"same");
  let b = arena.seq_one_byte(maps.int_one, b"same");
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    assert!(!a.equals(b.cast()), "不同指针的双内部化按不变量必不等");
    assert!(a.equals(a.cast()));
  }
  OK
}

/// IsEqualTo (string-inl.h:698-820): 全串/前缀/免检 三型 × u8/u16。
#[test]
fn is_equal_to_modes() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let seq = arena.seq_one_byte(maps.seq_one, b"abcdef");
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    // u8 全串
    assert!(seq.is_equal_to_u8(b"abcdef", EqualityType::WholeString));
    assert!(!seq.is_equal_to_u8(b"abcdeX", EqualityType::WholeString));
    assert!(!seq.is_equal_to_u8(b"abcdef_", EqualityType::WholeString));
    // u8 前缀
    assert!(seq.is_equal_to_u8(b"abc", EqualityType::Prefix));
    assert!(seq.is_equal_to_u8(b"", EqualityType::Prefix));
    assert!(!seq.is_equal_to_u8(b"abd", EqualityType::Prefix));
    assert!(!seq.is_equal_to_u8(b"abcdefg", EqualityType::Prefix));
    // u8 免检
    assert!(seq.is_equal_to_u8(b"abcdef", EqualityType::NoLengthCheck));
    // u16 (交叉宽度)
    let units = "abcdef".encode_utf16().collect::<Vec<u16>>();
    assert!(seq.is_equal_to_u16(&units, EqualityType::WholeString));
    let bad = "abcdeX".encode_utf16().collect::<Vec<u16>>();
    assert!(!seq.is_equal_to_u16(&bad, EqualityType::WholeString));

    // Cons 路径 (离线迭代器)
    let l = arena.seq_one_byte(maps.seq_one, b"ab");
    let r = arena.seq_one_byte(maps.seq_one, b"cdef");
    let cons = arena.cons(maps.cons_one, 6, l, r);
    assert!(cons.is_equal_to_u8(b"abcdef", EqualityType::WholeString));
    assert!(cons.is_equal_to_u8(b"abcd", EqualityType::Prefix));
    assert!(!cons.is_equal_to_u8(b"abcdeg", EqualityType::WholeString));
    assert!(cons.is_equal_to_u16(&units, EqualityType::WholeString));

    // Sliced 路径 (偏移累积)
    let padded = arena.seq_one_byte(maps.seq_one, b"[abcdef]");
    let sliced = arena.sliced(maps.sliced_one, 6, padded, 1);
    assert!(sliced.is_equal_to_u8(b"abcdef", EqualityType::WholeString));
    assert!(sliced.is_equal_to_u8(b"abcd", EqualityType::Prefix));

    // Thin 路径
    let thin = arena.thin(maps.thin_one, 6, cons);
    assert!(thin.is_equal_to_u8(b"abcdef", EqualityType::WholeString));

    // 双字节串 vs u8 数据 (交叉宽度)
    let units2 = [0x2028u16, b'x' as u16, b'y' as u16];
    let seq2 = arena.seq_two_byte(maps.seq_two, &units2);
    assert!(!seq2.is_equal_to_u8(b"xy", EqualityType::Prefix));
  }
  OK
}

/// External 串相等 (cached resource_data 路径)。
#[test]
fn equals_external() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let data: &'static [u8] = b"external payload";
  let ext = arena.external_one_byte(maps.ext_one, data);
  let seq = arena.seq_one_byte(maps.seq_one, data);
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    assert!(ext.is_equal_to_u8(data, EqualityType::WholeString));
    assert!(ext.equals(seq.cast()));
    assert!(seq.equals(ext.cast()));
    let data2: &'static [u16] = &[0x41, 0x42];
    let ext2 = arena.external_two_byte(maps.ext_two, data2);
    assert!(ext2.is_equal_to_u16(data2, EqualityType::WholeString));
    assert!(!ext2.equals(ext.cast()));
  }
  OK
}
