// v8-objects-rest/src/templates.rs
// 对应原版 v8/src/objects/templates.h 与 templates-inl.h

use std::mem::{offset_of, size_of};

use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  smi::Smi,
  tagged::{Object, Tagged},
};

/// 模板基类 (TemplateInfo)
///
/// 存储模板通用标志（可缓存性、只读提升）与序列号 (serial_number)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TemplateInfo {
  /// 继承自 HeapObject (Map 8B)
  pub header: HeapObject,
  /// 标志与序列号组合位域 (Smi 编码, 8B)
  pub template_info_flags: Tagged<Smi>,
}

const _: () = assert!(size_of::<TemplateInfo>() == 16);
const _: () = assert!(offset_of!(TemplateInfo, template_info_flags) == 8);

impl TemplateInfo {
  pub const IS_CACHEABLE_MASK: u32 = 1 << 0;
  pub const SHOULD_PROMOTE_TO_READ_ONLY_MASK: u32 = 1 << 1;
  pub const SERIAL_NUMBER_SHIFT: u32 = 2;
  pub const SERIAL_NUMBER_MASK: u32 = 0x1fff_ffff << Self::SERIAL_NUMBER_SHIFT;

  /// 构造初始 TemplateInfo
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self {
      header: HeapObject::new(map),
      template_info_flags: Tagged::new(0),
    }
  }

  #[inline]
  fn raw_flags(&self) -> u32 {
    Smi::value(self.template_info_flags) as u32
  }

  #[inline]
  fn set_raw_flags(&mut self, flags: u32) {
    self.template_info_flags = Tagged::new(Smi::from_int(flags as i32).ptr());
  }

  /// 是否可缓存实例
  #[inline]
  pub fn is_cacheable(&self) -> bool {
    (self.raw_flags() & Self::IS_CACHEABLE_MASK) != 0
  }

  /// 设置是否可缓存实例
  #[inline]
  pub fn set_is_cacheable(&mut self, value: bool) {
    let mut flags = self.raw_flags();
    if value {
      flags |= Self::IS_CACHEABLE_MASK;
    } else {
      flags &= !Self::IS_CACHEABLE_MASK;
    }
    self.set_raw_flags(flags);
  }

  /// 是否应提升至只读堆
  #[inline]
  pub fn should_promote_to_read_only(&self) -> bool {
    (self.raw_flags() & Self::SHOULD_PROMOTE_TO_READ_ONLY_MASK) != 0
  }

  /// 设置是否提升至只读堆
  #[inline]
  pub fn set_should_promote_to_read_only(&mut self, value: bool) {
    let mut flags = self.raw_flags();
    if value {
      flags |= Self::SHOULD_PROMOTE_TO_READ_ONLY_MASK;
    } else {
      flags &= !Self::SHOULD_PROMOTE_TO_READ_ONLY_MASK;
    }
    self.set_raw_flags(flags);
  }

  /// 获取分配序列号
  #[inline]
  pub fn serial_number(&self) -> u32 {
    (self.raw_flags() & Self::SERIAL_NUMBER_MASK) >> Self::SERIAL_NUMBER_SHIFT
  }

  /// 设置分配序列号
  #[inline]
  pub fn set_serial_number(&mut self, num: u32) {
    let flags = (self.raw_flags() & !Self::SERIAL_NUMBER_MASK)
      | ((num << Self::SERIAL_NUMBER_SHIFT) & Self::SERIAL_NUMBER_MASK);
    self.set_raw_flags(flags);
  }
}

/// 带属性列表的模板基类 (TemplateInfoWithProperties)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TemplateInfoWithProperties {
  /// 继承自 TemplateInfo (16B)
  pub template_info: TemplateInfo,
  /// 属性数量 (Smi, 8B, offset 16)
  pub number_of_properties: Tagged<Smi>,
  /// 静态属性列表 (ArrayList 或 Undefined, 8B, offset 24)
  pub property_list: Tagged<Object>,
  /// 属性访问器列表 (ArrayList 或 Undefined, 8B, offset 32)
  pub property_accessors: Tagged<Object>,
}

const _: () = assert!(size_of::<TemplateInfoWithProperties>() == 40);
const _: () = assert!(offset_of!(TemplateInfoWithProperties, number_of_properties) == 16);
const _: () = assert!(offset_of!(TemplateInfoWithProperties, property_list) == 24);
const _: () = assert!(offset_of!(TemplateInfoWithProperties, property_accessors) == 32);

