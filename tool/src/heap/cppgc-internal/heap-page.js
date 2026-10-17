#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap-page.js — v8/src/heap/cppgc-internal/heap-page.h + .cc → v8-heap-cppgc/src/heap_page.rs.skel
// 页三件: BasePage (类型分派/标记字节) + NormalPage (迭代器/头尺寸公式) + LargePage (分配尺寸公式/保证对齐)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap-page.h";
const REL_CC = "src/heap/cppgc-internal/heap-page.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

// ── ① BasePage ────────────────────────────────────────────────────────
need(src, "class V8_EXPORT_PRIVATE BasePage : public BasePageHandle {", "BasePage 声明");
need(src, "enum class PageType : uint8_t { kNormal, kLarge };", "PageType 枚举");
need(src, "bool is_large() const { return type_ == PageType::kLarge; }", "is_large");
need(src, "static BasePage* FromInnerAddress(const HeapBase*, void*);", "FromInnerAddress");
need(src, "static void Destroy(BasePage*);", "Destroy");
need(src, "void ChangeOwner(BaseSpace&);", "ChangeOwner");
need(src, "size_t AllocatedBytesAtLastGC() const;", "上次 GC 字节声明");
need(src, "HeapObjectHeader* TryObjectHeaderFromInnerAddress(void* address) const;", "TryObjectHeader");
need(src, "void SynchronizedLoad() const {", "TSAN 同步读");
need(src, "std::atomic_thread_fence(std::memory_order_seq_cst);", "发布栅栏");
need(src, "void IncrementMarkedBytes(size_t value) {", "IncrementMarkedBytes");
need(src, "std::atomic<size_t> marked_bytes_{0};", "原子标记字节");
need(src, "size_t discarded_memory_ = 0;", "discard 字段");
need(src, "bool contains_young_objects_ = false;", "年轻对象标志");
// ── ② NormalPage ──────────────────────────────────────────────────────
need(src, "class V8_EXPORT_PRIVATE NormalPage final : public BasePage {", "NormalPage 声明");
need(src, "class IteratorImpl : v8::base::iterator<std::forward_iterator_tag, T> {", "前向迭代器");
need(src, "if (reinterpret_cast<ConstAddress>(p_) == lab_start_) {", "迭代器跳 LAB");
need(src, "static constexpr size_t PageHeaderSize() {", "NormalPage::PageHeaderSize");
need(src, "return RoundUp(sizeof(NormalPage), kAllocationGranularity);", "头尺寸公式");
need(src, "static constexpr size_t PayloadSize();", "PayloadSize 声明");
need(src, "return kPageSize - RoundUp(sizeof(NormalPage), kAllocationGranularity);", "载荷公式");
need(src, "size_t allocated_bytes_at_last_gc_ = 0;", "上次 GC 字节字段");
need(src, "mutable PlatformAwareObjectStartBitmap object_start_bitmap_;", "起始位图");
// ── ③ LargePage ───────────────────────────────────────────────────────
need(src, "class V8_EXPORT_PRIVATE LargePage final : public BasePage {", "LargePage 声明");
need(src, "static constexpr size_t kGuaranteedObjectAlignment =", "保证对齐常量");
need(src, "2 * kAllocationGranularity;", "2×粒度");
need(src, "return RoundUp<kGuaranteedObjectAlignment>(sizeof(LargePage) +\n                                               sizeof(HeapObjectHeader)) -\n           sizeof(HeapObjectHeader);", "大页头公式");
need(src, "static size_t AllocationSize(size_t size);", "AllocationSize 声明");
need(src, "size_t payload_size_;", "payload_size_ 字段");
need(src, "size_t ObjectSize() const {\n    DCHECK_GT(payload_size_, sizeof(HeapObjectHeader));\n    return payload_size_ - sizeof(HeapObjectHeader);", "大页对象尺寸");
// ── ④ .cc 公式与实体 ─────────────────────────────────────────────────
need(srcCc, "return PageHeaderSize() + payload_size;", "AllocationSize = 头+载荷");
need(srcCc, "static_assert(kGuaranteedObjectAlignment <=", "对齐上限断言");
need(srcCc, "page_backend.TryAllocateLargePageMemory(allocation_size);", "大页内存");
need(srcCc, "backend->FreeLargePageMemory(reinterpret_cast<Address>(page));", "大页释放");
need(srcCc, "DCHECK_LE(kLargeObjectSizeThreshold, size);", "大页下限断言");
// hash 特化
need(src, "template <>\nstruct hash<const cppgc::internal::BasePage*> {", "hash 特化");
need(src, "api_constants::kPageSizeBits;", "页位移哈希");
// 方法实体计数
const defs = (srcCc.match(/^(?:HeapBase&|BasePage\*|const BasePage\*|void|size_t|HeapObjectHeader\*|const HeapObjectHeader\*|NormalPage\*|NormalPage::iterator|NormalPage::const_iterator|Address|ConstAddress|LargePage\*|bool) (?:BasePage|NormalPage|LargePage)::/gm) || []).length;
if (defs < 30) throw new Error(`页方法实体计数偏低: ${defs} (期望 >=30)`);
n += defs;

