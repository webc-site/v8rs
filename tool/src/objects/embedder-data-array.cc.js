#!/usr/bin/env bun
// tool/src/objects/embedder-data-array.cc.js — 覆盖 v8/src/objects/embedder-data-array.{cc,-inl.h}
// (差集: h/tq 归既有 embedder-data-array.h.js 席)
// → v8-objects-arrays/src/embedder_data_array_cc.rs.skel
// 机械提取: slots_start/slots_end/length -inl 语义 + EnsureCapacity cc 语义骨架。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/embedder-data-array.cc");
const inl = readV8("src/objects/embedder-data-array-inl.h");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── 1. -inl.h 语义锚 ───────────────────────────────────────────────────
must(inl.includes("int EmbedderDataArray::length() const { return length_.load().value(); }"),
  "length() 语义锚缺失");
must(inl.includes("void EmbedderDataArray::set_length(int value) {\n  length_.store(this, Smi::FromInt(value));"),
  "set_length 语义锚缺失");
must(inl.includes("Address EmbedderDataArray::slots_start() {\n  return reinterpret_cast<Address>(this) + SizeFor(0);"),
  "slots_start 语义锚缺失");
must(inl.includes("Address EmbedderDataArray::slots_end() {\n  return reinterpret_cast<Address>(this) + SizeFor(length());"),
  "slots_end 语义锚缺失");
const inlDefs = [...inl.matchAll(/^[\w:<>,& ]*EmbedderDataArray::(\w+)\(/gm)].map((m) => m[1]);
must(inlDefs.length === 4, `-inl 方法定义应 4, 实得 ${inlDefs.length}: ${JSON.stringify(inlDefs)}`);

// ── 2. cc 语义锚 (EnsureCapacity 双配置) ───────────────────────────────
const defs = [...cc.matchAll(/^[\w:<>,& ]*EmbedderDataArray::(\w+)\(/gm)].map((m) => m[1]);
must(JSON.stringify(defs) === JSON.stringify(["EnsureCapacity"]), `cc 方法定义应 EnsureCapacity, 实得 ${JSON.stringify(defs)}`);
must(cc.includes("if (index < array->length()) return array;"), "EnsureCapacity 提前返回锚缺失");
must(cc.includes("DCHECK_LT(index, kEmbedderDataArrayMaxLength);"), "上限 DCHECK 锚缺失");
must(cc.includes("isolate->factory()->NewEmbedderDataArray(index + 1)"), "扩容公式锚缺失");
// 压缩分支: 逐槽 LoadCppHeapPointerHandle / handle 复制 / tagged 无屏障复制
must(cc.includes("CppHeapPointerHandle LoadCppHeapPointerHandle(const EmbedderDataSlot& slot)"),
  "LoadCppHeapPointerHandle 锚缺失");
must(cc.includes("if (src_handle != kNullCppHeapPointerHandle) {"), "handle 非空分支锚缺失");
must(cc.includes("dest.store_tagged_without_barrier(src.load_tagged());"), "tagged 无屏障复制锚缺失");
// 非压缩分支: 整块 MemCopy
must(cc.includes("size_t size = array->length() * kEmbedderDataSlotSize;"), "MemCopy 尺寸公式锚缺失");
must(cc.includes("MemCopy(reinterpret_cast<void*>(new_array->slots_start()),"),
  "MemCopy 调用锚缺失");
must(cc.includes("reinterpret_cast<void*>(array->slots_start()), size);"), "MemCopy 源锚缺失");

// ── 3. 生成 ────────────────────────────────────────────────────────────
const body = `${banner("src/objects/embedder-data-array.cc.js", "src/objects/embedder-data-array.{cc,-inl.h}", "差集 (h/tq 归 embedder-data-array.h.js 席): -inl 存取语义 + EnsureCapacity 骨架")}
//! 差集骨架: EmbedderDataArray 行内语义与扩容。
//! 语义实现 (isolate/factory/slot 表) 依赖未落地符号, 留 Developer 就地修订。

// —— -inl 语义 (4 定义; Rust 无 inl, 并入实现模块) ——
/// v8: src/objects/embedder-data-array-inl.h length(): length_ Smi 解值。
pub fn embedder_data_array_inl_methods() -> &'static [&'static str] {
  &[
    "length",          // length_.load().value()
    "set_length",      // length_.store(Smi::FromInt(value))
    "slots_start",     // self 地址 + SizeFor(0)
    "slots_end",       // self 地址 + SizeFor(length())
  ]
}

// —— cc 语义: EnsureCapacity ——
/// v8: src/objects/embedder-data-array.cc EnsureCapacity —
///   index < length → 原样返回; 否则新数组 length = index+1:
///   - 指针压缩开: 逐槽复制 — CppHeapPointerHandle 非空走 handle 复制
///     (DuplicateEntry/store_handle_without_barrier), 空走 tagged 无屏障复制
///   - 非压缩: 整块 MemCopy (length * kEmbedderDataSlotSize 字节, 新空间分配免写屏障)
/// Rust 侧签名骨架:
/// pub fn ensure_capacity(
///   isolate: &Isolate, array: &EmbedderDataArray, index: usize,
/// ) -> Result<EmbedderDataArray> { todo!("就地修订") }
pub const ENSURE_CAPACITY_SEMANTICS: &[&str] = &[
  "提前返回: index < array.length()",
  "扩容目标长度: index + 1",
  "压缩配置: 逐槽复制 (handle 路径 / tagged 无屏障路径)",
  "非压缩配置: 整块 MemCopy size = length * K_EMBEDDER_DATA_SLOT_SIZE",
  "新空间尾分配 → 免写屏障",
];
`;

const p = writeSrc("v8-objects-arrays", "embedder_data_array_cc.rs.skel", body);
console.log(
  `embedder-data-array.cc.js: ${p} — -inl ${inlDefs.length} 定义、cc ${defs.length} 方法 + ${5} 条语义规则、断言 ${checks} 项通过`,
);
