#!/usr/bin/env bun
// tool/src/regexp/regexp-compiler.h.js — v8/src/regexp/regexp-compiler.h → v8-regexp-syntax/src/tables_generated.rs
// 机械提取 compiler_constants 命名区间表 (\s/\w/\d/行终止符), [from, to+1) 对 + 终结标记,
// 转为 Rust [from, to] 闭区间对; 对数断言; 区间代数在 char_range.rs
import { banner, readV8, writeSrc } from "../../lib.js";

const src = readV8("src/regexp/regexp-compiler.h");

const RANGE_END_MARKER = 0x110000;
const K_LEAD_SURROGATE_START = 0xd800;

// 求值 C++ int 字面量: '\t' / '\r' + 1 / 0x00A0 / kRangeEndMarker
function evalConst(tok) {
  tok = tok.trim();
  if (tok === "kRangeEndMarker") return RANGE_END_MARKER;
  if (tok === "kLeadSurrogateStart") return K_LEAD_SURROGATE_START;
  const plus = tok.match(/^(.+?)\s*\+\s*(\d+)$/);
  if (plus) return evalConst(plus[1]) + Number(plus[2]);
  const chr = tok.match(/^'(\\?.)'/);
  if (chr) {
    const c = chr[1];
    if (c.startsWith("\\")) {
      return { "\\t": 9, "\\r": 13, "\\n": 10 }[c] ?? (() => {
        throw new Error(`未知字符字面量 ${c}`);
      })();
    }
    return c.charCodeAt(0);
  }
  const num = tok.match(/^0[xX][0-9a-fA-F]+$/);
  if (num) return parseInt(tok, 16);
  if (/^-?\d+$/.test(tok)) return parseInt(tok, 10);
  throw new Error(`无法求值: ${tok}`);
}

function extractTable(name) {
  const re = new RegExp(`constexpr (?:int|base::uc32) ${name}\\[\\] = \\{([\\s\\S]*?)\\};`);
  const m = src.match(re);
  if (!m) throw new Error(`regexp-compiler.h: ${name} 结构漂移`);
  return m[1].split(",").filter((s) => s.trim()).map(evalConst);
}

const tables = [
  { rust: "SPACE_RANGES", cpp: "kSpaceRanges" },
  { rust: "WORD_RANGES", cpp: "kWordRanges" },
  { rust: "DIGIT_RANGES", cpp: "kDigitRanges" },
  { rust: "LINE_TERMINATOR_RANGES", cpp: "kLineTerminatorRanges" },
  { rust: "SURROGATE_RANGES", cpp: "kSurrogateRanges" },
];

const out = tables.map((t) => {
  const raw = extractTable(t.cpp);
  if (raw[raw.length - 1] !== RANGE_END_MARKER) throw new Error(`${t.cpp}: 终结标记缺失`);
  // 闭区间对: 原版 [from, to+1) 偶对 → [from, to+1-1]
  const closed = [];
  for (let i = 0; i + 1 < raw.length - 1; i += 2) closed.push([raw[i], raw[i + 1] - 1]);
  return { rust: t.rust, cpp: t.cpp, closed, rawLen: raw.length };
});

for (const t of out) console.log(`${t.cpp}: ${t.closed.length} 个闭区间 (原始 ${t.rawLen} 项) ✓`);

const tableSrc = out
  .map(
    (t) => `/// ${t.cpp}: [from, to] 闭区间对 (原版 [from, to+1) + 终结标记展开)
pub static ${t.rust}: [(u32, u32); ${t.closed.length}] = [
${t.closed.map((p) => `  (${p[0]}, ${p[1]}),`).join("\n")}
];`,
  )
  .join("\n\n");

const body = `${banner("src/regexp/regexp-compiler.h.js", "src/regexp/regexp-compiler.h", "compiler_constants 标准类区间表; 消费方 char_range.rs")}

/// 原版 kRangeEndMarker (终结标记, 展开时已剥除)
pub const RANGE_END_MARKER: u32 = ${RANGE_END_MARKER};

${tableSrc}

// ── 对数断言: 表项数 == 原版数组长度 - 1 (终结标记) 且为偶数 ──
${out.map((t) => `const _: () = assert!(${t.rust}.len() * 2 + 1 == ${t.rawLen});`).join("\n")}
`;

writeSrc("v8-regexp-syntax", "tables_generated.rs", body);
console.log("written: tables_generated.rs");
