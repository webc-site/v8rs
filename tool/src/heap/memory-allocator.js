#!/usr/bin/env bun
// tool/src/heap/memory-allocator.js
// 机械提取 v8/src/heap/memory-allocator.cc (族 .h 归他席, 读取仅作上下文):
// MemoryAllocator 方法清单 + 静态字段

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/heap/memory-allocator.cc");
const hSrc = readV8("src/heap/memory-allocator.h"); // 上下文 (本席表外)

const anchors = [
  [ccSrc, "size_t MemoryAllocator::commit_page_size_ = 0;"],
  [ccSrc, "size_t MemoryAllocator::commit_page_size_bits_ = 0;"],
  [ccSrc, "Address MemoryAllocator::AllocateAlignedMemory("],
  [ccSrc, "size_t MemoryAllocator::ComputeChunkSize(size_t area_size,"],
  [ccSrc, "MemoryAllocator::AllocateUninitializedChunkAt(BaseSpace* space,"],
  [ccSrc, "void MemoryAllocator::PartialFreeMemory(BasePage* chunk, Address start_free,"],
  [ccSrc, "void MemoryAllocator::UnregisterMemoryChunk(BasePage* base_page) {"],
  [ccSrc, "void MemoryAllocator::UnregisterMutableMemoryChunk(MutablePage* page) {"],
  [ccSrc, "void MemoryAllocator::UnregisterReadOnlyPage(ReadOnlyPage* page) {"],
  [ccSrc, "void MemoryAllocator::PreFreeMemory(MutablePage* chunk_metadata) {"],
  [hSrc, "class MemoryAllocator final {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// cc 方法清单真提取
const fns = [...new Set((ccSrc.match(/^[\w:<>&*]+ MemoryAllocator::(\w+)\(/gm) || [])
  .map((s) => s.match(/MemoryAllocator::(\w+)\(/)[1])
  .concat((ccSrc.match(/^void MemoryAllocator::(\w+)\(/gm) || []).map((s) => s.match(/::(\w+)\(/)[1])))];
if (fns.length < 25) throw new Error(`cc 方法计数 ${fns.length} < 25`);
console.log(`memory-allocator: cc 方法 ${fns.length}, 静态字段 2 (commit_page_size_/_bits_)`);

let rust = banner(
  "src/heap/memory-allocator.js",
  "src/heap/memory-allocator.cc (族 .h: src/heap/memory-allocator.h)",
  "内存分配器: 页/池的底层申请释放",
);

rust += `/// v8: src/heap/memory-allocator.cc MemoryAllocator 静态字段 (进程级)
pub static COMMIT_PAGE_SIZE: AtomicUsize = AtomicUsize::new(0);
/// v8: commit_page_size_bits_ = 0
pub static COMMIT_PAGE_SIZE_BITS: AtomicUsize = AtomicUsize::new(0);

/// v8: src/heap/memory-allocator.h MemoryAllocator
/// 本脚本覆盖 cc 实现面 (${fns.length} 方法), h 声明面归他席。
pub mod memory_allocator_cc {
  /// v8: AllocateAlignedMemory — 对齐申请 + 失败回退 optional
  /// v8: ComputeChunkSize(area_size, chunk_metadata) — 页头 + area 对齐
  /// v8: AllocateUninitializedChunkAt — 三分支 (pooled/常规/共享)
  /// v8: PartialFreeMemory — 部分释放 (commit 缩减)
  /// v8: UnregisterMemoryChunk — 分派 Mutable/ReadOnly 注销
  /// v8: UnregisterMutableMemoryChunk / UnregisterSharedMemoryChunk / UnregisterReadOnlyPage
  /// v8: FreeReadOnlyPage / FreeMemoryRegion / PreFreeMemory / PerformFreeMemory
  /// v8: GetPooledChunksCount / GetSharedPooledChunksCount / GetTotalPooledChunksCount
  /// v8: ReleasePooledChunksImmediately / TearDown / MemoryAllocator(构造)
  /// 共 ${fns.length} 个: ${fns.slice(0, 12).join(", ")} ...
}

impl MemoryAllocator {
  /// v8: src/heap/memory-allocator.cc MemoryAllocator::AllocateAlignedMemory
  pub fn allocate_aligned_memory(&mut self, size: usize, alignment: usize, executable: Executability, tag: PageTag) -> Option<Address> { todo!() }
  /// v8: MemoryAllocator::AllocateUninitializedChunkAt
  pub fn allocate_uninitialized_chunk_at(&mut self, space: &mut BaseSpace, chunk_size: usize, address_hint: Address, executable: Executability, heap: Heap) -> Option<MemoryChunkAllocationResult> { todo!() }
  /// v8: MemoryAllocator::PartialFreeMemory
  pub fn partial_free_memory(&mut self, chunk: &mut BasePage, start_free: Address, bytes_to_free: usize, new_area_end: Address) { todo!() }
  /// v8: MemoryAllocator::UnregisterMemoryChunk / UnregisterMutableMemoryChunk / UnregisterReadOnlyPage
  /// v8: MemoryAllocator::PreFreeMemory / FreeReadOnlyPage / FreeMemoryRegion
  /// v8: MemoryAllocator::TearDown / ReleasePooledChunksImmediately
}

// cc 方法对数: ${fns.length}
`;

const p = writeSrc("v8-heap-core", "memory_allocator.rs.skel", rust);
console.log(`written ${p}`);
