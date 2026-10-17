// v8-compiler-broker/src/broker.rs
// 对应原版 v8/src/compiler/js-heap-broker.h 与 js-heap-broker.cc
//! JSHeapBroker: V8 优化编译器堆对象快照代理中心
//!
//! 在多线程编译架构下，优化编译器运行在独立的后台线程中，
//! 所有对 JS 堆对象的访问均通过 `JSHeapBroker` 代理，
//! 避免后台编译器与主线程 JavaScript 执行发生数据竞争。

use gxhash::HashMap;
use v8_compiler_graph::FeedbackSource;
use v8_zone::Zone;

use crate::{
  compilation_dependencies::CompilationDependencies,
  error::{Error, Result},
  heap_refs::{
    AllocationSiteData, AllocationSiteRef, HeapObjectData, HeapObjectPayload, HeapObjectRef,
    JSFunctionData, JSFunctionRef, JSObjectData, MapData, MapRef, ObjectData, ObjectPayload,
    ObjectRef, OddballType, PropertyCellData, PropertyCellRef, SharedFunctionInfoData,
    SharedFunctionInfoRef,
  },
};

/// 编译器堆代理状态模式 (BrokerMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum BrokerMode {
  /// 初始禁用态: 仅做基础初始化，尚未开始序列化
  #[default]
  Disabled = 0,
  /// 序列化就绪态: 主线程并发抓取堆对象快照，存入 Broker Zone
  Serializing = 1,
  /// 序列化完成态: 后台编译线程只读消费缓存快照
  Serialized = 2,
  /// 退休态: 编译完成，等待销毁或提交依赖
  Retired = 3,
}

