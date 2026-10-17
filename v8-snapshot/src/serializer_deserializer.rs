// 由 tool/src/snapshot/serializer-deserializer.js 从 v8/src/snapshot/serializer-deserializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 序列化/反序列化公共字节码体系

/// v8: src/snapshot/serializer-deserializer.h SlotType
/// 槽位类型 (原版 enum class SlotType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SlotType {
  // kAnySlot
  AnySlot = 0,
  // kMapSlot
  MapSlot = 1,
}

/// v8: src/snapshot/serializer-deserializer.h Bytecode
/// 序列化字节码 (原版 enum Bytecode : uint8_t, 共 36 个命名入口)。
/// 段布局:
/// - 0x00..0x22  逐条字节码区 (kNewObject/kBackref 各占 4 码按空间编码)
/// - 0x40..0x5f  kRootArrayConstants 段 (按 RootIndex 编码)
/// - 0x60..0x7f  kFixedRawData 段 (按原始数据长度编码)
/// - 0x80..0x8f  kFixedRepeatRoot 段 (按重复数编码)
/// - 0x90..0x97  kHotObject 段 (热对象环)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Bytecode {
  // kNewObject
  NewObject = 0x0,
  // kBackref
  Backref = 0x4,
  // kReadOnlyHeapRef
  ReadOnlyHeapRef = 0x5,
  // kStartupObjectCache
  StartupObjectCache = 0x6,
  // kRootArray
  RootArray = 0x7,
  // kAttachedReference
  AttachedReference = 0x8,
  // kSharedHeapObjectCache
  SharedHeapObjectCache = 0x9,
  // kNop
  Nop = 0xa,
  // kSynchronize
  Synchronize = 0xb,
  // kVariableRepeatRoot
  VariableRepeatRoot = 0xc,
  // kOffHeapBackingStore
  OffHeapBackingStore = 0xd,
  // kOffHeapResizableBackingStore
  OffHeapResizableBackingStore = 0xe,
  // kEmbedderFieldsData
  EmbedderFieldsData = 0xf,
  // kApiWrapperFieldsData
  ApiWrapperFieldsData = 0x10,
  // kVariableRawData
  VariableRawData = 0x11,
  // kApiReference
  ApiReference = 0x12,
  // kExternalReference
  ExternalReference = 0x13,
  // kSandboxedApiReference
  SandboxedApiReference = 0x14,
  // kSandboxedExternalReference
  SandboxedExternalReference = 0x15,
  // kSandboxedRawExternalReference
  SandboxedRawExternalReference = 0x16,
  // kClearedWeakReference
  ClearedWeakReference = 0x17,
  // kWeakPrefix
  WeakPrefix = 0x18,
  // kRegisterPendingForwardRef
  RegisterPendingForwardRef = 0x19,
  // kResolvePendingForwardRef
  ResolvePendingForwardRef = 0x1a,
  // kNewContextlessMetaMap
  NewContextlessMetaMap = 0x1b,
  // kNewContextfulMetaMap
  NewContextfulMetaMap = 0x1c,
  // kExtendedMapBitFieldEx
  ExtendedMapBitFieldEx = 0x1d,
  // kIndirectPointerPrefix
  IndirectPointerPrefix = 0x1e,
  // kInitializeSelfIndirectPointer
  InitializeSelfIndirectPointer = 0x1f,
  // kAllocateJSDispatchEntry
  AllocateJSDispatchEntry = 0x20,
  // kJSDispatchEntry
  JSDispatchEntry = 0x21,
  // kProtectedPointerPrefix
  ProtectedPointerPrefix = 0x22,
  // kRootArrayConstants (段基址)
  RootArrayConstants = 0x40,
  // kFixedRawData (段基址)
  FixedRawData = 0x60,
  // kFixedRepeatRoot (段基址)
  FixedRepeatRoot = 0x80,
  // kHotObject (段基址)
  HotObject = 0x90,
}

impl Bytecode {
  /// 命名入口总数
  pub const COUNT: usize = 36;

