// v8-compiler-broker/src/heap_refs.rs
// 对应原版 v8/src/compiler/heap-refs.h 与 heap-refs.cc
//! HeapRef 引用族与快照对象模型
//!
//! 提供在后台编译线程上只读且无竞争访问堆对象元数据的安全引用层。
//! 引用对象轻量且实现了 Copy，内部仅持有对 Zone 中已序列化快照数据的借用。

use std::hash::{Hash, Hasher};

use v8_objects_core::{instance_type::InstanceType, property_details::PropertyDetails};
use v8_objects_js_core::field_index::FieldIndex;
use v8_objects_js_func::FunctionKind;

use crate::{
  broker::JSHeapBroker,
  error::{Error, Result},
};

/// 属性访问模式 (AccessMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum AccessMode {
  /// 读取属性
  #[default]
  Load = 0,
  /// 存储属性
  Store = 1,
  /// 字面量内部存储
  StoreInLiteral = 2,
  /// 查询属性是否存在
  Has = 3,
  /// 定义属性 (如 Object.defineProperty)
  Define = 4,
}

impl AccessMode {
  /// 是否属于任一写操作
  #[inline]
  pub const fn is_any_store(self) -> bool {
    matches!(self, Self::Store | Self::StoreInLiteral | Self::Define)
  }

  /// 是否属于定义类写操作
  #[inline]
  pub const fn is_defining_store(self) -> bool {
    matches!(self, Self::StoreInLiteral | Self::Define)
  }
}

/// 特殊单例类型 (OddballType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum OddballType {
  /// 非 Oddball
  #[default]
  None = 0,
  /// 通用布尔类型
  Boolean = 1,
  /// 布尔真
  True = 2,
  /// 布尔假
  False = 3,
  /// undefined
  Undefined = 4,
  /// null
  Null = 5,
}

impl OddballType {
  /// 是否为布尔单例
  #[inline]
  pub const fn is_boolean(self) -> bool {
    matches!(self, Self::Boolean | Self::True | Self::False)
  }

  /// 尝试转换为静态布尔值 (True -> Some(true), False/Undefined/Null -> Some(false))
  #[inline]
  pub const fn to_boolean(self) -> Option<bool> {
    match self {
      Self::True => Some(true),
      Self::False | Self::Undefined | Self::Null => Some(false),
      Self::None | Self::Boolean => None,
    }
  }
}

/// 元素存储种类 (ElementsKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum ElementsKind {
  /// 连续 Smi
  #[default]
  PackedSmi = 0,
  /// 稀疏 Smi
  HoleySmi = 1,
  /// 连续 Tagged 元素
  PackedElements = 2,
  /// 稀疏 Tagged 元素
  HoleyElements = 3,
  /// 连续双精度浮点
  PackedDouble = 4,
  /// 稀疏双精度浮点
  HoleyDouble = 5,
  /// 连续不可扩展
  PackedNonextensible = 6,
  /// 稀疏不可扩展
  HoleyNonextensible = 7,
  /// 连续封闭
  PackedSealed = 8,
  /// 稀疏封闭
  HoleySealed = 9,
  /// 连续冻结
  PackedFrozen = 10,
  /// 稀疏冻结
  HoleyFrozen = 11,
  /// 共享数组
  SharedArray = 12,
  /// 字典模式元素
  Dictionary = 13,
  /// Sloppy arguments 快速元素
  FastSloppyArguments = 14,
  /// Sloppy arguments 字典元素
  SlowSloppyArguments = 15,
  /// 字符串包装对象快速元素
  FastStringWrapper = 16,
  /// 字符串包装对象字典元素
  SlowStringWrapper = 17,
}

impl ElementsKind {
  /// 是否为连续/紧凑元素种类
  #[inline]
  pub const fn is_packed(self) -> bool {
    matches!(
      self,
      Self::PackedSmi
        | Self::PackedElements
        | Self::PackedDouble
        | Self::PackedNonextensible
        | Self::PackedSealed
        | Self::PackedFrozen
    )
  }

  /// 是否为稀疏洞元素种类
  #[inline]
  pub const fn is_holey(self) -> bool {
    matches!(
      self,
      Self::HoleySmi
        | Self::HoleyElements
        | Self::HoleyDouble
        | Self::HoleyNonextensible
        | Self::HoleySealed
        | Self::HoleyFrozen
    )
  }

  /// 是否为字典模式元素
  #[inline]
  pub const fn is_dictionary(self) -> bool {
    matches!(
      self,
      Self::Dictionary | Self::SlowSloppyArguments | Self::SlowStringWrapper
    )
  }
}

