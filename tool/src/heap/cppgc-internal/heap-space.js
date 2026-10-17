#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap-space.js — v8/src/heap/cppgc-internal/heap-space.h + .cc → v8-heap-cppgc/src/heap_space.rs.skel
// 页空间: BaseSpace (页集合管理) + NormalPageSpace (LAB + FreeList) + LargePageSpace
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap-space.h";
const REL_CC = "src/heap/cppgc-internal/heap-space.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE BaseSpace {", "BaseSpace 声明");
need(src, "using Pages = std::vector<BasePage*>;", "Pages");
need(src, "enum class PageType { kNormal, kLarge };", "PageType 枚举");
need(src, "bool is_large() const { return type_ == PageType::kLarge; }", "is_large");
need(src, "void AddPage(BasePage*);", "AddPage 声明");
need(src, "void RemovePage(BasePage*);", "RemovePage 声明");
need(src, "Pages RemoveAllPages();", "RemoveAllPages 声明");
need(src, "v8::base::Mutex& pages_mutex() const { return pages_mutex_; }", "pages_mutex");
need(src, "bool is_compactable() const { return is_compactable_; }", "is_compactable");
need(src, "class LinearAllocationBuffer {", "LAB 类");
need(src, "Address Allocate(size_t alloc_size) {", "LAB::Allocate");
need(src, "void Set(Address ptr, size_t size) {", "LAB::Set");
need(src, "class V8_EXPORT_PRIVATE NormalPageSpace final : public BaseSpace {", "NormalPageSpace");
need(src, "static NormalPageSpace& From(BaseSpace& space) {", "NormalPageSpace::From");
need(src, "LinearAllocationBuffer& linear_allocation_buffer() { return current_lab_; }", "LAB 访问器");
need(src, "FreeList& free_list() { return free_list_; }", "free_list 访问器");
need(src, "class V8_EXPORT_PRIVATE LargePageSpace final : public BaseSpace {", "LargePageSpace");
need(src, "LargePageSpace(RawHeap* heap, size_t index);", "LargePageSpace 构造");
// .cc 实体
need(srcCc, "v8::base::MutexGuard lock(&pages_mutex_);\n  DCHECK_EQ(pages_.cend(), std::find(pages_.cbegin(), pages_.cend(), page));", "AddPage 互斥查重");
need(srcCc, "pages_.erase(it);", "RemovePage 删除");
need(srcCc, "Pages pages = std::move(pages_);\n  pages_.clear();", "RemoveAllPages 移交");
need(srcCc, "    : BaseSpace(heap, index, PageType::kNormal, is_compactable) {}", "Normal 构造委托");
need(srcCc, "    : BaseSpace(heap, index, PageType::kLarge, false /* is_compactable */) {}", "Large 构造委托");

