#!/usr/bin/env bun
// tool/src/compiler/turboshaft/type-parser.h.js
// 机械提取 v8/src/compiler/turboshaft/type-parser.{h,cc}
// → v8-turboshaft/src/type_parser.rs.skel
//
// 提取面: Type 文法 (前缀 / Set / Range); 数值解析类型表; 约束锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/type-parser.h");
const cc = readV8("src/compiler/turboshaft/type-parser.cc");

// 1. cc 文法前缀
const prefixes = [...cc.matchAll(/ConsumeIf\("(\w+)"\)/g)].map((m) => m[1]);
const uniquePrefixes = [...new Set(prefixes)];
if (uniquePrefixes.length !== 4) throw new Error(`类型前缀 ${uniquePrefixes.length} != 4`);

// 2. 哨兵
for (const [re, name] of [
  [/class TypeParser \{/, "解析器类"],
  [/std::optional<Type> Parse\(\)/, "Parse 入口"],
  [/std::optional<Type> ParseType\(\);/, "ParseType 声明"],
  [/std::optional<T> ParseRange\(\)/, "ParseRange"],
  [/std::optional<T> ParseSet\(\)/, "ParseSet"],
  [/CHECK_LE\(elements->size\(\), T::kMaxSetSize\)/, "集合大小约束"],
  [/base::sort\(elements\)/, "集合排序去重"],
  [/std::stoul|std::stoull|std::stof|std::stod/, "数值解析"],
  [/Skip trailing whitespace/, "尾随空白"],
]) {
  if (!(re.test(h) || re.test(cc))) throw new Error(`哨兵缺失: ${name}`);
}

// 3. 数值类型分派
const numeric = [...h.matchAll(/std::is_same_v<T, (\w+)>[\s\S]{0,120}?std::(sto\w+)\(/g)].map((m) => `${m[1]}->${m[2]}`);
if (numeric.length !== 4) throw new Error(`数值分派 ${numeric.length} != 4`);

let rust = banner(
  "src/compiler/turboshaft/type-parser.h.js",
  "src/compiler/turboshaft/type-parser.{h,cc}",
  "类型文法解析: 字符串 -> Type (%CheckTurboshaftTypeOf)",
);

rust += `
/// 原版 TypeParser — 解析 Type::ToString() 逆文法:
///   Type := ${uniquePrefixes.join(" | ")} [ Set | Range ]
///   Set  := "{" v ("," v)* "}"   (升序去重, 1..kMaxSetSize)
///   Range:= "[" v "," v "]"      (非 Word 类型须 from <= to)
/// v 的数值文法按目标类型: ${numeric.join(", ")}
/// v8: src/compiler/turboshaft/type-parser.{h,cc}
pub struct TypeParser<'s> {
  pub s: &'s str,
  pub pos: usize,
}

impl<'s> TypeParser<'s> {
  /// v8: Parse — 跳尾随空白后必须到串尾
  pub fn parse(&mut self) -> Option<Type> { todo!() }
  /// v8: cc ParseType — 4 前缀分派:
${uniquePrefixes.map((p) => `  ///   "${p}" -> ${p}Type (Float32 任意态原版误返 Float64Type::Any)`).join("\n")}
  fn parse_type(&mut self) -> Option<Type> { todo!() }
  /// v8: ParseRange<T> — "[" from "," to "]", Word 可回绕不检序
  fn parse_range<T: FromStr>(&mut self) -> Option<T> { todo!() }
  /// v8: ParseSet<T> — sort + unique, CHECK 1 <= n <= kMaxSetSize
  fn parse_set<T: Ord>(&mut self) -> Option<Vec<T>> { todo!() }
  /// v8: ConsumeIf / IsNext — 前缀消费 (跳前导空白)
  fn consume_if(&mut self, prefix: &str) -> bool { todo!() }
  /// v8: ReadValue<T> — 按类型 sto* 解析, read==0 为失败
  fn read_value<T: FromStr>(&mut self) -> Option<T> { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "type_parser.rs.skel", rust);
console.log(`written ${p}`);
console.log(`前缀=${uniquePrefixes.length} 数值分派=${numeric.length}`);
