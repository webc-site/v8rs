// 由 tool/src/objects/property-details.h.js 从 v8/src/objects/property-details.h + property-details-inl.h 生成; 修订就地改, 复跑覆盖
// 显式位段常量 + 取放函数; Print 族推迟归 L4 v8-diagnostics

//! PropertyDetails: 属性的类型/属性标志/存储位置的位编码 (fast 与 dictionary 双布局)。

use crate::{smi::Smi, tagged::Tagged};

// ── 常量 (property-details.h:242-251) ─────────────────────────────────────
/// 描述符索引位宽 (Bits3/DescriptorPointer 共用锚点)。
pub const DESCRIPTOR_INDEX_BIT_COUNT: u32 = 10;
/// 对象内首属性偏移的位宽限制。
pub const FIRST_INOBJECT_PROPERTY_OFFSET_BIT_COUNT: u32 = 8;
/// 描述符数组最大描述符数 (1<<10 - 4)。
pub const MAX_NUMBER_OF_DESCRIPTORS: u32 = (1 << DESCRIPTOR_INDEX_BIT_COUNT) - 4;
/// 无效 enum cache 哨兵 (1<<10 - 1)。
pub const INVALID_ENUM_CACHE_SENTINEL: u32 = (1 << DESCRIPTOR_INDEX_BIT_COUNT) - 1;
const _: () = assert!(MAX_NUMBER_OF_DESCRIPTORS == 1020);
const _: () = assert!(INVALID_ENUM_CACHE_SENTINEL == 1023);

// ── PropertyAttributes (ES6 6.1.7.1; 位标志非 scoped enum → newtype) ─────
/// 属性属性标志位集合 (对应原版 enum PropertyAttributes)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PropertyAttributes(pub u8);

impl PropertyAttributes {
  pub const NONE: Self = Self(0);
  pub const READ_ONLY: Self = Self(1 << 0);
  pub const DONT_ENUM: Self = Self(1 << 1);
  pub const DONT_DELETE: Self = Self(1 << 2);
  /// 全部属性位掩码。
  pub const ALL_ATTRIBUTES_MASK: Self = Self(7);
  /// 封闭 (不可配置)。
  pub const SEALED: Self = Self::DONT_DELETE;
  /// 冻结 (不可配置 + 只读)。
  pub const FROZEN: Self = Self(Self::SEALED.0 | Self::READ_ONLY.0);
  /// 属性不存在哨兵 (仅作查找返回值, 永不存入 details)。
  pub const ABSENT: Self = Self(64);

  /// 对应原版 PropertyAttributesFromInt (值域校验后转)。
  #[inline]
  #[must_use]
  pub const fn from_int(value: i32) -> Self {
    debug_assert!(value >= 0 && (value & !Self::ALL_ATTRIBUTES_MASK.0 as i32) == 0);
    Self(value as u8)
  }

  /// 并集。
  #[inline]
  #[must_use]
  pub const fn union(self, other: Self) -> Self {
    Self(self.0 | other.0)
  }

  /// 是否包含指定位。
  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    (self.0 & other.0) == other.0
  }

  /// 获取底层位表示。
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u8 {
    self.0
  }
}

/// 属性位计数 (原版 kPropertyAttributesBitsCount)。
pub const PROPERTY_ATTRIBUTES_BITS_COUNT: u32 = 3;
/// 属性位组合数 (原版 kPropertyAttributesCombinationsCount)。
pub const PROPERTY_ATTRIBUTES_COMBINATIONS_COUNT: u32 = 1 << PROPERTY_ATTRIBUTES_BITS_COUNT;

// ── PropertyFilter (过滤标志位 → newtype) ─────────────────────────────────
/// 属性过滤标志 (对应原版 enum PropertyFilter)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PropertyFilter(pub u8);

impl PropertyFilter {
  pub const ALL_PROPERTIES: Self = Self(0);
  pub const ONLY_WRITABLE: Self = Self(1);
  pub const ONLY_ENUMERABLE: Self = Self(2);
  pub const ONLY_CONFIGURABLE: Self = Self(4);
  pub const SKIP_STRINGS: Self = Self(8);
  pub const SKIP_SYMBOLS: Self = Self(16);
  pub const PRIVATE_NAMES_ONLY: Self = Self(32);
  /// 可枚举字符串 (ONLY_ENUMERABLE | SKIP_SYMBOLS)。
  pub const ENUMERABLE_STRINGS: Self = Self(Self::ONLY_ENUMERABLE.0 | Self::SKIP_SYMBOLS.0);
}

