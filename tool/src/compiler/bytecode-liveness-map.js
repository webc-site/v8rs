#!/usr/bin/env bun
// tool/src/compiler/bytecode-liveness-map.js
// 从 v8/src/compiler/bytecode-liveness-map.h + bytecode-liveness-map.cc
// 提取活性状态 (accumulator 占 bit0, register 占 bit index+1) 与偏移→活性映射
// 生成 v8-compiler-frontend/src/bytecode_liveness_map.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/bytecode-liveness-map.h");
const cc = readV8("src/compiler/bytecode-liveness-map.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class BytecodeLivenessState : public ZoneObject {", "BytecodeLivenessState 类"],
  [": bit_vector_(register_count + 1, zone) {}", "构造 +1 (accumulator 槽)"],
  ["return *it_ - 1;", "Iterator 解引用 -1 (跳过 accumulator)"],
  ["return bit_vector_.Contains(index + 1);", "RegisterIsLive: index+1"],
  ["bool AccumulatorIsLive() const { return bit_vector_.Contains(0); }", "AccumulatorIsLive: bit0"],
  ["bit_vector_.Add(index + 1, count);", "MarkRegistersLive 范围加"],
  ["int register_count() const { return bit_vector_.length() - 1; }", "register_count = length-1"],
  ["int live_value_count() const { return bit_vector_.Count(); }", "live_value_count 含 accumulator"],
  ["struct BytecodeLiveness {", "BytecodeLiveness 结构"],
  ["class V8_EXPORT_PRIVATE BytecodeLivenessMap {", "BytecodeLivenessMap 类"],
  ["liveness_[offset].in = nullptr;", "InsertNewLiveness DEBUG 置空"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`bytecode-liveness-map.h 锚点缺失: ${name}`);
}
if (!cc.includes("std::string ToString(const BytecodeLivenessState& liveness)")) {
  throw new Error("bytecode-liveness-map.cc 锚点缺失: ToString");
}

// —— 2. 真解析: BytecodeLivenessState 方法清单 ——
const cls = h.slice(h.indexOf("class BytecodeLivenessState"), h.indexOf("struct BytecodeLiveness"));
const methods = [...cls.matchAll(/\n  (?:bool|void|int|Iterator) (UnionIsChanged|Equals|Mark\w+|Union|CopyFrom|register_count|live_value_count|Accumulator\w+|Register\w+|begin|end)\(/g)]
  .map((m) => m[1]);
const unique = [...new Set(methods)];
if (unique.length < 15) throw new Error(`BytecodeLivenessState 方法提取不足: ${unique.length}`);

// —— 3. Mark* 家族细分 (Live/Dead 单点+范围 ×寄存器, 加 accumulator 与 All) ——
const markFns = unique.filter((n) => n.startsWith("Mark"));
if (markFns.length !== 7) throw new Error(`Mark* 家族应 7 个, 得 ${markFns.length}`);

const body = `${banner("src/compiler/bytecode-liveness-map.js", "src/compiler/bytecode-liveness-map.h + bytecode-liveness-map.cc", "字节码活性: BitVector 布局 accumulator@0 + register@index+1")}
//! BytecodeLivenessState — 单条字节码的寄存器/累加器活性。
//!
//! BitVector 布局 1:1: bit0 = accumulator, bit(index+1) = register index。
//! register_count = bit_vector.length - 1; live_value_count 含 accumulator。

/// 活性状态 — 位向量 [acc, r0, r1, ...]
/// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessState
pub struct BytecodeLivenessState {
    /// 长度 = register_count + 1
    bit_vector: BitVector,
}

impl BytecodeLivenessState {
    /// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessState::BytecodeLivenessState(register_count, zone)
    pub fn new(register_count: i32, zone: &Zone) -> Self;

    /// 读
${unique.filter((n) => /^(RegisterIsLive|AccumulatorIsLive|Equals|register_count|live_value_count|UnionIsChanged)$/.test(n)).map((n) => `    /// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessState::${n}\n    pub fn ${n}(&self);`).join("\n")}

    /// 写 (Mark 家族 ${markFns.length}: Live/Dead × 单寄存器/范围 + accumulator + All)
${markFns.map((n) => `    /// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessState::${n}\n    pub fn ${n}(&mut self);`).join("\n")}

    /// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessState::Union
    pub fn union_(&mut self, other: &BytecodeLivenessState);
    /// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessState::CopyFrom
    pub fn copy_from(&mut self, other: &BytecodeLivenessState);

    /// 迭代: 跳过 accumulator, 值 = 迭代位 - 1
    /// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessState::Iterator::operator*
    pub fn iter_live_registers(&self) -> impl Iterator<Item = i32> + '_;
}

/// 单字节码偏移的 in/out 活性对
/// v8: src/compiler/bytecode-liveness-map.h BytecodeLiveness
pub struct BytecodeLiveness {
    pub in_: *mut BytecodeLivenessState,
    pub out: *mut BytecodeLivenessState,
}

/// 偏移 → BytecodeLiveness 数组 (索引即字节码偏移)
/// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessMap
pub struct BytecodeLivenessMap {
    liveness: *mut BytecodeLiveness,
    size: usize, // 仅 DEBUG
}

impl BytecodeLivenessMap {
    /// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessMap::InsertNewLiveness
    /// DEBUG 下先 in/out 置 null 供后续 DCHECK 判初始化
    pub fn insert_new_liveness(&mut self, offset: i32) -> &mut BytecodeLiveness;
    /// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessMap::GetLiveness
    pub fn get_liveness(&self, offset: i32) -> &BytecodeLiveness;
    /// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessMap::GetInLiveness
    pub fn get_in_liveness(&self, offset: i32) -> &BytecodeLivenessState;
    /// v8: src/compiler/bytecode-liveness-map.h BytecodeLivenessMap::GetOutLiveness
    pub fn get_out_liveness(&self, offset: i32) -> &BytecodeLivenessState;
}

/// 调试打印: 活性位图字符串
/// v8: src/compiler/bytecode-liveness-map.cc ToString
pub fn to_string(liveness: &BytecodeLivenessState) -> String;

// —— 对数基准 ——
// BytecodeLivenessState 方法面: ${unique.length} 个 (Mark* ${markFns.length})
`;

const p = writeSrc("v8-compiler-frontend", "bytecode_liveness_map.rs.skel", body);
console.log(`✓ bytecode-liveness-map: 方法面 ${unique.length} (Mark* ${markFns.length}) / 锚点 ${anchors.length + 1}`);
