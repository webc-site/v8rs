#!/usr/bin/env bun
// tool/src/compiler/opcodes.cc.js
// 机械提取 v8/src/compiler/opcodes.cc (+ opcodes.h X-macro 树):
// IrOpcode 枚举全表 + 助记符表 (kMnemonics) + Mnemonic 钳制语义
// 产物为 v8-compiler-passes 侧基线; 枚举正典面与谓词位表在
// v8-compiler-graph/src/opcodes.rs (opcodes.h.js), 挂接时由 Developer 收敛

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/compiler/opcodes.cc");
const src = readV8("src/compiler/opcodes.h");

// —— opcodes.cc 锚 ——
if (!ccSrc.includes("char const* const kMnemonics[] = {")) throw new Error("kMnemonics 缺失");
if (!ccSrc.includes("#define DECLARE_MNEMONIC(x, ...) #x,")) throw new Error("DECLARE_MNEMONIC 缺失");
if (!ccSrc.includes("ALL_OP_LIST(DECLARE_MNEMONIC)")) throw new Error("ALL_OP_LIST 展开缺失");
if (!ccSrc.includes('"UnknownOpcode"')) throw new Error("UnknownOpcode 尾槽缺失");
if (!/size_t const n = std::min<size_t>\(value, arraysize\(kMnemonics\) - 1\);/.test(ccSrc)) {
  throw new Error("Mnemonic 钳制式缺失");
}

// —— 目标配置 (与 tool/src/compiler/opcodes.h.js 对齐) ——
const CONFIG = {
  V8_ENABLE_WEBASSEMBLY: true,
  V8_ENABLE_SIMD128: true,
  V8_ENABLE_SIMD256: false,
  V8_ENABLE_SANDBOX_HARDWARE_SUPPORT: false,
  V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA: true,
};
const IF_MACROS = {
  IF_WASM: CONFIG.V8_ENABLE_WEBASSEMBLY,
  IF_SIMD128: CONFIG.V8_ENABLE_SIMD128,
  IF_SIMD256: CONFIG.V8_ENABLE_SIMD256,
  IF_HARDWARE_SANDBOX: CONFIG.V8_ENABLE_SANDBOX_HARDWARE_SUPPORT,
};

function stripCPED(text) {
  const re = /#ifdef V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA\n([\s\S]*?)#else\n([\s\S]*?)#endif[^\n]*\n/;
  const one = text.replace(re, (_m, a) => (CONFIG.V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA ? a : ""));
  if (!/SIMPLIFIED_CPED_OP_LIST/.test(one)) throw new Error("CPED 分支未匹配");
  return one;
}

function extractDefines(text) {
  const defines = new Map();
  const re = /^#define\s+([A-Z0-9_]+)\s*\(\s*(\w+)\s*\)/gm;
  let m;
  while ((m = re.exec(text)) !== null) {
    const name = m[1];
    let end = m.index + m[0].length;
    for (;;) {
      const nl = text.indexOf("\n", end);
      const line = text.slice(end, nl < 0 ? undefined : nl);
      if (/\\\s*$/.test(line)) end = nl + 1;
      else {
        end = nl < 0 ? text.length : nl;
        break;
      }
    }
    const body = text.slice(m.index + m[0].length, end).replace(/\\\s*/g, " ").replace(/\/\/[^\n]*/g, " ").trim();
    defines.set(name, { vName: m[2], body });
  }
  return defines;
}

const defines = extractDefines(stripCPED(src));
if (!defines.has("ALL_OP_LIST")) throw new Error("ALL_OP_LIST 未找到");

function tokenize(body) {
  const toks = [];
  let depth = 0;
  let cur = "";
  for (const ch of body) {
    if (ch === "(") depth++;
    if (ch === ")") depth--;
    if (depth === 0 && /\s/.test(ch)) {
      if (cur.trim()) toks.push(cur.trim());
      cur = "";
    } else cur += ch;
  }
  if (cur.trim()) toks.push(cur.trim());
  return toks;
}

function parseCall(tok) {
  const i = tok.indexOf("(");
  if (i < 0 || !tok.endsWith(")")) throw new Error(`非法 token ${tok}`);
  const name = tok.slice(0, i);
  const args = [];
  let depth = 0;
  let cur = "";
  for (const ch of tok.slice(i + 1, -1)) {
    if (ch === "(") depth++;
    if (ch === ")") depth--;
    if (ch === "," && depth === 0) {
      args.push(cur.trim());
      cur = "";
    } else cur += ch;
  }
  if (cur.trim()) args.push(cur.trim());
  return { name, args };
}

