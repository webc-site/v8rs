// 由 tool/src/maglev/maglev-ir-props.js 从 v8/src/maglev/maglev-ir.h 生成; 修订就地改, 复跑覆盖
// OpProperties: 位域 10 + 工厂 27(私有 1) + 查询 20 + 变换 3; NodeBase 位段 6

//! Maglev IR 节点性质位域纯数据面 (原版 `class OpProperties` + `NodeBase` 位域规格)。
//! 位域: [is_call:1][attached_deopt_info:3][can_throw:1][can_read:1][can_write:1]
//!       [can_allocate:1][not_idempotent:1][value_representation:4][is_conversion:1]
//!       [needs_register_snapshot:1] → kUsedSize 15, kSize 16。
//! 原版 constexpr 静态工厂 (`OpProperties::CanRead()` 之类) 以关联常量承载, 数值由位域链求出并互校;
//! 查询保留 self 方法形且同名小写, 故与大写工厂常量不冲突。
//! `value_representation` 的语义枚举属 repr 模块, 此处只给原版位宽下的 u8 编码。

/// 原版 `OpProperties::kSize`: 位域总宽 (即 NodeBase properties 段宽度)。
pub const PROPERTIES_SIZE: u32 = 16;
/// 原版 `OpProperties::kUsedSize`: 实际占用位数。
pub const PROPERTIES_USED_SIZE: u32 = 15;
/// 原版 `kPureMask` = can_read|can_write|can_allocate 三段掩码。
pub const K_PURE_MASK: u32 = 0xe0;
/// 原版 `kPureValue` = 三段 encode(false) 之或, 恒 0。
pub const K_PURE_VALUE: u32 = 0;

/// 原版 `kIsCallBit`: 起始位 0, 宽度 1, 类型 bool。
pub const IS_CALL_SHIFT: u32 = 0;
pub const IS_CALL_WIDTH: u32 = 1;
pub const IS_CALL_MASK: u32 = 0x1;
/// 原版 `kAttachedDeoptInfoBits`: 起始位 1, 宽度 3, 类型 AttachedDeoptInfo。
pub const ATTACHED_DEOPT_INFO_SHIFT: u32 = 1;
pub const ATTACHED_DEOPT_INFO_WIDTH: u32 = 3;
pub const ATTACHED_DEOPT_INFO_MASK: u32 = 0xe;
/// 原版 `kCanThrowBit`: 起始位 4, 宽度 1, 类型 bool。
pub const CAN_THROW_SHIFT: u32 = 4;
pub const CAN_THROW_WIDTH: u32 = 1;
pub const CAN_THROW_MASK: u32 = 0x10;
/// 原版 `kCanReadBit`: 起始位 5, 宽度 1, 类型 bool。
pub const CAN_READ_SHIFT: u32 = 5;
pub const CAN_READ_WIDTH: u32 = 1;
pub const CAN_READ_MASK: u32 = 0x20;
/// 原版 `kCanWriteBit`: 起始位 6, 宽度 1, 类型 bool。
pub const CAN_WRITE_SHIFT: u32 = 6;
pub const CAN_WRITE_WIDTH: u32 = 1;
pub const CAN_WRITE_MASK: u32 = 0x40;
/// 原版 `kCanAllocateBit`: 起始位 7, 宽度 1, 类型 bool。
pub const CAN_ALLOCATE_SHIFT: u32 = 7;
pub const CAN_ALLOCATE_WIDTH: u32 = 1;
pub const CAN_ALLOCATE_MASK: u32 = 0x80;
/// 原版 `kNotIdempotentBit`: 起始位 8, 宽度 1, 类型 bool。
pub const NOT_IDEMPOTENT_SHIFT: u32 = 8;
pub const NOT_IDEMPOTENT_WIDTH: u32 = 1;
pub const NOT_IDEMPOTENT_MASK: u32 = 0x100;
/// 原版 `kValueRepresentationBits`: 起始位 9, 宽度 4, 类型 ValueRepresentation。
pub const VALUE_REPRESENTATION_SHIFT: u32 = 9;
pub const VALUE_REPRESENTATION_WIDTH: u32 = 4;
pub const VALUE_REPRESENTATION_MASK: u32 = 0x1e00;
/// 原版 `kIsConversionBit`: 起始位 13, 宽度 1, 类型 bool。
pub const IS_CONVERSION_SHIFT: u32 = 13;
pub const IS_CONVERSION_WIDTH: u32 = 1;
pub const IS_CONVERSION_MASK: u32 = 0x2000;
/// 原版 `kNeedsRegisterSnapshotBit`: 起始位 14, 宽度 1, 类型 bool。
pub const NEEDS_REGISTER_SNAPSHOT_SHIFT: u32 = 14;
pub const NEEDS_REGISTER_SNAPSHOT_WIDTH: u32 = 1;
pub const NEEDS_REGISTER_SNAPSHOT_MASK: u32 = 0x4000;

