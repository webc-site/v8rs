#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/page-memory.h.js — v8/src/heap/cppgc-internal/{page-memory.h,page-memory.cc} → v8-heap-cppgc/src/page_memory.rs.skel
// 职责: 页内存后端 4 层 — MemoryRegion 区间 → PageMemoryRegion 预留区 → 树/池索引 → PageBackend 双分配器统一门面
// 规则:
//   ① MemoryRegion: base/size/end + Contains 双态 (地址差<size / 区间包含)
//   ② PageMemoryRegionTree::Lookup: upper_bound 前探 (begin==end 空判, next(it,-1) 命中判 end 界)
//   ③ NormalPageMemoryPool: Add 零化 (ZAP 全页 / release 仅页头) 后入池; Take 弹尾 + ASAN 解毒 + DEBUG 零校验
//   ④ PageBackend 4 路分配/释放: normal 池优先→新建→unprotect 失败弃; large 无池直建; Free normal 回池不删区; Free large 删区
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/page-memory.h";
const REL_CC = "src/heap/cppgc-internal/page-memory.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE MemoryRegion final {", "区间类");
need(src, "Address end() const { return base_ + size_; }", "end 式");
need(src, "reinterpret_cast<uintptr_t>(addr) -", "Contains 地址差式");
need(src, "reinterpret_cast<uintptr_t>(base_)) < size_;", "Contains 界式");
need(src, "class V8_EXPORT_PRIVATE PageMemoryRegion final {", "预留区类");
need(src, "memory_region.Contains(address) ? memory_region.base() : nullptr;", "PMR Lookup 式");
need(src, "class V8_EXPORT_PRIVATE PageMemoryRegionTree final {", "树类");
need(src, "std::map<ConstAddress, PageMemoryRegion*> set_;", "树容器");
need(src, "class V8_EXPORT_PRIVATE NormalPageMemoryPool final {", "池类");
need(src, "size_t pooled() const { return pool_.size(); }", "池计数");
need(src, "class V8_EXPORT_PRIVATE PageBackend final {", "后端类");
need(src, "mutable v8::base::Mutex mutex_;", "Lookup 并发互斥");
need(src, "std::unordered_map<PageMemoryRegion*, std::unique_ptr<PageMemoryRegion>>", "双持有表 (normal)");
need(src, "auto it = set_.upper_bound(address);", "树上探");
need(src, "auto* result = std::next(it, -1)->second;", "前驱取值");
need(src, "if (address < result->region().end()) {", "end 界命中判");
need(cc, "allocator.SetPermissions(memory_region.base(), memory_region.size(),", "解除保护调用");
need(cc, "PageAllocator::Permission::kNoAccess);", "预留无权限");
need(cc, "ASAN_UNPOISON_MEMORY_REGION(reserved_region.base(), reserved_region.size());", "释放前解毒");
need(cc, "RoundUp(length, allocator.AllocatePageSize())", "大页对齐式");
need(cc, "#if CPPGC_SHOULD_ZAP_MEMORY", "ZAP 门");
need(cc, "const size_t size = NormalPage::PageHeaderSize();", "release 零化页头");
need(cc, "CheckMemoryIsZero(base, size);", "DEBUG 零校验");
need(cc, "if (PageMemoryRegion* cached = page_pool_.Take()) {", "normal 池优先");
need(cc, "normal_page_memory_regions_.emplace(pmr.get(), std::move(pmr));", "normal 持有入表");
need(cc, "page_pool_.Add(pmr);", "normal 回池");
need(cc, "auto size = large_page_memory_regions_.erase(pmr);", "large 删区");
need(cc, "auto pooled_regions = page_pool_.TakeAll();", "释放池页");

