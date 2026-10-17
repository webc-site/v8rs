#!/usr/bin/env bun
// tool/src/wasm/wasm-objects.cc.js (族合一: wasm-objects.cc + wasm-objects-inl.h)
// 大 .cc 规则面: 32 位内存分配启发式、WasmMemoryObject::Grow 状态机、
// dispatch table uses 列表布局、全局缓冲最小偏移、异常值数组元素宽、
// -inl.h 访问器宏展开计数与 SBXCHECK 防线; 另出方法签名骨架。
// torque 类布局常量在 wasm_objects_consts.rs (wasm-objects.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/wasm/wasm-objects.cc");
const inl = readV8("src/wasm/wasm-objects-inl.h");

// ---------- 1. dispatch table uses 列表保留槽 ----------
const reserved = cc.match(/static constexpr int kReservedSlotOffset = (\d+);/);
if (!reserved || reserved[1] !== "1") throw new Error("kReservedSlotOffset != 1");

// ---------- 2. 32 位内存分配启发式 ----------
if (!cc.includes("constexpr int kGBPages = 1024 * 1024 * 1024 / wasm::kWasmPageSize;")) {
  throw new Error("kGBPages 公式缺失");
}
// 启发式分支序: 初始超 1GB -> initial; 有最大 -> min(maximum, alloc_max); 共享 -> alloc_max; 否则 initial
const heur = [];
if (cc.includes("if (initial > kGBPages) {")) heur.push("initial>1GB->initial");
if (/heuristic_maximum = std::min\(maximum, allocation_maximum\);/.test(cc)) heur.push("hasMax->min(max,allocMax)");
if (cc.includes("// If shared memory has no maximum, we use the allocation_maximum as an")) heur.push("shared->allocMax");
if (/heuristic_maximum = initial;/.test(cc)) heur.push("noMax->initial");
if (heur.length !== 4) throw new Error(`启发式分支 ${heur.length} != 4`);

// ---------- 3. Grow 状态机 ----------
// 3a. 最大页钳制
if (!cc.includes("size_t max_pages = memory_object->is_memory64() ? wasm::max_mem64_pages()")) {
  throw new Error("max_pages 双轨缺失");
}
// 3b. 超限返回 -1
if (!cc.includes("if (pages > max_pages - old_pages) return -1;")) {
  throw new Error("Grow 超限 -1 缺失");
}
// 3c. must_grow_in_place 四条件
const mustGrow = cc.match(
  /const bool must_grow_in_place =\s*backing_store->is_shared\(\) \|\| backing_store->has_guard_regions\(\) \|\|\s*backing_store->is_resizable_by_js\(\) \|\| pages == 0;/,
);
if (!mustGrow) throw new Error("must_grow_in_place 四条件缺失");
// 3d. API 中断禁止非共享增长
if (!cc.includes("isolate->is_executing_api_interrupt() && !backing_store->is_shared() &&")) {
  throw new Error("API 中断禁止增长缺失");
}
// 3e. min_growth 公式 (0.5MB + 12.5%: 8 页 + old/8)
if (!cc.includes("size_t min_growth = old_pages + 8 + (old_pages >> 3);")) {
  throw new Error("min_growth 公式缺失");
}
// 3f. new_capacity: 先 min_growth 后 max_pages
if (!cc.includes("size_t new_capacity = std::min(max_pages, std::max(new_pages, min_growth));")) {
  throw new Error("new_capacity 公式缺失");
}
// 3g. bit_ceil 报告大小 (避免增长循环触发过多 GC)
if (!cc.includes("std::bit_ceil(new_byte_length)")) throw new Error("bit_ceil 报告缺失");
// 3h. 成功返回 old_pages
if (!cc.includes("return static_cast<int32_t>(old_pages);  // success")) {
  throw new Error("成功返回 old_pages 缺失");
}
// 3i. -1 路径: 内存 Grow 4 处 (超限/API 中断/in-place 失败/OOM 拷贝), 表 Grow 1 处 (超限)
const failReturns = [...cc.matchAll(/return -1;/g)];
if (failReturns.length !== 5) throw new Error(`-1 路径 ${failReturns.length} != 5`);

