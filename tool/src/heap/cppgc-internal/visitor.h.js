#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/visitor.h.js — v8/src/heap/cppgc-internal/{visitor.h,visitor.cc} → v8-heap-cppgc/src/visitor.rs.skel
// 职责: 访问者密钥工厂 + VisitorBase/RootVisitorBase + ConservativeTracingVisitor 保守扫链
// 规则:
//   ① VisitorFactory::CreateKey — Key 惯用法在 Rust 侧以私有构造 + 工厂 trait 对应
//   ② TraceConservatively: DISABLE_ASAN (扫毒区), 按字扫, <= kSentinelValue 跳过, VisitPossiblePointers + 全指针补访
//   ③ TryTracePointerConservatively: cage 界 → Lookup 页 → TryObjectHeaderFromInnerAddress → TraceConservativelyIfNeeded
//   ④ TraceConservativelyIfNeeded(header): 完建 → VisitFullyConstructed; 在建 → VisitInConstruction (回调 TraceConservatively)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/visitor.h";
const REL_CC = "src/heap/cppgc-internal/visitor.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class VisitorFactory final {", "工厂类");
need(src, "static constexpr Visitor::Key CreateKey() { return {}; }", "造钥匙");
need(src, "class VisitorBase : public cppgc::Visitor {", "内部 Visitor 基");
need(src, "static void TraceRawForTesting(cppgc::Visitor* visitor, const T* t) {", "测试原始 trace");
need(src, "class RootVisitorBase : public RootVisitor {", "根 Visitor 基");
need(src, "class V8_EXPORT_PRIVATE ConservativeTracingVisitor {", "保守访问者类");
need(src, "virtual void TraceConservativelyIfNeeded(const void*);", "地址版虚声明");
need(src, "void TraceConservativelyIfNeeded(HeapObjectHeader&);", "头版非虚声明");
need(src, "void TraceConservatively(const HeapObjectHeader&);", "保守扫声明");
need(src, "using TraceConservativelyCallback = void(ConservativeTracingVisitor*,", "回调签名别名");
need(src, "virtual void VisitFullyConstructedConservatively(HeapObjectHeader&);", "完建虚钩子");
need(src, "virtual void VisitInConstructionConservatively(HeapObjectHeader&,", "在建虚钩子");
need(src, "void TryTracePointerConservatively(ConstAddress address);", "试访指针声明");
need(cc, "using PointerRepresentation = CompressedPointer;", "压缩指针表示");
need(cc, "using PointerRepresentation = RawPointer;", "原始指针表示");
need(cc, "DISABLE_ASAN", "ASAN 禁用标");
need(cc, "if (maybe_full_ptr <= SentinelPointer::kSentinelValue) {", "哨兵界跳过");
need(cc, "PointerRepresentation::VisitPossiblePointers(", "候选指针访");
need(cc, "if (V8_LIKELY(!CagedHeapBase::IsWithinCage(address))) return;", "cage 界快退");
need(cc, "page_backend_.Lookup(address));", "页后端定位");
need(cc, "page->TryObjectHeaderFromInnerAddress(address));", "内地址试取头");
need(cc, "if (!header.IsInConstruction<AccessMode::kNonAtomic>()) {", "在建判定分支");
need(cc, "GlobalGCInfoTable::GCInfoFromIndex(header.GetGCInfoIndex()).trace});", "GCInfo trace 取用");

