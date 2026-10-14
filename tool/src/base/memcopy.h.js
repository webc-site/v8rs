#!/usr/bin/env bun
// tool/src/base/memcopy.h.js — 族: src/base/memcopy.h
// 覆盖: MemCopy (NEON/标量两路) + MemMove + MemCopyAndSwitchEndianness (大端)
//        + ReplicateValueWord + Relaxed_Memset
// → v8-base/src/memcopy.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/memcopy.h");

// —— 锚点断言 ——
if (!src.includes("#define V8_OPTIMIZE_WITH_NEON")) throw new Error("缺 NEON 门");
if (!src.includes("V8_CLANG_NO_SANITIZE(\"alignment\") void OverlappingWrites(")) throw new Error("缺重叠写模板");
if (!src.includes("inline void SimdMemCopy(void* dst, const void* src, size_t count)")) throw new Error("缺 SimdMemCopy");
if (!src.includes("bits::CountLeadingZeros(count - 1)")) throw new Error("缺 CLZ 阶梯分派");
if (!src.includes("V8_INLINE void MemMove(void* dest, const void* src, size_t size)")) throw new Error("缺 MemMove");
if (!src.includes("constexpr uintptr_t ReplicateValueWord(T value)")) throw new Error("缺值复制模板");
if (!src.includes("V8_INLINE void Relaxed_Memset(T* destination, T value, size_t count)")) throw new Error("缺 Relaxed_Memset");

// —— 真解析: CASE X-macro 展开 (MemCopy 16 + MemMove 16) 与端字交换分派 ——
const memCopyCases = [...src.matchAll(/^      CASE\((\d+)\)/gm)].map((m) => Number(m[1]));
if (memCopyCases.length !== 32) throw new Error(`CASE 展开 ${memCopyCases.length} != 32`);
const byteReverse = [...src.matchAll(/COPY_LOOP\(u?int(\d+)_t, bits::ByteReverse(\d+)\)/g)].map((m) => m[2]);
if (byteReverse.join(",") !== "16,32,64") throw new Error(`ByteReverse 分派 ${byteReverse}`);
if (!src.includes("case 16:")) throw new Error("缺 16 字节逐字节反转分支");
console.log(`memcopy: MemCopy 2 路 (NEON/标量), CASE 快路径 ${memCopyCases.length}, 端字交换 4 档, Relaxed_Memset 1`);

let rust = banner(
  "src/base/memcopy.h.js",
  "src/base/memcopy.h",
  "memcpy 族平台特化 (NEON SIMD / 定长快路径 / relaxed memset)",
);

rust += `
//! v8: src/base/memcopy.h

/// v8: src/base/memcopy.h MemCopy(dest, src, size)
/// DEBUG 下 DCHECK 区间不重叠; 两实现路:
///   NEON (arm64+__ARM_NEON): SimdMemCopy — CLZ(order) 阶梯:
///     1→u16 直拷, 2→u16 重叠写, 3→u32, 4→u64, 5→双 vld1q/vst1q 首尾, 33+→首块+步进主循环
///   标量: size 1..=16 定长 memcpy 内联展开 (X-macro CASE), 其余走通用 memcpy
/// Rust: ptr::copy_nonoverlapping + Developer 按 target_arch 补 SIMD 分支
pub fn mem_copy(dest: *mut u8, src: *const u8, size: usize);

/// v8: src/base/memcopy.h MemMove — 同 MemMove 语义, 1..=16 定长 + 通用 memmove
/// Rust: ptr::copy
pub fn mem_move(dest: *mut u8, src: *const u8, size: usize);

/// v8: src/base/memcopy.h MemCopyAndSwitchEndianness (仅 V8_TARGET_BIG_ENDIAN)
/// element_size ∈ {1(直拷), 2,4,8(ByteReverse), 16(逐字节 s[15-j])}, 其余 UNREACHABLE
pub fn mem_copy_and_switch_endianness(dst: *mut u8, src: *const u8, num_elements: usize, element_size: usize);

/// v8: src/base/memcopy.h ReplicateValueWord<T>
/// 把 value 位型复制到 uintptr 全字: kMultiplier = usize::MAX / T::MAX;
/// sizeof(T) >= word 直接 cast
pub const fn replicate_value_word<T: Integral>(value: T) -> usize;

/// v8: src/base/memcopy.h Relaxed_Memset<T>
/// 三段式: 对齐前导逐元素 relaxed store → 字级 __atomic_store_n (may_alias 别名字)
///         → 尾部逐元素; word_val = ReplicateValueWord(value)
pub fn relaxed_memset<T: Integral>(destination: *mut T, value: T, count: usize);
`;

writeSrc("v8-base", "memcopy.rs.skel", rust);
console.log("written v8-base/src/memcopy.rs.skel");
