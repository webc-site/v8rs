// 原版测试 1:1 转写
//! 来源: v8/test/mjsunit/harmony/bigint/and.js (bigint-tester.py 金标准 20 组),
//! v8/test/mjsunit/compiler/bigint-bitwise-and.js (TestBitwiseAnd 符号四组合 + 一 digit 溢出增长)。
//! 覆盖 BitwiseAnd 顶层编排 (src/bitwise.rs): 符号派发 / 预算长度 / PosNeg 换参 / 规格化 / 零短路。

use aok::{OK, Void};
use v8_bigint::{
  bigint::Digit,
  bitwise::{bitwise_and, bitwise_and_owned, bitwise_and_result_length},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 十六进制幅度 → 小端 digit 向量 (剔除高位零 digit, 与 norm 一致)
fn mag(hex: &str) -> Vec<Digit> {
  let mut d = Vec::new();
  let mut end = hex.len();
  while end > 0 {
    let start = end.saturating_sub(16);
    d.push(u64::from_str_radix(&hex[start..end], 16).unwrap());
    end = start;
  }
  while d.last() == Some(&0) {
    d.pop();
  }
  d
}

/// and.js data 数组 20 组, 逐字节照抄 (neg, hex 幅度) 三元组
const AND_VECTORS: [(bool, &str, bool, &str, bool, &str); 20] = [
  (
    false,
    "9252b94f220ded0c18706998886397699c5a25527575d",
    true,
    "286817ba2e8fd8",
    false,
    "9252b94f220ded0c1870699888639769185a045015008",
  ),
  (
    false,
    "15a28152354fe421bbe4649d4d22d536ac34f9ba962",
    false,
    "8",
    false,
    "0",
  ),
  (
    false,
    "d09791bf7ae6703fae8d654104f54fcd957bbf028540b8e7c0f859fdcc1b7abd617e",
    false,
    "24b473d03",
    false,
    "24a052102",
  ),
  (
    false,
    "2f73fbd05e9f7f70beba3dc7cd883732149a3d02a422f90f336",
    false,
    "4490583f8",
    false,
    "9008330",
  ),
  (
    false,
    "d6ea27844b3d356774e73de10d393ab9cadb81848e3b4a34d1f7440d3",
    false,
    "1639eeee731f3ba09234b5e15ec92f8bfb5cea28a0e89c59ed1c45b5f04",
    false,
    "10ea26000b39200234a5214009290ab948ca008088184824104504000",
  ),
  (
    true,
    "131f5e2c196269fd140c8727a9a02170e0dddb6aec109a89bc",
    false,
    "401ec4eae8b756f2b222f6f565fb6acecce3c524e9b453",
    false,
    "12c488800242f230005655448b0a0204810124613440",
  ),
  (
    true,
    "5bcba128bf7b949780a4bd02782a63ae79f646a33b8672f3f692ab98",
    true,
    "a81dd8ff624072fce3a00eb0bd5700e5666c3beb7d59701c42489",
    true,
    "5bcba1fdbfffb497afeebf02fb2bf7fe7ff666e3bfb7f7f7f7d6afa0",
  ),
  (
    false,
    "c997546528786548270061bc",
    false,
    "75023f046c04f9095f64d3b8c75ab21950a",
    false,
    "499014640838044023000108",
  ),
  (
    false,
    "5ad0abd6e43ef787248af2607fb29b9c3f7a1e7b106269ff3b",
    true,
    "d483c8630392db4b620e0f6",
    false,
    "5ad0abd6e43ef787248af2607fb29380394a06520040491f0a",
  ),
  (
    true,
    "9ed448bdb4794e4c0485db9ebe6cad0b0",
    true,
    "549a70753c9af",
    true,
    "9ed448bdb4794e4c0485df9efe7dbd9b0",
  ),
  (
    true,
    "3a3fe42647633612082372714484767efc9673ce964b76f9f53a63d5",
    false,
    "99615e5edd0506cbb5512fb56ee2dd1238b",
    false,
    "986116189800068881100b0000608c1000b",
  ),
  (
    true,
    "ac1b8238c08f5",
    true,
    "b9439dfbbc70e12b3a1fc7ccbfebeacd9bb321d83d3a",
    true,
    "b9439dfbbc70e12b3a1fc7ccbfebeacfdbbb23dc3dfe",
  ),
  (
    false,
    "120580503",
    false,
    "90e13d34618496af407fabefabdd23892f488",
    false,
    "20100400",
  ),
  (false, "aa51c", false, "2122a9c43b2531d5", false, "2114"),
  (
    false,
    "af43eb353c1547dc6ad79768e9a6",
    false,
    "55c80f89d8fe82dbf69eca557dd0479b6a449db77e38b2241ec81aa7d4042766dd3e63",
    false,
    "28022014081007d4000706482822",
  ),
  (
    false,
    "bd38b2d88212ceea6fbe3890769e46236",
    false,
    "eb22e568873e9bfdb5d2ddbc03",
    false,
    "88202468863a83890540c42002",
  ),
  (
    false,
    "24cd8c9634eddf319c57b54dffb49ce4974528f03b2c732d052ad496",
    true,
    "4079cf",
    false,
    "24cd8c9634eddf319c57b54dffb49ce4974528f03b2c732d052a8410",
  ),
  (
    true,
    "61f263476a1212fce55cf7d437f4364b73a1815a641",
    true,
    "19226cb79772cca166043a29d1f0fe1df5",
    true,
    "61f263476b9236fffd7fffde37f477ebffbf8ffbff5",
  ),
  (
    true,
    "44309b08a65bc7dc1e53d598d82ccdaa852941d7c851d907a6ac",
    false,
    "dd93dc17d38658ee30453d5f27e",
    false,
    "5d1304178102408230402d05054",
  ),
  (
    false,
    "1e3eea4668e7a6c3bfa87d3f7a419bd18a",
    true,
    "592d83940f884f9",
    false,
    "1e3eea4668e7a6c3bfa86d274201035102",
  ),
];

/// and.js 主循环: d.a & d.b === d.r, 20 组全断言 (符号 + 幅度)
/// 同时走非自有 API: 按预算长度精确开缓冲, 校验与 owned 版一致
#[test]
fn and_vectors() -> Void {
  for (na, a, nb, b, nr, r) in AND_VECTORS {
    let (x, y, want) = (mag(a), mag(b), mag(r));
    let (sign, got) = bitwise_and_owned(&x, na, &y, nb);
    assert_eq!(sign, nr, "sign: {a} & {b}");
    assert_eq!(got, want, "mag: {a} & {b}");

    let (s2, budget) = bitwise_and_result_length(&x, na, &y, nb);
    let mut z = vec![0; budget as usize];
    let (s3, view) = bitwise_and(&mut z, &x, na, &y, nb);
    assert_eq!(s2, s3);
    assert_eq!(s3, nr, "budget sign: {a} & {b}");
    assert_eq!(view, want.as_slice(), "budget mag: {a} & {b}");
    assert!(view.len() <= budget as usize);
  }
  OK
}

/// compiler/bigint-bitwise-and.js TestBitwiseAnd 断言组 (非优化路径 5 例)
#[test]
fn and_sign_dispatch() -> Void {
  // fn(0b1100n, 0b1010n) == 0b1000n
  assert_eq!(
    bitwise_and_owned(&[0b1100], false, &[0b1010], false),
    (false, vec![0b1000u64])
  );
  // fn(-0b100n, -0b110n) == -0b1000n
  assert_eq!(
    bitwise_and_owned(&[0b100], true, &[0b110], true),
    (true, vec![0b1000u64])
  );
  // fn(-0b100n, 0b1010n) == 0b1000n (负正分支换参)
  assert_eq!(
    bitwise_and_owned(&[0b100], true, &[0b1010], false),
    (false, vec![0b1000u64])
  );
  // fn(0b1100n, -0b110n) == 0b1000n
  assert_eq!(
    bitwise_and_owned(&[0b1100], false, &[0b110], true),
    (false, vec![0b1000u64])
  );
  OK
}

/// compiler/bigint-bitwise-and.js: 结果长出一 digit — -(2n**63n + 1n) & -(2n**63n) == -(2n**64n)
#[test]
fn and_grow_out_of_digit() -> Void {
  let x = [1u64 + (1 << 63)]; // 2^63 + 1 (单 digit)
  let y = [1u64 << 63]; // 2^63 (单 digit)
  let (neg, mag) = bitwise_and_owned(&x, true, &y, true);
  assert!(neg);
  assert_eq!(mag, vec![0u64, 1]); // 2^64, 预算 max(1,1)+1 = 2 digit 恰够用
  assert_eq!(bitwise_and_result_length(&x, true, &y, true), (true, 2));
  OK
}

/// tq 零短路: xlength == 0 → 返回零; ylength == 0 → 返回零
#[test]
fn and_zero_shortcut() -> Void {
  for (x, nx, y, ny) in [
    (&[][..], false, &[5][..], true),
    (&[7][..], false, &[][..], false),
    (&[][..], false, &[][..], false),
  ] {
    let (neg, mag) = bitwise_and_owned(x, nx, y, ny);
    assert!(!neg);
    assert!(mag.is_empty());
    assert_eq!(bitwise_and_result_length(x, nx, y, ny), (false, 0));
  }
  OK
}

/// 全溢出位: 负数补码高位全 1 不进正侧高位之外
/// (2^64-1) & -2^64 == 0; (2^66-1) & -2^64 == 3*2^64
#[test]
fn and_full_overflow_bits() -> Void {
  assert_eq!(
    bitwise_and_owned(&[Digit::MAX], false, &[0, 1], true),
    (false, vec![])
  );
  assert_eq!(
    bitwise_and_owned(&[Digit::MAX, 3], false, &[0, 1], true),
    (false, vec![0u64, 3])
  );
  // -(2^64-1) & -(2^64-1) == -(2^64-1) (两补码全 F 形态相交仍为自身)
  assert_eq!(
    bitwise_and_owned(&[Digit::MAX], true, &[Digit::MAX], true),
    (true, vec![Digit::MAX])
  );
  // x & -1n == x (正 x, 负 y 幅度 1)
  assert_eq!(
    bitwise_and_owned(&[42, 9], false, &[1], true),
    (false, vec![42u64, 9])
  );
  OK
}

/// 预算公式四分支逐项: pospos=min, negneg=max+1, posneg=正侧长度
#[test]
fn and_result_length_budget() -> Void {
  let (a, b) = ([1u64, 2, 3], [4u64, 5]);
  assert_eq!(bitwise_and_result_length(&a, false, &b, false), (false, 2));
  assert_eq!(bitwise_and_result_length(&a, true, &b, true), (true, 4));
  assert_eq!(bitwise_and_result_length(&a, false, &b, true), (false, 3));
  assert_eq!(bitwise_and_result_length(&a, true, &b, false), (false, 2));
  // 非规范输入 (带尾零) 预算按 norm 后长度计
  assert_eq!(
    bitwise_and_result_length(&[1, 2, 3, 0], false, &[4, 5, 0], false),
    (false, 2)
  );
  OK
}
