#!/usr/bin/env bun
// tool/src/builtins/generate-bytecodes-builtins-list.js — 机械提取 v8/src/builtins/generate-bytecodes-builtins-list.cc
// 产物: 顶层常量直译 + 函数签名骨架 (字节码 handler 表生成器, 语义不展开)。
// 任务映射: v8-builtins-misc (聚合基线脚本将其归于 v8-builtins-core, 两处并存不覆盖)。
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { readV8, writeSrc, banner, root } from "../../lib.js";

// rustfmt 归一 (与聚合基线脚本一致)
function fmt(body) {
  const r = spawnSync("rustfmt", ["--edition", "2024", "--config-path", join(root, "rustfmt.toml")], { input: body, encoding: "utf8" });
  if (r.status !== 0) throw new Error("rustfmt 归一失败: " + r.stderr);
  return r.stdout;
}

const CC = "src/builtins/generate-bytecodes-builtins-list.cc";
// 对数基准 (生成时核定): WriteBytecode/WriteHeader/main
const EXPECT = { consts: 2, fns: 3 };
const ANCHORS = ["kIllegalBytecodeHandler", "WriteHeader"];

const src = readV8(CC);
for (const a of ANCHORS) if (!src.includes(a)) throw new Error(CC + " 锚点失配: " + a);

// 顶层 const int 提取
const consts = [...src.matchAll(/^const int (\w+) = (-?\d+);$/gm)].map((m) => ({ name: m[1], val: m[2] }));
if (consts.length !== EXPECT.consts) throw new Error(`应 ${EXPECT.consts} 个顶层常量 实际 ${consts.length}`);

// 函数签名骨架 (名 + 起始行)
const fns = [...src.matchAll(/^(?:void|constexpr int|int) (\w+)\(/gm)].map((m) => ({
  name: m[1],
  line: src.slice(0, m.index).split("\n").length,
  ret: m[0].split(" ")[0] === "constexpr" ? "constexpr int" : m[0].split(" ")[0],
}));
if (fns.length !== EXPECT.fns) throw new Error(`应 ${EXPECT.fns} 个函数 实际 ${fns.length}`);

let rust = banner("src/builtins/generate-bytecodes-builtins-list.js", CC, "常量直译 + 函数签名骨架 (字节码内建清单生成器)");
rust += "//! 任务映射: v8-builtins-misc; 聚合基线另归 v8-builtins-core (并存不覆盖)\n\n";

rust += "/// 非法字节码 handler 哨兵 (原版 const int kIllegalBytecodeHandler)\n";
for (const c of consts) rust += `pub const ${c.name.replace(/^k/, "K_").toUpperCase()}: i32 = ${c.val}; // 原版: ${c.name}\n`;
rust += "\n/// 偏移表大小 (原版 kTableSize = BytecodeOperands::kOperandScaleCount * Bytecodes::kBytecodeCount; 依赖字节码计数, 不在此硬编)\n";
rust += "pub const K_TABLE_SIZE_EXPR: &str = \"BytecodeOperands::kOperandScaleCount * Bytecodes::kBytecodeCount\";\n\n";

rust += "/// 函数签名骨架 (名, 返回种类, 原版行号)\npub const FUNCTIONS: &[(&str, &str, u32)] = &[\n";
for (const f of fns) rust += `  ("${f.name}", "${f.ret}", ${f.line}),\n`;
rust += "];\n\n";

rust += "/// 该 .cc 为构建期工具: 展开字节码内建清单头 (BUILTIN_LIST_BYTECODE_HANDLERS), 运行时语义不在迁移范围\n";
rust += "pub const IS_BUILD_TIME_TOOL: bool = true;\n\n";

rust += "// —— 对数断言 ——\nconst _: () = {\n";
for (const c of consts) rust += `  assert!(${c.name.replace(/^k/, "K_").toUpperCase()} == ${c.val});\n`;
rust += `  assert!(FUNCTIONS.len() == ${fns.length});\n`;
rust += "};\n";

const p = writeSrc("v8-builtins-misc", "generate_bytecodes_builtins_list.rs", fmt(rust));
console.log(`[generate-bytecodes-builtins-list.cc] consts=${consts.length} fns=${fns.length} -> ${p}`);
