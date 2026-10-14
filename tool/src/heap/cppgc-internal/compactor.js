#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/compactor.js — v8/src/heap/cppgc-internal/compactor.h + .cc → v8-heap-cppgc/src/compactor.rs.skel
// 压缩器: MovableReferences 槽重定位 (含内指针) + ShouldCompact 阈值判定 + 空间紧凑
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/compactor.h";
const REL_CC = "src/heap/cppgc-internal/compactor.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE Compactor final {", "Compactor 声明");
need(src, "using CompactableSpaceHandling = SweepingConfig::CompactableSpaceHandling;", "处理枚举别名");
need(src, "void InitializeIfShouldCompact(GCConfig::MarkingType, StackState);", "InitializeIfShouldCompact");
need(src, "void CancelIfShouldNotCompact(GCConfig::MarkingType, StackState);", "CancelIfShouldNotCompact");
need(src, "CompactableSpaceHandling CompactSpacesIfEnabled();", "CompactSpacesIfEnabled");
need(src, "void EnableForNextGCForTesting();", "EnableForNextGCForTesting");
need(src, "bool ShouldCompact(GCConfig::MarkingType, StackState) const;", "ShouldCompact 声明");
need(src, "bool is_enabled_ = false;", "is_enabled_ 字段");
need(src, "bool is_cancelled_ = false;", "is_cancelled_ 字段");
need(src, "bool enable_for_next_gc_for_testing_ = false;", "测试旗字段");
need(src, "~Compactor() { DCHECK(!is_enabled_); }", "析构禁启用断言");
// ── .cc: MovableReferences + 阈值 + 主流程 ────────────────────────────
need(srcCc, "class MovableReferences final {", "MovableReferences 声明");
need(srcCc, "void AddOrFilter(MovableReference*);", "AddOrFilter 声明");
need(srcCc, "void Relocate(Address from, Address to, size_t size_including_header);", "Relocate 声明");
need(srcCc, "void RelocateInteriorReferences(Address from, Address to, size_t size);", "内指针重定位声明");
need(srcCc, "static constexpr size_t kFreeListSizeThreshold = 512 * kKB;", "自由链阈值 512KB");
need(srcCc, "size_t UpdateHeapResidency(const std::vector<NormalPageSpace*>& spaces) {", "驻留计算");
need(srcCc, "return free_list_size > kFreeListSizeThreshold;", "阈值判定");
need(srcCc, "marking_type == GCConfig::MarkingType::kAtomic &&", "原子+保守拒压");
need(srcCc, "stack_state == StackState::kMayContainHeapPointers)) {", "保守栈条件");
need(srcCc, "if (enable_for_next_gc_for_testing_) {\n    return true;\n  }", "测试旗直通");
need(srcCc, "compaction_worklists_->movable_slots_worklist()->Clear();", "取消清队");
need(srcCc, "const StickyBits sticky_bits = heap_.heap()->sticky_bits();", "sticky 位读取");
need(srcCc, "for (NormalPageSpace* space : compactable_spaces_) {", "逐空间紧凑");
need(srcCc, "DCHECK_NULL(heap_.heap()->marker());", "禁标记期启用断言");
need(srcCc, "void CompactSpace(NormalPageSpace* space, MovableReferences& movable_references,", "CompactSpace 声明");
// 方法实体计数: Compactor 5 + MovableReferences 3 (AddOrFilter/Relocate/RelocateInterior; UpdateCallbacks 仅类内声明无类外体) + UpdateHeapResidency + CompactSpace = 10
const defs = (srcCc.match(/^(?:void|bool|size_t|Compactor::CompactableSpaceHandling|Compactor) Compactor::|^void MovableReferences::|^size_t UpdateHeapResidency|^void CompactSpace/gm) || []).length;
if (defs !== 10) throw new Error(`compactor.cc 实体计数失配: ${defs} want 10`);
n += defs;

