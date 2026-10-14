#!/usr/bin/env bun
// tool/src/heap/memory-chunk-inl.h.js — v8/src/heap/memory-chunk-inl.h → v8-heap-core/src/memory_chunk.rs (Metadata 族 + 谓词实体)
// 规则:
//   ① 定义全集 10: MetadataImpl ×2 / Metadata ×4 / MetadataNoIsolateCheck ×2 /
//      IsEvacuationCandidate / InReadOnlySpace — 逐签名锚
//   ② 目标配置展开侧: sandbox 表查找分支保留 (metadata_index_ & mask / CheckIfMetadataAccessibleFromIsolate /
//      CHECK_EQ(metadata->Chunk(), this)); 非沙箱 return metadata_ 剥除;
//      InReadOnlySpace 走非连续 RO 分支 → IsFlagSet(READ_ONLY_HEAP) 保留, kContiguousReadOnlySpaceMask 剥除
//   ③ Metadata 族依赖 heap BasePage/Isolate/IsolateGroup (他票) → 推迟双引用台账
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/memory-chunk-inl.h";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-mcinl.ii");
  for (let i = 0; i < 200; i++) {
    const r = spawnSync("clang", ["-E", "-P", "-x", "c++", "-std=c++20",
      "-DV8_COMPRESS_POINTERS", "-DV8_ENABLE_SANDBOX", "-DV8_EXTERNAL_CODE_SPACE",
      "-I", ".", "-I", "include", "-I", STUB, file, "-o", out], { cwd: V8, encoding: "utf8" });
    if (r.status === 0) return readFileSync(out, "utf8");
    const miss = (r.stderr.match(/fatal error: '([^']+)' file not found/) || [])[1];
    if (!miss) throw new Error(`clang -E ${file} 硬失败:\n${r.stderr}`);
    const p = join(STUB, miss);
    mkdirSync(dirname(p), { recursive: true });
    writeFileSync(p, "#pragma once\n");
  }
  throw new Error(`clang -E ${file} 桩头迭代超限`);
}

let n = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const count = (s, re) => (s.match(re) || []).length;

// ── ① 定义全集 10 ──────────────────────────────────────────────────────────
const DEFS = [
  ["BasePage* MemoryChunk::MetadataImpl(const Isolate* isolate) {", 19],
  ["const BasePage* MemoryChunk::MetadataImpl(const Isolate* isolate) const {", 45],
  ["BasePage* MemoryChunk::Metadata(const Isolate* isolate) {", 49],
  ["const BasePage* MemoryChunk::Metadata(const Isolate* isolate) const {", 53],
  ["BasePage* MemoryChunk::Metadata() {", 57],
  ["const BasePage* MemoryChunk::Metadata() const {", 61],
  ["BasePage* MemoryChunk::MetadataNoIsolateCheck() {", 65],
  ["const BasePage* MemoryChunk::MetadataNoIsolateCheck() const {", 69],
  ["bool MemoryChunk::IsEvacuationCandidate() const {", 73],
  ["V8_INLINE bool MemoryChunk::InReadOnlySpace() const {", 83],
];
for (const [sig, ln] of DEFS) {
  const i = src.indexOf(sig);
  if (i < 0) fail(`定义缺 ${sig}`, i, "found");
  const gotLine = src.slice(0, i).split(String.fromCharCode(10)).length;
  if (gotLine !== ln) fail(`定义行号 ${sig}`, gotLine, ln);
  n += 2;
}

// ── ② 展开侧 ───────────────────────────────────────────────────────────────
const expanded = preprocess(REL);
const miStart = expanded.indexOf("BasePage* MemoryChunk::MetadataImpl(const Isolate* isolate) {");
if (miStart < 0) fail("展开 MetadataImpl 锚", miStart, "found");
n++;
const miRegion = expanded.slice(miStart, expanded.indexOf("\n}", miStart) + 2);
if (count(miRegion, /metadata_index_ & MemoryChunkConstants::kMetadataPointerTableSizeMask/) !== 1) fail("展开表索引掩码式", 0, 1);
n++;
if (count(miRegion, /CheckIfMetadataAccessibleFromIsolate\(isolate\)/) !== 1) fail("展开 isolate 校验调用", 0, 1);
n++;
if (count(miRegion, /metadata->Chunk\(\)/) !== 1) fail("展开 chunk 回指校验 (CHECK_EQ 操作数)", 0, 1);
n++;
if (count(miRegion, /return metadata_;/) !== 0) fail("展开后 MetadataImpl 非沙箱分支残留", 0, 0);
n++;
const roStart = expanded.indexOf("bool MemoryChunk::InReadOnlySpace() const {");
if (roStart < 0) fail("展开 InReadOnlySpace 锚", roStart, "found");
n++;
const roRegion = expanded.slice(roStart, expanded.indexOf("\n}", roStart) + 2);
if (count(roRegion, /IsFlagSet\(READ_ONLY_HEAP\)/) !== 1) fail("展开 RO flag 分支", 0, 1);
n++;
if (count(roRegion, /kContiguousReadOnlySpaceMask/) !== 0) fail("展开后连续 RO 地址分支残留", 0, 0);
n++;
if (count(roRegion, /SynchronizedLoad\(\);/) !== 1) fail("展开 TSAN 前置 SynchronizedLoad", 0, 1);
n++;
// IsEvacuationCandidate DEBUG DCHECK 区剥除后仅剩 flag 读取
const evStart = expanded.indexOf("bool MemoryChunk::IsEvacuationCandidate() const {");
if (evStart < 0) fail("展开 IsEvacuationCandidate 锚", evStart, "found");
n++;
const evRegion = expanded.slice(evStart, expanded.indexOf("\n}", evStart) + 2);
if (count(evRegion, /DCHECK/g) !== 0) fail("展开后 DEBUG DCHECK 残留", count(evRegion, /DCHECK/g), 0);
n++;
if (count(evRegion, /return IsFlagSet\(EVACUATION_CANDIDATE\);/) !== 1) fail("展开 evacuation flag 返回", 0, 1);
n++;

