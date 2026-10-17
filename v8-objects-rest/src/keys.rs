// 由 tool/src/objects/keys.h.js 从 v8/src/objects/keys.{h,cc} 生成; 修订就地改, 复跑覆盖
// 键收集枚举 (v8 公开枚举值 1:1 对齐)
//! KeyAccumulator 键收集配置枚举。

/// 原版 enum AddKeyConversion (顺序值)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AddKeyConversion {
  /// DO_NOT_CONVERT
  DoNotConvert = 0,
  /// CONVERT_TO_ARRAY_INDEX
  ConvertToArrayIndex = 1,
}

/// 原版 enum class GetKeysConversion (值 = v8::KeyConversionMode)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GetKeysConversion {
  /// kKeepNumbers
  Keepnumbers = 1,
  /// kConvertToString
  Converttostring = 0,
  /// kNoNumbers
  Nonumbers = 2,
}

/// 原版 enum class KeyCollectionMode (值 = v8::KeyCollectionMode)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KeyCollectionMode {
  /// kOwnOnly
  Ownonly = 0,
  /// kIncludePrototypes
  Includeprototypes = 1,
}

/// KeyAccumulator 私有分类 (原版 enum IndexedOrNamed)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IndexedOrNamed {
  /// kIndexed
  Indexed = 0,
  /// kNamed
  Named = 1,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(GetKeysConversion::KeepNumbers as u8 == 1);
  assert!(GetKeysConversion::ConvertToString as u8 == 0);
  assert!(GetKeysConversion::NoNumbers as u8 == 2);
  assert!(KeyCollectionMode::OwnOnly as u8 == 0);
  assert!(KeyCollectionMode::IncludePrototypes as u8 == 1);
};
