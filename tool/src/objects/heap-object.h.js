#!/usr/bin/env bun
// tool/src/objects/heap-object.h.js — HeapObject 主族差集骨架生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/heap-object.h + heap-object.tq + heap-object-inl.h
//   + heap-object-field-inl.h + heap-object-set-map-inl.h
//   → v8-objects-core/src/heap_object.rs.skel
//   (heap_object.rs 已有手工落地实现, 直出主文件会覆盖修订, 故走 .skel 差集 — 同 smi.rs.skel 惯例)
// 规则面: 谓词声明 X-macro 展开计数 / set_map 变体面 / 字段访问器族 / tq 布局注解;
//   语义面 (写屏障/GC 场景) 留 doc + todo!
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/heap-object.h");
const tq = readV8("src/objects/heap-object.tq");
const inl = readV8("src/objects/heap-object-inl.h");
const fieldInl = readV8("src/objects/heap-object-field-inl.h");
const setMapInl = readV8("src/objects/heap-object-set-map-inl.h");
const listMacros = readV8("src/objects/object-list-macros.h");
const defs = readV8("src/objects/objects-definitions.h");

let checks = 0;
const need = (cond, msg) => { checks++; if (!cond) throw new Error(msg); };

// —— heap-object.tq 布局 ——
const tqAnnots = [...tq.matchAll(/^@(\w+)/gm)].map((m) => m[1]);
need(tqAnnots.join() === "abstract,doNotGenerateCast,doNotGenerateCppClass,cppObjectLayoutDefinition",
  `heap-object.tq 注解漂移: ${tqAnnots}`);
