// 由 tool/src/objects/literal-objects.h.js 从 v8/src/objects/literal-objects.{h,-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// 字面量描述类 5 件套: 布局 + 位段 + 索引公式 + 方法清单

//! 字面量描述族: 对象/数组/正则/类字面量的 boilerplate 与共享闭包信息。
//! 配置: 64 位非压缩 (kTaggedSize=8, FixedArrayBase 头 = map(8)+length(4)+padding(4) = 16, Struct 头 = map(8))。

// —— ClassBoilerplate::ValueKind ——
/// 原版 enum ValueKind。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ValueKind {
  /// kData
  Data = 0,
  /// kGetter
  Getter = 1,
  /// kSetter
  Setter = 2,
  /// kAutoAccessor
  AutoAccessor = 3,
}

impl ValueKind {
  /// 变体数
  pub const COUNT: usize = 4;
}

// —— ComputedEntryFlags 位段 ——
/// 原版 ValueKindBits (shift 0, 2 bit)。
pub const COMPUTED_ENTRY_VALUE_KIND_SHIFT: u32 = 0;
pub const COMPUTED_ENTRY_VALUE_KIND_BITS: u32 = 2;
/// 原版 KeyIndexBits (shift 2, 29 bit)。
pub const COMPUTED_ENTRY_KEY_INDEX_SHIFT: u32 = 2;
pub const COMPUTED_ENTRY_KEY_INDEX_BITS: u32 = 29;

// —— DefineClassArgumentsIndices ——
/// 原版 kConstructorArgumentIndex。
pub const K_CONSTRUCTOR_ARGUMENT_INDEX: i32 = 1;
/// 原版 kPrototypeArgumentIndex。
pub const K_PROTOTYPE_ARGUMENT_INDEX: i32 = 2;
/// 原版 kFirstDynamicArgumentIndex (方法闭包与计算属性名的首个动态实参)。
pub const K_FIRST_DYNAMIC_ARGUMENT_INDEX: i32 = 3;

// —— ClassBoilerplate 常量 ——
/// 原版 kMinimumClassPropertiesCount。
pub const K_MINIMUM_CLASS_PROPERTIES_COUNT: usize = 6;
/// 原版 kMinimumPrototypePropertiesCount。
pub const K_MINIMUM_PROTOTYPE_PROPERTIES_COUNT: usize = 1;

// —— 64 位布局 ——
/// v8: src/objects/literal-objects.h ObjectBoilerplateDescription:
///   kHeaderSize = kLengthOffset(8) + 3 * kTaggedSize = 32 (TAGGED_SIZE_8_BYTES);
///   固定字段 backing_store_size(32)/flags(40), raw_entries 自 48 起 (TaggedArrayBase 尾续)
pub const K_OBJECT_BOILERPLATE_DESCRIPTION_HEADER_SIZE: usize = 32;
/// v8: src/objects/literal-objects.h ArrayBoilerplateDescription: flags(8) + constant_elements(16)
pub const K_ARRAY_BOILERPLATE_DESCRIPTION_SIZE: usize = 24;
/// v8: src/objects/literal-objects.h RegExpBoilerplateDescription:
///   data (TrustedPointerMember, 8) + flags (SmiTagged<JSRegExpFlags>, 8)
pub const K_REG_EXP_BOILERPLATE_DESCRIPTION_SIZE: usize = 24;
/// v8: src/objects/literal-objects.h ClassBoilerplate: 7 字段 (arguments_count Smi + 6 tagged)
pub const K_CLASS_BOILERPLATE_HEADER_SIZE: usize = 64;
/// v8: src/objects/literal-objects.h PrototypeSharedClosureInfo: 3 tagged 字段
pub const K_PROTOTYPE_SHARED_CLOSURE_INFO_SIZE: usize = 32;

// —— ObjectBoilerplateDescription 索引公式 ——
/// 原版 kElementsPerEntry。
pub const K_ELEMENTS_PER_ENTRY: usize = 2;

/// 原版 NameIndex(i) = i * 2。
#[inline]
#[must_use]
pub const fn name_index(i: usize) -> usize {
  i * K_ELEMENTS_PER_ENTRY
}

/// 原版 ValueIndex(i) = i * 2 + 1。
#[inline]
#[must_use]
pub const fn value_index(i: usize) -> usize {
  i * K_ELEMENTS_PER_ENTRY + 1
}

/// 原版 New: capacity = boilerplate * kElementsPerEntry (上限 kMaxCapacity 检查)。
#[inline]
#[must_use]
pub const fn obd_capacity(boilerplate: u32) -> u32 {
  boilerplate * K_ELEMENTS_PER_ENTRY as u32
}

/// 原版 boilerplate_properties_count = capacity / kElementsPerEntry。
#[inline]
#[must_use]
pub const fn boilerplate_properties_count(capacity: u32) -> usize {
  (capacity / K_ELEMENTS_PER_ENTRY as u32) as usize
}

