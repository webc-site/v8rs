// 由 tool/src/compiler/turboshaft/representations.h.js 从 v8/src/compiler/turboshaft/representations.{h,cc} 生成; 修订就地改, 复跑覆盖
// 表示体系: 寄存器/内存表示枚举与属性表 (x64+arm64, 压缩指针开)

// MachineRepresentation / MachineType 来自 v8-compiler-graph::machine_type
// (原版 src/codegen/machine-type.h)。
use v8_compiler_graph::{MachineRepresentation, MachineType};

/// 原版 MaybeRegisterRepresentation — 可空寄存器表示 (含 kNone 哨兵)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MaybeRegisterRepresentation {
  /// kWord32
  Word32 = 0,
  /// kWord64
  Word64 = 1,
  /// kFloat32
  Float32 = 2,
  /// kFloat64
  Float64 = 3,
  /// kTagged
  Tagged = 4,
  /// kCompressed
  Compressed = 5,
  /// kSimd128
  Simd128 = 6,
  /// kSimd256
  Simd256 = 7,
  /// kNone
  None = 8,
}

impl MaybeRegisterRepresentation {
  /// 原版 kInvalid 哨兵 (static_cast<Enum>(-1)); Rust 侧以 Option 表达
  pub const INVALID: u8 = 255;

  /// 原版 IsWord
  #[inline]
  #[must_use]
  pub const fn is_word(self) -> bool {
    match self {
      MaybeRegisterRepresentation::Word32 => true,
      MaybeRegisterRepresentation::Word64 => true,
      MaybeRegisterRepresentation::Float32 => false,
      MaybeRegisterRepresentation::Float64 => false,
      MaybeRegisterRepresentation::Tagged => false,
      MaybeRegisterRepresentation::Compressed => false,
      MaybeRegisterRepresentation::Simd128 => false,
      MaybeRegisterRepresentation::Simd256 => false,
      MaybeRegisterRepresentation::None => false,
    }
  }

  /// 原版 IsFloat
  #[inline]
  #[must_use]
  pub const fn is_float(self) -> bool {
    match self {
      MaybeRegisterRepresentation::Float32 => true,
      MaybeRegisterRepresentation::Float64 => true,
      MaybeRegisterRepresentation::Word32 => false,
      MaybeRegisterRepresentation::Word64 => false,
      MaybeRegisterRepresentation::Tagged => false,
      MaybeRegisterRepresentation::Compressed => false,
      MaybeRegisterRepresentation::Simd128 => false,
      MaybeRegisterRepresentation::Simd256 => false,
      MaybeRegisterRepresentation::None => false,
    }
  }

  /// 原版 IsTaggedOrCompressed
  #[inline]
  #[must_use]
  pub const fn is_tagged_or_compressed(self) -> bool {
    match self {
      MaybeRegisterRepresentation::Tagged => true,
      MaybeRegisterRepresentation::Compressed => true,
      MaybeRegisterRepresentation::Word32 => false,
      MaybeRegisterRepresentation::Word64 => false,
      MaybeRegisterRepresentation::Float32 => false,
      MaybeRegisterRepresentation::Float64 => false,
      MaybeRegisterRepresentation::Simd128 => false,
      MaybeRegisterRepresentation::Simd256 => false,
      MaybeRegisterRepresentation::None => false,
    }
  }

  /// 原版 bit_width (位宽; kSystemPointerSize=8 展开)
  #[inline]
  #[must_use]
  pub const fn bit_width(self) -> u16 {
    match self {
      MaybeRegisterRepresentation::Word32 => 32,
      MaybeRegisterRepresentation::Word64 => 64,
      MaybeRegisterRepresentation::Float32 => 32,
      MaybeRegisterRepresentation::Float64 => 64,
      MaybeRegisterRepresentation::Tagged => 64,
      MaybeRegisterRepresentation::Compressed => 32,
      MaybeRegisterRepresentation::Simd128 => 128,
      MaybeRegisterRepresentation::Simd256 => 256,
      MaybeRegisterRepresentation::None => 0,
    }
  }

