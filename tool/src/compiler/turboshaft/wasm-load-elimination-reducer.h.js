#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-load-elimination-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-load-elimination-reducer.{h,cc}
// → v8-turboshaft/src/wasm_load_elimination_reducer.rs.skel
//
// 提取面: 负索引哨兵常量 (6); kLoadLikeType; WasmMemoryAddress/KeyData/
// BaseData 结构; EntriesWithOffsets 枚举; analyzer 方法面。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-load-elimination-reducer.h");
const cc = readV8("src/compiler/turboshaft/wasm-load-elimination-reducer.cc");

// 1. 负索引哨兵常量
const negConsts = [...h.matchAll(/^static constexpr int (k\w+Index) = (-\d+);/gm)].map((m) => ({ name: m[1], value: +m[2] }));
if (negConsts.length !== 6) throw new Error(`负索引常量 ${negConsts.length} != 6`);

// 2. kLoadLikeType / kLoadLikeSize
const llt = h.match(/static constexpr wasm::ModuleTypeIndex kLoadLikeType\{wasm::kV8MaxWasmTypes \+ 1\};/);
const lls = h.match(/static constexpr int kLoadLikeSize = 4;.*fair dice roll/);
if (!llt || !lls) throw new Error("缺 kLoadLikeType/Size");

// 3. WasmMemoryAddress 字段
const wmaM = h.match(/struct WasmMemoryAddress \{([\s\S]*?)\n\};/);
if (!wmaM) throw new Error("未找到 WasmMemoryAddress");
const wmaFields = [...wmaM[1].matchAll(/^\s{2}(\w[\w:<>. ]*?)\s+(\w+);/gm)].map((m) => `${m[1]} ${m[2]}`);
if (wmaFields.length !== 5) throw new Error(`WasmMemoryAddress 字段 ${wmaFields.length} != 5`);

// 4. 枚举与结构哨兵
for (const [re, name] of [
  [/struct KeyData \{/, "KeyData"],
  [/Key\* prev_same_base = nullptr;/, "同基链"],
  [/Key next_same_offset = \{\};/, "同偏移链 (值语义)"],
  [/struct OffsetListTraits \{/, "OffsetListTraits"],
  [/struct BaseListTraits \{/, "BaseListTraits"],
  [/struct BaseData \{/, "BaseData"],
  [/DoublyThreadedList<Key, BaseListTraits>/, "双链表"],
  [/enum class EntriesWithOffsets \{ ([^}]+)\}/, "EntriesWithOffsets"],
  [/class V8_EXPORT_PRIVATE WasmLoadEliminationReducer : public Next/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(WasmLoadElimination\)/, "boilerplate"],
  [/struct Snapshot \{/, "内嵌 Snapshot"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}
const ewo = h.match(/enum class EntriesWithOffsets \{\s*k(\w+),\s*k(\w+)\s*\}/);
if (!ewo) throw new Error("EntriesWithOffsets 值解析失败");

// 5. cc 方法面
const ccMethods = [...new Set([...cc.matchAll(/^void WasmLoadEliminationAnalyzer::(\w+)\(/gm)].map((m) => m[1]))];

let rust = banner(
  "src/compiler/turboshaft/wasm-load-elimination-reducer.h.js",
  "src/compiler/turboshaft/wasm-load-elimination-reducer.{h,cc}",
  "wasm 载入消除: 依内存地址追踪冗余 load",
);

rust += `
/// 原版负索引哨兵 — 表内特殊键 (非真实字段偏移):
${negConsts.map((c) => `/// - ${c.name} = ${c.value}`).join("\n")}
/// 另: kLoadLikeType = ModuleTypeIndex{kV8MaxWasmTypes+1} (哨兵类型),
/// kLoadLikeSize = 4 ("fair dice roll" 注释)。
/// v8: wasm-load-elimination-reducer.h 顶部常量
pub const K_ARRAY_LENGTH_FIELD_INDEX: i32 = ${negConsts.find((c) => c.name === "kArrayLengthFieldIndex").value};
pub const K_STRING_PREPARE_FOR_GET_CODEUNIT_INDEX: i32 = ${negConsts.find((c) => c.name === "kStringPrepareForGetCodeunitIndex").value};
pub const K_STRING_AS_WTF16_INDEX: i32 = ${negConsts.find((c) => c.name === "kStringAsWtf16Index").value};
pub const K_ANY_CONVERT_EXTERN_INDEX: i32 = ${negConsts.find((c) => c.name === "kAnyConvertExternIndex").value};
pub const K_ASSERT_NOT_NULL_INDEX: i32 = ${negConsts.find((c) => c.name === "kAssertNotNullIndex").value};
pub const K_GET_DESC_INDEX: i32 = ${negConsts.find((c) => c.name === "kGetDescIndex").value};

/// 原版 WasmMemoryAddress — 消除键的五元组
/// v8: wasm-load-elimination-reducer.h WasmMemoryAddress
pub struct WasmMemoryAddress {
${wmaFields.map((f) => `  pub ${f.split(" ").slice(-1)[0]}: ${f.split(" ")[0]},`).join("\n")}
}

/// 原版 KeyData — 快照表键数据: WasmMemoryAddress + 同基/同偏移双向链
/// (同基链 prev 为指针 next 为值; OffsetListTraits/BaseListTraits 供
/// DoublyThreadedList 使用; BaseData 按 base 聚合 with_offsets 键)。
/// v8: wasm-load-elimination-reducer.h KeyData/BaseData
pub struct KeyData {
  pub mem: WasmMemoryAddress,
  pub prev_same_base: Option<Key>,
  pub next_same_offset: Key,
}

/// 原版 EntriesWithOffsets — offset 化简后旧键处置
/// v8: wasm-load-elimination-reducer.h EntriesWithOffsets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntriesWithOffsets {
  /// kInvalidate: 作废
  Invalidate,
  /// kKeep: 保留
  Keep,
}

/// 原版 analyzer/reducer 面:
/// - cc analyzer 方法 (${ccMethods.length} 个): ${ccMethods.join(", ")}
/// - reducer 持内嵌 Snapshot 结构, 按块恢复/合并
/// v8: wasm-load-elimination-reducer.{h,cc}
pub struct WasmLoadEliminationReducer;
`;

const p = writeSrc("v8-turboshaft", "wasm_load_elimination_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`负常量=${negConsts.length} 地址字段=${wmaFields.length} cc方法=${ccMethods.length}`);
