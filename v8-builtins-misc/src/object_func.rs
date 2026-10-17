// 由 tool/src/builtins/builtins-misc.js 生成; 来源 v8/src/builtins
// 内建名表 + 核心调度元数据 (103 内建)

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// 内建名表 (共 103 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  AsyncFunctionConstructor,
  AsyncGeneratorFunctionConstructor,
  CallSitePrototypeGetColumnNumber,
  CallSitePrototypeGetEnclosingColumnNumber,
  CallSitePrototypeGetEnclosingLineNumber,
  CallSitePrototypeGetEvalOrigin,
  CallSitePrototypeGetFileName,
  CallSitePrototypeGetFunction,
  CallSitePrototypeGetFunctionName,
  CallSitePrototypeGetLineNumber,
  CallSitePrototypeGetMethodName,
  CallSitePrototypeGetPosition,
  CallSitePrototypeGetPromiseIndex,
  CallSitePrototypeGetScriptHash,
  CallSitePrototypeGetScriptNameOrSourceURL,
  CallSitePrototypeGetThis,
  CallSitePrototypeGetTypeName,
  CallSitePrototypeIsAsync,
  CallSitePrototypeIsConstructor,
  CallSitePrototypeIsEval,
  CallSitePrototypeIsNative,
  CallSitePrototypeIsPromiseAll,
  CallSitePrototypeIsToplevel,
  CallSitePrototypeToString,
  ConstructForwardAllArgs,
  #[strum(serialize = "ConstructForwardAllArgs_Baseline")]
  ConstructForwardAllArgsBaseline,
  #[strum(serialize = "ConstructForwardAllArgs_WithFeedback")]
  ConstructForwardAllArgsWithFeedback,
  ConstructForwardVarargs,
  ConstructFunctionForwardVarargs,
  ConstructVarargs,
  ConstructWithArrayLike,
  ConstructWithSpread,
  #[strum(serialize = "ConstructWithSpread_Baseline")]
  ConstructWithSpreadBaseline,
  #[strum(serialize = "ConstructWithSpread_WithFeedback")]
  ConstructWithSpreadWithFeedback,
  #[strum(serialize = "Construct_Baseline")]
  ConstructBaseline,
  #[strum(serialize = "Construct_WithFeedback")]
  ConstructWithFeedback,
  CreateGeneratorObject,
  CreateIterResultObject,
  ErrorCaptureStackTrace,
  ErrorConstructor,
  ErrorIsError,
  ErrorPrototypeToString,
  FastNewClosure,
  FastNewObject,
  FunctionConstructor,
  FunctionPrototypeBind,
  FunctionPrototypeLegacyArgumentsGetter,
  FunctionPrototypeLegacyArgumentsSetter,
  FunctionPrototypeLegacyCallerGetter,
  FunctionPrototypeLegacyCallerSetter,
  FunctionPrototypeToString,
  GeneratorFunctionConstructor,
  HasProperty,
  InstanceOf,
  #[strum(serialize = "InstanceOf_Baseline")]
  InstanceOfBaseline,
  #[strum(serialize = "InstanceOf_WithFeedback")]
  InstanceOfWithFeedback,
  InterpreterForwardAllArgsThenConstruct,
  MathCeilContinuation,
  MathClz32Continuation,
  MathFloorContinuation,
  MathRoundContinuation,
  ObjectAssign,
  ObjectCreate,
  ObjectDefineGetter,
  ObjectDefineProperties,
  ObjectDefineProperty,
  ObjectDefineSetter,
  ObjectEntries,
  ObjectFreeze,
  ObjectGetOwnPropertyDescriptor,
  ObjectGetOwnPropertyDescriptors,
  ObjectGetOwnPropertyNames,
  ObjectGetOwnPropertySymbols,
  ObjectHasOwn,
  ObjectIs,
  ObjectIsFrozen,
  ObjectIsSealed,
  ObjectKeys,
  ObjectLookupGetter,
  ObjectLookupSetter,
  ObjectPrototypeGetProto,
  ObjectPrototypeHasOwnProperty,
  ObjectPrototypeIsPrototypeOf,
  ObjectPrototypePropertyIsEnumerable,
  ObjectPrototypeSetProto,
  ObjectSeal,
  ObjectToString,
  ObjectValues,
  OrdinaryGetOwnPropertyDescriptor,
  OrdinaryHasInstance,
  PlainPrimitiveToNumber,
  SymbolConstructor,
  SymbolFor,
  SymbolKeyFor,
  ToBigInt,
  ToBigIntConvertNumber,
  ToBooleanLazyDeoptContinuation,
  ToNumber,
  ToNumberConvertBigInt,
  #[strum(serialize = "ToNumber_Baseline")]
  ToNumberBaseline,
  #[strum(serialize = "ToNumeric_Baseline")]
  ToNumericBaseline,
  Typeof,
  #[strum(serialize = "Typeof_Baseline")]
  TypeofBaseline,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::AsyncFunctionConstructor => "AsyncFunctionConstructor",
      Self::AsyncGeneratorFunctionConstructor => "AsyncGeneratorFunctionConstructor",
      Self::CallSitePrototypeGetColumnNumber => "CallSitePrototypeGetColumnNumber",
      Self::CallSitePrototypeGetEnclosingColumnNumber => {
        "CallSitePrototypeGetEnclosingColumnNumber"
      }
      Self::CallSitePrototypeGetEnclosingLineNumber => "CallSitePrototypeGetEnclosingLineNumber",
      Self::CallSitePrototypeGetEvalOrigin => "CallSitePrototypeGetEvalOrigin",
      Self::CallSitePrototypeGetFileName => "CallSitePrototypeGetFileName",
      Self::CallSitePrototypeGetFunction => "CallSitePrototypeGetFunction",
      Self::CallSitePrototypeGetFunctionName => "CallSitePrototypeGetFunctionName",
      Self::CallSitePrototypeGetLineNumber => "CallSitePrototypeGetLineNumber",
      Self::CallSitePrototypeGetMethodName => "CallSitePrototypeGetMethodName",
      Self::CallSitePrototypeGetPosition => "CallSitePrototypeGetPosition",
      Self::CallSitePrototypeGetPromiseIndex => "CallSitePrototypeGetPromiseIndex",
      Self::CallSitePrototypeGetScriptHash => "CallSitePrototypeGetScriptHash",
      Self::CallSitePrototypeGetScriptNameOrSourceURL => {
        "CallSitePrototypeGetScriptNameOrSourceURL"
      }
      Self::CallSitePrototypeGetThis => "CallSitePrototypeGetThis",
      Self::CallSitePrototypeGetTypeName => "CallSitePrototypeGetTypeName",
      Self::CallSitePrototypeIsAsync => "CallSitePrototypeIsAsync",
      Self::CallSitePrototypeIsConstructor => "CallSitePrototypeIsConstructor",
      Self::CallSitePrototypeIsEval => "CallSitePrototypeIsEval",
      Self::CallSitePrototypeIsNative => "CallSitePrototypeIsNative",
      Self::CallSitePrototypeIsPromiseAll => "CallSitePrototypeIsPromiseAll",
      Self::CallSitePrototypeIsToplevel => "CallSitePrototypeIsToplevel",
      Self::CallSitePrototypeToString => "CallSitePrototypeToString",
      Self::ConstructForwardAllArgs => "ConstructForwardAllArgs",
      Self::ConstructForwardAllArgsBaseline => "ConstructForwardAllArgs_Baseline",
      Self::ConstructForwardAllArgsWithFeedback => "ConstructForwardAllArgs_WithFeedback",
      Self::ConstructForwardVarargs => "ConstructForwardVarargs",
      Self::ConstructFunctionForwardVarargs => "ConstructFunctionForwardVarargs",
      Self::ConstructVarargs => "ConstructVarargs",
      Self::ConstructWithArrayLike => "ConstructWithArrayLike",
      Self::ConstructWithSpread => "ConstructWithSpread",
      Self::ConstructWithSpreadBaseline => "ConstructWithSpread_Baseline",
      Self::ConstructWithSpreadWithFeedback => "ConstructWithSpread_WithFeedback",
      Self::ConstructBaseline => "Construct_Baseline",
      Self::ConstructWithFeedback => "Construct_WithFeedback",
      Self::CreateGeneratorObject => "CreateGeneratorObject",
      Self::CreateIterResultObject => "CreateIterResultObject",
      Self::ErrorCaptureStackTrace => "ErrorCaptureStackTrace",
      Self::ErrorConstructor => "ErrorConstructor",
      Self::ErrorIsError => "ErrorIsError",
      Self::ErrorPrototypeToString => "ErrorPrototypeToString",
      Self::FastNewClosure => "FastNewClosure",
      Self::FastNewObject => "FastNewObject",
      Self::FunctionConstructor => "FunctionConstructor",
      Self::FunctionPrototypeBind => "FunctionPrototypeBind",
      Self::FunctionPrototypeLegacyArgumentsGetter => "FunctionPrototypeLegacyArgumentsGetter",
      Self::FunctionPrototypeLegacyArgumentsSetter => "FunctionPrototypeLegacyArgumentsSetter",
      Self::FunctionPrototypeLegacyCallerGetter => "FunctionPrototypeLegacyCallerGetter",
      Self::FunctionPrototypeLegacyCallerSetter => "FunctionPrototypeLegacyCallerSetter",
      Self::FunctionPrototypeToString => "FunctionPrototypeToString",
      Self::GeneratorFunctionConstructor => "GeneratorFunctionConstructor",
      Self::HasProperty => "HasProperty",
      Self::InstanceOf => "InstanceOf",
      Self::InstanceOfBaseline => "InstanceOf_Baseline",
      Self::InstanceOfWithFeedback => "InstanceOf_WithFeedback",
      Self::InterpreterForwardAllArgsThenConstruct => "InterpreterForwardAllArgsThenConstruct",
      Self::MathCeilContinuation => "MathCeilContinuation",
      Self::MathClz32Continuation => "MathClz32Continuation",
      Self::MathFloorContinuation => "MathFloorContinuation",
      Self::MathRoundContinuation => "MathRoundContinuation",
      Self::ObjectAssign => "ObjectAssign",
      Self::ObjectCreate => "ObjectCreate",
      Self::ObjectDefineGetter => "ObjectDefineGetter",
      Self::ObjectDefineProperties => "ObjectDefineProperties",
      Self::ObjectDefineProperty => "ObjectDefineProperty",
      Self::ObjectDefineSetter => "ObjectDefineSetter",
      Self::ObjectEntries => "ObjectEntries",
      Self::ObjectFreeze => "ObjectFreeze",
      Self::ObjectGetOwnPropertyDescriptor => "ObjectGetOwnPropertyDescriptor",
      Self::ObjectGetOwnPropertyDescriptors => "ObjectGetOwnPropertyDescriptors",
      Self::ObjectGetOwnPropertyNames => "ObjectGetOwnPropertyNames",
      Self::ObjectGetOwnPropertySymbols => "ObjectGetOwnPropertySymbols",
      Self::ObjectHasOwn => "ObjectHasOwn",
      Self::ObjectIs => "ObjectIs",
      Self::ObjectIsFrozen => "ObjectIsFrozen",
      Self::ObjectIsSealed => "ObjectIsSealed",
      Self::ObjectKeys => "ObjectKeys",
      Self::ObjectLookupGetter => "ObjectLookupGetter",
      Self::ObjectLookupSetter => "ObjectLookupSetter",
      Self::ObjectPrototypeGetProto => "ObjectPrototypeGetProto",
      Self::ObjectPrototypeHasOwnProperty => "ObjectPrototypeHasOwnProperty",
      Self::ObjectPrototypeIsPrototypeOf => "ObjectPrototypeIsPrototypeOf",
      Self::ObjectPrototypePropertyIsEnumerable => "ObjectPrototypePropertyIsEnumerable",
      Self::ObjectPrototypeSetProto => "ObjectPrototypeSetProto",
      Self::ObjectSeal => "ObjectSeal",
      Self::ObjectToString => "ObjectToString",
      Self::ObjectValues => "ObjectValues",
      Self::OrdinaryGetOwnPropertyDescriptor => "OrdinaryGetOwnPropertyDescriptor",
      Self::OrdinaryHasInstance => "OrdinaryHasInstance",
      Self::PlainPrimitiveToNumber => "PlainPrimitiveToNumber",
      Self::SymbolConstructor => "SymbolConstructor",
      Self::SymbolFor => "SymbolFor",
      Self::SymbolKeyFor => "SymbolKeyFor",
      Self::ToBigInt => "ToBigInt",
      Self::ToBigIntConvertNumber => "ToBigIntConvertNumber",
      Self::ToBooleanLazyDeoptContinuation => "ToBooleanLazyDeoptContinuation",
      Self::ToNumber => "ToNumber",
      Self::ToNumberConvertBigInt => "ToNumberConvertBigInt",
      Self::ToNumberBaseline => "ToNumber_Baseline",
      Self::ToNumericBaseline => "ToNumeric_Baseline",
      Self::Typeof => "Typeof",
      Self::TypeofBaseline => "Typeof_Baseline",
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

  /// 是否为 CallSite 栈帧内建
  #[inline]
  pub const fn is_callsite(&self) -> bool {
    matches!(
      self,
      Self::CallSitePrototypeGetColumnNumber
        | Self::CallSitePrototypeGetEnclosingColumnNumber
        | Self::CallSitePrototypeGetEnclosingLineNumber
        | Self::CallSitePrototypeGetEvalOrigin
        | Self::CallSitePrototypeGetFileName
        | Self::CallSitePrototypeGetFunction
        | Self::CallSitePrototypeGetFunctionName
        | Self::CallSitePrototypeGetLineNumber
        | Self::CallSitePrototypeGetMethodName
        | Self::CallSitePrototypeGetPosition
        | Self::CallSitePrototypeGetPromiseIndex
        | Self::CallSitePrototypeGetScriptHash
        | Self::CallSitePrototypeGetScriptNameOrSourceURL
        | Self::CallSitePrototypeGetThis
        | Self::CallSitePrototypeGetTypeName
        | Self::CallSitePrototypeIsAsync
        | Self::CallSitePrototypeIsConstructor
        | Self::CallSitePrototypeIsEval
        | Self::CallSitePrototypeIsNative
        | Self::CallSitePrototypeIsPromiseAll
        | Self::CallSitePrototypeIsToplevel
        | Self::CallSitePrototypeToString
    )
  }

  /// 是否为构造器相关内建
  #[inline]
  pub const fn is_constructor(&self) -> bool {
    matches!(
      self,
      Self::AsyncFunctionConstructor
        | Self::AsyncGeneratorFunctionConstructor
        | Self::ConstructForwardAllArgs
        | Self::ConstructForwardAllArgsBaseline
        | Self::ConstructForwardAllArgsWithFeedback
        | Self::ConstructForwardVarargs
        | Self::ConstructFunctionForwardVarargs
        | Self::ConstructVarargs
        | Self::ConstructWithArrayLike
        | Self::ConstructWithSpread
        | Self::ConstructWithSpreadBaseline
        | Self::ConstructWithSpreadWithFeedback
        | Self::ConstructBaseline
        | Self::ConstructWithFeedback
        | Self::ErrorConstructor
        | Self::FunctionConstructor
        | Self::GeneratorFunctionConstructor
        | Self::SymbolConstructor
    )
  }

  /// 是否为 Function 原型方法
  #[inline]
  pub const fn is_function_prototype(&self) -> bool {
    matches!(
      self,
      Self::FunctionPrototypeBind
        | Self::FunctionPrototypeLegacyArgumentsGetter
        | Self::FunctionPrototypeLegacyArgumentsSetter
        | Self::FunctionPrototypeLegacyCallerGetter
        | Self::FunctionPrototypeLegacyCallerSetter
        | Self::FunctionPrototypeToString
    )
  }

  /// 是否为 Error 异常内建
  #[inline]
  pub const fn is_error(&self) -> bool {
    matches!(
      self,
      Self::ErrorCaptureStackTrace
        | Self::ErrorConstructor
        | Self::ErrorIsError
        | Self::ErrorPrototypeToString
    )
  }

  /// 是否为 Baseline 或包含反馈内建
  #[inline]
  pub const fn is_baseline_or_feedback(&self) -> bool {
    matches!(
      self,
      Self::ConstructForwardAllArgsBaseline
        | Self::ConstructForwardAllArgsWithFeedback
        | Self::ConstructWithSpreadBaseline
        | Self::ConstructWithSpreadWithFeedback
        | Self::ConstructBaseline
        | Self::ConstructWithFeedback
        | Self::InstanceOfBaseline
        | Self::InstanceOfWithFeedback
        | Self::ToNumberBaseline
        | Self::ToNumericBaseline
        | Self::TypeofBaseline
    )
  }
}

