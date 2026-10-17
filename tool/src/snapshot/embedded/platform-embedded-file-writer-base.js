#!/usr/bin/env bun
// tool/src/snapshot/embedded/platform-embedded-file-writer-base.js
// 机械提取 v8/src/snapshot/embedded/platform-embedded-file-writer-base.{h,cc} 的
// DataDirective/目标平台枚举与写入器 trait 面

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-base.h");
const ccSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-base.cc");

// 1. DataDirective 枚举 (顺序赋值)
const ddM = hSrc.match(/enum DataDirective \{([^}]+)\};/);
if (!ddM) throw new Error("未找到 DataDirective 枚举");
const directives = ddM[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (directives.length !== 4) {
  throw new Error(`DataDirective 条数不符: 期望 4, 实得 ${directives.length}`);
}
const directiveNames = directives.map((d) => d.replace(/^k/, ""));

// 2. EmbeddedTargetOs / EmbeddedTargetArch 枚举
function parseEnum(re, expect, label) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  const items = m[1]
    .replace(/\/\/.*$/gm, "") // 去行注释 (注释可能换行悬挂在条目后)
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
  if (items.length !== expect) {
    throw new Error(`${label} 条数不符: 期望 ${expect}, 实得 ${items.length}`);
  }
  return items.map((t) => {
    const nm = t.match(/^k(\w+)$/);
    if (!nm) throw new Error(`${label} 条目异常: ${t}`);
    return nm[1];
  });
}
const targetOs = parseEnum(/enum class EmbeddedTargetOs \{([^}]+)\};/, 9, "EmbeddedTargetOs");
const targetArch = parseEnum(/enum class EmbeddedTargetArch \{([^}]+)\};/, 5, "EmbeddedTargetArch");

// 3. cc: DataDirectiveSize 表 (机械提取 sizeof 语义)
const sizeM = ccSrc.match(/int DataDirectiveSize\(DataDirective directive\) \{([\s\S]*?)\n\}/);
if (!sizeM) throw new Error("未找到 DataDirectiveSize");
const sizes = [...sizeM[1].matchAll(/case k(\w+):\s*return (\d+);/g)].map(
  (m) => ({ name: m[1], bytes: parseInt(m[2], 10) }),
);
if (sizes.length !== 4) throw new Error(`DataDirectiveSize 条数不符: 期望 4, 实得 ${sizes.length}`);
const sizeByName = Object.fromEntries(sizes.map((s) => [s.name, s.bytes]));
if (sizeByName.Byte !== 1 || sizeByName.Long !== 4 || sizeByName.Quad !== 8 || sizeByName.Octa !== 16) {
  throw new Error(`DataDirectiveSize 值异常: ${JSON.stringify(sizeByName)}`);
}

// 4. trait 方法面 (纯虚 + 默认)
const clsM = hSrc.match(/class PlatformEmbeddedFileWriterBase \{([\s\S]*?)\n protected:/);
if (!clsM) throw new Error("未找到 PlatformEmbeddedFileWriterBase 类体");
const pureVirt = [...clsM[1].matchAll(/virtual\s+[\w:<>*]+\s+(\w+)\([^)]*\)\s*=\s*0/g)].map((m) => m[1]);
const hasDefault = [...clsM[1].matchAll(/virtual\s+[\w:<>*]+\s+(\w+)\([^)]*\)\s*\{/g)].map((m) => m[1]);
if (pureVirt.length !== 15) {
  throw new Error(`纯虚方法数不符: 期望 15, 实得 ${pureVirt.length}: ${pureVirt.join(",")}`);
}
console.log(`DataDirective ${directives.length} + Os ${targetOs.length} + Arch ${targetArch.length} + trait ${pureVirt.length}`);

let rust = banner(
  "src/snapshot/embedded/platform-embedded-file-writer-base.js",
  "src/snapshot/embedded/platform-embedded-file-writer-base.{h,cc}",
  "平台嵌入文件写入器基类",
);
rust += `
/// 汇编数据指令 (原版 enum DataDirective, 顺序赋值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DataDirective {
`;
directiveNames.forEach((n, i) => {
  rust += `  /// k${n} — ${sizeByName[n]} 字节/项\n  ${n.charAt(0).toUpperCase() + n.slice(1)} = ${i},\n`;
});
rust += `}

impl DataDirective {
  /// 原版 DataDirectiveSize(directive)
  #[inline]
  #[must_use]
  pub const fn size(self) -> usize {
    match self {
      Self::Byte => ${sizeByName.Byte},
      Self::Long => ${sizeByName.Long},
      Self::Quad => ${sizeByName.Quad},
      Self::Octa => ${sizeByName.Octa},
    }
  }
}

/// 嵌入目标操作系统 (原版 enum class EmbeddedTargetOs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbeddedTargetOs {
`;
for (const n of targetOs) rust += `  /// k${n}\n  ${n.charAt(0).toUpperCase() + n.slice(1)},\n`;
rust += `}

/// 嵌入目标架构 (原版 enum class EmbeddedTargetArch)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbeddedTargetArch {
`;
for (const n of targetArch) rust += `  /// k${n}\n  ${n.charAt(0).toUpperCase() + n.slice(1)},\n`;
rust += `}

/// PlatformEmbeddedFileWriterBase (原版抽象基类, 为 embedded.S 输出平台相关汇编):
/// 纯虚方法面 (共 ${pureVirt.length} 项):
`;
for (const m of pureVirt) rust += `/// - \`${m}\`\n`;
rust += `
/// 带默认实现: ${hasDefault.join(", ")};
/// MaybeEmitUnwindData 仅 x64 Windows 发射 unwind 数据, 其余平台空操作。
/// 工厂: NewPlatformEmbeddedFileWriter(target_arch, target_os) 按目标选实现。
`;

const p = writeSrc("v8-snapshot", "platform_embedded_file_writer_base.rs", rust);
console.log(`written ${p}`);
