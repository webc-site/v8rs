// 由 tool/src/objects/elements-kind.cc.js 从 v8/src/objects/elements-kind.cc (+elements-kind.h 只读锚) 生成; 修订就地改, 复跑覆盖
// ElementsKind 全枚举与转换表; 对数 clang -E 两遍 (默认/压缩)
//! ElementsKind: 元素种类枚举与转换规则。
//! 配置: 指针压缩开 (crate 主轨, 与 v8_common::K_TAGGED_SIZE_LOG2=2 对齐;
//! clang -E 两遍对数: 非压缩分支 kTaggedSizeLog2=3 / SYSTEM_POINTER_ELEMENTS=PACKED_ELEMENTS)。

use v8_common::K_TAGGED_SIZE_LOG2;
use v8_objects_core::tagged_constants::HEAP_OBJECT_TAG;

use crate::fixed_array_base::K_HEADER_SIZE;

/// 真成员数 (含 WASM_ARRAY/NO_ELEMENTS; 不含 14 别名)。
pub const K_REAL_KIND_COUNT: u8 = 44;
/// kElementsKindCount = LAST_ELEMENTS_KIND - FIRST_ELEMENTS_KIND + 1。
pub const K_ELEMENTS_KIND_COUNT: usize = 42;
/// kFastElementsKindCount。
pub const K_FAST_ELEMENTS_KIND_COUNT: usize = 6;
/// kFastElementsKindPackedToHoley = HOLEY_SMI - PACKED_SMI。
pub const K_FAST_ELEMENTS_KIND_PACKED_TO_HOLEY: u8 = 1;
/// kElementsKindBits (map bit_field2 位段宽)。
pub const K_ELEMENTS_KIND_BITS: u32 = 6;
/// kFastElementsKindBits。
pub const K_FAST_ELEMENTS_KIND_BITS: u32 = 3;
/// TypedArrayAndRabGsabTypedArrayElementsKindTableSize = bit_ceil(12+12)。
pub const TYPED_ARRAY_AND_RAB_GSAB_TABLE_SIZE: usize = 32;

const _: () = assert!((1 << K_ELEMENTS_KIND_BITS) > 41);
const _: () = assert!((1 << (K_ELEMENTS_KIND_BITS - 1)) <= 41);

/// v8: src/objects/elements-kind.h enum ElementsKind : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ElementsKind {
  PackedSmiElements,
  HoleySmiElements,
  PackedElements,
  HoleyElements,
  PackedDoubleElements,
  HoleyDoubleElements,
  PackedNonextensibleElements,
  HoleyNonextensibleElements,
  PackedSealedElements,
  HoleySealedElements,
  PackedFrozenElements,
  HoleyFrozenElements,
  SharedArrayElements,
  DictionaryElements,
  FastSloppyArgumentsElements,
  SlowSloppyArgumentsElements,
  FastStringWrapperElements,
  SlowStringWrapperElements,
  Uint8Elements,
  Int8Elements,
  Uint16Elements,
  Int16Elements,
  Uint32Elements,
  Int32Elements,
  Biguint64Elements,
  Bigint64Elements,
  Uint8ClampedElements,
  Float32Elements,
  Float64Elements,
  Float16Elements,
  RabGsabUint8Elements,
  RabGsabInt8Elements,
  RabGsabUint16Elements,
  RabGsabInt16Elements,
  RabGsabUint32Elements,
  RabGsabInt32Elements,
  RabGsabBiguint64Elements,
  RabGsabBigint64Elements,
  RabGsabUint8ClampedElements,
  RabGsabFloat32Elements,
  RabGsabFloat64Elements,
  RabGsabFloat16Elements,
  WasmArrayElements,
  NoElements,
}

/// v8: src/objects/elements-kind.h FIRST_ELEMENTS_KIND = PACKED_SMI_ELEMENTS
pub const FIRST_ELEMENTS_KIND: u8 = 0;

/// v8: src/objects/elements-kind.h LAST_ELEMENTS_KIND = RAB_GSAB_FLOAT16_ELEMENTS
pub const LAST_ELEMENTS_KIND: u8 = 41;

/// v8: src/objects/elements-kind.h FIRST_FAST_ELEMENTS_KIND = PACKED_SMI_ELEMENTS
pub const FIRST_FAST_ELEMENTS_KIND: u8 = 0;

/// v8: src/objects/elements-kind.h LAST_FAST_ELEMENTS_KIND = HOLEY_DOUBLE_ELEMENTS
pub const LAST_FAST_ELEMENTS_KIND: u8 = 5;

/// v8: src/objects/elements-kind.h FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND = UINT8_ELEMENTS
pub const FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND: u8 = 18;

/// v8: src/objects/elements-kind.h LAST_FIXED_TYPED_ARRAY_ELEMENTS_KIND = FLOAT16_ELEMENTS
pub const LAST_FIXED_TYPED_ARRAY_ELEMENTS_KIND: u8 = 29;

