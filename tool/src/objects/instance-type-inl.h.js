#!/usr/bin/env bun
// tool/src/objects/instance-type-inl.h.js — 实例类型检查器内联面生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/instance-type-inl.h → v8-objects-core/src/instance_type_inl.rs
//   (instance-type.h / instance-type-checker.h / instance-types-gen.h 归 instance-type.h.js → instance_type.rs;
//    本脚本登记 -inl 侧: TryGetMapRootIdxFor 表 / 字符串位掩码谓词 / 派生谓词)
// 配置: WASM 开 / TDZ 关 / 静态根关 (V8_STATIC_ROOTS_BOOL=false, 非压缩基线)
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/instance-type-inl.h");
const defs = readV8("src/objects/objects-definitions.h");

let checks = 0;
const need = (cond, msg) => { checks++; if (!cond) throw new Error(msg); };

// —— Map::TryGetMapRootIdxFor: STRUCT_LIST(X-macro) + 显式 case + wasm case ——
const explicitCases = [...inl.matchAll(/^    case (\w+):\s*$\n\s*return RootIndex::k(\w+);/gm)].map((m) => ({ type: m[1], root: m[2] }));
need(explicitCases.length === 16, `case 总数应 16 (非 wasm 14 + wasm 2), 实得 ${explicitCases.length}`);
need(explicitCases[0].type === "DESCRIPTOR_ARRAY_TYPE" && explicitCases[0].root === "DescriptorArrayMap", "首显式 case 漂移");
need(explicitCases[13].type === "SORT_STATE_TYPE" && explicitCases[13].root === "SortStateMap", "末显式 case 漂移");
const wasmCases = explicitCases.slice(-2).map((c) => ({ type: c.type, root: c.root }));
need(wasmCases.length === 2 && wasmCases[0].type === "WASM_FAST_API_CALL_DATA_TYPE" && wasmCases[1].type === "WASM_STRING_VIEW_ITER_TYPE",
  `wasm case 应 2 (WasmFastApiCallData/WasmStringViewIter), 实得 ${JSON.stringify(wasmCases)}`);
