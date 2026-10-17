#!/usr/bin/env bun
// tool/src/base/functional/function-ref.h.js — 族: src/base/functional/function-ref.h
// 覆盖: FunctionRef<R(Args...)> — 非持有可调用引用 (包 absl::FunctionRef, 收紧转换)
// → v8-base/src/function_ref.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/functional/function-ref.h");

// —— 锚点断言 ——
if (!src.includes("#include \"absl/functional/function_ref.h\"")) throw new Error("缺 absl 依赖");
if (!src.includes("class FunctionRef;")) throw new Error("缺前置声明");
if (!src.includes("class FunctionRef<R(Args...)> {")) throw new Error("缺偏特化");
if (!src.includes("absl::FunctionRef<R(Args...)> wrapped_func_ref_;")) throw new Error("缺包装成员");
if (!src.includes("FunctionRef() = delete;")) throw new Error("缺默认构造禁用");
if (!src.includes("FunctionRef& operator=(const FunctionRef&) = delete;")) throw new Error("缺赋值禁用");

// —— 真解析: 兼容性约束构成 ——
if (!src.includes("std::convertible_to<internal::ExtractReturnType<RunType>, R>")) throw new Error("缺返回型转换约束");
if (!src.includes("std::same_as<internal::ExtractArgs<RunType>, internal::TypeList<Args...>>")) throw new Error("缺参数精确匹配约束");
if (!src.includes("(!is_instantiation<std::decay_t<Functor>, FunctionRef>)")) throw new Error("缺自引用排除");
if (!src.includes("(!is_instantiation<std::decay_t<Functor>, absl::FunctionRef>)")) throw new Error("缺 absl 排除");
if (!src.includes("FunctionRef(Func* func) : wrapped_func_ref_(func) {}")) throw new Error("缺函数指针特化构造");
console.log("function-ref: 构造 2 (泛型 functor + 函数指针), 兼容谓词 4 合取, 禁 2 (默认构造/赋值)");

let rust = banner(
  "src/base/functional/function-ref.h.js",
  "src/base/functional/function-ref.h",
  "非持有可调用引用 (trivially copyable, 无空态)",
);

rust += `
//! v8: src/base/functional/function-ref.h

/// v8: src/base/functional/function-ref.h FunctionRef<R(Args...)>
/// 语义: 类型擦除参数位专用 — 同步调用, 不拷贝不接管所有权;
///       无 null 态 (默认构造 delete); 禁赋值防悬垂; LIFETIME_BOUND 注记
/// 转换约束 (比 absl 严): 返回型可转换 (int→bool, D*→B*), 参数型必须精确 same_as;
/// 禁止静默丢弃返回值 (absl 允许 []{return 42;} 当 void() 用, 本类拒绝)
/// Rust 对应: &dyn Fn(Args...) -> R (引用天然非持有 + 无空态), 登记如下
pub type FunctionRef<'a, R, A> = &'a dyn Fn(A) -> R;

// v8: 兼容谓词 kCompatibleFunctor = convertible_to(RunType 的返回型, R)
//     && same_as(RunType 参数列, TypeList<Args...>)
//     && !is_instantiation<Functor, FunctionRef> && !is_instantiation<Functor, absl::FunctionRef>
// v8: FunctionRef(Func* func) — 函数指针构造 (豁免生命周期分析; 成员函数指针 TODO 登记)
`;

writeSrc("v8-base", "function_ref.rs.skel", rust);
console.log("written v8-base/src/function_ref.rs.skel");