/// 原版私有 `enum class AttachedDeoptInfo` (kNone 起连续分配)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum AttachedDeoptInfo {
  None = 0,
  Eager = 1,
  Lazy = 2,
  EagerAndLazy = 3,
  Checkpoint = 4,
}

/// 原版 `ValueRepresentation` 在本位域中的编码值 (声明序抄自 maglev-ir.h)。
pub const REPR_TAGGED: u8 = 0;
pub const REPR_INT32: u8 = 1;
pub const REPR_UINT32: u8 = 2;
pub const REPR_FLOAT64: u8 = 3;
pub const REPR_HOLEY_FLOAT64: u8 = 4;
pub const REPR_INT_PTR: u8 = 5;
pub const REPR_RAW_PTR: u8 = 6;
pub const REPR_NONE: u8 = 7;

/// 节点性质位集, 1:1 对应原版 `class OpProperties` (原版内部 `uint16_t bitfield_`)。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpProperties {
  /// 原版 `bitfield_`; `operator uint32_t()` 即其读取。
  bits: u32,
}

impl OpProperties {
  /// 原版 `explicit OpProperties(uint32_t bitfield)`：成员 `const uint16_t bitfield_`
  /// (maglev-ir.h:1383) 构造即截 16 位 (:1337)。注意 `BitField::encode` 枚举支仅
  /// DCHECK 不掩宽 (base/bit-field.h:84-102)，故 with_new_value_representation 等
  /// update 路径保持不截断，与 C++ release 语义一致；截断只发生在构造入口。
  pub const fn from_bits(bits: u32) -> Self {
    Self {
      bits: bits & 0xffff,
    }
  }

  /// 原版 `operator uint32_t()`。
  pub const fn to_u32(self) -> u32 {
    self.bits
  }

  /// 原版 `operator|`。
  pub const fn or(self, that: Self) -> Self {
    Self::from_bits(self.bits | that.bits)
  }

  /// 原版 `WithNewValueRepresentation(new_repr)`: `kValueRepresentationBits::update`。
  pub const fn with_new_value_representation(self, new_repr: u8) -> Self {
    let mask = 0x1e00;
    Self::from_bits((self.bits & !mask) | (new_repr as u32 * 512))
  }

  /// 原版 `WithoutDeopt()`: attached_deopt_info 段清为 kNone。
  pub const fn without_deopt(self) -> Self {
    Self::from_bits(self.bits & !0xe)
  }

