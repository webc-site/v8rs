// 由 tool/src/objects/feedback-transition-table.cc.js 从 v8/src/objects/feedback-transition-table.cc + src/common/globals.h (Binary/CompareOperationFeedback) 生成; 修订就地改, 复跑覆盖
// 反馈类型转移表与编码表, 编译期真实现

//! BinaryOperationFeedback / CompareOperationFeedback: 类型反馈格转移表。
//! 反馈按位或合成, 走向更泛化类型; 表与原版 constexpr BuildTransitionMap 同构。

// ── BinaryOperationFeedback ── 12 类型索引, stride 16, lut 512 项 ──
pub const BINARY_TYPE_NAMES: [&str; 12] = [
  "None",
  "SignedSmall",
  "SignedSmallInputs",
  "AdditiveSafeInteger",
  "Number",
  "NumberOrOddball",
  "BigInt64",
  "BigInt",
  "String",
  "StringWrapper",
  "StringOrStringWrapper",
  "Any",
];
pub const BINARY_TYPE_VALUES: [u32; 12] = [
  0x0, 0x1, 0x3, 0x7, 0xf, 0x1f, 0x20, 0x60, 0x80, 0x100, 0x180, 0x1ff,
];

/// TypeIndex → Type 值 (原版 DecodeTypeIndex)
#[inline]
#[must_use]
pub const fn binary_decode_type_index(index: usize) -> u32 {
  BINARY_TYPE_VALUES[index]
}

/// feedback 值 → 最小覆盖 TypeIndex (原版 CalculateTypeIndex, X-macro 序首个覆盖者)
#[inline]
#[must_use]
pub const fn binary_calculate_type_index(feedback_value: u32) -> u8 {
  let mut i = 0;
  while i < BINARY_TYPE_VALUES.len() {
    if BINARY_TYPE_VALUES[i] & feedback_value == feedback_value {
      return i as u8;
    }
    i += 1;
  }
  (BINARY_TYPE_VALUES.len() - 1) as u8
}

/// 双反馈合成 (原版 CombineTypeIndex = CalculateTypeIndex(a|b))
#[inline]
#[must_use]
pub const fn binary_combine_type_index(a: usize, b: usize) -> u8 {
  binary_calculate_type_index(BINARY_TYPE_VALUES[a] | BINARY_TYPE_VALUES[b])
}

const fn binary_build_transition_map() -> [[u8; 16]; 12] {
  let mut m = [[0u8; 16]; 12];
  let mut i = 0;
  while i < 12 {
    let mut j = 0;
    while j < 12 {
      m[i][j] = binary_combine_type_index(i, j);
      j += 1;
    }
    i += 1;
  }
  m
}

/// 原版 kTransitionMap (u8 表, 编译期与原版 constexpr BuildTransitionMap 同算法)
pub static BINARY_TRANSITION_MAP: [[u8; 16]; 12] = binary_build_transition_map();

const fn binary_build_encode_table() -> [u8; 512] {
  let mut lut = [0u8; 512];
  let mut v = 0;
  while v < 512 {
    lut[v] = binary_calculate_type_index(v as u32);
    v += 1;
  }
  lut
}

/// 原版 kFeedbackEncodeTable (0..=kAny 全值反查表)
pub static BINARY_FEEDBACK_ENCODE_TABLE: [u8; 512] = binary_build_encode_table();

/// v8: src/objects/feedback-transition-table.cc BinaryOperationFeedback::GetTransitionMapAddress
///   — 表首地址供生成代码侧读取
#[inline]
#[must_use]
pub fn binary_transition_map_address() -> usize {
  BINARY_TRANSITION_MAP.as_ptr() as usize
}

/// v8: src/objects/feedback-transition-table.cc BinaryOperationFeedback::GetFeedbackEncodeTableAddress
#[inline]
#[must_use]
pub fn binary_feedback_encode_table_address() -> usize {
  BINARY_FEEDBACK_ENCODE_TABLE.as_ptr() as usize
}

// ── CompareOperationFeedback ── 17 类型索引, stride 32, lut 1024 项 ──
pub const COMPARE_TYPE_NAMES: [&str; 17] = [
  "None",
  "Boolean",
  "NullOrUndefined",
  "Oddball",
  "SignedSmall",
  "Number",
  "NumberOrBoolean",
  "NumberOrOddball",
  "InternalizedString",
  "String",
  "StringOrOddball",
  "Receiver",
  "ReceiverOrNullOrUndefined",
  "BigInt64",
  "BigInt",
  "Symbol",
  "Any",
];
pub const COMPARE_TYPE_VALUES: [u32; 17] = [
  0x0, 0x4, 0x8, 0xc, 0x1, 0x3, 0x7, 0xf, 0x10, 0x30, 0x3c, 0x200, 0x208, 0x80, 0x180, 0x40, 0x3ff,
];

/// TypeIndex → Type 值 (原版 DecodeTypeIndex)
#[inline]
#[must_use]
pub const fn cmp_decode_type_index(index: usize) -> u32 {
  COMPARE_TYPE_VALUES[index]
}

