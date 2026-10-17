// 由 tool/src/builtins/builtins-typedarray.js 生成; 来源 v8/src/builtins 下 28 文件
// 模块索引
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

#![forbid(unsafe_code)]

mod buffer;
mod error;
mod typed_array;

pub use buffer::{
  Builtin as BufferBuiltin, DataViewOp, TQ_DECLS as BUFFER_TQ_DECLS, array_buffer_constructor,
  array_buffer_constructor_do_not_initialize, array_buffer_prototype_resize,
  array_buffer_prototype_slice, array_buffer_prototype_slice_to_immutable,
  array_buffer_prototype_transfer, array_buffer_prototype_transfer_to_fixed_length,
  array_buffer_prototype_transfer_to_immutable, atomics_compare_exchange, atomics_exchange,
  atomics_is_lock_free, atomics_load, atomics_notify, atomics_pause, atomics_store, atomics_wait,
  atomics_wait_async, data_view_constructor, shared_array_buffer_prototype_get_byte_length,
  shared_array_buffer_prototype_grow, shared_array_buffer_prototype_slice,
  shared_array_constructor, shared_array_is_shared_array,
};
pub use error::{Error, Result};
pub use typed_array::{
  Builtin as TypedArrayBuiltin, TQ_DECLS as TYPED_ARRAY_TQ_DECLS, TypedArrayElementsKind,
  typed_array_base_constructor, typed_array_constructor, typed_array_prototype_buffer,
  typed_array_prototype_byte_length, typed_array_prototype_byte_offset,
  typed_array_prototype_copy_within, typed_array_prototype_fill, typed_array_prototype_includes,
  typed_array_prototype_index_of, typed_array_prototype_last_index_of,
  typed_array_prototype_length, typed_array_prototype_reverse, typed_array_prototype_to_string_tag,
  uint8_array_from_base64, uint8_array_from_hex, uint8_array_prototype_set_from_base64,
  uint8_array_prototype_set_from_hex, uint8_array_prototype_to_base64,
  uint8_array_prototype_to_hex,
};
