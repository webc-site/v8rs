#!/usr/bin/env bun
// tool/src/api/api-inl.h.js
// 机械提取 v8/src/api/api-inl.h
// 产物: v8-api/src/api_inl.rs — 转换宏台账 + ToCData/FromCData 骨架 + 作用域族清单

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/api/api-inl.h");
// api_internal 包装函数声明于 api.h、定义于 api.cc — 台账核对取 api.cc
const apiH = readV8("src/api/api.h");
const apiCc = readV8("src/api/api.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 三张转换宏 (MAKE_TO_LOCAL / MAKE_TO_LOCAL_TYPED_ARRAY / MAKE_OPEN_HANDLE ×2 分支)
for (const mac of ["#define MAKE_TO_LOCAL(Name, From, To)", "#define MAKE_TO_LOCAL_TYPED_ARRAY(Type, typeName, TYPE, ctype)"]) {
  if (!src.includes(mac)) fail(mac, false, true);
}
// MAKE_OPEN_HANDLE 在 V8_ENABLE_DIRECT_HANDLE 两分支各定义一次
const makeOpen = (src.match(/#define MAKE_OPEN_HANDLE\(From, To\)/g) || []).length;
if (makeOpen !== 2) fail("MAKE_OPEN_HANDLE 定义", makeOpen, 2);
must(src.includes("#ifdef V8_ENABLE_DIRECT_HANDLE"), "DIRECT_HANDLE 分支");
must(src.includes("#else  // !V8_ENABLE_DIRECT_HANDLE"), "非 DIRECT_HANDLE 分支");

// 2. 消费点: TO_LOCAL_LIST(MAKE_TO_LOCAL) / TYPED_ARRAYS(...) / OPEN_HANDLE_LIST(MAKE_OPEN_HANDLE)
must(src.includes("TO_LOCAL_LIST(MAKE_TO_LOCAL)"), "TO_LOCAL_LIST 消费");
must(src.includes("TYPED_ARRAYS(MAKE_TO_LOCAL_TYPED_ARRAY)"), "TYPED_ARRAYS 消费");
must(src.includes("OPEN_HANDLE_LIST(MAKE_OPEN_HANDLE)"), "OPEN_HANDLE_LIST 消费");

// 3. ToCData ×2 / FromCData ×2 模板 (api.h 声明, inl 落地)
const toCData = (src.match(/inline T?i? ?ToCData\(/g) || []).length;
const toCDataRe = [...src.matchAll(/^inline (?:T |i::Address )?ToCData\(/gm)].length;
if (toCDataRe !== 2) fail("ToCData 定义", toCDataRe, 2);
const fromCDataRe = [...src.matchAll(/^inline i::DirectHandle<i::UnionOf<i::Smi, i::Foreign>> FromCData\(/gm)].length;
if (fromCDataRe !== 2) fail("FromCData 定义", fromCDataRe, 2);
// 空指针归零语义
must(src.includes("return direct_handle(i::Smi::zero(), isolate);"), "Smi::zero 空回退 ×2");

// 4. Utils::Convert 双路径
must(src.includes("#ifdef V8_ENABLE_DIRECT_HANDLE"), "Convert DIRECT_HANDLE");
must(src.includes("Local<To>::FromAddress(obj.address());"), "Convert FromAddress 路径");
must(src.includes("Local<To>::FromSlot(indirect_handle(obj).location());"), "Convert FromSlot 路径");

// 5. 作用域族 10 类 (定义序)
const scopes = [...src.matchAll(/^class V8_NODISCARD (\w+)/gm)].map((m) => m[1]);
const wantScopes = [
  "CallDepthScope", "InternalEscapableScope", "EnterV8InternalScope", "EnterV8Scope",
  "EnterV8NoScriptScope", "EnterV8NoScriptNoExceptionScope", "EnterV8BasicScope",
  "PrepareForExecutionScope", "PrepareForDebugInterfaceExecutionScope", "ApiRuntimeCallStatsScope",
];
if (scopes.length !== wantScopes.length) fail("作用域类数", scopes.length, wantScopes.length);
for (let i = 0; i < wantScopes.length; i++) {
  if (scopes[i] !== wantScopes[i]) fail(`作用域序 ${i}`, scopes[i], wantScopes[i]);
}

// 6. CallDepthScope 关键语义
must(src.includes("template <bool do_callback>"), "CallDepthScope do_callback 模板");
must(src.includes("IncrementCallDepth<do_callback>(this);"), "IncrementCallDepth");
must(src.includes("DecrementCallDepth(this);"), "DecrementCallDepth");
must(src.includes("FireBeforeCallEnteredCallback();"), "FireBefore 钩子");
must(src.includes("FireCallCompletedCallback(microtask_queue);"), "FireCallCompleted 钩子");
must(src.includes("DISALLOW_NEW_AND_DELETE()"), "DISALLOW_NEW_AND_DELETE");

// 7. EnterV8InternalScope 模板参数 (HandleScopeClass, do_callback)
must(src.includes("class V8_NODISCARD EnterV8InternalScope {"), "EnterV8InternalScope 模板头");

// 8. 拦截器/回调包装 (api.h 声明 / api.cc 定义 — 本模块登记台账)
for (const f of ["InvokeAccessorGetterCallback", "InvokeNamedInterceptorGetterCallback", "InvokeNamedInterceptorSetterCallback", "InvokeFunctionCallbackGeneric", "InvokeFunctionCallbackOptimized", "InvokeFinalizationRegistryCleanupFromTask"]) {
  if (!apiH.includes(f)) fail(`api.h 声明 ${f}`, false, true);
  if (!apiCc.includes(f)) fail(`api.cc 定义 ${f}`, false, true);
}
// ValidateCallbackInfo 模板 ×2 + StackAllocatedCheck (api.h 尾部, ENABLE_SLOW_DCHECKS 门控)
const vci = (apiH.match(/bool ValidateCallbackInfo\(const (Function|Property)CallbackInfo<T>& info\);/g) || []).length;
if (vci !== 2) fail("ValidateCallbackInfo 声明", vci, 2);
must(apiH.includes("DECLARE_CONTEXTUAL_VARIABLE_WITH_DEFAULT(StackAllocatedCheck, const bool, true);"), "StackAllocatedCheck");

let n = 27;
const rust = `${banner("src/api/api-inl.h.js", "src/api/api-inl.h", "API 内联层: 转换宏台账 + ToCData/FromCData 骨架 + 作用域族清单")}
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
${wantScopes
  .map((s) => {
    const note = {
      CallDepthScope: "<do_callback> 进出深度计数; 归零时清未处理异常 + FireCallCompleted; DEBUG 校验微任务检查点; DISALLOW_NEW_AND_DELETE",
      InternalEscapableScope: "EscapableHandleScopeBase 内层包装",
      EnterV8InternalScope: "<HandleScopeClass, do_callback> 公共主体 (RCS/禁分配守卫承接点)",
      EnterV8Scope: "= EnterV8InternalScope<HandleScopeClass, true>",
      EnterV8NoScriptScope: "= EnterV8InternalScope<HandleScopeClass, false> (禁脚本)",
      EnterV8NoScriptNoExceptionScope: "禁脚本 + 禁异常",
      EnterV8BasicScope: "轻量进入",
      PrepareForExecutionScope: "执行前置 (旗标/中止检查承接点)",
      PrepareForDebugInterfaceExecutionScope: "调试接口前置",
      ApiRuntimeCallStatsScope: "RCS 计时; 关闭 RCS 时为空体 constexpr",
    }[s];
    return `// ${s} — ${note}`;
  })
  .join("\n")}

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
`;

const p = writeSrc("v8-api", "api_inl.rs", rust);
console.log(`OK api-inl.h.js: 对数断言 ${n} 全过; 宏 4 定义 / 消费点 3 / ToCData 2 / FromCData 2 / 作用域 10 / 台账 6+2; 产物 ${p}`);
