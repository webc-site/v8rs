// 由 tool/src/builtins/builtins-collections.js 生成; 来源 v8/src/builtins 下 7 文件
// 内建名表 + 核心调度元数据 (43 内建)
// 来源: builtins-collections.cc, builtins-collections-gen.cc, builtins-collections-gen.h, collections.tq 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// 集合变体枚举 (对应 v8/src/builtins/builtins-collections-gen.h: CollectionsBuiltinsAssembler::Variant)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum CollectionVariant {
  Map,
  Set,
  WeakMap,
  WeakSet,
}

impl CollectionVariant {
  #[inline]
  pub const fn is_map(&self) -> bool {
    matches!(self, Self::Map | Self::WeakMap)
  }

  #[inline]
  pub const fn is_set(&self) -> bool {
    matches!(self, Self::Set | Self::WeakSet)
  }

  #[inline]
  pub const fn is_weak(&self) -> bool {
    matches!(self, Self::WeakMap | Self::WeakSet)
  }
}

/// 分组强制转换模式 (对应 v8/src/builtins/builtins-collections-gen.h: GroupByCoercionMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum GroupByCoercionMode {
  Zero,
  Property,
}

/// Map / Set / WeakMap / WeakSet 核心内建名表 (43 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  FindOrderedHashMapEntry,
  FindOrderedHashSetEntry,
  MapConstructor,
  MapIteratorPrototypeNext,
  MapIteratorToList,
  MapPrototypeClear,
  MapPrototypeDelete,
  MapPrototypeEntries,
  MapPrototypeForEach,
  MapPrototypeGet,
  MapPrototypeGetOrInsert,
  MapPrototypeGetOrInsertComputed,
  MapPrototypeGetSize,
  MapPrototypeHas,
  MapPrototypeKeys,
  MapPrototypeSet,
  MapPrototypeValues,
  OrderedHashTableHealIndex,
  SetConstructor,
  SetIteratorPrototypeNext,
  SetOrSetIteratorToList,
  SetPrototypeAdd,
  SetPrototypeClear,
  SetPrototypeDelete,
  SetPrototypeEntries,
  SetPrototypeForEach,
  SetPrototypeGetSize,
  SetPrototypeHas,
  SetPrototypeValues,
  WeakCollectionDelete,
  WeakCollectionSet,
  WeakMapConstructor,
  WeakMapLookupHashIndex,
  WeakMapPrototypeDelete,
  WeakMapPrototypeGet,
  WeakMapPrototypeGetOrInsert,
  WeakMapPrototypeGetOrInsertComputed,
  WeakMapPrototypeHas,
  WeakMapPrototypeSet,
  WeakSetConstructor,
  WeakSetPrototypeAdd,
  WeakSetPrototypeDelete,
  WeakSetPrototypeHas,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::FindOrderedHashMapEntry => "FindOrderedHashMapEntry",
      Self::FindOrderedHashSetEntry => "FindOrderedHashSetEntry",
      Self::MapConstructor => "MapConstructor",
      Self::MapIteratorPrototypeNext => "MapIteratorPrototypeNext",
      Self::MapIteratorToList => "MapIteratorToList",
      Self::MapPrototypeClear => "MapPrototypeClear",
      Self::MapPrototypeDelete => "MapPrototypeDelete",
      Self::MapPrototypeEntries => "MapPrototypeEntries",
      Self::MapPrototypeForEach => "MapPrototypeForEach",
      Self::MapPrototypeGet => "MapPrototypeGet",
      Self::MapPrototypeGetOrInsert => "MapPrototypeGetOrInsert",
      Self::MapPrototypeGetOrInsertComputed => "MapPrototypeGetOrInsertComputed",
      Self::MapPrototypeGetSize => "MapPrototypeGetSize",
      Self::MapPrototypeHas => "MapPrototypeHas",
      Self::MapPrototypeKeys => "MapPrototypeKeys",
      Self::MapPrototypeSet => "MapPrototypeSet",
      Self::MapPrototypeValues => "MapPrototypeValues",
      Self::OrderedHashTableHealIndex => "OrderedHashTableHealIndex",
      Self::SetConstructor => "SetConstructor",
      Self::SetIteratorPrototypeNext => "SetIteratorPrototypeNext",
      Self::SetOrSetIteratorToList => "SetOrSetIteratorToList",
      Self::SetPrototypeAdd => "SetPrototypeAdd",
      Self::SetPrototypeClear => "SetPrototypeClear",
      Self::SetPrototypeDelete => "SetPrototypeDelete",
      Self::SetPrototypeEntries => "SetPrototypeEntries",
      Self::SetPrototypeForEach => "SetPrototypeForEach",
      Self::SetPrototypeGetSize => "SetPrototypeGetSize",
      Self::SetPrototypeHas => "SetPrototypeHas",
      Self::SetPrototypeValues => "SetPrototypeValues",
      Self::WeakCollectionDelete => "WeakCollectionDelete",
      Self::WeakCollectionSet => "WeakCollectionSet",
      Self::WeakMapConstructor => "WeakMapConstructor",
      Self::WeakMapLookupHashIndex => "WeakMapLookupHashIndex",
      Self::WeakMapPrototypeDelete => "WeakMapPrototypeDelete",
      Self::WeakMapPrototypeGet => "WeakMapPrototypeGet",
      Self::WeakMapPrototypeGetOrInsert => "WeakMapPrototypeGetOrInsert",
      Self::WeakMapPrototypeGetOrInsertComputed => "WeakMapPrototypeGetOrInsertComputed",
      Self::WeakMapPrototypeHas => "WeakMapPrototypeHas",
      Self::WeakMapPrototypeSet => "WeakMapPrototypeSet",
      Self::WeakSetConstructor => "WeakSetConstructor",
      Self::WeakSetPrototypeAdd => "WeakSetPrototypeAdd",
      Self::WeakSetPrototypeDelete => "WeakSetPrototypeDelete",
      Self::WeakSetPrototypeHas => "WeakSetPrototypeHas",
    }
  }

  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取对应的接口描述符符号
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 是否为 Map 相关内建
  #[inline]
  pub const fn is_map(&self) -> bool {
    matches!(
      self,
      Self::MapConstructor
        | Self::MapIteratorPrototypeNext
        | Self::MapIteratorToList
        | Self::MapPrototypeClear
        | Self::MapPrototypeDelete
        | Self::MapPrototypeEntries
        | Self::MapPrototypeForEach
        | Self::MapPrototypeGet
        | Self::MapPrototypeGetOrInsert
        | Self::MapPrototypeGetOrInsertComputed
        | Self::MapPrototypeGetSize
        | Self::MapPrototypeHas
        | Self::MapPrototypeKeys
        | Self::MapPrototypeSet
        | Self::MapPrototypeValues
    )
  }

  /// 是否为 Set 相关内建
  #[inline]
  pub const fn is_set(&self) -> bool {
    matches!(
      self,
      Self::SetConstructor
        | Self::SetIteratorPrototypeNext
        | Self::SetOrSetIteratorToList
        | Self::SetPrototypeAdd
        | Self::SetPrototypeClear
        | Self::SetPrototypeDelete
        | Self::SetPrototypeEntries
        | Self::SetPrototypeForEach
        | Self::SetPrototypeGetSize
        | Self::SetPrototypeHas
        | Self::SetPrototypeValues
    )
  }

  /// 是否为 Weak 弱引用集合相关内建
  #[inline]
  pub const fn is_weak(&self) -> bool {
    matches!(
      self,
      Self::WeakCollectionDelete
        | Self::WeakCollectionSet
        | Self::WeakMapConstructor
        | Self::WeakMapLookupHashIndex
        | Self::WeakMapPrototypeDelete
        | Self::WeakMapPrototypeGet
        | Self::WeakMapPrototypeGetOrInsert
        | Self::WeakMapPrototypeGetOrInsertComputed
        | Self::WeakMapPrototypeHas
        | Self::WeakMapPrototypeSet
        | Self::WeakSetConstructor
        | Self::WeakSetPrototypeAdd
        | Self::WeakSetPrototypeDelete
        | Self::WeakSetPrototypeHas
    )
  }

  /// 是否为构造函数内建
  #[inline]
  pub const fn is_constructor(&self) -> bool {
    matches!(
      self,
      Self::MapConstructor
        | Self::SetConstructor
        | Self::WeakMapConstructor
        | Self::WeakSetConstructor
    )
  }

  /// 是否为底层哈希表维护内建
  #[inline]
  pub const fn is_hash_table_internal(&self) -> bool {
    matches!(
      self,
      Self::FindOrderedHashMapEntry
        | Self::FindOrderedHashSetEntry
        | Self::OrderedHashTableHealIndex
        | Self::WeakMapLookupHashIndex
    )
  }
}

