// 由 tool/src/objects/hash-seed-wrapper.h.js 从 v8/src/objects/hash-seed-wrapper.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// HashSeedWrapper 布局登记
//! HashSeedWrapper: 只读堆中的哈希种子包装 (单字段非对齐载荷)。

/// 布局字段 (h 侧唯一成员; 非对齐值成员)。
pub const HASH_SEED_WRAPPER_FIELDS: [(&str, &str); 1] = [("data_", "HashSeed::Data")];

// —— 提取不变式断言 ——
const _: () = {
  assert!(HASH_SEED_WRAPPER_FIELDS.len() == 1);
  assert!(HASH_SEED_WRAPPER_FIELDS[0].0 == "data_");
};
