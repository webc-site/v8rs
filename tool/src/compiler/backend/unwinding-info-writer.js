#!/usr/bin/env bun
// tool/src/compiler/backend/unwinding-info-writer.js
// v8/src/compiler/backend/unwinding-info-writer.h → v8-compiler-backend/src/unwinding_info_writer.rs.skel
// 顶层 UnwindingInfoWriter: 架构分发壳 (5 架构 include + 不支持占位类)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/unwinding-info-writer.h");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// 架构分发表 (真解析 #if/#elif 链)
const arches = [];
for (const m of h.matchAll(/#(?:if|elif) V8_TARGET_ARCH_(\w+)\n#include "([^"]+)"/g)) {
  arches.push([m[1], m[2]]);
}
if (arches.length !== 5) throw new Error(`架构分发 ${arches.length} != 5`);
const ARCH_TABLE = [
  ["ARM", "src/compiler/backend/arm/unwinding-info-writer-arm.h"],
  ["ARM64", "src/compiler/backend/arm64/unwinding-info-writer-arm64.h"],
  ["X64", "src/compiler/backend/x64/unwinding-info-writer-x64.h"],
  ["S390X", "src/compiler/backend/s390/unwinding-info-writer-s390.h"],
  ["PPC64", "src/compiler/backend/ppc/unwinding-info-writer-ppc.h"],
];
for (let i = 0; i < 5; i++) {
  if (arches[i][0] !== ARCH_TABLE[i][0] || arches[i][1] !== ARCH_TABLE[i][1])
    throw new Error(`架构分发序 ${i}: ${arches[i]} != ${ARCH_TABLE[i]}`);
  checks++;
}
must(h.includes("#else"), "h 不支持分支");
must(h.includes("static_assert(!v8_flags.perf_prof_unwinding_info.value(),"), "h 占位 static_assert");
must(h.includes("class UnwindingInfoWriter {"), "h 占位类");
// 占位类 5 空方法 + nullptr writer
must(count(h, /\{\}/g) === 5, "h 占位空方法 5");
must(h.includes("EhFrameWriter* eh_frame_writer() { return nullptr; }"), "h 占位 nullptr");
must(h.includes("Placeholder for unsupported architectures."), "h 占位注释");

// ── 产物 ────────────────────────────────────────────────────────────────────
const rows = ARCH_TABLE.map(([a, p]) => `//! - V8_TARGET_ARCH_${a} → ${p}`).join("\n");
const skel = `${banner("src/compiler/backend/unwinding-info-writer.js",
  "src/compiler/backend/unwinding-info-writer.h",
  "UnwindingInfoWriter 架构分发壳 (占位类为不支持架构的空实现)")}
//! 台账: 纯 #if 分发头 — 实现见各架构镜像 (各架构头自带同名类的真实定义):
${rows}
//! 其他架构 (ia32/mips64/loong64/riscv) 走 #else 占位类: 5 空方法 +
//!   eh_frame_writer()=nullptr + static_assert(!perf_prof_unwinding_info)。

/// v8: UnwindingInfoWriter 占位面 (不支持架构) — 真实面在架构镜像文件
// pub struct UnwindingInfoWriter; // 分发壳: 按 target arch 选 arm/arm64/x64/s390/ppc 镜像
/// v8: SetNumberOfInstructionBlocks / BeginInstructionBlock / EndInstructionBlock / Finish
// 占位空实现; 真实现写 eh_frame (EhFrameWriter)

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARCH_DISPATCH.len() == 5); // ARM/ARM64/X64/S390X/PPC64
};
pub const ARCH_DISPATCH: &[&str] = &["ARM", "ARM64", "X64", "S390X", "PPC64"];
`;

writeSrc("v8-compiler-backend", "unwinding_info_writer.rs.skel", skel);
console.log(`unwinding-info-writer.js: 对数 ${checks} 条全过; 架构分发 5 (ARM/ARM64/X64/S390X/PPC64); ` +
  `占位空方法 5; 产物 unwinding_info_writer.rs.skel`);