// 统一提供 103 项零成本内建工厂函数
pub const fn async_function_constructor() -> Builtin {
  Builtin::AsyncFunctionConstructor
}

pub const fn async_generator_function_constructor() -> Builtin {
  Builtin::AsyncGeneratorFunctionConstructor
}

pub const fn call_site_prototype_get_column_number() -> Builtin {
  Builtin::CallSitePrototypeGetColumnNumber
}

pub const fn call_site_prototype_get_enclosing_column_number() -> Builtin {
  Builtin::CallSitePrototypeGetEnclosingColumnNumber
}

pub const fn call_site_prototype_get_enclosing_line_number() -> Builtin {
  Builtin::CallSitePrototypeGetEnclosingLineNumber
}

pub const fn call_site_prototype_get_eval_origin() -> Builtin {
  Builtin::CallSitePrototypeGetEvalOrigin
}

pub const fn call_site_prototype_get_file_name() -> Builtin {
  Builtin::CallSitePrototypeGetFileName
}

pub const fn call_site_prototype_get_function() -> Builtin {
  Builtin::CallSitePrototypeGetFunction
}

pub const fn call_site_prototype_get_function_name() -> Builtin {
  Builtin::CallSitePrototypeGetFunctionName
}

pub const fn call_site_prototype_get_line_number() -> Builtin {
  Builtin::CallSitePrototypeGetLineNumber
}