/// v8: src/objects/elements-kind.h FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND = RAB_GSAB_UINT8_ELEMENTS
pub const FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND: u8 = 30;

/// v8: src/objects/elements-kind.h LAST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND = RAB_GSAB_FLOAT16_ELEMENTS
pub const LAST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND: u8 = 41;

/// v8: src/objects/elements-kind.h TERMINAL_FAST_ELEMENTS_KIND = HOLEY_ELEMENTS
pub const TERMINAL_FAST_ELEMENTS_KIND: u8 = 3;

/// v8: src/objects/elements-kind.h FIRST_ANY_NONEXTENSIBLE_ELEMENTS_KIND = PACKED_NONEXTENSIBLE_ELEMENTS
pub const FIRST_ANY_NONEXTENSIBLE_ELEMENTS_KIND: u8 = 6;

/// v8: src/objects/elements-kind.h LAST_ANY_NONEXTENSIBLE_ELEMENTS_KIND = SHARED_ARRAY_ELEMENTS
pub const LAST_ANY_NONEXTENSIBLE_ELEMENTS_KIND: u8 = 12;

/// v8: src/objects/elements-kind.h FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND = UINT8_ELEMENTS
pub const FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND: u8 = 18;

/// v8: src/objects/elements-kind.h LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND = BIGINT64_ELEMENTS
pub const LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND: u8 = 25;

/// SYSTEM_POINTER_ELEMENTS (压缩配置别名 → PACKED_DOUBLE_ELEMENTS;
/// 非压缩分支为 PACKED_ELEMENTS, clang -E 双分支对数)。
pub const SYSTEM_POINTER_ELEMENTS: ElementsKind = ElementsKind::PackedDoubleElements;

impl ElementsKind {
  /// 首个 kind (PACKED_SMI_ELEMENTS)。
  pub const FIRST: Self = Self::PackedSmiElements;
  /// 最后一个 kind (RAB_GSAB_FLOAT16_ELEMENTS)。
  pub const LAST: Self = Self::RabGsabFloat16Elements;

  /// 真成员全集 (按枚举值升序)。
  pub const ALL: [Self; 44] = [
    Self::PackedSmiElements,
    Self::HoleySmiElements,
    Self::PackedElements,
    Self::HoleyElements,
    Self::PackedDoubleElements,
    Self::HoleyDoubleElements,
    Self::PackedNonextensibleElements,
    Self::HoleyNonextensibleElements,
    Self::PackedSealedElements,
    Self::HoleySealedElements,
    Self::PackedFrozenElements,
    Self::HoleyFrozenElements,
    Self::SharedArrayElements,
    Self::DictionaryElements,
    Self::FastSloppyArgumentsElements,
    Self::SlowSloppyArgumentsElements,
    Self::FastStringWrapperElements,
    Self::SlowStringWrapperElements,
    Self::Uint8Elements,
    Self::Int8Elements,
    Self::Uint16Elements,
    Self::Int16Elements,
    Self::Uint32Elements,
    Self::Int32Elements,
    Self::Biguint64Elements,
    Self::Bigint64Elements,
    Self::Uint8ClampedElements,
    Self::Float32Elements,
    Self::Float64Elements,
    Self::Float16Elements,
    Self::RabGsabUint8Elements,
    Self::RabGsabInt8Elements,
    Self::RabGsabUint16Elements,
    Self::RabGsabInt16Elements,
    Self::RabGsabUint32Elements,
    Self::RabGsabInt32Elements,
    Self::RabGsabBiguint64Elements,
    Self::RabGsabBigint64Elements,
    Self::RabGsabUint8ClampedElements,
    Self::RabGsabFloat32Elements,
    Self::RabGsabFloat64Elements,
    Self::RabGsabFloat16Elements,
    Self::WasmArrayElements,
    Self::NoElements,
  ];

  /// v8: src/objects/elements-kind.h GetInitialFastElementsKind
  pub const INITIAL_FAST: Self = Self::PackedSmiElements;

  /// 数值 → 枚举 (0..=43 之外返回 None)。
  #[must_use]
  pub const fn from_u8(v: u8) -> Option<Self> {
    if v <= 43 {
      Some(Self::ALL[v as usize])
    } else {
      None
    }
  }

  /// v8: src/objects/elements-kind.h IsDictionaryElementsKind
  pub const fn is_dictionary_elements_kind(self) -> bool {
    matches!(self, ElementsKind::DictionaryElements)
  }

  /// v8: src/objects/elements-kind.h IsFastArgumentsElementsKind
  pub const fn is_fast_arguments_elements_kind(self) -> bool {
    matches!(self, ElementsKind::FastSloppyArgumentsElements)
  }

  /// v8: src/objects/elements-kind.h IsSlowArgumentsElementsKind
  pub const fn is_slow_arguments_elements_kind(self) -> bool {
    matches!(self, ElementsKind::SlowSloppyArgumentsElements)
  }

