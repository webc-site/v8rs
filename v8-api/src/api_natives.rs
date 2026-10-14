// 由 tool/src/api/api-natives.h.js 从 v8/src/api/api-natives.{h,cc} 生成; 修订就地改, 复跑覆盖
// ApiNatives: 模板实例化面签名骨架

//! API 原生实例化 — 原版 v8::internal::ApiNatives
//!
//! 语义: FunctionTemplateInfo/ObjectTemplateInfo → JSFunction/JSObject 实例化管道
//! (含函数缓存、属性/访问器装配、远程对象)。

/// 函数模板缓存初始容量 (原版 kInitialFunctionCacheSize)
pub const INITIAL_FUNCTION_CACHE_SIZE: i32 = 256;

// —— 签名骨架 (h 10 声明 / cc ApiNatives:: 定义 8 + 匿名辅助, 1:1) ——
// CreateAccessorFunctionTemplateInfo — create_accessor_function_template_info(isolate, callback, length, side_effect_type) — 回调模板包装
// InstantiateFunction — instantiate_function(isolate, native_context, data, maybe_name) — 带上下文重载
// InstantiateFunction — instantiate_function(isolate, data, maybe_name) — 短重载
// InstantiateObject — instantiate_object(isolate, data, new_target) — MaybeHandle<JSObject>
// InstantiateRemoteObject — instantiate_remote_object(data) — 远程对象 (跨上下文)
// CreateApiFunction — create_api_function(isolate, native_context, obj, prototype, type, name) — InstanceType 定型
// AddDataProperty — add_data_property(isolate, info, name, value, attributes) — Object 值
// AddDataProperty — add_data_property(isolate, info, name, intrinsic, attributes) — Intrinsic 值
// AddAccessorProperty — add_accessor_property(isolate, info, name, getter, setter, attributes)
// AddNativeDataProperty — add_native_data_property(isolate, info, property: AccessorInfo)

// cc 匿名 namespace 台账:
//   InstantiateObject / InstantiateFunction (递归主体) / ConfigureInstance (属性装配)
//   CreateApiFunction: remove_prototype 早退 / read_only_prototype 换 map /
//   TheHole prototype → 新建原型 / prototype provider → constructor 属性挂接
