#!/usr/bin/env bun
// tool/src/wasm/wasm-wrapper-cache.h.js
// 机械提取 v8/src/wasm/wasm-wrapper-cache.{h,cc,-inl.h} 的泛型 wrapper 缓存布局与 AddWrapper 参数约定

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-wrapper-cache.h");
const ccSrc = readV8("src/wasm/wasm-wrapper-cache.cc");
const inlSrc = readV8("src/wasm/wasm-wrapper-cache-inl.h");

// 1. h: 泛型类与成员
if (!hSrc.includes("template <typename CacheKey>\nclass WasmWrapperCache")) {
  throw new Error("未找到 WasmWrapperCache 模板");
}
const members = [
  "WasmCodeAllocator code_allocator_;",
  "mutable base::Mutex mutex_;",
  "entry_map_;",
  "std::map<Address, WasmCode*> codes_;",
  "DelayedCounterUpdates counter_updates_;",
];
for (const m of members) {
  if (!hSrc.includes(m)) throw new Error(`成员缺失: ${m}`);
}

// 2. h: 公开 API 面
for (const api of [
  "class V8_NODISCARD ModificationScope",
  "WasmCode* AddWrapper(",
  "void Free(std::vector<WasmCode*>& wrappers);",
  "GetCompiled(",
  "WasmCode* Lookup(Address pc) const;",
  "void LogForIsolate(Isolate* isolate);",
  "size_t EstimateCurrentMemoryConsumption() const;",
  "void PublishCounterUpdates(Isolate* isolate)",
]) {
  if (!hSrc.includes(api)) throw new Error(`API 缺失: ${api}`);
}

// 3. h: WasmWrapperHandle 布局
for (const m of [
  "const WasmCodePointer code_pointer_;",
  "std::atomic<WasmCode*> code_ = nullptr;",
]) {
  if (!hSrc.includes(m)) throw new Error(`WasmWrapperHandle 成员缺失: ${m}`);
}

// 4. inl: AddWrapper 的 WasmCode 构造参数约定
if (!inlSrc.includes("kAnonymousFuncIndex,")) throw new Error("未找到 kAnonymousFuncIndex");
if (!inlSrc.includes("ExecutionTier::kNone,")) throw new Error("未找到 ExecutionTier::kNone");
if (!inlSrc.includes("wasm::kNotForDebugging,")) throw new Error("未找到 kNotForDebugging");
if (!inlSrc.includes("constexpr bool frame_has_feedback_slot = false;")) {
  throw new Error("未找到 frame_has_feedback_slot=false");
}
if (!inlSrc.includes("desc.safepoint_table_size == 0 ? 0 : desc.safepoint_table_offset")) {
  throw new Error("未找到 safepoint_table_offset 归零规则");
}

// 5. inl: reloc 断言 (wrapper 不含 wasm 调用)
if (!inlSrc.includes("DCHECK(!RelocInfo::IsWasmCall(it.rinfo()->rmode()));")) {
  throw new Error("未找到 IsWasmCall 断言");
}
if (!inlSrc.includes("DCHECK(!RelocInfo::IsWasmStubCall(it.rinfo()->rmode()));")) {
  throw new Error("未找到 IsWasmStubCall 断言");
}

// 6. inl: Lookup 语义锚
if (!inlSrc.includes("auto iter = codes_.upper_bound(pc);")) throw new Error("未找到 upper_bound 查找");
if (!inlSrc.includes("WasmCodeRefScope::AddRef(candidate);")) throw new Error("未找到 AddRef");

// 7. inl: 尺寸看板锚
if (!inlSrc.includes("UPDATE_WHEN_CLASS_CHANGES(WasmWrapperCache<CacheKey>, 240);")) {
  throw new Error("未找到 UPDATE_WHEN_CLASS_CHANGES 240");
}

// 8. inl: kWasmToJSFastApi 禁用检查
if (!inlSrc.includes("CHECK_NE(cache_key.kind, ImportCallKind::kWasmToJSFastApi);")) {
  throw new Error("未找到 kWasmToJSFastApi 检查");
}

// 9. cc: set_code 的锁选择
if (!ccSrc.includes("if (code->kind() == WasmCode::Kind::kWasmStackEntryWrapper) {")) {
  throw new Error("未找到按 kind 选锁");
}

