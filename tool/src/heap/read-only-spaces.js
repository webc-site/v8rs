#!/usr/bin/env bun
// tool/src/heap/read-only-spaces.js
// 机械提取 v8/src/heap/read-only-spaces.cc (族 .h 归他席, 读取仅作上下文):
// ReadOnlyPage / ReadOnlyArtifacts / ReadOnlySpace 方法清单

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/heap/read-only-spaces.cc");
const hSrc = readV8("src/heap/read-only-spaces.h"); // 上下文 (本席表外)

const anchors = [
  [ccSrc, "ReadOnlyArtifacts::~ReadOnlyArtifacts() {"],
  [ccSrc, "void ReadOnlyArtifacts::Initialize(Isolate* isolate,"],
  [ccSrc, "void ReadOnlyArtifacts::ReinstallReadOnlySpace(Isolate* isolate) {"],
  [ccSrc, "void ReadOnlyArtifacts::VerifyHeapAndSpaceRelationships(Isolate* isolate) {"],
  [ccSrc, "void ReadOnlyArtifacts::InitializeChecksum("],
  [ccSrc, "void ReadOnlyArtifacts::VerifyChecksum(SnapshotData* read_only_snapshot_data,"],
  [ccSrc, "ReadOnlySpace::ReadOnlySpace(Heap* heap) : BaseSpace(heap, RO_SPACE) {}"],
  [ccSrc, "void SharedReadOnlySpace::TearDown(MemoryAllocator* memory_allocator) {"],
  [ccSrc, "void ReadOnlySpace::TearDown(MemoryAllocator* memory_allocator) {"],
  [ccSrc, "void ReadOnlySpace::DetachPagesAndAddToArtifacts(ReadOnlyArtifacts* artifacts) {"],
  [ccSrc, "ReadOnlyPage::ReadOnlyPage(Heap* heap, BaseSpace* space, size_t chunk_size,"],
  [hSrc, "class ReadOnlyPage final : public BasePage {"],
  [hSrc, "class ReadOnlyArtifacts final {"],
  [hSrc, "class ReadOnlySpace : public BaseSpace {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

const nArt = (ccSrc.match(/ReadOnlyArtifacts::(\w+)\(/g) || []).length;
const nSpace = (ccSrc.match(/(?:^|\s)ReadOnlySpace::(\w+)\(/g) || []).length;
const nShared = (ccSrc.match(/SharedReadOnlySpace::(\w+)\(/g) || []).length;
const nPage = (ccSrc.match(/ReadOnlyPage::(\w+)\(/g) || []).length;
if (nArt < 5 || nSpace < 3 || nPage < 3 || nShared < 1) throw new Error(`cc 计数漂移: art=${nArt} space=${nSpace} shared=${nShared} page=${nPage}`);
console.log(`read-only-spaces: cc 方法 art=${nArt} space=${nSpace} shared=${nShared} page=${nPage}, 类 3`);

let rust = banner(
  "src/heap/read-only-spaces.js",
  "src/heap/read-only-spaces.cc (族 .h: src/heap/read-only-spaces.h)",
  "只读页/空间与跨 isolate artifacts",
);

rust += `/// v8: src/heap/read-only-spaces.h ReadOnlyPage (BasePage 派生)
/// 构造: (heap, space, chunk_size, area_start, area_end, virtual_memory, executable)
/// v8: ReadOnlyPage::ReadOnlyPage (cc) 及页释放路径

/// v8: src/heap/read-only-spaces.h ReadOnlyArtifacts
/// 持有脱离的只读页集合, 供后续 isolate 重新挂载;
/// 含 checksum 校验 (InitializeChecksum / VerifyChecksum) 与
/// shared 状态 (ReinstallReadOnlySpace / VerifyHeapAndSpaceRelationships)。
/// cc 方法 ${nArt} 个。

/// v8: src/heap/read-only-spaces.h ReadOnlySpace (BaseSpace(RO_SPACE))
/// cc 方法 ${nSpace} 个 (构造 BaseSpace(heap, RO_SPACE) / TearDown /
/// DetachPagesAndAddToArtifacts / AttachPageToSpace / RemovePage /
/// ShrinkPages / ReleaseFreeMemory / SetPermissionsForPages 等)
/// v8: SharedReadOnlySpace::TearDown (cc, ${nShared} 个)

/// v8: src/heap/read-only-spaces.h kReadOnlySpaceArtifacts 共享面
`;

const p = writeSrc("v8-heap-core", "read_only_spaces.rs.skel", rust);
console.log(`written ${p}`);
