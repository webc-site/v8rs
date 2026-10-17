#!/usr/bin/env bun
// tool/src/base/lazy-instance.h.js — 族: src/base/lazy-instance.h
// 覆盖: LazyInstanceImpl + 6 个 Trait + LazyInstance/LazyStaticInstance/LazyDynamicInstance
//        + LeakyObject + 4 个初始化宏 + DEFINE_LAZY_LEAKY_OBJECT_GETTER
// → v8-base/src/lazy_instance.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/lazy-instance.h");

// —— 锚点断言 ——
const structs = [
  "LeakyInstanceTrait", "StaticallyAllocatedInstanceTrait", "DynamicallyAllocatedInstanceTrait",
  "DefaultConstructTrait", "DefaultCreateTrait", "ThreadSafeInitOnceTrait",
  "SingleThreadInitOnceTrait", "LazyInstanceImpl", "LazyStaticInstance", "LazyInstance",
  "LazyDynamicInstance", "LeakyObject",
];
for (const s of structs) if (!src.includes(`struct ${s}`) && !src.includes(`class ${s}`)) throw new Error(`缺 ${s}`);

// —— 真解析: 宏与默认模板参 ——
const initMacros = [...src.matchAll(/^#define (LAZY_\w+_INITIALIZER)/gm)].map((m) => m[1]);
if (initMacros.join(",") !== "LAZY_STATIC_INSTANCE_INITIALIZER,LAZY_DYNAMIC_INSTANCE_INITIALIZER,LAZY_INSTANCE_INITIALIZER") {
  throw new Error(`初始化宏 ${initMacros}`);
}
if (!src.includes("#define LAZY_STATIC_INSTANCE_INITIALIZER { V8_ONCE_INIT, { {} } }")) throw new Error("缺 static 初值");
if (!src.includes("#define LAZY_DYNAMIC_INSTANCE_INITIALIZER { V8_ONCE_INIT, 0 }")) throw new Error("缺 dynamic 初值");
if (!src.includes("#define DEFINE_LAZY_LEAKY_OBJECT_GETTER(T, FunctionName, ...)")) throw new Error("缺 getter 宏");
if (!src.includes("alignas(AlignmentType) mutable StorageType storage_;")) throw new Error("缺存储成员");
if (!src.includes("alignas(T) char storage_[sizeof(T)];")) throw new Error("缺 LeakyObject 存储");
console.log(`lazy-instance: struct ${structs.length}, 初始化宏 ${initMacros.length}+getter 1, 方法 Pointer/Get 2`);

let rust = banner(
  "src/base/lazy-instance.h.js",
  "src/base/lazy-instance.h",
  "惰性单实例 (线程安全 once + 静态/动态两模式)",
);

rust += `
//! v8: src/base/lazy-instance.h
//! Rust 对应: std::sync::Once + OnceLock / LazyLock; 结构面登记如下

/// v8: src/base/lazy-instance.h LazyInstanceImpl<T, Alloc, Create, InitOnce, Destroy>
/// 存储: mutable OnceType once_ + alignas(T) StorageType storage_ (静态模式 char[sizeof(T)])
pub struct LazyInstanceImpl<T> {
    once: std::sync::Once,
    storage: std::cell::UnsafeCell<MaybeUninit<T>>,
}

impl<T> LazyInstanceImpl<T> {
    /// v8: LazyInstanceImpl::Pointer — Init() 后返回 T*
    pub fn pointer(&self) -> *mut T;
    /// v8: LazyInstanceImpl::Get — Init() 后返回 const T&
    pub fn get(&self) -> &T;
    /// v8: LazyInstanceImpl::Init — InitOnceTrait::Init(CallOnce) 一次性构造
    fn init(&self);
}

// v8: Trait 面 (Rust 侧由泛型参数/闭包表达):
// - LeakyInstanceTrait: Destroy 空操作 (泄漏式, 静态变量友好)
// - StaticallyAllocatedInstanceTrait: StorageType = char[sizeof(T)] (全局数据段)
// - DynamicallyAllocatedInstanceTrait: StorageType = T* (堆分配, 配 LAZY_DYNAMIC_INSTANCE_INITIALIZER)
// - DefaultConstructTrait: placement-new T()
// - DefaultCreateTrait: new T 返回指针
// - ThreadSafeInitOnceTrait: CallOnce (默认)
// - SingleThreadInitOnceTrait: 查 ONCE_STATE_UNINITIALIZED 后直调 (无线程安全)
// v8: LazyInstance / LazyStaticInstance / LazyDynamicInstance 三入口模板别名

/// v8: src/base/lazy-instance.h LeakyObject<T>
/// 构造时 placement-new, 永不析构 (trivially destructible), 用于静态惰性变量
pub struct LeakyObject<T> { storage: [u8; std::mem::size_of::<T>()] }

impl<T> LeakyObject<T> {
    /// v8: LeakyObject::get — reinterpret_cast<T*>(storage_)
    pub fn get(&self) -> &T;
}

// v8: DEFINE_LAZY_LEAKY_OBJECT_GETTER(T, FunctionName, ...) — 函数级 static LeakyObject;
//     Rust: fn f() -> &'static T { static O: OnceLock<T> ... }, 宏登记跳过
`;

writeSrc("v8-base", "lazy_instance.rs.skel", rust);
console.log("written v8-base/src/lazy_instance.rs.skel");
