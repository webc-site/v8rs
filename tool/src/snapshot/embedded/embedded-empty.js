#!/usr/bin/env bun
// tool/src/snapshot/embedded/embedded-empty.js
// 机械提取 v8/src/snapshot/embedded/embedded-empty.cc 的空嵌入 blob 桩

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/snapshot/embedded/embedded-empty.cc");

// 1. 四个嵌入符号桩
const syms = [...src.matchAll(/(v8_Default_embedded_blob_\w+)/g)];
const uniq = [...new Set(syms.map((m) => m[1]))];
if (uniq.length !== 4) {
  throw new Error(`嵌入符号数不符: 期望 4, 实得 ${uniq.length}: ${uniq.join(",")}`);
}
// 2. DrumBrake 分支登记
const drum = src.includes("#if V8_ENABLE_DRUMBRAKE");
if (!drum) throw new Error("未找到 V8_ENABLE_DRUMBRAKE 分支");
console.log(`空嵌入符号 ${uniq.length}`);

let rust = banner("src/snapshot/embedded/embedded-empty.js", "src/snapshot/embedded/embedded-empty.cc", "空嵌入 blob 桩");
rust += `
/// 无内嵌数据构建桩 (原版 embedded-empty.cc, "Used for building without embedded data"):
/// 四个 C 链接符号桩 (机械登记):
`;
for (const s of uniq) rust += `/// - \`${s}\` — 单字节 {0} 或 0\n`;
rust += `
/// 分支登记: #if V8_ENABLE_DRUMBRAKE — DrumBrake 解释器指令 handler 桩
/// (FOREACH_LOAD_STORE_INSTR_HANDLER 展开为 Builtins_##name 空指针)。
`;

const p = writeSrc("v8-snapshot", "embedded_empty.rs", rust);
console.log(`written ${p}`);
