use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_builtins_collections::{
  COLLECTIONS_TQ_DECLS, CollectionVariant, CollectionsBuiltin, Error, GroupByCoercionMode,
  SETOPS_TQ_DECLS, SetopsBuiltin, find_ordered_hash_map_entry, find_ordered_hash_set_entry,
  map_constructor, map_iterator_prototype_next, map_iterator_to_list, map_prototype_clear,
  map_prototype_delete, map_prototype_entries, map_prototype_for_each, map_prototype_get,
  map_prototype_get_or_insert, map_prototype_get_or_insert_computed, map_prototype_get_size,
  map_prototype_has, map_prototype_keys, map_prototype_set, map_prototype_values,
  ordered_hash_table_heal_index, set_constructor, set_iterator_prototype_next,
  set_or_set_iterator_to_list, set_prototype_add, set_prototype_clear, set_prototype_delete,
  set_prototype_difference, set_prototype_entries, set_prototype_for_each, set_prototype_get_size,
  set_prototype_has, set_prototype_intersection, set_prototype_is_disjoint_from,
  set_prototype_is_subset_of, set_prototype_is_superset_of, set_prototype_symmetric_difference,
  set_prototype_union, set_prototype_values, weak_collection_delete, weak_collection_set,
  weak_map_constructor, weak_map_lookup_hash_index, weak_map_prototype_delete,
  weak_map_prototype_get, weak_map_prototype_get_or_insert,
  weak_map_prototype_get_or_insert_computed, weak_map_prototype_has, weak_map_prototype_set,
  weak_set_constructor, weak_set_prototype_add, weak_set_prototype_delete, weak_set_prototype_has,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_collection_variant_and_groupby() -> Void {
  let map_v = CollectionVariant::Map;
  let set_v = CollectionVariant::Set;
  let weak_map_v = CollectionVariant::WeakMap;
  let weak_set_v = CollectionVariant::WeakSet;

  assert!(map_v.is_map());
  assert!(!map_v.is_set());
  assert!(!map_v.is_weak());

  assert!(!set_v.is_map());
  assert!(set_v.is_set());
  assert!(!set_v.is_weak());

  assert!(weak_map_v.is_map());
  assert!(!weak_map_v.is_set());
  assert!(weak_map_v.is_weak());

  assert!(!weak_set_v.is_map());
  assert!(weak_set_v.is_set());
  assert!(weak_set_v.is_weak());

  assert_eq!(GroupByCoercionMode::Zero.to_string(), "Zero");
  assert_eq!(GroupByCoercionMode::Property.to_string(), "Property");

  info!("test_collection_variant_and_groupby passed");
  OK
}

#[test]
fn test_collections_builtins() -> Void {
  let builtins: Vec<_> = CollectionsBuiltin::iter().collect();
  assert_eq!(builtins.len(), 43);

  let mut maps = 0;
  let mut sets = 0;
  let mut weaks = 0;
  let mut ctors = 0;
  let mut hash_tables = 0;

  for b in &builtins {
    let name = b.name();
    let display_str = b.to_string();
    assert_eq!(name, display_str.as_str());

    let parsed = CollectionsBuiltin::from_str(name).unwrap();
    assert_eq!(*b, parsed);

    let gid = b.global_id();
    assert!(
      gid.is_some(),
      "Builtin {} must exist in BUILTIN_TABLE",
      name
    );

    let desc = b.descriptor_name();
    assert!(desc.starts_with("Builtin_"));
    assert!(desc.ends_with("_InterfaceDescriptor"));

    if b.is_map() {
      maps += 1;
    }
    if b.is_set() {
      sets += 1;
    }
    if b.is_weak() {
      weaks += 1;
    }
    if b.is_constructor() {
      ctors += 1;
    }
    if b.is_hash_table_internal() {
      hash_tables += 1;
    }
  }

  assert_eq!(maps, 15);
  assert_eq!(sets, 11);
  assert_eq!(weaks, 14);
  assert_eq!(ctors, 4);
  assert_eq!(hash_tables, 4);

  // 全量 43 个工厂函数断言覆盖
  assert_eq!(
    find_ordered_hash_map_entry(),
    CollectionsBuiltin::FindOrderedHashMapEntry
  );
  assert_eq!(
    find_ordered_hash_set_entry(),
    CollectionsBuiltin::FindOrderedHashSetEntry
  );
  assert_eq!(map_constructor(), CollectionsBuiltin::MapConstructor);
  assert_eq!(
    map_iterator_prototype_next(),
    CollectionsBuiltin::MapIteratorPrototypeNext
  );
  assert_eq!(
    map_iterator_to_list(),
    CollectionsBuiltin::MapIteratorToList
  );
  assert_eq!(map_prototype_clear(), CollectionsBuiltin::MapPrototypeClear);
  assert_eq!(
    map_prototype_delete(),
    CollectionsBuiltin::MapPrototypeDelete
  );
  assert_eq!(
    map_prototype_entries(),
    CollectionsBuiltin::MapPrototypeEntries
  );
  assert_eq!(
    map_prototype_for_each(),
    CollectionsBuiltin::MapPrototypeForEach
  );
  assert_eq!(map_prototype_get(), CollectionsBuiltin::MapPrototypeGet);
  assert_eq!(
    map_prototype_get_or_insert(),
    CollectionsBuiltin::MapPrototypeGetOrInsert
  );
  assert_eq!(
    map_prototype_get_or_insert_computed(),
    CollectionsBuiltin::MapPrototypeGetOrInsertComputed
  );
  assert_eq!(
    map_prototype_get_size(),
    CollectionsBuiltin::MapPrototypeGetSize
  );
  assert_eq!(map_prototype_has(), CollectionsBuiltin::MapPrototypeHas);
  assert_eq!(map_prototype_keys(), CollectionsBuiltin::MapPrototypeKeys);
  assert_eq!(map_prototype_set(), CollectionsBuiltin::MapPrototypeSet);
  assert_eq!(
    map_prototype_values(),
    CollectionsBuiltin::MapPrototypeValues
  );
  assert_eq!(
    ordered_hash_table_heal_index(),
    CollectionsBuiltin::OrderedHashTableHealIndex
  );
  assert_eq!(set_constructor(), CollectionsBuiltin::SetConstructor);
  assert_eq!(
    set_iterator_prototype_next(),
    CollectionsBuiltin::SetIteratorPrototypeNext
  );
  assert_eq!(
    set_or_set_iterator_to_list(),
    CollectionsBuiltin::SetOrSetIteratorToList
  );
  assert_eq!(set_prototype_add(), CollectionsBuiltin::SetPrototypeAdd);
  assert_eq!(set_prototype_clear(), CollectionsBuiltin::SetPrototypeClear);
  assert_eq!(
    set_prototype_delete(),
    CollectionsBuiltin::SetPrototypeDelete
  );
  assert_eq!(
    set_prototype_entries(),
    CollectionsBuiltin::SetPrototypeEntries
  );
  assert_eq!(
    set_prototype_for_each(),
    CollectionsBuiltin::SetPrototypeForEach
  );
  assert_eq!(
    set_prototype_get_size(),
    CollectionsBuiltin::SetPrototypeGetSize
  );
  assert_eq!(set_prototype_has(), CollectionsBuiltin::SetPrototypeHas);
  assert_eq!(
    set_prototype_values(),
    CollectionsBuiltin::SetPrototypeValues
  );
  assert_eq!(
    weak_collection_delete(),
    CollectionsBuiltin::WeakCollectionDelete
  );
  assert_eq!(weak_collection_set(), CollectionsBuiltin::WeakCollectionSet);
  assert_eq!(
    weak_map_constructor(),
    CollectionsBuiltin::WeakMapConstructor
  );
  assert_eq!(
    weak_map_lookup_hash_index(),
    CollectionsBuiltin::WeakMapLookupHashIndex
  );
  assert_eq!(
    weak_map_prototype_delete(),
    CollectionsBuiltin::WeakMapPrototypeDelete
  );
  assert_eq!(
    weak_map_prototype_get(),
    CollectionsBuiltin::WeakMapPrototypeGet
  );
  assert_eq!(
    weak_map_prototype_get_or_insert(),
    CollectionsBuiltin::WeakMapPrototypeGetOrInsert
  );
  assert_eq!(
    weak_map_prototype_get_or_insert_computed(),
    CollectionsBuiltin::WeakMapPrototypeGetOrInsertComputed
  );
  assert_eq!(
    weak_map_prototype_has(),
    CollectionsBuiltin::WeakMapPrototypeHas
  );
  assert_eq!(
    weak_map_prototype_set(),
    CollectionsBuiltin::WeakMapPrototypeSet
  );
  assert_eq!(
    weak_set_constructor(),
    CollectionsBuiltin::WeakSetConstructor
  );
  assert_eq!(
    weak_set_prototype_add(),
    CollectionsBuiltin::WeakSetPrototypeAdd
  );
  assert_eq!(
    weak_set_prototype_delete(),
    CollectionsBuiltin::WeakSetPrototypeDelete
  );
  assert_eq!(
    weak_set_prototype_has(),
    CollectionsBuiltin::WeakSetPrototypeHas
  );

  assert_eq!(COLLECTIONS_TQ_DECLS.len(), 32);

  info!("test_collections_builtins passed");
  OK
}

#[test]
fn test_setops_builtins() -> Void {
  let builtins: Vec<_> = SetopsBuiltin::iter().collect();
  assert_eq!(builtins.len(), 7);

  let mut predicates = 0;
  let mut generators = 0;

  for b in &builtins {
    let name = b.name();
    let display_str = b.to_string();
    assert_eq!(name, display_str.as_str());

    let parsed = SetopsBuiltin::from_str(name).unwrap();
    assert_eq!(*b, parsed);

    let desc = b.descriptor_name();
    assert!(desc.starts_with("Builtin_"));
    assert!(desc.ends_with("_InterfaceDescriptor"));

    let _ = b.global_id();

    if b.is_boolean_predicate() {
      predicates += 1;
    }
    if b.is_set_generator() {
      generators += 1;
    }
  }

  assert_eq!(predicates, 3);
  assert_eq!(generators, 4);

  // 全量 7 个工厂函数断言覆盖
  assert_eq!(
    set_prototype_difference(),
    SetopsBuiltin::SetPrototypeDifference
  );
  assert_eq!(
    set_prototype_intersection(),
    SetopsBuiltin::SetPrototypeIntersection
  );
  assert_eq!(
    set_prototype_is_disjoint_from(),
    SetopsBuiltin::SetPrototypeIsDisjointFrom
  );
  assert_eq!(
    set_prototype_is_subset_of(),
    SetopsBuiltin::SetPrototypeIsSubsetOf
  );
  assert_eq!(
    set_prototype_is_superset_of(),
    SetopsBuiltin::SetPrototypeIsSupersetOf
  );
  assert_eq!(
    set_prototype_symmetric_difference(),
    SetopsBuiltin::SetPrototypeSymmetricDifference
  );
  assert_eq!(set_prototype_union(), SetopsBuiltin::SetPrototypeUnion);

  assert_eq!(SETOPS_TQ_DECLS.len(), 5);

  info!("test_setops_builtins passed");
  OK
}

#[test]
fn test_error_formatting() -> Void {
  let err1 = Error::NotAnObject;
  assert_eq!(format!("{err1}"), "Collection target is not an object");

  let err2 = Error::InvalidWeakKey;
  assert_eq!(
    format!("{err2}"),
    "Key is not an object (WeakCollection requires object keys)"
  );

  let err3 = Error::CapacityOverflow;
  assert_eq!(format!("{err3}"), "OrderedHashTable capacity overflow");

  let err4 = Error::SetOperationFailed("unsupported iterable".into());
  assert_eq!(
    format!("{err4}"),
    "Set operation failed: unsupported iterable"
  );

  info!("test_error_formatting passed");
  OK
}
