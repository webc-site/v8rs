//! const 块不变式断言用的字符串/序列 const 比较工具
//! (标准库 str/slice PartialEq 非 const, 此处以字节循环补齐)

/// const 字符串相等
pub(crate) const fn str_eq(a: &str, b: &str) -> bool {
  let (ab, bb) = (a.as_bytes(), b.as_bytes());
  if ab.len() != bb.len() {
    return false;
  }
  let mut i = 0;
  while i < ab.len() {
    if ab[i] != bb[i] {
      return false;
    }
    i += 1;
  }
  true
}

/// const 序列包含
pub(crate) const fn contains(seq: &[&str], s: &str) -> bool {
  let mut i = 0;
  while i < seq.len() {
    if str_eq(seq[i], s) {
      return true;
    }
    i += 1;
  }
  false
}
