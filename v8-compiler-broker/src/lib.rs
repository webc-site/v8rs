// v8-compiler-broker/src/lib.rs
//! v8-compiler-broker: V8 优化编译器堆对象快照代理系统
//!
//! 包含 `JSHeapBroker`、`HeapRef` 引用族与 `CompilationDependencies` 编译期假设依赖记录系统。

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod broker;
pub mod compilation_dependencies;
pub mod error;
pub mod heap_refs;

pub use broker::{BrokerMode, JSHeapBroker, ProcessedFeedbackSnapshot};
pub use compilation_dependencies::{CompilationDependencies, CompilationDependency};
pub use error::{Error, Result};
pub use heap_refs::{
  AccessMode, AllocationSiteData, AllocationSiteRef, DescriptorSnapshot, ElementsKind,
  HeapObjectData, HeapObjectPayload, HeapObjectRef, HeapObjectType, JSFunctionData, JSFunctionRef,
  JSObjectData, MapData, MapRef, ObjectData, ObjectPayload, ObjectRef, OddballRef, OddballType,
  PropertyCellData, PropertyCellRef, SharedFunctionInfoData, SharedFunctionInfoRef,
};
pub use v8_objects_js_core::field_index::FieldIndex;
