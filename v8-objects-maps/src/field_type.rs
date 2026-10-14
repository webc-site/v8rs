// 由 tool/src/objects/field-type.h.js 从 v8/src/objects/field-type.{h,cc} 生成; 修订就地改, 复跑覆盖
// FieldType: 字段类型哨兵与签名骨架

use crate::error::Error;

/// FieldType 全静态命名空间 (原版 class FieldType : AllStatic)
/// 表示 Map 字段的类型: None(未定) / Any(任意) / Class(以 Map 限定的类)
pub struct FieldType;

impl FieldType {
  /// GC 可清除字段类型时, 每次存储必须更新字段类型 (原版 kFieldTypesCanBeClearedOnGC)
  pub const FIELD_TYPES_CAN_BE_CLEARED_ON_GC: bool = true;

  /// None 哨兵的 Smi 编号 — 以 Smi 编码区分于 Class(Map 指针)
  #[inline]
  #[must_use]
  pub const fn none_repr() -> u32 {
    2
  }

  /// Any 哨兵的 Smi 编号
  #[inline]
  #[must_use]
  pub const fn any_repr() -> u32 {
    1
  }
}

/// 语义别名: None 哨兵 (原版 FieldType::None, 实现为 Smi::FromInt(2))
pub const FIELD_TYPE_NONE: u32 = 2;
/// 语义别名: Any 哨兵 (原版 FieldType::Any, 实现为 Smi::FromInt(1))
pub const FIELD_TYPE_ANY: u32 = 1;

// —— 签名骨架 (原版公开接口, 供 Developer 就地补语义) ——
// Tagged<FieldType> FieldType::None();
// Tagged<FieldType> FieldType::Any();
// DirectHandle<FieldType> FieldType::None(Isolate*);
// DirectHandle<FieldType> FieldType::Any(Isolate*);
// Tagged<FieldType> FieldType::Class(Tagged<Map>);
// DirectHandle<FieldType> FieldType::Class(DirectHandle<Map>, Isolate*);
// bool FieldType::NowContains(Tagged<FieldType>, Tagged<Object>);
// Tagged<Map> FieldType::AsClass(Tagged<FieldType>);
// bool FieldType::NowStable(Tagged<FieldType>);
// bool FieldType::NowIs(Tagged<FieldType>, Tagged<FieldType>);
// bool FieldType::Equals(Tagged<FieldType>, Tagged<FieldType>);
// bool IsClass(Tagged<FieldType>);
// inline bool IsNone(Tagged<FieldType>) { obj == None }
// inline bool IsAny(Tagged<FieldType>) { obj == Any }

fn _signatures_placeholder(_: &Error) {}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FIELD_TYPE_NONE == 2);
  assert!(FIELD_TYPE_ANY == 1);
  assert!(FIELD_TYPE_NONE != FIELD_TYPE_ANY);
  assert!(FieldType::FIELD_TYPES_CAN_BE_CLEARED_ON_GC);
};
