#!/usr/bin/env bun
// tool/src/regexp/regexp-flags.h.js — v8/src/regexp/regexp-flags.h → v8-regexp-syntax/src/flags_generated.rs
// 机械提取 REGEXP_FLAG_LIST X-macro (5 元组: lower / Camel / LowerCamel / char / bit),
// 展开 bit 常量 + 对数断言 (总数 / 连续位 / 字符升序), 语义方法在 flags.rs
import { banner, readV8, writeSrc } from "../../lib.js";

const src = readV8("src/regexp/regexp-flags.h");
const m = src.match(/#define REGEXP_FLAG_LIST\(V\)([\s\S]*?)\n\n/);
if (!m) throw new Error("regexp-flags.h: REGEXP_FLAG_LIST 结构漂移");

// V(lower, Camel, LowerCamel, 'c', bit)
const rows = [...m[1].matchAll(/V\((\w+),\s*(\w+),\s*(\w+),\s*'(.)',\s*(\d+)\)/g)].map((r) => ({
  lower: r[1],
  camel: r[2],
  lowerCamel: r[3],
  ch: r[4],
  bit: Number(r[5]),
}));
if (rows.length === 0) throw new Error("REGEXP_FLAG_LIST 零条目");

// 对数断言基线 (脚本内计算, 与生成代码的 const 断言一致)
const total = rows.length;
const bits = rows.map((r) => r.bit).sort((a, b) => a - b);
const contiguous = bits.every((b, i) => b === i);
const chars = rows.map((r) => r.ch).sort().every((c, i, a) => i === 0 || c > a[i - 1]);
if (!contiguous) throw new Error("flag bit 不连续");
if (!chars) throw new Error("flag 字符非升序");
console.log(`regexp-flags: ${total} 个 flag, bit 0..${total - 1} 连续, 字符升序 ✓`);

const consts = rows
  .map((r) => `/// '${r.ch}'`)
  .flatMap((c, i) => [c, `pub const ${rows[i].camel.toUpperCase()}: u16 = 1 << ${rows[i].bit};`])
  .join("\n");

const body = `${banner("src/regexp/regexp-flags.h.js", "src/regexp/regexp-flags.h", "REGEXP_FLAG_LIST X-macro 展开; 语义方法见 flags.rs")}

// ── 对数断言: 表实数 == 宏实数 (${total}) ──
pub const NUM_FLAGS: usize = ${total};

// bit 连续性: 0..${total - 1} (原版 static_assert "contiguous bits")
const _: () = assert!((1 << NUM_FLAGS) - 1 == ${"{ " + rows.map((r) => `1 << ${r.bit}`).join(" | ") + " }"});

// 字符集 (原版 operator<< 顺序 = 宏序, 已按字符升序)
pub const FLAG_CHARS: [char; NUM_FLAGS] = [${rows.map((r) => `'${r.ch}'`).join(", ")}];

// ── bit 常量 (原版 enum class Flag, 位序 1:1) ──
${consts}

/// flag 名元组 (lower 形态, 宏序)
pub const FLAG_NAMES: [&str; NUM_FLAGS] = [${rows.map((r) => `"${r.lower}"`).join(", ")}];
`;

writeSrc("v8-regexp-syntax", "flags_generated.rs", body);
console.log("written: flags_generated.rs");