pub const fn call_site_prototype_get_method_name() -> Builtin {
  Builtin::CallSitePrototypeGetMethodName
}

pub const fn call_site_prototype_get_position() -> Builtin {
  Builtin::CallSitePrototypeGetPosition
}

pub const fn call_site_prototype_get_promise_index() -> Builtin {
  Builtin::CallSitePrototypeGetPromiseIndex
}

pub const fn call_site_prototype_get_script_hash() -> Builtin {
  Builtin::CallSitePrototypeGetScriptHash
}

pub const fn call_site_prototype_get_script_name_or_source_url() -> Builtin {
  Builtin::CallSitePrototypeGetScriptNameOrSourceURL
}

pub const fn call_site_prototype_get_this() -> Builtin {
  Builtin::CallSitePrototypeGetThis
}

pub const fn call_site_prototype_get_type_name() -> Builtin {
  Builtin::CallSitePrototypeGetTypeName
}

pub const fn call_site_prototype_is_async() -> Builtin {
  Builtin::CallSitePrototypeIsAsync
}

pub const fn call_site_prototype_is_constructor() -> Builtin {
  Builtin::CallSitePrototypeIsConstructor
}

pub const fn call_site_prototype_is_eval() -> Builtin {
  Builtin::CallSitePrototypeIsEval
}

