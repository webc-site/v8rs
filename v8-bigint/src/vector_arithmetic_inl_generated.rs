// 由 tool/src/bigint/vector-arithmetic-inl.h.js 从 v8/src/bigint/vector-arithmetic-inl.h 生成; 修订就地改, 复跑覆盖
// 位向量内联运算骨架

/// 位向量内联运算 (原版 vector-arithmetic-inl.h, 语义实现见手写 vector_arithmetic.rs)
pub mod vector_arithmetic_inl_skel {
  /// Z += X, 溢出返回 1 否则 0 (原版 AddAndReturnOverflow; 先逐位 digit_add2 后补进位)
  pub fn add_and_return_overflow(_z: &mut [u64], _x: &[u64]) -> u64 {
    unimplemented!("骨架")
  }

  /// Z -= X, 借位溢出返回借位值 (原版 SubAndReturnBorrow)
  pub fn sub_and_return_borrow(_z: &mut [u64], _x: &[u64]) -> u64 {
    unimplemented!("骨架")
  }

  /// 最高位是否恰在最高位上 (原版 IsBitNormalized: msd >> (kDigitBits-1) == 1)
  pub fn is_bit_normalized(_x: &[u64]) -> bool {
    unimplemented!("骨架")
  }

  /// 有效位长 = len*kDigitBits - clz(msd) (原版 BitLength)
  pub fn bit_length(_x: &[u64]) -> u32 {
    unimplemented!("骨架")
  }

  /// 内联函数总数对数守卫
  pub const INLINE_FNS: [&str; 4] = ["AddAndReturnOverflow", "SubAndReturnBorrow", "IsBitNormalized", "BitLength"];
}