  /// v8: src/objects/elements-kind.h IsSloppyArgumentsElementsKind
  pub const fn is_sloppy_arguments_elements_kind(self) -> bool {
    (self as u8) >= 14 && (self as u8) <= 15
  }

  /// v8: src/objects/elements-kind.h IsStringWrapperElementsKind
  pub const fn is_string_wrapper_elements_kind(self) -> bool {
    (self as u8) >= 16 && (self as u8) <= 17
  }

  /// v8: src/objects/elements-kind.h IsTypedArrayElementsKind
  pub const fn is_typed_array_elements_kind(self) -> bool {
    (self as u8) >= 18 && (self as u8) <= 29
  }

  /// v8: src/objects/elements-kind.h IsRabGsabTypedArrayElementsKind
  pub const fn is_rab_gsab_typed_array_elements_kind(self) -> bool {
    (self as u8) >= 30 && (self as u8) <= 41
  }

  /// v8: src/objects/elements-kind.h IsTypedArrayOrRabGsabTypedArrayElementsKind
  pub const fn is_typed_array_or_rab_gsab_typed_array_elements_kind(self) -> bool {
    (self as u8) >= 18 && (self as u8) <= 41
  }

  /// v8: src/objects/elements-kind.h IsBigIntTypedArrayElementsKind
  pub const fn is_big_int_typed_array_elements_kind(self) -> bool {
    matches!(self, ElementsKind::Bigint64Elements)
      || matches!(self, ElementsKind::Biguint64Elements)
      || matches!(self, ElementsKind::RabGsabBigint64Elements)
      || matches!(self, ElementsKind::RabGsabBiguint64Elements)
  }

  /// v8: src/objects/elements-kind.h IsFloat16TypedArrayElementsKind
  pub const fn is_float16_typed_array_elements_kind(self) -> bool {
    matches!(self, ElementsKind::Float16Elements)
      || matches!(self, ElementsKind::RabGsabFloat16Elements)
  }

  /// v8: src/objects/elements-kind.h IsFloatTypedArrayElementsKind
  pub const fn is_float_typed_array_elements_kind(self) -> bool {
    matches!(self, ElementsKind::Float16Elements)
      || matches!(self, ElementsKind::RabGsabFloat16Elements)
      || matches!(self, ElementsKind::Float32Elements)
      || matches!(self, ElementsKind::Float64Elements)
      || matches!(self, ElementsKind::RabGsabFloat32Elements)
      || matches!(self, ElementsKind::RabGsabFloat64Elements)
  }

  /// v8: src/objects/elements-kind.h IsSignedIntTypedArrayElementsKind
  pub const fn is_signed_int_typed_array_elements_kind(self) -> bool {
    matches!(self, ElementsKind::Int8Elements)
      || matches!(self, ElementsKind::RabGsabInt8Elements)
      || matches!(self, ElementsKind::Int16Elements)
      || matches!(self, ElementsKind::RabGsabInt16Elements)
      || matches!(self, ElementsKind::Int32Elements)
      || matches!(self, ElementsKind::RabGsabInt32Elements)
  }

  /// v8: src/objects/elements-kind.h IsUnsignedIntTypedArrayElementsKind
  pub const fn is_unsigned_int_typed_array_elements_kind(self) -> bool {
    matches!(self, ElementsKind::Uint8ClampedElements)
      || matches!(self, ElementsKind::RabGsabUint8ClampedElements)
      || matches!(self, ElementsKind::Uint8Elements)
      || matches!(self, ElementsKind::RabGsabUint8Elements)
      || matches!(self, ElementsKind::Uint16Elements)
      || matches!(self, ElementsKind::RabGsabUint16Elements)
      || matches!(self, ElementsKind::Uint32Elements)
      || matches!(self, ElementsKind::RabGsabUint32Elements)
  }

  /// v8: src/objects/elements-kind.h IsWasmArrayElementsKind
  pub const fn is_wasm_array_elements_kind(self) -> bool {
    matches!(self, ElementsKind::WasmArrayElements)
  }

  /// v8: src/objects/elements-kind.h IsSharedArrayElementsKind
  pub const fn is_shared_array_elements_kind(self) -> bool {
    matches!(self, ElementsKind::SharedArrayElements)
  }

  /// v8: src/objects/elements-kind.h IsTerminalElementsKind
  pub const fn is_terminal_elements_kind(self) -> bool {
    (self as u8) == TERMINAL_FAST_ELEMENTS_KIND
      || self.is_typed_array_or_rab_gsab_typed_array_elements_kind()
      || self.is_rab_gsab_typed_array_elements_kind()
  }

  /// v8: src/objects/elements-kind.h IsFastElementsKind
  pub const fn is_fast_elements_kind(self) -> bool {
    (self as u8) <= 5
  }