/// feedback 值 → 最小覆盖 TypeIndex (原版 CalculateTypeIndex, X-macro 序首个覆盖者)
#[inline]
#[must_use]
pub const fn cmp_calculate_type_index(feedback_value: u32) -> u8 {
  let mut i = 0;
  while i < COMPARE_TYPE_VALUES.len() {
    if COMPARE_TYPE_VALUES[i] & feedback_value == feedback_value {
      return i as u8;
    }
    i += 1;
  }
  (COMPARE_TYPE_VALUES.len() - 1) as u8
}

/// 双反馈合成 (原版 CombineTypeIndex = CalculateTypeIndex(a|b))
#[inline]
#[must_use]
pub const fn cmp_combine_type_index(a: usize, b: usize) -> u8 {
  cmp_calculate_type_index(COMPARE_TYPE_VALUES[a] | COMPARE_TYPE_VALUES[b])
}

const fn cmp_build_transition_map() -> [[u8; 32]; 17] {
  let mut m = [[0u8; 32]; 17];
  let mut i = 0;
  while i < 17 {
    let mut j = 0;
    while j < 17 {
      m[i][j] = cmp_combine_type_index(i, j);
      j += 1;
    }
    i += 1;
  }
  m
}

/// 原版 kTransitionMap (u8 表, 编译期与原版 constexpr BuildTransitionMap 同算法)
pub static COMPARE_TRANSITION_MAP: [[u8; 32]; 17] = cmp_build_transition_map();

const fn cmp_build_encode_table() -> [u8; 1024] {
  let mut lut = [0u8; 1024];
  let mut v = 0;
  while v < 1024 {
    lut[v] = cmp_calculate_type_index(v as u32);
    v += 1;
  }
  lut
}

/// 原版 kFeedbackEncodeTable (0..=kAny 全值反查表)
pub static COMPARE_FEEDBACK_ENCODE_TABLE: [u8; 1024] = cmp_build_encode_table();

/// v8: src/objects/feedback-transition-table.cc CompareOperationFeedback::GetTransitionMapAddress
///   — 表首地址供生成代码侧读取
#[inline]
#[must_use]
pub fn cmp_transition_map_address() -> usize {
  COMPARE_TRANSITION_MAP.as_ptr() as usize
}

/// v8: src/objects/feedback-transition-table.cc CompareOperationFeedback::GetFeedbackEncodeTableAddress
#[inline]
#[must_use]
pub fn cmp_feedback_encode_table_address() -> usize {
  COMPARE_FEEDBACK_ENCODE_TABLE.as_ptr() as usize
}

// ── 对数不变式 (oracle: clang -E 展开后 globals.h 明文) ──
const _: () = {
  assert!(BINARY_TYPE_VALUES[0] == 0x0);
  assert!(BINARY_TYPE_VALUES[2] == 0x3); // kSignedSmallInputs
  assert!(BINARY_TYPE_VALUES[5] == 0x1f); // kNumberOrOddball
  assert!(BINARY_TYPE_VALUES[8] == 0x80); // kString
  assert!(BINARY_TYPE_VALUES[BINARY_TYPE_VALUES.len() - 1] == 0x1ff); // kAny
  assert!(COMPARE_TYPE_VALUES[0] == 0x0);
  assert!(COMPARE_TYPE_VALUES[1] == 0x4); // kBoolean
  assert!(COMPARE_TYPE_VALUES[2] == 0x8); // kNullOrUndefined
  assert!(COMPARE_TYPE_VALUES[3] == 0xc); // kOddball = Boolean|NullOrUndefined
  assert!(COMPARE_TYPE_VALUES[4] == 0x1); // kSignedSmall
  assert!(COMPARE_TYPE_VALUES[5] == 0x3); // kNumber
  assert!(COMPARE_TYPE_VALUES[6] == 0x7); // kNumberOrBoolean
  assert!(COMPARE_TYPE_VALUES[9] == 0x30); // kString = Internalized|OtherString
  assert!(COMPARE_TYPE_VALUES[13] == 0x80); // kBigInt64
  assert!(COMPARE_TYPE_VALUES[14] == 0x180); // kBigInt
  assert!(COMPARE_TYPE_VALUES[15] == 0x40); // kSymbol
  assert!(COMPARE_TYPE_VALUES[COMPARE_TYPE_VALUES.len() - 1] == 0x3ff); // kAny
  // transition map 抽样: combine(kSignedSmall,kNumber)=kNumber(4/5), combine(kNone,x)=x
  assert!(binary_combine_type_index(4, 5) == 5);
  assert!(binary_combine_type_index(0, 11) == 11);
  assert!(cmp_combine_type_index(0, 16) == 16);
  assert!(cmp_combine_type_index(4, 6) == 6); // SignedSmall|NumberOrBoolean → NumberOrBoolean
  assert!(cmp_combine_type_index(2, 9) == 10); // NullOrUndefined|String → StringOrOddball
  assert!(BINARY_FEEDBACK_ENCODE_TABLE[0x1ff] == 11);
  assert!(BINARY_FEEDBACK_ENCODE_TABLE[0x3] == 2);
  assert!(COMPARE_FEEDBACK_ENCODE_TABLE[0x3ff] == 16);
  assert!(COMPARE_FEEDBACK_ENCODE_TABLE[0x7] == 6);
  assert!(BINARY_TRANSITION_MAP.len() == 12 && BINARY_TRANSITION_MAP[0].len() == 16);
  assert!(COMPARE_TRANSITION_MAP.len() == 17 && COMPARE_TRANSITION_MAP[0].len() == 32);
};
