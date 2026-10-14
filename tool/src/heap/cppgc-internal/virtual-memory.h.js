#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/virtual-memory.h.js — v8/src/heap/cppgc-internal/{virtual-memory.h,virtual-memory.cc} → v8-heap-cppgc/src/virtual_memory.rs.skel
// 职责: VirtualMemory RAII 预留区 — 双构造 (对齐预留 / 接管已留) + 移动语义 + 析构释放
// 规则:
//   ① 对齐构造: size 对齐 CommitPageSize (DCHECK), AllocatePages(hint, RoundUp(size), RoundUp(alignment), kNoAccess), 成功才落 size
//   ② 接管构造: 直接落 start/size; ③ move 构造/赋值: 源 Reset; 赋值 DCHECK 目标空; ④ Drop: IsReserved 才 FreePages
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/virtual-memory.h";
const REL_CC = "src/heap/cppgc-internal/virtual-memory.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE VirtualMemory {", "类声明");
need(src, "VirtualMemory() = default;", "空默认构造");
need(src, "VirtualMemory(PageAllocator*, size_t size, size_t alignment,", "对齐预留构造声明");
need(src, "void* hint = nullptr);", "hint 默认参");
need(src, "VirtualMemory(PageAllocator*, void* reserved, size_t size);", "接管构造声明");
need(src, "~VirtualMemory() V8_NOEXCEPT;", "析构声明");
need(src, "VirtualMemory(VirtualMemory&&) V8_NOEXCEPT;", "移动构造声明");
need(src, "VirtualMemory& operator=(VirtualMemory&&) V8_NOEXCEPT;", "移动赋值声明");
need(src, "bool IsReserved() const { return start_ != nullptr; }", "预留判定式");
need(cc, "DCHECK(IsAligned(size, page_allocator->CommitPageSize()));", "提交页对齐断言");
need(cc, "RoundUp(size, page_size),", "size 上取整");
need(cc, "RoundUp(alignment, page_size),", "alignment 上取整");
need(cc, "PageAllocator::kNoAccess);", "预留无权限");
need(cc, "size_ = RoundUp(size, page_size);", "成功才落尺寸");
need(cc, "page_allocator_->FreePages(start_, size_);", "析构释放");
need(cc, "DCHECK(!IsReserved());", "赋值目标空断言");
need(cc, "other.Reset();", "源复位调用");
if ((cc.match(/other\.Reset\(\);/g) || []).length !== 2)
  throw new Error(`move 后源 Reset 计数: got=${(cc.match(/other\.Reset\(\);/g) || []).length} want=2`);
n++;

const body = `${banner("src/heap/cppgc-internal/virtual-memory.h.js", REL_H, "VirtualMemory RAII 预留区 — 对齐预留/接管双构造 + move + Drop 释放 (cc 实现一并落地)")}
// ══ VirtualMemory (原版 virtual-memory.h:17-58, 实现 virtual-memory.cc:13-60) ══

#[derive(Default)]
pub struct VirtualMemory {
    page_allocator: *mut PageAllocator, // = nullptr
    start: *mut u8,                     // = nullptr
    size: usize,                        // = 0
}

impl VirtualMemory {
    /// 原版 cc:13-26 — 对齐预留: 双 RoundUp (size/alignment → AllocatePageSize), kNoAccess; 失败保持空
    pub fn new_reserved(page_allocator: *mut PageAllocator, size: usize, alignment: usize, hint: *mut u8) -> Self {
        let mut vm = Self { page_allocator, start: std::ptr::null_mut(), size: 0 };
        debug_assert!(!page_allocator.is_null());
        let pa = unsafe { &*page_allocator };
        debug_assert!(is_aligned(size, pa.commit_page_size()));
        let page_size = pa.allocate_page_size();
        vm.start = pa.allocate_pages(hint, round_up(size, page_size), round_up(alignment, page_size), PageAllocatorPermission::NoAccess);
        if !vm.start.is_null() {
            vm.size = round_up(size, page_size);
        }
        vm
    }

    /// 原版 cc:28-33 — 接管已预留区: 直接落 start/size
    pub fn new_takeover(page_allocator: *mut PageAllocator, reserved: *mut u8, size: usize) -> Self {
        debug_assert!(!page_allocator.is_null());
        debug_assert!(is_aligned(size, unsafe { &*page_allocator }.commit_page_size()));
        Self { page_allocator, start: reserved, size }
    }

    /// 原版 h:39 — start 非空即已预留
    pub fn is_reserved(&self) -> bool {
        !self.start.is_null()
    }

    /// 原版 h:41-44 — DCHECK 预留后返回 start
    pub fn address(&self) -> *mut u8 {
        debug_assert!(self.is_reserved());
        self.start
    }

    /// 原版 h:46-49
    pub fn size(&self) -> usize {
        debug_assert!(self.is_reserved());
        self.size
    }

    /// 原版 cc:57-60 — 复位到默认态 (move 源清空)
    fn reset(&mut self) {
        self.start = std::ptr::null_mut();
        self.size = 0;
    }
}

/// 原版 cc:41-46 — move 构造: 三字段移走 + 源 Reset
/// Rust 侧 move 语义天然独占, 显式封装以保留"源复位"语义面
impl VirtualMemory {
    pub fn move_from(other: &mut Self) -> Self {
        let taken = Self { page_allocator: other.page_allocator, start: other.start, size: other.size };
        other.reset();
        taken
    }
}

/// 原版 cc:48-55 — move 赋值: DCHECK 目标未预留 (防泄漏), 移三字段, 源 Reset
impl VirtualMemory {
    pub fn move_assign(&mut self, other: &mut Self) -> &mut Self {
        debug_assert!(!self.is_reserved());
        self.page_allocator = other.page_allocator;
        self.start = other.start;
        self.size = other.size;
        other.reset();
        self
    }
}

/// 原版 cc:35-39 — Drop: 已预留才 FreePages
impl Drop for VirtualMemory {
    fn drop(&mut self) {
        if self.is_reserved() {
            unsafe { &mut *self.page_allocator }.free_pages(self.start, self.size);
        }
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "virtual_memory.rs.skel", body);
console.log(`OK virtual-memory.h.js: 锚 ${n} 条 (h9+cc9+Reset 计数1); 双构造 + move; 产物 ${out}`);
