#!/usr/bin/env bun
// tool/src/wasm/wasm-engine.h.js
// 机械提取 v8/src/wasm/wasm-engine.h 的 NativeModuleCache 键序与引擎 API 面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-engine.h");

// 1. 两级结构
if (!hSrc.includes("class NativeModuleCache {")) throw new Error("未找到 NativeModuleCache");
if (!hSrc.includes("class V8_EXPORT_PRIVATE WasmEngine {")) throw new Error("未找到 WasmEngine");

// 2. NativeModuleCache::Key 排序键序 (operator< 逐段)
const ltMatch = hSrc.match(/bool operator<\(const Key& other\) const \{([\s\S]*?)\n  \};/);
if (!ltMatch) throw new Error("未找到 Key::operator<");
const lt = ltMatch[1];
const keyOrder = [];
if (lt.includes("if (prefix_hash != other.prefix_hash)")) keyOrder.push("prefix_hash");
if (lt.includes("if (bytes.size() != other.bytes.size())")) keyOrder.push("bytes.size()");
if (lt.includes("enabled_features <=> other.enabled_features")) keyOrder.push("enabled_features");
if (lt.includes("compile_imports <=> other.compile_imports")) keyOrder.push("compile_imports");
if (lt.includes("return memcmp(bytes.begin(), other.bytes.begin(), bytes.size()) < 0;")) keyOrder.push("memcmp(bytes)");
if (keyOrder.length !== 5) throw new Error(`键序 ${keyOrder.length} != 5`);

// 3. Key 相等三要素
if (!hSrc.includes("bool eq = bytes == other.bytes &&")) throw new Error("未找到相等判定");

// 4. 缓存表值语义
if (!hSrc.includes("std::map<Key, std::optional<std::weak_ptr<NativeModule>>> map_;")) {
  throw new Error("未找到 map 值语义");
}

// 5. NativeModuleCache API
for (const api of [
  "MaybeGetNativeModule(",
  "GetStreamingCompilationOwnership(",
  "StreamingCompilationFailed(",
  "static size_t PrefixHash(",
]) {
  if (!hSrc.includes(api)) throw new Error(`缓存 API 缺失: ${api}`);
}

// 6. WasmEngine 同步/异步 API 面
const engineApis = [
  "bool SyncValidate(",
  "MaybeDirectHandle<WasmModuleObject> SyncCompile(",
  "MaybeDirectHandle<WasmInstanceObject> SyncInstantiate(",
  "void AsyncCompile(",
  "void AsyncInstantiate(",
  "StartStreamingCompilation(",
  "void CompileFunction(NativeModule* native_module, uint32_t function_index,",
  "void EnterDebuggingForIsolate(",
  "void LeaveDebuggingForIsolate(",
  "MaybeDirectHandle<WasmModuleObject> ImportNativeModule(",
  "void FlushLiftoffCode();",
];
for (const api of engineApis) {
  if (!hSrc.includes(api)) throw new Error(`引擎 API 缺失: ${api}`);
}

// 7. 代码 GC 面
if (!hSrc.includes("using DeadCodeMap = std::unordered_map<NativeModule*, std::vector<WasmCode*>>;")) {
  throw new Error("未找到 DeadCodeMap");
}
for (const gc of ["ReportLiveCodeForGC(", "ReportLiveCodeFromStackForGC(", "AddPotentiallyDeadCode("]) {
  if (!hSrc.includes(gc)) throw new Error(`GC API 缺失: ${gc}`);
}

// 8. 非确定性标志 (进程级原子)
if (!hSrc.includes("return had_nondeterminism_.load(std::memory_order_relaxed);")) {
  throw new Error("未找到 had_nondeterminism 读");
}
if (!hSrc.includes("had_nondeterminism_.exchange(false, std::memory_order_relaxed);")) {
  throw new Error("未找到 had_nondeterminism 清零");
}