need(/extern class HeapObject extends StrongTagged \{\s*const map: Map;/.test(tq), "heap-object.tq 应只有 const map: Map 一个字段");

// —— 布局断言: map_ 偏移 0 (Internals::kHeapObjectMapOffset 契约) ——
need(/static_assert\(offsetof\(HeapObject, map_\) == Internals::kHeapObjectMapOffset\)/.test(h), "map_ offsetof 断言缺失");
need(/using MapField = TaggedField<MapWord, 0>;/.test(h), "MapField = TaggedField<MapWord, 0> 缺失");

// —— 谓词声明 X-macro 面 (heap-object.h 尾部) ——
const heapObjExtra = [...h.matchAll(/^IS_TYPE_FUNCTION_DECL\((\w+)\)$/gm)].map((m) => m[1]);
need(heapObjExtra.join() === "HashTableBase,SloppyArgumentsElements,SmallOrderedHashTable,PropertyDictionary,AnyHole,TdzHole,UndefinedContextCell,NullOrUndefined",
  `显式谓词声明应 8 项 (TDZ 关含 TdzHole): ${heapObjExtra}`);
// ODDBALL_LIST / HOLE_LIST / STRUCT_LIST 展开 (object-list-macros.h + objects-definitions.h)
const oddballs = [...listMacros.matchAll(/^  V\((\w+), \w+, \w+\)/gm)].length; // 该正则会撞到别的列表, 改为按段提取
void oddballs;
function listEntries(src, name) {
  const m = src.match(new RegExp(`#define ${name}\\(V\\)([\\s\\S]*?)\\n\\n`));
  if (!m) throw new Error(`未找到 ${name}`);
  return [...m[1].matchAll(/V\((\w+)(?:,|\)|,)/g)].map((x) => x[1]);
}
const ODDBALL = listEntries(listMacros, "ODDBALL_LIST");
need(ODDBALL.join() === "Undefined,Null,True,False", `ODDBALL_LIST 漂移: ${ODDBALL}`);
// HOLE_LIST: 12 显式 + TDZ_HOLE_LIST (TDZ 关 → DisabledTdzHole) = 13
const holeBlock = (listMacros.match(/#define HOLE_LIST\(V\)([\s\S]*?)\n\n/) ?? ["", ""])[1];
const holeExplicit = [...holeBlock.matchAll(/^  V\((\w+),/gm)].map((m) => m[1]);
need(holeExplicit.join() === "TheHole,PropertyCellHole,HashTableHole,PromiseHole,ExceptionHole,TerminationException,UninitializedHole,ArgumentsMarker,OptimizedOut,StaleRegister,SelfReferenceMarker,BasicBlockCountersMarker",
  `HOLE_LIST 显式 12 项漂移: ${holeExplicit}`);
need(/#ifndef V8_ENABLE_TDZ_HOLE\s*\nIS_TYPE_FUNCTION_DECL\(TdzHole\)/.test(h), "TdzHole #ifndef 声明缺失");
need(/IS_TYPE_FUNCTION_DECL\(UndefinedContextCell\)$/m.test(h) && /IS_TYPE_FUNCTION_DECL\(NullOrUndefined\)$/m.test(h),
  "UndefinedContextCell / NullOrUndefined 声明缺失");
// STRUCT_LIST_GENERATOR 条目 (objects-definitions.h; WASM 开 → 39+1=40? 独立 awk 计数: V(_, 39 + IF_WASM 1 = 40)
const genBlock = defs.match(/#define STRUCT_LIST_GENERATOR\(V, _\)([\s\S]*?)IF_WASM\(V, _, WASM_EXCEPTION_TAG_TYPE/);
need(!!genBlock, "STRUCT_LIST_GENERATOR 未找到");
const structEntries = [...(genBlock ? genBlock[1] : "").matchAll(/V\(_, (\w+), (\w+)/g)].map((m) => ({ type: m[1], name: m[2] }));
need(structEntries.length === 39, `STRUCT_LIST_GENERATOR 常规条目应 39, 实得 ${structEntries.length}`);
const STRUCT_LIST_TOTAL = structEntries.length + 1; // + WASM_EXCEPTION_TAG (WASM 开)
need(defs.includes("IF_WASM(V, _, WASM_EXCEPTION_TAG_TYPE, WasmExceptionTag, wasm_exception_tag)"), "IF_WASM 尾条目缺失");

// —— heap-object-inl.h 谓词面 ——
const castTraits = [...inl.matchAll(/^DEF_CAST_TRAITS\((\w+)\)/gm)].map((m) => m[1]);
need(castTraits.length === 31, `DEF_CAST_TRAITS 应 31, 实得 ${castTraits.length}`);
const heapPreds = [...inl.matchAll(/^DEF_HEAP_OBJECT_PREDICATE\((\w+)\)/gm)].map((m) => m[1]);
need(heapPreds.join() === "IsDependentCode,IsSmiStringCache", `DEF_HEAP_OBJECT_PREDICATE 漂移: ${heapPreds}`);
const ptrOverloads = [...inl.matchAll(/^HEAPOBJECT_PTR_OVERLOAD\((\w+)\)/gm)].map((m) => m[1]);
need(ptrOverloads.join() === "FreeSpaceOrFiller,JSError,SmallOrderedHashTable", `HEAPOBJECT_PTR_OVERLOAD 漂移: ${ptrOverloads}`);

// —— set_map 变体面 (heap-object-set-map-inl.h) ——
const setMapDefs = [...setMapInl.matchAll(/^void HeapObject::(\w+)\(/gm)].map((m) => m[1]);
for (const fn of ["set_map", "set_map_safe_transition", "set_map_safe_transition_no_write_barrier", "set_map_no_write_barrier", "set_map_word", "set_map_word_forwarded", "set_map_after_allocation"])
  need(setMapDefs.includes(fn), `set-map 族缺 ${fn}`);
const setMapWordCount = setMapDefs.filter((n) => n === "set_map_word" || n === "set_map_word_forwarded").length;
need(setMapWordCount === 4, `set_map_word/forwarded 重载应 4, 实得 ${setMapWordCount}`); // relaxed/release 各 2
need(/bool HeapObject::relaxed_compare_and_swap_map_word_forwarded/.test(setMapInl), "map word CAS 缺失");
need(/WriteBarrierMode AllocationWitness::WriteBarrierModeForAllocation/.test(setMapInl), "WriteBarrierModeForAllocation 缺失");

// —— VerificationMode / EmitWriteBarrier 枚举 ——
need(/enum class VerificationMode \{\s*kSafeMapTransition,\s*kPotentialLayoutChange,\s*\};/.test(h), "VerificationMode 枚举漂移");
need(/enum class EmitWriteBarrier \{\s*kYes,\s*kNo,\s*\};/.test(h), "EmitWriteBarrier 枚举漂移");

// —— heap-object-field-inl.h 字段访问器族 ——
const fieldDefs = [...fieldInl.matchAll(/^(?:template <[^>]+>\s*)?(?:\w[\w:<>* &]*) HeapObject::(\w+)\(/gm)].map((m) => m[1]);
const FIELD_FAMILY = ["ReadSandboxedPointerField", "WriteSandboxedPointerField", "ReadBoundedSizeField", "WriteBoundedSizeField",
  "InitExternalPointerField", "ReadExternalPointerField", "ReadCppHeapPointerField", "WriteExternalPointerField",
  "SetupLazilyInitializedExternalPointerField", "IsLazilyInitializedExternalPointerFieldInitialized",
  "WriteLazilyInitializedExternalPointerField", "SetupLazilyInitializedCppHeapPointerField",
  "WriteLazilyInitializedCppHeapPointerField", "RawField", "RawMaybeWeakField", "RawInstructionStreamField",
  "RawExternalPointerField", "RawCppHeapPointerField", "RawIndirectPointerField"];
let fieldHit = 0;
for (const fn of FIELD_FAMILY) if (fieldDefs.includes(fn)) fieldHit++;
need(fieldHit === FIELD_FAMILY.length, `字段访问器族应 ${FIELD_FAMILY.length} 命中, 实得 ${fieldHit}`);

const body = `${banner("src/objects/heap-object.h.js", "src/objects/heap-object.{h,tq} + heap-object-inl.h + heap-object-field-inl.h + heap-object-set-map-inl.h", "差集骨架 .skel; heap_object.rs 已有落地实现, 合并后可删")}
//! HeapObject 主族差集骨架 (原版 5 文件)
//!
//! 已落地: 基础布局/map 访问 (heap_object.rs).
//! 本骨架补: 谓词声明面 / set_map 变体矩阵 / 受保护与外指字段访问器族 /
//! AllocationWitness — 语义留 todo!, Developer 修订环落位.

// ===== 布局 (heap-object.tq) =====
// tq 注解: ${tqAnnots.join(" ")}
// extern class HeapObject extends StrongTagged { const map: Map; }
// 非压缩基线: K_HEADER_SIZE = kTaggedSize = 8; map_ 偏移 0 (Internals::kHeapObjectMapOffset)

/// HeapObject 头大小 (仅 map 字段, 非压缩 kTaggedSize=8)
pub const K_HEADER_SIZE: usize = 8;

/// map_ 字段偏移 (原版 static_assert offsetof == Internals::kHeapObjectMapOffset)
pub const K_HEAP_OBJECT_MAP_OFFSET: usize = 0;

// ===== 谓词声明面 (heap-object.h 尾部 X-macro) =====
// HEAP_OBJECT_TYPE_LIST(IS_TYPE_FUNCTION_DECL) + 显式 ${heapObjExtra.length} 项 + ODDBALL ${ODDBALL.length} 项
// + HOLE ${holeExplicit.length + 1} 项 (TDZ 关: DisabledTdzHole) + TdzHole(#ifndef) + UndefinedContextCell + NullOrUndefined
// + STRUCT_LIST ${STRUCT_LIST_TOTAL} 项 (WASM 开, 含 WasmExceptionTag)

/// 显式类型谓词补充声明 (heap-object.h, 原版 ${heapObjExtra.length} 项)
pub const EXTRA_TYPE_PREDICATES: [&str; ${heapObjExtra.length}] = [${heapObjExtra.map((n) => `"${n}"`).join(", ")}];

/// ODDBALL 谓词 (ODDBALL_LIST 展开, ${ODDBALL.length} 项)
pub const ODDBALL_PREDICATES: [&str; ${ODDBALL.length}] = [${ODDBALL.map((n) => `"Is${n}"`).join(", ")}];

/// HOLE 谓词 (HOLE_LIST 展开, TDZ 关 → DisabledTdzHole, ${holeExplicit.length + 1} 项)
pub const HOLE_PREDICATES: [&str; ${holeExplicit.length + 1}] = [${[...holeExplicit, "DisabledTdzHole"].map((n) => `"Is${n}"`).join(", ")}];

/// STRUCT 谓词 (STRUCT_LIST 展开, WASM 开, ${STRUCT_LIST_TOTAL} 项; 摘录 TYPE→Name 对)
pub const STRUCT_PREDICATES: [(&str, &str); ${STRUCT_LIST_TOTAL}] = [
${[...structEntries, { type: "WASM_EXCEPTION_TAG_TYPE", name: "WasmExceptionTag" }].map((e) => `  ("${e.type}", "${e.name}"),`).join("\n")}
];

// ===== set_map 变体矩阵 (heap-object-set-map-inl.h) =====
// 维度: 写屏障(kYes/kNo) × 内存序(relaxed/release) × 验证模式(kSafeMapTransition/kPotentialLayoutChange)
// 公开入口 8 个 + 私有模板 set_map<emit_write_barrier, MemoryOrder, IsolateT>(isolate, value, order, mode)

/// map 写入入口矩阵 (公开 ${setMapDefs.filter((n) => n.startsWith("set_map") || n.startsWith("relaxed_")).length} 个, 详见 doc)
pub const SET_MAP_ENTRIES: [(&str, bool, &str); 8] = [
  // (入口, 发写屏障, 验证模式)
  ("set_map", true, "kPotentialLayoutChange"),
  ("set_map(release)", true, "kPotentialLayoutChange"),
  ("set_map_safe_transition", true, "kSafeMapTransition"),
  ("set_map_safe_transition(release)", true, "kSafeMapTransition"),
  ("set_map_safe_transition_no_write_barrier", false, "kSafeMapTransition"),
  ("set_map_safe_transition_no_write_barrier(release)", false, "kSafeMapTransition"),
  ("set_map_no_write_barrier", false, "kPotentialLayoutChange"),
  ("set_map_no_write_barrier(release)", false, "kPotentialLayoutChange"),
];

/// 分配后写 map (原版 set_map_after_allocation: relaxed 写 map word, 按模式补写屏障)
pub fn set_map_after_allocation(/* isolate, value, mode: WriteBarrierMode */) {
  todo!("就地修订: set_map_word(value, kRelaxedStore) + WriteBarrier::ForValue")
}

/// map 字槽 (原版 map_slot: ObjectSlot(MapField::address(this)))
pub fn map_slot(/* this */) /* -> ObjectSlot */ {
  todo!("就地修订: ObjectSlot(MapField::address)")
}

/// 写 map 字 (原版 set_map_word ×2 / set_map_word_forwarded ×2: relaxed+release)
pub fn set_map_word(/* map 或 target_object, 内存序 */) {
  todo!("就地修订: MapWord::FromMap / FromForwardingAddress → MapField::*_Store_Map_Word")
}

/// map 字 CAS 转发 (原版 relaxed_compare_and_swap_map_word_forwarded)
pub fn relaxed_compare_and_swap_map_word_forwarded(/* old_map_word, new_target */) -> bool {
  todo!("就地修订: MapField::Relaxed_CompareAndSwap, 成功判定 result == old.ptr()")
}

/// 填充对象 map 写入 (原版 SetFillerMap, 经 WritableFreeSpace 写保护路径)
pub fn set_filler_map(/* writable_page, value */) {
  todo!("就地修订: WriteHeaderSlot<Map, offsetof(HeapObject, map_)>")
}

// ===== AllocationWitness =====
/// 分配见证的写屏障模式裁决 (原版 WriteBarrierModeForAllocation:
/// disable_write_barriers / kReadOnly / kYoung 未单代 → SKIP, 其余 UPDATE)
pub fn write_barrier_mode_for_allocation(/* object, allocation: AllocationType */) /* -> WriteBarrierMode */ {
  todo!("就地修订: 三条件 SKIP 判定")
}

// ===== 字段访问器族 (heap-object-field-inl.h, ${FIELD_FAMILY.length} 族) =====
/// 沙箱指针字段读 (40 位有界偏移 + cage base)
pub fn read_sandboxed_pointer_field(/* offset, cage_base */) /* -> Address */ {
  todo!("就地修订: 委托 i::ReadSandboxedPointerField(field_address(offset), cage_base)")
}

/// 外部指针字段族 (Init/Read/Write × 强类型 tag / 运行时 tag 两种形态)
pub fn external_pointer_field_family() {
  todo!("就地修订: InitExternalPointerField(2 重载)/ReadExternalPointerField(2)/WriteExternalPointerField(2), EPT 写屏障")
}

/// 惰性外部指针字段族 (Setup/IsInitialized/Write: 首写分配 EPT 项)
pub fn lazily_initialized_external_pointer_field_family() {
  todo!("就地修订: null handle 判定 + AllocateAndInitializeEntry + Release_Store")
}

/// CppHeap 指针字段族 (Read ×2 / Setup / Write, 经 CppHeapPointerTable)
pub fn cpp_heap_pointer_field_family() {
  todo!("就地修订: 委托 i::ReadCppHeapPointerField / CppHeapPointerSlot::init")
}

/// 原始槽位取景族 (RawField/RawMaybeWeakField/RawInstructionStreamField/RawExternalPointerField/RawCppHeapPointerField/RawIndirectPointerField)
pub fn raw_field_family() {
  todo!("就地修订: field_address(byte_offset) 包各槽位 newtype")
}

// ===== 谓词落地锚 (heap-object-inl.h) =====
/// DEF_CAST_TRAITS 清单 (31 项, 供类型系统对账)
pub const CAST_TRAITS: [&str; ${castTraits.length}] = [${castTraits.map((n) => `"${n}"`).join(", ")}];

/// 非 map 判定型谓词 (DEF_HEAP_OBJECT_PREDICATE, ${heapPreds.length} 项)
pub const ALIAS_PREDICATES: [(&str, &str); ${heapPreds.length}] = [
${heapPreds.map((n, i) => `  ("${n}", "${i === 0 ? "IsWeakArrayList" : "IsFixedArray"}"),`).join("\n")}
];
`;

const p = writeSrc("v8-objects-core", "heap_object.rs.skel", body);
console.log(`heap-object.h.js: 已生成 ${p} (must 断言 ${checks} 项: STRUCT_LIST ${STRUCT_LIST_TOTAL}, CAST_TRAITS ${castTraits.length}, set_map 矩阵 8, 字段族 ${FIELD_FAMILY.length}, tq 注解 ${tqAnnots.length})`);