// ── 枚举 (property-details.h:85-93, 255-265) ─────────────────────────────
/// 属性种类: 数据或访问器 (顺序有意义, 须放入 KindField)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PropertyKind {
  Data = 0,
  Accessor = 1,
}

/// 属性存储位置: 字段或描述符 (kField 比 kDescriptor 更泛化)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PropertyLocation {
  Field = 0,
  Descriptor = 1,
}

/// 属性常量性: kMutable 更泛化。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PropertyConstness {
  Mutable = 0,
  Const = 1,
}

/// PropertyCell 的单元格类型 (仅当 cell 仍有效时有意义)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PropertyCellType {
  /// 不再作为常量追踪。
  Mutable = 0,
  /// PropertyCell 的 PREMONOMORPHIC 态。
  Undefined = 1,
  /// 仅赋值过一次。
  Constant = 2,
  /// 仅赋值过一种类型。
  ConstantType = 3,
  /// 状态迁移中的临时值 (仅后台线程可见)。
  InTransition = 4,
}

impl PropertyCellType {
  /// 非持有 cell 的字典值 (原版 kNoCell = kMutable 别名; 须为 0)。
  pub const NO_CELL: Self = Self::Mutable;
}

// ── 位段常量 (BitField 链机械展开; 见脚本头映射登记) ────────────────────────
/// kind 位段起始位 (property-details.h BitField 链机械展开)
pub const KIND_SHIFT: u32 = 0;
/// kind 位段宽度
pub const KIND_SIZE: u32 = 1;
/// kind 位段掩码
pub const KIND_MASK: u32 = (1u32 << 1) - 1;

/// constness 位段起始位 (property-details.h BitField 链机械展开)
pub const CONSTNESS_SHIFT: u32 = 1;
/// constness 位段宽度
pub const CONSTNESS_SIZE: u32 = 1;
/// constness 位段掩码
pub const CONSTNESS_MASK: u32 = ((1u32 << 1) - 1) << 1;

/// attributes 位段起始位 (property-details.h BitField 链机械展开)
pub const ATTRIBUTES_SHIFT: u32 = 2;
/// attributes 位段宽度
pub const ATTRIBUTES_SIZE: u32 = 3;
/// attributes 位段掩码
pub const ATTRIBUTES_MASK: u32 = ((1u32 << 3) - 1) << 2;

/// property_cell_type 位段起始位 (property-details.h BitField 链机械展开)
pub const PROPERTY_CELL_TYPE_SHIFT: u32 = 5;
/// property_cell_type 位段宽度
pub const PROPERTY_CELL_TYPE_SIZE: u32 = 3;
/// property_cell_type 位段掩码
pub const PROPERTY_CELL_TYPE_MASK: u32 = ((1u32 << 3) - 1) << 5;

/// dictionary_storage 位段起始位 (property-details.h BitField 链机械展开)
pub const DICTIONARY_STORAGE_SHIFT: u32 = 8;
/// dictionary_storage 位段宽度
pub const DICTIONARY_STORAGE_SIZE: u32 = 23;
/// dictionary_storage 位段掩码
pub const DICTIONARY_STORAGE_MASK: u32 = ((1u32 << 23) - 1) << 8;

/// location 位段起始位 (property-details.h BitField 链机械展开)
pub const LOCATION_SHIFT: u32 = 5;
/// location 位段宽度
pub const LOCATION_SIZE: u32 = 1;
/// location 位段掩码
pub const LOCATION_MASK: u32 = ((1u32 << 1) - 1) << 5;

/// representation 位段起始位 (property-details.h BitField 链机械展开)
pub const REPRESENTATION_SHIFT: u32 = 6;
/// representation 位段宽度
pub const REPRESENTATION_SIZE: u32 = 3;
/// representation 位段掩码
pub const REPRESENTATION_MASK: u32 = ((1u32 << 3) - 1) << 6;

/// descriptor_pointer 位段起始位 (property-details.h BitField 链机械展开)
pub const DESCRIPTOR_POINTER_SHIFT: u32 = 9;
/// descriptor_pointer 位段宽度
pub const DESCRIPTOR_POINTER_SIZE: u32 = 10;
/// descriptor_pointer 位段掩码
pub const DESCRIPTOR_POINTER_MASK: u32 = ((1u32 << 10) - 1) << 9;

