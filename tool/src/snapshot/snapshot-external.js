#!/usr/bin/env bun
// tool/src/snapshot/snapshot-external.js
// 机械提取 v8/src/snapshot/snapshot-external.cc 的外部快照加载契约

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/snapshot/snapshot-external.cc");

// 1. 函数面
const fns = [...src.matchAll(/(?:void|const v8::StartupData\*|bool)\s+(?:Snapshot::)?(\w+)\(/g)].map((m) => m[1]);
if (fns.length !== 3) {
  throw new Error(`外部快照函数数不符: 期望 3, 实得 ${fns.length}: ${fns.join(",")}`);
}
// 2. 平台分支登记
const android = (src.match(/#ifdef V8_TARGET_OS_ANDROID/g) || []).length;
if (android !== 3) throw new Error(`Android 分支数异常: ${android}`);
// 3. 错误前置 (仅外部快照构建编译)
if (!src.includes("#error snapshot-external.cc is used only for the external snapshot build.")) {
  throw new Error("未找到 #error 前置");
}
console.log(`外部快照函数 ${fns.length} + Android 分支 ${android}`);

let rust = banner("src/snapshot/snapshot-external.js", "src/snapshot/snapshot-external.cc", "外部快照加载");
rust += `
/// 外部快照构建桩 (原版 snapshot-external.cc, 仅 V8_USE_EXTERNAL_STARTUP_DATA
/// 构建编译, 否则 #error)。
/// 函数面 (共 ${fns.length} 条):
`;
for (const f of fns) rust += `/// - \`${f}\`\n`;
rust += `
/// 契约: external_startup_blob 全局单槽 + LazyMutex 互斥;
/// SetSnapshotFromFile 只允许装载一次 (重复装载 DCHECK);
/// 分支登记: #ifdef V8_TARGET_OS_ANDROID x${android} — Android 上外部快照
/// 校验和每进程最多核验一次 (external_startup_checksum_verified 单次闸)。
`;

const p = writeSrc("v8-snapshot", "snapshot_external.rs", rust);
console.log(`written ${p}`);
