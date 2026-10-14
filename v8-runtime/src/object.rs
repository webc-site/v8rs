// 由 tool/src/runtime/gen-runtime.js 生成
// 对象/数组/集合/TypedArray/Classes/Proxy 运行时函数

use strum::{Display, EnumIter, EnumString};

use crate::table::{RuntimeFunction, RuntimeFunctionId, find_runtime_function};

/// 对象与集合操作运行时函数 (123 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum ObjectRuntimeFunction {
  #[strum(serialize = "Runtime_ObjectKeys", serialize = "ObjectKeys")]
  ObjectKeys,
  #[strum(
    serialize = "Runtime_ObjectGetOwnPropertyNames",
    serialize = "ObjectGetOwnPropertyNames"
  )]
  ObjectGetOwnPropertyNames,
  #[strum(
    serialize = "Runtime_ObjectGetOwnPropertyNamesTryFast",
    serialize = "ObjectGetOwnPropertyNamesTryFast"
  )]
  ObjectGetOwnPropertyNamesTryFast,
  #[strum(
    serialize = "Runtime_ObjectHasOwnProperty",
    serialize = "ObjectHasOwnProperty"
  )]
  ObjectHasOwnProperty,
  #[strum(
    serialize = "Runtime_HasOwnConstDataProperty",
    serialize = "HasOwnConstDataProperty"
  )]
  HasOwnConstDataProperty,
  #[strum(
    serialize = "Runtime_IsDictPropertyConstTrackingEnabled",
    serialize = "IsDictPropertyConstTrackingEnabled"
  )]
  IsDictPropertyConstTrackingEnabled,
  #[strum(
    serialize = "Runtime_AddDictionaryProperty",
    serialize = "AddDictionaryProperty"
  )]
  AddDictionaryProperty,
  #[strum(serialize = "Runtime_AddPrivateBrand", serialize = "AddPrivateBrand")]
  AddPrivateBrand,
  #[strum(serialize = "Runtime_ObjectCreate", serialize = "ObjectCreate")]
  ObjectCreate,
  #[strum(
    serialize = "Runtime_InternalSetPrototype",
    serialize = "InternalSetPrototype"
  )]
  InternalSetPrototype,
  #[strum(
    serialize = "Runtime_OptimizeObjectForAddingMultipleProperties",
    serialize = "OptimizeObjectForAddingMultipleProperties"
  )]
  OptimizeObjectForAddingMultipleProperties,
  #[strum(serialize = "Runtime_ObjectValues", serialize = "ObjectValues")]
  ObjectValues,
  #[strum(
    serialize = "Runtime_ObjectValuesSkipFastPath",
    serialize = "ObjectValuesSkipFastPath"
  )]
  ObjectValuesSkipFastPath,
  #[strum(serialize = "Runtime_ObjectEntries", serialize = "ObjectEntries")]
  ObjectEntries,
  #[strum(
    serialize = "Runtime_ObjectEntriesSkipFastPath",
    serialize = "ObjectEntriesSkipFastPath"
  )]
  ObjectEntriesSkipFastPath,
  #[strum(
    serialize = "Runtime_ObjectIsExtensible",
    serialize = "ObjectIsExtensible"
  )]
  ObjectIsExtensible,
  #[strum(
    serialize = "Runtime_JSReceiverPreventExtensionsThrow",
    serialize = "JSReceiverPreventExtensionsThrow"
  )]
  JSReceiverPreventExtensionsThrow,
  #[strum(
    serialize = "Runtime_JSReceiverPreventExtensionsDontThrow",
    serialize = "JSReceiverPreventExtensionsDontThrow"
  )]
  JSReceiverPreventExtensionsDontThrow,
  #[strum(
    serialize = "Runtime_JSReceiverGetPrototypeOf",
    serialize = "JSReceiverGetPrototypeOf"
  )]
  JSReceiverGetPrototypeOf,
  #[strum(
    serialize = "Runtime_JSReceiverSetPrototypeOfThrow",
    serialize = "JSReceiverSetPrototypeOfThrow"
  )]
  JSReceiverSetPrototypeOfThrow,
  #[strum(
    serialize = "Runtime_JSReceiverSetPrototypeOfDontThrow",
    serialize = "JSReceiverSetPrototypeOfDontThrow"
  )]
  JSReceiverSetPrototypeOfDontThrow,
  #[strum(serialize = "Runtime_GetProperty", serialize = "GetProperty")]
  GetProperty,
  #[strum(serialize = "Runtime_SetKeyedProperty", serialize = "SetKeyedProperty")]
  SetKeyedProperty,
  #[strum(
    serialize = "Runtime_DefineObjectOwnProperty",
    serialize = "DefineObjectOwnProperty"
  )]
  DefineObjectOwnProperty,
  #[strum(serialize = "Runtime_SetNamedProperty", serialize = "SetNamedProperty")]
  SetNamedProperty,
  #[strum(serialize = "Runtime_DeleteProperty", serialize = "DeleteProperty")]
  DeleteProperty,
  #[strum(
    serialize = "Runtime_ShrinkNameDictionary",
    serialize = "ShrinkNameDictionary"
  )]
  ShrinkNameDictionary,
  #[strum(
    serialize = "Runtime_ShrinkSwissNameDictionary",
    serialize = "ShrinkSwissNameDictionary"
  )]
  ShrinkSwissNameDictionary,
  #[strum(serialize = "Runtime_HasProperty", serialize = "HasProperty")]
  HasProperty,
  #[strum(
    serialize = "Runtime_GetOwnPropertyKeys",
    serialize = "GetOwnPropertyKeys"
  )]
  GetOwnPropertyKeys,
  #[strum(serialize = "Runtime_ToFastProperties", serialize = "ToFastProperties")]
  ToFastProperties,
  #[strum(
    serialize = "Runtime_AllocateHeapNumber",
    serialize = "AllocateHeapNumber"
  )]
  AllocateHeapNumber,
  #[strum(serialize = "Runtime_NewObject", serialize = "NewObject")]
  NewObject,
  #[strum(serialize = "Runtime_GetDerivedMap", serialize = "GetDerivedMap")]
  GetDerivedMap,
  #[strum(
    serialize = "Runtime_CompleteInobjectSlackTrackingForMap",
    serialize = "CompleteInobjectSlackTrackingForMap"
  )]
  CompleteInobjectSlackTrackingForMap,
  #[strum(
    serialize = "Runtime_TryMigrateInstance",
    serialize = "TryMigrateInstance"
  )]
  TryMigrateInstance,
  #[strum(
    serialize = "Runtime_TryMigrateInstanceAndMarkMapAsMigrationTarget",
    serialize = "TryMigrateInstanceAndMarkMapAsMigrationTarget"
  )]
  TryMigrateInstanceAndMarkMapAsMigrationTarget,
  #[strum(
    serialize = "Runtime_DefineAccessorPropertyUnchecked",
    serialize = "DefineAccessorPropertyUnchecked"
  )]
  DefineAccessorPropertyUnchecked,
  #[strum(serialize = "Runtime_SetFunctionName", serialize = "SetFunctionName")]
  SetFunctionName,
  #[strum(
    serialize = "Runtime_DefineKeyedOwnPropertyInLiteral",
    serialize = "DefineKeyedOwnPropertyInLiteral"
  )]
  DefineKeyedOwnPropertyInLiteral,
  #[strum(
    serialize = "Runtime_HasFastPackedElements",
    serialize = "HasFastPackedElements"
  )]
  HasFastPackedElements,
  #[strum(serialize = "Runtime_IsJSReceiver", serialize = "IsJSReceiver")]
  IsJSReceiver,
  #[strum(serialize = "Runtime_GetFunctionName", serialize = "GetFunctionName")]
  GetFunctionName,
  #[strum(
    serialize = "Runtime_DefineGetterPropertyUnchecked",
    serialize = "DefineGetterPropertyUnchecked"
  )]
  DefineGetterPropertyUnchecked,
  #[strum(
    serialize = "Runtime_SetDataProperties",
    serialize = "SetDataProperties"
  )]
  SetDataProperties,
  #[strum(
    serialize = "Runtime_CopyDataProperties",
    serialize = "CopyDataProperties"
  )]
  CopyDataProperties,
  #[strum(
    serialize = "Runtime_CopyDataPropertiesWithExcludedPropertiesOnStack",
    serialize = "CopyDataPropertiesWithExcludedPropertiesOnStack"
  )]
  CopyDataPropertiesWithExcludedPropertiesOnStack,
  #[strum(
    serialize = "Runtime_DefineSetterPropertyUnchecked",
    serialize = "DefineSetterPropertyUnchecked"
  )]
  DefineSetterPropertyUnchecked,
  #[strum(serialize = "Runtime_ToObject", serialize = "ToObject")]
  ToObject,
  #[strum(serialize = "Runtime_ToNumber", serialize = "ToNumber")]
  ToNumber,
  #[strum(serialize = "Runtime_ToNumeric", serialize = "ToNumeric")]
  ToNumeric,
  #[strum(serialize = "Runtime_ToLength", serialize = "ToLength")]
  ToLength,
  #[strum(serialize = "Runtime_ToString", serialize = "ToString")]
  ToString,
  #[strum(serialize = "Runtime_ToName", serialize = "ToName")]
  ToName,
  #[strum(
    serialize = "Runtime_HasInPrototypeChain",
    serialize = "HasInPrototypeChain"
  )]
  HasInPrototypeChain,
  #[strum(
    serialize = "Runtime_CreateIterResultObject",
    serialize = "CreateIterResultObject"
  )]
  CreateIterResultObject,
  #[strum(
    serialize = "Runtime_CreateDataProperty",
    serialize = "CreateDataProperty"
  )]
  CreateDataProperty,
  #[strum(
    serialize = "Runtime_SetOwnPropertyIgnoreAttributes",
    serialize = "SetOwnPropertyIgnoreAttributes"
  )]
  SetOwnPropertyIgnoreAttributes,
  #[strum(
    serialize = "Runtime_GetOwnPropertyDescriptorObject",
    serialize = "GetOwnPropertyDescriptorObject"
  )]
  GetOwnPropertyDescriptorObject,
  #[strum(serialize = "Runtime_GetPrivateMember", serialize = "GetPrivateMember")]
  GetPrivateMember,
  #[strum(serialize = "Runtime_SetPrivateMember", serialize = "SetPrivateMember")]
  SetPrivateMember,
  #[strum(
    serialize = "Runtime_LoadPrivateSetter",
    serialize = "LoadPrivateSetter"
  )]
  LoadPrivateSetter,
  #[strum(
    serialize = "Runtime_LoadPrivateGetter",
    serialize = "LoadPrivateGetter"
  )]
  LoadPrivateGetter,
  #[strum(
    serialize = "Runtime_CreatePrivateAccessors",
    serialize = "CreatePrivateAccessors"
  )]
  CreatePrivateAccessors,
  #[strum(
    serialize = "Runtime_SwissTableAllocate",
    serialize = "SwissTableAllocate"
  )]
  SwissTableAllocate,
  #[strum(serialize = "Runtime_SwissTableAdd", serialize = "SwissTableAdd")]
  SwissTableAdd,
  #[strum(
    serialize = "Runtime_SwissTableFindEntry",
    serialize = "SwissTableFindEntry"
  )]
  SwissTableFindEntry,
  #[strum(serialize = "Runtime_SwissTableUpdate", serialize = "SwissTableUpdate")]
  SwissTableUpdate,
  #[strum(serialize = "Runtime_SwissTableDelete", serialize = "SwissTableDelete")]
  SwissTableDelete,
  #[strum(serialize = "Runtime_SwissTableEquals", serialize = "SwissTableEquals")]
  SwissTableEquals,
  #[strum(
    serialize = "Runtime_SwissTableElementsCount",
    serialize = "SwissTableElementsCount"
  )]
  SwissTableElementsCount,
  #[strum(serialize = "Runtime_SwissTableKeyAt", serialize = "SwissTableKeyAt")]
  SwissTableKeyAt,
  #[strum(
    serialize = "Runtime_SwissTableValueAt",
    serialize = "SwissTableValueAt"
  )]
  SwissTableValueAt,
  #[strum(
    serialize = "Runtime_SwissTableDetailsAt",
    serialize = "SwissTableDetailsAt"
  )]
  SwissTableDetailsAt,
  #[strum(
    serialize = "Runtime_IterableToListWithInterceptor",
    serialize = "IterableToListWithInterceptor"
  )]
  IterableToListWithInterceptor,
  #[strum(
    serialize = "Runtime_CheckFastIterableToListPrototype",
    serialize = "CheckFastIterableToListPrototype"
  )]
  CheckFastIterableToListPrototype,
  #[strum(
    serialize = "Runtime_TransitionElementsKind",
    serialize = "TransitionElementsKind"
  )]
  TransitionElementsKind,
  #[strum(
    serialize = "Runtime_TransitionElementsKindWithKind",
    serialize = "TransitionElementsKindWithKind"
  )]
  TransitionElementsKindWithKind,
  #[strum(serialize = "Runtime_NewArray", serialize = "NewArray")]
  NewArray,
  #[strum(
    serialize = "Runtime_NormalizeElements",
    serialize = "NormalizeElements"
  )]
  NormalizeElements,
  #[strum(
    serialize = "Runtime_GrowArrayElements",
    serialize = "GrowArrayElements"
  )]
  GrowArrayElements,
  #[strum(serialize = "Runtime_ArrayIsArray", serialize = "ArrayIsArray")]
  ArrayIsArray,
  #[strum(serialize = "Runtime_IsArray", serialize = "IsArray")]
  IsArray,
  #[strum(
    serialize = "Runtime_ArraySpeciesConstructor",
    serialize = "ArraySpeciesConstructor"
  )]
  ArraySpeciesConstructor,
  #[strum(
    serialize = "Runtime_ArrayIncludes_Slow",
    serialize = "ArrayIncludes_Slow",
    serialize = "ArrayIncludesSlow"
  )]
  ArrayIncludesSlow,
  #[strum(serialize = "Runtime_ArrayIndexOf", serialize = "ArrayIndexOf")]
  ArrayIndexOf,
  #[strum(serialize = "Runtime_TheHole", serialize = "TheHole")]
  TheHole,
  #[strum(
    serialize = "Runtime_OrderedHashSetGrow",
    serialize = "OrderedHashSetGrow"
  )]
  OrderedHashSetGrow,
  #[strum(serialize = "Runtime_SetGrow", serialize = "SetGrow")]
  SetGrow,
  #[strum(serialize = "Runtime_SetShrink", serialize = "SetShrink")]
  SetShrink,
  #[strum(
    serialize = "Runtime_OrderedHashSetShrink",
    serialize = "OrderedHashSetShrink"
  )]
  OrderedHashSetShrink,
  #[strum(serialize = "Runtime_MapShrink", serialize = "MapShrink")]
  MapShrink,
  #[strum(serialize = "Runtime_MapGrow", serialize = "MapGrow")]
  MapGrow,
  #[strum(
    serialize = "Runtime_OrderedHashMapGrow",
    serialize = "OrderedHashMapGrow"
  )]
  OrderedHashMapGrow,
  #[strum(
    serialize = "Runtime_WeakCollectionDelete",
    serialize = "WeakCollectionDelete"
  )]
  WeakCollectionDelete,
  #[strum(
    serialize = "Runtime_WeakCollectionSet",
    serialize = "WeakCollectionSet"
  )]
  WeakCollectionSet,
  #[strum(
    serialize = "Runtime_ArrayBufferDetach",
    serialize = "ArrayBufferDetach"
  )]
  ArrayBufferDetach,
  #[strum(
    serialize = "Runtime_ArrayBufferSetDetachKey",
    serialize = "ArrayBufferSetDetachKey"
  )]
  ArrayBufferSetDetachKey,
  #[strum(
    serialize = "Runtime_TypedArrayCopyElements",
    serialize = "TypedArrayCopyElements"
  )]
  TypedArrayCopyElements,
  #[strum(
    serialize = "Runtime_TypedArrayGetBuffer",
    serialize = "TypedArrayGetBuffer"
  )]
  TypedArrayGetBuffer,
  #[strum(
    serialize = "Runtime_GrowableSharedArrayBufferByteLength",
    serialize = "GrowableSharedArrayBufferByteLength"
  )]
  GrowableSharedArrayBufferByteLength,
  #[strum(
    serialize = "Runtime_TypedArraySortFast",
    serialize = "TypedArraySortFast"
  )]
  TypedArraySortFast,
  #[strum(serialize = "Runtime_TypedArraySet", serialize = "TypedArraySet")]
  TypedArraySet,
  #[strum(
    serialize = "Runtime_ArrayBufferMaxByteLength",
    serialize = "ArrayBufferMaxByteLength"
  )]
  ArrayBufferMaxByteLength,
  #[strum(
    serialize = "Runtime_ThrowUnsupportedSuperError",
    serialize = "ThrowUnsupportedSuperError"
  )]
  ThrowUnsupportedSuperError,
  #[strum(
    serialize = "Runtime_ThrowConstructorNonCallableError",
    serialize = "ThrowConstructorNonCallableError"
  )]
  ThrowConstructorNonCallableError,
  #[strum(
    serialize = "Runtime_ThrowStaticPrototypeError",
    serialize = "ThrowStaticPrototypeError"
  )]
  ThrowStaticPrototypeError,
  #[strum(
    serialize = "Runtime_ThrowSuperAlreadyCalledError",
    serialize = "ThrowSuperAlreadyCalledError"
  )]
  ThrowSuperAlreadyCalledError,
  #[strum(
    serialize = "Runtime_ThrowSuperNotCalled",
    serialize = "ThrowSuperNotCalled"
  )]
  ThrowSuperNotCalled,
  #[strum(
    serialize = "Runtime_ThrowNotSuperConstructor",
    serialize = "ThrowNotSuperConstructor"
  )]
  ThrowNotSuperConstructor,
  #[strum(serialize = "Runtime_DefineClass", serialize = "DefineClass")]
  DefineClass,
  #[strum(serialize = "Runtime_LoadFromSuper", serialize = "LoadFromSuper")]
  LoadFromSuper,
  #[strum(
    serialize = "Runtime_LoadKeyedFromSuper",
    serialize = "LoadKeyedFromSuper"
  )]
  LoadKeyedFromSuper,
  #[strum(serialize = "Runtime_StoreToSuper", serialize = "StoreToSuper")]
  StoreToSuper,
  #[strum(
    serialize = "Runtime_StoreKeyedToSuper",
    serialize = "StoreKeyedToSuper"
  )]
  StoreKeyedToSuper,
  #[strum(serialize = "Runtime_IsJSProxy", serialize = "IsJSProxy")]
  IsJSProxy,
  #[strum(
    serialize = "Runtime_JSProxyGetHandler",
    serialize = "JSProxyGetHandler"
  )]
  JSProxyGetHandler,
  #[strum(serialize = "Runtime_JSProxyGetTarget", serialize = "JSProxyGetTarget")]
  JSProxyGetTarget,
  #[strum(
    serialize = "Runtime_GetPropertyWithReceiver",
    serialize = "GetPropertyWithReceiver"
  )]
  GetPropertyWithReceiver,
  #[strum(
    serialize = "Runtime_SetPropertyWithReceiver",
    serialize = "SetPropertyWithReceiver"
  )]
  SetPropertyWithReceiver,
  #[strum(
    serialize = "Runtime_CheckProxyGetSetTrapResult",
    serialize = "CheckProxyGetSetTrapResult"
  )]
  CheckProxyGetSetTrapResult,
  #[strum(
    serialize = "Runtime_CheckProxyHasTrapResult",
    serialize = "CheckProxyHasTrapResult"
  )]
  CheckProxyHasTrapResult,
  #[strum(
    serialize = "Runtime_CheckProxyDeleteTrapResult",
    serialize = "CheckProxyDeleteTrapResult"
  )]
  CheckProxyDeleteTrapResult,
}

