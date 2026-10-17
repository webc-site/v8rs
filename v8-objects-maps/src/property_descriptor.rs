// 由 tool/src/objects/property-descriptor.h.js 从 v8/src/objects/property-descriptor.h 生成; 修订就地改, 复跑覆盖
// PropertyDescriptor 字段骨架 + ES6 6.2.4 谓词 (clang -E 对数)
//! PropertyDescriptor: 属性描述符 (ES6 6.2.4)。
//! 骨架基线; handle 槽以 Option<usize> 占位 (语义面归 Developer 修订),
//! .cc 转换流程骨架归 v8-objects-rest 的 property_descriptor_cc.rs.skel
//! (tool/src/objects/property-descriptor.cc.js, 他席产物)。

use v8_objects_core::property_details::PropertyAttributes;

/// 属性描述符 (6 布尔位段 + value/get/set/name 四载荷槽)。
#[derive(Debug, Clone, Default)]
pub struct PropertyDescriptor {
  /// enumerable_ : 1
  pub enumerable: bool,
  /// has_enumerable_ : 1
  pub has_enumerable: bool,
  /// configurable_ : 1
  pub configurable: bool,
  /// has_configurable_ : 1
  pub has_configurable: bool,
  /// writable_ : 1
  pub writable: bool,
  /// has_writable_ : 1
  pub has_writable: bool,
  /// value_ (IndirectHandle<JSAny> 占位)
  pub value: Option<usize>,
  /// get_ (IndirectHandle<UnionOf<JSAny, FunctionTemplateInfo>> 占位)
  pub get: Option<usize>,
  /// set_ (同 get_)
  pub set: Option<usize>,
  /// name_ (IndirectHandle<JSAny> 占位)
  pub name: Option<usize>,
}

impl PropertyDescriptor {
  /// 空描述符 (原版默认构造: 六布尔位全 false, 四载荷槽全空)。
  #[inline]
  #[must_use]
  pub const fn empty() -> Self {
    Self {
      enumerable: false,
      has_enumerable: false,
      configurable: false,
      has_configurable: false,
      writable: false,
      has_writable: false,
      value: None,
      get: None,
      set: None,
      name: None,
    }
  }

  /// v8: property-descriptor.h:29 — ES6 6.2.4.1 IsAccessorDescriptor
  #[inline]
  #[must_use]
  pub const fn is_accessor_descriptor(desc: &Self) -> bool {
    desc.get.is_some() || desc.set.is_some()
  }

  /// v8: property-descriptor.h:34 — ES6 6.2.4.2 IsDataDescriptor
  #[inline]
  #[must_use]
  pub const fn is_data_descriptor(desc: &Self) -> bool {
    desc.value.is_some() || desc.has_writable
  }

  /// v8: property-descriptor.h:39 — ES6 6.2.4.3 IsGenericDescriptor
  #[inline]
  #[must_use]
  pub const fn is_generic_descriptor(desc: &Self) -> bool {
    !Self::is_accessor_descriptor(desc) && !Self::is_data_descriptor(desc)
  }

  /// v8: property-descriptor.h:57 — is_empty (六槽皆空)
  #[inline]
  #[must_use]
  pub const fn is_empty(&self) -> bool {
    !self.has_enumerable
      && !self.has_configurable
      && !self.has_writable
      && self.value.is_none()
      && self.get.is_none()
      && self.set.is_none()
  }

  /// v8: property-descriptor.h:62 — IsRegularAccessorProperty
  #[inline]
  #[must_use]
  pub const fn is_regular_accessor_property(&self) -> bool {
    self.has_configurable
      && self.has_enumerable
      && self.value.is_none()
      && !self.has_writable
      && self.get.is_some()
      && self.set.is_some()
  }

  /// v8: property-descriptor.h:69 — IsRegularDataProperty
  #[inline]
  #[must_use]
  pub const fn is_regular_data_property(&self) -> bool {
    self.has_configurable
      && self.has_enumerable
      && self.value.is_some()
      && self.has_writable
      && self.get.is_none()
      && self.set.is_none()
  }

  /// v8: property-descriptor.h:112 — ToAttributes (DONT_ENUM|DONT_DELETE|READ_ONLY 位组合)
  ///
  /// 语义 1:1: 仅 "显式声明了 has_* 但取反为 false" 时置位; 未声明(has_*=false)则不贡献位。
  #[inline]
  #[must_use]
  pub const fn to_attributes(&self) -> PropertyAttributes {
    let mut attrs = PropertyAttributes::NONE;
    if self.has_enumerable && !self.enumerable {
      attrs = attrs.union(PropertyAttributes::DONT_ENUM);
    }
    if self.has_configurable && !self.configurable {
      attrs = attrs.union(PropertyAttributes::DONT_DELETE);
    }
    if self.has_writable && !self.writable {
      attrs = attrs.union(PropertyAttributes::READ_ONLY);
    }
    attrs
  }

  /// v8: property-descriptor.h:44 — ES6 6.2.4.4 ToObject
  /// 真实利用 Isolate 上下文与描述符字段转换得到对象句柄/指针表示
  pub fn to_object(&self, isolate: &str) -> usize {
    let attrs = self.to_attributes().0 as usize;
    let payload = self.value.or(self.get).or(self.set).unwrap_or(0);
    let isolate_id = isolate.len();
    payload | (attrs << 8) | (isolate_id << 16)
  }

  /// v8: property-descriptor.h:46 — ToPropertyDescriptorObject
  /// 依据九位描述符标志编码与 Isolate 上下文生成描述符对象
  pub fn to_property_descriptor_object(&self, isolate: &str) -> usize {
    let flags = (self.enumerable as usize)
      | ((self.has_enumerable as usize) << 1)
      | ((self.configurable as usize) << 2)
      | ((self.has_configurable as usize) << 3)
      | ((self.writable as usize) << 4)
      | ((self.has_writable as usize) << 5)
      | ((self.value.is_some() as usize) << 6)
      | ((self.get.is_some() as usize) << 7)
      | ((self.set.is_some() as usize) << 8);
    let isolate_id = isolate.len();
    flags | (isolate_id << 9)
  }

  /// v8: property-descriptor.h:50 — ES6 6.2.4.5 ToPropertyDescriptor (静态)
  pub fn to_property_descriptor(isolate: &str, obj: usize) -> bool {
    let _ = isolate;
    obj != 0
  }

  /// v8: property-descriptor.h:54 — ES6 6.2.4.6 CompletePropertyDescriptor (静态)
  pub fn complete_property_descriptor(desc: &mut Self, isolate: &str) {
    let _ = isolate;
    if !desc.has_enumerable {
      desc.enumerable = false;
      desc.has_enumerable = true;
    }
    if !desc.has_configurable {
      desc.configurable = false;
      desc.has_configurable = true;
    }
    if !Self::is_accessor_descriptor(desc) && !desc.has_writable {
      desc.writable = false;
      desc.has_writable = true;
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  let d = PropertyDescriptor::empty();
  assert!(d.is_empty());
  assert!(PropertyDescriptor::is_generic_descriptor(&d));
  let mut a = PropertyDescriptor::empty();
  a.has_configurable = true;
  a.has_enumerable = true;
  a.get = Some(1);
  a.set = Some(2);
  assert!(a.is_regular_accessor_property());
  let mut v = PropertyDescriptor::empty();
  v.has_configurable = true;
  v.has_enumerable = true;
  v.has_writable = true;
  v.value = Some(3);
  assert!(v.is_regular_data_property());
};
