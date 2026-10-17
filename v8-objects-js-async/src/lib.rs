// v8-objects-js-async/src/lib.rs
// JSPromise 与 JSGeneratorObject 核心异步对象族统一导出

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;

// ── objects 文件族语义面 (W6O4 收编挂载; 推迟台账见 readme/zh.md) ──
pub mod allocation_site_scopes;
pub mod allocation_site_tq;
pub mod backing_store_cc;
pub mod bigint_tq;
pub mod cpp_heap_external_object;
pub mod cpp_heap_object_wrapper;
pub mod hash_seed_wrapper;
pub mod heap_number;
pub mod intl_objects;
pub mod js_generator;
pub mod js_promise;
pub mod managed;
pub mod microtask;
pub mod oddball;
pub mod option_utils;
pub mod promise;
pub mod regexp_match_info;
pub mod simd;
pub mod value_serializer_skel;
pub mod waiter_queue_node;

pub use backing_store_cc::{BackingStore, BackingStoreInit};
pub use error::{Error, Result};
pub use js_generator::{
  AsyncGeneratorRequest, AsyncGeneratorRequestBuf, AsyncGeneratorRequestExt, GeneratorFields,
  JSAsyncFunctionExt, JSAsyncFunctionObject, JSAsyncFunctionObjectBuf, JSAsyncGeneratorExt,
  JSAsyncGeneratorObject, JSAsyncGeneratorObjectBuf, JSGeneratorExt, JSGeneratorObject,
  JSGeneratorObjectBuf, K_ASYNC_FUNC_AWAIT_REJECT_CLOSURE_OFFSET,
  K_ASYNC_FUNC_AWAIT_RESOLVE_CLOSURE_OFFSET, K_ASYNC_FUNC_PROMISE_OFFSET,
  K_ASYNC_FUNCTION_HEADER_SIZE, K_ASYNC_GEN_IS_AWAITING_OFFSET, K_ASYNC_GEN_QUEUE_OFFSET,
  K_ASYNC_GENERATOR_HEADER_SIZE, K_ASYNC_GENERATOR_REQUEST_HEADER_SIZE, K_CONTEXT_OFFSET,
  K_CONTINUATION_OFFSET, K_FUNCTION_OFFSET, K_GENERATOR_CLOSED, K_GENERATOR_EXECUTING,
  K_GENERATOR_HEADER_SIZE, K_INPUT_OR_DEBUG_POS_OFFSET, K_MAX_ASYNC_FUNCTION_IN_OBJECT_PROPERTIES,
  K_MAX_ASYNC_GENERATOR_IN_OBJECT_PROPERTIES, K_MAX_GENERATOR_IN_OBJECT_PROPERTIES,
  K_PARAMETERS_AND_REGISTERS_OFFSET, K_RECEIVER_OFFSET, K_REQ_NEXT_OFFSET, K_REQ_PROMISE_OFFSET,
  K_REQ_RESUME_MODE_OFFSET, K_REQ_VALUE_OFFSET, K_RESUME_MODE_OFFSET, K_YIELDED_VALUE_OFFSET,
  ResumeMode, TaggedJSGeneratorExt,
};
pub use js_promise::{
  JSPromise, JSPromiseBuf, JSPromiseExt, K_ASYNC_TASK_ID_BITS, K_ASYNC_TASK_ID_MASK,
  K_ASYNC_TASK_ID_SHIFT, K_FLAGS_OFFSET, K_HAS_HANDLER_BITS, K_HAS_HANDLER_MASK,
  K_HAS_HANDLER_SHIFT, K_HEADER_SIZE as K_JS_PROMISE_HEADER_SIZE, K_INVALID_ASYNC_TASK_ID,
  K_IS_NATIVE_RESOLVER_INVOKED_BITS, K_IS_NATIVE_RESOLVER_INVOKED_MASK,
  K_IS_NATIVE_RESOLVER_INVOKED_SHIFT, K_IS_SILENT_BITS, K_IS_SILENT_MASK, K_IS_SILENT_SHIFT,
  K_MAX_ASYNC_TASK_ID, K_MAX_IN_OBJECT_PROPERTIES as K_MAX_PROMISE_IN_OBJECT_PROPERTIES,
  K_REACTIONS_OR_RESULT_OFFSET, K_STATUS_BITS, K_STATUS_MASK, K_STATUS_SHIFT, PromiseState,
  TaggedJSPromiseExt, get_next_async_task_id,
};
