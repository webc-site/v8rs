// 由 tool/src/builtins/collections.tq.js 从 v8/src/builtins/collections.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-collections; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 45 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "collections", "namespace collections"),
  (
    "const",
    "kSetPrototypeValues",
    "const kSetPrototypeValues: constexpr BuiltinsName generates 'Builtin::kSetPrototypeValues'",
  ),
  (
    "const",
    "kSetPrototypeHas",
    "const kSetPrototypeHas: constexpr BuiltinsName generates 'Builtin::kSetPrototypeHas'",
  ),
  (
    "const",
    "kMapPrototypeKeys",
    "const kMapPrototypeKeys: constexpr BuiltinsName generates 'Builtin::kMapPrototypeKeys'",
  ),
  (
    "const",
    "kMapPrototypeHas",
    "const kMapPrototypeHas: constexpr BuiltinsName generates 'Builtin::kMapPrototypeHas'",
  ),
  ("struct", "SetRecord", "struct SetRecord"),
  (
    "macro",
    "CodeStubAssembler::CloneFixedArray",
    "macro CodeStubAssembler::CloneFixedArray( FixedArrayBase, constexpr ExtractFixedArrayFlag): FixedArrayBase",
  ),
  (
    "macro",
    "CollectionsBuiltinsAssembler::AddToSetTable",
    "macro CollectionsBuiltinsAssembler::AddToSetTable( implicit context: Context)(OrderedHashSet, JSAny, String): OrderedHashSet",
  ),
  (
    "macro",
    "CollectionsBuiltinsAssembler::TableHasKey",
    "macro CollectionsBuiltinsAssembler::TableHasKey( implicit context: Context)(OrderedHashSet, JSAny): bool",
  ),
  (
    "macro",
    "CollectionsBuiltinsAssembler::TableHasKey",
    "macro CollectionsBuiltinsAssembler::TableHasKey( implicit context: Context)(OrderedHashMap, JSAny): bool",
  ),
  (
    "macro",
    "CollectionsBuiltinsAssembler::DeleteFromSetTable",
    "macro CollectionsBuiltinsAssembler::DeleteFromSetTable( implicit context: Context)(OrderedHashSet, JSAny): Smi labels NotFound",
  ),
  ("struct", "KeyIndexPair", "struct KeyIndexPair"),
  (
    "struct",
    "OrderedHashSetIndexPair",
    "struct OrderedHashSetIndexPair",
  ),
  (
    "macro",
    "CollectionsBuiltinsAssembler::NextKeyIndexPairUnmodifiedTable",
    "macro CollectionsBuiltinsAssembler::NextKeyIndexPairUnmodifiedTable( OrderedHashSet, int32, int32, intptr): KeyIndexPair labels Done",
  ),
  (
    "struct",
    "UnmodifiedOrderedHashSetIterator",
    "struct UnmodifiedOrderedHashSetIterator",
  ),
  (
    "macro",
    "CollectionsBuiltinsAssembler::TransitionOrderedHashSetNoUpdate",
    "macro CollectionsBuiltinsAssembler::TransitionOrderedHashSetNoUpdate( OrderedHashSet, intptr): OrderedHashSetIndexPair",
  ),
  (
    "macro",
    "CollectionsBuiltinsAssembler::NextKeyIndexPair",
    "macro CollectionsBuiltinsAssembler::NextKeyIndexPair( OrderedHashSet, intptr): KeyIndexPair labels Done",
  ),
  (
    "struct",
    "OrderedHashSetIterator",
    "struct OrderedHashSetIterator",
  ),
  (
    "macro",
    "LoadOrderedHashTableMetadata",
    "macro LoadOrderedHashTableMetadata( table: OrderedHashMap|OrderedHashSet, fieldIndex: constexpr int32): int32",
  ),
  (
    "const",
    "kOrderedHashSetNumberOfBucketsIndex",
    "const kOrderedHashSetNumberOfBucketsIndex: constexpr int32 generates 'OrderedHashSet::NumberOfBucketsIndex()'",
  ),
  (
    "const",
    "kOrderedHashSetNumberOfElementsIndex",
    "const kOrderedHashSetNumberOfElementsIndex: constexpr int32 generates 'OrderedHashSet::NumberOfElementsIndex()'",
  ),
  (
    "const",
    "kOrderedHashSetNumberOfDeletedElementsIndex",
    "const kOrderedHashSetNumberOfDeletedElementsIndex: constexpr int32 generates 'OrderedHashSet::NumberOfDeletedElementsIndex()'",
  ),
  (
    "macro",
    "NewUnmodifiedOrderedHashSetIterator",
    "macro NewUnmodifiedOrderedHashSetIterator(table: OrderedHashSet): UnmodifiedOrderedHashSetIterator",
  ),
  (
    "macro",
    "NewOrderedHashSetIterator",
    "macro NewOrderedHashSetIterator(table: OrderedHashSet): OrderedHashSetIterator",
  ),
  ("struct", "KeyValueIndexTuple", "struct KeyValueIndexTuple"),
  (
    "macro",
    "CollectionsBuiltinsAssembler::NextKeyValueIndexTupleUnmodifiedTable",
    "macro CollectionsBuiltinsAssembler::NextKeyValueIndexTupleUnmodifiedTable( OrderedHashMap, int32, int32, intptr): KeyValueIndexTuple labels Done",
  ),
  (
    "macro",
    "CollectionsBuiltinsAssembler::NextKeyValueIndexTuple",
    "macro CollectionsBuiltinsAssembler::NextKeyValueIndexTuple( OrderedHashMap, intptr): KeyValueIndexTuple labels Done",
  ),
  (
    "macro",
    "CollectionsBuiltinsAssembler::UnsafeStoreValueInOrderedHashMapEntry",
    "macro CollectionsBuiltinsAssembler::UnsafeStoreValueInOrderedHashMapEntry( OrderedHashMap, Object, intptr): void",
  ),
  (
    "const",
    "kOrderedHashMapEntrySize",
    "const kOrderedHashMapEntrySize: constexpr int32 generates 'OrderedHashMap::kEntrySize'",
  ),
  (
    "struct",
    "UnmodifiedOrderedHashMapIterator",
    "struct UnmodifiedOrderedHashMapIterator",
  ),
  (
    "const",
    "kOrderedHashMapNumberOfBucketsIndex",
    "const kOrderedHashMapNumberOfBucketsIndex: constexpr int32 generates 'OrderedHashMap::NumberOfBucketsIndex()'",
  ),
  (
    "const",
    "kOrderedHashMapNumberOfElementsIndex",
    "const kOrderedHashMapNumberOfElementsIndex: constexpr int32 generates 'OrderedHashMap::NumberOfElementsIndex()'",
  ),
  (
    "const",
    "kOrderedHashMapNumberOfDeletedElementsIndex",
    "const kOrderedHashMapNumberOfDeletedElementsIndex: constexpr int32 generates 'OrderedHashMap::NumberOfDeletedElementsIndex()'",
  ),
  (
    "macro",
    "NewUnmodifiedOrderedHashMapIterator",
    "macro NewUnmodifiedOrderedHashMapIterator(table: OrderedHashMap): UnmodifiedOrderedHashMapIterator",
  ),
  (
    "macro",
    "LoadKeyValuePairNoSideEffects",
    "macro LoadKeyValuePairNoSideEffects(implicit context: Context)(o: JSAny): KeyValuePair labels MayHaveSideEffects",
  ),
  (
    "macro",
    "LoadKeyValuePair",
    "macro LoadKeyValuePair(implicit context: Context)(o: JSAny): KeyValuePair",
  ),
  (
    "macro",
    "GetSetRecord",
    "macro GetSetRecord( implicit context: Context)(obj: JSAny, methodName: constexpr string): SetRecord",
  ),
  (
    "macro",
    "GetKeysIterator",
    "macro GetKeysIterator( implicit context: Context)(set: JSReceiver, keys: Callable): iterator::IteratorRecord",
  ),
  (
    "macro",
    "CheckSetRecordHasJSSetMethods",
    "macro CheckSetRecordHasJSSetMethods(setRecord: SetRecord): void labels HasUserProvidedMethods",
  ),
  (
    "macro",
    "CheckSetRecordHasJSMapMethods",
    "macro CheckSetRecordHasJSMapMethods(setRecord: SetRecord): void labels HasUserProvidedMethods",
  ),
  (
    "macro",
    "ShrinkOrderedHashSetIfNeeded",
    "macro ShrinkOrderedHashSetIfNeeded( implicit context: Context)(numberOfElements: Smi, resultSetData: OrderedHashSet): OrderedHashSet",
  ),
  (
    "struct",
    "StableJSSetBackingTableWitness",
    "struct StableJSSetBackingTableWitness",
  ),
  (
    "struct",
    "StableJSMapBackingTableWitness",
    "struct StableJSMapBackingTableWitness",
  ),
  (
    "macro",
    "NewStableBackingTableWitness",
    "macro NewStableBackingTableWitness(o: JSSet): StableJSSetBackingTableWitness",
  ),
  (
    "macro",
    "NewStableBackingTableWitness",
    "macro NewStableBackingTableWitness(o: JSMap): StableJSMapBackingTableWitness",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  ("kSetPrototypeValues", "Builtin::kSetPrototypeValues"),
  ("kSetPrototypeHas", "Builtin::kSetPrototypeHas"),
  ("kMapPrototypeKeys", "Builtin::kMapPrototypeKeys"),
  ("kMapPrototypeHas", "Builtin::kMapPrototypeHas"),
  (
    "kOrderedHashSetNumberOfBucketsIndex",
    "OrderedHashSet::NumberOfBucketsIndex()",
  ),
  (
    "kOrderedHashSetNumberOfElementsIndex",
    "OrderedHashSet::NumberOfElementsIndex()",
  ),
  (
    "kOrderedHashSetNumberOfDeletedElementsIndex",
    "OrderedHashSet::NumberOfDeletedElementsIndex()",
  ),
  ("kOrderedHashMapEntrySize", "OrderedHashMap::kEntrySize"),
  (
    "kOrderedHashMapNumberOfBucketsIndex",
    "OrderedHashMap::NumberOfBucketsIndex()",
  ),
  (
    "kOrderedHashMapNumberOfElementsIndex",
    "OrderedHashMap::NumberOfElementsIndex()",
  ),
  (
    "kOrderedHashMapNumberOfDeletedElementsIndex",
    "OrderedHashMap::NumberOfDeletedElementsIndex()",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 45);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 11);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 9);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 24);
};