/// offset_in_words 位段起始位 (property-details.h BitField 链机械展开)
pub const OFFSET_IN_WORDS_SHIFT: u32 = 19;
/// offset_in_words 位段宽度
pub const OFFSET_IN_WORDS_SIZE: u32 = 11;
/// offset_in_words 位段掩码
pub const OFFSET_IN_WORDS_MASK: u32 = ((1u32 << 11) - 1) << 19;

/// in_object 位段起始位 (property-details.h BitField 链机械展开)
pub const IN_OBJECT_SHIFT: u32 = 30;
/// in_object 位段宽度
pub const IN_OBJECT_SIZE: u32 = 1;
/// in_object 位段掩码
pub const IN_OBJECT_MASK: u32 = ((1u32 << 1) - 1) << 30;

/// 字典模式属性掩码下的只读属性位 (原版 kAttributesReadOnlyMask)。
pub const ATTRIBUTES_READ_ONLY_MASK: u32 =
  (PropertyAttributes::READ_ONLY.0 as u32) << ATTRIBUTES_SHIFT;
/// 字典模式属性掩码下的不可删除位 (原版 kAttributesDontDeleteMask)。
pub const ATTRIBUTES_DONT_DELETE_MASK: u32 =
  (PropertyAttributes::DONT_DELETE.0 as u32) << ATTRIBUTES_SHIFT;
/// 字典模式属性掩码下的不可枚举位 (原版 kAttributesDontEnumMask)。
pub const ATTRIBUTES_DONT_ENUM_MASK: u32 =
  (PropertyAttributes::DONT_ENUM.0 as u32) << ATTRIBUTES_SHIFT;

/// 位段取值: (value & mask) >> shift。
#[inline]
const fn bits_get(value: u32, shift: u32, mask: u32) -> u32 {
  (value & mask) >> shift
}

/// 位段放置: 覆盖 value 中 [shift, shift+size) 位为 bits。
#[inline]
const fn bits_set(value: u32, shift: u32, mask: u32, bits: u32) -> u32 {
  (value & !mask) | ((bits << shift) & mask)
}

// ── Representation (property-details.h:95-240) ───────────────────────────
/// 字段表示法 (字段值的最具体类型信息)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Representation {
  kind: Kind,
}

/// 表示法种类 (原版 Representation::Kind)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i8)]
pub enum Kind {
  None = 0,
  Smi = 1,
  Double = 2,
  HeapObject = 3,
  Tagged = 4,
  /// WasmObject 字段: 实际类型取自 Map 关联的 Wasm RTT。
  WasmValue = 5,
}

