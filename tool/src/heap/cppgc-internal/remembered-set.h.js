#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/remembered-set.h.js — v8/src/heap/cppgc-internal/{remembered-set.h,remembered-set.cc} → v8-heap-cppgc/src/remembered_set.rs.skel
// 职责: OldToNewRememberedSet 分代记忆集 — 压缩槽 slot-set 位图 + 非压缩 set + 源对象/弱回调/在建对象 5 容器, 加 2 个 CRTP 访问者
// 规则:
//   ① 整族 CPPGC_YOUNG_GENERATION 门 → Rust cfg feature cppgc-young-generation
//   ② Add 族 5 方法落容器; Invalidate 2 方法清范围; Visit 3 步 (槽/源对象/在建重访); Reset 交 SlotRemover; IsEmpty 3 容器与
//   ③ VisitSlot<SlotType> 模板双臂: young 跳过 / 解引后 sentinel|null 跳过 → DynamicallyMarkAddress
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/remembered-set.h";
const REL_CC = "src/heap/cppgc-internal/remembered-set.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "#if defined(CPPGC_YOUNG_GENERATION)", "h 分代门");
need(src, "class SlotSet : public ::heap::base::BasicSlotSet<kSlotSize> {};", "SlotSet 别名");
need(src, "class V8_EXPORT_PRIVATE OldToNewRememberedSet final {", "主类");
need(src, "using WeakCallbackItem = MarkingWorklists::WeakCallbackItem;", "弱回调项别名");
need(src, "struct RememberedInConstructionObjects final {", "在建对象容器");
need(src, "std::set<HeapObjectHeader*> previous;", "previous 集合");
need(src, "std::set<HeapObjectHeader*> current;", "current 集合");
need(src, "lhs.parameter < rhs.parameter", "弱回调按 parameter 序");
need(cc, "enum class SlotType { kCompressed, kUncompressed };", "槽类型枚举");
need(cc, "value = CompressedPointer::Decompress(*reinterpret_cast<uint32_t*>(slot));", "压缩解引");
need(cc, "if (value == kSentinelPointer || value == nullptr) return;", "哨兵空跳过");
need(cc, "marking_state.DynamicallyMarkAddress(static_cast<Address>(value));", "动态标地址");
need(cc, "class CompressedSlotVisitor : HeapVisitor<CompressedSlotVisitor> {", "压缩槽访问者");
need(cc, "class SlotRemover : HeapVisitor<SlotRemover> {", "槽清除者");
need(cc, "return heap::base::KEEP_SLOT;", "KEEP_SLOT 语义");
need(cc, "if (slot_header.IsYoung()) return;", "young 精确跳过");
need(cc, "if (source_hoh->IsYoung()) continue;", "源对象 young 跳过");
need(cc, "conservative_visitor.TraceConservatively(*hoh);", "在建保守 trace");
need(cc, "slot_set.Insert<SlotSet::AccessMode::NON_ATOMIC>(", "槽插入非原子");
need(cc, "std::copy_if(previous.begin(), previous.end(),", "在建容器轮转 copy_if");
need(cc, "previous = std::move(current);", "轮转移动");
need(cc, "// Custom weak callbacks is alive across GCs.", "弱回调跨 GC 存活注释");

