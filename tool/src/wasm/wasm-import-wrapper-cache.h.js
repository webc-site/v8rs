#!/usr/bin/env bun
// tool/src/wasm/wasm-import-wrapper-cache.h.js
// 机械提取 v8/src/wasm/wasm-import-wrapper-cache.{h,cc} 的键组合与 builtin 短路规则

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-import-wrapper-cache.h");
const ccSrc = readV8("src/wasm/wasm-import-wrapper-cache.cc");

// 1. h: 键字段
for (const f of [
  "ImportCallKind kind;",
  "const CanonicalSig* sig;",
  "int expected_arity;",
  "Suspend suspend;",
]) {
  if (!hSrc.includes(f)) throw new Error(`键字段缺失: ${f}`);
}

// 2. h: hash 组合
const hashMatch = hSrc.match(
  /return base::hash_combine\(static_cast<uint8_t>\(key\.kind\),\s*key\.sig->index\(\), key\.expected_arity\);/,
);
if (!hashMatch) throw new Error("未找到 hash_combine 组合");

// 3. cc: WasmWrapperHandle 生命周期 (AllocateAndInitializeEntry / FreeEntry / DecRef)
if (!ccSrc.includes("GetProcessWideWasmCodePointerTable()->AllocateAndInitializeEntry(")) {
  throw new Error("未找到表项分配");
}
if (!ccSrc.includes("GetProcessWideWasmCodePointerTable()->FreeEntry(code_pointer_);")) {
  throw new Error("未找到表项释放");
}
if (!ccSrc.includes("bool should_free = wasm_code->DecRef();")) {
  throw new Error("未找到 DecRef");
}
if (!ccSrc.includes("if (code) code->IncRef();")) throw new Error("未找到 IncRef");

// 4. cc: builtin 短路链 (按序)
for (const a of [
  "if (key.kind != ImportCallKind::kJSFunction) return {};",
  "if (!IsJSCompatibleSignature(key.sig)) {\n    return Builtin::kWasmToJsWrapperInvalidSig;",
  "if (UseGenericWasmToJSWrapper(key.kind, key.sig, key.suspend)) {\n    return Builtin::kWasmToJsWrapperAsm;",
]) {
  if (!ccSrc.includes(a)) throw new Error(`builtin 短路缺失: ${a}`);
}

// 5. cc: CompileWrapper 的 kind 分派
const unreachableKinds = [...ccSrc.matchAll(
  /case ImportCallKind::(\w+):\s*\n\s*(?:case ImportCallKind::\w+:\s*\n\s*)*UNREACHABLE\(\);/g,
)].flatMap((m) => [...m[0].matchAll(/ImportCallKind::(\w+)/g)].map((x) => x[1]));
if (unreachableKinds.join(",") !== "kLinkError,kWasmToWasm,kWasmToJSFastApi") {
  throw new Error(`不可编译 kind 异常: ${unreachableKinds.join(",")}`);
}
const tfKinds = [...ccSrc.matchAll(/case ImportCallKind::(\w+):/g)].map((m) => m[1]);
if (!ccSrc.includes("code_kind = WasmCode::Kind::kWasmToJsWrapper;")) {
  throw new Error("未找到 kWasmToJsWrapper");
}
if (!ccSrc.includes("code_kind = wasm::WasmCode::Kind::kWasmToCapiWrapper;")) {
  throw new Error("未找到 kWasmToCapiWrapper");
}

// 6. cc: fast-call wrapper 不缓存
if (!ccSrc.includes("// Note: the wrapper we're about to compile is specific to this")) {
  throw new Error("未找到 fast-call 不缓存注释");
}

// 7. cc: Get 的 builtin 分支直接造 handle 不编译
if (!ccSrc.includes("Address call_target = Builtins::EmbeddedEntryOf(*builtin);")) {
  throw new Error("未找到 EmbeddedEntryOf");
}