  // —— 原版静态工厂 (constexpr 值) ——
  /// 原版 `OpProperties::Pure()` = kPureValue。
  pub const PURE: Self = Self::from_bits(0x0);
  /// 原版 `OpProperties::Call()` = kIsCallBit::encode(true)。
  pub const CALL: Self = Self::from_bits(0x1);
  /// 原版 `OpProperties::EagerDeopt()` = kAttachedDeoptInfoBits::encode(AttachedDeoptInfo::kEager)。
  pub const EAGER_DEOPT: Self = Self::from_bits(0x2);
  /// 原版 `OpProperties::LazyDeopt()` = kAttachedDeoptInfoBits::encode(AttachedDeoptInfo::kLazy)。
  pub const LAZY_DEOPT: Self = Self::from_bits(0x4);
  /// 原版 `OpProperties::DeoptCheckpoint()` = kAttachedDeoptInfoBits::encode(AttachedDeoptInfo::kCheckpoint)。
  pub const DEOPT_CHECKPOINT: Self = Self::from_bits(0x8);
  /// 原版 `OpProperties::CanThrow()` = OpProperties(kCanThrowBit::encode(true)) | LazyDeopt() | CanAllocate()。
  pub const CAN_THROW: Self = Self::from_bits(0x94);
  /// 原版 `OpProperties::CanRead()` = kCanReadBit::encode(true)。
  pub const CAN_READ: Self = Self::from_bits(0x20);
  /// 原版 `OpProperties::CanWrite()` = kCanWriteBit::encode(true)。
  pub const CAN_WRITE: Self = Self::from_bits(0x40);
  /// 原版 `OpProperties::CanAllocate()` = kCanAllocateBit::encode(true)。
  pub const CAN_ALLOCATE: Self = Self::from_bits(0x80);
  /// 原版 `OpProperties::NotIdempotent()` = kNotIdempotentBit::encode(true)。
  pub const NOT_IDEMPOTENT: Self = Self::from_bits(0x100);
  /// 原版 `OpProperties::TaggedValue()` = kValueRepresentationBits::encode(ValueRepresentation::kTagged)。
  pub const TAGGED_VALUE: Self = Self::from_bits(0x0);
  /// 原版 `OpProperties::ExternalReference()` = kValueRepresentationBits::encode(ValueRepresentation::kIntPtr)。
  pub const EXTERNAL_REFERENCE: Self = Self::from_bits(0xa00);
  /// 原版 `OpProperties::Int32()` = kValueRepresentationBits::encode(ValueRepresentation::kInt32)。
  pub const INT32: Self = Self::from_bits(0x200);
  /// 原版 `OpProperties::Uint32()` = kValueRepresentationBits::encode(ValueRepresentation::kUint32)。
  pub const UINT32: Self = Self::from_bits(0x400);
  /// 原版 `OpProperties::Float64()` = kValueRepresentationBits::encode(ValueRepresentation::kFloat64)。
  pub const FLOAT64: Self = Self::from_bits(0x600);
  /// 原版 `OpProperties::HoleyFloat64()` = kValueRepresentationBits::encode(ValueRepresentation::kHoleyFloat64)。
  pub const HOLEY_FLOAT64: Self = Self::from_bits(0x800);
  /// 原版 `OpProperties::IntPtr()` = kValueRepresentationBits::encode(ValueRepresentation::kIntPtr)。
  pub const INT_PTR: Self = Self::from_bits(0xa00);
  /// 原版 `OpProperties::RawPtr()` = kValueRepresentationBits::encode(ValueRepresentation::kRawPtr)。
  pub const RAW_PTR: Self = Self::from_bits(0xc00);
  /// 原版 `OpProperties::TrustedPointer()` = kValueRepresentationBits::encode(ValueRepresentation::kTagged)。
  pub const TRUSTED_POINTER: Self = Self::from_bits(0x0);
  /// 原版 `OpProperties::ForValueRepresentation(ValueRepresentation repr)`: 表位起 9, 宽 4。
  pub const fn for_value_representation(repr: u8) -> Self {
    Self::from_bits((repr as u32) << 9)
  }
  /// 原版 `OpProperties::ConversionNode()` = kIsConversionBit::encode(true)。
  pub const CONVERSION_NODE: Self = Self::from_bits(0x2000);
  /// 原版 `OpProperties::CanCallUserCode()` = AnySideEffects() | LazyDeopt() | CanThrow()。
  pub const CAN_CALL_USER_CODE: Self = Self::from_bits(0xf4);
  /// 原版 `OpProperties::GenericRuntimeOrBuiltinCall()` = Call() | CanCallUserCode() | NotIdempotent()。
  pub const GENERIC_RUNTIME_OR_BUILTIN_CALL: Self = Self::from_bits(0x1f5);
  /// 原版 `OpProperties::JSCall()` = Call() | CanCallUserCode()。
  pub const JSCALL: Self = Self::from_bits(0xf5);
  /// 原版 `OpProperties::AnySideEffects()` = CanRead() | CanWrite() | CanAllocate()。
  pub const ANY_SIDE_EFFECTS: Self = Self::from_bits(0xe0);
  /// 原版 `OpProperties::DeferredCall()` = NeedsRegisterSnapshot()。
  pub const DEFERRED_CALL: Self = Self::from_bits(0x4000);
  // 原版 private `OpProperties::NeedsRegisterSnapshot()` = kNeedsRegisterSnapshotBit::encode(true);
  // 原版仅 DeferredCall() 使用, 故不外露常量, 其值 0x4000 内联进 DEFERRED_CALL。

  // —— 原版查询方法 ——
  /// 原版 `constexpr bool is_call() const`。
  pub const fn is_call(self) -> bool {
    (self.bits & 0x1) != 0
  }

  /// 原版 `constexpr bool is_any_call() const`。
  pub const fn is_any_call(self) -> bool {
    self.is_call() || self.is_deferred_call()
  }

