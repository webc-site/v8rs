#!/usr/bin/env bun
// tool/src/heap/allocation-stats.h.js — v8/src/heap/allocation-stats.h → v8-heap-core/src/allocation.rs (AllocationStats 区)
// 规则:
//   ① 方法全集 11 (含 DEBUG-only AllocatedOnPage) + 字段 4 + #ifdef DEBUG 守护 7 实测
//   ② clang -E 目标配置 (DEBUG 关) 展开侧: allocated_on_page_ 剥除 0 处 / atomic 字段 2 /
//      非 debug 方法 10 逐名在案 → Rust 侧 DEBUG 账目以 cfg(debug_assertions) 隔离
//   ③ BasePage 未转写 → 前向声明承接 (opaque 类型, C++ 形参只传指针的等价物)
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/allocation-stats.h";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-as.ii");
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

// ── ① 源码锚: 方法 11 + 字段 4 + DEBUG 守护 7 + allocated_on_page_ 7 ──────
const FNS = [
  ["operator=", /AllocationStats& operator=\(const AllocationStats& stats\)/, 1],
  ["Clear", /void Clear\(\) \{/, 1],
  ["ClearSize", /void ClearSize\(\) \{/, 1],
  ["Capacity", /size_t Capacity\(\) const \{ return capacity_; \}/, 1],
  ["MaxCapacity", /size_t MaxCapacity\(\) const \{ return max_capacity_; \}/, 1],
  ["Size", /size_t Size\(\) const \{ return size_; \}/, 1],
  ["AllocatedOnPage(DEBUG)", /size_t AllocatedOnPage\(const BasePage\* page\) const/, 1],
  ["IncreaseAllocatedBytes", /void IncreaseAllocatedBytes\(size_t bytes, const BasePage\* page\)/, 1],
  ["DecreaseAllocatedBytes", /void DecreaseAllocatedBytes\(size_t bytes, const BasePage\* page\)/, 1],
  ["DecreaseCapacity", /void DecreaseCapacity\(size_t bytes\) \{/, 1],
  ["IncreaseCapacity", /void IncreaseCapacity\(size_t bytes\) \{/, 1],
];
for (const [label, re, want] of FNS) {
  if (count(src, re) !== want) fail(`锚 ${label}`, count(src, re), want);
  n++;
}
for (const [label, re, want] of [
  ["atomic capacity_", /std::atomic<size_t> capacity_;/, 1],
  ["max_capacity_", /size_t max_capacity_;/, 1],
  ["atomic size_", /std::atomic<size_t> size_;/, 1],
  ["debug 页账目字段", /std::unordered_map<const BasePage\*, size_t, base::hash<const BasePage\*>>/, 1],
  ["DEBUG 守护", /#ifdef DEBUG/g, 7],
  ["allocated_on_page_ 引用", /allocated_on_page_/g, 8],
]) {
  const got = re.global ? count(src, re) : count(src, re);
  if (got !== want) fail(label, got, want);
  n++;
}

// ── ② 展开侧 (目标配置 DEBUG 关) ───────────────────────────────────────────
const expanded = preprocess(REL);
const clsStart = expanded.indexOf("class AllocationStats {");
if (clsStart < 0) fail("展开类体锚", clsStart, "found");
n++;
const region = expanded.slice(clsStart, expanded.indexOf("};", clsStart) + 2);
if (count(region, /allocated_on_page_/g) !== 0) fail("展开后 debug 账目残留", count(region, /allocated_on_page_/g), 0);
n++;
if (count(region, /AllocatedOnPage/g) !== 0) fail("展开后 AllocatedOnPage 残留", count(region, /AllocatedOnPage/g), 0);
n++;
if (count(region, /std::atomic<size_t>/g) !== 2) fail("展开 atomic 字段数", count(region, /std::atomic<size_t>/g), 2);
n++;
for (const m of ["IncreaseAllocatedBytes", "DecreaseAllocatedBytes", "DecreaseCapacity", "IncreaseCapacity", "Capacity", "MaxCapacity", "Size", "Clear", "ClearSize"]) {
  if (!region.includes(m)) fail(`展开缺方法 ${m}`, true, false);
  n++;
}
if (count(region, /const BasePage\* page/g) !== 2) fail("展开 BasePage 形参数 (增/减字节)", count(region, /const BasePage\* page/g), 2);
n++;

// ── ④ rs 侧落地锚 (HC-1R: v8-heap-core/src/allocation.rs AllocationStats 区, 只增不减) ──
// 区界: 从 AllocationStats 节头起 (排除 AllocationOrigin/AllocationResult 区, 与 allocation-result 脚本锚互不串台)。
const rs = readFileSync(join(root, "v8-heap-core", "src", "allocation.rs"), "utf8");
const rsStatsStart = rs.indexOf("AllocationStats (原版 allocation-stats.h");
if (rsStatsStart < 0) fail("rs AllocationStats 区界锚", rsStatsStart, "found");
n++;
const rsRegion = rs.slice(rsStatsStart);
for (const [label, re, want] of [
  ["结构声明", /pub struct AllocationStats \{/, 1],
  ["atomic capacity_ (h:103)", /capacity_: AtomicUsize,/, 1],
  ["max_capacity_ (h:106)", /max_capacity_: usize,/, 1],
  ["atomic size_ (h:109)", /size_: AtomicUsize,/, 1],
  ["DEBUG 页账目字段 (h:112)", /allocated_on_page_: HashMap<usize, usize>,/, 1],
  ["BasePage opaque 单点承接 (memory_chunk.rs)", /pub use crate::memory_chunk::BasePage;/, 1],
  ["ctor(默认 Clear) h:26", /pub fn new\(\) -> Self/, 1],
  ["operator= → assign h:28", /pub fn assign\(&mut self, stats: &Self\)/, 1],
  ["Clear h:39", /pub fn clear\(&mut self\)/, 1],
  ["ClearSize h:45", /pub fn clear_size\(&mut self\)/, 1],
  ["Capacity h:53", /pub fn capacity\(&self\) -> usize/, 1],
  ["MaxCapacity h:54", /pub fn max_capacity\(&self\) -> usize/, 1],
  ["Size h:55", /pub fn size\(&self\) -> usize/, 1],
  ["AllocatedOnPage h:57 (DEBUG 门)", /pub fn allocated_on_page\(&self, page: \*const BasePage\) -> Option<usize>/, 1],
  ["IncreaseAllocatedBytes h:62", /pub fn increase_allocated_bytes\(&mut self, bytes: usize, page: \*const BasePage\)/, 1],
  ["DecreaseAllocatedBytes h:75", /pub fn decrease_allocated_bytes\(&mut self, bytes: usize, page: \*const BasePage\)/, 1],
  ["DecreaseCapacity h:84", /pub fn decrease_capacity\(&mut self, bytes: usize\)/, 1],
  ["IncreaseCapacity h:90", /pub fn increase_capacity\(&mut self, bytes: usize\)/, 1],
  ["std::atomic 默认序 → SeqCst (全区)", /Ordering::SeqCst/, 13],
  ["增字节 fetch_add", /fetch_add\(bytes, Ordering::SeqCst\)/, 1],
  ["减字节 fetch_sub", /fetch_sub\(bytes, Ordering::SeqCst\)/, 1],
  ["h:63-64 DCHECK_IMPLIES 8GB 对齐面", /!V8_COMPRESS_POINTERS_8GB \|\| bytes\.is_multiple_of\(K_OBJECT_ALIGNMENT_8GB_HEAP\)/, 1],
  ["h:93-95 max_capacity_ 刷新式", /if new_capacity > self\.max_capacity_/, 1],
  ["h:70/79 页账目 entry (增+减)", /self\.allocated_on_page_\.entry\(page as usize\)/, 2],
  ["h:57 页账目查询", /\.get\(&\(page as usize\)\)\.copied\(\)/, 1],
  ["DEBUG 守护 #ifdef DEBUG(7) ↔ cfg(debug_assertions)(7)", /#\[cfg\(debug_assertions\)\]/, 7],
  ["DCHECK 面以 debug_assert 承接 (禁 panic)", /debug_assert!\(/, 7],
]) {
  const reG = new RegExp(re.source, "g");
  if (count(rsRegion, reG) !== want) fail(`rs 锚 ${label}`, count(rsRegion, reG), want);
  n++;
}

const RS_ARM = `
// ── rs 侧落地锚 (HC-1R 实测台账) ──
// Landed @ v8-heap-core/src/allocation.rs (AllocationStats 区, 27 锚):
//   11 方法 + ctor(new=Default→Clear 语义) 全落; 字段 4 (capacity_/size_ AtomicUsize, max_capacity_ usize,
//   allocated_on_page_ HashMap<usize,usize>); BasePage 形参统一引 memory_chunk.rs 单点 opaque 声明 (无重复定义);
//   原子序实测 Ordering::SeqCst ×13 — 承接 C++ std::atomic 默认序 (上文旧述 "relaxed" 以本锚为准订正);
//   DEBUG 账目实测 #[cfg(debug_assertions)] ×7 ↔ 原版 #ifdef DEBUG ×7 同数;
//   AllocatedOnPage 的 unordered_map::at 缺键异常面 → Option 承接 (本仓禁 panic, None 即原版缺账);
//   h:63-64 DCHECK_IMPLIES(8GB 对齐) 落 debug_assert, 票面所述 "align 方法" 本仓源中不存在 (属 DCHECK 面)。
`;

const skel = `${banner("src/heap/allocation-stats.h.js", REL, "AllocationStats 11 方法台账; DEBUG 账目 cfg(debug_assertions) 隔离; BasePage 前向声明承接")
}
// ══ AllocationStats (原版 allocation-stats.h:24-115) → v8-heap-core/src/allocation.rs ══
// 处置逐项 (原版行号实测):
//   ctor(默认 Clear) h:26 落地 / operator= h:28 落地(assign 法) / Clear h:39 落地 / ClearSize h:45 落地 /
//   Capacity h:53 落地 / MaxCapacity h:54 落地 / Size h:55 落地 /
//   AllocatedOnPage h:57 — DEBUG-only → cfg(debug_assertions) 门落地 (页账目 Vec 承接, 键为 BasePage 指针地址)
//   IncreaseAllocatedBytes h:62 / DecreaseAllocatedBytes h:75 — 落地 (fetch_add/sub, relaxed 对齐原版默认)
//   DecreaseCapacity h:84 / IncreaseCapacity h:90 落地
// 字段: capacity_ atomic h:103 / max_capacity_ h:106 / size_ atomic h:109 / allocated_on_page_ h:112 (DEBUG)
// BasePage 未转写 (heap/base-page.h 属他票) → 本 crate 内 opaque 前向声明 enum BasePage {} 承接指针形参
// 展开实测: DEBUG 关时 allocated_on_page_ 与 AllocatedOnPage 全剥除 (0 残留), atomic ×2 保留
${RS_ARM}`;
writeSrc("v8-heap-core", "allocation_stats_h.skel", skel);
console.log(`OK allocation-stats.h.js: 对数断言 ${n} 条全过 (原版/展开侧 31 + rs 落地锚 28); 方法锚 11 + 字段/守护锚 6 (DEBUG 守护 7 实测 / 引用 8 实测) + 展开侧 (剥除 0×2 / atomic 2 / 方法名 9 / BasePage 形参 2); rs 侧 12 方法/字段/式锚 25 + 区界 1 + cfg×7/SeqCst×13/debug_assert×7 同数锚 3; 产物 v8-heap-core/src/allocation_stats_h.skel`);
