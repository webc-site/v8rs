// 由 tool/src/api/api-arguments.h.js 从 v8/src/api/api-arguments.{h,-inl.h,cc} 生成; 修订就地改, 复跑覆盖
// API 回调参数栈: 槽位索引展开 + Call* 方法骨架

//! API 回调参数栈 — 原版 v8::internal::{PropertyCallbackArguments, FunctionCallbackArguments}
//!
//! 语义: ApiCallbackExit/ApiAccessorExit 帧布局上的参数槽视图 (Relocatable 根,
//! IterateInstance 全量访问); 帧槽位与 include/v8-function-callback.h 两枚举 1:1 对齐。

// —— 槽位索引宏展开 (原版 FunctionCallbackInfo 枚举; kFrameCPSlotCount=0, x64/arm64) ——
/// 帧块: 新目标槽, 仅 API_CONSTRUCT_EXIT 存在 (原版 kNewTargetIndex = -1)
pub const FCA_NEW_TARGET_INDEX: i32 = -1;
/// 原版 kArgcIndex (枚举自 0 起)
pub const FCA_ARGC_INDEX: i32 = 0;
/// 原版 kFrameSPIndex
pub const FCA_FRAME_SP_INDEX: i32 = 1;
/// 原版 kFrameTypeIndex
pub const FCA_FRAME_TYPE_INDEX: i32 = 2;
/// 原版 kFrameConstantPoolIndex (+kFrameCPSlotCount=0 → kFrameFPIndex=4, kFramePCIndex=5)
pub const FCA_FRAME_CP_INDEX: i32 = 3;
/// 原版 kFirstApiArgumentIndex = kIsolateIndex
pub const FCA_FIRST_API_ARG_INDEX: i32 = 6;
/// 原版 kIsolateIndex
pub const FCA_ISOLATE_INDEX: i32 = 6;
/// 原版 kReturnValueIndex
pub const FCA_RETURN_VALUE_INDEX: i32 = 7;
/// 原版 kContextIndex
pub const FCA_CONTEXT_INDEX: i32 = 8;
/// 原版 kTargetIndex
pub const FCA_TARGET_INDEX: i32 = 9;
/// 原版 kReceiverIndex (JS 参数块起点)
pub const FCA_RECEIVER_INDEX: i32 = 10;
/// 原版 kFirstJSArgumentIndex
pub const FCA_FIRST_JS_ARG_INDEX: i32 = 11;
/// 原版 kArgsLength = kReceiverIndex + 1
pub const FCA_ARGS_LENGTH: i32 = 11;
/// 原版 kOptionalArgsLength (仅构造帧新目标槽)
pub const FCA_OPTIONAL_ARGS_LENGTH: i32 = 1;
/// 原版 kApiArgsLength = kReceiverIndex - kFirstApiArgumentIndex
pub const FCA_API_ARGS_LENGTH: i32 = 4;

// —— 槽位索引宏展开 (原版 PropertyCallbackInfo 枚举) ——
/// 原版 kPropertyKeyIndex (枚举自 0 起)
pub const PCA_PROPERTY_KEY_INDEX: i32 = 0;
/// 原版 kFrameSPIndex
pub const PCA_FRAME_SP_INDEX: i32 = 1;
/// 原版 kFrameTypeIndex
pub const PCA_FRAME_TYPE_INDEX: i32 = 2;
/// 原版 kFrameConstantPoolIndex (+kFrameCPSlotCount=0)
pub const PCA_FRAME_CP_INDEX: i32 = 3;
/// 原版 kFirstApiArgumentIndex = kIsolateIndex
pub const PCA_FIRST_API_ARG_INDEX: i32 = 6;
/// 原版 kIsolateIndex
pub const PCA_ISOLATE_INDEX: i32 = 6;
/// 原版 kReturnValueIndex
pub const PCA_RETURN_VALUE_INDEX: i32 = 7;
/// 原版 kCallbackInfoIndex
pub const PCA_CALLBACK_INFO_INDEX: i32 = 8;
/// 原版 kHolderIndex
pub const PCA_HOLDER_INDEX: i32 = 9;
/// 可选段起点 (原版 kFirstOptionalArgument = kShouldThrowOnErrorIndex)
pub const PCA_FIRST_OPTIONAL_INDEX: i32 = 10;
/// 原版 kShouldThrowOnErrorIndex
pub const PCA_SHOULD_THROW_ON_ERROR_INDEX: i32 = 10;
/// 原版 kValueIndex (CallApiSetter 内置句柄暂存)
pub const PCA_VALUE_INDEX: i32 = 11;
/// 原版 kFullArgsLength (枚举自然位)
pub const PCA_FULL_ARGS_LENGTH: i32 = 12;
/// 原版 kMandatoryArgsLength = kFirstOptionalArgument
pub const PCA_MANDATORY_ARGS_LENGTH: i32 = 10;
/// 原版 kOptionalArgsLength = kFullArgsLength - kFirstOptionalArgument
pub const PCA_OPTIONAL_ARGS_LENGTH: i32 = 2;
/// 原版 kMandatoryApiArgsLength = kMandatoryArgsLength - kFirstApiArgumentIndex
pub const PCA_MANDATORY_API_ARGS_LENGTH: i32 = 4;
/// 原版 kFullApiArgsLength = kFullArgsLength - kFirstApiArgumentIndex
pub const PCA_FULL_API_ARGS_LENGTH: i32 = 6;