// 统一提供 43 项内建工厂函数
pub const fn find_ordered_hash_map_entry() -> Builtin {
  Builtin::FindOrderedHashMapEntry
}

pub const fn find_ordered_hash_set_entry() -> Builtin {
  Builtin::FindOrderedHashSetEntry
}

pub const fn map_constructor() -> Builtin {
  Builtin::MapConstructor
}

pub const fn map_iterator_prototype_next() -> Builtin {
  Builtin::MapIteratorPrototypeNext
}

pub const fn map_iterator_to_list() -> Builtin {
  Builtin::MapIteratorToList
}

pub const fn map_prototype_clear() -> Builtin {
  Builtin::MapPrototypeClear
}

pub const fn map_prototype_delete() -> Builtin {
  Builtin::MapPrototypeDelete
}

pub const fn map_prototype_entries() -> Builtin {
  Builtin::MapPrototypeEntries
}

pub const fn map_prototype_for_each() -> Builtin {
  Builtin::MapPrototypeForEach
}

pub const fn map_prototype_get() -> Builtin {
  Builtin::MapPrototypeGet
}

pub const fn map_prototype_get_or_insert() -> Builtin {
  Builtin::MapPrototypeGetOrInsert
}

pub const fn map_prototype_get_or_insert_computed() -> Builtin {
  Builtin::MapPrototypeGetOrInsertComputed
}

