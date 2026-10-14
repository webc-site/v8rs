// 迁移自 v8/test/unittests/utils/bit-vector-unittest.cc
// 断言规约: TEST_F(BitVectorTest, X) → fn x; EXPECT_* → assert*; 迭代器逐位断言
// 1:1; Zone 参数在 Rust 版中移除 (Vec 承载), 断言强度守恒。
use aok::{OK, Void};
use v8_utils::bit_vector::{BitVector, GrowableBitVector};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// SmallBitVector
#[test]
fn small_bit_vector() -> Void {
  let mut v = BitVector::new(15);
  v.add(1);
  assert!(v.contains(1));
  v.remove(0);
  assert!(!v.contains(0));
  v.add(0);
  v.add(1);
  let mut w = BitVector::new(15);
  w.add(1);
  v.intersect(&w);
  assert!(!v.contains(0));
  assert!(v.contains(1));
  OK
}

// SmallBitVectorIterator: 正反迭代器逐位推进
#[test]
fn small_bit_vector_iterator() -> Void {
  let mut v = BitVector::new(64);
  v.add(27);
  v.add(30);
  v.add(31);
  v.add(33);

  let mut iter = v.iter();
  assert_eq!(iter.next(), Some(27));
  assert_eq!(iter.next(), Some(30));
  assert_eq!(iter.next(), Some(31));
  assert_eq!(iter.next(), Some(33));
  assert_eq!(iter.next(), None);

  let mut riter = v.iter_rev();
  assert_eq!(riter.next(), Some(33));
  assert_eq!(riter.next(), Some(31));
  assert_eq!(riter.next(), Some(30));
  assert_eq!(riter.next(), Some(27));
  assert_eq!(riter.next(), None);
  OK
}

// Union
#[test]
fn union() -> Void {
  let mut v = BitVector::new(15);
  v.add(0);
  let mut w = BitVector::new(15);
  w.add(1);
  v.union(&w);
  assert!(v.contains(0));
  assert!(v.contains(1));
  OK
}

// CopyFrom
#[test]
fn copy_from() -> Void {
  let mut v = BitVector::new(15);
  v.add(0);
  let mut w = BitVector::new(15);
  w.copy_from(&v);
  assert!(w.contains(0));
  w.add(1);
  let u = BitVector::clone_from(&w);
  assert!(u.contains(0));
  assert!(u.contains(1));
  v.union(&w);
  assert!(v.contains(0));
  assert!(v.contains(1));
  OK
}

// Union2: 跨字 (bit 33)
#[test]
fn union2() -> Void {
  let mut v = BitVector::new(35);
  v.add(0);
  let mut w = BitVector::new(35);
  w.add(33);
  v.union(&w);
  assert!(v.contains(0));
  assert!(v.contains(33));
  OK
}

// Intersect
#[test]
fn intersect() -> Void {
  let mut v = BitVector::new(35);
  v.add(32);
  v.add(33);
  let mut w = BitVector::new(35);
  w.add(33);
  v.intersect(&w);
  assert!(!v.contains(32));
  assert!(v.contains(33));
  let mut r = BitVector::new(35);
  r.copy_from(&v);
  assert!(!r.contains(32));
  assert!(r.contains(33));
  OK
}

// Resize
#[test]
fn resize() -> Void {
  let mut v = BitVector::new(35);
  v.add(32);
  v.add(33);
  assert!(v.contains(32));
  assert!(v.contains(33));
  assert!(!v.contains(22));
  assert!(!v.contains(34));
  v.resize(50);
  assert!(v.contains(32));
  assert!(v.contains(33));
  assert!(!v.contains(22));
  assert!(!v.contains(34));
  assert!(!v.contains(43));
  v.resize(300);
  assert!(v.contains(32));
  assert!(v.contains(33));
  assert!(!v.contains(22));
  assert!(!v.contains(34));
  assert!(!v.contains(43));
  assert!(!v.contains(243));
  OK
}

// BigBitVectorIterator: 大向量正反迭代 + 扩容后重迭代
#[test]
fn big_bit_vector_iterator() -> Void {
  let mut v = BitVector::new(500);
  v.add(27);
  v.add(300);
  v.add(499);
  let mut iter = v.iter();
  assert_eq!(iter.next(), Some(27));
  assert_eq!(iter.next(), Some(300));
  assert_eq!(iter.next(), Some(499));
  assert_eq!(iter.next(), None);

  let mut riter = v.iter_rev();
  assert_eq!(riter.next(), Some(499));
  assert_eq!(riter.next(), Some(300));
  assert_eq!(riter.next(), Some(27));
  assert_eq!(riter.next(), None);

  // Remove small entries, add another big one.
  v.resize(1000);
  v.remove(27);
  v.remove(300);
  v.add(500);
  let mut iter = v.iter();
  assert_eq!(iter.next(), Some(499));
  assert_eq!(iter.next(), Some(500));
  assert_eq!(iter.next(), None);

  let mut riter = v.iter_rev();
  assert_eq!(riter.next(), Some(500));
  assert_eq!(riter.next(), Some(499));
  assert_eq!(riter.next(), None);
  OK
}

// MoveConstructorInline: move 后数据完整, 新向量不影响 (Rust move 语义天然保证)
#[test]
fn move_constructor_inline() -> Void {
  let mut v = BitVector::new(30);
  v.add(12);
  v.add(29);
  assert!(v.contains(12));
  assert!(v.contains(29));
  assert!(!v.contains(22));
  assert!(!v.contains(28));
  let a = v; // move (上游 std::move 构造)
  assert!(a.contains(12));
  assert!(a.contains(29));
  assert!(!a.contains(22));
  assert!(!a.contains(28));
  // 上游: v 重新赋值后不得影响 a
  let mut v2 = BitVector::new(31);
  v2.add(22);
  v2.add(28);
  assert!(a.contains(12));
  assert!(a.contains(29));
  assert!(!a.contains(22));
  assert!(!a.contains(28));
  OK
}

