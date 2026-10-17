#!/usr/bin/env bun
// tool/src/wasm/interpreter/instruction-handlers.h.js
// 机械展开 v8/src/wasm/interpreter/instruction-handlers.h 的 FOREACH_INSTR_HANDLER
// X-macro 宏链, 生成 InstructionHandler 枚举。
// 配置: 缺省分支 — V8_DRUMBRAKE_BOUNDS_CHECKS 关, V8_ENABLE_DRUMBRAKE_TRACING 关。
// 对数锚: clang -E 展开共 1690 个 handler (硬规则 4, 原版侧编译器产物)。

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/interpreter/instruction-handlers.h");

// 取宏定义体 (续行拼接)
function macroBody(name) {
  const start = hSrc.indexOf(`#define ${name}`);
  if (start < 0) throw new Error(`未找到宏 ${name}`);
  const lines = hSrc.slice(start).split("\n");
  const out = [];
  for (let i = 1; i < lines.length; i++) {
    const cont = /\\\s*$/.test(lines[i]);
    out.push(lines[i].replace(/\\\s*$/, ""));
    if (!cont) break;
  }
  return out.join("\n");
}

const stripCmt = (s) => s.replace(/\/\/.*$/gm, "");

// 带参宏条目: V(__VA_ARGS__ __VA_OPT__(, ) NAME)
function paramNames(name) {
  const b = stripCmt(macroBody(name));
  const names = [...b.matchAll(/V\(__VA_ARGS__ __VA_OPT__\(, \) (\w+)\)/g)].map((m) => m[1]);
  if (names.length === 0) throw new Error(`${name} 无带参条目`);
  // 确认无漏: 条目总数应相等
  const total = (b.match(/V\(__VA_ARGS__/g) || []).length;
  if (total !== names.length) throw new Error(`${name} 条目解析不全: ${names.length}/${total}`);
  return names;
}

// 无参宏条目: V(NAME) 字面量
function plainNames(name) {
  const b = stripCmt(macroBody(name));
  return [...b.matchAll(/^\s*V\((\w+)\)/gm)].map((m) => m[1]);
}

// —— 各节名单 ——
const regular = paramNames("FOREACH_LOAD_STORE_REGULAR_INSTR_HANDLER");
const superInstr = paramNames("FOREACH_LOAD_STORE_SUPER_INSTR_HANDLER");
const atomic = paramNames("FOREACH_ATOMIC_INSTR_HANDLER");

// NO_BOUNDSCHECK: 字面条目 + 尾部三个子宏调用
const nbBody = stripCmt(macroBody("FOREACH_NO_BOUNDSCHECK_INSTR_HANDLER"));
const nbPlain = [...nbBody.matchAll(/^\s*V\((\w+)\)/gm)].map((m) => m[1]);
const nbTail = [...nbBody.matchAll(/^\s*(FOREACH_\w+_INSTR_HANDLER)\(V\)\s*$/gm)].map((m) => m[1]);
if (nbTail.join(",") !== "FOREACH_MULTIMEM64_LOAD_STORE_INSTR_HANDLER,FOREACH_MULTIMEM64_ATOMIC_INSTR_HANDLER,FOREACH_ATOMIC_INSTR_HANDLER") {
  throw new Error(`NO_BOUNDSCHECK 尾部组成异常: ${nbTail}`);
}

// GENERATE_MULTIMEM64_INSTR_HANDLER(V, name) → name×{MultiMem64, MultiMem, Mem64}
const GEN3 = (n) => [`${n}_MultiMem64`, `${n}_MultiMem`, `${n}_Mem64`];
// GENERATE_MEM64_ONLY_INSTR_HANDLER(V, name) → name_Mem64
const GEN_MEM64 = (n) => [`${n}_Mem64`];

// —— 缺省 FOREACH_INSTR_HANDLER 组装 (源顺序) ——
const handlers = [];
const seen = new Set();
function push(n, from) {
  if (seen.has(n)) throw new Error(`重复 handler ${n} (来自 ${from})`);
  seen.add(n);
  handlers.push(n);
}
// 1) FOREACH_LOAD_STORE_INSTR_HANDLER(V) = REGULAR(V) + SUPER(V), 字面展开
for (const n of regular) push(n, "LOAD_STORE_REGULAR");
for (const n of superInstr) push(n, "LOAD_STORE_SUPER");
// 2) FOREACH_NO_BOUNDSCHECK_INSTR_HANDLER(V):
//    2a) 字面条目
for (const n of nbPlain) push(n, "NO_BOUNDSCHECK");
//    2b) MULTIMEM64_LOAD_STORE = REGULAR(GENERATE_MULTIMEM64) + SUPER(GENERATE_MEM64_ONLY)
for (const n of regular) for (const g of GEN3(n)) push(g, "MM64_LOAD_STORE");
for (const n of superInstr) for (const g of GEN_MEM64(n)) push(g, "MM64_LOAD_STORE");
//    2c) MULTIMEM64_ATOMIC = ATOMIC(GENERATE_MULTIMEM64)
for (const n of atomic) for (const g of GEN3(n)) push(g, "MM64_ATOMIC");
//    2d) ATOMIC(V) 字面
for (const n of atomic) push(n, "ATOMIC");

// —— 对数锚 (clang -E): 1690 ——
const ANCHOR = 1690;
if (handlers.length !== ANCHOR) {
  throw new Error(`handler 条数不符: ${handlers.length}, 锚 ${ANCHOR}`);
}
// 分节复核 (与 clang -E 产物后缀分布一致)
const cnt = (re) => handlers.filter((h) => re.test(h)).length;
if (cnt(/_MultiMem64$/) !== 162) throw new Error(`MultiMem64 计数不符: ${cnt(/_MultiMem64$/)}`);
if (cnt(/_MultiMem$/) !== 162) throw new Error(`MultiMem 计数不符: ${cnt(/_MultiMem$/)}`);
if (cnt(/_Mem64$/) !== 185) throw new Error(`Mem64 计数不符: ${cnt(/_Mem64$/)}`);
if (cnt(/_Idx32_Idx64$/) !== 1 || cnt(/_Idx64_Idx32$/) !== 1) throw new Error("MemMem 变体计数不符");

console.log(`instruction-handlers.h: handlers=${handlers.length} (clang -E 锚 ${ANCHOR})`);

// —— 生成 Rust ——
let rust = banner(
  "src/wasm/interpreter/instruction-handlers.h.js",
  "src/wasm/interpreter/instruction-handlers.h",
  "InstructionHandler 枚举 (X-macro 全展开; 缺省配置: 无 BOUNDS_CHECKS / 无 TRACING)",
);

rust += `
/// 指令处理函数枚举 (对应原版 enum InstructionHandler : uint16_t)
/// 展开自 FOREACH_INSTR_HANDLER 宏链; 对数锚: clang -E 展开 ${ANCHOR} 项
/// 配置: V8_DRUMBRAKE_BOUNDS_CHECKS 关, V8_ENABLE_DRUMBRAKE_TRACING 关
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum InstructionHandler {
`;

for (const [i, h] of handlers.entries()) {
  rust += `  /// k_${h}\n  ${h} = ${i},\n`;
}

rust += `}

impl InstructionHandler {
  /// 处理函数总数 (原版 kInstructionCount)
  pub const COUNT: u16 = ${handlers.length};

  /// 从数值反查 (超表 large-id 以取模落表, 见原版 ReadFnId)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u16) -> Option<Self> {
    match val {
`;
for (const [i, h] of handlers.entries()) {
  rust += `      ${i} => Some(Self::${h}),\n`;
}
rust += `      _ => None,
    }
  }
}

/// 处理函数尺寸类别 (对应原版 enum InstrHandlerSize)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InstrHandlerSize {
  /// Large = 0 (false)
  Large = 0,
  /// Small = 1 (true)
  Small = 1,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(InstructionHandler::COUNT == ${handlers.length});
};
`;

const p = writeSrc("v8-wasm-interp", "instruction_handlers_generated.rs", rust);
console.log(`written ${p}`);
