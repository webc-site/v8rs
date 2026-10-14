// 由 tool/src/sandbox/cppheap-pointer-table.h.js 从 v8/src/sandbox/cppheap-pointer-table.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// CppHeapPointerTable 骨架

// scheme 实值解析自 include/v8-internal.h; TagType 枚举在
// src/sandbox/indirect-pointer-tag.h (既有覆盖面)。

/// 原 CppHeapPointerTable::CppHeapPointerTablePayloadTaggingScheme
pub mod cppheap_pointer_table_payload_tagging_scheme {
  /// TagType = CppHeapPointerTag (indirect-pointer-tag.h)
  pub type TagType = u16;
  /// 原 kCppHeapPointerMarkBit
  pub const MARK_BIT: u64 = 1ULL;
  /// 原 kCppHeapPointerTagShift
  pub const TAG_SHIFT: u64 = 1;
  /// 原 kCppHeapPointerTagMask = ((1ULL << (kCppHeapPointerPayloadShift - kCppHeapPointerTagShift)) - 1) << kCppHeapPointerTagShift
  pub const TAG_MASK: u64 = 0xFFFE;
  /// 原 kCppHeapPointerPayloadShift
  pub const PAYLOAD_SHIFT: u64 = 16;
  /// 原 kPayloadMask = ~0ULL (payload 全宽)
  pub const PAYLOAD_MASK: u64 = u64::MAX;
  /// 原 kFreeEntryTag = CppHeapPointerTag::kFreeEntryTag
  pub const FREE_ENTRY_TAG_NAME: &str = "kFreeEntryTag";
  /// 原 kEvacuationEntryTag = CppHeapPointerTag::kEvacuationEntryTag
  pub const EVACUATION_ENTRY_TAG_NAME: &str = "kEvacuationEntryTag";
}

/// 原 CppHeapPointerHandle = uint32_t (v8-internal.h)
pub type CppHeapPointerHandle = u32;

/// 表是否写保护 (原 IsWriteProtected)
pub const IS_WRITE_PROTECTED: bool = false;

/// 原 kNullHandle = kNullCppHeapPointerHandle
pub const NULL_HANDLE: CppHeapPointerHandle = 0;

/// 容量断言锚: kMaxCppHeapPointers == ExternalEntityTable::kMaxCapacity
/// (kMaxCppHeapPointers = kCppHeapPointerTableReservationSize / 8, 随压缩/沙箱配置分支)
pub const CAPACITY_ASSERT_SOURCE: &str = "kMaxCppHeapPointers";

/// 原 class CppHeapPointerTable (Entry + Space + inl 访问器, template 块 2 处)
pub struct CppHeapPointerTable;

// TODO(dev): 依 ExternalEntityTable 基类补 Entry/Space/Get/Set 实现。

// —— 提取不变式断言 ——
const _: () = {
  use cppheap_pointer_table_payload_tagging_scheme as s;
  assert!(s::MARK_BIT == 1);
  assert!(s::TAG_SHIFT == 1);
  assert!(s::PAYLOAD_SHIFT == 16);
  assert!(s::TAG_MASK == 0xFFFE);
  assert!(NULL_HANDLE == 0);
  assert!(!IS_WRITE_PROTECTED);
};