pub const fn call_site_prototype_is_native() -> Builtin {
  Builtin::CallSitePrototypeIsNative
}

pub const fn call_site_prototype_is_promise_all() -> Builtin {
  Builtin::CallSitePrototypeIsPromiseAll
}

pub const fn call_site_prototype_is_toplevel() -> Builtin {
  Builtin::CallSitePrototypeIsToplevel
}

pub const fn call_site_prototype_to_string() -> Builtin {
  Builtin::CallSitePrototypeToString
}

pub const fn construct_forward_all_args() -> Builtin {
  Builtin::ConstructForwardAllArgs
}

pub const fn construct_forward_all_args_baseline() -> Builtin {
  Builtin::ConstructForwardAllArgsBaseline
}

pub const fn construct_forward_all_args_with_feedback() -> Builtin {
  Builtin::ConstructForwardAllArgsWithFeedback
}

pub const fn construct_forward_varargs() -> Builtin {
  Builtin::ConstructForwardVarargs
}

pub const fn construct_function_forward_varargs() -> Builtin {
  Builtin::ConstructFunctionForwardVarargs
}

pub const fn construct_varargs() -> Builtin {
  Builtin::ConstructVarargs
}

pub const fn construct_with_array_like() -> Builtin {
  Builtin::ConstructWithArrayLike
}

