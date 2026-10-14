#!/usr/bin/env bun
// tool/src/regexp/experimental/experimental-bytecode.cc.js
//   — v8/src/regexp/experimental/experimental-bytecode.cc → v8-regexp-exec/src/experimental/bytecode.rs (Dump 层)
// 规则: 三个 operator<< 的输出串全集机械提取 (19 opcode 标签 + ASSERTION 子表 7 项,
//   其中 END_OF_BUFFER 为 UNREACHABLE 不产串) + PrintAsciiOrHex 分支语义
//   (isprint 前置 + std::hex 粘性) + DigitsRequiredBelow 循环界 + setfill/setw/endl 三件套
//   + LookaroundPayload 打印 4 段格式
// 产物: 骨架追加至 bytecode.cc.rs.skel + 落地对数核对 (bytecode.rs 19 标签全命中;
//   测试文件 sticky-hex 金标准向量逐字命中 — Dump 是金标准面, 断言强度不降级)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../../lib.js";

const cc = readV8("src/regexp/experimental/experimental-bytecode.cc");

let assertCount = 0;
const must = (cond, msg) => {
  assertCount++;
  if (!cond) throw new Error(`对数失配: ${msg}`);
};
const count = (src, re) => (src.match(re) || []).length;

// ── 原版侧解析 ─────────────────────────────────────────────────────────────

// 1) 顶层 switch 19 case (operator<< Instruction, cc:27-118)
const caseLabels = [...cc.matchAll(/case Instruction::([A-Z_]+):/g)].map((m) => m[1]);
const topCases = [...new Set(caseLabels)];
must(topCases.length === 19, `顶层 case 应 19, 实得 ${topCases.length}: ${topCases.join(",")}`);

// 2) ASSERTION 子表 7 case (cc:43-66); END_OF_BUFFER → UNREACHABLE() 是唯一不出串的
const assertionCases = [...cc.matchAll(/case Assertion::Type::([A-Z_]+):/g)].map((m) => m[1]);
must(assertionCases.length === 7, `ASSERTION 子表应 7, 实得 ${assertionCases.length}`);
const eobIdx = assertionCases.indexOf("END_OF_BUFFER");
must(eobIdx >= 0 && cc.split("END_OF_BUFFER:")[1].startsWith("\n          //"), "END_OF_BUFFER case 锚点");
must(count(cc, /UNREACHABLE\(\);/g) === 1, "UNREACHABLE 恰 1 处 (END_OF_BUFFER)");

// 3) PrintAsciiOrHex (cc:16-23): isprint 前置判定 + "0x" + std::hex 粘性 (std::hex 从不复原)
must(cc.includes("if (c < 128 && std::isprint(c))"), "PrintAsciiOrHex 判定式");
must(cc.includes('os << "0x" << std::hex'), "PrintAsciiOrHex hex 分支 (粘性源头)");
must(count(cc, /std::dec/g) === 0, "原版 0 处 std::dec 复原 — hex 粘性成立 (金标准关键)");

// 4) DigitsRequiredBelow (cc:124-132) 与向量打印 (cc:136-147)
must(cc.includes("int result = 1;"), "DigitsRequiredBelow 初值 1");
must(cc.includes("for (int i = 10; i < n; i *= 10)"), "DigitsRequiredBelow 循环界");
must(cc.includes("std::setfill('0') << std::setw(line_digit_num)"), "行号 setfill/setw");
must(count(cc, /std::endl/g) === 1, "行尾 std::endl ×1");

// 5) LookaroundPayload 打印 (cc:149-155): `<index> (ahead|behind, positive|negative)`
for (const s of ['<< " ("', '"ahead"', '"behind"', '"positive"', '"negative"', '<< ")"']) {
  must(cc.includes(s), `LookaroundPayload 打印段 ${s}`);
}

// ── 产物: Dump 层骨架 ──────────────────────────────────────────────────────
const skel = `${banner(
  "src/regexp/experimental/experimental-bytecode.cc.js",
  "src/regexp/experimental/experimental-bytecode.cc",
  "Dump 层骨架基线 (审计专用): 与 h 脚本同产物族, 落地以 bytecode.rs 就地修订为准",
)}
//! Ostream 状态机: basefield(decimal/hex 粘性) + fill + width(一次性), 逐插入项复刻
//! 19 opcode 标签串 (hex 触发后同流数值转小写 hex — 原版真实行为)
//! ASSERTION 子表 6 出串 + END_OF_BUFFER unreachable! (原版 UNREACHABLE)
//! dump(&[Instruction]) → String: 行号 setfill('0') setw(digits_required_below(len)) ": "
//!   inst "\\n"; Display(Instruction) = 全新流 (十进制起步)
`;
writeSrc("v8-regexp-exec", "experimental/bytecode.cc.rs.skel", skel);

