#!/usr/bin/env bun
// tool/src/base/free_deleter.h.js — 族: src/base/free_deleter.h
// 覆盖: FreeDeleter 函数对象 (unique_ptr 配 free 释放)
// → v8-base/src/free_deleter.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/free_deleter.h");

// —— 锚点断言 ——
if (!src.includes("struct FreeDeleter {")) throw new Error("缺 FreeDeleter");
if (!src.includes("inline void operator()(void* ptr) const { base::Free(ptr); }")) throw new Error("缺 operator() 委托 base::Free");
if (!src.includes("#include \"src/base/platform/memory.h\"")) throw new Error("缺 platform/memory include");
console.log("free_deleter: 函数对象 1");

let rust = banner(
  "src/base/free_deleter.h.js",
  "src/base/free_deleter.h",
  "free 释放器, Rust Drop/自定义分配器表达",
);

rust += `
//! v8: src/base/free_deleter.h

// v8: struct FreeDeleter { void operator()(void* ptr) const { base::Free(ptr); } }
// 用途: std::unique_ptr<T, base::FreeDeleter> 包 malloc 分配指针
// Rust 对应: 由自定义 allocator / Drop 实现; libc::free 直调
// 登记跳过 (无独立类型面), 语义锚定 base::Free (platform/memory, 别席)
`;

writeSrc("v8-base", "free_deleter.rs.skel", rust);
console.log("written v8-base/src/free_deleter.rs.skel");