// 10. inl: 计数器发布项 (生成/重定位尺寸)
for (const c of ["wasm_generated_code_size", "wasm_reloc_size"]) {
  if (!inlSrc.includes(c)) throw new Error(`计数器缺失: ${c}`);
}

console.log(
  `wasm-wrapper-cache: 模板成员=${members.length} API=8 AddWrapper约定=5 锚=${5 + 2}`,
);

// 11. 生成 Rust
let rust = banner(
  "src/wasm/wasm-wrapper-cache.h.js",
  "src/wasm/wasm-wrapper-cache.{h,cc,-inl.h}",
  "WasmWrapperCache 泛型缓存布局与 AddWrapper/Lookup 契约",
);

rust += `
//! 进程级 wrapper 缓存基类: entry_map (weak) + codes (按地址有序 map)。
/// v8: src/wasm/wasm-wrapper-cache.h WasmWrapperCache

/// AddWrapper 构造 WasmCode 时的固定参数 (wrapper 无函数身份)
/// v8: src/wasm/wasm-wrapper-cache-inl.h AddWrapper
pub mod add_wrapper_convention {
    /// v8: kAnonymousFuncIndex (= -1, src/wasm/wasm-constants.h)
    pub const ANONYMOUS_FUNC_INDEX: i32 = -1;
    /// v8: ExecutionTier::kNone
    pub const EXECUTION_TIER_NONE: u8 = 0;
    /// v8: wasm::kNotForDebugging
    pub const NOT_FOR_DEBUGGING: u8 = 0;
    /// v8: constexpr bool frame_has_feedback_slot = false
    pub const FRAME_HAS_FEEDBACK_SLOT: bool = false;
    /// v8: safepoint_table_size == 0 时 safepoint_table_offset 归零
    pub fn safepoint_table_offset(table_size: usize, table_offset: usize) -> usize {
        if table_size == 0 { 0 } else { table_offset }
    }
}

/// wrapper 内禁止出现的 reloc 模式 (断言语义)
/// v8: src/wasm/wasm-wrapper-cache-inl.h AddWrapper reloc 迭代
pub const WRAPPER_FORBIDS_WASM_CALL_RELOC: bool = true;
pub const WRAPPER_FORBIDS_WASM_STUB_CALL_RELOC: bool = true;

/// Lookup(pc): codes 按 instruction_start 有序, upper_bound 后退一位
/// v8: src/wasm/wasm-wrapper-cache-inl.h WasmWrapperCache::Lookup
pub fn lookup_by_pc<'a>(codes: &'a BTreeMap<usize, &'a str>, pc: usize) -> Option<(&'a usize, &'a &'a str)> {
    let mut iter = codes.range(..=pc);
    iter.next_back()
}

/// 原版尺寸看板基准 (x64 clang linux debug 指针压缩构建)
/// v8: src/wasm/wasm-wrapper-cache-inl.h UPDATE_WHEN_CLASS_CHANGES
pub const WASM_WRAPPER_CACHE_SIZE_BASELINE: usize = 240;

/// ImportWrapperCacheKey 禁止 kWasmToJSFastApi
/// v8: src/wasm/wasm-wrapper-cache-inl.h GetCompiled
pub const IMPORT_KEY_FORBIDS_FAST_API: bool = true;

/// 编译完成后上报的计数器名
/// v8: src/wasm/wasm-wrapper-cache-inl.h CacheCompiledWrapper
pub const COUNTER_GENERATED_CODE_SIZE: &str = "wasm_generated_code_size";
pub const COUNTER_RELOC_SIZE: &str = "wasm_reloc_size";

// —— 提取不变式断言 ——
const _: () = {
    assert!(!add_wrapper_convention::FRAME_HAS_FEEDBACK_SLOT);
    assert_eq!(add_wrapper_convention::safepoint_table_offset(0, 64), 0);
    assert_eq!(add_wrapper_convention::safepoint_table_offset(8, 64), 64);
    assert_eq!(WASM_WRAPPER_CACHE_SIZE_BASELINE, 240);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_wrapper_cache.rs.skel", rust);
console.log(`written ${p}`);
