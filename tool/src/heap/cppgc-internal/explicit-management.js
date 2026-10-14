#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/explicit-management.js — v8/src/heap/cppgc-internal/explicit-management.cc → v8-heap-cppgc/src/explicit_management.rs.skel
// 嵌入方手工管理: FreeUnreferencedObject (LAB 回收/大页销毁) + Resize (Grow/Shrink 靠 LAB)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/explicit-management.cc";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`explicit-management.cc 缺锚 ${label}: ${needle}`);
  n++;
};

need("bool InGC(HeapHandle& heap_handle) {", "InGC 判定");
need("heap.in_atomic_pause() || heap.marker() ||", "GC 中三条件");
need("void ExplicitManagementImpl::FreeUnreferencedObject(HeapHandle& heap_handle,", "FreeUnreferencedObject 签名");
need("header.Finalize();", "先终结");
need("heap_base.remembered_set().InvalidateRememberedSlotsInRange(", "年轻代槽失效");
need("base_page->space().RemovePage(base_page);", "大页摘除");
need("LargePage::Destroy(LargePage::From(base_page));", "大页销毁");
need("if (payload_end == lab.start()) {  // Returning to LAB.", "LAB 回收分支");
need("lab.Set(reinterpret_cast<Address>(&header), lab.size() + header_size);", "LAB 回退");
need("normal_space.free_list().Add({&header, header_size});", "自由链归还");
need("bool Grow(HeapObjectHeader& header, BasePage& base_page, size_t new_size,", "Grow 签名");
need("DCHECK_GE(new_size, header.AllocatedSize() + kAllocationGranularity);", "Grow 粒度断言");
need("if (lab.start() == header.ObjectEnd() && lab.size() >= size_delta) {", "Grow LAB 邻接判定");
need("header.SetAllocatedSize(new_size);", "尺寸改写");
need("bool Shrink(HeapObjectHeader& header, BasePage& base_page, size_t new_size,", "Shrink 签名");
need("size_delta >= ObjectAllocator::kSmallestSpaceSize", "回链最小尺寸启发");
need("bool ExplicitManagementImpl::Resize(void* object, size_t new_object_size) {", "Resize 签名");
need("if (base_page->is_large()) {\n    return false;\n  }", "大页拒绝");
need("RoundUp<kAllocationGranularity>(", "粒度对齐");
need("new_size >= kLargeObjectSizeThreshold", "大对象阈值上限");

const body = `${banner("src/heap/cppgc-internal/explicit-management.js", REL, "对象级手动释放/改尺寸 (GC 期间禁用)")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/explicit-management.cc InGC
fn in_gc(heap: &HeapBase) -> bool {
    heap.in_atomic_pause() || heap.marker().is_some() || heap.sweeper().is_sweeping_in_progress()
}

/// v8: include/cppgc/explicit-management.h ExplicitManagementImpl — 实现于本 cc
pub mod explicit_management_impl {
    /// v8: src/heap/cppgc-internal/explicit-management.cc FreeUnreferencedObject
    /// GC 中 → no-op。流程: Finalize → (年轻代: remembered set 失效 + 标记字节回退) →
    /// 大页: 摘页 + NotifyExplicitFree(PayloadSize) + Destroy;
    /// 常规: payload_end 吻合 lab.start → LAB 回退 (清 object_start 位); 否则 NotifyExplicitFree + 入自由链。
    pub fn free_unreferenced_object(heap_handle: &mut HeapHandle, object: *mut core::ffi::c_void) {
        todo!("按页型分派释放")
    }

    /// v8: src/heap/cppgc-internal/explicit-management.cc Resize
    /// 大页一律 false; new_size = RoundUp<granularity>(sizeof(HeapObjectHeader)+new_object_size);
    /// 越界 (超 kLargeObjectSizeThreshold 或小于头) false; 大→Grow, 小→Shrink, 等→true。
    pub fn resize(object: *mut core::ffi::c_void, new_object_size: usize) -> bool {
        todo!("Grow/Shrink 分派")
    }

    /// v8: src/heap/cppgc-internal/explicit-management.cc Grow
    /// 仅当 lab.start()==header.ObjectEnd() 且 lab.size()>=delta:
    /// LAB 前移分配 delta, SetMemoryAccessible, SetAllocatedSize(new_size); (年轻代: 已标记则 IncrementMarkedBytes)
    fn grow(header: &mut HeapObjectHeader, base_page: &mut BasePage, new_size: usize, size_delta: usize) -> bool {
        todo!("LAB 邻接扩张")
    }

    /// v8: src/heap/cppgc-internal/explicit-management.cc Shrink
    /// LAB 邻接 → lab.Set(free_start, size+delta) + SetMemoryInaccessible;
    /// 否则 delta >= ObjectAllocator::kSmallestSpaceSize 才回自由链 (NotifyExplicitFree + SetBit);
    /// (年轻代: remembered set 失效 + 已标记则 DecrementMarkedBytes)。恒返回 true (避免嵌入方搬内存)。
    fn shrink(header: &mut HeapObjectHeader, base_page: &mut BasePage, new_size: usize, size_delta: usize) -> bool {
        todo!("LAB 回退/自由链归还")
    }
}
`;

writeSrc("v8-heap-cppgc", "explicit_management.rs.skel", body);
console.log(`explicit-management.js: 对数 ${n} 项 (InGC 2 + Free 8 + Grow 4 + Shrink 2 + Resize 3) → v8-heap-cppgc/src/explicit_management.rs.skel`);
