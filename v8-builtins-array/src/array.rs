// 由 tool/src/builtins/builtins-array.js 生成; 来源 v8/src/builtins 下 33 文件
// 内建名表 + 核心调度元数据 (33 内建)
// 来源: builtins-array.cc, builtins-array-gen.cc, builtins-array-gen.h, array.tq, array-at.tq, array-concat.tq 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// 内建名表 (来源: builtins-array.cc, builtins-array-gen.cc, builtins-array-gen.h, array.tq, array-at.tq, array-concat.tq 等)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  ArrayConcat,
  ArrayConstructor,
  ArrayConstructorImpl,
  ArrayIncludes,
  ArrayIncludesHoleyDoubles,
  ArrayIncludesPackedDoubles,
  ArrayIncludesSmi,
  ArrayIncludesSmiOrObject,
  ArrayIndexOf,
  ArrayIndexOfHoleyDoubles,
  ArrayIndexOfPackedDoubles,
  ArrayIndexOfSmi,
  ArrayIndexOfSmiOrObject,
  ArrayIteratorPrototypeNext,
  ArrayNArgumentsConstructor,
  ArrayPop,
  ArrayPrototypeEntries,
  ArrayPrototypeFill,
  ArrayPrototypeKeys,
  ArrayPrototypePop,
  ArrayPrototypePush,
  ArrayPrototypeValues,
  ArrayPush,
  ArrayShift,
  ArrayUnshift,
  CloneFastJSArray,
  CloneFastJSArrayFillingHoles,
  CreateArrayFromSlowBoilerplate,
  CreateArrayFromSlowBoilerplateHelper,
  CreateObjectFromSlowBoilerplate,
  CreateObjectFromSlowBoilerplateHelper,
  ExtractFastJSArray,
  TypedArrayPrototypeMap,
}

impl Builtin {
  /// 获取内建的 PascalCase 规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::ArrayConcat => "ArrayConcat",
      Self::ArrayConstructor => "ArrayConstructor",
      Self::ArrayConstructorImpl => "ArrayConstructorImpl",
      Self::ArrayIncludes => "ArrayIncludes",
      Self::ArrayIncludesHoleyDoubles => "ArrayIncludesHoleyDoubles",
      Self::ArrayIncludesPackedDoubles => "ArrayIncludesPackedDoubles",
      Self::ArrayIncludesSmi => "ArrayIncludesSmi",
      Self::ArrayIncludesSmiOrObject => "ArrayIncludesSmiOrObject",
      Self::ArrayIndexOf => "ArrayIndexOf",
      Self::ArrayIndexOfHoleyDoubles => "ArrayIndexOfHoleyDoubles",
      Self::ArrayIndexOfPackedDoubles => "ArrayIndexOfPackedDoubles",
      Self::ArrayIndexOfSmi => "ArrayIndexOfSmi",
      Self::ArrayIndexOfSmiOrObject => "ArrayIndexOfSmiOrObject",
      Self::ArrayIteratorPrototypeNext => "ArrayIteratorPrototypeNext",
      Self::ArrayNArgumentsConstructor => "ArrayNArgumentsConstructor",
      Self::ArrayPop => "ArrayPop",
      Self::ArrayPrototypeEntries => "ArrayPrototypeEntries",
      Self::ArrayPrototypeFill => "ArrayPrototypeFill",
      Self::ArrayPrototypeKeys => "ArrayPrototypeKeys",
      Self::ArrayPrototypePop => "ArrayPrototypePop",
      Self::ArrayPrototypePush => "ArrayPrototypePush",
      Self::ArrayPrototypeValues => "ArrayPrototypeValues",
      Self::ArrayPush => "ArrayPush",
      Self::ArrayShift => "ArrayShift",
      Self::ArrayUnshift => "ArrayUnshift",
      Self::CloneFastJSArray => "CloneFastJSArray",
      Self::CloneFastJSArrayFillingHoles => "CloneFastJSArrayFillingHoles",
      Self::CreateArrayFromSlowBoilerplate => "CreateArrayFromSlowBoilerplate",
      Self::CreateArrayFromSlowBoilerplateHelper => "CreateArrayFromSlowBoilerplateHelper",
      Self::CreateObjectFromSlowBoilerplate => "CreateObjectFromSlowBoilerplate",
      Self::CreateObjectFromSlowBoilerplateHelper => "CreateObjectFromSlowBoilerplateHelper",
      Self::ExtractFastJSArray => "ExtractFastJSArray",
      Self::TypedArrayPrototypeMap => "TypedArrayPrototypeMap",
    }
  }

  /// 获取对应的接口描述符符号名称 (Builtin_<Name>_InterfaceDescriptor)
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 在 v8-builtins-core 全局表中检索内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 是否属于 Array.prototype 上的原型方法
  #[inline]
  pub const fn is_prototype_method(&self) -> bool {
    matches!(
      self,
      Self::ArrayPrototypeEntries
        | Self::ArrayPrototypeFill
        | Self::ArrayPrototypeKeys
        | Self::ArrayPrototypePop
        | Self::ArrayPrototypePush
        | Self::ArrayPrototypeValues
        | Self::ArrayIteratorPrototypeNext
    )
  }

  /// 是否为查找搜索类内建 (IndexOf / Includes 族)
  #[inline]
  pub const fn is_search_method(&self) -> bool {
    matches!(
      self,
      Self::ArrayIncludes
        | Self::ArrayIncludesHoleyDoubles
        | Self::ArrayIncludesPackedDoubles
        | Self::ArrayIncludesSmi
        | Self::ArrayIncludesSmiOrObject
        | Self::ArrayIndexOf
        | Self::ArrayIndexOfHoleyDoubles
        | Self::ArrayIndexOfPackedDoubles
        | Self::ArrayIndexOfSmi
        | Self::ArrayIndexOfSmiOrObject
    )
  }

  /// 是否为快数组克隆/提取类内建
  #[inline]
  pub const fn is_fast_clone_method(&self) -> bool {
    matches!(
      self,
      Self::CloneFastJSArray | Self::CloneFastJSArrayFillingHoles | Self::ExtractFastJSArray
    )
  }

  /// 是否为构造函数类内建
  #[inline]
  pub const fn is_constructor(&self) -> bool {
    matches!(
      self,
      Self::ArrayConstructor | Self::ArrayConstructorImpl | Self::ArrayNArgumentsConstructor
    )
  }
}

