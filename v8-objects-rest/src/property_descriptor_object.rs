// 由 tool/src/objects/property-descriptor-object.h.js 从 v8/src/objects/property-descriptor-object.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// PropertyDescriptorObject 标志位链展开与布局
//! PropertyDescriptorObject: 属性描述符堆对象; 9 位标志链 + 3 载荷字段。

/// 原版 IsEnumerableBit (bit 0)。
pub const IS_ENUMERABLE_BIT_MASK: u32 = 0x1;
/// 原版 HasEnumerableBit (bit 1)。
pub const HAS_ENUMERABLE_BIT_MASK: u32 = 0x2;
/// 原版 IsConfigurableBit (bit 2)。
pub const IS_CONFIGURABLE_BIT_MASK: u32 = 0x4;
/// 原版 HasConfigurableBit (bit 3)。
pub const HAS_CONFIGURABLE_BIT_MASK: u32 = 0x8;
/// 原版 IsWritableBit (bit 4)。
pub const IS_WRITABLE_BIT_MASK: u32 = 0x10;
/// 原版 HasWritableBit (bit 5)。
pub const HAS_WRITABLE_BIT_MASK: u32 = 0x20;
/// 原版 HasValueBit (bit 6)。
pub const HAS_VALUE_BIT_MASK: u32 = 0x40;
/// 原版 HasGetBit (bit 7)。
pub const HAS_GET_BIT_MASK: u32 = 0x80;
/// 原版 HasSetBit (bit 8)。
pub const HAS_SET_BIT_MASK: u32 = 0x100;

/// 标志位总数 (原版 kFlagCount)。
pub const FLAG_COUNT: u32 = 9;

/// 原版 enum Flag (各 = 对应位掩码; kNone = 0)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Flag {
  /// kNone
  None = 0,
  /// kIsEnumerable
  Isenumerable = 0x1,
  /// kHasEnumerable
  Hasenumerable = 0x2,
  /// kIsConfigurable
  Isconfigurable = 0x4,
  /// kHasConfigurable
  Hasconfigurable = 0x8,
  /// kIsWritable
  Iswritable = 0x10,
  /// kHasWritable
  Haswritable = 0x20,
  /// kHasValue
  Hasvalue = 0x40,
  /// kHasGet
  Hasget = 0x80,
  /// kHasSet
  Hasset = 0x100,
}

/// 常规访问器属性位组合 (原版 kRegularAccessorPropertyBits)。
pub const REGULAR_ACCESSOR_PROPERTY_BITS: u32 = 0x189;
/// 常规数据属性位组合 (原版 kRegularDataPropertyBits)。
pub const REGULAR_DATA_PROPERTY_BITS: u32 = 0x69;
/// 全部 Has 位掩码 (原版 kHasMask)。
pub const HAS_MASK: u32 = 0x1e9;

/// Torque 布局字段表 (property-descriptor-object.tq, 顺序 1:1)。
pub const LAYOUT_FIELDS: [(&str, &str); 4] = [
  ("flags", "SmiTagged<PropertyDescriptorObjectFlags>"),
  ("value", "JSAny|TheHole"),
  ("get", "JSAny|FunctionTemplateInfo|TheHole"),
  ("set", "JSAny|FunctionTemplateInfo|TheHole"),
];

/// Torque 侧位序 (bitfield struct PropertyDescriptorObjectFlags, 与 C++ 位链 1:1)。
pub const TQ_FLAG_ORDER: [&str; 9] = [
  "is_enumerable",
  "has_enumerable",
  "is_configurable",
  "has_configurable",
  "is_writable",
  "has_writable",
  "has_value",
  "has_get",
  "has_set",
];

/// 位段解码 (原版 BitField::decode)。
#[inline]
#[must_use]
pub const fn decode(flags: u32, mask: u32) -> bool {
  (flags & mask) != 0
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FLAG_COUNT == 9);
  assert!(IS_ENUMERABLE_BIT_MASK == 0x1);
  assert!(HAS_SET_BIT_MASK == 0x100);
  assert!(REGULAR_ACCESSOR_PROPERTY_BITS == 0x189);
  assert!(REGULAR_DATA_PROPERTY_BITS == 0x69);
  assert!(HAS_MASK == 0x1e9);
  assert!(LAYOUT_FIELDS.len() == 4);
  assert!(TQ_FLAG_ORDER.len() == 9);
  assert!(TQ_FLAG_ORDER[0] == "is_enumerable" && TQ_FLAG_ORDER[8] == "has_set");
};
