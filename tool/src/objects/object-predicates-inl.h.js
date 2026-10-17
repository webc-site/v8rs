#!/usr/bin/env bun
// tool/src/objects/object-predicates-inl.h.js — 对象谓词内联实现生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/object-predicates-inl.h → v8-objects-core/src/object_predicates_inl.rs
//   (object-predicates.h 的名单表面归 tool/src/objects/object-predicates.h.js → object_predicates.rs;
//    本脚本登记 -inl 侧 13 个手写谓词的语义骨架与 IsShared 的 InstanceType switch 表)
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/object-predicates-inl.h");

// —— 手写谓词清单 (按定义顺序) ——
const preds = [...inl.matchAll(/^bool (\w+)\(Tagged<(Object|HeapObject)> obj\)/gm)].map((m) => m[1]);
const EXPECT = ["IsTaggedIndex", "IsZero", "IsNoSharedNameSentinel",
  "IsJSObjectThatCanBeTrackedAsPrototype", "IsJSObjectThatCanBeTrackedAsPrototype", // Object+HeapObject 双重载
  "IsAnyObjectThatCanBeTrackedAsPrototype", "IsAnyObjectThatCanBeTrackedAsPrototype",
  "IsNumber", "IsNumeric", "IsShared"];
if (preds.length !== EXPECT.length || preds.join() !== EXPECT.join())
  throw new Error(`谓词清单漂移 (应 ${EXPECT.length} 项):\n  期望 ${EXPECT}\n  实得 ${preds}`);
// 去重后 9 个语义
const uniquePreds = [...new Set(preds)];
if (uniquePreds.length !== 8) throw new Error(`去重谓词应 8 (10 定义 - 2 双重载), 实得 ${uniquePreds.length}`);

// —— IsShared 的 InstanceType switch 表 (WASM 开: 含 WASM_STRUCT/ARRAY) ——
const sharedCases = [...inl.matchAll(/^    case (\w+):/gm)].map((m) => m[1]);
if (sharedCases.length !== 15) throw new Error(`IsShared case 应 15 (WASM 开: 共享串6+wasm2+内化串6+heap_number1), 实得 ${sharedCases.length}`);
const directShared = ["SHARED_SEQ_TWO_BYTE_STRING_TYPE", "SHARED_SEQ_ONE_BYTE_STRING_TYPE", "SHARED_EXTERNAL_TWO_BYTE_STRING_TYPE",
  "SHARED_EXTERNAL_ONE_BYTE_STRING_TYPE", "SHARED_UNCACHED_EXTERNAL_TWO_BYTE_STRING_TYPE", "SHARED_UNCACHED_EXTERNAL_ONE_BYTE_STRING_TYPE"];
for (const c of directShared)
  if (!sharedCases.includes(c)) throw new Error(`IsShared 直接共享段缺 ${c}`);
const wasmShared = ["WASM_STRUCT_TYPE", "WASM_ARRAY_TYPE"];
for (const c of wasmShared)
  if (!sharedCases.includes(c)) throw new Error(`IsShared wasm 段缺 ${c}`);
const internalizedShared = ["INTERNALIZED_TWO_BYTE_STRING_TYPE", "INTERNALIZED_ONE_BYTE_STRING_TYPE",
  "EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE", "EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE",
  "UNCACHED_EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE", "UNCACHED_EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE"];
for (const c of internalizedShared)
  if (!sharedCases.includes(c)) throw new Error(`IsShared 内化串段缺 ${c}`);
if (!sharedCases.includes("HEAP_NUMBER_TYPE")) throw new Error("IsShared 缺 HEAP_NUMBER_TYPE 分支");

// —— 语义锚 ——
if (!/IsSmi\(obj\) &&\s*\n\s*TaggedIndex::IsValid/.test(inl)) throw new Error("IsTaggedIndex 语义锚缺失");
if (!/return obj == Smi::zero\(\);/.test(inl)) throw new Error("IsZero 语义锚缺失");
if (!/obj == SharedFunctionInfo::kNoSharedNameSentinel/.test(inl)) throw new Error("IsNoSharedNameSentinel 语义锚缺失");
if (!/IsJSObject\(obj\) && !HeapLayout::InWritableSharedSpace\(obj\)/.test(inl)) throw new Error("原型可跟踪谓词语义锚缺失");
if (!/\(IsJSObject\(obj\) \|\| IsWasmObject\(obj\)\)/.test(inl)) throw new Error("IsAnyObjectThatCanBeTrackedAsPrototype wasm 合取缺失");
if (!/if \(IsSmi\(obj\)\) return true;\s*\n\s*return IsHeapNumber\(Cast<HeapObject>\(obj\)\);/.test(inl)) throw new Error("IsNumber 语义锚缺失");
if (!/IsHeapNumber\(heap_object\) \|\| IsBigInt\(heap_object\)/.test(inl)) throw new Error("IsNumeric 语义锚缺失");
if (!/InstanceTypeChecker::IsAlwaysSharedSpaceJSObject\(instance_type\)/.test(inl)) throw new Error("IsShared 快路径锚缺失");

