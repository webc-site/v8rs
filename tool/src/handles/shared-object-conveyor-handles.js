#!/usr/bin/env bun
// tool/src/handles/shared-object-conveyor-handles.js — shared-object-conveyor-handles.h + .cc
// → v8-handles/src/shared_object_conveyor_handles.rs
// ValueSerializer→ValueDeserializer 共享对象传递的保活包装。
import { readV8, writeSrc } from "../../lib.js";

const h = readV8("src/handles/shared-object-conveyor-handles.h");
const cc = readV8("src/handles/shared-object-conveyor-handles.cc");

// ---- 1. API 面对数 ----
const api = [
  "SharedObjectConveyorHandles(Isolate* isolate)",
  "uint32_t Persist(Tagged<HeapObject> shared_object)",
  "bool HasPersisted(uint32_t object_id) const",
  "Tagged<HeapObject> GetPersisted(uint32_t object_id) const",
];
for (const sig of api) if (!(h + cc).includes(sig.split("(")[0])) throw new Error(`SharedObjectConveyorHandles ${sig} 未命中`);

// 数据成员对数
if (!h.includes("std::unique_ptr<PersistentHandles> persistent_handles_;")) throw new Error("成员 persistent_handles_ 未命中");
if (!h.includes("std::vector<Handle<HeapObject>> shared_objects_;")) throw new Error("成员 shared_objects_ 未命中");
// Persist 语义: id 即 push 前的 size
if (!cc.includes("uint32_t id = static_cast<uint32_t>(shared_objects_.size());")) throw new Error("Persist id 语义未命中");
if (!cc.includes("return id;")) throw new Error("Persist 返回 id 未命中");

// ---- 2. 生成 Rust 初版 ----
const body = `//! 由 tool/src/handles/shared-object-conveyor-handles.js
//! 从 v8/src/handles/shared-object-conveyor-handles{.h,.cc} 生成
//! PersistentHandles 的包装: ValueSerializer → ValueDeserializer 的共享对象保活通道
//! (postMessage 类 API), 每个共享对象持跨 GC 稳定 id (原类注释)。
//! Isolate 依赖推迟: 原构造从 shared_space_isolate 取 NewPersistentHandles,
//! 此处显式注入已构造容器。
use crate::error::{Error, Result};
use crate::handles::{Handle, HeapObject, Tagged};

/// 共享对象传递句柄集 (原 class SharedObjectConveyorHandles)
pub struct SharedObjectConveyorHandles {
  /// 保活容器 (原 std::unique_ptr<PersistentHandles> persistent_handles_)
  persistent_handles: Box<crate::persistent_handles::PersistentHandles>,
  /// id → 句柄 (原 std::vector<Handle<HeapObject>> shared_objects_)
  shared_objects: Vec<Handle<HeapObject>>,
}

impl SharedObjectConveyorHandles {
  /// 注入持久容器构造 (原 SharedObjectConveyorHandles(Isolate*)):
  /// embedder 拥有实例生命周期
  pub fn new(persistent_handles: Box<crate::persistent_handles::PersistentHandles>) -> Self {
    Self { persistent_handles, shared_objects: Vec::new() }
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
`;

const p = writeSrc("v8-handles", "shared_object_conveyor_handles.rs", body);
console.log(`✔ v8-handles/src/shared_object_conveyor_handles.rs (${body.split("\n").length} 行)`);
console.log(`  API 对数: ${api.length}/4 面命中, 数据成员 2/2 命中, Persist id 语义命中`);
