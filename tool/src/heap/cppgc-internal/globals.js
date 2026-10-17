#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/globals.js — v8/src/heap/cppgc-internal/globals.h → v8-heap-cppgc/src/globals.rs.skel
// cppgc 全局常量/页掩码/AccessMode 枚举/指针宽度相关槽位大小
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/globals.h";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`globals.h 缺锚 ${label}: ${needle}`);
  n++;
};

// ── ① 类型别名 + 尺寸常量 (字面量逐个对锚) ──────────────────────────────
need("using Address = uint8_t*;", "Address");
need("using ConstAddress = const uint8_t*;", "ConstAddress");
const CONSTS = [
  ["kKB", "1024"],
  ["kMB", "kKB * 1024"],
  ["kGB", "kMB * 1024"],
  ["kPageSizeLog2", "17"],
  ["kPageSize", "1 << kPageSizeLog2"],
  ["kPageOffsetMask", "kPageSize - 1"],
  ["kPageBaseMask", "~kPageOffsetMask"],
  ["kLargeObjectSizeThreshold", "kPageSize / 2"],
  ["kFreeListEntrySize", "2 * sizeof(uintptr_t)"],
];
for (const [name] of CONSTS) need(`constexpr size_t ${name}`, name);
need("constexpr GCInfoIndex kFreeListGCInfoIndex = 0;", "kFreeListGCInfoIndex");
need("enum class AccessMode : uint8_t { kNonAtomic, kAtomic };", "AccessMode");

// ── ② 配置条件: 目标取 64 位 + 非指针压缩 (与 v8 参考构建一致) ──────────
need("constexpr size_t kAllocationGranularity = 8;", "granularity 64bit 分支");
need("constexpr size_t kAllocationGranularity = 4;", "granularity 32bit 分支(不取)");
need("constexpr size_t kAllocationMask = kAllocationGranularity - 1;", "kAllocationMask");
need("constexpr size_t kSlotSize = sizeof(uint32_t);", "kSlotSize 压缩分支(不取)");
need("constexpr size_t kSlotSize = sizeof(uintptr_t);", "kSlotSize 非压缩分支");
n += 4;

// Rust 侧落地: 64 位非压缩配置
const body = `${banner("src/heap/cppgc-internal/globals.js", REL, "cppgc 内部全局常量; 目标配置 V8_HOST_ARCH_64_BIT + !CPPGC_POINTER_COMPRESSION")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/globals.h Address
pub type Address = *mut u8;
/// v8: src/heap/cppgc-internal/globals.h ConstAddress
pub type ConstAddress = *const u8;

/// v8: src/heap/cppgc-internal/globals.h AccessMode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AccessMode {
    NonAtomic,
    Atomic,
}

pub mod constants {
    /// v8: src/heap/cppgc-internal/globals.h kKB
    pub const K_KB: usize = 1024;
    /// v8: src/heap/cppgc-internal/globals.h kMB
    pub const K_MB: usize = K_KB * 1024;
    /// v8: src/heap/cppgc-internal/globals.h kGB
    pub const K_GB: usize = K_MB * 1024;

    /// v8: src/heap/cppgc-internal/globals.h kAllocationGranularity (V8_HOST_ARCH_64_BIT)
    pub const K_ALLOCATION_GRANULARITY: usize = 8;
    /// v8: src/heap/cppgc-internal/globals.h kAllocationMask
    pub const K_ALLOCATION_MASK: usize = K_ALLOCATION_GRANULARITY - 1;

    /// v8: src/heap/cppgc-internal/globals.h kPageSizeLog2
    pub const K_PAGE_SIZE_LOG2: usize = 17;
    /// v8: src/heap/cppgc-internal/globals.h kPageSize
    pub const K_PAGE_SIZE: usize = 1 << K_PAGE_SIZE_LOG2;
    /// v8: src/heap/cppgc-internal/globals.h kPageOffsetMask
    pub const K_PAGE_OFFSET_MASK: usize = K_PAGE_SIZE - 1;
    /// v8: src/heap/cppgc-internal/globals.h kPageBaseMask
    pub const K_PAGE_BASE_MASK: usize = !K_PAGE_OFFSET_MASK;

    /// v8: src/heap/cppgc-internal/globals.h kLargeObjectSizeThreshold
    pub const K_LARGE_OBJECT_SIZE_THRESHOLD: usize = K_PAGE_SIZE / 2;

    /// v8: src/heap/cppgc-internal/globals.h kFreeListGCInfoIndex
    pub const K_FREE_LIST_GC_INFO_INDEX: u16 = 0;
    /// v8: src/heap/cppgc-internal/globals.h kFreeListEntrySize
    pub const K_FREE_LIST_ENTRY_SIZE: usize = 2 * core::mem::size_of::<usize>();

    /// v8: src/heap/cppgc-internal/globals.h kSlotSize (!CPPGC_POINTER_COMPRESSION)
    pub const K_SLOT_SIZE: usize = core::mem::size_of::<usize>();
}
`;

writeSrc("v8-heap-cppgc", "globals.rs.skel", body);
console.log(`globals.js: 对数 ${n} 项 (别名 2 + 枚举 1 + 常量 12 + 配置分支 4) → v8-heap-cppgc/src/globals.rs.skel`);