// ---------- 4. 异常值数组元素宽 ----------
const excElem = cc.match(/constexpr uint32_t kBytesPerExceptionValuesArrayElement = (\d+);/);
if (!excElem || excElem[1] !== "2") throw new Error("异常值数组元素宽 != 2");

// ---------- 5. dispatch table uses 初始容量 ----------
const initLen = cc.match(/constexpr uint32_t kInitialLength = (\d+);\s*\/\/ 1 slot \+ 1 pair\./);
if (!initLen || initLen[1] !== "3") throw new Error("uses 初始容量 != 3");
// AddUse: 每条用记 (instance, table_index) 对, used_length += 2
if (!cc.includes("SetUsedLength(uses, cursor + 2);")) throw new Error("AddUse 步进 2 缺失");

// ---------- 6. 全局缓冲最小偏移公式 ----------
if (!cc.includes("static constexpr int kMinRefOffsetFromTaggedBuffer =\n      FixedArray::OffsetOfElementAt(0) - kHeapObjectTag;")) {
  throw new Error("kMinRefOffset 公式缺失");
}
if (!cc.includes("static constexpr int kMinNumOffsetFromTaggedBuffer =\n      ByteArray::OffsetOfElementAt(0) - kHeapObjectTag;")) {
  throw new Error("kMinNumOffset 公式缺失");
}

// ---------- 7. 表增长上限防线 ----------
if (!cc.includes("SBXCHECK_LE(new_length, wasm::max_table_size());")) {
  throw new Error("表增长上限 SBXCHECK 缺失");
}