/// 堆对象静态类型信息快照
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeapObjectType {
  instance_type: InstanceType,
  elements_kind: ElementsKind,
  oddball_type: OddballType,
  is_callable: bool,
  is_undetectable: bool,
}

impl HeapObjectType {
  /// 构造新的类型信息快照
  pub const fn new(
    instance_type: InstanceType,
    elements_kind: ElementsKind,
    oddball_type: OddballType,
    is_callable: bool,
    is_undetectable: bool,
  ) -> Self {
    Self {
      instance_type,
      elements_kind,
      oddball_type,
      is_callable,
      is_undetectable,
    }
  }

  /// 获取实例类型
  #[inline]
  pub const fn instance_type(&self) -> InstanceType {
    self.instance_type
  }

  /// 获取元素种类
  #[inline]
  pub const fn elements_kind(&self) -> ElementsKind {
    self.elements_kind
  }

  /// 获取 Oddball 种类
  #[inline]
  pub const fn oddball_type(&self) -> OddballType {
    self.oddball_type
  }

  /// 是否可调用
  #[inline]
  pub const fn is_callable(&self) -> bool {
    self.is_callable
  }

  /// 是否不可检测 (如 document.all)
  #[inline]
  pub const fn is_undetectable(&self) -> bool {
    self.is_undetectable
  }
}

/// 属性描述符快照数据
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptorSnapshot<'z> {
  /// 属性名称
  pub key: &'z str,
  /// 属性详情 (包含类型、位置、只读/常量标志)
  pub details: PropertyDetails,
  /// 字段索引 (若是字段属性)
  pub field_index: Option<FieldIndex>,
  /// 引入该字段的 Map 地址
  pub field_owner_map: Option<u64>,
}

/// Map 对象快照数据
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapData<'z> {
  /// 对象实例分配大小（字节）
  pub instance_size: usize,
  /// 实例类型编号
  pub instance_type: InstanceType,
  /// 对象内属性数量
  pub in_object_properties: usize,
  /// 拥有自身的描述符数量
  pub number_of_own_descriptors: usize,
  /// 元素存储种类
  pub elements_kind: ElementsKind,
  /// 是否稳定 (未发生原型改变或迁移)
  pub is_stable: bool,
  /// 是否为构造函数 Map
  pub is_constructor: bool,
  /// 是否可调用
  pub is_callable: bool,
  /// 是否已废弃
  pub is_deprecated: bool,
  /// 是否不可检测
  pub is_undetectable: bool,
  /// 原型对象地址
  pub prototype: Option<u64>,
  /// 构造函数或反向指针地址
  pub constructor_or_back_pointer: Option<u64>,
  /// 描述符表快照
  pub descriptors: Vec<DescriptorSnapshot<'z>>,
  /// 构造计数器 (用于 Slack Tracking 预测与状态跟踪)
  pub construction_counter: u32,
}

/// JSFunction 快照数据
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JSFunctionData {
  /// 关联的 SharedFunctionInfo 地址
  pub shared_address: u64,
  /// 初始 Map 地址
  pub initial_map_address: Option<u64>,
  /// 实例原型地址
  pub prototype_address: Option<u64>,
  /// 上下文 Context 地址
  pub context_address: Option<u64>,
  /// 是否存在原型
  pub has_prototype: bool,
}

/// SharedFunctionInfo 快照数据
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedFunctionInfoData {
  /// 内建函数 ID (若为内建函数)
  pub builtin_id: Option<u32>,
  /// 函数种类
  pub function_kind: FunctionKind,
  /// 是否已完成编译
  pub is_compiled: bool,
  /// 是否为顶层脚本函数
  pub is_toplevel: bool,
  /// 是否含有字节码数组
  pub has_bytecode_array: bool,
  /// 形参个数
  pub parameter_count: usize,
}

/// JSObject 快照数据
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct JSObjectData {
  /// 对象内属性槽位值地址
  pub in_object_properties: Vec<Option<u64>>,
  /// 元素种类
  pub elements_kind: ElementsKind,
}

/// 全局属性单元快照数据 (PropertyCellData)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyCellData {
  /// 属性单元详情 (包含 Constness、CellType 等)
  pub details: PropertyDetails,
  /// 存储的目标值虚拟地址
  pub value_address: u64,
}

/// 分配点快照数据 (AllocationSiteData)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationSiteData {
  /// 元素存储种类
  pub elements_kind: ElementsKind,
  /// 样板对象 (boilerplate) 虚拟地址
  pub boilerplate_address: Option<u64>,
  /// 是否预留给老生代 (Pretenure)
  pub is_pretenured: bool,
}

