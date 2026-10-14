#!/usr/bin/env bun
// tool/src/wasm/wasm-memory-map-descriptor.cc.js
// 机械提取 v8/src/wasm/wasm-memory-map-descriptor.cc 的 memfd 命名、平台门控、
// 映射检查清单与错误消息。头侧骨架在 v8-wasm-core/src/wasm_memory_map_descriptor.rs.skel。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/wasm-memory-map-descriptor.cc");

// ---------- 1. 平台门控: 仅 Linux 且非 Android ----------
if (!src.includes("#if V8_OS_LINUX && !V8_OS_ANDROID")) {
  throw new Error("Linux 非 Android 门控缺失");
}

// ---------- 2. memfd 命名与标志 ----------
if (!src.includes('memfd_create("wasm_memory_map_descriptor", MFD_CLOEXEC)')) {
  throw new Error("memfd 命名缺失");
}

// ---------- 3. 匿名 fd 语义: V8 拥有, 关闭时回收, -1 哨兵 ----------
if (!src.includes("FdOwnership::kAnonymousFdOwnedByV8 &&\n      file_descriptor_ != -1")) {
  throw new Error("匿名 fd 回收语义缺失");
}
if (!src.includes("// instead of hardcoding -1 here.")) throw new Error("-1 哨兵注释缺失");

// ---------- 4. 双次映射防线与错误消息 ----------
if (!src.includes('isolate->ThrowError("WasmMemoryMapDescriptor::Map called more than once");')) {
  throw new Error("双次映射错误消息缺失");
}

// ---------- 5. Map 拒绝条件 (返回 0) ----------
const mapRejects = [];
if (src.includes("// so far.")) mapRejects.push("共享内存");
if (src.includes("// we need to test it.")) mapRejects.push("memory64");
if (/\/\/ Could not determine file size\./.test(src)) mapRejects.push("fstat 失败");
if (/\/\/ Overflow/.test(src)) mapRejects.push("size+offset 溢出");
if (/size \+ offset > backing_store->byte_length\(\)/.test(src)) mapRejects.push("越界 buffer");
if (/ret_val == MAP_FAILED/.test(src)) mapRejects.push("mmap 失败");
if (mapRejects.length !== 6) throw new Error(`Map 拒绝条件 ${mapRejects.length} != 6`);

// ---------- 6. 尺寸对齐: 文件大小向上取整到页分配器页宽 ----------
if (!src.includes("RoundUp(stat_for_size.st_size,")) throw new Error("文件尺寸取整缺失");
if (!src.includes("GetArrayBufferPageAllocator()->AllocatePageSize()")) {
  throw new Error("页分配器页宽缺失");
}

// ---------- 7. mmap 保护位: R|W, FIXED|SHARED; Unmap 用 ANONYMOUS 覆盖 ----------
if (!src.includes("mmap(target, size, PROT_READ | PROT_WRITE,")) throw new Error("mmap 保护位缺失");
if (!src.includes("MAP_FIXED | MAP_SHARED, this->file_descriptor(), 0)")) {
  throw new Error("映射标志缺失");
}
if (!src.includes("MAP_FIXED | MAP_SHARED | MAP_ANONYMOUS, -1, 0)")) {
  throw new Error("解除映射标志缺失");
}

// ---------- 8. Unmap 后置检查 (Map 时已通过, 仍复查) ----------
const unmapChecks = [];
if (src.includes("CHECK(!memory->is_memory64());")) unmapChecks.push("非 memory64");
if (src.includes("CHECK(!backing_store->is_shared());")) unmapChecks.push("非共享");
if (src.includes("CHECK_EQ(size_ % GetArrayBufferPageAllocator()->AllocatePageSize(), 0);")) unmapChecks.push("尺寸页对齐");
if (/CHECK_GE\(size_ \+ offset_, size_\);/.test(src)) unmapChecks.push("无溢出");
if (/CHECK_LE\(size_ \+ offset_, backing_store->byte_length\(\)\);/.test(src)) unmapChecks.push("界内");
if (unmapChecks.length !== 5) throw new Error(`Unmap 复查 ${unmapChecks.length} != 5`);