impl Representation {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self { kind: Kind::None }
  }
  #[inline]
  #[must_use]
  pub const fn tagged() -> Self {
    Self { kind: Kind::Tagged }
  }
  #[inline]
  #[must_use]
  pub const fn smi() -> Self {
    Self { kind: Kind::Smi }
  }
  #[inline]
  #[must_use]
  pub const fn double() -> Self {
    Self { kind: Kind::Double }
  }
  #[inline]
  #[must_use]
  pub const fn heap_object() -> Self {
    Self {
      kind: Kind::HeapObject,
    }
  }
  #[inline]
  #[must_use]
  pub const fn wasm_value() -> Self {
    Self {
      kind: Kind::WasmValue,
    }
  }
  #[inline]
  #[must_use]
  pub const fn from_kind(kind: Kind) -> Self {
    Self { kind }
  }

  #[inline]
  #[must_use]
  pub const fn kind(&self) -> Kind {
    self.kind
  }
  #[inline]
  #[must_use]
  pub const fn is_none(&self) -> bool {
    matches!(self.kind, Kind::None)
  }
  #[inline]
  #[must_use]
  pub const fn is_wasm_value(&self) -> bool {
    matches!(self.kind, Kind::WasmValue)
  }
  #[inline]
  #[must_use]
  pub const fn is_tagged(&self) -> bool {
    matches!(self.kind, Kind::Tagged)
  }
  #[inline]
  #[must_use]
  pub const fn is_smi(&self) -> bool {
    matches!(self.kind, Kind::Smi)
  }
  #[inline]
  #[must_use]
  pub const fn is_smi_or_tagged(&self) -> bool {
    matches!(self.kind, Kind::Smi | Kind::Tagged)
  }
  #[inline]
  #[must_use]
  pub const fn is_double(&self) -> bool {
    matches!(self.kind, Kind::Double)
  }
  #[inline]
  #[must_use]
  pub const fn is_heap_object(&self) -> bool {
    matches!(self.kind, Kind::HeapObject)
  }

  /// 加载兼容 (原版 IsCompatibleForLoad): 双精度位一致即可。
  #[inline]
  #[must_use]
  pub const fn is_compatible_for_load(&self, other: &Self) -> bool {
    self.is_double() == other.is_double()
  }
  /// 存储兼容 (原版 IsCompatibleForStore)。
  #[inline]
  #[must_use]
  pub const fn is_compatible_for_store(&self, other: &Self) -> bool {
    self.equals(other)
  }
  #[inline]
  #[must_use]
  pub const fn equals(&self, other: &Self) -> bool {
    // const fn 禁 enum == → 比较判别值
    (self.kind as u8) == (other.kind as u8)
  }

  /// 变更到更泛化表示是否可能触发 map 弃用 (原版 MightCauseMapDeprecation)。
  #[inline]
  #[must_use]
  pub const fn might_cause_map_deprecation(&self) -> bool {
    // HeapObject→Tagged 可原地改; 装箱 double→Tagged 迁移恒为原地;
    // WasmValue 视作不可变更 (等价 "原地")。None/Smi→Double 需装箱分配 → 需弃用。
    !(self.is_tagged() || self.is_heap_object() || self.is_double() || self.is_wasm_value())
  }

  /// 是否可原地变更为 other (原版 CanBeInPlaceChangedTo)。
  #[inline]
  #[must_use]
  pub const fn can_be_in_place_changed_to(&self, other: &Self) -> bool {
    if self.equals(other) {
      return true;
    }
    if self.is_wasm_value() || other.is_wasm_value() {
      return false;
    }
    // None 的默认未初始化值可被 smi/tagged 覆写, 但 double 需装箱分配。
    if self.is_none() {
      return !other.is_double();
    }
    // 其余 (Smi/Double/HeapObject) 只能原地泛化到 Tagged。
    other.is_tagged()
  }

  /// 可原地变更的最泛化表示 (原版 MostGenericInPlaceChange)。
  #[inline]
  #[must_use]
  pub const fn most_generic_in_place_change(&self) -> Self {
    if self.is_wasm_value() {
      return Self::wasm_value();
    }
    Self::tagged()
  }

  /// 严格更泛化判定 (原版 is_more_general_than; Kind 判别值顺序)。
  #[inline]
  #[must_use]
  pub const fn is_more_general_than(&self, other: &Self) -> bool {
    if self.is_wasm_value() {
      return false;
    }
    if self.is_heap_object() {
      return other.is_none();
    }
    (self.kind as u8) > (other.kind as u8)
  }

  /// 本表示可否放入 other (原版 fits_into)。
  #[inline]
  #[must_use]
  pub const fn fits_into(&self, other: &Self) -> bool {
    other.is_more_general_than(self) || other.equals(self)
  }

  /// 泛化合并 (原版 generalize; 不相容时取 Tagged)。
  #[inline]
  #[must_use]
  pub const fn generalize(&self, other: Self) -> Self {
    if other.fits_into(self) {
      return *self;
    }
    if other.is_more_general_than(self) {
      return other;
    }
    Self::tagged()
  }

  /// 字段字节大小 (原版 size; None 为非法输入)。
  #[inline]
  #[must_use]
  pub const fn size(&self) -> usize {
    debug_assert!(!self.is_none());
    if self.is_double() {
      8
    } else {
      4 // kTaggedSize (压缩指针默认配置)
    }
  }

  /// 单字符助记符 (原版 Mnemonic)。
  #[inline]
  #[must_use]
  pub const fn mnemonic(&self) -> &'static str {
    match self.kind {
      Kind::None => "v",
      Kind::Tagged => "t",
      Kind::Smi => "s",
      Kind::Double => "d",
      Kind::HeapObject => "h",
      Kind::WasmValue => "w",
    }
  }

  /// 位段编码值 (原版 EncodeRepresentation = kind 判别值)。
  #[inline]
  #[must_use]
  pub const fn encode(&self) -> u32 {
    self.kind as u32
  }

  /// 位段解码 (原版 DecodeRepresentation)。
  #[inline]
  #[must_use]
  pub const fn decode(bits: u32) -> Self {
    Self::from_kind(match bits {
      0 => Kind::None,
      1 => Kind::Smi,
      2 => Kind::Double,
      3 => Kind::HeapObject,
      4 => Kind::Tagged,
      _ => Kind::WasmValue,
    })
  }
}

// ── FieldStorageLocation (property-details.h:267-307) ────────────────────
/// 字段存储位置 (对象内或 PropertyArray 外置)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldStorageLocation {
  /// 自存储区起 (JSObject 首地址或 PropertyArray 首地址) 的偏移, 单位: word。
  pub offset_in_words: u16,
  /// 是否在对象内。
  pub is_in_object: bool,
}