impl TemplateInfoWithProperties {
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self {
      template_info: TemplateInfo::new(map),
      number_of_properties: Tagged::new(0),
      property_list: Tagged::NULL,
      property_accessors: Tagged::NULL,
    }
  }

  #[inline]
  pub fn number_of_properties(&self) -> i32 {
    Smi::value(self.number_of_properties)
  }

  #[inline]
  pub fn set_number_of_properties(&mut self, count: i32) {
    self.number_of_properties = Tagged::new(Smi::from_int(count).ptr());
  }
}

/// 函数模板信息 (FunctionTemplateInfo)
///
/// 存储 C++ API 导出的原生函数模板信息、参数、签名校验与调用回调。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FunctionTemplateInfo {
  /// 继承自 TemplateInfoWithProperties (40B)
  pub template_info: TemplateInfoWithProperties,
  /// 类名 (String 或 Undefined, 8B, offset 40)
  pub class_name: Tagged<Object>,
  /// 接口名 (String 或 Undefined, 8B, offset 48)
  pub interface_name: Tagged<Object>,
  /// 函数签名 FunctionTemplateInfo (8B, offset 56)
  pub signature: Tagged<Object>,
  /// 罕用数据 (FunctionTemplateRareData, 8B, offset 64)
  pub rare_data: Tagged<Object>,
  /// 关联的 SharedFunctionInfo (8B, offset 72)
  pub shared_function_info: Tagged<Object>,
  /// 缓存属性名 (8B, offset 80)
  pub cached_property_name: Tagged<Object>,
  /// 回调关联数据 (8B, offset 88)
  pub callback_data: Tagged<Object>,
  /// 标志位域 (4B, offset 96)
  pub flag: u32,
  /// 形参长度 length (2B, offset 100)
  pub length: i16,
  /// 实例类型 (2B, offset 102)
  pub instance_type: u16,
  /// 异常上下文 (4B, offset 104)
  pub exception_context: u32,
  /// 64 位对齐填充 (4B, offset 108)
  pub optional_padding: u32,
  /// 回调 C 函数外部指针 (8B, offset 112)
  pub callback: usize,
}

const _: () = assert!(size_of::<FunctionTemplateInfo>() == 120);
const _: () = assert!(offset_of!(FunctionTemplateInfo, class_name) == 40);
const _: () = assert!(offset_of!(FunctionTemplateInfo, interface_name) == 48);
const _: () = assert!(offset_of!(FunctionTemplateInfo, signature) == 56);
const _: () = assert!(offset_of!(FunctionTemplateInfo, rare_data) == 64);
const _: () = assert!(offset_of!(FunctionTemplateInfo, shared_function_info) == 72);
const _: () = assert!(offset_of!(FunctionTemplateInfo, cached_property_name) == 80);
const _: () = assert!(offset_of!(FunctionTemplateInfo, callback_data) == 88);
const _: () = assert!(offset_of!(FunctionTemplateInfo, flag) == 96);
const _: () = assert!(offset_of!(FunctionTemplateInfo, length) == 100);
const _: () = assert!(offset_of!(FunctionTemplateInfo, instance_type) == 102);
const _: () = assert!(offset_of!(FunctionTemplateInfo, exception_context) == 104);
const _: () = assert!(offset_of!(FunctionTemplateInfo, optional_padding) == 108);
const _: () = assert!(offset_of!(FunctionTemplateInfo, callback) == 112);

impl FunctionTemplateInfo {
  // 标志位移
  pub const IS_OBJECT_TEMPLATE_CALL_HANDLER_SHIFT: u32 = 0;
  pub const HAS_SIDE_EFFECTS_SHIFT: u32 = 1;
  pub const UNDETECTABLE_SHIFT: u32 = 2;
  pub const NEEDS_ACCESS_CHECK_SHIFT: u32 = 3;
  pub const READ_ONLY_PROTOTYPE_SHIFT: u32 = 4;
  pub const REMOVE_PROTOTYPE_SHIFT: u32 = 5;
  pub const ACCEPT_ANY_RECEIVER_SHIFT: u32 = 6;
  pub const PUBLISHED_SHIFT: u32 = 7;
  pub const ALLOWED_RECEIVER_START_SHIFT: u32 = 8;
  pub const ALLOWED_RECEIVER_START_SIZE: u32 = 12;
  pub const ALLOWED_RECEIVER_END_SHIFT: u32 = 20;
  pub const ALLOWED_RECEIVER_END_SIZE: u32 = 12;

