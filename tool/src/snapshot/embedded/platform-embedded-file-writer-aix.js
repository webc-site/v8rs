#!/usr/bin/env bun
// tool/src/snapshot/embedded/platform-embedded-file-writer-aix.js
// 机械提取 v8/src/snapshot/embedded/platform-embedded-file-writer-aix.{h,cc}:
// XCOFF 风格指令表

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-aix.h");
const ccSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-aix.cc");

// 1. SYMBOL_PREFIX
const prefixM = ccSrc.match(/#define SYMBOL_PREFIX "([^"]*)"/);
if (!prefixM) throw new Error("未找到 SYMBOL_PREFIX");

// 2. DirectiveAsString (AIX: kQuad 用 .llong)
const dirM = ccSrc.match(/const char\* DirectiveAsString\(DataDirective directive\) \{([\s\S]*?)\n\}/);
if (!dirM) throw new Error("未找到 DirectiveAsString");
const dirTable = [...dirM[1].matchAll(/case k(\w+):\s*return "(\.[\w.]+)";/g)].map(
  (m) => ({ name: m[1], directive: m[2] }),
);
if (dirTable.length !== 3) throw new Error(`指令表条数不符: 期望 3 (kOcta 走 default UNREACHABLE), 实得 ${dirTable.length}`);

// 3. 段/对齐指令
const directives = [...ccSrc.matchAll(/fprintf\(fp_, "(\.[\w[][^"]*)"/g)].map((m) => m[1]);
if (directives.length < 6) throw new Error(`指令过少: ${directives.length}`);

// 4. 实现方法面
const methods = [...new Set([...ccSrc.matchAll(/PlatformEmbeddedFileWriterAIX::(\w+)\(/g)].map((m) => m[1]))];
if (methods.length !== 16) {
  throw new Error(`AIX 实现方法数不符: 期望 16, 实得 ${methods.length}`);
}
console.log(`前缀 "${prefixM[1]}" + 指令表 ${dirTable.length} + 方法 ${methods.length}`);

let rust = banner(
  "src/snapshot/embedded/platform-embedded-file-writer-aix.js",
  "src/snapshot/embedded/platform-embedded-file-writer-aix.{h,cc}",
  "XCOFF 风格平台写入器 (AIX)",
);
rust += `
/// PlatformEmbeddedFileWriterAIX (XCOFF 风格)。
/// 符号前缀 (原版 SYMBOL_PREFIX 机械展开): "${prefixM[1]}"
/// 汇编指令表 (原版 DirectiveAsString; AIX 特例 kQuad → \`.llong\`):
`;
for (const d of dirTable) rust += `/// - k${d.name} → \`${d.directive}\`\n`;
rust += `
/// 段/对齐指令 (机械提取): ${directives.map((s) => `\`${s}\``).join(", ")}
/// 函数描述符: .csect <name>[DS] + .llong .<name>, 0, 0 (AIX 调用约定);
/// 源信息用 .xline; 全局符号 .globl <name>, hidden。
/// 实现方法面 (共 ${methods.length} 项): ${methods.join(", ")}
`;

const p = writeSrc("v8-snapshot", "platform_embedded_file_writer_aix.rs", rust);
console.log(`written ${p}`);
