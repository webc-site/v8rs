// 由 tool/src/api/api.h.js 从 v8/src/api/api.{h,cc} 生成; 修订就地改, 复跑覆盖
// API 内部层: 三张 X-macro 句柄转换表显式展开 + 静态根表 + 签名骨架

//! API 内部层 — 原版 v8::Utils / api_internal (api.h) + api.cc 常量面
//!
//! 语义: 内部 Tagged 对象 ↔ v8 API 句柄的双向转换矩阵; 静态根只读空间
//! 导出值的编译期校验。

// ══ ① TO_LOCAL_LIST 显式展开 (原版 44 条; wasm 条目按特性门控) ══
/// (函数名, 内部类型, API 类型, is_wasm)
pub const TO_LOCAL_LIST: [(&str, &str, &str, bool); 44] = [
  ("ToLocal", "AccessorPair", "debug::AccessorPair", false),
  ("ToLocal", "NativeContext", "Context", false),
  ("ToLocal", "Object", "Value", false),
  ("ToLocal", "Module", "Module", false),
  ("ToLocal", "Name", "Name", false),
  ("ToLocal", "String", "String", false),
  ("ToLocal", "InternalizedString", "String", false),
  ("ToLocal", "Symbol", "Symbol", false),
  ("ToLocal", "JSDate", "Object", false),
  ("ToLocal", "JSRegExp", "RegExp", false),
  ("ToLocal", "JSReceiver", "Object", false),
  ("ToLocal", "JSObject", "Object", false),
  ("ToLocal", "JSFunction", "Function", false),
  ("ToLocal", "JSArray", "Array", false),
  ("ToLocal", "JSMap", "Map", false),
  ("ToLocal", "JSSet", "Set", false),
  ("ToLocal", "JSProxy", "Proxy", false),
  ("ToLocal", "JSArrayBuffer", "ArrayBuffer", false),
  ("ToLocal", "JSArrayBufferView", "ArrayBufferView", false),
  ("ToLocal", "JSDataView", "DataView", false),
  ("ToLocal", "JSPromise", "Promise", false),
  ("ToLocal", "JSRabGsabDataView", "DataView", false),
  ("ToLocal", "JSTypedArray", "TypedArray", false),
  ("ToLocalShared", "JSArrayBuffer", "SharedArrayBuffer", false),
  ("ToLocal", "FunctionTemplateInfo", "FunctionTemplate", false),
  ("ToLocal", "ObjectTemplateInfo", "ObjectTemplate", false),
  (
    "ToLocal",
    "DictionaryTemplateInfo",
    "DictionaryTemplate",
    false,
  ),
  (
    "SignatureToLocal",
    "FunctionTemplateInfo",
    "Signature",
    false,
  ),
  ("MessageToLocal", "Object", "Message", false),
  ("StackTraceToLocal", "StackTraceInfo", "StackTrace", false),
  ("StackFrameToLocal", "StackFrameInfo", "StackFrame", false),
  ("NumberToLocal", "Object", "Number", false),
  ("IntegerToLocal", "Object", "Integer", false),
  ("Uint32ToLocal", "Object", "Uint32", false),
  ("ToLocal", "BigInt", "BigInt", false),
  ("ExternalToLocal", "JSObject", "External", false),
  ("CallableToLocal", "JSReceiver", "Function", false),
  ("ToLocalPrimitive", "Object", "Primitive", false),
  ("FixedArrayToLocal", "FixedArray", "FixedArray", false),
  (
    "PrimitiveArrayToLocal",
    "FixedArray",
    "PrimitiveArray",
    false,
  ),
  ("ToLocal", "ScriptOrModule", "ScriptOrModule", false),
  (
    "CppHeapExternalToLocal",
    "CppHeapExternalObject",
    "CppHeapExternal",
    false,
  ),
  ("ToLocal", "WasmMemoryObject", "WasmMemoryObject", true),
  ("ToLocal", "WasmModuleObject", "WasmModuleObject", true),
];

/// 根据 API 类型查询对应的句柄转换规则
pub fn find_to_local_rule(api_type: &str) -> Option<(&'static str, &'static str, bool)> {
  TO_LOCAL_LIST
    .iter()
    .find(|(_, _, api, _)| *api == api_type)
    .map(|(fn_name, internal, _, is_wasm)| (*fn_name, *internal, *is_wasm))
}