const body = `${banner("src/objects/object-predicates-inl.h.js", "src/objects/object-predicates-inl.h", "13 定义 / 9 语义; 名单表面在 object_predicates.rs")}
//! 对象谓词内联实现 (原版 object-predicates-inl.h)
//!
//! 名单型谓词 (Is##Type X-macro 家族) 的声明与展开表面在
//! object_predicates.rs; 本模块承接 ${preds.length} 个定义 (${uniquePreds.length} 个语义, 其中
//! 原型跟踪谓词各带 Object/HeapObject 双重载).

/// TaggedIndex 判定: Smi 且值域经 TaggedIndex::IsValid
pub fn is_tagged_index(/* obj: Tagged<Object> */) -> bool {
  todo!("就地修订: IsSmi(obj) && TaggedIndex::IsValid(value)")
}

/// 零值判定: 与 Smi::zero() 全等
pub fn is_zero(/* obj */) -> bool {
  todo!("就地修订: obj == Smi::zero()")
}

/// 无共享名哨兵判定 (SharedFunctionInfo::kNoSharedNameSentinel)
pub fn is_no_shared_name_sentinel(/* obj */) -> bool {
  todo!("就地修订: obj == kNoSharedNameSentinel")
}

/// 可作原型跟踪的 JSObject (Object 重载先 TryCast HeapObject)
pub fn is_js_object_that_can_be_tracked_as_prototype(/* obj: Tagged<Object> | Tagged<HeapObject> */) -> bool {
  todo!("就地修订: IsJSObject(obj) && !HeapLayout::InWritableSharedSpace(obj)")
}

/// 可作原型跟踪的任意对象 (JSObject 或 WasmObject, 排除共享空间)
pub fn is_any_object_that_can_be_tracked_as_prototype(/* obj */) -> bool {
  todo!("就地修订: (IsJSObject || IsWasmObject) && !InWritableSharedSpace")
}

/// Number 判定: Smi 或 HeapNumber
pub fn is_number(/* obj: Tagged<Object> */) -> bool {
  todo!("就地修订: IsSmi→true, 否则 IsHeapNumber")
}

/// Numeric 判定: Smi 或 HeapNumber 或 BigInt
pub fn is_numeric(/* obj: Tagged<Object> */) -> bool {
  todo!("就地修订: IsSmi→true, 否则 IsHeapNumber || IsBigInt")
}

/// IsShared 直判 InstanceType 分段表 (${sharedCases.length} 个 case, WASM 开: 共享串 6 + wasm 2 + 内化串 6 + HEAP_NUMBER 1)
///
/// - 直接共享: 共享串 6 种 (SHARED_{SEQ,EXTERNAL,UNCACHED_EXTERNAL}_{ONE,TWO}_BYTE)
/// - 条件共享: WASM_STRUCT/WASM_ARRAY (InAnySharedSpace), 内化串 6 种
///   (v8_flags.shared_string_table 开时共享), HEAP_NUMBER (InWritableSharedSpace)
pub const IS_SHARED_CASES: [&str; ${sharedCases.length}] = [
${sharedCases.map((c) => `  "${c}",`).join("\n")}
];

/// 共享性判定 (原版 IsShared, 快路径 IsAlwaysSharedSpaceJSObject + switch 表)
pub fn is_shared(/* obj: Tagged<Object> */) -> bool {
  todo!("就地修订: Smi/RO → true; IsAlwaysSharedSpaceJSObject → true; 按 IS_SHARED_CASES 分段")
}
`;

const p = writeSrc("v8-objects-core", "object_predicates_inl.rs", body);
console.log(`object-predicates-inl.h.js: 已生成 ${p} (定义 ${preds.length}, 语义 ${uniquePreds.length}, IsShared case ${sharedCases.length})`);