impl ObjectRuntimeFunction {
  pub const fn name(&self) -> &'static str {
    match self {
      Self::ObjectKeys => "ObjectKeys",
      Self::ObjectGetOwnPropertyNames => "ObjectGetOwnPropertyNames",
      Self::ObjectGetOwnPropertyNamesTryFast => "ObjectGetOwnPropertyNamesTryFast",
      Self::ObjectHasOwnProperty => "ObjectHasOwnProperty",
      Self::HasOwnConstDataProperty => "HasOwnConstDataProperty",
      Self::IsDictPropertyConstTrackingEnabled => "IsDictPropertyConstTrackingEnabled",
      Self::AddDictionaryProperty => "AddDictionaryProperty",
      Self::AddPrivateBrand => "AddPrivateBrand",
      Self::ObjectCreate => "ObjectCreate",
      Self::InternalSetPrototype => "InternalSetPrototype",
      Self::OptimizeObjectForAddingMultipleProperties => {
        "OptimizeObjectForAddingMultipleProperties"
      }
      Self::ObjectValues => "ObjectValues",
      Self::ObjectValuesSkipFastPath => "ObjectValuesSkipFastPath",
      Self::ObjectEntries => "ObjectEntries",
      Self::ObjectEntriesSkipFastPath => "ObjectEntriesSkipFastPath",
      Self::ObjectIsExtensible => "ObjectIsExtensible",
      Self::JSReceiverPreventExtensionsThrow => "JSReceiverPreventExtensionsThrow",
      Self::JSReceiverPreventExtensionsDontThrow => "JSReceiverPreventExtensionsDontThrow",
      Self::JSReceiverGetPrototypeOf => "JSReceiverGetPrototypeOf",
      Self::JSReceiverSetPrototypeOfThrow => "JSReceiverSetPrototypeOfThrow",
      Self::JSReceiverSetPrototypeOfDontThrow => "JSReceiverSetPrototypeOfDontThrow",
      Self::GetProperty => "GetProperty",
      Self::SetKeyedProperty => "SetKeyedProperty",
      Self::DefineObjectOwnProperty => "DefineObjectOwnProperty",
      Self::SetNamedProperty => "SetNamedProperty",
      Self::DeleteProperty => "DeleteProperty",
      Self::ShrinkNameDictionary => "ShrinkNameDictionary",
      Self::ShrinkSwissNameDictionary => "ShrinkSwissNameDictionary",
      Self::HasProperty => "HasProperty",
      Self::GetOwnPropertyKeys => "GetOwnPropertyKeys",
      Self::ToFastProperties => "ToFastProperties",
      Self::AllocateHeapNumber => "AllocateHeapNumber",
      Self::NewObject => "NewObject",
      Self::GetDerivedMap => "GetDerivedMap",
      Self::CompleteInobjectSlackTrackingForMap => "CompleteInobjectSlackTrackingForMap",
      Self::TryMigrateInstance => "TryMigrateInstance",
      Self::TryMigrateInstanceAndMarkMapAsMigrationTarget => {
        "TryMigrateInstanceAndMarkMapAsMigrationTarget"
      }
      Self::DefineAccessorPropertyUnchecked => "DefineAccessorPropertyUnchecked",
      Self::SetFunctionName => "SetFunctionName",
      Self::DefineKeyedOwnPropertyInLiteral => "DefineKeyedOwnPropertyInLiteral",
      Self::HasFastPackedElements => "HasFastPackedElements",
      Self::IsJSReceiver => "IsJSReceiver",
      Self::GetFunctionName => "GetFunctionName",
      Self::DefineGetterPropertyUnchecked => "DefineGetterPropertyUnchecked",
      Self::SetDataProperties => "SetDataProperties",
      Self::CopyDataProperties => "CopyDataProperties",
      Self::CopyDataPropertiesWithExcludedPropertiesOnStack => {
        "CopyDataPropertiesWithExcludedPropertiesOnStack"
      }
      Self::DefineSetterPropertyUnchecked => "DefineSetterPropertyUnchecked",
      Self::ToObject => "ToObject",
      Self::ToNumber => "ToNumber",
      Self::ToNumeric => "ToNumeric",
      Self::ToLength => "ToLength",
      Self::ToString => "ToString",
      Self::ToName => "ToName",
      Self::HasInPrototypeChain => "HasInPrototypeChain",
      Self::CreateIterResultObject => "CreateIterResultObject",
      Self::CreateDataProperty => "CreateDataProperty",
      Self::SetOwnPropertyIgnoreAttributes => "SetOwnPropertyIgnoreAttributes",
      Self::GetOwnPropertyDescriptorObject => "GetOwnPropertyDescriptorObject",
      Self::GetPrivateMember => "GetPrivateMember",
      Self::SetPrivateMember => "SetPrivateMember",
      Self::LoadPrivateSetter => "LoadPrivateSetter",
      Self::LoadPrivateGetter => "LoadPrivateGetter",
      Self::CreatePrivateAccessors => "CreatePrivateAccessors",
      Self::SwissTableAllocate => "SwissTableAllocate",
      Self::SwissTableAdd => "SwissTableAdd",
      Self::SwissTableFindEntry => "SwissTableFindEntry",
      Self::SwissTableUpdate => "SwissTableUpdate",
      Self::SwissTableDelete => "SwissTableDelete",
      Self::SwissTableEquals => "SwissTableEquals",
      Self::SwissTableElementsCount => "SwissTableElementsCount",
      Self::SwissTableKeyAt => "SwissTableKeyAt",
      Self::SwissTableValueAt => "SwissTableValueAt",
      Self::SwissTableDetailsAt => "SwissTableDetailsAt",
      Self::IterableToListWithInterceptor => "IterableToListWithInterceptor",
      Self::CheckFastIterableToListPrototype => "CheckFastIterableToListPrototype",
      Self::TransitionElementsKind => "TransitionElementsKind",
      Self::TransitionElementsKindWithKind => "TransitionElementsKindWithKind",
      Self::NewArray => "NewArray",
      Self::NormalizeElements => "NormalizeElements",
      Self::GrowArrayElements => "GrowArrayElements",
      Self::ArrayIsArray => "ArrayIsArray",
      Self::IsArray => "IsArray",
      Self::ArraySpeciesConstructor => "ArraySpeciesConstructor",
      Self::ArrayIncludesSlow => "ArrayIncludes_Slow",
      Self::ArrayIndexOf => "ArrayIndexOf",
      Self::TheHole => "TheHole",
      Self::OrderedHashSetGrow => "OrderedHashSetGrow",
      Self::SetGrow => "SetGrow",
      Self::SetShrink => "SetShrink",
      Self::OrderedHashSetShrink => "OrderedHashSetShrink",
      Self::MapShrink => "MapShrink",
      Self::MapGrow => "MapGrow",
      Self::OrderedHashMapGrow => "OrderedHashMapGrow",
      Self::WeakCollectionDelete => "WeakCollectionDelete",
      Self::WeakCollectionSet => "WeakCollectionSet",
      Self::ArrayBufferDetach => "ArrayBufferDetach",
      Self::ArrayBufferSetDetachKey => "ArrayBufferSetDetachKey",
      Self::TypedArrayCopyElements => "TypedArrayCopyElements",
      Self::TypedArrayGetBuffer => "TypedArrayGetBuffer",
      Self::GrowableSharedArrayBufferByteLength => "GrowableSharedArrayBufferByteLength",
      Self::TypedArraySortFast => "TypedArraySortFast",
      Self::TypedArraySet => "TypedArraySet",
      Self::ArrayBufferMaxByteLength => "ArrayBufferMaxByteLength",
      Self::ThrowUnsupportedSuperError => "ThrowUnsupportedSuperError",
      Self::ThrowConstructorNonCallableError => "ThrowConstructorNonCallableError",
      Self::ThrowStaticPrototypeError => "ThrowStaticPrototypeError",
      Self::ThrowSuperAlreadyCalledError => "ThrowSuperAlreadyCalledError",
      Self::ThrowSuperNotCalled => "ThrowSuperNotCalled",
      Self::ThrowNotSuperConstructor => "ThrowNotSuperConstructor",
      Self::DefineClass => "DefineClass",
      Self::LoadFromSuper => "LoadFromSuper",
      Self::LoadKeyedFromSuper => "LoadKeyedFromSuper",
      Self::StoreToSuper => "StoreToSuper",
      Self::StoreKeyedToSuper => "StoreKeyedToSuper",
      Self::IsJSProxy => "IsJSProxy",
      Self::JSProxyGetHandler => "JSProxyGetHandler",
      Self::JSProxyGetTarget => "JSProxyGetTarget",
      Self::GetPropertyWithReceiver => "GetPropertyWithReceiver",
      Self::SetPropertyWithReceiver => "SetPropertyWithReceiver",
      Self::CheckProxyGetSetTrapResult => "CheckProxyGetSetTrapResult",
      Self::CheckProxyHasTrapResult => "CheckProxyHasTrapResult",
      Self::CheckProxyDeleteTrapResult => "CheckProxyDeleteTrapResult",
    }
  }

  pub fn descriptor(&self) -> Option<&'static RuntimeFunction> {
    find_runtime_function(self.name())
  }

  pub fn function_id(&self) -> Option<RuntimeFunctionId> {
    self.descriptor().map(|f| f.id)
  }

  pub const fn is_object(&self) -> bool {
    matches!(
      self,
      Self::ObjectKeys
        | Self::ObjectGetOwnPropertyNames
        | Self::ObjectGetOwnPropertyNamesTryFast
        | Self::ObjectHasOwnProperty
        | Self::HasOwnConstDataProperty
        | Self::IsDictPropertyConstTrackingEnabled
        | Self::AddDictionaryProperty
        | Self::AddPrivateBrand
        | Self::ObjectCreate
        | Self::InternalSetPrototype
        | Self::OptimizeObjectForAddingMultipleProperties
        | Self::ObjectValues
        | Self::ObjectValuesSkipFastPath
        | Self::ObjectEntries
        | Self::ObjectEntriesSkipFastPath
        | Self::ObjectIsExtensible
        | Self::JSReceiverPreventExtensionsThrow
        | Self::JSReceiverPreventExtensionsDontThrow
        | Self::JSReceiverGetPrototypeOf
        | Self::JSReceiverSetPrototypeOfThrow
        | Self::JSReceiverSetPrototypeOfDontThrow
        | Self::GetProperty
        | Self::SetKeyedProperty
        | Self::DefineObjectOwnProperty
        | Self::SetNamedProperty
        | Self::DeleteProperty
        | Self::ShrinkNameDictionary
        | Self::ShrinkSwissNameDictionary
        | Self::HasProperty
        | Self::GetOwnPropertyKeys
        | Self::ToFastProperties
        | Self::AllocateHeapNumber
        | Self::NewObject
        | Self::GetDerivedMap
        | Self::CompleteInobjectSlackTrackingForMap
        | Self::TryMigrateInstance
        | Self::TryMigrateInstanceAndMarkMapAsMigrationTarget
        | Self::DefineAccessorPropertyUnchecked
        | Self::SetFunctionName
        | Self::DefineKeyedOwnPropertyInLiteral
        | Self::HasFastPackedElements
        | Self::IsJSReceiver
        | Self::GetFunctionName
        | Self::DefineGetterPropertyUnchecked
        | Self::SetDataProperties
        | Self::CopyDataProperties
        | Self::CopyDataPropertiesWithExcludedPropertiesOnStack
        | Self::DefineSetterPropertyUnchecked
        | Self::ToObject
        | Self::ToNumber
        | Self::ToNumeric
        | Self::ToLength
        | Self::ToString
        | Self::ToName
        | Self::HasInPrototypeChain
        | Self::CreateIterResultObject
        | Self::CreateDataProperty
        | Self::SetOwnPropertyIgnoreAttributes
        | Self::GetOwnPropertyDescriptorObject
        | Self::GetPrivateMember
        | Self::SetPrivateMember
        | Self::LoadPrivateSetter
        | Self::LoadPrivateGetter
        | Self::CreatePrivateAccessors
        | Self::SwissTableAllocate
        | Self::SwissTableAdd
        | Self::SwissTableFindEntry
        | Self::SwissTableUpdate
        | Self::SwissTableDelete
        | Self::SwissTableEquals
        | Self::SwissTableElementsCount
        | Self::SwissTableKeyAt
        | Self::SwissTableValueAt
        | Self::SwissTableDetailsAt
        | Self::IterableToListWithInterceptor
        | Self::CheckFastIterableToListPrototype
    )
  }

  pub const fn is_array(&self) -> bool {
    matches!(
      self,
      Self::TransitionElementsKind
        | Self::TransitionElementsKindWithKind
        | Self::NewArray
        | Self::NormalizeElements
        | Self::GrowArrayElements
        | Self::ArrayIsArray
        | Self::IsArray
        | Self::ArraySpeciesConstructor
        | Self::ArrayIncludesSlow
        | Self::ArrayIndexOf
    )
  }

  pub const fn is_collections(&self) -> bool {
    matches!(
      self,
      Self::TheHole
        | Self::OrderedHashSetGrow
        | Self::SetGrow
        | Self::SetShrink
        | Self::OrderedHashSetShrink
        | Self::MapShrink
        | Self::MapGrow
        | Self::OrderedHashMapGrow
        | Self::WeakCollectionDelete
        | Self::WeakCollectionSet
    )
  }

  pub const fn is_typedarray(&self) -> bool {
    matches!(
      self,
      Self::ArrayBufferDetach
        | Self::ArrayBufferSetDetachKey
        | Self::TypedArrayCopyElements
        | Self::TypedArrayGetBuffer
        | Self::GrowableSharedArrayBufferByteLength
        | Self::TypedArraySortFast
        | Self::TypedArraySet
        | Self::ArrayBufferMaxByteLength
    )
  }

  pub const fn is_classes(&self) -> bool {
    matches!(
      self,
      Self::ThrowUnsupportedSuperError
        | Self::ThrowConstructorNonCallableError
        | Self::ThrowStaticPrototypeError
        | Self::ThrowSuperAlreadyCalledError
        | Self::ThrowSuperNotCalled
        | Self::ThrowNotSuperConstructor
        | Self::DefineClass
        | Self::LoadFromSuper
        | Self::LoadKeyedFromSuper
        | Self::StoreToSuper
        | Self::StoreKeyedToSuper
    )
  }

  pub const fn is_proxy(&self) -> bool {
    matches!(
      self,
      Self::IsJSProxy
        | Self::JSProxyGetHandler
        | Self::JSProxyGetTarget
        | Self::GetPropertyWithReceiver
        | Self::SetPropertyWithReceiver
        | Self::CheckProxyGetSetTrapResult
        | Self::CheckProxyHasTrapResult
        | Self::CheckProxyDeleteTrapResult
    )
  }
}