  /// v8: src/objects/elements-kind.h IsTransitionElementsKind
  pub const fn is_transition_elements_kind(self) -> bool {
    self.is_fast_elements_kind()
      || self.is_typed_array_or_rab_gsab_typed_array_elements_kind()
      || matches!(self, ElementsKind::FastSloppyArgumentsElements)
      || matches!(self, ElementsKind::FastStringWrapperElements)
  }

  /// v8: src/objects/elements-kind.h IsDoubleElementsKind
  pub const fn is_double_elements_kind(self) -> bool {
    (self as u8) >= 4 && (self as u8) <= 5
  }

  /// v8: src/objects/elements-kind.h IsAnyNonextensibleElementsKindUnchecked
  pub const fn is_any_nonextensible_elements_kind_unchecked(self) -> bool {
    (self as u8) >= 6 && (self as u8) <= 12
  }

  /// v8: src/objects/elements-kind.h IsAnyNonextensibleElementsKind
  pub const fn is_any_nonextensible_elements_kind(self) -> bool {
    self.is_any_nonextensible_elements_kind_unchecked()
  }

  /// v8: src/objects/elements-kind.h IsNonextensibleElementsKind
  pub const fn is_nonextensible_elements_kind(self) -> bool {
    (self as u8) >= 6 && (self as u8) <= 7
  }

  /// v8: src/objects/elements-kind.h IsSealedElementsKind
  pub const fn is_sealed_elements_kind(self) -> bool {
    self.is_shared_array_elements_kind() || (self as u8) >= 8 && (self as u8) <= 9
  }

  /// v8: src/objects/elements-kind.h IsFrozenElementsKind
  pub const fn is_frozen_elements_kind(self) -> bool {
    (self as u8) >= 10 && (self as u8) <= 11
  }

  /// v8: src/objects/elements-kind.h IsFastOrNonextensibleOrSealedElementsKind
  pub const fn is_fast_or_nonextensible_or_sealed_elements_kind(self) -> bool {
    (self as u8) <= 9
  }

  /// v8: src/objects/elements-kind.h IsSmiOrObjectElementsKind
  pub const fn is_smi_or_object_elements_kind(self) -> bool {
    (self as u8) <= 3
  }

  /// v8: src/objects/elements-kind.h IsSmiElementsKind
  pub const fn is_smi_elements_kind(self) -> bool {
    (self as u8) <= 1
  }

  /// v8: src/objects/elements-kind.h IsFastNumberElementsKind
  pub const fn is_fast_number_elements_kind(self) -> bool {
    self.is_smi_elements_kind() || self.is_double_elements_kind()
  }

  /// v8: src/objects/elements-kind.h IsObjectElementsKind
  pub const fn is_object_elements_kind(self) -> bool {
    (self as u8) >= 2 && (self as u8) <= 3
  }

  /// v8: src/objects/elements-kind.h IsAnyHoleyNonextensibleElementsKind
  pub const fn is_any_holey_nonextensible_elements_kind(self) -> bool {
    matches!(self, ElementsKind::HoleyNonextensibleElements)
      || matches!(self, ElementsKind::HoleySealedElements)
      || matches!(self, ElementsKind::HoleyFrozenElements)
  }

  /// v8: src/objects/elements-kind.h IsHoleyElementsKind
  pub const fn is_holey_elements_kind(self) -> bool {
    (self as u8) & 1 == 1 && (self as u8) <= 5
  }

  /// v8: src/objects/elements-kind.h IsHoleyElementsKindForRead
  pub const fn is_holey_elements_kind_for_read(self) -> bool {
    (self as u8) & 1 == 1 && (self as u8) <= 11
  }

  /// v8: src/objects/elements-kind.h IsHoleyOrDictionaryElementsKind
  pub const fn is_holey_or_dictionary_elements_kind(self) -> bool {
    self.is_holey_elements_kind_for_read() || matches!(self, ElementsKind::DictionaryElements)
  }

  /// v8: src/objects/elements-kind.h IsFastPackedElementsKind
  pub const fn is_fast_packed_elements_kind(self) -> bool {
    (self as u8) & 1 == 0 && (self as u8) <= 4
  }