  /// 原版 machine_representation
  #[inline]
  #[must_use]
  pub const fn machine_representation(self) -> MachineRepresentation {
    match self {
      MaybeRegisterRepresentation::Word32 => MachineRepresentation::Word32,
      MaybeRegisterRepresentation::Word64 => MachineRepresentation::Word64,
      MaybeRegisterRepresentation::Float32 => MachineRepresentation::Float32,
      MaybeRegisterRepresentation::Float64 => MachineRepresentation::Float64,
      MaybeRegisterRepresentation::Tagged => MachineRepresentation::Tagged,
      MaybeRegisterRepresentation::Compressed => MachineRepresentation::Compressed,
      MaybeRegisterRepresentation::Simd128 => MachineRepresentation::Simd128,
      MaybeRegisterRepresentation::Simd256 => MachineRepresentation::Simd256,
      MaybeRegisterRepresentation::None => MachineRepresentation::None,
    }
  }

  /// 原版 MaxUnsignedValue
  #[inline]
  #[must_use]
  pub const fn max_unsigned_value(self) -> u64 {
    match self {
      MaybeRegisterRepresentation::Word32 => u32::MAX as u64,
      MaybeRegisterRepresentation::Word64 => u64::MAX,
      _ => 0,
    }
  }

  /// 原版 operator<< 名 (representations.cc)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      MaybeRegisterRepresentation::Word32 => "Word32",
      MaybeRegisterRepresentation::Word64 => "Word64",
      MaybeRegisterRepresentation::Float32 => "Float32",
      MaybeRegisterRepresentation::Float64 => "Float64",
      MaybeRegisterRepresentation::Tagged => "Tagged",
      MaybeRegisterRepresentation::Compressed => "Compressed",
      MaybeRegisterRepresentation::Simd128 => "Simd128",
      MaybeRegisterRepresentation::Simd256 => "Simd256",
      MaybeRegisterRepresentation::None => "None",
    }
  }
}
/// 原版 RegisterRepresentation — 非空寄存器表示 (MaybeRegisterRepresentation 前 8 项子集)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RegisterRepresentation {
  /// kWord32
  Word32 = 0,
  /// kWord64
  Word64 = 1,
  /// kFloat32
  Float32 = 2,
  /// kFloat64
  Float64 = 3,
  /// kTagged
  Tagged = 4,
  /// kCompressed
  Compressed = 5,
  /// kSimd128
  Simd128 = 6,
  /// kSimd256
  Simd256 = 7,
}

impl RegisterRepresentation {
  /// 上采样为 Maybe 表示
  #[inline]
  #[must_use]
  pub const fn to_maybe(self) -> MaybeRegisterRepresentation {
    match self {
      RegisterRepresentation::Word32 => MaybeRegisterRepresentation::Word32,
      RegisterRepresentation::Word64 => MaybeRegisterRepresentation::Word64,
      RegisterRepresentation::Float32 => MaybeRegisterRepresentation::Float32,
      RegisterRepresentation::Float64 => MaybeRegisterRepresentation::Float64,
      RegisterRepresentation::Tagged => MaybeRegisterRepresentation::Tagged,
      RegisterRepresentation::Compressed => MaybeRegisterRepresentation::Compressed,
      RegisterRepresentation::Simd128 => MaybeRegisterRepresentation::Simd128,
      RegisterRepresentation::Simd256 => MaybeRegisterRepresentation::Simd256,
    }
  }