/// 堆对象底层载荷快照
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeapObjectPayload<'z> {
  /// Map 结构快照
  Map(MapData<'z>),
  /// JSFunction 快照
  JSFunction(JSFunctionData),
  /// SharedFunctionInfo 快照
  SharedFunctionInfo(SharedFunctionInfoData),
  /// JSObject 快照
  JSObject(JSObjectData),
  /// PropertyCell 快照
  PropertyCell(PropertyCellData),
  /// AllocationSite 快照
  AllocationSite(AllocationSiteData),
  /// Oddball 快照
  Oddball {
    /// Oddball 单例种类
    oddball_type: OddballType,
    /// 具体的布尔值
    to_boolean: bool,
  },
  /// 通用堆对象
  Generic {
    /// 实例类型
    instance_type: InstanceType,
  },
}

/// 堆对象元数据快照结构
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeapObjectData<'z> {
  /// 该对象的 Map 地址
  pub map_address: u64,
  /// 专有载荷快照
  pub payload: HeapObjectPayload<'z>,
}

/// 对象底层存储枚举 (Smi 或 HeapObject)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectPayload<'z> {
  /// 立即数小整数
  Smi(i32),
  /// 堆对象结构
  HeapObject(HeapObjectData<'z>),
}

/// Zone 内分配的对象快照数据实体 (对应原版 ObjectData)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectData<'z> {
  /// 内存虚拟地址标识
  pub address: u64,
  /// 载荷数据
  pub payload: ObjectPayload<'z>,
}

// ── Ref 引用族定义 ────────────────────────────────────────────────────────

/// 堆对象代理系统的通用根引用 (对应原版 ObjectRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectRef<'z>(pub(crate) &'z ObjectData<'z>);

impl<'z> Hash for ObjectRef<'z> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.address().hash(state);
  }
}

impl<'z> ObjectRef<'z> {
  /// 包装底层 ObjectData 构造引用
  #[inline]
  pub const fn from_data(data: &'z ObjectData<'z>) -> Self {
    Self(data)
  }

  /// 获取底层 ObjectData 引用
  #[inline]
  pub const fn data(&self) -> &'z ObjectData<'z> {
    self.0
  }

  /// 获取对象虚拟内存地址
  #[inline]
  pub const fn address(&self) -> u64 {
    self.0.address
  }

  /// 对象身份哈希值
  #[inline]
  pub const fn hash_value(&self) -> u64 {
    self.address()
  }

  /// 判断引用相等性
  #[inline]
  pub fn equals(&self, other: &Self) -> bool {
    self.address() == other.address()
  }

  /// 是否为小整数 Smi
  #[inline]
  pub const fn is_smi(&self) -> bool {
    matches!(self.0.payload, ObjectPayload::Smi(_))
  }

  /// 转换为 Smi 数值
  #[inline]
  pub const fn as_smi(&self) -> Option<i32> {
    match self.0.payload {
      ObjectPayload::Smi(val) => Some(val),
      ObjectPayload::HeapObject(_) => None,
    }
  }

  /// 是否为堆对象
  #[inline]
  pub const fn is_heap_object(&self) -> bool {
    matches!(self.0.payload, ObjectPayload::HeapObject(_))
  }

