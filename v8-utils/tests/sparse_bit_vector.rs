// 迁移自 v8/test/unittests/utils/sparse-bit-vector-unittest.cc
// 断言规约: TEST_F(SparseBitVectorTest, X) → fn x; EXPECT_THAT(ElementsAre) →
// collect() == vec![]; kMaxInt → usize::MAX。
use aok::{OK, Void};
use v8_utils::sparse_bit_vector::SparseBitVector;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 构造 + 收集 (上游 SparseBitVectorBuilder::ToStdVector)
fn vector_of(values: &[usize]) -> Vec<usize> {
  let mut v = SparseBitVector::new();
  for &i in values {
    v.add(i);
  }
  v.iter().collect()
}

// ConstructionAndIteration
#[test]
fn construction_and_iteration() -> Void {
  assert_eq!(vector_of(&[0, 2, 4]), vec![0, 2, 4]);
  assert_eq!(
    vector_of(&[2000, 8000, 6000, 10000]),
    vec![2000, 6000, 8000, 10000]
  );
  assert_eq!(vector_of(&[0, 2, 2, 0, 4, 2, 4]), vec![0, 2, 4]);
  assert_eq!(
    vector_of(&[7, 15, 31, 63, 127, 255]),
    vec![7, 15, 31, 63, 127, 255]
  );
  assert_eq!(
    vector_of(&[255, 127, 63, 31, 15, 7]),
    vec![7, 15, 31, 63, 127, 255]
  );
  OK
}

// Contains
#[test]
fn contains() -> Void {
  let mut v = SparseBitVector::new();
  for &i in &[0, 2, 4] {
    v.add(i);
  }
  assert!(v.contains(0));
  assert!(!v.contains(1));
  assert!(v.contains(2));
  assert!(!v.contains(3));
  assert!(v.contains(4));

  let mut big = SparseBitVector::new();
  for &i in &[2000, 8000, 6000, 10000] {
    big.add(i);
  }
  assert!(big.contains(6000));
  OK
}

// Remove
#[test]
fn remove() -> Void {
  // (add..., remove r) → 期望迭代序列
  let cases: &[(&[usize], usize, Vec<usize>)] = &[
    (&[0, 2, 4], 0, vec![2, 4]),
    (&[0, 2, 4], 1, vec![0, 2, 4]),
    (&[0, 2, 4], 2, vec![0, 4]),
    (&[0, 2, 4], 3, vec![0, 2, 4]),
    (&[0, 2, 4], 4, vec![0, 2]),
    (&[2000, 8000, 6000], usize::MAX, vec![2000, 6000, 8000]),
    (&[2000, 8000, 6000], 8000, vec![2000, 6000]),
    (&[2000, 8000, 6000], 2000, vec![6000, 8000]),
  ];
  for &(adds, r, ref expected) in cases {
    let mut v = SparseBitVector::new();
    for &i in adds {
      v.add(i);
    }
    v.remove(r);
    assert_eq!(
      &v.iter().collect::<Vec<_>>(),
      expected,
      "remove {r} from {adds:?}"
    );
  }
  OK
}

// 补充: Union (上游无单测, 关键路径补测)
#[test]
fn union() -> Void {
  let mut a = SparseBitVector::new();
  a.add(0);
  a.add(7000); // 跨段
  let mut b = SparseBitVector::new();
  b.add(6000);
  b.add(20000);
  a.union(&b);
  assert_eq!(a.iter().collect::<Vec<_>>(), vec![0, 6000, 7000, 20000]);

  // 同位段合并
  let mut c = SparseBitVector::new();
  c.add(6000);
  c.add(7000);
  let mut d = SparseBitVector::new();
  d.add(6001);
  d.add(7001);
  c.union(&d);
  assert_eq!(c.iter().collect::<Vec<_>>(), vec![6000, 6001, 7000, 7001]);
  OK
}
