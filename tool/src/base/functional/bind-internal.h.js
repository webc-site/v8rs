#!/usr/bin/env bun
// tool/src/base/functional/bind-internal.h.js — 族: src/base/functional/bind-internal.h
// 覆盖: TypeList + ExtractArgs/ReturnType/CallableRunType + FunctorTraits/DecayedFunctorTraits
//        (lambda/函数指针/方法 3 主特化 + noexcept/const X-macro 6 展开)
// → v8-base/src/bind_internal.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/functional/bind-internal.h");

// —— 锚点断言 ——
if (!src.includes("struct TypeList {};")) throw new Error("缺 TypeList");
if (!src.includes("struct ExtractArgsImpl;")) throw new Error("缺 ExtractArgsImpl");
if (!src.includes("using ExtractArgs = typename ExtractArgsImpl<Signature>::ArgsList;")) throw new Error("缺 ExtractArgs");
if (!src.includes("using ExtractReturnType = typename ExtractArgsImpl<Signature>::ReturnType;")) throw new Error("缺 ExtractReturnType");
if (!src.includes("using ExtractCallableRunType =")) throw new Error("缺 ExtractCallableRunType");
if (!src.includes("struct FunctorTraits;")) throw new Error("缺 FunctorTraits 前置");
if (!src.includes("struct DecayedFunctorTraits;")) throw new Error("缺 DecayedFunctorTraits 前置");

// —— 真解析: X-macro 与特化计数 ——
const qualsExpansions = (src.match(/^BIND_INTERNAL_EXTRACT_CALLABLE_RUN_TYPE_WITH_QUALS\(/gm) || []).length;
if (qualsExpansions !== 4) throw new Error(`QUALS 展开 ${qualsExpansions} != 4`);
const constQualExpansions = (src.match(/^BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONST_AND_QUALS\(/gm) || []).length;
if (constQualExpansions !== 2) throw new Error(`CONST_AND_QUALS 展开 ${constQualExpansions} != 2`);
const decayedSpecs = [...src.matchAll(/^struct DecayedFunctorTraits</gm)].length;
if (decayedSpecs !== 5) throw new Error(`DecayedFunctorTraits 直接特化 ${decayedSpecs} != 5 (另有 X-macro 2 派生)`);
// 三主特化的标志位
if (!src.includes("requires HasNonOverloadedCallOp<Functor>")) throw new Error("缺 lambda/函对象特化");
if (!src.includes("struct DecayedFunctorTraits<R (*)(Args...), BoundArgs...> {")) throw new Error("缺函数指针特化");
if (!src.includes("struct DecayedFunctorTraits<R (Receiver::*)(Args...), BoundArgs...> {")) throw new Error("缺方法特化");
if (!src.includes("RunType = R(Receiver*, Args...);")) throw new Error("缺方法 RunType 改写");
console.log(`bind-internal: DecayedFunctorTraits 特化 ${decayedSpecs} (3 主 + noexcept 派生), X-macro 4+2, FunctorTraits 2 分派`);

let rust = banner(
  "src/base/functional/bind-internal.h.js",
  "src/base/functional/bind-internal.h",
  "bind 内部类型萃取 (Chromium 移植, Rust 泛型天然表达)",
);

rust += `
//! v8: src/base/functional/bind-internal.h

// v8: TypeList<Types...> — 类型列表; Rust 元编程用 tuple/cons 表达, 登记跳过
// v8: ExtractArgsImpl<R(Args...)> → { ReturnType: R, ArgsList: TypeList<Args...> }
// v8: ExtractArgs / ExtractReturnType — 签名拆解别名
// v8: ExtractCallableRunType — 经 operator() 的 4 种 qualifier
//     (无/const/noexcept/const noexcept, X-macro 展开) 取 R(Args...)

// v8: DecayedFunctorTraits 三主特化标志位表:
// | 特化 | RunType | is_method | is_nullable | is_stateless |
// | --- | --- | --- | --- | --- |
// | lambda/函对象 (HasNonOverloadedCallOp) | ExtractCallableRunType | false | false | is_empty |
// | 函数指针 R(*)(Args...) | R(Args...) | false | true | true |
// | 方法 R(Receiver::*)(Args...) | R(Receiver*, Args...) | true | true | true |
// 派生: noexcept 函数指针 / const 方法 (RunType → const Receiver*) /
//        const/noexcept 方法 (X-macro 2 展开)

// v8: FunctorTraits 双分派 — 一般走 decay 后的 DecayedFunctorTraits;
//     重载 operator() (HasOverloadedCallOp) 时用未 decay 型, RunType 由
//     declval<Functor>(bound_args...)(decay_t<BoundArgs>...) 现场合成

// Rust 侧: Fn/FnMut/FnOnce trait 体系 + impl Trait 直接覆盖绝大多数萃取需求;
//           方法指针场景由 Developer 按具体 bind 使用点定收接形态, 类型面登记跳过
`;

writeSrc("v8-base", "bind_internal.rs.skel", rust);
console.log("written v8-base/src/bind_internal.rs.skel");
