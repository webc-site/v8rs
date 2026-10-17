// 由 tool/src/bigint/fromstring.cc.js 从 v8/src/bigint/fromstring.cc (+bigint-inl.h config) 生成; 修订就地改, 复跑覆盖
// 字符串解析骨架

/// 字符串解析阈值 (原版 bigint-inl.h config; 64 位为当前目标)
pub mod from_string_config {
  /// Classic → Large 门槛 (32 位 165 / 64 位 25)
  pub const FROM_STRING_LARGE_THRESHOLD_32: u32 = 165;
  pub const FROM_STRING_LARGE_THRESHOLD_64: u32 = 25;

  /// 2^k 底数快路径标记 (原版 FromStringBasePowerOfTwo)
  pub const POWER_OF_TWO_FAST_PATH: bool = true;
}

/// 解析入口骨架 (语义见手写 from_string.rs)
pub mod from_string_skel {
  /// 逐步累加经典解析 (原版 FromStringClassic)
  pub fn from_string_classic(_z: &mut [u64], _acc: &mut usize) {
    unimplemented!("骨架")
  }
  /// 超大数分块解析 (原版 FromStringLarge; 结果长度 ≥ LARGE_THRESHOLD)
  pub fn from_string_large(_z: &mut [u64], _acc: &mut usize) {
    unimplemented!("骨架")
  }
  /// 2^k 底数直接按位移重排 (原版 FromStringBasePowerOfTwo)
  pub fn from_string_base_power_of_two(_z: &mut [u64], _acc: &mut usize) {
    unimplemented!("骨架")
  }
  /// 总分派 (原版 FromString; 底数是否 2^k 与长度二选一)
  pub fn from_string(_z: &mut [u64], _acc: &mut usize) {
    unimplemented!("骨架")
  }

  /// 入口对数守卫
  pub const ENTRY_POINTS: [&str; 4] = ["FromStringClassic", "FromStringLarge", "FromStringBasePowerOfTwo", "FromString"];
}