  /// v8: src/objects/elements-kind.h ElementsKindToShiftSize
  /// (WASM_ARRAY/NO_ELEMENTS 原版 CONSTEXPR_UNREACHABLE; 偏差: 按 tagged 对齐返回)。
  #[must_use]
  pub const fn to_shift_size(self) -> u32 {
    match self {
      ElementsKind::PackedSmiElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::HoleySmiElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::PackedElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::HoleyElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::PackedDoubleElements => 3,
      ElementsKind::HoleyDoubleElements => 3,
      ElementsKind::PackedNonextensibleElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::HoleyNonextensibleElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::PackedSealedElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::HoleySealedElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::PackedFrozenElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::HoleyFrozenElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::SharedArrayElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::DictionaryElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::FastSloppyArgumentsElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::SlowSloppyArgumentsElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::FastStringWrapperElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::SlowStringWrapperElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::Uint8Elements => 0,
      ElementsKind::Int8Elements => 0,
      ElementsKind::Uint16Elements => 1,
      ElementsKind::Int16Elements => 1,
      ElementsKind::Uint32Elements => 2,
      ElementsKind::Int32Elements => 2,
      ElementsKind::Biguint64Elements => 3,
      ElementsKind::Bigint64Elements => 3,
      ElementsKind::Uint8ClampedElements => 0,
      ElementsKind::Float32Elements => 2,
      ElementsKind::Float64Elements => 3,
      ElementsKind::Float16Elements => 1,
      ElementsKind::RabGsabUint8Elements => 0,
      ElementsKind::RabGsabInt8Elements => 0,
      ElementsKind::RabGsabUint16Elements => 1,
      ElementsKind::RabGsabInt16Elements => 1,
      ElementsKind::RabGsabUint32Elements => 2,
      ElementsKind::RabGsabInt32Elements => 2,
      ElementsKind::RabGsabBiguint64Elements => 3,
      ElementsKind::RabGsabBigint64Elements => 3,
      ElementsKind::RabGsabUint8ClampedElements => 0,
      ElementsKind::RabGsabFloat32Elements => 2,
      ElementsKind::RabGsabFloat64Elements => 3,
      ElementsKind::RabGsabFloat16Elements => 1,
      ElementsKind::WasmArrayElements => K_TAGGED_SIZE_LOG2,
      ElementsKind::NoElements => K_TAGGED_SIZE_LOG2,
    }
  }

  /// v8: src/objects/elements-kind.h ElementsKindToByteSize
  #[must_use]
  pub const fn to_byte_size(self) -> u32 {
    1 << self.to_shift_size()
  }