impl FieldStorageLocation {
  /// PropertyArray 首个外置字段偏移 (map + length = 2 words)。
  pub const FIRST_OUT_OF_OBJECT_OFFSET_IN_WORDS: u16 = 2;

  /// 首字段位置 (原版 First): 对象内有空间则内置, 否则外置。
  #[inline]
  #[must_use]
  pub const fn first(in_object_start_in_words: u8, instance_size_in_words: u16) -> Self {
    if (in_object_start_in_words as u16) < instance_size_in_words {
      Self {
        offset_in_words: in_object_start_in_words as u16,
        is_in_object: true,
      }
    } else {
      Self {
        offset_in_words: Self::FIRST_OUT_OF_OBJECT_OFFSET_IN_WORDS,
        is_in_object: false,
      }
    }
  }

  /// 下一字段位置 (原版 Next): 内置装满后外移到 PropertyArray。
  #[inline]
  #[must_use]
  pub const fn next(&self, instance_size_in_words: u16, field_width_in_words: u16) -> Self {
    let next_offset_in_words = self.offset_in_words + field_width_in_words;
    if !self.is_in_object {
      return Self {
        offset_in_words: next_offset_in_words,
        is_in_object: false,
      };
    }
    if next_offset_in_words == instance_size_in_words {
      // 下一属性将溢出对象内字段, 移到外置。
      return Self {
        offset_in_words: Self::FIRST_OUT_OF_OBJECT_OFFSET_IN_WORDS,
        is_in_object: false,
      };
    }
    Self {
      offset_in_words: next_offset_in_words,
      is_in_object: true,
    }
  }
}

// ── PropertyDetails (property-details.h:311-588) ─────────────────────────
/// 属性详情位编码 (fast 与 dictionary 对象共用一个 32 位字)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PropertyDetails {
  value: u32,
}

impl PropertyDetails {
  /// 字典索引初值 (原版 kInitialIndex)。
  pub const INITIAL_INDEX: u32 = 1;
  /// 字典常量性追踪默认关闭 (V8_DICT_PROPERTY_CONST_TRACKING_BOOL=false)。
  pub const CONST_IF_DICT_CONST_TRACKING: PropertyConstness = PropertyConstness::Mutable;

  /// 字典全局 (GlobalDictionary) 属性 (原版首个构造)。
  #[inline]
  #[must_use]
  pub const fn for_global_dictionary(
    kind: PropertyKind,
    attributes: PropertyAttributes,
    cell_type: PropertyCellType,
    dictionary_index: u32,
  ) -> Self {
    // 全局属性经 PropertyCellTypeField 追踪常量性, Constness 固定 kMutable 简化断言。
    Self {
      value: bits_set(
        bits_set(
          bits_set(
            bits_set(
              bits_set(
                bits_set(0, KIND_SHIFT, KIND_MASK, kind as u32),
                LOCATION_SHIFT,
                LOCATION_MASK,
                PropertyLocation::Field as u32,
              ),
              ATTRIBUTES_SHIFT,
              ATTRIBUTES_MASK,
              attributes.0 as u32,
            ),
            CONSTNESS_SHIFT,
            CONSTNESS_MASK,
            PropertyConstness::Mutable as u32,
          ),
          DICTIONARY_STORAGE_SHIFT,
          DICTIONARY_STORAGE_MASK,
          dictionary_index,
        ),
        PROPERTY_CELL_TYPE_SHIFT,
        PROPERTY_CELL_TYPE_MASK,
        cell_type as u32,
      ),
    }
  }

  /// 字典模式属性/元素 (原版第二个构造)。
  #[inline]
  #[must_use]
  pub const fn for_dictionary(
    kind: PropertyKind,
    attributes: PropertyAttributes,
    constness: PropertyConstness,
    dictionary_index: u32,
  ) -> Self {
    Self {
      value: bits_set(
        bits_set(
          bits_set(
            bits_set(
              bits_set(0, KIND_SHIFT, KIND_MASK, kind as u32),
              LOCATION_SHIFT,
              LOCATION_MASK,
              PropertyLocation::Field as u32,
            ),
            ATTRIBUTES_SHIFT,
            ATTRIBUTES_MASK,
            attributes.0 as u32,
          ),
          CONSTNESS_SHIFT,
          CONSTNESS_MASK,
          constness as u32,
        ),
        DICTIONARY_STORAGE_SHIFT,
        DICTIONARY_STORAGE_MASK,
        dictionary_index,
      ),
    }
  }

