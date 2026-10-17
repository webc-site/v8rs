// 由 tool/src/api/api-inl.h.js 从 v8/src/api/api-inl.h 生成; 修订就地改, 复跑覆盖
// API 内联层: 转换宏台账 + ToCData/FromCData 骨架 + 作用域族清单

//! API 内联层 — 原版 v8/src/api/api-inl.h
//!
//! 语义: api.h 声明的内联落地 — 句柄转换宏实例化、C 数据指针往返、
//! V8 进入/退出作用域族 (call depth / script 允许位 / 微任务回调钩子)。

// ══ ① 转换宏台账 (3 宏, 与 api.h 三表一一对应; 展开体 = Utils::Convert 直通) ══
// MAKE_TO_LOCAL(Name, From, To)      — Utils::Name(DirectHandle<From>) → Convert → Local<To>
// MAKE_TO_LOCAL_TYPED_ARRAY(T, t, T, c) — Local<Type##Array> + DCHECK 元素类型
// MAKE_OPEN_HANDLE(From, To) ×2:
//   V8_ENABLE_DIRECT_HANDLE: OpenHandle(CreateHandleForCurrentTraced) +
//     OpenDirectHandle(FromAddress) + OpenIndirectHandle(转发 OpenHandle)
//   !V8_ENABLE_DIRECT_HANDLE: OpenHandle(FromSlot 间接句柄) 单路径
// 消费: TO_LOCAL_LIST ×44 / TYPED_ARRAYS / OPEN_HANDLE_LIST ×56 (见 api.rs 表)

// ══ ② ToCData / FromCData 签名骨架 (2+2, 1:1) ══
// to_c_data<tag, T>(isolate, obj: Tagged<Object>) -> *mut T — sizeof(T)==sizeof(Address)
// to_c_data<tag>(isolate, obj) -> Address
// from_c_data<tag, T>(isolate, obj: *mut T) -> DirectHandle<Smi|Foreign>
// from_c_data<tag>(isolate, obj: Address) -> DirectHandle<Smi|Foreign>
// 空指针/空地址 → Smi::zero (2 处 1:1)

// ══ ③ Utils::Convert 双路径 (1:1) ══
// V8_ENABLE_DIRECT_HANDLE: Local<To>::FromAddress(obj.address()) — 空判先行
// 否则: Local<To>::FromSlot(indirect_handle(obj).location()) — 间接句柄伪装直句柄
// 前置 DCHECK: 非 TheHole (IsSmi / null 放行)

// ══ ④ 作用域族 (10 类, 定义序 1:1) ══
// CallDepthScope — <do_callback> 进出深度计数; 归零时清未处理异常 + FireCallCompleted; DEBUG 校验微任务检查点; DISALLOW_NEW_AND_DELETE
// InternalEscapableScope — EscapableHandleScopeBase 内层包装
// EnterV8InternalScope — <HandleScopeClass, do_callback> 公共主体 (RCS/禁分配守卫承接点)
// EnterV8Scope — = EnterV8InternalScope<HandleScopeClass, true>
// EnterV8NoScriptScope — = EnterV8InternalScope<HandleScopeClass, false> (禁脚本)
// EnterV8NoScriptNoExceptionScope — 禁脚本 + 禁异常
// EnterV8BasicScope — 轻量进入
// PrepareForExecutionScope — 执行前置 (旗标/中止检查承接点)
// PrepareForDebugInterfaceExecutionScope — 调试接口前置
// ApiRuntimeCallStatsScope — RCS 计时; 关闭 RCS 时为空体 constexpr

// ══ ⑤ 拦截器/回调包装 (api_internal: api.h 声明 / api.cc 定义, 台账 1:1) ══
// InvokeAccessorGetterCallback(property, info) — CallApiGetter 内置侧包装
// InvokeNamedInterceptorGetterCallback(property, info) -> Intercepted
// InvokeNamedInterceptorSetterCallback(property, value, info) -> Intercepted
// InvokeFunctionCallbackGeneric(info) / InvokeFunctionCallbackOptimized(info)
//   — api_callback_thunk_argument 槽取回调地址
// InvokeFinalizationRegistryCleanupFromTask(native_context, registry)
// validate_callback_info<T> ×2 (Function/Property) — EXPORT_TEMPLATE 双实例化
// StackAllocatedCheck (ENABLE_SLOW_DCHECKS 门控, 默认 true)

// 平台分支台账:
//   V8_ENABLE_DIRECT_HANDLE ×2 (MAKE_OPEN_HANDLE / Convert) — 按句柄模式二选一
//   ENABLE_SLOW_DCHECKS ×1 — cfg 门控登记