  /// 原版 FromMachineRepresentation (kMapWord → Tagged, 不支持 map packing;
  /// kIndirectPointer/kSandboxedPointer → WordPtr)
  #[inline]
  #[must_use]
  pub const fn from_machine_representation(rep: MachineRepresentation) -> Self {
    match rep {
      MachineRepresentation::Bit => RegisterRepresentation::Word32,
      MachineRepresentation::Word8 => RegisterRepresentation::Word32,
      MachineRepresentation::Word16 => RegisterRepresentation::Word32,
      MachineRepresentation::Word32 => RegisterRepresentation::Word32,
      MachineRepresentation::Word64 => RegisterRepresentation::Word64,
      MachineRepresentation::TaggedSigned => RegisterRepresentation::Tagged,
      MachineRepresentation::TaggedPointer => RegisterRepresentation::Tagged,
      MachineRepresentation::Tagged => RegisterRepresentation::Tagged,
      MachineRepresentation::ProtectedPointer => RegisterRepresentation::Tagged,
      MachineRepresentation::CompressedPointer => RegisterRepresentation::Compressed,
      MachineRepresentation::Compressed => RegisterRepresentation::Compressed,
      MachineRepresentation::Float16 => RegisterRepresentation::Float32,
      MachineRepresentation::Float32 => RegisterRepresentation::Float32,
      MachineRepresentation::Float64 => RegisterRepresentation::Float64,
      MachineRepresentation::Simd128 => RegisterRepresentation::Simd128,
      MachineRepresentation::Simd256 => RegisterRepresentation::Simd256,
      MachineRepresentation::MapWord => RegisterRepresentation::Tagged,
      // 原版 kIndirectPointer/kSandboxedPointer → WordPtr (64 位目标即 Word64)
      MachineRepresentation::IndirectPointer => RegisterRepresentation::Word64,
      MachineRepresentation::SandboxedPointer => RegisterRepresentation::Word64,
      // 原版 kNone/kFloat16RawBits → UNREACHABLE (该映射不以这些表示调用)
      MachineRepresentation::None | MachineRepresentation::Float16RawBits => {
        unreachable!()
      }
    }
  }

  /// 原版 FromMachineType
  #[inline]
  #[must_use]
  pub const fn from_machine_type(ty: MachineType) -> Self {
    Self::from_machine_representation(ty.representation())
  }

  /// 原版 MapTaggedToWord (压缩指针开: Tagged → Word32)
  #[inline]
  #[must_use]
  pub const fn map_tagged_to_word(self) -> Self {
    if self as u8 == RegisterRepresentation::Tagged as u8 {
      RegisterRepresentation::Word32
    } else {
      self
    }
  }
}

/// 原版 WordRepresentation — RegisterRepresentation 的 Word32/Word64 约束子类型;
/// Rust 侧不另立枚举, 以 RegisterRepresentation + 类型谓词表达。
impl RegisterRepresentation {
  /// 原版 WordRepresentation::MinSignedValue
  #[inline]
  #[must_use]
  pub const fn word_min_signed_value(self) -> i64 {
    match self {
      RegisterRepresentation::Word32 => i32::MIN as i64,
      _ => i64::MIN,
    }
  }

  /// 原版 WordRepresentation::MaxSignedValue
  #[inline]
  #[must_use]
  pub const fn word_max_signed_value(self) -> i64 {
    match self {
      RegisterRepresentation::Word32 => i32::MAX as i64,
      _ => i64::MAX,
    }
  }
}

/// 原版 MemoryRepresentation — 内存加载/存储表示
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MemoryRepresentation {
  /// kInt8
  Int8 = 0,
  /// kUint8
  Uint8 = 1,
  /// kInt16
  Int16 = 2,
  /// kUint16
  Uint16 = 3,
  /// kInt32
  Int32 = 4,
  /// kUint32
  Uint32 = 5,
  /// kInt64
  Int64 = 6,
  /// kUint64
  Uint64 = 7,
  /// kFloat16
  Float16 = 8,
  /// kFloat32
  Float32 = 9,
  /// kFloat64
  Float64 = 10,
  /// kAnyTagged
  AnyTagged = 11,
  /// kTaggedPointer
  TaggedPointer = 12,
  /// kTaggedSigned
  TaggedSigned = 13,
  /// kAnyUncompressedTagged
  AnyUncompressedTagged = 14,
  /// kUncompressedTaggedPointer
  UncompressedTaggedPointer = 15,
  /// kUncompressedTaggedSigned
  UncompressedTaggedSigned = 16,
  /// kProtectedPointer
  ProtectedPointer = 17,
  /// kIndirectPointer
  IndirectPointer = 18,
  /// kSandboxedPointer
  SandboxedPointer = 19,
  /// kTrustedPointer
  TrustedPointer = 20,
  /// kSimd128
  Simd128 = 21,
  /// kSimd256
  Simd256 = 22,
}

