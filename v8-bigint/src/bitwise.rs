// 由手工从 v8/src/bigint/bigint.h 位运算声明面 + v8/src/builtins/builtins-bigint.tq BigIntBitwiseAndImpl 转写
//! BitwiseAnd 顶层编排: 符号派发 / 结果长度预算 / PosNeg 换参 / 规格化
//! digit 级内核 BitwiseAnd_PosPos/NegNeg/PosNeg 已在 bigint.rs 1:1 转写 (对应 bigint-inl.h:1146-1187,
//! 声明见 bigint.h:271-275), 本模块只做 objects 层缺失的编排, 不重复内核
//! 本 crate 边界外推迟登记 (无隐式桩, 显式不实现): MutableBigInt_BitwiseAnd*AndCanonicalize 三壳
//! (objects/bigint.cc:1723-1757, Tagged<BigInt>/Factory 硬耦合) 与 AllocateEmptyBigIntNoThrow /
//! BigIntTooBig 抛错面 (tq 壳层, 归 L4 builtins); Canonicalize 面由 norm() 截断视图承担, 不引 heap 依赖

use crate::bigint::{Digit, bitwise_and_neg_neg, bitwise_and_pos_neg, bitwise_and_pos_pos, norm};

/// v8: builtins-bigint.tq:467-489 BigIntBitwiseAndImpl 三处长度公式与四分支符号
/// BitwiseAnd 结果长度预算与结果符号; 返回 (结果是否负, 预算长度)
/// 任一输入为零时按正零返回 (false, 0)
pub fn bitwise_and_result_length(
  x: &[Digit],
  x_negative: bool,
  y: &[Digit],
  y_negative: bool,
) -> (bool, u32) {
  let (xlen, ylen) = (norm(x).len(), norm(y).len());
  // case: 0n & y / x & 0n
  if xlen == 0 || ylen == 0 {
    return (false, 0);
  }
  match (x_negative, y_negative) {
    (false, false) => (false, xlen.min(ylen) as u32),
    (true, true) => (true, xlen.max(ylen) as u32 + 1),
    (false, true) => (false, xlen as u32),
    (true, false) => (false, ylen as u32),
  }
}

/// v8: builtins-bigint.tq:454 BigIntBitwiseAndImpl + objects/bigint.cc:1723-1757 三处内核调用
/// Z := X & Y (sign+magnitude 输入, 负输入隐式转补码, 语义见 bigint.h:267-270 注释块)
/// 返回 (结果是否负, 规格化后的结果视图); 要求 z.len() >= bitwise_and_result_length 的预算
pub fn bitwise_and<'a>(
  z: &'a mut [Digit],
  x: &[Digit],
  x_negative: bool,
  y: &[Digit],
  y_negative: bool,
) -> (bool, &'a [Digit]) {
  let (xn, yn) = (norm(x), norm(y));
  // 任一操作数为零: 原版直接返回零操作数, 此处统一按正零返回
  if xn.is_empty() || yn.is_empty() {
    return (false, &[]);
  }
  let budget = bitwise_and_result_length(x, x_negative, y, y_negative).1;
  debug_assert!(z.len() >= budget as usize);
  match (x_negative, y_negative) {
    (false, false) => bitwise_and_pos_pos(z, xn, yn),
    (true, true) => bitwise_and_neg_neg(z, xn, yn),
    // 正 X 负 Y; X 负 Y 正时按 bigint.h:274 "callers must swap" 注释交换参数
    (false, true) => bitwise_and_pos_neg(z, xn, yn),
    (true, false) => bitwise_and_pos_neg(z, yn, xn),
  }
  // 结果符号仅 NegNeg 为负 (其余分支原版分配时即钉死 kPositiveSign)
  (x_negative && y_negative, norm(z))
}

/// v8: builtins-bigint.tq:454 BigIntBitwiseAndImpl 全函数 (分配面由 Vec 承担)
/// 自分配结果的 BitwiseAnd; 返回 (结果是否负, 规格化幅度)
pub fn bitwise_and_owned(
  x: &[Digit],
  x_negative: bool,
  y: &[Digit],
  y_negative: bool,
) -> (bool, Vec<Digit>) {
  let (negative, len) = bitwise_and_result_length(x, x_negative, y, y_negative);
  let mut z = vec![0; len as usize];
  let (sign, mag) = bitwise_and(&mut z, x, x_negative, y, y_negative);
  debug_assert_eq!(sign, negative);
  (sign, mag.to_vec())
}
