#!/usr/bin/env bun
// tool/src/heap/memory-chunk-layout.h.js — v8/src/heap/memory-chunk-layout.h → v8-heap-core/src/memory_chunk.rs (MemoryChunkLayout 区)
// 规则:
//   ① 函数全集 7 逐签名锚 + 内部 static_assert 2 + DCHECK 1
//   ② 目标配置展开侧: 类体截区核对 7 定义 / DCHECK_NE 剥除 / 两枚 static_assert 保留
//   ③ 布局数值链: sizeof(MemoryChunk)=16 (repr(C) flags usize + index u32) ;
//      ObjectStartOffsetInDataPage=16 ; AllocatableMemoryInDataPage=2^18-16=262128 ;
//      CodePage 依赖 InstructionStream::kHeaderSize (objects 他票, const 参数注入 + 双引用推迟)
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/memory-chunk-layout.h";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-mclay.ii");
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

// ── ① 源码锚: 7 函数 + 2 内断言 + 1 DCHECK + 类声明 ────────────────────────
const FNS = [
  ["static constexpr intptr_t ObjectStartOffsetInCodePage()", 18],
  ["static constexpr size_t AllocatableMemoryInCodePage()", 26],
  ["static constexpr size_t ObjectStartOffsetInDataPage()", 30],
  ["static constexpr size_t AllocatableMemoryInDataPage()", 35],
  ["static constexpr size_t ObjectStartOffsetInMemoryChunk(", 42],
  ["static constexpr size_t AllocatableMemoryInMemoryChunk(", 51],
  ["static constexpr int MaxRegularCodeObjectSize()", 61],
];
for (const [sig, ln] of FNS) {
  if (count(src, new RegExp(sig.replace(/[().]/g, (c) => "\\" + c))) !== 1) fail(`函数锚 ${sig}`, 0, 1);
  const i = src.indexOf(sig);
  const gotLine = i < 0 ? -1 : src.slice(0, i).split(String.fromCharCode(10)).length;
  if (gotLine !== ln) fail(`函数行号 ${sig}`, gotLine, ln);
  n += 2;
}
for (const [label, re, want] of [
  ["内断言 kMaxRegularHeapObjectSize", /static_assert\(kMaxRegularHeapObjectSize <= kAllocatableMemoryInDataPage\);/, 1],
  ["内断言 kMaxRegularCodeObjectSize", /static_assert\(kMaxRegularCodeObjectSize <= kMaxRegularHeapObjectSize\);/, 1],
  ["DCHECK_NE CODE_LO_SPACE", /DCHECK_NE\(space, CODE_LO_SPACE\);/, 1],
  ["类声明", /class V8_EXPORT_PRIVATE MemoryChunkLayout final \{/, 1],
  ["RoundUp 消费", /RoundUp\(sizeof\(MemoryChunk\)/, 1],
  ["InstructionStream 依赖", /sizeof\(MemoryChunk\) \+ InstructionStream::kHeaderSize/, 1],
]) {
  if (count(src, re) !== want) fail(label, count(src, re), want);
  n++;
}

// ── ② 展开侧 (DEBUG 关: DCHECK_NE 剥除; static_assert 保留) ────────────────
const expanded = preprocess(REL);
const clsStart = expanded.indexOf("class MemoryChunkLayout final {");
if (clsStart < 0) fail("展开类体锚", clsStart, "found");
n++;
const clsEnd = expanded.indexOf("};", clsStart);
const region = expanded.slice(clsStart, clsEnd);
for (const [sig] of FNS) {
  const name = sig.replace("static constexpr ", "").split("(")[0];
  if (count(region, new RegExp(name.replace(/[()]/g, (c) => "\\" + c) + "\\(")) < 1) fail(`展开函数名 ${name}`, 0, ">=1");
  n++;
}
if (count(region, /DCHECK_NE/) !== 0) fail("展开后 DCHECK_NE 残留", count(region, /DCHECK_NE/g), 0);
n++;
if (count(region, /static_assert\(kMaxRegularHeapObjectSize/) !== 1) fail("展开内断言 1", 0, 1);
n++;
if (count(region, /static_assert\(kMaxRegularCodeObjectSize/) !== 1) fail("展开内断言 2", 0, 1);
n++;

// ── ③ rs 侧落地锚 (HC-1R2a: v8-heap-core/src/memory_chunk.rs 实态对数, 只增不减) ──
const rs = readFileSync(join(root, "v8-heap-core", "src", "memory_chunk.rs"), "utf8");
const RS_LAYOUT_FNS = [
  "object_start_offset_in_code_page", "allocatable_memory_in_code_page",
  "object_start_offset_in_data_page", "allocatable_memory_in_data_page",
  "object_start_offset_in_memory_chunk", "allocatable_memory_in_memory_chunk",
  "max_regular_code_object_size",
];
for (const name of RS_LAYOUT_FNS) {
  if (count(rs, new RegExp(`pub const fn ${name}\\(`, "g")) !== 1) fail(`rs 布局函数 ${name}`, 0, 1);
  n++;
}
for (const [label, re, want] of [
  ["IStream 头注入常量 (推迟台账 10)", /pub const K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED: usize = 16/, 1],
  ["layout:21 双引用", /layout:21/, 1],
  ["layout:38 断言代入", /K_MAX_REGULAR_HEAP_OBJECT_SIZE <= MemoryChunkLayout::allocatable_memory_in_data_page/, 1],
  ["layout:64 断言代入", /max_regular_code_object_size\(K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED\)/, 1],
  ["MemoryChunkLayout 类型锚", /pub struct MemoryChunkLayout/, 1],
]) {
  if (count(rs, re) !== want) fail(`rs 锚 ${label}`, count(rs, re), want);
  n++;
}

const skel = `${banner("src/heap/memory-chunk-layout.h.js", REL, "MemoryChunkLayout 7 函数台账; 布局数值链 + InstructionStream 依赖推迟")
}
// ══ MemoryChunkLayout (原版 memory-chunk-layout.h:14-67) → v8-heap-core/src/memory_chunk.rs ══
${FNS.map(([sig, ln]) => `// layout:${String(ln).padEnd(4)} ${sig}`).join("\n")}
// 目标配置数值链 (Rust 侧 const 计算 + tests 布局金标准逐项断言):
//   sizeof(MemoryChunk) = 16 (repr(C): flags usize@0 + metadata_index u32@8, pad 至 16)
//   ObjectStartOffsetInDataPage = RoundUp(16, ALIGN_TO_ALLOCATION_ALIGNMENT(8)) = 16
//   AllocatableMemoryInDataPage = 2^18 - 16 = 262128; 断言 kMaxRegularHeapObjectSize(131072) <= 262128 ✓
//   ObjectStartOffsetInCodePage = RoundUp(16 + IStream头, kCodeAlignment) - IStream头
//     — InstructionStream::kHeaderSize 属 objects/instruction-stream.h (他票): 目标配置推导值 16
//       (TrustedObject 4 + 保护指针 ×2 各 4 + body_size u32 4, 无嵌入式常量池 x64/arm64);
//       Rust 侧以 const 参数注入 + 双引用推迟登记 (原版 layout:21 ↔ memory_chunk.rs 推迟台账)
//   MaxRegularCodeObjectSize = RoundDown(码页可分配/2, kTaggedSize) — 依赖上项, 同法注入
// DCHECK_NE(space, CODE_LO_SPACE) layout:53 — release 剥除 (展开实测 0), Rust 不复活
// ── rs 侧落地锚 (HC-1R2a 实测台账) ──
// Landed @ v8-heap-core/src/memory_chunk.rs (MemoryChunkLayout 区): 7 函数全落, 头尺寸参数注入式承接
//   (K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED=16, 推迟台账 10 双引用 layout:21);
//   测试面 tests/memory_chunk_unittest.rs::layout_fns_gold 逐项金标准 (16/262128/码页 cfg 双形态)。
`;
writeSrc("v8-heap-core", "memory_chunk_layout_h.skel", skel);
console.log(`OK memory-chunk-layout.h.js: 对数断言 ${n} 条全过; 7 函数逐名逐行号双校准 + 源码锚 6 + 展开侧 (类体 / 函数名 7 / DCHECK 剥除 0 / 内断言 2) + rs 侧落地锚 12 (函数 7 + 注入常量/双引用/断言代入/类型锚 5); 产物 v8-heap-core/src/memory_chunk_layout_h.skel`);