pub const fn object_keys() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ObjectKeys
}

pub const fn object_get_own_property_names() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ObjectGetOwnPropertyNames
}

pub const fn object_get_own_property_names_try_fast() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ObjectGetOwnPropertyNamesTryFast
}

pub const fn object_has_own_property() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ObjectHasOwnProperty
}

pub const fn has_own_const_data_property() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::HasOwnConstDataProperty
}

pub const fn is_dict_property_const_tracking_enabled() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::IsDictPropertyConstTrackingEnabled
}

pub const fn add_dictionary_property() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::AddDictionaryProperty
}

pub const fn add_private_brand() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::AddPrivateBrand
}

pub const fn object_create() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ObjectCreate
}

pub const fn internal_set_prototype() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::InternalSetPrototype
}

pub const fn optimize_object_for_adding_multiple_properties() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::OptimizeObjectForAddingMultipleProperties
}

pub const fn object_values() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ObjectValues
}

pub const fn object_values_skip_fast_path() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ObjectValuesSkipFastPath
}

pub const fn object_entries() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ObjectEntries
}

pub const fn object_entries_skip_fast_path() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ObjectEntriesSkipFastPath
}

pub const fn object_is_extensible() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ObjectIsExtensible
}

pub const fn js_receiver_prevent_extensions_throw() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::JSReceiverPreventExtensionsThrow
}

