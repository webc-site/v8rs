// 由 tool/src/snapshot/snapshot-data.js 从 v8/src/snapshot/snapshot-data.{h,cc} 生成; 修订就地改, 复跑覆盖
// 快照数据 blob 头布局

/// v8: src/snapshot/snapshot-data.h SerializedData header
/// SerializedData: 快照 blob 基类。头部署 uint32 槽, 小端读写
/// (原版 base::Read/WriteLittleEndianValue)。
pub mod serialized_data {
  /// ChunkSizeBits = BitField<uint32_t, 0, 31>: 压缩块大小
  pub const CHUNK_SIZE_OFFSET: u32 = 0;
  pub const CHUNK_SIZE_BITS: u32 = 31;
  pub const CHUNK_SIZE_MASK: u32 = ((1u32 << CHUNK_SIZE_BITS) - 1) << CHUNK_SIZE_OFFSET;

  /// IsLastChunkBits = BitField<bool, 31, 1>: 末块标记
  pub const IS_LAST_CHUNK_OFFSET: u32 = 31;
  pub const IS_LAST_CHUNK_BITS: u32 = 1;
  pub const IS_LAST_CHUNK_MASK: u32 = 1u32 << IS_LAST_CHUNK_OFFSET;

  /// 魔数槽偏移 (原版 kMagicNumberOffset)
  pub const MAGIC_NUMBER_OFFSET: u32 = 0;
  /// 魔数 = 0xC0DE0000 ^ ExternalReferenceTable::kSize
  /// (原版表达式: 0xC0DE0000 ^ ExternalReferenceTable::kSize; ExternalReferenceTable::kSize 跨模块常量链,
  /// 由 Developer 对齐 v8-common/external-reference-table 产物后落值)
  pub const MAGIC_NUMBER_EXPR: &str = "0xC0DE0000 ^ ExternalReferenceTable::kSize";

  /// 原版 ChunkSizeBits::encode
  #[inline]
  #[must_use]
  pub const fn encode_chunk_size(v: u32) -> u32 {
    (v << CHUNK_SIZE_OFFSET) & CHUNK_SIZE_MASK
  }
  /// 原版 IsLastChunkBits::encode
  #[inline]
  #[must_use]
  pub const fn encode_is_last_chunk(v: bool) -> u32 {
    if v { IS_LAST_CHUNK_MASK } else { 0 }
  }
}

/// v8: src/snapshot/snapshot-data.h SnapshotData header
/// SnapshotData: 预留区大小 + 序列化载荷包装。
/// 头布局 (uint32 槽):
/// [0] 魔数 + (内部) 外部引用数
/// [1] 载荷长度
/// 之后为序列化载荷
pub mod layout {
  /// kUInt32Size 机械展开
  pub const UINT32_SIZE: u32 = 4;
  /// 载荷长度槽偏移 = kMagicNumberOffset + kUInt32Size
  pub const PAYLOAD_LENGTH_OFFSET: u32 = UINT32_SIZE;
  /// 头大小 = kPayloadLengthOffset + kUInt32Size
  pub const HEADER_SIZE: u32 = PAYLOAD_LENGTH_OFFSET + UINT32_SIZE;
}

// cc 实现 (机械提取函数名):
