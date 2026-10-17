#!/usr/bin/env bun
// tool/src/wasm/wasm-memory-map-descriptor.h.js
// 机械提取 v8/src/wasm/wasm-memory-map-descriptor.{h,cc} 的 fd 所有权/映射检查链与 mmap 约定

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-memory-map-descriptor.h");
const ccSrc = readV8("src/wasm/wasm-memory-map-descriptor.cc");

// 1. h: FdOwnership 枚举
const enumMatch = hSrc.match(/enum class FdOwnership \{\s*kAnonymousFdOwnedByV8,\s*kFdOwnedByEmbedder,\s*\};/);
if (!enumMatch) throw new Error("未找到 FdOwnership 枚举");

// 2. h: 字段布局
for (const f of [
  "PlatformFileDescriptor file_descriptor_;",
  "size_t offset_;",
  "size_t size_;",
  "FdOwnership fd_ownership_;",
  "v8::Global<v8::WasmMemoryObject> mapped_memory_;",
]) {
  if (!hSrc.includes(f)) throw new Error(`字段缺失: ${f}`);
}

// 3. h: cppgc 指针 tag
if (!hSrc.includes("kPointerTag =\n      kWasmMemoryMapDescriptorTag;")) {
  throw new Error("未找到 kPointerTag");
}

// 4. cc: 平台门 (Linux 非 Android 才有实现)
if (!ccSrc.includes("#if V8_OS_LINUX && !V8_OS_ANDROID")) {
  throw new Error("未找到平台门");
}
const platformGateCount = (ccSrc.match(/#if V8_OS_LINUX && !V8_OS_ANDROID/g) || []).length;
if (platformGateCount !== 5) throw new Error(`平台门 ${platformGateCount} 处 != 5`);

// 5. cc: 匿名 fd 约定
if (!ccSrc.includes('memfd_create("wasm_memory_map_descriptor", MFD_CLOEXEC)')) {
  throw new Error("未找到 memfd_create 约定");
}
if (!ccSrc.includes("file_descriptor_ != -1")) throw new Error("未找到 -1 哨兵");
if (!ccSrc.includes("FdOwnership::kAnonymousFdOwnedByV8 &&")) {
  throw new Error("未找到析构关闭条件");
}

// 6. cc: Map 检查链 (按序)
const mapChecks = [
  'isolate->ThrowError("WasmMemoryMapDescriptor::Map called more than once")',
  "if (backing_store->is_shared())",
  "if (memory->is_memory64())",
  "if (size + offset < size)",
  "if (size + offset > backing_store->byte_length())",
];
for (const c of mapChecks) {
  if (!ccSrc.includes(c)) throw new Error(`Map 检查缺失: ${c}`);
}

// 7. cc: mmap 标志
if (!ccSrc.includes("PROT_READ | PROT_WRITE")) throw new Error("未找到 PROT 组合");
if (!ccSrc.includes("MAP_FIXED | MAP_SHARED, this->file_descriptor(), 0")) {
  throw new Error("未找到 Map 的 mmap 标志");
}
if (!ccSrc.includes("MAP_FIXED | MAP_SHARED | MAP_ANONYMOUS, -1, 0")) {
  throw new Error("未找到 Unmap 的 mmap 标志");
}

// 8. cc: 页对齐
if (!ccSrc.includes("RoundUp(stat_for_size.st_size,")) throw new Error("未找到 RoundUp 页对齐");
if (!ccSrc.includes("size_ % GetArrayBufferPageAllocator()->AllocatePageSize(), 0")) {
  throw new Error("未找到 Unmap 对齐检查");
}

console.log(
  `wasm-memory-map-descriptor: 枚举=2 平台门=${platformGateCount} Map检查=${mapChecks.length}`,
);

// 9. 生成 Rust
let rust = banner(
  "src/wasm/wasm-memory-map-descriptor.h.js",
  "src/wasm/wasm-memory-map-descriptor.{h,cc}",
  "WasmMemoryMapDescriptor fd 所有权与 mmap/unmap 契约",
);

rust += `
//! WebAssembly.Memory 内存映射描述符 (Linux memfd, wasm_memory_control flag)。
/// v8: src/wasm/wasm-memory-map-descriptor.h WasmMemoryMapDescriptor

/// fd 所有权
/// v8: src/wasm/wasm-memory-map-descriptor.h enum class FdOwnership
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FdOwnership {
    /// v8: kAnonymousFdOwnedByV8 — 析构时由 V8 close
    AnonymousFdOwnedByV8,
    /// v8: kFdOwnedByEmbedder — 生命周期归嵌入方
    FdOwnedByEmbedder,
}

/// 无效 fd 哨柄 (析构关闭检查用)
/// v8: src/wasm/wasm-memory-map-descriptor.cc 析构 "instead of hardcoding -1"
pub const INVALID_FD: i32 = -1;

/// 匿名描述符的 memfd 名与标志
/// v8: src/wasm/wasm-memory-map-descriptor.cc NewFromAnonymous
pub const MEMFD_NAME: &str = "wasm_memory_map_descriptor";
pub const MEMFD_CLOEXEC: bool = true;

/// Map 的 mmap 保护位与标志
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
pub const MAP_PROT: &str = "PROT_READ | PROT_WRITE";
pub const MAP_FLAGS: &str = "MAP_FIXED | MAP_SHARED";

/// Unmap 用匿名页覆盖原区域
/// v8: src/wasm/wasm-memory-map-descriptor.cc Unmap
pub const UNMAP_FLAGS: &str = "MAP_FIXED | MAP_SHARED | MAP_ANONYMOUS";

/// 重复 Map 的错误消息
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
pub const MAP_TWICE_ERROR: &str = "WasmMemoryMapDescriptor::Map called more than once";

/// Map 前置拒绝条件 (任一命中返回 0)
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
pub fn map_rejected(is_shared_memory: bool, is_memory64: bool) -> bool {
    is_shared_memory || is_memory64
}

/// 尺寸溢出/越界拒绝
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
pub fn map_size_rejected(size: usize, offset: usize, byte_length: usize) -> bool {
    size.checked_add(offset).map_or(true, |sum| sum > byte_length)
}

/// 文件尺寸向上取整到页尺寸后再判定
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map (RoundUp st_size)
pub fn round_up_to_page(file_size: usize, page_size: usize) -> usize {
    file_size.div_ceil(page_size) * page_size
}

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(INVALID_FD, -1);
    assert_eq!(round_up_to_page(0x1800, 0x1000), 0x2000);
    assert!(map_rejected(true, false));
    assert!(map_size_rejected(0x2000, 0x2000, 0x3000));
    assert!(!map_size_rejected(0x2000, 0x1000, 0x3000));
};
`;

const p = writeSrc("v8-wasm-core", "wasm_memory_map_descriptor.rs.skel", rust);
console.log(`written ${p}`);