const body = `${banner("src/heap/cppgc-internal/heap-space.js", `${REL} + ${REL_CC}`, "页空间: 页集合 + LAB + 自由链持有")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/heap-space.h BaseSpace::PageType
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PageType {
    Normal,
    Large,
}

/// v8: src/heap/cppgc-internal/heap-space.h BaseSpace
pub struct BaseSpace {
    /// v8: heap_
    heap: *mut RawHeap,
    /// v8: pages_ (Pages = Vec<BasePage*>)
    pages: Vec<*mut BasePage>,
    /// v8: pages_mutex_ (mutable)
    pages_mutex: Mutex,
    /// v8: index_
    index: usize,
    /// v8: type_
    page_type: PageType,
    /// v8: is_compactable_
    is_compactable: bool,
}

impl BaseSpace {
    /// v8: src/heap/cppgc-internal/heap-space.cc BaseSpace::BaseSpace
    pub fn new(heap: *mut RawHeap, index: usize, page_type: PageType, is_compactable: bool) -> Self {
        Self { heap, pages: Vec::new(), pages_mutex: Mutex::new(), index, page_type, is_compactable }
    }

    /// v8: src/heap/cppgc-internal/heap-space.h BaseSpace::is_large
    pub fn is_large(&self) -> bool { self.page_type == PageType::Large }
    /// v8: BaseSpace::index
    pub fn index(&self) -> usize { self.index }
    /// v8: BaseSpace::is_compactable
    pub fn is_compactable(&self) -> bool { self.is_compactable }
    /// v8: BaseSpace::size
    pub fn size(&self) -> usize { self.pages.len() }
    /// v8: BaseSpace::raw_heap
    pub fn raw_heap(&self) -> *mut RawHeap { self.heap }

    /// v8: src/heap/cppgc-internal/heap-space.cc BaseSpace::AddPage — 锁内查重后 push
    pub fn add_page(&mut self, page: *mut BasePage) { todo!("MutexGuard + find 断言") }
    /// v8: src/heap/cppgc-internal/heap-space.cc BaseSpace::RemovePage — 锁内 find + erase
    pub fn remove_page(&mut self, page: *mut BasePage) { todo!("MutexGuard + erase") }
    /// v8: src/heap/cppgc-internal/heap-space.cc BaseSpace::RemoveAllPages — move 出清
    pub fn remove_all_pages(&mut self) -> Vec<*mut BasePage> {
        core::mem::take(&mut self.pages)
    }
}

/// v8: src/heap/cppgc-internal/heap-space.h NormalPageSpace::LinearAllocationBuffer
pub struct LinearAllocationBuffer {
    /// v8: start_
    start: *mut u8,
    /// v8: limit_
    limit: *mut u8,
}

impl LinearAllocationBuffer {
    /// v8: LAB::Allocate — 起点前移返回旧起点
    pub fn allocate(&mut self, alloc_size: usize) -> *mut u8 {
        debug_assert!(self.size() >= alloc_size);
        let result = self.start;
        self.start = unsafe { self.start.add(alloc_size) };
        result
    }
    /// v8: LAB::Set
    pub fn set(&mut self, ptr: *mut u8, size: usize) {
        self.start = ptr;
        self.limit = unsafe { ptr.add(size) };
    }
    /// v8: LAB::start / limit / size
    pub fn start(&self) -> *mut u8 { self.start }
    pub fn limit(&self) -> *mut u8 { self.limit }
    pub fn size(&self) -> usize { self.limit as usize - self.start as usize }
}

/// v8: src/heap/cppgc-internal/heap-space.h NormalPageSpace
pub struct NormalPageSpace {
    /// v8: BaseSpace 基类子对象
    base: BaseSpace,
    /// v8: current_lab_
    current_lab: LinearAllocationBuffer,
    /// v8: free_list_
    free_list: FreeList,
}

impl NormalPageSpace {
    /// v8: src/heap/cppgc-internal/heap-space.cc NormalPageSpace 构造 — PageType::kNormal
    pub fn new(heap: *mut RawHeap, index: usize, is_compactable: bool) -> Self {
        Self { base: BaseSpace::new(heap, index, PageType::Normal, is_compactable),
               current_lab: LinearAllocationBuffer { start: core::ptr::null_mut(), limit: core::ptr::null_mut() },
               free_list: FreeList::default() }
    }
    /// v8: NormalPageSpace::linear_allocation_buffer
    pub fn linear_allocation_buffer(&mut self) -> &mut LinearAllocationBuffer { &mut self.current_lab }
    /// v8: NormalPageSpace::free_list
    pub fn free_list(&mut self) -> &mut FreeList { &mut self.free_list }
}

/// v8: src/heap/cppgc-internal/heap-space.h LargePageSpace
pub struct LargePageSpace {
    /// v8: BaseSpace 基类子对象
    base: BaseSpace,
}

impl LargePageSpace {
    /// v8: src/heap/cppgc-internal/heap-space.cc LargePageSpace 构造 — kLarge + 恒非紧凑
    pub fn new(heap: *mut RawHeap, index: usize) -> Self {
        Self { base: BaseSpace::new(heap, index, PageType::Large, false) }
    }
}
`;

writeSrc("v8-heap-cppgc", "heap_space.rs.skel", body);
console.log(`heap-space.js: 对数 ${n} 项 (.h 17 + .cc 5) → v8-heap-cppgc/src/heap_space.rs.skel`);
