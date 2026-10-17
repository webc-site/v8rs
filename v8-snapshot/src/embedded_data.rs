// 由 tool/src/snapshot/embedded/embedded-data.js 从 v8/src/snapshot/embedded/embedded-data.{h,cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// 内嵌 blob 布局

/// ReorderedBuiltinIndex (原版 using, 内嵌快照序索引)
pub type ReorderedBuiltinIndex = u32;

// 内嵌 blob 数据区布局 (原版 h 注释机械登记):
// data: [0] 数据区哈希 | [1] 代码区哈希 | [2] 内嵌相关堆对象哈希
//       [3..] 布局描述表 (builtin id 序) | [n] 内建查找表 (end_offset 升序, 快照序)
//       [x..] 各 builtin 元数据区
// code: 各 builtin 指令区 (内嵌快照序)

/// v8: src/snapshot/embedded/embedded-data.h LayoutDescription
/// LayoutDescription 表项 (原版 struct LayoutDescription, 3 x u32):
pub mod layout_description {
  /// instruction_offset @ 0
  pub const INSTRUCTION_OFFSET_OFFSET: u32 = 0;
  /// instruction_length @ 4
  pub const INSTRUCTION_LENGTH_OFFSET: u32 = 4;
  /// metadata_offset @ 8
  pub const METADATA_OFFSET_OFFSET: u32 = 8;
  /// sizeof = 3 * kUInt32Size
  pub const SIZE: u32 = 12;
}

/// v8: src/snapshot/embedded/embedded-data.h BuiltinLookupEntry
/// BuiltinLookupEntry 表项 (原版 struct BuiltinLookupEntry, 2 x u32):
pub mod builtin_lookup_entry {
  /// end_offset @ 0
  pub const END_OFFSET_OFFSET: u32 = 0;
  /// builtin_id @ 4
  pub const BUILTIN_ID_OFFSET: u32 = 4;
  /// sizeof = 2 * kUInt32Size
  pub const SIZE: u32 = 8;
}

/// v8: src/snapshot/embedded/embedded-data.h EmbeddedBlob* / *TableOffset
/// blob 数据区偏移链 (原版 constexpr 函数逐条转写, kSizetSize=8)。
/// 固定前缀偏移 (与表容量无关) 为常量; 表大小依赖 `kTableSize = Builtins::kBuiltinCount`
/// (跨模块编译期常量, 由 v8-builtins 落值), 故以 `const fn` 参数化, 避免伪常量失真。
pub mod blob_layout {
  use super::{
    builtin_lookup_entry::SIZE as BUILTIN_LOOKUP_ENTRY_SIZE,
    layout_description::SIZE as LAYOUT_DESCRIPTION_SIZE,
  };

  /// v8: src/snapshot/embedded/embedded-data.h EmbeddedBlobDataHashOffset
  pub const EMBEDDED_BLOB_DATA_HASH_OFFSET: u32 = 0;
  /// 哈希宽度 = kSizetSize
  pub const EMBEDDED_BLOB_DATA_HASH_SIZE: u32 = 8;
  /// v8: src/snapshot/embedded/embedded-data.h EmbeddedBlobCodeHashOffset
  #[inline]
  #[must_use]
  pub const fn embedded_blob_code_hash_offset() -> u32 {
    EMBEDDED_BLOB_DATA_HASH_OFFSET + EMBEDDED_BLOB_DATA_HASH_SIZE
  }
  pub const EMBEDDED_BLOB_CODE_HASH_SIZE: u32 = 8;

  /// v8: src/snapshot/embedded/embedded-data.h IsolateHashOffset
  #[inline]
  #[must_use]
  pub const fn isolate_hash_offset() -> u32 {
    embedded_blob_code_hash_offset() + EMBEDDED_BLOB_CODE_HASH_SIZE
  }
  pub const ISOLATE_HASH_SIZE: u32 = 8;

  /// v8: src/snapshot/embedded/embedded-data.h LayoutDescriptionTableOffset
  #[inline]
  #[must_use]
  pub const fn layout_description_table_offset() -> u32 {
    isolate_hash_offset() + ISOLATE_HASH_SIZE
  }

  /// v8: src/snapshot/embedded/embedded-data.h LayoutDescriptionTableSize
  /// = sizeof(LayoutDescription) * kTableSize
  #[inline]
  #[must_use]
  pub const fn layout_description_table_size(table_size: u32) -> u32 {
    LAYOUT_DESCRIPTION_SIZE * table_size
  }

  /// v8: src/snapshot/embedded/embedded-data.h BuiltinLookupEntryTableOffset
  #[inline]
  #[must_use]
  pub const fn builtin_lookup_entry_table_offset(table_size: u32) -> u32 {
    layout_description_table_offset() + layout_description_table_size(table_size)
  }

  /// v8: src/snapshot/embedded/embedded-data.h BuiltinLookupEntryTableSize
  /// = sizeof(BuiltinLookupEntry) * kTableSize
  #[inline]
  #[must_use]
  pub const fn builtin_lookup_entry_table_size(table_size: u32) -> u32 {
    BUILTIN_LOOKUP_ENTRY_SIZE * table_size
  }

  /// v8: src/snapshot/embedded/embedded-data.h FixedDataSize
  #[inline]
  #[must_use]
  pub const fn fixed_data_size(table_size: u32) -> u32 {
    builtin_lookup_entry_table_offset(table_size) + builtin_lookup_entry_table_size(table_size)
  }

  /// v8: src/snapshot/embedded/embedded-data.h RawMetadataOffset
  #[inline]
  #[must_use]
  pub const fn raw_metadata_offset(table_size: u32) -> u32 {
    fixed_data_size(table_size)
  }

  /// 代码区独立成段, 起点恒 0
  pub const RAW_CODE_OFFSET: u32 = 0;

  /// 哈希覆盖起点 (原版 cc kFirstHashedDataOffset = IsolateHashOffset())
  pub const FIRST_HASHED_DATA_OFFSET: u32 = 16;
}

/// 对齐函数 (原版 PadAndAlignCode/PadAndAlignData):
/// - 指令: RoundUp<kCodeAlignment>(size + 1) — 尾部至少 1 字节供 int3 填充;
///   kCodeAlignment = 1 << kCodeAlignmentBits, x64 = 64 (bits 6),
///   arm64 = 32 (bits 5, 其他架构分支)
/// - 元数据: RoundUp<kMetadataAlignment>(size), kMetadataAlignment = kIntSize
///   = 4
pub const CODE_ALIGNMENT_X64: u32 = 64;
pub const CODE_ALIGNMENT_ARM64: u32 = 32;
pub const METADATA_ALIGNMENT: u32 = 4;

// cc 侧重定位掩码 (原版 kRelocMask):
// CODE_TARGET | RELATIVE_CODE_TARGET — FinalizeEmbeddedCodeTargets 用,
// 断言基准 Builtins::kAllBuiltinsAreIsolateIndependent。
// cc 规模: 482 行 (语义由 Developer 落)。
