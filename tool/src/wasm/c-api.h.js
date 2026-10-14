#!/usr/bin/env bun
// tool/src/wasm/c-api.h.js
// 机械提取 v8/src/wasm/c-api.{h,cc} 的 wasm-api 实现层常量与 implement 模板面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/c-api.h");
const ccSrc = readV8("src/wasm/c-api.cc");

// 1. h: StoreImpl 契约
if (!hSrc.includes("class StoreImpl {")) throw new Error("未找到 StoreImpl");
for (const m of [
  "v8::Isolate* isolate_ = nullptr;",
  "v8::Eternal<v8::Context> context_;",
  "i::IndirectHandle<i::JSWeakMap> host_info_map_;",
]) {
  if (!hSrc.includes(m)) throw new Error(`StoreImpl 成员缺失: ${m}`);
}

// 2. h: LIBWASM_STATIC 门
if (!hSrc.includes("#define LIBWASM_STATIC 1")) throw new Error("未找到 LIBWASM_STATIC");

// 3. cc: implement<T> 模板面
const impls = [...ccSrc.matchAll(/struct implement<(\w+)> \{/g)].map((m) => m[1]);
if (impls.length < 10) throw new Error(`implement 面 ${impls.length} < 10`);

// 4. cc: Counter 名字盒尺寸
if (!ccSrc.includes("constexpr int kNameBoxSize = 64;")) throw new Error("未找到 kNameBoxSize=64");
if (!ccSrc.includes("constexpr int kValueBoxSize = 13;")) throw new Error("未找到 kValueBoxSize=13");

// 5. cc: 嵌套实例化深度限制
const stackLimit = ccSrc.match(/constexpr int kStackLimit = (\d+);/);
if (!stackLimit) throw new Error("未找到 kStackLimit");
const stackLimitVal = parseInt(stackLimit[1], 10);

// 6. cc: ManagedData 类型 id
if (!ccSrc.includes("static constexpr i::ManagedTypeId kTypeID =")) {
  throw new Error("未找到 ManagedData kTypeID");
}

// 7. cc: RefImpl 通用引用包装
if (!ccSrc.includes("class RefImpl {")) throw new Error("未找到 RefImpl");

// 8. cc: FrameImpl (栈检查)
if (!ccSrc.includes("struct FrameImpl {")) throw new Error("未找到 FrameImpl");

// 9. cc: StoreImpl::GetHostInfo 生命周期约束
if (!hSrc.includes("const i::DisallowGarbageCollection& no_gc")) {
  throw new Error("未找到 no_gc 生命周期");
}

console.log(`c-api: implement面=${impls.length} stackLimit=${stackLimitVal}`);

// 10. 生成 Rust
let rust = banner(
  "src/wasm/c-api.h.js",
  "src/wasm/c-api.{h,cc}",
  "wasm-c-api 实现层 (StoreImpl 与 implement 模板面)",
);

rust += `
//! wasm-api (third_party/wasm-api) 的 V8 实现层。
/// v8: src/wasm/c-api.h StoreImpl

/// 静态链接标志 (非共享构建时置 1)
/// v8: src/wasm/c-api.h LIBWASM_STATIC
pub const LIBWASM_STATIC: bool = true;

/// StoreImpl 状态 (一 Store 一 isolate 一 Eternal context)
/// v8: src/wasm/c-api.h StoreImpl
pub struct StoreImpl {
    /// v8: v8::Isolate::CreateParams create_params_
    pub create_params: IsolateCreateParams,
    /// v8: v8::Isolate* isolate_
    pub isolate: Option<*mut Isolate>,
    /// v8: v8::Eternal<v8::Context> context_
    pub context: Eternal<Context>,
    /// v8: host_info_map_ — host 数据挂 JSWeakMap (GetHostInfo 需 no_gc 域)
    pub host_info_map: IndirectHandle<JSWeakMap>,
}

/// implement<T> 特化面 (wasm-api 对象 → V8 内部表示)
/// v8: src/wasm/c-api.cc implement<Config/Engine/Store/ValType/...>
pub const IMPLEMENT_SPECIALIZATIONS: &[&str] = &[
${impls.map((i) => `    "${i}",`).join("\n")}
];

/// Counter 名字/值盒尺寸 (指标盒固定缓冲)
/// v8: src/wasm/c-api.cc EngineImpl
pub const COUNTER_NAME_BOX_SIZE: usize = 64;
pub const COUNTER_VALUE_BOX_SIZE: usize = 13;

/// 嵌套实例化深度上限
/// v8: src/wasm/c-api.cc kStackLimit
pub const STACK_LIMIT: usize = ${stackLimitVal};

/// ManagedData 模板携带类型 id (外部对象身份)
/// v8: src/wasm/c-api.cc ManagedData::kTypeID
pub const MANAGED_DATA_HAS_TYPE_ID: bool = true;

/// Ref<->JS 对象通用包装 (RefImpl<C, JSType>)
/// v8: src/wasm/c-api.cc RefImpl
pub const GENERIC_REF_IMPL: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(COUNTER_NAME_BOX_SIZE, 64);
    assert_eq!(COUNTER_VALUE_BOX_SIZE, 13);
    assert_eq!(STACK_LIMIT, ${stackLimitVal});
    assert!(IMPLEMENT_SPECIALIZATIONS.len() >= 10);
};
`;

const p = writeSrc("v8-wasm-core", "c_api.rs.skel", rust);
console.log(`written ${p}`);