  /// 转换为 HeapObjectRef
  #[inline]
  pub const fn as_heap_object(&self) -> Option<HeapObjectRef<'z>> {
    match &self.0.payload {
      ObjectPayload::HeapObject(data) => Some(HeapObjectRef(self.0, data)),
      ObjectPayload::Smi(_) => None,
    }
  }

  /// 是否为 Map
  #[inline]
  pub const fn is_map(&self) -> bool {
    if let Some(ho) = self.as_heap_object() {
      ho.is_map()
    } else {
      false
    }
  }

  /// 转换为 MapRef
  #[inline]
  pub const fn as_map(&self) -> Option<MapRef<'z>> {
    if let Some(ho) = self.as_heap_object() {
      ho.as_map()
    } else {
      None
    }
  }

  /// 是否为 JSFunction
  #[inline]
  pub const fn is_js_function(&self) -> bool {
    if let Some(ho) = self.as_heap_object() {
      ho.is_js_function()
    } else {
      false
    }
  }

  /// 转换为 JSFunctionRef
  #[inline]
  pub const fn as_js_function(&self) -> Option<JSFunctionRef<'z>> {
    if let Some(ho) = self.as_heap_object() {
      ho.as_js_function()
    } else {
      None
    }
  }

  /// 是否为 SharedFunctionInfo
  #[inline]
  pub const fn is_shared_function_info(&self) -> bool {
    if let Some(ho) = self.as_heap_object() {
      ho.is_shared_function_info()
    } else {
      false
    }
  }

  /// 转换为 SharedFunctionInfoRef
  #[inline]
  pub const fn as_shared_function_info(&self) -> Option<SharedFunctionInfoRef<'z>> {
    if let Some(ho) = self.as_heap_object() {
      ho.as_shared_function_info()
    } else {
      None
    }
  }

  /// 是否为 null
  #[inline]
  pub const fn is_null(&self) -> bool {
    if let Some(ho) = self.as_heap_object() {
      ho.is_null()
    } else {
      false
    }
  }

  /// 是否为 undefined
  #[inline]
  pub const fn is_undefined(&self) -> bool {
    if let Some(ho) = self.as_heap_object() {
      ho.is_undefined()
    } else {
      false
    }
  }

  /// 是否为布尔类型
  #[inline]
  pub const fn is_boolean(&self) -> bool {
    if let Some(ho) = self.as_heap_object() {
      ho.is_boolean()
    } else {
      false
    }
  }

  /// 是否为 Oddball 单例
  #[inline]
  pub const fn is_oddball(&self) -> bool {
    if let Some(ho) = self.as_heap_object() {
      ho.is_oddball()
    } else {
      false
    }
  }

  /// 转换为 OddballRef
  #[inline]
  pub const fn as_oddball(&self) -> Option<OddballRef<'z>> {
    if let Some(ho) = self.as_heap_object() {
      ho.as_oddball()
    } else {
      None
    }
  }

  /// 是否为 PropertyCell
  #[inline]
  pub const fn is_property_cell(&self) -> bool {
    if let Some(ho) = self.as_heap_object() {
      ho.is_property_cell()
    } else {
      false
    }
  }

  /// 转换为 PropertyCellRef
  #[inline]
  pub const fn as_property_cell(&self) -> Option<PropertyCellRef<'z>> {
    if let Some(ho) = self.as_heap_object() {
      ho.as_property_cell()
    } else {
      None
    }
  }

  /// 是否为 AllocationSite
  #[inline]
  pub const fn is_allocation_site(&self) -> bool {
    if let Some(ho) = self.as_heap_object() {
      ho.is_allocation_site()
    } else {
      false
    }
  }

  /// 转换为 AllocationSiteRef
  #[inline]
  pub const fn as_allocation_site(&self) -> Option<AllocationSiteRef<'z>> {
    if let Some(ho) = self.as_heap_object() {
      ho.as_allocation_site()
    } else {
      None
    }
  }

  /// 尝试获取静态布尔真假值
  #[inline]
  pub fn to_boolean(&self) -> Option<bool> {
    self.try_get_boolean_value()
  }

  /// 尝试获取静态布尔真假值
  pub fn try_get_boolean_value(&self) -> Option<bool> {
    match &self.0.payload {
      ObjectPayload::Smi(val) => Some(*val != 0),
      ObjectPayload::HeapObject(ho) => match &ho.payload {
        HeapObjectPayload::Oddball { to_boolean, .. } => Some(*to_boolean),
        HeapObjectPayload::Map(m) => Some(!m.is_undetectable),
        _ => Some(true),
      },
    }
  }
}

/// 堆对象引用 (对应原版 HeapObjectRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeapObjectRef<'z>(
  pub(crate) &'z ObjectData<'z>,
  pub(crate) &'z HeapObjectData<'z>,
);

impl<'z> Hash for HeapObjectRef<'z> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.address().hash(state);
  }
}