pub const fn js_receiver_prevent_extensions_dont_throw() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::JSReceiverPreventExtensionsDontThrow
}

pub const fn js_receiver_get_prototype_of() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::JSReceiverGetPrototypeOf
}

pub const fn js_receiver_set_prototype_of_throw() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::JSReceiverSetPrototypeOfThrow
}

pub const fn js_receiver_set_prototype_of_dont_throw() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::JSReceiverSetPrototypeOfDontThrow
}

pub const fn get_property() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::GetProperty
}

pub const fn set_keyed_property() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SetKeyedProperty
}

pub const fn define_object_own_property() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::DefineObjectOwnProperty
}

pub const fn set_named_property() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SetNamedProperty
}

pub const fn delete_property() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::DeleteProperty
}

pub const fn shrink_name_dictionary() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ShrinkNameDictionary
}

pub const fn shrink_swiss_name_dictionary() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ShrinkSwissNameDictionary
}

pub const fn has_property() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::HasProperty
}

pub const fn get_own_property_keys() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::GetOwnPropertyKeys
}

pub const fn to_fast_properties() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ToFastProperties
}

pub const fn allocate_heap_number() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::AllocateHeapNumber
}

pub const fn new_object() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::NewObject
}

pub const fn get_derived_map() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::GetDerivedMap
}

