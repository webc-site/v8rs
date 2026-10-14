// 由 tool/src/objects/turbofan-types.h.js 从 v8/src/objects/turbofan-types.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// TurbofanType 位集标志链展开 (低 32 位 + 高 4 位)
//! TurbofanType 位集 (类型反馈的位标志编码)。
//! 类层级 (TurbofanBitset/RangeType/...) 与 SizeFor 语义面登记跳过。
//! tq 侧类型清单 (turbofan-types.tq) 与本表 1:1 对应, 由 Developer 就地接线。

// ── TurbofanTypeLowBits ──────────────────────────────────────────────────────────────────
/// 原版 UnusedPaddingField1Bit (bit 0)。
pub const UNUSED_PADDING_FIELD1_BIT_MASK: u32 = 0x1;
/// 原版 OtherUnsigned31Bit (bit 1)。
pub const OTHER_UNSIGNED31_BIT_MASK: u32 = 0x2;
/// 原版 OtherUnsigned32Bit (bit 2)。
pub const OTHER_UNSIGNED32_BIT_MASK: u32 = 0x4;
/// 原版 OtherSigned32Bit (bit 3)。
pub const OTHER_SIGNED32_BIT_MASK: u32 = 0x8;
/// 原版 OtherNumberBit (bit 4)。
pub const OTHER_NUMBER_BIT_MASK: u32 = 0x10;
/// 原版 OtherStringBit (bit 5)。
pub const OTHER_STRING_BIT_MASK: u32 = 0x20;
/// 原版 Negative31Bit (bit 6)。
pub const NEGATIVE31_BIT_MASK: u32 = 0x40;
/// 原版 NullBit (bit 7)。
pub const NULL_BIT_MASK: u32 = 0x80;
/// 原版 UndefinedBit (bit 8)。
pub const UNDEFINED_BIT_MASK: u32 = 0x100;
/// 原版 BooleanBit (bit 9)。
pub const BOOLEAN_BIT_MASK: u32 = 0x200;
/// 原版 Unsigned30Bit (bit 10)。
pub const UNSIGNED30_BIT_MASK: u32 = 0x400;
/// 原版 MinusZeroBit (bit 11)。
pub const MINUS_ZERO_BIT_MASK: u32 = 0x800;
/// 原版 NaNBit (bit 12)。
pub const NA_NBIT_MASK: u32 = 0x1000;
/// 原版 SymbolBit (bit 13)。
pub const SYMBOL_BIT_MASK: u32 = 0x2000;
/// 原版 InternalizedStringBit (bit 14)。
pub const INTERNALIZED_STRING_BIT_MASK: u32 = 0x4000;
/// 原版 OtherCallableBit (bit 15)。
pub const OTHER_CALLABLE_BIT_MASK: u32 = 0x8000;
/// 原版 OtherObjectBit (bit 16)。
pub const OTHER_OBJECT_BIT_MASK: u32 = 0x10000;
/// 原版 OtherUndetectableBit (bit 17)。
pub const OTHER_UNDETECTABLE_BIT_MASK: u32 = 0x20000;
/// 原版 CallableProxyBit (bit 18)。
pub const CALLABLE_PROXY_BIT_MASK: u32 = 0x40000;
/// 原版 OtherProxyBit (bit 19)。
pub const OTHER_PROXY_BIT_MASK: u32 = 0x80000;
/// 原版 CallableFunctionBit (bit 20)。
pub const CALLABLE_FUNCTION_BIT_MASK: u32 = 0x100000;
/// 原版 ClassConstructorBit (bit 21)。
pub const CLASS_CONSTRUCTOR_BIT_MASK: u32 = 0x200000;
/// 原版 BoundFunctionBit (bit 22)。
pub const BOUND_FUNCTION_BIT_MASK: u32 = 0x400000;
/// 原版 OtherInternalBit (bit 23)。
pub const OTHER_INTERNAL_BIT_MASK: u32 = 0x800000;
/// 原版 ExternalPointerBit (bit 24)。
pub const EXTERNAL_POINTER_BIT_MASK: u32 = 0x1000000;
/// 原版 ArrayBit (bit 25)。
pub const ARRAY_BIT_MASK: u32 = 0x2000000;
/// 原版 UnsignedBigInt63Bit (bit 26)。
pub const UNSIGNED_BIG_INT63_BIT_MASK: u32 = 0x4000000;
/// 原版 OtherUnsignedBigInt64Bit (bit 27)。
pub const OTHER_UNSIGNED_BIG_INT64_BIT_MASK: u32 = 0x8000000;
/// 原版 NegativeBigInt63Bit (bit 28)。
pub const NEGATIVE_BIG_INT63_BIT_MASK: u32 = 0x10000000;
/// 原版 OtherBigIntBit (bit 29)。
pub const OTHER_BIG_INT_BIT_MASK: u32 = 0x20000000;
/// 原版 WasmObjectBit (bit 30)。
pub const WASM_OBJECT_BIT_MASK: u32 = 0x40000000;
/// 原版 SandboxedPointerBit (bit 31)。
pub const SANDBOXED_POINTER_BIT_MASK: u32 = 0x-80000000;