  /// 构造初始 FunctionTemplateInfo
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self {
      template_info: TemplateInfoWithProperties::new(map),
      class_name: Tagged::NULL,
      interface_name: Tagged::NULL,
      signature: Tagged::NULL,
      rare_data: Tagged::NULL,
      shared_function_info: Tagged::NULL,
      cached_property_name: Tagged::NULL,
      callback_data: Tagged::NULL,
      flag: 0,
      length: 0,
      instance_type: 0,
      exception_context: 0,
      optional_padding: 0,
      callback: 0,
    }
  }

  #[inline]
  pub const fn is_object_template_call_handler(&self) -> bool {
    (self.flag & (1 << Self::IS_OBJECT_TEMPLATE_CALL_HANDLER_SHIFT)) != 0
  }

  #[inline]
  pub fn set_is_object_template_call_handler(&mut self, val: bool) {
    if val {
      self.flag |= 1 << Self::IS_OBJECT_TEMPLATE_CALL_HANDLER_SHIFT;
    } else {
      self.flag &= !(1 << Self::IS_OBJECT_TEMPLATE_CALL_HANDLER_SHIFT);
    }
  }

  #[inline]
  pub const fn has_side_effects(&self) -> bool {
    (self.flag & (1 << Self::HAS_SIDE_EFFECTS_SHIFT)) != 0
  }

  #[inline]
  pub fn set_has_side_effects(&mut self, val: bool) {
    if val {
      self.flag |= 1 << Self::HAS_SIDE_EFFECTS_SHIFT;
    } else {
      self.flag &= !(1 << Self::HAS_SIDE_EFFECTS_SHIFT);
    }
  }

  #[inline]
  pub const fn undetectable(&self) -> bool {
    (self.flag & (1 << Self::UNDETECTABLE_SHIFT)) != 0
  }

  #[inline]
  pub fn set_undetectable(&mut self, val: bool) {
    if val {
      self.flag |= 1 << Self::UNDETECTABLE_SHIFT;
    } else {
      self.flag &= !(1 << Self::UNDETECTABLE_SHIFT);
    }
  }

  #[inline]
  pub const fn needs_access_check(&self) -> bool {
    (self.flag & (1 << Self::NEEDS_ACCESS_CHECK_SHIFT)) != 0
  }

  #[inline]
  pub fn set_needs_access_check(&mut self, val: bool) {
    if val {
      self.flag |= 1 << Self::NEEDS_ACCESS_CHECK_SHIFT;
    } else {
      self.flag &= !(1 << Self::NEEDS_ACCESS_CHECK_SHIFT);
    }
  }

  #[inline]
  pub const fn read_only_prototype(&self) -> bool {
    (self.flag & (1 << Self::READ_ONLY_PROTOTYPE_SHIFT)) != 0
  }

  #[inline]
  pub fn set_read_only_prototype(&mut self, val: bool) {
    if val {
      self.flag |= 1 << Self::READ_ONLY_PROTOTYPE_SHIFT;
    } else {
      self.flag &= !(1 << Self::READ_ONLY_PROTOTYPE_SHIFT);
    }
  }

  #[inline]
  pub const fn remove_prototype(&self) -> bool {
    (self.flag & (1 << Self::REMOVE_PROTOTYPE_SHIFT)) != 0
  }

  #[inline]
  pub fn set_remove_prototype(&mut self, val: bool) {
    if val {
      self.flag |= 1 << Self::REMOVE_PROTOTYPE_SHIFT;
    } else {
      self.flag &= !(1 << Self::REMOVE_PROTOTYPE_SHIFT);
    }
  }

  #[inline]
  pub const fn accept_any_receiver(&self) -> bool {
    (self.flag & (1 << Self::ACCEPT_ANY_RECEIVER_SHIFT)) != 0
  }

  #[inline]
  pub fn set_accept_any_receiver(&mut self, val: bool) {
    if val {
      self.flag |= 1 << Self::ACCEPT_ANY_RECEIVER_SHIFT;
    } else {
      self.flag &= !(1 << Self::ACCEPT_ANY_RECEIVER_SHIFT);
    }
  }

  #[inline]
  pub const fn published(&self) -> bool {
    (self.flag & (1 << Self::PUBLISHED_SHIFT)) != 0
  }

  #[inline]
  pub fn set_published(&mut self, val: bool) {
    if val {
      self.flag |= 1 << Self::PUBLISHED_SHIFT;
    } else {
      self.flag &= !(1 << Self::PUBLISHED_SHIFT);
    }
  }

  #[inline]
  pub const fn allowed_receiver_instance_type_range_start(&self) -> u16 {
    ((self.flag >> Self::ALLOWED_RECEIVER_START_SHIFT)
      & ((1 << Self::ALLOWED_RECEIVER_START_SIZE) - 1)) as u16
  }

  #[inline]
  pub fn set_allowed_receiver_instance_type_range_start(&mut self, start: u16) {
    let mask = ((1 << Self::ALLOWED_RECEIVER_START_SIZE) - 1) << Self::ALLOWED_RECEIVER_START_SHIFT;
    self.flag =
      (self.flag & !mask) | (((start as u32) << Self::ALLOWED_RECEIVER_START_SHIFT) & mask);
  }

  #[inline]
  pub const fn allowed_receiver_instance_type_range_end(&self) -> u16 {
    ((self.flag >> Self::ALLOWED_RECEIVER_END_SHIFT) & ((1 << Self::ALLOWED_RECEIVER_END_SIZE) - 1))
      as u16
  }

  #[inline]
  pub fn set_allowed_receiver_instance_type_range_end(&mut self, end: u16) {
    let mask = ((1 << Self::ALLOWED_RECEIVER_END_SIZE) - 1) << Self::ALLOWED_RECEIVER_END_SHIFT;
    self.flag = (self.flag & !mask) | (((end as u32) << Self::ALLOWED_RECEIVER_END_SHIFT) & mask);
  }
}

