// v8-compiler-broker/src/compilation_dependencies.rs
// 对应原版 v8/src/compiler/compilation-dependencies.h 与 compilation-dependencies.cc
//! 编译期假设依赖记录系统 (CompilationDependencies)
//!
//! 在优化编译（Turbofan / Maglev）生成代码期间，编译器做出各种乐观假设
//! （例如：Map 保持稳定未改变、函数的初始原型未被篡改、字段仍然是只读常量）。
//! 若运行期这些假设被打破，已生成的机器代码将被反优化（Deoptimize）。

use gxhash::HashSet;
use v8_objects_core::property_details::PropertyConstness;

use crate::{
  broker::JSHeapBroker,
  error::{Error, Result},
  heap_refs::{ElementsKind, HeapObjectRef, JSFunctionRef, MapRef},
};

/// 编译期独立假设依赖条目
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompilationDependency<'z> {
  /// 函数 initial_map 保持不变依赖
  InitialMap {
    /// 函数地址
    function_address: u64,
    /// 预期的 initial_map 地址
    initial_map_address: u64,
  },
  /// Map 保持稳定 (Stable) 未发生多态转移或结构演化
  StableMap {
    /// Map 地址
    map_address: u64,
  },
  /// 函数原型对象未被替换依赖
  PrototypeProperty {
    /// 函数地址
    function_address: u64,
    /// 原型对象地址
    prototype_address: u64,
  },
  /// 属性字段常量性 (Constness) 假设
  FieldConstness {
    /// 目标 Map 地址
    map_address: u64,
    /// 描述符索引
    descriptor_index: usize,
    /// 预期的常量性
    expected_constness: PropertyConstness,
  },
  /// 全局属性单元 PropertyCell 状态假设
  GlobalProperty {
    /// 单元地址
    cell_address: u64,
    /// 是否为只读常量
    is_read_only: bool,
  },
  /// 语言保护器 (Protector) 有效性假设
  Protector {
    /// 保护器唯一编号
    protector_id: u32,
    /// 预期的有效性状态
    expected_valid: bool,
  },
  /// Slack Tracking 未发生状态改变假设
  NoSlackTrackingChange {
    /// Map 地址
    map_address: u64,
    /// 预期的构造计数器数值
    expected_construction_counter: u32,
  },
  /// 分配点 (AllocationSite) 元素存储格式假设
  ElementsKind {
    /// 分配点地址
    site_address: u64,
    /// 元素存储格式
    elements_kind: ElementsKind,
  },
  /// 自定义扩展依赖
  Custom {
    /// 描述符
    tag: &'z str,
    /// 键
    key: u64,
    /// 预期值
    expected: u64,
  },
}

/// 编译依赖收集器 (CompilationDependencies)
#[derive(Debug, Clone, Default)]
pub struct CompilationDependencies<'z> {
  /// 已去重的依赖集合 (未提交态)
  dependencies: HashSet<CompilationDependency<'z>>,
  /// 固化后的依赖快照 (提交后保存)
  committed_dependencies: Option<Vec<CompilationDependency<'z>>>,
  /// 事务状态: 是否已提交固化
  is_committed: bool,
}

impl<'z> CompilationDependencies<'z> {
  /// 创建空的依赖收集器
  pub fn new() -> Self {
    Self {
      dependencies: HashSet::default(),
      committed_dependencies: None,
      is_committed: false,
    }
  }

  /// 内部记录依赖，若已固化提交则阻断写入
  pub fn record_dependency(&mut self, dep: CompilationDependency<'z>) -> Result<()> {
    if self.is_committed {
      return Err(Error::DependenciesAlreadyCommitted);
    }
    self.dependencies.insert(dep);
    Ok(())
  }

