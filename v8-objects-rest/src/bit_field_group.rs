// 由 tool/src/objects/bit-field-group.h.js 从 v8/src/objects/bit-field-group.h 生成; 修订就地改, 复跑覆盖
// 位域组契约与 get/with 语义

//! 位域组 (原版 bit-field-group.h)
//!
//! `BitFieldGroup<Derived, U>` 是对象内位域成员的承载基类:
//! 派生 struct 里全部 `using XBit = base::BitField<..>` 按声明序无缝铺满 U,
//! metagen 由此生成 Torque bitfield struct (布局两侧互验).

/// 命名/布局规则 (原版 doc 1:1, 3 条)
pub const BIT_FIELD_GROUP_RULES: [&str; 3] = [
  "Every type alias in the struct must be a base::BitField; they are read in",
  "The Torque field name is the alias name without its Bit/Bits suffix, in",
  "The Torque struct name is the C++ name with the enclosing class names",
];

/// 位域组 (U 为无符号底层; 位链由派生类型的 BitField 别名定义)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BitFieldGroup<U: Copy> {
  /// 原始位串 (原版 value_, 0 初始化)
  pub value: U,
}

// get<Bit>/with<Bit> 依赖位段基建 (Bit::FieldType / Bit::update) 与 Derived 回包,
// 待位段类型落位后由 Developer 就地实现 (原版语义锚):
//   get<Bit>:  Bit::decode(value_)
//   with<Bit>: Derived(Bit::update(value_, value)) — 副本不落盘, 调用方写回成员