// ── 落地对数核对 ───────────────────────────────────────────────────────────
const landed = readFileSync(join(root, "v8-regexp-exec", "src", "experimental", "bytecode.rs"), "utf8");
const missing = [];
const land = (re, want, label) => {
  const got = count(landed, re);
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
};
// 19 opcode 打印标签逐串命中 (ACCEPT 等 4 个无载荷指令无尾空格, 其余带 " " 或 "[")
const LABELS = [
  "CONSUME_RANGE [", "RANGE_COUNT", "ASSERTION", "FORK", "JMP", "ACCEPT",
  "SET_REGISTER_TO_CP", "CLEAR_REGISTER", "SET_QUANTIFIER_TO_CLOCK", "FILTER_QUANTIFIER",
  "FILTER_GROUP", "FILTER_LOOKAROUND", "FILTER_CHILD", "BEGIN_LOOP", "END_LOOP",
  "START_LOOKAROUND", "END_LOOKAROUND", "WRITE_LOOKAROUND_TABLE", "READ_LOOKAROUND_TABLE",
];
const esc = (s) => s.replace(/[[\]().*+?^$|\\]/g, (x) => "\\" + x);
for (const label of LABELS) {
  land(new RegExp(`self\\.text\\("${esc(label)}[ \\[]?"\\)`), 1, `Rust 打印标签 ${label}`);
}
// ASSERTION 子表: 6 串 + END_OF_BUFFER 走 unreachable!
for (const n of ["START_OF_INPUT", "END_OF_INPUT", "START_OF_LINE", "END_OF_LINE", "BOUNDARY", "NON_BOUNDARY"]) {
  land(new RegExp(`"${n}"`), 1, `ASSERTION 子串 ${n}`);
}
land(/AssertionType::EndOfBuffer => \{\s*\n\s*unreachable!/g, 1, "END_OF_BUFFER → unreachable! (原版 UNREACHABLE)");
// Ostream 状态机三要素 + hex 粘性注释锚
land(/hex: bool,/g, 1, "basefield 粘性字段");
land(/fn print_ascii_or_hex/g, 1, "PrintAsciiOrHex");
land(/fn digits_required_below/g, 1, "DigitsRequiredBelow");
land(/os\.fill = '0';/g, 1, "setfill('0')");
land(/os\.width = line_digit_num;/g, 1, "setw(行宽)");
// 金标准测试向量逐字命中 (cctest 式逐字符对账承接面)
const tests = readFileSync(join(root, "v8-regexp-exec", "tests", "experimental_bytecode_unittest.rs"), "utf8");
const GOLDENS = [
  ["CONSUME_RANGE [0x0, 0xffff]", "consume_any_char 十六进制"],
  ["CONSUME_RANGE [0xffff, 0x0]", "fail 空区间"],
  ["CONSUME_RANGE [ , ~]", "isprint 边界 0x20/0x7E"],
  ["1: RANGE_COUNT 11", "hex 粘性: 17 → 11"],
  ["2: FORK f", "hex 粘性: 15 → f"],
  ["0a: ACCEPT", "hex 粘性 + setw(2) + setfill: 行号 10 → 0a"],
  ["1: READ_LOOKAROUND_TABLE c (ahead, positive)", "hex 粘性穿透位域 index"],
  ["5 (behind, positive)", "LookaroundPayload Display"],
];
for (const [g, label] of GOLDENS) {
  const c = count(tests, new RegExp(g.replace(/[[\]().*+?^$|\\]/g, (x) => "\\" + x), "g"));
  if (c < 1) missing.push(`测试金标准 ${label}: got=${c} want>=1`);
  assertCount++;
}
if (missing.length) throw new Error(`bytecode.rs/tests 落地对数缺项 (票单):\n  ${missing.join("\n  ")}`);

console.log(
  `experimental-bytecode.cc.js: 对数断言 ${assertCount} 条全过; 原版顶层 case ×${topCases.length}` +
    ` / ASSERTION 子表 7 (END_OF_BUFFER=UNREACHABLE 唯一不出串) / std::dec 复原 0 处 (hex 粘性成立)` +
    ` / setfill+setw+endl 三件套 / LookaroundPayload 4 段格式; 落地 19 标签 + 6 子串 + 6 状态机锚` +
    ` + ${GOLDENS.length} 金标准向量逐项命中, 缺项 0; 产物 v8-regexp-exec/src/regexp/experimental/experimental-bytecode.cc.rs.skel`,
);
