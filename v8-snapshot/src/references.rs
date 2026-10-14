// 由 tool/src/snapshot/references.js 从 v8/src/snapshot/references.h 生成; 修订就地改, 复跑覆盖
// 序列化引用与快照空间

/// v8: src/snapshot/references.h SnapshotSpace
/// SnapshotSpace: 快照分区空间。值必须连续且从 0 起 — 直接用作数组下标
/// (原版 enum class SnapshotSpace : uint8_t)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SnapshotSpace {
  // kReadOnlyHeap
  ReadOnlyHeap = 0,
  // kOld
  Old = 1,
  // kCode
  Code = 2,
  // kTrusted
  Trusted = 3,
}

/// 快照空间总数 (原版 kNumberOfSnapshotSpaces, 与 static_assert 4 == 一致)
pub const NUMBER_OF_SNAPSHOT_SPACES: usize = 4;

impl SnapshotSpace {
  /// 全空间数组序 (原版按 0..kNumberOfSnapshotSpaces 遍历)
  pub const ALL: [SnapshotSpace; NUMBER_OF_SNAPSHOT_SPACES] = [
    SnapshotSpace::ReadOnlyHeap,
    SnapshotSpace::Old,
    SnapshotSpace::Code,
    SnapshotSpace::Trusted,
  ];

  /// 从数值反查 (原版以空间值直接索引)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    match val {
      0 => Some(Self::ReadOnlyHeap),
      1 => Some(Self::Old),
      2 => Some(Self::Code),
      3 => Some(Self::Trusted),
      _ => None,
    }
  }
}

/// v8: src/snapshot/references.h SerializerReference::SpecialValueType
/// SerializerReference 内嵌特殊值类型 (原版 SerializerReference::SpecialValueType,
/// private 枚举, 顺序赋值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SpecialValueType {
  // kBackReference
  BackReference = 0,
  // kAttachedReference
  AttachedReference = 1,
  // kOffHeapBackingStore
  OffHeapBackingStore = 2,
  // kBuiltinReference
  BuiltinReference = 3,
}

/// v8: src/snapshot/references.h SerializerReference TypeBits/ValueBits
/// SerializerReference 位域布局 (原版 TypeBits / ValueBits, 打包进单个 u32):
/// - bit 0..2   TypeBits   : SpecialValueType, 宽 2
/// - bit 2..32  ValueBits  : u32 索引, 宽 30
pub mod serializer_reference_bits {
  /// TypeBits 起始位
  pub const TYPE_OFFSET: u32 = 0;
  /// TypeBits 宽度
  pub const TYPE_BITS: u32 = 2;
  /// ValueBits 起始位 (TypeBits::kSize)
  pub const VALUE_OFFSET: u32 = 2;
  /// ValueBits 宽度 (32 - TypeBits::kSize)
  pub const VALUE_BITS: u32 = 30;

  /// TypeBits 掩码
  pub const TYPE_MASK: u32 = ((1 << TYPE_BITS) - 1) << TYPE_OFFSET;
  /// ValueBits 掩码
  pub const VALUE_MASK: u32 = ((1 << VALUE_BITS) - 1) << VALUE_OFFSET;

  /// 原版 TypeBits::encode
  #[inline]
  #[must_use]
  pub const fn encode_type(t: u8) -> u32 {
    ((t as u32) & ((1 << TYPE_BITS) - 1)) << TYPE_OFFSET
  }

  /// 原版 TypeBits::decode
  #[inline]
  #[must_use]
  pub const fn decode_type(bits: u32) -> u8 {
    ((bits & TYPE_MASK) >> TYPE_OFFSET) as u8
  }

  /// 原版 ValueBits::encode
  #[inline]
  #[must_use]
  pub const fn encode_value(v: u32) -> u32 {
    v << VALUE_OFFSET
  }

  /// 原版 ValueBits::decode
  #[inline]
  #[must_use]
  pub const fn decode_value(bits: u32) -> u32 {
    (bits & VALUE_MASK) >> VALUE_OFFSET
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(NUMBER_OF_SNAPSHOT_SPACES == 4);
  assert!(SnapshotSpace::ReadOnlyHeap as u8 == 0);
  assert!(SnapshotSpace::Old as u8 == 1);
  assert!(SnapshotSpace::Code as u8 == 2);
  assert!(SnapshotSpace::Trusted as u8 == 3);
  assert!(SpecialValueType::BackReference as u8 == 0);
  assert!(SpecialValueType::AttachedReference as u8 == 1);
  assert!(SpecialValueType::OffHeapBackingStore as u8 == 2);
  assert!(SpecialValueType::BuiltinReference as u8 == 3);
  assert!(serializer_reference_bits::TYPE_BITS == 2);
  assert!(serializer_reference_bits::VALUE_BITS == 30);
};
