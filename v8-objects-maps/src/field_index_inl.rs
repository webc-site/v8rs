// 由 tool/src/objects/field-index-inl.h.js 从 v8/src/objects/field-index-inl.h 生成; 修订就地改, 复跑覆盖
// FieldIndex 工厂入口 + GetLoadByFieldIndex 编码规则 (语义面就地实现)
//! FieldIndex inline 面: 由 Map/PropertyDetails 构造字段索引。
//! (与 field_index.rs 的位段定义互补; 位段/编解码见 crate::field_index)

use v8_objects_core::{
  map::Map,
  property_details::{Kind, PropertyDetails, Representation},
  tagged::Tagged,
};

use crate::{
  error,
  field_index::{
    Encoding, FieldIndex, JS_OBJECT_HEADER_SIZE_IN_WORDS, PROPERTY_ARRAY_DATA_START_BIT_COUNT,
    PROPERTY_ARRAY_DATA_START_IN_WORDS,
  },
};

/// 原版 FieldIndex::ForInObjectOffset — offset 直取, encoding 显式给 (kWord32/kTagged/kDouble 对齐断言 ×3)。
///
/// v8: field-index-inl.h:21 — return FieldIndex(true, offset, encoding, 0, 0)
#[inline]
#[must_use]
pub fn for_in_object_offset(offset: i32, encoding: Encoding) -> FieldIndex {
  // DCHECK_IMPLIES 对齐检查 (压缩配置 kTaggedSize=4, kInt32Size=4, kDoubleSize=8)。
  match encoding {
    Encoding::Word32 | Encoding::Tagged => debug_assert!(offset.unsigned_abs().is_multiple_of(4)),
    Encoding::Double => debug_assert!(offset.unsigned_abs().is_multiple_of(8)),
  }
  FieldIndex::new(true, offset, encoding, 0, 0)
}

/// 原版 FieldIndex::ForPropertyIndex — inobject 首选, 越界转 PropertyArray (2 条 static_assert 护数据起点位宽)。
///
/// v8: field-index-inl.h:57
pub fn for_property_index(
  map: Tagged<Map>,
  property_index: i32,
  representation: Representation,
) -> FieldIndex {
  // SAFETY: map 指针有效, 指向已分配的 Map 堆对象。
  let m = unsafe { &*map.as_ptr() };
  let inobject_count = m.get_in_object_properties();
  let is_inobject = property_index < inobject_count;
  let (first_field_offset_in_storage, offset) = if is_inobject {
    (
      m.get_in_object_property_offset(0),
      m.get_in_object_property_offset(property_index),
    )
  } else {
    // 越界属性存于 PropertyArray: 数据起点 (kFirstOutOfObjectOffsetInWords 词) + 相对 outobject 下标。
    let out_index = property_index - inobject_count;
    let data_start_bytes = PROPERTY_ARRAY_DATA_START_IN_WORDS * K_TAGGED_SIZE_I32;
    (
      data_start_bytes,
      data_start_bytes + out_index * K_TAGGED_SIZE_I32,
    )
  };
  FieldIndex::new(
    is_inobject,
    offset,
    field_encoding(representation),
    inobject_count,
    first_field_offset_in_storage,
  )
}

/// 原版 FieldIndex::ForDescriptor — 读 instance_descriptors(kAcquireLoad) 的 details 后走 ForDetails。
///
/// v8: field-index-inl.h:92
pub fn for_descriptor(map: Tagged<Map>, descriptor_index: usize) -> error::Result<FieldIndex> {
  use crate::descriptor_array::{DescriptorArray, DescriptorArrayExt};
  // SAFETY: map.instance_descriptors 槽位有效 (DescriptorArray)。
  let descriptors: Tagged<DescriptorArray> = unsafe { (*map.as_ptr()).instance_descriptors.cast() };
  let details: PropertyDetails = descriptors.get_details(descriptor_index)?;
  Ok(for_details(map, details))
}

/// 原版 FieldIndex::ForDetails — details.is_in_object() + field_offset()*kTaggedSize + FieldEncoding(representation)。
///
/// v8: field-index-inl.h:100
pub fn for_details(map: Tagged<Map>, details: PropertyDetails) -> FieldIndex {
  let is_inobject = details.is_in_object();
  let offset = details.field_offset() as i32 * K_TAGGED_SIZE_I32;
  let encoding = field_encoding(details.representation());
  // SAFETY: map 指针有效。
  let m = unsafe { &*map.as_ptr() };
  let inobject_count = m.get_in_object_properties();
  let first_field_offset_in_storage = if is_inobject {
    m.get_in_object_property_offset(0)
  } else {
    PROPERTY_ARRAY_DATA_START_IN_WORDS * K_TAGGED_SIZE_I32
  };
  FieldIndex::new(
    is_inobject,
    offset,
    encoding,
    inobject_count,
    first_field_offset_in_storage,
  )
}

