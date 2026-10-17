#!/usr/bin/env bun
// tool/src/diagnostics/x64/disasm-x64.cc.js
// 机械提取 v8/src/diagnostics/x64/disasm-x64.cc 的反汇编规则面:
//   OperandType/InstructionType/Prefixes 枚举, ByteMnemonic 四表,
//   conditional_code_suffix/cmov_instructions/cmp_pseudo_op 表,
//   InstructionTable 256 项构建 (照抄 Clear/Init/CopyTable/SetTableRange 算法),
//   寄存器名表 (X-macro GENERAL_REGISTERS/DOUBLE_REGISTERS_AVX512/YMM_REGISTERS_AVX512 展开)。
// 语义面 (InstructionDecode 解码实现 ~3200 行) 不属于规则代码, 登记跳过。
// 产物: v8-codegen-x64/src/disasm_x64_generated.rs (禁覆写面旁路, 带 _generated 后缀)

import { readV8, writeSrc, banner } from "../../../lib.js";

const CC_REL = "src/diagnostics/x64/disasm-x64.cc";
const REG_REL = "src/codegen/x64/register-x64.h";
const cc = readV8(CC_REL);
const regH = readV8(REG_REL);

// —— 枚举求值器: 支持 NAME = 数字 | NAME | NAME(自动+1) ——
function parseEnum(body) {
  const out = [];
  let next = 0;
  const clean = body.replace(/\/\/[^\n]*/g, ""); // 先去注释再切分, 注释内含逗号
  for (let raw of clean.split(",")) {
    const m = raw.trim().match(/^(\w+)\s*(?:=\s*(.+))?$/);
    if (!m) continue;
    const [, name, expr] = m;
    let val;
    if (expr === undefined) {
      val = next;
    } else if (/^0x[0-9a-fA-F]+$|^\d+$/.test(expr.trim())) {
      val = Number(expr.trim());
    } else {
      // 形如 A | B 或单名引用
      val = expr.split("|").reduce((acc, part) => {
        const k = part.trim();
        const found = out.find((e) => e.name === k);
        if (!found) throw new Error(`枚举引用未定义: ${k} (于 ${name})`);
        return acc | found.value;
      }, 0);
    }
    out.push({ name, value: val });
    next = val + 1;
  }
  return out;
}

// 1. OperandType
const operandTypeM = cc.match(/enum OperandType\s*\{([^;]+)\};/);
if (!operandTypeM) throw new Error("未找到 OperandType 枚举");
const operandTypes = parseEnum(operandTypeM[1]);
if (operandTypes.length !== 10) throw new Error(`OperandType 条数不符: ${operandTypes.length} != 10`);

// 2. InstructionType
const instrTypeM = cc.match(/enum InstructionType\s*\{([^;]+)\};/);
if (!instrTypeM) throw new Error("未找到 InstructionType 枚举");
const instrTypes = parseEnum(instrTypeM[1]);
if (instrTypes.length !== 9) throw new Error(`InstructionType 条数不符: ${instrTypes.length} != 9`);

// 3. Prefixes
const prefixesM = cc.match(/enum Prefixes\s*\{([^;]+)\};/);
if (!prefixesM) throw new Error("未找到 Prefixes 枚举");
const prefixes = parseEnum(prefixesM[1].replace(/REPEQ_PREFIX\s*=\s*REP_PREFIX/, ""));
if (prefixes.length !== 11) throw new Error(`Prefixes 条数不符: ${prefixes.length} != 11`);
const repeq = prefixes.find((p) => p.name === "REP_PREFIX");
if (!repeq) throw new Error("未找到 REP_PREFIX (REPEQ_PREFIX 别名锚)");

// 4. ByteMnemonic 四表: {0xNN, OP_ORDER, "mnem"}, 以 {-1,...} 终止
function parseByteMnemonics(tableName, expectCount) {
  const re = new RegExp(`static const ByteMnemonic ${tableName}\\[\\]\\s*=\\s*\\{([\\s\\S]*?)\\};`);
  const m = cc.match(re);
  if (!m) throw new Error(`未找到 ${tableName}`);
  const entries = [...m[1].matchAll(/\{(-?0x[0-9a-fA-F]+|-?\d+),\s*(\w+),\s*"([^"]*)"\}/g)].map(
    (e) => ({ b: Number(e[1]), op: e[2], mnem: e[3] }),
  );
  const real = entries.filter((e) => e.b >= 0);
  if (real.length !== expectCount) throw new Error(`${tableName} 条数不符: ${real.length} != ${expectCount}`);
  const term = entries.find((e) => e.b === -1);
  if (!term) throw new Error(`${tableName} 缺 {-1} 终止项`);
  return real;
}
const twoOperands = parseByteMnemonics("two_operands_instr", 42);
const zeroOperands = parseByteMnemonics("zero_operands_instr", 17);
const callJump = parseByteMnemonics("call_jump_instr", 2);
const shortImmediate = parseByteMnemonics("short_immediate_instr", 8);

