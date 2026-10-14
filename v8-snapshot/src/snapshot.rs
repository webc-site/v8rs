// 由 tool/src/snapshot/snapshot.js 从 v8/src/snapshot/snapshot.{h,cc} 生成; 修订就地改, 复跑覆盖
// 快照门面与 blob 头布局

/// v8: src/snapshot/snapshot.h SerializerFlag
/// SerializerFlag 位标志 (原版 enum SerializerFlag, base::Flags 包装)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SerializerFlags(u8);

impl SerializerFlags {
  /// v8: src/snapshot/snapshot.h SerializerFlag::kAllowUnknownExternalReferencesForTesting
  pub const ALLOW_UNKNOWN_EXTERNAL_REFERENCES_FOR_TESTING: u8 = 1 << 0;
  /// v8: src/snapshot/snapshot.h SerializerFlag::kAllowActiveIsolateForTesting
  pub const ALLOW_ACTIVE_ISOLATE_FOR_TESTING: u8 = 1 << 1;
  /// v8: src/snapshot/snapshot.h SerializerFlag::kReconstructReadOnlyAndSharedObjectCachesForTesting
  pub const RECONSTRUCT_READ_ONLY_AND_SHARED_OBJECT_CACHES_FOR_TESTING: u8 = 1 << 2;
  /// v8: src/snapshot/snapshot.h SerializerFlag::kAllowSerializingAllTrustedObjects
  pub const ALLOW_SERIALIZING_ALL_TRUSTED_OBJECTS: u8 = 1 << 3;

  /// 原版 kDefaultSerializerFlags = {} — 全默认空标志
  pub const DEFAULT: Self = Self(0);

  #[inline]
  #[must_use]
  pub const fn contains(self, flag: u8) -> bool {
    (self.0 & flag) != 0
  }

  #[inline]
  #[must_use]
  pub const fn with(self, flag: u8) -> Self {
    Self(self.0 | flag)
  }

  #[inline]
  #[must_use]
  pub const fn without(self, flag: u8) -> Self {
    Self(self.0 & !flag)
  }

  #[inline]
  #[must_use]
  pub const fn bits(self) -> u8 {
    self.0
  }
}

// Snapshot 门面 (原版 class Snapshot : AllStatic, 全静态)。方法面 (共 15 项):
// - `ClearReconstructableDataForSerialization`
// - `Create`
// - `Initialize`
// - `NewContextFromSnapshot`
// - `SerializeDeserializeAndVerifyForTesting`
// - `HasContextSnapshot`
// - `EmbedsScript`
// - `GetExpectedChecksum`
// - `CalculateChecksum`
// - `VerifyChecksum`
// - `ExtractRehashability`
// - `ExtractReadOnlySnapshotChecksum`
// - `VersionIsValid`
// - `ShouldVerifyChecksum`
// - `SnapshotIsValid`

// 语义分组:
// - 序列化: ClearReconstructableDataForSerialization / Create
// - 反序列化: Initialize / NewContextFromSnapshot
// - 测试: SerializeDeserializeAndVerifyForTesting
// - 辅助: HasContextSnapshot / EmbedsScript / GetExpectedChecksum / CalculateChecksum /
//   VerifyChecksum / ExtractRehashability / ExtractReadOnlySnapshotChecksum / VersionIsValid
// - 快照源实现方: DefaultSnapshotBlob / ShouldVerifyChecksum (DEBUG 另有 SnapshotIsValid)

/// SnapshotCreatorImpl 上下文槽索引 (原版 constexpr)
pub const DEFAULT_CONTEXT_INDEX: usize = 0;
pub const FIRST_ADDTL_CONTEXT_INDEX: usize = 1;

/// v8: src/snapshot/snapshot.cc blob header layout
/// 快照 blob 头布局 (原版 snapshot.cc 匿名 namespace, uint32 槽, 小端):
/// [0] 上下文数 | [1] 可重散列标志 | [2] 校验和 | [3] RO 快照校验和
/// [4..67] 64 字节版本串 | [68] startup 偏移 | [69] 共享堆偏移 | [70..] 各上下文偏移
pub mod snapshot_blob_layout {
  pub const NUMBER_OF_CONTEXTS_OFFSET: u32 = 0;
  pub const REHASHABILITY_OFFSET: u32 = 4;
  pub const CHECKSUM_OFFSET: u32 = 8;
  pub const READ_ONLY_SNAPSHOT_CHECKSUM_OFFSET: u32 = 12;
  pub const VERSION_STRING_OFFSET: u32 = 16;
  pub const VERSION_STRING_LENGTH: u32 = 64;
  pub const STARTUP_OFFSET_OFFSET: u32 = 80;
  pub const SHARED_HEAP_OFFSET_OFFSET: u32 = 84;
  pub const FIRST_CONTEXT_OFFSET_OFFSET: u32 = 88;

  /// RO 快照紧跟头后, POINTER_SIZE_ALIGN(8) 对齐
  /// (原版 ReadOnlySnapshotOffset(num_contexts))
  #[inline]
  #[must_use]
  pub const fn read_only_snapshot_offset(num_contexts: u32) -> u32 {
    (FIRST_CONTEXT_OFFSET_OFFSET + num_contexts * 4 + 7) & !7
  }

  /// 原版 ContextSnapshotOffsetOffset(index)
  #[inline]
  #[must_use]
  pub const fn context_snapshot_offset_offset(index: u32) -> u32 {
    FIRST_CONTEXT_OFFSET_OFFSET + index * 4
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SerializerFlags::ALLOW_UNKNOWN_EXTERNAL_REFERENCES_FOR_TESTING == 1);
  assert!(SerializerFlags::ALLOW_ACTIVE_ISOLATE_FOR_TESTING == 2);
  assert!(SerializerFlags::RECONSTRUCT_READ_ONLY_AND_SHARED_OBJECT_CACHES_FOR_TESTING == 4);
  assert!(SerializerFlags::ALLOW_SERIALIZING_ALL_TRUSTED_OBJECTS == 8);
  assert!(DEFAULT_CONTEXT_INDEX == 0);
  assert!(FIRST_ADDTL_CONTEXT_INDEX == 1);
  assert!(snapshot_blob_layout::VERSION_STRING_LENGTH == 64);
  assert!(snapshot_blob_layout::FIRST_CONTEXT_OFFSET_OFFSET == 88);
  assert!(snapshot_blob_layout::read_only_snapshot_offset(0) == 88);
  assert!(snapshot_blob_layout::read_only_snapshot_offset(2) == 96);
};
