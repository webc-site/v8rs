// 由 tool/src/wasm/wasm-serialization.h.js 从 v8/src/wasm/wasm-serialization.{h,cc} 生成; 修订就地改, 复跑覆盖
// Native module 序列化头部布局与函数编码标记

/// 序列化头部的定长字段布局 (字偏移), 之后为变长 compile-time imports
/// 原版布局注释: [0] magic [1] version hash [2] CPU features [3] flag hash
/// [4] enabled features [5..] compile time imports (变长)
pub mod header {
  /// 单个 uint32 字段的字节宽 (原版 kUInt32Size)
  pub const UINT32_SIZE: usize = 4;

  /// version hash 字段字节偏移 (原版 kVersionHashOffset = kUInt32Size)
  pub const VERSION_HASH_OFFSET: usize = 4;

  /// CPU features 字段字节偏移
  pub const CPU_FEATURES_OFFSET: usize = 8;

  /// flag hash 字段字节偏移
  pub const FLAG_HASH_OFFSET: usize = 12;

  /// enabled features 字段字节偏移
  pub const ENABLED_FEATURES_OFFSET: usize = 16;

  /// compile time imports 字段起始字节偏移 (变长区)
  pub const COMPILE_TIME_IMPORTS_OFFSET: usize = 20;

  /// 变长区之前的定长字段数
  pub const FIXED_HEADER_WORDS: usize = 5;
}

// 头部字 0: magic number (uint32_t)
// 头部字 1: version hash (uint32_t)
// 头部字 2: supported CPU features (uint32_t)
// 头部字 3: flag hash (uint32_t)
// 头部字 4: enabled features (via flags and OT) (uint32_t)

/// 序列化魔数 (原版 SerializedData::kMagicNumber)
/// 登记公式: 0xC0DE0000 ^ ExternalReferenceTable::kSize
/// (kSize 依赖 STATS_COUNTER_NATIVE_CODE_LIST 宏链, 数值待 Developer 侧以
///  编译器产物折叠; 禁在此编造)
pub const MAGIC_NUMBER_FORMULA: &str = "0xC0DE0000 ^ ExternalReferenceTable::kSize";

/// 序列化魔数前缀 (公式中的固定部分)
pub const MAGIC_NUMBER_PREFIX: u32 = 0xC0DE_0000;

/// 函数编码标记: 懒函数 (原版 wasm-serialization.cc kLazyFunction)
pub const LAZY_FUNCTION: u8 = 2;

/// 函数编码标记: 急切函数 (原版 kEagerFunction)
pub const EAGER_FUNCTION: u8 = 3;

/// 函数编码标记: TurboFan 函数 (原版 kTurboFanFunction)
pub const TURBOFAN_FUNCTION: u8 = 4;

// —— 提取不变式断言 ——
const _: () = {
  assert!(header::VERSION_HASH_OFFSET == 4);
  assert!(header::COMPILE_TIME_IMPORTS_OFFSET == 20);
  assert!(header::FIXED_HEADER_WORDS == 5);
  assert!(LAZY_FUNCTION < EAGER_FUNCTION && EAGER_FUNCTION < TURBOFAN_FUNCTION);
};