/// TurbofanTypeLowBits 标志 (原版 enum Flag; kNone = 0)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TurbofanTypeLowBitsFlag {
  /// kNone
  None = 0,
  /// kUnusedPaddingField1
  Unusedpaddingfield1 = 0x1,
  /// kOtherUnsigned31
  Otherunsigned31 = 0x2,
  /// kOtherUnsigned32
  Otherunsigned32 = 0x4,
  /// kOtherSigned32
  Othersigned32 = 0x8,
  /// kOtherNumber
  Othernumber = 0x10,
  /// kOtherString
  Otherstring = 0x20,
  /// kNegative31
  Negative31 = 0x40,
  /// kNull
  Null = 0x80,
  /// kUndefined
  Undefined = 0x100,
  /// kBoolean
  Boolean = 0x200,
  /// kUnsigned30
  Unsigned30 = 0x400,
  /// kMinusZero
  Minuszero = 0x800,
  /// kNaN
  Nan = 0x1000,
  /// kSymbol
  Symbol = 0x2000,
  /// kInternalizedString
  Internalizedstring = 0x4000,
  /// kOtherCallable
  Othercallable = 0x8000,
  /// kOtherObject
  Otherobject = 0x10000,
  /// kOtherUndetectable
  Otherundetectable = 0x20000,
  /// kCallableProxy
  Callableproxy = 0x40000,
  /// kOtherProxy
  Otherproxy = 0x80000,
  /// kCallableFunction
  Callablefunction = 0x100000,
  /// kClassConstructor
  Classconstructor = 0x200000,
  /// kBoundFunction
  Boundfunction = 0x400000,
  /// kOtherInternal
  Otherinternal = 0x800000,
  /// kExternalPointer
  Externalpointer = 0x1000000,
  /// kArray
  Array = 0x2000000,
  /// kUnsignedBigInt63
  Unsignedbigint63 = 0x4000000,
  /// kOtherUnsignedBigInt64
  Otherunsignedbigint64 = 0x8000000,
  /// kNegativeBigInt63
  Negativebigint63 = 0x10000000,
  /// kOtherBigInt
  Otherbigint = 0x20000000,
  /// kWasmObject
  Wasmobject = 0x40000000,
  /// kSandboxedPointer
  Sandboxedpointer = 0x80000000,
}

// ── TurbofanTypeHighBits ──────────────────────────────────────────────────────────────────
/// 原版 MachineBit (bit 0)。
pub const MACHINE_BIT_MASK: u32 = 0x1;
/// 原版 HoleBit (bit 1)。
pub const HOLE_BIT_MASK: u32 = 0x2;
/// 原版 StringWrapperBit (bit 2)。
pub const STRING_WRAPPER_BIT_MASK: u32 = 0x4;
/// 原版 TypedArrayBit (bit 3)。
pub const TYPED_ARRAY_BIT_MASK: u32 = 0x8;

/// TurbofanTypeHighBits 标志 (原版 enum Flag; kNone = 0)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TurbofanTypeHighBitsFlag {
  /// kNone
  None = 0,
  /// kMachine
  Machine = 0x1,
  /// kHole
  Hole = 0x2,
  /// kStringWrapper
  Stringwrapper = 0x4,
  /// kTypedArray
  Typedarray = 0x8,
}

/// 低 32 位段位总数 (原版 kFlagCount)。
pub const LOW_FLAG_COUNT: u32 = 32;
/// 高位段位总数 (原版 kFlagCount)。
pub const HIGH_FLAG_COUNT: u32 = 4;

// —— 提取不变式断言 ——
const _: () = {
  assert!(LOW_FLAG_COUNT == 32);
  assert!(HIGH_FLAG_COUNT == 4);
  assert!(TURBOFAN_TYPE_LOW_BITS_UNUSED_PADDING_FIELD1_MASK == 0x1);
  assert!(TURBOFAN_TYPE_LOW_BITS_SANDBOXED_POINTER_MASK == 0x80000000);
  assert!(TURBOFAN_TYPE_HIGH_BITS_MACHINE_MASK == 0x1);
  assert!(TURBOFAN_TYPE_HIGH_BITS_TYPED_ARRAY_MASK == 0x8);
};
