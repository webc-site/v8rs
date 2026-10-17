#!/usr/bin/env bun
// tool/src/wasm/wasm-export-wrapper-cache.h.js
// 机械提取 v8/src/wasm/wasm-export-wrapper-cache.{h,cc} 的开地址哈希布局参数

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-export-wrapper-cache.h");
const ccSrc = readV8("src/wasm/wasm-export-wrapper-cache.cc");

// 1. h: 布局常量
const consts = [
  ["kUnused", "static constexpr int kUnused = -1;"],
  ["kNumUsedElementsIndex", "static constexpr int kNumUsedElementsIndex = 0;"],
  ["kReservedSlots", "static constexpr int kReservedSlots = 1;"],
  ["kSlotsPerEntry", "static constexpr int kSlotsPerEntry = 2;"],
];
for (const [name, lit] of consts) {
  if (!hSrc.includes(lit)) throw new Error(`布局常量缺失: ${name}`);
}

// 2. h: hash 即身份
if (!hSrc.includes("return sig_index.index;")) throw new Error("未找到 Hash=sig_index");

// 3. h: 探测序列
if (!hSrc.includes("return InternalIndex(hash & (capacity - 1));")) {
  throw new Error("未找到 FirstProbe 位掩码");
}
const nextMatch = hSrc.match(/return InternalIndex\(\(last\.as_uint32\(\) \+ number\) & \(capacity - 1\)\);/);
if (!nextMatch) throw new Error("未找到 NextProbe 线性步进");

// 4. h: 槽位映射
if (!hSrc.includes("return kReservedSlots + entry.as_int() * kSlotsPerEntry;")) {
  throw new Error("未找到 ToArraySlot");
}

// 5. cc: 容量必须 2 的幂 + 总长换算
if (!ccSrc.includes("DCHECK(base::bits::IsPowerOfTwo(capacity));")) {
  throw new Error("未找到容量 2 幂断言");
}
if (!ccSrc.includes("capacity = capacity * kSlotsPerEntry + kReservedSlots;")) {
  throw new Error("未找到总长换算");
}

// 6. cc: 初始容量与扩容策略
const initMatch = ccSrc.match(/static constexpr uint32_t kInitialCapacity = (\d+);/);
if (!initMatch) throw new Error("未找到初始容量");
const initialCapacity = parseInt(initMatch[1], 10);
if (!ccSrc.includes("if (used < capacity / 2) return cache;")) {
  throw new Error("未找到 50% 负载阈值");
}
if (!ccSrc.includes("Tagged<WeakFixedArray> grown = *New(isolate, capacity * 2);")) {
  throw new Error("未找到翻倍扩容");
}

// 7. cc: Capacity 反算
if (!ccSrc.includes("return cache->ulength().value() >> 1;")) {
  throw new Error("未找到 Capacity >>1");
}

// 8. cc: 查找命中/未命中路径
if (!ccSrc.includes("if (key == kUnused) return {};")) throw new Error("未找到 kUnused 空槽返回");
if (!ccSrc.includes("if (value.IsCleared()) return {};")) throw new Error("未找到清弱引用返回");

console.log(
  `wasm-export-wrapper-cache: 布局常量=${consts.length} 初始容量=${initialCapacity}`,
);

// 9. 生成 Rust
let rust = banner(
  "src/wasm/wasm-export-wrapper-cache.h.js",
  "src/wasm/wasm-export-wrapper-cache.{h,cc}",
  "JS-to-Wasm wrapper 的 WeakFixedArray 开地址缓存",
);

rust += `
//! Isolate 级 JS-to-Wasm wrapper 缓存: WeakFixedArray 开地址表, key=Smi(canonical index)。
/// v8: src/wasm/wasm-export-wrapper-cache.h WasmExportWrapperCache

/// 布局参数
/// v8: src/wasm/wasm-export-wrapper-cache.h 私有常量
pub mod layout {
    /// v8: kUnused = -1 (空槽哨兵, Smi)
    pub const UNUSED: i32 = -1;
    /// v8: kNumUsedElementsIndex = 0 (slot0 存已用条目数)
    pub const NUM_USED_ELEMENTS_INDEX: usize = 0;
    /// v8: kReservedSlots = 1
    pub const RESERVED_SLOTS: usize = 1;
    /// v8: kSlotsPerEntry = 2 (key Smi + weak value)
    pub const SLOTS_PER_ENTRY: usize = 2;
}

/// 初始桶数
/// v8: src/wasm/wasm-export-wrapper-cache.cc EnsureCapacity kInitialCapacity
pub const INITIAL_CAPACITY: u32 = ${initialCapacity};

/// 负载上限: used < capacity/2 即满足, 否则翻倍
/// v8: src/wasm/wasm-export-wrapper-cache.cc EnsureCapacity
pub const LOAD_FACTOR_NUM: u32 = 1;
pub const LOAD_FACTOR_DEN: u32 = 2;

/// hash = canonical type index 本身 (身份即键)
/// v8: src/wasm/wasm-export-wrapper-cache.h Hash
pub fn hash(sig_index: u32) -> u32 {
    sig_index
}

/// 首探: 位掩码取模 (容量恒 2 的幂)
/// v8: src/wasm/wasm-export-wrapper-cache.h FirstProbe
pub fn first_probe(hash: u32, capacity: u32) -> u32 {
    hash & (capacity - 1)
}

/// 后续探测: (last + n) & (capacity-1), n 从 1 递增
/// v8: src/wasm/wasm-export-wrapper-cache.h NextProbe
pub fn next_probe(last: u32, number: u32, capacity: u32) -> u32 {
    (last.wrapping_add(number)) & (capacity - 1)
}

/// 桶号 → WeakFixedArray 槽位: reserved + entry*2
/// v8: src/wasm/wasm-export-wrapper-cache.h ToArraySlot
pub fn to_array_slot(entry: usize) -> usize {
    layout::RESERVED_SLOTS + entry * layout::SLOTS_PER_ENTRY
}

/// 数组总长 = capacity*2 + 1; 反向 Capacity = len >> 1
/// v8: src/wasm/wasm-export-wrapper-cache.cc New / Capacity
pub fn array_length_for(capacity: u32) -> u32 {
    capacity * layout::SLOTS_PER_ENTRY as u32 + layout::RESERVED_SLOTS as u32
}
pub fn capacity_of(array_length: u32) -> u32 {
    array_length >> 1
}

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(layout::UNUSED, -1);
    assert_eq!(INITIAL_CAPACITY, 16);
    assert_eq!(array_length_for(16), 33);
    assert_eq!(capacity_of(33), 16);
    assert_eq!(to_array_slot(0), 1);
    assert_eq!(to_array_slot(5), 11);
    assert_eq!(first_probe(0x1234, 16), 0x4);
    assert_eq!(next_probe(15, 1, 16), 0);
    assert!(INITIAL_CAPACITY.is_power_of_two());
};
`;

const p = writeSrc("v8-wasm-core", "wasm_export_wrapper_cache.rs.skel", rust);
console.log(`written ${p}`);
