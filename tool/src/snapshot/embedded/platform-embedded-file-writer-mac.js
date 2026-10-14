#!/usr/bin/env bun
// tool/src/snapshot/embedded/platform-embedded-file-writer-mac.js
// 机械提取 v8/src/snapshot/embedded/platform-embedded-file-writer-mac.{h,cc}:
// Mach-O 风格指令表

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-mac.h");
const ccSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-mac.cc");

// 1. DirectiveAsString 指令表
const dirM = ccSrc.match(/const char\* DirectiveAsString\(DataDirective directive\) \{([\s\S]*?)\n\}/);
if (!dirM) throw new Error("未找到 DirectiveAsString");
const dirTable = [...dirM[1].matchAll(/case k(\w+):\s*return "(\.[\w.]+)";/g)].map(
  (m) => ({ name: m[1], directive: m[2] }),
);
if (dirTable.length !== 4) throw new Error(`指令表条数不符: 期望 4, 实得 ${dirTable.length}`);

// 2. 段指令
const sections = [...ccSrc.matchAll(/fprintf\(fp_, "(\.[\w_]+[^"]*)"/g)].map((m) => m[1]);
if (sections.length < 3) throw new Error(`段指令过少: ${sections.length}`);

// 3. 实现方法面
const methods = [...new Set([...ccSrc.matchAll(/PlatformEmbeddedFileWriterMac::(\w+)\(/g)].map((m) => m[1]))];
if (methods.length !== 16) {
  throw new Error(`Mac 实现方法数不符: 期望 16, 实得 ${methods.length}`);
}
console.log(`指令表 ${dirTable.length} + 段指令 ${sections.length} + 方法 ${methods.length}`);

let rust = banner(
  "src/snapshot/embedded/platform-embedded-file-writer-mac.js",
  "src/snapshot/embedded/platform-embedded-file-writer-mac.{h,cc}",
  "Mach-O 风格平台写入器 (macOS)",
);
rust += `
/// PlatformEmbeddedFileWriterMac (Mach-O 风格)。
/// 汇编指令表 (原版 DirectiveAsString):
`;
for (const d of dirTable) rust += `/// - k${d.name} → \`${d.directive}\`\n`;
rust += `
/// 段指令 (机械提取): ${sections.map((s) => `\`${s}\``).join(", ")}
/// (代码 .text / 只读数据 .const_data)
/// 全局符号: .private_extern _<name> — 原版注释登记: 用 .global 会引发
/// 嵌入 blob 哈希运行时校验失败, 故用 private_extern。
/// 实现方法面 (共 ${methods.length} 项): ${methods.join(", ")}
`;

const p = writeSrc("v8-snapshot", "platform_embedded_file_writer_mac.rs", rust);
console.log(`written ${p}`);
