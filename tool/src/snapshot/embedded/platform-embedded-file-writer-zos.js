#!/usr/bin/env bun
// tool/src/snapshot/embedded/platform-embedded-file-writer-zos.js
// 机械提取 v8/src/snapshot/embedded/platform-embedded-file-writer-zos.{h,cc}:
// z/OS GOFF 风格

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-zos.h");
const ccSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-zos.cc");

// 1. 十六进制字面量格式 (原版 HexLiteral: %.16lx)
const hexM = ccSrc.match(/return fprintf\(fp_, "(%\.16lx)", static_cast<unsigned long>\(value\)\);/);
if (!hexM) throw new Error("未找到 HexLiteral 格式");

// 2. 文件尾声 (end) / 段指令
const epilogueM = ccSrc.match(/FileEpilogue\(\) \{ fprintf\(fp_, " end\\n"\); \}/);
if (!epilogueM) throw new Error("未找到 FileEpilogue");
const directives = [...ccSrc.matchAll(/fprintf\(fp_, "([^"]*[.\\\w][^"]*)"/g)].map((m) => m[1]);
if (directives.length !== 2) throw new Error(`指令不符: 期望 2 (HexLiteral + 尾声), 实得 ${directives.length}`);

// 3. 实现方法面
const methods = [...new Set([...ccSrc.matchAll(/PlatformEmbeddedFileWriterZOS::(\w+)\(/g)].map((m) => m[1]))];
if (methods.length !== 20) {
  throw new Error(`ZOS 实现方法数不符: 期望 20, 实得 ${methods.length}`);
}
console.log(`HexLiteral + 尾声 + 指令 ${directives.length} + 方法 ${methods.length}`);

let rust = banner(
  "src/snapshot/embedded/platform-embedded-file-writer-zos.js",
  "src/snapshot/embedded/platform-embedded-file-writer-zos.{h,cc}",
  "z/OS 平台写入器",
);
rust += `
/// PlatformEmbeddedFileWriterZOS (z/OS)。
/// 十六进制字面量 (原版 HexLiteral): \`${hexM[1]}\` — 16 位宽零填充;
/// 文件尾声 (原版 FileEpilogue): \` end\`。
/// fprintf 串登记 (共 ${directives.length} 条, z/OS 段指令走 GOFF 宏, 其余登记):
`;
for (const d of directives) rust += `/// - \`${d}\`\n`;
rust += `
/// 实现方法面 (共 ${methods.length} 项): ${methods.join(", ")}
`;

const p = writeSrc("v8-snapshot", "platform_embedded_file_writer_zos.rs", rust);
console.log(`written ${p}`);
