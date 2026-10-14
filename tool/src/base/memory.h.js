#!/usr/bin/env bun
// tool/src/base/memory.h.js — 族: src/base/memory.h
// 覆盖: Memory<T> 双重载 + ReadUnalignedValue 4 重载 + WriteUnalignedValue 2
//        + Read/WriteLittleEndianValue 各 2 重载
// → v8-base/src/memory.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/memory.h");

// —— 锚点断言 ——
if (!src.includes("using Address = uintptr_t;")) throw new Error("缺 Address 别名");
if (!src.includes("inline T& Memory(Address addr) {")) throw new Error("缺 Memory(Address)");
if (!src.includes("DCHECK(IsAligned(addr, alignof(T)));")) throw new Error("缺对齐检查");
if (!src.includes("static inline V ReadUnalignedValue(Address p) {")) throw new Error("缺非对齐读");
if (!src.includes("static inline void WriteUnalignedValue(Address p, V value) {")) throw new Error("缺非对齐写");

// —— 真解析: 重载计数 ——
const readUnaligned = (src.match(/^static inline V ReadUnalignedValue\(/gm) || []).length;
const writeUnaligned = (src.match(/^static inline void WriteUnalignedValue\(/gm) || []).length;
const readLE = (src.match(/^static inline V ReadLittleEndianValue\(/gm) || []).length;
const writeLE = (src.match(/^static inline void WriteLittleEndianValue\(/gm) || []).length;
if (readUnaligned !== 4) throw new Error(`ReadUnalignedValue ${readUnaligned} != 4`);
if (writeUnaligned !== 2) throw new Error(`WriteUnalignedValue ${writeUnaligned} != 2`);
if (readLE !== 2) throw new Error(`ReadLittleEndianValue ${readLE} != 2`);
if (writeLE !== 2) throw new Error(`WriteLittleEndianValue ${writeLE} != 2`);
if (!src.includes("#if !defined(V8_OS_SOLARIS)")) throw new Error("缺 Solaris 条件编译");
if (!src.includes("dst[i] = src[sizeof(V) - i - 1];")) throw new Error("缺大端逐字节反转");
console.log(`memory: Memory 2, ReadUnaligned ${readUnaligned}, WriteUnaligned ${writeUnaligned}, LE 读写 ${readLE}+${writeLE}`);

let rust = banner(
  "src/base/memory.h.js",
  "src/base/memory.h",
  "裸内存访问 (对齐引用 + 非对齐/小端读写)",
);

rust += `
//! v8: src/base/memory.h

/// v8: src/base/memory.h Address = uintptr_t
pub type Address = usize;

/// v8: src/base/memory.h Memory<T>(Address) — reinterpret 引用, DCHECK 对齐 alignof(T)
/// v8: Memory<T>(uint8_t*) — 指针版转 Address 再走上者
/// Rust: &*(addr as *const T), 由 Developer 包 unsafe + debug_assert 对齐
pub fn memory_ref<T>(addr: Address) -> &T;

/// v8: src/base/memory.h ReadUnalignedValue<V>(Address)
/// memcpy 到栈副本返回 (ASSERT_TRIVIALLY_COPYABLE); char[]/int8_t[](非 Solaris)/uint8_t[] 转发重载共 4
/// Rust: ptr::read_unaligned
pub unsafe fn read_unaligned_value<V: Copy>(p: Address) -> V;

/// v8: src/base/memory.h WriteUnalignedValue<V>(Address, value) — memcpy 写; char[] 转发共 2
pub unsafe fn write_unaligned_value<V: Copy>(p: Address, value: V);

/// v8: src/base/memory.h ReadLittleEndianValue — 小端目标直读; 大端逐字节反序
/// v8: WriteLittleEndianValue — 同上写侧; V* 重载带禁数组 static_assert
/// Rust: 目标恒小端时退化为非对齐读写, cfg(target_endian = "big") 分支 Developer 补
pub unsafe fn read_little_endian_value<V: Copy>(p: Address) -> V;
pub unsafe fn write_little_endian_value<V: Copy>(p: Address, value: V);
`;

writeSrc("v8-base", "memory.rs.skel", rust);
console.log("written v8-base/src/memory.rs.skel");