/// 对象模板信息 (ObjectTemplateInfo)
///
/// 存储 C++ API 导出的对象模板，包含构造函数引用与嵌入字段数配置。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ObjectTemplateInfo {
  /// 继承自 TemplateInfoWithProperties (40B)
  pub template_info: TemplateInfoWithProperties,
  /// 构造函数模板引用 (FunctionTemplateInfo 或 Undefined, 8B, offset 40)
  pub constructor: Tagged<Object>,
  /// 嵌入字段数量与标志 (SmiTagged, 8B, offset 48)
  pub data: Tagged<Smi>,
}

const _: () = assert!(size_of::<ObjectTemplateInfo>() == 56);
const _: () = assert!(offset_of!(ObjectTemplateInfo, constructor) == 40);
const _: () = assert!(offset_of!(ObjectTemplateInfo, data) == 48);

impl ObjectTemplateInfo {
  pub const IS_IMMUTABLE_PROTOTYPE_MASK: u32 = 1 << 0;
  pub const IS_CODE_KIND_MASK: u32 = 1 << 1;
  pub const EMBEDDER_FIELD_COUNT_SHIFT: u32 = 2;
  pub const EMBEDDER_FIELD_COUNT_MASK: u32 = 0x0fff_ffff << Self::EMBEDDER_FIELD_COUNT_SHIFT;

  #[inline]
  pub const fn new(map: Tagged<Map>, constructor: Tagged<Object>) -> Self {
    Self {
      template_info: TemplateInfoWithProperties::new(map),
      constructor,
      data: Tagged::new(0),
    }
  }

  #[inline]
  fn raw_data(&self) -> u32 {
    Smi::value(self.data) as u32
  }

  #[inline]
  fn set_raw_data(&mut self, data: u32) {
    self.data = Tagged::new(Smi::from_int(data as i32).ptr());
  }

  #[inline]
  pub fn is_immutable_prototype(&self) -> bool {
    (self.raw_data() & Self::IS_IMMUTABLE_PROTOTYPE_MASK) != 0
  }

  #[inline]
  pub fn set_immutable_prototype(&mut self, val: bool) {
    let mut d = self.raw_data();
    if val {
      d |= Self::IS_IMMUTABLE_PROTOTYPE_MASK;
    } else {
      d &= !Self::IS_IMMUTABLE_PROTOTYPE_MASK;
    }
    self.set_raw_data(d);
  }

  #[inline]
  pub fn is_code_kind(&self) -> bool {
    (self.raw_data() & Self::IS_CODE_KIND_MASK) != 0
  }

  #[inline]
  pub fn set_code_kind(&mut self, val: bool) {
    let mut d = self.raw_data();
    if val {
      d |= Self::IS_CODE_KIND_MASK;
    } else {
      d &= !Self::IS_CODE_KIND_MASK;
    }
    self.set_raw_data(d);
  }