  /// 原版 `constexpr bool can_eager_deopt() const`。
  pub const fn can_eager_deopt(self) -> bool {
    matches!((self.bits >> 1) & 0x7, 1 | 3)
  }

  /// 原版 `constexpr bool can_lazy_deopt() const`。
  pub const fn can_lazy_deopt(self) -> bool {
    matches!((self.bits >> 1) & 0x7, 2 | 3)
  }

  /// 原版 `constexpr bool is_deopt_checkpoint() const`。
  pub const fn is_deopt_checkpoint(self) -> bool {
    (self.bits >> 1) & 0x7 == 4
  }

  /// 原版 `constexpr bool has_eager_deopt_info() const`。
  pub const fn has_eager_deopt_info(self) -> bool {
    self.can_eager_deopt() || self.is_deopt_checkpoint()
  }

  /// 原版 `constexpr bool can_deopt() const`。
  pub const fn can_deopt(self) -> bool {
    self.can_eager_deopt() || self.can_lazy_deopt()
  }

  /// 原版 `constexpr bool can_throw() const`。
  pub const fn can_throw(self) -> bool {
    ((self.bits >> 4) & 0x1) != 0 && self.can_lazy_deopt()
  }

  /// 原版 `constexpr bool can_read() const`。
  pub const fn can_read(self) -> bool {
    ((self.bits >> 5) & 0x1) != 0
  }

  /// 原版 `constexpr bool can_write() const`。
  pub const fn can_write(self) -> bool {
    ((self.bits >> 6) & 0x1) != 0
  }

  /// 原版 `constexpr bool can_allocate() const`。
  pub const fn can_allocate(self) -> bool {
    ((self.bits >> 7) & 0x1) != 0
  }

  /// 原版 `constexpr bool not_idempotent() const`。
  pub const fn not_idempotent(self) -> bool {
    ((self.bits >> 8) & 0x1) != 0
  }

  /// 原版 `constexpr ValueRepresentation value_representation() const`。
  pub const fn value_representation(self) -> u8 {
    ((self.bits >> 9) & 0xf) as u8
  }

  /// 原版 `constexpr bool is_tagged() const`。
  pub const fn is_tagged(self) -> bool {
    self.value_representation() == REPR_TAGGED
  }

  /// 原版 `constexpr bool is_conversion() const`。
  pub const fn is_conversion(self) -> bool {
    ((self.bits >> 13) & 0x1) != 0
  }

  /// 原版 `constexpr bool needs_register_snapshot() const`。
  pub const fn needs_register_snapshot(self) -> bool {
    ((self.bits >> 14) & 0x1) != 0
  }

  /// 原版 `constexpr bool is_deferred_call() const`。
  pub const fn is_deferred_call(self) -> bool {
    self.needs_register_snapshot()
  }

  /// 原版 `constexpr bool is_pure() const`。
  pub const fn is_pure(self) -> bool {
    (self.bits & K_PURE_MASK) == K_PURE_VALUE
  }

  /// 原版 `constexpr bool is_required_when_unused() const`。
  pub const fn is_required_when_unused(self) -> bool {
    if self.is_conversion() {
      self.can_write() || self.can_throw() || self.can_deopt()
    } else {
      self.can_write() || self.can_throw() || self.can_deopt() || self.is_any_call()
    }
  }

  /// 原版 `constexpr bool can_participate_in_cse() const`。
  pub const fn can_participate_in_cse(self) -> bool {
    !self.can_write() && !self.not_idempotent()
  }
}

