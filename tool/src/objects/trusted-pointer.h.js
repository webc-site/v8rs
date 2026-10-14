#!/usr/bin/env bun
// tool/src/objects/trusted-pointer.h.js — trusted.rs 差集骨架生成 (源 3/4)
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/trusted-pointer.h → v8-objects-arrays/src/trusted.rs (差集骨架)
// 产物: v8-objects-arrays/src/trusted_pointer_h.rs.skel (-inl 骨架合并到本文件, 最终以就地修订 src 为准)
// 断言: TRUSTED_POINTER_TAG_TO_TYPE_MAP X-macro 逐值 14 项比对; TrustedPointerField/
//       TrustedPointerMember 方法名全集与声明计数; handle_ 初值; CodePointerMember 别名
// 推迟登记: Code/BytecodeArray/InterpreterData/UncompiledData/RegExpData/Wasm* 具体类型
//   (分布在上游其他模块, 本 crate 仅落 tag→类型名对照表与 trait 形, 供 AA-1 接驳)
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/trusted-pointer.h");

let checks = 0;
const need = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(msg);
};
const count = (s, re) => [...s.matchAll(re)].length;

// —— X-macro TRUSTED_POINTER_TAG_TO_TYPE_MAP 逐值提取 (禁"理解"宏, 机械展开) ——
const mapMatch = h.match(/#define TRUSTED_POINTER_TAG_TO_TYPE_MAP\(V\)([\s\S]*?)\ntemplate </);
if (!mapMatch) throw new Error("TRUSTED_POINTER_TAG_TO_TYPE_MAP 宏体提取失败");
checks++;
// 续行拼接 + IF_WASM(V, tag, Type) 归一为 V(tag, Type), 按原文顺序机械展开
const body = mapMatch[1]
  .replace(/\\\n/g, "\n")
  .replace(/IF_WASM\(V,/g, "V(");
const entries = [...body.matchAll(/V\(\s*(\w+),\s*(\w+)\)/g)].map((m) => [m[1], m[2]]);
const expected = [
  ["kCodeIndirectPointerTag", "Code"],
  ["kBytecodeArrayIndirectPointerTag", "BytecodeArray"],
  ["kInterpreterDataIndirectPointerTag", "InterpreterData"],
  ["kUncompiledDataIndirectPointerTag", "UncompiledData"],
  ["kRegExpDataIndirectPointerTag", "RegExpData"],
  ["kWasmDispatchTableIndirectPointerTag", "WasmDispatchTable"],
  ["kSharedWasmDispatchTableIndirectPointerTag", "WasmDispatchTable"],
  ["kWasmTrustedInstanceDataIndirectPointerTag", "WasmTrustedInstanceData"],
  ["kSharedWasmTrustedInstanceDataIndirectPointerTag", "WasmTrustedInstanceData"],
  ["kWasmInternalFunctionIndirectPointerTag", "WasmInternalFunction"],
  ["kWasmSuspenderIndirectPointerTag", "WasmSuspenderObject"],
  ["kWasmFunctionDataIndirectPointerTagRange", "WasmFunctionData"],
  ["kWasmExportedFunctionDataIndirectPointerTag", "WasmExportedFunctionData"],
  ["kWasmCapiFunctionDataIndirectPointerTag", "WasmCapiFunctionData"],
];
need(entries.length === 14, `tag→type 映射应 14 项, 实得 ${entries.length}`);
entries.forEach((e, i) => {
  need(e[0] === expected[i][0] && e[1] === expected[i][1], `映射 [${i}] 漂移: 期望 ${expected[i]}, 实得 ${e}`);
});

// —— detail::TrustedPointerType 默认形与 TrustedTypeFor 别名 ——
need(/using type = ExposedTrustedObject;/.test(h), "TrustedPointerType 默认类型应为 ExposedTrustedObject");
need(/using TrustedTypeFor = typename detail::TrustedPointerType<tag_range>::type;/.test(h), "TrustedTypeFor 别名缺失");

// —— TrustedPointerField 声明计数 ——
for (const [name, c] of [
  ["WriteTrustedPointerField(", 1], ["ReadMaybeEmptyTrustedPointerField(", 1],
  ["IsTrustedPointerFieldEmpty(", 1], ["IsTrustedPointerFieldUnpublished(", 1],
  ["ClearTrustedPointerField(", 2], ["ReadTrustedPointerField(", 2], // 2 声明 (注释引用无括号)
]) {
  const hits = count(h, new RegExp(name.replace(/[()]/g, "\\$&"), "g"));
  need(hits === c, `TrustedPointerField.${name.slice(0, -1)} 声明计数应 ${c}, 实得 ${hits}`);
}

// —— TrustedPointerMember 模板形与成员全集 ——
need(/template <typename T, IndirectPointerTagRange kTagRange>\nclass TrustedPointerMember/.test(h), "TrustedPointerMember 模板形漂移 (Rust 以 const 泛参表达)");
need(/: public TaggedMember<T>/.test(h), "非 sandbox 构建应继承 TaggedMember<T>");
need(/handle_\{kNullIndirectPointerHandle\}/.test(h), "handle_ 初值应为 kNullIndirectPointerHandle");
need(/friend class IndirectPointerSlot;/.test(h), "IndirectPointerSlot friend 缺失");
for (const [name, c] of [
  ["load(", 1], ["load_maybe_empty(", 1], ["store(", 1], ["store_no_write_barrier(", 1],
  ["Acquire_Load(", 1], ["Acquire_Load_maybe_empty(", 1], ["Release_Store(", 1],
  ["Release_Store_no_write_barrier(", 1], ["is_empty(", 1], ["is_unpublished(", 1],
  ["clear(", 1], ["storage_address(", 1],
]) {
  const hits = count(h, new RegExp("\\b" + name.replace(/[()]/g, "\\$&"), "g"));
  need(hits === c, `TrustedPointerMember.${name.slice(0, -1)} 声明计数应 ${c}, 实得 ${hits}`);
}
// ctor: 类名( 出现 5 次 = 4 ctor 声明 + 1 注释/模板? 实测 4 ctor + 1 别名行? 不, 别名用 <>; 提取守卫
need(count(h, /TrustedPointerMember\(const AllocationWitness&/g) === 4, "AllocationWitness ctor 应 4 处 (推迟登记: witness 未移植)");
need(/using CodePointerMember = TrustedPointerMember<Code, kCodeIndirectPointerTag>;/.test(h), "CodePointerMember 别名缺失");
need(/class IsolateForSandbox;/.test(h), "IsolateForSandbox 前向声明缺失 (Rust 以 trait 形承接)");

const bodyTable = entries.map(([t, ty]) => `  ("${t}", "${ty}")`).join(",\n");
const skel = `${banner("src/objects/trusted-pointer.h.js", "src/objects/trusted-pointer.h", "trusted.rs 差集骨架 (trusted-pointer.h 部分), 合并后就地修订")}
//! 差集骨架: TrustedPointerField / TrustedPointerMember / tag→type 映射

// X-macro TRUSTED_POINTER_TAG_TO_TYPE_MAP 机械展开 (14 项, 顺序与原文一致):
pub const TRUSTED_POINTER_TAG_TO_TYPE: &[(&str, &str)] = &[
${bodyTable},
];

pub struct TrustedPointerField; // AllStatic
impl TrustedPointerField {
  pub fn read_trusted_pointer_field<T, const TAG_RANGE: IndirectPointerTagRange>(...); // x2 (默认 acquire)
  pub fn read_maybe_empty_trusted_pointer_field<T, const TAG_RANGE: IndirectPointerTagRange>(...);
  pub fn write_trusted_pointer_field<const TAG_RANGE: IndirectPointerTagRange>(...);
  pub fn is_trusted_pointer_field_empty(host, offset) -> bool;
  pub fn is_trusted_pointer_field_unpublished<const TAG_RANGE>(...);
  pub fn clear_trusted_pointer_field(host, offset); // + ReleaseStoreTag 重载
}

#[repr(C)]
pub struct TrustedPointerMember<T, const K_TAG_RANGE: IndirectPointerTagRange> {
  handle: AtomicU32, // 初值 kNullIndirectPointerHandle; 非 sandbox 构建为 TaggedMember<T> (登记差异)
  _ty: PhantomData<T>,
}
// 方法全集 (load/load_maybe_empty/store/store_no_write_barrier/acquire_load/
// acquire_load_maybe_empty/release_store/release_store_no_write_barrier/is_empty/
// is_unpublished/clear/storage_address) + 4 AllocationWitness ctor (推迟登记)
// CodePointerMember = TrustedPointerMember<Code, {CODE}> — Code 属 codegen crate, 别名推迟
`;

const p = writeSrc("v8-objects-arrays", "trusted_pointer_h.rs.skel", skel);
console.log(`trusted-pointer.h.js: 骨架 ${p} (对数断言 ${checks} 项通过; X-macro 14 项逐值比对, member 方法 ${12} 族)`);