  #[inline]
  pub fn embedder_field_count(&self) -> i32 {
    ((self.raw_data() & Self::EMBEDDER_FIELD_COUNT_MASK) >> Self::EMBEDDER_FIELD_COUNT_SHIFT) as i32
  }

  #[inline]
  pub fn set_embedder_field_count(&mut self, count: i32) {
    let d = (self.raw_data() & !Self::EMBEDDER_FIELD_COUNT_MASK)
      | (((count as u32) << Self::EMBEDDER_FIELD_COUNT_SHIFT) & Self::EMBEDDER_FIELD_COUNT_MASK);
    self.set_raw_data(d);
  }
}

/// 字典模板信息 (DictionaryTemplateInfo)
///
/// 存储通过 ObjectTemplate::NewDictionaryTemplate 创建的原型字典属性名。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DictionaryTemplateInfo {
  /// 继承自 TemplateInfo (16B)
  pub template_info: TemplateInfo,
  /// 属性名数组 (FixedArray, 8B, offset 16)
  pub property_names: Tagged<Object>,
}

const _: () = assert!(size_of::<DictionaryTemplateInfo>() == 24);
const _: () = assert!(offset_of!(DictionaryTemplateInfo, property_names) == 16);

impl DictionaryTemplateInfo {
  #[inline]
  pub const fn new(map: Tagged<Map>, property_names: Tagged<Object>) -> Self {
    Self {
      template_info: TemplateInfo::new(map),
      property_names,
    }
  }

  #[inline]
  pub const fn property_names(&self) -> Tagged<Object> {
    self.property_names
  }

  #[inline]
  pub fn set_property_names(&mut self, names: Tagged<Object>) {
    self.property_names = names;
  }
}

/// 函数模板罕用数据结构 (FunctionTemplateRareData)
///
/// 当 FunctionTemplateInfo 设置了稀有属性（如原型提供者、拦截器等）时按需分配。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FunctionTemplateRareData {
  /// 继承自 HeapObject (8B)
  pub header: HeapObject,
  /// 原型模板 (8B, offset 8)
  pub prototype_template: Tagged<Object>,
  /// 原型提供者模板 (8B, offset 16)
  pub prototype_provider_template: Tagged<Object>,
  /// 父级模板 (8B, offset 24)
  pub parent_template: Tagged<Object>,
  /// 命名属性拦截器 (8B, offset 32)
  pub named_property_handler: Tagged<Object>,
  /// 索引属性拦截器 (8B, offset 40)
  pub indexed_property_handler: Tagged<Object>,
  /// 实例模板 (8B, offset 48)
  pub instance_template: Tagged<Object>,
  /// 实例调用处理器 (8B, offset 56)
  pub instance_call_handler: Tagged<Object>,
  /// 访问检查信息 (8B, offset 64)
  pub access_check_info: Tagged<Object>,
  /// C 函数重载数组 (8B, offset 72)
  pub c_function_overloads: Tagged<Object>,
}

const _: () = assert!(size_of::<FunctionTemplateRareData>() == 80);
const _: () = assert!(offset_of!(FunctionTemplateRareData, prototype_template) == 8);
const _: () = assert!(offset_of!(FunctionTemplateRareData, prototype_provider_template) == 16);
const _: () = assert!(offset_of!(FunctionTemplateRareData, parent_template) == 24);
const _: () = assert!(offset_of!(FunctionTemplateRareData, named_property_handler) == 32);
const _: () = assert!(offset_of!(FunctionTemplateRareData, indexed_property_handler) == 40);
const _: () = assert!(offset_of!(FunctionTemplateRareData, instance_template) == 48);
const _: () = assert!(offset_of!(FunctionTemplateRareData, instance_call_handler) == 56);
const _: () = assert!(offset_of!(FunctionTemplateRareData, access_check_info) == 64);
const _: () = assert!(offset_of!(FunctionTemplateRareData, c_function_overloads) == 72);

impl FunctionTemplateRareData {
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self {
      header: HeapObject::new(map),
      prototype_template: Tagged::NULL,
      prototype_provider_template: Tagged::NULL,
      parent_template: Tagged::NULL,
      named_property_handler: Tagged::NULL,
      indexed_property_handler: Tagged::NULL,
      instance_template: Tagged::NULL,
      instance_call_handler: Tagged::NULL,
      access_check_info: Tagged::NULL,
      c_function_overloads: Tagged::NULL,
    }
  }
}
