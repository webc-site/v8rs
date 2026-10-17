// 由 tool/src/objects/map.tq.js 从 v8/src/objects/map.tq 生成; 修订就地改, 复跑覆盖
// Map tq 位段/布局字段表 (clang -E map.h 对数, 压缩+WASM 配置)
//! Map 的 Torque 侧布局镜像: 位段表 + 字段表 + macro 清单。
//! 目标配置: 压缩指针 (K_TAGGED_SIZE=4, TAGGED_SIZE_8_BYTES 关 → 无 optional_padding),
//! WASM 开 (instance_descriptors/dependent_code 取联合类型分支), INTL 开。

// ── MapBits1 (tq 位序 1:1; 容器 uint8, 占 7 位) ──
/// is_callable: bool (1 bit @0)。
pub const MAP_BITS1_IS_CALLABLE_SHIFT: u32 = 0;
pub const MAP_BITS1_IS_CALLABLE_MASK: u8 = 0x1;
/// is_undetectable: bool (1 bit @1)。
pub const MAP_BITS1_IS_UNDETECTABLE_SHIFT: u32 = 1;
pub const MAP_BITS1_IS_UNDETECTABLE_MASK: u8 = 0x2;
/// has_named_interceptor: bool (1 bit @2)。
pub const MAP_BITS1_HAS_NAMED_INTERCEPTOR_SHIFT: u32 = 2;
pub const MAP_BITS1_HAS_NAMED_INTERCEPTOR_MASK: u8 = 0x4;
/// is_access_check_needed: bool (1 bit @3)。
pub const MAP_BITS1_IS_ACCESS_CHECK_NEEDED_SHIFT: u32 = 3;
pub const MAP_BITS1_IS_ACCESS_CHECK_NEEDED_MASK: u8 = 0x8;
/// has_indexed_interceptor: bool (1 bit @4)。
pub const MAP_BITS1_HAS_INDEXED_INTERCEPTOR_SHIFT: u32 = 4;
pub const MAP_BITS1_HAS_INDEXED_INTERCEPTOR_MASK: u8 = 0x10;
/// is_constructor: bool (1 bit @5)。
pub const MAP_BITS1_IS_CONSTRUCTOR_SHIFT: u32 = 5;
pub const MAP_BITS1_IS_CONSTRUCTOR_MASK: u8 = 0x20;
/// is_extended_map: bool (1 bit @6)。
pub const MAP_BITS1_IS_EXTENDED_MAP_SHIFT: u32 = 6;
pub const MAP_BITS1_IS_EXTENDED_MAP_MASK: u8 = 0x40;

// ── MapBits2 (tq 位序 1:1; 容器 uint8, 占 8 位) ──
/// new_target_is_base: bool (1 bit @0)。
pub const MAP_BITS2_NEW_TARGET_IS_BASE_SHIFT: u32 = 0;
pub const MAP_BITS2_NEW_TARGET_IS_BASE_MASK: u8 = 0x1;
/// is_immutable_prototype: bool (1 bit @1)。
pub const MAP_BITS2_IS_IMMUTABLE_PROTOTYPE_SHIFT: u32 = 1;
pub const MAP_BITS2_IS_IMMUTABLE_PROTOTYPE_MASK: u8 = 0x2;
/// elements_kind: ElementsKind (6 bit @2)。
pub const MAP_BITS2_ELEMENTS_KIND_SHIFT: u32 = 2;
pub const MAP_BITS2_ELEMENTS_KIND_MASK: u8 = 0xfc;

// ── MapBits3 (tq 位序 1:1; 容器 uint32, 占 32 位) ──
/// enum_length: int32 (10 bit @0)。
pub const MAP_BITS3_ENUM_LENGTH_SHIFT: u32 = 0;
pub const MAP_BITS3_ENUM_LENGTH_MASK: u32 = 0x3ff;
/// number_of_own_descriptors: int32 (10 bit @10)。
pub const MAP_BITS3_NUMBER_OF_OWN_DESCRIPTORS_SHIFT: u32 = 10;
pub const MAP_BITS3_NUMBER_OF_OWN_DESCRIPTORS_MASK: u32 = 0xffc00;
/// is_prototype_map: bool (1 bit @20)。
pub const MAP_BITS3_IS_PROTOTYPE_MAP_SHIFT: u32 = 20;
pub const MAP_BITS3_IS_PROTOTYPE_MAP_MASK: u32 = 0x100000;
/// is_dictionary_map: bool (1 bit @21)。
pub const MAP_BITS3_IS_DICTIONARY_MAP_SHIFT: u32 = 21;
pub const MAP_BITS3_IS_DICTIONARY_MAP_MASK: u32 = 0x200000;
/// owns_descriptors: bool (1 bit @22)。
pub const MAP_BITS3_OWNS_DESCRIPTORS_SHIFT: u32 = 22;
pub const MAP_BITS3_OWNS_DESCRIPTORS_MASK: u32 = 0x400000;
/// is_in_retained_map_list: bool (1 bit @23)。
pub const MAP_BITS3_IS_IN_RETAINED_MAP_LIST_SHIFT: u32 = 23;
pub const MAP_BITS3_IS_IN_RETAINED_MAP_LIST_MASK: u32 = 0x800000;
/// is_deprecated: bool (1 bit @24)。
pub const MAP_BITS3_IS_DEPRECATED_SHIFT: u32 = 24;
pub const MAP_BITS3_IS_DEPRECATED_MASK: u32 = 0x1000000;
/// is_unstable: bool (1 bit @25)。
pub const MAP_BITS3_IS_UNSTABLE_SHIFT: u32 = 25;
pub const MAP_BITS3_IS_UNSTABLE_MASK: u32 = 0x2000000;
/// is_migration_target: bool (1 bit @26)。
pub const MAP_BITS3_IS_MIGRATION_TARGET_SHIFT: u32 = 26;
pub const MAP_BITS3_IS_MIGRATION_TARGET_MASK: u32 = 0x4000000;
/// is_extensible: bool (1 bit @27)。
pub const MAP_BITS3_IS_EXTENSIBLE_SHIFT: u32 = 27;
pub const MAP_BITS3_IS_EXTENSIBLE_MASK: u32 = 0x8000000;
/// may_have_interesting_properties: bool (1 bit @28)。
pub const MAP_BITS3_MAY_HAVE_INTERESTING_PROPERTIES_SHIFT: u32 = 28;
pub const MAP_BITS3_MAY_HAVE_INTERESTING_PROPERTIES_MASK: u32 = 0x10000000;
/// construction_counter: int32 (3 bit @29)。
pub const MAP_BITS3_CONSTRUCTION_COUNTER_SHIFT: u32 = 29;
pub const MAP_BITS3_CONSTRUCTION_COUNTER_MASK: u32 = 0xe0000000;