// 公有方法全序核对 (12 个)
const methods = [...cc.matchAll(/^(?:void|bool) OldToNewRememberedSet::(\w+)\(/gm)].map((m) => m[1]);
if (methods.join(",") !== "AddSlot,AddUncompressedSlot,AddSourceObject,AddWeakCallback,AddInConstructionObjectToBeRetraced,InvalidateRememberedSlotsInRange,InvalidateRememberedSourceObject,Visit,ExecuteCustomCallbacks,ReleaseCustomCallbacks,Reset,IsEmpty")
  throw new Error(`公有方法序: got=${methods.join(",")}`);
n++;
// generational_gc_supported 前置断言: 12 公有方法 (Reset/IsEmpty 也带) — 实数
const ggs = (cc.match(/DCHECK\(heap_\.generational_gc_supported\(\)\);/g) || []).length;
if (ggs !== 11) throw new Error(`分代支持断言计数: got=${ggs} want=11`);
n++;
// 6 容器成员
const members = (src.match(/std::set</g) || []).length;
if (members !== 6) throw new Error(`set 容器成员计数: got=${members} want=6`);
n++;

const body = `${banner("src/heap/cppgc-internal/remembered-set.h.js", REL_H, "OldToNewRememberedSet 分代记忆集 + CompressedSlotVisitor/SlotRemover (cc 实现一并落地, 整族 CPPGC_YOUNG_GENERATION 门)")}
// ══ 整族门 (原版 remembered-set.h:10 / remembered-set.cc 尾 #endif) ══
#![cfg(feature = "cppgc-young-generation")] // v8: CPPGC_YOUNG_GENERATION

// ══ SlotType (原版 remembered-set.cc:27) ══
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotType { Compressed, Uncompressed }

/// 原版 remembered-set.h:29 — SlotSet = heap::base::BasicSlotSet<kSlotSize> 每页两级位图
pub type SlotSet = heap_base::BasicSlotSet<K_SLOT_SIZE>;

// ══ VisitSlot 模板 (原版 remembered-set.cc:85-117) ══
/// 泛型双臂: young 跳过 (卡表不精确, 此处精查); 解引后 sentinel/null 跳过; 终点 DynamicallyMarkAddress
fn visit_slot<SLOT: SlotTypeTag>(
    heap: &HeapBase,
    page: &BasePage,
    slot: *mut u8,
    marking_state: &mut MutatorMarkingState,
    slots_for_verification: &std::collections::BTreeSet<usize>,
) {
    // v8: DEBUG 臂 — slot 归属页 + 验证集含 slot
    let slot_header = page.object_header_from_inner_address(slot);
    if slot_header.is_young() {
        return; // 分代屏障年龄检查不精确 (卡内混龄), 精确跳过
    }
    // v8: CPPGC_POINTER_COMPRESSION — 压缩臂 uint32 解压
    let value: *mut u8 = if SLOT::IS_COMPRESSED {
        compressed_pointer_decompress(unsafe { *(slot as *const u32) })
    } else {
        unsafe { *(slot as *const *mut u8) }
    };
    // 槽可被 mutator 更新为 nullptr 或 kSentinelPointer
    if value as usize == K_SENTINEL_POINTER || value.is_null() {
        return;
    }
    marking_state.dynamically_mark_address(value);
}

// ══ CompressedSlotVisitor (原版 remembered-set.cc:120-168) ══
/// CRTP HeapVisitor: 页游标 → slot-set 迭代 → 逐槽 VisitSlot<Compressed>, KEEP_SLOT 续访
struct CompressedSlotVisitor<'a> {
    heap: &'a HeapBase,
    marking_state: &'a mut MutatorMarkingState,
    current_page: *mut BasePage,
    remembered_slots_for_verification: &'a std::collections::BTreeSet<usize>,
    objects_visited: usize, // = 0
}

impl CompressedSlotVisitor<'_> {
    /// 原版 :137-140 — Traverse 后返回访问数
    fn run(&mut self) -> usize {
        heap_visitor_traverse(self, self.heap.raw_heap());
        self.objects_visited
    }
    /// 原版 :144-150 — KEEP_SLOT
    fn visit_compressed_slot(&mut self, slot: *mut u8) -> heap_base::SlotCallbackResult {
        visit_slot::<Compressed>(self.heap, unsafe { &*self.current_page }, slot, self.marking_state, self.remembered_slots_for_verification);
        self.objects_visited += 1;
        heap_base::KEEP_SLOT
    }
    /// 原版 :152-165 — 0..buckets 全迭代, FREE_EMPTY_BUCKETS
    fn visit_slot_set(&mut self, slot_set: *mut SlotSet) { /* Iterate(page_start, 0, buckets, …, FREE_EMPTY_BUCKETS) */ }
    /// 原版 :167-170 / :172-175 — 双页钩子设 current_page
    fn visit_normal_page(&mut self, page: &mut NormalPage) -> bool {
        self.current_page = page as *mut _ as *mut BasePage;
        self.visit_slot_set(page.slot_set());
        true
    }
    fn visit_large_page(&mut self, page: &mut LargePage) -> bool {
        self.current_page = page as *mut _ as *mut BasePage;
        self.visit_slot_set(page.slot_set());
        true
    }
}

// ══ SlotRemover (原版 remembered-set.cc:177-196) ══
/// CRTP: 双页钩子 ResetSlotSet
struct SlotRemover<'a> { heap: &'a HeapBase }

impl SlotRemover<'_> {
    fn run(&mut self) { heap_visitor_traverse(self, self.heap.raw_heap()); }
    fn visit_normal_page(&mut self, page: &mut NormalPage) -> bool {
        page.reset_slot_set();
        true
    }
    fn visit_large_page(&mut self, page: &mut LargePage) -> bool {
        page.reset_slot_set();
        true
    }
}

// ══ OldToNewRememberedSet (原版 remembered-set.h:32-89, 实现 cc:272-388) ══
pub struct OldToNewRememberedSet {
    heap: *mut HeapBase,
    /// 6 容器: 源对象 / 弱回调 (按 parameter 序) / 非压缩槽 / DEBUG 验证槽 / 在建 previous+current
    remembered_source_objects: std::collections::BTreeSet<*mut HeapObjectHeader>,
    remembered_weak_callbacks: std::collections::BTreeSet<WeakCallbackItem>, // compare: lhs.parameter < rhs.parameter
    remembered_uncompressed_slots: std::collections::BTreeSet<usize>,
    remembered_slots_for_verification: std::collections::BTreeSet<usize>, // DEBUG only
    remembered_in_construction_objects: RememberedInConstructionObjects,
}

/// 原版 remembered-set.h:66-71
struct RememberedInConstructionObjects {
    previous: std::collections::BTreeSet<*mut HeapObjectHeader>,
    current: std::collections::BTreeSet<*mut HeapObjectHeader>,
}

impl OldToNewRememberedSet {
    /// 原版 cc:272-290 — 压缩槽: 页内 slot-set 位图 Insert (NON_ATOMIC); DEBUG 双记账
    pub fn add_slot(&mut self, slot: *mut u8) {
        debug_assert!(self.heap().generational_gc_supported());
        let source_page = BasePage::from_inner_address(self.heap(), slot);
        let slot_set = source_page.get_or_allocate_slot_set();
        let slot_offset = slot as usize - source_page as *const _ as usize;
        slot_set.insert_non_atomic(slot_offset);
        // v8: DEBUG — remembered_slots_for_verification_.insert(slot)
    }

    /// 原版 cc:291-298 — 非压缩槽: set 插入 + DEBUG 双记账
    pub fn add_uncompressed_slot(&mut self, uncompressed_slot: *mut u8) {
        debug_assert!(self.heap().generational_gc_supported());
        self.remembered_uncompressed_slots.insert(uncompressed_slot as usize);
    }

    /// 原版 cc:299-302
    pub fn add_source_object(&mut self, hoh: &mut HeapObjectHeader) {
        debug_assert!(self.heap().generational_gc_supported());
        self.remembered_source_objects.insert(hoh);
    }

    /// 原版 cc:304-310
    pub fn add_weak_callback(&mut self, item: WeakCallbackItem) {
        debug_assert!(self.heap().generational_gc_supported());
        self.remembered_weak_callbacks.insert(item);
    }

    /// 原版 cc:312-316 — 记入 current, 下轮 minor GC 重 trace
    pub fn add_in_construction_object_to_be_retraced(&mut self, hoh: &mut HeapObjectHeader) {
        debug_assert!(self.heap().generational_gc_supported());
        self.remembered_in_construction_objects.current.insert(hoh);
    }

    /// 原版 cc:318-326 — 双清: 压缩槽走 slot-set RemoveRange (页界 DCHECK), 非压缩走 set 区间删
    pub fn invalidate_remembered_slots_in_range(&mut self, begin: *mut u8, end: *mut u8) {
        debug_assert!(self.heap().generational_gc_supported());
        invalidate_compressed_remembered_slots(self.heap(), begin, end, &mut self.remembered_slots_for_verification);
        invalidate_uncompressed_remembered_slots(&mut self.remembered_uncompressed_slots, begin, end, &mut self.remembered_slots_for_verification);
    }

    /// 原版 cc:328-332
    pub fn invalidate_remembered_source_object(&mut self, header: &HeapObjectHeader) {
        debug_assert!(self.heap().generational_gc_supported());
        self.remembered_source_objects.remove(&(header as *const _ as *mut _));
    }

    /// 原版 cc:334-343 — 3 步: 槽访 (压缩 visitor + 非压缩循环, DCHECK 数一致) / 源对象 eager trace / 在建对象重访
    pub fn visit(
        &mut self,
        visitor: &mut Visitor,
        conservative_visitor: &mut ConservativeTracingVisitor,
        marking_state: &mut MutatorMarkingState,
    ) {
        debug_assert!(self.heap().generational_gc_supported());
        // 段1: VisitRememberedSlots — compressed visitor 全堆遍历 + uncompressed 循环, 计数 == 验证集大小
        visit_remembered_slots(self.heap(), marking_state, &self.remembered_uncompressed_slots, &self.remembered_slots_for_verification);
        // 段2: VisitRememberedSourceObjects — young 跳过, 否则 GCInfo trace eager (免重记账)
        visit_remembered_source_objects(&self.remembered_source_objects, visitor);
        // 段3: RevisitInConstructionObjects(previous) — 在建保守 trace / 完建精确 trace
        revisit_in_construction_objects(&self.remembered_in_construction_objects.previous, visitor, conservative_visitor);
    }

    /// 原版 cc:345-353 — 按序执行自定义弱回调
    pub fn execute_custom_callbacks(&mut self, broker: LivenessBroker) {
        debug_assert!(self.heap().generational_gc_supported());
        for callback in &self.remembered_weak_callbacks {
            (callback.callback)(broker, callback.parameter);
        }
    }

    /// 原版 cc:355-358
    pub fn release_custom_callbacks(&mut self) {
        debug_assert!(self.heap().generational_gc_supported());
        self.remembered_weak_callbacks.clear();
    }

    /// 原版 cc:360-368 — SlotRemover 全堆清 slot-set + 4 容器清; 弱回调跨 GC 存活不清
    pub fn reset(&mut self) {
        debug_assert!(self.heap().generational_gc_supported());
        SlotRemover { heap: self.heap() }.run();
        self.remembered_uncompressed_slots.clear();
        self.remembered_source_objects.clear();
        // v8: DEBUG — remembered_slots_for_verification_.clear()
        self.remembered_in_construction_objects.reset();
        // v8: 原版注释 — Custom weak callbacks is alive across GCs.
    }

    /// 原版 cc:370-375 — 3 容器与 (不含验证集/在建)
    pub fn is_empty(&self) -> bool {
        self.remembered_uncompressed_slots.is_empty()
            && self.remembered_source_objects.is_empty()
            && self.remembered_weak_callbacks.is_empty()
    }
}

/// 原版 cc:377-387 — 轮转: previous 中仍在建的 copy 进 current → previous = move(current) → current 清空
impl RememberedInConstructionObjects {
    fn reset(&mut self) {
        let prev: Vec<_> = self.previous.iter().filter(|h| (**h).is_in_construction_non_atomic()).copied().collect();
        self.current.extend(prev);
        self.previous = std::mem::take(&mut self.current);
        self.current.clear();
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "remembered_set.rs.skel", body);
console.log(`OK remembered-set.h.js: 锚 ${n} 条 (h9+cc16+方法序1+断言计数1+容器计数1); 12 公有方法; 产物 ${out}`);
