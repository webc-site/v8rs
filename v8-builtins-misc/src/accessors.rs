// 由 tool/src/builtins/accessors.js 从 v8/src/builtins/accessors.{h,cc} 生成; 修订就地改, 复跑覆盖
// X-macro accessor 表机械展开 + 方法登记, 函数体不展开
//! 同名族合并脚本 (accessors.h + accessors.cc)
//! 任务映射: v8-builtins-misc; 聚合基线另归 v8-builtins-proxy-reflect (并存不覆盖)

/// 预定义 accessor info 名单 — X-macro ACCESSOR_INFO_LIST_GENERATOR 展开 (V(_, name, Pascal, 效应标志…))
pub const ACCESSOR_INFO_LIST: &[(&str, &str)] = &[
  ("arguments_iterator", "ArgumentsIterator"),
  ("array_length", "ArrayLength"),
  ("bound_function_length", "BoundFunctionLength"),
  ("bound_function_name", "BoundFunctionName"),
  ("function_name", "FunctionName"),
  ("function_length", "FunctionLength"),
  ("function_prototype", "FunctionPrototype"),
  ("module_namespace_property", "ModuleNamespaceEntry"),
  ("string_length", "StringLength"),
  ("value_unavailable", "ValueUnavailable"),
  ("wrapped_function_length", "WrappedFunctionLength"),
  ("wrapped_function_name", "WrappedFunctionName"),
  ("lazy_closure", "InstantiateLazyClosure"),
];

/// 独立 getter 名单 — X-macro ACCESSOR_GETTER_LIST 展开 (当前为空)
pub const ACCESSOR_GETTER_LIST: &[&str] = &[];

/// 独立 setter 名单 — X-macro ACCESSOR_SETTER_LIST 展开
pub const ACCESSOR_SETTER_LIST: &[&str] = &[
  "ArrayLengthSetter",
  "FunctionPrototypeSetter",
  "ModuleNamespaceEntrySetter",
  "ReconfigureToDataProperty",
];

/// callback 名单 — X-macro ACCESSOR_CALLBACK_LIST_GENERATOR 展开
pub const ACCESSOR_CALLBACK_LIST: &[&str] = &["ErrorStackGetter", "ErrorStackSetter"];

/// 原版 static constexpr 计数 — X-macro COUNT_ACCESSOR(__VA_ARGS__) +1 机械展开
pub const K_ACCESSOR_INFO_COUNT: i32 = 13;
pub const K_ACCESSOR_GETTER_COUNT: i32 = 0;
pub const K_ACCESSOR_SETTER_COUNT: i32 = 4;
pub const K_ACCESSOR_CALLBACK_COUNT: i32 = 2;

/// accessors.cc 侧 Accessors::<方法>( 定义点登记 (名, 原版行号; 同名多次定义按出现次数记)
pub const CC_METHODS: &[(&str, u32)] = &[
  ("MakeAccessor", 31),
  ("IsJSObjectFieldAccessor", 67),
  ("ReplaceAccessorWithDataProperty", 92),
  ("ReconfigureToDataProperty", 117),
  ("ReplaceAccessorWithDataProperty", 128),
  ("ArgumentsIteratorGetter", 137),
  ("MakeArgumentsIteratorInfo", 147),
  ("ArrayLengthGetter", 157),
  ("ArrayLengthSetter", 170),
  ("MakeArrayLengthInfo", 234),
  ("ModuleNamespaceEntryGetter", 243),
  ("ModuleNamespaceEntrySetter", 256),
  ("MakeModuleNamespaceEntryInfo", 275),
  ("StringLengthGetter", 285),
  ("MakeStringLengthInfo", 299),
  ("FunctionPrototypeGetter", 308),
  ("FunctionPrototypeSetter", 321),
  ("MakeFunctionPrototypeInfo", 335),
  ("FunctionLengthGetter", 345),
  ("MakeFunctionLengthInfo", 355),
  ("FunctionNameGetter", 364),
  ("MakeFunctionNameInfo", 373),
  ("FunctionGetArguments", 495),
  ("GetLegacyFunctionArguments", 510),
  ("GetLegacyFunctionCaller", 677),
  ("BoundFunctionLengthGetter", 696),
  ("MakeBoundFunctionLengthInfo", 710),
  ("BoundFunctionNameGetter", 720),
  ("MakeBoundFunctionNameInfo", 734),
  ("WrappedFunctionLengthGetter", 744),
  ("MakeWrappedFunctionLengthInfo", 758),
  ("InstantiateLazyClosureGetter", 768),
  ("MakeInstantiateLazyClosureInfo", 811),
  ("ValueUnavailableGetter", 826),
  ("MakeValueUnavailableInfo", 835),
  ("WrappedFunctionNameGetter", 845),
  ("MakeWrappedFunctionNameInfo", 859),
  ("ErrorStackGetter", 869),
  ("ErrorStackSetter", 888),
];

// —— 对数断言 ——
const _: () = {
  assert!(ACCESSOR_INFO_LIST.len() == 13);
  assert!(ACCESSOR_GETTER_LIST.len() == 0);
  assert!(ACCESSOR_SETTER_LIST.len() == 4);
  assert!(ACCESSOR_CALLBACK_LIST.len() == 2);
  assert!(K_ACCESSOR_INFO_COUNT as usize == ACCESSOR_INFO_LIST.len());
  assert!(K_ACCESSOR_SETTER_COUNT as usize == ACCESSOR_SETTER_LIST.len());
  assert!(K_ACCESSOR_CALLBACK_COUNT as usize == ACCESSOR_CALLBACK_LIST.len());
  assert!(CC_METHODS.len() == 39);
};