impl<'z> HeapObjectRef<'z> {
  /// 获取基础 ObjectRef
  #[inline]
  pub const fn object(&self) -> ObjectRef<'z> {
    ObjectRef(self.0)
  }

  /// 获取对象虚拟内存地址
  #[inline]
  pub const fn address(&self) -> u64 {
    self.0.address
  }

  /// 获取该堆对象的 Map 地址
  #[inline]
  pub const fn map_address(&self) -> u64 {
    self.1.map_address
  }

  /// 通过 broker 查询当前对象的 MapRef
  pub fn map<'a>(&self, broker: &JSHeapBroker<'z, 'a>) -> Result<MapRef<'z>> {
    let map_obj = broker.get_ref(self.1.map_address)?;
    map_obj.as_map().ok_or(Error::TypeMismatch {
      expected: "Map",
      actual: "NonMapObject",
    })
  }

  /// 获取静态 HeapObjectType
  pub fn get_heap_object_type<'a>(&self, broker: &JSHeapBroker<'z, 'a>) -> Result<HeapObjectType> {
    let map = self.map(broker)?;
    Ok(HeapObjectType::new(
      map.instance_type(),
      map.elements_kind(),
      match &self.1.payload {
        HeapObjectPayload::Oddball { oddball_type, .. } => *oddball_type,
        _ => OddballType::None,
      },
      map.is_callable(),
      map.is_undetectable(),
    ))
  }

  /// 是否为 Map
  #[inline]
  pub const fn is_map(&self) -> bool {
    matches!(self.1.payload, HeapObjectPayload::Map(_))
  }

  /// 转换为 MapRef
  #[inline]
  pub const fn as_map(&self) -> Option<MapRef<'z>> {
    match &self.1.payload {
      HeapObjectPayload::Map(map_data) => Some(MapRef(self.0, self.1, map_data)),
      _ => None,
    }
  }

  /// 是否为 JSFunction
  #[inline]
  pub const fn is_js_function(&self) -> bool {
    matches!(self.1.payload, HeapObjectPayload::JSFunction(_))
  }

  /// 转换为 JSFunctionRef
  #[inline]
  pub const fn as_js_function(&self) -> Option<JSFunctionRef<'z>> {
    match &self.1.payload {
      HeapObjectPayload::JSFunction(func_data) => Some(JSFunctionRef(self.0, self.1, func_data)),
      _ => None,
    }
  }

  /// 是否为 SharedFunctionInfo
  #[inline]
  pub const fn is_shared_function_info(&self) -> bool {
    matches!(self.1.payload, HeapObjectPayload::SharedFunctionInfo(_))
  }

  /// 转换为 SharedFunctionInfoRef
  #[inline]
  pub const fn as_shared_function_info(&self) -> Option<SharedFunctionInfoRef<'z>> {
    match &self.1.payload {
      HeapObjectPayload::SharedFunctionInfo(sfi_data) => {
        Some(SharedFunctionInfoRef(self.0, self.1, sfi_data))
      }
      _ => None,
    }
  }

  /// 是否为 null
  #[inline]
  pub const fn is_null(&self) -> bool {
    matches!(
      self.1.payload,
      HeapObjectPayload::Oddball {
        oddball_type: OddballType::Null,
        ..
      }
    )
  }

  /// 是否为 undefined
  #[inline]
  pub const fn is_undefined(&self) -> bool {
    matches!(
      self.1.payload,
      HeapObjectPayload::Oddball {
        oddball_type: OddballType::Undefined,
        ..
      }
    )
  }

  /// 是否为布尔类型
  #[inline]
  pub const fn is_boolean(&self) -> bool {
    matches!(
      self.1.payload,
      HeapObjectPayload::Oddball {
        oddball_type: OddballType::Boolean | OddballType::True | OddballType::False,
        ..
      }
    )
  }

  /// 是否为 Oddball 单例
  #[inline]
  pub const fn is_oddball(&self) -> bool {
    matches!(self.1.payload, HeapObjectPayload::Oddball { .. })
  }

  /// 转换为 OddballRef
  #[inline]
  pub const fn as_oddball(&self) -> Option<OddballRef<'z>> {
    match &self.1.payload {
      HeapObjectPayload::Oddball {
        oddball_type,
        to_boolean,
      } => Some(OddballRef(self.0, self.1, *oddball_type, *to_boolean)),
      _ => None,
    }
  }

  /// 是否为 PropertyCell
  #[inline]
  pub const fn is_property_cell(&self) -> bool {
    matches!(self.1.payload, HeapObjectPayload::PropertyCell(_))
  }

  /// 转换为 PropertyCellRef
  #[inline]
  pub const fn as_property_cell(&self) -> Option<PropertyCellRef<'z>> {
    match &self.1.payload {
      HeapObjectPayload::PropertyCell(data) => Some(PropertyCellRef(self.0, self.1, data)),
      _ => None,
    }
  }

  /// 是否为 AllocationSite
  #[inline]
  pub const fn is_allocation_site(&self) -> bool {
    matches!(self.1.payload, HeapObjectPayload::AllocationSite(_))
  }

  /// 转换为 AllocationSiteRef
  #[inline]
  pub const fn as_allocation_site(&self) -> Option<AllocationSiteRef<'z>> {
    match &self.1.payload {
      HeapObjectPayload::AllocationSite(data) => Some(AllocationSiteRef(self.0, self.1, data)),
      _ => None,
    }
  }
}

