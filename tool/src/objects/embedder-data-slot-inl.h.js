#!/usr/bin/env bun
// tool/src/objects/embedder-data-slot-inl.h.js — 覆盖 v8/src/objects/embedder-data-slot-inl.h
// (差集: h 归既有 embedder-data-slot.h.js 席)
// → v8-objects-arrays/src/embedder_data_slot_inl.rs.skel
// 机械提取: 槽构造/读写/指针存取/序列化判定方法清单 + 配置分支规则面。

import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/embedder-data-slot-inl.h");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── 1. 方法定义清单 (含 static 与模板重载) ─────────────────────────────
const defs = [...inl.matchAll(/^(?:\/\/ static\n)?(?:template <[\s\S]*?>\n)?[\w:<>,*& ]*EmbedderDataSlot::(\w+)\(/gm)]
  .map((m) => m[1]);
const unique = [...new Set(defs)];
must(unique.length === 15, `方法名应 15 个, 实得 ${unique.length}: ${JSON.stringify(unique)}`);
for (const need of ["EmbedderDataSlot", "Initialize", "load_tagged", "store_smi", "store_tagged",
  "clear_cpp_heap_pointer_field", "ToAlignedPointer", "ToGenericAlignedPointer", "DeprecatedToAlignedPointer",
  "store_aligned_pointer", "store_tagged_without_barrier", "store_handle_without_barrier", "load_raw",
  "store_raw", "MustClearDuringSerialization"])
  must(unique.includes(need), `-inl 缺方法 ${need}`);
// 构造函数双形态
must(inl.includes("EmbedderDataSlot::EmbedderDataSlot(Tagged<EmbedderDataArray> array,"),
  "数组形态构造锚缺失");
must(inl.includes("EmbedderDataSlot::EmbedderDataSlot(Tagged<JSObject> object,"),
  "JSObject 形态构造锚缺失");
// ToAlignedPointer 双重载 (CppHeapPointerTagRange / ExternalPointerTagRange)
const toAlignedCount = (inl.match(/EmbedderDataSlot::ToAlignedPointer\(/g) ?? []).length;
must(toAlignedCount === 2, `ToAlignedPointer 应 2 重载, 实得 ${toAlignedCount}`);
// store_aligned_pointer 双形态 (HeapObject host / DirectHandle host 模板)
const storeAlignedCount = (inl.match(/EmbedderDataSlot::store_aligned_pointer\(/g) ?? []).length;
must(storeAlignedCount === 2, `store_aligned_pointer 应 2 形态, 实得 ${storeAlignedCount}`);

// ── 2. 配置分支锚 (V8_COMPRESS_POINTERS) ───────────────────────────────
const compressBlocks = (inl.match(/#ifdef V8_COMPRESS_POINTERS/g) ?? []).length;
must(compressBlocks >= 8, `V8_COMPRESS_POINTERS 分支应 ≥8 处, 实得 ${compressBlocks}`);
must(inl.includes("clear_cpp_heap_pointer_field(Address slot_address)"), "槽清理函数锚缺失");
must(inl.includes("slot_address + kCppHeapPointerOffset"), "kCppHeapPointerOffset 偏移锚缺失");
must(inl.includes("ObjectSlot(address() + kTaggedPayloadOffset)"), "kTaggedPayloadOffset 锚缺失");
// store_raw 双 32 位存储语义 (压缩下两段写)
must(inl.includes("static_assert(kSmiShiftSize == 0);") &&
  inl.includes("static_assert(SmiValuesAre31Bits());") &&
  inl.includes("static_assert(kTaggedSize == kInt32Size);"),
  "store_raw 压缩前提 static_assert 三连锚缺失");
must(inl.includes("// We have to do two 32-bit stores here because"), "双 32 位写原因注释锚缺失");
must(inl.includes("AsAtomicTagged::Relaxed_Store("), "hi 段原子写锚缺失");
// ToAlignedPointer 压缩: handle → CppHeapPointerTable 查表
must(inl.includes("isolate.GetCppHeapPointerTable().Get(handle, tag_range);"), "指针表查表锚缺失");
must(inl.includes("if (handle == kNullCppHeapPointerHandle) {"), "空 handle 锚缺失");
// store_aligned_pointer 压缩: table.AllocateAndInitializeEntry / Set / DuplicateEntry
must(inl.includes("table.AllocateAndInitializeEntry(space, value, tag);"), "表项分配锚缺失");
must(inl.includes("table.DuplicateEntry(space, handle);"), "表项复制锚缺失");
must(inl.includes("WriteBarrier::ForCppHeapPointer("), "CppHeap 写屏障锚缺失");
// store_tagged 压缩: cage base 同域 CHECK
must(inl.includes("GetPtrComprCageBaseAddress(value.ptr()) =="), "cage base 同域检查锚缺失");
// MustClearDuringSerialization: 序列化须避免外指针 handle
must(inl.includes("Serialization must avoid writing external pointer handles"), "序列化注释锚缺失");

// ── 3. 生成 ────────────────────────────────────────────────────────────
const body = `${banner("src/objects/embedder-data-slot-inl.h.js", "src/objects/embedder-data-slot-inl.h", "差集 (h 归 embedder-data-slot.h.js 席): 槽方法清单 + 配置分支规则")}
//! 差集骨架: EmbedderDataSlot 行内读写语义。
//! 依赖 (CppHeapPointerTable/WriteBarrier/cage base) 未落地, 语义留 Developer 就地修订。
//! 配置: 脚本登记原版双配置分支; v8-objects-arrays 配置登记为 64 位非压缩。

/// -inl 方法清单 (${unique.length} 方法名, 构造 2 形态 / ToAlignedPointer 2 重载 / store_aligned_pointer 2 形态)。
pub const EMBEDDER_DATA_SLOT_INL_METHODS: &[&str] = &[
${unique.map((d) => `  "${d}",`).join("\n")}
];

/// 配置分支规则面 (每条 = 压缩分支 / 非压缩分支)。
pub const EMBEDDER_DATA_SLOT_CONFIG_RULES: &[(&str, &str)] = &[
  ("clear_cpp_heap_pointer_field: CppHeapPointerSlot Relaxed_StoreHandle(空 handle)",
   "clear_cpp_heap_pointer_field: WriteUnalignedValue(槽+kCppHeapPointerOffset, kNullAddress)"),
  ("ToAlignedPointer: handle 空或表查 kNullAddress → nullptr; 否则表查返回指针",
   "ToAlignedPointer: ReadUnalignedValue<Address>(槽+kCppHeapPointerOffset) 直读"),
  ("store_aligned_pointer: handle 空→table.Set; 非空→AllocateAndInitializeEntry+Release_StoreHandle; tagged 部分写 Smi::zero",
   "store_aligned_pointer: WriteUnalignedValue 直写 + tagged 部分写 Smi::zero + 非空时 ForCppHeapPointer 屏障"),
  ("load_raw: ReadUnalignedValue<RawData>(槽地址) 单次读 8 字节",
   "load_raw: RawData{ReadUnalignedValue(pointer 段), ObjectSlot Relaxed_Load(tagged 段)}"),
  ("store_raw: 两段 32 位写 (lo 经 ObjectSlot Relaxed_Store, hi 经 AsAtomicTagged::Relaxed_Store) — tagged 原子性 + 槽仅 4 字节对齐",
   "store_raw: tagged 段 Relaxed_Store + pointer 段 WriteUnalignedValue"),
  ("MustClearDuringSerialization: CppHeapPointerHandle 原子读 != 空 handle",
   "MustClearDuringSerialization: ReadUnalignedValue != kNullAddress"),
];

/// 通用语义要点 (与配置无关)。
pub const EMBEDDER_DATA_SLOT_SEMANTICS: &[&str] = &[
  "构造: EmbedderDataArray 形态 = FIELD_ADDR(array, OffsetOfElementAt(entry_index)); JSObject 形态 = FIELD_ADDR(object, GetEmbedderFieldOffset)",
  "Initialize: tagged 段写初值 (Smi 或只读堆对象) + 清 cpp 指针段",
  "store_tagged: 压缩下 CHECK value 与宿主同 cage base; 写后清 cpp 指针段 (双写保 tagged 段独立)",
  "store_handle_without_barrier: DuplicateEntry 复制 handle, 新 handle 空槽 Release_StoreHandle, tagged 段写 Smi::zero",
  "ToAlignedPointer(ExternalPointerTagRange): 包装对象 tag 命中 → 解包 pointer, 否则 nullptr",
];
`;

const p = writeSrc("v8-objects-arrays", "embedder_data_slot_inl.rs.skel", body);
console.log(
  `embedder-data-slot-inl.h.js: ${p} — 方法 ${unique.length}、配置分支规则 6、通用语义 5、断言 ${checks} 项通过`,
);
