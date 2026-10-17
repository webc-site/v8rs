// 由 tool/src/builtins/builtins-array.js 生成; 来源 v8/src/builtins 下 2 文件
// 模块索引
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

#![forbid(unsafe_code)]

mod array;
mod error;
mod growable;

pub use array::{
  Builtin as ArrayBuiltin, TQ_DECLS as ARRAY_TQ_DECLS, array_concat, array_constructor,
  array_constructor_impl, array_includes, array_includes_holey_doubles,
  array_includes_packed_doubles, array_includes_smi, array_includes_smi_or_object, array_index_of,
  array_index_of_holey_doubles, array_index_of_packed_doubles, array_index_of_smi,
  array_index_of_smi_or_object, array_iterator_prototype_next, array_n_arguments_constructor,
  array_pop, array_prototype_entries, array_prototype_fill, array_prototype_keys,
  array_prototype_pop, array_prototype_push, array_prototype_values, array_push, array_shift,
  array_unshift, clone_fast_js_array, clone_fast_js_array_filling_holes,
  create_array_from_slow_boilerplate, create_array_from_slow_boilerplate_helper,
  create_object_from_slow_boilerplate, create_object_from_slow_boilerplate_helper,
  extract_fast_js_array, typed_array_prototype_map,
};
pub use error::{Error, Result};
pub use growable::{
  Builtin as GrowableBuiltin, DEFAULT_MIN_GROWTH, GrowableFixedArray, MAX_FIXED_ARRAY_CAPACITY,
  TQ_DECLS as GROWABLE_TQ_DECLS,
};
