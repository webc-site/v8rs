// 由 tool/src/codegen/machine-type.cc.js 从 v8/src/codegen/machine-type.cc;src/codegen/atomic-memory-order.h;src/codegen/callable.h 生成; 修订就地改, 复跑覆盖
// 机器表示字符串表 + 原子内存序 + Callable 骨架

// MachineRepresentation/MachineSemantic 枚举本体见 machine-type.h 覆盖脚本
// (tool/src/compiler/use-info.js → v8-compiler-graph/src/machine_type.rs);
/// 原版 enum class MachineRepresentation : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum MachineRepresentation {
  #[default]
  None = 0,
  Bit = 1,
  Word8 = 2,
  Word16 = 3,
  Word32 = 4,
  Word64 = 5,
  MapWord = 6,
  TaggedSigned = 7,
  TaggedPointer = 8,
  Tagged = 9,
  CompressedPointer = 10,
  Compressed = 11,
  ProtectedPointer = 12,
  IndirectPointer = 13,
  SandboxedPointer = 14,
  Float16RawBits = 15,
  Float16 = 16,
  Float32 = 17,
  Float64 = 18,
  Simd128 = 19,
  Simd256 = 20,
}

/// 架构指针表示 (64 位目标等价于 Word64)
pub const MACHINE_REPRESENTATION_POINTER: MachineRepresentation = MachineRepresentation::Word64;

/// 原版 MachineReprToString: representation → 调试字符串 (按 switch 分支顺序)
pub const MACHINE_REPR_TO_STRING: &[(&str, &str)] = &[
  ("kNone", "kMachNone"),
  ("kBit", "kRepBit"),
  ("kWord8", "kRepWord8"),
  ("kWord16", "kRepWord16"),
  ("kWord32", "kRepWord32"),
  ("kWord64", "kRepWord64"),
  ("kFloat16", "kRepFloat16"),
  ("kFloat16RawBits", "kRepFloat16RawBits"),
  ("kFloat32", "kRepFloat32"),
  ("kFloat64", "kRepFloat64"),
  ("kSimd128", "kRepSimd128"),
  ("kSimd256", "kRepSimd256"),
  ("kTaggedSigned", "kRepTaggedSigned"),
  ("kTaggedPointer", "kRepTaggedPointer"),
  ("kTagged", "kRepTagged"),
  ("kCompressedPointer", "kRepCompressedPointer"),
  ("kCompressed", "kRepCompressed"),
  ("kProtectedPointer", "kRepProtectedPointer"),
  ("kIndirectPointer", "kRepIndirectPointer"),
  ("kMapWord", "kRepMapWord"),
  ("kSandboxedPointer", "kRepSandboxedPointer"),
];

/// 原版 operator<<(MachineSemantic): semantic → 调试字符串
pub const MACHINE_SEMANTIC_TO_STRING: &[(&str, &str)] = &[
  ("kNone", "kMachNone"),
  ("kBool", "kTypeBool"),
  ("kInt32", "kTypeInt32"),
  ("kUint32", "kTypeUint32"),
  ("kInt64", "kTypeInt64"),
  ("kUint64", "kTypeUint64"),
  ("kSignedBigInt64", "kTypeSignedBigInt64"),
  ("kUnsignedBigInt64", "kTypeUnsignedBigInt64"),
  ("kNumber", "kTypeNumber"),
  ("kHoleyFloat64", "kTypeHoleyFloat64"),
  ("kAny", "kTypeAny"),
];

/// 原版 IsSubtype 真值对 (rep1 != rep2 且返回 true 的全部组合);
/// 其余情形: 相等即子类型, 其他一律 false。
pub const IS_SUBTYPE_EXTRA_PAIRS: &[(&str, &str)] = &[
  ("kTaggedSigned", "kTagged"),
  ("kTaggedPointer", "kTagged"),
  ("kCompressedPointer", "kCompressed"),
];

