// 由 tool/src/builtins/builtins-collections.js 生成; 来源 v8/src/builtins 下 2 文件
// 模块索引
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

#![forbid(unsafe_code)]

mod collections;
mod error;
mod setops;

pub use collections::{
  Builtin as CollectionsBuiltin, CollectionVariant, GroupByCoercionMode,
  TQ_DECLS as COLLECTIONS_TQ_DECLS, find_ordered_hash_map_entry, find_ordered_hash_set_entry,
  map_constructor, map_iterator_prototype_next, map_iterator_to_list, map_prototype_clear,
  map_prototype_delete, map_prototype_entries, map_prototype_for_each, map_prototype_get,
  map_prototype_get_or_insert, map_prototype_get_or_insert_computed, map_prototype_get_size,
  map_prototype_has, map_prototype_keys, map_prototype_set, map_prototype_values,
  ordered_hash_table_heal_index, set_constructor, set_iterator_prototype_next,
  set_or_set_iterator_to_list, set_prototype_add, set_prototype_clear, set_prototype_delete,
  set_prototype_entries, set_prototype_for_each, set_prototype_get_size, set_prototype_has,
  set_prototype_values, weak_collection_delete, weak_collection_set, weak_map_constructor,
  weak_map_lookup_hash_index, weak_map_prototype_delete, weak_map_prototype_get,
  weak_map_prototype_get_or_insert, weak_map_prototype_get_or_insert_computed,
  weak_map_prototype_has, weak_map_prototype_set, weak_set_constructor, weak_set_prototype_add,
  weak_set_prototype_delete, weak_set_prototype_has,
};
pub use error::{Error, Result};
pub use setops::{
  Builtin as SetopsBuiltin, TQ_DECLS as SETOPS_TQ_DECLS, set_prototype_difference,
  set_prototype_intersection, set_prototype_is_disjoint_from, set_prototype_is_subset_of,
  set_prototype_is_superset_of, set_prototype_symmetric_difference, set_prototype_union,
};