/// Map 结构引用 (对应原版 MapRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapRef<'z>(
  pub(crate) &'z ObjectData<'z>,
  pub(crate) &'z HeapObjectData<'z>,
  pub(crate) &'z MapData<'z>,
);

impl<'z> Hash for MapRef<'z> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.address().hash(state);
  }
}

impl<'z> MapRef<'z> {
  /// 获取底层 HeapObjectRef
  #[inline]
  pub const fn heap_object(&self) -> HeapObjectRef<'z> {
    HeapObjectRef(self.0, self.1)
  }

  /// 获取底层 ObjectRef
  #[inline]
  pub const fn object(&self) -> ObjectRef<'z> {
    ObjectRef(self.0)
  }

  /// Map 内存虚拟地址
  #[inline]
  pub const fn address(&self) -> u64 {
    self.0.address
  }

  /// 对象实例分配字节数
  #[inline]
  pub const fn instance_size(&self) -> usize {
    self.2.instance_size
  }

  /// 实例类型编号
  #[inline]
  pub const fn instance_type(&self) -> InstanceType {
    self.2.instance_type
  }

  /// 对象内属性数量
  #[inline]
  pub const fn in_object_properties(&self) -> usize {
    self.2.in_object_properties
  }

  /// 描述符表包含的属性数量
  #[inline]
  pub const fn number_of_own_descriptors(&self) -> usize {
    self.2.number_of_own_descriptors
  }

  /// 元素种类
  #[inline]
  pub const fn elements_kind(&self) -> ElementsKind {
    self.2.elements_kind
  }

  /// 是否为稳定 Map
  #[inline]
  pub const fn is_stable(&self) -> bool {
    self.2.is_stable
  }

  /// 是否为构造函数 Map
  #[inline]
  pub const fn is_constructor(&self) -> bool {
    self.2.is_constructor
  }

  /// 是否可调用
  #[inline]
  pub const fn is_callable(&self) -> bool {
    self.2.is_callable
  }

  /// 是否已废弃
  #[inline]
  pub const fn is_deprecated(&self) -> bool {
    self.2.is_deprecated
  }

  /// 是否不可检测
  #[inline]
  pub const fn is_undetectable(&self) -> bool {
    self.2.is_undetectable
  }

  /// 获取 Slack Tracking 构造计数器
  #[inline]
  pub const fn construction_counter(&self) -> u32 {
    self.2.construction_counter
  }

  /// 原型对象引用
  pub fn prototype<'a>(&self, broker: &JSHeapBroker<'z, 'a>) -> Result<Option<HeapObjectRef<'z>>> {
    if let Some(proto_addr) = self.2.prototype {
      let obj = broker.get_ref(proto_addr)?;
      Ok(obj.as_heap_object())
    } else {
      Ok(None)
    }
  }

  /// 构造函数或反向指针引用
  pub fn constructor_or_back_pointer<'a>(
    &self,
    broker: &JSHeapBroker<'z, 'a>,
  ) -> Result<Option<ObjectRef<'z>>> {
    if let Some(addr) = self.2.constructor_or_back_pointer {
      let obj = broker.get_ref(addr)?;
      Ok(Some(obj))
    } else {
      Ok(None)
    }
  }

  /// 获取描述符切片
  #[inline]
  pub fn descriptors(&self) -> &[DescriptorSnapshot<'z>] {
    &self.2.descriptors
  }

  /// 按索引查询属性详情 (PropertyDetails)
  pub fn get_property_details(&self, descriptor_index: usize) -> Option<PropertyDetails> {
    self.2.descriptors.get(descriptor_index).map(|d| d.details)
  }

  /// 按索引查询属性名称
  pub fn get_property_name(&self, descriptor_index: usize) -> Option<&'z str> {
    self.2.descriptors.get(descriptor_index).map(|d| d.key)
  }

  /// 按索引查询字段索引 (FieldIndex)
  pub fn get_field_index(&self, descriptor_index: usize) -> Option<FieldIndex> {
    self
      .2
      .descriptors
      .get(descriptor_index)
      .and_then(|d| d.field_index)
  }

  /// 查找字段所有者 Map
  pub fn find_field_owner<'a>(
    &self,
    descriptor_index: usize,
    broker: &JSHeapBroker<'z, 'a>,
  ) -> Result<Option<MapRef<'z>>> {
    if let Some(desc) = self.2.descriptors.get(descriptor_index)
      && let Some(owner_addr) = desc.field_owner_map
    {
      let obj = broker.get_ref(owner_addr)?;
      return Ok(obj.as_map());
    }
    Ok(None)
  }
}

