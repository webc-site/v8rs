#!/usr/bin/env bun
// tool/src/wasm/wasm-disassembler.cc.js
// 机械提取 v8/src/wasm/wasm-disassembler.cc 的默认对齐表 (GetDefaultAlignment)、
// 十六进制字符表、原始字符串转义与签名打印格式。
// 常量骨架在 v8-wasm-core/src/wasm_disassembler_consts.rs (wasm-disassembler.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/wasm-disassembler.cc");

// ---------- 1. 十六进制字符表 ----------
if (!src.includes('static constexpr char kHexChars[] = "0123456789abcdef";')) {
  throw new Error("小写十六进制表缺失");
}
if (!src.includes('static constexpr char kUpperHexChars[] = "0123456789ABCDEF";')) {
  throw new Error("大写十六进制表缺失");
}

// ---------- 2. 默认对齐表: log2 对齐, 按访问宽分组 (机器提取各组 case 计数) ----------
// 提取 GetDefaultAlignment switch 的返回值组
const fnBlock = src.match(/uint32_t GetDefaultAlignment\(WasmOpcode opcode\) \{([\s\S]*?)\n\}/);
if (!fnBlock) throw new Error("GetDefaultAlignment 缺失");
const body = fnBlock[1];
const retGroups = [...body.matchAll(/return (\d);/g)].map((m) => Number(m[1]));
if (retGroups.join(",") !== "4,3,2,1,0") throw new Error(`对齐组序异常: ${retGroups.join(",")}`);
// 各组操作码计数
const groupSizes = [];
const cases = body.split(/return \d;/).slice(0, 5).map((seg) => (seg.match(/case kExpr\w+:/g) || []).length);
if (JSON.stringify(cases) !== JSON.stringify([2, 14, 11, 11, 9])) {
  throw new Error(`各组操作码数异常: ${cases.join(",")}`);
}
// 原子操作走 MachineType 派生
if (!body.includes("ATOMIC_OP_LIST(CASE)")) throw new Error("原子操作派生缺失");
if (!body.includes("ATOMIC_STORE_OP_LIST(CASE)")) throw new Error("原子存操作派生缺失");

// ---------- 3. 原始字符串转义 (PrintStringRaw) ----------
if (!src.includes("if (b < 32 || b >= 127 || b == '\"' || b == '\\\\') {")) {
  throw new Error("字符串转义条件缺失");
}
if (!/out << '\\\\' << kHexChars\[b >> 4\] << kHexChars\[b & 0xF\];/.test(src)) {
  throw new Error("转义十六进制格式缺失");
}

// ---------- 4. 签名单行打印格式 ----------
if (!src.includes('out << " (param ";')) throw new Error("带名参数格式缺失");
if (!src.includes('out << " (param";')) throw new Error("匿名参数格式缺失");
if (!src.includes('out << " (result ";')) throw new Error("结果格式缺失");

// ---------- 5. 指令名打印 (空格 + 括号语义组) ----------
if (!src.includes('out << " (" << WasmOpcodes::OpcodeName(opcode);')) {
  throw new Error("指令语义组格式缺失");
}

console.log(`wasm-disassembler: 对齐组=${retGroups.length} 各组操作码=${cases.join("/")}`);

// ---------- 生成 Rust ----------
const ALIGN_DESC = [
  "S128 整体访问 (kS128LoadMem/kS128StoreMem), 16 字节",
  "64 位标量与 S128 半宽/64 位 lane 访问, 8 字节",
  "32 位标量与 S128 32 位 lane 访问, 4 字节",
  "16 位标量、F16 与 S128 16 位 lane 访问, 2 字节",
  "8 位标量与 S128 8 位 lane 访问, 1 字节",
];
let alignTable = "";
for (let i = 0; i < retGroups.length; i++) {
  alignTable += `    (${retGroups[i]}, ${cases[i]}, "${ALIGN_DESC[i]}"),\n`;
}

const rust = `${banner(
  "src/wasm/wasm-disassembler.cc.js",
  "src/wasm/wasm-disassembler.cc",
  "默认对齐表与 WAT 打印格式 (骨架在 wasm_disassembler_consts.rs)",
)}
//! WAT 文本打印规则。

/// 十六进制字符表 (小写用于字节转义, 大写用于立即数)
/// v8: src/wasm/wasm-disassembler.cc kHexChars / kUpperHexChars
pub const HEX_CHARS_LOWER: &str = "0123456789abcdef";
pub const HEX_CHARS_UPPER: &str = "0123456789ABCDEF";

/// 默认对齐表: (log2 对齐, 组内操作码数, 说明)
/// 对齐值即 .wasm 二进制里的 alignment 字段 (4 => 1<<4 == 16 字节);
/// 原子操作的对齐由 MachineType 表示宽派生 (ATOMIC_OP_LIST/ATOMIC_STORE_OP_LIST)
/// v8: src/wasm/wasm-disassembler.cc GetDefaultAlignment
#[rustfmt::skip]
pub static DEFAULT_ALIGNMENT_GROUPS: &[(u32, usize, &str)] = &[
${alignTable}];

/// 原始字符串转义: 控制字符/非 ASCII/引号/反斜杠 -> "\\xx" (小写十六进制)
/// v8: src/wasm/wasm-disassembler.cc PrintStringRaw
pub const STRING_ESCAPE_RULE: &str = "b < 32 || b >= 127 || b == '\\"' || b == '\\\\' -> \\\\xx";

/// 签名单行格式: 带名参数逐个 " (param <名> <类型>)"; 匿名合并 " (param <类型>...)";
/// 每个返回值单独 " (result <类型>)"
/// v8: src/wasm/wasm-disassembler.cc PrintSignatureOneLine
#[rustfmt::skip]
pub static SIGNATURE_FORMATS: &[&str] = &[
    " (param <name> <type>)  — 参数带名",
    " (param <type> ...)     — 参数匿名合并",
    " (result <type>)        — 每返回值一组",
];

/// 语义组指令格式: " (<name>" 打开折叠组
/// v8: src/wasm/wasm-disassembler.cc FunctionBodyDisassembler
pub const OPCODE_GROUP_FORMAT: &str = " (<opcode-name>";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS.len(), 5);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[0].0, 4);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[0].1, 2);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[1].1, 14);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[2].1, 11);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[3].1, 11);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[4].1, 9);
    assert_eq!(HEX_CHARS_LOWER.len(), 16);
    assert_eq!(HEX_CHARS_UPPER.as_bytes()[10], b'A');
    assert_eq!(SIGNATURE_FORMATS.len(), 3);
};
`;

const p = writeSrc("v8-wasm-objects", "wasm_disassembler_cc.rs", rust);
console.log(`written ${p}`);