function expandList(listName) {
  const def = defines.get(listName);
  if (!def) throw new Error(`列表 ${listName} 未定义`);
  const out = [];
  for (const tok of tokenize(def.body)) {
    const { name, args } = parseCall(tok);
    if (name === def.vName) {
      if (!args.length) throw new Error(`${listName}: V() 空参`);
      out.push(args[0]);
    } else if (name in IF_MACROS) {
      if (!IF_MACROS[name]) continue;
      if (args.length && defines.has(args[0])) {
        out.push(...expandList(args[0]));
      } else {
        for (const item of args.slice(1)) out.push(item);
      }
    } else if (defines.has(name)) {
      out.push(...expandList(name));
    } else {
      throw new Error(`${listName}: 未识别调用 ${tok}`);
    }
  }
  return out;
}

const ALL = expandList("ALL_OP_LIST");
const dup = ALL.filter((n, i) => ALL.indexOf(n) !== i);
if (dup.length) throw new Error(`重复 opcode: ${dup.join(",")}`);

// —— 对数断言 (原版条目数; 独立展开, 不取 opcodes.h.js 结果) ——
const TOTAL = 1036;
if (ALL.length !== TOTAL) throw new Error(`IrOpcode 总数 ${ALL.length}, 期望 ${TOTAL}`);
const TOP = {
  CONTROL_OP_LIST: 22,
  COMMON_OP_LIST: 47,
  SIMPLIFIED_OP_LIST: 322,
  MACHINE_OP_LIST: 234,
  MACHINE_SIMD128_OP_LIST: 287,
  JS_OP_LIST: 124,
};
const topLists = {};
for (const [k, n] of Object.entries(TOP)) {
  const l = expandList(k);
  topLists[k] = l;
  if (l.length !== n) throw new Error(`${k} ${l.length}, 期望 ${n}`);
}
const topSum = Object.values(TOP).reduce((a, b) => a + b, 0);
if (topSum !== TOTAL) throw new Error(`顶层分区和 ${topSum} != ${TOTAL}`);
// 区间哨兵 (判别值锚)
const idx = (n) => ALL.indexOf(n);
const SENTINELS = { Start: 0, End: 21, StaticAssert: 68, Word32Clz: 391, TraceInstruction: 624, JSEqual: 912, JSDebugger: 1035 };
for (const [n, v] of Object.entries(SENTINELS)) {
  if (idx(n) !== v) throw new Error(`哨兵 ${n}=${idx(n)}, 期望 ${v}`);
}
// 分区连续性: 顶层六表按序拼接即全集
{
  const concat = Object.values(topLists).flat();
  if (concat.join() !== ALL.join()) throw new Error("顶层六表拼接与 ALL_OP_LIST 不一致");
}

console.log(`opcodes.cc: IrOpcode ${ALL.length} (${Object.entries(TOP).map(([k, v]) => `${k.split("_")[0]}=${v}`).join(" ")}), 尾槽 UnknownOpcode`);

// —— 生成 ——
const last = ALL[TOTAL - 1];

// 枚举行: 按顶层分段注释 + 96 列折行
const TOP_OF = {};
{
  let off = 0;
  for (const [k, l] of Object.entries(topLists)) {
    for (let i = 0; i < l.length; i++) TOP_OF[off + i] = k;
    off += l.length;
  }
}
const enumLines = [];
{
  let curTop = "";
  let col = 0;
  let line = "  ";
  for (let i = 0; i < TOTAL; i++) {
    const t = TOP_OF[i];
    if (t !== curTop) {
      if (line.trim()) enumLines.push(line.trimEnd());
      line = "  ";
      col = 0;
      enumLines.push(`  // ${t}`);
      curTop = t;
    }
    let tok = `${ALL[i]},`;
    if (i === 0) tok = `${ALL[i]} = 0,`;
    if (col + tok.length > 96) {
      enumLines.push(line.trimEnd());
      line = "  ";
      col = 0;
    }
    if (col > 0) {
      line += " ";
      col++;
    }
    line += tok;
    col += tok.length;
  }
  if (line.trim()) enumLines.push(line.trimEnd());
}

// 助记符表: 每行 6 个, 行尾注释首末名
const tableLines = [];
for (let i = 0; i < TOTAL; i += 6) {
  const chunk = ALL.slice(i, Math.min(i + 6, TOTAL)).map((n) => `"${n}"`);
  tableLines.push(`  ${chunk.join(", ")}, // ${ALL[i]}..${ALL[Math.min(i + 5, TOTAL - 1)]}`);
}

let rust = banner(
  "src/compiler/opcodes.cc.js",
  "src/compiler/opcodes.cc + opcodes.h (X-macro 树)",
  "IrOpcode 全表 + 助记符表 (kMnemonics); 正典谓词面在 v8-compiler-graph/src/opcodes.rs",
);