  /// 快模式属性 (原版第三个构造; field_offset 单位字节, 须 word 对齐)。
  #[inline]
  #[must_use]
  pub const fn for_fast_field(
    kind: PropertyKind,
    attributes: PropertyAttributes,
    location: PropertyLocation,
    constness: PropertyConstness,
    representation: Representation,
    field_offset: u32,
    in_object: bool,
  ) -> Self {
    debug_assert!(field_offset.is_multiple_of(4)); // kTaggedSize 对齐
    Self {
      value: bits_set(
        bits_set(
          bits_set(
            bits_set(
              bits_set(
                bits_set(0, KIND_SHIFT, KIND_MASK, kind as u32),
                ATTRIBUTES_SHIFT,
                ATTRIBUTES_MASK,
                attributes.0 as u32,
              ),
              LOCATION_SHIFT,
              LOCATION_MASK,
              location as u32,
            ),
            CONSTNESS_SHIFT,
            CONSTNESS_MASK,
            constness as u32,
          ),
          REPRESENTATION_SHIFT,
          REPRESENTATION_MASK,
          representation.encode(),
        ),
        OFFSET_IN_WORDS_SHIFT,
        OFFSET_IN_WORDS_MASK,
        field_offset / 4,
      ) | ((in_object as u32) << IN_OBJECT_SHIFT),
    }
  }

  /// 快模式属性 (不带实际索引; 原版第四个构造)。
  #[inline]
  #[must_use]
  pub const fn for_fast(
    kind: PropertyKind,
    attributes: PropertyAttributes,
    location: PropertyLocation,
    constness: PropertyConstness,
    representation: Representation,
  ) -> Self {
    Self {
      value: bits_set(
        bits_set(
          bits_set(
            bits_set(0, KIND_SHIFT, KIND_MASK, kind as u32),
            ATTRIBUTES_SHIFT,
            ATTRIBUTES_MASK,
            attributes.0 as u32,
          ),
          LOCATION_SHIFT,
          LOCATION_MASK,
          location as u32,
        ),
        CONSTNESS_SHIFT,
        CONSTNESS_MASK,
        constness as u32,
      ) | (representation.encode() << REPRESENTATION_SHIFT),
    }
  }

  /// 空详情 (原版 Empty)。
  #[inline]
  #[must_use]
  pub const fn empty() -> Self {
    Self::for_global_dictionary(
      PropertyKind::Data,
      PropertyAttributes::NONE,
      PropertyCellType::NO_CELL,
      0,
    )
  }

  /// 以原始位值构造 (调用者保证值域)。
  #[inline]
  #[must_use]
  pub const fn from_raw(value: u32) -> Self {
    Self { value }
  }

  /// 原始位值。
  #[inline]
  #[must_use]
  pub const fn to_raw(&self) -> u32 {
    self.value
  }

  /// 从 Smi 载入 (property-details-inl.h:17)。
  #[inline]
  #[must_use]
  pub const fn from_smi(smi: Tagged<Smi>) -> Self {
    Self {
      value: Smi::value(smi) as u32,
    }
  }

  /// 编码为 Smi (property-details-inl.h:19-24):
  /// 左移 1 再算术右移做符号扩展, 使第 31 位可用。
  #[inline]
  #[must_use]
  pub fn as_smi(&self) -> Tagged<Smi> {
    let value = ((self.value << 1) as i32) >> 1;
    Smi::from_int(value)
  }

  /// 属性种类。
  #[inline]
  #[must_use]
  pub const fn kind(&self) -> PropertyKind {
    match bits_get(self.value, KIND_SHIFT, KIND_MASK) {
      0 => PropertyKind::Data,
      _ => PropertyKind::Accessor,
    }
  }

  /// 存储位置。
  #[inline]
  #[must_use]
  pub const fn location(&self) -> PropertyLocation {
    match bits_get(self.value, LOCATION_SHIFT, LOCATION_MASK) {
      0 => PropertyLocation::Field,
      _ => PropertyLocation::Descriptor,
    }
  }

  /// 常量性。
  #[inline]
  #[must_use]
  pub const fn constness(&self) -> PropertyConstness {
    match bits_get(self.value, CONSTNESS_SHIFT, CONSTNESS_MASK) {
      0 => PropertyConstness::Mutable,
      _ => PropertyConstness::Const,
    }
  }