pub const fn map_prototype_get_size() -> Builtin {
  Builtin::MapPrototypeGetSize
}

pub const fn map_prototype_has() -> Builtin {
  Builtin::MapPrototypeHas
}

pub const fn map_prototype_keys() -> Builtin {
  Builtin::MapPrototypeKeys
}

pub const fn map_prototype_set() -> Builtin {
  Builtin::MapPrototypeSet
}

pub const fn map_prototype_values() -> Builtin {
  Builtin::MapPrototypeValues
}

pub const fn ordered_hash_table_heal_index() -> Builtin {
  Builtin::OrderedHashTableHealIndex
}

pub const fn set_constructor() -> Builtin {
  Builtin::SetConstructor
}

pub const fn set_iterator_prototype_next() -> Builtin {
  Builtin::SetIteratorPrototypeNext
}

pub const fn set_or_set_iterator_to_list() -> Builtin {
  Builtin::SetOrSetIteratorToList
}

pub const fn set_prototype_add() -> Builtin {
  Builtin::SetPrototypeAdd
}

pub const fn set_prototype_clear() -> Builtin {
  Builtin::SetPrototypeClear
}

pub const fn set_prototype_delete() -> Builtin {
  Builtin::SetPrototypeDelete
}

pub const fn set_prototype_entries() -> Builtin {
  Builtin::SetPrototypeEntries
}

pub const fn set_prototype_for_each() -> Builtin {
  Builtin::SetPrototypeForEach
}

pub const fn set_prototype_get_size() -> Builtin {
  Builtin::SetPrototypeGetSize
}

pub const fn set_prototype_has() -> Builtin {
  Builtin::SetPrototypeHas
}

pub const fn set_prototype_values() -> Builtin {
  Builtin::SetPrototypeValues
}

pub const fn weak_collection_delete() -> Builtin {
  Builtin::WeakCollectionDelete
}

pub const fn weak_collection_set() -> Builtin {
  Builtin::WeakCollectionSet
}

pub const fn weak_map_constructor() -> Builtin {
  Builtin::WeakMapConstructor
}

pub const fn weak_map_lookup_hash_index() -> Builtin {
  Builtin::WeakMapLookupHashIndex
}

pub const fn weak_map_prototype_delete() -> Builtin {
  Builtin::WeakMapPrototypeDelete
}

pub const fn weak_map_prototype_get() -> Builtin {
  Builtin::WeakMapPrototypeGet
}

pub const fn weak_map_prototype_get_or_insert() -> Builtin {
  Builtin::WeakMapPrototypeGetOrInsert
}

pub const fn weak_map_prototype_get_or_insert_computed() -> Builtin {
  Builtin::WeakMapPrototypeGetOrInsertComputed
}

pub const fn weak_map_prototype_has() -> Builtin {
  Builtin::WeakMapPrototypeHas
}

pub const fn weak_map_prototype_set() -> Builtin {
  Builtin::WeakMapPrototypeSet
}

pub const fn weak_set_constructor() -> Builtin {
  Builtin::WeakSetConstructor
}

pub const fn weak_set_prototype_add() -> Builtin {
  Builtin::WeakSetPrototypeAdd
}

pub const fn weak_set_prototype_delete() -> Builtin {
  Builtin::WeakSetPrototypeDelete
}

pub const fn weak_set_prototype_has() -> Builtin {
  Builtin::WeakSetPrototypeHas
}

/// 来自 `collections.tq` 等的声明清单 (共 32 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("struct", "SetRecord"),
  ("extern macro", "CodeStubAssembler"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("struct", "KeyIndexPair"),
  ("struct", "OrderedHashSetIndexPair"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("struct", "UnmodifiedOrderedHashSetIterator"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("struct", "OrderedHashSetIterator"),
  ("macro", "LoadOrderedHashTableMetadata"),
  ("macro", "NewUnmodifiedOrderedHashSetIterator"),
  ("macro", "NewOrderedHashSetIterator"),
  ("struct", "KeyValueIndexTuple"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("struct", "UnmodifiedOrderedHashMapIterator"),
  ("macro", "NewUnmodifiedOrderedHashMapIterator"),
  ("macro", "LoadKeyValuePairNoSideEffects"),
  ("macro", "CheckSetRecordHasJSSetMethods"),
  ("macro", "CheckSetRecordHasJSMapMethods"),
  ("macro", "ShrinkOrderedHashSetIfNeeded"),
  ("struct", "StableJSSetBackingTableWitness"),
  ("struct", "StableJSMapBackingTableWitness"),
  ("macro", "NewStableBackingTableWitness"),
  ("macro", "NewStableBackingTableWitness"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
  ("extern macro", "CollectionsBuiltinsAssembler"),
];
