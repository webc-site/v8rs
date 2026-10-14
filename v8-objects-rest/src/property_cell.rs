// v8-objects-rest/src/property_cell.rs
// 对应原版 v8/src/objects/property-cell.h 与 property-cell-inl.h

use std::mem::{offset_of, size_of};

pub use v8_objects_core::property_details::PropertyCellType;
use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  name::Name,
  property_details::PropertyDetails,
  smi::Smi,
  tagged::{Object, Tagged},
};

/// 全局属性单元格 (PropertyCell)
///
/// 用于存储全局对象 (GlobalObject/GlobalDictionary) 的属性条目与常量性追踪信息。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PropertyCell {
  /// 继承自 HeapObject
  pub header: HeapObject,
  /// 全局属性名
  pub name: Tagged<Name>,
  /// 原始属性细节 (以 Smi 编码，存储 PropertyDetails 及 PropertyCellType)
  pub property_details_raw: Tagged<Smi>,
  /// 属性当前值
  pub value: Tagged<Object>,
  /// 依赖该单元格类型的编译优化代码列表
  pub dependent_code: Tagged<Object>,
}

// 静态断言保证 64 位未压缩 tagged 布局精确匹配原版
const _: () = assert!(size_of::<PropertyCell>() == 40);
const _: () = assert!(offset_of!(PropertyCell, name) == 8);
const _: () = assert!(offset_of!(PropertyCell, property_details_raw) == 16);
const _: () = assert!(offset_of!(PropertyCell, value) == 24);
const _: () = assert!(offset_of!(PropertyCell, dependent_code) == 32);

impl PropertyCell {
  /// 构造新的 PropertyCell
  #[inline]
  pub fn new(
    map: Tagged<Map>,
    name: Tagged<Name>,
    details: PropertyDetails,
    value: Tagged<Object>,
    dependent_code: Tagged<Object>,
  ) -> Self {
    Self {
      header: HeapObject::new(map),
      name,
      property_details_raw: Tagged::new(details.as_smi().ptr()),
      value,
      dependent_code,
    }
  }

  /// 获取属性名称
  #[inline]
  pub const fn name(&self) -> Tagged<Name> {
    self.name
  }

  /// 设置属性名称
  #[inline]
  pub fn set_name(&mut self, name: Tagged<Name>) {
    self.name = name;
  }

  /// 获取原始属性细节 Smi 槽
  #[inline]
  pub const fn property_details_raw(&self) -> Tagged<Smi> {
    self.property_details_raw
  }

  /// 设置原始属性细节 Smi 槽
  #[inline]
  pub fn set_property_details_raw(&mut self, raw: Tagged<Smi>) {
    self.property_details_raw = raw;
  }

  /// 解析 PropertyDetails
  #[inline]
  pub const fn property_details(&self) -> PropertyDetails {
    PropertyDetails::from_smi(self.property_details_raw)
  }

  /// 设置 PropertyDetails (编码为 Smi)
  #[inline]
  pub fn set_property_details(&mut self, details: PropertyDetails) {
    self.property_details_raw = Tagged::new(details.as_smi().ptr());
  }

  /// 获取当前单元格类别 (PropertyCellType)
  #[inline]
  pub const fn cell_type(&self) -> PropertyCellType {
    self.property_details().cell_type()
  }

  /// 获取单元格存储的值
  #[inline]
  pub const fn value(&self) -> Tagged<Object> {
    self.value
  }

  /// 设置单元格存储的值
  #[inline]
  pub fn set_value(&mut self, value: Tagged<Object>) {
    self.value = value;
  }

  /// 获取关联的依赖代码列表
  #[inline]
  pub const fn dependent_code(&self) -> Tagged<Object> {
    self.dependent_code
  }

  /// 设置关联的依赖代码列表
  #[inline]
  pub fn set_dependent_code(&mut self, dependent_code: Tagged<Object>) {
    self.dependent_code = dependent_code;
  }

  /// 属性状态迁移 (原版 PropertyCell::Transition, property-cell-inl.h:91-102)
  ///
  /// 先写入 InTransition 中间状态，随后更新值，最后写入新的最终属性细节。
  #[inline]
  pub fn transition(&mut self, new_details: PropertyDetails, new_value: Tagged<Object>) {
    let transition_marker = new_details.set_cell_type(PropertyCellType::InTransition);
    self.set_property_details(transition_marker);
    self.set_value(new_value);
    self.set_property_details(new_details);
  }

  /// 检查属性细节与值是否兼容
  #[inline]
  #[must_use]
  pub fn check_data_is_compatible(details: PropertyDetails, value: Tagged<Object>) -> bool {
    match details.cell_type() {
      PropertyCellType::Undefined => value.is_null(),
      PropertyCellType::Constant | PropertyCellType::ConstantType | PropertyCellType::Mutable => {
        true
      }
      PropertyCellType::InTransition => false,
    }
  }

  /// 判断更新值后是否仍保持同一常量类型 (对应 C++ RemainsConstantType)
  #[inline]
  #[must_use]
  pub fn remains_constant_type(&self, new_value: Tagged<Object>) -> bool {
    if self.value == new_value {
      return true;
    }
    if self.value.is_smi() && new_value.is_smi() {
      return true;
    }
    false
  }

  /// 判断是否允许迁移到指定细节与新值 (原版 CanTransitionTo)
  #[inline]
  #[must_use]
  pub fn can_transition_to(&self, new_details: PropertyDetails, new_value: Tagged<Object>) -> bool {
    if !Self::check_data_is_compatible(new_details, new_value) {
      return false;
    }
    let current_type = self.cell_type();
    let new_type = new_details.cell_type();
    match (current_type, new_type) {
      (PropertyCellType::InTransition, _) | (_, PropertyCellType::InTransition) => false,
      (PropertyCellType::Undefined, PropertyCellType::Undefined) => false,
      (PropertyCellType::Undefined, _) => true,
      (PropertyCellType::Constant, PropertyCellType::Undefined) => false,
      (PropertyCellType::Constant, PropertyCellType::Constant) => self.value == new_value,
      (PropertyCellType::Constant, PropertyCellType::ConstantType | PropertyCellType::Mutable) => {
        true
      }
      (
        PropertyCellType::ConstantType,
        PropertyCellType::ConstantType | PropertyCellType::Mutable,
      ) => true,
      (PropertyCellType::Mutable, PropertyCellType::Mutable) => true,
      _ => false,
    }
  }

  /// 计算初次赋值时的单元格类别 (原版 InitialType)
  #[inline]
  #[must_use]
  pub fn initial_type(value: Tagged<Object>) -> PropertyCellType {
    if value.is_null() {
      PropertyCellType::Undefined
    } else {
      PropertyCellType::Constant
    }
  }

  /// 计算更新值后的预期单元格类别 (原版 UpdatedType)
  #[inline]
  #[must_use]
  pub fn updated_type(
    &self,
    new_value: Tagged<Object>,
    details: PropertyDetails,
  ) -> PropertyCellType {
    match details.cell_type() {
      PropertyCellType::Undefined => PropertyCellType::Constant,
      PropertyCellType::Constant => {
        if self.value == new_value {
          PropertyCellType::Constant
        } else if self.remains_constant_type(new_value) {
          PropertyCellType::ConstantType
        } else {
          PropertyCellType::Mutable
        }
      }
      PropertyCellType::ConstantType => {
        if self.remains_constant_type(new_value) {
          PropertyCellType::ConstantType
        } else {
          PropertyCellType::Mutable
        }
      }
      PropertyCellType::Mutable | PropertyCellType::InTransition => PropertyCellType::Mutable,
    }
  }
}