// STRUCT_LIST 展开 (与 heap-object.h.js 同法; WASM 开 → 39+1)
const genBlock = defs.match(/#define STRUCT_LIST_GENERATOR\(V, _\)([\s\S]*?)IF_WASM\(V, _, WASM_EXCEPTION_TAG_TYPE/);
need(!!genBlock, "STRUCT_LIST_GENERATOR 未找到");
const structEntries = [...(genBlock ? genBlock[1] : "").matchAll(/V\(_, (\w+), (\w+)/g)].map((m) => ({ type: m[1], name: m[2] }));
need(structEntries.length === 39, `STRUCT_LIST 常规条目应 39, 实得 ${structEntries.length}`);
const structRows = [...structEntries, { type: "WASM_EXCEPTION_TAG_TYPE", name: "WasmExceptionTag" }];

// —— InstanceTypeChecker 字符串谓词位掩码面 (constexpr Is*(InstanceType)) ——
// 函数体可能先有 static_assert/DCHECK, 取块内首个 return 行
function firstReturn(name) {
  const m = inl.match(new RegExp(`V8_INLINE (?:constexpr )?bool ${name}\\(InstanceType instance_type\\) \\{([\\s\\S]*?)\\n\\}`));
  if (!m) return null;
  const r = m[1].match(/return ([^;]+);/);
  return r ? r[1].replace(/\s+/g, " ").trim() : null;
}
const PRED_NAMES = ["IsInternalizedString", "IsSeqString", "IsSeqOneByteString", "IsSeqTwoByteString",
  "IsExternalString", "IsExternalOneByteString", "IsExternalTwoByteString", "IsUncachedExternalString",
  "IsConsString", "IsSlicedString", "IsThinString", "IsOneByteString", "IsTwoByteString", "IsSharedString",
  "IsIndirectString", "IsDirectString"];
const stringPreds = PRED_NAMES.map((n) => ({ name: n, expr: firstReturn(n) }));
need(stringPreds.every((p) => p.expr !== null), `谓词提取失败: ${stringPreds.filter((p) => p.expr === null).map((p) => p.name)}`);
const byName = Object.fromEntries(stringPreds.map((p) => [p.name, p.expr]));
need(byName["IsInternalizedString"] === "(instance_type & (kIsNotStringMask | kIsNotInternalizedMask)) == (kStringTag | kInternalizedTag)", "IsInternalizedString 表达式漂移");
need(byName["IsSeqString"] === "(instance_type & (kIsNotStringMask | kStringRepresentationMask)) == kSeqStringTag", "IsSeqString 表达式漂移");
need(byName["IsThinString"] === "(instance_type & (kIsNotStringMask | kStringRepresentationMask)) == kThinStringTag", "IsThinString 表达式漂移");
need(byName["IsOneByteString"] === "(instance_type & kStringEncodingMask) == kOneByteStringTag", "IsOneByteString 表达式漂移");
need(byName["IsIndirectString"] === "(instance_type & (kIsNotStringMask | kIsIndirectStringMask)) == kIsIndirectStringTag", "IsIndirectString 表达式漂移");
need(byName["IsUncachedExternalString"] === "(instance_type & (kIsNotStringMask | kUncachedExternalStringMask | kStringRepresentationMask)) == (kExternalStringTag | kUncachedExternalStringTag)", "IsUncachedExternalString 表达式漂移");

// —— 派生谓词 (组合判定, 含非字符串) ——
const derivedPreds = [...inl.matchAll(/V8_INLINE constexpr bool (Is\w+)\(InstanceType instance_type\) \{\s*\n\s*return !?Is\w+\(/g)].map((m) => m[1]);
for (const need2 of ["IsReferenceComparable", "IsGcSafeCode", "IsAbstractCode", "IsFreeSpaceOrFiller", "IsMaybeReadOnlyJSObject", "IsPropertyDictionary", "IsNativeContextSpecific", "IsJSApiWrapperObject", "IsCppHeapPointerWrapperObject"])
  need(derivedPreds.includes(need2) || inl.includes(`bool ${need2}(InstanceType`) || inl.includes(`bool ${need2}(\n`), `派生谓词缺 ${need2}`);

// —— 静态根面登记 (V8_STATIC_ROOTS_BOOL 分支, 基线关, 只登记存在性) ——
const staticRootsBlock = inl.match(/#if V8_STATIC_ROOTS_BOOL[\s\S]*?#else\s*\n\s*inline bool MayHaveMapCheckFastCase\(InstanceType type\) \{ return false; \}/);
need(!!staticRootsBlock, "V8_STATIC_ROOTS_BOOL 双分支缺失");
const rangeListCount = (inl.match(/kUniqueMapRangeOfInstanceTypeRangeList/g) || []).length;
need(rangeListCount >= 3, "kUniqueMapRangeOfInstanceTypeRangeList 登记缺失");
const stringRangeEntries = [...(inl.match(/struct kUniqueMapRangeOfStringType \{([\s\S]*?)\n\};/)?.[1].matchAll(/TaggedAddressRange k(\w+) =/g) || [])].map((m) => m[1]);
need(stringRangeEntries.join() === "SeqString,InternalizedString,ExternalString,UncachedExternalString,ConsString,SlicedString,ThinString,SharedSeqString,SharedExternalString,DirectString,IndirectString",
  `kUniqueMapRangeOfStringType 应 11 段: ${stringRangeEntries}`);

// —— 宏展开骨架面 ——
need(inl.includes("#define INSTANCE_TYPE_CHECKER1(type, forinstancetype)"), "INSTANCE_TYPE_CHECKER1 宏缺失");
need(inl.includes("#define INSTANCE_TYPE_CHECKER_RANGE1(type, first_instance_type,"), "INSTANCE_TYPE_CHECKER_RANGE1 宏缺失");
need(inl.includes("INSTANCE_TYPE_CHECKERS_SINGLE(INSTANCE_TYPE_CHECKER1)"), "SINGLE 列表展开缺失");
need(inl.includes("INSTANCE_TYPE_CHECKERS_RANGE(INSTANCE_TYPE_CHECKER_RANGE1)"), "RANGE 列表展开缺失");

const body = `${banner("src/objects/instance-type-inl.h.js", "src/objects/instance-type-inl.h", "基线: 静态根关; 位掩码谓词 1:1 承接")}
//! 实例类型检查器内联面 (原版 instance-type-inl.h)
//!
//! 非静态根基线下 INSTANCE_TYPE_CHECKER2/RANGE2 退化为
//! \`Is##type(map->instance_type())\`, 位掩码语义全部落在 InstanceType 重载上.

// ===== Map::TryGetMapRootIdxFor: 唯一 map 表 =====
/// 实例类型 → 唯一只读 map 的 RootIndex (显式 ${explicitCases.length} + wasm ${wasmCases.length} + STRUCT_LIST ${structRows.length} = ${explicitCases.length + wasmCases.length + structRows.length} 行)
pub const UNIQUE_MAP_OF_INSTANCE_TYPE: [(&str, &str); ${explicitCases.length + wasmCases.length + structRows.length}] = [
  // (INSTANCE_TYPE, RootIndex 名)
${[...structRows.map((e) => ({ type: e.type, root: `${e.name}Map` })), ...explicitCases.map((e) => ({ type: e.type, root: e.root })), ...wasmCases.map((e) => ({ type: e.type, root: e.root }))]
    .map((e) => `  ("${e.type}", "${e.root}"),`).join("\n")}
];

/// 查实例类型的唯一 map (原版 UniqueMapOfInstanceType: SINGLE 检查器链 → TryGetMapRootIdxFor 兜底)
pub fn unique_map_of_instance_type(/* type: InstanceType */) /* -> Option<RootIndex> */ {
  todo!("就地修订: 先查 UNIQUE_MAP_OF_INSTANCE_TYPE 表, miss 返回 None")
}

// ===== 字符串位掩码谓词 (InstanceType 重载, ${stringPreds.length} 个) =====
${stringPreds.map((p) => `/// 原版: ${p.expr}\npub fn ${p.name.replace(/^Is/, "is_").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(/* instance_type: InstanceType */) -> bool {\n  todo!("就地修订: ${p.expr.replace(/"/g, "'")}")\n}\n`).join("")}
// ===== 派生谓词 (组合判定) =====
/// 引用可比较 (非 String/BigInt/HeapNumber — 值语义类型不可按引用比较)
pub fn is_reference_comparable(/* instance_type */) -> bool {
  todo!("就地修订: !IsString && !IsBigInt && != HEAP_NUMBER_TYPE")
}

/// GC 安全 Code 判定 (原版 IsGcSafeCode = IsCode)
pub fn is_gc_safe_code(/* instance_type */) -> bool {
  todo!("就地修订: 直接转发 is_code")
}

/// AbstractCode 判定 (BytecodeArray 或 Code)
pub fn is_abstract_code(/* instance_type */) -> bool {
  todo!("就地修订: IsBytecodeArray(it) || IsCode(it)")
}

/// FreeSpace/Filler 判定 (GC 填充对象)
pub fn is_free_space_or_filler(/* instance_type */) -> bool {
  todo!("就地修订: it == FREE_SPACE_TYPE || it == FILLER_TYPE")
}

/// 绑定 native context 的对象判定 (Context 全是; JSReceiver 除外清单)
pub fn is_native_context_specific(/* instance_type */) -> bool {
  todo!("就地修订: IsContext→true; !IsJSReceiver→false; 例外: MaybeReadOnlyJSObject/AlwaysSharedSpaceJSObject/WasmObject")
}

// ===== 静态根面 (登记不展开; 基线 V8_STATIC_ROOTS_BOOL=false) =====
/// 静态根开关下 map 地址区间快查表条目数 (kUniqueMapRangeOfInstanceTypeRangeList, 原版 9)
pub const STATIC_ROOTS_RANGE_LIST_LEN: usize = 9;

/// 字符串 map 地址分段 (kUniqueMapRangeOfStringType, ${stringRangeEntries.length} 段; 静态根开时生效)
pub const STRING_MAP_RANGES: [&str; ${stringRangeEntries.length}] = [${stringRangeEntries.map((n) => `"${n}"`).join(", ")}];

/// map 一致性检查 (静态根开: 压缩值比 StaticReadOnlyRootsPointerTable; 关: 退化为 instance_type 比较)
pub fn check_instance_map(/* expected: RootIndex, map: Tagged<Map> */) -> bool {
  todo!("就地修订: 按静态根开关分支")
}
`;

const p = writeSrc("v8-objects-core", "instance_type_inl.rs", body);
console.log(`instance-type-inl.h.js: 已生成 ${p} (must ${checks}: 唯一 map 表 ${explicitCases.length + wasmCases.length + structRows.length} 行, 位掩码谓词 ${stringPreds.length}, 字符串区间 ${stringRangeEntries.length} 段)`);