// 9. 进程级生命周期
if (!hSrc.includes("static void InitializeOncePerProcess();")) throw new Error("未找到 InitializeOncePerProcess");
if (!hSrc.includes("static void GlobalTearDown();")) throw new Error("未找到 GlobalTearDown");

console.log(`wasm-engine: 键序=${keyOrder.length} 引擎API=${engineApis.length}`);

// 10. 生成 Rust
let rust = banner(
  "src/wasm/wasm-engine.h.js",
  "src/wasm/wasm-engine.h",
  "WasmEngine 进程级引擎与 NativeModuleCache 键序",
);

rust += `
//! 进程级 wasm 引擎: 编译/实例化/代码 GC/NativeModule 缓存。
/// v8: src/wasm/wasm-engine.h WasmEngine

/// NativeModuleCache 键排序序 (字典序逐段比较)
/// v8: src/wasm/wasm-engine.h NativeModuleCache::Key::operator<
pub const NATIVE_MODULE_KEY_ORDER: &[&str] = &[
    "prefix_hash",
    "bytes.size",
    "enabled_features",
    "compile_imports",
    "memcmp_bytes",
];

/// 键相等三要素 (prefix_hash 冗余存储, 仅加速)
/// v8: src/wasm/wasm-engine.h NativeModuleCache::Key::operator==
pub const KEY_EQUALITY: &[&str] = &["bytes", "enabled_features", "compile_imports"];

/// 缓存条目三态
/// v8: src/wasm/wasm-engine.h map_ 注释
#[derive(Debug)]
pub enum CacheEntry {
    /// v8: nullopt — 某线程正在创建, 其他线程等待条件变量
    BeingCreated,
    /// v8: 有效 weak_ptr — 缓存的 NativeModule
    Alive(std::weak_ptr<NativeModule>),
    /// v8: 过期 weak_ptr — 模块已死, 待清理
    Expired,
}

/// PrefixHash: wire bytes 到 code section 头的哈希 (流式编译去重启发式)
/// v8: src/wasm/wasm-engine.h NativeModuleCache::PrefixHash
pub const PREFIX_HASH_COVERS_UP_TO_CODE_SECTION: bool = true;

/// 引擎级 API 面 (方法名清单)
/// v8: src/wasm/wasm-engine.h WasmEngine 公开方法
pub const ENGINE_API: &[&str] = &[
${engineApis.map((a) => `    "${a.replace(/[(;]/g, "").trim()}",`).join("\n")}
    "RemoveCompileJob",
    "AddIsolate",
    "RemoveIsolate",
    "LogCode",
    "LogWrapperCode",
    "EnableCodeLogging",
    "LogOutstandingCodesForIsolate",
    "NewNativeModule",
    "NewUnownedNativeModule",
    "UseNativeModuleInIsolate",
    "UpdateNativeModuleCache",
    "FreeNativeModule",
    "GetOrCreateScript",
    "EstimateCurrentMemoryConsumption",
    "InitializeOncePerProcess",
    "GlobalTearDown",
    "NativeModuleCount",
];

/// 进程级非确定性检测标志 (relaxed 原子)
/// v8: src/wasm/wasm-engine.h had_nondeterminism_
pub static HAD_NONDETERMINISM: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// 死代码映射 (NativeModule → 其死代码列表)
/// v8: src/wasm/wasm-engine.h DeadCodeMap
pub type DeadCodeMap = std::collections::HashMap<*mut NativeModule, Vec<*mut WasmCode>>;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(NATIVE_MODULE_KEY_ORDER.len(), 5);
    assert_eq!(KEY_EQUALITY.len(), 3);
    assert_eq!(ENGINE_API.len(), ${engineApis.length} + 21);
    assert!(!HAD_NONDETERMINISM.load(std::sync::atomic::Ordering::Relaxed));
};
`;

const p = writeSrc("v8-wasm-core", "wasm_engine.rs.skel", rust);
console.log(`written ${p}`);