// 调度入口 (安全常量构造)
pub const fn array_concat() -> Builtin {
  Builtin::ArrayConcat
}

pub const fn array_constructor() -> Builtin {
  Builtin::ArrayConstructor
}

pub const fn array_constructor_impl() -> Builtin {
  Builtin::ArrayConstructorImpl
}

pub const fn array_includes() -> Builtin {
  Builtin::ArrayIncludes
}

pub const fn array_includes_holey_doubles() -> Builtin {
  Builtin::ArrayIncludesHoleyDoubles
}

pub const fn array_includes_packed_doubles() -> Builtin {
  Builtin::ArrayIncludesPackedDoubles
}

pub const fn array_includes_smi() -> Builtin {
  Builtin::ArrayIncludesSmi
}

pub const fn array_includes_smi_or_object() -> Builtin {
  Builtin::ArrayIncludesSmiOrObject
}

pub const fn array_index_of() -> Builtin {
  Builtin::ArrayIndexOf
}

pub const fn array_index_of_holey_doubles() -> Builtin {
  Builtin::ArrayIndexOfHoleyDoubles
}

pub const fn array_index_of_packed_doubles() -> Builtin {
  Builtin::ArrayIndexOfPackedDoubles
}

pub const fn array_index_of_smi() -> Builtin {
  Builtin::ArrayIndexOfSmi
}

pub const fn array_index_of_smi_or_object() -> Builtin {
  Builtin::ArrayIndexOfSmiOrObject
}

pub const fn array_iterator_prototype_next() -> Builtin {
  Builtin::ArrayIteratorPrototypeNext
}

pub const fn array_n_arguments_constructor() -> Builtin {
  Builtin::ArrayNArgumentsConstructor
}

pub const fn array_pop() -> Builtin {
  Builtin::ArrayPop
}

pub const fn array_prototype_entries() -> Builtin {
  Builtin::ArrayPrototypeEntries
}

pub const fn array_prototype_fill() -> Builtin {
  Builtin::ArrayPrototypeFill
}

pub const fn array_prototype_keys() -> Builtin {
  Builtin::ArrayPrototypeKeys
}

pub const fn array_prototype_pop() -> Builtin {
  Builtin::ArrayPrototypePop
}

pub const fn array_prototype_push() -> Builtin {
  Builtin::ArrayPrototypePush
}

pub const fn array_prototype_values() -> Builtin {
  Builtin::ArrayPrototypeValues
}

pub const fn array_push() -> Builtin {
  Builtin::ArrayPush
}

pub const fn array_shift() -> Builtin {
  Builtin::ArrayShift
}

pub const fn array_unshift() -> Builtin {
  Builtin::ArrayUnshift
}

