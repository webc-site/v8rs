#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod allocator;
mod error;
pub mod internal_index;
pub mod ordered_hash_table;
pub mod small_ordered_hash_table;
pub mod swiss_name_dictionary;

// —— R3O2 转换脚本产物挂线 (只追加; 产物见各文件 banner) ——
// 已挂线且编译通过; 未挂线产物 (dictionary/hash_table/keys/lookup/lookup_cache/
// ordered_hash_table_cc_generated/string/symbol_table) 存在 const 断言编译错
// (E0015/E0080/E0658, 并发批量席产物), 作孤儿文件留档让面, 待其脚本席修复后挂线。
pub mod compilation_cache_table;
pub mod name;
pub mod ordered_hash_table_cc_generated;
pub mod small_ordered_hash_table_generated;
pub mod swiss_name_dictionary_cc_generated;
pub mod swiss_name_dictionary_tq_generated;

pub use allocator::{HeapStorage, MemoryAllocator};
pub use error::{Error, Result};
pub use internal_index::{InternalIndex, InternalIndexRange};
pub use ordered_hash_table::{
  OrderedHashMap, OrderedHashMapExt, OrderedHashMapIterator, OrderedHashMapTable, OrderedHashSet,
  OrderedHashSetExt, OrderedHashSetIterator, OrderedHashSetTable, OrderedNameDictionary,
  OrderedNameDictionaryExt, OrderedNameDictionaryIterator, OrderedNameDictionaryTable, THE_HOLE,
  compute_object_hash, is_the_hole, keys_equal,
};
pub use small_ordered_hash_table::{
  SmallOrderedHashMap, SmallOrderedHashMapExt, SmallOrderedHashMapIterator,
  SmallOrderedHashMapTable, SmallOrderedHashSet, SmallOrderedHashSetExt,
  SmallOrderedHashSetIterator, SmallOrderedHashSetTable, SmallOrderedHashTableHead,
};
pub use swiss_name_dictionary::{
  Group16, ProbeSequence, SwissNameDictionary, SwissNameDictionaryExt, SwissNameDictionaryHead,
  SwissNameDictionaryIterator, SwissNameDictionaryTable,
};