  /// 从数值反查 (段内偏移码不在枚举内, 返回 None — 用段辅助函数解码)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    match val {
      0x0 => Some(Self::NewObject),
      0x4 => Some(Self::Backref),
      0x5 => Some(Self::ReadOnlyHeapRef),
      0x6 => Some(Self::StartupObjectCache),
      0x7 => Some(Self::RootArray),
      0x8 => Some(Self::AttachedReference),
      0x9 => Some(Self::SharedHeapObjectCache),
      0xa => Some(Self::Nop),
      0xb => Some(Self::Synchronize),
      0xc => Some(Self::VariableRepeatRoot),
      0xd => Some(Self::OffHeapBackingStore),
      0xe => Some(Self::OffHeapResizableBackingStore),
      0xf => Some(Self::EmbedderFieldsData),
      0x10 => Some(Self::ApiWrapperFieldsData),
      0x11 => Some(Self::VariableRawData),
      0x12 => Some(Self::ApiReference),
      0x13 => Some(Self::ExternalReference),
      0x14 => Some(Self::SandboxedApiReference),
      0x15 => Some(Self::SandboxedExternalReference),
      0x16 => Some(Self::SandboxedRawExternalReference),
      0x17 => Some(Self::ClearedWeakReference),
      0x18 => Some(Self::WeakPrefix),
      0x19 => Some(Self::RegisterPendingForwardRef),
      0x1a => Some(Self::ResolvePendingForwardRef),
      0x1b => Some(Self::NewContextlessMetaMap),
      0x1c => Some(Self::NewContextfulMetaMap),
      0x1d => Some(Self::ExtendedMapBitFieldEx),
      0x1e => Some(Self::IndirectPointerPrefix),
      0x1f => Some(Self::InitializeSelfIndirectPointer),
      0x20 => Some(Self::AllocateJSDispatchEntry),
      0x21 => Some(Self::JSDispatchEntry),
      0x22 => Some(Self::ProtectedPointerPrefix),
      0x40 => Some(Self::RootArrayConstants),
      0x60 => Some(Self::FixedRawData),
      0x80 => Some(Self::FixedRepeatRoot),
      0x90 => Some(Self::HotObject),
      _ => None,
    }
  }
}

/// v8: src/snapshot/serializer-deserializer.h UNUSED_SERIALIZER_BYTE_CODES
/// 空闲/预留字节码 (原版 UNUSED_SERIALIZER_BYTE_CODES X-macro 机械展开,
/// 共 133 个; 段内未占码位另行空闲, 不在此表)#[rustfmt::skip]
pub static UNUSED_SERIALIZER_BYTE_CODES: [u8; 133] = [
  0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32,
  0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x98, 0x99, 0x9a,
  0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa,
  0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba,
  0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca,
  0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda,
  0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea,
  0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa,
  0xfb, 0xfc, 0xfd, 0xfe, 0xff,
];

/// 段宽常量 (原版 static const int)
/// 首 32 个根数组条目直接编入字节码
pub const ROOT_ARRAY_CONSTANTS_COUNT: usize = 32;
/// 32 个常用原始数据长度
pub const FIXED_RAW_DATA_COUNT: usize = 32;
/// 16 个常用重复根数
pub const FIXED_REPEAT_ROOT_COUNT: usize = 16;
/// 8 个热对象槽
pub const HOT_OBJECT_COUNT: usize = 8;
/// 新对象后的双对齐哨兵 (原版 kDoubleAlignmentSentinel)
pub const DOUBLE_ALIGNMENT_SENTINEL: i32 = 0;
/// 空后备存储引用哨兵 (原版 kEmptyBackingStoreRefSentinel)
pub const EMPTY_BACKING_STORE_REF_SENTINEL: u32 = 0;

/// 段编码范围 (原版 kFirstEncodable* / kLastEncodable*, 机械展开)
pub const FIRST_ENCODABLE_FIXED_RAW_DATA_SIZE: i32 = 1;
pub const LAST_ENCODABLE_FIXED_RAW_DATA_SIZE: i32 = 32;
pub const FIRST_ENCODABLE_REPEAT_ROOT_COUNT: i32 = 2;
pub const LAST_ENCODABLE_FIXED_REPEAT_ROOT_COUNT: i32 = 17;
pub const FIRST_ENCODABLE_VARIABLE_REPEAT_ROOT_COUNT: i32 = 18;

/// v8: src/snapshot/serializer-deserializer.h SpaceEncoder/BytecodeValueEncoder
/// SpaceEncoder / BytecodeValueEncoder 机械展开:
/// 段基址 + (value - min) 即码位。快照空间共 4 个 (NUMBER_OF_SNAPSHOT_SPACES),
/// 故 kNewObject/kBackref 各占 4 码。
pub mod encode {
  use super::Bytecode;

  /// 快照空间数 (原版 kNumberOfSnapshotSpaces, 对齐 references.rs)
  pub const NUMBER_OF_SNAPSHOT_SPACES: usize = 4;

  /// 原版 SpaceEncoder<kNewObject>::Encode(space): 0x00..0x03
  #[inline]
  #[must_use]
  pub const fn encode_new_object(space: u8) -> u8 {
    Bytecode::NewObject as u8 + space
  }