pub const fn construct_with_spread() -> Builtin {
  Builtin::ConstructWithSpread
}

pub const fn construct_with_spread_baseline() -> Builtin {
  Builtin::ConstructWithSpreadBaseline
}

pub const fn construct_with_spread_with_feedback() -> Builtin {
  Builtin::ConstructWithSpreadWithFeedback
}

pub const fn construct_baseline() -> Builtin {
  Builtin::ConstructBaseline
}

pub const fn construct_with_feedback() -> Builtin {
  Builtin::ConstructWithFeedback
}

pub const fn create_generator_object() -> Builtin {
  Builtin::CreateGeneratorObject
}

pub const fn create_iter_result_object() -> Builtin {
  Builtin::CreateIterResultObject
}

pub const fn error_capture_stack_trace() -> Builtin {
  Builtin::ErrorCaptureStackTrace
}

pub const fn error_constructor() -> Builtin {
  Builtin::ErrorConstructor
}

pub const fn error_is_error() -> Builtin {
  Builtin::ErrorIsError
}

pub const fn error_prototype_to_string() -> Builtin {
  Builtin::ErrorPrototypeToString
}

pub const fn fast_new_closure() -> Builtin {
  Builtin::FastNewClosure
}

pub const fn fast_new_object() -> Builtin {
  Builtin::FastNewObject
}