const body = `${banner("src/heap/cppgc-internal/compactor.js", `${REL} + ${REL_CC}`, "页压缩: 槽登记/重定位 + 阈值触发")}
#![allow(dead_code)]

pub mod constants {
    /// v8: src/heap/cppgc-internal/compactor.cc kFreeListSizeThreshold
    pub const K_FREE_LIST_SIZE_THRESHOLD: usize = 512 * K_KB;
}

/// v8: src/heap/cppgc-internal/compactor.cc MovableReferences
/// 一次压缩增强 GC 的生命周期内登记可移动对象引用槽; 对象移动后 Relocate 校正槽 (含内指针)。
pub struct MovableReferences {
    /// v8: heap_ / heap_has_move_listeners_
    heap: *mut HeapBase,
    heap_has_move_listeners: bool,
}

impl MovableReferences {
    /// v8: src/heap/cppgc-internal/compactor.cc MovableReferences::AddOrFilter
    /// 槽在死对象 (freelist index / 未标记页) 上则丢弃; 否则登记 + 内指针区间展开。
    pub fn add_or_filter(&mut self, slot: *mut MovableReference) { todo!("死槽过滤 + 区间树登记") }
    /// v8: MovableReferences::Relocate — 背仓 from→to 整体搬迁校正
    pub fn relocate(&mut self, from: *mut u8, to: *mut u8, size_including_header: usize) { todo!("精确槽 + 内指针校正") }
    /// v8: MovableReferences::RelocateInteriorReferences — 指向 from..from+size 内部的槽按偏移平移
    pub fn relocate_interior_references(&mut self, from: *mut u8, to: *mut u8, size: usize) { todo!("偏移平移") }
    /// v8: MovableReferences::UpdateCallbacks — OnMove 通知 (类内声明, 语义面留锚)
    pub fn update_callbacks(&mut self) { todo!("MoveListener 回放") }
}

/// v8: src/heap/cppgc-internal/compactor.h Compactor
pub struct Compactor {
    /// v8: heap_
    heap: *mut RawHeap,
    /// v8: compactable_spaces_ — 堆持有空间, 此处不拥有
    compactable_spaces: Vec<*mut NormalPageSpace>,
    /// v8: compaction_worklists_
    compaction_worklists: Option<Box<CompactionWorklists>>,
    /// v8: is_enabled_ / is_cancelled_ / enable_for_next_gc_for_testing_
    is_enabled: bool,
    is_cancelled: bool,
    enable_for_next_gc_for_testing: bool,
}

impl Compactor {
    /// v8: src/heap/cppgc-internal/compactor.cc Compactor 构造 — 收集全部 is_compactable 空间
    pub fn new(heap: *mut RawHeap) -> Self {
        Self { heap, compactable_spaces: Vec::new(), compaction_worklists: None,
               is_enabled: false, is_cancelled: false, enable_for_next_gc_for_testing: false }
    }

    /// v8: src/heap/cppgc-internal/compactor.cc Compactor::ShouldCompact
    /// 无可紧凑空间, 或 (原子 GC + 保守栈) → false (测试旗时 DCHECK);
    /// 测试旗 → true; 否则 UpdateHeapResidency > kFreeListSizeThreshold(512KB)。
    pub fn should_compact(&self, marking_type: MarkingType, stack_state: StackState) -> bool {
        if self.compactable_spaces.is_empty()
            || (marking_type == MarkingType::Atomic && stack_state == StackState::MayContainHeapPointers)
        {
            debug_assert!(!self.enable_for_next_gc_for_testing);
            return false;
        }
        if self.enable_for_next_gc_for_testing { return true; }
        let free_list_size = update_heap_residency(&self.compactable_spaces);
        free_list_size > constants::K_FREE_LIST_SIZE_THRESHOLD
    }

    /// v8: src/heap/cppgc-internal/compactor.cc Compactor::InitializeIfShouldCompact
    /// 判定通过 → 建 worklists + is_enabled=true, is_cancelled=false。
    pub fn initialize_if_should_compact(&mut self, marking_type: MarkingType, stack_state: StackState) {
        debug_assert!(!self.is_enabled);
        if !self.should_compact(marking_type, stack_state) { return; }
        self.compaction_worklists = Some(Box::new(CompactionWorklists::default()));
        self.is_enabled = true;
        self.is_cancelled = false;
    }

    /// v8: src/heap/cppgc-internal/compactor.cc Compactor::CancelIfShouldNotCompact
    /// 已启用但不再应压 → is_cancelled=true, is_enabled=false。
    pub fn cancel_if_should_not_compact(&mut self, marking_type: MarkingType, stack_state: StackState) {
        if !self.is_enabled || self.should_compact(marking_type, stack_state) { return; }
        self.is_cancelled = true;
        self.is_enabled = false;
    }

    /// v8: src/heap/cppgc-internal/compactor.cc Compactor::CompactSpacesIfEnabled
    /// 取消: 清 movable 队; 未启用 → kSweep;
    /// 否则: MovableReferences 装载全部槽 → 按 sticky_bits 逐空间 CompactSpace → 复位旗 → kIgnore。
    pub fn compact_spaces_if_enabled(&mut self) -> CompactableSpaceHandling {
        if self.is_cancelled && self.compaction_worklists.is_some() {
            self.compaction_worklists = None;
        }
        if !self.is_enabled { return CompactableSpaceHandling::Sweep; }
        let sticky_bits = unsafe { (*(*self.heap).heap()).sticky_bits() };
        let _ = sticky_bits;
        todo!("MovableReferences 装载 + CompactSpace 循环")
    }

    /// v8: src/heap/cppgc-internal/compactor.cc Compactor::EnableForNextGCForTesting
    /// DCHECK 无 marker; 置旗。
    pub fn enable_for_next_gc_for_testing(&mut self) {
        self.enable_for_next_gc_for_testing = true;
    }
}

/// v8: src/heap/cppgc-internal/compactor.cc UpdateHeapResidency
/// Σ free_list().Size() (空空间计 0)。
fn update_heap_residency(spaces: &[*mut NormalPageSpace]) -> usize {
    spaces.iter().map(|s| unsafe { &**s }.free_list().size()).sum()
}
`;

writeSrc("v8-heap-cppgc", "compactor.rs.skel", body);
console.log(`compactor.js: 对数 ${n} 项 (.h 11 + .cc 15 + 实体计数 10) → v8-heap-cppgc/src/compactor.rs.skel`);