rust += `
//! IrOpcode 助记符面 — opcodes.cc 的 kMnemonics 全表基线。
//!
//! - kMnemonics = ALL_OP_LIST(DECLARE_MNEMONIC) 展开 + 尾槽 "UnknownOpcode"
//! - Mnemonic(value): n = min(value, 表长-1) 钳制后取表 (越界 → 尾槽)
//! - 目标构建: wasm 开 / SIMD128 开 / SIMD256 关 / 硬件沙箱关 / CPED 开
//! - 枚举判别值与 v8-compiler-graph/src/opcodes.rs 同源 (同一 X-macro 展开侧),
//!   挂接时由 Developer 收敛为跨 crate 依赖, 不长期双表
//!
//! 展开计数 (对数): 总 ${TOTAL} = CONTROL ${TOP.CONTROL_OP_LIST} + COMMON ${TOP.COMMON_OP_LIST} + SIMPLIFIED ${TOP.SIMPLIFIED_OP_LIST} + MACHINE ${TOP.MACHINE_OP_LIST} + SIMD128 ${TOP.MACHINE_SIMD128_OP_LIST} + JS ${TOP.JS_OP_LIST}

/// IrOpcode — 原版 \`enum Value { k##x }\`; 变体名去 k 前缀, 判别值 = 展开序
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum IrOpcode {
${enumLines.join("\n")}
}

/// 助记符表 — kMnemonics (与枚举判别值同序, 索引即 opcode)
/// v8: src/compiler/opcodes.cc:18 kMnemonics
pub const MNEMONICS: [&str; IrOpcode::COUNT] = [
${tableLines.join("\n")}
];

impl IrOpcode {
  /// 展开条目总数 (kMnemonics 有效槽数)
  pub const COUNT: usize = ${TOTAL};
  /// 原版 kLast = 展开计数 - 1
  pub const LAST: Self = Self::${last};
  /// 原版 kLast 的裸值
  pub const LAST_U16: u16 = ${TOTAL - 1};
  /// 未知 opcode 的助记符 (kMnemonics 尾槽)
  pub const UNKNOWN: &'static str = "UnknownOpcode";

  /// 原版 IrOpcode::Mnemonic — 本体的取表式
  /// v8: src/compiler/opcodes.cc:28
  #[must_use]
  pub const fn mnemonic(self) -> &'static str {
    MNEMONICS[self as usize]
  }

  /// 原版 IrOpcode::Mnemonic 的裸值形态 — n = min(value, 表长-1) 钳制
  /// (value 越界 → 尾槽 "UnknownOpcode"; DCHECK 输入域在原版为 debug 断言)
  /// v8: src/compiler/opcodes.cc:28
  #[must_use]
  pub const fn mnemonic_of(value: u16) -> &'static str {
    if value as usize <= Self::LAST_U16 as usize {
      MNEMONICS[value as usize]
    } else {
      Self::UNKNOWN
    }
  }
}

impl core::fmt::Display for IrOpcode {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.mnemonic())
  }
}

// —— 生成不变式断言 (数值面) ——
const _: () = {
  assert!(IrOpcode::COUNT == ${TOTAL});
  assert!(MNEMONICS.len() == ${TOTAL});
  assert!(IrOpcode::LAST_U16 == ${TOTAL - 1});
  assert!(IrOpcode::Start as u16 == 0);
  assert!(IrOpcode::End as u16 == 21);
  assert!(IrOpcode::StaticAssert as u16 == 68);
  assert!(IrOpcode::Word32Clz as u16 == 391);
  assert!(IrOpcode::TraceInstruction as u16 == 624);
  assert!(IrOpcode::JSEqual as u16 == 912);
  assert!(IrOpcode::JSDebugger as u16 == 1035);
  assert!(IrOpcode::LAST as u16 == 1035);
};

// —— 字符串面对数 (str 相等非 const, 挂接后随 crate 测试跑; 暂以标准 #[test] 占位,
//    挂接时由 Developer 改写为仓库规范测试样式) ——
#[cfg(test)]
mod generated_checks {
  use super::IrOpcode;

  #[test]
  fn mnemonic_table_strings() {
    assert_eq!(IrOpcode::Start.mnemonic(), "Start");
    assert_eq!(IrOpcode::JSDebugger.mnemonic(), "JSDebugger");
    assert_eq!(IrOpcode::mnemonic_of(1035), "JSDebugger");
    assert_eq!(IrOpcode::mnemonic_of(1036), "UnknownOpcode");
    assert_eq!(IrOpcode::mnemonic_of(u16::MAX), "UnknownOpcode");
  }
}
`;

const p = writeSrc("v8-compiler-passes", "opcodes_cc.rs.skel", rust);
console.log(`written ${p}`);
