// 由 tool/src/sandbox/trusted-pointer-table.h.js 从 v8/src/sandbox/trusted-pointer-table.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// TrustedPointerTable 骨架

// scheme 实值解析自 src/sandbox/indirect-pointer-tag.h, 按目标双分支展开:
// x64 (LA57 五级页表) 与其他目标 (arm64 等)。IndirectPointerTag 枚举本体
// 在 indirect-pointer-tag.h (既有 tags 覆盖面)。

/// 原 TrustedPointerTable::TrustedPointerTableTaggingScheme
pub mod trusted_pointer_table_tagging_scheme {
  /// TagType = IndirectPointerTag (u16)
  pub type TagType = u16;

  /// x64 + LA57 分支 (原 #if V8_TARGET_ARCH_X64 && V8_SUPPORT_LA57)
  pub mod la57 {
    /// 原 kTrustedPointerTableTagMask
    pub const TAG_MASK: u64 = 0xFE00000000000000;
    /// 原 kTrustedPointerTableMarkBit
    pub const MARK_BIT: u64 = 0x100000000000000;
    /// 原 kTrustedPointerTablePayloadMask
    pub const PAYLOAD_MASK: u64 = 0xFFFFFFFFFFFFFF;
    /// 原 kTrustedPointerTableTagShift
    pub const TAG_SHIFT: u64 = 57;
  }

  /// 其他目标分支 (arm64 / x64 非 LA57)
  pub mod non_la57 {
    pub const TAG_MASK: u64 = 0xFFFE000000000000;
    pub const MARK_BIT: u64 = 0x1000000000000;
    pub const PAYLOAD_MASK: u64 = 0xFFFFFFFFFFFF;
    pub const TAG_SHIFT: u64 = 49;
  }

  /// 原 kTrustedPointerTablePayloadShift (双分支共用)
  pub const PAYLOAD_SHIFT: u64 = 0;
  /// 原 kFreeEntryTag = kIndirectPointerFreeEntryTag
  pub const FREE_ENTRY_TAG_NAME: &str = "kIndirectPointerFreeEntryTag";
  /// 原 kEvacuationEntryTag = kIndirectPointerEvacuationEntryTag
  pub const EVACUATION_ENTRY_TAG_NAME: &str = "kIndirectPointerEvacuationEntryTag";
}

/// 表是否写保护 (原 IsWriteProtected)
pub const IS_WRITE_PROTECTED: bool = false;

/// 容量断言锚: kMaxTrustedPointers == kMaxCapacity
/// (kMaxTrustedPointers = kTrustedPointerTableReservationSize(64MB) / 8)
pub const CAPACITY_ASSERT_SOURCE: &str = "kMaxTrustedPointers";

/// 原 class TrustedPointerTable (Entry + Space)
pub struct TrustedPointerTable;

// TODO(dev): 依 ExternalEntityTable 基类补 Entry/Get/Set/Space 实现。

// —— 提取不变式断言 ——
const _: () = {
  use trusted_pointer_table_tagging_scheme as s;
  assert!(s::la57::TAG_SHIFT == 57);
  assert!(s::non_la57::TAG_SHIFT == 49);
  assert!(s::la57::PAYLOAD_MASK == 0x00FF_FFFF_FFFF_FFFF);
  assert!(s::non_la57::PAYLOAD_MASK == 0x0000_FFFF_FFFF_FFFF);
  assert!(!IS_WRITE_PROTECTED);
};