// ---------- 9. 弱引用: 映射后置弱, 防实例泄漏 ----------
if (!/mapped_memory_\.Reset\(isolate, Utils::ToLocal\(memory\)\);\s*mapped_memory_\.SetWeak\(\);/.test(src)) {
  throw new Error("弱引用持有缺失");
}

console.log(`wasm-memory-map-descriptor: Map拒绝=${mapRejects.length} Unmap复查=${unmapChecks.length}`);

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/wasm-memory-map-descriptor.cc.js",
  "src/wasm/wasm-memory-map-descriptor.cc",
  "内存映射描述符平台门控与映射检查清单",
)}
//! WasmMemoryMapDescriptor: 文件描述符映射进 wasm 线性内存。

/// 平台门控: 仅 Linux 且非 Android 实现, 其余平台 Map/Unmap/NewFromAnonymous 直接失败
/// v8: src/wasm/wasm-memory-map-descriptor.cc V8_OS_LINUX && !V8_OS_ANDROID
pub const PLATFORM_GATE: &str = "V8_OS_LINUX && !V8_OS_ANDROID";

/// 匿名 fd 的 memfd 名与标志
/// v8: src/wasm/wasm-memory-map-descriptor.cc NewFromAnonymous
pub const MEMFD_NAME: &str = "wasm_memory_map_descriptor";
pub const MEMFD_FLAGS: &str = "MFD_CLOEXEC";
/// V8 拥有的匿名 fd 在析构时关闭 (-1 为无效哨兵)
/// v8: src/wasm/wasm-memory-map-descriptor.cc ~WasmMemoryMapDescriptor
pub const INVALID_FD: i32 = -1;

/// 双次映射错误消息
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
pub const DOUBLE_MAP_ERROR: &str = "WasmMemoryMapDescriptor::Map called more than once";

/// Map 拒绝条件 (返回 0, 共 6 条)
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
#[rustfmt::skip]
pub static MAP_REJECTS: &[&str] = &[
    "已映射过 (双次调用)",
    "共享内存 (并发未处理)",
    "memory64 (待测试)",
    "fstat 取文件大小失败",
    "size + offset 溢出",
    "size + offset 越界 backing_store.byte_length",
];

/// 文件大小向上取整到页分配器页宽后才映射
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
pub const SIZE_ROUNDING: &str = "RoundUp(st_size, AllocatePageSize)";

/// mmap 标志: Map 用 PROT_READ|PROT_WRITE + MAP_FIXED|MAP_SHARED;
/// Unmap 用 MAP_FIXED|MAP_SHARED|MAP_ANONYMOUS 覆盖回退
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map / Unmap
pub const MMAP_PROT: &str = "PROT_READ | PROT_WRITE";
pub const MMAP_FLAGS_MAP: &str = "MAP_FIXED | MAP_SHARED";
pub const MMAP_FLAGS_UNMAP: &str = "MAP_FIXED | MAP_SHARED | MAP_ANONYMOUS";

/// Unmap 复查清单 (Map 时已通过的条件, 共 5 条)
/// v8: src/wasm/wasm-memory-map-descriptor.cc Unmap
#[rustfmt::skip]
pub static UNMAP_RECHECKS: &[&str] = &[
    "非 memory64",
    "非共享 backing store",
    "size 页对齐",
    "size + offset 无溢出",
    "size + offset 界内",
];

/// 映射后以弱引用持有 memory 对象, 实例可被回收
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
pub const HOLD_WEAK_MEMORY: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(MAP_REJECTS.len(), 6);
    assert_eq!(UNMAP_RECHECKS.len(), 5);
    assert_eq!(INVALID_FD, -1);
};
`;

const p = writeSrc("v8-wasm-objects", "wasm_memory_map_descriptor_cc.rs", rust);
console.log(`written ${p}`);
