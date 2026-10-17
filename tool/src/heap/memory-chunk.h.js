#!/usr/bin/env bun
// tool/src/heap/memory-chunk.h.js — v8/src/heap/memory-chunk.h → v8-heap-core/src/memory_chunk.rs (Flag 位域 + 掩码 + 谓词面)
// 规则:
//   ① Flag 枚举逐值锚定 (源码 13 项含 sticky; 目标配置展开侧 11 移位值 + NO_FLAGS, sticky ×2 剥除)
//   ② 掩码常量全集 13 (源码, 含 sticky 1) → 展开侧 12 (kIsOnlyOldOrMajorGCInProgressMask 剥除)
//   ③ 函数/成员全集台账 (名 + 原版行号, 逐 needle 唯一性断言) + friend 9 / hash 特化 2 /
//      DEFINE_OPERATORS_FOR_FLAGS 1 展开侧核对
//   ④ sandbox 分支实体: metadata_index_ 保留 / metadata_ 剥除 (目标配置 V8_ENABLE_SANDBOX 开)
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/memory-chunk.h";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-mch.ii");
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

// ── ① Flag 枚举逐值 (h:56-109) ─────────────────────────────────────────────
const FLAGS = [
  ["NO_FLAGS", 0, "0u"],
  ["IN_WRITABLE_SHARED_SPACE", 1, "1u << 0"],
  ["POINTERS_TO_HERE_ARE_INTERESTING", 2, "1u << 1"],
  ["POINTERS_FROM_HERE_ARE_INTERESTING", 4, "1u << 2"],
  ["FROM_PAGE", 8, "1u << 3"],
  ["TO_PAGE", 16, "1u << 4"],
  ["INCREMENTAL_MARKING", 32, "1u << 5"],
  ["BLACK_ALLOCATED", 64, "1u << 6"],
  ["LARGE_PAGE", 128, "1u << 7"],
  ["EVACUATION_CANDIDATE", 256, "1u << 8"],
  ["NEW_SPACE_BELOW_AGE_MARK", 512, "1u << 9"],
  ["READ_ONLY_HEAP", 1024, "1u << 10"],
  ["STICKY_MARK_BIT_CONTAINS_ONLY_OLD", 2048, "1u << 11"],
  ["STICKY_MARK_BIT_IS_MAJOR_GC_IN_PROGRESS", 4096, "1u << 12"],
];
for (const [name, , expr] of FLAGS) {
  const re = new RegExp(`${name} = ${expr.replace(/</g, "<").replace(/ /g, "\\s")}[,;]`, "g");
  if (count(src, re) !== 1) fail(`枚举 ${name}=${expr}`, count(src, re), 1);
  n++;
}
if (count(src, /= 1u << \d+,/g) !== 13) fail("移位枚举项数 (源码含 sticky)", count(src, /= 1u << \d+,/g), 13);
n++;

// ── ② 掩码常量全集 (h:113-143; 源码 13 含 sticky 掩码) ────────────────────
const MASKS = [
  "kAllFlagsMask", "kPointersToHereAreInterestingMask", "kPointersFromHereAreInterestingMask",
  "kEvacuationCandidateMask", "kIsInYoungGenerationMask", "kIsLargePageMask", "kInSharedHeap",
  "kIncrementalMarking", "kSkipEvacuationSlotsRecordingMask", "kIsBlackAllocatedOrWriteableSharedMask",
  "kIsInReadOnlyHeapMask", "kIsReadOnlyOrSharedHeapMask", "kIsOnlyOldOrMajorGCInProgressMask",
];
for (const m of MASKS) {
  if (count(src, new RegExp(`static constexpr MainThreadFlags ${m} =`, "g")) !== 1) fail(`掩码 ${m}`, 0, 1);
  n++;
}