  /// v8: src/objects/elements-kind.cc ElementsKindToString
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      ElementsKind::PackedSmiElements => "PACKED_SMI_ELEMENTS",
      ElementsKind::HoleySmiElements => "HOLEY_SMI_ELEMENTS",
      ElementsKind::PackedElements => "PACKED_ELEMENTS",
      ElementsKind::HoleyElements => "HOLEY_ELEMENTS",
      ElementsKind::PackedDoubleElements => "PACKED_DOUBLE_ELEMENTS",
      ElementsKind::HoleyDoubleElements => "HOLEY_DOUBLE_ELEMENTS",
      ElementsKind::PackedNonextensibleElements => "PACKED_NONEXTENSIBLE_ELEMENTS",
      ElementsKind::HoleyNonextensibleElements => "HOLEY_NONEXTENSIBLE_ELEMENTS",
      ElementsKind::PackedSealedElements => "PACKED_SEALED_ELEMENTS",
      ElementsKind::HoleySealedElements => "HOLEY_SEALED_ELEMENTS",
      ElementsKind::PackedFrozenElements => "PACKED_FROZEN_ELEMENTS",
      ElementsKind::HoleyFrozenElements => "HOLEY_FROZEN_ELEMENTS",
      ElementsKind::DictionaryElements => "DICTIONARY_ELEMENTS",
      ElementsKind::FastSloppyArgumentsElements => "FAST_SLOPPY_ARGUMENTS_ELEMENTS",
      ElementsKind::SlowSloppyArgumentsElements => "SLOW_SLOPPY_ARGUMENTS_ELEMENTS",
      ElementsKind::FastStringWrapperElements => "FAST_STRING_WRAPPER_ELEMENTS",
      ElementsKind::SlowStringWrapperElements => "SLOW_STRING_WRAPPER_ELEMENTS",
      ElementsKind::Uint8Elements => "UINT8_ELEMENTS",
      ElementsKind::Int8Elements => "INT8_ELEMENTS",
      ElementsKind::Uint16Elements => "UINT16_ELEMENTS",
      ElementsKind::Int16Elements => "INT16_ELEMENTS",
      ElementsKind::Uint32Elements => "UINT32_ELEMENTS",
      ElementsKind::Int32Elements => "INT32_ELEMENTS",
      ElementsKind::Biguint64Elements => "BIGUINT64_ELEMENTS",
      ElementsKind::Bigint64Elements => "BIGINT64_ELEMENTS",
      ElementsKind::Uint8ClampedElements => "UINT8_CLAMPED_ELEMENTS",
      ElementsKind::Float32Elements => "FLOAT32_ELEMENTS",
      ElementsKind::Float64Elements => "FLOAT64_ELEMENTS",
      ElementsKind::Float16Elements => "FLOAT16_ELEMENTS",
      ElementsKind::RabGsabUint8Elements => "RAB_GSAB_UINT8_ELEMENTS",
      ElementsKind::RabGsabInt8Elements => "RAB_GSAB_INT8_ELEMENTS",
      ElementsKind::RabGsabUint16Elements => "RAB_GSAB_UINT16_ELEMENTS",
      ElementsKind::RabGsabInt16Elements => "RAB_GSAB_INT16_ELEMENTS",
      ElementsKind::RabGsabUint32Elements => "RAB_GSAB_UINT32_ELEMENTS",
      ElementsKind::RabGsabInt32Elements => "RAB_GSAB_INT32_ELEMENTS",
      ElementsKind::RabGsabBiguint64Elements => "RAB_GSAB_BIGUINT64_ELEMENTS",
      ElementsKind::RabGsabBigint64Elements => "RAB_GSAB_BIGINT64_ELEMENTS",
      ElementsKind::RabGsabUint8ClampedElements => "RAB_GSAB_UINT8_CLAMPED_ELEMENTS",
      ElementsKind::RabGsabFloat32Elements => "RAB_GSAB_FLOAT32_ELEMENTS",
      ElementsKind::RabGsabFloat64Elements => "RAB_GSAB_FLOAT64_ELEMENTS",
      ElementsKind::RabGsabFloat16Elements => "RAB_GSAB_FLOAT16_ELEMENTS",
      ElementsKind::WasmArrayElements => "WASM_ARRAY_ELEMENTS",
      ElementsKind::SharedArrayElements => "SHARED_ARRAY_ELEMENTS",
      ElementsKind::NoElements => "NO_ELEMENTS",
    }
  }

  /// v8: src/objects/elements-kind.cc GetDefaultHeaderSizeForElementsKind
  /// (OFFSET_OF_DATA_START(FixedArray) 取 crate 压缩主轨 K_HEADER_SIZE=8, 减 kHeapObjectTag=1)。
  #[must_use]
  pub const fn default_header_size(self) -> usize {
    if self.is_typed_array_or_rab_gsab_typed_array_elements_kind() {
      0
    } else {
      K_HEADER_SIZE - HEAP_OBJECT_TAG
    }
  }

  /// v8: src/objects/elements-kind.cc GetSequenceIndexFromFastElementsKind
  #[must_use]
  pub const fn sequence_index_from_fast(self) -> usize {
    match self {
      Self::PackedSmiElements => 0,
      Self::HoleySmiElements => 1,
      Self::PackedDoubleElements => 2,
      Self::HoleyDoubleElements => 3,
      Self::PackedElements => 4,
      Self::HoleyElements => 5,
      _ => 6, // 原版 UNREACHABLE; 非 fast 返回越界哨兵
    }
  }

  /// v8: src/objects/elements-kind.cc GetFastElementsKindFromSequenceIndex
  #[must_use]
  pub const fn fast_from_sequence_index(i: usize) -> Self {
    FAST_ELEMENTS_KIND_SEQUENCE[i % K_FAST_ELEMENTS_KIND_COUNT]
  }

  /// v8: src/objects/elements-kind.cc GetNextTransitionElementsKind
  #[must_use]
  pub const fn next_transition(self) -> Self {
    Self::fast_from_sequence_index(self.sequence_index_from_fast() + 1)
  }

  /// v8: src/objects/elements-kind.h GetPackedElementsKind
  #[must_use]
  pub const fn packed(self) -> Self {
    match self {
      Self::HoleySmiElements => Self::PackedSmiElements,
      Self::HoleyDoubleElements => Self::PackedDoubleElements,
      Self::HoleyElements => Self::PackedElements,
      k => k,
    }
  }

  /// v8: src/objects/elements-kind.h GetHoleyElementsKind
  #[must_use]
  pub const fn holey(self) -> Self {
    match self {
      Self::PackedSmiElements => Self::HoleySmiElements,
      Self::PackedDoubleElements => Self::HoleyDoubleElements,
      Self::PackedElements => Self::HoleyElements,
      Self::PackedNonextensibleElements => Self::HoleyNonextensibleElements,
      k => k,
    }
  }

  /// v8: src/objects/elements-kind.h FastSmiToObjectElementsKind
  #[must_use]
  pub const fn fast_smi_to_object(self) -> Self {
    match self {
      Self::PackedSmiElements => Self::PackedElements,
      _ => Self::HoleyElements,
    }
  }

  /// v8: src/objects/elements-kind.h GetCorrespondingRabGsabElementsKind
  #[must_use]
  pub const fn corresponding_rab_gsab(self) -> Self {
    let v = (self as u8)
      .wrapping_sub(FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND)
      .wrapping_add(FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND);
    match Self::from_u8(v) {
      Some(k) => k,
      None => Self::RabGsabFloat16Elements,
    }
  }

  /// v8: src/objects/elements-kind.h GetCorrespondingNonRabGsabElementsKind
  #[must_use]
  pub const fn corresponding_non_rab_gsab(self) -> Self {
    let v = (self as u8)
      .wrapping_sub(FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND)
      .wrapping_add(FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND);
    match Self::from_u8(v) {
      Some(k) => k,
      None => Self::Float16Elements,
    }
  }
}