// MoveAssignInline
#[test]
fn move_assign_inline() -> Void {
  let mut v = BitVector::new(30);
  v.add(12);
  v.add(29);
  assert!(v.contains(12));
  assert!(v.contains(29));
  assert!(!v.contains(22));
  assert!(!v.contains(28));
  let a = v; // move 赋值 (上游 a = std::move(v), 默认构造被优化)
  assert!(a.contains(12));
  assert!(a.contains(29));
  assert!(!a.contains(22));
  assert!(!a.contains(28));
  let mut v2 = BitVector::new(31);
  v2.add(22);
  v2.add(28);
  assert!(a.contains(12));
  assert!(a.contains(29));
  assert!(!a.contains(22));
  assert!(!a.contains(28));
  OK
}

// MoveConstructorLarge: 跨字 (bit 133)
#[test]
fn move_constructor_large() -> Void {
  let mut v = BitVector::new(200);
  v.add(31);
  v.add(133);
  assert!(v.contains(31));
  assert!(v.contains(133));
  assert!(!v.contains(22));
  assert!(!v.contains(134));
  let a = v;
  assert!(a.contains(31));
  assert!(a.contains(133));
  assert!(!a.contains(22));
  assert!(!a.contains(134));
  let mut v2 = BitVector::new(205);
  v2.add(22);
  v2.add(134);
  assert!(a.contains(31));
  assert!(a.contains(133));
  assert!(!a.contains(22));
  assert!(!a.contains(134));
  OK
}

// MoveAssignLarge
#[test]
fn move_assign_large() -> Void {
  let mut v = BitVector::new(200);
  v.add(31);
  v.add(133);
  assert!(v.contains(31));
  assert!(v.contains(133));
  assert!(!v.contains(22));
  assert!(!v.contains(134));
  let a = v; // move 赋值
  assert!(a.contains(31));
  assert!(a.contains(133));
  assert!(!a.contains(22));
  assert!(!a.contains(134));
  let mut v2 = BitVector::new(205);
  v2.add(22);
  v2.add(134);
  assert!(a.contains(31));
  assert!(a.contains(133));
  assert!(!a.contains(22));
  assert!(!a.contains(134));
  OK
}

// BoundedAddRemoveAndCopyFrom: 区间操作全组合 (start × count)
#[test]
fn bounded_add_remove_and_copy_from() -> Void {
  const LENGTH: usize = 250;
  const STARTS: [usize; 11] = [0, 1, 7, 31, 32, 63, 64, 65, 127, 128, 190];
  const COUNTS: [usize; 12] = [0, 1, 2, 5, 31, 32, 33, 63, 64, 65, 128, 135];

  for &start in &STARTS {
    for &count in &COUNTS {
      if start + count > LENGTH {
        continue;
      }

      // Add(start, count)
      let mut added = BitVector::new(LENGTH);
      added.add_range(start, count);
      for i in 0..LENGTH {
        assert_eq!(i >= start && i < start + count, added.contains(i));
      }

      // AddAll + Remove(start, count)
      let mut removed = BitVector::new(LENGTH);
      removed.add_all();
      removed.remove_range(start, count);
      for i in 0..LENGTH {
        assert_eq!(i < start || i >= start + count, removed.contains(i));
      }

      // CopyFrom(src, start, count): 全 1 / 全 0 两种底
      let mut src = BitVector::new(LENGTH);
      for i in 0..LENGTH {
        if (i * 7 + 3) % 5 < 2 {
          src.add(i);
        }
      }

      let mut dst_ones = BitVector::new(LENGTH);
      dst_ones.add_all();
      dst_ones.copy_from_range(&src, start, count);

      let mut dst_zeros = BitVector::new(LENGTH);
      dst_zeros.copy_from_range(&src, start, count);

      for i in 0..LENGTH {
        if i >= start && i < start + count {
          assert_eq!(src.contains(i), dst_ones.contains(i));
          assert_eq!(src.contains(i), dst_zeros.contains(i));
        } else {
          assert!(dst_ones.contains(i));
          assert!(!dst_zeros.contains(i));
        }
      }
    }
  }
  OK
}

// 补充: GrowableBitVector 扩容路径 (上游无独立单测, 关键路径补测)
#[test]
fn growable_bit_vector_grow() -> Void {
  let mut g = GrowableBitVector::new(0);
  assert!(!g.contains(5000));
  g.add(5000);
  assert!(g.contains(5000));
  assert_eq!(g.length(), 8192); // bit_ceil(5001) → 8192
  g.add(1);
  assert!(g.contains(1));
  let mut other = GrowableBitVector::new(0);
  other.add(9000);
  g.add_all(&other);
  assert!(g.contains(9000));
  assert_eq!(g.iter().collect::<Vec<_>>(), vec![1, 5000, 9000]);
  OK
}

// 补充: CopyFrom(src, index, 0) 零长度退化 — 上游 ForRange count==0 早退,
// 不得触碰任何字 (index 与字边界对齐相等时 word_of(index) 恰在界外)
#[test]
fn copy_from_zero_count_noop() -> Void {
  let mut src = BitVector::new(64);
  src.add(0);
  src.add(63);
  // 字对齐长度: data 恰 1 字, word_of(64) == 1 越界
  let mut dst = BitVector::new(64);
  dst.copy_from_range(&src, 64, 0);
  assert!(!dst.contains(0));
  assert!(!dst.contains(63));
  dst.copy_from_range(&src, 0, 0);
  assert_eq!(dst.count(), 0);
  OK
}