const body = `${banner("src/heap/cppgc-internal/heap-page.js", `${REL} + ${REL_CC}`, "常规/大页骨架: 头公式 + 起始位图 + 标记字节")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/heap-page.h BasePage::PageType
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PageType {
    Normal,
    Large,
}

/// v8: src/heap/cppgc-internal/heap-page.h BasePage
pub struct BasePage {
    /// v8: space_
    space: *mut BaseSpace,
    /// v8: type_
    page_type: PageType,
    /// v8: contains_young_objects_
    contains_young_objects: bool,
    /// v8: discarded_memory_
    discarded_memory: usize,
    /// v8: marked_bytes_ (atomic, relaxed)
    marked_bytes: AtomicUsize,
}

impl BasePage {
    /// v8: src/heap/cppgc-internal/heap-page.cc BasePage::BasePage
    pub fn new(space: *mut BaseSpace, page_type: PageType) -> Self {
        Self { space, page_type, contains_young_objects: false, discarded_memory: 0, marked_bytes: AtomicUsize::new(0) }
    }

    /// v8: BasePage::FromPayload — 委托 BasePageHandle::FromPayload
    pub fn from_payload(payload: *const core::ffi::c_void) -> *mut BasePage { todo!("BasePageHandle 换算") }
    /// v8: src/heap/cppgc-internal/heap-page.cc BasePage::FromInnerAddress — 页地址 → 页对象
    pub fn from_inner_address(heap: *const HeapBase, address: *mut core::ffi::c_void) -> *mut BasePage { todo!("页范围判定") }
    /// v8: src/heap/cppgc-internal/heap-page.cc BasePage::heap
    pub fn heap(&self) -> &HeapBase { todo!("space_->raw_heap()->heap()") }
    /// v8: BasePage::space / is_large
    pub fn space(&self) -> &BaseSpace { unsafe { &*self.space } }
    pub fn is_large(&self) -> bool { self.page_type == PageType::Large }

    /// v8: src/heap/cppgc-internal/heap-page.h SynchronizedLoad/SynchronizedStore
    /// TSAN: type_ 原子读写; 实际为 seq_cst 栅栏。
    pub fn synchronized_store(&mut self) { core::sync::atomic::fence(Ordering::SeqCst); }

    /// v8: BasePage::IncrementMarkedBytes / Decrement / Reset / marked_bytes
    pub fn increment_marked_bytes(&self, value: usize) { self.marked_bytes.fetch_add(value, Ordering::Relaxed); }
    pub fn decrement_marked_bytes(&self, value: usize) { self.marked_bytes.fetch_sub(value, Ordering::Relaxed); }
    pub fn reset_marked_bytes(&self, new_value: usize) { self.marked_bytes.store(new_value, Ordering::Relaxed); }
    pub fn marked_bytes(&self) -> usize { self.marked_bytes.load(Ordering::Relaxed) }

    /// v8: BasePage::IncrementDiscardedMemory / Reset / discarded_memory
    pub fn increment_discarded_memory(&mut self, value: usize) { self.discarded_memory += value; }
    pub fn reset_discarded_memory(&mut self) { self.discarded_memory = 0; }
    pub fn discarded_memory(&self) -> usize { self.discarded_memory }

    /// v8: BasePage::ObjectHeaderFromInnerAddress
    /// 大页直取; 常规页位图 FindHeader + 前向 fixup (逐头推进 + LAB 跳段 + 补位图位)。
    pub fn object_header_from_inner_address(&self, address: *mut core::ffi::c_void) -> &HeapObjectHeader { todo!("fixup 循环") }
    /// v8: ObjectHeaderFromInnerAddressConcurrent — 无 fixup 版
    pub fn object_header_from_inner_address_concurrent(&self, address: *mut core::ffi::c_void) -> &HeapObjectHeader { todo!("纯查询") }
    /// v8: src/heap/cppgc-internal/heap-page.cc BasePage::AllocatedSize
    pub fn allocated_size(&self) -> usize { todo!("按页型: kPageSize 或 LargePage::AllocationSize") }
    /// v8: src/heap/cppgc-internal/heap-page.cc BasePage::ChangeOwner
    pub fn change_owner(&mut self, space: *mut BaseSpace) { self.space = space; }
}

/// v8: src/heap/cppgc-internal/heap-page.h NormalPage
pub struct NormalPage {
    /// v8: BasePage 基类子对象
    pub base: BasePage,
    /// v8: allocated_bytes_at_last_gc_
    allocated_bytes_at_last_gc: usize,
    /// v8: object_start_bitmap_
    object_start_bitmap: PlatformAwareObjectStartBitmap,
}

impl NormalPage {
    /// v8: NormalPage::PageHeaderSize = RoundUp(sizeof(NormalPage), kAllocationGranularity)
    pub const fn page_header_size() -> usize { round_up_const(core::mem::size_of::<NormalPage>(), K_ALLOCATION_GRANULARITY) }
    /// v8: NormalPage::PayloadSize = kPageSize - PageHeaderSize()
    pub const fn payload_size() -> usize { K_PAGE_SIZE - Self::page_header_size() }

    /// v8: src/heap/cppgc-internal/heap-page.cc NormalPage::TryCreate — 常规页内存 + 构造 + SynchronizedStore
    pub fn try_create(page_backend: &mut PageBackend, space: &mut NormalPageSpace) -> *mut NormalPage { todo!("TryAllocateNormalPageMemory") }
    /// v8: src/heap/cppgc-internal/heap-page.cc NormalPage::Destroy
    pub fn destroy(page: *mut NormalPage) { todo!("NotifyFreedMemory + FreeNormalPageMemory") }

    /// v8: NormalPage::PayloadStart = 页基址 + PageHeaderSize; PayloadEnd = +PayloadSize
    pub fn payload_start(&self) -> *mut u8 { todo!("基址换算") }
    pub fn payload_end(&self) -> *mut u8 { unsafe { self.payload_start().add(Self::payload_size()) } }
    /// v8: NormalPage::begin/end — IteratorImpl 跳 LAB 推进 (p += size/sizeof(T))
    pub fn iter_mut(&mut self) -> PageIterator<'_> { todo!("迭代器") }
    /// v8: object_start_bitmap / AllocatedBytesAtLastGC / SetAllocatedBytesAtLastGC
    pub fn object_start_bitmap(&self) -> &PlatformAwareObjectStartBitmap { &self.object_start_bitmap }
    pub fn allocated_bytes_at_last_gc(&self) -> usize { self.allocated_bytes_at_last_gc }
    pub fn set_allocated_bytes_at_last_gc(&mut self, bytes: usize) { self.allocated_bytes_at_last_gc = bytes; }
}

/// v8: src/heap/cppgc-internal/heap-page.h LargePage
pub struct LargePage {
    /// v8: BasePage 基类子对象
    pub base: BasePage,
    /// v8: payload_size_
    payload_size: usize,
}

pub mod constants {
    /// v8: src/heap/cppgc-internal/heap-page.h LargePage::kGuaranteedObjectAlignment
    pub const K_GUARANTEED_OBJECT_ALIGNMENT: usize = 2 * K_ALLOCATION_GRANULARITY;
}

impl LargePage {
    /// v8: LargePage::PageHeaderSize
    /// = RoundUp<2×granularity>(sizeof(LargePage) + sizeof(HeapObjectHeader)) - sizeof(HeapObjectHeader)
    /// 头不对齐 granularity, 使 +HeapObjectHeader 后用户对象对齐 kGuaranteedObjectAlignment。
    pub const fn page_header_size() -> usize {
        round_up_const(core::mem::size_of::<LargePage>() + core::mem::size_of::<HeapObjectHeader>(),
                       constants::K_GUARANTEED_OBJECT_ALIGNMENT)
            - core::mem::size_of::<HeapObjectHeader>()
    }
    /// v8: src/heap/cppgc-internal/heap-page.cc LargePage::AllocationSize = PageHeaderSize() + payload_size
    pub fn allocation_size(payload_size: usize) -> usize { Self::page_header_size() + payload_size }

    /// v8: src/heap/cppgc-internal/heap-page.cc LargePage::TryCreate
    /// 断言 size >= kLargeObjectSizeThreshold 且 kGuaranteedObjectAlignment <= kMaxSupportedAlignment。
    pub fn try_create(page_backend: &mut PageBackend, space: &mut LargePageSpace, size: usize) -> *mut LargePage { todo!("TryAllocateLargePageMemory") }
    /// v8: src/heap/cppgc-internal/heap-page.cc LargePage::Destroy — NotifyFreedMemory(AllocationSize) + Free
    pub fn destroy(page: *mut LargePage) { todo!("释放大页内存") }

    /// v8: LargePage::ObjectHeader — PayloadStart 处单头
    pub fn object_header(&self) -> &HeapObjectHeader { todo!("reinterpret") }
    /// v8: LargePage::PayloadSize / ObjectSize (= payload - 头) / AllocatedBytesAtLastGC (= ObjectSize)
    pub fn payload_size(&self) -> usize { self.payload_size }
    pub fn object_size(&self) -> usize { self.payload_size - core::mem::size_of::<HeapObjectHeader>() }
}

/// v8: src/heap/cppgc-internal/heap-page.h v8::base::hash<BasePage*>
/// 页指针 >> api_constants::kPageSizeBits (压缩构建用 u32 地址)。
pub fn base_page_hash(page: *const BasePage) -> usize {
    (page as usize) >> K_PAGE_SIZE_BITS
}
`;

writeSrc("v8-heap-cppgc", "heap_page.rs.skel", body);
console.log(`heap-page.js: 对数 ${n} 项 (.h 25 + .cc 6 + 实体计数 ${defs}) → v8-heap-cppgc/src/heap_page.rs.skel`);
