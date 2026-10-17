// 由 tool/src/objects/descriptor-array.tq.js 从 v8/src/objects/descriptor-array.tq 生成; 修订就地改, 复跑覆盖
// DescriptorArray tq 布局 (clang -E descriptor-array.h 对数, 压缩配置)
//! DescriptorArray 的 Torque 侧布局镜像: EnumCache / DescriptorEntry /
//! DescriptorArrayFlags / 头部字段偏移 (压缩指针 K_TAGGED_SIZE=4)。

/// tq extern class EnumCache 字段表 (顺序 1:1)。
pub const ENUM_CACHE_FIELDS: [(&str, &str); 2] =
  [("keys", "FixedArray"), ("indices", "FixedArray")];

/// tq @export struct DescriptorEntry 字段表 (顺序 1:1; 尾随变长数组元素类型)。
pub const DESCRIPTOR_ENTRY_FIELDS: [(&str, &str); 3] = [
  ("key", "Name | Undefined"),
  ("details", "Smi | Undefined"),
  (
    "value",
    "JSAny | Weak<Map> | AccessorInfo | AccessorPair | ClassPositions | NumberDictionary",
  ),
];

/// DescriptorArrayFlags: fast_iterable 位段 (2 bit @0, 容器 uint32)。
pub const FAST_ITERABLE_SHIFT: u32 = 0;
/// fast_iterable 掩码。
pub const FAST_ITERABLE_MASK: u32 = 0x3;

// ── DescriptorArray 头部布局 (压缩 K_TAGGED_SIZE=4) ──────────────────────
/// number_of_all_descriptors 偏移 (u16)。
pub const K_NUMBER_OF_ALL_DESCRIPTORS_OFFSET: usize = 4;
/// number_of_descriptors 偏移 (u16)。
pub const K_NUMBER_OF_DESCRIPTORS_OFFSET: usize = 6;
/// flags 偏移 (u32)。
pub const K_FLAGS_OFFSET: usize = 8;
/// enum_cache 偏移 (tagged)。
pub const K_ENUM_CACHE_OFFSET: usize = 12;
/// descriptors 数组起点 = 原版 kHeaderSize / kDescriptorsOffset。
pub const K_DESCRIPTORS_OFFSET: usize = 16;

/// tq extern class DescriptorArray 头字段表 (顺序 1:1, 含尾随变长数组声明)。
pub const DESCRIPTOR_ARRAY_FIELDS: [(&str, &str); 5] = [
  ("number_of_all_descriptors", "uint16"),
  ("number_of_descriptors", "uint16"),
  ("flags", "DescriptorArrayFlags"),
  ("enum_cache", "EnumCache"),
  ("descriptors[number_of_all_descriptors]", "DescriptorEntry"),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(ENUM_CACHE_FIELDS.len() == 2 && DESCRIPTOR_ENTRY_FIELDS.len() == 3);
  assert!(DESCRIPTOR_ARRAY_FIELDS.len() == 5);
  assert!((FAST_ITERABLE_MASK == 0x3) && (FAST_ITERABLE_SHIFT == 0));
  assert!(K_DESCRIPTORS_OFFSET == 16);
  assert!(
    (K_NUMBER_OF_DESCRIPTORS_OFFSET == 6) && (K_FLAGS_OFFSET == 8) && (K_ENUM_CACHE_OFFSET == 12)
  );
};