impl MemoryRepresentation {
  /// 原版 IsSigned (整数表示符号性)
  #[inline]
  #[must_use]
  pub const fn is_signed(self) -> bool {
    match self {
      MemoryRepresentation::Int8 => true,
      MemoryRepresentation::Int16 => true,
      MemoryRepresentation::Int32 => true,
      MemoryRepresentation::Int64 => true,
      MemoryRepresentation::Uint8 => false,
      MemoryRepresentation::Uint16 => false,
      MemoryRepresentation::Uint32 => false,
      MemoryRepresentation::Uint64 => false,
      _ => false,
    }
  }

  /// 原版 IsCompressibleTagged (指针压缩相关 load/store 判定)
  #[inline]
  #[must_use]
  pub const fn is_compressible_tagged(self) -> bool {
    match self {
      MemoryRepresentation::AnyTagged => true,
      MemoryRepresentation::TaggedPointer => true,
      MemoryRepresentation::TaggedSigned => true,
      MemoryRepresentation::Int8 => false,
      MemoryRepresentation::Int16 => false,
      MemoryRepresentation::Int32 => false,
      MemoryRepresentation::Int64 => false,
      MemoryRepresentation::Uint8 => false,
      MemoryRepresentation::Uint16 => false,
      MemoryRepresentation::Uint32 => false,
      MemoryRepresentation::Uint64 => false,
      MemoryRepresentation::Float16 => false,
      MemoryRepresentation::Float32 => false,
      MemoryRepresentation::Float64 => false,
      MemoryRepresentation::AnyUncompressedTagged => false,
      MemoryRepresentation::UncompressedTaggedPointer => false,
      MemoryRepresentation::UncompressedTaggedSigned => false,
      MemoryRepresentation::IndirectPointer => false,
      MemoryRepresentation::ProtectedPointer => false,
      MemoryRepresentation::SandboxedPointer => false,
      MemoryRepresentation::TrustedPointer => false,
      MemoryRepresentation::Simd128 => false,
      MemoryRepresentation::Simd256 => false,
    }
  }

  /// 原版 ToRegisterRepresentation
  #[inline]
  #[must_use]
  pub const fn to_register_representation(self) -> RegisterRepresentation {
    match self {
      MemoryRepresentation::Int8 => RegisterRepresentation::Word32,
      MemoryRepresentation::Uint8 => RegisterRepresentation::Word32,
      MemoryRepresentation::Int16 => RegisterRepresentation::Word32,
      MemoryRepresentation::Uint16 => RegisterRepresentation::Word32,
      MemoryRepresentation::Int32 => RegisterRepresentation::Word32,
      MemoryRepresentation::Uint32 => RegisterRepresentation::Word32,
      MemoryRepresentation::Int64 => RegisterRepresentation::Word64,
      MemoryRepresentation::Uint64 => RegisterRepresentation::Word64,
      MemoryRepresentation::Float16 => RegisterRepresentation::Float32,
      MemoryRepresentation::Float32 => RegisterRepresentation::Float32,
      MemoryRepresentation::Float64 => RegisterRepresentation::Float64,
      MemoryRepresentation::AnyTagged => RegisterRepresentation::Tagged,
      MemoryRepresentation::TaggedPointer => RegisterRepresentation::Tagged,
      MemoryRepresentation::TaggedSigned => RegisterRepresentation::Tagged,
      MemoryRepresentation::AnyUncompressedTagged => RegisterRepresentation::Tagged,
      MemoryRepresentation::UncompressedTaggedPointer => RegisterRepresentation::Tagged,
      MemoryRepresentation::UncompressedTaggedSigned => RegisterRepresentation::Tagged,
      MemoryRepresentation::IndirectPointer => RegisterRepresentation::Tagged,
      MemoryRepresentation::ProtectedPointer => RegisterRepresentation::Tagged,
      MemoryRepresentation::TrustedPointer => RegisterRepresentation::Tagged,
      MemoryRepresentation::SandboxedPointer => RegisterRepresentation::Word64,
      MemoryRepresentation::Simd128 => RegisterRepresentation::Simd128,
      MemoryRepresentation::Simd256 => RegisterRepresentation::Simd256,
    }
  }