/// 反馈插槽处理快照
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessedFeedbackSnapshot<'z> {
  /// 信息不足
  Insufficient,
  /// 单态/多态 Map 列表
  Maps(Vec<MapRef<'z>>),
  /// 调用目标
  CallTarget(Option<HeapObjectRef<'z>>),
  /// 二元操作类型提示
  BinaryHint(u32),
  /// 比较操作类型提示
  CompareHint(u32),
}

/// 堆对象代理属主 (JSHeapBroker)
pub struct JSHeapBroker<'z, 'a> {
  /// Zone arena 内存分配器 (借用 'z)
  zone: &'z Zone<'a>,
  /// 当前状态模式
  mode: BrokerMode,
  /// 是否启用详细追踪日志
  tracing_enabled: bool,
  /// 指针压缩笼基地址 (Cage Base)
  cage_base: u64,
  /// 目标编译本地上下文 NativeContext 虚拟地址
  target_native_context: Option<u64>,
  /// Zone 分配的对象快照快速缓存表 (地址 -> ObjectData)
  refs: HashMap<u64, &'z ObjectData<'z>>,
  /// 处理后的反馈信息快照表 (FeedbackSource -> 快照)
  feedback: HashMap<FeedbackSource, ProcessedFeedbackSnapshot<'z>>,
  /// 语言保护器状态表 (protector_id -> is_valid)
  protectors: HashMap<u32, bool>,
  /// 自定义编译期假设数据表 ((tag, key) -> expected)
  custom_data: HashMap<(&'z str, u64), u64>,
  /// 关联的编译期假设依赖记录器
  dependencies: Option<CompilationDependencies<'z>>,
}

impl<'z, 'a> JSHeapBroker<'z, 'a> {
  /// 创建新的 JSHeapBroker 实例 (初始处于 Disabled 模式)
  pub fn new(zone: &'z Zone<'a>) -> Self {
    Self::with_tracing(zone, false)
  }

  /// 创建带追踪配置的 JSHeapBroker
  pub fn with_tracing(zone: &'z Zone<'a>, tracing_enabled: bool) -> Self {
    Self {
      zone,
      mode: BrokerMode::Disabled,
      tracing_enabled,
      cage_base: 0,
      target_native_context: None,
      refs: HashMap::default(),
      feedback: HashMap::default(),
      protectors: HashMap::default(),
      custom_data: HashMap::default(),
      dependencies: None,
    }
  }

  /// 获取所关联的 Zone 引用
  #[inline]
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  /// 获取当前 BrokerMode 模式
  #[inline]
  pub const fn mode(&self) -> BrokerMode {
    self.mode
  }

  /// 是否启用了追踪日志
  #[inline]
  pub const fn tracing_enabled(&self) -> bool {
    self.tracing_enabled
  }

  /// 设置追踪日志开关
  #[inline]
  pub fn set_tracing_enabled(&mut self, enabled: bool) {
    self.tracing_enabled = enabled;
  }

  /// 获取指针压缩基地址
  #[inline]
  pub const fn cage_base(&self) -> u64 {
    self.cage_base
  }

  /// 设置指针压缩基地址
  #[inline]
  pub fn set_cage_base(&mut self, cage_base: u64) {
    self.cage_base = cage_base;
  }

  /// 获取目标 NativeContext 地址
  #[inline]
  pub const fn target_native_context(&self) -> Option<u64> {
    self.target_native_context
  }

  /// 设置目标 NativeContext 地址
  #[inline]
  pub fn set_target_native_context(&mut self, address: u64) {
    self.target_native_context = Some(address);
  }

  /// 是否允许向 Broker 执行序列化写入
  #[inline]
  pub const fn serializing_allowed(&self) -> bool {
    matches!(self.mode, BrokerMode::Serializing)
  }

  /// 状态机迁移: 初始化并开始序列化 (Disabled -> Serializing)
  pub fn initialize_and_start_serializing(
    &mut self,
    target_native_context: Option<u64>,
  ) -> Result<()> {
    if self.mode != BrokerMode::Disabled {
      return Err(Error::InvalidModeTransition {
        from: self.mode,
        to: BrokerMode::Serializing,
      });
    }

    if self.tracing_enabled {
      log::trace!("[JSHeapBroker] Starting serialization");
    }

    self.mode = BrokerMode::Serializing;
    self.refs.clear();
    self.feedback.clear();
    self.protectors.clear();
    self.custom_data.clear();
    self.target_native_context = target_native_context;
    Ok(())
  }

  /// 状态机迁移: 停止序列化 (Serializing -> Serialized)
  pub fn stop_serializing(&mut self) -> Result<()> {
    if self.mode != BrokerMode::Serializing {
      return Err(Error::InvalidModeTransition {
        from: self.mode,
        to: BrokerMode::Serialized,
      });
    }

    if self.tracing_enabled {
      log::trace!("[JSHeapBroker] Stopping serialization");
    }

    self.mode = BrokerMode::Serialized;
    Ok(())
  }

  /// 状态机迁移: 退休注销 (Serialized -> Retired)
  pub fn retire(&mut self) -> Result<()> {
    if self.mode != BrokerMode::Serialized {
      return Err(Error::InvalidModeTransition {
        from: self.mode,
        to: BrokerMode::Retired,
      });
    }

    if self.tracing_enabled {
      log::trace!("[JSHeapBroker] Retiring broker");
    }

    self.mode = BrokerMode::Retired;
    Ok(())
  }

  /// 挂载编译期依赖记录器
  pub fn attach_dependencies(&mut self, dependencies: CompilationDependencies<'z>) {
    self.dependencies = Some(dependencies);
  }

  /// 读取编译期依赖引用
  #[inline]
  pub fn dependencies(&self) -> Option<&CompilationDependencies<'z>> {
    self.dependencies.as_ref()
  }

  /// 获取编译期依赖可变引用
  #[inline]
  pub fn dependencies_mut(&mut self) -> Option<&mut CompilationDependencies<'z>> {
    self.dependencies.as_mut()
  }

  /// 查询保护器是否有效 (默认未记录时为 true，符合 V8 初始有效假设)
  #[inline]
  pub fn is_protector_valid(&self, protector_id: u32) -> bool {
    self.protectors.get(&protector_id).copied().unwrap_or(true)
  }

  /// 设置保护器有效性状态
  #[inline]
  pub fn set_protector(&mut self, protector_id: u32, is_valid: bool) {
    self.protectors.insert(protector_id, is_valid);
  }

  /// 使保护器失效 (InvalidateProtector)
  #[inline]
  pub fn invalidate_protector(&mut self, protector_id: u32) {
    self.set_protector(protector_id, false);
  }

  /// 查询自定义假设数据
  #[inline]
  pub fn get_custom_data(&self, tag: &str, key: u64) -> Option<u64> {
    self.custom_data.get(&(tag, key)).copied()
  }

  /// 记录自定义假设数据
  #[inline]
  pub fn set_custom_data(&mut self, tag: &'z str, key: u64, value: u64) {
    self.custom_data.insert((tag, key), value);
  }

  /// 注册原始 ObjectData 快照入 Zone 与缓存表
  pub fn register_data(&mut self, data: ObjectData<'z>) -> Result<ObjectRef<'z>> {
    if !self.serializing_allowed() {
      return Err(Error::SerializationNotAllowed(self.mode));
    }

    let addr = data.address;
    let allocated = self
      .zone
      .make(data)
      .map_err(|e| Error::ZoneAllocation(e.to_string()))?;
    let static_ref: &'z ObjectData<'z> = allocated;
    self.refs.insert(addr, static_ref);
    Ok(ObjectRef::from_data(static_ref))
  }

  /// 构造并缓存小整数 Smi 引用
  pub fn make_smi(&mut self, value: i32) -> Result<ObjectRef<'z>> {
    // Smi 编码虚拟地址 (最低位置 0，高 32 位装载值)
    let addr = ((value as u32 as u64) << 32) | 0x1;
    let data = ObjectData {
      address: addr,
      payload: ObjectPayload::Smi(value),
    };
    self.register_data(data)
  }

  /// 构造并缓存 Map 引用
  pub fn make_map(&mut self, address: u64, map_data: MapData<'z>) -> Result<MapRef<'z>> {
    // Map 的 map 指针指向自身或 MetaMap (Address)
    let ho_data = HeapObjectData {
      map_address: address,
      payload: HeapObjectPayload::Map(map_data),
    };
    let data = ObjectData {
      address,
      payload: ObjectPayload::HeapObject(ho_data),
    };
    let obj_ref = self.register_data(data)?;
    obj_ref.as_map().ok_or(Error::TypeMismatch {
      expected: "Map",
      actual: "NonMap",
    })
  }

  /// 构造并缓存 JSFunction 引用
  pub fn make_js_function(
    &mut self,
    address: u64,
    func_data: JSFunctionData,
    map_address: u64,
  ) -> Result<JSFunctionRef<'z>> {
    let ho_data = HeapObjectData {
      map_address,
      payload: HeapObjectPayload::JSFunction(func_data),
    };
    let data = ObjectData {
      address,
      payload: ObjectPayload::HeapObject(ho_data),
    };
    let obj_ref = self.register_data(data)?;
    obj_ref.as_js_function().ok_or(Error::TypeMismatch {
      expected: "JSFunction",
      actual: "NonJSFunction",
    })
  }

  /// 构造并缓存 SharedFunctionInfo 引用
  pub fn make_shared_function_info(
    &mut self,
    address: u64,
    sfi_data: SharedFunctionInfoData,
    map_address: u64,
  ) -> Result<SharedFunctionInfoRef<'z>> {
    let ho_data = HeapObjectData {
      map_address,
      payload: HeapObjectPayload::SharedFunctionInfo(sfi_data),
    };
    let data = ObjectData {
      address,
      payload: ObjectPayload::HeapObject(ho_data),
    };
    let obj_ref = self.register_data(data)?;
    obj_ref
      .as_shared_function_info()
      .ok_or(Error::TypeMismatch {
        expected: "SharedFunctionInfo",
        actual: "NonSharedFunctionInfo",
      })
  }

  /// 构造并缓存通用 JSObject 引用
  pub fn make_js_object(
    &mut self,
    address: u64,
    js_object_data: JSObjectData,
    map_address: u64,
  ) -> Result<HeapObjectRef<'z>> {
    let ho_data = HeapObjectData {
      map_address,
      payload: HeapObjectPayload::JSObject(js_object_data),
    };
    let data = ObjectData {
      address,
      payload: ObjectPayload::HeapObject(ho_data),
    };
    let obj_ref = self.register_data(data)?;
    obj_ref.as_heap_object().ok_or(Error::TypeMismatch {
      expected: "HeapObject",
      actual: "Smi",
    })
  }

  /// 构造并缓存 Oddball 引用 (默认由 oddball_type 决定真假)
  pub fn make_oddball(
    &mut self,
    address: u64,
    oddball_type: OddballType,
    map_address: u64,
  ) -> Result<ObjectRef<'z>> {
    let to_boolean = matches!(oddball_type, OddballType::True);
    self.make_oddball_with_boolean(address, oddball_type, to_boolean, map_address)
  }

  /// 构造并显式指定布尔判定值的 Oddball 引用
  pub fn make_oddball_with_boolean(
    &mut self,
    address: u64,
    oddball_type: OddballType,
    to_boolean: bool,
    map_address: u64,
  ) -> Result<ObjectRef<'z>> {
    let ho_data = HeapObjectData {
      map_address,
      payload: HeapObjectPayload::Oddball {
        oddball_type,
        to_boolean,
      },
    };
    let data = ObjectData {
      address,
      payload: ObjectPayload::HeapObject(ho_data),
    };
    self.register_data(data)
  }

  /// 构造布尔真假值 Oddball 引用
  pub fn make_oddball_boolean(
    &mut self,
    address: u64,
    value: bool,
    map_address: u64,
  ) -> Result<ObjectRef<'z>> {
    self.make_oddball_with_boolean(
      address,
      if value {
        OddballType::True
      } else {
        OddballType::False
      },
      value,
      map_address,
    )
  }

  /// 构造并缓存 PropertyCell 引用
  pub fn make_property_cell(
    &mut self,
    address: u64,
    cell_data: PropertyCellData,
    map_address: u64,
  ) -> Result<PropertyCellRef<'z>> {
    let ho_data = HeapObjectData {
      map_address,
      payload: HeapObjectPayload::PropertyCell(cell_data),
    };
    let data = ObjectData {
      address,
      payload: ObjectPayload::HeapObject(ho_data),
    };
    let obj_ref = self.register_data(data)?;
    obj_ref.as_property_cell().ok_or(Error::TypeMismatch {
      expected: "PropertyCell",
      actual: "NonPropertyCell",
    })
  }

  /// 构造并缓存 AllocationSite 引用
  pub fn make_allocation_site(
    &mut self,
    address: u64,
    site_data: AllocationSiteData,
    map_address: u64,
  ) -> Result<AllocationSiteRef<'z>> {
    let ho_data = HeapObjectData {
      map_address,
      payload: HeapObjectPayload::AllocationSite(site_data),
    };
    let data = ObjectData {
      address,
      payload: ObjectPayload::HeapObject(ho_data),
    };
    let obj_ref = self.register_data(data)?;
    obj_ref.as_allocation_site().ok_or(Error::TypeMismatch {
      expected: "AllocationSite",
      actual: "NonAllocationSite",
    })
  }

  /// 按虚拟地址查询对象引用 (若不存在则报错)
  pub fn get_ref(&self, address: u64) -> Result<ObjectRef<'z>> {
    self
      .refs
      .get(&address)
      .copied()
      .map(ObjectRef::from_data)
      .ok_or(Error::ObjectNotFound { address })
  }

  /// 尝试按虚拟地址查询对象引用 (若不存在返回 None)
  #[inline]
  pub fn try_get_ref(&self, address: u64) -> Option<ObjectRef<'z>> {
    self.refs.get(&address).copied().map(ObjectRef::from_data)
  }

  /// 获取当前已缓存对象总数
  #[inline]
  pub fn refs_count(&self) -> usize {
    self.refs.len()
  }

  /// 是否包含指定 FeedbackSource 的反馈
  #[inline]
  pub fn has_feedback(&self, source: &FeedbackSource) -> bool {
    self.feedback.contains_key(source)
  }

  /// 查询指定 FeedbackSource 的反馈
  #[inline]
  pub fn get_feedback(&self, source: &FeedbackSource) -> Option<&ProcessedFeedbackSnapshot<'z>> {
    self.feedback.get(source)
  }

  /// 记录插槽反馈
  pub fn set_feedback(
    &mut self,
    source: FeedbackSource,
    feedback: ProcessedFeedbackSnapshot<'z>,
  ) -> Result<()> {
    if !self.serializing_allowed() {
      return Err(Error::SerializationNotAllowed(self.mode));
    }
    self.feedback.insert(source, feedback);
    Ok(())
  }
}