pub const fn complete_inobject_slack_tracking_for_map() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::CompleteInobjectSlackTrackingForMap
}

pub const fn try_migrate_instance() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::TryMigrateInstance
}

pub const fn try_migrate_instance_and_mark_map_as_migration_target() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::TryMigrateInstanceAndMarkMapAsMigrationTarget
}

pub const fn define_accessor_property_unchecked() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::DefineAccessorPropertyUnchecked
}

pub const fn set_function_name() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SetFunctionName
}

pub const fn define_keyed_own_property_in_literal() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::DefineKeyedOwnPropertyInLiteral
}

pub const fn has_fast_packed_elements() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::HasFastPackedElements
}

pub const fn is_js_receiver() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::IsJSReceiver
}

pub const fn get_function_name() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::GetFunctionName
}

pub const fn define_getter_property_unchecked() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::DefineGetterPropertyUnchecked
}

pub const fn set_data_properties() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SetDataProperties
}

pub const fn copy_data_properties() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::CopyDataProperties
}

pub const fn copy_data_properties_with_excluded_properties_on_stack() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::CopyDataPropertiesWithExcludedPropertiesOnStack
}

pub const fn define_setter_property_unchecked() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::DefineSetterPropertyUnchecked
}

pub const fn to_object() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ToObject
}

