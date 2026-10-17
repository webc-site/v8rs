// 由 tool/src/compiler/use-info.js 从 v8/src/codegen/machine-type.h (最小子集) 生成; 修订就地改, 复跑覆盖

//! MachineRepresentation / MachineSemantic / MachineType — codegen 层最小子集
//!
//! 完整 machine-type.h (ElementSize*/MachineReprToString 等) 属 codegen 迁移层;
//! 此处仅内联 use-info.h 所需面, 后续独立 crate 时整体迁出。
//! 64 位目标: PointerRepresentation = Word64 (kSystemPointerSize == 8)。

/// 原版 MachineRepresentation — 表示类别 (integral 连续升序, FP/SIMD 殿后)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MachineRepresentation {
  /// kNone
  None = 0,
  /// kBit
  Bit = 1,
  /// kWord8
  Word8 = 2,
  /// kWord16
  Word16 = 3,
  /// kWord32
  Word32 = 4,
  /// kWord64
  Word64 = 5,
  /// kMapWord
  MapWord = 6,
  /// kTaggedSigned
  TaggedSigned = 7,
  /// kTaggedPointer
  TaggedPointer = 8,
  /// kTagged
  Tagged = 9,
  /// kCompressedPointer
  CompressedPointer = 10,
  /// kCompressed
  Compressed = 11,
  /// kProtectedPointer
  ProtectedPointer = 12,
  /// kIndirectPointer
  IndirectPointer = 13,
  /// kSandboxedPointer
  SandboxedPointer = 14,
  /// kFloat16RawBits
  Float16RawBits = 15,
  /// kFloat16
  Float16 = 16,
  /// kFloat32
  Float32 = 17,
  /// kFloat64
  Float64 = 18,
  /// kSimd128
  Simd128 = 19,
  /// kSimd256
  Simd256 = 20,
}

/// 原版 MachineSemantic — 语义类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MachineSemantic {
  /// kNone
  None = 0,
  /// kBool
  Bool = 1,
  /// kInt32
  Int32 = 2,
  /// kUint32
  Uint32 = 3,
  /// kInt64
  Int64 = 4,
  /// kUint64
  Uint64 = 5,
  /// kSignedBigInt64
  SignedBigInt64 = 6,
  /// kUnsignedBigInt64
  UnsignedBigInt64 = 7,
  /// kNumber
  Number = 8,
  /// kHoleyFloat64
  HoleyFloat64 = 9,
  /// kAny
  Any = 10,
}

/// 原版 MachineType — (representation, semantic) 值对
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MachineType {
  rep: MachineRepresentation,
  sem: MachineSemantic,
}

impl MachineType {
  /// 原 representation()
  pub const fn representation(self) -> MachineRepresentation {
    self.rep
  }
  /// 原 semantic()
  pub const fn semantic(self) -> MachineSemantic {
    self.sem
  }
  /// 原 IsNone
  pub const fn is_none(self) -> bool {
    matches!(self.rep, MachineRepresentation::None)
  }
  /// 原 IsMapWord
  pub const fn is_map_word(self) -> bool {
    matches!(self.rep, MachineRepresentation::MapWord)
  }
  /// 原 IsSigned
  pub const fn is_signed(self) -> bool {
    matches!(self.sem, MachineSemantic::Int32 | MachineSemantic::Int64)
  }
  /// 原 IsUnsigned
  pub const fn is_unsigned(self) -> bool {
    matches!(self.sem, MachineSemantic::Uint32 | MachineSemantic::Uint64)
  }
  /// 原 IsTagged
  pub const fn is_tagged(self) -> bool {
    matches!(
      self.rep,
      MachineRepresentation::TaggedPointer
        | MachineRepresentation::TaggedSigned
        | MachineRepresentation::Tagged
    )
  }
  /// 原 IsTaggedSigned
  pub const fn is_tagged_signed(self) -> bool {
    matches!(self.rep, MachineRepresentation::TaggedSigned)
  }
  /// 原 IsTaggedPointer
  pub const fn is_tagged_pointer(self) -> bool {
    matches!(self.rep, MachineRepresentation::TaggedPointer)
  }
  /// 原 IsCompressed
  pub const fn is_compressed(self) -> bool {
    matches!(
      self.rep,
      MachineRepresentation::CompressedPointer | MachineRepresentation::Compressed
    )
  }
  /// 原 IsCompressedPointer
  pub const fn is_compressed_pointer(self) -> bool {
    matches!(self.rep, MachineRepresentation::CompressedPointer)
  }
  /// 原 IsIndirectPointer
  pub const fn is_indirect_pointer(self) -> bool {
    matches!(self.rep, MachineRepresentation::IndirectPointer)
  }