pub const fn function_constructor() -> Builtin {
  Builtin::FunctionConstructor
}

pub const fn function_prototype_bind() -> Builtin {
  Builtin::FunctionPrototypeBind
}

pub const fn function_prototype_legacy_arguments_getter() -> Builtin {
  Builtin::FunctionPrototypeLegacyArgumentsGetter
}

pub const fn function_prototype_legacy_arguments_setter() -> Builtin {
  Builtin::FunctionPrototypeLegacyArgumentsSetter
}

pub const fn function_prototype_legacy_caller_getter() -> Builtin {
  Builtin::FunctionPrototypeLegacyCallerGetter
}

pub const fn function_prototype_legacy_caller_setter() -> Builtin {
  Builtin::FunctionPrototypeLegacyCallerSetter
}

pub const fn function_prototype_to_string() -> Builtin {
  Builtin::FunctionPrototypeToString
}

pub const fn generator_function_constructor() -> Builtin {
  Builtin::GeneratorFunctionConstructor
}

pub const fn has_property() -> Builtin {
  Builtin::HasProperty
}

pub const fn instance_of() -> Builtin {
  Builtin::InstanceOf
}

pub const fn instance_of_baseline() -> Builtin {
  Builtin::InstanceOfBaseline
}

pub const fn instance_of_with_feedback() -> Builtin {
  Builtin::InstanceOfWithFeedback
}

pub const fn interpreter_forward_all_args_then_construct() -> Builtin {
  Builtin::InterpreterForwardAllArgsThenConstruct
}

pub const fn math_ceil_continuation() -> Builtin {
  Builtin::MathCeilContinuation
}

pub const fn math_clz32_continuation() -> Builtin {
  Builtin::MathClz32Continuation
}

pub const fn math_floor_continuation() -> Builtin {
  Builtin::MathFloorContinuation
}

pub const fn math_round_continuation() -> Builtin {
  Builtin::MathRoundContinuation
}

pub const fn object_assign() -> Builtin {
  Builtin::ObjectAssign
}

pub const fn object_create() -> Builtin {
  Builtin::ObjectCreate
}

pub const fn object_define_getter() -> Builtin {
  Builtin::ObjectDefineGetter
}

pub const fn object_define_properties() -> Builtin {
  Builtin::ObjectDefineProperties
}

pub const fn object_define_property() -> Builtin {
  Builtin::ObjectDefineProperty
}

pub const fn object_define_setter() -> Builtin {
  Builtin::ObjectDefineSetter
}

pub const fn object_entries() -> Builtin {
  Builtin::ObjectEntries
}

pub const fn object_freeze() -> Builtin {
  Builtin::ObjectFreeze
}

pub const fn object_get_own_property_descriptor() -> Builtin {
  Builtin::ObjectGetOwnPropertyDescriptor
}

pub const fn object_get_own_property_descriptors() -> Builtin {
  Builtin::ObjectGetOwnPropertyDescriptors
}

pub const fn object_get_own_property_names() -> Builtin {
  Builtin::ObjectGetOwnPropertyNames
}

pub const fn object_get_own_property_symbols() -> Builtin {
  Builtin::ObjectGetOwnPropertySymbols
}

pub const fn object_has_own() -> Builtin {
  Builtin::ObjectHasOwn
}

pub const fn object_is() -> Builtin {
  Builtin::ObjectIs
}

pub const fn object_is_frozen() -> Builtin {
  Builtin::ObjectIsFrozen
}

pub const fn object_is_sealed() -> Builtin {
  Builtin::ObjectIsSealed
}

pub const fn object_keys() -> Builtin {
  Builtin::ObjectKeys
}

pub const fn object_lookup_getter() -> Builtin {
  Builtin::ObjectLookupGetter
}