const body = `${banner("src/heap/cppgc-internal/visitor.h.js", REL_H, "VisitorFactory/VisitorBase/RootVisitorBase + ConservativeTracingVisitor 保守扫链 (cc 实现一并落地)")}
// ══ VisitorFactory + 双 Base (原版 visitor.h:18-45) ══
// Key 惯用法: 外部无法直接构造 cppgc::Visitor, 只有内部工厂能造钥匙。

pub struct VisitorFactory;
impl VisitorFactory {
    /// 原版 visitor.h:20 — constexpr Key
    pub fn create_key() -> VisitorKey { VisitorKey }
}

/// cppgc::Visitor 内部版 — 工厂钥匙构造
pub struct VisitorBase {
    pub visitor: cppgc::Visitor, // Visitor(VisitorFactory::CreateKey())
}

impl VisitorBase {
    /// 原版 visitor.h:27-30 — 测试用: 直接调 TraceImpl
    pub fn trace_raw_for_testing<T>(visitor: &mut cppgc::Visitor, t: *const T) {
        visitor.trace_impl(t);
    }
}

/// RootVisitor 内部版 — 同钥匙构造
pub struct RootVisitorBase {
    pub root_visitor: RootVisitor,
}

// ══ ConservativeTracingVisitor (原版 visitor.h:48-75, 实现 visitor.cc:39-129) ══
// v8: PointerRepresentation — CPPGC_POINTER_COMPRESSION 开 → CompressedPointer, 否则 RawPointer

pub struct ConservativeTracingVisitor {
    pub heap: *mut HeapBase,
    pub page_backend: *mut PageBackend,
    pub visitor: *mut cppgc::Visitor,
    /// 原版 visitor.h:58 — 在建对象保守访回调
    pub in_construction_callback: Option<fn(&mut ConservativeTracingVisitor, &HeapObjectHeader)>,
}

impl ConservativeTracingVisitor {
    /// 原版 visitor.cc:39-41
    pub fn new(heap: &mut HeapBase, page_backend: &mut PageBackend, visitor: &mut cppgc::Visitor) -> Self {
        Self { heap, page_backend, visitor, in_construction_callback: None }
    }

    /// 原版 visitor.cc:46-73 — DISABLE_ASAN (可能扫毒区); 按机器字逐扫; <= 哨兵跳过;
    /// 候选指针访 + 全指针补访 (如 TracedReference 指向区外, 见 UnifiedHeapConservativeMarkingVisitor)
    #[inline(never)] // v8: DISABLE_ASAN 对应 — 保守扫不得被 ASAN 插桩
    pub fn trace_conservatively(&mut self, header: &HeapObjectHeader) {
        let object_view = ObjectView::new(header);
        let word = object_view.start() as *mut usize;
        for i in 0..(object_view.size() / std::mem::size_of::<usize>()) {
            let mut maybe_full_ptr = unsafe { *word.add(i) };
            // v8: MSAN — 局部副本保持原 MSAN 状态 (对象可含未初始化/填充字节)
            msan_memory_is_initialized(&mut maybe_full_ptr);
            // 首个 OS 页与首个 cage 页都不含 Oilpan 对象
            if maybe_full_ptr <= SENTINEL_POINTER_VALUE {
                continue;
            }
            // v8: PointerRepresentation::VisitPossiblePointers — 压缩候选对齐扫描
            pointer_representation_visit_possible_pointers(maybe_full_ptr as *mut u8, &mut |raw| self.trace_conservatively_if_needed_addr(raw));
            // 全指针补访 (区外引用, 如 v8::TracedReference)
            self.trace_conservatively_if_needed_addr(maybe_full_ptr as *mut u8);
        }
    }

    /// 原版 visitor.cc:75-94 — cage 界快退 → Lookup 页 → 试取头 → 需要则保守访
    pub fn try_trace_pointer_conservatively(&mut self, address: *const u8) {
        // v8: CPPGC_CAGED_HEAP — TODO(chromium:1056170) SIMD 栈扫
        if !CagedHeapBase::is_within_cage(address) {
            return;
        }
        let page = unsafe { &*self.page_backend }.lookup(address) as *const BasePage;
        if page.is_null() {
            return;
        }
        debug_assert_eq!(self.heap, unsafe { &*page }.heap() as *mut _);
        let header = unsafe { &*page }.try_object_header_from_inner_address(address);
        if header.is_null() {
            return;
        }
        self.trace_conservatively_if_needed_header(unsafe { &mut *header });
    }

    /// 原版 visitor.cc:96-108 — 地址版: <= 哨兵跳过; 候选指针回调走 Try 版
    pub fn trace_conservatively_if_needed_addr(&mut self, address: *const u8) {
        if address as usize <= SENTINEL_POINTER_VALUE {
            return;
        }
        pointer_representation_visit_possible_pointers(address as *mut u8, &mut |raw| {
            self.try_trace_pointer_conservatively(raw as *const u8)
        });
    }

    /// 原版 visitor.cc:110-121 — 头版: 完建 → Fully 钩子; 在建 → InConstruction 钩子 (默认空, 调用方传 TraceConservatively)
    pub fn trace_conservatively_if_needed_header(&mut self, header: &mut HeapObjectHeader) {
        if !header.is_in_construction::<AccessMode::NonAtomic>() {
            self.visit_fully_constructed_conservatively(header);
        } else if let Some(cb) = self.in_construction_callback {
            cb(self, header);
        }
    }

    /// 原版 visitor.cc:123-129 — 完建: visitor.Visit(ObjectStart, {start, GCInfo trace})
    pub fn visit_fully_constructed_conservatively(&mut self, header: &mut HeapObjectHeader) {
        let trace = GlobalGcInfoTable::gc_info_from_index(header.get_gc_info_index()).trace;
        unsafe { &mut *self.visitor }.visit(header.object_start(), TraceDescriptor { base_object_payload: header.object_start(), trace });
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "visitor.rs.skel", body);
console.log(`OK visitor.h.js: 锚 ${n} 条 (h13+cc9); 保守扫 4 链; 产物 ${out}`);
