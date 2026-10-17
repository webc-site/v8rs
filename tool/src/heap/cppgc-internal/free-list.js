#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/free-list.js — v8/src/heap/cppgc-internal/free-list.cc (+ .h 对数源) → v8-heap-cppgc/src/free_list.rs.skel
// cppgc 自由链: 桶序 = log2(bit_floor(size)), Entry 复用 HeapObjectHeader, 惰性 filler
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/free-list.cc";
const REL_H = "src/heap/cppgc-internal/free-list.h";
const src = readV8(REL);
const srcH = readV8(REL_H);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

// ── ① .h: 类面 ────────────────────────────────────────────────────────
need(srcH, "class Filler : public HeapObjectHeader {", "Filler 声明");
need(srcH, "struct Block {\n    void* address;\n    size_t size;\n  };", "Block 结构");
need(srcH, "Block Allocate(size_t);", "Allocate 声明");
need(srcH, "std::pair<Address, Address> AddReturningUnusedBounds(Block);", "AddReturningUnusedBounds 声明");
need(srcH, "void Append(FreeList&&);", "Append 声明");
need(srcH, "std::array<Entry*, kPageSizeLog2> free_list_heads_;", "heads 桶数组");
need(srcH, "std::array<Entry*, kPageSizeLog2> free_list_tails_;", "tails 桶数组");
need(srcH, "size_t biggest_free_list_index_ = 0;", "biggest_free_list_index_");
// ── ② .cc: 桶函数 + Entry ─────────────────────────────────────────────
need(src, "uint32_t BucketIndexForSize(uint32_t size) {", "桶函数");
need(src, "v8::base::bits::WhichPowerOfTwo(std::bit_floor(size));", "bit_floor→幂");
need(src, "class FreeList::Entry : public HeapObjectHeader {", "Entry 声明");
need(src, "static_assert(sizeof(Entry) == kFreeListEntrySize, \"Sizes must match\");", "Entry 尺寸断言");
need(src, "explicit Entry(size_t size) : HeapObjectHeader(size, kFreeListGCInfoIndex) {", "Entry 构造");
need(src, "void Link(Entry** previous_next) {", "Link");
need(src, "void Unlink(Entry** previous_next) {", "Unlink");
need(src, "auto& filler = Filler::CreateAt(block.address, size);", "小件 filler");
need(src, "DCHECK_GT(kPageSize, size);", "页内上限断言");
need(src, "biggest_free_list_index_ = std::max(biggest_free_list_index_, index);", "最大桶更新");
need(src, "size_t bucket_size = static_cast<size_t>(1) << biggest_free_list_index_;", "桶尺寸推导");
need(src, "if (allocation_size > bucket_size) {", "末桶容量判定");
need(src, "entry->AllocatedSize() < allocation_size) break;", "末桶线性拒绝");
need(src, "other.free_list_heads_[index] = nullptr;", "Append 清源");
need(src, "std::fill(free_list_heads_.begin(), free_list_heads_.end(), nullptr);", "Clear");
need(src, "void FreeList::CollectStatistics(\n    HeapStatistics::FreeListStatistics& free_list_stats) {", "CollectStatistics 实体");
// 方法实体计数: 构造2 + operator=1 + Add1 + AddReturningUnusedBounds1 + Append1 + Allocate1 + Clear1 + Size1 + IsEmpty1 + ContainsForTesting1 + IsConsistent1 + CollectStatistics1 = 13
const defs = (src.match(/^void FreeList::|^std::pair<Address, Address> FreeList::|^FreeList::Block FreeList::|^size_t FreeList::|^bool FreeList::|^FreeList& FreeList::|^FreeList::FreeList\(/gm) || []).length;
if (defs !== 13) throw new Error(`FreeList 方法定义计数失配: ${defs} want 13`);
n += defs;

const body = `${banner("src/heap/cppgc-internal/free-list.js", `${REL} + ${REL_H}`, "2^k 桶自由链; Entry 挂 HeapObjectHeader")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/free-list.cc BucketIndexForSize
/// 桶号 = WhichPowerOfTwo(bit_floor(size)) — 桶 k 的最小条目尺寸 2^k
pub fn bucket_index_for_size(size: u32) -> u32 {
    // bit_floor: 保留最高位; WhichPowerOfTwo → trailing_zeros
    (size & size.wrapping_neg()).trailing_zeros()
}

/// v8: src/heap/cppgc-internal/free-list.h FreeList::Block
#[derive(Clone, Copy)]
pub struct Block {
    pub address: *mut u8,
    pub size: usize,
}

/// v8: src/heap/cppgc-internal/free-list.cc FreeList::Entry
/// 头即 HeapObjectHeader(size, kFreeListGCInfoIndex=0) + next 指针; sizeof == 2*usize。
struct Entry {
    header: HeapObjectHeader,
    next: *mut Entry,
}

/// v8: src/heap/cppgc-internal/free-list.h FreeList
pub struct FreeList {
    /// v8: free_list_heads_ [kPageSizeLog2]
    heads: [*mut Entry; K_PAGE_SIZE_LOG2],
    /// v8: free_list_tails_ [kPageSizeLog2]
    tails: [*mut Entry; K_PAGE_SIZE_LOG2],
    /// v8: biggest_free_list_index_ = 0
    biggest_free_list_index: usize,
}

impl FreeList {
    /// v8: src/heap/cppgc-internal/free-list.cc FreeList::Add → AddReturningUnusedBounds
    /// size < sizeof(Entry)(2 字) → Filler 占位, 返回 (filler_end, filler_end);
    /// 否则建 Entry 入桶 (桶号 BucketIndexForSize), 更新 biggest, 返回 (payload_start, address+size)。
    pub fn add_returning_unused_bounds(&mut self, block: Block) -> (*mut u8, *mut u8) {
        debug_assert!(K_PAGE_SIZE > block.size);
        debug_assert!(core::mem::size_of::<HeapObjectHeader>() <= block.size);
        todo!("Entry::CreateAt + Link + tail/biggest 维护")
    }

    /// v8: src/heap/cppgc-internal/free-list.cc FreeList::Allocate
    /// 自最大桶向下找; 末桶 (allocation_size > bucket_size) 只查首条目, 不线性扫;
    /// 命中即 Unlink 并返回 {entry, entry->AllocatedSize()}, 未命中 {null, 0}。
    pub fn allocate(&mut self, allocation_size: usize) -> Block {
        todo!("自顶向下桶扫描")
    }

    /// v8: src/heap/cppgc-internal/free-list.cc FreeList::Append
    /// 逐桶: 对方 tail 接我方 head; 我方空尾则领养对方尾; 清空对方; biggest 取 max。
    pub fn append(&mut self, other: &mut FreeList) { todo!("逐桶拼接") }

    /// v8: src/heap/cppgc-internal/free-list.cc FreeList::Clear
    pub fn clear(&mut self) {
        self.heads.fill(core::ptr::null_mut());
        self.tails.fill(core::ptr::null_mut());
        self.biggest_free_list_index = 0;
    }

    /// v8: src/heap/cppgc-internal/free-list.cc FreeList::Size — 全链累计 AllocatedSize
    pub fn size(&self) -> usize { todo!("遍历累加") }
    /// v8: src/heap/cppgc-internal/free-list.cc FreeList::IsEmpty — 全桶头空
    pub fn is_empty(&self) -> bool { self.heads.iter().all(|e| e.is_null()) }
    /// v8: src/heap/cppgc-internal/free-list.cc FreeList::ContainsForTesting
    pub fn contains_for_testing(&self, block: Block) -> bool { todo!("区间包含查询") }
    /// v8: src/heap/cppgc-internal/free-list.cc FreeList::IsConsistent
    /// head/tail 同空, 或同非空且 tail->Next()==null
    fn is_consistent(&self, index: usize) -> bool { todo!("头尾一致性") }
}
`;

writeSrc("v8-heap-cppgc", "free_list.rs.skel", body);
console.log(`free-list.js: 对数 ${n} 项 (.h 8 + .cc 13 + 方法计数 12) → v8-heap-cppgc/src/free_list.rs.skel`);