/// 原版 AtomicMemoryOrder (编码须与 Wasm wire bytes 一致)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AtomicMemoryOrder {
  /// kSeqCst
  SeqCst = 0,
  /// kAcqRel
  AcqRel = 1,
}

impl AtomicMemoryOrder {
  /// 原版 hash_value: 直接取编码值
  #[inline]
  #[must_use]
  pub const fn hash_value(self) -> u8 {
    self as u8
  }

  /// 原版 operator<< 文本
  #[inline]
  #[must_use]
  pub const fn name(self) -> &'static str {
    match self {
      Self::SeqCst => "kSeqCst",
      Self::AcqRel => "kAcqRel",
    }
  }
}

/// 原版 Callable: 代码句柄 + 调用接口描述符 的绑定。
/// Handle<Code> / CallInterfaceDescriptor 依赖 v8-heap/v8-codegen 接线, 此处为骨架。
#[derive(Debug, Clone)]
pub struct Callable {
  /// 原版 code_: Handle<Code>
  pub code: HandleCode,
  /// 原版 descriptor_: CallInterfaceDescriptor
  pub descriptor: CallInterfaceDescriptor,
}

impl MachineRepresentation {
  /// u8 编码 → 枚举 (占位码即 machine-type.h 精确序号 0..=20); 越界返回 None
  #[inline]
  #[must_use]
  pub const fn from_code(code: u8) -> Option<Self> {
    match code {
      0 => Some(Self::None),
      1 => Some(Self::Bit),
      2 => Some(Self::Word8),
      3 => Some(Self::Word16),
      4 => Some(Self::Word32),
      5 => Some(Self::Word64),
      6 => Some(Self::MapWord),
      7 => Some(Self::TaggedSigned),
      8 => Some(Self::TaggedPointer),
      9 => Some(Self::Tagged),
      10 => Some(Self::CompressedPointer),
      11 => Some(Self::Compressed),
      12 => Some(Self::ProtectedPointer),
      13 => Some(Self::IndirectPointer),
      14 => Some(Self::SandboxedPointer),
      15 => Some(Self::Float16RawBits),
      16 => Some(Self::Float16),
      17 => Some(Self::Float32),
      18 => Some(Self::Float64),
      19 => Some(Self::Simd128),
      20 => Some(Self::Simd256),
      _ => None,
    }
  }
}

/// 架构指针宽度 (原版 kSystemPointerSize, 64 位目标 = 8)
pub const K_SYSTEM_POINTER_SIZE: i32 = 8;
/// 架构指针宽度 log2 (原版 kSystemPointerSizeLog2 = 3)
pub const K_SYSTEM_POINTER_SIZE_LOG2: u32 = 3;
/// tagged 宽度 log2 (原版 kTaggedSizeLog2, 无指针压缩 64 位目标 = 3)
pub const K_TAGGED_SIZE_LOG2: u32 = 3;

/// v8: src/codegen/machine-type.h:446 ElementSizeLog2Of — 元素字节数的 log2
#[must_use]
pub const fn element_size_log2_of(rep: MachineRepresentation) -> u32 {
  match rep {
    MachineRepresentation::Bit | MachineRepresentation::Word8 => 0,
    MachineRepresentation::Word16 | MachineRepresentation::Float16 => 1,
    MachineRepresentation::Word32
    | MachineRepresentation::Float32
    | MachineRepresentation::IndirectPointer => 2,
    MachineRepresentation::Word64 | MachineRepresentation::Float64 => 3,
    MachineRepresentation::Simd128 => 4,
    MachineRepresentation::Simd256 => 5,
    MachineRepresentation::TaggedSigned
    | MachineRepresentation::TaggedPointer
    | MachineRepresentation::Tagged
    | MachineRepresentation::MapWord
    | MachineRepresentation::CompressedPointer
    | MachineRepresentation::Compressed
    | MachineRepresentation::ProtectedPointer => K_TAGGED_SIZE_LOG2,
    MachineRepresentation::SandboxedPointer => K_SYSTEM_POINTER_SIZE_LOG2,
    // v8: machine-type.h:478 UNREACHABLE — kNone / kFloat16RawBits 无元素尺寸
    // (const 上下文以字面量 panic 映射原版 UNREACHABLE)
    MachineRepresentation::None | MachineRepresentation::Float16RawBits => {
      panic!("ElementSizeLog2Of: 原版 UNREACHABLE 的非法表示")
    }
  }
}

