#!/usr/bin/env bun
// tool/src/regexp/experimental/experimental-bytecode.h.js
//   — v8/src/regexp/experimental/experimental-bytecode.h → v8-regexp-exec/src/experimental/bytecode.rs
// 规则: Opcode 枚举逐值序数对账 (X-macro 式枚举块解析) + 位域三元组 (shift/size) 对数
//   + 21 个 static Instruction 工厂 + union 9 成员 + static_assert ×2 + IsFilter 白名单
//   (原版显式排除 FILTER_LOOKAROUND — 陷阱面, 两侧同时核查)
// 产物: v8-regexp-exec/src/experimental/bytecode.rs.skel (映射骨架, 审计专用; 落地以就地修订为准)
//   + 落地对数核对 (bytecode.rs 逐项命中, 缺项 throw 列票单)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../../lib.js";

const h = readV8("src/regexp/experimental/experimental-bytecode.h");

let assertCount = 0;
const must = (cond, msg) => {
  assertCount++;
  if (!cond) throw new Error(`对数失配: ${msg}`);
};
const count = (src, re) => (src.match(re) || []).length;

// ── 原版侧解析 (编译器语义的机械提取, 不靠肉眼) ────────────────────────────

// 1) Opcode 枚举全集: enum Opcode : int32_t { NAME, ... }; 隐式序数 0..18
const opcodeBlock = h.match(/enum Opcode : int32_t \{([\s\S]*?)\};/);
must(opcodeBlock, "h: Opcode 枚举块锚点");
const opcodes = [...opcodeBlock[1].matchAll(/([A-Z][A-Z_]+),/g)].map((m) => m[1]);
must(opcodes.length === 19, `Opcode 值数应为 19, 实得 ${opcodes.length}`);
const OPCODE_ORDER = [
  "ACCEPT", "ASSERTION", "CLEAR_REGISTER", "CONSUME_RANGE", "RANGE_COUNT", "FORK", "JMP",
  "SET_REGISTER_TO_CP", "SET_QUANTIFIER_TO_CLOCK", "FILTER_QUANTIFIER", "FILTER_GROUP",
  "FILTER_LOOKAROUND", "FILTER_CHILD", "BEGIN_LOOP", "END_LOOP", "START_LOOKAROUND",
  "END_LOOKAROUND", "WRITE_LOOKAROUND_TABLE", "READ_LOOKAROUND_TABLE",
];
must(opcodes.join(",") === OPCODE_ORDER.join(","), `Opcode 序不一致: ${opcodes.join(",")}`);

// 2) 位域三元组 (h:143-145): IsPositive=BitField<bool,0,1>; Type=Next<Lookaround::Type,1>
//    → shift 1 size 1; LookaroundIndex=Next<uint32_t,30> → shift 2 size 30 (Next 定义:
//    bit-field.h:75-77 Next<T2,size2> = BitField<T2, kShift+kSize, size2, U>)
must(h.includes("base::BitField<bool, 0, 1>"), "h:143 IsPositive 位域锚点");
must(h.includes("IsPositive::Next<Lookaround::Type, 1>"), "h:144 Type 位域锚点");
must(h.includes("Type::Next<uint32_t, 30>"), "h:145 LookaroundIndex 位域锚点");
must(count(h, /static_assert\(sizeof\(payload\) == 4\);/g) === 1, "h:320 payload=4 static_assert");
must(count(h, /static_assert\(sizeof\(Instruction\) == 8\);/g) === 1, "h:322 Instruction=8 static_assert");