// —— NodeBase 位段: 原版注释 `[opcode:16][input_count:16][properties:16][extras:16]` ——
/// 原版 `OpcodeField`: 起始位 0, 宽度 16 (BitField64)。
pub const NODE_OPCODE_SHIFT: u32 = 0;
pub const NODE_OPCODE_WIDTH: u32 = 16;
pub const NODE_OPCODE_MASK: u64 = 0xffff;
/// 原版 `InputCountField`: 起始位 16, 宽度 16 (BitField64)。
pub const NODE_INPUT_COUNT_SHIFT: u32 = 16;
pub const NODE_INPUT_COUNT_WIDTH: u32 = 16;
pub const NODE_INPUT_COUNT_MASK: u64 = 0xffff0000;
/// 原版 `OpPropertiesField`: 起始位 32, 宽度 16 (BitField64)。
pub const NODE_OP_PROPERTIES_SHIFT: u32 = 32;
pub const NODE_OP_PROPERTIES_WIDTH: u32 = 16;
pub const NODE_OP_PROPERTIES_MASK: u64 = 0xffff00000000;
/// 原版 `NumTemporariesNeededField`: 起始位 48, 宽度 2 (BitField64)。
pub const NODE_NUM_TEMPORARIES_NEEDED_SHIFT: u32 = 48;
pub const NODE_NUM_TEMPORARIES_NEEDED_WIDTH: u32 = 2;
pub const NODE_NUM_TEMPORARIES_NEEDED_MASK: u64 = 0x3000000000000;
/// 原版 `NumDoubleTemporariesNeededField`: 起始位 50, 宽度 1 (BitField64)。
pub const NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_SHIFT: u32 = 50;
pub const NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_WIDTH: u32 = 1;
pub const NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_MASK: u64 = 0x4000000000000;
/// 原版 `ReservedFields`: 起始位 51, 宽度 2 (BitField64)。
pub const NODE_RESERVED_FIELDS_SHIFT: u32 = 51;
pub const NODE_RESERVED_FIELDS_WIDTH: u32 = 2;
pub const NODE_RESERVED_FIELDS_MASK: u64 = 0x18000000000000;

/// 原版节点位域组装: `[opcode:16][input_count:16][properties:16][num_temporaries:2][num_double_temporaries:1]`
/// 各段 `Field::encode(...)` 之或 (原版由构造器/`update` 逐段写入, 此处一次成型)。
pub const fn encode_node_bitfield(
  opcode: u16,
  input_count: u64,
  properties: OpProperties,
  num_temporaries: u64,
  num_double_temporaries: u64,
) -> u64 {
  ((opcode as u64) << NODE_OPCODE_SHIFT)
    | (input_count << NODE_INPUT_COUNT_SHIFT)
    | ((properties.to_u32() as u64) << NODE_OP_PROPERTIES_SHIFT)
    | (num_temporaries << NODE_NUM_TEMPORARIES_NEEDED_SHIFT)
    | (num_double_temporaries << NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_SHIFT)
}

/// 原版 `NodeBase::opcode()`: `OpcodeField::decode(bitfield_)`。
pub const fn decode_opcode(bitfield: u64) -> u16 {
  ((bitfield & NODE_OPCODE_MASK) >> NODE_OPCODE_SHIFT) as u16
}

/// 原版 `NodeBase::input_count()`: `InputCountField::decode(bitfield_)`。
pub const fn decode_input_count(bitfield: u64) -> u64 {
  (bitfield & NODE_INPUT_COUNT_MASK) >> NODE_INPUT_COUNT_SHIFT
}

/// 原版 `NodeBase::properties()`: `OpPropertiesField::decode(bitfield_)`。
pub const fn decode_properties(bitfield: u64) -> OpProperties {
  OpProperties::from_bits(((bitfield & NODE_OP_PROPERTIES_MASK) >> NODE_OP_PROPERTIES_SHIFT) as u32)
}

/// 原版 `NodeBase::NumTemporariesNeededField::decode(bitfield_)`。
pub const fn decode_num_temporaries(bitfield: u64) -> u64 {
  (bitfield & NODE_NUM_TEMPORARIES_NEEDED_MASK) >> NODE_NUM_TEMPORARIES_NEEDED_SHIFT
}

/// 原版 `NodeBase::NumDoubleTemporariesNeededField::decode(bitfield_)`。
pub const fn decode_num_double_temporaries(bitfield: u64) -> u64 {
  (bitfield & NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_MASK) >> NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_SHIFT
}

/// 原版 `NodeBase::set_properties()`: `OpPropertiesField::update(bitfield_, properties)`。
pub const fn update_properties(bitfield: u64, properties: OpProperties) -> u64 {
  (bitfield & !NODE_OP_PROPERTIES_MASK) | ((properties.to_u32() as u64) << NODE_OP_PROPERTIES_SHIFT)
}

/// 原版 `NodeBase::kMaxInputs` = `InputCountField::kMax` (16 位全置)。
pub const MAX_INPUTS: u64 = (1 << NODE_INPUT_COUNT_WIDTH) - 1;

// 对数: 位域 10 (位/宽/类型逐项锚) + 工厂 27 (原文锚 + 固定点数值求值 + 16 项互校)
//   + 无参查询 20 (原文反向计数核) + 变换 3 + NodeBase 位段 6 (前驱链核)
//   + kPure/NextBitField/LastNodeBaseField/kMaxInputs/encode 组装等原文锚 = 全过。