pub const fn to_number() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ToNumber
}

pub const fn to_numeric() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ToNumeric
}

pub const fn to_length() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ToLength
}

pub const fn to_string() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ToString
}

pub const fn to_name() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ToName
}

pub const fn has_in_prototype_chain() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::HasInPrototypeChain
}

pub const fn create_iter_result_object() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::CreateIterResultObject
}

pub const fn create_data_property() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::CreateDataProperty
}

pub const fn set_own_property_ignore_attributes() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SetOwnPropertyIgnoreAttributes
}

pub const fn get_own_property_descriptor_object() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::GetOwnPropertyDescriptorObject
}

pub const fn get_private_member() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::GetPrivateMember
}

pub const fn set_private_member() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SetPrivateMember
}

pub const fn load_private_setter() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::LoadPrivateSetter
}

pub const fn load_private_getter() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::LoadPrivateGetter
}

pub const fn create_private_accessors() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::CreatePrivateAccessors
}

pub const fn swiss_table_allocate() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SwissTableAllocate
}

pub const fn swiss_table_add() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SwissTableAdd
}

pub const fn swiss_table_find_entry() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SwissTableFindEntry
}

pub const fn swiss_table_update() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SwissTableUpdate
}

pub const fn swiss_table_delete() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SwissTableDelete
}