/// JSFunction 函数对象引用 (对应原版 JSFunctionRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JSFunctionRef<'z>(
  pub(crate) &'z ObjectData<'z>,
  pub(crate) &'z HeapObjectData<'z>,
  pub(crate) &'z JSFunctionData,
);

impl<'z> Hash for JSFunctionRef<'z> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.address().hash(state);
  }
}

impl<'z> JSFunctionRef<'z> {
  /// 获取底层 HeapObjectRef
  #[inline]
  pub const fn heap_object(&self) -> HeapObjectRef<'z> {
    HeapObjectRef(self.0, self.1)
  }

  /// 获取底层 ObjectRef
  #[inline]
  pub const fn object(&self) -> ObjectRef<'z> {
    ObjectRef(self.0)
  }

  /// 对象内存地址
  #[inline]
  pub const fn address(&self) -> u64 {
    self.0.address
  }

  /// 获取对应的 SharedFunctionInfoRef
  pub fn shared<'a>(&self, broker: &JSHeapBroker<'z, 'a>) -> Result<SharedFunctionInfoRef<'z>> {
    let obj = broker.get_ref(self.2.shared_address)?;
    obj
      .as_shared_function_info()
      .ok_or(Error::MissingSharedFunctionInfo)
  }

  /// 是否包含初始 Map
  #[inline]
  pub const fn has_initial_map(&self) -> bool {
    self.2.initial_map_address.is_some()
  }

  /// 获取函数初始 Map
  pub fn initial_map<'a>(&self, broker: &JSHeapBroker<'z, 'a>) -> Result<MapRef<'z>> {
    let map_addr = self.2.initial_map_address.ok_or(Error::MissingInitialMap)?;
    let obj = broker.get_ref(map_addr)?;
    obj.as_map().ok_or(Error::TypeMismatch {
      expected: "Map",
      actual: "NonMapObject",
    })
  }

  /// 是否包含实例原型对象
  #[inline]
  pub const fn has_instance_prototype(&self) -> bool {
    self.2.prototype_address.is_some()
  }

  /// 获取实例原型对象
  pub fn instance_prototype<'a>(&self, broker: &JSHeapBroker<'z, 'a>) -> Result<HeapObjectRef<'z>> {
    let proto_addr = self.2.prototype_address.ok_or(Error::MissingPrototype)?;
    let obj = broker.get_ref(proto_addr)?;
    obj.as_heap_object().ok_or(Error::TypeMismatch {
      expected: "HeapObject",
      actual: "Smi",
    })
  }

  /// 获取上下文 Context 对象
  pub fn context<'a>(&self, broker: &JSHeapBroker<'z, 'a>) -> Result<Option<HeapObjectRef<'z>>> {
    if let Some(ctx_addr) = self.2.context_address {
      let obj = broker.get_ref(ctx_addr)?;
      Ok(obj.as_heap_object())
    } else {
      Ok(None)
    }
  }
}

/// SharedFunctionInfo 函数共享元数据引用 (对应原版 SharedFunctionInfoRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedFunctionInfoRef<'z>(
  pub(crate) &'z ObjectData<'z>,
  pub(crate) &'z HeapObjectData<'z>,
  pub(crate) &'z SharedFunctionInfoData,
);

impl<'z> Hash for SharedFunctionInfoRef<'z> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.address().hash(state);
  }
}

impl<'z> SharedFunctionInfoRef<'z> {
  /// 获取底层 HeapObjectRef
  #[inline]
  pub const fn heap_object(&self) -> HeapObjectRef<'z> {
    HeapObjectRef(self.0, self.1)
  }

  /// 获取底层 ObjectRef
  #[inline]
  pub const fn object(&self) -> ObjectRef<'z> {
    ObjectRef(self.0)
  }

  /// 对象虚拟内存地址
  #[inline]
  pub const fn address(&self) -> u64 {
    self.0.address
  }

  /// 获取内建函数 ID
  #[inline]
  pub const fn builtin_id(&self) -> Option<u32> {
    self.2.builtin_id
  }

  /// 获取函数种类
  #[inline]
  pub const fn function_kind(&self) -> FunctionKind {
    self.2.function_kind
  }

  /// 是否已完成编译
  #[inline]
  pub const fn is_compiled(&self) -> bool {
    self.2.is_compiled
  }

  /// 是否为顶层脚本函数
  #[inline]
  pub const fn is_toplevel(&self) -> bool {
    self.2.is_toplevel
  }

  /// 是否含有字节码
  #[inline]
  pub const fn has_bytecode_array(&self) -> bool {
    self.2.has_bytecode_array
  }

  /// 获取形参数量
  #[inline]
  pub const fn parameter_count(&self) -> usize {
    self.2.parameter_count
  }
}