  /// 属性标志。
  #[inline]
  #[must_use]
  pub const fn attributes(&self) -> PropertyAttributes {
    PropertyAttributes(bits_get(self.value, ATTRIBUTES_SHIFT, ATTRIBUTES_MASK) as u8)
  }

  /// 种类与标志同时匹配 (原版 HasKindAndAttributes)。
  #[inline]
  #[must_use]
  pub const fn has_kind_and_attributes(
    &self,
    kind: PropertyKind,
    attributes: PropertyAttributes,
  ) -> bool {
    (self.value & (KIND_MASK | ATTRIBUTES_MASK))
      == ((kind as u32) << KIND_SHIFT) | ((attributes.0 as u32) << ATTRIBUTES_SHIFT)
  }

  /// 字典序号 (原版 dictionary_index)。
  #[inline]
  #[must_use]
  pub const fn dictionary_index(&self) -> u32 {
    bits_get(
      self.value,
      DICTIONARY_STORAGE_SHIFT,
      DICTIONARY_STORAGE_MASK,
    )
  }

  /// 字段表示法。
  #[inline]
  #[must_use]
  pub const fn representation(&self) -> Representation {
    Representation::decode(bits_get(
      self.value,
      REPRESENTATION_SHIFT,
      REPRESENTATION_MASK,
    ))
  }

  /// 字段偏移 (单位: word; 原版 field_offset)。
  #[inline]
  #[must_use]
  pub const fn field_offset(&self) -> u16 {
    bits_get(self.value, OFFSET_IN_WORDS_SHIFT, OFFSET_IN_WORDS_MASK) as u16
  }

  /// 字段宽度 (单位: word; property-details-inl.h:26-29, 恒 1)。
  #[inline]
  #[must_use]
  pub const fn field_width_in_words(&self) -> u32 {
    debug_assert!(matches!(self.location(), PropertyLocation::Field));
    1
  }

  /// 是否对象内字段。
  #[inline]
  #[must_use]
  pub const fn is_in_object(&self) -> bool {
    bits_get(self.value, IN_OBJECT_SHIFT, IN_OBJECT_MASK) != 0
  }

  /// 存储位置结构体 (原版 storage_location)。
  #[inline]
  #[must_use]
  pub const fn storage_location(&self) -> FieldStorageLocation {
    FieldStorageLocation {
      offset_in_words: self.field_offset(),
      is_in_object: self.is_in_object(),
    }
  }

  /// 只读属性。
  #[inline]
  #[must_use]
  pub const fn is_read_only(&self) -> bool {
    self.attributes().contains(PropertyAttributes::READ_ONLY)
  }
  /// 可配置 (= 未设 DONT_DELETE)。
  #[inline]
  #[must_use]
  pub const fn is_configurable(&self) -> bool {
    !self.attributes().contains(PropertyAttributes::DONT_DELETE)
  }
  /// 不可枚举。
  #[inline]
  #[must_use]
  pub const fn is_dont_enum(&self) -> bool {
    self.attributes().contains(PropertyAttributes::DONT_ENUM)
  }
  /// 可枚举。
  #[inline]
  #[must_use]
  pub const fn is_enumerable(&self) -> bool {
    !self.is_dont_enum()
  }

  /// PropertyCell 类型 (字典全局属性)。
  #[inline]
  #[must_use]
  pub const fn cell_type(&self) -> PropertyCellType {
    match bits_get(
      self.value,
      PROPERTY_CELL_TYPE_SHIFT,
      PROPERTY_CELL_TYPE_MASK,
    ) {
      1 => PropertyCellType::Undefined,
      2 => PropertyCellType::Constant,
      3 => PropertyCellType::ConstantType,
      4 => PropertyCellType::InTransition,
      _ => PropertyCellType::Mutable,
    }
  }

  /// 排序键索引 (原版 pointer)。
  #[inline]
  #[must_use]
  pub const fn pointer(&self) -> u32 {
    bits_get(
      self.value,
      DESCRIPTOR_POINTER_SHIFT,
      DESCRIPTOR_POINTER_MASK,
    )
  }

  /// 更新排序键索引 (原版 set_pointer, 返回新值)。
  #[inline]
  #[must_use]
  pub const fn set_pointer(&self, i: u32) -> Self {
    Self {
      value: bits_set(
        self.value,
        DESCRIPTOR_POINTER_SHIFT,
        DESCRIPTOR_POINTER_MASK,
        i,
      ),
    }
  }