/// v8: src/codegen/machine-type.h:490 ElementSizeInBytes — 元素字节数
#[must_use]
pub const fn element_size_in_bytes(rep: MachineRepresentation) -> i32 {
  1 << element_size_log2_of(rep)
}

/// v8: src/codegen/machine-type.h:495 ElementSizeInBits — 元素位数
#[must_use]
pub const fn element_size_in_bits(rep: MachineRepresentation) -> i32 {
  8 * element_size_in_bytes(rep)
}

/// v8: src/codegen/machine-type.h:514 ElementSizeInPointers — 指针单位向上取整
#[must_use]
pub const fn element_size_in_pointers(rep: MachineRepresentation) -> i32 {
  (element_size_in_bytes(rep) + K_SYSTEM_POINTER_SIZE - 1) / K_SYSTEM_POINTER_SIZE
}

/// v8: src/codegen/machine-type.h:521 RepresentationBit — 表示 → 位掩码
#[must_use]
pub const fn representation_bit(rep: MachineRepresentation) -> i32 {
  1 << (rep as i32)
}

/// v8: src/codegen/machine-type.h:481 kMaximumReprSizeLog2 = ElementSizeLog2Of(kSimd128)
pub const K_MAXIMUM_REPR_SIZE_LOG2: u32 = element_size_log2_of(MachineRepresentation::Simd128);
/// v8: src/codegen/machine-type.h:483 kMaximumReprSizeInBytes
pub const K_MAXIMUM_REPR_SIZE_IN_BYTES: i32 = 1 << K_MAXIMUM_REPR_SIZE_LOG2;

/// 占位类型: Handle<Code> (待 Developer 接线到堆侧)
pub type HandleCode = u32;
/// 占位类型: CallInterfaceDescriptor (待 Developer 接线到 interface_descriptors)
pub type CallInterfaceDescriptor = u32;

impl Callable {
  /// 原版构造: Callable(code, descriptor)
  #[inline]
  #[must_use]
  pub const fn new(code: HandleCode, descriptor: CallInterfaceDescriptor) -> Self {
    Self { code, descriptor }
  }

  /// 原版 code()
  #[inline]
  #[must_use]
  pub const fn code(&self) -> HandleCode {
    self.code
  }

  /// 原版 descriptor()
  #[inline]
  #[must_use]
  pub const fn descriptor(&self) -> CallInterfaceDescriptor {
    self.descriptor
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(MACHINE_REPR_TO_STRING.len() == 21);
  assert!(MACHINE_SEMANTIC_TO_STRING.len() == 11);
  assert!(AtomicMemoryOrder::SeqCst as u8 == 0);
  assert!(AtomicMemoryOrder::AcqRel as u8 == 1);
  // 64 位目标元素尺寸表抽查 (machine-type.h)
  assert!(element_size_in_pointers(MachineRepresentation::Word32) == 1);
  assert!(element_size_in_pointers(MachineRepresentation::Simd128) == 2);
  assert!(element_size_in_pointers(MachineRepresentation::Simd256) == 4);
  assert!(element_size_in_pointers(MachineRepresentation::Tagged) == 1);
  assert!(K_MAXIMUM_REPR_SIZE_LOG2 == 4);
  assert!(matches!(
    MachineRepresentation::from_code(20),
    Some(MachineRepresentation::Simd256)
  ));
  assert!(MachineRepresentation::from_code(21).is_none());
};