/// 字段偏移位掩码 (bits 6..16, 共 11 位)
const STORAGE_OFFSET_MASK: u32 = (1 << 11) - 1;

/// 原版 FieldIndex::ForSmiLoadHandler — LoadHandler 位解码 (IsInobject/StorageOffsetInWords/IsDouble)。
///
/// v8: field-index-inl.h:27-43
pub fn for_smi_load_handler(map: Tagged<Map>, handler: i32) -> FieldIndex {
  let handler_u32 = handler as u32;
  // LoadHandler::KindBits: bits 0..3, kField = 5
  let kind = handler_u32 & 0b1111;
  debug_assert_eq!(kind, 5, "handler kind must be kField");

  // LoadHandler::StorageOffsetInWordsBits: bits 6..16 (11 bits)
  let storage_offset_in_words = ((handler_u32 >> 6) & STORAGE_OFFSET_MASK) as i32;
  // LoadHandler::IsInobjectBits: bit 17 (1 bit)
  let is_inobject = ((handler_u32 >> 17) & 1) != 0;
  // LoadHandler::IsDoubleBits: bit 18 (1 bit)
  let is_double = ((handler_u32 >> 18) & 1) != 0;

  // SAFETY: map 指针有效
  let m = unsafe { &*map.as_ptr() };
  let inobject_count = m.get_in_object_properties();
  let first_field_offset_in_storage = if is_inobject {
    m.get_in_object_property_offset(0)
  } else {
    PROPERTY_ARRAY_DATA_START_IN_WORDS * K_TAGGED_SIZE_I32
  };
  let encoding = if is_double {
    Encoding::Double
  } else {
    Encoding::Tagged
  };

  FieldIndex::new(
    is_inobject,
    storage_offset_in_words * K_TAGGED_SIZE_I32,
    encoding,
    inobject_count,
    first_field_offset_in_storage,
  )
}

/// 原版 FieldIndex::FieldEncoding — Representation::Kind → Encoding。
#[inline]
#[must_use]
pub const fn field_encoding(representation: Representation) -> Encoding {
  match representation.kind() {
    Kind::Double => Encoding::Double,
    Kind::None | Kind::Smi | Kind::HeapObject | Kind::Tagged | Kind::WasmValue => Encoding::Tagged,
  }
}

/// 原版 GetLoadByFieldIndex: inobject 自 JSObject::kHeaderSize 起零基;
/// out-of-object 自 PropertyArray 数据起点起零基且取 -raw-1 消零歧义;
/// 整体 <<1, 最低位 = mutable double box 标志。
#[inline]
#[must_use]
pub const fn get_load_by_field_index(
  offset_in_words: i32,
  is_inobject: bool,
  is_double: bool,
) -> i32 {
  let mut result = offset_in_words;
  if is_inobject {
    result -= JS_OBJECT_HEADER_SIZE_IN_WORDS;
  } else {
    result -= PROPERTY_ARRAY_DATA_START_IN_WORDS;
    result = -result - 1;
  }
  let result = ((result as u32) << 1) as i32;
  if is_double { result | 1 } else { result }
}

/// 方法版 (对齐原版 FieldIndex::GetLoadByFieldIndex() const)。
impl FieldIndex {
  #[inline]
  #[must_use]
  pub const fn get_load_by_field_index_self(&self) -> i32 {
    get_load_by_field_index(self.offset_in_words(), self.is_inobject(), self.is_double())
  }
}

// ── 常量别名 ──
const K_TAGGED_SIZE_I32: i32 = v8_common::K_TAGGED_SIZE as i32;

// —— 提取不变式断言 ——
const _: () = {
  // 原版 field-index-inl.h:53-58 护数据起点位宽断言
  assert!(
    (PROPERTY_ARRAY_DATA_START_IN_WORDS * K_TAGGED_SIZE_I32)
      < (1 << PROPERTY_ARRAY_DATA_START_BIT_COUNT)
  );
  assert!(
    (PROPERTY_ARRAY_DATA_START_IN_WORDS * K_TAGGED_SIZE_I32)
      >= (1 << (PROPERTY_ARRAY_DATA_START_BIT_COUNT - 1))
  );

  // -raw-1 消零歧义: out-of-object 第 0 槽编码为 -1 (<<1 前为奇负)
  assert!(get_load_by_field_index(PROPERTY_ARRAY_DATA_START_IN_WORDS, false, false) == -2);
  assert!(get_load_by_field_index(PROPERTY_ARRAY_DATA_START_IN_WORDS + 1, false, false) == -4);
  // inobject: 相对 JSObject 头的字槽数, 正数
  assert!(get_load_by_field_index(JS_OBJECT_HEADER_SIZE_IN_WORDS, true, false) == 0);
  // double 标志占最低位
  assert!(get_load_by_field_index(JS_OBJECT_HEADER_SIZE_IN_WORDS, true, true) == 1);
};