  /// 原版 SizeInBytesLog2 (kTaggedSizeLog2=2, kSystemPointerSizeLog2=3)
  #[inline]
  #[must_use]
  pub const fn size_in_bytes_log2(self) -> u8 {
    match self {
      MemoryRepresentation::Int8 => 0,
      MemoryRepresentation::Uint8 => 0,
      MemoryRepresentation::Int16 => 1,
      MemoryRepresentation::Uint16 => 1,
      MemoryRepresentation::Float16 => 1,
      MemoryRepresentation::Int32 => 2,
      MemoryRepresentation::Uint32 => 2,
      MemoryRepresentation::Float32 => 2,
      MemoryRepresentation::IndirectPointer => 2,
      MemoryRepresentation::TrustedPointer => 2,
      MemoryRepresentation::Int64 => 3,
      MemoryRepresentation::Uint64 => 3,
      MemoryRepresentation::Float64 => 3,
      MemoryRepresentation::SandboxedPointer => 3,
      MemoryRepresentation::AnyTagged => 2, // kTaggedSizeLog2 (压缩指针)
      MemoryRepresentation::TaggedPointer => 2, // kTaggedSizeLog2 (压缩指针)
      MemoryRepresentation::TaggedSigned => 2, // kTaggedSizeLog2 (压缩指针)
      MemoryRepresentation::ProtectedPointer => 2, // kTaggedSizeLog2 (压缩指针)
      MemoryRepresentation::AnyUncompressedTagged => 3, // kSystemPointerSizeLog2
      MemoryRepresentation::UncompressedTaggedPointer => 3, // kSystemPointerSizeLog2
      MemoryRepresentation::UncompressedTaggedSigned => 3, // kSystemPointerSizeLog2
      MemoryRepresentation::Simd128 => 4,
      MemoryRepresentation::Simd256 => 5,
    }
  }

  /// 原版 SizeInBytes
  #[inline]
  #[must_use]
  pub const fn size_in_bytes(self) -> u8 {
    1 << self.size_in_bytes_log2()
  }

  /// 原版 FromMachineRepresentation (整型默认无符号)
  #[inline]
  #[must_use]
  pub const fn from_machine_representation(rep: MachineRepresentation) -> Self {
    match rep {
      MachineRepresentation::Word8 => MemoryRepresentation::Uint8,
      MachineRepresentation::Word16 => MemoryRepresentation::Uint16,
      MachineRepresentation::Word32 => MemoryRepresentation::Uint32,
      MachineRepresentation::Word64 => MemoryRepresentation::Uint64,
      MachineRepresentation::TaggedSigned => MemoryRepresentation::TaggedSigned,
      MachineRepresentation::TaggedPointer => MemoryRepresentation::TaggedPointer,
      MachineRepresentation::Tagged => MemoryRepresentation::AnyTagged,
      MachineRepresentation::Float16 => MemoryRepresentation::Float16,
      MachineRepresentation::Float32 => MemoryRepresentation::Float32,
      MachineRepresentation::Float64 => MemoryRepresentation::Float64,
      MachineRepresentation::SandboxedPointer => MemoryRepresentation::SandboxedPointer,
      MachineRepresentation::Simd128 => MemoryRepresentation::Simd128,
      MachineRepresentation::Simd256 => MemoryRepresentation::Simd256,
      // 原版这些表示 → UNREACHABLE (内存表示体系不含位图/map/压缩/间接/裸 bit 变体)
      MachineRepresentation::None
      | MachineRepresentation::Bit
      | MachineRepresentation::MapWord
      | MachineRepresentation::CompressedPointer
      | MachineRepresentation::Compressed
      | MachineRepresentation::ProtectedPointer
      | MachineRepresentation::IndirectPointer
      | MachineRepresentation::Float16RawBits => {
        unreachable!()
      }
    }
  }

