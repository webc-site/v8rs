#!/usr/bin/env bun
// tool/src/wasm/wasm-stack-wrapper-cache.h.js
// 机械提取 v8/src/wasm/wasm-stack-wrapper-cache.{h,cc} 的栈入口包装缓存契约

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-stack-wrapper-cache.h");
const ccSrc = readV8("src/wasm/wasm-stack-wrapper-cache.cc");

// 1. h: key 结构
if (!hSrc.includes("struct StackEntryWrapperCacheKey")) {
  throw new Error("未找到 StackEntryWrapperCacheKey");
}
if (!hSrc.includes("const CanonicalSig* sig;")) throw new Error("未找到 sig 成员");

// 2. h: hash = canonical type index
const hashMatch = hSrc.match(/return key\.sig->index\(\)\.index;/);
if (!hashMatch) throw new Error("未找到 Hash::operator() 实现");

// 3. h: 派生类与编译入口
if (!hSrc.includes("class WasmStackEntryWrapperCache")) {
  throw new Error("未找到 WasmStackEntryWrapperCache");
}
if (!hSrc.includes("std::pair<WasmCompilationResult, WasmCode::Kind> CompileWrapper")) {
  throw new Error("未找到 CompileWrapper override 声明");
}

// 4. h: 与基类同尺寸 static_assert
if (!hSrc.includes("static_assert(sizeof(WasmWrapperCache<StackEntryWrapperCacheKey>) ==\n              sizeof(WasmStackEntryWrapperCache));")) {
  throw new Error("未找到与基类同尺寸断言");
}

// 5. cc: 编译产物 kind
if (!ccSrc.includes("compiler::CompileWasmStackEntryWrapper(cache_key.sig);")) {
  throw new Error("未找到 CompileWasmStackEntryWrapper 调用");
}
if (!ccSrc.includes("WasmCode::kWasmStackEntryWrapper")) {
  throw new Error("未找到 kWasmStackEntryWrapper kind");
}
if (!ccSrc.includes("#if V8_ENABLE_TURBOFAN")) throw new Error("未找到 turbofan 门");

console.log("wasm-stack-wrapper-cache: key=1 hash=CanonicalSig::index kind=kWasmStackEntryWrapper");

// 6. 生成 Rust
let rust = banner(
  "src/wasm/wasm-stack-wrapper-cache.h.js",
  "src/wasm/wasm-stack-wrapper-cache.{h,cc}",
  "WasmStackEntryWrapperCache 键与编译契约",
);

rust += `
//! 栈切换入口 wrapper 的进程级缓存 (turbofan 编译)。
/// v8: src/wasm/wasm-stack-wrapper-cache.h WasmStackEntryWrapperCache

/// 缓存键: canonical 签名指针
/// v8: src/wasm/wasm-stack-wrapper-cache.h StackEntryWrapperCacheKey
#[derive(Debug)]
pub struct StackEntryWrapperCacheKey {
    /// v8: const CanonicalSig* sig
    pub sig: &'static CanonicalSig,
}

impl StackEntryWrapperCacheKey {
    /// hash 直接取 canonical type index
    /// v8: src/wasm/wasm-stack-wrapper-cache.h StackEntryWrapperCacheKey::Hash
    pub fn hash(&self) -> u32 {
        self.sig.index.index
    }
}

/// 编译产物代码类别 (WasmCode::Kind 变体名)
/// v8: src/wasm/wasm-stack-wrapper-cache.cc CompileWrapper
pub const STACK_ENTRY_WRAPPER_CODE_KIND: &str = "kWasmStackEntryWrapper";

/// 派生缓存类与泛型基类尺寸一致 (无新增字段)
/// v8: src/wasm/wasm-stack-wrapper-cache.h static_assert
pub const DERIVED_SAME_SIZE_AS_BASE: bool = true;

/// 编译入口 (turbofan 关闭时不可达)
/// v8: src/wasm/wasm-stack-wrapper-cache.cc WasmStackEntryWrapperCache::CompileWrapper
pub fn compile_wrapper(_sig: &CanonicalSig) -> WasmCompilationResult {
    // /// v8: compiler::CompileWasmStackEntryWrapper — 接 turbofan 编译层
    unimplemented!("CompileWasmStackEntryWrapper")
}

// —— 提取不变式断言 ——
const _: () = {
    assert!(DERIVED_SAME_SIZE_AS_BASE);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_stack_wrapper_cache.rs.skel", rust);
console.log(`written ${p}`);
