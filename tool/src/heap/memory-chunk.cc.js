#!/usr/bin/env bun
// tool/src/heap/memory-chunk.cc.js — v8/src/heap/memory-chunk.cc → v8-heap-core/src/memory_chunk.rs (实体函数区)
// 规则:
//   ① 函数定义全集 10 (ctor / ClearMetadataPointer / MetadataTableIndex / SandboxSafeInReadOnlySpace /
//      InitializationMemoryFence / SynchronizedLoad(TSAN) / Offset(DEBUG) / OffsetMaybeOutOfRange(DEBUG))
//      + constexpr 掩码静态定义 8 + static_assert 2
//   ② 目标配置展开侧: TSAN/DEBUG 区剥除 → 保留定义 5; 三 cage 分支索引算术逐锚
//   ③ IsolateGroup/CodeRange/ReadOnlyPage 硬耦合 → MetadataTableEnv 窄 trait + 推迟双引用
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/memory-chunk.cc";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-mccc.ii");
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

// ── ① 源码锚 ───────────────────────────────────────────────────────────────
const DEFS = [
  ["MemoryChunk::MemoryChunk(MainThreadFlags flags, BasePage* metadata)", 46],
  ["void MemoryChunk::ClearMetadataPointer(BasePage* metadata)", 67],
  ["uint32_t MemoryChunk::MetadataTableIndex(Address chunk_address)", 82],
  ["bool MemoryChunk::SandboxSafeInReadOnlySpace() const", 112],
  ["void MemoryChunk::InitializationMemoryFence()", 134],
  ["void MemoryChunk::SynchronizedLoad() const", 165],
  ["size_t MemoryChunk::Offset(Address addr) const", 190],
  ["size_t MemoryChunk::OffsetMaybeOutOfRange(Address addr) const", 196],
];
for (const [sig] of DEFS) {
  if (count(src, new RegExp(sig.replace(/[().*+?^${}|\[\]\\]/g, (c) => "\\" + c))) !== 1) fail(`定义 ${sig}`, 0, 1);
  n++;
}
if (count(src, /constexpr MemoryChunk::MainThreadFlags/g) !== 7) fail("constexpr 掩码定义数", count(src, /constexpr MemoryChunk::MainThreadFlags/g), 7);
n++;
if (count(src, /static_assert\(kClearedWeakHeapObjectLower32 > 0\);/) !== 1) fail("静态断言 1", 0, 1);
n++;
if (count(src, /static_assert\(kClearedWeakHeapObjectLower32 < sizeof\(MemoryChunk\)\);/) !== 1) fail("静态断言 2", 0, 1);
n++;
if (count(src, /static_assert\(kPtrComprCageReservationSize == kPtrComprCageBaseAlignment\);/) !== 1) fail("cage 对齐断言", 0, 1);
n++;
// MetadataTableIndex 三分支算术锚
for (const [label, re, want] of [
  ["主 cage 分支", /V8HeapCompressionScheme::GetPtrComprCageBaseAddress\(chunk_address\) ==/, 1],
  ["trusted cage 分支", /GetTrustedPtrComprCage\(\)/, 1],
  ["code range 分支", /CodeRange\* code_range = IsolateGroup::current\(\)->GetCodeRange\(\);/, 1],
  ["index = 偏移 + 页序 (三处)", /\(offset >> kPageSizeBits\);/g, 3],
]) {
  if (count(src, re) !== want) fail(label, count(src, re), want);
  n++;
}

// ── ② 展开侧 (目标配置: TSAN 关 / DEBUG 关) ────────────────────────────────
const expanded = preprocess(REL);
if (count(expanded, /void MemoryChunk::SynchronizedLoad/g) !== 0) fail("展开后 TSAN 定义残留", 0, 0);
n++;
if (count(expanded, /size_t MemoryChunk::Offset/g) !== 0) fail("展开后 DEBUG Offset 残留", count(expanded, /size_t MemoryChunk::Offset/g), 0);
n++;
for (const [sig, want] of [
  ["MemoryChunk::MemoryChunk\\(MainThreadFlags", 1],
  ["void MemoryChunk::ClearMetadataPointer", 1],
  ["uint32_t MemoryChunk::MetadataTableIndex", 1],
  ["bool MemoryChunk::SandboxSafeInReadOnlySpace", 1],
  ["void MemoryChunk::InitializationMemoryFence", 1],
]) {
  if (count(expanded, new RegExp(sig, "g")) !== want) fail(`展开定义 ${sig}`, 0, want);
  n++;
}
if (count(expanded, /\(offset >> kPageSizeBits\);/g) !== 3) fail("展开三分支算术", count(expanded, /\(offset >> kPageSizeBits\);/g), 3);
n++;
if (count(expanded, /base::SeqCst_MemoryFence\(\);/) !== 1) fail("展开 fence 调用", 0, 1);
n++;

