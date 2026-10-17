#!/usr/bin/env bun
// tool/src/compiler/turboshaft/string-view.h.js
// 机械提取 v8/src/compiler/turboshaft/string-view.h
// → v8-turboshaft/src/string_view.rs.skel
//
// 提取面: StringView ForeachIterable 概念面 (Begin/IsEnd/Advance/
// Dereference); 编码步长规则。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/string-view.h");

// 1. 哨兵
for (const [re, name] of [
  [/class StringView \{/, "StringView 类"],
  [/using value_type = V<Word32>;/, "value_type"],
  [/using iterator_type = V<WordPtr>;/, "iterator_type"],
  [/iterator_type Begin\(A& assembler\)/, "Begin"],
  [/OptionalV<Word32> IsEnd\(A& assembler/, "IsEnd"],
  [/iterator_type Advance\(A& assembler/, "Advance"],
  [/value_type Dereference\(A& assembler/, "Dereference"],
  [/OFFSET_OF_DATA_START\(SeqOneByteString\) ==\s*\n?\s*OFFSET_OF_DATA_START\(SeqTwoByteString\)/, "数据偏移一致"],
  [/String::ONE_BYTE_ENCODING \? 1 : 2/, "步长规则"],
  [/can_rely_on_no_gc_ == nullptr/, "无 GC 保证分支"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. 步长与表示规则
const strides = [...h.matchAll(/ONE_BYTE_ENCODING \? (\d+) : (\d+)/g)].map((m) => `${m[1]}/${m[2]}`);
const reps = [...h.matchAll(/MemoryRepresentation::Uint(\d+)\(\)/g)].map((m) => +m[1]);

let rust = banner(
  "src/compiler/turboshaft/string-view.h.js",
  "src/compiler/turboshaft/string-view.h",
  "字符串视图: 逐字符迭代 (ForeachIterable)",
);

rust += `
/// 原版 StringView — 实现 ForeachIterable 概念, 迭代字符串字符。
/// 前提: SeqOneByteString 与 SeqTwoByteString 的数据起始偏移相等
/// (static_assert), 步长 1/2 按编码; 无 GC 保证时原版 UNIMPLEMENTED
/// (can_rely_on_no_gc_ == nullptr 分支)。
/// v8: src/compiler/turboshaft/string-view.h StringView
pub struct StringView {
  pub string: V<String>,
  pub encoding: StringEncoding,
  pub start_index: ConstOrV<WordPtr>,
  /// character_count 未给且非常量时原版 UNIMPLEMENTED (TODO 从 string 读)
  pub character_count: ConstOrV<WordPtr>,
  pub end_offset: V<WordPtr>,
}

impl StringView {
  /// v8: Begin — begin = BitcastTaggedToWordPtr(string) +
  ///   (data_offset - kHeapObjectTag) + start_index * stride
  /// end_offset_ = begin + count * stride
  pub fn begin<A: Assembler>(&mut self, asm: &mut A) -> V<WordPtr> { todo!() }
  /// v8: IsEnd — UintPtrLessThanOrEqual(end_offset_, it)
  pub fn is_end<A: Assembler>(&self, asm: &mut A, it: V<WordPtr>) -> OptionV<Word32> { todo!() }
  /// v8: Advance — it + stride (one-byte=1 / two-byte=2)
  pub fn advance<A: Assembler>(&self, asm: &mut A, it: V<WordPtr>) -> V<WordPtr> { todo!() }
  /// v8: Dereference — Load RawAligned, Uint8/Uint16 (${[...new Set(reps)].join("/")} 按编码)
  pub fn dereference<A: Assembler>(&self, asm: &mut A, it: V<WordPtr>) -> V<Word32> { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "string_view.rs.skel", rust);
console.log(`written ${p}`);
console.log(`步长规则=${strides.length} 加载表示=${[...new Set(reps)].join(",")}`);
