#!/usr/bin/env bun
// tool/src/snapshot/embedded/platform-embedded-file-writer-generic.js
// 机械提取 v8/src/snapshot/embedded/platform-embedded-file-writer-generic.{h,cc}:
// ELF 风格 (Linux/Android/ChromeOS/Fuchsia/Starboard) 指令表

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-generic.h");
const ccSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-generic.cc");

// 1. SYMBOL_PREFIX 宏机械展开
const prefixM = ccSrc.match(/#define SYMBOL_PREFIX "([^"]*)"/);
if (!prefixM) throw new Error("未找到 SYMBOL_PREFIX");

// 2. DirectiveAsString 指令表
const dirM = ccSrc.match(/const char\* DirectiveAsString\(DataDirective directive\) \{([\s\S]*?)\n\}/);
if (!dirM) throw new Error("未找到 DirectiveAsString");
const dirTable = [...dirM[1].matchAll(/case k(\w+):\s*return "(\.[\w.]+)";/g)].map(
  (m) => ({ name: m[1], directive: m[2] }),
);
if (dirTable.length !== 4) throw new Error(`指令表条数不符: 期望 4, 实得 ${dirTable.length}`);

// 3. 段指令 (机械提取 fprintf 指令串)
const sections = [...ccSrc.matchAll(/fprintf\(fp_, "(\.(?:section|text)[^"]*)"/g)].map((m) => m[1]);
if (sections.length < 2) throw new Error(`段指令过少: ${sections.length}`);

// 4. 实现方法面
const methods = [...new Set([...ccSrc.matchAll(/PlatformEmbeddedFileWriterGeneric::(\w+)\(/g)].map((m) => m[1]))];
if (methods.length !== 17) {
  throw new Error(`Generic 实现方法数不符: 期望 17, 实得 ${methods.length}`);
}
console.log(`前缀 "${prefixM[1]}" + 指令表 ${dirTable.length} + 方法 ${methods.length}`);

let rust = banner(
  "src/snapshot/embedded/platform-embedded-file-writer-generic.js",
  "src/snapshot/embedded/platform-embedded-file-writer-generic.{h,cc}",
  "ELF 风格平台写入器 (Linux/Android/ChromeOS/Fuchsia/Starboard)",
);
rust += `
/// PlatformEmbeddedFileWriterGeneric (ELF 风格)。
/// 符号前缀 (原版 SYMBOL_PREFIX 宏机械展开): "${prefixM[1]}"
/// 汇编指令表 (原版 DirectiveAsString):
`;
for (const d of dirTable) rust += `/// - k${d.name} → \`${d.directive}\`\n`;
rust += `
/// 段指令 (机械提取): ${sections.map((s) => `\`${s}\``).join(", ")}
/// 全局符号: .global + .hidden (降动态链接开销);
/// ChromeOS 特例 SectionText 走 .text.hot.embedded。
/// 实现方法面 (共 ${methods.length} 项): ${methods.join(", ")}
`;

const p = writeSrc("v8-snapshot", "platform_embedded_file_writer_generic.rs", rust);
console.log(`written ${p}`);