  /// 原版 ToMachineType
  #[inline]
  #[must_use]
  pub const fn to_machine_type(self) -> MachineType {
    match self {
      MemoryRepresentation::Int8 => MachineType::int8(),
      MemoryRepresentation::Uint8 => MachineType::uint8(),
      MemoryRepresentation::Int16 => MachineType::int16(),
      MemoryRepresentation::Uint16 => MachineType::uint16(),
      MemoryRepresentation::Int32 => MachineType::int32(),
      MemoryRepresentation::Uint32 => MachineType::uint32(),
      MemoryRepresentation::Int64 => MachineType::int64(),
      MemoryRepresentation::Uint64 => MachineType::uint64(),
      MemoryRepresentation::Float16 => MachineType::float16(),
      MemoryRepresentation::Float32 => MachineType::float32(),
      MemoryRepresentation::Float64 => MachineType::float64(),
      MemoryRepresentation::AnyTagged => MachineType::any_tagged(),
      MemoryRepresentation::TaggedPointer => MachineType::tagged_pointer(),
      MemoryRepresentation::TaggedSigned => MachineType::tagged_signed(),
      MemoryRepresentation::AnyUncompressedTagged => MachineType::any_tagged(),
      MemoryRepresentation::UncompressedTaggedPointer => MachineType::tagged_pointer(),
      MemoryRepresentation::UncompressedTaggedSigned => MachineType::tagged_signed(),
      MemoryRepresentation::ProtectedPointer => MachineType::protected_pointer(),
      MemoryRepresentation::IndirectPointer => MachineType::indirect_pointer(),
      MemoryRepresentation::SandboxedPointer => MachineType::sandboxed_pointer(),
      MemoryRepresentation::TrustedPointer => MachineType::uint32(),
      MemoryRepresentation::Simd128 => MachineType::simd128(),
      MemoryRepresentation::Simd256 => MachineType::simd256(),
    }
  }

  /// 原版 operator<< 名 (representations.cc)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      MemoryRepresentation::Int8 => "Int8",
      MemoryRepresentation::Uint8 => "Uint8",
      MemoryRepresentation::Int16 => "Int16",
      MemoryRepresentation::Uint16 => "Uint16",
      MemoryRepresentation::Int32 => "Int32",
      MemoryRepresentation::Uint32 => "Uint32",
      MemoryRepresentation::Int64 => "Int64",
      MemoryRepresentation::Uint64 => "Uint64",
      MemoryRepresentation::Float16 => "Float16",
      MemoryRepresentation::Float32 => "Float32",
      MemoryRepresentation::Float64 => "Float64",
      MemoryRepresentation::AnyTagged => "AnyTagged",
      MemoryRepresentation::TaggedPointer => "TaggedPointer",
      MemoryRepresentation::TaggedSigned => "TaggedSigned",
      MemoryRepresentation::AnyUncompressedTagged => "AnyUncompressedTagged",
      MemoryRepresentation::UncompressedTaggedPointer => "UncompressedTaggedPointer",
      MemoryRepresentation::UncompressedTaggedSigned => "UncompressedTaggedSigned",
      MemoryRepresentation::ProtectedPointer => "ProtectedPointer",
      MemoryRepresentation::IndirectPointer => "IndirectPointer",
      MemoryRepresentation::SandboxedPointer => "SandboxedPointer",
      MemoryRepresentation::TrustedPointer => "TrustedPointer",
      MemoryRepresentation::Simd128 => "Simd128",
      MemoryRepresentation::Simd256 => "Simd256",
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(MaybeRegisterRepresentation::None as u8 == 8);
  assert!(RegisterRepresentation::Simd256 as u8 == 7);
  assert!(MemoryRepresentation::Simd256 as u8 == 22);
  assert!(MemoryRepresentation::Int8.size_in_bytes_log2() == 0);
  assert!(MemoryRepresentation::AnyTagged.size_in_bytes_log2() == 2);
  assert!(MemoryRepresentation::Simd128.size_in_bytes_log2() == 4);
  assert!(MaybeRegisterRepresentation::Tagged.bit_width() == 64);
  assert!(MaybeRegisterRepresentation::Compressed.bit_width() == 32);
  assert!(MemoryRepresentation::Int32.is_signed());
  assert!(!MemoryRepresentation::Uint32.is_signed());
  assert!(MemoryRepresentation::AnyTagged.is_compressible_tagged());
  assert!(!MemoryRepresentation::IndirectPointer.is_compressible_tagged());
};