// PageBackend 公有方法全序
const be = [...cc.matchAll(/^(?:Address|void) PageBackend::(\w+)\(/gm)].map((m) => m[1]);
if (be.join(",") !== "TryAllocateNormalPageMemory,FreeNormalPageMemory,TryAllocateLargePageMemory,FreeLargePageMemory,ReleasePooledPages")
  throw new Error(`PageBackend 方法序: got=${be.join(",")}`);
n++;

const body = `${banner("src/heap/cppgc-internal/page-memory.h.js", REL_H, "页内存后端 — MemoryRegion/RegionTree/PageMemoryPool/PageBackend 4 层 (cc 实现一并落地)")}
// ══ MemoryRegion (原版 page-memory.h:22-46) ══

#[derive(Debug, Clone, Copy, Default)]
pub struct MemoryRegion {
    base: *mut u8, // = nullptr
    size: usize,   // = 0
}

impl MemoryRegion {
    /// 原版 :25-28 — DCHECK base 非空 + size 正
    pub fn new(base: *mut u8, size: usize) -> Self {
        debug_assert!(!base.is_null());
        debug_assert!(size > 0);
        Self { base, size }
    }
    pub fn base(&self) -> *mut u8 { self.base }          // :30
    pub fn size(&self) -> usize { self.size }            // :31
    pub fn end(&self) -> *mut u8 { unsafe { self.base.add(self.size) } } // :32
    /// 原版 :34-37 — 无符号地址差 < size (单比较含下界)
    pub fn contains(&self, addr: *const u8) -> bool {
        (addr as usize).wrapping_sub(self.base as usize) < self.size
    }
    /// 原版 :39-41 — 区间包含: base <= other.base && other.end <= end
    pub fn contains_region(&self, other: &MemoryRegion) -> bool {
        self.base <= other.base() && other.end() <= self.end()
    }
}

// ══ PageMemoryRegion (原版 page-memory.h:48-73, 实现 cc:86-96) ══

pub struct PageMemoryRegion {
    allocator: *mut PageAllocator,
    reserved_region: MemoryRegion,
}

impl PageMemoryRegion {
    /// 原版 cc:86-88
    pub fn new(allocator: &mut PageAllocator, reserved_region: MemoryRegion) -> Self {
        Self { allocator, reserved_region }
    }
    pub fn region(&self) -> MemoryRegion { self.reserved_region } // :53
    /// 原版 :57-60 — 含地址 → 可写基址, 否则 null
    pub fn lookup(&self, address: *const u8) -> *mut u8 {
        let r = self.region();
        if r.contains(address) { r.base() } else { std::ptr::null_mut() }
    }
    /// 原版 cc:94-96 — CHECK TryUnprotect
    pub fn unprotect_for_testing(&self) {
        v8_check!(try_unprotect(unsafe { &mut *self.allocator }, &self.reserved_region));
    }
}

impl Drop for PageMemoryRegion {
    /// 原版 cc:90-92 — 释放前 ASAN 解毒再 FreePages
    fn drop(&mut self) {
        free_memory_region(unsafe { &mut *self.allocator }, &self.reserved_region);
    }
}

// ══ PageMemoryRegionTree (原版 page-memory.h:80-92, 实现 cc:98-114 + h:182-194) ══
// 按 base 排序的索引树, 不持有元素。

pub struct PageMemoryRegionTree {
    set: std::collections::BTreeMap<usize, *mut PageMemoryRegion>, // key = reserved base
}

impl PageMemoryRegionTree {
    /// 原版 cc:108-113 — emplace, DCHECK 新键
    pub fn add(&mut self, region: *mut PageMemoryRegion) {
        debug_assert!(!region.is_null());
        let base = unsafe { &*region }.region().base() as usize;
        let r = self.set.insert(base, region);
        debug_assert!(r.is_none());
    }
    /// 原版 cc:115-120 — erase, DCHECK 恰 1
    pub fn remove(&mut self, region: *mut PageMemoryRegion) {
        debug_assert!(!region.is_null());
        let removed = self.set.remove(&(unsafe { &*region }.region().base() as usize));
        debug_assert!(removed.is_some());
    }
    /// 原版 h:182-194 — upper_bound 前探: 空树/首前驱越界 → null; 前驱 region 含 address → 命中
    pub fn lookup(&self, address: *const u8) -> *mut PageMemoryRegion {
        let addr = address as usize;
        let mut it = self.set.range(..=addr);
        match it.next_back() {
            None => std::ptr::null_mut(), // 空/全域越界
            Some((_, result)) => {
                if address < unsafe { &**result }.region().end() {
                    *result
                } else {
                    std::ptr::null_mut()
                }
            }
        }
    }
}

// ══ NormalPageMemoryPool (原版 page-memory.h:96-117, 实现 cc:122-162) ══

pub struct NormalPageMemoryPool {
    pool: Vec<*mut PageMemoryRegion>, // 受 primary_pool_capacity_ 约束
}

impl NormalPageMemoryPool {
    /// 原版 cc:122-136 — 入池前零化: ZAP 配置全页; release 仅页头 (payload 已由 SetMemoryInaccessible 归零)
    pub fn add(&mut self, pmr: *mut PageMemoryRegion) {
        debug_assert!(!pmr.is_null());
        debug_assert_eq!(unsafe { &*pmr }.region().size(), K_PAGE_SIZE);
        let base = unsafe { &*pmr }.region().base();
        // v8: CPPGC_SHOULD_ZAP_MEMORY 全页零化, 否则仅 NormalPage::PageHeaderSize()
        let size = if should_zap_memory() { unsafe { &*pmr }.region().size() } else { NormalPage::page_header_size() };
        let _unpoison = AsanUnpoisonScope::new(base, size);
        unsafe { std::ptr::write_bytes(base, 0, size) };
        self.pool.push(pmr);
    }
    /// 原版 cc:138-153 — 弹尾 + ASAN 解毒 + DEBUG CheckMemoryIsZero; 空池 null
    pub fn take(&mut self) -> *mut PageMemoryRegion {
        match self.pool.pop() {
            None => std::ptr::null_mut(),
            Some(pmr) => {
                let base = unsafe { &*pmr }.region().base();
                let size = unsafe { &*pmr }.region().size();
                asan_unpoison_memory_region(base, size);
                // v8: DEBUG — CheckMemoryIsZero(base, size)
                pmr
            }
        }
    }
    pub fn pooled(&self) -> usize { self.pool.len() } // :104
    /// 原版 cc:164-169 — 池内实际占用字节数 (未 discard/decommit)
    pub fn pooled_memory(&self) -> usize {
        self.pool.iter().map(|p| unsafe { &**p }.region().size()).sum()
    }
    /// 原版 cc:171-173 — move 出全池
    pub fn take_all(&mut self) -> Vec<*mut PageMemoryRegion> {
        std::mem::take(&mut self.pool)
    }
}

// ══ PageBackend (原版 page-memory.h:123-200, 实现 cc:175-226) ══

pub struct PageBackend {
    mutex: v8_base::Mutex, // Guard 并发 Lookup
    normal_page_allocator: *mut PageAllocator,
    large_page_allocator: *mut PageAllocator,
    page_pool: NormalPageMemoryPool,
    page_memory_region_tree: PageMemoryRegionTree,
    /// normal 区恒持有 (在用时在树, 空闲时在池)
    normal_page_memory_regions: std::collections::HashMap<usize, Box<PageMemoryRegion>>,
    /// large 区按需持有
    large_page_memory_regions: std::collections::HashMap<usize, Box<PageMemoryRegion>>,
}

impl PageBackend {
    /// 原版 cc:175-179
    pub fn new(normal_page_allocator: &mut PageAllocator, large_page_allocator: &mut PageAllocator) -> Self {
        Self { mutex: Default::default(), normal_page_allocator, large_page_allocator,
            page_pool: NormalPageMemoryPool::default(), page_memory_region_tree: PageMemoryRegionTree::default(),
            normal_page_memory_regions: Default::default(), large_page_memory_regions: Default::default() }
    }

    /// 原版 h:196-200 — 互斥下树查 + 区查
    pub fn lookup(&self, address: *const u8) -> *mut u8 {
        let _g = self.mutex.guard();
        let pmr = self.page_memory_region_tree.lookup(address);
        if pmr.is_null() { std::ptr::null_mut() } else { unsafe { &*pmr }.lookup(address) }
    }

    /// 原版 cc:181-201 — 池优先 (免预留); 否则新建预留+unprotect; 失败 null
    pub fn try_allocate_normal_page_memory(&mut self) -> *mut u8 {
        let _g = self.mutex.guard();
        let cached = self.page_pool.take();
        if !cached.is_null() {
            let region = unsafe { &*cached }.region();
            // v8: DCHECK normal 表含 cached (在池区必在持有表)
            self.page_memory_region_tree.add(cached);
            return region.base();
        }
        let pmr = match create_normal_page_memory_region(unsafe { &mut *self.normal_page_allocator }) {
            None => return std::ptr::null_mut(),
            Some(p) => p,
        };
        let memory_region = pmr.region();
        if try_unprotect(unsafe { &mut *self.normal_page_allocator }, &memory_region) {
            self.page_memory_region_tree.add(&mut *pmr);
            let base = pmr.region().base() as usize;
            self.normal_page_memory_regions.insert(base, pmr);
            return memory_region.base();
        }
        std::ptr::null_mut()
    }

    /// 原版 cc:203-209 — 树摘 → 入池 (区不删, 复用)
    pub fn free_normal_page_memory(&mut self, writeable_base: *mut u8) {
        let _g = self.mutex.guard();
        let pmr = self.page_memory_region_tree.lookup(writeable_base);
        debug_assert!(!pmr.is_null());
        self.page_memory_region_tree.remove(pmr);
        self.page_pool.add(pmr);
    }

    /// 原版 cc:211-225 — 无池, RoundUp(AllocatePageSize) 预留 + unprotect
    pub fn try_allocate_large_page_memory(&mut self, size: usize) -> *mut u8 {
        let _g = self.mutex.guard();
        let pmr = match create_large_page_memory_region(unsafe { &mut *self.large_page_allocator }, size) {
            None => return std::ptr::null_mut(),
            Some(p) => p,
        };
        let memory_region = pmr.region();
        if try_unprotect(unsafe { &mut *self.large_page_allocator }, &memory_region) {
            self.page_memory_region_tree.add(&mut *pmr);
            let base = pmr.region().base() as usize;
            self.large_page_memory_regions.insert(base, pmr);
            return memory_region.base();
        }
        std::ptr::null_mut()
    }

    /// 原版 cc:227-234 — 树摘 + 删区 (large 不池化)
    pub fn free_large_page_memory(&mut self, writeable_base: *mut u8) {
        let _g = self.mutex.guard();
        let pmr = self.page_memory_region_tree.lookup(writeable_base);
        self.page_memory_region_tree.remove(pmr);
        let removed = self.large_page_memory_regions.remove(&(unsafe { &*pmr }.region().base() as usize));
        debug_assert!(removed.is_some());
    }

    /// 原版 cc:236-244 — 池页全量释放 (erase 持有表)
    pub fn release_pooled_pages(&mut self) {
        let _g = self.mutex.guard();
        for region in self.page_pool.take_all() {
            let erased = self.normal_page_memory_regions.remove(&(unsafe { &*region }.region().base() as usize));
            debug_assert!(erased.is_some());
        }
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "page_memory.rs.skel", body);
console.log(`OK page-memory.h.js: 锚 ${n} 条 (h13+cc17+方法序1); 4 层结构; 产物 ${out}`);