pub const fn swiss_table_equals() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SwissTableEquals
}

pub const fn swiss_table_elements_count() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SwissTableElementsCount
}

pub const fn swiss_table_key_at() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SwissTableKeyAt
}

pub const fn swiss_table_value_at() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SwissTableValueAt
}

pub const fn swiss_table_details_at() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SwissTableDetailsAt
}

pub const fn iterable_to_list_with_interceptor() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::IterableToListWithInterceptor
}

pub const fn check_fast_iterable_to_list_prototype() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::CheckFastIterableToListPrototype
}

pub const fn transition_elements_kind() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::TransitionElementsKind
}

pub const fn transition_elements_kind_with_kind() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::TransitionElementsKindWithKind
}

pub const fn new_array() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::NewArray
}

pub const fn normalize_elements() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::NormalizeElements
}

pub const fn grow_array_elements() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::GrowArrayElements
}

pub const fn array_is_array() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ArrayIsArray
}

pub const fn is_array() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::IsArray
}

pub const fn array_species_constructor() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ArraySpeciesConstructor
}

pub const fn array_includes_slow() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ArrayIncludesSlow
}

pub const fn array_index_of() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ArrayIndexOf
}

pub const fn the_hole() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::TheHole
}

pub const fn ordered_hash_set_grow() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::OrderedHashSetGrow
}

