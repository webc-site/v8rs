// v8-objects-js-core/src/field_index.rs
// 对应原版 v8/src/objects/field-index.h + field-index-inl.h

//! FieldIndex: 字段索引抽象，用于在 JSObject 快属性访问中统一寻址对象内属性与对象外 PropertyArray。

use v8_common::K_TAGGED_SIZE;
use v8_objects_core::{
  map::Map,
  property_details::{PropertyDetails, Representation},
  tagged::Tagged,
};
use v8_objects_maps::{DescriptorArray, DescriptorArrayExt, PropertyArray};

use crate::{error::Result, js_object::K_HEADER_SIZE as K_JS_OBJECT_HEADER_SIZE};

/// 字段编码格式（对应原版 FieldIndex::Encoding）
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FieldEncoding {
  Tagged,
  Double,
  Word32,
}

/// 字段索引抽象，统一寻址对象内属性与外部 PropertyArray 槽位
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldIndex {
  is_inobject: bool,
  offset: usize,
  encoding: FieldEncoding,
  inobject_property_count: usize,
  first_field_offset_in_storage: usize,
}

impl FieldIndex {
  /// 创建新的字段索引
  #[inline]
  pub const fn new(
    is_inobject: bool,
    offset: usize,
    encoding: FieldEncoding,
    inobject_property_count: usize,
    first_field_offset_in_storage: usize,
  ) -> Self {
    Self {
      is_inobject,
      offset,
      encoding,
      inobject_property_count,
      first_field_offset_in_storage,
    }
  }

  /// 是否为对象内属性
  #[inline]
  pub const fn is_inobject(&self) -> bool {
    self.is_inobject
  }

  /// 在宿主对象（JSObject 或 PropertyArray）中的字节偏移量
  #[inline]
  pub const fn offset(&self) -> usize {
    self.offset
  }

  /// 字段编码格式
  #[inline]
  pub const fn encoding(&self) -> FieldEncoding {
    self.encoding
  }

  /// 是否为 double 字段
  #[inline]
  pub const fn is_double(&self) -> bool {
    matches!(self.encoding, FieldEncoding::Double)
  }

  /// 在宿主对象中的 slot 槽位序号
  #[inline]
  pub const fn offset_in_words(&self) -> usize {
    self.offset / PropertyArray::K_SLOT_SIZE
  }

  /// 外部 PropertyArray 槽位下标（仅当 !is_inobject 时有效）
  #[inline]
  pub fn outobject_array_index(&self) -> usize {
    debug_assert!(!self.is_inobject);
    PropertyArray::offset_in_words_to_index(self.offset_in_words())
  }

  /// 属性索引值（以第一个对象内属性为基准的整体逻辑下标）
  #[inline]
  pub fn property_index(&self) -> usize {
    let index = (self
      .offset
      .saturating_sub(self.first_field_offset_in_storage))
      / PropertyArray::K_SLOT_SIZE;
    if self.is_inobject {
      index
    } else {
      index + self.inobject_property_count
    }
  }

  /// 对应原版 FieldIndex::GetLoadByFieldIndex，计算 IC 指令 LoadFieldByIndex 的编码值
  #[inline]
  pub fn get_load_by_field_index(&self) -> i32 {
    let mut result = self.offset_in_words() as i32;
    if self.is_inobject() {
      result -= (K_JS_OBJECT_HEADER_SIZE / PropertyArray::K_SLOT_SIZE) as i32;
    } else {
      result -= (PropertyArray::K_HEADER_SIZE / PropertyArray::K_SLOT_SIZE) as i32;
      result = -result - 1;
    }
    result = (result as u32).wrapping_shl(1) as i32;
    if self.is_double() { result | 1 } else { result }
  }

  /// 从属性逻辑下标构建 FieldIndex（对应原版 FieldIndex::ForPropertyIndex）
  pub fn for_property_index(
    map: Tagged<Map>,
    property_index: usize,
    representation: Representation,
  ) -> Self {
    // SAFETY: map 必须指向有效的 Map 堆对象结构
    let map_ref = unsafe { map.as_ref() };
    let inobject_count = map_ref.get_in_object_properties().max(0) as usize;
    let is_inobject = property_index < inobject_count;
    let encoding = if representation.is_double() {
      FieldEncoding::Double
    } else {
      FieldEncoding::Tagged
    };
    let slot_size = PropertyArray::K_SLOT_SIZE;
    if is_inobject {
      let mut start_slots = map_ref.get_in_object_properties_start_in_words() as usize;
      if start_slots == 0 {
        start_slots = K_JS_OBJECT_HEADER_SIZE / slot_size;
      }
      let first_offset = start_slots * slot_size;
      let offset = (start_slots + property_index) * slot_size;
      Self::new(true, offset, encoding, inobject_count, first_offset)
    } else {
      let first_offset = PropertyArray::K_HEADER_SIZE;
      let out_index = property_index - inobject_count;
      let offset = PropertyArray::offset_of_element_at(out_index);
      Self::new(false, offset, encoding, inobject_count, first_offset)
    }
  }

  /// 从 PropertyDetails 构建 FieldIndex（对应原版 FieldIndex::ForDetails）
  pub fn for_details(map: Tagged<Map>, details: PropertyDetails) -> Self {
    let is_inobject = details.is_in_object();
    // SAFETY: map 必须指向有效的 Map 堆对象结构
    let map_ref = unsafe { map.as_ref() };
    let inobject_count = map_ref.get_in_object_properties().max(0) as usize;
    let offset = details.field_offset() as usize * K_TAGGED_SIZE;
    let encoding = if details.representation().is_double() {
      FieldEncoding::Double
    } else {
      FieldEncoding::Tagged
    };
    let first_offset = if is_inobject {
      let slot_size = PropertyArray::K_SLOT_SIZE;
      let mut start_slots = map_ref.get_in_object_properties_start_in_words() as usize;
      if start_slots == 0 {
        start_slots = K_JS_OBJECT_HEADER_SIZE / slot_size;
      }
      start_slots * slot_size
    } else {
      PropertyArray::K_HEADER_SIZE
    };
    Self::new(is_inobject, offset, encoding, inobject_count, first_offset)
  }

  /// 从描述符数组下标构建 FieldIndex（对应原版 FieldIndex::ForDescriptor）
  pub fn for_descriptor(map: Tagged<Map>, descriptor_index: usize) -> Result<Self> {
    // SAFETY: map 必须指向有效的 Map 堆对象结构
    let map_ref = unsafe { map.as_ref() };
    let desc_tagged: Tagged<DescriptorArray> = Tagged::new(map_ref.instance_descriptors.ptr());
    let details = desc_tagged.get_details(descriptor_index)?;
    Ok(Self::for_details(map, details))
  }

  /// 直接构建对象内偏移的 FieldIndex
  #[inline]
  pub const fn for_in_object_offset(offset: usize, encoding: FieldEncoding) -> Self {
    Self::new(true, offset, encoding, 0, 0)
  }

  /// 直接构建对象外下标的 FieldIndex
  #[inline]
  pub const fn for_out_object_index(index: usize, encoding: FieldEncoding) -> Self {
    let offset = PropertyArray::offset_of_element_at(index);
    Self::new(false, offset, encoding, 0, PropertyArray::K_HEADER_SIZE)
  }
}