  /// 更新 cell 类型 (原版 set_cell_type)。
  #[inline]
  #[must_use]
  pub const fn set_cell_type(&self, ty: PropertyCellType) -> Self {
    Self {
      value: bits_set(
        self.value,
        PROPERTY_CELL_TYPE_SHIFT,
        PROPERTY_CELL_TYPE_MASK,
        ty as u32,
      ),
    }
  }

  /// 更新字典序号 (原版 set_index)。
  #[inline]
  #[must_use]
  pub const fn set_index(&self, index: u32) -> Self {
    Self {
      value: bits_set(
        self.value,
        DICTIONARY_STORAGE_SHIFT,
        DICTIONARY_STORAGE_MASK,
        index,
      ),
    }
  }

  /// 字典序号可否容纳 (原版 CanSetIndex/IsValidIndex)。
  #[inline]
  #[must_use]
  pub const fn is_valid_index(index: u32) -> bool {
    index <= DICTIONARY_STORAGE_MASK >> DICTIONARY_STORAGE_SHIFT
  }

  /// 换表示法 (原版 CopyWithRepresentation)。
  #[inline]
  #[must_use]
  pub const fn copy_with_representation(&self, representation: Representation) -> Self {
    Self {
      value: bits_set(
        self.value,
        REPRESENTATION_SHIFT,
        REPRESENTATION_MASK,
        representation.encode(),
      ),
    }
  }

  /// 换常量性 (原版 CopyWithConstness)。
  #[inline]
  #[must_use]
  pub const fn copy_with_constness(&self, constness: PropertyConstness) -> Self {
    Self {
      value: bits_set(
        self.value,
        CONSTNESS_SHIFT,
        CONSTNESS_MASK,
        constness as u32,
      ),
    }
  }

  /// 追加属性标志位 (原版 CopyAddAttributes)。
  #[inline]
  #[must_use]
  pub const fn copy_add_attributes(&self, new_attributes: PropertyAttributes) -> Self {
    let merged = self.attributes().union(new_attributes);
    Self {
      value: bits_set(
        self.value,
        ATTRIBUTES_SHIFT,
        ATTRIBUTES_MASK,
        merged.0 as u32,
      ),
    }
  }

  /// 压缩为单字节 (原版 ToByte; 仅非全局字典属性, 枚举序号须为 0)。
  #[inline]
  #[must_use]
  pub const fn to_byte(&self) -> u8 {
    debug_assert!(matches!(self.location(), PropertyLocation::Field));
    debug_assert!(matches!(self.cell_type(), PropertyCellType::Mutable));
    debug_assert!(self.dictionary_index() == 0);
    self.value as u8
  }

  /// 从 ToByte 产物还原 (原版 FromByte; 0 扩展即置 Field/NoCell/序号 0)。
  #[inline]
  #[must_use]
  pub const fn from_byte(encoded: u8) -> Self {
    Self {
      value: encoded as u32,
    }
  }
}

// ── 泛化谓词 (property-details.h:590-605) ────────────────────────────────
/// kField 位置比 kDescriptor 更泛化; kDescriptor 只泛化到自身。
#[inline]
#[must_use]
pub const fn location_is_generalizable_to(a: PropertyLocation, b: PropertyLocation) -> bool {
  matches!(b, PropertyLocation::Field) || matches!(a, PropertyLocation::Descriptor)
}

/// kMutable 常量性比 kConst 更泛化; kConst 只泛化到自身。
#[inline]
#[must_use]
pub const fn constness_is_generalizable_to(a: PropertyConstness, b: PropertyConstness) -> bool {
  matches!(b, PropertyConstness::Mutable) || matches!(a, PropertyConstness::Const)
}

/// 常量性合并 (原版 GeneralizeConstness): 任一 Mutable 即 Mutable。
#[inline]
#[must_use]
pub const fn generalize_constness(a: PropertyConstness, b: PropertyConstness) -> PropertyConstness {
  if matches!(a, PropertyConstness::Mutable) {
    PropertyConstness::Mutable
  } else {
    b
  }
}

// 编译期布局对数 (对齐原版 static_assert 组)
const _: () = assert!(DICTIONARY_STORAGE_SHIFT + DICTIONARY_STORAGE_SIZE - 1 == 30);
const _: () = assert!(IN_OBJECT_SHIFT + IN_OBJECT_SIZE - 1 == 30);
const _: () = assert!(KIND_SHIFT + KIND_SIZE - 1 < 8);
const _: () = assert!(CONSTNESS_SHIFT + CONSTNESS_SIZE - 1 < 8);
const _: () = assert!(ATTRIBUTES_SHIFT + ATTRIBUTES_SIZE - 1 < 8);
