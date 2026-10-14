#!/usr/bin/env bun
// tool/src/snapshot/mksnapshot.js
// 机械提取 v8/src/snapshot/mksnapshot.cc 的嵌入/输出文件骨架常量

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/snapshot/mksnapshot.cc");

// 1. SnapshotFileWriter 方法面
const clsM = src.match(/class SnapshotFileWriter \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 SnapshotFileWriter 类体");
const methods = [...clsM[1].matchAll(/^\s{2}(?:void)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 5) {
  throw new Error(`SnapshotFileWriter 方法数不符: 期望 5, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. 生成的嵌入文件骨架 (cc 内 fprintf 模板, 机械提取)
const embeddedDataM = src.match(/"alignas\(kPointerAlignment\) static const uint8_t blob_data\[\] = \{\\n"\)/);
if (!embeddedDataM) throw new Error("未找到嵌入 blob 模板");
const sizeM = src.match(/"static const int blob_size = %d;\\n"/);
if (!sizeM) throw new Error("未找到 blob_size 模板");

// 3. 嵌入链 (机械登记): EmbeddedData::FromBlob → EmbeddedFileWriter::WriteEmbedded
const embM = src.match(/i::EmbeddedData embedded_blob = i::EmbeddedData::FromBlob\(\);/);
const writerM = src.includes("writer->WriteEmbedded(&embedded_blob)");
if (!embM || !writerM) throw new Error("未找到嵌入链调用");
console.log(`SnapshotFileWriter 方法 ${methods.length} + 嵌入模板 2`);

let rust = banner("src/snapshot/mksnapshot.js", "src/snapshot/mksnapshot.cc", "mksnapshot 工具骨架");
rust += `
/// mksnapshot: 离线快照生成工具入口 (346 行, 语义由 Developer 落)。

/// SnapshotFileWriter 方法面 (原版, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// 生成的嵌入文件骨架 (原版 fprintf 模板, 机械登记):
/// \`\`\`c
/// alignas(kPointerAlignment) static const uint8_t blob_data[] = { ... };
/// static const int blob_size = <n>;
/// static const v8::StartupData blob = { blob_data, blob_size };
/// \`\`\`
/// 嵌入链: EmbeddedData::FromBlob → EmbeddedFileWriter::WriteEmbedded
/// (符号约定对齐 embedded_data.rs / embedded_file_writer)。
`;

const p = writeSrc("v8-snapshot", "mksnapshot.rs", rust);
console.log(`written ${p}`);