// ── ③ rs 侧落地锚 (HC-1R2a: v8-heap-core/src/memory_chunk.rs 实态对数, 只增不减) ──
const rs = readFileSync(join(root, "v8-heap-core", "src", "memory_chunk.rs"), "utf8");
for (const [label, re, want] of [
  ["with_metadata_index (cc:46 数据面)", /pub fn with_metadata_index/, 1],
  ["with_metadata_direct (cc:48-51 非沙箱承接位)", /pub fn with_metadata_direct/, 1],
  ["main_cage_metadata_index (cc:84-90)", /pub const fn main_cage_metadata_index/, 1],
  ["metadata_table_index (cc:82-110 三分支)", /pub fn metadata_table_index/, 1],
  ["MetadataTableEnv 窄 trait", /pub trait MetadataTableEnv/, 1],
  ["InitializationMemoryFence (cc:134-135)", /pub fn initialization_memory_fence/, 1],
  ["SynchronizedLoad (h:295 空实现)", /pub fn synchronized_load/, 1],
  ["Offset release 式 (h:279)", /pub fn offset\(&self, addr: Address\)/, 1],
  ["OffsetMaybeOutOfRange release 式 (h:280)", /pub fn offset_maybe_out_of_range/, 1],
  ["cc:86 cage 对齐断言代入", /K_PTR_COMPR_CAGE_RESERVATION_SIZE == K_PTR_COMPR_CAGE_BASE_ALIGNMENT/, 1],
  ["cc:25-26 清弱引用断言代入", /K_CLEARED_WEAK_HEAP_OBJECT_LOWER32/, 1],
  ["ClearMetadataPointer 不落地 (推迟台账 4)", /pub fn clear_metadata_pointer/, 0],
  ["SandboxSafeInReadOnlySpace 不落地 (推迟台账 5)", /pub fn sandbox_safe_in_read_only_space/, 0],
  ["推迟台账双行号区 (cc 侧引用)", /cc:53-62/, 1],
]) {
  if (count(rs, re) !== want) fail(`rs 锚 ${label}`, count(rs, re), want);
  n++;
}

const skel = `${banner("src/heap/memory-chunk.cc.js", REL, "MemoryChunk 实体台账: 8 定义 (目标配置留 5) + 掩码定义 7 + 断言 4 + 三分支索引")
}
// ══ memory-chunk.cc 处置台账 (原版行号实测; 展开侧=目标配置) ══
${DEFS.map(([sig, ln]) => `// cc:${String(ln).padEnd(4)} ${sig}`).join("\n")}
// 展开实测: SynchronizedLoad (THREAD_SANITIZER, cc:163-186) 与 Offset/OffsetMaybeOutOfRange
//   (DEBUG, cc:188-201) 在目标配置剥除 → Rust: synchronized_load 为空实现 (h:295 非 TSAN 分支),
//   offset 取 release 式 addr - address(); DEBUG DCHECK 版登记推迟 (依赖 Metadata area_start/size)
// constexpr 掩码静态定义 cc:29-44 ×7 → Rust pub const 掩码即定义处, 无需再落
// ctor cc:46: sandbox 分支写 metadata_pointer_table (IsolateGroup 耦合) →
//   Rust 落数据面 with_metadata_index; 表写入走 MetadataTableEnv trait (cc:54-61 双引用在模块)
// MetadataTableIndex cc:82-110: 三分支 (主 cage / trusted cage / code range) →
//   Rust 泛型 fn metadata_table_index<E: MetadataTableEnv>; 主 cage 纯算术另落
//   main_cage_metadata_index 供无 env 消费与测试
// SandboxSafeInReadOnlySpace cc:112-130: ReadOnlyPage 反查 (读只页属他票) → 推迟双引用
// ClearMetadataPointer cc:67-79: 表读写 + Isolate::FromHeap → 推迟双引用 (trait 面在模块登记)
// ── rs 侧落地锚 (HC-1R2a 实测台账) ──
// Landed @ v8-heap-core/src/memory_chunk.rs: 数据面 ctor/三分支索引/fence/空 SynchronizedLoad/offset release 式全落;
//   推迟台账 3-6 双引用本脚本展开侧剥除清单 (表写入/clear/RO 反查/DEBUG Offset)。
`;
writeSrc("v8-heap-core", "memory_chunk_cc.skel", skel);
console.log(`OK memory-chunk.cc.js: 对数断言 ${n} 条全过; 定义 8 逐名 + 掩码 7 + 断言 4 + 三分支锚 4 + 展开侧 (TSAN/DEBUG 剥除 / 保留 5 / 算术 3 / fence 1) + rs 侧落地锚 14 (含 0 检推迟臂 2); 产物 v8-heap-core/src/memory_chunk_cc.skel`);