// —— tq 布局表 ——
/// tq 5 类字段表 (1:1 顺序)。
pub const LITERAL_OBJECTS_TQ_LAYOUT: &[(&str, &str, &[&str])] = &[
  ("ObjectBoilerplateDescription", "FixedArrayBase", &["backing_store_size", "flags", "raw_entries[length]"]),
  ("ArrayBoilerplateDescription", "Struct", &["flags", "constant_elements"]),
  ("RegExpBoilerplateDescription", "Struct", &["data", "flags"]),
  ("ClassBoilerplate", "Struct", &["arguments_count", "static_properties_template", "static_elements_template", "static_computed_properties", "instance_properties_template", "instance_elements_template", "instance_computed_properties"]),
  ("PrototypeSharedClosureInfo", "Struct", &["boilerplate_description", "closure_feedback_cell_array", "context"]),
];

// —— -inl 方法清单 (30 定义) ——
/// v8: src/objects/literal-objects-inl.h — Rust 无 inl, 语义并入实现。
pub const LITERAL_OBJECTS_INL_METHODS: &[&str] = &[
  "ObjectBoilerplateDescription::New",
  "ObjectBoilerplateDescription::backing_store_size",
  "ObjectBoilerplateDescription::set_backing_store_size",
  "ObjectBoilerplateDescription::flags",
  "ObjectBoilerplateDescription::set_flags",
  "ObjectBoilerplateDescription::name",
  "ObjectBoilerplateDescription::value",
  "ObjectBoilerplateDescription::set_key_value",
  "ObjectBoilerplateDescription::set_value",
  "ObjectBoilerplateDescription::boilerplate_properties_count",
  "ClassBoilerplate::ClassBoilerplate",
  "ClassBoilerplate::arguments_count",
  "ClassBoilerplate::static_properties_template",
  "ClassBoilerplate::static_elements_template",
  "ClassBoilerplate::static_computed_properties",
  "ClassBoilerplate::instance_properties_template",
  "ClassBoilerplate::instance_elements_template",
  "ClassBoilerplate::instance_computed_properties",
  "ArrayBoilerplateDescription::ArrayBoilerplateDescription",
  "ArrayBoilerplateDescription::flags",
  "ArrayBoilerplateDescription::constant_elements",
  "ArrayBoilerplateDescription::elements_kind",
  "ArrayBoilerplateDescription::is_empty",
  "RegExpBoilerplateDescription::RegExpBoilerplateDescription",
  "RegExpBoilerplateDescription::data",
  "RegExpBoilerplateDescription::flags",
  "PrototypeSharedClosureInfo::PrototypeSharedClosureInfo",
  "PrototypeSharedClosureInfo::boilerplate_description",
  "PrototypeSharedClosureInfo::closure_feedback_cell_array",
  "PrototypeSharedClosureInfo::context",
];

// —— cc 语义要点 ——
/// v8: src/objects/literal-objects.cc
///   AddToPropertiesTemplate/AddToElementsTemplate → AddToDictionaryTemplate 转发
///   (NameDictionary/SwissNameDictionary/NumberDictionary 三宿主, 模板显式实例化)
///   ClassBoilerplate::New: static/instance 两 ObjectDescriptor (最小容量 6/1) →
///   AddToPropertiesTemplate/AddToElementsTemplate 填充 → 构造实参排列
///   [constructor(1), prototype(2), dynamic(3..)]
pub const LITERAL_OBJECTS_CC_METHODS: &[&str] = &[
  "ClassBoilerplate::AddToPropertiesTemplate",
  "ClassBoilerplate::AddToElementsTemplate",
  "ClassBoilerplate::New",
  "ArrayBoilerplateDescription::BriefPrintDetails",
  "RegExpBoilerplateDescription::BriefPrintDetails",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(ValueKind::COUNT == 4);
  assert!(COMPUTED_ENTRY_VALUE_KIND_BITS + COMPUTED_ENTRY_KEY_INDEX_BITS == 31);
  assert!(K_CONSTRUCTOR_ARGUMENT_INDEX == 1);
  assert!(K_PROTOTYPE_ARGUMENT_INDEX == 2);
  assert!(K_FIRST_DYNAMIC_ARGUMENT_INDEX == 3);
  assert!(K_MINIMUM_CLASS_PROPERTIES_COUNT == 6);
  assert!(K_MINIMUM_PROTOTYPE_PROPERTIES_COUNT == 1);
  assert!(K_OBJECT_BOILERPLATE_DESCRIPTION_HEADER_SIZE == 32);
  assert!(K_ARRAY_BOILERPLATE_DESCRIPTION_SIZE == 24);
  assert!(K_REG_EXP_BOILERPLATE_DESCRIPTION_SIZE == 24);
  assert!(K_CLASS_BOILERPLATE_HEADER_SIZE == 64);
  assert!(K_PROTOTYPE_SHARED_CLOSURE_INFO_SIZE == 32);
  assert!(name_index(3) == 6);
  assert!(value_index(3) == 7);
  assert!(obd_capacity(5) == 10);
  assert!(boilerplate_properties_count(10) == 5);
  assert!(LITERAL_OBJECTS_TQ_LAYOUT.len() == 5);
};
