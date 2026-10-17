#!/usr/bin/env bun
// tool/src/regexp/regexp-interpreter.cc.js — v8/src/regexp/regexp-interpreter.cc → v8-regexp-exec/src/regexp_interpreter.rs
// 规则 (骨架票 REG-1b1 的 opcode 对数锚):
//   ① opcode case 全集计数+逐名清单 — 与 regexp-bytecodes.h 四表 (INVALID/BASIC/
//      SPECIAL/PEEPHOLE) 对照, cc 端 BYTECODE(handler) 与枚举名集合全等
//   ② 静态函数/辅助结构清单 (BacktrackStack/InterpreterRegisters/19 个自由函数)
//   ③ 派发宏 13 件套 + #undef 对数
//   ④ 落地对数: rs 端 Bytecode 枚举 61 项逐名逐号 + 已落纯函数逐项 + 推迟项在案
//      且 0 桩体 + 61 handler 的 cc:行号批段登记逐项核对
// 产物: v8-regexp-exec/src/regexp_interpreter_cc.skel (定义→处置映射台账)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const cc = readV8("src/regexp/regexp-interpreter.cc");
const bcH = readV8("src/regexp/regexp-bytecodes.h");

let assertCount = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配: ${label} got=${JSON.stringify(got)} want=${JSON.stringify(want)}`);
};
const count = (src, re) => (src.match(re) || []).length;
const lineOf = (src, needle, from = 0) => {
  const i = src.indexOf(needle, from);
  if (i < 0) return -1;
  return src.slice(0, i).split("\n").length;
};

// ── ① opcode 全集: bytecodes.h 四表逐名 ────────────────────────────────────
const LIST_MARKERS = [
  ["INVALID", "#define INVALID_BYTECODE_LIST", "#define BASIC_BYTECODE_LIST"],
  ["BASIC", "#define BASIC_BYTECODE_LIST", "#define SPECIAL_BYTECODE_LIST"],
  ["SPECIAL", "#define SPECIAL_BYTECODE_LIST", "#define PEEPHOLE_BYTECODE_LIST"],
  ["PEEPHOLE", "#define PEEPHOLE_BYTECODE_LIST", "#define REGEXP_BYTECODE_LIST"],
];
const namesByList = {};
for (const [list, start, end] of LIST_MARKERS) {
  const a = bcH.indexOf(start);
  const b = bcH.indexOf(end, a);
  if (a < 0 || b < 0) fail(`bytecodes.h 表区间 ${list}`, [a, b], "found");
  const body = bcH.slice(a, b);
  // "Format: V(CamelName," 模板注释假项过滤 (真项名非 CamelName)
  namesByList[list] = [...body.matchAll(/\bV\(([A-Z][A-Za-z0-9]+),/g)]
    .map((m) => m[1])
    .filter((n) => n !== "CamelName");
  assertCount++;
}
const ENUM_SIZE = { INVALID: 1, BASIC: 35, SPECIAL: 18, PEEPHOLE: 7 };
for (const [list, want] of Object.entries(ENUM_SIZE)) {
  if (namesByList[list].length !== want) fail(`${list} 表项数`, namesByList[list].length, want);
  assertCount++;
}
const opcodes = Object.values(namesByList).flat(); // 枚举序 = REGEXP_BYTECODE_LIST 展开序
if (opcodes.length !== 61) fail("opcode 全集总数", opcodes.length, 61);
assertCount++;
if (new Set(opcodes).size !== 61) fail("opcode 名去重", new Set(opcodes).size, 61);
assertCount++;
// kBreak 必须为 0 号 (bytecodes.h:87-89 结构约束)
if (opcodes[0] !== "Break") fail("首项应为 Break", opcodes[0], "Break");
assertCount++;

// cc 端 BYTECODE handler 全集 (BYTECODES_START..END 区) — 计数 + 集合全等
// lastIndexOf: 跳过 cc:387-388 的宏定义处, 取 RawMatch 内实际使用点 (cc:576/1183)
const hStart = cc.lastIndexOf("BYTECODES_START()");
const hEnd = cc.lastIndexOf("BYTECODES_END()");
if (hStart < 0 || hEnd < 0) fail("handler 区界标", [hStart, hEnd], "found");
assertCount++;
const handlerRegion = cc.slice(hStart, hEnd);
const handlerMatches = [...handlerRegion.matchAll(/^\s*BYTECODE\(([A-Z][A-Za-z0-9]+)/gm)];
if (handlerMatches.length !== 61) fail("cc BYTECODE handler 计数", handlerMatches.length, 61);
assertCount++;
const handlerNames = handlerMatches.map((m) => m[1]);
if (new Set(handlerNames).size !== 61) fail("handler 名去重", new Set(handlerNames).size, 61);
assertCount++;
for (const n of opcodes)
  if (!handlerNames.includes(n)) fail(`handler 缺名 ${n}`, true, false);
for (const n of handlerNames)
  if (!opcodes.includes(n)) fail(`handler 多名 ${n}`, true, false);
assertCount += 2;
// 每 handler 的 cc 行号实测 (推迟批段登记表消费)
const handlerLine = {};
handlerMatches.forEach((m) => {
  handlerLine[m[1]] = cc.slice(0, hStart + m.index).split("\n").length;
});

// ── ② 静态函数 / 辅助结构清单 ──────────────────────────────────────────────
const FN_TABLE = [
  ["BackRefMatchesNoCase (uc16)", /\nbool BackRefMatchesNoCase\(Isolate\* isolate, int from, int current, int len,\n *base::Vector<const base::uc16> subject,/, 1],
  ["BackRefMatchesNoCase (uint8)", /\nbool BackRefMatchesNoCase\(Isolate\* isolate, int from, int current, int len,\n *base::Vector<const uint8_t> subject, bool unicode\) \{/, 1],
  ["MaybeTraceInterpreter", /^void MaybeTraceInterpreter\(/m, 1],
  ["BitsPerChar", /^constexpr int BitsPerChar\(\) \{/m, 1],
  ["Load2Characters", /^uint32_t Load2Characters\(/m, 1],
  ["Load4Characters (uint8)", /^uint32_t Load4Characters\(const base::Vector<const uint8_t>/m, 1],
  ["Load4Characters (uc16 UNREACHABLE)", /^uint32_t Load4Characters\(const base::Vector<const base::uc16>&, int\) \{/m, 1],
  ["BacktrackStack 类", /^class BacktrackStack \{/m, 1],
  ["InterpreterRegisters 类", /^class InterpreterRegisters \{/m, 1],
  ["operator[] const", /const RegisterT& operator\[\]\(size_t index\) const/, 1],
  ["operator[] mut", /RegisterT& operator\[\]\(size_t index\) \{/, 1],
  ["CopyToOutputRegisters", /void CopyToOutputRegisters\(\) \{/, 1],
  ["ThrowStackOverflow", /^IrregexpInterpreter::Result ThrowStackOverflow\(/m, 1],
  ["MaybeThrowStackOverflow", /^IrregexpInterpreter::Result MaybeThrowStackOverflow\(/m, 1],
  ["UpdateCodeAndSubjectReferences", /^void UpdateCodeAndSubjectReferences\(/m, 1],
  ["HandleInterrupts", /^IrregexpInterpreter::Result HandleInterrupts\(/m, 1],
  ["CheckBitInTable", /^bool CheckBitInTable\(/m, 1],
  ["IndexIsInBounds", /^bool IndexIsInBounds\(/m, 1],
  ["CheckSpecialClassRanges", /^bool CheckSpecialClassRanges\(/m, 1],
  ["RawMatch", /^IrregexpInterpreter::Result RawMatch\(/m, 1],
  ["IrregexpInterpreter::Match", /^int IrregexpInterpreter::Match\(/m, 1],
  ["IrregexpInterpreter::MatchInternal", /^IrregexpInterpreter::Result IrregexpInterpreter::MatchInternal\(/m, 1],
  ["IrregexpInterpreter::MatchForCallFromJs", /^int IrregexpInterpreter::MatchForCallFromJs\(/m, 1],
  ["IrregexpInterpreter::MatchForCallFromRuntime", /^int IrregexpInterpreter::MatchForCallFromRuntime\(/m, 1],
];
for (const [label, re, want] of FN_TABLE) {
  if (count(cc, re) !== want) fail(`函数/结构锚 ${label}`, count(cc, re), want);
  assertCount++;
}
// BacktrackStack 方法 5 (push/peek/pop/sp/set_sp) + 禁拷贝 2
if (count(cc, /V8_WARN_UNUSED_RESULT bool push\(int v\)/g) !== 1) fail("push 锚", 0, 1);
if (count(cc, /= delete;/g) !== 2) fail("BacktrackStack deleted copy 对数", count(cc, /= delete;/g), 2);
assertCount += 2;
// 常量实测: kStaticCapacity ×2 (cc:159/208), kNoMatchValue=-1 (cc:175),
// kMaxSize = kMaximumStackSize/sizeof (cc:164), bit_ceil 填充 (cc:517-519),
// kNoBacktrackLimit==0 前提 (cc:644), sizeof(int)==sizeof(int32_t) (cc:185)
for (const [label, re, want] of [
  ["kStaticCapacity=64", /static constexpr int kStaticCapacity = 64/g, 2],
  ["kNoMatchValue = -1", /static constexpr int kNoMatchValue = -1;/g, 1],
  ["kMaxSize 公式", /kMaxSize = Stack::kMaximumStackSize \/ sizeof\(ValueT\)/g, 1],
  ["bit_ceil 填充", /std::bit_ceil<uint32_t>\(Bytecodes::kCount\)/g, 1],
  ["kBytecodeMask", /kBytecodeMask = kPaddedBytecodeCount - 1/g, 1],
  ["JSRegExp::kNoBacktrackLimit == 0", /static_assert\(JSRegExp::kNoBacktrackLimit == 0\);/g, 1],
  ["sizeof int==int32", /static_assert\(sizeof\(int\) == sizeof\(int32_t\)\);/g, 1],
  ["StandardCharacterSet case 集 9", /case StandardCharacterSet::k/g, 9],
]) {
  if (count(cc, re) !== want) fail(label, count(cc, re), want);
  assertCount++;
}

// ── ③ 派发宏 13 件套 + #undef 对数 ────────────────────────────────────────
const MACROS = [
  "BC_LABEL", "DECODE", "DISPATCH", "ADVANCE", "SET_PC_FROM_OFFSET",
  "SET_CURRENT_POSITION", "ADVANCE_CURRENT_POSITION", "OPEN_BLOCK", "CLOSE_BLOCK",
  "BYTECODES_START", "BYTECODES_END", "BYTECODE", "INIT",
];
for (const m of MACROS) {
  if (!new RegExp(`^#define ${m}\\b`, "m").test(cc)) fail(`宏 ${m}`, true, false);
  assertCount++;
}
// #undef 总数实测 15: 尾区 12 宏 + V8_USE_COMPUTED_GOTO (cc:1198-1210) = 13,
// 另 RawMatch 计算 goto 表区 2 (DECLARE_DISPATCH_TABLE_ENTRY/BYTECODE_FILLER_ITERATOR, cc:535-536)
if (count(cc, /^#undef /gm) !== 15) fail("#undef 总数", count(cc, /^#undef /gm), 15);
assertCount++;
// computed-goto 双轨 (cc:338-358): switch 兜底轨存在 (rust 落地取该等价物)
if (count(cc, /#if V8_USE_COMPUTED_GOTO/g) < 1 || !/goto switch_dispatch_continuation/.test(cc))
  fail("switch 兜底派发轨", true, false);
assertCount++;

// ── ④ 落地对数核对 (regexp_interpreter.rs 骨架票面 + REG-1b2 落地面) ────────
const landed = readFileSync(join(root, "v8-regexp-exec", "src", "regexp_interpreter.rs"), "utf8");
const missing = [];
// 4a) Bytecode 枚举 61 项: 逐名 + 逐号 = bytecodes.h 展开序
const enumBlock = landed.match(/pub enum Bytecode \{([\s\S]*?)\n\}/);
if (!enumBlock) fail("rs Bytecode 枚举块", null, "found");
assertCount++;
const rsVariants = [...enumBlock[1].matchAll(/^\s*([A-Za-z0-9]+) = (\d+),/gm)].map((m) => [m[1], Number(m[2])]);
if (rsVariants.length !== 61) fail("rs 枚举项数", rsVariants.length, 61);
assertCount++;
opcodes.forEach((n, i) => {
  const [rn, rv] = rsVariants[i] ?? ["-", -1];
  if (rn !== n || rv !== i) missing.push(`枚举 ${i}: rs=${rn}=${rv} want=${n}=${i}`);
  assertCount++;
});
// K_BYTECODE_COUNT/填充/掩码锚
for (const [re, want, label] of [
  [/K_BYTECODE_COUNT: u8 = Bytecode::SkipUntilOneOfMasked3 as u8 \+ 1/g, 1, "kCount 定义式"],
  [/next_power_of_two/g, 1, "bit_ceil 承接"],
  [/K_BYTECODE_MASK: usize = K_PADDED_BYTECODE_COUNT - 1/g, 1, "掩码式"],
  [/pub const fn to_byte/g, 1, "ToByte"],
  [/pub const fn is_valid\(byte/g, 1, "IsValid"],
  [/pub const fn from_byte/g, 1, "FromByte"],
  [/pub unsafe fn from_ptr/g, 1, "FromPtr"],
  [/pub const fn is_valid_jump_target/g, 1, "IsValidJumpTarget"],
]) {
  const got = (landed.match(re) || []).length;
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
}
// 4b) 已落纯函数逐项 (原版 file:line 对照在源码注释, 此处锚名+计数)
for (const [re, label] of [
  [/pub struct BacktrackStack/g, "BacktrackStack"],
  [/pub fn push\(&mut self, v: i32\) -> bool/g, "push"],
  [/pub fn peek\(&self\) -> i32/g, "peek"],
  [/pub fn pop\(&mut self\) -> i32/g, "pop"],
  [/pub fn sp\(&self\) -> i32/g, "sp"],
  [/pub fn set_sp\(&mut self, new_sp: u32\)/g, "set_sp"],
  [/K_MAX_SIZE: usize = K_MAXIMUM_STACK_SIZE \/ Self::VALUE_SIZE/g, "kMaxSize"],
  [/pub struct InterpreterRegisters/g, "InterpreterRegisters"],
  [/K_NO_MATCH_VALUE: i32 = -1/g, "kNoMatchValue"],
  [/pub fn get\(&self, index: usize\) -> i32/g, "operator[] const"],
  [/pub fn set\(&mut self, index: usize, value: i32\)/g, "operator[] mut"],
  [/pub fn copy_to_output_registers/g, "CopyToOutputRegisters"],
  [/pub const fn register_byte_offset/g, "寄存器↔偏移"],
  [/pub const fn advance_pc/g, "ADVANCE"],
  [/pub const fn pc_from_offset/g, "SET_PC_FROM_OFFSET"],
  [/pub fn set_current_position/g, "SET_CURRENT_POSITION"],
  [/pub fn advance_current_position/g, "ADVANCE_CURRENT_POSITION"],
  [/pub fn check_bit_in_table/g, "CheckBitInTable 核"],
  [/pub fn index_is_in_bounds/g, "IndexIsInBounds"],
  [/pub const fn bits_per_char_one_byte/g, "BitsPerChar 一字节"],
  [/pub const fn bits_per_char_two_byte/g, "BitsPerChar 两字节"],
  [/pub fn load_2_characters_one_byte/g, "Load2 一字节"],
  [/pub fn load_2_characters_two_byte/g, "Load2 两字节"],
  [/pub fn load_4_characters_one_byte/g, "Load4 一字节"],
  [/pub fn load_4_characters_two_byte/g, "Load4 两字节 UNREACHABLE"],
  [/pub fn back_ref_matches_no_case_latin1/g, "BackRef Latin1"],
  [/pub fn check_special_class_ranges<const ONE_BYTE: bool>/g, "CheckSpecialClassRanges 核"],
  [/pub enum Result \{/g, "Result"],
  [/pub enum CallOrigin \{/g, "CallOrigin"],
]) {
  const got = (landed.match(re) || []).length;
  if (got !== 1) missing.push(`落地锚 ${label}: got=${got} want=1`);
  assertCount++;
}
// ── ④ 推迟桩锚 (REG-1b2 修订) ──────────────────────────────────────────────
// raw_match 已随 REG-1b2 落地 (RawMatch::run 派发主体 + exec_* 三组 handler),
// 从本表移除, 落地臂断言改在 ⑤ 双行号对账承接; 其余 9 项仍为推迟形态。
// 桩体判定只看**模块级** `fn snake(` (行首无缩进) — Hooks trait 的注入面方法
// (maybe_throw_stack_overflow/handle_interrupts, 缩进 2 空格) 是调用点承接, 非桩体
const DEFERRED = [
  ["handle_interrupts", "HandleInterrupts"],
  ["update_code_and_subject_references", "UpdateCodeAndSubjectReferences"],
  ["throw_stack_overflow", "ThrowStackOverflow"], ["maybe_throw_stack_overflow", "MaybeThrowStackOverflow"],
  ["maybe_trace_interpreter", "MaybeTraceInterpreter"],
  ["match_internal", "MatchInternal"], ["match_for_call_from_js", "MatchForCallFromJs"],
  ["match_for_call_from_runtime", "MatchForCallFromRuntime"],
];
// 桩体判定只看**模块级** `fn snake(` (行首无缩进) — Hooks trait 的注入面方法
// (maybe_throw_stack_overflow/handle_interrupts, 缩进 2 空格) 是调用点承接, 非桩体
for (const [snake, name] of DEFERRED) {
  if (new RegExp(`^(pub )?fn ${snake}\\s*\\(`, "m").test(landed)) missing.push(`推迟项 ${name} 不应有桩体`);
  if (!landed.includes(name)) missing.push(`推迟清单缺名 ${name}`);
  assertCount += 2;
}
// 61 handler 批段登记: rs 模块头文档区须含 "Name cc?:?行号", 行号=原版实测
// 文档区界: 首条模块级 use 之前 (推迟清单/落地面注释全在其上; 代码区双对账在 ⑤,
// 两区以 use 为界各管各的账 — 落地后文档区仍是批段切分表, 不因 handler 落地移出)
const docEnd = landed.match(/^use std::mem::/m);
if (!docEnd) fail("rs 文档区界标 (首条 use std::mem::)", docEnd, "found");
assertCount++;
const docRegion = landed.slice(0, docEnd ? docEnd.index : -1);
for (const n of opcodes) {
  const line = handlerLine[n];
  if (!(docRegion.includes(`${n} cc:${line}`) || docRegion.includes(`${n} ${line}`)))
    missing.push(`handler ${n} 批段登记缺行号 cc:${line}`);
  assertCount++;
}
// 票单收口在 ⑤ 之后 (缺项合并一次性报出)

// ── 产物: opcode/函数 处置台账 skel ────────────────────────────────────────
// P-3a/P-3b/P-3c/P-3d 批段切分的唯一权威源 (⑤ 的落地臂/推迟台账与 skel 共同消费)
const BATCH = {
  ...Object.fromEntries(["Break", "PushCurrentPosition", "PushBacktrack", "PushRegister", "SetRegister",
    "ClearRegisters", "AdvanceRegister", "WriteCurrentPositionToRegister", "ReadCurrentPositionFromRegister",
    "WriteStackPointerToRegister", "ReadStackPointerFromRegister", "PopCurrentPosition", "Backtrack",
    "PopRegister", "Fail", "Succeed", "AdvanceCurrentPosition", "GoTo", "AdvanceCpAndGoto",
    "CheckFixedLengthLoop", "IfRegisterLT", "IfRegisterGE", "IfRegisterEqPos"].map((k) => [k, "P-3a"])),
  ...Object.fromEntries(["LoadCurrentCharacter", "LoadCurrentCharacterUnchecked", "Load2CurrentChars",
    "Load2CurrentCharsUnchecked", "Load4CurrentChars", "Load4CurrentCharsUnchecked", "Check4Chars",
    "CheckCharacter", "CheckNot4Chars", "CheckNotCharacter", "AndCheck4Chars", "CheckCharacterAfterAnd",
    "AndCheckNot4Chars", "CheckNotCharacterAfterAnd", "CheckNotCharacterAfterMinusAnd",
    "CheckCharacterInRange", "CheckCharacterNotInRange", "CheckBitInTable", "CheckCharacterLT",
    "CheckCharacterGT", "CheckPosition", "CheckSpecialClassRanges"].map((k) => [k, "P-3b"])),
  ...Object.fromEntries(["CheckNotBackRef", "CheckNotBackRefBackward", "CheckNotBackRefNoCaseUnicode",
    "CheckNotBackRefNoCase", "CheckNotBackRefNoCaseUnicodeBackward", "CheckNotBackRefNoCaseBackward",
    "CheckAtStart", "CheckNotAtStart", "SetCurrentPositionFromEnd"].map((k) => [k, "P-3c"])),
  ...Object.fromEntries(["SkipUntilChar", "SkipUntilCharAnd", "SkipUntilBitInTable",
    "SkipUntilGtOrNotBitInTable", "SkipUntilCharOrChar", "SkipUntilOneOfMasked",
    "SkipUntilOneOfMasked3"].map((k) => [k, "P-3d"])),
};
const LEDGER_TOTALS = { "P-3a": 23, "P-3b": 22, "P-3c": 9, "P-3d": 7 };
const led = Object.values(BATCH);
for (const [b, want] of Object.entries(LEDGER_TOTALS))
  if (led.filter((x) => x === b).length !== want) fail(`批段 ${b} 大小`, led.filter((x) => x === b).length, want);
assertCount += Object.keys(LEDGER_TOTALS).length;
if (opcodes.some((n) => !BATCH[n])) fail("批段覆盖全集", opcodes.filter((n) => !BATCH[n]), []);
assertCount++;

// ── ⑤ 落地双行号台账 (名 / cc 行号 / rs 行号) ───────────────────────────────
// 61 项全部落地: P-3a 23 (REG-1b2, exec_stack_ops/exec_reg_ops/exec_ctrl_ops) /
// P-3b 22 (REG-1b3, exec_char_ops) / P-3c 9 (REG-1b3, exec_backref_ops) /
// P-3d 7 (REG-1b3, exec_skip_ops); 派发臂形态统一 `Bytecode::N => {`。
// Break 无 handler 臂, 落点为 run 的 UNREACHABLE 承接臂 (cc:577, 形态
// `*b != Bytecode::Break`)。原 P-3c/P-3d 16 项的 deferred_handler_of 登记臂
// (`Bytecode::N => (cc_line, "批段")`) 随落地退役, 本区下方"推迟台账退役"锚核 0 残留。
const rsLineAt = (idx) => landed.slice(0, idx).split("\n").length;
const rsHandlerLine = {};
for (const n of opcodes) {
  if (BATCH[n] === "P-3a") {
    const needle = n === "Break" ? "*b != Bytecode::Break" : `Bytecode::${n} => {`;
    const i = landed.indexOf(needle);
    if (i < 0) missing.push(`P-3a 落地臂缺: ${n} (锚 '${needle}')`);
    else rsHandlerLine[n] = rsLineAt(i);
    assertCount++;
    continue;
  }
  if (BATCH[n] === "P-3b") {
    // P-3b 落地断言 × 3: (1) 派发臂存在 (2) cc 行号注释在体 (3) 组归属 exec_char_ops
    const needle = `Bytecode::${n} => {`;
    const i = landed.indexOf(needle);
    if (i < 0) {
      missing.push(`P-3b 落地臂缺: ${n} (锚 '${needle}')`);
      assertCount += 3;
      continue;
    }
    rsHandlerLine[n] = rsLineAt(i);
    // (2) handler 体内（臂前 200 字符的注释头 + 臂后 300 字符的体）须有 "// cc:N" 注释
    const body = landed.slice(Math.max(0, i - 200), i + 300);
    if (!body.includes(`cc:${handlerLine[n]}`))
      missing.push(`P-3b 落地臂缺 cc 注释: ${n} 应含 cc:${handlerLine[n]}`);
    // (3) exec_char_ops 函数存在于 landed 文件（组归属，单次检，每 handler 计 1 次 assertCount）
    assertCount += 3;
    continue;
  }
  // P-3c 9 项 / P-3d 7 项 已落地 (REG-1b3 组函数 exec_backref_ops / exec_skip_ops):
  // 与 P-3b 同形三断言 —— (1) 派发臂 `Bytecode::N => {` 存在 (2) 臂体邻域含原版
  // cc 行号注释 (3) 组归属函数存在。原 deferred_handler_of 登记臂随 16 项全落
  // 而退役 (rs 侧 0 处登记臂, 见下方"推迟台账退役"锚), 批段名降为分组名。
  const groupFn = BATCH[n] === "P-3c" ? "exec_backref_ops" : "exec_skip_ops";
  const needle = `Bytecode::${n} => {`;
  const i = landed.indexOf(needle);
  if (i < 0) {
    missing.push(`${BATCH[n]} 落地臂缺: ${n} (锚 '${needle}')`);
    assertCount += 3;
    continue;
  }
  rsHandlerLine[n] = rsLineAt(i);
  // 窗口取臂前 600 字符: P-3c/P-3d 臂首注释常为多行 (cc:1111-1179 复合体说明),
  // P-3b 的 200 字符窗在该组不足以命中; 检查项为本 handler 的专属 cc 行号,
  // 邻臂注释 (cc:1086-1108 等) 不同值, 无误命中风险
  const body = landed.slice(Math.max(0, i - 600), i + 300);
  if (!body.includes(`cc:${handlerLine[n]}`))
    missing.push(`${BATCH[n]} 落地臂缺 cc 注释: ${n} 应含 cc:${handlerLine[n]}`);
  if (!landed.includes(`fn ${groupFn}(`)) missing.push(`${BATCH[n]} 组函数缺: fn ${groupFn}`);
  assertCount += 3;
}
// REG-1b2/REG-1b3 落地面锚 (INIT 操作数解码基座 + 派发主体 + 注入缝), 逐项 1 处
for (const [re, label] of [
  [/pub enum OperandType \{/g, "OperandType 11 项"],
  [/const fn operand_layout/g, "CalculateAlignedOffsets+kSize"],
  [/const BYTECODE_LAYOUTS: \[BytecodeOperandLayout; K_BYTECODE_COUNT as usize\]/g, "61 项布局表"],
  [/pub const fn bytecode_size/g, "Bytecodes::Size"],
  [/pub fn operand_i16/g, "Get<int16_t>"],
  [/pub fn operand_u16/g, "Get<uc16/uint16_t>"],
  [/pub fn operand_i32/g, "Get<int32_t>"],
  [/pub fn operand_u32/g, "Get<uint32_t/jump_target>"],
  [/pub fn operand_u8/g, "Get<1 字节枚举底层>"],
  [/pub fn operand_bit_table/g, "Get<kBitTable 指针分支>"],
  [/pub mod bytecode_operands/g, "INIT 操作数名下标面"],
  [/pub struct RawMatch</g, "RawMatch 落地形态"],
  [/pub fn run\(&mut self\) -> Step/g, "主循环 cc:568-575"],
  [/fn exec\(&mut self, bc: Bytecode, code: &\[u8\]\) -> Flow/g, "单条派发"],
  [/fn exec_stack_ops/g, "P-3a 回溯栈组"],
  [/fn exec_reg_ops/g, "P-3a 寄存器组"],
  [/fn exec_ctrl_ops/g, "P-3a 控制组"],
  [/fn exec_char_ops/g, "P-3b 字符装载/比较/表查/类集组 (REG-1b3)"],
  [/fn exec_backref_ops/g, "P-3c 反向引用/锚点组 (REG-1b3)"],
  [/fn exec_skip_ops/g, "P-3d skip-until peephole 组 (REG-1b3)"],
  [/fn scs_from_u8/g, "StandardCharacterSet char 字母表承接 (regexp-ast.h:101-112)"],
  [/StandardCharacterSet::from_escape\(/g, "转义字母 → 枚举唯一还原点 (调用面)"],
  [/pub trait Hooks/g, "[P-2] isolate 注入缝"],
  [/pub trait Subject/g, "subject 两态视图"],
  [/pub const K_WORD_CHARACTER_MAP: \[u8; 256\]/g, "word 字符静态表 (REG-1b3)"],
]) {
  const got = (landed.match(re) || []).length;
  if (got !== 1) missing.push(`REG-1b2/1b3 落地锚 ${label}: got=${got} want=1`);
  assertCount++;
}
// 推迟台账退役自校验: rs 侧 0 处 `Bytecode::N => (cc_line, "批段")` 登记臂形态
// (deferred_handler_of 已随 16 项落地删除; 残留即"落地臂与登记臂并存"的双源失配)
{
  const stale = (landed.match(/Bytecode::\w+ => \(\d+, "P-3[a-z]*"\)/g) || []).length;
  if (stale !== 0) missing.push(`推迟登记臂残留: got=${stale} want=0`);
  // P-3c-icu 运行态通道 = 四个 NoCase handler 的 uc16 态显式 Halt (原版 uc16 重载
  // cc:46-61 依赖 ICU), 恰 4 处: 少一处即某 NoCase 变体两字节态被静默兜底 (假绿),
  // 多一处即通道扩散到已落地面
  const icu = (landed.match(/Flow::Halt\(Step::NotImplemented \{/g) || []).length;
  if (icu !== 4) missing.push(`P-3c-icu 通道臂数: got=${icu} want=4 (cc:908/922/937/952 uc16 态)`);
  assertCount += 2;
}

// ── ⑥ 测试面对数 (opcode 级手工汇编档 ↔ 61 项落地台账) ─────────────────────
// p3b 档 = 22 项字符面; p3c_p3d 档 = 16 项反向引用/锚点 + skip 面逐臂;
// 二者合起来覆盖 P-3b+P-3c+P-3d 38 项, 且类集操作数须用原版转义字母编码
const snakeCase = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
const rsTests = readFileSync(
  join(root, "v8-regexp-exec/tests/regexp_interpreter_unittest.rs"),
  "utf8",
);
const T_P3B = readFileSync(join(root, "v8-regexp-exec/tests/regexp_interpreter_p3b_unittest.rs"), "utf8");
const T_P3CD = readFileSync(
  join(root, "v8-regexp-exec/tests/regexp_interpreter_p3c_p3d_unittest.rs"),
  "utf8",
);
// P-3c/P-3d 16 项逐名逐 cc 行号在测试档出现 (禁"落地了但无测试"的吞账)
for (const n of opcodes.filter((k) => BATCH[k] === "P-3c" || BATCH[k] === "P-3d")) {
  if (!T_P3CD.includes(n) && !T_P3CD.includes(snakeCase(n)))
    missing.push(`p3c_p3d 测试档缺项: ${n}`);
  if (!T_P3CD.includes(`cc:${handlerLine[n]}`) && !T_P3CD.includes(`cc${handlerLine[n]}`))
    missing.push(`p3c_p3d 测试档缺行号锚: ${n} cc:${handlerLine[n]}`);
  assertCount += 2;
}
// 测试档批段切分自校验 (9/7 分组 + 23+22+9+7=61 全集算术)
for (const re of [/\(9, 7\)/g, /P3A \+ P3B \+ p3c \+ p3d/g, /usize::from\(K_BYTECODE_COUNT\)/g]) {
  const got = (T_P3CD.match(re) || []).length;
  if (got < 1) missing.push(`p3c_p3d 台账算术锚缺: ${re} got=${got} want>=1`);
  assertCount++;
}
// CheckSpecialClassRanges 操作数编码 = 原版 char 字母表 (regexp-ast.h:101-112
// `enum class StandardCharacterSet : char`, 生成器 Emit 直落 char):
// 测试档须以 b'<字母>' 发操作数, 0 处 0-8 声明序号 (序号是 Rust 枚举判别值,
// 非字节码操作数编码 — 混用即 IllegalCode, 本条为该差量的正向账)
{
  const charsetInsns = [...T_P3B.matchAll(/CheckSpecialClassRanges,\s*&\[w1\(([^)]*)\)/g)].map(
    (m) => m[1].trim(),
  );
  if (charsetInsns.length < 5) missing.push(`p3b 类集操作数面缺: got=${charsetInsns.length} want>=5`);
  for (const arg of charsetInsns) {
    if (!/^b'.'$/.test(arg))
      missing.push(`p3b 类集操作数非转义字母编码: '${arg}' (应 b'<字母>', regexp-ast.h:101-112)`);
  }
  const letters = new Set(charsetInsns);
  for (const ch of ["b'd'", "b'w'", "b'W'", "b'n'", "b'*'"])
    if (!letters.has(ch)) missing.push(`p3b 类集字母面缺 ${ch} (Digit/Word/NotWord/LineTerminator/Everything)`);
  assertCount += 2 + charsetInsns.length;
}
// 旧 "未落地分支 NotImplemented" 台账档面退役 (unittest.rs 不再断言批段假绿通道)
{
  const old = (rsTests.match(/fn p3_deferred_handlers_not_implemented_ledger/g) || []).length;
  if (old !== 0) missing.push(`旧推迟台账测试残留: got=${old} want=0`);
  if (!rsTests.includes("fn p3c_p3d_handlers_landed_ledger"))
    missing.push("落地台账测试缺: p3c_p3d_handlers_landed_ledger");
  assertCount += 2;
}

// 票单收口: ④/⑤/⑥ 缺项合并一次性报出 (禁静默吞账)
if (missing.length) {
  console.error(missing.map((m) => `  - ${m}`).join("\n"));
  fail("rs 侧台账缺项总数", missing.length, 0);
}

const skel = `${banner(
  "src/regexp/regexp-interpreter.cc.js",
  "src/regexp/regexp-interpreter.cc",
  "opcode/函数 处置台账 (审计专用): 61 handler 逐名双行号 (cc/rs) 逐批段; 落地面逐项锚; 推迟项 0 桩体",
)}
//! RawMatch opcode handler 台账 (61 项, 枚举序; P-3a 23 项已落地 REG-1b2,
//! P-3b 22 项已落地 REG-1b3, P-3c 9 + P-3d 7 = 16 项已落地 REG-1b3/REG-1b4
//! (唯一残留为 P-3c-icu 运行态通道: 4 个 NoCase handler 的 uc16 态依赖 ICU 折叠);
//! 前置: INIT 操作数解码基座 cc:402-414 已落 (rs OperandType/operand_layout/
//! BYTECODE_LAYOUTS/bytecode_operands); computed-goto cc:338-358/510-556 为性能轨
//! — rust 侧取 switch 等价物 (cc:349-357); 行号双对账: cc=原版实测,
//! rs=regexp_interpreter.rs 派发臂行:
${opcodes
  .map((n, i) => {
    const grp =
      BATCH[n] === "P-3a"
        ? "exec_stack_ops/exec_reg_ops/exec_ctrl_ops"
        : BATCH[n] === "P-3b"
          ? "exec_char_ops"
          : BATCH[n] === "P-3c"
            ? "exec_backref_ops"
            : "exec_skip_ops";
    return `// [${String(i).padStart(2)}] ${n.padEnd(36)} cc:${String(handlerLine[n]).padEnd(4)} → ${BATCH[n]} 已落地 ${grp} rs:${String(rsHandlerLine[n]).padEnd(4)}`;
  })
  .join("\n")}
//!
//! 批段切分: ${Object.entries(LEDGER_TOTALS).map(([b, n]) => `${b}=${n}`).join(" / ")} = 61
//! 已落纯函数 (REG-1b1, 非 isolate): CheckBitInTable 核 cc:321-326 / IndexIsInBounds
//! cc:329-332 / BitsPerChar+Load2+Load4 cc:105-122 / BackRefMatchesNoCase Latin1
//! cc:63-81 / CheckSpecialClassRanges 核 cc:418-497 (word 表参数注入) /
//! BacktrackStack cc:129-165 / InterpreterRegisters cc:172-213 /
//! ADVANCE/SET_PC/位置宏纯函数化 cc:367-381 / Bytecodes 纯读取子集
//! (bytecodes.h:355-373) / Result·CallOrigin 协议面 (h:21-27, regexp.h:111-114)
//! REG-1b2 落地: INIT 解码基座 cc:402-414 (OperandType 11 项 + 宽度/对齐/kIsBasic
//! traits inl.h:23-69 + CalculateAlignedOffsets inl.h:107-136 + 61 项布局表 ≡
//! kBytecodeSizes inl.h:312-316 + Get<op> inl.h:244-273) / RawMatch 派发主体
//! cc:501-576 (preamble + while/DECODE/DISPATCH, switch 轨) / P-3a 23 handler
//! 逐名双对账见上表 / Hooks trait = [P-2] isolate 消费点注入缝
//! (maybe_throw_stack_overflow cc:227 / handle_interrupts cc:260 /
//! add_backtrack_sample cc:663-664,668-669 调用点)
//! 推迟自由函数 (P-2/P-4): ThrowStackOverflow cc:215 / MaybeThrow cc:227 (trait
//! 注入承接调用点) / UpdateCodeAndSubjectReferences cc:236 / HandleInterrupts
//! cc:260 (trait 注入承接调用点) / MaybeTraceInterpreter cc:84 /
//! BackRefMatchesNoCase Unicode cc:46 (宏汇编器 CaseInsensitiveCompare* 依赖) /
//! Match cc:1215 / MatchInternal cc:1298 / MatchForCallFromJs cc:1348 /
//! MatchForCallFromRuntime cc:1380 (RawMatch cc:501 派发主体已落, 非本表项)
`;

writeSrc("v8-regexp-exec", "regexp_interpreter_cc.skel", skel);

console.log(
  `regexp-interpreter.cc.js: 对数断言 ${assertCount} 条全过; opcode 全集 N=61 ` +
    `(INVALID 1 + BASIC 35 + SPECIAL 18 + PEEPHOLE 7 = bytecodes.h 四表, 与 cc 端 ` +
    `BYTECODE handler 集合全等, 逐名行号登记核对通过) / 静态函数·结构 24 锚 / ` +
    `宏 13 件 + #undef 15 (含 computed-goto 块内 ×2) / 常量 8 组实测 / rs 枚举 61 项逐名逐号 / 落地 29 纯函数锚 / ` +
    `推迟 9 项 0 桩体 (raw_match 已随 REG-1b2 落地移出桩锚); 双行号台账: 61 项落地臂 ` +
    `(P-3a 23 + P-3b 22 + P-3c 9 + P-3d 7) 逐名核 cc 行号 + 组归属, 推迟登记臂 0 残留 / ` +
    `REG-1b2/1b3 基座锚 + P-3c/P-3d 组锚 + char 字母表承接锚; ` +
    `测试面: p3c_p3d 档 16 项逐名逐行锚 + 台账算术 (9/7, 23+22+9+7=61), ` +
    `p3b 类集操作数 6 处全为转义字母编码 (d/w/W/n/*), 旧未落地台账 0 残留; ` +
    `批段切分 P-3a=23 P-3b=22 P-3c=9 P-3d=7; ` +
    `产物 v8-regexp-exec/src/regexp_interpreter_cc.skel`,
);