// ── ExtendedMapBitsEx (tq 位序 1:1; 容器 uint8, 占 8 位) ──
/// map_kind: ExtendedMapKind (3 bit @0)。
pub const EXTENDED_MAP_BITS_EX_MAP_KIND_SHIFT: u32 = 0;
pub const EXTENDED_MAP_BITS_EX_MAP_KIND_MASK: u8 = 0x7;
/// map_size_in_words: uint8 (5 bit @3)。
pub const EXTENDED_MAP_BITS_EX_MAP_SIZE_IN_WORDS_SHIFT: u32 = 3;
pub const EXTENDED_MAP_BITS_EX_MAP_SIZE_IN_WORDS_MASK: u8 = 0xf8;

/// ExtendedMapKind 首个已定义变体 (开放枚举; 全量见 v8_common::globals::ExtendedMapKind)。
pub const EXTENDED_MAP_KIND_JS_INTERCEPTOR_MAP: u8 = 0;

/// tq extern class Map 布局字段表 (压缩+WASM 配置展开, 顺序 1:1)。
pub const MAP_LAYOUT_FIELDS: [(&str, &str); 14] = [
  ("instance_size_in_words", "uint8"),
  (
    "inobject_properties_start_or_constructor_function_index",
    "uint8",
  ),
  ("used_or_unused_instance_size_in_words", "uint8"),
  ("visitor_id", "uint8"),
  ("instance_type", "InstanceType"),
  ("bit_field", "MapBits1"),
  ("bit_field2", "MapBits2"),
  ("bit_field3", "MapBits3"),
  ("prototype", "JSPrototype"),
  ("constructor_or_back_pointer_or_native_context", "Object"),
  ("instance_descriptors", "DescriptorArray | WasmStruct"),
  ("dependent_code", "DependentCode | Map"),
  ("prototype_validity_cell", "Zero | Cell"),
  (
    "transitions_or_prototype_info",
    "Map | Weak<Map> | TransitionArray | PrototypeInfo | PrototypeSharedClosureInfo | Zero",
  ),
];

/// TAGGED_SIZE_8_BYTES 分支的 optional_padding 是否存在 (压缩配置 = 否)。
pub const OPTIONAL_PADDING_PRESENT: bool = false;

/// 类内 macro 清单 (macro PrototypeInfo / PrototypeSharedClosureInfo / IsUndetectable)。
pub const MAP_CLASS_MACROS: [&str; 3] = [
  "PrototypeInfo",
  "PrototypeSharedClosureInfo",
  "IsUndetectable",
];

/// @export macro 清单 (+ extern macro IsSpecialReceiverInstanceType)。
pub const MAP_EXPORT_MACROS: [&str; 4] = [
  "LoadMapPrototypeInfo",
  "LoadMapPrototypeSharedClosureInfo",
  "IsSimpleObjectMap",
  "IsSpecialReceiverInstanceType",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAP_LAYOUT_FIELDS.len() == 14);
  assert!((MAP_BITS1_IS_CALLABLE_MASK == 0x01) && (MAP_BITS1_IS_EXTENDED_MAP_MASK == 0x40));
  assert!((MAP_BITS2_ELEMENTS_KIND_SHIFT == 2) && (MAP_BITS2_ELEMENTS_KIND_MASK == 0xfc));
  assert!((MAP_BITS3_ENUM_LENGTH_MASK == 0x3ff) && (MAP_BITS3_CONSTRUCTION_COUNTER_SHIFT == 29));
  assert!(
    (EXTENDED_MAP_BITS_EX_MAP_KIND_MASK == 0x07)
      && (EXTENDED_MAP_BITS_EX_MAP_SIZE_IN_WORDS_SHIFT == 3)
  );
  assert!(!OPTIONAL_PADDING_PRESENT);
};
