// 由 tool/src/objects/arguments.h.js 从 v8/src/objects/arguments.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// arguments 槽位索引与布局常量 (压缩配置)
//! arguments: JS 参数对象槽位与 sloppy arguments 元素布局。
//! 目标配置: 指针压缩开 (kTaggedSize=4, TAGGED_SIZE_8_BYTES=false)。

/// kTaggedSize (压缩配置)。
pub const TAGGED_SIZE: usize = 4;

// ── JSSloppyArgumentsObject in-object 槽 (相对 JSArgumentsObject::kHeaderSize) ──
/// "length" 槽索引。
pub const SLOPPY_LENGTH_SLOT_INDEX: usize = 0;
/// "callee" 槽索引。
pub const SLOPPY_CALLEE_SLOT_INDEX: usize = 1;
/// in-object 属性槽总数。
pub const SLOPPY_IN_OBJECT_PROPERTY_COUNT: usize = 2;

// ── JSStrictArgumentsObject in-object 槽 ────────────────────────────────────
/// "length" 槽索引。
pub const STRICT_LENGTH_SLOT_INDEX: usize = 0;
/// in-object 属性槽总数。
pub const STRICT_IN_OBJECT_PROPERTY_COUNT: usize = 1;

/// in-object 槽偏移 (原版 kLengthOffset = kHeaderSize + index * kTaggedSize)。
#[inline]
#[must_use]
pub const fn in_object_offset(header_size: usize, slot_index: usize) -> usize {
  header_size + slot_index * TAGGED_SIZE
}

// ── SloppyArgumentsElements 布局 (压缩配置求值) ────────────────────────────
/// length_ 字段偏移 (原版 kLengthOffset = sizeof(HeapObject))。
pub const SLOPPY_LENGTH_OFFSET: u32 = 4;
/// 头部大小 (原版 kHeaderSize = kLengthOffset + kApiInt32Size + 2 * kTaggedSize)。
pub const SLOPPY_HEADER_SIZE: u32 = 16;

/// SloppyArgumentsElements 固定布局字段 (h 侧顺序 1:1; 尾部为 mapped_entries 变长段)。
pub const SLOPPY_ELEMENTS_FIELDS: [(&str, &str); 2] = [
  ("context_", "Context"),
  ("arguments_", "UnionOf<FixedArray, NumberDictionary>"),
];

/// AliasedArgumentsEntry 载荷字段 (Smi 槽)。
pub const ALIASED_ENTRY_FIELD: (&str, &str) = ("aliased_context_slot_", "Smi");

// —— 提取不变式断言 ——
const _: () = {
  assert!(SLOPPY_LENGTH_SLOT_INDEX == 0 && SLOPPY_CALLEE_SLOT_INDEX == 1);
  assert!(STRICT_IN_OBJECT_PROPERTY_COUNT == 1);
  assert!(SLOPPY_HEADER_SIZE == 16);
  assert!(SLOPPY_ELEMENTS_FIELDS.len() == 2);
};
