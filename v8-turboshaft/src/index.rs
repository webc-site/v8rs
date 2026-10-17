// 由 tool/src/compiler/turboshaft/index.h.js 从 v8/src/compiler/turboshaft/index.h 生成; 修订就地改, 复跑覆盖
// OpIndex/BlockIndex 索引与抽象类型标签
use crate::representations::RegisterRepresentation;

/// 原版 OperationStorageSlot — 操作缓冲存储槽 (64 位)
pub type OperationStorageSlot = u64;

/// 原版 kSlotsPerId — 每个操作至少占 2 槽, 每 2 槽分配一个 id
pub const SLOTS_PER_ID: usize = 2;

/// 原版 kGenerationMaskShift — DEBUG 世代位偏移 (release 布局不含世代位)
pub const GENERATION_MASK_SHIFT: u32 = 1;

/// 原版 kGenerationMask
pub const GENERATION_MASK: u32 = 1 << GENERATION_MASK_SHIFT;

/// 原版 kUnmaskGenerationMask
pub const UNMASK_GENERATION_MASK: u32 = !GENERATION_MASK;

/// 原版 kExternalIdFlag — 外部节点 id 编码标志 (offset 低 0 位残余)
pub const EXTERNAL_ID_FLAG: u32 = 1;

// DEBUG 世代位编码 (FromOffset 里 offset |= generation_bit << 1) 仅 DEBUG 布局;
// release 布局 offset 即纯偏移, 此处按 release 语义生成。

/// 原版 OpIndex — 操作缓冲偏移索引 (32 位, 缓冲重分配下稳定)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct OpIndex(u32);

impl OpIndex {
  /// 原版 Invalid() — 全 1 哨兵
  pub const INVALID: OpIndex = OpIndex(u32::MAX);

  /// 原版 FromOffset (offset 须为 OperationStorageSlot 大小的倍数)
  #[inline]
  #[must_use]
  pub const fn from_offset(offset: u32) -> Self {
    Self(offset)
  }

  /// 原版 id() — offset / sizeof(slot) / kSlotsPerId (紧凑 id, 供 side-table)
  #[inline]
  #[must_use]
  pub const fn id(self) -> u32 {
    self.0 / 8 / SLOTS_PER_ID as u32
  }

  /// 原版 hash() — Invalid 也返回 id
  #[inline]
  #[must_use]
  pub const fn hash(self) -> u32 {
    self.0 / 8 / SLOTS_PER_ID as u32
  }

  /// 原版 offset()
  #[inline]
  #[must_use]
  pub const fn offset(self) -> u32 {
    self.0 & UNMASK_GENERATION_MASK
  }

  /// 原版 valid()
  #[inline]
  #[must_use]
  pub const fn valid(self) -> bool {
    self.0 != Self::INVALID.0
  }

  /// 原版 IsExternalId
  #[inline]
  #[must_use]
  pub const fn is_external_id(self) -> bool {
    self.0 % 8 == EXTERNAL_ID_FLAG
  }

  /// 原版 EncodeExternalId — 跨图节点来源 (Turbofan/Maglev) 编码
  #[must_use]
  pub const fn encode_external_id(id: u32) -> Self {
    Self(id * 8 + EXTERNAL_ID_FLAG)
  }

  /// 原版 DecodeExternalId
  #[inline]
  #[must_use]
  pub const fn decode_external_id(self) -> u32 {
    self.0 / 8
  }
}

impl Default for OpIndex {
  fn default() -> Self {
    Self::INVALID
  }
}

/// 原版 BlockIndex — 已绑定块索引 (支配块序号更小)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BlockIndex(u32);

impl BlockIndex {
  /// 原版 Invalid()
  pub const INVALID: BlockIndex = BlockIndex(u32::MAX);

  #[inline]
  #[must_use]
  pub const fn new(id: u32) -> Self {
    Self(id)
  }

  #[inline]
  #[must_use]
  pub const fn id(self) -> u32 {
    self.0
  }

  /// 原版 valid()
  #[inline]
  #[must_use]
  pub const fn valid(self) -> bool {
    self.0 != Self::INVALID.0
  }
}

impl Default for BlockIndex {
  fn default() -> Self {
    Self::INVALID
  }
}

/// 原版 OptionalOpIndex — Rust 侧直接用 Option<OpIndex>;
/// 保留别名以登记原版形态。
pub type OptionalOpIndex = Option<OpIndex>;

// ---------------------------------------------------------------------------
// 抽象类型标签族 (V<T> 的类型层标记; Rust 侧以单元结构体登记)
// ---------------------------------------------------------------------------
/// 原版 WordWithBits<32>
pub struct Word32;

/// 原版 WordWithBits<64>
pub struct Word64;

/// 原版 WordPtr = Word64 (64 位目标)
pub struct WordPtr;

/// 原版 FloatWithBits<32>
pub struct Float32;

/// 原版 FloatWithBits<64>
pub struct Float64;

/// 原版 WordWithBits<128>
pub struct Simd128;

/// 原版 WordWithBits<256>
pub struct Simd256;

/// 原版抽象标签 (Any)
pub struct Compressed;

/// 原版抽象标签 (Any)
pub struct InternalTag;

/// 原版内部标签 (InternalTag 子类)
pub struct EagerFrameState;

/// 原版内部标签 (InternalTag 子类)
pub struct LazyFrameState;

/// 原版 AnyFrameState = UntaggedUnion<EagerFrameState, LazyFrameState>
pub struct AnyFrameState;

/// 原版 v_traits<T>::rep 映射表 — 抽象类型 → 寄存器表示 (None = nullrep)
pub fn abstract_rep(name: &str) -> Option<RegisterRepresentation> {
  match name {
    "Any" => None,
    "None" => None,
    "Compressed" => Some(RegisterRepresentation::Compressed),
    "Word32" => Some(RegisterRepresentation::Word32),
    "Word64" => Some(RegisterRepresentation::Word64),
    "Float32" => Some(RegisterRepresentation::Float32),
    "Float64" => Some(RegisterRepresentation::Float64),
    "Smi" => Some(RegisterRepresentation::Tagged),
    "Simd128" => Some(RegisterRepresentation::Simd128),
    "Simd256" => Some(RegisterRepresentation::Simd256),
    _ => None,
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SLOTS_PER_ID == 2);
  assert!(OpIndex::INVALID.0 == u32::MAX);
  assert!(OpIndex::from_offset(16).id() == 1);
  assert!(!OpIndex::INVALID.valid());
  assert!(OpIndex::encode_external_id(3).is_external_id());
  assert!(OpIndex::encode_external_id(3).decode_external_id() == 3);
  assert!(BlockIndex::INVALID.id() == u32::MAX);
  assert!(GENERATION_MASK == 2);
};
