#!/usr/bin/env bun
// tool/src/base/contextual.h.js — 族: src/base/contextual.h
// 覆盖: ContextualVariable (CRTP + Scope 栈) + 2 声明宏 + ContextualClass 别名
//        + ContextualVariableWithDefault
// → v8-base/src/contextual.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/contextual.h");

// —— 锚点断言 ——
if (!src.includes("class V8_EXPORT_PRIVATE ContextualVariable {")) throw new Error("缺 ContextualVariable");
if (!src.includes("class V8_NODISCARD Scope {")) throw new Error("缺 Scope 嵌套类");
if (!src.includes("inline static thread_local Scope* top_ = nullptr;")) throw new Error("缺 thread_local top_");
if (!src.includes("static VarType& Get() {")) throw new Error("缺 Get");
if (!src.includes("static bool HasScope()")) throw new Error("缺 HasScope");
if (!src.includes("#define DECLARE_CONTEXTUAL_VARIABLE(VarName, ...)")) throw new Error("缺声明宏");
if (!src.includes("#define EXPORT_CONTEXTUAL_VARIABLE(VarName)")) throw new Error("缺导出宏");
if (!src.includes("using ContextualClass = ContextualVariable<T, T>;")) throw new Error("缺 ContextualClass");
if (!src.includes("class V8_EXPORT_PRIVATE ContextualVariableWithDefault")) throw new Error("缺 WithDefault");
if (!src.includes("inline static thread_local VarType default_value_{default_args...};")) throw new Error("缺 default_value_");

// —— 真解析: Scope 生命周期语义锚 ——
if (!src.includes("Top() = this;")) throw new Error("缺构造压栈");
if (!src.includes("DCHECK_EQ(this, Top());")) throw new Error("缺析构栈纪律检查");
const macroCount = (src.match(/^#define DECLARE_CONTEXTUAL/gm) || []).length;
if (macroCount !== 2) throw new Error(`DECLARE 宏数 ${macroCount} != 2`);
console.log("contextual: 类 2 (Variable + WithDefault), 嵌套 Scope 1, 声明宏 2, 别名 1");

let rust = banner(
  "src/base/contextual.h.js",
  "src/base/contextual.h",
  "嵌套作用域式线程局部全局变量 (scope stack)",
);

rust += `
//! v8: src/base/contextual.h

/// v8: src/base/contextual.h ContextualVariable<Derived, VarType>
/// CRTP; 仅存指向当前 Scope 的指针 (thread_local top_), 真值存于 Scope 栈上对象;
/// Get() 要求至少一个活跃 Scope; 仅同线程使用
pub struct ContextualVariable<V> {
    /// v8: inline static thread_local Scope* top_ = nullptr
    top: std::cell::Cell<*mut Scope<V>>,
}

/// v8: src/base/contextual.h ContextualVariable::Scope
/// 构造: value_(args...), previous_(Top()), Top()=this (压栈)
/// 析构: DCHECK 栈纪律后回滚 previous_ — Rust 侧 Drop 保证
pub struct Scope<V> {
    value: V,
    previous: *mut Scope<V>,
}

impl<V> Scope<V> {
    /// v8: Scope::Scope(Args&&...) — 压栈
    pub fn new(value: V) -> Self;
    /// v8: Scope::Value
    pub fn value(&mut self) -> &mut V;
}

impl<V> Drop for Scope<V> {
    /// v8: ~Scope — Top() = previous_; 原版 DCHECK_EQ(this, Top()) 栈纪律
    fn drop(&mut self);
}

impl<V> ContextualVariable<V> {
    /// v8: ContextualVariable::Get — Top()->Value(); 无 Scope 为 UB (DCHECK)
    pub fn get() -> &'static V;
    /// v8: ContextualVariable::HasScope — Top() != nullptr
    pub fn has_scope() -> bool;
}

// v8: DECLARE_CONTEXTUAL_VARIABLE(VarName, VarType) — struct VarName : ContextualVariable<VarName, VarType> {}
// v8: EXPORT_CONTEXTUAL_VARIABLE(VarName) — USING_V8_SHARED 时的 DLL 导出特化, Rust 无对应, 登记跳过

/// v8: src/base/contextual.h ContextualClass<T> = ContextualVariable<T, T>
/// v8: src/base/contextual.h ContextualVariableWithDefault<Derived, VarType, default_args...>
/// Get(): HasScope() ? Base::Get() : default_value_ (thread_local 默认值)
pub struct ContextualVariableWithDefault<V> { default_value: V }
`;

writeSrc("v8-base", "contextual.rs.skel", rust);
console.log("written v8-base/src/contextual.rs.skel");