pub const fn object_lookup_setter() -> Builtin {
  Builtin::ObjectLookupSetter
}

pub const fn object_prototype_get_proto() -> Builtin {
  Builtin::ObjectPrototypeGetProto
}

pub const fn object_prototype_has_own_property() -> Builtin {
  Builtin::ObjectPrototypeHasOwnProperty
}

pub const fn object_prototype_is_prototype_of() -> Builtin {
  Builtin::ObjectPrototypeIsPrototypeOf
}

pub const fn object_prototype_property_is_enumerable() -> Builtin {
  Builtin::ObjectPrototypePropertyIsEnumerable
}

pub const fn object_prototype_set_proto() -> Builtin {
  Builtin::ObjectPrototypeSetProto
}

pub const fn object_seal() -> Builtin {
  Builtin::ObjectSeal
}

pub const fn object_to_string() -> Builtin {
  Builtin::ObjectToString
}

pub const fn object_values() -> Builtin {
  Builtin::ObjectValues
}

pub const fn ordinary_get_own_property_descriptor() -> Builtin {
  Builtin::OrdinaryGetOwnPropertyDescriptor
}

pub const fn ordinary_has_instance() -> Builtin {
  Builtin::OrdinaryHasInstance
}

pub const fn plain_primitive_to_number() -> Builtin {
  Builtin::PlainPrimitiveToNumber
}

pub const fn symbol_constructor() -> Builtin {
  Builtin::SymbolConstructor
}

pub const fn symbol_for() -> Builtin {
  Builtin::SymbolFor
}

pub const fn symbol_key_for() -> Builtin {
  Builtin::SymbolKeyFor
}

pub const fn to_big_int() -> Builtin {
  Builtin::ToBigInt
}

pub const fn to_big_int_convert_number() -> Builtin {
  Builtin::ToBigIntConvertNumber
}

pub const fn to_boolean_lazy_deopt_continuation() -> Builtin {
  Builtin::ToBooleanLazyDeoptContinuation
}

pub const fn to_number() -> Builtin {
  Builtin::ToNumber
}

pub const fn to_number_convert_big_int() -> Builtin {
  Builtin::ToNumberConvertBigInt
}

pub const fn to_number_baseline() -> Builtin {
  Builtin::ToNumberBaseline
}

pub const fn to_numeric_baseline() -> Builtin {
  Builtin::ToNumericBaseline
}

pub const fn r#typeof() -> Builtin {
  Builtin::Typeof
}

pub const fn typeof_baseline() -> Builtin {
  Builtin::TypeofBaseline
}

/// 来自 object_func 相关 .tq 的声明元数据 (共 30 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("extern macro", "OrdinaryHasInstance"),
  ("javascript builtin", "FunctionPrototypeHasInstance"),
  ("macro", "CheckAccessor"),
  ("extern macro", "ConstructorBuiltinsAssembler"),
  ("extern macro", "ConstructorBuiltinsAssembler"),
  ("extern macro", "ConstructorBuiltinsAssembler"),
  ("extern macro", "ConstructorBuiltinsAssembler"),
  ("extern macro", "ConstructorBuiltinsAssembler"),
  ("extern macro", "ConstructorBuiltinsAssembler"),
  ("extern macro", "LoadContextFromBaseline"),
  ("builtin", "FastNewClosureBaseline"),
  ("builtin", "FastNewFunctionContextEval"),
  ("builtin", "FastNewFunctionContextFunction"),
  ("builtin", "FastNewFunctionContextFunctionWithCells"),
  ("builtin", "CreateRegExpLiteral"),
  ("builtin", "CreateShallowArrayLiteral"),
  ("builtin", "CreateEmptyArrayLiteral"),
  ("builtin", "CreateShallowObjectLiteral"),
  ("builtin", "CreateEmptyLiteralObject"),
  ("javascript builtin", "GenericLazyDeoptContinuation"),
  ("extern macro", "OrdinaryToPrimitive"),
  ("builtin", "StringToNumber"),
  ("builtin", "NumberToString"),
  ("builtin", "SmiToString"),
  ("builtin", "Int32ToString"),
  ("builtin", "Float64ToString"),
  ("builtin", "ToBoolean"),
  ("struct", "ToBooleanForBaselineJumpResult"),
  ("builtin", "ToBooleanForBaselineJump"),
  ("javascript builtin", "FastConsoleAssert"),
];