  /// 记录函数初始 Map 依赖，并返回该 MapRef
  pub fn depend_on_initial_map(
    &mut self,
    function: JSFunctionRef<'z>,
    initial_map: MapRef<'z>,
  ) -> Result<MapRef<'z>> {
    self.record_dependency(CompilationDependency::InitialMap {
      function_address: function.address(),
      initial_map_address: initial_map.address(),
    })?;
    Ok(initial_map)
  }

  /// 记录 Map 保持稳定 (Stable) 假设
  pub fn depend_on_stable_map(&mut self, map: MapRef<'z>) -> Result<()> {
    self.record_dependency(CompilationDependency::StableMap {
      map_address: map.address(),
    })
  }

  /// 记录函数原型属性未被篡改假设
  pub fn depend_on_prototype_property(
    &mut self,
    function: JSFunctionRef<'z>,
    prototype: HeapObjectRef<'z>,
  ) -> Result<HeapObjectRef<'z>> {
    self.record_dependency(CompilationDependency::PrototypeProperty {
      function_address: function.address(),
      prototype_address: prototype.address(),
    })?;
    Ok(prototype)
  }

  /// 记录字段常量性假设
  pub fn depend_on_field_constness(
    &mut self,
    map: MapRef<'z>,
    descriptor_index: usize,
    expected_constness: PropertyConstness,
  ) -> Result<()> {
    self.record_dependency(CompilationDependency::FieldConstness {
      map_address: map.address(),
      descriptor_index,
      expected_constness,
    })
  }

  /// 记录全局属性单元有效性假设
  pub fn depend_on_global_property(&mut self, cell_address: u64, is_read_only: bool) -> Result<()> {
    self.record_dependency(CompilationDependency::GlobalProperty {
      cell_address,
      is_read_only,
    })
  }

  /// 记录保护器有效性假设
  pub fn depend_on_protector(&mut self, protector_id: u32, is_valid: bool) -> Result<bool> {
    if is_valid {
      self.record_dependency(CompilationDependency::Protector {
        protector_id,
        expected_valid: true,
      })?;
    }
    Ok(is_valid)
  }

  /// 记录 Slack Tracking 不变假设
  pub fn depend_on_no_slack_tracking_change(&mut self, map: MapRef<'z>) -> Result<()> {
    self.record_dependency(CompilationDependency::NoSlackTrackingChange {
      map_address: map.address(),
      expected_construction_counter: map.construction_counter(),
    })
  }

  /// 记录分配点元素类型不变假设
  pub fn depend_on_elements_kind(
    &mut self,
    site_address: u64,
    elements_kind: ElementsKind,
  ) -> Result<()> {
    self.record_dependency(CompilationDependency::ElementsKind {
      site_address,
      elements_kind,
    })
  }

  /// 记录自定义扩展假设依赖
  pub fn depend_on_custom(&mut self, tag: &'z str, key: u64, expected: u64) -> Result<()> {
    self.record_dependency(CompilationDependency::Custom { tag, key, expected })
  }

  /// 获取当前已记录的依赖条目数量
  #[inline]
  pub fn count(&self) -> usize {
    if self.is_committed {
      self.committed_dependencies.as_ref().map_or(0, |d| d.len())
    } else {
      self.dependencies.len()
    }
  }

  /// 当前是否未记录任何依赖
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.count() == 0
  }

  /// 是否包含指定依赖
  #[inline]
  pub fn contains(&self, dependency: &CompilationDependency<'z>) -> bool {
    if self.is_committed {
      self
        .committed_dependencies
        .as_ref()
        .is_some_and(|d| d.contains(dependency))
    } else {
      self.dependencies.contains(dependency)
    }
  }

  /// 依赖是否已提交固化
  #[inline]
  pub const fn is_committed(&self) -> bool {
    self.is_committed
  }

  /// 获取已固化的依赖列表（若已提交）
  #[inline]
  pub fn committed_dependencies(&self) -> Option<&[CompilationDependency<'z>]> {
    self.committed_dependencies.as_deref()
  }

  /// 校验所有已记录的编译期依赖是否对当前 Broker 快照仍然有效
  pub fn validate<'a>(&self, broker: &JSHeapBroker<'z, 'a>) -> Result<bool> {
    let check_dep = |dep: &CompilationDependency<'z>| -> Result<()> {
      match dep {
        CompilationDependency::InitialMap {
          function_address,
          initial_map_address,
        } => {
          let func_obj = broker.get_ref(*function_address)?;
          let func = func_obj.as_js_function().ok_or(Error::TypeMismatch {
            expected: "JSFunction",
            actual: "NonFunction",
          })?;
          let current_map = func.initial_map(broker)?;
          if current_map.address() != *initial_map_address {
            return Err(Error::DependencyValidationFailed {
              reason: format!(
                "InitialMap changed: expected {initial_map_address:#x}, got {:#x}",
                current_map.address()
              ),
            });
          }
        }
        CompilationDependency::StableMap { map_address } => {
          let map_obj = broker.get_ref(*map_address)?;
          let map = map_obj.as_map().ok_or(Error::TypeMismatch {
            expected: "Map",
            actual: "NonMap",
          })?;
          if !map.is_stable() {
            return Err(Error::DependencyValidationFailed {
              reason: format!("Map {map_address:#x} is no longer stable"),
            });
          }
          if map.is_deprecated() {
            return Err(Error::DeprecatedMap {
              address: *map_address,
            });
          }
        }
        CompilationDependency::PrototypeProperty {
          function_address,
          prototype_address,
        } => {
          let func_obj = broker.get_ref(*function_address)?;
          let func = func_obj.as_js_function().ok_or(Error::TypeMismatch {
            expected: "JSFunction",
            actual: "NonFunction",
          })?;
          let current_proto = func.instance_prototype(broker)?;
          if current_proto.address() != *prototype_address {
            return Err(Error::DependencyValidationFailed {
              reason: format!(
                "Prototype changed: expected {prototype_address:#x}, got {:#x}",
                current_proto.address()
              ),
            });
          }
        }
        CompilationDependency::FieldConstness {
          map_address,
          descriptor_index,
          expected_constness,
        } => {
          let map_obj = broker.get_ref(*map_address)?;
          let map = map_obj.as_map().ok_or(Error::TypeMismatch {
            expected: "Map",
            actual: "NonMap",
          })?;
          let details = map.get_property_details(*descriptor_index).ok_or(
            Error::DependencyValidationFailed {
              reason: format!(
                "Descriptor index {descriptor_index} not found on map {map_address:#x}"
              ),
            },
          )?;
          if details.constness() != *expected_constness {
            return Err(Error::DependencyValidationFailed {
              reason: format!(
                "Field constness changed: expected {expected_constness:?}, got {:?}",
                details.constness()
              ),
            });
          }
        }
        CompilationDependency::GlobalProperty {
          cell_address,
          is_read_only,
        } => {
          let cell_obj = broker.get_ref(*cell_address)?;
          let cell = cell_obj.as_property_cell().ok_or(Error::TypeMismatch {
            expected: "PropertyCell",
            actual: "NonPropertyCell",
          })?;
          if cell.property_details().is_read_only() != *is_read_only {
            return Err(Error::DependencyValidationFailed {
              reason: format!(
                "GlobalProperty cell {cell_address:#x} read_only changed: expected {is_read_only}, got {}",
                cell.property_details().is_read_only()
              ),
            });
          }
        }
        CompilationDependency::Protector {
          protector_id,
          expected_valid,
        } => {
          let current_valid = broker.is_protector_valid(*protector_id);
          if current_valid != *expected_valid {
            return Err(Error::DependencyValidationFailed {
              reason: format!(
                "Protector {protector_id} validity changed: expected {expected_valid}, got {current_valid}"
              ),
            });
          }
        }
        CompilationDependency::NoSlackTrackingChange {
          map_address,
          expected_construction_counter,
        } => {
          let map_obj = broker.get_ref(*map_address)?;
          let map = map_obj.as_map().ok_or(Error::TypeMismatch {
            expected: "Map",
            actual: "NonMap",
          })?;
          if map.construction_counter() != *expected_construction_counter {
            return Err(Error::DependencyValidationFailed {
              reason: format!(
                "Slack tracking construction counter changed for map {map_address:#x}: expected {expected_construction_counter}, got {}",
                map.construction_counter()
              ),
            });
          }
        }
        CompilationDependency::ElementsKind {
          site_address,
          elements_kind,
        } => {
          let obj = broker.get_ref(*site_address)?;
          let actual_kind = if let Some(site) = obj.as_allocation_site() {
            site.elements_kind()
          } else if let Some(map) = obj.as_map() {
            map.elements_kind()
          } else {
            return Err(Error::TypeMismatch {
              expected: "AllocationSite or Map",
              actual: "NonAllocationSiteOrMap",
            });
          };
          if actual_kind != *elements_kind {
            return Err(Error::DependencyValidationFailed {
              reason: format!(
                "ElementsKind dependency mismatch for {site_address:#x}: expected {elements_kind:?}, got {actual_kind:?}"
              ),
            });
          }
        }
        CompilationDependency::Custom { tag, key, expected } => {
          if let Some(val) = broker.get_custom_data(tag, *key)
            && val != *expected
          {
            return Err(Error::DependencyValidationFailed {
              reason: format!(
                "Custom dependency ({tag}, {key}) mismatch: expected {expected}, got {val}"
              ),
            });
          }
        }
      }
      Ok(())
    };

    if self.is_committed {
      if let Some(committed) = &self.committed_dependencies {
        for dep in committed {
          check_dep(dep)?;
        }
      }
    } else {
      for dep in &self.dependencies {
        check_dep(dep)?;
      }
    }
    Ok(true)
  }

  /// 提交依赖 (验证通过后正式固化，清空未提交集合，阻断重复提交与后续写入)
  pub fn commit<'a>(&mut self, broker: &JSHeapBroker<'z, 'a>) -> Result<bool> {
    if self.is_committed {
      return Err(Error::DependenciesAlreadyCommitted);
    }
    self.validate(broker)?;
    let deps: Vec<_> = self.dependencies.drain().collect();
    self.committed_dependencies = Some(deps);
    self.is_committed = true;
    Ok(true)
  }
}