// 5. conditional_code_suffix[16]
const condM = cc.match(/static const char\* const conditional_code_suffix\[\]\s*=\s*\{([^;]+)\};/);
if (!condM) throw new Error("未找到 conditional_code_suffix");
const condSuffix = [...condM[1].matchAll(/"([^"]+)"/g)].map((s) => s[1]);
if (condSuffix.length !== 16) throw new Error(`conditional_code_suffix 条数不符: ${condSuffix.length} != 16`);

// 6. cmov_instructions[16] 与 cmp_pseudo_op[16]
const cmovM = cc.match(/static const InstructionDesc cmov_instructions\[16\]\s*=\s*\{([\s\S]*?)\};/);
if (!cmovM) throw new Error("未找到 cmov_instructions");
const cmov = [...cmovM[1].matchAll(/\{"([^"]+)"/g)].map((s) => s[1]);
if (cmov.length !== 16) throw new Error(`cmov_instructions 条数不符: ${cmov.length} != 16`);

const cmpPseudoM = cc.match(/static const char\* const cmp_pseudo_op\[16\]\s*=\s*\{([^;]+)\};/);
if (!cmpPseudoM) throw new Error("未找到 cmp_pseudo_op");
const cmpPseudo = [...cmpPseudoM[1].matchAll(/"([^"]+)"/g)].map((s) => s[1]);
if (cmpPseudo.length !== 16) throw new Error(`cmp_pseudo_op 条数不符: ${cmpPseudo.length} != 16`);

// 7. 指令表构建 — 机械照抄 InstructionTable::Clear/Init/CopyTable/
//    SetTableRange/AddJumpConditionalShort 的表构造算法 (同序同覆盖),
//    产出 256 项终态。语义解码实现不在本脚本范围。
const BYTE_SIZE_OPERAND_FLAG = 4; // 原 OperandType::BYTE_SIZE_OPERAND_FLAG
const opVal = (name) => {
  const e = operandTypes.find((t) => t.name === name);
  if (!e) throw new Error(`OperandType 缺 ${name}`);
  return e.value;
};
const tyVal = (name) => {
  const e = instrTypes.find((t) => t.name === name);
  if (!e) throw new Error(`InstructionType 缺 ${name}`);
  return e.value;
};

// Clear(): 全表 (bad)/NO_INSTR/UNSET/false
const table = Array.from({ length: 256 }, () => ({
  mnem: "(bad)",
  type: "NO_INSTR",
  opOrder: "UNSET_OP_ORDER",
  byteSize: false,
  nullMnem: false,
}));

function copyTable(entries, typeName) {
  for (const e of entries) {
    const id = table[e.b];
    if (id.type !== "NO_INSTR") throw new Error(`指令表 0x${e.b.toString(16)} 重复登记 (原 DCHECK_EQ(NO_INSTR, ...))`);
    id.mnem = e.mnem;
    id.opOrder = operandTypes.find((t) => t.value === (opVal(e.op) & ~BYTE_SIZE_OPERAND_FLAG))?.name;
    id.type = typeName;
    id.byteSize = (opVal(e.op) & BYTE_SIZE_OPERAND_FLAG) !== 0;
  }
}
function setTableRange(typeName, start, end, byteSize, mnem) {
  for (let b = start; b <= end; b++) {
    const id = table[b];
    if (id.type !== "NO_INSTR") throw new Error(`SetTableRange 0x${b.toString(16)} 重复登记`);
    id.mnem = mnem;
    id.type = typeName;
    id.byteSize = byteSize;
  }
}

copyTable(twoOperands, "TWO_OPERANDS_INSTR");
copyTable(zeroOperands, "ZERO_OPERANDS_INSTR");
copyTable(callJump, "CALL_JUMP_INSTR");
copyTable(shortImmediate, "SHORT_IMMEDIATE_INSTR");
for (let b = 0x70; b <= 0x7f; b++) {
  if (table[b].type !== "NO_INSTR") throw new Error(`跳转短条件 0x${b.toString(16)} 重复登记`);
  table[b].mnem = ""; // 原版置 nullptr, 由条件码现算
  table[b].nullMnem = true;
  table[b].type = "JUMP_CONDITIONAL_SHORT_INSTR";
}
setTableRange("PUSHPOP_INSTR", 0x50, 0x57, false, "push");
setTableRange("PUSHPOP_INSTR", 0x58, 0x5f, false, "pop");
setTableRange("MOVE_REG_INSTR", 0xb8, 0xbf, false, "mov");

const filled = table.filter((e) => e.type !== "NO_INSTR").length;
console.log(`指令表 256 项构建完成, 非 NO_INSTR ${filled} 项`);

// 8. 寄存器名表: X-macro 展开 ( MAKE_REG_NAME )
// 宏体 = #define NAME(V) 起, 后续以 `\` 续行的所有行
function macroBody(macroName, src) {
  const lines = src.split("\n");
  const start = lines.findIndex((l) => l.startsWith(`#define ${macroName}(V)`));
  if (start < 0) throw new Error(`未找到 X-macro ${macroName}`);
  const body = [];
  for (let i = start + 1; i < lines.length && lines[i - 1].trimEnd().endsWith("\\"); i++) {
    body.push(lines[i]);
  }
  return body.join("\n"); // 不含 #define 首行, 防拼接宏自引用
}
function xmacroNames(macroName, src) {
  const names = [...macroBody(macroName, src).matchAll(/V\((\w+)\)/g)].map((e) => e[1]);
  if (!names.length) throw new Error(`X-macro ${macroName} 展开为空`);
  return names;
}
function xmacroConcat(macroName, src) {
  // 形如 #define X(V) A(V) \ B(V) 的拼接宏
  const parts = [...macroBody(macroName, src).matchAll(/\b([A-Z0-9_]+)\(V\)/g)].map((m) => m[1]);
  if (!parts.length) throw new Error(`拼接宏无组成部分: ${macroName}`);
  return parts.flatMap((p) => xmacroNames(p, src));
}
const generalRegs = xmacroNames("GENERAL_REGISTERS", regH);
if (generalRegs.length !== 16) throw new Error(`GENERAL_REGISTERS 展开条数不符: ${generalRegs.length} != 16`);

// byte_cpu_regs / zmm_regs 直接取自 .cc 字面量
const byteRegsM = cc.match(/static constexpr const char\* const byte_cpu_regs\[\]\{([\s\S]*?)\};/);
if (!byteRegsM) throw new Error("未找到 byte_cpu_regs");
const byteRegs = [...byteRegsM[1].matchAll(/"([^"]+)"/g)].map((s) => s[1]);
if (byteRegs.length !== 16) throw new Error(`byte_cpu_regs 条数不符: ${byteRegs.length} != 16`);

const zmmM = cc.match(/static constexpr const char\* const zmm_regs\[32\]\s*=\s*\{([\s\S]*?)\};/);
if (!zmmM) throw new Error("未找到 zmm_regs");
const zmmRegs = [...zmmM[1].matchAll(/"([^"]+)"/g)].map((s) => s[1]);
if (zmmRegs.length !== 32) throw new Error(`zmm_regs 条数不符: ${zmmRegs.length} != 32`);

// xmm_regs = DOUBLE_REGISTERS_AVX512 = DOUBLE_REGISTERS + EXTENDED_XMM_REGISTERS
// ymm_regs = YMM_REGISTERS_AVX512 = YMM_REGISTERS + EXTENDED_YMM_REGISTERS
const xmmRegs = xmacroConcat("DOUBLE_REGISTERS_AVX512", regH);
const ymmRegs = xmacroConcat("YMM_REGISTERS_AVX512", regH);
if (xmmRegs.length !== 32) throw new Error(`DOUBLE_REGISTERS_AVX512 展开条数不符: ${xmmRegs.length} != 32`);
if (ymmRegs.length !== 32) throw new Error(`YMM_REGISTERS_AVX512 展开条数不符: ${ymmRegs.length} != 32`);

// —— 生成 Rust ——
// 枚举变体转 UpperCamelCase (免 non_camel_case_types), doc 锚保留 C++ 原名
const toPascal = (name) =>
  name.split("_").map((w) => (w.length ? w[0].toUpperCase() + w.slice(1).toLowerCase() : w)).join("");
const enumRust = (name, doc, items, reprType, valueFmt = (v) => v) =>
  `/// ${doc}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(${reprType})]\npub enum ${name} {\n` +
  items.map((e) => `  /// 原 \`${e.name}\`\n  ${toPascal(e.name)} = ${valueFmt(e.value)},`).join("\n") +
  "\n}\n";

let rust = banner("src/diagnostics/x64/disasm-x64.cc.js", CC_REL, "x64 反汇编规则面: 枚举/助记表/256 项指令表/寄存器名表");
rust += `
// 原文件为 #if V8_TARGET_ARCH_X64 门控 (x64 专用); 本产物即 x64 目标展开。
// 语义解码实现 (DisassemblerX64::InstructionDecode) 不在机械提取范围, 由 Developer 修订补齐。

`;
rust += enumRust("OperandType", "操作数编排方式 (原 enum OperandType)", operandTypes, "u8");
rust += "\n" + enumRust("InstructionType", "指令形态 (原 enum InstructionType)", instrTypes, "u8");
rust += "\n" + enumRust("Prefixes", "前缀字节 (原 enum Prefixes)", prefixes, "u8", (v) => `0x${v.toString(16).toUpperCase()}`);
rust += `
/// 原 REPEQ_PREFIX 是 REP_PREFIX 的别名 (变体名 PascalCase 化)
pub const REPEQ_PREFIX: Prefixes = Prefixes::RepPrefix;

/// 单字节助记项 (原 struct ByteMnemonic)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteMnemonic {
  /// 机器码字节; 原版以 -1 终止表, 此处展开为定长表不再需要终止项
  pub b: u8,
  pub op_order: OperandType,
  pub mnem: &'static str,
}

/// 展开原 two_operands_instr (双操作数指令, ${twoOperands.length} 项)
#[rustfmt::skip]
pub static TWO_OPERANDS_INSTR: [ByteMnemonic; ${twoOperands.length}] = [
${twoOperands
  .map((e) => `  ByteMnemonic { b: 0x${e.b.toString(16).toUpperCase().padStart(2, "0")}, op_order: OperandType::${toPascal(e.op)}, mnem: "${e.mnem}" },`)
  .join("\n")}
];

/// 展开原 zero_operands_instr (零操作数指令, ${zeroOperands.length} 项)
#[rustfmt::skip]
pub static ZERO_OPERANDS_INSTR: [ByteMnemonic; ${zeroOperands.length}] = [
${zeroOperands
  .map((e) => `  ByteMnemonic { b: 0x${e.b.toString(16).toUpperCase().padStart(2, "0")}, op_order: OperandType::${toPascal(e.op)}, mnem: "${e.mnem}" },`)
  .join("\n")}
];

/// 展开原 call_jump_instr (call/jmp 直移, ${callJump.length} 项)
#[rustfmt::skip]
pub static CALL_JUMP_INSTR: [ByteMnemonic; ${callJump.length}] = [
${callJump
  .map((e) => `  ByteMnemonic { b: 0x${e.b.toString(16).toUpperCase().padStart(2, "0")}, op_order: OperandType::${toPascal(e.op)}, mnem: "${e.mnem}" },`)
  .join("\n")}
];

/// 展开原 short_immediate_instr (短立即数指令, ${shortImmediate.length} 项)
#[rustfmt::skip]
pub static SHORT_IMMEDIATE_INSTR: [ByteMnemonic; ${shortImmediate.length}] = [
${shortImmediate
  .map((e) => `  ByteMnemonic { b: 0x${e.b.toString(16).toUpperCase().padStart(2, "0")}, op_order: OperandType::${toPascal(e.op)}, mnem: "${e.mnem}" },`)
  .join("\n")}
];

/// 条件码后缀表 (原 conditional_code_suffix[16], 索引即条件码)
#[rustfmt::skip]
pub static CONDITIONAL_CODE_SUFFIX: [&str; 16] = [
${condSuffix.map((s) => `  "${s}",`).join("\n")}
];

/// CMOV 指令助记表 (原 cmov_instructions[16], 索引即条件码)
#[rustfmt::skip]
pub static CMOV_MNEMONICS: [&str; 16] = [
${cmov.map((s) => `  "${s}",`).join("\n")}
];

/// SSE compare 伪操作表 (原 cmp_pseudo_op[16])
#[rustfmt::skip]
pub static CMP_PSEUDO_OP: [&str; 16] = [
${cmpPseudo.map((s) => `  "${s}",`).join("\n")}
];

/// 指令描述 (原 struct InstructionDesc)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionDesc {
  /// 助记符; 原版短跳转项为 nullptr (由条件码现算), 此处用 empty_str 标记
  pub mnem: &'static str,
  pub ty: InstructionType,
  pub op_order: OperandType,
  /// 定长 8 位操作
  pub byte_size_operation: bool,
}

/// 256 项单字节指令终态表 — 机械执行原 InstructionTable::Clear/Init/
/// CopyTable/SetTableRange/AddJumpConditionalShort 后的展开结果。
#[rustfmt::skip]
pub static INSTRUCTION_TABLE: [InstructionDesc; 256] = [
`;

for (let i = 0; i < 256; i += 4) {
  const cells = table
    .slice(i, i + 4)
    .map(
      (e) =>
        `InstructionDesc { mnem: "${e.mnem}", ty: InstructionType::${toPascal(e.type)}, op_order: OperandType::${toPascal(e.opOrder)}, byte_size_operation: ${e.byteSize} },`,
    );
  rust += `  ${cells.join(" ")}\n`;
}

rust += `];

/// 判定短跳转项 (原版 mnem 为 nullptr, 条件码现算)
#[inline]
#[must_use]
pub const fn is_jump_conditional_short(desc: &InstructionDesc) -> bool {
  desc.mnem.is_empty() && matches!(desc.ty, InstructionType::JumpConditionalShortInstr)
}

// —— 寄存器名表 (原文件尾部, X-macro MAKE_REG_NAME 展开) ——

/// 原 cpu_regs = GENERAL_REGISTERS(MAKE_REG_NAME) 展开
#[rustfmt::skip]
pub static CPU_REGS: [&str; 16] = [
${generalRegs.map((s) => `  "${s}",`).join("\n")}
];

/// 原 byte_cpu_regs[16] (REX 前 8 位寄存器名带后缀)
#[rustfmt::skip]
pub static BYTE_CPU_REGS: [&str; 16] = [
${byteRegs.map((s) => `  "${s}",`).join("\n")}
];

/// 原 xmm_regs = DOUBLE_REGISTERS_AVX512(MAKE_REG_NAME) 展开
#[rustfmt::skip]
pub static XMM_REGS: [&str; 32] = [
${xmmRegs.map((s) => `  "${s}",`).join("\n")}
];

/// 原 ymm_regs = YMM_REGISTERS_AVX512(MAKE_REG_NAME) 展开
#[rustfmt::skip]
pub static YMM_REGS: [&str; 32] = [
${ymmRegs.map((s) => `  "${s}",`).join("\n")}
];

/// 原 zmm_regs[32]
#[rustfmt::skip]
pub static ZMM_REGS: [&str; 32] = [
${zmmRegs.map((s) => `  "${s}",`).join("\n")}
];

// —— 编译期可用原语: str 逐字节等值 (const 上下文 PartialEq<&str> 不可用) ——
#[inline]
#[must_use]
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(INSTRUCTION_TABLE.len() == 256);
  assert!(str_eq(INSTRUCTION_TABLE[0x00].mnem, "add"));
  assert!(matches!(INSTRUCTION_TABLE[0x50].ty, InstructionType::PushpopInstr));
  assert!(matches!(INSTRUCTION_TABLE[0x70].ty, InstructionType::JumpConditionalShortInstr));
  assert!(str_eq(INSTRUCTION_TABLE[0xB8].mnem, "mov"));
  assert!(str_eq(INSTRUCTION_TABLE[0xE8].mnem, "call"));
  assert!(str_eq(CMOV_MNEMONICS[0], "cmovo"));
  assert!(str_eq(CMP_PSEUDO_OP[0], "eq"));
  assert!(str_eq(CONDITIONAL_CODE_SUFFIX[0], "o"));
  assert!(str_eq(CPU_REGS[0], "rax"));
  assert!(str_eq(BYTE_CPU_REGS[0], "al"));
  assert!(str_eq(XMM_REGS[0], "xmm0"));
  assert!(str_eq(YMM_REGS[0], "ymm0"));
  assert!(str_eq(ZMM_REGS[31], "zmm31"));
};
`;

const p = writeSrc("v8-codegen-x64", "disasm_x64_generated.rs", rust);
console.log(`written ${p}`);