// ── ③ rs 侧落地锚 (HC-1R2a: v8-heap-core/src/memory_chunk.rs 实态对数, 只增不减) ──
const rs = readFileSync(join(root, "v8-heap-core", "src", "memory_chunk.rs"), "utf8");
for (const [label, re, want] of [
  ["IsEvacuationCandidate (inl:80 flag 式)", /pub fn is_evacuation_candidate/, 1],
  ["InReadOnlySpace (inl:92-94 非连续分支)", /pub fn in_read_only_space/, 1],
  ["InReadOnlySpace 前置 SynchronizedLoad (inl:86)", /self\.synchronized_load\(\);/, 1],
  ["RO 旗读取式 (inl:93)", /is_flag_set\(READ_ONLY_HEAP\)/, 1],
  ["MetadataImpl 不落地 (推迟台账 1)", /fn metadata_impl/, 0],
  ["Metadata() 不落地 (推迟台账 1)", /pub fn metadata\(/, 0],
  ["推迟双引用 inl 主区 (inl:18-41)", /inl:18-41/, 1],
  ["推迟双引用 DEBUG 双检区 (inl:74-79)", /inl:74-79/, 1],
  ["不实现连续 RO 分支 (inl:87-91)", /inl:87-91/, 1],
]) {
  if (count(rs, re) !== want) fail(`rs 锚 ${label}`, count(rs, re), want);
  n++;
}

const skel = `${banner("src/heap/memory-chunk-inl.h.js", REL, "MemoryChunk inl 台账: 定义 10 逐项行号双校准 (名+行号)")
}
// ══ memory-chunk-inl.h 处置台账 (实体行号实测) ══
${DEFS.map(([sig, ln]) => `// inl:${String(ln).padEnd(4)} ${sig}`).join("\n")}
// Metadata 族 (MetadataImpl×2 + Metadata×4 + MetadataNoIsolateCheck×2):
//   依赖 heap::BasePage (base-page.h 他票) + IsolateGroup 表 + Isolate →
//   Rust: 前向声明 opaque BasePage; 表访问面 = MetadataTableEnv trait (与 .cc 共用);
//   实体推迟双引用: 原版 inl:18-71 ↔ memory_chunk.rs 「推迟台账」区
// IsEvacuationCandidate inl:73-81 — 目标配置仅剩 flag 读取 (DEBUG DCHECK 双检区剥除, 见展开实测) → 落地
// InReadOnlySpace inl:83-95 — 非连续 RO 分支 (READ_ONLY_HEAP flag) 落地;
//   连续 RO 地址掩码分支 (CONTIGUOUS_COMPRESSED_READ_ONLY_SPACE, 非目标) 登记不实现;
//   TSAN 前置 SynchronizedLoad inl:86 空实现承接 (展开实测调用点保留)
// ── rs 侧落地锚 (HC-1R2a 实测台账) ──
// Landed @ v8-heap-core/src/memory_chunk.rs: inl 可迁面 (IsEvacuationCandidate/InReadOnlySpace 非连续分支 +
//   inl:86 前置空操作) 全落; Metadata 族 8 定义 = 推迟台账 1 (inl:18-71 双行号照录), 连续 RO 分支 = 推迟台账 8 侧翼。
`;
writeSrc("v8-heap-core", "memory_chunk_inl_h.skel", skel);
console.log(`OK memory-chunk-inl.h.js: 对数断言 ${n} 条全过; 定义 10 逐名+逐行号双校准 + 展开侧 (表索引掩码 / isolate 校验 / chunk 回指 / metadata_ 剥除 0 / RO flag 分支 1 / 连续 RO 剥除 0 / SynchronizedLoad 1 / DCHECK 剥除 0 / evacuation 返回 1) + rs 侧落地锚 9 (含 0 检推迟臂 2); 产物 v8-heap-core/src/memory_chunk_inl_h.skel`);
