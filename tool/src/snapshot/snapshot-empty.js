#!/usr/bin/env bun
// tool/src/snapshot/snapshot-empty.js
// 机械提取 v8/src/snapshot/snapshot-empty.cc 的空快照桩契约

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/snapshot/snapshot-empty.cc");

// 1. 桩函数面 (机械提取)
const stubs = [...src.matchAll(/(?:void|const v8::StartupData\*|bool)\s+(?:Snapshot::)?(\w+)\([^)]*\)/g)].map((m) => m[1]);
if (stubs.length !== 6) {
  throw new Error(`空快照桩函数数不符: 期望 6, 实得 ${stubs.length}: ${stubs.join(",")}`);
}
// 2. V8_USE_EXTERNAL_STARTUP_DATA 分支
const ext = (src.match(/#ifdef V8_USE_EXTERNAL_STARTUP_DATA/g) || []).length;
if (ext !== 1) throw new Error(`外部快照分支数异常: ${ext}`);
console.log(`空快照桩 ${stubs.length} 条`);

let rust = banner("src/snapshot/snapshot-empty.js", "src/snapshot/snapshot-empty.cc", "空快照桩 (无快照构建)");
rust += `
/// 空快照构建桩 (原版 snapshot-empty.cc, "Used for building without snapshots"):
/// 桩函数面 (共 ${stubs.length} 条):
`;
for (const s of stubs) rust += `/// - \`${s}\` — UNREACHABLE 或返回空/null\n`;
rust += `
/// 分支登记: #ifdef V8_USE_EXTERNAL_STARTUP_DATA x${ext} — 该配置下
/// SetNativesFromFile/SetSnapshotFromFile 为 UNREACHABLE 桩;
/// DefaultSnapshotBlob 恒空, ShouldVerifyChecksum 恒 false。
`;

const p = writeSrc("v8-snapshot", "snapshot_empty.rs", rust);
console.log(`written ${p}`);
