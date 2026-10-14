#!/usr/bin/env bun
// tool/src/objects/property.h.js — 覆盖 property.h 同名族:
//   v8/src/objects/property.h + property.cc (Descriptor 构造/工厂, cc:59-121)
//   → v8-objects-core/src/property.rs
//
// 映射登记 (C++ → Rust):
//   class Descriptor final → struct Descriptor { key: Tagged<Name>,
//     value: Tagged<Object> (MaybeObject 弱引用位由 tag 位承载), details }
//   GetKey/GetValue/GetDetails/GetSortedKeyIndex/SetSortedKeyIndex → 关联函数
//   Descriptor::Descriptor() 空构造 (details = Smi::zero) → Descriptor::new
//   Descriptor(key, value, details) / (key, value, kind..in_object) → with_details / with_parts
//   DataField(无 isolate 重载, property.cc:93-102) → data_field
//   AccessorConstant (property.cc:114-121) → accessor_constant
//
// 范围裁剪登记 (推迟):
//   - DataField(isolate, ...) (property.cc:83-91): FieldType::Any(isolate) 需 isolate/
//     FieldType 对象族 → 归 v8-objects-maps (DescriptorArray 落地时承接)
//   - DataConstant (property.cc:104-112): Object::OptimalRepresentation 属 objects 家族
//     (他人文件), 落地后承接
//   - operator<< 打印族 (property.cc:18-57): 归 L4 v8-diagnostics
//   - IsUniqueName / IsAnyPrivate 的 DCHECK 前置: 依赖 String/Symbol 运行时判定,
//     调用方保证 (debug_assert 语义以注释登记)
import { banner, readV8, writeSrc } from "../../lib.js";

const src = readV8("src/objects/property.h");
const cc = readV8("src/objects/property.cc");

const mustContain = (s, sub, file) => {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${file} 无 "${sub.slice(0, 48)}..."`);
};
mustContain(src, "class V8_EXPORT_PRIVATE Descriptor final {", "property.h");
mustContain(src, "static Descriptor DataField(", "property.h");
mustContain(src, "static Descriptor AccessorConstant(", "property.h");
mustContain(cc, "Descriptor::Descriptor() : details_(Smi::zero()) {}", "property.cc");
mustContain(cc, "PropertyKind::kAccessor, attributes,", "property.cc");
mustContain(cc, "DCHECK(IsSmi(*wrapped_field_type) || IsWeak(*wrapped_field_type));", "property.cc");

const body = `${banner("src/objects/property.h.js", "src/objects/property.h + property.cc(Descriptor 构造)", "isolate 依赖工厂与 DataConstant/OptimalRepresentation 推迟, 登记见脚本头")}
//! Descriptor: 实例描述符数组元素的抽象 (键 + 值 + 属性详情)。

use crate::name::Name;
use crate::property_details::{
  PropertyAttributes, PropertyConstness, PropertyDetails, PropertyKind, PropertyLocation,
  Representation,
};
use crate::smi::Smi;
use crate::tagged::{Object, Tagged};

/// 实例描述符数组元素 (对应原版 class Descriptor)。
///
/// key 须为 UniqueName (内部化字符串或 Symbol); value 可为弱引用
/// (字段类型包装), 弱位由 Tagged 指针 tag 位承载。
#[derive(Debug, Clone, Copy)]
pub struct Descriptor {
  key: Tagged<Name>,
  value: Tagged<Object>,
  details: PropertyDetails,
}

impl Default for Descriptor {
  /// 原版 Descriptor() 空构造 (property.cc:59): details = Smi::zero()。
  fn default() -> Self {
    Self {
      key: Tagged::NULL,
      value: Tagged::NULL,
      details: PropertyDetails::from_smi(Smi::from_int(0)),
    }
  }
}

impl Descriptor {
  /// 空描述符 (原版 Descriptor())。
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// 以显式详情构造 (原版 Descriptor(key, value, details); key 须 UniqueName,
  /// 私有名须不可枚举 — 前置条件由调用方保证, 原版 DCHECK)。
  #[inline]
  #[must_use]
  pub const fn with_details(key: Tagged<Name>, value: Tagged<Object>, details: PropertyDetails) -> Self {
    Self { key, value, details }
  }

  /// 键 (原版 GetKey)。
  #[inline]
  #[must_use]
  pub const fn key(&self) -> Tagged<Name> {
    self.key
  }

  /// 值 (原版 GetValue; MaybeObject 语义)。
  #[inline]
  #[must_use]
  pub const fn value(&self) -> Tagged<Object> {
    self.value
  }

  /// 详情 (原版 GetDetails)。
  #[inline]
  #[must_use]
  pub const fn details(&self) -> PropertyDetails {
    self.details
  }

  /// 排序键索引 (原版 GetSortedKeyIndex)。
  #[inline]
  #[must_use]
  pub const fn sorted_key_index(&self) -> u32 {
    self.details.pointer()
  }

  /// 设置排序键索引 (原版 SetSortedKeyIndex)。
  #[inline]
  pub fn set_sorted_key_index(&mut self, index: u32) {
    self.details = self.details.set_pointer(index);
  }

  /// 数据字段描述符 (原版无 isolate 的 DataField 重载, property.cc:93-102;
  /// wrapped_field_type 须为 Smi 或弱引用, 原版 DCHECK 由调用方保证;
  /// field_offset 单位字节)。
  #[inline]
  #[must_use]
  pub const fn data_field(
    key: Tagged<Name>,
    field_offset: u32,
    attributes: PropertyAttributes,
    constness: PropertyConstness,
    representation: Representation,
    wrapped_field_type: Tagged<Object>,
    in_object: bool,
  ) -> Self {
    Self::with_details(
      key,
      wrapped_field_type,
      PropertyDetails::for_fast_field(
        PropertyKind::Data,
        attributes,
        PropertyLocation::Field,
        constness,
        representation,
        field_offset,
        in_object,
      ),
    )
  }

  /// 访问器常量描述符 (原版 AccessorConstant, property.cc:114-121)。
  #[inline]
  #[must_use]
  pub const fn accessor_constant(
    key: Tagged<Name>,
    foreign: Tagged<Object>,
    attributes: PropertyAttributes,
  ) -> Self {
    Self::with_details(
      key,
      foreign,
      PropertyDetails::for_fast_field(
        PropertyKind::Accessor,
        attributes,
        PropertyLocation::Descriptor,
        PropertyConstness::Const,
        Representation::tagged(),
        0,
        false,
      ),
    )
  }
}
`;

writeSrc("v8-objects-core", "property.rs", body);
console.log("property.rs: Descriptor + 默认构造/分项构造/data_field/accessor_constant; isolate 工厂推迟已登记");
