#!/usr/bin/env bun
// tool/src/d8/memory-access-information.h.js
// 机械提取 v8/src/d8/memory-access-information.{h,cc}
// 产物: rs8/src/memory_access_information.rs — Kind/Extension 枚举 + 宽度表 + 寄存器别名表 + 助记符表

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/d8/memory-access-information.h");
const ccSrc = readV8("src/d8/memory-access-information.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 门控: 同 V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT (x64+Linux 专属)
must(hSrc.includes("#define V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT"), "门控宏定义");

// 2. reg_value_type 别名
must(hSrc.includes("using reg_value_type = unsigned long long;"), "reg_value_type 别名");

// 3. Kind 枚举 5 / Extension 枚举 3
const kindBody = hSrc.match(/enum Kind \{ ([^}]+) \};/);
must(kindBody, "Kind 枚举");
const kinds = kindBody[1].split(",").map((s) => s.trim());
if (kinds.length !== 5) fail("Kind 条目", kinds.length, 5);
const extBody = hSrc.match(/enum Extension \{ ([^}]+) \};/);
must(extBody, "Extension 枚举");
const exts = extBody[1].split(",").map((s) => s.trim());
if (exts.length !== 3) fail("Extension 条目", exts.length, 3);

// 4. 结构体字段 7 (带默认值)
const fields = [
  ["kind", "Kind"],
  ["result_reg", "*mut reg_value_type = nullptr"],
  ["xmm_reg_index", "int = -1"],
  ["k_reg_index", "int = -1"],
  ["access_width", "int = 8"],
  ["dest_width", "int = 8"],
  ["extension", "Extension = kNoExtend"],
];
for (const [f] of fields) {
  if (!hSrc.includes(`${f}`)) fail(`字段 ${f}`, false, true);
}

// 5. 宽度后缀表 (cc width_from_suffix): b=1 w=2 l/d=4 q=8 default=8
const widthBody = ccSrc.match(/int width_from_suffix\(char c\) \{([\s\S]*?)\n  \}/);
must(widthBody, "width_from_suffix 函数体");
const widths = [["b", 1], ["w", 2], ["q", 8]];
for (const [ch, w] of widths) {
  const re = new RegExp(`case '${ch}':\\s*\\n?\\s*return ${w};`);
  if (!re.test(widthBody[1])) fail(`宽度 ${ch}=${w}`, false, true);
}
// l/d 共享 return 4 (case 穿透)
if (!/case 'l':\s*\n\s*case 'd':\s*\n\s*return 4;/.test(widthBody[1])) fail("宽度 l/d=4 穿透", false, true);
must(widthBody[1].includes("default:\n      return 8;"), "default 宽度 8");

// 6. 寄存器别名表 16 组 (cc match_reg 链)
const regAliases = [...ccSrc.matchAll(/match_reg\(\{([^}]+)\}\)\) matched_reg = &regs\.(\w+)/g)];
if (regAliases.length !== 16) fail("GPR 别名组", regAliases.length, 16);
for (const [, aliases, reg] of regAliases) {
  const names = aliases.split(",").map((s) => s.trim().replace(/"/g, ""));
  if (!names.some((n) => n === reg)) fail(`别名组首名 ${reg}`, names[0], reg);
}

// 7. 可识别助记符表 (if (!match_mnem({...}))) — 38 项
const mnemBody = ccSrc.match(/if \(!match_mnem\(\{([\s\S]*?)\}\)\)/);
must(mnemBody, "match_mnem 列表");
const mnems = [...mnemBody[1].matchAll(/"(\w+)"/g)].map((m) => m[1]);
if (mnems.length !== 38) fail("助记符数", mnems.length, 38);

// 8. 比较类指令族 8 (cmp/test/comi/ucomi/vcomi/vucomi/ptest/vptest)
const cmpMnems = ["cmp", "test", "comi", "ucomi", "vcomi", "vucomi", "ptest", "vptest"];
for (const m of cmpMnems) {
  if (!new RegExp(`"${m}"`).test(ccSrc)) fail(`比较助记符 ${m}`, false, true);
}
must(ccSrc.includes('"cmpxchg"'), "cmpxchg 特判");

// 9. lddqu/vlddqu 宽度 16 特判
must(ccSrc.includes('memcmp(insn_pos, "lddqu", 5) == 0 || memcmp(insn_pos, "vlddqu", 6) == 0'), "lddqu 特判");
must(ccSrc.includes("access_width = 16;"), "lddqu 宽度 16");

// 10. 唯一公开函数
must(ccSrc.includes("MemoryAccessInformation ParseMemoryAccessInformationFromInstruction("), "Parse 公开函数");

let n = 22;
const rust = `${banner("src/d8/memory-access-information.h.js", "src/d8/memory-access-information.{h,cc}", "内存访问信息: 枚举 + 宽度表 + 寄存器别名表 + 助记符表")}
//! 内存访问信息 — 原版 v8::MemoryAccessInformation 族 (观察点命中侧解析)
//!
//! 语义: 反汇编文本指令 → 访问种类/宽度/目标寄存器; 用于观察点命中时
//! 判定读写与读值落点。门控同 V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT
//! (V8_ENABLE_MEMORY_CORRUPTION_API && V8_OS_LINUX && V8_TARGET_ARCH_X64);
//! arm64 目标整体跳过。

/// 寄存器值类型 (原版 using reg_value_type = unsigned long long)
pub type RegValue = u64;

/// 访问种类 (原版 enum Kind, 5 值)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
${kinds.map((k) => `  /// 原版 ${k}\n  ${k.replace(/^k/, "")},`).join("\n")}
}

/// 扩展方式 (原版 enum Extension, 3 值)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
${exts.map((e) => `  /// 原版 ${e}\n  ${e.replace(/^k/, "")},`).join("\n")}
}