// ══ ② TO_LOCAL_NAME_LIST 展开 (原版 15 条 — Utils 转换函数名清单) ══
pub const TO_LOCAL_NAMES: [&str; 15] = [
  "ToLocal",
  "ToLocalShared",
  "SignatureToLocal",
  "MessageToLocal",
  "StackTraceToLocal",
  "StackFrameToLocal",
  "NumberToLocal",
  "IntegerToLocal",
  "Uint32ToLocal",
  "ExternalToLocal",
  "CallableToLocal",
  "ToLocalPrimitive",
  "FixedArrayToLocal",
  "PrimitiveArrayToLocal",
  "CppHeapExternalToLocal",
];

// ══ ③ OPEN_HANDLE_LIST 展开 (原版 55 条 — OpenHandle/OpenDirectHandle/
//    OpenIndirectHandle 三联声明的 (API From, 内部 To) 矩阵; IF_WASM ×1) ══
pub const OPEN_HANDLE_LIST: [(&str, &str, bool); 56] = [
  (
    "TemplateInfoWithProperties",
    "TemplateInfoWithProperties",
    false,
  ),
  ("FunctionTemplateInfo", "FunctionTemplateInfo", false),
  ("ObjectTemplateInfo", "ObjectTemplateInfo", false),
  ("DictionaryTemplateInfo", "DictionaryTemplateInfo", false),
  ("FunctionTemplateInfo", "FunctionTemplateInfo", false),
  ("Object", "Object", false),
  ("Number", "Number", false),
  ("JSRegExp", "JSRegExp", false),
  ("JSReceiver", "JSReceiver", false),
  ("JSArray", "JSArray", false),
  ("JSMap", "JSMap", false),
  ("JSSet", "JSSet", false),
  ("JSArrayBuffer", "JSArrayBuffer", false),
  ("JSArrayBufferView", "JSArrayBufferView", false),
  ("JSTypedArray", "JSTypedArray", false),
  ("JSTypedArray", "JSTypedArray", false),
  ("JSTypedArray", "JSTypedArray", false),
  ("JSTypedArray", "JSTypedArray", false),
  ("JSTypedArray", "JSTypedArray", false),
  ("JSTypedArray", "JSTypedArray", false),
  ("JSTypedArray", "JSTypedArray", false),
  ("JSTypedArray", "JSTypedArray", false),
  ("JSTypedArray", "JSTypedArray", false),
  ("JSTypedArray", "JSTypedArray", false),
  ("JSTypedArray", "JSTypedArray", false),
  (
    "JSDataViewOrRabGsabDataView",
    "JSDataViewOrRabGsabDataView",
    false,
  ),
  ("JSArrayBuffer", "JSArrayBuffer", false),
  ("Name", "Name", false),
  ("String", "String", false),
  ("Symbol", "Symbol", false),
  ("JSFunction", "JSFunction", false),
  ("SharedFunctionInfo", "SharedFunctionInfo", false),
  ("SharedFunctionInfo", "SharedFunctionInfo", false),
  ("Module", "Module", false),
  ("JSReceiver", "JSReceiver", false),
  ("Script", "Script", false),
  ("JSMessageObject", "JSMessageObject", false),
  ("NativeContext", "NativeContext", false),
  ("Object", "Object", false),
  ("StackTraceInfo", "StackTraceInfo", false),
  ("StackFrameInfo", "StackFrameInfo", false),
  ("JSProxy", "JSProxy", false),
  ("JSGeneratorObject", "JSGeneratorObject", false),
  ("HeapObject", "HeapObject", false),
  ("Script", "Script", false),
  ("EphemeronHashTable", "EphemeronHashTable", false),
  ("AccessorPair", "AccessorPair", false),
  ("JSPromise", "JSPromise", false),
  ("Object", "Object", false),
  ("FixedArray", "FixedArray", false),
  ("BigInt", "BigInt", false),
  ("ScriptOrModule", "ScriptOrModule", false),
  ("FixedArray", "FixedArray", false),
  ("ModuleRequest", "ModuleRequest", false),
  ("CppHeapExternalObject", "CppHeapExternalObject", false),
  ("WasmMemoryObject", "WasmMemoryObject", true),
];

