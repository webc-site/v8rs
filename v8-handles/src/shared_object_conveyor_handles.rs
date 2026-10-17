//! 由 tool/src/handles/shared-object-conveyor-handles.js
//! 从 v8/src/handles/shared-object-conveyor-handles{.h,.cc} 生成
//! PersistentHandles 的包装: ValueSerializer → ValueDeserializer 的共享对象保活通道
//! (postMessage 类 API), 每个共享对象持跨 GC 稳定 id (原类注释)。
//! Isolate 依赖推迟: 原构造从 shared_space_isolate 取 NewPersistentHandles,
//! 此处显式注入已构造容器。
use crate::{
  error::{Error, Result},
  handles::{Handle, HeapObject, Tagged},
  persistent_handles::PersistentHandles,
};

/// 共享对象传递句柄集 (原 class SharedObjectConveyorHandles)
pub struct SharedObjectConveyorHandles {
  /// 保活容器 (原 std::unique_ptr<PersistentHandles> persistent_handles_)
  persistent_handles: Box<PersistentHandles>,
  /// id → 句柄 (原 std::vector<Handle<HeapObject>> shared_objects_)
  shared_objects: Vec<Handle<HeapObject>>,
}

impl SharedObjectConveyorHandles {
  /// 注入持久容器构造 (原 SharedObjectConveyorHandles(Isolate*)):
  /// embedder 拥有实例生命周期
  pub fn new(persistent_handles: Box<PersistentHandles>) -> Self {
    Self {
      persistent_handles,
      shared_objects: Vec::new(),
    }
  }

  /// 登记共享对象并返回稳定 id (原 Persist): id 即 push 前的 size
  pub fn persist(&mut self, shared_object: Tagged<HeapObject>) -> u32 {
    let id = self.shared_objects.len() as u32;
    let handle = self.persistent_handles.new_handle(shared_object);
    self.shared_objects.push(handle);
    id
  }

  /// id 是否已登记 (原 HasPersisted)
  pub fn has_persisted(&self, object_id: u32) -> bool {
    object_id < self.shared_objects.len() as u32
  }

  /// 取回登记对象 (原 GetPersisted: DCHECK(HasPersisted) → Result)
  pub fn get_persisted(&self, object_id: u32) -> Result<Tagged<HeapObject>> {
    if !self.has_persisted(object_id) {
      return Err(Error::InvalidObjectId(object_id));
    }
    Ok(self.shared_objects[object_id as usize].value())
  }
}