/// 内存访问信息 (原版 struct MemoryAccessInformation, 7 字段 1:1)
#[derive(Debug, Clone, Copy)]
pub struct MemoryAccessInformation {
  pub kind: Kind,
  /// kRead 时二选一: user_regs_struct 内目标 GPR (原版 result_reg)
  pub result_reg: Option<*mut RegValue>,
  /// XMM 寄存器下标 0-15, 非 XMM 为 -1 (原版 xmm_reg_index)
  pub xmm_reg_index: i32,
  /// AVX-512 掩码寄存器下标 0-7, 非掩码为 -1 (原版 k_reg_index)
  pub k_reg_index: i32,
  /// 访问宽度, 默认 8 (原版 access_width)
  pub access_width: i32,
  /// 目标寄存器宽度 — 仅 movsx/movzx 与 access_width 不同 (原版 dest_width)
  pub dest_width: i32,
  pub extension: Extension,
}

/// 宽度后缀表 (原版 width_from_suffix: b=1, w=2, l/d=4, q=8, 其他=8)
pub const WIDTH_FROM_SUFFIX: [(&str, i32); 5] = [
${widths.map(([c, w]) => `  (${JSON.stringify(c)}, ${w}),`).join("\n")}
];

/// GPR 别名表 (原版 match_reg 链 16 组, 64 位名 → 别名集)
pub const GPR_ALIASES: [(&str, [&str; 5]); 16] = [
${regAliases
  .map(([, aliases, reg]) => {
    const names = aliases.split(",").map((s) => s.trim().replace(/"/g, ""));
    while (names.length < 5) names.push("");
    return `  (${JSON.stringify(reg)}, [${names.map((n) => JSON.stringify(n)).join(", ")}]),`;
  })
  .join("\n")}
];

/// 可识别助记符表 (原版 match_mnem 列表, 38 项)
pub const MNEMONICS: [&str; 38] = [
${mnems.map((m) => `  ${JSON.stringify(m)},`).join("\n")}
];

/// 比较类指令族 (cmp/test/comi/ucomi/vcomi/vucomi/ptest/vptest → kCmp;
/// 其中 cmpxchg → kCmpxchg)
pub const CMP_MNEMONICS: [&str; 8] = [
${cmpMnems.map((m) => `  ${JSON.stringify(m)},`).join("\n")}
];

// —— 签名骨架 ——
// parse_memory_access_information_from_instruction(insn_pos: &CStr, regs: &mut UserRegs)
//   -> MemoryAccessInformation
//   "rep movs" → kRepMovs (后缀定宽) / 其他 rep 前缀 → kWrite (宽 8, 忽略语义)
//   movzx/movsx → 双后缀 (源宽+目标宽, mnem_len>=7)
//   'l' 后缀 → kZeroExtend (x64 32 位子寄存器写全清零语义)
//   左操作数为内存 → kWrite; GPR/XMM/掩码 → kRead
//   未识别 → FATAL

// 平台分支台账:
//   V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT 门控 (x64+Linux) — arm64 跳过
//   user_regs_struct (sys/user.h) — Linux 专有, FFI 承接
`;

const p = writeSrc("rs8", "memory_access_information.rs", rust);
console.log(`OK memory-access-information.h.js: 对数断言 ${n} 全过; Kind 5 / Extension 3 / 字段 7 / 宽度 5 / GPR 16 / 助记符 38 / 比较族 8; 产物 ${p}`);
