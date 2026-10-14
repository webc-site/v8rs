//! v8-base/src/vector.rs 集成测试 — 对应 vector-unittest.cc
//! 用例名一一对应原版 TEST(Vector, X): Factories / Equals / ConstexprFactories /
//! SpanConversion / SpanCompatibleMethods / ArrayConversion。
//! 原版 consteval/constexpr 断言 (ConstexprFactories) 在 Rust 以运行时等值断言承接。

use aok::{OK, Void};
use v8_base::vector::{
  ConstVector, Vector, array_vector, cstr_vector, hash_value, one_byte_vector, static_char_vector,
  static_one_byte_vector, str_vector, vector_of,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn factories() -> Void {
  // StaticCharVector("foo") → 去尾 \0, 长 3
  let v = static_char_vector(b"foo\0");
  assert_eq!(v.size(), 3);
  assert_eq!(v.as_slice(), b"foo");
  // CStrVector: strlen 语义
  let v = cstr_vector(b"foo\0bar");
  assert_eq!(v.as_slice(), b"foo");
  // StrVector(string_view)
  let v = str_vector("foo");
  assert_eq!(v.as_slice(), b"foo");
  // OneByteVector(data, length)
  let v = one_byte_vector(b"foobar");
  assert_eq!(v.size(), 6);
  // StaticOneByteVector
  let v = static_one_byte_vector(b"foo\0");
  assert_eq!(v.as_slice(), b"foo");
  // ArrayVector("foo") → 含 \0, 长 4
  let v = array_vector(b"foo\0");
  assert_eq!(v.size(), 4);
  // VectorOf(start, size) 读写形态
  let mut data = [1i32, 2, 3];
  let v = vector_of(&mut data);
  assert_eq!(v.size(), 3);
  OK
}

#[test]
fn equals() -> Void {
  let a = [1u8, 2, 3];
  let b = [1u8, 2, 3];
  let c = [1u8, 2];
  let va = ConstVector::new(&a);
  let vb = ConstVector::new(&b);
  let vc = ConstVector::new(&c);
  let empty: ConstVector<u8> = ConstVector::new(&[]);
  assert!(va == vb);
  assert!(va != vc);
  assert!(empty == ConstVector::new(&[]));
  // Vector<T> 与 Vector<const T> 交叉判等 (原版 operator== span 语义)
  let mut wa = [1u8, 2, 3];
  let mut rw = Vector::new(&mut wa);
  assert!(rw == va);
  assert!(va == rw);
  rw.as_mut_slice()[0] = 9;
  assert!(rw != vb);
  OK
}

#[test]
fn constexpr_factories() -> Void {
  // 默认构造空视图 (原版 constexpr Vector() = default)
  let v = Vector::<u32>::default();
  assert!(v.is_empty());
  assert_eq!(v.size(), 0);
  let cv = ConstVector::<u32>::default();
  assert!(cv.is_empty());
  // 空 default 可安全只读遍历
  assert_eq!(cv.iter().count(), 0);
  OK
}

#[test]
fn span_conversion() -> Void {
  let mut data = [1i32, 2, 3, 4];
  {
    let v = Vector::new(&mut data);
    let cv: ConstVector<'_, i32> = v.into_const();
    assert_eq!(cv.as_slice(), &[1, 2, 3, 4]);
  }
  let mut data2 = [5i32, 6, 7];
  {
    let v = Vector::new(&mut data2);
    let cv = v.as_const();
    assert_eq!(cv.size(), 3);
  }
  // SubVector / first / last (原版 span 成员形态)
  let cv = ConstVector::new(&[1i32, 2, 3, 4, 5]);
  assert_eq!(cv.sub_vector(1, 4).as_slice(), &[2, 3, 4]);
  assert_eq!(cv.sub_vector_from(2).as_slice(), &[3, 4, 5]);
  assert_eq!(cv.first(2).as_slice(), &[1, 2]);
  assert_eq!(cv.last(3).as_slice(), &[3, 4, 5]);
  let mut data3 = [1i32, 2, 3];
  let v = Vector::new(&mut data3).sub_vector(0, 2);
  assert_eq!(v.as_slice(), &[1, 2]);
  // truncate / offset
  let mut data4 = [1i32, 2, 3, 4];
  let mut v = Vector::new(&mut data4);
  v.truncate(2);
  assert_eq!(v.as_slice(), &[1, 2]);
  let v = Vector::new(&mut data4).offset(2);
  assert_eq!(v.as_slice(), &[3, 4]);
  // OverwriteWith (等长 memcpy)
  let mut dst = [0i32; 2];
  let mut dv = Vector::new(&mut dst);
  dv.overwrite_with(&[7, 8]);
  assert_eq!(dst, [7, 8]);
  OK
}

#[test]
fn span_compatible_methods() -> Void {
  let data = [1i32, 2, 3];
  let cv = ConstVector::new(&data);
  assert_eq!(cv.size(), 3);
  assert_eq!(cv.length(), 3);
  assert!(!cv.is_empty());
  assert_eq!(cv.at(0), Some(&1));
  assert_eq!(cv.at(3), None); // 原版 DCHECK 路径以 Option 承接
  assert_eq!(cv.front(), Some(&1));
  assert_eq!(cv.back(), Some(&3));
  assert_eq!(cv.data(), data.as_ptr());
  assert_eq!(cv.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
  assert_eq!(cv.riter().copied().collect::<Vec<_>>(), vec![3, 2, 1]);
  // 读写视图同集合访问器
  let mut m = [1i32, 2, 3];
  let m_ptr = m.as_ptr();
  let mv = Vector::new(&mut m);
  assert_eq!(mv.at_ref(2), Some(&3));
  assert_eq!(mv.get_unchecked(1), &2);
  assert_eq!(mv.cbegin() as usize, m_ptr as usize);
  assert_eq!(mv.length(), 3);
  // subspan 与 copy_from
  let mut a = [0i32; 3];
  let mut b = [4i32, 5, 6];
  {
    let mut av = Vector::new(&mut a);
    let bv = ConstVector::new(&b);
    av.copy_from(bv);
  }
  assert_eq!(a.as_slice(), &[4, 5, 6]);
  let mut c = [0i32; 2];
  let mut cv = Vector::new(&mut c);
  cv.copy_from_array(&[9, 9]);
  assert_eq!(c, [9, 9]);
  let v = Vector::new(&mut b).subspan(1, 2);
  assert_eq!(v.as_slice(), &[5, 6]);
  OK
}

#[test]
fn array_conversion() -> Void {
  // Vector<u32> → Vector<u16> 字节重解释 (原版 cast; 对齐与 trivial 前提满足)
  let mut data = [0x1122_3344u32, 0x5566_7788];
  let v = Vector::new(&mut data).cast::<u16>();
  assert_eq!(v.size(), 4);
  // 小端: 低半字先出
  assert_eq!(v.at(0), Some(&0x3344));
  assert_eq!(v.at(2), Some(&0x7788));
  assert_eq!(v.at(3), Some(&0x5566));
  // const 侧 cast
  let cv = ConstVector::new(&data).cast::<u16>();
  assert_eq!(cv.size(), 4);
  // ArrayVector 定长数组入口已在 factories 覆盖
  OK
}

#[test]
fn hash_value_matches_elements() -> Void {
  // hash_value(Vector<const T>) = hash_range 逐元素
  let a = ConstVector::new(&[1i32, 2, 3]);
  let b = ConstVector::new(&[1i32, 2, 3]);
  let c = ConstVector::new(&[3i32, 2, 1]);
  assert_eq!(hash_value(a), hash_value(b));
  assert_ne!(hash_value(a), hash_value(c));
  OK
}