/// v8: src/objects/elements-kind.cc kFastElementsKindSequence
pub const FAST_ELEMENTS_KIND_SEQUENCE: [ElementsKind; 6] = [
  ElementsKind::PackedSmiElements,
  ElementsKind::HoleySmiElements,
  ElementsKind::PackedDoubleElements,
  ElementsKind::HoleyDoubleElements,
  ElementsKind::PackedElements,
  ElementsKind::HoleyElements,
];

/// v8: src/objects/elements-kind.cc kTypedArrayAndRabGsabTypedArrayElementsKindShifts
/// (索引 = kind - FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND; 尾部 bit_ceil 填充 0)。
pub const TYPED_ARRAY_AND_RAB_GSAB_SHIFTS: [u8; 32] = [
  0, 0, 1, 1, 2, 2, 3, 3, 0, 2, 3, 1, 0, 0, 1, 1, 2, 2, 3, 3, 0, 2, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// v8: src/objects/elements-kind.cc kTypedArrayAndRabGsabTypedArrayElementsKindSizes
pub const TYPED_ARRAY_AND_RAB_GSAB_SIZES: [u8; 32] = [
  1, 1, 2, 2, 4, 4, 8, 8, 1, 4, 8, 2, 1, 1, 2, 2, 4, 4, 8, 8, 1, 4, 8, 2, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// v8: src/objects/elements-kind.h UnionElementsKindUptoPackedness
#[must_use]
pub fn union_elements_kind_upto_packedness(a: &mut ElementsKind, b: ElementsKind) -> bool {
  match *a {
    ElementsKind::HoleySmiElements | ElementsKind::PackedSmiElements => {
      if matches!(
        b,
        ElementsKind::PackedSmiElements | ElementsKind::HoleySmiElements
      ) {
        if (*a as u8) < (b as u8) {
          *a = b;
        }
        true
      } else {
        false
      }
    }
    ElementsKind::PackedElements | ElementsKind::HoleyElements => {
      if matches!(
        b,
        ElementsKind::PackedElements | ElementsKind::HoleyElements
      ) {
        if (*a as u8) < (b as u8) {
          *a = b;
        }
        true
      } else {
        false
      }
    }
    ElementsKind::PackedDoubleElements | ElementsKind::HoleyDoubleElements => {
      if matches!(
        b,
        ElementsKind::PackedDoubleElements | ElementsKind::HoleyDoubleElements
      ) {
        if (*a as u8) < (b as u8) {
          *a = b;
        }
        true
      } else {
        false
      }
    }
    _ => false,
  }
}

/// v8: src/objects/elements-kind.cc UnionElementsKindUptoSize
#[must_use]
pub fn union_elements_kind_upto_size(a: &mut ElementsKind, b: ElementsKind) -> bool {
  match *a {
    ElementsKind::PackedSmiElements => match b {
      ElementsKind::PackedSmiElements
      | ElementsKind::HoleySmiElements
      | ElementsKind::PackedElements
      | ElementsKind::HoleyElements => {
        *a = b;
        true
      }
      _ => false,
    },
    ElementsKind::HoleySmiElements => match b {
      ElementsKind::PackedSmiElements | ElementsKind::HoleySmiElements => {
        *a = ElementsKind::HoleySmiElements;
        true
      }
      ElementsKind::PackedElements | ElementsKind::HoleyElements => {
        *a = ElementsKind::HoleyElements;
        true
      }
      _ => false,
    },
    ElementsKind::PackedElements => match b {
      ElementsKind::PackedSmiElements | ElementsKind::PackedElements => {
        *a = ElementsKind::PackedElements;
        true
      }
      ElementsKind::HoleySmiElements | ElementsKind::HoleyElements => {
        *a = ElementsKind::HoleyElements;
        true
      }
      _ => false,
    },
    ElementsKind::HoleyElements => match b {
      ElementsKind::PackedSmiElements
      | ElementsKind::HoleySmiElements
      | ElementsKind::PackedElements
      | ElementsKind::HoleyElements => {
        *a = ElementsKind::HoleyElements;
        true
      }
      _ => false,
    },
    ElementsKind::PackedDoubleElements => match b {
      ElementsKind::PackedDoubleElements | ElementsKind::HoleyDoubleElements => {
        *a = b;
        true
      }
      _ => false,
    },
    ElementsKind::HoleyDoubleElements => match b {
      ElementsKind::PackedDoubleElements | ElementsKind::HoleyDoubleElements => {
        *a = ElementsKind::HoleyDoubleElements;
        true
      }
      _ => false,
    },
    _ => false,
  }
}

/// v8: src/objects/elements-kind.cc IsMoreGeneralElementsKindTransition
#[must_use]
pub const fn is_more_general_elements_kind_transition(
  from: ElementsKind,
  to: ElementsKind,
) -> bool {
  if !from.is_fast_elements_kind() {
    return false;
  }
  if !(to.is_fast_elements_kind() || matches!(to, ElementsKind::DictionaryElements)) {
    return false;
  }
  match from {
    ElementsKind::PackedSmiElements => !matches!(to, ElementsKind::PackedSmiElements),
    ElementsKind::HoleySmiElements => !matches!(
      to,
      ElementsKind::PackedSmiElements | ElementsKind::HoleySmiElements
    ),
    ElementsKind::PackedDoubleElements => !matches!(
      to,
      ElementsKind::PackedSmiElements
        | ElementsKind::HoleySmiElements
        | ElementsKind::PackedDoubleElements
    ),
    ElementsKind::HoleyDoubleElements => matches!(
      to,
      ElementsKind::PackedElements | ElementsKind::HoleyElements
    ),
    ElementsKind::PackedElements => matches!(to, ElementsKind::HoleyElements),
    _ => false,
  }
}

/// v8: src/objects/elements-kind.h GetMoreGeneralElementsKind
#[must_use]
pub const fn get_more_general_elements_kind(from: ElementsKind, to: ElementsKind) -> ElementsKind {
  if is_more_general_elements_kind_transition(from, to) {
    to
  } else {
    from
  }
}

/// v8: src/objects/elements-kind.h IsSimpleMapChangeTransition
#[must_use]
pub const fn is_simple_map_change_transition(from: ElementsKind, to: ElementsKind) -> bool {
  ((from.holey() as u8) == (to as u8))
    || (from.is_smi_elements_kind() && to.is_object_elements_kind())
}

// —— 提取不变式断言 (clang -E 对数; const 兼容: 枚举比较走 u8 / matches!) ——
const _: () = {
  assert!(ElementsKind::ALL.len() == 44);
  assert!(ElementsKind::PackedSmiElements as u8 == 0);
  assert!(ElementsKind::RabGsabFloat16Elements as u8 == 41);
  assert!(ElementsKind::WasmArrayElements as u8 == 42);
  assert!(ElementsKind::NoElements as u8 == 43);
  assert!(LAST_ELEMENTS_KIND == 41);
  assert!(FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND == 18);
  assert!(LAST_FIXED_TYPED_ARRAY_ELEMENTS_KIND == 29);
  assert!(FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND == 30);
  assert!(LAST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND == 41);
  assert!(matches!(
    SYSTEM_POINTER_ELEMENTS,
    ElementsKind::PackedDoubleElements
  ));
  assert!(K_ELEMENTS_KIND_COUNT == 42);
  assert!(TYPED_ARRAY_AND_RAB_GSAB_TABLE_SIZE == 32);
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SHIFTS.len() == 32);
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SHIFTS[0] == 0); // uint8
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SHIFTS[10] == 3); // float64 (序: ...Float32=9, Float64=10, Float16=11)
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SIZES[10] == 8);
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SHIFTS[22] == 3); // rab_gsab float64
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SHIFTS[11] == 1); // float16
  assert!(matches!(
    FAST_ELEMENTS_KIND_SEQUENCE[0],
    ElementsKind::PackedSmiElements
  ));
  assert!(ElementsKind::Uint8Elements.to_byte_size() == 1);
  assert!(ElementsKind::Float64Elements.to_byte_size() == 8);
  assert!(ElementsKind::PackedSmiElements.to_shift_size() == K_TAGGED_SIZE_LOG2);
  assert!(ElementsKind::PackedElements.to_str().len() == 15);
  assert!(ElementsKind::NoElements.to_str().len() == 11);
  assert!(ElementsKind::Uint8Elements.to_str().as_bytes()[0] == b'U');
  assert!(matches!(
    ElementsKind::HoleyElements.packed(),
    ElementsKind::PackedElements
  ));
  assert!(matches!(
    ElementsKind::PackedNonextensibleElements.holey(),
    ElementsKind::HoleyNonextensibleElements
  ));
  assert!(ElementsKind::HoleySmiElements.sequence_index_from_fast() == 1);
  assert!(matches!(
    ElementsKind::PackedSmiElements.next_transition(),
    ElementsKind::HoleySmiElements
  ));
  assert!(matches!(
    ElementsKind::HoleyElements.next_transition(),
    ElementsKind::PackedSmiElements
  ));
  assert!(matches!(
    ElementsKind::Uint8Elements.corresponding_rab_gsab(),
    ElementsKind::RabGsabUint8Elements
  ));
  assert!(matches!(
    ElementsKind::RabGsabFloat16Elements.corresponding_non_rab_gsab(),
    ElementsKind::Float16Elements
  ));
  assert!(is_simple_map_change_transition(
    ElementsKind::PackedSmiElements,
    ElementsKind::HoleySmiElements
  ));
  assert!(!is_more_general_elements_kind_transition(
    ElementsKind::HoleyElements,
    ElementsKind::PackedElements
  ));
  assert!(is_more_general_elements_kind_transition(
    ElementsKind::PackedSmiElements,
    ElementsKind::DictionaryElements
  ));
};