/// Oddball 单例引用 (对应原版 OddballRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OddballRef<'z>(
  pub(crate) &'z ObjectData<'z>,
  pub(crate) &'z HeapObjectData<'z>,
  pub(crate) OddballType,
  pub(crate) bool,
);

impl<'z> Hash for OddballRef<'z> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.address().hash(state);
  }
}

impl<'z> OddballRef<'z> {
  /// 获取底层 HeapObjectRef
  #[inline]
  pub const fn heap_object(&self) -> HeapObjectRef<'z> {
    HeapObjectRef(self.0, self.1)
  }

  /// 获取底层 ObjectRef
  #[inline]
  pub const fn object(&self) -> ObjectRef<'z> {
    ObjectRef(self.0)
  }

  /// 对象内存虚拟地址
  #[inline]
  pub const fn address(&self) -> u64 {
    self.0.address
  }

  /// Oddball 单例类型
  #[inline]
  pub const fn oddball_type(&self) -> OddballType {
    self.2
  }

  /// 布尔真假判定值
  #[inline]
  pub const fn to_boolean_value(&self) -> bool {
    self.3
  }

  /// 布尔真假判定值
  #[inline]
  pub const fn to_boolean(&self) -> bool {
    self.3
  }
}

/// 全局属性单元引用 (对应原版 PropertyCellRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyCellRef<'z>(
  pub(crate) &'z ObjectData<'z>,
  pub(crate) &'z HeapObjectData<'z>,
  pub(crate) &'z PropertyCellData,
);

impl<'z> Hash for PropertyCellRef<'z> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.address().hash(state);
  }
}

impl<'z> PropertyCellRef<'z> {
  /// 获取底层 HeapObjectRef
  #[inline]
  pub const fn heap_object(&self) -> HeapObjectRef<'z> {
    HeapObjectRef(self.0, self.1)
  }

  /// 获取底层 ObjectRef
  #[inline]
  pub const fn object(&self) -> ObjectRef<'z> {
    ObjectRef(self.0)
  }

  /// 对象内存虚拟地址
  #[inline]
  pub const fn address(&self) -> u64 {
    self.0.address
  }

  /// 获取属性详情 (PropertyDetails)
  #[inline]
  pub const fn property_details(&self) -> PropertyDetails {
    self.2.details
  }

  /// 获取存储的目标值虚拟地址
  #[inline]
  pub const fn value_address(&self) -> u64 {
    self.2.value_address
  }

  /// 查询属性单元当前存储的值对象引用
  pub fn value<'a>(&self, broker: &JSHeapBroker<'z, 'a>) -> Result<ObjectRef<'z>> {
    broker.get_ref(self.2.value_address)
  }
}

/// 分配点引用 (对应原版 AllocationSiteRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationSiteRef<'z>(
  pub(crate) &'z ObjectData<'z>,
  pub(crate) &'z HeapObjectData<'z>,
  pub(crate) &'z AllocationSiteData,
);

impl<'z> Hash for AllocationSiteRef<'z> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.address().hash(state);
  }
}

impl<'z> AllocationSiteRef<'z> {
  /// 获取底层 HeapObjectRef
  #[inline]
  pub const fn heap_object(&self) -> HeapObjectRef<'z> {
    HeapObjectRef(self.0, self.1)
  }

  /// 获取底层 ObjectRef
  #[inline]
  pub const fn object(&self) -> ObjectRef<'z> {
    ObjectRef(self.0)
  }

  /// 对象内存虚拟地址
  #[inline]
  pub const fn address(&self) -> u64 {
    self.0.address
  }

  /// 获取元素种类
  #[inline]
  pub const fn elements_kind(&self) -> ElementsKind {
    self.2.elements_kind
  }

  /// 获取样板对象地址
  #[inline]
  pub const fn boilerplate_address(&self) -> Option<u64> {
    self.2.boilerplate_address
  }

  /// 是否预留给老生代 (Pretenure)
  #[inline]
  pub const fn is_pretenured(&self) -> bool {
    self.2.is_pretenured
  }

  /// 查询样板对象 (Boilerplate) 堆对象引用
  pub fn boilerplate<'a>(
    &self,
    broker: &JSHeapBroker<'z, 'a>,
  ) -> Result<Option<HeapObjectRef<'z>>> {
    if let Some(addr) = self.2.boilerplate_address {
      let obj = broker.get_ref(addr)?;
      Ok(obj.as_heap_object())
    } else {
      Ok(None)
    }
  }
}