pub const fn clone_fast_js_array() -> Builtin {
  Builtin::CloneFastJSArray
}

pub const fn clone_fast_js_array_filling_holes() -> Builtin {
  Builtin::CloneFastJSArrayFillingHoles
}

pub const fn create_array_from_slow_boilerplate() -> Builtin {
  Builtin::CreateArrayFromSlowBoilerplate
}

pub const fn create_array_from_slow_boilerplate_helper() -> Builtin {
  Builtin::CreateArrayFromSlowBoilerplateHelper
}

pub const fn create_object_from_slow_boilerplate() -> Builtin {
  Builtin::CreateObjectFromSlowBoilerplate
}

pub const fn create_object_from_slow_boilerplate_helper() -> Builtin {
  Builtin::CreateObjectFromSlowBoilerplateHelper
}

pub const fn extract_fast_js_array() -> Builtin {
  Builtin::ExtractFastJSArray
}

pub const fn typed_array_prototype_map() -> Builtin {
  Builtin::TypedArrayPrototypeMap
}

/// .tq 声明 67 条 (macro/struct/type/extern)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("type", "GenericElementsAccessor"),
  ("type", "FastPackedSmiElements"),
  ("type", "FastPackedObjectElements"),
  ("type", "FastPackedDoubleElements"),
  ("type", "FastSmiOrObjectElements"),
  ("type", "FastDoubleElements"),
  ("type", "DictionaryElements"),
  ("macro", "EnsureWriteableFastElements"),
  ("macro", "LoadElementOrUndefined"),
  ("macro", "LoadElementOrUndefined"),
  ("macro", "StoreArrayHole"),
  ("macro", "StoreArrayHole"),
  ("extern macro", "SetPropertyLength"),
  ("macro", "EnsureArrayLengthWritable"),
  ("macro", "CreateJSArrayWithElements"),
  ("macro", "ConvertRelativeIndex"),
  ("macro", "ConvertAndClampRelativeIndex"),
  ("macro", "TryFastArrayCopyWithin"),
  ("macro", "FastFilterSpeciesCreate"),
  ("struct", "FlatVector"),
  ("macro", "NewFlatVector"),
  ("struct", "FlattenedLengthResult"),
  ("javascript builtin", "ArrayIsArray"),
  ("type", "LoadJoinElementFn"),
  ("extern macro", "ArrayBuiltinsAssembler"),
  ("builtin", "LoadJoinTypedElement"),
  ("macro", "CannotUseSameArrayAccessor"),
  ("macro", "AddStringLength"),
  ("macro", "StoreAndGrowFixedArray"),
  ("struct", "Buffer"),
  ("macro", "NewBuffer"),
  ("macro", "BufferJoin"),
  ("macro", "LoadJoinStack"),
  ("macro", "SetJoinStack"),
  ("builtin", "JoinStackPush"),
  ("macro", "JoinStackPushInline"),
  ("builtin", "JoinStackPop"),
  ("macro", "JoinStackPopInline"),
  ("macro", "ClampLastIndexOfFrom"),
  ("macro", "LastIndexOfNumberInFixedArray"),
  ("macro", "LastIndexOfAnyInFixedArray"),
  ("macro", "FastArrayLastIndexOfFixedArray"),
  ("macro", "LastIndexOfDoubleValue"),
  ("macro", "LastIndexOfUndefinedInDoubles"),
  ("macro", "FastArrayLastIndexOfDoubles"),
  ("macro", "TryFastArrayLastIndexOf"),
  ("struct", "Vector"),
  ("macro", "NewVector"),
  ("macro", "LoadElement"),
  ("macro", "StoreElement"),
  ("macro", "FastArrayReverse"),
  ("macro", "TryFastPackedArrayReverse"),
  ("macro", "TryFastArrayShift"),
  ("macro", "HandleSimpleArgumentsSlice"),
  ("macro", "HandleFastAliasedSloppyArgumentsSlice"),
  ("macro", "HandleFastSlice"),
  ("macro", "Extract"),
  ("macro", "Extract"),
  ("macro", "DoMoveElements"),
  ("macro", "StoreHoles"),
  ("macro", "DoCopyElements"),
  ("macro", "InsertArgumentsIntoFastPackedArray"),
  ("macro", "FastSplice"),
  ("macro", "FastPackedDoubleArrayToReversed"),
  ("macro", "FastArrayToReversed"),
  ("macro", "TryFastArrayToReversed"),
  ("macro", "CopyFastPackedArrayForToSpliced"),
];
