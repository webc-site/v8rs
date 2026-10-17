#!/usr/bin/env bun
// tool/src/snapshot/embedded/embedded-file-writer.js
// 机械提取 v8/src/snapshot/embedded/embedded-file-writer.{h,cc} 的常量与方法面

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/snapshot/embedded/embedded-file-writer.h");
const ccSrc = readV8("src/snapshot/embedded/embedded-file-writer.cc");

// 1. 常量
const tmpLenM = hSrc.match(/static constexpr int kTemporaryStringLength = (\d+);/);
const firstIdM = hSrc.match(/static constexpr int kFirstExternalFilenameId = (\d+);/);
if (!tmpLenM || !firstIdM) throw new Error("未找到 kTemporaryStringLength/kFirstExternalFilenameId");
const tmpLen = parseInt(tmpLenM[1], 10);
const firstId = parseInt(firstIdM[1], 10);
if (tmpLen !== 256 || firstId !== 1) throw new Error(`常量异常: ${tmpLen}/${firstId}`);

// 2. 方法面 (h, public)
const pubM = hSrc.match(/class EmbeddedFileWriter : public EmbeddedFileWriterInterface \{([\s\S]*?)\n private:/);
if (!pubM) throw new Error("未找到 EmbeddedFileWriter public 区");
const methods = [...pubM[1].matchAll(/^\s{2}(?:void|int|static FILE\*)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length < 6) throw new Error(`EmbeddedFileWriter 方法过少: ${methods.length}`);

// 3. cc 实现清点
const ccFns = [...ccSrc.matchAll(/^(?:void|int|const char\*) EmbeddedFileWriter::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length < 5) throw new Error(`cc 实现过少: ${ccFns.length}`);
console.log(`常量 2 + 方法 ${methods.length} + cc ${ccFns.length} 实现`);

let rust = banner(
  "src/snapshot/embedded/embedded-file-writer.js",
  "src/snapshot/embedded/embedded-file-writer.{h,cc}",
  "嵌入汇编文件写入器",
);
rust += `
/// 临时字符串缓冲区长度 (原版 kTemporaryStringLength)
pub const TEMPORARY_STRING_LENGTH: usize = ${tmpLen};
/// 外部编译文件名合成 ID 首值 (原版 kFirstExternalFilenameId)
pub const FIRST_EXTERNAL_FILENAME_ID: i32 = ${firstId};

/// EmbeddedFileWriter 方法面 (原版 h public 区, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// cc 实现 (机械清点): ${ccFns.join(", ")}
/// 输出契约: FilePrologue → 布局描述/内建标签 → 代码段 (WriteBuiltin 循环,
/// PadAndAlignCode 对齐) → 数据段 → FileEpilogue; 平台差异全部委托
/// PlatformEmbeddedFileWriterBase (见 platform_embedded_file_writer_base)。
`;

const p = writeSrc("v8-snapshot", "embedded_file_writer.rs", rust);
console.log(`written ${p}`);