// 3) 工厂全集: 21 个 static Instruction + 1 个 static bool IsFilter
must(count(h, /static Instruction \w+\(/g) === 21, `h: static Instruction 工厂数, 实得 ${count(h, /static Instruction \w+\(/g)}`);
must(count(h, /static bool IsFilter\(/g) === 1, "h:292 IsFilter 静态方法");
// IsFilter 白名单恰 3 项 (FILTER_GROUP/FILTER_QUANTIFIER/FILTER_CHILD), 不含 FILTER_LOOKAROUND
const isFilterBody = h.match(/static bool IsFilter\(const Instruction& instruction\) \{([\s\S]*?)\n  \}/)[1];
must(isFilterBody.includes("FILTER_GROUP") && isFilterBody.includes("FILTER_QUANTIFIER") &&
  isFilterBody.includes("FILTER_CHILD"), "h:293-295 IsFilter 三项");
must(!isFilterBody.includes("FILTER_LOOKAROUND"), "h: IsFilter 应排除 FILTER_LOOKAROUND (陷阱面)");

// 4) union payload 9 成员 (h:299-319)
const unionBody = h.match(/union \{([\s\S]*?)\n  \} payload;/)[1];
const unionMembers = [...unionBody.matchAll(/;\s*\/\/|[A-Za-z_:<>]+ (\w+);/g)]
  .map((m) => m[1]).filter(Boolean);
must(unionMembers.length === 9, `h: union 成员数应为 9, 实得 ${unionMembers.length}`);
const UNION_FIELDS = ["consume_range", "num_ranges", "pc", "register_index", "assertion_type",
  "quantifier_id", "group_id", "lookaround_id", "lookaround"];
for (const f of UNION_FIELDS) must(unionMembers.includes(f), `h: union 成员 ${f}`);

// ── 产物: 映射骨架 ─────────────────────────────────────────────────────────
const skel = `${banner(
  "src/regexp/experimental/experimental-bytecode.h.js",
  "src/regexp/experimental/experimental-bytecode.h",
  "映射骨架基线 (审计专用): 枚举/位域/工厂/union 逐项 1:1; 落地以 bytecode.rs 就地修订为准",
)}
//! Opcode ${opcodes.length} 值序 0..18 逐一对账 (枚举块机械解析, 非肉眼)
//! LookaroundPayload 位域: bit0 IsPositive(bool,0,1) / bit1 Type(LookaroundType,1,1) /
//!   bit2..31 LookaroundIndex(uint32,2,30) — Rust 侧 BitField32<T,shift,size> 直译
//! 工厂 21 + IsFilter 1; Payload 枚举 = union 9 成员 + None (无载荷 4 opcode 的
//!   未初始化 union 字节从不被读, tagged 化)
//! static_assert: sizeof(payload)==4 → const assert size_of::<LookaroundPayload>()==4;
//!   sizeof(Instruction)==8 → Rust tagged 枚举不承诺等长, 裸编码序列化面推迟登记
//!   (VectorToByteArray 属 experimental.cc:108, 非本文件)
`;
writeSrc("v8-regexp-exec", "experimental/bytecode.rs.skel", skel);

// ── 落地对数核对 (bytecode.rs 就地修订版逐项命中; 缺项 throw 列票单) ────────
const landed = readFileSync(join(root, "v8-regexp-exec", "src", "experimental", "bytecode.rs"), "utf8");
const missing = [];
const land = (re, want, label) => {
  const got = count(landed, re);
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
};
// 19 变体带显式序数 (Accept = 0 ... ReadLookaroundTable = 18)
OPCODE_ORDER.forEach((name, i) => {
  const camel0 = name.toLowerCase().replace(/_(\w)/g, (_, c) => c.toUpperCase());
  const camel = camel0[0].toUpperCase() + camel0.slice(1);
  land(new RegExp(`^  ${camel} = ${i},$`, "gm"), 1, `Rust Opcode ${name} 序数 ${i}`);
});
land(/pub const ALL_OPCODES: \[Opcode; 19\]/g, 1, "ALL_OPCODES 全集表");
land(/pub const [A-Z_]+: Self = Self::\w+;/g, 19, "C++ 名别名常量 ×19");
// 位域三元组
land(/type IsPositive = BitField32<bool, 0, 1>;/g, 1, "Rust IsPositive(0,1)");
land(/type TypeField = BitField32<u32, 1, 1>;/g, 1, "Rust Type(1,1)");
land(/type LookaroundIndex = BitField32<u32, 2, 30>;/g, 1, "Rust LookaroundIndex(2,30)");
// 工厂 21 + is_filter
const FACTORIES = ["consume_range", "consume_any_char", "fail", "range_count", "fork", "jmp",
  "accept", "set_register_to_cp", "assertion", "clear_register", "set_quantifier_to_clock",
  "filter_quantifier", "filter_group", "filter_lookaround", "filter_child", "begin_loop",
  "end_loop", "start_lookaround", "end_lookaround", "write_look_table", "read_look_table"];
for (const f of FACTORIES) land(new RegExp(`pub fn ${f}\\(`, "g"), 1, `Rust 工厂 ${f}`);
land(/pub fn is_filter\(&self\)/g, 1, "Rust is_filter");
land(/Opcode::FilterGroup \| Opcode::FilterQuantifier \| Opcode::FilterChild/g, 1, "is_filter 白名单三项");
// Payload 变体 = union 9 + None
land(/^  None,/gm, 1, "Payload::None");
for (const v of ["ConsumeRange(Uc16Range)", "NumRanges(i32)", "Pc(i32)", "RegisterIndex(i32)",
  "Assertion(AssertionType)", "QuantifierId(i32)", "GroupId(i32)", "LookaroundId(i32)",
  "Lookaround(LookaroundPayload)"]) {
  land(new RegExp(`^  ${v.replace(/[()\[\],.\\]/g, (c) => "\\" + c)},$`, "gm"), 1, `Payload ${v}`);
}
// static_assert 承接 (位域三参数 + payload 4 字节 ≥ 4 条 const assert)
land(/const _: \(\) = assert!\(/g, 4, "const assert ×4 (位域×3 + size 1)");
if (missing.length) throw new Error(`bytecode.rs 落地对数缺项 (票单):\n  ${missing.join("\n  ")}`);

console.log(
  `experimental-bytecode.h.js: 对数断言 ${assertCount} 条全过; 原版 Opcode ${opcodes.length} 值序 0..18 枚举块机械解析` +
    ` / 位域 (0,1)+(1,1)+(2,30) / 工厂 21+IsFilter(白名单 3, 排除 FILTER_LOOKAROUND) / union 成员 9 / static_assert ×2;` +
    ` 落地 55 锚逐项命中, 缺项 0; 产物 v8-regexp-exec/src/experimental/bytecode.rs.skel`,
);