// ══ ④ 静态根导出表 (api.cc; V8_STATIC_ROOTS_BOOL 门控 — x64/arm64 指针压缩开 → 展开) ══
/// (API 名, 原版取值表达式; 编译期校验 Internals::StaticReadOnlyRoot 一致)
pub const EXPORTED_STATIC_ROOTS: [(&str, &str); 8] = [
  ("UndefinedValue", "i::StaticReadOnlyRoot::kUndefinedValue"),
  ("NullValue", "i::StaticReadOnlyRoot::kNullValue"),
  ("TrueValue", "i::StaticReadOnlyRoot::kTrueValue"),
  ("FalseValue", "i::StaticReadOnlyRoot::kFalseValue"),
  ("EmptyString", "i::StaticReadOnlyRoot::kempty_string"),
  ("TheHoleValue", "i::StaticReadOnlyRoot::kTheHoleValue"),
  ("StringMapLowerBound", "kStringMapRange.first"),
  ("StringMapUpperBound", "kStringMapRange.second"),
];

/// 根据根名称查询静态根导出表达式
pub fn find_exported_static_root(name: &str) -> Option<&'static str> {
  EXPORTED_STATIC_ROOTS
    .iter()
    .find(|(n, _)| *n == name)
    .map(|(_, expr)| *expr)
}

/// 编译期校验的静态根数 (原版 kNumberOfCheckedStaticRoots = 0 + PLUS_ONE × N)
pub const NUMBER_OF_CHECKED_STATIC_ROOTS: i32 = 8;

// ══ ⑤ api.cc 匿名枚举 ══
/// FastIterateNonNullReceiverMode 侧返回态 (原版 enum class FastIterateResult)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastIterateResult {
  /// 原版 kException
  Exception,
  /// 原版 kBreak
  Break,
  /// 原版 kSlowPath
  SlowPath,
  /// 原版 kFinished
  Finished,
}

/// Map→Array 视图种类 (原版 enum class MapAsArrayKind; 取值 = 迭代器实例类型)
pub enum MapAsArrayKind {
  /// 原版 kEntries
  Entries,
  /// 原版 kKeys
  Keys,
  /// 原版 kValues
  Values,
}

/// Set→Array 视图种类 (原版 enum class SetAsArrayKind)
pub enum SetAsArrayKind {
  /// 原版 kEntries
  Entries,
  /// 原版 kValues
  Values,
}

// ══ ⑥ Utils 签名骨架 ══
// api_check(condition, location, message) -> bool — 失败走 report_api_failure
// report_oom_failure(isolate, location, details)
// to_maybe(value) / to_maybe_local(obj) — Local ↔ MaybeLocal 适配
// open_persistent(persistent) -> Handle<Object> ×2 重载
// convert<From, To>(obj) -> Local<To> — V8_ENABLE_DIRECT_HANDLE 双路径
// open_handle / open_direct_handle / open_indirect_handle — OPEN_HANDLE_LIST 三联 ×55
// to_local_<Type>Array ×TYPED_ARRAYS — 类型表由 objects 侧脚本落地
// 私有: report_api_failure(location, message) — V8_NOINLINE V8_PRESERVE_MOST

// ══ ⑦ 其他骨架 ══
// ApiFunction { addr } — address()
// RegisteredExtension: register(ext) / unregister_all() / extension() / next() /
//   first_extension() — 链表节点 (first_extension_ 静态头)
// api_internal (builtin 侧包装, profiling/side-effect 检查):
//   InvokeAccessorGetterCallback()
//   InvokeNamedInterceptorGetterCallback()
//   InvokeNamedInterceptorSetterCallback()
//   InvokeFunctionCallbackGeneric()
//   InvokeFunctionCallbackOptimized()
//   InvokeFinalizationRegistryCleanupFromTask()
//   ConvertDouble<T>(double) -> T — EXPORT_TEMPLATE 双实例化
//   ValidateCallbackInfo<T>(info) -> bool ×2 (Function/Property)

// 语义承接点台账:
//   kStringMapRange = UniqueMapRangeOfInstanceTypeRange(FIRST_STRING_TYPE, LAST_STRING_TYPE)
//     — 依赖静态根布局, 由 roots 模块数值代入
//   TO_LOCAL_LIST 展开体 = Utils::Convert<From, To> 直通 (api-inl 侧落地)
//   api.cc 12752 行 — 本脚本只取常量/枚举/表; 函数体由 Developer 按调用面落地

// 平台分支台账:
//   V8_STATIC_ROOTS_BOOL: 指针压缩 + 固定布局 (x64/arm64) → 静态根表展开
//   IF_WASM ×3 (TO_LOCAL ×2 + OPEN_HANDLE ×1) — V8_ENABLE_WEBASSEMBLY 门控