pub const fn set_grow() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SetGrow
}

pub const fn set_shrink() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SetShrink
}

pub const fn ordered_hash_set_shrink() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::OrderedHashSetShrink
}

pub const fn map_shrink() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::MapShrink
}

pub const fn map_grow() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::MapGrow
}

pub const fn ordered_hash_map_grow() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::OrderedHashMapGrow
}

pub const fn weak_collection_delete() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::WeakCollectionDelete
}

pub const fn weak_collection_set() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::WeakCollectionSet
}

pub const fn array_buffer_detach() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ArrayBufferDetach
}

pub const fn array_buffer_set_detach_key() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ArrayBufferSetDetachKey
}

pub const fn typed_array_copy_elements() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::TypedArrayCopyElements
}

pub const fn typed_array_get_buffer() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::TypedArrayGetBuffer
}

pub const fn growable_shared_array_buffer_byte_length() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::GrowableSharedArrayBufferByteLength
}

pub const fn typed_array_sort_fast() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::TypedArraySortFast
}

pub const fn typed_array_set() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::TypedArraySet
}

pub const fn array_buffer_max_byte_length() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ArrayBufferMaxByteLength
}

pub const fn throw_unsupported_super_error() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ThrowUnsupportedSuperError
}

pub const fn throw_constructor_non_callable_error() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ThrowConstructorNonCallableError
}

pub const fn throw_static_prototype_error() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ThrowStaticPrototypeError
}

pub const fn throw_super_already_called_error() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ThrowSuperAlreadyCalledError
}

pub const fn throw_super_not_called() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ThrowSuperNotCalled
}

pub const fn throw_not_super_constructor() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::ThrowNotSuperConstructor
}

pub const fn define_class() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::DefineClass
}

pub const fn load_from_super() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::LoadFromSuper
}

pub const fn load_keyed_from_super() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::LoadKeyedFromSuper
}

pub const fn store_to_super() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::StoreToSuper
}

pub const fn store_keyed_to_super() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::StoreKeyedToSuper
}

pub const fn is_js_proxy() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::IsJSProxy
}

pub const fn js_proxy_get_handler() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::JSProxyGetHandler
}

pub const fn js_proxy_get_target() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::JSProxyGetTarget
}

pub const fn get_property_with_receiver() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::GetPropertyWithReceiver
}

pub const fn set_property_with_receiver() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::SetPropertyWithReceiver
}

pub const fn check_proxy_get_set_trap_result() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::CheckProxyGetSetTrapResult
}

pub const fn check_proxy_has_trap_result() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::CheckProxyHasTrapResult
}

pub const fn check_proxy_delete_trap_result() -> ObjectRuntimeFunction {
  ObjectRuntimeFunction::CheckProxyDeleteTrapResult
}
