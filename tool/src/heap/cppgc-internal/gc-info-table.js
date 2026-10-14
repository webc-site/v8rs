#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/gc-info-table.js — v8/src/heap/cppgc-internal/gc-info-table.h + .cc → v8-heap-cppgc/src/gc_info_table.rs.skel
// GCInfo 类型注册表: 14bit 索引上限/惰性扩容/只读段推进 + GlobalGCInfoTable 单例
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/gc-info-table.h";
const REL_CC = "src/heap/cppgc-internal/gc-info-table.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

// ── ① 表上限常量 ───────────────────────────────────────────────────────
need(src, "class V8_EXPORT GCInfoTable final {", "GCInfoTable 声明");
need(src, "static constexpr GCInfoIndex kMaxIndex = 1 << 14;", "kMaxIndex 1<<14");
need(src, "static constexpr GCInfoIndex kMinIndex = 1;", "kMinIndex 1");
need(src, "static constexpr GCInfoIndex kInitialWantedLimit = 512;", "kInitialWantedLimit 512");
need(src, "class V8_EXPORT GlobalGCInfoTable final {", "GlobalGCInfoTable 声明");
need(src, "static void Initialize(PageAllocator& page_allocator);", "Initialize");
need(src, "static GCInfoTable& GetMutable() { return *global_table_; }", "GetMutable");
need(src, "GCInfo& TableSlotForTesting(GCInfoIndex index) { return table_[index]; }", "TableSlotForTesting");
// 成员 5
for (const f of ["GCInfo* table_;", "uint8_t* read_only_table_end_;",
  "GCInfoIndex current_index_ = kMinIndex;", "GCInfoIndex limit_ = 0;",
  "v8::base::Mutex table_mutex_;"]) need(src, f, "GCInfoTable 字段");
// ── ② .cc: 条目尺寸约束 + 扩容语义 ────────────────────────────────────
need(srcCc, "constexpr size_t kEntrySize = sizeof(GCInfo);", "kEntrySize");
need(srcCc, 'static_assert(v8::base::bits::IsPowerOfTwo(kEntrySize),', "2 的幂断言");
need(srcCc, "GCInfoTable* GlobalGCInfoTable::global_table_ = nullptr;", "单例定义");
need(srcCc, "const GCInfoIndex new_limit = (limit_) ? 2 * limit_ : InitialTableLimit();", "倍增扩容");
need(srcCc, "CHECK_GT(new_limit, limit_);", "扩容单调断言");
need(srcCc, "PageAllocator::kRead));", "只读段回退");
need(srcCc, "const GCInfoIndex new_index = current_index_++;", "索引自增");
need(srcCc, "CHECK_LT(new_index, GCInfoTable::kMaxIndex);", "上限断言");
need(srcCc, "registered_index.store(new_index, std::memory_order_release);", "release 发布");

const body = `${banner("src/heap/cppgc-internal/gc-info-table.js", `${REL} + ${REL_CC}`, "GCInfo 注册表骨架: 预留-按需提交-只读推进")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/gc-info-table.h GCInfoTable
pub struct GcInfoTable {
    /// v8: page_allocator_ — 页分配器 (整表预留 + 分段提交)
    page_allocator: PageAllocatorHandle,
    /// v8: oom_handler_
    oom_handler: FatalOutOfMemoryHandlerHandle,
    /// v8: table_ — 每类一个 GCInfo; HeapObjectHeader 持索引
    table: *mut GcInfo,
    /// v8: read_only_table_end_ — 只读段前沿
    read_only_table_end: *mut u8,
    /// v8: current_index_ = kMinIndex
    current_index: GcInfoIndex,
    /// v8: limit_ = 0 — 当前已提交表上限 (不含)
    limit: GcInfoIndex,
    /// v8: table_mutex_
    table_mutex: Mutex,
}

pub mod constants {
    /// v8: src/heap/cppgc-internal/gc-info-table.h GCInfoTable::kMaxIndex
    pub const K_MAX_INDEX: GcInfoIndex = 1 << 14;
    /// v8: src/heap/cppgc-internal/gc-info-table.h GCInfoTable::kMinIndex
    pub const K_MIN_INDEX: GcInfoIndex = 1;
    /// v8: src/heap/cppgc-internal/gc-info-table.h GCInfoTable::kInitialWantedLimit
    pub const K_INITIAL_WANTED_LIMIT: GcInfoIndex = 512;
    /// v8: src/heap/cppgc-internal/gc-info-table.cc kEntrySize
    /// static_assert(IsPowerOfTwo(sizeof(GCInfo))) — 条目不跨提交段边界
    pub const K_ENTRY_SIZE: usize = core::mem::size_of::<GcInfo>();
}

impl GcInfoTable {
    /// v8: src/heap/cppgc-internal/gc-info-table.cc GCInfoTable::RegisterNewGCInfo
    /// 锁内二次检查 → 满则 Resize (limit 倍增或取 InitialTableLimit) → current_index_++ → release 发布
    pub fn register_new_gc_info(&mut self, registered_index: &AtomicU16, info: GcInfo) -> GcInfoIndex {
        let _guard = self.table_mutex.lock();
        if let Some(idx) = registered_index.load(Ordering::Relaxed).checked_nonzero() {
            return idx;
        }
        if self.current_index == self.limit {
            self.resize();
        }
        let new_index = self.current_index;
        self.current_index += 1;
        assert!(new_index < constants::K_MAX_INDEX);
        unsafe { *self.table.add(new_index as usize) = info; }
        registered_index.store(new_index, Ordering::Release);
        new_index
    }

    /// v8: src/heap/cppgc-internal/gc-info-table.cc GCInfoTable::Resize
    /// 新段设 ReadWrite, 旧段回收为 Read, 校验新段零初始化 (CheckMemoryIsZeroed)。
    fn resize(&mut self) {
        /// v8: Resize — new_limit = limit ? 2*limit : InitialTableLimit()
        todo!("Resize: 倍增 + SetPermissions(ReadWrite) + 只读回退 + CheckMemoryIsZeroed")
    }

    /// v8: src/heap/cppgc-internal/gc-info-table.h GCInfoTable::GCInfoFromIndex
    pub fn gc_info_from_index(&self, index: GcInfoIndex) -> &GcInfo {
        debug_assert!(index >= constants::K_MIN_INDEX);
        debug_assert!(index < constants::K_MAX_INDEX);
        unsafe { &*self.table.add(index as usize) }
    }

    /// v8: src/heap/cppgc-internal/gc-info-table.h GCInfoTable::NumberOfGCInfos
    pub fn number_of_gc_infos(&self) -> GcInfoIndex { self.current_index }
    /// v8: src/heap/cppgc-internal/gc-info-table.h GCInfoTable::LimitForTesting
    pub fn limit_for_testing(&self) -> GcInfoIndex { self.limit }
}

/// v8: src/heap/cppgc-internal/gc-info-table.h GlobalGCInfoTable
pub struct GlobalGcInfoTable;

impl GlobalGcInfoTable {
    /// v8: src/heap/cppgc-internal/gc-info-table.cc GlobalGCInfoTable::Initialize
    /// LeakyObject 单例; 重复 Initialize 需同一 page_allocator (CHECK_EQ)。
    pub fn initialize(page_allocator: PageAllocatorHandle) { todo!("LeakyObject 单例初始化") }
}
`;

writeSrc("v8-heap-cppgc", "gc_info_table.rs.skel", body);
console.log(`gc-info-table.js: 对数 ${n} 项 (.h 12 + .cc 9) → v8-heap-cppgc/src/gc_info_table.rs.skel`);