console.log(
  `wasm-import-wrapper-cache: 键字段=4 不可编译=${unreachableKinds.length} case总数=${tfKinds.length}`,
);

// 8. 生成 Rust
let rust = banner(
  "src/wasm/wasm-import-wrapper-cache.h.js",
  "src/wasm/wasm-import-wrapper-cache.{h,cc}",
  "WasmImportWrapperCache 键、builtin 短路与编译分派",
);

rust += `
//! Wasm-to-JS 导入 wrapper 进程级缓存 + builtin 短路。
/// v8: src/wasm/wasm-import-wrapper-cache.h WasmImportWrapperCache

/// 缓存键四元组
/// v8: src/wasm/wasm-import-wrapper-cache.h ImportWrapperCacheKey
#[derive(Debug)]
pub struct ImportWrapperCacheKey<'a> {
    /// v8: ImportCallKind kind
    pub kind: ImportCallKind,
    /// v8: const CanonicalSig* sig
    pub sig: &'a CanonicalSig,
    /// v8: int expected_arity
    pub expected_arity: i32,
    /// v8: Suspend suspend
    pub suspend: Suspend,
}

/// hash 组合: (u8)kind ⊕ sig.index ⊕ expected_arity (suspend 不入 hash)
/// v8: src/wasm/wasm-import-wrapper-cache.h ImportWrapperCacheKey::Hash
pub fn key_hash(kind: u8, sig_index: u32, expected_arity: i32) -> u64 {
    hash_combine(&[kind as u64, sig_index as u64, expected_arity as u64])
}

/// builtin 短路表 (仅 kJSFunction kind)
/// v8: src/wasm/wasm-import-wrapper-cache.cc BuiltinForWrapper
pub fn builtin_for_wrapper(kind: ImportCallKind, js_compatible: bool, use_generic: bool) -> Option<&'static str> {
    if kind != ImportCallKind::JSFunction {
        return None;
    }
    if !js_compatible {
        return Some("kWasmToJsWrapperInvalidSig");
    }
    if use_generic {
        return Some("kWasmToJsWrapperAsm");
    }
    None
}

/// 需要真编译的 kind → 产物代码类别
/// v8: src/wasm/wasm-import-wrapper-cache.cc CompileWrapper
pub fn compile_kind(kind: ImportCallKind) -> Option<&'static str> {
    Some(match kind {
        ImportCallKind::JSFunction
        | ImportCallKind::UseCallBuiltin
        | ImportCallKind::RuntimeTypeError => "kWasmToJsWrapper",
        ImportCallKind::WasmToCapi => "kWasmToCapiWrapper",
        ImportCallKind::LinkError | ImportCallKind::WasmToWasm | ImportCallKind::WasmToJSFastApi => {
            return None // UNREACHABLE
        }
    })
}

/// fast-call wrapper 与实例化绑定, 不进缓存
/// v8: src/wasm/wasm-import-wrapper-cache.cc CompileWasmJsFastCallWrapper
pub const FAST_CALL_WRAPPER_NOT_CACHED: bool = true;

/// handle 生命周期: 构造时占用 code pointer 表项, 析构时释放并 DecRef
/// v8: src/wasm/wasm-import-wrapper-cache.cc WasmWrapperHandle 构造/析构
pub const HANDLE_OWNS_TABLE_ENTRY: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert!(FAST_CALL_WRAPPER_NOT_CACHED);
    assert_eq!(compile_kind(ImportCallKind::WasmToCapi), Some("kWasmToCapiWrapper"));
    assert_eq!(compile_kind(ImportCallKind::LinkError), None);
    assert_eq!(
        builtin_for_wrapper(ImportCallKind::JSFunction, false, true),
        Some("kWasmToJsWrapperInvalidSig")
    );
};
`;

const p = writeSrc("v8-wasm-core", "wasm_import_wrapper_cache.rs.skel", rust);
console.log(`written ${p}`);
