// 由 tool/src/snapshot/code-serializer.js 从 v8/src/snapshot/code-serializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 代码缓存序列化头布局

/// v8: src/snapshot/code-serializer.h SerializedCodeData header
/// SerializedCodeData 头布局 (原版 code-serializer.h, uint32 槽; 例外: 256 位源哈希)。
/// 偏移链机械展开 (基点 kMagicNumberOffset = 0):
pub mod serialized_code_data_layout {
  /// kVersionHashOffset = kMagicNumberOffset + kUInt32Size
  pub const VERSION_HASH_OFFSET: u32 = 4;
  /// kSourceHashOffset = kVersionHashOffset + kUInt32Size
  pub const SOURCE_HASH_OFFSET: u32 = 8;
  /// kFlagHashOffset = kSourceHashOffset + kSizeOfSha256Digest
  pub const FLAG_HASH_OFFSET: u32 = 40;
  /// kReadOnlySnapshotChecksumOffset
  pub const READ_ONLY_SNAPSHOT_CHECKSUM_OFFSET: u32 = 44;
  /// kPayloadLengthOffset
  pub const PAYLOAD_LENGTH_OFFSET: u32 = 48;
  /// kTrustedPayloadLengthOffset (双载荷: untrusted + trusted)
  pub const TRUSTED_PAYLOAD_LENGTH_OFFSET: u32 = 52;
  /// kChecksumOffset
  pub const CHECKSUM_OFFSET: u32 = 56;
  /// kUnalignedHeaderSize
  pub const UNALIGNED_HEADER_SIZE: u32 = 60;
  /// kHeaderSize = POINTER_SIZE_ALIGN(kUnalignedHeaderSize)
  pub const HEADER_SIZE: u32 = 64;

  /// SourceHash 尺寸 = kSizeOfSha256Digest
  pub const SOURCE_HASH_SIZE: usize = 32;
}

/// 健全性检查结果 (原版 SerializedCodeSanityCheckResult, typedef 自
/// v8::ScriptCompiler::CachedData::CompatibilityCheckResult, 9 档):
/// 原版 static_assert(kLast == 9) — 档位定义在公开 API 头 (v8.h), 由 Developer
/// 对齐 v8 crate 落枚举; 本模块登记断言基准。
pub const SANITY_CHECK_RESULT_COUNT: usize = 9;

// cc 侧 (机械登记, 1035 行, 语义由 Developer 落):
// SerializeSharedFunctionInfo, SerializeObjectImpl, SerializeGeneric, CreateInterpreterDataForDeserializedCode, FinalizeDeserialization, BaselineBatchCompileIfSparkplugCompiled, BaselineBatchCompileIfSparkplugCompiled, StartDeserializeOffThread, NotifySourceTextAvailable, HashString
// 主入口: CodeSerializer::Serialize → 头填充 (魔数/版本哈希/旗标哈希/RO 校验和/
// 双载荷长度/校验和) → UntrustedPayload + TrustedPayload 双段。