// PCA 转发常量语义 (原版 h 内 T:: 别名, 14 条 1:1):
// kGetterApiArgsLength = kMandatoryApiArgsLength = 4
// kSetterApiArgsLength = kFullApiArgsLength = 6

// —— PropertyCallbackArguments 方法骨架 (h, 18 Call* + 辅助 1:1) ——
// CallAccessorGetter(isolate, accessor_info/interceptor, name) -> DirectHandle
// CallAccessorSetter(isolate, interceptor, name/index[, value/desc]) -> Intercepted
// CallNamedQuery(isolate, interceptor, name/index) -> Handle
// CallNamedGetter(isolate, accessor_info/interceptor, name) -> DirectHandle
// CallNamedSetter(isolate, interceptor, name/index[, value/desc]) -> Intercepted
// CallNamedDefiner(isolate, interceptor, name/index[, value/desc]) -> Intercepted
// CallNamedDeleter(isolate, interceptor, name/index[, value/desc]) -> Intercepted
// CallNamedDescriptor(isolate, interceptor, name/index) -> Handle
// CallNamedEnumerator(isolate, interceptor) -> DirectHandle<JSObjectOrUndefined>
// CallIndexedQuery(isolate, interceptor, name/index) -> Handle
// CallIndexedGetter(isolate, accessor_info/interceptor, name) -> DirectHandle
// CallIndexedSetter(isolate, interceptor, name/index[, value/desc]) -> Intercepted
// CallIndexedDefiner(isolate, interceptor, name/index[, value/desc]) -> Intercepted
// CallIndexedDeleter(isolate, interceptor, name/index[, value/desc]) -> Intercepted
// CallIndexedDescriptor(isolate, interceptor, name/index) -> Handle
// CallIndexedEnumerator(isolate, interceptor) -> DirectHandle<JSObjectOrUndefined>
// CallIndexedIndexOf(isolate, interceptor, value, start_index, end_index, in_out_length*) -> u32
// CallIndexedIterableToList(isolate, interceptor) -> DirectHandle
// 辅助: AcceptSideEffects (DEBUG 下清 javascript_execution_counter) /
//   GetBooleanReturnValue / ShouldThrowOnError / is_named /
//   set_property_key(name|index) / holder() / Initialize / CallPropertyEnumerator /
//   GetPropertyCallbackInfo / GetAccessorInfo / GetInterceptorInfo / IsNamed /
//   GetPropertyName / GetPropertyIndex / GetReturnValue<V> / slot_at
// 构造: (isolate, holder) 与 (isolate, holder, should_throw) — 后三槽留给 CallXXX 填充

// —— FunctionCallbackArguments 方法骨架 (h, 1:1) ——
// ctor ×2 ([[Call]] 无 new_target / [[Construct]] 带 new_target; ArgT ∈ {DirectHandle<Object>, Address})
// call_or_construct(isolate, function, is_construct) -> Tagged<JSAny>
// static get_target(info) — kTargetIndex 读取
// slot_at(index) — 前移 kOptionalArgsLength
// iterate_instance(v) — 访问 kNewTargetIndex 槽 + [kFirstApiArgumentIndex, end)
// 存储: base::SmallVector<Address, 16> values_

// —— api-arguments-inl.h 宏台账 ——
// DCHECK_NAME_COMPATIBLE(interceptor, name) — 命名/索引一致性自检
// PREPARE_CALLBACK_INFO_ACCESSOR(ISOLATE, F, API_RETURN_TYPE, …) — 回调信息装配
// PREPARE_CALLBACK_INFO_INTERCEPTOR(ISOLATE, F, API_RETURN_TYPE, …) — 同上, 拦截器版

// —— cc 落地定义 (38 行, 1:1) ——
// FunctionCallbackArguments::IterateInstance / PropertyCallbackArguments::IterateInstance
// static_assert ×2: BuiltinArguments::kNumExtraArgs(WithReceiver) ==
//   BuiltinExitFrameConstants::kNumExtraArgs(WithReceiver)

// 平台分支台账:
//   I::kFrameCPSlotCount: PPC64=1, x64/arm64=0 — 已按 0 展开; PPC64 登记跳过
//   DEBUG 槽 zap / javascript_execution_counter — cfg(debug_assertions) 承接