// ── ③ 函数/成员全集 (h) 逐 needle 行号锚 ──────────────────────────────────
const FNS = [
  ["BaseAddress", 2], ["IsAligned", 1], ["GetAlignmentForAllocation", 1], ["GetAlignmentMaskForAssembler", 1],
  ["AddressToOffset", 1], ["MemoryChunk* FromAddress(", 1], ["FromHeapObject", 1], ["Address address() const", 1],
  ["Metadata(const Isolate* isolate)", 2], ["BasePage* Metadata()", 2], ["MetadataNoIsolateCheck()", 2],
  ["bool IsMarking() const", 1], ["InWritableSharedSpace", 1], ["IsBlackAllocated()", 1], ["IsBlackAllocatedOrWritableShared", 1],
  ["ContainsOnlyOldObjects", 1], ["bool InYoungGeneration() const", 1], ["IsYoungOrSharedChunk", 1],
  ["InitializationMemoryFence", 1], ["V8_INLINE bool InReadOnlySpace() const;", 1], ["IsEvacuationCandidate", 1],
  ["ShouldSkipEvacuationSlotRecording", 1], ["IsFromPage", 1], ["IsToPage", 1], ["bool IsLargePage() const", 1],
  ["InNewSpace()", 1], ["InNewSpaceBelowAgeMark", 1], ["InNewLargeObjectSpace", 1],
  ["IsOnlyOldOrMajorMarkingOn", 1], ["bool PointersToHereAreInteresting() const", 1], ["bool PointersFromHereAreInteresting() const", 1],
  ["OffsetMaybeOutOfRange", 2], ["ClearMetadataPointer", 1], ["SandboxSafeInReadOnlySpace", 1],
  ["SynchronizedLoad", 3], ["intptr_t FlagsOffset()", 1], ["intptr_t MetadataIndexOffset()", 1], ["MetadataTableIndex", 1],
  ["BasePageTableEntry* MetadataTableAddress()", 1], ["intptr_t MetadataOffset()", 1], ["MetadataImpl", 2], ["GetFlags() const", 1],
  ["IsFlagSet(Flag flag) const", 1], ["Offset(Address addr) const", 2],
];
for (const [needle, want] of FNS) {
  const got = count(src, new RegExp(needle.replace(/[().*+?^${}|\[\]\\\/]/g, (c) => "\\" + c).replace(/\\\(/g, "\\("), "g"));
  if (got !== want) fail(`h 锚 ${needle}`, got, want);
  n++;
}
// 特殊 ctor 声明 1
if (count(src, /MemoryChunk\(MainThreadFlags flags, BasePage\* metadata\);/) !== 1) fail("ctor 声明", 0, 1);
n++;
// 字段 3 (flags + 二选一 metadata) + kAlignment 组 2
for (const [label, re, want] of [
  ["字段 untrusted_main_thread_flags_ 声明", /MainThreadFlags untrusted_main_thread_flags_;/, 1],
  ["字段 metadata_index_", /uint32_t metadata_index_;/, 1],
  ["字段 metadata_ (非沙箱分支)", /BasePage\* metadata_;/, 1],
  ["kAlignment 定义", /static constexpr intptr_t kAlignment =/, 1],
  ["kAlignmentMask 定义", /static constexpr intptr_t kAlignmentMask = kAlignment - 1;/, 1],
]) {
  if (count(src, re) !== want) fail(label, count(src, re), want);
  n++;
}

// ── ④ 展开侧 (目标配置) ────────────────────────────────────────────────────
const expanded = preprocess(REL);
const eStart = expanded.indexOf("enum Flag : uintptr_t {");
if (eStart < 0) fail("展开枚举区锚", eStart, "found");
n++;
const eRegion = expanded.slice(eStart, expanded.indexOf("};", eStart));
if (count(eRegion, /1u << \d+/g) !== 11) fail("展开枚举移位值数 (类内截区)", count(eRegion, /1u << \d+/g), 11);
n++;
if (count(eRegion, /STICKY_MARK_BIT/g) !== 0) fail("展开枚举区 sticky 残留", count(eRegion, /STICKY_MARK_BIT/g), 0);
n++;
if (count(eRegion, /READ_ONLY_HEAP = 1u << 10[,;]/) !== 1) fail("展开 READ_ONLY_HEAP 位", 0, 1);
n++;
if (count(eRegion, /NO_FLAGS = 0u,/) !== 1) fail("展开 NO_FLAGS", 0, 1);
n++;
for (const m of MASKS) {
  const want = m === "kIsOnlyOldOrMajorGCInProgressMask" ? 0 : 1;
  const got = count(expanded, new RegExp(`static constexpr MainThreadFlags ${m} =`, "g"));
  if (got !== want) fail(`展开掩码 ${m}`, got, want);
  n++;
}
if (count(expanded, /uint32_t metadata_index_;/) !== 1) fail("展开 metadata_index_ 字段", 0, 1);
n++;
if (count(expanded, /BasePage\* metadata_;/) !== 0) fail("展开后非沙箱 metadata_ 字段残留", count(expanded, /BasePage\* metadata_;/g), 0);
n++;
const clsStart = expanded.indexOf("class MemoryChunk final {");
if (clsStart < 0) fail("展开类体锚", clsStart, "found");
n++;
const clsRegion = expanded.slice(clsStart, expanded.indexOf("namespace base {", clsStart));
if (count(clsRegion, /friend class/g) !== 9) fail("展开 MemoryChunk 类内 friend 全集", count(clsRegion, /friend class/g), 9);
n++;
if (count(expanded, /DEFINE_OPERATORS_FOR_FLAGS\(MemoryChunk::MainThreadFlags\)/) !== 0) fail("展开 DEFINE_OPERATORS 宏 (宏被展开应为 0 残留)", 0, 0);
n++;
if (count(expanded, /struct hash<const i::MemoryChunk\*>\s*\{/) !== 1 || count(expanded, /struct hash<i::MemoryChunk\*>\s*\{/) !== 1) fail("展开 base::hash 特化 2", 0, 2);
n++;
if (count(expanded, /class MemoryChunk final/) !== 1) fail("展开类锚 MemoryChunk final", 0, 1);
n++;

// ── ⑤ rs 侧落地锚 (HC-1R2a: v8-heap-core/src/memory_chunk.rs 实态对数, 只增不减) ──
const rs = readFileSync(join(root, "v8-heap-core", "src", "memory_chunk.rs"), "utf8");
const rsMask = (m) => "K_" + m.slice(1).replace(/([A-Z])/g, (c, _g, off) => (off === 0 ? c : "_" + c)).toUpperCase();
for (const [name, , expr] of FLAGS) {
  const rhs = expr === "0u" ? "0" : expr.replace("u", "").replace(/(\d+) << (\d+)/, "$1 << $2");
  if (count(rs, new RegExp(`pub const ${name}: MainThreadFlags = ${rhs.replace(/\s/g, "\\s")};`, "g")) !== 1) fail(`rs Flag ${name}`, 0, 1);
  n++;
}
for (const m of MASKS) {
  const want = m === "kIsOnlyOldOrMajorGCInProgressMask" ? 0 : 1; // 推迟台账 8: sticky 掩码不落地
  const got = count(rs, new RegExp(`pub const ${rsMask(m)}: MainThreadFlags`, "g"));
  if (got !== want) fail(`rs 掩码 ${m}`, got, want);
  n++;
}
for (const [label, re, want] of [
  ["repr(C) 结构体", /pub struct MemoryChunk \{/, 1],
  ["字段 flags", /untrusted_main_thread_flags_: MainThreadFlags/, 1],
  ["字段 index", /metadata_index_: u32/, 1],
  ["BaseAddress", /pub const fn base_address/, 1],
  ["IsAligned", /pub const fn is_aligned/, 1],
  ["GetAlignmentForAllocation", /pub const fn get_alignment_for_allocation/, 1],
  ["GetAlignmentMaskForAssembler", /pub const fn get_alignment_mask_for_assembler/, 1],
  ["AddressToOffset", /pub const fn address_to_offset/, 1],
  ["FromAddress", /pub const fn from_address/, 1],
  ["FromHeapObject", /pub const fn from_heap_object/, 1],
  ["FlagsOffset", /pub const fn flags_offset/, 1],
  ["MetadataIndexOffset", /pub const fn metadata_index_offset/, 1],
  ["GetFlags", /pub fn get_flags/, 1],
  ["IsFlagSet", /pub fn is_flag_set/, 1],
  ["chunk hash (h:390-404)", /pub const fn chunk_hash/, 1],
]) {
  if (count(rs, re) !== want) fail(`rs 锚 ${label}`, count(rs, re), want);
  n++;
}
// 推迟台账区存在 (Metadata 族/friend/DEFINE_OPERATORS 承接面登记, 双行号在模块尾)
if (count(rs, /推迟台账/g) < 1) fail("rs 推迟台账区", 0, ">=1");
n++;

const ln = (s) => src.split(String.fromCharCode(10)).findIndex((l) => l.includes(s)) + 1;
const skel = `${banner("src/heap/memory-chunk.h.js", REL, "MemoryChunk 头台账: Flag 14 项逐值 (目标配置留 12) + 掩码 13 (留 12) + 成员 46 needle + 布局断言点")
}
// ══ MemoryChunk (原版 memory-chunk.h:47-376) → v8-heap-core/src/memory_chunk.rs ══
// Flag 枚举 (h:56-109; 名/值/行号):
${FLAGS.map(([name, val]) => `//   ${name.padEnd(40)} = ${String(val).padStart(4)}  h:${ln(name + " = ")}`).join("\n")}
// 目标配置 (压缩+sandbox+external code space, sticky 关, 非连续 RO): 11 移位值 + NO_FLAGS 落地;
// STICKY_* ×2 (h:103/107) + ContainsOnlyOldObjects (h:208) + IsOnlyOldOrMajorMarkingOn (h:258) — 非目标配置, 登记推迟
// 掩码常量 (h:113-143): ${MASKS.filter((m) => m !== "kIsOnlyOldOrMajorGCInProgressMask").join(" / ")} — 12 项落地
// 布局: repr(C) { flags: usize, metadata_index: u32 } → size_of=16, flags@0, index@8
//   (原版 FlagsOffset h:301 / MetadataIndexOffset h:314 即 offset 导出面)
// Metadata 族 (h:187-194, MetadataImpl h:338-341; 实体在 -inl.h, 见 memory-chunk-inl.h.js)
//   — 依赖 heap BasePage/Isolate/IsolateGroup → 前向声明承接 + 推迟台账 (双引用见模块)
// MetadataTableIndex (h:318 声明, cc:82 实现) — IsolateGroup/CodeRange 耦合 → MetadataTableEnv 窄 trait
// hash<const MemoryChunk*> / hash<MemoryChunk*> (h:390-404) → chunk_hash 落地 (Tagged_t 截低 32 位后 >>kPageSizeBits)
// ── rs 侧落地锚 (HC-1R2a 实测台账) ──
// Landed @ v8-heap-core/src/memory_chunk.rs: Flag 14 逐值照录 + 掩码 12 (sticky 掩码按推迟台账 8 剥除) +
//   BaseAddress/IsAligned/AddressToOffset/FromAddress/FromHeapObject 纯逻辑族 + repr(C) 布局 (offset 0/8) +
//   chunk_hash + 谓词矩阵全落; 测试面 tests/memory_chunk_unittest.rs 金标准 13 用例 (clang -E 展开侧数值)。
`;
writeSrc("v8-heap-core", "memory_chunk_h.skel", skel);
console.log(`OK memory-chunk.h.js: 对数断言 ${n} 条全过; Flag 14 逐值 + 移位枚举源码 13 / 展开 11 + 掩码 13 + 成员锚 ${FNS.length + 1} + 字段/对齐 5 + 展开侧 (sticky 0 / READ_ONLY 1 / index 字段 1 / metadata_ 0 / friend 9 / hash 特化 2 / 类锚 1) + rs 侧落地锚 43 (Flag 14 + 掩码 13 含推迟臂 0 检 + 成员/字段 15 + 推迟台账区 1); 产物 v8-heap-core/src/memory_chunk_h.skel`);