  /// 原 PointerRepresentation — 64 位目标恒 Word64
  pub const fn pointer_representation() -> MachineRepresentation {
    MachineRepresentation::Word64
  }
  /// 原 UintPtr / IntPtr (64 位)
  pub const fn uint_ptr() -> Self {
    Self::uint64()
  }
  pub const fn int_ptr() -> Self {
    Self::int64()
  }
  pub const fn int8() -> Self {
    Self::new(MachineRepresentation::Word8, MachineSemantic::Int32)
  }
  pub const fn uint8() -> Self {
    Self::new(MachineRepresentation::Word8, MachineSemantic::Uint32)
  }
  pub const fn int16() -> Self {
    Self::new(MachineRepresentation::Word16, MachineSemantic::Int32)
  }
  pub const fn uint16() -> Self {
    Self::new(MachineRepresentation::Word16, MachineSemantic::Uint32)
  }
  pub const fn int32() -> Self {
    Self::new(MachineRepresentation::Word32, MachineSemantic::Int32)
  }
  pub const fn uint32() -> Self {
    Self::new(MachineRepresentation::Word32, MachineSemantic::Uint32)
  }
  pub const fn int64() -> Self {
    Self::new(MachineRepresentation::Word64, MachineSemantic::Int64)
  }
  pub const fn uint64() -> Self {
    Self::new(MachineRepresentation::Word64, MachineSemantic::Uint64)
  }
  pub const fn float32() -> Self {
    Self::new(MachineRepresentation::Float32, MachineSemantic::Number)
  }
  pub const fn float64() -> Self {
    Self::new(MachineRepresentation::Float64, MachineSemantic::Number)
  }
  pub const fn new(representation: MachineRepresentation, semantic: MachineSemantic) -> Self {
    Self {
      rep: representation,
      sem: semantic,
    }
  }
  /// 原 TaggedSigned()
  pub const fn tagged_signed() -> Self {
    Self::new(MachineRepresentation::TaggedSigned, MachineSemantic::Any)
  }
  /// 原 TaggedPointer()
  pub const fn tagged_pointer() -> Self {
    Self::new(MachineRepresentation::TaggedPointer, MachineSemantic::Any)
  }
  /// 原 AnyTagged()
  pub const fn any_tagged() -> Self {
    Self::new(MachineRepresentation::Tagged, MachineSemantic::Any)
  }
  /// 原 Pointer() (64 位)
  pub const fn pointer() -> Self {
    Self::new(Self::pointer_representation(), MachineSemantic::None)
  }
  /// 原 Float16()
  pub const fn float16() -> Self {
    Self::new(MachineRepresentation::Float16, MachineSemantic::Number)
  }
  /// 原 SandboxedPointer()
  pub const fn sandboxed_pointer() -> Self {
    Self::new(
      MachineRepresentation::SandboxedPointer,
      MachineSemantic::Int64,
    )
  }
  /// 原 ProtectedPointer()
  pub const fn protected_pointer() -> Self {
    Self::new(
      MachineRepresentation::ProtectedPointer,
      MachineSemantic::Any,
    )
  }
  /// 原 IndirectPointer()
  pub const fn indirect_pointer() -> Self {
    Self::new(
      MachineRepresentation::IndirectPointer,
      MachineSemantic::Int32,
    )
  }
  /// 原 Simd128()
  pub const fn simd128() -> Self {
    Self::new(MachineRepresentation::Simd128, MachineSemantic::None)
  }
  /// 原 Simd256()
  pub const fn simd256() -> Self {
    Self::new(MachineRepresentation::Simd256, MachineSemantic::None)
  }
  /// 原 Bool()
  pub const fn bool_type() -> Self {
    Self::new(MachineRepresentation::Bit, MachineSemantic::Bool)
  }
  /// 原 None()
  pub const fn none() -> Self {
    Self::new(MachineRepresentation::None, MachineSemantic::None)
  }
}