// ---------- 8. 方法定义清单 (骨架对数基准) ----------
const methods = [...cc.matchAll(/^[A-Za-z][A-Za-z0-9_:<>, &*]* (Wasm[A-Za-z0-9_]+)::([A-Za-z0-9_]+)\(/gm)].map((m) => `${m[1]}::${m[2]}`);
if (methods.length !== 77) throw new Error(`方法定义 ${methods.length} != 77`);
const classes = [...new Set(methods.map((m) => m.split("::")[0]))];

// ---------- 9. -inl.h: 访问器宏与防线 ----------
// 本地宏调用计数 (PRIMITIVE / WTI_TAGGED / WTI_OPTIONAL_TAGGED / DEF_GETTER)
const primAccessors = (inl.match(/^PRIMITIVE_ACCESSORS\(/gm) || []).length;
const wtiTagged = (inl.match(/^WTI_TAGGED_ACCESSORS\(/gm) || []).length;
const wtiOptional = (inl.match(/^WTI_OPTIONAL_TAGGED_ACCESSORS\(/gm) || []).length;
const defGetters = (inl.match(/^\s*DEF_GETTER\(/gm) || []).length;
if (primAccessors !== 6) throw new Error(`PRIMITIVE_ACCESSORS ${primAccessors} != 6`);
if (wtiTagged !== 12) throw new Error(`WTI_TAGGED_ACCESSORS ${wtiTagged} != 12`);
if (wtiOptional !== 2) throw new Error(`WTI_OPTIONAL_TAGGED_ACCESSORS ${wtiOptional} != 2`);
if (defGetters !== 1) throw new Error(`DEF_GETTER ${defGetters} != 1`);
// 本地宏定义: OPTIONAL_ACCESSORS (has_ + 取值) 与 PRIMITIVE_ACCESSORS (未对齐读写)
if (!inl.includes("#define OPTIONAL_ACCESSORS(holder, name, type, offset)")) {
  throw new Error("OPTIONAL_ACCESSORS 本地宏定义缺失");
}
if (!inl.includes("#define PRIMITIVE_ACCESSORS(holder, name, type, offset)")) {
  throw new Error("PRIMITIVE_ACCESSORS 本地宏定义缺失");
}
if (!inl.includes("ReadMaybeUnalignedValue<type>(FIELD_ADDR(this, offset));")) {
  throw new Error("未对齐读语义缺失");
}
const accessorTotal = primAccessors + wtiTagged + wtiOptional + defGetters;
if (!inl.includes("SBXCHECK_EQ(bases_and_sizes_length % 2u, 0u);")) {
  throw new Error("bases_and_sizes 偶数防线缺失");
}
if (!inl.includes("static_assert(sizeof(WasmCodePointer) == sizeof(uint32_t));")) {
  throw new Error("WasmCodePointer 宽度断言缺失");
}

console.log(`wasm-objects: 启发式分支=${heur.length} -1路径=${failReturns.length} 方法=${methods.length} 类=${classes.length} 访问器宏=${accessorTotal}`);

// ---------- 生成 Rust ----------
let rust = `${banner(
  "src/wasm/wasm-objects.cc.js",
  "src/wasm/wasm-objects.{cc,-inl.h}",
  "Wasm 对象行为规则: 内存分配启发式/Grow 状态机/uses 列表布局 + 方法签名骨架",
)}
//! Wasm 堆对象行为规则面 (对象布局常量在 wasm_objects_consts.rs)。

/// dispatch table uses 列表: 首槽保留, 其后 (instance, table_index) 对
/// v8: src/wasm/wasm-objects.cc kReservedSlotOffset
pub const USES_RESERVED_SLOT_OFFSET: u32 = 1;
/// uses 列表初始容量: 1 保留槽 + 1 对
/// v8: src/wasm/wasm-objects.cc WasmDispatchTable::MaybeGrowUsesList
pub const USES_INITIAL_LENGTH: u32 = 3;
/// AddUse 每条记录步进 2 (instance + table_index)
/// v8: src/wasm/wasm-objects.cc WasmDispatchTable::AddUse
pub const USES_ENTRY_STEP: u32 = 2;

/// 32 位平台内存预留启发式: 1GB 页数上限 (1024^3 / kWasmPageSize)
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::New
pub const K_GB_PAGES_FORMULA: &str = "1024 * 1024 * 1024 / kWasmPageSize";
/// 启发式分支序: 初始>1GB 用初始; 有最大取 min(maximum, allocMax); 共享无最大用 allocMax; 非共享无最大用初始
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::New (V8_TARGET_ARCH_32_BIT)
#[rustfmt::skip]
pub static MEMORY_RESERVE_HEURISTIC: &[&str] = &[
    "initial>1GB -> initial",
    "hasMaximum -> min(maximum, allocMax)",
    "shared&&!maximum -> allocMax",
    "!shared&&!maximum -> initial",
];

/// 必须原地增长的四条件
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow
#[rustfmt::skip]
pub static MUST_GROW_IN_PLACE: &[&str] = &[
    "backing_store.is_shared()",
    "backing_store.has_guard_regions()",
    "backing_store.is_resizable_by_js()",
    "pages == 0",
];

/// 拷贝增长最小步进: old_pages + 8 + old_pages/8 (约 0.5MB + 12.5%)
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow
pub fn min_growth_pages(old_pages: usize) -> usize {
    old_pages + 8 + (old_pages >> 3)
}

/// 新容量: min(max_pages, max(new_pages, min_growth)) — 先下限后上限
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow
pub fn new_capacity_pages(max_pages: usize, new_pages: usize, old_pages: usize) -> usize {
    max_pages.min(new_pages.max(min_growth_pages(old_pages)))
}

/// Grow 失败 (-1) 路径: 内存 4 条 + 表超限 1 条
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow / WasmTableObject::Grow
pub const GROW_FAIL_PATHS: usize = 4;
pub const TABLE_GROW_FAIL_PATHS: usize = 1;

/// 成功返回旧页数 (共享内存返回同步后的原值)
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow
pub const GROW_SUCCESS_RETURNS: &str = "old_pages";
/// 托管对象报告大小取 bit_ceil, 避免循环增长触发过多 GC
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow
pub const GROW_REPORT_ROUNDING: &str = "bit_ceil(new_byte_length)";

/// 异常值数组元素宽度 (uint16)
/// v8: src/wasm/wasm-objects.cc kBytesPerExceptionValuesArrayElement
pub const K_BYTES_PER_EXCEPTION_VALUES_ARRAY_ELEMENT: u32 = ${excElem[1]};

/// 全局引用值最小偏移: FixedArray::OffsetOfElementAt(0) - kHeapObjectTag
/// v8: src/wasm/wasm-objects.cc WasmGlobalObject::New
pub const K_MIN_REF_OFFSET_FROM_TAGGED_BUFFER: &str = "FixedArray::OffsetOfElementAt(0) - kHeapObjectTag";
/// 全局数值最小偏移: ByteArray::OffsetOfElementAt(0) - kHeapObjectTag
/// v8: src/wasm/wasm-objects.cc WasmGlobalObject::New
pub const K_MIN_NUM_OFFSET_FROM_TAGGED_BUFFER: &str = "ByteArray::OffsetOfElementAt(0) - kHeapObjectTag";

/// -inl.h 侧防线: bases_and_sizes 长度必须为偶数, memory_index < len/2
/// v8: src/wasm/wasm-objects-inl.h WasmTrustedInstanceData::memory_base/memory_size
pub const BASES_AND_SIZES_LAYOUT: &str = "偶数长 (base,size) 对; memory_index < len/2";
/// WasmCodePointer 与 u32 同宽
/// v8: src/wasm/wasm-objects-inl.h static_assert(sizeof(WasmCodePointer) == sizeof(uint32_t))
pub const CODE_POINTER_WIDTH: usize = 4;
/// -inl.h 访问器宏展开数 (DEF_GETTER/ACCESSORS/TRUSTED_POINTER_ACCESSORS/OPTIONAL_ACCESSORS)
pub const INL_ACCESSOR_MACROS: usize = ${accessorTotal};

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(USES_RESERVED_SLOT_OFFSET, 1);
    assert_eq!(USES_INITIAL_LENGTH, 3);
    assert_eq!(USES_ENTRY_STEP, 2);
    assert_eq!(MEMORY_RESERVE_HEURISTIC.len(), 4);
    assert_eq!(MUST_GROW_IN_PLACE.len(), 4);
    assert_eq!(min_growth_pages(0), 8);
    assert_eq!(min_growth_pages(64), 80);
    assert_eq!(new_capacity_pages(100, 20, 64), 100);
    assert_eq!(new_capacity_pages(200, 20, 64), 80);
    assert_eq!(GROW_FAIL_PATHS, 4);
    assert_eq!(K_BYTES_PER_EXCEPTION_VALUES_ARRAY_ELEMENT, 2);
    assert_eq!(CODE_POINTER_WIDTH, 4);
    assert_eq!(INL_ACCESSOR_MACROS, 21);
};

// —— 方法签名骨架 (就地修订 todo) ——
`;

for (const cls of classes) {
  rust += `
/// ${cls} 方法骨架
`;
  for (const m of methods.filter((x) => x.startsWith(`${cls}::`))) {
    const name = m.split("::")[1];
    rust += `/// v8: src/wasm/wasm-objects.cc ${m}\npub fn ${cls.replace(/^Wasm/, "").toLowerCase()}_${name.toLowerCase()}() {\n    todo!("就地修订: ${m}")\n}\n`;
  }
}

const p = writeSrc("v8-wasm-objects", "wasm_objects_cc.rs", rust);
console.log(`written ${p}`);