  /// 原版 SpaceEncoder<kNewObject>::Decode
  #[inline]
  #[must_use]
  pub const fn decode_new_object(bc: u8) -> u8 {
    bc - Bytecode::NewObject as u8
  }

  /// 原版 SpaceEncoder<kBackref>::Encode(space): 0x04..0x07
  #[inline]
  #[must_use]
  pub const fn encode_backref(space: u8) -> u8 {
    Bytecode::Backref as u8 + space
  }

  /// 原版 FixedRawDataWithSize::Encode(size): 0x60..0x7f
  #[inline]
  #[must_use]
  pub const fn encode_fixed_raw_data(size: i32) -> u8 {
    (Bytecode::FixedRawData as u8 as i32 + size - super::FIRST_ENCODABLE_FIXED_RAW_DATA_SIZE) as u8
  }

  /// 原版 FixedRawDataWithSize::Decode
  #[inline]
  #[must_use]
  pub const fn decode_fixed_raw_data(bc: u8) -> i32 {
    bc as i32 - Bytecode::FixedRawData as u8 as i32 + super::FIRST_ENCODABLE_FIXED_RAW_DATA_SIZE
  }

  /// 原版 FixedRepeatRootWithCount::Encode(count): 0x80..0x8f
  #[inline]
  #[must_use]
  pub const fn encode_fixed_repeat_root(count: i32) -> u8 {
    (Bytecode::FixedRepeatRoot as u8 as i32 + count - super::FIRST_ENCODABLE_REPEAT_ROOT_COUNT)
      as u8
  }

  /// 原版 FixedRepeatRootWithCount::Decode
  #[inline]
  #[must_use]
  pub const fn decode_fixed_repeat_root(bc: u8) -> i32 {
    bc as i32 - Bytecode::FixedRepeatRoot as u8 as i32 + super::FIRST_ENCODABLE_REPEAT_ROOT_COUNT
  }

  /// 原版 RootArrayConstant::Encode(root_index): 0x40..0x5f
  #[inline]
  #[must_use]
  pub const fn encode_root_array_constant(root_index: i32) -> u8 {
    (Bytecode::RootArrayConstants as u8 as i32 + root_index) as u8
  }

  /// 原版 HotObject::Encode(index): 0x90..0x97
  #[inline]
  #[must_use]
  pub const fn encode_hot_object(index: i32) -> u8 {
    (Bytecode::HotObject as u8 as i32 + index) as u8
  }

  /// 原版 VariableRepeatRootCount::Encode(count) (变长重复, 减去可编码下限)
  #[inline]
  #[must_use]
  pub const fn encode_variable_repeat_root(count: i32) -> i32 {
    count - super::FIRST_ENCODABLE_VARIABLE_REPEAT_ROOT_COUNT
  }

  /// 原版 VariableRepeatRootCount::Decode
  #[inline]
  #[must_use]
  pub const fn decode_variable_repeat_root(value: i32) -> i32 {
    value + super::FIRST_ENCODABLE_VARIABLE_REPEAT_ROOT_COUNT
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(Bytecode::COUNT == 36);
  assert!(Bytecode::NewObject as u8 == 0x00);
  assert!(Bytecode::Backref as u8 == 0x04);
  assert!(Bytecode::ProtectedPointerPrefix as u8 == 0x22);
  assert!(Bytecode::RootArrayConstants as u8 == 0x40);
  assert!(Bytecode::FixedRawData as u8 == 0x60);
  assert!(Bytecode::FixedRepeatRoot as u8 == 0x80);
  assert!(Bytecode::HotObject as u8 == 0x90);
  assert!(UNUSED_SERIALIZER_BYTE_CODES[0] == 0x23);
  assert!(UNUSED_SERIALIZER_BYTE_CODES[133 - 1] == 0xff);
  assert!(ROOT_ARRAY_CONSTANTS_COUNT == 0x20);
  assert!(FIXED_RAW_DATA_COUNT == 0x20);
  assert!(FIXED_REPEAT_ROOT_COUNT == 0x10);
  assert!(HOT_OBJECT_COUNT == 8);
  assert!(encode::encode_new_object(3) == 0x03);
  assert!(encode::encode_backref(3) == 0x07);
  assert!(encode::decode_fixed_raw_data(0x60) == 1);
  assert!(encode::encode_fixed_raw_data(0x20) == 0x7f);
  assert!(encode::decode_fixed_repeat_root(0x80) == 2);
  assert!(encode::encode_hot_object(7) == 0x97);
};
